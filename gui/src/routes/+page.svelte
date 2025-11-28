<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  interface SystemInfo {
    version: string;
    modules: string[];
    capabilities: string[];
  }

  let systemInfo: SystemInfo | null = null;
  let loading = true;
  let error: string | null = null;

  const quickActions = [
    { label: 'Run Grover', href: '/algorithm?algo=grover', color: 'quantum-primary' },
    { label: 'Run Shor', href: '/algorithm?algo=shor', color: 'quantum-secondary' },
    { label: 'Build Circuit', href: '/circuit', color: 'quantum-accent' },
    { label: 'Explore S7', href: '/topology', color: 'quantum-success' },
  ];

  onMount(async () => {
    try {
      systemInfo = await invoke<SystemInfo>('get_system_info');
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });
</script>

<div class="flex-1 overflow-auto p-6">
  <!-- Header -->
  <div class="mb-8">
    <h1 class="text-3xl font-bold text-white mb-2">Welcome to QOPS</h1>
    <p class="text-slate-400">Quantum Operator Processing System - Research Framework</p>
  </div>

  <!-- Quick Actions -->
  <div class="grid grid-cols-4 gap-4 mb-8">
    {#each quickActions as action}
      <a
        href={action.href}
        class="card card-hover flex flex-col items-center justify-center py-8 text-center"
      >
        <div class="w-12 h-12 rounded-full bg-{action.color}/20 flex items-center justify-center mb-3">
          <div class="w-6 h-6 rounded-full bg-{action.color}"></div>
        </div>
        <span class="font-medium text-white">{action.label}</span>
      </a>
    {/each}
  </div>

  <!-- System Info -->
  <div class="grid grid-cols-2 gap-6">
    <!-- Modules -->
    <div class="card">
      <h2 class="text-lg font-semibold text-white mb-4">Modules</h2>
      {#if loading}
        <div class="animate-pulse space-y-2">
          {#each Array(8) as _}
            <div class="h-6 bg-surface-700 rounded"></div>
          {/each}
        </div>
      {:else if systemInfo}
        <ul class="space-y-2">
          {#each systemInfo.modules as module}
            <li class="flex items-center gap-2 text-slate-300">
              <span class="w-2 h-2 rounded-full bg-quantum-success"></span>
              <span class="font-mono text-sm">{module}</span>
            </li>
          {/each}
        </ul>
      {:else if error}
        <p class="text-quantum-error">{error}</p>
      {/if}
    </div>

    <!-- Capabilities -->
    <div class="card">
      <h2 class="text-lg font-semibold text-white mb-4">Capabilities</h2>
      {#if loading}
        <div class="animate-pulse space-y-2">
          {#each Array(8) as _}
            <div class="h-6 bg-surface-700 rounded"></div>
          {/each}
        </div>
      {:else if systemInfo}
        <ul class="space-y-2">
          {#each systemInfo.capabilities.slice(0, 8) as cap}
            <li class="flex items-center gap-2 text-slate-300">
              <svg class="w-4 h-4 text-quantum-success" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
              </svg>
              <span class="text-sm">{cap}</span>
            </li>
          {/each}
        </ul>
      {:else if error}
        <p class="text-quantum-error">{error}</p>
      {/if}
    </div>
  </div>

  <!-- Info Cards -->
  <div class="grid grid-cols-3 gap-6 mt-6">
    <div class="card">
      <h3 class="font-semibold text-quantum-primary mb-2">Genesis Pipeline</h3>
      <p class="text-sm text-slate-400 mb-3">S7 permutation group with 5040 nodes for operator mining.</p>
      <a href="/topology" class="text-sm text-quantum-accent hover:underline">Explore Topology</a>
    </div>
    <div class="card">
      <h3 class="font-semibold text-quantum-secondary mb-2">Quantum Pipeline</h3>
      <p class="text-sm text-slate-400 mb-3">Metatron Cube-13 with 1 center + 6 hexagon + 6 cube nodes.</p>
      <a href="/resonance" class="text-sm text-quantum-accent hover:underline">View Resonance</a>
    </div>
    <div class="card">
      <h3 class="font-semibold text-quantum-accent mb-2">Circuit Simulator</h3>
      <p class="text-sm text-slate-400 mb-3">Universal gate set with state vector simulation up to 20 qubits.</p>
      <a href="/circuit" class="text-sm text-quantum-accent hover:underline">Build Circuits</a>
    </div>
  </div>
</div>
