//! Mining artefacts with blueprint history.

use qops_core::Signature5D;
use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// An operator artefact mined from the topology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artefact {
    /// Unique identifier
    pub id: Uuid,
    /// Current position in graph
    pub node: NodeIndex,
    /// Signature at this artefact
    pub signature: Signature5D,
    /// Blueprint: sequence of transformations
    pub blueprint: Vec<Transformation>,
    /// Resonance score
    pub resonance: f64,
    /// Whether certified as Mandorla
    pub is_mandorla: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Stability metric
    pub stability: f64,
}

/// A transformation in the blueprint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    /// Type of transformation
    pub kind: TransformationType,
    /// Source node
    pub from: NodeIndex,
    /// Target node
    pub to: NodeIndex,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Types of transformations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransformationType {
    /// Transposition (swap two elements)
    Transposition(usize, usize),
    /// Rotation
    Rotation,
    /// Reflection
    Reflection,
    /// Inversion
    Inversion,
}

impl Artefact {
    /// Create a new artefact
    pub fn new(node: NodeIndex, signature: Signature5D) -> Self {
        let resonance = qops_core::resonance_5d(&signature);
        let is_mandorla = resonance >= 0.85 &&
            signature.psi * signature.rho * signature.omega >= 0.5;

        Self {
            id: Uuid::new_v4(),
            node,
            signature,
            blueprint: Vec::new(),
            resonance,
            is_mandorla,
            created_at: Utc::now(),
            stability: 0.5,
        }
    }

    /// Create artefact from just a signature (uses dummy node)
    pub fn from_signature(signature: Signature5D) -> Self {
        Self::new(NodeIndex::new(0), signature)
    }

    /// Check if artefact is in Mandorla zone
    pub fn is_mandorla(&self) -> bool {
        self.resonance >= 0.85 &&
            self.signature.psi * self.signature.rho * self.signature.omega >= 0.5
    }

    /// Add a transformation to the blueprint
    pub fn add_transformation(&mut self, transform: Transformation) {
        self.blueprint.push(transform);
    }

    /// Update signature and recalculate resonance
    pub fn update_signature(&mut self, signature: Signature5D) {
        self.signature = signature;
        self.resonance = qops_core::resonance_5d(&signature);
    }

    /// Check if artefact qualifies for Mandorla certification
    pub fn check_mandorla(&mut self, threshold: f64) {
        self.is_mandorla = self.resonance >= threshold && self.stability >= 0.8;
    }

    /// Get blueprint length
    pub fn blueprint_len(&self) -> usize {
        self.blueprint.len()
    }

    /// Get path from blueprint
    pub fn get_path(&self) -> Vec<NodeIndex> {
        let mut path = Vec::new();
        for t in &self.blueprint {
            if path.is_empty() {
                path.push(t.from);
            }
            path.push(t.to);
        }
        path
    }
}

impl Transformation {
    /// Create a new transformation
    pub fn new(kind: TransformationType, from: NodeIndex, to: NodeIndex) -> Self {
        Self {
            kind,
            from,
            to,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artefact_creation() {
        let sig = Signature5D::new(0.8, 0.7, 0.6, 0.5, 0.3);
        let artefact = Artefact::new(NodeIndex::new(0), sig);

        assert!(artefact.resonance > 0.0);
        assert!(!artefact.is_mandorla);
    }

    #[test]
    fn test_mandorla_check() {
        let sig = Signature5D::new(0.95, 0.9, 0.9, 0.8, 0.2);
        let mut artefact = Artefact::new(NodeIndex::new(0), sig);
        artefact.stability = 0.9;

        artefact.check_mandorla(0.85);
        assert!(artefact.is_mandorla);
    }
}
