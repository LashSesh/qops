//! Hypercube Structure
//!
//! The main 5D Hypercube structure that contains vertices, edges, and
//! supports self-compilation through the HDAG execution framework.

use crate::coordinates::{Coord5D, CoordinateSystem};
use crate::vertex::{HypercubeVertex, VertexState, VertexType};
use crate::edge::{HypercubeEdge, EdgeType, EdgeWeight};
use crate::operators::{Operator5D, OperatorFamily, OperatorType};
use crate::error::{HypercubeError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for the Hypercube
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypercubeConfig {
    /// Dimension of the hypercube (default: 5)
    pub dimension: usize,
    /// Include all corner vertices at initialization
    pub include_corners: bool,
    /// Include origin vertex
    pub include_origin: bool,
    /// Maximum depth for expansion
    pub max_depth: usize,
    /// Resonance threshold for vertex activation
    pub resonance_threshold: f64,
    /// Expansion rule to use
    pub expansion_rule: CubeExpansionRule,
}

impl Default for HypercubeConfig {
    fn default() -> Self {
        Self {
            dimension: 5,
            include_corners: true,
            include_origin: true,
            max_depth: 10,
            resonance_threshold: 0.5,
            expansion_rule: CubeExpansionRule::Triton,
        }
    }
}

/// Expansion rules for the hypercube
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CubeExpansionRule {
    /// Lattice-based expansion (all neighbors)
    Lattice,
    /// Resonance-guided expansion (high resonance neighbors first)
    ResonanceGuided,
    /// TRITON spiral expansion
    Triton,
    /// Operator-driven expansion
    OperatorDriven,
    /// Random exploration
    Random,
    /// Hybrid TRITON + Resonance
    HybridTriton,
}

/// State of the hypercube
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HypercubeState {
    /// Just created, not initialized
    Created,
    /// Initialized with structure
    Initialized,
    /// Expansion in progress
    Expanding,
    /// Ready for compilation
    ReadyToCompile,
    /// Compilation in progress
    Compiling,
    /// Compilation complete
    Compiled,
    /// Error state
    Error,
}

/// The main Hypercube structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypercube {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Configuration
    pub config: HypercubeConfig,
    /// Current state
    pub state: HypercubeState,
    /// Coordinate system
    pub coordinate_system: CoordinateSystem,
    /// Vertices indexed by ID
    pub vertices: HashMap<String, HypercubeVertex>,
    /// Edges indexed by ID
    pub edges: HashMap<String, HypercubeEdge>,
    /// Vertex adjacency (vertex_id -> list of edge_ids)
    pub adjacency: HashMap<String, Vec<String>>,
    /// Operator families associated with this cube
    #[serde(skip)]
    pub operator_families: Vec<OperatorFamily>,
    /// Best vertex found so far
    pub best_vertex_id: Option<String>,
    /// Best resonance achieved
    pub best_resonance: f64,
    /// Expansion iteration count
    pub expansion_iteration: usize,
    /// Statistics
    pub stats: HypercubeStats,
}

/// Statistics for the hypercube
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HypercubeStats {
    pub total_vertices: usize,
    pub active_vertices: usize,
    pub total_edges: usize,
    pub max_depth_reached: usize,
    pub avg_resonance: f64,
    pub expansion_steps: usize,
    pub operators_applied: usize,
}

impl Hypercube {
    /// Create a new hypercube
    pub fn new(name: &str, config: HypercubeConfig) -> Self {
        let mut cube = Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            config,
            state: HypercubeState::Created,
            coordinate_system: CoordinateSystem::standard(),
            vertices: HashMap::new(),
            edges: HashMap::new(),
            adjacency: HashMap::new(),
            operator_families: Vec::new(),
            best_vertex_id: None,
            best_resonance: 0.0,
            expansion_iteration: 0,
            stats: HypercubeStats::default(),
        };

        cube.initialize();
        cube
    }

    /// Create with default configuration
    pub fn default_cube(name: &str) -> Self {
        Self::new(name, HypercubeConfig::default())
    }

    /// Initialize the hypercube structure
    fn initialize(&mut self) {
        // Add origin if configured
        if self.config.include_origin {
            let origin = HypercubeVertex::origin();
            self.add_vertex(origin);
        }

        // Add corners if configured
        if self.config.include_corners {
            let corners = HypercubeVertex::all_corners();
            for corner in corners {
                self.add_vertex(corner);
            }

            // Create lattice edges between adjacent corners
            self.create_lattice_edges();
        }

        self.state = HypercubeState::Initialized;
        self.update_stats();
    }

    /// Add a vertex to the hypercube
    pub fn add_vertex(&mut self, mut vertex: HypercubeVertex) -> String {
        let id = vertex.id.clone();
        vertex.update_resonance();

        // Update best vertex tracking
        if vertex.resonance > self.best_resonance {
            self.best_resonance = vertex.resonance;
            self.best_vertex_id = Some(id.clone());
        }

        self.adjacency.insert(id.clone(), Vec::new());
        self.vertices.insert(id.clone(), vertex);
        self.stats.total_vertices += 1;

        id
    }

    /// Add an edge to the hypercube
    pub fn add_edge(&mut self, edge: HypercubeEdge) -> String {
        let id = edge.id.clone();

        // Update adjacency
        self.adjacency
            .entry(edge.from_id.clone())
            .or_default()
            .push(id.clone());

        if !edge.directed {
            self.adjacency
                .entry(edge.to_id.clone())
                .or_default()
                .push(id.clone());
        }

        self.edges.insert(id.clone(), edge);
        self.stats.total_edges += 1;

        id
    }

    /// Create lattice edges between existing vertices
    fn create_lattice_edges(&mut self) {
        let vertex_ids: Vec<String> = self.vertices.keys().cloned().collect();

        for i in 0..vertex_ids.len() {
            for j in (i + 1)..vertex_ids.len() {
                let v1 = &self.vertices[&vertex_ids[i]];
                let v2 = &self.vertices[&vertex_ids[j]];

                if v1.is_adjacent(v2) {
                    // Find which dimension differs
                    let arr1 = v1.coordinate.to_array();
                    let arr2 = v2.coordinate.to_array();
                    let mut dim = 0;
                    for d in 0..5 {
                        if (arr1[d] - arr2[d]).abs() > 0.01 {
                            dim = d;
                            break;
                        }
                    }

                    let edge = HypercubeEdge::lattice(&vertex_ids[i], &vertex_ids[j], dim, 1.0);
                    self.add_edge(edge);
                }
            }
        }
    }

    /// Get a vertex by ID
    pub fn get_vertex(&self, id: &str) -> Option<&HypercubeVertex> {
        self.vertices.get(id)
    }

    /// Get a mutable vertex by ID
    pub fn get_vertex_mut(&mut self, id: &str) -> Option<&mut HypercubeVertex> {
        self.vertices.get_mut(id)
    }

    /// Get an edge by ID
    pub fn get_edge(&self, id: &str) -> Option<&HypercubeEdge> {
        self.edges.get(id)
    }

    /// Get edges from a vertex
    pub fn edges_from(&self, vertex_id: &str) -> Vec<&HypercubeEdge> {
        self.adjacency
            .get(vertex_id)
            .map(|edge_ids| {
                edge_ids
                    .iter()
                    .filter_map(|eid| self.edges.get(eid))
                    .filter(|e| e.from_id == vertex_id || !e.directed)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get neighbors of a vertex
    pub fn neighbors(&self, vertex_id: &str) -> Vec<&HypercubeVertex> {
        self.edges_from(vertex_id)
            .iter()
            .filter_map(|edge| {
                let other_id = edge.other_vertex(vertex_id)?;
                self.vertices.get(other_id)
            })
            .collect()
    }

    /// Perform one expansion step
    pub fn expand_step(&mut self) -> Result<usize> {
        if self.expansion_iteration >= self.config.max_depth {
            return Ok(0);
        }

        self.state = HypercubeState::Expanding;
        let mut new_vertices = 0;

        match self.config.expansion_rule {
            CubeExpansionRule::Lattice => {
                new_vertices = self.expand_lattice()?;
            }
            CubeExpansionRule::ResonanceGuided => {
                new_vertices = self.expand_resonance_guided()?;
            }
            CubeExpansionRule::Triton => {
                new_vertices = self.expand_triton()?;
            }
            CubeExpansionRule::OperatorDriven => {
                new_vertices = self.expand_operator_driven()?;
            }
            CubeExpansionRule::Random => {
                new_vertices = self.expand_random()?;
            }
            CubeExpansionRule::HybridTriton => {
                new_vertices = self.expand_hybrid_triton()?;
            }
        }

        self.expansion_iteration += 1;
        self.stats.expansion_steps += 1;
        self.update_stats();

        if self.expansion_iteration >= self.config.max_depth {
            self.state = HypercubeState::ReadyToCompile;
        }

        Ok(new_vertices)
    }

    /// Expand using lattice rule (all neighbors)
    fn expand_lattice(&mut self) -> Result<usize> {
        let active_ids: Vec<String> = self.vertices
            .iter()
            .filter(|(_, v)| v.can_process() && v.depth < self.config.max_depth)
            .map(|(id, _)| id.clone())
            .collect();

        let mut count = 0;
        for vid in active_ids {
            if let Some(vertex) = self.vertices.get(&vid) {
                let new_neighbors = vertex.generate_neighbors();

                for mut neighbor in new_neighbors {
                    // Check if we already have a vertex at this coordinate
                    let exists = self.vertices.values().any(|v| {
                        v.coordinate.distance(&neighbor.coordinate) < 0.01
                    });

                    if !exists && neighbor.coordinate.resonance() >= self.config.resonance_threshold {
                        let from_id = vid.clone();
                        neighbor.set_parent(&from_id);

                        let weight = EdgeWeight::from_coords(
                            &vertex.coordinate,
                            &neighbor.coordinate,
                        );
                        let to_id = self.add_vertex(neighbor);

                        let edge = HypercubeEdge::expansion(&from_id, &to_id, weight);
                        self.add_edge(edge);
                        count += 1;
                    }
                }
            }

            // Mark vertex as processed
            if let Some(v) = self.vertices.get_mut(&vid) {
                v.set_state(VertexState::Completed);
            }
        }

        Ok(count)
    }

    /// Expand using resonance-guided rule
    fn expand_resonance_guided(&mut self) -> Result<usize> {
        // Sort vertices by resonance and expand from highest first
        let mut active: Vec<(String, f64)> = self.vertices
            .iter()
            .filter(|(_, v)| v.can_process() && v.depth < self.config.max_depth)
            .map(|(id, v)| (id.clone(), v.resonance))
            .collect();

        active.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let mut count = 0;
        let max_expand = 10; // Limit expansions per step

        for (vid, _) in active.into_iter().take(max_expand) {
            if let Some(vertex) = self.vertices.get(&vid) {
                let new_neighbors = vertex.generate_neighbors();

                // Sort neighbors by resonance potential
                let mut scored_neighbors: Vec<_> = new_neighbors
                    .into_iter()
                    .map(|n| {
                        let res = n.coordinate.resonance();
                        (n, res)
                    })
                    .collect();
                scored_neighbors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                for (mut neighbor, res) in scored_neighbors.into_iter().take(3) {
                    let exists = self.vertices.values().any(|v| {
                        v.coordinate.distance(&neighbor.coordinate) < 0.01
                    });

                    if !exists && res >= self.config.resonance_threshold {
                        let from_id = vid.clone();
                        neighbor.set_parent(&from_id);

                        let weight = EdgeWeight::from_coords(
                            &vertex.coordinate,
                            &neighbor.coordinate,
                        );
                        let to_id = self.add_vertex(neighbor);

                        let edge = HypercubeEdge::expansion(&from_id, &to_id, weight);
                        self.add_edge(edge);
                        count += 1;
                    }
                }
            }

            if let Some(v) = self.vertices.get_mut(&vid) {
                v.set_state(VertexState::Completed);
            }
        }

        Ok(count)
    }

    /// Expand using TRITON spiral rule
    fn expand_triton(&mut self) -> Result<usize> {
        use std::f64::consts::PI;

        // Golden ratio for spiral
        const PHI: f64 = 1.618033988749895;

        let best_id = self.best_vertex_id.clone();
        if best_id.is_none() {
            return self.expand_lattice();
        }

        let best_id = best_id.unwrap();
        let best_coord = self.vertices.get(&best_id)
            .map(|v| v.coordinate)
            .unwrap_or(Coord5D::center());

        let mut count = 0;
        let layer = self.expansion_iteration;
        let radius = (layer as f64 + 1.0) / 10.0;

        // Generate spiral points around best
        for i in 0..10 {
            let angle = 2.0 * PI * (i as f64) / 10.0 + (layer as f64) * PHI;

            // Create 5D spiral coordinates
            let offset = Coord5D::new(
                radius * angle.cos(),
                radius * angle.sin(),
                radius * (angle * 0.5).cos(),
                radius * (angle * 0.3).sin(),
                radius * (angle * 0.2).cos(),
            );

            let new_coord = best_coord.add(&offset).clamp_unit();
            let res = new_coord.resonance();

            // Check if position is new
            let exists = self.vertices.values().any(|v| {
                v.coordinate.distance(&new_coord) < 0.05
            });

            if !exists && res >= self.config.resonance_threshold {
                let mut vertex = HypercubeVertex::new(VertexType::Generated, new_coord);
                vertex.set_parent(&best_id);
                vertex.depth = layer + 1;

                let weight = EdgeWeight::from_coords(&best_coord, &new_coord);
                let to_id = self.add_vertex(vertex);

                let edge = HypercubeEdge::expansion(&best_id, &to_id, weight);
                self.add_edge(edge);
                count += 1;
            }
        }

        Ok(count)
    }

    /// Expand using operator-driven rule
    fn expand_operator_driven(&mut self) -> Result<usize> {
        use crate::operators::{
            DoubleKickOperator, SwapWaveOperator, PhaseIntegrationOperator, WeightTransformOperator
        };

        let active_ids: Vec<String> = self.vertices
            .iter()
            .filter(|(_, v)| v.can_process() && v.depth < self.config.max_depth)
            .map(|(id, _)| id.clone())
            .take(5)
            .collect();

        let operators: Vec<Box<dyn Operator5D>> = vec![
            Box::new(DoubleKickOperator::default()),
            Box::new(SwapWaveOperator::default()),
            Box::new(PhaseIntegrationOperator::default()),
            Box::new(WeightTransformOperator::default()),
        ];

        let mut count = 0;

        for vid in active_ids {
            if let Some(vertex) = self.vertices.get(&vid) {
                let coord = vertex.coordinate;

                for op in &operators {
                    let new_coord = op.apply(&coord);
                    let res = new_coord.resonance();

                    let exists = self.vertices.values().any(|v| {
                        v.coordinate.distance(&new_coord) < 0.05
                    });

                    if !exists && res >= self.config.resonance_threshold {
                        let mut new_vertex = HypercubeVertex::new(VertexType::Generated, new_coord);
                        new_vertex.set_parent(&vid);
                        new_vertex.depth = vertex.depth + 1;

                        let weight = EdgeWeight::from_coords(&coord, &new_coord)
                            .with_operator_cost(0.1);
                        let to_id = self.add_vertex(new_vertex);

                        let edge = HypercubeEdge::operator(&vid, &to_id, op.operator_type(), 0.1);
                        self.add_edge(edge);

                        self.stats.operators_applied += 1;
                        count += 1;
                    }
                }
            }

            if let Some(v) = self.vertices.get_mut(&vid) {
                v.set_state(VertexState::Completed);
            }
        }

        Ok(count)
    }

    /// Expand using random exploration
    fn expand_random(&mut self) -> Result<usize> {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut count = 0;

        for _ in 0..10 {
            let coord = Coord5D::new(
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
            );

            let res = coord.resonance();
            if res >= self.config.resonance_threshold {
                let exists = self.vertices.values().any(|v| {
                    v.coordinate.distance(&coord) < 0.1
                });

                if !exists {
                    let mut vertex = HypercubeVertex::new(VertexType::Generated, coord);
                    vertex.depth = self.expansion_iteration;
                    self.add_vertex(vertex);
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    /// Expand using hybrid TRITON + resonance
    fn expand_hybrid_triton(&mut self) -> Result<usize> {
        // Mix of TRITON spiral and resonance-guided
        let triton_count = self.expand_triton()?;
        let resonance_count = self.expand_resonance_guided()?;
        Ok(triton_count + resonance_count)
    }

    /// Update statistics
    fn update_stats(&mut self) {
        self.stats.total_vertices = self.vertices.len();
        self.stats.total_edges = self.edges.len();

        self.stats.active_vertices = self.vertices
            .values()
            .filter(|v| v.is_active())
            .count();

        if !self.vertices.is_empty() {
            self.stats.avg_resonance = self.vertices
                .values()
                .map(|v| v.resonance)
                .sum::<f64>() / self.vertices.len() as f64;
        }

        self.stats.max_depth_reached = self.vertices
            .values()
            .map(|v| v.depth)
            .max()
            .unwrap_or(0);
    }

    /// Get the best vertex
    pub fn best_vertex(&self) -> Option<&HypercubeVertex> {
        self.best_vertex_id.as_ref().and_then(|id| self.vertices.get(id))
    }

    /// Add an operator family
    pub fn add_operator_family(&mut self, family: OperatorFamily) {
        self.operator_families.push(family);
    }

    /// Get vertices sorted by resonance
    pub fn vertices_by_resonance(&self) -> Vec<&HypercubeVertex> {
        let mut vertices: Vec<_> = self.vertices.values().collect();
        vertices.sort_by(|a, b| b.resonance.partial_cmp(&a.resonance).unwrap());
        vertices
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| HypercubeError::SerializationError(e.to_string()))
    }

    /// Import from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| HypercubeError::SerializationError(e.to_string()))
    }
}

impl std::fmt::Display for Hypercube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hypercube[{}] '{}' ({:?}): {} vertices, {} edges, best_res={:.4}",
            &self.id[..8], self.name, self.state,
            self.vertices.len(), self.edges.len(), self.best_resonance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypercube_creation() {
        let cube = Hypercube::default_cube("test");
        assert_eq!(cube.state, HypercubeState::Initialized);
        assert!(!cube.vertices.is_empty());
    }

    #[test]
    fn test_hypercube_expansion() {
        let mut cube = Hypercube::default_cube("test");
        let initial_count = cube.vertices.len();

        let new_count = cube.expand_step().unwrap();
        assert!(cube.vertices.len() >= initial_count);
    }

    #[test]
    fn test_hypercube_best_tracking() {
        let cube = Hypercube::default_cube("test");
        assert!(cube.best_resonance > 0.0);
        assert!(cube.best_vertex_id.is_some());
    }
}
