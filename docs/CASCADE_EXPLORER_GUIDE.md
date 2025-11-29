# FUQ! Cascade Explorer Guide

The Cascade Explorer visualizes the dimensional collapse process from the FUQ! (Full Unitary Quantum) algorithm, showing how high-dimensional hypercubes collapse through successive projections.

## Dimensional Transitions

The cascade follows Algorithm 1 from the FUQ! paper:

```
Hⁿ → H⁵ → H³ → H² → H¹
```

### Stage Breakdown

| Transition | Color | Description |
|------------|-------|-------------|
| Hⁿ → H⁵ | Purple | Initial projection from arbitrary n |
| H⁵ → H³ | Blue | Core 5D to 3D reduction |
| H³ → H² | Orange | Planarity emergence |
| H² → H¹ | Gold | Final eigenvalue extraction |

## Visual Elements

### State Cloud

The particle cloud represents the quantum state distribution:

- **Dense cloud** = High-dimensional state
- **Compressed cloud** = Lower dimensional projection
- Particles move toward center as dimensions collapse

### Dimensional Axes

Radial lines emanating from center:

- Number of axes = current dimension
- Axes shrink as dimension decreases
- Endpoint markers show axis termination

### Eigenvalue Bars

Bottom-right bar chart shows eigenvalue collapse:

- 5 bars initially (one per dimension)
- Bars shrink proportionally
- Invisible bars = collapsed dimensions

### Boundary Circle

Dashed circle indicating dimensional bounds:

- Radius proportional to dimension
- Color matches current transition
- Shrinks smoothly during animation

## Controls

### Playback Controls

| Button | Action |
|--------|--------|
| ⟲ | Reset to H⁵ |
| ◀ | Previous stage |
| ▶/‖ | Play/Pause animation |
| ▶ | Next stage |

### Progress Bar

Four-segment progress indicator:
- Each segment = one transition
- Filled = completed
- Gradient = in progress
- Empty = pending

## Animation Details

### State Cloud Behavior

During collapse:
1. Particles accelerate toward center
2. Velocity constrained by shrinking bounds
3. Bounce off boundary (dissipation effect)
4. Size decreases with dimension

### Eigenvalue Animation

- Bars animate in sync with dimension
- Height: `value × 60 × (dim/5)`
- Opacity: Full if `i < dim`, else 0.2

### Color Transitions

Colors blend smoothly between stages using the `interpolateColor` function from the color system.

## Interpretation Guide

### High Resonance Indicators

- **Fast convergence** = Well-conditioned initial state
- **Smooth particle motion** = Stable eigenspectrum
- **Uniform compression** = Balanced dimensional weights

### Warning Signs

- **Erratic particles** = Numerical instability
- **Asymmetric collapse** = Dimensional bias
- **Stalled progress** = Degenerate eigenvalues

## Integration with Hypercube Studio

The Cascade Explorer receives data from the Hypercube Studio:

```typescript
interface CascadeData {
  currentDimension: number;
  eigenvalues: number[];
  statePositions: [number, number, number][];
  transitionProgress: number;
}
```

## Mathematical Background

### Eigenvalue Flow

As dimension d decreases from n to 1:

```
λ_i(d) = λ_i(n) × exp(-α(n-d))
```

Where α is the collapse rate parameter.

### State Compression

The state cloud volume scales as:

```
V(d) ∝ d^(d/2)
```

Creating the visual compression effect.

### Resonance Preservation

The FUQ! algorithm preserves topological invariants:

- Chern number remains constant
- Berry phase accumulates predictably
- Winding number quantized

## Example Session

1. **Initialize** with 5D hypercube result
2. **Play** animation at normal speed
3. **Pause** at H³ to examine planarity
4. **Observe** eigenvalue bar collapse
5. **Note** final H¹ eigenvalue (resonance score)

## Advanced Features

### Custom Collapse Rates

Adjust animation speed per transition:

```svelte
<CascadePlayer
  currentDimension={5}
  eigenvalues={[0.9, 0.7, 0.5, 0.3, 0.1]}
  onDimensionChange={(dim) => console.log('Now at H' + dim)}
/>
```

### State Export

Export collapse states for analysis:
- Intermediate eigenvalue snapshots
- Particle position histories
- Transition timing data
