//! Mining kernel for operator-space exploration.
//!
//! Per Section 6 of the specification, we define a mining kernel:
//! M = (Q, S, F, R)
//!
//! Where:
//! - Q is the hypercube state space
//! - S is a search procedure on Q and the HDAG G
//! - F is a set of filters and constraints
//! - R is the resonance function

use crate::blueprint::{Blueprint, BlueprintCandidate, BlueprintConstraint, CandidateSource, ConstraintType};
use crate::error::{KernelError, Result};
use crate::operators::{ComposeConfig, ComposeOperator, ExtractConfig, ExtractOperator};
use crate::resonance::{ResonanceEvaluator, ResonanceLevel, ResonanceModel, ResonanceThreshold};
use crate::state::{CoreSignature, State, StateSpace};
use qops_hypercube::{Coord5D, Hypercube, HypercubeConfig, HDAG};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Mining kernel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningConfig {
    /// Maximum iterations
    pub max_iterations: usize,
    /// Target resonance threshold
    pub target_resonance: f64,
    /// Maximum candidates to track
    pub max_candidates: usize,
    /// Exploration/exploitation balance (0.0 = exploit, 1.0 = explore)
    pub exploration_rate: f64,
    /// Search strategy
    pub strategy: SearchStrategy,
    /// Enable parallel mining
    pub parallel: bool,
    /// Convergence threshold
    pub convergence_epsilon: f64,
    /// Stagnation limit
    pub stagnation_limit: usize,
}

impl Default for MiningConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            target_resonance: 0.8,
            max_candidates: 50,
            exploration_rate: 0.3,
            strategy: SearchStrategy::default(),
            parallel: false,
            convergence_epsilon: 1e-4,
            stagnation_limit: 20,
        }
    }
}

/// Search strategy enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchStrategy {
    /// Greedy best-first search
    Greedy,
    /// Stochastic exploration with softmax selection
    Stochastic { temperature: f64 },
    /// Beam search with fixed width
    Beam { width: usize },
    /// Evolutionary search
    Evolutionary { population_size: usize, mutation_rate: f64 },
    /// TRITON spiral search (uses existing TRITON optimizer)
    Triton,
    /// Hybrid approach
    Hybrid,
}

impl Default for SearchStrategy {
    fn default() -> Self {
        SearchStrategy::Stochastic { temperature: 1.0 }
    }
}

/// Filter set for constraint enforcement
#[derive(Debug, Clone, Default)]
pub struct FilterSet {
    /// Constraints to apply
    pub constraints: Vec<BlueprintConstraint>,
    /// Minimum resonance filter
    pub min_resonance: Option<f64>,
    /// Maximum resonance filter (for diversity)
    pub max_resonance: Option<f64>,
    /// Stability bounds
    pub stability_bounds: Option<(f64, f64)>,
    /// Custom filter functions (by name)
    pub custom_filters: Vec<String>,
}

impl FilterSet {
    /// Create a new filter set
    pub fn new() -> Self {
        Self::default()
    }

    /// Add minimum resonance filter
    pub fn with_min_resonance(mut self, min: f64) -> Self {
        self.min_resonance = Some(min);
        self.constraints.push(BlueprintConstraint {
            name: "min_resonance".to_string(),
            constraint_type: ConstraintType::MinResonance(min),
            hard: true,
            weight: 1.0,
        });
        self
    }

    /// Add stability bounds
    pub fn with_stability_bounds(mut self, min: f64, max: f64) -> Self {
        self.stability_bounds = Some((min, max));
        self.constraints.push(BlueprintConstraint {
            name: "stability_bounds".to_string(),
            constraint_type: ConstraintType::DimensionBounds {
                dimension: 1, // rho
                min,
                max,
            },
            hard: true,
            weight: 1.0,
        });
        self
    }

    /// Check if a state passes all filters
    pub fn passes(&self, state: &State) -> bool {
        let sig = state.to_core();
        let res = sig.simple_resonance();

        // Check resonance bounds
        if let Some(min) = self.min_resonance {
            if res < min {
                return false;
            }
        }
        if let Some(max) = self.max_resonance {
            if res > max {
                return false;
            }
        }

        // Check stability bounds
        if let Some((min, max)) = self.stability_bounds {
            if sig.rho < min || sig.rho > max {
                return false;
            }
        }

        true
    }
}

/// Mining result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningResult {
    /// Discovered candidates (ranked)
    pub candidates: Vec<BlueprintCandidate>,
    /// Total iterations performed
    pub iterations: usize,
    /// Final best resonance
    pub best_resonance: f64,
    /// Mining statistics
    pub stats: MiningStats,
    /// Converged
    pub converged: bool,
}

/// Mining statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MiningStats {
    /// Total candidates evaluated
    pub total_evaluated: usize,
    /// Candidates that passed filters
    pub passed_filters: usize,
    /// Candidates merged
    pub merged_count: usize,
    /// Average resonance at each iteration
    pub resonance_history: Vec<f64>,
    /// Stagnation count
    pub stagnation_count: usize,
    /// Time elapsed (ms)
    pub elapsed_ms: u64,
}

/// Mining kernel: M = (Q, S, F, R)
pub struct MiningKernel {
    /// Configuration
    pub config: MiningConfig,
    /// State space Q
    pub state_space: StateSpace,
    /// Filter set F
    pub filters: FilterSet,
    /// Resonance evaluator R
    pub resonance: ResonanceEvaluator,
    /// Random number generator
    rng: StdRng,
}

impl MiningKernel {
    /// Create a new mining kernel
    pub fn new(config: MiningConfig) -> Self {
        Self {
            config,
            state_space: StateSpace::default_5d(),
            filters: FilterSet::new(),
            resonance: ResonanceEvaluator::default(),
            rng: StdRng::from_entropy(),
        }
    }

    /// Set filter set
    pub fn with_filters(mut self, filters: FilterSet) -> Self {
        self.filters = filters;
        self
    }

    /// Set resonance model
    pub fn with_resonance(mut self, model: ResonanceModel, thresholds: ResonanceThreshold) -> Self {
        self.resonance = ResonanceEvaluator::new(model, thresholds);
        self
    }

    /// Set seed for reproducibility
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.rng = StdRng::seed_from_u64(seed);
        self
    }

    /// Run the mining kernel
    pub fn mine(&mut self, seeds: &[State]) -> Result<MiningResult> {
        let start_time = std::time::Instant::now();

        // Initialize frontier with seed states
        let mut frontier: Vec<BlueprintCandidate> = seeds
            .iter()
            .enumerate()
            .map(|(i, s)| {
                BlueprintCandidate::new(
                    Blueprint::from_state(&format!("seed_{}", i), s.clone()),
                    0,
                    CandidateSource::Seed,
                )
            })
            .collect();

        let mut best_candidates: Vec<BlueprintCandidate> = Vec::new();
        let mut stats = MiningStats::default();
        let mut last_best_resonance = 0.0;
        let mut stagnation_count = 0;

        for iteration in 0..self.config.max_iterations {
            // Step 1: Expand frontier based on strategy
            let expanded = self.expand_frontier(&frontier, iteration);
            stats.total_evaluated += expanded.len();

            // Step 2: Evaluate and filter
            let filtered: Vec<BlueprintCandidate> = expanded
                .into_iter()
                .filter(|c| self.filters.passes(&c.blueprint.state))
                .collect();
            stats.passed_filters += filtered.len();

            // Step 3: Update best candidates
            best_candidates.extend(filtered.clone());
            best_candidates.sort_by(|a, b| b.resonance_score.partial_cmp(&a.resonance_score).unwrap());
            best_candidates.truncate(self.config.max_candidates);

            // Step 4: Check for convergence
            let current_best = best_candidates
                .first()
                .map(|c| c.resonance_score)
                .unwrap_or(0.0);
            stats.resonance_history.push(current_best);

            if (current_best - last_best_resonance).abs() < self.config.convergence_epsilon {
                stagnation_count += 1;
                stats.stagnation_count = stagnation_count;
            } else {
                stagnation_count = 0;
            }
            last_best_resonance = current_best;

            // Check termination conditions
            if current_best >= self.config.target_resonance {
                stats.elapsed_ms = start_time.elapsed().as_millis() as u64;
                return Ok(MiningResult {
                    candidates: best_candidates,
                    iterations: iteration + 1,
                    best_resonance: current_best,
                    stats,
                    converged: true,
                });
            }

            if stagnation_count >= self.config.stagnation_limit {
                stats.elapsed_ms = start_time.elapsed().as_millis() as u64;
                return Ok(MiningResult {
                    candidates: best_candidates,
                    iterations: iteration + 1,
                    best_resonance: current_best,
                    stats,
                    converged: false,
                });
            }

            // Step 5: Update frontier for next iteration
            frontier = self.select_frontier(&best_candidates, &filtered);
        }

        stats.elapsed_ms = start_time.elapsed().as_millis() as u64;
        let best_resonance = best_candidates
            .first()
            .map(|c| c.resonance_score)
            .unwrap_or(0.0);

        Ok(MiningResult {
            candidates: best_candidates,
            iterations: self.config.max_iterations,
            best_resonance,
            stats,
            converged: false,
        })
    }

    /// Expand the frontier based on search strategy
    fn expand_frontier(&mut self, frontier: &[BlueprintCandidate], generation: usize) -> Vec<BlueprintCandidate> {
        match &self.config.strategy {
            SearchStrategy::Greedy => self.expand_greedy(frontier, generation),
            SearchStrategy::Stochastic { temperature } => {
                self.expand_stochastic(frontier, generation, *temperature)
            }
            SearchStrategy::Beam { width } => self.expand_beam(frontier, generation, *width),
            SearchStrategy::Evolutionary {
                population_size,
                mutation_rate,
            } => self.expand_evolutionary(frontier, generation, *population_size, *mutation_rate),
            SearchStrategy::Triton => self.expand_triton(frontier, generation),
            SearchStrategy::Hybrid => self.expand_hybrid(frontier, generation),
        }
    }

    /// Greedy expansion
    fn expand_greedy(&mut self, frontier: &[BlueprintCandidate], generation: usize) -> Vec<BlueprintCandidate> {
        let mut expanded = Vec::new();

        // Take top candidates and generate neighbors
        for candidate in frontier.iter().take(10) {
            let neighbors = self.generate_neighbors(&candidate.blueprint.state, generation);
            expanded.extend(neighbors);
        }

        expanded
    }

    /// Stochastic expansion with temperature-based selection
    fn expand_stochastic(
        &mut self,
        frontier: &[BlueprintCandidate],
        generation: usize,
        temperature: f64,
    ) -> Vec<BlueprintCandidate> {
        let mut expanded = Vec::new();

        // Softmax selection
        let weights: Vec<f64> = frontier
            .iter()
            .map(|c| (c.resonance_score / temperature).exp())
            .collect();
        let total: f64 = weights.iter().sum();

        // Sample based on weights
        let num_samples = (frontier.len() / 2).max(5).min(20);
        for _ in 0..num_samples {
            let r: f64 = self.rng.gen::<f64>() * total;
            let mut cumsum = 0.0;

            for (i, &w) in weights.iter().enumerate() {
                cumsum += w;
                if cumsum >= r {
                    let neighbors = self.generate_neighbors(&frontier[i].blueprint.state, generation);
                    expanded.extend(neighbors);
                    break;
                }
            }
        }

        // Add some random exploration
        if self.rng.gen::<f64>() < self.config.exploration_rate {
            let random_state = self.generate_random_state();
            expanded.push(BlueprintCandidate::new(
                Blueprint::from_state("random", random_state),
                generation,
                CandidateSource::Random,
            ));
        }

        expanded
    }

    /// Beam search expansion
    fn expand_beam(
        &mut self,
        frontier: &[BlueprintCandidate],
        generation: usize,
        width: usize,
    ) -> Vec<BlueprintCandidate> {
        let mut expanded = Vec::new();

        // Take top `width` candidates
        for candidate in frontier.iter().take(width) {
            let neighbors = self.generate_neighbors(&candidate.blueprint.state, generation);
            expanded.extend(neighbors);
        }

        expanded
    }

    /// Evolutionary expansion
    fn expand_evolutionary(
        &mut self,
        frontier: &[BlueprintCandidate],
        generation: usize,
        population_size: usize,
        mutation_rate: f64,
    ) -> Vec<BlueprintCandidate> {
        let mut expanded = Vec::new();

        // Selection and mutation
        let elite_count = (population_size / 4).max(2);
        for candidate in frontier.iter().take(elite_count) {
            // Keep elites
            expanded.push(candidate.clone());

            // Generate mutations
            for _ in 0..3 {
                if self.rng.gen::<f64>() < mutation_rate {
                    let mutant = self.mutate(&candidate.blueprint.state, generation);
                    expanded.push(mutant);
                }
            }
        }

        // Crossover
        if frontier.len() >= 2 {
            for _ in 0..population_size / 2 {
                let i = self.rng.gen_range(0..frontier.len());
                let j = self.rng.gen_range(0..frontier.len());
                if i != j {
                    let child = self.crossover(
                        &frontier[i].blueprint.state,
                        &frontier[j].blueprint.state,
                        generation,
                    );
                    expanded.push(child);
                }
            }
        }

        expanded
    }

    /// TRITON-style spiral expansion
    fn expand_triton(&mut self, frontier: &[BlueprintCandidate], generation: usize) -> Vec<BlueprintCandidate> {
        let mut expanded = Vec::new();

        // Spiral expansion around best candidates
        for candidate in frontier.iter().take(5) {
            let sig = candidate.blueprint.signature();
            let delta = 0.1 / (generation as f64 + 1.0).sqrt(); // Decreasing step size

            // Spiral pattern in 5D
            for angle in 0..8 {
                let theta = (angle as f64) * std::f64::consts::PI / 4.0;
                let new_sig = CoreSignature::new(
                    sig.psi + delta * theta.cos(),
                    sig.rho + delta * theta.sin(),
                    sig.omega + delta * (theta * 2.0).cos(),
                    sig.chi + delta * 0.5,
                    sig.eta - delta * 0.5,
                );

                expanded.push(BlueprintCandidate::new(
                    Blueprint::from_signature(&format!("triton_{}_{}", generation, angle), new_sig),
                    generation,
                    CandidateSource::HdagExpansion {
                        node_id: candidate.blueprint.id.clone(),
                    },
                ));
            }
        }

        expanded
    }

    /// Hybrid expansion
    fn expand_hybrid(&mut self, frontier: &[BlueprintCandidate], generation: usize) -> Vec<BlueprintCandidate> {
        let mut expanded = Vec::new();

        // Mix strategies based on generation
        if generation % 3 == 0 {
            expanded.extend(self.expand_triton(frontier, generation));
        } else if generation % 3 == 1 {
            expanded.extend(self.expand_stochastic(frontier, generation, 1.0));
        } else {
            expanded.extend(self.expand_evolutionary(frontier, generation, 20, 0.3));
        }

        expanded
    }

    /// Generate neighbors of a state
    fn generate_neighbors(&mut self, state: &State, generation: usize) -> Vec<BlueprintCandidate> {
        let sig = state.to_core();
        let delta = 0.05;
        let mut neighbors = Vec::new();

        // Generate neighbors along each dimension
        let perturbations = [
            ("psi+", CoreSignature::new(sig.psi + delta, sig.rho, sig.omega, sig.chi, sig.eta)),
            ("psi-", CoreSignature::new(sig.psi - delta, sig.rho, sig.omega, sig.chi, sig.eta)),
            ("rho+", CoreSignature::new(sig.psi, sig.rho + delta, sig.omega, sig.chi, sig.eta)),
            ("rho-", CoreSignature::new(sig.psi, sig.rho - delta, sig.omega, sig.chi, sig.eta)),
            ("omega+", CoreSignature::new(sig.psi, sig.rho, sig.omega + delta, sig.chi, sig.eta)),
            ("omega-", CoreSignature::new(sig.psi, sig.rho, sig.omega - delta, sig.chi, sig.eta)),
        ];

        for (name, new_sig) in perturbations {
            neighbors.push(BlueprintCandidate::new(
                Blueprint::from_signature(&format!("neighbor_{}_{}", generation, name), new_sig),
                generation,
                CandidateSource::Mutated {
                    parent_id: "parent".to_string(),
                },
            ));
        }

        neighbors
    }

    /// Mutate a state
    fn mutate(&mut self, state: &State, generation: usize) -> BlueprintCandidate {
        let sig = state.to_core();
        let mutation_strength = 0.1;

        let new_sig = CoreSignature::new(
            sig.psi + self.rng.gen_range(-mutation_strength..mutation_strength),
            sig.rho + self.rng.gen_range(-mutation_strength..mutation_strength),
            sig.omega + self.rng.gen_range(-mutation_strength..mutation_strength),
            sig.chi + self.rng.gen_range(-mutation_strength..mutation_strength),
            sig.eta + self.rng.gen_range(-mutation_strength..mutation_strength),
        );

        BlueprintCandidate::new(
            Blueprint::from_signature(&format!("mutant_{}", generation), new_sig),
            generation,
            CandidateSource::Mutated {
                parent_id: "mutant_parent".to_string(),
            },
        )
    }

    /// Crossover two states
    fn crossover(&mut self, state1: &State, state2: &State, generation: usize) -> BlueprintCandidate {
        let sig1 = state1.to_core();
        let sig2 = state2.to_core();
        let t: f64 = self.rng.gen();

        let child_sig = sig1.lerp(&sig2, t);

        BlueprintCandidate::new(
            Blueprint::from_signature(&format!("crossover_{}", generation), child_sig),
            generation,
            CandidateSource::Composed {
                parent_ids: vec!["parent1".to_string(), "parent2".to_string()],
            },
        )
    }

    /// Generate a random state
    fn generate_random_state(&mut self) -> State {
        let sig = CoreSignature::new(
            self.rng.gen(),
            self.rng.gen(),
            self.rng.gen(),
            self.rng.gen(),
            self.rng.gen(),
        );
        State::Core(sig)
    }

    /// Select frontier for next iteration
    fn select_frontier(
        &mut self,
        best: &[BlueprintCandidate],
        recent: &[BlueprintCandidate],
    ) -> Vec<BlueprintCandidate> {
        let mut frontier = Vec::new();

        // Include best candidates
        frontier.extend(best.iter().take(self.config.max_candidates / 2).cloned());

        // Include some recent candidates for diversity
        for c in recent.iter().take(self.config.max_candidates / 2) {
            if !frontier.iter().any(|f| f.blueprint.id == c.blueprint.id) {
                frontier.push(c.clone());
            }
        }

        frontier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mining_kernel_creation() {
        let config = MiningConfig::default();
        let kernel = MiningKernel::new(config);
        assert_eq!(kernel.state_space.total_dimension(), 5);
    }

    #[test]
    fn test_filter_set() {
        let filters = FilterSet::new()
            .with_min_resonance(0.3)
            .with_stability_bounds(0.2, 0.9);

        let good_state = State::Core(CoreSignature::new(0.8, 0.5, 0.7, 0.5, 0.3));
        let bad_state = State::Core(CoreSignature::new(0.1, 0.1, 0.1, 0.5, 0.5));

        assert!(filters.passes(&good_state));
        assert!(!filters.passes(&bad_state));
    }

    #[test]
    fn test_mining_basic() {
        let config = MiningConfig {
            max_iterations: 10,
            target_resonance: 0.5,
            max_candidates: 10,
            ..Default::default()
        };

        let mut kernel = MiningKernel::new(config).with_seed(42);

        let seeds = vec![State::Core(CoreSignature::center())];
        let result = kernel.mine(&seeds).unwrap();

        assert!(!result.candidates.is_empty());
        assert!(result.iterations <= 10);
    }

    #[test]
    fn test_search_strategies() {
        let strategies = vec![
            SearchStrategy::Greedy,
            SearchStrategy::Stochastic { temperature: 1.0 },
            SearchStrategy::Beam { width: 5 },
            SearchStrategy::Triton,
        ];

        for strategy in strategies {
            let config = MiningConfig {
                max_iterations: 5,
                strategy,
                ..Default::default()
            };

            let mut kernel = MiningKernel::new(config).with_seed(42);
            let seeds = vec![State::Core(CoreSignature::center())];
            let result = kernel.mine(&seeds).unwrap();

            assert!(!result.candidates.is_empty());
        }
    }
}
