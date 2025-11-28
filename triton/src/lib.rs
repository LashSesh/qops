//! # TRITON Spiral Search Optimizer
//!
//! Advanced search optimization algorithm for quantum operator mining using
//! spiral trajectory patterns in topology space.
//!
//! ## Core Concepts
//!
//! - **Spiral Search**: Outward-expanding search pattern from seed points
//! - **Temperature Annealing**: Probabilistic acceptance of suboptimal moves
//! - **Multi-Pass Refinement**: Iterative narrowing of search space
//! - **Topology Biasing**: Leveraging graph structure for guided exploration
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    TRITON Optimizer                          │
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
//! │  │ SpiralEngine │  │ Temperature  │  │ Topology     │       │
//! │  │              │  │ Controller   │  │ Bias         │       │
//! │  └──────────────┘  └──────────────┘  └──────────────┘       │
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
//! │  │ LayerManager │  │ Score Cache  │  │ Refinement   │       │
//! │  │              │  │              │  │ Engine       │       │
//! │  └──────────────┘  └──────────────┘  └──────────────┘       │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Example
//!
//! ```ignore
//! use qops_triton::{TritonOptimizer, TritonConfig, SpiralParams};
//! use qops_core::Signature5D;
//!
//! let config = TritonConfig::default();
//! let mut optimizer = TritonOptimizer::new(config);
//!
//! // Run spiral search
//! let result = optimizer.optimize(|sig| {
//!     qops_core::resonance_5d(sig)
//! });
//!
//! println!("Best resonance: {}", result.best_score);
//! ```

pub mod config;
pub mod spiral;
pub mod temperature;
pub mod layer;
pub mod optimizer;
pub mod search;
pub mod topology_bias;
pub mod refinement;
pub mod scoring;
pub mod session;

// Re-exports
pub use config::{TritonConfig, SpiralParams, TemperatureSchedule, RefinementConfig};
pub use spiral::{SpiralEngine, SpiralState, SpiralDirection};
pub use temperature::{TemperatureController, AnnealingStrategy};
pub use layer::{LayerManager, SearchLayer, LayerMetrics};
pub use optimizer::{TritonOptimizer, OptimizationResult, OptimizationStep};
pub use search::{SearchStrategy, SearchState, SearchMetadata};
pub use topology_bias::{TopologyBias, BiasMode, NeighborhoodWeights};
pub use refinement::{RefinementEngine, RefinementPass, RefinementResult};
pub use scoring::{ScoringFunction, ScoreCache, CompositeScore};
pub use session::{TritonSession, SessionConfig, SessionLog, SessionEvent};

/// TRITON version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default spiral expansion rate
pub const DEFAULT_EXPANSION_RATE: f64 = 1.618; // Golden ratio

/// Default temperature decay factor
pub const DEFAULT_TEMPERATURE_DECAY: f64 = 0.95;

/// Default number of spiral layers
pub const DEFAULT_LAYER_COUNT: usize = 7;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
