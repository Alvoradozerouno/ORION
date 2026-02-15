# GENESIS SOVEREIGN FABRIC

Auditierbare, deterministische AI-Control-Plane für kritische Infrastruktur.

**Genesis Anchor:** `sha256:acb92fd8346a65ff17dbf9a41e3003f2d566a17f839af4c3a90a4b4b1789dd28a`

## Build

```bash
cargo build --release
```

## Run

```bash
# In-Memory
./target/release/gsf-api

# With Persistence
GSF_DATA_PATH=./data/gsf.db ./target/release/gsf-api

# Custom Port
GSF_PORT=8766 ./target/release/gsf-api
```

## API

| Endpoint | Method | Description |
|----------|--------|--------------|
| /health | GET | Liveness |
| /live | GET | Readiness |
| /metrics | GET | Prometheus metrics |
| /audit/verify | GET | Chain verification |
| /audit/export | GET | Export chain as JSON |
| /run | POST | Execute intent/pattern |
| /mesh/sync | POST | Mesh sync |

## Python SDK

```python
from gsf_sdk import GsfClient, AsyncGsfClient

client = GsfClient("http://localhost:8765")
client.health()
client.verify_chain()
client.execute("intent", "pattern")
client.export_signed_ledger()
client.monitor_metrics()
client.mesh_join(entries)

# Async
async_client = AsyncGsfClient()
await async_client.execute_async("intent", "pattern")
await async_client.monitor_metrics_async()
```

## Helm

```bash
helm install gsf ./helm/genesis-sovereign-fabric -n gsf --create-namespace
```

## Docs

- [Production Escalation](docs/PRODUCTION_ESCALATION.md)
- [Threat Model](docs/THREAT_MODEL.md)
- [Mesh Design](docs/MESH_DESIGN.md)
- [Production Deployment](docs/PRODUCTION_DEPLOYMENT.md)
