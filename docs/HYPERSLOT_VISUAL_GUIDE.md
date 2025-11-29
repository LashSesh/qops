# HyperSlot Visual Guide

The HyperSlot Engine provides a gamified interface for quantum slots mining with visual feedback on resonance discovery.

## Overview

The slot machine metaphor makes sequence mining accessible:
- **Reels** = Dimensional operators
- **Symbols** = 5D coordinates (ψ, ρ, ω, χ, η)
- **Jackpot** = High-resonance discovery

## Reel Animations

### Spin Animation

During mining, reels display spinning motion:

```
┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐
│  ψ  │ │  ρ  │ │  ω  │ │  χ  │ │  η  │
│ ↕↕↕ │ │ ↕↕↕ │ │ ↕↕↕ │ │ ↕↕↕ │ │ ↕↕↕ │
└─────┘ └─────┘ └─────┘ └─────┘ └─────┘
```

**Animation Phases:**
1. **Acceleration** - Reels speed up (ease-in)
2. **Full Spin** - Maximum velocity
3. **Deceleration** - Gradual slowdown (ease-out-expo)
4. **Settle** - Final symbol display

### Reel Mechanics

```typescript
interface ReelAnimation {
  offset: number;      // Current position (0-1)
  symbolIndex: number; // Displayed symbol
  velocity: number;    // Spin speed
  stopping: boolean;   // Deceleration phase
}
```

## Symbol System

### Core Symbols

| Symbol | Color | Weight | Description |
|--------|-------|--------|-------------|
| ψ (psi) | Blue | +0.40 | Primary amplitude |
| ρ (rho) | Violet | +0.30 | Phase coupling |
| ω (omega) | Amber | +0.30 | Frequency |
| χ (chi) | Cyan | +0.05 | Coherence |
| η (eta) | Red | -0.05 | Damping |

### Bonus Symbols

| Symbol | Color | Weight | Trigger |
|--------|-------|--------|---------|
| ⭐ Star | Gold | +0.10 | Free spin |
| 💎 Diamond | Purple | +0.15 | Multiplier |
| ⬤ Circle | Emerald | +0.05 | Respin |

## Scoring Visualization

### Resonance Display

Large central display shows current resonance:

```
         ╔═══════════╗
         ║   85.7%   ║
         ║ Resonance ║
         ╚═══════════╝
```

**Color Coding:**
- Red (< 50%): Low resonance
- Amber (50-80%): Medium resonance
- Emerald (> 80%): High resonance

### Win Line

Horizontal gradient bar below reels:
- Pulses brighter on high scores
- Color matches resonance tier

## Result Cards

### Standard Result

```
┌──────────────────────────────┐
│ ⬡ Sequence #42              │
├──────────────────────────────┤
│ Symbols: ψ ρ ω ψ ρ          │
│ Resonance: 0.8234           │
│ Depth: 3                    │
│ Coord: (0.72, 0.65, 0.58,   │
│         0.41, 0.33)         │
├──────────────────────────────┤
│ [Open in Hypercube Studio]  │
└──────────────────────────────┘
```

### Card Features

- **Small hypercube preview** - 2D projection canvas
- **Score heatmap** - Dimensional contribution bars
- **Topology values** - Chern/Berry/Winding numbers
- **Cascade depth** - Discovery path length

## Jackpot Animation

### Trigger Conditions

Jackpot activates when:
- Chern number ≈ 1.0 (within 0.01)
- Berry phase ≈ 2π (within 0.1 rad)

### Animation Sequence

1. **Golden Flash** (0-500ms)
   - Screen-wide gold overlay
   - Rapid intensity pulsing (8 Hz)

2. **Collapse Animation** (500-2000ms)
   - Particle explosion from center
   - 720° rotation with ease-out
   - Scale bounce (1.0 → 1.3 → 1.0)

3. **Sound Pulse** (if enabled)
   - Ascending tone sweep
   - Resonance chord on completion

### Visual Effects

```typescript
const jackpotFlash = createJackpotFlash(3000);

// Each frame:
const { intensity, scale, rotation } = jackpotFlash(elapsed);
element.style.filter = `brightness(${1 + intensity})`;
element.style.transform = `scale(${scale}) rotate(${rotation}deg)`;
```

## Mining Statistics

### Session Stats Panel

```
┌────────────────────────────────┐
│ Spins: 127    │ Time: 4.2s    │
├────────────────────────────────┤
│ Best: 0.9123  │ Converged: ✓  │
│ Steps: 89     │ To Best: 42   │
└────────────────────────────────┘
```

### Top Sequences Table

Scrollable list of best discoveries:
- Rank number (#1, #2, etc.)
- Symbol sequence preview (first 5)
- Resonance score (green highlight)

## Integration

### With Hypercube Studio

"Open in Hypercube Studio" button:
1. Serializes sequence to 5D coordinate
2. Navigates to Studio page
3. Pre-loads coordinate as seed
4. Initiates session with discovery context

### Backend Commands

```typescript
// Run slots engine
const result = await runSlotsEngine(
  steps: 50,
  entropy: 'resonance',
  strategy: 'beam',
  targetResonance: 0.8
);

// Mine specific sequence
const mined = await slotsMineSequence(
  depth: 10,
  strategy: 'genetic',
  target: 0.9,
  beamWidth: 20
);
```

## Configuration

### Entropy Distributions

| Distribution | Description |
|--------------|-------------|
| Uniform | Equal probability |
| Stochastic | Gaussian sampling |
| Resonance | Optimized for high values |
| Bimodal | Two-peak distribution |

### Mining Strategies

| Strategy | Best For |
|----------|----------|
| Greedy | Quick exploration |
| Beam | Balanced search |
| Genetic | Deep optimization |
| Random | Baseline comparison |

## Tips for High Scores

1. **Use resonance-optimized entropy** for better initial samples
2. **Increase beam width** for thorough search
3. **Target 0.85+ resonance** for Pfauenthron tier
4. **Monitor convergence** to know when to stop
5. **Export jackpot results** to Hypercube Studio for analysis
