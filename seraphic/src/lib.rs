//! # QOPS Seraphic Calibration Shell
//!
//! Meta-algorithm for fixpoint-directed configuration evolution.
//!
//! ## Core Concepts
//!
//! - **Performance Triplet Φ(c) = (ψ, ρ, ω)**: Quality, Stability, Efficiency
//! - **Mandorla Field M(t)**: 16-dimensional resonance field
//! - **Double-Kick Operator T = Φ_V ∘ Φ_U**: Configuration evolution
//! - **Proof-of-Resonance (PoR)**: Acceptance criterion
//! - **CRI**: Calibration Regime Initialization

pub mod calibrator;
pub mod por;
pub mod cri;

pub use calibrator::{SeraphicCalibrator, CalibratorConfig, CalibrationResult};
pub use por::ProofOfResonanceValidator;
pub use cri::CalibrationRegimeInitializer;

/// Seraphic version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
