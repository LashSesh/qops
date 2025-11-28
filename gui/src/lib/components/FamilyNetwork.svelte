<script lang="ts">
  /**
   * FamilyNetwork - Visualizes operator family relationships
   *
   * Shows families as nodes with:
   * - Size based on member count
   * - Color based on characteristics
   * - Connections based on resonance similarity
   */

  interface FamilyCharacteristics {
    is_high_quality: boolean;
    is_stable: boolean;
    is_efficient: boolean;
  }

  interface Family {
    name: string;
    member_count: number;
    avg_resonance: number;
    characteristics: FamilyCharacteristics;
  }

  export let families: Family[] = [];
  export let width = 400;
  export let height = 300;
  export let title = 'Operator Family Network';
  export let showConnections = true;

  // Layout calculation
  $: cx = width / 2;
  $: cy = height / 2;

  // Calculate node positions using force-directed-like layout
  $: nodes = families.map((family, idx) => {
    const angle = (idx / families.length) * 2 * Math.PI;
    const radiusFactor = 0.6 + (family.avg_resonance * 0.3);
    const radius = Math.min(width, height) * 0.35 * radiusFactor;

    return {
      ...family,
      x: cx + radius * Math.cos(angle),
      y: cy + radius * Math.sin(angle),
      size: 15 + Math.sqrt(family.member_count) * 5,
    };
  });

  // Calculate connections based on resonance similarity
  $: connections = showConnections ? calculateConnections(nodes) : [];

  function calculateConnections(
    nodeList: Array<Family & { x: number; y: number; size: number }>
  ): Array<{ from: number; to: number; strength: number }> {
    const conn: Array<{ from: number; to: number; strength: number }> = [];
    const threshold = 0.1; // Connect families within 0.1 resonance difference

    for (let i = 0; i < nodeList.length; i++) {
      for (let j = i + 1; j < nodeList.length; j++) {
        const diff = Math.abs(nodeList[i].avg_resonance - nodeList[j].avg_resonance);
        if (diff < threshold) {
          conn.push({
            from: i,
            to: j,
            strength: 1 - (diff / threshold),
          });
        }
      }
    }
    return conn;
  }

  function getNodeColor(family: Family): string {
    const { is_high_quality, is_stable, is_efficient } = family.characteristics;

    // Combine characteristics into a color
    if (is_high_quality && is_stable && is_efficient) {
      return 'rgb(34, 197, 94)'; // Emerald - all three
    } else if (is_high_quality && is_stable) {
      return 'rgb(245, 158, 11)'; // Amber - quality + stable
    } else if (is_high_quality) {
      return 'rgb(139, 92, 246)'; // Violet - quality only
    } else if (is_stable) {
      return 'rgb(59, 130, 246)'; // Blue - stable only
    } else {
      return 'rgb(100, 116, 139)'; // Slate - basic
    }
  }

  function getNodeGlow(family: Family): string {
    const color = getNodeColor(family);
    return color.replace('rgb', 'rgba').replace(')', ', 0.4)');
  }

  let hoveredNode: number | null = null;
</script>

<div class="family-network">
  <div class="flex items-center justify-between mb-2">
    <h3 class="text-sm font-medium text-white">{title}</h3>
    {#if families.length > 0}
      <span class="text-xs text-slate-400">{families.length} families</span>
    {/if}
  </div>

  <div class="relative bg-surface-900 rounded overflow-hidden" style="width: {width}px; height: {height}px;">
    {#if families.length > 0}
      <svg {width} {height}>
        <!-- Connections -->
        {#each connections as conn}
          <line
            x1={nodes[conn.from].x}
            y1={nodes[conn.from].y}
            x2={nodes[conn.to].x}
            y2={nodes[conn.to].y}
            stroke="rgb(71, 85, 105)"
            stroke-width={conn.strength * 2}
            opacity={0.3 + conn.strength * 0.3}
          />
        {/each}

        <!-- Nodes -->
        {#each nodes as node, idx}
          <!-- Glow effect for hovered node -->
          {#if hoveredNode === idx}
            <circle
              cx={node.x}
              cy={node.y}
              r={node.size + 8}
              fill={getNodeGlow(node)}
              class="transition-all"
            />
          {/if}

          <!-- Node circle -->
          <circle
            cx={node.x}
            cy={node.y}
            r={node.size}
            fill={getNodeColor(node)}
            stroke="rgb(30, 41, 59)"
            stroke-width="2"
            class="cursor-pointer transition-transform"
            on:mouseenter={() => hoveredNode = idx}
            on:mouseleave={() => hoveredNode = null}
          >
            <title>
              {node.name}
              Members: {node.member_count}
              Resonance: {node.avg_resonance.toFixed(4)}
            </title>
          </circle>

          <!-- Node label -->
          <text
            x={node.x}
            y={node.y + node.size + 12}
            text-anchor="middle"
            fill={hoveredNode === idx ? 'white' : 'rgb(148, 163, 184)'}
            font-size="10"
            class="pointer-events-none"
          >
            {node.name.length > 12 ? node.name.slice(0, 10) + '...' : node.name}
          </text>

          <!-- Member count inside node -->
          <text
            x={node.x}
            y={node.y + 4}
            text-anchor="middle"
            fill="white"
            font-size="11"
            font-weight="bold"
            class="pointer-events-none"
          >
            {node.member_count}
          </text>
        {/each}

        <!-- Center point (represents topology center) -->
        <circle
          cx={cx}
          cy={cy}
          r="6"
          fill="rgb(51, 65, 85)"
          stroke="rgb(71, 85, 105)"
          stroke-width="1"
        />
      </svg>

      <!-- Hovered node details -->
      {#if hoveredNode !== null && nodes[hoveredNode]}
        {@const node = nodes[hoveredNode]}
        <div class="absolute top-2 left-2 bg-surface-800/95 px-3 py-2 rounded text-sm max-w-48">
          <div class="font-medium text-white truncate">{node.name}</div>
          <div class="grid grid-cols-2 gap-x-4 mt-1 text-xs">
            <span class="text-slate-400">Members:</span>
            <span class="text-white">{node.member_count}</span>
            <span class="text-slate-400">Resonance:</span>
            <span class="text-amber-400">{node.avg_resonance.toFixed(4)}</span>
          </div>
          <div class="flex gap-2 mt-2">
            {#if node.characteristics.is_high_quality}
              <span class="text-xs bg-violet-500/20 text-violet-400 px-1.5 py-0.5 rounded">Quality</span>
            {/if}
            {#if node.characteristics.is_stable}
              <span class="text-xs bg-blue-500/20 text-blue-400 px-1.5 py-0.5 rounded">Stable</span>
            {/if}
            {#if node.characteristics.is_efficient}
              <span class="text-xs bg-emerald-500/20 text-emerald-400 px-1.5 py-0.5 rounded">Efficient</span>
            {/if}
          </div>
        </div>
      {/if}
    {:else}
      <div class="flex items-center justify-center h-full text-slate-500 text-sm">
        No family data
      </div>
    {/if}
  </div>

  <!-- Legend -->
  <div class="mt-2 flex flex-wrap gap-3 text-xs">
    <div class="flex items-center gap-1">
      <div class="w-3 h-3 rounded-full bg-emerald-500"></div>
      <span class="text-slate-400">All traits</span>
    </div>
    <div class="flex items-center gap-1">
      <div class="w-3 h-3 rounded-full bg-amber-500"></div>
      <span class="text-slate-400">Quality+Stable</span>
    </div>
    <div class="flex items-center gap-1">
      <div class="w-3 h-3 rounded-full bg-violet-500"></div>
      <span class="text-slate-400">Quality</span>
    </div>
    <div class="flex items-center gap-1">
      <div class="w-3 h-3 rounded-full bg-blue-500"></div>
      <span class="text-slate-400">Stable</span>
    </div>
    <div class="flex items-center gap-1">
      <div class="w-3 h-3 rounded-full bg-slate-500"></div>
      <span class="text-slate-400">Basic</span>
    </div>
  </div>
</div>

<style>
  .cursor-pointer {
    cursor: pointer;
  }

  .transition-transform {
    transition: transform 0.2s ease;
  }

  circle.cursor-pointer:hover {
    transform-origin: center;
    transform: scale(1.1);
  }
</style>
