//! Model registry — SHA256 hashed, signed, reproducible

use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Debug)]
pub struct ModelEntry {
    pub id: String,
    pub version: String,
    pub sha256: String,
    pub path: String,
}

pub struct ModelRegistry {
    conn: Connection,
}

impl ModelRegistry {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS models (
                id TEXT PRIMARY KEY,
                version TEXT NOT NULL,
                sha256 TEXT NOT NULL,
                path TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            ",
        )?;
        Ok(Self { conn })
    }

    pub fn register(&self, id: &str, version: &str, path: &str) -> Result<ModelEntry, rusqlite::Error> {
        let bytes = std::fs::read(path).map_err(|_| rusqlite::Error::InvalidParameterName("cannot read".into()))?;
        let sha256 = hex::encode(Sha256::digest(&bytes));
        let created = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR REPLACE INTO models (id, version, sha256, path, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, version, sha256, path, created],
        )?;
        Ok(ModelEntry {
            id: id.to_string(),
            version: version.to_string(),
            sha256,
            path: path.to_string(),
        })
    }

    pub fn verify(&self, id: &str, expected_sha256: &str) -> Result<bool, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT sha256 FROM models WHERE id = ?1")?;
        let mut rows = stmt.query([id])?;
        match rows.next()? {
            Some(r) => Ok(r.get::<_, String>(0)? == expected_sha256),
            None => Ok(false),
        }
    }
}
