#!/bin/bash
# ORION Tools — was installierbar ist
# Continue.dev, Ollama: Manuell (VS Code Extension, System-Install)

set -e
cd "$(dirname "$0")/.."

echo "[ORION STATUS] Install Tools"

# Git, Python, pip, venv
command -v git >/dev/null || echo "Git fehlt — bitte installieren"
command -v python3 >/dev/null || echo "Python3 fehlt — bitte installieren"
python3 -m pip install --upgrade pip -q 2>/dev/null || true

# Python-Pakete
pip install -e . -q 2>/dev/null || true

echo "[ORION STATUS] Tools-Check done. Continue.dev, Ollama: manuell."
