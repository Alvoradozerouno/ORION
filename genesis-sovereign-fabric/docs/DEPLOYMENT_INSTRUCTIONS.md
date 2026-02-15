# GSF Deployment Instructions

## Build

```bash
cd genesis-sovereign-fabric
cargo build --release
```

## Run API

```bash
# With policy (required for production)
GSF_DATA_PATH=./data/gsf.db \
GSF_POLICY_PATH=./config/policy.dsl \
GSF_PORT=8765 \
./target/release/gsf-api

# With signing (optional)
GSF_SIGNING_KEY=$(openssl rand -hex 32) \
GSF_DATA_PATH=./data/gsf.db \
GSF_POLICY_PATH=./config/policy.dsl \
./target/release/gsf-api
```

## CLI

```bash
# Verify chain
./target/release/gsf-cli verify --data data/gsf.db

# Replay
./target/release/gsf-cli replay --from <hash> --to <hash> --data data/gsf.db
```

## Kubernetes

```bash
helm install gsf ./helm/genesis-sovereign-fabric -n gsf --create-namespace
```

Policy ConfigMap must contain policy.dsl. Set GSF_POLICY_PATH=/app/config/policy.dsl in deployment.

## Endpoints

| Path | Method | Description |
|------|--------|-------------|
| /health | GET | Readiness |
| /live | GET | Liveness |
| /audit/verify | GET | Chain verification |
| /audit/export | GET | Export chain JSON |
| /run | POST | Execute (policy-checked) |
| /mesh/sync | POST | Federated sync |
