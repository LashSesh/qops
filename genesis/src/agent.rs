//! Traversal agents for operator mining.

use crate::metatron_cube::MetatronCube;
use qops_core::{ResonanceTopology, Signature5D, resonance_5d};
use petgraph::graph::NodeIndex;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Traversal strategy for agents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraversalStrategy {
    /// Maximize spectral quality (ψ)
    GradientAscent,
    /// Maximize structural coherence (ω)
    StabilityMaximization,
    /// Seek self-similar patterns
    CycleRecognition,
    /// Weighted combination of all metrics
    Balanced,
    /// Uniform random walk
    Random,
}

impl Default for TraversalStrategy {
    fn default() -> Self {
        Self::Balanced
    }
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Maximum steps per traversal
    pub max_steps: usize,
    /// Strategy to use
    pub strategy: TraversalStrategy,
    /// Exploration rate (for epsilon-greedy)
    pub exploration_rate: f64,
    /// Target resonance threshold
    pub target_resonance: f64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_steps: 100,
            strategy: TraversalStrategy::Balanced,
            exploration_rate: 0.1,
            target_resonance: 0.85,
        }
    }
}

/// Traversal agent for operator mining
#[derive(Debug, Clone)]
pub struct Agent {
    /// Agent ID
    pub id: uuid::Uuid,
    /// Current position in the graph
    pub position: NodeIndex,
    /// Path taken so far
    pub path: Vec<NodeIndex>,
    /// Configuration
    pub config: AgentConfig,
    /// Current signature
    pub signature: Signature5D,
    /// Steps taken
    pub steps: usize,
    /// Best resonance encountered
    pub best_resonance: f64,
}

impl Agent {
    /// Create a new agent at a starting position
    pub fn new(start: NodeIndex, config: AgentConfig) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            position: start,
            path: vec![start],
            config,
            signature: Signature5D::default(),
            steps: 0,
            best_resonance: 0.0,
        }
    }

    /// Take a single step in the graph
    pub fn step(&mut self, cube: &MetatronCube) -> Option<NodeIndex> {
        if self.steps >= self.config.max_steps {
            return None;
        }

        let neighbors = cube.neighbors(&self.position);
        if neighbors.is_empty() {
            return None;
        }

        // Choose next node based on strategy
        let next = match self.config.strategy {
            TraversalStrategy::Random => self.random_step(&neighbors),
            TraversalStrategy::GradientAscent => self.gradient_step(cube, &neighbors),
            TraversalStrategy::StabilityMaximization => self.stability_step(cube, &neighbors),
            TraversalStrategy::CycleRecognition => self.cycle_step(&neighbors),
            TraversalStrategy::Balanced => self.balanced_step(cube, &neighbors),
        };

        // Update state
        self.position = next;
        self.path.push(next);
        self.steps += 1;

        // Update signature from new position
        if let Some(sig) = cube.signature_at(&next) {
            self.signature = sig.to_5d();
            let res = resonance_5d(&self.signature);
            if res > self.best_resonance {
                self.best_resonance = res;
            }
        }

        Some(next)
    }

    fn random_step(&self, neighbors: &[NodeIndex]) -> NodeIndex {
        let mut rng = rand::thread_rng();
        *neighbors.choose(&mut rng).unwrap()
    }

    fn gradient_step(&self, cube: &MetatronCube, neighbors: &[NodeIndex]) -> NodeIndex {
        let mut rng = rand::thread_rng();

        // Epsilon-greedy: sometimes explore randomly
        if rng.gen::<f64>() < self.config.exploration_rate {
            return self.random_step(neighbors);
        }

        // Find neighbor with highest psi
        neighbors
            .iter()
            .max_by(|a, b| {
                let sig_a = cube.signature_at(a).map(|s| s.psi()).unwrap_or(0.0);
                let sig_b = cube.signature_at(b).map(|s| s.psi()).unwrap_or(0.0);
                sig_a.partial_cmp(&sig_b).unwrap()
            })
            .copied()
            .unwrap_or_else(|| self.random_step(neighbors))
    }

    fn stability_step(&self, cube: &MetatronCube, neighbors: &[NodeIndex]) -> NodeIndex {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < self.config.exploration_rate {
            return self.random_step(neighbors);
        }

        // Find neighbor with highest omega (stability)
        neighbors
            .iter()
            .max_by(|a, b| {
                let sig_a = cube.signature_at(a).map(|s| s.omega()).unwrap_or(0.0);
                let sig_b = cube.signature_at(b).map(|s| s.omega()).unwrap_or(0.0);
                sig_a.partial_cmp(&sig_b).unwrap()
            })
            .copied()
            .unwrap_or_else(|| self.random_step(neighbors))
    }

    fn cycle_step(&self, neighbors: &[NodeIndex]) -> NodeIndex {
        // Prefer nodes already in path (seeking cycles)
        for node in neighbors {
            if self.path.contains(node) {
                return *node;
            }
        }
        self.random_step(neighbors)
    }

    fn balanced_step(&self, cube: &MetatronCube, neighbors: &[NodeIndex]) -> NodeIndex {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < self.config.exploration_rate {
            return self.random_step(neighbors);
        }

        // Find neighbor with highest overall resonance
        neighbors
            .iter()
            .max_by(|a, b| {
                let sig_a = cube.signature_at(a).map(|s| s.resonance()).unwrap_or(0.0);
                let sig_b = cube.signature_at(b).map(|s| s.resonance()).unwrap_or(0.0);
                sig_a.partial_cmp(&sig_b).unwrap()
            })
            .copied()
            .unwrap_or_else(|| self.random_step(neighbors))
    }

    /// Run full traversal
    pub fn traverse(&mut self, cube: &MetatronCube) {
        while self.step(cube).is_some() {}
    }

    /// Check if target resonance was reached
    pub fn reached_target(&self) -> bool {
        self.best_resonance >= self.config.target_resonance
    }

    /// Get current resonance
    pub fn current_resonance(&self) -> f64 {
        resonance_5d(&self.signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let cube = MetatronCube::new();
        let config = AgentConfig::default();
        let agent = Agent::new(cube.identity_node(), config);

        assert_eq!(agent.steps, 0);
        assert!(!agent.path.is_empty());
    }

    #[test]
    fn test_agent_step() {
        let mut cube = MetatronCube::new();
        cube.randomize_signatures();

        let config = AgentConfig {
            max_steps: 10,
            ..Default::default()
        };
        let mut agent = Agent::new(cube.identity_node(), config);

        agent.step(&cube);
        assert_eq!(agent.steps, 1);
        assert_eq!(agent.path.len(), 2);
    }

    #[test]
    fn test_agent_traverse() {
        let mut cube = MetatronCube::new();
        cube.randomize_signatures();

        let config = AgentConfig {
            max_steps: 5,
            ..Default::default()
        };
        let mut agent = Agent::new(cube.identity_node(), config);

        agent.traverse(&cube);
        assert_eq!(agent.steps, 5);
    }
}
