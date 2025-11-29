<script lang="ts">
  /**
   * ChernGauge - Circular topology gauge visualization
   *
   * Displays Chern number as a circular gauge where:
   * - Full circle = perfect topology (Chern = 1)
   * - Color gradient from red (0) to gold (1)
   * - Animated arc with glow effect
   */

  import { onMount, onDestroy } from 'svelte';
  import { getResonanceColor, getGlowColor } from './colors';
  import { createPulse, easing, tween } from './animations';

  export let value: number = 0;  // Chern number (0-1)
  export let size: number = 150;
  export let title: string = 'Chern Number';
  export let animated: boolean = true;
  export let showLabel: boolean = true;

  let displayValue = 0;
  let animationFrame: number;
  let pulseTime = 0;

  $: radius = (size - 20) / 2;
  $: center = size / 2;
  $: circumference = 2 * Math.PI * (radius - 10);
  $: strokeDasharray = `${circumference * displayValue} ${circumference * (1 - displayValue)}`;
  $: strokeColor = getResonanceColor(displayValue);
  $: glowEffect = displayValue >= 0.9 ? getGlowColor(displayValue) : 'none';
  $: pulseScale = displayValue >= 0.95 ? createPulse(1, 1.05, 2)(pulseTime) : 1;

  // Animate value changes
  $: if (animated && typeof value === 'number') {
    tween(displayValue, value, 800, easing.easeOutCubic, (v) => {
      displayValue = v;
    });
  } else {
    displayValue = value;
  }

  onMount(() => {
    if (animated) {
      function animate() {
        pulseTime += 0.016;
        animationFrame = requestAnimationFrame(animate);
      }
      animate();
    }
  });

  onDestroy(() => {
    if (animationFrame) {
      cancelAnimationFrame(animationFrame);
    }
  });

  function polarToCartesian(cx: number, cy: number, r: number, angle: number) {
    const rad = ((angle - 90) * Math.PI) / 180;
    return {
      x: cx + r * Math.cos(rad),
      y: cy + r * Math.sin(rad),
    };
  }

  function describeArc(x: number, y: number, r: number, startAngle: number, endAngle: number) {
    const start = polarToCartesian(x, y, r, endAngle);
    const end = polarToCartesian(x, y, r, startAngle);
    const largeArcFlag = endAngle - startAngle <= 180 ? 0 : 1;
    return [
      'M', start.x, start.y,
      'A', r, r, 0, largeArcFlag, 0, end.x, end.y
    ].join(' ');
  }

  $: arcPath = displayValue > 0 ? describeArc(center, center, radius - 10, 0, displayValue * 360) : '';
</script>

<div class="chern-gauge" style="width: {size}px; height: {size}px;">
  <svg width={size} height={size} style="transform: scale({pulseScale})">
    <!-- Background circle -->
    <circle
      cx={center}
      cy={center}
      r={radius - 10}
      fill="none"
      stroke="rgba(255,255,255,0.1)"
      stroke-width="8"
    />

    <!-- Gradient definition -->
    <defs>
      <linearGradient id="chernGradient" x1="0%" y1="0%" x2="100%" y2="0%">
        <stop offset="0%" style="stop-color:#ef4444;stop-opacity:1" />
        <stop offset="50%" style="stop-color:#f59e0b;stop-opacity:1" />
        <stop offset="100%" style="stop-color:#fbbf24;stop-opacity:1" />
      </linearGradient>
      <filter id="chernGlow" x="-50%" y="-50%" width="200%" height="200%">
        <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
        <feMerge>
          <feMergeNode in="coloredBlur"/>
          <feMergeNode in="SourceGraphic"/>
        </feMerge>
      </filter>
    </defs>

    <!-- Value arc -->
    {#if displayValue > 0.001}
      <path
        d={arcPath}
        fill="none"
        stroke={strokeColor}
        stroke-width="8"
        stroke-linecap="round"
        filter={displayValue >= 0.9 ? 'url(#chernGlow)' : 'none'}
      />
    {/if}

    <!-- Tick marks -->
    {#each [0, 0.25, 0.5, 0.75, 1] as tick}
      {@const pos = polarToCartesian(center, center, radius - 25, tick * 360)}
      <circle
        cx={pos.x}
        cy={pos.y}
        r="2"
        fill={tick <= displayValue ? strokeColor : 'rgba(255,255,255,0.2)'}
      />
    {/each}

    <!-- Center value display -->
    {#if showLabel}
      <text
        x={center}
        y={center - 8}
        text-anchor="middle"
        fill={strokeColor}
        font-size="24"
        font-weight="bold"
        font-family="monospace"
      >
        {displayValue.toFixed(2)}
      </text>
      <text
        x={center}
        y={center + 15}
        text-anchor="middle"
        fill="rgba(255,255,255,0.6)"
        font-size="10"
      >
        {title}
      </text>
    {/if}

    <!-- Perfect topology indicator -->
    {#if displayValue >= 0.99}
      <circle
        cx={center}
        cy={center}
        r={radius + 5}
        fill="none"
        stroke="#fbbf24"
        stroke-width="2"
        opacity="0.5"
      >
        <animate attributeName="r" values="{radius + 5};{radius + 15};{radius + 5}" dur="2s" repeatCount="indefinite"/>
        <animate attributeName="opacity" values="0.5;0.1;0.5" dur="2s" repeatCount="indefinite"/>
      </circle>
    {/if}
  </svg>
</div>

<style>
  .chern-gauge {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
