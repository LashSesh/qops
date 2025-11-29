# Hypercube Kernel for Generative Theomimesis

A domain-agnostic kernel for blueprint mining and transformation on high-dimensional state spaces.

## Overview

The Hypercube Kernel implements the specification from "A Hypercube-Based Kernel for Generative Theomimesis". It provides a neutral, domain-agnostic framework for mining, composing, and materializing blueprints through high-dimensional state space exploration.

## Architecture

The kernel follows a layered architecture (L1-L7):

```
+-------------------------------------------------------------------------+
|                HYPERCUBE KERNEL FOR GENERATIVE THEOMIMESIS              |
|                                                                         |
|  L1: Domain Adapter Layer     - Maps objects to H^n via D_d             |
|  L2: Spectral/Signature Layer - Computes signatures and R(v)            |
|  L3: Hypercube/HDAG Layer     - Maintains Q and G structures            |
|  L4: Mining Layer             - Implements M = (Q, S, F, R)             |
|  L5: Materialization Layer    - Applies M to create artefacts           |
|  L6: User/Integration Layer   - GUI, API, CLI interfaces                |
|  L7: Ledger/Governance Layer  - Records transformations B -> A          |
+-------------------------------------------------------------------------+
```

## Core Concepts

### State Space H^n

The kernel operates in a high-dimensional state space H^n with a core 5D signature:

| Dimension | Symbol | Description |
|-----------|--------|-------------|
| psi (ψ)   | Intensity | Amplitude/strength |
| rho (ρ)   | Coherence | Correlation/consistency |
| omega (ω) | Frequency | Periodicity/oscillation |
| chi (χ)   | Coupling | Entanglement/connection |
| eta (η)   | Dissipation | Decay/entropy |

### Resonance Function R(v)

The resonance function measures alignment with ideal pattern families:

- **Simple**: `R(v) = ψ · ρ · ω`
- **Extended**: `R(v) = ψ · ρ · ω · (1 + α·χ - β·η)`
- **Weighted**: `R(v) = w_ψ·ψ + w_ρ·ρ + w_ω·ω`
- **Geometric**: `R(v) = (ψ · ρ · ω)^(1/3)`
- **Harmonic**: `R(v) = 3 / (1/ψ + 1/ρ + 1/ω)`

### Core Operators

1. **Extract (Ex)**: `Input -> {B1, ..., Bk}` - Generates blueprint candidates
2. **Compose (Co)**: `{(Bi, vi)} -> {(B'j, v'j)}` - Filters, merges, and prunes candidates
3. **Materialize (M)**: `B -> A` - Transforms blueprints into artefacts

### Mining Kernel M = (Q, S, F, R)

The mining kernel operates with:
- **Q**: Hypercube state space
- **S**: Search strategy
- **F**: Filter set for constraints
- **R**: Resonance function

## Module Structure

```
kernel/
├── Cargo.toml           # Crate manifest
└── src/
    ├── lib.rs           # Module exports
    ├── state.rs         # H^n state space
    ├── domain_adapters.rs # D_d adapters
    ├── resonance.rs     # R(v) implementations
    ├── operators.rs     # Ex, Co, M operators
    ├── mining.rs        # Mining kernel
    ├── materialization.rs # Artefact generation
    ├── ledger.rs        # Transformation ledger
    ├── config.rs        # Configuration
    ├── error.rs         # Error types
    └── blueprint.rs     # Blueprint types
```

## CLI Usage

### Mining

```bash
# Run mining with default settings
qops kernel mine

# Run with custom configuration
qops kernel mine --config configs/kernel_mining.toml

# Specify iterations and target
qops kernel mine --iterations 500 --target 0.8

# Choose search strategy
qops kernel mine --strategy triton
qops kernel mine --strategy evolutionary
qops kernel mine --strategy beam
```

### Materialization

```bash
# Materialize a blueprint
qops kernel materialize my_blueprint --output-type json

# Materialize with custom output directory
qops kernel materialize my_blueprint --output ./artefacts
```

### Ledger

```bash
# List recent transformations
qops kernel ledger list

# Show statistics
qops kernel ledger stats

# Verify integrity
qops kernel ledger verify

# Export to JSON
qops kernel ledger export --output ledger.json
```

### System Info

```bash
# Show kernel information
qops kernel info
```

## Configuration

Configuration is done via TOML files. See `configs/kernel_mining.toml` for a complete example.

### Key Configuration Sections

```toml
[state]
dimensions = 5
normalization = "clamp"

[resonance]
model = "simple"
min_threshold = 0.3
target_threshold = 0.7

[mining]
max_iterations = 1000
target_resonance = 0.7
strategy = "hybrid"
exploration_rate = 0.3

[materialization]
output_format = "json"
validate = true

[ledger]
storage = "file"
path = "./ledger.jsonl"
```

## Search Strategies

| Strategy | Description |
|----------|-------------|
| `greedy` | Always select the best candidate |
| `stochastic` | Softmax selection with temperature |
| `beam` | Beam search with fixed width |
| `evolutionary` | Genetic algorithm with crossover/mutation |
| `triton` | TRITON spiral search optimizer |
| `hybrid` | Combined TRITON + Evolutionary |

## API Examples

### Basic Mining

```rust
use qops_kernel::{
    MiningKernel, MiningConfig, SearchStrategy,
    CoreSignature, State,
};

// Create mining configuration
let config = MiningConfig {
    max_iterations: 1000,
    target_resonance: 0.7,
    strategy: SearchStrategy::Hybrid,
    ..Default::default()
};

// Create seed states
let seed = State::Core(CoreSignature::new(0.5, 0.5, 0.5, 0.5, 0.5));

// Run mining
let mut miner = MiningKernel::new(config);
let result = miner.mine(&[seed])?;

println!("Best resonance: {}", result.best_resonance);
for candidate in &result.candidates {
    println!("  {} - R={:.4}", candidate.blueprint.name, candidate.resonance_score);
}
```

### Materialization

```rust
use qops_kernel::{
    Materializer, Blueprint, CoreSignature, State,
    materialization::ArtefactType,
};
use std::path::PathBuf;

// Create blueprint
let state = State::Core(CoreSignature::new(0.8, 0.75, 0.9, 0.6, 0.3));
let blueprint = Blueprint::from_state("my_blueprint", state);

// Create materializer
let mut materializer = Materializer::new(PathBuf::from("./output"));

// Materialize
let result = materializer.materialize(&blueprint, ArtefactType::Data)?;
println!("Created artefact: {}", result.artefact.id);
```

### Using the Ledger

```rust
use qops_kernel::{MemoryLedger, KernelLedger, TransformationEntry};
use chrono::Utc;
use std::collections::HashMap;

let mut ledger = MemoryLedger::new();

// Record a transformation
let entry = TransformationEntry {
    blueprint_id: "bp_001".to_string(),
    artefact_id: "art_001".to_string(),
    timestamp: Utc::now(),
    parameters: HashMap::new(),
    resonance_score: 0.85,
    constraints: vec!["min_resonance".to_string()],
};

let record = ledger.record_transformation(entry)?;
println!("Recorded: {} (hash: {})", record.id, record.hash);

// Verify integrity
assert!(ledger.verify_integrity()?);
```

## Integration with Existing QOPS

The kernel integrates with existing QOPS components:

- **qops-hypercube**: Uses Hypercube and HDAG for state space navigation
- **qops-core**: Builds on core Signature5D and Coord5D types
- **qops-triton**: Leverages TRITON optimizer for search strategies
- **qops-genesis**: Can process Genesis pipeline outputs

## Blueprint vs Artefact

| Blueprint | Artefact |
|-----------|----------|
| Abstract specification | Concrete output |
| High-dimensional state | Generated content |
| Candidate in mining | Result of materialization |
| May have constraints | Fully validated |