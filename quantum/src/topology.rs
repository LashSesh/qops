//! CUBE-13 Topology Engine
//!
//! Advanced topology analysis, operator embedding, and metrics for the
//! 13-node Metatron Cube geometry.

use crate::graph::MetatronGraph;
use nalgebra::DMatrix;
use qops_core::{Signature5D, resonance_5d, ResonanceTopology};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Node types in CUBE-13 topology
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cube13NodeType {
    /// Central node (index 0)
    Center,
    /// Hexagon nodes (indices 1-6)
    Hexagon,
    /// Outer cube nodes (indices 7-12)
    Cube,
}

impl Cube13NodeType {
    /// Get node type from index
    pub fn from_index(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::Center),
            1..=6 => Some(Self::Hexagon),
            7..=12 => Some(Self::Cube),
            _ => None,
        }
    }

    /// Get all nodes of this type
    pub fn indices(&self) -> Vec<usize> {
        match self {
            Self::Center => vec![0],
            Self::Hexagon => (1..=6).collect(),
            Self::Cube => (7..=12).collect(),
        }
    }
}

/// Cube-13 topology engine for advanced analysis
#[derive(Debug, Clone)]
pub struct Cube13Engine {
    graph: MetatronGraph,
    distance_matrix: DMatrix<f64>,
    embeddings: HashMap<usize, Signature5D>,
}

impl Cube13Engine {
    /// Create new Cube-13 engine
    pub fn new() -> Self {
        let graph = MetatronGraph::new();
        let distance_matrix = Self::compute_distances(&graph);

        Self {
            graph,
            distance_matrix,
            embeddings: HashMap::new(),
        }
    }

    /// Compute shortest path distances
    fn compute_distances(graph: &MetatronGraph) -> DMatrix<f64> {
        let adj = graph.adjacency_matrix();
        let n = 13;
        let mut dist = DMatrix::from_element(n, n, f64::INFINITY);

        // Initialize with adjacency
        for i in 0..n {
            dist[(i, i)] = 0.0;
            for j in 0..n {
                if adj[(i, j)] > 0.0 {
                    dist[(i, j)] = 1.0;
                }
            }
        }

        // Floyd-Warshall
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[(i, k)] + dist[(k, j)] < dist[(i, j)] {
                        dist[(i, j)] = dist[(i, k)] + dist[(k, j)];
                    }
                }
            }
        }

        dist
    }

    /// Get distance between two nodes
    pub fn distance(&self, from: usize, to: usize) -> f64 {
        if from >= 13 || to >= 13 {
            return f64::INFINITY;
        }
        self.distance_matrix[(from, to)]
    }

    /// Get graph diameter
    pub fn diameter(&self) -> f64 {
        let mut max: f64 = 0.0;
        for i in 0..13 {
            for j in 0..13 {
                if self.distance_matrix[(i, j)] < f64::INFINITY {
                    max = max.max(self.distance_matrix[(i, j)]);
                }
            }
        }
        max
    }

    /// Embed a signature into the topology
    pub fn embed(&mut self, node: usize, sig: Signature5D) {
        if node < 13 {
            self.embeddings.insert(node, sig);
        }
    }

    /// Get embedding at node
    pub fn embedding_at(&self, node: usize) -> Option<&Signature5D> {
        self.embeddings.get(&node)
    }

    /// Find best embedding position for a signature
    pub fn find_best_embedding(&self, sig: &Signature5D) -> usize {
        let _target_res = resonance_5d(sig);

        // Score each node based on neighborhood compatibility
        let scores: Vec<(usize, f64)> = (0..13)
            .map(|node| {
                let neighborhood_score = self.neighborhood_compatibility(node, sig);
                let centrality = self.node_centrality(node);
                let score = 0.6 * neighborhood_score + 0.4 * centrality;
                (node, score)
            })
            .collect();

        scores.iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(n, _)| *n)
            .unwrap_or(0)
    }

    /// Compute neighborhood compatibility
    fn neighborhood_compatibility(&self, node: usize, sig: &Signature5D) -> f64 {
        let neighbors = self.graph.neighbors(&node);
        if neighbors.is_empty() {
            return 0.5;
        }

        let avg_res: f64 = neighbors.iter()
            .filter_map(|n| self.embeddings.get(n))
            .map(|s| resonance_5d(s))
            .sum::<f64>() / neighbors.len() as f64;

        let sig_res = resonance_5d(sig);

        // Compatibility is higher when resonances are similar
        1.0 - (sig_res - avg_res).abs()
    }

    /// Compute node centrality
    pub fn node_centrality(&self, node: usize) -> f64 {
        if node >= 13 {
            return 0.0;
        }

        // Closeness centrality
        let total_dist: f64 = (0..13)
            .filter(|&i| i != node)
            .map(|i| self.distance_matrix[(node, i)])
            .filter(|&d| d < f64::INFINITY)
            .sum();

        if total_dist == 0.0 {
            return 0.0;
        }

        12.0 / total_dist
    }

    /// Compute commutation cluster around a node
    pub fn commutation_cluster(&self, center: usize, radius: usize) -> Vec<usize> {
        (0..13)
            .filter(|&n| self.distance(center, n) as usize <= radius)
            .collect()
    }

    /// Find resonance neighborhood (nodes with similar resonance)
    pub fn resonance_neighborhood(&self, target_res: f64, threshold: f64) -> Vec<usize> {
        (0..13)
            .filter(|n| {
                self.embeddings.get(n)
                    .map(|s| (resonance_5d(s) - target_res).abs() < threshold)
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Extract operator families from embeddings
    pub fn extract_operator_families(&self, threshold: f64) -> Vec<OperatorCluster> {
        let mut clusters: Vec<OperatorCluster> = Vec::new();

        for (node, sig) in &self.embeddings {
            let mut found = false;

            for cluster in &mut clusters {
                if cluster.is_compatible(sig, threshold) {
                    cluster.add_member(*node, *sig);
                    found = true;
                    break;
                }
            }

            if !found {
                let mut cluster = OperatorCluster::new();
                cluster.add_member(*node, *sig);
                clusters.push(cluster);
            }
        }

        clusters
    }

    /// Run topology walk
    pub fn topology_walk(&self, start: usize, steps: usize) -> TopologyWalkResult {
        let mut path = vec![start];
        let mut current = start;
        let mut rng = rand::thread_rng();
        use rand::seq::SliceRandom;

        for _ in 0..steps {
            let neighbors = self.graph.neighbors(&current);
            if neighbors.is_empty() {
                break;
            }

            // Choose next node based on resonance gradient
            let next = if let Some(best) = neighbors.iter()
                .filter_map(|n| self.embeddings.get(n).map(|s| (*n, resonance_5d(s))))
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            {
                best.0
            } else {
                *neighbors.choose(&mut rng).unwrap()
            };

            path.push(next);
            current = next;
        }

        let visited: HashSet<usize> = path.iter().copied().collect();
        let coverage = visited.len() as f64 / 13.0;
        let path_len = path.len();

        TopologyWalkResult {
            path,
            coverage,
            final_node: current,
            steps_taken: steps.min(path_len.saturating_sub(1)),
        }
    }

    /// Compute topology metrics
    pub fn compute_metrics(&self) -> TopologyMetrics {
        let avg_resonance = if self.embeddings.is_empty() {
            0.0
        } else {
            self.embeddings.values()
                .map(|s| resonance_5d(s))
                .sum::<f64>() / self.embeddings.len() as f64
        };

        let center_res = self.embeddings.get(&0)
            .map(|s| resonance_5d(s))
            .unwrap_or(0.0);

        let hexagon_res: f64 = (1..=6)
            .filter_map(|n| self.embeddings.get(&n))
            .map(|s| resonance_5d(s))
            .sum::<f64>() / 6.0;

        let cube_res: f64 = (7..=12)
            .filter_map(|n| self.embeddings.get(&n))
            .map(|s| resonance_5d(s))
            .sum::<f64>() / 6.0;

        // Compute coherence (variance of resonances)
        let variance = if self.embeddings.len() > 1 {
            self.embeddings.values()
                .map(|s| (resonance_5d(s) - avg_resonance).powi(2))
                .sum::<f64>() / self.embeddings.len() as f64
        } else {
            0.0
        };

        let coherence = 1.0 - variance.sqrt();

        TopologyMetrics {
            avg_resonance,
            center_resonance: center_res,
            hexagon_avg_resonance: hexagon_res,
            cube_avg_resonance: cube_res,
            coherence,
            embedding_count: self.embeddings.len(),
            coverage: self.embeddings.len() as f64 / 13.0,
        }
    }

    /// Get underlying graph
    pub fn graph(&self) -> &MetatronGraph {
        &self.graph
    }
}

impl Default for Cube13Engine {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a topology walk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyWalkResult {
    /// Path taken
    pub path: Vec<usize>,
    /// Coverage (fraction of nodes visited)
    pub coverage: f64,
    /// Final node
    pub final_node: usize,
    /// Steps actually taken
    pub steps_taken: usize,
}

/// Metrics for topology analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyMetrics {
    /// Average resonance across all embeddings
    pub avg_resonance: f64,
    /// Center node resonance
    pub center_resonance: f64,
    /// Average hexagon node resonance
    pub hexagon_avg_resonance: f64,
    /// Average cube node resonance
    pub cube_avg_resonance: f64,
    /// Coherence (low variance = high coherence)
    pub coherence: f64,
    /// Number of embedded operators
    pub embedding_count: usize,
    /// Coverage (fraction of nodes with embeddings)
    pub coverage: f64,
}

/// Cluster of operators in topology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorCluster {
    /// Cluster ID
    pub id: String,
    /// Members (node index, signature)
    members: Vec<(usize, Signature5D)>,
    /// Centroid signature
    centroid: Option<Signature5D>,
}

impl OperatorCluster {
    /// Create new cluster
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            members: Vec::new(),
            centroid: None,
        }
    }

    /// Add member
    pub fn add_member(&mut self, node: usize, sig: Signature5D) {
        self.members.push((node, sig));
        self.update_centroid();
    }

    /// Check compatibility
    pub fn is_compatible(&self, sig: &Signature5D, threshold: f64) -> bool {
        if let Some(centroid) = &self.centroid {
            Self::signature_distance(sig, centroid) < threshold
        } else {
            true
        }
    }

    fn signature_distance(a: &Signature5D, b: &Signature5D) -> f64 {
        ((a.psi - b.psi).powi(2) +
         (a.rho - b.rho).powi(2) +
         (a.omega - b.omega).powi(2) +
         (a.chi - b.chi).powi(2) +
         (a.eta - b.eta).powi(2)).sqrt()
    }

    fn update_centroid(&mut self) {
        if self.members.is_empty() {
            self.centroid = None;
            return;
        }

        let n = self.members.len() as f64;
        let mut sum = Signature5D::new(0.0, 0.0, 0.0, 0.0, 0.0);

        for (_, sig) in &self.members {
            sum.psi += sig.psi / n;
            sum.rho += sig.rho / n;
            sum.omega += sig.omega / n;
            sum.chi += sig.chi / n;
            sum.eta += sig.eta / n;
        }

        self.centroid = Some(sum);
    }

    /// Get members
    pub fn members(&self) -> &[(usize, Signature5D)] {
        &self.members
    }

    /// Get centroid
    pub fn centroid(&self) -> Option<&Signature5D> {
        self.centroid.as_ref()
    }

    /// Get average resonance
    pub fn avg_resonance(&self) -> f64 {
        if self.members.is_empty() {
            return 0.0;
        }
        self.members.iter()
            .map(|(_, s)| resonance_5d(s))
            .sum::<f64>() / self.members.len() as f64
    }
}

impl Default for OperatorCluster {
    fn default() -> Self {
        Self::new()
    }
}

/// Interactive topology exploration state
#[derive(Debug, Clone)]
pub struct TopologyExplorer {
    engine: Cube13Engine,
    current_node: usize,
    exploration_history: Vec<usize>,
    #[allow(dead_code)]
    discovered_clusters: Vec<OperatorCluster>,
}

impl TopologyExplorer {
    /// Create new explorer
    pub fn new() -> Self {
        Self {
            engine: Cube13Engine::new(),
            current_node: 0,
            exploration_history: vec![0],
            discovered_clusters: Vec::new(),
        }
    }

    /// Move to a node
    pub fn move_to(&mut self, node: usize) -> bool {
        if node >= 13 {
            return false;
        }

        // Check if adjacent or allow any move
        let neighbors = self.engine.graph().neighbors(&self.current_node);
        if neighbors.contains(&node) || self.exploration_history.len() == 1 {
            self.current_node = node;
            self.exploration_history.push(node);
            true
        } else {
            false
        }
    }

    /// Get current position
    pub fn current(&self) -> usize {
        self.current_node
    }

    /// Get neighbors of current node
    pub fn neighbors(&self) -> Vec<usize> {
        self.engine.graph().neighbors(&self.current_node)
    }

    /// Embed signature at current node
    pub fn embed_here(&mut self, sig: Signature5D) {
        self.engine.embed(self.current_node, sig);
    }

    /// Get exploration coverage
    pub fn coverage(&self) -> f64 {
        let visited: HashSet<usize> = self.exploration_history.iter().copied().collect();
        visited.len() as f64 / 13.0
    }

    /// Get metrics
    pub fn metrics(&self) -> TopologyMetrics {
        self.engine.compute_metrics()
    }

    /// Reset to center
    pub fn reset(&mut self) {
        self.current_node = 0;
        self.exploration_history = vec![0];
    }
}

impl Default for TopologyExplorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube13_engine() {
        let engine = Cube13Engine::new();
        assert_eq!(engine.diameter(), 3.0); // Max distance in cube-13
    }

    #[test]
    fn test_embedding() {
        let mut engine = Cube13Engine::new();
        let sig = Signature5D::new(0.8, 0.7, 0.6, 0.5, 0.2);

        engine.embed(0, sig);
        assert!(engine.embedding_at(0).is_some());
    }

    #[test]
    fn test_topology_walk() {
        let mut engine = Cube13Engine::new();

        // Embed some signatures
        for i in 0..13 {
            let sig = Signature5D::new(
                0.5 + (i as f64 * 0.03),
                0.5,
                0.5,
                0.5,
                0.2,
            );
            engine.embed(i, sig);
        }

        let result = engine.topology_walk(0, 10);
        assert!(!result.path.is_empty());
        assert!(result.coverage > 0.0);
    }

    #[test]
    fn test_explorer() {
        let mut explorer = TopologyExplorer::new();

        assert_eq!(explorer.current(), 0);
        assert!(explorer.move_to(1)); // Hexagon node
        assert_eq!(explorer.current(), 1);
    }
}
