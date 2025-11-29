//! Report generation for research results
//!
//! Generate comprehensive reports in various formats.

use crate::{
    BenchmarkSuite, ExperimentResult,
    StatisticalSummary, ComparisonResult, PlotData
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Report format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportFormat {
    Markdown,
    Html,
    Latex,
    PlainText,
}

/// Report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportSection {
    /// Title and metadata
    Header {
        title: String,
        author: Option<String>,
        date: DateTime<Utc>,
        abstract_text: Option<String>,
    },
    /// Free-form text
    Text(String),
    /// Code block
    Code { language: String, content: String },
    /// Data table
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
        caption: Option<String>,
    },
    /// Plot reference
    Figure {
        data: PlotData,
        caption: String,
    },
    /// Statistical summary
    Statistics {
        name: String,
        summary: StatisticalSummary,
    },
    /// Comparison results
    Comparison(ComparisonResult),
    /// Section heading
    Heading { level: usize, text: String },
    /// Bullet list
    List(Vec<String>),
    /// Key-value pairs
    KeyValues(Vec<(String, String)>),
}

/// Research report builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub sections: Vec<ReportSection>,
    pub format: ReportFormat,
    pub metadata: HashMap<String, String>,
}

impl Report {
    /// Create new empty report
    pub fn new(format: ReportFormat) -> Self {
        Self {
            sections: Vec::new(),
            format,
            metadata: HashMap::new(),
        }
    }

    /// Create markdown report
    pub fn markdown() -> Self {
        Self::new(ReportFormat::Markdown)
    }

    /// Add header
    pub fn header(mut self, title: &str) -> Self {
        self.sections.push(ReportSection::Header {
            title: title.to_string(),
            author: None,
            date: Utc::now(),
            abstract_text: None,
        });
        self
    }

    /// Add header with details
    pub fn header_full(mut self, title: &str, author: &str, abstract_text: &str) -> Self {
        self.sections.push(ReportSection::Header {
            title: title.to_string(),
            author: Some(author.to_string()),
            date: Utc::now(),
            abstract_text: Some(abstract_text.to_string()),
        });
        self
    }

    /// Add section heading
    pub fn heading(mut self, level: usize, text: &str) -> Self {
        self.sections.push(ReportSection::Heading {
            level,
            text: text.to_string(),
        });
        self
    }

    /// Add text paragraph
    pub fn text(mut self, text: &str) -> Self {
        self.sections.push(ReportSection::Text(text.to_string()));
        self
    }

    /// Add code block
    pub fn code(mut self, language: &str, content: &str) -> Self {
        self.sections.push(ReportSection::Code {
            language: language.to_string(),
            content: content.to_string(),
        });
        self
    }

    /// Add table
    pub fn table(mut self, headers: Vec<&str>, rows: Vec<Vec<String>>, caption: Option<&str>) -> Self {
        self.sections.push(ReportSection::Table {
            headers: headers.into_iter().map(|s| s.to_string()).collect(),
            rows,
            caption: caption.map(|s| s.to_string()),
        });
        self
    }

    /// Add figure
    pub fn figure(mut self, data: PlotData, caption: &str) -> Self {
        self.sections.push(ReportSection::Figure {
            data,
            caption: caption.to_string(),
        });
        self
    }

    /// Add statistics section
    pub fn statistics(mut self, name: &str, summary: StatisticalSummary) -> Self {
        self.sections.push(ReportSection::Statistics {
            name: name.to_string(),
            summary,
        });
        self
    }

    /// Add comparison
    pub fn comparison(mut self, comp: ComparisonResult) -> Self {
        self.sections.push(ReportSection::Comparison(comp));
        self
    }

    /// Add list
    pub fn list(mut self, items: Vec<&str>) -> Self {
        self.sections.push(ReportSection::List(
            items.into_iter().map(|s| s.to_string()).collect()
        ));
        self
    }

    /// Add key-value section
    pub fn key_values(mut self, pairs: Vec<(&str, &str)>) -> Self {
        self.sections.push(ReportSection::KeyValues(
            pairs.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
        ));
        self
    }

    /// Add benchmark suite results
    pub fn add_benchmark_suite(mut self, suite: &BenchmarkSuite) -> Self {
        self = self.heading(2, &format!("Benchmark Suite: {}", suite.name));

        if !suite.description.is_empty() {
            self = self.text(&suite.description);
        }

        // Summary table
        let headers = vec!["Benchmark", "Mean (ms)", "Std (ms)", "Min (ms)", "Max (ms)"];
        let rows: Vec<Vec<String>> = suite.results.iter()
            .map(|r| vec![
                r.config.name.clone(),
                format!("{:.3}", r.mean_duration().as_secs_f64() * 1000.0),
                format!("{:.3}", r.std_duration().as_secs_f64() * 1000.0),
                format!("{:.3}", r.min_duration().as_secs_f64() * 1000.0),
                format!("{:.3}", r.max_duration().as_secs_f64() * 1000.0),
            ])
            .collect();

        self = self.table(headers, rows, Some("Benchmark Results Summary"));
        self
    }

    /// Add experiment results
    pub fn add_experiment(mut self, result: &ExperimentResult) -> Self {
        self = self.heading(2, &format!("Experiment: {}", result.config.name));
        self = self.text(&result.config.description);

        let status_str = format!("{:?}", result.status);
        let total_runs_str = result.runs.len().to_string();
        let success_rate_str = format!("{:.1}%", result.success_rate() * 100.0);

        let pairs = vec![
            ("ID", result.id.as_str()),
            ("Status", status_str.as_str()),
            ("Total Runs", total_runs_str.as_str()),
            ("Success Rate", success_rate_str.as_str()),
        ];

        self = self.key_values(pairs);
        self
    }

    /// Render to string
    pub fn render(&self) -> String {
        match self.format {
            ReportFormat::Markdown => self.render_markdown(),
            ReportFormat::Html => self.render_html(),
            ReportFormat::Latex => self.render_latex(),
            ReportFormat::PlainText => self.render_text(),
        }
    }

    fn render_markdown(&self) -> String {
        let mut out = String::new();

        for section in &self.sections {
            match section {
                ReportSection::Header { title, author, date, abstract_text } => {
                    out.push_str(&format!("# {}\n\n", title));
                    if let Some(author) = author {
                        out.push_str(&format!("**Author:** {}\n\n", author));
                    }
                    out.push_str(&format!("**Date:** {}\n\n", date.format("%Y-%m-%d")));
                    if let Some(abs) = abstract_text {
                        out.push_str(&format!("## Abstract\n\n{}\n\n", abs));
                    }
                }
                ReportSection::Heading { level, text } => {
                    out.push_str(&format!("{} {}\n\n", "#".repeat(*level), text));
                }
                ReportSection::Text(text) => {
                    out.push_str(&format!("{}\n\n", text));
                }
                ReportSection::Code { language, content } => {
                    out.push_str(&format!("```{}\n{}\n```\n\n", language, content));
                }
                ReportSection::Table { headers, rows, caption } => {
                    // Header
                    out.push_str(&format!("| {} |\n", headers.join(" | ")));
                    out.push_str(&format!("|{}|\n", headers.iter().map(|_| "---").collect::<Vec<_>>().join("|")));
                    // Rows
                    for row in rows {
                        out.push_str(&format!("| {} |\n", row.join(" | ")));
                    }
                    if let Some(cap) = caption {
                        out.push_str(&format!("\n*{}*\n", cap));
                    }
                    out.push('\n');
                }
                ReportSection::Statistics { name, summary } => {
                    out.push_str(&format!("### {}\n\n", name));
                    out.push_str(&format!("- **N:** {}\n", summary.count));
                    out.push_str(&format!("- **Mean:** {:.4}\n", summary.mean));
                    out.push_str(&format!("- **Std:** {:.4}\n", summary.std));
                    out.push_str(&format!("- **Range:** [{:.4}, {:.4}]\n", summary.min, summary.max));
                    out.push_str(&format!("- **Median:** {:.4}\n\n", summary.median));
                }
                ReportSection::Comparison(comp) => {
                    out.push_str(&format!("### Comparison: {} vs {}\n\n", comp.algorithm_a, comp.algorithm_b));
                    out.push_str(&format!("**Metric:** {}\n\n", comp.metric));
                    out.push_str(&format!("| Algorithm | Mean | Std |\n"));
                    out.push_str("|---|---|---|\n");
                    out.push_str(&format!("| {} | {:.4} | {:.4} |\n",
                        comp.algorithm_a, comp.stats_a.mean, comp.stats_a.std));
                    out.push_str(&format!("| {} | {:.4} | {:.4} |\n\n",
                        comp.algorithm_b, comp.stats_b.mean, comp.stats_b.std));
                    if let Some(speedup) = comp.speedup {
                        out.push_str(&format!("**Speedup:** {:.2}x\n\n", speedup));
                    }
                }
                ReportSection::List(items) => {
                    for item in items {
                        out.push_str(&format!("- {}\n", item));
                    }
                    out.push('\n');
                }
                ReportSection::KeyValues(pairs) => {
                    for (k, v) in pairs {
                        out.push_str(&format!("- **{}:** {}\n", k, v));
                    }
                    out.push('\n');
                }
                ReportSection::Figure { data, caption } => {
                    out.push_str(&format!("*Figure: {}*\n\n", caption));
                    out.push_str("```\n");
                    out.push_str(&data.to_json().unwrap_or_default());
                    out.push_str("\n```\n\n");
                }
            }
        }

        out
    }

    fn render_html(&self) -> String {
        let mut out = String::new();
        out.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        out.push_str("<meta charset=\"UTF-8\">\n");
        out.push_str("<style>\n");
        out.push_str("body { font-family: Arial, sans-serif; max-width: 900px; margin: 0 auto; padding: 20px; }\n");
        out.push_str("table { border-collapse: collapse; width: 100%; margin: 1em 0; }\n");
        out.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        out.push_str("th { background-color: #f4f4f4; }\n");
        out.push_str("pre { background: #f4f4f4; padding: 1em; overflow-x: auto; }\n");
        out.push_str("</style>\n</head>\n<body>\n");

        for section in &self.sections {
            match section {
                ReportSection::Header { title, author, date, abstract_text } => {
                    out.push_str(&format!("<h1>{}</h1>\n", title));
                    if let Some(author) = author {
                        out.push_str(&format!("<p><strong>Author:</strong> {}</p>\n", author));
                    }
                    out.push_str(&format!("<p><strong>Date:</strong> {}</p>\n", date.format("%Y-%m-%d")));
                    if let Some(abs) = abstract_text {
                        out.push_str(&format!("<h2>Abstract</h2>\n<p>{}</p>\n", abs));
                    }
                }
                ReportSection::Heading { level, text } => {
                    out.push_str(&format!("<h{}>{}</h{}>\n", level, text, level));
                }
                ReportSection::Text(text) => {
                    out.push_str(&format!("<p>{}</p>\n", text));
                }
                ReportSection::Table { headers, rows, caption } => {
                    out.push_str("<table>\n");
                    out.push_str("<tr>");
                    for h in headers {
                        out.push_str(&format!("<th>{}</th>", h));
                    }
                    out.push_str("</tr>\n");
                    for row in rows {
                        out.push_str("<tr>");
                        for cell in row {
                            out.push_str(&format!("<td>{}</td>", cell));
                        }
                        out.push_str("</tr>\n");
                    }
                    out.push_str("</table>\n");
                    if let Some(cap) = caption {
                        out.push_str(&format!("<p><em>{}</em></p>\n", cap));
                    }
                }
                _ => {}
            }
        }

        out.push_str("</body>\n</html>\n");
        out
    }

    fn render_latex(&self) -> String {
        let mut out = String::new();
        out.push_str("\\documentclass{article}\n");
        out.push_str("\\usepackage{booktabs}\n");
        out.push_str("\\begin{document}\n\n");

        for section in &self.sections {
            match section {
                ReportSection::Header { title, author, date, .. } => {
                    out.push_str(&format!("\\title{{{}}}\n", title));
                    if let Some(author) = author {
                        out.push_str(&format!("\\author{{{}}}\n", author));
                    }
                    out.push_str(&format!("\\date{{{}}}\n", date.format("%Y-%m-%d")));
                    out.push_str("\\maketitle\n\n");
                }
                ReportSection::Heading { level, text } => {
                    let cmd = match level {
                        1 => "section",
                        2 => "subsection",
                        _ => "subsubsection",
                    };
                    out.push_str(&format!("\\{}{{{}}}\n\n", cmd, text));
                }
                ReportSection::Text(text) => {
                    out.push_str(&format!("{}\n\n", text));
                }
                _ => {}
            }
        }

        out.push_str("\\end{document}\n");
        out
    }

    fn render_text(&self) -> String {
        let mut out = String::new();

        for section in &self.sections {
            match section {
                ReportSection::Header { title, .. } => {
                    out.push_str(&format!("{}\n", title));
                    out.push_str(&"=".repeat(title.len()));
                    out.push_str("\n\n");
                }
                ReportSection::Heading { level, text } => {
                    let prefix = match level {
                        1 => "# ",
                        2 => "## ",
                        _ => "### ",
                    };
                    out.push_str(&format!("{}{}\n\n", prefix, text));
                }
                ReportSection::Text(text) => {
                    out.push_str(&format!("{}\n\n", text));
                }
                _ => {}
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_builder() {
        let report = Report::markdown()
            .header("Test Report")
            .heading(2, "Introduction")
            .text("This is a test.")
            .list(vec!["Item 1", "Item 2"]);

        let rendered = report.render();
        assert!(rendered.contains("# Test Report"));
        assert!(rendered.contains("## Introduction"));
    }

    #[test]
    fn test_html_render() {
        let report = Report::new(ReportFormat::Html)
            .header("HTML Report")
            .text("Hello World");

        let html = report.render();
        assert!(html.contains("<h1>HTML Report</h1>"));
    }
}
