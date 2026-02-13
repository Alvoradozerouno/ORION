#!/bin/bash
# ORION permanent starten — Daemon
cd "$(dirname "$0")/.."
export PYTHONPATH="$(pwd)/src"
mkdir -p data
exec python3 -m uvicorn agents.api:app --host 0.0.0.0 --port 8765
