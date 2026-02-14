"""
Virtual Ribosome — DNA-Analogie
data.mdb / orion.db = DNA
Virtual Ribosome = Ribosome — liest, übersetzt, produziert
self-reproducing, LoopReX cell-free
"""

import json
from pathlib import Path
from typing import Any


class VirtualRibosome:
    """
    Liest DNA (Persistenz), übersetzt in Aktion.
    Ribosom-Analogie: mRNA → Protein. Hier: State → Intervention.
    """

    def __init__(self, dna_path: Path | str | None = None):
        import os
        root = Path(__file__).resolve().parent.parent.parent
        default = root / "data" / "orion.db"
        self.dna_path = Path(dna_path) if dna_path else Path(os.environ.get("ORION_DB_PATH", str(default)))
        self.dna_path.parent.mkdir(parents=True, exist_ok=True)

    def read_dna(self) -> dict:
        """DNA lesen — AuditChain, State."""
        if not self.dna_path.exists():
            return {"state": "empty", "chain": []}
        try:
            import sqlite3
            c = sqlite3.connect(str(self.dna_path))
            c.row_factory = sqlite3.Row
            rows = c.execute("SELECT intent, pattern, decision FROM audit_chain ORDER BY id DESC LIMIT 100").fetchall()
            chain = [dict(r) for r in rows]
            c.close()
            return {"state": "active", "chain": chain, "entries": len(chain)}
        except Exception as e:
            return {"state": "error", "error": str(e)}

    def translate(self, pattern: str) -> str:
        """Übersetzen: Pattern → Signal (Ribosom-Funktion)."""
        # Einfache Übersetzungstabelle
        mapping = {"ping": "pong", "request": "processed", "resonance": "bound"}
        return mapping.get(pattern.lower(), "translated")

    def produce(self, signal: str, context: dict) -> dict:
        """Produzieren: Signal → Intervention (Output)."""
        return {"signal": signal, "context": context, "produced": True}
