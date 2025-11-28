//! QOPS CLI - Unified Quantum Operator Processing System
//!
//! A comprehensive CLI for quantum algorithm research and experimentation.
//!
//! Features:
//! - Modern CLI with clap derive macros
//! - Structured logging with tracing
//! - Progress indicators
//! - Colored output

use clap::{Parser, Subcommand, Args, ValueEnum};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::time::Instant;

/// QOPS - Unified Quantum Operator Processing System
///
/// A comprehensive framework for quantum algorithm research, experimentation, and education.
#[derive(Parser)]
#[command(name = "qops")]
#[command(author = "QOPS Unified Team")]
#[command(version)]
#[command(about = "Quantum Operator Processing System - Research Framework", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Output format
    #[arg(long, global = true, default_value = "text")]
    format: OutputFormat,
}

#[derive(ValueEnum, Clone, Debug, Default)]
enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Subcommand)]
enum Commands {
    /// Display system information and capabilities
    Info,

    /// Run Genesis operator mining on S7 topology (5040 nodes)
    Genesis(GenesisArgs),

    /// Run quantum algorithms on Cube-13 topology
    Quantum(QuantumArgs),

    /// Quantum circuit simulation and manipulation
    Circuit(CircuitArgs),

    /// Classical quantum algorithms (Grover, Shor, QFT, QPE, VQE, QAOA)
    #[command(visible_alias = "algo")]
    Algorithm(AlgorithmArgs),

    /// Research tools: experiments, analysis, comparison
    Research(ResearchArgs),

    /// Run benchmarks on quantum algorithms
    #[command(visible_alias = "bench")]
    Benchmark(BenchmarkArgs),

    /// Seraphic calibration shell for configuration evolution
    Calibrate(CalibrateArgs),
}

// ============================================================================
// GENESIS COMMAND
// ============================================================================

#[derive(Args)]
struct GenesisArgs {
    /// Number of agents for swarm traversal
    #[arg(short, long, default_value_t = 5)]
    agents: usize,

    /// Steps per agent
    #[arg(short, long, default_value_t = 20)]
    steps: usize,

    /// Traversal strategy
    #[arg(long, default_value = "balanced")]
    strategy: TraversalStrategyArg,
}

#[derive(ValueEnum, Clone, Debug, Default)]
enum TraversalStrategyArg {
    #[default]
    Balanced,
    Explorative,
    Exploitative,
    Random,
}

fn run_genesis(args: GenesisArgs) {
    println!("\n{}", "Genesis Pipeline - S7 Operator Mining".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_genesis::{TraversalEngine, AgentConfig, TraversalStrategy};

    let strategy = match args.strategy {
        TraversalStrategyArg::Balanced => TraversalStrategy::Balanced,
        TraversalStrategyArg::Explorative => TraversalStrategy::Explorative,
        TraversalStrategyArg::Exploitative => TraversalStrategy::Exploitative,
        TraversalStrategyArg::Random => TraversalStrategy::Random,
    };

    let config = AgentConfig {
        max_steps: args.steps,
        strategy,
        ..Default::default()
    };

    println!("{}: {} agents, {} steps, {:?} strategy",
        "Configuration".yellow(),
        args.agents, args.steps, args.strategy);
    println!();

    let pb = ProgressBar::new(args.agents as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} agents")
        .unwrap()
        .progress_chars("#>-"));

    let mut engine = TraversalEngine::new();
    let start = Instant::now();

    let artefacts = engine.run_swarm(args.agents, config);

    for _ in 0..args.agents {
        pb.inc(1);
    }
    pb.finish_with_message("Mining complete");

    let elapsed = start.elapsed();

    println!("\n{}", "Results:".green().bold());
    println!("{}", "-".repeat(50).dimmed());

    for (i, artefact) in artefacts.iter().enumerate() {
        let status = if artefact.is_mandorla {
            "M".green()
        } else {
            "o".dimmed()
        };
        println!("  {} Artefact {:2}: resonance = {:.4}",
            status, i + 1, artefact.resonance);
    }

    if let Some(best) = engine.best_artefact() {
        println!("\n{}: resonance = {:.4}",
            "Best artefact".green().bold(), best.resonance);
    }

    println!("\n{}: {:?}", "Elapsed time".dimmed(), elapsed);
}

// ============================================================================
// QUANTUM COMMAND
// ============================================================================

#[derive(Args)]
struct QuantumArgs {
    #[command(subcommand)]
    mode: QuantumMode,
}

#[derive(Subcommand)]
enum QuantumMode {
    /// Continuous-time quantum walk on Cube-13
    Walk {
        /// Time points to evaluate
        #[arg(short, long, default_value = "0.5,1.0,2.0,5.0")]
        times: String,
    },
    /// VQE on Metatron graph
    Vqe {
        /// Number of ansatz layers
        #[arg(short, long, default_value_t = 2)]
        layers: usize,
    },
    /// QAOA MaxCut
    Qaoa {
        /// Number of QAOA layers
        #[arg(short, long, default_value_t = 3)]
        layers: usize,
    },
}

fn run_quantum(args: QuantumArgs) {
    match args.mode {
        QuantumMode::Walk { times } => run_quantum_walk(&times),
        QuantumMode::Vqe { layers } => run_legacy_vqe(layers),
        QuantumMode::Qaoa { layers } => run_legacy_qaoa(layers),
    }
}

fn run_quantum_walk(times_str: &str) {
    println!("\n{}", "Continuous-Time Quantum Walk on Cube-13".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_quantum::{MetatronGraph, MetatronHamiltonian, QuantumState};
    use qops_quantum::quantum_walk::ContinuousQuantumWalk;

    let graph = MetatronGraph::new();
    let hamiltonian = MetatronHamiltonian::from_graph(&graph);
    let qw = ContinuousQuantumWalk::new(hamiltonian);

    let initial = QuantumState::basis_state(0).unwrap();
    println!("{}: |0> (center node)", "Initial state".yellow());
    println!();

    let times: Vec<f64> = times_str.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    println!("{}", "Time evolution:".green());
    println!("{}", "-".repeat(50).dimmed());
    for t in times {
        let evolved = qw.evolve(&initial, t);
        let probs = evolved.probabilities();

        println!("  t={:5.2}: P(center)={:.3}, P(hex)={:.3}, P(cube)={:.3}",
            t, probs[0],
            probs[1..7].iter().sum::<f64>(),
            probs[7..13].iter().sum::<f64>());
    }
}

fn run_legacy_vqe(layers: usize) {
    println!("\n{}", "VQE on Metatron Graph".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_quantum::{MetatronGraph, MetatronHamiltonian};
    use qops_quantum::vqa::VQE;

    let graph = MetatronGraph::new();
    let hamiltonian = MetatronHamiltonian::from_graph(&graph);
    let vqe = VQE::new(hamiltonian, layers);

    println!("{}: {} layers", "Configuration".yellow(), layers);
    println!();

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap());
    pb.set_message("Running optimization...");

    let result = vqe.run();

    pb.finish_and_clear();

    println!("{}", "Results:".green().bold());
    println!("  Ground energy: {:.6}", result.ground_energy);
    println!("  Iterations: {}", result.iterations);
    println!("  Converged: {}", if result.converged { "Yes".green() } else { "No".red() });
}

fn run_legacy_qaoa(layers: usize) {
    println!("\n{}", "QAOA MaxCut".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_quantum::vqa::QAOA;

    let adjacency = vec![
        vec![1, 2, 3],
        vec![0, 2, 4],
        vec![0, 1, 5],
        vec![0, 4, 5],
        vec![1, 3, 5],
        vec![2, 3, 4],
    ];

    let qaoa = QAOA::new(layers);
    let result = qaoa.run_maxcut(&adjacency);

    println!("{}", "Results:".green().bold());
    println!("  Best cost: {:.2}", result.best_cost);
    println!("  Approximation ratio: {:.4}", result.approximation_ratio);
}

// ============================================================================
// CIRCUIT COMMAND
// ============================================================================

#[derive(Args)]
struct CircuitArgs {
    #[command(subcommand)]
    circuit_type: CircuitType,
}

#[derive(Subcommand)]
enum CircuitType {
    /// Create and simulate Bell state
    Bell,
    /// Create GHZ state
    Ghz {
        #[arg(short, long, default_value_t = 3)]
        qubits: usize,
    },
    /// Quantum Fourier Transform
    Qft {
        #[arg(short, long, default_value_t = 3)]
        qubits: usize,
    },
    /// Random circuit
    Random {
        #[arg(short, long, default_value_t = 3)]
        qubits: usize,
        #[arg(short, long, default_value_t = 5)]
        depth: usize,
    },
}

fn run_circuit(args: CircuitArgs) {
    match args.circuit_type {
        CircuitType::Bell => run_bell_circuit(),
        CircuitType::Ghz { qubits } => run_ghz_circuit(qubits),
        CircuitType::Qft { qubits } => run_qft_circuit(qubits),
        CircuitType::Random { qubits, depth } => run_random_circuit(qubits, depth),
    }
}

fn run_bell_circuit() {
    println!("\n{}", "Bell State Circuit".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_circuits::{Circuit, QuantumRegister, Measurement};

    let circuit = Circuit::bell_state();
    let mut reg = QuantumRegister::new(2);
    reg.apply_circuit(&circuit).unwrap();

    println!("{}: H(0) -> CNOT(0,1)", "Circuit".yellow());
    println!("{}: {}", "State".yellow(), reg);
    println!();

    let stats = Measurement::measure_all(&reg, 1000);
    println!("{} (1000 shots):", "Measurement statistics".green());
    for (outcome, count) in &stats.counts {
        let bar_len = (*count as f64 / 10.0) as usize;
        let bar = "#".repeat(bar_len.min(50));
        println!("  |{}>: {:4} {:50} ({:.1}%)",
            outcome, count, bar.cyan(), *count as f64 / 10.0);
    }
}

fn run_ghz_circuit(qubits: usize) {
    println!("\n{}", format!("GHZ State ({} qubits)", qubits).cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_circuits::{Circuit, QuantumRegister, Measurement};

    let circuit = Circuit::ghz_state(qubits);
    let mut reg = QuantumRegister::new(qubits);
    reg.apply_circuit(&circuit).unwrap();

    println!("{}: {}", "Circuit depth".yellow(), circuit.depth());
    println!("{}: {}", "Gate count".yellow(), circuit.gate_count());
    println!();

    let stats = Measurement::measure_all(&reg, 1000);
    println!("{} (1000 shots):", "Measurement statistics".green());
    let mut sorted: Vec<_> = stats.counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (outcome, count) in sorted.iter().take(5) {
        println!("  |{}>: {} ({:.1}%)", outcome, count, **count as f64 / 10.0);
    }
}

fn run_qft_circuit(qubits: usize) {
    println!("\n{}", format!("Quantum Fourier Transform ({} qubits)", qubits).cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_circuits::{Circuit, QuantumRegister, Gate};
    use qops_algorithms::QuantumFourierTransform;

    let qft = QuantumFourierTransform::new(qubits);
    let circuit = qft.build_circuit();

    println!("{}: {}", "Circuit depth".yellow(), circuit.depth());
    println!("{}: {}", "Gate count".yellow(), circuit.gate_count());

    let mut reg = QuantumRegister::new(qubits);
    reg.apply_single_gate(&Gate::x(), 0).unwrap();
    println!("\n{}: |1>", "Initial state".yellow());

    qft.apply(&mut reg).unwrap();

    println!("{}", "\nAfter QFT:".green());
    let probs = reg.state.probabilities();
    for (i, p) in probs.iter().enumerate().take(8) {
        if *p > 0.001 {
            println!("  |{:0width$b}>: {:.4}", i, p, width = qubits);
        }
    }
}

fn run_random_circuit(qubits: usize, depth: usize) {
    println!("\n{}", format!("Random Circuit ({} qubits, depth {})", qubits, depth).cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_circuits::{Circuit, Gate, QuantumRegister, Measurement};
    use rand::Rng;

    let mut circuit = Circuit::new(qubits);
    let mut rng = rand::thread_rng();

    for _ in 0..depth {
        for q in 0..qubits {
            let gate = match rng.gen_range(0..4) {
                0 => Gate::h(),
                1 => Gate::x(),
                2 => Gate::t(),
                _ => Gate::s(),
            };
            circuit.add_gate(gate, vec![q]).ok();
        }
        if qubits > 1 {
            let q1 = rng.gen_range(0..qubits);
            let q2 = (q1 + 1) % qubits;
            circuit.add_gate(Gate::cnot(), vec![q1, q2]).ok();
        }
    }

    let mut reg = QuantumRegister::new(qubits);
    reg.apply_circuit(&circuit).unwrap();

    println!("{}: {}", "Actual depth".yellow(), circuit.depth());
    println!("{}: {}", "Total gates".yellow(), circuit.gate_count());
    println!();

    let stats = Measurement::measure_all(&reg, 1000);
    println!("{}", "Measurement statistics:".green());
    let mut sorted: Vec<_> = stats.counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (outcome, count) in sorted.iter().take(5) {
        println!("  |{}>: {} ({:.1}%)", outcome, count, **count as f64 / 10.0);
    }
}

// ============================================================================
// ALGORITHM COMMAND
// ============================================================================

#[derive(Args)]
struct AlgorithmArgs {
    #[command(subcommand)]
    algorithm: AlgorithmType,
}

#[derive(Subcommand)]
enum AlgorithmType {
    /// Grover's search algorithm
    Grover {
        #[arg(short, long, default_value_t = 3)]
        qubits: usize,
        #[arg(short, long, default_value_t = 5)]
        target: usize,
        #[arg(short, long, default_value_t = 1000)]
        shots: usize,
    },
    /// Shor's factorization algorithm
    Shor {
        #[arg(short, long, default_value_t = 15)]
        number: u64,
    },
    /// Quantum Phase Estimation
    Qpe {
        #[arg(short, long, default_value_t = 4)]
        qubits: usize,
        #[arg(short, long, default_value_t = 0.25)]
        phase: f64,
    },
    /// Variational Quantum Eigensolver
    Vqe {
        #[arg(short, long, default_value_t = 2)]
        qubits: usize,
        #[arg(short, long, default_value_t = 2)]
        layers: usize,
    },
    /// QAOA - Quantum Approximate Optimization
    Qaoa {
        #[arg(short, long, default_value_t = 4)]
        qubits: usize,
        #[arg(short, long, default_value_t = 2)]
        layers: usize,
    },
}

fn run_algorithm(args: AlgorithmArgs) {
    match args.algorithm {
        AlgorithmType::Grover { qubits, target, shots } => run_grover(qubits, target, shots),
        AlgorithmType::Shor { number } => run_shor(number),
        AlgorithmType::Qpe { qubits, phase } => run_qpe(qubits, phase),
        AlgorithmType::Vqe { qubits, layers } => run_vqe(qubits, layers),
        AlgorithmType::Qaoa { qubits, layers } => run_qaoa(qubits, layers),
    }
}

fn run_grover(qubits: usize, target: usize, shots: usize) {
    println!("\n{}", "Grover's Search Algorithm".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_algorithms::{Grover, Oracle};

    println!("{}", "Configuration:".yellow());
    println!("  Qubits: {}", qubits);
    println!("  Target: |{:0width$b}> ({})", target, target, width = qubits);
    println!("  Search space: {} states", 1 << qubits);
    println!();

    let oracle = Oracle::marked_state(qubits, target);
    let grover = Grover::new(qubits, oracle);

    println!("{}: {}", "Optimal iterations".yellow(), grover.optimal_iterations());
    println!("{}: {:.2}%",
        "Theoretical success".yellow(),
        grover.theoretical_success_probability() * 100.0);
    println!();

    let pb = ProgressBar::new(shots as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len}")
        .unwrap());

    let result = grover.run_with_shots(shots);
    pb.finish_and_clear();

    println!("{}", "Results:".green().bold());
    println!("  Measured state: |{:0width$b}>", result.measured_state, width = qubits);
    println!("  Success probability: {:.2}%", result.success_probability * 100.0);
    println!("  Is solution: {}",
        if result.is_solution { "Yes".green() } else { "No".red() });

    println!("\n{}", "Top measurements:".green());
    let mut sorted: Vec<_> = result.counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (outcome, count) in sorted.iter().take(5) {
        let is_target = usize::from_str_radix(outcome, 2).unwrap_or(0) == target;
        let marker = if is_target { " <-- target".green() } else { "".normal() };
        println!("  |{}>: {} ({:.1}%){}",
            outcome, count, **count as f64 / shots as f64 * 100.0, marker);
    }
}

fn run_shor(number: u64) {
    println!("\n{}", "Shor's Factorization Algorithm".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_algorithms::{Shor, FactorizationMethod};

    println!("{}: {}", "Number to factor".yellow(), number);
    println!();

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap());
    pb.set_message("Running factorization...");

    let shor = Shor::new(number)
        .with_method(FactorizationMethod::Simulated)
        .with_max_attempts(10);

    let result = shor.run();
    pb.finish_and_clear();

    println!("{}", "Results:".green().bold());
    if result.success {
        println!("  {} Factorization successful!", "OK".green());
        let factors_str = result.factors.iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(" x ");
        println!("  Factors: {} = {}", number, factors_str.cyan());
        if let Some(period) = result.period {
            println!("  Period found: r = {}", period);
        }
        println!("  Attempts: {}", result.attempts);
    } else {
        println!("  {} Factorization failed after {} attempts", "FAIL".red(), result.attempts);
    }
}

fn run_qpe(precision: usize, phase: f64) {
    println!("\n{}", "Quantum Phase Estimation".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_algorithms::QuantumPhaseEstimation;

    println!("{}", "Configuration:".yellow());
    println!("  Precision qubits: {}", precision);
    println!("  True phase: {:.4} (= {:.4}pi)", phase, phase * 2.0);
    println!();

    let qpe = QuantumPhaseEstimation::for_gate(precision, &qops_circuits::Gate::t()).unwrap();
    let result = qpe.estimate_known_phase(phase, 1000);

    println!("{}", "Results:".green().bold());
    println!("  Estimated phase: {:.4}", result.phase);
    println!("  Error: {:.6}", (result.phase - phase).abs());
    println!("  Error bound: {:.6}", result.error_bound());
    println!("  Confidence: {:.1}%", result.confidence * 100.0);
}

fn run_vqe(qubits: usize, layers: usize) {
    println!("\n{}", "Variational Quantum Eigensolver".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_algorithms::{VQE, VQEConfig, vqe::PauliSum, Ansatz};

    println!("{}", "Configuration:".yellow());
    println!("  Qubits: {}", qubits);
    println!("  Ansatz layers: {}", layers);
    println!();

    let hamiltonian = PauliSum::transverse_ising(qubits, 1.0, 0.5);
    let config = VQEConfig {
        num_qubits: qubits,
        ansatz: Ansatz::RealAmplitudes,
        layers,
        max_iterations: 50,
        ..Default::default()
    };

    let vqe = VQE::new(config, hamiltonian);
    println!("{}: {}", "Parameters".yellow(), vqe.num_parameters());

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} Running optimization...")
        .unwrap());

    let result = vqe.run();
    pb.finish_and_clear();

    println!("\n{}", "Results:".green().bold());
    println!("  Ground energy: {:.6}", result.energy);
    println!("  Evaluations: {}", result.num_evaluations);
    println!("  Converged: {}", if result.converged { "Yes".green() } else { "No".red() });
    println!("  Final variance: {:.6}", result.variance);
}

fn run_qaoa(qubits: usize, layers: usize) {
    println!("\n{}", "QAOA - Quantum Approximate Optimization".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_algorithms::{QAOA, QAOAConfig};

    println!("{}", "Configuration:".yellow());
    println!("  Qubits: {}", qubits);
    println!("  Layers (p): {}", layers);
    println!();

    // Create ring graph
    let edges: Vec<(usize, usize)> = (0..qubits)
        .map(|i| (i, (i + 1) % qubits))
        .collect();

    println!("{}: MaxCut on {}-node ring graph", "Problem".yellow(), qubits);
    println!("  Edges: {:?}", edges);
    println!();

    let qaoa = QAOA::max_cut(edges, layers);

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} Running optimization...")
        .unwrap());

    let result = qaoa.run();
    pb.finish_and_clear();

    println!("{}", "Results:".green().bold());
    println!("  Best solution: {:?}", result.best_solution);
    println!("  Cut value: {:.0}", result.best_cost);

    println!("\n{}", "Top solutions:".green());
    let mut sorted: Vec<_> = result.solution_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (solution, count) in sorted.iter().take(5) {
        println!("  {}: {} times", solution, count);
    }
}

// ============================================================================
// RESEARCH COMMAND
// ============================================================================

#[derive(Args)]
struct ResearchArgs {
    #[command(subcommand)]
    mode: ResearchMode,
}

#[derive(Subcommand)]
enum ResearchMode {
    /// Run structured experiments
    Experiment {
        /// Experiment name
        #[arg(short, long, default_value = "grover_scaling")]
        name: String,
    },
    /// Analyze experiment results
    Analyze,
    /// Compare algorithm performance
    Compare,
}

fn run_research(args: ResearchArgs) {
    match args.mode {
        ResearchMode::Experiment { name } => run_experiment(&name),
        ResearchMode::Analyze => run_analyze(),
        ResearchMode::Compare => run_compare(),
    }
}

fn run_experiment(name: &str) {
    println!("\n{}", "Running Experiment".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_research::{Experiment, Parameter};
    use std::collections::HashMap;

    let experiment = Experiment::new(name)
        .description("Grover algorithm scaling with problem size")
        .parameter(Parameter::new("qubits", vec![2, 3, 4, 5]))
        .repetitions(3);

    println!("{}: {}", "Experiment".yellow(), name);
    println!("{}: {}", "Total runs".yellow(), 4 * 3);
    println!();

    let result = experiment.run(|params| {
        let qubits = params.get("qubits")
            .and_then(|v| v.as_i64())
            .unwrap_or(2) as usize;

        use qops_algorithms::{Grover, Oracle};

        let oracle = Oracle::marked_state(qubits, 0);
        let grover = Grover::new(qubits, oracle);
        let grover_result = grover.run_with_shots(100);

        let mut data = HashMap::new();
        data.insert("success_prob".to_string(), serde_json::json!(grover_result.success_probability));
        data.insert("iterations".to_string(), serde_json::json!(grover_result.iterations));
        data
    });

    println!("{}", "Results:".green().bold());
    println!("{}", "-".repeat(50).dimmed());
    let means = result.mean_by_param("success_prob", "qubits");
    for (qubits, mean) in means {
        println!("  {} qubits: {:.2}% success", qubits.trim_matches('"'), mean * 100.0);
    }
}

fn run_analyze() {
    println!("\n{}", "Analysis Tools".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());
    println!("Run an experiment first, then analyze the results.");
    println!("Use: qops research experiment");
}

fn run_compare() {
    println!("\n{}", "Algorithm Comparison".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_research::Comparison;

    let algo_a = vec![1.2, 1.3, 1.1, 1.4, 1.2];
    let algo_b = vec![0.8, 0.9, 0.7, 0.85, 0.9];

    let result = Comparison::compare("Algorithm A", &algo_a, "Algorithm B", &algo_b, "execution_time")
        .unwrap();

    println!("{}", result.summary());
}

// ============================================================================
// BENCHMARK COMMAND
// ============================================================================

#[derive(Args)]
struct BenchmarkArgs {
    #[command(subcommand)]
    benchmark: BenchmarkType,
}

#[derive(Subcommand)]
enum BenchmarkType {
    /// Benchmark Grover's algorithm
    Grover {
        /// Qubit counts (comma-separated)
        #[arg(short, long, default_value = "2,3,4")]
        qubits: String,
    },
    /// Benchmark QFT
    Qft {
        /// Qubit counts (comma-separated)
        #[arg(short, long, default_value = "2,3,4,5")]
        qubits: String,
    },
    /// Benchmark state vector simulation
    Simulation {
        /// Qubit counts (comma-separated)
        #[arg(short, long, default_value = "4,6,8,10")]
        qubits: String,
    },
}

fn run_benchmark(args: BenchmarkArgs) {
    match args.benchmark {
        BenchmarkType::Grover { qubits } => benchmark_grover(&qubits),
        BenchmarkType::Qft { qubits } => benchmark_qft(&qubits),
        BenchmarkType::Simulation { qubits } => benchmark_simulation(&qubits),
    }
}

fn benchmark_grover(qubits_str: &str) {
    println!("\n{}", "Grover Algorithm Benchmark".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_research::benchmark::quantum_benchmarks;

    let qubits: Vec<usize> = qubits_str.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let suite = quantum_benchmarks::grover_scaling(&qubits, 100);
    println!("{}", suite.comparison_table());
}

fn benchmark_qft(qubits_str: &str) {
    println!("\n{}", "QFT Benchmark".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_research::benchmark::quantum_benchmarks;

    let qubits: Vec<usize> = qubits_str.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let suite = quantum_benchmarks::qft_scaling(&qubits);
    println!("{}", suite.comparison_table());
}

fn benchmark_simulation(qubits_str: &str) {
    println!("\n{}", "State Vector Simulation Benchmark".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_research::benchmark::quantum_benchmarks;

    let qubits: Vec<usize> = qubits_str.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let suite = quantum_benchmarks::simulation_scaling(&qubits, 10);
    println!("{}", suite.comparison_table());
}

// ============================================================================
// CALIBRATE COMMAND
// ============================================================================

#[derive(Args)]
struct CalibrateArgs {
    /// Number of calibration steps
    #[arg(short, long, default_value_t = 10)]
    steps: usize,

    /// Target resonance value
    #[arg(short, long, default_value_t = 0.85)]
    target: f64,
}

fn run_calibrate(args: CalibrateArgs) {
    println!("\n{}", "Seraphic Calibration Shell".cyan().bold());
    println!("{}\n", "=".repeat(50).dimmed());

    use qops_seraphic::SeraphicCalibrator;
    use qops_core::Signature3D;

    let mut calibrator = SeraphicCalibrator::default();
    calibrator.initialize(
        qops_core::Configuration::new("default"),
        Signature3D::new(0.5, 0.5, 0.5),
    );

    println!("{}: {} steps, target resonance = {:.2}",
        "Configuration".yellow(), args.steps, args.target);
    println!();

    let pb = ProgressBar::new(args.steps as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} steps")
        .unwrap());

    let results = calibrator.run(args.steps);

    for _ in &results {
        pb.inc(1);
    }
    pb.finish_and_clear();

    println!("{}", "Step |   psi  |   rho  |  omega | Accept | CRI".dimmed());
    println!("{}", "-----+--------+--------+--------+--------+-----".dimmed());
    for result in &results {
        let accept = if result.accepted { "Y".green() } else { " ".normal() };
        let cri = if result.cri_triggered { "!".yellow() } else { " ".normal() };
        println!(" {:3} | {:.4} | {:.4} | {:.4} |   {}    |  {}",
            result.step,
            result.performance.psi,
            result.performance.rho,
            result.performance.omega,
            accept,
            cri);
    }

    let final_perf = calibrator.current_performance();
    println!();
    println!("{}: psi={:.4} rho={:.4} omega={:.4}",
        "Final".green().bold(),
        final_perf.psi, final_perf.rho, final_perf.omega);
}

// ============================================================================
// INFO COMMAND
// ============================================================================

fn print_info() {
    println!("\n{}", "QOPS System Information".cyan().bold());
    println!("{}\n", "=".repeat(70).dimmed());

    println!("{}: {}", "Version".yellow(), env!("CARGO_PKG_VERSION"));
    println!();

    println!("{}", "MODULES:".yellow().bold());
    println!("{}", "-".repeat(70).dimmed());
    println!("  {} | Shared types, signatures, resonance framework", "Core      ".cyan());
    println!("  {} | S7 topology operator mining (5040 permutation nodes)", "Genesis   ".cyan());
    println!("  {} | Cube-13 quantum algorithms (VQE, QAOA, quantum walks)", "Quantum   ".cyan());
    println!("  {} | Quantum circuit simulator (gates, registers, measurement)", "Circuits  ".cyan());
    println!("  {} | Classical algorithms (Grover, Shor, QFT, QPE, VQE, QAOA)", "Algorithms".cyan());
    println!("  {} | Benchmarking, experiments, analysis, visualization", "Research  ".cyan());
    println!("  {} | Meta-algorithm for fixpoint-directed calibration", "Seraphic  ".cyan());
    println!("  {} | Bridges between Genesis and Quantum pipelines", "Adapters  ".cyan());
    println!();

    println!("{}", "TOPOLOGY:".yellow().bold());
    println!("{}", "-".repeat(70).dimmed());
    println!("  Genesis Pipeline: S7 permutation group (7! = 5040 nodes)");
    println!("  Quantum Pipeline: Metatron Cube-13 (1 center + 6 hexagon + 6 cube)");
    println!();

    println!("{}", "CAPABILITIES:".yellow().bold());
    println!("{}", "-".repeat(70).dimmed());
    let caps = vec![
        "Universal quantum gate set (H, X, Y, Z, CNOT, Toffoli, etc.)",
        "State vector simulation (up to ~20 qubits)",
        "Grover's search algorithm",
        "Shor's factorization algorithm",
        "Quantum Fourier Transform (QFT)",
        "Quantum Phase Estimation (QPE)",
        "Variational Quantum Eigensolver (VQE)",
        "QAOA for combinatorial optimization",
        "Hamiltonian simulation (Trotter decomposition)",
        "Noise models (depolarizing, amplitude damping)",
        "Benchmarking and performance analysis",
        "Experiment framework with reproducibility",
    ];
    for cap in caps {
        println!("  {} {}", "OK".green(), cap);
    }
    println!();

    println!("{}: MIT", "License".yellow());
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    let cli = Cli::parse();

    // Initialize tracing
    if cli.verbose {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .with_target(true)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");
        info!("Verbose mode enabled");
    }

    match cli.command {
        Commands::Info => print_info(),
        Commands::Genesis(args) => run_genesis(args),
        Commands::Quantum(args) => run_quantum(args),
        Commands::Circuit(args) => run_circuit(args),
        Commands::Algorithm(args) => run_algorithm(args),
        Commands::Research(args) => run_research(args),
        Commands::Benchmark(args) => run_benchmark(args),
        Commands::Calibrate(args) => run_calibrate(args),
    }
}
