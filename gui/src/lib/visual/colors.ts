/**
 * QOPS Adaptive Color System
 *
 * Provides color schemes based on resonance stages:
 * - Kosmokrator (blue)
 * - Chronokrator (violet)
 * - Pfauenthron (amber/gold)
 * - Finalized (emerald)
 */

// Stage colors following the holistic resonance architecture
export const stageColors = {
  discovery: '#3b82f6',       // Blue
  kosmokrator: '#00bfff',     // Deep sky blue
  chronokrator: '#9932cc',    // Violet
  pfauenthron: '#ffd700',     // Gold
  finalized: '#10b981',       // Emerald
};

// Operator type colors
export const operatorColors = {
  DK: '#a78bfa',    // Double Kick - purple pulse
  SC: '#fb923c',    // Swap/Cycle - orange flow
  PI: '#3b82f6',    // Path Integration - blue integrator
  WT: '#14b8a6',    // Weight Transform - teal
};

// Resonance gradient colors
export const resonanceGradient = [
  { stop: 0.0, color: '#1e3a5f' },   // Deep blue
  { stop: 0.3, color: '#3b82f6' },   // Blue
  { stop: 0.5, color: '#8b5cf6' },   // Violet
  { stop: 0.7, color: '#f59e0b' },   // Amber
  { stop: 0.85, color: '#fbbf24' },  // Gold
  { stop: 1.0, color: '#10b981' },   // Emerald
];

// Dimensional collapse transition colors
export const cascadeColors = {
  n_to_5: '#9333ea',   // Purple - n -> 5
  d5_to_3: '#3b82f6',  // Blue - 5 -> 3
  d3_to_2: '#f97316',  // Orange - 3 -> 2
  d2_to_1: '#fbbf24',  // Gold - 2 -> 1
};

/**
 * Get color based on resonance value
 */
export function getResonanceColor(value: number): string {
  const v = Math.max(0, Math.min(1, value));

  if (v < 0.3) {
    // Blue range (Kosmokrator)
    const t = v / 0.3;
    return interpolateColor('#1e3a5f', '#3b82f6', t);
  } else if (v < 0.6) {
    // Violet range (Chronokrator)
    const t = (v - 0.3) / 0.3;
    return interpolateColor('#3b82f6', '#8b5cf6', t);
  } else if (v < 0.85) {
    // Amber range (Pfauenthron)
    const t = (v - 0.6) / 0.25;
    return interpolateColor('#8b5cf6', '#fbbf24', t);
  } else {
    // Emerald range (Finalized)
    const t = (v - 0.85) / 0.15;
    return interpolateColor('#fbbf24', '#10b981', t);
  }
}

/**
 * Get stage color with optional intensity
 */
export function getStageColor(stage: string, intensity = 1.0): string {
  const baseColor = stageColors[stage as keyof typeof stageColors] || '#64748b';
  if (intensity === 1.0) return baseColor;
  return adjustBrightness(baseColor, intensity);
}

/**
 * Get operator color
 */
export function getOperatorColor(operatorType: string): string {
  return operatorColors[operatorType as keyof typeof operatorColors] || '#64748b';
}

/**
 * Interpolate between two colors
 */
export function interpolateColor(color1: string, color2: string, t: number): string {
  const c1 = hexToRgb(color1);
  const c2 = hexToRgb(color2);

  if (!c1 || !c2) return color1;

  const r = Math.round(c1.r + (c2.r - c1.r) * t);
  const g = Math.round(c1.g + (c2.g - c1.g) * t);
  const b = Math.round(c1.b + (c2.b - c1.b) * t);

  return `rgb(${r}, ${g}, ${b})`;
}

/**
 * Convert hex to RGB
 */
export function hexToRgb(hex: string): { r: number; g: number; b: number } | null {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16),
      }
    : null;
}

/**
 * Adjust color brightness
 */
export function adjustBrightness(hex: string, factor: number): string {
  const rgb = hexToRgb(hex);
  if (!rgb) return hex;

  const r = Math.min(255, Math.round(rgb.r * factor));
  const g = Math.min(255, Math.round(rgb.g * factor));
  const b = Math.min(255, Math.round(rgb.b * factor));

  return `rgb(${r}, ${g}, ${b})`;
}

/**
 * Create CSS gradient string from resonance gradient
 */
export function createResonanceGradient(direction = 'to right'): string {
  const stops = resonanceGradient
    .map((s) => `${s.color} ${s.stop * 100}%`)
    .join(', ');
  return `linear-gradient(${direction}, ${stops})`;
}

/**
 * Get glow color for high-resonance elements
 */
export function getGlowColor(resonance: number): string {
  if (resonance >= 0.95) return '0 0 20px #10b981, 0 0 40px #10b98155';
  if (resonance >= 0.85) return '0 0 15px #fbbf24, 0 0 30px #fbbf2455';
  if (resonance >= 0.7) return '0 0 10px #8b5cf6, 0 0 20px #8b5cf655';
  return 'none';
}
