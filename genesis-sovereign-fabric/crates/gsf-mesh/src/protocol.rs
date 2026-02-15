//! Mesh sync protocol — signed ledger, deterministic merge

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedLedgerEntry {
    pub prev_hash: String,
    pub entry_hash: String,
    pub signature: String,
    pub timestamp: String,
    pub genesis_anchor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshSyncRequest {
    pub node_id: String,
    pub entries: Vec<SignedLedgerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshSyncResponse {
    pub accepted: u32,
    pub rejected: u32,
    pub head_hash: Option<String>,
    pub conflict: bool,
}

/// mTLS-only: client cert required. Peer registry validates.
pub struct MeshSyncProtocol;

impl MeshSyncProtocol {
    pub fn validate_prev_hash(local_head: &str, entry_prev: &str) -> bool {
        local_head == entry_prev
    }

    pub fn deterministic_merge(
        local_chain: &[String],
        incoming: &[SignedLedgerEntry],
    ) -> (Vec<SignedLedgerEntry>, bool) {
        let mut result = Vec::new();
        let mut local_head = local_chain.last().map(String::as_str).unwrap_or("");
        let mut conflict = false;
        for e in incoming {
            if e.prev_hash == local_head {
                result.push(e.clone());
                local_head = &e.entry_hash;
            } else {
                conflict = true;
                break;
            }
        }
        (result, conflict)
    }
}
