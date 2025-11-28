//! Hypercube Edge Types
//!
//! Defines the edges (connections) between vertices in the Hypercube structure.

use crate::coordinates::Coord5D;
use crate::operators::OperatorType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of hypercube edge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeType {
    /// Standard lattice edge (unit distance in one dimension)
    Lattice,
    /// Diagonal edge (connects non-adjacent vertices)
    Diagonal,
    /// Operator application edge
    Operator,
    /// Expansion edge (from parent to child during expansion)
    Expansion,
    /// Compilation edge (in HDAG)
    Compilation,
    /// Virtual edge (for visualization/analysis)
    Virtual,
}

/// Weight/cost associated with an edge
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EdgeWeight {
    /// Distance component
    pub distance: f64,
    /// Resonance transition cost
    pub resonance_cost: f64,
    /// Operator application cost
    pub operator_cost: f64,
    /// Total combined weight
    pub total: f64,
}

impl EdgeWeight {
    /// Create a new edge weight
    pub fn new(distance: f64, resonance_cost: f64, operator_cost: f64) -> Self {
        let total = distance + resonance_cost + operator_cost;
        Self {
            distance,
            resonance_cost,
            operator_cost,
            total,
        }
    }

    /// Create unit weight
    pub fn unit() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// Create zero weight
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Create from distance only
    pub fn from_distance(d: f64) -> Self {
        Self::new(d, 0.0, 0.0)
    }

    /// Create from coordinates
    pub fn from_coords(from: &Coord5D, to: &Coord5D) -> Self {
        let distance = from.distance(to);
        let resonance_cost = (from.resonance() - to.resonance()).abs();
        Self::new(distance, resonance_cost, 0.0)
    }

    /// Add operator cost
    pub fn with_operator_cost(mut self, cost: f64) -> Self {
        self.operator_cost = cost;
        self.total = self.distance + self.resonance_cost + self.operator_cost;
        self
    }
}

impl Default for EdgeWeight {
    fn default() -> Self {
        Self::unit()
    }
}

/// An edge in the hypercube
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypercubeEdge {
    /// Unique identifier
    pub id: String,
    /// Source vertex ID
    pub from_id: String,
    /// Target vertex ID
    pub to_id: String,
    /// Edge type
    pub edge_type: EdgeType,
    /// Edge weight
    pub weight: EdgeWeight,
    /// Dimension along which this edge lies (for lattice edges)
    pub dimension: Option<usize>,
    /// Associated operator type (for operator edges)
    pub operator_type: Option<OperatorType>,
    /// Is edge directed
    pub directed: bool,
    /// Is edge active
    pub active: bool,
    /// Traversal count
    pub traversal_count: usize,
    /// Metadata
    pub metadata: EdgeMetadata,
}

/// Edge metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EdgeMetadata {
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Last traversal timestamp
    pub last_traversal: Option<String>,
    /// Average resonance gain when traversing
    pub avg_resonance_gain: f64,
    /// Tags
    pub tags: Vec<String>,
}

impl HypercubeEdge {
    /// Create a new edge
    pub fn new(from_id: &str, to_id: &str, edge_type: EdgeType, weight: EdgeWeight) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from_id: from_id.to_string(),
            to_id: to_id.to_string(),
            edge_type,
            weight,
            dimension: None,
            operator_type: None,
            directed: true,
            active: true,
            traversal_count: 0,
            metadata: EdgeMetadata::default(),
        }
    }

    /// Create a lattice edge
    pub fn lattice(from_id: &str, to_id: &str, dimension: usize, distance: f64) -> Self {
        let mut edge = Self::new(from_id, to_id, EdgeType::Lattice, EdgeWeight::from_distance(distance));
        edge.dimension = Some(dimension);
        edge.directed = false; // Lattice edges are bidirectional
        edge
    }

    /// Create an operator edge
    pub fn operator(from_id: &str, to_id: &str, op_type: OperatorType, cost: f64) -> Self {
        let mut edge = Self::new(from_id, to_id, EdgeType::Operator, EdgeWeight::zero().with_operator_cost(cost));
        edge.operator_type = Some(op_type);
        edge
    }

    /// Create an expansion edge
    pub fn expansion(from_id: &str, to_id: &str, weight: EdgeWeight) -> Self {
        Self::new(from_id, to_id, EdgeType::Expansion, weight)
    }

    /// Set dimension
    pub fn with_dimension(mut self, dim: usize) -> Self {
        self.dimension = Some(dim);
        self
    }

    /// Set operator type
    pub fn with_operator(mut self, op_type: OperatorType) -> Self {
        self.operator_type = Some(op_type);
        self
    }

    /// Set as undirected
    pub fn undirected(mut self) -> Self {
        self.directed = false;
        self
    }

    /// Record a traversal
    pub fn record_traversal(&mut self, resonance_gain: f64) {
        self.traversal_count += 1;

        // Update average resonance gain
        let n = self.traversal_count as f64;
        self.metadata.avg_resonance_gain =
            ((n - 1.0) * self.metadata.avg_resonance_gain + resonance_gain) / n;

        self.metadata.last_traversal = Some(chrono::Utc::now().to_rfc3339());
    }

    /// Check if edge connects two specific vertices (in either direction if undirected)
    pub fn connects(&self, v1: &str, v2: &str) -> bool {
        if self.from_id == v1 && self.to_id == v2 {
            return true;
        }
        if !self.directed && self.from_id == v2 && self.to_id == v1 {
            return true;
        }
        false
    }

    /// Get the other vertex ID given one end
    pub fn other_vertex(&self, vertex_id: &str) -> Option<&str> {
        if self.from_id == vertex_id {
            Some(&self.to_id)
        } else if self.to_id == vertex_id && !self.directed {
            Some(&self.from_id)
        } else {
            None
        }
    }

    /// Deactivate edge
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Activate edge
    pub fn activate(&mut self) {
        self.active = true;
    }
}

impl std::fmt::Display for HypercubeEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrow = if self.directed { "->" } else { "<->" };
        write!(f, "Edge[{}] {} {} {} ({:?}, w={:.3})",
            &self.id[..8], &self.from_id[..8], arrow, &self.to_id[..8],
            self.edge_type, self.weight.total)
    }
}

/// Edge builder for fluent construction
pub struct EdgeBuilder {
    edge: HypercubeEdge,
}

impl EdgeBuilder {
    /// Start building an edge
    pub fn new(from_id: &str, to_id: &str) -> Self {
        Self {
            edge: HypercubeEdge::new(from_id, to_id, EdgeType::Lattice, EdgeWeight::unit()),
        }
    }

    /// Set edge type
    pub fn edge_type(mut self, t: EdgeType) -> Self {
        self.edge.edge_type = t;
        self
    }

    /// Set weight
    pub fn weight(mut self, w: EdgeWeight) -> Self {
        self.edge.weight = w;
        self
    }

    /// Set dimension
    pub fn dimension(mut self, d: usize) -> Self {
        self.edge.dimension = Some(d);
        self
    }

    /// Set operator type
    pub fn operator(mut self, op: OperatorType) -> Self {
        self.edge.operator_type = Some(op);
        self.edge.edge_type = EdgeType::Operator;
        self
    }

    /// Set as undirected
    pub fn undirected(mut self) -> Self {
        self.edge.directed = false;
        self
    }

    /// Build the edge
    pub fn build(self) -> HypercubeEdge {
        self.edge
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_creation() {
        let edge = HypercubeEdge::lattice("v1", "v2", 0, 1.0);
        assert_eq!(edge.from_id, "v1");
        assert_eq!(edge.to_id, "v2");
        assert_eq!(edge.dimension, Some(0));
        assert!(!edge.directed);
    }

    #[test]
    fn test_edge_connects() {
        let edge = HypercubeEdge::lattice("v1", "v2", 0, 1.0);
        assert!(edge.connects("v1", "v2"));
        assert!(edge.connects("v2", "v1")); // Undirected
    }

    #[test]
    fn test_edge_weight() {
        let from = Coord5D::origin();
        let to = Coord5D::unit();
        let weight = EdgeWeight::from_coords(&from, &to);

        assert!(weight.distance > 0.0);
        assert!(weight.total > 0.0);
    }

    #[test]
    fn test_edge_builder() {
        let edge = EdgeBuilder::new("v1", "v2")
            .edge_type(EdgeType::Operator)
            .operator(OperatorType::DK)
            .weight(EdgeWeight::zero().with_operator_cost(0.5))
            .build();

        assert_eq!(edge.edge_type, EdgeType::Operator);
        assert_eq!(edge.operator_type, Some(OperatorType::DK));
    }
}
