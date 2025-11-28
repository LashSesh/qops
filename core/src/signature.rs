//! Signature types for performance and quality metrics.
//!
//! The signature system provides a unified representation for:
//! - Quality (ψ/psi) - Semantic/algorithmic quality
//! - Stability (ρ/rho) - Robustness and consistency
//! - Efficiency (ω/omega) - Computational efficiency
//!
//! Extended signatures add:
//! - Topological coherence (χ/chi)
//! - Resonance fluctuation (η/eta)

use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

/// 3-dimensional signature (ψ, ρ, ω)
///
/// Used by QSO and Seraphic Calibration Shell for performance tracking.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Signature3D {
    /// Quality metric (0.0 - 1.0)
    pub psi: f64,
    /// Stability metric (0.0 - 1.0)
    pub rho: f64,
    /// Efficiency metric (0.0 - 1.0)
    pub omega: f64,
}

impl Signature3D {
    /// Create a new 3D signature
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self {
            psi: psi.clamp(0.0, 1.0),
            rho: rho.clamp(0.0, 1.0),
            omega: omega.clamp(0.0, 1.0),
        }
    }

    /// Create a zero signature
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Create a unit signature (all 1.0)
    pub fn unit() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// Compute the product ψ·ρ·ω
    pub fn product(&self) -> f64 {
        self.psi * self.rho * self.omega
    }

    /// Compute weighted sum with standard weights (0.4, 0.3, 0.3)
    pub fn weighted_sum(&self) -> f64 {
        0.4 * self.psi + 0.3 * self.rho + 0.3 * self.omega
    }

    /// Compute Euclidean distance to another signature
    pub fn distance(&self, other: &Self) -> f64 {
        ((self.psi - other.psi).powi(2)
            + (self.rho - other.rho).powi(2)
            + (self.omega - other.omega).powi(2))
        .sqrt()
    }

    /// Convert to vector representation
    pub fn to_vec(&self) -> [f64; 3] {
        [self.psi, self.rho, self.omega]
    }

    /// Extend to 5D signature with default extensions
    pub fn to_5d(&self) -> Signature5D {
        Signature5D::from_3d(self)
    }
}

impl Default for Signature3D {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5)
    }
}

impl Add for Signature3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.psi + other.psi,
            self.rho + other.rho,
            self.omega + other.omega,
        )
    }
}

impl Sub for Signature3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.psi - other.psi,
            self.rho - other.rho,
            self.omega - other.omega,
        )
    }
}

impl Mul<f64> for Signature3D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self::new(self.psi * scalar, self.rho * scalar, self.omega * scalar)
    }
}

/// 5-dimensional signature (ψ, ρ, ω, χ, η)
///
/// Extended signature used by Genesis/MOGE for deeper analysis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Signature5D {
    /// Quality metric (0.0 - 1.0)
    pub psi: f64,
    /// Stability metric (0.0 - 1.0)
    pub rho: f64,
    /// Efficiency metric (0.0 - 1.0)
    pub omega: f64,
    /// Topological coherence (0.0 - 1.0)
    pub chi: f64,
    /// Resonance fluctuation (0.0 - 1.0)
    pub eta: f64,
}

impl Signature5D {
    /// Create a new 5D signature
    pub fn new(psi: f64, rho: f64, omega: f64, chi: f64, eta: f64) -> Self {
        Self {
            psi: psi.clamp(0.0, 1.0),
            rho: rho.clamp(0.0, 1.0),
            omega: omega.clamp(0.0, 1.0),
            chi: chi.clamp(0.0, 1.0),
            eta: eta.clamp(0.0, 1.0),
        }
    }

    /// Create from 3D signature with default extensions
    pub fn from_3d(sig: &Signature3D) -> Self {
        Self::new(sig.psi, sig.rho, sig.omega, 0.5, 0.5)
    }

    /// Create a zero signature
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0)
    }

    /// Create a unit signature
    pub fn unit() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0, 1.0)
    }

    /// Extract the 3D core signature
    pub fn to_3d(&self) -> Signature3D {
        Signature3D::new(self.psi, self.rho, self.omega)
    }

    /// Compute the product ψ·ρ·ω
    pub fn product(&self) -> f64 {
        self.psi * self.rho * self.omega
    }

    /// Compute extended product including χ and η
    pub fn extended_product(&self) -> f64 {
        self.psi * self.rho * self.omega * (1.0 + self.chi - self.eta)
    }

    /// Compute Euclidean distance to another signature
    pub fn distance(&self, other: &Self) -> f64 {
        ((self.psi - other.psi).powi(2)
            + (self.rho - other.rho).powi(2)
            + (self.omega - other.omega).powi(2)
            + (self.chi - other.chi).powi(2)
            + (self.eta - other.eta).powi(2))
        .sqrt()
    }

    /// Convert to vector representation
    pub fn to_vec(&self) -> [f64; 5] {
        [self.psi, self.rho, self.omega, self.chi, self.eta]
    }

    /// Clamp all values to valid range [0, 1]
    pub fn clamp(&mut self) {
        self.psi = self.psi.clamp(0.0, 1.0);
        self.rho = self.rho.clamp(0.0, 1.0);
        self.omega = self.omega.clamp(0.0, 1.0);
        self.chi = self.chi.clamp(0.0, 1.0);
        self.eta = self.eta.clamp(0.0, 1.0);
    }
}

impl Default for Signature5D {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.5, 0.5)
    }
}

impl Add for Signature5D {
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

impl Sub for Signature5D {
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

impl Mul<f64> for Signature5D {
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

/// Unified signature type that can be either 3D or 5D
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Signature {
    /// 3-dimensional signature
    D3(Signature3D),
    /// 5-dimensional signature
    D5(Signature5D),
}

impl Signature {
    /// Get the core triplet (ψ, ρ, ω)
    pub fn triplet(&self) -> (f64, f64, f64) {
        match self {
            Signature::D3(s) => (s.psi, s.rho, s.omega),
            Signature::D5(s) => (s.psi, s.rho, s.omega),
        }
    }

    /// Get psi value
    pub fn psi(&self) -> f64 {
        match self {
            Signature::D3(s) => s.psi,
            Signature::D5(s) => s.psi,
        }
    }

    /// Get rho value
    pub fn rho(&self) -> f64 {
        match self {
            Signature::D3(s) => s.rho,
            Signature::D5(s) => s.rho,
        }
    }

    /// Get omega value
    pub fn omega(&self) -> f64 {
        match self {
            Signature::D3(s) => s.omega,
            Signature::D5(s) => s.omega,
        }
    }

    /// Convert to 3D signature
    pub fn to_3d(&self) -> Signature3D {
        match self {
            Signature::D3(s) => *s,
            Signature::D5(s) => s.to_3d(),
        }
    }

    /// Convert to 5D signature
    pub fn to_5d(&self) -> Signature5D {
        match self {
            Signature::D3(s) => s.to_5d(),
            Signature::D5(s) => *s,
        }
    }

    /// Compute weighted resonance score
    pub fn resonance(&self) -> f64 {
        match self {
            Signature::D3(s) => s.weighted_sum(),
            Signature::D5(s) => {
                let base = 0.4 * s.psi + 0.3 * s.rho + 0.3 * s.omega;
                let correction = 0.05 * s.chi - 0.05 * s.eta;
                (base + correction).clamp(0.0, 1.0)
            }
        }
    }
}

impl Default for Signature {
    fn default() -> Self {
        Signature::D3(Signature3D::default())
    }
}

impl From<Signature3D> for Signature {
    fn from(s: Signature3D) -> Self {
        Signature::D3(s)
    }
}

impl From<Signature5D> for Signature {
    fn from(s: Signature5D) -> Self {
        Signature::D5(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_signature3d_creation() {
        let sig = Signature3D::new(0.8, 0.7, 0.6);
        assert_relative_eq!(sig.psi, 0.8);
        assert_relative_eq!(sig.rho, 0.7);
        assert_relative_eq!(sig.omega, 0.6);
    }

    #[test]
    fn test_signature3d_clamping() {
        let sig = Signature3D::new(1.5, -0.5, 0.5);
        assert_relative_eq!(sig.psi, 1.0);
        assert_relative_eq!(sig.rho, 0.0);
        assert_relative_eq!(sig.omega, 0.5);
    }

    #[test]
    fn test_signature3d_weighted_sum() {
        let sig = Signature3D::new(1.0, 1.0, 1.0);
        assert_relative_eq!(sig.weighted_sum(), 1.0);

        let sig2 = Signature3D::new(0.5, 0.5, 0.5);
        assert_relative_eq!(sig2.weighted_sum(), 0.5);
    }

    #[test]
    fn test_signature5d_from_3d() {
        let sig3 = Signature3D::new(0.8, 0.7, 0.6);
        let sig5 = sig3.to_5d();

        assert_relative_eq!(sig5.psi, 0.8);
        assert_relative_eq!(sig5.rho, 0.7);
        assert_relative_eq!(sig5.omega, 0.6);
        assert_relative_eq!(sig5.chi, 0.5);
        assert_relative_eq!(sig5.eta, 0.5);
    }

    #[test]
    fn test_signature_distance() {
        let a = Signature3D::new(0.5, 0.5, 0.5);
        let b = Signature3D::new(0.5, 0.5, 0.5);
        assert_relative_eq!(a.distance(&b), 0.0);

        let c = Signature3D::new(1.0, 0.5, 0.5);
        assert_relative_eq!(a.distance(&c), 0.5);
    }

    #[test]
    fn test_unified_signature() {
        let sig3 = Signature::D3(Signature3D::new(0.8, 0.7, 0.6));
        let sig5 = Signature::D5(Signature5D::new(0.8, 0.7, 0.6, 0.5, 0.5));

        assert_relative_eq!(sig3.psi(), 0.8);
        assert_relative_eq!(sig5.psi(), 0.8);
    }
}
