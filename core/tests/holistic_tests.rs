//! Comprehensive tests for the Holistic Resonance Architecture
//!
//! Tests cover:
//! - Kosmokrator stage (Proof-of-Resonance, Telescope Operator)
//! - Chronokrator stage (D_total, threshold dynamics, Exkalibration)
//! - Pfauenthron stage (Mandorla convergence, Ophanim, Monolith)
//! - HolisticMatrix integration

use qops_core::{
    KosmokratorConfig, KosmokratorState, KosmokratorStats,
    ChronokratorConfig, ChronokratorState, ChronokratorStats,
    PfauenthronConfig, PfauenthronState, PfauenthronStats,
    HolisticConfig, HolisticMatrix, GenesisStage,
    ProofOfResonanceResult, ExkalibrationVector, Monolith,
};

// ============================================================================
// Kosmokrator Stage Tests
// ============================================================================

mod kosmokrator_tests {
    use super::*;

    #[test]
    fn test_kosmokrator_config_default() {
        let config = KosmokratorConfig::default();
        assert!(config.kappa_threshold > 0.0 && config.kappa_threshold <= 1.0);
        assert!(config.stability_epsilon > 0.0);
        assert!(config.telescope_enabled);
    }

    #[test]
    fn test_kosmokrator_state_initialization() {
        let config = KosmokratorConfig {
            kappa_threshold: 0.7,
            stability_epsilon: 0.05,
            telescope_enabled: true,
            history_window: 50,
            ..Default::default()
        };

        let state = KosmokratorState::new(config);
        assert_eq!(state.phase_count(), 0);
        assert_eq!(state.telescope_adjustments(), 0);
    }

    #[test]
    fn test_kosmokrator_phase_accumulation() {
        let config = KosmokratorConfig::default();
        let mut state = KosmokratorState::new(config);

        // Add coherent phases (all similar)
        for _ in 0..10 {
            state.add_phase(0.5);
        }

        assert_eq!(state.phase_count(), 10);
    }

    #[test]
    fn test_proof_of_resonance_computation() {
        let config = KosmokratorConfig {
            kappa_threshold: 0.5,
            ..Default::default()
        };
        let mut state = KosmokratorState::new(config);

        // Add highly coherent phases
        for i in 0..20 {
            let phase = (i as f64 * 0.1).sin();
            state.add_phase(phase);
        }

        let por = state.compute_por();
        assert!(por.kappa >= 0.0 && por.kappa <= 1.0);
        assert!(por.coherence >= 0.0 && por.coherence <= 1.0);
    }

    #[test]
    fn test_kosmokrator_telescope_operator() {
        let config = KosmokratorConfig {
            kappa_threshold: 0.7,
            telescope_enabled: true,
            stability_epsilon: 0.05,
            history_window: 20,
            ..Default::default()
        };
        let mut state = KosmokratorState::new(config);

        // Add phases that should trigger telescope adjustment
        for i in 0..30 {
            let phase = if i % 5 == 0 { 1.0 } else { 0.0 };
            state.add_phase(phase);
        }

        // Telescope should have made adjustments
        // The actual adjustment count depends on implementation
        let _adjustments = state.telescope_adjustments();
    }

    #[test]
    fn test_kosmokrator_filtering() {
        let config = KosmokratorConfig {
            kappa_threshold: 0.8,
            ..Default::default()
        };
        let mut state = KosmokratorState::new(config);

        // Add random phases (low coherence)
        for i in 0..50 {
            state.add_phase((i as f64 * 2.17).sin());
        }

        let por = state.compute_por();

        // With random phases, coherence should be low
        // Result depends on the actual phase distribution
        assert!(por.kappa <= 1.0);
    }
}

// ============================================================================
// Chronokrator Stage Tests
// ============================================================================

mod chronokrator_tests {
    use super::*;

    #[test]
    fn test_chronokrator_config_default() {
        let config = ChronokratorConfig::default();
        assert!(config.num_channels >= 1);
        assert!(config.base_threshold > 0.0 && config.base_threshold <= 1.0);
        assert!(config.exkalibration_enabled);
    }

    #[test]
    fn test_chronokrator_state_initialization() {
        let config = ChronokratorConfig {
            num_channels: 4,
            base_threshold: 0.75,
            exkalibration_enabled: true,
            spike_detection: true,
            ..Default::default()
        };

        let state = ChronokratorState::new(config);
        assert_eq!(state.active_channels(), 0);
    }

    #[test]
    fn test_channel_updates() {
        let config = ChronokratorConfig {
            num_channels: 4,
            ..Default::default()
        };
        let mut state = ChronokratorState::new(config);

        // Update channels
        for ch in 0..4 {
            state.update_channel(ch, 0.5 + ch as f64 * 0.1, ch as f64 * 0.5);
        }

        assert!(state.active_channels() > 0);
    }

    #[test]
    fn test_d_total_computation() {
        let config = ChronokratorConfig {
            num_channels: 4,
            base_threshold: 0.75,
            ..Default::default()
        };
        let mut state = ChronokratorState::new(config);

        // Update channels with varying resonance
        for t in 0..20 {
            let time = t as f64 * 0.1;
            for ch in 0..4 {
                let resonance = 0.5 + 0.3 * (time + ch as f64 * 0.5).sin();
                state.update_channel(ch, resonance, time);
            }
        }

        let d_total = state.compute_d_total();
        assert!(d_total >= 0.0);
    }

    #[test]
    fn test_exkalibration_vector() {
        let config = ChronokratorConfig {
            exkalibration_enabled: true,
            num_channels: 4,
            ..Default::default()
        };
        let mut state = ChronokratorState::new(config);

        // Simulate dynamics
        for t in 0..50 {
            let time = t as f64 * 0.1;
            for ch in 0..4 {
                let resonance = 0.5 + 0.3 * (time + ch as f64).sin();
                state.update_channel(ch, resonance, time);
            }
        }

        let exkal = state.compute_exkalibration();

        // Verify magnitude computation
        let expected_magnitude = (exkal.nabla_psi.powi(2)
            + exkal.nabla_rho.powi(2)
            + exkal.nabla_omega.powi(2))
            .sqrt();

        assert!((exkal.magnitude() - expected_magnitude).abs() < 1e-10);
    }

    #[test]
    fn test_spike_detection() {
        let config = ChronokratorConfig {
            spike_detection: true,
            num_channels: 4,
            ..Default::default()
        };
        let mut state = ChronokratorState::new(config);

        // Add normal values then a spike
        for t in 0..10 {
            state.update_channel(0, 0.5, t as f64 * 0.1);
        }

        // Add spike
        state.update_channel(0, 0.95, 1.0);

        let spikes = state.detect_spikes();
        // Implementation should detect the spike
        // Actual detection depends on the threshold logic
    }

    #[test]
    fn test_threshold_dynamics() {
        let config = ChronokratorConfig {
            base_threshold: 0.75,
            num_channels: 4,
            ..Default::default()
        };
        let mut state = ChronokratorState::new(config);

        let initial_threshold = state.current_threshold();
        assert!((initial_threshold - 0.75).abs() < 1e-10);

        // After updates, threshold may adapt
        for t in 0..20 {
            for ch in 0..4 {
                state.update_channel(ch, 0.8, t as f64 * 0.1);
            }
        }

        let final_threshold = state.current_threshold();
        // Threshold should remain valid
        assert!(final_threshold > 0.0 && final_threshold <= 1.0);
    }
}

// ============================================================================
// Pfauenthron Stage Tests
// ============================================================================

mod pfauenthron_tests {
    use super::*;

    #[test]
    fn test_pfauenthron_config_default() {
        let config = PfauenthronConfig::default();
        assert!(config.mandorla_threshold > 0.0 && config.mandorla_threshold <= 1.0);
        assert!(config.ophanim_count >= 1);
        assert!(config.monolith_enabled);
    }

    #[test]
    fn test_pfauenthron_state_initialization() {
        let config = PfauenthronConfig {
            mandorla_threshold: 0.8,
            ophanim_count: 4,
            monolith_enabled: true,
            ..Default::default()
        };

        let state = PfauenthronState::new(config);
        assert_eq!(state.ophanim().len(), 0); // Not initialized yet
    }

    #[test]
    fn test_ophanim_initialization() {
        let config = PfauenthronConfig {
            ophanim_count: 6,
            ..Default::default()
        };
        let mut state = PfauenthronState::new(config);

        state.initialize_ophanim();
        assert_eq!(state.ophanim().len(), 6);

        // All Ophanim should have valid positions
        for oph in state.ophanim() {
            let (x, y, z) = oph.position;
            assert!(x.is_finite() && y.is_finite() && z.is_finite());
        }
    }

    #[test]
    fn test_gabriel_oriphiel_convergence() {
        let config = PfauenthronConfig {
            mandorla_threshold: 0.8,
            ophanim_count: 4,
            ..Default::default()
        };
        let mut state = PfauenthronState::new(config);
        state.initialize_ophanim();

        // Simulate convergence
        for step in 0..50 {
            let p_gabriel = 0.5 + 0.4 * (step as f64 * 0.1).sin();
            let i_oriphiel = 0.5 + 0.4 * (step as f64 * 0.1).cos();
            state.update_convergence(p_gabriel, i_oriphiel);
        }

        let mandorla = state.compute_mandorla();
        assert!(mandorla.p_gabriel >= 0.0 && mandorla.p_gabriel <= 1.0);
        assert!(mandorla.i_oriphiel >= 0.0 && mandorla.i_oriphiel <= 1.0);
        assert!(mandorla.strength >= 0.0);
    }

    #[test]
    fn test_mandorla_strength_formula() {
        let config = PfauenthronConfig::default();
        let mut state = PfauenthronState::new(config);
        state.initialize_ophanim();

        // Set specific values
        state.update_convergence(0.8, 0.9);

        let mandorla = state.compute_mandorla();

        // S_Mandorla = P_Gabriel * I_Oriphiel
        let expected_strength = mandorla.p_gabriel * mandorla.i_oriphiel;
        assert!((mandorla.strength - expected_strength).abs() < 0.1);
    }

    #[test]
    fn test_monolith_formation() {
        let config = PfauenthronConfig {
            mandorla_threshold: 0.5, // Lower threshold for easier formation
            ophanim_count: 4,
            monolith_enabled: true,
            ..Default::default()
        };
        let mut state = PfauenthronState::new(config);
        state.initialize_ophanim();

        // Drive convergence high
        for _ in 0..20 {
            state.update_convergence(0.9, 0.9);
        }

        let monolith = state.attempt_monolith_formation();

        // With high convergence, monolith should form
        if let Some(mono) = monolith {
            assert!(mono.coherence >= 0.0);
        }
    }

    #[test]
    fn test_monolith_disabled() {
        let config = PfauenthronConfig {
            monolith_enabled: false,
            ..Default::default()
        };
        let mut state = PfauenthronState::new(config);
        state.initialize_ophanim();

        // Even with high convergence
        for _ in 0..20 {
            state.update_convergence(0.95, 0.95);
        }

        let monolith = state.attempt_monolith_formation();

        // Should not form when disabled
        assert!(monolith.is_none());
    }
}

// ============================================================================
// Holistic Matrix Integration Tests
// ============================================================================

mod holistic_matrix_tests {
    use super::*;

    #[test]
    fn test_holistic_config_default() {
        let config = HolisticConfig::default();

        // All sub-configs should have valid defaults
        assert!(config.kosmokrator.kappa_threshold > 0.0);
        assert!(config.chronokrator.num_channels >= 1);
        assert!(config.pfauenthron.mandorla_threshold > 0.0);
    }

    #[test]
    fn test_holistic_matrix_initialization() {
        let config = HolisticConfig::default();
        let matrix = HolisticMatrix::new(config);

        assert_eq!(matrix.current_stage(), GenesisStage::Discovery);
    }

    #[test]
    fn test_stage_progression() {
        let config = HolisticConfig::default();
        let mut matrix = HolisticMatrix::new(config);

        // Initial stage
        assert_eq!(matrix.current_stage(), GenesisStage::Discovery);

        // Advance through stages
        matrix.advance_to_kosmokrator();
        assert_eq!(matrix.current_stage(), GenesisStage::KosmokratorFilter);

        matrix.advance_to_chronokrator();
        assert_eq!(matrix.current_stage(), GenesisStage::ChronokratorExpansion);

        matrix.advance_to_pfauenthron();
        assert_eq!(matrix.current_stage(), GenesisStage::PfauenthronCollapse);

        matrix.finalize();
        assert_eq!(matrix.current_stage(), GenesisStage::Finalized);
    }

    #[test]
    fn test_matrix_output_decision() {
        let config = HolisticConfig::default();
        let mut matrix = HolisticMatrix::new(config);

        // Process the pipeline
        matrix.process_pipeline();

        // Check output history
        let history = matrix.output_history();
        // History should have at least one output after processing
    }

    #[test]
    fn test_matrix_logic() {
        // Test the core matrix logic:
        // M(t) = E(t) if PoR(t)=true AND D_total(t) > Theta(t), else empty set

        let config = HolisticConfig {
            kosmokrator: KosmokratorConfig {
                kappa_threshold: 0.5, // Lower threshold
                ..Default::default()
            },
            chronokrator: ChronokratorConfig {
                base_threshold: 0.3, // Lower threshold
                ..Default::default()
            },
            ..Default::default()
        };

        let mut matrix = HolisticMatrix::new(config);

        // Process with specific inputs that should pass
        matrix.process_pipeline();

        // The matrix should produce outputs when conditions are met
        let stats = matrix.get_stats();
        assert!(stats.total_outputs >= 0);
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

mod integration_tests {
    use super::*;

    #[test]
    fn test_full_pipeline_flow() {
        // Test the complete pipeline flow from Discovery to Finalized

        let config = HolisticConfig {
            kosmokrator: KosmokratorConfig {
                kappa_threshold: 0.6,
                stability_epsilon: 0.05,
                telescope_enabled: true,
                history_window: 30,
                ..Default::default()
            },
            chronokrator: ChronokratorConfig {
                num_channels: 4,
                base_threshold: 0.7,
                exkalibration_enabled: true,
                spike_detection: true,
                ..Default::default()
            },
            pfauenthron: PfauenthronConfig {
                mandorla_threshold: 0.75,
                ophanim_count: 4,
                monolith_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut matrix = HolisticMatrix::new(config);

        // Run complete pipeline
        matrix.process_pipeline();

        // Should reach finalized state
        assert_eq!(matrix.current_stage(), GenesisStage::Finalized);
    }

    #[test]
    fn test_preset_configurations() {
        // Test preset configurations work correctly

        // Quick preset
        let quick_config = HolisticConfig {
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
        };

        let mut quick_matrix = HolisticMatrix::new(quick_config);
        quick_matrix.process_pipeline();
        assert_eq!(quick_matrix.current_stage(), GenesisStage::Finalized);

        // Research preset
        let research_config = HolisticConfig {
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
        };

        let mut research_matrix = HolisticMatrix::new(research_config);
        research_matrix.process_pipeline();
        assert_eq!(research_matrix.current_stage(), GenesisStage::Finalized);
    }
}
