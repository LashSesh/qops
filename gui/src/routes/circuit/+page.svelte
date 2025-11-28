<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  interface Gate {
    name: string;
    gate_type: string;
    qubits: number[];
    parameter: number | null;
  }

  interface Circuit {
    id: string;
    name: string;
    qubits: number;
    depth: number;
    gate_count: number;
    gates: Gate[];
  }

  interface SimulationResult {
    probabilities: number[];
    counts: Record<string, number>;
    shots: number;
  }

  let qubits = 3;
  let circuit: Circuit | null = null;
  let simulationResult: SimulationResult | null = null;
  let selectedGate = 'h';
  let selectedQubit = 0;
  let parameter: number | null = null;
  let loading = false;
  let error: string | null = null;

  const gateTypes = [
    { id: 'h', label: 'H', color: 'gate-h' },
    { id: 'x', label: 'X', color: 'gate-x' },
    { id: 'y', label: 'Y', color: 'gate-y' },
    { id: 'z', label: 'Z', color: 'gate-z' },
    { id: 't', label: 'T', color: 'gate-t' },
    { id: 's', label: 'S', color: 'gate-s' },
    { id: 'cnot', label: 'CNOT', color: 'gate-cnot' },
  ];

  async function createCircuit() {
    loading = true;
    error = null;
    try {
      circuit = await invoke<Circuit>('create_circuit', { qubits, name: 'my_circuit' });
      simulationResult = null;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function addGate() {
    if (!circuit) return;
    loading = true;
    error = null;
    try {
      const qubitsArr = selectedGate === 'cnot'
        ? [selectedQubit, (selectedQubit + 1) % circuit.qubits]
        : [selectedQubit];
      circuit = await invoke<Circuit>('add_gate', {
        circuitId: circuit.id,
        gateType: selectedGate,
        qubits: qubitsArr,
        parameter,
      });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function removeGate(index: number) {
    if (!circuit) return;
    loading = true;
    error = null;
    try {
      circuit = await invoke<Circuit>('remove_gate', {
        circuitId: circuit.id,
        gateIndex: index,
      });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function simulateCircuit() {
    if (!circuit) return;
    loading = true;
    error = null;
    try {
      simulationResult = await invoke<SimulationResult>('simulate_circuit', {
        circuitId: circuit.id,
        shots: 1000,
      });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function formatBinary(n: number, width: number): string {
    return n.toString(2).padStart(width, '0');
  }
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <!-- Header -->
  <div class="p-4 border-b border-surface-700 flex items-center justify-between">
    <h1 class="text-xl font-bold text-white">Circuit Builder</h1>
    <div class="flex items-center gap-4">
      <div class="flex items-center gap-2">
        <label class="text-slate-400 text-sm">Qubits:</label>
        <input type="number" bind:value={qubits} min="1" max="10" class="input w-20" />
      </div>
      <button on:click={createCircuit} class="btn-primary" disabled={loading}>
        New Circuit
      </button>
      {#if circuit}
        <button on:click={simulateCircuit} class="btn-accent" disabled={loading}>
          Simulate
        </button>
      {/if}
    </div>
  </div>

  <div class="flex-1 flex overflow-hidden">
    <!-- Gate Palette -->
    <div class="w-48 bg-surface-800 border-r border-surface-700 p-4">
      <h3 class="text-sm font-semibold text-slate-400 mb-3">Gates</h3>
      <div class="grid grid-cols-2 gap-2">
        {#each gateTypes as gate}
          <button
            on:click={() => selectedGate = gate.id}
            class="h-12 rounded-lg flex items-center justify-center font-mono font-bold text-white transition-all {gate.color}"
            class:ring-2={selectedGate === gate.id}
            class:ring-white={selectedGate === gate.id}
          >
            {gate.label}
          </button>
        {/each}
      </div>

      <div class="mt-4">
        <label class="text-sm text-slate-400 block mb-2">Target Qubit:</label>
        <input type="number" bind:value={selectedQubit} min="0" max={circuit ? circuit.qubits - 1 : 9} class="input w-full" />
      </div>

      {#if circuit}
        <button on:click={addGate} class="btn-primary w-full mt-4" disabled={loading}>
          Add Gate
        </button>
      {/if}
    </div>

    <!-- Circuit Canvas -->
    <div class="flex-1 p-4 overflow-auto">
      {#if error}
        <div class="bg-quantum-error/20 border border-quantum-error text-quantum-error p-4 rounded-lg mb-4">
          {error}
        </div>
      {/if}

      {#if circuit}
        <div class="card mb-4">
          <div class="flex items-center justify-between mb-4">
            <h2 class="font-semibold text-white">{circuit.name}</h2>
            <div class="text-sm text-slate-400">
              Qubits: {circuit.qubits} | Depth: {circuit.depth} | Gates: {circuit.gate_count}
            </div>
          </div>

          <!-- Circuit visualization -->
          <div class="bg-surface-900 rounded-lg p-4 font-mono text-sm overflow-x-auto">
            {#each Array(circuit.qubits) as _, q}
              <div class="flex items-center gap-2 mb-2">
                <span class="text-slate-400 w-8">q{q}:</span>
                <div class="flex items-center">
                  <span class="text-slate-600">|0&gt;--</span>
                  {#each circuit.gates as gate, i}
                    {#if gate.qubits.includes(q)}
                      <button
                        on:click={() => removeGate(i)}
                        class="px-2 py-1 rounded text-white text-xs mx-1 hover:opacity-80 gate-{gate.name.toLowerCase().replace(/[^a-z]/g, '')}"
                        title="Click to remove"
                      >
                        {gate.name}
                      </button>
                      <span class="text-slate-600">--</span>
                    {:else}
                      <span class="text-slate-600">------</span>
                    {/if}
                  {/each}
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Simulation Results -->
        {#if simulationResult}
          <div class="card">
            <h3 class="font-semibold text-white mb-4">Simulation Results ({simulationResult.shots} shots)</h3>
            <div class="space-y-2">
              {#each Object.entries(simulationResult.counts).sort((a, b) => b[1] - a[1]).slice(0, 10) as [state, count]}
                <div class="flex items-center gap-4">
                  <span class="font-mono text-slate-300 w-24">|{state}&gt;</span>
                  <div class="flex-1 h-6 bg-surface-700 rounded overflow-hidden">
                    <div
                      class="h-full bg-quantum-primary"
                      style="width: {(count / simulationResult.shots) * 100}%"
                    ></div>
                  </div>
                  <span class="text-slate-400 text-sm w-20 text-right">
                    {((count / simulationResult.shots) * 100).toFixed(1)}%
                  </span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      {:else}
        <div class="flex items-center justify-center h-full text-slate-400">
          Click "New Circuit" to start building
        </div>
      {/if}
    </div>
  </div>
</div>
