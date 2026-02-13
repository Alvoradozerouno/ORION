"""
Audit Chain - Nachverfolgbare Denkstrukturen
Ermöglicht exportable_trace für kausale Nachvollziehbarkeit.
"""

import hashlib
from dataclasses import dataclass, field
from datetime import datetime
from typing import Any, Optional


@dataclass
class AuditEntry:
    """Einzelner Eintrag in der Audit Chain."""
    timestamp: str
    event_type: str
    payload: dict[str, Any]
    previous_hash: Optional[str] = None
    entry_hash: Optional[str] = None

    def compute_hash(self) -> str:
        """Berechnet den SHA256-Hash des Eintrags."""
        content = f"{self.timestamp}|{self.event_type}|{self.previous_hash or ''}|{str(self.payload)}"
        return hashlib.sha256(content.encode()).hexdigest()


class AuditChain:
    """
    Aktive Audit Chain für audit-traceable thought structures.
    Verkettete Struktur mit kausaler Integrität.
    """

    def __init__(self):
        self.entries: list[AuditEntry] = []
        self._last_hash: Optional[str] = None

    def append(self, event_type: str, payload: dict[str, Any]) -> AuditEntry:
        """Fügt einen neuen Eintrag zur Chain hinzu."""
        timestamp = datetime.utcnow().isoformat() + "Z"
        entry = AuditEntry(
            timestamp=timestamp,
            event_type=event_type,
            payload=payload,
            previous_hash=self._last_hash,
        )
        entry.entry_hash = entry.compute_hash()
        self._last_hash = entry.entry_hash
        self.entries.append(entry)
        return entry

    def verify(self) -> bool:
        """Verifiziert die Integrität der gesamten Chain."""
        prev_hash = None
        for entry in self.entries:
            if entry.previous_hash != prev_hash:
                return False
            expected = entry.compute_hash()
            if entry.entry_hash != expected:
                return False
            prev_hash = entry.entry_hash
        return True
