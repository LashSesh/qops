//! # QOPS Core
//!
//! Core types and traits for the Unified Quantum Operator Processing System.
//!
//! This crate provides the foundational abstractions shared between:
//! - **Genesis Pipeline** (S7 5040-node topology for operator mining)
//! - **Quantum Pipeline** (Cube-13 topology for quantum algorithms)
//! - **Seraphic Calibration Shell** (Meta-algorithm for fixpoint evolution)
//!
//! ## Core Concepts
//!
//! - [`Signature`] - Performance/quality metric triplet (ψ, ρ, ω)
//! - [`ResonanceTopology`] - Trait for resonance-based graph structures
//! - [`CalibrationOperator`] - Trait for configuration evolution operators
//! - [`GenerativePipeline`] - Trait for generative processing pipelines
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │              qops-core                   │
//! │  ┌─────────┐ ┌──────────┐ ┌──────────┐ │
//! │  │Signature│ │Resonance │ │Operators │ │
//! │  └─────────┘ └──────────┘ └──────────┘ │
//! │  ┌─────────┐ ┌──────────┐ ┌──────────┐ │
//! │  │Topology │ │ Pipeline │ │  Ledger  │ │
//! │  └─────────┘ └──────────┘ └──────────┘ │
//! └─────────────────────────────────────────┘
//! ```

pub mod signature;
pub mod resonance;
pub mod topology;
pub mod operators;
pub mod pipeline;
pub mod ledger;
pub mod field;
pub mod error;
pub mod holistic;

// Re-exports
pub use signature::{Signature, Signature3D, Signature5D};
pub use resonance::{resonance, resonance_5d, resonance_3d, resonance_gradient, validate_invariant, ResonanceConfig};
pub use topology::{ResonanceTopology, NodeSignature};
pub use operators::{CalibrationOperator, Configuration, DoubleKickOperator, UpdateKick, StabilizationKick};
pub use pipeline::{GenerativePipeline, PipelineConfig, PipelineState, SimplePipeline};
pub use ledger::{ResonanceLedger, LedgerEntry, ProofOfResonance, MemoryLedger};
pub use field::{MandorlaField, FieldVector, ResonanceAttractor};
pub use error::{QopsError, Result};
pub use holistic::{
    // Stages
    GenesisStage,
    // Kosmokrator
    KosmokratorConfig, KosmokratorState, KosmokratorStats,
    ProofOfResonanceResult, OperatorCandidate,
    // Chronokrator
    ChronokratorConfig, ChronokratorState, ChronokratorStats,
    ResonanceChannel, ExkalibrationVector, SpikeEvent,
    // Pfauenthron
    PfauenthronConfig, PfauenthronState, PfauenthronStats,
    Ophanim, MandorlaField as HolisticMandorlaField, Monolith, FinalizedFamily,
    // Matrix
    HolisticConfig, HolisticMatrix, HolisticStats, MatrixOutput,
};

/// QOPS version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// System name
pub const SYSTEM_NAME: &str = "QOPS - Unified Quantum Operator Processing System";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
