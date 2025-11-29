//! Resonance model for evaluating state coherence.
//!
//! Per Section 4 of the specification, the resonance function R: H^n → R
//! captures how well a state aligns with an internal notion of coherence,
//! consistency, or "ideal form".
//!
//! The simple core example is R(v) = ψ·ρ·ω, but the module supports
//! configurable functional forms.

use crate::error::{KernelError, Result};
use crate::state::{CoreSignature, State};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Resonance function trait.
///
/// Any resonance model must implement this trait.
pub trait ResonanceFunction: Send + Sync {
    /// Compute resonance score for a state
    fn compute(&self, state: &State) -> f64;

    /// Compute resonance for core signature directly
    fn compute_signature(&self, sig: &CoreSignature) -> f64 {
        self.compute(&State::Core(*sig))
    }

    /// Get the resonance function name
    fn name(&self) -> &str;

    /// Check if a state meets the threshold
    fn meets_threshold(&self, state: &State, threshold: f64) -> bool {
        self.compute(state) >= threshold
    }
}

/// Simple multiplicative resonance: R(v) = ψ·ρ·ω
///
/// This is the default resonance model as specified in Section 4.1.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimpleResonance;

impl ResonanceFunction for SimpleResonance {
    fn compute(&self, state: &State) -> f64 {
        let sig = state.to_core();
        sig.psi * sig.rho * sig.omega
    }

    fn name(&self) -> &str {
        "simple"
    }
}

/// Weighted resonance: R(v) = w_ψ·ψ + w_ρ·ρ + w_ω·ω + w_χ·χ + w_η·η
///
/// Uses configurable weights for each dimension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedResonance {
    /// Weight for psi
    pub w_psi: f64,
    /// Weight for rho
    pub w_rho: f64,
    /// Weight for omega
    pub w_omega: f64,
    /// Weight for chi
    pub w_chi: f64,
    /// Weight for eta
    pub w_eta: f64,
}

impl Default for WeightedResonance {
    fn default() -> Self {
        // Default weights from QOPS core
        Self {
            w_psi: 0.4,
            w_rho: 0.3,
            w_omega: 0.3,
            w_chi: 0.05,
            w_eta: -0.05,
        }
    }
}

impl WeightedResonance {
    /// Create with custom weights
    pub fn new(w_psi: f64, w_rho: f64, w_omega: f64, w_chi: f64, w_eta: f64) -> Self {
        Self {
            w_psi,
            w_rho,
            w_omega,
            w_chi,
            w_eta,
        }
    }

    /// Create balanced weights
    pub fn balanced() -> Self {
        Self::new(0.2, 0.2, 0.2, 0.2, 0.2)
    }

    /// Create quality-focused weights
    pub fn quality_focused() -> Self {
        Self::new(0.5, 0.2, 0.2, 0.1, 0.0)
    }

    /// Create stability-focused weights
    pub fn stability_focused() -> Self {
        Self::new(0.2, 0.5, 0.2, 0.1, 0.0)
    }
}

impl ResonanceFunction for WeightedResonance {
    fn compute(&self, state: &State) -> f64 {
        let sig = state.to_core();
        let result = self.w_psi * sig.psi
            + self.w_rho * sig.rho
            + self.w_omega * sig.omega
            + self.w_chi * sig.chi
            + self.w_eta * sig.eta;
        result.clamp(0.0, 1.0)
    }

    fn name(&self) -> &str {
        "weighted"
    }
}

/// Extended multiplicative resonance with topological correction
///
/// R(v) = ψ·ρ·ω·(1 + α·χ - β·η)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedResonance {
    /// Alpha coefficient for chi correction
    pub alpha: f64,
    /// Beta coefficient for eta correction
    pub beta: f64,
}

impl Default for ExtendedResonance {
    fn default() -> Self {
        Self {
            alpha: 0.1,
            beta: 0.05,
        }
    }
}

impl ResonanceFunction for ExtendedResonance {
    fn compute(&self, state: &State) -> f64 {
        let sig = state.to_core();
        let base = sig.psi * sig.rho * sig.omega;
        let correction = 1.0 + self.alpha * sig.chi - self.beta * sig.eta;
        (base * correction).clamp(0.0, 1.0)
    }

    fn name(&self) -> &str {
        "extended"
    }
}

/// Geometric mean resonance
///
/// R(v) = (ψ·ρ·ω·χ·(1-η))^(1/5)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GeometricResonance;

impl ResonanceFunction for GeometricResonance {
    fn compute(&self, state: &State) -> f64 {
        let sig = state.to_core();
        let product = sig.psi * sig.rho * sig.omega * sig.chi * (1.0 - sig.eta).max(0.01);
        product.powf(0.2).clamp(0.0, 1.0)
    }

    fn name(&self) -> &str {
        "geometric"
    }
}

/// Harmonic mean resonance
///
/// R(v) = 5 / (1/ψ + 1/ρ + 1/ω + 1/χ + 1/(1-η))
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HarmonicResonance;

impl ResonanceFunction for HarmonicResonance {
    fn compute(&self, state: &State) -> f64 {
        let sig = state.to_core();
        let eps = 0.01; // Prevent division by zero

        let inv_sum = 1.0 / (sig.psi + eps)
            + 1.0 / (sig.rho + eps)
            + 1.0 / (sig.omega + eps)
            + 1.0 / (sig.chi + eps)
            + 1.0 / ((1.0 - sig.eta).max(eps));

        (5.0 / inv_sum).clamp(0.0, 1.0)
    }

    fn name(&self) -> &str {
        "harmonic"
    }
}

/// Configurable resonance model supporting multiple function types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResonanceModel {
    /// Simple multiplicative
    Simple(SimpleResonance),
    /// Weighted linear combination
    Weighted(WeightedResonance),
    /// Extended with topological correction
    Extended(ExtendedResonance),
    /// Geometric mean
    Geometric(GeometricResonance),
    /// Harmonic mean
    Harmonic(HarmonicResonance),
    /// Custom function (by name, resolved at runtime)
    Custom(String),
}

impl Default for ResonanceModel {
    fn default() -> Self {
        ResonanceModel::Simple(SimpleResonance)
    }
}

impl ResonanceModel {
    /// Compute resonance using the model
    pub fn compute(&self, state: &State) -> f64 {
        match self {
            ResonanceModel::Simple(r) => r.compute(state),
            ResonanceModel::Weighted(r) => r.compute(state),
            ResonanceModel::Extended(r) => r.compute(state),
            ResonanceModel::Geometric(r) => r.compute(state),
            ResonanceModel::Harmonic(r) => r.compute(state),
            ResonanceModel::Custom(_) => {
                // Default to simple for unknown custom functions
                SimpleResonance.compute(state)
            }
        }
    }

    /// Get model name
    pub fn name(&self) -> &str {
        match self {
            ResonanceModel::Simple(r) => r.name(),
            ResonanceModel::Weighted(r) => r.name(),
            ResonanceModel::Extended(r) => r.name(),
            ResonanceModel::Geometric(r) => r.name(),
            ResonanceModel::Harmonic(r) => r.name(),
            ResonanceModel::Custom(n) => n,
        }
    }
}

/// Resonance threshold configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResonanceThreshold {
    /// Minimum threshold for acceptance
    pub accept: f64,
    /// Threshold for high-quality candidates
    pub high_quality: f64,
    /// Threshold for Mandorla-level resonance
    pub mandorla: f64,
}

impl Default for ResonanceThreshold {
    fn default() -> Self {
        Self {
            accept: 0.5,
            high_quality: 0.75,
            mandorla: 0.85,
        }
    }
}

impl ResonanceThreshold {
    /// Create custom thresholds
    pub fn new(accept: f64, high_quality: f64, mandorla: f64) -> Self {
        Self {
            accept,
            high_quality,
            mandorla,
        }
    }

    /// Strict thresholds
    pub fn strict() -> Self {
        Self::new(0.7, 0.85, 0.95)
    }

    /// Lenient thresholds
    pub fn lenient() -> Self {
        Self::new(0.3, 0.5, 0.7)
    }

    /// Check threshold level for a resonance value
    pub fn level(&self, resonance: f64) -> ResonanceLevel {
        if resonance >= self.mandorla {
            ResonanceLevel::Mandorla
        } else if resonance >= self.high_quality {
            ResonanceLevel::HighQuality
        } else if resonance >= self.accept {
            ResonanceLevel::Acceptable
        } else {
            ResonanceLevel::BelowThreshold
        }
    }
}

/// Resonance level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResonanceLevel {
    /// Below minimum threshold
    BelowThreshold,
    /// Meets minimum threshold
    Acceptable,
    /// High quality
    HighQuality,
    /// Mandorla-level (ideal form)
    Mandorla,
}

impl ResonanceLevel {
    /// Get level name
    pub fn name(&self) -> &str {
        match self {
            ResonanceLevel::BelowThreshold => "below_threshold",
            ResonanceLevel::Acceptable => "acceptable",
            ResonanceLevel::HighQuality => "high_quality",
            ResonanceLevel::Mandorla => "mandorla",
        }
    }

    /// Check if level is acceptable or higher
    pub fn is_acceptable(&self) -> bool {
        !matches!(self, ResonanceLevel::BelowThreshold)
    }
}

/// Resonance gradient computation
#[derive(Debug, Clone, Default)]
pub struct ResonanceGradient {
    /// Gradient components
    pub grad: [f64; 5],
}

impl ResonanceGradient {
    /// Compute numerical gradient of resonance function at a point
    pub fn compute(model: &ResonanceModel, state: &State, delta: f64) -> Self {
        let base_res = model.compute(state);
        let sig = state.to_core();
        let mut grad = [0.0; 5];

        // Gradient for each dimension
        let perturbations = [
            CoreSignature::new(sig.psi + delta, sig.rho, sig.omega, sig.chi, sig.eta),
            CoreSignature::new(sig.psi, sig.rho + delta, sig.omega, sig.chi, sig.eta),
            CoreSignature::new(sig.psi, sig.rho, sig.omega + delta, sig.chi, sig.eta),
            CoreSignature::new(sig.psi, sig.rho, sig.omega, sig.chi + delta, sig.eta),
            CoreSignature::new(sig.psi, sig.rho, sig.omega, sig.chi, sig.eta + delta),
        ];

        for (i, perturbed) in perturbations.iter().enumerate() {
            let perturbed_state = State::Core(*perturbed);
            grad[i] = (model.compute(&perturbed_state) - base_res) / delta;
        }

        Self { grad }
    }

    /// Get gradient magnitude
    pub fn magnitude(&self) -> f64 {
        self.grad.iter().map(|g| g * g).sum::<f64>().sqrt()
    }

    /// Get normalized gradient direction
    pub fn direction(&self) -> [f64; 5] {
        let mag = self.magnitude();
        if mag < 1e-10 {
            return [0.0; 5];
        }
        let mut dir = [0.0; 5];
        for (i, &g) in self.grad.iter().enumerate() {
            dir[i] = g / mag;
        }
        dir
    }
}

/// Resonance evaluator combining model and thresholds
#[derive(Debug, Clone)]
pub struct ResonanceEvaluator {
    /// Resonance model
    pub model: ResonanceModel,
    /// Thresholds
    pub thresholds: ResonanceThreshold,
    /// History of evaluations
    history: Vec<(State, f64)>,
    /// Maximum history size
    max_history: usize,
}

impl Default for ResonanceEvaluator {
    fn default() -> Self {
        Self {
            model: ResonanceModel::default(),
            thresholds: ResonanceThreshold::default(),
            history: Vec::new(),
            max_history: 1000,
        }
    }
}

impl ResonanceEvaluator {
    /// Create with model and thresholds
    pub fn new(model: ResonanceModel, thresholds: ResonanceThreshold) -> Self {
        Self {
            model,
            thresholds,
            history: Vec::new(),
            max_history: 1000,
        }
    }

    /// Evaluate a state
    pub fn evaluate(&mut self, state: &State) -> f64 {
        let res = self.model.compute(state);
        self.record(state.clone(), res);
        res
    }

    /// Evaluate without recording
    pub fn evaluate_without_record(&self, state: &State) -> f64 {
        self.model.compute(state)
    }

    /// Get level for a state
    pub fn level(&self, state: &State) -> ResonanceLevel {
        let res = self.model.compute(state);
        self.thresholds.level(res)
    }

    /// Check if state is acceptable
    pub fn is_acceptable(&self, state: &State) -> bool {
        self.model.compute(state) >= self.thresholds.accept
    }

    /// Check if state is Mandorla-level
    pub fn is_mandorla(&self, state: &State) -> bool {
        self.model.compute(state) >= self.thresholds.mandorla
    }

    /// Compute gradient at state
    pub fn gradient(&self, state: &State) -> ResonanceGradient {
        ResonanceGradient::compute(&self.model, state, 0.01)
    }

    /// Record evaluation
    fn record(&mut self, state: State, res: f64) {
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        self.history.push((state, res));
    }

    /// Get average resonance from history
    pub fn average_resonance(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        self.history.iter().map(|(_, r)| r).sum::<f64>() / self.history.len() as f64
    }

    /// Get best resonance from history
    pub fn best_resonance(&self) -> Option<f64> {
        self.history.iter().map(|(_, r)| *r).max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_resonance() {
        let model = SimpleResonance;
        let sig = CoreSignature::new(0.8, 0.8, 0.8, 0.5, 0.5);
        let state = State::Core(sig);

        let res = model.compute(&state);
        assert!((res - 0.512).abs() < 0.001);
    }

    #[test]
    fn test_weighted_resonance() {
        let model = WeightedResonance::default();
        let sig = CoreSignature::new(1.0, 1.0, 1.0, 0.5, 0.5);
        let state = State::Core(sig);

        let res = model.compute(&state);
        assert!(res > 0.9);
    }

    #[test]
    fn test_resonance_threshold() {
        let threshold = ResonanceThreshold::default();

        assert_eq!(threshold.level(0.3), ResonanceLevel::BelowThreshold);
        assert_eq!(threshold.level(0.6), ResonanceLevel::Acceptable);
        assert_eq!(threshold.level(0.8), ResonanceLevel::HighQuality);
        assert_eq!(threshold.level(0.9), ResonanceLevel::Mandorla);
    }

    #[test]
    fn test_resonance_gradient() {
        let model = ResonanceModel::Simple(SimpleResonance);
        let sig = CoreSignature::center();
        let state = State::Core(sig);

        let grad = ResonanceGradient::compute(&model, &state, 0.01);

        // For simple resonance R = psi*rho*omega, gradient should be positive for psi, rho, omega
        assert!(grad.grad[0] > 0.0); // d/d(psi)
        assert!(grad.grad[1] > 0.0); // d/d(rho)
        assert!(grad.grad[2] > 0.0); // d/d(omega)
    }

    #[test]
    fn test_resonance_evaluator() {
        let mut evaluator = ResonanceEvaluator::default();

        let sig = CoreSignature::new(0.9, 0.8, 0.7, 0.5, 0.3);
        let state = State::Core(sig);

        let res = evaluator.evaluate(&state);
        assert!(res > 0.4);
        assert!(evaluator.is_acceptable(&state));
    }
}
