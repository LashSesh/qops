# QOPS - Quantum Research Framework

**A comprehensive framework for quantum algorithm research, experimentation, and education.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-2.0-blue.svg)](https://tauri.app)

## Overview

QOPS (Quantum Operator Processing System) is a Rust-based quantum computing research framework that combines:

- **Desktop GUI Application**: Modern Tauri-based interface with Svelte frontend
- **Quantum Circuit Simulator**: Universal gate set, state vector simulation
- **Classical Quantum Algorithms**: Grover, Shor, QFT, QPE, VQE, QAOA
- **Research Tools**: Benchmarking, experiments, analysis, visualization
- **Genesis Pipeline**: S7 topology operator mining (5040 nodes)
- **Quantum Pipeline**: Cube-13 topology algorithms (13 nodes)
- **Seraphic Calibration**: Meta-algorithm for configuration optimization

## Desktop Application

QOPS includes a full-featured desktop GUI built with Tauri 2.0 and SvelteKit:

```
+-------------------------------------------------------------------+
|  QOPS - Quantum Operator Processing System         [_][O][X]       |
+-------------------------------------------------------------------+
| Dashboard | Circuits | Algorithms | Topology | Resonance | Settings|
+-------------------------------------------------------------------+
|                                                                     |
|  Welcome to QOPS                                                   |
|  ====================================================================|
|                                                                     |
|  [Run Grover]    [Run Shor]    [Build Circuit]    [Explore S7]     |
|                                                                     |
+-------------------------------------------------------------------+
```

**Features:**
- Visual Circuit Builder with drag-and-drop gates
- Algorithm Runner with real-time visualization
- S7 Topology Explorer (5040 nodes)
- Cube-13 Quantum Walk visualization
- Seraphic Calibration interface
- Dark quantum-themed UI

See [docs/GUI_README.md](docs/GUI_README.md) for GUI documentation.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              QOPS Framework                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  Circuits   │  │ Algorithms  │  │  Research   │  │    CLI      │        │
│  │   (sim)     │  │  (quantum)  │  │   (tools)   │  │ (interface) │        │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘        │
│         │                │                │                │               │
│  ┌──────┴────────────────┴────────────────┴────────────────┴──────┐        │
│  │                            Core                                  │        │
│  └──────┬────────────────┬────────────────┬────────────────┬──────┘        │
│         │                │                │                │               │
│  ┌──────┴──────┐  ┌──────┴──────┐  ┌──────┴──────┐  ┌──────┴──────┐        │
│  │   Genesis   │  │   Quantum   │  │  Seraphic   │  │  Adapters   │        │
│  │  (S7 5040)  │  │  (Cube 13)  │  │    (SCS)    │  │  (bridges)  │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Features

### Quantum Circuit Simulator (`qops-circuits`)

- **Qubits & States**: Single/multi-qubit states, Bloch sphere representation
- **Universal Gate Set**: H, X, Y, Z, S, T, Rx, Ry, Rz, CNOT, CZ, Toffoli, etc.
- **Circuit Building**: Fluent API for circuit construction
- **Measurement**: Projective measurement, Pauli expectations, tomography
- **Noise Models**: Depolarizing, amplitude damping, thermal relaxation

### Quantum Algorithms (`qops-algorithms`)

| Algorithm | Description | Speedup |
|-----------|-------------|---------|
| **Grover's Search** | Unstructured database search | O(√N) |
| **Shor's Factorization** | Integer factorization | Exponential |
| **QFT** | Quantum Fourier Transform | O(n²) vs O(n·2ⁿ) |
| **QPE** | Quantum Phase Estimation | Eigenvalue finding |
| **VQE** | Variational Quantum Eigensolver | Ground state energy |
| **QAOA** | Quantum Approximate Optimization | Combinatorial opt. |
| **Hamiltonian Simulation** | Time evolution | Trotter-Suzuki |

### Research Tools (`qops-research`)

- **Benchmarking**: Performance measurement, comparison tables
- **Experiments**: Structured experiments with parameter sweeps
- **Analysis**: Statistical summary, correlation, regression, t-tests
- **Visualization**: Export to JSON, CSV, Matplotlib, Gnuplot, LaTeX
- **Reports**: Generate Markdown/HTML/LaTeX reports

## Quick Start

### Installation

```bash
git clone https://github.com/LashSesh/qops.git
cd qops
cargo build --release
```

### Basic Usage

```bash
# Show all available commands
cargo run --bin qops -- help

# System information
cargo run --bin qops -- info

# Run Grover's algorithm
cargo run --bin qops -- algorithm grover --qubits 4 --target 5

# Factor a number with Shor's algorithm
cargo run --bin qops -- algorithm shor --number 15

# Create and simulate a Bell state
cargo run --bin qops -- circuit bell

# Run VQE for ground state energy
cargo run --bin qops -- algorithm vqe --qubits 2

# Benchmark quantum algorithms
cargo run --bin qops -- benchmark qft --qubits 2,3,4,5

# Run scaling experiment
cargo run --bin qops -- research experiment
```

### Programmatic Usage

```rust
use qops_circuits::{Circuit, QuantumRegister, Gate, Measurement};
use qops_algorithms::{Grover, Oracle};

// Create a Bell state
let circuit = Circuit::new(2)
    .h(0)
    .cnot(0, 1);

let mut reg = QuantumRegister::new(2);
reg.apply_circuit(&circuit).unwrap();

// Measure
let stats = Measurement::measure_all(&reg, 1000);
println!("Results: {:?}", stats.counts);

// Run Grover's search
let oracle = Oracle::marked_state(4, 5);
let grover = Grover::new(4, oracle);
let result = grover.run();
println!("Found: {}", result.measured_state);
```

## Workspace Structure

```
qops/
├── core/           # Shared types, signatures, resonance framework
├── genesis/        # S7 topology operator mining (5040 nodes)
├── quantum/        # Cube-13 quantum algorithms (13 nodes)
├── circuits/       # Quantum circuit simulator (NEW)
├── algorithms/     # Classical quantum algorithms (NEW)
├── research/       # Benchmarking & experiments (NEW)
├── seraphic/       # Calibration meta-algorithm
├── adapters/       # Bridge modules
└── cli/            # Command-line interface
```

## Module Details

### Core (`qops-core`)

Foundational types shared across all modules:

- **Signature System**: 3D (ψ, ρ, ω) and 5D (ψ, ρ, ω, χ, η) metrics
- **Resonance**: R(v) = 0.4·ψ + 0.3·ρ + 0.3·ω + 0.05·χ - 0.05·η
- **Topology Trait**: `ResonanceTopology` for graph structures
- **Pipeline**: Generative processing abstraction
- **Ledger**: Hash-chained result storage

### Circuits (`qops-circuits`)

Full quantum circuit simulation:

```rust
// Build a QFT circuit
let qft = Circuit::qft(4);

// Apply to register
let mut reg = QuantumRegister::new(4);
reg.apply_circuit(&qft)?;

// Get measurement statistics
let counts = reg.get_counts(1000);
```

### Algorithms (`qops-algorithms`)

Ready-to-use quantum algorithms:

```rust
// Grover's search
let grover = Grover::new(5, Oracle::marked_state(5, 17));
let result = grover.run();

// Shor's factorization
let shor = Shor::new(21);
let factors = shor.run();

// VQE for Heisenberg model
let hamiltonian = PauliSum::heisenberg(4, 1.0);
let vqe = VQE::for_hamiltonian(hamiltonian);
let ground_energy = vqe.run();

// QAOA MaxCut
let qaoa = QAOA::max_cut(edges, 3);
let solution = qaoa.run();
```

### Research (`qops-research`)

Tools for systematic research:

```rust
// Run scaling experiment
let result = Experiment::new("grover_scaling")
    .parameter(Parameter::new("qubits", vec![2, 3, 4, 5, 6]))
    .repetitions(10)
    .run(|params| { /* ... */ });

// Benchmark comparison
let suite = quantum_benchmarks::qft_scaling(&[2, 4, 6, 8]);
println!("{}", suite.comparison_table());

// Statistical analysis
let mut analysis = Analysis::new();
analysis.add_series("times", measurements);
let fit = analysis.linear_regression("qubits", "times");
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `qops info` | Display system information |
| `qops circuit <type>` | Simulate quantum circuits |
| `qops algorithm <algo>` | Run quantum algorithms |
| `qops benchmark <algo>` | Benchmark performance |
| `qops research <mode>` | Research tools |
| `qops genesis` | S7 operator mining |
| `qops quantum` | Cube-13 algorithms |
| `qops calibrate` | Seraphic calibration |

## Performance

State vector simulation scales exponentially with qubit count:

| Qubits | States | Memory (approx) |
|--------|--------|-----------------|
| 10 | 1,024 | 16 KB |
| 15 | 32,768 | 512 KB |
| 20 | 1,048,576 | 16 MB |
| 25 | 33,554,432 | 512 MB |
| 30 | 1,073,741,824 | 16 GB |

For larger systems, consider using noise models or approximate simulation.

## Development

### Running Tests

```bash
cargo test --workspace
```

### Building Documentation

```bash
cargo doc --workspace --open
```

### Release Build

```bash
cargo build --release
```

## Mathematical Background

### Resonance Formula

The unified performance metric:

```
R(v) = 0.4·ψ + 0.3·ρ + 0.3·ω + 0.05·χ - 0.05·η
```

Where:
- ψ (psi): Quality / Spectral coherence
- ρ (rho): Stability / Robustness
- ω (omega): Efficiency / Performance
- χ (chi): Topological coherence (5D)
- η (eta): Fluctuation measure (5D)

### Double-Kick Operator

Configuration evolution: T = Φ_V ∘ Φ_U

1. **Φ_U (Update Kick)**: Improves quality metric
2. **Φ_V (Stabilization Kick)**: Improves stability and efficiency

## Origins

QOPS fuses two systems:

1. **Genesis Engine (MOGE)**: Metatronic Operator Genesis Engine
   - S7 permutation topology (7! = 5040 nodes)
   - Agent-based operator mining with resonance feedback

2. **MetatronQSO**: Quantum State Operator Framework
   - 13-node Metatron Cube geometry
   - Variational quantum algorithms (VQE, QAOA)
   - Seraphic Calibration Shell

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please read our contributing guidelines and submit PRs.

## Authors

QOPS Unified Team
