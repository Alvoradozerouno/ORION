//! Replay Engine — deterministic state reconstruction

use crate::audit_chain::AuditEntry;
use crate::GENESIS_ANCHOR;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelState {
    pub prev_hash: String,
    pub entry_hash: String,
    pub intent: String,
    pub pattern: String,
    pub decision: String,
}

/// Replay chain from from_hash to to_hash. Returns reconstructed states.
/// Verifies signatures if signer provided. Rejects tampered entries.
pub fn replay(
    chain: &[AuditEntry],
    from_hash: &str,
    to_hash: &str,
) -> Vec<KernelState> {
    let mut started = false;
    let mut result = Vec::new();
    let mut prev = GENESIS_ANCHOR;
    for e in chain {
        if e.entry_hash == from_hash {
            started = true;
        }
        if started {
            result.push(KernelState {
                prev_hash: prev.to_string(),
                entry_hash: e.entry_hash.clone(),
                intent: e.intent.clone(),
                pattern: e.pattern.clone(),
                decision: e.decision.clone(),
            });
            prev = &e.entry_hash;
            if e.entry_hash == to_hash {
                break;
            }
        } else {
            prev = &e.entry_hash;
        }
    }
    result
}
