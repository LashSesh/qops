//! High-level traversal API.

use crate::agent::{Agent, AgentConfig};
use crate::artefact::Artefact;
use crate::metatron_cube::MetatronCube;

/// Traversal engine for running mining operations
pub struct TraversalEngine {
    cube: MetatronCube,
    agents: Vec<Agent>,
    artefacts: Vec<Artefact>,
}

impl TraversalEngine {
    /// Create a new traversal engine
    pub fn new() -> Self {
        let mut cube = MetatronCube::new();
        cube.randomize_signatures();

        Self {
            cube,
            agents: Vec::new(),
            artefacts: Vec::new(),
        }
    }

    /// Run a single agent traversal
    pub fn run_single(&mut self, config: AgentConfig) -> Artefact {
        let mut agent = Agent::new(self.cube.identity_node(), config);
        agent.traverse(&self.cube);

        let artefact = Artefact::new(agent.position, agent.signature);
        self.artefacts.push(artefact.clone());

        artefact
    }

    /// Run multiple agents
    pub fn run_swarm(&mut self, num_agents: usize, config: AgentConfig) -> Vec<Artefact> {
        let mut results = Vec::new();

        for _ in 0..num_agents {
            let artefact = self.run_single(config.clone());
            results.push(artefact);
        }

        results
    }

    /// Get all mined artefacts
    pub fn artefacts(&self) -> &[Artefact] {
        &self.artefacts
    }

    /// Get best artefact by resonance
    pub fn best_artefact(&self) -> Option<&Artefact> {
        self.artefacts.iter().max_by(|a, b| {
            a.resonance.partial_cmp(&b.resonance).unwrap()
        })
    }

    /// Clear artefacts
    pub fn clear(&mut self) {
        self.artefacts.clear();
        self.agents.clear();
    }
}

impl Default for TraversalEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traversal_engine() {
        let mut engine = TraversalEngine::new();
        let config = AgentConfig {
            max_steps: 10,
            ..Default::default()
        };

        let artefact = engine.run_single(config);
        assert!(artefact.resonance > 0.0);
    }

    #[test]
    fn test_swarm() {
        let mut engine = TraversalEngine::new();
        let config = AgentConfig {
            max_steps: 5,
            ..Default::default()
        };

        let artefacts = engine.run_swarm(3, config);
        assert_eq!(artefacts.len(), 3);
    }
}
