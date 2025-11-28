<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  interface TopologyInfo {
    node_count: number;
    edge_count: number;
    topology_type: string;
  }

  interface NodeDetails {
    id: number;
    permutation: number[];
    signature: { psi: number; rho: number; omega: number; chi: number | null; eta: number | null };
    neighbor_count: number;
  }

  interface GenesisResult {
    artefacts: { id: number; resonance: number; is_mandorla: boolean; node_path: number[] }[];
    best_resonance: number;
    mandorla_count: number;
    total_steps: number;
  }

  let topologyInfo: TopologyInfo | null = null;
  let selectedNode: NodeDetails | null = null;
  let genesisResult: GenesisResult | null = null;
  let loading = false;
  let error: string | null = null;

  // Genesis params
  let agents = 5;
  let steps = 20;
  let strategy = 'balanced';

  onMount(async () => {
    await loadTopologyInfo();
  });

  async function loadTopologyInfo() {
    loading = true;
    try {
      topologyInfo = await invoke<TopologyInfo>('get_s7_topology_info');
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function loadNodeDetails(nodeId: number) {
    loading = true;
    error = null;
    try {
      selectedNode = await invoke<NodeDetails>('get_node_details', { nodeId });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function runGenesis() {
    loading = true;
    error = null;
    try {
      genesisResult = await invoke<GenesisResult>('run_genesis_mining', {
        agents,
        steps,
        strategy,
      });
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
    <h1 class="text-xl font-bold text-white">Topology Explorer</h1>
    <p class="text-slate-400 text-sm mt-1">S7 Permutation Group - 5040 nodes</p>
  </div>

  <div class="flex-1 flex overflow-hidden">
    <!-- Controls Panel -->
    <div class="w-80 bg-surface-800 border-r border-surface-700 p-4 overflow-y-auto">
      <h3 class="text-sm font-semibold text-slate-400 mb-3">Genesis Mining</h3>

      <div class="space-y-4">
        <div>
          <label class="block text-sm text-slate-400 mb-2">Agents</label>
          <input type="number" bind:value={agents} min="1" max="50" class="input w-full" />
        </div>
        <div>
          <label class="block text-sm text-slate-400 mb-2">Steps per Agent</label>
          <input type="number" bind:value={steps} min="5" max="100" class="input w-full" />
        </div>
        <div>
          <label class="block text-sm text-slate-400 mb-2">Strategy</label>
          <select bind:value={strategy} class="input w-full">
            <option value="balanced">Balanced</option>
            <option value="explorative">Explorative</option>
            <option value="exploitative">Exploitative</option>
            <option value="random">Random</option>
          </select>
        </div>

        <button on:click={runGenesis} class="btn-primary w-full" disabled={loading}>
          {#if loading}
            <span class="animate-spin mr-2">@</span>
          {/if}
          Run Mining
        </button>
      </div>

      {#if topologyInfo}
        <div class="mt-6 p-4 bg-surface-700 rounded-lg">
          <h4 class="text-sm font-semibold text-white mb-2">Topology Info</h4>
          <div class="text-sm text-slate-400 space-y-1">
            <p>Type: {topologyInfo.topology_type}</p>
            <p>Nodes: {topologyInfo.node_count}</p>
            <p>Edges: {topologyInfo.edge_count}</p>
          </div>
        </div>
      {/if}

      <div class="mt-6">
        <h4 class="text-sm font-semibold text-slate-400 mb-2">Inspect Node</h4>
        <div class="flex gap-2">
          <input
            type="number"
            min="0"
            max={topologyInfo ? topologyInfo.node_count - 1 : 100}
            placeholder="Node ID"
            class="input flex-1"
            on:change={(e) => loadNodeDetails(parseInt(e.currentTarget.value))}
          />
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 p-6 overflow-y-auto">
      {#if error}
        <div class="bg-quantum-error/20 border border-quantum-error text-quantum-error p-4 rounded-lg mb-4">
          {error}
        </div>
      {/if}

      <!-- Genesis Results -->
      {#if genesisResult}
        <div class="card mb-6">
          <h2 class="text-lg font-semibold text-white mb-4">Mining Results</h2>

          <div class="grid grid-cols-3 gap-4 mb-4">
            <div class="bg-surface-700 p-4 rounded-lg">
              <div class="text-slate-400 text-sm">Best Resonance</div>
              <div class="text-2xl font-mono text-quantum-success">{genesisResult.best_resonance.toFixed(4)}</div>
            </div>
            <div class="bg-surface-700 p-4 rounded-lg">
              <div class="text-slate-400 text-sm">Mandorla Found</div>
              <div class="text-2xl font-mono text-quantum-primary">{genesisResult.mandorla_count}</div>
            </div>
            <div class="bg-surface-700 p-4 rounded-lg">
              <div class="text-slate-400 text-sm">Total Steps</div>
              <div class="text-2xl font-mono text-white">{genesisResult.total_steps}</div>
            </div>
          </div>

          <h3 class="text-sm font-semibold text-slate-400 mb-2">Artefacts</h3>
          <div class="space-y-2 max-h-64 overflow-y-auto">
            {#each genesisResult.artefacts as artefact}
              <div class="flex items-center gap-4 p-3 bg-surface-700 rounded-lg">
                <div class="w-8 h-8 rounded-full flex items-center justify-center"
                  class:bg-quantum-success={artefact.is_mandorla}
                  class:bg-surface-600={!artefact.is_mandorla}
                >
                  {#if artefact.is_mandorla}
                    <span class="text-white font-bold">M</span>
                  {:else}
                    <span class="text-slate-400">{artefact.id}</span>
                  {/if}
                </div>
                <div class="flex-1">
                  <div class="text-white text-sm">Artefact #{artefact.id + 1}</div>
                  <div class="text-slate-400 text-xs">Resonance: {artefact.resonance.toFixed(4)}</div>
                </div>
                <div class="w-32 h-2 bg-surface-600 rounded-full overflow-hidden">
                  <div
                    class="h-full bg-quantum-primary"
                    style="width: {artefact.resonance * 100}%"
                  ></div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Node Details -->
      {#if selectedNode}
        <div class="card">
          <h2 class="text-lg font-semibold text-white mb-4">Node Details</h2>

          <div class="grid grid-cols-2 gap-4">
            <div class="bg-surface-700 p-4 rounded-lg">
              <div class="text-slate-400 text-sm mb-1">Node ID</div>
              <div class="text-xl font-mono text-white">{selectedNode.id}</div>
            </div>
            <div class="bg-surface-700 p-4 rounded-lg">
              <div class="text-slate-400 text-sm mb-1">Neighbors</div>
              <div class="text-xl font-mono text-white">{selectedNode.neighbor_count}</div>
            </div>
          </div>

          <div class="mt-4 bg-surface-700 p-4 rounded-lg">
            <div class="text-slate-400 text-sm mb-2">Permutation</div>
            <div class="font-mono text-quantum-accent text-lg">
              [{selectedNode.permutation.join(', ')}]
            </div>
          </div>

          <div class="mt-4">
            <div class="text-slate-400 text-sm mb-2">Signature</div>
            <div class="grid grid-cols-3 gap-2">
              <div class="bg-surface-700 p-3 rounded-lg text-center">
                <div class="text-xs text-slate-400">psi</div>
                <div class="text-lg font-mono text-quantum-primary">{selectedNode.signature.psi.toFixed(3)}</div>
              </div>
              <div class="bg-surface-700 p-3 rounded-lg text-center">
                <div class="text-xs text-slate-400">rho</div>
                <div class="text-lg font-mono text-quantum-secondary">{selectedNode.signature.rho.toFixed(3)}</div>
              </div>
              <div class="bg-surface-700 p-3 rounded-lg text-center">
                <div class="text-xs text-slate-400">omega</div>
                <div class="text-lg font-mono text-quantum-accent">{selectedNode.signature.omega.toFixed(3)}</div>
              </div>
            </div>
          </div>
        </div>
      {/if}

      {#if !genesisResult && !selectedNode}
        <div class="flex items-center justify-center h-full text-slate-400">
          Run Genesis mining or select a node to inspect
        </div>
      {/if}
    </div>
  </div>
</div>
