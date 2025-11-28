//! QOPS GUI Library
//!
//! This module provides the Tauri application setup and command registration.

mod commands;
mod error;
mod state;

use state::AppState;
use tauri::Manager;

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            // Circuit commands
            commands::circuits::create_circuit,
            commands::circuits::add_gate,
            commands::circuits::remove_gate,
            commands::circuits::simulate_circuit,
            commands::circuits::get_circuit,
            commands::circuits::get_circuit_qasm,
            commands::circuits::delete_circuit,
            commands::circuits::list_circuits,
            // Algorithm commands
            commands::algorithms::run_grover,
            commands::algorithms::run_shor,
            commands::algorithms::run_qft,
            commands::algorithms::run_iqft,
            commands::algorithms::run_qpe,
            commands::algorithms::run_vqe,
            commands::algorithms::run_qaoa,
            // Genesis commands (TRITON-powered)
            commands::genesis::run_genesis_mining,
            commands::genesis::get_genesis_families,
            commands::genesis::get_s7_topology_info,
            commands::genesis::get_node_details,
            // Quantum commands (Cube-13 topology)
            commands::quantum::run_quantum_walk,
            commands::quantum::get_cube13_info,
            commands::quantum::get_cube13_metrics,
            commands::quantum::get_cube13_node,
            commands::quantum::run_cube13_walk,
            commands::quantum::embed_in_cube13,
            commands::quantum::run_cube13_vqe,
            // Research commands
            commands::research::run_experiment,
            commands::research::run_benchmark,
            // Calibration commands (with sweeps & auto-tuning)
            commands::calibration::run_calibration,
            commands::calibration::get_calibration_status,
            commands::calibration::run_hyperparameter_sweep,
            commands::calibration::run_auto_tune,
            // System commands
            commands::system::get_system_info,
        ])
        .setup(|app| {
            // Additional setup if needed
            tracing::info!("QOPS Desktop Application started");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
