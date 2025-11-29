//! Hamiltonian Simulation
//!
//! Algorithms for simulating quantum time evolution.
//!
//! ## Problem
//! Given H and t, compute U(t) = exp(-iHt)
//!
//! ## Methods
//! - Trotter-Suzuki decomposition (first, second, and fourth order)

use qops_circuits::{Circuit, QuantumRegister, Complex};
use crate::{AlgorithmError, Result, vqe::PauliSum};
use nalgebra::DMatrix;
use std::f64::consts::PI;
use serde::{Deserialize, Serialize};

/// Trotter decomposition order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrotterOrder {
    /// First-order: exp(-iHt) ≈ Π exp(-iH_k t)
    First,
    /// Second-order (symmetric): exp(-iHt) ≈ Π exp(-iH_k t/2) Π exp(-iH_k t/2)†
    Second,
    /// Fourth-order
    Fourth,
}

/// Trotter decomposition for Hamiltonian simulation
#[derive(Debug, Clone)]
pub struct TrotterDecomposition {
    /// Hamiltonian as sum of Pauli terms
    pub hamiltonian: PauliSum,
    /// Trotter order
    pub order: TrotterOrder,
    /// Number of Trotter steps
    pub steps: usize,
}

impl TrotterDecomposition {
    /// Create new Trotter decomposition
    pub fn new(hamiltonian: PauliSum, order: TrotterOrder, steps: usize) -> Self {
        Self { hamiltonian, order, steps }
    }

    /// First-order Trotter decomposition
    pub fn first_order(hamiltonian: PauliSum, steps: usize) -> Self {
        Self::new(hamiltonian, TrotterOrder::First, steps)
    }

    /// Second-order Trotter decomposition
    pub fn second_order(hamiltonian: PauliSum, steps: usize) -> Self {
        Self::new(hamiltonian, TrotterOrder::Second, steps)
    }

    /// Build circuit for time evolution
    pub fn build_circuit(&self, time: f64) -> Circuit {
        let n = self.hamiltonian.num_qubits();
        let dt = time / self.steps as f64;

        let mut circuit = Circuit::with_name(n, &format!("Trotter_t={:.4}", time));

        for _ in 0..self.steps {
            match self.order {
                TrotterOrder::First => {
                    circuit = self.add_first_order_step(circuit, dt);
                }
                TrotterOrder::Second => {
                    circuit = self.add_second_order_step(circuit, dt);
                }
                TrotterOrder::Fourth => {
                    circuit = self.add_fourth_order_step(circuit, dt);
                }
            }
        }

        circuit
    }

    /// Add first-order Trotter step
    fn add_first_order_step(&self, mut circuit: Circuit, dt: f64) -> Circuit {
        for term in &self.hamiltonian.terms {
            circuit = self.add_pauli_rotation(circuit, &term.pauli, term.coefficient * dt);
        }
        circuit
    }

    /// Add second-order Trotter step
    fn add_second_order_step(&self, mut circuit: Circuit, dt: f64) -> Circuit {
        // First half
        for term in &self.hamiltonian.terms {
            circuit = self.add_pauli_rotation(circuit, &term.pauli, term.coefficient * dt / 2.0);
        }

        // Second half (reverse order)
        for term in self.hamiltonian.terms.iter().rev() {
            circuit = self.add_pauli_rotation(circuit, &term.pauli, term.coefficient * dt / 2.0);
        }

        circuit
    }

    /// Add fourth-order Trotter step (using Suzuki's formula)
    fn add_fourth_order_step(&self, mut circuit: Circuit, dt: f64) -> Circuit {
        let p = 1.0 / (4.0 - 4.0_f64.powf(1.0 / 3.0));

        // S2(p*dt) S2(p*dt) S2((1-4p)*dt) S2(p*dt) S2(p*dt)
        circuit = self.add_second_order_step(circuit, p * dt);
        circuit = self.add_second_order_step(circuit, p * dt);
        circuit = self.add_second_order_step(circuit, (1.0 - 4.0 * p) * dt);
        circuit = self.add_second_order_step(circuit, p * dt);
        circuit = self.add_second_order_step(circuit, p * dt);

        circuit
    }

    /// Add rotation for a Pauli string: exp(-iθP)
    fn add_pauli_rotation(&self, mut circuit: Circuit, pauli: &str, theta: f64) -> Circuit {
        let _n = pauli.len();

        // Find non-identity positions
        let non_identity: Vec<(usize, char)> = pauli.chars()
            .enumerate()
            .filter(|(_, c)| *c != 'I' && *c != 'i')
            .collect();

        if non_identity.is_empty() {
            // Global phase, ignore
            return circuit;
        }

        // Change basis for X and Y
        for &(i, p) in &non_identity {
            match p {
                'X' | 'x' => circuit = circuit.h(i),
                'Y' | 'y' => circuit = circuit.rx(PI / 2.0, i),
                _ => {}
            }
        }

        // CNOT ladder to compute parity
        if non_identity.len() > 1 {
            for w in non_identity.windows(2) {
                circuit = circuit.cnot(w[0].0, w[1].0);
            }
        }

        // Rz rotation on last qubit
        let last_qubit = non_identity.last().unwrap().0;
        circuit = circuit.rz(2.0 * theta, last_qubit);

        // Undo CNOT ladder
        if non_identity.len() > 1 {
            for w in non_identity.windows(2).rev() {
                circuit = circuit.cnot(w[0].0, w[1].0);
            }
        }

        // Undo basis change
        for &(i, p) in &non_identity {
            match p {
                'X' | 'x' => circuit = circuit.h(i),
                'Y' | 'y' => circuit = circuit.rx(-PI / 2.0, i),
                _ => {}
            }
        }

        circuit
    }

    /// Estimate error bound
    pub fn error_bound(&self, time: f64) -> f64 {
        let dt = time / self.steps as f64;
        let num_terms = self.hamiltonian.terms.len() as f64;

        // Rough error bounds (assuming unit-norm terms)
        match self.order {
            TrotterOrder::First => num_terms.powi(2) * dt.powi(2) * self.steps as f64,
            TrotterOrder::Second => num_terms.powi(3) * dt.powi(3) * self.steps as f64,
            TrotterOrder::Fourth => num_terms.powi(5) * dt.powi(5) * self.steps as f64,
        }
    }
}

/// Hamiltonian simulation using various methods
pub struct HamiltonianSimulation {
    /// Hamiltonian
    pub hamiltonian: PauliSum,
    /// Simulation method
    pub method: SimulationMethod,
}

/// Simulation method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationMethod {
    /// Trotter-Suzuki decomposition (first, second, or fourth order)
    Trotter(TrotterOrder),
}

impl HamiltonianSimulation {
    pub fn new(hamiltonian: PauliSum, method: SimulationMethod) -> Self {
        Self { hamiltonian, method }
    }

    /// Create using Trotter decomposition
    pub fn trotter(hamiltonian: PauliSum, order: TrotterOrder) -> Self {
        Self::new(hamiltonian, SimulationMethod::Trotter(order))
    }

    /// Simulate time evolution using the configured method
    pub fn evolve(&self, initial_state: &mut QuantumRegister, time: f64, steps: usize) -> Result<()> {
        match self.method {
            SimulationMethod::Trotter(order) => {
                let trotter = TrotterDecomposition::new(
                    self.hamiltonian.clone(),
                    order,
                    steps,
                );
                let circuit = trotter.build_circuit(time);
                initial_state.apply_circuit(&circuit)
                    .map_err(|e| AlgorithmError::CircuitError(e.to_string()))
            }
        }
    }

    /// Compute exact evolution operator (for small systems)
    pub fn exact_evolution(&self, time: f64) -> Result<DMatrix<Complex>> {
        let n = self.hamiltonian.num_qubits();
        let dim = 1 << n;

        // Build Hamiltonian matrix
        let h_matrix = self.build_hamiltonian_matrix()?;

        // Compute exp(-iHt) via eigendecomposition
        let eigen = h_matrix.clone().symmetric_eigen();
        let eigenvalues = eigen.eigenvalues;
        let eigenvectors = eigen.eigenvectors;

        // U = V exp(-iDt) V†
        let mut diag = DMatrix::zeros(dim, dim);
        for i in 0..dim {
            let phase = Complex::from_polar(1.0, -eigenvalues[i] * time);
            diag[(i, i)] = phase;
        }

        // Convert eigenvectors to complex
        let v_complex: DMatrix<Complex> = DMatrix::from_fn(dim, dim, |i, j| {
            Complex::new(eigenvectors[(i, j)], 0.0)
        });

        Ok(&v_complex * &diag * v_complex.adjoint())
    }

    /// Build Hamiltonian matrix
    fn build_hamiltonian_matrix(&self) -> Result<DMatrix<f64>> {
        let n = self.hamiltonian.num_qubits();
        let dim = 1 << n;
        let mut matrix = DMatrix::zeros(dim, dim);

        for term in &self.hamiltonian.terms {
            let pauli_matrix = Self::pauli_string_matrix(&term.pauli)?;
            matrix += term.coefficient * pauli_matrix;
        }

        Ok(matrix)
    }

    /// Build matrix for a Pauli string
    fn pauli_string_matrix(pauli: &str) -> Result<DMatrix<f64>> {
        let i_mat = DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, 1.0]);
        let x_mat = DMatrix::from_row_slice(2, 2, &[0.0, 1.0, 1.0, 0.0]);
        let _y_mat_real = DMatrix::from_row_slice(2, 2, &[0.0, 0.0, 0.0, 0.0]); // Y has imaginary entries
        let z_mat = DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, -1.0]);

        let mut result: Option<DMatrix<f64>> = None;

        for c in pauli.chars() {
            let mat = match c {
                'I' | 'i' => &i_mat,
                'X' | 'x' => &x_mat,
                'Z' | 'z' => &z_mat,
                'Y' | 'y' => {
                    return Err(AlgorithmError::InvalidParameter(
                        "Y Pauli requires complex matrix handling".to_string()
                    ));
                }
                _ => return Err(AlgorithmError::InvalidParameter(
                    format!("Invalid Pauli character: {}", c)
                )),
            };

            result = Some(match result {
                None => mat.clone(),
                Some(r) => r.kronecker(mat),
            });
        }

        result.ok_or_else(|| AlgorithmError::InvalidParameter("Empty Pauli string".to_string()))
    }
}

/// Quantum walk Hamiltonian
pub struct QuantumWalkHamiltonian {
    /// Adjacency matrix of the graph
    pub adjacency: DMatrix<f64>,
    /// Hopping strength
    pub gamma: f64,
}

impl QuantumWalkHamiltonian {
    pub fn new(adjacency: DMatrix<f64>, gamma: f64) -> Self {
        Self { adjacency, gamma }
    }

    /// Create from edge list
    pub fn from_edges(num_vertices: usize, edges: &[(usize, usize)]) -> Self {
        let mut adj = DMatrix::zeros(num_vertices, num_vertices);
        for &(i, j) in edges {
            adj[(i, j)] = 1.0;
            adj[(j, i)] = 1.0;
        }
        Self::new(adj, 1.0)
    }

    /// Convert to Pauli Hamiltonian (for simulation)
    pub fn to_pauli_sum(&self) -> PauliSum {
        let n = (self.adjacency.nrows() as f64).log2().ceil() as usize;
        let mut hamiltonian = PauliSum::new();

        // H = -γ A (negative adjacency as Hamiltonian)
        // This is approximate - full implementation needs proper encoding

        // Add ZZ terms for edges
        for i in 0..self.adjacency.nrows() {
            for j in (i + 1)..self.adjacency.ncols() {
                if self.adjacency[(i, j)].abs() > 1e-10 {
                    let mut pauli = vec!['I'; n];
                    if i < n && j < n {
                        pauli[i] = 'Z';
                        pauli[j] = 'Z';
                        hamiltonian.add_term(
                            -self.gamma * self.adjacency[(i, j)],
                            &pauli.iter().collect::<String>()
                        );
                    }
                }
            }
        }

        hamiltonian
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trotter_circuit() {
        let mut hamiltonian = PauliSum::new();
        hamiltonian.add_term(1.0, "ZZ");
        hamiltonian.add_term(0.5, "XI");
        hamiltonian.add_term(0.5, "IX");

        let trotter = TrotterDecomposition::first_order(hamiltonian, 2);
        let circuit = trotter.build_circuit(1.0);

        assert_eq!(circuit.num_qubits, 2);
        assert!(circuit.gate_count() > 0);
    }

    #[test]
    fn test_second_order_trotter() {
        let hamiltonian = PauliSum::transverse_ising(2, 1.0, 0.5);
        let trotter = TrotterDecomposition::second_order(hamiltonian, 4);

        let circuit = trotter.build_circuit(0.5);
        assert!(circuit.gate_count() > 0);
    }

    #[test]
    fn test_error_bound() {
        let hamiltonian = PauliSum::heisenberg(3, 1.0);
        let trotter = TrotterDecomposition::first_order(hamiltonian, 10);

        let error = trotter.error_bound(1.0);
        assert!(error > 0.0);
    }

    #[test]
    fn test_quantum_walk_hamiltonian() {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let walk = QuantumWalkHamiltonian::from_edges(4, &edges);

        assert_eq!(walk.adjacency.nrows(), 4);
        assert_eq!(walk.adjacency[(0, 1)], 1.0);
    }
}
