//! Phase 8 — Governance Layer
//! Policy versioning, change control, execution freeze

use crate::error::Result;
use crate::persistence::Persistence;

pub const KEY_EXECUTION_FROZEN: &str = "governance.execution_frozen";
pub const KEY_GOVERNANCE_MODE: &str = "governance.mode";
pub const KEY_APPROVED_POLICY_HASHES: &str = "governance.approved_policy_hashes";

/// Governance mode: normal, read_only_emergency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceMode {
    Normal,
    ReadOnlyEmergency,
}

impl GovernanceMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "read_only" | "read_only_emergency" | "emergency" => Self::ReadOnlyEmergency,
            _ => Self::Normal,
        }
    }
}

/// Check if execution is frozen. Persistence optional.
pub fn is_execution_frozen(persistence: Option<&Persistence>) -> bool {
    let Some(p) = persistence else {
        return false;
    };
    matches!(
        p.load_kernel_state(KEY_EXECUTION_FROZEN).ok().flatten().as_deref(),
        Some("1") | Some("true") | Some("yes")
    )
}

/// Set execution freeze. Requires signed audit event in production.
pub fn set_execution_frozen(persistence: &Persistence, frozen: bool) -> Result<()> {
    let val = if frozen { "1" } else { "0" };
    persistence.save_kernel_state(KEY_EXECUTION_FROZEN, val)
}

/// Check if policy hash is in approved list. Empty list = allow all.
pub fn is_policy_approved(persistence: Option<&Persistence>, policy_hash: &str) -> bool {
    let Some(p) = persistence else {
        return true;
    };
    let Some(approved) = p.load_kernel_state(KEY_APPROVED_POLICY_HASHES).ok().flatten() else {
        return true;
    };
    if approved.is_empty() {
        return true;
    }
    approved.split(',').any(|h| h.trim() == policy_hash)
}
