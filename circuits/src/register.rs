//! Quantum Register - Multi-qubit state vector simulation
//!
//! This module provides the core simulation engine for quantum circuits.

use crate::{Complex, Gate, Circuit, CircuitError, Result, ZERO, ONE};
use nalgebra::DMatrix;
use rand::Rng;

/// State vector representation of a quantum register
#[derive(Debug, Clone)]
pub struct StateVector {
    /// Number of qubits
    pub num_qubits: usize,
    /// State amplitudes (length = 2^num_qubits)
    amplitudes: Vec<Complex>,
}

impl StateVector {
    /// Create a new state vector initialized to |0...0⟩
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;
        let mut amplitudes = vec![ZERO; dim];
        amplitudes[0] = ONE; // |0...0⟩
        Self { num_qubits, amplitudes }
    }

    /// Create from existing amplitudes
    pub fn from_amplitudes(amplitudes: Vec<Complex>) -> Result<Self> {
        let dim = amplitudes.len();
        if dim == 0 || (dim & (dim - 1)) != 0 {
            return Err(CircuitError::InvalidState(
                "Amplitudes length must be a power of 2".to_string()
            ));
        }
        let num_qubits = (dim as f64).log2() as usize;
        Ok(Self { num_qubits, amplitudes })
    }

    /// Get the dimension (2^n)
    pub fn dimension(&self) -> usize {
        self.amplitudes.len()
    }

    /// Get amplitude for a basis state
    pub fn amplitude(&self, index: usize) -> Complex {
        self.amplitudes.get(index).copied().unwrap_or(ZERO)
    }

    /// Get all amplitudes
    pub fn amplitudes(&self) -> &[Complex] {
        &self.amplitudes
    }

    /// Set amplitudes (will normalize)
    pub fn set_amplitudes(&mut self, amplitudes: Vec<Complex>) -> Result<()> {
        if amplitudes.len() != self.dimension() {
            return Err(CircuitError::DimensionMismatch {
                expected: self.dimension(),
                actual: amplitudes.len(),
            });
        }
        self.amplitudes = amplitudes;
        self.normalize();
        Ok(())
    }

    /// Create a uniform superposition
    pub fn uniform_superposition(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;
        let amp = Complex::new(1.0 / (dim as f64).sqrt(), 0.0);
        Self {
            num_qubits,
            amplitudes: vec![amp; dim],
        }
    }

    /// Create a specific basis state |i⟩
    pub fn basis_state(num_qubits: usize, index: usize) -> Result<Self> {
        let dim = 1 << num_qubits;
        if index >= dim {
            return Err(CircuitError::InvalidQubitIndex(index, num_qubits));
        }
        let mut amplitudes = vec![ZERO; dim];
        amplitudes[index] = ONE;
        Ok(Self { num_qubits, amplitudes })
    }

    /// Normalize the state vector
    pub fn normalize(&mut self) {
        let norm: f64 = self.amplitudes.iter().map(|a| a.norm_sqr()).sum::<f64>().sqrt();
        if norm > 1e-15 {
            for amp in &mut self.amplitudes {
                *amp /= norm;
            }
        }
    }

    /// Check if normalized
    pub fn is_normalized(&self) -> bool {
        let norm_sq: f64 = self.amplitudes.iter().map(|a| a.norm_sqr()).sum();
        (norm_sq - 1.0).abs() < 1e-10
    }

    /// Get the norm squared
    pub fn norm_squared(&self) -> f64 {
        self.amplitudes.iter().map(|a| a.norm_sqr()).sum()
    }

    /// Probability distribution over basis states
    pub fn probabilities(&self) -> Vec<f64> {
        self.amplitudes.iter().map(|a| a.norm_sqr()).collect()
    }

    /// Probability of measuring a specific basis state
    pub fn probability(&self, index: usize) -> f64 {
        self.amplitudes.get(index).map(|a| a.norm_sqr()).unwrap_or(0.0)
    }

    /// Inner product with another state
    pub fn inner_product(&self, other: &StateVector) -> Complex {
        self.amplitudes.iter()
            .zip(other.amplitudes.iter())
            .map(|(a, b)| a.conj() * b)
            .sum()
    }

    /// Fidelity with another state: |⟨ψ|φ⟩|²
    pub fn fidelity(&self, other: &StateVector) -> f64 {
        self.inner_product(other).norm_sqr()
    }

    /// Convert to nalgebra vector
    pub fn to_vector(&self) -> DMatrix<Complex> {
        DMatrix::from_column_slice(self.dimension(), 1, &self.amplitudes)
    }

    /// Apply a matrix operator
    pub fn apply_matrix(&mut self, matrix: &DMatrix<Complex>) {
        let state_vec = self.to_vector();
        let result = matrix * state_vec;
        self.amplitudes = result.iter().copied().collect();
    }
}

/// Quantum Register with simulation capabilities
#[derive(Debug, Clone)]
pub struct QuantumRegister {
    /// The quantum state
    pub state: StateVector,
    /// Classical bits for measurement results
    pub classical_bits: Vec<bool>,
    /// History of applied gates (for debugging/visualization)
    pub gate_history: Vec<String>,
}

impl QuantumRegister {
    /// Create a new quantum register with n qubits
    pub fn new(num_qubits: usize) -> Self {
        Self {
            state: StateVector::new(num_qubits),
            classical_bits: vec![false; num_qubits],
            gate_history: Vec::new(),
        }
    }

    /// Create from a state vector
    pub fn from_state(state: StateVector) -> Self {
        let num_qubits = state.num_qubits;
        Self {
            state,
            classical_bits: vec![false; num_qubits],
            gate_history: Vec::new(),
        }
    }

    /// Number of qubits
    pub fn num_qubits(&self) -> usize {
        self.state.num_qubits
    }

    /// Reset to |0...0⟩
    pub fn reset(&mut self) {
        self.state = StateVector::new(self.num_qubits());
        self.classical_bits = vec![false; self.num_qubits()];
        self.gate_history.clear();
    }

    /// Apply a single-qubit gate to qubit at index
    pub fn apply_single_gate(&mut self, gate: &Gate, qubit: usize) -> Result<()> {
        if gate.num_qubits != 1 {
            return Err(CircuitError::InvalidParameter(
                format!("Expected single-qubit gate, got {}-qubit gate", gate.num_qubits)
            ));
        }
        if qubit >= self.num_qubits() {
            return Err(CircuitError::InvalidQubitIndex(qubit, self.num_qubits()));
        }

        let full_matrix = self.expand_single_gate(gate, qubit);
        self.state.apply_matrix(&full_matrix);
        self.gate_history.push(format!("{}({})", gate.name, qubit));
        Ok(())
    }

    /// Apply a two-qubit gate
    pub fn apply_two_qubit_gate(&mut self, gate: &Gate, qubit1: usize, qubit2: usize) -> Result<()> {
        if gate.num_qubits != 2 {
            return Err(CircuitError::InvalidParameter(
                format!("Expected two-qubit gate, got {}-qubit gate", gate.num_qubits)
            ));
        }
        if qubit1 == qubit2 {
            return Err(CircuitError::SameQubitIndex(qubit1, qubit2));
        }
        if qubit1 >= self.num_qubits() {
            return Err(CircuitError::InvalidQubitIndex(qubit1, self.num_qubits()));
        }
        if qubit2 >= self.num_qubits() {
            return Err(CircuitError::InvalidQubitIndex(qubit2, self.num_qubits()));
        }

        let full_matrix = self.expand_two_qubit_gate(gate, qubit1, qubit2);
        self.state.apply_matrix(&full_matrix);
        self.gate_history.push(format!("{}({},{})", gate.name, qubit1, qubit2));
        Ok(())
    }

    /// Apply a circuit
    pub fn apply_circuit(&mut self, circuit: &Circuit) -> Result<()> {
        for instruction in &circuit.instructions {
            match instruction.qubits.len() {
                1 => self.apply_single_gate(&instruction.gate, instruction.qubits[0])?,
                2 => self.apply_two_qubit_gate(
                    &instruction.gate,
                    instruction.qubits[0],
                    instruction.qubits[1],
                )?,
                _ => {
                    // For 3+ qubit gates, use general expansion
                    let full_matrix = self.expand_multi_qubit_gate(&instruction.gate, &instruction.qubits)?;
                    self.state.apply_matrix(&full_matrix);
                    self.gate_history.push(format!("{}({:?})", instruction.gate.name, instruction.qubits));
                }
            }
        }
        Ok(())
    }

    /// Measure a single qubit, collapsing the state
    pub fn measure(&mut self, qubit: usize) -> Result<bool> {
        if qubit >= self.num_qubits() {
            return Err(CircuitError::InvalidQubitIndex(qubit, self.num_qubits()));
        }

        let mut rng = rand::thread_rng();
        let prob_one = self.probability_of_one(qubit);
        let result = rng.gen::<f64>() < prob_one;

        // Collapse the state
        self.collapse(qubit, result);
        self.classical_bits[qubit] = result;

        Ok(result)
    }

    /// Measure all qubits
    pub fn measure_all(&mut self) -> Vec<bool> {
        let mut rng = rand::thread_rng();
        let probs = self.state.probabilities();

        // Sample from the distribution
        let r: f64 = rng.gen();
        let mut cumulative = 0.0;
        let mut outcome = 0;

        for (i, &p) in probs.iter().enumerate() {
            cumulative += p;
            if r < cumulative {
                outcome = i;
                break;
            }
        }

        // Convert outcome to bit string
        let results: Vec<bool> = (0..self.num_qubits())
            .map(|i| (outcome >> i) & 1 == 1)
            .collect();

        // Collapse to the measured state
        self.state = StateVector::basis_state(self.num_qubits(), outcome).unwrap();
        self.classical_bits = results.clone();

        results
    }

    /// Sample measurements without collapsing (for statistics)
    pub fn sample(&self, shots: usize) -> Vec<Vec<bool>> {
        let mut rng = rand::thread_rng();
        let probs = self.state.probabilities();

        (0..shots).map(|_| {
            let r: f64 = rng.gen();
            let mut cumulative = 0.0;
            let mut outcome = 0;

            for (i, &p) in probs.iter().enumerate() {
                cumulative += p;
                if r < cumulative {
                    outcome = i;
                    break;
                }
            }

            (0..self.num_qubits())
                .map(|i| (outcome >> i) & 1 == 1)
                .collect()
        }).collect()
    }

    /// Get measurement statistics
    pub fn get_counts(&self, shots: usize) -> std::collections::HashMap<String, usize> {
        let samples = self.sample(shots);
        let mut counts = std::collections::HashMap::new();

        for sample in samples {
            let key: String = sample.iter()
                .rev()
                .map(|&b| if b { '1' } else { '0' })
                .collect();
            *counts.entry(key).or_insert(0) += 1;
        }

        counts
    }

    /// Probability of measuring |1⟩ on a specific qubit
    fn probability_of_one(&self, qubit: usize) -> f64 {
        let dim = self.state.dimension();
        let mut prob = 0.0;

        for i in 0..dim {
            if (i >> qubit) & 1 == 1 {
                prob += self.state.probability(i);
            }
        }

        prob
    }

    /// Collapse state after measurement
    fn collapse(&mut self, qubit: usize, result: bool) {
        let dim = self.state.dimension();
        let mut new_amplitudes = vec![ZERO; dim];
        let mut norm_sq = 0.0;

        for i in 0..dim {
            let bit = (i >> qubit) & 1 == 1;
            if bit == result {
                new_amplitudes[i] = self.state.amplitude(i);
                norm_sq += self.state.probability(i);
            }
        }

        // Normalize
        let norm = norm_sq.sqrt();
        if norm > 1e-15 {
            for amp in &mut new_amplitudes {
                *amp /= norm;
            }
        }

        self.state.amplitudes = new_amplitudes;
    }

    /// Expand single-qubit gate to full Hilbert space
    fn expand_single_gate(&self, gate: &Gate, qubit: usize) -> DMatrix<Complex> {
        let n = self.num_qubits();
        let dim = 1 << n;
        let gate_matrix = gate.matrix();

        let mut full_matrix = DMatrix::zeros(dim, dim);

        for i in 0..dim {
            for j in 0..dim {
                // Check if all qubits except 'qubit' are the same
                let mask = !(1 << qubit);
                if (i & mask) == (j & mask) {
                    let qi = (i >> qubit) & 1;
                    let qj = (j >> qubit) & 1;
                    full_matrix[(i, j)] = gate_matrix[(qi, qj)];
                }
            }
        }

        full_matrix
    }

    /// Expand two-qubit gate to full Hilbert space
    fn expand_two_qubit_gate(&self, gate: &Gate, qubit1: usize, qubit2: usize) -> DMatrix<Complex> {
        let n = self.num_qubits();
        let dim = 1 << n;
        let gate_matrix = gate.matrix();

        let mut full_matrix = DMatrix::zeros(dim, dim);

        for i in 0..dim {
            for j in 0..dim {
                // Check if all qubits except qubit1 and qubit2 are the same
                let mask = !((1 << qubit1) | (1 << qubit2));
                if (i & mask) == (j & mask) {
                    let q1_i = (i >> qubit1) & 1;
                    let q2_i = (i >> qubit2) & 1;
                    let q1_j = (j >> qubit1) & 1;
                    let q2_j = (j >> qubit2) & 1;

                    // Map to gate matrix indices (qubit1 is control, qubit2 is target)
                    let gi = q1_i * 2 + q2_i;
                    let gj = q1_j * 2 + q2_j;

                    full_matrix[(i, j)] = gate_matrix[(gi, gj)];
                }
            }
        }

        full_matrix
    }

    /// Expand multi-qubit gate
    fn expand_multi_qubit_gate(&self, gate: &Gate, qubits: &[usize]) -> Result<DMatrix<Complex>> {
        let n = self.num_qubits();
        let dim = 1 << n;
        let _gate_dim = 1 << qubits.len();
        let gate_matrix = gate.matrix();

        let mut full_matrix = DMatrix::zeros(dim, dim);

        // Build mask for qubits not involved in the gate
        let mut mask = (1 << n) - 1;
        for &q in qubits {
            mask &= !(1 << q);
        }

        for i in 0..dim {
            for j in 0..dim {
                // Check if non-gate qubits are the same
                if (i & mask) != (j & mask) {
                    continue;
                }

                // Extract gate qubit indices
                let mut gi = 0;
                let mut gj = 0;
                for (k, &q) in qubits.iter().enumerate() {
                    gi |= ((i >> q) & 1) << k;
                    gj |= ((j >> q) & 1) << k;
                }

                full_matrix[(i, j)] = gate_matrix[(gi, gj)];
            }
        }

        Ok(full_matrix)
    }

    /// Get state as string
    pub fn state_string(&self) -> String {
        let dim = self.state.dimension();
        let n = self.num_qubits();

        let terms: Vec<String> = (0..dim)
            .filter_map(|i| {
                let amp = self.state.amplitude(i);
                if amp.norm_sqr() > 1e-10 {
                    let basis: String = (0..n)
                        .rev()
                        .map(|b| if (i >> b) & 1 == 1 { '1' } else { '0' })
                        .collect();
                    Some(format!("({:.4})|{}⟩", amp, basis))
                } else {
                    None
                }
            })
            .collect();

        terms.join(" + ")
    }
}

impl std::fmt::Display for QuantumRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Gate;
    use approx::assert_relative_eq;

    #[test]
    fn test_initial_state() {
        let reg = QuantumRegister::new(2);
        assert_relative_eq!(reg.state.probability(0), 1.0, epsilon = 1e-10);
        assert_relative_eq!(reg.state.probability(1), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_hadamard() {
        let mut reg = QuantumRegister::new(1);
        reg.apply_single_gate(&Gate::h(), 0).unwrap();

        assert_relative_eq!(reg.state.probability(0), 0.5, epsilon = 1e-10);
        assert_relative_eq!(reg.state.probability(1), 0.5, epsilon = 1e-10);
    }

    #[test]
    fn test_bell_state() {
        let mut reg = QuantumRegister::new(2);
        reg.apply_single_gate(&Gate::h(), 0).unwrap();
        reg.apply_two_qubit_gate(&Gate::cnot(), 0, 1).unwrap();

        // Bell state: (|00⟩ + |11⟩)/√2
        assert_relative_eq!(reg.state.probability(0b00), 0.5, epsilon = 1e-10);
        assert_relative_eq!(reg.state.probability(0b01), 0.0, epsilon = 1e-10);
        assert_relative_eq!(reg.state.probability(0b10), 0.0, epsilon = 1e-10);
        assert_relative_eq!(reg.state.probability(0b11), 0.5, epsilon = 1e-10);
    }

    #[test]
    fn test_measurement_statistics() {
        let mut reg = QuantumRegister::new(1);
        reg.apply_single_gate(&Gate::h(), 0).unwrap();

        let counts = reg.get_counts(10000);
        let zeros = *counts.get("0").unwrap_or(&0) as f64;
        let ones = *counts.get("1").unwrap_or(&0) as f64;

        // Should be roughly 50-50
        assert!((zeros / 10000.0 - 0.5).abs() < 0.05);
        assert!((ones / 10000.0 - 0.5).abs() < 0.05);
    }
}
