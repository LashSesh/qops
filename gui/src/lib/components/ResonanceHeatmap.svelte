<script lang="ts">
  /**
   * ResonanceHeatmap - Visualizes resonance values as a color-coded heatmap
   *
   * Colors:
   * - Blue: Low resonance (Kosmokrator filtered)
   * - Violet: Medium resonance (Chronokrator expanded)
   * - Gold/Amber: High resonance (Pfauenthron candidates)
   * - Green: Finalized (Monolith members)
   */

  export let data: number[] = [];
  export let width = 300;
  export let height = 200;
  export let cellSize = 10;
  export let title = 'Resonance Distribution';
  export let showLegend = true;

  // Calculate grid dimensions
  $: cols = Math.floor(width / cellSize);
  $: rows = Math.floor(height / cellSize);

  // Normalize data to grid
  $: gridData = createGridData(data, cols, rows);

  function createGridData(values: number[], c: number, r: number): number[][] {
    const grid: number[][] = [];
    const totalCells = c * r;

    for (let row = 0; row < r; row++) {
      const rowData: number[] = [];
      for (let col = 0; col < c; col++) {
        const idx = row * c + col;
        if (idx < values.length) {
          rowData.push(values[idx]);
        } else if (values.length > 0) {
          // Wrap around or interpolate
          rowData.push(values[idx % values.length]);
        } else {
          rowData.push(0);
        }
      }
      grid.push(rowData);
    }
    return grid;
  }

  function getColor(value: number): string {
    // Clamp between 0 and 1
    const v = Math.max(0, Math.min(1, value));

    if (v < 0.3) {
      // Blue range (Kosmokrator)
      const intensity = v / 0.3;
      return `rgb(${Math.floor(30 + intensity * 30)}, ${Math.floor(50 + intensity * 80)}, ${Math.floor(150 + intensity * 105)})`;
    } else if (v < 0.6) {
      // Violet range (Chronokrator)
      const intensity = (v - 0.3) / 0.3;
      return `rgb(${Math.floor(100 + intensity * 60)}, ${Math.floor(50 + intensity * 30)}, ${Math.floor(200 + intensity * 55)})`;
    } else if (v < 0.85) {
      // Amber range (Pfauenthron)
      const intensity = (v - 0.6) / 0.25;
      return `rgb(${Math.floor(200 + intensity * 45)}, ${Math.floor(150 + intensity * 30)}, ${Math.floor(50 - intensity * 30)})`;
    } else {
      // Green range (Finalized)
      const intensity = (v - 0.85) / 0.15;
      return `rgb(${Math.floor(50 + intensity * 20)}, ${Math.floor(180 + intensity * 60)}, ${Math.floor(80 + intensity * 40)})`;
    }
  }

  function getStats() {
    if (data.length === 0) return { min: 0, max: 0, avg: 0 };
    const min = Math.min(...data);
    const max = Math.max(...data);
    const avg = data.reduce((a, b) => a + b, 0) / data.length;
    return { min, max, avg };
  }

  $: stats = getStats();
</script>

<div class="resonance-heatmap">
  <div class="flex items-center justify-between mb-2">
    <h3 class="text-sm font-medium text-white">{title}</h3>
    {#if data.length > 0}
      <span class="text-xs text-slate-400">{data.length} values</span>
    {/if}
  </div>

  <div class="relative bg-surface-900 rounded overflow-hidden" style="width: {width}px; height: {height}px;">
    {#if data.length > 0}
      <svg {width} {height}>
        {#each gridData as row, rowIdx}
          {#each row as value, colIdx}
            <rect
              x={colIdx * cellSize}
              y={rowIdx * cellSize}
              width={cellSize - 1}
              height={cellSize - 1}
              fill={getColor(value)}
              rx="1"
            >
              <title>{value.toFixed(4)}</title>
            </rect>
          {/each}
        {/each}
      </svg>
    {:else}
      <div class="flex items-center justify-center h-full text-slate-500 text-sm">
        No data
      </div>
    {/if}
  </div>

  {#if showLegend && data.length > 0}
    <div class="mt-3">
      <!-- Gradient legend -->
      <div class="h-3 rounded overflow-hidden flex">
        <div class="flex-1 bg-gradient-to-r from-blue-600 to-blue-400"></div>
        <div class="flex-1 bg-gradient-to-r from-violet-500 to-violet-400"></div>
        <div class="flex-1 bg-gradient-to-r from-amber-500 to-amber-400"></div>
        <div class="flex-1 bg-gradient-to-r from-emerald-500 to-emerald-400"></div>
      </div>
      <div class="flex justify-between text-xs text-slate-400 mt-1">
        <span>0.0</span>
        <span>0.3</span>
        <span>0.6</span>
        <span>0.85</span>
        <span>1.0</span>
      </div>

      <!-- Stats -->
      <div class="flex gap-4 mt-2 text-xs">
        <div>
          <span class="text-slate-400">Min:</span>
          <span class="text-white ml-1">{stats.min.toFixed(4)}</span>
        </div>
        <div>
          <span class="text-slate-400">Avg:</span>
          <span class="text-white ml-1">{stats.avg.toFixed(4)}</span>
        </div>
        <div>
          <span class="text-slate-400">Max:</span>
          <span class="text-white ml-1">{stats.max.toFixed(4)}</span>
        </div>
      </div>
    </div>
  {/if}
</div>
