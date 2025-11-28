//! Circuit-related Tauri commands

use super::*;
use crate::error::{AppError, Result};
use crate::state::AppState;
use qops_circuits::{Circuit, Gate, QuantumRegister, Measurement};
use tauri::State;

/// Create a new quantum circuit
#[tauri::command]
pub async fn create_circuit(
    state: State<'_, AppState>,
    qubits: usize,
    name: Option<String>,
) -> Result<CircuitDto> {
    if qubits == 0 || qubits > 20 {
        return Err(AppError::InvalidParameter(
            "Qubit count must be between 1 and 20".to_string(),
        ));
    }

    let id = AppState::new_id();
    let circuit_name = name.unwrap_or_else(|| format!("circuit_{}", &id[..8]));
    let circuit = Circuit::with_name(qubits, &circuit_name);

    let dto = CircuitDto {
        id: id.clone(),
        name: circuit_name,
        qubits: circuit.num_qubits,
        depth: circuit.depth(),
        gate_count: circuit.gate_count(),
        gates: vec![],
    };

    state.circuits.lock().unwrap().insert(id, circuit);

    Ok(dto)
}

/// Add a gate to a circuit
#[tauri::command]
pub async fn add_gate(
    state: State<'_, AppState>,
    circuit_id: String,
    gate_type: String,
    qubits: Vec<usize>,
    parameter: Option<f64>,
) -> Result<CircuitDto> {
    let mut circuits = state.circuits.lock().unwrap();
    let circuit = circuits
        .get_mut(&circuit_id)
        .ok_or_else(|| AppError::NotFound(format!("Circuit {}", circuit_id)))?;

    let gate = match gate_type.to_lowercase().as_str() {
        "h" | "hadamard" => Gate::h(),
        "x" | "paulix" => Gate::x(),
        "y" | "pauliy" => Gate::y(),
        "z" | "pauliz" => Gate::z(),
        "s" | "phase" => Gate::s(),
        "sdg" => Gate::sdg(),
        "t" => Gate::t(),
        "tdg" => Gate::tdg(),
        "rx" => Gate::rx(parameter.unwrap_or(std::f64::consts::PI / 2.0)),
        "ry" => Gate::ry(parameter.unwrap_or(std::f64::consts::PI / 2.0)),
        "rz" => Gate::rz(parameter.unwrap_or(std::f64::consts::PI / 2.0)),
        "cnot" | "cx" => Gate::cnot(),
        "cz" => Gate::cz(),
        "cy" => Gate::cy(),
        "swap" => Gate::swap(),
        "iswap" => Gate::iswap(),
        "toffoli" | "ccx" => Gate::toffoli(),
        "fredkin" | "cswap" => Gate::fredkin(),
        _ => return Err(AppError::InvalidParameter(format!("Unknown gate type: {}", gate_type))),
    };

    circuit.add_gate(gate.clone(), qubits.clone())?;

    let gates = circuit
        .instructions
        .iter()
        .map(|inst| GateDto {
            name: inst.gate.name.clone(),
            gate_type: format!("{:?}", inst.gate.gate_type),
            qubits: inst.qubits.clone(),
            parameter: inst.gate.parameter,
        })
        .collect();

    Ok(CircuitDto {
        id: circuit_id,
        name: circuit.name.clone(),
        qubits: circuit.num_qubits,
        depth: circuit.depth(),
        gate_count: circuit.gate_count(),
        gates,
    })
}

/// Remove a gate from a circuit
#[tauri::command]
pub async fn remove_gate(
    state: State<'_, AppState>,
    circuit_id: String,
    gate_index: usize,
) -> Result<CircuitDto> {
    let mut circuits = state.circuits.lock().unwrap();
    let circuit = circuits
        .get_mut(&circuit_id)
        .ok_or_else(|| AppError::NotFound(format!("Circuit {}", circuit_id)))?;

    if gate_index >= circuit.instructions.len() {
        return Err(AppError::InvalidParameter(format!(
            "Gate index {} out of range",
            gate_index
        )));
    }

    circuit.instructions.remove(gate_index);

    let gates = circuit
        .instructions
        .iter()
        .map(|inst| GateDto {
            name: inst.gate.name.clone(),
            gate_type: format!("{:?}", inst.gate.gate_type),
            qubits: inst.qubits.clone(),
            parameter: inst.gate.parameter,
        })
        .collect();

    Ok(CircuitDto {
        id: circuit_id,
        name: circuit.name.clone(),
        qubits: circuit.num_qubits,
        depth: circuit.depth(),
        gate_count: circuit.gate_count(),
        gates,
    })
}

/// Simulate a circuit
#[tauri::command]
pub async fn simulate_circuit(
    state: State<'_, AppState>,
    circuit_id: String,
    shots: usize,
) -> Result<SimulationResultDto> {
    let circuits = state.circuits.lock().unwrap();
    let circuit = circuits
        .get(&circuit_id)
        .ok_or_else(|| AppError::NotFound(format!("Circuit {}", circuit_id)))?;

    let mut register = QuantumRegister::new(circuit.num_qubits);
    register.apply_circuit(circuit)?;

    let probabilities = register.state.probabilities();
    let stats = Measurement::measure_all(&register, shots);

    Ok(SimulationResultDto {
        probabilities,
        counts: stats.counts,
        shots,
    })
}

/// Get circuit details
#[tauri::command]
pub async fn get_circuit(
    state: State<'_, AppState>,
    circuit_id: String,
) -> Result<CircuitDto> {
    let circuits = state.circuits.lock().unwrap();
    let circuit = circuits
        .get(&circuit_id)
        .ok_or_else(|| AppError::NotFound(format!("Circuit {}", circuit_id)))?;

    let gates = circuit
        .instructions
        .iter()
        .map(|inst| GateDto {
            name: inst.gate.name.clone(),
            gate_type: format!("{:?}", inst.gate.gate_type),
            qubits: inst.qubits.clone(),
            parameter: inst.gate.parameter,
        })
        .collect();

    Ok(CircuitDto {
        id: circuit_id,
        name: circuit.name.clone(),
        qubits: circuit.num_qubits,
        depth: circuit.depth(),
        gate_count: circuit.gate_count(),
        gates,
    })
}

/// Export circuit as QASM
#[tauri::command]
pub async fn get_circuit_qasm(
    state: State<'_, AppState>,
    circuit_id: String,
) -> Result<String> {
    let circuits = state.circuits.lock().unwrap();
    let circuit = circuits
        .get(&circuit_id)
        .ok_or_else(|| AppError::NotFound(format!("Circuit {}", circuit_id)))?;

    Ok(circuit.to_qasm())
}

/// Delete a circuit
#[tauri::command]
pub async fn delete_circuit(
    state: State<'_, AppState>,
    circuit_id: String,
) -> Result<bool> {
    let mut circuits = state.circuits.lock().unwrap();
    Ok(circuits.remove(&circuit_id).is_some())
}

/// List all circuits
#[tauri::command]
pub async fn list_circuits(state: State<'_, AppState>) -> Result<Vec<CircuitDto>> {
    let circuits = state.circuits.lock().unwrap();

    Ok(circuits
        .iter()
        .map(|(id, circuit)| {
            let gates = circuit
                .instructions
                .iter()
                .map(|inst| GateDto {
                    name: inst.gate.name.clone(),
                    gate_type: format!("{:?}", inst.gate.gate_type),
                    qubits: inst.qubits.clone(),
                    parameter: inst.gate.parameter,
                })
                .collect();

            CircuitDto {
                id: id.clone(),
                name: circuit.name.clone(),
                qubits: circuit.num_qubits,
                depth: circuit.depth(),
                gate_count: circuit.gate_count(),
                gates,
            }
        })
        .collect())
}
