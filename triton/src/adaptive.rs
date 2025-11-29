//! Adaptive TRITON Spiral Search Enhancement
//!
//! Advanced features for spiral search optimization:
//! - Adaptive radius based on search progress
//! - Dynamic cooling with temperature memory
//! - Topology-aware Gaussian bias
//! - Spiral layer memory for informed search
//! - Convergence stabilizer
//! - Drift corrector
//! - Fine-grain local search mode
//! - Resonance-weighted expansion

use crate::config::TritonConfig;
use crate::spiral::{SpiralEngine, SpiralTrajectory};
use qops_core::{Signature5D, resonance_5d, HolisticMatrix, HolisticConfig, OperatorCandidate};
use serde::{Deserialize, Serialize};
use rand::Rng;
use std::f64::consts::PI;

// ============================================================================
// ADAPTIVE RADIUS CONTROLLER
// ============================================================================

/// Configuration for adaptive radius
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveRadiusConfig {
    /// Base radius
    pub base_radius: f64,
    /// Minimum radius (convergence floor)
    pub min_radius: f64,
    /// Maximum radius (exploration ceiling)
    pub max_radius: f64,
    /// Contraction rate when improving
    pub contraction_rate: f64,
    /// Expansion rate when stuck
    pub expansion_rate: f64,
    /// Stagnation threshold (steps without improvement)
    pub stagnation_threshold: usize,
    /// Success threshold for contraction
    pub success_rate_target: f64,
}

impl Default for AdaptiveRadiusConfig {
    fn default() -> Self {
        Self {
            base_radius: 0.2,
            min_radius: 0.005,
            max_radius: 0.8,
            contraction_rate: 0.9,
            expansion_rate: 1.2,
            stagnation_threshold: 20,
            success_rate_target: 0.4,
        }
    }
}

/// Adaptive radius controller
#[derive(Debug, Clone)]
pub struct AdaptiveRadiusController {
    config: AdaptiveRadiusConfig,
    current_radius: f64,
    history: Vec<f64>,
    successes: usize,
    failures: usize,
    steps_since_improvement: usize,
}

impl AdaptiveRadiusController {
    /// Create new adaptive radius controller
    pub fn new(config: AdaptiveRadiusConfig) -> Self {
        Self {
            current_radius: config.base_radius,
            config,
            history: Vec::new(),
            successes: 0,
            failures: 0,
            steps_since_improvement: 0,
        }
    }

    /// Get current radius
    pub fn radius(&self) -> f64 {
        self.current_radius
    }

    /// Record a successful step (improvement found)
    pub fn record_success(&mut self) {
        self.successes += 1;
        self.steps_since_improvement = 0;
        self.history.push(self.current_radius);
        self.adapt();
    }

    /// Record a failed step (no improvement)
    pub fn record_failure(&mut self) {
        self.failures += 1;
        self.steps_since_improvement += 1;
        self.adapt();
    }

    /// Adapt radius based on performance
    fn adapt(&mut self) {
        let total = self.successes + self.failures;
        if total == 0 {
            return;
        }

        let success_rate = self.successes as f64 / total as f64;

        // Contract if success rate is high (converging)
        if success_rate > self.config.success_rate_target {
            self.current_radius *= self.config.contraction_rate;
        }
        // Expand if stagnating
        else if self.steps_since_improvement >= self.config.stagnation_threshold {
            self.current_radius *= self.config.expansion_rate;
            self.steps_since_improvement = 0;
        }

        // Clamp to valid range
        self.current_radius = self.current_radius.clamp(
            self.config.min_radius,
            self.config.max_radius,
        );
    }

    /// Force contraction (for local search)
    pub fn contract(&mut self) {
        self.current_radius *= self.config.contraction_rate;
        self.current_radius = self.current_radius.max(self.config.min_radius);
    }

    /// Force expansion (for exploration)
    pub fn expand(&mut self) {
        self.current_radius *= self.config.expansion_rate;
        self.current_radius = self.current_radius.min(self.config.max_radius);
    }

    /// Get statistics
    pub fn stats(&self) -> AdaptiveRadiusStats {
        let total = self.successes + self.failures;
        AdaptiveRadiusStats {
            current_radius: self.current_radius,
            successes: self.successes,
            failures: self.failures,
            success_rate: if total > 0 { self.successes as f64 / total as f64 } else { 0.0 },
            steps_since_improvement: self.steps_since_improvement,
        }
    }
}

/// Adaptive radius statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveRadiusStats {
    pub current_radius: f64,
    pub successes: usize,
    pub failures: usize,
    pub success_rate: f64,
    pub steps_since_improvement: usize,
}

// ============================================================================
// DYNAMIC COOLING CONTROLLER
// ============================================================================

/// Dynamic cooling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicCoolingConfig {
    /// Initial temperature
    pub initial_temp: f64,
    /// Final temperature target
    pub final_temp: f64,
    /// Base cooling rate
    pub base_cooling_rate: f64,
    /// Reheat factor when stuck
    pub reheat_factor: f64,
    /// Steps to consider for cooling decision
    pub memory_window: usize,
    /// Enable adaptive reheating
    pub adaptive_reheat: bool,
}

impl Default for DynamicCoolingConfig {
    fn default() -> Self {
        Self {
            initial_temp: 1.0,
            final_temp: 0.001,
            base_cooling_rate: 0.98,
            reheat_factor: 1.5,
            memory_window: 50,
            adaptive_reheat: true,
        }
    }
}

/// Dynamic cooling controller with temperature memory
#[derive(Debug, Clone)]
pub struct DynamicCoolingController {
    config: DynamicCoolingConfig,
    temperature: f64,
    temp_history: Vec<f64>,
    score_history: Vec<f64>,
    step: usize,
    reheat_count: usize,
}

impl DynamicCoolingController {
    /// Create new dynamic cooling controller
    pub fn new(config: DynamicCoolingConfig) -> Self {
        Self {
            temperature: config.initial_temp,
            config,
            temp_history: Vec::new(),
            score_history: Vec::new(),
            step: 0,
            reheat_count: 0,
        }
    }

    /// Get current temperature
    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// Cool the temperature
    pub fn cool(&mut self, current_score: f64, best_score: f64) {
        self.step += 1;
        self.score_history.push(current_score);
        self.temp_history.push(self.temperature);

        // Check if we should reheat
        if self.config.adaptive_reheat && self.should_reheat(best_score) {
            self.reheat();
        } else {
            // Apply dynamic cooling
            let cooling_rate = self.compute_dynamic_cooling_rate();
            self.temperature *= cooling_rate;
        }

        // Ensure minimum temperature
        self.temperature = self.temperature.max(self.config.final_temp);
    }

    /// Check if reheating is needed
    fn should_reheat(&self, best_score: f64) -> bool {
        if self.score_history.len() < self.config.memory_window {
            return false;
        }

        // Check if stuck (no improvement in window)
        let window: Vec<_> = self.score_history.iter()
            .rev()
            .take(self.config.memory_window)
            .collect();

        let max_recent = window.iter().copied().copied().fold(f64::MIN, f64::max);
        let min_recent = window.iter().copied().copied().fold(f64::MAX, f64::min);

        // Stuck if variance is very low and not at best
        let variance = max_recent - min_recent;
        variance < 0.001 && max_recent < best_score * 0.99
    }

    /// Compute dynamic cooling rate based on progress
    fn compute_dynamic_cooling_rate(&self) -> f64 {
        if self.score_history.len() < 2 {
            return self.config.base_cooling_rate;
        }

        let recent_improvement = if self.score_history.len() >= 10 {
            let n = self.score_history.len();
            self.score_history[n - 1] - self.score_history[n - 10]
        } else {
            0.0
        };

        // Slow down cooling if improving, speed up if stuck
        if recent_improvement > 0.01 {
            // Improving - cool slower
            (self.config.base_cooling_rate + 1.0) / 2.0
        } else if recent_improvement < -0.01 {
            // Worsening - cool faster
            self.config.base_cooling_rate * 0.9
        } else {
            self.config.base_cooling_rate
        }
    }

    /// Reheat the temperature
    pub fn reheat(&mut self) {
        self.temperature = (self.temperature * self.config.reheat_factor)
            .min(self.config.initial_temp);
        self.reheat_count += 1;
    }

    /// Accept move based on temperature (Metropolis criterion)
    pub fn accept(&self, current_score: f64, new_score: f64) -> bool {
        if new_score >= current_score {
            return true;
        }

        let delta = new_score - current_score;
        let probability = (delta / self.temperature).exp();

        rand::thread_rng().gen::<f64>() < probability
    }

    /// Get cooling statistics
    pub fn stats(&self) -> DynamicCoolingStats {
        DynamicCoolingStats {
            temperature: self.temperature,
            step: self.step,
            reheat_count: self.reheat_count,
            temp_history_len: self.temp_history.len(),
        }
    }
}

/// Dynamic cooling statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicCoolingStats {
    pub temperature: f64,
    pub step: usize,
    pub reheat_count: usize,
    pub temp_history_len: usize,
}

// ============================================================================
// TOPOLOGY-AWARE GAUSSIAN BIAS
// ============================================================================

/// Topology-aware bias configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyBiasConfig {
    /// Number of Gaussian centers to maintain
    pub num_centers: usize,
    /// Standard deviation for Gaussian
    pub sigma: f64,
    /// Decay rate for old centers
    pub decay_rate: f64,
    /// Weight for topology bias vs random
    pub bias_weight: f64,
    /// Minimum weight before center is removed
    pub min_weight: f64,
}

impl Default for TopologyBiasConfig {
    fn default() -> Self {
        Self {
            num_centers: 10,
            sigma: 0.15,
            decay_rate: 0.95,
            bias_weight: 0.6,
            min_weight: 0.01,
        }
    }
}

/// A Gaussian center for biased sampling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaussianCenter {
    /// Center position
    pub center: [f64; 5],
    /// Weight (importance)
    pub weight: f64,
    /// Score at this center
    pub score: f64,
}

/// Topology-aware Gaussian bias sampler
#[derive(Debug, Clone)]
pub struct TopologyGaussianBias {
    config: TopologyBiasConfig,
    centers: Vec<GaussianCenter>,
    rng: rand::rngs::StdRng,
}

impl TopologyGaussianBias {
    /// Create new Gaussian bias sampler
    pub fn new(config: TopologyBiasConfig) -> Self {
        Self::with_seed(config, rand::random())
    }

    /// Create with specific seed
    pub fn with_seed(config: TopologyBiasConfig, seed: u64) -> Self {
        use rand::SeedableRng;
        Self {
            config,
            centers: Vec::new(),
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    /// Add a new center
    pub fn add_center(&mut self, sig: &Signature5D, score: f64) {
        let center = GaussianCenter {
            center: sig.to_vec(),
            weight: 1.0,
            score,
        };

        self.centers.push(center);

        // Keep only top centers
        if self.centers.len() > self.config.num_centers {
            self.centers.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            self.centers.truncate(self.config.num_centers);
        }

        // Decay old centers
        for c in &mut self.centers {
            c.weight *= self.config.decay_rate;
        }

        // Remove weak centers
        self.centers.retain(|c| c.weight >= self.config.min_weight);
    }

    /// Sample a point with topology bias
    pub fn sample(&mut self, current: &Signature5D, radius: f64) -> Signature5D {
        // Decide between biased and random sampling
        if self.centers.is_empty() || self.rng.gen::<f64>() > self.config.bias_weight {
            // Random perturbation
            return self.random_perturbation(current, radius);
        }

        // Select a center weighted by score
        let total_weight: f64 = self.centers.iter().map(|c| c.weight * c.score).sum();
        if total_weight <= 0.0 {
            return self.random_perturbation(current, radius);
        }

        let mut cumulative = 0.0;
        let threshold = self.rng.gen::<f64>() * total_weight;

        // Find the selected center first, then sample
        let mut selected_center: Option<[f64; 5]> = None;
        for center in &self.centers {
            cumulative += center.weight * center.score;
            if cumulative >= threshold {
                selected_center = Some(center.center);
                break;
            }
        }

        if let Some(center) = selected_center {
            // Sample from Gaussian around this center
            let sigma = self.config.sigma;
            return self.gaussian_sample(&center, sigma);
        }

        self.random_perturbation(current, radius)
    }

    /// Sample from Gaussian distribution
    fn gaussian_sample(&mut self, center: &[f64; 5], sigma: f64) -> Signature5D {
        let mut normal = || {
            // Box-Muller transform
            let u1: f64 = self.rng.gen();
            let u2: f64 = self.rng.gen();
            (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
        };

        Signature5D::new(
            (center[0] + sigma * normal()).clamp(0.0, 1.0),
            (center[1] + sigma * normal()).clamp(0.0, 1.0),
            (center[2] + sigma * normal()).clamp(0.0, 1.0),
            (center[3] + sigma * 0.5 * normal()).clamp(0.0, 1.0),
            (center[4] + sigma * 0.3 * normal()).clamp(0.0, 1.0),
        )
    }

    /// Random perturbation
    fn random_perturbation(&mut self, current: &Signature5D, radius: f64) -> Signature5D {
        Signature5D::new(
            (current.psi + self.rng.gen_range(-radius..radius)).clamp(0.0, 1.0),
            (current.rho + self.rng.gen_range(-radius..radius)).clamp(0.0, 1.0),
            (current.omega + self.rng.gen_range(-radius..radius)).clamp(0.0, 1.0),
            (current.chi + self.rng.gen_range(-radius * 0.5..radius * 0.5)).clamp(0.0, 1.0),
            (current.eta + self.rng.gen_range(-radius * 0.3..radius * 0.3)).clamp(0.0, 1.0),
        )
    }

    /// Get current centers
    pub fn centers(&self) -> &[GaussianCenter] {
        &self.centers
    }
}

// ============================================================================
// SPIRAL LAYER MEMORY
// ============================================================================

/// Memory of best results per spiral layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiralLayerMemory {
    /// Best signature per layer
    pub layer_bests: Vec<Option<LayerBest>>,
    /// Layer-wise statistics
    pub layer_stats: Vec<LayerStats>,
}

/// Best result in a layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerBest {
    pub signature: [f64; 5],
    pub score: f64,
    pub point_index: usize,
}

/// Statistics for a layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerStats {
    pub layer: usize,
    pub points_evaluated: usize,
    pub avg_score: f64,
    pub max_score: f64,
    pub min_score: f64,
    pub variance: f64,
}

impl SpiralLayerMemory {
    /// Create new layer memory
    pub fn new(num_layers: usize) -> Self {
        Self {
            layer_bests: vec![None; num_layers],
            layer_stats: Vec::new(),
        }
    }

    /// Record a point evaluation
    pub fn record(&mut self, layer: usize, sig: &Signature5D, score: f64, point_index: usize) {
        // Ensure we have enough layers
        while self.layer_bests.len() <= layer {
            self.layer_bests.push(None);
        }

        // Update best for this layer
        let is_best = match &self.layer_bests[layer] {
            None => true,
            Some(best) => score > best.score,
        };

        if is_best {
            self.layer_bests[layer] = Some(LayerBest {
                signature: sig.to_vec(),
                score,
                point_index,
            });
        }
    }

    /// Finalize layer statistics
    pub fn finalize_layer(&mut self, layer: usize, scores: &[f64]) {
        if scores.is_empty() {
            return;
        }

        let n = scores.len() as f64;
        let sum: f64 = scores.iter().sum();
        let avg = sum / n;
        let max = scores.iter().copied().fold(f64::MIN, f64::max);
        let min = scores.iter().copied().fold(f64::MAX, f64::min);
        let variance: f64 = scores.iter().map(|s| (s - avg).powi(2)).sum::<f64>() / n;

        self.layer_stats.push(LayerStats {
            layer,
            points_evaluated: scores.len(),
            avg_score: avg,
            max_score: max,
            min_score: min,
            variance,
        });
    }

    /// Get best layer
    pub fn best_layer(&self) -> Option<(usize, f64)> {
        self.layer_bests.iter()
            .enumerate()
            .filter_map(|(i, opt)| opt.as_ref().map(|b| (i, b.score)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    /// Get convergence trend across layers
    pub fn convergence_trend(&self) -> Vec<f64> {
        self.layer_stats.iter().map(|s| s.max_score).collect()
    }

    /// Get number of layers visited (layers with recorded data)
    pub fn layers_visited(&self) -> usize {
        self.layer_bests.iter().filter(|opt| opt.is_some()).count()
    }
}

// ============================================================================
// CONVERGENCE STABILIZER
// ============================================================================

/// Convergence stabilizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceStabilizerConfig {
    /// Window size for convergence detection
    pub window_size: usize,
    /// Threshold for convergence
    pub convergence_threshold: f64,
    /// Number of confirmations needed
    pub confirmation_count: usize,
    /// Enable early stopping
    pub early_stop: bool,
}

impl Default for ConvergenceStabilizerConfig {
    fn default() -> Self {
        Self {
            window_size: 50,
            convergence_threshold: 1e-5,
            confirmation_count: 3,
            early_stop: true,
        }
    }
}

/// Convergence stabilizer
#[derive(Debug, Clone)]
pub struct ConvergenceStabilizer {
    config: ConvergenceStabilizerConfig,
    score_history: Vec<f64>,
    best_history: Vec<f64>,
    confirmation: usize,
    converged: bool,
}

impl ConvergenceStabilizer {
    /// Create new convergence stabilizer
    pub fn new(config: ConvergenceStabilizerConfig) -> Self {
        Self {
            config,
            score_history: Vec::new(),
            best_history: Vec::new(),
            confirmation: 0,
            converged: false,
        }
    }

    /// Record a score
    pub fn record(&mut self, score: f64, best_score: f64) {
        self.score_history.push(score);
        self.best_history.push(best_score);
        self.check_convergence();
    }

    /// Check for convergence
    fn check_convergence(&mut self) {
        if self.best_history.len() < self.config.window_size {
            return;
        }

        let window: Vec<_> = self.best_history.iter()
            .rev()
            .take(self.config.window_size)
            .collect();

        let max = window.iter().copied().copied().fold(f64::MIN, f64::max);
        let min = window.iter().copied().copied().fold(f64::MAX, f64::min);

        if max - min < self.config.convergence_threshold {
            self.confirmation += 1;
        } else {
            self.confirmation = 0;
        }

        if self.confirmation >= self.config.confirmation_count {
            self.converged = true;
        }
    }

    /// Check if converged
    pub fn is_converged(&self) -> bool {
        self.converged
    }

    /// Should stop early
    pub fn should_stop(&self) -> bool {
        self.config.early_stop && self.converged
    }

    /// Get convergence point
    pub fn convergence_point(&self) -> Option<usize> {
        if self.converged {
            Some(self.best_history.len().saturating_sub(self.config.window_size))
        } else {
            None
        }
    }
}

// ============================================================================
// DRIFT CORRECTOR
// ============================================================================

/// Drift corrector for preventing search drift
#[derive(Debug, Clone)]
pub struct DriftCorrector {
    /// Initial best signature
    anchor: Option<Signature5D>,
    /// Maximum allowed drift from anchor
    max_drift: f64,
    /// Current drift
    current_drift: f64,
    /// Correction strength
    correction_strength: f64,
}

impl DriftCorrector {
    /// Create new drift corrector
    pub fn new(max_drift: f64, correction_strength: f64) -> Self {
        Self {
            anchor: None,
            max_drift,
            current_drift: 0.0,
            correction_strength,
        }
    }

    /// Set the anchor point
    pub fn set_anchor(&mut self, sig: &Signature5D) {
        self.anchor = Some(*sig);
    }

    /// Check and correct for drift
    pub fn correct(&mut self, sig: &Signature5D) -> Signature5D {
        let anchor = match &self.anchor {
            Some(a) => a,
            None => {
                self.anchor = Some(*sig);
                return *sig;
            }
        };

        let drift = sig.distance(anchor);
        self.current_drift = drift;

        if drift > self.max_drift {
            // Pull back towards anchor
            let factor = 1.0 - self.correction_strength * (drift - self.max_drift) / drift;
            Signature5D::new(
                anchor.psi + factor * (sig.psi - anchor.psi),
                anchor.rho + factor * (sig.rho - anchor.rho),
                anchor.omega + factor * (sig.omega - anchor.omega),
                anchor.chi + factor * (sig.chi - anchor.chi),
                anchor.eta + factor * (sig.eta - anchor.eta),
            )
        } else {
            *sig
        }
    }

    /// Get current drift
    pub fn drift(&self) -> f64 {
        self.current_drift
    }
}

// ============================================================================
// ADAPTIVE TRITON OPTIMIZER
// ============================================================================

/// Configuration for adaptive TRITON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveTritonConfig {
    /// Base TRITON config
    pub base: TritonConfig,
    /// Adaptive radius config
    pub radius: AdaptiveRadiusConfig,
    /// Dynamic cooling config
    pub cooling: DynamicCoolingConfig,
    /// Topology bias config
    pub topology_bias: TopologyBiasConfig,
    /// Convergence stabilizer config
    pub convergence: ConvergenceStabilizerConfig,
    /// Enable drift correction
    pub drift_correction: bool,
    /// Maximum drift allowed
    pub max_drift: f64,
    /// Enable fine-grain local search
    pub local_search: bool,
    /// Local search iterations
    pub local_iterations: usize,
    /// Enable resonance weighting
    pub resonance_weighted: bool,
    /// Integrate with Holistic Matrix
    pub holistic_integration: bool,
}

impl Default for AdaptiveTritonConfig {
    fn default() -> Self {
        Self {
            base: TritonConfig::default(),
            radius: AdaptiveRadiusConfig::default(),
            cooling: DynamicCoolingConfig::default(),
            topology_bias: TopologyBiasConfig::default(),
            convergence: ConvergenceStabilizerConfig::default(),
            drift_correction: true,
            max_drift: 0.5,
            local_search: true,
            local_iterations: 100,
            resonance_weighted: true,
            holistic_integration: true,
        }
    }
}

/// Result from adaptive TRITON optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveOptimizationResult {
    /// Best signature found
    pub best_signature: [f64; 5],
    /// Best score
    pub best_score: f64,
    /// Total iterations
    pub iterations: usize,
    /// Converged
    pub converged: bool,
    /// Layer memory
    pub layer_memory: SpiralLayerMemory,
    /// Radius statistics
    pub radius_stats: AdaptiveRadiusStats,
    /// Cooling statistics
    pub cooling_stats: DynamicCoolingStats,
    /// Convergence point (if any)
    pub convergence_point: Option<usize>,
    /// Holistic matrix output (if enabled)
    pub holistic_output: Option<HolisticMatrixOutput>,
}

/// Holistic matrix integration output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolisticMatrixOutput {
    pub valid_outputs: usize,
    pub monolith_count: usize,
    pub family_count: usize,
    pub current_stage: String,
}

/// Adaptive TRITON Optimizer with all enhancements
pub struct AdaptiveTritonOptimizer {
    config: AdaptiveTritonConfig,
    spiral: SpiralEngine,
    radius_controller: AdaptiveRadiusController,
    cooling: DynamicCoolingController,
    topology_bias: TopologyGaussianBias,
    layer_memory: SpiralLayerMemory,
    convergence: ConvergenceStabilizer,
    drift_corrector: DriftCorrector,
    holistic: Option<HolisticMatrix>,
    best_signature: Option<Signature5D>,
    best_score: f64,
    iteration: usize,
    trajectory: SpiralTrajectory,
}

impl AdaptiveTritonOptimizer {
    /// Create new adaptive TRITON optimizer
    pub fn new(config: AdaptiveTritonConfig) -> Self {
        let spiral = SpiralEngine::new(config.base.spiral.clone());
        let num_layers = config.base.spiral.layers;

        let holistic = if config.holistic_integration {
            Some(HolisticMatrix::new(HolisticConfig::default()))
        } else {
            None
        };

        Self {
            spiral,
            radius_controller: AdaptiveRadiusController::new(config.radius.clone()),
            cooling: DynamicCoolingController::new(config.cooling.clone()),
            topology_bias: TopologyGaussianBias::new(config.topology_bias.clone()),
            layer_memory: SpiralLayerMemory::new(num_layers),
            convergence: ConvergenceStabilizer::new(config.convergence.clone()),
            drift_corrector: DriftCorrector::new(config.max_drift, 0.3),
            holistic,
            best_signature: None,
            best_score: 0.0,
            iteration: 0,
            trajectory: SpiralTrajectory::new(),
            config,
        }
    }

    /// Run optimization with default resonance scoring
    pub fn optimize(&mut self) -> AdaptiveOptimizationResult {
        self.optimize_with_scorer(|sig| resonance_5d(sig))
    }

    /// Run optimization with custom scoring function
    pub fn optimize_with_scorer<F>(&mut self, scorer: F) -> AdaptiveOptimizationResult
    where
        F: Fn(&Signature5D) -> f64,
    {
        let mut current_layer = 0;
        let mut layer_scores: Vec<f64> = Vec::new();
        let mut holistic_candidates: Vec<OperatorCandidate> = Vec::new();

        // Phase 1: Spiral exploration with adaptive components
        while let Some(point) = self.spiral.next_point() {
            self.iteration += 1;

            // Apply drift correction
            let point = if self.config.drift_correction {
                self.drift_corrector.correct(&point)
            } else {
                point
            };

            // Score the point
            let score = scorer(&point);

            // Apply resonance weighting
            let weighted_score = if self.config.resonance_weighted {
                score * (1.0 + 0.1 * resonance_5d(&point))
            } else {
                score
            };

            // Record in layer memory
            self.layer_memory.record(
                self.spiral.state().layer,
                &point,
                weighted_score,
                self.spiral.state().point_index,
            );

            layer_scores.push(weighted_score);

            // Update best
            let improved = weighted_score > self.best_score;
            if improved {
                self.best_score = weighted_score;
                self.best_signature = Some(point);
                self.radius_controller.record_success();
                self.drift_corrector.set_anchor(&point);
            } else {
                self.radius_controller.record_failure();
            }

            // Record for topology bias
            if weighted_score > 0.5 {
                self.topology_bias.add_center(&point, weighted_score);
            }

            // Record trajectory
            self.trajectory.record(&point, weighted_score);

            // Update cooling
            self.cooling.cool(weighted_score, self.best_score);

            // Check convergence
            self.convergence.record(weighted_score, self.best_score);

            // Collect candidates for holistic processing
            if self.config.holistic_integration {
                holistic_candidates.push(OperatorCandidate {
                    id: format!("spiral_{}", self.iteration),
                    signature: point,
                    phase: self.spiral.state().angle,
                    resonance: weighted_score,
                    stability: self.radius_controller.stats().success_rate,
                    is_mandorla: weighted_score >= 0.85,
                    node_index: self.iteration,
                    discovered_at: self.iteration as f64,
                });
            }

            // Check for layer change
            if self.spiral.state().layer > current_layer {
                self.layer_memory.finalize_layer(current_layer, &layer_scores);
                self.trajectory.mark_layer();
                layer_scores.clear();
                current_layer = self.spiral.state().layer;
            }

            // Early stopping
            if self.convergence.should_stop() {
                break;
            }

            if self.iteration >= self.config.base.max_iterations {
                break;
            }
        }

        // Finalize last layer
        if !layer_scores.is_empty() {
            self.layer_memory.finalize_layer(current_layer, &layer_scores);
        }

        // Phase 2: Local search with fine-grain refinement
        if self.config.local_search {
            if let Some(best) = self.best_signature {
                let refined = self.local_search(&best, &scorer);
                let refined_score = scorer(&refined);
                if refined_score > self.best_score {
                    self.best_score = refined_score;
                    self.best_signature = Some(refined);
                }
            }
        }

        // Phase 3: Holistic matrix integration
        let holistic_output = if self.config.holistic_integration {
            self.process_holistic(holistic_candidates)
        } else {
            None
        };

        // Build result
        let sig = self.best_signature.unwrap_or(Signature5D::default());

        AdaptiveOptimizationResult {
            best_signature: sig.to_vec(),
            best_score: self.best_score,
            iterations: self.iteration,
            converged: self.convergence.is_converged(),
            layer_memory: self.layer_memory.clone(),
            radius_stats: self.radius_controller.stats(),
            cooling_stats: self.cooling.stats(),
            convergence_point: self.convergence.convergence_point(),
            holistic_output,
        }
    }

    /// Fine-grain local search
    fn local_search<F>(&mut self, start: &Signature5D, scorer: &F) -> Signature5D
    where
        F: Fn(&Signature5D) -> f64,
    {
        let mut current = *start;
        let mut current_score = scorer(&current);

        for _ in 0..self.config.local_iterations {
            // Contract radius for fine-grain search
            self.radius_controller.contract();
            let radius = self.radius_controller.radius();

            // Sample with topology bias
            let candidate = self.topology_bias.sample(&current, radius);
            let candidate_score = scorer(&candidate);

            // Accept based on cooling
            if self.cooling.accept(current_score, candidate_score) {
                current = candidate;
                current_score = candidate_score;
            }

            self.cooling.cool(candidate_score, self.best_score.max(current_score));
        }

        current
    }

    /// Process candidates through holistic matrix
    fn process_holistic(&mut self, candidates: Vec<OperatorCandidate>) -> Option<HolisticMatrixOutput> {
        let matrix = self.holistic.as_mut()?;

        // Process in batches
        let batch_size = 50;
        for (i, batch) in candidates.chunks(batch_size).enumerate() {
            let t = i as f64;
            matrix.process(batch.to_vec(), t);
        }

        let stats = matrix.stats();
        Some(HolisticMatrixOutput {
            valid_outputs: stats.valid_outputs,
            monolith_count: stats.pfauenthron.monolith_count,
            family_count: stats.pfauenthron.family_count,
            current_stage: format!("{:?}", stats.current_stage),
        })
    }

    /// Get trajectory
    pub fn trajectory(&self) -> &SpiralTrajectory {
        &self.trajectory
    }

    /// Reset optimizer
    pub fn reset(&mut self) {
        self.spiral.reset();
        self.radius_controller = AdaptiveRadiusController::new(self.config.radius.clone());
        self.cooling = DynamicCoolingController::new(self.config.cooling.clone());
        self.topology_bias = TopologyGaussianBias::new(self.config.topology_bias.clone());
        self.layer_memory = SpiralLayerMemory::new(self.config.base.spiral.layers);
        self.convergence = ConvergenceStabilizer::new(self.config.convergence.clone());
        self.drift_corrector = DriftCorrector::new(self.config.max_drift, 0.3);
        self.best_signature = None;
        self.best_score = 0.0;
        self.iteration = 0;
        self.trajectory = SpiralTrajectory::new();

        if self.config.holistic_integration {
            self.holistic = Some(HolisticMatrix::new(HolisticConfig::default()));
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SpiralParams;

    #[test]
    fn test_adaptive_radius() {
        let mut controller = AdaptiveRadiusController::new(AdaptiveRadiusConfig::default());

        let initial = controller.radius();
        controller.record_success();
        controller.record_success();
        controller.record_success();

        // Should contract after successes
        assert!(controller.radius() < initial);
    }

    #[test]
    fn test_dynamic_cooling() {
        let mut cooling = DynamicCoolingController::new(DynamicCoolingConfig::default());

        let initial_temp = cooling.temperature();
        cooling.cool(0.5, 0.5);
        cooling.cool(0.5, 0.5);

        // Temperature should decrease
        assert!(cooling.temperature() < initial_temp);
    }

    #[test]
    fn test_topology_bias() {
        let mut bias = TopologyGaussianBias::new(TopologyBiasConfig::default());
        let sig = Signature5D::new(0.8, 0.7, 0.6, 0.5, 0.4);

        bias.add_center(&sig, 0.9);
        assert_eq!(bias.centers().len(), 1);

        let sampled = bias.sample(&sig, 0.1);
        // Should be close to center
        assert!(sampled.psi > 0.0 && sampled.psi <= 1.0);
    }

    #[test]
    fn test_convergence_stabilizer() {
        let mut stabilizer = ConvergenceStabilizer::new(ConvergenceStabilizerConfig {
            window_size: 5,
            convergence_threshold: 0.01,
            confirmation_count: 2,
            early_stop: true,
        });

        for _ in 0..20 {
            stabilizer.record(0.9, 0.9);
        }

        assert!(stabilizer.is_converged());
    }

    #[test]
    fn test_spiral_layer_memory_layers_visited() {
        use qops_core::Signature5D;
        
        let mut memory = SpiralLayerMemory::new(5);
        
        // Initially no layers visited
        assert_eq!(memory.layers_visited(), 0);
        
        // Record in layer 0
        let sig = Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5);
        memory.record(0, &sig, 0.8, 0);
        assert_eq!(memory.layers_visited(), 1);
        
        // Record in layer 2 (skipping layer 1)
        memory.record(2, &sig, 0.9, 1);
        assert_eq!(memory.layers_visited(), 2);
        
        // Recording again in the same layer shouldn't increase count
        memory.record(0, &sig, 0.85, 2);
        assert_eq!(memory.layers_visited(), 2);
    }

    #[test]
    fn test_adaptive_triton() {
        let config = AdaptiveTritonConfig {
            base: TritonConfig {
                max_iterations: 50,
                spiral: SpiralParams {
                    layers: 2,
                    points_per_layer: 6,
                    ..Default::default()
                },
                ..Default::default()
            },
            holistic_integration: false, // Disable for speed
            ..Default::default()
        };

        let mut optimizer = AdaptiveTritonOptimizer::new(config);
        let result = optimizer.optimize();

        assert!(result.best_score > 0.0);
        assert!(result.iterations > 0);
    }
}
