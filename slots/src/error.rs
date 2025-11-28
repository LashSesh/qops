//! Error types for the Slots engine.

use thiserror::Error;

/// Slots-specific errors
#[derive(Error, Debug)]
pub enum SlotsError {
    #[error("Invalid slot configuration: {0}")]
    InvalidConfig(String),

    #[error("Slot not found: {0}")]
    SlotNotFound(String),

    #[error("Lattice error: {0}")]
    LatticeError(String),

    #[error("Entropy generation error: {0}")]
    EntropyError(String),

    #[error("Mining error: {0}")]
    MiningError(String),

    #[error("Invalid topology: {0}")]
    InvalidTopology(String),

    #[error("Session error: {0}")]
    SessionError(String),

    #[error("Hypercube integration error: {0}")]
    HypercubeError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Result type for Slots operations
pub type Result<T> = std::result::Result<T, SlotsError>;
