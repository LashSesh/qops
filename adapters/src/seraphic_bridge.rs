//! Bridge for Seraphic Calibration integration.

use qops_core::{Signature, Signature3D, GenerativePipeline};
use qops_seraphic::{SeraphicCalibrator, CalibratorConfig};

/// Bridge for integrating Seraphic calibration with pipelines
pub struct SeraphicBridge {
    calibrator: SeraphicCalibrator,
    calibration_interval: usize,
    steps_since_calibration: usize,
}

impl SeraphicBridge {
    /// Create new bridge
    pub fn new(config: CalibratorConfig, calibration_interval: usize) -> Self {
        Self {
            calibrator: SeraphicCalibrator::new(config),
            calibration_interval,
            steps_since_calibration: 0,
        }
    }

    /// Initialize from pipeline
    pub fn initialize<P: GenerativePipeline>(&mut self, pipeline: &P) {
        let performance = pipeline.get_performance().to_3d();
        self.calibrator.initialize(qops_core::Configuration::default(), performance);
    }

    /// Check if calibration should run
    pub fn should_calibrate(&self) -> bool {
        self.steps_since_calibration >= self.calibration_interval
    }

    /// Run calibration and return feedback
    pub fn calibrate<P: GenerativePipeline>(&mut self, pipeline: &mut P) -> Signature {
        let result = self.calibrator.step();

        // Apply feedback to pipeline
        let feedback = Signature::D3(result.performance);
        pipeline.apply_calibration(&feedback);

        self.steps_since_calibration = 0;

        feedback
    }

    /// Record pipeline step
    pub fn record_step<P: GenerativePipeline>(&mut self, pipeline: &P) {
        self.steps_since_calibration += 1;

        if self.should_calibrate() {
            // Auto-calibrate if interval reached
            // In practice, caller would handle this
        }
    }

    /// Get current calibrator performance
    pub fn current_performance(&self) -> Signature3D {
        *self.calibrator.current_performance()
    }

    /// Get calibration history
    pub fn history_len(&self) -> usize {
        self.calibrator.history().len()
    }
}

impl Default for SeraphicBridge {
    fn default() -> Self {
        Self::new(CalibratorConfig::default(), 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qops_core::SimplePipeline;

    #[test]
    fn test_bridge_creation() {
        let bridge = SeraphicBridge::default();
        assert_eq!(bridge.calibration_interval, 10);
    }

    #[test]
    fn test_should_calibrate() {
        let mut bridge = SeraphicBridge::new(CalibratorConfig::default(), 5);

        assert!(!bridge.should_calibrate());

        bridge.steps_since_calibration = 5;
        assert!(bridge.should_calibrate());
    }
}
