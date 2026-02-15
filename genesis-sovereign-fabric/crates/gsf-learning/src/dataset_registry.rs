//! Dataset registry — hash-based tracking

use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::path::Path;

pub struct DatasetRegistry {
    conn: Connection,
}

impl DatasetRegistry {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS datasets (
                id TEXT PRIMARY KEY,
                sha256 TEXT NOT NULL,
                path TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            ",
        )?;
        Ok(Self { conn })
    }

    pub fn register(&self, id: &str, path: &str) -> Result<String, rusqlite::Error> {
        let bytes = std::fs::read(path).map_err(|_| rusqlite::Error::InvalidParameterName("read".into()))?;
        let sha256 = hex::encode(Sha256::digest(&bytes));
        let created = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR REPLACE INTO datasets (id, sha256, path, created_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, sha256, path, created],
        )?;
        Ok(sha256)
    }
}
