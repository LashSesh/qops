//! MetatronGraph - 13-node quantum graph structure.

use nalgebra::DMatrix;
use qops_core::{ResonanceTopology, Signature, Signature3D};
use std::collections::HashMap;

/// 13-node Metatron Cube graph
#[derive(Debug, Clone)]
pub struct MetatronGraph {
    /// Adjacency matrix
    adjacency: DMatrix<f64>,
    /// Node signatures
    signatures: HashMap<usize, Signature>,
}

impl MetatronGraph {
    /// Create a new Metatron graph
    pub fn new() -> Self {
        let mut adjacency = DMatrix::zeros(13, 13);

        // Center node (0) connects to all hexagon nodes (1-6)
        for i in 1..=6 {
            adjacency[(0, i)] = 1.0;
            adjacency[(i, 0)] = 1.0;
        }

        // Hexagon nodes connect to adjacent hexagon nodes
        for i in 1..=6 {
            let next = if i == 6 { 1 } else { i + 1 };
            adjacency[(i, next)] = 1.0;
            adjacency[(next, i)] = 1.0;
        }

        // Hexagon nodes connect to cube nodes
        for i in 1..=6 {
            adjacency[(i, i + 6)] = 1.0;
            adjacency[(i + 6, i)] = 1.0;
        }

        // Cube nodes connect to adjacent cube nodes
        for i in 7..=12 {
            let next = if i == 12 { 7 } else { i + 1 };
            adjacency[(i, next)] = 1.0;
            adjacency[(next, i)] = 1.0;
        }

        // Initialize signatures
        let signatures: HashMap<usize, Signature> = (0..13)
            .map(|i| (i, Signature::D3(Signature3D::default())))
            .collect();

        Self { adjacency, signatures }
    }

    /// Get adjacency matrix
    pub fn adjacency_matrix(&self) -> &DMatrix<f64> {
        &self.adjacency
    }

    /// Get degree of a node
    pub fn node_degree(&self, node: usize) -> usize {
        if node >= 13 {
            return 0;
        }
        self.adjacency.row(node).iter().filter(|&&x| x > 0.0).count()
    }

    /// Get Laplacian matrix
    pub fn laplacian(&self) -> DMatrix<f64> {
        let mut laplacian = -self.adjacency.clone();
        for i in 0..13 {
            laplacian[(i, i)] = self.node_degree(i) as f64;
        }
        laplacian
    }
}

impl Default for MetatronGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ResonanceTopology for MetatronGraph {
    type NodeId = usize;

    fn node_count(&self) -> usize {
        13
    }

    fn edge_count(&self) -> usize {
        let sum: f64 = self.adjacency.iter().sum();
        (sum / 2.0) as usize
    }

    fn nodes(&self) -> Vec<Self::NodeId> {
        (0..13).collect()
    }

    fn neighbors(&self, node: &Self::NodeId) -> Vec<Self::NodeId> {
        if *node >= 13 {
            return Vec::new();
        }
        (0..13)
            .filter(|&i| self.adjacency[(*node, i)] > 0.0)
            .collect()
    }

    fn signature_at(&self, node: &Self::NodeId) -> Option<Signature> {
        self.signatures.get(node).cloned()
    }

    fn set_signature(&mut self, node: &Self::NodeId, signature: Signature) {
        if *node < 13 {
            self.signatures.insert(*node, signature);
        }
    }

    fn has_node(&self, node: &Self::NodeId) -> bool {
        *node < 13
    }

    fn has_edge(&self, from: &Self::NodeId, to: &Self::NodeId) -> bool {
        if *from >= 13 || *to >= 13 {
            return false;
        }
        self.adjacency[(*from, *to)] > 0.0
    }

    fn seed_node(&self) -> Self::NodeId {
        0 // Center node
    }

    fn adjacency(&self) -> HashMap<Self::NodeId, Vec<Self::NodeId>> {
        (0..13).map(|i| (i, self.neighbors(&i))).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metatron_graph() {
        let graph = MetatronGraph::new();

        assert_eq!(graph.node_count(), 13);
        assert!(graph.edge_count() > 0);
    }

    #[test]
    fn test_center_connectivity() {
        let graph = MetatronGraph::new();

        // Center connects to all hexagon nodes
        let neighbors = graph.neighbors(&0);
        assert_eq!(neighbors.len(), 6);
    }

    #[test]
    fn test_laplacian() {
        let graph = MetatronGraph::new();
        let laplacian = graph.laplacian();

        // Row sums should be zero
        for i in 0..13 {
            let row_sum: f64 = laplacian.row(i).iter().sum();
            assert!((row_sum).abs() < 1e-10);
        }
    }
}
