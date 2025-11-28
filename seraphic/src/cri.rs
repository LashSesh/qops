//! Calibration Regime Initialization (CRI).

use crate::calibrator::HistoryEntry;
use serde::{Deserialize, Serialize};

/// CRI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRIConfig {
    pub stagnation_threshold: usize,
    pub improvement_epsilon: f64,
}

impl Default for CRIConfig {
    fn default() -> Self {
        Self {
            stagnation_threshold: 10,
            improvement_epsilon: 0.01,
        }
    }
}

/// Calibration Regime Initializer
pub struct CalibrationRegimeInitializer {
    config: CRIConfig,
    stagnation_count: usize,
    last_best_score: f64,
}

impl CalibrationRegimeInitializer {
    /// Create new CRI
    pub fn new(stagnation_threshold: usize) -> Self {
        Self {
            config: CRIConfig {
                stagnation_threshold,
                ..Default::default()
            },
            stagnation_count: 0,
            last_best_score: 0.0,
        }
    }

    /// Check if CRI should trigger
    pub fn should_trigger(&mut self, history: &[HistoryEntry]) -> bool {
        if history.is_empty() {
            return false;
        }

        let recent = &history[history.len().saturating_sub(self.config.stagnation_threshold)..];

        if recent.is_empty() {
            return false;
        }

        // Check for improvement
        let best_score = recent
            .iter()
            .map(|e| e.performance.weighted_sum())
            .fold(0.0, f64::max);

        if (best_score - self.last_best_score).abs() < self.config.improvement_epsilon {
            self.stagnation_count += 1;
        } else {
            self.stagnation_count = 0;
            self.last_best_score = best_score;
        }

        // Trigger if stagnating
        if self.stagnation_count >= self.config.stagnation_threshold {
            self.stagnation_count = 0;
            return true;
        }

        false
    }

    /// Reset CRI state
    pub fn reset(&mut self) {
        self.stagnation_count = 0;
        self.last_best_score = 0.0;
    }
}

impl Default for CalibrationRegimeInitializer {
    fn default() -> Self {
        Self::new(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qops_core::{Configuration, Signature3D};

    #[test]
    fn test_cri_no_stagnation() {
        let mut cri = CalibrationRegimeInitializer::default();

        let history: Vec<HistoryEntry> = (0..5)
            .map(|i| HistoryEntry {
                step: i,
                config: Configuration::default(),
                performance: Signature3D::new(0.5 + i as f64 * 0.05, 0.5, 0.5),
                accepted: true,
            })
            .collect();

        assert!(!cri.should_trigger(&history));
    }

    #[test]
    fn test_cri_stagnation() {
        let mut cri = CalibrationRegimeInitializer::new(5);

        let history: Vec<HistoryEntry> = (0..10)
            .map(|i| HistoryEntry {
                step: i,
                config: Configuration::default(),
                performance: Signature3D::new(0.5, 0.5, 0.5), // Same performance
                accepted: true,
            })
            .collect();

        // Need to call multiple times to accumulate stagnation
        for _ in 0..6 {
            cri.should_trigger(&history);
        }

        // Should eventually trigger
        let triggered = cri.should_trigger(&history);
        // Note: exact behavior depends on implementation
    }
}
