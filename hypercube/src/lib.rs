//! # QOPS Hypercube-HDAG 5D Framework
//!
//! Self-compiling cubes with hierarchical directed acyclic graph (HDAG) execution,
//! implementing the 5D operator framework for quantum operator synthesis.
//!
//! ## Core Concepts
//!
//! - **Hypercube**: 5D self-compiling cube structure with coordinate system (ψ, ρ, ω, χ, η)
//! - **HDAG**: Hierarchical Directed Acyclic Graph for execution flow
//! - **5D Operators**: DK (Double Kick), SW (Swap Wave), PI (Phase Integration), WT (Weight Transform)
//! - **Compilation Operator Ξ**: Transforms operator families into executable artifacts
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                     HYPERCUBE-HDAG 5D FRAMEWORK                          │
//! │                                                                          │
//! │   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐             │
//! │   │  HYPERCUBE   │ -> │     HDAG     │ -> │   ARTIFACT   │             │
//! │   │   (5D Cube)  │    │  (Exec DAG)  │    │  (Output)    │             │
//! │   │              │    │              │    │              │             │
//! │   │  DK,SW,PI,WT │    │   Nodes +    │    │   Compiled   │             │
//! │   │  Operators   │    │   Edges      │    │   Families   │             │
//! │   └──────────────┘    └──────────────┘    └──────────────┘             │
//! │         │                    │                    │                     │
//! │         v                    v                    v                     │
//! │   ┌─────────────────────────────────────────────────────────┐          │
//! │   │               Ξ Compilation Operator                      │          │
//! │   │     Self-compiling cube → operator family synthesis       │          │
//! │   └─────────────────────────────────────────────────────────┘          │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```

pub mod cube;
pub mod hdag;
pub mod operators;
pub mod compiler;
pub mod coordinates;
pub mod vertex;
pub mod edge;
pub mod artifact;
pub mod session;
pub mod triton_mode;
pub mod error;

// Re-exports
pub use cube::{Hypercube, HypercubeConfig, HypercubeState, CubeExpansionRule};
pub use hdag::{HDAG, HDAGNode, HDAGEdge, HDAGExecutor, ExecutionResult};
pub use operators::{
    Operator5D, OperatorType, OperatorFamily,
    DoubleKickOperator, SwapWaveOperator, PhaseIntegrationOperator, WeightTransformOperator,
    CompilationOperator,
};
pub use compiler::{HypercubeCompiler, CompilationConfig, CompilationResult};
pub use coordinates::{Coord5D, CoordinateSystem, CoordinateTransform};
pub use vertex::{HypercubeVertex, VertexType, VertexState};
pub use edge::{HypercubeEdge, EdgeType, EdgeWeight};
pub use artifact::{HypercubeArtifact, ArtifactType, ArtifactMetadata};
pub use session::{HypercubeSession, SessionConfig, SessionResult};
pub use triton_mode::{HypercubeTritonMode, TritonExpansionConfig};
pub use error::{HypercubeError, Result};

/// Hypercube framework version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default cube dimension
pub const DEFAULT_DIMENSION: usize = 5;

/// Default expansion iterations
pub const DEFAULT_EXPANSION_ITERATIONS: usize = 10;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
