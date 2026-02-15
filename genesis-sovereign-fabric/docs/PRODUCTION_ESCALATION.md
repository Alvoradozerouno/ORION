# GSF Production Escalation

## Phase 1 — Cryptographic Hardening ✅

- **KeyStore** (`gsf-crypto`): `LocalKeyStore`, `KeyVersionId`, `RevocationList`, CRL support
- **Vault Integration**: `VaultSigner` (feature `vault`), Transit engine, fail-closed
- **Canonical Signing**: `canonical_sign_payload()` — stable field order, no timestamp in scope

## Phase 2 — Mesh Security

- mTLS: Design in `gsf-mesh`, rustls-based implementation pending
- Mesh Identity Registry: Approved peer list in protocol
- Fork Resolution: `deterministic_merge`, longest valid chain

## Phase 3 — Model Governance

- SBOM: `gsf-registry::SbomGenerator` (CycloneDX)
- Container Signing: cosign integration — CI/Helm
- Model Artifact: `artifact_sha256` mandatory, Ed25519 in registry

## Phase 4 — Observability ✅

- **Structured Logging**: `GSF_JSON_LOGS=1` → tracing JSON
- **Prometheus**: `/metrics` — `gsf_policy_check_duration_us`, `gsf_audit_append_duration_us`, `gsf_mesh_sync_latency_us`, `gsf_run_requests_total`, `gsf_run_requests_denied`
- OpenTelemetry: Span integration pending

## Phase 5 — Formal Testing ✅

- **Determinism Harness**: `bench_determinism_check_10x` — same action N times, hash match
- Property-based: QuickCheck structure in place
- Fuzz: Policy DSL, mesh sync — harness ready

## Phase 6 — Performance ✅

- **Benchmarks**: `policy_check`, `audit_append_latency`, `chain_verify_100`, `replay_10k_entries`, `determinism_check_10x`
- **Targets**: policy < 500µs, audit append < 2ms, replay 10k < 1s
- Chaos: `chaos_chain_verify_50`, `chaos_replay_20`

## Phase 7 — Python SDK ✅

- `execute()`, `verify_chain()`, `monitor_metrics()`, `export_signed_ledger()`, `mesh_join()`
- `AsyncGsfClient`: `execute_async()`, `verify_chain_async()`, `export_chain_async()`, `monitor_metrics_async()`
- Retry logic, TLS verification, timeout

## Phase 8 — Governance ✅

- **Execution Freeze**: `governance.execution_frozen` in kernel_state → 503 on /run
- **Policy Approval**: `governance.approved_policy_hashes` — refuse unknown versions
- **Governance Mode**: `GovernanceMode::ReadOnlyEmergency`

## Phase 9 — Hardware Attestation

- TPM: `gsf-hardware::HardwareIdentity`, `device_seed()`, `sign_genesis_anchor()`
- Remote attestation endpoint: pending

## Environment

| Variable | Purpose |
|---------|---------|
| `GSF_JSON_LOGS=1` | JSON structured logs |
| `GSF_SIGNING_KEY` | Hex Ed25519 seed (32 bytes) |
| `VAULT_ADDR`, `VAULT_TOKEN`, `GSF_VAULT_TRANSIT_KEY` | Vault Transit (feature `vault`) |
| `GSF_DATA_PATH` | SQLite path |
| `GSF_POLICY_PATH` | Policy YAML path |
| `GSF_PORT` | API port (default 8765) |
