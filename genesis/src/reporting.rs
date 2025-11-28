//! # Genesis Reporting Module
//!
//! Provides comprehensive reporting and export functionality for Genesis mining results.
//!
//! ## Supported Formats
//!
//! - **Markdown**: Human-readable Genesis_Report.md
//! - **JSON**: Machine-readable structured data
//! - **CSV**: Spreadsheet-compatible tabular data
//!
//! ## Usage
//!
//! ```ignore
//! use qops_genesis::reporting::{GenesisReporter, ReportFormat};
//!
//! let reporter = GenesisReporter::new(&mining_result);
//! let report = reporter.generate(ReportFormat::Markdown)?;
//! ```

use crate::holistic_mining::{HolisticMiningResult, StageLogEntry, StageMetrics};
use crate::family::FamilyCharacteristics;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Report format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    /// Human-readable Markdown
    Markdown,
    /// Machine-readable JSON
    Json,
    /// Spreadsheet-compatible CSV
    Csv,
}

/// Report configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    /// Include stage logs
    pub include_stage_logs: bool,
    /// Include family details
    pub include_family_details: bool,
    /// Include pipeline summary
    pub include_pipeline_summary: bool,
    /// Include Monolith structure
    pub include_monolith: bool,
    /// Include timestamp
    pub include_timestamp: bool,
    /// Report title
    pub title: String,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            include_stage_logs: true,
            include_family_details: true,
            include_pipeline_summary: true,
            include_monolith: true,
            include_timestamp: true,
            title: "Genesis Holistic Mining Report".to_string(),
        }
    }
}

/// Finalized family summary for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyReportEntry {
    pub name: String,
    pub member_count: usize,
    pub avg_resonance: f64,
    pub is_high_quality: bool,
    pub is_stable: bool,
    pub is_efficient: bool,
}

/// Pipeline stage summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStageSummary {
    pub stage_name: String,
    pub input_count: usize,
    pub output_count: usize,
    pub avg_resonance: f64,
    pub duration_ms: u64,
    pub reduction_rate: f64,
}

/// Monolith summary for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonolithSummary {
    pub coherence: f64,
    pub family_count: usize,
    pub finalized: bool,
}

/// Complete report data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisReport {
    pub title: String,
    pub timestamp: String,
    pub duration_ms: u64,
    pub best_resonance: f64,
    pub matrix_outputs: usize,
    pub pipeline_summary: Vec<PipelineStageSummary>,
    pub families: Vec<FamilyReportEntry>,
    pub monolith: Option<MonolithSummary>,
    pub stage_logs: Vec<StageLogSummary>,
    pub statistics: ReportStatistics,
}

/// Stage log summary for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageLogSummary {
    pub stage: String,
    pub message: String,
    pub timestamp: String,
}

/// Statistical summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportStatistics {
    pub total_candidates_discovered: usize,
    pub total_families_finalized: usize,
    pub pipeline_efficiency: f64,
    pub avg_family_resonance: f64,
    pub high_quality_family_count: usize,
}

/// Genesis report generator
pub struct GenesisReporter {
    result: HolisticMiningResult,
    config: ReportConfig,
}

impl GenesisReporter {
    /// Create a new reporter from mining results
    pub fn new(result: HolisticMiningResult) -> Self {
        Self {
            result,
            config: ReportConfig::default(),
        }
    }

    /// Set custom configuration
    pub fn with_config(mut self, config: ReportConfig) -> Self {
        self.config = config;
        self
    }

    /// Generate report data
    pub fn build_report(&self) -> GenesisReport {
        let timestamp = chrono::Utc::now().to_rfc3339();

        // Build pipeline summary
        let mut pipeline_summary = vec![];

        // Discovery stage
        pipeline_summary.push(PipelineStageSummary {
            stage_name: "Discovery".to_string(),
            input_count: 0,
            output_count: self.result.candidates_discovered,
            avg_resonance: 0.0,
            duration_ms: 0,
            reduction_rate: 0.0,
        });

        // Kosmokrator stage
        let kos_reduction = if self.result.candidates_discovered > 0 {
            1.0 - (self.result.candidates_after_kosmokrator as f64 / self.result.candidates_discovered as f64)
        } else {
            0.0
        };
        pipeline_summary.push(PipelineStageSummary {
            stage_name: "Kosmokrator".to_string(),
            input_count: self.result.candidates_discovered,
            output_count: self.result.candidates_after_kosmokrator,
            avg_resonance: 0.0,
            duration_ms: 0,
            reduction_rate: kos_reduction,
        });

        // Chronokrator stage
        let chrono_reduction = if self.result.candidates_after_kosmokrator > 0 {
            1.0 - (self.result.candidates_after_chronokrator as f64 / self.result.candidates_after_kosmokrator as f64)
        } else {
            0.0
        };
        pipeline_summary.push(PipelineStageSummary {
            stage_name: "Chronokrator".to_string(),
            input_count: self.result.candidates_after_kosmokrator,
            output_count: self.result.candidates_after_chronokrator,
            avg_resonance: 0.0,
            duration_ms: 0,
            reduction_rate: chrono_reduction,
        });

        // Pfauenthron stage
        pipeline_summary.push(PipelineStageSummary {
            stage_name: "Pfauenthron".to_string(),
            input_count: self.result.candidates_after_chronokrator,
            output_count: self.result.finalized_families.len(),
            avg_resonance: self.result.best_resonance,
            duration_ms: 0,
            reduction_rate: 0.0,
        });

        // Build family entries
        let families: Vec<FamilyReportEntry> = self.result.finalized_families
            .iter()
            .map(|f| FamilyReportEntry {
                name: f.name.clone(),
                member_count: f.member_count,
                avg_resonance: f.avg_resonance,
                is_high_quality: f.characteristics.is_high_quality,
                is_stable: f.characteristics.is_stable,
                is_efficient: f.characteristics.is_efficient,
            })
            .collect();

        // Build monolith summary
        let monolith = self.result.monolith.as_ref().map(|m| MonolithSummary {
            coherence: m.coherence,
            family_count: m.family_count,
            finalized: m.finalized,
        });

        // Build stage logs
        let stage_logs: Vec<StageLogSummary> = self.result.stage_logs
            .iter()
            .map(|log| StageLogSummary {
                stage: format!("{:?}", log.stage),
                message: log.message.clone(),
                timestamp: log.timestamp.to_rfc3339(),
            })
            .collect();

        // Calculate statistics
        let avg_family_resonance = if !families.is_empty() {
            families.iter().map(|f| f.avg_resonance).sum::<f64>() / families.len() as f64
        } else {
            0.0
        };

        let high_quality_count = families.iter()
            .filter(|f| f.is_high_quality)
            .count();

        let pipeline_efficiency = if self.result.candidates_discovered > 0 {
            self.result.finalized_families.len() as f64 / self.result.candidates_discovered as f64
        } else {
            0.0
        };

        let statistics = ReportStatistics {
            total_candidates_discovered: self.result.candidates_discovered,
            total_families_finalized: self.result.finalized_families.len(),
            pipeline_efficiency,
            avg_family_resonance,
            high_quality_family_count: high_quality_count,
        };

        GenesisReport {
            title: self.config.title.clone(),
            timestamp,
            duration_ms: self.result.duration_ms,
            best_resonance: self.result.best_resonance,
            matrix_outputs: self.result.matrix_outputs,
            pipeline_summary,
            families,
            monolith,
            stage_logs,
            statistics,
        }
    }

    /// Generate report in specified format
    pub fn generate(&self, format: ReportFormat) -> Result<String, String> {
        let report = self.build_report();

        match format {
            ReportFormat::Markdown => Ok(self.to_markdown(&report)),
            ReportFormat::Json => serde_json::to_string_pretty(&report)
                .map_err(|e| e.to_string()),
            ReportFormat::Csv => Ok(self.to_csv(&report)),
        }
    }

    /// Generate Markdown report
    fn to_markdown(&self, report: &GenesisReport) -> String {
        let mut md = String::new();

        // Header
        md.push_str(&format!("# {}\n\n", report.title));

        if self.config.include_timestamp {
            md.push_str(&format!("**Generated**: {}\n\n", report.timestamp));
        }

        // Summary section
        md.push_str("## Summary\n\n");
        md.push_str(&format!("- **Duration**: {} ms\n", report.duration_ms));
        md.push_str(&format!("- **Best Resonance**: {:.4}\n", report.best_resonance));
        md.push_str(&format!("- **Matrix Outputs**: {}\n", report.matrix_outputs));
        md.push_str(&format!("- **Finalized Families**: {}\n\n", report.families.len()));

        // Pipeline summary
        if self.config.include_pipeline_summary {
            md.push_str("## Pipeline Summary\n\n");
            md.push_str("| Stage | Input | Output | Reduction |\n");
            md.push_str("|-------|-------|--------|----------|\n");

            for stage in &report.pipeline_summary {
                md.push_str(&format!(
                    "| {} | {} | {} | {:.1}% |\n",
                    stage.stage_name,
                    stage.input_count,
                    stage.output_count,
                    stage.reduction_rate * 100.0
                ));
            }
            md.push('\n');
        }

        // Families section
        if self.config.include_family_details && !report.families.is_empty() {
            md.push_str("## Finalized Operator Families\n\n");
            md.push_str("| Name | Members | Resonance | Quality | Stable | Efficient |\n");
            md.push_str("|------|---------|-----------|---------|--------|----------|\n");

            for family in &report.families {
                md.push_str(&format!(
                    "| {} | {} | {:.4} | {} | {} | {} |\n",
                    family.name,
                    family.member_count,
                    family.avg_resonance,
                    if family.is_high_quality { "Yes" } else { "No" },
                    if family.is_stable { "Yes" } else { "No" },
                    if family.is_efficient { "Yes" } else { "No" },
                ));
            }
            md.push('\n');
        }

        // Monolith section
        if self.config.include_monolith {
            if let Some(mono) = &report.monolith {
                md.push_str("## Monolith Structure\n\n");
                md.push_str(&format!("- **Coherence**: {:.4}\n", mono.coherence));
                md.push_str(&format!("- **Families**: {}\n", mono.family_count));
                md.push_str(&format!("- **Finalized**: {}\n\n", if mono.finalized { "Yes" } else { "No" }));
            }
        }

        // Statistics section
        md.push_str("## Statistics\n\n");
        md.push_str(&format!("- **Total Candidates Discovered**: {}\n", report.statistics.total_candidates_discovered));
        md.push_str(&format!("- **Total Families Finalized**: {}\n", report.statistics.total_families_finalized));
        md.push_str(&format!("- **Pipeline Efficiency**: {:.2}%\n", report.statistics.pipeline_efficiency * 100.0));
        md.push_str(&format!("- **Average Family Resonance**: {:.4}\n", report.statistics.avg_family_resonance));
        md.push_str(&format!("- **High Quality Families**: {}\n\n", report.statistics.high_quality_family_count));

        // Stage logs
        if self.config.include_stage_logs && !report.stage_logs.is_empty() {
            md.push_str("## Stage Logs\n\n");
            for log in &report.stage_logs {
                md.push_str(&format!("- **[{}]** {}: {}\n", log.timestamp, log.stage, log.message));
            }
            md.push('\n');
        }

        // Footer
        md.push_str("---\n\n");
        md.push_str("*Generated by QOPS Genesis Pipeline*\n");

        md
    }

    /// Generate CSV report (families only)
    fn to_csv(&self, report: &GenesisReport) -> String {
        let mut csv = String::new();

        // Header
        csv.push_str("name,member_count,avg_resonance,is_high_quality,is_stable,is_efficient\n");

        // Data rows
        for family in &report.families {
            csv.push_str(&format!(
                "{},{},{:.4},{},{},{}\n",
                family.name,
                family.member_count,
                family.avg_resonance,
                family.is_high_quality,
                family.is_stable,
                family.is_efficient,
            ));
        }

        csv
    }
}

/// Export holistic mining result to file
pub fn export_to_file(
    result: &HolisticMiningResult,
    path: &str,
    format: ReportFormat,
) -> Result<(), String> {
    let reporter = GenesisReporter::new(result.clone());
    let content = reporter.generate(format)?;

    std::fs::write(path, content).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_format() {
        assert_eq!(ReportFormat::Markdown, ReportFormat::Markdown);
        assert_ne!(ReportFormat::Json, ReportFormat::Csv);
    }

    #[test]
    fn test_default_config() {
        let config = ReportConfig::default();
        assert!(config.include_stage_logs);
        assert!(config.include_family_details);
    }
}
