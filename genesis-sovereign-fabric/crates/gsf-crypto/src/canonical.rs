//! Deterministic Signature Mode — canonical field ordering, no timestamp drift in scope

use serde::Serialize;
use std::collections::BTreeMap;

/// Canonical JSON: keys sorted, stable encoding. No timestamp in signature scope.
/// Payload format: genesis|prev_hash|entry_hash|intent|pattern|decision (fields in order)
pub fn canonical_sign_payload(
    genesis_anchor: &str,
    prev_hash: &str,
    entry_hash: &str,
    intent: &str,
    pattern: &str,
    decision: &str,
) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}",
        genesis_anchor, prev_hash, entry_hash, intent, pattern, decision
    )
}

/// Canonical JSON serialization for arbitrary structs — BTreeMap for stable key order
pub fn to_canonical_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let v = serde_json::to_value(value)?;
    canonical_value_to_string(&v)
}

fn canonical_value_to_string(v: &serde_json::Value) -> Result<String, serde_json::Error> {
    match v {
        serde_json::Value::Null => Ok("null".to_string()),
        serde_json::Value::Bool(b) => Ok(b.to_string()),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::String(s) => Ok(serde_json::to_string(s)?),
        serde_json::Value::Array(arr) => {
            let parts: Result<Vec<_>, _> =
                arr.iter().map(|e| canonical_value_to_string(e)).collect();
            Ok(format!("[{}]", parts?.join(",")))
        }
        serde_json::Value::Object(obj) => {
            let mut sorted: BTreeMap<&str, &serde_json::Value> = BTreeMap::new();
            for (k, v) in obj {
                sorted.insert(k, v);
            }
            let parts: Result<Vec<String>, _> = sorted
                .iter()
                .map(|(k, v)| {
                    Ok(format!(
                        "{}:{}",
                        serde_json::to_string(k)?,
                        canonical_value_to_string(v)?
                    ))
                })
                .collect();
            Ok(format!("{{{}}}", parts?.join(",")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_payload_deterministic() {
        let p1 = canonical_sign_payload("a", "b", "c", "i", "p", "d");
        let p2 = canonical_sign_payload("a", "b", "c", "i", "p", "d");
        assert_eq!(p1, p2);
        assert_eq!(p1, "a|b|c|i|p|d");
    }
}
