//! Scoring functions and caching for TRITON.

use qops_core::{Signature5D, resonance_5d};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for scoring functions
pub trait ScoringFunction: Send + Sync {
    /// Compute score for a signature
    fn score(&self, sig: &Signature5D) -> f64;

    /// Get scoring function name
    fn name(&self) -> &str;
}

/// Default resonance scoring
pub struct ResonanceScorer;

impl ScoringFunction for ResonanceScorer {
    fn score(&self, sig: &Signature5D) -> f64 {
        resonance_5d(sig)
    }

    fn name(&self) -> &str {
        "resonance"
    }
}

/// Weighted scoring function
pub struct WeightedScorer {
    weights: [f64; 5],
    name: String,
}

impl WeightedScorer {
    /// Create new weighted scorer
    pub fn new(weights: [f64; 5]) -> Self {
        Self {
            weights,
            name: "weighted".to_string(),
        }
    }

    /// Quality-focused scorer
    pub fn quality_focused() -> Self {
        Self::new([0.6, 0.2, 0.2, 0.0, 0.0])
    }

    /// Stability-focused scorer
    pub fn stability_focused() -> Self {
        Self::new([0.2, 0.6, 0.2, 0.0, 0.0])
    }

    /// Balanced scorer
    pub fn balanced() -> Self {
        Self::new([0.35, 0.25, 0.25, 0.1, 0.05])
    }
}

impl ScoringFunction for WeightedScorer {
    fn score(&self, sig: &Signature5D) -> f64 {
        let values = [sig.psi, sig.rho, sig.omega, sig.chi, sig.eta];
        let score: f64 = self.weights.iter()
            .zip(values.iter())
            .map(|(w, v)| w * v)
            .sum();
        score.clamp(0.0, 1.0)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Composite score combining multiple scoring functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeScore {
    /// Individual scores
    pub scores: HashMap<String, f64>,
    /// Combined score
    pub combined: f64,
    /// Signature that was scored
    pub signature: [f64; 5],
}

impl CompositeScore {
    /// Create from single score
    pub fn single(sig: &Signature5D, score: f64, name: &str) -> Self {
        let mut scores = HashMap::new();
        scores.insert(name.to_string(), score);

        Self {
            scores,
            combined: score,
            signature: [sig.psi, sig.rho, sig.omega, sig.chi, sig.eta],
        }
    }

    /// Create from multiple scores
    pub fn multi(sig: &Signature5D, scores: Vec<(&str, f64)>, weights: &[f64]) -> Self {
        let mut score_map = HashMap::new();
        let mut combined = 0.0;
        let total_weight: f64 = weights.iter().sum();

        for ((name, score), weight) in scores.iter().zip(weights.iter()) {
            score_map.insert(name.to_string(), *score);
            combined += score * weight / total_weight;
        }

        Self {
            scores: score_map,
            combined,
            signature: [sig.psi, sig.rho, sig.omega, sig.chi, sig.eta],
        }
    }
}

/// Cache for scores to avoid recomputation
#[derive(Debug, Clone)]
pub struct ScoreCache {
    cache: HashMap<[u64; 5], f64>,
    hits: usize,
    misses: usize,
    max_size: usize,
}

impl ScoreCache {
    /// Create new cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
            max_size,
        }
    }

    /// Hash signature to cache key
    fn sig_to_key(sig: &Signature5D) -> [u64; 5] {
        [
            (sig.psi * 1e10) as u64,
            (sig.rho * 1e10) as u64,
            (sig.omega * 1e10) as u64,
            (sig.chi * 1e10) as u64,
            (sig.eta * 1e10) as u64,
        ]
    }

    /// Get cached score or compute and cache
    pub fn get_or_compute<F>(&mut self, sig: &Signature5D, compute: F) -> f64
    where
        F: FnOnce(&Signature5D) -> f64,
    {
        let key = Self::sig_to_key(sig);

        if let Some(&score) = self.cache.get(&key) {
            self.hits += 1;
            return score;
        }

        self.misses += 1;
        let score = compute(sig);

        // Evict oldest entries if cache is full
        if self.cache.len() >= self.max_size {
            // Simple strategy: remove random 10%
            let to_remove: Vec<_> = self.cache.keys().take(self.max_size / 10).cloned().collect();
            for k in to_remove {
                self.cache.remove(&k);
            }
        }

        self.cache.insert(key, score);
        score
    }

    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize, f64) {
        let total = self.hits + self.misses;
        let hit_rate = if total > 0 {
            self.hits as f64 / total as f64
        } else {
            0.0
        };
        (self.hits, self.misses, hit_rate)
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

impl Default for ScoreCache {
    fn default() -> Self {
        Self::new(100_000)
    }
}

/// Decomposition cost scorer (for operator mining)
pub struct DecompositionScorer {
    /// Target gate set size
    target_size: usize,
}

impl DecompositionScorer {
    /// Create new decomposition scorer
    pub fn new(target_size: usize) -> Self {
        Self { target_size }
    }
}

impl ScoringFunction for DecompositionScorer {
    fn score(&self, sig: &Signature5D) -> f64 {
        // Score based on how well the signature suggests efficient decomposition
        // Lower eta (fluctuation) is better for stable decomposition
        // Higher omega (efficiency) is better
        let stability = 1.0 - sig.eta;
        let efficiency = sig.omega;
        let quality = sig.psi;

        // Combined score favoring stable, efficient decompositions
        0.4 * quality + 0.3 * efficiency + 0.3 * stability
    }

    fn name(&self) -> &str {
        "decomposition"
    }
}

/// Mandorla scorer (targets Mandorla zone)
pub struct MandorlaScorer {
    /// Target resonance
    target: f64,
}

impl MandorlaScorer {
    /// Create new Mandorla scorer
    pub fn new(target: f64) -> Self {
        Self { target }
    }
}

impl ScoringFunction for MandorlaScorer {
    fn score(&self, sig: &Signature5D) -> f64 {
        let res = resonance_5d(sig);

        // Bonus for being near target
        let distance = (res - self.target).abs();
        let proximity_bonus = (1.0 - distance * 2.0).max(0.0);

        // Check Mandorla conditions
        let product = sig.psi * sig.rho * sig.omega;
        let mandorla_check = product >= 0.5 && product <= 0.9;

        if mandorla_check && res >= 0.85 {
            1.0 // Perfect Mandorla
        } else if res >= 0.85 {
            0.9 + proximity_bonus * 0.1
        } else {
            res * 0.8 + proximity_bonus * 0.2
        }
    }

    fn name(&self) -> &str {
        "mandorla"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_scorer() {
        let scorer = ResonanceScorer;
        let sig = Signature5D::new(0.9, 0.8, 0.7, 0.6, 0.1);
        let score = scorer.score(&sig);
        assert!(score > 0.7);
    }

    #[test]
    fn test_weighted_scorer() {
        let scorer = WeightedScorer::quality_focused();
        let sig = Signature5D::new(1.0, 0.5, 0.5, 0.0, 0.0);
        let score = scorer.score(&sig);
        assert!(score > 0.7);
    }

    #[test]
    fn test_score_cache() {
        let mut cache = ScoreCache::new(1000);
        let sig = Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5);

        let score1 = cache.get_or_compute(&sig, resonance_5d);
        let score2 = cache.get_or_compute(&sig, resonance_5d);

        assert_eq!(score1, score2);

        let (hits, misses, rate) = cache.stats();
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
        assert!((rate - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_composite_score() {
        let sig = Signature5D::new(0.8, 0.7, 0.6, 0.5, 0.2);
        let scores = vec![
            ("quality", 0.8),
            ("stability", 0.7),
            ("efficiency", 0.6),
        ];
        let weights = [0.5, 0.3, 0.2];

        let composite = CompositeScore::multi(&sig, scores, &weights);
        assert!(composite.combined > 0.6);
    }
}
