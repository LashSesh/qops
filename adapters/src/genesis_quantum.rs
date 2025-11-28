//! Bridge between Genesis (S7) and Quantum (Cube-13) systems.

use qops_core::Signature5D;
use qops_quantum::QuantumState;
use num_complex::Complex64;

/// Bridge for converting between Genesis and Quantum representations
pub struct GenesisQuantumBridge;

impl GenesisQuantumBridge {
    /// Convert Genesis signature to Quantum state
    pub fn signature_to_state(sig: &Signature5D) -> QuantumState {
        let mut state = QuantumState::uniform();

        // Modulate amplitudes based on signature
        for i in 0..state.dimension() {
            let phase = std::f64::consts::TAU * (sig.psi * (i as f64) / 13.0);
            let amp = Complex64::new(phase.cos(), phase.sin());
            state.amplitudes[i] *= amp * Complex64::new(sig.rho, 0.0);
        }

        state.normalize();
        state
    }

    /// Convert Quantum probabilities to Genesis signature
    pub fn state_to_signature(state: &QuantumState) -> Signature5D {
        let probs = state.probabilities();

        // Extract signature from probability distribution
        let psi = probs.iter().cloned().fold(0.0, f64::max); // Max probability
        let rho = 1.0 - probs.iter().map(|&p| (p - 1.0/13.0).powi(2)).sum::<f64>().sqrt(); // Uniformity
        let omega = probs[0]; // Center node probability

        Signature5D::new(psi, rho, omega, 0.5, 0.3)
    }

    /// Map S7 node index to Cube-13 region
    pub fn s7_to_cube13(s7_index: usize) -> usize {
        // Map 5040 nodes to 13 regions
        s7_index % 13
    }

    /// Map Cube-13 node to S7 region
    pub fn cube13_to_s7_range(cube13_index: usize) -> std::ops::Range<usize> {
        let block_size = 5040 / 13;
        let start = cube13_index * block_size;
        let end = if cube13_index == 12 { 5040 } else { start + block_size };
        start..end
    }

    /// Create projection matrix from S7 to Cube-13
    pub fn projection_matrix() -> Vec<Vec<f64>> {
        let mut matrix = vec![vec![0.0; 5040]; 13];

        for i in 0..5040 {
            let target = Self::s7_to_cube13(i);
            matrix[target][i] = 1.0 / (5040.0 / 13.0); // Normalized
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_to_state() {
        let sig = Signature5D::new(0.8, 0.7, 0.6, 0.5, 0.3);
        let state = GenesisQuantumBridge::signature_to_state(&sig);

        assert!((state.norm() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_state_to_signature() {
        let state = QuantumState::uniform();
        let sig = GenesisQuantumBridge::state_to_signature(&state);

        assert!(sig.psi > 0.0);
        assert!(sig.rho > 0.0);
    }

    #[test]
    fn test_s7_mapping() {
        let mapped = GenesisQuantumBridge::s7_to_cube13(0);
        assert_eq!(mapped, 0);

        let mapped = GenesisQuantumBridge::s7_to_cube13(5039);
        assert!(mapped < 13);
    }
}
