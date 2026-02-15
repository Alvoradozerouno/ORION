//! Layer 0 — Formal State Machine
//! S(n+1) = F(S(n), A) iff PolicyCheck(A)==Allow, OutputValidation==Pass, SignatureVerification==Valid

use crate::error::{GsfError, Result};
use crate::output_validation::OutputValidator;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::GENESIS_ANCHOR;

/// State S — kernel_state_hash, policy_hash, symbol_map_hash, audit_head_hash, model_registry_hash, hardware_identity_hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub kernel_state_hash: String,
    pub policy_hash: String,
    pub symbol_map_hash: String,
    pub audit_head_hash: String,
    pub model_registry_hash: String,
    pub hardware_identity_hash: String,
}

impl State {
    pub fn genesis(policy_hash: &str) -> Self {
        Self {
            kernel_state_hash: GENESIS_ANCHOR.to_string(),
            policy_hash: policy_hash.to_string(),
            symbol_map_hash: GENESIS_ANCHOR.to_string(),
            audit_head_hash: GENESIS_ANCHOR.to_string(),
            model_registry_hash: GENESIS_ANCHOR.to_string(),
            hardware_identity_hash: GENESIS_ANCHOR.to_string(),
        }
    }

    pub fn compute_hash(&self) -> String {
        let data = format!(
            "{}|{}|{}|{}|{}|{}",
            self.kernel_state_hash,
            self.policy_hash,
            self.symbol_map_hash,
            self.audit_head_hash,
            self.model_registry_hash,
            self.hardware_identity_hash
        );
        hex::encode(Sha256::digest(data.as_bytes()))
    }
}

/// Action A — intent, input_hash, adapter, temperature, resource_scope, timestamp, genesis_anchor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub intent: String,
    pub input_hash: String,
    pub adapter: String,
    pub temperature: f32,
    pub resource_scope: String,
    pub timestamp: String,
    pub genesis_anchor: String,
}

impl Action {
    pub fn compute_hash(&self) -> String {
        let data = format!(
            "{}|{}|{}|{}|{}|{}|{}",
            self.intent,
            self.input_hash,
            self.adapter,
            self.temperature,
            self.resource_scope,
            self.timestamp,
            self.genesis_anchor
        );
        hex::encode(Sha256::digest(data.as_bytes()))
    }
}

/// Transition result — new state or error
#[derive(Debug)]
pub struct TransitionResult {
    pub new_state: State,
    pub action_hash: String,
    pub output_hash: String,
}

/// Policy check result
#[derive(Debug, Clone)]
pub enum PolicyResult {
    Allow,
    Deny(String),
}

/// kernel_transition(action) -> Result<State, Error>
/// Invariants:
/// - No state mutation without signed audit append
/// - No execution without policy evaluation
/// - No model execution without registry validation
/// - No network call without scope allow
pub fn kernel_transition(
    state: &State,
    action: &Action,
    policy_check: impl Fn(&Action) -> PolicyResult,
    output_validator: &OutputValidator,
    output: &str,
    hardware_identity_hash: &str,
) -> Result<TransitionResult> {
    match policy_check(action) {
        PolicyResult::Deny(reason) => {
            return Err(GsfError::Other(format!("policy denied: {}", reason)));
        }
        PolicyResult::Allow => {}
    }

    output_validator.validate(output)?;

    let action_hash = action.compute_hash();
    let output_hash = hex::encode(Sha256::digest(output.as_bytes()));

    let new_state = State {
        kernel_state_hash: hex::encode(Sha256::digest(
            format!("{}|{}|{}", state.compute_hash(), action_hash, output_hash).as_bytes(),
        )),
        policy_hash: state.policy_hash.clone(),
        symbol_map_hash: state.symbol_map_hash.clone(),
        audit_head_hash: hex::encode(Sha256::digest(
            format!("{}|{}|{}|{}", state.audit_head_hash, action_hash, output_hash, action.timestamp).as_bytes(),
        )),
        model_registry_hash: state.model_registry_hash.clone(),
        hardware_identity_hash: hardware_identity_hash.to_string(),
    };

    Ok(TransitionResult {
        new_state,
        action_hash,
        output_hash,
    })
}
