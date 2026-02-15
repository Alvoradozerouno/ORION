//! Policy DSL parser and runtime check

use crate::action::{Action, Decision};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Policy {
    pub version: String,
    pub genesis: Option<String>,
    pub scope: Scope,
    pub invariante: Invariante,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scope {
    #[serde(default)]
    pub hardware: bool,
    #[serde(default)]
    pub network_outbound: bool,
    #[serde(default)]
    pub fs_write_paths: Vec<String>,
    #[serde(default)]
    pub adapter_invocation: bool,
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            hardware: false,
            network_outbound: false,
            fs_write_paths: vec!["data/".to_string(), "interventions.jsonl".to_string()],
            adapter_invocation: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Invariante {
    #[serde(default)]
    pub blocked_patterns: Vec<String>,
    #[serde(default = "default_true")]
    pub deny_on_match: bool,
}

fn default_true() -> bool {
    true
}

impl Default for Invariante {
    fn default() -> Self {
        Self {
            blocked_patterns: vec![
                "rm -rf".to_string(),
                "DROP TABLE".to_string(),
                "DELETE FROM".to_string(),
                "format".to_string(),
            ],
            deny_on_match: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub name: String,
    #[serde(default)]
    pub when: String,
    #[serde(default)]
    pub require: Vec<String>,
    #[serde(default)]
    pub llm_allowed: bool,
    #[serde(default)]
    pub temperature_max: Option<f32>,
}

/// Parse policy from YAML string
pub fn parse_policy(yaml: &str) -> Result<Policy, String> {
    serde_yaml::from_str(yaml).map_err(|e| e.to_string())
}

/// Runtime action check — returns Allow or Deny(reason)
pub fn check(action: &Action, policy: &Policy) -> Decision {
    if !policy.scope.hardware && action.intent.contains("hardware") {
        return Decision::Deny("hardware not in scope".to_string());
    }
    if !policy.scope.network_outbound && action.network_target.is_some() {
        return Decision::Deny("network_outbound not in scope".to_string());
    }
    if !policy.scope.adapter_invocation && action.adapter_invocation {
        return Decision::Deny("adapter_invocation not in scope".to_string());
    }
    if let Some(ref path) = action.filesystem_path {
        let allowed = policy
            .scope
            .fs_write_paths
            .iter()
            .any(|p| path.starts_with(p));
        if !allowed {
            return Decision::Deny(format!("path {} not in fs_write_paths", path));
        }
    }
    let combined = format!("{} {}", action.intent, action.pattern).to_lowercase();
    for blocked in &policy.invariante.blocked_patterns {
        if combined.contains(&blocked.to_lowercase()) && policy.invariante.deny_on_match {
            return Decision::Deny(format!("blocked pattern: {}", blocked));
        }
    }
    Decision::Allow
}
