//! Main TRITON optimizer implementation.

use crate::config::TritonConfig;
use crate::layer::LayerManager;
use crate::refinement::RefinementEngine;
use crate::scoring::{ScoreCache, ScoringFunction, ResonanceScorer};
use crate::search::{SearchHistory, SearchMetadata, SearchState, SearchStrategy};
use crate::spiral::SpiralEngine;
use crate::temperature::TemperatureController;
use crate::topology_bias::TopologyBias;
use qops_core::Signature5D;
use serde::{Deserialize, Serialize};

/// Result of an optimization run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    /// Best signature found
    pub best_signature: [f64; 5],
    /// Best score achieved
    pub best_score: f64,
    /// Total iterations
    pub iterations: usize,
    /// Total points evaluated
    pub points_evaluated: usize,
    /// Did converge
    pub converged: bool,
    /// Search metadata
    pub metadata: SearchMetadata,
    /// Convergence history
    pub history: Vec<f64>,
    /// Layer-by-layer scores
    pub layer_scores: Vec<f64>,
}

impl OptimizationResult {
    /// Get best signature as Signature5D
    pub fn signature(&self) -> Signature5D {
        Signature5D::new(
            self.best_signature[0],
            self.best_signature[1],
            self.best_signature[2],
            self.best_signature[3],
            self.best_signature[4],
        )
    }
}

/// A single optimization step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStep {
    /// Step number
    pub step: usize,
    /// Current signature
    pub signature: [f64; 5],
    /// Current score
    pub score: f64,
    /// Was accepted
    pub accepted: bool,
    /// Current temperature
    pub temperature: f64,
    /// Current layer
    pub layer: usize,
}

/// Main TRITON optimizer
pub struct TritonOptimizer {
    config: TritonConfig,
    spiral: SpiralEngine,
    temperature: TemperatureController,
    refinement: RefinementEngine,
    layers: LayerManager,
    bias: TopologyBias,
    cache: ScoreCache,
    state: SearchState,
    history: SearchHistory,
    metadata: SearchMetadata,
}

impl TritonOptimizer {
    /// Create new optimizer with default config
    pub fn new(config: TritonConfig) -> Self {
        let spiral = if config.deterministic {
            SpiralEngine::with_seed(config.spiral.clone(), config.seed)
        } else {
            SpiralEngine::new(config.spiral.clone())
        };

        let temperature = if config.deterministic {
            TemperatureController::with_seed(
                config.temperature.clone(),
                config.max_iterations,
                config.seed,
            )
        } else {
            TemperatureController::new(config.temperature.clone(), config.max_iterations)
        };

        let refinement = if config.deterministic {
            RefinementEngine::with_seed(config.refinement.clone(), config.seed)
        } else {
            RefinementEngine::new(config.refinement.clone())
        };

        Self {
            spiral,
            temperature,
            refinement,
            layers: LayerManager::new(),
            bias: TopologyBias::default(),
            cache: ScoreCache::default(),
            state: SearchState {
                max_iterations: config.max_iterations,
                strategy: SearchStrategy::Triton,
                ..Default::default()
            },
            history: SearchHistory::new(),
            metadata: SearchMetadata::new(SearchStrategy::Triton),
            config,
        }
    }

    /// Run optimization with default resonance scoring
    pub fn optimize(&mut self) -> OptimizationResult {
        self.optimize_with_scorer(&ResonanceScorer)
    }

    /// Run optimization with custom scoring function
    pub fn optimize_with_scorer<S: ScoringFunction>(&mut self, scorer: &S) -> OptimizationResult {
        self.metadata.add_param("scorer", scorer.name());
        self.metadata.add_param("layers", self.config.spiral.layers);
        self.metadata.add_param("max_iterations", self.config.max_iterations);

        // Phase 1: Spiral exploration
        self.spiral_phase(scorer);

        // Phase 2: Local refinement
        if let Some(best) = self.state.best_as_signature() {
            let refined = self.refinement.refine_with_scorer(&best, scorer);
            let score = self.score_cached(&refined, scorer);
            self.state.update_best(&refined, score);
        }

        // Phase 3: Final polish with annealing
        self.annealing_phase(scorer);

        self.finalize()
    }

    /// Spiral exploration phase
    fn spiral_phase<S: ScoringFunction>(&mut self, scorer: &S) {
        let mut current_layer = 0;
        self.layers.start_layer(self.config.spiral.initial_radius);

        while let Some(point) = self.spiral.next_point() {
            let score = self.score_cached(&point, scorer);

            // Record in layer
            self.layers.record(point, score);
            self.bias.record_visit(&point, score);

            // Update state
            let improved = self.state.update_best(&point, score);
            self.state.iteration += 1;

            // Record history
            self.history.record(
                score,
                self.state.best_score,
                self.temperature.temperature(),
                improved,
            );

            // Check for layer change
            if self.spiral.state().layer > current_layer {
                current_layer = self.spiral.state().layer;
                self.layers.start_layer(self.spiral.state().radius);
                self.bias.update_progress(self.spiral.progress());
            }

            // Early termination check
            if self.should_terminate() {
                break;
            }
        }
    }

    /// Simulated annealing refinement phase
    fn annealing_phase<S: ScoringFunction>(&mut self, scorer: &S) {
        let mut current = self.state.best_as_signature()
            .unwrap_or_else(|| Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.25));

        let annealing_iterations = self.config.max_iterations / 4;

        for _ in 0..annealing_iterations {
            if self.should_terminate() {
                break;
            }

            // Generate neighbor with temperature-dependent radius
            let _radius = self.temperature.temperature() * 0.2;
            let neighbor = self.spiral.random_perturbation(&current);
            let neighbor_score = self.score_cached(&neighbor, scorer);
            let current_score = self.score_cached(&current, scorer);

            // Accept or reject
            if self.temperature.accept(current_score, neighbor_score) {
                current = neighbor;
                self.state.update_best(&current, neighbor_score);
            }

            self.temperature.advance();
            self.state.iteration += 1;

            self.history.record(
                neighbor_score,
                self.state.best_score,
                self.temperature.temperature(),
                neighbor_score >= current_score,
            );
        }
    }

    /// Score a signature with caching
    fn score_cached<S: ScoringFunction>(&mut self, sig: &Signature5D, scorer: &S) -> f64 {
        self.cache.get_or_compute(sig, |s| scorer.score(s))
    }

    /// Check if optimization should terminate
    fn should_terminate(&self) -> bool {
        if self.state.iteration >= self.config.max_iterations {
            return true;
        }

        // Convergence check
        if self.state.best_score >= 0.99 {
            return true;
        }

        // Stagnation check
        if self.state.is_stagnated(self.config.max_iterations / 10) {
            return true;
        }

        false
    }

    /// Finalize and return result
    fn finalize(&mut self) -> OptimizationResult {
        self.state.complete = true;
        self.state.converged = self.state.best_score >= 0.9 ||
            self.history.convergence_point(0.001).is_some();

        self.metadata.complete(&self.state);
        self.metadata.points_evaluated = self.cache.size();

        let layer_scores = self.layers.convergence_trend();

        OptimizationResult {
            best_signature: self.state.best_signature.unwrap_or([0.5; 5]),
            best_score: self.state.best_score,
            iterations: self.state.iteration,
            points_evaluated: self.metadata.points_evaluated,
            converged: self.state.converged,
            metadata: self.metadata.clone(),
            history: self.history.best_scores.clone(),
            layer_scores,
        }
    }

    /// Get current state
    pub fn state(&self) -> &SearchState {
        &self.state
    }

    /// Get search history
    pub fn history(&self) -> &SearchHistory {
        &self.history
    }

    /// Reset optimizer for reuse
    pub fn reset(&mut self) {
        self.spiral.reset();
        self.temperature.reset();
        self.refinement.reset();
        self.layers = LayerManager::new();
        self.cache.clear();
        self.state = SearchState {
            max_iterations: self.config.max_iterations,
            strategy: SearchStrategy::Triton,
            ..Default::default()
        };
        self.history = SearchHistory::new();
        self.metadata = SearchMetadata::new(SearchStrategy::Triton);
    }

    /// Run a single step (for interactive use)
    pub fn step<S: ScoringFunction>(&mut self, scorer: &S) -> Option<OptimizationStep> {
        if self.state.complete {
            return None;
        }

        let point = self.spiral.next_point()?;
        let score = self.score_cached(&point, scorer);
        let accepted = self.state.update_best(&point, score);

        self.state.iteration += 1;
        self.temperature.advance();

        Some(OptimizationStep {
            step: self.state.iteration,
            signature: [point.psi, point.rho, point.omega, point.chi, point.eta],
            score,
            accepted,
            temperature: self.temperature.temperature(),
            layer: self.spiral.state().layer,
        })
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize, f64) {
        self.cache.stats()
    }
}

/// Builder for TRITON optimizer
pub struct TritonOptimizerBuilder {
    config: TritonConfig,
}

impl TritonOptimizerBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            config: TritonConfig::default(),
        }
    }

    /// Set maximum iterations
    pub fn max_iterations(mut self, n: usize) -> Self {
        self.config.max_iterations = n;
        self
    }

    /// Set number of spiral layers
    pub fn layers(mut self, n: usize) -> Self {
        self.config.spiral.layers = n;
        self
    }

    /// Set initial temperature
    pub fn temperature(mut self, t: f64) -> Self {
        self.config.temperature.initial = t;
        self
    }

    /// Enable deterministic mode
    pub fn deterministic(mut self, seed: u64) -> Self {
        self.config.deterministic = true;
        self.config.seed = seed;
        self
    }

    /// Enable verbose logging
    pub fn verbose(mut self) -> Self {
        self.config.verbose = true;
        self
    }

    /// Build optimizer
    pub fn build(self) -> TritonOptimizer {
        TritonOptimizer::new(self.config)
    }
}

impl Default for TritonOptimizerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let config = TritonConfig::quick();
        let optimizer = TritonOptimizer::new(config);
        assert!(!optimizer.state().complete);
    }

    #[test]
    fn test_optimization() {
        let config = TritonConfig {
            max_iterations: 50,
            spiral: crate::config::SpiralParams {
                layers: 2,
                points_per_layer: 6,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut optimizer = TritonOptimizer::new(config);
        let result = optimizer.optimize();

        assert!(result.best_score > 0.0);
        assert!(result.iterations > 0);
    }

    #[test]
    fn test_builder() {
        let optimizer = TritonOptimizerBuilder::new()
            .max_iterations(100)
            .layers(3)
            .deterministic(42)
            .build();

        assert_eq!(optimizer.config.max_iterations, 100);
        assert!(optimizer.config.deterministic);
    }

    #[test]
    fn test_stepping() {
        let config = TritonConfig::quick();
        let mut optimizer = TritonOptimizer::new(config);

        let step = optimizer.step(&ResonanceScorer);
        assert!(step.is_some());

        let step = step.unwrap();
        assert_eq!(step.step, 1);
    }
}
