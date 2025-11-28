//! TRITON configuration types.

use serde::{Deserialize, Serialize};

/// Main TRITON optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TritonConfig {
    /// Spiral search parameters
    pub spiral: SpiralParams,
    /// Temperature schedule for annealing
    pub temperature: TemperatureSchedule,
    /// Refinement configuration
    pub refinement: RefinementConfig,
    /// Maximum iterations
    pub max_iterations: usize,
    /// Convergence threshold
    pub convergence_threshold: f64,
    /// Enable parallel processing
    pub parallel: bool,
    /// Number of parallel workers
    pub workers: usize,
    /// Deterministic mode (fixed seed)
    pub deterministic: bool,
    /// Random seed for deterministic mode
    pub seed: u64,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for TritonConfig {
    fn default() -> Self {
        Self {
            spiral: SpiralParams::default(),
            temperature: TemperatureSchedule::default(),
            refinement: RefinementConfig::default(),
            max_iterations: 1000,
            convergence_threshold: 1e-6,
            parallel: true,
            workers: 4,
            deterministic: false,
            seed: 42,
            verbose: false,
        }
    }
}

impl TritonConfig {
    /// Create config for quick exploration
    pub fn quick() -> Self {
        Self {
            max_iterations: 100,
            spiral: SpiralParams {
                layers: 3,
                expansion_rate: 1.5,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create config for thorough search
    pub fn thorough() -> Self {
        Self {
            max_iterations: 5000,
            spiral: SpiralParams {
                layers: 10,
                expansion_rate: 1.618,
                ..Default::default()
            },
            refinement: RefinementConfig {
                passes: 5,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create config for deterministic reproducible search
    pub fn deterministic(seed: u64) -> Self {
        Self {
            deterministic: true,
            seed,
            parallel: false,
            ..Default::default()
        }
    }
}

/// Spiral search parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiralParams {
    /// Number of spiral layers
    pub layers: usize,
    /// Initial radius
    pub initial_radius: f64,
    /// Expansion rate per layer (default: golden ratio)
    pub expansion_rate: f64,
    /// Angular step (radians per iteration)
    pub angular_step: f64,
    /// Points per layer
    pub points_per_layer: usize,
    /// Enable radius adaptation
    pub adaptive_radius: bool,
    /// Minimum radius
    pub min_radius: f64,
    /// Maximum radius
    pub max_radius: f64,
}

impl Default for SpiralParams {
    fn default() -> Self {
        Self {
            layers: 7,
            initial_radius: 0.1,
            expansion_rate: 1.618, // Golden ratio
            angular_step: std::f64::consts::PI / 6.0, // 30 degrees
            points_per_layer: 12,
            adaptive_radius: true,
            min_radius: 0.01,
            max_radius: 1.0,
        }
    }
}

impl SpiralParams {
    /// Compute radius for a given layer
    pub fn radius_at_layer(&self, layer: usize) -> f64 {
        let r = self.initial_radius * self.expansion_rate.powi(layer as i32);
        r.clamp(self.min_radius, self.max_radius)
    }

    /// Total number of search points
    pub fn total_points(&self) -> usize {
        self.layers * self.points_per_layer
    }
}

/// Temperature schedule for simulated annealing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureSchedule {
    /// Initial temperature
    pub initial: f64,
    /// Final temperature
    pub final_temp: f64,
    /// Decay factor per step
    pub decay: f64,
    /// Annealing schedule type
    pub schedule: AnnealingScheduleType,
    /// Reheat factor (for adaptive schedules)
    pub reheat_factor: f64,
    /// Steps between reheat checks
    pub reheat_interval: usize,
}

impl Default for TemperatureSchedule {
    fn default() -> Self {
        Self {
            initial: 1.0,
            final_temp: 0.001,
            decay: 0.95,
            schedule: AnnealingScheduleType::Exponential,
            reheat_factor: 1.5,
            reheat_interval: 100,
        }
    }
}

/// Annealing schedule types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnealingScheduleType {
    /// T(n) = T0 * decay^n
    Exponential,
    /// T(n) = T0 / (1 + n)
    Linear,
    /// T(n) = T0 / log(1 + n)
    Logarithmic,
    /// T(n) = T0 * (1 - n/N)^2
    Quadratic,
    /// Adaptive based on acceptance rate
    Adaptive,
}

/// Refinement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefinementConfig {
    /// Number of refinement passes
    pub passes: usize,
    /// Shrink factor per pass
    pub shrink_factor: f64,
    /// Minimum search radius for refinement
    pub min_radius: f64,
    /// Enable gradient-based refinement
    pub gradient_refinement: bool,
    /// Gradient step size
    pub gradient_step: f64,
    /// Local search iterations
    pub local_iterations: usize,
}

impl Default for RefinementConfig {
    fn default() -> Self {
        Self {
            passes: 3,
            shrink_factor: 0.5,
            min_radius: 0.001,
            gradient_refinement: true,
            gradient_step: 0.01,
            local_iterations: 50,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = TritonConfig::default();
        assert_eq!(config.spiral.layers, 7);
        assert!(!config.deterministic);
    }

    #[test]
    fn test_spiral_radius() {
        let params = SpiralParams::default();
        let r0 = params.radius_at_layer(0);
        let r1 = params.radius_at_layer(1);
        assert!(r1 > r0);
    }

    #[test]
    fn test_total_points() {
        let params = SpiralParams::default();
        assert_eq!(params.total_points(), 84); // 7 * 12
    }
}
