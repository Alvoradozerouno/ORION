//! Prometheus metrics — policy_check_duration, audit_append_duration, replay_duration, mesh_sync_latency

use std::sync::atomic::{AtomicU64, Ordering};

static POLICY_CHECK_DURATION_US: AtomicU64 = AtomicU64::new(0);
static AUDIT_APPEND_DURATION_US: AtomicU64 = AtomicU64::new(0);
static REPLAY_DURATION_US: AtomicU64 = AtomicU64::new(0);
static MESH_SYNC_LATENCY_US: AtomicU64 = AtomicU64::new(0);
static RUN_REQUESTS_TOTAL: AtomicU64 = AtomicU64::new(0);
static RUN_REQUESTS_DENIED: AtomicU64 = AtomicU64::new(0);

pub fn record_policy_check_duration_us(us: u64) {
    POLICY_CHECK_DURATION_US.store(us, Ordering::Relaxed);
}

pub fn record_audit_append_duration_us(us: u64) {
    AUDIT_APPEND_DURATION_US.store(us, Ordering::Relaxed);
}

pub fn record_replay_duration_us(us: u64) {
    REPLAY_DURATION_US.store(us, Ordering::Relaxed);
}

pub fn record_mesh_sync_latency_us(us: u64) {
    MESH_SYNC_LATENCY_US.store(us, Ordering::Relaxed);
}

pub fn inc_run_requests_total() {
    RUN_REQUESTS_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn inc_run_requests_denied() {
    RUN_REQUESTS_DENIED.fetch_add(1, Ordering::Relaxed);
}

pub fn metrics_prometheus() -> String {
    format!(
        r#"# HELP gsf_policy_check_duration_us Policy check duration in microseconds (last)
# TYPE gsf_policy_check_duration_us gauge
gsf_policy_check_duration_us {}
# HELP gsf_audit_append_duration_us Audit append duration in microseconds (last)
# TYPE gsf_audit_append_duration_us gauge
gsf_audit_append_duration_us {}
# HELP gsf_replay_duration_us Replay duration in microseconds (last)
# TYPE gsf_replay_duration_us gauge
gsf_replay_duration_us {}
# HELP gsf_mesh_sync_latency_us Mesh sync latency in microseconds (last)
# TYPE gsf_mesh_sync_latency_us gauge
gsf_mesh_sync_latency_us {}
# HELP gsf_run_requests_total Total /run requests
# TYPE gsf_run_requests_total counter
gsf_run_requests_total {}
# HELP gsf_run_requests_denied Denied /run requests (policy)
# TYPE gsf_run_requests_denied counter
gsf_run_requests_denied {}
"#,
        POLICY_CHECK_DURATION_US.load(Ordering::Relaxed),
        AUDIT_APPEND_DURATION_US.load(Ordering::Relaxed),
        REPLAY_DURATION_US.load(Ordering::Relaxed),
        MESH_SYNC_LATENCY_US.load(Ordering::Relaxed),
        RUN_REQUESTS_TOTAL.load(Ordering::Relaxed),
        RUN_REQUESTS_DENIED.load(Ordering::Relaxed),
    )
}
