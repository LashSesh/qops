# Quantum Slots Engine (QSlots) Specification

This document describes the Quantum Slots Engine integrated into QOPS, providing entropy-driven slot evaluation with sequence mining capabilities.

## Overview

The QSlots engine provides a slot-machine metaphor for operator generation and resonance optimization. It features:

- **Slot-based symbol evaluation** with weighted outcomes
- **Entropy mapping** with configurable distributions
- **Lattice search** for optimal sequences
- **Sequence mining** with multiple strategies
- **Hypercube integration** for artifact generation

## Architecture

```
qops-slots/
├── Cargo.toml
└── src/
    ├── lib.rs                    # Module exports
    ├── error.rs                  # Error types
    ├── slot.rs                   # Slot, SlotSymbol, SlotConfig
    ├── spin.rs                   # SlotSpin mechanics
    ├── entropy.rs                # EntropyMapper with distributions
    ├── lattice.rs                # SlotLattice for grid search
    ├── topology.rs               # SlotTopology definitions
    ├── generator.rs              # SlotGenerator
    ├── miner.rs                  # SequenceMiner
    ├── session.rs                # SlotsSession management
    └── hypercube_integration.rs  # SlotsHypercubeAdapter
```

## Slot Symbols

The engine uses 8 primary symbols aligned with QOPS metrics:

| Symbol | Weight | Description |
|--------|--------|-------------|
| Psi | +0.40 | Quality / Spectral coherence |
| Rho | +0.30 | Stability / Robustness |
| Omega | +0.30 | Efficiency / Performance |
| Chi | +0.05 | Topological coherence |
| Eta | -0.05 | Fluctuation measure |
| Star | +0.10 | Bonus multiplier |
| Diamond | +0.15 | High-value symbol |
| Circle | +0.05 | Completion symbol |

### Resonance Calculation

For a sequence of symbols S = [s1, s2, ..., sn]:
```
R(S) = sum(weight(si) * value(si)) / n
```

Where value(si) is the entropy-generated value in [0, 1].

## Entropy Mapping

The entropy mapper generates random values according to configurable distributions:

### Uniform Distribution
```
f(x) = 1 for x in [0, 1]
```

### Normal Distribution
```
f(x) = (1 / (sigma * sqrt(2*pi))) * exp(-(x - mu)^2 / (2*sigma^2))
```
Default: mu = 0.5, sigma = 0.2

### Exponential Distribution
```
f(x) = lambda * exp(-lambda * x)
```
Default: lambda = 2.0

### Beta Distribution
```
f(x) = (x^(alpha-1) * (1-x)^(beta-1)) / B(alpha, beta)
```
Default: alpha = 2.0, beta = 5.0

### Bimodal Distribution
Two peaks at configurable positions.
Default: peak1 = 0.3, peak2 = 0.8

### Resonance Optimized
Specialized distribution biased toward high-resonance outcomes.

## Mining Strategies

### Greedy
Always selects the best immediate outcome:
```
next = argmax(evaluate(candidates))
```

### Stochastic (Simulated Annealing)
Accepts worse solutions with decreasing probability:
```
P(accept) = exp(-delta / T)
T = T * cooling_rate
```

### Beam Search
Maintains multiple candidate sequences:
```
beam = top_k(candidates, beam_width)
```

### Evolutionary
Genetic algorithm with:
- **Selection**: Tournament selection
- **Crossover**: Single-point crossover
- **Mutation**: Random symbol substitution

### TRITON
Uses TRITON spiral search adapted for sequence space.

## Lattice Search

The slot lattice provides a multi-dimensional grid for sequence exploration:

```rust
let lattice = SlotLattice::new(LatticeConfig {
    dimensions: 5,
    resolution: 10,
    ..Default::default()
});

// Find neighbors
let neighbors = lattice.neighbors(&current_position);

// Search for optimal
let best = lattice.search(start, target_resonance)?;
```

## Slot Topologies

### Ring Topology
Circular arrangement of slots with neighbor connections.

### Grid Topology
2D grid arrangement for spatial relationships.

### Hypercube Topology
n-dimensional hypercube connectivity.

## CLI Commands

### Run Slots Engine
```bash
qops slots run --steps 50 --entropy stochastic
```

### Sequence Mining
```bash
qops slots sequence-mine --depth 10 --strategy beam
```

### Slots Info
```bash
qops slots info
```

## Tauri Commands

| Command | Description |
|---------|-------------|
| `run_slots_engine` | Run complete slots session |
| `slots_mine_sequence` | Mine optimal sequences |
| `get_slots_info` | Get engine information |
| `get_mining_strategies` | List available strategies |
| `get_entropy_distributions` | List distributions |
| `slots_generate_artifacts` | Generate from coordinate |
| `run_hypercube_slots_mode` | Run in hypercube mode |
| `get_slots_config_options` | Get config options |

## GUI Integration

### Slots Dashboard (`/slots`)

Features:
- **Engine Mode** - Run slots with configurable entropy
- **Mining Mode** - Sequence mining visualization
- **Hypercube Integration** - Artifact generation

### Components

- `SlotMachineVisualizer.svelte` - Animated slot display
- `Coord5DDisplay.svelte` - 5D coordinate radar chart

## Hypercube Integration

The slots engine integrates with the hypercube framework:

### SlotArtifact
Represents a mined sequence as a hypercube artifact:
```rust
pub struct SlotArtifact {
    pub id: String,
    pub name: String,
    pub sequence: MinedSequence,
    pub coordinate: Coord5D,
    pub resonance: f64,
    pub source_node: Option<String>,
}
```

### SlotsHypercubeAdapter
Bridges slots and hypercube:
```rust
let mut adapter = SlotsHypercubeAdapter::default_adapter();

// Generate from coordinate
let artifacts = adapter.generate_from_coord(coord)?;

// Generate for HDAG
let hdag_artifacts = adapter.generate_for_hdag(&hdag)?;
```

### HypercubeSlotsMode
Combined execution mode:
```rust
let mut mode = HypercubeSlotsMode::new();
mode.set_coordinate(Coord5D::new(0.7, 0.6, 0.5, 0.4, 0.3));
let artifacts = mode.execute()?;
```

## Example Usage

```rust
use qops_slots::{
    SlotsSession, SlotsSessionConfig,
    SequenceMiner, MinerConfig, MiningStrategy,
    EntropyConfig, EntropyDistribution,
};

// Configure entropy
let entropy_config = EntropyConfig {
    distribution: EntropyDistribution::ResonanceOptimized,
    seed: Some(42),
    ..Default::default()
};

// Configure mining
let miner_config = MinerConfig {
    depth: 20,
    strategy: MiningStrategy::BeamSearch,
    target_resonance: 0.85,
    beam_width: 10,
    ..Default::default()
};

// Create session
let config = SlotsSessionConfig {
    entropy_config,
    miner_config,
    spins_before_mine: 10,
    ..Default::default()
};

let mut session = SlotsSession::new(config);
let result = session.run()?;

println!("Best resonance: {:.4}", result.best_resonance);
println!("Top sequence: {:?}", result.best_sequence);
```

## Sequence to 5D Mapping

Mined sequences map to 5D coordinates:

```rust
impl MinedSequence {
    pub fn to_hypercube_coord(&self) -> Coord5D {
        let psi = self.avg_value_for_symbol(SlotSymbol::Psi);
        let rho = self.avg_value_for_symbol(SlotSymbol::Rho);
        let omega = self.avg_value_for_symbol(SlotSymbol::Omega);
        let chi = self.avg_value_for_symbol(SlotSymbol::Chi);
        let eta = self.avg_value_for_symbol(SlotSymbol::Eta);

        Coord5D::new(psi, rho, omega, chi, eta)
    }
}
```

## Configuration Options

### SlotsSessionConfig
```rust
pub struct SlotsSessionConfig {
    pub entropy_config: EntropyConfig,
    pub miner_config: MinerConfig,
    pub spins_before_mine: usize,
    pub auto_mine: bool,
    pub export_results: bool,
}
```

### MinerConfig
```rust
pub struct MinerConfig {
    pub depth: usize,
    pub strategy: MiningStrategy,
    pub target_resonance: f64,
    pub beam_width: usize,
    pub population_size: usize,
    pub mutation_rate: f64,
    pub max_iterations: usize,
    pub convergence_threshold: f64,
}
```

## Performance Considerations

- **Beam width**: Higher values = better results, more memory
- **Depth**: Longer sequences = more computation
- **Population size**: For evolutionary strategy
- **Entropy seed**: Set for reproducibility

## Mathematical Foundation

### Sequence Space
The space of all possible sequences of length n over k symbols:
```
|S| = k^n
```

For 8 symbols and depth 10: 8^10 = 1,073,741,824 possible sequences.

### Optimization Objective
```
maximize R(S) = sum(w_i * v_i) / n
subject to: S in valid_sequences
```

### Mining Complexity
- Greedy: O(n * k) per sequence
- Beam: O(n * k * beam_width)
- Evolutionary: O(generations * population * n)