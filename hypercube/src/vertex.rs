//! Hypercube Vertex Types
//!
//! Defines the vertices (nodes) of the Hypercube structure.

use crate::coordinates::Coord5D;
use crate::operators::OperatorFamily;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of hypercube vertex
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VertexType {
    /// Origin vertex at center of cube
    Origin,
    /// Corner vertex (2^5 = 32 corners in 5D)
    Corner,
    /// Edge midpoint vertex
    EdgeMidpoint,
    /// Face center vertex
    FaceCenter,
    /// Cell center vertex
    CellCenter,
    /// Hypercell center (4D cell)
    HypercellCenter,
    /// Custom/generated vertex
    Generated,
}

impl VertexType {
    /// Get the number of vertices of this type in a 5D hypercube
    pub fn count_in_hypercube(&self) -> usize {
        match self {
            VertexType::Origin => 1,
            VertexType::Corner => 32,       // 2^5
            VertexType::EdgeMidpoint => 80,  // C(5,1) * 2^4
            VertexType::FaceCenter => 80,    // C(5,2) * 2^3
            VertexType::CellCenter => 40,    // C(5,3) * 2^2
            VertexType::HypercellCenter => 10, // C(5,4) * 2^1
            VertexType::Generated => 0,      // Dynamic
        }
    }
}

/// State of a hypercube vertex
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VertexState {
    /// Vertex is inactive
    Inactive,
    /// Vertex is active and ready
    Active,
    /// Vertex is being processed
    Processing,
    /// Vertex has completed processing
    Completed,
    /// Vertex is in error state
    Error,
    /// Vertex is locked (no modifications)
    Locked,
}

/// A vertex in the hypercube
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypercubeVertex {
    /// Unique identifier
    pub id: String,
    /// Vertex type
    pub vertex_type: VertexType,
    /// 5D coordinate
    pub coordinate: Coord5D,
    /// Current state
    pub state: VertexState,
    /// Associated operator family (if any)
    pub operator_family: Option<String>,
    /// Resonance value at this vertex
    pub resonance: f64,
    /// Parent vertex ID (for expansion tracking)
    pub parent_id: Option<String>,
    /// Child vertex IDs
    pub children: Vec<String>,
    /// Depth in the hypercube hierarchy
    pub depth: usize,
    /// Metadata
    pub metadata: VertexMetadata,
}

/// Vertex metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VertexMetadata {
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Last update timestamp
    pub updated_at: Option<String>,
    /// Number of visits during search
    pub visit_count: usize,
    /// Best resonance achieved from this vertex
    pub best_resonance: f64,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Custom properties
    pub properties: std::collections::HashMap<String, String>,
}

impl HypercubeVertex {
    /// Create a new vertex
    pub fn new(vertex_type: VertexType, coordinate: Coord5D) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            vertex_type,
            coordinate,
            state: VertexState::Inactive,
            operator_family: None,
            resonance: coordinate.resonance(),
            parent_id: None,
            children: Vec::new(),
            depth: 0,
            metadata: VertexMetadata::default(),
        }
    }

    /// Create an origin vertex
    pub fn origin() -> Self {
        Self::new(VertexType::Origin, Coord5D::center())
    }

    /// Create a corner vertex at the given corner index (0-31)
    pub fn corner(index: usize) -> Self {
        let bits = index & 0b11111;
        let coord = Coord5D::new(
            if bits & 1 != 0 { 1.0 } else { 0.0 },
            if bits & 2 != 0 { 1.0 } else { 0.0 },
            if bits & 4 != 0 { 1.0 } else { 0.0 },
            if bits & 8 != 0 { 1.0 } else { 0.0 },
            if bits & 16 != 0 { 1.0 } else { 0.0 },
        );
        Self::new(VertexType::Corner, coord)
    }

    /// Create all 32 corner vertices
    pub fn all_corners() -> Vec<Self> {
        (0..32).map(Self::corner).collect()
    }

    /// Set vertex state
    pub fn set_state(&mut self, state: VertexState) {
        self.state = state;
        self.metadata.updated_at = Some(chrono::Utc::now().to_rfc3339());
    }

    /// Set parent vertex
    pub fn set_parent(&mut self, parent_id: &str) {
        self.parent_id = Some(parent_id.to_string());
    }

    /// Add a child vertex
    pub fn add_child(&mut self, child_id: &str) {
        self.children.push(child_id.to_string());
    }

    /// Check if vertex is active
    pub fn is_active(&self) -> bool {
        self.state == VertexState::Active
    }

    /// Check if vertex can be processed
    pub fn can_process(&self) -> bool {
        matches!(self.state, VertexState::Active | VertexState::Inactive)
    }

    /// Update resonance value
    pub fn update_resonance(&mut self) {
        self.resonance = self.coordinate.resonance();
        if self.resonance > self.metadata.best_resonance {
            self.metadata.best_resonance = self.resonance;
        }
    }

    /// Record a visit
    pub fn record_visit(&mut self) {
        self.metadata.visit_count += 1;
        self.metadata.updated_at = Some(chrono::Utc::now().to_rfc3339());
    }

    /// Get distance to another vertex
    pub fn distance_to(&self, other: &Self) -> f64 {
        self.coordinate.distance(&other.coordinate)
    }

    /// Check if this vertex is adjacent to another in the hypercube lattice
    /// Adjacent vertices differ in exactly one coordinate by 1
    pub fn is_adjacent(&self, other: &Self) -> bool {
        let diff = self.coordinate.sub(&other.coordinate);
        let arr = diff.to_array();

        let mut non_zero_count = 0;
        for &v in &arr {
            if v.abs() > 0.01 {
                if (v.abs() - 1.0).abs() > 0.01 {
                    return false; // Non-unit difference
                }
                non_zero_count += 1;
            }
        }

        non_zero_count == 1
    }

    /// Generate neighbors in the hypercube lattice
    pub fn generate_neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        let arr = self.coordinate.to_array();

        for dim in 0..5 {
            // Neighbor in positive direction
            if arr[dim] < 0.99 {
                let mut new_arr = arr;
                new_arr[dim] = (arr[dim] + 1.0).min(1.0);
                let mut neighbor = Self::new(VertexType::Generated, Coord5D::from_vec(&new_arr));
                neighbor.set_parent(&self.id);
                neighbor.depth = self.depth + 1;
                neighbors.push(neighbor);
            }

            // Neighbor in negative direction
            if arr[dim] > 0.01 {
                let mut new_arr = arr;
                new_arr[dim] = (arr[dim] - 1.0).max(0.0);
                let mut neighbor = Self::new(VertexType::Generated, Coord5D::from_vec(&new_arr));
                neighbor.set_parent(&self.id);
                neighbor.depth = self.depth + 1;
                neighbors.push(neighbor);
            }
        }

        neighbors
    }
}

impl std::fmt::Display for HypercubeVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vertex[{}] {:?} @ {} (R={:.3})",
            &self.id[..8], self.vertex_type, self.coordinate, self.resonance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corner_creation() {
        let corners = HypercubeVertex::all_corners();
        assert_eq!(corners.len(), 32);

        // Check corner 0 (all zeros)
        let c0 = &corners[0];
        assert_eq!(c0.coordinate.psi, 0.0);
        assert_eq!(c0.coordinate.eta, 0.0);

        // Check corner 31 (all ones)
        let c31 = &corners[31];
        assert_eq!(c31.coordinate.psi, 1.0);
        assert_eq!(c31.coordinate.eta, 1.0);
    }

    #[test]
    fn test_adjacency() {
        let v1 = HypercubeVertex::corner(0); // (0,0,0,0,0)
        let v2 = HypercubeVertex::corner(1); // (1,0,0,0,0)
        let v3 = HypercubeVertex::corner(3); // (1,1,0,0,0)

        assert!(v1.is_adjacent(&v2));
        assert!(!v1.is_adjacent(&v3));
    }

    #[test]
    fn test_neighbor_generation() {
        let origin = HypercubeVertex::origin();
        let neighbors = origin.generate_neighbors();

        // Center point should have 10 neighbors (5 dims * 2 directions)
        assert_eq!(neighbors.len(), 10);
    }
}
