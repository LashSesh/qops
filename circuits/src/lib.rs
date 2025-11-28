//! # QOPS Quantum Circuit Simulator
//!
//! A comprehensive quantum circuit simulator for research and experimentation.
//!
//! ## Features
//!
//! - **Qubit**: Single qubit representation with Bloch sphere coordinates
//! - **Gates**: Universal quantum gate set (Pauli, Hadamard, Phase, CNOT, etc.)
//! - **Circuit**: Quantum circuit construction and manipulation
//! - **Register**: Multi-qubit quantum registers with state vector simulation
//! - **Measurement**: Projective measurements with probability distribution
//! - **Noise**: Noise models for realistic simulation (optional)
//!
//! ## Example
//!
//! ```rust
//! use qops_circuits::{Circuit, Gate, QuantumRegister};
//!
//! // Create a 2-qubit register
//! let mut reg = QuantumRegister::new(2);
//!
//! // Build a Bell state circuit
//! let circuit = Circuit::new(2)
//!     .h(0)           // Hadamard on qubit 0
//!     .cnot(0, 1);    // CNOT with control=0, target=1
//!
//! // Execute the circuit
//! reg.apply_circuit(&circuit);
//!
//! // Measure
//! let result = reg.measure_all();
//! ```

pub mod qubit;
pub mod gates;
pub mod circuit;
pub mod register;
pub mod measurement;
pub mod noise;
pub mod error;

pub use qubit::{Qubit, BlochCoordinates};
pub use gates::{Gate, GateType, ControlledGate, ParameterizedGate};
pub use circuit::{Circuit, CircuitBuilder, CircuitInstruction};
pub use register::{QuantumRegister, StateVector};
pub use measurement::{Measurement, MeasurementResult, MeasurementBasis};
pub use noise::{NoiseModel, NoiseChannel, DepolarizingNoise, AmplitudeDamping};
pub use error::{CircuitError, Result};

use num_complex::Complex64;

/// Complex number type alias
pub type Complex = Complex64;

/// Zero complex number
pub const ZERO: Complex = Complex64::new(0.0, 0.0);

/// One complex number
pub const ONE: Complex = Complex64::new(1.0, 0.0);

/// Imaginary unit
pub const I: Complex = Complex64::new(0.0, 1.0);

/// 1/sqrt(2) for Hadamard normalization
pub const FRAC_1_SQRT_2: f64 = std::f64::consts::FRAC_1_SQRT_2;

/// Module version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(ZERO.re, 0.0);
        assert_eq!(ONE.re, 1.0);
        assert_eq!(I.im, 1.0);
    }
}
