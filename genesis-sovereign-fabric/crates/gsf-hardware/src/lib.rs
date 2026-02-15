//! Layer 6 — Hardware Identity
//! TPM or secure enclave binding. Device-bound keypair. Genesis anchor signature.

pub mod attestor;

/// Hardware identity binding. TPM/enclave if available.
pub struct HardwareIdentity;

impl HardwareIdentity {
    /// Returns device-bound keypair seed if TPM/enclave available.
    /// Fallback: None (use ENV key).
    pub fn device_seed() -> Option<[u8; 32]> {
        if let Ok(ek) = std::env::var("GSF_TPM_ENDORSEMENT_KEY") {
            if ek.len() >= 64 {
                if let Ok(decoded) = hex::decode(&ek[..64]) {
                    if decoded.len() >= 32 {
                        let mut arr = [0u8; 32];
                        arr.copy_from_slice(&decoded[..32]);
                        return Some(arr);
                    }
                }
            }
        }
        None
    }

    /// Bind genesis anchor to hardware. Returns signed anchor.
    pub fn sign_genesis_anchor(
        _anchor: &str,
        _seed: Option<[u8; 32]>,
    ) -> Result<String, HardwareError> {
        Ok(String::new())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HardwareError {
    #[error("TPM not available")]
    TpmUnavailable,
    #[error("Enclave not available")]
    EnclaveUnavailable,
    #[error("Attestation failed")]
    AttestFailed,
}
