//! Refinement engine for multi-pass optimization.

use crate::config::RefinementConfig;
use crate::scoring::ScoringFunction;
use qops_core::{Signature5D, resonance_5d, resonance_gradient};
use serde::{Deserialize, Serialize};
use rand::Rng;

/// Result of a refinement pass
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefinementResult {
    /// Pass number
    pub pass: usize,
    /// Initial score
    pub initial_score: f64,
    /// Final score after refinement
    pub final_score: f64,
    /// Improvement achieved
    pub improvement: f64,
    /// Iterations performed
    pub iterations: usize,
    /// Final signature
    pub signature: [f64; 5],
    /// Search radius used
    pub radius: f64,
}

/// A single refinement pass
#[derive(Debug, Clone)]
pub struct RefinementPass {
    pub index: usize,
    pub radius: f64,
    pub iterations: usize,
    pub use_gradient: bool,
    pub gradient_step: f64,
}

impl RefinementPass {
    /// Create new refinement pass
    pub fn new(index: usize, radius: f64, iterations: usize) -> Self {
        Self {
            index,
            radius,
            iterations,
            use_gradient: true,
            gradient_step: 0.01,
        }
    }
}

/// Refinement engine for local optimization
#[derive(Debug, Clone)]
pub struct RefinementEngine {
    config: RefinementConfig,
    rng: rand::rngs::StdRng,
    passes: Vec<RefinementPass>,
    results: Vec<RefinementResult>,
}

impl RefinementEngine {
    /// Create new refinement engine
    pub fn new(config: RefinementConfig) -> Self {
        Self::with_seed(config, rand::random())
    }

    /// Create with specific seed
    pub fn with_seed(config: RefinementConfig, seed: u64) -> Self {
        use rand::SeedableRng;

        let mut passes = Vec::new();
        let mut radius = 0.1;

        for i in 0..config.passes {
            passes.push(RefinementPass {
                index: i,
                radius,
                iterations: config.local_iterations,
                use_gradient: config.gradient_refinement,
                gradient_step: config.gradient_step,
            });
            radius *= config.shrink_factor;
        }

        Self {
            config,
            rng: rand::rngs::StdRng::seed_from_u64(seed),
            passes,
            results: Vec::new(),
        }
    }

    /// Run all refinement passes on a signature
    pub fn refine(&mut self, sig: &Signature5D) -> Signature5D {
        let mut current = *sig;

        for pass in &self.passes.clone() {
            let result = self.run_pass(pass, &current);
            current = Signature5D::new(
                result.signature[0],
                result.signature[1],
                result.signature[2],
                result.signature[3],
                result.signature[4],
            );
            self.results.push(result);
        }

        current
    }

    /// Run all refinement passes with custom scoring
    pub fn refine_with_scorer<S: ScoringFunction>(
        &mut self,
        sig: &Signature5D,
        scorer: &S,
    ) -> Signature5D {
        let mut current = *sig;

        for pass in &self.passes.clone() {
            let result = self.run_pass_with_scorer(pass, &current, scorer);
            current = Signature5D::new(
                result.signature[0],
                result.signature[1],
                result.signature[2],
                result.signature[3],
                result.signature[4],
            );
            self.results.push(result);
        }

        current
    }

    /// Run a single refinement pass
    fn run_pass(&mut self, pass: &RefinementPass, sig: &Signature5D) -> RefinementResult {
        let initial_score = resonance_5d(sig);
        let mut best = *sig;
        let mut best_score = initial_score;
        let mut iterations = 0;

        for _ in 0..pass.iterations {
            iterations += 1;

            // Try local perturbation
            let candidate = if pass.use_gradient && self.rng.gen::<f64>() > 0.3 {
                self.gradient_step(&best, pass.gradient_step)
            } else {
                self.random_perturbation(&best, pass.radius)
            };

            let score = resonance_5d(&candidate);

            if score > best_score {
                best = candidate;
                best_score = score;
            }
        }

        RefinementResult {
            pass: pass.index,
            initial_score,
            final_score: best_score,
            improvement: best_score - initial_score,
            iterations,
            signature: [best.psi, best.rho, best.omega, best.chi, best.eta],
            radius: pass.radius,
        }
    }

    /// Run a single refinement pass with custom scorer
    fn run_pass_with_scorer<S: ScoringFunction>(
        &mut self,
        pass: &RefinementPass,
        sig: &Signature5D,
        scorer: &S,
    ) -> RefinementResult {
        let initial_score = scorer.score(sig);
        let mut best = *sig;
        let mut best_score = initial_score;
        let mut iterations = 0;

        for _ in 0..pass.iterations {
            iterations += 1;

            let candidate = if pass.use_gradient && self.rng.gen::<f64>() > 0.3 {
                self.gradient_step(&best, pass.gradient_step)
            } else {
                self.random_perturbation(&best, pass.radius)
            };

            let score = scorer.score(&candidate);

            if score > best_score {
                best = candidate;
                best_score = score;
            }
        }

        RefinementResult {
            pass: pass.index,
            initial_score,
            final_score: best_score,
            improvement: best_score - initial_score,
            iterations,
            signature: [best.psi, best.rho, best.omega, best.chi, best.eta],
            radius: pass.radius,
        }
    }

    /// Take gradient step
    fn gradient_step(&self, sig: &Signature5D, step: f64) -> Signature5D {
        let grad = resonance_gradient(sig, 0.001);

        Signature5D::new(
            (sig.psi + step * grad[0]).clamp(0.0, 1.0),
            (sig.rho + step * grad[1]).clamp(0.0, 1.0),
            (sig.omega + step * grad[2]).clamp(0.0, 1.0),
            (sig.chi + step * grad[3]).clamp(0.0, 1.0),
            (sig.eta + step * grad[4]).clamp(0.0, 1.0),
        )
    }

    /// Random perturbation within radius
    fn random_perturbation(&mut self, sig: &Signature5D, radius: f64) -> Signature5D {
        Signature5D::new(
            (sig.psi + self.rng.gen_range(-radius..radius)).clamp(0.0, 1.0),
            (sig.rho + self.rng.gen_range(-radius..radius)).clamp(0.0, 1.0),
            (sig.omega + self.rng.gen_range(-radius..radius)).clamp(0.0, 1.0),
            (sig.chi + self.rng.gen_range(-radius * 0.5..radius * 0.5)).clamp(0.0, 1.0),
            (sig.eta + self.rng.gen_range(-radius * 0.3..radius * 0.3)).clamp(0.0, 1.0),
        )
    }

    /// Get refinement results
    pub fn results(&self) -> &[RefinementResult] {
        &self.results
    }

    /// Get total improvement
    pub fn total_improvement(&self) -> f64 {
        self.results.iter().map(|r| r.improvement).sum()
    }

    /// Reset engine
    pub fn reset(&mut self) {
        self.results.clear();
    }
}

impl Default for RefinementEngine {
    fn default() -> Self {
        Self::new(RefinementConfig::default())
    }
}

/// Nelder-Mead simplex refinement
pub struct SimplexRefiner {
    alpha: f64, // reflection
    gamma: f64, // expansion
    rho: f64,   // contraction
    sigma: f64, // shrink
}

impl SimplexRefiner {
    /// Create new simplex refiner
    pub fn new() -> Self {
        Self {
            alpha: 1.0,
            gamma: 2.0,
            rho: 0.5,
            sigma: 0.5,
        }
    }

    /// Refine a signature using Nelder-Mead
    pub fn refine<F>(&self, sig: &Signature5D, max_iter: usize, scorer: F) -> Signature5D
    where
        F: Fn(&Signature5D) -> f64,
    {
        // Initialize simplex around starting point
        let mut simplex: Vec<(Signature5D, f64)> = Vec::new();

        // Starting point
        simplex.push((*sig, scorer(sig)));

        // Additional vertices
        let delta = 0.05;
        let mut current = *sig;
        for i in 0..5 {
            let mut perturbed = current;
            match i {
                0 => perturbed.psi = (perturbed.psi + delta).min(1.0),
                1 => perturbed.rho = (perturbed.rho + delta).min(1.0),
                2 => perturbed.omega = (perturbed.omega + delta).min(1.0),
                3 => perturbed.chi = (perturbed.chi + delta).min(1.0),
                4 => perturbed.eta = (perturbed.eta + delta).min(1.0),
                _ => {}
            }
            simplex.push((perturbed, scorer(&perturbed)));
        }

        // Main loop
        for _ in 0..max_iter {
            // Sort by score (descending - we want to maximize)
            simplex.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            // Check convergence
            let spread = simplex.first().unwrap().1 - simplex.last().unwrap().1;
            if spread < 1e-8 {
                break;
            }

            // Centroid of all points except worst
            let centroid = self.compute_centroid(&simplex[..simplex.len() - 1]);

            // Reflection
            let worst = &simplex.last().unwrap().0;
            let reflected = self.reflect(&centroid, worst);
            let reflected_score = scorer(&reflected);

            if reflected_score > simplex.first().unwrap().1 {
                // Try expansion
                let expanded = self.expand(&centroid, &reflected);
                let expanded_score = scorer(&expanded);

                if expanded_score > reflected_score {
                    simplex.pop();
                    simplex.push((expanded, expanded_score));
                } else {
                    simplex.pop();
                    simplex.push((reflected, reflected_score));
                }
            } else if reflected_score > simplex[simplex.len() - 2].1 {
                simplex.pop();
                simplex.push((reflected, reflected_score));
            } else {
                // Contraction
                let contracted = self.contract(&centroid, worst);
                let contracted_score = scorer(&contracted);

                if contracted_score > simplex.last().unwrap().1 {
                    simplex.pop();
                    simplex.push((contracted, contracted_score));
                } else {
                    // Shrink
                    let best = simplex.first().unwrap().0;
                    for i in 1..simplex.len() {
                        let shrunk = self.shrink(&best, &simplex[i].0);
                        simplex[i] = (shrunk, scorer(&shrunk));
                    }
                }
            }
        }

        simplex.first().unwrap().0
    }

    fn compute_centroid(&self, points: &[(Signature5D, f64)]) -> Signature5D {
        let n = points.len() as f64;
        let mut sum = Signature5D::new(0.0, 0.0, 0.0, 0.0, 0.0);

        for (sig, _) in points {
            sum.psi += sig.psi / n;
            sum.rho += sig.rho / n;
            sum.omega += sig.omega / n;
            sum.chi += sig.chi / n;
            sum.eta += sig.eta / n;
        }

        sum
    }

    fn reflect(&self, centroid: &Signature5D, worst: &Signature5D) -> Signature5D {
        Signature5D::new(
            (centroid.psi + self.alpha * (centroid.psi - worst.psi)).clamp(0.0, 1.0),
            (centroid.rho + self.alpha * (centroid.rho - worst.rho)).clamp(0.0, 1.0),
            (centroid.omega + self.alpha * (centroid.omega - worst.omega)).clamp(0.0, 1.0),
            (centroid.chi + self.alpha * (centroid.chi - worst.chi)).clamp(0.0, 1.0),
            (centroid.eta + self.alpha * (centroid.eta - worst.eta)).clamp(0.0, 1.0),
        )
    }

    fn expand(&self, centroid: &Signature5D, reflected: &Signature5D) -> Signature5D {
        Signature5D::new(
            (centroid.psi + self.gamma * (reflected.psi - centroid.psi)).clamp(0.0, 1.0),
            (centroid.rho + self.gamma * (reflected.rho - centroid.rho)).clamp(0.0, 1.0),
            (centroid.omega + self.gamma * (reflected.omega - centroid.omega)).clamp(0.0, 1.0),
            (centroid.chi + self.gamma * (reflected.chi - centroid.chi)).clamp(0.0, 1.0),
            (centroid.eta + self.gamma * (reflected.eta - centroid.eta)).clamp(0.0, 1.0),
        )
    }

    fn contract(&self, centroid: &Signature5D, worst: &Signature5D) -> Signature5D {
        Signature5D::new(
            (centroid.psi + self.rho * (worst.psi - centroid.psi)).clamp(0.0, 1.0),
            (centroid.rho + self.rho * (worst.rho - centroid.rho)).clamp(0.0, 1.0),
            (centroid.omega + self.rho * (worst.omega - centroid.omega)).clamp(0.0, 1.0),
            (centroid.chi + self.rho * (worst.chi - centroid.chi)).clamp(0.0, 1.0),
            (centroid.eta + self.rho * (worst.eta - centroid.eta)).clamp(0.0, 1.0),
        )
    }

    fn shrink(&self, best: &Signature5D, point: &Signature5D) -> Signature5D {
        Signature5D::new(
            best.psi + self.sigma * (point.psi - best.psi),
            best.rho + self.sigma * (point.rho - best.rho),
            best.omega + self.sigma * (point.omega - best.omega),
            best.chi + self.sigma * (point.chi - best.chi),
            best.eta + self.sigma * (point.eta - best.eta),
        )
    }
}

impl Default for SimplexRefiner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refinement_engine() {
        let config = RefinementConfig {
            passes: 2,
            local_iterations: 10,
            ..Default::default()
        };
        let mut engine = RefinementEngine::new(config);
        let sig = Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5);

        let refined = engine.refine(&sig);
        assert!(resonance_5d(&refined) >= resonance_5d(&sig));
    }

    #[test]
    fn test_simplex_refiner() {
        let refiner = SimplexRefiner::new();
        let sig = Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5);

        let refined = refiner.refine(&sig, 50, resonance_5d);
        assert!(resonance_5d(&refined) >= resonance_5d(&sig));
    }
}
