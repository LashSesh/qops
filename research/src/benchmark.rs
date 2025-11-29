//! Benchmarking framework for quantum algorithms
//!
//! Provides tools for measuring and comparing algorithm performance.

use crate::{ResearchError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Benchmark name
    pub name: String,
    /// Number of repetitions
    pub repetitions: usize,
    /// Warmup runs (not counted)
    pub warmup: usize,
    /// Timeout per run
    pub timeout: Option<Duration>,
    /// Random seed for reproducibility
    pub seed: Option<u64>,
    /// Custom parameters
    pub params: HashMap<String, serde_json::Value>,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            name: "unnamed".to_string(),
            repetitions: 10,
            warmup: 2,
            timeout: Some(Duration::from_secs(60)),
            seed: None,
            params: HashMap::new(),
        }
    }
}

impl BenchmarkConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn repetitions(mut self, n: usize) -> Self {
        self.repetitions = n;
        self
    }

    pub fn warmup(mut self, n: usize) -> Self {
        self.warmup = n;
        self
    }

    pub fn param<T: Serialize>(mut self, key: &str, value: T) -> Self {
        self.params.insert(key.to_string(), serde_json::to_value(value).unwrap());
        self
    }
}

/// Single benchmark measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    /// Execution time
    pub duration: Duration,
    /// Memory usage (if available)
    pub memory_bytes: Option<usize>,
    /// Gate count
    pub gate_count: Option<usize>,
    /// Circuit depth
    pub circuit_depth: Option<usize>,
    /// Success probability
    pub success_probability: Option<f64>,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

impl Measurement {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            memory_bytes: None,
            gate_count: None,
            circuit_depth: None,
            success_probability: None,
            custom_metrics: HashMap::new(),
        }
    }

    pub fn with_gate_count(mut self, count: usize) -> Self {
        self.gate_count = Some(count);
        self
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.circuit_depth = Some(depth);
        self
    }

    pub fn with_success_prob(mut self, prob: f64) -> Self {
        self.success_probability = Some(prob);
        self
    }

    pub fn with_metric(mut self, name: &str, value: f64) -> Self {
        self.custom_metrics.insert(name.to_string(), value);
        self
    }
}

/// Benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Unique identifier
    pub id: String,
    /// Configuration used
    pub config: BenchmarkConfig,
    /// Individual measurements
    pub measurements: Vec<Measurement>,
    /// Start time
    pub started_at: DateTime<Utc>,
    /// End time
    pub completed_at: DateTime<Utc>,
    /// Whether benchmark completed successfully
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

impl BenchmarkResult {
    /// Mean execution time
    pub fn mean_duration(&self) -> Duration {
        if self.measurements.is_empty() {
            return Duration::ZERO;
        }
        let total: Duration = self.measurements.iter().map(|m| m.duration).sum();
        total / self.measurements.len() as u32
    }

    /// Standard deviation of execution time
    pub fn std_duration(&self) -> Duration {
        if self.measurements.len() < 2 {
            return Duration::ZERO;
        }

        let mean = self.mean_duration().as_secs_f64();
        let variance: f64 = self.measurements.iter()
            .map(|m| {
                let diff = m.duration.as_secs_f64() - mean;
                diff * diff
            })
            .sum::<f64>() / (self.measurements.len() - 1) as f64;

        Duration::from_secs_f64(variance.sqrt())
    }

    /// Minimum execution time
    pub fn min_duration(&self) -> Duration {
        self.measurements.iter()
            .map(|m| m.duration)
            .min()
            .unwrap_or(Duration::ZERO)
    }

    /// Maximum execution time
    pub fn max_duration(&self) -> Duration {
        self.measurements.iter()
            .map(|m| m.duration)
            .max()
            .unwrap_or(Duration::ZERO)
    }

    /// Mean success probability
    pub fn mean_success_probability(&self) -> Option<f64> {
        let probs: Vec<f64> = self.measurements.iter()
            .filter_map(|m| m.success_probability)
            .collect();

        if probs.is_empty() {
            None
        } else {
            Some(probs.iter().sum::<f64>() / probs.len() as f64)
        }
    }

    /// Mean gate count
    pub fn mean_gate_count(&self) -> Option<f64> {
        let counts: Vec<f64> = self.measurements.iter()
            .filter_map(|m| m.gate_count.map(|c| c as f64))
            .collect();

        if counts.is_empty() {
            None
        } else {
            Some(counts.iter().sum::<f64>() / counts.len() as f64)
        }
    }

    /// Summary statistics
    pub fn summary(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Benchmark: {}\n", self.config.name));
        s.push_str(&format!("Repetitions: {}\n", self.measurements.len()));
        s.push_str(&format!("Mean time: {:?}\n", self.mean_duration()));
        s.push_str(&format!("Std time: {:?}\n", self.std_duration()));
        s.push_str(&format!("Min time: {:?}\n", self.min_duration()));
        s.push_str(&format!("Max time: {:?}\n", self.max_duration()));

        if let Some(prob) = self.mean_success_probability() {
            s.push_str(&format!("Mean success prob: {:.4}\n", prob));
        }

        if let Some(count) = self.mean_gate_count() {
            s.push_str(&format!("Mean gate count: {:.1}\n", count));
        }

        s
    }
}

/// Main benchmark runner
pub struct Benchmark {
    config: BenchmarkConfig,
}

impl Benchmark {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    /// Create with name only
    pub fn named(name: &str) -> Self {
        Self::new(BenchmarkConfig::new(name))
    }

    /// Run benchmark with a function that returns a Measurement
    pub fn run<F>(&self, mut f: F) -> BenchmarkResult
    where
        F: FnMut() -> Measurement,
    {
        let started_at = Utc::now();
        let mut measurements = Vec::new();

        // Warmup runs
        for _ in 0..self.config.warmup {
            let _ = f();
        }

        // Actual measurements
        for _ in 0..self.config.repetitions {
            let measurement = f();
            measurements.push(measurement);
        }

        let completed_at = Utc::now();

        BenchmarkResult {
            id: Uuid::new_v4().to_string(),
            config: self.config.clone(),
            measurements,
            started_at,
            completed_at,
            success: true,
            error: None,
        }
    }

    /// Run benchmark timing only
    pub fn run_timed<F, T>(&self, mut f: F) -> BenchmarkResult
    where
        F: FnMut() -> T,
    {
        self.run(|| {
            let start = Instant::now();
            let _ = f();
            Measurement::new(start.elapsed())
        })
    }
}

/// Benchmark suite for running multiple benchmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuite {
    /// Suite name
    pub name: String,
    /// Description
    pub description: String,
    /// Results from all benchmarks
    pub results: Vec<BenchmarkResult>,
    /// Suite metadata
    pub metadata: HashMap<String, String>,
}

impl BenchmarkSuite {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            results: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }

    /// Generate comparison table
    pub fn comparison_table(&self) -> String {
        let mut table = String::new();
        table.push_str(&format!("{:<30} {:>15} {:>15} {:>15}\n",
            "Benchmark", "Mean (ms)", "Std (ms)", "Success %"));
        table.push_str(&"-".repeat(75));
        table.push('\n');

        for result in &self.results {
            let mean_ms = result.mean_duration().as_secs_f64() * 1000.0;
            let std_ms = result.std_duration().as_secs_f64() * 1000.0;
            let success = result.mean_success_probability()
                .map(|p| format!("{:.1}%", p * 100.0))
                .unwrap_or_else(|| "N/A".to_string());

            table.push_str(&format!("{:<30} {:>15.3} {:>15.3} {:>15}\n",
                result.config.name, mean_ms, std_ms, success));
        }

        table
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| ResearchError::SerializationError(e.to_string()))
    }
}

/// Pre-built quantum algorithm benchmarks
pub mod quantum_benchmarks {
    use super::*;
    use qops_circuits::{Circuit, QuantumRegister};
    use qops_algorithms::{Grover, Oracle, QuantumFourierTransform};

    /// Benchmark Grover's algorithm scaling
    pub fn grover_scaling(qubits: &[usize], shots: usize) -> BenchmarkSuite {
        let mut suite = BenchmarkSuite::new("Grover Scaling")
            .description("Grover's algorithm performance vs problem size");

        for &n in qubits {
            let config = BenchmarkConfig::new(&format!("grover_{}_qubits", n))
                .repetitions(5)
                .param("qubits", n)
                .param("shots", shots);

            let benchmark = Benchmark::new(config);

            let result = benchmark.run(|| {
                let start = Instant::now();

                // Search for random target
                let target = 0;
                let oracle = Oracle::marked_state(n, target);
                let grover = Grover::new(n, oracle);
                let grover_result = grover.run_with_shots(shots);

                let circuit = grover.build_circuit();

                Measurement::new(start.elapsed())
                    .with_gate_count(circuit.gate_count())
                    .with_depth(circuit.depth())
                    .with_success_prob(grover_result.success_probability)
            });

            suite.add_result(result);
        }

        suite
    }

    /// Benchmark QFT scaling
    pub fn qft_scaling(qubits: &[usize]) -> BenchmarkSuite {
        let mut suite = BenchmarkSuite::new("QFT Scaling")
            .description("Quantum Fourier Transform performance vs qubit count");

        for &n in qubits {
            let config = BenchmarkConfig::new(&format!("qft_{}_qubits", n))
                .repetitions(10)
                .param("qubits", n);

            let benchmark = Benchmark::new(config);

            let result = benchmark.run(|| {
                let start = Instant::now();

                let qft = QuantumFourierTransform::new(n);
                let circuit = qft.build_circuit();

                let mut reg = QuantumRegister::new(n);
                qft.apply(&mut reg).ok();

                Measurement::new(start.elapsed())
                    .with_gate_count(circuit.gate_count())
                    .with_depth(circuit.depth())
            });

            suite.add_result(result);
        }

        suite
    }

    /// Benchmark circuit simulation scaling
    pub fn simulation_scaling(qubits: &[usize], depth: usize) -> BenchmarkSuite {
        let mut suite = BenchmarkSuite::new("Simulation Scaling")
            .description("State vector simulation performance vs qubit count");

        for &n in qubits {
            let config = BenchmarkConfig::new(&format!("sim_{}_qubits", n))
                .repetitions(5)
                .param("qubits", n)
                .param("depth", depth);

            let benchmark = Benchmark::new(config);

            let result = benchmark.run(|| {
                let start = Instant::now();

                let mut circuit = Circuit::new(n);
                for _ in 0..depth {
                    for q in 0..n {
                        circuit = circuit.h(q);
                    }
                    for q in 0..n.saturating_sub(1) {
                        circuit = circuit.cnot(q, q + 1);
                    }
                }

                let mut reg = QuantumRegister::new(n);
                reg.apply_circuit(&circuit).ok();

                Measurement::new(start.elapsed())
                    .with_gate_count(circuit.gate_count())
                    .with_depth(circuit.depth())
            });

            suite.add_result(result);
        }

        suite
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_basic() {
        let config = BenchmarkConfig::new("test")
            .repetitions(3)
            .warmup(1);

        let benchmark = Benchmark::new(config);
        let result = benchmark.run_timed(|| {
            std::thread::sleep(Duration::from_millis(1));
        });

        assert!(result.success);
        assert_eq!(result.measurements.len(), 3);
    }

    #[test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::new("Test Suite");

        let result1 = BenchmarkResult {
            id: "1".to_string(),
            config: BenchmarkConfig::new("bench1"),
            measurements: vec![Measurement::new(Duration::from_millis(10))],
            started_at: Utc::now(),
            completed_at: Utc::now(),
            success: true,
            error: None,
        };

        suite.add_result(result1);
        assert_eq!(suite.results.len(), 1);
    }

    #[test]
    fn test_statistics() {
        let result = BenchmarkResult {
            id: "test".to_string(),
            config: BenchmarkConfig::new("stats_test"),
            measurements: vec![
                Measurement::new(Duration::from_millis(10)),
                Measurement::new(Duration::from_millis(20)),
                Measurement::new(Duration::from_millis(15)),
            ],
            started_at: Utc::now(),
            completed_at: Utc::now(),
            success: true,
            error: None,
        };

        assert_eq!(result.mean_duration(), Duration::from_millis(15));
        assert_eq!(result.min_duration(), Duration::from_millis(10));
        assert_eq!(result.max_duration(), Duration::from_millis(20));
    }
}
