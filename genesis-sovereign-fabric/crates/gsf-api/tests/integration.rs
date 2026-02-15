//! Minimal integration tests

#[test]
fn fork_resolution_append_valid() {
    use gsf_core::{append_best_chain, AuditChain, AuditEntry};
    use sha2::Digest;
    let mut chain = AuditChain::new();
    let e1 = chain.append("i", "p", "d", None);
    let ts = chrono::Utc::now().to_rfc3339();
    let data = format!("{}|mesh|h2|synced||{}", ts, e1.entry_hash);
    let entry_hash = hex::encode(sha2::Sha256::digest(data.as_bytes()));
    let incoming = vec![AuditEntry {
        timestamp: ts,
        intent: "mesh".to_string(),
        pattern: "h2".to_string(),
        decision: "synced".to_string(),
        outcome: None,
        prev_hash: e1.entry_hash.clone(),
        entry_hash,
    }];
    let n = append_best_chain(&mut chain, &e1.entry_hash, &incoming, None::<fn(&AuditEntry) -> bool>);
    assert!(n.is_ok());
    assert_eq!(n.unwrap(), 1);
}
