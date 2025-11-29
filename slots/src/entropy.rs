//! Entropy Mapping System
//!
//! Maps randomness sources to structured operator outcomes.

use rand::Rng;
use rand_distr::{Distribution, Normal, Uniform};
use serde::{Deserialize, Serialize};

/// Entropy distribution type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntropyDistribution {
    /// Uniform distribution
    Uniform,
    /// Normal/Gaussian distribution
    Normal,
    /// Exponential (favors low values)
    Exponential,
    /// Beta distribution (configurable shape)
    Beta,
    /// Bimodal (two peaks)
    Bimodal,
    /// Resonance-optimized (favors high resonance outcomes)
    ResonanceOptimized,
}

/// Entropy source type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntropySource {
    /// Pseudo-random number generator
    PRNG,
    /// Time-based entropy
    Temporal,
    /// Hash-based (deterministic from seed)
    Hash,
    /// Mixed sources
    Mixed,
}

/// Entropy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyConfig {
    /// Distribution type
    pub distribution: EntropyDistribution,
    /// Source type
    pub source: EntropySource,
    /// Depth (number of entropy values generated per call)
    pub depth: usize,
    /// Seed for reproducibility (if Some)
    pub seed: Option<u64>,
    /// Mean for normal distribution
    pub mean: f64,
    /// Standard deviation for normal distribution
    pub std_dev: f64,
    /// Bias factor (shifts distribution)
    pub bias: f64,
}

impl Default for EntropyConfig {
    fn default() -> Self {
        Self {
            distribution: EntropyDistribution::Uniform,
            source: EntropySource::PRNG,
            depth: 8,
            seed: None,
            mean: 0.5,
            std_dev: 0.2,
            bias: 0.0,
        }
    }
}

impl EntropyConfig {
    /// Create stochastic config
    pub fn stochastic() -> Self {
        Self {
            distribution: EntropyDistribution::Normal,
            std_dev: 0.3,
            ..Default::default()
        }
    }

    /// Create resonance-optimized config
    pub fn resonance_optimized() -> Self {
        Self {
            distribution: EntropyDistribution::ResonanceOptimized,
            bias: 0.2,
            ..Default::default()
        }
    }

    /// Create deterministic config
    pub fn deterministic(seed: u64) -> Self {
        Self {
            source: EntropySource::Hash,
            seed: Some(seed),
            ..Default::default()
        }
    }
}

/// Entropy mapper for generating structured random values
pub struct EntropyMapper {
    config: EntropyConfig,
    rng: rand::rngs::ThreadRng,
    buffer: Vec<f64>,
    position: usize,
}

impl EntropyMapper {
    /// Create a new entropy mapper
    pub fn new(config: EntropyConfig) -> Self {
        Self {
            config,
            rng: rand::thread_rng(),
            buffer: Vec::new(),
            position: 0,
        }
    }

    /// Create with default config
    pub fn default_mapper() -> Self {
        Self::new(EntropyConfig::default())
    }

    /// Generate a single entropy value
    pub fn generate(&mut self) -> f64 {
        let raw = self.generate_raw();
        self.apply_bias(raw)
    }

    /// Generate multiple entropy values
    pub fn generate_batch(&mut self, count: usize) -> Vec<f64> {
        (0..count).map(|_| self.generate()).collect()
    }

    /// Generate entropy sequence of configured depth
    pub fn generate_sequence(&mut self) -> Vec<f64> {
        self.generate_batch(self.config.depth)
    }

    /// Generate raw value based on distribution
    fn generate_raw(&mut self) -> f64 {
        match self.config.distribution {
            EntropyDistribution::Uniform => {
                self.rng.gen::<f64>()
            }
            EntropyDistribution::Normal => {
                let normal = Normal::new(self.config.mean, self.config.std_dev).unwrap();
                normal.sample(&mut self.rng).clamp(0.0, 1.0)
            }
            EntropyDistribution::Exponential => {
                let u: f64 = self.rng.gen();
                1.0 - (-u.ln()).min(5.0) / 5.0
            }
            EntropyDistribution::Beta => {
                // Simplified beta using two uniforms
                let u1: f64 = self.rng.gen();
                let u2: f64 = self.rng.gen();
                (u1.powf(0.5) + u2.powf(0.5)) / 2.0
            }
            EntropyDistribution::Bimodal => {
                if self.rng.gen::<f64>() < 0.5 {
                    let normal = Normal::new(0.3, 0.1).unwrap();
                    (normal.sample(&mut self.rng) as f64).clamp(0.0, 1.0)
                } else {
                    let normal = Normal::new(0.7, 0.1).unwrap();
                    (normal.sample(&mut self.rng) as f64).clamp(0.0, 1.0)
                }
            }
            EntropyDistribution::ResonanceOptimized => {
                // Distribution that favors high resonance values
                // More probability mass in high psi, rho, omega regions
                let u: f64 = self.rng.gen();
                let adjusted = u.powf(0.7); // Shift toward higher values
                0.3 + 0.7 * adjusted
            }
        }
    }

    /// Apply bias to value
    fn apply_bias(&self, value: f64) -> f64 {
        (value + self.config.bias).clamp(0.0, 1.0)
    }

    /// Map entropy to slot index (for symbol selection)
    pub fn map_to_index(&mut self, max_index: usize) -> usize {
        let entropy = self.generate();
        ((entropy * max_index as f64) as usize).min(max_index - 1)
    }

    /// Map entropy to 5D coordinate
    pub fn map_to_coord5d(&mut self) -> [f64; 5] {
        let seq = self.generate_sequence();
        [
            seq.get(0).copied().unwrap_or(0.5),
            seq.get(1).copied().unwrap_or(0.5),
            seq.get(2).copied().unwrap_or(0.5),
            seq.get(3).copied().unwrap_or(0.5),
            seq.get(4).copied().unwrap_or(0.5),
        ]
    }

    /// Fill buffer for batched generation
    pub fn fill_buffer(&mut self, size: usize) {
        self.buffer = self.generate_batch(size);
        self.position = 0;
    }

    /// Get next value from buffer
    pub fn next_buffered(&mut self) -> f64 {
        if self.position >= self.buffer.len() {
            self.fill_buffer(self.config.depth);
        }
        let value = self.buffer[self.position];
        self.position += 1;
        value
    }

    /// Get current config
    pub fn config(&self) -> &EntropyConfig {
        &self.config
    }

    /// Update config
    pub fn set_config(&mut self, config: EntropyConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_distribution() {
        let mut mapper = EntropyMapper::default_mapper();
        let values: Vec<f64> = (0..100).map(|_| mapper.generate()).collect();

        // All values should be in [0, 1]
        assert!(values.iter().all(|&v| v >= 0.0 && v <= 1.0));
    }

    #[test]
    fn test_normal_distribution() {
        let config = EntropyConfig {
            distribution: EntropyDistribution::Normal,
            mean: 0.5,
            std_dev: 0.15,
            ..Default::default()
        };
        let mut mapper = EntropyMapper::new(config);
        let values: Vec<f64> = (0..100).map(|_| mapper.generate()).collect();

        // Mean should be close to 0.5
        let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;
        assert!((mean - 0.5).abs() < 0.2);
    }

    #[test]
    fn test_resonance_optimized() {
        let config = EntropyConfig::resonance_optimized();
        let mut mapper = EntropyMapper::new(config);
        let values: Vec<f64> = (0..100).map(|_| mapper.generate()).collect();

        // Should favor higher values
        let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;
        assert!(mean > 0.4); // Should be biased upward
    }

    #[test]
    fn test_coord5d_mapping() {
        let mut mapper = EntropyMapper::default_mapper();
        let coord = mapper.map_to_coord5d();

        assert_eq!(coord.len(), 5);
        assert!(coord.iter().all(|&v| v >= 0.0 && v <= 1.0));
    }
}
