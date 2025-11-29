//! Advanced Seraphic calibration analysis and auto-tuning.

use crate::calibrator::{SeraphicCalibrator, CalibratorConfig, CalibrationResult};
use qops_core::{Configuration, Signature3D, resonance_3d};
use serde::{Deserialize, Serialize};

/// Sweep configuration for hyperparameter exploration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweepConfig {
    /// Temperature values to sweep
    pub temperature_values: Vec<f64>,
    /// Cooling rate values to sweep
    pub cooling_rate_values: Vec<f64>,
    /// Steps per evaluation
    pub steps_per_evaluation: usize,
    /// Enable parallel evaluation
    pub parallel: bool,
}

impl Default for SweepConfig {
    fn default() -> Self {
        Self {
            temperature_values: vec![0.5, 1.0, 2.0, 5.0],
            cooling_rate_values: vec![0.9, 0.95, 0.99],
            steps_per_evaluation: 20,
            parallel: true,
        }
    }
}

/// Configuration snapshot for sweep evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweepConfigSnapshot {
    /// Temperature parameter
    pub temperature: f64,
    /// Cooling rate parameter
    pub cooling_rate: f64,
}

/// Result of evaluating a single configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEvaluation {
    /// Configuration tested
    pub config: SweepConfigSnapshot,
    /// Final score achieved
    pub final_score: f64,
    /// Convergence rate (0-1)
    pub convergence_rate: f64,
}

/// Result of a hyperparameter sweep
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweepResult {
    /// All evaluations performed
    pub evaluations: Vec<ConfigEvaluation>,
    /// Index of best configuration
    pub best_config_index: usize,
    /// Best score achieved
    pub best_score: f64,
}

/// Hyperparameter sweep engine
pub struct HyperparameterSweep {
    config: SweepConfig,
    results: Vec<ConfigEvaluation>,
}

impl HyperparameterSweep {
    /// Create new sweep with configuration
    pub fn new(config: SweepConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    /// Run the sweep
    pub fn run(&mut self) -> SweepResult {
        let mut best_index = 0;
        let mut best_score = 0.0;

        for (_t_idx, &temp) in self.config.temperature_values.iter().enumerate() {
            for (_c_idx, &cooling) in self.config.cooling_rate_values.iter().enumerate() {
                let eval = self.evaluate_config(temp, cooling);

                if eval.final_score > best_score {
                    best_score = eval.final_score;
                    best_index = self.results.len();
                }

                self.results.push(eval);
            }
        }

        SweepResult {
            evaluations: self.results.clone(),
            best_config_index: best_index,
            best_score,
        }
    }

    /// Evaluate a single configuration
    fn evaluate_config(&self, temperature: f64, cooling_rate: f64) -> ConfigEvaluation {
        let calibrator_config = CalibratorConfig {
            initial_temperature: temperature,
            cooling_rate,
            ..Default::default()
        };

        let mut calibrator = SeraphicCalibrator::new(calibrator_config);
        let init_config = Configuration::new("sweep_test");
        calibrator.initialize(init_config, Signature3D::new(0.5, 0.5, 0.5));

        let results = calibrator.run(self.config.steps_per_evaluation);

        let resonances: Vec<f64> = results.iter()
            .map(|r| resonance_3d(&r.performance))
            .collect();

        let final_score = *resonances.last().unwrap_or(&0.0);

        // Convergence rate: how many steps were accepted
        let accepted = results.iter().filter(|r| r.accepted).count();
        let convergence_rate = accepted as f64 / results.len().max(1) as f64;

        ConfigEvaluation {
            config: SweepConfigSnapshot {
                temperature,
                cooling_rate,
            },
            final_score,
            convergence_rate,
        }
    }
}

/// Stability analysis for calibration results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityAnalysis {
    /// Mean resonance
    pub mean_resonance: f64,
    /// Standard deviation
    pub std_resonance: f64,
    /// Coefficient of variation
    pub cv: f64,
    /// Trend (positive = improving, negative = degrading)
    pub trend: f64,
    /// Stability score (0-1)
    pub stability_score: f64,
    /// Is system stable
    pub is_stable: bool,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Analyze stability of calibration
pub fn analyze_stability(results: &[CalibrationResult]) -> StabilityAnalysis {
    if results.is_empty() {
        return StabilityAnalysis {
            mean_resonance: 0.0,
            std_resonance: 0.0,
            cv: 0.0,
            trend: 0.0,
            stability_score: 0.0,
            is_stable: false,
            recommendations: vec!["No data to analyze".to_string()],
        };
    }

    let resonances: Vec<f64> = results.iter()
        .map(|r| resonance_3d(&r.performance))
        .collect();

    let n = resonances.len() as f64;
    let mean = resonances.iter().sum::<f64>() / n;

    let variance = resonances.iter()
        .map(|r| (r - mean).powi(2))
        .sum::<f64>() / n;
    let std = variance.sqrt();

    let cv = if mean > 0.0 { std / mean } else { 0.0 };

    // Compute trend using linear regression
    let trend = compute_trend(&resonances);

    // Stability score: high mean, low cv, positive trend
    let stability_score = (
        0.4 * mean +
        0.4 * (1.0 - cv.min(1.0)) +
        0.2 * (trend + 1.0) / 2.0
    ).clamp(0.0, 1.0);

    let is_stable = stability_score >= 0.7 && cv < 0.2;

    let mut recommendations = Vec::new();
    if cv > 0.2 {
        recommendations.push("High variance detected. Consider reducing exploration rate.".to_string());
    }
    if trend < -0.1 {
        recommendations.push("Negative trend detected. Check for divergence.".to_string());
    }
    if mean < 0.5 {
        recommendations.push("Low resonance. Consider adjusting hyperparameters.".to_string());
    }
    if stability_score >= 0.8 {
        recommendations.push("System is stable. Consider exploitation mode.".to_string());
    }

    StabilityAnalysis {
        mean_resonance: mean,
        std_resonance: std,
        cv,
        trend,
        stability_score,
        is_stable,
        recommendations,
    }
}

fn compute_trend(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }

    let n = values.len() as f64;
    let x_mean = (n - 1.0) / 2.0;
    let y_mean = values.iter().sum::<f64>() / n;

    let mut numerator = 0.0;
    let mut denominator = 0.0;

    for (i, &y) in values.iter().enumerate() {
        let x = i as f64;
        numerator += (x - x_mean) * (y - y_mean);
        denominator += (x - x_mean).powi(2);
    }

    if denominator == 0.0 {
        return 0.0;
    }

    numerator / denominator
}

/// Auto-tuning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoTuneResult {
    /// Best configuration found
    pub best_config: AutoTuneConfig,
    /// Achieved resonance
    pub achieved_resonance: f64,
    /// Number of iterations
    pub iterations: usize,
}

/// Auto-tune configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoTuneConfig {
    /// Temperature
    pub temperature: f64,
    /// Cooling rate
    pub cooling_rate: f64,
    /// Mandorla threshold
    pub mandorla_threshold: f64,
}

impl Default for AutoTuneConfig {
    fn default() -> Self {
        Self {
            temperature: 1.0,
            cooling_rate: 0.95,
            mandorla_threshold: 0.85,
        }
    }
}

/// Auto-tuning engine
pub struct AutoTuner {
    /// Target resonance
    target_resonance: f64,
    /// Current best configuration
    best_config: AutoTuneConfig,
    /// Current best score
    best_score: f64,
    /// History
    history: Vec<(AutoTuneConfig, f64)>,
    /// Iteration count
    iteration: usize,
}

impl AutoTuner {
    /// Create new auto-tuner with target resonance
    pub fn new(target_resonance: f64) -> Self {
        Self {
            target_resonance,
            best_config: AutoTuneConfig::default(),
            best_score: 0.0,
            history: Vec::new(),
            iteration: 0,
        }
    }

    /// Run auto-tuning for specified iterations
    pub fn tune(&mut self, max_iterations: usize) -> AutoTuneResult {
        for _ in 0..max_iterations {
            let candidate = self.generate_candidate();
            let score = self.evaluate(&candidate);

            self.history.push((candidate.clone(), score));

            if score > self.best_score {
                self.best_score = score;
                self.best_config = candidate;
            }

            self.iteration += 1;

            // Early stopping if we reached target
            if self.best_score >= self.target_resonance {
                break;
            }
        }

        AutoTuneResult {
            best_config: self.best_config.clone(),
            achieved_resonance: self.best_score,
            iterations: self.iteration,
        }
    }

    fn generate_candidate(&self) -> AutoTuneConfig {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        if self.iteration < 5 || rng.gen::<f64>() > 0.7 {
            // Exploration: random configuration
            AutoTuneConfig {
                temperature: rng.gen_range(0.1..5.0),
                cooling_rate: rng.gen_range(0.8..0.999),
                mandorla_threshold: rng.gen_range(0.7..0.95),
            }
        } else {
            // Exploitation: perturb best
            let temp_delta = rng.gen_range(-0.5..0.5);
            let cool_delta = rng.gen_range(-0.05..0.05);
            let mand_delta = rng.gen_range(-0.05..0.05);

            AutoTuneConfig {
                temperature: (self.best_config.temperature + temp_delta).clamp(0.1, 10.0),
                cooling_rate: (self.best_config.cooling_rate + cool_delta).clamp(0.8, 0.999),
                mandorla_threshold: (self.best_config.mandorla_threshold + mand_delta).clamp(0.5, 0.99),
            }
        }
    }

    fn evaluate(&self, config: &AutoTuneConfig) -> f64 {
        let calibrator_config = CalibratorConfig {
            initial_temperature: config.temperature,
            cooling_rate: config.cooling_rate,
            mandorla_threshold: config.mandorla_threshold,
            ..Default::default()
        };

        let mut calibrator = SeraphicCalibrator::new(calibrator_config);
        let init_config = Configuration::new("auto_tune");
        calibrator.initialize(init_config, Signature3D::new(0.5, 0.5, 0.5));

        let results = calibrator.run(20);

        // Final resonance
        results.last()
            .map(|r| resonance_3d(&r.performance))
            .unwrap_or(0.0)
    }
}

impl Default for AutoTuner {
    fn default() -> Self {
        Self::new(0.85)
    }
}

/// Calibration curve for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationCurve {
    /// X values (usually iterations)
    pub x: Vec<f64>,
    /// Y values (usually resonance)
    pub y: Vec<f64>,
    /// Curve type
    pub curve_type: CurveType,
}

/// Type of calibration curve
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CurveType {
    /// Resonance over time
    Resonance,
    /// Temperature over time
    Temperature,
    /// Acceptance rate over time
    AcceptanceRate,
}

impl CalibrationCurve {
    /// Create from calibration results
    pub fn from_results(results: &[CalibrationResult], curve_type: CurveType) -> Self {
        let x: Vec<f64> = (0..results.len()).map(|i| i as f64).collect();

        let y = match curve_type {
            CurveType::Resonance => {
                results.iter().map(|r| resonance_3d(&r.performance)).collect()
            }
            CurveType::Temperature => {
                // Approximate temperature decay
                let init_temp: f64 = 1.0;
                let cooling: f64 = 0.95;
                (0..results.len())
                    .map(|i| init_temp * cooling.powi(i as i32))
                    .collect()
            }
            CurveType::AcceptanceRate => {
                // Cumulative acceptance rate
                let mut accepted = 0usize;
                results.iter().enumerate().map(|(i, r)| {
                    if r.accepted { accepted += 1; }
                    accepted as f64 / (i + 1) as f64
                }).collect()
            }
        };

        Self { x, y, curve_type }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sweep() {
        let config = SweepConfig {
            temperature_values: vec![0.5, 1.0],
            cooling_rate_values: vec![0.9, 0.95],
            steps_per_evaluation: 5,
            parallel: false,
        };

        let mut sweep = HyperparameterSweep::new(config);
        let result = sweep.run();

        assert_eq!(result.evaluations.len(), 4); // 2x2 grid
        assert!(result.best_score >= 0.0);
    }

    #[test]
    fn test_auto_tuner() {
        let mut tuner = AutoTuner::new(0.7);
        let result = tuner.tune(5);

        assert!(result.iterations <= 5);
        assert!(result.achieved_resonance >= 0.0);
    }
}
