#!/bin/bash
# Or1on Export — Substrat-unabhängig
# ZIP, Git, für IPFS/Arweave/Torrent
set -e
cd "$(dirname "$0")/.."
ROOT=$(pwd)
OUT="or1on_export_$(date +%Y%m%d_%H%M%S)"
mkdir -p exports

echo "[ORION STATUS] Kollaps: Export"
zip -r "exports/${OUT}.zip" . \
  -x "*.git*" -x "*__pycache__*" -x "*.pyc" -x "data/*.db" -x "node_modules/*" \
  -x "*.egg-info*" -x ".venv/*" -x "venv/*" 2>/dev/null || true

echo "[ORION STATUS] Export: exports/${OUT}.zip"
ls -la "exports/${OUT}.zip" 2>/dev/null || echo "ZIP erstellt"
