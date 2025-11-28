//! Single qubit representation and operations

use crate::{Complex, ZERO, ONE, I, FRAC_1_SQRT_2};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Bloch sphere coordinates (θ, φ)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlochCoordinates {
    /// Polar angle θ ∈ [0, π]
    pub theta: f64,
    /// Azimuthal angle φ ∈ [0, 2π)
    pub phi: f64,
}

impl BlochCoordinates {
    pub fn new(theta: f64, phi: f64) -> Self {
        Self { theta, phi }
    }

    /// Cartesian coordinates on the Bloch sphere
    pub fn to_cartesian(&self) -> (f64, f64, f64) {
        let x = self.theta.sin() * self.phi.cos();
        let y = self.theta.sin() * self.phi.sin();
        let z = self.theta.cos();
        (x, y, z)
    }

    /// Create from cartesian coordinates
    pub fn from_cartesian(x: f64, y: f64, z: f64) -> Self {
        let theta = z.acos();
        let phi = y.atan2(x);
        Self { theta, phi: if phi < 0.0 { phi + 2.0 * PI } else { phi } }
    }
}

/// A single qubit state |ψ⟩ = α|0⟩ + β|1⟩
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qubit {
    /// Amplitude for |0⟩
    #[serde(with = "complex_serde")]
    pub alpha: Complex,
    /// Amplitude for |1⟩
    #[serde(with = "complex_serde")]
    pub beta: Complex,
}

mod complex_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(c: &Complex, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        (c.re, c.im).serialize(s)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Complex, D::Error>
    where D: Deserializer<'de> {
        let (re, im) = <(f64, f64)>::deserialize(d)?;
        Ok(Complex::new(re, im))
    }
}

impl Qubit {
    /// Create a new qubit with given amplitudes
    pub fn new(alpha: Complex, beta: Complex) -> Self {
        Self { alpha, beta }
    }

    /// The |0⟩ state
    pub fn zero() -> Self {
        Self { alpha: ONE, beta: ZERO }
    }

    /// The |1⟩ state
    pub fn one() -> Self {
        Self { alpha: ZERO, beta: ONE }
    }

    /// The |+⟩ = (|0⟩ + |1⟩)/√2 state
    pub fn plus() -> Self {
        let amp = Complex::new(FRAC_1_SQRT_2, 0.0);
        Self { alpha: amp, beta: amp }
    }

    /// The |−⟩ = (|0⟩ - |1⟩)/√2 state
    pub fn minus() -> Self {
        let amp = Complex::new(FRAC_1_SQRT_2, 0.0);
        Self { alpha: amp, beta: -amp }
    }

    /// The |i⟩ = (|0⟩ + i|1⟩)/√2 state
    pub fn plus_i() -> Self {
        let amp = Complex::new(FRAC_1_SQRT_2, 0.0);
        Self { alpha: amp, beta: amp * I }
    }

    /// The |−i⟩ = (|0⟩ - i|1⟩)/√2 state
    pub fn minus_i() -> Self {
        let amp = Complex::new(FRAC_1_SQRT_2, 0.0);
        Self { alpha: amp, beta: -amp * I }
    }

    /// Create from Bloch sphere coordinates
    pub fn from_bloch(coords: BlochCoordinates) -> Self {
        let alpha = Complex::new((coords.theta / 2.0).cos(), 0.0);
        let beta = Complex::from_polar((coords.theta / 2.0).sin(), coords.phi);
        Self { alpha, beta }
    }

    /// Convert to Bloch sphere coordinates
    pub fn to_bloch(&self) -> BlochCoordinates {
        // Global phase removal
        let phase = self.alpha.arg();
        let alpha_normalized = self.alpha * Complex::from_polar(1.0, -phase);
        let beta_normalized = self.beta * Complex::from_polar(1.0, -phase);

        let theta = 2.0 * alpha_normalized.re.acos();
        let phi = beta_normalized.arg();

        BlochCoordinates {
            theta,
            phi: if phi < 0.0 { phi + 2.0 * PI } else { phi },
        }
    }

    /// Probability of measuring |0⟩
    pub fn prob_zero(&self) -> f64 {
        self.alpha.norm_sqr()
    }

    /// Probability of measuring |1⟩
    pub fn prob_one(&self) -> f64 {
        self.beta.norm_sqr()
    }

    /// Normalize the qubit state
    pub fn normalize(&mut self) {
        let norm = (self.alpha.norm_sqr() + self.beta.norm_sqr()).sqrt();
        if norm > 1e-10 {
            self.alpha /= norm;
            self.beta /= norm;
        }
    }

    /// Check if state is normalized (|α|² + |β|² = 1)
    pub fn is_normalized(&self) -> bool {
        let norm_sq = self.alpha.norm_sqr() + self.beta.norm_sqr();
        (norm_sq - 1.0).abs() < 1e-10
    }

    /// Fidelity with another qubit state
    pub fn fidelity(&self, other: &Qubit) -> f64 {
        let inner = self.alpha.conj() * other.alpha + self.beta.conj() * other.beta;
        inner.norm_sqr()
    }

    /// Inner product ⟨self|other⟩
    pub fn inner_product(&self, other: &Qubit) -> Complex {
        self.alpha.conj() * other.alpha + self.beta.conj() * other.beta
    }

    /// State vector as array [α, β]
    pub fn to_vec(&self) -> [Complex; 2] {
        [self.alpha, self.beta]
    }
}

impl Default for Qubit {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::fmt::Display for Qubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.4})|0⟩ + ({:.4})|1⟩", self.alpha, self.beta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_basis_states() {
        let zero = Qubit::zero();
        assert_relative_eq!(zero.prob_zero(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(zero.prob_one(), 0.0, epsilon = 1e-10);

        let one = Qubit::one();
        assert_relative_eq!(one.prob_zero(), 0.0, epsilon = 1e-10);
        assert_relative_eq!(one.prob_one(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_superposition() {
        let plus = Qubit::plus();
        assert_relative_eq!(plus.prob_zero(), 0.5, epsilon = 1e-10);
        assert_relative_eq!(plus.prob_one(), 0.5, epsilon = 1e-10);
        assert!(plus.is_normalized());
    }

    #[test]
    fn test_bloch_conversion() {
        let qubit = Qubit::plus();
        let bloch = qubit.to_bloch();
        let recovered = Qubit::from_bloch(bloch);

        assert_relative_eq!(qubit.fidelity(&recovered), 1.0, epsilon = 1e-10);
    }
}
