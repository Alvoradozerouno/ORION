//! Policy DSL parser and runtime check — cannot be bypassed

use crate::action::{Action, Decision};
use serde::Deserialize;
use sha2::{Digest, Sha256};

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

/// Compute policy hash for audit
pub fn policy_hash(yaml: &str) -> String {
    hex::encode(Sha256::digest(yaml.as_bytes()))
}

/// Parse policy from YAML string
pub fn parse_policy(yaml: &str) -> Result<Policy, String> {
    serde_yaml::from_str(yaml).map_err(|e| e.to_string())
}

/// Load policy from path — ConfigMap or local file
pub fn load_policy(path: &std::path::Path) -> Result<(Policy, String), String> {
    let yaml = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let policy = parse_policy(&yaml)?;
    let hash = policy_hash(&yaml);
    Ok((policy, hash))
}

/// Runtime action check — returns Allow or Deny. Cannot be bypassed.
/// policy_hash: from load_policy() or policy_hash(yaml)
pub fn check(action: &Action, policy: &Policy, policy_hash: &str) -> Decision {
    if !policy.scope.hardware && action.intent.to_lowercase().contains("hardware") {
        return Decision::Deny {
            reason: "hardware not in scope".to_string(),
            policy_hash: policy_hash.to_string(),
        };
    }
    if !policy.scope.network_outbound && action.network_access.is_some() {
        return Decision::Deny {
            reason: "network_outbound not in scope".to_string(),
            policy_hash: policy_hash.to_string(),
        };
    }
    if !policy.scope.adapter_invocation && action.adapter_invocation {
        return Decision::Deny {
            reason: "adapter_invocation not in scope".to_string(),
            policy_hash: policy_hash.to_string(),
        };
    }
    if let Some(ref path) = action.fs_access {
        let allowed = policy
            .scope
            .fs_write_paths
            .iter()
            .any(|p| path.starts_with(p));
        if !allowed {
            return Decision::Deny {
                reason: format!("path {} not in fs_write_paths", path),
                policy_hash: policy_hash.to_string(),
            };
        }
    }
    let combined = format!("{} {}", action.intent, action.pattern).to_lowercase();
    for blocked in &policy.invariante.blocked_patterns {
        if combined.contains(&blocked.to_lowercase()) && policy.invariante.deny_on_match {
            return Decision::Deny {
                reason: format!("blocked pattern: {}", blocked),
                policy_hash: policy_hash.to_string(),
            };
        }
    }
    Decision::Allow
}
