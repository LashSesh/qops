//! Sequence Miner
//!
//! Mines optimal operator sequences from slot configurations.

use crate::lattice::{SlotLattice, LatticeConfig, LatticeResult};
use crate::entropy::{EntropyMapper, EntropyConfig, EntropyDistribution};
use crate::slot::SlotSymbol;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use qops_hypercube::coordinates::Coord5D;

/// Miner configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerConfig {
    /// Lattice configuration
    pub lattice_config: LatticeConfig,
    /// Entropy configuration
    pub entropy_config: EntropyConfig,
    /// Mining depth (iterations)
    pub depth: usize,
    /// Target resonance
    pub target_resonance: f64,
    /// Maximum attempts per step
    pub max_attempts: usize,
    /// Keep top N sequences
    pub keep_top: usize,
    /// Mining strategy
    pub strategy: MiningStrategy,
}

/// Mining strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MiningStrategy {
    /// Greedy (always take best)
    Greedy,
    /// Stochastic (probabilistic selection)
    Stochastic,
    /// Beam search (keep multiple candidates)
    BeamSearch,
    /// Evolutionary (mutation + selection)
    Evolutionary,
    /// TRITON-inspired spiral
    Triton,
}

impl Default for MinerConfig {
    fn default() -> Self {
        Self {
            lattice_config: LatticeConfig::default(),
            entropy_config: EntropyConfig::default(),
            depth: 50,
            target_resonance: 0.8,
            max_attempts: 10,
            keep_top: 5,
            strategy: MiningStrategy::BeamSearch,
        }
    }
}

impl MinerConfig {
    /// Quick mining config
    pub fn quick() -> Self {
        Self {
            depth: 20,
            max_attempts: 5,
            keep_top: 3,
            strategy: MiningStrategy::Greedy,
            ..Default::default()
        }
    }

    /// Thorough mining config
    pub fn thorough() -> Self {
        Self {
            depth: 100,
            max_attempts: 20,
            keep_top: 10,
            strategy: MiningStrategy::Evolutionary,
            entropy_config: EntropyConfig::resonance_optimized(),
            ..Default::default()
        }
    }
}

/// A mined sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinedSequence {
    /// Sequence ID
    pub id: String,
    /// Symbol sequence
    pub symbols: Vec<SlotSymbol>,
    /// Values for each position
    pub values: Vec<f64>,
    /// Final resonance
    pub resonance: f64,
    /// Mining step at which this was found
    pub found_at_step: usize,
    /// Total score
    pub score: f64,
    /// 5D coordinate representation
    pub coord5d: [f64; 5],
    /// Generation (for evolutionary)
    pub generation: usize,
}

impl MinedSequence {
    /// Create new mined sequence
    pub fn new(symbols: Vec<SlotSymbol>, values: Vec<f64>, step: usize) -> Self {
        let resonance = Self::compute_resonance(&symbols, &values);
        let score = Self::compute_score(&symbols, &values);
        let coord5d = Self::to_coord5d(&symbols, &values);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            symbols,
            values,
            resonance,
            found_at_step: step,
            score,
            coord5d,
            generation: 0,
        }
    }

    /// Compute resonance from symbols and values
    fn compute_resonance(symbols: &[SlotSymbol], values: &[f64]) -> f64 {
        if symbols.is_empty() {
            return 0.0;
        }

        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for (symbol, &value) in symbols.iter().zip(values.iter()) {
            let weight = symbol.base_value().abs();
            weighted_sum += weight * value;
            total_weight += weight;
        }

        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.5
        }
    }

    /// Compute score
    fn compute_score(symbols: &[SlotSymbol], values: &[f64]) -> f64 {
        let mut score = 0.0;

        for (symbol, &value) in symbols.iter().zip(values.iter()) {
            score += symbol.base_value() * value;
        }

        // Bonus for matching symbols
        let unique_symbols: std::collections::HashSet<_> = symbols.iter().collect();
        if unique_symbols.len() < symbols.len() {
            score *= 1.0 + (symbols.len() - unique_symbols.len()) as f64 * 0.1;
        }

        score
    }

    /// Convert to 5D coordinate
    fn to_coord5d(symbols: &[SlotSymbol], values: &[f64]) -> [f64; 5] {
        let mut sums = [0.0; 5];
        let mut counts = [0; 5];

        for (symbol, &value) in symbols.iter().zip(values.iter()) {
            if let Some(dim) = symbol.to_dimension() {
                sums[dim] += value;
                counts[dim] += 1;
            }
        }

        for i in 0..5 {
            if counts[i] > 0 {
                sums[i] /= counts[i] as f64;
            } else {
                sums[i] = 0.5;
            }
        }

        sums
    }

    /// Get as Coord5D
    pub fn to_hypercube_coord(&self) -> Coord5D {
        Coord5D::from_vec(&self.coord5d)
    }
}

/// Mining result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningResult {
    /// Best sequence found
    pub best_sequence: MinedSequence,
    /// Top sequences
    pub top_sequences: Vec<MinedSequence>,
    /// Total steps performed
    pub total_steps: usize,
    /// Steps until best found
    pub steps_to_best: usize,
    /// Best resonance achieved
    pub best_resonance: f64,
    /// Mining time (ms)
    pub mining_time_ms: u64,
    /// Convergence achieved
    pub converged: bool,
    /// Strategy used
    pub strategy: MiningStrategy,
}

/// Sequence miner
pub struct SequenceMiner {
    config: MinerConfig,
    lattice: SlotLattice,
    entropy_mapper: EntropyMapper,
    candidates: Vec<MinedSequence>,
    best: Option<MinedSequence>,
    current_step: usize,
    current_generation: usize,
}

impl SequenceMiner {
    /// Create a new miner
    pub fn new(config: MinerConfig) -> Self {
        let lattice = SlotLattice::new(config.lattice_config.clone());
        let entropy_mapper = EntropyMapper::new(config.entropy_config.clone());

        Self {
            config,
            lattice,
            entropy_mapper,
            candidates: Vec::new(),
            best: None,
            current_step: 0,
            current_generation: 0,
        }
    }

    /// Create with default config
    pub fn default_miner() -> Self {
        Self::new(MinerConfig::default())
    }

    /// Mine sequences
    pub fn mine(&mut self) -> Result<MiningResult> {
        let start = std::time::Instant::now();
        self.candidates.clear();
        self.best = None;
        self.current_step = 0;
        self.current_generation = 0;

        match self.config.strategy {
            MiningStrategy::Greedy => self.mine_greedy()?,
            MiningStrategy::Stochastic => self.mine_stochastic()?,
            MiningStrategy::BeamSearch => self.mine_beam_search()?,
            MiningStrategy::Evolutionary => self.mine_evolutionary()?,
            MiningStrategy::Triton => self.mine_triton()?,
        }

        let mining_time = start.elapsed().as_millis() as u64;
        let best = self.best.clone().unwrap_or_else(|| self.generate_initial_sequence());
        let steps_to_best = best.found_at_step;

        // Sort candidates
        self.candidates.sort_by(|a, b| b.resonance.partial_cmp(&a.resonance).unwrap());
        let top_sequences: Vec<_> = self.candidates.iter().take(self.config.keep_top).cloned().collect();

        Ok(MiningResult {
            best_sequence: best.clone(),
            top_sequences,
            total_steps: self.current_step,
            steps_to_best,
            best_resonance: best.resonance,
            mining_time_ms: mining_time,
            converged: best.resonance >= self.config.target_resonance,
            strategy: self.config.strategy,
        })
    }

    /// Greedy mining
    fn mine_greedy(&mut self) -> Result<()> {
        for step in 0..self.config.depth {
            self.current_step = step;
            let seq = self.generate_sequence();

            if self.best.is_none() || seq.resonance > self.best.as_ref().unwrap().resonance {
                self.best = Some(seq.clone());
            }
            self.candidates.push(seq);

            if self.best.as_ref().unwrap().resonance >= self.config.target_resonance {
                break;
            }
        }
        Ok(())
    }

    /// Stochastic mining
    fn mine_stochastic(&mut self) -> Result<()> {
        let mut temperature = 1.0;
        let cooling_rate = 0.95;

        for step in 0..self.config.depth {
            self.current_step = step;

            for _ in 0..self.config.max_attempts {
                let seq = self.generate_sequence();

                let accept = if self.best.is_none() {
                    true
                } else {
                    let delta = seq.resonance - self.best.as_ref().unwrap().resonance;
                    if delta > 0.0 {
                        true
                    } else {
                        let prob = (delta / temperature).exp();
                        rand::random::<f64>() < prob
                    }
                };

                if accept {
                    if self.best.is_none() || seq.resonance > self.best.as_ref().unwrap().resonance {
                        self.best = Some(seq.clone());
                    }
                    self.candidates.push(seq);
                }
            }

            temperature *= cooling_rate;

            if self.best.as_ref().map(|b| b.resonance >= self.config.target_resonance).unwrap_or(false) {
                break;
            }
        }
        Ok(())
    }

    /// Beam search mining
    fn mine_beam_search(&mut self) -> Result<()> {
        // Initialize beam
        let mut beam: Vec<MinedSequence> = (0..self.config.keep_top)
            .map(|_| self.generate_sequence())
            .collect();

        for step in 0..self.config.depth {
            self.current_step = step;

            // Expand each beam candidate
            let mut new_candidates = Vec::new();
            for candidate in &beam {
                for _ in 0..self.config.max_attempts {
                    let mut seq = self.generate_sequence();
                    // Bias towards candidate's good dimensions
                    for i in 0..5 {
                        if candidate.coord5d[i] > 0.6 {
                            let idx = i % seq.values.len();
                            let old_val = seq.values[idx];
                            seq.values[idx] = (old_val + candidate.coord5d[i]) / 2.0;
                        }
                    }
                    seq.resonance = MinedSequence::compute_resonance(&seq.symbols, &seq.values);
                    seq.coord5d = MinedSequence::to_coord5d(&seq.symbols, &seq.values);
                    new_candidates.push(seq);
                }
            }

            // Keep best
            new_candidates.sort_by(|a, b| b.resonance.partial_cmp(&a.resonance).unwrap());
            beam = new_candidates.into_iter().take(self.config.keep_top).collect();

            if let Some(best_candidate) = beam.first() {
                if self.best.is_none() || best_candidate.resonance > self.best.as_ref().unwrap().resonance {
                    self.best = Some(best_candidate.clone());
                }
            }

            self.candidates.extend(beam.clone());

            if self.best.as_ref().map(|b| b.resonance >= self.config.target_resonance).unwrap_or(false) {
                break;
            }
        }
        Ok(())
    }

    /// Evolutionary mining
    fn mine_evolutionary(&mut self) -> Result<()> {
        let population_size = self.config.keep_top * 2;
        let mutation_rate = 0.2;

        // Initialize population
        let mut population: Vec<MinedSequence> = (0..population_size)
            .map(|_| self.generate_sequence())
            .collect();

        for generation in 0..(self.config.depth / 5).max(1) {
            self.current_generation = generation;

            // Sort by fitness
            population.sort_by(|a, b| b.resonance.partial_cmp(&a.resonance).unwrap());

            // Update best
            if let Some(best_candidate) = population.first() {
                if self.best.is_none() || best_candidate.resonance > self.best.as_ref().unwrap().resonance {
                    self.best = Some(best_candidate.clone());
                }
            }

            // Check convergence
            if self.best.as_ref().map(|b| b.resonance >= self.config.target_resonance).unwrap_or(false) {
                break;
            }

            // Selection (top half)
            population.truncate(population_size / 2);

            // Crossover + Mutation
            let mut offspring = Vec::new();
            while offspring.len() + population.len() < population_size {
                let parent1 = &population[rand::random::<usize>() % population.len()];
                let parent2 = &population[rand::random::<usize>() % population.len()];

                let mut child = self.crossover(parent1, parent2);
                child.generation = generation + 1;

                if rand::random::<f64>() < mutation_rate {
                    self.mutate(&mut child);
                }

                offspring.push(child);
            }

            population.extend(offspring);
            self.current_step += 5;
        }

        self.candidates = population;
        Ok(())
    }

    /// TRITON-inspired spiral mining
    fn mine_triton(&mut self) -> Result<()> {
        use std::f64::consts::PI;
        const PHI: f64 = 1.618033988749895;

        let mut center = Coord5D::center();
        let mut radius = 0.1;

        for step in 0..self.config.depth {
            self.current_step = step;
            let layer = step / 10;

            // Generate spiral points
            for i in 0..10 {
                let angle = 2.0 * PI * (i as f64) / 10.0 + (layer as f64) * PHI;

                // Create sequence biased towards spiral position
                let bias_coord = Coord5D::new(
                    (center.psi + radius * angle.cos()).clamp(0.0, 1.0),
                    (center.rho + radius * angle.sin()).clamp(0.0, 1.0),
                    (center.omega + radius * (angle * 0.5).cos()).clamp(0.0, 1.0),
                    (center.chi + radius * (angle * 0.3).sin()).clamp(0.0, 1.0),
                    (center.eta + radius * (angle * 0.2).cos()).clamp(0.0, 1.0),
                );

                let seq = self.generate_biased_sequence(&bias_coord);

                if self.best.is_none() || seq.resonance > self.best.as_ref().unwrap().resonance {
                    self.best = Some(seq.clone());
                    // Move center towards best
                    center = seq.to_hypercube_coord();
                }
                self.candidates.push(seq);
            }

            radius *= PHI.powf(0.1);

            if self.best.as_ref().map(|b| b.resonance >= self.config.target_resonance).unwrap_or(false) {
                break;
            }
        }
        Ok(())
    }

    /// Generate a random sequence
    fn generate_sequence(&mut self) -> MinedSequence {
        self.lattice.reset();
        let _ = self.lattice.spin(&mut self.entropy_mapper);

        let symbols: Vec<SlotSymbol> = self.lattice.nodes()
            .map(|n| n.slot.value.symbol)
            .collect();
        let values: Vec<f64> = self.lattice.nodes()
            .map(|n| n.slot.value.value)
            .collect();

        MinedSequence::new(symbols, values, self.current_step)
    }

    /// Generate a biased sequence
    fn generate_biased_sequence(&mut self, bias: &Coord5D) -> MinedSequence {
        let mut seq = self.generate_sequence();

        // Adjust values based on bias
        for i in 0..seq.values.len().min(5) {
            let bias_val = match i {
                0 => bias.psi,
                1 => bias.rho,
                2 => bias.omega,
                3 => bias.chi,
                4 => bias.eta,
                _ => 0.5,
            };
            seq.values[i] = (seq.values[i] + bias_val) / 2.0;
        }

        seq.resonance = MinedSequence::compute_resonance(&seq.symbols, &seq.values);
        seq.coord5d = MinedSequence::to_coord5d(&seq.symbols, &seq.values);
        seq
    }

    /// Generate initial sequence
    fn generate_initial_sequence(&mut self) -> MinedSequence {
        self.generate_sequence()
    }

    /// Crossover two sequences
    fn crossover(&self, parent1: &MinedSequence, parent2: &MinedSequence) -> MinedSequence {
        let len = parent1.symbols.len().min(parent2.symbols.len());
        let crossover_point = rand::random::<usize>() % len.max(1);

        let mut symbols = Vec::new();
        let mut values = Vec::new();

        for i in 0..len {
            if i < crossover_point {
                symbols.push(parent1.symbols[i]);
                values.push(parent1.values[i]);
            } else {
                symbols.push(parent2.symbols[i]);
                values.push(parent2.values[i]);
            }
        }

        MinedSequence::new(symbols, values, self.current_step)
    }

    /// Mutate a sequence
    fn mutate(&mut self, seq: &mut MinedSequence) {
        if seq.values.is_empty() {
            return;
        }

        let idx = rand::random::<usize>() % seq.values.len();
        let delta: f64 = (rand::random::<f64>() - 0.5) * 0.2;
        seq.values[idx] = (seq.values[idx] + delta).clamp(0.0, 1.0);

        seq.resonance = MinedSequence::compute_resonance(&seq.symbols, &seq.values);
        seq.coord5d = MinedSequence::to_coord5d(&seq.symbols, &seq.values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miner_creation() {
        let miner = SequenceMiner::default_miner();
        assert!(miner.best.is_none());
    }

    #[test]
    fn test_mine_greedy() {
        let mut miner = SequenceMiner::new(MinerConfig {
            depth: 10,
            strategy: MiningStrategy::Greedy,
            ..Default::default()
        });

        let result = miner.mine().unwrap();
        assert!(result.best_resonance >= 0.0);
    }

    #[test]
    fn test_mine_beam_search() {
        let mut miner = SequenceMiner::new(MinerConfig {
            depth: 10,
            strategy: MiningStrategy::BeamSearch,
            ..Default::default()
        });

        let result = miner.mine().unwrap();
        assert!(!result.top_sequences.is_empty());
    }

    #[test]
    fn test_mined_sequence() {
        let symbols = vec![SlotSymbol::Psi, SlotSymbol::Rho, SlotSymbol::Omega];
        let values = vec![0.8, 0.7, 0.6];
        let seq = MinedSequence::new(symbols, values, 0);

        assert!(seq.resonance > 0.0);
        assert_eq!(seq.coord5d.len(), 5);
    }
}
