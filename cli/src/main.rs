//! QOPS CLI - Unified Quantum Operator Processing System
//!
//! A comprehensive CLI for quantum algorithm research and experimentation.
//!
//! Usage:
//!   qops <command> [options]
//!
//! Commands:
//!   genesis     - Run Genesis operator mining (S7 topology)
//!   quantum     - Run quantum algorithms (Cube-13)
//!   circuit     - Quantum circuit simulation
//!   algorithm   - Classical quantum algorithms (Grover, Shor, QFT, etc.)
//!   research    - Research tools (benchmarks, experiments)
//!   calibrate   - Seraphic calibration shell
//!   info        - System information

use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "genesis" => run_genesis(&args[2..]),
        "quantum" => run_quantum(&args[2..]),
        "circuit" => run_circuit(&args[2..]),
        "algorithm" | "algo" => run_algorithm(&args[2..]),
        "research" => run_research(&args[2..]),
        "benchmark" | "bench" => run_benchmark(&args[2..]),
        "calibrate" => run_calibrate(&args[2..]),
        "info" => print_info(),
        "help" | "--help" | "-h" => print_usage(),
        "version" | "--version" | "-v" => print_version(),
        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}

fn print_version() {
    println!("QOPS v{}", env!("CARGO_PKG_VERSION"));
}

fn print_usage() {
    println!(r#"
╔═══════════════════════════════════════════════════════════════════════════════╗
║            QOPS - Unified Quantum Operator Processing System                  ║
║                   Quantum Research Framework v{}                          ║
╚═══════════════════════════════════════════════════════════════════════════════╝

USAGE:
    qops <COMMAND> [OPTIONS]

COMMANDS:
    genesis       Run Genesis operator mining on S7 topology (5040 nodes)
    quantum       Run quantum algorithms on Cube-13 topology
    circuit       Quantum circuit simulation and manipulation
    algorithm     Classical quantum algorithms (Grover, Shor, QFT, QPE, VQE, QAOA)
    research      Research tools: experiments, analysis, comparison
    benchmark     Run benchmarks on quantum algorithms
    calibrate     Seraphic calibration shell for configuration evolution
    info          Display system information and capabilities
    help          Show this help message

EXAMPLES:
    qops genesis --agents 10 --steps 50
    qops algorithm grover --qubits 4 --target 5
    qops algorithm shor --number 15
    qops circuit bell --qubits 2
    qops benchmark qft --qubits 2,3,4,5
    qops research experiment --name scaling

For more information on a specific command, run:
    qops <COMMAND> --help
"#, env!("CARGO_PKG_VERSION"));
}

fn print_info() {
    println!(r#"
╔═══════════════════════════════════════════════════════════════════════════════╗
║                     QOPS System Information                                   ║
╚═══════════════════════════════════════════════════════════════════════════════╝

VERSION: {}

MODULES:
  ┌─────────────────────────────────────────────────────────────────────────────┐
  │ Core       │ Shared types, signatures, resonance framework                  │
  │ Genesis    │ S7 topology operator mining (5040 permutation nodes)           │
  │ Quantum    │ Cube-13 quantum algorithms (VQE, QAOA, quantum walks)          │
  │ Circuits   │ Quantum circuit simulator (gates, registers, measurement)      │
  │ Algorithms │ Classical algorithms (Grover, Shor, QFT, QPE, VQE, QAOA)       │
  │ Research   │ Benchmarking, experiments, analysis, visualization             │
  │ Seraphic   │ Meta-algorithm for fixpoint-directed calibration               │
  │ Adapters   │ Bridges between Genesis and Quantum pipelines                  │
  └─────────────────────────────────────────────────────────────────────────────┘

TOPOLOGY:
  • Genesis Pipeline: S7 permutation group (7! = 5040 nodes)
  • Quantum Pipeline: Metatron Cube-13 (1 center + 6 hexagon + 6 cube)

CAPABILITIES:
  ✓ Universal quantum gate set (H, X, Y, Z, CNOT, Toffoli, etc.)
  ✓ State vector simulation (up to ~20 qubits)
  ✓ Grover's search algorithm
  ✓ Shor's factorization algorithm
  ✓ Quantum Fourier Transform (QFT)
  ✓ Quantum Phase Estimation (QPE)
  ✓ Variational Quantum Eigensolver (VQE)
  ✓ QAOA for combinatorial optimization
  ✓ Hamiltonian simulation (Trotter decomposition)
  ✓ Noise models (depolarizing, amplitude damping)
  ✓ Benchmarking and performance analysis
  ✓ Experiment framework with reproducibility

LICENSE: MIT
"#, env!("CARGO_PKG_VERSION"));
}

// ============================================================================
// GENESIS COMMAND
// ============================================================================

fn run_genesis(args: &[String]) {
    println!("\n🔮 Genesis Pipeline - S7 Operator Mining\n");

    let mut agents = 5;
    let mut steps = 20;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--agents" | "-a" => {
                if i + 1 < args.len() {
                    agents = args[i + 1].parse().unwrap_or(5);
                    i += 1;
                }
            }
            "--steps" | "-s" => {
                if i + 1 < args.len() {
                    steps = args[i + 1].parse().unwrap_or(20);
                    i += 1;
                }
            }
            "--help" | "-h" => {
                println!("Usage: qops genesis [OPTIONS]");
                println!();
                println!("Options:");
                println!("  --agents, -a <N>   Number of agents (default: 5)");
                println!("  --steps, -s <N>    Steps per agent (default: 20)");
                return;
            }
            _ => {}
        }
        i += 1;
    }

    println!("Configuration: {} agents, {} steps each", agents, steps);
    println!();

    use qops_genesis::{TraversalEngine, AgentConfig, TraversalStrategy};

    let mut engine = TraversalEngine::new();
    let config = AgentConfig {
        max_steps: steps,
        strategy: TraversalStrategy::Balanced,
        ..Default::default()
    };

    println!("Mining operators on S7 topology...\n");
    let artefacts = engine.run_swarm(agents, config);

    println!("Results:");
    println!("─────────────────────────────────────────");
    for (i, artefact) in artefacts.iter().enumerate() {
        let status = if artefact.is_mandorla { "✓" } else { "○" };
        println!(
            "  {} Artefact {:2}: resonance = {:.4}",
            status, i + 1, artefact.resonance
        );
    }

    if let Some(best) = engine.best_artefact() {
        println!();
        println!("Best artefact: resonance = {:.4}", best.resonance);
    }
}

// ============================================================================
// QUANTUM COMMAND (Legacy Cube-13)
// ============================================================================

fn run_quantum(args: &[String]) {
    let mode = args.first().map(|s| s.as_str()).unwrap_or("walk");

    match mode {
        "--walk" | "walk" => run_quantum_walk(),
        "--vqe" | "vqe" => run_legacy_vqe(),
        "--qaoa" | "qaoa" => run_legacy_qaoa(),
        "--help" | "-h" => {
            println!("Usage: qops quantum <MODE>");
            println!();
            println!("Modes:");
            println!("  walk   Continuous-time quantum walk on Cube-13");
            println!("  vqe    VQE on Metatron graph");
            println!("  qaoa   QAOA MaxCut");
        }
        _ => {
            println!("Unknown mode: {}. Use: walk, vqe, qaoa", mode);
        }
    }
}

fn run_quantum_walk() {
    println!("\n🌀 Continuous-Time Quantum Walk on Cube-13\n");

    use qops_quantum::{MetatronGraph, MetatronHamiltonian, QuantumState};
    use qops_quantum::quantum_walk::ContinuousQuantumWalk;

    let graph = MetatronGraph::new();
    let hamiltonian = MetatronHamiltonian::from_graph(&graph);
    let qw = ContinuousQuantumWalk::new(hamiltonian);

    let initial = QuantumState::basis_state(0).unwrap();
    println!("Initial state: |0⟩ (center node)");
    println!();

    println!("Time evolution:");
    println!("─────────────────────────────────────────");
    for t in [0.5, 1.0, 2.0, 5.0] {
        let evolved = qw.evolve(&initial, t);
        let probs = evolved.probabilities();

        println!(
            "  t={:.1}: P(center)={:.3}, P(hex)={:.3}, P(cube)={:.3}",
            t,
            probs[0],
            probs[1..7].iter().sum::<f64>(),
            probs[7..13].iter().sum::<f64>()
        );
    }
}

fn run_legacy_vqe() {
    println!("\n⚛️ VQE on Metatron Graph\n");

    use qops_quantum::{MetatronGraph, MetatronHamiltonian};
    use qops_quantum::vqa::VQE;

    let graph = MetatronGraph::new();
    let hamiltonian = MetatronHamiltonian::from_graph(&graph);
    let vqe = VQE::new(hamiltonian, 2);

    println!("Running optimization...");
    let result = vqe.run();

    println!();
    println!("Results:");
    println!("  Ground energy: {:.6}", result.ground_energy);
    println!("  Iterations: {}", result.iterations);
    println!("  Converged: {}", result.converged);
}

fn run_legacy_qaoa() {
    println!("\n🔄 QAOA MaxCut\n");

    use qops_quantum::vqa::QAOA;

    let adjacency = vec![
        vec![1, 2, 3],
        vec![0, 2, 4],
        vec![0, 1, 5],
        vec![0, 4, 5],
        vec![1, 3, 5],
        vec![2, 3, 4],
    ];

    let qaoa = QAOA::new(3);
    let result = qaoa.run_maxcut(&adjacency);

    println!("Results:");
    println!("  Best cost: {:.2}", result.best_cost);
    println!("  Approximation ratio: {:.4}", result.approximation_ratio);
}

// ============================================================================
// CIRCUIT COMMAND
// ============================================================================

fn run_circuit(args: &[String]) {
    let mode = args.first().map(|s| s.as_str()).unwrap_or("bell");

    match mode {
        "bell" => run_bell_circuit(),
        "ghz" => run_ghz_circuit(args),
        "qft" => run_qft_circuit(args),
        "random" => run_random_circuit(args),
        "--help" | "-h" => {
            println!("Usage: qops circuit <TYPE> [OPTIONS]");
            println!();
            println!("Types:");
            println!("  bell             Create and simulate Bell state");
            println!("  ghz --qubits N   Create GHZ state");
            println!("  qft --qubits N   Quantum Fourier Transform");
            println!("  random --qubits N --depth D   Random circuit");
        }
        _ => {
            println!("Unknown circuit type: {}. Use --help for options.", mode);
        }
    }
}

fn run_bell_circuit() {
    println!("\n🔔 Bell State Circuit\n");

    use qops_circuits::{Circuit, QuantumRegister, Measurement};

    let circuit = Circuit::bell_state();
    let mut reg = QuantumRegister::new(2);
    reg.apply_circuit(&circuit).unwrap();

    println!("Circuit: H(0) → CNOT(0,1)");
    println!("State: {}", reg);
    println!();

    let stats = Measurement::measure_all(&reg, 1000);
    println!("Measurement statistics (1000 shots):");
    for (outcome, count) in &stats.counts {
        println!("  |{}⟩: {} ({:.1}%)", outcome, count, *count as f64 / 10.0);
    }
}

fn run_ghz_circuit(args: &[String]) {
    let qubits = parse_arg(args, "--qubits", 3);

    println!("\n🌟 GHZ State ({} qubits)\n", qubits);

    use qops_circuits::{Circuit, QuantumRegister, Measurement};

    let circuit = Circuit::ghz_state(qubits);
    let mut reg = QuantumRegister::new(qubits);
    reg.apply_circuit(&circuit).unwrap();

    println!("Circuit depth: {}", circuit.depth());
    println!("Gate count: {}", circuit.gate_count());
    println!();

    let stats = Measurement::measure_all(&reg, 1000);
    println!("Measurement statistics (1000 shots):");
    let mut sorted: Vec<_> = stats.counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (outcome, count) in sorted.iter().take(5) {
        println!("  |{}⟩: {} ({:.1}%)", outcome, count, **count as f64 / 10.0);
    }
}

fn run_qft_circuit(args: &[String]) {
    let qubits = parse_arg(args, "--qubits", 3);

    println!("\n📊 Quantum Fourier Transform ({} qubits)\n", qubits);

    use qops_circuits::{Circuit, QuantumRegister};
    use qops_algorithms::QuantumFourierTransform;

    let qft = QuantumFourierTransform::new(qubits);
    let circuit = qft.build_circuit();

    println!("Circuit depth: {}", circuit.depth());
    println!("Gate count: {}", circuit.gate_count());

    let mut reg = QuantumRegister::new(qubits);
    // Start with |1⟩ state
    reg.apply_single_gate(&qops_circuits::Gate::x(), 0).unwrap();
    println!("\nInitial state: |1⟩");

    qft.apply(&mut reg).unwrap();

    println!("After QFT:");
    let probs = reg.state.probabilities();
    for (i, p) in probs.iter().enumerate().take(8) {
        if *p > 0.001 {
            println!("  |{:0width$b}⟩: {:.4}", i, p, width = qubits);
        }
    }
}

fn run_random_circuit(args: &[String]) {
    let qubits = parse_arg(args, "--qubits", 3);
    let depth = parse_arg(args, "--depth", 5);

    println!("\n🎲 Random Circuit ({} qubits, depth {})\n", qubits, depth);

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

    println!("Generated circuit:");
    println!("  Depth: {}", circuit.depth());
    println!("  Gates: {}", circuit.gate_count());
    println!();

    let stats = Measurement::measure_all(&reg, 1000);
    println!("Measurement statistics:");
    let mut sorted: Vec<_> = stats.counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (outcome, count) in sorted.iter().take(5) {
        println!("  |{}⟩: {} ({:.1}%)", outcome, count, **count as f64 / 10.0);
    }
}

// ============================================================================
// ALGORITHM COMMAND
// ============================================================================

fn run_algorithm(args: &[String]) {
    let algo = args.first().map(|s| s.as_str()).unwrap_or("help");

    match algo {
        "grover" => run_grover(args),
        "shor" => run_shor(args),
        "qft" => run_qft_circuit(args),
        "qpe" => run_qpe(args),
        "vqe" => run_vqe(args),
        "qaoa" => run_qaoa(args),
        "--help" | "-h" | "help" => {
            println!("Usage: qops algorithm <ALGORITHM> [OPTIONS]");
            println!();
            println!("Algorithms:");
            println!("  grover  --qubits N --target T   Grover's search");
            println!("  shor    --number N              Shor's factorization");
            println!("  qft     --qubits N              Quantum Fourier Transform");
            println!("  qpe     --qubits N --phase P    Quantum Phase Estimation");
            println!("  vqe     --qubits N              Variational Quantum Eigensolver");
            println!("  qaoa    --qubits N --layers L   QAOA MaxCut");
        }
        _ => {
            println!("Unknown algorithm: {}. Use --help for options.", algo);
        }
    }
}

fn run_grover(args: &[String]) {
    let qubits = parse_arg(args, "--qubits", 3);
    let target = parse_arg(args, "--target", 5);
    let shots = parse_arg(args, "--shots", 1000);

    println!("\n🔍 Grover's Search Algorithm\n");
    println!("Configuration:");
    println!("  Qubits: {}", qubits);
    println!("  Target: |{:0width$b}⟩ ({})", target, target, width = qubits);
    println!("  Search space: {} states", 1 << qubits);
    println!();

    use qops_algorithms::{Grover, Oracle};

    let oracle = Oracle::marked_state(qubits, target);
    let grover = Grover::new(qubits, oracle);

    println!("Optimal iterations: {}", grover.optimal_iterations());
    println!("Theoretical success probability: {:.2}%",
        grover.theoretical_success_probability() * 100.0);
    println!();

    println!("Running {} measurement shots...", shots);
    let result = grover.run_with_shots(shots);

    println!();
    println!("Results:");
    println!("  Measured state: |{:0width$b}⟩", result.measured_state, width = qubits);
    println!("  Success probability: {:.2}%", result.success_probability * 100.0);
    println!("  Is solution: {}", if result.is_solution { "✓ Yes" } else { "✗ No" });

    println!();
    println!("Top measurements:");
    let mut sorted: Vec<_> = result.counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (outcome, count) in sorted.iter().take(5) {
        let marker = if usize::from_str_radix(outcome, 2).unwrap_or(0) == target { "←" } else { "" };
        println!("  |{}⟩: {} ({:.1}%) {}", outcome, count, **count as f64 / shots as f64 * 100.0, marker);
    }
}

fn run_shor(args: &[String]) {
    let number = parse_arg(args, "--number", 15) as u64;

    println!("\n🔢 Shor's Factorization Algorithm\n");
    println!("Number to factor: {}", number);
    println!();

    use qops_algorithms::{Shor, FactorizationMethod};

    let shor = Shor::new(number)
        .with_method(FactorizationMethod::Simulated)
        .with_max_attempts(10);

    println!("Running factorization...");
    let result = shor.run();

    println!();
    println!("Results:");
    if result.success {
        println!("  ✓ Factorization successful!");
        println!("  Factors: {} = {}", number, result.factors.iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(" × "));
        if let Some(period) = result.period {
            println!("  Period found: r = {}", period);
        }
        println!("  Attempts: {}", result.attempts);
    } else {
        println!("  ✗ Factorization failed after {} attempts", result.attempts);
    }
}

fn run_qpe(args: &[String]) {
    let precision = parse_arg(args, "--qubits", 4);
    let phase: f64 = args.iter()
        .position(|a| a == "--phase")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.25);

    println!("\n📐 Quantum Phase Estimation\n");
    println!("Configuration:");
    println!("  Precision qubits: {}", precision);
    println!("  True phase: {:.4} (= {:.4}π)", phase, phase * 2.0);
    println!();

    use qops_algorithms::QuantumPhaseEstimation;

    let qpe = QuantumPhaseEstimation::for_gate(precision, &qops_circuits::Gate::t()).unwrap();
    let result = qpe.estimate_known_phase(phase, 1000);

    println!("Results:");
    println!("  Estimated phase: {:.4}", result.phase);
    println!("  Error: {:.6}", (result.phase - phase).abs());
    println!("  Error bound: {:.6}", result.error_bound());
    println!("  Confidence: {:.1}%", result.confidence * 100.0);
}

fn run_vqe(args: &[String]) {
    let qubits = parse_arg(args, "--qubits", 2);
    let layers = parse_arg(args, "--layers", 2);

    println!("\n⚗️ Variational Quantum Eigensolver\n");
    println!("Configuration:");
    println!("  Qubits: {}", qubits);
    println!("  Ansatz layers: {}", layers);
    println!();

    use qops_algorithms::{VQE, VQEConfig, vqe::PauliSum, Ansatz};

    let hamiltonian = PauliSum::transverse_ising(qubits, 1.0, 0.5);

    let config = VQEConfig {
        num_qubits: qubits,
        ansatz: Ansatz::RealAmplitudes,
        layers,
        max_iterations: 50,
        ..Default::default()
    };

    let vqe = VQE::new(config, hamiltonian);
    println!("Parameters: {}", vqe.num_parameters());
    println!("Running optimization...\n");

    let result = vqe.run();

    println!("Results:");
    println!("  Ground energy: {:.6}", result.energy);
    println!("  Evaluations: {}", result.num_evaluations);
    println!("  Converged: {}", result.converged);
    println!("  Final variance: {:.6}", result.variance);
}

fn run_qaoa(args: &[String]) {
    let qubits = parse_arg(args, "--qubits", 4);
    let layers = parse_arg(args, "--layers", 2);

    println!("\n🔄 QAOA - Quantum Approximate Optimization\n");
    println!("Configuration:");
    println!("  Qubits: {}", qubits);
    println!("  Layers (p): {}", layers);
    println!();

    use qops_algorithms::{QAOA, QAOAConfig};

    // Create simple ring graph
    let edges: Vec<(usize, usize)> = (0..qubits)
        .map(|i| (i, (i + 1) % qubits))
        .collect();

    println!("Problem: MaxCut on {}-node ring graph", qubits);
    println!("Edges: {:?}", edges);
    println!();

    let qaoa = QAOA::max_cut(edges, layers);
    println!("Running optimization...\n");

    let result = qaoa.run();

    println!("Results:");
    println!("  Best solution: {:?}", result.best_solution);
    println!("  Cut value: {:.0}", result.best_cost);

    println!();
    println!("Top solutions:");
    let mut sorted: Vec<_> = result.solution_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (solution, count) in sorted.iter().take(5) {
        println!("  {}: {} times", solution, count);
    }
}

// ============================================================================
// RESEARCH COMMAND
// ============================================================================

fn run_research(args: &[String]) {
    let mode = args.first().map(|s| s.as_str()).unwrap_or("help");

    match mode {
        "experiment" => run_experiment(args),
        "analyze" => run_analyze(args),
        "compare" => run_compare(args),
        "--help" | "-h" | "help" => {
            println!("Usage: qops research <MODE> [OPTIONS]");
            println!();
            println!("Modes:");
            println!("  experiment  Run structured experiments");
            println!("  analyze     Analyze experiment results");
            println!("  compare     Compare algorithm performance");
        }
        _ => {
            println!("Unknown mode: {}. Use --help.", mode);
        }
    }
}

fn run_experiment(args: &[String]) {
    println!("\n📊 Running Experiment\n");

    use qops_research::{Experiment, Parameter};
    use std::collections::HashMap;

    let experiment = Experiment::new("grover_scaling")
        .description("Grover algorithm scaling with problem size")
        .parameter(Parameter::new("qubits", vec![2, 3, 4, 5]))
        .repetitions(3);

    println!("Experiment: Grover Scaling Study");
    println!("Total runs: {}", 4 * 3);
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

    println!("Results:");
    println!("─────────────────────────────────────────");
    let means = result.mean_by_param("success_prob", "qubits");
    for (qubits, mean) in means {
        println!("  {} qubits: {:.2}% success", qubits.trim_matches('"'), mean * 100.0);
    }
}

fn run_analyze(_args: &[String]) {
    println!("\n📈 Analysis Tools\n");
    println!("Run an experiment first, then analyze the results.");
    println!("Use: qops research experiment");
}

fn run_compare(_args: &[String]) {
    println!("\n⚖️ Algorithm Comparison\n");

    use qops_research::Comparison;

    // Compare random data for demonstration
    let algo_a = vec![1.2, 1.3, 1.1, 1.4, 1.2];
    let algo_b = vec![0.8, 0.9, 0.7, 0.85, 0.9];

    let result = Comparison::compare("Algorithm A", &algo_a, "Algorithm B", &algo_b, "execution_time")
        .unwrap();

    println!("{}", result.summary());
}

// ============================================================================
// BENCHMARK COMMAND
// ============================================================================

fn run_benchmark(args: &[String]) {
    let algo = args.first().map(|s| s.as_str()).unwrap_or("help");

    match algo {
        "grover" => benchmark_grover(args),
        "qft" => benchmark_qft(args),
        "simulation" => benchmark_simulation(args),
        "--help" | "-h" | "help" => {
            println!("Usage: qops benchmark <ALGORITHM> [OPTIONS]");
            println!();
            println!("Algorithms:");
            println!("  grover      --qubits 2,3,4,5    Benchmark Grover's algorithm");
            println!("  qft         --qubits 2,3,4,5    Benchmark QFT");
            println!("  simulation  --qubits 2,3,4,5    Benchmark state vector simulation");
        }
        _ => {
            println!("Unknown benchmark: {}. Use --help.", algo);
        }
    }
}

fn benchmark_grover(args: &[String]) {
    let qubits_str = args.iter()
        .position(|a| a == "--qubits")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("2,3,4");

    let qubits: Vec<usize> = qubits_str.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    println!("\n⏱️ Grover Algorithm Benchmark\n");

    use qops_research::benchmark::quantum_benchmarks;

    let suite = quantum_benchmarks::grover_scaling(&qubits, 100);
    println!("{}", suite.comparison_table());
}

fn benchmark_qft(args: &[String]) {
    let qubits_str = args.iter()
        .position(|a| a == "--qubits")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("2,3,4,5");

    let qubits: Vec<usize> = qubits_str.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    println!("\n⏱️ QFT Benchmark\n");

    use qops_research::benchmark::quantum_benchmarks;

    let suite = quantum_benchmarks::qft_scaling(&qubits);
    println!("{}", suite.comparison_table());
}

fn benchmark_simulation(args: &[String]) {
    let qubits_str = args.iter()
        .position(|a| a == "--qubits")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("4,6,8,10");

    let qubits: Vec<usize> = qubits_str.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    println!("\n⏱️ State Vector Simulation Benchmark\n");

    use qops_research::benchmark::quantum_benchmarks;

    let suite = quantum_benchmarks::simulation_scaling(&qubits, 10);
    println!("{}", suite.comparison_table());
}

// ============================================================================
// CALIBRATE COMMAND
// ============================================================================

fn run_calibrate(args: &[String]) {
    println!("\n🌀 Seraphic Calibration Shell\n");

    let steps = parse_arg(args, "--steps", 10);

    use qops_seraphic::SeraphicCalibrator;
    use qops_core::Signature3D;

    let mut calibrator = SeraphicCalibrator::default();
    calibrator.initialize(
        qops_core::Configuration::new("default"),
        Signature3D::new(0.5, 0.5, 0.5),
    );

    println!("Running {} calibration steps...\n", steps);

    let results = calibrator.run(steps);

    println!("Step │   ψ    │   ρ    │   ω    │ Accept │ CRI");
    println!("─────┼────────┼────────┼────────┼────────┼─────");
    for result in &results {
        let accept = if result.accepted { "✓" } else { " " };
        let cri = if result.cri_triggered { "!" } else { " " };
        println!(
            " {:3} │ {:.4} │ {:.4} │ {:.4} │   {}    │  {}",
            result.step,
            result.performance.psi,
            result.performance.rho,
            result.performance.omega,
            accept,
            cri
        );
    }

    let final_perf = calibrator.current_performance();
    println!();
    println!("Final: ψ={:.4} ρ={:.4} ω={:.4}",
        final_perf.psi, final_perf.rho, final_perf.omega);
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn parse_arg(args: &[String], flag: &str, default: usize) -> usize {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}
