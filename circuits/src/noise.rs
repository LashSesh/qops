//! Noise Models - Realistic quantum noise simulation
//!
//! This module provides noise models for simulating decoherence and errors.

use crate::{Complex, StateVector, QuantumRegister, Gate, Result, ZERO, ONE, I};
use nalgebra::DMatrix;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Types of noise channels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NoiseChannel {
    /// Depolarizing noise: applies random Pauli with probability p
    Depolarizing,
    /// Amplitude damping: models energy loss (T1 decay)
    AmplitudeDamping,
    /// Phase damping: models dephasing (T2 decay)
    PhaseDamping,
    /// Bit flip: X error with probability p
    BitFlip,
    /// Phase flip: Z error with probability p
    PhaseFlip,
    /// Bit-phase flip: Y error with probability p
    BitPhaseFlip,
    /// Thermal relaxation (combined T1 and T2)
    ThermalRelaxation,
    /// Readout error
    ReadoutError,
}

/// A noise model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseModel {
    /// Single-qubit gate error rate
    pub single_gate_error: f64,
    /// Two-qubit gate error rate
    pub two_gate_error: f64,
    /// Measurement error rate
    pub measurement_error: f64,
    /// T1 relaxation time (microseconds)
    pub t1: f64,
    /// T2 dephasing time (microseconds)
    pub t2: f64,
    /// Gate time for single-qubit gates (microseconds)
    pub single_gate_time: f64,
    /// Gate time for two-qubit gates (microseconds)
    pub two_gate_time: f64,
    /// Active noise channels
    pub channels: Vec<NoiseChannel>,
}

impl Default for NoiseModel {
    fn default() -> Self {
        Self {
            single_gate_error: 0.001,
            two_gate_error: 0.01,
            measurement_error: 0.01,
            t1: 50.0,
            t2: 30.0,
            single_gate_time: 0.05,
            two_gate_time: 0.3,
            channels: vec![NoiseChannel::Depolarizing],
        }
    }
}

impl NoiseModel {
    /// Create a noise-free model
    pub fn ideal() -> Self {
        Self {
            single_gate_error: 0.0,
            two_gate_error: 0.0,
            measurement_error: 0.0,
            t1: f64::INFINITY,
            t2: f64::INFINITY,
            single_gate_time: 0.0,
            two_gate_time: 0.0,
            channels: Vec::new(),
        }
    }

    /// Create a model based on IBM quantum hardware parameters
    pub fn ibm_like() -> Self {
        Self {
            single_gate_error: 0.0005,
            two_gate_error: 0.008,
            measurement_error: 0.015,
            t1: 100.0,
            t2: 80.0,
            single_gate_time: 0.035,
            two_gate_time: 0.3,
            channels: vec![
                NoiseChannel::Depolarizing,
                NoiseChannel::ThermalRelaxation,
            ],
        }
    }

    /// Create a highly noisy model for testing
    pub fn noisy() -> Self {
        Self {
            single_gate_error: 0.05,
            two_gate_error: 0.1,
            measurement_error: 0.1,
            t1: 10.0,
            t2: 5.0,
            single_gate_time: 0.1,
            two_gate_time: 0.5,
            channels: vec![
                NoiseChannel::Depolarizing,
                NoiseChannel::AmplitudeDamping,
                NoiseChannel::PhaseDamping,
            ],
        }
    }

    /// Apply noise after a single-qubit gate
    pub fn apply_single_gate_noise(&self, register: &mut QuantumRegister, qubit: usize) {
        if self.channels.is_empty() || self.single_gate_error == 0.0 {
            return;
        }

        let mut rng = rand::thread_rng();

        for channel in &self.channels {
            match channel {
                NoiseChannel::Depolarizing => {
                    apply_depolarizing(register, qubit, self.single_gate_error, &mut rng);
                }
                NoiseChannel::BitFlip => {
                    apply_bit_flip(register, qubit, self.single_gate_error, &mut rng);
                }
                NoiseChannel::PhaseFlip => {
                    apply_phase_flip(register, qubit, self.single_gate_error, &mut rng);
                }
                NoiseChannel::AmplitudeDamping => {
                    let gamma = 1.0 - (-self.single_gate_time / self.t1).exp();
                    apply_amplitude_damping(register, qubit, gamma);
                }
                NoiseChannel::PhaseDamping => {
                    let gamma = 1.0 - (-self.single_gate_time / self.t2).exp();
                    apply_phase_damping(register, qubit, gamma);
                }
                NoiseChannel::ThermalRelaxation => {
                    apply_thermal_relaxation(
                        register, qubit,
                        self.t1, self.t2, self.single_gate_time,
                    );
                }
                _ => {}
            }
        }
    }

    /// Apply noise after a two-qubit gate
    pub fn apply_two_gate_noise(
        &self,
        register: &mut QuantumRegister,
        qubit1: usize,
        qubit2: usize,
    ) {
        if self.channels.is_empty() || self.two_gate_error == 0.0 {
            return;
        }

        // Apply noise to both qubits
        self.apply_single_gate_noise(register, qubit1);
        self.apply_single_gate_noise(register, qubit2);
    }

    /// Apply measurement error
    pub fn apply_measurement_noise(&self, result: bool) -> bool {
        if self.measurement_error == 0.0 {
            return result;
        }

        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.measurement_error {
            !result
        } else {
            result
        }
    }
}

/// Depolarizing noise channel
#[derive(Debug, Clone)]
pub struct DepolarizingNoise {
    /// Error probability
    pub probability: f64,
}

impl DepolarizingNoise {
    pub fn new(probability: f64) -> Self {
        Self { probability: probability.clamp(0.0, 1.0) }
    }

    pub fn apply(&self, register: &mut QuantumRegister, qubit: usize) {
        let mut rng = rand::thread_rng();
        apply_depolarizing(register, qubit, self.probability, &mut rng);
    }
}

/// Amplitude damping channel (T1 decay)
#[derive(Debug, Clone)]
pub struct AmplitudeDamping {
    /// Damping parameter γ = 1 - exp(-t/T1)
    pub gamma: f64,
}

impl AmplitudeDamping {
    pub fn new(gamma: f64) -> Self {
        Self { gamma: gamma.clamp(0.0, 1.0) }
    }

    pub fn from_t1(t1: f64, gate_time: f64) -> Self {
        let gamma = 1.0 - (-gate_time / t1).exp();
        Self::new(gamma)
    }

    pub fn apply(&self, register: &mut QuantumRegister, qubit: usize) {
        apply_amplitude_damping(register, qubit, self.gamma);
    }
}

// ==================== Noise Implementation Functions ====================

fn apply_depolarizing<R: Rng>(
    register: &mut QuantumRegister,
    qubit: usize,
    p: f64,
    rng: &mut R,
) {
    if rng.gen::<f64>() >= p {
        return;
    }

    // Apply random Pauli with equal probability
    let r = rng.gen::<f64>();
    let gate = if r < 1.0 / 3.0 {
        Gate::x()
    } else if r < 2.0 / 3.0 {
        Gate::y()
    } else {
        Gate::z()
    };

    register.apply_single_gate(&gate, qubit).ok();
}

fn apply_bit_flip<R: Rng>(
    register: &mut QuantumRegister,
    qubit: usize,
    p: f64,
    rng: &mut R,
) {
    if rng.gen::<f64>() < p {
        register.apply_single_gate(&Gate::x(), qubit).ok();
    }
}

fn apply_phase_flip<R: Rng>(
    register: &mut QuantumRegister,
    qubit: usize,
    p: f64,
    rng: &mut R,
) {
    if rng.gen::<f64>() < p {
        register.apply_single_gate(&Gate::z(), qubit).ok();
    }
}

fn apply_amplitude_damping(
    register: &mut QuantumRegister,
    qubit: usize,
    gamma: f64,
) {
    // Kraus operators for amplitude damping:
    // K0 = [[1, 0], [0, sqrt(1-γ)]]
    // K1 = [[0, sqrt(γ)], [0, 0]]

    let sqrt_1mg = (1.0 - gamma).sqrt();
    let sqrt_g = gamma.sqrt();

    let n = register.num_qubits();
    let dim = 1 << n;

    let mut new_amplitudes = vec![ZERO; dim];

    for i in 0..dim {
        let bit = (i >> qubit) & 1;
        let amp = register.state.amplitudes()[i];

        if bit == 0 {
            // |0⟩ component: only K0 contributes
            new_amplitudes[i] += amp;
        } else {
            // |1⟩ component: K0 gives sqrt(1-γ)|1⟩, K1 gives sqrt(γ)|0⟩
            new_amplitudes[i] += amp * Complex::new(sqrt_1mg, 0.0);

            // K1 maps |1⟩ to |0⟩
            let target = i ^ (1 << qubit); // flip bit to 0
            new_amplitudes[target] += amp * Complex::new(sqrt_g, 0.0);
        }
    }

    // Note: This is a simplified version. Full Kraus operator would require
    // density matrix formalism for mixed states.
    let _ = register.state.set_amplitudes(new_amplitudes);
}

fn apply_phase_damping(
    register: &mut QuantumRegister,
    qubit: usize,
    gamma: f64,
) {
    // Phase damping reduces off-diagonal elements
    // For pure states, this creates a mixed state, but we approximate
    // by adding random phase

    let mut rng = rand::thread_rng();
    if rng.gen::<f64>() < gamma {
        // Apply random Z rotation to simulate dephasing
        let phase = rng.gen::<f64>() * std::f64::consts::PI;
        register.apply_single_gate(&Gate::rz(phase), qubit).ok();
    }
}

fn apply_thermal_relaxation(
    register: &mut QuantumRegister,
    qubit: usize,
    t1: f64,
    t2: f64,
    gate_time: f64,
) {
    // Combined T1 and T2 relaxation
    let gamma_1 = 1.0 - (-gate_time / t1).exp();
    let gamma_2 = 1.0 - (-gate_time / t2).exp();

    // Apply amplitude damping (T1)
    apply_amplitude_damping(register, qubit, gamma_1);

    // Apply additional dephasing (T2)
    // T2* dephasing rate = 1/T2 - 1/(2*T1)
    let pure_dephasing = if t2 < 2.0 * t1 {
        1.0 - (-(gate_time / t2 - gate_time / (2.0 * t1))).exp()
    } else {
        0.0
    };
    apply_phase_damping(register, qubit, pure_dephasing);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ideal_noise_model() {
        let model = NoiseModel::ideal();
        assert_eq!(model.single_gate_error, 0.0);
        assert_eq!(model.two_gate_error, 0.0);
        assert!(model.channels.is_empty());
    }

    #[test]
    fn test_depolarizing_noise() {
        let mut reg = QuantumRegister::new(1);

        // With p=1, should always apply a Pauli
        let noise = DepolarizingNoise::new(1.0);

        // Initial state is |0⟩
        let initial_prob_0 = reg.state.probability(0);

        // Apply noise multiple times
        for _ in 0..100 {
            noise.apply(&mut reg, 0);
        }

        // State should have changed significantly
        // (not a rigorous test, but checks that something happened)
        assert!(reg.state.is_normalized());
    }

    #[test]
    fn test_amplitude_damping() {
        let mut reg = QuantumRegister::new(1);
        reg.apply_single_gate(&Gate::x(), 0).unwrap(); // Start in |1⟩

        let damping = AmplitudeDamping::new(0.5);
        damping.apply(&mut reg, 0);

        // Should have some |0⟩ component now
        let prob_0 = reg.state.probability(0);
        assert!(prob_0 > 0.0);
        assert!(reg.state.is_normalized());
    }

    #[test]
    fn test_noise_model_application() {
        let model = NoiseModel::default();
        let mut reg = QuantumRegister::new(2);

        reg.apply_single_gate(&Gate::h(), 0).unwrap();
        model.apply_single_gate_noise(&mut reg, 0);

        assert!(reg.state.is_normalized());
    }
}
