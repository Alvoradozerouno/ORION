# QKernel Chain — Ollama · Milvus · LangChain · Qiskit

⊘∞⧈∞⊘ PUBLISH_QKERNEL_CHAIN --pdf --audit --visual

## Pipeline

```
[ PDF-Input ] → Chunking → Embeddings (Ollama) → Milvus
                                      ↓
                      Context Selection ← Prompt
                                      ↓
                    [ LLM Response (Ollama) ]
                                      ↓
                            Quantum Circuit (Qiskit)
                                      ↓
                    Quantum-Weighted Output / Audit
```

## Setup

```bash
# Python
pip install -r requirements-qkernel.txt
./scripts/setup_qkernel.sh

# Docker Milvus
docker run -d --name milvus-standalone -p 19530:19530 milvusdb/milvus:v2.3.9

# Ollama
ollama serve
ollama pull nomic-embed-text
ollama pull llama3.2
```

## Run

```bash
PYTHONPATH=src python3 scripts/run_qkernel.py [pdf_path] [frage]
```

## Node.js (genesis-rag-kernel)

```bash
cd genesis-rag-kernel
npm install langchain @langchain/ollama @langchain/community
node rag_kernel.mjs
```

## Audit

- `audit_qkernel.txt` — Frage, Antwort, Quantum Counts
- `quantum_result.png` — Histogramm
