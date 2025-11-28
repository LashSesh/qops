//! Calibration-related Tauri commands

use super::*;
use crate::error::{AppError, Result};
use crate::state::AppState;
use qops_seraphic::SeraphicCalibrator;
use qops_core::{Configuration, Signature3D};
use tauri::State;

/// Run Seraphic calibration
#[tauri::command]
pub async fn run_calibration(
    state: State<'_, AppState>,
    steps: usize,
    target: f64,
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

    let final_perf = calibrator.current_performance();

    // Store calibrator for later queries
    {
        let mut cal = state.calibrator.lock().unwrap();
        *cal = Some(calibrator);
    }

    Ok(CalibrationResultDto {
        steps: step_dtos,
        final_signature: SignatureDto {
            psi: final_perf.psi,
            rho: final_perf.rho,
            omega: final_perf.omega,
            chi: None,
            eta: None,
        },
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
