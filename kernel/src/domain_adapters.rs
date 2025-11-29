//! Domain adapters for mapping domain-specific objects into H^n.
//!
//! Per the specification (Section 3), for each domain d we define:
//! D_d: Object_d → v ∈ H^n
//!
//! This module provides:
//! - The DomainAdapter trait for implementing domain-specific embeddings
//! - Example adapters for blueprints, code artefacts, and configurations
//! - Normalization and feature extraction utilities

use crate::error::{KernelError, Result};
use crate::state::{CoreSignature, ExtendedState, State, StateSpace};
use qops_core::Signature5D;
use qops_hypercube::Coord5D;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for domain adapters that map objects to the state space.
///
/// Each domain d requires an adapter D_d that:
/// - Extracts features (structural, syntactic, semantic)
/// - Projects into the core signature (ψ, ρ, ω, χ, η)
/// - Optionally augments with topological or spectral features
/// - Normalizes to maintain stability across domains
pub trait DomainAdapter: Send + Sync {
    /// The domain-specific object type
    type Object;

    /// Domain identifier
    fn domain_id(&self) -> &str;

    /// Embed a domain object into the state space
    fn embed(&self, object: &Self::Object) -> Result<State>;

    /// Extract core signature from domain object
    fn extract_signature(&self, object: &Self::Object) -> Result<CoreSignature>;

    /// Reverse mapping (if possible) - reconstruct object from state
    fn reconstruct(&self, state: &State) -> Result<Self::Object>;

    /// Check if reconstruction is supported
    fn supports_reconstruction(&self) -> bool {
        false
    }

    /// Normalize a state for this domain
    fn normalize(&self, state: &State) -> State {
        // Default: clamp to valid range
        let core = state.to_core();
        State::Core(CoreSignature::new(
            core.psi.clamp(0.0, 1.0),
            core.rho.clamp(0.0, 1.0),
            core.omega.clamp(0.0, 1.0),
            core.chi.clamp(0.0, 1.0),
            core.eta.clamp(0.0, 1.0),
        ))
    }

    /// Get state space configuration for this domain
    fn state_space(&self) -> StateSpace {
        StateSpace::default_5d()
    }
}

/// Blueprint adapter for embedding blueprint structures into H^n.
///
/// Blueprints are abstract structured candidate objects as defined in spec Section 2.3.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintAdapter {
    /// Domain identifier
    pub domain: String,
    /// Feature weights for signature computation
    pub feature_weights: BlueprintWeights,
    /// Normalization mode
    pub normalization: NormalizationMode,
}

/// Weights for blueprint feature extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintWeights {
    /// Weight for structural features → ψ
    pub structural: f64,
    /// Weight for stability features → ρ
    pub stability: f64,
    /// Weight for efficiency features → ω
    pub efficiency: f64,
    /// Weight for connectivity features → χ
    pub connectivity: f64,
    /// Weight for causality features → η
    pub causality: f64,
}

impl Default for BlueprintWeights {
    fn default() -> Self {
        Self {
            structural: 1.0,
            stability: 1.0,
            efficiency: 1.0,
            connectivity: 1.0,
            causality: 1.0,
        }
    }
}

/// Normalization mode for domain adapters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizationMode {
    /// No normalization
    None,
    /// Clamp to [0, 1]
    Clamp,
    /// Min-max normalization
    MinMax,
    /// Z-score normalization
    ZScore,
    /// Softmax normalization
    Softmax,
}

/// Generic blueprint structure for adapter input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintObject {
    /// Blueprint identifier
    pub id: String,
    /// Blueprint name
    pub name: String,
    /// Blueprint type/category
    pub blueprint_type: String,
    /// Raw features as key-value pairs
    pub features: HashMap<String, f64>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl BlueprintObject {
    /// Create a new blueprint object
    pub fn new(id: &str, name: &str, blueprint_type: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            blueprint_type: blueprint_type.to_string(),
            features: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a feature
    pub fn with_feature(mut self, key: &str, value: f64) -> Self {
        self.features.insert(key.to_string(), value);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

impl BlueprintAdapter {
    /// Create a new blueprint adapter
    pub fn new(domain: &str) -> Self {
        Self {
            domain: domain.to_string(),
            feature_weights: BlueprintWeights::default(),
            normalization: NormalizationMode::Clamp,
        }
    }

    /// Create with custom weights
    pub fn with_weights(mut self, weights: BlueprintWeights) -> Self {
        self.feature_weights = weights;
        self
    }

    /// Set normalization mode
    pub fn with_normalization(mut self, mode: NormalizationMode) -> Self {
        self.normalization = mode;
        self
    }

    /// Extract psi (quality/structural) from features
    fn extract_psi(&self, features: &HashMap<String, f64>) -> f64 {
        let keys = ["quality", "structure", "completeness", "psi"];
        self.aggregate_features(features, &keys) * self.feature_weights.structural
    }

    /// Extract rho (stability) from features
    fn extract_rho(&self, features: &HashMap<String, f64>) -> f64 {
        let keys = ["stability", "robustness", "consistency", "rho"];
        self.aggregate_features(features, &keys) * self.feature_weights.stability
    }

    /// Extract omega (efficiency) from features
    fn extract_omega(&self, features: &HashMap<String, f64>) -> f64 {
        let keys = ["efficiency", "performance", "speed", "omega"];
        self.aggregate_features(features, &keys) * self.feature_weights.efficiency
    }

    /// Extract chi (connectivity) from features
    fn extract_chi(&self, features: &HashMap<String, f64>) -> f64 {
        let keys = ["connectivity", "coupling", "coherence", "chi"];
        self.aggregate_features(features, &keys) * self.feature_weights.connectivity
    }

    /// Extract eta (causality) from features
    fn extract_eta(&self, features: &HashMap<String, f64>) -> f64 {
        let keys = ["causality", "ordering", "fluctuation", "eta"];
        self.aggregate_features(features, &keys) * self.feature_weights.causality
    }

    /// Aggregate features by keys
    fn aggregate_features(&self, features: &HashMap<String, f64>, keys: &[&str]) -> f64 {
        let mut sum = 0.0;
        let mut count = 0;

        for key in keys {
            if let Some(&val) = features.get(*key) {
                sum += val;
                count += 1;
            }
        }

        if count > 0 {
            sum / count as f64
        } else {
            0.5 // Default to center
        }
    }

    /// Apply normalization
    fn apply_normalization(&self, value: f64) -> f64 {
        match self.normalization {
            NormalizationMode::None => value,
            NormalizationMode::Clamp => value.clamp(0.0, 1.0),
            NormalizationMode::MinMax => value.clamp(0.0, 1.0),
            NormalizationMode::ZScore => {
                // Simplified z-score to [0,1] mapping
                (1.0 / (1.0 + (-value).exp())).clamp(0.0, 1.0)
            }
            NormalizationMode::Softmax => {
                // Single value softmax is identity
                value.clamp(0.0, 1.0)
            }
        }
    }
}

impl DomainAdapter for BlueprintAdapter {
    type Object = BlueprintObject;

    fn domain_id(&self) -> &str {
        &self.domain
    }

    fn embed(&self, object: &Self::Object) -> Result<State> {
        let sig = self.extract_signature(object)?;
        Ok(State::Core(sig))
    }

    fn extract_signature(&self, object: &Self::Object) -> Result<CoreSignature> {
        let psi = self.apply_normalization(self.extract_psi(&object.features));
        let rho = self.apply_normalization(self.extract_rho(&object.features));
        let omega = self.apply_normalization(self.extract_omega(&object.features));
        let chi = self.apply_normalization(self.extract_chi(&object.features));
        let eta = self.apply_normalization(self.extract_eta(&object.features));

        Ok(CoreSignature::new(psi, rho, omega, chi, eta))
    }

    fn reconstruct(&self, state: &State) -> Result<Self::Object> {
        let core = state.to_core();
        let mut obj = BlueprintObject::new(
            &uuid::Uuid::new_v4().to_string(),
            "reconstructed",
            &self.domain,
        );

        obj.features.insert("psi".to_string(), core.psi);
        obj.features.insert("rho".to_string(), core.rho);
        obj.features.insert("omega".to_string(), core.omega);
        obj.features.insert("chi".to_string(), core.chi);
        obj.features.insert("eta".to_string(), core.eta);

        Ok(obj)
    }

    fn supports_reconstruction(&self) -> bool {
        true
    }
}

/// Artefact adapter for embedding materialized artefacts.
///
/// Artefacts are materialised instances of blueprints (code, configs, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtefactAdapter {
    /// Domain identifier
    pub domain: String,
    /// Artefact type (code, config, document, etc.)
    pub artefact_type: String,
}

/// Generic artefact structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtefactObject {
    /// Artefact identifier
    pub id: String,
    /// Artefact type
    pub artefact_type: String,
    /// Source blueprint ID
    pub blueprint_id: Option<String>,
    /// Content (serialized)
    pub content: String,
    /// File path (if applicable)
    pub path: Option<String>,
    /// Quality metrics
    pub metrics: HashMap<String, f64>,
    /// Creation timestamp
    pub created_at: String,
}

impl ArtefactAdapter {
    /// Create a new artefact adapter
    pub fn new(domain: &str, artefact_type: &str) -> Self {
        Self {
            domain: domain.to_string(),
            artefact_type: artefact_type.to_string(),
        }
    }
}

impl DomainAdapter for ArtefactAdapter {
    type Object = ArtefactObject;

    fn domain_id(&self) -> &str {
        &self.domain
    }

    fn embed(&self, object: &Self::Object) -> Result<State> {
        let sig = self.extract_signature(object)?;
        Ok(State::Core(sig))
    }

    fn extract_signature(&self, object: &Self::Object) -> Result<CoreSignature> {
        // Extract from metrics or use defaults
        let psi = object.metrics.get("quality").copied().unwrap_or(0.5);
        let rho = object.metrics.get("stability").copied().unwrap_or(0.5);
        let omega = object.metrics.get("efficiency").copied().unwrap_or(0.5);
        let chi = object.metrics.get("connectivity").copied().unwrap_or(0.5);
        let eta = object.metrics.get("causality").copied().unwrap_or(0.5);

        Ok(CoreSignature::new(psi, rho, omega, chi, eta))
    }

    fn reconstruct(&self, _state: &State) -> Result<Self::Object> {
        Err(KernelError::AdapterError(
            "Artefact reconstruction not supported".to_string(),
        ))
    }
}

/// Code adapter for source code artefacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAdapter {
    /// Language
    pub language: String,
    /// Analysis depth
    pub depth: AnalysisDepth,
}

/// Depth of code analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisDepth {
    /// Shallow analysis (line count, basic metrics)
    Shallow,
    /// Medium analysis (structure, complexity)
    Medium,
    /// Deep analysis (semantic, patterns)
    Deep,
}

/// Code object representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeObject {
    /// File path
    pub path: String,
    /// Language
    pub language: String,
    /// Source content
    pub content: String,
    /// Line count
    pub line_count: usize,
    /// Complexity metrics
    pub complexity: f64,
    /// Test coverage (if available)
    pub coverage: Option<f64>,
}

impl CodeAdapter {
    /// Create a new code adapter
    pub fn new(language: &str) -> Self {
        Self {
            language: language.to_string(),
            depth: AnalysisDepth::Medium,
        }
    }

    /// Set analysis depth
    pub fn with_depth(mut self, depth: AnalysisDepth) -> Self {
        self.depth = depth;
        self
    }
}

impl DomainAdapter for CodeAdapter {
    type Object = CodeObject;

    fn domain_id(&self) -> &str {
        &self.language
    }

    fn embed(&self, object: &Self::Object) -> Result<State> {
        let sig = self.extract_signature(object)?;
        Ok(State::Core(sig))
    }

    fn extract_signature(&self, object: &Self::Object) -> Result<CoreSignature> {
        // Quality based on coverage and complexity
        let psi = object.coverage.unwrap_or(0.5);

        // Stability based on line count (normalized)
        let rho = (1.0 - (object.line_count as f64 / 10000.0).min(1.0)).max(0.2);

        // Efficiency inversely related to complexity
        let omega = (1.0 - object.complexity / 100.0).clamp(0.0, 1.0);

        // Connectivity and causality defaults
        let chi = 0.5;
        let eta = 0.3;

        Ok(CoreSignature::new(psi, rho, omega, chi, eta))
    }

    fn reconstruct(&self, _state: &State) -> Result<Self::Object> {
        Err(KernelError::AdapterError(
            "Code reconstruction not supported".to_string(),
        ))
    }
}

/// Operator family adapter for embedding operator families from Genesis
#[derive(Debug, Clone)]
pub struct OperatorFamilyAdapter {
    /// Source domain
    pub source: String,
}

impl OperatorFamilyAdapter {
    /// Create a new operator family adapter
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }
}

impl DomainAdapter for OperatorFamilyAdapter {
    type Object = qops_hypercube::OperatorFamily;

    fn domain_id(&self) -> &str {
        &self.source
    }

    fn embed(&self, object: &Self::Object) -> Result<State> {
        // Use the seed coordinate as the state
        let coord = &object.seed;
        Ok(State::Core(CoreSignature::from_coord5d(coord)))
    }

    fn extract_signature(&self, object: &Self::Object) -> Result<CoreSignature> {
        let coord = &object.seed;
        Ok(CoreSignature::from_coord5d(coord))
    }

    fn reconstruct(&self, _state: &State) -> Result<Self::Object> {
        Err(KernelError::AdapterError(
            "Operator family reconstruction not supported directly".to_string(),
        ))
    }
}

/// Adapter registry for managing multiple domain adapters
#[derive(Default)]
pub struct AdapterRegistry {
    /// Registered blueprint adapters
    blueprint_adapters: HashMap<String, BlueprintAdapter>,
    /// Registered artefact adapters
    artefact_adapters: HashMap<String, ArtefactAdapter>,
    /// Registered code adapters
    code_adapters: HashMap<String, CodeAdapter>,
}

impl AdapterRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a blueprint adapter
    pub fn register_blueprint(&mut self, adapter: BlueprintAdapter) {
        self.blueprint_adapters
            .insert(adapter.domain.clone(), adapter);
    }

    /// Register an artefact adapter
    pub fn register_artefact(&mut self, adapter: ArtefactAdapter) {
        self.artefact_adapters
            .insert(adapter.domain.clone(), adapter);
    }

    /// Register a code adapter
    pub fn register_code(&mut self, adapter: CodeAdapter) {
        self.code_adapters
            .insert(adapter.language.clone(), adapter);
    }

    /// Get blueprint adapter by domain
    pub fn get_blueprint(&self, domain: &str) -> Option<&BlueprintAdapter> {
        self.blueprint_adapters.get(domain)
    }

    /// Get artefact adapter by domain
    pub fn get_artefact(&self, domain: &str) -> Option<&ArtefactAdapter> {
        self.artefact_adapters.get(domain)
    }

    /// Get code adapter by language
    pub fn get_code(&self, language: &str) -> Option<&CodeAdapter> {
        self.code_adapters.get(language)
    }

    /// List all registered domains
    pub fn domains(&self) -> Vec<String> {
        let mut domains: Vec<String> = self.blueprint_adapters.keys().cloned().collect();
        domains.extend(self.artefact_adapters.keys().cloned());
        domains.extend(self.code_adapters.keys().cloned());
        domains.sort();
        domains.dedup();
        domains
    }

    /// Create default registry with common adapters
    pub fn default_registry() -> Self {
        let mut registry = Self::new();

        // Blueprint adapters
        registry.register_blueprint(BlueprintAdapter::new("protocol"));
        registry.register_blueprint(BlueprintAdapter::new("architecture"));
        registry.register_blueprint(BlueprintAdapter::new("configuration"));
        registry.register_blueprint(BlueprintAdapter::new("operator_network"));

        // Artefact adapters
        registry.register_artefact(ArtefactAdapter::new("code", "source"));
        registry.register_artefact(ArtefactAdapter::new("config", "configuration"));
        registry.register_artefact(ArtefactAdapter::new("document", "markdown"));

        // Code adapters
        registry.register_code(CodeAdapter::new("rust"));
        registry.register_code(CodeAdapter::new("typescript"));
        registry.register_code(CodeAdapter::new("python"));

        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blueprint_adapter() {
        let adapter = BlueprintAdapter::new("test");

        let obj = BlueprintObject::new("1", "test_blueprint", "protocol")
            .with_feature("quality", 0.8)
            .with_feature("stability", 0.7)
            .with_feature("efficiency", 0.6);

        let sig = adapter.extract_signature(&obj).unwrap();
        assert!(sig.psi > 0.0);
        assert!(sig.rho > 0.0);
    }

    #[test]
    fn test_blueprint_embed() {
        let adapter = BlueprintAdapter::new("architecture");

        let obj = BlueprintObject::new("2", "arch", "architecture")
            .with_feature("psi", 0.9)
            .with_feature("rho", 0.8);

        let state = adapter.embed(&obj).unwrap();
        let core = state.to_core();
        assert!(core.psi > 0.5);
    }

    #[test]
    fn test_adapter_registry() {
        let registry = AdapterRegistry::default_registry();

        assert!(registry.get_blueprint("protocol").is_some());
        assert!(registry.get_code("rust").is_some());
        assert!(!registry.domains().is_empty());
    }

    #[test]
    fn test_code_adapter() {
        let adapter = CodeAdapter::new("rust");

        let code = CodeObject {
            path: "src/lib.rs".to_string(),
            language: "rust".to_string(),
            content: "fn main() {}".to_string(),
            line_count: 100,
            complexity: 10.0,
            coverage: Some(0.85),
        };

        let sig = adapter.extract_signature(&code).unwrap();
        assert_eq!(sig.psi, 0.85); // From coverage
    }
}
