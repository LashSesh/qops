# Hypercube Studio Guide

The Hypercube Studio is a multi-panel scientific workbench for exploring the 5D Self-Compiling Cube Framework.

## Overview

The studio provides four main operational modes:

1. **Session Mode** - Full hypercube exploration session
2. **Compile Mode** - Single-shot compilation with TRITON
3. **Expand Mode** - Step-by-step expansion with different rules
4. **HDAG Mode** - Hierarchical DAG execution visualization

## Panels

### Panel 1: Hypercube Projection

The main 3D projection panel shows the hypercube state cloud:

- **Vertices** - Colored nodes representing 5D coordinates
- **Edges** - Connections between adjacent vertices
- **Animation** - Continuous rotation for depth perception

**Color Mapping:**
| Resonance | Color |
|-----------|-------|
| < 0.3 | Blue |
| 0.3-0.6 | Violet |
| 0.6-0.85 | Amber |
| > 0.85 | Emerald |

**Controls:**
- Click vertex to select and view details
- Toggle labels to show resonance values
- Enable/disable animation rotation

### Panel 2: HDAG Layer Navigator

Visualizes the Hierarchical Directed Acyclic Graph:

- **Input nodes** (Blue) - Entry points
- **Operator nodes** (Violet) - DK/SC/PI/WT operators
- **Merge nodes** (Amber) - Combination points
- **Compilation nodes** (Red) - ξ compilation
- **Output nodes** (Emerald) - Results

**Node Animation:**
- Executing node: Pulsing yellow ring
- Completed node: Green checkmark

### Panel 3: Operator Timeline

Interactive timeline showing operator application sequence:

- DK - Purple pulse effect
- SC - Orange flow-wave
- PI - Blue integrator line

Click any operator to view:
- Input/Output states
- 2D plane projections (ψ-ρ, ω-χ)
- Vector rotation visualization

### Panel 4: Convergence Graph

Tracks convergence over iterations:

- **Main Line** - Value over iterations
- **Threshold** - Target resonance line
- **Bright Spots** - Resonant transitions
- **Convergence Indicator** - Green checkmark when stable

## Controls

### Seed Coordinate

Five sliders control the initial 5D seed:

| Dimension | Range | Description |
|-----------|-------|-------------|
| ψ (psi) | 0-1 | Primary amplitude |
| ρ (rho) | 0-1 | Phase coupling |
| ω (omega) | 0-1 | Frequency modulation |
| χ (chi) | 0-1 | Coherence factor |
| η (eta) | 0-1 | Entanglement weight |

### Session Presets

| Preset | Agents | Steps | Description |
|--------|--------|-------|-------------|
| Quick | 5 | 20 | Fast exploration |
| Thorough | 10 | 50 | Balanced search |
| Research | 20 | 100 | Deep investigation |

### Expansion Rules

| Rule | Description |
|------|-------------|
| Lattice | Grid-based expansion |
| Resonance | Follow high-resonance paths |
| TRITON | Golden spiral search |
| Operator | DK/SC/PI-driven exploration |
| Random | Stochastic sampling |
| Hybrid | Combined strategies |

## Visual Modes

### Standard View
Default 2D projection with rotation animation.

### Sliced View
Dimensional slicing to view specific hyperplanes.

### Heatmap View
Resonance intensity as color gradient overlay.

### Network View
Graph-based connectivity visualization.

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Space | Play/Pause animation |
| R | Reset to initial state |
| T | Toggle TRITON mode |
| L | Toggle labels |
| 1-4 | Switch panels |

## Best Practices

1. **Start with Quick preset** to get initial exploration
2. **Identify high-resonance regions** using heatmap view
3. **Use TRITON mode** for focused optimization
4. **Analyze HDAG execution** to understand operator effects
5. **Monitor convergence** for stability assessment

## Example Workflow

```
1. Set seed to center (0.5, 0.5, 0.5, 0.5, 0.5)
2. Run Quick session to explore space
3. Identify best coordinate from results
4. Use as new seed and run Thorough session
5. Enable HDAG mode to visualize operator flow
6. Export results for further analysis
```
