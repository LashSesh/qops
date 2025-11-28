//! Quantum-related Tauri commands (Cube-13 operations)

use super::*;
use crate::error::{AppError, Result};
use qops_quantum::{MetatronGraph, MetatronHamiltonian, QuantumState};
use qops_quantum::quantum_walk::ContinuousQuantumWalk;
use qops_quantum::vqa::VQE;

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
