//! Signed Decision Ledger — Ed25519 per entry, KeyStore, canonical signing

use crate::{audit_chain::AuditEntry, GENESIS_ANCHOR};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use gsf_crypto::canonical_sign_payload;
use serde::{Deserialize, Serialize};

/// Key version in AuditEntry — verify() must reject expired/revoked keys
pub const DEFAULT_KEY_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedEntry {
    pub genesis_anchor: String,
    pub prev_hash: String,
    pub entry_hash: String,
    pub signature: String,
    pub timestamp: String,
    #[serde(default)]
    pub key_version_id: Option<u32>,
    pub intent: String,
    pub pattern: String,
    pub decision: String,
}

pub struct LedgerSigner {
    keypair: Keypair,
}

impl LedgerSigner {
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let secret = ed25519_dalek::SecretKey::from_bytes(seed).expect("invalid seed");
        let public = PublicKey::from(&secret);
        let keypair = Keypair { secret, public };
        Self { keypair }
    }

    pub fn from_env() -> Option<Self> {
        let hex_key = std::env::var("GSF_SIGNING_KEY").ok()?;
        let bytes: Vec<u8> = hex::decode(hex_key).ok()?;
        let arr: [u8; 32] = bytes.try_into().ok()?;
        Some(Self::from_seed(&arr))
    }

    /// Deterministic signature mode — canonical field order, no timestamp in scope
    pub fn sign_entry(&self, entry: &AuditEntry) -> SignedEntry {
        let payload = canonical_sign_payload(
            GENESIS_ANCHOR,
            &entry.prev_hash,
            &entry.entry_hash,
            &entry.intent,
            &entry.pattern,
            &entry.decision,
        );
        let sig = self.keypair.sign(payload.as_bytes());
        SignedEntry {
            genesis_anchor: GENESIS_ANCHOR.to_string(),
            prev_hash: entry.prev_hash.clone(),
            entry_hash: entry.entry_hash.clone(),
            signature: hex::encode(sig.to_bytes()),
            timestamp: entry.timestamp.clone(),
            key_version_id: Some(DEFAULT_KEY_VERSION),
            intent: entry.intent.clone(),
            pattern: entry.pattern.clone(),
            decision: entry.decision.clone(),
        }
    }

    pub fn verifying_key(&self) -> PublicKey {
        self.keypair.public
    }

    pub fn sign_payload(&self, payload: &str) -> String {
        hex::encode(self.keypair.sign(payload.as_bytes()).to_bytes())
    }
}

pub fn verify_signed_entry(signed: &SignedEntry, verifying_key: &PublicKey) -> bool {
    let payload = canonical_sign_payload(
        &signed.genesis_anchor,
        &signed.prev_hash,
        &signed.entry_hash,
        &signed.intent,
        &signed.pattern,
        &signed.decision,
    );
    let sig_bytes = match hex::decode(&signed.signature) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let arr: [u8; 64] = match sig_bytes.as_slice().try_into() {
        Ok(a) => a,
        Err(_) => return false,
    };
    let sig = match Signature::from_bytes(&arr) {
        Ok(s) => s,
        Err(_) => return false,
    };
    verifying_key.verify(payload.as_bytes(), &sig).is_ok()
}
