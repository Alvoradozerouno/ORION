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
| /audit/verify | GET | Chain verification |
| /audit/export | GET | Export chain as JSON |
| /run | POST | Execute intent/pattern |

## Python SDK

```python
from gsf_sdk import GsfClient
client = GsfClient("http://localhost:8765")
print(client.health())
print(client.verify_audit())
```

## Helm

```bash
helm install gsf ./helm/genesis-sovereign-fabric -n gsf --create-namespace
```

## Docs

- [Architecture](../docs/GENESIS_SOVEREIGN_FABRIC_ARCHITECTURE.md)
- [Threat Model](docs/THREAT_MODEL.md)
- [Mesh Design](docs/MESH_DESIGN.md)
- [Production Deployment](docs/PRODUCTION_DEPLOYMENT.md)
