//! Error types for the circuit module

use thiserror::Error;

/// Circuit error types
#[derive(Error, Debug, Clone)]
pub enum CircuitError {
    #[error("Invalid qubit index {0}, register has {1} qubits")]
    InvalidQubitIndex(usize, usize),

    #[error("Qubit indices must be different for two-qubit gates: got {0} and {1}")]
    SameQubitIndex(usize, usize),

    #[error("State vector dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Gate matrix is not unitary")]
    NonUnitaryGate,

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Circuit depth exceeded maximum: {0}")]
    MaxDepthExceeded(usize),

    #[error("Measurement error: {0}")]
    MeasurementError(String),

    #[error("Normalization error: state norm is {0}, expected 1.0")]
    NormalizationError(f64),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, CircuitError>;
