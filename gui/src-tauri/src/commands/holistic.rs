//! Holistic Resonance Architecture Tauri commands
//!
//! Provides commands for the three-stage Genesis mining pipeline:
//! Kosmokrator -> Chronokrator -> Pfauenthron/Monolith

use super::*;
use crate::error::{AppError, Result};
use crate::state::AppState;
use tauri::State;

// ============================================================================
// DTOs for Holistic Resonance Architecture
// ============================================================================

/// Genesis stage enum for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GenesisStageDto {
    Discovery,
    Kosmokrator,
    Chronokrator,
    Pfauenthron,
    Finalized,
}

/// Kosmokrator configuration DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KosmokratorConfigDto {
    pub kappa_threshold: f64,
    pub stability_epsilon: f64,
    pub telescope_enabled: bool,
    pub history_window: usize,
}

impl Default for KosmokratorConfigDto {
    fn default() -> Self {
        Self {
            kappa_threshold: 0.7,
            stability_epsilon: 0.05,
            telescope_enabled: true,
            history_window: 50,
        }
    }
}

/// Kosmokrator result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KosmokratorResultDto {
    pub input_count: usize,
    pub passed_count: usize,
    pub avg_kappa: f64,
    pub max_kappa: f64,
    pub min_kappa: f64,
    pub telescope_adjustments: usize,
    pub candidates: Vec<CandidateDto>,
}

/// Proof-of-Resonance result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfResonanceDto {
    pub kappa: f64,
    pub coherence: f64,
    pub passed: bool,
    pub stability: f64,
}

/// Chronokrator configuration DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronokratorConfigDto {
    pub num_channels: usize,
    pub base_threshold: f64,
    pub exkalibration_enabled: bool,
    pub spike_detection: bool,
}

impl Default for ChronokratorConfigDto {
    fn default() -> Self {
        Self {
            num_channels: 4,
            base_threshold: 0.75,
            exkalibration_enabled: true,
            spike_detection: true,
        }
    }
}

/// Chronokrator result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronokratorResultDto {
    pub active_channels: usize,
    pub d_total: f64,
    pub current_threshold: f64,
    pub above_threshold: bool,
    pub exkalibration: ExkalibrationDto,
    pub spikes: Vec<SpikeDto>,
    pub channel_history: Vec<f64>,
}

/// Exkalibration vector DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExkalibrationDto {
    pub nabla_psi: f64,
    pub nabla_rho: f64,
    pub nabla_omega: f64,
    pub magnitude: f64,
}

/// Spike event DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikeDto {
    pub channel: usize,
    pub time: f64,
    pub intensity: f64,
}

/// Pfauenthron configuration DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfauenthronConfigDto {
    pub mandorla_threshold: f64,
    pub ophanim_count: usize,
    pub monolith_enabled: bool,
}

impl Default for PfauenthronConfigDto {
    fn default() -> Self {
        Self {
            mandorla_threshold: 0.8,
            ophanim_count: 4,
            monolith_enabled: true,
        }
    }
}

/// Pfauenthron result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfauenthronResultDto {
    pub ophanim: Vec<OphanimDto>,
    pub mandorla: MandorlaDto,
    pub monolith: Option<MonolithDto>,
}

/// Ophanim node DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OphanimDto {
    pub id: usize,
    pub resonance: f64,
    pub active: bool,
    pub position: (f64, f64, f64),
}

/// Mandorla field DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandorlaDto {
    pub p_gabriel: f64,
    pub i_oriphiel: f64,
    pub strength: f64,
    pub converged: bool,
}

/// Monolith structure DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonolithDto {
    pub coherence: f64,
    pub family_count: usize,
    pub families: Vec<FinalizedFamilyDto>,
    pub finalized: bool,
    pub creation_time: String,
}

/// Operator candidate DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateDto {
    pub id: usize,
    pub node_id: usize,
    pub resonance: f64,
    pub signature: SignatureDto,
    pub por_result: Option<ProofOfResonanceDto>,
    pub stage: GenesisStageDto,
}

/// Finalized operator family DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizedFamilyDto {
    pub name: String,
    pub member_count: usize,
    pub avg_resonance: f64,
    pub characteristics: FamilyCharacteristicsDto,
    pub finalization_time: String,
}

/// Family characteristics (re-export from genesis module)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyCharacteristicsDto {
    pub is_high_quality: bool,
    pub is_stable: bool,
    pub is_efficient: bool,
}

/// Holistic mining configuration DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolisticMiningConfigDto {
    pub kosmokrator: KosmokratorConfigDto,
    pub chronokrator: ChronokratorConfigDto,
    pub pfauenthron: PfauenthronConfigDto,
    pub num_agents: usize,
    pub steps_per_agent: usize,
    pub use_adaptive_triton: bool,
    pub preset: String,
}

impl Default for HolisticMiningConfigDto {
    fn default() -> Self {
        Self {
            kosmokrator: KosmokratorConfigDto::default(),
            chronokrator: ChronokratorConfigDto::default(),
            pfauenthron: PfauenthronConfigDto::default(),
            num_agents: 10,
            steps_per_agent: 50,
            use_adaptive_triton: true,
            preset: "thorough".to_string(),
        }
    }
}

/// Holistic mining result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolisticMiningResultDto {
    pub stage: GenesisStageDto,
    pub candidates_discovered: usize,
    pub candidates_after_kosmokrator: usize,
    pub candidates_after_chronokrator: usize,
    pub finalized_families: Vec<FinalizedFamilyDto>,
    pub best_resonance: f64,
    pub matrix_outputs: usize,
    pub monolith: Option<MonolithDto>,
    pub duration_ms: u64,
    pub stage_logs: Vec<StageLogDto>,
}

/// Stage log entry DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageLogDto {
    pub stage: GenesisStageDto,
    pub message: String,
    pub timestamp: String,
    pub metrics: Option<StageMetricsDto>,
}

/// Stage metrics DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageMetricsDto {
    pub input_count: usize,
    pub output_count: usize,
    pub avg_resonance: f64,
    pub duration_ms: u64,
}

/// TRITON adaptive result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TritonAdaptiveResultDto {
    pub best_score: f64,
    pub iterations: usize,
    pub layers_explored: usize,
    pub converged: bool,
    pub trajectory: Vec<TrajectoryPointDto>,
    pub holistic_output: Option<HolisticMatrixOutputDto>,
}

/// Trajectory point for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryPointDto {
    pub iteration: usize,
    pub score: f64,
    pub layer: usize,
    pub temperature: f64,
    pub radius: f64,
}

/// Holistic matrix output DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolisticMatrixOutputDto {
    pub outputs: usize,
    pub final_stage: GenesisStageDto,
    pub decision: String,
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Run holistic multi-stage mining pipeline
#[tauri::command]
pub async fn run_holistic_mining(
    _state: State<'_, AppState>,
    config: HolisticMiningConfigDto,
) -> Result<HolisticMiningResultDto> {
    use qops_genesis::{HolisticMiningConfig, HolisticMiningSession, MiningConfig, StageMetrics};
    use qops_core::{KosmokratorConfig, ChronokratorConfig, PfauenthronConfig, GenesisStage};

    // Build internal config from DTO
    let mining = MiningConfig {
        num_agents: config.num_agents,
        steps_per_agent: config.steps_per_agent,
        ..Default::default()
    };

    let mining_config = HolisticMiningConfig {
        mining,
        kosmokrator: KosmokratorConfig {
            kappa_threshold: config.kosmokrator.kappa_threshold,
            epsilon: config.kosmokrator.stability_epsilon,
            stability_window: config.kosmokrator.history_window,
            ..Default::default()
        },
        chronokrator: ChronokratorConfig {
            num_channels: config.chronokrator.num_channels,
            base_threshold: config.chronokrator.base_threshold,
            ..Default::default()
        },
        pfauenthron: PfauenthronConfig {
            convergence_epsilon: 0.01,
            num_ophanim: config.pfauenthron.ophanim_count,
            emit_monolith: config.pfauenthron.monolith_enabled,
            ..Default::default()
        },
        adaptive_triton: config.use_adaptive_triton,
        log_stages: true,
        ..Default::default()
    };

    let mut session = HolisticMiningSession::new(mining_config);

    // Run the full pipeline using the mine() method
    let result = session.mine();

    // Convert finalized families
    let families: Vec<FinalizedFamilyDto> = result.finalized_families
        .iter()
        .map(|f| FinalizedFamilyDto {
            name: f.name.clone(),
            member_count: f.member_count,
            avg_resonance: f.avg_resonance,
            characteristics: FamilyCharacteristicsDto {
                is_high_quality: f.is_high_quality,
                is_stable: f.is_stable,
                is_efficient: f.avg_resonance >= 0.6,
            },
            finalization_time: chrono::Utc::now().to_rfc3339(),
        })
        .collect();

    // Convert stage logs - extract message from metrics
    let stage_logs: Vec<StageLogDto> = result.stage_log
        .iter()
        .map(|log| {
            let message = match &log.metrics {
                StageMetrics::Discovery { nodes_visited, unique_nodes, max_resonance } => {
                    format!("Discovered {} nodes, {} unique, max resonance {:.4}",
                           nodes_visited, unique_nodes, max_resonance)
                }
                StageMetrics::Kosmokrator { kappa, por_passed, exclusion_rate } => {
                    format!("PoR kappa={:.4}, passed={}, exclusion={:.1}%",
                           kappa, por_passed, exclusion_rate * 100.0)
                }
                StageMetrics::Chronokrator { d_total, threshold, spike_count, exkalibration_magnitude } => {
                    format!("D_total={:.4}, threshold={:.4}, spikes={}, exkal={:.4}",
                           d_total, threshold, spike_count, exkalibration_magnitude)
                }
                StageMetrics::Pfauenthron { mandorla_score, is_converged, monolith_emitted, families_formed } => {
                    format!("Mandorla={:.4}, converged={}, monolith={}, families={}",
                           mandorla_score, is_converged, monolith_emitted, families_formed)
                }
            };

            StageLogDto {
                stage: match log.stage {
                    GenesisStage::Discovery => GenesisStageDto::Discovery,
                    GenesisStage::KosmokratorFilter => GenesisStageDto::Kosmokrator,
                    GenesisStage::ChronokratorExpansion => GenesisStageDto::Chronokrator,
                    GenesisStage::PfauenthronCollapse => GenesisStageDto::Pfauenthron,
                    GenesisStage::Finalized => GenesisStageDto::Finalized,
                },
                message,
                timestamp: chrono::Utc::now().to_rfc3339(),
                metrics: Some(StageMetricsDto {
                    input_count: log.candidates_in,
                    output_count: log.candidates_out,
                    avg_resonance: result.mining_stats.avg_resonance,
                    duration_ms: log.duration as u64,
                }),
            }
        })
        .collect();

    // Convert first monolith if present
    let monolith = result.monoliths.first().map(|m| MonolithDto {
        coherence: m.coherence,
        family_count: m.family_count,
        families: m.families.iter().map(|f| FinalizedFamilyDto {
            name: f.name.clone(),
            member_count: f.member_count,
            avg_resonance: f.avg_resonance,
            characteristics: FamilyCharacteristicsDto {
                is_high_quality: f.is_high_quality,
                is_stable: f.is_stable,
                is_efficient: f.avg_resonance >= 0.6,
            },
            finalization_time: chrono::Utc::now().to_rfc3339(),
        }).collect(),
        finalized: m.finalized,
        creation_time: chrono::Utc::now().to_rfc3339(),
    });

    Ok(HolisticMiningResultDto {
        stage: match result.final_stage {
            GenesisStage::Discovery => GenesisStageDto::Discovery,
            GenesisStage::KosmokratorFilter => GenesisStageDto::Kosmokrator,
            GenesisStage::ChronokratorExpansion => GenesisStageDto::Chronokrator,
            GenesisStage::PfauenthronCollapse => GenesisStageDto::Pfauenthron,
            GenesisStage::Finalized => GenesisStageDto::Finalized,
        },
        candidates_discovered: result.candidates_discovered,
        candidates_after_kosmokrator: result.candidates_after_kosmokrator,
        candidates_after_chronokrator: result.candidates_after_chronokrator,
        finalized_families: families,
        best_resonance: result.best_resonance,
        matrix_outputs: result.holistic_stats.total_outputs,
        monolith,
        duration_ms: result.duration_ms,
        stage_logs,
    })
}

/// Run only Kosmokrator filter stage
#[tauri::command]
pub async fn run_kosmokrator_stage(
    _state: State<'_, AppState>,
    config: KosmokratorConfigDto,
    candidates: Vec<CandidateDto>,
) -> Result<KosmokratorResultDto> {
    use qops_core::{KosmokratorConfig, KosmokratorState, OperatorCandidate, Signature5D};

    let internal_config = KosmokratorConfig {
        kappa_threshold: config.kappa_threshold,
        epsilon: config.stability_epsilon,
        stability_window: config.history_window,
        ..Default::default()
    };

    let mut state_kos = KosmokratorState::new(internal_config);

    // Convert DTO candidates to internal OperatorCandidates
    let internal_candidates: Vec<OperatorCandidate> = candidates.iter().enumerate().map(|(i, c)| {
        OperatorCandidate {
            id: format!("candidate_{}", i),
            signature: Signature5D::new(
                c.signature.psi,
                c.signature.rho,
                c.signature.omega,
                c.signature.chi.unwrap_or(0.5),
                c.signature.eta.unwrap_or(0.5),
            ),
            phase: c.resonance * std::f64::consts::PI,
            resonance: c.resonance,
            stability: 0.5,
            is_mandorla: c.resonance >= 0.85,
            node_index: c.node_id,
            discovered_at: 0.0,
        }
    }).collect();

    // Extract phases and compute PoR
    let phases: Vec<f64> = internal_candidates.iter().map(|c| c.phase).collect();
    let por_result = state_kos.compute_por(&phases, 0.0);

    // Filter using internal filter method
    let filtered = state_kos.filter(internal_candidates.clone(), 0.0);
    let stats = state_kos.stats();

    // Convert back to DTO
    let output_candidates: Vec<CandidateDto> = filtered.iter().enumerate().map(|(i, c)| {
        let original = &candidates[i.min(candidates.len() - 1)];
        CandidateDto {
            id: original.id,
            node_id: c.node_index,
            resonance: c.resonance,
            signature: SignatureDto {
                psi: c.signature.psi,
                rho: c.signature.rho,
                omega: c.signature.omega,
                chi: Some(c.signature.chi),
                eta: Some(c.signature.eta),
            },
            por_result: Some(ProofOfResonanceDto {
                kappa: por_result.kappa,
                coherence: por_result.stability,
                passed: por_result.passed,
                stability: por_result.stability,
            }),
            stage: GenesisStageDto::Kosmokrator,
        }
    }).collect();

    Ok(KosmokratorResultDto {
        input_count: candidates.len(),
        passed_count: output_candidates.len(),
        avg_kappa: stats.current_kappa,
        max_kappa: stats.current_kappa,
        min_kappa: stats.current_kappa,
        telescope_adjustments: 0,
        candidates: output_candidates,
    })
}

/// Run only Chronokrator expansion stage
#[tauri::command]
pub async fn run_chronokrator_stage(
    _state: State<'_, AppState>,
    config: ChronokratorConfigDto,
) -> Result<ChronokratorResultDto> {
    use qops_core::{ChronokratorConfig, ChronokratorState, OperatorCandidate, Signature5D};

    let internal_config = ChronokratorConfig {
        num_channels: config.num_channels,
        base_threshold: config.base_threshold,
        compute_exkalibration: config.exkalibration_enabled,
        ..Default::default()
    };

    let mut chrono_state = ChronokratorState::new(internal_config);

    // Create simulated candidates for channel initialization
    let candidates: Vec<OperatorCandidate> = (0..config.num_channels).map(|i| {
        let t = i as f64 * 0.1;
        OperatorCandidate {
            id: format!("channel_{}", i),
            signature: Signature5D::new(
                0.5 + 0.3 * t.sin(),
                0.5 + 0.3 * t.cos(),
                0.5,
                0.5,
                0.5,
            ),
            phase: t * std::f64::consts::PI,
            resonance: 0.5 + 0.3 * t.sin(),
            stability: 0.7,
            is_mandorla: false,
            node_index: i,
            discovered_at: t,
        }
    }).collect();

    chrono_state.init_channels(&candidates);

    // Run expansion simulation
    let mut spikes = Vec::new();
    for t in 0..100 {
        let time = t as f64 * 0.1;
        if let Some(exkal) = chrono_state.expand(&candidates, time) {
            spikes.push(SpikeDto {
                channel: 0,
                time,
                intensity: exkal.magnitude,
            });
        }
    }

    let stats = chrono_state.stats();
    let final_exkal = chrono_state.compute_exkalibration(&candidates, stats.current_t);

    Ok(ChronokratorResultDto {
        active_channels: stats.num_channels,
        d_total: stats.current_d_total,
        current_threshold: stats.current_threshold,
        above_threshold: stats.current_d_total > stats.current_threshold,
        exkalibration: ExkalibrationDto {
            nabla_psi: final_exkal.gradient[0],
            nabla_rho: final_exkal.gradient[1],
            nabla_omega: final_exkal.gradient[2],
            magnitude: final_exkal.magnitude,
        },
        spikes,
        channel_history: chrono_state.d_total_history.clone(),
    })
}

/// Run only Pfauenthron collapse stage
#[tauri::command]
pub async fn run_pfauenthron_stage(
    _state: State<'_, AppState>,
    config: PfauenthronConfigDto,
) -> Result<PfauenthronResultDto> {
    use qops_core::{PfauenthronConfig, PfauenthronState, OperatorCandidate, Signature5D, ExkalibrationVector};

    let internal_config = PfauenthronConfig {
        convergence_epsilon: 0.01,
        num_ophanim: config.ophanim_count,
        emit_monolith: config.monolith_enabled,
        ..Default::default()
    };

    let mut pfau_state = PfauenthronState::new(internal_config);

    // Create simulated candidates
    let candidates: Vec<OperatorCandidate> = (0..config.ophanim_count).map(|i| {
        let t = i as f64 * 0.2;
        OperatorCandidate {
            id: format!("ophanim_{}", i),
            signature: Signature5D::new(
                0.6 + 0.3 * t.sin(),
                0.6 + 0.3 * t.cos(),
                0.7,
                0.6,
                0.5,
            ),
            phase: t * std::f64::consts::PI,
            resonance: 0.7 + 0.2 * t.sin(),
            stability: 0.8,
            is_mandorla: true,
            node_index: i,
            discovered_at: t,
        }
    }).collect();

    pfau_state.init_ophanim(&candidates);

    // Create Exkalibration for Mandorla computation
    let exkal = ExkalibrationVector {
        gradient: [0.8, 0.7, 0.6, 0.5, 0.4],
        magnitude: 1.0,
        direction: [0.4, 0.35, 0.3, 0.25, 0.2],
        timestamp: 0.0,
        valid: true,
    };

    // Run convergence simulation
    for t in 0..50 {
        let time = t as f64 * 0.1;
        let _ = pfau_state.compute_mandorla(&candidates, &exkal, time);
    }

    // Attempt collapse
    let monolith = pfau_state.collapse(&candidates, &exkal, 5.0);
    let stats = pfau_state.stats();

    // Build ophanim DTOs from stats
    let ophanim_dtos: Vec<OphanimDto> = (0..config.ophanim_count).map(|i| {
        OphanimDto {
            id: i,
            resonance: 0.7 + 0.1 * (i as f64 * 0.3).sin(),
            active: true,
            position: (i as f64 * 0.2, i as f64 * 0.15, 0.5),
        }
    }).collect();

    let mandorla_dto = MandorlaDto {
        p_gabriel: stats.current_mandorla * 0.5 + 0.25,
        i_oriphiel: stats.current_mandorla * 0.5 + 0.25,
        strength: stats.current_mandorla,
        converged: stats.is_converged,
    };

    let monolith_dto = monolith.map(|m| MonolithDto {
        coherence: m.coherence,
        family_count: m.family_count,
        families: m.families.iter().map(|f| FinalizedFamilyDto {
            name: f.name.clone(),
            member_count: f.member_count,
            avg_resonance: f.avg_resonance,
            characteristics: FamilyCharacteristicsDto {
                is_high_quality: f.is_high_quality,
                is_stable: f.is_stable,
                is_efficient: f.avg_resonance >= 0.6,
            },
            finalization_time: chrono::Utc::now().to_rfc3339(),
        }).collect(),
        finalized: m.finalized,
        creation_time: chrono::Utc::now().to_rfc3339(),
    });

    Ok(PfauenthronResultDto {
        ophanim: ophanim_dtos,
        mandorla: mandorla_dto,
        monolith: monolith_dto,
    })
}

/// Run adaptive TRITON spiral search
#[tauri::command]
pub async fn run_adaptive_triton(
    _state: State<'_, AppState>,
    iterations: usize,
    with_holistic: bool,
) -> Result<TritonAdaptiveResultDto> {
    use qops_triton::{AdaptiveTritonConfig, AdaptiveTritonOptimizer, TritonConfig, SpiralParams};

    let base_config = TritonConfig {
        spiral: SpiralParams {
            expansion_rate: 1.618,
            initial_radius: 1.0,
            layers: 7,
            points_per_layer: 12,
            ..Default::default()
        },
        max_iterations: iterations,
        ..Default::default()
    };

    let adaptive_config = AdaptiveTritonConfig {
        base: base_config,
        holistic_integration: with_holistic,
        ..Default::default()
    };

    let mut optimizer = AdaptiveTritonOptimizer::new(adaptive_config);

    let result = optimizer.optimize();

    // Build trajectory from internal state - create simulated trajectory
    let trajectory: Vec<TrajectoryPointDto> = (0..result.iterations.min(100)).map(|i| {
        TrajectoryPointDto {
            iteration: i,
            score: result.best_score * (0.5 + 0.5 * (i as f64 / result.iterations as f64)),
            layer: i % 7,
            temperature: result.cooling_stats.temperature * (1.0 - i as f64 / result.iterations.max(1) as f64),
            radius: result.radius_stats.current_radius,
        }
    }).collect();

    let holistic_output = result.holistic_output.map(|h| HolisticMatrixOutputDto {
        outputs: h.valid_outputs,
        final_stage: match h.current_stage.as_str() {
            "discovery" => GenesisStageDto::Discovery,
            "kosmokrator" => GenesisStageDto::Kosmokrator,
            "chronokrator" => GenesisStageDto::Chronokrator,
            "pfauenthron" => GenesisStageDto::Pfauenthron,
            "finalized" => GenesisStageDto::Finalized,
            _ => GenesisStageDto::Discovery,
        },
        decision: format!("Families: {}, Monoliths: {}", h.family_count, h.monolith_count),
    });

    Ok(TritonAdaptiveResultDto {
        best_score: result.best_score,
        iterations: result.iterations,
        layers_explored: result.layer_memory.layers_visited(),
        converged: result.converged,
        trajectory,
        holistic_output,
    })
}

/// Get preset configurations for holistic mining
#[tauri::command]
pub async fn get_holistic_presets() -> Result<Vec<(String, HolisticMiningConfigDto)>> {
    Ok(vec![
        ("quick".to_string(), HolisticMiningConfigDto {
            kosmokrator: KosmokratorConfigDto {
                kappa_threshold: 0.6,
                ..Default::default()
            },
            chronokrator: ChronokratorConfigDto {
                num_channels: 2,
                ..Default::default()
            },
            pfauenthron: PfauenthronConfigDto {
                mandorla_threshold: 0.7,
                ..Default::default()
            },
            num_agents: 5,
            steps_per_agent: 20,
            use_adaptive_triton: false,
            preset: "quick".to_string(),
        }),
        ("thorough".to_string(), HolisticMiningConfigDto::default()),
        ("research".to_string(), HolisticMiningConfigDto {
            kosmokrator: KosmokratorConfigDto {
                kappa_threshold: 0.8,
                history_window: 100,
                ..Default::default()
            },
            chronokrator: ChronokratorConfigDto {
                num_channels: 6,
                ..Default::default()
            },
            pfauenthron: PfauenthronConfigDto {
                mandorla_threshold: 0.9,
                ophanim_count: 6,
                ..Default::default()
            },
            num_agents: 20,
            steps_per_agent: 100,
            use_adaptive_triton: true,
            preset: "research".to_string(),
        }),
    ])
}

/// Export holistic mining results to various formats
#[tauri::command]
pub async fn export_holistic_results(
    result: HolisticMiningResultDto,
    format: String,
    path: Option<String>,
) -> Result<String> {
    let output = match format.as_str() {
        "json" => {
            serde_json::to_string_pretty(&result)
                .map_err(|e| AppError::Serialization(e.to_string()))?
        }
        "csv" => {
            let mut csv = String::new();
            csv.push_str("family_name,member_count,avg_resonance,is_high_quality,is_stable,is_efficient\n");
            for family in &result.finalized_families {
                csv.push_str(&format!(
                    "{},{},{:.4},{},{},{}\n",
                    family.name,
                    family.member_count,
                    family.avg_resonance,
                    family.characteristics.is_high_quality,
                    family.characteristics.is_stable,
                    family.characteristics.is_efficient,
                ));
            }
            csv
        }
        "md" | "markdown" => {
            let mut md = String::new();
            md.push_str("# Genesis Holistic Mining Report\n\n");
            md.push_str(&format!("**Duration**: {} ms\n\n", result.duration_ms));
            md.push_str(&format!("**Best Resonance**: {:.4}\n\n", result.best_resonance));
            md.push_str(&format!("**Matrix Outputs**: {}\n\n", result.matrix_outputs));

            md.push_str("## Pipeline Summary\n\n");
            md.push_str(&format!("- Discovered: {} candidates\n", result.candidates_discovered));
            md.push_str(&format!("- After Kosmokrator: {} candidates\n", result.candidates_after_kosmokrator));
            md.push_str(&format!("- After Chronokrator: {} candidates\n", result.candidates_after_chronokrator));
            md.push_str(&format!("- Finalized Families: {}\n\n", result.finalized_families.len()));

            md.push_str("## Finalized Operator Families\n\n");
            md.push_str("| Name | Members | Avg Resonance | Quality | Stable | Efficient |\n");
            md.push_str("|------|---------|---------------|---------|--------|----------|\n");
            for family in &result.finalized_families {
                md.push_str(&format!(
                    "| {} | {} | {:.4} | {} | {} | {} |\n",
                    family.name,
                    family.member_count,
                    family.avg_resonance,
                    if family.characteristics.is_high_quality { "Yes" } else { "No" },
                    if family.characteristics.is_stable { "Yes" } else { "No" },
                    if family.characteristics.is_efficient { "Yes" } else { "No" },
                ));
            }

            if let Some(mono) = &result.monolith {
                md.push_str("\n## Monolith Structure\n\n");
                md.push_str(&format!("- Coherence: {:.4}\n", mono.coherence));
                md.push_str(&format!("- Families: {}\n", mono.family_count));
                md.push_str(&format!("- Finalized: {}\n", mono.finalized));
            }

            md
        }
        _ => {
            return Err(AppError::InvalidParameter(
                format!("Unsupported format: {}. Use json, csv, or md.", format)
            ));
        }
    };

    // If path provided, write to file
    if let Some(file_path) = path {
        std::fs::write(&file_path, &output)
            .map_err(|e| AppError::Io(e.to_string()))?;
        Ok(format!("Exported to: {}", file_path))
    } else {
        Ok(output)
    }
}
