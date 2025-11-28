/**
 * Tauri command wrappers for QOPS
 *
 * This module provides TypeScript interfaces and wrapper functions
 * for all Tauri commands available in the QOPS desktop application.
 */

import { invoke } from '@tauri-apps/api/core';

// ============================================================================
// Type Definitions
// ============================================================================

export interface ComplexDto {
  re: number;
  im: number;
}

export interface GateDto {
  name: string;
  gate_type: string;
  qubits: number[];
  parameter: number | null;
}

export interface CircuitDto {
  id: string;
  name: string;
  qubits: number;
  depth: number;
  gate_count: number;
  gates: GateDto[];
}

export interface SimulationResultDto {
  probabilities: number[];
  counts: Record<string, number>;
  shots: number;
}

export interface GroverResultDto {
  measured_state: number;
  measured_state_binary: string;
  success_probability: number;
  iterations: number;
  is_solution: boolean;
  counts: Record<string, number>;
  theoretical_probability: number;
}

export interface ShorResultDto {
  success: boolean;
  number: number;
  factors: number[];
  period: number | null;
  attempts: number;
}

export interface QftResultDto {
  qubits: number;
  depth: number;
  gate_count: number;
  output_probabilities: number[];
}

export interface QpeResultDto {
  estimated_phase: number;
  true_phase: number;
  error: number;
  confidence: number;
}

export interface VqeResultDto {
  energy: number;
  iterations: number;
  converged: boolean;
  variance: number;
  parameters: number[];
}

export interface QaoaResultDto {
  best_solution: boolean[];
  best_cost: number;
  approximation_ratio: number;
  solution_counts: Record<string, number>;
}

export interface SignatureDto {
  psi: number;
  rho: number;
  omega: number;
  chi: number | null;
  eta: number | null;
}

export interface TopologyInfoDto {
  node_count: number;
  edge_count: number;
  topology_type: string;
}

export interface NodeDetailsDto {
  id: number;
  permutation: number[];
  signature: SignatureDto;
  neighbor_count: number;
}

export interface ArtefactDto {
  id: number;
  resonance: number;
  is_mandorla: boolean;
  node_path: number[];
}

export interface GenesisResultDto {
  artefacts: ArtefactDto[];
  best_resonance: number;
  mandorla_count: number;
  total_steps: number;
}

export interface ExtendedGenesisResultDto {
  artefacts: ArtefactDto[];
  best_resonance: number;
  mandorla_count: number;
  total_steps: number;
  families: FamilyDto[];
  triton_info: TritonInfoDto | null;
  stats: MiningStatsDto;
}

export interface FamilyDto {
  name: string;
  member_count: number;
  avg_resonance: number;
  characteristics: FamilyCharacteristicsDto;
}

export interface FamilyCharacteristicsDto {
  is_high_quality: boolean;
  is_stable: boolean;
  is_efficient: boolean;
}

export interface TritonInfoDto {
  best_score: number;
  iterations: number;
  converged: boolean;
}

export interface MiningStatsDto {
  avg_resonance: number;
  std_resonance: number;
  unique_nodes: number;
  duration_ms: number;
}

export interface Cube13MetricsDto {
  avg_resonance: number;
  center_resonance: number;
  hexagon_avg_resonance: number;
  cube_avg_resonance: number;
  coherence: number;
  embedding_count: number;
  coverage: number;
}

export interface Cube13NodeDto {
  id: number;
  node_type: string;
  centrality: number;
  neighbors: number[];
  embedding: SignatureDto | null;
}

export interface TopologyWalkDto {
  path: number[];
  coverage: number;
  final_node: number;
  steps_taken: number;
}

export interface SweepResultDto {
  total_configurations: number;
  best_config_index: number;
  best_score: number;
  evaluations: SweepEvaluationDto[];
}

export interface SweepEvaluationDto {
  config_index: number;
  parameters: Record<string, number>;
  final_score: number;
  convergence_rate: number;
}

export interface AutoTuneResultDto {
  best_temperature: number;
  best_cooling_rate: number;
  best_mandorla_threshold: number;
  achieved_resonance: number;
  iterations: number;
}

export interface QuantumWalkResultDto {
  time_points: number[];
  center_probabilities: number[];
  hex_probabilities: number[];
  cube_probabilities: number[];
}

export interface CalibrationStepDto {
  step: number;
  signature: SignatureDto;
  accepted: boolean;
  cri_triggered: boolean;
}

export interface CalibrationResultDto {
  steps: CalibrationStepDto[];
  final_signature: SignatureDto;
  accepted_count: number;
}

export interface ExperimentResultDto {
  name: string;
  total_runs: number;
  results: unknown[];
  summary: Record<string, number>;
}

export interface BenchmarkConfigDto {
  qubits: number;
  mean_time_ms: number;
  std_time_ms: number;
  success_rate: number;
}

export interface BenchmarkResultDto {
  algorithm: string;
  configurations: BenchmarkConfigDto[];
  comparison_table: string;
}

export interface SystemInfoDto {
  version: string;
  modules: string[];
  capabilities: string[];
}

// ============================================================================
// Circuit Commands
// ============================================================================

export async function createCircuit(qubits: number, name?: string): Promise<CircuitDto> {
  return invoke('create_circuit', { qubits, name });
}

export async function addGate(
  circuitId: string,
  gateType: string,
  qubits: number[],
  parameter?: number
): Promise<CircuitDto> {
  return invoke('add_gate', { circuitId, gateType, qubits, parameter });
}

export async function removeGate(circuitId: string, gateIndex: number): Promise<CircuitDto> {
  return invoke('remove_gate', { circuitId, gateIndex });
}

export async function simulateCircuit(circuitId: string, shots: number): Promise<SimulationResultDto> {
  return invoke('simulate_circuit', { circuitId, shots });
}

export async function getCircuit(circuitId: string): Promise<CircuitDto> {
  return invoke('get_circuit', { circuitId });
}

export async function getCircuitQasm(circuitId: string): Promise<string> {
  return invoke('get_circuit_qasm', { circuitId });
}

export async function deleteCircuit(circuitId: string): Promise<boolean> {
  return invoke('delete_circuit', { circuitId });
}

export async function listCircuits(): Promise<CircuitDto[]> {
  return invoke('list_circuits');
}

// ============================================================================
// Algorithm Commands
// ============================================================================

export async function runGrover(qubits: number, target: number, shots: number): Promise<GroverResultDto> {
  return invoke('run_grover', { qubits, target, shots });
}

export async function runShor(number: number): Promise<ShorResultDto> {
  return invoke('run_shor', { number });
}

export async function runQft(qubits: number, inputState?: number): Promise<QftResultDto> {
  return invoke('run_qft', { qubits, inputState });
}

export async function runIqft(qubits: number, inputState?: number): Promise<QftResultDto> {
  return invoke('run_iqft', { qubits, inputState });
}

export async function runQpe(precision: number, phase: number, shots: number): Promise<QpeResultDto> {
  return invoke('run_qpe', { precision, phase, shots });
}

export async function runVqe(qubits: number, layers: number, maxIterations: number): Promise<VqeResultDto> {
  return invoke('run_vqe', { qubits, layers, maxIterations });
}

export async function runQaoa(edges: [number, number][], layers: number, shots: number): Promise<QaoaResultDto> {
  return invoke('run_qaoa', { edges, layers, shots });
}

// ============================================================================
// Genesis Commands (TRITON-powered)
// ============================================================================

export async function runGenesisMining(
  agents: number,
  steps: number,
  strategy: string
): Promise<ExtendedGenesisResultDto> {
  return invoke('run_genesis_mining', { agents, steps, strategy });
}

export async function getGenesisFamilies(): Promise<FamilyDto[]> {
  return invoke('get_genesis_families');
}

export async function getS7TopologyInfo(): Promise<TopologyInfoDto> {
  return invoke('get_s7_topology_info');
}

export async function getNodeDetails(nodeId: number): Promise<NodeDetailsDto> {
  return invoke('get_node_details', { nodeId });
}

// ============================================================================
// Quantum Commands (Cube-13 Topology)
// ============================================================================

export async function runQuantumWalk(times: number[]): Promise<QuantumWalkResultDto> {
  return invoke('run_quantum_walk', { times });
}

export async function getCube13Info(): Promise<TopologyInfoDto> {
  return invoke('get_cube13_info');
}

export async function getCube13Metrics(): Promise<Cube13MetricsDto> {
  return invoke('get_cube13_metrics');
}

export async function getCube13Node(nodeId: number): Promise<Cube13NodeDto> {
  return invoke('get_cube13_node', { nodeId });
}

export async function runCube13Walk(start: number, steps: number): Promise<TopologyWalkDto> {
  return invoke('run_cube13_walk', { start, steps });
}

export async function embedInCube13(
  node: number,
  psi: number,
  rho: number,
  omega: number,
  chi: number,
  eta: number
): Promise<Cube13MetricsDto> {
  return invoke('embed_in_cube13', { node, psi, rho, omega, chi, eta });
}

export async function runCube13Vqe(layers: number): Promise<VqeResultDto> {
  return invoke('run_cube13_vqe', { layers });
}

// ============================================================================
// Research Commands
// ============================================================================

export async function runExperiment(
  algorithm: string,
  qubitRange: number[],
  repetitions: number,
  shots: number
): Promise<ExperimentResultDto> {
  return invoke('run_experiment', { algorithm, qubitRange, repetitions, shots });
}

export async function runBenchmark(
  algorithm: string,
  qubitCounts: number[],
  shots: number
): Promise<BenchmarkResultDto> {
  return invoke('run_benchmark', { algorithm, qubitCounts, shots });
}

// ============================================================================
// Calibration Commands (with Sweeps & Auto-Tuning)
// ============================================================================

export async function runCalibration(steps: number, target: number): Promise<CalibrationResultDto> {
  return invoke('run_calibration', { steps, target });
}

export async function getCalibrationStatus(): Promise<SignatureDto> {
  return invoke('get_calibration_status');
}

export async function runHyperparameterSweep(
  temperatureRange: number[],
  coolingRateRange: number[],
  stepsPerConfig: number
): Promise<SweepResultDto> {
  return invoke('run_hyperparameter_sweep', {
    temperatureRange,
    coolingRateRange,
    stepsPerConfig,
  });
}

export async function runAutoTune(
  targetResonance: number,
  maxIterations: number
): Promise<AutoTuneResultDto> {
  return invoke('run_auto_tune', { targetResonance, maxIterations });
}

// ============================================================================
// System Commands
// ============================================================================

export async function getSystemInfo(): Promise<SystemInfoDto> {
  return invoke('get_system_info');
}

// ============================================================================
// Hypercube Types
// ============================================================================

export interface Coord5DDto {
  psi: number;
  rho: number;
  omega: number;
  chi: number;
  eta: number;
}

export interface HypercubeVertexDto {
  id: string;
  coordinate: Coord5DDto;
  resonance: number;
  depth: number;
}

export interface HypercubeStatsDto {
  total_vertices: number;
  total_edges: number;
  max_depth_reached: number;
  best_resonance: number;
  avg_resonance: number;
}

export interface CompilationResultDto {
  output: Coord5DDto;
  resonance: number;
  iterations: number;
  threshold_met: boolean;
  artifact_count: number;
}

export interface HDAGNodeDto {
  id: string;
  name: string;
  node_type: string;
  input: Coord5DDto | null;
  output: Coord5DDto | null;
}

export interface HDAGEdgeDto {
  from: string;
  to: string;
  label: string | null;
  edge_type: string;
}

export interface HDAGInfoDto {
  name: string;
  nodes: HDAGNodeDto[];
  edges: HDAGEdgeDto[];
  execution_order: string[];
}

export interface HDAGExecutionResultDto {
  output: Coord5DDto;
  resonance: number;
  nodes_executed: number;
  nodes_failed: number;
  total_time_ms: number;
  artifact_count: number;
}

export interface HypercubeSessionResultDto {
  session_id: string;
  state: string;
  best_coordinate: Coord5DDto;
  best_resonance: number;
  compilation_result: CompilationResultDto | null;
  total_time_ms: number;
  expansion_steps: number;
  total_vertices: number;
  artifact_count: number;
}

export interface HypercubePresetDto {
  name: string;
  description: string;
  max_depth: number;
  expansion_rule: string;
}

// ============================================================================
// Slots Types
// ============================================================================

export interface MinedSequenceDto {
  id: string;
  symbols: string[];
  values: number[];
  resonance: number;
  coord5d: [number, number, number, number, number];
  depth: number;
}

export interface SlotsMiningResultDto {
  best_resonance: number;
  total_steps: number;
  steps_to_best: number;
  mining_time_ms: number;
  converged: boolean;
  top_sequences: MinedSequenceDto[];
}

export interface SlotsSessionResultDto {
  session_id: string;
  spin_count: number;
  best_resonance: number;
  best_sequence: MinedSequenceDto | null;
  mining_result: SlotsMiningResultDto | null;
  total_time_ms: number;
}

export interface SlotArtifactDto {
  id: string;
  name: string;
  coordinate: Coord5DDto;
  resonance: number;
  source_node: string | null;
}

export interface SlotsConfigDto {
  entropy_distribution: string;
  mining_strategy: string;
  mining_depth: number;
  target_resonance: number;
}

export interface MiningStrategyDto {
  name: string;
  description: string;
}

export interface EntropyDistributionDto {
  name: string;
  description: string;
}

// ============================================================================
// Hypercube Commands
// ============================================================================

export async function compileHypercube(
  seedPsi: number,
  seedRho: number,
  seedOmega: number,
  seedChi: number,
  seedEta: number,
  iterations: number,
  useTriton: boolean
): Promise<CompilationResultDto> {
  return invoke('compile_hypercube', {
    seedPsi,
    seedRho,
    seedOmega,
    seedChi,
    seedEta,
    iterations,
    useTriton,
  });
}

export async function expandCubeStep(
  currentVertices: number,
  expansionRule: string,
  iterations: number
): Promise<HypercubeStatsDto> {
  return invoke('expand_cube_step', { currentVertices, expansionRule, iterations });
}

export async function getHypercubeInfo(): Promise<Record<string, unknown>> {
  return invoke('get_hypercube_info');
}

export async function hdagExecute(
  pipelineType: string,
  seedPsi: number,
  seedRho: number,
  seedOmega: number,
  seedChi: number,
  seedEta: number
): Promise<HDAGExecutionResultDto> {
  return invoke('hdag_execute', {
    pipelineType,
    seedPsi,
    seedRho,
    seedOmega,
    seedChi,
    seedEta,
  });
}

export async function getHdagInfo(pipelineType: string): Promise<HDAGInfoDto> {
  return invoke('get_hdag_info', { pipelineType });
}

export async function runHypercubeSession(
  preset: string,
  seedPsi?: number,
  seedRho?: number,
  seedOmega?: number,
  seedChi?: number,
  seedEta?: number
): Promise<HypercubeSessionResultDto> {
  return invoke('run_hypercube_session', {
    preset,
    seedPsi,
    seedRho,
    seedOmega,
    seedChi,
    seedEta,
  });
}

export async function getHypercubePresets(): Promise<HypercubePresetDto[]> {
  return invoke('get_hypercube_presets');
}

// ============================================================================
// Slots Commands
// ============================================================================

export async function runSlotsEngine(
  steps: number,
  entropyDistribution: string,
  miningStrategy: string,
  targetResonance: number
): Promise<SlotsSessionResultDto> {
  return invoke('run_slots_engine', {
    steps,
    entropyDistribution,
    miningStrategy,
    targetResonance,
  });
}

export async function slotsMineSequence(
  depth: number,
  strategy: string,
  targetResonance: number,
  beamWidth?: number
): Promise<SlotsMiningResultDto> {
  return invoke('slots_mine_sequence', {
    depth,
    strategy,
    targetResonance,
    beamWidth,
  });
}

export async function getSlotsInfo(): Promise<Record<string, unknown>> {
  return invoke('get_slots_info');
}

export async function getMiningStrategies(): Promise<MiningStrategyDto[]> {
  return invoke('get_mining_strategies');
}

export async function getEntropyDistributions(): Promise<EntropyDistributionDto[]> {
  return invoke('get_entropy_distributions');
}

export async function slotsGenerateArtifacts(
  coordPsi: number,
  coordRho: number,
  coordOmega: number,
  coordChi: number,
  coordEta: number
): Promise<SlotArtifactDto[]> {
  return invoke('slots_generate_artifacts', {
    coordPsi,
    coordRho,
    coordOmega,
    coordChi,
    coordEta,
  });
}

export async function runHypercubeSlotsMode(
  coordPsi?: number,
  coordRho?: number,
  coordOmega?: number,
  coordChi?: number,
  coordEta?: number
): Promise<SlotArtifactDto[]> {
  return invoke('run_hypercube_slots_mode', {
    coordPsi,
    coordRho,
    coordOmega,
    coordChi,
    coordEta,
  });
}

export async function getSlotsConfigOptions(): Promise<SlotsConfigDto> {
  return invoke('get_slots_config_options');
}
