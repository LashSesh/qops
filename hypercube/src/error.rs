//! Error types for the Hypercube framework.

use thiserror::Error;

/// Hypercube-specific errors
#[derive(Error, Debug)]
pub enum HypercubeError {
    #[error("Invalid coordinate: {0}")]
    InvalidCoordinate(String),

    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Invalid operator configuration: {0}")]
    InvalidOperator(String),

    #[error("HDAG execution error: {0}")]
    ExecutionError(String),

    #[error("Compilation failed: {0}")]
    CompilationError(String),

    #[error("Vertex not found: {0}")]
    VertexNotFound(String),

    #[error("Edge not found: {0}")]
    EdgeNotFound(String),

    #[error("Cycle detected in HDAG")]
    CycleDetected,

    #[error("Invalid expansion rule: {0}")]
    InvalidExpansionRule(String),

    #[error("Session error: {0}")]
    SessionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Core error: {0}")]
    CoreError(#[from] qops_core::QopsError),
}

/// Result type for Hypercube operations
pub type Result<T> = std::result::Result<T, HypercubeError>;
