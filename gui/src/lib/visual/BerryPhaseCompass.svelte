<script lang="ts">
  /**
   * BerryPhaseCompass - Polar plot for Berry phase visualization
   *
   * Shows:
   * - Animated vector rotating with phase accumulation
   * - 2π detection triggers glow animation
   * - Phase history trail
   */

  import { onMount, onDestroy } from 'svelte';
  import { easing } from './animations';

  export let phase: number = 0;  // Current phase in radians
  export let size: number = 150;
  export let title: string = 'Berry Phase';
  export let animated: boolean = true;
  export let showHistory: boolean = true;

  let displayPhase = 0;
  let phaseHistory: number[] = [];
  let animationFrame: number;
  let glowActive = false;
  let glowOpacity = 0;

  const maxHistoryLength = 50;

  $: center = size / 2;
  $: radius = (size - 40) / 2;
  $: normalizedPhase = ((displayPhase % (2 * Math.PI)) + 2 * Math.PI) % (2 * Math.PI);
  $: phaseAngle = normalizedPhase - Math.PI / 2; // Start from top
  $: arrowX = center + radius * 0.8 * Math.cos(phaseAngle);
  $: arrowY = center + radius * 0.8 * Math.sin(phaseAngle);
  $: completedCycles = Math.floor(displayPhase / (2 * Math.PI));
  $: isNear2Pi = normalizedPhase > 0.95 * 2 * Math.PI || normalizedPhase < 0.05 * 2 * Math.PI;

  // Track phase crossings using a separate variable to avoid reactive loops
  let lastCheckedPhase = 0;

  function checkPhaseCrossing(currentPhase: number) {
    const crossedUp = lastCheckedPhase < 2 * Math.PI && currentPhase >= 2 * Math.PI;
    const crossedDown = lastCheckedPhase >= 0 && currentPhase < 0;
    if (crossedUp || crossedDown) {
      triggerGlow();
    }
    lastCheckedPhase = currentPhase;
  }

  function triggerGlow() {
    glowActive = true;
    glowOpacity = 1;
    setTimeout(() => {
      glowActive = false;
    }, 1000);
  }

  // Animate value changes
  $: if (animated) {
    const diff = phase - displayPhase;
    displayPhase += diff * 0.1;
    checkPhaseCrossing(displayPhase);

    // Track history
    if (showHistory && Math.abs(diff) > 0.01) {
      phaseHistory = [...phaseHistory, normalizedPhase].slice(-maxHistoryLength);
    }
  } else {
    displayPhase = phase;
  }

  onMount(() => {
    if (animated) {
      function animate() {
        if (glowActive) {
          glowOpacity *= 0.95;
        }
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

  function getHistoryPoint(histPhase: number, index: number) {
    const angle = histPhase - Math.PI / 2;
    const r = radius * 0.7 * (0.5 + 0.5 * (index / maxHistoryLength));
    return {
      x: center + r * Math.cos(angle),
      y: center + r * Math.sin(angle),
    };
  }

  // Arrow head calculation
  function getArrowHead(x: number, y: number, angle: number, length: number = 10) {
    const a1 = angle + Math.PI * 0.85;
    const a2 = angle - Math.PI * 0.85;
    return {
      p1: { x: x + length * Math.cos(a1), y: y + length * Math.sin(a1) },
      p2: { x: x + length * Math.cos(a2), y: y + length * Math.sin(a2) },
    };
  }

  $: arrowHead = getArrowHead(arrowX, arrowY, phaseAngle);
</script>

<div class="berry-phase-compass" style="width: {size}px; height: {size}px;">
  <svg width={size} height={size}>
    <!-- Glow effect on 2π crossing -->
    {#if glowActive}
      <circle
        cx={center}
        cy={center}
        r={radius + 10}
        fill="none"
        stroke="#10b981"
        stroke-width="4"
        opacity={glowOpacity}
      >
        <animate attributeName="r" values="{radius};{radius + 20}" dur="0.5s"/>
      </circle>
    {/if}

    <!-- Background circles -->
    <circle
      cx={center}
      cy={center}
      r={radius}
      fill="none"
      stroke="rgba(255,255,255,0.1)"
      stroke-width="1"
    />
    <circle
      cx={center}
      cy={center}
      r={radius * 0.5}
      fill="none"
      stroke="rgba(255,255,255,0.05)"
      stroke-width="1"
    />

    <!-- Cardinal directions -->
    {#each [0, 90, 180, 270] as deg}
      {@const rad = (deg - 90) * Math.PI / 180}
      {@const x1 = center + (radius - 5) * Math.cos(rad)}
      {@const y1 = center + (radius - 5) * Math.sin(rad)}
      {@const x2 = center + (radius + 5) * Math.cos(rad)}
      {@const y2 = center + (radius + 5) * Math.sin(rad)}
      <line x1={x1} y1={y1} x2={x2} y2={y2} stroke="rgba(255,255,255,0.3)" stroke-width="2"/>
    {/each}

    <!-- Phase labels -->
    <text x={center} y="12" text-anchor="middle" fill="rgba(255,255,255,0.5)" font-size="9">0</text>
    <text x={size - 8} y={center + 3} text-anchor="middle" fill="rgba(255,255,255,0.5)" font-size="9">π/2</text>
    <text x={center} y={size - 5} text-anchor="middle" fill="rgba(255,255,255,0.5)" font-size="9">π</text>
    <text x="8" y={center + 3} text-anchor="middle" fill="rgba(255,255,255,0.5)" font-size="9">3π/2</text>

    <!-- Phase history trail -->
    {#if showHistory}
      {#each phaseHistory as histPhase, i}
        {@const pt = getHistoryPoint(histPhase, i)}
        <circle
          cx={pt.x}
          cy={pt.y}
          r="2"
          fill="#8b5cf6"
          opacity={0.2 + 0.6 * (i / maxHistoryLength)}
        />
      {/each}
    {/if}

    <!-- Main phase vector -->
    <line
      x1={center}
      y1={center}
      x2={arrowX}
      y2={arrowY}
      stroke={isNear2Pi ? '#10b981' : '#f59e0b'}
      stroke-width="3"
      stroke-linecap="round"
    />

    <!-- Arrow head -->
    <polygon
      points="{arrowX},{arrowY} {arrowHead.p1.x},{arrowHead.p1.y} {arrowHead.p2.x},{arrowHead.p2.y}"
      fill={isNear2Pi ? '#10b981' : '#f59e0b'}
    />

    <!-- Center point -->
    <circle cx={center} cy={center} r="4" fill="#f59e0b"/>

    <!-- Value display -->
    <text
      x={center}
      y={center + radius + 18}
      text-anchor="middle"
      fill="white"
      font-size="11"
      font-weight="bold"
      font-family="monospace"
    >
      {(normalizedPhase / Math.PI).toFixed(2)}π
    </text>

    <!-- Cycles counter -->
    {#if completedCycles > 0}
      <text
        x={center}
        y={center - 5}
        text-anchor="middle"
        fill="#10b981"
        font-size="10"
        font-weight="bold"
      >
        ×{completedCycles}
      </text>
    {/if}

    <!-- Title -->
    <text
      x={center}
      y={size - 5}
      text-anchor="middle"
      fill="rgba(255,255,255,0.6)"
      font-size="9"
    >
      {title}
    </text>
  </svg>
</div>

<style>
  .berry-phase-compass {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
