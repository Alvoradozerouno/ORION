//! Layer 7 — Benchmark harness

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gsf_core::{AuditChain, SymbolMap, WorkflowEngine, GENESIS_ANCHOR};

fn bench_append_event(c: &mut Criterion) {
    c.bench_function("append_event", |b| {
        b.iter(|| {
            let mut eng = WorkflowEngine::new();
            eng.symbol_map.register("ping", "pong", None);
            black_box(eng.execute("BENCH", "ping").unwrap());
        })
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

criterion_group!(benches, bench_append_event, bench_policy_check, bench_chain_verify);
criterion_main!(benches);
