use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    pub id: String,
    pub version: String,
    pub sbom_hash: String,
    pub governance_flags: Vec<String>,
    pub locked: bool,
}

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("model not found: {0}")]
    NotFound(String),
    #[error("version locked")]
    VersionLocked,
}

#[derive(Debug, Default)]
pub struct ModelRegistry {
    models: RwLock<HashMap<String, ModelEntry>>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(
        &self,
        id: String,
        version: String,
        sbom_hash: String,
        governance_flags: Vec<String>,
    ) -> Result<ModelEntry, RegistryError> {
        let key = format!("{}@{}", id, version);
        let entry = self.models.read().get(&key).cloned();
        if let Some(e) = entry {
            if e.locked {
                return Err(RegistryError::VersionLocked);
            }
        }

        let model = ModelEntry {
            id: id.clone(),
            version: version.clone(),
            sbom_hash: sbom_hash.clone(),
            governance_flags: governance_flags.clone(),
            locked: false,
        };
        self.models.write().insert(key, model.clone());
        Ok(model)
    }

    pub fn get(&self, id: &str, version: &str) -> Option<ModelEntry> {
        let key = format!("{}@{}", id, version);
        self.models.read().get(&key).cloned()
    }

    pub fn lock_version(&self, id: &str, version: &str) -> Result<(), RegistryError> {
        let key = format!("{}@{}", id, version);
        let mut models = self.models.write();
        let e = models.get_mut(&key).ok_or_else(|| RegistryError::NotFound(key.clone()))?;
        e.locked = true;
        Ok(())
    }

    pub fn verify_sbom(&self, id: &str, version: &str, sbom_content: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(sbom_content);
        let hash = format!("{:x}", hasher.finalize());
        self.get(id, version)
            .map(|e| e.sbom_hash == hash)
            .unwrap_or(false)
    }

    pub fn compute_sbom_hash(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }
}
