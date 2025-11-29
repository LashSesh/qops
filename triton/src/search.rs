//! Search strategy definitions and state management.

use qops_core::Signature5D;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Search strategy enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchStrategy {
    /// Pure spiral search
    Spiral,
    /// Simulated annealing
    Annealing,
    /// Genetic/evolutionary search
    Evolutionary,
    /// Hybrid spiral + annealing
    HybridSpiralAnnealing,
    /// Multi-start local search
    MultiStart,
    /// Particle swarm optimization
    ParticleSwarm,
    /// TRITON default (spiral + annealing + refinement)
    Triton,
}

impl Default for SearchStrategy {
    fn default() -> Self {
        Self::Triton
    }
}

impl SearchStrategy {
    /// Get human-readable name
    pub fn name(&self) -> &str {
        match self {
            Self::Spiral => "Spiral Search",
            Self::Annealing => "Simulated Annealing",
            Self::Evolutionary => "Evolutionary Search",
            Self::HybridSpiralAnnealing => "Hybrid Spiral-Annealing",
            Self::MultiStart => "Multi-Start Local Search",
            Self::ParticleSwarm => "Particle Swarm",
            Self::Triton => "TRITON Spiral Optimizer",
        }
    }
}

/// Current state of the search process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchState {
    /// Current iteration
    pub iteration: usize,
    /// Total iterations planned
    pub max_iterations: usize,
    /// Current best signature
    pub best_signature: Option<[f64; 5]>,
    /// Current best score
    pub best_score: f64,
    /// Current temperature (if annealing)
    pub temperature: f64,
    /// Current search radius
    pub radius: f64,
    /// Strategy being used
    pub strategy: SearchStrategy,
    /// Is search complete
    pub complete: bool,
    /// Did search converge
    pub converged: bool,
    /// Stagnation counter
    pub stagnation: usize,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            iteration: 0,
            max_iterations: 1000,
            best_signature: None,
            best_score: 0.0,
            temperature: 1.0,
            radius: 0.1,
            strategy: SearchStrategy::Triton,
            complete: false,
            converged: false,
            stagnation: 0,
        }
    }
}

impl SearchState {
    /// Update best if improved
    pub fn update_best(&mut self, sig: &Signature5D, score: f64) -> bool {
        if score > self.best_score {
            self.best_signature = Some([sig.psi, sig.rho, sig.omega, sig.chi, sig.eta]);
            self.best_score = score;
            self.stagnation = 0;
            true
        } else {
            self.stagnation += 1;
            false
        }
    }

    /// Get progress as percentage
    pub fn progress(&self) -> f64 {
        self.iteration as f64 / self.max_iterations as f64
    }

    /// Check if stagnated
    pub fn is_stagnated(&self, threshold: usize) -> bool {
        self.stagnation >= threshold
    }

    /// Get best signature as Signature5D
    pub fn best_as_signature(&self) -> Option<Signature5D> {
        self.best_signature.map(|[psi, rho, omega, chi, eta]| {
            Signature5D::new(psi, rho, omega, chi, eta)
        })
    }
}

/// Metadata about the search process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMetadata {
    /// Search ID
    pub id: String,
    /// Strategy used
    pub strategy: SearchStrategy,
    /// Start time
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// End time
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Duration in milliseconds
    pub duration_ms: Option<u64>,
    /// Total iterations
    pub total_iterations: usize,
    /// Total points evaluated
    pub points_evaluated: usize,
    /// Final best score
    pub final_score: f64,
    /// Did converge
    pub converged: bool,
    /// Convergence iteration
    pub convergence_iteration: Option<usize>,
    /// Parameters used
    pub parameters: HashMap<String, String>,
}

impl SearchMetadata {
    /// Create new metadata
    pub fn new(strategy: SearchStrategy) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            strategy,
            start_time: chrono::Utc::now(),
            end_time: None,
            duration_ms: None,
            total_iterations: 0,
            points_evaluated: 0,
            final_score: 0.0,
            converged: false,
            convergence_iteration: None,
            parameters: HashMap::new(),
        }
    }

    /// Mark search as complete
    pub fn complete(&mut self, state: &SearchState) {
        self.end_time = Some(chrono::Utc::now());
        self.duration_ms = Some(
            (self.end_time.unwrap() - self.start_time).num_milliseconds() as u64
        );
        self.total_iterations = state.iteration;
        self.final_score = state.best_score;
        self.converged = state.converged;
    }

    /// Add parameter
    pub fn add_param(&mut self, key: &str, value: impl ToString) {
        self.parameters.insert(key.to_string(), value.to_string());
    }
}

/// Search history for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistory {
    /// Score history
    pub scores: Vec<f64>,
    /// Best score at each iteration
    pub best_scores: Vec<f64>,
    /// Temperature history (if annealing)
    pub temperatures: Vec<f64>,
    /// Acceptance history
    pub acceptances: Vec<bool>,
}

impl SearchHistory {
    /// Create new history
    pub fn new() -> Self {
        Self {
            scores: Vec::new(),
            best_scores: Vec::new(),
            temperatures: Vec::new(),
            acceptances: Vec::new(),
        }
    }

    /// Record a step
    pub fn record(&mut self, score: f64, best: f64, temp: f64, accepted: bool) {
        self.scores.push(score);
        self.best_scores.push(best);
        self.temperatures.push(temp);
        self.acceptances.push(accepted);
    }

    /// Get moving average of scores
    pub fn moving_average(&self, window: usize) -> Vec<f64> {
        if self.scores.len() < window {
            return self.scores.clone();
        }

        self.scores.windows(window)
            .map(|w| w.iter().sum::<f64>() / w.len() as f64)
            .collect()
    }

    /// Get acceptance rate over time
    pub fn acceptance_rate_over_time(&self, window: usize) -> Vec<f64> {
        if self.acceptances.len() < window {
            return vec![];
        }

        self.acceptances.windows(window)
            .map(|w| w.iter().filter(|&&x| x).count() as f64 / w.len() as f64)
            .collect()
    }

    /// Detect convergence point
    pub fn convergence_point(&self, tolerance: f64) -> Option<usize> {
        if self.best_scores.len() < 10 {
            return None;
        }

        for i in 10..self.best_scores.len() {
            let window = &self.best_scores[i - 10..i];
            let spread = window.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
                - window.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

            if spread < tolerance {
                return Some(i);
            }
        }

        None
    }
}

impl Default for SearchHistory {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for multi-start search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiStartConfig {
    /// Number of starting points
    pub num_starts: usize,
    /// Iterations per start
    pub iterations_per_start: usize,
    /// Starting point generation method
    pub start_method: StartMethod,
    /// Diversity requirement between starts
    pub min_diversity: f64,
}

impl Default for MultiStartConfig {
    fn default() -> Self {
        Self {
            num_starts: 10,
            iterations_per_start: 100,
            start_method: StartMethod::LatinHypercube,
            min_diversity: 0.2,
        }
    }
}

/// Method for generating starting points
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StartMethod {
    /// Uniform random
    Random,
    /// Latin hypercube sampling
    LatinHypercube,
    /// Grid-based
    Grid,
    /// Sobol sequence
    Sobol,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_state() {
        let mut state = SearchState::default();
        let sig = Signature5D::new(0.8, 0.8, 0.8, 0.5, 0.2);

        assert!(state.update_best(&sig, 0.8));
        assert_eq!(state.best_score, 0.8);
        assert!(!state.update_best(&sig, 0.7));
        assert_eq!(state.stagnation, 1);
    }

    #[test]
    fn test_search_metadata() {
        let mut meta = SearchMetadata::new(SearchStrategy::Triton);
        meta.add_param("layers", 7);
        meta.add_param("temperature", 1.0);

        assert!(!meta.id.is_empty());
        assert_eq!(meta.strategy, SearchStrategy::Triton);
    }

    #[test]
    fn test_search_history() {
        let mut history = SearchHistory::new();

        for i in 0..20 {
            history.record(0.5 + i as f64 * 0.01, 0.5 + i as f64 * 0.01, 1.0 - i as f64 * 0.05, true);
        }

        let avg = history.moving_average(5);
        assert_eq!(avg.len(), 16);

        let rates = history.acceptance_rate_over_time(10);
        assert!(!rates.is_empty());
    }
}
