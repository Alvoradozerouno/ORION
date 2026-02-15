//! Layer 5 — Learning Pipeline (Controlled)
//! Dataset registry, training audit, parameter snapshot hashing

pub mod dataset_registry;
pub mod training_audit;

pub use dataset_registry::DatasetRegistry;
pub use training_audit::TrainingAuditLog;
