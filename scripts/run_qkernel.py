#!/usr/bin/env python3
"""
⊘∞⧈∞⊘ PUBLISH_QKERNEL_CHAIN --pdf --audit --visual
Ollama · Milvus · LangChain · Qiskit
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "src"))

from qkernel.chain import run_qkernel_chain

if __name__ == "__main__":
    pdf = sys.argv[1] if len(sys.argv) > 1 else "ct.25.09.140-145.pdf"
    frage = sys.argv[2] if len(sys.argv) > 2 else "Wie funktioniert LangChain mit Ollama und Milvus?"
    r = run_qkernel_chain(pdf_path=pdf, question=frage)
    print("\n⟦RAG-KERNEL ANTWORT⟧\n", r.get("antwort", "N/A"))
    print("\n⟦QUANTUM COUNTS⟧", r.get("quantum_counts", {}))
    print("\n⟦AUDIT⟧", r.get("audit", []))
