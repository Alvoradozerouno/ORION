//! Layer 4 — Federated Mesh
//! mTLS-only, signed ledger sync, conflict detection, fork resolution

pub mod conflict;
pub mod protocol;

pub use conflict::{ConflictDetection, ForkResolution};
pub use protocol::MeshSyncProtocol;
