# QOPS Desktop Application

A modern Tauri-based desktop application for quantum algorithm research, experimentation, and education.

## Features

- **Circuit Builder**: Visual drag-and-drop quantum circuit construction
- **Algorithm Runner**: Execute Grover, Shor, QFT, QPE, VQE, and QAOA
- **Topology Explorer**: Visualize S7 permutation group and Cube-13 structures
- **Resonance Analyzer**: Seraphic calibration and quantum walk analysis
- **Research Tools**: Experiments, benchmarking, and comparison

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
│   │   ├── circuit/       # Circuit Builder
│   │   ├── algorithm/     # Algorithm Runner
│   │   ├── topology/      # Topology Explorer
│   │   └── resonance/     # Resonance Analyzer
│   └── lib/
│       ├── components/    # UI components
│       └── tauri/         # Tauri command wrappers
│
└── src-tauri/             # Rust backend
    └── src/
        ├── commands/      # Tauri commands
        │   ├── circuits.rs
        │   ├── algorithms.rs
        │   ├── genesis.rs
        │   └── quantum.rs
        ├── state.rs       # Application state
        └── error.rs       # Error handling
```

## Available Commands

### Circuit Commands
- `create_circuit` - Create a new quantum circuit
- `add_gate` - Add a gate to a circuit
- `simulate_circuit` - Run simulation with measurement

### Algorithm Commands
- `run_grover` - Grover's search algorithm
- `run_shor` - Shor's factorization
- `run_qft` - Quantum Fourier Transform
- `run_qpe` - Quantum Phase Estimation
- `run_vqe` - Variational Quantum Eigensolver
- `run_qaoa` - Quantum Approximate Optimization

### Topology Commands
- `run_genesis_mining` - S7 operator mining
- `get_s7_topology_info` - S7 topology information
- `run_quantum_walk` - Cube-13 quantum walk

### Calibration Commands
- `run_calibration` - Seraphic calibration
- `get_calibration_status` - Current resonance status

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
