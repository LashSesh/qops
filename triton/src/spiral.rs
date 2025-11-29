//! Spiral search engine implementation.

use crate::config::SpiralParams;
use qops_core::Signature5D;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Spiral direction for search
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiralDirection {
    /// Outward expansion
    Outward,
    /// Inward contraction
    Inward,
    /// Bidirectional (alternating)
    Bidirectional,
}

impl Default for SpiralDirection {
    fn default() -> Self {
        Self::Outward
    }
}

/// Current state of spiral search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiralState {
    /// Current layer index
    pub layer: usize,
    /// Current angle (radians)
    pub angle: f64,
    /// Current radius
    pub radius: f64,
    /// Point index within layer
    pub point_index: usize,
    /// Total points visited
    pub total_visited: usize,
    /// Direction of spiral
    pub direction: SpiralDirection,
    /// Current center point (in parameter space)
    pub center: [f64; 5],
}

impl Default for SpiralState {
    fn default() -> Self {
        Self {
            layer: 0,
            angle: 0.0,
            radius: 0.1,
            point_index: 0,
            total_visited: 0,
            direction: SpiralDirection::Outward,
            center: [0.5, 0.5, 0.5, 0.5, 0.25], // Default center
        }
    }
}

/// Spiral search engine
#[derive(Debug, Clone)]
pub struct SpiralEngine {
    params: SpiralParams,
    state: SpiralState,
    rng: rand::rngs::StdRng,
}

impl SpiralEngine {
    /// Create new spiral engine
    pub fn new(params: SpiralParams) -> Self {
        Self::with_seed(params, rand::random())
    }

    /// Create with specific seed
    pub fn with_seed(params: SpiralParams, seed: u64) -> Self {
        use rand::SeedableRng;
        Self {
            state: SpiralState {
                radius: params.initial_radius,
                ..Default::default()
            },
            params,
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    /// Set center point for spiral
    pub fn set_center(&mut self, center: [f64; 5]) {
        self.state.center = center;
    }

    /// Set center from signature
    pub fn set_center_from_signature(&mut self, sig: &Signature5D) {
        self.state.center = [sig.psi, sig.rho, sig.omega, sig.chi, sig.eta];
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        self.state = SpiralState {
            radius: self.params.initial_radius,
            center: self.state.center,
            ..Default::default()
        };
    }

    /// Get current state
    pub fn state(&self) -> &SpiralState {
        &self.state
    }

    /// Check if search is complete
    pub fn is_complete(&self) -> bool {
        self.state.layer >= self.params.layers
    }

    /// Generate next point in spiral
    pub fn next_point(&mut self) -> Option<Signature5D> {
        if self.is_complete() {
            return None;
        }

        // Compute 5D point from spiral coordinates
        let point = self.compute_spiral_point();

        // Advance state
        self.advance();

        Some(point)
    }

    /// Compute point at current spiral position
    fn compute_spiral_point(&self) -> Signature5D {
        let r = self.state.radius;
        let theta = self.state.angle;

        // Project 2D spiral into 5D space using trigonometric expansion
        // This creates a spiral that touches all 5 dimensions
        let phi1 = theta;
        let phi2 = theta * 1.618; // Golden ratio offset
        let phi3 = theta * 2.0;
        let phi4 = theta * 0.618;

        // Compute offsets
        let d_psi = r * phi1.cos() * phi2.sin();
        let d_rho = r * phi1.sin() * phi3.cos();
        let d_omega = r * phi2.cos() * phi4.sin();
        let d_chi = r * phi3.sin() * phi1.cos() * 0.5;
        let d_eta = r * phi4.cos() * phi2.sin() * 0.3;

        // Apply to center with clamping
        let psi = (self.state.center[0] + d_psi).clamp(0.0, 1.0);
        let rho = (self.state.center[1] + d_rho).clamp(0.0, 1.0);
        let omega = (self.state.center[2] + d_omega).clamp(0.0, 1.0);
        let chi = (self.state.center[3] + d_chi).clamp(0.0, 1.0);
        let eta = (self.state.center[4] + d_eta).clamp(0.0, 1.0);

        Signature5D::new(psi, rho, omega, chi, eta)
    }

    /// Advance spiral state
    fn advance(&mut self) {
        self.state.point_index += 1;
        self.state.total_visited += 1;
        self.state.angle += self.params.angular_step;

        // Check if layer is complete
        if self.state.point_index >= self.params.points_per_layer {
            self.state.point_index = 0;
            self.state.layer += 1;

            // Update radius for new layer
            match self.state.direction {
                SpiralDirection::Outward => {
                    self.state.radius = self.params.radius_at_layer(self.state.layer);
                }
                SpiralDirection::Inward => {
                    if self.state.layer < self.params.layers {
                        let remaining = self.params.layers - self.state.layer - 1;
                        self.state.radius = self.params.radius_at_layer(remaining);
                    }
                }
                SpiralDirection::Bidirectional => {
                    if self.state.layer % 2 == 0 {
                        self.state.radius = self.params.radius_at_layer(self.state.layer);
                    } else {
                        self.state.radius /= self.params.expansion_rate;
                    }
                }
            }
        }
    }

    /// Generate random perturbation within current radius
    pub fn random_perturbation(&mut self, sig: &Signature5D) -> Signature5D {
        let r = self.state.radius;

        let psi = (sig.psi + self.rng.gen_range(-r..r)).clamp(0.0, 1.0);
        let rho = (sig.rho + self.rng.gen_range(-r..r)).clamp(0.0, 1.0);
        let omega = (sig.omega + self.rng.gen_range(-r..r)).clamp(0.0, 1.0);
        let chi = (sig.chi + self.rng.gen_range(-r * 0.5..r * 0.5)).clamp(0.0, 1.0);
        let eta = (sig.eta + self.rng.gen_range(-r * 0.3..r * 0.3)).clamp(0.0, 1.0);

        Signature5D::new(psi, rho, omega, chi, eta)
    }

    /// Get all points in current layer
    pub fn layer_points(&self) -> Vec<Signature5D> {
        let mut points = Vec::new();
        let mut engine = self.clone();
        engine.state.point_index = 0;

        for _ in 0..self.params.points_per_layer {
            points.push(engine.compute_spiral_point());
            engine.state.angle += self.params.angular_step;
        }

        points
    }

    /// Get progress as percentage
    pub fn progress(&self) -> f64 {
        let total = self.params.total_points();
        if total == 0 {
            return 1.0;
        }
        self.state.total_visited as f64 / total as f64
    }
}

/// Spiral trajectory for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiralTrajectory {
    /// Points in the trajectory
    pub points: Vec<[f64; 5]>,
    /// Scores at each point
    pub scores: Vec<f64>,
    /// Layer boundaries
    pub layer_boundaries: Vec<usize>,
}

impl SpiralTrajectory {
    /// Create new empty trajectory
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            scores: Vec::new(),
            layer_boundaries: vec![0],
        }
    }

    /// Record a point
    pub fn record(&mut self, sig: &Signature5D, score: f64) {
        self.points.push([sig.psi, sig.rho, sig.omega, sig.chi, sig.eta]);
        self.scores.push(score);
    }

    /// Mark layer boundary
    pub fn mark_layer(&mut self) {
        self.layer_boundaries.push(self.points.len());
    }

    /// Get best point index
    pub fn best_index(&self) -> Option<usize> {
        self.scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
    }

    /// Get best score
    pub fn best_score(&self) -> Option<f64> {
        self.scores.iter().copied().fold(None, |max, x| {
            Some(max.map_or(x, |m: f64| m.max(x)))
        })
    }
}

impl Default for SpiralTrajectory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_engine_creation() {
        let params = SpiralParams::default();
        let engine = SpiralEngine::new(params);
        assert_eq!(engine.state().layer, 0);
    }

    #[test]
    fn test_next_point() {
        let params = SpiralParams {
            layers: 2,
            points_per_layer: 4,
            ..Default::default()
        };
        let mut engine = SpiralEngine::new(params);

        let mut count = 0;
        while engine.next_point().is_some() {
            count += 1;
        }

        assert_eq!(count, 8); // 2 layers * 4 points
    }

    #[test]
    fn test_progress() {
        let params = SpiralParams {
            layers: 2,
            points_per_layer: 4,
            ..Default::default()
        };
        let mut engine = SpiralEngine::new(params);

        assert_eq!(engine.progress(), 0.0);

        for _ in 0..4 {
            engine.next_point();
        }

        assert!((engine.progress() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_trajectory() {
        let mut traj = SpiralTrajectory::new();
        let sig = Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5);

        traj.record(&sig, 0.7);
        traj.record(&sig, 0.9);
        traj.record(&sig, 0.8);

        assert_eq!(traj.best_index(), Some(1));
        assert_eq!(traj.best_score(), Some(0.9));
    }
}
