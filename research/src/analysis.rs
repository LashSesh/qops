//! Statistical analysis for quantum computing results
//!
//! Provides tools for analyzing measurement outcomes and algorithm performance.

use crate::ExperimentResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Statistical summary of a dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalSummary {
    /// Number of samples
    pub count: usize,
    /// Mean value
    pub mean: f64,
    /// Standard deviation
    pub std: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Median value
    pub median: f64,
    /// 25th percentile
    pub q1: f64,
    /// 75th percentile
    pub q3: f64,
    /// Variance
    pub variance: f64,
    /// Standard error of mean
    pub sem: f64,
}

impl StatisticalSummary {
    /// Compute summary from data
    pub fn from_data(data: &[f64]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let count = data.len();
        let mean = data.iter().sum::<f64>() / count as f64;

        let variance = if count > 1 {
            data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (count - 1) as f64
        } else {
            0.0
        };

        let std = variance.sqrt();
        let sem = std / (count as f64).sqrt();

        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = sorted[0];
        let max = sorted[count - 1];
        let median = percentile(&sorted, 50.0);
        let q1 = percentile(&sorted, 25.0);
        let q3 = percentile(&sorted, 75.0);

        Some(Self {
            count,
            mean,
            std,
            min,
            max,
            median,
            q1,
            q3,
            variance,
            sem,
        })
    }

    /// 95% confidence interval
    pub fn confidence_interval_95(&self) -> (f64, f64) {
        let margin = 1.96 * self.sem;
        (self.mean - margin, self.mean + margin)
    }

    /// Format as string
    pub fn to_string_detailed(&self) -> String {
        let (ci_low, ci_high) = self.confidence_interval_95();
        format!(
            "n={}, mean={:.4} ± {:.4}, median={:.4}, range=[{:.4}, {:.4}], 95% CI=[{:.4}, {:.4}]",
            self.count, self.mean, self.std, self.median, self.min, self.max, ci_low, ci_high
        )
    }
}

/// Calculate percentile from sorted data
fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Quantum-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMetrics {
    /// Fidelity with target state
    pub fidelity: Option<f64>,
    /// Trace distance from target
    pub trace_distance: Option<f64>,
    /// Entanglement entropy
    pub entanglement_entropy: Option<f64>,
    /// Quantum volume estimate
    pub quantum_volume: Option<usize>,
    /// Circuit metrics
    pub gate_count: Option<usize>,
    pub circuit_depth: Option<usize>,
    pub two_qubit_gate_count: Option<usize>,
    /// Success probability
    pub success_probability: Option<f64>,
    /// Approximation ratio (for optimization)
    pub approximation_ratio: Option<f64>,
}

impl Default for QuantumMetrics {
    fn default() -> Self {
        Self {
            fidelity: None,
            trace_distance: None,
            entanglement_entropy: None,
            quantum_volume: None,
            gate_count: None,
            circuit_depth: None,
            two_qubit_gate_count: None,
            success_probability: None,
            approximation_ratio: None,
        }
    }
}

/// Analysis toolkit
pub struct Analysis {
    data: HashMap<String, Vec<f64>>,
}

impl Analysis {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Add data series
    pub fn add_series(&mut self, name: &str, values: Vec<f64>) {
        self.data.insert(name.to_string(), values);
    }

    /// Create from experiment result
    pub fn from_experiment(result: &ExperimentResult, field: &str) -> Self {
        let mut analysis = Self::new();

        let values: Vec<f64> = result.runs.iter()
            .filter_map(|run| run.data.get(field).and_then(|v| v.as_f64()))
            .collect();

        analysis.add_series(field, values);
        analysis
    }

    /// Get summary for a series
    pub fn summary(&self, name: &str) -> Option<StatisticalSummary> {
        self.data.get(name).and_then(|d| StatisticalSummary::from_data(d))
    }

    /// Compute correlation between two series
    pub fn correlation(&self, name1: &str, name2: &str) -> Option<f64> {
        let data1 = self.data.get(name1)?;
        let data2 = self.data.get(name2)?;

        if data1.len() != data2.len() || data1.is_empty() {
            return None;
        }

        let n = data1.len() as f64;
        let mean1 = data1.iter().sum::<f64>() / n;
        let mean2 = data2.iter().sum::<f64>() / n;

        let mut cov = 0.0;
        let mut var1 = 0.0;
        let mut var2 = 0.0;

        for (x, y) in data1.iter().zip(data2.iter()) {
            let dx = x - mean1;
            let dy = y - mean2;
            cov += dx * dy;
            var1 += dx * dx;
            var2 += dy * dy;
        }

        let denom = (var1 * var2).sqrt();
        if denom < 1e-10 {
            None
        } else {
            Some(cov / denom)
        }
    }

    /// Linear regression
    pub fn linear_regression(&self, x_name: &str, y_name: &str) -> Option<LinearFit> {
        let x_data = self.data.get(x_name)?;
        let y_data = self.data.get(y_name)?;

        if x_data.len() != y_data.len() || x_data.len() < 2 {
            return None;
        }

        let n = x_data.len() as f64;
        let sum_x: f64 = x_data.iter().sum();
        let sum_y: f64 = y_data.iter().sum();
        let sum_xy: f64 = x_data.iter().zip(y_data.iter()).map(|(x, y)| x * y).sum();
        let sum_x2: f64 = x_data.iter().map(|x| x * x).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        // R-squared
        let mean_y = sum_y / n;
        let ss_tot: f64 = y_data.iter().map(|y| (y - mean_y).powi(2)).sum();
        let ss_res: f64 = x_data.iter().zip(y_data.iter())
            .map(|(x, y)| (y - (slope * x + intercept)).powi(2))
            .sum();
        let r_squared = 1.0 - ss_res / ss_tot;

        Some(LinearFit {
            slope,
            intercept,
            r_squared,
        })
    }

    /// Perform t-test between two series
    pub fn t_test(&self, name1: &str, name2: &str) -> Option<TTestResult> {
        let data1 = self.data.get(name1)?;
        let data2 = self.data.get(name2)?;

        let n1 = data1.len() as f64;
        let n2 = data2.len() as f64;

        if n1 < 2.0 || n2 < 2.0 {
            return None;
        }

        let mean1 = data1.iter().sum::<f64>() / n1;
        let mean2 = data2.iter().sum::<f64>() / n2;

        let var1 = data1.iter().map(|x| (x - mean1).powi(2)).sum::<f64>() / (n1 - 1.0);
        let var2 = data2.iter().map(|x| (x - mean2).powi(2)).sum::<f64>() / (n2 - 1.0);

        let se = (var1 / n1 + var2 / n2).sqrt();
        let t_statistic = (mean1 - mean2) / se;

        // Welch's degrees of freedom
        let df = (var1 / n1 + var2 / n2).powi(2) /
            ((var1 / n1).powi(2) / (n1 - 1.0) + (var2 / n2).powi(2) / (n2 - 1.0));

        // Approximate p-value (using normal distribution for large samples)
        let p_value = 2.0 * (1.0 - normal_cdf(t_statistic.abs()));

        Some(TTestResult {
            t_statistic,
            degrees_of_freedom: df,
            p_value,
            mean_difference: mean1 - mean2,
        })
    }

    /// Print summary of all series
    pub fn print_summary(&self) {
        println!("Analysis Summary");
        println!("================");

        for (name, _) in &self.data {
            if let Some(summary) = self.summary(name) {
                println!("\n{}:", name);
                println!("  {}", summary.to_string_detailed());
            }
        }
    }
}

impl Default for Analysis {
    fn default() -> Self {
        Self::new()
    }
}

/// Linear regression result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearFit {
    pub slope: f64,
    pub intercept: f64,
    pub r_squared: f64,
}

impl LinearFit {
    pub fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}

/// T-test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTestResult {
    pub t_statistic: f64,
    pub degrees_of_freedom: f64,
    pub p_value: f64,
    pub mean_difference: f64,
}

impl TTestResult {
    pub fn is_significant(&self, alpha: f64) -> bool {
        self.p_value < alpha
    }
}

/// Approximate normal CDF
fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Error function approximation
fn erf(x: f64) -> f64 {
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

/// Fidelity analysis for quantum states
pub struct FidelityAnalysis;

impl FidelityAnalysis {
    /// Estimate fidelity from measurement counts
    pub fn from_counts(
        counts: &HashMap<String, usize>,
        target_state: &str,
        total_shots: usize,
    ) -> f64 {
        let target_count = counts.get(target_state).copied().unwrap_or(0);
        target_count as f64 / total_shots as f64
    }

    /// Estimate fidelity from multiple state preparations
    pub fn average_fidelity(fidelities: &[f64]) -> StatisticalSummary {
        StatisticalSummary::from_data(fidelities).unwrap_or(StatisticalSummary {
            count: 0,
            mean: 0.0,
            std: 0.0,
            min: 0.0,
            max: 0.0,
            median: 0.0,
            q1: 0.0,
            q3: 0.0,
            variance: 0.0,
            sem: 0.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistical_summary() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let summary = StatisticalSummary::from_data(&data).unwrap();

        assert_eq!(summary.count, 5);
        assert!((summary.mean - 3.0).abs() < 1e-10);
        assert_eq!(summary.min, 1.0);
        assert_eq!(summary.max, 5.0);
    }

    #[test]
    fn test_correlation() {
        let mut analysis = Analysis::new();
        analysis.add_series("x", vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        analysis.add_series("y", vec![2.0, 4.0, 6.0, 8.0, 10.0]);

        let corr = analysis.correlation("x", "y").unwrap();
        assert!((corr - 1.0).abs() < 1e-10); // Perfect correlation
    }

    #[test]
    fn test_linear_regression() {
        let mut analysis = Analysis::new();
        analysis.add_series("x", vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        analysis.add_series("y", vec![3.0, 5.0, 7.0, 9.0, 11.0]); // y = 2x + 1

        let fit = analysis.linear_regression("x", "y").unwrap();
        assert!((fit.slope - 2.0).abs() < 1e-10);
        assert!((fit.intercept - 1.0).abs() < 1e-10);
        assert!((fit.r_squared - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_t_test() {
        let mut analysis = Analysis::new();
        analysis.add_series("a", vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        analysis.add_series("b", vec![10.0, 11.0, 12.0, 13.0, 14.0]);

        let result = analysis.t_test("a", "b").unwrap();
        assert!(result.is_significant(0.05)); // Should be significant
    }
}
