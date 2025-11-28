# Hypercube-HDAG 5D Framework Integration

This document describes the integration of the Hypercube-HDAG 5D Framework into QOPS, providing self-compiling cube structures with hierarchical DAG execution.

## Overview

The Hypercube framework extends QOPS with a 5-dimensional coordinate system that maps directly to the resonance metrics used throughout the system. It provides:

- **Self-compiling cubes** that evolve through expansion and compilation phases
- **HDAG (Hierarchical Directed Acyclic Graph)** for structured pipeline execution
- **5D operators** (DK, SW, PI, WT, Xi) for coordinate transformations
- **TRITON integration** for spiral search optimization
- **Artifact generation** for operator family compilation

## Architecture

```
qops-hypercube/
├── Cargo.toml
└── src/
    ├── lib.rs              # Module exports
    ├── error.rs            # Error types
    ├── coordinates.rs      # Coord5D - 5D coordinate system
    ├── operators.rs        # 5D operators (DK, SW, PI, WT, Xi)
    ├── vertex.rs           # HypercubeVertex
    ├── edge.rs             # HypercubeEdge
    ├── cube.rs             # Hypercube struct with expansion rules
    ├── hdag.rs             # HDAG and HDAGExecutor
    ├── artifact.rs         # HypercubeArtifact and collections
    ├── compiler.rs         # HypercubeCompiler
    ├── session.rs          # HypercubeSession management
    └── triton_mode.rs      # TRITON integration
```

## 5D Coordinate System (Coord5D)

The coordinate system aligns with QOPS resonance metrics:

| Dimension | Symbol | Weight | Description |
|-----------|--------|--------|-------------|
| psi       | psi    | 0.40   | Quality / Spectral coherence |
| rho       | rho    | 0.30   | Stability / Robustness |
| omega     | omega  | 0.30   | Efficiency / Performance |
| chi       | chi    | 0.05   | Topological coherence |
| eta       | eta    | -0.05  | Fluctuation measure |

### Resonance Formula

```
R(v) = 0.4 * psi + 0.3 * rho + 0.3 * omega + 0.05 * chi - 0.05 * eta
```

## 5D Operators

### DK (Double Kick)
Perturbation dynamics operator that applies controlled disturbances:
```rust
DK(coord, strength) -> coord'
```

### SW (Swap Wave)
Dimensional exchange operator:
```rust
SW(coord, dim1, dim2) -> coord'
```

### PI (Phase Integration)
Phase alignment operator using sinusoidal integration:
```rust
PI(coord, phase) -> coord'
```

### WT (Weight Transform)
Weighted mapping operator with configurable weights:
```rust
WT(coord, weights) -> coord'
```

### Xi (Compilation Operator)
The compilation meta-operator that combines other operators:
```rust
Xi(coord, operators) -> coord' + artifacts
```

## Cube Expansion Rules

The hypercube supports multiple expansion strategies:

1. **Lattice** - Expands to all neighboring lattice points
2. **ResonanceGuided** - Prioritizes high-resonance directions
3. **Triton** - Uses TRITON spiral search for expansion
4. **OperatorDriven** - Applies operators to determine expansion
5. **Random** - Random walk expansion
6. **HybridTriton** - Combines TRITON with resonance guidance

## HDAG (Hierarchical DAG)

### Node Types

- **Input** - Entry points with seed coordinates
- **Output** - Result collection points
- **Operator** - Transform nodes applying 5D operators
- **Merge** - Combines multiple input streams
- **Compilation** - Runs the Xi compilation operator

### Pipeline Types

#### Standard Pipeline
```
Input -> Transform -> Operator -> Compilation -> Output
```

#### Parallel Pipeline
```
Input -> [Branch1, Branch2, Branch3] -> Merge -> Compilation -> Output
```

### Execution

```rust
let hdag = HDAG::standard_pipeline(seed);
let mut executor = HDAGExecutor::new(hdag);
let result = executor.execute()?;
```

## CLI Commands

### Hypercube Compile
```bash
qops hypercube compile --iterations 10 --triton
```

### Hypercube Expand
```bash
qops hypercube expand --iterations 5 --rule triton
```

### HDAG Execute
```bash
qops hypercube exec-hdag --parallel
```

### Hypercube Info
```bash
qops hypercube info
```

## Tauri Commands

| Command | Description |
|---------|-------------|
| `compile_hypercube` | Compile a hypercube from seed |
| `expand_cube_step` | Expand hypercube step by step |
| `get_hypercube_info` | Get framework information |
| `hdag_execute` | Execute HDAG pipeline |
| `get_hdag_info` | Get HDAG structure |
| `run_hypercube_session` | Run full session |
| `get_hypercube_presets` | Get available presets |

## GUI Integration

### Hypercube Studio (`/hypercube`)

Features:
- **Session Mode** - Run complete hypercube sessions with presets
- **Compile Mode** - Manual compilation with TRITON option
- **Expand Mode** - Step-by-step expansion visualization
- **HDAG Mode** - Visual HDAG execution with node highlighting

### Components

- `HypercubeCanvas.svelte` - 5D projection visualization
- `HDAGGraph.svelte` - HDAG node/edge visualization
- `Coord5DDisplay.svelte` - Radar chart for coordinates

## Session Presets

| Preset | Max Depth | Expansion Rule | Description |
|--------|-----------|----------------|-------------|
| quick | 3 | Triton | Fast exploration |
| thorough | 10 | HybridTriton | Deep analysis |
| research | 20 | OperatorDriven | Research-grade |

## Integration with QOPS

### Genesis Integration

The hypercube framework can wrap Genesis operator mining:
- Operators discovered in Genesis become HDAG nodes
- Operator families map to hypercube vertices
- TRITON spiral search drives expansion

### Slots Integration

See `SLOTS_ENGINE_SPEC.md` for details on how slots generate artifacts as HDAG outputs.

## Example Usage

```rust
use qops_hypercube::{
    Hypercube, HypercubeConfig, CubeExpansionRule,
    HypercubeCompiler, CompilationConfig,
    HDAG, HDAGExecutor,
    Coord5D,
};

// Create a hypercube
let config = HypercubeConfig {
    max_depth: 10,
    expansion_rule: CubeExpansionRule::Triton,
    ..Default::default()
};
let mut cube = Hypercube::new("my_cube", config);

// Expand
for _ in 0..10 {
    cube.expand_step()?;
}

// Compile
let compiler = HypercubeCompiler::new(CompilationConfig::default());
let result = compiler.compile(&mut cube)?;

println!("Output resonance: {:.4}", result.resonance);
println!("Artifacts: {}", result.artifacts.len());

// Or use HDAG
let seed = Coord5D::new(0.8, 0.7, 0.6, 0.5, 0.4);
let hdag = HDAG::standard_pipeline(seed);
let mut executor = HDAGExecutor::new(hdag);
let hdag_result = executor.execute()?;
```

## Mathematical Foundation

The hypercube operates on a bounded 5-dimensional unit hypercube [0,1]^5. Each vertex has:
- A unique coordinate in 5D space
- A computed resonance value
- Connections to neighboring vertices

The compilation process:
1. Selects high-resonance vertices
2. Applies operator families to transform coordinates
3. Generates artifacts (operator families, compiled results)
4. Produces a final output coordinate

## Future Enhancements

- [ ] GPU-accelerated expansion
- [ ] Distributed HDAG execution
- [ ] Real-time visualization of cube evolution
- [ ] Custom operator DSL
- [ ] Persistence and serialization of cube state
