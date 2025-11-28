//! Quantum Gates - Universal Gate Set
//!
//! This module provides a comprehensive set of quantum gates for circuit construction.
//!
//! ## Single-Qubit Gates
//! - Pauli gates: X, Y, Z
//! - Hadamard: H
//! - Phase gates: S, T, Rz
//! - Rotation gates: Rx, Ry, Rz
//!
//! ## Two-Qubit Gates
//! - CNOT (CX)
//! - CZ, CY
//! - SWAP
//! - iSWAP
//!
//! ## Three-Qubit Gates
//! - Toffoli (CCX)
//! - Fredkin (CSWAP)

use crate::{Complex, ZERO, ONE, I, FRAC_1_SQRT_2};
use nalgebra::{DMatrix, Matrix2, Matrix4};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Gate type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateType {
    // Single-qubit gates
    Identity,
    PauliX,
    PauliY,
    PauliZ,
    Hadamard,
    Phase,      // S gate
    PhaseDag,   // S†
    TGate,
    TGateDag,   // T†
    SqrtX,      // √X
    SqrtXDag,   // √X†

    // Parameterized single-qubit gates
    Rx,
    Ry,
    Rz,
    U1,
    U2,
    U3,

    // Two-qubit gates
    CNOT,
    CZ,
    CY,
    SWAP,
    ISWAP,
    SqrtSWAP,

    // Controlled parameterized gates
    CRx,
    CRy,
    CRz,
    CPhase,

    // Three-qubit gates
    Toffoli,
    Fredkin,

    // Custom gate
    Custom,
}

/// A quantum gate with its matrix representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gate {
    /// Gate type
    pub gate_type: GateType,
    /// Gate name
    pub name: String,
    /// Number of qubits the gate acts on
    pub num_qubits: usize,
    /// Optional parameter (for Rx, Ry, Rz, etc.)
    pub parameter: Option<f64>,
    /// Gate matrix (flattened, row-major)
    matrix_data: Vec<(f64, f64)>, // (re, im) pairs
    matrix_rows: usize,
}

impl Gate {
    /// Create a custom gate from a matrix
    pub fn custom(name: &str, matrix: DMatrix<Complex>) -> Self {
        let n = matrix.nrows();
        let num_qubits = (n as f64).log2() as usize;

        let matrix_data: Vec<(f64, f64)> = matrix.iter()
            .map(|c| (c.re, c.im))
            .collect();

        Self {
            gate_type: GateType::Custom,
            name: name.to_string(),
            num_qubits,
            parameter: None,
            matrix_data,
            matrix_rows: n,
        }
    }

    /// Get the gate matrix
    pub fn matrix(&self) -> DMatrix<Complex> {
        let n = self.matrix_rows;
        let data: Vec<Complex> = self.matrix_data.iter()
            .map(|(re, im)| Complex::new(*re, *im))
            .collect();
        DMatrix::from_row_slice(n, n, &data)
    }

    /// Get the gate matrix as a 2x2 matrix (for single-qubit gates)
    pub fn matrix_2x2(&self) -> Option<Matrix2<Complex>> {
        if self.num_qubits != 1 {
            return None;
        }
        let m = self.matrix();
        Some(Matrix2::new(m[(0, 0)], m[(0, 1)], m[(1, 0)], m[(1, 1)]))
    }

    /// Get the gate matrix as a 4x4 matrix (for two-qubit gates)
    pub fn matrix_4x4(&self) -> Option<Matrix4<Complex>> {
        if self.num_qubits != 2 {
            return None;
        }
        let m = self.matrix();
        Some(Matrix4::from_fn(|i, j| m[(i, j)]))
    }

    /// Get the adjoint (conjugate transpose) of the gate
    pub fn adjoint(&self) -> Self {
        let m = self.matrix();
        let adj = m.adjoint();
        let matrix_data: Vec<(f64, f64)> = adj.iter()
            .map(|c| (c.re, c.im))
            .collect();

        Self {
            gate_type: self.gate_type,
            name: format!("{}†", self.name),
            num_qubits: self.num_qubits,
            parameter: self.parameter,
            matrix_data,
            matrix_rows: self.matrix_rows,
        }
    }

    /// Check if the gate is unitary
    pub fn is_unitary(&self) -> bool {
        let m = self.matrix();
        let product = &m * m.adjoint();
        let identity = DMatrix::identity(m.nrows(), m.ncols());

        (product - identity).norm() < 1e-10
    }

    // ==================== Single-Qubit Gates ====================

    /// Identity gate I
    pub fn identity() -> Self {
        Self::from_matrix_2x2(GateType::Identity, "I", Matrix2::identity())
    }

    /// Pauli-X gate (NOT gate, bit flip)
    /// |0⟩ ↔ |1⟩
    pub fn x() -> Self {
        Self::from_matrix_2x2(
            GateType::PauliX,
            "X",
            Matrix2::new(ZERO, ONE, ONE, ZERO),
        )
    }

    /// Pauli-Y gate
    pub fn y() -> Self {
        Self::from_matrix_2x2(
            GateType::PauliY,
            "Y",
            Matrix2::new(ZERO, -I, I, ZERO),
        )
    }

    /// Pauli-Z gate (phase flip)
    /// |0⟩ → |0⟩, |1⟩ → -|1⟩
    pub fn z() -> Self {
        Self::from_matrix_2x2(
            GateType::PauliZ,
            "Z",
            Matrix2::new(ONE, ZERO, ZERO, -ONE),
        )
    }

    /// Hadamard gate
    /// Creates superposition: |0⟩ → |+⟩, |1⟩ → |−⟩
    pub fn h() -> Self {
        let h = Complex::new(FRAC_1_SQRT_2, 0.0);
        Self::from_matrix_2x2(
            GateType::Hadamard,
            "H",
            Matrix2::new(h, h, h, -h),
        )
    }

    /// S gate (Phase gate, √Z)
    /// |0⟩ → |0⟩, |1⟩ → i|1⟩
    pub fn s() -> Self {
        Self::from_matrix_2x2(
            GateType::Phase,
            "S",
            Matrix2::new(ONE, ZERO, ZERO, I),
        )
    }

    /// S† gate (inverse of S)
    pub fn sdg() -> Self {
        Self::from_matrix_2x2(
            GateType::PhaseDag,
            "S†",
            Matrix2::new(ONE, ZERO, ZERO, -I),
        )
    }

    /// T gate (π/8 gate, √S)
    pub fn t() -> Self {
        let t_phase = Complex::from_polar(1.0, PI / 4.0);
        Self::from_matrix_2x2(
            GateType::TGate,
            "T",
            Matrix2::new(ONE, ZERO, ZERO, t_phase),
        )
    }

    /// T† gate (inverse of T)
    pub fn tdg() -> Self {
        let t_phase = Complex::from_polar(1.0, -PI / 4.0);
        Self::from_matrix_2x2(
            GateType::TGateDag,
            "T†",
            Matrix2::new(ONE, ZERO, ZERO, t_phase),
        )
    }

    /// √X gate (square root of X)
    pub fn sqrt_x() -> Self {
        let half = Complex::new(0.5, 0.0);
        let half_i = Complex::new(0.5, 0.5);
        let half_mi = Complex::new(0.5, -0.5);
        Self::from_matrix_2x2(
            GateType::SqrtX,
            "√X",
            Matrix2::new(half_i, half_mi, half_mi, half_i),
        )
    }

    /// Rotation around X-axis: Rx(θ) = exp(-iθX/2)
    pub fn rx(theta: f64) -> Self {
        let cos = Complex::new((theta / 2.0).cos(), 0.0);
        let sin = Complex::new(0.0, -(theta / 2.0).sin());
        let mut gate = Self::from_matrix_2x2(
            GateType::Rx,
            &format!("Rx({:.4})", theta),
            Matrix2::new(cos, sin, sin, cos),
        );
        gate.parameter = Some(theta);
        gate
    }

    /// Rotation around Y-axis: Ry(θ) = exp(-iθY/2)
    pub fn ry(theta: f64) -> Self {
        let cos = Complex::new((theta / 2.0).cos(), 0.0);
        let sin = Complex::new((theta / 2.0).sin(), 0.0);
        let mut gate = Self::from_matrix_2x2(
            GateType::Ry,
            &format!("Ry({:.4})", theta),
            Matrix2::new(cos, -sin, sin, cos),
        );
        gate.parameter = Some(theta);
        gate
    }

    /// Rotation around Z-axis: Rz(θ) = exp(-iθZ/2)
    pub fn rz(theta: f64) -> Self {
        let neg_phase = Complex::from_polar(1.0, -theta / 2.0);
        let pos_phase = Complex::from_polar(1.0, theta / 2.0);
        let mut gate = Self::from_matrix_2x2(
            GateType::Rz,
            &format!("Rz({:.4})", theta),
            Matrix2::new(neg_phase, ZERO, ZERO, pos_phase),
        );
        gate.parameter = Some(theta);
        gate
    }

    /// U1 gate (phase gate with parameter)
    /// U1(λ) = diag(1, e^{iλ})
    pub fn u1(lambda: f64) -> Self {
        let phase = Complex::from_polar(1.0, lambda);
        let mut gate = Self::from_matrix_2x2(
            GateType::U1,
            &format!("U1({:.4})", lambda),
            Matrix2::new(ONE, ZERO, ZERO, phase),
        );
        gate.parameter = Some(lambda);
        gate
    }

    /// U3 gate (general single-qubit unitary)
    /// U3(θ, φ, λ) = Rz(φ)·Ry(θ)·Rz(λ)
    pub fn u3(theta: f64, phi: f64, lambda: f64) -> Self {
        let cos = (theta / 2.0).cos();
        let sin = (theta / 2.0).sin();

        let u00 = Complex::new(cos, 0.0);
        let u01 = -Complex::from_polar(sin, lambda);
        let u10 = Complex::from_polar(sin, phi);
        let u11 = Complex::from_polar(cos, phi + lambda);

        Self::from_matrix_2x2(
            GateType::U3,
            &format!("U3({:.4},{:.4},{:.4})", theta, phi, lambda),
            Matrix2::new(u00, u01, u10, u11),
        )
    }

    // ==================== Two-Qubit Gates ====================

    /// CNOT gate (Controlled-X, CX)
    /// |00⟩ → |00⟩, |01⟩ → |01⟩, |10⟩ → |11⟩, |11⟩ → |10⟩
    pub fn cnot() -> Self {
        Self::from_matrix_4x4(
            GateType::CNOT,
            "CNOT",
            Matrix4::new(
                ONE, ZERO, ZERO, ZERO,
                ZERO, ONE, ZERO, ZERO,
                ZERO, ZERO, ZERO, ONE,
                ZERO, ZERO, ONE, ZERO,
            ),
        )
    }

    /// CZ gate (Controlled-Z)
    pub fn cz() -> Self {
        Self::from_matrix_4x4(
            GateType::CZ,
            "CZ",
            Matrix4::new(
                ONE, ZERO, ZERO, ZERO,
                ZERO, ONE, ZERO, ZERO,
                ZERO, ZERO, ONE, ZERO,
                ZERO, ZERO, ZERO, -ONE,
            ),
        )
    }

    /// CY gate (Controlled-Y)
    pub fn cy() -> Self {
        Self::from_matrix_4x4(
            GateType::CY,
            "CY",
            Matrix4::new(
                ONE, ZERO, ZERO, ZERO,
                ZERO, ONE, ZERO, ZERO,
                ZERO, ZERO, ZERO, -I,
                ZERO, ZERO, I, ZERO,
            ),
        )
    }

    /// SWAP gate
    /// |00⟩ → |00⟩, |01⟩ → |10⟩, |10⟩ → |01⟩, |11⟩ → |11⟩
    pub fn swap() -> Self {
        Self::from_matrix_4x4(
            GateType::SWAP,
            "SWAP",
            Matrix4::new(
                ONE, ZERO, ZERO, ZERO,
                ZERO, ZERO, ONE, ZERO,
                ZERO, ONE, ZERO, ZERO,
                ZERO, ZERO, ZERO, ONE,
            ),
        )
    }

    /// iSWAP gate
    pub fn iswap() -> Self {
        Self::from_matrix_4x4(
            GateType::ISWAP,
            "iSWAP",
            Matrix4::new(
                ONE, ZERO, ZERO, ZERO,
                ZERO, ZERO, I, ZERO,
                ZERO, I, ZERO, ZERO,
                ZERO, ZERO, ZERO, ONE,
            ),
        )
    }

    /// √SWAP gate
    pub fn sqrt_swap() -> Self {
        let half = Complex::new(0.5, 0.0);
        let half_i = Complex::new(0.5, 0.5);
        let half_mi = Complex::new(0.5, -0.5);
        Self::from_matrix_4x4(
            GateType::SqrtSWAP,
            "√SWAP",
            Matrix4::new(
                ONE, ZERO, ZERO, ZERO,
                ZERO, half_i, half_mi, ZERO,
                ZERO, half_mi, half_i, ZERO,
                ZERO, ZERO, ZERO, ONE,
            ),
        )
    }

    /// Controlled-Rz gate
    pub fn crz(theta: f64) -> Self {
        let neg_phase = Complex::from_polar(1.0, -theta / 2.0);
        let pos_phase = Complex::from_polar(1.0, theta / 2.0);
        let mut gate = Self::from_matrix_4x4(
            GateType::CRz,
            &format!("CRz({:.4})", theta),
            Matrix4::new(
                ONE, ZERO, ZERO, ZERO,
                ZERO, ONE, ZERO, ZERO,
                ZERO, ZERO, neg_phase, ZERO,
                ZERO, ZERO, ZERO, pos_phase,
            ),
        );
        gate.parameter = Some(theta);
        gate
    }

    /// Controlled-Phase gate (CPhase, CP)
    pub fn cphase(theta: f64) -> Self {
        let phase = Complex::from_polar(1.0, theta);
        let mut gate = Self::from_matrix_4x4(
            GateType::CPhase,
            &format!("CP({:.4})", theta),
            Matrix4::new(
                ONE, ZERO, ZERO, ZERO,
                ZERO, ONE, ZERO, ZERO,
                ZERO, ZERO, ONE, ZERO,
                ZERO, ZERO, ZERO, phase,
            ),
        );
        gate.parameter = Some(theta);
        gate
    }

    // ==================== Three-Qubit Gates ====================

    /// Toffoli gate (CCX, CCNOT)
    pub fn toffoli() -> Self {
        let mut matrix = DMatrix::identity(8, 8);
        // Swap |110⟩ (6) and |111⟩ (7)
        matrix[(6, 6)] = ZERO;
        matrix[(6, 7)] = ONE;
        matrix[(7, 6)] = ONE;
        matrix[(7, 7)] = ZERO;

        let matrix_data: Vec<(f64, f64)> = matrix.iter()
            .map(|c| (c.re, c.im))
            .collect();

        Self {
            gate_type: GateType::Toffoli,
            name: "Toffoli".to_string(),
            num_qubits: 3,
            parameter: None,
            matrix_data,
            matrix_rows: 8,
        }
    }

    /// Fredkin gate (CSWAP)
    pub fn fredkin() -> Self {
        let mut matrix = DMatrix::identity(8, 8);
        // Swap |101⟩ (5) and |110⟩ (6) when control is |1⟩
        matrix[(5, 5)] = ZERO;
        matrix[(5, 6)] = ONE;
        matrix[(6, 5)] = ONE;
        matrix[(6, 6)] = ZERO;

        let matrix_data: Vec<(f64, f64)> = matrix.iter()
            .map(|c| (c.re, c.im))
            .collect();

        Self {
            gate_type: GateType::Fredkin,
            name: "Fredkin".to_string(),
            num_qubits: 3,
            parameter: None,
            matrix_data,
            matrix_rows: 8,
        }
    }

    // ==================== Helper Methods ====================

    fn from_matrix_2x2(gate_type: GateType, name: &str, m: Matrix2<Complex>) -> Self {
        let matrix_data = vec![
            (m[(0, 0)].re, m[(0, 0)].im),
            (m[(0, 1)].re, m[(0, 1)].im),
            (m[(1, 0)].re, m[(1, 0)].im),
            (m[(1, 1)].re, m[(1, 1)].im),
        ];
        Self {
            gate_type,
            name: name.to_string(),
            num_qubits: 1,
            parameter: None,
            matrix_data,
            matrix_rows: 2,
        }
    }

    fn from_matrix_4x4(gate_type: GateType, name: &str, m: Matrix4<Complex>) -> Self {
        let mut matrix_data = Vec::with_capacity(16);
        for i in 0..4 {
            for j in 0..4 {
                matrix_data.push((m[(i, j)].re, m[(i, j)].im));
            }
        }
        Self {
            gate_type,
            name: name.to_string(),
            num_qubits: 2,
            parameter: None,
            matrix_data,
            matrix_rows: 4,
        }
    }
}

/// A controlled gate wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlledGate {
    /// The base gate to control
    pub base_gate: Gate,
    /// Number of control qubits
    pub num_controls: usize,
}

impl ControlledGate {
    /// Create a controlled version of any gate
    pub fn new(base_gate: Gate, num_controls: usize) -> Self {
        Self { base_gate, num_controls }
    }

    /// Get the full matrix for the controlled gate
    pub fn matrix(&self) -> DMatrix<Complex> {
        let base_matrix = self.base_gate.matrix();
        let base_dim = base_matrix.nrows();
        let total_dim = base_dim * (1 << self.num_controls);

        let mut full_matrix = DMatrix::identity(total_dim, total_dim);

        // The controlled gate acts only when all control qubits are |1⟩
        // This corresponds to the last block of the matrix
        let start = total_dim - base_dim;
        for i in 0..base_dim {
            for j in 0..base_dim {
                full_matrix[(start + i, start + j)] = base_matrix[(i, j)];
            }
        }

        full_matrix
    }
}

/// Parameterized gate for variational circuits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterizedGate {
    /// Gate type (must be Rx, Ry, Rz, U1, U2, U3)
    pub gate_type: GateType,
    /// Parameter name for symbolic reference
    pub param_name: String,
    /// Current parameter value
    pub value: f64,
}

impl ParameterizedGate {
    pub fn new(gate_type: GateType, param_name: &str, initial_value: f64) -> Self {
        Self {
            gate_type,
            param_name: param_name.to_string(),
            value: initial_value,
        }
    }

    /// Get the gate with current parameter value
    pub fn to_gate(&self) -> Gate {
        match self.gate_type {
            GateType::Rx => Gate::rx(self.value),
            GateType::Ry => Gate::ry(self.value),
            GateType::Rz => Gate::rz(self.value),
            GateType::U1 => Gate::u1(self.value),
            _ => Gate::identity(),
        }
    }

    /// Update the parameter value
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_pauli_gates() {
        // X² = I
        let x = Gate::x().matrix();
        let x_squared = &x * &x;
        assert!((x_squared - DMatrix::identity(2, 2)).norm() < 1e-10);

        // Y² = I
        let y = Gate::y().matrix();
        let y_squared = &y * &y;
        assert!((y_squared - DMatrix::identity(2, 2)).norm() < 1e-10);

        // Z² = I
        let z = Gate::z().matrix();
        let z_squared = &z * &z;
        assert!((z_squared - DMatrix::identity(2, 2)).norm() < 1e-10);
    }

    #[test]
    fn test_hadamard() {
        // H² = I
        let h = Gate::h().matrix();
        let h_squared = &h * &h;
        assert!((h_squared - DMatrix::identity(2, 2)).norm() < 1e-10);
    }

    #[test]
    fn test_unitarity() {
        assert!(Gate::x().is_unitary());
        assert!(Gate::y().is_unitary());
        assert!(Gate::z().is_unitary());
        assert!(Gate::h().is_unitary());
        assert!(Gate::s().is_unitary());
        assert!(Gate::t().is_unitary());
        assert!(Gate::cnot().is_unitary());
        assert!(Gate::toffoli().is_unitary());
    }

    #[test]
    fn test_rotation_gates() {
        // Rx(2π) = -I (up to global phase)
        let rx_2pi = Gate::rx(2.0 * PI);
        assert!(rx_2pi.is_unitary());

        // Rz(π) = Z (up to global phase)
        let rz_pi = Gate::rz(PI);
        assert!(rz_pi.is_unitary());
    }

    #[test]
    fn test_cnot() {
        let cnot = Gate::cnot();
        assert_eq!(cnot.num_qubits, 2);
        assert!(cnot.is_unitary());
    }
}
