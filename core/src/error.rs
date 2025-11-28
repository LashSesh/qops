//! Error types for QOPS.

use thiserror::Error;

/// Result type for QOPS operations
pub type Result<T> = std::result::Result<T, QopsError>;

/// Error types in QOPS
#[derive(Error, Debug)]
pub enum QopsError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Pipeline error
    #[error("Pipeline error: {0}")]
    Pipeline(String),

    /// Topology error
    #[error("Topology error: {0}")]
    Topology(String),

    /// Ledger error
    #[error("Ledger error: {0}")]
    Ledger(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Convergence error
    #[error("Convergence error: {0}")]
    Convergence(String),

    /// Quantum error
    #[error("Quantum error: {0}")]
    Quantum(String),

    /// Calibration error
    #[error("Calibration error: {0}")]
    Calibration(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl QopsError {
    /// Create a configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Configuration(msg.into())
    }

    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create a pipeline error
    pub fn pipeline(msg: impl Into<String>) -> Self {
        Self::Pipeline(msg.into())
    }

    /// Create a topology error
    pub fn topology(msg: impl Into<String>) -> Self {
        Self::Topology(msg.into())
    }

    /// Create a ledger error
    pub fn ledger(msg: impl Into<String>) -> Self {
        Self::Ledger(msg.into())
    }

    /// Create a quantum error
    pub fn quantum(msg: impl Into<String>) -> Self {
        Self::Quantum(msg.into())
    }

    /// Create a calibration error
    pub fn calibration(msg: impl Into<String>) -> Self {
        Self::Calibration(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = QopsError::config("Invalid parameter");
        assert!(err.to_string().contains("Configuration"));

        let err = QopsError::validation("Failed check");
        assert!(err.to_string().contains("Validation"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let qops_err: QopsError = io_err.into();
        assert!(matches!(qops_err, QopsError::Io(_)));
    }
}
