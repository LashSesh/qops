//! Error types for the GUI application

use serde::Serialize;
use thiserror::Error;

/// Application error types
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Circuit error: {0}")]
    Circuit(String),

    #[error("Algorithm error: {0}")]
    Algorithm(String),

    #[error("Genesis error: {0}")]
    Genesis(String),

    #[error("Quantum error: {0}")]
    Quantum(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("State error: {0}")]
    State(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<qops_circuits::CircuitError> for AppError {
    fn from(e: qops_circuits::CircuitError) -> Self {
        AppError::Circuit(e.to_string())
    }
}

impl From<qops_algorithms::AlgorithmError> for AppError {
    fn from(e: qops_algorithms::AlgorithmError) -> Self {
        AppError::Algorithm(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Serialization(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
