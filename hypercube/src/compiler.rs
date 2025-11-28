//! Hypercube Compiler
//!
//! Compiles hypercube structures into executable artifacts using the Ξ operator.

use crate::cube::{Hypercube, HypercubeState};
use crate::hdag::{HDAG, HDAGExecutor, ExecutionResult};
use crate::artifact::{HypercubeArtifact, ArtifactType, ArtifactCollection};
use crate::coordinates::Coord5D;
use crate::operators::{OperatorFamily, CompilationOperator, CompilationMode};
use crate::error::{HypercubeError, Result};
use serde::{Deserialize, Serialize};

/// Configuration for hypercube compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationConfig {
    /// Compilation mode
    pub mode: CompilationMode,
    /// Target resonance threshold
    pub resonance_threshold: f64,
    /// Maximum compilation iterations
    pub max_iterations: usize,
    /// Generate intermediate checkpoints
    pub checkpoints: bool,
    /// Extract operator families
    pub extract_families: bool,
    /// Use HDAG for execution
    pub use_hdag: bool,
    /// Parallel compilation (for multiple seeds)
    pub parallel: bool,
}

impl Default for CompilationConfig {
    fn default() -> Self {
        Self {
            mode: CompilationMode::Balanced,
            resonance_threshold: 0.8,
            max_iterations: 100,
            checkpoints: false,
            extract_families: true,
            use_hdag: true,
            parallel: false,
        }
    }
}

impl CompilationConfig {
    /// Create fast compilation config
    pub fn fast() -> Self {
        Self {
            mode: CompilationMode::Fast,
            max_iterations: 20,
            ..Default::default()
        }
    }

    /// Create optimized compilation config
    pub fn optimized() -> Self {
        Self {
            mode: CompilationMode::Optimized,
            max_iterations: 200,
            checkpoints: true,
            ..Default::default()
        }
    }

    /// Create research compilation config
    pub fn research() -> Self {
        Self {
            mode: CompilationMode::Research,
            max_iterations: 500,
            checkpoints: true,
            extract_families: true,
            ..Default::default()
        }
    }
}

/// Result of hypercube compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    /// Primary output coordinate
    pub output: Coord5D,
    /// Output resonance
    pub resonance: f64,
    /// Generated artifacts
    pub artifacts: Vec<HypercubeArtifact>,
    /// Extracted operator families
    pub families: Vec<String>,
    /// Total compilation time (ms)
    pub compilation_time_ms: u64,
    /// Number of iterations performed
    pub iterations: usize,
    /// Did compilation meet threshold
    pub threshold_met: bool,
    /// HDAG execution result (if used)
    pub hdag_result: Option<ExecutionResult>,
    /// Compilation statistics
    pub stats: CompilationStats,
}

/// Compilation statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompilationStats {
    /// Initial resonance
    pub initial_resonance: f64,
    /// Final resonance
    pub final_resonance: f64,
    /// Resonance improvement
    pub resonance_delta: f64,
    /// Number of operator applications
    pub operator_applications: usize,
    /// Number of checkpoints generated
    pub checkpoint_count: usize,
    /// Average resonance during compilation
    pub avg_resonance: f64,
    /// Maximum resonance achieved
    pub max_resonance: f64,
}

/// The Hypercube Compiler
pub struct HypercubeCompiler {
    config: CompilationConfig,
    xi_operator: CompilationOperator,
    artifacts: ArtifactCollection,
}

impl HypercubeCompiler {
    /// Create a new compiler
    pub fn new(config: CompilationConfig) -> Self {
        let xi_operator = CompilationOperator::new(config.mode)
            .with_threshold(config.resonance_threshold);

        Self {
            config,
            xi_operator,
            artifacts: ArtifactCollection::new(),
        }
    }

    /// Create with default config
    pub fn default_compiler() -> Self {
        Self::new(CompilationConfig::default())
    }

    /// Compile a hypercube
    pub fn compile(&mut self, cube: &mut Hypercube) -> Result<CompilationResult> {
        let start = std::time::Instant::now();

        // Ensure cube is ready for compilation
        if !matches!(cube.state, HypercubeState::ReadyToCompile | HypercubeState::Initialized | HypercubeState::Expanding) {
            cube.state = HypercubeState::ReadyToCompile;
        }

        cube.state = HypercubeState::Compiling;

        let result = if self.config.use_hdag {
            self.compile_with_hdag(cube)?
        } else {
            self.compile_direct(cube)?
        };

        cube.state = HypercubeState::Compiled;

        let mut final_result = result;
        final_result.compilation_time_ms = start.elapsed().as_millis() as u64;

        Ok(final_result)
    }

    /// Direct compilation without HDAG
    fn compile_direct(&mut self, cube: &Hypercube) -> Result<CompilationResult> {
        let mut stats = CompilationStats::default();
        let mut iterations = 0;
        let mut resonance_sum = 0.0;

        // Get best vertices to compile
        let vertices = cube.vertices_by_resonance();
        if vertices.is_empty() {
            return Err(HypercubeError::CompilationError("No vertices to compile".to_string()));
        }

        let initial_coord = vertices[0].coordinate;
        stats.initial_resonance = initial_coord.resonance();

        // Build operator family from cube
        let family = if !cube.operator_families.is_empty() {
            cube.operator_families[0].clone()
        } else {
            OperatorFamily::resonance_optimized("default", initial_coord)
        };

        // Iterative compilation
        let mut current = initial_coord;
        let mut max_resonance = current.resonance();

        for i in 0..self.config.max_iterations {
            let new_coord = self.xi_operator.compile_family(&family);
            let res = new_coord.resonance();

            resonance_sum += res;
            iterations = i + 1;

            if res > max_resonance {
                max_resonance = res;
                current = new_coord;
            }

            // Generate checkpoint
            if self.config.checkpoints && i % 10 == 0 {
                let checkpoint = HypercubeArtifact::checkpoint(
                    &format!("checkpoint_{}", i),
                    current,
                    &cube.id,
                    "",
                );
                self.artifacts.add(checkpoint);
                stats.checkpoint_count += 1;
            }

            // Check if threshold met
            if res >= self.config.resonance_threshold {
                break;
            }

            stats.operator_applications += family.operators.len();
        }

        stats.final_resonance = current.resonance();
        stats.resonance_delta = stats.final_resonance - stats.initial_resonance;
        stats.max_resonance = max_resonance;
        stats.avg_resonance = if iterations > 0 { resonance_sum / iterations as f64 } else { 0.0 };

        // Generate final artifact
        let artifact = HypercubeArtifact::compiled_family(
            &format!("{}_compiled", cube.name),
            current,
            family.operator_sequence.iter().map(|t| t.code().to_string()).collect(),
        );
        self.artifacts.add(artifact.clone());

        Ok(CompilationResult {
            output: current,
            resonance: current.resonance(),
            artifacts: vec![artifact],
            families: vec![family.name],
            compilation_time_ms: 0,
            iterations,
            threshold_met: current.resonance() >= self.config.resonance_threshold,
            hdag_result: None,
            stats,
        })
    }

    /// Compilation using HDAG
    fn compile_with_hdag(&mut self, cube: &Hypercube) -> Result<CompilationResult> {
        let mut stats = CompilationStats::default();

        // Get seed from best vertex
        let seed = cube.best_vertex()
            .map(|v| v.coordinate)
            .unwrap_or(Coord5D::center());

        stats.initial_resonance = seed.resonance();

        // Create and execute HDAG
        let hdag = HDAG::standard_pipeline(seed);
        let mut executor = HDAGExecutor::new(hdag);

        let exec_result = executor.execute()?;

        stats.final_resonance = exec_result.resonance;
        stats.resonance_delta = stats.final_resonance - stats.initial_resonance;
        stats.max_resonance = exec_result.resonance;
        stats.operator_applications = exec_result.nodes_executed;

        // Generate artifact
        let artifact = HypercubeArtifact::new(
            &format!("{}_hdag_output", cube.name),
            ArtifactType::CompiledFamily,
            exec_result.output,
        );
        self.artifacts.add(artifact.clone());

        Ok(CompilationResult {
            output: exec_result.output,
            resonance: exec_result.resonance,
            artifacts: vec![artifact],
            families: vec!["HDAG Pipeline".to_string()],
            compilation_time_ms: exec_result.total_time_ms,
            iterations: exec_result.nodes_executed,
            threshold_met: exec_result.resonance >= self.config.resonance_threshold,
            hdag_result: Some(exec_result),
            stats,
        })
    }

    /// Compile a coordinate directly (without cube)
    pub fn compile_coordinate(&mut self, coord: Coord5D) -> Result<CompilationResult> {
        let start = std::time::Instant::now();
        let mut stats = CompilationStats::default();

        stats.initial_resonance = coord.resonance();

        let hdag = HDAG::standard_pipeline(coord);
        let mut executor = HDAGExecutor::new(hdag);
        let exec_result = executor.execute()?;

        stats.final_resonance = exec_result.resonance;
        stats.resonance_delta = stats.final_resonance - stats.initial_resonance;

        let artifact = HypercubeArtifact::optimized_coordinate("direct_compile", exec_result.output);

        Ok(CompilationResult {
            output: exec_result.output,
            resonance: exec_result.resonance,
            artifacts: vec![artifact],
            families: vec![],
            compilation_time_ms: start.elapsed().as_millis() as u64,
            iterations: exec_result.nodes_executed,
            threshold_met: exec_result.resonance >= self.config.resonance_threshold,
            hdag_result: Some(exec_result),
            stats,
        })
    }

    /// Get collected artifacts
    pub fn artifacts(&self) -> &ArtifactCollection {
        &self.artifacts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::HypercubeConfig;

    #[test]
    fn test_compiler_creation() {
        let compiler = HypercubeCompiler::default_compiler();
        assert_eq!(compiler.config.mode, CompilationMode::Balanced);
    }

    #[test]
    fn test_direct_compilation() {
        let mut cube = Hypercube::default_cube("test");
        let config = CompilationConfig {
            use_hdag: false,
            max_iterations: 10,
            ..Default::default()
        };
        let mut compiler = HypercubeCompiler::new(config);

        let result = compiler.compile(&mut cube).unwrap();
        assert!(result.resonance > 0.0);
    }

    #[test]
    fn test_hdag_compilation() {
        let mut cube = Hypercube::default_cube("test");
        let config = CompilationConfig {
            use_hdag: true,
            ..Default::default()
        };
        let mut compiler = HypercubeCompiler::new(config);

        let result = compiler.compile(&mut cube).unwrap();
        assert!(result.hdag_result.is_some());
    }

    #[test]
    fn test_coordinate_compilation() {
        let mut compiler = HypercubeCompiler::default_compiler();
        let coord = Coord5D::center();

        let result = compiler.compile_coordinate(coord).unwrap();
        assert!(result.resonance > 0.0);
    }
}
