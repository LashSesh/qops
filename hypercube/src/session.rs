//! Hypercube Session Management
//!
//! Provides session-based workflow for hypercube operations.

use crate::cube::{Hypercube, HypercubeConfig, CubeExpansionRule};
use crate::compiler::{HypercubeCompiler, CompilationConfig, CompilationResult};
use crate::artifact::ArtifactCollection;
use crate::coordinates::Coord5D;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session name
    pub name: String,
    /// Hypercube configuration
    pub cube_config: HypercubeConfig,
    /// Compilation configuration
    pub compilation_config: CompilationConfig,
    /// Auto-expand on creation
    pub auto_expand: bool,
    /// Number of auto-expansion steps
    pub expansion_steps: usize,
    /// Auto-compile after expansion
    pub auto_compile: bool,
    /// Export results on completion
    pub export_results: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            name: "default_session".to_string(),
            cube_config: HypercubeConfig::default(),
            compilation_config: CompilationConfig::default(),
            auto_expand: true,
            expansion_steps: 5,
            auto_compile: true,
            export_results: false,
        }
    }
}

impl SessionConfig {
    /// Create a quick session config
    pub fn quick() -> Self {
        Self {
            name: "quick_session".to_string(),
            cube_config: HypercubeConfig {
                max_depth: 3,
                expansion_rule: CubeExpansionRule::Triton,
                ..Default::default()
            },
            compilation_config: CompilationConfig::fast(),
            expansion_steps: 3,
            ..Default::default()
        }
    }

    /// Create a thorough session config
    pub fn thorough() -> Self {
        Self {
            name: "thorough_session".to_string(),
            cube_config: HypercubeConfig {
                max_depth: 10,
                expansion_rule: CubeExpansionRule::HybridTriton,
                ..Default::default()
            },
            compilation_config: CompilationConfig::optimized(),
            expansion_steps: 10,
            ..Default::default()
        }
    }

    /// Create a research session config
    pub fn research() -> Self {
        Self {
            name: "research_session".to_string(),
            cube_config: HypercubeConfig {
                max_depth: 20,
                expansion_rule: CubeExpansionRule::OperatorDriven,
                resonance_threshold: 0.6,
                ..Default::default()
            },
            compilation_config: CompilationConfig::research(),
            expansion_steps: 15,
            export_results: true,
            ..Default::default()
        }
    }
}

/// Session state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    /// Session created
    Created,
    /// Expansion in progress
    Expanding,
    /// Expansion complete
    Expanded,
    /// Compilation in progress
    Compiling,
    /// Session complete
    Completed,
    /// Session failed
    Failed,
}

/// Session result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    /// Session ID
    pub session_id: String,
    /// Final state
    pub state: SessionState,
    /// Best coordinate found
    pub best_coordinate: Coord5D,
    /// Best resonance
    pub best_resonance: f64,
    /// Compilation result (if compiled)
    pub compilation_result: Option<CompilationResult>,
    /// Total session time (ms)
    pub total_time_ms: u64,
    /// Number of expansion steps performed
    pub expansion_steps: usize,
    /// Total vertices generated
    pub total_vertices: usize,
    /// Artifact count
    pub artifact_count: usize,
}

/// Hypercube session
pub struct HypercubeSession {
    /// Session ID
    pub id: String,
    /// Configuration
    pub config: SessionConfig,
    /// Current state
    pub state: SessionState,
    /// The hypercube
    cube: Hypercube,
    /// Compiler
    compiler: HypercubeCompiler,
    /// Collected artifacts
    artifacts: ArtifactCollection,
    /// Session start time
    start_time: std::time::Instant,
    /// Expansion step count
    expansion_count: usize,
}

impl HypercubeSession {
    /// Create a new session
    pub fn new(config: SessionConfig) -> Self {
        let cube = Hypercube::new(&config.name, config.cube_config.clone());
        let compiler = HypercubeCompiler::new(config.compilation_config.clone());

        Self {
            id: Uuid::new_v4().to_string(),
            config,
            state: SessionState::Created,
            cube,
            compiler,
            artifacts: ArtifactCollection::new(),
            start_time: std::time::Instant::now(),
            expansion_count: 0,
        }
    }

    /// Create with default config
    pub fn default_session() -> Self {
        Self::new(SessionConfig::default())
    }

    /// Create a quick session
    pub fn quick_session() -> Self {
        Self::new(SessionConfig::quick())
    }

    /// Run the full session pipeline
    pub fn run(&mut self) -> Result<SessionResult> {
        self.start_time = std::time::Instant::now();

        // Expansion phase
        if self.config.auto_expand {
            self.expand(self.config.expansion_steps)?;
        }

        // Compilation phase
        let compilation_result = if self.config.auto_compile {
            Some(self.compile()?)
        } else {
            None
        };

        self.state = SessionState::Completed;

        Ok(self.build_result(compilation_result))
    }

    /// Run expansion steps
    pub fn expand(&mut self, steps: usize) -> Result<usize> {
        self.state = SessionState::Expanding;
        let mut total_new = 0;

        for _ in 0..steps {
            let new_count = self.cube.expand_step()?;
            total_new += new_count;
            self.expansion_count += 1;

            if new_count == 0 {
                break; // No more expansion possible
            }
        }

        self.state = SessionState::Expanded;
        Ok(total_new)
    }

    /// Run compilation
    pub fn compile(&mut self) -> Result<CompilationResult> {
        self.state = SessionState::Compiling;
        let result = self.compiler.compile(&mut self.cube)?;

        // Collect artifacts
        for artifact in &result.artifacts {
            self.artifacts.add(artifact.clone());
        }

        Ok(result)
    }

    /// Get the hypercube
    pub fn cube(&self) -> &Hypercube {
        &self.cube
    }

    /// Get mutable hypercube
    pub fn cube_mut(&mut self) -> &mut Hypercube {
        &mut self.cube
    }

    /// Get artifacts
    pub fn artifacts(&self) -> &ArtifactCollection {
        &self.artifacts
    }

    /// Get best coordinate
    pub fn best_coordinate(&self) -> Coord5D {
        self.cube.best_vertex()
            .map(|v| v.coordinate)
            .unwrap_or(Coord5D::center())
    }

    /// Get best resonance
    pub fn best_resonance(&self) -> f64 {
        self.cube.best_resonance
    }

    /// Build session result
    fn build_result(&self, compilation_result: Option<CompilationResult>) -> SessionResult {
        SessionResult {
            session_id: self.id.clone(),
            state: self.state,
            best_coordinate: self.best_coordinate(),
            best_resonance: self.best_resonance(),
            compilation_result,
            total_time_ms: self.start_time.elapsed().as_millis() as u64,
            expansion_steps: self.expansion_count,
            total_vertices: self.cube.vertices.len(),
            artifact_count: self.artifacts.len(),
        }
    }

    /// Export session to JSON
    pub fn export_json(&self) -> Result<String> {
        self.cube.to_json()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = HypercubeSession::default_session();
        assert_eq!(session.state, SessionState::Created);
    }

    #[test]
    fn test_quick_session() {
        let mut session = HypercubeSession::quick_session();
        let result = session.run().unwrap();

        assert_eq!(result.state, SessionState::Completed);
        assert!(result.best_resonance > 0.0);
    }

    #[test]
    fn test_session_expansion() {
        let mut session = HypercubeSession::default_session();
        let new_vertices = session.expand(3).unwrap();

        assert!(new_vertices > 0 || session.cube.vertices.len() > 0);
        assert_eq!(session.state, SessionState::Expanded);
    }
}
