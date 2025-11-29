<script lang="ts">
  /**
   * CascadePlayer - Animated FUQ! Cascade dimensional collapse visualization
   *
   * Animates the flow from Hⁿ → H⁵ → H³ → H² → H¹:
   * - Dimensional axes shrink
   * - Eigenvalue collapse animation
   * - State cloud compression
   * - Color-coded transitions
   */

  import { onMount, onDestroy } from 'svelte';
  import { cascadeColors } from './colors';
  import { easing, createCollapseAnimation } from './animations';

  export let currentDimension: number = 5;
  export let eigenvalues: number[] = [];
  export let isPlaying: boolean = false;
  export let width: number = 400;
  export let height: number = 300;
  export let onDimensionChange: ((dim: number) => void) | null = null;

  interface CascadeStage {
    from: number;
    to: number;
    color: string;
    label: string;
  }

  const cascadeStages: CascadeStage[] = [
    { from: 5, to: 5, color: cascadeColors.n_to_5, label: 'Hⁿ → H⁵' },
    { from: 5, to: 3, color: cascadeColors.d5_to_3, label: 'H⁵ → H³' },
    { from: 3, to: 2, color: cascadeColors.d3_to_2, label: 'H³ → H²' },
    { from: 2, to: 1, color: cascadeColors.d2_to_1, label: 'H² → H¹' },
  ];

  let stageIndex = 0;
  let stageProgress = 0;
  let animationFrame: number;
  let particles: Array<{ x: number; y: number; vx: number; vy: number; size: number; dim: number }> = [];
  let lastTime = 0;

  $: currentStage = cascadeStages[Math.min(stageIndex, cascadeStages.length - 1)];
  $: effectiveDimension = currentStage.from + (currentStage.to - currentStage.from) * stageProgress;
  $: centerX = width / 2;
  $: centerY = height / 2;

  // Initialize particles representing state cloud
  function initParticles(count: number = 50) {
    particles = Array.from({ length: count }, () => ({
      x: centerX + (Math.random() - 0.5) * 150,
      y: centerY + (Math.random() - 0.5) * 150,
      vx: (Math.random() - 0.5) * 2,
      vy: (Math.random() - 0.5) * 2,
      size: 2 + Math.random() * 4,
      dim: 5,
    }));
  }

  // Update particle positions based on dimension
  function updateParticles(dim: number) {
    const compressionFactor = dim / 5;
    const maxRadius = 120 * compressionFactor;

    particles = particles.map((p) => {
      // Move toward center as dimension decreases
      const dx = centerX - p.x;
      const dy = centerY - p.y;
      const dist = Math.sqrt(dx * dx + dy * dy);

      let newX = p.x + p.vx;
      let newY = p.y + p.vy;

      // Constrain to shrinking bounds
      if (dist > maxRadius) {
        newX = centerX + (dx / dist) * maxRadius * -1;
        newY = centerY + (dy / dist) * maxRadius * -1;
      }

      // Bounce off boundaries
      if (Math.abs(newX - centerX) > maxRadius) {
        p.vx *= -0.8;
        newX = centerX + Math.sign(newX - centerX) * maxRadius;
      }
      if (Math.abs(newY - centerY) > maxRadius) {
        p.vy *= -0.8;
        newY = centerY + Math.sign(newY - centerY) * maxRadius;
      }

      return {
        ...p,
        x: newX,
        y: newY,
        size: 2 + (dim / 5) * 4,
        dim,
      };
    });
  }

  function animate(timestamp: number) {
    if (!lastTime) lastTime = timestamp;
    const delta = (timestamp - lastTime) / 1000;
    lastTime = timestamp;

    if (isPlaying) {
      stageProgress += delta * 0.5; // Speed of animation

      if (stageProgress >= 1) {
        stageProgress = 0;
        stageIndex++;

        if (stageIndex >= cascadeStages.length) {
          stageIndex = cascadeStages.length - 1;
          stageProgress = 1;
        }

        onDimensionChange?.(cascadeStages[stageIndex].to);
      }

      updateParticles(effectiveDimension);
    }

    animationFrame = requestAnimationFrame(animate);
  }

  onMount(() => {
    initParticles();
    animationFrame = requestAnimationFrame(animate);
  });

  onDestroy(() => {
    if (animationFrame) {
      cancelAnimationFrame(animationFrame);
    }
  });

  export function play() {
    isPlaying = true;
  }

  export function pause() {
    isPlaying = false;
  }

  export function reset() {
    stageIndex = 0;
    stageProgress = 0;
    initParticles();
    onDimensionChange?.(5);
  }

  export function stepForward() {
    if (stageIndex < cascadeStages.length - 1) {
      stageIndex++;
      stageProgress = 0;
      onDimensionChange?.(cascadeStages[stageIndex].to);
    }
  }

  export function stepBackward() {
    if (stageIndex > 0) {
      stageIndex--;
      stageProgress = 0;
      onDimensionChange?.(cascadeStages[stageIndex].from);
    }
  }

  // Generate dimensional axes for visualization
  function getDimensionalAxes(dim: number) {
    const axes = [];
    const axisLength = 80 * (dim / 5);

    for (let i = 0; i < Math.ceil(dim); i++) {
      const angle = (i / dim) * Math.PI * 2 - Math.PI / 2;
      axes.push({
        x1: centerX,
        y1: centerY,
        x2: centerX + axisLength * Math.cos(angle),
        y2: centerY + axisLength * Math.sin(angle),
        opacity: i < dim ? 1 : dim - Math.floor(dim),
      });
    }
    return axes;
  }

  $: axes = getDimensionalAxes(effectiveDimension);

  // Eigenvalue bar heights
  $: eigenBars = eigenvalues.slice(0, 5).map((v, i) => ({
    value: v,
    height: Math.max(5, v * 60 * (effectiveDimension / 5)),
    visible: i < effectiveDimension,
  }));
</script>

<div class="cascade-player" style="width: {width}px;">
  <!-- Main visualization -->
  <div class="relative bg-surface-900 rounded-lg overflow-hidden" style="height: {height}px;">
    <svg {width} {height}>
      <!-- Background grid -->
      <defs>
        <pattern id="cascadeGrid" width="20" height="20" patternUnits="userSpaceOnUse">
          <path d="M 20 0 L 0 0 0 20" fill="none" stroke="rgba(255,255,255,0.03)" stroke-width="0.5"/>
        </pattern>
        <radialGradient id="collapseGlow" cx="50%" cy="50%" r="50%">
          <stop offset="0%" style="stop-color:{currentStage.color};stop-opacity:0.3" />
          <stop offset="100%" style="stop-color:{currentStage.color};stop-opacity:0" />
        </radialGradient>
      </defs>
      <rect width="100%" height="100%" fill="url(#cascadeGrid)"/>

      <!-- Central glow -->
      <circle cx={centerX} cy={centerY} r={100} fill="url(#collapseGlow)"/>

      <!-- Dimensional boundary circle -->
      <circle
        cx={centerX}
        cy={centerY}
        r={120 * (effectiveDimension / 5)}
        fill="none"
        stroke={currentStage.color}
        stroke-width="2"
        stroke-dasharray="5 5"
        opacity="0.5"
      />

      <!-- Dimensional axes -->
      {#each axes as axis}
        <line
          x1={axis.x1}
          y1={axis.y1}
          x2={axis.x2}
          y2={axis.y2}
          stroke={currentStage.color}
          stroke-width="2"
          opacity={axis.opacity * 0.7}
        />
        <circle
          cx={axis.x2}
          cy={axis.y2}
          r="4"
          fill={currentStage.color}
          opacity={axis.opacity}
        />
      {/each}

      <!-- State particles -->
      {#each particles as particle}
        <circle
          cx={particle.x}
          cy={particle.y}
          r={particle.size}
          fill={currentStage.color}
          opacity="0.6"
        />
      {/each}

      <!-- Center point -->
      <circle cx={centerX} cy={centerY} r="6" fill={currentStage.color}/>

      <!-- Dimension label -->
      <text
        x={centerX}
        y={centerY + 4}
        text-anchor="middle"
        fill="white"
        font-size="12"
        font-weight="bold"
      >
        H{Math.round(effectiveDimension)}
      </text>
    </svg>

    <!-- Stage indicator -->
    <div class="absolute top-3 left-3 text-white">
      <div class="text-lg font-bold" style="color: {currentStage.color}">
        {currentStage.label}
      </div>
      <div class="text-xs text-slate-400">
        Dimension: {effectiveDimension.toFixed(2)}
      </div>
    </div>

    <!-- Eigenvalue bars -->
    <div class="absolute bottom-3 right-3 flex items-end gap-1">
      {#each eigenBars as bar, i}
        <div
          class="w-4 rounded-t transition-all duration-300"
          style="
            height: {bar.height}px;
            background: {currentStage.color};
            opacity: {bar.visible ? 0.8 : 0.2};
          "
          title="λ{i + 1}: {bar.value.toFixed(3)}"
        ></div>
      {/each}
      <span class="text-xs text-slate-500 ml-1">λ</span>
    </div>
  </div>

  <!-- Controls -->
  <div class="flex items-center justify-center gap-2 mt-3">
    <button
      on:click={reset}
      class="p-2 bg-surface-700 rounded hover:bg-surface-600 transition-colors"
      title="Reset"
    >
      <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
      </svg>
    </button>
    <button
      on:click={stepBackward}
      class="p-2 bg-surface-700 rounded hover:bg-surface-600 transition-colors"
      title="Previous stage"
    >
      <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
    </button>
    <button
      on:click={() => isPlaying ? pause() : play()}
      class="p-3 rounded-full transition-colors"
      style="background: {currentStage.color}"
      title={isPlaying ? 'Pause' : 'Play'}
    >
      {#if isPlaying}
        <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24">
          <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z"/>
        </svg>
      {:else}
        <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24">
          <path d="M8 5v14l11-7z"/>
        </svg>
      {/if}
    </button>
    <button
      on:click={stepForward}
      class="p-2 bg-surface-700 rounded hover:bg-surface-600 transition-colors"
      title="Next stage"
    >
      <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>
  </div>

  <!-- Stage progress -->
  <div class="flex gap-1 mt-3">
    {#each cascadeStages as stage, i}
      <div
        class="flex-1 h-1 rounded-full transition-colors"
        style="background: {i < stageIndex ? stage.color : i === stageIndex ? `linear-gradient(90deg, ${stage.color} ${stageProgress * 100}%, rgba(255,255,255,0.2) ${stageProgress * 100}%)` : 'rgba(255,255,255,0.1)'}"
      ></div>
    {/each}
  </div>

  <!-- Stage labels -->
  <div class="flex justify-between mt-1 text-xs text-slate-500">
    {#each cascadeStages as stage}
      <span>{stage.label}</span>
    {/each}
  </div>
</div>

<style>
  .cascade-player {
    background: rgba(15, 23, 42, 0.5);
    padding: 12px;
    border-radius: 12px;
  }
</style>
