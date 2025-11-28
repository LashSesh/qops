//! Quantum Approximate Optimization Algorithm (QAOA)
//!
//! Variational algorithm for combinatorial optimization problems.
//!
//! ## Algorithm
//! |ψ(β,γ)⟩ = U_B(β_p)U_C(γ_p)...U_B(β_1)U_C(γ_1)|+⟩^n
//!
//! where:
//! - U_C(γ) = exp(-iγC) is the cost/problem unitary
//! - U_B(β) = exp(-iβB) is the mixer unitary

use qops_circuits::{Circuit, Gate, QuantumRegister, Measurement};
use crate::{AlgorithmError, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// QAOA configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAOAConfig {
    /// Number of qubits
    pub num_qubits: usize,
    /// Number of QAOA layers (p)
    pub layers: usize,
    /// Maximum optimization iterations
    pub max_iterations: usize,
    /// Number of measurement shots
    pub shots: usize,
    /// Mixer type
    pub mixer: MixerType,
}

impl Default for QAOAConfig {
    fn default() -> Self {
        Self {
            num_qubits: 4,
            layers: 2,
            max_iterations: 100,
            shots: 1024,
            mixer: MixerType::X,
        }
    }
}

/// Mixer unitary type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MixerType {
    /// Standard X mixer: exp(-iβ Σ X_i)
    X,
    /// XY mixer for constrained problems
    XY,
    /// Grover mixer
    Grover,
}

/// Cost function definition
#[derive(Debug, Clone)]
pub enum CostFunction {
    /// MaxCut problem
    MaxCut(Vec<(usize, usize, f64)>),  // (i, j, weight)
    /// Minimum vertex cover
    VertexCover(Vec<(usize, usize)>, f64),  // edges, penalty
    /// Traveling salesman (QUBO formulation)
    TSP(Vec<Vec<f64>>),  // distance matrix
    /// Generic QUBO: minimize x^T Q x
    QUBO(Vec<Vec<f64>>),
    /// Custom Ising model
    Ising(Vec<f64>, Vec<(usize, usize, f64)>),  // h_i, J_ij
}

impl CostFunction {
    /// Evaluate cost for a binary string
    pub fn evaluate(&self, bitstring: &[bool]) -> f64 {
        match self {
            CostFunction::MaxCut(edges) => {
                edges.iter()
                    .filter(|(i, j, _)| bitstring[*i] != bitstring[*j])
                    .map(|(_, _, w)| w)
                    .sum()
            }

            CostFunction::VertexCover(edges, penalty) => {
                let num_vertices = bitstring.len();
                let mut cost = 0.0;

                // Count selected vertices
                for &b in bitstring {
                    if b {
                        cost += 1.0;
                    }
                }

                // Add penalty for uncovered edges
                for &(i, j) in edges {
                    if !bitstring[i] && !bitstring[j] {
                        cost += penalty;
                    }
                }

                cost
            }

            CostFunction::QUBO(q) => {
                let n = bitstring.len();
                let mut cost = 0.0;

                for i in 0..n {
                    if bitstring[i] {
                        cost += q[i][i];
                        for j in (i+1)..n {
                            if bitstring[j] {
                                cost += q[i][j] + q[j][i];
                            }
                        }
                    }
                }

                cost
            }

            CostFunction::Ising(h, j_couplings) => {
                let n = bitstring.len();
                let mut cost = 0.0;

                // Local fields: h_i * s_i (s_i = 2*x_i - 1)
                for i in 0..n {
                    let s_i = if bitstring[i] { 1.0 } else { -1.0 };
                    cost += h[i] * s_i;
                }

                // Couplings: J_ij * s_i * s_j
                for &(i, j, j_ij) in j_couplings {
                    let s_i = if bitstring[i] { 1.0 } else { -1.0 };
                    let s_j = if bitstring[j] { 1.0 } else { -1.0 };
                    cost += j_ij * s_i * s_j;
                }

                cost
            }

            CostFunction::TSP(distances) => {
                // Simplified TSP cost (assumes proper encoding)
                let n = (bitstring.len() as f64).sqrt() as usize;
                let mut cost = 0.0;

                // This would need proper decoding of TSP solution
                // Placeholder implementation
                for i in 0..n-1 {
                    cost += distances[i][i+1];
                }
                cost += distances[n-1][0];

                cost
            }
        }
    }

    /// Number of qubits needed
    pub fn num_qubits(&self) -> usize {
        match self {
            CostFunction::MaxCut(edges) => {
                edges.iter().flat_map(|(i, j, _)| [*i, *j]).max().unwrap_or(0) + 1
            }
            CostFunction::VertexCover(edges, _) => {
                edges.iter().flat_map(|(i, j)| [*i, *j]).max().unwrap_or(0) + 1
            }
            CostFunction::QUBO(q) => q.len(),
            CostFunction::Ising(h, _) => h.len(),
            CostFunction::TSP(d) => d.len() * d.len(),
        }
    }
}

/// QAOA result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAOAResult {
    /// Best solution found (as bitstring)
    pub best_solution: Vec<bool>,
    /// Best cost value
    pub best_cost: f64,
    /// Optimal parameters
    pub optimal_params: QAOAParams,
    /// Approximation ratio (if optimal known)
    pub approximation_ratio: Option<f64>,
    /// Energy history during optimization
    pub energy_history: Vec<f64>,
    /// Solution counts
    pub solution_counts: std::collections::HashMap<String, usize>,
}

/// QAOA parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAOAParams {
    /// Gamma parameters (one per layer)
    pub gamma: Vec<f64>,
    /// Beta parameters (one per layer)
    pub beta: Vec<f64>,
}

impl QAOAParams {
    pub fn random(layers: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            gamma: (0..layers).map(|_| rng.gen_range(0.0..2.0 * PI)).collect(),
            beta: (0..layers).map(|_| rng.gen_range(0.0..PI)).collect(),
        }
    }

    pub fn zeros(layers: usize) -> Self {
        Self {
            gamma: vec![0.0; layers],
            beta: vec![0.0; layers],
        }
    }

    pub fn to_vec(&self) -> Vec<f64> {
        self.gamma.iter().chain(self.beta.iter()).copied().collect()
    }

    pub fn from_vec(vec: &[f64], layers: usize) -> Self {
        Self {
            gamma: vec[..layers].to_vec(),
            beta: vec[layers..].to_vec(),
        }
    }
}

/// QAOA algorithm implementation
pub struct QAOA {
    /// Configuration
    pub config: QAOAConfig,
    /// Cost function
    pub cost_function: CostFunction,
}

impl QAOA {
    /// Create new QAOA instance
    pub fn new(config: QAOAConfig, cost_function: CostFunction) -> Self {
        Self { config, cost_function }
    }

    /// Create QAOA for MaxCut problem
    pub fn max_cut(edges: Vec<(usize, usize)>, layers: usize) -> Self {
        let weighted_edges: Vec<(usize, usize, f64)> = edges.into_iter()
            .map(|(i, j)| (i, j, 1.0))
            .collect();

        let num_qubits = weighted_edges.iter()
            .flat_map(|(i, j, _)| [*i, *j])
            .max()
            .unwrap_or(0) + 1;

        let config = QAOAConfig {
            num_qubits,
            layers,
            ..Default::default()
        };

        Self {
            config,
            cost_function: CostFunction::MaxCut(weighted_edges),
        }
    }

    /// Build cost unitary circuit
    fn build_cost_unitary(&self, gamma: f64) -> Circuit {
        let n = self.config.num_qubits;
        let mut circuit = Circuit::with_name(n, "U_C");

        match &self.cost_function {
            CostFunction::MaxCut(edges) => {
                for &(i, j, w) in edges {
                    // ZZ interaction: exp(-iγw Z_i Z_j)
                    circuit = circuit.cnot(i, j);
                    circuit = circuit.rz(2.0 * gamma * w, j);
                    circuit = circuit.cnot(i, j);
                }
            }

            CostFunction::Ising(h, j_couplings) => {
                // Local fields
                for i in 0..n {
                    circuit = circuit.rz(2.0 * gamma * h[i], i);
                }

                // Couplings
                for &(i, j, j_ij) in j_couplings {
                    circuit = circuit.cnot(i, j);
                    circuit = circuit.rz(2.0 * gamma * j_ij, j);
                    circuit = circuit.cnot(i, j);
                }
            }

            _ => {
                // Generic QUBO encoding
                // This is a placeholder - full implementation would need proper encoding
            }
        }

        circuit
    }

    /// Build mixer unitary circuit
    fn build_mixer_unitary(&self, beta: f64) -> Circuit {
        let n = self.config.num_qubits;
        let mut circuit = Circuit::with_name(n, "U_B");

        match self.config.mixer {
            MixerType::X => {
                // Standard X mixer: exp(-iβ Σ X_i) = ⊗_i Rx(2β)
                for i in 0..n {
                    circuit = circuit.rx(2.0 * beta, i);
                }
            }

            MixerType::XY => {
                // XY mixer for constrained problems
                for i in 0..n - 1 {
                    // XX + YY interaction
                    circuit = circuit.h(i);
                    circuit = circuit.h(i + 1);
                    circuit = circuit.cnot(i, i + 1);
                    circuit = circuit.rz(beta, i + 1);
                    circuit = circuit.cnot(i, i + 1);
                    circuit = circuit.h(i);
                    circuit = circuit.h(i + 1);

                    circuit = circuit.rx(PI / 2.0, i);
                    circuit = circuit.rx(PI / 2.0, i + 1);
                    circuit = circuit.cnot(i, i + 1);
                    circuit = circuit.rz(beta, i + 1);
                    circuit = circuit.cnot(i, i + 1);
                    circuit = circuit.rx(-PI / 2.0, i);
                    circuit = circuit.rx(-PI / 2.0, i + 1);
                }
            }

            MixerType::Grover => {
                // Grover mixer: 2|s⟩⟨s| - I
                for i in 0..n {
                    circuit = circuit.h(i);
                    circuit = circuit.x(i);
                }

                // Multi-controlled Z
                if n > 1 {
                    circuit = circuit.h(n - 1);
                    for i in 0..n - 1 {
                        circuit = circuit.cnot(i, n - 1);
                    }
                    circuit = circuit.h(n - 1);
                }

                for i in 0..n {
                    circuit = circuit.x(i);
                    circuit = circuit.h(i);
                }
            }
        }

        circuit
    }

    /// Build complete QAOA circuit
    pub fn build_circuit(&self, params: &QAOAParams) -> Circuit {
        let n = self.config.num_qubits;
        let mut circuit = Circuit::with_name(n, "QAOA");

        // Initial state: |+⟩^n
        for i in 0..n {
            circuit = circuit.h(i);
        }

        // QAOA layers
        for layer in 0..self.config.layers {
            let cost_circuit = self.build_cost_unitary(params.gamma[layer]);
            let mixer_circuit = self.build_mixer_unitary(params.beta[layer]);

            circuit.append(&cost_circuit).ok();
            circuit.append(&mixer_circuit).ok();
        }

        circuit
    }

    /// Evaluate expected cost for given parameters
    pub fn evaluate(&self, params: &QAOAParams) -> f64 {
        let circuit = self.build_circuit(params);
        let mut register = QuantumRegister::new(self.config.num_qubits);
        register.apply_circuit(&circuit).ok();

        let counts = register.get_counts(self.config.shots);

        // Compute expected cost
        let mut total_cost = 0.0;
        for (bitstring, count) in &counts {
            let bits: Vec<bool> = bitstring.chars().map(|c| c == '1').collect();
            let cost = self.cost_function.evaluate(&bits);
            total_cost += cost * (*count as f64);
        }

        total_cost / self.config.shots as f64
    }

    /// Run QAOA optimization
    pub fn run(&self) -> QAOAResult {
        let mut best_params = QAOAParams::random(self.config.layers);
        let mut best_cost = self.evaluate(&best_params);
        let mut energy_history = vec![best_cost];

        // Simple grid search + local optimization
        // For production, use proper optimizer (COBYLA, Nelder-Mead, etc.)

        let mut rng = rand::thread_rng();

        for _ in 0..self.config.max_iterations {
            // Perturb parameters
            let trial_params = QAOAParams {
                gamma: best_params.gamma.iter()
                    .map(|&g| g + rng.gen_range(-0.2..0.2))
                    .collect(),
                beta: best_params.beta.iter()
                    .map(|&b| b + rng.gen_range(-0.1..0.1))
                    .collect(),
            };

            let cost = self.evaluate(&trial_params);
            energy_history.push(cost);

            // For MaxCut, we want to maximize (minimize negative)
            let is_better = match &self.cost_function {
                CostFunction::MaxCut(_) => cost > best_cost,
                _ => cost < best_cost,
            };

            if is_better {
                best_cost = cost;
                best_params = trial_params;
            }
        }

        // Final measurement to get solution distribution
        let circuit = self.build_circuit(&best_params);
        let mut register = QuantumRegister::new(self.config.num_qubits);
        register.apply_circuit(&circuit).ok();

        let counts = register.get_counts(self.config.shots);

        // Find best solution
        let (best_bitstring, _) = counts.iter()
            .max_by(|(a_str, a_count), (b_str, b_count)| {
                let a_bits: Vec<bool> = a_str.chars().map(|c| c == '1').collect();
                let b_bits: Vec<bool> = b_str.chars().map(|c| c == '1').collect();
                let a_cost = self.cost_function.evaluate(&a_bits);
                let b_cost = self.cost_function.evaluate(&b_bits);

                match &self.cost_function {
                    CostFunction::MaxCut(_) => a_cost.partial_cmp(&b_cost).unwrap(),
                    _ => b_cost.partial_cmp(&a_cost).unwrap(),
                }
            })
            .unwrap();

        let best_solution: Vec<bool> = best_bitstring.chars().map(|c| c == '1').collect();
        let actual_best_cost = self.cost_function.evaluate(&best_solution);

        QAOAResult {
            best_solution,
            best_cost: actual_best_cost,
            optimal_params: best_params,
            approximation_ratio: None,
            energy_history,
            solution_counts: counts,
        }
    }

    /// Compute approximation ratio (requires known optimal)
    pub fn approximation_ratio(&self, found: f64, optimal: f64) -> f64 {
        match &self.cost_function {
            CostFunction::MaxCut(_) => found / optimal,
            _ => optimal / found,
        }
    }
}

/// CVaR (Conditional Value at Risk) QAOA variant
pub struct CVaR_QAOA {
    /// Base QAOA
    pub qaoa: QAOA,
    /// CVaR parameter α ∈ (0, 1]
    pub alpha: f64,
}

impl CVaR_QAOA {
    pub fn new(qaoa: QAOA, alpha: f64) -> Self {
        Self { qaoa, alpha: alpha.clamp(0.01, 1.0) }
    }

    /// Evaluate CVaR objective
    pub fn evaluate(&self, params: &QAOAParams) -> f64 {
        let circuit = self.qaoa.build_circuit(params);
        let mut register = QuantumRegister::new(self.qaoa.config.num_qubits);
        register.apply_circuit(&circuit).ok();

        let counts = register.get_counts(self.qaoa.config.shots);

        // Compute costs and sort
        let mut costs: Vec<(f64, usize)> = counts.iter()
            .map(|(bitstring, &count)| {
                let bits: Vec<bool> = bitstring.chars().map(|c| c == '1').collect();
                (self.qaoa.cost_function.evaluate(&bits), count)
            })
            .collect();

        // Sort by cost (ascending for minimization)
        costs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Take best α fraction
        let cutoff = (self.alpha * self.qaoa.config.shots as f64) as usize;
        let mut total = 0.0;
        let mut count = 0usize;

        for (cost, c) in costs {
            let take = c.min(cutoff - count);
            total += cost * take as f64;
            count += take;
            if count >= cutoff {
                break;
            }
        }

        total / count as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maxcut_simple() {
        // Simple triangle graph
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let qaoa = QAOA::max_cut(edges, 1);

        let params = QAOAParams::random(1);
        let cost = qaoa.evaluate(&params);

        // MaxCut of triangle is 2
        assert!(cost >= 0.0 && cost <= 3.0);
    }

    #[test]
    fn test_qaoa_circuit() {
        let edges = vec![(0, 1)];
        let qaoa = QAOA::max_cut(edges, 2);

        let params = QAOAParams::zeros(2);
        let circuit = qaoa.build_circuit(&params);

        assert_eq!(circuit.num_qubits, 2);
    }

    #[test]
    fn test_cost_function_evaluation() {
        let cost = CostFunction::MaxCut(vec![(0, 1, 1.0), (1, 2, 1.0)]);

        // 010 has both edges cut
        assert_eq!(cost.evaluate(&[false, true, false]), 2.0);

        // 000 has no edges cut
        assert_eq!(cost.evaluate(&[false, false, false]), 0.0);
    }

    #[test]
    fn test_ising_cost() {
        let h = vec![0.5, -0.5];
        let j = vec![(0, 1, 1.0)];
        let cost = CostFunction::Ising(h, j);

        // Evaluate some configurations
        let _ = cost.evaluate(&[true, true]);
        let _ = cost.evaluate(&[true, false]);
    }
}
