<script lang="ts">
  /**
   * TopologyPlot - Visualizes topological invariants and phase space
   *
   * Shows topology-related metrics:
   * - Chern number
   * - Berry phase
   * - Winding number
   * - Phase space trajectory
   */

  export let chernNumber: number = 0;
  export let berryPhase: number = 0;
  export let windingNumber: number = 0;
  export let phaseTrajectory: Array<{ x: number; y: number }> = [];
  export let width: number = 300;
  export let height: number = 200;

  $: normalizedBerry = (berryPhase % (2 * Math.PI)) / (2 * Math.PI);
  $: trajectoryPath = phaseTrajectory.length > 1
    ? phaseTrajectory.map((p, i) =>
        `${i === 0 ? 'M' : 'L'} ${30 + p.x * (width - 60)} ${height - 30 - p.y * (height - 60)}`
      ).join(' ')
    : '';
</script>

<div class="topology-plot" style="width: {width}px;">
  <!-- Phase space plot -->
  <div class="bg-surface-900 rounded-lg overflow-hidden" style="height: {height}px;">
    <svg {width} {height}>
      <!-- Grid -->
      <defs>
        <pattern id="topoGrid" width="20" height="20" patternUnits="userSpaceOnUse">
          <path d="M 20 0 L 0 0 0 20" fill="none" stroke="rgba(255,255,255,0.05)" stroke-width="0.5"/>
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill="url(#topoGrid)"/>

      <!-- Axes -->
      <line x1="30" y1={height - 30} x2={width - 30} y2={height - 30} stroke="rgba(255,255,255,0.3)" stroke-width="1"/>
      <line x1="30" y1="30" x2="30" y2={height - 30} stroke="rgba(255,255,255,0.3)" stroke-width="1"/>

      <!-- Axis labels -->
      <text x={width / 2} y={height - 8} text-anchor="middle" fill="rgba(255,255,255,0.5)" font-size="10">Phase</text>
      <text x="10" y={height / 2} text-anchor="middle" fill="rgba(255,255,255,0.5)" font-size="10" transform="rotate(-90, 10, {height / 2})">Amplitude</text>

      <!-- Phase trajectory -->
      {#if trajectoryPath}
        <path
          d={trajectoryPath}
          fill="none"
          stroke="#8b5cf6"
          stroke-width="2"
          opacity="0.8"
        />
        <!-- Trajectory points -->
        {#each phaseTrajectory as point, i}
          <circle
            cx={30 + point.x * (width - 60)}
            cy={height - 30 - point.y * (height - 60)}
            r={i === phaseTrajectory.length - 1 ? 4 : 2}
            fill="#8b5cf6"
            opacity={0.3 + 0.7 * (i / phaseTrajectory.length)}
          />
        {/each}
      {:else}
        <text
          x={width / 2}
          y={height / 2}
          text-anchor="middle"
          fill="rgba(255,255,255,0.3)"
          font-size="12"
        >
          No trajectory data
        </text>
      {/if}

      <!-- Winding visualization -->
      {#if windingNumber !== 0}
        <g transform="translate({width - 50}, 50)">
          <circle r="20" fill="none" stroke="#f59e0b" stroke-width="2" stroke-dasharray="3 2"/>
          <text y="4" text-anchor="middle" fill="#f59e0b" font-size="14" font-weight="bold">
            {windingNumber > 0 ? '+' : ''}{windingNumber}
          </text>
        </g>
      {/if}
    </svg>
  </div>

  <!-- Metrics display -->
  <div class="grid grid-cols-3 gap-2 mt-3">
    <div class="bg-surface-800 rounded-lg p-2 text-center">
      <div class="text-lg font-bold text-amber-400">{chernNumber.toFixed(2)}</div>
      <div class="text-xs text-slate-400">Chern</div>
    </div>
    <div class="bg-surface-800 rounded-lg p-2 text-center">
      <div class="text-lg font-bold text-violet-400">{normalizedBerry.toFixed(2)}×2π</div>
      <div class="text-xs text-slate-400">Berry</div>
    </div>
    <div class="bg-surface-800 rounded-lg p-2 text-center">
      <div class="text-lg font-bold text-emerald-400">{windingNumber}</div>
      <div class="text-xs text-slate-400">Winding</div>
    </div>
  </div>
</div>
