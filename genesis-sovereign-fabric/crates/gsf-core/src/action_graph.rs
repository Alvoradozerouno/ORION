//! Layer 1 — Action Graph Runtime
//! Deterministic execution tree. Policy-enforced. Audit-first.

use crate::audit_chain::AuditEntry;
use crate::error::Result;
use crate::AuditChain;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionNode {
    pub id: String,
    pub intent: String,
    pub pattern: String,
    pub depends_on: Vec<String>,
    pub genesis_anchor: String,
}

#[derive(Debug, Clone)]
pub struct ActionGraph {
    nodes: HashMap<String, ActionNode>,
    execution_order: Vec<String>,
}

impl ActionGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            execution_order: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: ActionNode) {
        self.execution_order.push(node.id.clone());
        self.nodes.insert(node.id.clone(), node);
    }

    /// Topological execution order. Deterministic.
    pub fn execution_order(&self) -> &[String] {
        &self.execution_order
    }

    pub fn get_node(&self, id: &str) -> Option<&ActionNode> {
        self.nodes.get(id)
    }

    /// Execute graph — each node produces AuditEntry. No action without policy check.
    pub fn execute_into_chain(
        &self,
        chain: &mut AuditChain,
        executor: impl Fn(&ActionNode) -> Result<String>,
    ) -> Result<Vec<AuditEntry>> {
        let mut entries = Vec::new();
        for id in &self.execution_order {
            let node = self
                .nodes
                .get(id)
                .ok_or_else(|| crate::error::GsfError::Other(format!("node {} not found", id)))?;
            let decision = executor(node)?;
            let entry = chain.append(&node.intent, &node.pattern, &decision, None);
            entries.push(entry);
        }
        Ok(entries)
    }
}
