<script lang="ts">
  /**
   * OperatorFlow - Visualizes DK/SC/PI operator sequence and effects
   *
   * Shows:
   * - Operator timeline with clickable nodes
   * - 2D plane visuals (ψ-ρ, ω-χ)
   * - Vector rotations and gradient flows
   */

  import { operatorColors } from './colors';

  export let operators: Array<{
    type: 'DK' | 'SC' | 'PI' | 'WT';
    timestamp: number;
    effect: { input: number[]; output: number[] };
  }> = [];
  export let width: number = 400;
  export let height: number = 200;
  export let selectedIndex: number | null = null;
  export let onSelect: ((index: number) => void) | null = null;

  $: timelineWidth = width - 60;
  $: maxTimestamp = operators.length > 0 ? Math.max(...operators.map(o => o.timestamp)) : 1;

  function getOperatorPosition(timestamp: number) {
    return 30 + (timestamp / maxTimestamp) * timelineWidth;
  }

  function getOperatorIcon(type: string): string {
    switch (type) {
      case 'DK': return 'DK';  // Double Kick
      case 'SC': return 'SC';  // Swap/Cycle
      case 'PI': return 'PI';  // Path Integration
      case 'WT': return 'WT';  // Weight Transform
      default: return '?';
    }
  }

  function handleOperatorClick(index: number) {
    selectedIndex = index;
    onSelect?.(index);
  }

  $: selectedOp = selectedIndex !== null ? operators[selectedIndex] : null;
</script>

<div class="operator-flow" style="width: {width}px;">
  <!-- Timeline view -->
  <div class="relative bg-surface-900 rounded-lg p-4" style="height: {height}px;">
    <svg width={width - 32} height={height - 32}>
      <!-- Timeline axis -->
      <line
        x1="30"
        y1={height / 2 - 16}
        x2={width - 62}
        y2={height / 2 - 16}
        stroke="rgba(255,255,255,0.2)"
        stroke-width="2"
      />

      <!-- Time markers -->
      {#each [0, 0.25, 0.5, 0.75, 1] as t}
        <line
          x1={30 + t * timelineWidth}
          y1={height / 2 - 24}
          x2={30 + t * timelineWidth}
          y2={height / 2 - 8}
          stroke="rgba(255,255,255,0.3)"
          stroke-width="1"
        />
        <text
          x={30 + t * timelineWidth}
          y={height / 2 + 5}
          text-anchor="middle"
          fill="rgba(255,255,255,0.4)"
          font-size="9"
        >
          {(t * maxTimestamp).toFixed(1)}
        </text>
      {/each}

      <!-- Operator connections -->
      {#each operators as op, i}
        {#if i > 0}
          {@const prevOp = operators[i - 1]}
          <line
            x1={getOperatorPosition(prevOp.timestamp)}
            y1={height / 2 - 16}
            x2={getOperatorPosition(op.timestamp)}
            y2={height / 2 - 16}
            stroke={operatorColors[op.type]}
            stroke-width="3"
            opacity="0.5"
          />
        {/if}
      {/each}

      <!-- Operator nodes -->
      {#each operators as op, i}
        {@const x = getOperatorPosition(op.timestamp)}
        {@const y = height / 2 - 16}
        {@const isSelected = selectedIndex === i}
        {@const opLabel = op.type === 'DK' ? 'Double Kick' : op.type === 'SC' ? 'Swap/Cycle' : op.type === 'PI' ? 'Path Integration' : 'Weight Transform'}
        <g
          transform="translate({x}, {y})"
          class="cursor-pointer"
          on:click={() => handleOperatorClick(i)}
          on:keydown={(e) => e.key === 'Enter' && handleOperatorClick(i)}
          role="button"
          tabindex="0"
          aria-label="{opLabel} operator at timestamp {op.timestamp.toFixed(3)}"
        >
          <!-- Pulse effect for selected -->
          {#if isSelected}
            <circle r="20" fill="none" stroke={operatorColors[op.type]} stroke-width="2">
              <animate attributeName="r" values="15;25;15" dur="1.5s" repeatCount="indefinite"/>
              <animate attributeName="opacity" values="0.8;0.2;0.8" dur="1.5s" repeatCount="indefinite"/>
            </circle>
          {/if}

          <!-- Node circle -->
          <circle
            r="12"
            fill={operatorColors[op.type]}
            stroke={isSelected ? 'white' : 'transparent'}
            stroke-width="2"
          />

          <!-- Operator label -->
          <text
            y="4"
            text-anchor="middle"
            fill="white"
            font-size="8"
            font-weight="bold"
          >
            {getOperatorIcon(op.type)}
          </text>
        </g>
      {/each}

      <!-- Arrow markers -->
      <defs>
        <marker id="flowArrow" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
          <polygon points="0 0, 10 3.5, 0 7" fill="rgba(255,255,255,0.5)"/>
        </marker>
      </defs>

      <!-- Flow direction indicator -->
      <line
        x1={width - 90}
        y1={height / 2 - 16}
        x2={width - 70}
        y2={height / 2 - 16}
        stroke="rgba(255,255,255,0.5)"
        stroke-width="2"
        marker-end="url(#flowArrow)"
      />
    </svg>

    <!-- Title -->
    <div class="absolute top-2 left-2 text-xs text-slate-400">
      Operator Timeline
    </div>
  </div>

  <!-- Selected operator details -->
  {#if selectedOp}
    <div class="mt-3 bg-surface-800 rounded-lg p-3">
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center gap-2">
          <div
            class="w-8 h-8 rounded-full flex items-center justify-center text-white text-xs font-bold"
            style="background: {operatorColors[selectedOp.type]}"
          >
            {selectedOp.type}
          </div>
          <div>
            <div class="text-white text-sm font-medium">
              {selectedOp.type === 'DK' ? 'Double Kick' :
               selectedOp.type === 'SC' ? 'Swap/Cycle' :
               selectedOp.type === 'PI' ? 'Path Integration' :
               'Weight Transform'}
            </div>
            <div class="text-xs text-slate-400">t = {selectedOp.timestamp.toFixed(3)}</div>
          </div>
        </div>
      </div>

      <!-- Effect visualization (2D projection) -->
      <div class="grid grid-cols-2 gap-2">
        <!-- Input state -->
        <div class="bg-surface-700 rounded p-2">
          <div class="text-xs text-slate-400 mb-1">Input</div>
          <div class="flex flex-wrap gap-1">
            {#each selectedOp.effect.input as val, i}
              <span class="text-xs text-white font-mono">
                {['ψ', 'ρ', 'ω', 'χ', 'η'][i]}:{val.toFixed(2)}
              </span>
            {/each}
          </div>
        </div>

        <!-- Output state -->
        <div class="bg-surface-700 rounded p-2">
          <div class="text-xs text-slate-400 mb-1">Output</div>
          <div class="flex flex-wrap gap-1">
            {#each selectedOp.effect.output as val, i}
              <span class="text-xs font-mono" style="color: {operatorColors[selectedOp.type]}">
                {['ψ', 'ρ', 'ω', 'χ', 'η'][i]}:{val.toFixed(2)}
              </span>
            {/each}
          </div>
        </div>
      </div>

      <!-- Vector change visualization -->
      <div class="mt-2 h-16 bg-surface-700 rounded relative overflow-hidden">
        <svg width="100%" height="64">
          <!-- ψ-ρ plane -->
          <g transform="translate(40, 32)">
            <circle r="25" fill="none" stroke="rgba(255,255,255,0.1)" stroke-width="1"/>
            <!-- Input vector -->
            <line
              x1="0"
              y1="0"
              x2={selectedOp.effect.input[0] * 25}
              y2={-selectedOp.effect.input[1] * 25}
              stroke="rgba(255,255,255,0.5)"
              stroke-width="2"
            />
            <!-- Output vector -->
            <line
              x1="0"
              y1="0"
              x2={selectedOp.effect.output[0] * 25}
              y2={-selectedOp.effect.output[1] * 25}
              stroke={operatorColors[selectedOp.type]}
              stroke-width="2"
            />
            <circle r="3" fill="white"/>
          </g>
          <text x="40" y="60" text-anchor="middle" fill="rgba(255,255,255,0.4)" font-size="8">ψ-ρ</text>

          <!-- ω-χ plane -->
          <g transform="translate(120, 32)">
            <circle r="25" fill="none" stroke="rgba(255,255,255,0.1)" stroke-width="1"/>
            <!-- Input vector -->
            <line
              x1="0"
              y1="0"
              x2={selectedOp.effect.input[2] * 25}
              y2={-selectedOp.effect.input[3] * 25}
              stroke="rgba(255,255,255,0.5)"
              stroke-width="2"
            />
            <!-- Output vector -->
            <line
              x1="0"
              y1="0"
              x2={selectedOp.effect.output[2] * 25}
              y2={-selectedOp.effect.output[3] * 25}
              stroke={operatorColors[selectedOp.type]}
              stroke-width="2"
            />
            <circle r="3" fill="white"/>
          </g>
          <text x="120" y="60" text-anchor="middle" fill="rgba(255,255,255,0.4)" font-size="8">ω-χ</text>
        </svg>
      </div>
    </div>
  {:else}
    <div class="mt-3 bg-surface-800 rounded-lg p-4 text-center text-slate-500 text-sm">
      Click an operator to view details
    </div>
  {/if}

  <!-- Legend -->
  <div class="flex items-center justify-center gap-4 mt-3 text-xs">
    {#each Object.entries(operatorColors) as [type, color]}
      <div class="flex items-center gap-1">
        <div class="w-3 h-3 rounded-full" style="background: {color}"></div>
        <span class="text-slate-400">{type}</span>
      </div>
    {/each}
  </div>
</div>
