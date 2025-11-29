<script lang="ts">
  /**
   * ConvergenceGraph - Visualizes cascade convergence and fixed-point detection
   *
   * Shows:
   * - Convergence line over iterations
   * - Fixed-point detection wave
   * - Resonant transitions as bright spots
   */

  export let data: Array<{ iteration: number; value: number; isResonant?: boolean }> = [];
  export let width: number = 400;
  export let height: number = 150;
  export let title: string = 'Convergence';
  export let threshold: number | null = null;
  export let showResonantPoints: boolean = true;

  $: padding = { top: 20, right: 20, bottom: 30, left: 40 };
  $: plotWidth = width - padding.left - padding.right;
  $: plotHeight = height - padding.top - padding.bottom;

  $: xScale = (i: number) =>
    padding.left + (data.length > 1 ? (i / (data.length - 1)) * plotWidth : plotWidth / 2);

  $: maxValue = Math.max(...data.map(d => d.value), threshold || 0, 1);
  $: minValue = Math.min(...data.map(d => d.value), 0);
  $: valueRange = maxValue - minValue || 1;

  $: yScale = (v: number) =>
    padding.top + plotHeight - ((v - minValue) / valueRange) * plotHeight;

  $: linePath = data.length > 1
    ? data.map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(i)} ${yScale(d.value)}`).join(' ')
    : '';

  $: areaPath = data.length > 1
    ? `${linePath} L ${xScale(data.length - 1)} ${yScale(minValue)} L ${xScale(0)} ${yScale(minValue)} Z`
    : '';

  $: resonantPoints = data.filter(d => d.isResonant);

  // Detect convergence (stable last 10%)
  $: isConverged = data.length > 10 && (() => {
    const lastN = data.slice(-Math.ceil(data.length * 0.1));
    const avg = lastN.reduce((s, d) => s + d.value, 0) / lastN.length;
    const variance = lastN.reduce((s, d) => s + Math.pow(d.value - avg, 2), 0) / lastN.length;
    return variance < 0.001;
  })();

  $: convergencePoint = isConverged ? data[data.length - 1]?.value : null;
</script>

<div class="convergence-graph" style="width: {width}px;">
  <div class="flex items-center justify-between mb-2">
    <h3 class="text-sm font-medium text-white">{title}</h3>
    {#if isConverged}
      <span class="text-xs text-emerald-400 flex items-center gap-1">
        <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
        </svg>
        Converged: {convergencePoint?.toFixed(4)}
      </span>
    {:else if data.length > 0}
      <span class="text-xs text-slate-400">{data.length} iterations</span>
    {/if}
  </div>

  <div class="bg-surface-900 rounded-lg overflow-hidden" style="height: {height}px;">
    <svg {width} {height}>
      <!-- Gradient definition -->
      <defs>
        <linearGradient id="convergenceGradient" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" style="stop-color:#8b5cf6;stop-opacity:0.3" />
          <stop offset="100%" style="stop-color:#8b5cf6;stop-opacity:0" />
        </linearGradient>
        <filter id="glowFilter" x="-50%" y="-50%" width="200%" height="200%">
          <feGaussianBlur stdDeviation="2" result="coloredBlur"/>
          <feMerge>
            <feMergeNode in="coloredBlur"/>
            <feMergeNode in="SourceGraphic"/>
          </feMerge>
        </filter>
      </defs>

      <!-- Grid lines -->
      {#each [0.25, 0.5, 0.75, 1] as frac}
        <line
          x1={padding.left}
          y1={yScale(minValue + frac * valueRange)}
          x2={width - padding.right}
          y2={yScale(minValue + frac * valueRange)}
          stroke="rgba(255,255,255,0.05)"
          stroke-width="1"
        />
        <text
          x={padding.left - 5}
          y={yScale(minValue + frac * valueRange) + 3}
          text-anchor="end"
          fill="rgba(255,255,255,0.3)"
          font-size="8"
        >
          {(minValue + frac * valueRange).toFixed(2)}
        </text>
      {/each}

      <!-- Threshold line -->
      {#if threshold !== null}
        <line
          x1={padding.left}
          y1={yScale(threshold)}
          x2={width - padding.right}
          y2={yScale(threshold)}
          stroke="#f59e0b"
          stroke-width="1"
          stroke-dasharray="4 2"
          opacity="0.7"
        />
        <text
          x={width - padding.right + 2}
          y={yScale(threshold) + 3}
          fill="#f59e0b"
          font-size="8"
        >
          θ
        </text>
      {/if}

      <!-- Area fill -->
      {#if areaPath}
        <path d={areaPath} fill="url(#convergenceGradient)"/>
      {/if}

      <!-- Main line -->
      {#if linePath}
        <path
          d={linePath}
          fill="none"
          stroke="#8b5cf6"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      {/if}

      <!-- Resonant points (bright spots) -->
      {#if showResonantPoints}
        {#each data as point, i}
          {#if point.isResonant}
            <circle
              cx={xScale(i)}
              cy={yScale(point.value)}
              r="6"
              fill="#fbbf24"
              filter="url(#glowFilter)"
            >
              <animate attributeName="r" values="4;8;4" dur="1s" repeatCount="indefinite"/>
              <animate attributeName="opacity" values="1;0.5;1" dur="1s" repeatCount="indefinite"/>
            </circle>
          {/if}
        {/each}
      {/if}

      <!-- Current point indicator -->
      {#if data.length > 0}
        {@const lastPoint = data[data.length - 1]}
        <circle
          cx={xScale(data.length - 1)}
          cy={yScale(lastPoint.value)}
          r="4"
          fill={isConverged ? '#10b981' : '#8b5cf6'}
        />
      {/if}

      <!-- X-axis labels -->
      <text
        x={padding.left}
        y={height - 8}
        fill="rgba(255,255,255,0.3)"
        font-size="8"
      >
        0
      </text>
      <text
        x={width - padding.right}
        y={height - 8}
        text-anchor="end"
        fill="rgba(255,255,255,0.3)"
        font-size="8"
      >
        {data.length}
      </text>
      <text
        x={width / 2}
        y={height - 8}
        text-anchor="middle"
        fill="rgba(255,255,255,0.4)"
        font-size="9"
      >
        Iteration
      </text>
    </svg>
  </div>

  <!-- Stats -->
  {#if data.length > 0}
    <div class="flex items-center justify-between mt-2 text-xs text-slate-400">
      <span>Min: {minValue.toFixed(4)}</span>
      <span>Max: {maxValue.toFixed(4)}</span>
      {#if resonantPoints.length > 0}
        <span class="text-amber-400">{resonantPoints.length} resonant</span>
      {/if}
    </div>
  {/if}
</div>
