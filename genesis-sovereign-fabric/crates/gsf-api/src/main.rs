//! GENESIS SOVEREIGN FABRIC — API
//! REST API auf Port 8765

use axum::{routing::get, Json, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health)).route("/audit/verify", get(verify));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8765").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok", "service": "gsf-api"}))
}

async fn verify() -> Json<serde_json::Value> {
    let mut engine = gsf_core::WorkflowEngine::new();
    engine.symbol_map.register("ping", "pong", None);
    let _ = engine.execute("INIT", "ping");
    Json(serde_json::json!({
        "chain_verified": engine.audit_chain.verify(),
        "entries": engine.audit_chain.export().len()
    }))
}
