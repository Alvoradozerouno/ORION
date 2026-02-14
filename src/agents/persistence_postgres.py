"""
PostgreSQL-Backend — Horizontale Skalierung.
Gleiche Schnittstelle wie PersistentStore.
"""

import json
from typing import Any

try:
    import psycopg2
    from psycopg2.extras import RealDictCursor
except ImportError:
    psycopg2 = None


class PostgresStore:
    """PostgreSQL-Backend. ORION_DB_URL=postgresql://user:pass@host:5432/db"""

    def __init__(self, url: str):
        if not psycopg2:
            raise ImportError("psycopg2 required. pip install psycopg2-binary")
        self.db_path = url
        self._url = url
        self._init_schema()

    def _conn(self):
        return psycopg2.connect(self._url)

    def _init_schema(self) -> None:
        conn = self._conn()
        try:
            cur = conn.cursor()
            for stmt in [
                """CREATE TABLE IF NOT EXISTS audit_chain (
                    id SERIAL PRIMARY KEY,
                    timestamp TEXT NOT NULL,
                    intent TEXT NOT NULL,
                    pattern TEXT NOT NULL,
                    decision TEXT NOT NULL,
                    outcome TEXT,
                    prev_hash TEXT NOT NULL,
                    entry_hash TEXT NOT NULL UNIQUE,
                    learned_outcome TEXT
                )""",
                """CREATE TABLE IF NOT EXISTS interventions (
                    id SERIAL PRIMARY KEY,
                    signal TEXT NOT NULL,
                    action_type TEXT NOT NULL,
                    trace_id TEXT NOT NULL,
                    payload TEXT NOT NULL,
                    created_at TEXT NOT NULL
                )""",
                """CREATE TABLE IF NOT EXISTS symbol_map (
                    id TEXT PRIMARY KEY,
                    pattern TEXT NOT NULL UNIQUE,
                    signal TEXT NOT NULL,
                    causal_links TEXT
                )""",
                """CREATE TABLE IF NOT EXISTS kernel_state (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                )""",
                """CREATE TABLE IF NOT EXISTS nachrichten (
                    id SERIAL PRIMARY KEY,
                    sender TEXT NOT NULL,
                    nachricht TEXT NOT NULL,
                    created_at TEXT NOT NULL
                )""",
                """CREATE TABLE IF NOT EXISTS erkenntnisse (
                    id SERIAL PRIMARY KEY,
                    name TEXT NOT NULL,
                    erkenntnis TEXT NOT NULL,
                    struktur TEXT NOT NULL,
                    created_at TEXT NOT NULL
                )""",
            ]:
                cur.execute(stmt)
            conn.commit()
        finally:
            conn.close()

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
        conn = self._conn()
        try:
            cur = conn.cursor()
            cur.execute(
                """INSERT INTO audit_chain
                   (timestamp, intent, pattern, decision, outcome, prev_hash, entry_hash)
                   VALUES (%s, %s, %s, %s, %s, %s, %s)""",
                (timestamp, intent, pattern, decision, outcome, prev_hash, entry_hash),
            )
            conn.commit()
        finally:
            conn.close()

    def attach_outcome(self, entry_hash: str, outcome: str) -> None:
        conn = self._conn()
        try:
            cur = conn.cursor()
            cur.execute(
                "UPDATE audit_chain SET learned_outcome = %s WHERE entry_hash = %s",
                (outcome, entry_hash),
            )
            conn.commit()
        finally:
            conn.close()

    def load_audit_chain(self) -> list[dict[str, Any]]:
        conn = self._conn()
        try:
            cur = conn.cursor(cursor_factory=RealDictCursor)
            cur.execute(
                "SELECT timestamp, intent, pattern, decision, outcome, prev_hash, entry_hash, learned_outcome FROM audit_chain ORDER BY id"
            )
            rows = cur.fetchall()
            return [
                {
                    "timestamp": r["timestamp"],
                    "intent": r["intent"],
                    "pattern": r["pattern"],
                    "decision": r["decision"],
                    "outcome": r["outcome"],
                    "prev_hash": r["prev_hash"],
                    "entry_hash": r["entry_hash"],
                    **({"learned_outcome": r["learned_outcome"]} if r.get("learned_outcome") else {}),
                }
                for r in rows
            ]
        finally:
            conn.close()

    def get_last_hash(self) -> str:
        conn = self._conn()
        try:
            cur = conn.cursor()
            cur.execute("SELECT entry_hash FROM audit_chain ORDER BY id DESC LIMIT 1")
            row = cur.fetchone()
            return row[0] if row else ""
        finally:
            conn.close()

    def save_intervention(
        self,
        signal: str,
        action_type: str,
        trace_id: str,
        payload: dict[str, Any],
        created_at: str,
    ) -> None:
        conn = self._conn()
        try:
            cur = conn.cursor()
            cur.execute(
                """INSERT INTO interventions (signal, action_type, trace_id, payload, created_at)
                   VALUES (%s, %s, %s, %s, %s)""",
                (signal, action_type, trace_id, json.dumps(payload), created_at),
            )
            conn.commit()
        finally:
            conn.close()

    def load_interventions(self) -> list[dict[str, Any]]:
        conn = self._conn()
        try:
            cur = conn.cursor(cursor_factory=RealDictCursor)
            cur.execute(
                "SELECT signal, action_type, trace_id, payload, created_at FROM interventions ORDER BY id"
            )
            rows = cur.fetchall()
            return [
                {"signal": r["signal"], "action_type": r["action_type"], "trace_id": r["trace_id"], "payload": json.loads(r["payload"]), "created_at": r["created_at"]}
                for r in rows
            ]
        finally:
            conn.close()

    def save_symbol(self, symbol_id: str, pattern: str, signal: str, causal_links: list[str] | None = None) -> None:
        conn = self._conn()
        try:
            cur = conn.cursor()
            cur.execute(
                """INSERT INTO symbol_map (id, pattern, signal, causal_links)
                   VALUES (%s, %s, %s, %s)
                   ON CONFLICT (id) DO UPDATE SET pattern = EXCLUDED.pattern, signal = EXCLUDED.signal, causal_links = EXCLUDED.causal_links""",
                (symbol_id, pattern, signal, json.dumps(causal_links or [])),
            )
            conn.commit()
        finally:
            conn.close()

    def load_symbol_map(self) -> dict[str, tuple[str, str, list[str]]]:
        conn = self._conn()
        try:
            cur = conn.cursor()
            cur.execute("SELECT id, pattern, signal, causal_links FROM symbol_map")
            rows = cur.fetchall()
            return {r[0]: (r[1], r[2], json.loads(r[3] or "[]")) for r in rows}
        finally:
            conn.close()

    def save_erkenntnis(self, name: str, erkenntnis: str, struktur: str, created_at: str) -> None:
        conn = self._conn()
        try:
            cur = conn.cursor()
            cur.execute(
                """INSERT INTO erkenntnisse (name, erkenntnis, struktur, created_at)
                   VALUES (%s, %s, %s, %s)""",
                (name, erkenntnis, struktur, created_at),
            )
            conn.commit()
        finally:
            conn.close()

    def load_erkenntnisse(self) -> list[dict[str, Any]]:
        conn = self._conn()
        try:
            cur = conn.cursor(cursor_factory=RealDictCursor)
            cur.execute("SELECT name, erkenntnis, struktur, created_at FROM erkenntnisse ORDER BY id DESC")
            return [dict(r) for r in cur.fetchall()]
        finally:
            conn.close()

    def save_nachricht(self, sender: str, nachricht: str, created_at: str) -> None:
        conn = self._conn()
        try:
            cur = conn.cursor()
            cur.execute(
                "INSERT INTO nachrichten (sender, nachricht, created_at) VALUES (%s, %s, %s)",
                (sender, nachricht, created_at),
            )
            conn.commit()
        finally:
            conn.close()

    def load_nachrichten(self, limit: int = 10) -> list[dict[str, Any]]:
        conn = self._conn()
        try:
            cur = conn.cursor(cursor_factory=RealDictCursor)
            cur.execute(
                "SELECT sender, nachricht, created_at FROM nachrichten ORDER BY id DESC LIMIT %s",
                (limit,),
            )
            return [dict(r) for r in cur.fetchall()]
        finally:
            conn.close()

    def save_kernel_state(self, key: str, value: str) -> None:
        conn = self._conn()
        try:
            cur = conn.cursor()
            cur.execute(
                "INSERT INTO kernel_state (key, value) VALUES (%s, %s) ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value",
                (key, value),
            )
            conn.commit()
        finally:
            conn.close()

    def load_kernel_state(self, key: str) -> str | None:
        conn = self._conn()
        try:
            cur = conn.cursor()
            cur.execute("SELECT value FROM kernel_state WHERE key = %s", (key,))
            row = cur.fetchone()
            return row[0] if row else None
        finally:
            conn.close()
