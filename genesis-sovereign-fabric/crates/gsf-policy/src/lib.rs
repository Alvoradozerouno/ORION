//! Policy DSL — YAML-based, scope, invariants, rules

mod action;
mod policy;

pub use action::{Action, Decision};
pub use policy::{check, parse_policy, Invariante, Policy, Rule, Scope};