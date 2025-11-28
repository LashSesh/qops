<script lang="ts">
  /**
   * HDAGGraph - Visualizes a Hierarchical Directed Acyclic Graph
   *
   * Displays nodes connected by edges with execution order highlighting.
   * Supports different node types: Input, Output, Operator, Merge, Compilation.
   */

  import type { HDAGInfoDto, HDAGNodeDto, HDAGEdgeDto } from '$lib/tauri/commands';

  export let hdagInfo: HDAGInfoDto | null = null;
  export let width = 600;
  export let height = 400;
  export let executingNode: string | null = null;
  export let completedNodes: string[] = [];

  // Node type colors
  const nodeColors: Record<string, string> = {
    'Input': '#3b82f6',      // Blue
    'Output': '#10b981',     // Emerald
    'Operator': '#8b5cf6',   // Violet
    'Merge': '#f59e0b',      // Amber
    'Compilation': '#ef4444', // Red
    'Transform': '#06b6d4',  // Cyan
  };

  // Calculate node positions using a layered layout
  function calculateLayout(nodes: HDAGNodeDto[], edges: HDAGEdgeDto[]): Map<string, { x: number; y: number }> {
    const positions = new Map<string, { x: number; y: number }>();

    if (nodes.length === 0) return positions;

    // Group nodes by type for layering
    const layers: HDAGNodeDto[][] = [];
    const inputNodes = nodes.filter(n => n.node_type.includes('Input'));
    const outputNodes = nodes.filter(n => n.node_type.includes('Output'));
    const compilationNodes = nodes.filter(n => n.node_type.includes('Compilation'));
    const mergeNodes = nodes.filter(n => n.node_type.includes('Merge'));
    const otherNodes = nodes.filter(n =>
      !n.node_type.includes('Input') &&
      !n.node_type.includes('Output') &&
      !n.node_type.includes('Compilation') &&
      !n.node_type.includes('Merge')
    );

    if (inputNodes.length > 0) layers.push(inputNodes);
    if (otherNodes.length > 0) layers.push(otherNodes);
    if (mergeNodes.length > 0) layers.push(mergeNodes);
    if (compilationNodes.length > 0) layers.push(compilationNodes);
    if (outputNodes.length > 0) layers.push(outputNodes);

    const layerCount = layers.length;
    const layerWidth = (width - 100) / (layerCount || 1);

    layers.forEach((layer, layerIdx) => {
      const layerHeight = (height - 80) / (layer.length || 1);
      layer.forEach((node, nodeIdx) => {
        positions.set(node.id, {
          x: 50 + layerIdx * layerWidth + layerWidth / 2,
          y: 40 + nodeIdx * layerHeight + layerHeight / 2
        });
      });
    });

    return positions;
  }

  function getNodeColor(nodeType: string): string {
    for (const [key, color] of Object.entries(nodeColors)) {
      if (nodeType.includes(key)) return color;
    }
    return '#64748b'; // Slate default
  }

  function getNodeStatus(nodeId: string): 'idle' | 'executing' | 'completed' {
    if (executingNode === nodeId) return 'executing';
    if (completedNodes.includes(nodeId)) return 'completed';
    return 'idle';
  }

  $: nodePositions = hdagInfo ? calculateLayout(hdagInfo.nodes, hdagInfo.edges) : new Map();
</script>

<div class="hdag-graph">
  <div class="bg-surface-900 rounded-lg overflow-hidden" style="width: {width}px; height: {height}px;">
    <svg {width} {height}>
      <!-- Background -->
      <rect width="100%" height="100%" fill="#0f172a"/>

      {#if hdagInfo}
        <!-- Edges -->
        {#each hdagInfo.edges as edge}
          {#if nodePositions.has(edge.from) && nodePositions.has(edge.to)}
            {@const fromPos = nodePositions.get(edge.from)}
            {@const toPos = nodePositions.get(edge.to)}
            <g>
              <!-- Arrow line -->
              <line
                x1={fromPos?.x}
                y1={fromPos?.y}
                x2={toPos?.x}
                y2={toPos?.y}
                stroke="rgba(255,255,255,0.3)"
                stroke-width="2"
                marker-end="url(#arrowhead)"
              />
              <!-- Edge label -->
              {#if edge.label}
                <text
                  x={(fromPos?.x ?? 0 + (toPos?.x ?? 0)) / 2}
                  y={(fromPos?.y ?? 0 + (toPos?.y ?? 0)) / 2 - 5}
                  text-anchor="middle"
                  fill="rgba(255,255,255,0.5)"
                  font-size="10"
                >
                  {edge.label}
                </text>
              {/if}
            </g>
          {/if}
        {/each}

        <!-- Arrow marker definition -->
        <defs>
          <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill="rgba(255,255,255,0.3)"/>
          </marker>
        </defs>

        <!-- Nodes -->
        {#each hdagInfo.nodes as node}
          {#if nodePositions.has(node.id)}
            {@const pos = nodePositions.get(node.id)}
            {@const status = getNodeStatus(node.id)}
            <g transform="translate({pos?.x}, {pos?.y})">
              <!-- Node background with status indication -->
              {#if status === 'executing'}
                <circle r="28" fill="none" stroke="#fbbf24" stroke-width="2">
                  <animate attributeName="r" values="28;32;28" dur="1s" repeatCount="indefinite"/>
                  <animate attributeName="opacity" values="1;0.5;1" dur="1s" repeatCount="indefinite"/>
                </circle>
              {/if}

              <!-- Main node circle -->
              <circle
                r="24"
                fill={getNodeColor(node.node_type)}
                stroke={status === 'completed' ? '#10b981' : 'rgba(255,255,255,0.2)'}
                stroke-width="2"
                opacity={status === 'completed' ? 1 : 0.8}
              />

              <!-- Node icon based on type -->
              <text
                y="4"
                text-anchor="middle"
                fill="white"
                font-size="12"
                font-weight="bold"
              >
                {#if node.node_type.includes('Input')}
                  IN
                {:else if node.node_type.includes('Output')}
                  OUT
                {:else if node.node_type.includes('Compilation')}
                  Xi
                {:else if node.node_type.includes('Merge')}
                  M
                {:else}
                  OP
                {/if}
              </text>

              <!-- Node name -->
              <text
                y="40"
                text-anchor="middle"
                fill="rgba(255,255,255,0.8)"
                font-size="10"
              >
                {node.name.length > 15 ? node.name.substring(0, 15) + '...' : node.name}
              </text>

              <!-- Status checkmark for completed nodes -->
              {#if status === 'completed'}
                <circle cx="16" cy="-16" r="8" fill="#10b981"/>
                <text x="16" y="-12" text-anchor="middle" fill="white" font-size="10">OK</text>
              {/if}
            </g>
          {/if}
        {/each}
      {:else}
        <text x={width / 2} y={height / 2} text-anchor="middle" fill="rgba(255,255,255,0.5)">
          No HDAG data loaded
        </text>
      {/if}
    </svg>
  </div>

  <!-- Legend and info -->
  {#if hdagInfo}
    <div class="mt-3 flex items-center justify-between text-xs text-slate-400">
      <div class="flex items-center gap-3">
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 rounded-full" style="background-color: {nodeColors.Input}"></div>
          <span>Input</span>
        </div>
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 rounded-full" style="background-color: {nodeColors.Operator}"></div>
          <span>Operator</span>
        </div>
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 rounded-full" style="background-color: {nodeColors.Merge}"></div>
          <span>Merge</span>
        </div>
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 rounded-full" style="background-color: {nodeColors.Compilation}"></div>
          <span>Compilation</span>
        </div>
        <div class="flex items-center gap-1">
          <div class="w-3 h-3 rounded-full" style="background-color: {nodeColors.Output}"></div>
          <span>Output</span>
        </div>
      </div>
      <span>{hdagInfo.nodes.length} nodes, {hdagInfo.edges.length} edges</span>
    </div>

    <!-- Execution order -->
    {#if hdagInfo.execution_order.length > 0}
      <div class="mt-2 p-2 bg-surface-800 rounded text-xs">
        <span class="text-slate-400">Execution order: </span>
        <span class="text-white">
          {hdagInfo.execution_order.map(id => {
            const node = hdagInfo?.nodes.find(n => n.id === id);
            return node?.name || id.substring(0, 8);
          }).join(' -> ')}
        </span>
      </div>
    {/if}
  {/if}
</div>
