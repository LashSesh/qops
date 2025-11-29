<script lang="ts">
  import { onMount } from 'svelte';
  import {
    runSlotsEngine,
    slotsMineSequence,
    getSlotsInfo,
    getMiningStrategies,
    getEntropyDistributions,
    slotsGenerateArtifacts,
    runHypercubeSlotsMode,
    type SlotsSessionResultDto,
    type SlotsMiningResultDto,
    type SlotArtifactDto,
    type MiningStrategyDto,
    type EntropyDistributionDto,
  } from '$lib/tauri/commands';
  import SlotMachineVisualizer from '$lib/components/SlotMachineVisualizer.svelte';
  import Coord5DDisplay from '$lib/components/Coord5DDisplay.svelte';

  // State
  let activeTab: 'engine' | 'mine' | 'artifacts' = 'engine';
  let loading = false;
  let isSpinning = false;
  let error: string | null = null;

  // Engine settings
  let steps = 50;
  let entropyDistribution = 'uniform';
  let miningStrategy = 'beam';
  let targetResonance = 0.8;
  let sessionResult: SlotsSessionResultDto | null = null;

  // Mining settings
  let miningDepth = 10;
  let beamWidth = 10;
  let miningResult: SlotsMiningResultDto | null = null;

  // Artifact generation
  let artifactCoord = { psi: 0.7, rho: 0.6, omega: 0.5, chi: 0.4, eta: 0.3 };
  let artifacts: SlotArtifactDto[] = [];

  // Info
  let slotsInfo: Record<string, unknown> | null = null;
  let strategies: MiningStrategyDto[] = [];
  let distributions: EntropyDistributionDto[] = [];

  // Helper getters for template access
  $: slotsSymbols = (slotsInfo?.symbols as Array<{name: string; weight: number; description: string}> | undefined) ?? [];
  $: slotsFeatures = (slotsInfo?.features as string[] | undefined) ?? [];

  onMount(async () => {
    try {
      slotsInfo = await getSlotsInfo();
      strategies = await getMiningStrategies();
      distributions = await getEntropyDistributions();
    } catch (e) {
      error = String(e);
    }
  });

  async function handleRunEngine() {
    loading = true;
    isSpinning = true;
    error = null;

    // Spin animation for a bit
    await new Promise(r => setTimeout(r, 1500));

    try {
      sessionResult = await runSlotsEngine(steps, entropyDistribution, miningStrategy, targetResonance);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
      isSpinning = false;
    }
  }

  async function handleMine() {
    loading = true;
    error = null;
    try {
      miningResult = await slotsMineSequence(miningDepth, miningStrategy, targetResonance, beamWidth);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleGenerateArtifacts() {
    loading = true;
    error = null;
    try {
      artifacts = await slotsGenerateArtifacts(
        artifactCoord.psi,
        artifactCoord.rho,
        artifactCoord.omega,
        artifactCoord.chi,
        artifactCoord.eta
      );
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleHypercubeMode() {
    loading = true;
    error = null;
    try {
      artifacts = await runHypercubeSlotsMode(
        artifactCoord.psi,
        artifactCoord.rho,
        artifactCoord.omega,
        artifactCoord.chi,
        artifactCoord.eta
      );
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
    <h1 class="text-xl font-bold text-white">Quantum Slots Engine</h1>
    <p class="text-slate-400 text-sm mt-1">Entropy-driven slot evaluation with sequence mining</p>
  </div>

  <!-- Tab navigation -->
  <div class="flex border-b border-surface-700">
    {#each [
      { id: 'engine', label: 'Engine' },
      { id: 'mine', label: 'Sequence Mining' },
      { id: 'artifacts', label: 'Hypercube Integration' },
    ] as tab}
      <button
        on:click={() => activeTab = tab.id}
        class="px-4 py-3 text-sm font-medium transition-colors"
        class:text-white={activeTab === tab.id}
        class:border-b-2={activeTab === tab.id}
        class:border-quantum-primary={activeTab === tab.id}
        class:text-slate-400={activeTab !== tab.id}
      >
        {tab.label}
      </button>
    {/each}
  </div>

  <div class="flex-1 p-6 overflow-y-auto">
    {#if error}
      <div class="bg-red-500/20 border border-red-500 text-red-400 p-4 rounded-lg mb-4">
        {error}
      </div>
    {/if}

    <div class="grid grid-cols-3 gap-6">
      <!-- Slot Machine Visualizer -->
      <div class="card">
        <SlotMachineVisualizer {sessionResult} {isSpinning} reelCount={5} />
      </div>

      <!-- Main Content Panel -->
      <div class="card col-span-2">
        {#if activeTab === 'engine'}
          <h2 class="text-lg font-semibold text-white mb-4">Slots Engine</h2>

          <div class="grid grid-cols-2 gap-4 mb-4">
            <div>
              <label class="block text-sm text-slate-400 mb-1">Steps</label>
              <input type="number" bind:value={steps} min="10" max="200" class="input w-full" />
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-1">Target Resonance</label>
              <input type="number" bind:value={targetResonance} min="0" max="1" step="0.05" class="input w-full" />
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-1">Entropy Distribution</label>
              <select bind:value={entropyDistribution} class="input w-full">
                {#each distributions as dist}
                  <option value={dist.name}>{dist.name}</option>
                {/each}
              </select>
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-1">Mining Strategy</label>
              <select bind:value={miningStrategy} class="input w-full">
                {#each strategies as strat}
                  <option value={strat.name}>{strat.name}</option>
                {/each}
              </select>
            </div>
          </div>

          <button
            on:click={handleRunEngine}
            class="w-full py-3 px-4 bg-gradient-to-r from-violet-600 to-fuchsia-600 rounded-lg font-medium text-white hover:opacity-90 transition-opacity disabled:opacity-50"
            disabled={loading}
          >
            {#if loading}
              <span class="flex items-center justify-center gap-2">
                <svg class="animate-spin h-5 w-5" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                </svg>
                Spinning...
              </span>
            {:else}
              SPIN
            {/if}
          </button>

          {#if sessionResult && sessionResult.best_sequence}
            <div class="mt-4 p-4 bg-surface-700 rounded-lg">
              <h3 class="text-sm font-medium text-white mb-2">Best Sequence</h3>
              <div class="flex flex-wrap gap-2 mb-3">
                {#each sessionResult.best_sequence.symbols as symbol}
                  <span class="px-2 py-1 bg-surface-600 rounded text-xs text-slate-300">{symbol}</span>
                {/each}
              </div>
              <div class="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <span class="text-slate-400">Resonance:</span>
                  <span class="text-emerald-400 ml-1">{sessionResult.best_sequence.resonance.toFixed(4)}</span>
                </div>
                <div>
                  <span class="text-slate-400">Depth:</span>
                  <span class="text-white ml-1">{sessionResult.best_sequence.depth}</span>
                </div>
              </div>
            </div>
          {/if}

        {:else if activeTab === 'mine'}
          <h2 class="text-lg font-semibold text-white mb-4">Sequence Mining</h2>

          <div class="grid grid-cols-3 gap-4 mb-4">
            <div>
              <label class="block text-sm text-slate-400 mb-1">Depth</label>
              <input type="number" bind:value={miningDepth} min="1" max="50" class="input w-full" />
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-1">Beam Width</label>
              <input type="number" bind:value={beamWidth} min="1" max="50" class="input w-full" />
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-1">Strategy</label>
              <select bind:value={miningStrategy} class="input w-full">
                {#each strategies as strat}
                  <option value={strat.name}>{strat.name}</option>
                {/each}
              </select>
            </div>
          </div>

          <button
            on:click={handleMine}
            class="w-full py-3 px-4 bg-quantum-primary rounded-lg font-medium text-white hover:opacity-90 transition-opacity disabled:opacity-50 mb-4"
            disabled={loading}
          >
            {loading ? 'Mining...' : 'Mine Sequences'}
          </button>

          {#if miningResult}
            <div class="space-y-4">
              <div class="grid grid-cols-4 gap-4">
                <div class="bg-surface-700 p-3 rounded-lg text-center">
                  <div class="text-xl font-bold text-emerald-400">{miningResult.best_resonance.toFixed(4)}</div>
                  <div class="text-xs text-slate-400">Best</div>
                </div>
                <div class="bg-surface-700 p-3 rounded-lg text-center">
                  <div class="text-xl font-bold text-blue-400">{miningResult.total_steps}</div>
                  <div class="text-xs text-slate-400">Steps</div>
                </div>
                <div class="bg-surface-700 p-3 rounded-lg text-center">
                  <div class="text-xl font-bold text-amber-400">{miningResult.steps_to_best}</div>
                  <div class="text-xs text-slate-400">To Best</div>
                </div>
                <div class="bg-surface-700 p-3 rounded-lg text-center">
                  <div class="text-xl font-bold" class:text-emerald-400={miningResult.converged} class:text-amber-400={!miningResult.converged}>
                    {miningResult.converged ? 'Yes' : 'No'}
                  </div>
                  <div class="text-xs text-slate-400">Converged</div>
                </div>
              </div>

              <div>
                <h3 class="text-sm font-medium text-white mb-2">Top Sequences</h3>
                <div class="max-h-48 overflow-y-auto space-y-2">
                  {#each miningResult.top_sequences as seq, i}
                    <div class="bg-surface-700 p-2 rounded flex items-center justify-between text-sm">
                      <div class="flex items-center gap-2">
                        <span class="text-slate-500">#{i + 1}</span>
                        <div class="flex gap-1">
                          {#each seq.symbols.slice(0, 5) as sym}
                            <span class="text-xs text-slate-300">{sym}</span>
                          {/each}
                          {#if seq.symbols.length > 5}
                            <span class="text-xs text-slate-500">...</span>
                          {/if}
                        </div>
                      </div>
                      <span class="text-emerald-400 font-mono">{seq.resonance.toFixed(4)}</span>
                    </div>
                  {/each}
                </div>
              </div>
            </div>
          {/if}

        {:else if activeTab === 'artifacts'}
          <h2 class="text-lg font-semibold text-white mb-4">Hypercube Integration</h2>

          <div class="mb-4">
            <Coord5DDisplay coordinate={artifactCoord} size={180} title="Target Coordinate" />
          </div>

          <div class="space-y-3 mb-4">
            {#each ['psi', 'rho', 'omega', 'chi', 'eta'] as dim}
              <div>
                <label class="block text-xs text-slate-400 mb-1">{dim}</label>
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.01"
                  bind:value={artifactCoord[dim]}
                  class="w-full"
                />
              </div>
            {/each}
          </div>

          <div class="flex gap-2 mb-4">
            <button
              on:click={handleGenerateArtifacts}
              class="flex-1 py-2 px-4 bg-quantum-primary rounded-lg font-medium text-white hover:opacity-90 transition-opacity disabled:opacity-50"
              disabled={loading}
            >
              Generate Artifacts
            </button>
            <button
              on:click={handleHypercubeMode}
              class="flex-1 py-2 px-4 bg-violet-600 rounded-lg font-medium text-white hover:opacity-90 transition-opacity disabled:opacity-50"
              disabled={loading}
            >
              Hypercube Mode
            </button>
          </div>

          {#if artifacts.length > 0}
            <div>
              <h3 class="text-sm font-medium text-white mb-2">Generated Artifacts ({artifacts.length})</h3>
              <div class="max-h-64 overflow-y-auto space-y-2">
                {#each artifacts as artifact}
                  <div class="bg-surface-700 p-3 rounded">
                    <div class="flex items-center justify-between mb-2">
                      <span class="text-sm font-medium text-white">{artifact.name}</span>
                      <span class="text-emerald-400 font-mono text-sm">{artifact.resonance.toFixed(4)}</span>
                    </div>
                    <div class="text-xs text-slate-400">
                      Coord: ({artifact.coordinate.psi.toFixed(2)}, {artifact.coordinate.rho.toFixed(2)}, {artifact.coordinate.omega.toFixed(2)}, {artifact.coordinate.chi.toFixed(2)}, {artifact.coordinate.eta.toFixed(2)})
                    </div>
                    {#if artifact.source_node}
                      <div class="text-xs text-slate-500 mt-1">Source: {artifact.source_node}</div>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        {/if}
      </div>
    </div>

    <!-- Info panel -->
    {#if slotsInfo}
      <div class="mt-6 card">
        <h3 class="text-sm font-semibold text-white mb-3">Engine Info</h3>
        <div class="grid grid-cols-3 gap-4 text-xs">
          <div>
            <span class="text-slate-400">Version:</span>
            <span class="text-white ml-1">{slotsInfo.version}</span>
          </div>
          <div>
            <span class="text-slate-400">Symbols:</span>
            <span class="text-white ml-1">{slotsSymbols.length}</span>
          </div>
          <div>
            <span class="text-slate-400">Features:</span>
            <span class="text-white ml-1">{slotsFeatures.length}</span>
          </div>
        </div>

        <div class="mt-3 flex flex-wrap gap-2">
          {#each slotsSymbols as sym}
            <div class="px-2 py-1 bg-surface-700 rounded text-xs" title={sym.description}>
              <span class="text-slate-300">{sym.name}</span>
              <span class="text-slate-500 ml-1">({sym.weight > 0 ? '+' : ''}{sym.weight})</span>
            </div>
          {/each}
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
