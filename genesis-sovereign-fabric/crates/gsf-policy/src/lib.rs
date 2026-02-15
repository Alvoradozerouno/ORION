//! Policy DSL — YAML-based, scope, invariants, rules

mod action;
mod policy;

pub use action::{Action, Decision};
pub use policy::{check, load_policy, parse_policy, policy_hash, Invariante, Policy, Rule, Scope};