//! Calibration operators for configuration evolution.
//!
//! This module implements the Double-Kick operator T = Φ_V ∘ Φ_U that moves
//! configurations towards fixpoint attractors through:
//! - Φ_U: Update kick (improves quality ψ)
//! - Φ_V: Stabilization kick (improves stability ρ and efficiency ω)

use crate::field::MandorlaField;
use crate::signature::Signature3D;
use serde::{Deserialize, Serialize};

/// Configuration for a generative system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /// Algorithm/ansatz type
    pub algorithm: String,
    /// Depth/complexity parameter
    pub depth: usize,
    /// Learning rate
    pub learning_rate: f64,
    /// Maximum iterations
    pub max_iterations: usize,
    /// Number of random restarts
    pub num_restarts: usize,
    /// Optimizer type
    pub optimizer: String,
    /// Custom parameters
    pub params: std::collections::HashMap<String, f64>,
}

impl Configuration {
    /// Create a new configuration
    pub fn new(algorithm: &str) -> Self {
        Self {
            algorithm: algorithm.to_string(),
            depth: 2,
            learning_rate: 0.01,
            max_iterations: 100,
            num_restarts: 3,
            optimizer: "Adam".to_string(),
            params: std::collections::HashMap::new(),
        }
    }

    /// Compute distance to another configuration
    pub fn distance(&self, other: &Self) -> f64 {
        let depth_diff = (self.depth as f64 - other.depth as f64).abs();
        let lr_diff = (self.learning_rate - other.learning_rate).abs() * 100.0;
        let iter_diff = (self.max_iterations as f64 - other.max_iterations as f64).abs() / 100.0;
        let restart_diff = (self.num_restarts as f64 - other.num_restarts as f64).abs();

        (depth_diff.powi(2) + lr_diff.powi(2) + iter_diff.powi(2) + restart_diff.powi(2)).sqrt()
    }

    /// Convert to dictionary
    pub fn to_dict(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        map.insert("algorithm".to_string(), self.algorithm.clone());
        map.insert("depth".to_string(), self.depth.to_string());
        map.insert("learning_rate".to_string(), self.learning_rate.to_string());
        map.insert("max_iterations".to_string(), self.max_iterations.to_string());
        map.insert("num_restarts".to_string(), self.num_restarts.to_string());
        map.insert("optimizer".to_string(), self.optimizer.clone());
        map
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new("default")
    }
}

/// Trait for configuration evolution operators
pub trait CalibrationOperator: Clone + Send + Sync {
    /// Apply the operator to a configuration
    fn apply(
        &self,
        config: &Configuration,
        performance: &Signature3D,
        field: Option<&MandorlaField>,
    ) -> Configuration;
}

/// Update kick Φ_U that improves quality (ψ)
///
/// Φ_U(c) = c + η_U ∇_c ψ(c)
#[derive(Debug, Clone)]
pub struct UpdateKick {
    /// Step size η_U
    pub step_size: f64,
}

impl UpdateKick {
    /// Create a new update kick operator
    pub fn new(step_size: f64) -> Self {
        Self { step_size }
    }
}

impl Default for UpdateKick {
    fn default() -> Self {
        Self::new(0.3)
    }
}

impl CalibrationOperator for UpdateKick {
    fn apply(
        &self,
        config: &Configuration,
        performance: &Signature3D,
        _field: Option<&MandorlaField>,
    ) -> Configuration {
        let mut new_config = config.clone();

        // Heuristics for quality improvement
        // 1. Metatron/specialized ansatz with optimal depth is good
        if config.depth < 3 && performance.psi < 0.9 {
            new_config.depth = config.depth + 1;
        }

        // 2. Adam optimizer is generally reliable
        if config.optimizer != "Adam" && performance.psi < 0.8 {
            new_config.optimizer = "Adam".to_string();
        }

        // 3. Adjust learning rate
        if performance.psi < 0.7 {
            new_config.learning_rate = (config.learning_rate * 1.1).min(0.1);
        }

        // 4. More iterations if quality is low
        if performance.psi < 0.6 {
            new_config.max_iterations = (config.max_iterations * 12 / 10).min(500);
        }

        new_config
    }
}

/// Stabilization kick Φ_V that improves stability (ρ) and efficiency (ω)
///
/// Φ_V(c) = c + η_V R(c)
#[derive(Debug, Clone)]
pub struct StabilizationKick {
    /// Step size η_V
    pub step_size: f64,
}

impl StabilizationKick {
    /// Create a new stabilization kick operator
    pub fn new(step_size: f64) -> Self {
        Self { step_size }
    }
}

impl Default for StabilizationKick {
    fn default() -> Self {
        Self::new(0.2)
    }
}

impl CalibrationOperator for StabilizationKick {
    fn apply(
        &self,
        config: &Configuration,
        performance: &Signature3D,
        _field: Option<&MandorlaField>,
    ) -> Configuration {
        let mut new_config = config.clone();

        // Heuristics for stability improvement
        // 1. Multiple random starts increase stability
        if performance.rho < 0.8 && config.num_restarts < 5 {
            new_config.num_restarts = config.num_restarts + 1;
        }

        // 2. Lower depth can be more stable
        if performance.rho < 0.7 && config.depth > 1 {
            new_config.depth = config.depth - 1;
        }

        // Heuristics for efficiency improvement
        // 1. Lower depth is more efficient
        if performance.omega < 0.7 && config.depth > 2 {
            new_config.depth = config.depth - 1;
        }

        // 2. Reduce iterations if already converging
        if performance.omega < 0.6 && config.max_iterations > 50 {
            new_config.max_iterations = (config.max_iterations * 9 / 10).max(50);
        }

        new_config
    }
}

/// Double-Kick operator T = Φ_V ∘ Φ_U
///
/// Applies update kick followed by stabilization kick to create
/// locally contractive dynamics towards fixpoint attractors.
#[derive(Debug, Clone)]
pub struct DoubleKickOperator {
    /// Update kick operator
    pub update_kick: UpdateKick,
    /// Stabilization kick operator
    pub stabilization_kick: StabilizationKick,
}

impl DoubleKickOperator {
    /// Create a new double-kick operator
    pub fn new(update_step: f64, stabilization_step: f64) -> Self {
        Self {
            update_kick: UpdateKick::new(update_step),
            stabilization_kick: StabilizationKick::new(stabilization_step),
        }
    }

    /// Apply T = Φ_V ∘ Φ_U to configuration
    pub fn apply(
        &self,
        config: &Configuration,
        performance: &Signature3D,
        field: Option<&MandorlaField>,
    ) -> Configuration {
        // First: update kick (improve quality)
        let intermediate = self.update_kick.apply(config, performance, field);

        // Second: stabilization kick (improve stability and efficiency)
        self.stabilization_kick.apply(&intermediate, performance, field)
    }

    /// Iterate T multiple times towards fixpoint
    pub fn iterate(
        &self,
        config: &Configuration,
        performance: &Signature3D,
        field: Option<&MandorlaField>,
        num_iterations: usize,
    ) -> (Configuration, f64) {
        let mut current = config.clone();
        let mut distances = Vec::new();

        for _ in 0..num_iterations {
            let next = self.apply(&current, performance, field);
            let dist = current.distance(&next);
            distances.push(dist);

            // Check for fixpoint convergence
            if dist < 0.01 {
                break;
            }

            current = next;
        }

        // Estimate convergence rate
        let convergence_rate = if distances.len() >= 2 {
            distances.last().unwrap_or(&0.0) / (distances.first().unwrap_or(&1.0) + 1e-10)
        } else {
            0.0
        };

        (current, convergence_rate)
    }

    /// Check if the operator is contractive based on distance sequence
    pub fn is_contractive(distances: &[f64]) -> bool {
        if distances.len() < 2 {
            return false;
        }

        for i in 1..distances.len() {
            if distances[i] >= distances[i - 1] {
                return false;
            }
        }

        true
    }
}

impl Default for DoubleKickOperator {
    fn default() -> Self {
        Self::new(0.3, 0.2)
    }
}

impl CalibrationOperator for DoubleKickOperator {
    fn apply(
        &self,
        config: &Configuration,
        performance: &Signature3D,
        field: Option<&MandorlaField>,
    ) -> Configuration {
        DoubleKickOperator::apply(self, config, performance, field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_distance() {
        let c1 = Configuration::default();
        let c2 = Configuration::default();

        assert!(c1.distance(&c2) < 0.001);

        let mut c3 = Configuration::default();
        c3.depth = 5;

        assert!(c1.distance(&c3) > 0.0);
    }

    #[test]
    fn test_update_kick() {
        let kick = UpdateKick::default();
        let config = Configuration::default();
        let perf = Signature3D::new(0.5, 0.5, 0.5);

        let new_config = kick.apply(&config, &perf, None);

        // Should have made some changes for low quality
        assert!(config.distance(&new_config) > 0.0);
    }

    #[test]
    fn test_stabilization_kick() {
        let kick = StabilizationKick::default();
        let config = Configuration::default();
        let perf = Signature3D::new(0.5, 0.5, 0.5);

        let new_config = kick.apply(&config, &perf, None);

        // Should have made some changes for low stability
        assert!(config.distance(&new_config) > 0.0);
    }

    #[test]
    fn test_double_kick() {
        let op = DoubleKickOperator::default();
        let config = Configuration::default();
        let perf = Signature3D::new(0.5, 0.5, 0.5);

        let (final_config, convergence) = op.iterate(&config, &perf, None, 5);

        // Should have evolved
        assert!(config.distance(&final_config) > 0.0);
        // Convergence rate should be less than 1 for contractive operator
        assert!(convergence < 1.0);
    }
}
