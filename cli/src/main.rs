//! QOPS CLI - Unified Quantum Operator Processing System
//!
//! Usage:
//!   qops genesis [--agents N] [--steps S]
//!   qops quantum [--walk|--vqe|--qaoa]
//!   qops calibrate [--steps N]
//!   qops info

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "genesis" => run_genesis(&args[2..]),
        "quantum" => run_quantum(&args[2..]),
        "calibrate" => run_calibrate(&args[2..]),
        "info" => print_info(),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("QOPS - Unified Quantum Operator Processing System");
    println!();
    println!("Usage:");
    println!("  qops genesis   - Run Genesis operator mining");
    println!("  qops quantum   - Run quantum algorithms");
    println!("  qops calibrate - Run Seraphic calibration");
    println!("  qops info      - Show system information");
    println!("  qops help      - Show this help message");
}

fn print_info() {
    println!("QOPS - Unified Quantum Operator Processing System");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Components:");
    println!("  - Core: {}", qops_core::VERSION);
    println!("  - Genesis: S7 Topology ({} nodes)", qops_genesis::S7_NODE_COUNT);
    println!("  - Quantum: Cube-13 Topology ({} nodes)", qops_quantum::METATRON_DIMENSION);
    println!("  - Seraphic: Calibration Shell");
    println!();
    println!("Architecture:");
    println!("  Genesis Pipeline (MOGE) <-> Core <-> Quantum Pipeline (QSO)");
    println!("                            |");
    println!("                    Seraphic Calibration");
}

fn run_genesis(args: &[String]) {
    println!("Running Genesis Pipeline (S7 Operator Mining)...");
    println!();

    let mut agents = 5;
    let mut steps = 20;

    // Parse arguments
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
            _ => {}
        }
        i += 1;
    }

    println!("Configuration:");
    println!("  Agents: {}", agents);
    println!("  Steps per agent: {}", steps);
    println!();

    // Run traversal
    use qops_genesis::{TraversalEngine, AgentConfig, TraversalStrategy};

    let mut engine = TraversalEngine::new();
    let config = AgentConfig {
        max_steps: steps,
        strategy: TraversalStrategy::Balanced,
        ..Default::default()
    };

    println!("Mining operators...");
    let artefacts = engine.run_swarm(agents, config);

    println!();
    println!("Results:");
    for (i, artefact) in artefacts.iter().enumerate() {
        println!(
            "  Artefact {}: resonance={:.4}, mandorla={}",
            i + 1,
            artefact.resonance,
            artefact.is_mandorla
        );
    }

    if let Some(best) = engine.best_artefact() {
        println!();
        println!("Best artefact: resonance={:.4}", best.resonance);
    }
}

fn run_quantum(args: &[String]) {
    println!("Running Quantum Pipeline (Cube-13)...");
    println!();

    let mode = args.first().map(|s| s.as_str()).unwrap_or("walk");

    match mode {
        "--walk" | "walk" => run_quantum_walk(),
        "--vqe" | "vqe" => run_vqe(),
        "--qaoa" | "qaoa" => run_qaoa(),
        _ => {
            println!("Unknown quantum mode: {}", mode);
            println!("Options: walk, vqe, qaoa");
        }
    }
}

fn run_quantum_walk() {
    println!("Running Continuous-Time Quantum Walk...");

    use qops_quantum::{MetatronGraph, MetatronHamiltonian, QuantumState};
    use qops_quantum::quantum_walk::ContinuousQuantumWalk;

    let graph = MetatronGraph::new();
    let hamiltonian = MetatronHamiltonian::from_graph(&graph);
    let qw = ContinuousQuantumWalk::new(hamiltonian);

    let initial = QuantumState::basis_state(0).unwrap();
    println!("Initial state: |0⟩ (center node)");

    for t in [0.5, 1.0, 2.0, 5.0] {
        let evolved = qw.evolve(&initial, t);
        let probs = evolved.probabilities();

        println!();
        println!("t = {:.1}:", t);
        println!("  P(center) = {:.4}", probs[0]);
        println!("  P(hexagon) = {:.4}", probs[1..7].iter().sum::<f64>());
        println!("  P(cube) = {:.4}", probs[7..13].iter().sum::<f64>());
    }
}

fn run_vqe() {
    println!("Running VQE (Variational Quantum Eigensolver)...");

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

fn run_qaoa() {
    println!("Running QAOA (Quantum Approximate Optimization)...");

    use qops_quantum::vqa::QAOA;

    // Create simple graph problem
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

    println!();
    println!("Results:");
    println!("  Best cost: {:.2}", result.best_cost);
    println!("  Approximation ratio: {:.4}", result.approximation_ratio);
}

fn run_calibrate(args: &[String]) {
    println!("Running Seraphic Calibration Shell...");
    println!();

    let mut steps = 10;

    // Parse arguments
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--steps" | "-s" => {
                if i + 1 < args.len() {
                    steps = args[i + 1].parse().unwrap_or(10);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    use qops_seraphic::SeraphicCalibrator;
    use qops_core::Signature3D;

    let mut calibrator = SeraphicCalibrator::default();
    calibrator.initialize(
        qops_core::Configuration::new("default"),
        Signature3D::new(0.5, 0.5, 0.5),
    );

    println!("Running {} calibration steps...", steps);
    println!();

    let results = calibrator.run(steps);

    println!("Results:");
    for result in &results {
        println!(
            "  Step {}: ψ={:.3} ρ={:.3} ω={:.3} accepted={} CRI={}",
            result.step,
            result.performance.psi,
            result.performance.rho,
            result.performance.omega,
            result.accepted,
            result.cri_triggered
        );
    }

    println!();
    let final_perf = calibrator.current_performance();
    println!(
        "Final performance: ψ={:.4} ρ={:.4} ω={:.4}",
        final_perf.psi, final_perf.rho, final_perf.omega
    );
}
