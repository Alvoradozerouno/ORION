//! KeyStore abstraction — multiple active keys, version tagging, CRL support

use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyVersionId(pub u32);

impl std::fmt::Display for KeyVersionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeyVersionId {
    pub const fn v1() -> Self {
        Self(1)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVersion {
    pub id: KeyVersionId,
    pub public_key_hex: String,
    pub created_at: String,
    pub expires_at: Option<String>,
}

impl KeyVersion {
    pub fn is_expired(&self, now: &str) -> bool {
        if let Some(ref exp) = self.expires_at {
            now >= exp
        } else {
            false
        }
    }
}

/// Revocation list — revoked key version IDs. verify() must reject these.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevocationList {
    revoked: HashSet<KeyVersionId>,
}

impl RevocationList {
    pub fn new() -> Self {
        Self {
            revoked: HashSet::new(),
        }
    }

    pub fn revoke(&mut self, id: KeyVersionId) {
        self.revoked.insert(id);
    }

    pub fn is_revoked(&self, id: KeyVersionId) -> bool {
        self.revoked.contains(&id)
    }

    pub fn revoke_many(&mut self, ids: impl IntoIterator<Item = KeyVersionId>) {
        for id in ids {
            self.revoked.insert(id);
        }
    }
}

#[derive(Error, Debug)]
pub enum KeyStoreError {
    #[error("Key not found: {0}")]
    KeyNotFound(KeyVersionId),
    #[error("Key revoked: {0}")]
    KeyRevoked(KeyVersionId),
    #[error("Key expired: {0}")]
    KeyExpired(KeyVersionId),
    #[error("Invalid key material")]
    InvalidKeyMaterial,
    #[error("{0}")]
    Other(String),
}

/// KeyStore — multiple active signing keys, version tagging, CRL check
pub trait KeyStore: Send + Sync {
    /// Current active key for signing
    fn current_key_id(&self) -> KeyVersionId;

    /// Sign payload with current key. Returns (signature_hex, key_version_id)
    fn sign(&self, payload: &[u8]) -> Result<(String, KeyVersionId), KeyStoreError>;

    /// Verify signature. Returns Err if key revoked/expired/not found
    fn verify(
        &self,
        payload: &[u8],
        signature_hex: &str,
        key_version: KeyVersionId,
    ) -> Result<bool, KeyStoreError>;

    /// All active (non-revoked, non-expired) key versions
    fn active_keys(&self) -> Vec<KeyVersion>;
}

/// Local KeyStore — Ed25519 keypairs, in-memory or from env
pub struct LocalKeyStore {
    keys: Vec<(KeyVersionId, Keypair, KeyVersion)>,
    current: KeyVersionId,
    crl: RevocationList,
}

impl LocalKeyStore {
    pub fn from_seed(seed: &[u8; 32], version: KeyVersionId) -> Result<Self, KeyStoreError> {
        let secret = ed25519_dalek::SecretKey::from_bytes(seed)
            .map_err(|_| KeyStoreError::InvalidKeyMaterial)?;
        let public = PublicKey::from(&secret);
        let keypair = Keypair { secret, public };
        let created = chrono::Utc::now().to_rfc3339();
        let kv = KeyVersion {
            id: version,
            public_key_hex: hex::encode(public.as_bytes()),
            created_at: created.clone(),
            expires_at: None,
        };
        Ok(Self {
            keys: vec![(version, keypair, kv)],
            current: version,
            crl: RevocationList::new(),
        })
    }

    pub fn from_env() -> Option<Self> {
        let hex_key = std::env::var("GSF_SIGNING_KEY").ok()?;
        let bytes: Vec<u8> = hex::decode(hex_key).ok()?;
        let arr: [u8; 32] = bytes.try_into().ok()?;
        Self::from_seed(&arr, KeyVersionId::v1()).ok()
    }

    /// Add new key version for rotation
    pub fn add_key(&mut self, seed: &[u8; 32], version: KeyVersionId) -> Result<(), KeyStoreError> {
        let secret = ed25519_dalek::SecretKey::from_bytes(seed)
            .map_err(|_| KeyStoreError::InvalidKeyMaterial)?;
        let public = PublicKey::from(&secret);
        let keypair = Keypair { secret, public };
        let created = chrono::Utc::now().to_rfc3339();
        let kv = KeyVersion {
            id: version,
            public_key_hex: hex::encode(public.as_bytes()),
            created_at: created,
            expires_at: None,
        };
        self.keys.push((version, keypair, kv));
        self.current = version;
        Ok(())
    }

    /// Rotate to new key — call add_key then set_current
    pub fn set_current(&mut self, version: KeyVersionId) -> Result<(), KeyStoreError> {
        if self.keys.iter().any(|(id, _, _)| *id == version) {
            self.current = version;
            Ok(())
        } else {
            Err(KeyStoreError::KeyNotFound(version))
        }
    }

    pub fn with_crl(mut self, crl: RevocationList) -> Self {
        self.crl = crl;
        self
    }

    pub fn revoke(&mut self, id: KeyVersionId) {
        self.crl.revoke(id);
    }
}

impl KeyStore for LocalKeyStore {
    fn current_key_id(&self) -> KeyVersionId {
        self.current
    }

    fn sign(&self, payload: &[u8]) -> Result<(String, KeyVersionId), KeyStoreError> {
        let now = chrono::Utc::now().to_rfc3339();
        for (id, kp, kv) in &self.keys {
            if *id == self.current {
                if self.crl.is_revoked(*id) {
                    return Err(KeyStoreError::KeyRevoked(*id));
                }
                if kv.is_expired(&now) {
                    return Err(KeyStoreError::KeyExpired(*id));
                }
                let sig = kp.sign(payload);
                return Ok((hex::encode(sig.to_bytes()), *id));
            }
        }
        Err(KeyStoreError::KeyNotFound(self.current))
    }

    fn verify(
        &self,
        payload: &[u8],
        signature_hex: &str,
        key_version: KeyVersionId,
    ) -> Result<bool, KeyStoreError> {
        if self.crl.is_revoked(key_version) {
            return Err(KeyStoreError::KeyRevoked(key_version));
        }
        let (_, _, kv) = self
            .keys
            .iter()
            .find(|(id, _, _)| *id == key_version)
            .ok_or(KeyStoreError::KeyNotFound(key_version))?;
        let now = chrono::Utc::now().to_rfc3339();
        if kv.is_expired(&now) {
            return Err(KeyStoreError::KeyExpired(key_version));
        }
        let pk_bytes = hex::decode(&kv.public_key_hex).map_err(|_| KeyStoreError::InvalidKeyMaterial)?;
        let pk = PublicKey::from_bytes(pk_bytes.as_slice().try_into().map_err(|_| KeyStoreError::InvalidKeyMaterial)?)
            .map_err(|_| KeyStoreError::InvalidKeyMaterial)?;
        let sig_bytes = hex::decode(signature_hex).map_err(|_| KeyStoreError::InvalidKeyMaterial)?;
        let sig_arr: [u8; 64] = sig_bytes.as_slice().try_into().map_err(|_| KeyStoreError::InvalidKeyMaterial)?;
        let sig = Signature::from_bytes(&sig_arr).map_err(|_| KeyStoreError::InvalidKeyMaterial)?;
        Ok(pk.verify(payload, &sig).is_ok())
    }

    fn active_keys(&self) -> Vec<KeyVersion> {
        let now = chrono::Utc::now().to_rfc3339();
        self.keys
            .iter()
            .filter(|(id, _, kv)| !self.crl.is_revoked(*id) && !kv.is_expired(&now))
            .map(|(_, _, kv)| kv.clone())
            .collect()
    }
}
