//! Error types for research tools

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ResearchError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Experiment failed: {0}")]
    ExperimentFailed(String),

    #[error("Benchmark error: {0}")]
    BenchmarkError(String),

    #[error("Analysis error: {0}")]
    AnalysisError(String),

    #[error("Export error: {0}")]
    ExportError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

pub type Result<T> = std::result::Result<T, ResearchError>;
