//! Data visualization and export utilities
//!
//! Export data for plotting with external tools (matplotlib, gnuplot, etc.)

use crate::{ResearchError, Result, BenchmarkSuite, ExperimentResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Gnuplot,
    Latex,
}

/// Data point for plotting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub error: Option<f64>,
    pub label: Option<String>,
}

/// Series of data points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSeries {
    pub name: String,
    pub points: Vec<DataPoint>,
    pub color: Option<String>,
    pub style: Option<String>,
}

impl DataSeries {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: Vec::new(),
            color: None,
            style: None,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(DataPoint { x, y, error: None, label: None });
    }

    pub fn add_point_with_error(&mut self, x: f64, y: f64, error: f64) {
        self.points.push(DataPoint { x, y, error: Some(error), label: None });
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn with_style(mut self, style: &str) -> Self {
        self.style = Some(style.to_string());
        self
    }
}

/// Plot data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotData {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub series: Vec<DataSeries>,
    pub x_log_scale: bool,
    pub y_log_scale: bool,
    pub legend_position: Option<String>,
}

impl PlotData {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            x_label: "x".to_string(),
            y_label: "y".to_string(),
            series: Vec::new(),
            x_log_scale: false,
            y_log_scale: false,
            legend_position: None,
        }
    }

    pub fn x_label(mut self, label: &str) -> Self {
        self.x_label = label.to_string();
        self
    }

    pub fn y_label(mut self, label: &str) -> Self {
        self.y_label = label.to_string();
        self
    }

    pub fn add_series(&mut self, series: DataSeries) {
        self.series.push(series);
    }

    pub fn log_scale(mut self, x: bool, y: bool) -> Self {
        self.x_log_scale = x;
        self.y_log_scale = y;
        self
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| ResearchError::SerializationError(e.to_string()))
    }

    /// Export to CSV
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();

        // Header
        let mut header = vec!["x".to_string()];
        for series in &self.series {
            header.push(series.name.clone());
            if self.series.iter().any(|s| s.points.iter().any(|p| p.error.is_some())) {
                header.push(format!("{}_error", series.name));
            }
        }
        csv.push_str(&header.join(","));
        csv.push('\n');

        // Find all unique x values
        let mut x_values: Vec<f64> = self.series.iter()
            .flat_map(|s| s.points.iter().map(|p| p.x))
            .collect();
        x_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        x_values.dedup();

        // Write data rows
        for x in x_values {
            let mut row = vec![format!("{}", x)];
            for series in &self.series {
                if let Some(point) = series.points.iter().find(|p| (p.x - x).abs() < 1e-10) {
                    row.push(format!("{}", point.y));
                    if let Some(err) = point.error {
                        row.push(format!("{}", err));
                    }
                } else {
                    row.push(String::new());
                }
            }
            csv.push_str(&row.join(","));
            csv.push('\n');
        }

        csv
    }

    /// Export to Gnuplot script
    pub fn to_gnuplot(&self) -> String {
        let mut script = String::new();

        script.push_str(&format!("set title '{}'\n", self.title));
        script.push_str(&format!("set xlabel '{}'\n", self.x_label));
        script.push_str(&format!("set ylabel '{}'\n", self.y_label));

        if self.x_log_scale {
            script.push_str("set logscale x\n");
        }
        if self.y_log_scale {
            script.push_str("set logscale y\n");
        }

        script.push_str("set grid\n");
        script.push_str("set key outside right\n\n");

        // Data blocks
        for (i, series) in self.series.iter().enumerate() {
            script.push_str(&format!("$data{} << EOD\n", i));
            for point in &series.points {
                if let Some(err) = point.error {
                    script.push_str(&format!("{} {} {}\n", point.x, point.y, err));
                } else {
                    script.push_str(&format!("{} {}\n", point.x, point.y));
                }
            }
            script.push_str("EOD\n\n");
        }

        // Plot command
        script.push_str("plot ");
        let plots: Vec<String> = self.series.iter().enumerate()
            .map(|(i, s)| {
                let style = s.style.as_deref().unwrap_or("lines");
                if s.points.iter().any(|p| p.error.is_some()) {
                    format!("$data{} using 1:2:3 with yerrorbars title '{}'", i, s.name)
                } else {
                    format!("$data{} using 1:2 with {} title '{}'", i, style, s.name)
                }
            })
            .collect();
        script.push_str(&plots.join(", \\\n     "));
        script.push('\n');

        script
    }

    /// Export to LaTeX (TikZ/PGFPlots)
    pub fn to_latex(&self) -> String {
        let mut latex = String::new();

        latex.push_str("\\begin{tikzpicture}\n");
        latex.push_str("\\begin{axis}[\n");
        latex.push_str(&format!("    title={{{}}},\n", self.title));
        latex.push_str(&format!("    xlabel={{{}}},\n", self.x_label));
        latex.push_str(&format!("    ylabel={{{}}},\n", self.y_label));
        latex.push_str("    grid=major,\n");
        latex.push_str("    legend pos=outer north east,\n");

        if self.x_log_scale {
            latex.push_str("    xmode=log,\n");
        }
        if self.y_log_scale {
            latex.push_str("    ymode=log,\n");
        }

        latex.push_str("]\n\n");

        for series in &self.series {
            latex.push_str(&format!("\\addplot+[mark=*] coordinates {{\n"));
            for point in &series.points {
                latex.push_str(&format!("    ({}, {})\n", point.x, point.y));
            }
            latex.push_str("};\n");
            latex.push_str(&format!("\\addlegendentry{{{}}}\n\n", series.name));
        }

        latex.push_str("\\end{axis}\n");
        latex.push_str("\\end{tikzpicture}\n");

        latex
    }

    /// Generate Python/Matplotlib code
    pub fn to_matplotlib(&self) -> String {
        let mut py = String::new();

        py.push_str("import matplotlib.pyplot as plt\nimport numpy as np\n\n");
        py.push_str("fig, ax = plt.subplots(figsize=(10, 6))\n\n");

        for series in &self.series {
            let x_data: Vec<String> = series.points.iter().map(|p| p.x.to_string()).collect();
            let y_data: Vec<String> = series.points.iter().map(|p| p.y.to_string()).collect();

            py.push_str(&format!("x_{} = np.array([{}])\n", series.name.replace(" ", "_"), x_data.join(", ")));
            py.push_str(&format!("y_{} = np.array([{}])\n", series.name.replace(" ", "_"), y_data.join(", ")));

            if series.points.iter().any(|p| p.error.is_some()) {
                let err_data: Vec<String> = series.points.iter()
                    .map(|p| p.error.unwrap_or(0.0).to_string())
                    .collect();
                py.push_str(&format!("err_{} = np.array([{}])\n", series.name.replace(" ", "_"), err_data.join(", ")));
                py.push_str(&format!(
                    "ax.errorbar(x_{name}, y_{name}, yerr=err_{name}, label='{label}', capsize=3)\n",
                    name = series.name.replace(" ", "_"),
                    label = series.name
                ));
            } else {
                py.push_str(&format!(
                    "ax.plot(x_{name}, y_{name}, 'o-', label='{label}')\n",
                    name = series.name.replace(" ", "_"),
                    label = series.name
                ));
            }
            py.push('\n');
        }

        py.push_str(&format!("ax.set_xlabel('{}')\n", self.x_label));
        py.push_str(&format!("ax.set_ylabel('{}')\n", self.y_label));
        py.push_str(&format!("ax.set_title('{}')\n", self.title));

        if self.x_log_scale {
            py.push_str("ax.set_xscale('log')\n");
        }
        if self.y_log_scale {
            py.push_str("ax.set_yscale('log')\n");
        }

        py.push_str("ax.legend()\n");
        py.push_str("ax.grid(True, alpha=0.3)\n");
        py.push_str("plt.tight_layout()\n");
        py.push_str("plt.savefig('plot.png', dpi=150)\n");
        py.push_str("plt.show()\n");

        py
    }
}

/// Data export utilities
pub struct DataExport;

impl DataExport {
    /// Export benchmark suite to plot data
    pub fn benchmark_to_plot(suite: &BenchmarkSuite, x_param: &str, y_metric: &str) -> PlotData {
        let mut plot = PlotData::new(&suite.name)
            .x_label(x_param)
            .y_label(y_metric);

        let mut series = DataSeries::new(&suite.name);

        for result in &suite.results {
            if let Some(x_val) = result.config.params.get(x_param).and_then(|v| v.as_f64()) {
                let y_val = match y_metric {
                    "time_ms" => result.mean_duration().as_secs_f64() * 1000.0,
                    "gate_count" => result.mean_gate_count().unwrap_or(0.0),
                    "success_prob" => result.mean_success_probability().unwrap_or(0.0),
                    _ => 0.0,
                };

                let error = result.std_duration().as_secs_f64() * 1000.0;
                series.add_point_with_error(x_val, y_val, error);
            }
        }

        plot.add_series(series);
        plot
    }

    /// Export experiment to plot data
    pub fn experiment_to_plot(
        result: &ExperimentResult,
        x_param: &str,
        y_field: &str,
    ) -> PlotData {
        let mut plot = PlotData::new(&result.config.name)
            .x_label(x_param)
            .y_label(y_field);

        let means = result.mean_by_param(y_field, x_param);

        let mut series = DataSeries::new(&result.config.name);
        let mut points: Vec<(f64, f64)> = means.iter()
            .filter_map(|(k, &v)| {
                k.trim_matches('"').parse::<f64>().ok().map(|x| (x, v))
            })
            .collect();

        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (x, y) in points {
            series.add_point(x, y);
        }

        plot.add_series(series);
        plot
    }

    /// Create histogram data
    pub fn histogram(data: &[f64], bins: usize) -> PlotData {
        let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let bin_width = (max - min) / bins as f64;

        let mut counts = vec![0usize; bins];
        for &val in data {
            let idx = ((val - min) / bin_width) as usize;
            let idx = idx.min(bins - 1);
            counts[idx] += 1;
        }

        let mut plot = PlotData::new("Histogram")
            .x_label("Value")
            .y_label("Count");

        let mut series = DataSeries::new("counts").with_style("boxes");
        for (i, count) in counts.iter().enumerate() {
            let x = min + (i as f64 + 0.5) * bin_width;
            series.add_point(x, *count as f64);
        }

        plot.add_series(series);
        plot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_series() {
        let mut series = DataSeries::new("test");
        series.add_point(1.0, 2.0);
        series.add_point(2.0, 4.0);

        assert_eq!(series.points.len(), 2);
    }

    #[test]
    fn test_plot_export() {
        let mut plot = PlotData::new("Test Plot")
            .x_label("X")
            .y_label("Y");

        let mut series = DataSeries::new("data");
        series.add_point(1.0, 1.0);
        series.add_point(2.0, 4.0);
        series.add_point(3.0, 9.0);
        plot.add_series(series);

        let csv = plot.to_csv();
        assert!(csv.contains("x,data"));

        let json = plot.to_json().unwrap();
        assert!(json.contains("Test Plot"));
    }

    #[test]
    fn test_histogram() {
        let data = vec![1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
        let plot = DataExport::histogram(&data, 4);

        assert_eq!(plot.series.len(), 1);
    }
}
