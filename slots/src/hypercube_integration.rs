//! Hypercube Integration
//!
//! Integrates slots as artifact outputs from HDAG nodes.

use crate::miner::{MinedSequence, MinerConfig};
use crate::lattice::LatticeConfig;
use crate::session::{SlotsSession, SlotsSessionConfig};
use crate::error::Result;
use qops_hypercube::coordinates::Coord5D;
use qops_hypercube::artifact::{HypercubeArtifact, ArtifactType};
use qops_hypercube::hdag::{HDAG, HDAGNode, HDAGNodeType, HDAGEdge};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A slot artifact (slots as HDAG output)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotArtifact {
    /// Artifact ID
    pub id: String,
    /// Name
    pub name: String,
    /// Source mined sequence
    pub sequence: MinedSequence,
    /// 5D coordinate
    pub coordinate: Coord5D,
    /// Resonance
    pub resonance: f64,
    /// Source HDAG node
    pub source_node: Option<String>,
    /// Metadata
    pub metadata: SlotArtifactMetadata,
}

/// Slot artifact metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlotArtifactMetadata {
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Mining steps
    pub mining_steps: usize,
    /// Mining strategy used
    pub strategy: Option<String>,
    /// Symbol distribution
    pub symbol_counts: std::collections::HashMap<String, usize>,
}

impl SlotArtifact {
    /// Create from a mined sequence
    pub fn from_sequence(sequence: MinedSequence, source_node: Option<&str>) -> Self {
        let coordinate = sequence.to_hypercube_coord();
        let resonance = sequence.resonance;

        // Count symbols
        let mut symbol_counts = std::collections::HashMap::new();
        for symbol in &sequence.symbols {
            let key = format!("{}", symbol);
            *symbol_counts.entry(key).or_insert(0) += 1;
        }

        Self {
            id: Uuid::new_v4().to_string(),
            name: format!("slot_artifact_{}", &sequence.id[..8]),
            sequence,
            coordinate,
            resonance,
            source_node: source_node.map(|s| s.to_string()),
            metadata: SlotArtifactMetadata {
                created_at: Some(chrono::Utc::now().to_rfc3339()),
                mining_steps: 0,
                strategy: None,
                symbol_counts,
            },
        }
    }

    /// Convert to hypercube artifact
    pub fn to_hypercube_artifact(&self) -> HypercubeArtifact {
        let mut artifact = HypercubeArtifact::new(
            &self.name,
            ArtifactType::CompiledFamily,
            self.coordinate,
        );

        artifact.metadata.source_node = self.source_node.clone();

        artifact
    }
}

/// Adapter for integrating slots with hypercube
pub struct SlotsHypercubeAdapter {
    /// Configuration
    config: AdapterConfig,
    /// Slot artifacts generated
    artifacts: Vec<SlotArtifact>,
}

/// Adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    /// Mining configuration
    pub miner_config: MinerConfig,
    /// Lattice configuration
    pub lattice_config: LatticeConfig,
    /// Generate HDAG
    pub generate_hdag: bool,
    /// Connect to existing HDAG
    pub connect_to_hdag: bool,
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            miner_config: MinerConfig::default(),
            lattice_config: LatticeConfig::default(),
            generate_hdag: true,
            connect_to_hdag: true,
        }
    }
}

impl SlotsHypercubeAdapter {
    /// Create a new adapter
    pub fn new(config: AdapterConfig) -> Self {
        Self {
            config,
            artifacts: Vec::new(),
        }
    }

    /// Create with default config
    pub fn default_adapter() -> Self {
        Self::new(AdapterConfig::default())
    }

    /// Generate slot artifacts from a coordinate
    pub fn generate_from_coord(&mut self, coord: Coord5D) -> Result<Vec<SlotArtifact>> {
        // Create session biased towards the coordinate
        let session_config = SlotsSessionConfig {
            miner_config: MinerConfig {
                target_resonance: coord.resonance(),
                ..self.config.miner_config.clone()
            },
            ..Default::default()
        };

        let mut session = SlotsSession::new(session_config);

        // Run mining
        let mining_result = session.run_mining()?;

        // Convert to slot artifacts
        let artifacts: Vec<SlotArtifact> = mining_result.top_sequences
            .into_iter()
            .map(|seq| SlotArtifact::from_sequence(seq, None))
            .collect();

        self.artifacts.extend(artifacts.clone());

        Ok(artifacts)
    }

    /// Generate slot artifacts for HDAG execution
    pub fn generate_for_hdag(&mut self, hdag: &HDAG) -> Result<Vec<SlotArtifact>> {
        let mut artifacts = Vec::new();

        // Generate artifacts for each output node
        for node in hdag.nodes() {
            if matches!(node.node_type, HDAGNodeType::Output | HDAGNodeType::Compilation) {
                if let Some(output) = &node.output {
                    let seq_artifacts = self.generate_from_coord(*output)?;

                    for mut artifact in seq_artifacts {
                        artifact.source_node = Some(node.id.clone());
                        artifacts.push(artifact);
                    }
                }
            }
        }

        Ok(artifacts)
    }

    /// Create an HDAG that uses slots for artifact generation
    pub fn create_slots_hdag(&self, seed: Coord5D) -> HDAG {
        let mut hdag = HDAG::new("Slots-Enhanced Pipeline");

        // Input
        let input_id = hdag.add_node(HDAGNode::input("Slot Input", seed));

        // Slot mining nodes
        let mine_psi = hdag.add_node(HDAGNode::new("Mine Psi", HDAGNodeType::Operator));
        let mine_rho = hdag.add_node(HDAGNode::new("Mine Rho", HDAGNodeType::Operator));
        let mine_omega = hdag.add_node(HDAGNode::new("Mine Omega", HDAGNodeType::Operator));

        // Merge
        let merge_id = hdag.add_node(HDAGNode::new("Merge Slots", HDAGNodeType::Merge));

        // Compilation
        let compile_id = hdag.add_node(HDAGNode::compilation("Slot Compile"));

        // Output
        let output_id = hdag.add_node(HDAGNode::output("Slot Artifact Output"));

        // Edges
        hdag.add_edge(&input_id, &mine_psi, HDAGEdge::data().with_label("psi")).ok();
        hdag.add_edge(&input_id, &mine_rho, HDAGEdge::data().with_label("rho")).ok();
        hdag.add_edge(&input_id, &mine_omega, HDAGEdge::data().with_label("omega")).ok();

        hdag.add_edge(&mine_psi, &merge_id, HDAGEdge::data()).ok();
        hdag.add_edge(&mine_rho, &merge_id, HDAGEdge::data()).ok();
        hdag.add_edge(&mine_omega, &merge_id, HDAGEdge::data()).ok();

        hdag.add_edge(&merge_id, &compile_id, HDAGEdge::data()).ok();
        hdag.add_edge(&compile_id, &output_id, HDAGEdge::data()).ok();

        hdag.compute_execution_order().ok();
        hdag
    }

    /// Integrate slot artifacts into hypercube
    pub fn integrate(&self, artifacts: &[SlotArtifact]) -> Vec<HypercubeArtifact> {
        artifacts.iter()
            .map(|a| a.to_hypercube_artifact())
            .collect()
    }

    /// Get all generated artifacts
    pub fn artifacts(&self) -> &[SlotArtifact] {
        &self.artifacts
    }

    /// Get best artifact by resonance
    pub fn best_artifact(&self) -> Option<&SlotArtifact> {
        self.artifacts.iter()
            .max_by(|a, b| a.resonance.partial_cmp(&b.resonance).unwrap())
    }

    /// Clear artifacts
    pub fn clear(&mut self) {
        self.artifacts.clear();
    }
}

/// Combined Hypercube-Slots mode
pub struct HypercubeSlotsMode {
    adapter: SlotsHypercubeAdapter,
    hypercube_coord: Option<Coord5D>,
}

impl HypercubeSlotsMode {
    /// Create new mode
    pub fn new() -> Self {
        Self {
            adapter: SlotsHypercubeAdapter::default_adapter(),
            hypercube_coord: None,
        }
    }

    /// Set hypercube coordinate
    pub fn set_coordinate(&mut self, coord: Coord5D) {
        self.hypercube_coord = Some(coord);
    }

    /// Run slots inside cube execution
    pub fn execute(&mut self) -> Result<Vec<SlotArtifact>> {
        let coord = self.hypercube_coord.unwrap_or(Coord5D::center());
        self.adapter.generate_from_coord(coord)
    }

    /// Get adapter
    pub fn adapter(&self) -> &SlotsHypercubeAdapter {
        &self.adapter
    }

    /// Get mutable adapter
    pub fn adapter_mut(&mut self) -> &mut SlotsHypercubeAdapter {
        &mut self.adapter
    }
}

impl Default for HypercubeSlotsMode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slot::SlotSymbol;

    #[test]
    fn test_slot_artifact_creation() {
        let symbols = vec![SlotSymbol::Psi, SlotSymbol::Rho, SlotSymbol::Omega];
        let values = vec![0.8, 0.7, 0.6];
        let seq = MinedSequence::new(symbols, values, 0);

        let artifact = SlotArtifact::from_sequence(seq, Some("test_node"));

        assert!(artifact.resonance > 0.0);
        assert!(artifact.source_node.is_some());
    }

    #[test]
    fn test_adapter_creation() {
        let adapter = SlotsHypercubeAdapter::default_adapter();
        assert!(adapter.artifacts.is_empty());
    }

    #[test]
    fn test_slots_hdag() {
        let adapter = SlotsHypercubeAdapter::default_adapter();
        let hdag = adapter.create_slots_hdag(Coord5D::center());

        assert!(hdag.node_count() > 0);
    }

    #[test]
    fn test_hypercube_slots_mode() {
        let mut mode = HypercubeSlotsMode::new();
        mode.set_coordinate(Coord5D::new(0.7, 0.6, 0.5, 0.4, 0.3));

        let artifacts = mode.execute().unwrap();
        assert!(!artifacts.is_empty());
    }
}
