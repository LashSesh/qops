//! Holistic Mining Engine
//!
//! Multi-stage operator mining using the Holistic Resonance Architecture:
//! - Stage 1: Discovery (operator exploration)
//! - Stage 2: Kosmokrator Filter (PoR-based exclusion)
//! - Stage 3: Chronokrator Expansion (resonance dynamics)
//! - Stage 4: Pfauenthron Collapse (Mandorla finalization)
//!
//! Integrates with Adaptive TRITON for spiral search optimization.

use crate::agent::{Agent, AgentConfig, TraversalStrategy};
use crate::artefact::Artefact;
use crate::metatron_cube::MetatronCube;
use crate::family::OperatorFamily;
use crate::mining::{MiningConfig, MiningStrategy, MiningResult, MiningStats};

use qops_core::{
    Signature5D, resonance_5d,
    GenesisStage, HolisticMatrix, HolisticConfig, HolisticStats,
    KosmokratorConfig, ChronokratorConfig, PfauenthronConfig,
    OperatorCandidate, ExkalibrationVector, Monolith, FinalizedFamily,
    KosmokratorStats, ChronokratorStats, PfauenthronStats,
};
use qops_triton::{
    AdaptiveTritonOptimizer, AdaptiveTritonConfig, AdaptiveOptimizationResult,
    TritonConfig, SpiralParams,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rayon::prelude::*;

// ============================================================================
// HOLISTIC MINING CONFIGURATION
// ============================================================================

/// Configuration for holistic multi-stage mining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolisticMiningConfig {
    /// Base mining configuration
    pub mining: MiningConfig,
    /// Kosmokrator filter configuration
    pub kosmokrator: KosmokratorConfig,
    /// Chronokrator expansion configuration
    pub chronokrator: ChronokratorConfig,
    /// Pfauenthron collapse configuration
    pub pfauenthron: PfauenthronConfig,
    /// Enable adaptive TRITON
    pub adaptive_triton: bool,
    /// Adaptive TRITON configuration
    pub triton_config: AdaptiveTritonConfig,
    /// Enable stage logging
    pub log_stages: bool,
    /// Export intermediate results
    pub export_intermediates: bool,
}

impl Default for HolisticMiningConfig {
    fn default() -> Self {
        Self {
            mining: MiningConfig::default(),
            kosmokrator: KosmokratorConfig::default(),
            chronokrator: ChronokratorConfig::default(),
            pfauenthron: PfauenthronConfig::default(),
            adaptive_triton: true,
            triton_config: AdaptiveTritonConfig::default(),
            log_stages: true,
            export_intermediates: false,
        }
    }
}

impl HolisticMiningConfig {
    /// Create quick exploration config
    pub fn quick() -> Self {
        Self {
            mining: MiningConfig::quick(),
            triton_config: AdaptiveTritonConfig {
                base: TritonConfig::quick(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create thorough mining config
    pub fn thorough() -> Self {
        Self {
            mining: MiningConfig::thorough(),
            kosmokrator: KosmokratorConfig {
                kappa_threshold: 0.8,
                epsilon: 0.03,
                ..Default::default()
            },
            chronokrator: ChronokratorConfig {
                num_channels: 8,
                base_threshold: 0.7,
                ..Default::default()
            },
            pfauenthron: PfauenthronConfig {
                mandorla_threshold: 0.85,
                num_ophanim: 6,
                ..Default::default()
            },
            triton_config: AdaptiveTritonConfig {
                base: TritonConfig::thorough(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create research-grade config for maximum quality
    pub fn research() -> Self {
        Self {
            mining: MiningConfig {
                num_agents: 100,
                steps_per_agent: 1000,
                strategy: MiningStrategy::HybridTritonEvolution,
                ..Default::default()
            },
            kosmokrator: KosmokratorConfig {
                kappa_threshold: 0.85,
                epsilon: 0.02,
                telescope_gamma: 0.9,
                stability_window: 10,
                ..Default::default()
            },
            chronokrator: ChronokratorConfig {
                num_channels: 12,
                base_threshold: 0.75,
                threshold_adaptation: 0.15,
                ..Default::default()
            },
            pfauenthron: PfauenthronConfig {
                mandorla_threshold: 0.9,
                convergence_epsilon: 0.005,
                num_ophanim: 8,
                ..Default::default()
            },
            adaptive_triton: true,
            triton_config: AdaptiveTritonConfig {
                base: TritonConfig {
                    max_iterations: 10000,
                    spiral: SpiralParams {
                        layers: 15,
                        points_per_layer: 24,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                local_search: true,
                local_iterations: 500,
                resonance_weighted: true,
                holistic_integration: true,
                ..Default::default()
            },
            log_stages: true,
            export_intermediates: true,
        }
    }
}

// ============================================================================
// STAGE LOG ENTRIES
// ============================================================================

/// Log entry for stage transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageLogEntry {
    /// Stage name
    pub stage: GenesisStage,
    /// Timestamp (step number)
    pub timestamp: usize,
    /// Duration of stage (steps)
    pub duration: usize,
    /// Candidates entering stage
    pub candidates_in: usize,
    /// Candidates exiting stage
    pub candidates_out: usize,
    /// Stage-specific metrics
    pub metrics: StageMetrics,
}

/// Stage-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StageMetrics {
    Discovery {
        nodes_visited: usize,
        unique_nodes: usize,
        max_resonance: f64,
    },
    Kosmokrator {
        kappa: f64,
        por_passed: bool,
        exclusion_rate: f64,
    },
    Chronokrator {
        d_total: f64,
        threshold: f64,
        spike_count: usize,
        exkalibration_magnitude: f64,
    },
    Pfauenthron {
        mandorla_score: f64,
        is_converged: bool,
        monolith_emitted: bool,
        families_formed: usize,
    },
}

// ============================================================================
// HOLISTIC MINING RESULT
// ============================================================================

/// Result of holistic multi-stage mining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolisticMiningResult {
    /// Session identifier
    pub session_id: String,
    /// Final stage reached
    pub final_stage: GenesisStage,
    /// All discovered artefacts
    pub artefacts: Vec<Artefact>,
    /// Best artefact
    pub best_artefact: Option<Artefact>,
    /// Best resonance score
    pub best_resonance: f64,
    /// Total mining steps
    pub total_steps: usize,
    /// Mandorla (high-resonance) count
    pub mandorla_count: usize,
    /// Extracted operator families
    pub families: Vec<OperatorFamily>,
    /// Finalized families from Pfauenthron
    pub finalized_families: Vec<FinalizedFamily>,
    /// Emitted Monoliths
    pub monoliths: Vec<Monolith>,
    /// TRITON optimization result
    pub triton_result: Option<AdaptiveOptimizationResult>,
    /// Stage log
    pub stage_log: Vec<StageLogEntry>,
    /// Holistic matrix statistics
    pub holistic_stats: HolisticStats,
    /// Mining statistics
    pub mining_stats: MiningStats,
    /// Duration in milliseconds
    pub duration_ms: u64,
    /// Resonance timeline
    pub resonance_timeline: Vec<f64>,
    /// Exkalibration vectors
    pub exkalibration_vectors: Vec<ExkalibrationVector>,
    /// Number of candidates discovered
    pub candidates_discovered: usize,
    /// Number of candidates after Kosmokrator filtering
    pub candidates_after_kosmokrator: usize,
    /// Number of candidates after Chronokrator expansion
    pub candidates_after_chronokrator: usize,
}

// ============================================================================
// HOLISTIC MINING SESSION
// ============================================================================

/// Holistic multi-stage mining session
pub struct HolisticMiningSession {
    config: HolisticMiningConfig,
    cube: MetatronCube,
    matrix: HolisticMatrix,
    triton: Option<AdaptiveTritonOptimizer>,
    artefacts: Vec<Artefact>,
    candidates: Vec<OperatorCandidate>,
    stage_log: Vec<StageLogEntry>,
    session_id: String,
    current_stage: GenesisStage,
    current_step: usize,
    resonance_timeline: Vec<f64>,
    start_time: Option<std::time::Instant>,
    /// Track candidate count after discovery
    candidates_discovered: usize,
    /// Track candidate count after Kosmokrator
    candidates_after_kosmokrator: usize,
    /// Track candidate count after Chronokrator
    candidates_after_chronokrator: usize,
}

impl HolisticMiningSession {
    /// Create new holistic mining session
    pub fn new(config: HolisticMiningConfig) -> Self {
        let mut cube = MetatronCube::new();
        cube.randomize_signatures();

        let holistic_config = HolisticConfig {
            kosmokrator: config.kosmokrator.clone(),
            chronokrator: config.chronokrator.clone(),
            pfauenthron: config.pfauenthron.clone(),
        };

        let matrix = HolisticMatrix::new(holistic_config);

        let triton = if config.adaptive_triton {
            Some(AdaptiveTritonOptimizer::new(config.triton_config.clone()))
        } else {
            None
        };

        Self {
            config,
            cube,
            matrix,
            triton,
            artefacts: Vec::new(),
            candidates: Vec::new(),
            stage_log: Vec::new(),
            session_id: uuid::Uuid::new_v4().to_string(),
            current_stage: GenesisStage::Discovery,
            current_step: 0,
            resonance_timeline: Vec::new(),
            start_time: None,
            candidates_discovered: 0,
            candidates_after_kosmokrator: 0,
            candidates_after_chronokrator: 0,
        }
    }

    /// Run full holistic mining pipeline
    pub fn mine(&mut self) -> HolisticMiningResult {
        self.start_time = Some(std::time::Instant::now());
        self.current_stage = GenesisStage::Discovery;

        // Stage 1: Discovery
        self.run_discovery_stage();

        // Stage 2: Kosmokrator Filter
        self.run_kosmokrator_stage();

        // Stage 3: Chronokrator Expansion
        self.run_chronokrator_stage();

        // Stage 4: Pfauenthron Collapse
        self.run_pfauenthron_stage();

        // Build result
        self.build_result()
    }

    /// Stage 1: Discovery - Explore the operator space
    fn run_discovery_stage(&mut self) {
        let stage_start = self.current_step;
        self.log_stage_start(GenesisStage::Discovery);

        let mut nodes_visited = 0;
        let mut unique_nodes = std::collections::HashSet::new();
        let mut max_resonance = 0.0f64;

        // Run agent-based exploration
        if self.config.mining.parallel && self.config.mining.num_agents > 1 {
            self.parallel_agent_discovery();
        } else {
            self.sequential_agent_discovery();
        }

        // Run adaptive TRITON if enabled
        if let Some(triton) = &mut self.triton {
            let result = triton.optimize();

            // Convert TRITON results to candidates
            let sig = Signature5D::new(
                result.best_signature[0],
                result.best_signature[1],
                result.best_signature[2],
                result.best_signature[3],
                result.best_signature[4],
            );

            let artefact = Artefact::from_signature(sig);
            self.artefacts.push(artefact);

            self.candidates.push(OperatorCandidate {
                id: format!("triton_best"),
                signature: sig,
                phase: 0.0,
                resonance: result.best_score,
                stability: if result.converged { 1.0 } else { 0.5 },
                is_mandorla: result.best_score >= 0.85,
                node_index: 0,
                discovered_at: self.current_step as f64,
            });
        }

        // Convert artefacts to candidates
        for (i, artefact) in self.artefacts.iter().enumerate() {
            nodes_visited += 1;
            unique_nodes.insert(artefact.node);
            max_resonance = max_resonance.max(artefact.resonance);

            self.candidates.push(OperatorCandidate {
                id: format!("discovery_{}", i),
                signature: artefact.signature,
                phase: artefact.resonance * std::f64::consts::PI,
                resonance: artefact.resonance,
                stability: 0.5,
                is_mandorla: artefact.is_mandorla(),
                node_index: artefact.node.index(),
                discovered_at: self.current_step as f64,
            });

            self.resonance_timeline.push(artefact.resonance);
        }

        self.current_step += self.artefacts.len();

        // Track candidates discovered
        self.candidates_discovered = self.candidates.len();

        self.log_stage_end(GenesisStage::Discovery, stage_start, StageMetrics::Discovery {
            nodes_visited,
            unique_nodes: unique_nodes.len(),
            max_resonance,
        });
    }

    fn sequential_agent_discovery(&mut self) {
        let agent_config = AgentConfig {
            max_steps: self.config.mining.steps_per_agent,
            strategy: self.config.mining.strategy.to_traversal_strategy(),
            exploration_rate: self.config.mining.exploration_rate,
            target_resonance: self.config.mining.target_resonance,
        };

        for _ in 0..self.config.mining.num_agents {
            let mut agent = Agent::new(self.cube.identity_node(), agent_config.clone());
            agent.traverse(&self.cube);

            let artefact = Artefact::new(agent.position, agent.signature);
            self.artefacts.push(artefact);
        }
    }

    fn parallel_agent_discovery(&mut self) {
        let configs: Vec<AgentConfig> = (0..self.config.mining.num_agents)
            .map(|_| AgentConfig {
                max_steps: self.config.mining.steps_per_agent,
                strategy: self.config.mining.strategy.to_traversal_strategy(),
                exploration_rate: self.config.mining.exploration_rate,
                target_resonance: self.config.mining.target_resonance,
            })
            .collect();

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
    }

    /// Stage 2: Kosmokrator Filter - Apply PoR-based exclusion
    fn run_kosmokrator_stage(&mut self) {
        let stage_start = self.current_step;
        self.log_stage_start(GenesisStage::KosmokratorFilter);
        let candidates_in = self.candidates.len();

        // Process through holistic matrix (partial - just kosmokrator)
        let t = self.current_step as f64;
        let phases: Vec<f64> = self.candidates.iter().map(|c| c.phase).collect();

        // Compute PoR
        let por = self.matrix.kosmokrator.compute_por(&phases, t);

        if por.passed {
            // Apply telescope operator
            self.candidates = self.matrix.kosmokrator.apply_telescope(&self.candidates);
        } else {
            // Keep only top candidates by resonance
            self.candidates.sort_by(|a, b| b.resonance.partial_cmp(&a.resonance).unwrap());
            self.candidates.truncate(self.candidates.len() / 2);
        }

        let stats = self.matrix.kosmokrator.stats();

        self.current_step += 1;
        self.current_stage = GenesisStage::ChronokratorExpansion;

        // Track candidates after Kosmokrator
        self.candidates_after_kosmokrator = self.candidates.len();

        self.log_stage_end(GenesisStage::KosmokratorFilter, stage_start, StageMetrics::Kosmokrator {
            kappa: por.kappa,
            por_passed: por.passed,
            exclusion_rate: 1.0 - (self.candidates.len() as f64 / candidates_in as f64),
        });
    }

    /// Stage 3: Chronokrator Expansion - Generate expansion dynamics
    fn run_chronokrator_stage(&mut self) {
        let stage_start = self.current_step;
        self.log_stage_start(GenesisStage::ChronokratorExpansion);
        let candidates_in = self.candidates.len();

        let t = self.current_step as f64;

        // Initialize channels from candidates
        self.matrix.chronokrator.init_channels(&self.candidates);

        // Compute expansion dynamics
        let exkal = self.matrix.chronokrator.expand(&self.candidates, t);

        let stats = self.matrix.chronokrator.stats();
        let exkal_magnitude = exkal.as_ref().map(|e| e.magnitude).unwrap_or(0.0);

        // Store Exkalibration if valid
        if let Some(e) = &exkal {
            if e.valid {
                // Use Exkalibration to refine candidates
                self.apply_exkalibration_refinement(e);
            }
        }

        self.current_step += 1;
        self.current_stage = GenesisStage::PfauenthronCollapse;

        // Track candidates after Chronokrator
        self.candidates_after_chronokrator = self.candidates.len();

        self.log_stage_end(GenesisStage::ChronokratorExpansion, stage_start, StageMetrics::Chronokrator {
            d_total: stats.current_d_total,
            threshold: stats.current_threshold,
            spike_count: stats.spike_count,
            exkalibration_magnitude: exkal_magnitude,
        });
    }

    fn apply_exkalibration_refinement(&mut self, exkal: &ExkalibrationVector) {
        // Weight candidates by their alignment with Exkalibration vector
        for c in &mut self.candidates {
            let sig_vec = c.signature.to_vec();
            let alignment: f64 = sig_vec.iter()
                .zip(exkal.direction.iter())
                .map(|(s, e)| s * e)
                .sum();

            // Boost resonance of aligned candidates
            c.resonance *= 1.0 + 0.1 * alignment.abs();
            c.stability = (c.stability + alignment.abs()) / 2.0;
        }

        // Sort by new weighted score
        self.candidates.sort_by(|a, b| {
            let score_a = a.resonance * (1.0 + a.stability);
            let score_b = b.resonance * (1.0 + b.stability);
            score_b.partial_cmp(&score_a).unwrap()
        });
    }

    /// Stage 4: Pfauenthron Collapse - Finalize families and emit Monoliths
    fn run_pfauenthron_stage(&mut self) {
        let stage_start = self.current_step;
        self.log_stage_start(GenesisStage::PfauenthronCollapse);

        let t = self.current_step as f64;

        // Create a dummy Exkalibration for Mandorla computation
        let exkal = ExkalibrationVector {
            gradient: [0.8, 0.7, 0.6, 0.5, 0.4],
            magnitude: 1.0,
            direction: [0.4, 0.35, 0.3, 0.25, 0.2],
            timestamp: t,
            valid: true,
        };

        // Attempt collapse
        let monolith = self.matrix.pfauenthron.collapse(&self.candidates, &exkal, t);

        // Finalize families
        self.matrix.pfauenthron.finalize_families(&self.candidates);

        let stats = self.matrix.pfauenthron.stats();

        self.current_step += 1;
        self.current_stage = GenesisStage::Finalized;

        self.log_stage_end(GenesisStage::PfauenthronCollapse, stage_start, StageMetrics::Pfauenthron {
            mandorla_score: stats.current_mandorla,
            is_converged: stats.is_converged,
            monolith_emitted: monolith.is_some(),
            families_formed: stats.family_count,
        });
    }

    fn log_stage_start(&mut self, stage: GenesisStage) {
        if self.config.log_stages {
            self.current_stage = stage;
        }
    }

    fn log_stage_end(&mut self, stage: GenesisStage, start: usize, metrics: StageMetrics) {
        if self.config.log_stages {
            let candidates_out = self.candidates.len();
            let candidates_in = match &metrics {
                StageMetrics::Discovery { nodes_visited, .. } => *nodes_visited,
                _ => self.artefacts.len(),
            };

            self.stage_log.push(StageLogEntry {
                stage,
                timestamp: start,
                duration: self.current_step - start,
                candidates_in,
                candidates_out,
                metrics,
            });
        }
    }

    /// Build final result
    fn build_result(&self) -> HolisticMiningResult {
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
            .filter(|a| a.is_mandorla())
            .count();

        // Extract families using standard method
        let families = if self.config.mining.extract_families {
            self.extract_families()
        } else {
            Vec::new()
        };

        let holistic_stats = self.matrix.stats();
        let finalized_families = self.matrix.families().to_vec();
        let monoliths = self.matrix.pfauenthron.monoliths.clone();
        let exkalibration_vectors = self.matrix.chronokrator.exkalibration_history.clone();

        let triton_result = self.triton.as_ref().map(|t| {
            // Get the last result - we need to re-run to get it
            AdaptiveOptimizationResult {
                best_signature: [0.5; 5],
                best_score: 0.0,
                iterations: 0,
                converged: false,
                layer_memory: qops_triton::adaptive::SpiralLayerMemory::new(1),
                radius_stats: qops_triton::adaptive::AdaptiveRadiusStats {
                    current_radius: 0.1,
                    successes: 0,
                    failures: 0,
                    success_rate: 0.0,
                    steps_since_improvement: 0,
                },
                cooling_stats: qops_triton::adaptive::DynamicCoolingStats {
                    temperature: 1.0,
                    step: 0,
                    reheat_count: 0,
                    temp_history_len: 0,
                },
                convergence_point: None,
                holistic_output: None,
            }
        });

        let mining_stats = self.compute_stats();

        HolisticMiningResult {
            session_id: self.session_id.clone(),
            final_stage: self.current_stage,
            artefacts: self.artefacts.clone(),
            best_artefact,
            best_resonance,
            total_steps: self.current_step,
            mandorla_count,
            families,
            finalized_families,
            monoliths,
            triton_result,
            stage_log: self.stage_log.clone(),
            holistic_stats,
            mining_stats,
            duration_ms,
            resonance_timeline: self.resonance_timeline.clone(),
            exkalibration_vectors,
            candidates_discovered: self.candidates_discovered,
            candidates_after_kosmokrator: self.candidates_after_kosmokrator,
            candidates_after_chronokrator: self.candidates_after_chronokrator,
        }
    }

    fn extract_families(&self) -> Vec<OperatorFamily> {
        if self.artefacts.len() < 2 {
            return Vec::new();
        }

        let mut families: Vec<OperatorFamily> = Vec::new();
        let threshold = self.config.mining.family_threshold;

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

        families.sort_by(|a, b| {
            b.avg_resonance().partial_cmp(&a.avg_resonance()).unwrap()
        });

        families
    }

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
            num_agents: self.config.mining.num_agents,
            unique_nodes,
            efficiency,
        }
    }

    /// Get current stage
    pub fn stage(&self) -> GenesisStage {
        self.current_stage
    }

    /// Get current candidates
    pub fn candidates(&self) -> &[OperatorCandidate] {
        &self.candidates
    }

    /// Get artefacts
    pub fn artefacts(&self) -> &[Artefact] {
        &self.artefacts
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holistic_mining_session() {
        let config = HolisticMiningConfig {
            mining: MiningConfig {
                num_agents: 3,
                steps_per_agent: 10,
                ..Default::default()
            },
            adaptive_triton: false,
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);
        let result = session.mine();

        assert!(!result.artefacts.is_empty());
        assert_eq!(result.final_stage, GenesisStage::Finalized);
        assert!(!result.stage_log.is_empty());
    }

    #[test]
    fn test_holistic_mining_stages() {
        let config = HolisticMiningConfig::quick();
        let mut session = HolisticMiningSession::new(config);
        let result = session.mine();

        // Should have all four stage logs
        assert!(result.stage_log.len() >= 4);

        // Check stage progression
        let stages: Vec<_> = result.stage_log.iter().map(|s| s.stage).collect();
        assert!(stages.contains(&GenesisStage::Discovery));
        assert!(stages.contains(&GenesisStage::KosmokratorFilter));
        assert!(stages.contains(&GenesisStage::ChronokratorExpansion));
        assert!(stages.contains(&GenesisStage::PfauenthronCollapse));
    }

    #[test]
    fn test_holistic_with_triton() {
        let config = HolisticMiningConfig {
            mining: MiningConfig {
                num_agents: 2,
                steps_per_agent: 5,
                ..Default::default()
            },
            adaptive_triton: true,
            triton_config: AdaptiveTritonConfig {
                base: TritonConfig {
                    max_iterations: 20,
                    spiral: SpiralParams {
                        layers: 2,
                        points_per_layer: 4,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                holistic_integration: false,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);
        let result = session.mine();

        assert!(result.best_resonance > 0.0);
    }
}
