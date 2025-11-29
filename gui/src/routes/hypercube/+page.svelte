<script lang="ts">
  import { onMount } from 'svelte';
  import {
    compileHypercube,
    expandCubeStep,
    getHypercubeInfo,
    hdagExecute,
    getHdagInfo,
    runHypercubeSession,
    getHypercubePresets,
    type Coord5DDto,
    type HypercubeStatsDto,
    type CompilationResultDto,
    type HDAGInfoDto,
    type HDAGExecutionResultDto,
    type HypercubeSessionResultDto,
  } from '$lib/tauri/commands';
  import HDAGGraph from '$lib/components/HDAGGraph.svelte';
  import Coord5DDisplay from '$lib/components/Coord5DDisplay.svelte';

  // State
  let activeTab: 'compile' | 'expand' | 'hdag' | 'session' = 'session';
  let loading = false;
  let error: string | null = null;

  // Seed coordinates
  let seed: Coord5DDto = { psi: 0.5, rho: 0.5, omega: 0.5, chi: 0.5, eta: 0.5 };

  // Compilation settings
  let iterations = 10;
  let useTriton = true;
  let compilationResult: CompilationResultDto | null = null;

  // Expansion settings
  let expansionRule = 'triton';
  let expansionIterations = 5;
  let expansionStats: HypercubeStatsDto | null = null;

  // HDAG settings
  let pipelineType = 'standard';
  let hdagInfo: HDAGInfoDto | null = null;
  let hdagResult: HDAGExecutionResultDto | null = null;
  let executingNode: string | null = null;
  let completedNodes: string[] = [];

  // Session settings
  let sessionPreset = 'quick';
  let sessionResult: HypercubeSessionResultDto | null = null;

  // Info
  let hypercubeInfo: Record<string, unknown> | null = null;
  let presets: Array<{ name: string; description: string; max_depth: number; expansion_rule: string }> = [];

  // Helper getters for template access
  $: hypercubeOperators = (hypercubeInfo?.operators as string[] | undefined) ?? [];
  $: hypercubeCoordinates = (hypercubeInfo?.coordinates as string[] | undefined) ?? [];

  const expansionRules = ['lattice', 'resonance', 'triton', 'operator', 'random', 'hybrid'];

  onMount(async () => {
    try {
      hypercubeInfo = await getHypercubeInfo();
      presets = await getHypercubePresets();
      hdagInfo = await getHdagInfo(pipelineType);
    } catch (e) {
      error = String(e);
    }
  });

  async function handleCompile() {
    loading = true;
    error = null;
    try {
      compilationResult = await compileHypercube(
        seed.psi, seed.rho, seed.omega, seed.chi, seed.eta,
        iterations, useTriton
      );
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleExpand() {
    loading = true;
    error = null;
    try {
      expansionStats = await expandCubeStep(0, expansionRule, expansionIterations);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleHdagExecute() {
    loading = true;
    error = null;
    completedNodes = [];
    executingNode = null;

    try {
      // Get HDAG info first
      hdagInfo = await getHdagInfo(pipelineType);

      // Simulate execution animation
      if (hdagInfo) {
        for (const nodeId of hdagInfo.execution_order) {
          executingNode = nodeId;
          await new Promise(r => setTimeout(r, 500));
          completedNodes = [...completedNodes, nodeId];
        }
      }

      // Execute
      hdagResult = await hdagExecute(
        pipelineType,
        seed.psi, seed.rho, seed.omega, seed.chi, seed.eta
      );
      executingNode = null;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleSession() {
    loading = true;
    error = null;
    try {
      sessionResult = await runHypercubeSession(
        sessionPreset,
        seed.psi, seed.rho, seed.omega, seed.chi, seed.eta
      );
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function loadHdagInfo() {
    try {
      hdagInfo = await getHdagInfo(pipelineType);
      completedNodes = [];
      executingNode = null;
      hdagResult = null;
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <!-- Header -->
  <div class="p-4 border-b border-surface-700">
    <h1 class="text-xl font-bold text-white">Hypercube Studio</h1>
    <p class="text-slate-400 text-sm mt-1">5D Self-Compiling Cube Framework with HDAG Execution</p>
  </div>

  <!-- Tab navigation -->
  <div class="flex border-b border-surface-700">
    {#each [
      { id: 'session', label: 'Session' },
      { id: 'compile', label: 'Compile' },
      { id: 'expand', label: 'Expand' },
      { id: 'hdag', label: 'HDAG' },
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
      <!-- Seed Coordinate Panel -->
      <div class="card">
        <h2 class="text-lg font-semibold text-white mb-4">Seed Coordinate</h2>
        <Coord5DDisplay coordinate={seed} size={200} />

        <div class="mt-4 space-y-3">
          {#each ['psi', 'rho', 'omega', 'chi', 'eta'] as dim}
            <div>
              <label class="block text-xs text-slate-400 mb-1">{dim}</label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                bind:value={seed[dim]}
                class="w-full"
              />
              <div class="text-right text-xs text-slate-400">{seed[dim].toFixed(2)}</div>
            </div>
          {/each}
        </div>
      </div>

      <!-- Main Content Panel -->
      <div class="card col-span-2">
        {#if activeTab === 'session'}
          <h2 class="text-lg font-semibold text-white mb-4">Session Mode</h2>

          <div class="mb-4">
            <label class="block text-sm text-slate-400 mb-2">Preset</label>
            <div class="flex gap-2">
              {#each presets as preset}
                <button
                  on:click={() => sessionPreset = preset.name}
                  class="flex-1 py-2 px-3 rounded text-sm font-medium transition-all"
                  class:bg-quantum-primary={sessionPreset === preset.name}
                  class:text-white={sessionPreset === preset.name}
                  class:bg-surface-700={sessionPreset !== preset.name}
                  class:text-slate-400={sessionPreset !== preset.name}
                >
                  {preset.name}
                </button>
              {/each}
            </div>
          </div>

          <button
            on:click={handleSession}
            class="w-full py-3 px-4 bg-quantum-primary rounded-lg font-medium text-white hover:opacity-90 transition-opacity disabled:opacity-50 mb-4"
            disabled={loading}
          >
            {loading ? 'Running Session...' : 'Run Session'}
          </button>

          {#if sessionResult}
            <div class="space-y-4">
              <div class="grid grid-cols-3 gap-4">
                <div class="bg-surface-700 p-4 rounded-lg text-center">
                  <div class="text-2xl font-bold text-emerald-400">{sessionResult.best_resonance.toFixed(4)}</div>
                  <div class="text-xs text-slate-400 mt-1">Best Resonance</div>
                </div>
                <div class="bg-surface-700 p-4 rounded-lg text-center">
                  <div class="text-2xl font-bold text-blue-400">{sessionResult.total_vertices}</div>
                  <div class="text-xs text-slate-400 mt-1">Vertices</div>
                </div>
                <div class="bg-surface-700 p-4 rounded-lg text-center">
                  <div class="text-2xl font-bold text-amber-400">{sessionResult.expansion_steps}</div>
                  <div class="text-xs text-slate-400 mt-1">Expansions</div>
                </div>
              </div>

              <div class="bg-surface-700 p-4 rounded-lg">
                <div class="text-sm text-slate-400 mb-2">Best Coordinate</div>
                <Coord5DDisplay coordinate={sessionResult.best_coordinate} size={150} showResonance={false} />
              </div>
            </div>
          {/if}

        {:else if activeTab === 'compile'}
          <h2 class="text-lg font-semibold text-white mb-4">Compile Hypercube</h2>

          <div class="space-y-4 mb-4">
            <div>
              <label class="block text-sm text-slate-400 mb-1">Iterations</label>
              <input type="number" bind:value={iterations} min="1" max="50" class="input w-full" />
            </div>
            <label class="flex items-center gap-2">
              <input type="checkbox" bind:checked={useTriton} class="rounded" />
              <span class="text-sm text-slate-300">Use TRITON Mode</span>
            </label>
          </div>

          <button
            on:click={handleCompile}
            class="w-full py-3 px-4 bg-quantum-primary rounded-lg font-medium text-white hover:opacity-90 transition-opacity disabled:opacity-50 mb-4"
            disabled={loading}
          >
            {loading ? 'Compiling...' : 'Compile'}
          </button>

          {#if compilationResult}
            <div class="space-y-4">
              <div class="grid grid-cols-2 gap-4">
                <div class="bg-surface-700 p-4 rounded-lg">
                  <div class="text-sm text-slate-400">Output Resonance</div>
                  <div class="text-2xl font-bold text-emerald-400">{compilationResult.resonance.toFixed(4)}</div>
                </div>
                <div class="bg-surface-700 p-4 rounded-lg">
                  <div class="text-sm text-slate-400">Artifacts</div>
                  <div class="text-2xl font-bold text-blue-400">{compilationResult.artifact_count}</div>
                </div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg">
                <div class="text-sm text-slate-400 mb-2">Output Coordinate</div>
                <Coord5DDisplay coordinate={compilationResult.output} size={150} />
              </div>
            </div>
          {/if}

        {:else if activeTab === 'expand'}
          <h2 class="text-lg font-semibold text-white mb-4">Expand Hypercube</h2>

          <div class="space-y-4 mb-4">
            <div>
              <label class="block text-sm text-slate-400 mb-1">Expansion Rule</label>
              <select bind:value={expansionRule} class="input w-full">
                {#each expansionRules as rule}
                  <option value={rule}>{rule}</option>
                {/each}
              </select>
            </div>
            <div>
              <label class="block text-sm text-slate-400 mb-1">Iterations</label>
              <input type="number" bind:value={expansionIterations} min="1" max="20" class="input w-full" />
            </div>
          </div>

          <button
            on:click={handleExpand}
            class="w-full py-3 px-4 bg-quantum-primary rounded-lg font-medium text-white hover:opacity-90 transition-opacity disabled:opacity-50 mb-4"
            disabled={loading}
          >
            {loading ? 'Expanding...' : 'Expand'}
          </button>

          {#if expansionStats}
            <div class="grid grid-cols-2 gap-4">
              <div class="bg-surface-700 p-4 rounded-lg text-center">
                <div class="text-2xl font-bold text-blue-400">{expansionStats.total_vertices}</div>
                <div class="text-xs text-slate-400 mt-1">Vertices</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg text-center">
                <div class="text-2xl font-bold text-violet-400">{expansionStats.total_edges}</div>
                <div class="text-xs text-slate-400 mt-1">Edges</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg text-center">
                <div class="text-2xl font-bold text-amber-400">{expansionStats.max_depth_reached}</div>
                <div class="text-xs text-slate-400 mt-1">Max Depth</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg text-center">
                <div class="text-2xl font-bold text-emerald-400">{expansionStats.best_resonance.toFixed(4)}</div>
                <div class="text-xs text-slate-400 mt-1">Best Resonance</div>
              </div>
            </div>
          {/if}

        {:else if activeTab === 'hdag'}
          <h2 class="text-lg font-semibold text-white mb-4">HDAG Execution</h2>

          <div class="mb-4">
            <label class="block text-sm text-slate-400 mb-1">Pipeline Type</label>
            <div class="flex gap-2">
              <button
                on:click={() => { pipelineType = 'standard'; loadHdagInfo(); }}
                class="flex-1 py-2 px-3 rounded text-sm font-medium transition-all"
                class:bg-quantum-primary={pipelineType === 'standard'}
                class:text-white={pipelineType === 'standard'}
                class:bg-surface-700={pipelineType !== 'standard'}
                class:text-slate-400={pipelineType !== 'standard'}
              >
                Standard
              </button>
              <button
                on:click={() => { pipelineType = 'parallel'; loadHdagInfo(); }}
                class="flex-1 py-2 px-3 rounded text-sm font-medium transition-all"
                class:bg-quantum-primary={pipelineType === 'parallel'}
                class:text-white={pipelineType === 'parallel'}
                class:bg-surface-700={pipelineType !== 'parallel'}
                class:text-slate-400={pipelineType !== 'parallel'}
              >
                Parallel
              </button>
            </div>
          </div>

          <HDAGGraph {hdagInfo} width={500} height={300} {executingNode} {completedNodes} />

          <button
            on:click={handleHdagExecute}
            class="w-full py-3 px-4 bg-quantum-primary rounded-lg font-medium text-white hover:opacity-90 transition-opacity disabled:opacity-50 mt-4 mb-4"
            disabled={loading}
          >
            {loading ? 'Executing...' : 'Execute HDAG'}
          </button>

          {#if hdagResult}
            <div class="grid grid-cols-2 gap-4">
              <div class="bg-surface-700 p-4 rounded-lg text-center">
                <div class="text-2xl font-bold text-emerald-400">{hdagResult.resonance.toFixed(4)}</div>
                <div class="text-xs text-slate-400 mt-1">Output Resonance</div>
              </div>
              <div class="bg-surface-700 p-4 rounded-lg text-center">
                <div class="text-2xl font-bold text-blue-400">{hdagResult.nodes_executed}</div>
                <div class="text-xs text-slate-400 mt-1">Nodes Executed</div>
              </div>
            </div>
          {/if}
        {/if}
      </div>
    </div>

    <!-- Info panel -->
    {#if hypercubeInfo}
      <div class="mt-6 card">
        <h3 class="text-sm font-semibold text-white mb-3">Framework Info</h3>
        <div class="grid grid-cols-4 gap-4 text-xs">
          <div>
            <span class="text-slate-400">Dimensions:</span>
            <span class="text-white ml-1">{hypercubeInfo.dimensions}</span>
          </div>
          <div>
            <span class="text-slate-400">Operators:</span>
            <span class="text-white ml-1">{hypercubeOperators.join(', ')}</span>
          </div>
          <div>
            <span class="text-slate-400">Coordinates:</span>
            <span class="text-white ml-1">{hypercubeCoordinates.join(', ')}</span>
          </div>
          <div>
            <span class="text-slate-400">Version:</span>
            <span class="text-white ml-1">{hypercubeInfo.version}</span>
          </div>
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
