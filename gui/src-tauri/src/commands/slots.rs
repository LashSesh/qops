//! Slots Tauri commands
//!
//! Commands for the Quantum Slots Engine, including slot spinning, sequence mining,
//! and hypercube integration.

use super::{
    Coord5DDto, MinedSequenceDto, SlotsMiningResultDto, SlotsSessionResultDto,
    SlotArtifactDto, SlotsConfigDto,
};
use qops_slots::{
    SlotsSession, SlotsSessionConfig,
    SequenceMiner, MinerConfig, MiningStrategy,
    EntropyConfig, EntropyDistribution,
    SlotSymbol,
    hypercube_integration::{SlotsHypercubeAdapter, AdapterConfig, HypercubeSlotsMode},
};
use qops_hypercube::Coord5D;

// ============================================================================
// Conversion helpers
// ============================================================================

fn coord5d_to_dto(c: &Coord5D) -> Coord5DDto {
    Coord5DDto {
        psi: c.psi,
        rho: c.rho,
        omega: c.omega,
        chi: c.chi,
        eta: c.eta,
    }
}

fn mined_sequence_to_dto(seq: &qops_slots::MinedSequence) -> MinedSequenceDto {
    MinedSequenceDto {
        id: seq.id.clone(),
        symbols: seq.symbols.iter().map(|s| format!("{}", s)).collect(),
        values: seq.values.clone(),
        resonance: seq.resonance,
        coord5d: seq.coord5d,
        depth: seq.depth,
    }
}

fn parse_entropy_distribution(name: &str) -> EntropyDistribution {
    match name {
        "uniform" => EntropyDistribution::Uniform,
        "normal" => EntropyDistribution::Normal { mean: 0.5, std_dev: 0.2 },
        "exponential" => EntropyDistribution::Exponential { lambda: 2.0 },
        "beta" => EntropyDistribution::Beta { alpha: 2.0, beta: 5.0 },
        "bimodal" => EntropyDistribution::Bimodal { peak1: 0.3, peak2: 0.8 },
        "resonance" => EntropyDistribution::ResonanceOptimized,
        _ => EntropyDistribution::Uniform,
    }
}

fn parse_mining_strategy(name: &str) -> MiningStrategy {
    match name {
        "greedy" => MiningStrategy::Greedy,
        "stochastic" => MiningStrategy::Stochastic,
        "beam" => MiningStrategy::BeamSearch,
        "evolutionary" => MiningStrategy::Evolutionary,
        "triton" => MiningStrategy::Triton,
        _ => MiningStrategy::BeamSearch,
    }
}

// ============================================================================
// Slots engine commands
// ============================================================================

/// Run the slots engine with configuration
#[tauri::command]
pub fn run_slots_engine(
    steps: usize,
    entropy_distribution: String,
    mining_strategy: String,
    target_resonance: f64,
) -> Result<SlotsSessionResultDto, String> {
    let entropy_config = EntropyConfig {
        distribution: parse_entropy_distribution(&entropy_distribution),
        seed: None,
        ..Default::default()
    };

    let miner_config = MinerConfig {
        depth: steps,
        strategy: parse_mining_strategy(&mining_strategy),
        target_resonance,
        ..Default::default()
    };

    let config = SlotsSessionConfig {
        entropy_config,
        miner_config,
        spins_before_mine: steps.min(20),
        ..Default::default()
    };

    let mut session = SlotsSession::new(config);
    let result = session.run()
        .map_err(|e| format!("Slots session failed: {}", e))?;

    Ok(SlotsSessionResultDto {
        session_id: result.session_id,
        spin_count: result.spin_count,
        best_resonance: result.best_resonance,
        best_sequence: result.best_sequence.as_ref().map(|s| mined_sequence_to_dto(s)),
        mining_result: result.mining_result.as_ref().map(|mr| SlotsMiningResultDto {
            best_resonance: mr.best_resonance,
            total_steps: mr.total_steps,
            steps_to_best: mr.steps_to_best,
            mining_time_ms: mr.mining_time_ms,
            converged: mr.converged,
            top_sequences: mr.top_sequences.iter().map(|s| mined_sequence_to_dto(s)).collect(),
        }),
        total_time_ms: result.total_time_ms,
    })
}

/// Mine operator sequences
#[tauri::command]
pub fn slots_mine_sequence(
    depth: usize,
    strategy: String,
    target_resonance: f64,
    beam_width: Option<usize>,
) -> Result<SlotsMiningResultDto, String> {
    let config = MinerConfig {
        depth,
        strategy: parse_mining_strategy(&strategy),
        target_resonance,
        beam_width: beam_width.unwrap_or(10),
        ..Default::default()
    };

    let mut miner = SequenceMiner::new(config);
    let result = miner.mine()
        .map_err(|e| format!("Mining failed: {}", e))?;

    Ok(SlotsMiningResultDto {
        best_resonance: result.best_resonance,
        total_steps: result.total_steps,
        steps_to_best: result.steps_to_best,
        mining_time_ms: result.mining_time_ms,
        converged: result.converged,
        top_sequences: result.top_sequences.iter().map(|s| mined_sequence_to_dto(s)).collect(),
    })
}

/// Get slots engine info
#[tauri::command]
pub fn get_slots_info() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "name": "Quantum Slots Engine (QSlots)",
        "version": "0.1.0",
        "symbols": [
            { "name": "Psi", "weight": 0.4, "description": "Quality / Spectral coherence" },
            { "name": "Rho", "weight": 0.3, "description": "Stability / Robustness" },
            { "name": "Omega", "weight": 0.3, "description": "Efficiency / Performance" },
            { "name": "Chi", "weight": 0.05, "description": "Topological coherence" },
            { "name": "Eta", "weight": -0.05, "description": "Fluctuation measure" },
            { "name": "Star", "weight": 0.1, "description": "Bonus multiplier" },
            { "name": "Diamond", "weight": 0.15, "description": "High-value symbol" },
            { "name": "Circle", "weight": 0.05, "description": "Completion symbol" }
        ],
        "entropy_distributions": [
            "uniform", "normal", "exponential", "beta", "bimodal", "resonance"
        ],
        "mining_strategies": [
            "greedy", "stochastic", "beam", "evolutionary", "triton"
        ],
        "features": [
            "Slot evaluation engine",
            "Entropy mapping",
            "Lattice search",
            "Sequence mining",
            "Hypercube integration"
        ]
    }))
}

/// Get available mining strategies
#[tauri::command]
pub fn get_mining_strategies() -> Result<Vec<serde_json::Value>, String> {
    Ok(vec![
        serde_json::json!({
            "name": "greedy",
            "description": "Always take the best immediate outcome"
        }),
        serde_json::json!({
            "name": "stochastic",
            "description": "Simulated annealing with temperature decay"
        }),
        serde_json::json!({
            "name": "beam",
            "description": "Beam search maintaining multiple candidates"
        }),
        serde_json::json!({
            "name": "evolutionary",
            "description": "Genetic algorithm with mutation and crossover"
        }),
        serde_json::json!({
            "name": "triton",
            "description": "TRITON spiral search optimization"
        }),
    ])
}

/// Get available entropy distributions
#[tauri::command]
pub fn get_entropy_distributions() -> Result<Vec<serde_json::Value>, String> {
    Ok(vec![
        serde_json::json!({
            "name": "uniform",
            "description": "Equal probability across range"
        }),
        serde_json::json!({
            "name": "normal",
            "description": "Gaussian distribution centered at 0.5"
        }),
        serde_json::json!({
            "name": "exponential",
            "description": "Exponential decay distribution"
        }),
        serde_json::json!({
            "name": "beta",
            "description": "Beta distribution for bounded values"
        }),
        serde_json::json!({
            "name": "bimodal",
            "description": "Two-peak distribution"
        }),
        serde_json::json!({
            "name": "resonance",
            "description": "Optimized for high resonance outcomes"
        }),
    ])
}

// ============================================================================
// Hypercube integration commands
// ============================================================================

/// Generate slot artifacts from a 5D coordinate
#[tauri::command]
pub fn slots_generate_artifacts(
    coord_psi: f64,
    coord_rho: f64,
    coord_omega: f64,
    coord_chi: f64,
    coord_eta: f64,
) -> Result<Vec<SlotArtifactDto>, String> {
    let coord = Coord5D::new(coord_psi, coord_rho, coord_omega, coord_chi, coord_eta);

    let mut adapter = SlotsHypercubeAdapter::default_adapter();
    let artifacts = adapter.generate_from_coord(coord)
        .map_err(|e| format!("Artifact generation failed: {}", e))?;

    Ok(artifacts.iter().map(|a| SlotArtifactDto {
        id: a.id.clone(),
        name: a.name.clone(),
        coordinate: coord5d_to_dto(&a.coordinate),
        resonance: a.resonance,
        source_node: a.source_node.clone(),
    }).collect())
}

/// Run slots inside hypercube mode
#[tauri::command]
pub fn run_hypercube_slots_mode(
    coord_psi: Option<f64>,
    coord_rho: Option<f64>,
    coord_omega: Option<f64>,
    coord_chi: Option<f64>,
    coord_eta: Option<f64>,
) -> Result<Vec<SlotArtifactDto>, String> {
    let mut mode = HypercubeSlotsMode::new();

    if let (Some(psi), Some(rho), Some(omega), Some(chi), Some(eta)) =
        (coord_psi, coord_rho, coord_omega, coord_chi, coord_eta) {
        mode.set_coordinate(Coord5D::new(psi, rho, omega, chi, eta));
    }

    let artifacts = mode.execute()
        .map_err(|e| format!("Hypercube slots mode failed: {}", e))?;

    Ok(artifacts.iter().map(|a| SlotArtifactDto {
        id: a.id.clone(),
        name: a.name.clone(),
        coordinate: coord5d_to_dto(&a.coordinate),
        resonance: a.resonance,
        source_node: a.source_node.clone(),
    }).collect())
}

/// Get slots configuration options
#[tauri::command]
pub fn get_slots_config_options() -> Result<SlotsConfigDto, String> {
    Ok(SlotsConfigDto {
        entropy_distribution: "uniform".to_string(),
        mining_strategy: "beam".to_string(),
        mining_depth: 10,
        target_resonance: 0.8,
    })
}
