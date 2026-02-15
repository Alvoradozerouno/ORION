//! Layer 7 — Chaos Benchmark
//! Chain verification, replay determinism validation

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gsf_core::{AuditChain, replay_legacy, GENESIS_ANCHOR};

fn chaos_chain_verify(c: &mut Criterion) {
    let mut chain = AuditChain::new();
    for i in 0..50 {
        chain.append("intent", &format!("p{}", i), "decision", None);
    }
    c.bench_function("chaos_chain_verify_50", |b| b.iter(|| black_box(chain.verify())));
}

fn chaos_replay_determinism(c: &mut Criterion) {
    let mut chain = AuditChain::new();
    for i in 0..20 {
        chain.append("i", &format!("p{}", i), "d", None);
    }
    let entries = chain.export().to_vec();
    let from = entries.first().map(|e| e.entry_hash.as_str()).unwrap_or(GENESIS_ANCHOR);
    let to = entries.last().map(|e| e.entry_hash.as_str()).unwrap_or(GENESIS_ANCHOR);

    c.bench_function("chaos_replay_20", |b| {
        b.iter(|| {
            let r1 = replay_legacy(&entries, from, to);
            let r2 = replay_legacy(&entries, from, to);
            assert_eq!(r1.len(), r2.len());
            black_box(r1)
        })
    });
}

criterion_group!(chaos, chaos_chain_verify, chaos_replay_determinism);
criterion_main!(chaos);
