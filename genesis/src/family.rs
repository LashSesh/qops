//! Operator Family extraction and management.
//!
//! Families group similar operators based on signature proximity.

use crate::artefact::Artefact;
use qops_core::Signature5D;
use serde::{Deserialize, Serialize};

/// An operator family - a group of similar artefacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorFamily {
    /// Family ID
    pub id: String,
    /// Family name (auto-generated or custom)
    pub name: String,
    /// Members of this family
    members: Vec<Artefact>,
    /// Centroid signature
    centroid: Option<Signature5D>,
    /// Family characteristics
    pub characteristics: FamilyCharacteristics,
}

/// Characteristics of an operator family
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FamilyCharacteristics {
    /// Dominant quality dimension
    pub dominant_dimension: Option<String>,
    /// Average psi
    pub avg_psi: f64,
    /// Average rho
    pub avg_rho: f64,
    /// Average omega
    pub avg_omega: f64,
    /// Average chi
    pub avg_chi: f64,
    /// Average eta
    pub avg_eta: f64,
    /// Spread (variance) in signature space
    pub spread: f64,
    /// Coherence (1 - spread)
    pub coherence: f64,
}

impl OperatorFamily {
    /// Create new empty family
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            members: Vec::new(),
            centroid: None,
            characteristics: FamilyCharacteristics::default(),
        }
    }

    /// Create family from initial artefact
    pub fn from_artefact(artefact: Artefact) -> Self {
        let mut family = Self::new();
        family.add_member(artefact);
        family
    }

    /// Add member to family
    pub fn add_member(&mut self, artefact: Artefact) {
        self.members.push(artefact);
        self.update_centroid();
        self.update_characteristics();
        self.update_name();
    }

    /// Check if signature is similar to family
    pub fn is_similar(&self, sig: &Signature5D, threshold: f64) -> bool {
        if let Some(centroid) = &self.centroid {
            Self::signature_distance(sig, centroid) < threshold
        } else {
            true // Empty family accepts anything
        }
    }

    /// Compute distance between two signatures
    fn signature_distance(a: &Signature5D, b: &Signature5D) -> f64 {
        ((a.psi - b.psi).powi(2) +
         (a.rho - b.rho).powi(2) +
         (a.omega - b.omega).powi(2) +
         (a.chi - b.chi).powi(2) +
         (a.eta - b.eta).powi(2)).sqrt()
    }

    /// Update centroid from members
    fn update_centroid(&mut self) {
        if self.members.is_empty() {
            self.centroid = None;
            return;
        }

        let n = self.members.len() as f64;
        let mut sum = Signature5D::new(0.0, 0.0, 0.0, 0.0, 0.0);

        for member in &self.members {
            sum.psi += member.signature.psi;
            sum.rho += member.signature.rho;
            sum.omega += member.signature.omega;
            sum.chi += member.signature.chi;
            sum.eta += member.signature.eta;
        }

        self.centroid = Some(Signature5D::new(
            sum.psi / n,
            sum.rho / n,
            sum.omega / n,
            sum.chi / n,
            sum.eta / n,
        ));
    }

    /// Update family characteristics
    fn update_characteristics(&mut self) {
        if self.members.is_empty() {
            self.characteristics = FamilyCharacteristics::default();
            return;
        }

        let n = self.members.len() as f64;

        // Compute averages
        let avg_psi: f64 = self.members.iter().map(|m| m.signature.psi).sum::<f64>() / n;
        let avg_rho: f64 = self.members.iter().map(|m| m.signature.rho).sum::<f64>() / n;
        let avg_omega: f64 = self.members.iter().map(|m| m.signature.omega).sum::<f64>() / n;
        let avg_chi: f64 = self.members.iter().map(|m| m.signature.chi).sum::<f64>() / n;
        let avg_eta: f64 = self.members.iter().map(|m| m.signature.eta).sum::<f64>() / n;

        // Determine dominant dimension
        let dims = [
            ("psi", avg_psi),
            ("rho", avg_rho),
            ("omega", avg_omega),
            ("chi", avg_chi),
        ];
        let dominant = dims.iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(name, _)| name.to_string());

        // Compute spread (average distance from centroid)
        let spread = if let Some(centroid) = &self.centroid {
            self.members.iter()
                .map(|m| Self::signature_distance(&m.signature, centroid))
                .sum::<f64>() / n
        } else {
            0.0
        };

        self.characteristics = FamilyCharacteristics {
            dominant_dimension: dominant,
            avg_psi,
            avg_rho,
            avg_omega,
            avg_chi,
            avg_eta,
            spread,
            coherence: 1.0 - spread.min(1.0),
        };
    }

    /// Update family name based on characteristics
    fn update_name(&mut self) {
        let prefix = match self.characteristics.dominant_dimension.as_deref() {
            Some("psi") => "Quality",
            Some("rho") => "Stability",
            Some("omega") => "Efficiency",
            Some("chi") => "Topological",
            _ => "Mixed",
        };

        let quality = if self.avg_resonance() >= 0.85 {
            "Elite"
        } else if self.avg_resonance() >= 0.7 {
            "Strong"
        } else if self.avg_resonance() >= 0.5 {
            "Standard"
        } else {
            "Weak"
        };

        self.name = format!("{}-{}-{}", prefix, quality, &self.id[..8]);
    }

    /// Get family members
    pub fn members(&self) -> &[Artefact] {
        &self.members
    }

    /// Get member count
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Get centroid
    pub fn centroid(&self) -> Option<&Signature5D> {
        self.centroid.as_ref()
    }

    /// Get average resonance
    pub fn avg_resonance(&self) -> f64 {
        if self.members.is_empty() {
            return 0.0;
        }
        self.members.iter().map(|m| m.resonance).sum::<f64>() / self.members.len() as f64
    }

    /// Get best member
    pub fn best(&self) -> Option<&Artefact> {
        self.members.iter()
            .max_by(|a, b| a.resonance.partial_cmp(&b.resonance).unwrap())
    }

    /// Get mandorla members
    pub fn mandorlas(&self) -> Vec<&Artefact> {
        self.members.iter()
            .filter(|m| m.is_mandorla())
            .collect()
    }

    /// Check if family is coherent (low spread)
    pub fn is_coherent(&self, threshold: f64) -> bool {
        self.characteristics.spread < threshold
    }

    /// Merge another family into this one
    pub fn merge(&mut self, other: &OperatorFamily) {
        for member in &other.members {
            self.members.push(member.clone());
        }
        self.update_centroid();
        self.update_characteristics();
        self.update_name();
    }
}

impl Default for OperatorFamily {
    fn default() -> Self {
        Self::new()
    }
}

/// Family clustering algorithm
pub struct FamilyClusterer {
    /// Distance threshold for grouping
    pub threshold: f64,
    /// Maximum families to create
    pub max_families: usize,
}

impl FamilyClusterer {
    /// Create new clusterer
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            max_families: 20,
        }
    }

    /// Cluster artefacts into families
    pub fn cluster(&self, artefacts: &[Artefact]) -> Vec<OperatorFamily> {
        if artefacts.is_empty() {
            return Vec::new();
        }

        let mut families: Vec<OperatorFamily> = Vec::new();

        for artefact in artefacts {
            let mut assigned = false;

            // Try to find a matching family
            for family in &mut families {
                if family.is_similar(&artefact.signature, self.threshold) {
                    family.add_member(artefact.clone());
                    assigned = true;
                    break;
                }
            }

            // Create new family if no match found
            if !assigned && families.len() < self.max_families {
                families.push(OperatorFamily::from_artefact(artefact.clone()));
            } else if !assigned {
                // Add to most similar family if at max
                if let Some(closest) = families.iter_mut()
                    .min_by(|a, b| {
                        let dist_a = a.centroid()
                            .map(|c| Self::distance(&artefact.signature, c))
                            .unwrap_or(f64::MAX);
                        let dist_b = b.centroid()
                            .map(|c| Self::distance(&artefact.signature, c))
                            .unwrap_or(f64::MAX);
                        dist_a.partial_cmp(&dist_b).unwrap()
                    })
                {
                    closest.add_member(artefact.clone());
                }
            }
        }

        // Sort by average resonance (descending)
        families.sort_by(|a, b| {
            b.avg_resonance().partial_cmp(&a.avg_resonance()).unwrap()
        });

        families
    }

    fn distance(a: &Signature5D, b: &Signature5D) -> f64 {
        ((a.psi - b.psi).powi(2) +
         (a.rho - b.rho).powi(2) +
         (a.omega - b.omega).powi(2) +
         (a.chi - b.chi).powi(2) +
         (a.eta - b.eta).powi(2)).sqrt()
    }
}

impl Default for FamilyClusterer {
    fn default() -> Self {
        Self::new(0.15)
    }
}

/// Family evaluation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMetrics {
    /// Family ID
    pub family_id: String,
    /// Family name
    pub family_name: String,
    /// Size
    pub size: usize,
    /// Average resonance
    pub avg_resonance: f64,
    /// Best resonance
    pub best_resonance: f64,
    /// Coherence
    pub coherence: f64,
    /// Mandorla rate
    pub mandorla_rate: f64,
    /// Quality score (composite)
    pub quality_score: f64,
}

impl FamilyMetrics {
    /// Compute metrics for a family
    pub fn from_family(family: &OperatorFamily) -> Self {
        let size = family.size();
        let avg_resonance = family.avg_resonance();
        let best_resonance = family.best().map(|m| m.resonance).unwrap_or(0.0);
        let coherence = family.characteristics.coherence;
        let mandorla_rate = if size > 0 {
            family.mandorlas().len() as f64 / size as f64
        } else {
            0.0
        };

        // Quality score combines resonance, coherence, and mandorla rate
        let quality_score = 0.5 * avg_resonance + 0.3 * coherence + 0.2 * mandorla_rate;

        Self {
            family_id: family.id.clone(),
            family_name: family.name.clone(),
            size,
            avg_resonance,
            best_resonance,
            coherence,
            mandorla_rate,
            quality_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::NodeIndex;

    #[test]
    fn test_operator_family() {
        let mut family = OperatorFamily::new();

        let sig = Signature5D::new(0.8, 0.7, 0.6, 0.5, 0.2);
        let artefact = Artefact::from_signature(sig);
        family.add_member(artefact);

        assert_eq!(family.size(), 1);
        assert!(family.centroid().is_some());
    }

    #[test]
    fn test_family_similarity() {
        let mut family = OperatorFamily::new();

        let sig1 = Signature5D::new(0.8, 0.7, 0.6, 0.5, 0.2);
        family.add_member(Artefact::from_signature(sig1));

        let sig2 = Signature5D::new(0.81, 0.71, 0.61, 0.51, 0.21);
        assert!(family.is_similar(&sig2, 0.1));

        let sig3 = Signature5D::new(0.3, 0.3, 0.3, 0.3, 0.3);
        assert!(!family.is_similar(&sig3, 0.1));
    }

    #[test]
    fn test_family_clusterer() {
        let clusterer = FamilyClusterer::new(0.2);

        let artefacts: Vec<Artefact> = (0..10)
            .map(|i| {
                let base = 0.5 + (i as f64 * 0.02);
                Artefact::from_signature(Signature5D::new(base, base, base, base, 0.2))
            })
            .collect();

        let families = clusterer.cluster(&artefacts);
        assert!(!families.is_empty());
    }
}
