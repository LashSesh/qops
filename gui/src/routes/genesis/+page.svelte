<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  // Stage enum values
  type GenesisStage = 'discovery' | 'kosmokrator' | 'chronokrator' | 'pfauenthron' | 'finalized';

  // Configuration interfaces
  interface KosmokratorConfig {
    kappa_threshold: number;
    stability_epsilon: number;
    telescope_enabled: boolean;
    history_window: number;
  }

  interface ChronokratorConfig {
    num_channels: number;
    base_threshold: number;
    exkalibration_enabled: boolean;
    spike_detection: boolean;
  }

  interface PfauenthronConfig {
    mandorla_threshold: number;
    ophanim_count: number;
    monolith_enabled: boolean;
  }

  interface HolisticMiningConfig {
    kosmokrator: KosmokratorConfig;
    chronokrator: ChronokratorConfig;
    pfauenthron: PfauenthronConfig;
    num_agents: number;
    steps_per_agent: number;
    use_adaptive_triton: boolean;
    preset: string;
  }

  // Result interfaces
  interface FamilyCharacteristics {
    is_high_quality: boolean;
    is_stable: boolean;
    is_efficient: boolean;
  }

  interface FinalizedFamily {
    name: string;
    member_count: number;
    avg_resonance: number;
    characteristics: FamilyCharacteristics;
    finalization_time: string;
  }

  interface StageMetrics {
    input_count: number;
    output_count: number;
    avg_resonance: number;
    duration_ms: number;
  }

  interface StageLog {
    stage: GenesisStage;
    message: string;
    timestamp: string;
    metrics: StageMetrics | null;
  }

  interface Monolith {
    coherence: number;
    family_count: number;
    families: FinalizedFamily[];
    finalized: boolean;
    creation_time: string;
  }

  interface HolisticMiningResult {
    stage: GenesisStage;
    candidates_discovered: number;
    candidates_after_kosmokrator: number;
    candidates_after_chronokrator: number;
    finalized_families: FinalizedFamily[];
    best_resonance: number;
    matrix_outputs: number;
    monolith: Monolith | null;
    duration_ms: number;
    stage_logs: StageLog[];
  }

  // State
  let config: HolisticMiningConfig = {
    kosmokrator: { kappa_threshold: 0.7, stability_epsilon: 0.05, telescope_enabled: true, history_window: 50 },
    chronokrator: { num_channels: 4, base_threshold: 0.75, exkalibration_enabled: true, spike_detection: true },
    pfauenthron: { mandorla_threshold: 0.8, ophanim_count: 4, monolith_enabled: true },
    num_agents: 10,
    steps_per_agent: 50,
    use_adaptive_triton: true,
    preset: 'thorough',
  };

  let result: HolisticMiningResult | null = null;
  let loading = false;
  let error: string | null = null;
  let currentStage: GenesisStage = 'discovery';
  let stageProgress = 0;
  let animatingStage = false;

  // Preset options
  const presets = ['quick', 'thorough', 'research'];

  // Stage configuration
  const stages: { id: GenesisStage; name: string; color: string; bgColor: string; description: string }[] = [
    { id: 'discovery', name: 'Discovery', color: 'text-blue-400', bgColor: 'bg-blue-500', description: 'Operator candidate generation' },
    { id: 'kosmokrator', name: 'Kosmokrator', color: 'text-blue-500', bgColor: 'bg-blue-600', description: 'Proof-of-Resonance filtering' },
    { id: 'chronokrator', name: 'Chronokrator', color: 'text-violet-500', bgColor: 'bg-violet-600', description: 'Resonance expansion dynamics' },
    { id: 'pfauenthron', name: 'Pfauenthron', color: 'text-amber-400', bgColor: 'bg-amber-500', description: 'Mandorla convergence' },
    { id: 'finalized', name: 'Monolith', color: 'text-emerald-400', bgColor: 'bg-emerald-500', description: 'Finalized structure' },
  ];

  function applyPreset(preset: string) {
    config.preset = preset;
    switch (preset) {
      case 'quick':
        config.kosmokrator.kappa_threshold = 0.6;
        config.chronokrator.num_channels = 2;
        config.pfauenthron.mandorla_threshold = 0.7;
        config.num_agents = 5;
        config.steps_per_agent = 20;
        config.use_adaptive_triton = false;
        break;
      case 'research':
        config.kosmokrator.kappa_threshold = 0.8;
        config.kosmokrator.history_window = 100;
        config.chronokrator.num_channels = 6;
        config.pfauenthron.mandorla_threshold = 0.9;
        config.pfauenthron.ophanim_count = 6;
        config.num_agents = 20;
        config.steps_per_agent = 100;
        config.use_adaptive_triton = true;
        break;
      default: // thorough
        config.kosmokrator.kappa_threshold = 0.7;
        config.kosmokrator.history_window = 50;
        config.chronokrator.num_channels = 4;
        config.pfauenthron.mandorla_threshold = 0.8;
        config.pfauenthron.ophanim_count = 4;
        config.num_agents = 10;
        config.steps_per_agent = 50;
        config.use_adaptive_triton = true;
    }
  }

  async function runHolisticMining() {
    loading = true;
    error = null;
    result = null;
    stageProgress = 0;
    currentStage = 'discovery';
    animatingStage = true;

    // Animate through stages
    const stageDelays = [500, 800, 1000, 1200];
    for (let i = 0; i < stages.length - 1; i++) {
      currentStage = stages[i].id;
      stageProgress = ((i + 1) / stages.length) * 100;
      await new Promise(resolve => setTimeout(resolve, stageDelays[i] || 500));
    }

    try {
      result = await invoke<HolisticMiningResult>('run_holistic_mining', { config });
      currentStage = 'finalized';
      stageProgress = 100;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
      animatingStage = false;
    }
  }

  async function exportResults(format: string) {
    if (!result) return;
    try {
      const output = await invoke<string>('export_holistic_results', {
        result,
        format,
        path: null,
      });
      // Create download
      const blob = new Blob([output], { type: format === 'json' ? 'application/json' : 'text/plain' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `genesis_holistic.${format}`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (e) {
      error = String(e);
    }
  }

  function getStageIndex(stage: GenesisStage): number {
    return stages.findIndex(s => s.id === stage);
  }

  function isStageActive(stage: GenesisStage): boolean {
    return getStageIndex(currentStage) >= getStageIndex(stage);
  }

  function isStageComplete(stage: GenesisStage): boolean {
    return getStageIndex(currentStage) > getStageIndex(stage);
  }
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <!-- Header -->
  <div class="p-4 border-b border-surface-700">
    <h1 class="text-xl font-bold text-white">Genesis Holistic Miner</h1>
    <p class="text-slate-400 text-sm mt-1">Multi-stage S7 operator mining with Kosmokrator -> Chronokrator -> Pfauenthron pipeline</p>
  </div>

  <div class="flex-1 p-6 overflow-y-auto">
    {#if error}
      <div class="bg-red-500/20 border border-red-500 text-red-400 p-4 rounded-lg mb-4">
        {error}
      </div>
    {/if}

    <!-- Stage Pipeline Visualization -->
    <div class="card mb-6">
      <h2 class="text-lg font-semibold text-white mb-4">Resonance Pipeline</h2>

      <!-- Progress bar -->
      <div class="h-2 bg-surface-700 rounded-full mb-4 overflow-hidden">
        <div
          class="h-full transition-all duration-500 ease-out"
          class:bg-gradient-to-r={true}
          class:from-blue-500={true}
          class:via-violet-500={true}
          class:to-amber-500={true}
          style="width: {stageProgress}%"
        ></div>
      </div>

      <!-- Stage indicators -->
      <div class="flex justify-between items-center">
        {#each stages as stage, i}
          <div class="flex flex-col items-center" style="width: {100 / stages.length}%">
            <div
              class="w-12 h-12 rounded-full flex items-center justify-center transition-all duration-500 relative"
              class:bg-surface-600={!isStageActive(stage.id)}
              class:scale-110={currentStage === stage.id && animatingStage}
            >
              <!-- Background circle with stage color when active -->
              {#if isStageActive(stage.id)}
                <div class="absolute inset-0 {stage.bgColor} rounded-full opacity-80"></div>
              {/if}

              <!-- Pulse animation for current stage -->
              {#if currentStage === stage.id && animatingStage}
                <div class="absolute inset-0 {stage.bgColor} rounded-full animate-ping opacity-30"></div>
              {/if}

              <!-- Stage icon/number -->
              <span class="relative z-10 font-bold text-white">
                {#if isStageComplete(stage.id)}
                  <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                {:else}
                  {i + 1}
                {/if}
              </span>
            </div>

            <div class="mt-2 text-center">
              <div class="text-sm font-medium {isStageActive(stage.id) ? stage.color : 'text-slate-500'}">
                {stage.name}
              </div>
              <div class="text-xs text-slate-500 mt-1 hidden sm:block">
                {stage.description}
              </div>
            </div>
          </div>

          <!-- Connector line -->
          {#if i < stages.length - 1}
            <div
              class="h-0.5 flex-1 transition-all duration-500"
              class:bg-surface-600={!isStageComplete(stages[i].id)}
              class:bg-gradient-to-r={isStageComplete(stages[i].id)}
              class:from-blue-500={isStageComplete(stages[i].id) && i === 0}
              class:to-violet-500={isStageComplete(stages[i].id) && i === 1}
              class:from-violet-500={isStageComplete(stages[i].id) && i === 2}
              class:to-amber-500={isStageComplete(stages[i].id) && i === 2}
            ></div>
          {/if}
        {/each}
      </div>
    </div>

    <div class="grid grid-cols-3 gap-6">
      <!-- Configuration Panel -->
      <div class="card col-span-1">
        <h2 class="text-lg font-semibold text-white mb-4">Configuration</h2>

        <!-- Preset selector -->
        <div class="mb-4">
          <label class="block text-sm text-slate-400 mb-2">Preset</label>
          <div class="flex gap-2">
            {#each presets as preset}
              <button
                on:click={() => applyPreset(preset)}
                class="flex-1 py-2 px-3 rounded text-sm font-medium transition-all"
                class:bg-quantum-primary={config.preset === preset}
                class:text-white={config.preset === preset}
                class:bg-surface-700={config.preset !== preset}
                class:text-slate-400={config.preset !== preset}
              >
                {preset.charAt(0).toUpperCase() + preset.slice(1)}
              </button>
            {/each}
          </div>
        </div>

        <!-- Mining parameters -->
        <div class="space-y-3 mb-4">
          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block text-xs text-slate-400 mb-1">Agents</label>
              <input type="number" bind:value={config.num_agents} min="1" max="50" class="input w-full text-sm" />
            </div>
            <div>
              <label class="block text-xs text-slate-400 mb-1">Steps</label>
              <input type="number" bind:value={config.steps_per_agent} min="10" max="200" class="input w-full text-sm" />
            </div>
          </div>

          <label class="flex items-center gap-2">
            <input type="checkbox" bind:checked={config.use_adaptive_triton} class="rounded bg-surface-700 border-surface-600" />
            <span class="text-sm text-slate-300">Adaptive TRITON</span>
          </label>
        </div>

        <!-- Kosmokrator config -->
        <div class="border-t border-surface-700 pt-3 mb-3">
          <h3 class="text-sm font-medium text-blue-400 mb-2">Kosmokrator</h3>
          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block text-xs text-slate-400 mb-1">kappa</label>
              <input type="number" bind:value={config.kosmokrator.kappa_threshold} min="0" max="1" step="0.05" class="input w-full text-sm" />
            </div>
            <div>
              <label class="block text-xs text-slate-400 mb-1">epsilon</label>
              <input type="number" bind:value={config.kosmokrator.stability_epsilon} min="0" max="0.5" step="0.01" class="input w-full text-sm" />
            </div>
          </div>
        </div>

        <!-- Chronokrator config -->
        <div class="border-t border-surface-700 pt-3 mb-3">
          <h3 class="text-sm font-medium text-violet-400 mb-2">Chronokrator</h3>
          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block text-xs text-slate-400 mb-1">Channels</label>
              <input type="number" bind:value={config.chronokrator.num_channels} min="2" max="8" class="input w-full text-sm" />
            </div>
            <div>
              <label class="block text-xs text-slate-400 mb-1">Threshold</label>
              <input type="number" bind:value={config.chronokrator.base_threshold} min="0" max="1" step="0.05" class="input w-full text-sm" />
            </div>
          </div>
        </div>

        <!-- Pfauenthron config -->
        <div class="border-t border-surface-700 pt-3 mb-4">
          <h3 class="text-sm font-medium text-amber-400 mb-2">Pfauenthron</h3>
          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block text-xs text-slate-400 mb-1">Mandorla</label>
              <input type="number" bind:value={config.pfauenthron.mandorla_threshold} min="0" max="1" step="0.05" class="input w-full text-sm" />
            </div>
            <div>
              <label class="block text-xs text-slate-400 mb-1">Ophanim</label>
              <input type="number" bind:value={config.pfauenthron.ophanim_count} min="2" max="8" class="input w-full text-sm" />
            </div>
          </div>
        </div>

        <button
          on:click={runHolisticMining}
          class="w-full py-3 px-4 bg-gradient-to-r from-blue-600 via-violet-600 to-amber-500 rounded-lg font-medium text-white hover:opacity-90 transition-opacity disabled:opacity-50"
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
            Start Holistic Mining
          {/if}
        </button>
      </div>

      <!-- Results Panel -->
      <div class="card col-span-2">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-lg font-semibold text-white">Results</h2>
          {#if result}
            <div class="flex gap-2">
              <button on:click={() => exportResults('json')} class="text-xs bg-surface-700 hover:bg-surface-600 text-slate-300 px-3 py-1 rounded">
                Export JSON
              </button>
              <button on:click={() => exportResults('csv')} class="text-xs bg-surface-700 hover:bg-surface-600 text-slate-300 px-3 py-1 rounded">
                Export CSV
              </button>
              <button on:click={() => exportResults('md')} class="text-xs bg-surface-700 hover:bg-surface-600 text-slate-300 px-3 py-1 rounded">
                Export MD
              </button>
            </div>
          {/if}
        </div>

        {#if result}
          <!-- Summary stats -->
          <div class="grid grid-cols-4 gap-4 mb-6">
            <div class="bg-surface-700 p-4 rounded-lg text-center">
              <div class="text-2xl font-bold text-blue-400">{result.candidates_discovered}</div>
              <div class="text-xs text-slate-400 mt-1">Discovered</div>
            </div>
            <div class="bg-surface-700 p-4 rounded-lg text-center">
              <div class="text-2xl font-bold text-violet-400">{result.candidates_after_kosmokrator}</div>
              <div class="text-xs text-slate-400 mt-1">After Kosmokrator</div>
            </div>
            <div class="bg-surface-700 p-4 rounded-lg text-center">
              <div class="text-2xl font-bold text-amber-400">{result.candidates_after_chronokrator}</div>
              <div class="text-xs text-slate-400 mt-1">After Chronokrator</div>
            </div>
            <div class="bg-surface-700 p-4 rounded-lg text-center">
              <div class="text-2xl font-bold text-emerald-400">{result.finalized_families.length}</div>
              <div class="text-xs text-slate-400 mt-1">Families</div>
            </div>
          </div>

          <!-- Best resonance & duration -->
          <div class="flex gap-4 mb-6">
            <div class="flex-1 bg-gradient-to-r from-blue-500/20 via-violet-500/20 to-amber-500/20 border border-surface-600 p-4 rounded-lg">
              <div class="text-sm text-slate-400">Best Resonance</div>
              <div class="text-3xl font-bold text-white">{result.best_resonance.toFixed(4)}</div>
            </div>
            <div class="flex-1 bg-surface-700 p-4 rounded-lg">
              <div class="text-sm text-slate-400">Duration</div>
              <div class="text-3xl font-bold text-white">{result.duration_ms} ms</div>
            </div>
            <div class="flex-1 bg-surface-700 p-4 rounded-lg">
              <div class="text-sm text-slate-400">Matrix Outputs</div>
              <div class="text-3xl font-bold text-white">{result.matrix_outputs}</div>
            </div>
          </div>

          <!-- Monolith status -->
          {#if result.monolith}
            <div class="bg-emerald-500/10 border border-emerald-500/50 p-4 rounded-lg mb-6">
              <div class="flex items-center gap-2 mb-2">
                <div class="w-3 h-3 bg-emerald-500 rounded-full animate-pulse"></div>
                <span class="font-semibold text-emerald-400">MONOLITH FORMED</span>
              </div>
              <div class="grid grid-cols-3 gap-4 text-sm">
                <div>
                  <span class="text-slate-400">Coherence:</span>
                  <span class="text-white ml-2">{result.monolith.coherence.toFixed(4)}</span>
                </div>
                <div>
                  <span class="text-slate-400">Families:</span>
                  <span class="text-white ml-2">{result.monolith.family_count}</span>
                </div>
                <div>
                  <span class="text-slate-400">Finalized:</span>
                  <span class="ml-2" class:text-emerald-400={result.monolith.finalized} class:text-amber-400={!result.monolith.finalized}>
                    {result.monolith.finalized ? 'Yes' : 'In progress'}
                  </span>
                </div>
              </div>
            </div>
          {/if}

          <!-- Finalized families table -->
          {#if result.finalized_families.length > 0}
            <div class="mb-6">
              <h3 class="text-sm font-semibold text-white mb-3">Finalized Operator Families</h3>
              <div class="max-h-64 overflow-y-auto">
                <table class="w-full text-sm">
                  <thead class="text-slate-400 sticky top-0 bg-surface-800">
                    <tr>
                      <th class="text-left py-2 px-3">Name</th>
                      <th class="text-right py-2 px-3">Members</th>
                      <th class="text-right py-2 px-3">Resonance</th>
                      <th class="text-center py-2 px-3">Quality</th>
                      <th class="text-center py-2 px-3">Stable</th>
                      <th class="text-center py-2 px-3">Efficient</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each result.finalized_families as family}
                      <tr class="border-t border-surface-700 hover:bg-surface-700/50">
                        <td class="py-2 px-3 text-slate-300">{family.name}</td>
                        <td class="py-2 px-3 text-right font-mono">{family.member_count}</td>
                        <td class="py-2 px-3 text-right font-mono text-amber-400">{family.avg_resonance.toFixed(4)}</td>
                        <td class="py-2 px-3 text-center">
                          {#if family.characteristics.is_high_quality}
                            <span class="text-emerald-400">Y</span>
                          {:else}
                            <span class="text-slate-500">-</span>
                          {/if}
                        </td>
                        <td class="py-2 px-3 text-center">
                          {#if family.characteristics.is_stable}
                            <span class="text-emerald-400">Y</span>
                          {:else}
                            <span class="text-slate-500">-</span>
                          {/if}
                        </td>
                        <td class="py-2 px-3 text-center">
                          {#if family.characteristics.is_efficient}
                            <span class="text-emerald-400">Y</span>
                          {:else}
                            <span class="text-slate-500">-</span>
                          {/if}
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            </div>
          {/if}

          <!-- Stage logs -->
          {#if result.stage_logs.length > 0}
            <div>
              <h3 class="text-sm font-semibold text-white mb-3">Stage Logs</h3>
              <div class="max-h-48 overflow-y-auto space-y-2">
                {#each result.stage_logs as log}
                  <div class="bg-surface-700 p-3 rounded text-sm">
                    <div class="flex items-center justify-between mb-1">
                      <span class="font-medium"
                        class:text-blue-400={log.stage === 'kosmokrator' || log.stage === 'discovery'}
                        class:text-violet-400={log.stage === 'chronokrator'}
                        class:text-amber-400={log.stage === 'pfauenthron'}
                        class:text-emerald-400={log.stage === 'finalized'}
                      >
                        {log.stage.toUpperCase()}
                      </span>
                      <span class="text-xs text-slate-500">{log.timestamp}</span>
                    </div>
                    <p class="text-slate-300">{log.message}</p>
                    {#if log.metrics}
                      <div class="mt-2 flex gap-4 text-xs text-slate-400">
                        <span>In: {log.metrics.input_count}</span>
                        <span>Out: {log.metrics.output_count}</span>
                        <span>Avg res: {log.metrics.avg_resonance.toFixed(4)}</span>
                        <span>Duration: {log.metrics.duration_ms}ms</span>
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        {:else}
          <div class="flex flex-col items-center justify-center h-64 text-slate-500">
            <svg class="w-16 h-16 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
            </svg>
            <p class="text-lg">Configure and run holistic mining</p>
            <p class="text-sm mt-2">The pipeline will guide operators through Kosmokrator, Chronokrator, and Pfauenthron stages</p>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .card {
    @apply bg-surface-800 border border-surface-700 rounded-lg p-4;
  }

  .input {
    @apply bg-surface-700 border border-surface-600 rounded px-3 py-2 text-white focus:outline-none focus:border-quantum-primary;
  }

  @keyframes ping {
    75%, 100% {
      transform: scale(1.5);
      opacity: 0;
    }
  }

  .animate-ping {
    animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
  }
</style>
