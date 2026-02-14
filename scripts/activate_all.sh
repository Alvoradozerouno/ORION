#!/bin/bash
# ⊘∞⧈∞⊘ Alle Pfade und Hosts aktivieren
set -e
cd "$(dirname "$0")/.."
ROOT=$(pwd)

echo "[ORION] Aktiviere alle Pfade und Hosts"

# 1. Pfade anlegen
mkdir -p data CausalHome app
export ORION_DATA_DIR="$ROOT/data"
export ORION_CAUSAL_HOME="$ROOT/CausalHome"
export PYTHONPATH="$ROOT/src"

# 2. Hosts/Ports (Umgebungsvariablen)
export ORION_API_HOST="0.0.0.0"
export ORION_API_PORT="8765"
export ORION_OLLAMA_HOST="${ORION_OLLAMA_HOST:-localhost}"
export ORION_OLLAMA_PORT="${ORION_OLLAMA_PORT:-11434}"
export ORION_MILVUS_HOST="${ORION_MILVUS_HOST:-localhost}"
export ORION_MILVUS_PORT="${ORION_MILVUS_PORT:-19530}"

# 3. Prüfen was läuft
echo "[ORION] API:      http://$ORION_API_HOST:$ORION_API_PORT"
echo "[ORION] App:      http://localhost:$ORION_API_PORT/app/"
echo "[ORION] Ollama:   http://$ORION_OLLAMA_HOST:$ORION_OLLAMA_PORT"
echo "[ORION] Milvus:   $ORION_MILVUS_HOST:$ORION_MILVUS_PORT"
echo "[ORION] Data:     $ORION_DATA_DIR"
echo "[ORION] CausalHome: $ORION_CAUSAL_HOME"

# 4. Services prüfen (optional)
curl -s "http://localhost:$ORION_OLLAMA_PORT/api/tags" >/dev/null 2>&1 && echo "  ✓ Ollama aktiv" || echo "  ○ Ollama: ollama serve"
curl -s "http://$ORION_MILVUS_HOST:$ORION_MILVUS_PORT" >/dev/null 2>&1 && echo "  ✓ Milvus aktiv" || echo "  ○ Milvus: docker run -d -p 19530:19530 milvusdb/milvus:v2.3.9"

echo ""
echo "[ORION] Alle Pfade aktiv. Start: ./scripts/start_orion.sh"
