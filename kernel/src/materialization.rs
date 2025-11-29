//! Materialization layer for artefact generation.
//!
//! Per Section 5.3 and Section 7.1 (L5), the Materialization Layer applies the
//! Materialize operator M to selected blueprints to create artefacts A.
//!
//! This step is irreversible at the level of the ledger: the transformation
//! B → A is recorded and becomes part of the system history.

use crate::blueprint::{Blueprint, BlueprintCandidate, BlueprintContent, BlueprintMetadata};
use crate::error::{KernelError, Result};
use crate::ledger::{KernelLedger, LedgerRecord, TransformationEntry};
use crate::operators::MaterializeOperator;
use crate::resonance::ResonanceModel;
use crate::state::{CoreSignature, State};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// Artefact output representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtefactOutput {
    /// Artefact identifier
    pub id: String,
    /// Source blueprint ID
    pub blueprint_id: String,
    /// Artefact type
    pub artefact_type: ArtefactType,
    /// Content
    pub content: ArtefactContent,
    /// Final state/signature
    pub final_state: State,
    /// Final resonance score
    pub final_resonance: f64,
    /// Output paths (if files were written)
    pub output_paths: Vec<PathBuf>,
    /// Metadata
    pub metadata: ArtefactMetadata,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Types of artefacts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtefactType {
    /// Code repository/files
    Code,
    /// Configuration files
    Configuration,
    /// Documentation
    Document,
    /// Operator implementation
    OperatorImplementation,
    /// Quantum circuit
    QuantumCircuit,
    /// Data/JSON output
    Data,
    /// Generic artefact
    Generic(String),
}

impl ArtefactType {
    /// Get type name
    pub fn name(&self) -> &str {
        match self {
            ArtefactType::Code => "code",
            ArtefactType::Configuration => "configuration",
            ArtefactType::Document => "document",
            ArtefactType::OperatorImplementation => "operator",
            ArtefactType::QuantumCircuit => "circuit",
            ArtefactType::Data => "data",
            ArtefactType::Generic(s) => s,
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &str {
        match self {
            ArtefactType::Code => "rs",
            ArtefactType::Configuration => "toml",
            ArtefactType::Document => "md",
            ArtefactType::OperatorImplementation => "rs",
            ArtefactType::QuantumCircuit => "qasm",
            ArtefactType::Data => "json",
            ArtefactType::Generic(_) => "txt",
        }
    }
}

/// Artefact content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtefactContent {
    /// Text content
    Text(String),
    /// JSON data
    Json(serde_json::Value),
    /// Binary data (base64 encoded)
    Binary(String),
    /// Multiple files
    Files(HashMap<String, String>),
    /// Reference to external content
    Reference { uri: String, hash: String },
}

/// Artefact metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArtefactMetadata {
    /// Materialization parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Constraints applied
    pub constraints_applied: Vec<String>,
    /// Source blueprint metadata
    pub source_metadata: Option<BlueprintMetadata>,
    /// Generation pipeline
    pub pipeline: Vec<String>,
    /// Hash of content
    pub content_hash: Option<String>,
}

/// Materialization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterializationResult {
    /// Generated artefact
    pub artefact: ArtefactOutput,
    /// Ledger entry (if recorded)
    pub ledger_entry: Option<LedgerRecord>,
    /// Success flag
    pub success: bool,
    /// Any warnings
    pub warnings: Vec<String>,
}

/// Materializer for transforming blueprints to artefacts
pub struct Materializer {
    /// Materialize operator
    pub operator: MaterializeOperator,
    /// Resonance model
    pub resonance_model: ResonanceModel,
    /// Output directory
    pub output_dir: PathBuf,
    /// Whether to actually write files
    pub write_files: bool,
    /// Ledger for recording transformations
    pub ledger: Option<Box<dyn KernelLedger>>,
}

impl Materializer {
    /// Create a new materializer
    pub fn new(output_dir: PathBuf) -> Self {
        Self {
            operator: MaterializeOperator::default(),
            resonance_model: ResonanceModel::default(),
            output_dir,
            write_files: true,
            ledger: None,
        }
    }

    /// Set to dry run mode
    pub fn dry_run(mut self) -> Self {
        self.write_files = false;
        self
    }

    /// Set ledger
    pub fn with_ledger(mut self, ledger: Box<dyn KernelLedger>) -> Self {
        self.ledger = Some(ledger);
        self
    }

    /// Set resonance model
    pub fn with_resonance(mut self, model: ResonanceModel) -> Self {
        self.resonance_model = model;
        self
    }

    /// Materialize a blueprint into an artefact
    pub fn materialize(
        &mut self,
        blueprint: &Blueprint,
        artefact_type: ArtefactType,
    ) -> Result<MaterializationResult> {
        // Check if blueprint can be materialized
        if !self.operator.can_materialize(blueprint) {
            return Err(KernelError::ResonanceThresholdNotMet {
                current: blueprint.resonance(),
                threshold: self.operator.config.min_resonance,
            });
        }

        let mut warnings = Vec::new();

        // Check constraints
        if !blueprint.satisfies_constraints() {
            warnings.push("Not all constraints are satisfied".to_string());
        }

        // Generate artefact ID
        let artefact_id = Uuid::new_v4().to_string();

        // Generate content based on blueprint
        let content = self.generate_content(blueprint, &artefact_type)?;

        // Calculate content hash
        let content_hash = self.hash_content(&content);

        // Determine output paths
        let output_paths = if self.write_files {
            self.write_artefact_files(&artefact_id, &artefact_type, &content)?
        } else {
            Vec::new()
        };

        // Create artefact
        let artefact = ArtefactOutput {
            id: artefact_id.clone(),
            blueprint_id: blueprint.id.clone(),
            artefact_type,
            content,
            final_state: blueprint.state.clone(),
            final_resonance: blueprint.resonance(),
            output_paths,
            metadata: ArtefactMetadata {
                parameters: HashMap::new(),
                constraints_applied: blueprint
                    .constraints
                    .iter()
                    .map(|c| c.name.clone())
                    .collect(),
                source_metadata: Some(blueprint.metadata.clone()),
                pipeline: vec!["extract".to_string(), "compose".to_string(), "materialize".to_string()],
                content_hash: Some(content_hash),
            },
            created_at: Utc::now(),
        };

        // Record to ledger
        let ledger_entry = if let Some(ledger) = &mut self.ledger {
            let entry = TransformationEntry {
                blueprint_id: blueprint.id.clone(),
                artefact_id: artefact_id.clone(),
                timestamp: artefact.created_at,
                parameters: HashMap::new(),
                resonance_score: blueprint.resonance(),
                constraints: blueprint.constraints.iter().map(|c| c.name.clone()).collect(),
            };
            let record = ledger.record_transformation(entry)?;
            Some(record)
        } else {
            None
        };

        Ok(MaterializationResult {
            artefact,
            ledger_entry,
            success: true,
            warnings,
        })
    }

    /// Materialize the best candidate from a set
    pub fn materialize_best(
        &mut self,
        candidates: &[BlueprintCandidate],
        artefact_type: ArtefactType,
    ) -> Result<MaterializationResult> {
        let best = self
            .operator
            .select(candidates)
            .ok_or_else(|| KernelError::MaterializationError("No suitable candidates".to_string()))?;

        // Clone the blueprint to break the borrow chain
        let blueprint = best.blueprint.clone();
        self.materialize(&blueprint, artefact_type)
    }

    /// Generate content from blueprint
    fn generate_content(
        &self,
        blueprint: &Blueprint,
        artefact_type: &ArtefactType,
    ) -> Result<ArtefactContent> {
        match artefact_type {
            ArtefactType::Data => {
                let data = serde_json::json!({
                    "id": blueprint.id,
                    "name": blueprint.name,
                    "type": blueprint.blueprint_type.name(),
                    "state": {
                        "psi": blueprint.signature().psi,
                        "rho": blueprint.signature().rho,
                        "omega": blueprint.signature().omega,
                        "chi": blueprint.signature().chi,
                        "eta": blueprint.signature().eta,
                    },
                    "resonance": blueprint.resonance(),
                    "constraints_satisfied": blueprint.satisfies_constraints(),
                    "metadata": blueprint.metadata,
                });
                Ok(ArtefactContent::Json(data))
            }
            ArtefactType::Configuration => {
                let sig = blueprint.signature();
                let config = format!(
                    r#"# Generated Configuration
# Blueprint: {}
# Resonance: {:.4}

[signature]
psi = {:.6}
rho = {:.6}
omega = {:.6}
chi = {:.6}
eta = {:.6}

[parameters]
resonance_threshold = 0.5
"#,
                    blueprint.name,
                    blueprint.resonance(),
                    sig.psi,
                    sig.rho,
                    sig.omega,
                    sig.chi,
                    sig.eta
                );
                Ok(ArtefactContent::Text(config))
            }
            ArtefactType::Document => {
                let sig = blueprint.signature();
                let doc = format!(
                    r#"# {}

## Overview

This document describes the materialized blueprint with the following characteristics:

- **Blueprint ID**: {}
- **Type**: {}
- **Resonance Score**: {:.4}

## State Signature

| Dimension | Value |
|-----------|-------|
| ψ (psi)   | {:.4} |
| ρ (rho)   | {:.4} |
| ω (omega) | {:.4} |
| χ (chi)   | {:.4} |
| η (eta)   | {:.4} |

## Constraints

{}

"#,
                    blueprint.name,
                    blueprint.id,
                    blueprint.blueprint_type.name(),
                    blueprint.resonance(),
                    sig.psi,
                    sig.rho,
                    sig.omega,
                    sig.chi,
                    sig.eta,
                    if blueprint.constraints.is_empty() {
                        "No constraints defined.".to_string()
                    } else {
                        blueprint
                            .constraints
                            .iter()
                            .map(|c| format!("- {}", c.name))
                            .collect::<Vec<_>>()
                            .join("\n")
                    }
                );
                Ok(ArtefactContent::Text(doc))
            }
            _ => {
                // Default to JSON data
                let data = serde_json::json!({
                    "blueprint": blueprint.id,
                    "state": blueprint.state.to_vec(),
                    "resonance": blueprint.resonance(),
                });
                Ok(ArtefactContent::Json(data))
            }
        }
    }

    /// Hash content for integrity verification
    fn hash_content(&self, content: &ArtefactContent) -> String {
        use sha2::{Digest, Sha256};

        let bytes = match content {
            ArtefactContent::Text(s) => s.as_bytes().to_vec(),
            ArtefactContent::Json(v) => v.to_string().into_bytes(),
            ArtefactContent::Binary(b) => b.as_bytes().to_vec(),
            ArtefactContent::Files(f) => {
                f.values().map(|s| s.as_bytes()).flatten().copied().collect()
            }
            ArtefactContent::Reference { uri, hash } => format!("{}:{}", uri, hash).into_bytes(),
        };

        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        format!("{:x}", hasher.finalize())
    }

    /// Write artefact files to disk
    fn write_artefact_files(
        &self,
        artefact_id: &str,
        artefact_type: &ArtefactType,
        content: &ArtefactContent,
    ) -> Result<Vec<PathBuf>> {
        let mut paths = Vec::new();

        // Ensure output directory exists
        std::fs::create_dir_all(&self.output_dir)?;

        match content {
            ArtefactContent::Text(text) => {
                let filename = format!("{}.{}", artefact_id, artefact_type.extension());
                let path = self.output_dir.join(&filename);
                std::fs::write(&path, text)?;
                paths.push(path);
            }
            ArtefactContent::Json(value) => {
                let filename = format!("{}.json", artefact_id);
                let path = self.output_dir.join(&filename);
                let json_str = serde_json::to_string_pretty(value)?;
                std::fs::write(&path, json_str)?;
                paths.push(path);
            }
            ArtefactContent::Binary(data) => {
                let filename = format!("{}.bin", artefact_id);
                let path = self.output_dir.join(&filename);
                std::fs::write(&path, data)?;
                paths.push(path);
            }
            ArtefactContent::Files(files) => {
                for (name, content) in files {
                    let path = self.output_dir.join(name);
                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(&path, content)?;
                    paths.push(path);
                }
            }
            ArtefactContent::Reference { .. } => {
                // No files to write for references
            }
        }

        Ok(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_artefact_type() {
        assert_eq!(ArtefactType::Code.extension(), "rs");
        assert_eq!(ArtefactType::Configuration.extension(), "toml");
        assert_eq!(ArtefactType::Data.extension(), "json");
    }

    #[test]
    fn test_materializer_dry_run() {
        let dir = tempdir().unwrap();
        let mut materializer = Materializer::new(dir.path().to_path_buf()).dry_run();

        let blueprint = Blueprint::from_signature(
            "test",
            CoreSignature::new(0.9, 0.8, 0.7, 0.5, 0.3),
        );

        let result = materializer
            .materialize(&blueprint, ArtefactType::Data)
            .unwrap();

        assert!(result.success);
        assert!(result.artefact.output_paths.is_empty()); // Dry run
    }

    #[test]
    fn test_materializer_write_files() {
        let dir = tempdir().unwrap();
        let mut materializer = Materializer::new(dir.path().to_path_buf());

        let blueprint = Blueprint::from_signature(
            "test_write",
            CoreSignature::new(0.9, 0.8, 0.7, 0.5, 0.3),
        );

        let result = materializer
            .materialize(&blueprint, ArtefactType::Data)
            .unwrap();

        assert!(result.success);
        assert!(!result.artefact.output_paths.is_empty());
        assert!(result.artefact.output_paths[0].exists());
    }

    #[test]
    fn test_content_generation() {
        let dir = tempdir().unwrap();
        let materializer = Materializer::new(dir.path().to_path_buf());

        let blueprint = Blueprint::from_signature(
            "content_test",
            CoreSignature::center(),
        );

        // Test different content types
        let data_content = materializer
            .generate_content(&blueprint, &ArtefactType::Data)
            .unwrap();
        assert!(matches!(data_content, ArtefactContent::Json(_)));

        let config_content = materializer
            .generate_content(&blueprint, &ArtefactType::Configuration)
            .unwrap();
        assert!(matches!(config_content, ArtefactContent::Text(_)));
    }
}
