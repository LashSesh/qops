//! Error types for quantum algorithms

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AlgorithmError {
    #[error("Invalid number of qubits: {0}")]
    InvalidQubitCount(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Convergence failed after {0} iterations")]
    ConvergenceFailed(usize),

    #[error("Oracle error: {0}")]
    OracleError(String),

    #[error("Factorization failed: {0}")]
    FactorizationFailed(String),

    #[error("Circuit error: {0}")]
    CircuitError(String),

    #[error("Numerical error: {0}")]
    NumericalError(String),
}

pub type Result<T> = std::result::Result<T, AlgorithmError>;
