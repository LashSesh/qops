//! Graph-based Hamiltonian construction.

use crate::graph::MetatronGraph;
use crate::state::QuantumState;
use nalgebra::DMatrix;
use num_complex::Complex64;

/// Metatron Hamiltonian from graph structure
#[derive(Debug, Clone)]
pub struct MetatronHamiltonian {
    /// Hamiltonian matrix
    matrix: DMatrix<Complex64>,
    /// Eigenvalues
    eigenvalues: Vec<f64>,
    /// Eigenvectors
    #[allow(dead_code)]
    eigenvectors: DMatrix<Complex64>,
}

impl MetatronHamiltonian {
    /// Create Hamiltonian from graph (negative Laplacian)
    pub fn from_graph(graph: &MetatronGraph) -> Self {
        let laplacian = graph.laplacian();
        let n = laplacian.nrows();

        // Convert to complex matrix (H = -L for quantum walk)
        let mut matrix = DMatrix::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                matrix[(i, j)] = Complex64::new(-laplacian[(i, j)], 0.0);
            }
        }

        // Compute eigendecomposition via real symmetric eigensolver
        // (Hamiltonian is Hermitian with real entries from Laplacian)
        let real_matrix: DMatrix<f64> = DMatrix::from_fn(n, n, |i, j| matrix[(i, j)].re);
        let eigen = real_matrix.symmetric_eigen();

        let eigenvalues = eigen.eigenvalues.iter().copied().collect();
        let eigenvectors = DMatrix::from_fn(n, n, |i, j| {
            Complex64::new(eigen.eigenvectors[(i, j)], 0.0)
        });

        Self {
            matrix,
            eigenvalues,
            eigenvectors,
        }
    }

    /// Get Hamiltonian matrix
    pub fn matrix(&self) -> &DMatrix<Complex64> {
        &self.matrix
    }

    /// Get dimension
    pub fn dimension(&self) -> usize {
        self.matrix.nrows()
    }

    /// Apply Hamiltonian to a state: H|ψ⟩
    pub fn apply(&self, state: &QuantumState) -> QuantumState {
        let result = &self.matrix * &state.amplitudes;
        QuantumState::new(result)
    }

    /// Compute ground state energy (minimum eigenvalue)
    pub fn ground_energy(&self) -> f64 {
        // Simplified: compute via power method inverse would be proper
        self.eigenvalues.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// Time evolution operator exp(-iHt)
    pub fn propagator(&self, t: f64) -> DMatrix<Complex64> {
        let n = self.matrix.nrows();
        let mut prop = DMatrix::zeros(n, n);

        // Simplified: for small t, use exp(-iHt) ≈ I - iHt
        // Proper implementation would use eigendecomposition
        for i in 0..n {
            prop[(i, i)] = Complex64::new(1.0, 0.0);
        }

        let i_unit = Complex64::new(0.0, -t);
        prop += &self.matrix * i_unit;

        prop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamiltonian_from_graph() {
        let graph = MetatronGraph::new();
        let h = MetatronHamiltonian::from_graph(&graph);

        assert_eq!(h.dimension(), 13);
    }

    #[test]
    fn test_hamiltonian_hermitian() {
        let graph = MetatronGraph::new();
        let h = MetatronHamiltonian::from_graph(&graph);

        // Check H = H†
        let n = h.dimension();
        for i in 0..n {
            for j in 0..n {
                let hij = h.matrix()[(i, j)];
                let hji = h.matrix()[(j, i)];
                assert!((hij - hji.conj()).norm() < 1e-10);
            }
        }
    }
}
