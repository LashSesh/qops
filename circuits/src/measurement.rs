//! Quantum Measurement - Projective measurements and observables
//!
//! This module provides measurement operations for quantum states.

use crate::{Complex, QuantumRegister, Result, CircuitError, ZERO, ONE};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Measurement basis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeasurementBasis {
    /// Computational basis (Z-basis): |0⟩, |1⟩
    Computational,
    /// X-basis (Hadamard basis): |+⟩, |−⟩
    X,
    /// Y-basis: |i⟩, |−i⟩
    Y,
    /// Bell basis (for two qubits)
    Bell,
}

/// Result of a single measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementResult {
    /// Measured qubit indices
    pub qubits: Vec<usize>,
    /// Measurement outcome as integer
    pub outcome: usize,
    /// Measurement outcome as bit string
    pub bitstring: String,
    /// Probability of this outcome
    pub probability: f64,
    /// Measurement basis used
    pub basis: MeasurementBasis,
}

impl MeasurementResult {
    /// Get outcome as boolean vector
    pub fn as_bools(&self) -> Vec<bool> {
        self.bitstring.chars().map(|c| c == '1').collect()
    }
}

/// Measurement statistics from multiple shots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementStatistics {
    /// Number of shots
    pub shots: usize,
    /// Counts per outcome
    pub counts: HashMap<String, usize>,
    /// Measured qubits
    pub qubits: Vec<usize>,
    /// Basis used
    pub basis: MeasurementBasis,
}

impl MeasurementStatistics {
    /// Get probabilities from counts
    pub fn probabilities(&self) -> HashMap<String, f64> {
        self.counts.iter()
            .map(|(k, &v)| (k.clone(), v as f64 / self.shots as f64))
            .collect()
    }

    /// Get most frequent outcome
    pub fn most_frequent(&self) -> Option<(&String, usize)> {
        self.counts.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(s, &c)| (s, c))
    }

    /// Get entropy of the distribution
    pub fn entropy(&self) -> f64 {
        let probs = self.probabilities();
        -probs.values()
            .filter(|&&p| p > 0.0)
            .map(|&p| p * p.log2())
            .sum::<f64>()
    }

    /// Display as histogram
    pub fn histogram(&self, width: usize) -> String {
        let max_count = self.counts.values().max().copied().unwrap_or(1);
        let mut lines = Vec::new();

        let mut sorted: Vec<_> = self.counts.iter().collect();
        sorted.sort_by_key(|(k, _)| *k);

        for (outcome, &count) in sorted {
            let bar_len = (count as f64 / max_count as f64 * width as f64) as usize;
            let bar = "█".repeat(bar_len);
            let prob = count as f64 / self.shots as f64;
            lines.push(format!("{}: {} {:.2}% ({})", outcome, bar, prob * 100.0, count));
        }

        lines.join("\n")
    }
}

/// Measurement operations
pub struct Measurement;

impl Measurement {
    /// Measure specific qubits in computational basis
    pub fn measure_qubits(
        register: &QuantumRegister,
        qubits: &[usize],
        shots: usize,
    ) -> Result<MeasurementStatistics> {
        for &q in qubits {
            if q >= register.num_qubits() {
                return Err(CircuitError::InvalidQubitIndex(q, register.num_qubits()));
            }
        }

        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut rng = rand::thread_rng();

        let probs = register.state.probabilities();

        for _ in 0..shots {
            // Sample from full distribution
            let r: f64 = rand::Rng::gen(&mut rng);
            let mut cumulative = 0.0;
            let mut full_outcome = 0;

            for (i, &p) in probs.iter().enumerate() {
                cumulative += p;
                if r < cumulative {
                    full_outcome = i;
                    break;
                }
            }

            // Extract only the measured qubits
            let mut outcome = 0;
            for (i, &q) in qubits.iter().enumerate() {
                if (full_outcome >> q) & 1 == 1 {
                    outcome |= 1 << i;
                }
            }

            let bitstring: String = (0..qubits.len())
                .rev()
                .map(|i| if (outcome >> i) & 1 == 1 { '1' } else { '0' })
                .collect();

            *counts.entry(bitstring).or_insert(0) += 1;
        }

        Ok(MeasurementStatistics {
            shots,
            counts,
            qubits: qubits.to_vec(),
            basis: MeasurementBasis::Computational,
        })
    }

    /// Measure all qubits
    pub fn measure_all(
        register: &QuantumRegister,
        shots: usize,
    ) -> MeasurementStatistics {
        let qubits: Vec<usize> = (0..register.num_qubits()).collect();
        Self::measure_qubits(register, &qubits, shots).unwrap()
    }

    /// Measure in X basis
    pub fn measure_x_basis(
        register: &mut QuantumRegister,
        qubit: usize,
    ) -> Result<bool> {
        use crate::Gate;

        // Apply Hadamard to rotate X basis to computational basis
        register.apply_single_gate(&Gate::h(), qubit)?;
        register.measure(qubit)
    }

    /// Measure in Y basis
    pub fn measure_y_basis(
        register: &mut QuantumRegister,
        qubit: usize,
    ) -> Result<bool> {
        use crate::Gate;

        // Apply S†H to rotate Y basis to computational basis
        register.apply_single_gate(&Gate::sdg(), qubit)?;
        register.apply_single_gate(&Gate::h(), qubit)?;
        register.measure(qubit)
    }

    /// Compute expectation value of a Pauli observable
    pub fn expectation_pauli(
        register: &QuantumRegister,
        pauli: &str,
    ) -> Result<f64> {
        if pauli.len() != register.num_qubits() {
            return Err(CircuitError::InvalidParameter(
                format!("Pauli string length {} doesn't match qubit count {}",
                    pauli.len(), register.num_qubits())
            ));
        }

        // Build Pauli operator matrix
        let pauli_matrix = Self::build_pauli_operator(pauli)?;

        // Compute ⟨ψ|P|ψ⟩
        let state_vec = register.state.to_vector();
        let result = state_vec.adjoint() * &pauli_matrix * &state_vec;

        Ok(result[(0, 0)].re)
    }

    /// Build Pauli operator from string (e.g., "XYZ" for X⊗Y⊗Z)
    fn build_pauli_operator(pauli_string: &str) -> Result<DMatrix<Complex>> {
        let i = Complex::new(0.0, 1.0);

        let pauli_i = DMatrix::from_row_slice(2, 2, &[ONE, ZERO, ZERO, ONE]);
        let pauli_x = DMatrix::from_row_slice(2, 2, &[ZERO, ONE, ONE, ZERO]);
        let pauli_y = DMatrix::from_row_slice(2, 2, &[ZERO, -i, i, ZERO]);
        let pauli_z = DMatrix::from_row_slice(2, 2, &[ONE, ZERO, ZERO, -ONE]);

        let mut result: Option<DMatrix<Complex>> = None;

        for c in pauli_string.chars() {
            let p = match c {
                'I' | 'i' => pauli_i.clone(),
                'X' | 'x' => pauli_x.clone(),
                'Y' | 'y' => pauli_y.clone(),
                'Z' | 'z' => pauli_z.clone(),
                _ => return Err(CircuitError::InvalidParameter(
                    format!("Invalid Pauli character: {}", c)
                )),
            };

            result = Some(match result {
                Some(r) => r.kronecker(&p),
                None => p,
            });
        }

        result.ok_or_else(|| CircuitError::InvalidParameter("Empty Pauli string".to_string()))
    }

    /// Compute variance of a Pauli observable
    pub fn variance_pauli(
        register: &QuantumRegister,
        pauli: &str,
    ) -> Result<f64> {
        let exp = Self::expectation_pauli(register, pauli)?;

        // For Pauli operators, <P²> = 1, so Var(P) = 1 - <P>²
        Ok(1.0 - exp * exp)
    }
}

/// Tomography operations
pub struct StateTomography;

impl StateTomography {
    /// Perform single-qubit state tomography
    /// Returns Bloch vector (x, y, z)
    pub fn single_qubit_tomography(
        prepare_state: impl Fn() -> QuantumRegister,
        shots: usize,
    ) -> (f64, f64, f64) {
        // Measure in Z basis
        let reg_z = prepare_state();
        let stats_z = Measurement::measure_all(&reg_z, shots);
        let p0_z = *stats_z.counts.get("0").unwrap_or(&0) as f64 / shots as f64;
        let z = 2.0 * p0_z - 1.0;

        // Measure in X basis (apply H before measurement)
        let mut reg_x = prepare_state();
        reg_x.apply_single_gate(&crate::Gate::h(), 0).unwrap();
        let stats_x = Measurement::measure_all(&reg_x, shots);
        let p0_x = *stats_x.counts.get("0").unwrap_or(&0) as f64 / shots as f64;
        let x = 2.0 * p0_x - 1.0;

        // Measure in Y basis (apply S†H before measurement)
        let mut reg_y = prepare_state();
        reg_y.apply_single_gate(&crate::Gate::sdg(), 0).unwrap();
        reg_y.apply_single_gate(&crate::Gate::h(), 0).unwrap();
        let stats_y = Measurement::measure_all(&reg_y, shots);
        let p0_y = *stats_y.counts.get("0").unwrap_or(&0) as f64 / shots as f64;
        let y = 2.0 * p0_y - 1.0;

        (x, y, z)
    }

    /// Estimate purity from tomography data
    pub fn estimate_purity(bloch_vector: (f64, f64, f64)) -> f64 {
        let (x, y, z) = bloch_vector;
        0.5 * (1.0 + x*x + y*y + z*z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Gate, Circuit};
    use approx::assert_relative_eq;

    #[test]
    fn test_measurement_statistics() {
        let mut reg = QuantumRegister::new(1);
        reg.apply_single_gate(&Gate::h(), 0).unwrap();

        let stats = Measurement::measure_all(&reg, 10000);

        let p0 = *stats.counts.get("0").unwrap_or(&0) as f64 / 10000.0;
        let p1 = *stats.counts.get("1").unwrap_or(&0) as f64 / 10000.0;

        // Should be roughly 50-50 for |+⟩ state
        assert!((p0 - 0.5).abs() < 0.05);
        assert!((p1 - 0.5).abs() < 0.05);
    }

    #[test]
    fn test_pauli_expectation() {
        // |0⟩ state should have <Z> = 1
        let reg = QuantumRegister::new(1);
        let exp_z = Measurement::expectation_pauli(&reg, "Z").unwrap();
        assert_relative_eq!(exp_z, 1.0, epsilon = 1e-10);

        // |+⟩ state should have <X> = 1, <Z> = 0
        let mut reg_plus = QuantumRegister::new(1);
        reg_plus.apply_single_gate(&Gate::h(), 0).unwrap();

        let exp_x = Measurement::expectation_pauli(&reg_plus, "X").unwrap();
        let exp_z = Measurement::expectation_pauli(&reg_plus, "Z").unwrap();

        assert_relative_eq!(exp_x, 1.0, epsilon = 1e-10);
        assert_relative_eq!(exp_z, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_bell_state_correlation() {
        let mut reg = QuantumRegister::new(2);
        reg.apply_circuit(&Circuit::bell_state()).unwrap();

        // Bell state should have <ZZ> = 1, <XX> = 1
        let exp_zz = Measurement::expectation_pauli(&reg, "ZZ").unwrap();
        let exp_xx = Measurement::expectation_pauli(&reg, "XX").unwrap();

        assert_relative_eq!(exp_zz, 1.0, epsilon = 1e-10);
        assert_relative_eq!(exp_xx, 1.0, epsilon = 1e-10);
    }
}
