//! Configuration types for the kernel module.
//!
//! Provides TOML-serializable configuration for all kernel components:
//! - State space parameters
//! - Resonance computation settings
//! - Mining kernel configuration
//! - Materialization options
//! - Ledger settings

use crate::error::{KernelError, Result};
use crate::mining::SearchStrategy;
use crate::resonance::ResonanceModel;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main kernel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelConfig {
    /// State space configuration
    #[serde(default)]
    pub state: StateParameters,

    /// Resonance configuration
    #[serde(default)]
    pub resonance: ResonanceParameters,

    /// Mining configuration
    #[serde(default)]
    pub mining: MiningParameters,

    /// Materialization configuration
    #[serde(default)]
    pub materialization: MaterializationParameters,

    /// Ledger configuration
    #[serde(default)]
    pub ledger: LedgerParameters,

    /// Domain adapter configurations
    #[serde(default)]
    pub adapters: HashMap<String, AdapterConfig>,

    /// Output directory for generated artefacts
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,

    /// Enable verbose logging
    #[serde(default)]
    pub verbose: bool,
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("./output")
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            state: StateParameters::default(),
            resonance: ResonanceParameters::default(),
            mining: MiningParameters::default(),
            materialization: MaterializationParameters::default(),
            ledger: LedgerParameters::default(),
            adapters: HashMap::new(),
            output_dir: default_output_dir(),
            verbose: false,
        }
    }
}

impl KernelConfig {
    /// Load configuration from a TOML file
    pub fn load(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: KernelConfig = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn save(&self, path: &std::path::Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| KernelError::ConfigError(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        self.state.validate()?;
        self.resonance.validate()?;
        self.mining.validate()?;
        self.materialization.validate()?;
        Ok(())
    }

    /// Create a minimal configuration for testing
    pub fn minimal() -> Self {
        Self {
            mining: MiningParameters {
                max_iterations: 100,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

/// State space parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateParameters {
    /// Number of dimensions (default 5 for core signature)
    #[serde(default = "default_dimensions")]
    pub dimensions: usize,

    /// Use extended state beyond core signature
    #[serde(default)]
    pub use_extended: bool,

    /// Additional dimensions when using extended state
    #[serde(default)]
    pub extra_dimensions: usize,

    /// Normalization mode for state vectors
    #[serde(default)]
    pub normalization: NormalizationMode,

    /// Coordinate bounds (min, max) for each dimension
    #[serde(default)]
    pub bounds: Option<Vec<(f64, f64)>>,
}

fn default_dimensions() -> usize {
    5
}

impl Default for StateParameters {
    fn default() -> Self {
        Self {
            dimensions: 5,
            use_extended: false,
            extra_dimensions: 0,
            normalization: NormalizationMode::default(),
            bounds: None,
        }
    }
}

impl StateParameters {
    fn validate(&self) -> Result<()> {
        if self.dimensions == 0 {
            return Err(KernelError::ConfigError(
                "Dimensions must be at least 1".to_string(),
            ));
        }
        if let Some(bounds) = &self.bounds {
            if bounds.len() != self.dimensions {
                return Err(KernelError::ConfigError(format!(
                    "Bounds length {} does not match dimensions {}",
                    bounds.len(),
                    self.dimensions
                )));
            }
            for (i, (min, max)) in bounds.iter().enumerate() {
                if min >= max {
                    return Err(KernelError::ConfigError(format!(
                        "Invalid bounds for dimension {}: min {} >= max {}",
                        i, min, max
                    )));
                }
            }
        }
        Ok(())
    }
}

/// Normalization mode for state vectors
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NormalizationMode {
    /// No normalization
    #[default]
    None,
    /// Clamp to [0, 1]
    Clamp,
    /// Min-max normalization
    MinMax,
    /// Z-score normalization
    ZScore,
}

/// Resonance computation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceParameters {
    /// Resonance model to use
    #[serde(default)]
    pub model: ResonanceModel,

    /// Weights for weighted resonance model
    #[serde(default)]
    pub weights: Option<ResonanceWeights>,

    /// Minimum threshold for acceptance
    #[serde(default = "default_min_threshold")]
    pub min_threshold: f64,

    /// Target threshold for optimization
    #[serde(default = "default_target_threshold")]
    pub target_threshold: f64,

    /// Alpha parameter for extended resonance
    #[serde(default = "default_alpha")]
    pub alpha: f64,

    /// Beta parameter for extended resonance
    #[serde(default = "default_beta")]
    pub beta: f64,
}

fn default_min_threshold() -> f64 {
    0.3
}

fn default_target_threshold() -> f64 {
    0.7
}

fn default_alpha() -> f64 {
    0.2
}

fn default_beta() -> f64 {
    0.1
}

impl Default for ResonanceParameters {
    fn default() -> Self {
        Self {
            model: ResonanceModel::default(),
            weights: None,
            min_threshold: 0.3,
            target_threshold: 0.7,
            alpha: 0.2,
            beta: 0.1,
        }
    }
}

impl ResonanceParameters {
    fn validate(&self) -> Result<()> {
        if self.min_threshold < 0.0 || self.min_threshold > 1.0 {
            return Err(KernelError::ConfigError(format!(
                "min_threshold {} out of range [0, 1]",
                self.min_threshold
            )));
        }
        if self.target_threshold < self.min_threshold {
            return Err(KernelError::ConfigError(
                "target_threshold must be >= min_threshold".to_string(),
            ));
        }
        if let Some(w) = &self.weights {
            w.validate()?;
        }
        Ok(())
    }
}

/// Weights for resonance calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceWeights {
    /// Weight for psi (intensity)
    #[serde(default = "default_weight")]
    pub psi: f64,
    /// Weight for rho (coherence)
    #[serde(default = "default_weight")]
    pub rho: f64,
    /// Weight for omega (frequency)
    #[serde(default = "default_weight")]
    pub omega: f64,
    /// Weight for chi (coupling)
    #[serde(default)]
    pub chi: f64,
    /// Weight for eta (dissipation)
    #[serde(default)]
    pub eta: f64,
}

fn default_weight() -> f64 {
    1.0
}

impl Default for ResonanceWeights {
    fn default() -> Self {
        Self {
            psi: 1.0,
            rho: 1.0,
            omega: 1.0,
            chi: 0.0,
            eta: 0.0,
        }
    }
}

impl ResonanceWeights {
    fn validate(&self) -> Result<()> {
        let sum = self.psi + self.rho + self.omega + self.chi.abs() + self.eta.abs();
        if sum <= 0.0 {
            return Err(KernelError::ConfigError(
                "Weights must sum to a positive value".to_string(),
            ));
        }
        Ok(())
    }

    /// Normalize weights to sum to 1.0 (for primary weights)
    pub fn normalize(&self) -> Self {
        let sum = self.psi + self.rho + self.omega;
        if sum <= 0.0 {
            return self.clone();
        }
        Self {
            psi: self.psi / sum,
            rho: self.rho / sum,
            omega: self.omega / sum,
            chi: self.chi,
            eta: self.eta,
        }
    }
}

/// Mining kernel parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningParameters {
    /// Maximum iterations for mining
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    /// Target resonance score
    #[serde(default = "default_target_resonance")]
    pub target_resonance: f64,

    /// Exploration rate (0.0-1.0) for stochastic strategies
    #[serde(default = "default_exploration_rate")]
    pub exploration_rate: f64,

    /// Search strategy
    #[serde(default)]
    pub strategy: SearchStrategy,

    /// Beam width for beam search
    #[serde(default = "default_beam_width")]
    pub beam_width: usize,

    /// Population size for evolutionary strategies
    #[serde(default = "default_population_size")]
    pub population_size: usize,

    /// Mutation rate for evolutionary strategies
    #[serde(default = "default_mutation_rate")]
    pub mutation_rate: f64,

    /// Number of elite candidates to preserve
    #[serde(default = "default_elite_count")]
    pub elite_count: usize,

    /// Maximum candidates to track
    #[serde(default = "default_max_candidates")]
    pub max_candidates: usize,

    /// Random seed for reproducibility (None = random)
    #[serde(default)]
    pub seed: Option<u64>,

    /// Enable parallel processing
    #[serde(default = "default_true")]
    pub parallel: bool,

    /// Number of threads (None = auto)
    #[serde(default)]
    pub threads: Option<usize>,
}

fn default_max_iterations() -> usize {
    1000
}

fn default_target_resonance() -> f64 {
    0.7
}

fn default_exploration_rate() -> f64 {
    0.3
}

fn default_beam_width() -> usize {
    10
}

fn default_population_size() -> usize {
    50
}

fn default_mutation_rate() -> f64 {
    0.1
}

fn default_elite_count() -> usize {
    5
}

fn default_max_candidates() -> usize {
    100
}

fn default_true() -> bool {
    true
}

impl Default for MiningParameters {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            target_resonance: 0.7,
            exploration_rate: 0.3,
            strategy: SearchStrategy::default(),
            beam_width: 10,
            population_size: 50,
            mutation_rate: 0.1,
            elite_count: 5,
            max_candidates: 100,
            seed: None,
            parallel: true,
            threads: None,
        }
    }
}

impl MiningParameters {
    fn validate(&self) -> Result<()> {
        if self.max_iterations == 0 {
            return Err(KernelError::ConfigError(
                "max_iterations must be at least 1".to_string(),
            ));
        }
        if self.exploration_rate < 0.0 || self.exploration_rate > 1.0 {
            return Err(KernelError::ConfigError(format!(
                "exploration_rate {} out of range [0, 1]",
                self.exploration_rate
            )));
        }
        if self.mutation_rate < 0.0 || self.mutation_rate > 1.0 {
            return Err(KernelError::ConfigError(format!(
                "mutation_rate {} out of range [0, 1]",
                self.mutation_rate
            )));
        }
        if self.beam_width == 0 {
            return Err(KernelError::ConfigError(
                "beam_width must be at least 1".to_string(),
            ));
        }
        Ok(())
    }
}

/// Materialization parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterializationParameters {
    /// Output format for artefacts
    #[serde(default)]
    pub output_format: OutputFormat,

    /// Generate intermediate files
    #[serde(default)]
    pub generate_intermediates: bool,

    /// Validate artefacts before writing
    #[serde(default = "default_true")]
    pub validate: bool,

    /// Overwrite existing files
    #[serde(default)]
    pub overwrite: bool,

    /// Template directory for code generation
    #[serde(default)]
    pub template_dir: Option<PathBuf>,

    /// Custom output handlers
    #[serde(default)]
    pub handlers: HashMap<String, String>,
}

impl Default for MaterializationParameters {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::default(),
            generate_intermediates: false,
            validate: true,
            overwrite: false,
            template_dir: None,
            handlers: HashMap::new(),
        }
    }
}

impl MaterializationParameters {
    fn validate(&self) -> Result<()> {
        // Template dir must exist if specified
        if let Some(ref dir) = self.template_dir {
            if !dir.exists() && !dir.as_os_str().is_empty() {
                // Allow non-existent paths during config creation
                // Actual validation happens at materialization time
            }
        }
        Ok(())
    }
}

/// Output format for materialized artefacts
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    /// JSON format
    #[default]
    Json,
    /// TOML format
    Toml,
    /// YAML format (if supported)
    Yaml,
    /// Raw text
    Text,
    /// Binary
    Binary,
}

/// Ledger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerParameters {
    /// Ledger storage type
    #[serde(default)]
    pub storage: LedgerStorage,

    /// Path for file-based ledger
    #[serde(default)]
    pub path: Option<PathBuf>,

    /// Enable integrity verification on load
    #[serde(default = "default_true")]
    pub verify_on_load: bool,

    /// Maximum records to keep in memory
    #[serde(default)]
    pub max_memory_records: Option<usize>,

    /// Auto-export interval (in records)
    #[serde(default)]
    pub export_interval: Option<usize>,
}

impl Default for LedgerParameters {
    fn default() -> Self {
        Self {
            storage: LedgerStorage::default(),
            path: None,
            verify_on_load: true,
            max_memory_records: None,
            export_interval: None,
        }
    }
}

/// Ledger storage type
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerStorage {
    /// In-memory only
    #[default]
    Memory,
    /// File-based (JSONL)
    File,
    /// Both memory and file
    Hybrid,
}

/// Domain adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    /// Adapter type identifier
    pub adapter_type: String,

    /// Whether this adapter is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Custom weights for this adapter
    #[serde(default)]
    pub weights: Option<ResonanceWeights>,

    /// Adapter-specific settings
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            adapter_type: "generic".to_string(),
            enabled: true,
            weights: None,
            settings: HashMap::new(),
        }
    }
}

/// Builder for kernel configuration
pub struct ConfigBuilder {
    config: KernelConfig,
}

impl ConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            config: KernelConfig::default(),
        }
    }

    /// Set output directory
    pub fn output_dir(mut self, path: PathBuf) -> Self {
        self.config.output_dir = path;
        self
    }

    /// Enable verbose mode
    pub fn verbose(mut self, enabled: bool) -> Self {
        self.config.verbose = enabled;
        self
    }

    /// Set mining parameters
    pub fn mining(mut self, params: MiningParameters) -> Self {
        self.config.mining = params;
        self
    }

    /// Set resonance parameters
    pub fn resonance(mut self, params: ResonanceParameters) -> Self {
        self.config.resonance = params;
        self
    }

    /// Set materialization parameters
    pub fn materialization(mut self, params: MaterializationParameters) -> Self {
        self.config.materialization = params;
        self
    }

    /// Set ledger parameters
    pub fn ledger(mut self, params: LedgerParameters) -> Self {
        self.config.ledger = params;
        self
    }

    /// Add an adapter configuration
    pub fn adapter(mut self, name: &str, config: AdapterConfig) -> Self {
        self.config.adapters.insert(name.to_string(), config);
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<KernelConfig> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = KernelConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .verbose(true)
            .mining(MiningParameters {
                max_iterations: 500,
                ..Default::default()
            })
            .build()
            .unwrap();

        assert!(config.verbose);
        assert_eq!(config.mining.max_iterations, 500);
    }

    #[test]
    fn test_config_save_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_config.toml");

        let config = KernelConfig {
            verbose: true,
            mining: MiningParameters {
                max_iterations: 200,
                target_resonance: 0.8,
                ..Default::default()
            },
            ..Default::default()
        };

        config.save(&path).unwrap();
        let loaded = KernelConfig::load(&path).unwrap();

        assert!(loaded.verbose);
        assert_eq!(loaded.mining.max_iterations, 200);
        assert!((loaded.mining.target_resonance - 0.8).abs() < 1e-6);
    }

    #[test]
    fn test_invalid_config() {
        let config = KernelConfig {
            resonance: ResonanceParameters {
                min_threshold: 1.5, // Invalid
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_resonance_weights_normalize() {
        let weights = ResonanceWeights {
            psi: 2.0,
            rho: 3.0,
            omega: 5.0,
            chi: 0.1,
            eta: 0.05,
        };

        let normalized = weights.normalize();
        let sum = normalized.psi + normalized.rho + normalized.omega;
        assert!((sum - 1.0).abs() < 1e-6);
    }
}
