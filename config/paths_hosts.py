"""
ORION — Alle Pfade und Hosts
Zentrale Konfiguration. Aktivierbar via Umgebungsvariablen.
"""

import os
from pathlib import Path

# Workspace-Root (relativ zu dieser Datei)
ROOT = Path(__file__).resolve().parent.parent

# === PFADE ===
DATA_DIR = Path(os.environ.get("ORION_DATA_DIR", str(ROOT / "data")))
CAUSAL_HOME = Path(os.environ.get("ORION_CAUSAL_HOME", str(ROOT / "CausalHome")))
ORION_DB = DATA_DIR / "orion.db"
STATE_FILE = Path(os.environ.get("ORION_STATE_FILE", str(ROOT / "ORION_TOTAL_AUTONOMY_STATE.json")))
AUDIT_FILE = Path(os.environ.get("ORION_AUDIT_FILE", str(ROOT / "audit_qkernel.txt")))
QUANTUM_OUTPUT = Path(os.environ.get("ORION_QUANTUM_OUTPUT", str(ROOT / "quantum_result.png")))
PDF_DEFAULT = Path(os.environ.get("ORION_PDF_PATH", str(ROOT / "ct.25.09.140-145.pdf")))
APP_DIR = ROOT / "app"
SRC_DIR = ROOT / "src"

# === HOSTS ===
API_HOST = os.environ.get("ORION_API_HOST", "0.0.0.0")
API_PORT = int(os.environ.get("ORION_API_PORT", "8765"))
OLLAMA_HOST = os.environ.get("ORION_OLLAMA_HOST", "localhost")
OLLAMA_PORT = int(os.environ.get("ORION_OLLAMA_PORT", "11434"))
OLLAMA_BASE_URL = f"http://{OLLAMA_HOST}:{OLLAMA_PORT}"
MILVUS_HOST = os.environ.get("ORION_MILVUS_HOST", "localhost")
MILVUS_PORT = int(os.environ.get("ORION_MILVUS_PORT", "19530"))

# === AKTIVIERUNG ===
def ensure_paths():
    """Alle Pfade anlegen."""
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    CAUSAL_HOME.mkdir(parents=True, exist_ok=True)
    return True


def get_connection_args():
    """Milvus-Verbindung."""
    return {"host": MILVUS_HOST, "port": MILVUS_PORT}


def get_ollama_base_url():
    return OLLAMA_BASE_URL
