//! Immutable AuditChain. SHA256-verkettet.
//! Genesis: sha256:acb92fd8346a65ff17dbf9a41e3003f2d566a17f839af4c3a90a4b4b1789dd28a

use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

pub const GENESIS_ANCHOR: &str = "acb92fd8346a65ff17dbf9a41e3003f2d566a17f839af4c3a90a4b4b1789dd28a";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: String,
    pub intent: String,
    pub pattern: String,
    pub decision: String,
    pub outcome: Option<String>,
    pub prev_hash: String,
    pub entry_hash: String,
}

pub struct AuditChain {
    chain: Vec<AuditEntry>,
    last_hash: String,
}

impl AuditChain {
    pub fn new() -> Self {
        Self {
            chain: Vec::new(),
            last_hash: GENESIS_ANCHOR.to_string(),
        }
    }

    pub fn append(
        &mut self,
        intent: &str,
        pattern: &str,
        decision: &str,
        outcome: Option<&str>,
    ) -> AuditEntry {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let data = format!(
            "{}|{}|{}|{}|{}|{}",
            timestamp,
            intent,
            pattern,
            decision,
            outcome.unwrap_or(""),
            self.last_hash
        );
        let entry_hash = hex::encode(Sha256::digest(data.as_bytes()));
        let entry = AuditEntry {
            timestamp,
            intent: intent.to_string(),
            pattern: pattern.to_string(),
            decision: decision.to_string(),
            outcome: outcome.map(String::from),
            prev_hash: self.last_hash.clone(),
            entry_hash: entry_hash.clone(),
        };
        self.last_hash = entry_hash;
        self.chain.push(entry.clone());
        entry
    }

    pub fn verify(&self) -> bool {
        let mut prev = GENESIS_ANCHOR;
        for e in &self.chain {
            let data = format!(
                "{}|{}|{}|{}|{}|{}",
                e.timestamp,
                e.intent,
                e.pattern,
                e.decision,
                e.outcome.as_deref().unwrap_or(""),
                prev
            );
            let expected = hex::encode(Sha256::digest(data.as_bytes()));
            if expected != e.entry_hash {
                return false;
            }
            prev = &e.entry_hash;
        }
        true
    }

    pub fn export(&self) -> &[AuditEntry] {
        &self.chain
    }

    pub fn restore_entry(&mut self, entry: AuditEntry) {
        self.last_hash = entry.entry_hash.clone();
        self.chain.push(entry);
    }

    pub fn set_last_hash(&mut self, h: &str) {
        self.last_hash = h.to_string();
    }
}
