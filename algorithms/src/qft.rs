//! Quantum Fourier Transform (QFT)
//!
//! The quantum analog of the discrete Fourier transform.
//!
//! ## Transform
//! |j⟩ → (1/√N) Σₖ exp(2πijk/N) |k⟩
//!
//! ## Applications
//! - Phase estimation
//! - Shor's algorithm
//! - Quantum signal processing

use qops_circuits::{Circuit, Gate, QuantumRegister, Complex};
use crate::{AlgorithmError, Result};
use std::f64::consts::PI;

/// Quantum Fourier Transform
pub struct QuantumFourierTransform {
    /// Number of qubits
    pub num_qubits: usize,
    /// Whether to include swap operations for output ordering
    pub swap_output: bool,
    /// Use approximate QFT (truncate small rotations)
    pub approximate: bool,
    /// Approximation cutoff (skip rotations smaller than 2π/2^cutoff)
    pub approximation_cutoff: usize,
}

impl QuantumFourierTransform {
    /// Create a new QFT instance
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            swap_output: true,
            approximate: false,
            approximation_cutoff: 10,
        }
    }

    /// Disable output swapping (useful when QFT is part of larger algorithm)
    pub fn without_swap(mut self) -> Self {
        self.swap_output = false;
        self
    }

    /// Use approximate QFT for better performance on noisy hardware
    pub fn approximate(mut self, cutoff: usize) -> Self {
        self.approximate = true;
        self.approximation_cutoff = cutoff;
        self
    }

    /// Build the QFT circuit
    pub fn build_circuit(&self) -> Circuit {
        let n = self.num_qubits;
        let mut circuit = Circuit::with_name(n, "QFT");

        for i in 0..n {
            // Hadamard on qubit i
            circuit = circuit.h(i);

            // Controlled rotations
            for j in (i + 1)..n {
                let k = j - i + 1;

                // Skip small rotations in approximate QFT
                if self.approximate && k > self.approximation_cutoff {
                    continue;
                }

                // Controlled-R_k rotation
                let theta = PI / (1 << (k - 1)) as f64;
                circuit = circuit.cphase(theta, j, i);
            }
        }

        // Swap qubits to get correct output ordering
        if self.swap_output {
            for i in 0..n / 2 {
                circuit = circuit.swap(i, n - 1 - i);
            }
        }

        circuit
    }

    /// Apply QFT to a register
    pub fn apply(&self, register: &mut QuantumRegister) -> Result<()> {
        if register.num_qubits() != self.num_qubits {
            return Err(AlgorithmError::InvalidQubitCount(
                format!("Expected {} qubits, got {}", self.num_qubits, register.num_qubits())
            ));
        }

        let circuit = self.build_circuit();
        register.apply_circuit(&circuit)
            .map_err(|e| AlgorithmError::CircuitError(e.to_string()))
    }

    /// Compute classical DFT for comparison
    pub fn classical_dft(input: &[Complex]) -> Vec<Complex> {
        let n = input.len();
        let mut output = vec![Complex::new(0.0, 0.0); n];

        for k in 0..n {
            for j in 0..n {
                let angle = 2.0 * PI * (j * k) as f64 / n as f64;
                let phase = Complex::from_polar(1.0, angle);
                output[k] += input[j] * phase;
            }
            output[k] /= (n as f64).sqrt();
        }

        output
    }

    /// Get QFT matrix (for analysis)
    pub fn matrix(&self) -> Vec<Vec<Complex>> {
        let n = 1 << self.num_qubits;
        let norm = 1.0 / (n as f64).sqrt();

        (0..n).map(|j| {
            (0..n).map(|k| {
                let angle = 2.0 * PI * (j * k) as f64 / n as f64;
                Complex::from_polar(norm, angle)
            }).collect()
        }).collect()
    }
}

/// Inverse Quantum Fourier Transform
pub struct IQFT {
    /// Number of qubits
    pub num_qubits: usize,
    /// Whether to include swap operations
    pub swap_input: bool,
}

impl IQFT {
    /// Create a new inverse QFT instance
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            swap_input: true,
        }
    }

    /// Build the inverse QFT circuit
    pub fn build_circuit(&self) -> Circuit {
        // IQFT is the reverse of QFT with negative phases
        let n = self.num_qubits;
        let mut circuit = Circuit::with_name(n, "IQFT");

        // Swap first if needed
        if self.swap_input {
            for i in 0..n / 2 {
                circuit = circuit.swap(i, n - 1 - i);
            }
        }

        // Reverse order of operations
        for i in (0..n).rev() {
            // Controlled rotations (with negative angles)
            for j in ((i + 1)..n).rev() {
                let k = j - i + 1;
                let theta = -PI / (1 << (k - 1)) as f64;
                circuit = circuit.cphase(theta, j, i);
            }

            // Hadamard on qubit i
            circuit = circuit.h(i);
        }

        circuit
    }

    /// Apply inverse QFT to a register
    pub fn apply(&self, register: &mut QuantumRegister) -> Result<()> {
        if register.num_qubits() != self.num_qubits {
            return Err(AlgorithmError::InvalidQubitCount(
                format!("Expected {} qubits, got {}", self.num_qubits, register.num_qubits())
            ));
        }

        let circuit = self.build_circuit();
        register.apply_circuit(&circuit)
            .map_err(|e| AlgorithmError::CircuitError(e.to_string()))
    }
}

/// QFT-based quantum adder (Draper adder)
pub struct QuantumAdder {
    /// Number of qubits per number
    pub num_qubits: usize,
}

impl QuantumAdder {
    pub fn new(num_qubits: usize) -> Self {
        Self { num_qubits }
    }

    /// Build circuit to add classical value b to quantum register |a⟩
    /// Result: |a⟩ → |a + b mod 2^n⟩
    pub fn add_classical(&self, b: usize) -> Circuit {
        let n = self.num_qubits;
        let mut circuit = Circuit::with_name(n, &format!("Add_{}", b));

        // Apply QFT
        let qft = QuantumFourierTransform::new(n).without_swap();
        circuit.append(&qft.build_circuit()).ok();

        // Apply phase rotations based on b
        for j in 0..n {
            let mut angle = 0.0;
            for k in 0..n {
                if (b >> k) & 1 == 1 {
                    angle += PI / (1 << (j - k)) as f64;
                }
            }
            if angle.abs() > 1e-10 {
                circuit = circuit.rz(2.0 * angle, n - 1 - j);
            }
        }

        // Apply inverse QFT
        let iqft = IQFT::new(n);
        let iqft_circuit = iqft.build_circuit();
        // Remove swaps from IQFT since we didn't swap in QFT
        circuit.append(&iqft_circuit).ok();

        circuit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_qft_inverse() {
        let n = 3;
        let mut reg = QuantumRegister::new(n);

        // Start with some state
        reg.apply_single_gate(&Gate::h(), 0).unwrap();
        reg.apply_single_gate(&Gate::x(), 1).unwrap();

        let original_probs = reg.state.probabilities().clone();

        // Apply QFT
        let qft = QuantumFourierTransform::new(n);
        qft.apply(&mut reg).unwrap();

        // Apply inverse QFT
        let iqft = IQFT::new(n);
        iqft.apply(&mut reg).unwrap();

        // Should return to original state
        let final_probs = reg.state.probabilities();
        for (i, (&orig, &final_p)) in original_probs.iter().zip(final_probs.iter()).enumerate() {
            assert_relative_eq!(orig, final_p, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_qft_basis_state() {
        // QFT of |0⟩ should give uniform superposition
        let mut reg = QuantumRegister::new(2);

        let qft = QuantumFourierTransform::new(2);
        qft.apply(&mut reg).unwrap();

        let probs = reg.state.probabilities();
        for &p in &probs {
            assert_relative_eq!(p, 0.25, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_approximate_qft() {
        let n = 4;

        let exact_qft = QuantumFourierTransform::new(n);
        let approx_qft = QuantumFourierTransform::new(n).approximate(2);

        // Both should work, approximate has fewer gates
        let exact_circuit = exact_qft.build_circuit();
        let approx_circuit = approx_qft.build_circuit();

        assert!(approx_circuit.gate_count() <= exact_circuit.gate_count());
    }
}
