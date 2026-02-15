//! Layer 3 — Model Governance
//! Model registry, version hashing, SBOM, supply chain verification

pub mod model_registry;
pub mod sbom;

pub use model_registry::ModelRegistry;
pub use sbom::SbomGenerator;
