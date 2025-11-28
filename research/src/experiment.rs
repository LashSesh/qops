//! Experiment framework for structured quantum research
//!
//! Provides tools for defining, running, and tracking experiments.

use crate::{ResearchError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Parameter definition for experiments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter values to sweep
    pub values: Vec<serde_json::Value>,
    /// Parameter description
    pub description: Option<String>,
}

impl Parameter {
    pub fn new<T: Serialize>(name: &str, values: Vec<T>) -> Self {
        Self {
            name: name.to_string(),
            values: values.into_iter()
                .map(|v| serde_json::to_value(v).unwrap())
                .collect(),
            description: None,
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn range(name: &str, start: i64, end: i64, step: i64) -> Self {
        let values: Vec<i64> = (start..=end).step_by(step as usize).collect();
        Self::new(name, values)
    }

    pub fn linspace(name: &str, start: f64, end: f64, n: usize) -> Self {
        let step = (end - start) / (n - 1) as f64;
        let values: Vec<f64> = (0..n).map(|i| start + i as f64 * step).collect();
        Self::new(name, values)
    }
}

/// Experiment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    /// Experiment name
    pub name: String,
    /// Description
    pub description: String,
    /// Parameters to sweep
    pub parameters: Vec<Parameter>,
    /// Number of repetitions per configuration
    pub repetitions: usize,
    /// Random seeds for reproducibility
    pub seeds: Option<Vec<u64>>,
    /// Tags for organization
    pub tags: Vec<String>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl ExperimentConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            parameters: Vec::new(),
            repetitions: 1,
            seeds: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn parameter(mut self, param: Parameter) -> Self {
        self.parameters.push(param);
        self
    }

    pub fn repetitions(mut self, n: usize) -> Self {
        self.repetitions = n;
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Generate all parameter combinations
    pub fn parameter_grid(&self) -> Vec<HashMap<String, serde_json::Value>> {
        if self.parameters.is_empty() {
            return vec![HashMap::new()];
        }

        let mut grid = vec![HashMap::new()];

        for param in &self.parameters {
            let mut new_grid = Vec::new();
            for combo in &grid {
                for value in &param.values {
                    let mut new_combo = combo.clone();
                    new_combo.insert(param.name.clone(), value.clone());
                    new_grid.push(new_combo);
                }
            }
            grid = new_grid;
        }

        grid
    }

    /// Total number of runs
    pub fn total_runs(&self) -> usize {
        self.parameter_grid().len() * self.repetitions
    }
}

/// Single experiment run result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResult {
    /// Run index
    pub index: usize,
    /// Parameters used
    pub parameters: HashMap<String, serde_json::Value>,
    /// Repetition index
    pub repetition: usize,
    /// Result data
    pub data: HashMap<String, serde_json::Value>,
    /// Timing information
    pub duration_ms: f64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Success status
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

/// Complete experiment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    /// Unique identifier
    pub id: String,
    /// Configuration used
    pub config: ExperimentConfig,
    /// All run results
    pub runs: Vec<RunResult>,
    /// Start time
    pub started_at: DateTime<Utc>,
    /// Completion time
    pub completed_at: Option<DateTime<Utc>>,
    /// Overall status
    pub status: ExperimentStatus,
}

/// Experiment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExperimentStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl ExperimentResult {
    /// Get results for a specific parameter combination
    pub fn filter_by_params(&self, params: &HashMap<String, serde_json::Value>) -> Vec<&RunResult> {
        self.runs.iter()
            .filter(|run| {
                params.iter().all(|(k, v)| {
                    run.parameters.get(k) == Some(v)
                })
            })
            .collect()
    }

    /// Get all unique values for a parameter
    pub fn unique_values(&self, param: &str) -> Vec<serde_json::Value> {
        let mut values: Vec<serde_json::Value> = self.runs.iter()
            .filter_map(|run| run.parameters.get(param).cloned())
            .collect();
        values.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        values.dedup();
        values
    }

    /// Get mean of a result field grouped by parameter
    pub fn mean_by_param(&self, result_field: &str, group_param: &str) -> HashMap<String, f64> {
        let mut grouped: HashMap<String, Vec<f64>> = HashMap::new();

        for run in &self.runs {
            if let (Some(param_val), Some(result_val)) = (
                run.parameters.get(group_param),
                run.data.get(result_field).and_then(|v| v.as_f64())
            ) {
                let key = param_val.to_string();
                grouped.entry(key).or_default().push(result_val);
            }
        }

        grouped.into_iter()
            .map(|(k, v)| (k, v.iter().sum::<f64>() / v.len() as f64))
            .collect()
    }

    /// Success rate
    pub fn success_rate(&self) -> f64 {
        if self.runs.is_empty() {
            return 0.0;
        }
        let successful = self.runs.iter().filter(|r| r.success).count();
        successful as f64 / self.runs.len() as f64
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| ResearchError::SerializationError(e.to_string()))
    }

    /// Summary statistics
    pub fn summary(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Experiment: {}\n", self.config.name));
        s.push_str(&format!("ID: {}\n", self.id));
        s.push_str(&format!("Status: {:?}\n", self.status));
        s.push_str(&format!("Total runs: {}\n", self.runs.len()));
        s.push_str(&format!("Success rate: {:.1}%\n", self.success_rate() * 100.0));

        if !self.config.parameters.is_empty() {
            s.push_str("\nParameters:\n");
            for param in &self.config.parameters {
                s.push_str(&format!("  - {}: {} values\n", param.name, param.values.len()));
            }
        }

        s
    }
}

/// Main experiment runner
pub struct Experiment {
    config: ExperimentConfig,
    result: ExperimentResult,
}

impl Experiment {
    pub fn new(name: &str) -> Self {
        let config = ExperimentConfig::new(name);
        let result = ExperimentResult {
            id: Uuid::new_v4().to_string(),
            config: config.clone(),
            runs: Vec::new(),
            started_at: Utc::now(),
            completed_at: None,
            status: ExperimentStatus::Pending,
        };

        Self { config, result }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.config = self.config.description(desc);
        self.result.config = self.config.clone();
        self
    }

    pub fn parameter(mut self, param: Parameter) -> Self {
        self.config = self.config.parameter(param);
        self.result.config = self.config.clone();
        self
    }

    pub fn repetitions(mut self, n: usize) -> Self {
        self.config = self.config.repetitions(n);
        self.result.config = self.config.clone();
        self
    }

    /// Run the experiment with a callback
    pub fn run<F>(mut self, mut f: F) -> ExperimentResult
    where
        F: FnMut(&HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value>,
    {
        self.result.status = ExperimentStatus::Running;
        self.result.started_at = Utc::now();

        let grid = self.config.parameter_grid();
        let mut run_index = 0;

        for params in &grid {
            for rep in 0..self.config.repetitions {
                let start = std::time::Instant::now();

                let (data, success, error) = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| f(params))
                ) {
                    Ok(data) => (data, true, None),
                    Err(e) => {
                        let err_msg = e.downcast_ref::<&str>()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "Unknown error".to_string());
                        (HashMap::new(), false, Some(err_msg))
                    }
                };

                let run_result = RunResult {
                    index: run_index,
                    parameters: params.clone(),
                    repetition: rep,
                    data,
                    duration_ms: start.elapsed().as_secs_f64() * 1000.0,
                    timestamp: Utc::now(),
                    success,
                    error,
                };

                self.result.runs.push(run_result);
                run_index += 1;
            }
        }

        self.result.completed_at = Some(Utc::now());
        self.result.status = if self.result.runs.iter().all(|r| r.success) {
            ExperimentStatus::Completed
        } else {
            ExperimentStatus::Failed
        };

        self.result
    }

    /// Run with progress callback
    pub fn run_with_progress<F, P>(mut self, mut f: F, mut progress: P) -> ExperimentResult
    where
        F: FnMut(&HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value>,
        P: FnMut(usize, usize),
    {
        self.result.status = ExperimentStatus::Running;
        self.result.started_at = Utc::now();

        let grid = self.config.parameter_grid();
        let total = grid.len() * self.config.repetitions;
        let mut run_index = 0;

        for params in &grid {
            for rep in 0..self.config.repetitions {
                progress(run_index, total);

                let start = std::time::Instant::now();
                let data = f(params);

                let run_result = RunResult {
                    index: run_index,
                    parameters: params.clone(),
                    repetition: rep,
                    data,
                    duration_ms: start.elapsed().as_secs_f64() * 1000.0,
                    timestamp: Utc::now(),
                    success: true,
                    error: None,
                };

                self.result.runs.push(run_result);
                run_index += 1;
            }
        }

        self.result.completed_at = Some(Utc::now());
        self.result.status = ExperimentStatus::Completed;
        self.result
    }
}

/// Experiment templates for common quantum research tasks
pub mod templates {
    use super::*;

    /// Template for algorithm scaling study
    pub fn scaling_study(name: &str, qubit_range: std::ops::Range<usize>) -> Experiment {
        Experiment::new(name)
            .description("Study algorithm scaling with problem size")
            .parameter(Parameter::new("qubits", qubit_range.collect::<Vec<_>>()))
            .repetitions(5)
    }

    /// Template for parameter sweep
    pub fn parameter_sweep(name: &str, param_name: &str, values: Vec<f64>) -> Experiment {
        Experiment::new(name)
            .description(&format!("Sweep over {} parameter", param_name))
            .parameter(Parameter::new(param_name, values))
            .repetitions(3)
    }

    /// Template for noise study
    pub fn noise_study(name: &str) -> Experiment {
        Experiment::new(name)
            .description("Study algorithm performance under noise")
            .parameter(Parameter::linspace("noise_level", 0.0, 0.1, 11))
            .repetitions(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_grid() {
        let config = ExperimentConfig::new("test")
            .parameter(Parameter::new("a", vec![1, 2]))
            .parameter(Parameter::new("b", vec!["x", "y"]));

        let grid = config.parameter_grid();
        assert_eq!(grid.len(), 4); // 2 * 2 combinations
    }

    #[test]
    fn test_experiment_run() {
        let result = Experiment::new("test")
            .parameter(Parameter::new("x", vec![1, 2, 3]))
            .repetitions(2)
            .run(|params| {
                let x = params.get("x").unwrap().as_i64().unwrap();
                let mut data = HashMap::new();
                data.insert("result".to_string(), serde_json::json!(x * 2));
                data
            });

        assert_eq!(result.runs.len(), 6); // 3 params * 2 reps
        assert_eq!(result.status, ExperimentStatus::Completed);
    }

    #[test]
    fn test_linspace_parameter() {
        let param = Parameter::linspace("theta", 0.0, 1.0, 5);
        assert_eq!(param.values.len(), 5);
    }
}
