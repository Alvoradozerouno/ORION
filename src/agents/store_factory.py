"""
Store Factory — SQLite oder PostgreSQL.
ORION_DB_URL: leer oder sqlite → SQLite. postgresql://... → PostgreSQL.
"""

import os
from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from .persistence import PersistentStore


def get_store(data_dir: Path | str | None = None) -> "PersistentStore":
    url = os.environ.get("ORION_DB_URL", "").strip()
    if url.startswith("postgresql://") or url.startswith("postgres://"):
        try:
            from .persistence_postgres import PostgresStore
            return PostgresStore(url)
        except ImportError:
            raise RuntimeError(
                "ORION_DB_URL=postgresql:// gesetzt, aber psycopg2 nicht installiert. "
                "pip install psycopg2-binary"
            )
    from .persistence import PersistentStore, get_db_path
    base = data_dir or os.environ.get("ORION_DATA_DIR", "")
    base = Path(base) if base else None
    path = get_db_path(base) if base else get_db_path()
    return PersistentStore(path)
