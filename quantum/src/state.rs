//! Quantum state representation.

use nalgebra::DVector;
use num_complex::Complex64;
use crate::METATRON_DIMENSION;

/// Quantum state vector
#[derive(Debug, Clone)]
pub struct QuantumState {
    /// Complex amplitudes
    pub amplitudes: DVector<Complex64>,
}

impl QuantumState {
    /// Create a new quantum state with given amplitudes
    pub fn new(amplitudes: DVector<Complex64>) -> Self {
        Self { amplitudes }
    }

    /// Create a zero state
    pub fn zeros(dim: usize) -> Self {
        Self {
            amplitudes: DVector::zeros(dim),
        }
    }

    /// Create a basis state |i⟩
    pub fn basis_state(index: usize) -> qops_core::Result<Self> {
        if index >= METATRON_DIMENSION {
            return Err(qops_core::QopsError::quantum("Invalid basis state index"));
        }

        let mut amplitudes = DVector::zeros(METATRON_DIMENSION);
        amplitudes[index] = Complex64::new(1.0, 0.0);

        Ok(Self { amplitudes })
    }

    /// Create a uniform superposition state
    pub fn uniform() -> Self {
        let amp = Complex64::new(1.0 / (METATRON_DIMENSION as f64).sqrt(), 0.0);
        let amplitudes = DVector::from_element(METATRON_DIMENSION, amp);
        Self { amplitudes }
    }

    /// Get the dimension
    pub fn dimension(&self) -> usize {
        self.amplitudes.len()
    }

    /// Compute the norm
    pub fn norm(&self) -> f64 {
        self.amplitudes
            .iter()
            .map(|a| a.norm_sqr())
            .sum::<f64>()
            .sqrt()
    }

    /// Normalize the state
    pub fn normalize(&mut self) {
        let n = self.norm();
        if n > 1e-10 {
            self.amplitudes /= Complex64::new(n, 0.0);
        }
    }

    /// Get probability distribution
    pub fn probabilities(&self) -> Vec<f64> {
        self.amplitudes.iter().map(|a| a.norm_sqr()).collect()
    }

    /// Inner product with another state
    pub fn inner_product(&self, other: &Self) -> Complex64 {
        self.amplitudes
            .iter()
            .zip(other.amplitudes.iter())
            .map(|(a, b)| a.conj() * b)
            .sum()
    }

    /// Fidelity with another state
    pub fn fidelity(&self, other: &Self) -> f64 {
        self.inner_product(other).norm_sqr()
    }

    /// Expectation value of a Hermitian operator (represented as real diagonal)
    pub fn expectation(&self, diagonal: &[f64]) -> f64 {
        self.amplitudes
            .iter()
            .zip(diagonal.iter())
            .map(|(a, &d)| a.norm_sqr() * d)
            .sum()
    }
}

impl Default for QuantumState {
    fn default() -> Self {
        Self::basis_state(0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basis_state() {
        let state = QuantumState::basis_state(0).unwrap();
        assert!((state.norm() - 1.0).abs() < 1e-10);

        let probs = state.probabilities();
        assert!((probs[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_uniform_state() {
        let state = QuantumState::uniform();
        assert!((state.norm() - 1.0).abs() < 1e-10);

        let probs = state.probabilities();
        let expected = 1.0 / METATRON_DIMENSION as f64;
        for p in probs {
            assert!((p - expected).abs() < 1e-10);
        }
    }

    #[test]
    fn test_inner_product() {
        let state1 = QuantumState::basis_state(0).unwrap();
        let state2 = QuantumState::basis_state(0).unwrap();
        let state3 = QuantumState::basis_state(1).unwrap();

        assert!((state1.inner_product(&state2).norm() - 1.0).abs() < 1e-10);
        assert!(state1.inner_product(&state3).norm() < 1e-10);
    }
}
