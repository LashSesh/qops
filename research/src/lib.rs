//! # QOPS Research Tools
//!
//! Comprehensive toolkit for quantum algorithm research and experimentation.
//!
//! ## Features
//!
//! - **Benchmarking**: Performance evaluation of quantum algorithms
//! - **Experiments**: Structured experiment management and reproducibility
//! - **Analysis**: Statistical analysis of quantum computation results
//! - **Visualization**: Data export for plotting (JSON, CSV)
//! - **Comparison**: Compare different algorithms and configurations
//!
//! ## Example
//!
//! ```rust,ignore
//! use qops_research::{Experiment, Benchmark, Analysis};
//!
//! // Create an experiment
//! let mut exp = Experiment::new("grover_scaling")
//!     .description("Study Grover speedup with problem size")
//!     .parameter("qubits", vec![2, 3, 4, 5]);
//!
//! // Run benchmarks
//! let results = exp.run(|params| {
//!     // Run Grover's algorithm with given parameters
//!     // Return measurement results
//! });
//!
//! // Analyze results
//! let analysis = Analysis::from_experiment(&results);
//! let summary = analysis.summary("execution_time");
//! ```

pub mod benchmark;
pub mod experiment;
pub mod analysis;
pub mod visualization;
pub mod comparison;
pub mod report;
pub mod error;

pub use benchmark::{Benchmark, BenchmarkConfig, BenchmarkResult, BenchmarkSuite};
pub use experiment::{Experiment, ExperimentConfig, ExperimentResult, Parameter};
pub use analysis::{Analysis, StatisticalSummary, QuantumMetrics};
pub use visualization::{DataExport, PlotData, ExportFormat};
pub use comparison::{Comparison, ComparisonResult, AlgorithmComparison};
pub use report::{Report, ReportFormat, ReportSection};
pub use error::{ResearchError, Result};

/// Module version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
