//! Calibration-related Tauri commands
//!
//! Provides Seraphic calibration with hyperparameter sweeps and auto-tuning.

use super::*;
use crate::error::{AppError, Result};
use crate::state::AppState;
use qops_seraphic::{SeraphicCalibrator, HyperparameterSweep, SweepConfig, AutoTuner};
use qops_core::{Configuration, Signature3D};
use tauri::State;

/// Sweep result DTO
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SweepResultDto {
    pub total_configurations: usize,
    pub best_config_index: usize,
    pub best_score: f64,
    pub evaluations: Vec<SweepEvaluationDto>,
}

/// Single sweep evaluation DTO
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SweepEvaluationDto {
    pub config_index: usize,
    pub parameters: std::collections::HashMap<String, f64>,
    pub final_score: f64,
    pub convergence_rate: f64,
}

/// Auto-tune result DTO
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AutoTuneResultDto {
    pub best_temperature: f64,
    pub best_cooling_rate: f64,
    pub best_mandorla_threshold: f64,
    pub achieved_resonance: f64,
    pub iterations: usize,
}

/// Run Seraphic calibration
#[tauri::command]
pub async fn run_calibration(
    state: State<'_, AppState>,
    steps: usize,
    _target: f64,
) -> Result<CalibrationResultDto> {
    if steps == 0 || steps > 1000 {
        return Err(AppError::InvalidParameter(
            "Steps must be between 1 and 1000".to_string(),
        ));
    }

    let mut calibrator = SeraphicCalibrator::default();
    calibrator.initialize(
        Configuration::new("gui_calibration"),
        Signature3D::new(0.5, 0.5, 0.5),
    );

    let results = calibrator.run(steps);

    let step_dtos: Vec<CalibrationStepDto> = results
        .iter()
        .map(|r| CalibrationStepDto {
            step: r.step,
            signature: SignatureDto {
                psi: r.performance.psi,
                rho: r.performance.rho,
                omega: r.performance.omega,
                chi: None,
                eta: None,
            },
            accepted: r.accepted,
            cri_triggered: r.cri_triggered,
        })
        .collect();

    let accepted_count = results.iter().filter(|r| r.accepted).count();

    // Get final performance before moving calibrator
    let final_perf = calibrator.current_performance();
    let final_signature = SignatureDto {
        psi: final_perf.psi,
        rho: final_perf.rho,
        omega: final_perf.omega,
        chi: None,
        eta: None,
    };

    // Store calibrator for later queries
    {
        let mut cal = state.calibrator.lock().unwrap();
        *cal = Some(calibrator);
    }

    Ok(CalibrationResultDto {
        steps: step_dtos,
        final_signature,
        accepted_count,
    })
}

/// Get current calibration status
#[tauri::command]
pub async fn get_calibration_status(
    state: State<'_, AppState>,
) -> Result<SignatureDto> {
    let cal_lock = state.calibrator.lock().unwrap();

    match cal_lock.as_ref() {
        Some(cal) => {
            let perf = cal.current_performance();
            Ok(SignatureDto {
                psi: perf.psi,
                rho: perf.rho,
                omega: perf.omega,
                chi: None,
                eta: None,
            })
        }
        None => Ok(SignatureDto {
            psi: 0.0,
            rho: 0.0,
            omega: 0.0,
            chi: None,
            eta: None,
        }),
    }
}

/// Run hyperparameter sweep
#[tauri::command]
pub async fn run_hyperparameter_sweep(
    temperature_range: Vec<f64>,
    cooling_rate_range: Vec<f64>,
    steps_per_config: usize,
) -> Result<SweepResultDto> {
    if temperature_range.is_empty() || cooling_rate_range.is_empty() {
        return Err(AppError::InvalidParameter(
            "At least one value required for each parameter range".to_string(),
        ));
    }

    let config = SweepConfig {
        temperature_values: temperature_range.clone(),
        cooling_rate_values: cooling_rate_range.clone(),
        steps_per_evaluation: steps_per_config,
        parallel: true,
    };

    let mut sweep = HyperparameterSweep::new(config);
    let result = sweep.run();

    let evaluations: Vec<SweepEvaluationDto> = result.evaluations
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let mut params = std::collections::HashMap::new();
            params.insert("temperature".to_string(), e.config.temperature);
            params.insert("cooling_rate".to_string(), e.config.cooling_rate);

            SweepEvaluationDto {
                config_index: i,
                parameters: params,
                final_score: e.final_score,
                convergence_rate: e.convergence_rate,
            }
        })
        .collect();

    Ok(SweepResultDto {
        total_configurations: result.evaluations.len(),
        best_config_index: result.best_config_index,
        best_score: result.best_score,
        evaluations,
    })
}

/// Run auto-tuning for calibration parameters
#[tauri::command]
pub async fn run_auto_tune(
    target_resonance: f64,
    max_iterations: usize,
) -> Result<AutoTuneResultDto> {
    if target_resonance <= 0.0 || target_resonance > 1.0 {
        return Err(AppError::InvalidParameter(
            "Target resonance must be between 0 and 1".to_string(),
        ));
    }

    let mut tuner = AutoTuner::new(target_resonance);
    let result = tuner.tune(max_iterations);

    Ok(AutoTuneResultDto {
        best_temperature: result.best_config.temperature,
        best_cooling_rate: result.best_config.cooling_rate,
        best_mandorla_threshold: result.best_config.mandorla_threshold,
        achieved_resonance: result.achieved_resonance,
        iterations: result.iterations,
    })
}
