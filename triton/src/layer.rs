//! Layer management for spiral search.

use qops_core::Signature5D;
use serde::{Deserialize, Serialize};

/// Metrics for a single search layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerMetrics {
    /// Layer index
    pub layer: usize,
    /// Number of points evaluated
    pub points_evaluated: usize,
    /// Best score in layer
    pub best_score: f64,
    /// Average score in layer
    pub avg_score: f64,
    /// Score variance
    pub variance: f64,
    /// Number of improvements found
    pub improvements: usize,
    /// Layer radius
    pub radius: f64,
}

impl Default for LayerMetrics {
    fn default() -> Self {
        Self {
            layer: 0,
            points_evaluated: 0,
            best_score: 0.0,
            avg_score: 0.0,
            variance: 0.0,
            improvements: 0,
            radius: 0.0,
        }
    }
}

/// Search layer containing points and their scores
#[derive(Debug, Clone)]
pub struct SearchLayer {
    /// Layer index
    pub index: usize,
    /// Layer radius
    pub radius: f64,
    /// Points in layer
    points: Vec<Signature5D>,
    /// Scores for each point
    scores: Vec<f64>,
    /// Best point index
    best_index: Option<usize>,
    /// Metrics for this layer
    metrics: LayerMetrics,
}

impl SearchLayer {
    /// Create new layer
    pub fn new(index: usize, radius: f64) -> Self {
        Self {
            index,
            radius,
            points: Vec::new(),
            scores: Vec::new(),
            best_index: None,
            metrics: LayerMetrics {
                layer: index,
                radius,
                ..Default::default()
            },
        }
    }

    /// Add a point with its score
    pub fn add_point(&mut self, point: Signature5D, score: f64) {
        let idx = self.points.len();
        self.points.push(point);
        self.scores.push(score);

        // Update best
        if self.best_index.is_none() || score > self.scores[self.best_index.unwrap()] {
            self.best_index = Some(idx);
        }

        // Update metrics
        self.update_metrics();
    }

    /// Update layer metrics
    fn update_metrics(&mut self) {
        self.metrics.points_evaluated = self.scores.len();

        if self.scores.is_empty() {
            return;
        }

        self.metrics.best_score = self.best_index
            .map(|i| self.scores[i])
            .unwrap_or(0.0);

        let sum: f64 = self.scores.iter().sum();
        self.metrics.avg_score = sum / self.scores.len() as f64;

        // Compute variance
        let var_sum: f64 = self.scores
            .iter()
            .map(|&s| (s - self.metrics.avg_score).powi(2))
            .sum();
        self.metrics.variance = var_sum / self.scores.len() as f64;
    }

    /// Get best point
    pub fn best_point(&self) -> Option<&Signature5D> {
        self.best_index.map(|i| &self.points[i])
    }

    /// Get best score
    pub fn best_score(&self) -> f64 {
        self.metrics.best_score
    }

    /// Get metrics
    pub fn metrics(&self) -> &LayerMetrics {
        &self.metrics
    }

    /// Get all points
    pub fn points(&self) -> &[Signature5D] {
        &self.points
    }

    /// Get all scores
    pub fn scores(&self) -> &[f64] {
        &self.scores
    }

    /// Get top N points
    pub fn top_n(&self, n: usize) -> Vec<(Signature5D, f64)> {
        let mut indexed: Vec<(usize, f64)> = self.scores
            .iter()
            .enumerate()
            .map(|(i, &s)| (i, s))
            .collect();

        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        indexed
            .into_iter()
            .take(n)
            .map(|(i, s)| (self.points[i], s))
            .collect()
    }
}

/// Manager for multiple search layers
#[derive(Debug, Clone)]
pub struct LayerManager {
    layers: Vec<SearchLayer>,
    current_layer: usize,
    global_best: Option<(Signature5D, f64)>,
    improvements_per_layer: Vec<usize>,
}

impl LayerManager {
    /// Create new layer manager
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            current_layer: 0,
            global_best: None,
            improvements_per_layer: Vec::new(),
        }
    }

    /// Start a new layer
    pub fn start_layer(&mut self, radius: f64) {
        let index = self.layers.len();
        self.layers.push(SearchLayer::new(index, radius));
        self.current_layer = index;
        self.improvements_per_layer.push(0);
    }

    /// Record a point in current layer
    pub fn record(&mut self, point: Signature5D, score: f64) {
        if self.layers.is_empty() {
            self.start_layer(0.1);
        }

        let prev_best = self.global_best.map(|(_, s)| s).unwrap_or(0.0);

        self.layers[self.current_layer].add_point(point, score);

        // Update global best
        if self.global_best.is_none() || score > prev_best {
            self.global_best = Some((point, score));

            if self.current_layer < self.improvements_per_layer.len() {
                self.improvements_per_layer[self.current_layer] += 1;
            }
        }
    }

    /// Get global best
    pub fn global_best(&self) -> Option<(Signature5D, f64)> {
        self.global_best
    }

    /// Get current layer
    pub fn current(&self) -> Option<&SearchLayer> {
        self.layers.get(self.current_layer)
    }

    /// Get all layers
    pub fn layers(&self) -> &[SearchLayer] {
        &self.layers
    }

    /// Get metrics for all layers
    pub fn all_metrics(&self) -> Vec<LayerMetrics> {
        self.layers.iter().map(|l| l.metrics().clone()).collect()
    }

    /// Get layer count
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// Total points evaluated
    pub fn total_points(&self) -> usize {
        self.layers.iter().map(|l| l.points().len()).sum()
    }

    /// Find best layer (by average score)
    pub fn best_layer_by_avg(&self) -> Option<usize> {
        self.layers
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.metrics().avg_score.partial_cmp(&b.metrics().avg_score).unwrap()
            })
            .map(|(i, _)| i)
    }

    /// Get convergence trend
    pub fn convergence_trend(&self) -> Vec<f64> {
        self.layers.iter().map(|l| l.best_score()).collect()
    }
}

impl Default for LayerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_layer() {
        let mut layer = SearchLayer::new(0, 0.1);

        layer.add_point(Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5), 0.7);
        layer.add_point(Signature5D::new(0.6, 0.6, 0.6, 0.6, 0.6), 0.8);
        layer.add_point(Signature5D::new(0.4, 0.4, 0.4, 0.4, 0.4), 0.6);

        assert_eq!(layer.best_score(), 0.8);
        assert_eq!(layer.points().len(), 3);
    }

    #[test]
    fn test_layer_manager() {
        let mut manager = LayerManager::new();

        manager.start_layer(0.1);
        manager.record(Signature5D::new(0.5, 0.5, 0.5, 0.5, 0.5), 0.7);

        manager.start_layer(0.2);
        manager.record(Signature5D::new(0.6, 0.6, 0.6, 0.6, 0.6), 0.9);

        assert_eq!(manager.layer_count(), 2);
        assert_eq!(manager.global_best().unwrap().1, 0.9);
    }

    #[test]
    fn test_top_n() {
        let mut layer = SearchLayer::new(0, 0.1);

        for i in 0..10 {
            let score = i as f64 / 10.0;
            layer.add_point(Signature5D::new(score, score, score, score, score), score);
        }

        let top = layer.top_n(3);
        assert_eq!(top.len(), 3);
        assert_eq!(top[0].1, 0.9);
    }
}
