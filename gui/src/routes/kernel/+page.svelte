<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Coord5DDisplay from '$lib/components/Coord5DDisplay.svelte';

  // Types
  interface SearchStrategy {
    type: string;
    temperature?: number;
    width?: number;
    population_size?: number;
    mutation_rate?: number;
  }

  interface MiningConfig {
    max_iterations: number;
    target_resonance: number;
    max_candidates: number;
    exploration_rate: number;
    strategy: SearchStrategy;
    convergence_epsilon: number;
  }

  interface BlueprintCandidate {
    id: string;
    name: string;
    signature: { psi: number; rho: number; omega: number; chi: number | null; eta: number | null };
    resonance_score: number;
    quality_level: string;
  }

  interface MiningResult {
    best_resonance: number;
    iterations: number;
    converged: boolean;
    candidates: BlueprintCandidate[];
    total_candidates_explored: number;
    stagnation_count: number;
    duration_ms: number;
  }

  interface MaterializationConfig {
    artefact_type: string;
    write_files: boolean;
    record_in_ledger: boolean;
    output_format: string;
  }

  interface MaterializationResult {
    success: boolean;
    artefact: {
      id: string;
      blueprint_id: string;
      artefact_type: string;
      content: unknown;
      final_resonance: number;
      created_at: string;
    };
    ledger_entry_id: string | null;
    warnings: string[];
  }

  interface KernelInfo {
    version: string;
    name: string;
    dimensions: number;
    available_strategies: string[];
    artefact_types: string[];
    resonance_models: string[];
  }

  // State
  let mode: 'mining' | 'materialize' = 'mining';
  let loading = false;
  let error: string | null = null;

  // Mining state
  let seedCoord = { psi: 0.5, rho: 0.5, omega: 0.5, chi: 0.5, eta: 0.5 };
  let miningConfig: MiningConfig = {
    max_iterations: 100,
    target_resonance: 0.8,
    max_candidates: 50,
    exploration_rate: 0.3,
    strategy: { type: 'stochastic', temperature: 1.0 },
    convergence_epsilon: 1e-4,
  };
  let miningResult: MiningResult | null = null;
  let selectedCandidate: BlueprintCandidate | null = null;

  // Materialization state
  let materializeConfig: MaterializationConfig = {
    artefact_type: 'data',
    write_files: false,
    record_in_ledger: true,
    output_format: 'json',
  };
  let materializeResult: MaterializationResult | null = null;

  // Kernel info
  let kernelInfo: KernelInfo | null = null;

  const strategies = [
    { value: 'greedy', label: 'Greedy' },
    { value: 'stochastic', label: 'Stochastic' },
    { value: 'beam', label: 'Beam Search' },
    { value: 'evolutionary', label: 'Evolutionary' },
    { value: 'triton', label: 'TRITON' },
    { value: 'hybrid', label: 'Hybrid' },
  ];

  const artefactTypes = [
    { value: 'data', label: 'Data (JSON)' },
    { value: 'code', label: 'Code' },
    { value: 'configuration', label: 'Configuration' },
    { value: 'document', label: 'Document' },
    { value: 'operator', label: 'Operator' },
    { value: 'circuit', label: 'Circuit' },
  ];

  const presets = [
    { name: 'quick', iterations: 50, resonance: 0.7, strategy: 'greedy' },
    { name: 'balanced', iterations: 100, resonance: 0.8, strategy: 'stochastic' },
    { name: 'thorough', iterations: 200, resonance: 0.85, strategy: 'hybrid' },
    { name: 'research', iterations: 500, resonance: 0.9, strategy: 'evolutionary' },
  ];

  function applyPreset(preset: typeof presets[0]) {
    miningConfig.max_iterations = preset.iterations;
    miningConfig.target_resonance = preset.resonance;
    miningConfig.strategy.type = preset.strategy;
  }

  async function loadKernelInfo() {
    try {
      kernelInfo = await invoke<KernelInfo>('get_kernel_info');
    } catch (e) {
      console.error('Failed to load kernel info:', e);
    }
  }

  async function runMining() {
    loading = true;
    error = null;
    miningResult = null;
    selectedCandidate = null;

    try {
      miningResult = await invoke<MiningResult>('run_kernel_mining', {
        seedPsi: seedCoord.psi,
        seedRho: seedCoord.rho,
        seedOmega: seedCoord.omega,
        seedChi: seedCoord.chi,
        seedEta: seedCoord.eta,
        config: miningConfig,
      });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function materialize() {
    if (!selectedCandidate) {
      error = 'Please select a candidate blueprint first';
      return;
    }

    loading = true;
    error = null;
    materializeResult = null;

    try {
      materializeResult = await invoke<MaterializationResult>('materialize_blueprint', {
        blueprintId: selectedCandidate.id,
        blueprintPsi: selectedCandidate.signature.psi,
        blueprintRho: selectedCandidate.signature.rho,
        blueprintOmega: selectedCandidate.signature.omega,
        blueprintChi: selectedCandidate.signature.chi ?? 0.5,
        blueprintEta: selectedCandidate.signature.eta ?? 0.5,
        config: materializeConfig,
      });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function selectCandidate(candidate: BlueprintCandidate) {
    selectedCandidate = candidate;
    mode = 'materialize';
  }

  // Load kernel info on mount
  loadKernelInfo();
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <!-- Header -->
  <div class="p-4 border-b border-surface-700">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-xl font-bold text-white">Kernel Mining & Materialization</h1>
        <p class="text-slate-400 text-sm mt-1">M = (Q, S, F, R) blueprint mining with artefact generation</p>
      </div>
      {#if kernelInfo}
        <div class="text-right text-sm">
          <div class="text-slate-400">{kernelInfo.name}</div>
          <div class="text-slate-500">v{kernelInfo.version} | {kernelInfo.dimensions}D</div>
        </div>
      {/if}
    </div>
  </div>

  <div class="flex-1 p-6 overflow-y-auto">
    {#if error}
      <div class="bg-red-500/20 border border-red-500 text-red-400 p-4 rounded-lg mb-4">
        {error}
      </div>
    {/if}

    <!-- Mode Tabs -->
    <div class="flex gap-2 mb-6">
      <button
        on:click={() => mode = 'mining'}
        class="px-4 py-2 rounded-lg font-medium transition-all"
        class:bg-quantum-primary={mode === 'mining'}
        class:text-white={mode === 'mining'}
        class:bg-surface-700={mode !== 'mining'}
        class:text-slate-400={mode !== 'mining'}
      >
        Mining (Q, S, F, R)
      </button>
      <button
        on:click={() => mode = 'materialize'}
        class="px-4 py-2 rounded-lg font-medium transition-all"
        class:bg-quantum-primary={mode === 'materialize'}
        class:text-white={mode === 'materialize'}
        class:bg-surface-700={mode !== 'materialize'}
        class:text-slate-400={mode !== 'materialize'}
        disabled={!miningResult}
      >
        Materialize (B -> A)
      </button>
    </div>

    {#if mode === 'mining'}
      <div class="grid grid-cols-3 gap-6">
        <!-- Configuration Panel -->
        <div class="card">
          <h2 class="text-lg font-semibold text-white mb-4">Mining Configuration</h2>

          <!-- Presets -->
          <div class="mb-4">
            <label class="block text-sm text-slate-400 mb-2">Preset</label>
            <div class="grid grid-cols-2 gap-2">
              {#each presets as preset}
                <button
                  on:click={() => applyPreset(preset)}
                  class="py-2 px-3 rounded text-sm font-medium bg-surface-700 text-slate-300 hover:bg-surface-600 transition-colors capitalize"
                >
                  {preset.name}
                </button>
              {/each}
            </div>
          </div>

          <!-- Seed Coordinate -->
          <div class="mb-4">
            <label class="block text-sm text-slate-400 mb-2">Seed Coordinate (5D)</label>
            <div class="grid grid-cols-5 gap-2">
              <div>
                <label class="block text-xs text-slate-500">psi</label>
                <input type="number" bind:value={seedCoord.psi} min="0" max="1" step="0.1" class="input w-full text-sm" />
              </div>
              <div>
                <label class="block text-xs text-slate-500">rho</label>
                <input type="number" bind:value={seedCoord.rho} min="0" max="1" step="0.1" class="input w-full text-sm" />
              </div>
              <div>
                <label class="block text-xs text-slate-500">omega</label>
                <input type="number" bind:value={seedCoord.omega} min="0" max="1" step="0.1" class="input w-full text-sm" />
              </div>
              <div>
                <label class="block text-xs text-slate-500">chi</label>
                <input type="number" bind:value={seedCoord.chi} min="0" max="1" step="0.1" class="input w-full text-sm" />
              </div>
              <div>
                <label class="block text-xs text-slate-500">eta</label>
                <input type="number" bind:value={seedCoord.eta} min="0" max="1" step="0.1" class="input w-full text-sm" />
              </div>
            </div>
          </div>

          <!-- Strategy -->
          <div class="mb-4">
            <label class="block text-sm text-slate-400 mb-2">Search Strategy (S)</label>
            <select bind:value={miningConfig.strategy.type} class="input w-full">
              {#each strategies as s}
                <option value={s.value}>{s.label}</option>
              {/each}
            </select>
          </div>

          <!-- Parameters -->
          <div class="grid grid-cols-2 gap-3 mb-4">
            <div>
              <label class="block text-xs text-slate-400 mb-1">Max Iterations</label>
              <input type="number" bind:value={miningConfig.max_iterations} min="10" max="1000" class="input w-full text-sm" />
            </div>
            <div>
              <label class="block text-xs text-slate-400 mb-1">Target R(v)</label>
              <input type="number" bind:value={miningConfig.target_resonance} min="0" max="1" step="0.05" class="input w-full text-sm" />
            </div>
            <div>
              <label class="block text-xs text-slate-400 mb-1">Max Candidates</label>
              <input type="number" bind:value={miningConfig.max_candidates} min="10" max="200" class="input w-full text-sm" />
            </div>
            <div>
              <label class="block text-xs text-slate-400 mb-1">Exploration</label>
              <input type="number" bind:value={miningConfig.exploration_rate} min="0" max="1" step="0.1" class="input w-full text-sm" />
            </div>
          </div>

          <button
            on:click={runMining}
            class="w-full py-3 bg-quantum-primary rounded-lg font-medium text-white hover:bg-quantum-primary/90 transition-opacity disabled:opacity-50"
            disabled={loading}
          >
            {#if loading}
              <span class="flex items-center justify-center gap-2">
                <svg class="animate-spin h-5 w-5" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                </svg>
                Mining...
              </span>
            {:else}
              Run Mining Kernel
            {/if}
          </button>
        </div>

        <!-- Results Panel -->
        <div class="card col-span-2">
          <h2 class="text-lg font-semibold text-white mb-4">Blueprint Candidates</h2>

          {#if miningResult}
            <!-- Summary -->
            <div class="grid grid-cols-4 gap-4 mb-6">
              <div class="bg-surface-700 p-4 rounded-lg text-center">
                <div class="text-2xl font-bold text-amber-400">{miningResult.best_resonance.toFixed(4)}</div>
                <div class="text-xs text-slate-400 mt-1">Best R(v)</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg text-center">
                <div class="text-2xl font-bold text-blue-400">{miningResult.iterations}</div>
                <div class="text-xs text-slate-400 mt-1">Iterations</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg text-center">
                <div class="text-2xl font-bold text-violet-400">{miningResult.candidates.length}</div>
                <div class="text-xs text-slate-400 mt-1">Candidates</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg text-center">
                <div class="text-2xl font-bold" class:text-emerald-400={miningResult.converged} class:text-slate-400={!miningResult.converged}>
                  {miningResult.converged ? 'Yes' : 'No'}
                </div>
                <div class="text-xs text-slate-400 mt-1">Converged</div>
              </div>
            </div>

            <!-- Candidates table -->
            <div class="max-h-96 overflow-y-auto">
              <table class="w-full text-sm">
                <thead class="text-slate-400 sticky top-0 bg-surface-800">
                  <tr>
                    <th class="text-left py-2 px-3">Blueprint</th>
                    <th class="text-right py-2 px-3">R(v)</th>
                    <th class="text-right py-2 px-3">psi</th>
                    <th class="text-right py-2 px-3">rho</th>
                    <th class="text-right py-2 px-3">omega</th>
                    <th class="text-center py-2 px-3">Quality</th>
                    <th class="text-center py-2 px-3">Action</th>
                  </tr>
                </thead>
                <tbody>
                  {#each miningResult.candidates as candidate}
                    <tr
                      class="border-t border-surface-700 hover:bg-surface-700/50 cursor-pointer"
                      class:bg-quantum-primary/20={selectedCandidate?.id === candidate.id}
                      on:click={() => selectedCandidate = candidate}
                    >
                      <td class="py-2 px-3 text-slate-300 font-mono text-xs">{candidate.id.slice(0, 12)}...</td>
                      <td class="py-2 px-3 text-right font-mono text-amber-400">{candidate.resonance_score.toFixed(4)}</td>
                      <td class="py-2 px-3 text-right font-mono">{candidate.signature.psi.toFixed(2)}</td>
                      <td class="py-2 px-3 text-right font-mono">{candidate.signature.rho.toFixed(2)}</td>
                      <td class="py-2 px-3 text-right font-mono">{candidate.signature.omega.toFixed(2)}</td>
                      <td class="py-2 px-3 text-center">
                        <span class="px-2 py-1 rounded text-xs"
                          class:bg-emerald-500/20={candidate.quality_level === 'Excellent'}
                          class:text-emerald-400={candidate.quality_level === 'Excellent'}
                          class:bg-blue-500/20={candidate.quality_level === 'Good'}
                          class:text-blue-400={candidate.quality_level === 'Good'}
                          class:bg-amber-500/20={candidate.quality_level === 'Acceptable'}
                          class:text-amber-400={candidate.quality_level === 'Acceptable'}
                          class:bg-slate-500/20={candidate.quality_level === 'Low'}
                          class:text-slate-400={candidate.quality_level === 'Low'}
                        >
                          {candidate.quality_level}
                        </span>
                      </td>
                      <td class="py-2 px-3 text-center">
                        <button
                          on:click|stopPropagation={() => selectCandidate(candidate)}
                          class="text-xs bg-quantum-primary/20 text-quantum-primary hover:bg-quantum-primary/30 px-2 py-1 rounded"
                        >
                          Materialize
                        </button>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>

            <div class="mt-4 text-sm text-slate-500">
              Explored {miningResult.total_candidates_explored} total candidates in {miningResult.duration_ms}ms
            </div>
          {:else}
            <div class="flex flex-col items-center justify-center h-64 text-slate-500">
              <svg class="w-16 h-16 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                  d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
              <p class="text-lg">Configure and run mining</p>
              <p class="text-sm mt-2">The kernel will search for high-resonance blueprints</p>
            </div>
          {/if}
        </div>
      </div>
    {:else}
      <!-- Materialization Mode -->
      <div class="grid grid-cols-2 gap-6">
        <!-- Selected Blueprint -->
        <div class="card">
          <h2 class="text-lg font-semibold text-white mb-4">Selected Blueprint</h2>

          {#if selectedCandidate}
            <div class="bg-surface-700 p-4 rounded-lg mb-4">
              <div class="flex justify-between items-start mb-3">
                <div>
                  <div class="text-sm text-slate-400">Blueprint ID</div>
                  <div class="font-mono text-white">{selectedCandidate.id}</div>
                </div>
                <div class="text-right">
                  <div class="text-2xl font-bold text-amber-400">{selectedCandidate.resonance_score.toFixed(4)}</div>
                  <div class="text-xs text-slate-400">Resonance</div>
                </div>
              </div>

              <Coord5DDisplay
                psi={selectedCandidate.signature.psi}
                rho={selectedCandidate.signature.rho}
                omega={selectedCandidate.signature.omega}
                chi={selectedCandidate.signature.chi ?? 0.5}
                eta={selectedCandidate.signature.eta ?? 0.5}
              />
            </div>

            <!-- Materialization Config -->
            <div class="space-y-4">
              <div>
                <label class="block text-sm text-slate-400 mb-2">Artefact Type</label>
                <select bind:value={materializeConfig.artefact_type} class="input w-full">
                  {#each artefactTypes as t}
                    <option value={t.value}>{t.label}</option>
                  {/each}
                </select>
              </div>

              <div class="flex gap-4">
                <label class="flex items-center gap-2">
                  <input type="checkbox" bind:checked={materializeConfig.write_files} class="rounded bg-surface-700 border-surface-600" />
                  <span class="text-sm text-slate-300">Write to files</span>
                </label>
                <label class="flex items-center gap-2">
                  <input type="checkbox" bind:checked={materializeConfig.record_in_ledger} class="rounded bg-surface-700 border-surface-600" />
                  <span class="text-sm text-slate-300">Record in ledger</span>
                </label>
              </div>

              <button
                on:click={materialize}
                class="w-full py-3 bg-emerald-600 rounded-lg font-medium text-white hover:bg-emerald-500 transition-colors disabled:opacity-50"
                disabled={loading}
              >
                {#if loading}
                  Materializing...
                {:else}
                  Materialize Blueprint -> Artefact
                {/if}
              </button>
            </div>
          {:else}
            <div class="text-slate-500 text-center py-8">
              No blueprint selected. Run mining first and select a candidate.
            </div>
          {/if}
        </div>

        <!-- Materialization Result -->
        <div class="card">
          <h2 class="text-lg font-semibold text-white mb-4">Generated Artefact</h2>

          {#if materializeResult}
            <div class="bg-emerald-500/10 border border-emerald-500/50 p-4 rounded-lg mb-4">
              <div class="flex items-center gap-2 mb-2">
                <div class="w-3 h-3 bg-emerald-500 rounded-full"></div>
                <span class="font-semibold text-emerald-400">ARTEFACT GENERATED</span>
              </div>
              <div class="text-sm text-slate-300">
                Blueprint successfully materialized into {materializeResult.artefact.artefact_type} artefact
              </div>
            </div>

            <div class="space-y-3 mb-4">
              <div class="flex justify-between">
                <span class="text-slate-400">Artefact ID</span>
                <span class="font-mono text-white">{materializeResult.artefact.id.slice(0, 16)}...</span>
              </div>
              <div class="flex justify-between">
                <span class="text-slate-400">Type</span>
                <span class="text-white capitalize">{materializeResult.artefact.artefact_type}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-slate-400">Final Resonance</span>
                <span class="font-mono text-amber-400">{materializeResult.artefact.final_resonance.toFixed(4)}</span>
              </div>
              {#if materializeResult.ledger_entry_id}
                <div class="flex justify-between">
                  <span class="text-slate-400">Ledger Entry</span>
                  <span class="font-mono text-slate-300">{materializeResult.ledger_entry_id.slice(0, 12)}...</span>
                </div>
              {/if}
            </div>

            <!-- Content Preview -->
            <div class="bg-surface-700 p-4 rounded-lg">
              <div class="text-sm text-slate-400 mb-2">Content Preview</div>
              <pre class="text-xs text-slate-300 overflow-x-auto max-h-48">{JSON.stringify(materializeResult.artefact.content, null, 2)}</pre>
            </div>

            {#if materializeResult.warnings.length > 0}
              <div class="mt-4 bg-amber-500/10 border border-amber-500/50 p-3 rounded">
                <div class="text-sm text-amber-400 font-medium mb-1">Warnings</div>
                {#each materializeResult.warnings as warning}
                  <div class="text-xs text-amber-300">{warning}</div>
                {/each}
              </div>
            {/if}
          {:else}
            <div class="flex flex-col items-center justify-center h-64 text-slate-500">
              <svg class="w-16 h-16 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                  d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              </svg>
              <p class="text-lg">No artefact generated yet</p>
              <p class="text-sm mt-2">Select a blueprint and click Materialize</p>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .card {
    @apply bg-surface-800 border border-surface-700 rounded-lg p-4;
  }

  .input {
    @apply bg-surface-700 border border-surface-600 rounded px-3 py-2 text-white focus:outline-none focus:border-quantum-primary;
  }
</style>
