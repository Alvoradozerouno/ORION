//! Stub attestor — deterministic JSON, nonce-bound, Ed25519.
//! No TPM claims. No random strings.

use ed25519_dalek::{Keypair, Signer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestRequest {
    pub nonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestResponse {
    pub mode: String,
    pub nonce: String,
    pub statement: String,
    pub signature_b64: String,
}

fn get_signing_key() -> Option<Keypair> {
    let hex_key = std::env::var("GSF_ATTEST_KEY").ok()?;
    let bytes = hex::decode(hex_key).ok()?;
    let arr: [u8; 32] = bytes.try_into().ok()?;
    let secret = ed25519_dalek::SecretKey::from_bytes(&arr).ok()?;
    let public = ed25519_dalek::PublicKey::from(&secret);
    Some(Keypair { secret, public })
}

/// Produce attestation. Deterministic: statement = sorted JSON.
/// Nonce bound. Ed25519 signature. No placeholders.
pub fn attest(nonce: &str) -> Result<AttestResponse, crate::HardwareError> {
    let statement = serde_json::json!({
        "nonce": nonce,
        "mode": "stub",
        "genesis_anchor": "acb92fd8346a65ff17dbf9a41e3003f2d566a17f839af4c3a90a4b4b1789dd28a",
    });
    let statement_str = serde_json::to_string(&statement).map_err(|_| crate::HardwareError::AttestFailed)?;

    let keypair = get_signing_key().ok_or(crate::HardwareError::AttestFailed)?;
    let sig = keypair.sign(statement_str.as_bytes());
    use base64::Engine;
    let signature_b64 = base64::engine::general_purpose::STANDARD.encode(sig.to_bytes().as_slice());

    Ok(AttestResponse {
        mode: "stub".to_string(),
        nonce: nonce.to_string(),
        statement: statement_str,
        signature_b64,
    })
}
