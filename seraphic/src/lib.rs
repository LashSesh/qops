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
//! - **Analysis**: Hyperparameter sweeps, stability analysis, auto-tuning
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                 SERAPHIC Calibration Shell                   │
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
//! │  │ Calibrator   │  │    PoR       │  │    CRI       │       │
//! │  │              │  │  Validator   │  │ Initializer  │       │
//! │  └──────────────┘  └──────────────┘  └──────────────┘       │
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
//! │  │ Hyperparameter│ │  Stability   │  │   Auto       │       │
//! │  │   Sweep      │  │  Analysis    │  │   Tuner      │       │
//! │  └──────────────┘  └──────────────┘  └──────────────┘       │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod calibrator;
pub mod por;
pub mod cri;
pub mod analysis;

pub use calibrator::{SeraphicCalibrator, CalibratorConfig, CalibrationResult, HistoryEntry};
pub use por::ProofOfResonanceValidator;
pub use cri::CalibrationRegimeInitializer;
pub use analysis::{
    HyperparameterSweep, SweepConfig, SweepResult, SweepConfigSnapshot, ConfigEvaluation,
    StabilityAnalysis, analyze_stability,
    AutoTuner, AutoTuneResult, AutoTuneConfig, CalibrationCurve, CurveType,
};

/// Seraphic version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
