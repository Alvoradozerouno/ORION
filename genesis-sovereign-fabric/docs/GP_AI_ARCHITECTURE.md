# GENESIS SOVEREIGN FABRIC — GP-AI CORE Architecture

## Layer 1 — Kernel Core

- **Action Graph Runtime:** `gsf-core::action_graph` — deterministic execution tree
- **Policy-Enforced:** No action without policy check
- **Audit-First:** No state change without signed ledger append
- **Output Validation:** `gsf-core::output_validation` — no output without validation
- **Genesis Anchor:** Embedded in every state

## Layer 2 — Execution Fabric

- **Task Scheduler:** `gsf-fabric::scheduler` — resource-aware, semaphore-limited
- **Temperature Cap:** `gsf-fabric::temperature_cap` — deterministic enforcement
- **Model Sandbox:** Adapter isolation (Ollama, OpenAI-compatible)

## Layer 3 — Model Governance

- **Model Registry:** `gsf-registry::model_registry` — SHA256, version hashing
- **SBOM:** `gsf-registry::sbom` — CycloneDX format
- **Supply Chain:** Hash-based verification

## Layer 4 — Federated Mesh

- **Protocol:** `gsf-mesh::protocol` — signed ledger sync
- **Conflict Detection:** `gsf-mesh::conflict`
- **Fork Resolution:** Longest valid chain wins
- **mTLS:** Client cert required (peer registry)

## Layer 5 — Learning Pipeline

- **Dataset Registry:** `gsf-learning::dataset_registry` — hash-based
- **Training Audit:** `gsf-learning::training_audit` — param snapshot hashing
- **Reproducible:** All logged

## Layer 6 — Hardware Identity

- **gsf-hardware:** TPM/enclave binding abstraction
- **Device-Bound Keypair:** GSF_TPM_ENDORSEMENT_KEY
- **Genesis Anchor Signature:** Optional hardware sign

## Layer 7 — Benchmarking

- **gsf-bench:** Criterion harness
- **Metrics:** append_event, policy_check, chain_verify
- **Replay Determinism:** Via replay_engine

## Directory Structure

```
genesis-sovereign-fabric/
├── crates/
│   ├── gsf-core/      # Layer 1
│   ├── gsf-policy/
│   ├── gsf-api/
│   ├── gsf-fabric/    # Layer 2
│   ├── gsf-registry/  # Layer 3
│   ├── gsf-mesh/      # Layer 4
│   ├── gsf-learning/   # Layer 5
│   ├── gsf-hardware/  # Layer 6
│   └── gsf-bench/     # Layer 7
├── .github/workflows/ci.yml
├── config/policy.dsl
├── helm/
└── python/gsf_sdk/
```
