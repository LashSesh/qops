//! Dynamic Tripolar Logic (DTL).

use serde::{Deserialize, Serialize};
use rand::Rng;

/// Tripolar state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TripolarValue {
    /// Active (L+)
    Active,
    /// Inactive (L-)
    Inactive,
    /// Dynamic/undetermined (Ld)
    Dynamic,
}

impl TripolarValue {
    /// Convert to numerical value
    pub fn to_value(&self) -> f64 {
        match self {
            TripolarValue::Active => 1.0,
            TripolarValue::Inactive => 0.0,
            TripolarValue::Dynamic => 0.5,
        }
    }

    /// Create from numerical value
    pub fn from_value(v: f64) -> Self {
        if v > 0.7 {
            TripolarValue::Active
        } else if v < 0.3 {
            TripolarValue::Inactive
        } else {
            TripolarValue::Dynamic
        }
    }
}

impl Default for TripolarValue {
    fn default() -> Self {
        TripolarValue::Dynamic
    }
}

/// DTL State for a system
#[derive(Debug, Clone)]
pub struct DTLState {
    /// Values at each node
    pub values: Vec<TripolarValue>,
    /// Phases for dynamic nodes
    pub phases: Vec<f64>,
}

impl DTLState {
    /// Create new DTL state
    pub fn new(size: usize) -> Self {
        Self {
            values: vec![TripolarValue::Dynamic; size],
            phases: vec![0.0; size],
        }
    }

    /// Get information content
    pub fn information_content(&self) -> f64 {
        // Binary: log2(2^n) = n bits
        // Tripolar: log2(3^n) = n * log2(3) ≈ n * 1.585 bits
        self.values.len() as f64 * 3.0_f64.log2()
    }

    /// Information advantage over binary
    pub fn information_advantage(&self) -> f64 {
        let binary_bits = self.values.len() as f64;
        let tripolar_bits = self.information_content();
        (tripolar_bits - binary_bits) / binary_bits * 100.0
    }

    /// Collapse dynamic states based on phases
    pub fn collapse(&mut self) {
        let mut rng = rand::thread_rng();
        for (i, value) in self.values.iter_mut().enumerate() {
            if *value == TripolarValue::Dynamic {
                let threshold = (self.phases[i]).sin().abs();
                if rng.gen::<f64>() < threshold {
                    *value = TripolarValue::Active;
                } else {
                    *value = TripolarValue::Inactive;
                }
            }
        }
    }

    /// Evolve phases
    pub fn evolve_phases(&mut self, dt: f64, coupling: f64) {
        let n = self.phases.len();
        let mut new_phases = self.phases.clone();

        for i in 0..n {
            // Kuramoto-like dynamics
            let neighbors_avg: f64 = self.phases.iter().sum::<f64>() / n as f64;
            new_phases[i] += dt * coupling * (neighbors_avg - self.phases[i]).sin();
        }

        self.phases = new_phases;
    }
}

impl Default for DTLState {
    fn default() -> Self {
        Self::new(13) // Metatron dimension
    }
}

/// DTL Operations
pub struct DTLOperations;

impl DTLOperations {
    /// Tripolar AND
    pub fn and(a: TripolarValue, b: TripolarValue) -> TripolarValue {
        match (a, b) {
            (TripolarValue::Active, TripolarValue::Active) => TripolarValue::Active,
            (TripolarValue::Inactive, _) | (_, TripolarValue::Inactive) => TripolarValue::Inactive,
            _ => TripolarValue::Dynamic,
        }
    }

    /// Tripolar OR
    pub fn or(a: TripolarValue, b: TripolarValue) -> TripolarValue {
        match (a, b) {
            (TripolarValue::Inactive, TripolarValue::Inactive) => TripolarValue::Inactive,
            (TripolarValue::Active, _) | (_, TripolarValue::Active) => TripolarValue::Active,
            _ => TripolarValue::Dynamic,
        }
    }

    /// Tripolar NOT
    pub fn not(a: TripolarValue) -> TripolarValue {
        match a {
            TripolarValue::Active => TripolarValue::Inactive,
            TripolarValue::Inactive => TripolarValue::Active,
            TripolarValue::Dynamic => TripolarValue::Dynamic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dtl_state() {
        let state = DTLState::new(13);

        // 58.5% advantage for 13 nodes
        let advantage = state.information_advantage();
        assert!((advantage - 58.5).abs() < 1.0);
    }

    #[test]
    fn test_dtl_operations() {
        use TripolarValue::*;

        assert_eq!(DTLOperations::and(Active, Active), Active);
        assert_eq!(DTLOperations::and(Active, Inactive), Inactive);
        assert_eq!(DTLOperations::and(Active, Dynamic), Dynamic);

        assert_eq!(DTLOperations::or(Inactive, Inactive), Inactive);
        assert_eq!(DTLOperations::or(Active, Inactive), Active);

        assert_eq!(DTLOperations::not(Active), Inactive);
        assert_eq!(DTLOperations::not(Dynamic), Dynamic);
    }

    #[test]
    fn test_phase_evolution() {
        let mut state = DTLState::new(5);
        state.phases = vec![0.0, 0.5, 1.0, 1.5, 2.0];

        state.evolve_phases(0.1, 1.0);

        // Phases should have changed
        assert!(state.phases.iter().any(|&p| (p - 0.0).abs() > 1e-10));
    }
}
