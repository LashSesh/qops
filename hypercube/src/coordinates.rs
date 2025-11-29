//! 5D Coordinate System for Hypercube Framework
//!
//! Maps the Hypercube coordinates (ψ, ρ, ω, χ, η) to the QOPS resonance metrics:
//! - ψ (psi): Quality / Spectral coherence
//! - ρ (rho): Stability / Robustness
//! - ω (omega): Efficiency / Performance
//! - χ (chi): Topological coherence (5D extension)
//! - η (eta): Fluctuation measure (5D extension)

use qops_core::Signature5D;
use serde::{Deserialize, Serialize};

/// 5D coordinate in the Hypercube space
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coord5D {
    /// ψ - Quality / Spectral coherence (dimension 0)
    pub psi: f64,
    /// ρ - Stability / Robustness (dimension 1)
    pub rho: f64,
    /// ω - Efficiency / Performance (dimension 2)
    pub omega: f64,
    /// χ - Topological coherence (dimension 3)
    pub chi: f64,
    /// η - Fluctuation measure (dimension 4)
    pub eta: f64,
}

impl Coord5D {
    /// Create a new 5D coordinate
    pub fn new(psi: f64, rho: f64, omega: f64, chi: f64, eta: f64) -> Self {
        Self { psi, rho, omega, chi, eta }
    }

    /// Create from a vector
    pub fn from_vec(v: &[f64; 5]) -> Self {
        Self::new(v[0], v[1], v[2], v[3], v[4])
    }

    /// Create origin coordinate
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0)
    }

    /// Create unit coordinate (all ones)
    pub fn unit() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0, 1.0)
    }

    /// Create centered coordinate (all 0.5)
    pub fn center() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.5, 0.5)
    }

    /// Convert to array
    pub fn to_array(&self) -> [f64; 5] {
        [self.psi, self.rho, self.omega, self.chi, self.eta]
    }

    /// Convert to QOPS Signature5D
    pub fn to_signature(&self) -> Signature5D {
        Signature5D::new(self.psi, self.rho, self.omega, self.chi, self.eta)
    }

    /// Create from QOPS Signature5D
    pub fn from_signature(sig: &Signature5D) -> Self {
        Self::new(sig.psi, sig.rho, sig.omega, sig.chi, sig.eta)
    }

    /// Compute Euclidean distance to another coordinate
    pub fn distance(&self, other: &Self) -> f64 {
        let d = [
            self.psi - other.psi,
            self.rho - other.rho,
            self.omega - other.omega,
            self.chi - other.chi,
            self.eta - other.eta,
        ];
        d.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// Compute L1 (Manhattan) distance
    pub fn distance_l1(&self, other: &Self) -> f64 {
        (self.psi - other.psi).abs()
            + (self.rho - other.rho).abs()
            + (self.omega - other.omega).abs()
            + (self.chi - other.chi).abs()
            + (self.eta - other.eta).abs()
    }

    /// Compute magnitude (distance from origin)
    pub fn magnitude(&self) -> f64 {
        self.distance(&Self::origin())
    }

    /// Normalize to unit vector
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag < 1e-10 {
            return Self::origin();
        }
        Self::new(
            self.psi / mag,
            self.rho / mag,
            self.omega / mag,
            self.chi / mag,
            self.eta / mag,
        )
    }

    /// Add another coordinate
    pub fn add(&self, other: &Self) -> Self {
        Self::new(
            self.psi + other.psi,
            self.rho + other.rho,
            self.omega + other.omega,
            self.chi + other.chi,
            self.eta + other.eta,
        )
    }

    /// Subtract another coordinate
    pub fn sub(&self, other: &Self) -> Self {
        Self::new(
            self.psi - other.psi,
            self.rho - other.rho,
            self.omega - other.omega,
            self.chi - other.chi,
            self.eta - other.eta,
        )
    }

    /// Scale by a factor
    pub fn scale(&self, factor: f64) -> Self {
        Self::new(
            self.psi * factor,
            self.rho * factor,
            self.omega * factor,
            self.chi * factor,
            self.eta * factor,
        )
    }

    /// Dot product with another coordinate
    pub fn dot(&self, other: &Self) -> f64 {
        self.psi * other.psi
            + self.rho * other.rho
            + self.omega * other.omega
            + self.chi * other.chi
            + self.eta * other.eta
    }

    /// Linear interpolation to another coordinate
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        self.add(&other.sub(self).scale(t))
    }

    /// Clamp all values to [0, 1] range
    pub fn clamp_unit(&self) -> Self {
        Self::new(
            self.psi.clamp(0.0, 1.0),
            self.rho.clamp(0.0, 1.0),
            self.omega.clamp(0.0, 1.0),
            self.chi.clamp(0.0, 1.0),
            self.eta.clamp(0.0, 1.0),
        )
    }

    /// Compute resonance score using QOPS formula
    /// R(v) = 0.4·ψ + 0.3·ρ + 0.3·ω + 0.05·χ - 0.05·η
    pub fn resonance(&self) -> f64 {
        let base = 0.4 * self.psi + 0.3 * self.rho + 0.3 * self.omega;
        let correction = 0.05 * self.chi - 0.05 * self.eta;
        (base + correction).clamp(0.0, 1.0)
    }

    /// Get dimension by index
    pub fn get(&self, dim: usize) -> f64 {
        match dim {
            0 => self.psi,
            1 => self.rho,
            2 => self.omega,
            3 => self.chi,
            4 => self.eta,
            _ => 0.0,
        }
    }

    /// Set dimension by index
    pub fn set(&mut self, dim: usize, value: f64) {
        match dim {
            0 => self.psi = value,
            1 => self.rho = value,
            2 => self.omega = value,
            3 => self.chi = value,
            4 => self.eta = value,
            _ => {}
        }
    }
}

impl Default for Coord5D {
    fn default() -> Self {
        Self::center()
    }
}

impl std::fmt::Display for Coord5D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(ψ={:.3}, ρ={:.3}, ω={:.3}, χ={:.3}, η={:.3})",
            self.psi, self.rho, self.omega, self.chi, self.eta)
    }
}

/// 5D coordinate system with transformation capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateSystem {
    /// Origin point of the coordinate system
    pub origin: Coord5D,
    /// Basis vectors (5x5 matrix as column vectors)
    pub basis: [[f64; 5]; 5],
    /// Scale factors for each dimension
    pub scale: [f64; 5],
}

impl CoordinateSystem {
    /// Create a standard coordinate system
    pub fn standard() -> Self {
        Self {
            origin: Coord5D::origin(),
            basis: [
                [1.0, 0.0, 0.0, 0.0, 0.0], // ψ axis
                [0.0, 1.0, 0.0, 0.0, 0.0], // ρ axis
                [0.0, 0.0, 1.0, 0.0, 0.0], // ω axis
                [0.0, 0.0, 0.0, 1.0, 0.0], // χ axis
                [0.0, 0.0, 0.0, 0.0, 1.0], // η axis
            ],
            scale: [1.0; 5],
        }
    }

    /// Create a coordinate system centered at a point
    pub fn centered_at(center: Coord5D) -> Self {
        Self {
            origin: center,
            basis: [
                [1.0, 0.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 0.0, 1.0],
            ],
            scale: [1.0; 5],
        }
    }

    /// Transform a local coordinate to global coordinate
    pub fn to_global(&self, local: &Coord5D) -> Coord5D {
        let local_arr = local.to_array();
        let mut result = [0.0; 5];

        for i in 0..5 {
            for j in 0..5 {
                result[i] += self.basis[j][i] * local_arr[j] * self.scale[j];
            }
            result[i] += self.origin.get(i);
        }

        Coord5D::from_vec(&result)
    }

    /// Transform a global coordinate to local coordinate
    pub fn to_local(&self, global: &Coord5D) -> Coord5D {
        // Subtract origin
        let shifted = global.sub(&self.origin);
        let shifted_arr = shifted.to_array();

        // For orthonormal basis, inverse is transpose
        // Apply inverse transform (simplified for orthonormal case)
        let mut result = [0.0; 5];
        for i in 0..5 {
            for j in 0..5 {
                result[i] += self.basis[i][j] * shifted_arr[j];
            }
            if self.scale[i].abs() > 1e-10 {
                result[i] /= self.scale[i];
            }
        }

        Coord5D::from_vec(&result)
    }
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        Self::standard()
    }
}

/// Coordinate transformation operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateTransform {
    /// Rotation angles for each pair of dimensions (10 pairs in 5D)
    pub rotations: [f64; 10],
    /// Translation vector
    pub translation: Coord5D,
    /// Scale factors
    pub scale: [f64; 5],
}

impl CoordinateTransform {
    /// Create identity transform
    pub fn identity() -> Self {
        Self {
            rotations: [0.0; 10],
            translation: Coord5D::origin(),
            scale: [1.0; 5],
        }
    }

    /// Create a translation transform
    pub fn translation(offset: Coord5D) -> Self {
        Self {
            rotations: [0.0; 10],
            translation: offset,
            scale: [1.0; 5],
        }
    }

    /// Create a scaling transform
    pub fn scaling(factors: [f64; 5]) -> Self {
        Self {
            rotations: [0.0; 10],
            translation: Coord5D::origin(),
            scale: factors,
        }
    }

    /// Create a rotation transform in a specific plane
    /// plane_index: 0=(ψ,ρ), 1=(ψ,ω), 2=(ψ,χ), 3=(ψ,η), 4=(ρ,ω), 5=(ρ,χ), 6=(ρ,η), 7=(ω,χ), 8=(ω,η), 9=(χ,η)
    pub fn rotation(plane_index: usize, angle: f64) -> Self {
        let mut rotations = [0.0; 10];
        if plane_index < 10 {
            rotations[plane_index] = angle;
        }
        Self {
            rotations,
            translation: Coord5D::origin(),
            scale: [1.0; 5],
        }
    }

    /// Apply transform to a coordinate
    pub fn apply(&self, coord: &Coord5D) -> Coord5D {
        // Apply scale first
        let mut result = Coord5D::new(
            coord.psi * self.scale[0],
            coord.rho * self.scale[1],
            coord.omega * self.scale[2],
            coord.chi * self.scale[3],
            coord.eta * self.scale[4],
        );

        // Apply rotations (simplified - full implementation would use rotation matrices)
        // For now, apply translation only
        result = result.add(&self.translation);

        result
    }

    /// Compose two transforms (self then other)
    pub fn compose(&self, other: &Self) -> Self {
        // Simplified composition - just combines translation and scale
        Self {
            rotations: [0.0; 10], // Full rotation composition is complex
            translation: self.translation.add(&other.translation),
            scale: [
                self.scale[0] * other.scale[0],
                self.scale[1] * other.scale[1],
                self.scale[2] * other.scale[2],
                self.scale[3] * other.scale[3],
                self.scale[4] * other.scale[4],
            ],
        }
    }

    /// Get inverse transform
    pub fn inverse(&self) -> Self {
        let inv_scale = [
            if self.scale[0].abs() > 1e-10 { 1.0 / self.scale[0] } else { 0.0 },
            if self.scale[1].abs() > 1e-10 { 1.0 / self.scale[1] } else { 0.0 },
            if self.scale[2].abs() > 1e-10 { 1.0 / self.scale[2] } else { 0.0 },
            if self.scale[3].abs() > 1e-10 { 1.0 / self.scale[3] } else { 0.0 },
            if self.scale[4].abs() > 1e-10 { 1.0 / self.scale[4] } else { 0.0 },
        ];

        Self {
            rotations: self.rotations.map(|r| -r),
            translation: self.translation.scale(-1.0),
            scale: inv_scale,
        }
    }
}

impl Default for CoordinateTransform {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_coord5d_creation() {
        let c = Coord5D::new(0.5, 0.6, 0.7, 0.8, 0.9);
        assert_relative_eq!(c.psi, 0.5);
        assert_relative_eq!(c.rho, 0.6);
        assert_relative_eq!(c.omega, 0.7);
        assert_relative_eq!(c.chi, 0.8);
        assert_relative_eq!(c.eta, 0.9);
    }

    #[test]
    fn test_coord5d_distance() {
        let a = Coord5D::origin();
        let b = Coord5D::unit();
        let dist = a.distance(&b);
        assert_relative_eq!(dist, 5.0_f64.sqrt(), epsilon = 1e-10);
    }

    #[test]
    fn test_coord5d_resonance() {
        let c = Coord5D::new(1.0, 1.0, 1.0, 1.0, 0.0);
        let res = c.resonance();
        assert_relative_eq!(res, 1.0, epsilon = 0.1);
    }

    #[test]
    fn test_coordinate_system() {
        let cs = CoordinateSystem::standard();
        let local = Coord5D::new(0.5, 0.5, 0.5, 0.5, 0.5);
        let global = cs.to_global(&local);
        let back = cs.to_local(&global);

        assert_relative_eq!(local.psi, back.psi, epsilon = 1e-10);
        assert_relative_eq!(local.rho, back.rho, epsilon = 1e-10);
    }

    #[test]
    fn test_transform_identity() {
        let t = CoordinateTransform::identity();
        let c = Coord5D::new(0.5, 0.6, 0.7, 0.8, 0.9);
        let result = t.apply(&c);

        assert_relative_eq!(c.psi, result.psi, epsilon = 1e-10);
        assert_relative_eq!(c.rho, result.rho, epsilon = 1e-10);
    }
}
