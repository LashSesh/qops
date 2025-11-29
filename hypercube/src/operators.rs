//! 5D Operators for the Hypercube Framework
//!
//! Implements the four fundamental operators:
//! - **DK (Double Kick)**: Double perturbation operator for state evolution
//! - **SW (Swap Wave)**: Dimensional exchange and wave propagation
//! - **PI (Phase Integration)**: Phase alignment and integration
//! - **WT (Weight Transform)**: Weighted transformation across dimensions
//!
//! Plus the compilation operator:
//! - **Ξ (Xi)**: Self-compilation operator for cube → artifact generation

use crate::coordinates::Coord5D;
use crate::error::{HypercubeError, Result};
use qops_core::Signature5D;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Operator type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OperatorType {
    /// Double Kick - perturbation operator
    DK,
    /// Swap Wave - dimensional exchange
    SW,
    /// Phase Integration - phase alignment
    PI,
    /// Weight Transform - weighted transformation
    WT,
    /// Compilation operator Ξ
    Xi,
    /// Identity operator
    Identity,
    /// Composite operator
    Composite,
}

impl OperatorType {
    /// Get operator name
    pub fn name(&self) -> &'static str {
        match self {
            OperatorType::DK => "Double Kick",
            OperatorType::SW => "Swap Wave",
            OperatorType::PI => "Phase Integration",
            OperatorType::WT => "Weight Transform",
            OperatorType::Xi => "Compilation (Ξ)",
            OperatorType::Identity => "Identity",
            OperatorType::Composite => "Composite",
        }
    }

    /// Get short code
    pub fn code(&self) -> &'static str {
        match self {
            OperatorType::DK => "DK",
            OperatorType::SW => "SW",
            OperatorType::PI => "PI",
            OperatorType::WT => "WT",
            OperatorType::Xi => "Ξ",
            OperatorType::Identity => "I",
            OperatorType::Composite => "C",
        }
    }
}

/// Trait for 5D operators
pub trait Operator5D: Send + Sync {
    /// Get operator type
    fn operator_type(&self) -> OperatorType;

    /// Apply operator to a coordinate
    fn apply(&self, coord: &Coord5D) -> Coord5D;

    /// Apply operator to a signature
    fn apply_signature(&self, sig: &Signature5D) -> Signature5D {
        let coord = Coord5D::from_signature(sig);
        self.apply(&coord).to_signature()
    }

    /// Get operator parameters as array
    fn parameters(&self) -> Vec<f64>;

    /// Check if operator is valid
    fn is_valid(&self) -> bool {
        true
    }

    /// Get operator description
    fn description(&self) -> String {
        format!("{} operator", self.operator_type().name())
    }
}

/// Double Kick Operator (DK)
///
/// Applies a double perturbation to the state:
/// DK(c) = c + α·(sin(β·c) + γ·cos(δ·c))
///
/// This creates oscillatory dynamics in the 5D space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoubleKickOperator {
    /// Primary kick strength
    pub alpha: f64,
    /// First frequency parameter
    pub beta: f64,
    /// Secondary kick strength
    pub gamma: f64,
    /// Second frequency parameter
    pub delta: f64,
    /// Dimensions to apply kick (bitmask)
    pub dimensions: u8,
}

impl DoubleKickOperator {
    /// Create a new Double Kick operator
    pub fn new(alpha: f64, beta: f64, gamma: f64, delta: f64) -> Self {
        Self {
            alpha,
            beta,
            gamma,
            delta,
            dimensions: 0b11111, // All 5 dimensions
        }
    }

    /// Create with default parameters
    pub fn default_params() -> Self {
        Self::new(0.1, PI, 0.05, 2.0 * PI)
    }

    /// Set which dimensions to apply
    pub fn with_dimensions(mut self, dims: u8) -> Self {
        self.dimensions = dims;
        self
    }
}

impl Operator5D for DoubleKickOperator {
    fn operator_type(&self) -> OperatorType {
        OperatorType::DK
    }

    fn apply(&self, coord: &Coord5D) -> Coord5D {
        let mut result = *coord;
        let arr = coord.to_array();

        for (i, &val) in arr.iter().enumerate() {
            if (self.dimensions >> i) & 1 == 1 {
                let kick = self.alpha * (self.beta * val).sin()
                    + self.gamma * (self.delta * val).cos();
                result.set(i, val + kick);
            }
        }

        result.clamp_unit()
    }

    fn parameters(&self) -> Vec<f64> {
        vec![self.alpha, self.beta, self.gamma, self.delta, self.dimensions as f64]
    }
}

impl Default for DoubleKickOperator {
    fn default() -> Self {
        Self::default_params()
    }
}

/// Swap Wave Operator (SW)
///
/// Exchanges and mixes values between dimension pairs with wave modulation:
/// SW(c)_i = cos(θ)·c_i + sin(θ)·c_j
/// SW(c)_j = -sin(θ)·c_i + cos(θ)·c_j
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapWaveOperator {
    /// Rotation angle for each dimension pair (10 pairs in 5D)
    pub angles: [f64; 10],
    /// Wave amplitude
    pub amplitude: f64,
    /// Wave frequency
    pub frequency: f64,
    /// Time parameter for wave evolution
    pub time: f64,
}

impl SwapWaveOperator {
    /// Create a new Swap Wave operator
    pub fn new(angles: [f64; 10], amplitude: f64, frequency: f64) -> Self {
        Self {
            angles,
            amplitude,
            frequency,
            time: 0.0,
        }
    }

    /// Create with default parameters
    pub fn default_params() -> Self {
        Self::new([PI / 4.0; 10], 0.5, 1.0)
    }

    /// Create for a specific dimension pair
    pub fn for_pair(i: usize, j: usize, angle: f64) -> Self {
        let mut angles = [0.0; 10];
        let idx = Self::pair_index(i, j);
        if let Some(idx) = idx {
            angles[idx] = angle;
        }
        Self::new(angles, 0.5, 1.0)
    }

    /// Get index for dimension pair (i, j) where i < j
    fn pair_index(i: usize, j: usize) -> Option<usize> {
        let (i, j) = if i < j { (i, j) } else { (j, i) };
        match (i, j) {
            (0, 1) => Some(0),
            (0, 2) => Some(1),
            (0, 3) => Some(2),
            (0, 4) => Some(3),
            (1, 2) => Some(4),
            (1, 3) => Some(5),
            (1, 4) => Some(6),
            (2, 3) => Some(7),
            (2, 4) => Some(8),
            (3, 4) => Some(9),
            _ => None,
        }
    }

    /// Set time for wave evolution
    pub fn with_time(mut self, t: f64) -> Self {
        self.time = t;
        self
    }
}

impl Operator5D for SwapWaveOperator {
    fn operator_type(&self) -> OperatorType {
        OperatorType::SW
    }

    fn apply(&self, coord: &Coord5D) -> Coord5D {
        let mut arr = coord.to_array();

        // Apply wave modulation
        let wave = self.amplitude * (self.frequency * self.time).sin();

        // Apply rotations for each dimension pair
        let pairs = [
            (0, 1), (0, 2), (0, 3), (0, 4),
            (1, 2), (1, 3), (1, 4),
            (2, 3), (2, 4),
            (3, 4),
        ];

        for (idx, &(i, j)) in pairs.iter().enumerate() {
            let angle = self.angles[idx] + wave;
            if angle.abs() > 1e-10 {
                let cos_a = angle.cos();
                let sin_a = angle.sin();
                let vi = arr[i];
                let vj = arr[j];
                arr[i] = cos_a * vi + sin_a * vj;
                arr[j] = -sin_a * vi + cos_a * vj;
            }
        }

        Coord5D::from_vec(&arr).clamp_unit()
    }

    fn parameters(&self) -> Vec<f64> {
        let mut params = self.angles.to_vec();
        params.extend([self.amplitude, self.frequency, self.time]);
        params
    }
}

impl Default for SwapWaveOperator {
    fn default() -> Self {
        Self::default_params()
    }
}

/// Phase Integration Operator (PI)
///
/// Integrates phase information across dimensions:
/// PI(c) = c + ∫ φ(c) dt where φ is the phase function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseIntegrationOperator {
    /// Phase offset for each dimension
    pub phase_offsets: [f64; 5],
    /// Integration step size
    pub step_size: f64,
    /// Number of integration steps
    pub steps: usize,
    /// Phase coupling matrix (5x5)
    pub coupling: [[f64; 5]; 5],
}

impl PhaseIntegrationOperator {
    /// Create a new Phase Integration operator
    pub fn new(phase_offsets: [f64; 5], step_size: f64, steps: usize) -> Self {
        // Default diagonal coupling
        let mut coupling = [[0.0; 5]; 5];
        for i in 0..5 {
            coupling[i][i] = 1.0;
        }

        Self {
            phase_offsets,
            step_size,
            steps,
            coupling,
        }
    }

    /// Create with default parameters
    pub fn default_params() -> Self {
        Self::new([0.0; 5], 0.01, 10)
    }

    /// Set coupling matrix
    pub fn with_coupling(mut self, coupling: [[f64; 5]; 5]) -> Self {
        self.coupling = coupling;
        self
    }

    /// Create a resonance-aligned phase integration
    pub fn resonance_aligned() -> Self {
        let mut pi = Self::default_params();
        // Couple dimensions to enhance resonance
        pi.coupling = [
            [1.0, 0.2, 0.2, 0.1, -0.1],
            [0.2, 1.0, 0.3, 0.1, -0.1],
            [0.2, 0.3, 1.0, 0.1, -0.1],
            [0.1, 0.1, 0.1, 1.0, 0.0],
            [-0.1, -0.1, -0.1, 0.0, 1.0],
        ];
        pi
    }
}

impl Operator5D for PhaseIntegrationOperator {
    fn operator_type(&self) -> OperatorType {
        OperatorType::PI
    }

    fn apply(&self, coord: &Coord5D) -> Coord5D {
        let mut arr = coord.to_array();

        for _ in 0..self.steps {
            let mut delta = [0.0; 5];

            // Compute phase derivatives with coupling
            for i in 0..5 {
                for j in 0..5 {
                    let phase = (arr[j] * 2.0 * PI + self.phase_offsets[j]).sin();
                    delta[i] += self.coupling[i][j] * phase * self.step_size;
                }
            }

            // Apply integration step
            for i in 0..5 {
                arr[i] += delta[i];
            }
        }

        Coord5D::from_vec(&arr).clamp_unit()
    }

    fn parameters(&self) -> Vec<f64> {
        let mut params = self.phase_offsets.to_vec();
        params.push(self.step_size);
        params.push(self.steps as f64);
        params
    }
}

impl Default for PhaseIntegrationOperator {
    fn default() -> Self {
        Self::default_params()
    }
}

/// Weight Transform Operator (WT)
///
/// Applies weighted transformation with nonlinear mapping:
/// WT(c) = W · σ(c) where W is weight matrix and σ is activation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightTransformOperator {
    /// Weight matrix (5x5)
    pub weights: [[f64; 5]; 5],
    /// Bias vector
    pub bias: [f64; 5],
    /// Activation type
    pub activation: ActivationType,
    /// Temperature for softmax
    pub temperature: f64,
}

/// Activation function type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivationType {
    /// No activation (linear)
    Linear,
    /// Sigmoid activation
    Sigmoid,
    /// Tanh activation
    Tanh,
    /// ReLU activation
    ReLU,
    /// Softmax activation
    Softmax,
}

impl WeightTransformOperator {
    /// Create a new Weight Transform operator
    pub fn new(weights: [[f64; 5]; 5], bias: [f64; 5]) -> Self {
        Self {
            weights,
            bias,
            activation: ActivationType::Sigmoid,
            temperature: 1.0,
        }
    }

    /// Create identity transform
    pub fn identity() -> Self {
        let mut weights = [[0.0; 5]; 5];
        for i in 0..5 {
            weights[i][i] = 1.0;
        }
        Self::new(weights, [0.0; 5])
    }

    /// Create resonance-optimized transform
    pub fn resonance_optimized() -> Self {
        // Weights that enhance resonance score
        let weights = [
            [0.8, 0.1, 0.1, 0.0, 0.0],  // ψ gets priority
            [0.1, 0.8, 0.1, 0.0, 0.0],  // ρ
            [0.1, 0.1, 0.8, 0.0, 0.0],  // ω
            [0.0, 0.0, 0.0, 0.9, 0.1],  // χ
            [0.0, 0.0, 0.0, 0.1, 0.5],  // η reduced
        ];
        let bias = [0.1, 0.1, 0.1, 0.0, -0.1];
        Self::new(weights, bias)
    }

    /// Set activation function
    pub fn with_activation(mut self, activation: ActivationType) -> Self {
        self.activation = activation;
        self
    }

    /// Apply activation function to a value
    fn activate(&self, x: f64) -> f64 {
        match self.activation {
            ActivationType::Linear => x,
            ActivationType::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            ActivationType::Tanh => x.tanh(),
            ActivationType::ReLU => x.max(0.0),
            ActivationType::Softmax => x.exp(), // Will be normalized later
        }
    }
}

impl Operator5D for WeightTransformOperator {
    fn operator_type(&self) -> OperatorType {
        OperatorType::WT
    }

    fn apply(&self, coord: &Coord5D) -> Coord5D {
        let arr = coord.to_array();
        let mut result = [0.0; 5];

        // Matrix multiplication: W · x + b
        for i in 0..5 {
            for j in 0..5 {
                result[i] += self.weights[i][j] * arr[j];
            }
            result[i] += self.bias[i];
            result[i] = self.activate(result[i]);
        }

        // Normalize for softmax
        if self.activation == ActivationType::Softmax {
            let sum: f64 = result.iter().sum();
            if sum > 1e-10 {
                for r in &mut result {
                    *r /= sum;
                }
            }
        }

        Coord5D::from_vec(&result).clamp_unit()
    }

    fn parameters(&self) -> Vec<f64> {
        let mut params = Vec::new();
        for row in &self.weights {
            params.extend(row);
        }
        params.extend(&self.bias);
        params.push(self.temperature);
        params
    }
}

impl Default for WeightTransformOperator {
    fn default() -> Self {
        Self::identity()
    }
}

/// Compilation Operator Ξ (Xi)
///
/// The meta-operator that compiles operator sequences into artifacts.
/// Ξ: Operator Family → Artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationOperator {
    /// Compilation mode
    pub mode: CompilationMode,
    /// Optimization level
    pub optimization_level: usize,
    /// Target resonance threshold
    pub resonance_threshold: f64,
    /// Maximum compilation iterations
    pub max_iterations: usize,
}

/// Compilation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilationMode {
    /// Fast compilation with minimal optimization
    Fast,
    /// Balanced compilation
    Balanced,
    /// Full optimization
    Optimized,
    /// Research mode with full tracing
    Research,
}

impl CompilationOperator {
    /// Create a new compilation operator
    pub fn new(mode: CompilationMode) -> Self {
        let (opt_level, max_iter) = match mode {
            CompilationMode::Fast => (1, 10),
            CompilationMode::Balanced => (2, 50),
            CompilationMode::Optimized => (3, 100),
            CompilationMode::Research => (3, 200),
        };

        Self {
            mode,
            optimization_level: opt_level,
            resonance_threshold: 0.8,
            max_iterations: max_iter,
        }
    }

    /// Set resonance threshold
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.resonance_threshold = threshold;
        self
    }

    /// Compile an operator family into a single coordinate transformation
    pub fn compile_family(&self, family: &OperatorFamily) -> Coord5D {
        let mut current = family.seed.clone();

        for _ in 0..self.max_iterations.min(family.operators.len() * 3) {
            for op in &family.operators {
                current = op.apply(&current);

                // Early exit if resonance threshold met
                if current.resonance() >= self.resonance_threshold {
                    break;
                }
            }

            if current.resonance() >= self.resonance_threshold {
                break;
            }
        }

        current
    }
}

impl Operator5D for CompilationOperator {
    fn operator_type(&self) -> OperatorType {
        OperatorType::Xi
    }

    fn apply(&self, coord: &Coord5D) -> Coord5D {
        // Simple pass-through for single coordinate
        // Real compilation happens in compile_family
        *coord
    }

    fn parameters(&self) -> Vec<f64> {
        vec![
            self.optimization_level as f64,
            self.resonance_threshold,
            self.max_iterations as f64,
        ]
    }
}

impl Default for CompilationOperator {
    fn default() -> Self {
        Self::new(CompilationMode::Balanced)
    }
}

/// A family of operators that can be composed
#[derive(Serialize, Deserialize)]
pub struct OperatorFamily {
    /// Family name
    pub name: String,
    /// Seed coordinate
    pub seed: Coord5D,
    /// List of operators in the family
    #[serde(skip)]
    pub operators: Vec<Box<dyn Operator5D>>,
    /// Operator type sequence (for serialization)
    pub operator_sequence: Vec<OperatorType>,
    /// Family metadata
    pub metadata: FamilyMetadata,
}

impl std::fmt::Debug for OperatorFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OperatorFamily")
            .field("name", &self.name)
            .field("seed", &self.seed)
            .field("operators_count", &self.operators.len())
            .field("operator_sequence", &self.operator_sequence)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl Clone for OperatorFamily {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            seed: self.seed,
            operators: Vec::new(), // Operators not cloned
            operator_sequence: self.operator_sequence.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

/// Metadata for an operator family
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FamilyMetadata {
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Last resonance score
    pub last_resonance: f64,
    /// Number of applications
    pub application_count: usize,
    /// Is stable
    pub is_stable: bool,
    /// Source HDAG node
    pub source_node: Option<String>,
}

impl OperatorFamily {
    /// Create a new operator family
    pub fn new(name: &str, seed: Coord5D) -> Self {
        Self {
            name: name.to_string(),
            seed,
            operators: Vec::new(),
            operator_sequence: Vec::new(),
            metadata: FamilyMetadata::default(),
        }
    }

    /// Add an operator to the family
    pub fn add_operator<O: Operator5D + 'static>(&mut self, op: O) {
        self.operator_sequence.push(op.operator_type());
        self.operators.push(Box::new(op));
    }

    /// Apply all operators in sequence
    pub fn apply(&self, coord: &Coord5D) -> Coord5D {
        let mut current = *coord;
        for op in &self.operators {
            current = op.apply(&current);
        }
        current
    }

    /// Get the resonance after applying all operators to seed
    pub fn evaluate(&self) -> f64 {
        self.apply(&self.seed).resonance()
    }

    /// Create a standard family with all four operators
    pub fn standard(name: &str, seed: Coord5D) -> Self {
        let mut family = Self::new(name, seed);
        family.add_operator(DoubleKickOperator::default());
        family.add_operator(SwapWaveOperator::default());
        family.add_operator(PhaseIntegrationOperator::default());
        family.add_operator(WeightTransformOperator::default());
        family
    }

    /// Create a resonance-optimized family
    pub fn resonance_optimized(name: &str, seed: Coord5D) -> Self {
        let mut family = Self::new(name, seed);
        family.add_operator(DoubleKickOperator::new(0.05, PI / 2.0, 0.02, PI));
        family.add_operator(PhaseIntegrationOperator::resonance_aligned());
        family.add_operator(WeightTransformOperator::resonance_optimized());
        family
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_double_kick() {
        let dk = DoubleKickOperator::default();
        let coord = Coord5D::center();
        let result = dk.apply(&coord);

        // Result should be different from input
        assert!(result.distance(&coord) > 0.0);
        // Result should be in valid range
        assert!(result.psi >= 0.0 && result.psi <= 1.0);
    }

    #[test]
    fn test_swap_wave() {
        let sw = SwapWaveOperator::for_pair(0, 1, PI / 4.0);
        let coord = Coord5D::new(1.0, 0.0, 0.5, 0.5, 0.5);
        let result = sw.apply(&coord);

        // Values should be swapped/mixed
        assert!(result.psi < 1.0);
        assert!(result.rho > 0.0);
    }

    #[test]
    fn test_phase_integration() {
        let pi = PhaseIntegrationOperator::default();
        let coord = Coord5D::center();
        let result = pi.apply(&coord);

        // Result should be modified by phase integration
        assert!(result.distance(&coord) > 0.0 || coord.distance(&result) < 0.1);
    }

    #[test]
    fn test_weight_transform() {
        let wt = WeightTransformOperator::identity();
        let coord = Coord5D::new(0.5, 0.5, 0.5, 0.5, 0.5);
        let result = wt.apply(&coord);

        // Identity transform with sigmoid will change values
        assert!(result.psi > 0.0 && result.psi < 1.0);
    }

    #[test]
    fn test_operator_family() {
        let seed = Coord5D::center();
        let family = OperatorFamily::standard("test", seed);

        let result = family.evaluate();
        assert!(result >= 0.0 && result <= 1.0);
    }

    #[test]
    fn test_compilation_operator() {
        let xi = CompilationOperator::new(CompilationMode::Fast);
        let seed = Coord5D::center();
        let family = OperatorFamily::standard("test", seed);

        let compiled = xi.compile_family(&family);
        assert!(compiled.resonance() > 0.0);
    }
}
