//! Proof-of-Resonance validator.

use qops_core::{MandorlaField, Signature3D};
use serde::{Deserialize, Serialize};

/// PoR check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoRResult {
    pub accepted: bool,
    pub score: f64,
    pub quality_improved: bool,
    pub stability_maintained: bool,
    pub efficiency_acceptable: bool,
}

/// Proof-of-Resonance validator
pub struct ProofOfResonanceValidator {
    threshold: f64,
    quality_weight: f64,
    stability_weight: f64,
    efficiency_weight: f64,
}

impl ProofOfResonanceValidator {
    /// Create new validator
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            quality_weight: 0.4,
            stability_weight: 0.3,
            efficiency_weight: 0.3,
        }
    }

    /// Check if candidate should be accepted
    pub fn check(
        &self,
        current: &Signature3D,
        candidate: &Signature3D,
        _field: &MandorlaField,
    ) -> PoRResult {
        // Quality improvement check
        let quality_improved = candidate.psi >= current.psi;

        // Stability check (shouldn't decrease significantly)
        let stability_maintained = candidate.rho >= current.rho * 0.95;

        // Efficiency check
        let efficiency_acceptable = candidate.omega >= current.omega * 0.9;

        // Compute score
        let current_score = self.quality_weight * current.psi
            + self.stability_weight * current.rho
            + self.efficiency_weight * current.omega;

        let candidate_score = self.quality_weight * candidate.psi
            + self.stability_weight * candidate.rho
            + self.efficiency_weight * candidate.omega;

        // Accept if score improved or above threshold
        let accepted = candidate_score >= current_score
            || candidate_score >= self.threshold;

        PoRResult {
            accepted,
            score: candidate_score,
            quality_improved,
            stability_maintained,
            efficiency_acceptable,
        }
    }
}

impl Default for ProofOfResonanceValidator {
    fn default() -> Self {
        Self::new(0.7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_por_accept_improvement() {
        let por = ProofOfResonanceValidator::default();
        let field = MandorlaField::default();

        let current = Signature3D::new(0.5, 0.5, 0.5);
        let candidate = Signature3D::new(0.6, 0.55, 0.52);

        let result = por.check(&current, &candidate, &field);
        assert!(result.accepted);
        assert!(result.quality_improved);
    }

    #[test]
    fn test_por_reject_degradation() {
        let por = ProofOfResonanceValidator::default();
        let field = MandorlaField::default();

        let current = Signature3D::new(0.8, 0.8, 0.8);
        let candidate = Signature3D::new(0.5, 0.5, 0.5);

        let result = por.check(&current, &candidate, &field);
        assert!(!result.accepted);
    }
}
