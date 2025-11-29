<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  type Algorithm = 'grover' | 'shor' | 'qft' | 'qpe' | 'vqe' | 'qaoa';

  let selectedAlgorithm: Algorithm = 'grover';
  let loading = false;
  let error: string | null = null;
  let result: any = null;

  // Grover params
  let groverQubits = 3;
  let groverTarget = 5;
  let groverShots = 1000;

  // Shor params
  let shorNumber = 15;

  // QFT params
  let qftQubits = 3;

  // QPE params
  let qpeQubits = 4;
  let qpePhase = 0.25;

  // VQE params
  let vqeQubits = 2;
  let vqeLayers = 2;

  // QAOA params
  let qaoaQubits = 4;
  let qaoaLayers = 2;

  const algorithms: { id: Algorithm; label: string; description: string }[] = [
    { id: 'grover', label: "Grover's Search", description: 'Quadratic speedup for unstructured search' },
    { id: 'shor', label: "Shor's Factorization", description: 'Factor integers in polynomial time' },
    { id: 'qft', label: 'Quantum Fourier Transform', description: 'Exponential speedup over classical FFT' },
    { id: 'qpe', label: 'Phase Estimation', description: 'Estimate eigenvalues of unitary operators' },
    { id: 'vqe', label: 'VQE', description: 'Find ground state energy of Hamiltonians' },
    { id: 'qaoa', label: 'QAOA', description: 'Approximate optimization for combinatorial problems' },
  ];

  async function runAlgorithm() {
    loading = true;
    error = null;
    result = null;

    try {
      switch (selectedAlgorithm) {
        case 'grover':
          result = await invoke('run_grover', {
            qubits: groverQubits,
            target: groverTarget,
            shots: groverShots
          });
          break;
        case 'shor':
          result = await invoke('run_shor', { number: shorNumber });
          break;
        case 'qft':
          result = await invoke('run_qft', { qubits: qftQubits, inputState: 1 });
          break;
        case 'qpe':
          result = await invoke('run_qpe', { precision: qpeQubits, phase: qpePhase, shots: 1000 });
          break;
        case 'vqe':
          result = await invoke('run_vqe', { qubits: vqeQubits, layers: vqeLayers, maxIterations: 50 });
          break;
        case 'qaoa':
          const edges: [number, number][] = [];
          for (let i = 0; i < qaoaQubits; i++) {
            edges.push([i, (i + 1) % qaoaQubits]);
          }
          result = await invoke('run_qaoa', { edges, layers: qaoaLayers, shots: 1000 });
          break;
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <!-- Header -->
  <div class="p-4 border-b border-surface-700">
    <h1 class="text-xl font-bold text-white">Quantum Algorithms</h1>
    <p class="text-slate-400 text-sm mt-1">Run classical quantum algorithms with simulation</p>
  </div>

  <div class="flex-1 flex overflow-hidden">
    <!-- Algorithm Selection -->
    <div class="w-72 bg-surface-800 border-r border-surface-700 p-4 overflow-y-auto">
      <h3 class="text-sm font-semibold text-slate-400 mb-3">Select Algorithm</h3>
      <div class="space-y-2">
        {#each algorithms as algo}
          <button
            on:click={() => { selectedAlgorithm = algo.id; result = null; }}
            class="w-full text-left p-3 rounded-lg transition-all"
            class:bg-quantum-primary={selectedAlgorithm === algo.id}
            class:bg-surface-700={selectedAlgorithm !== algo.id}
          >
            <div class="font-medium text-white">{algo.label}</div>
            <div class="text-xs text-slate-400 mt-1">{algo.description}</div>
          </button>
        {/each}
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 p-6 overflow-y-auto">
      {#if error}
        <div class="bg-quantum-error/20 border border-quantum-error text-quantum-error p-4 rounded-lg mb-4">
          {error}
        </div>
      {/if}

      <!-- Parameters -->
      <div class="card mb-6">
        <h2 class="text-lg font-semibold text-white mb-4">Parameters</h2>

        {#if selectedAlgorithm === 'grover'}
          <div class="grid grid-cols-3 gap-4">
            <div>
              <label class="block text-sm text-slate-400 mb-2">Qubits</label>
              <input type="number" bind:value={groverQubits} min="1" max="10" class="input w-full" />
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-2">Target State</label>
              <input type="number" bind:value={groverTarget} min="0" max={Math.pow(2, groverQubits) - 1} class="input w-full" />
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-2">Shots</label>
              <input type="number" bind:value={groverShots} min="100" max="10000" class="input w-full" />
            </div>
          </div>
        {:else if selectedAlgorithm === 'shor'}
          <div>
            <label class="block text-sm text-slate-400 mb-2">Number to Factor</label>
            <input type="number" bind:value={shorNumber} min="4" max="100" class="input w-48" />
          </div>
        {:else if selectedAlgorithm === 'qft'}
          <div>
            <label class="block text-sm text-slate-400 mb-2">Qubits</label>
            <input type="number" bind:value={qftQubits} min="1" max="10" class="input w-48" />
          </div>
        {:else if selectedAlgorithm === 'qpe'}
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm text-slate-400 mb-2">Precision Qubits</label>
              <input type="number" bind:value={qpeQubits} min="1" max="10" class="input w-full" />
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-2">Phase (0-1)</label>
              <input type="number" bind:value={qpePhase} min="0" max="1" step="0.01" class="input w-full" />
            </div>
          </div>
        {:else if selectedAlgorithm === 'vqe'}
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm text-slate-400 mb-2">Qubits</label>
              <input type="number" bind:value={vqeQubits} min="1" max="6" class="input w-full" />
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-2">Ansatz Layers</label>
              <input type="number" bind:value={vqeLayers} min="1" max="5" class="input w-full" />
            </div>
          </div>
        {:else if selectedAlgorithm === 'qaoa'}
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm text-slate-400 mb-2">Graph Nodes</label>
              <input type="number" bind:value={qaoaQubits} min="3" max="8" class="input w-full" />
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-2">QAOA Layers</label>
              <input type="number" bind:value={qaoaLayers} min="1" max="5" class="input w-full" />
            </div>
          </div>
        {/if}

        <button on:click={runAlgorithm} class="btn-primary mt-4" disabled={loading}>
          {#if loading}
            <span class="animate-spin mr-2">@</span>
          {/if}
          Run Algorithm
        </button>
      </div>

      <!-- Results -->
      {#if result}
        <div class="card">
          <h2 class="text-lg font-semibold text-white mb-4">Results</h2>

          {#if selectedAlgorithm === 'grover'}
            <div class="grid grid-cols-2 gap-4 mb-4">
              <div class="bg-surface-700 p-4 rounded-lg">
                <div class="text-slate-400 text-sm">Measured State</div>
                <div class="text-2xl font-mono text-white">|{result.measured_state_binary}&gt;</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg">
                <div class="text-slate-400 text-sm">Success Probability</div>
                <div class="text-2xl font-bold text-quantum-success">{(result.success_probability * 100).toFixed(1)}%</div>
              </div>
            </div>
            <div class="text-sm text-slate-400">
              Iterations: {result.iterations} | Solution found: {result.is_solution ? 'Yes' : 'No'}
            </div>
          {:else if selectedAlgorithm === 'shor'}
            {#if result.success}
              <div class="bg-quantum-success/20 border border-quantum-success p-4 rounded-lg">
                <div class="text-quantum-success font-semibold mb-2">Factorization Successful!</div>
                <div class="text-white text-lg">
                  {result.number} = {result.factors.join(' x ')}
                </div>
              </div>
            {:else}
              <div class="bg-quantum-error/20 border border-quantum-error p-4 rounded-lg">
                Factorization failed after {result.attempts} attempts
              </div>
            {/if}
          {:else if selectedAlgorithm === 'qft'}
            <div class="text-sm text-slate-400 mb-4">
              Depth: {result.depth} | Gates: {result.gate_count}
            </div>
            <div class="space-y-2">
              {#each result.output_probabilities.slice(0, 8) as prob, i}
                {#if prob > 0.001}
                  <div class="flex items-center gap-4">
                    <span class="font-mono text-slate-300 w-24">|{i.toString(2).padStart(result.qubits, '0')}&gt;</span>
                    <div class="flex-1 h-4 bg-surface-700 rounded overflow-hidden">
                      <div class="h-full bg-quantum-accent" style="width: {prob * 100}%"></div>
                    </div>
                    <span class="text-slate-400 text-sm w-16 text-right">{(prob * 100).toFixed(1)}%</span>
                  </div>
                {/if}
              {/each}
            </div>
          {:else if selectedAlgorithm === 'qpe'}
            <div class="grid grid-cols-2 gap-4">
              <div class="bg-surface-700 p-4 rounded-lg">
                <div class="text-slate-400 text-sm">Estimated Phase</div>
                <div class="text-2xl font-mono text-white">{result.estimated_phase.toFixed(4)}</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg">
                <div class="text-slate-400 text-sm">Error</div>
                <div class="text-2xl font-mono text-quantum-warning">{result.error.toFixed(6)}</div>
              </div>
            </div>
          {:else if selectedAlgorithm === 'vqe'}
            <div class="grid grid-cols-2 gap-4">
              <div class="bg-surface-700 p-4 rounded-lg">
                <div class="text-slate-400 text-sm">Ground Energy</div>
                <div class="text-2xl font-mono text-white">{result.energy.toFixed(6)}</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg">
                <div class="text-slate-400 text-sm">Converged</div>
                <div class="text-2xl font-bold" class:text-quantum-success={result.converged} class:text-quantum-error={!result.converged}>
                  {result.converged ? 'Yes' : 'No'}
                </div>
              </div>
            </div>
          {:else if selectedAlgorithm === 'qaoa'}
            <div class="grid grid-cols-2 gap-4 mb-4">
              <div class="bg-surface-700 p-4 rounded-lg">
                <div class="text-slate-400 text-sm">Best Cut Value</div>
                <div class="text-2xl font-mono text-white">{result.best_cost}</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg">
                <div class="text-slate-400 text-sm">Approximation Ratio</div>
                <div class="text-2xl font-mono text-quantum-success">{result.approximation_ratio.toFixed(4)}</div>
              </div>
            </div>
            <div class="text-sm text-slate-400">
              Best solution: {result.best_solution.map((b) => b ? '1' : '0').join('')}
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</div>
