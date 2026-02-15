//! No output without validation. Schema enforcement.

use crate::error::{GsfError, Result};
use serde::Deserialize;
use std::collections::HashSet;

/// Validates output against allowed patterns. Rejects non-conforming.
pub struct OutputValidator {
    allowed_patterns: HashSet<String>,
    max_length: usize,
}

impl OutputValidator {
    pub fn new(allowed_patterns: Vec<String>, max_length: usize) -> Self {
        Self {
            allowed_patterns: allowed_patterns.into_iter().collect(),
            max_length,
        }
    }

    pub fn validate(&self, output: &str) -> Result<()> {
        if output.len() > self.max_length {
            return Err(GsfError::Other(format!(
                "output length {} exceeds max {}",
                output.len(),
                self.max_length
            )));
        }
        if !self.allowed_patterns.is_empty() {
            let ok = self
                .allowed_patterns
                .iter()
                .any(|p| output.contains(p) || p == "*");
            if !ok {
                return Err(GsfError::Other("output failed pattern validation".to_string()));
            }
        }
        Ok(())
    }
}

/// JSON schema validation placeholder — extend with jsonschema crate
pub fn validate_json_schema<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T> {
    serde_json::from_str(json).map_err(|e| GsfError::Other(e.to_string()))
}
