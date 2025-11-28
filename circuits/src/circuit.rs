//! Quantum Circuit - Circuit construction and manipulation
//!
//! Provides a fluent API for building quantum circuits.

use crate::{Gate, Result, CircuitError};
use serde::{Deserialize, Serialize};

/// A single instruction in a quantum circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitInstruction {
    /// The gate to apply
    pub gate: Gate,
    /// Target qubit indices
    pub qubits: Vec<usize>,
    /// Optional classical condition
    pub condition: Option<ClassicalCondition>,
}

/// Classical condition for conditional gates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassicalCondition {
    /// Classical register index
    pub register: usize,
    /// Expected value
    pub value: usize,
}

/// A quantum circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    /// Number of qubits
    pub num_qubits: usize,
    /// Number of classical bits
    pub num_classical_bits: usize,
    /// Circuit instructions
    pub instructions: Vec<CircuitInstruction>,
    /// Circuit name
    pub name: String,
}

impl Circuit {
    /// Create a new circuit with n qubits
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            num_classical_bits: num_qubits,
            instructions: Vec::new(),
            name: String::from("circuit"),
        }
    }

    /// Create a named circuit
    pub fn with_name(num_qubits: usize, name: &str) -> Self {
        Self {
            num_qubits,
            num_classical_bits: num_qubits,
            instructions: Vec::new(),
            name: name.to_string(),
        }
    }

    /// Set the number of classical bits
    pub fn classical_bits(mut self, n: usize) -> Self {
        self.num_classical_bits = n;
        self
    }

    /// Get circuit depth (number of layers)
    pub fn depth(&self) -> usize {
        if self.instructions.is_empty() {
            return 0;
        }

        // Track when each qubit becomes free
        let mut qubit_layers = vec![0usize; self.num_qubits];

        for instruction in &self.instructions {
            // Find the maximum layer among all qubits this gate touches
            let max_layer = instruction.qubits.iter()
                .map(|&q| qubit_layers[q])
                .max()
                .unwrap_or(0);

            // Place the gate at max_layer + 1
            let gate_layer = max_layer + 1;

            // Update all touched qubits
            for &q in &instruction.qubits {
                qubit_layers[q] = gate_layer;
            }
        }

        qubit_layers.into_iter().max().unwrap_or(0)
    }

    /// Get total gate count
    pub fn gate_count(&self) -> usize {
        self.instructions.len()
    }

    /// Count gates by type
    pub fn gate_counts(&self) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();
        for instruction in &self.instructions {
            *counts.entry(instruction.gate.name.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// Add a gate to the circuit
    pub fn add_gate(&mut self, gate: Gate, qubits: Vec<usize>) -> Result<&mut Self> {
        for &q in &qubits {
            if q >= self.num_qubits {
                return Err(CircuitError::InvalidQubitIndex(q, self.num_qubits));
            }
        }
        self.instructions.push(CircuitInstruction {
            gate,
            qubits,
            condition: None,
        });
        Ok(self)
    }

    /// Add a conditional gate
    pub fn add_conditional_gate(
        &mut self,
        gate: Gate,
        qubits: Vec<usize>,
        condition: ClassicalCondition,
    ) -> Result<&mut Self> {
        for &q in &qubits {
            if q >= self.num_qubits {
                return Err(CircuitError::InvalidQubitIndex(q, self.num_qubits));
            }
        }
        self.instructions.push(CircuitInstruction {
            gate,
            qubits,
            condition: Some(condition),
        });
        Ok(self)
    }

    // ==================== Single-Qubit Gates ====================

    /// Apply Identity gate
    pub fn id(mut self, qubit: usize) -> Self {
        self.add_gate(Gate::identity(), vec![qubit]).ok();
        self
    }

    /// Apply Pauli-X gate
    pub fn x(mut self, qubit: usize) -> Self {
        self.add_gate(Gate::x(), vec![qubit]).ok();
        self
    }

    /// Apply Pauli-Y gate
    pub fn y(mut self, qubit: usize) -> Self {
        self.add_gate(Gate::y(), vec![qubit]).ok();
        self
    }

    /// Apply Pauli-Z gate
    pub fn z(mut self, qubit: usize) -> Self {
        self.add_gate(Gate::z(), vec![qubit]).ok();
        self
    }

    /// Apply Hadamard gate
    pub fn h(mut self, qubit: usize) -> Self {
        self.add_gate(Gate::h(), vec![qubit]).ok();
        self
    }

    /// Apply S gate
    pub fn s(mut self, qubit: usize) -> Self {
        self.add_gate(Gate::s(), vec![qubit]).ok();
        self
    }

    /// Apply S† gate
    pub fn sdg(mut self, qubit: usize) -> Self {
        self.add_gate(Gate::sdg(), vec![qubit]).ok();
        self
    }

    /// Apply T gate
    pub fn t(mut self, qubit: usize) -> Self {
        self.add_gate(Gate::t(), vec![qubit]).ok();
        self
    }

    /// Apply T† gate
    pub fn tdg(mut self, qubit: usize) -> Self {
        self.add_gate(Gate::tdg(), vec![qubit]).ok();
        self
    }

    /// Apply Rx rotation
    pub fn rx(mut self, theta: f64, qubit: usize) -> Self {
        self.add_gate(Gate::rx(theta), vec![qubit]).ok();
        self
    }

    /// Apply Ry rotation
    pub fn ry(mut self, theta: f64, qubit: usize) -> Self {
        self.add_gate(Gate::ry(theta), vec![qubit]).ok();
        self
    }

    /// Apply Rz rotation
    pub fn rz(mut self, theta: f64, qubit: usize) -> Self {
        self.add_gate(Gate::rz(theta), vec![qubit]).ok();
        self
    }

    /// Apply U1 gate
    pub fn u1(mut self, lambda: f64, qubit: usize) -> Self {
        self.add_gate(Gate::u1(lambda), vec![qubit]).ok();
        self
    }

    /// Apply U3 gate (general single-qubit unitary)
    pub fn u3(mut self, theta: f64, phi: f64, lambda: f64, qubit: usize) -> Self {
        self.add_gate(Gate::u3(theta, phi, lambda), vec![qubit]).ok();
        self
    }

    // ==================== Two-Qubit Gates ====================

    /// Apply CNOT gate
    pub fn cnot(mut self, control: usize, target: usize) -> Self {
        self.add_gate(Gate::cnot(), vec![control, target]).ok();
        self
    }

    /// Alias for CNOT
    pub fn cx(self, control: usize, target: usize) -> Self {
        self.cnot(control, target)
    }

    /// Apply CZ gate
    pub fn cz(mut self, qubit1: usize, qubit2: usize) -> Self {
        self.add_gate(Gate::cz(), vec![qubit1, qubit2]).ok();
        self
    }

    /// Apply CY gate
    pub fn cy(mut self, control: usize, target: usize) -> Self {
        self.add_gate(Gate::cy(), vec![control, target]).ok();
        self
    }

    /// Apply SWAP gate
    pub fn swap(mut self, qubit1: usize, qubit2: usize) -> Self {
        self.add_gate(Gate::swap(), vec![qubit1, qubit2]).ok();
        self
    }

    /// Apply iSWAP gate
    pub fn iswap(mut self, qubit1: usize, qubit2: usize) -> Self {
        self.add_gate(Gate::iswap(), vec![qubit1, qubit2]).ok();
        self
    }

    /// Apply controlled-Rz gate
    pub fn crz(mut self, theta: f64, control: usize, target: usize) -> Self {
        self.add_gate(Gate::crz(theta), vec![control, target]).ok();
        self
    }

    /// Apply controlled-phase gate
    pub fn cphase(mut self, theta: f64, qubit1: usize, qubit2: usize) -> Self {
        self.add_gate(Gate::cphase(theta), vec![qubit1, qubit2]).ok();
        self
    }

    // ==================== Three-Qubit Gates ====================

    /// Apply Toffoli gate
    pub fn toffoli(mut self, control1: usize, control2: usize, target: usize) -> Self {
        self.add_gate(Gate::toffoli(), vec![control1, control2, target]).ok();
        self
    }

    /// Alias for Toffoli
    pub fn ccx(self, control1: usize, control2: usize, target: usize) -> Self {
        self.toffoli(control1, control2, target)
    }

    /// Apply Fredkin gate
    pub fn fredkin(mut self, control: usize, target1: usize, target2: usize) -> Self {
        self.add_gate(Gate::fredkin(), vec![control, target1, target2]).ok();
        self
    }

    /// Alias for Fredkin
    pub fn cswap(self, control: usize, target1: usize, target2: usize) -> Self {
        self.fredkin(control, target1, target2)
    }

    // ==================== Multi-Qubit Operations ====================

    /// Apply Hadamard to all qubits
    pub fn h_all(mut self) -> Self {
        for q in 0..self.num_qubits {
            self = self.h(q);
        }
        self
    }

    /// Apply X to all qubits
    pub fn x_all(mut self) -> Self {
        for q in 0..self.num_qubits {
            self = self.x(q);
        }
        self
    }

    /// Create a barrier (visual separator, no operation)
    pub fn barrier(self) -> Self {
        // Barriers are visual only, no actual gate
        self
    }

    // ==================== Circuit Composition ====================

    /// Append another circuit
    pub fn append(&mut self, other: &Circuit) -> Result<&mut Self> {
        if other.num_qubits > self.num_qubits {
            return Err(CircuitError::InvalidParameter(
                format!("Cannot append {}-qubit circuit to {}-qubit circuit",
                    other.num_qubits, self.num_qubits)
            ));
        }
        self.instructions.extend(other.instructions.clone());
        Ok(self)
    }

    /// Get the inverse circuit
    pub fn inverse(&self) -> Self {
        let mut inv = Circuit::new(self.num_qubits);
        inv.name = format!("{}†", self.name);

        // Reverse the order and take adjoint of each gate
        for instruction in self.instructions.iter().rev() {
            inv.instructions.push(CircuitInstruction {
                gate: instruction.gate.adjoint(),
                qubits: instruction.qubits.clone(),
                condition: instruction.condition.clone(),
            });
        }

        inv
    }

    /// Repeat the circuit n times
    pub fn repeat(&self, n: usize) -> Self {
        let mut repeated = Circuit::new(self.num_qubits);
        repeated.name = format!("{}×{}", self.name, n);

        for _ in 0..n {
            repeated.instructions.extend(self.instructions.clone());
        }

        repeated
    }

    /// Convert to OpenQASM 2.0 string
    pub fn to_qasm(&self) -> String {
        let mut qasm = String::new();
        qasm.push_str("OPENQASM 2.0;\n");
        qasm.push_str("include \"qelib1.inc\";\n\n");
        qasm.push_str(&format!("qreg q[{}];\n", self.num_qubits));
        qasm.push_str(&format!("creg c[{}];\n\n", self.num_classical_bits));

        for instruction in &self.instructions {
            let gate_name = instruction.gate.name.to_lowercase();
            let qubits: Vec<String> = instruction.qubits.iter()
                .map(|q| format!("q[{}]", q))
                .collect();

            qasm.push_str(&format!("{} {};\n", gate_name, qubits.join(", ")));
        }

        qasm
    }
}

/// Builder pattern for more complex circuits
pub struct CircuitBuilder {
    circuit: Circuit,
}

impl CircuitBuilder {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            circuit: Circuit::new(num_qubits),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.circuit.name = name.to_string();
        self
    }

    pub fn add(mut self, gate: Gate, qubits: Vec<usize>) -> Self {
        self.circuit.add_gate(gate, qubits).ok();
        self
    }

    pub fn h(mut self, qubit: usize) -> Self {
        self.circuit = self.circuit.h(qubit);
        self
    }

    pub fn x(mut self, qubit: usize) -> Self {
        self.circuit = self.circuit.x(qubit);
        self
    }

    pub fn cnot(mut self, control: usize, target: usize) -> Self {
        self.circuit = self.circuit.cnot(control, target);
        self
    }

    pub fn build(self) -> Circuit {
        self.circuit
    }
}

impl Default for Circuit {
    fn default() -> Self {
        Self::new(1)
    }
}

impl std::fmt::Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Circuit '{}' ({} qubits, depth {})", self.name, self.num_qubits, self.depth())?;
        for (i, instruction) in self.instructions.iter().enumerate() {
            writeln!(f, "  {}: {} on {:?}", i, instruction.gate.name, instruction.qubits)?;
        }
        Ok(())
    }
}

// ==================== Common Circuit Patterns ====================

impl Circuit {
    /// Create a Bell state circuit
    pub fn bell_state() -> Self {
        Circuit::new(2)
            .h(0)
            .cnot(0, 1)
    }

    /// Create a GHZ state circuit for n qubits
    pub fn ghz_state(n: usize) -> Self {
        let mut circuit = Circuit::new(n).h(0);
        for i in 0..n-1 {
            circuit = circuit.cnot(i, i + 1);
        }
        circuit
    }

    /// Create a quantum Fourier transform circuit
    pub fn qft(n: usize) -> Self {
        use std::f64::consts::PI;

        let mut circuit = Circuit::with_name(n, "QFT");

        for i in 0..n {
            circuit = circuit.h(i);
            for j in (i+1)..n {
                let k = j - i + 1;
                let theta = PI / (1 << (k - 1)) as f64;
                circuit = circuit.cphase(theta, j, i);
            }
        }

        // Swap qubits to get correct output ordering
        for i in 0..n/2 {
            circuit = circuit.swap(i, n - 1 - i);
        }

        circuit
    }

    /// Create inverse QFT
    pub fn iqft(n: usize) -> Self {
        Circuit::qft(n).inverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_creation() {
        let circuit = Circuit::new(3)
            .h(0)
            .cnot(0, 1)
            .cnot(1, 2);

        assert_eq!(circuit.num_qubits, 3);
        assert_eq!(circuit.gate_count(), 3);
    }

    #[test]
    fn test_circuit_depth() {
        let circuit = Circuit::new(2)
            .h(0)
            .h(1)  // Can be parallel with previous H
            .cnot(0, 1);

        assert_eq!(circuit.depth(), 2);
    }

    #[test]
    fn test_bell_state() {
        let circuit = Circuit::bell_state();
        assert_eq!(circuit.num_qubits, 2);
        assert_eq!(circuit.gate_count(), 2);
    }

    #[test]
    fn test_ghz_state() {
        let circuit = Circuit::ghz_state(4);
        assert_eq!(circuit.num_qubits, 4);
        assert_eq!(circuit.gate_count(), 4); // 1 H + 3 CNOTs
    }

    #[test]
    fn test_inverse() {
        let circuit = Circuit::new(2)
            .h(0)
            .s(0)
            .cnot(0, 1);

        let inverse = circuit.inverse();
        assert_eq!(inverse.gate_count(), circuit.gate_count());
    }
}
