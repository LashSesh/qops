//! Tauri commands module

pub mod algorithms;
pub mod calibration;
pub mod circuits;
pub mod genesis;
pub mod holistic;
pub mod hypercube;
pub mod kernel;
pub mod quantum;
pub mod research;
pub mod slots;
pub mod system;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Data Transfer Objects (DTOs)
// ============================================================================

/// Complex number DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexDto {
    pub re: f64,
    pub im: f64,
}

/// Gate DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateDto {
    pub name: String,
    pub gate_type: String,
    pub qubits: Vec<usize>,
    pub parameter: Option<f64>,
}

/// Circuit DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitDto {
    pub id: String,
    pub name: String,
    pub qubits: usize,
    pub depth: usize,
    pub gate_count: usize,
    pub gates: Vec<GateDto>,
}

/// Simulation result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResultDto {
    pub probabilities: Vec<f64>,
    pub counts: HashMap<String, usize>,
    pub shots: usize,
}

/// Grover result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroverResultDto {
    pub measured_state: usize,
    pub measured_state_binary: String,
    pub success_probability: f64,
    pub iterations: usize,
    pub is_solution: bool,
    pub counts: HashMap<String, usize>,
    pub theoretical_probability: f64,
}

/// Shor result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShorResultDto {
    pub success: bool,
    pub number: u64,
    pub factors: Vec<u64>,
    pub period: Option<u64>,
    pub attempts: usize,
}

/// QFT result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QftResultDto {
    pub qubits: usize,
    pub depth: usize,
    pub gate_count: usize,
    pub output_probabilities: Vec<f64>,
}

/// QPE result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QpeResultDto {
    pub estimated_phase: f64,
    pub true_phase: f64,
    pub error: f64,
    pub confidence: f64,
}

/// VQE result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VqeResultDto {
    pub energy: f64,
    pub iterations: usize,
    pub converged: bool,
    pub variance: f64,
    pub parameters: Vec<f64>,
}

/// QAOA result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QaoaResultDto {
    pub best_solution: Vec<bool>,
    pub best_cost: f64,
    pub approximation_ratio: f64,
    pub solution_counts: HashMap<String, usize>,
}

/// Genesis result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisResultDto {
    pub artefacts: Vec<ArtefactDto>,
    pub best_resonance: f64,
    pub mandorla_count: usize,
    pub total_steps: usize,
}

/// Artefact DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtefactDto {
    pub id: usize,
    pub resonance: f64,
    pub is_mandorla: bool,
    pub node_path: Vec<usize>,
}

/// Topology info DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyInfoDto {
    pub node_count: usize,
    pub edge_count: usize,
    pub topology_type: String,
}

/// Node details DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDetailsDto {
    pub id: usize,
    pub permutation: Vec<u8>,
    pub signature: SignatureDto,
    pub neighbor_count: usize,
}

/// Signature DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureDto {
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,
    pub chi: Option<f64>,
    pub eta: Option<f64>,
}

/// Quantum walk result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumWalkResultDto {
    pub time_points: Vec<f64>,
    pub center_probabilities: Vec<f64>,
    pub hex_probabilities: Vec<f64>,
    pub cube_probabilities: Vec<f64>,
}

/// Calibration result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationResultDto {
    pub steps: Vec<CalibrationStepDto>,
    pub final_signature: SignatureDto,
    pub accepted_count: usize,
}

/// Calibration step DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationStepDto {
    pub step: usize,
    pub signature: SignatureDto,
    pub accepted: bool,
    pub cri_triggered: bool,
}

/// Experiment result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResultDto {
    pub name: String,
    pub total_runs: usize,
    pub results: Vec<serde_json::Value>,
    pub summary: HashMap<String, f64>,
}

/// Benchmark result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResultDto {
    pub algorithm: String,
    pub configurations: Vec<BenchmarkConfigDto>,
    pub comparison_table: String,
}

/// Benchmark config DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfigDto {
    pub qubits: usize,
    pub mean_time_ms: f64,
    pub std_time_ms: f64,
    pub success_rate: f64,
}

/// System info DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfoDto {
    pub version: String,
    pub modules: Vec<String>,
    pub capabilities: Vec<String>,
}

// ============================================================================
// Hypercube DTOs
// ============================================================================

/// 5D Coordinate DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coord5DDto {
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,
    pub chi: f64,
    pub eta: f64,
}

/// Hypercube vertex DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypercubeVertexDto {
    pub id: String,
    pub coordinate: Coord5DDto,
    pub resonance: f64,
    pub depth: usize,
}

/// Hypercube statistics DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypercubeStatsDto {
    pub total_vertices: usize,
    pub total_edges: usize,
    pub max_depth_reached: usize,
    pub best_resonance: f64,
    pub avg_resonance: f64,
}

/// Compilation result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResultDto {
    pub output: Coord5DDto,
    pub resonance: f64,
    pub iterations: usize,
    pub threshold_met: bool,
    pub artifact_count: usize,
}

/// HDAG node DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDAGNodeDto {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub input: Option<Coord5DDto>,
    pub output: Option<Coord5DDto>,
}

/// HDAG edge DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDAGEdgeDto {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub edge_type: String,
}

/// HDAG info DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDAGInfoDto {
    pub name: String,
    pub nodes: Vec<HDAGNodeDto>,
    pub edges: Vec<HDAGEdgeDto>,
    pub execution_order: Vec<String>,
}

/// HDAG execution result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDAGExecutionResultDto {
    pub output: Coord5DDto,
    pub resonance: f64,
    pub nodes_executed: usize,
    pub nodes_failed: usize,
    pub total_time_ms: u64,
    pub artifact_count: usize,
}

/// Hypercube session result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypercubeSessionResultDto {
    pub session_id: String,
    pub state: String,
    pub best_coordinate: Coord5DDto,
    pub best_resonance: f64,
    pub compilation_result: Option<CompilationResultDto>,
    pub total_time_ms: u64,
    pub expansion_steps: usize,
    pub total_vertices: usize,
    pub artifact_count: usize,
}

// ============================================================================
// Slots DTOs
// ============================================================================

/// Slot symbol DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotSymbolDto {
    pub name: String,
    pub weight: f64,
}

/// Slot value DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotValueDto {
    pub symbol: SlotSymbolDto,
    pub value: f64,
    pub entropy: f64,
}

/// Mined sequence DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinedSequenceDto {
    pub id: String,
    pub symbols: Vec<String>,
    pub values: Vec<f64>,
    pub resonance: f64,
    pub coord5d: [f64; 5],
    pub depth: usize,
}

/// Slots mining result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotsMiningResultDto {
    pub best_resonance: f64,
    pub total_steps: usize,
    pub steps_to_best: usize,
    pub mining_time_ms: u64,
    pub converged: bool,
    pub top_sequences: Vec<MinedSequenceDto>,
}

/// Slots session result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotsSessionResultDto {
    pub session_id: String,
    pub spin_count: usize,
    pub best_resonance: f64,
    pub best_sequence: Option<MinedSequenceDto>,
    pub mining_result: Option<SlotsMiningResultDto>,
    pub total_time_ms: u64,
}

/// Slot artifact DTO (for hypercube integration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotArtifactDto {
    pub id: String,
    pub name: String,
    pub coordinate: Coord5DDto,
    pub resonance: f64,
    pub source_node: Option<String>,
}

/// Slots configuration DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotsConfigDto {
    pub entropy_distribution: String,
    pub mining_strategy: String,
    pub mining_depth: usize,
    pub target_resonance: f64,
}
