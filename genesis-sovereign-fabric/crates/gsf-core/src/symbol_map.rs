//! Deterministische Entscheidungs-Matrix.
//! Pattern → Signal. Kein LLM. Lookup only.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: String,
    pub pattern: String,
    pub signal: String,
    pub causal_links: Vec<String>,
}

pub struct SymbolMap {
    pattern_to_id: HashMap<String, String>,
    symbols: HashMap<String, Symbol>,
}

impl SymbolMap {
    pub fn new() -> Self {
        Self {
            pattern_to_id: HashMap::new(),
            symbols: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        pattern: &str,
        signal: &str,
        links: Option<Vec<String>>,
    ) -> Symbol {
        let id = format!("sym_{}", self.symbols.len());
        let s = Symbol {
            id: id.clone(),
            pattern: pattern.to_string(),
            signal: signal.to_string(),
            causal_links: links.unwrap_or_default(),
        };
        self.pattern_to_id
            .insert(pattern.to_string(), id.clone());
        self.symbols.insert(id, s.clone());
        s
    }

    /// Deterministischer Lookup. Kein Zufall.
    pub fn collapse(&self, pattern: &str) -> Option<&Symbol> {
        self.pattern_to_id
            .get(pattern)
            .and_then(|id| self.symbols.get(id))
    }

    pub fn restore_symbol(&mut self, s: Symbol) {
        self.pattern_to_id
            .insert(s.pattern.clone(), s.id.clone());
        self.symbols.insert(s.id.clone(), s);
    }
}
