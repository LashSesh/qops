//! Topology-aware biasing for TRITON search.

use qops_core::{Signature5D, resonance_5d};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Bias mode for topology-aware search
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiasMode {
    /// No bias, uniform exploration
    Uniform,
    /// Bias towards high-resonance regions
    ResonanceSeeking,
    /// Bias towards unexplored regions
    ExplorationSeeking,
    /// Bias towards cluster centers
    ClusterCentered,
    /// Bias towards boundary regions
    BoundaryExploring,
    /// Adaptive bias based on search progress
    Adaptive,
}

impl Default for BiasMode {
    fn default() -> Self {
        Self::Adaptive
    }
}

/// Weights for neighborhood exploration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeighborhoodWeights {
    /// Weight for resonance contribution
    pub resonance: f64,
    /// Weight for novelty contribution
    pub novelty: f64,
    /// Weight for clustering contribution
    pub clustering: f64,
    /// Weight for distance contribution
    pub distance: f64,
}

impl Default for NeighborhoodWeights {
    fn default() -> Self {
        Self {
            resonance: 0.4,
            novelty: 0.3,
            clustering: 0.2,
            distance: 0.1,
        }
    }
}

impl NeighborhoodWeights {
    /// Exploration-focused weights
    pub fn exploration() -> Self {
        Self {
            resonance: 0.2,
            novelty: 0.5,
            clustering: 0.2,
            distance: 0.1,
        }
    }

    /// Exploitation-focused weights
    pub fn exploitation() -> Self {
        Self {
            resonance: 0.6,
            novelty: 0.1,
            clustering: 0.2,
            distance: 0.1,
        }
    }
}

/// Topology bias manager
#[derive(Debug, Clone)]
pub struct TopologyBias {
    mode: BiasMode,
    weights: NeighborhoodWeights,
    visited: HashMap<[u64; 5], usize>,
    high_resonance_centers: Vec<Signature5D>,
    exploration_progress: f64,
}

impl TopologyBias {
    /// Create new topology bias
    pub fn new(mode: BiasMode) -> Self {
        Self {
            mode,
            weights: NeighborhoodWeights::default(),
            visited: HashMap::new(),
            high_resonance_centers: Vec::new(),
            exploration_progress: 0.0,
        }
    }

    /// Create with custom weights
    pub fn with_weights(mode: BiasMode, weights: NeighborhoodWeights) -> Self {
        Self {
            mode,
            weights,
            visited: HashMap::new(),
            high_resonance_centers: Vec::new(),
            exploration_progress: 0.0,
        }
    }

    /// Record a visited point
    pub fn record_visit(&mut self, sig: &Signature5D, score: f64) {
        let key = Self::sig_to_key(sig);
        *self.visited.entry(key).or_insert(0) += 1;

        // Track high resonance centers
        if score >= 0.8 {
            // Check if this is far from existing centers
            let dominated = self.high_resonance_centers.iter().any(|c| {
                Self::signature_distance(sig, c) < 0.1
            });

            if !dominated {
                self.high_resonance_centers.push(*sig);
                // Keep only top centers
                if self.high_resonance_centers.len() > 20 {
                    self.high_resonance_centers.remove(0);
                }
            }
        }
    }

    /// Update exploration progress
    pub fn update_progress(&mut self, progress: f64) {
        self.exploration_progress = progress;

        // Adaptive mode switches based on progress
        if self.mode == BiasMode::Adaptive {
            if progress < 0.3 {
                self.weights = NeighborhoodWeights::exploration();
            } else if progress > 0.7 {
                self.weights = NeighborhoodWeights::exploitation();
            } else {
                self.weights = NeighborhoodWeights::default();
            }
        }
    }

    /// Compute bias weight for a candidate point
    pub fn bias_weight(&self, sig: &Signature5D) -> f64 {
        match self.mode {
            BiasMode::Uniform => 1.0,
            BiasMode::ResonanceSeeking => self.resonance_bias(sig),
            BiasMode::ExplorationSeeking => self.exploration_bias(sig),
            BiasMode::ClusterCentered => self.cluster_bias(sig),
            BiasMode::BoundaryExploring => self.boundary_bias(sig),
            BiasMode::Adaptive => self.adaptive_bias(sig),
        }
    }

    fn resonance_bias(&self, sig: &Signature5D) -> f64 {
        // Higher resonance = higher bias
        resonance_5d(sig)
    }

    fn exploration_bias(&self, sig: &Signature5D) -> f64 {
        let key = Self::sig_to_key(sig);
        let visits = *self.visited.get(&key).unwrap_or(&0);

        // Less visited = higher bias
        1.0 / (1.0 + visits as f64)
    }

    fn cluster_bias(&self, sig: &Signature5D) -> f64 {
        if self.high_resonance_centers.is_empty() {
            return 1.0;
        }

        // Distance to nearest high-resonance center
        let min_dist = self.high_resonance_centers.iter()
            .map(|c| Self::signature_distance(sig, c))
            .fold(f64::MAX, f64::min);

        // Closer to center = higher bias (with decay)
        (-min_dist * 5.0).exp()
    }

    fn boundary_bias(&self, sig: &Signature5D) -> f64 {
        // Check if any component is near boundary
        let components = [sig.psi, sig.rho, sig.omega, sig.chi, sig.eta];
        let boundary_score: f64 = components.iter()
            .map(|&c| {
                let dist_low = c;
                let dist_high = 1.0 - c;
                dist_low.min(dist_high)
            })
            .sum();

        // Lower boundary distance = higher bias
        1.0 - (boundary_score / 5.0)
    }

    fn adaptive_bias(&self, sig: &Signature5D) -> f64 {
        let res = self.resonance_bias(sig);
        let exp = self.exploration_bias(sig);
        let cls = self.cluster_bias(sig);
        let dst = self.distance_to_best(sig);

        self.weights.resonance * res +
        self.weights.novelty * exp +
        self.weights.clustering * cls +
        self.weights.distance * dst
    }

    fn distance_to_best(&self, sig: &Signature5D) -> f64 {
        if self.high_resonance_centers.is_empty() {
            return 0.5;
        }

        let best = self.high_resonance_centers.last().unwrap();
        let dist = Self::signature_distance(sig, best);

        // Moderate distance is good (not too close, not too far)
        let optimal_dist = 0.2;
        1.0 - (dist - optimal_dist).abs().min(1.0)
    }

    fn sig_to_key(sig: &Signature5D) -> [u64; 5] {
        [
            (sig.psi * 100.0) as u64,
            (sig.rho * 100.0) as u64,
            (sig.omega * 100.0) as u64,
            (sig.chi * 100.0) as u64,
            (sig.eta * 100.0) as u64,
        ]
    }

    fn signature_distance(a: &Signature5D, b: &Signature5D) -> f64 {
        ((a.psi - b.psi).powi(2) +
         (a.rho - b.rho).powi(2) +
         (a.omega - b.omega).powi(2) +
         (a.chi - b.chi).powi(2) +
         (a.eta - b.eta).powi(2)).sqrt()
    }

    /// Get current mode
    pub fn mode(&self) -> BiasMode {
        self.mode
    }

    /// Set mode
    pub fn set_mode(&mut self, mode: BiasMode) {
        self.mode = mode;
    }

    /// Get high resonance centers
    pub fn centers(&self) -> &[Signature5D] {
        &self.high_resonance_centers
    }

    /// Get visit count for a signature
    pub fn visit_count(&self, sig: &Signature5D) -> usize {
        let key = Self::sig_to_key(sig);
        *self.visited.get(&key).unwrap_or(&0)
    }

    /// Total unique regions visited
    pub fn regions_visited(&self) -> usize {
        self.visited.len()
    }
}

impl Default for TopologyBias {
    fn default() -> Self {
        Self::new(BiasMode::Adaptive)
    }
}

/// Apply topology bias to a list of candidates
pub fn apply_topology_bias(
    candidates: &[(Signature5D, f64)],
    bias: &TopologyBias,
) -> Vec<(Signature5D, f64, f64)> {
    candidates.iter()
        .map(|(sig, score)| {
            let weight = bias.bias_weight(sig);
            let adjusted = score * weight;
            (*sig, *score, adjusted)
        })
        .collect()
}

/// Select best candidate considering bias
pub fn select_biased(
    candidates: &[(Signature5D, f64)],
    bias: &TopologyBias,
) -> Option<(Signature5D, f64)> {
    let weighted = apply_topology_bias(candidates, bias);

    weighted.iter()
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .map(|(sig, score, _)| (*sig, *score))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_bias() {
        let mut bias = TopologyBias::new(BiasMode::ExplorationSeeking);
        let sig = Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5);

        let w1 = bias.bias_weight(&sig);
        bias.record_visit(&sig, 0.7);
        let w2 = bias.bias_weight(&sig);

        assert!(w2 < w1); // Less weight after visit
    }

    #[test]
    fn test_adaptive_mode() {
        let mut bias = TopologyBias::new(BiasMode::Adaptive);

        bias.update_progress(0.1);
        assert_eq!(bias.weights.novelty, 0.5); // Exploration

        bias.update_progress(0.9);
        assert_eq!(bias.weights.resonance, 0.6); // Exploitation
    }

    #[test]
    fn test_select_biased() {
        let bias = TopologyBias::new(BiasMode::Uniform);
        let candidates = vec![
            (Signature5D::new(0.8, 0.8, 0.8, 0.5, 0.2), 0.8),
            (Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5), 0.5),
        ];

        let selected = select_biased(&candidates, &bias);
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().1, 0.8);
    }
}
