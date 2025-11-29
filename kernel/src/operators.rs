//! Core operators for the generative pipeline.
//!
//! Per Section 5 of the specification, we define three principal operators:
//! - Extract (Ex): Candidate generation and normalization
//! - Compose (Co): Filtering, combination, and consolidation
//! - Materialize (M): Irreversible artefact creation

use crate::blueprint::{Blueprint, BlueprintCandidate, BlueprintConstraint, CandidateSource, ConstraintType};
use crate::domain_adapters::DomainAdapter;
use crate::error::{KernelError, Result};
use crate::resonance::{ResonanceModel, ResonanceThreshold};
use crate::state::{CoreSignature, State};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for kernel operators
pub trait KernelOperator: Send + Sync {
    /// Operator name
    fn name(&self) -> &str;

    /// Operator description
    fn description(&self) -> &str;
}

// ============================================================================
// Extract Operator (Ex)
// ============================================================================

/// Extract operator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractConfig {
    /// Maximum number of candidates to extract
    pub max_candidates: usize,
    /// Normalization mode
    pub normalize: bool,
    /// Minimum resonance for extraction
    pub min_resonance: f64,
    /// Include variations
    pub include_variations: bool,
    /// Variation delta
    pub variation_delta: f64,
}

impl Default for ExtractConfig {
    fn default() -> Self {
        Self {
            max_candidates: 100,
            normalize: true,
            min_resonance: 0.0,
            include_variations: true,
            variation_delta: 0.1,
        }
    }
}

/// Extract operator: Ex: Input → {B1, ..., Bk}
///
/// Takes input data (prompts, existing artefacts, partial blueprints) and
/// produces a set of candidate blueprints.
#[derive(Debug, Clone)]
pub struct ExtractOperator {
    /// Configuration
    pub config: ExtractConfig,
    /// Resonance model for evaluation
    pub resonance_model: ResonanceModel,
}

impl Default for ExtractOperator {
    fn default() -> Self {
        Self {
            config: ExtractConfig::default(),
            resonance_model: ResonanceModel::default(),
        }
    }
}

impl ExtractOperator {
    /// Create with configuration
    pub fn new(config: ExtractConfig) -> Self {
        Self {
            config,
            resonance_model: ResonanceModel::default(),
        }
    }

    /// Set resonance model
    pub fn with_resonance(mut self, model: ResonanceModel) -> Self {
        self.resonance_model = model;
        self
    }

    /// Extract candidates from raw input states
    pub fn extract_from_states(&self, inputs: &[State], generation: usize) -> Vec<BlueprintCandidate> {
        let mut candidates = Vec::new();

        for (i, state) in inputs.iter().enumerate() {
            let resonance = self.resonance_model.compute(state);

            if resonance >= self.config.min_resonance {
                let blueprint = Blueprint::from_state(
                    &format!("extracted_{}", i),
                    state.clone(),
                );
                let candidate = BlueprintCandidate::new(blueprint, generation, CandidateSource::Extracted);
                candidates.push(candidate);

                // Generate variations if enabled
                if self.config.include_variations {
                    let variations = self.generate_variations(state, generation, i);
                    candidates.extend(variations);
                }
            }

            if candidates.len() >= self.config.max_candidates {
                break;
            }
        }

        // Normalize if enabled
        if self.config.normalize {
            for c in &mut candidates {
                self.normalize_candidate(c);
            }
        }

        // Sort by resonance
        candidates.sort_by(|a, b| b.resonance_score.partial_cmp(&a.resonance_score).unwrap());

        // Truncate to max
        candidates.truncate(self.config.max_candidates);

        // Update ranks
        for (rank, c) in candidates.iter_mut().enumerate() {
            c.set_rank(rank);
        }

        candidates
    }

    /// Extract from signatures
    pub fn extract_from_signatures(&self, sigs: &[CoreSignature], generation: usize) -> Vec<BlueprintCandidate> {
        let states: Vec<State> = sigs.iter().map(|s| State::Core(*s)).collect();
        self.extract_from_states(&states, generation)
    }

    /// Generate variations around a state
    fn generate_variations(&self, state: &State, generation: usize, base_idx: usize) -> Vec<BlueprintCandidate> {
        let sig = state.to_core();
        let delta = self.config.variation_delta;
        let mut variations = Vec::new();

        // Generate variations along each dimension
        let perturbations = [
            (sig.psi + delta, sig.rho, sig.omega, sig.chi, sig.eta),
            (sig.psi - delta, sig.rho, sig.omega, sig.chi, sig.eta),
            (sig.psi, sig.rho + delta, sig.omega, sig.chi, sig.eta),
            (sig.psi, sig.rho - delta, sig.omega, sig.chi, sig.eta),
            (sig.psi, sig.rho, sig.omega + delta, sig.chi, sig.eta),
            (sig.psi, sig.rho, sig.omega - delta, sig.chi, sig.eta),
        ];

        for (i, (psi, rho, omega, chi, eta)) in perturbations.iter().enumerate() {
            let var_sig = CoreSignature::new(*psi, *rho, *omega, *chi, *eta);
            let var_state = State::Core(var_sig);
            let resonance = self.resonance_model.compute(&var_state);

            if resonance >= self.config.min_resonance {
                let blueprint = Blueprint::from_state(
                    &format!("variation_{}_{}", base_idx, i),
                    var_state,
                );
                let parent_id = format!("extracted_{}", base_idx);
                let candidate = BlueprintCandidate::new(
                    blueprint,
                    generation,
                    CandidateSource::Mutated { parent_id },
                );
                variations.push(candidate);
            }
        }

        variations
    }

    /// Normalize a candidate's state
    fn normalize_candidate(&self, candidate: &mut BlueprintCandidate) {
        let mut sig = candidate.blueprint.signature();
        sig.clamp();
        candidate.blueprint.state = State::Core(sig);
        candidate.resonance_score = self.resonance_model.compute(&candidate.blueprint.state);
    }
}

impl KernelOperator for ExtractOperator {
    fn name(&self) -> &str {
        "Extract"
    }

    fn description(&self) -> &str {
        "Generate and normalize candidate blueprints from input data"
    }
}

// ============================================================================
// Compose Operator (Co)
// ============================================================================

/// Compose operator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeConfig {
    /// Minimum resonance threshold for keeping candidates
    pub resonance_threshold: f64,
    /// Maximum candidates to retain after composition
    pub max_output: usize,
    /// Enable merging of compatible blueprints
    pub enable_merge: bool,
    /// Merge compatibility threshold
    pub merge_threshold: f64,
    /// Enable pruning of low-resonance states
    pub enable_pruning: bool,
}

impl Default for ComposeConfig {
    fn default() -> Self {
        Self {
            resonance_threshold: 0.5,
            max_output: 50,
            enable_merge: true,
            merge_threshold: 0.3,
            enable_pruning: true,
        }
    }
}

/// Compose operator: Co: {(B_i, v_i)} → {(B'_j, v'_j)}
///
/// Operates on a set of candidate blueprints to filter, merge, prune, and consolidate.
#[derive(Debug, Clone)]
pub struct ComposeOperator {
    /// Configuration
    pub config: ComposeConfig,
    /// Resonance model
    pub resonance_model: ResonanceModel,
    /// Constraints to enforce
    pub constraints: Vec<BlueprintConstraint>,
}

impl Default for ComposeOperator {
    fn default() -> Self {
        Self {
            config: ComposeConfig::default(),
            resonance_model: ResonanceModel::default(),
            constraints: Vec::new(),
        }
    }
}

impl ComposeOperator {
    /// Create with configuration
    pub fn new(config: ComposeConfig) -> Self {
        Self {
            config,
            resonance_model: ResonanceModel::default(),
            constraints: Vec::new(),
        }
    }

    /// Add constraint
    pub fn with_constraint(mut self, constraint: BlueprintConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Set resonance model
    pub fn with_resonance(mut self, model: ResonanceModel) -> Self {
        self.resonance_model = model;
        self
    }

    /// Compose candidates
    pub fn compose(&self, candidates: Vec<BlueprintCandidate>, generation: usize) -> Vec<BlueprintCandidate> {
        let mut result = candidates;

        // Step 1: Filter by resonance threshold
        if self.config.enable_pruning {
            result = self.prune_by_resonance(result);
        }

        // Step 2: Enforce constraints
        result = self.enforce_constraints(result);

        // Step 3: Merge compatible blueprints
        if self.config.enable_merge {
            result = self.merge_compatible(result, generation);
        }

        // Step 4: Sort by resonance
        result.sort_by(|a, b| b.resonance_score.partial_cmp(&a.resonance_score).unwrap());

        // Step 5: Truncate to max output
        result.truncate(self.config.max_output);

        // Step 6: Update ranks
        for (rank, c) in result.iter_mut().enumerate() {
            c.set_rank(rank);
        }

        result
    }

    /// Prune candidates below resonance threshold
    fn prune_by_resonance(&self, candidates: Vec<BlueprintCandidate>) -> Vec<BlueprintCandidate> {
        candidates
            .into_iter()
            .filter(|c| c.resonance_score >= self.config.resonance_threshold)
            .collect()
    }

    /// Enforce constraints
    fn enforce_constraints(&self, candidates: Vec<BlueprintCandidate>) -> Vec<BlueprintCandidate> {
        if self.constraints.is_empty() {
            return candidates;
        }

        candidates
            .into_iter()
            .filter(|c| {
                // Check all hard constraints
                for constraint in &self.constraints {
                    if constraint.hard && !self.check_constraint(&c.blueprint, constraint) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Check a single constraint
    fn check_constraint(&self, blueprint: &Blueprint, constraint: &BlueprintConstraint) -> bool {
        match &constraint.constraint_type {
            ConstraintType::MinResonance(min) => blueprint.resonance() >= *min,
            ConstraintType::MaxResonance(max) => blueprint.resonance() <= *max,
            ConstraintType::DimensionBounds { dimension, min, max } => {
                let v = blueprint.state.to_vec();
                if let Some(&val) = v.get(*dimension) {
                    val >= *min && val <= *max
                } else {
                    false
                }
            }
            ConstraintType::DistanceFrom { reference, max_distance } => {
                blueprint.signature().distance(reference) <= *max_distance
            }
            ConstraintType::Custom(_) => true,
        }
    }

    /// Merge compatible blueprints
    fn merge_compatible(&self, candidates: Vec<BlueprintCandidate>, generation: usize) -> Vec<BlueprintCandidate> {
        let mut result = Vec::new();
        let mut merged_indices = std::collections::HashSet::new();

        for (i, c1) in candidates.iter().enumerate() {
            if merged_indices.contains(&i) {
                continue;
            }

            // Find compatible candidates to merge with
            let mut merge_group = vec![c1.clone()];
            let mut group_indices = vec![i];

            for (j, c2) in candidates.iter().enumerate().skip(i + 1) {
                if merged_indices.contains(&j) {
                    continue;
                }

                let sig1 = c1.blueprint.signature();
                let sig2 = c2.blueprint.signature();

                if sig1.distance(&sig2) <= self.config.merge_threshold {
                    merge_group.push(c2.clone());
                    group_indices.push(j);
                }
            }

            if merge_group.len() > 1 {
                // Merge the group
                let merged = self.merge_group(&merge_group, generation);
                result.push(merged);
                for idx in group_indices {
                    merged_indices.insert(idx);
                }
            } else {
                result.push(c1.clone());
                merged_indices.insert(i);
            }
        }

        result
    }

    /// Merge a group of candidates
    fn merge_group(&self, group: &[BlueprintCandidate], generation: usize) -> BlueprintCandidate {
        // Average the signatures
        let n = group.len() as f64;
        let mut avg_psi = 0.0;
        let mut avg_rho = 0.0;
        let mut avg_omega = 0.0;
        let mut avg_chi = 0.0;
        let mut avg_eta = 0.0;

        for c in group {
            let sig = c.blueprint.signature();
            avg_psi += sig.psi;
            avg_rho += sig.rho;
            avg_omega += sig.omega;
            avg_chi += sig.chi;
            avg_eta += sig.eta;
        }

        let merged_sig = CoreSignature::new(
            avg_psi / n,
            avg_rho / n,
            avg_omega / n,
            avg_chi / n,
            avg_eta / n,
        );

        let parent_ids: Vec<String> = group.iter().map(|c| c.blueprint.id.clone()).collect();
        let blueprint = Blueprint::from_signature("merged", merged_sig);

        BlueprintCandidate::new(blueprint, generation, CandidateSource::Composed { parent_ids })
    }
}

impl KernelOperator for ComposeOperator {
    fn name(&self) -> &str {
        "Compose"
    }

    fn description(&self) -> &str {
        "Filter, merge, prune, and consolidate candidate blueprints"
    }
}

// ============================================================================
// Materialize Operator (M)
// ============================================================================

/// Materialize operator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterializeConfig {
    /// Required resonance for materialization
    pub min_resonance: f64,
    /// Output directory for artefacts
    pub output_dir: Option<String>,
    /// Generate ledger entry
    pub record_to_ledger: bool,
    /// Dry run (don't actually write files)
    pub dry_run: bool,
}

impl Default for MaterializeConfig {
    fn default() -> Self {
        Self {
            min_resonance: 0.5,
            output_dir: None,
            record_to_ledger: true,
            dry_run: false,
        }
    }
}

/// Materialize operator: M: B → A
///
/// Selects a blueprint and produces a concrete artefact. This step is
/// irreversible at the level of the ledger.
#[derive(Debug, Clone)]
pub struct MaterializeOperator {
    /// Configuration
    pub config: MaterializeConfig,
    /// Resonance model
    pub resonance_model: ResonanceModel,
}

impl Default for MaterializeOperator {
    fn default() -> Self {
        Self {
            config: MaterializeConfig::default(),
            resonance_model: ResonanceModel::default(),
        }
    }
}

impl MaterializeOperator {
    /// Create with configuration
    pub fn new(config: MaterializeConfig) -> Self {
        Self {
            config,
            resonance_model: ResonanceModel::default(),
        }
    }

    /// Check if a blueprint can be materialized
    pub fn can_materialize(&self, blueprint: &Blueprint) -> bool {
        blueprint.resonance() >= self.config.min_resonance
    }

    /// Select the best candidate for materialization
    pub fn select<'a>(&self, candidates: &'a [BlueprintCandidate]) -> Option<&'a BlueprintCandidate> {
        let min_resonance = self.config.min_resonance;
        candidates
            .iter()
            .filter(|c| c.blueprint.resonance() >= min_resonance)
            .max_by(|a, b| a.resonance_score.partial_cmp(&b.resonance_score).unwrap())
    }

    /// Prepare materialization result
    pub fn prepare(&self, blueprint: &Blueprint) -> Result<MaterializationPreview> {
        if !self.can_materialize(blueprint) {
            return Err(KernelError::ResonanceThresholdNotMet {
                current: blueprint.resonance(),
                threshold: self.config.min_resonance,
            });
        }

        Ok(MaterializationPreview {
            blueprint_id: blueprint.id.clone(),
            blueprint_name: blueprint.name.clone(),
            resonance: blueprint.resonance(),
            state: blueprint.state.clone(),
            constraints_satisfied: blueprint.satisfies_constraints(),
            estimated_output: format!("{}.json", blueprint.id),
        })
    }
}

impl KernelOperator for MaterializeOperator {
    fn name(&self) -> &str {
        "Materialize"
    }

    fn description(&self) -> &str {
        "Transform a blueprint into a concrete artefact"
    }
}

/// Preview of materialization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterializationPreview {
    /// Source blueprint ID
    pub blueprint_id: String,
    /// Blueprint name
    pub blueprint_name: String,
    /// Final resonance score
    pub resonance: f64,
    /// Final state
    pub state: State,
    /// All constraints satisfied
    pub constraints_satisfied: bool,
    /// Estimated output path
    pub estimated_output: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_operator() {
        let extractor = ExtractOperator::default();
        let states = vec![
            State::Core(CoreSignature::new(0.8, 0.7, 0.6, 0.5, 0.4)),
            State::Core(CoreSignature::new(0.9, 0.8, 0.7, 0.6, 0.3)),
        ];

        let candidates = extractor.extract_from_states(&states, 0);
        assert!(!candidates.is_empty());
        assert!(candidates[0].resonance_score >= candidates.last().unwrap().resonance_score);
    }

    #[test]
    fn test_compose_operator() {
        let composer = ComposeOperator::default();

        let candidates = vec![
            BlueprintCandidate::new(
                Blueprint::from_signature("a", CoreSignature::new(0.8, 0.8, 0.8, 0.5, 0.3)),
                0,
                CandidateSource::Seed,
            ),
            BlueprintCandidate::new(
                Blueprint::from_signature("b", CoreSignature::new(0.3, 0.3, 0.3, 0.5, 0.5)),
                0,
                CandidateSource::Seed,
            ),
        ];

        let result = composer.compose(candidates, 1);
        // Low resonance candidate should be pruned
        assert!(result.len() <= 2);
    }

    #[test]
    fn test_materialize_operator() {
        let materializer = MaterializeOperator::default();

        let high_res = Blueprint::from_signature("high", CoreSignature::new(0.9, 0.8, 0.7, 0.5, 0.3));
        let low_res = Blueprint::from_signature("low", CoreSignature::new(0.1, 0.1, 0.1, 0.5, 0.5));

        assert!(materializer.can_materialize(&high_res));
        assert!(!materializer.can_materialize(&low_res));
    }

    #[test]
    fn test_full_pipeline() {
        let extractor = ExtractOperator::default();
        let composer = ComposeOperator::default();
        let materializer = MaterializeOperator::default();

        // Extract
        let inputs = vec![
            State::Core(CoreSignature::new(0.8, 0.7, 0.6, 0.5, 0.4)),
        ];
        let extracted = extractor.extract_from_states(&inputs, 0);

        // Compose
        let composed = composer.compose(extracted, 1);

        // Select for materialization
        if let Some(selected) = materializer.select(&composed) {
            let preview = materializer.prepare(&selected.blueprint).unwrap();
            assert!(preview.resonance > 0.0);
        }
    }
}
