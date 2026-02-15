//! Layer 3 — Signed Audit Ledger (Immutable)
//! prev_hash, action_hash, output_hash, decision, timestamp, genesis_anchor, ed25519_signature

use crate::ledger::LedgerSigner;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedAuditEntry {
    pub prev_hash: String,
    pub action_hash: String,
    pub output_hash: String,
    pub decision: String,
    pub timestamp: String,
    pub genesis_anchor: String,
    pub ed25519_signature: String,
}

impl SignedAuditEntry {
    pub fn compute_entry_hash(&self) -> String {
        let data = format!(
            "{}|{}|{}|{}|{}|{}",
            self.prev_hash,
            self.action_hash,
            self.output_hash,
            self.decision,
            self.timestamp,
            self.genesis_anchor
        );
        hex::encode(Sha256::digest(data.as_bytes()))
    }

    pub fn sign(&mut self, signer: &LedgerSigner) {
        let payload = format!(
            "{}|{}|{}|{}|{}|{}",
            self.prev_hash,
            self.action_hash,
            self.output_hash,
            self.decision,
            self.timestamp,
            self.genesis_anchor
        );
        self.ed25519_signature = signer.sign_payload(&payload);
    }

    pub fn verify(&self, verifying_key: &PublicKey) -> bool {
        let payload = format!(
            "{}|{}|{}|{}|{}|{}",
            self.prev_hash,
            self.action_hash,
            self.output_hash,
            self.decision,
            self.timestamp,
            self.genesis_anchor
        );
        let sig_bytes = match hex::decode(&self.ed25519_signature) {
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
}
