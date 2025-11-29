//! # QOPS Kernel - Hypercube-Based Generative Theomimesis
//!
//! A domain-agnostic kernel for blueprint mining and transformation on high-dimensional
//! state spaces. This module implements the specification from "A Hypercube-Based Kernel
//! for Generative Theomimesis".
//!
//! ## Core Concepts
//!
//! - **State Space H^n**: High-dimensional state representation with core signature (ψ, ρ, ω, χ, η)
//! - **Domain Adapters**: Map domain-specific objects into the common state space
//! - **Resonance Function R**: Measures alignment with ideal pattern families
//! - **Core Operators**: Extract, Compose, Materialize pipeline
//! - **Mining Kernel M**: (Q, S, F, R) for operator-space exploration
//!
//! ## Architecture (Layers L1-L7)
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                 HYPERCUBE KERNEL FOR GENERATIVE THEOMIMESIS              │
//! │                                                                          │
//! │  L1: Domain Adapter Layer     - Maps objects to H^n via D_d              │
//! │  L2: Spectral/Signature Layer - Computes signatures and R(v)             │
//! │  L3: Hypercube/HDAG Layer     - Maintains Q and G structures             │
//! │  L4: Mining Layer             - Implements M = (Q, S, F, R)              │
//! │  L5: Materialization Layer    - Applies M to create artefacts            │
//! │  L6: User/Integration Layer   - GUI, API, CLI interfaces                 │
//! │  L7: Ledger/Governance Layer  - Records transformations B → A            │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```

pub mod state;
pub mod domain_adapters;
pub mod resonance;
pub mod operators;
pub mod mining;
pub mod materialization;
pub mod ledger;
pub mod config;
pub mod error;
pub mod blueprint;

// Re-exports
pub use state::{State, StateSpace, CoreSignature, ExtendedState};
pub use domain_adapters::{DomainAdapter, BlueprintAdapter, ArtefactAdapter};
pub use resonance::{ResonanceFunction, ResonanceModel, ResonanceThreshold};
pub use operators::{ExtractOperator, ComposeOperator, MaterializeOperator, KernelOperator};
pub use mining::{MiningKernel, MiningConfig, MiningResult, SearchStrategy, FilterSet};
pub use materialization::{Materializer, MaterializationResult, ArtefactOutput};
pub use ledger::{KernelLedger, LedgerRecord, TransformationEntry, MemoryLedger, FileLedger};
pub use config::{KernelConfig, MiningParameters, ResonanceParameters};
pub use error::{KernelError, Result};
pub use blueprint::{Blueprint, BlueprintCandidate, BlueprintMetadata};

/// Kernel version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default state space dimensionality
pub const DEFAULT_DIMENSION: usize = 5;

/// Kernel name
pub const KERNEL_NAME: &str = "Hypercube Kernel for Generative Theomimesis";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
