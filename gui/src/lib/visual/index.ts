/**
 * QOPS Visual Engine Library
 *
 * Modular visualization infrastructure for the Quantum Operator Processing System.
 *
 * Modules:
 * - HypercubeRenderer: Three.js-based 3D/4D hypercube projection
 * - PlotEngine: D3.js-powered scientific plots
 * - OperatorRenderer: Canvas2D operator flow visualization
 * - ResonanceVisualizer: WebGL resonance field rendering
 * - CascadeEngine: Dimensional collapse transition animations
 * - ColorSystem: Adaptive color and motion system
 */

// Core exports
export { default as ChernGauge } from './ChernGauge.svelte';
export { default as BerryPhaseCompass } from './BerryPhaseCompass.svelte';
export { default as StabilityMeter } from './StabilityMeter.svelte';
export { default as CascadePlayer } from './CascadePlayer.svelte';
export { default as OperatorFlow } from './OperatorFlow.svelte';
export { default as TopologyPlot } from './TopologyPlot.svelte';
export { default as VectorField } from './VectorField.svelte';
export { default as ConvergenceGraph } from './ConvergenceGraph.svelte';

// Color utilities
export * from './colors';

// Animation utilities
export * from './animations';
