//! Mesh mTLS — client cert fingerprint verification.
//! GSF_MESH_ALLOWED_FINGERPRINTS=sha256,comma,separated
//! X-Client-Cert-Fingerprint header (set by reverse proxy after mTLS).

use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
};

pub async fn require_mesh_peer(req: Request, next: Next) -> Response<Body> {
    if std::env::var("GSF_MESH_MTLS_DISABLED").unwrap_or_default() == "1" {
        return next.run(req).await;
    }

    let fingerprint = req
        .headers()
        .get("X-Client-Cert-Fingerprint")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim().to_lowercase());

    let allowed = std::env::var("GSF_MESH_ALLOWED_FINGERPRINTS")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if allowed.is_empty() {
        tracing::warn!("GSF_MESH_ALLOWED_FINGERPRINTS empty — mesh sync denied (fail closed)");
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("mesh peer not allowed"))
            .unwrap();
    }

    let fp = fingerprint.as_deref().unwrap_or("");
    if fp.is_empty() {
        tracing::warn!("mesh sync: no X-Client-Cert-Fingerprint header");
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("client cert required"))
            .unwrap();
    }

    if !allowed.iter().any(|a| a == fp) {
        tracing::warn!(fingerprint = %fp, "mesh sync: peer fingerprint not in allowlist");
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("peer not in allowlist"))
            .unwrap();
    }

    next.run(req).await
}
