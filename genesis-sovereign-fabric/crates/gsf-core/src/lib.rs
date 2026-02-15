//! GENESIS SOVEREIGN FABRIC — Core
//! Deterministic Workflow, AuditChain, SymbolMap, Persistence, Signed Ledger

pub mod action_graph;
pub mod audit_chain;
pub mod error;
pub mod fork_resolution;
pub mod governance;
pub mod ledger;
pub mod output_validation;
pub mod persistence;
pub mod replay_engine;
pub mod signed_ledger;
pub mod state_machine;
pub mod symbol_map;
pub mod workflow_engine;

pub use action_graph::{ActionGraph, ActionNode};
pub use audit_chain::{AuditChain, AuditEntry, GENESIS_ANCHOR};
pub use signed_ledger::SignedAuditEntry;
pub use state_machine::{Action as StateAction, PolicyResult, State, TransitionResult, kernel_transition};
pub use error::{GsfError, Result};
pub use fork_resolution::{append_best_chain, select_longest_valid};
pub use governance::{is_execution_frozen, is_policy_approved, set_execution_frozen, GovernanceMode, KEY_EXECUTION_FROZEN};
pub use ledger::{LedgerSigner, SignedEntry};
pub use output_validation::{OutputValidator, validate_json_schema};
pub use persistence::Persistence;
pub use replay_engine::{replay, replay_legacy, KernelState};
pub use symbol_map::{Symbol, SymbolMap};
pub use workflow_engine::WorkflowEngine;
