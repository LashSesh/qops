<script lang="ts">
  /**
   * HypercubeCanvas - Visualizes a 5D hypercube projected to 2D
   *
   * Displays vertices as nodes with resonance-based coloring,
   * edges as connections, and supports interactive exploration.
   */

  import type { Coord5DDto, HypercubeVertexDto } from '$lib/tauri/commands';

  export let vertices: HypercubeVertexDto[] = [];
  export let width = 500;
  export let height = 400;
  export let selectedVertex: HypercubeVertexDto | null = null;
  export let showLabels = false;
  export let animationEnabled = true;

  // Animation state
  let animationAngle = 0;
  let animationFrame: number;

  $: if (animationEnabled) {
    startAnimation();
  } else {
    stopAnimation();
  }

  function startAnimation() {
    function animate() {
      animationAngle += 0.01;
      animationFrame = requestAnimationFrame(animate);
    }
    animate();
  }

  function stopAnimation() {
    if (animationFrame) {
      cancelAnimationFrame(animationFrame);
    }
  }

  // Project 5D coordinate to 2D for visualization
  function project5Dto2D(coord: Coord5DDto, angle: number): { x: number; y: number } {
    // Use psi, rho, omega for primary position, chi and eta for modulation
    const baseX = coord.psi * 0.4 + coord.rho * 0.3 + coord.chi * 0.2;
    const baseY = coord.omega * 0.4 + coord.rho * 0.3 + coord.eta * 0.2;

    // Apply rotation based on animation angle
    const rotatedX = baseX * Math.cos(angle) - baseY * Math.sin(angle) * 0.3;
    const rotatedY = baseY * Math.cos(angle * 0.7) + baseX * Math.sin(angle) * 0.3;

    // Map to canvas coordinates
    const x = (rotatedX + 1) / 2 * (width - 80) + 40;
    const y = (rotatedY + 1) / 2 * (height - 80) + 40;

    return { x, y };
  }

  // Get color based on resonance
  function getResonanceColor(resonance: number): string {
    if (resonance >= 0.85) {
      return '#10b981'; // Emerald - high resonance
    } else if (resonance >= 0.6) {
      return '#f59e0b'; // Amber - medium-high
    } else if (resonance >= 0.3) {
      return '#8b5cf6'; // Violet - medium
    } else {
      return '#3b82f6'; // Blue - low
    }
  }

  // Get node size based on depth
  function getNodeSize(depth: number): number {
    return Math.max(4, 12 - depth * 1.5);
  }

  function handleVertexClick(vertex: HypercubeVertexDto) {
    selectedVertex = vertex;
  }

  $: projectedVertices = vertices.map(v => ({
    ...v,
    projected: project5Dto2D(v.coordinate, animationAngle)
  }));
</script>

<div class="hypercube-canvas">
  <div class="bg-surface-900 rounded-lg overflow-hidden" style="width: {width}px; height: {height}px;">
    <svg {width} {height}>
      <!-- Background grid -->
      <defs>
        <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
          <path d="M 40 0 L 0 0 0 40" fill="none" stroke="rgba(255,255,255,0.05)" stroke-width="0.5"/>
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill="url(#grid)"/>

      <!-- Edges (simplified - connect nearby vertices) -->
      {#each projectedVertices as v1, i}
        {#each projectedVertices.slice(i + 1) as v2}
          {#if Math.abs(v1.depth - v2.depth) <= 1}
            <line
              x1={v1.projected.x}
              y1={v1.projected.y}
              x2={v2.projected.x}
              y2={v2.projected.y}
              stroke="rgba(255,255,255,0.1)"
              stroke-width="0.5"
            />
          {/if}
        {/each}
      {/each}

      <!-- Vertices -->
      {#each projectedVertices as vertex}
        <g
          transform="translate({vertex.projected.x}, {vertex.projected.y})"
          class="cursor-pointer"
          on:click={() => handleVertexClick(vertex)}
          on:keydown={(e) => e.key === 'Enter' && handleVertexClick(vertex)}
          role="button"
          tabindex="0"
        >
          <circle
            r={getNodeSize(vertex.depth)}
            fill={getResonanceColor(vertex.resonance)}
            stroke={selectedVertex?.id === vertex.id ? '#fff' : 'transparent'}
            stroke-width="2"
            opacity={0.8}
          />
          {#if showLabels}
            <text
              y={getNodeSize(vertex.depth) + 12}
              text-anchor="middle"
              fill="rgba(255,255,255,0.6)"
              font-size="8"
            >
              {vertex.resonance.toFixed(2)}
            </text>
          {/if}
        </g>
      {/each}

      <!-- Center indicator -->
      <circle cx={width / 2} cy={height / 2} r="2" fill="rgba(255,255,255,0.3)"/>
    </svg>
  </div>

  <!-- Legend -->
  <div class="flex items-center justify-between mt-3 text-xs text-slate-400">
    <div class="flex items-center gap-4">
      <div class="flex items-center gap-1">
        <div class="w-3 h-3 rounded-full bg-blue-500"></div>
        <span>&lt;0.3</span>
      </div>
      <div class="flex items-center gap-1">
        <div class="w-3 h-3 rounded-full bg-violet-500"></div>
        <span>0.3-0.6</span>
      </div>
      <div class="flex items-center gap-1">
        <div class="w-3 h-3 rounded-full bg-amber-500"></div>
        <span>0.6-0.85</span>
      </div>
      <div class="flex items-center gap-1">
        <div class="w-3 h-3 rounded-full bg-emerald-500"></div>
        <span>&gt;0.85</span>
      </div>
    </div>
    <span>{vertices.length} vertices</span>
  </div>

  <!-- Selected vertex info -->
  {#if selectedVertex}
    <div class="mt-3 p-3 bg-surface-800 rounded-lg text-sm">
      <div class="text-white font-medium mb-2">Selected Vertex</div>
      <div class="grid grid-cols-2 gap-2 text-xs">
        <div>
          <span class="text-slate-400">ID:</span>
          <span class="text-white ml-1">{selectedVertex.id.substring(0, 8)}...</span>
        </div>
        <div>
          <span class="text-slate-400">Depth:</span>
          <span class="text-white ml-1">{selectedVertex.depth}</span>
        </div>
        <div>
          <span class="text-slate-400">Resonance:</span>
          <span class="text-white ml-1">{selectedVertex.resonance.toFixed(4)}</span>
        </div>
        <div>
          <span class="text-slate-400">Coord:</span>
          <span class="text-white ml-1">
            ({selectedVertex.coordinate.psi.toFixed(2)},
             {selectedVertex.coordinate.rho.toFixed(2)},
             {selectedVertex.coordinate.omega.toFixed(2)})
          </span>
        </div>
      </div>
    </div>
  {/if}
</div>
