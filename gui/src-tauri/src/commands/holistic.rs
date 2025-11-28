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
    state: State<'_, AppState>,
    config: HolisticMiningConfigDto,
) -> Result<HolisticMiningResultDto> {
    use qops_genesis::{HolisticMiningConfig, HolisticMiningSession};
    use qops_core::{HolisticConfig, KosmokratorConfig, ChronokratorConfig, PfauenthronConfig};
    use std::time::Instant;

    let start = Instant::now();

    // Build internal config from DTO
    let holistic_config = HolisticConfig {
        kosmokrator: KosmokratorConfig {
            kappa_threshold: config.kosmokrator.kappa_threshold,
            stability_epsilon: config.kosmokrator.stability_epsilon,
            telescope_enabled: config.kosmokrator.telescope_enabled,
            history_window: config.kosmokrator.history_window,
            ..Default::default()
        },
        chronokrator: ChronokratorConfig {
            num_channels: config.chronokrator.num_channels,
            base_threshold: config.chronokrator.base_threshold,
            exkalibration_enabled: config.chronokrator.exkalibration_enabled,
            spike_detection: config.chronokrator.spike_detection,
            ..Default::default()
        },
        pfauenthron: PfauenthronConfig {
            mandorla_threshold: config.pfauenthron.mandorla_threshold,
            ophanim_count: config.pfauenthron.ophanim_count,
            monolith_enabled: config.pfauenthron.monolith_enabled,
            ..Default::default()
        },
        ..Default::default()
    };

    let mining_config = HolisticMiningConfig {
        holistic: holistic_config,
        num_agents: config.num_agents,
        steps_per_agent: config.steps_per_agent,
        use_adaptive_triton: config.use_adaptive_triton,
        export_stage_logs: true,
        ..Default::default()
    };

    let mut session = HolisticMiningSession::new(mining_config);

    // Run all stages
    session.run_discovery();
    let discovered = session.candidates().len();

    session.run_kosmokrator();
    let after_kos = session.candidates().len();

    session.run_chronokrator();
    let after_chrono = session.candidates().len();

    session.run_pfauenthron();
    let result = session.finalize();

    let duration = start.elapsed();

    // Convert finalized families
    let families: Vec<FinalizedFamilyDto> = result.finalized_families
        .iter()
        .map(|f| FinalizedFamilyDto {
            name: f.name.clone(),
            member_count: f.member_count,
            avg_resonance: f.avg_resonance,
            characteristics: FamilyCharacteristicsDto {
                is_high_quality: f.characteristics.is_high_quality,
                is_stable: f.characteristics.is_stable,
                is_efficient: f.characteristics.is_efficient,
            },
            finalization_time: chrono::Utc::now().to_rfc3339(),
        })
        .collect();

    // Convert stage logs
    let stage_logs: Vec<StageLogDto> = result.stage_logs
        .iter()
        .map(|log| StageLogDto {
            stage: match log.stage {
                qops_genesis::GenesisStage::Discovery => GenesisStageDto::Discovery,
                qops_genesis::GenesisStage::KosmokratorFilter => GenesisStageDto::Kosmokrator,
                qops_genesis::GenesisStage::ChronokratorExpansion => GenesisStageDto::Chronokrator,
                qops_genesis::GenesisStage::PfauenthronCollapse => GenesisStageDto::Pfauenthron,
                qops_genesis::GenesisStage::Finalized => GenesisStageDto::Finalized,
            },
            message: log.message.clone(),
            timestamp: log.timestamp.to_rfc3339(),
            metrics: log.metrics.as_ref().map(|m| StageMetricsDto {
                input_count: m.input_count,
                output_count: m.output_count,
                avg_resonance: m.avg_resonance,
                duration_ms: m.duration_ms,
            }),
        })
        .collect();

    // Convert monolith if present
    let monolith = result.monolith.as_ref().map(|m| MonolithDto {
        coherence: m.coherence,
        family_count: m.family_count,
        families: m.families.iter().map(|f| FinalizedFamilyDto {
            name: f.name.clone(),
            member_count: f.member_count,
            avg_resonance: f.avg_resonance,
            characteristics: FamilyCharacteristicsDto {
                is_high_quality: f.characteristics.is_high_quality,
                is_stable: f.characteristics.is_stable,
                is_efficient: f.characteristics.is_efficient,
            },
            finalization_time: chrono::Utc::now().to_rfc3339(),
        }).collect(),
        finalized: m.finalized,
        creation_time: chrono::Utc::now().to_rfc3339(),
    });

    Ok(HolisticMiningResultDto {
        stage: GenesisStageDto::Finalized,
        candidates_discovered: discovered,
        candidates_after_kosmokrator: after_kos,
        candidates_after_chronokrator: after_chrono,
        finalized_families: families,
        best_resonance: result.best_resonance,
        matrix_outputs: result.matrix_outputs,
        monolith,
        duration_ms: duration.as_millis() as u64,
        stage_logs,
    })
}

/// Run only Kosmokrator filter stage
#[tauri::command]
pub async fn run_kosmokrator_stage(
    state: State<'_, AppState>,
    config: KosmokratorConfigDto,
    candidates: Vec<CandidateDto>,
) -> Result<KosmokratorResultDto> {
    use qops_core::{KosmokratorConfig, KosmokratorState};

    let internal_config = KosmokratorConfig {
        kappa_threshold: config.kappa_threshold,
        stability_epsilon: config.stability_epsilon,
        telescope_enabled: config.telescope_enabled,
        history_window: config.history_window,
        ..Default::default()
    };

    let mut state_kos = KosmokratorState::new(internal_config);

    // Process candidates
    let mut passed_candidates = Vec::new();
    let mut kappa_values = Vec::new();

    for candidate in &candidates {
        // Simulate phase from resonance
        let phase = (candidate.resonance * std::f64::consts::PI).sin();
        state_kos.add_phase(phase);
    }

    let por_result = state_kos.compute_por();

    // Calculate statistics
    let avg_kappa = por_result.kappa;
    let max_kappa = kappa_values.iter().cloned().fold(0.0_f64, f64::max);
    let min_kappa = kappa_values.iter().cloned().fold(1.0_f64, f64::min);

    // Filter candidates based on PoR
    let passed_count = candidates.iter()
        .filter(|c| c.resonance >= config.kappa_threshold)
        .count();

    let output_candidates: Vec<CandidateDto> = candidates.into_iter()
        .filter(|c| c.resonance >= config.kappa_threshold)
        .map(|mut c| {
            c.por_result = Some(ProofOfResonanceDto {
                kappa: por_result.kappa,
                coherence: por_result.coherence,
                passed: por_result.passed,
                stability: por_result.stability,
            });
            c.stage = GenesisStageDto::Kosmokrator;
            c
        })
        .collect();

    Ok(KosmokratorResultDto {
        input_count: output_candidates.len() + (passed_count.saturating_sub(output_candidates.len())),
        passed_count,
        avg_kappa,
        max_kappa,
        min_kappa,
        telescope_adjustments: state_kos.telescope_adjustments(),
        candidates: output_candidates,
    })
}

/// Run only Chronokrator expansion stage
#[tauri::command]
pub async fn run_chronokrator_stage(
    state: State<'_, AppState>,
    config: ChronokratorConfigDto,
) -> Result<ChronokratorResultDto> {
    use qops_core::{ChronokratorConfig, ChronokratorState};

    let internal_config = ChronokratorConfig {
        num_channels: config.num_channels,
        base_threshold: config.base_threshold,
        exkalibration_enabled: config.exkalibration_enabled,
        spike_detection: config.spike_detection,
        ..Default::default()
    };

    let mut chrono_state = ChronokratorState::new(internal_config);

    // Simulate dynamics
    let time_steps = 100;
    for t in 0..time_steps {
        let time = t as f64 * 0.1;
        for ch in 0..config.num_channels {
            let phase_offset = ch as f64 * std::f64::consts::PI / (config.num_channels as f64);
            let resonance = 0.5 + 0.3 * (time + phase_offset).sin();
            chrono_state.update_channel(ch, resonance, time);
        }
    }

    let d_total = chrono_state.compute_d_total();
    let exkal = chrono_state.compute_exkalibration();
    let spikes = chrono_state.detect_spikes();
    let threshold = chrono_state.current_threshold();

    let spike_dtos: Vec<SpikeDto> = spikes.iter()
        .map(|s| SpikeDto {
            channel: s.channel,
            time: s.time,
            intensity: s.intensity,
        })
        .collect();

    Ok(ChronokratorResultDto {
        active_channels: config.num_channels,
        d_total,
        current_threshold: threshold,
        above_threshold: d_total > threshold,
        exkalibration: ExkalibrationDto {
            nabla_psi: exkal.nabla_psi,
            nabla_rho: exkal.nabla_rho,
            nabla_omega: exkal.nabla_omega,
            magnitude: exkal.magnitude(),
        },
        spikes: spike_dtos,
        channel_history: chrono_state.channel_history(),
    })
}

/// Run only Pfauenthron collapse stage
#[tauri::command]
pub async fn run_pfauenthron_stage(
    state: State<'_, AppState>,
    config: PfauenthronConfigDto,
) -> Result<PfauenthronResultDto> {
    use qops_core::{PfauenthronConfig, PfauenthronState};

    let internal_config = PfauenthronConfig {
        mandorla_threshold: config.mandorla_threshold,
        ophanim_count: config.ophanim_count,
        monolith_enabled: config.monolith_enabled,
        ..Default::default()
    };

    let mut pfau_state = PfauenthronState::new(internal_config);
    pfau_state.initialize_ophanim();

    // Simulate Gabriel-Oriphiel convergence
    for step in 0..50 {
        let p_gabriel = 0.5 + 0.4 * (step as f64 * 0.1).sin();
        let i_oriphiel = 0.5 + 0.4 * (step as f64 * 0.1).cos();
        pfau_state.update_convergence(p_gabriel, i_oriphiel);
    }

    let mandorla = pfau_state.compute_mandorla();
    let monolith = pfau_state.attempt_monolith_formation();

    let ophanim_dtos: Vec<OphanimDto> = pfau_state.ophanim()
        .iter()
        .enumerate()
        .map(|(i, o)| OphanimDto {
            id: i,
            resonance: o.resonance,
            active: o.active,
            position: o.position,
        })
        .collect();

    let mandorla_dto = MandorlaDto {
        p_gabriel: mandorla.p_gabriel,
        i_oriphiel: mandorla.i_oriphiel,
        strength: mandorla.strength,
        converged: mandorla.strength >= config.mandorla_threshold,
    };

    let monolith_dto = monolith.map(|m| MonolithDto {
        coherence: m.coherence,
        family_count: m.family_count,
        families: vec![],
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
    state: State<'_, AppState>,
    iterations: usize,
    with_holistic: bool,
) -> Result<TritonAdaptiveResultDto> {
    use qops_triton::{AdaptiveTritonConfig, AdaptiveTritonOptimizer, TritonConfig, SpiralParams};

    let base_config = TritonConfig {
        spiral: SpiralParams {
            expansion_rate: 1.618,
            initial_radius: 1.0,
            max_layers: 7,
        },
        max_iterations: iterations,
        ..Default::default()
    };

    let adaptive_config = AdaptiveTritonConfig {
        base: base_config,
        ..Default::default()
    };

    let mut optimizer = AdaptiveTritonOptimizer::new(adaptive_config);

    if with_holistic {
        optimizer.enable_holistic_integration();
    }

    let result = optimizer.optimize(|sig| {
        qops_core::resonance_5d(sig)
    });

    let trajectory: Vec<TrajectoryPointDto> = result.trajectory
        .iter()
        .map(|t| TrajectoryPointDto {
            iteration: t.iteration,
            score: t.score,
            layer: t.layer,
            temperature: t.temperature,
            radius: t.radius,
        })
        .collect();

    let holistic_output = result.holistic_output.as_ref().map(|h| HolisticMatrixOutputDto {
        outputs: h.outputs,
        final_stage: match h.final_stage {
            qops_core::GenesisStage::Discovery => GenesisStageDto::Discovery,
            qops_core::GenesisStage::KosmokratorFilter => GenesisStageDto::Kosmokrator,
            qops_core::GenesisStage::ChronokratorExpansion => GenesisStageDto::Chronokrator,
            qops_core::GenesisStage::PfauenthronCollapse => GenesisStageDto::Pfauenthron,
            qops_core::GenesisStage::Finalized => GenesisStageDto::Finalized,
        },
        decision: h.decision.clone(),
    });

    Ok(TritonAdaptiveResultDto {
        best_score: result.best_score,
        iterations: result.iterations,
        layers_explored: result.layers_explored,
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
