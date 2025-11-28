//! System-related Tauri commands

use super::*;
use crate::error::Result;

/// Get system information
#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfoDto> {
    let modules = vec![
        "qops-core".to_string(),
        "qops-genesis".to_string(),
        "qops-quantum".to_string(),
        "qops-circuits".to_string(),
        "qops-algorithms".to_string(),
        "qops-research".to_string(),
        "qops-seraphic".to_string(),
        "qops-adapters".to_string(),
    ];

    let capabilities = vec![
        "Universal quantum gate set (H, X, Y, Z, CNOT, Toffoli)".to_string(),
        "State vector simulation (up to ~20 qubits)".to_string(),
        "Grover's search algorithm".to_string(),
        "Shor's factorization algorithm".to_string(),
        "Quantum Fourier Transform (QFT)".to_string(),
        "Quantum Phase Estimation (QPE)".to_string(),
        "Variational Quantum Eigensolver (VQE)".to_string(),
        "QAOA for combinatorial optimization".to_string(),
        "Hamiltonian simulation".to_string(),
        "Noise models".to_string(),
        "S7 topology operator mining".to_string(),
        "Cube-13 quantum algorithms".to_string(),
        "Seraphic calibration".to_string(),
    ];

    Ok(SystemInfoDto {
        version: env!("CARGO_PKG_VERSION").to_string(),
        modules,
        capabilities,
    })
}
