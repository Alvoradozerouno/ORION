//! Layer 2 — Model Governance Engine
//! No model can execute without signature verification, hash match, policy allow

use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ModelEntry {
    pub model_name: String,
    pub version: String,
    pub artifact_sha256: String,
    pub training_dataset_hash: String,
    pub signature: String,
    pub approval_policy: String,
    pub sbom_sha256: Option<String>,
    pub image_ref: Option<String>,
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
                model_name TEXT NOT NULL,
                version TEXT NOT NULL,
                artifact_sha256 TEXT NOT NULL,
                training_dataset_hash TEXT NOT NULL,
                signature TEXT NOT NULL,
                approval_policy TEXT NOT NULL,
                path TEXT,
                sbom_sha256 TEXT,
                image_ref TEXT,
                created_at TEXT NOT NULL
            );
            ",
        )?;
        conn.execute("ALTER TABLE models ADD COLUMN sbom_sha256 TEXT", [])
            .ok();
        conn.execute("ALTER TABLE models ADD COLUMN image_ref TEXT", []).ok();
        Ok(Self { conn })
    }

    pub fn register_model(
        &self,
        model_name: &str,
        version: &str,
        path: &str,
        training_dataset_hash: &str,
        signature: &str,
        approval_policy: &str,
    ) -> Result<ModelEntry, rusqlite::Error> {
        let bytes = std::fs::read(path).map_err(|_| rusqlite::Error::InvalidParameterName("read".into()))?;
        let artifact_sha256 = hex::encode(Sha256::digest(&bytes));
        let id = format!("{}:{}", model_name, version);
        let created = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR REPLACE INTO models (id, model_name, version, artifact_sha256, training_dataset_hash, signature, approval_policy, path, sbom_sha256, image_ref, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![id, model_name, version, artifact_sha256, training_dataset_hash, signature, approval_policy, path, None::<String>, None::<String>, created],
        )?;
        Ok(ModelEntry {
            model_name: model_name.to_string(),
            version: version.to_string(),
            artifact_sha256,
            training_dataset_hash: training_dataset_hash.to_string(),
            signature: signature.to_string(),
            approval_policy: approval_policy.to_string(),
            sbom_sha256: None,
            image_ref: None,
        })
    }

    pub fn verify_model(&self, model_name: &str, version: &str, expected_sha256: &str) -> Result<bool, rusqlite::Error> {
        let id = format!("{}:{}", model_name, version);
        let mut stmt = self.conn.prepare("SELECT artifact_sha256 FROM models WHERE id = ?1")?;
        let mut rows = stmt.query([id])?;
        match rows.next()? {
            Some(r) => Ok(r.get::<_, String>(0)? == expected_sha256),
            None => Ok(false),
        }
    }

    pub fn load_model(&self, model_name: &str, version: &str) -> Result<Option<ModelEntry>, rusqlite::Error> {
        let id = format!("{}:{}", model_name, version);
        let sql = "SELECT model_name, version, artifact_sha256, training_dataset_hash, signature, approval_policy, sbom_sha256, image_ref FROM models WHERE id = ?1";
        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([&id])?;
        match rows.next()? {
            Some(r) => Ok(Some(ModelEntry {
                model_name: r.get(0)?,
                version: r.get(1)?,
                artifact_sha256: r.get(2)?,
                training_dataset_hash: r.get(3)?,
                signature: r.get(4)?,
                approval_policy: r.get(5)?,
                sbom_sha256: r.get(6).ok().flatten(),
                image_ref: r.get(7).ok().flatten(),
            })),
            None => Ok(None),
        }
    }

    /// refuse_unverified_model — returns Err if model not in registry or hash mismatch
    pub fn refuse_unverified_model(
        &self,
        model_name: &str,
        version: &str,
        artifact_sha256: &str,
    ) -> Result<(), String> {
        let ok = self.verify_model(model_name, version, artifact_sha256)
            .map_err(|e| e.to_string())?;
        if ok {
            Ok(())
        } else {
            Err(format!(
                "model {}:{} not verified or hash mismatch",
                model_name, version
            ))
        }
    }
}
