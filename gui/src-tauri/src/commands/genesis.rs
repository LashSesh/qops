//! Genesis-related Tauri commands

use super::*;
use crate::error::{AppError, Result};
use crate::state::AppState;
use qops_genesis::{TraversalEngine, AgentConfig, TraversalStrategy, MetatronCube};
use qops_core::ResonanceTopology;
use tauri::State;

/// Run Genesis operator mining
#[tauri::command]
pub async fn run_genesis_mining(
    state: State<'_, AppState>,
    agents: usize,
    steps: usize,
    strategy: String,
) -> Result<GenesisResultDto> {
    if agents == 0 || agents > 100 {
        return Err(AppError::InvalidParameter(
            "Agent count must be between 1 and 100".to_string(),
        ));
    }

    let strat = match strategy.to_lowercase().as_str() {
        "balanced" => TraversalStrategy::Balanced,
        "explorative" => TraversalStrategy::Explorative,
        "exploitative" => TraversalStrategy::Exploitative,
        "random" => TraversalStrategy::Random,
        _ => TraversalStrategy::Balanced,
    };

    let config = AgentConfig {
        max_steps: steps,
        strategy: strat,
        ..Default::default()
    };

    let mut engine = TraversalEngine::new();
    let artefacts = engine.run_swarm(agents, config);

    let artefact_dtos: Vec<ArtefactDto> = artefacts
        .iter()
        .enumerate()
        .map(|(i, a)| ArtefactDto {
            id: i,
            resonance: a.resonance,
            is_mandorla: a.is_mandorla,
            node_path: vec![], // Simplified for now
        })
        .collect();

    let best_resonance = artefacts
        .iter()
        .map(|a| a.resonance)
        .fold(0.0, f64::max);

    let mandorla_count = artefacts.iter().filter(|a| a.is_mandorla).count();

    // Store topology for later queries
    {
        let mut topo = state.s7_topology.lock().unwrap();
        *topo = Some(MetatronCube::new());
    }

    Ok(GenesisResultDto {
        artefacts: artefact_dtos,
        best_resonance,
        mandorla_count,
        total_steps: agents * steps,
    })
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
