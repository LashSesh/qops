<script lang="ts">
  /**
   * Coord5DDisplay - Visualizes a 5D coordinate as a radar chart
   *
   * Shows psi, rho, omega, chi, eta dimensions with resonance calculation.
   */

  import type { Coord5DDto } from '$lib/tauri/commands';

  export let coordinate: Coord5DDto;
  export let size = 200;
  export let showLabels = true;
  export let showResonance = true;
  export let title = '';

  // Dimensions and their properties
  const dimensions = [
    { key: 'psi', label: 'psi', color: '#3b82f6', weight: 0.4 },
    { key: 'rho', label: 'rho', color: '#8b5cf6', weight: 0.3 },
    { key: 'omega', label: 'omega', color: '#f59e0b', weight: 0.3 },
    { key: 'chi', label: 'chi', color: '#06b6d4', weight: 0.05 },
    { key: 'eta', label: 'eta', color: '#ef4444', weight: -0.05 },
  ];

  const center = size / 2;
  const radius = (size - 60) / 2;
  const angleStep = (2 * Math.PI) / dimensions.length;

  // Calculate resonance
  function calculateResonance(coord: Coord5DDto): number {
    return 0.4 * coord.psi + 0.3 * coord.rho + 0.3 * coord.omega + 0.05 * coord.chi - 0.05 * coord.eta;
  }

  // Get point position for a dimension value
  function getPoint(dimIndex: number, value: number): { x: number; y: number } {
    const angle = dimIndex * angleStep - Math.PI / 2; // Start from top
    const r = value * radius;
    return {
      x: center + r * Math.cos(angle),
      y: center + r * Math.sin(angle)
    };
  }

  // Create polygon path for the coordinate
  function createPath(coord: Coord5DDto): string {
    const values = [coord.psi, coord.rho, coord.omega, coord.chi, coord.eta];
    const points = values.map((v, i) => getPoint(i, Math.max(0, Math.min(1, v))));
    return points.map(p => `${p.x},${p.y}`).join(' ');
  }

  // Get axis endpoint
  function getAxisEnd(dimIndex: number): { x: number; y: number } {
    const angle = dimIndex * angleStep - Math.PI / 2;
    return {
      x: center + radius * Math.cos(angle),
      y: center + radius * Math.sin(angle)
    };
  }

  // Get label position (slightly outside the axis)
  function getLabelPosition(dimIndex: number): { x: number; y: number } {
    const angle = dimIndex * angleStep - Math.PI / 2;
    const r = radius + 20;
    return {
      x: center + r * Math.cos(angle),
      y: center + r * Math.sin(angle)
    };
  }

  $: resonance = calculateResonance(coordinate);
  $: polygonPath = createPath(coordinate);
</script>

<div class="coord5d-display">
  {#if title}
    <div class="text-sm font-medium text-white mb-2">{title}</div>
  {/if}

  <div class="bg-surface-900 rounded-lg p-2" style="width: {size}px; height: {size}px;">
    <svg width={size} height={size}>
      <!-- Background circles (grid) -->
      {#each [0.2, 0.4, 0.6, 0.8, 1.0] as level}
        <circle
          cx={center}
          cy={center}
          r={level * radius}
          fill="none"
          stroke="rgba(255,255,255,0.1)"
          stroke-width="1"
        />
      {/each}

      <!-- Axes -->
      {#each dimensions as dim, i}
        {@const end = getAxisEnd(i)}
        <line
          x1={center}
          y1={center}
          x2={end.x}
          y2={end.y}
          stroke="rgba(255,255,255,0.2)"
          stroke-width="1"
        />
      {/each}

      <!-- Data polygon -->
      <polygon
        points={polygonPath}
        fill="rgba(59, 130, 246, 0.3)"
        stroke="#3b82f6"
        stroke-width="2"
      />

      <!-- Data points -->
      {#each dimensions as dim, i}
        {@const value = coordinate[dim.key as keyof Coord5DDto]}
        {@const point = getPoint(i, Math.max(0, Math.min(1, value)))}
        <circle
          cx={point.x}
          cy={point.y}
          r="4"
          fill={dim.color}
          stroke="white"
          stroke-width="1"
        />
      {/each}

      <!-- Labels -->
      {#if showLabels}
        {#each dimensions as dim, i}
          {@const labelPos = getLabelPosition(i)}
          {@const value = coordinate[dim.key as keyof Coord5DDto]}
          <text
            x={labelPos.x}
            y={labelPos.y}
            text-anchor="middle"
            dominant-baseline="middle"
            fill={dim.color}
            font-size="10"
            font-weight="bold"
          >
            {dim.label}
          </text>
          <text
            x={labelPos.x}
            y={labelPos.y + 12}
            text-anchor="middle"
            fill="rgba(255,255,255,0.6)"
            font-size="9"
          >
            {value.toFixed(2)}
          </text>
        {/each}
      {/if}

      <!-- Center dot -->
      <circle cx={center} cy={center} r="2" fill="rgba(255,255,255,0.5)"/>
    </svg>
  </div>

  <!-- Resonance display -->
  {#if showResonance}
    <div class="mt-2 text-center">
      <div class="text-xs text-slate-400">Resonance</div>
      <div class="text-lg font-bold" style="color: {resonance >= 0.8 ? '#10b981' : resonance >= 0.5 ? '#f59e0b' : '#ef4444'}">
        {resonance.toFixed(4)}
      </div>
    </div>
  {/if}

  <!-- Dimension breakdown -->
  <div class="mt-2 grid grid-cols-5 gap-1 text-xs">
    {#each dimensions as dim}
      {@const value = coordinate[dim.key as keyof Coord5DDto]}
      <div class="text-center">
        <div style="color: {dim.color}">{dim.label}</div>
        <div class="text-white">{value.toFixed(2)}</div>
      </div>
    {/each}
  </div>
</div>
