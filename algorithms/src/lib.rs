//! # QOPS Quantum Algorithms
//!
//! Classical quantum algorithms for research and education.
//!
//! ## Implemented Algorithms
//!
//! ### Search & Optimization
//! - **Grover's Algorithm**: Quadratic speedup for unstructured search
//! - **QAOA**: Quantum Approximate Optimization Algorithm
//!
//! ### Number Theory
//! - **Shor's Algorithm**: Integer factorization
//! - **Quantum Phase Estimation (QPE)**: Eigenvalue estimation
//!
//! ### Transforms
//! - **Quantum Fourier Transform (QFT)**: Quantum analog of DFT
//!
//! ### Simulation
//! - **Hamiltonian Simulation**: Time evolution of quantum systems
//! - **VQE**: Variational Quantum Eigensolver
//!
//! ## Example
//!
//! ```rust
//! use qops_algorithms::grover::{Grover, Oracle};
//!
//! // Search for |101⟩ in 3-qubit space
//! let oracle = Oracle::marked_state(3, 0b101);
//! let grover = Grover::new(3, oracle);
//! let result = grover.run();
//! ```

pub mod grover;
pub mod qft;
pub mod qpe;
pub mod shor;
pub mod vqe;
pub mod qaoa;
pub mod hamiltonian;
pub mod error;

pub use grover::{Grover, Oracle, GroverResult};
pub use qft::{QuantumFourierTransform, IQFT};
pub use qpe::{QuantumPhaseEstimation, QPEResult};
pub use shor::{Shor, ShorResult, FactorizationMethod};
pub use vqe::{VQE, VQEConfig, VQEResult, Ansatz};
pub use qaoa::{QAOA, QAOAConfig, QAOAResult, CostFunction};
pub use hamiltonian::{HamiltonianSimulation, TrotterDecomposition};
pub use error::{AlgorithmError, Result};

/// Module version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
