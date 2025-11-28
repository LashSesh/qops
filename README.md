# QOPS - Unified Quantum Operator Processing System

**A fusion of Genesis-Engine (MOGE) and MetatronQSO (Q⊗DASH) into a unified, modular architecture.**

## Overview

QOPS combines two powerful systems:

- **Genesis Pipeline (MOGE)**: S7 topology operator mining (5040 nodes)
- **Quantum Pipeline (QSO)**: Cube-13 topology quantum algorithms (13 nodes)
- **Seraphic Calibration Shell**: Meta-algorithm for fixpoint-directed evolution

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         QOPS                                 │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐   │
│  │   Genesis    │    │   Quantum    │    │   Seraphic   │   │
│  │  (S7 5040)   │    │  (Cube 13)   │    │    (SCS)     │   │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘   │
│         │                   │                   │           │
│         └─────────┬─────────┴─────────┬─────────┘           │
│                   │                   │                      │
│            ┌──────┴───────┐    ┌──────┴───────┐              │
│            │   Adapters   │    │     Core     │              │
│            └──────────────┘    └──────────────┘              │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Core Concepts

### Unified Signature System (ψ, ρ, ω)

- **ψ (psi)**: Quality / Spectral coherence
- **ρ (rho)**: Stability / Robustness
- **ω (omega)**: Efficiency / Performance

Extended to 5D with χ (chi) and η (eta) for topological coherence and fluctuation.

### Resonance Formula

```
R(v) = 0.4·ψ + 0.3·ρ + 0.3·ω + 0.05·χ - 0.05·η
```

### Double-Kick Operator (T = Φ_V ∘ Φ_U)

1. **Φ_U (Update Kick)**: Improves quality
2. **Φ_V (Stabilization Kick)**: Improves stability and efficiency

## Workspace Structure

```
qops/
├── core/           # Shared types, traits, and utilities
├── genesis/        # S7 topology operator mining (MOGE)
├── quantum/        # Cube-13 quantum algorithms (QSO)
├── seraphic/       # Calibration Shell meta-algorithm
├── adapters/       # Bridge modules for integration
└── cli/            # Command-line interface
```

## Quick Start

### Build

```bash
cargo build --release
```

### Run Tests

```bash
cargo test --workspace
```

### CLI Usage

```bash
# Show system info
cargo run --bin qops -- info

# Run Genesis operator mining
cargo run --bin qops -- genesis --agents 5 --steps 50

# Run quantum walk
cargo run --bin qops -- quantum walk

# Run VQE
cargo run --bin qops -- quantum vqe

# Run QAOA
cargo run --bin qops -- quantum qaoa

# Run Seraphic calibration
cargo run --bin qops -- calibrate --steps 20
```

## Components

### Core (`qops-core`)

- `Signature` - 3D/5D performance metrics
- `ResonanceTopology` - Graph topology trait
- `CalibrationOperator` - Configuration evolution
- `GenerativePipeline` - Pipeline abstraction
- `MandorlaField` - 16D resonance field
- `ResonanceLedger` - Hash-chained storage

### Genesis (`qops-genesis`)

- `MetatronCube` - S7 permutation graph (5040 nodes)
- `Agent` - Traversal agents with strategies
- `Artefact` - Mining results with blueprints
- `Cubechain` - Hypercube-DAG ledger
- `MetaCognition` - Self-reflection layer
- `KNO` - Double-kick operators

### Quantum (`qops-quantum`)

- `MetatronGraph` - 13-node Metatron geometry
- `QuantumState` - Complex amplitude vectors
- `MetatronHamiltonian` - Graph Hamiltonians
- `VQE/QAOA/VQC` - Variational algorithms
- `ContinuousQuantumWalk` - Quantum walks
- `DTL` - Dynamic Tripolar Logic (58.5% advantage)

### Seraphic (`qops-seraphic`)

- `SeraphicCalibrator` - Main orchestrator
- `ProofOfResonance` - Acceptance validation
- `CRI` - Calibration Regime Initialization

## Origins

This system fuses:

1. **genesis-engine (MOGE)**: Metatronic Operator Genesis Engine
   - S7 permutation topology (5040 nodes)
   - Operator mining with resonance feedback
   - Meta-cognition and self-reflection

2. **qso (MetatronQSO)**: Quantum State Operator Framework
   - 13-node Metatron Cube geometry
   - VQE, QAOA, VQC algorithms
   - Seraphic Calibration Shell
   - DioniceOS integration

## License

MIT License

## Authors

QOPS Unified Team (fusion of MOGE and QSO development teams)
