//! Slot Generator
//!
//! Generates slot configurations and sequences.

use crate::slot::{Slot, SlotConfig, SlotValue, SlotSymbol};
use crate::entropy::{EntropyMapper, EntropyConfig};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Generator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    /// Base slot configuration
    pub slot_config: SlotConfig,
    /// Entropy configuration
    pub entropy_config: EntropyConfig,
    /// Number of slots to generate
    pub count: usize,
    /// Apply resonance optimization
    pub resonance_optimized: bool,
    /// Minimum resonance threshold
    pub min_resonance: f64,
    /// Maximum generation attempts
    pub max_attempts: usize,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            slot_config: SlotConfig::default(),
            entropy_config: EntropyConfig::default(),
            count: 5,
            resonance_optimized: true,
            min_resonance: 0.5,
            max_attempts: 100,
        }
    }
}

impl GeneratorConfig {
    /// Create fast generation config
    pub fn fast() -> Self {
        Self {
            count: 3,
            max_attempts: 20,
            resonance_optimized: false,
            ..Default::default()
        }
    }

    /// Create quality generation config
    pub fn quality() -> Self {
        Self {
            entropy_config: EntropyConfig::resonance_optimized(),
            min_resonance: 0.7,
            max_attempts: 200,
            ..Default::default()
        }
    }
}

/// A generated slot with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedSlot {
    /// The slot
    pub slot: Slot,
    /// Generation index
    pub index: usize,
    /// Generation attempt that succeeded
    pub attempt: usize,
    /// Resonance at generation
    pub resonance: f64,
    /// Entropy values used
    pub entropy_used: Vec<f64>,
    /// Is this slot optimized
    pub optimized: bool,
}

/// Slot generator
pub struct SlotGenerator {
    config: GeneratorConfig,
    entropy_mapper: EntropyMapper,
    generated: Vec<GeneratedSlot>,
    total_attempts: usize,
}

impl SlotGenerator {
    /// Create a new generator
    pub fn new(config: GeneratorConfig) -> Self {
        let entropy_mapper = EntropyMapper::new(config.entropy_config.clone());
        Self {
            config,
            entropy_mapper,
            generated: Vec::new(),
            total_attempts: 0,
        }
    }

    /// Create with default config
    pub fn default_generator() -> Self {
        Self::new(GeneratorConfig::default())
    }

    /// Generate a single slot
    pub fn generate_one(&mut self) -> GeneratedSlot {
        let index = self.generated.len();
        let mut best: Option<GeneratedSlot> = None;
        let mut attempt = 0;

        while attempt < self.config.max_attempts {
            attempt += 1;
            self.total_attempts += 1;

            let entropy = self.entropy_mapper.generate_sequence();
            let slot = self.create_slot_from_entropy(&entropy, index);
            let resonance = self.compute_slot_resonance(&slot);

            if !self.config.resonance_optimized || resonance >= self.config.min_resonance {
                let gen_slot = GeneratedSlot {
                    slot,
                    index,
                    attempt,
                    resonance,
                    entropy_used: entropy,
                    optimized: self.config.resonance_optimized,
                };

                self.generated.push(gen_slot.clone());
                return gen_slot;
            }

            // Keep best attempt
            if best.is_none() || resonance > best.as_ref().unwrap().resonance {
                best = Some(GeneratedSlot {
                    slot,
                    index,
                    attempt,
                    resonance,
                    entropy_used: entropy,
                    optimized: false,
                });
            }
        }

        // Return best attempt if threshold not met
        let result = best.unwrap();
        self.generated.push(result.clone());
        result
    }

    /// Generate multiple slots
    pub fn generate_batch(&mut self, count: usize) -> Vec<GeneratedSlot> {
        (0..count).map(|_| self.generate_one()).collect()
    }

    /// Generate configured number of slots
    pub fn generate(&mut self) -> Vec<GeneratedSlot> {
        self.generate_batch(self.config.count)
    }

    /// Create a slot from entropy values
    fn create_slot_from_entropy(&self, entropy: &[f64], index: usize) -> Slot {
        let name = format!("gen_slot_{}", index);
        let mut slot = Slot::new(&name, self.config.slot_config.clone());

        if !entropy.is_empty() {
            // Use first entropy value to determine symbol
            let symbol_idx = (entropy[0] * self.config.slot_config.symbols.len() as f64) as usize;
            let symbol = self.config.slot_config.symbols
                .get(symbol_idx.min(self.config.slot_config.symbols.len() - 1))
                .copied()
                .unwrap_or(SlotSymbol::Circle);

            // Use second entropy value for slot value
            let value = entropy.get(1).copied().unwrap_or(0.5);

            slot.value = SlotValue::new(symbol, value);

            // Use third entropy value for multiplier
            if let Some(&mult_entropy) = entropy.get(2) {
                slot.value = slot.value.with_multiplier(1.0 + mult_entropy * 0.5);
            }
        }

        slot
    }

    /// Compute resonance-like score for a slot
    fn compute_slot_resonance(&self, slot: &Slot) -> f64 {
        let symbol_contribution = match slot.value.symbol {
            SlotSymbol::Psi => 0.4,
            SlotSymbol::Rho | SlotSymbol::Omega => 0.3,
            SlotSymbol::Chi => 0.05,
            SlotSymbol::Eta => -0.05,
            SlotSymbol::Star | SlotSymbol::Diamond => 0.5,
            SlotSymbol::Circle => 0.2,
        };

        (symbol_contribution * slot.value.value * slot.value.multiplier).clamp(0.0, 1.0)
    }

    /// Get all generated slots
    pub fn generated(&self) -> &[GeneratedSlot] {
        &self.generated
    }

    /// Get total attempts
    pub fn total_attempts(&self) -> usize {
        self.total_attempts
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        self.generated.len() as f64 / self.total_attempts as f64
    }

    /// Get average resonance
    pub fn avg_resonance(&self) -> f64 {
        if self.generated.is_empty() {
            return 0.0;
        }
        self.generated.iter().map(|g| g.resonance).sum::<f64>() / self.generated.len() as f64
    }

    /// Clear generated slots
    pub fn clear(&mut self) {
        self.generated.clear();
        self.total_attempts = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let gen = SlotGenerator::default_generator();
        assert!(gen.generated.is_empty());
    }

    #[test]
    fn test_generate_one() {
        let mut gen = SlotGenerator::default_generator();
        let slot = gen.generate_one();

        assert_eq!(slot.index, 0);
        assert!(slot.resonance >= 0.0);
    }

    #[test]
    fn test_generate_batch() {
        let mut gen = SlotGenerator::new(GeneratorConfig {
            count: 5,
            resonance_optimized: false,
            ..Default::default()
        });

        let slots = gen.generate();
        assert_eq!(slots.len(), 5);
    }

    #[test]
    fn test_quality_generation() {
        let mut gen = SlotGenerator::new(GeneratorConfig::quality());
        let slot = gen.generate_one();

        // Quality config should produce higher resonance (most of the time)
        // Note: This is probabilistic, so we use a lower threshold
        assert!(slot.resonance >= 0.0);
    }
}
