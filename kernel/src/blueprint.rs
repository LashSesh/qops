//! Blueprint types for the kernel.
//!
//! Blueprints are structured candidate objects in abstract form as defined in Section 2.3.
//! They represent protocol specifications, software architectures, configurations, etc.

use crate::error::{KernelError, Result};
use crate::state::{CoreSignature, State};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A blueprint is a structured candidate object in abstract form.
///
/// Per the spec, blueprints exist in the hypercube/HDAG space and are
/// transformed through the kernel operators before materialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blueprint {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Blueprint type/category
    pub blueprint_type: BlueprintType,
    /// State representation in H^n
    pub state: State,
    /// Structural content
    pub content: BlueprintContent,
    /// Metadata
    pub metadata: BlueprintMetadata,
    /// Constraints that must be satisfied
    pub constraints: Vec<BlueprintConstraint>,
}

/// Blueprint type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlueprintType {
    /// Protocol specification
    Protocol,
    /// Software architecture
    Architecture,
    /// Generative model configuration
    Configuration,
    /// Operator network
    OperatorNetwork,
    /// Generic blueprint
    Generic(String),
}

impl BlueprintType {
    /// Get type name
    pub fn name(&self) -> &str {
        match self {
            BlueprintType::Protocol => "protocol",
            BlueprintType::Architecture => "architecture",
            BlueprintType::Configuration => "configuration",
            BlueprintType::OperatorNetwork => "operator_network",
            BlueprintType::Generic(s) => s,
        }
    }
}

/// Blueprint content structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlueprintContent {
    /// Structured key-value pairs
    Structured(HashMap<String, serde_json::Value>),
    /// Raw text content
    Text(String),
    /// Binary content (base64 encoded)
    Binary(String),
    /// Reference to external content
    Reference { uri: String, hash: Option<String> },
    /// Composite of multiple contents
    Composite(Vec<BlueprintContent>),
}

impl Default for BlueprintContent {
    fn default() -> Self {
        BlueprintContent::Structured(HashMap::new())
    }
}

/// Blueprint metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlueprintMetadata {
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Last modified timestamp
    pub modified_at: Option<String>,
    /// Author/source
    pub author: Option<String>,
    /// Version
    pub version: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Custom metadata
    pub custom: HashMap<String, String>,
    /// Parent blueprint ID (if derived)
    pub parent_id: Option<String>,
    /// Resonance score at creation
    pub initial_resonance: Option<f64>,
}

/// Blueprint constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintConstraint {
    /// Constraint name
    pub name: String,
    /// Constraint type
    pub constraint_type: ConstraintType,
    /// Constraint is hard (must be satisfied) or soft (preference)
    pub hard: bool,
    /// Weight for soft constraints
    pub weight: f64,
}

/// Types of constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    /// Minimum resonance threshold
    MinResonance(f64),
    /// Maximum resonance (for diversity)
    MaxResonance(f64),
    /// Dimension bounds
    DimensionBounds { dimension: usize, min: f64, max: f64 },
    /// Distance from reference state
    DistanceFrom { reference: CoreSignature, max_distance: f64 },
    /// Custom constraint (evaluated externally)
    Custom(String),
}

impl Blueprint {
    /// Create a new blueprint
    pub fn new(name: &str, blueprint_type: BlueprintType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            blueprint_type,
            state: State::default(),
            content: BlueprintContent::default(),
            metadata: BlueprintMetadata::default(),
            constraints: Vec::new(),
        }
    }

    /// Create from state
    pub fn from_state(name: &str, state: State) -> Self {
        let mut bp = Self::new(name, BlueprintType::Generic("from_state".to_string()));
        bp.state = state;
        bp
    }

    /// Create from core signature
    pub fn from_signature(name: &str, sig: CoreSignature) -> Self {
        Self::from_state(name, State::Core(sig))
    }

    /// Set content
    pub fn with_content(mut self, content: BlueprintContent) -> Self {
        self.content = content;
        self
    }

    /// Add constraint
    pub fn with_constraint(mut self, constraint: BlueprintConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Add tag
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.metadata.tags.push(tag.to_string());
        self
    }

    /// Get core signature
    pub fn signature(&self) -> CoreSignature {
        self.state.to_core()
    }

    /// Get resonance score
    pub fn resonance(&self) -> f64 {
        self.state.resonance()
    }

    /// Check if all hard constraints are satisfied
    pub fn satisfies_constraints(&self) -> bool {
        for c in &self.constraints {
            if c.hard && !self.check_constraint(c) {
                return false;
            }
        }
        true
    }

    /// Check a single constraint
    fn check_constraint(&self, constraint: &BlueprintConstraint) -> bool {
        match &constraint.constraint_type {
            ConstraintType::MinResonance(min) => self.resonance() >= *min,
            ConstraintType::MaxResonance(max) => self.resonance() <= *max,
            ConstraintType::DimensionBounds { dimension, min, max } => {
                let v = self.state.to_vec();
                if let Some(&val) = v.get(*dimension) {
                    val >= *min && val <= *max
                } else {
                    false
                }
            }
            ConstraintType::DistanceFrom { reference, max_distance } => {
                let sig = self.signature();
                sig.distance(reference) <= *max_distance
            }
            ConstraintType::Custom(_) => true, // External evaluation needed
        }
    }

    /// Compute total constraint satisfaction score (0.0 to 1.0)
    pub fn constraint_score(&self) -> f64 {
        if self.constraints.is_empty() {
            return 1.0;
        }

        let mut score = 0.0;
        let mut total_weight = 0.0;

        for c in &self.constraints {
            let satisfied = self.check_constraint(c);
            let weight = if c.hard { 1.0 } else { c.weight };
            total_weight += weight;
            if satisfied {
                score += weight;
            }
        }

        if total_weight > 0.0 {
            score / total_weight
        } else {
            1.0
        }
    }
}

/// A candidate blueprint during mining/composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintCandidate {
    /// The blueprint
    pub blueprint: Blueprint,
    /// Resonance score
    pub resonance_score: f64,
    /// Rank in candidate set
    pub rank: usize,
    /// Generation/iteration when discovered
    pub generation: usize,
    /// Source (how it was generated)
    pub source: CandidateSource,
    /// Additional scores
    pub scores: HashMap<String, f64>,
}

/// How a candidate was generated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CandidateSource {
    /// Initial seed
    Seed,
    /// Extracted from input
    Extracted,
    /// Composed from other candidates
    Composed { parent_ids: Vec<String> },
    /// Mutated from parent
    Mutated { parent_id: String },
    /// Random exploration
    Random,
    /// HDAG node expansion
    HdagExpansion { node_id: String },
}

impl BlueprintCandidate {
    /// Create a new candidate
    pub fn new(blueprint: Blueprint, generation: usize, source: CandidateSource) -> Self {
        let resonance_score = blueprint.resonance();
        Self {
            blueprint,
            resonance_score,
            rank: 0,
            generation,
            source,
            scores: HashMap::new(),
        }
    }

    /// Create from blueprint with scores
    pub fn with_scores(mut self, scores: HashMap<String, f64>) -> Self {
        self.scores = scores;
        self
    }

    /// Update rank
    pub fn set_rank(&mut self, rank: usize) {
        self.rank = rank;
    }

    /// Get blueprint ID
    pub fn id(&self) -> &str {
        &self.blueprint.id
    }

    /// Get combined score (resonance + constraint satisfaction)
    pub fn combined_score(&self) -> f64 {
        let constraint_score = self.blueprint.constraint_score();
        0.7 * self.resonance_score + 0.3 * constraint_score
    }
}

/// Builder for creating blueprints
pub struct BlueprintBuilder {
    name: String,
    blueprint_type: BlueprintType,
    state: Option<State>,
    content: BlueprintContent,
    metadata: BlueprintMetadata,
    constraints: Vec<BlueprintConstraint>,
}

impl BlueprintBuilder {
    /// Create a new builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            blueprint_type: BlueprintType::Generic("default".to_string()),
            state: None,
            content: BlueprintContent::default(),
            metadata: BlueprintMetadata::default(),
            constraints: Vec::new(),
        }
    }

    /// Set blueprint type
    pub fn blueprint_type(mut self, t: BlueprintType) -> Self {
        self.blueprint_type = t;
        self
    }

    /// Set state
    pub fn state(mut self, s: State) -> Self {
        self.state = Some(s);
        self
    }

    /// Set signature
    pub fn signature(mut self, sig: CoreSignature) -> Self {
        self.state = Some(State::Core(sig));
        self
    }

    /// Set content
    pub fn content(mut self, c: BlueprintContent) -> Self {
        self.content = c;
        self
    }

    /// Add constraint
    pub fn constraint(mut self, c: BlueprintConstraint) -> Self {
        self.constraints.push(c);
        self
    }

    /// Add min resonance constraint
    pub fn min_resonance(self, threshold: f64) -> Self {
        self.constraint(BlueprintConstraint {
            name: "min_resonance".to_string(),
            constraint_type: ConstraintType::MinResonance(threshold),
            hard: true,
            weight: 1.0,
        })
    }

    /// Set author
    pub fn author(mut self, author: &str) -> Self {
        self.metadata.author = Some(author.to_string());
        self
    }

    /// Add tag
    pub fn tag(mut self, tag: &str) -> Self {
        self.metadata.tags.push(tag.to_string());
        self
    }

    /// Build the blueprint
    pub fn build(self) -> Blueprint {
        Blueprint {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            blueprint_type: self.blueprint_type,
            state: self.state.unwrap_or_default(),
            content: self.content,
            metadata: self.metadata,
            constraints: self.constraints,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blueprint_creation() {
        let bp = Blueprint::new("test", BlueprintType::Protocol);
        assert_eq!(bp.name, "test");
        assert!(!bp.id.is_empty());
    }

    #[test]
    fn test_blueprint_from_signature() {
        let sig = CoreSignature::new(0.8, 0.7, 0.6, 0.5, 0.4);
        let bp = Blueprint::from_signature("test", sig);

        assert_eq!(bp.signature().psi, 0.8);
    }

    #[test]
    fn test_constraint_satisfaction() {
        let sig = CoreSignature::new(0.9, 0.8, 0.7, 0.5, 0.3);
        let bp = Blueprint::from_signature("constrained", sig)
            .with_constraint(BlueprintConstraint {
                name: "min_res".to_string(),
                constraint_type: ConstraintType::MinResonance(0.4),
                hard: true,
                weight: 1.0,
            });

        assert!(bp.satisfies_constraints());
    }

    #[test]
    fn test_blueprint_builder() {
        let bp = BlueprintBuilder::new("built")
            .blueprint_type(BlueprintType::Architecture)
            .signature(CoreSignature::center())
            .min_resonance(0.5)
            .tag("test")
            .build();

        assert_eq!(bp.name, "built");
        assert!(bp.metadata.tags.contains(&"test".to_string()));
    }

    #[test]
    fn test_candidate() {
        let bp = Blueprint::from_signature("cand", CoreSignature::center());
        let cand = BlueprintCandidate::new(bp, 0, CandidateSource::Seed);

        assert_eq!(cand.generation, 0);
        assert!(cand.resonance_score > 0.0);
    }
}
