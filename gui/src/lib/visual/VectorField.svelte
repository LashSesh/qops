<script lang="ts">
  /**
   * VectorField - Visualizes 2D vector field for gradient flows
   *
   * Shows:
   * - Arrow field representing gradients
   * - Color-coded magnitude
   * - Flow lines
   */

  export let vectors: Array<{ x: number; y: number; vx: number; vy: number }> = [];
  export let width: number = 300;
  export let height: number = 200;
  export let gridSize: number = 20;
  export let title: string = 'Gradient Field';
  export let colorByMagnitude: boolean = true;

  $: cols = Math.floor(width / gridSize);
  $: rows = Math.floor(height / gridSize);

  // Generate grid-based vectors if none provided
  $: displayVectors = vectors.length > 0 ? vectors : generateDefaultField();

  function generateDefaultField() {
    const field: Array<{ x: number; y: number; vx: number; vy: number }> = [];
    for (let row = 0; row < rows; row++) {
      for (let col = 0; col < cols; col++) {
        const x = (col + 0.5) * gridSize;
        const y = (row + 0.5) * gridSize;
        // Default: circular flow around center
        const dx = x - width / 2;
        const dy = y - height / 2;
        const dist = Math.sqrt(dx * dx + dy * dy) || 1;
        field.push({
          x,
          y,
          vx: -dy / dist * 0.5,
          vy: dx / dist * 0.5,
        });
      }
    }
    return field;
  }

  function getArrowColor(vx: number, vy: number): string {
    if (!colorByMagnitude) return '#8b5cf6';

    const magnitude = Math.sqrt(vx * vx + vy * vy);
    if (magnitude > 0.7) return '#10b981';
    if (magnitude > 0.4) return '#f59e0b';
    return '#3b82f6';
  }

  function getArrowPath(x: number, y: number, vx: number, vy: number, scale: number = 15): string {
    const magnitude = Math.sqrt(vx * vx + vy * vy);
    if (magnitude < 0.01) return '';

    const nx = vx / magnitude;
    const ny = vy / magnitude;
    const len = magnitude * scale;

    const endX = x + nx * len;
    const endY = y + ny * len;

    // Arrow head
    const headLen = 4;
    const headAngle = Math.PI / 6;
    const angle = Math.atan2(ny, nx);

    const h1x = endX - headLen * Math.cos(angle - headAngle);
    const h1y = endY - headLen * Math.sin(angle - headAngle);
    const h2x = endX - headLen * Math.cos(angle + headAngle);
    const h2y = endY - headLen * Math.sin(angle + headAngle);

    return `M ${x} ${y} L ${endX} ${endY} M ${endX} ${endY} L ${h1x} ${h1y} M ${endX} ${endY} L ${h2x} ${h2y}`;
  }
</script>

<div class="vector-field" style="width: {width}px;">
  <div class="flex items-center justify-between mb-2">
    <h3 class="text-sm font-medium text-white">{title}</h3>
    <span class="text-xs text-slate-400">{displayVectors.length} vectors</span>
  </div>

  <div class="bg-surface-900 rounded-lg overflow-hidden" style="height: {height}px;">
    <svg {width} {height}>
      <!-- Background -->
      <rect width="100%" height="100%" fill="#0f172a"/>

      <!-- Grid lines -->
      {#each Array(cols) as _, i}
        <line
          x1={(i + 1) * gridSize}
          y1="0"
          x2={(i + 1) * gridSize}
          y2={height}
          stroke="rgba(255,255,255,0.03)"
          stroke-width="1"
        />
      {/each}
      {#each Array(rows) as _, i}
        <line
          x1="0"
          y1={(i + 1) * gridSize}
          x2={width}
          y2={(i + 1) * gridSize}
          stroke="rgba(255,255,255,0.03)"
          stroke-width="1"
        />
      {/each}

      <!-- Vector arrows -->
      {#each displayVectors as vec}
        <path
          d={getArrowPath(vec.x, vec.y, vec.vx, vec.vy)}
          fill="none"
          stroke={getArrowColor(vec.vx, vec.vy)}
          stroke-width="1.5"
          stroke-linecap="round"
          opacity="0.8"
        />
      {/each}

      <!-- Center marker -->
      <circle cx={width / 2} cy={height / 2} r="3" fill="rgba(255,255,255,0.3)"/>
    </svg>
  </div>

  <!-- Legend -->
  {#if colorByMagnitude}
    <div class="flex items-center justify-center gap-4 mt-2 text-xs">
      <div class="flex items-center gap-1">
        <div class="w-4 h-0.5 bg-blue-500"></div>
        <span class="text-slate-400">Low</span>
      </div>
      <div class="flex items-center gap-1">
        <div class="w-4 h-0.5 bg-amber-500"></div>
        <span class="text-slate-400">Medium</span>
      </div>
      <div class="flex items-center gap-1">
        <div class="w-4 h-0.5 bg-emerald-500"></div>
        <span class="text-slate-400">High</span>
      </div>
    </div>
  {/if}
</div>
