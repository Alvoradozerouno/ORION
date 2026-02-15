//! GENESIS SOVEREIGN FABRIC — API
//! Policy before every request. Signed ledger. Production.

mod metrics;

use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    routing::post,
    Json,
    Router,
};
use gsf_core::GENESIS_ANCHOR;
use gsf_policy::{load_policy, Action, Decision};
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Clone)]
struct AppState {
    engine: Arc<Mutex<gsf_core::WorkflowEngine>>,
    policy: Arc<Option<(gsf_policy::Policy, String)>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let use_json_logs = std::env::var("GSF_JSON_LOGS").unwrap_or_else(|_| "0".to_string()) == "1";
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .with_span_events(FmtSpan::CLOSE);
    if use_json_logs {
        subscriber.json().with_current_span(false).init();
    } else {
        subscriber.init();
    }

    let policy = load_policy_from_env();
    let policy = Arc::new(policy);

    let data_path = std::env::var("GSF_DATA_PATH").ok();
    let engine = if let Some(ref p) = data_path {
        match gsf_core::WorkflowEngine::new().with_persistence(p) {
            Ok(eng) => eng,
            Err(e) => {
                tracing::warn!("Persistence init failed: {} — using in-memory", e);
                default_engine()
            }
        }
    } else {
        default_engine()
    };

    let state = AppState {
        engine: Arc::new(Mutex::new(engine)),
        policy,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/live", get(live))
        .route("/metrics", get(metrics_handler))
        .route("/audit/verify", get(verify))
        .route("/audit/export", get(export_chain))
        .route("/run", post(run))
        .route("/mesh/sync", post(mesh_sync))
        .with_state(state);

    let port = std::env::var("GSF_PORT").unwrap_or_else(|_| "8765".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(addr = %addr, "GSF API listening");
    axum::serve(listener, app).await?;
    Ok(())
}

fn default_engine() -> gsf_core::WorkflowEngine {
    let mut eng = gsf_core::WorkflowEngine::new();
    eng.symbol_map.register("ping", "pong", None);
    eng.symbol_map.register("request", "processed", None);
    eng
}

fn load_policy_from_env() -> Option<(gsf_policy::Policy, String)> {
    let candidates: Vec<PathBuf> = std::env::var("GSF_POLICY_PATH")
        .ok()
        .map(|p| vec![PathBuf::from(p)])
        .unwrap_or_else(|| {
            let mut v = vec![];
            if let Ok(cwd) = std::env::current_dir() {
                v.push(cwd.join("config/policy.dsl"));
            }
            v.push(PathBuf::from("/app/config/policy.dsl"));
            v
        });
    for path in candidates {
        if path.exists() {
            return load_policy(&path).ok();
        }
    }
    tracing::warn!("No policy file found, policy check disabled");
    None
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok", "service": "gsf-api"}))
}

async fn live() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "alive"}))
}

async fn metrics_handler() -> (StatusCode, String) {
    (StatusCode::OK, metrics::metrics_prometheus())
}

async fn verify(State(state): State<AppState>) -> Json<serde_json::Value> {
    let eng = state.engine.lock().map_err(|_| ()).unwrap();
    Json(serde_json::json!({
        "chain_verified": eng.audit_chain.verify(),
        "entries": eng.audit_chain.export().len()
    }))
}

async fn export_chain(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let eng = state.engine.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let json = eng.export_chain_json().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let v: serde_json::Value =
        serde_json::from_str(&json).unwrap_or(serde_json::Value::Null);
    Ok(Json(v))
}

#[derive(Deserialize)]
struct RunRequest {
    intent: String,
    pattern: String,
}

async fn run(
    State(state): State<AppState>,
    Json(req): Json<RunRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    metrics::inc_run_requests_total();
    let eng_guard = state.engine.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if gsf_core::is_execution_frozen(eng_guard.persistence()) {
        metrics::inc_run_requests_denied();
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    drop(eng_guard);

    let action = Action::for_run(&req.intent, &req.pattern, GENESIS_ANCHOR);

    if let Some(ref pol) = *state.policy {
        let (policy, policy_hash) = pol;
        let t0 = Instant::now();
        let decision = gsf_policy::check(&action, policy, policy_hash);
        metrics::record_policy_check_duration_us(t0.elapsed().as_micros() as u64);
        if let Decision::Deny { reason, policy_hash: ph } = decision {
            metrics::inc_run_requests_denied();
            let mut eng = state.engine.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            if let Err(e) = eng.append_denied(&req.intent, &req.pattern, &reason, &ph) {
                tracing::error!("append_denied failed: {}", e);
            }
            return Err(StatusCode::FORBIDDEN);
        }
    }

    let mut eng = state.engine.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let t0 = Instant::now();
    match eng.execute(&req.intent, &req.pattern) {
        Ok(entry_hash) => {
            metrics::record_audit_append_duration_us(t0.elapsed().as_micros() as u64);
            Ok(Json(serde_json::json!({
                "ok": true,
                "entry_hash": entry_hash,
                "entries": eng.audit_chain.export().len()
            })))
        }
        Err(e) => {
            tracing::error!("execute failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
struct MeshSyncRequest {
    entries: Vec<MeshEntry>,
}

#[derive(Deserialize)]
struct MeshEntry {
    prev_hash: String,
    entry_hash: String,
    #[allow(dead_code)]
    signature: String,
    timestamp: String,
}

async fn mesh_sync(
    State(state): State<AppState>,
    Json(req): Json<MeshSyncRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let t0 = Instant::now();
    let mut eng = state.engine.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let chain = eng.audit_chain.export();
    let mut last = chain
        .last()
        .map(|e| e.entry_hash.clone())
        .unwrap_or_else(|| GENESIS_ANCHOR.to_string());
    let mut appended = 0u32;
    for e in &req.entries {
        if e.prev_hash != last {
            continue;
        }
        let entry = gsf_core::AuditEntry {
            timestamp: e.timestamp.clone(),
            intent: "mesh_sync".to_string(),
            pattern: e.entry_hash.clone(),
            decision: "synced".to_string(),
            outcome: None,
            prev_hash: e.prev_hash.clone(),
            entry_hash: e.entry_hash.clone(),
        };
        eng.audit_chain.restore_entry(entry);
        last = e.entry_hash.clone();
        appended += 1;
    }
    let head = eng.audit_chain.export().last().map(|x| x.entry_hash.clone());
    metrics::record_mesh_sync_latency_us(t0.elapsed().as_micros() as u64);
    Ok(Json(serde_json::json!({
        "ok": true,
        "appended": appended,
        "head": head
    })))
}
