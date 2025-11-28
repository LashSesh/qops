//! Slot Spin Mechanics
//!
//! Handles the spinning animation and value generation dynamics.

use serde::{Deserialize, Serialize};

/// Spin state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpinState {
    /// Not spinning
    Idle,
    /// Accelerating
    Accelerating,
    /// At full speed
    FullSpeed,
    /// Decelerating
    Decelerating,
    /// Stopped
    Stopped,
}

/// Spin dynamics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinDynamics {
    /// Acceleration rate
    pub acceleration: f64,
    /// Maximum angular velocity
    pub max_velocity: f64,
    /// Deceleration rate
    pub deceleration: f64,
    /// Base multiplier
    pub base_multiplier: f64,
    /// Velocity boost per spin
    pub velocity_boost: f64,
}

impl Default for SpinDynamics {
    fn default() -> Self {
        Self {
            acceleration: 5.0,
            max_velocity: 20.0,
            deceleration: 3.0,
            base_multiplier: 1.0,
            velocity_boost: 0.1,
        }
    }
}

/// Slot spin mechanics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotSpin {
    /// Current state
    pub state: SpinState,
    /// Dynamics configuration
    pub dynamics: SpinDynamics,
    /// Current angular velocity
    pub velocity: f64,
    /// Current angle (radians)
    pub angle: f64,
    /// Number of complete rotations
    pub rotations: usize,
    /// Spin count (total spins)
    pub spin_count: usize,
    /// Accumulated momentum
    pub momentum: f64,
    /// Start timestamp (ms)
    pub start_time: Option<u64>,
}

impl Default for SlotSpin {
    fn default() -> Self {
        Self {
            state: SpinState::Idle,
            dynamics: SpinDynamics::default(),
            velocity: 0.0,
            angle: 0.0,
            rotations: 0,
            spin_count: 0,
            momentum: 0.0,
            start_time: None,
        }
    }
}

impl SlotSpin {
    /// Create with custom dynamics
    pub fn with_dynamics(dynamics: SpinDynamics) -> Self {
        Self {
            dynamics,
            ..Default::default()
        }
    }

    /// Start spinning
    pub fn start(&mut self) {
        self.state = SpinState::Accelerating;
        self.velocity = 1.0;
        self.spin_count += 1;
        self.start_time = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64);
    }

    /// Update spin state (called each frame/tick)
    pub fn update(&mut self, delta_time: f64) {
        match self.state {
            SpinState::Idle | SpinState::Stopped => {}
            SpinState::Accelerating => {
                self.velocity += self.dynamics.acceleration * delta_time;
                if self.velocity >= self.dynamics.max_velocity {
                    self.velocity = self.dynamics.max_velocity;
                    self.state = SpinState::FullSpeed;
                }
            }
            SpinState::FullSpeed => {
                // Maintain constant velocity
            }
            SpinState::Decelerating => {
                self.velocity -= self.dynamics.deceleration * delta_time;
                if self.velocity <= 0.0 {
                    self.velocity = 0.0;
                    self.state = SpinState::Stopped;
                }
            }
        }

        // Update angle
        if self.velocity > 0.0 {
            let delta_angle = self.velocity * delta_time;
            self.angle += delta_angle;

            // Count rotations
            while self.angle >= 2.0 * std::f64::consts::PI {
                self.angle -= 2.0 * std::f64::consts::PI;
                self.rotations += 1;
            }

            // Accumulate momentum
            self.momentum += delta_angle * 0.1;
        }
    }

    /// Start deceleration
    pub fn decelerate(&mut self) {
        if matches!(self.state, SpinState::Accelerating | SpinState::FullSpeed) {
            self.state = SpinState::Decelerating;
        }
    }

    /// Get multiplier based on spin state
    pub fn get_multiplier(&self) -> f64 {
        let base = self.dynamics.base_multiplier;
        let rotation_bonus = (self.rotations as f64) * 0.01;
        let momentum_bonus = (self.momentum * 0.1).min(0.5);
        let spin_bonus = (self.spin_count as f64).sqrt() * self.dynamics.velocity_boost;

        base + rotation_bonus + momentum_bonus + spin_bonus
    }

    /// Reset spin state
    pub fn reset(&mut self) {
        self.state = SpinState::Idle;
        self.velocity = 0.0;
        self.angle = 0.0;
        self.rotations = 0;
        self.momentum = 0.0;
        self.start_time = None;
    }

    /// Check if spinning
    pub fn is_spinning(&self) -> bool {
        !matches!(self.state, SpinState::Idle | SpinState::Stopped)
    }

    /// Get elapsed time since start (ms)
    pub fn elapsed_ms(&self) -> u64 {
        self.start_time.map(|start| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            now - start
        }).unwrap_or(0)
    }

    /// Get normalized position (0.0 - 1.0 around the wheel)
    pub fn normalized_position(&self) -> f64 {
        self.angle / (2.0 * std::f64::consts::PI)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spin_creation() {
        let spin = SlotSpin::default();
        assert_eq!(spin.state, SpinState::Idle);
        assert_eq!(spin.velocity, 0.0);
    }

    #[test]
    fn test_spin_start() {
        let mut spin = SlotSpin::default();
        spin.start();

        assert_eq!(spin.state, SpinState::Accelerating);
        assert!(spin.velocity > 0.0);
        assert_eq!(spin.spin_count, 1);
    }

    #[test]
    fn test_spin_update() {
        let mut spin = SlotSpin::default();
        spin.start();

        let initial_velocity = spin.velocity;
        spin.update(0.1);

        assert!(spin.velocity > initial_velocity);
    }

    #[test]
    fn test_multiplier() {
        let mut spin = SlotSpin::default();

        // Multiple spins should increase multiplier
        for _ in 0..5 {
            spin.start();
            spin.momentum += 1.0;
        }

        assert!(spin.get_multiplier() > 1.0);
    }
}
