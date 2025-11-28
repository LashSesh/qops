//! Shor's Algorithm - Integer Factorization
//!
//! Provides exponential speedup for factoring large integers.
//!
//! ## Complexity
//! - Classical (best known): O(exp(n^{1/3}))
//! - Quantum: O(n³)
//!
//! ## Algorithm
//! 1. Choose random a < N
//! 2. Compute gcd(a, N). If > 1, done.
//! 3. Use quantum period finding to get order r of a mod N
//! 4. If r is even and a^{r/2} ≠ -1 (mod N):
//!    - gcd(a^{r/2} ± 1, N) are factors

use crate::{AlgorithmError, Result, QuantumPhaseEstimation};
use qops_circuits::{Circuit, Gate, QuantumRegister, Measurement};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Factorization method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorizationMethod {
    /// Full quantum Shor's algorithm
    Full,
    /// Classical simulation of the quantum part
    Simulated,
    /// Hybrid classical-quantum approach
    Hybrid,
}

/// Result of Shor's algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShorResult {
    /// The number that was factored
    pub n: u64,
    /// Found factors (may be trivial)
    pub factors: Vec<u64>,
    /// The base 'a' used in period finding
    pub base: u64,
    /// The period/order found
    pub period: Option<u64>,
    /// Number of attempts made
    pub attempts: usize,
    /// Whether factorization was successful
    pub success: bool,
    /// Method used
    pub method: FactorizationMethod,
}

impl ShorResult {
    /// Check if factors are non-trivial
    pub fn is_nontrivial(&self) -> bool {
        self.factors.len() >= 2 &&
        self.factors.iter().all(|&f| f > 1 && f < self.n)
    }

    /// Verify the factorization
    pub fn verify(&self) -> bool {
        if self.factors.is_empty() {
            return false;
        }
        self.factors.iter().product::<u64>() == self.n
    }
}

/// Shor's factorization algorithm
pub struct Shor {
    /// Number to factor
    pub n: u64,
    /// Factorization method
    pub method: FactorizationMethod,
    /// Maximum number of attempts
    pub max_attempts: usize,
    /// Number of qubits for precision
    pub precision_qubits: usize,
}

impl Shor {
    /// Create a new Shor's algorithm instance
    pub fn new(n: u64) -> Self {
        // Precision qubits: need 2*log2(N) + 1 for reliable estimation
        let precision = 2 * (n as f64).log2().ceil() as usize + 1;

        Self {
            n,
            method: FactorizationMethod::Simulated,
            max_attempts: 10,
            precision_qubits: precision.min(20), // Cap for simulation
        }
    }

    /// Set the factorization method
    pub fn with_method(mut self, method: FactorizationMethod) -> Self {
        self.method = method;
        self
    }

    /// Set maximum attempts
    pub fn with_max_attempts(mut self, max: usize) -> Self {
        self.max_attempts = max;
        self
    }

    /// Run Shor's algorithm
    pub fn run(&self) -> ShorResult {
        // Check for trivial cases
        if self.n < 2 {
            return ShorResult {
                n: self.n,
                factors: vec![self.n],
                base: 0,
                period: None,
                attempts: 0,
                success: false,
                method: self.method,
            };
        }

        if self.n % 2 == 0 {
            return ShorResult {
                n: self.n,
                factors: vec![2, self.n / 2],
                base: 0,
                period: None,
                attempts: 0,
                success: true,
                method: self.method,
            };
        }

        // Check if N is a prime power
        if let Some(factors) = self.check_prime_power() {
            return ShorResult {
                n: self.n,
                factors,
                base: 0,
                period: None,
                attempts: 0,
                success: true,
                method: self.method,
            };
        }

        // Main Shor's algorithm loop
        let mut rng = rand::thread_rng();

        for attempt in 0..self.max_attempts {
            // Choose random a in [2, N-1]
            let a = rng.gen_range(2..self.n);

            // Check if we got lucky with gcd
            let g = gcd(a, self.n);
            if g > 1 && g < self.n {
                return ShorResult {
                    n: self.n,
                    factors: vec![g, self.n / g],
                    base: a,
                    period: None,
                    attempts: attempt + 1,
                    success: true,
                    method: self.method,
                };
            }

            // Find the period of a^x mod N
            let period = match self.method {
                FactorizationMethod::Full => self.quantum_period_finding(a),
                FactorizationMethod::Simulated => self.simulated_period_finding(a),
                FactorizationMethod::Hybrid => self.hybrid_period_finding(a),
            };

            if let Some(r) = period {
                if r > 0 && r % 2 == 0 {
                    // Try to extract factors
                    let ar2 = mod_pow(a, r / 2, self.n);

                    if ar2 != self.n - 1 {
                        let factor1 = gcd(ar2 + 1, self.n);
                        let factor2 = gcd(ar2 + self.n - 1, self.n); // ar2 - 1 mod n

                        if factor1 > 1 && factor1 < self.n {
                            return ShorResult {
                                n: self.n,
                                factors: vec![factor1, self.n / factor1],
                                base: a,
                                period: Some(r),
                                attempts: attempt + 1,
                                success: true,
                                method: self.method,
                            };
                        }

                        if factor2 > 1 && factor2 < self.n {
                            return ShorResult {
                                n: self.n,
                                factors: vec![factor2, self.n / factor2],
                                base: a,
                                period: Some(r),
                                attempts: attempt + 1,
                                success: true,
                                method: self.method,
                            };
                        }
                    }
                }
            }
        }

        // Failed to factor
        ShorResult {
            n: self.n,
            factors: vec![self.n],
            base: 0,
            period: None,
            attempts: self.max_attempts,
            success: false,
            method: self.method,
        }
    }

    /// Check if N is a prime power (a^k for some prime a)
    fn check_prime_power(&self) -> Option<Vec<u64>> {
        let n = self.n;
        for k in 2..=((n as f64).log2().ceil() as u32) {
            let root = (n as f64).powf(1.0 / k as f64).round() as u64;

            for candidate in [root.saturating_sub(1), root, root + 1] {
                if candidate > 1 && candidate.pow(k) == n {
                    return Some(vec![candidate; k as usize]);
                }
            }
        }
        None
    }

    /// Quantum period finding (full implementation)
    fn quantum_period_finding(&self, a: u64) -> Option<u64> {
        // This would require a full modular exponentiation circuit
        // For now, fall back to simulation
        self.simulated_period_finding(a)
    }

    /// Simulated quantum period finding
    fn simulated_period_finding(&self, a: u64) -> Option<u64> {
        let n = self.n;

        // Classical simulation: find the true period
        let mut current = a;
        for r in 1..=n {
            if current == 1 {
                return Some(r);
            }
            current = (current * a) % n;
        }

        None
    }

    /// Hybrid period finding (quantum simulation + classical post-processing)
    fn hybrid_period_finding(&self, a: u64) -> Option<u64> {
        // Simulate phase estimation
        let true_period = self.simulated_period_finding(a)?;
        let phase = 1.0 / true_period as f64;

        // Simulate noisy measurement
        let precision = self.precision_qubits;
        let mut rng = rand::thread_rng();

        // Add some noise to simulate quantum measurement
        let noise: f64 = rng.gen_range(-0.5..0.5) / (1 << precision) as f64;
        let measured_phase = phase + noise;

        // Use continued fractions to recover period
        self.continued_fractions(measured_phase, self.n)
    }

    /// Extract period from phase using continued fractions
    fn continued_fractions(&self, phase: f64, max_denominator: u64) -> Option<u64> {
        // Continued fractions algorithm
        let mut coefficients = Vec::new();
        let mut x = phase;

        for _ in 0..20 {
            let a = x.floor() as i64;
            coefficients.push(a);

            let frac = x - a as f64;
            if frac.abs() < 1e-10 {
                break;
            }
            x = 1.0 / frac;
        }

        // Convert back to fraction
        let mut p_prev = 1i64;
        let mut p_curr = coefficients[0];
        let mut q_prev = 0i64;
        let mut q_curr = 1i64;

        for &a in coefficients.iter().skip(1) {
            let p_next = a * p_curr + p_prev;
            let q_next = a * q_curr + q_prev;

            p_prev = p_curr;
            p_curr = p_next;
            q_prev = q_curr;
            q_curr = q_next;

            if q_curr as u64 > max_denominator {
                break;
            }
        }

        if q_curr > 0 && (q_curr as u64) <= max_denominator {
            Some(q_curr as u64)
        } else if q_prev > 0 {
            Some(q_prev as u64)
        } else {
            None
        }
    }
}

/// Greatest common divisor
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Modular exponentiation: a^exp mod m
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1u64;
    base %= modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp /= 2;
        base = (base * base) % modulus;
    }

    result
}

/// Order finding circuit builder
pub struct OrderFindingCircuit {
    /// Number to find order in
    pub n: u64,
    /// Base for order finding
    pub a: u64,
    /// Number of precision qubits
    pub precision_qubits: usize,
}

impl OrderFindingCircuit {
    pub fn new(n: u64, a: u64, precision_qubits: usize) -> Self {
        Self { n, a, precision_qubits }
    }

    /// Build the order finding circuit (simplified version)
    pub fn build(&self) -> Circuit {
        let work_qubits = (self.n as f64).log2().ceil() as usize;
        let total = self.precision_qubits + work_qubits;

        let mut circuit = Circuit::with_name(total, "OrderFinding");

        // Initialize precision register in superposition
        for i in 0..self.precision_qubits {
            circuit = circuit.h(i);
        }

        // Initialize work register to |1⟩
        circuit = circuit.x(self.precision_qubits);

        // Controlled modular exponentiation would go here
        // This is extremely complex and typically decomposed into
        // many elementary gates. For simulation, we use phase kickback.

        // Placeholder: apply phase kickback based on order
        // In a real implementation, this would be controlled-U^{2^k}
        // where U|y⟩ = |ay mod N⟩

        // Inverse QFT on precision register
        for i in 0..self.precision_qubits {
            for j in 0..i {
                let theta = -PI / (1 << (i - j)) as f64;
                circuit = circuit.cphase(theta, j, i);
            }
            circuit = circuit.h(i);
        }

        circuit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor_15() {
        let shor = Shor::new(15).with_method(FactorizationMethod::Simulated);
        let result = shor.run();

        assert!(result.success);
        assert!(result.verify());
        assert!(result.factors.contains(&3) || result.factors.contains(&5));
    }

    #[test]
    fn test_factor_21() {
        let shor = Shor::new(21).with_method(FactorizationMethod::Simulated);
        let result = shor.run();

        assert!(result.success);
        assert!(result.verify());
    }

    #[test]
    fn test_factor_even() {
        let shor = Shor::new(12);
        let result = shor.run();

        assert!(result.success);
        assert_eq!(result.factors[0], 2);
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(7, 2, 15), 4);
        assert_eq!(mod_pow(3, 5, 7), 5);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(15, 21), 3);
        assert_eq!(gcd(17, 13), 1);
    }
}
