//! GENESIS SOVEREIGN FABRIC — API
//! REST API. Production-ready.

use axum::{extract::State, routing::get, routing::post, Json, Router};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Clone)]
struct AppState {
    engine: Arc<Mutex<gsf_core::WorkflowEngine>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let data_path = std::env::var("GSF_DATA_PATH").ok();
    let engine = if let Some(ref p) = data_path {
        match gsf_core::WorkflowEngine::new()
            .with_persistence(p)
            .map_err(|e| eprintln!("Persistence init failed: {} — using in-memory", e))
        {
            Ok(eng) => eng,
            Err(_) => {
                let mut eng = gsf_core::WorkflowEngine::new();
                eng.symbol_map.register("ping", "pong", None);
                eng.symbol_map.register("request", "processed", None);
                eng
            }
        }
    } else {
        let mut eng = gsf_core::WorkflowEngine::new();
        eng.symbol_map.register("ping", "pong", None);
        eng.symbol_map.register("request", "processed", None);
        eng
    };

    let state = AppState {
        engine: Arc::new(Mutex::new(engine)),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/audit/verify", get(verify))
        .route("/audit/export", get(export_chain))
        .route("/run", post(run))
        .with_state(state);

    let port = std::env::var("GSF_PORT").unwrap_or_else(|_| "8765".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    tracing::info!("GSF API listening on {}", addr);
    axum::serve(listener, app).await.expect("serve");
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok", "service": "gsf-api"}))
}

async fn verify(State(state): State<AppState>) -> Json<serde_json::Value> {
    let eng = state.engine.lock().unwrap();
    Json(serde_json::json!({
        "chain_verified": eng.audit_chain.verify(),
        "entries": eng.audit_chain.export().len()
    }))
}

async fn export_chain(State(state): State<AppState>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let eng = state.engine.lock().unwrap();
    match eng.export_chain_json() {
        Ok(json) => {
            let v: serde_json::Value = serde_json::from_str(&json).unwrap_or(serde_json::Value::Null);
            Ok(Json(v))
        }
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
struct RunRequest {
    intent: String,
    pattern: String,
}

async fn run(State(state): State<AppState>, Json(req): Json<RunRequest>) -> Json<serde_json::Value> {
    let mut eng = state.engine.lock().unwrap();
    match eng.execute(&req.intent, &req.pattern) {
        Ok(entry_hash) => Json(serde_json::json!({
            "ok": true,
            "entry_hash": entry_hash,
            "entries": eng.audit_chain.export().len()
        })),
        Err(e) => Json(serde_json::json!({
            "ok": false,
            "error": e.to_string()
        })),
    }
}
