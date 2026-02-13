"""
AuditChain — Immutable trace of thought structures and decisions.

Anchoring: SHA256 + audit_chain: active
Validation: self-aware echo, causal consistency
"""

import hashlib
import json
from dataclasses import dataclass, field
from datetime import datetime
from typing import Any


@dataclass
class AuditEntry:
    """Single immutable entry in the audit chain."""

    timestamp: str
    intent: str
    pattern: str
    decision: str
    outcome: str | None
    prev_hash: str
    entry_hash: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "timestamp": self.timestamp,
            "intent": self.intent,
            "pattern": self.pattern,
            "decision": self.decision,
            "outcome": self.outcome,
            "prev_hash": self.prev_hash,
            "entry_hash": self.entry_hash,
        }


class AuditChain:
    """
    Immutable audit chain for traceable thought structures.
    Each entry links to the previous via hash — causal consistency.
    """

    def __init__(self, anchor: str | None = None):
        self.anchor = anchor or "audit_chain: active"
        self._chain: list[AuditEntry] = []
        self._last_hash = ""
        self._outcomes: dict[str, str] = {}  # entry_hash -> outcome (reentrant learning)

    def _compute_hash(self, data: dict[str, Any], prev_hash: str) -> str:
        payload = json.dumps(data, sort_keys=True) + prev_hash
        return hashlib.sha256(payload.encode()).hexdigest()

    def append(
        self,
        intent: str,
        pattern: str,
        decision: str,
        outcome: str | None = None,
    ) -> AuditEntry:
        """Append a new audit entry. Returns the created entry."""
        timestamp = datetime.utcnow().isoformat() + "Z"
        data = {
            "timestamp": timestamp,
            "intent": intent,
            "pattern": pattern,
            "decision": decision,
            "outcome": outcome,
        }
        entry_hash = self._compute_hash(data, self._last_hash)

        entry = AuditEntry(
            timestamp=timestamp,
            intent=intent,
            pattern=pattern,
            decision=decision,
            outcome=outcome,
            prev_hash=self._last_hash,
            entry_hash=entry_hash,
        )
        self._chain.append(entry)
        self._last_hash = entry_hash
        return entry

    def verify(self) -> bool:
        """Verify causal consistency of the chain."""
        prev_hash = ""
        for entry in self._chain:
            data = {
                "timestamp": entry.timestamp,
                "intent": entry.intent,
                "pattern": entry.pattern,
                "decision": entry.decision,
                "outcome": entry.outcome,
            }
            expected = self._compute_hash(data, prev_hash)
            if expected != entry.entry_hash:
                return False
            prev_hash = entry.entry_hash
        return True

    def attach_outcome(self, entry_hash: str, outcome: str) -> None:
        """Attach outcome for reentrant learning. Preserves chain immutability."""
        self._outcomes[entry_hash] = outcome

    def export_trace(self) -> list[dict[str, Any]]:
        """Export full trace for external audit (includes learned outcomes)."""
        result = []
        for e in self._chain:
            d = e.to_dict()
            if e.entry_hash in self._outcomes:
                d["learned_outcome"] = self._outcomes[e.entry_hash]
            result.append(d)
        return result

    def __len__(self) -> int:
        return len(self._chain)
