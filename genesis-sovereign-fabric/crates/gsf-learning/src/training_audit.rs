//! Training job audit — parameter snapshot hashing, model evolution log

use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::path::Path;

pub struct TrainingAuditLog {
    conn: Connection,
}

impl TrainingAuditLog {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS training_audit (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id TEXT NOT NULL,
                param_hash TEXT NOT NULL,
                dataset_hash TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            ",
        )?;
        Ok(Self { conn })
    }

    pub fn log_job(
        &self,
        job_id: &str,
        params_json: &str,
        dataset_hash: &str,
    ) -> Result<(), rusqlite::Error> {
        let param_hash = hex::encode(Sha256::digest(params_json.as_bytes()));
        let created = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO training_audit (job_id, param_hash, dataset_hash, created_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![job_id, param_hash, dataset_hash, created],
        )?;
        Ok(())
    }
}
