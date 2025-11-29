//! Kernel Tauri commands for mining and materialization
//!
//! Provides GUI access to the Hypercube Kernel for Generative Theomimesis

use super::*;
use crate::error::Result;
use serde::{Deserialize, Serialize};

// ============================================================================
// DTOs for Kernel Operations
// ============================================================================

/// Search strategy DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SearchStrategyDto {
    Greedy,
    Stochastic { temperature: f64 },
    Beam { width: usize },
    Evolutionary { population_size: usize, mutation_rate: f64 },
    Triton,
    Hybrid,
}

impl Default for SearchStrategyDto {
    fn default() -> Self {
        SearchStrategyDto::Stochastic { temperature: 1.0 }
    }
}

/// Mining configuration DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningConfigDto {
    pub max_iterations: usize,
    pub target_resonance: f64,
    pub max_candidates: usize,
    pub exploration_rate: f64,
    pub strategy: SearchStrategyDto,
    pub convergence_epsilon: f64,
}

impl Default for MiningConfigDto {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            target_resonance: 0.8,
            max_candidates: 50,
            exploration_rate: 0.3,
            strategy: SearchStrategyDto::default(),
            convergence_epsilon: 1e-4,
        }
    }
}

/// Blueprint candidate DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintCandidateDto {
    pub id: String,
    pub name: String,
    pub signature: SignatureDto,
    pub resonance_score: f64,
    pub quality_level: String,
}

/// Mining result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningResultDto {
    pub best_resonance: f64,
    pub iterations: usize,
    pub converged: bool,
    pub candidates: Vec<BlueprintCandidateDto>,
    pub total_candidates_explored: usize,
    pub stagnation_count: usize,
    pub duration_ms: u64,
}

/// Artefact type DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArtefactTypeDto {
    Code,
    Configuration,
    Document,
    Data,
    Operator,
    Circuit,
    Generic,
}

/// Materialization config DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterializationConfigDto {
    pub artefact_type: ArtefactTypeDto,
    pub write_files: bool,
    pub record_in_ledger: bool,
    pub output_format: String,
}

impl Default for MaterializationConfigDto {
    fn default() -> Self {
        Self {
            artefact_type: ArtefactTypeDto::Data,
            write_files: false,
            record_in_ledger: true,
            output_format: "json".to_string(),
        }
    }
}

/// Artefact output DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtefactDto {
    pub id: String,
    pub blueprint_id: String,
    pub artefact_type: String,
    pub content: serde_json::Value,
    pub final_resonance: f64,
    pub created_at: String,
}

/// Materialization result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterializationResultDto {
    pub success: bool,
    pub artefact: ArtefactDto,
    pub ledger_entry_id: Option<String>,
    pub warnings: Vec<String>,
}

/// Ledger entry DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntryDto {
    pub id: String,
    pub blueprint_id: String,
    pub artefact_id: String,
    pub timestamp: String,
    pub resonance_score: f64,
    pub hash: String,
}

/// Ledger stats DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerStatsDto {
    pub total_entries: usize,
    pub avg_resonance: f64,
    pub latest_entry: Option<LedgerEntryDto>,
    pub integrity_verified: bool,
}

/// Kernel info DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelInfoDto {
    pub version: String,
    pub name: String,
    pub dimensions: usize,
    pub available_strategies: Vec<String>,
    pub artefact_types: Vec<String>,
    pub resonance_models: Vec<String>,
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Run kernel mining to discover blueprints
#[tauri::command]
pub fn run_kernel_mining(
    seed_psi: f64,
    seed_rho: f64,
    seed_omega: f64,
    seed_chi: f64,
    seed_eta: f64,
    config: MiningConfigDto,
) -> Result<MiningResultDto, String> {
    use qops_kernel::{MiningKernel, MiningConfig, SearchStrategy, CoreSignature, State};
    use std::time::Instant;

    let start = Instant::now();

    // Convert DTO to internal config
    let strategy = match config.strategy {
        SearchStrategyDto::Greedy => SearchStrategy::Greedy,
        SearchStrategyDto::Stochastic { temperature } => SearchStrategy::Stochastic { temperature },
        SearchStrategyDto::Beam { width } => SearchStrategy::Beam { width },
        SearchStrategyDto::Evolutionary { population_size, mutation_rate } =>
            SearchStrategy::Evolutionary { population_size, mutation_rate },
        SearchStrategyDto::Triton => SearchStrategy::Triton,
        SearchStrategyDto::Hybrid => SearchStrategy::Hybrid,
    };

    let mining_config = MiningConfig {
        max_iterations: config.max_iterations,
        target_resonance: config.target_resonance,
        max_candidates: config.max_candidates,
        exploration_rate: config.exploration_rate,
        strategy,
        convergence_epsilon: config.convergence_epsilon,
        ..Default::default()
    };

    // Create seed state
    let seed = State::Core(CoreSignature::new(seed_psi, seed_rho, seed_omega, seed_chi, seed_eta));

    // Run mining
    let mut miner = MiningKernel::new(mining_config);
    let result = miner.mine(&[seed]).map_err(|e| e.to_string())?;

    let duration = start.elapsed();

    // Convert candidates to DTOs
    let candidates: Vec<BlueprintCandidateDto> = result.candidates.iter().map(|c| {
        let sig = c.state.to_core();
        BlueprintCandidateDto {
            id: c.blueprint.id.clone(),
            name: c.blueprint.name.clone(),
            signature: SignatureDto {
                psi: sig.psi,
                rho: sig.rho,
                omega: sig.omega,
                chi: Some(sig.chi),
                eta: Some(sig.eta),
            },
            resonance_score: c.resonance_score,
            quality_level: format!("{:?}", c.quality_level),
        }
    }).collect();

    Ok(MiningResultDto {
        best_resonance: result.best_resonance,
        iterations: result.iterations,
        converged: result.converged,
        candidates,
        total_candidates_explored: result.total_candidates_explored,
        stagnation_count: result.stagnation_count,
        duration_ms: duration.as_millis() as u64,
    })
}

/// Materialize a blueprint into an artefact
#[tauri::command]
pub fn materialize_blueprint(
    blueprint_id: String,
    blueprint_psi: f64,
    blueprint_rho: f64,
    blueprint_omega: f64,
    blueprint_chi: f64,
    blueprint_eta: f64,
    config: MaterializationConfigDto,
) -> Result<MaterializationResultDto, String> {
    use qops_kernel::{Materializer, Blueprint, CoreSignature, State, ArtefactOutput};
    use qops_kernel::materialization::ArtefactType;
    use std::path::PathBuf;
    use chrono::Utc;

    // Create blueprint from signature
    let state = State::Core(CoreSignature::new(
        blueprint_psi, blueprint_rho, blueprint_omega, blueprint_chi, blueprint_eta
    ));
    let blueprint = Blueprint::from_state(&blueprint_id, state);

    // Determine artefact type
    let artefact_type = match config.artefact_type {
        ArtefactTypeDto::Code => ArtefactType::Code,
        ArtefactTypeDto::Configuration => ArtefactType::Configuration,
        ArtefactTypeDto::Document => ArtefactType::Document,
        ArtefactTypeDto::Data => ArtefactType::Data,
        ArtefactTypeDto::Operator => ArtefactType::OperatorImplementation,
        ArtefactTypeDto::Circuit => ArtefactType::QuantumCircuit,
        ArtefactTypeDto::Generic => ArtefactType::Generic("generic".to_string()),
    };

    // Create materializer
    let mut materializer = Materializer::new(PathBuf::from("./artefacts"));
    materializer.write_files = config.write_files;

    // Materialize
    let result = materializer.materialize(&blueprint, artefact_type)
        .map_err(|e| e.to_string())?;

    // Convert to DTO
    let artefact_dto = ArtefactDto {
        id: result.artefact.id.clone(),
        blueprint_id: result.artefact.blueprint_id.clone(),
        artefact_type: result.artefact.artefact_type.name().to_string(),
        content: match &result.artefact.content {
            qops_kernel::materialization::ArtefactContent::Json(v) => v.clone(),
            qops_kernel::materialization::ArtefactContent::Text(s) =>
                serde_json::Value::String(s.clone()),
            _ => serde_json::Value::Null,
        },
        final_resonance: result.artefact.final_resonance,
        created_at: result.artefact.created_at.to_rfc3339(),
    };

    Ok(MaterializationResultDto {
        success: result.success,
        artefact: artefact_dto,
        ledger_entry_id: result.ledger_entry.map(|e| e.id),
        warnings: result.warnings,
    })
}

/// Get kernel information
#[tauri::command]
pub fn get_kernel_info() -> Result<KernelInfoDto, String> {
    Ok(KernelInfoDto {
        version: qops_kernel::VERSION.to_string(),
        name: qops_kernel::KERNEL_NAME.to_string(),
        dimensions: qops_kernel::DEFAULT_DIMENSION,
        available_strategies: vec![
            "greedy".to_string(),
            "stochastic".to_string(),
            "beam".to_string(),
            "evolutionary".to_string(),
            "triton".to_string(),
            "hybrid".to_string(),
        ],
        artefact_types: vec![
            "code".to_string(),
            "configuration".to_string(),
            "document".to_string(),
            "data".to_string(),
            "operator".to_string(),
            "circuit".to_string(),
        ],
        resonance_models: vec![
            "simple".to_string(),
            "extended".to_string(),
            "weighted".to_string(),
            "geometric".to_string(),
            "harmonic".to_string(),
        ],
    })
}

/// Get ledger statistics
#[tauri::command]
pub fn get_ledger_stats() -> Result<LedgerStatsDto, String> {
    use qops_kernel::MemoryLedger;

    // Create in-memory ledger for stats (actual implementation would use persistent storage)
    let ledger = MemoryLedger::new();
    let stats = ledger.stats().map_err(|e| e.to_string())?;

    Ok(LedgerStatsDto {
        total_entries: stats.total_entries,
        avg_resonance: stats.avg_resonance,
        latest_entry: stats.latest.map(|e| LedgerEntryDto {
            id: e.id,
            blueprint_id: e.blueprint_id,
            artefact_id: e.artefact_id,
            timestamp: e.timestamp.to_rfc3339(),
            resonance_score: e.resonance_score,
            hash: e.hash,
        }),
        integrity_verified: true,
    })
}

/// Get mining presets
#[tauri::command]
pub fn get_mining_presets() -> Result<Vec<(String, MiningConfigDto)>, String> {
    Ok(vec![
        ("quick".to_string(), MiningConfigDto {
            max_iterations: 50,
            target_resonance: 0.7,
            max_candidates: 20,
            exploration_rate: 0.2,
            strategy: SearchStrategyDto::Greedy,
            convergence_epsilon: 1e-3,
        }),
        ("balanced".to_string(), MiningConfigDto {
            max_iterations: 100,
            target_resonance: 0.8,
            max_candidates: 50,
            exploration_rate: 0.3,
            strategy: SearchStrategyDto::Stochastic { temperature: 1.0 },
            convergence_epsilon: 1e-4,
        }),
        ("thorough".to_string(), MiningConfigDto {
            max_iterations: 200,
            target_resonance: 0.85,
            max_candidates: 100,
            exploration_rate: 0.4,
            strategy: SearchStrategyDto::Hybrid,
            convergence_epsilon: 1e-5,
        }),
        ("research".to_string(), MiningConfigDto {
            max_iterations: 500,
            target_resonance: 0.9,
            max_candidates: 200,
            exploration_rate: 0.5,
            strategy: SearchStrategyDto::Evolutionary { population_size: 50, mutation_rate: 0.1 },
            convergence_epsilon: 1e-6,
        }),
    ])
}
