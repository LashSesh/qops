//! Generative pipeline abstraction.
//!
//! This module defines the [`GenerativePipeline`] trait that provides a unified
//! interface for different processing pipelines in QOPS:
//! - Genesis Pipeline (operator mining)
//! - Quantum Pipeline (VQA, quantum walks)
//! - Custom pipelines

use crate::error::Result;
use crate::signature::Signature;
use serde::{Deserialize, Serialize};

/// Pipeline execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineState {
    /// Pipeline is not initialized
    Uninitialized,
    /// Pipeline is ready to process
    Ready,
    /// Pipeline is currently processing
    Running,
    /// Pipeline has completed successfully
    Completed,
    /// Pipeline encountered an error
    Error,
    /// Pipeline is paused
    Paused,
}

impl Default for PipelineState {
    fn default() -> Self {
        Self::Uninitialized
    }
}

/// Configuration for a pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Maximum number of steps
    pub max_steps: usize,
    /// Convergence threshold
    pub convergence_threshold: f64,
    /// Enable adaptive feedback
    pub adaptive_feedback: bool,
    /// Feedback strength
    pub feedback_strength: f64,
    /// Logging verbosity
    pub verbose: bool,
    /// Checkpoint interval (0 = no checkpoints)
    pub checkpoint_interval: usize,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_steps: 1000,
            convergence_threshold: 0.001,
            adaptive_feedback: true,
            feedback_strength: 0.1,
            verbose: false,
            checkpoint_interval: 100,
        }
    }
}

/// Result of a pipeline step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult<O> {
    /// Step number
    pub step: usize,
    /// Output of this step
    pub output: O,
    /// Performance signature after this step
    pub performance: Signature,
    /// Whether convergence was reached
    pub converged: bool,
    /// Additional metrics
    pub metrics: std::collections::HashMap<String, f64>,
}

impl<O> StepResult<O> {
    /// Create a new step result
    pub fn new(step: usize, output: O, performance: Signature) -> Self {
        Self {
            step,
            output,
            performance,
            converged: false,
            metrics: std::collections::HashMap::new(),
        }
    }
}

/// Trait for generative processing pipelines
pub trait GenerativePipeline: Send + Sync {
    /// Input type for the pipeline
    type Input;
    /// Output type for the pipeline
    type Output: Clone;
    /// Configuration type
    type Config;

    /// Initialize the pipeline with configuration
    fn initialize(&mut self, config: Self::Config) -> Result<()>;

    /// Execute a single processing step
    fn step(&mut self, input: Self::Input) -> Result<StepResult<Self::Output>>;

    /// Get current performance signature
    fn get_performance(&self) -> Signature;

    /// Apply calibration feedback
    fn apply_calibration(&mut self, feedback: &Signature);

    /// Get current state
    fn state(&self) -> PipelineState;

    /// Reset the pipeline
    fn reset(&mut self);

    /// Check if the pipeline has converged
    fn has_converged(&self) -> bool;

    /// Get pipeline name
    fn name(&self) -> &str;
}

/// Simple pipeline implementation for demonstration
#[derive(Debug, Clone)]
pub struct SimplePipeline {
    name: String,
    state: PipelineState,
    config: PipelineConfig,
    current_step: usize,
    performance: Signature,
    converged: bool,
}

impl SimplePipeline {
    /// Create a new simple pipeline
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: PipelineState::Uninitialized,
            config: PipelineConfig::default(),
            current_step: 0,
            performance: Signature::default(),
            converged: false,
        }
    }
}

impl GenerativePipeline for SimplePipeline {
    type Input = Vec<f64>;
    type Output = Vec<f64>;
    type Config = PipelineConfig;

    fn initialize(&mut self, config: Self::Config) -> Result<()> {
        self.config = config;
        self.state = PipelineState::Ready;
        self.current_step = 0;
        self.converged = false;
        Ok(())
    }

    fn step(&mut self, input: Self::Input) -> Result<StepResult<Self::Output>> {
        self.state = PipelineState::Running;
        self.current_step += 1;

        // Simple processing: normalize and apply feedback
        let output: Vec<f64> = input.iter().map(|x| x.tanh()).collect();

        // Update performance based on output
        let quality = output.iter().map(|x| x.abs()).sum::<f64>() / output.len() as f64;
        let stability = 1.0 - output.iter().map(|x| (x - quality).powi(2)).sum::<f64>().sqrt();
        let efficiency = 1.0 / (self.current_step as f64).sqrt();

        self.performance = Signature::D3(crate::signature::Signature3D::new(
            quality.min(1.0),
            stability.max(0.0).min(1.0),
            efficiency.min(1.0),
        ));

        // Check convergence
        if self.current_step >= self.config.max_steps {
            self.converged = true;
            self.state = PipelineState::Completed;
        }

        Ok(StepResult::new(
            self.current_step,
            output,
            self.performance,
        ))
    }

    fn get_performance(&self) -> Signature {
        self.performance
    }

    fn apply_calibration(&mut self, feedback: &Signature) {
        if self.config.adaptive_feedback {
            // Blend current performance with feedback
            let current = self.performance.to_3d();
            let target = feedback.to_3d();
            let strength = self.config.feedback_strength;

            self.performance = Signature::D3(crate::signature::Signature3D::new(
                current.psi + strength * (target.psi - current.psi),
                current.rho + strength * (target.rho - current.rho),
                current.omega + strength * (target.omega - current.omega),
            ));
        }
    }

    fn state(&self) -> PipelineState {
        self.state
    }

    fn reset(&mut self) {
        self.state = PipelineState::Ready;
        self.current_step = 0;
        self.performance = Signature::default();
        self.converged = false;
    }

    fn has_converged(&self) -> bool {
        self.converged
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Pipeline registry for managing multiple pipelines
#[derive(Default)]
pub struct PipelineRegistry {
    pipelines: std::collections::HashMap<String, Box<dyn PipelineMeta>>,
}

/// Trait object wrapper for pipeline metadata
pub trait PipelineMeta: Send + Sync {
    fn name(&self) -> &str;
    fn state(&self) -> PipelineState;
    fn performance(&self) -> Signature;
}

impl PipelineRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a pipeline
    pub fn register(&mut self, pipeline: Box<dyn PipelineMeta>) {
        let name = pipeline.name().to_string();
        self.pipelines.insert(name, pipeline);
    }

    /// Get pipeline names
    pub fn names(&self) -> Vec<&str> {
        self.pipelines.keys().map(|s| s.as_str()).collect()
    }

    /// Get a pipeline by name
    pub fn get(&self, name: &str) -> Option<&dyn PipelineMeta> {
        self.pipelines.get(name).map(|p| p.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pipeline() {
        let mut pipeline = SimplePipeline::new("test");
        pipeline.initialize(PipelineConfig::default()).unwrap();

        assert_eq!(pipeline.state(), PipelineState::Ready);

        let result = pipeline.step(vec![0.5, 0.6, 0.7]).unwrap();
        assert_eq!(result.step, 1);
        assert!(!result.output.is_empty());
    }

    #[test]
    fn test_pipeline_calibration() {
        let mut pipeline = SimplePipeline::new("test");
        pipeline.initialize(PipelineConfig::default()).unwrap();

        pipeline.step(vec![0.5, 0.5, 0.5]).unwrap();
        let perf_before = pipeline.get_performance();

        let target = Signature::D3(crate::signature::Signature3D::new(1.0, 1.0, 1.0));
        pipeline.apply_calibration(&target);

        let perf_after = pipeline.get_performance();
        assert!(perf_after.psi() > perf_before.psi());
    }
}
