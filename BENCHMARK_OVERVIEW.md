# QOPS Benchmark Overview

This document describes the benchmark suites available in QOPS, covering both the ported QSO benchmark families and the new QOPS/FUQ!/Hypercube benchmarks.

## Quick Start

```bash
# Run quick benchmarks (for CI/development)
cargo run --release --bin qops -- benchmark quick

# Run specific benchmark suite
cargo run --release --bin qops -- benchmark vqe
cargo run --release --bin qops -- benchmark qaoa
cargo run --release --bin qops -- benchmark hypercube

# Run all benchmarks
cargo run --release --bin qops -- benchmark all

# Run with small mode (faster, fewer iterations)
cargo run --release --bin qops -- benchmark vqe --small

# Specify output directory
cargo run --release --bin qops -- benchmark vqe --output my_results/
```

## Benchmark Families

### 1. VQE Benchmarks (vqe_bench.json)

**Purpose:** Benchmark the Variational Quantum Eigensolver for ground state energy estimation.

**Metrics:**
- `energy`: Final computed energy
- `evaluations`: Number of quantum circuit evaluations
- `converged`: Whether the optimizer converged
- `variance`: Energy variance

**Parameters:**
- `qubits`: Number of qubits (default: 2, 3, 4)
- `layers`: Ansatz circuit layers (default: 1, 2, 3)

**Command:** `qops benchmark vqe [--small]`

---

### 2. VQC Benchmarks (vqc_bench.json)

**Purpose:** Benchmark Variational Quantum Classifiers for quantum machine learning.

**Metrics:**
- `accuracy`: Classification accuracy
- `samples`: Number of classification samples
- `gate_count`: Number of gates in the circuit

**Parameters:**
- `qubits`: Feature encoding qubits (default: 2, 3, 4)
- `layers`: Variational layers (default: 1, 2)
- `samples`: Classification samples per run (default: 100)

**Command:** `qops benchmark vqc [--small]`

---

### 3. QAOA Benchmarks (qaoa_bench.json)

**Purpose:** Benchmark the Quantum Approximate Optimization Algorithm for MaxCut problems.

**Metrics:**
- `cut_value`: Best cut value found
- `approximation_ratio`: Ratio to optimal solution
- `evaluations`: Number of optimization evaluations

**Parameters:**
- `nodes`: Graph nodes (default: 4, 6, 8)
- `layers`: QAOA layers/depth (default: 1, 2, 3)

**Command:** `qops benchmark qaoa [--small]`

---

### 4. Quantum Walk Benchmarks (quantum_walk_bench.json)

**Purpose:** Benchmark continuous-time quantum walks on graph structures.

**Metrics:**
- `tvd`: Total variation distance from uniform distribution
- `hitting_probability`: Probability at target node
- `speedup_estimate`: Estimated speedup over classical walk
- `mixing_quality`: Quality of mixing (1 - tvd)

**Parameters:**
- `graph_size`: Number of nodes (default: 4, 8, 13)
- `time_steps`: Evolution times (default: 0.5, 1.0, 2.0, 5.0)

**Command:** `qops benchmark quantum-walk [--small]` or `qops benchmark qwalk [--small]`

---

### 5. Advanced Algorithms Benchmarks (advanced_algorithms_bench.json)

**Purpose:** Benchmark Grover's search, QFT, and QPE algorithms.

**Metrics:**
- Grover: `success_probability`, `quantum_evaluations`, `speedup`
- QFT: `fidelity`, `gate_count`, `circuit_depth`
- QPE: `estimated_phase`, `error`, `confidence`, `error_bound`

**Parameters:**
- `grover_qubits`: Qubits for Grover (default: 2, 3, 4, 5)
- `qft_qubits`: Qubits for QFT (default: 2, 3, 4, 5, 6)
- `qpe_precision`: Precision bits for QPE (default: 3, 4, 5)

**Command:** `qops benchmark advanced [--small]`

---

### 6. Integration Benchmarks (integration_bench.json)

**Purpose:** Benchmark cross-module compatibility and pipeline integration.

**Benchmarks:**
- `circuits_algorithms_integration`: Test circuits module with VQE
- `vqe_qaoa_pipeline`: Combined VQE + QAOA workflow

**Metrics:**
- Combined energy/cost values
- Total evaluations
- Gate counts

**Command:** `qops benchmark integration [--small]`

---

### 7. Cross-System Benchmarks (cross_system_bench.json)

**Purpose:** Compare QOPS against external frameworks (Qiskit, Cirq, PennyLane).

**Note:** External framework comparison requires feature flags or environment variables. By default, only QOPS internal benchmarks are run.

**Metrics:**
- `framework_id`: Framework identifier (0 = QOPS)
- `gate_count`, `circuit_depth`: Circuit metrics
- Timing information

**Command:** `qops benchmark cross [--small]`

---

### 8. Hypercube Cascade Benchmarks (hypercube_cascade_bench.json)

**Purpose:** Benchmark the new Hypercube-HDAG framework cascades (H^n → H^1).

**Metrics:**
- `total_vertices`, `total_edges`: Graph size metrics
- `best_resonance`: Best resonance score achieved
- `convergence_rate`: Rate of resonance improvement
- `max_depth`: Maximum expansion depth reached
- HDAG execution: `nodes_executed`, `nodes_failed`, `output_resonance`, `artifacts_generated`

**Parameters:**
- `dimensions`: Hypercube dimensions (default: 3, 4, 5)
- `hdag_sizes`: HDAG sizes to test (default: 5, 10, 20)
- `cascade_steps`: Expansion steps (default: 5, 10, 20)

**Command:** `qops benchmark hypercube [--small]`

---

### 9. Mining Benchmarks (mining_bench.json)

**Purpose:** Benchmark operator and sequence mining efficiency.

**Metrics:**
- `best_resonance`: Best resonance found
- `total_steps`: Total mining steps
- `steps_to_best`: Steps to reach best solution
- `time_to_best_ms`: Time to best solution
- `converged`: Whether mining converged
- Quality categories: `jackpot_count`, `good_count`, `okay_count`, `miss_count`

**Parameters:**
- `mining_depths`: Depths to test (default: 5, 10, 20)
- `strategies`: Mining strategies (default: greedy, beam, triton)

**Command:** `qops benchmark mining [--small]`

---

### 10. Topology Benchmarks (topology_bench.json)

**Purpose:** Benchmark topological invariant computations (Chern numbers, Berry phases).

**Benchmarks:**
- `chern_{n}n`: Chern number computation on n×n grid
- `berry_{n}n`: Berry phase computation around closed loop
- `resonance_{n}n`: Resonance metric computation

**Metrics:**
- `chern_number`: Computed Chern number
- `berry_phase`: Computed Berry phase (in units of π)
- `grid_points`, `loop_points`: Computation size
- Resonance: `avg_resonance`, `max_resonance`, `min_resonance`, `resonance_spread`

**Parameters:**
- `graph_sizes`: Sizes to test (default: 4, 8, 13, 20)

**Command:** `qops benchmark topology [--small]`

---

### 11. GUI Latency Benchmarks (gui_latency_bench.json)

**Purpose:** Benchmark backend latency for typical GUI operations.

**Operations:**
- `hypercube_start`: Initialize hypercube session
- `cascade_run`: Run cascade expansion
- `slots_spin`: Run slots engine spin

**Metrics:**
- `operation_type`: Operation identifier
- Duration timing

**Command:** `qops benchmark gui-latency [--small]`

---

## Composite Commands

### Quick Benchmarks
Run a minimal subset for CI/development:
```bash
qops benchmark quick
```
Runs: VQE (small), QAOA (small), QWalk (small), Hypercube (small)

### All Benchmarks
Run complete benchmark suite:
```bash
qops benchmark all
```
Runs all benchmark types with full parameters.

---

## Output Format

All benchmarks output JSON files to `bench_results/` (configurable via `--output`).

Each file follows the schema documented in [BENCHMARK_SCHEMA.md](BENCHMARK_SCHEMA.md).

Key fields in every output:
- `metadata`: Timestamp, git commit, system info, versions
- `benchmark_type`: Type identifier
- `suite`: Full benchmark suite with results
- `summary`: Aggregate statistics

---

## CI Integration

Benchmarks are automatically run via GitHub Actions:

1. **Quick Benchmarks**: On every push/PR to main
2. **Full Suite**: On manual workflow dispatch
3. **Nightly**: Scheduled at 2:00 AM UTC with regression detection

Artifacts are uploaded and retained:
- Quick: 30 days
- Full: 90 days
- Nightly: 365 days (for scientific analysis)

---

## Using Results for Scientific Reports

Benchmark results are designed for inclusion in DOI-backed technical reports:

1. **Reproducibility**: Each result includes git commit hash and system info
2. **Standardized Schema**: Consistent JSON structure for automated processing
3. **Versioning**: Benchmark version tracked for compatibility
4. **Artifact Retention**: Nightly results kept for 1 year

### Example: Extract metrics for a paper

```python
import json
from datetime import datetime

with open('bench_results/vqe_bench.json') as f:
    data = json.load(f)

print(f"Timestamp: {data['metadata']['timestamp']}")
print(f"Commit: {data['metadata']['git_commit']}")
print(f"Total benchmarks: {data['summary']['total_benchmarks']}")

for result in data['suite']['results']:
    print(f"{result['config']['name']}: {result['measurements'][0]['duration']['secs']}s")
```

---

## Legacy Benchmarks

The following legacy benchmark commands are deprecated but still available:

```bash
qops benchmark grover-legacy --qubits 2,3,4
qops benchmark qft-legacy --qubits 2,3,4,5
qops benchmark sim-legacy --qubits 4,6,8,10
```

These provide backward compatibility with the original QSO benchmark interface.
