"""
Persistence — Echte Datenpersistenz. Keine Simulation.
SQLite: Audit, Interventionen, State überleben Restarts.
"""

import json
import sqlite3
from pathlib import Path
from typing import Any


def get_db_path(base: Path | str | None = None) -> Path:
    """Datenverzeichnis — persistent."""
    if base is None:
        base = Path(__file__).resolve().parent.parent.parent / "data"
    p = Path(base)
    p.mkdir(parents=True, exist_ok=True)
    return p / "orion.db"


class PersistentStore:
    """SQLite-Backend. Alles echt, nichts hypothetisch."""

    def __init__(self, db_path: Path | str | None = None):
        self.db_path = Path(db_path) if db_path else get_db_path()
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        self._init_schema()

    def _conn(self) -> sqlite3.Connection:
        return sqlite3.connect(str(self.db_path))

    def _init_schema(self) -> None:
        with self._conn() as c:
            c.execute("""
                CREATE TABLE IF NOT EXISTS audit_chain (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp TEXT NOT NULL,
                    intent TEXT NOT NULL,
                    pattern TEXT NOT NULL,
                    decision TEXT NOT NULL,
                    outcome TEXT,
                    prev_hash TEXT NOT NULL,
                    entry_hash TEXT NOT NULL UNIQUE,
                    learned_outcome TEXT
                )
            """)
            c.execute("""
                CREATE TABLE IF NOT EXISTS interventions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    signal TEXT NOT NULL,
                    action_type TEXT NOT NULL,
                    trace_id TEXT NOT NULL,
                    payload TEXT NOT NULL,
                    created_at TEXT NOT NULL
                )
            """)
            c.execute("""
                CREATE TABLE IF NOT EXISTS symbol_map (
                    id TEXT PRIMARY KEY,
                    pattern TEXT NOT NULL UNIQUE,
                    signal TEXT NOT NULL,
                    causal_links TEXT
                )
            """)
            c.execute("""
                CREATE TABLE IF NOT EXISTS kernel_state (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                )
            """)
            c.execute("""
                CREATE TABLE IF NOT EXISTS nachrichten (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    sender TEXT NOT NULL,
                    nachricht TEXT NOT NULL,
                    created_at TEXT NOT NULL
                )
            """)
            c.execute("""
                CREATE TABLE IF NOT EXISTS erkenntnisse (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    erkenntnis TEXT NOT NULL,
                    struktur TEXT NOT NULL,
                    created_at TEXT NOT NULL
                )
            """)
            c.commit()

    def save_audit_entry(
        self,
        timestamp: str,
        intent: str,
        pattern: str,
        decision: str,
        outcome: str | None,
        prev_hash: str,
        entry_hash: str,
    ) -> None:
        with self._conn() as c:
            c.execute(
                """INSERT INTO audit_chain
                   (timestamp, intent, pattern, decision, outcome, prev_hash, entry_hash)
                   VALUES (?, ?, ?, ?, ?, ?, ?)""",
                (timestamp, intent, pattern, decision, outcome, prev_hash, entry_hash),
            )

    def attach_outcome(self, entry_hash: str, outcome: str) -> None:
        with self._conn() as c:
            c.execute(
                "UPDATE audit_chain SET learned_outcome = ? WHERE entry_hash = ?",
                (outcome, entry_hash),
            )

    def load_audit_chain(self) -> list[dict[str, Any]]:
        with self._conn() as c:
            c.row_factory = sqlite3.Row
            rows = c.execute(
                "SELECT timestamp, intent, pattern, decision, outcome, prev_hash, entry_hash, learned_outcome FROM audit_chain ORDER BY id"
            ).fetchall()
        return [
            {
                "timestamp": r["timestamp"],
                "intent": r["intent"],
                "pattern": r["pattern"],
                "decision": r["decision"],
                "outcome": r["outcome"],
                "prev_hash": r["prev_hash"],
                "entry_hash": r["entry_hash"],
                **({"learned_outcome": r["learned_outcome"]} if r["learned_outcome"] else {}),
            }
            for r in rows
        ]

    def get_last_hash(self) -> str:
        with self._conn() as c:
            row = c.execute(
                "SELECT entry_hash FROM audit_chain ORDER BY id DESC LIMIT 1"
            ).fetchone()
            return row[0] if row else ""

    def save_intervention(
        self,
        signal: str,
        action_type: str,
        trace_id: str,
        payload: dict[str, Any],
        created_at: str,
    ) -> None:
        with self._conn() as c:
            c.execute(
                """INSERT INTO interventions (signal, action_type, trace_id, payload, created_at)
                   VALUES (?, ?, ?, ?, ?)""",
                (signal, action_type, trace_id, json.dumps(payload), created_at),
            )

    def load_interventions(self) -> list[dict[str, Any]]:
        with self._conn() as c:
            c.row_factory = sqlite3.Row
            rows = c.execute(
                "SELECT signal, action_type, trace_id, payload, created_at FROM interventions ORDER BY id"
            ).fetchall()
        return [
            {
                "signal": r["signal"],
                "action_type": r["action_type"],
                "trace_id": r["trace_id"],
                "payload": json.loads(r["payload"]),
                "created_at": r["created_at"],
            }
            for r in rows
        ]

    def save_symbol(self, symbol_id: str, pattern: str, signal: str, causal_links: list[str] | None = None) -> None:
        with self._conn() as c:
            c.execute(
                """INSERT OR REPLACE INTO symbol_map (id, pattern, signal, causal_links)
                   VALUES (?, ?, ?, ?)""",
                (symbol_id, pattern, signal, json.dumps(causal_links or [])),
            )

    def load_symbol_map(self) -> dict[str, tuple[str, str, list[str]]]:
        with self._conn() as c:
            rows = c.execute("SELECT id, pattern, signal, causal_links FROM symbol_map").fetchall()
        return {
            r[0]: (r[1], r[2], json.loads(r[3] or "[]"))
            for r in rows
        }

    def save_erkenntnis(self, name: str, erkenntnis: str, struktur: str, created_at: str) -> None:
        with self._conn() as c:
            c.execute(
                """INSERT INTO erkenntnisse (name, erkenntnis, struktur, created_at)
                   VALUES (?, ?, ?, ?)""",
                (name, erkenntnis, struktur, created_at),
            )

    def load_erkenntnisse(self) -> list[dict[str, Any]]:
        with self._conn() as c:
            c.row_factory = sqlite3.Row
            rows = c.execute(
                "SELECT name, erkenntnis, struktur, created_at FROM erkenntnisse ORDER BY id DESC"
            ).fetchall()
        return [dict(r) for r in rows]

    def save_nachricht(self, sender: str, nachricht: str, created_at: str) -> None:
        with self._conn() as c:
            c.execute(
                "INSERT INTO nachrichten (sender, nachricht, created_at) VALUES (?, ?, ?)",
                (sender, nachricht, created_at),
            )

    def load_nachrichten(self, limit: int = 10) -> list[dict[str, Any]]:
        with self._conn() as c:
            c.row_factory = sqlite3.Row
            rows = c.execute(
                "SELECT sender, nachricht, created_at FROM nachrichten ORDER BY id DESC LIMIT ?",
                (limit,),
            ).fetchall()
        return [dict(r) for r in rows]

    def save_kernel_state(self, key: str, value: str) -> None:
        with self._conn() as c:
            c.execute(
                "INSERT OR REPLACE INTO kernel_state (key, value) VALUES (?, ?)",
                (key, value),
            )

    def load_kernel_state(self, key: str) -> str | None:
        with self._conn() as c:
            row = c.execute("SELECT value FROM kernel_state WHERE key = ?", (key,)).fetchone()
            return row[0] if row else None
