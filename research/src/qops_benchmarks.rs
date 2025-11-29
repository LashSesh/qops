//! QOPS Benchmark Suite
//!
//! Comprehensive benchmark framework for quantum algorithms, hypercube cascades,
//! mining operations, and topology computations.
//!
//! This module ports the QSO benchmark families to the QOPS framework and
//! extends them with new benchmarks for the Hypercube/FUQ! capabilities.

use crate::{BenchmarkConfig, Benchmark, BenchmarkResult, BenchmarkSuite};
use crate::benchmark::Measurement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;
use chrono::{DateTime, Utc};

/// System information for benchmark reproducibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub cpu_count: usize,
    pub hostname: String,
}

impl SystemInfo {
    pub fn current() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            cpu_count: std::thread::available_parallelism()
                .map(|p| p.get())
                .unwrap_or(1),
            hostname: std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string()),
        }
    }
}

/// Metadata for all benchmark result files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetadata {
    pub timestamp: DateTime<Utc>,
    pub git_commit: Option<String>,
    pub system_info: SystemInfo,
    pub benchmark_version: String,
    pub qops_version: String,
}

impl BenchmarkMetadata {
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            git_commit: get_git_commit(),
            system_info: SystemInfo::current(),
            benchmark_version: "1.0.0".to_string(),
            qops_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for BenchmarkMetadata {
    fn default() -> Self {
        Self::new()
    }
}

/// Get current git commit hash
fn get_git_commit() -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Benchmark output schema for all benchmark files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkOutput {
    pub metadata: BenchmarkMetadata,
    pub benchmark_type: String,
    pub suite: BenchmarkSuite,
    pub summary: BenchmarkSummary,
}

/// Summary statistics for benchmark output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_benchmarks: usize,
    pub total_measurements: usize,
    pub total_duration_ms: f64,
    pub success_rate: f64,
}

impl BenchmarkOutput {
    pub fn new(benchmark_type: &str, suite: BenchmarkSuite) -> Self {
        let total_measurements: usize = suite.results.iter()
            .map(|r| r.measurements.len())
            .sum();
        let total_duration_ms: f64 = suite.results.iter()
            .flat_map(|r| r.measurements.iter())
            .map(|m| m.duration.as_secs_f64() * 1000.0)
            .sum();
        let success_count = suite.results.iter().filter(|r| r.success).count();

        Self {
            metadata: BenchmarkMetadata::new(),
            benchmark_type: benchmark_type.to_string(),
            summary: BenchmarkSummary {
                total_benchmarks: suite.results.len(),
                total_measurements,
                total_duration_ms,
                success_rate: if suite.results.is_empty() {
                    1.0
                } else {
                    success_count as f64 / suite.results.len() as f64
                },
            },
            suite,
        }
    }

    /// Save benchmark output to JSON file
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, json)
    }
}

// =============================================================================
// VQE Benchmarks
// =============================================================================

/// VQE benchmark configuration
#[derive(Debug, Clone)]
pub struct VqeBenchConfig {
    pub qubits: Vec<usize>,
    pub layers: Vec<usize>,
    pub repetitions: usize,
    pub small_mode: bool,
}

impl Default for VqeBenchConfig {
    fn default() -> Self {
        Self {
            qubits: vec![2, 3, 4],
            layers: vec![1, 2, 3],
            repetitions: 5,
            small_mode: false,
        }
    }
}

impl VqeBenchConfig {
    pub fn small() -> Self {
        Self {
            qubits: vec![2],
            layers: vec![1],
            repetitions: 2,
            small_mode: true,
        }
    }
}

/// Run VQE benchmarks
pub fn run_vqe_benchmarks(config: VqeBenchConfig) -> BenchmarkOutput {
    use qops_algorithms::{VQE, VQEConfig, Ansatz, vqe::PauliSum};

    let mut suite = BenchmarkSuite::new("VQE Benchmark Suite")
        .description("Variational Quantum Eigensolver performance benchmarks");

    for &qubits in &config.qubits {
        for &layers in &config.layers {
            let bench_config = BenchmarkConfig::new(&format!("vqe_{}q_{}l", qubits, layers))
                .repetitions(config.repetitions)
                .warmup(1)
                .param("qubits", qubits)
                .param("layers", layers);

            let benchmark = Benchmark::new(bench_config);

            let result = benchmark.run(|| {
                let start = Instant::now();

                let hamiltonian = PauliSum::transverse_ising(qubits, 1.0, 0.5);
                let vqe_config = VQEConfig {
                    num_qubits: qubits,
                    ansatz: Ansatz::RealAmplitudes,
                    layers,
                    max_iterations: if config.small_mode { 10 } else { 50 },
                    ..Default::default()
                };

                let vqe = VQE::new(vqe_config, hamiltonian);
                let vqe_result = vqe.run();

                let elapsed = start.elapsed();
                Measurement::new(elapsed)
                    .with_metric("energy", vqe_result.energy)
                    .with_metric("evaluations", vqe_result.num_evaluations as f64)
                    .with_metric("converged", if vqe_result.converged { 1.0 } else { 0.0 })
                    .with_metric("variance", vqe_result.variance)
            });

            suite.add_result(result);
        }
    }

    BenchmarkOutput::new("vqe", suite)
}

// =============================================================================
// VQC Benchmarks (Variational Quantum Classifier)
// =============================================================================

/// VQC benchmark configuration
#[derive(Debug, Clone)]
pub struct VqcBenchConfig {
    pub qubits: Vec<usize>,
    pub layers: Vec<usize>,
    pub samples: usize,
    pub repetitions: usize,
    pub small_mode: bool,
}

impl Default for VqcBenchConfig {
    fn default() -> Self {
        Self {
            qubits: vec![2, 3, 4],
            layers: vec![1, 2],
            samples: 100,
            repetitions: 5,
            small_mode: false,
        }
    }
}

impl VqcBenchConfig {
    pub fn small() -> Self {
        Self {
            qubits: vec![2],
            layers: vec![1],
            samples: 20,
            repetitions: 2,
            small_mode: true,
        }
    }
}

/// Run VQC (classification) benchmarks
pub fn run_vqc_benchmarks(config: VqcBenchConfig) -> BenchmarkOutput {
    use qops_circuits::{Circuit, QuantumRegister};
    use rand::Rng;

    let mut suite = BenchmarkSuite::new("VQC Benchmark Suite")
        .description("Variational Quantum Classifier benchmarks for quantum machine learning");

    for &qubits in &config.qubits {
        for &layers in &config.layers {
            let bench_config = BenchmarkConfig::new(&format!("vqc_{}q_{}l", qubits, layers))
                .repetitions(config.repetitions)
                .warmup(1)
                .param("qubits", qubits)
                .param("layers", layers)
                .param("samples", config.samples);

            let benchmark = Benchmark::new(bench_config);

            let result = benchmark.run(|| {
                let start = Instant::now();
                let mut rng = rand::thread_rng();
                let mut correct = 0;

                for _ in 0..config.samples {
                    // Create parametrized circuit
                    let mut circuit = Circuit::new(qubits);
                    
                    // Feature encoding
                    for q in 0..qubits {
                        let theta = rng.gen::<f64>() * std::f64::consts::PI;
                        circuit = circuit.ry(theta, q);
                    }
                    
                    // Variational layers
                    for _ in 0..layers {
                        for q in 0..qubits {
                            let theta = rng.gen::<f64>() * std::f64::consts::PI;
                            circuit = circuit.ry(theta, q);
                        }
                        for q in 0..qubits.saturating_sub(1) {
                            circuit = circuit.cnot(q, q + 1);
                        }
                    }

                    let mut reg = QuantumRegister::new(qubits);
                    let _ = reg.apply_circuit(&circuit);

                    // Simulated classification accuracy
                    let prob = reg.state.probabilities()[0];
                    let predicted = if prob > 0.5 { 0 } else { 1 };
                    let actual: usize = rng.gen_range(0..2);
                    if predicted == actual {
                        correct += 1;
                    }
                }

                let accuracy = correct as f64 / config.samples as f64;

                Measurement::new(start.elapsed())
                    .with_metric("accuracy", accuracy)
                    .with_metric("samples", config.samples as f64)
                    .with_gate_count(qubits * (1 + layers * 2) + (qubits - 1) * layers)
            });

            suite.add_result(result);
        }
    }

    BenchmarkOutput::new("vqc", suite)
}

// =============================================================================
// QAOA Benchmarks
// =============================================================================

/// QAOA benchmark configuration
#[derive(Debug, Clone)]
pub struct QaoaBenchConfig {
    pub nodes: Vec<usize>,
    pub layers: Vec<usize>,
    pub repetitions: usize,
    pub small_mode: bool,
}

impl Default for QaoaBenchConfig {
    fn default() -> Self {
        Self {
            nodes: vec![4, 6, 8],
            layers: vec![1, 2, 3],
            repetitions: 5,
            small_mode: false,
        }
    }
}

impl QaoaBenchConfig {
    pub fn small() -> Self {
        Self {
            nodes: vec![4],
            layers: vec![1],
            repetitions: 2,
            small_mode: true,
        }
    }
}

/// Run QAOA MaxCut benchmarks
pub fn run_qaoa_benchmarks(config: QaoaBenchConfig) -> BenchmarkOutput {
    use qops_algorithms::QAOA;

    let mut suite = BenchmarkSuite::new("QAOA Benchmark Suite")
        .description("QAOA MaxCut problem benchmarks");

    for &nodes in &config.nodes {
        for &layers in &config.layers {
            let bench_config = BenchmarkConfig::new(&format!("qaoa_{}n_{}p", nodes, layers))
                .repetitions(config.repetitions)
                .warmup(1)
                .param("nodes", nodes)
                .param("layers", layers);

            let benchmark = Benchmark::new(bench_config);

            let result = benchmark.run(|| {
                let start = Instant::now();

                // Create ring graph for MaxCut
                let edges: Vec<(usize, usize)> = (0..nodes)
                    .map(|i| (i, (i + 1) % nodes))
                    .collect();

                let qaoa = QAOA::max_cut(edges, layers);
                let qaoa_result = qaoa.run();

                // Compute approximation ratio (optimal MaxCut on ring = n for even n)
                let optimal = nodes as f64;
                let approx_ratio = qaoa_result.best_cost / optimal;

                Measurement::new(start.elapsed())
                    .with_metric("cut_value", qaoa_result.best_cost)
                    .with_metric("approximation_ratio", approx_ratio)
                    .with_metric("evaluations", qaoa_result.energy_history.len() as f64)
            });

            suite.add_result(result);
        }
    }

    BenchmarkOutput::new("qaoa", suite)
}

// =============================================================================
// Quantum Walk Benchmarks
// =============================================================================

/// Quantum Walk benchmark configuration
#[derive(Debug, Clone)]
pub struct QWalkBenchConfig {
    pub graph_sizes: Vec<usize>,
    pub time_steps: Vec<f64>,
    pub repetitions: usize,
    pub small_mode: bool,
}

impl Default for QWalkBenchConfig {
    fn default() -> Self {
        Self {
            graph_sizes: vec![4, 8, 13],
            time_steps: vec![0.5, 1.0, 2.0, 5.0],
            repetitions: 5,
            small_mode: false,
        }
    }
}

impl QWalkBenchConfig {
    pub fn small() -> Self {
        Self {
            graph_sizes: vec![4],
            time_steps: vec![1.0],
            repetitions: 2,
            small_mode: true,
        }
    }
}

/// Run quantum walk benchmarks
pub fn run_qwalk_benchmarks(config: QWalkBenchConfig) -> BenchmarkOutput {
    use qops_quantum::{MetatronGraph, MetatronHamiltonian, QuantumState};
    use qops_quantum::quantum_walk::ContinuousQuantumWalk;

    let mut suite = BenchmarkSuite::new("Quantum Walk Benchmark Suite")
        .description("Continuous-time quantum walk benchmarks measuring mixing and hitting times");

    for &size in &config.graph_sizes {
        for &t in &config.time_steps {
            let bench_config = BenchmarkConfig::new(&format!("qwalk_{}n_t{:.1}", size, t))
                .repetitions(config.repetitions)
                .warmup(1)
                .param("graph_size", size)
                .param("time", t);

            let benchmark = Benchmark::new(bench_config);

            let result = benchmark.run(|| {
                let start = Instant::now();

                // Use Metatron graph (13 nodes) or smaller subgraph
                let graph = MetatronGraph::new();
                let hamiltonian = MetatronHamiltonian::from_graph(&graph);
                let qw = ContinuousQuantumWalk::new(hamiltonian);

                let initial = QuantumState::basis_state(0).unwrap();
                let evolved = qw.evolve(&initial, t);

                // Compute mixing metrics
                let probs = evolved.probabilities();
                let uniform = 1.0 / probs.len() as f64;
                let tvd: f64 = probs.iter()
                    .map(|p| (p - uniform).abs())
                    .sum::<f64>() / 2.0;

                // Hitting time metric (probability at target)
                let target = if size <= probs.len() { size - 1 } else { probs.len() - 1 };
                let hitting_prob = probs.get(target).copied().unwrap_or(0.0);

                // Speedup estimate (classical vs quantum)
                let classical_steps = size as f64;
                let quantum_steps = t;
                let speedup = classical_steps / quantum_steps.max(0.001);

                Measurement::new(start.elapsed())
                    .with_metric("tvd", tvd)
                    .with_metric("hitting_probability", hitting_prob)
                    .with_metric("speedup_estimate", speedup)
                    .with_metric("mixing_quality", 1.0 - tvd)
            });

            suite.add_result(result);
        }
    }

    BenchmarkOutput::new("quantum_walk", suite)
}

// =============================================================================
// Advanced Algorithms Benchmarks (Grover, QFT, QPE)
// =============================================================================

/// Advanced algorithms benchmark configuration
#[derive(Debug, Clone)]
pub struct AdvancedBenchConfig {
    pub grover_qubits: Vec<usize>,
    pub qft_qubits: Vec<usize>,
    pub qpe_precision: Vec<usize>,
    pub shots: usize,
    pub repetitions: usize,
    pub small_mode: bool,
}

impl Default for AdvancedBenchConfig {
    fn default() -> Self {
        Self {
            grover_qubits: vec![2, 3, 4, 5],
            qft_qubits: vec![2, 3, 4, 5, 6],
            qpe_precision: vec![3, 4, 5],
            shots: 1000,
            repetitions: 5,
            small_mode: false,
        }
    }
}

impl AdvancedBenchConfig {
    pub fn small() -> Self {
        Self {
            grover_qubits: vec![2, 3],
            qft_qubits: vec![2, 3],
            qpe_precision: vec![3],
            shots: 100,
            repetitions: 2,
            small_mode: true,
        }
    }
}

/// Run advanced algorithms benchmarks
pub fn run_advanced_benchmarks(config: AdvancedBenchConfig) -> BenchmarkOutput {
    use qops_algorithms::{Grover, Oracle, QuantumFourierTransform, QuantumPhaseEstimation};
    use qops_circuits::{QuantumRegister, Gate};

    let mut suite = BenchmarkSuite::new("Advanced Algorithms Benchmark Suite")
        .description("Grover, QFT, QPE, and other advanced quantum algorithms");

    // Grover benchmarks
    for &qubits in &config.grover_qubits {
        let bench_config = BenchmarkConfig::new(&format!("grover_{}q", qubits))
            .repetitions(config.repetitions)
            .warmup(1)
            .param("qubits", qubits)
            .param("shots", config.shots);

        let benchmark = Benchmark::new(bench_config);

        let result = benchmark.run(|| {
            let start = Instant::now();

            let target = 0;
            let oracle = Oracle::marked_state(qubits, target);
            let grover = Grover::new(qubits, oracle);
            let grover_result = grover.run_with_shots(config.shots);
            let circuit = grover.build_circuit();

            // Compute quantum speedup
            let search_space = 1 << qubits;
            let classical_queries = (search_space / 2) as f64;
            let quantum_queries = grover.optimal_iterations() as f64;
            let speedup = classical_queries / quantum_queries.max(1.0);

            Measurement::new(start.elapsed())
                .with_gate_count(circuit.gate_count())
                .with_depth(circuit.depth())
                .with_success_prob(grover_result.success_probability)
                .with_metric("quantum_evaluations", quantum_queries)
                .with_metric("speedup", speedup)
        });

        suite.add_result(result);
    }

    // QFT benchmarks
    for &qubits in &config.qft_qubits {
        let bench_config = BenchmarkConfig::new(&format!("qft_{}q", qubits))
            .repetitions(config.repetitions)
            .warmup(1)
            .param("qubits", qubits);

        let benchmark = Benchmark::new(bench_config);

        let result = benchmark.run(|| {
            let start = Instant::now();

            let qft = QuantumFourierTransform::new(qubits);
            let circuit = qft.build_circuit();

            let mut reg = QuantumRegister::new(qubits);
            let _ = reg.apply_single_gate(&Gate::x(), 0);
            let _ = qft.apply(&mut reg);

            // Compute fidelity to expected uniform superposition
            let probs = reg.state.probabilities();
            let expected = 1.0 / (1 << qubits) as f64;
            let fidelity: f64 = probs.iter()
                .map(|p| (p * expected).sqrt())
                .sum::<f64>()
                .powi(2);

            Measurement::new(start.elapsed())
                .with_gate_count(circuit.gate_count())
                .with_depth(circuit.depth())
                .with_metric("fidelity", fidelity)
        });

        suite.add_result(result);
    }

    // QPE benchmarks
    for &precision in &config.qpe_precision {
        let bench_config = BenchmarkConfig::new(&format!("qpe_{}p", precision))
            .repetitions(config.repetitions)
            .warmup(1)
            .param("precision", precision);

        let benchmark = Benchmark::new(bench_config);

        let result = benchmark.run(|| {
            let start = Instant::now();

            let qpe = QuantumPhaseEstimation::for_gate(precision, &Gate::t()).unwrap();
            let true_phase = 0.125; // T gate has phase 2π/8 = 0.125
            let qpe_result = qpe.estimate_known_phase(true_phase, config.shots);

            let error = (qpe_result.phase - true_phase).abs();

            Measurement::new(start.elapsed())
                .with_metric("estimated_phase", qpe_result.phase)
                .with_metric("error", error)
                .with_metric("confidence", qpe_result.confidence)
                .with_metric("error_bound", qpe_result.error_bound())
        });

        suite.add_result(result);
    }

    BenchmarkOutput::new("advanced_algorithms", suite)
}

// =============================================================================
// Integration Benchmarks
// =============================================================================

/// Integration benchmark configuration
#[derive(Debug, Clone)]
pub struct IntegrationBenchConfig {
    pub repetitions: usize,
    pub small_mode: bool,
}

impl Default for IntegrationBenchConfig {
    fn default() -> Self {
        Self {
            repetitions: 5,
            small_mode: false,
        }
    }
}

impl IntegrationBenchConfig {
    pub fn small() -> Self {
        Self {
            repetitions: 2,
            small_mode: true,
        }
    }
}

/// Run integration benchmarks (cross-module compatibility)
pub fn run_integration_benchmarks(config: IntegrationBenchConfig) -> BenchmarkOutput {
    use qops_circuits::{Circuit, QuantumRegister};
    use qops_algorithms::{VQE, VQEConfig, Ansatz, vqe::PauliSum, QAOA};

    let mut suite = BenchmarkSuite::new("Integration Benchmark Suite")
        .description("Cross-module compatibility and pipeline integration benchmarks");

    // Circuits + Algorithms integration
    let bench_config = BenchmarkConfig::new("circuits_algorithms_integration")
        .repetitions(config.repetitions)
        .warmup(1);

    let benchmark = Benchmark::new(bench_config);
    let result = benchmark.run(|| {
        let start = Instant::now();

        // Create circuit
        let mut circuit = Circuit::new(3);
        circuit = circuit.h(0).h(1).h(2);
        circuit = circuit.cnot(0, 1).cnot(1, 2);

        let mut reg = QuantumRegister::new(3);
        let _ = reg.apply_circuit(&circuit);

        // Use in VQE context
        let hamiltonian = PauliSum::transverse_ising(3, 1.0, 0.5);
        let vqe_config = VQEConfig {
            num_qubits: 3,
            ansatz: Ansatz::RealAmplitudes,
            layers: 1,
            max_iterations: 10,
            ..Default::default()
        };
        let vqe = VQE::new(vqe_config, hamiltonian);
        let vqe_result = vqe.run();

        Measurement::new(start.elapsed())
            .with_metric("vqe_energy", vqe_result.energy)
            .with_gate_count(circuit.gate_count())
    });
    suite.add_result(result);

    // VQE + QAOA pipeline integration
    let bench_config = BenchmarkConfig::new("vqe_qaoa_pipeline")
        .repetitions(config.repetitions)
        .warmup(1);

    let benchmark = Benchmark::new(bench_config);
    let result = benchmark.run(|| {
        let start = Instant::now();

        // VQE stage
        let hamiltonian = PauliSum::transverse_ising(4, 1.0, 0.5);
        let vqe_config = VQEConfig {
            num_qubits: 4,
            ansatz: Ansatz::RealAmplitudes,
            layers: 1,
            max_iterations: 10,
            ..Default::default()
        };
        let vqe = VQE::new(vqe_config, hamiltonian);
        let vqe_result = vqe.run();

        // QAOA stage
        let edges: Vec<(usize, usize)> = (0..4).map(|i| (i, (i + 1) % 4)).collect();
        let qaoa = QAOA::max_cut(edges, 1);
        let qaoa_result = qaoa.run();

        Measurement::new(start.elapsed())
            .with_metric("vqe_energy", vqe_result.energy)
            .with_metric("qaoa_cost", qaoa_result.best_cost)
            .with_metric("total_evaluations", (vqe_result.num_evaluations + qaoa_result.energy_history.len()) as f64)
    });
    suite.add_result(result);

    BenchmarkOutput::new("integration", suite)
}

// =============================================================================
// Cross-System Comparison Benchmarks
// =============================================================================

/// Cross-system benchmark configuration
#[derive(Debug, Clone)]
pub struct CrossSystemBenchConfig {
    pub qubits: Vec<usize>,
    pub repetitions: usize,
    pub enable_qiskit: bool,
    pub enable_cirq: bool,
    pub enable_pennylane: bool,
    pub small_mode: bool,
}

impl Default for CrossSystemBenchConfig {
    fn default() -> Self {
        Self {
            qubits: vec![2, 3, 4],
            repetitions: 5,
            enable_qiskit: false,
            enable_cirq: false,
            enable_pennylane: false,
            small_mode: false,
        }
    }
}

impl CrossSystemBenchConfig {
    pub fn small() -> Self {
        Self {
            qubits: vec![2],
            repetitions: 2,
            enable_qiskit: false,
            enable_cirq: false,
            enable_pennylane: false,
            small_mode: true,
        }
    }
}

/// Run cross-system comparison benchmarks
pub fn run_cross_system_benchmarks(config: CrossSystemBenchConfig) -> BenchmarkOutput {
    use qops_circuits::{Circuit, QuantumRegister};

    let mut suite = BenchmarkSuite::new("Cross-System Comparison Benchmark Suite")
        .description("Comparison benchmarks between QOPS and external frameworks");

    // QOPS internal benchmarks (always run)
    for &qubits in &config.qubits {
        let bench_config = BenchmarkConfig::new(&format!("qops_bell_{}", qubits))
            .repetitions(config.repetitions)
            .warmup(1)
            .param("qubits", qubits)
            .param("framework", "qops");

        let benchmark = Benchmark::new(bench_config);

        let result = benchmark.run(|| {
            let start = Instant::now();

            // Create Bell-like entangled state
            let mut circuit = Circuit::new(qubits);
            circuit = circuit.h(0);
            for i in 0..qubits - 1 {
                circuit = circuit.cnot(i, i + 1);
            }

            let mut reg = QuantumRegister::new(qubits);
            let _ = reg.apply_circuit(&circuit);

            Measurement::new(start.elapsed())
                .with_gate_count(circuit.gate_count())
                .with_depth(circuit.depth())
                .with_metric("framework_id", 0.0)
        });

        suite.add_result(result);
    }

    if config.enable_qiskit {
        suite.metadata.insert("qiskit_enabled".to_string(), "true".to_string());
    }

    if config.enable_cirq {
        suite.metadata.insert("cirq_enabled".to_string(), "true".to_string());
    }

    if config.enable_pennylane {
        suite.metadata.insert("pennylane_enabled".to_string(), "true".to_string());
    }

    BenchmarkOutput::new("cross_system", suite)
}

// =============================================================================
// Hypercube Cascade Benchmarks
// =============================================================================

/// Hypercube cascade benchmark configuration
#[derive(Debug, Clone)]
pub struct HypercubeBenchConfig {
    pub dimensions: Vec<usize>,
    pub hdag_sizes: Vec<usize>,
    pub cascade_steps: Vec<usize>,
    pub repetitions: usize,
    pub small_mode: bool,
}

impl Default for HypercubeBenchConfig {
    fn default() -> Self {
        Self {
            dimensions: vec![3, 4, 5],
            hdag_sizes: vec![5, 10, 20],
            cascade_steps: vec![5, 10, 20],
            repetitions: 5,
            small_mode: false,
        }
    }
}

impl HypercubeBenchConfig {
    pub fn small() -> Self {
        Self {
            dimensions: vec![3],
            hdag_sizes: vec![5],
            cascade_steps: vec![5],
            repetitions: 2,
            small_mode: true,
        }
    }
}

/// Run hypercube cascade benchmarks
pub fn run_hypercube_benchmarks(config: HypercubeBenchConfig) -> BenchmarkOutput {
    use qops_hypercube::{
        Hypercube, HypercubeConfig, CubeExpansionRule,
        HDAG, HDAGExecutor, Coord5D,
    };

    let mut suite = BenchmarkSuite::new("Hypercube Cascade Benchmark Suite")
        .description("Hypercube-HDAG cascade benchmarks measuring convergence and performance");

    // Dimension scaling benchmarks
    for &dim in &config.dimensions {
        for &steps in &config.cascade_steps {
            let bench_config = BenchmarkConfig::new(&format!("hypercube_{}d_{}s", dim, steps))
                .repetitions(config.repetitions)
                .warmup(1)
                .param("dimension", dim)
                .param("steps", steps);

            let benchmark = Benchmark::new(bench_config);

            let result = benchmark.run(|| {
                let start = Instant::now();

                let cube_config = HypercubeConfig {
                    max_depth: steps,
                    expansion_rule: CubeExpansionRule::Triton,
                    ..Default::default()
                };

                let mut cube = Hypercube::new("cascade_bench", cube_config);
                
                let mut total_vertices = 0;
                for _ in 0..steps {
                    let added = cube.expand_step().unwrap_or(0);
                    total_vertices += added;
                }
                let total_edges = cube.stats.total_edges;

                let initial_resonance = 0.5;
                let final_resonance = cube.best_resonance;
                let convergence_rate = (final_resonance - initial_resonance) / steps as f64;

                Measurement::new(start.elapsed())
                    .with_metric("total_vertices", total_vertices as f64)
                    .with_metric("total_edges", total_edges as f64)
                    .with_metric("best_resonance", final_resonance)
                    .with_metric("convergence_rate", convergence_rate)
                    .with_metric("max_depth", cube.stats.max_depth_reached as f64)
            });

            suite.add_result(result);
        }
    }

    // HDAG execution benchmarks
    for &size in &config.hdag_sizes {
        let bench_config = BenchmarkConfig::new(&format!("hdag_exec_{}", size))
            .repetitions(config.repetitions)
            .warmup(1)
            .param("hdag_size", size);

        let benchmark = Benchmark::new(bench_config);

        let result = benchmark.run(|| {
            let start = Instant::now();

            let seed = Coord5D::center();
            let hdag = HDAG::standard_pipeline(seed);

            let mut executor = HDAGExecutor::new(hdag);
            let exec_result = executor.execute().unwrap();

            Measurement::new(start.elapsed())
                .with_metric("nodes_executed", exec_result.nodes_executed as f64)
                .with_metric("nodes_failed", exec_result.nodes_failed as f64)
                .with_metric("output_resonance", exec_result.resonance)
                .with_metric("artifacts_generated", exec_result.artifact_count as f64)
        });

        suite.add_result(result);
    }

    BenchmarkOutput::new("hypercube_cascade", suite)
}

// =============================================================================
// Mining Benchmarks
// =============================================================================

/// Mining benchmark configuration
#[derive(Debug, Clone)]
pub struct MiningBenchConfig {
    pub mining_depths: Vec<usize>,
    pub strategies: Vec<String>,
    pub repetitions: usize,
    pub small_mode: bool,
}

impl Default for MiningBenchConfig {
    fn default() -> Self {
        Self {
            mining_depths: vec![5, 10, 20],
            strategies: vec!["greedy".to_string(), "beam".to_string(), "triton".to_string()],
            repetitions: 5,
            small_mode: false,
        }
    }
}

impl MiningBenchConfig {
    pub fn small() -> Self {
        Self {
            mining_depths: vec![5],
            strategies: vec!["greedy".to_string()],
            repetitions: 2,
            small_mode: true,
        }
    }
}

/// Run mining benchmarks
pub fn run_mining_benchmarks(config: MiningBenchConfig) -> BenchmarkOutput {
    use qops_slots::{SequenceMiner, MinerConfig, MiningStrategy};

    let mut suite = BenchmarkSuite::new("Mining Benchmark Suite")
        .description("Operator and program mining benchmarks");

    for &depth in &config.mining_depths {
        for strategy_name in &config.strategies {
            let strategy = match strategy_name.as_str() {
                "greedy" => MiningStrategy::Greedy,
                "beam" => MiningStrategy::BeamSearch,
                "stochastic" => MiningStrategy::Stochastic,
                "evolutionary" => MiningStrategy::Evolutionary,
                "triton" => MiningStrategy::Triton,
                _ => MiningStrategy::Greedy,
            };

            let bench_config = BenchmarkConfig::new(&format!("mining_{}_{}", strategy_name, depth))
                .repetitions(config.repetitions)
                .warmup(1)
                .param("depth", depth)
                .param("strategy", strategy_name.clone());

            let benchmark = Benchmark::new(bench_config);

            let result = benchmark.run(|| {
                let start = Instant::now();

                let miner_config = MinerConfig {
                    depth,
                    strategy: strategy.clone(),
                    target_resonance: 0.8,
                    ..Default::default()
                };

                let mut miner = SequenceMiner::new(miner_config);
                let mining_result = miner.mine().unwrap();

                let time_to_best_ms = mining_result.mining_time_ms as f64 *
                    (mining_result.steps_to_best as f64 / mining_result.total_steps.max(1) as f64);

                let jackpot = mining_result.top_sequences.iter()
                    .filter(|s| s.resonance >= 0.9)
                    .count();
                let good = mining_result.top_sequences.iter()
                    .filter(|s| s.resonance >= 0.7 && s.resonance < 0.9)
                    .count();
                let okay = mining_result.top_sequences.iter()
                    .filter(|s| s.resonance >= 0.5 && s.resonance < 0.7)
                    .count();
                let miss = mining_result.top_sequences.iter()
                    .filter(|s| s.resonance < 0.5)
                    .count();

                Measurement::new(start.elapsed())
                    .with_metric("best_resonance", mining_result.best_resonance)
                    .with_metric("total_steps", mining_result.total_steps as f64)
                    .with_metric("steps_to_best", mining_result.steps_to_best as f64)
                    .with_metric("time_to_best_ms", time_to_best_ms)
                    .with_metric("converged", if mining_result.converged { 1.0 } else { 0.0 })
                    .with_metric("jackpot_count", jackpot as f64)
                    .with_metric("good_count", good as f64)
                    .with_metric("okay_count", okay as f64)
                    .with_metric("miss_count", miss as f64)
            });

            suite.add_result(result);
        }
    }

    BenchmarkOutput::new("mining", suite)
}

// =============================================================================
// Topology Benchmarks
// =============================================================================

/// Topology benchmark configuration
#[derive(Debug, Clone)]
pub struct TopologyBenchConfig {
    pub graph_sizes: Vec<usize>,
    pub repetitions: usize,
    pub small_mode: bool,
}

impl Default for TopologyBenchConfig {
    fn default() -> Self {
        Self {
            graph_sizes: vec![4, 8, 13, 20],
            repetitions: 5,
            small_mode: false,
        }
    }
}

impl TopologyBenchConfig {
    pub fn small() -> Self {
        Self {
            graph_sizes: vec![4],
            repetitions: 2,
            small_mode: true,
        }
    }
}

/// Run topology/geometry benchmarks
pub fn run_topology_benchmarks(config: TopologyBenchConfig) -> BenchmarkOutput {
    use qops_core::Signature3D;
    use std::f64::consts::PI;

    let mut suite = BenchmarkSuite::new("Topology Benchmark Suite")
        .description("Topological invariants and geometry computation benchmarks");

    for &size in &config.graph_sizes {
        // Chern number computation benchmark
        let bench_config = BenchmarkConfig::new(&format!("chern_{}n", size))
            .repetitions(config.repetitions)
            .warmup(1)
            .param("graph_size", size);

        let benchmark = Benchmark::new(bench_config);

        let result = benchmark.run(|| {
            let start = Instant::now();

            let mut chern_sum = 0.0;
            let grid_points = size * size;
            
            for i in 0..grid_points {
                for j in 0..grid_points {
                    let k1 = 2.0 * PI * i as f64 / size as f64;
                    let k2 = 2.0 * PI * j as f64 / size as f64;
                    
                    let d1 = k1.sin();
                    let d2 = k2.sin();
                    let d3 = 2.0 - k1.cos() - k2.cos();
                    let d_norm = (d1*d1 + d2*d2 + d3*d3).sqrt();
                    
                    if d_norm > 0.001 {
                        let curvature = (d1 * d2 - d2 * d1) / (d_norm * d_norm * d_norm);
                        chern_sum += curvature;
                    }
                }
            }
            
            let chern_number = chern_sum / (2.0 * PI);

            Measurement::new(start.elapsed())
                .with_metric("chern_number", chern_number)
                .with_metric("grid_points", grid_points as f64)
        });

        suite.add_result(result);

        // Berry phase computation benchmark
        let bench_config = BenchmarkConfig::new(&format!("berry_{}n", size))
            .repetitions(config.repetitions)
            .warmup(1)
            .param("graph_size", size);

        let benchmark = Benchmark::new(bench_config);

        let result = benchmark.run(|| {
            let start = Instant::now();

            let loop_points = size * 4;
            let mut berry_phase = 0.0;
            
            for i in 0..loop_points {
                let theta = 2.0 * PI * i as f64 / loop_points as f64;
                let theta_next = 2.0 * PI * (i + 1) as f64 / loop_points as f64;
                
                let cos_t = (theta / 2.0).cos();
                let sin_t = (theta / 2.0).sin();
                let cos_tn = (theta_next / 2.0).cos();
                let sin_tn = (theta_next / 2.0).sin();
                
                let overlap = cos_t * cos_tn + sin_t * sin_tn * (theta - theta_next).cos();
                if overlap.abs() > 1e-10 {
                    berry_phase += (1.0 - overlap).atan2(overlap);
                }
            }

            Measurement::new(start.elapsed())
                .with_metric("berry_phase", berry_phase / PI)
                .with_metric("loop_points", loop_points as f64)
        });

        suite.add_result(result);

        // Resonance metric computation benchmark
        let bench_config = BenchmarkConfig::new(&format!("resonance_{}n", size))
            .repetitions(config.repetitions)
            .warmup(1)
            .param("graph_size", size);

        let benchmark = Benchmark::new(bench_config);

        let result = benchmark.run(|| {
            let start = Instant::now();

            let mut total_resonance = 0.0;
            let mut max_resonance = 0.0_f64;
            let mut min_resonance = 1.0_f64;

            for i in 0..size {
                for j in 0..size {
                    let psi = i as f64 / size as f64;
                    let rho = j as f64 / size as f64;
                    let omega = (i + j) as f64 / (2 * size) as f64;

                    let sig = Signature3D::new(psi, rho, omega);
                    let res = sig.weighted_sum();

                    total_resonance += res;
                    max_resonance = max_resonance.max(res);
                    min_resonance = min_resonance.min(res);
                }
            }

            let avg_resonance = total_resonance / (size * size) as f64;

            Measurement::new(start.elapsed())
                .with_metric("avg_resonance", avg_resonance)
                .with_metric("max_resonance", max_resonance)
                .with_metric("min_resonance", min_resonance)
                .with_metric("resonance_spread", max_resonance - min_resonance)
        });

        suite.add_result(result);
    }

    BenchmarkOutput::new("topology", suite)
}

// =============================================================================
// GUI Latency Benchmarks
// =============================================================================

/// GUI latency benchmark configuration
#[derive(Debug, Clone)]
pub struct GuiLatencyBenchConfig {
    pub operations: Vec<String>,
    pub repetitions: usize,
    pub small_mode: bool,
}

impl Default for GuiLatencyBenchConfig {
    fn default() -> Self {
        Self {
            operations: vec![
                "hypercube_start".to_string(),
                "cascade_run".to_string(),
                "slots_spin".to_string(),
            ],
            repetitions: 10,
            small_mode: false,
        }
    }
}

impl GuiLatencyBenchConfig {
    pub fn small() -> Self {
        Self {
            operations: vec!["hypercube_start".to_string()],
            repetitions: 3,
            small_mode: true,
        }
    }
}

/// Run GUI latency benchmarks
pub fn run_gui_latency_benchmarks(config: GuiLatencyBenchConfig) -> BenchmarkOutput {
    use qops_hypercube::{Hypercube, HypercubeConfig, CubeExpansionRule};
    use qops_slots::{SlotsSession, SlotsSessionConfig};

    let mut suite = BenchmarkSuite::new("GUI Latency Benchmark Suite")
        .description("Backend latency benchmarks for typical GUI operations");

    for op in &config.operations {
        let bench_config = BenchmarkConfig::new(&format!("gui_{}", op))
            .repetitions(config.repetitions)
            .warmup(2)
            .param("operation", op.clone());

        let benchmark = Benchmark::new(bench_config);

        let result = match op.as_str() {
            "hypercube_start" => {
                benchmark.run(|| {
                    let start = Instant::now();

                    let cube_config = HypercubeConfig {
                        max_depth: 5,
                        expansion_rule: CubeExpansionRule::Triton,
                        ..Default::default()
                    };
                    let _cube = Hypercube::new("gui_test", cube_config);

                    Measurement::new(start.elapsed())
                        .with_metric("operation_type", 1.0)
                })
            }
            "cascade_run" => {
                benchmark.run(|| {
                    let start = Instant::now();

                    let cube_config = HypercubeConfig {
                        max_depth: 3,
                        expansion_rule: CubeExpansionRule::Triton,
                        ..Default::default()
                    };
                    let mut cube = Hypercube::new("cascade_test", cube_config);
                    for _ in 0..3 {
                        let _ = cube.expand_step();
                    }

                    Measurement::new(start.elapsed())
                        .with_metric("operation_type", 2.0)
                        .with_metric("steps", 3.0)
                })
            }
            "slots_spin" => {
                benchmark.run(|| {
                    let start = Instant::now();

                    let config = SlotsSessionConfig {
                        spins_before_mine: 5,
                        ..Default::default()
                    };
                    let mut session = SlotsSession::new(config);
                    let _ = session.run();

                    Measurement::new(start.elapsed())
                        .with_metric("operation_type", 3.0)
                })
            }
            _ => {
                benchmark.run(|| Measurement::new(std::time::Duration::from_millis(1)))
            }
        };

        suite.add_result(result);
    }

    BenchmarkOutput::new("gui_latency", suite)
}

// =============================================================================
// Benchmark Runner
// =============================================================================

/// Main benchmark runner
pub struct BenchmarkRunner {
    pub output_dir: String,
    pub small_mode: bool,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self {
            output_dir: "bench_results".to_string(),
            small_mode: false,
        }
    }
}

impl BenchmarkRunner {
    pub fn new(output_dir: &str, small_mode: bool) -> Self {
        Self {
            output_dir: output_dir.to_string(),
            small_mode,
        }
    }

    pub fn run_vqe(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { VqeBenchConfig::small() } else { VqeBenchConfig::default() };
        let output = run_vqe_benchmarks(config);
        let path = Path::new(&self.output_dir).join("vqe_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_vqc(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { VqcBenchConfig::small() } else { VqcBenchConfig::default() };
        let output = run_vqc_benchmarks(config);
        let path = Path::new(&self.output_dir).join("vqc_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_qaoa(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { QaoaBenchConfig::small() } else { QaoaBenchConfig::default() };
        let output = run_qaoa_benchmarks(config);
        let path = Path::new(&self.output_dir).join("qaoa_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_qwalk(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { QWalkBenchConfig::small() } else { QWalkBenchConfig::default() };
        let output = run_qwalk_benchmarks(config);
        let path = Path::new(&self.output_dir).join("quantum_walk_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_advanced(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { AdvancedBenchConfig::small() } else { AdvancedBenchConfig::default() };
        let output = run_advanced_benchmarks(config);
        let path = Path::new(&self.output_dir).join("advanced_algorithms_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_integration(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { IntegrationBenchConfig::small() } else { IntegrationBenchConfig::default() };
        let output = run_integration_benchmarks(config);
        let path = Path::new(&self.output_dir).join("integration_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_cross(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { CrossSystemBenchConfig::small() } else { CrossSystemBenchConfig::default() };
        let output = run_cross_system_benchmarks(config);
        let path = Path::new(&self.output_dir).join("cross_system_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_hypercube(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { HypercubeBenchConfig::small() } else { HypercubeBenchConfig::default() };
        let output = run_hypercube_benchmarks(config);
        let path = Path::new(&self.output_dir).join("hypercube_cascade_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_mining(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { MiningBenchConfig::small() } else { MiningBenchConfig::default() };
        let output = run_mining_benchmarks(config);
        let path = Path::new(&self.output_dir).join("mining_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_topology(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { TopologyBenchConfig::small() } else { TopologyBenchConfig::default() };
        let output = run_topology_benchmarks(config);
        let path = Path::new(&self.output_dir).join("topology_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_gui_latency(&self) -> std::io::Result<BenchmarkOutput> {
        let config = if self.small_mode { GuiLatencyBenchConfig::small() } else { GuiLatencyBenchConfig::default() };
        let output = run_gui_latency_benchmarks(config);
        let path = Path::new(&self.output_dir).join("gui_latency_bench.json");
        output.save(&path)?;
        Ok(output)
    }

    pub fn run_all(&self) -> std::io::Result<Vec<BenchmarkOutput>> {
        let mut outputs = Vec::new();
        outputs.push(self.run_vqe()?);
        outputs.push(self.run_vqc()?);
        outputs.push(self.run_qaoa()?);
        outputs.push(self.run_qwalk()?);
        outputs.push(self.run_advanced()?);
        outputs.push(self.run_integration()?);
        outputs.push(self.run_cross()?);
        outputs.push(self.run_hypercube()?);
        outputs.push(self.run_mining()?);
        outputs.push(self.run_topology()?);
        outputs.push(self.run_gui_latency()?);
        Ok(outputs)
    }

    pub fn run_quick(&self) -> std::io::Result<Vec<BenchmarkOutput>> {
        let mut outputs = Vec::new();
        let small_runner = BenchmarkRunner::new(&self.output_dir, true);
        outputs.push(small_runner.run_vqe()?);
        outputs.push(small_runner.run_qaoa()?);
        outputs.push(small_runner.run_qwalk()?);
        outputs.push(small_runner.run_hypercube()?);
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_metadata() {
        let metadata = BenchmarkMetadata::new();
        assert!(!metadata.qops_version.is_empty());
        assert!(!metadata.benchmark_version.is_empty());
    }

    #[test]
    fn test_system_info() {
        let info = SystemInfo::current();
        assert!(!info.os.is_empty());
        assert!(!info.arch.is_empty());
        assert!(info.cpu_count > 0);
    }
}
