//! TRITON session management for persistent optimization.

use crate::config::TritonConfig;
use crate::optimizer::{OptimizationResult, TritonOptimizer};
use crate::scoring::ScoringFunction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session name
    pub name: String,
    /// Optimizer configuration
    pub optimizer_config: TritonConfig,
    /// Auto-save interval (iterations)
    pub autosave_interval: Option<usize>,
    /// Maximum runtime (seconds)
    pub max_runtime: Option<u64>,
    /// Target score to achieve
    pub target_score: Option<f64>,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            name: "triton_session".to_string(),
            optimizer_config: TritonConfig::default(),
            autosave_interval: Some(100),
            max_runtime: None,
            target_score: None,
        }
    }
}

/// Event types during session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionEvent {
    /// Session started
    Started { timestamp: chrono::DateTime<chrono::Utc> },
    /// New best found
    NewBest { score: f64, iteration: usize },
    /// Layer completed
    LayerComplete { layer: usize, best_in_layer: f64 },
    /// Refinement started
    RefinementStarted { initial_score: f64 },
    /// Refinement completed
    RefinementComplete { final_score: f64, improvement: f64 },
    /// Session paused
    Paused { iteration: usize },
    /// Session resumed
    Resumed { iteration: usize },
    /// Session completed
    Completed { final_score: f64, total_iterations: usize },
    /// Error occurred
    Error { message: String },
}

/// Session log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLog {
    /// Log entries
    pub events: Vec<(chrono::DateTime<chrono::Utc>, SessionEvent)>,
    /// Run history (best scores over time)
    pub score_history: Vec<f64>,
    /// Parameters logged
    pub parameters: HashMap<String, String>,
}

impl SessionLog {
    /// Create new log
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            score_history: Vec::new(),
            parameters: HashMap::new(),
        }
    }

    /// Log an event
    pub fn log(&mut self, event: SessionEvent) {
        self.events.push((chrono::Utc::now(), event));
    }

    /// Record score
    pub fn record_score(&mut self, score: f64) {
        self.score_history.push(score);
    }

    /// Get events of a specific type
    pub fn events_of_type(&self, event_type: &str) -> Vec<&SessionEvent> {
        self.events.iter()
            .filter_map(|(_, e)| {
                let matches = match (event_type, e) {
                    ("Started", SessionEvent::Started { .. }) => true,
                    ("NewBest", SessionEvent::NewBest { .. }) => true,
                    ("Completed", SessionEvent::Completed { .. }) => true,
                    _ => false,
                };
                if matches { Some(e) } else { None }
            })
            .collect()
    }
}

impl Default for SessionLog {
    fn default() -> Self {
        Self::new()
    }
}

/// TRITON optimization session
pub struct TritonSession {
    /// Session ID
    pub id: String,
    /// Session configuration
    config: SessionConfig,
    /// Underlying optimizer
    optimizer: TritonOptimizer,
    /// Session log
    log: SessionLog,
    /// Is session running
    running: bool,
    /// Is session paused
    paused: bool,
    /// Start time
    start_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Best results from all runs
    best_results: Vec<OptimizationResult>,
}

impl TritonSession {
    /// Create new session
    pub fn new(config: SessionConfig) -> Self {
        let optimizer = TritonOptimizer::new(config.optimizer_config.clone());

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            config,
            optimizer,
            log: SessionLog::new(),
            running: false,
            paused: false,
            start_time: None,
            best_results: Vec::new(),
        }
    }

    /// Start the session
    pub fn start(&mut self) {
        self.running = true;
        self.paused = false;
        self.start_time = Some(chrono::Utc::now());
        self.log.log(SessionEvent::Started {
            timestamp: self.start_time.unwrap(),
        });
    }

    /// Run a single optimization
    pub fn run(&mut self) -> OptimizationResult {
        if !self.running {
            self.start();
        }

        let result = self.optimizer.optimize();

        self.log.log(SessionEvent::Completed {
            final_score: result.best_score,
            total_iterations: result.iterations,
        });

        self.best_results.push(result.clone());
        result
    }

    /// Run with custom scorer
    pub fn run_with_scorer<S: ScoringFunction>(&mut self, scorer: &S) -> OptimizationResult {
        if !self.running {
            self.start();
        }

        let result = self.optimizer.optimize_with_scorer(scorer);

        self.log.log(SessionEvent::Completed {
            final_score: result.best_score,
            total_iterations: result.iterations,
        });

        self.best_results.push(result.clone());
        result
    }

    /// Run multiple optimizations
    pub fn run_multi(&mut self, runs: usize) -> Vec<OptimizationResult> {
        let mut results = Vec::new();

        for i in 0..runs {
            self.optimizer.reset();
            let result = self.run();
            results.push(result);

            tracing::info!("Run {}/{} complete, score: {:.4}", i + 1, runs, results.last().unwrap().best_score);
        }

        results
    }

    /// Pause the session
    pub fn pause(&mut self) {
        if self.running && !self.paused {
            self.paused = true;
            self.log.log(SessionEvent::Paused {
                iteration: self.optimizer.state().iteration,
            });
        }
    }

    /// Resume the session
    pub fn resume(&mut self) {
        if self.running && self.paused {
            self.paused = false;
            self.log.log(SessionEvent::Resumed {
                iteration: self.optimizer.state().iteration,
            });
        }
    }

    /// Stop the session
    pub fn stop(&mut self) {
        self.running = false;
        self.paused = false;
    }

    /// Get session log
    pub fn log(&self) -> &SessionLog {
        &self.log
    }

    /// Get all results
    pub fn results(&self) -> &[OptimizationResult] {
        &self.best_results
    }

    /// Get best result overall
    pub fn best_result(&self) -> Option<&OptimizationResult> {
        self.best_results.iter()
            .max_by(|a, b| a.best_score.partial_cmp(&b.best_score).unwrap())
    }

    /// Get session statistics
    pub fn statistics(&self) -> SessionStatistics {
        let scores: Vec<f64> = self.best_results.iter().map(|r| r.best_score).collect();

        let mean = if scores.is_empty() {
            0.0
        } else {
            scores.iter().sum::<f64>() / scores.len() as f64
        };

        let variance = if scores.len() > 1 {
            scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / (scores.len() - 1) as f64
        } else {
            0.0
        };

        let best = scores.iter().copied().fold(f64::MIN, f64::max);
        let worst = scores.iter().copied().fold(f64::MAX, f64::min);

        SessionStatistics {
            total_runs: self.best_results.len(),
            mean_score: mean,
            std_dev: variance.sqrt(),
            best_score: if scores.is_empty() { 0.0 } else { best },
            worst_score: if scores.is_empty() { 0.0 } else { worst },
            total_iterations: self.best_results.iter().map(|r| r.iterations).sum(),
            convergence_rate: self.best_results.iter()
                .filter(|r| r.converged).count() as f64 / self.best_results.len().max(1) as f64,
        }
    }

    /// Export session to JSON
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        let export = SessionExport {
            id: self.id.clone(),
            config: self.config.clone(),
            log: self.log.clone(),
            results: self.best_results.clone(),
            statistics: self.statistics(),
        };
        serde_json::to_string_pretty(&export)
    }

    /// Is session active
    pub fn is_running(&self) -> bool {
        self.running && !self.paused
    }
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatistics {
    /// Total optimization runs
    pub total_runs: usize,
    /// Mean best score
    pub mean_score: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Best score achieved
    pub best_score: f64,
    /// Worst score achieved
    pub worst_score: f64,
    /// Total iterations across all runs
    pub total_iterations: usize,
    /// Convergence rate
    pub convergence_rate: f64,
}

/// Session export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionExport {
    /// Session ID
    pub id: String,
    /// Configuration
    pub config: SessionConfig,
    /// Log
    pub log: SessionLog,
    /// Results
    pub results: Vec<OptimizationResult>,
    /// Statistics
    pub statistics: SessionStatistics,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let config = SessionConfig {
            optimizer_config: TritonConfig::quick(),
            ..Default::default()
        };
        let session = TritonSession::new(config);
        assert!(!session.id.is_empty());
    }

    #[test]
    fn test_session_run() {
        let config = SessionConfig {
            optimizer_config: TritonConfig {
                max_iterations: 20,
                ..TritonConfig::quick()
            },
            ..Default::default()
        };
        let mut session = TritonSession::new(config);

        let result = session.run();
        assert!(result.best_score > 0.0);
        assert_eq!(session.results().len(), 1);
    }

    #[test]
    fn test_session_statistics() {
        let config = SessionConfig {
            optimizer_config: TritonConfig {
                max_iterations: 10,
                ..TritonConfig::quick()
            },
            ..Default::default()
        };
        let mut session = TritonSession::new(config);

        session.run();
        session.optimizer.reset();
        session.run();

        let stats = session.statistics();
        assert_eq!(stats.total_runs, 2);
    }
}
