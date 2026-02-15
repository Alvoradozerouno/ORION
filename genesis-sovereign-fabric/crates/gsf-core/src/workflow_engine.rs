//! Deterministische Workflow Engine.
//! State Machine. Keine versteckten Zustände.

use crate::{AuditChain, SymbolMap};

pub struct WorkflowEngine {
    pub audit_chain: AuditChain,
    pub symbol_map: SymbolMap,
}

impl WorkflowEngine {
    pub fn new() -> Self {
        Self {
            audit_chain: AuditChain::new(),
            symbol_map: SymbolMap::new(),
        }
    }

    pub fn execute(&mut self, intent: &str, pattern: &str) -> Result<String, String> {
        let signal = self
            .symbol_map
            .collapse(pattern)
            .map(|s| s.signal.clone())
            .unwrap_or_else(|| "no_collapse".to_string());
        let entry = self.audit_chain.append(intent, pattern, &signal, None);
        Ok(entry.entry_hash)
    }
}
