//! Topology traits for resonance-based graph structures.
//!
//! This module defines the [`ResonanceTopology`] trait that abstracts over
//! different graph structures used in QOPS:
//! - S7 (5040 nodes) for Genesis/MOGE operator mining
//! - Cube-13 (13 nodes) for QSO quantum algorithms

use crate::signature::Signature;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

/// Signature associated with a node in the topology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSignature<Id: Clone> {
    /// Node identifier
    pub id: Id,
    /// Node's signature
    pub signature: Signature,
    /// Stability metric
    pub stability: f64,
    /// Whether this node is in a Mandorla zone
    pub is_mandorla: bool,
}

impl<Id: Clone> NodeSignature<Id> {
    /// Create a new node signature
    pub fn new(id: Id, signature: Signature) -> Self {
        Self {
            id,
            signature,
            stability: 0.5,
            is_mandorla: false,
        }
    }

    /// Resonance score for this node
    pub fn resonance(&self) -> f64 {
        self.signature.resonance()
    }
}

/// Trait for resonance-based graph topologies
///
/// This trait abstracts the graph structure, allowing different implementations:
/// - S7 permutation graph (5040 nodes)
/// - Metatron Cube graph (13 nodes)
/// - Custom topologies
pub trait ResonanceTopology: Clone + Send + Sync {
    /// Type for node identifiers
    type NodeId: Clone + Eq + Hash + Send + Sync;

    /// Total number of nodes in the topology
    fn node_count(&self) -> usize;

    /// Number of edges in the topology
    fn edge_count(&self) -> usize;

    /// Get all node IDs
    fn nodes(&self) -> Vec<Self::NodeId>;

    /// Get neighbors of a node
    fn neighbors(&self, node: &Self::NodeId) -> Vec<Self::NodeId>;

    /// Get the signature at a node
    fn signature_at(&self, node: &Self::NodeId) -> Option<Signature>;

    /// Set the signature at a node
    fn set_signature(&mut self, node: &Self::NodeId, signature: Signature);

    /// Compute resonance distance between two nodes
    fn resonance_distance(&self, a: &Self::NodeId, b: &Self::NodeId) -> f64 {
        match (self.signature_at(a), self.signature_at(b)) {
            (Some(sig_a), Some(sig_b)) => {
                let a_5d = sig_a.to_5d();
                let b_5d = sig_b.to_5d();
                a_5d.distance(&b_5d)
            }
            _ => f64::INFINITY,
        }
    }

    /// Get the degree of a node
    fn degree(&self, node: &Self::NodeId) -> usize {
        self.neighbors(node).len()
    }

    /// Check if a node exists
    fn has_node(&self, node: &Self::NodeId) -> bool;

    /// Check if an edge exists between two nodes
    fn has_edge(&self, from: &Self::NodeId, to: &Self::NodeId) -> bool;

    /// Get a seed/starting node (typically the center or root)
    fn seed_node(&self) -> Self::NodeId;

    /// Get the adjacency list
    fn adjacency(&self) -> HashMap<Self::NodeId, Vec<Self::NodeId>>;
}

/// Simple in-memory implementation of ResonanceTopology for testing
#[derive(Debug, Clone)]
pub struct SimpleTopology {
    nodes: Vec<usize>,
    edges: Vec<(usize, usize)>,
    signatures: HashMap<usize, Signature>,
}

impl SimpleTopology {
    /// Create a new empty topology
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            signatures: HashMap::new(),
        }
    }

    /// Create a complete graph with n nodes
    pub fn complete(n: usize) -> Self {
        let nodes: Vec<usize> = (0..n).collect();
        let mut edges = Vec::new();

        for i in 0..n {
            for j in (i + 1)..n {
                edges.push((i, j));
            }
        }

        let signatures: HashMap<usize, Signature> = nodes
            .iter()
            .map(|&id| (id, Signature::default()))
            .collect();

        Self {
            nodes,
            edges,
            signatures,
        }
    }

    /// Add a node
    pub fn add_node(&mut self, id: usize) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
            self.signatures.insert(id, Signature::default());
        }
    }

    /// Add an edge
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if !self.edges.contains(&(from, to)) && !self.edges.contains(&(to, from)) {
            self.edges.push((from, to));
        }
    }
}

impl Default for SimpleTopology {
    fn default() -> Self {
        Self::new()
    }
}

impl ResonanceTopology for SimpleTopology {
    type NodeId = usize;

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn nodes(&self) -> Vec<Self::NodeId> {
        self.nodes.clone()
    }

    fn neighbors(&self, node: &Self::NodeId) -> Vec<Self::NodeId> {
        self.edges
            .iter()
            .filter_map(|&(a, b)| {
                if a == *node {
                    Some(b)
                } else if b == *node {
                    Some(a)
                } else {
                    None
                }
            })
            .collect()
    }

    fn signature_at(&self, node: &Self::NodeId) -> Option<Signature> {
        self.signatures.get(node).cloned()
    }

    fn set_signature(&mut self, node: &Self::NodeId, signature: Signature) {
        if self.nodes.contains(node) {
            self.signatures.insert(*node, signature);
        }
    }

    fn has_node(&self, node: &Self::NodeId) -> bool {
        self.nodes.contains(node)
    }

    fn has_edge(&self, from: &Self::NodeId, to: &Self::NodeId) -> bool {
        self.edges.contains(&(*from, *to)) || self.edges.contains(&(*to, *from))
    }

    fn seed_node(&self) -> Self::NodeId {
        self.nodes.first().copied().unwrap_or(0)
    }

    fn adjacency(&self) -> HashMap<Self::NodeId, Vec<Self::NodeId>> {
        let mut adj = HashMap::new();
        for &node in &self.nodes {
            adj.insert(node, self.neighbors(&node));
        }
        adj
    }
}

/// Topology metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyMetrics {
    /// Total number of nodes
    pub node_count: usize,
    /// Total number of edges
    pub edge_count: usize,
    /// Average degree
    pub avg_degree: f64,
    /// Maximum degree
    pub max_degree: usize,
    /// Average resonance
    pub avg_resonance: f64,
    /// Nodes in Mandorla zones
    pub mandorla_count: usize,
}

impl TopologyMetrics {
    /// Compute metrics for a topology
    pub fn compute<T: ResonanceTopology>(topology: &T) -> Self {
        let nodes = topology.nodes();
        let node_count = nodes.len();
        let edge_count = topology.edge_count();

        let degrees: Vec<usize> = nodes.iter().map(|n| topology.degree(n)).collect();
        let avg_degree = if node_count > 0 {
            degrees.iter().sum::<usize>() as f64 / node_count as f64
        } else {
            0.0
        };
        let max_degree = degrees.into_iter().max().unwrap_or(0);

        let resonances: Vec<f64> = nodes
            .iter()
            .filter_map(|n| topology.signature_at(n).map(|s| s.resonance()))
            .collect();
        let avg_resonance = if !resonances.is_empty() {
            resonances.iter().sum::<f64>() / resonances.len() as f64
        } else {
            0.0
        };

        Self {
            node_count,
            edge_count,
            avg_degree,
            max_degree,
            avg_resonance,
            mandorla_count: 0, // Would be computed by specific implementations
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_topology() {
        let topo = SimpleTopology::complete(5);

        assert_eq!(topo.node_count(), 5);
        assert_eq!(topo.edge_count(), 10); // Complete graph has n*(n-1)/2 edges
    }

    #[test]
    fn test_neighbors() {
        let topo = SimpleTopology::complete(4);

        let neighbors = topo.neighbors(&0);
        assert_eq!(neighbors.len(), 3);
    }

    #[test]
    fn test_signatures() {
        let mut topo = SimpleTopology::complete(3);

        let new_sig = Signature::D3(crate::signature::Signature3D::new(0.9, 0.8, 0.7));
        topo.set_signature(&0, new_sig);

        let retrieved = topo.signature_at(&0).unwrap();
        assert!((retrieved.psi() - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_metrics() {
        let topo = SimpleTopology::complete(5);
        let metrics = TopologyMetrics::compute(&topo);

        assert_eq!(metrics.node_count, 5);
        assert_eq!(metrics.max_degree, 4);
    }
}
