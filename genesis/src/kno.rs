//! KNO Framework - Cyclic Conversion Operator system.

use num_complex::Complex64;

/// Double-Kick Operator
#[derive(Debug, Clone)]
pub struct DoubleKickOperator {
    pub coupling: f64,
    pub damping: f64,
}

impl DoubleKickOperator {
    pub fn new(coupling: f64, damping: f64) -> Self {
        Self { coupling, damping }
    }

    /// Apply kick to wave function
    pub fn apply(&self, psi: &WaveFunction) -> WaveFunction {
        let kicked: Vec<Complex64> = psi.amplitudes.iter()
            .map(|a| {
                let phase = (a.norm() * self.coupling).cos();
                a * Complex64::new(phase, 0.0) * self.damping
            })
            .collect();

        WaveFunction { amplitudes: kicked }
    }
}

impl Default for DoubleKickOperator {
    fn default() -> Self {
        Self::new(1.0, 0.99)
    }
}

/// Nullpunkt (Zero-point) Operator
#[derive(Debug, Clone)]
pub struct NullpunktOperator {
    pub epsilon: f64,
}

impl NullpunktOperator {
    pub fn new(epsilon: f64) -> Self {
        Self { epsilon }
    }

    /// Check if near zero-point
    pub fn is_near_nullpunkt(&self, value: f64) -> bool {
        value.abs() < self.epsilon
    }
}

impl Default for NullpunktOperator {
    fn default() -> Self {
        Self::new(1e-6)
    }
}

/// Wave function representation
#[derive(Debug, Clone)]
pub struct WaveFunction {
    pub amplitudes: Vec<Complex64>,
}

impl WaveFunction {
    /// Create new wave function
    pub fn new(size: usize) -> Self {
        Self {
            amplitudes: vec![Complex64::new(0.0, 0.0); size],
        }
    }

    /// Create basis state
    pub fn basis_state(size: usize, index: usize) -> Self {
        let mut amplitudes = vec![Complex64::new(0.0, 0.0); size];
        if index < size {
            amplitudes[index] = Complex64::new(1.0, 0.0);
        }
        Self { amplitudes }
    }

    /// Compute norm
    pub fn norm(&self) -> f64 {
        self.amplitudes.iter().map(|a| a.norm_sqr()).sum::<f64>().sqrt()
    }

    /// Normalize
    pub fn normalize(&mut self) {
        let n = self.norm();
        if n > 1e-10 {
            for a in &mut self.amplitudes {
                *a /= n;
            }
        }
    }

    /// Get probabilities
    pub fn probabilities(&self) -> Vec<f64> {
        self.amplitudes.iter().map(|a| a.norm_sqr()).collect()
    }
}

/// Berry connection for phase tracking
#[derive(Debug, Clone)]
pub struct BerryConnection {
    pub phase: f64,
    pub path: Vec<f64>,
}

impl BerryConnection {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            path: Vec::new(),
        }
    }

    /// Accumulate phase
    pub fn accumulate(&mut self, delta_phase: f64) {
        self.phase += delta_phase;
        self.path.push(self.phase);
    }

    /// Get geometric phase (Berry phase)
    pub fn berry_phase(&self) -> f64 {
        self.phase % (2.0 * std::f64::consts::PI)
    }
}

impl Default for BerryConnection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_kick() {
        let op = DoubleKickOperator::default();
        let psi = WaveFunction::basis_state(3, 0);

        let kicked = op.apply(&psi);
        assert!(kicked.norm() > 0.0);
    }

    #[test]
    fn test_wave_function() {
        let mut psi = WaveFunction::basis_state(5, 2);
        assert!((psi.norm() - 1.0).abs() < 1e-10);

        psi.normalize();
        let probs = psi.probabilities();
        assert!((probs[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_berry_connection() {
        let mut berry = BerryConnection::new();
        berry.accumulate(std::f64::consts::PI);
        berry.accumulate(std::f64::consts::PI);

        assert!((berry.berry_phase()).abs() < 1e-10);
    }
}
