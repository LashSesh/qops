# QOPS Visual System Overview

This document provides a comprehensive overview of the QOPS visualization infrastructure, designed for the Quantum Operator Processing System's exploration and research capabilities.

## Architecture

The visualization system is built on a modular architecture combining:

- **SvelteKit** - Frontend framework for reactive UI components
- **SVG/Canvas** - 2D rendering for plots, gauges, and diagrams
- **CSS Animations** - Smooth transitions and effects
- **TypeScript** - Type-safe component development

### Directory Structure

```
gui/src/lib/
├── visual/                  # Core visualization library
│   ├── index.ts            # Main exports
│   ├── colors.ts           # Adaptive color system
│   ├── animations.ts       # Animation utilities
│   ├── ChernGauge.svelte   # Topology gauge
│   ├── BerryPhaseCompass.svelte  # Phase visualization
│   ├── StabilityMeter.svelte     # Stability bar
│   ├── CascadePlayer.svelte      # Dimensional collapse
│   ├── OperatorFlow.svelte       # Operator timeline
│   ├── TopologyPlot.svelte       # Phase space plot
│   ├── VectorField.svelte        # Gradient field
│   └── ConvergenceGraph.svelte   # Convergence tracking
├── components/              # Page-level components
│   ├── HypercubeCanvas.svelte    # 5D projection
│   ├── ResonanceHeatmap.svelte   # Resonance grid
│   ├── SlotMachineVisualizer.svelte  # Slots UI
│   ├── HDAGGraph.svelte          # DAG visualization
│   └── SpiralTrajectory.svelte   # TRITON spiral
└── tauri/                   # Tauri command bindings
    └── commands.ts          # Backend communication
```

## Visual Engine Map

### Color System (`colors.ts`)

The color system follows the Holistic Resonance Architecture:

| Stage | Color | Hex |
|-------|-------|-----|
| Discovery | Blue | `#3b82f6` |
| Kosmokrator | Sky Blue | `#00bfff` |
| Chronokrator | Violet | `#9932cc` |
| Pfauenthron | Gold | `#ffd700` |
| Finalized | Emerald | `#10b981` |

**Operator Colors:**
- **DK** (Double Kick): Purple `#a78bfa`
- **SC** (Swap/Cycle): Orange `#fb923c`
- **PI** (Path Integration): Blue `#3b82f6`
- **WT** (Weight Transform): Teal `#14b8a6`

### Animation System (`animations.ts`)

Provides easing functions and animation utilities:

```typescript
import { easing, tween, createPulse } from '$lib/visual/animations';

// Smooth value transition
tween(0, 1, 800, easing.easeOutCubic, (value) => {
  displayValue = value;
});

// Pulsing animation
const pulse = createPulse(1.0, 1.1, 2);
scale = pulse(elapsedTime);
```

## Component Map

### Topology Dashboard Components

1. **ChernGauge** - Circular gauge showing Chern number (0-1)
   - Full circle = perfect topology
   - Glow effect at ≥0.99

2. **BerryPhaseCompass** - Polar plot for Berry phase
   - Rotating vector with phase accumulation
   - 2π detection triggers glow animation

3. **StabilityMeter** - Horizontal bar for execution stability
   - Based on Jacobian norms
   - Color-coded zones (red < 50%, amber 50-80%, green > 80%)

### Cascade Visualization

4. **CascadePlayer** - Animated dimensional collapse
   - Hⁿ → H⁵ → H³ → H² → H¹ transitions
   - Particle cloud compression
   - Eigenvalue bar collapse

5. **ConvergenceGraph** - Convergence line plot
   - Fixed-point detection
   - Resonant transitions as bright spots

### Operator Visualization

6. **OperatorFlow** - Timeline of DK/SC/PI operators
   - Clickable nodes with effect details
   - 2D plane projections (ψ-ρ, ω-χ)

7. **VectorField** - 2D gradient flow visualization
   - Arrow field with magnitude coloring

### Phase Space

8. **TopologyPlot** - Phase trajectory visualization
   - Chern, Berry, Winding numbers display

## Usage Example

```svelte
<script>
  import {
    ChernGauge,
    BerryPhaseCompass,
    StabilityMeter,
    CascadePlayer
  } from '$lib/visual';

  let chernNumber = 0.95;
  let berryPhase = Math.PI;
  let stability = 0.85;
</script>

<div class="topology-dashboard">
  <ChernGauge value={chernNumber} size={150} />
  <BerryPhaseCompass phase={berryPhase} size={150} />
  <StabilityMeter value={stability} width={200} />
  <CascadePlayer currentDimension={5} isPlaying={true} />
</div>
```

## Design Principles

1. **Dark Mode First** - All components optimized for dark backgrounds
2. **Performance** - SVG-based rendering with efficient updates
3. **Interactivity** - Clickable elements with hover states
4. **Animation** - Smooth transitions using CSS and requestAnimationFrame
5. **Responsiveness** - Size props for flexible layouts

## Backend Integration

Components receive data via Tauri commands:

```typescript
// From Rust backend
interface VisualizationData {
  state_positions: [number, number, number][];
  eigenvalues: number[];
  chern_number: number;
  berry_phase: number;
  resonance_field: number[];
  cascade_states: CubeState[];
}

// UX metadata
interface UXMetadata {
  dominant_color: string;
  collapse_intensity: number;
  resonance_peaks: number[];
  operator_highlight: string;
}
```

## Future Extensions

- WebGL-based 3D hypercube renderer (Three.js)
- Real-time streaming visualization
- VR/AR support for immersive exploration
- Sound synthesis for resonance feedback
