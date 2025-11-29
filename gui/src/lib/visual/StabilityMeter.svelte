<script lang="ts">
  /**
   * StabilityMeter - Bar visualization for HDAG execution stability
   *
   * Based on Jacobian norms:
   * - Green = stable execution
   * - Yellow = moderate stability
   * - Red = unstable
   */

  import { onMount, onDestroy } from 'svelte';
  import { easing, tween } from './animations';

  export let value: number = 0;  // Stability value (0-1)
  export let jacobianNorm: number | null = null;
  export let width: number = 200;
  export let height: number = 40;
  export let title: string = 'Stability';
  export let animated: boolean = true;
  export let showValue: boolean = true;

  let displayValue = 0;
  let animationFrame: number;
  let fluctuation = 0;

  $: barWidth = (width - 20) * Math.min(1, displayValue);
  $: stability = displayValue >= 0.8 ? 'stable' : displayValue >= 0.5 ? 'moderate' : 'unstable';
  $: barColor = displayValue >= 0.8 ? '#10b981' : displayValue >= 0.5 ? '#f59e0b' : '#ef4444';
  $: labelColor = displayValue >= 0.8 ? 'text-emerald-400' : displayValue >= 0.5 ? 'text-amber-400' : 'text-red-400';

  // Animate value changes
  $: if (animated && typeof value === 'number') {
    tween(displayValue, value, 500, easing.easeOutCubic, (v) => {
      displayValue = v;
    });
  } else {
    displayValue = value;
  }

  onMount(() => {
    if (animated) {
      function animate() {
        // Add slight fluctuation for visual interest
        fluctuation = Math.sin(Date.now() / 200) * (1 - displayValue) * 2;
        animationFrame = requestAnimationFrame(animate);
      }
      animate();
    }
  });

  onDestroy(() => {
    if (animationFrame) {
      cancelAnimationFrame(animationFrame);
    }
  });
</script>

<div class="stability-meter" style="width: {width}px;">
  <!-- Title row -->
  <div class="flex items-center justify-between mb-1">
    <span class="text-xs text-slate-400">{title}</span>
    <span class="text-xs font-medium {labelColor}">{stability}</span>
  </div>

  <!-- Bar container -->
  <div class="relative" style="height: {height}px;">
    <!-- Background -->
    <div
      class="absolute inset-0 bg-surface-700 rounded overflow-hidden"
      style="height: {height}px;"
    >
      <!-- Grid lines -->
      {#each [0.25, 0.5, 0.75] as mark}
        <div
          class="absolute top-0 bottom-0 w-px bg-surface-600"
          style="left: {mark * 100}%;"
        ></div>
      {/each}
    </div>

    <!-- Stability bar -->
    <div
      class="absolute top-1 bottom-1 left-1 rounded transition-all duration-300"
      style="
        width: {Math.max(4, barWidth + fluctuation)}px;
        background: linear-gradient(90deg, {barColor}cc, {barColor});
        box-shadow: 0 0 10px {barColor}55;
      "
    ></div>

    <!-- Danger zone indicator -->
    <div
      class="absolute top-0 bottom-0 left-0 bg-red-500/20 rounded-l"
      style="width: 25%;"
    ></div>

    <!-- Safe zone indicator -->
    <div
      class="absolute top-0 bottom-0 right-0 bg-emerald-500/10 rounded-r"
      style="width: 20%;"
    ></div>

    <!-- Threshold markers -->
    <div
      class="absolute top-0 bottom-0 w-0.5 bg-amber-500/50"
      style="left: 50%;"
    ></div>
    <div
      class="absolute top-0 bottom-0 w-0.5 bg-emerald-500/50"
      style="left: 80%;"
    ></div>

    <!-- Value indicator -->
    {#if showValue}
      <div
        class="absolute top-1/2 -translate-y-1/2 text-white text-sm font-bold font-mono"
        style="left: {Math.max(30, Math.min(width - 40, barWidth + 10))}px;"
      >
        {(displayValue * 100).toFixed(0)}%
      </div>
    {/if}
  </div>

  <!-- Jacobian norm display -->
  {#if jacobianNorm !== null}
    <div class="flex items-center justify-between mt-1 text-xs">
      <span class="text-slate-500">Jacobian Norm:</span>
      <span class="text-slate-400 font-mono">{jacobianNorm.toFixed(4)}</span>
    </div>
  {/if}

  <!-- Legend -->
  <div class="flex items-center gap-3 mt-2 text-xs text-slate-500">
    <div class="flex items-center gap-1">
      <div class="w-2 h-2 rounded-full bg-red-500"></div>
      <span>&lt;50%</span>
    </div>
    <div class="flex items-center gap-1">
      <div class="w-2 h-2 rounded-full bg-amber-500"></div>
      <span>50-80%</span>
    </div>
    <div class="flex items-center gap-1">
      <div class="w-2 h-2 rounded-full bg-emerald-500"></div>
      <span>&gt;80%</span>
    </div>
  </div>
</div>

<style>
  .stability-meter {
    padding: 8px;
    background: rgba(15, 23, 42, 0.5);
    border-radius: 8px;
  }
</style>
