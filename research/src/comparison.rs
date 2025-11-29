//! Algorithm comparison framework
//!
//! Tools for comparing quantum algorithms and configurations.

use crate::{BenchmarkResult, StatisticalSummary, Analysis};
use serde::{Deserialize, Serialize};

/// Comparison result for two algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    /// Name of first algorithm
    pub algorithm_a: String,
    /// Name of second algorithm
    pub algorithm_b: String,
    /// Metric being compared
    pub metric: String,
    /// Statistics for algorithm A
    pub stats_a: StatisticalSummary,
    /// Statistics for algorithm B
    pub stats_b: StatisticalSummary,
    /// Relative improvement (B over A)
    pub relative_improvement: f64,
    /// Speedup factor (for time comparisons)
    pub speedup: Option<f64>,
    /// P-value from statistical test
    pub p_value: Option<f64>,
    /// Whether difference is statistically significant
    pub significant: bool,
}

impl ComparisonResult {
    /// Format as human-readable summary
    pub fn summary(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Comparison: {} vs {}\n", self.algorithm_a, self.algorithm_b));
        s.push_str(&format!("Metric: {}\n", self.metric));
        s.push_str(&format!("{}: {:.4} ± {:.4}\n", self.algorithm_a, self.stats_a.mean, self.stats_a.std));
        s.push_str(&format!("{}: {:.4} ± {:.4}\n", self.algorithm_b, self.stats_b.mean, self.stats_b.std));
        s.push_str(&format!("Relative improvement: {:.2}%\n", self.relative_improvement * 100.0));
        if let Some(speedup) = self.speedup {
            s.push_str(&format!("Speedup: {:.2}x\n", speedup));
        }
        if let Some(p) = self.p_value {
            s.push_str(&format!("P-value: {:.4}\n", p));
        }
        s.push_str(&format!("Significant: {}\n", if self.significant { "Yes" } else { "No" }));
        s
    }
}

/// Algorithm comparison for a single metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmComparison {
    /// Algorithm name
    pub name: String,
    /// Mean value
    pub mean: f64,
    /// Standard deviation
    pub std: f64,
    /// Number of samples
    pub n: usize,
    /// Rank (1 = best)
    pub rank: usize,
}

/// Multi-algorithm comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiComparisonResult {
    /// Metric name
    pub metric: String,
    /// Whether lower is better
    pub lower_is_better: bool,
    /// Comparison for each algorithm
    pub algorithms: Vec<AlgorithmComparison>,
    /// Best algorithm name
    pub best: String,
}

impl MultiComparisonResult {
    /// Generate comparison table
    pub fn to_table(&self) -> String {
        let mut table = String::new();
        table.push_str(&format!("Metric: {} ({})\n", self.metric,
            if self.lower_is_better { "lower is better" } else { "higher is better" }));
        table.push_str(&format!("{:<30} {:>15} {:>15} {:>10}\n",
            "Algorithm", "Mean", "Std", "Rank"));
        table.push_str(&"-".repeat(70));
        table.push('\n');

        for alg in &self.algorithms {
            table.push_str(&format!("{:<30} {:>15.4} {:>15.4} {:>10}\n",
                alg.name, alg.mean, alg.std, alg.rank));
        }

        table.push_str(&format!("\nBest: {}\n", self.best));
        table
    }
}

/// Comparison toolkit
pub struct Comparison;

impl Comparison {
    /// Compare two sets of measurements
    pub fn compare(
        name_a: &str,
        data_a: &[f64],
        name_b: &str,
        data_b: &[f64],
        metric: &str,
    ) -> Option<ComparisonResult> {
        let stats_a = StatisticalSummary::from_data(data_a)?;
        let stats_b = StatisticalSummary::from_data(data_b)?;

        let relative_improvement = if stats_a.mean != 0.0 {
            (stats_a.mean - stats_b.mean) / stats_a.mean
        } else {
            0.0
        };

        let speedup = if stats_b.mean != 0.0 {
            Some(stats_a.mean / stats_b.mean)
        } else {
            None
        };

        // T-test
        let mut analysis = Analysis::new();
        analysis.add_series("a", data_a.to_vec());
        analysis.add_series("b", data_b.to_vec());

        let (p_value, significant) = analysis.t_test("a", "b")
            .map(|t| (Some(t.p_value), t.is_significant(0.05)))
            .unwrap_or((None, false));

        Some(ComparisonResult {
            algorithm_a: name_a.to_string(),
            algorithm_b: name_b.to_string(),
            metric: metric.to_string(),
            stats_a,
            stats_b,
            relative_improvement,
            speedup,
            p_value,
            significant,
        })
    }

    /// Compare multiple algorithms
    pub fn compare_multiple(
        algorithms: Vec<(&str, &[f64])>,
        metric: &str,
        lower_is_better: bool,
    ) -> MultiComparisonResult {
        let mut comparisons: Vec<AlgorithmComparison> = algorithms.iter()
            .filter_map(|(name, data)| {
                let summary = StatisticalSummary::from_data(data)?;
                Some(AlgorithmComparison {
                    name: name.to_string(),
                    mean: summary.mean,
                    std: summary.std,
                    n: summary.count,
                    rank: 0,
                })
            })
            .collect();

        // Rank algorithms
        comparisons.sort_by(|a, b| {
            if lower_is_better {
                a.mean.partial_cmp(&b.mean).unwrap()
            } else {
                b.mean.partial_cmp(&a.mean).unwrap()
            }
        });

        for (i, alg) in comparisons.iter_mut().enumerate() {
            alg.rank = i + 1;
        }

        let best = comparisons.first()
            .map(|a| a.name.clone())
            .unwrap_or_default();

        MultiComparisonResult {
            metric: metric.to_string(),
            lower_is_better,
            algorithms: comparisons,
            best,
        }
    }

    /// Compare benchmark results
    pub fn compare_benchmarks(
        result_a: &BenchmarkResult,
        result_b: &BenchmarkResult,
    ) -> Vec<ComparisonResult> {
        let mut comparisons = Vec::new();

        // Time comparison
        let times_a: Vec<f64> = result_a.measurements.iter()
            .map(|m| m.duration.as_secs_f64() * 1000.0)
            .collect();
        let times_b: Vec<f64> = result_b.measurements.iter()
            .map(|m| m.duration.as_secs_f64() * 1000.0)
            .collect();

        if let Some(comp) = Self::compare(
            &result_a.config.name,
            &times_a,
            &result_b.config.name,
            &times_b,
            "execution_time_ms"
        ) {
            comparisons.push(comp);
        }

        // Success probability comparison
        let probs_a: Vec<f64> = result_a.measurements.iter()
            .filter_map(|m| m.success_probability)
            .collect();
        let probs_b: Vec<f64> = result_b.measurements.iter()
            .filter_map(|m| m.success_probability)
            .collect();

        if !probs_a.is_empty() && !probs_b.is_empty() {
            if let Some(comp) = Self::compare(
                &result_a.config.name,
                &probs_a,
                &result_b.config.name,
                &probs_b,
                "success_probability"
            ) {
                comparisons.push(comp);
            }
        }

        // Gate count comparison
        let gates_a: Vec<f64> = result_a.measurements.iter()
            .filter_map(|m| m.gate_count.map(|c| c as f64))
            .collect();
        let gates_b: Vec<f64> = result_b.measurements.iter()
            .filter_map(|m| m.gate_count.map(|c| c as f64))
            .collect();

        if !gates_a.is_empty() && !gates_b.is_empty() {
            if let Some(comp) = Self::compare(
                &result_a.config.name,
                &gates_a,
                &result_b.config.name,
                &gates_b,
                "gate_count"
            ) {
                comparisons.push(comp);
            }
        }

        comparisons
    }
}

/// Scaling analysis
pub struct ScalingAnalysis;

impl ScalingAnalysis {
    /// Fit scaling function: y = a * x^b
    pub fn fit_power_law(x: &[f64], y: &[f64]) -> Option<(f64, f64, f64)> {
        if x.len() != y.len() || x.len() < 2 {
            return None;
        }

        // Transform to log-log space
        let log_x: Vec<f64> = x.iter().map(|&v| v.ln()).collect();
        let log_y: Vec<f64> = y.iter().map(|&v| v.ln()).collect();

        let mut analysis = Analysis::new();
        analysis.add_series("log_x", log_x);
        analysis.add_series("log_y", log_y);

        let fit = analysis.linear_regression("log_x", "log_y")?;

        // y = a * x^b => log(y) = log(a) + b*log(x)
        let b = fit.slope;
        let a = fit.intercept.exp();

        Some((a, b, fit.r_squared))
    }

    /// Fit exponential: y = a * exp(b * x)
    pub fn fit_exponential(x: &[f64], y: &[f64]) -> Option<(f64, f64, f64)> {
        if x.len() != y.len() || x.len() < 2 {
            return None;
        }

        // Transform to semi-log space
        let log_y: Vec<f64> = y.iter().map(|&v| v.ln()).collect();

        let mut analysis = Analysis::new();
        analysis.add_series("x", x.to_vec());
        analysis.add_series("log_y", log_y);

        let fit = analysis.linear_regression("x", "log_y")?;

        // y = a * exp(b*x) => log(y) = log(a) + b*x
        let b = fit.slope;
        let a = fit.intercept.exp();

        Some((a, b, fit.r_squared))
    }

    /// Determine scaling type
    pub fn classify_scaling(x: &[f64], y: &[f64]) -> ScalingType {
        let power = Self::fit_power_law(x, y);
        let exp = Self::fit_exponential(x, y);

        match (power, exp) {
            (Some((_, b, r2_pow)), Some((_, _, r2_exp))) => {
                if r2_exp > r2_pow && r2_exp > 0.95 {
                    ScalingType::Exponential
                } else if b.abs() < 0.5 && r2_pow > 0.9 {
                    ScalingType::Constant
                } else if (b - 1.0).abs() < 0.2 && r2_pow > 0.9 {
                    ScalingType::Linear
                } else if (b - 2.0).abs() < 0.3 && r2_pow > 0.9 {
                    ScalingType::Quadratic
                } else if r2_pow > 0.9 {
                    ScalingType::Polynomial(b)
                } else {
                    ScalingType::Unknown
                }
            }
            (Some((_, b, r2)), None) if r2 > 0.9 => {
                if b.abs() < 0.5 {
                    ScalingType::Constant
                } else {
                    ScalingType::Polynomial(b)
                }
            }
            (None, Some((_, _, r2))) if r2 > 0.9 => ScalingType::Exponential,
            _ => ScalingType::Unknown,
        }
    }
}

/// Type of scaling behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingType {
    Constant,
    Linear,
    Quadratic,
    Polynomial(f64),
    Exponential,
    Unknown,
}

impl ScalingType {
    pub fn description(&self) -> String {
        match self {
            ScalingType::Constant => "O(1)".to_string(),
            ScalingType::Linear => "O(n)".to_string(),
            ScalingType::Quadratic => "O(n²)".to_string(),
            ScalingType::Polynomial(p) => format!("O(n^{:.2})", p),
            ScalingType::Exponential => "O(exp(n))".to_string(),
            ScalingType::Unknown => "Unknown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        let data_a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let data_b = vec![0.5, 1.0, 1.5, 2.0, 2.5];

        let result = Comparison::compare("A", &data_a, "B", &data_b, "test").unwrap();

        assert!(result.speedup.is_some());
        assert!(result.speedup.unwrap() > 1.0); // A is slower
    }

    #[test]
    fn test_multi_compare() {
        let alg_a = vec![1.0, 1.1, 0.9];
        let alg_b = vec![2.0, 2.1, 1.9];
        let alg_c = vec![0.5, 0.6, 0.4];

        let result = Comparison::compare_multiple(
            vec![("A", &alg_a), ("B", &alg_b), ("C", &alg_c)],
            "time",
            true
        );

        assert_eq!(result.best, "C");
        assert_eq!(result.algorithms[0].rank, 1);
    }

    #[test]
    fn test_power_law_fit() {
        // y = 2 * x^2
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y: Vec<f64> = x.iter().map(|&v| 2.0 * v * v).collect();

        let (a, b, r2) = ScalingAnalysis::fit_power_law(&x, &y).unwrap();

        assert!((a - 2.0).abs() < 0.1);
        assert!((b - 2.0).abs() < 0.1);
        assert!(r2 > 0.99);
    }
}
