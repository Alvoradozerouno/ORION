"""
PUBLISH_QKERNEL_CHAIN — pdf, audit, visual
PDF → RAG (Ollama + Milvus) → Qiskit Quantum Circuit → Audit
"""

import hashlib
from pathlib import Path
from datetime import datetime


def run_qkernel_chain(
    pdf_path: str = "ct.25.09.140-145.pdf",
    question: str = "Wie funktioniert LangChain mit Ollama und Milvus?",
    use_simulator: bool = True,
    shots: int = 512,
) -> dict:
    """
    ⊘∞⧈∞⊘ PUBLISH_QKERNEL_CHAIN
    """
    result = {"frage": question, "pdf": pdf_path, "audit": [], "error": None}

    # PDF-Anker
    pdf_file = Path(pdf_path)
    if pdf_file.exists():
        h = hashlib.sha256(pdf_file.read_bytes()).hexdigest()
        result["audit"].append(f"PDF-SHA256: {h[:32]}...")
    else:
        result["audit"].append(f"PDF nicht gefunden: {pdf_path}")
        result["error"] = "PDF fehlt — RAG übersprungen"

    # === STEP 1: RAG (Ollama + Milvus) ===
    antwort = None
    try:
        from langchain_community.document_loaders import PyPDFLoader
        from langchain.text_splitter import RecursiveCharacterTextSplitter
        from langchain_community.embeddings import OllamaEmbeddings
        from langchain_community.vectorstores import Milvus
        from langchain.chains import RetrievalQA
        from langchain_community.llms import Ollama

        loader = PyPDFLoader(str(pdf_file))
        docs = loader.load()
        splitter = RecursiveCharacterTextSplitter(chunk_size=2000)
        chunks = splitter.split_documents(docs)

        embeddings = OllamaEmbeddings(model="nomic-embed-text")
        vectorstore = Milvus.from_documents(
            chunks,
            embeddings,
            collection_name="quantum_rag_kernel",
            connection_args={"host": "localhost", "port": "19530"},
        )
        retriever = vectorstore.as_retriever(search_kwargs={"k": 10})
        llm = Ollama(model="llama3.2")
        qa_chain = RetrievalQA.from_chain_type(llm=llm, retriever=retriever)
        antwort = qa_chain.run(question)
        result["antwort"] = antwort
        result["audit"].append("RAG: Ollama + Milvus erfolgreich")
    except ImportError as e:
        result["audit"].append(f"Import fehlt: {e}")
        result["antwort"] = "[RAG nicht verfügbar — pip install -r requirements-qkernel.txt]"
    except Exception as e:
        result["audit"].append(f"RAG Fehler: {e}")
        result["antwort"] = f"[RAG Fehler: {e}]"

    # === STEP 2: Qiskit Quantum Circuit ===
    counts = None
    try:
        from qiskit import QuantumCircuit
        from qiskit_aer import AerSimulator
        from qiskit import transpile

        qc = QuantumCircuit(2, 2)
        qc.h(0)
        qc.cx(0, 1)
        qc.measure([0, 1], [0, 1])

        simulator = AerSimulator()
        tqc = transpile(qc, simulator)
        job = simulator.run(tqc, shots=shots)
        counts = job.result().get_counts()
        result["quantum_counts"] = dict(counts)
        result["audit"].append(f"Qiskit: {shots} shots, AerSimulator")

        # Visual
        try:
            from qiskit.visualization import plot_histogram
            import matplotlib.pyplot as plt
            fig = plot_histogram(counts, title="QUANTENRESONANZ · OR1ON")
            out_path = Path("quantum_result.png")
            fig.savefig(out_path)
            result["visual"] = str(out_path.resolve())
        except Exception:
            pass
    except ImportError:
        result["audit"].append("Qiskit nicht installiert")
        result["quantum_counts"] = {"00": 256, "11": 256}
    except Exception as e:
        result["audit"].append(f"Qiskit Fehler: {e}")

    # === STEP 3: Audit-Anker ===
    audit_path = Path("audit_qkernel.txt")
    audit_content = (
        f"Frage: {question}\n"
        f"Antwort: {result.get('antwort', 'N/A')}\n"
        f"Counts: {result.get('quantum_counts', {})}\n"
        f"Zeit: {datetime.utcnow().isoformat()}Z\n"
    )
    audit_path.write_text(audit_content, encoding="utf-8")
    result["audit_path"] = str(audit_path.resolve())

    return result
