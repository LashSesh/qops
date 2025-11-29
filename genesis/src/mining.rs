//! Genesis Mining Engine with TRITON integration.
//!
//! Comprehensive mining system for operator discovery on S7 topology.

use crate::agent::{Agent, AgentConfig, TraversalStrategy};
use crate::artefact::Artefact;
use crate::metatron_cube::MetatronCube;
use crate::family::OperatorFamily;
use qops_core::Signature5D;
use qops_triton::{TritonOptimizer, TritonConfig, OptimizationResult};
use serde::{Deserialize, Serialize};
use rayon::prelude::*;

/// Mining strategy selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MiningStrategy {
    /// TRITON spiral search optimization
    Triton,
    /// Balanced traversal (default)
    Balanced,
    /// Aggressive exploration (high randomness)
    Explorative,
    /// Conservative exploitation (greedy)
    Exploitative,
    /// Pure random walk
    Random,
    /// Evolutionary search
    Evolutionary,
    /// Hybrid: TRITON + Evolutionary
    HybridTritonEvolution,
    /// Multi-agent swarm
    Swarm,
}

impl Default for MiningStrategy {
    fn default() -> Self {
        Self::Balanced
    }
}

impl MiningStrategy {
    /// Convert to agent traversal strategy
    pub fn to_traversal_strategy(&self) -> TraversalStrategy {
        match self {
            Self::Explorative | Self::Random => TraversalStrategy::Random,
            Self::Exploitative => TraversalStrategy::GradientAscent,
            Self::Balanced | Self::Triton | Self::Swarm => TraversalStrategy::Balanced,
            Self::Evolutionary | Self::HybridTritonEvolution => TraversalStrategy::CycleRecognition,
        }
    }
}

/// Mining session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningConfig {
    /// Mining strategy to use
    pub strategy: MiningStrategy,
    /// Number of mining agents
    pub num_agents: usize,
    /// Steps per agent
    pub steps_per_agent: usize,
    /// Target resonance threshold
    pub target_resonance: f64,
    /// Enable parallel mining
    pub parallel: bool,
    /// TRITON configuration (if using TRITON strategy)
    pub triton_config: Option<TritonConfig>,
    /// Exploration rate
    pub exploration_rate: f64,
    /// Enable family extraction
    pub extract_families: bool,
    /// Family similarity threshold
    pub family_threshold: f64,
}

impl Default for MiningConfig {
    fn default() -> Self {
        Self {
            strategy: MiningStrategy::Balanced,
            num_agents: 10,
            steps_per_agent: 100,
            target_resonance: 0.85,
            parallel: true,
            triton_config: None,
            exploration_rate: 0.1,
            extract_families: true,
            family_threshold: 0.1,
        }
    }
}

impl MiningConfig {
    /// Create config for TRITON mining
    pub fn triton() -> Self {
        Self {
            strategy: MiningStrategy::Triton,
            triton_config: Some(TritonConfig::default()),
            ..Default::default()
        }
    }

    /// Create config for quick exploration
    pub fn quick() -> Self {
        Self {
            num_agents: 5,
            steps_per_agent: 50,
            parallel: false,
            ..Default::default()
        }
    }

    /// Create config for thorough mining
    pub fn thorough() -> Self {
        Self {
            num_agents: 50,
            steps_per_agent: 500,
            strategy: MiningStrategy::HybridTritonEvolution,
            triton_config: Some(TritonConfig::thorough()),
            ..Default::default()
        }
    }
}

/// Result of a mining session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningResult {
    /// Session ID
    pub session_id: String,
    /// All discovered artefacts
    pub artefacts: Vec<Artefact>,
    /// Best artefact
    pub best_artefact: Option<Artefact>,
    /// Best resonance achieved
    pub best_resonance: f64,
    /// Total steps taken
    pub total_steps: usize,
    /// Mandorla count (high-resonance artefacts)
    pub mandorla_count: usize,
    /// Extracted operator families
    pub families: Vec<OperatorFamily>,
    /// Strategy used
    pub strategy: MiningStrategy,
    /// TRITON result (if applicable)
    pub triton_result: Option<OptimizationResult>,
    /// Mining duration in milliseconds
    pub duration_ms: u64,
    /// Statistics
    pub stats: MiningStats,
}

/// Mining statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningStats {
    /// Average resonance
    pub avg_resonance: f64,
    /// Resonance standard deviation
    pub std_resonance: f64,
    /// Min resonance
    pub min_resonance: f64,
    /// Max resonance
    pub max_resonance: f64,
    /// Number of agents used
    pub num_agents: usize,
    /// Unique nodes visited
    pub unique_nodes: usize,
    /// Exploration efficiency
    pub efficiency: f64,
}

/// Genesis Mining Session
pub struct MiningSession {
    config: MiningConfig,
    cube: MetatronCube,
    artefacts: Vec<Artefact>,
    triton: Option<TritonOptimizer>,
    session_id: String,
    start_time: Option<std::time::Instant>,
}

impl MiningSession {
    /// Create new mining session
    pub fn new(config: MiningConfig) -> Self {
        let mut cube = MetatronCube::new();
        cube.randomize_signatures();

        let triton = if config.strategy == MiningStrategy::Triton ||
                        config.strategy == MiningStrategy::HybridTritonEvolution {
            Some(TritonOptimizer::new(
                config.triton_config.clone().unwrap_or_default()
            ))
        } else {
            None
        };

        Self {
            config,
            cube,
            artefacts: Vec::new(),
            triton,
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: None,
        }
    }

    /// Run the mining session
    pub fn mine(&mut self) -> MiningResult {
        self.start_time = Some(std::time::Instant::now());

        match self.config.strategy {
            MiningStrategy::Triton => self.triton_mining(),
            MiningStrategy::HybridTritonEvolution => self.hybrid_mining(),
            MiningStrategy::Swarm => self.swarm_mining(),
            _ => self.agent_mining(),
        }
    }

    /// TRITON-based mining
    fn triton_mining(&mut self) -> MiningResult {
        let triton = self.triton.as_mut().expect("TRITON not initialized");

        // Run TRITON optimization
        let triton_result = triton.optimize();

        // Convert best signature to artefact
        let best_sig = Signature5D::new(
            triton_result.best_signature[0],
            triton_result.best_signature[1],
            triton_result.best_signature[2],
            triton_result.best_signature[3],
            triton_result.best_signature[4],
        );

        let artefact = Artefact::from_signature(best_sig);
        self.artefacts.push(artefact.clone());

        // Also run some agent traversals for diversity
        self.run_agents(self.config.num_agents / 2);

        self.build_result(Some(triton_result))
    }

    /// Hybrid TRITON + Evolution mining
    fn hybrid_mining(&mut self) -> MiningResult {
        // Phase 1: TRITON exploration
        let triton = self.triton.as_mut().expect("TRITON not initialized");
        let triton_result = triton.optimize();

        // Convert TRITON result
        let best_sig = Signature5D::new(
            triton_result.best_signature[0],
            triton_result.best_signature[1],
            triton_result.best_signature[2],
            triton_result.best_signature[3],
            triton_result.best_signature[4],
        );
        self.artefacts.push(Artefact::from_signature(best_sig));

        // Phase 2: Agent-based exploitation around TRITON result
        self.run_agents(self.config.num_agents);

        // Phase 3: Evolutionary refinement
        self.evolutionary_refinement();

        self.build_result(Some(triton_result))
    }

    /// Swarm-based mining
    fn swarm_mining(&mut self) -> MiningResult {
        if self.config.parallel {
            self.parallel_swarm_mining()
        } else {
            self.sequential_swarm_mining()
        }
    }

    fn parallel_swarm_mining(&mut self) -> MiningResult {
        let configs: Vec<AgentConfig> = (0..self.config.num_agents)
            .map(|_| AgentConfig {
                max_steps: self.config.steps_per_agent,
                strategy: self.config.strategy.to_traversal_strategy(),
                exploration_rate: self.config.exploration_rate,
                target_resonance: self.config.target_resonance,
            })
            .collect();

        // Run agents in parallel
        let artefacts: Vec<Artefact> = configs
            .par_iter()
            .map(|config| {
                let mut cube = MetatronCube::new();
                cube.randomize_signatures();
                let mut agent = Agent::new(cube.identity_node(), config.clone());
                agent.traverse(&cube);
                Artefact::new(agent.position, agent.signature)
            })
            .collect();

        self.artefacts.extend(artefacts);
        self.build_result(None)
    }

    fn sequential_swarm_mining(&mut self) -> MiningResult {
        self.run_agents(self.config.num_agents);
        self.build_result(None)
    }

    /// Standard agent-based mining
    fn agent_mining(&mut self) -> MiningResult {
        self.run_agents(self.config.num_agents);
        self.build_result(None)
    }

    /// Run a set of agents
    fn run_agents(&mut self, count: usize) {
        let agent_config = AgentConfig {
            max_steps: self.config.steps_per_agent,
            strategy: self.config.strategy.to_traversal_strategy(),
            exploration_rate: self.config.exploration_rate,
            target_resonance: self.config.target_resonance,
        };

        for _ in 0..count {
            let mut agent = Agent::new(self.cube.identity_node(), agent_config.clone());
            agent.traverse(&self.cube);

            let artefact = Artefact::new(agent.position, agent.signature);
            self.artefacts.push(artefact);
        }
    }

    /// Evolutionary refinement of best artefacts
    fn evolutionary_refinement(&mut self) {
        if self.artefacts.is_empty() {
            return;
        }

        // Select top performers
        self.artefacts.sort_by(|a, b| {
            b.resonance.partial_cmp(&a.resonance).unwrap()
        });

        let elite: Vec<_> = self.artefacts.iter().take(5).cloned().collect();

        // Generate offspring through crossover
        let mut rng = rand::thread_rng();
        use rand::Rng;

        for _ in 0..10 {
            let p1 = &elite[rng.gen_range(0..elite.len())];
            let p2 = &elite[rng.gen_range(0..elite.len())];

            let alpha: f64 = rng.gen();
            let child_sig = Signature5D::new(
                alpha * p1.signature.psi + (1.0 - alpha) * p2.signature.psi,
                alpha * p1.signature.rho + (1.0 - alpha) * p2.signature.rho,
                alpha * p1.signature.omega + (1.0 - alpha) * p2.signature.omega,
                alpha * p1.signature.chi + (1.0 - alpha) * p2.signature.chi,
                alpha * p1.signature.eta + (1.0 - alpha) * p2.signature.eta,
            );

            let artefact = Artefact::from_signature(child_sig);
            self.artefacts.push(artefact);
        }
    }

    /// Build mining result
    fn build_result(&self, triton_result: Option<OptimizationResult>) -> MiningResult {
        let duration_ms = self.start_time
            .map(|t| t.elapsed().as_millis() as u64)
            .unwrap_or(0);

        let best_artefact = self.artefacts.iter()
            .max_by(|a, b| a.resonance.partial_cmp(&b.resonance).unwrap())
            .cloned();

        let best_resonance = best_artefact.as_ref()
            .map(|a| a.resonance)
            .unwrap_or(0.0);

        let mandorla_count = self.artefacts.iter()
            .filter(|a| a.resonance >= 0.85 && a.is_mandorla())
            .count();

        let families = if self.config.extract_families {
            self.extract_families()
        } else {
            Vec::new()
        };

        let stats = self.compute_stats();

        MiningResult {
            session_id: self.session_id.clone(),
            artefacts: self.artefacts.clone(),
            best_artefact,
            best_resonance,
            total_steps: self.config.num_agents * self.config.steps_per_agent,
            mandorla_count,
            families,
            strategy: self.config.strategy,
            triton_result,
            duration_ms,
            stats,
        }
    }

    /// Extract operator families from artefacts
    fn extract_families(&self) -> Vec<OperatorFamily> {
        if self.artefacts.len() < 2 {
            return Vec::new();
        }

        let mut families: Vec<OperatorFamily> = Vec::new();
        let threshold = self.config.family_threshold;

        for artefact in &self.artefacts {
            let mut found_family = false;

            for family in &mut families {
                if family.is_similar(&artefact.signature, threshold) {
                    family.add_member(artefact.clone());
                    found_family = true;
                    break;
                }
            }

            if !found_family {
                let mut new_family = OperatorFamily::new();
                new_family.add_member(artefact.clone());
                families.push(new_family);
            }
        }

        // Sort families by average resonance
        families.sort_by(|a, b| {
            b.avg_resonance().partial_cmp(&a.avg_resonance()).unwrap()
        });

        families
    }

    /// Compute mining statistics
    fn compute_stats(&self) -> MiningStats {
        if self.artefacts.is_empty() {
            return MiningStats {
                avg_resonance: 0.0,
                std_resonance: 0.0,
                min_resonance: 0.0,
                max_resonance: 0.0,
                num_agents: 0,
                unique_nodes: 0,
                efficiency: 0.0,
            };
        }

        let resonances: Vec<f64> = self.artefacts.iter()
            .map(|a| a.resonance)
            .collect();

        let sum: f64 = resonances.iter().sum();
        let avg = sum / resonances.len() as f64;

        let var: f64 = resonances.iter()
            .map(|r| (r - avg).powi(2))
            .sum::<f64>() / resonances.len() as f64;
        let std = var.sqrt();

        let min = resonances.iter().copied().fold(f64::MAX, f64::min);
        let max = resonances.iter().copied().fold(f64::MIN, f64::max);

        // Count unique nodes
        let unique_nodes = self.artefacts.iter()
            .map(|a| a.node)
            .collect::<std::collections::HashSet<_>>()
            .len();

        let efficiency = unique_nodes as f64 / self.artefacts.len() as f64;

        MiningStats {
            avg_resonance: avg,
            std_resonance: std,
            min_resonance: min,
            max_resonance: max,
            num_agents: self.config.num_agents,
            unique_nodes,
            efficiency,
        }
    }

    /// Get current artefacts
    pub fn artefacts(&self) -> &[Artefact] {
        &self.artefacts
    }

    /// Get best artefact
    pub fn best(&self) -> Option<&Artefact> {
        self.artefacts.iter()
            .max_by(|a, b| a.resonance.partial_cmp(&b.resonance).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mining_session() {
        let config = MiningConfig::quick();
        let mut session = MiningSession::new(config);

        let result = session.mine();
        assert!(!result.artefacts.is_empty());
        assert!(result.best_resonance > 0.0);
    }

    #[test]
    fn test_triton_mining() {
        let config = MiningConfig {
            strategy: MiningStrategy::Triton,
            num_agents: 3,
            steps_per_agent: 10,
            triton_config: Some(TritonConfig {
                max_iterations: 20,
                ..TritonConfig::quick()
            }),
            ..Default::default()
        };
        let mut session = MiningSession::new(config);

        let result = session.mine();
        assert!(result.triton_result.is_some());
    }

    #[test]
    fn test_family_extraction() {
        let config = MiningConfig {
            num_agents: 20,
            steps_per_agent: 50,
            extract_families: true,
            family_threshold: 0.2,
            ..Default::default()
        };
        let mut session = MiningSession::new(config);

        let result = session.mine();
        // Families may or may not be found depending on results
        assert!(result.artefacts.len() >= 20);
    }
}
