//! Meta-Cognition Layer for introspective self-observation.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Meta-cognition configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaCognitionConfig {
    pub frequency_cycles: usize,
    pub lookback_cycles: usize,
    pub min_confidence: f64,
}

impl Default for MetaCognitionConfig {
    fn default() -> Self {
        Self {
            frequency_cycles: 500,
            lookback_cycles: 500,
            min_confidence: 0.9,
        }
    }
}

/// Self-reflection report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfReflectionReport {
    pub timestamp: DateTime<Utc>,
    pub cycle: usize,
    pub dominant_pattern: String,
    pub confidence: f64,
    pub meta_insights: Vec<String>,
    pub adaptation_suggestion: String,
}

/// Coherence pattern detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherencePattern {
    pub id: Uuid,
    pub name: String,
    pub confidence: f64,
    pub stability_correlation: f64,
}

/// Meta-cognition layer
pub struct MetaCognitionLayer {
    config: MetaCognitionConfig,
    patterns: Vec<CoherencePattern>,
    reports: Vec<SelfReflectionReport>,
    current_cycle: usize,
    last_introspection: usize,
}

impl MetaCognitionLayer {
    /// Create new meta-cognition layer
    pub fn new(config: MetaCognitionConfig) -> Self {
        Self {
            config,
            patterns: Vec::new(),
            reports: Vec::new(),
            current_cycle: 0,
            last_introspection: 0,
        }
    }

    /// Check if introspection should run
    pub fn should_run(&self) -> bool {
        self.current_cycle - self.last_introspection >= self.config.frequency_cycles
    }

    /// Run introspection
    pub fn run_introspection(&mut self, telemetry: Vec<(f64, f64, f64)>) -> SelfReflectionReport {
        // Analyze patterns
        let pattern = self.analyze_patterns(&telemetry);
        self.patterns.push(pattern.clone());

        // Generate insights
        let mut insights = Vec::new();
        if pattern.confidence > 0.9 {
            insights.push("High pattern confidence indicates strong system coherence".to_string());
        }

        let suggestion = if pattern.confidence < 0.7 {
            "Consider increasing mutation rate".to_string()
        } else {
            "System evolving optimally".to_string()
        };

        let report = SelfReflectionReport {
            timestamp: Utc::now(),
            cycle: self.current_cycle,
            dominant_pattern: pattern.name,
            confidence: pattern.confidence,
            meta_insights: insights,
            adaptation_suggestion: suggestion,
        };

        self.reports.push(report.clone());
        self.last_introspection = self.current_cycle;

        report
    }

    fn analyze_patterns(&self, telemetry: &[(f64, f64, f64)]) -> CoherencePattern {
        let mean_stability: f64 = telemetry.iter().map(|(_, _, s)| s).sum::<f64>()
            / telemetry.len().max(1) as f64;

        CoherencePattern {
            id: Uuid::new_v4(),
            name: "Stability Pattern".to_string(),
            confidence: mean_stability,
            stability_correlation: mean_stability * 0.9,
        }
    }

    /// Advance cycle
    pub fn tick(&mut self) {
        self.current_cycle += 1;
    }

    /// Get reports
    pub fn reports(&self) -> &[SelfReflectionReport] {
        &self.reports
    }
}

impl Default for MetaCognitionLayer {
    fn default() -> Self {
        Self::new(MetaCognitionConfig::default())
    }
}
