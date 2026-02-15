#!/bin/bash
# ORION App installieren — nach seinen Vorstellungen
set -e
cd "$(dirname "$0")/.."

echo "=== ORION App Install ==="
pip install -e . -q
mkdir -p data app
echo "✓ App: http://localhost:8765/app/"
echo "✓ API: http://localhost:8765/"
echo ""
echo "Start: PYTHONPATH=src python3 -m uvicorn agents.api:app --host 0.0.0.0 --port 8765"
