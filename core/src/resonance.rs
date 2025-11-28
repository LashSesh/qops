//! Resonance computation and validation logic.
//!
//! The resonance system provides:
//! - Resonance score computation: R(v) = 0.4·ψ + 0.3·ρ + 0.3·ω
//! - Invariant validation: |Δ(ψ·ρ·ω) + χ·η| < ε
//! - Equilibrium checking: d/dt(ψ·ρ·ω) ≈ 0
//! - Feedback dynamics for signature evolution

use crate::signature::{Signature, Signature3D, Signature5D};
use serde::{Deserialize, Serialize};

/// Configuration for resonance computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceConfig {
    /// Epsilon for invariance checking
    pub epsilon: f64,
    /// Equilibrium tolerance
    pub equilibrium_tolerance: f64,
    /// Feedback strength
    pub feedback_strength: f64,
    /// Time delta for equilibrium checking
    pub time_delta: f64,
    /// Target resonance for feedback
    pub target_resonance: f64,
    /// Weights for resonance computation
    pub weights: ResonanceWeights,
}

impl Default for ResonanceConfig {
    fn default() -> Self {
        Self {
            epsilon: 0.001,
            equilibrium_tolerance: 1e-4,
            feedback_strength: 0.1,
            time_delta: 0.01,
            target_resonance: 0.85,
            weights: ResonanceWeights::default(),
        }
    }
}

/// Weights for resonance formula
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResonanceWeights {
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,
    pub chi: f64,
    pub eta: f64,
}

impl Default for ResonanceWeights {
    fn default() -> Self {
        Self {
            psi: 0.4,
            rho: 0.3,
            omega: 0.3,
            chi: 0.05,
            eta: -0.05,
        }
    }
}

/// Compute resonance score for a 3D signature
///
/// Formula: R(v) = 0.4·ψ + 0.3·ρ + 0.3·ω
pub fn resonance_3d(sig: &Signature3D) -> f64 {
    0.4 * sig.psi + 0.3 * sig.rho + 0.3 * sig.omega
}

/// Compute resonance score for a 5D signature
///
/// Formula: R(v) = 0.4·ψ + 0.3·ρ + 0.3·ω + 0.05·χ - 0.05·η
pub fn resonance_5d(sig: &Signature5D) -> f64 {
    let base = 0.4 * sig.psi + 0.3 * sig.rho + 0.3 * sig.omega;
    let topo_correction = 0.05 * sig.chi;
    let fluct_correction = -0.05 * sig.eta;
    (base + topo_correction + fluct_correction).clamp(0.0, 1.0)
}

/// Compute resonance score for any signature type
pub fn resonance(sig: &Signature) -> f64 {
    match sig {
        Signature::D3(s) => resonance_3d(s),
        Signature::D5(s) => resonance_5d(s),
    }
}

/// Compute resonance with custom weights
pub fn resonance_weighted(sig: &Signature5D, weights: &ResonanceWeights) -> f64 {
    let result = weights.psi * sig.psi
        + weights.rho * sig.rho
        + weights.omega * sig.omega
        + weights.chi * sig.chi
        + weights.eta * sig.eta;
    result.clamp(0.0, 1.0)
}

/// Validate resonance invariant condition
///
/// Rule: |Δ(ψ·ρ·ω) + χ·η| < ε
pub fn validate_invariant(v1: &Signature5D, v2: &Signature5D, epsilon: f64) -> bool {
    let product1 = v1.psi * v1.rho * v1.omega;
    let product2 = v2.psi * v2.rho * v2.omega;

    let delta_product = (product2 - product1).abs();
    let chi_eta_term = v2.chi * v2.eta;

    let invariant = delta_product + chi_eta_term;

    invariant < epsilon
}

/// Check resonance equilibrium condition
///
/// Rule: d/dt(ψ·ρ·ω) ≈ 0
pub fn check_equilibrium(history: &[Signature5D], config: &ResonanceConfig) -> bool {
    if history.len() < 2 {
        return false;
    }

    let n = history.len();
    let recent = &history[n - 1];
    let previous = &history[n - 2];

    let product_recent = recent.psi * recent.rho * recent.omega;
    let product_previous = previous.psi * previous.rho * previous.omega;

    let derivative = (product_recent - product_previous) / config.time_delta;

    derivative.abs() < config.equilibrium_tolerance
}

/// Compute resonance gradient for a signature
pub fn resonance_gradient(sig: &Signature5D, delta: f64) -> [f64; 5] {
    let base_res = resonance_5d(sig);
    let mut grad = [0.0; 5];

    // Gradient w.r.t. psi
    let mut perturbed = *sig;
    perturbed.psi = (perturbed.psi + delta).min(1.0);
    grad[0] = (resonance_5d(&perturbed) - base_res) / delta;

    // Gradient w.r.t. rho
    perturbed = *sig;
    perturbed.rho = (perturbed.rho + delta).min(1.0);
    grad[1] = (resonance_5d(&perturbed) - base_res) / delta;

    // Gradient w.r.t. omega
    perturbed = *sig;
    perturbed.omega = (perturbed.omega + delta).min(1.0);
    grad[2] = (resonance_5d(&perturbed) - base_res) / delta;

    // Gradient w.r.t. chi
    perturbed = *sig;
    perturbed.chi = (perturbed.chi + delta).min(1.0);
    grad[3] = (resonance_5d(&perturbed) - base_res) / delta;

    // Gradient w.r.t. eta
    perturbed = *sig;
    perturbed.eta = (perturbed.eta + delta).min(1.0);
    grad[4] = (resonance_5d(&perturbed) - base_res) / delta;

    grad
}

/// Apply feedback to evolve signature towards target resonance
pub fn apply_feedback(sig: &mut Signature5D, config: &ResonanceConfig) {
    let current_res = resonance_5d(sig);
    let error = config.target_resonance - current_res;
    let delta = config.feedback_strength * error;

    // Apply proportional feedback
    sig.psi += delta * 0.4;
    sig.rho += delta * 0.3;
    sig.omega += delta * 0.3;

    // Reduce fluctuation if resonance is low
    if current_res < 0.7 {
        sig.eta *= 0.95;
    }

    // Clamp to valid range
    sig.clamp();
}

/// Resonance dynamics tracker
#[derive(Debug, Clone)]
pub struct ResonanceDynamics {
    config: ResonanceConfig,
    history: Vec<Signature5D>,
    current_resonance: f64,
}

impl ResonanceDynamics {
    /// Create new dynamics tracker
    pub fn new(config: ResonanceConfig) -> Self {
        Self {
            config,
            history: Vec::new(),
            current_resonance: 0.0,
        }
    }

    /// Record a signature observation
    pub fn record(&mut self, sig: Signature5D) {
        self.current_resonance = resonance_5d(&sig);
        self.history.push(sig);
    }

    /// Evolve a signature one step
    pub fn evolve(&mut self, sig: &mut Signature5D) {
        // Apply gradient-based evolution
        let grad = resonance_gradient(sig, 0.01);

        sig.psi += self.config.feedback_strength * grad[0];
        sig.rho += self.config.feedback_strength * grad[1];
        sig.omega += self.config.feedback_strength * grad[2];
        sig.chi += self.config.feedback_strength * grad[3];
        sig.eta += self.config.feedback_strength * grad[4];

        sig.clamp();
        self.record(*sig);
    }

    /// Check if system is at equilibrium
    pub fn is_at_equilibrium(&self) -> bool {
        check_equilibrium(&self.history, &self.config)
    }

    /// Get current resonance
    pub fn current_resonance(&self) -> f64 {
        self.current_resonance
    }

    /// Get history
    pub fn history(&self) -> &[Signature5D] {
        &self.history
    }

    /// Get recent trend (positive = improving, negative = degrading)
    pub fn trend(&self) -> f64 {
        if self.history.len() < 5 {
            return 0.0;
        }

        let n = self.history.len();
        let recent: f64 = self.history[n - 3..].iter().map(resonance_5d).sum::<f64>() / 3.0;
        let older: f64 = self.history[n - 5..n - 2]
            .iter()
            .map(resonance_5d)
            .sum::<f64>()
            / 3.0;

        recent - older
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_resonance_3d() {
        let sig = Signature3D::new(1.0, 1.0, 1.0);
        assert_relative_eq!(resonance_3d(&sig), 1.0);

        let sig2 = Signature3D::new(0.5, 0.5, 0.5);
        assert_relative_eq!(resonance_3d(&sig2), 0.5);
    }

    #[test]
    fn test_resonance_5d() {
        let sig = Signature5D::new(0.9, 0.8, 0.7, 0.6, 0.5);
        let res = resonance_5d(&sig);
        assert!(res > 0.7 && res <= 1.0);
    }

    #[test]
    fn test_invariant_validation() {
        let v1 = Signature5D::new(0.5, 0.5, 0.5, 0.1, 0.1);
        let v2 = Signature5D::new(0.5, 0.5, 0.51, 0.1, 0.1);

        assert!(validate_invariant(&v1, &v2, 0.1));
    }

    #[test]
    fn test_feedback() {
        let config = ResonanceConfig::default();
        let mut sig = Signature5D::new(0.3, 0.3, 0.3, 0.3, 0.3);
        let initial_res = resonance_5d(&sig);

        apply_feedback(&mut sig, &config);

        let final_res = resonance_5d(&sig);
        assert!(final_res > initial_res);
    }

    #[test]
    fn test_gradient() {
        let sig = Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5);
        let grad = resonance_gradient(&sig, 0.01);

        // Psi has highest weight, so gradient should be highest
        assert!(grad[0] > grad[1]);
        // Eta has negative weight, so gradient should be negative
        assert!(grad[4] < 0.0);
    }

    #[test]
    fn test_dynamics() {
        let config = ResonanceConfig::default();
        let mut dynamics = ResonanceDynamics::new(config);
        let mut sig = Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5);

        for _ in 0..10 {
            dynamics.evolve(&mut sig);
        }

        assert!(dynamics.current_resonance() > 0.5);
    }
}
