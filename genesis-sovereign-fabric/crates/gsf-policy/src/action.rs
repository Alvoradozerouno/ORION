//! Action and Decision types — production

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Action {
    pub intent: String,
    pub pattern: String,
    pub adapter_invocation: bool,
    pub temperature: f32,
    pub fs_access: Option<String>,
    pub network_access: Option<String>,
    pub timestamp: String,
    pub genesis_anchor: String,
}

impl Action {
    pub fn for_run(intent: &str, pattern: &str, genesis_anchor: &str) -> Self {
        Self {
            intent: intent.to_string(),
            pattern: pattern.to_string(),
            adapter_invocation: false,
            temperature: 0.0,
            fs_access: None,
            network_access: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            genesis_anchor: genesis_anchor.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decision {
    Allow,
    Deny { reason: String, policy_hash: String },
}
