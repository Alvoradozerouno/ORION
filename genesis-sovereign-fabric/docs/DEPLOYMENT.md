# GENESIS SOVEREIGN FABRIC — Deployment Guide

## Voraussetzungen

- Rust 1.70+
- Python 3.10+ (SDK)
- Kubernetes 1.28+ (Produktion)
- PostgreSQL 14+ (optional)

## Laptop / Development

```bash
cd genesis-sovereign-fabric
cargo build --release
./target/release/gsf-api --config config/local.yaml
```

## Docker

```bash
docker build -t gsf/core:latest .
docker run -p 8765:8765 -v ./data:/app/data gsf/core:latest
```

## Kubernetes

```bash
helm repo add gsf https://charts.genesis-sovereign-fabric.io
helm install gsf gsf/genesis-sovereign-fabric -f values-prod.yaml
```

## Air-Gapped

1. Images in internes Registry pushen
2. `values-air-gapped.yaml` verwenden
3. Vault für Secrets
4. Kein externer Netzwerkzugriff

## Verifizierung

```bash
./scripts/verify_chain.sh
./scripts/export_audit.sh --format sha256
```
