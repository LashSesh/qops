//! # QOPS Quantum Slots Engine (QSlots)
//!
//! A slot-based system for quantum operator generation using entropy mapping,
//! lattice search, and generative logic.
//!
//! ## Core Concepts
//!
//! - **Slot**: A configurable unit of operator generation with spin and entropy
//! - **Slot Lattice**: Multi-dimensional grid of interconnected slots
//! - **Entropy Mapping**: Maps randomness to structured operator outcomes
//! - **Sequence Mining**: Generates optimized operator sequences from slot configurations
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                      QUANTUM SLOTS ENGINE                                │
//! │                                                                          │
//! │   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐             │
//! │   │    SLOT      │ -> │   ENTROPY    │ -> │   SEQUENCE   │             │
//! │   │   LATTICE    │    │   MAPPING    │    │    MINER     │             │
//! │   │              │    │              │    │              │             │
//! │   │   Topology   │    │  Stochastic  │    │   Operator   │             │
//! │   │   Config     │    │  Generators  │    │   Families   │             │
//! │   └──────────────┘    └──────────────┘    └──────────────┘             │
//! │                                                                          │
//! │   ┌─────────────────────────────────────────────────────────┐          │
//! │   │            HYPERCUBE INTEGRATION                          │          │
//! │   │     Slots as artifact outputs from HDAG nodes             │          │
//! │   └─────────────────────────────────────────────────────────┘          │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```

pub mod slot;
pub mod lattice;
pub mod entropy;
pub mod generator;
pub mod miner;
pub mod topology;
pub mod spin;
pub mod session;
pub mod hypercube_integration;
pub mod error;

// Re-exports
pub use slot::{Slot, SlotConfig, SlotState, SlotValue, SlotSymbol};
pub use lattice::{SlotLattice, LatticeConfig, LatticeNode, LatticeEdge};
pub use entropy::{EntropyMapper, EntropyConfig, EntropySource, EntropyDistribution};
pub use generator::{SlotGenerator, GeneratorConfig, GeneratedSlot};
pub use miner::{SequenceMiner, MinerConfig, MiningResult, MinedSequence};
pub use topology::{SlotTopology, TopologyType, TopologyMetrics};
pub use spin::{SlotSpin, SpinState, SpinDynamics};
pub use session::{SlotsSession, SlotsSessionConfig, SlotsSessionResult};
pub use hypercube_integration::{SlotsHypercubeAdapter, SlotArtifact};
pub use error::{SlotsError, Result};

/// Slots engine version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default slot count
pub const DEFAULT_SLOT_COUNT: usize = 5;

/// Default entropy depth
pub const DEFAULT_ENTROPY_DEPTH: usize = 8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
