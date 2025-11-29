# Topology Dashboard Guide

The Topology Dashboard provides real-time visualization of topological invariants from the Holistic Resonance Architecture.

## Components

### 1. Chern Gauge

**Purpose:** Displays the Chern number, a topological invariant indicating the "winding" of the quantum state around the parameter space.

**Visual Design:**
- Circular gauge (0 to 1 scale)
- Arc fills clockwise from top
- Color gradient: Red → Amber → Gold
- Perfect topology (≥0.99) triggers pulsing glow

**Equations:**

The Chern number C is computed as:

```
C = (1/2π) ∮ F dA
```

Where F is the Berry curvature integrated over the Brillouin zone.

**Interpretation:**
| Value | Meaning |
|-------|---------|
| 0 | Trivial topology |
| 0.5 | Intermediate phase |
| 1.0 | Perfect topological state |

### 2. Berry Phase Compass

**Purpose:** Tracks accumulated geometric phase during adiabatic evolution.

**Visual Design:**
- Compass rose with rotating vector
- Phase displayed in units of π
- History trail shows past trajectory
- 2π crossing triggers emerald glow

**Equations:**

Berry phase γ:

```
γ = i ∮ ⟨ψ|∇_R|ψ⟩ · dR
```

Accumulated over the parameter path R.

**Interpretation:**
| Phase | State |
|-------|-------|
| 0 | Starting point |
| π | Half-cycle |
| 2π | Complete cycle (quantized) |
| n×2π | Multiple windings |

### 3. Resonance Heatmap

**Purpose:** 2D visualization of resonance field intensity across the parameter space.

**Visual Design:**
- Grid of colored cells
- Cell size configurable
- Color mapping by stage:
  - Blue: Kosmokrator (< 0.3)
  - Violet: Chronokrator (0.3-0.6)
  - Amber: Pfauenthron (0.6-0.85)
  - Emerald: Finalized (> 0.85)

**Equations:**

Resonance R at coordinate (ψ, ρ, ω, χ, η):

```
R = Σᵢ wᵢ × fᵢ(σᵢ)
```

Where fᵢ are feature functions and wᵢ are learned weights.

### 4. Stability Meter

**Purpose:** Indicates execution stability based on Jacobian norms.

**Visual Design:**
- Horizontal bar gauge
- Three zones:
  - Red (< 50%): Unstable
  - Amber (50-80%): Moderate
  - Green (> 80%): Stable
- Threshold markers at 50% and 80%

**Equations:**

Stability S from Jacobian J:

```
S = 1 / (1 + ||J||_F)
```

Where ||J||_F is the Frobenius norm.

**Interpretation:**
| Stability | Implication |
|-----------|-------------|
| < 0.5 | Numerical issues likely |
| 0.5-0.8 | Acceptable but monitor |
| > 0.8 | Robust execution |

## Visualization Logic

### Color Mapping Function

```typescript
function getResonanceColor(value: number): string {
  if (value < 0.3) return interpolate(BLUE, value / 0.3);
  if (value < 0.6) return interpolate(VIOLET, (value - 0.3) / 0.3);
  if (value < 0.85) return interpolate(AMBER, (value - 0.6) / 0.25);
  return interpolate(EMERALD, (value - 0.85) / 0.15);
}
```

### Animation Timing

| Component | Update Rate | Transition |
|-----------|-------------|------------|
| Chern Gauge | 30 fps | 800ms ease |
| Berry Compass | 60 fps | Continuous |
| Heatmap | On data | Instant |
| Stability | 30 fps | 500ms ease |

## Integration

### Data Requirements

```typescript
interface TopologyData {
  chern_number: number;      // 0-1
  berry_phase: number;       // radians
  resonance_field: number[]; // Grid values
  jacobian_norm: number;     // Stability metric
}
```

### Usage Example

```svelte
<script>
  import {
    ChernGauge,
    BerryPhaseCompass,
    ResonanceHeatmap,
    StabilityMeter
  } from '$lib/visual';

  export let topologyData;
</script>

<div class="grid grid-cols-2 gap-4">
  <ChernGauge value={topologyData.chern_number} />
  <BerryPhaseCompass phase={topologyData.berry_phase} />
  <ResonanceHeatmap data={topologyData.resonance_field} />
  <StabilityMeter
    value={1 / (1 + topologyData.jacobian_norm)}
    jacobianNorm={topologyData.jacobian_norm}
  />
</div>
```

## Scientific Context

### Holistic Resonance Architecture

The dashboard components map to the HRA stages:

1. **Kosmokrator** → Initial filtering (blue zone)
2. **Chronokrator** → Temporal expansion (violet zone)
3. **Pfauenthron** → Decision collapse (amber zone)
4. **Finalized** → Monolith emission (emerald zone)

### Topological Protection

Topological invariants provide:
- **Robustness** against perturbations
- **Quantization** of observables
- **Phase transition** detection

The dashboard visualizes these properties in real-time to guide exploration and identify stable quantum configurations.
