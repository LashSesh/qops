//! High-dimensional state space H^n for blueprint representation.
//!
//! Per the specification, we define:
//! - State Space H^n with coordinates v = (v1, v2, ..., vn)
//! - Core signature (ψ, ρ, ω, χ, η) as a 5D compact description
//! - Extended dimensions for topological features, spectral coefficients, etc.

use crate::error::{KernelError, Result};
use qops_core::{Signature5D, Signature};
use qops_hypercube::Coord5D;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

/// Core 5-dimensional signature as defined in the spec.
///
/// The core signature v_core = (ψ, ρ, ω, χ, η) provides a compact description
/// of a blueprint's informational profile.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CoreSignature {
    /// ψ (psi): Primary quality/potential metric [0.0, 1.0]
    pub psi: f64,
    /// ρ (rho): Density/stability metric [0.0, 1.0]
    pub rho: f64,
    /// ω (omega): Frequency/efficiency metric [0.0, 1.0]
    pub omega: f64,
    /// χ (chi): Connectivity/topological coherence [0.0, 1.0]
    pub chi: f64,
    /// η (eta): Causality/resonance fluctuation [0.0, 1.0]
    pub eta: f64,
}

impl CoreSignature {
    /// Create a new core signature with clamped values
    pub fn new(psi: f64, rho: f64, omega: f64, chi: f64, eta: f64) -> Self {
        Self {
            psi: psi.clamp(0.0, 1.0),
            rho: rho.clamp(0.0, 1.0),
            omega: omega.clamp(0.0, 1.0),
            chi: chi.clamp(0.0, 1.0),
            eta: eta.clamp(0.0, 1.0),
        }
    }

    /// Create zero signature
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0)
    }

    /// Create unit signature (all ones)
    pub fn unit() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0, 1.0)
    }

    /// Create center signature (all 0.5)
    pub fn center() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.5, 0.5)
    }

    /// Compute the core product ψ·ρ·ω
    pub fn core_product(&self) -> f64 {
        self.psi * self.rho * self.omega
    }

    /// Compute the extended product including χ and η
    pub fn extended_product(&self) -> f64 {
        self.psi * self.rho * self.omega * (1.0 + self.chi - self.eta)
    }

    /// Simple resonance R(v) = ψ·ρ·ω as per spec
    pub fn simple_resonance(&self) -> f64 {
        self.psi * self.rho * self.omega
    }

    /// Weighted resonance as per existing QOPS implementation
    pub fn weighted_resonance(&self) -> f64 {
        let base = 0.4 * self.psi + 0.3 * self.rho + 0.3 * self.omega;
        let correction = 0.05 * self.chi - 0.05 * self.eta;
        (base + correction).clamp(0.0, 1.0)
    }

    /// Euclidean distance to another signature
    pub fn distance(&self, other: &Self) -> f64 {
        ((self.psi - other.psi).powi(2)
            + (self.rho - other.rho).powi(2)
            + (self.omega - other.omega).powi(2)
            + (self.chi - other.chi).powi(2)
            + (self.eta - other.eta).powi(2))
        .sqrt()
    }

    /// Convert to array representation
    pub fn to_array(&self) -> [f64; 5] {
        [self.psi, self.rho, self.omega, self.chi, self.eta]
    }

    /// Create from array
    pub fn from_array(arr: &[f64; 5]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3], arr[4])
    }

    /// Convert to Signature5D (QOPS core type)
    pub fn to_signature5d(&self) -> Signature5D {
        Signature5D::new(self.psi, self.rho, self.omega, self.chi, self.eta)
    }

    /// Create from Signature5D
    pub fn from_signature5d(sig: &Signature5D) -> Self {
        Self::new(sig.psi, sig.rho, sig.omega, sig.chi, sig.eta)
    }

    /// Convert to Coord5D (hypercube coordinate)
    pub fn to_coord5d(&self) -> Coord5D {
        Coord5D::new(self.psi, self.rho, self.omega, self.chi, self.eta)
    }

    /// Create from Coord5D
    pub fn from_coord5d(coord: &Coord5D) -> Self {
        Self::new(coord.psi, coord.rho, coord.omega, coord.chi, coord.eta)
    }

    /// Clamp all values to valid range
    pub fn clamp(&mut self) {
        self.psi = self.psi.clamp(0.0, 1.0);
        self.rho = self.rho.clamp(0.0, 1.0);
        self.omega = self.omega.clamp(0.0, 1.0);
        self.chi = self.chi.clamp(0.0, 1.0);
        self.eta = self.eta.clamp(0.0, 1.0);
    }

    /// Linear interpolation between two signatures
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::new(
            self.psi + t * (other.psi - self.psi),
            self.rho + t * (other.rho - self.rho),
            self.omega + t * (other.omega - self.omega),
            self.chi + t * (other.chi - self.chi),
            self.eta + t * (other.eta - self.eta),
        )
    }
}

impl Default for CoreSignature {
    fn default() -> Self {
        Self::center()
    }
}

impl Add for CoreSignature {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.psi + other.psi,
            self.rho + other.rho,
            self.omega + other.omega,
            self.chi + other.chi,
            self.eta + other.eta,
        )
    }
}

impl Sub for CoreSignature {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.psi - other.psi,
            self.rho - other.rho,
            self.omega - other.omega,
            self.chi - other.chi,
            self.eta - other.eta,
        )
    }
}

impl Mul<f64> for CoreSignature {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self::new(
            self.psi * scalar,
            self.rho * scalar,
            self.omega * scalar,
            self.chi * scalar,
            self.eta * scalar,
        )
    }
}

impl From<Signature5D> for CoreSignature {
    fn from(sig: Signature5D) -> Self {
        Self::from_signature5d(&sig)
    }
}

impl From<CoreSignature> for Signature5D {
    fn from(sig: CoreSignature) -> Self {
        sig.to_signature5d()
    }
}

impl From<Coord5D> for CoreSignature {
    fn from(coord: Coord5D) -> Self {
        Self::from_coord5d(&coord)
    }
}

impl From<CoreSignature> for Coord5D {
    fn from(sig: CoreSignature) -> Self {
        sig.to_coord5d()
    }
}

/// Extended state with additional dimensions beyond the core signature.
///
/// This allows for topological features, spectral coefficients, or other
/// domain-specific invariants to be attached to the state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedState {
    /// Core 5D signature
    pub core: CoreSignature,
    /// Additional dimensions (topological features, spectral coefficients, etc.)
    pub extensions: Vec<f64>,
    /// Dimension labels for extensions
    pub extension_labels: Vec<String>,
}

impl ExtendedState {
    /// Create a new extended state with just the core signature
    pub fn from_core(core: CoreSignature) -> Self {
        Self {
            core,
            extensions: Vec::new(),
            extension_labels: Vec::new(),
        }
    }

    /// Create with extensions
    pub fn new(core: CoreSignature, extensions: Vec<f64>, labels: Vec<String>) -> Result<Self> {
        if extensions.len() != labels.len() {
            return Err(KernelError::DimensionMismatch {
                expected: labels.len(),
                got: extensions.len(),
            });
        }
        Ok(Self {
            core,
            extensions,
            extension_labels: labels,
        })
    }

    /// Add an extension dimension
    pub fn add_extension(&mut self, label: &str, value: f64) {
        self.extension_labels.push(label.to_string());
        self.extensions.push(value);
    }

    /// Get total dimensionality
    pub fn dimension(&self) -> usize {
        5 + self.extensions.len()
    }

    /// Convert to full vector
    pub fn to_vec(&self) -> Vec<f64> {
        let mut v = self.core.to_array().to_vec();
        v.extend(&self.extensions);
        v
    }

    /// Get extension by label
    pub fn get_extension(&self, label: &str) -> Option<f64> {
        self.extension_labels
            .iter()
            .position(|l| l == label)
            .map(|i| self.extensions[i])
    }
}

impl Default for ExtendedState {
    fn default() -> Self {
        Self::from_core(CoreSignature::default())
    }
}

/// Generic state representation in H^n.
///
/// This is the primary state type used throughout the kernel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum State {
    /// 5D core state
    Core(CoreSignature),
    /// Extended state with additional dimensions
    Extended(ExtendedState),
    /// Generic n-dimensional state
    Generic { dimension: usize, values: Vec<f64> },
}

impl State {
    /// Create a 5D core state
    pub fn core(psi: f64, rho: f64, omega: f64, chi: f64, eta: f64) -> Self {
        State::Core(CoreSignature::new(psi, rho, omega, chi, eta))
    }

    /// Create from core signature
    pub fn from_signature(sig: CoreSignature) -> Self {
        State::Core(sig)
    }

    /// Get the core signature (projects to 5D if needed)
    pub fn to_core(&self) -> CoreSignature {
        match self {
            State::Core(s) => *s,
            State::Extended(e) => e.core,
            State::Generic { values, .. } => {
                if values.len() >= 5 {
                    CoreSignature::new(values[0], values[1], values[2], values[3], values[4])
                } else {
                    CoreSignature::zero()
                }
            }
        }
    }

    /// Get dimensionality
    pub fn dimension(&self) -> usize {
        match self {
            State::Core(_) => 5,
            State::Extended(e) => e.dimension(),
            State::Generic { dimension, .. } => *dimension,
        }
    }

    /// Convert to vector
    pub fn to_vec(&self) -> Vec<f64> {
        match self {
            State::Core(s) => s.to_array().to_vec(),
            State::Extended(e) => e.to_vec(),
            State::Generic { values, .. } => values.clone(),
        }
    }

    /// Compute resonance (delegates to core signature)
    pub fn resonance(&self) -> f64 {
        self.to_core().simple_resonance()
    }

    /// Compute weighted resonance
    pub fn weighted_resonance(&self) -> f64 {
        self.to_core().weighted_resonance()
    }

    /// Distance to another state (in the shared dimension)
    pub fn distance(&self, other: &State) -> f64 {
        let v1 = self.to_vec();
        let v2 = other.to_vec();
        let min_len = v1.len().min(v2.len());

        v1.iter()
            .zip(v2.iter())
            .take(min_len)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

impl Default for State {
    fn default() -> Self {
        State::Core(CoreSignature::default())
    }
}

impl From<CoreSignature> for State {
    fn from(sig: CoreSignature) -> Self {
        State::Core(sig)
    }
}

impl From<Signature5D> for State {
    fn from(sig: Signature5D) -> Self {
        State::Core(CoreSignature::from_signature5d(&sig))
    }
}

impl From<Coord5D> for State {
    fn from(coord: Coord5D) -> Self {
        State::Core(CoreSignature::from_coord5d(&coord))
    }
}

/// State space H^n configuration and operations.
///
/// This represents the full state space and provides operations for
/// embedding, projection, and navigation within it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSpace {
    /// Base dimensionality (typically 5 for core signature)
    pub base_dimension: usize,
    /// Extended dimensions
    pub extension_dimensions: usize,
    /// Dimension labels
    pub dimension_labels: Vec<String>,
    /// Bounds for each dimension [min, max]
    pub bounds: Vec<(f64, f64)>,
}

impl StateSpace {
    /// Create default 5D state space
    pub fn default_5d() -> Self {
        Self {
            base_dimension: 5,
            extension_dimensions: 0,
            dimension_labels: vec![
                "psi".to_string(),
                "rho".to_string(),
                "omega".to_string(),
                "chi".to_string(),
                "eta".to_string(),
            ],
            bounds: vec![(0.0, 1.0); 5],
        }
    }

    /// Create state space with extensions
    pub fn with_extensions(extension_labels: Vec<String>) -> Self {
        let ext_count = extension_labels.len();
        let mut labels = vec![
            "psi".to_string(),
            "rho".to_string(),
            "omega".to_string(),
            "chi".to_string(),
            "eta".to_string(),
        ];
        labels.extend(extension_labels);

        Self {
            base_dimension: 5,
            extension_dimensions: ext_count,
            dimension_labels: labels,
            bounds: vec![(0.0, 1.0); 5 + ext_count],
        }
    }

    /// Total dimensionality
    pub fn total_dimension(&self) -> usize {
        self.base_dimension + self.extension_dimensions
    }

    /// Check if a state is valid within this space
    pub fn is_valid(&self, state: &State) -> bool {
        let v = state.to_vec();
        if v.len() != self.total_dimension() {
            return false;
        }

        v.iter()
            .zip(self.bounds.iter())
            .all(|(val, (min, max))| *val >= *min && *val <= *max)
    }

    /// Project a state to the core 5D signature
    pub fn project_to_core(&self, state: &State) -> CoreSignature {
        state.to_core()
    }

    /// Embed a core signature into this state space
    pub fn embed_core(&self, sig: CoreSignature) -> State {
        if self.extension_dimensions == 0 {
            State::Core(sig)
        } else {
            State::Extended(ExtendedState {
                core: sig,
                extensions: vec![0.5; self.extension_dimensions],
                extension_labels: self.dimension_labels[5..].to_vec(),
            })
        }
    }

    /// Create origin state (all zeros)
    pub fn origin(&self) -> State {
        if self.extension_dimensions == 0 {
            State::Core(CoreSignature::zero())
        } else {
            State::Generic {
                dimension: self.total_dimension(),
                values: vec![0.0; self.total_dimension()],
            }
        }
    }

    /// Create center state (all 0.5)
    pub fn center(&self) -> State {
        if self.extension_dimensions == 0 {
            State::Core(CoreSignature::center())
        } else {
            State::Generic {
                dimension: self.total_dimension(),
                values: vec![0.5; self.total_dimension()],
            }
        }
    }

    /// Create unit state (all ones)
    pub fn unit(&self) -> State {
        if self.extension_dimensions == 0 {
            State::Core(CoreSignature::unit())
        } else {
            State::Generic {
                dimension: self.total_dimension(),
                values: vec![1.0; self.total_dimension()],
            }
        }
    }
}

impl Default for StateSpace {
    fn default() -> Self {
        Self::default_5d()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_core_signature_creation() {
        let sig = CoreSignature::new(0.8, 0.7, 0.6, 0.5, 0.4);
        assert_relative_eq!(sig.psi, 0.8);
        assert_relative_eq!(sig.rho, 0.7);
        assert_relative_eq!(sig.omega, 0.6);
        assert_relative_eq!(sig.chi, 0.5);
        assert_relative_eq!(sig.eta, 0.4);
    }

    #[test]
    fn test_core_signature_clamping() {
        let sig = CoreSignature::new(1.5, -0.5, 0.5, 2.0, -1.0);
        assert_relative_eq!(sig.psi, 1.0);
        assert_relative_eq!(sig.rho, 0.0);
        assert_relative_eq!(sig.omega, 0.5);
        assert_relative_eq!(sig.chi, 1.0);
        assert_relative_eq!(sig.eta, 0.0);
    }

    #[test]
    fn test_simple_resonance() {
        let sig = CoreSignature::new(0.8, 0.8, 0.8, 0.5, 0.5);
        let res = sig.simple_resonance();
        assert_relative_eq!(res, 0.512, epsilon = 0.001);
    }

    #[test]
    fn test_signature_distance() {
        let a = CoreSignature::center();
        let b = CoreSignature::center();
        assert_relative_eq!(a.distance(&b), 0.0);

        let c = CoreSignature::unit();
        let dist = a.distance(&c);
        assert!(dist > 0.0);
    }

    #[test]
    fn test_state_conversion() {
        let core = CoreSignature::new(0.7, 0.6, 0.5, 0.4, 0.3);
        let sig5d = core.to_signature5d();
        let back = CoreSignature::from_signature5d(&sig5d);

        assert_relative_eq!(core.psi, back.psi);
        assert_relative_eq!(core.rho, back.rho);
    }

    #[test]
    fn test_state_space() {
        let space = StateSpace::default_5d();
        assert_eq!(space.total_dimension(), 5);

        let state = space.center();
        assert!(space.is_valid(&state));
    }

    #[test]
    fn test_extended_state() {
        let core = CoreSignature::center();
        let mut ext = ExtendedState::from_core(core);
        ext.add_extension("chern_number", 0.5);
        ext.add_extension("berry_phase", 0.7);

        assert_eq!(ext.dimension(), 7);
        assert_relative_eq!(ext.get_extension("chern_number").unwrap(), 0.5);
    }
}
