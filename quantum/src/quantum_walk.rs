//! Continuous-time quantum walks.

use crate::hamiltonian::MetatronHamiltonian;
use crate::state::QuantumState;

/// Continuous-time quantum walk
pub struct ContinuousQuantumWalk {
    hamiltonian: MetatronHamiltonian,
}

impl ContinuousQuantumWalk {
    /// Create new quantum walk
    pub fn new(hamiltonian: MetatronHamiltonian) -> Self {
        Self { hamiltonian }
    }

    /// Evolve state for time t: |ψ(t)⟩ = exp(-iHt)|ψ(0)⟩
    pub fn evolve(&self, initial: &QuantumState, t: f64) -> QuantumState {
        // Use propagator
        let propagator = self.hamiltonian.propagator(t);
        let new_amplitudes = &propagator * &initial.amplitudes;

        let mut state = QuantumState::new(new_amplitudes);
        state.normalize();
        state
    }

    /// Evolve with multiple time steps
    pub fn evolve_trajectory(&self, initial: &QuantumState, times: &[f64]) -> Vec<QuantumState> {
        let mut trajectory = Vec::with_capacity(times.len());
        let mut current = initial.clone();

        let mut prev_t = 0.0;
        for &t in times {
            let dt = t - prev_t;
            current = self.evolve(&current, dt);
            trajectory.push(current.clone());
            prev_t = t;
        }

        trajectory
    }

    /// Compute hitting time to target node
    pub fn hitting_time(&self, source: usize, target: usize, max_time: f64, dt: f64) -> Option<f64> {
        let initial = QuantumState::basis_state(source).ok()?;
        let mut t = 0.0;

        while t < max_time {
            let state = self.evolve(&initial, t);
            let prob = state.probabilities()[target];

            if prob > 0.5 {
                return Some(t);
            }

            t += dt;
        }

        None
    }

    /// Compute mixing time (when distribution is close to uniform)
    pub fn mixing_time(&self, source: usize, max_time: f64, dt: f64, epsilon: f64) -> Option<f64> {
        let initial = QuantumState::basis_state(source).ok()?;
        let n = initial.dimension();
        let uniform = 1.0 / n as f64;
        let mut t = 0.0;

        while t < max_time {
            let state = self.evolve(&initial, t);
            let probs = state.probabilities();

            // Check total variation distance
            let tvd: f64 = probs.iter().map(|&p| (p - uniform).abs()).sum::<f64>() / 2.0;

            if tvd < epsilon {
                return Some(t);
            }

            t += dt;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MetatronGraph;

    #[test]
    fn test_quantum_walk_evolution() {
        let graph = MetatronGraph::new();
        let h = MetatronHamiltonian::from_graph(&graph);
        let qw = ContinuousQuantumWalk::new(h);

        let initial = QuantumState::basis_state(0).unwrap();
        let evolved = qw.evolve(&initial, 1.0);

        // Norm should be preserved
        assert!((evolved.norm() - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_quantum_walk_spreading() {
        let graph = MetatronGraph::new();
        let h = MetatronHamiltonian::from_graph(&graph);
        let qw = ContinuousQuantumWalk::new(h);

        let initial = QuantumState::basis_state(0).unwrap();
        let evolved = qw.evolve(&initial, 2.0);

        // Probability should spread from center
        let probs = evolved.probabilities();
        assert!(probs[0] < 0.9); // Some spreading
    }
}
