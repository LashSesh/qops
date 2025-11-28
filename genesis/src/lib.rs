//! # QOPS Genesis Pipeline
//!
//! S7 Topology Operator Mining System based on the Metatron Cube (5040 nodes).
//!
//! ## Features
//!
//! - **MetatronCube**: S7 permutation graph with 5040 nodes
//! - **Agent**: Traversal agents with various strategies
//! - **Artefact**: Mining artefacts with blueprint history
//! - **Cubechain**: Hypercube-DAG ledger with Proof-of-Resonance
//! - **MetaCognition**: Self-reflection and pattern analysis layer
//! - **KNO Framework**: Cyclic Conversion Operator system
//! - **Mining**: TRITON-integrated mining session management
//! - **Family**: Operator family extraction and clustering
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    GENESIS Pipeline                          │
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
//! │  │ MetatronCube │  │    Agent     │  │   Artefact   │       │
//! │  │  (S7 5040)   │  │  Traversal   │  │   Mining     │       │
//! │  └──────────────┘  └──────────────┘  └──────────────┘       │
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
//! │  │   TRITON     │  │   Family     │  │  Evolution   │       │
//! │  │  Optimizer   │  │  Clustering  │  │   Engine     │       │
//! │  └──────────────┘  └──────────────┘  └──────────────┘       │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod metatron_cube;
pub mod agent;
pub mod artefact;
pub mod cubechain;
pub mod meta_cognition;
pub mod kno;
pub mod traversal;
pub mod evolution;
pub mod mining;
pub mod family;

pub use metatron_cube::MetatronCube;
pub use agent::{Agent, AgentConfig, TraversalStrategy};
pub use artefact::Artefact;
pub use cubechain::Cubechain;
pub use meta_cognition::MetaCognitionLayer;
pub use traversal::TraversalEngine;
pub use evolution::{EvolutionEngine, EvolutionConfig, GenerationStats};
pub use mining::{MiningSession, MiningConfig, MiningStrategy, MiningResult, MiningStats};
pub use family::{OperatorFamily, FamilyClusterer, FamilyMetrics, FamilyCharacteristics};

/// Number of nodes in S7 (7! permutations)
pub const S7_NODE_COUNT: usize = 5040;

/// Genesis Pipeline version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
