# GENESIS SOVEREIGN FABRIC — Threat Model

## Assets

- AuditChain (immutable decision log)
- Signing keys (Ed25519)
- Policy configuration
- Persistence (SQLite/PostgreSQL)

## Threats

| Threat | Mitigation |
|--------|------------|
| AuditChain tampering | SHA256 chain, append-only, verify on load |
| Key compromise | GSF_SIGNING_KEY in ENV/Vault only, no plaintext |
| Policy bypass | Policy check before every decision |
| Injection in LLM output | Output validation, schema enforcement |
| Supply chain | SBOM, signed images, air-gap option |
| Network exfiltration | No outbound in core; adapters isolated |
| Privilege escalation | runAsNonRoot, readOnlyRootFilesystem |

## Trust Boundaries

- Core: No network. No filesystem outside data/.
- Adapters: Network only when explicitly configured.
- API: Token auth optional. Rate limit.

## Assumptions

- Kernel is not compromised
- Secrets are in Vault or ENV
- No dynamic code execution

## Policy Enforcement

- Policy check before every /run request
- Denied requests: 403, entry appended with decision=denied
- Policy loaded from GSF_POLICY_PATH or config/policy.dsl
- Cannot bypass: check() is called in request path

## GP-AI Layers

| Layer | Mitigation |
|-------|------------|
| 1 Kernel | Action graph, output validation, genesis anchor |
| 2 Fabric | Resource limits, temperature cap |
| 3 Registry | Model SHA256, SBOM |
| 4 Mesh | mTLS, conflict detection, fork resolution |
| 5 Learning | Dataset hash, training audit |
| 6 Hardware | TPM binding optional |
| 7 Bench | Latency, throughput, replay verification |
