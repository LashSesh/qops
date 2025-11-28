<script lang="ts">
  /**
   * SpiralTrajectory - Visualizes TRITON spiral search trajectory
   *
   * Shows the spiral path with:
   * - Points colored by score
   * - Layer boundaries
   * - Best point highlighted
   * - Temperature/radius annotations
   */

  interface TrajectoryPoint {
    iteration: number;
    score: number;
    layer: number;
    temperature: number;
    radius: number;
  }

  export let trajectory: TrajectoryPoint[] = [];
  export let width = 400;
  export let height = 400;
  export let showLayers = true;
  export let showTemperature = true;
  export let title = 'TRITON Spiral Trajectory';

  // Calculate center
  $: cx = width / 2;
  $: cy = height / 2;
  $: maxRadius = Math.min(width, height) / 2 - 20;

  // Find best point
  $: bestPoint = trajectory.reduce(
    (best, p) => (p.score > best.score ? p : best),
    { score: -Infinity, iteration: 0, layer: 0, temperature: 0, radius: 0 }
  );

  // Calculate spiral points
  $: spiralPoints = trajectory.map((point, idx) => {
    // Golden ratio spiral angle
    const angle = idx * 2.39996; // Golden angle in radians
    const normalizedRadius = (point.radius / (Math.max(...trajectory.map(p => p.radius)) || 1)) * maxRadius * 0.9;
    const x = cx + normalizedRadius * Math.cos(angle);
    const y = cy + normalizedRadius * Math.sin(angle);
    return { ...point, x, y };
  });

  // Get unique layers for boundaries
  $: layers = [...new Set(trajectory.map(p => p.layer))].sort((a, b) => a - b);

  function getScoreColor(score: number): string {
    const maxScore = Math.max(...trajectory.map(p => p.score));
    const minScore = Math.min(...trajectory.map(p => p.score));
    const normalized = maxScore > minScore ? (score - minScore) / (maxScore - minScore) : 0.5;

    if (normalized < 0.33) {
      return `rgb(59, 130, 246)`; // Blue
    } else if (normalized < 0.66) {
      return `rgb(139, 92, 246)`; // Violet
    } else {
      return `rgb(245, 158, 11)`; // Amber
    }
  }

  function getLayerRadius(layer: number): number {
    const maxLayer = Math.max(...layers);
    return ((layer + 1) / (maxLayer + 1)) * maxRadius;
  }

  function getPathD(): string {
    if (spiralPoints.length < 2) return '';
    return spiralPoints
      .map((p, i) => (i === 0 ? `M ${p.x} ${p.y}` : `L ${p.x} ${p.y}`))
      .join(' ');
  }
</script>

<div class="spiral-trajectory">
  <div class="flex items-center justify-between mb-2">
    <h3 class="text-sm font-medium text-white">{title}</h3>
    {#if trajectory.length > 0}
      <span class="text-xs text-slate-400">{trajectory.length} iterations</span>
    {/if}
  </div>

  <div class="relative bg-surface-900 rounded overflow-hidden" style="width: {width}px; height: {height}px;">
    {#if trajectory.length > 0}
      <svg {width} {height}>
        <!-- Layer boundaries -->
        {#if showLayers}
          {#each layers as layer}
            <circle
              cx={cx}
              cy={cy}
              r={getLayerRadius(layer)}
              fill="none"
              stroke="rgb(51, 65, 85)"
              stroke-width="1"
              stroke-dasharray="4 4"
            />
            <text
              x={cx + getLayerRadius(layer) + 5}
              y={cy - 5}
              fill="rgb(100, 116, 139)"
              font-size="10"
            >
              L{layer}
            </text>
          {/each}
        {/if}

        <!-- Trajectory path -->
        <path
          d={getPathD()}
          fill="none"
          stroke="rgb(100, 116, 139)"
          stroke-width="1"
          opacity="0.5"
        />

        <!-- Trajectory points -->
        {#each spiralPoints as point, idx}
          <circle
            cx={point.x}
            cy={point.y}
            r={point.score === bestPoint.score ? 6 : 3}
            fill={getScoreColor(point.score)}
            opacity={0.7 + 0.3 * (idx / spiralPoints.length)}
          >
            <title>
              Iter: {point.iteration}
              Score: {point.score.toFixed(4)}
              Layer: {point.layer}
              Temp: {point.temperature.toFixed(3)}
            </title>
          </circle>
        {/each}

        <!-- Best point highlight -->
        {#if spiralPoints.length > 0}
          {@const best = spiralPoints.find(p => p.score === bestPoint.score)}
          {#if best}
            <circle
              cx={best.x}
              cy={best.y}
              r="10"
              fill="none"
              stroke="rgb(34, 197, 94)"
              stroke-width="2"
              class="animate-pulse"
            />
          {/if}
        {/if}

        <!-- Center marker -->
        <circle
          cx={cx}
          cy={cy}
          r="4"
          fill="rgb(99, 102, 241)"
        />
      </svg>

      <!-- Best score overlay -->
      <div class="absolute top-2 right-2 bg-surface-800/90 px-3 py-2 rounded text-sm">
        <div class="text-slate-400 text-xs">Best Score</div>
        <div class="text-emerald-400 font-mono font-bold">{bestPoint.score.toFixed(4)}</div>
      </div>
    {:else}
      <div class="flex items-center justify-center h-full text-slate-500 text-sm">
        No trajectory data
      </div>
    {/if}
  </div>

  {#if showTemperature && trajectory.length > 0}
    <!-- Temperature/Radius mini chart -->
    <div class="mt-3 h-16 bg-surface-700 rounded overflow-hidden relative">
      <svg width={width} height="64">
        <!-- Temperature line -->
        <path
          d={trajectory.map((p, i) => {
            const x = (i / trajectory.length) * width;
            const y = 64 - (p.temperature * 60);
            return i === 0 ? `M ${x} ${y}` : `L ${x} ${y}`;
          }).join(' ')}
          fill="none"
          stroke="rgb(239, 68, 68)"
          stroke-width="1.5"
          opacity="0.8"
        />
        <!-- Radius line -->
        <path
          d={trajectory.map((p, i) => {
            const maxR = Math.max(...trajectory.map(pt => pt.radius));
            const x = (i / trajectory.length) * width;
            const y = 64 - ((p.radius / maxR) * 60);
            return i === 0 ? `M ${x} ${y}` : `L ${x} ${y}`;
          }).join(' ')}
          fill="none"
          stroke="rgb(59, 130, 246)"
          stroke-width="1.5"
          opacity="0.8"
        />
      </svg>
      <div class="absolute bottom-1 left-2 flex gap-4 text-xs">
        <span class="flex items-center gap-1">
          <span class="w-3 h-0.5 bg-red-500"></span>
          <span class="text-slate-400">Temperature</span>
        </span>
        <span class="flex items-center gap-1">
          <span class="w-3 h-0.5 bg-blue-500"></span>
          <span class="text-slate-400">Radius</span>
        </span>
      </div>
    </div>
  {/if}

  <!-- Legend -->
  <div class="mt-2 flex gap-4 text-xs">
    <div class="flex items-center gap-1">
      <div class="w-3 h-3 rounded-full bg-blue-500"></div>
      <span class="text-slate-400">Low Score</span>
    </div>
    <div class="flex items-center gap-1">
      <div class="w-3 h-3 rounded-full bg-violet-500"></div>
      <span class="text-slate-400">Medium</span>
    </div>
    <div class="flex items-center gap-1">
      <div class="w-3 h-3 rounded-full bg-amber-500"></div>
      <span class="text-slate-400">High Score</span>
    </div>
    <div class="flex items-center gap-1">
      <div class="w-3 h-3 rounded-full border-2 border-emerald-500"></div>
      <span class="text-slate-400">Best</span>
    </div>
  </div>
</div>

<style>
  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  .animate-pulse {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
</style>
