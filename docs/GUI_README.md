# QOPS Desktop Application

A modern Tauri-based desktop application for quantum algorithm research, experimentation, and education.

## Features

- **Dashboard**: System overview and quick actions
- **Genesis Miner**: Holistic multi-stage S7 operator mining (Kosmokrator -> Chronokrator -> Pfauenthron)
- **Kernel Mining**: M = (Q, S, F, R) blueprint mining with artefact materialization
- **Hypercube Studio**: 5D self-compiling cubes with HDAG execution
- **Slots Engine**: Entropy-driven slot evaluation with sequence mining
- **Circuit Builder**: Visual drag-and-drop quantum circuit construction
- **Algorithm Runner**: Execute Grover, Shor, QFT, QPE, VQE, and QAOA
- **Topology Explorer**: Visualize S7 permutation group (5040 nodes) and Cube-13 structures
- **Resonance Analyzer**: Seraphic calibration and quantum walk analysis

## Screenshots (ASCII)

### Dashboard
```
+-------------------------------------------------------------------+
|  QOPS                                      [Settings] [?] [_][O][X]|
+-------------------------------------------------------------------+
| +--+ Dashboard                                                      |
| |  | Circuits                 +----------------------------------+ |
| |  | Algorithms               |  Welcome to QOPS                 | |
| |  | Topology                 |  ================================| |
| |  | Resonance                |                                  | |
| |  | Research                 |  [Run Grover]  [Run Shor]       | |
| +--+                          |  [Build Circuit] [Explore S7]   | |
|                               +----------------------------------+ |
+-------------------------------------------------------------------+
```

### Circuit Builder
```
+-------------------------------------------------------------------+
|  Circuit Builder - "my_circuit"                    [_][O][X]       |
+-------------------------------------------------------------------+
| [New] [Save] [Load] [Export QASM] [Simulate]                       |
+-------------------------------------------------------------------+
|  GATES    |                CIRCUIT CANVAS                | PROPS   |
|  +------+ | q0: ─[H]─●─────────────[M]──                | Qubit:3|
|  | H    | | q1: ────⊕─[T]─●────────[M]──                | Depth:4|
|  | X    | | q2: ──────────⊕─[H]────[M]──                | Gates:7|
|  +------+ +---------------------------------------------+ +------+
+-------------------------------------------------------------------+
```

## Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Platform-specific dependencies (see Tauri docs)

### Development

```bash
# Navigate to GUI directory
cd gui

# Install dependencies
npm install

# Run development server
npm run tauri:dev
```

### Production Build

```bash
# Build for current platform
npm run tauri:build

# Output in gui/src-tauri/target/release/bundle/
```

## Architecture

```
gui/
├── src/                    # Svelte frontend
│   ├── routes/            # SvelteKit pages
│   │   ├── genesis/       # Genesis Holistic Miner
│   │   ├── kernel/        # Kernel Mining & Materialization
│   │   ├── hypercube/     # Hypercube Studio
│   │   ├── slots/         # Slots Engine
│   │   ├── circuit/       # Circuit Builder
│   │   ├── algorithm/     # Algorithm Runner
│   │   ├── topology/      # Topology Explorer
│   │   └── resonance/     # Resonance Analyzer
│   └── lib/
│       ├── components/    # UI components (Coord5DDisplay, HDAGGraph, etc.)
│       ├── visual/        # Visualization components
│       └── tauri/         # Tauri command wrappers (commands.ts)
│
└── src-tauri/             # Rust backend
    └── src/
        ├── commands/      # Tauri commands
        │   ├── circuits.rs
        │   ├── algorithms.rs
        │   ├── genesis.rs
        │   ├── holistic.rs
        │   ├── hypercube.rs
        │   ├── kernel.rs
        │   ├── quantum.rs
        │   ├── slots.rs
        │   └── calibration.rs
        ├── state.rs       # Application state
        └── error.rs       # Error handling
```

## Available Commands

### Circuit Commands
- `create_circuit` - Create a new quantum circuit
- `add_gate` - Add a gate to a circuit
- `simulate_circuit` - Run simulation with measurement
- `get_circuit_qasm` - Export circuit to QASM format

### Algorithm Commands
- `run_grover` - Grover's search algorithm
- `run_shor` - Shor's factorization
- `run_qft` - Quantum Fourier Transform
- `run_qpe` - Quantum Phase Estimation
- `run_vqe` - Variational Quantum Eigensolver
- `run_qaoa` - Quantum Approximate Optimization

### Genesis/Holistic Commands
- `run_genesis_mining` - S7 operator mining
- `run_holistic_mining` - Full holistic pipeline (Kosmokrator -> Chronokrator -> Pfauenthron)
- `run_kosmokrator_stage` - Run exclusion axis stage
- `run_chronokrator_stage` - Run expansion axis stage
- `run_pfauenthron_stage` - Run collapse axis stage
- `export_holistic_results` - Export results to JSON/CSV/MD

### Kernel Commands
- `run_kernel_mining` - Run M = (Q, S, F, R) mining kernel
- `materialize_blueprint` - Transform blueprint into artefact
- `get_kernel_info` - Kernel version and capabilities
- `get_ledger_stats` - Transformation ledger statistics

### Hypercube Commands
- `compile_hypercube` - Compile from seed coordinate
- `expand_cube_step` - Expand hypercube iteratively
- `hdag_execute` - Execute HDAG pipeline
- `run_hypercube_session` - Run full session with preset

### Slots Commands
- `run_slots_engine` - Run slots engine session
- `slots_mine_sequence` - Mine optimal sequences
- `slots_generate_artifacts` - Generate artifacts from coordinates

### Topology Commands
- `get_s7_topology_info` - S7 topology information (5040 nodes)
- `get_cube13_info` - Cube-13 topology information
- `run_quantum_walk` - Cube-13 quantum walk

### Calibration Commands
- `run_calibration` - Seraphic calibration
- `get_calibration_status` - Current resonance status
- `run_hyperparameter_sweep` - Parameter optimization
- `run_auto_tune` - Automatic tuning

## Technology Stack

- **Frontend**: SvelteKit + Tailwind CSS
- **Backend**: Tauri 2.0 + Rust
- **Core**: QOPS quantum simulation crates

## Supported Platforms

- Windows 10/11 (x64)
- macOS 11+ (Intel & Apple Silicon)
- Linux (x64)

## License

MIT License - see LICENSE file for details.
