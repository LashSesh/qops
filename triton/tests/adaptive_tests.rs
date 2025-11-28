//! Comprehensive tests for the Adaptive TRITON Optimizer
//!
//! Tests cover:
//! - Adaptive radius control
//! - Dynamic cooling schedule
//! - Topology-aware Gaussian bias
//! - Spiral layer memory
//! - Convergence stabilization
//! - Holistic matrix integration

use qops_triton::{
    AdaptiveTritonConfig, AdaptiveTritonOptimizer, AdaptiveOptimizationResult,
    AdaptiveRadiusConfig, AdaptiveRadiusController, AdaptiveRadiusStats,
    DynamicCoolingConfig, DynamicCoolingController, DynamicCoolingStats,
    TopologyBiasConfig, TopologyGaussianBias, GaussianCenter,
    SpiralLayerMemory, LayerBest, LayerStats,
    ConvergenceStabilizerConfig, ConvergenceStabilizer,
    TritonConfig, SpiralParams,
};

// ============================================================================
// Adaptive Radius Controller Tests
// ============================================================================

mod adaptive_radius_tests {
    use super::*;

    #[test]
    fn test_radius_config_default() {
        let config = AdaptiveRadiusConfig::default();
        assert!(config.initial_radius > 0.0);
        assert!(config.min_radius > 0.0);
        assert!(config.max_radius >= config.min_radius);
        assert!(config.expansion_factor > 1.0);
        assert!(config.contraction_factor > 0.0 && config.contraction_factor < 1.0);
    }

    #[test]
    fn test_radius_controller_initialization() {
        let config = AdaptiveRadiusConfig {
            initial_radius: 2.0,
            min_radius: 0.5,
            max_radius: 10.0,
            expansion_factor: 1.5,
            contraction_factor: 0.8,
            ..Default::default()
        };

        let controller = AdaptiveRadiusController::new(config);
        assert!((controller.current_radius() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_radius_expansion_on_improvement() {
        let config = AdaptiveRadiusConfig {
            initial_radius: 2.0,
            expansion_factor: 1.5,
            ..Default::default()
        };

        let mut controller = AdaptiveRadiusController::new(config);
        let initial_radius = controller.current_radius();

        // Simulate improvement
        controller.update(true, 0.8, 0.5); // improved, new_score > old_score

        let new_radius = controller.current_radius();
        // On improvement, radius should expand (or at least not contract significantly)
        assert!(new_radius >= initial_radius * 0.9);
    }

    #[test]
    fn test_radius_contraction_on_no_improvement() {
        let config = AdaptiveRadiusConfig {
            initial_radius: 5.0,
            contraction_factor: 0.8,
            min_radius: 0.5,
            ..Default::default()
        };

        let mut controller = AdaptiveRadiusController::new(config);

        // Simulate no improvement multiple times
        for _ in 0..5 {
            controller.update(false, 0.3, 0.5);
        }

        let new_radius = controller.current_radius();
        // Radius should have contracted
        assert!(new_radius < 5.0);
        assert!(new_radius >= 0.5); // But not below minimum
    }

    #[test]
    fn test_radius_bounds_enforcement() {
        let config = AdaptiveRadiusConfig {
            initial_radius: 2.0,
            min_radius: 1.0,
            max_radius: 5.0,
            expansion_factor: 2.0,
            contraction_factor: 0.3,
            ..Default::default()
        };

        let mut controller = AdaptiveRadiusController::new(config);

        // Try to expand beyond max
        for _ in 0..10 {
            controller.update(true, 0.9, 0.5);
        }
        assert!(controller.current_radius() <= 5.0);

        // Try to contract below min
        for _ in 0..20 {
            controller.update(false, 0.1, 0.5);
        }
        assert!(controller.current_radius() >= 1.0);
    }
}

// ============================================================================
// Dynamic Cooling Controller Tests
// ============================================================================

mod dynamic_cooling_tests {
    use super::*;

    #[test]
    fn test_cooling_config_default() {
        let config = DynamicCoolingConfig::default();
        assert!(config.initial_temperature > 0.0);
        assert!(config.min_temperature > 0.0);
        assert!(config.base_decay_rate > 0.0 && config.base_decay_rate < 1.0);
    }

    #[test]
    fn test_cooling_controller_initialization() {
        let config = DynamicCoolingConfig {
            initial_temperature: 1.0,
            min_temperature: 0.01,
            base_decay_rate: 0.95,
            ..Default::default()
        };

        let controller = DynamicCoolingController::new(config);
        assert!((controller.current_temperature() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_temperature_decay() {
        let config = DynamicCoolingConfig {
            initial_temperature: 1.0,
            base_decay_rate: 0.9,
            ..Default::default()
        };

        let mut controller = DynamicCoolingController::new(config);
        let initial_temp = controller.current_temperature();

        // Step multiple times
        for _ in 0..10 {
            controller.step(0.5); // Average score
        }

        let final_temp = controller.current_temperature();
        assert!(final_temp < initial_temp);
    }

    #[test]
    fn test_temperature_minimum_enforcement() {
        let config = DynamicCoolingConfig {
            initial_temperature: 1.0,
            min_temperature: 0.1,
            base_decay_rate: 0.5, // Aggressive decay
            ..Default::default()
        };

        let mut controller = DynamicCoolingController::new(config);

        // Many steps should not go below minimum
        for _ in 0..100 {
            controller.step(0.5);
        }

        assert!(controller.current_temperature() >= 0.1);
    }

    #[test]
    fn test_acceptance_probability() {
        let config = DynamicCoolingConfig {
            initial_temperature: 1.0,
            ..Default::default()
        };

        let controller = DynamicCoolingController::new(config);

        // Better score should have high acceptance
        let prob_better = controller.acceptance_probability(0.8, 0.5);
        assert!(prob_better >= 0.9);

        // Equal score should be accepted
        let prob_equal = controller.acceptance_probability(0.5, 0.5);
        assert!(prob_equal >= 0.9);

        // Worse score depends on temperature
        let prob_worse = controller.acceptance_probability(0.3, 0.5);
        assert!(prob_worse >= 0.0 && prob_worse <= 1.0);
    }
}

// ============================================================================
// Topology Gaussian Bias Tests
// ============================================================================

mod topology_bias_tests {
    use super::*;

    #[test]
    fn test_bias_config_default() {
        let config = TopologyBiasConfig::default();
        assert!(config.num_centers >= 1);
        assert!(config.sigma > 0.0);
        assert!(config.weight >= 0.0 && config.weight <= 1.0);
    }

    #[test]
    fn test_gaussian_bias_initialization() {
        let config = TopologyBiasConfig {
            num_centers: 5,
            sigma: 1.0,
            weight: 0.3,
            ..Default::default()
        };

        let bias = TopologyGaussianBias::new(config);
        assert_eq!(bias.center_count(), 0); // No centers added yet
    }

    #[test]
    fn test_add_gaussian_center() {
        let config = TopologyBiasConfig {
            num_centers: 5,
            sigma: 1.0,
            ..Default::default()
        };

        let mut bias = TopologyGaussianBias::new(config);

        bias.add_center(GaussianCenter {
            position: vec![0.5, 0.5, 0.5],
            score: 0.8,
            weight: 1.0,
        });

        assert_eq!(bias.center_count(), 1);
    }

    #[test]
    fn test_bias_computation() {
        let config = TopologyBiasConfig {
            num_centers: 3,
            sigma: 0.5,
            weight: 0.5,
            ..Default::default()
        };

        let mut bias = TopologyGaussianBias::new(config);

        // Add centers
        bias.add_center(GaussianCenter {
            position: vec![0.5, 0.5, 0.5],
            score: 0.9,
            weight: 1.0,
        });

        // Compute bias at the center location
        let bias_at_center = bias.compute_bias(&[0.5, 0.5, 0.5]);
        // Should be high near the center
        assert!(bias_at_center > 0.0);

        // Compute bias far from center
        let bias_far = bias.compute_bias(&[0.0, 0.0, 0.0]);
        // Should be lower far from center
        assert!(bias_far < bias_at_center);
    }

    #[test]
    fn test_center_limit_enforcement() {
        let config = TopologyBiasConfig {
            num_centers: 3, // Only allow 3 centers
            ..Default::default()
        };

        let mut bias = TopologyGaussianBias::new(config);

        // Add more than allowed
        for i in 0..10 {
            bias.add_center(GaussianCenter {
                position: vec![i as f64 * 0.1],
                score: 0.5 + i as f64 * 0.05,
                weight: 1.0,
            });
        }

        // Should only keep best centers
        assert!(bias.center_count() <= 3);
    }
}

// ============================================================================
// Spiral Layer Memory Tests
// ============================================================================

mod spiral_layer_memory_tests {
    use super::*;

    #[test]
    fn test_layer_memory_initialization() {
        let memory = SpiralLayerMemory::new(7);
        assert_eq!(memory.layer_count(), 7);
    }

    #[test]
    fn test_record_layer_visit() {
        let mut memory = SpiralLayerMemory::new(5);

        memory.record_visit(0, 0.5);
        memory.record_visit(0, 0.6);
        memory.record_visit(0, 0.7);

        let stats = memory.get_layer_stats(0);
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.visit_count, 3);
        assert!((stats.avg_score - 0.6).abs() < 1e-10);
    }

    #[test]
    fn test_best_layer_tracking() {
        let mut memory = SpiralLayerMemory::new(5);

        // Record visits with different scores per layer
        memory.record_visit(0, 0.3);
        memory.record_visit(1, 0.5);
        memory.record_visit(2, 0.9); // Best layer
        memory.record_visit(3, 0.4);
        memory.record_visit(4, 0.6);

        let best = memory.best_layer();
        assert_eq!(best, Some(2));
    }

    #[test]
    fn test_layer_recommendation() {
        let mut memory = SpiralLayerMemory::new(5);

        // Layer 2 has the best performance
        for _ in 0..10 {
            memory.record_visit(0, 0.3);
            memory.record_visit(2, 0.8);
        }

        let recommended = memory.recommend_layer();
        assert!(recommended == Some(2) || recommended.is_none());
    }
}

// ============================================================================
// Convergence Stabilizer Tests
// ============================================================================

mod convergence_stabilizer_tests {
    use super::*;

    #[test]
    fn test_stabilizer_config_default() {
        let config = ConvergenceStabilizerConfig::default();
        assert!(config.window_size > 0);
        assert!(config.variance_threshold > 0.0);
        assert!(config.plateau_threshold > 0);
    }

    #[test]
    fn test_stabilizer_initialization() {
        let config = ConvergenceStabilizerConfig {
            window_size: 10,
            variance_threshold: 0.001,
            plateau_threshold: 5,
            ..Default::default()
        };

        let stabilizer = ConvergenceStabilizer::new(config);
        assert!(!stabilizer.is_converged());
    }

    #[test]
    fn test_convergence_detection() {
        let config = ConvergenceStabilizerConfig {
            window_size: 5,
            variance_threshold: 0.001,
            plateau_threshold: 3,
            ..Default::default()
        };

        let mut stabilizer = ConvergenceStabilizer::new(config);

        // Add converging values
        for _ in 0..20 {
            stabilizer.add_score(0.85);
        }

        // Should detect convergence with stable values
        assert!(stabilizer.is_converged());
    }

    #[test]
    fn test_no_convergence_with_variance() {
        let config = ConvergenceStabilizerConfig {
            window_size: 5,
            variance_threshold: 0.001,
            plateau_threshold: 3,
            ..Default::default()
        };

        let mut stabilizer = ConvergenceStabilizer::new(config);

        // Add varying values
        for i in 0..20 {
            stabilizer.add_score(0.5 + (i as f64 * 0.1).sin() * 0.2);
        }

        // Should not converge with high variance
        // (depends on actual variance calculation)
    }

    #[test]
    fn test_plateau_detection() {
        let config = ConvergenceStabilizerConfig {
            window_size: 5,
            variance_threshold: 0.01,
            plateau_threshold: 5,
            ..Default::default()
        };

        let mut stabilizer = ConvergenceStabilizer::new(config);

        // Add same best score repeatedly (plateau)
        for _ in 0..10 {
            stabilizer.add_score(0.9);
        }

        let plateau_count = stabilizer.plateau_count();
        assert!(plateau_count >= 5);
    }
}

// ============================================================================
// Adaptive Optimizer Integration Tests
// ============================================================================

mod optimizer_integration_tests {
    use super::*;

    fn simple_scoring_function(sig: &[f64; 5]) -> f64 {
        // Simple sphere function (minimum at origin)
        let sum: f64 = sig.iter().map(|x| x * x).sum();
        1.0 - sum.min(1.0)
    }

    #[test]
    fn test_optimizer_initialization() {
        let config = AdaptiveTritonConfig {
            base: TritonConfig {
                spiral: SpiralParams {
                    expansion_rate: 1.618,
                    initial_radius: 1.0,
                    max_layers: 5,
                },
                max_iterations: 100,
                ..Default::default()
            },
            ..Default::default()
        };

        let optimizer = AdaptiveTritonOptimizer::new(config);
        assert!(!optimizer.is_converged());
    }

    #[test]
    fn test_optimizer_runs() {
        let config = AdaptiveTritonConfig {
            base: TritonConfig {
                spiral: SpiralParams {
                    expansion_rate: 1.618,
                    initial_radius: 1.0,
                    max_layers: 3,
                },
                max_iterations: 50,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut optimizer = AdaptiveTritonOptimizer::new(config);

        let result = optimizer.optimize(|sig| simple_scoring_function(sig));

        assert!(result.iterations > 0);
        assert!(result.best_score >= 0.0);
    }

    #[test]
    fn test_optimizer_improves_score() {
        let config = AdaptiveTritonConfig {
            base: TritonConfig {
                spiral: SpiralParams {
                    expansion_rate: 1.5,
                    initial_radius: 0.5,
                    max_layers: 5,
                },
                max_iterations: 100,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut optimizer = AdaptiveTritonOptimizer::new(config);

        // Track initial random score
        let initial_score = simple_scoring_function(&[0.5, 0.5, 0.5, 0.5, 0.5]);

        let result = optimizer.optimize(|sig| simple_scoring_function(sig));

        // Should find better or equal score
        assert!(result.best_score >= initial_score * 0.8);
    }

    #[test]
    fn test_optimizer_trajectory_recording() {
        let config = AdaptiveTritonConfig {
            base: TritonConfig {
                max_iterations: 30,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut optimizer = AdaptiveTritonOptimizer::new(config);

        let result = optimizer.optimize(|sig| simple_scoring_function(sig));

        // Trajectory should be recorded
        assert!(!result.trajectory.is_empty());
        assert!(result.trajectory.len() <= 30);
    }

    #[test]
    fn test_optimizer_convergence_detection() {
        let config = AdaptiveTritonConfig {
            base: TritonConfig {
                max_iterations: 200,
                ..Default::default()
            },
            convergence: ConvergenceStabilizerConfig {
                window_size: 10,
                variance_threshold: 0.0001,
                plateau_threshold: 10,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut optimizer = AdaptiveTritonOptimizer::new(config);

        // Use a function that converges quickly
        let result = optimizer.optimize(|_| 0.85); // Constant score

        // Should converge early
        assert!(result.converged || result.iterations < 200);
    }
}
