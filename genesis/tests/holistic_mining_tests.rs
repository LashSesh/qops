//! Comprehensive tests for the Genesis Holistic Mining Session
//!
//! Tests cover:
//! - HolisticMiningConfig construction
//! - HolisticMiningSession pipeline execution
//! - Stage transitions and logging
//! - Result structure validation
//! - Export and reporting functionality

use qops_genesis::{
    HolisticMiningConfig, HolisticMiningSession, HolisticMiningResult,
    StageLogEntry, StageMetrics, GenesisStage,
    GenesisReporter, ReportFormat, ReportConfig, GenesisReport,
    MetatronCube, OperatorFamily, FamilyCharacteristics,
};
use qops_core::{HolisticConfig, KosmokratorConfig, ChronokratorConfig, PfauenthronConfig};

// ============================================================================
// Configuration Tests
// ============================================================================

mod config_tests {
    use super::*;

    #[test]
    fn test_holistic_mining_config_default() {
        let config = HolisticMiningConfig::default();

        assert!(config.num_agents > 0);
        assert!(config.steps_per_agent > 0);
        assert!(config.holistic.kosmokrator.kappa_threshold > 0.0);
        assert!(config.holistic.chronokrator.num_channels >= 1);
        assert!(config.holistic.pfauenthron.mandorla_threshold > 0.0);
    }

    #[test]
    fn test_quick_preset_config() {
        let config = HolisticMiningConfig {
            holistic: HolisticConfig {
                kosmokrator: KosmokratorConfig {
                    kappa_threshold: 0.6,
                    ..Default::default()
                },
                chronokrator: ChronokratorConfig {
                    num_channels: 2,
                    ..Default::default()
                },
                pfauenthron: PfauenthronConfig {
                    mandorla_threshold: 0.7,
                    ..Default::default()
                },
                ..Default::default()
            },
            num_agents: 5,
            steps_per_agent: 20,
            use_adaptive_triton: false,
            ..Default::default()
        };

        assert_eq!(config.num_agents, 5);
        assert_eq!(config.steps_per_agent, 20);
        assert!(!config.use_adaptive_triton);
    }

    #[test]
    fn test_research_preset_config() {
        let config = HolisticMiningConfig {
            holistic: HolisticConfig {
                kosmokrator: KosmokratorConfig {
                    kappa_threshold: 0.8,
                    history_window: 100,
                    ..Default::default()
                },
                chronokrator: ChronokratorConfig {
                    num_channels: 6,
                    ..Default::default()
                },
                pfauenthron: PfauenthronConfig {
                    mandorla_threshold: 0.9,
                    ophanim_count: 6,
                    ..Default::default()
                },
                ..Default::default()
            },
            num_agents: 20,
            steps_per_agent: 100,
            use_adaptive_triton: true,
            ..Default::default()
        };

        assert_eq!(config.num_agents, 20);
        assert_eq!(config.steps_per_agent, 100);
        assert!(config.use_adaptive_triton);
    }
}

// ============================================================================
// Session Tests
// ============================================================================

mod session_tests {
    use super::*;

    #[test]
    fn test_session_initialization() {
        let config = HolisticMiningConfig::default();
        let session = HolisticMiningSession::new(config);

        assert_eq!(session.current_stage(), GenesisStage::Discovery);
        assert!(session.candidates().is_empty());
    }

    #[test]
    fn test_discovery_stage() {
        let config = HolisticMiningConfig {
            num_agents: 3,
            steps_per_agent: 10,
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);
        session.run_discovery();

        // Should have discovered some candidates
        assert!(session.candidates().len() > 0);
    }

    #[test]
    fn test_kosmokrator_stage() {
        let config = HolisticMiningConfig {
            num_agents: 3,
            steps_per_agent: 10,
            holistic: HolisticConfig {
                kosmokrator: KosmokratorConfig {
                    kappa_threshold: 0.5, // Lower threshold for testing
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);
        session.run_discovery();
        let initial_count = session.candidates().len();

        session.run_kosmokrator();

        // Kosmokrator should filter some candidates
        let filtered_count = session.candidates().len();
        assert!(filtered_count <= initial_count);
    }

    #[test]
    fn test_chronokrator_stage() {
        let config = HolisticMiningConfig {
            num_agents: 5,
            steps_per_agent: 15,
            holistic: HolisticConfig {
                chronokrator: ChronokratorConfig {
                    num_channels: 4,
                    base_threshold: 0.5,
                    exkalibration_enabled: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);
        session.run_discovery();
        session.run_kosmokrator();
        session.run_chronokrator();

        // Should have Chronokrator stats available
        let stats = session.chronokrator_stats();
        assert!(stats.active_channels > 0);
    }

    #[test]
    fn test_pfauenthron_stage() {
        let config = HolisticMiningConfig {
            num_agents: 5,
            steps_per_agent: 15,
            holistic: HolisticConfig {
                pfauenthron: PfauenthronConfig {
                    mandorla_threshold: 0.5, // Lower for testing
                    ophanim_count: 4,
                    monolith_enabled: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);
        session.run_discovery();
        session.run_kosmokrator();
        session.run_chronokrator();
        session.run_pfauenthron();

        // Should have Pfauenthron stats available
        let stats = session.pfauenthron_stats();
        assert!(stats.ophanim_count > 0);
    }

    #[test]
    fn test_full_pipeline_execution() {
        let config = HolisticMiningConfig {
            num_agents: 5,
            steps_per_agent: 20,
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);

        // Run all stages
        session.run_discovery();
        session.run_kosmokrator();
        session.run_chronokrator();
        session.run_pfauenthron();

        let result = session.finalize();

        // Result should have valid structure
        assert!(result.candidates_discovered > 0);
        assert!(result.duration_ms > 0);
    }

    #[test]
    fn test_stage_logging() {
        let config = HolisticMiningConfig {
            export_stage_logs: true,
            num_agents: 3,
            steps_per_agent: 10,
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);

        session.run_discovery();
        session.run_kosmokrator();
        session.run_chronokrator();
        session.run_pfauenthron();

        let result = session.finalize();

        // Should have stage logs
        assert!(!result.stage_logs.is_empty());

        // Each major stage should be logged
        let stages_logged: Vec<_> = result.stage_logs.iter()
            .map(|l| format!("{:?}", l.stage))
            .collect();

        assert!(stages_logged.iter().any(|s| s.contains("Discovery")));
        assert!(stages_logged.iter().any(|s| s.contains("Kosmokrator")));
    }
}

// ============================================================================
// Result Tests
// ============================================================================

mod result_tests {
    use super::*;

    fn run_full_session() -> HolisticMiningResult {
        let config = HolisticMiningConfig {
            num_agents: 5,
            steps_per_agent: 20,
            export_stage_logs: true,
            holistic: HolisticConfig {
                kosmokrator: KosmokratorConfig {
                    kappa_threshold: 0.5,
                    ..Default::default()
                },
                pfauenthron: PfauenthronConfig {
                    mandorla_threshold: 0.5,
                    monolith_enabled: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);
        session.run_discovery();
        session.run_kosmokrator();
        session.run_chronokrator();
        session.run_pfauenthron();
        session.finalize()
    }

    #[test]
    fn test_result_candidates_counts() {
        let result = run_full_session();

        // Candidate counts should form a funnel
        assert!(result.candidates_discovered >= result.candidates_after_kosmokrator);
        assert!(result.candidates_after_kosmokrator >= result.candidates_after_chronokrator);
    }

    #[test]
    fn test_result_best_resonance() {
        let result = run_full_session();

        // Best resonance should be valid
        assert!(result.best_resonance >= 0.0 && result.best_resonance <= 1.0);
    }

    #[test]
    fn test_result_duration() {
        let result = run_full_session();

        // Duration should be positive
        assert!(result.duration_ms > 0);
    }

    #[test]
    fn test_result_matrix_outputs() {
        let result = run_full_session();

        // Matrix should have produced outputs
        assert!(result.matrix_outputs >= 0);
    }

    #[test]
    fn test_result_finalized_families() {
        let result = run_full_session();

        // Families should have valid structure
        for family in &result.finalized_families {
            assert!(!family.name.is_empty());
            assert!(family.member_count > 0);
            assert!(family.avg_resonance >= 0.0 && family.avg_resonance <= 1.0);
        }
    }
}

// ============================================================================
// Reporting Tests
// ============================================================================

mod reporting_tests {
    use super::*;

    fn create_sample_result() -> HolisticMiningResult {
        let config = HolisticMiningConfig {
            num_agents: 3,
            steps_per_agent: 10,
            export_stage_logs: true,
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);
        session.run_discovery();
        session.run_kosmokrator();
        session.run_chronokrator();
        session.run_pfauenthron();
        session.finalize()
    }

    #[test]
    fn test_reporter_creation() {
        let result = create_sample_result();
        let reporter = GenesisReporter::new(result);

        // Should be able to build report
        let report = reporter.build_report();
        assert!(!report.title.is_empty());
    }

    #[test]
    fn test_markdown_generation() {
        let result = create_sample_result();
        let reporter = GenesisReporter::new(result);

        let markdown = reporter.generate(ReportFormat::Markdown).unwrap();

        // Should contain expected sections
        assert!(markdown.contains("# "));
        assert!(markdown.contains("Summary"));
        assert!(markdown.contains("Pipeline"));
    }

    #[test]
    fn test_json_generation() {
        let result = create_sample_result();
        let reporter = GenesisReporter::new(result);

        let json = reporter.generate(ReportFormat::Json).unwrap();

        // Should be valid JSON
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&json);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_csv_generation() {
        let result = create_sample_result();
        let reporter = GenesisReporter::new(result);

        let csv = reporter.generate(ReportFormat::Csv).unwrap();

        // Should have header row
        assert!(csv.contains("name,member_count,avg_resonance"));
    }

    #[test]
    fn test_report_config_customization() {
        let result = create_sample_result();

        let config = ReportConfig {
            include_stage_logs: false,
            include_family_details: true,
            include_pipeline_summary: true,
            include_monolith: false,
            include_timestamp: true,
            title: "Custom Report".to_string(),
        };

        let reporter = GenesisReporter::new(result).with_config(config);
        let report = reporter.build_report();

        assert_eq!(report.title, "Custom Report");
    }

    #[test]
    fn test_report_statistics() {
        let result = create_sample_result();
        let reporter = GenesisReporter::new(result);
        let report = reporter.build_report();

        // Statistics should be valid
        assert!(report.statistics.pipeline_efficiency >= 0.0);
        assert!(report.statistics.pipeline_efficiency <= 1.0);
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

mod integration_tests {
    use super::*;

    #[test]
    fn test_metatron_cube_integration() {
        let cube = MetatronCube::new();
        assert_eq!(cube.node_count(), 5040); // S7 = 7!

        let config = HolisticMiningConfig {
            num_agents: 3,
            steps_per_agent: 10,
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);
        session.run_discovery();

        // Candidates should be from valid cube nodes
        for candidate in session.candidates() {
            assert!(candidate.node_id < 5040);
        }
    }

    #[test]
    fn test_adaptive_triton_integration() {
        let config = HolisticMiningConfig {
            use_adaptive_triton: true,
            num_agents: 3,
            steps_per_agent: 10,
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);
        session.run_discovery();
        session.run_kosmokrator();
        session.run_chronokrator();
        session.run_pfauenthron();

        let result = session.finalize();

        // Should complete successfully with TRITON
        assert!(result.duration_ms > 0);
    }

    #[test]
    fn test_end_to_end_pipeline() {
        // Full end-to-end test simulating real usage

        let config = HolisticMiningConfig {
            holistic: HolisticConfig {
                kosmokrator: KosmokratorConfig {
                    kappa_threshold: 0.6,
                    telescope_enabled: true,
                    ..Default::default()
                },
                chronokrator: ChronokratorConfig {
                    num_channels: 4,
                    exkalibration_enabled: true,
                    spike_detection: true,
                    ..Default::default()
                },
                pfauenthron: PfauenthronConfig {
                    mandorla_threshold: 0.7,
                    ophanim_count: 4,
                    monolith_enabled: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            num_agents: 10,
            steps_per_agent: 30,
            use_adaptive_triton: true,
            export_stage_logs: true,
            ..Default::default()
        };

        let mut session = HolisticMiningSession::new(config);

        // Run pipeline
        session.run_discovery();
        assert!(session.candidates().len() > 0);

        session.run_kosmokrator();
        let kos_stats = session.kosmokrator_stats();
        assert!(kos_stats.input_count > 0);

        session.run_chronokrator();
        let chrono_stats = session.chronokrator_stats();
        assert!(chrono_stats.d_total >= 0.0);

        session.run_pfauenthron();
        let pfau_stats = session.pfauenthron_stats();
        assert!(pfau_stats.mandorla_strength >= 0.0);

        let result = session.finalize();

        // Verify result integrity
        assert!(result.candidates_discovered > 0);
        assert!(result.duration_ms > 0);
        assert!(!result.stage_logs.is_empty());

        // Generate and verify report
        let reporter = GenesisReporter::new(result);
        let json = reporter.generate(ReportFormat::Json);
        assert!(json.is_ok());

        let md = reporter.generate(ReportFormat::Markdown);
        assert!(md.is_ok());
    }
}
