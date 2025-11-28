<script lang="ts">
  /**
   * SlotMachineVisualizer - Visualizes the Quantum Slots engine
   *
   * Displays slot reels with symbols, spin animation, and mined sequences.
   */

  import type { MinedSequenceDto, SlotsSessionResultDto } from '$lib/tauri/commands';

  export let sessionResult: SlotsSessionResultDto | null = null;
  export let isSpinning = false;
  export let reelCount = 5;

  // Symbol definitions
  const symbols = [
    { name: 'psi', label: 'psi', color: '#3b82f6', weight: 0.4 },
    { name: 'rho', label: 'rho', color: '#8b5cf6', weight: 0.3 },
    { name: 'omega', label: 'omega', color: '#f59e0b', weight: 0.3 },
    { name: 'chi', label: 'chi', color: '#06b6d4', weight: 0.05 },
    { name: 'eta', label: 'eta', color: '#ef4444', weight: -0.05 },
    { name: 'Star', label: 'star', color: '#fbbf24', weight: 0.1 },
    { name: 'Diamond', label: 'diamond', color: '#a855f7', weight: 0.15 },
    { name: 'Circle', label: 'circle', color: '#10b981', weight: 0.05 },
  ];

  // Current reel symbols (displayed)
  let reelSymbols: string[] = Array(reelCount).fill('psi');
  let spinOffsets: number[] = Array(reelCount).fill(0);

  // Update displayed symbols when session result changes
  $: if (sessionResult?.best_sequence) {
    reelSymbols = sessionResult.best_sequence.symbols.slice(0, reelCount);
    // Pad with empty if needed
    while (reelSymbols.length < reelCount) {
      reelSymbols.push('psi');
    }
  }

  // Spin animation
  let spinInterval: ReturnType<typeof setInterval>;

  $: if (isSpinning) {
    startSpin();
  } else {
    stopSpin();
  }

  function startSpin() {
    spinInterval = setInterval(() => {
      spinOffsets = spinOffsets.map(() => Math.random() * 360);
    }, 100);
  }

  function stopSpin() {
    if (spinInterval) {
      clearInterval(spinInterval);
      spinOffsets = Array(reelCount).fill(0);
    }
  }

  function getSymbolConfig(name: string) {
    return symbols.find(s => s.name.toLowerCase() === name.toLowerCase()) || symbols[0];
  }

  function formatResonance(value: number): string {
    return (value * 100).toFixed(1) + '%';
  }
</script>

<div class="slot-machine-visualizer">
  <!-- Main slot machine display -->
  <div class="bg-gradient-to-b from-surface-800 to-surface-900 rounded-xl p-4 border border-surface-700">
    <!-- Header -->
    <div class="text-center mb-4">
      <h3 class="text-lg font-bold text-white">Quantum Slots</h3>
      <p class="text-xs text-slate-400">5D Resonance Engine</p>
    </div>

    <!-- Reels container -->
    <div class="flex justify-center gap-2 mb-4">
      {#each reelSymbols as symbol, idx}
        {@const config = getSymbolConfig(symbol)}
        <div
          class="reel w-16 h-20 bg-surface-950 rounded-lg border-2 border-surface-600 flex items-center justify-center overflow-hidden"
          class:spinning={isSpinning}
          style="transform: translateY({isSpinning ? spinOffsets[idx] % 20 - 10 : 0}px)"
        >
          <div
            class="symbol flex flex-col items-center justify-center"
            style="color: {config.color}"
          >
            <!-- Symbol display -->
            <div class="text-2xl font-bold">
              {#if config.label === 'psi'}
                psi
              {:else if config.label === 'rho'}
                rho
              {:else if config.label === 'omega'}
                omega
              {:else if config.label === 'chi'}
                chi
              {:else if config.label === 'eta'}
                eta
              {:else if config.label === 'star'}
                star
              {:else if config.label === 'diamond'}
                diamond
              {:else if config.label === 'circle'}
                circle
              {:else}
                ?
              {/if}
            </div>
            <div class="text-xs mt-1 opacity-70">{config.name}</div>
          </div>
        </div>
      {/each}
    </div>

    <!-- Win line -->
    <div class="h-1 bg-gradient-to-r from-transparent via-amber-500 to-transparent mb-4"></div>

    <!-- Resonance display -->
    {#if sessionResult?.best_resonance}
      <div class="text-center">
        <div class="text-3xl font-bold" style="color: {sessionResult.best_resonance >= 0.8 ? '#10b981' : sessionResult.best_resonance >= 0.5 ? '#f59e0b' : '#ef4444'}">
          {formatResonance(sessionResult.best_resonance)}
        </div>
        <div class="text-xs text-slate-400 mt-1">Resonance Score</div>
      </div>
    {/if}
  </div>

  <!-- Statistics panel -->
  {#if sessionResult}
    <div class="mt-4 grid grid-cols-2 gap-3">
      <div class="bg-surface-800 rounded-lg p-3">
        <div class="text-xs text-slate-400">Spins</div>
        <div class="text-lg font-bold text-white">{sessionResult.spin_count}</div>
      </div>
      <div class="bg-surface-800 rounded-lg p-3">
        <div class="text-xs text-slate-400">Time</div>
        <div class="text-lg font-bold text-white">{sessionResult.total_time_ms}ms</div>
      </div>
    </div>

    <!-- Mining result -->
    {#if sessionResult.mining_result}
      <div class="mt-4 bg-surface-800 rounded-lg p-4">
        <h4 class="text-sm font-medium text-white mb-3">Mining Results</h4>
        <div class="grid grid-cols-2 gap-2 text-xs">
          <div>
            <span class="text-slate-400">Best Resonance:</span>
            <span class="text-emerald-400 ml-1">{sessionResult.mining_result.best_resonance.toFixed(4)}</span>
          </div>
          <div>
            <span class="text-slate-400">Total Steps:</span>
            <span class="text-white ml-1">{sessionResult.mining_result.total_steps}</span>
          </div>
          <div>
            <span class="text-slate-400">Steps to Best:</span>
            <span class="text-white ml-1">{sessionResult.mining_result.steps_to_best}</span>
          </div>
          <div>
            <span class="text-slate-400">Converged:</span>
            <span class={sessionResult.mining_result.converged ? 'text-emerald-400' : 'text-amber-400'}>
              {sessionResult.mining_result.converged ? 'Yes' : 'No'}
            </span>
          </div>
        </div>

        <!-- Top sequences -->
        {#if sessionResult.mining_result.top_sequences.length > 0}
          <div class="mt-3">
            <div class="text-xs text-slate-400 mb-2">Top Sequences</div>
            <div class="space-y-1">
              {#each sessionResult.mining_result.top_sequences.slice(0, 5) as seq, i}
                <div class="flex items-center justify-between text-xs bg-surface-900 rounded px-2 py-1">
                  <span class="text-slate-300">
                    #{i + 1} {seq.symbols.slice(0, 5).join(' ')}
                  </span>
                  <span class="text-emerald-400">{seq.resonance.toFixed(3)}</span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  {/if}

  <!-- Symbol legend -->
  <div class="mt-4 bg-surface-800 rounded-lg p-3">
    <div class="text-xs text-slate-400 mb-2">Symbol Weights</div>
    <div class="flex flex-wrap gap-2">
      {#each symbols as sym}
        <div class="flex items-center gap-1 text-xs">
          <div class="w-3 h-3 rounded" style="background-color: {sym.color}"></div>
          <span class="text-slate-300">{sym.name}</span>
          <span class="text-slate-500">({sym.weight > 0 ? '+' : ''}{sym.weight})</span>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .reel.spinning {
    animation: spin 0.1s ease-in-out infinite;
  }

  @keyframes spin {
    0%, 100% { transform: translateY(-5px); }
    50% { transform: translateY(5px); }
  }

  .symbol {
    transition: transform 0.3s ease-out;
  }
</style>
