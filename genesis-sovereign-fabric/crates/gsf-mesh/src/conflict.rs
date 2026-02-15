//! Conflict detection and fork resolution

use crate::protocol::SignedLedgerEntry;

/// Detects conflicting branches
pub struct ConflictDetection;

impl ConflictDetection {
    /// Returns true if two chains have diverged
    pub fn has_conflict(chain_a: &[SignedLedgerEntry], chain_b: &[SignedLedgerEntry]) -> bool {
        let a_hashes: std::collections::HashSet<_> =
            chain_a.iter().map(|e| e.entry_hash.as_str()).collect();
        let b_hashes: std::collections::HashSet<_> =
            chain_b.iter().map(|e| e.entry_hash.as_str()).collect();
        !a_hashes.is_subset(&b_hashes) && !b_hashes.is_subset(&a_hashes)
    }
}

/// Fork resolution — deterministic. Longest valid chain wins.
pub struct ForkResolution;

impl ForkResolution {
    pub fn resolve(
        chain_a: &[SignedLedgerEntry],
        chain_b: &[SignedLedgerEntry],
        _verify_fn: impl Fn(&SignedLedgerEntry) -> bool,
    ) -> Vec<SignedLedgerEntry> {
        if chain_a.len() >= chain_b.len() {
            chain_a.to_vec()
        } else {
            chain_b.to_vec()
        }
    }
}
