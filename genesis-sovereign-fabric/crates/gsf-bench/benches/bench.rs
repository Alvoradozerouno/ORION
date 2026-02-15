//! Layer 7 — Benchmark harness
//! Targets: policy < 500µs, audit append < 2ms, replay 10k < 1s

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gsf_core::{replay, AuditChain, AuditEntry, WorkflowEngine, GENESIS_ANCHOR};
use sha2::Digest;

fn skip_verify(_: &AuditEntry) -> bool {
    true
}

fn bench_append_event(c: &mut Criterion) {
    c.bench_function("append_event", |b| {
        b.iter(|| {
            let mut eng = WorkflowEngine::new();
            eng.symbol_map.register("ping", "pong", None);
            black_box(eng.execute("BENCH", "ping").unwrap());
        })
    });
}

fn bench_audit_append_latency(c: &mut Criterion) {
    let mut eng = WorkflowEngine::new();
    eng.symbol_map.register("ping", "pong", None);
    c.bench_function("audit_append_latency", |b| {
        b.iter(|| black_box(eng.append_event("BENCH", "ping", "pong", None).unwrap()))
    });
}

fn bench_policy_check(c: &mut Criterion) {
    let yaml = r#"
version: "1.0"
scope:
  hardware: false
  network_outbound: false
  fs_write_paths: ["data/"]
invariante:
  blocked_patterns: ["rm -rf"]
  deny_on_match: true
rules: []
"#;
    let policy = gsf_policy::parse_policy(yaml).unwrap();
    let ph = gsf_policy::policy_hash(yaml);
    let action = gsf_policy::Action::for_run("TEST", "ping", GENESIS_ANCHOR);

    c.bench_function("policy_check", |b| {
        b.iter(|| black_box(gsf_policy::check(&action, &policy, &ph)))
    });
}

fn bench_chain_verify(c: &mut Criterion) {
    let mut chain = AuditChain::new();
    for i in 0..100 {
        chain.append("intent", &format!("p{}", i), "decision", None);
    }

    c.bench_function("chain_verify_100", |b| b.iter(|| black_box(chain.verify())));
}

fn bench_replay_10k(c: &mut Criterion) {
    let mut chain = Vec::with_capacity(10_000);
    let mut prev = GENESIS_ANCHOR.to_string();
    for i in 0..10_000 {
        let ts = chrono::Utc::now().to_rfc3339();
        let data = format!("{}|intent|p{}|decision||{}", ts, i, prev);
        let entry_hash = hex::encode(sha2::Sha256::digest(data.as_bytes()));
        chain.push(AuditEntry {
            timestamp: ts,
            intent: "intent".to_string(),
            pattern: format!("p{}", i),
            decision: "decision".to_string(),
            outcome: None,
            prev_hash: prev.clone(),
            entry_hash: entry_hash.clone(),
        });
        prev = entry_hash;
    }
    let from = chain.first().map(|e| e.entry_hash.as_str()).unwrap_or(GENESIS_ANCHOR);
    let to = chain.last().map(|e| e.entry_hash.as_str()).unwrap_or(GENESIS_ANCHOR);

    c.bench_function("replay_10k_entries", |b| {
        b.iter(|| {
            black_box(replay(&chain, from, to, GENESIS_ANCHOR, Some(skip_verify)).unwrap())
        })
    });
}

fn bench_determinism_harness(c: &mut Criterion) {
    let mut eng = WorkflowEngine::new();
    eng.symbol_map.register("ping", "pong", None);
    let mut hashes = Vec::with_capacity(10);
    for _ in 0..10 {
        let mut e = WorkflowEngine::new();
        e.symbol_map.register("ping", "pong", None);
        let h = e.execute("DET", "ping").unwrap();
        hashes.push(h);
    }
    let first = &hashes[0];
    assert!(hashes.iter().all(|h| h == first), "determinism violated");
    c.bench_function("determinism_check_10x", |b| {
        b.iter(|| {
            let mut e = WorkflowEngine::new();
            e.symbol_map.register("ping", "pong", None);
            black_box(e.execute("DET", "ping").unwrap())
        })
    });
}

criterion_group!(
    benches,
    bench_append_event,
    bench_audit_append_latency,
    bench_policy_check,
    bench_chain_verify,
    bench_replay_10k,
    bench_determinism_harness
);
criterion_main!(benches);
