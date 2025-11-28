//! Quantum Slot Definition
//!
//! A slot is the fundamental unit of the QSlots engine, representing
//! a configurable state that can generate operator values.

use crate::spin::SlotSpin;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Slot symbol enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SlotSymbol {
    /// Psi symbol (ψ) - Quality
    Psi,
    /// Rho symbol (ρ) - Stability
    Rho,
    /// Omega symbol (ω) - Efficiency
    Omega,
    /// Chi symbol (χ) - Topological coherence
    Chi,
    /// Eta symbol (η) - Fluctuation
    Eta,
    /// Star - Wildcard/bonus
    Star,
    /// Diamond - High value
    Diamond,
    /// Circle - Standard value
    Circle,
}

impl SlotSymbol {
    /// Get all symbols
    pub fn all() -> Vec<Self> {
        vec![
            Self::Psi, Self::Rho, Self::Omega, Self::Chi, Self::Eta,
            Self::Star, Self::Diamond, Self::Circle,
        ]
    }

    /// Get resonance symbols only
    pub fn resonance_symbols() -> Vec<Self> {
        vec![Self::Psi, Self::Rho, Self::Omega, Self::Chi, Self::Eta]
    }

    /// Get base value for symbol
    pub fn base_value(&self) -> f64 {
        match self {
            Self::Psi => 0.4,    // Highest weight in resonance
            Self::Rho => 0.3,
            Self::Omega => 0.3,
            Self::Chi => 0.05,
            Self::Eta => -0.05,  // Negative contribution
            Self::Star => 0.5,   // Bonus multiplier
            Self::Diamond => 0.8,
            Self::Circle => 0.2,
        }
    }

    /// Convert to dimension index
    pub fn to_dimension(&self) -> Option<usize> {
        match self {
            Self::Psi => Some(0),
            Self::Rho => Some(1),
            Self::Omega => Some(2),
            Self::Chi => Some(3),
            Self::Eta => Some(4),
            _ => None,
        }
    }

    /// Get symbol character
    pub fn char(&self) -> char {
        match self {
            Self::Psi => 'ψ',
            Self::Rho => 'ρ',
            Self::Omega => 'ω',
            Self::Chi => 'χ',
            Self::Eta => 'η',
            Self::Star => '*',
            Self::Diamond => '◆',
            Self::Circle => '○',
        }
    }
}

impl std::fmt::Display for SlotSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}

/// Value held by a slot
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SlotValue {
    /// Primary symbol
    pub symbol: SlotSymbol,
    /// Numeric value (0.0 - 1.0)
    pub value: f64,
    /// Multiplier (from spin)
    pub multiplier: f64,
    /// Is locked
    pub locked: bool,
}

impl SlotValue {
    /// Create a new slot value
    pub fn new(symbol: SlotSymbol, value: f64) -> Self {
        Self {
            symbol,
            value: value.clamp(0.0, 1.0),
            multiplier: 1.0,
            locked: false,
        }
    }

    /// Create with multiplier
    pub fn with_multiplier(mut self, mult: f64) -> Self {
        self.multiplier = mult;
        self
    }

    /// Get effective value
    pub fn effective_value(&self) -> f64 {
        self.value * self.multiplier * self.symbol.base_value().abs()
    }

    /// Lock the value
    pub fn lock(&mut self) {
        self.locked = true;
    }

    /// Unlock the value
    pub fn unlock(&mut self) {
        self.locked = false;
    }
}

impl Default for SlotValue {
    fn default() -> Self {
        Self::new(SlotSymbol::Circle, 0.5)
    }
}

/// Slot state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SlotState {
    /// Slot is idle
    Idle,
    /// Slot is spinning
    Spinning,
    /// Slot is stopping
    Stopping,
    /// Slot has stopped with result
    Stopped,
    /// Slot is locked
    Locked,
    /// Slot is in bonus mode
    Bonus,
}

/// Slot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotConfig {
    /// Available symbols
    pub symbols: Vec<SlotSymbol>,
    /// Symbol weights (probability)
    pub weights: Vec<f64>,
    /// Minimum value
    pub min_value: f64,
    /// Maximum value
    pub max_value: f64,
    /// Spin duration (ms)
    pub spin_duration_ms: u64,
    /// Allow locking
    pub allow_lock: bool,
    /// Bonus probability
    pub bonus_probability: f64,
}

impl Default for SlotConfig {
    fn default() -> Self {
        Self {
            symbols: SlotSymbol::all(),
            weights: vec![0.15, 0.15, 0.15, 0.1, 0.1, 0.1, 0.15, 0.1],
            min_value: 0.0,
            max_value: 1.0,
            spin_duration_ms: 500,
            allow_lock: true,
            bonus_probability: 0.05,
        }
    }
}

impl SlotConfig {
    /// Create resonance-focused config
    pub fn resonance_focused() -> Self {
        Self {
            symbols: SlotSymbol::resonance_symbols(),
            weights: vec![0.25, 0.2, 0.2, 0.15, 0.2],
            min_value: 0.3,
            max_value: 1.0,
            bonus_probability: 0.1,
            ..Default::default()
        }
    }
}

/// A quantum slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    /// Unique identifier
    pub id: String,
    /// Slot name
    pub name: String,
    /// Configuration
    pub config: SlotConfig,
    /// Current state
    pub state: SlotState,
    /// Current value
    pub value: SlotValue,
    /// Spin mechanics
    pub spin: SlotSpin,
    /// History of values
    pub history: Vec<SlotValue>,
    /// Position in lattice
    pub position: Option<(usize, usize)>,
    /// Connected slot IDs
    pub connections: Vec<String>,
}

impl Slot {
    /// Create a new slot
    pub fn new(name: &str, config: SlotConfig) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            config,
            state: SlotState::Idle,
            value: SlotValue::default(),
            spin: SlotSpin::default(),
            history: Vec::new(),
            position: None,
            connections: Vec::new(),
        }
    }

    /// Create with default config
    pub fn default_slot(name: &str) -> Self {
        Self::new(name, SlotConfig::default())
    }

    /// Start spinning
    pub fn spin(&mut self) {
        if self.state != SlotState::Locked {
            self.state = SlotState::Spinning;
            self.spin.start();
        }
    }

    /// Stop and generate result
    pub fn stop(&mut self, entropy: f64) -> SlotValue {
        self.state = SlotState::Stopping;

        // Select symbol based on entropy and weights
        let symbol_idx = self.select_symbol(entropy);
        let symbol = self.config.symbols[symbol_idx];

        // Generate value
        let range = self.config.max_value - self.config.min_value;
        let raw_value = self.config.min_value + entropy * range;

        // Apply spin multiplier
        let multiplier = self.spin.get_multiplier();

        self.value = SlotValue::new(symbol, raw_value).with_multiplier(multiplier);

        // Check for bonus
        if entropy > (1.0 - self.config.bonus_probability) {
            self.state = SlotState::Bonus;
            self.value.multiplier *= 2.0;
        } else {
            self.state = SlotState::Stopped;
        }

        // Record history
        self.history.push(self.value);

        self.value
    }

    /// Select symbol based on entropy and weights
    fn select_symbol(&self, entropy: f64) -> usize {
        let total_weight: f64 = self.config.weights.iter().sum();
        let target = entropy * total_weight;

        let mut cumulative = 0.0;
        for (i, &weight) in self.config.weights.iter().enumerate() {
            cumulative += weight;
            if target <= cumulative {
                return i;
            }
        }

        self.config.symbols.len() - 1
    }

    /// Lock slot
    pub fn lock(&mut self) {
        if self.config.allow_lock {
            self.state = SlotState::Locked;
            self.value.lock();
        }
    }

    /// Unlock slot
    pub fn unlock(&mut self) {
        if self.state == SlotState::Locked {
            self.state = SlotState::Idle;
            self.value.unlock();
        }
    }

    /// Reset slot
    pub fn reset(&mut self) {
        self.state = SlotState::Idle;
        self.value = SlotValue::default();
        self.spin.reset();
    }

    /// Get average value from history
    pub fn average_value(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        self.history.iter().map(|v| v.effective_value()).sum::<f64>() / self.history.len() as f64
    }

    /// Get best value from history
    pub fn best_value(&self) -> Option<&SlotValue> {
        self.history.iter().max_by(|a, b| {
            a.effective_value().partial_cmp(&b.effective_value()).unwrap()
        })
    }

    /// Check if in bonus state
    pub fn is_bonus(&self) -> bool {
        self.state == SlotState::Bonus
    }
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Slot[{}] '{}' {:?}: {} = {:.3}",
            &self.id[..8], self.name, self.state,
            self.value.symbol, self.value.effective_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_creation() {
        let slot = Slot::default_slot("test");
        assert_eq!(slot.state, SlotState::Idle);
    }

    #[test]
    fn test_slot_spin() {
        let mut slot = Slot::default_slot("test");
        slot.spin();
        assert_eq!(slot.state, SlotState::Spinning);

        let value = slot.stop(0.5);
        assert!(value.value >= 0.0 && value.value <= 1.0);
    }

    #[test]
    fn test_slot_lock() {
        let mut slot = Slot::default_slot("test");
        slot.lock();
        assert_eq!(slot.state, SlotState::Locked);

        slot.spin(); // Should not change state when locked
        assert_eq!(slot.state, SlotState::Locked);
    }

    #[test]
    fn test_symbol_values() {
        assert_eq!(SlotSymbol::Psi.base_value(), 0.4);
        assert_eq!(SlotSymbol::Eta.base_value(), -0.05);
    }
}
