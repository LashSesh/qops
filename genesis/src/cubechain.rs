//! Cubechain - Hypercube-DAG Ledger with Proof-of-Resonance.

use crate::artefact::Artefact;
use qops_core::{MemoryLedger, LedgerEntry, ResonanceLedger, Signature, ProofOfResonance};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Cubechain ledger for operator storage
#[derive(Debug, Clone)]
pub struct Cubechain {
    /// Underlying ledger
    ledger: MemoryLedger,
    /// Artefact index
    artefacts: std::collections::HashMap<Uuid, Artefact>,
    /// DAG parents (for hypercube structure)
    parents: std::collections::HashMap<Uuid, Vec<Uuid>>,
    /// Current epoch
    epoch: usize,
}

impl Cubechain {
    /// Create a new Cubechain
    pub fn new() -> Self {
        let ledger = MemoryLedger::with_genesis(Signature::default());

        Self {
            ledger,
            artefacts: std::collections::HashMap::new(),
            parents: std::collections::HashMap::new(),
            epoch: 0,
        }
    }

    /// Add an artefact to the chain
    pub fn add_artefact(&mut self, artefact: Artefact, parent_ids: Vec<Uuid>) -> qops_core::Result<String> {
        // Create ledger entry
        let prev_hash = self.ledger.latest().map(|e| e.hash.clone()).unwrap_or_default();
        let payload = serde_json::to_string(&artefact).unwrap_or_default();

        let mut entry = LedgerEntry::new(
            &prev_hash,
            Signature::D5(artefact.signature),
            "artefact",
            &payload,
        );

        // Add Proof-of-Resonance if artefact is certified
        if artefact.is_mandorla {
            let proof = ProofOfResonance::new(
                0.85,
                artefact.resonance,
                artefact.stability,
                artefact.blueprint_len(),
            );
            entry.add_proof(proof);
        }

        // Store artefact
        let artefact_id = artefact.id;
        self.artefacts.insert(artefact_id, artefact);
        self.parents.insert(artefact_id, parent_ids);

        // Append to ledger
        self.ledger.append(entry)
    }

    /// Get an artefact by ID
    pub fn get_artefact(&self, id: &Uuid) -> Option<&Artefact> {
        self.artefacts.get(id)
    }

    /// Get all Mandorla-certified artefacts
    pub fn mandorla_artefacts(&self) -> Vec<&Artefact> {
        self.artefacts.values().filter(|a| a.is_mandorla).collect()
    }

    /// Get artefacts with minimum resonance
    pub fn query_by_resonance(&self, min_resonance: f64) -> Vec<&Artefact> {
        self.artefacts
            .values()
            .filter(|a| a.resonance >= min_resonance)
            .collect()
    }

    /// Get parent artefacts (DAG structure)
    pub fn get_parents(&self, id: &Uuid) -> Vec<&Artefact> {
        self.parents
            .get(id)
            .map(|parent_ids| {
                parent_ids
                    .iter()
                    .filter_map(|pid| self.artefacts.get(pid))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get chain length
    pub fn len(&self) -> usize {
        self.ledger.len()
    }

    /// Check if chain is empty
    pub fn is_empty(&self) -> bool {
        self.ledger.is_empty()
    }

    /// Verify chain integrity
    pub fn verify(&self) -> bool {
        self.ledger.verify_chain()
    }

    /// Advance epoch
    pub fn advance_epoch(&mut self) {
        self.epoch += 1;
    }

    /// Get current epoch
    pub fn current_epoch(&self) -> usize {
        self.epoch
    }

    /// Get statistics
    pub fn stats(&self) -> CubechainStats {
        let total = self.artefacts.len();
        let mandorla = self.artefacts.values().filter(|a| a.is_mandorla).count();
        let avg_resonance = if total > 0 {
            self.artefacts.values().map(|a| a.resonance).sum::<f64>() / total as f64
        } else {
            0.0
        };

        CubechainStats {
            total_artefacts: total,
            mandorla_count: mandorla,
            average_resonance: avg_resonance,
            ledger_entries: self.ledger.len(),
            current_epoch: self.epoch,
        }
    }
}

impl Default for Cubechain {
    fn default() -> Self {
        Self::new()
    }
}

/// Cubechain statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubechainStats {
    pub total_artefacts: usize,
    pub mandorla_count: usize,
    pub average_resonance: f64,
    pub ledger_entries: usize,
    pub current_epoch: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use qops_core::Signature5D;
    use petgraph::graph::NodeIndex;

    #[test]
    fn test_cubechain_creation() {
        let chain = Cubechain::new();
        assert!(chain.verify());
    }

    #[test]
    fn test_add_artefact() {
        let mut chain = Cubechain::new();

        let sig = Signature5D::new(0.9, 0.85, 0.8, 0.7, 0.3);
        let mut artefact = Artefact::new(NodeIndex::new(0), sig);
        artefact.stability = 0.9;
        artefact.check_mandorla(0.85);

        let result = chain.add_artefact(artefact, vec![]);
        assert!(result.is_ok());

        let stats = chain.stats();
        assert_eq!(stats.total_artefacts, 1);
        assert_eq!(stats.mandorla_count, 1);
    }
}
