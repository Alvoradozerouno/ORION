# Federated Mesh Layer — Design

## Overview

Cross-node audit sync with signature verification. Zero trust.

## Protocol

1. **Export**: Node A exports signed AuditChain entries (entry_hash, signature, prev_hash).
2. **Sync**: Node B receives entries over authenticated channel (mTLS).
3. **Verify**: Node B verifies each signature with A's public key.
4. **Append**: Node B appends only if chain continuity holds (prev_hash matches).
5. **Conflict**: If prev_hash mismatch, reject. No merge. Eventual consistency via deterministic ordering.

## Data Format

```json
{
  "genesis_anchor": "sha256:acb92fd...",
  "entries": [
    {
      "prev_hash": "...",
      "entry_hash": "...",
      "signature": "...",
      "timestamp": "..."
    }
  ],
  "node_id": "A",
  "public_key": "..."
}
```

## Conflict Resolution

- No automatic merge
- Last-write-wins per entry_hash
- Reject if prev_hash does not match local chain tail

## Security

- mTLS between nodes
- Public keys in registry (ConfigMap or Vault)
- No trust in payload without valid signature
