//! Genesis-related Tauri commands
//!
//! Provides TRITON-powered mining with operator family extraction.

use super::*;
use crate::error::{AppError, Result};
use crate::state::AppState;
use qops_genesis::{MiningSession, MiningConfig, MiningStrategy, MetatronCube};
use qops_core::ResonanceTopology;
use tauri::State;

/// Operator family DTO
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FamilyDto {
    pub name: String,
    pub member_count: usize,
    pub avg_resonance: f64,
    pub characteristics: FamilyCharacteristicsDto,
}

/// Family characteristics DTO
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FamilyCharacteristicsDto {
    pub is_high_quality: bool,
    pub is_stable: bool,
    pub is_efficient: bool,
}

/// Helper function to derive characteristics from OperatorFamily
fn derive_family_characteristics(avg_resonance: f64, coherence: f64, avg_psi: f64) -> FamilyCharacteristicsDto {
    FamilyCharacteristicsDto {
        is_high_quality: avg_resonance >= 0.7,
        is_stable: coherence >= 0.5,
        is_efficient: avg_psi >= 0.6,
    }
}

/// Extended Genesis result with families and TRITON info
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtendedGenesisResultDto {
    pub artefacts: Vec<ArtefactDto>,
    pub best_resonance: f64,
    pub mandorla_count: usize,
    pub total_steps: usize,
    pub families: Vec<FamilyDto>,
    pub triton_info: Option<TritonInfoDto>,
    pub stats: MiningStatsDto,
}

/// TRITON optimization info
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TritonInfoDto {
    pub best_score: f64,
    pub iterations: usize,
    pub converged: bool,
}

/// Mining statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MiningStatsDto {
    pub avg_resonance: f64,
    pub std_resonance: f64,
    pub unique_nodes: usize,
    pub duration_ms: u64,
}

/// Run Genesis operator mining with TRITON strategies
#[tauri::command]
pub async fn run_genesis_mining(
    state: State<'_, AppState>,
    agents: usize,
    steps: usize,
    strategy: String,
) -> Result<ExtendedGenesisResultDto> {
    if agents == 0 || agents > 100 {
        return Err(AppError::InvalidParameter(
            "Agent count must be between 1 and 100".to_string(),
        ));
    }

    let mining_strategy = match strategy.to_lowercase().as_str() {
        "balanced" => MiningStrategy::Balanced,
        "explorative" => MiningStrategy::Explorative,
        "exploitative" => MiningStrategy::Exploitative,
        "random" => MiningStrategy::Random,
        "triton" => MiningStrategy::Triton,
        "hybrid" | "hybrid_triton" => MiningStrategy::HybridTritonEvolution,
        "swarm" => MiningStrategy::Swarm,
        "evolutionary" => MiningStrategy::Evolutionary,
        _ => MiningStrategy::Balanced,
    };

    let config = MiningConfig {
        strategy: mining_strategy,
        num_agents: agents,
        steps_per_agent: steps,
        extract_families: true,
        ..Default::default()
    };

    let mut session = MiningSession::new(config);
    let result = session.mine();

    let artefact_dtos: Vec<ArtefactDto> = result.artefacts
        .iter()
        .enumerate()
        .map(|(i, a)| ArtefactDto {
            id: i,
            resonance: a.resonance,
            is_mandorla: a.is_mandorla(),
            node_path: vec![],
        })
        .collect();

    let family_dtos: Vec<FamilyDto> = result.families
        .iter()
        .map(|f| {
            let avg_res = f.avg_resonance();
            FamilyDto {
                name: f.name.clone(),
                member_count: f.members().len(),
                avg_resonance: avg_res,
                characteristics: derive_family_characteristics(
                    avg_res,
                    f.characteristics.coherence,
                    f.characteristics.avg_psi,
                ),
            }
        })
        .collect();

    let triton_info = result.triton_result.as_ref().map(|t| TritonInfoDto {
        best_score: t.best_score,
        iterations: t.iterations,
        converged: t.converged,
    });

    // Store topology for later queries
    {
        let mut topo = state.s7_topology.lock().unwrap();
        *topo = Some(MetatronCube::new());
    }

    Ok(ExtendedGenesisResultDto {
        artefacts: artefact_dtos,
        best_resonance: result.best_artefact.as_ref().map(|a| a.resonance).unwrap_or(0.0),
        mandorla_count: result.mandorla_count,
        total_steps: agents * steps,
        families: family_dtos,
        triton_info,
        stats: MiningStatsDto {
            avg_resonance: result.stats.avg_resonance,
            std_resonance: result.stats.std_resonance,
            unique_nodes: result.stats.unique_nodes,
            duration_ms: result.duration_ms,
        },
    })
}

/// Get operator families from the last mining session
#[tauri::command]
pub async fn get_genesis_families(
    state: State<'_, AppState>,
) -> Result<Vec<FamilyDto>> {
    // For now, return empty - in full implementation would cache families
    Ok(vec![])
}

/// Get S7 topology information
#[tauri::command]
pub async fn get_s7_topology_info(state: State<'_, AppState>) -> Result<TopologyInfoDto> {
    let topo_lock = state.s7_topology.lock().unwrap();

    let topology = match topo_lock.as_ref() {
        Some(t) => t,
        None => {
            // Create a new topology if not exists
            drop(topo_lock);
            let mut topo = state.s7_topology.lock().unwrap();
            *topo = Some(MetatronCube::new());
            drop(topo);

            let topo = state.s7_topology.lock().unwrap();
            return Ok(TopologyInfoDto {
                node_count: topo.as_ref().unwrap().node_count(),
                edge_count: topo.as_ref().unwrap().edge_count(),
                topology_type: "S7 Permutation Group".to_string(),
            });
        }
    };

    Ok(TopologyInfoDto {
        node_count: topology.node_count(),
        edge_count: topology.edge_count(),
        topology_type: "S7 Permutation Group".to_string(),
    })
}

/// Get node details
#[tauri::command]
pub async fn get_node_details(
    state: State<'_, AppState>,
    node_id: usize,
) -> Result<NodeDetailsDto> {
    let topo_lock = state.s7_topology.lock().unwrap();
    let topology = topo_lock
        .as_ref()
        .ok_or_else(|| AppError::State("Topology not initialized".to_string()))?;

    let nodes = topology.nodes();
    if node_id >= nodes.len() {
        return Err(AppError::NotFound(format!("Node {}", node_id)));
    }

    let node = nodes[node_id];
    let neighbors = topology.neighbors(&node);

    let signature = topology
        .signature_at(&node)
        .map(|s| {
            let s5d = s.to_5d();
            SignatureDto {
                psi: s5d.psi,
                rho: s5d.rho,
                omega: s5d.omega,
                chi: Some(s5d.chi),
                eta: Some(s5d.eta),
            }
        })
        .unwrap_or(SignatureDto {
            psi: 0.0,
            rho: 0.0,
            omega: 0.0,
            chi: None,
            eta: None,
        });

    // Get permutation from the cube
    let permutation = topology
        .permutation(node)
        .map(|p| p.to_vec())
        .unwrap_or_default();

    Ok(NodeDetailsDto {
        id: node_id,
        permutation,
        signature,
        neighbor_count: neighbors.len(),
    })
}
