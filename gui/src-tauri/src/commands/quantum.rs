//! Quantum-related Tauri commands (Cube-13 operations)
//!
//! Advanced Cube-13 topology exploration, quantum walks, and VQE.

use super::*;
use crate::error::{AppError, Result};
use qops_quantum::{MetatronGraph, MetatronHamiltonian, QuantumState};
use qops_quantum::quantum_walk::ContinuousQuantumWalk;
use qops_quantum::vqa::VQE;
use qops_quantum::topology::{Cube13Engine, Cube13NodeType, TopologyMetrics};
use qops_core::Signature5D;

/// Cube-13 topology metrics DTO
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cube13MetricsDto {
    pub avg_resonance: f64,
    pub center_resonance: f64,
    pub hexagon_avg_resonance: f64,
    pub cube_avg_resonance: f64,
    pub coherence: f64,
    pub embedding_count: usize,
    pub coverage: f64,
}

/// Cube-13 node details DTO
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cube13NodeDto {
    pub id: usize,
    pub node_type: String,
    pub centrality: f64,
    pub neighbors: Vec<usize>,
    pub embedding: Option<SignatureDto>,
}

/// Topology walk result DTO
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TopologyWalkDto {
    pub path: Vec<usize>,
    pub coverage: f64,
    pub final_node: usize,
    pub steps_taken: usize,
}

/// Run continuous-time quantum walk on Cube-13
#[tauri::command]
pub async fn run_quantum_walk(times: Vec<f64>) -> Result<QuantumWalkResultDto> {
    if times.is_empty() {
        return Err(AppError::InvalidParameter(
            "At least one time point required".to_string(),
        ));
    }

    let graph = MetatronGraph::new();
    let hamiltonian = MetatronHamiltonian::from_graph(&graph);
    let qw = ContinuousQuantumWalk::new(hamiltonian);

    let initial = QuantumState::basis_state(0)
        .map_err(|e| AppError::Quantum(e.to_string()))?;

    let mut center_probs = Vec::new();
    let mut hex_probs = Vec::new();
    let mut cube_probs = Vec::new();

    for t in &times {
        let evolved = qw.evolve(&initial, *t);
        let probs = evolved.probabilities();

        center_probs.push(probs[0]);
        hex_probs.push(probs[1..7].iter().sum());
        cube_probs.push(probs[7..13].iter().sum());
    }

    Ok(QuantumWalkResultDto {
        time_points: times,
        center_probabilities: center_probs,
        hex_probabilities: hex_probs,
        cube_probabilities: cube_probs,
    })
}

/// Get Cube-13 graph information
#[tauri::command]
pub async fn get_cube13_info() -> Result<TopologyInfoDto> {
    let graph = MetatronGraph::new();

    Ok(TopologyInfoDto {
        node_count: 13,
        edge_count: graph.edges().len(),
        topology_type: "Metatron Cube-13".to_string(),
    })
}

/// Get detailed Cube-13 topology metrics
#[tauri::command]
pub async fn get_cube13_metrics() -> Result<Cube13MetricsDto> {
    let engine = Cube13Engine::new();
    let metrics = engine.compute_metrics();

    Ok(Cube13MetricsDto {
        avg_resonance: metrics.avg_resonance,
        center_resonance: metrics.center_resonance,
        hexagon_avg_resonance: metrics.hexagon_avg_resonance,
        cube_avg_resonance: metrics.cube_avg_resonance,
        coherence: metrics.coherence,
        embedding_count: metrics.embedding_count,
        coverage: metrics.coverage,
    })
}

/// Get details for a specific Cube-13 node
#[tauri::command]
pub async fn get_cube13_node(node_id: usize) -> Result<Cube13NodeDto> {
    if node_id >= 13 {
        return Err(AppError::InvalidParameter(
            format!("Node ID must be 0-12, got {}", node_id),
        ));
    }

    let engine = Cube13Engine::new();
    let node_type = Cube13NodeType::from_index(node_id)
        .map(|t| match t {
            Cube13NodeType::Center => "center",
            Cube13NodeType::Hexagon => "hexagon",
            Cube13NodeType::Cube => "cube",
        })
        .unwrap_or("unknown");

    let neighbors = engine.graph().neighbors(&node_id);
    let centrality = engine.node_centrality(node_id);

    Ok(Cube13NodeDto {
        id: node_id,
        node_type: node_type.to_string(),
        centrality,
        neighbors,
        embedding: engine.embedding_at(node_id).map(|s| SignatureDto {
            psi: s.psi,
            rho: s.rho,
            omega: s.omega,
            chi: Some(s.chi),
            eta: Some(s.eta),
        }),
    })
}

/// Run topology walk on Cube-13
#[tauri::command]
pub async fn run_cube13_walk(start: usize, steps: usize) -> Result<TopologyWalkDto> {
    if start >= 13 {
        return Err(AppError::InvalidParameter(
            format!("Start node must be 0-12, got {}", start),
        ));
    }

    let mut engine = Cube13Engine::new();

    // Embed some signatures for the walk to use
    for i in 0..13 {
        let sig = Signature5D::new(
            0.5 + (i as f64 * 0.03),
            0.6 + (i as f64 * 0.02),
            0.5 + (i as f64 * 0.025),
            0.5,
            0.2,
        );
        engine.embed(i, sig);
    }

    let result = engine.topology_walk(start, steps);

    Ok(TopologyWalkDto {
        path: result.path,
        coverage: result.coverage,
        final_node: result.final_node,
        steps_taken: result.steps_taken,
    })
}

/// Embed a signature into Cube-13 topology
#[tauri::command]
pub async fn embed_in_cube13(
    node: usize,
    psi: f64,
    rho: f64,
    omega: f64,
    chi: f64,
    eta: f64,
) -> Result<Cube13MetricsDto> {
    if node >= 13 {
        return Err(AppError::InvalidParameter(
            format!("Node must be 0-12, got {}", node),
        ));
    }

    let mut engine = Cube13Engine::new();
    let sig = Signature5D::new(psi, rho, omega, chi, eta);
    engine.embed(node, sig);

    let metrics = engine.compute_metrics();

    Ok(Cube13MetricsDto {
        avg_resonance: metrics.avg_resonance,
        center_resonance: metrics.center_resonance,
        hexagon_avg_resonance: metrics.hexagon_avg_resonance,
        cube_avg_resonance: metrics.cube_avg_resonance,
        coherence: metrics.coherence,
        embedding_count: metrics.embedding_count,
        coverage: metrics.coverage,
    })
}

/// Run VQE on Cube-13 graph
#[tauri::command]
pub async fn run_cube13_vqe(layers: usize) -> Result<VqeResultDto> {
    let graph = MetatronGraph::new();
    let hamiltonian = MetatronHamiltonian::from_graph(&graph);
    let vqe = VQE::new(hamiltonian, layers);

    let result = vqe.run();

    Ok(VqeResultDto {
        energy: result.ground_energy,
        iterations: result.iterations,
        converged: result.converged,
        variance: 0.0, // Not provided by legacy VQE
        parameters: vec![],
    })
}
