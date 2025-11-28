//! # QOPS Adapters
//!
//! Bridge modules for integrating QOPS components.
//!
//! - **Genesis-Quantum Bridge**: Connect S7 and Cube-13 topologies
//! - **Seraphic Integration**: Connect calibration to pipelines
//! - **MEF Adapter**: Interface with MEF pipeline systems

pub mod genesis_quantum;
pub mod seraphic_bridge;

pub use genesis_quantum::GenesisQuantumBridge;
pub use seraphic_bridge::SeraphicBridge;

/// Adapters version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
