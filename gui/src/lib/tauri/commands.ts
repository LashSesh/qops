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
// Genesis Commands
// ============================================================================

export async function runGenesisMining(
  agents: number,
  steps: number,
  strategy: string
): Promise<GenesisResultDto> {
  return invoke('run_genesis_mining', { agents, steps, strategy });
}

export async function getS7TopologyInfo(): Promise<TopologyInfoDto> {
  return invoke('get_s7_topology_info');
}

export async function getNodeDetails(nodeId: number): Promise<NodeDetailsDto> {
  return invoke('get_node_details', { nodeId });
}

// ============================================================================
// Quantum Commands
// ============================================================================

export async function runQuantumWalk(times: number[]): Promise<QuantumWalkResultDto> {
  return invoke('run_quantum_walk', { times });
}

export async function getCube13Info(): Promise<TopologyInfoDto> {
  return invoke('get_cube13_info');
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
// Calibration Commands
// ============================================================================

export async function runCalibration(steps: number, target: number): Promise<CalibrationResultDto> {
  return invoke('run_calibration', { steps, target });
}

export async function getCalibrationStatus(): Promise<SignatureDto> {
  return invoke('get_calibration_status');
}

// ============================================================================
// System Commands
// ============================================================================

export async function getSystemInfo(): Promise<SystemInfoDto> {
  return invoke('get_system_info');
}
