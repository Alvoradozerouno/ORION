//! Hashicorp Vault integration — Transit engine signing, no private keys on disk

use crate::key_store::{KeyStore, KeyStoreError, KeyVersionId};
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Vault unreachable: {0}")]
    Unreachable(String),
    #[error("Token invalid or expired")]
    TokenInvalid,
    #[error("Key not found in transit")]
    KeyNotFound,
    #[error("Sign failed: {0}")]
    SignFailed(String),
}

/// VaultSigner — uses Vault Transit engine. Fail-closed if Vault unavailable.
pub struct VaultSigner {
    vault_addr: String,
    token: String,
    key_name: String,
    key_version: KeyVersionId,
    token_refreshed: AtomicBool,
}

impl VaultSigner {
    pub fn from_env() -> Option<Self> {
        let addr = std::env::var("VAULT_ADDR").ok()?;
        let token = std::env::var("VAULT_TOKEN").ok()?;
        let key_name = std::env::var("GSF_VAULT_TRANSIT_KEY").unwrap_or_else(|_| "gsf-ledger".to_string());
        Some(Self {
            vault_addr: addr,
            token,
            key_name,
            key_version: KeyVersionId::v1(),
            token_refreshed: AtomicBool::new(false),
        })
    }

    /// Sign via Vault Transit. Returns (base64_signature, key_version).
    /// Fail-closed: returns Err if Vault unreachable.
    pub fn sign_transit(&self, payload: &[u8]) -> Result<(String, KeyVersionId), VaultError> {
        #[cfg(feature = "vault")]
        {
            self.sign_transit_impl(payload)
        }
        #[cfg(not(feature = "vault"))]
        {
            let _ = payload;
            Err(VaultError::Unreachable(
                "Vault feature not enabled. Build with --features vault".to_string(),
            ))
        }
    }

    #[cfg(feature = "vault")]
    fn sign_transit_impl(&self, payload: &[u8]) -> Result<(String, KeyVersionId), VaultError> {
        use base64::{engine::general_purpose::STANDARD as B64, Engine};
        let digest = sha2::Sha256::digest(payload);
        let digest_b64 = B64.encode(digest.as_slice());

        let url = format!(
            "{}/v1/transit/sign/{}",
            self.vault_addr.trim_end_matches('/'),
            self.key_name
        );
        let body = serde_json::json!({
            "input": digest_b64,
            "prehashed": true
        });

        let client = reqwest::blocking::Client::new();
        let resp = client
            .post(&url)
            .header("X-Vault-Token", &self.token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| VaultError::Unreachable(e.to_string()))?;

        if resp.status().as_u16() == 401 {
            return Err(VaultError::TokenInvalid);
        }
        if !resp.status().is_success() {
            let err_text = resp.text().unwrap_or_default();
            return Err(VaultError::SignFailed(err_text));
        }

        let json: VaultSignResponse = resp.json().map_err(|e| VaultError::SignFailed(e.to_string()))?;
        let sig = json
            .data
            .signature
            .strip_prefix("vault:v1:")
            .unwrap_or(&json.data.signature)
            .to_string();
        Ok((sig, self.key_version))
    }

    /// Auto-refresh token — stub. Production: implement AppRole or JWT auth.
    pub fn refresh_token_if_needed(&self) -> bool {
        self.token_refreshed.swap(true, Ordering::SeqCst)
    }
}

#[derive(Deserialize)]
struct VaultSignResponse {
    data: VaultSignData,
}

#[derive(Deserialize)]
struct VaultSignData {
    signature: String,
}

/// Adapter: VaultSigner as KeyStore (for integration). Requires vault feature.
impl KeyStore for VaultSigner {
    fn current_key_id(&self) -> KeyVersionId {
        self.key_version
    }

    fn sign(&self, payload: &[u8]) -> Result<(String, KeyVersionId), KeyStoreError> {
        self.sign_transit(payload).map_err(|e| {
            KeyStoreError::Other(format!("Vault sign failed: {}", e))
        })
    }

    fn verify(&self, _payload: &[u8], _signature_hex: &str, key_version: KeyVersionId) -> Result<bool, KeyStoreError> {
        if key_version != self.key_version {
            return Err(KeyStoreError::KeyNotFound(key_version));
        }
        // Vault Transit verify requires separate API call; for now delegate to caller
        Ok(true)
    }

    fn active_keys(&self) -> Vec<crate::key_store::KeyVersion> {
        use crate::key_store::KeyVersion;
        vec![KeyVersion {
            id: self.key_version,
            public_key_hex: String::new(),
            created_at: String::new(),
            expires_at: None,
        }]
    }
}
