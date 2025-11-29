//! Error types for the kernel module.

use thiserror::Error;

/// Result type alias for kernel operations
pub type Result<T> = std::result::Result<T, KernelError>;

/// Kernel-specific error types
#[derive(Error, Debug)]
pub enum KernelError {
    /// State space dimension mismatch
    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    /// Invalid state values
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Resonance threshold not met
    #[error("Resonance threshold not met: {current} < {threshold}")]
    ResonanceThresholdNotMet { current: f64, threshold: f64 },

    /// Domain adapter error
    #[error("Domain adapter error: {0}")]
    AdapterError(String),

    /// Mining error
    #[error("Mining error: {0}")]
    MiningError(String),

    /// Materialization error
    #[error("Materialization error: {0}")]
    MaterializationError(String),

    /// Ledger error
    #[error("Ledger error: {0}")]
    LedgerError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Blueprint validation error
    #[error("Blueprint validation error: {0}")]
    BlueprintError(String),

    /// Constraint violation
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Hypercube error
    #[error("Hypercube error: {0}")]
    HypercubeError(String),

    /// HDAG execution error
    #[error("HDAG execution error: {0}")]
    HdagError(String),

    /// Generic internal error
    #[error("Internal kernel error: {0}")]
    InternalError(String),
}

impl From<serde_json::Error> for KernelError {
    fn from(e: serde_json::Error) -> Self {
        KernelError::SerializationError(e.to_string())
    }
}

impl From<toml::de::Error> for KernelError {
    fn from(e: toml::de::Error) -> Self {
        KernelError::ConfigError(e.to_string())
    }
}

impl From<qops_hypercube::HypercubeError> for KernelError {
    fn from(e: qops_hypercube::HypercubeError) -> Self {
        KernelError::HypercubeError(e.to_string())
    }
}
