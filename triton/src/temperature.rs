//! Temperature controller for simulated annealing.

use crate::config::{TemperatureSchedule, AnnealingScheduleType};
use serde::{Deserialize, Serialize};

/// Annealing strategy trait
pub trait AnnealingStrategy {
    /// Compute temperature at given step
    fn temperature(&self, step: usize) -> f64;

    /// Check if move should be accepted
    fn accept(&self, current_score: f64, new_score: f64, temperature: f64) -> bool;
}

/// Temperature controller for the optimization process
#[derive(Debug, Clone)]
pub struct TemperatureController {
    schedule: TemperatureSchedule,
    current_temp: f64,
    step: usize,
    max_steps: usize,
    acceptance_history: Vec<bool>,
    acceptance_rate: f64,
    rng: rand::rngs::StdRng,
}

impl TemperatureController {
    /// Create new temperature controller
    pub fn new(schedule: TemperatureSchedule, max_steps: usize) -> Self {
        Self::with_seed(schedule, max_steps, rand::random())
    }

    /// Create with specific seed
    pub fn with_seed(schedule: TemperatureSchedule, max_steps: usize, seed: u64) -> Self {
        use rand::SeedableRng;
        Self {
            current_temp: schedule.initial,
            step: 0,
            max_steps,
            acceptance_history: Vec::new(),
            acceptance_rate: 1.0,
            rng: rand::rngs::StdRng::seed_from_u64(seed),
            schedule,
        }
    }

    /// Get current temperature
    pub fn temperature(&self) -> f64 {
        self.current_temp
    }

    /// Get current step
    pub fn step(&self) -> usize {
        self.step
    }

    /// Get acceptance rate
    pub fn acceptance_rate(&self) -> f64 {
        self.acceptance_rate
    }

    /// Advance to next step
    pub fn advance(&mut self) {
        self.step += 1;
        self.update_temperature();
        self.update_acceptance_rate();

        // Check for reheat
        if self.should_reheat() {
            self.reheat();
        }
    }

    /// Update temperature based on schedule
    fn update_temperature(&mut self) {
        let t0 = self.schedule.initial;
        let n = self.step as f64;
        let n_max = self.max_steps as f64;

        self.current_temp = match self.schedule.schedule {
            AnnealingScheduleType::Exponential => {
                t0 * self.schedule.decay.powi(self.step as i32)
            }
            AnnealingScheduleType::Linear => {
                t0 * (1.0 - n / n_max).max(0.0)
            }
            AnnealingScheduleType::Logarithmic => {
                t0 / (1.0 + n).ln()
            }
            AnnealingScheduleType::Quadratic => {
                t0 * (1.0 - n / n_max).powi(2).max(0.0)
            }
            AnnealingScheduleType::Adaptive => {
                // Adjust based on acceptance rate
                if self.acceptance_rate > 0.5 {
                    self.current_temp * 0.99
                } else if self.acceptance_rate < 0.2 {
                    self.current_temp * 1.01
                } else {
                    self.current_temp * self.schedule.decay
                }
            }
        };

        // Enforce minimum temperature
        self.current_temp = self.current_temp.max(self.schedule.final_temp);
    }

    /// Update rolling acceptance rate
    fn update_acceptance_rate(&mut self) {
        // Keep last 100 acceptances
        if self.acceptance_history.len() > 100 {
            self.acceptance_history.remove(0);
        }

        if !self.acceptance_history.is_empty() {
            let accepted = self.acceptance_history.iter().filter(|&&x| x).count();
            self.acceptance_rate = accepted as f64 / self.acceptance_history.len() as f64;
        }
    }

    /// Check if reheat is needed
    fn should_reheat(&self) -> bool {
        if self.step % self.schedule.reheat_interval != 0 {
            return false;
        }

        // Reheat if acceptance rate is too low and temperature is low
        self.acceptance_rate < 0.1 && self.current_temp < self.schedule.initial * 0.1
    }

    /// Reheat the system
    fn reheat(&mut self) {
        self.current_temp *= self.schedule.reheat_factor;
        self.current_temp = self.current_temp.min(self.schedule.initial * 0.5);
        tracing::debug!(
            "Reheating to temperature: {:.4}",
            self.current_temp
        );
    }

    /// Decide whether to accept a move
    pub fn accept(&mut self, current_score: f64, new_score: f64) -> bool {
        use rand::Rng;

        // Always accept improvements
        if new_score >= current_score {
            self.acceptance_history.push(true);
            return true;
        }

        // Metropolis criterion for worse moves
        let delta = new_score - current_score;
        let probability = (delta / self.current_temp).exp();

        let accepted = self.rng.gen::<f64>() < probability;
        self.acceptance_history.push(accepted);

        accepted
    }

    /// Reset controller
    pub fn reset(&mut self) {
        self.current_temp = self.schedule.initial;
        self.step = 0;
        self.acceptance_history.clear();
        self.acceptance_rate = 1.0;
    }

    /// Get schedule progress
    pub fn progress(&self) -> f64 {
        self.step as f64 / self.max_steps as f64
    }

    /// Check if cooling is complete
    pub fn is_cold(&self) -> bool {
        self.current_temp <= self.schedule.final_temp * 1.01
    }
}

impl AnnealingStrategy for TemperatureController {
    fn temperature(&self, _step: usize) -> f64 {
        self.current_temp
    }

    fn accept(&self, current_score: f64, new_score: f64, temperature: f64) -> bool {
        if new_score >= current_score {
            return true;
        }

        let delta = new_score - current_score;
        let probability = (delta / temperature).exp();

        // Note: This implementation is deterministic for the trait
        probability > 0.5
    }
}

/// Temperature statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureStats {
    /// Initial temperature
    pub initial_temp: f64,
    /// Final temperature
    pub final_temp: f64,
    /// Current temperature
    pub current_temp: f64,
    /// Total steps
    pub total_steps: usize,
    /// Average acceptance rate
    pub avg_acceptance_rate: f64,
    /// Number of reheats
    pub reheat_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_controller() {
        let schedule = TemperatureSchedule::default();
        let mut controller = TemperatureController::new(schedule, 100);

        let initial = controller.temperature();
        for _ in 0..50 {
            controller.advance();
        }

        assert!(controller.temperature() < initial);
    }

    #[test]
    fn test_acceptance() {
        let schedule = TemperatureSchedule {
            initial: 1.0,
            ..Default::default()
        };
        let mut controller = TemperatureController::with_seed(schedule, 100, 42);

        // Better score should always be accepted
        let temp = controller.temperature();
        assert!(controller.accept(0.5, 0.7, temp));

        // Worse score at high temperature might be accepted
        controller.accept(0.7, 0.5, temp);
    }

    #[test]
    fn test_exponential_cooling() {
        let schedule = TemperatureSchedule {
            initial: 1.0,
            decay: 0.9,
            schedule: AnnealingScheduleType::Exponential,
            ..Default::default()
        };
        let mut controller = TemperatureController::new(schedule, 100);

        let t0 = controller.temperature();
        controller.advance();
        let t1 = controller.temperature();

        assert!((t1 - t0 * 0.9).abs() < 0.001);
    }
}
