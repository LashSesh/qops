# QOPS Frontend Components Reference

Complete reference for all Svelte components in the QOPS desktop application.

## Layout Components

### `+layout.svelte`

Main application layout with sidebar navigation.

**Location:** `src/routes/+layout.svelte`

**Structure:**
```
┌─────────────────────────────────────────────────────┐
│  [Logo] QOPS            [System Status] [Settings]  │
├──────────┬──────────────────────────────────────────┤
│          │                                          │
│  Nav     │           <slot />                       │
│  Items   │           (Page Content)                 │
│          │                                          │
│          │                                          │
└──────────┴──────────────────────────────────────────┘
```

**Navigation Items:**
- Dashboard (`/`)
- Circuits (`/circuit`)
- Algorithms (`/algorithm`)
- Topology (`/topology`)
- Resonance (`/resonance`)
- Settings (`/settings`)

**CSS Classes:**
- `.nav-item` - Navigation link
- `.nav-item.active` - Active navigation link

---

## Page Components

### Dashboard (`+page.svelte`)

**Location:** `src/routes/+page.svelte`

**Features:**
- System information display
- Quick action buttons
- Module status overview
- Capabilities list

**State:**
```typescript
let systemInfo: SystemInfoDto | null = null;
let loading = false;
let error: string | null = null;
```

**Sections:**
1. **Welcome Banner** - Title and description
2. **Quick Actions** - Buttons to run common algorithms
3. **Modules Grid** - List of available QOPS modules
4. **Capabilities** - Feature list

---

### Circuit Builder (`/circuit/+page.svelte`)

**Location:** `src/routes/circuit/+page.svelte`

**Features:**
- Create new circuits
- Add/remove quantum gates
- Visual circuit representation
- Simulate and view results
- Export to QASM

**State:**
```typescript
let circuits: CircuitDto[] = [];
let selectedCircuit: CircuitDto | null = null;
let simulationResult: SimulationResultDto | null = null;
let newCircuitQubits = 3;
let newCircuitName = '';
```

**Sections:**

1. **Header** - Title and "New Circuit" button
2. **Controls Panel** (left sidebar):
   - Gate palette (H, X, Y, Z, S, T, CNOT, CZ, Toffoli)
   - Circuit selector dropdown
   - Qubit count input
   - Shots input
   - Simulate button
3. **Circuit Canvas** (main area):
   - Gate visualization
   - Click to remove gates
4. **Results Panel** - Probability histogram

**Gate Palette:**
```typescript
const gates = [
  { name: 'H', type: 'H', qubits: 1, description: 'Hadamard' },
  { name: 'X', type: 'X', qubits: 1, description: 'Pauli-X (NOT)' },
  { name: 'Y', type: 'Y', qubits: 1, description: 'Pauli-Y' },
  { name: 'Z', type: 'Z', qubits: 1, description: 'Pauli-Z' },
  { name: 'S', type: 'S', qubits: 1, description: 'Phase (π/2)' },
  { name: 'T', type: 'T', qubits: 1, description: 'T-gate (π/4)' },
  { name: 'CNOT', type: 'CNOT', qubits: 2, description: 'Controlled-NOT' },
  { name: 'CZ', type: 'CZ', qubits: 2, description: 'Controlled-Z' },
  { name: 'CCX', type: 'Toffoli', qubits: 3, description: 'Toffoli' },
];
```

---

### Algorithm Runner (`/algorithm/+page.svelte`)

**Location:** `src/routes/algorithm/+page.svelte`

**Features:**
- Run multiple quantum algorithms
- Configure algorithm parameters
- View results with visualizations

**Supported Algorithms:**

| Algorithm | Parameters | Result Type |
|-----------|------------|-------------|
| Grover | qubits, target, shots | GroverResultDto |
| Shor | number | ShorResultDto |
| QFT | qubits, inputState | QftResultDto |
| QPE | precision, phase, shots | QpeResultDto |
| VQE | qubits, layers, maxIterations | VqeResultDto |
| QAOA | edges, layers, shots | QaoaResultDto |

**State:**
```typescript
let selectedAlgorithm = 'grover';
let loading = false;
let error: string | null = null;
let result: unknown = null;

// Algorithm-specific params
let groverParams = { qubits: 4, target: 5, shots: 1000 };
let shorParams = { number: 15 };
let qftParams = { qubits: 4, inputState: 0 };
let qpeParams = { precision: 4, phase: 0.25, shots: 1000 };
let vqeParams = { qubits: 2, layers: 2, maxIterations: 100 };
let qaoaParams = { layers: 2, shots: 1000 };
```

**Result Displays:**
- **Grover**: Success probability, iterations, measurement histogram
- **Shor**: Factors found, period, attempts
- **QFT**: Circuit depth, gate count, probability distribution
- **QPE**: Estimated phase, error, confidence
- **VQE**: Ground energy, convergence status, parameters
- **QAOA**: Best solution, approximation ratio, solution distribution

---

### Topology Explorer (`/topology/+page.svelte`)

**Location:** `src/routes/topology/+page.svelte`

**Features:**
- S7 topology information
- Node inspection
- Genesis mining execution
- Artefact visualization

**State:**
```typescript
let topologyInfo: TopologyInfoDto | null = null;
let selectedNode: NodeDetailsDto | null = null;
let genesisResult: GenesisResultDto | null = null;
let loading = false;
let error: string | null = null;

// Genesis params
let agents = 5;
let steps = 20;
let strategy = 'balanced';
```

**Sections:**

1. **Controls Panel** (left sidebar):
   - Genesis Mining controls
     - Agents input (1-50)
     - Steps input (5-100)
     - Strategy dropdown
     - Run Mining button
   - Topology Info display
   - Node inspection input

2. **Main Content**:
   - **Mining Results** (when available):
     - Best Resonance metric
     - Mandorla count
     - Total steps
     - Artefacts list with resonance bars
   - **Node Details** (when selected):
     - Node ID
     - Neighbor count
     - Permutation array
     - Signature values (ψ, ρ, ω)

---

### Resonance Analyzer (`/resonance/+page.svelte`)

**Location:** `src/routes/resonance/+page.svelte`

**Features:**
- Seraphic calibration execution
- Quantum walk simulation
- Cube-13 topology info
- Signature visualization

**State:**
```typescript
let calibrationResult: CalibrationResultDto | null = null;
let quantumWalkResult: QuantumWalkResultDto | null = null;
let cube13Info: TopologyInfoDto | null = null;
let currentSignature: SignatureDto | null = null;

// Calibration params
let calibrationSteps = 50;
let targetResonance = 0.8;

// Quantum walk params
let walkTimes = [0, 0.5, 1, 1.5, 2, 2.5, 3];
```

**Sections:**

1. **Controls Panel** (left sidebar):
   - Calibration controls
     - Steps input
     - Target resonance input
     - Run Calibration button
   - Quantum Walk controls
     - Time points configuration
     - Run Walk button
   - Current Status display

2. **Main Content**:
   - **Calibration Results**:
     - Final signature (ψ, ρ, ω, χ, η)
     - Accepted steps count
     - Step-by-step table
   - **Quantum Walk Results**:
     - Probability evolution graph
     - Time series data

---

### Settings (`/settings/+page.svelte`)

**Location:** `src/routes/settings/+page.svelte`

**Features:**
- Application preferences
- Theme selection
- Default parameters
- About information

**Settings Categories:**
- **Simulation**: Default shots, max qubits
- **Algorithms**: Default parameters for each algorithm
- **Genesis**: Default agents, steps, strategy
- **Display**: Theme, animations, precision

---

## Shared UI Classes

### Card Component

```html
<div class="card">
  <h2 class="text-lg font-semibold text-white mb-4">Title</h2>
  <!-- Content -->
</div>
```

**CSS:**
```css
.card {
  @apply bg-surface-800 rounded-lg p-6 border border-surface-700;
}
```

---

### Button Variants

**Primary Button:**
```html
<button class="btn-primary">Action</button>
```

**Secondary Button:**
```html
<button class="btn-secondary">Action</button>
```

**CSS:**
```css
.btn-primary {
  @apply bg-quantum-primary hover:bg-quantum-primary/80
         text-white font-medium py-2 px-4 rounded-lg
         transition-colors disabled:opacity-50;
}

.btn-secondary {
  @apply bg-surface-700 hover:bg-surface-600
         text-white font-medium py-2 px-4 rounded-lg
         transition-colors;
}
```

---

### Input Fields

```html
<input type="number" class="input" />
<select class="input">...</select>
```

**CSS:**
```css
.input {
  @apply bg-surface-700 border border-surface-600
         text-white rounded-lg px-3 py-2
         focus:border-quantum-primary focus:outline-none;
}
```

---

### Status Indicators

**Loading Spinner:**
```html
<span class="animate-spin">@</span>
```

**Error Display:**
```html
<div class="bg-quantum-error/20 border border-quantum-error text-quantum-error p-4 rounded-lg">
  {error}
</div>
```

**Success Display:**
```html
<div class="bg-quantum-success/20 border border-quantum-success text-quantum-success p-4 rounded-lg">
  {message}
</div>
```

---

## Color System

Based on the quantum lab theme in `tailwind.config.js`:

| Token | Hex | Usage |
|-------|-----|-------|
| `quantum-primary` | `#6366F1` | Primary actions, links |
| `quantum-secondary` | `#8B5CF6` | Secondary elements |
| `quantum-accent` | `#06B6D4` | Highlights, accents |
| `quantum-success` | `#10B981` | Success states |
| `quantum-warning` | `#F59E0B` | Warning states |
| `quantum-error` | `#EF4444` | Error states |
| `surface-900` | `#0F172A` | Main background |
| `surface-800` | `#1E293B` | Cards, panels |
| `surface-700` | `#334155` | Elevated surfaces |
| `surface-600` | `#475569` | Hover states |

---

## Typography

**Font Families:**
- **Sans:** Inter, system-ui, sans-serif
- **Mono:** JetBrains Mono, Consolas, monospace

**Usage:**
```html
<!-- Headings -->
<h1 class="text-xl font-bold text-white">Page Title</h1>
<h2 class="text-lg font-semibold text-white">Section Title</h2>
<h3 class="text-sm font-semibold text-slate-400">Subsection</h3>

<!-- Body text -->
<p class="text-slate-400 text-sm">Description text</p>

<!-- Monospace values -->
<div class="font-mono text-quantum-primary">{value}</div>
```

---

## Responsive Behavior

The application is designed for desktop use with Tauri:
- Minimum window size: 800×600
- Recommended: 1200×800
- Sidebar collapses on smaller windows

---

## State Management

Each page manages its own local state using Svelte's reactive variables. For cross-page state:

```typescript
// Tauri commands handle all backend state
import { invoke } from '@tauri-apps/api/core';

// Backend maintains:
// - Circuit store (HashMap<String, Circuit>)
// - Calibration state
// - Topology cache
```

---

## Event Handling

**Button clicks:**
```svelte
<button on:click={handleAction}>Action</button>
```

**Input changes:**
```svelte
<input bind:value={param} />
<input on:change={(e) => handleChange(e.currentTarget.value)} />
```

**Select changes:**
```svelte
<select bind:value={selectedOption}>
  <option value="a">Option A</option>
</select>
```

---

## Animations

Defined in `tailwind.config.js`:

```javascript
animation: {
  'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
  'spin-slow': 'spin 3s linear infinite',
}
```

**Usage:**
```html
<div class="animate-pulse-slow">Pulsing element</div>
<div class="animate-spin-slow">Spinning element</div>
```
