//! Layer 6 — Replay & Forensics Engine
//! Deterministic, signature validated, state hash must match, abort on mismatch

use crate::audit_chain::AuditEntry;
use crate::error::Result;
use crate::state_machine::State;
use crate::GENESIS_ANCHOR;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelState {
    pub prev_hash: String,
    pub entry_hash: String,
    pub intent: String,
    pub pattern: String,
    pub decision: String,
}

/// Replay chain. Returns Vec<State>. Deterministic.
/// Signature validated if verify_fn provided. Abort on mismatch.
pub fn replay<F>(chain: &[AuditEntry], from_hash: &str, to_hash: &str, policy_hash: &str, verify_fn: Option<F>) -> Result<Vec<State>>
where
    F: Fn(&AuditEntry) -> bool,
{
    let mut started = false;
    let mut states = Vec::new();
    let mut prev = GENESIS_ANCHOR;
    let mut state = State::genesis(policy_hash);

    for e in chain {
        if e.entry_hash == from_hash {
            started = true;
        }
        if started {
            if let Some(ref v) = verify_fn {
                if !v(e) {
                    return Err(crate::error::GsfError::SignatureVerificationFailed);
                }
            }
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
                return Err(crate::error::GsfError::ChainVerificationFailed);
            }
            state.audit_head_hash = e.entry_hash.clone();
            state.kernel_state_hash = hex::encode(Sha256::digest(
                format!("{}|{}", state.compute_hash(), e.entry_hash).as_bytes(),
            ));
            states.push(state.clone());
            prev = &e.entry_hash;
            if e.entry_hash == to_hash {
                break;
            }
        } else {
            prev = &e.entry_hash;
        }
    }
    Ok(states)
}

/// Replay chain to KernelState (legacy). No signature verification.
pub fn replay_legacy(chain: &[AuditEntry], from_hash: &str, to_hash: &str) -> Vec<KernelState> {
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
