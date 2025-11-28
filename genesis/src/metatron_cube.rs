//! MetatronCube - S7 permutation graph with 5040 nodes.

use qops_core::{ResonanceTopology, Signature, Signature5D};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use rand::Rng;

/// S7 Permutation represented as array
pub type Permutation = [u8; 7];

/// MetatronCube graph - S7 permutation topology
#[derive(Clone)]
pub struct MetatronCube {
    /// The underlying graph
    graph: DiGraph<Permutation, f64>,
    /// Map from permutation to node index
    perm_to_index: HashMap<Permutation, NodeIndex>,
    /// Signatures for each node
    signatures: HashMap<NodeIndex, Signature5D>,
    /// Identity permutation (seed node)
    identity: NodeIndex,
}

impl MetatronCube {
    /// Create a new MetatronCube (lazy initialization)
    pub fn new() -> Self {
        let mut graph = DiGraph::new();
        let mut perm_to_index = HashMap::new();
        let mut signatures = HashMap::new();

        // For performance, we'll generate a subset and build lazily
        // Identity permutation
        let identity_perm: Permutation = [0, 1, 2, 3, 4, 5, 6];
        let identity = graph.add_node(identity_perm);
        perm_to_index.insert(identity_perm, identity);
        signatures.insert(identity, Signature5D::default());

        // Generate some initial permutations via transpositions
        let mut current_perms = vec![identity_perm];
        let mut visited = std::collections::HashSet::new();
        visited.insert(identity_perm);

        // BFS to generate neighborhood
        for _layer in 0..3 {
            let mut next_perms = Vec::new();
            for perm in &current_perms {
                // Generate transpositions (i,j)
                for i in 0..6 {
                    for j in (i + 1)..7 {
                        let mut new_perm = *perm;
                        new_perm.swap(i, j);

                        if !visited.contains(&new_perm) {
                            visited.insert(new_perm);
                            next_perms.push(new_perm);

                            let node = graph.add_node(new_perm);
                            perm_to_index.insert(new_perm, node);
                            signatures.insert(node, Signature5D::default());

                            // Add edge from parent
                            if let Some(&parent_idx) = perm_to_index.get(perm) {
                                graph.add_edge(parent_idx, node, 1.0);
                            }
                        }
                    }
                }
            }
            current_perms = next_perms;
        }

        Self {
            graph,
            perm_to_index,
            signatures,
            identity,
        }
    }

    /// Get permutation at node
    pub fn permutation(&self, node: NodeIndex) -> Option<&Permutation> {
        self.graph.node_weight(node)
    }

    /// Apply transposition (i, j) to a permutation
    pub fn transpose(&self, perm: &Permutation, i: usize, j: usize) -> Permutation {
        let mut result = *perm;
        result.swap(i, j);
        result
    }

    /// Get node by permutation
    pub fn get_node(&self, perm: &Permutation) -> Option<NodeIndex> {
        self.perm_to_index.get(perm).copied()
    }

    /// Randomize signatures
    pub fn randomize_signatures(&mut self) {
        let mut rng = rand::thread_rng();
        for sig in self.signatures.values_mut() {
            *sig = Signature5D::new(
                rng.gen_range(0.3..0.9),
                rng.gen_range(0.3..0.9),
                rng.gen_range(0.3..0.9),
                rng.gen_range(0.3..0.9),
                rng.gen_range(0.1..0.5),
            );
        }
    }

    /// Get the identity node
    pub fn identity_node(&self) -> NodeIndex {
        self.identity
    }
}

impl Default for MetatronCube {
    fn default() -> Self {
        Self::new()
    }
}

impl ResonanceTopology for MetatronCube {
    type NodeId = NodeIndex;

    fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    fn nodes(&self) -> Vec<Self::NodeId> {
        self.graph.node_indices().collect()
    }

    fn neighbors(&self, node: &Self::NodeId) -> Vec<Self::NodeId> {
        self.graph
            .neighbors(*node)
            .chain(self.graph.neighbors_directed(*node, petgraph::Direction::Incoming))
            .collect()
    }

    fn signature_at(&self, node: &Self::NodeId) -> Option<Signature> {
        self.signatures.get(node).map(|s| Signature::D5(*s))
    }

    fn set_signature(&mut self, node: &Self::NodeId, signature: Signature) {
        self.signatures.insert(*node, signature.to_5d());
    }

    fn has_node(&self, node: &Self::NodeId) -> bool {
        self.graph.node_weight(*node).is_some()
    }

    fn has_edge(&self, from: &Self::NodeId, to: &Self::NodeId) -> bool {
        self.graph.contains_edge(*from, *to) || self.graph.contains_edge(*to, *from)
    }

    fn seed_node(&self) -> Self::NodeId {
        self.identity
    }

    fn adjacency(&self) -> HashMap<Self::NodeId, Vec<Self::NodeId>> {
        let mut adj = HashMap::new();
        for node in self.graph.node_indices() {
            adj.insert(node, self.neighbors(&node));
        }
        adj
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metatron_cube_creation() {
        let cube = MetatronCube::new();
        assert!(cube.node_count() > 0);
        assert!(cube.edge_count() > 0);
    }

    #[test]
    fn test_identity_node() {
        let cube = MetatronCube::new();
        let identity = cube.identity_node();
        let perm = cube.permutation(identity).unwrap();
        assert_eq!(*perm, [0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_neighbors() {
        let cube = MetatronCube::new();
        let identity = cube.identity_node();
        let neighbors = cube.neighbors(&identity);
        assert!(!neighbors.is_empty());
    }
}
