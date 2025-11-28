//! TRITON Integration for Hypercube
//!
//! Integrates the TRITON spiral search optimizer as the cube expansion rule,
//! enabling the `--hypercube-triton-mode` functionality.

use crate::cube::{Hypercube, CubeExpansionRule};
use crate::coordinates::Coord5D;
use crate::vertex::{HypercubeVertex, VertexType};
use crate::edge::{HypercubeEdge, EdgeWeight};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Golden ratio for TRITON spiral
pub const PHI: f64 = 1.618033988749895;

/// Configuration for TRITON expansion mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TritonExpansionConfig {
    /// Initial spiral radius
    pub initial_radius: f64,
    /// Expansion rate per layer (default: golden ratio)
    pub expansion_rate: f64,
    /// Number of spiral layers
    pub max_layers: usize,
    /// Points per layer
    pub points_per_layer: usize,
    /// Temperature for annealing
    pub temperature: f64,
    /// Cooling rate
    pub cooling_rate: f64,
    /// Use adaptive radius
    pub adaptive_radius: bool,
    /// Resonance-guided bias
    pub resonance_bias: f64,
}

impl Default for TritonExpansionConfig {
    fn default() -> Self {
        Self {
            initial_radius: 0.1,
            expansion_rate: PHI,
            max_layers: 7,
            points_per_layer: 10,
            temperature: 1.0,
            cooling_rate: 0.95,
            adaptive_radius: true,
            resonance_bias: 0.3,
        }
    }
}

impl TritonExpansionConfig {
    /// Create fast config
    pub fn fast() -> Self {
        Self {
            max_layers: 3,
            points_per_layer: 5,
            ..Default::default()
        }
    }

    /// Create thorough config
    pub fn thorough() -> Self {
        Self {
            max_layers: 12,
            points_per_layer: 15,
            ..Default::default()
        }
    }
}

/// TRITON mode state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TritonState {
    /// Current layer
    pub current_layer: usize,
    /// Current temperature
    pub temperature: f64,
    /// Current radius
    pub radius: f64,
    /// Best coordinate found
    pub best_coord: Coord5D,
    /// Best resonance
    pub best_resonance: f64,
    /// History of resonances per layer
    pub layer_resonances: Vec<f64>,
    /// Convergence flag
    pub converged: bool,
}

impl TritonState {
    /// Create new TRITON state
    pub fn new(seed: Coord5D, config: &TritonExpansionConfig) -> Self {
        Self {
            current_layer: 0,
            temperature: config.temperature,
            radius: config.initial_radius,
            best_coord: seed,
            best_resonance: seed.resonance(),
            layer_resonances: Vec::new(),
            converged: false,
        }
    }

    /// Check if converged
    pub fn check_convergence(&mut self, threshold: f64) -> bool {
        if self.layer_resonances.len() < 3 {
            return false;
        }

        let n = self.layer_resonances.len();
        let recent: f64 = self.layer_resonances[n-3..].iter().sum::<f64>() / 3.0;
        let earlier: f64 = self.layer_resonances[..n-3.min(n)].iter().sum::<f64>()
            / (n - 3).max(1) as f64;

        let improvement = recent - earlier;
        self.converged = improvement.abs() < threshold;
        self.converged
    }
}

/// Hypercube TRITON mode integrator
pub struct HypercubeTritonMode {
    config: TritonExpansionConfig,
    state: Option<TritonState>,
}

impl HypercubeTritonMode {
    /// Create new TRITON mode
    pub fn new(config: TritonExpansionConfig) -> Self {
        Self {
            config,
            state: None,
        }
    }

    /// Create with default config
    pub fn default_mode() -> Self {
        Self::new(TritonExpansionConfig::default())
    }

    /// Initialize with a seed coordinate
    pub fn initialize(&mut self, seed: Coord5D) {
        self.state = Some(TritonState::new(seed, &self.config));
    }

    /// Execute one layer of TRITON expansion
    pub fn expand_layer(&mut self, cube: &mut Hypercube) -> Result<usize> {
        let state = self.state.as_mut()
            .ok_or_else(|| crate::error::HypercubeError::SessionError("TRITON not initialized".to_string()))?;

        if state.converged || state.current_layer >= self.config.max_layers {
            return Ok(0);
        }

        let center = state.best_coord;
        let layer = state.current_layer;
        let radius = state.radius;

        let mut new_vertices = 0;
        let mut layer_best_resonance = 0.0;

        // Generate spiral points for this layer
        for i in 0..self.config.points_per_layer {
            // 5D spiral angle computations
            let base_angle = 2.0 * PI * (i as f64) / self.config.points_per_layer as f64;
            let layer_offset = (layer as f64) * PHI;
            let angle = base_angle + layer_offset;

            // Create 5D offset using spiral pattern
            let offset = self.compute_spiral_offset(angle, radius, i, layer);
            let new_coord = center.add(&offset).clamp_unit();
            let resonance = new_coord.resonance();

            // Simulated annealing acceptance
            let accept = if resonance > state.best_resonance {
                true
            } else {
                let delta = state.best_resonance - resonance;
                let prob = (-delta / state.temperature).exp();
                rand::random::<f64>() < prob
            };

            if accept && resonance >= cube.config.resonance_threshold {
                // Check if position is new
                let exists = cube.vertices.values().any(|v| {
                    v.coordinate.distance(&new_coord) < 0.03
                });

                if !exists {
                    let mut vertex = HypercubeVertex::new(VertexType::Generated, new_coord);
                    vertex.depth = layer + 1;

                    if let Some(best_id) = &cube.best_vertex_id {
                        vertex.set_parent(best_id);
                    }

                    let weight = EdgeWeight::from_coords(&center, &new_coord);
                    let vertex_id = cube.add_vertex(vertex);

                    if let Some(best_id) = &cube.best_vertex_id {
                        let edge = HypercubeEdge::expansion(best_id, &vertex_id, weight);
                        cube.add_edge(edge);
                    }

                    new_vertices += 1;

                    // Update best if improved
                    if resonance > state.best_resonance {
                        state.best_resonance = resonance;
                        state.best_coord = new_coord;
                    }
                }
            }

            if resonance > layer_best_resonance {
                layer_best_resonance = resonance;
            }
        }

        // Update state
        state.layer_resonances.push(layer_best_resonance);
        state.current_layer += 1;
        state.temperature *= self.config.cooling_rate;

        // Adaptive radius
        if self.config.adaptive_radius {
            state.radius *= self.config.expansion_rate;

            // Contract if resonance improving
            if layer_best_resonance > state.best_resonance * 0.95 {
                state.radius *= 0.8;
            }
        } else {
            state.radius *= self.config.expansion_rate;
        }

        // Check convergence
        state.check_convergence(0.001);

        Ok(new_vertices)
    }

    /// Compute spiral offset in 5D
    fn compute_spiral_offset(&self, angle: f64, radius: f64, point_idx: usize, layer: usize) -> Coord5D {
        // Use different frequency multipliers for each dimension
        // to create interesting 5D spiral patterns
        let freq_offsets = [1.0, 0.7, 0.5, 0.3, 0.2];

        // Add some variation based on point index
        let point_offset = (point_idx as f64) * 0.1;

        Coord5D::new(
            radius * (angle * freq_offsets[0] + point_offset).cos(),
            radius * (angle * freq_offsets[1] + point_offset).sin(),
            radius * (angle * freq_offsets[2]).cos() * 0.8,
            radius * (angle * freq_offsets[3]).sin() * 0.6,
            radius * (angle * freq_offsets[4]).cos() * 0.4 * (-layer as f64 * 0.1).exp(),
        )
    }

    /// Run full TRITON expansion
    pub fn run(&mut self, cube: &mut Hypercube) -> Result<TritonExpansionResult> {
        let seed = cube.best_vertex()
            .map(|v| v.coordinate)
            .unwrap_or(Coord5D::center());

        self.initialize(seed);

        let mut total_vertices = 0;

        for _ in 0..self.config.max_layers {
            let new_count = self.expand_layer(cube)?;
            total_vertices += new_count;

            if self.state.as_ref().map(|s| s.converged).unwrap_or(true) {
                break;
            }
        }

        let state = self.state.as_ref().unwrap();

        Ok(TritonExpansionResult {
            best_coordinate: state.best_coord,
            best_resonance: state.best_resonance,
            layers_completed: state.current_layer,
            total_vertices_added: total_vertices,
            converged: state.converged,
            final_temperature: state.temperature,
            final_radius: state.radius,
            layer_resonances: state.layer_resonances.clone(),
        })
    }

    /// Get current state
    pub fn state(&self) -> Option<&TritonState> {
        self.state.as_ref()
    }
}

/// Result of TRITON expansion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TritonExpansionResult {
    /// Best coordinate found
    pub best_coordinate: Coord5D,
    /// Best resonance achieved
    pub best_resonance: f64,
    /// Number of layers completed
    pub layers_completed: usize,
    /// Total vertices added
    pub total_vertices_added: usize,
    /// Did search converge
    pub converged: bool,
    /// Final temperature
    pub final_temperature: f64,
    /// Final radius
    pub final_radius: f64,
    /// Resonance per layer
    pub layer_resonances: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triton_mode_creation() {
        let triton = HypercubeTritonMode::default_mode();
        assert!(triton.state.is_none());
    }

    #[test]
    fn test_triton_initialization() {
        let mut triton = HypercubeTritonMode::default_mode();
        triton.initialize(Coord5D::center());

        assert!(triton.state.is_some());
        let state = triton.state.as_ref().unwrap();
        assert_eq!(state.current_layer, 0);
    }

    #[test]
    fn test_triton_expansion() {
        let mut cube = Hypercube::default_cube("test");
        let mut triton = HypercubeTritonMode::new(TritonExpansionConfig::fast());

        let result = triton.run(&mut cube).unwrap();

        assert!(result.best_resonance > 0.0);
        assert!(result.layers_completed > 0);
    }

    #[test]
    fn test_spiral_offset() {
        let triton = HypercubeTritonMode::default_mode();
        let offset = triton.compute_spiral_offset(PI, 0.1, 0, 0);

        // Offset should be within expected range
        assert!(offset.psi.abs() <= 0.15);
        assert!(offset.rho.abs() <= 0.15);
    }
}
