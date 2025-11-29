/**
 * QOPS Animation Utilities
 *
 * Provides animation helpers for smooth transitions and effects.
 */

/**
 * Easing functions
 */
export const easing = {
  linear: (t: number) => t,
  easeInQuad: (t: number) => t * t,
  easeOutQuad: (t: number) => t * (2 - t),
  easeInOutQuad: (t: number) => (t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t),
  easeInCubic: (t: number) => t * t * t,
  easeOutCubic: (t: number) => --t * t * t + 1,
  easeInOutCubic: (t: number) =>
    t < 0.5 ? 4 * t * t * t : (t - 1) * (2 * t - 2) * (2 * t - 2) + 1,
  easeInExpo: (t: number) => (t === 0 ? 0 : Math.pow(2, 10 * (t - 1))),
  easeOutExpo: (t: number) => (t === 1 ? 1 : 1 - Math.pow(2, -10 * t)),
  easeInOutExpo: (t: number) => {
    if (t === 0 || t === 1) return t;
    if (t < 0.5) return Math.pow(2, 10 * (2 * t - 1)) / 2;
    return (2 - Math.pow(2, -10 * (2 * t - 1))) / 2;
  },
  // Bounce effect for jackpot animations
  easeOutBounce: (t: number) => {
    if (t < 1 / 2.75) return 7.5625 * t * t;
    if (t < 2 / 2.75) return 7.5625 * (t -= 1.5 / 2.75) * t + 0.75;
    if (t < 2.5 / 2.75) return 7.5625 * (t -= 2.25 / 2.75) * t + 0.9375;
    return 7.5625 * (t -= 2.625 / 2.75) * t + 0.984375;
  },
  // Elastic for resonance pulse
  easeOutElastic: (t: number) => {
    const p = 0.3;
    return Math.pow(2, -10 * t) * Math.sin(((t - p / 4) * (2 * Math.PI)) / p) + 1;
  },
};

/**
 * Animation frame loop helper
 */
export function createAnimationLoop(
  callback: (deltaTime: number, elapsedTime: number) => boolean | void,
  fps = 60
): { start: () => void; stop: () => void } {
  let frameId: number | null = null;
  let lastTime = 0;
  let startTime = 0;
  const frameInterval = 1000 / fps;

  function loop(currentTime: number) {
    if (!startTime) startTime = currentTime;
    const deltaTime = currentTime - lastTime;

    if (deltaTime >= frameInterval) {
      lastTime = currentTime - (deltaTime % frameInterval);
      const elapsedTime = currentTime - startTime;

      if (callback(deltaTime / 1000, elapsedTime / 1000) === false) {
        return;
      }
    }

    frameId = requestAnimationFrame(loop);
  }

  return {
    start() {
      if (!frameId) {
        lastTime = 0;
        startTime = 0;
        frameId = requestAnimationFrame(loop);
      }
    },
    stop() {
      if (frameId) {
        cancelAnimationFrame(frameId);
        frameId = null;
      }
    },
  };
}

/**
 * Tween a value over time
 */
export function tween(
  from: number,
  to: number,
  duration: number,
  easingFn: (t: number) => number = easing.easeOutCubic,
  onUpdate: (value: number) => void,
  onComplete?: () => void
): () => void {
  const startTime = performance.now();
  let cancelled = false;
  let frameId: number | null = null;

  function update() {
    if (cancelled) return;

    const elapsed = performance.now() - startTime;
    const progress = Math.min(1, elapsed / duration);
    const easedProgress = easingFn(progress);
    const value = from + (to - from) * easedProgress;

    onUpdate(value);

    if (progress < 1) {
      frameId = requestAnimationFrame(update);
    } else {
      onComplete?.();
    }
  }

  frameId = requestAnimationFrame(update);

  return () => {
    cancelled = true;
    if (frameId !== null) {
      cancelAnimationFrame(frameId);
    }
  };
}

/**
 * Create a pulsing animation value
 */
export function createPulse(
  minValue: number,
  maxValue: number,
  frequency: number = 1
): (time: number) => number {
  return (time: number) => {
    const t = (Math.sin(time * frequency * Math.PI * 2) + 1) / 2;
    return minValue + (maxValue - minValue) * t;
  };
}

/**
 * Create a spiral motion path
 */
export function createSpiralPath(
  centerX: number,
  centerY: number,
  startRadius: number,
  endRadius: number,
  turns: number
): (t: number) => { x: number; y: number } {
  const PHI = 1.618033988749895; // Golden ratio

  return (t: number) => {
    const angle = t * turns * 2 * Math.PI;
    const radius = startRadius + (endRadius - startRadius) * t;
    const phiOffset = t * PHI;

    return {
      x: centerX + radius * Math.cos(angle + phiOffset),
      y: centerY + radius * Math.sin(angle + phiOffset),
    };
  };
}

/**
 * Create dimensional collapse animation parameters
 */
export interface CollapseState {
  dimension: number;
  progress: number;
  scale: number;
  opacity: number;
  color: string;
}

export function createCollapseAnimation(
  fromDim: number,
  toDim: number,
  duration: number = 1000
): (elapsed: number) => CollapseState {
  const colors: Record<string, string> = {
    'n-5': '#9333ea',
    '5-3': '#3b82f6',
    '3-2': '#f97316',
    '2-1': '#fbbf24',
  };

  const colorKey = `${fromDim}-${toDim}`;
  const color = colors[colorKey] || '#8b5cf6';

  return (elapsed: number) => {
    const progress = Math.min(1, elapsed / duration);
    const easedProgress = easing.easeInOutCubic(progress);

    return {
      dimension: fromDim + (toDim - fromDim) * easedProgress,
      progress: easedProgress,
      scale: 1 - easedProgress * 0.3,
      opacity: 1 - easedProgress * 0.2,
      color,
    };
  };
}

/**
 * Create slot reel spin animation
 */
export function createReelSpin(
  symbolCount: number,
  spins: number = 3,
  duration: number = 2000
): (elapsed: number) => { offset: number; symbolIndex: number } {
  const totalSymbols = symbolCount * spins;

  return (elapsed: number) => {
    const progress = Math.min(1, elapsed / duration);
    const easedProgress = easing.easeOutExpo(progress);
    const offset = easedProgress * totalSymbols;
    const symbolIndex = Math.floor(offset) % symbolCount;

    return {
      offset: offset % symbolCount,
      symbolIndex,
    };
  };
}

/**
 * Create jackpot flash animation
 */
export function createJackpotFlash(duration: number = 3000): (elapsed: number) => { intensity: number; scale: number; rotation: number } {
  return (elapsed: number) => {
    const progress = Math.min(1, elapsed / duration);

    // Multiple flash pulses
    const flashFreq = 8;
    const flash = Math.pow(Math.sin(progress * flashFreq * Math.PI), 2);

    // Scale bounce
    const scaleProgress = easing.easeOutBounce(Math.min(1, progress * 2));

    // Rotation
    const rotation = progress * 720 * easing.easeOutExpo(progress);

    return {
      intensity: flash * (1 - progress * 0.5),
      scale: 1 + scaleProgress * 0.3,
      rotation,
    };
  };
}

/**
 * Interpolate between two values with optional easing
 */
export function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

/**
 * Smoothstep interpolation
 */
export function smoothstep(edge0: number, edge1: number, x: number): number {
  const t = Math.max(0, Math.min(1, (x - edge0) / (edge1 - edge0)));
  return t * t * (3 - 2 * t);
}
