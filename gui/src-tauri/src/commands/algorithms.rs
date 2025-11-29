//! Algorithm-related Tauri commands

use super::*;
use crate::error::{AppError, Result};
use qops_algorithms::{
    Grover, Oracle, Shor, FactorizationMethod,
    QuantumFourierTransform, QuantumPhaseEstimation,
    VQE, VQEConfig, vqe::PauliSum, Ansatz,
    QAOA,
};
use qops_circuits::Gate;

/// Run Grover's search algorithm
#[tauri::command]
pub async fn run_grover(
    qubits: usize,
    target: usize,
    shots: usize,
) -> Result<GroverResultDto> {
    if qubits < 1 || qubits > 15 {
        return Err(AppError::InvalidParameter(
            "Qubit count must be between 1 and 15".to_string(),
        ));
    }

    if target >= (1 << qubits) {
        return Err(AppError::InvalidParameter(
            format!("Target {} out of range for {} qubits", target, qubits),
        ));
    }

    let oracle = Oracle::marked_state(qubits, target);
    let grover = Grover::new(qubits, oracle);

    let theoretical = grover.theoretical_success_probability();
    let result = grover.run_with_shots(shots);

    Ok(GroverResultDto {
        measured_state: result.measured_state,
        measured_state_binary: format!("{:0width$b}", result.measured_state, width = qubits),
        success_probability: result.success_probability,
        iterations: result.iterations,
        is_solution: result.is_solution,
        counts: result.counts,
        theoretical_probability: theoretical,
    })
}

/// Run Shor's factorization algorithm
#[tauri::command]
pub async fn run_shor(number: u64) -> Result<ShorResultDto> {
    if number < 4 {
        return Err(AppError::InvalidParameter(
            "Number must be >= 4".to_string(),
        ));
    }

    let shor = Shor::new(number)
        .with_method(FactorizationMethod::Simulated)
        .with_max_attempts(20);

    let result = shor.run();

    Ok(ShorResultDto {
        success: result.success,
        number,
        factors: result.factors,
        period: result.period,
        attempts: result.attempts,
    })
}

/// Run Quantum Fourier Transform
#[tauri::command]
pub async fn run_qft(qubits: usize, input_state: Option<usize>) -> Result<QftResultDto> {
    if qubits < 1 || qubits > 15 {
        return Err(AppError::InvalidParameter(
            "Qubit count must be between 1 and 15".to_string(),
        ));
    }

    use qops_circuits::QuantumRegister;

    let qft = QuantumFourierTransform::new(qubits);
    let circuit = qft.build_circuit();

    let mut reg = QuantumRegister::new(qubits);

    // Initialize to input state if provided
    if let Some(state) = input_state {
        for i in 0..qubits {
            if (state >> i) & 1 == 1 {
                reg.apply_single_gate(&Gate::x(), i).ok();
            }
        }
    }

    qft.apply(&mut reg).map_err(|e| AppError::Algorithm(e.to_string()))?;

    Ok(QftResultDto {
        qubits,
        depth: circuit.depth(),
        gate_count: circuit.gate_count(),
        output_probabilities: reg.state.probabilities(),
    })
}

/// Run inverse QFT
#[tauri::command]
pub async fn run_iqft(qubits: usize, input_state: Option<usize>) -> Result<QftResultDto> {
    if qubits < 1 || qubits > 15 {
        return Err(AppError::InvalidParameter(
            "Qubit count must be between 1 and 15".to_string(),
        ));
    }

    use qops_circuits::{Circuit, QuantumRegister};

    let circuit = Circuit::iqft(qubits);

    let mut reg = QuantumRegister::new(qubits);

    if let Some(state) = input_state {
        for i in 0..qubits {
            if (state >> i) & 1 == 1 {
                reg.apply_single_gate(&Gate::x(), i).ok();
            }
        }
    }

    reg.apply_circuit(&circuit)?;

    Ok(QftResultDto {
        qubits,
        depth: circuit.depth(),
        gate_count: circuit.gate_count(),
        output_probabilities: reg.state.probabilities(),
    })
}

/// Run Quantum Phase Estimation
#[tauri::command]
pub async fn run_qpe(precision: usize, phase: f64, shots: usize) -> Result<QpeResultDto> {
    if precision < 1 || precision > 10 {
        return Err(AppError::InvalidParameter(
            "Precision must be between 1 and 10".to_string(),
        ));
    }

    let qpe = QuantumPhaseEstimation::for_gate(precision, &Gate::t())
        .map_err(|e| AppError::Algorithm(e.to_string()))?;

    let result = qpe.estimate_known_phase(phase, shots);

    Ok(QpeResultDto {
        estimated_phase: result.phase,
        true_phase: phase,
        error: (result.phase - phase).abs(),
        confidence: result.confidence,
    })
}

/// Run Variational Quantum Eigensolver
#[tauri::command]
pub async fn run_vqe(qubits: usize, layers: usize, max_iterations: usize) -> Result<VqeResultDto> {
    if qubits < 1 || qubits > 8 {
        return Err(AppError::InvalidParameter(
            "Qubit count must be between 1 and 8".to_string(),
        ));
    }

    let hamiltonian = PauliSum::transverse_ising(qubits, 1.0, 0.5);

    let config = VQEConfig {
        num_qubits: qubits,
        ansatz: Ansatz::RealAmplitudes,
        layers,
        max_iterations,
        ..Default::default()
    };

    let vqe = VQE::new(config, hamiltonian);
    let result = vqe.run();

    Ok(VqeResultDto {
        energy: result.energy,
        iterations: result.num_evaluations,
        converged: result.converged,
        variance: result.variance,
        parameters: result.optimal_params,
    })
}

/// Run QAOA for MaxCut
#[tauri::command]
pub async fn run_qaoa(
    edges: Vec<(usize, usize)>,
    layers: usize,
    _shots: usize,
) -> Result<QaoaResultDto> {
    if edges.is_empty() {
        return Err(AppError::InvalidParameter(
            "At least one edge required".to_string(),
        ));
    }

    let qaoa = QAOA::max_cut(edges, layers);
    let result = qaoa.run();

    Ok(QaoaResultDto {
        best_solution: result.best_solution,
        best_cost: result.best_cost,
        approximation_ratio: result.approximation_ratio.unwrap_or(0.0),
        solution_counts: result.solution_counts,
    })
}
