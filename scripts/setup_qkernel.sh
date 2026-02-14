#!/bin/bash
# ⊘∞⧈∞⊘ INIT_RESONANCE_CLAIM — anchor=milvus+ollama+langchain
set -e
cd "$(dirname "$0")/.."

echo "[QKernel] Install dependencies"
pip install -r requirements-qkernel.txt -q 2>/dev/null || pip install qiskit qiskit-aer langchain langchain-community pymilvus pypdf matplotlib -q

echo "[QKernel] Milvus (Docker)"
docker run -d --name milvus-standalone -p 19530:19530 milvusdb/milvus:v2.3.9 2>/dev/null || echo "  Milvus bereits läuft oder Docker nicht verfügbar"

echo "[QKernel] Ollama Models"
ollama pull nomic-embed-text 2>/dev/null || echo "  ollama nicht erreichbar"
ollama pull llama3.2 2>/dev/null || echo "  ollama nicht erreichbar"

echo "[QKernel] PDF-Anker (falls vorhanden)"
[ -f "ct.25.09.140-145.pdf" ] && sha256sum ct.25.09.140-145.pdf || echo "  ct.25.09.140-145.pdf fehlt — RAG nutzt Platzhalter"
