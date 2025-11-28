//! Mandorla field and resonance attractors.
//!
//! The Mandorla field is a 16-dimensional resonance field that:
//! - Records historical performance patterns
//! - Guides configuration changes
//! - Identifies stable attractor zones

use crate::signature::{Signature, Signature3D};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Vector in the Mandorla field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldVector {
    /// Components of the vector
    pub components: Vec<f64>,
    /// Dimension of the vector
    pub dimension: usize,
}

impl FieldVector {
    /// Create a new field vector
    pub fn new(dimension: usize) -> Self {
        Self {
            components: vec![0.0; dimension],
            dimension,
        }
    }

    /// Create from components
    pub fn from_components(components: Vec<f64>) -> Self {
        let dimension = components.len();
        Self { components, dimension }
    }

    /// Create a random vector
    pub fn random(dimension: usize) -> Self {
        let mut rng = rand::thread_rng();
        let components: Vec<f64> = (0..dimension).map(|_| rng.gen_range(0.0..1.0)).collect();
        Self { components, dimension }
    }

    /// Compute the magnitude (L2 norm)
    pub fn magnitude(&self) -> f64 {
        self.components.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }

    /// Normalize the vector
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 1e-10 {
            for c in &mut self.components {
                *c /= mag;
            }
        }
    }

    /// Dot product with another vector
    pub fn dot(&self, other: &Self) -> f64 {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    /// Euclidean distance to another vector
    pub fn distance(&self, other: &Self) -> f64 {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Add another vector (in-place)
    pub fn add(&mut self, other: &Self) {
        for (a, b) in self.components.iter_mut().zip(other.components.iter()) {
            *a += b;
        }
    }

    /// Scale the vector (in-place)
    pub fn scale(&mut self, factor: f64) {
        for c in &mut self.components {
            *c *= factor;
        }
    }

    /// Encode a signature into field dimensions
    pub fn encode_signature(sig: &Signature, dimension: usize) -> Self {
        let mut components = vec![0.0; dimension];
        let sig5d = sig.to_5d();

        // Map signature components to field dimensions
        if dimension >= 5 {
            components[0] = sig5d.psi;
            components[1] = sig5d.rho;
            components[2] = sig5d.omega;
            components[3] = sig5d.chi;
            components[4] = sig5d.eta;
        }

        // Fill remaining dimensions with derived values
        if dimension > 5 {
            components[5] = sig5d.product(); // ψ·ρ·ω
            if dimension > 6 {
                components[6] = sig.resonance();
            }
            // Higher dimensions get combinations
            for i in 7..dimension {
                components[i] = (components[i % 5] + components[(i + 1) % 5]) / 2.0;
            }
        }

        Self { components, dimension }
    }
}

impl Default for FieldVector {
    fn default() -> Self {
        Self::new(16)
    }
}

/// Resonance attractor in the field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceAttractor {
    /// Center point of the attractor
    pub center: FieldVector,
    /// Basin of attraction radius
    pub basin_radius: f64,
    /// Attractor strength
    pub strength: f64,
    /// Stability score
    pub stability: f64,
    /// Whether this is a Mandorla zone
    pub is_mandorla: bool,
}

impl ResonanceAttractor {
    /// Create a new attractor
    pub fn new(center: FieldVector, basin_radius: f64, strength: f64) -> Self {
        Self {
            center,
            basin_radius,
            strength,
            stability: 0.5,
            is_mandorla: false,
        }
    }

    /// Check if a point is in the basin of attraction
    pub fn contains(&self, point: &FieldVector) -> bool {
        self.center.distance(point) < self.basin_radius
    }

    /// Pull a point towards the attractor
    pub fn attract(&self, point: &mut FieldVector) {
        let dist = self.center.distance(point);
        if dist < self.basin_radius && dist > 1e-10 {
            let pull = self.strength * (1.0 - dist / self.basin_radius);
            for (p, c) in point.components.iter_mut().zip(self.center.components.iter()) {
                *p += pull * (c - *p);
            }
        }
    }
}

/// Mandorla field - the resonance landscape
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandorlaField {
    /// Field dimension
    pub dimension: usize,
    /// Current field state
    pub state: FieldVector,
    /// History of field states
    history: Vec<FieldVector>,
    /// Detected attractors
    attractors: Vec<ResonanceAttractor>,
    /// Momentum for updates
    momentum: FieldVector,
    /// Decay factor for momentum
    pub decay: f64,
    /// Learning rate for updates
    pub learning_rate: f64,
}

impl MandorlaField {
    /// Create a new Mandorla field
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            state: FieldVector::new(dimension),
            history: Vec::new(),
            attractors: Vec::new(),
            momentum: FieldVector::new(dimension),
            decay: 0.9,
            learning_rate: 0.1,
        }
    }

    /// Update field with new injection vector
    pub fn update(&mut self, injection: FieldVector) {
        // Save current state to history
        self.history.push(self.state.clone());

        // Update momentum
        self.momentum.scale(self.decay);
        let mut delta = injection;
        delta.scale(self.learning_rate);
        self.momentum.add(&delta);

        // Apply momentum to state
        self.state.add(&self.momentum);

        // Apply attractor forces
        for attractor in &self.attractors {
            if attractor.contains(&self.state) {
                let mut attracted = self.state.clone();
                attractor.attract(&mut attracted);
                self.state = attracted;
            }
        }
    }

    /// Update from submodule performance metrics
    pub fn update_submodules(&mut self, performance: &Signature3D, algorithm: &str) {
        let mut injection = FieldVector::new(self.dimension);

        // Encode performance into injection
        injection.components[0] = performance.psi;
        injection.components[1] = performance.rho;
        injection.components[2] = performance.omega;

        // Algorithm-specific adjustments
        match algorithm {
            "VQE" | "QAOA" => {
                injection.components[3] = performance.psi * 0.9;
            }
            "QuantumWalk" => {
                injection.components[4] = performance.omega * 0.8;
            }
            _ => {}
        }

        self.update(injection);
    }

    /// Add an attractor
    pub fn add_attractor(&mut self, attractor: ResonanceAttractor) {
        self.attractors.push(attractor);
    }

    /// Get attractors
    pub fn attractors(&self) -> &[ResonanceAttractor] {
        &self.attractors
    }

    /// Find the nearest attractor
    pub fn nearest_attractor(&self) -> Option<&ResonanceAttractor> {
        self.attractors
            .iter()
            .min_by(|a, b| {
                let da = self.state.distance(&a.center);
                let db = self.state.distance(&b.center);
                da.partial_cmp(&db).unwrap()
            })
    }

    /// Check if current state is in a Mandorla zone
    pub fn is_in_mandorla(&self) -> bool {
        self.attractors
            .iter()
            .any(|a| a.is_mandorla && a.contains(&self.state))
    }

    /// Compute field gradient at current state
    pub fn gradient(&self) -> FieldVector {
        let mut grad = FieldVector::new(self.dimension);

        // Sum gradients from all attractors
        for attractor in &self.attractors {
            let dist = self.state.distance(&attractor.center);
            if dist > 1e-10 {
                let factor = attractor.strength / (dist * dist);
                for i in 0..self.dimension {
                    grad.components[i] +=
                        factor * (attractor.center.components[i] - self.state.components[i]);
                }
            }
        }

        grad
    }

    /// Get trend direction
    pub fn trend(&self) -> Option<FieldVector> {
        if self.history.len() < 3 {
            return None;
        }

        let n = self.history.len();
        let mut trend = FieldVector::new(self.dimension);

        for i in 0..self.dimension {
            let recent = self.history[n - 1].components[i];
            let older = self.history[n - 3].components[i];
            trend.components[i] = recent - older;
        }

        Some(trend)
    }

    /// Convert to dictionary for serialization
    pub fn to_dict(&self) -> std::collections::HashMap<String, serde_json::Value> {
        let mut map = std::collections::HashMap::new();
        map.insert(
            "dimension".to_string(),
            serde_json::json!(self.dimension),
        );
        map.insert(
            "state".to_string(),
            serde_json::json!(self.state.components),
        );
        map.insert(
            "attractors".to_string(),
            serde_json::json!(self.attractors.len()),
        );
        map
    }

    /// Create from dictionary
    pub fn from_dict(
        map: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        let dimension = map
            .get("dimension")
            .and_then(|v| v.as_u64())
            .unwrap_or(16) as usize;

        let components = map
            .get("state")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_f64())
                    .collect::<Vec<f64>>()
            })
            .unwrap_or_else(|| vec![0.0; dimension]);

        let mut field = Self::new(dimension);
        field.state = FieldVector::from_components(components);
        field
    }
}

impl Default for MandorlaField {
    fn default() -> Self {
        Self::new(16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_vector() {
        let v = FieldVector::new(5);
        assert_eq!(v.dimension, 5);
        assert_eq!(v.magnitude(), 0.0);
    }

    #[test]
    fn test_field_vector_operations() {
        let mut v1 = FieldVector::from_components(vec![1.0, 0.0, 0.0]);
        let v2 = FieldVector::from_components(vec![0.0, 1.0, 0.0]);

        assert!((v1.dot(&v2) - 0.0).abs() < 1e-10);
        assert!((v1.distance(&v2) - 2.0_f64.sqrt()).abs() < 1e-10);

        v1.add(&v2);
        assert!((v1.components[0] - 1.0).abs() < 1e-10);
        assert!((v1.components[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_resonance_attractor() {
        let center = FieldVector::from_components(vec![0.5, 0.5, 0.5]);
        let attractor = ResonanceAttractor::new(center, 0.3, 0.5);

        let near = FieldVector::from_components(vec![0.6, 0.5, 0.5]);
        let far = FieldVector::from_components(vec![1.0, 1.0, 1.0]);

        assert!(attractor.contains(&near));
        assert!(!attractor.contains(&far));
    }

    #[test]
    fn test_mandorla_field() {
        let mut field = MandorlaField::new(10);

        // Add attractor
        let center = FieldVector::from_components(vec![0.9; 10]);
        let attractor = ResonanceAttractor::new(center, 0.5, 0.5);
        field.add_attractor(attractor);

        // Update field
        let injection = FieldVector::from_components(vec![0.5; 10]);
        field.update(injection);

        assert!(!field.history.is_empty());
    }

    #[test]
    fn test_signature_encoding() {
        let sig = Signature::D3(Signature3D::new(0.8, 0.7, 0.6));
        let encoded = FieldVector::encode_signature(&sig, 10);

        assert_eq!(encoded.dimension, 10);
        assert!((encoded.components[0] - 0.8).abs() < 1e-10);
        assert!((encoded.components[1] - 0.7).abs() < 1e-10);
        assert!((encoded.components[2] - 0.6).abs() < 1e-10);
    }
}
