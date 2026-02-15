//! Fork Resolution Engine — longest valid chain, sequential append.

use crate::audit_chain::AuditEntry;
use crate::error::Result;
use sha2::{Digest, Sha256};
use std::collections::HashSet;

/// Verify single entry: prev_hash continuity, hash valid, no duplicate entry_hash.
fn verify_entry(e: &AuditEntry, prev_hash: &str, seen: &mut HashSet<String>) -> bool {
    if e.prev_hash != prev_hash {
        return false;
    }
    let data = format!(
        "{}|{}|{}|{}|{}|{}",
        e.timestamp,
        e.intent,
        e.pattern,
        e.decision,
        e.outcome.as_deref().unwrap_or(""),
        prev_hash
    );
    let expected = hex::encode(Sha256::digest(data.as_bytes()));
    if expected != e.entry_hash {
        return false;
    }
    if !seen.insert(e.entry_hash.clone()) {
        return false;
    }
    true
}

/// Select longest valid chain from candidates. Each candidate is a slice of entries.
/// Returns indices of chains that are valid extensions of local_head.
/// verify_fn: optional signature verification per entry.
pub fn select_longest_valid<F>(
    local_head: &str,
    candidates: &[Vec<AuditEntry>],
    verify_fn: Option<F>,
) -> Option<usize>
where
    F: Fn(&AuditEntry) -> bool,
{
    let mut best_len = 0u32;
    let mut best_idx = None;

    for (idx, chain) in candidates.iter().enumerate() {
        let mut prev = local_head;
        let mut seen = HashSet::new();
        let mut valid = true;
        for e in chain {
            if let Some(ref v) = verify_fn {
                if !v(e) {
                    valid = false;
                    break;
                }
            }
            if !verify_entry(e, prev, &mut seen) {
                valid = false;
                break;
            }
            prev = &e.entry_hash;
        }
        if valid && chain.len() as u32 > best_len {
            best_len = chain.len() as u32;
            best_idx = Some(idx);
        }
    }

    best_idx
}

/// Append best chain to engine. Rejects conflicting forks.
/// Returns number of entries appended.
pub fn append_best_chain<F>(
    chain: &mut crate::AuditChain,
    local_head: &str,
    incoming: &[AuditEntry],
    verify_fn: Option<F>,
) -> Result<u32>
where
    F: Fn(&AuditEntry) -> bool,
{
    let mut prev = local_head;
    let mut seen: HashSet<String> = chain.export().iter().map(|e| e.entry_hash.clone()).collect();
    let mut appended = 0u32;

    for e in incoming {
        if !verify_entry(e, prev, &mut seen) {
            return Err(crate::error::GsfError::ChainVerificationFailed);
        }
        if let Some(ref v) = verify_fn {
            if !v(e) {
                return Err(crate::error::GsfError::SignatureVerificationFailed);
            }
        }
        chain.restore_entry(e.clone());
        prev = &e.entry_hash;
        appended += 1;
    }

    Ok(appended)
}
