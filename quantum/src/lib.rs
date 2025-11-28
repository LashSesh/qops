//! # QOPS Quantum Pipeline
//!
//! Quantum computing algorithms based on the 13-node Metatron Cube geometry.
//!
//! ## Features
//!
//! - **MetatronGraph**: 13-node quantum graph (1 center + 6 hexagon + 6 cube)
//! - **QuantumState**: Complex amplitude state vectors
//! - **Hamiltonian**: Graph-based Hamiltonians with spectral decomposition
//! - **VQA**: Variational Quantum Algorithms (VQE, QAOA, VQC)
//! - **QuantumWalk**: Continuous-time quantum walks
//! - **DTL**: Dynamic Tripolar Logic
//! - **Topology**: CUBE-13 topology engine with embedding and analysis
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    CUBE-13 Topology                          │
//! │                                                              │
//! │                         [0]                                  │
//! │                       Center                                 │
//! │                     /   |   \                                │
//! │              [1]  [2]  [3]  [4]  [5]  [6]                   │
//! │                    Hexagon Ring                              │
//! │               |    |    |    |    |    |                     │
//! │              [7]  [8]  [9] [10] [11] [12]                   │
//! │                     Outer Cube                               │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod graph;
pub mod state;
pub mod hamiltonian;
pub mod vqa;
pub mod quantum_walk;
pub mod dtl;
pub mod topology;

pub use graph::MetatronGraph;
pub use state::QuantumState;
pub use hamiltonian::MetatronHamiltonian;
pub use quantum_walk::ContinuousQuantumWalk;
pub use topology::{
    Cube13Engine, Cube13NodeType, TopologyExplorer,
    TopologyMetrics, TopologyWalkResult, OperatorCluster,
};

/// Metatron dimension (13 nodes)
pub const METATRON_DIMENSION: usize = 13;

/// Quantum Pipeline version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
