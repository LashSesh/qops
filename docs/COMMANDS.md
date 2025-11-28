# QOPS Tauri Commands Reference

Complete reference for all Tauri IPC commands available in the QOPS desktop application.

## Circuit Commands

### `create_circuit`

Creates a new quantum circuit.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `qubits` | `number` | Yes | Number of qubits (1-30) |
| `name` | `string` | No | Optional circuit name |

**Returns:** `CircuitDto`

**Example:**
```typescript
const circuit = await invoke('create_circuit', { qubits: 4, name: 'Bell State' });
```

---

### `add_gate`

Adds a quantum gate to an existing circuit.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `circuitId` | `string` | Yes | Circuit UUID |
| `gateType` | `string` | Yes | Gate type (H, X, Y, Z, S, T, CNOT, CZ, Toffoli, Rx, Ry, Rz) |
| `qubits` | `number[]` | Yes | Target qubit indices |
| `parameter` | `number` | No | Rotation angle for parametric gates |

**Supported Gates:**
- **Single-qubit:** H, X, Y, Z, S, T, Rx, Ry, Rz
- **Two-qubit:** CNOT, CZ, SWAP
- **Three-qubit:** Toffoli, Fredkin

**Returns:** `CircuitDto`

**Example:**
```typescript
// Add Hadamard gate to qubit 0
await invoke('add_gate', { circuitId: id, gateType: 'H', qubits: [0] });

// Add CNOT with control=0, target=1
await invoke('add_gate', { circuitId: id, gateType: 'CNOT', qubits: [0, 1] });

// Add Rx rotation with angle π/4
await invoke('add_gate', { circuitId: id, gateType: 'Rx', qubits: [0], parameter: Math.PI / 4 });
```

---

### `remove_gate`

Removes a gate from a circuit by index.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `circuitId` | `string` | Yes | Circuit UUID |
| `gateIndex` | `number` | Yes | Index of gate to remove |

**Returns:** `CircuitDto`

---

### `simulate_circuit`

Simulates a circuit and returns measurement statistics.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `circuitId` | `string` | Yes | Circuit UUID |
| `shots` | `number` | Yes | Number of measurement shots |

**Returns:** `SimulationResultDto`

```typescript
interface SimulationResultDto {
  probabilities: number[];    // State probabilities
  counts: Record<string, number>;  // Measurement counts
  shots: number;
}
```

---

### `get_circuit_qasm`

Exports circuit as OpenQASM 2.0 string.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `circuitId` | `string` | Yes | Circuit UUID |

**Returns:** `string` (QASM format)

---

### `list_circuits`

Lists all circuits in the current session.

**Parameters:** None

**Returns:** `CircuitDto[]`

---

### `delete_circuit`

Deletes a circuit from memory.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `circuitId` | `string` | Yes | Circuit UUID |

**Returns:** `boolean`

---

## Algorithm Commands

### `run_grover`

Executes Grover's quantum search algorithm.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `qubits` | `number` | Yes | Number of qubits (2-20) |
| `target` | `number` | Yes | Target state to search for |
| `shots` | `number` | Yes | Measurement shots |

**Returns:** `GroverResultDto`

```typescript
interface GroverResultDto {
  measured_state: number;
  measured_state_binary: string;
  success_probability: number;
  iterations: number;
  is_solution: boolean;
  counts: Record<string, number>;
  theoretical_probability: number;
}
```

**Example:**
```typescript
// Search for state |5⟩ in 4-qubit space
const result = await invoke('run_grover', { qubits: 4, target: 5, shots: 1000 });
console.log(`Found: ${result.measured_state}, Success: ${result.success_probability}`);
```

---

### `run_shor`

Executes Shor's factorization algorithm.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `number` | `number` | Yes | Number to factor (must be composite) |

**Returns:** `ShorResultDto`

```typescript
interface ShorResultDto {
  success: boolean;
  number: number;
  factors: number[];
  period: number | null;
  attempts: number;
}
```

**Example:**
```typescript
const result = await invoke('run_shor', { number: 15 });
// result.factors = [3, 5]
```

---

### `run_qft`

Executes Quantum Fourier Transform.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `qubits` | `number` | Yes | Number of qubits |
| `inputState` | `number` | No | Initial computational basis state |

**Returns:** `QftResultDto`

```typescript
interface QftResultDto {
  qubits: number;
  depth: number;
  gate_count: number;
  output_probabilities: number[];
}
```

---

### `run_iqft`

Executes Inverse Quantum Fourier Transform.

**Parameters:** Same as `run_qft`

**Returns:** `QftResultDto`

---

### `run_qpe`

Executes Quantum Phase Estimation.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `precision` | `number` | Yes | Number of precision qubits |
| `phase` | `number` | Yes | True phase to estimate (0 to 1) |
| `shots` | `number` | Yes | Measurement shots |

**Returns:** `QpeResultDto`

```typescript
interface QpeResultDto {
  estimated_phase: number;
  true_phase: number;
  error: number;
  confidence: number;
}
```

---

### `run_vqe`

Executes Variational Quantum Eigensolver.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `qubits` | `number` | Yes | Number of qubits |
| `layers` | `number` | Yes | Ansatz circuit depth |
| `maxIterations` | `number` | Yes | Maximum optimization iterations |

**Returns:** `VqeResultDto`

```typescript
interface VqeResultDto {
  energy: number;
  iterations: number;
  converged: boolean;
  variance: number;
  parameters: number[];
}
```

---

### `run_qaoa`

Executes Quantum Approximate Optimization Algorithm for MaxCut.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `edges` | `[number, number][]` | Yes | Graph edges as vertex pairs |
| `layers` | `number` | Yes | QAOA circuit depth (p) |
| `shots` | `number` | Yes | Measurement shots |

**Returns:** `QaoaResultDto`

```typescript
interface QaoaResultDto {
  best_solution: boolean[];
  best_cost: number;
  approximation_ratio: number;
  solution_counts: Record<string, number>;
}
```

**Example:**
```typescript
// MaxCut on a triangle graph
const edges: [number, number][] = [[0, 1], [1, 2], [0, 2]];
const result = await invoke('run_qaoa', { edges, layers: 2, shots: 1000 });
```

---

## Genesis Commands (S7 Topology)

### `get_s7_topology_info`

Gets information about the S7 permutation topology.

**Parameters:** None

**Returns:** `TopologyInfoDto`

```typescript
interface TopologyInfoDto {
  node_count: number;    // 5040 for S7
  edge_count: number;
  topology_type: string; // "S7"
}
```

---

### `get_node_details`

Gets detailed information about a specific node.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeId` | `number` | Yes | Node index (0-5039) |

**Returns:** `NodeDetailsDto`

```typescript
interface NodeDetailsDto {
  id: number;
  permutation: number[];
  signature: SignatureDto;
  neighbor_count: number;
}
```

---

### `run_genesis_mining`

Runs the Genesis operator mining algorithm.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `agents` | `number` | Yes | Number of mining agents |
| `steps` | `number` | Yes | Steps per agent |
| `strategy` | `string` | Yes | Mining strategy: "balanced", "explorative", "exploitative", "random" |

**Returns:** `GenesisResultDto`

```typescript
interface GenesisResultDto {
  artefacts: ArtefactDto[];
  best_resonance: number;
  mandorla_count: number;
  total_steps: number;
}

interface ArtefactDto {
  id: number;
  resonance: number;
  is_mandorla: boolean;
  node_path: number[];
}
```

---

## Quantum Commands (Cube-13)

### `get_cube13_info`

Gets information about the Cube-13 topology.

**Parameters:** None

**Returns:** `TopologyInfoDto`

---

### `run_quantum_walk`

Simulates quantum walk on Cube-13.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `times` | `number[]` | Yes | Time points to sample |

**Returns:** `QuantumWalkResultDto`

```typescript
interface QuantumWalkResultDto {
  time_points: number[];
  center_probabilities: number[];
  hex_probabilities: number[];
  cube_probabilities: number[];
}
```

---

### `run_cube13_vqe`

Runs VQE on Cube-13 topology.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `layers` | `number` | Yes | VQE ansatz depth |

**Returns:** `VqeResultDto`

---

## Calibration Commands

### `run_calibration`

Runs Seraphic Calibration Shell.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `steps` | `number` | Yes | Number of calibration steps |
| `target` | `number` | Yes | Target resonance value |

**Returns:** `CalibrationResultDto`

```typescript
interface CalibrationResultDto {
  steps: CalibrationStepDto[];
  final_signature: SignatureDto;
  accepted_count: number;
}

interface CalibrationStepDto {
  step: number;
  signature: SignatureDto;
  accepted: boolean;
  cri_triggered: boolean;
}
```

---

### `get_calibration_status`

Gets current calibration status.

**Parameters:** None

**Returns:** `SignatureDto`

```typescript
interface SignatureDto {
  psi: number;    // Quality / Spectral coherence
  rho: number;    // Stability / Robustness
  omega: number;  // Efficiency / Performance
  chi: number | null;   // Topological coherence (5D)
  eta: number | null;   // Fluctuation measure (5D)
}
```

---

## Research Commands

### `run_experiment`

Runs a structured experiment with parameter sweeps.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `algorithm` | `string` | Yes | Algorithm to benchmark |
| `qubitRange` | `number[]` | Yes | Qubit counts to test |
| `repetitions` | `number` | Yes | Repetitions per config |
| `shots` | `number` | Yes | Measurement shots |

**Returns:** `ExperimentResultDto`

---

### `run_benchmark`

Runs performance benchmarks.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `algorithm` | `string` | Yes | Algorithm to benchmark |
| `qubitCounts` | `number[]` | Yes | Qubit configurations |
| `shots` | `number` | Yes | Measurement shots |

**Returns:** `BenchmarkResultDto`

```typescript
interface BenchmarkResultDto {
  algorithm: string;
  configurations: BenchmarkConfigDto[];
  comparison_table: string;
}
```

---

## System Commands

### `get_system_info`

Gets QOPS system information.

**Parameters:** None

**Returns:** `SystemInfoDto`

```typescript
interface SystemInfoDto {
  version: string;
  modules: string[];
  capabilities: string[];
}
```

---

## Error Handling

All commands may throw errors with the following structure:

```typescript
interface QopsError {
  message: string;
  code: string;
}
```

**Error Codes:**
- `CIRCUIT_NOT_FOUND` - Invalid circuit ID
- `INVALID_GATE` - Unsupported gate type
- `INVALID_QUBIT` - Qubit index out of range
- `SIMULATION_ERROR` - Simulation failed
- `ALGORITHM_ERROR` - Algorithm execution failed
- `INTERNAL_ERROR` - Unexpected internal error

**Example error handling:**
```typescript
try {
  const result = await invoke('run_grover', { qubits: 4, target: 5, shots: 1000 });
} catch (error) {
  console.error(`Error: ${error}`);
}
```

---

## TypeScript Wrapper

For convenience, use the TypeScript wrapper in `src/lib/tauri/commands.ts`:

```typescript
import { runGrover, createCircuit, simulateCircuit } from '$lib/tauri/commands';

// Type-safe API calls
const groverResult = await runGrover(4, 5, 1000);
const circuit = await createCircuit(3, 'My Circuit');
const simResult = await simulateCircuit(circuit.id, 1000);
```
