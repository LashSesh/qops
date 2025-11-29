# QOPS - Quantum Operator Processing System

**A comprehensive framework for quantum algorithm research, operator mining, and resonance-based optimization.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-2.0-blue.svg)](https://tauri.app)

## Overview

QOPS (Quantum Operator Processing System) is a Rust-based quantum computing research framework that combines:

- **Desktop GUI Application**: Modern Tauri-based interface with Svelte frontend
- **Holistic Resonance Architecture**: Three-axis operator filtering and optimization
- **TRITON Spiral Search**: Adaptive optimization with topology-aware exploration
- **Genesis Pipeline**: S7 topology operator mining (5040 nodes)
- **Quantum Pipeline**: Cube-13 topology algorithms (13 nodes)
- **Hypercube-HDAG Framework**: 5D self-compiling cubes with hierarchical execution
- **Quantum Slots Engine**: Entropy-driven slot evaluation with sequence mining
- **Classical Quantum Algorithms**: Grover, Shor, QFT, QPE, VQE, QAOA
- **Research Tools**: Benchmarking, experiments, analysis, visualization

## Holistic Resonance Architecture

The core innovation of QOPS is the three-axis Holistic Resonance Architecture based on the Pfauenthron framework:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    HOLISTIC RESONANCE PIPELINE                           │
│                                                                          │
│   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐             │
│   │  KOSMOKRATOR │ -> │ CHRONOKRATOR │ -> │ PFAUENTHRON  │             │
│   │  (Exclusion) │    │  (Expansion) │    │  (Collapse)  │             │
│   │              │    │              │    │              │             │
│   │  PoR Filter  │    │   D_total    │    │   Mandorla   │             │
│   │  Telescope   │    │ Exkalibration│    │   Monolith   │             │
│   └──────────────┘    └──────────────┘    └──────────────┘             │
│         │                    │                    │                     │
│         v                    v                    v                     │
│   ┌─────────────────────────────────────────────────────────┐          │
│   │               M(t) = E(t)  if  PoR(t) ∧ D(t) > Θ(t)     │          │
│   │                           else  ∅                        │          │
│   └─────────────────────────────────────────────────────────┘          │
└─────────────────────────────────────────────────────────────────────────┘
```

### Kosmokrator (Exclusion Axis)

Proof-of-Resonance filtering with coherence measurement:

```
κ(t) = |1/N · Σ exp(i·θⱼ(t))|
```

- **PoR Filter**: Excludes candidates below kappa threshold
- **Telescope Operator**: Adaptive stability adjustment
- **History Window**: Temporal coherence tracking

### Chronokrator (Expansion Axis)

Multi-channel resonance dynamics:

```
E(t) = ∇_{ψ,ρ,ω} Φ(t)   (Exkalibration Vector)
D_total(t) > Θ(t)        (Threshold Condition)
```

- **Multi-Channel Tracking**: Parallel resonance streams
- **Exkalibration Vector**: Gradient-based expansion
- **Spike Detection**: Resonance anomaly identification

### Pfauenthron (Collapse Axis)

Mandorla convergence and Monolith formation:

```
S_Mandorla = P_Gabriel · I_Oriphiel
```

- **O.P.H.A.N. Geometry**: Ophanim node configuration
- **Mandorla Field**: Gabriel-Oriphiel convergence
- **Monolith Formation**: Finalized operator family embedding

## TRITON Spiral Search Optimizer

Enhanced spiral search with adaptive features:

```
┌─────────────────────────────────────────────────────────────┐
│                    TRITON Adaptive Optimizer                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   Adaptive   │  │   Dynamic    │  │  Topology    │       │
│  │    Radius    │  │   Cooling    │  │   Bias       │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │    Layer     │  │ Convergence  │  │    Drift     │       │
│  │   Memory     │  │  Stabilizer  │  │  Corrector   │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

- **Adaptive Radius**: Expansion on improvement, contraction otherwise
- **Dynamic Cooling**: Temperature-based acceptance probability
- **Topology Gaussian Bias**: Guided exploration near best regions
- **Layer Memory**: Best-score tracking per spiral layer
- **Convergence Stabilizer**: Variance and plateau detection

## Hypercube-HDAG 5D Framework

The Hypercube framework provides self-compiling 5D cube structures with hierarchical DAG execution:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    HYPERCUBE-HDAG FRAMEWORK                              │
│                                                                          │
│   5D Coordinates: (psi, rho, omega, chi, eta)                            │
│                                                                          │
│   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐             │
│   │     DK       │    │     SW       │    │     PI       │             │
│   │ Double Kick  │    │  Swap Wave   │    │   Phase Int  │             │
│   └──────────────┘    └──────────────┘    └──────────────┘             │
│   ┌──────────────┐    ┌──────────────┐                                 │
│   │     WT       │    │     Xi       │                                 │
│   │Weight Trans  │    │ Compilation  │                                 │
│   └──────────────┘    └──────────────┘                                 │
│                                                                          │
│   HDAG Pipeline: Input -> [Operators] -> Merge -> Compile -> Output     │
└─────────────────────────────────────────────────────────────────────────┘
```

- **5D Operators**: DK (Double Kick), SW (Swap Wave), PI (Phase Integration), WT (Weight Transform), Xi (Compilation)
- **Expansion Rules**: Lattice, Resonance-Guided, TRITON, Operator-Driven, Hybrid
- **HDAG Execution**: Hierarchical DAG for structured pipeline execution

See [HYPERCUBE_INTEGRATION.md](HYPERCUBE_INTEGRATION.md) for details.

## Quantum Slots Engine (QSlots)

Entropy-driven slot evaluation with sequence mining capabilities:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    QUANTUM SLOTS ENGINE                                  │
│                                                                          │
│   Symbols: [psi] [rho] [omega] [chi] [eta] [Star] [Diamond] [Circle]     │
│   Weights:  0.4   0.3   0.3    0.05 -0.05   0.1    0.15     0.05        │
│                                                                          │
│   Entropy Distributions: Uniform | Normal | Exponential | Beta | Bimodal│
│   Mining Strategies: Greedy | Stochastic | Beam | Evolutionary | TRITON │
│                                                                          │
│   Session: Spin -> Evaluate -> Mine -> Generate Artifacts               │
└─────────────────────────────────────────────────────────────────────────┘
```

- **Symbol-based evaluation** with resonance-aligned weights
- **Configurable entropy** distributions for varied exploration
- **Multiple mining strategies** including TRITON spiral search
- **Hypercube integration** for artifact generation

See [SLOTS_ENGINE_SPEC.md](SLOTS_ENGINE_SPEC.md) for details.

## Desktop Application

QOPS includes a full-featured desktop GUI built with Tauri 2.0 and SvelteKit:

```
+-----------------------------------------------------------------------+
|  QOPS - Quantum Operator Processing System              [_][O][X]      |
+-----------------------------------------------------------------------+
| Dashboard | Genesis Miner | Circuits | Algorithms | Topology | ...     |
+-----------------------------------------------------------------------+
|                                                                         |
|  GENESIS HOLISTIC MINER                                                |
|  =====================                                                  |
|                                                                         |
|  [Discovery] -> [Kosmokrator] -> [Chronokrator] -> [Pfauenthron]       |
|      ●             ●                 ○                 ○               |
|                                                                         |
|  Configuration:              Results:                                   |
|  ┌─────────────────┐        ┌────────────────────────────────┐         |
|  │ Preset: thorough│        │ Discovered:    150 candidates  │         |
|  │ Agents: 10      │        │ After KOS:      45 candidates  │         |
|  │ Steps:  50      │        │ After CHR:      12 candidates  │         |
|  │ TRITON: Adaptive│        │ Families:        3 finalized   │         |
|  └─────────────────┘        └────────────────────────────────┘         |
|                                                                         |
|  [Start Holistic Mining]                                               |
+-----------------------------------------------------------------------+
```

**Features:**
- Genesis Miner with animated stage pipeline (blue -> violet -> gold)
- Resonance visualizer components (Heatmap, Spiral, Network)
- Visual Circuit Builder with drag-and-drop gates
- S7 Topology Explorer (5040 nodes)
- Export to JSON, CSV, Markdown
- Dark quantum-themed UI

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
│  │         (Holistic Resonance Architecture)                        │        │
│  └──────┬────────────────┬────────────────┬────────────────┬──────┘        │
│         │                │                │                │               │
│  ┌──────┴──────┐  ┌──────┴──────┐  ┌──────┴──────┐  ┌──────┴──────┐        │
│  │   Genesis   │  │   TRITON    │  │   Quantum   │  │  Seraphic   │        │
│  │  (S7 5040)  │  │  (Spiral)   │  │  (Cube 13)  │  │    (SCS)    │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

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

# Run holistic mining pipeline
cargo run --bin qops -- genesis holistic --agents 10 --steps 50 --preset thorough

# Run individual stages
cargo run --bin qops -- genesis stage kosmokrator --kappa 0.7 --export
cargo run --bin qops -- genesis stage chronokrator --channels 4 --visualize
cargo run --bin qops -- genesis stage pfauenthron --mandorla-threshold 0.8

# Run TRITON spiral search
cargo run --bin qops -- genesis spiral --adaptive --iterations 1000

# Finalize Monolith
cargo run --bin qops -- genesis finalize --export

# Run quantum algorithms
cargo run --bin qops -- algorithm grover --qubits 4 --target 5
cargo run --bin qops -- algorithm shor --number 15
cargo run --bin qops -- algorithm vqe --qubits 2

# Benchmark performance
cargo run --bin qops -- benchmark qft --qubits 2,3,4,5
```

### Programmatic Usage

```rust
use qops_genesis::{HolisticMiningConfig, HolisticMiningSession, GenesisReporter, ReportFormat};
use qops_core::{HolisticConfig, KosmokratorConfig, ChronokratorConfig, PfauenthronConfig};

// Configure holistic mining
let config = HolisticMiningConfig {
    holistic: HolisticConfig {
        kosmokrator: KosmokratorConfig {
            kappa_threshold: 0.7,
            telescope_enabled: true,
            ..Default::default()
        },
        chronokrator: ChronokratorConfig {
            num_channels: 4,
            exkalibration_enabled: true,
            ..Default::default()
        },
        pfauenthron: PfauenthronConfig {
            mandorla_threshold: 0.8,
            monolith_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    },
    num_agents: 10,
    steps_per_agent: 50,
    use_adaptive_triton: true,
    ..Default::default()
};

// Run the pipeline
let mut session = HolisticMiningSession::new(config);
session.run_discovery();
session.run_kosmokrator();
session.run_chronokrator();
session.run_pfauenthron();
let result = session.finalize();

// Generate report
let reporter = GenesisReporter::new(result);
let markdown = reporter.generate(ReportFormat::Markdown)?;
println!("{}", markdown);
```

## Workspace Structure

```
qops/
├── core/           # Shared types, Holistic Resonance Architecture
│   └── holistic.rs # Kosmokrator, Chronokrator, Pfauenthron stages
├── triton/         # TRITON Spiral Search Optimizer
│   └── adaptive.rs # Adaptive radius, cooling, topology bias
├── genesis/        # S7 topology operator mining (5040 nodes)
│   ├── holistic_mining.rs  # Multi-stage mining session
│   └── reporting.rs        # Export (JSON, CSV, MD)
├── quantum/        # Cube-13 quantum algorithms (13 nodes)
├── circuits/       # Quantum circuit simulator
├── algorithms/     # Classical quantum algorithms
├── research/       # Benchmarking & experiments
├── seraphic/       # Calibration meta-algorithm
├── hypercube/      # Hypercube-HDAG 5D Framework
│   ├── cube.rs     # Self-compiling hypercube
│   ├── hdag.rs     # Hierarchical DAG execution
│   └── operators.rs # 5D operators (DK, SW, PI, WT, Xi)
├── slots/          # Quantum Slots Engine (QSlots)
│   ├── miner.rs    # Sequence mining strategies
│   ├── entropy.rs  # Entropy distributions
│   └── hypercube_integration.rs # Slots-hypercube bridge
├── gui/            # Tauri + SvelteKit desktop application
│   ├── src-tauri/  # Rust backend with Tauri commands
│   └── src/        # SvelteKit frontend
│       ├── routes/genesis/    # Genesis Miner page
│       ├── routes/hypercube/  # Hypercube Studio
│       └── routes/slots/      # Slots Dashboard
└── cli/            # Command-line interface
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `qops info` | Display system information |
| `qops genesis holistic` | Run full holistic mining pipeline |
| `qops genesis stage <stage>` | Run individual stage (kosmokrator/chronokrator/pfauenthron) |
| `qops genesis spiral` | Run TRITON spiral search |
| `qops genesis finalize` | Finalize Monolith structure |
| `qops circuit <type>` | Simulate quantum circuits |
| `qops algorithm <algo>` | Run quantum algorithms |
| `qops benchmark <algo>` | Benchmark performance |
| `qops research <mode>` | Research tools |
| `qops quantum` | Cube-13 algorithms |
| `qops calibrate` | Seraphic calibration |
| `qops hypercube compile` | Compile hypercube from seed |
| `qops hypercube expand` | Expand hypercube step by step |
| `qops hypercube exec-hdag` | Execute HDAG pipeline |
| `qops slots run` | Run slots engine |
| `qops slots sequence-mine` | Mine optimal sequences |

## Quantum Algorithms

| Algorithm | Description | Speedup |
|-----------|-------------|---------|
| **Grover's Search** | Unstructured database search | O(√N) |
| **Shor's Factorization** | Integer factorization | Exponential |
| **QFT** | Quantum Fourier Transform | O(n²) vs O(n·2ⁿ) |
| **QPE** | Quantum Phase Estimation | Eigenvalue finding |
| **VQE** | Variational Quantum Eigensolver | Ground state energy |
| **QAOA** | Quantum Approximate Optimization | Combinatorial opt. |
| **Hamiltonian Simulation** | Time evolution | Trotter-Suzuki |

## Mathematical Background

### Holistic Matrix Decision Logic

```
M(t) = E(t)  if  PoR(t) = true  ∧  D_total(t) > Θ(t)
       ∅     otherwise
```

Where:
- `PoR(t)`: Proof-of-Resonance passes kappa threshold
- `D_total(t)`: Total resonance dynamics measure
- `Θ(t)`: Adaptive threshold
- `E(t)`: Exkalibration vector output

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

### Kosmokrator Coherence

```
κ(t) = |1/N · Σⱼ exp(i·θⱼ(t))|
```

Phase coherence measure for Proof-of-Resonance filtering.

### Mandorla Convergence

```
S_Mandorla = P_Gabriel · I_Oriphiel
```

Product of Gabriel (protective) and Oriphiel (integrative) components.

## Visualization Components

The GUI includes reusable Svelte components for resonance visualization:

- **ResonanceHeatmap**: Color-coded resonance distribution (blue -> violet -> gold -> green)
- **SpiralTrajectory**: TRITON spiral search path with score coloring
- **FamilyNetwork**: Operator family relationship graph
- **StageIndicator**: Animated pipeline stage progress

## Performance

State vector simulation scales exponentially with qubit count:

| Qubits | States | Memory (approx) |
|--------|--------|-----------------|
| 10 | 1,024 | 16 KB |
| 15 | 32,768 | 512 KB |
| 20 | 1,048,576 | 16 MB |
| 25 | 33,554,432 | 512 MB |
| 30 | 1,073,741,824 | 16 GB |

Genesis S7 topology contains 5040 nodes (7! permutations).

## Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run specific test modules
cargo test -p qops-core holistic
cargo test -p qops-triton adaptive
cargo test -p qops-genesis holistic_mining
```

### Building Documentation

```bash
cargo doc --workspace --open
```

### Release Build

```bash
cargo build --release
```

### Running the GUI

```bash
cd gui
npm install
npm run tauri dev
```

## Origins

QOPS fuses two systems:

1. **Genesis Engine (MOGE)**: Metatronic Operator Genesis Engine
   - S7 permutation topology (7! = 5040 nodes)
   - Agent-based operator mining with resonance feedback
   - Holistic Resonance Architecture (Kosmokrator/Chronokrator/Pfauenthron)

2. **MetatronQSO**: Quantum State Operator Framework
   - 13-node Metatron Cube geometry
   - Variational quantum algorithms (VQE, QAOA)
   - Seraphic Calibration Shell

3. **TRITON Optimizer**: Spiral Search System
   - Golden ratio spiral expansion
   - Adaptive temperature annealing
   - Topology-aware exploration

## Benchmarks & Scientific Use

QOPS includes a comprehensive benchmark suite for evaluating quantum algorithms, hypercube operations, and system performance.

### Quick Start

```bash
# Run quick benchmarks (for development/CI)
cargo run --release --bin qops -- benchmark quick

# Run specific benchmark suite
cargo run --release --bin qops -- benchmark vqe
cargo run --release --bin qops -- benchmark qaoa
cargo run --release --bin qops -- benchmark hypercube

# Run all benchmarks
cargo run --release --bin qops -- benchmark all
```

### Available Benchmarks

| Benchmark | Command | Description |
|-----------|---------|-------------|
| VQE | `benchmark vqe` | Variational Quantum Eigensolver |
| VQC | `benchmark vqc` | Variational Quantum Classifier |
| QAOA | `benchmark qaoa` | Quantum Approximate Optimization |
| QWalk | `benchmark quantum-walk` | Continuous-time quantum walks |
| Advanced | `benchmark advanced` | Grover, QFT, QPE algorithms |
| Integration | `benchmark integration` | Cross-module compatibility |
| Cross-System | `benchmark cross` | Framework comparison |
| Hypercube | `benchmark hypercube` | Hypercube-HDAG cascades |
| Mining | `benchmark mining` | Operator mining efficiency |
| Topology | `benchmark topology` | Topological invariants |
| GUI Latency | `benchmark gui-latency` | Backend operation latency |

### CI Integration

Benchmarks run automatically via GitHub Actions:
- **Quick benchmarks**: On every push/PR
- **Full suite**: Manual workflow dispatch
- **Nightly**: Scheduled runs with regression detection

Results are uploaded as artifacts suitable for scientific analysis and DOI-backed technical reports.

See [BENCHMARK_OVERVIEW.md](BENCHMARK_OVERVIEW.md) for detailed documentation and [BENCHMARK_SCHEMA.md](BENCHMARK_SCHEMA.md) for JSON output format.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please read our contributing guidelines and submit PRs.

## Authors

QOPS Unified Team
