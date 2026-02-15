# Production Deployment

## Prerequisites

- Kubernetes 1.28+
- Helm 3
- PersistentVolume provisioner

## Deploy

```bash
helm install gsf ./helm/genesis-sovereign-fabric \
  --set image.repository=your-registry/gsf-core \
  --set image.tag=0.1.0 \
  --set persistence.size=20Gi \
  --create-namespace \
  -n gsf
```

## Secrets

Store GSF_SIGNING_KEY in Vault or Kubernetes Secret. Inject via:

```yaml
env:
  - name: GSF_SIGNING_KEY
    valueFrom:
      secretKeyRef:
        name: gsf-secrets
        key: signing-key
```

## Verification

```bash
kubectl port-forward svc/gsf-genesis-sovereign-fabric 8765:8765 -n gsf
curl http://localhost:8765/health
curl http://localhost:8765/audit/verify
```

## Air-Gapped

1. Push images to internal registry
2. Use values-air-gapped.yaml
3. No external network egress
4. Policy ConfigMap from internal source
