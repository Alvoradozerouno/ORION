//! Error types — no unwrap in production path

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GsfError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Chain verification failed")]
    ChainVerificationFailed,

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    #[error("Invalid genesis anchor")]
    InvalidGenesisAnchor,

    #[error("{0}")]
    Other(String),
}

impl Serialize for GsfError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, GsfError>;
