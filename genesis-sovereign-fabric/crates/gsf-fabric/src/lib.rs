//! Layer 2 — Execution Fabric
//! Async scheduler, resource-aware runtime, model sandbox, temperature cap.

pub mod scheduler;
pub mod temperature_cap;

pub use scheduler::TaskScheduler;
pub use temperature_cap::TemperatureCap;
