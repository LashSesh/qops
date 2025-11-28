//! Research-related Tauri commands

use super::*;
use crate::error::{AppError, Result};
use qops_research::benchmark::quantum_benchmarks;
use std::collections::HashMap;

/// Run an experiment
#[tauri::command]
pub async fn run_experiment(
    algorithm: String,
    qubit_range: Vec<usize>,
    repetitions: usize,
    shots: usize,
) -> Result<ExperimentResultDto> {
    if qubit_range.is_empty() {
        return Err(AppError::InvalidParameter(
            "At least one qubit count required".to_string(),
        ));
    }

    use qops_algorithms::{Grover, Oracle};

    let mut all_results = Vec::new();
    let mut summary = HashMap::new();

    for &qubits in &qubit_range {
        let mut qubit_results = Vec::new();

        for _ in 0..repetitions {
            let result = match algorithm.to_lowercase().as_str() {
                "grover" => {
                    let oracle = Oracle::marked_state(qubits, 0);
                    let grover = Grover::new(qubits, oracle);
                    let grover_result = grover.run_with_shots(shots);
                    serde_json::json!({
                        "qubits": qubits,
                        "success_probability": grover_result.success_probability,
                        "iterations": grover_result.iterations
                    })
                }
                _ => {
                    return Err(AppError::InvalidParameter(format!(
                        "Unknown algorithm: {}",
                        algorithm
                    )))
                }
            };
            qubit_results.push(result);
        }

        all_results.extend(qubit_results);
    }

    // Calculate summary statistics
    for qubits in &qubit_range {
        let relevant: Vec<f64> = all_results
            .iter()
            .filter(|r| r["qubits"].as_u64().unwrap_or(0) == *qubits as u64)
            .filter_map(|r| r["success_probability"].as_f64())
            .collect();

        if !relevant.is_empty() {
            let mean = relevant.iter().sum::<f64>() / relevant.len() as f64;
            summary.insert(format!("{}_qubits_mean", qubits), mean);
        }
    }

    Ok(ExperimentResultDto {
        name: algorithm,
        total_runs: all_results.len(),
        results: all_results,
        summary,
    })
}

/// Run benchmarks
#[tauri::command]
pub async fn run_benchmark(
    algorithm: String,
    qubit_counts: Vec<usize>,
    shots: usize,
) -> Result<BenchmarkResultDto> {
    if qubit_counts.is_empty() {
        return Err(AppError::InvalidParameter(
            "At least one qubit count required".to_string(),
        ));
    }

    let suite = match algorithm.to_lowercase().as_str() {
        "grover" => quantum_benchmarks::grover_scaling(&qubit_counts, shots),
        "qft" => quantum_benchmarks::qft_scaling(&qubit_counts),
        "simulation" => quantum_benchmarks::simulation_scaling(&qubit_counts, 10),
        _ => {
            return Err(AppError::InvalidParameter(format!(
                "Unknown benchmark: {}",
                algorithm
            )))
        }
    };

    let comparison_table = suite.comparison_table();

    let configs: Vec<BenchmarkConfigDto> = qubit_counts
        .iter()
        .map(|&q| BenchmarkConfigDto {
            qubits: q,
            mean_time_ms: 0.0, // Would need to extract from suite
            std_time_ms: 0.0,
            success_rate: 0.0,
        })
        .collect();

    Ok(BenchmarkResultDto {
        algorithm,
        configurations: configs,
        comparison_table,
    })
}
