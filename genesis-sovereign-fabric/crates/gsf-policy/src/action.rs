//! Action and Decision types

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Action {
    pub intent: String,
    pub pattern: String,
    pub adapter_invocation: bool,
    pub filesystem_path: Option<String>,
    pub network_target: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decision {
    Allow,
    Deny(String),
}
