# QOPS Benchmark JSON Schema

This document defines the JSON schema for all QOPS benchmark output files.

## Top-Level Structure

All benchmark output files share this common structure:

```json
{
  "metadata": { ... },
  "benchmark_type": "string",
  "suite": { ... },
  "summary": { ... }
}
```

### Metadata Object

```json
{
  "metadata": {
    "timestamp": "2024-01-15T12:30:45.123456Z",
    "git_commit": "abc123def456...",
    "system_info": {
      "os": "linux",
      "arch": "x86_64",
      "cpu_count": 8,
      "hostname": "runner-abc123"
    },
    "benchmark_version": "1.0.0",
    "qops_version": "0.1.0"
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `timestamp` | ISO 8601 string | UTC timestamp when benchmarks completed |
| `git_commit` | string \| null | Git commit hash, null if not in a repo |
| `system_info.os` | string | Operating system (linux, macos, windows) |
| `system_info.arch` | string | Architecture (x86_64, aarch64, etc.) |
| `system_info.cpu_count` | integer | Number of available CPU cores |
| `system_info.hostname` | string | Machine hostname |
| `benchmark_version` | string | Benchmark suite version |
| `qops_version` | string | QOPS framework version |

### Benchmark Type

| Value | Description |
|-------|-------------|
| `vqe` | VQE benchmarks |
| `vqc` | VQC classification benchmarks |
| `qaoa` | QAOA MaxCut benchmarks |
| `quantum_walk` | Quantum walk benchmarks |
| `advanced_algorithms` | Grover, QFT, QPE benchmarks |
| `integration` | Cross-module integration benchmarks |
| `cross_system` | Framework comparison benchmarks |
| `hypercube_cascade` | Hypercube-HDAG cascade benchmarks |
| `mining` | Operator/sequence mining benchmarks |
| `topology` | Topological invariant benchmarks |
| `gui_latency` | GUI operation latency benchmarks |

### Suite Object

```json
{
  "suite": {
    "name": "VQE Benchmark Suite",
    "description": "Variational Quantum Eigensolver performance benchmarks",
    "results": [ ... ],
    "metadata": { }
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Suite display name |
| `description` | string | Suite description |
| `results` | array | Array of BenchmarkResult objects |
| `metadata` | object | Additional key-value metadata |

### Summary Object

```json
{
  "summary": {
    "total_benchmarks": 9,
    "total_measurements": 45,
    "total_duration_ms": 1234.56,
    "success_rate": 1.0
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `total_benchmarks` | integer | Number of benchmark configurations run |
| `total_measurements` | integer | Total measurement count across all benchmarks |
| `total_duration_ms` | float | Total execution time in milliseconds |
| `success_rate` | float | Fraction of successful benchmarks (0.0-1.0) |

---

## BenchmarkResult Object

Each benchmark configuration produces a result:

```json
{
  "id": "uuid-string",
  "config": {
    "name": "vqe_2q_1l",
    "repetitions": 5,
    "warmup": 1,
    "timeout": { "secs": 60, "nanos": 0 },
    "seed": null,
    "params": {
      "qubits": 2,
      "layers": 1
    }
  },
  "measurements": [ ... ],
  "started_at": "2024-01-15T12:30:45.000Z",
  "completed_at": "2024-01-15T12:30:46.500Z",
  "success": true,
  "error": null
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID string | Unique result identifier |
| `config.name` | string | Benchmark configuration name |
| `config.repetitions` | integer | Number of measurement repetitions |
| `config.warmup` | integer | Warmup runs (not counted) |
| `config.timeout` | Duration \| null | Per-run timeout |
| `config.seed` | integer \| null | Random seed for reproducibility |
| `config.params` | object | Custom parameters (varies by benchmark) |
| `measurements` | array | Array of Measurement objects |
| `started_at` | ISO 8601 | Benchmark start time |
| `completed_at` | ISO 8601 | Benchmark completion time |
| `success` | boolean | Whether benchmark completed successfully |
| `error` | string \| null | Error message if failed |

---

## Measurement Object

Each repetition produces a measurement:

```json
{
  "duration": {
    "secs": 0,
    "nanos": 123456789
  },
  "memory_bytes": null,
  "gate_count": 15,
  "circuit_depth": 5,
  "success_probability": 0.95,
  "custom_metrics": {
    "energy": -1.234,
    "evaluations": 100.0,
    "converged": 1.0
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `duration` | Duration | Execution time |
| `memory_bytes` | integer \| null | Memory usage if tracked |
| `gate_count` | integer \| null | Quantum gates in circuit |
| `circuit_depth` | integer \| null | Circuit depth |
| `success_probability` | float \| null | Success probability (0.0-1.0) |
| `custom_metrics` | object | Benchmark-specific metrics |

### Duration Format

```json
{
  "secs": 0,
  "nanos": 123456789
}
```

Total duration = `secs` + `nanos` / 1_000_000_000 seconds.

---

## Benchmark-Specific Custom Metrics

### VQE (`vqe`)

| Metric | Type | Description |
|--------|------|-------------|
| `energy` | float | Computed ground state energy |
| `evaluations` | float | Number of circuit evaluations |
| `converged` | float | 1.0 if converged, 0.0 otherwise |
| `variance` | float | Energy variance |

### VQC (`vqc`)

| Metric | Type | Description |
|--------|------|-------------|
| `accuracy` | float | Classification accuracy (0.0-1.0) |
| `samples` | float | Number of samples classified |

### QAOA (`qaoa`)

| Metric | Type | Description |
|--------|------|-------------|
| `cut_value` | float | Best MaxCut value found |
| `approximation_ratio` | float | Ratio to optimal |
| `evaluations` | float | Optimization evaluations |

### Quantum Walk (`quantum_walk`)

| Metric | Type | Description |
|--------|------|-------------|
| `tvd` | float | Total variation distance |
| `hitting_probability` | float | Probability at target |
| `speedup_estimate` | float | Classical vs quantum speedup |
| `mixing_quality` | float | 1 - tvd |

### Advanced - Grover

| Metric | Type | Description |
|--------|------|-------------|
| `quantum_evaluations` | float | Oracle calls |
| `speedup` | float | Speedup over classical |

### Advanced - QFT

| Metric | Type | Description |
|--------|------|-------------|
| `fidelity` | float | State fidelity |

### Advanced - QPE

| Metric | Type | Description |
|--------|------|-------------|
| `estimated_phase` | float | Estimated phase |
| `error` | float | Phase error |
| `confidence` | float | Confidence level |
| `error_bound` | float | Theoretical error bound |

### Hypercube (`hypercube_cascade`)

| Metric | Type | Description |
|--------|------|-------------|
| `total_vertices` | float | Vertices in hypercube |
| `total_edges` | float | Edges in hypercube |
| `best_resonance` | float | Best resonance achieved |
| `convergence_rate` | float | Resonance improvement rate |
| `max_depth` | float | Maximum depth reached |
| `nodes_executed` | float | HDAG nodes executed |
| `nodes_failed` | float | HDAG nodes failed |
| `output_resonance` | float | HDAG output resonance |
| `artifacts_generated` | float | Number of artifacts |

### Mining (`mining`)

| Metric | Type | Description |
|--------|------|-------------|
| `best_resonance` | float | Best resonance found |
| `total_steps` | float | Total mining steps |
| `steps_to_best` | float | Steps to find best |
| `time_to_best_ms` | float | Time to best (ms) |
| `converged` | float | 1.0 if converged |
| `jackpot_count` | float | Count ≥ 0.9 resonance |
| `good_count` | float | Count ≥ 0.7, < 0.9 |
| `okay_count` | float | Count ≥ 0.5, < 0.7 |
| `miss_count` | float | Count < 0.5 |

### Topology (`topology`)

| Metric | Type | Description |
|--------|------|-------------|
| `chern_number` | float | Computed Chern number |
| `grid_points` | float | Grid size for computation |
| `berry_phase` | float | Berry phase (units of π) |
| `loop_points` | float | Points on Berry loop |
| `avg_resonance` | float | Average resonance |
| `max_resonance` | float | Maximum resonance |
| `min_resonance` | float | Minimum resonance |
| `resonance_spread` | float | max - min resonance |

### GUI Latency (`gui_latency`)

| Metric | Type | Description |
|--------|------|-------------|
| `operation_type` | float | Operation identifier |
| `steps` | float | Steps if applicable |

---

## Example Complete File

```json
{
  "metadata": {
    "timestamp": "2024-01-15T12:30:45.123456Z",
    "git_commit": "abc123def456789",
    "system_info": {
      "os": "linux",
      "arch": "x86_64",
      "cpu_count": 8,
      "hostname": "dev-machine"
    },
    "benchmark_version": "1.0.0",
    "qops_version": "0.1.0"
  },
  "benchmark_type": "vqe",
  "suite": {
    "name": "VQE Benchmark Suite",
    "description": "Variational Quantum Eigensolver performance benchmarks",
    "results": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "config": {
          "name": "vqe_2q_1l",
          "repetitions": 5,
          "warmup": 1,
          "timeout": { "secs": 60, "nanos": 0 },
          "seed": null,
          "params": { "qubits": 2, "layers": 1 }
        },
        "measurements": [
          {
            "duration": { "secs": 0, "nanos": 50000000 },
            "memory_bytes": null,
            "gate_count": null,
            "circuit_depth": null,
            "success_probability": null,
            "custom_metrics": {
              "energy": -1.137,
              "evaluations": 100.0,
              "converged": 1.0,
              "variance": 0.001
            }
          }
        ],
        "started_at": "2024-01-15T12:30:45.000Z",
        "completed_at": "2024-01-15T12:30:45.300Z",
        "success": true,
        "error": null
      }
    ],
    "metadata": {}
  },
  "summary": {
    "total_benchmarks": 1,
    "total_measurements": 5,
    "total_duration_ms": 250.0,
    "success_rate": 1.0
  }
}
```

---

## Compatibility Notes

- All numeric metrics are stored as `f64` (IEEE 754 double precision)
- Boolean values in metrics are encoded as `1.0` (true) or `0.0` (false)
- Null values indicate the metric was not measured
- The schema version is tracked via `benchmark_version`
- Old and new results can be compared if `benchmark_version` matches
