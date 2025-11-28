//! Evolution engine for operator mining.

use qops_core::Signature5D;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Evolution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    pub population_size: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub elite_count: usize,
    pub generations: usize,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            population_size: 50,
            mutation_rate: 0.1,
            crossover_rate: 0.7,
            elite_count: 5,
            generations: 100,
        }
    }
}

/// Evolution statistics per generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationStats {
    pub generation: usize,
    pub best_resonance: f64,
    pub avg_resonance: f64,
    pub mandorla_count: usize,
}

/// Evolution engine
pub struct EvolutionEngine {
    config: EvolutionConfig,
    population: Vec<Signature5D>,
    stats: Vec<GenerationStats>,
    current_generation: usize,
}

impl EvolutionEngine {
    /// Create new evolution engine
    pub fn new(config: EvolutionConfig) -> Self {
        let population = Self::init_population(config.population_size);

        Self {
            config,
            population,
            stats: Vec::new(),
            current_generation: 0,
        }
    }

    fn init_population(size: usize) -> Vec<Signature5D> {
        let mut rng = rand::thread_rng();
        (0..size)
            .map(|_| {
                Signature5D::new(
                    rng.gen_range(0.3..0.9),
                    rng.gen_range(0.3..0.9),
                    rng.gen_range(0.3..0.9),
                    rng.gen_range(0.3..0.9),
                    rng.gen_range(0.1..0.5),
                )
            })
            .collect()
    }

    /// Run one generation
    pub fn evolve_generation(&mut self) -> GenerationStats {
        // Calculate fitness (resonance)
        let mut fitness: Vec<(usize, f64)> = self
            .population
            .iter()
            .enumerate()
            .map(|(i, sig)| (i, qops_core::resonance_5d(sig)))
            .collect();

        fitness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Statistics
        let best_resonance = fitness.first().map(|(_, f)| *f).unwrap_or(0.0);
        let avg_resonance = fitness.iter().map(|(_, f)| f).sum::<f64>() / fitness.len() as f64;
        let mandorla_count = fitness.iter().filter(|(_, f)| *f >= 0.85).count();

        // Selection: keep elites
        let mut new_population: Vec<Signature5D> = fitness
            .iter()
            .take(self.config.elite_count)
            .map(|(i, _)| self.population[*i])
            .collect();

        // Fill rest with crossover and mutation
        let mut rng = rand::thread_rng();
        while new_population.len() < self.config.population_size {
            // Tournament selection
            let parent1 = self.tournament_select(&fitness, &mut rng);
            let parent2 = self.tournament_select(&fitness, &mut rng);

            // Crossover
            let mut child = if rng.gen::<f64>() < self.config.crossover_rate {
                self.crossover(&parent1, &parent2, &mut rng)
            } else {
                parent1
            };

            // Mutation
            if rng.gen::<f64>() < self.config.mutation_rate {
                self.mutate(&mut child, &mut rng);
            }

            new_population.push(child);
        }

        self.population = new_population;
        self.current_generation += 1;

        let stats = GenerationStats {
            generation: self.current_generation,
            best_resonance,
            avg_resonance,
            mandorla_count,
        };

        self.stats.push(stats.clone());
        stats
    }

    fn tournament_select(&self, fitness: &[(usize, f64)], rng: &mut impl Rng) -> Signature5D {
        let i1 = rng.gen_range(0..fitness.len());
        let i2 = rng.gen_range(0..fitness.len());

        let winner = if fitness[i1].1 > fitness[i2].1 {
            fitness[i1].0
        } else {
            fitness[i2].0
        };

        self.population[winner]
    }

    fn crossover(&self, p1: &Signature5D, p2: &Signature5D, rng: &mut impl Rng) -> Signature5D {
        let alpha: f64 = rng.gen();
        Signature5D::new(
            alpha * p1.psi + (1.0 - alpha) * p2.psi,
            alpha * p1.rho + (1.0 - alpha) * p2.rho,
            alpha * p1.omega + (1.0 - alpha) * p2.omega,
            alpha * p1.chi + (1.0 - alpha) * p2.chi,
            alpha * p1.eta + (1.0 - alpha) * p2.eta,
        )
    }

    fn mutate(&self, sig: &mut Signature5D, rng: &mut impl Rng) {
        let delta: f64 = rng.gen_range(-0.1..0.1);
        match rng.gen_range(0..5) {
            0 => sig.psi = (sig.psi + delta).clamp(0.0, 1.0),
            1 => sig.rho = (sig.rho + delta).clamp(0.0, 1.0),
            2 => sig.omega = (sig.omega + delta).clamp(0.0, 1.0),
            3 => sig.chi = (sig.chi + delta).clamp(0.0, 1.0),
            _ => sig.eta = (sig.eta + delta).clamp(0.0, 1.0),
        }
    }

    /// Run full evolution
    pub fn run(&mut self) -> &[GenerationStats] {
        for _ in 0..self.config.generations {
            self.evolve_generation();
        }
        &self.stats
    }

    /// Get best individual
    pub fn best(&self) -> Option<Signature5D> {
        self.population
            .iter()
            .max_by(|a, b| {
                qops_core::resonance_5d(a)
                    .partial_cmp(&qops_core::resonance_5d(b))
                    .unwrap()
            })
            .copied()
    }
}

impl Default for EvolutionEngine {
    fn default() -> Self {
        Self::new(EvolutionConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolution() {
        let config = EvolutionConfig {
            population_size: 20,
            generations: 10,
            ..Default::default()
        };

        let mut engine = EvolutionEngine::new(config);
        let stats = engine.run();

        assert_eq!(stats.len(), 10);
        assert!(engine.best().is_some());
    }
}
