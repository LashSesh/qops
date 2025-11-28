//! Resonance ledger for storing and verifying entries.
//!
//! The ledger provides:
//! - Hash-chained entry storage
//! - Proof-of-Resonance verification
//! - Query by signature properties
//! - Replay capability

use crate::signature::Signature;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Entry in the resonance ledger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// Unique identifier
    pub id: Uuid,
    /// Hash of previous entry (for chaining)
    pub prev_hash: String,
    /// Entry's own hash
    pub hash: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Signature at this entry
    pub signature: Signature,
    /// Resonance score
    pub resonance: f64,
    /// Entry type/category
    pub entry_type: String,
    /// Payload data (JSON)
    pub payload: String,
    /// Proof-of-Resonance data
    pub proof: Option<ProofOfResonance>,
}

impl LedgerEntry {
    /// Create a new ledger entry
    pub fn new(
        prev_hash: &str,
        signature: Signature,
        entry_type: &str,
        payload: &str,
    ) -> Self {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let resonance = signature.resonance();

        let mut entry = Self {
            id,
            prev_hash: prev_hash.to_string(),
            hash: String::new(),
            timestamp,
            signature,
            resonance,
            entry_type: entry_type.to_string(),
            payload: payload.to_string(),
            proof: None,
        };

        entry.hash = entry.compute_hash();
        entry
    }

    /// Compute the hash of this entry
    pub fn compute_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            self.id,
            self.prev_hash,
            self.timestamp.timestamp(),
            self.resonance,
            self.entry_type,
            self.payload
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verify the entry's hash
    pub fn verify_hash(&self) -> bool {
        self.hash == self.compute_hash()
    }

    /// Add proof-of-resonance
    pub fn add_proof(&mut self, proof: ProofOfResonance) {
        self.proof = Some(proof);
    }
}

/// Proof-of-Resonance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfResonance {
    /// Resonance threshold met
    pub threshold: f64,
    /// Achieved resonance
    pub achieved: f64,
    /// Stability over time
    pub stability: f64,
    /// Number of validation steps
    pub validation_steps: usize,
    /// Validator signature
    pub validator_hash: String,
    /// Timestamp of proof
    pub timestamp: DateTime<Utc>,
}

impl ProofOfResonance {
    /// Create a new proof
    pub fn new(threshold: f64, achieved: f64, stability: f64, validation_steps: usize) -> Self {
        let validator_hash = format!(
            "{:x}",
            Sha256::digest(format!("{}{}{}", achieved, stability, validation_steps).as_bytes())
        );

        Self {
            threshold,
            achieved,
            stability,
            validation_steps,
            validator_hash,
            timestamp: Utc::now(),
        }
    }

    /// Verify the proof
    pub fn verify(&self) -> bool {
        // Check that achieved resonance meets threshold
        if self.achieved < self.threshold {
            return false;
        }

        // Check stability is acceptable
        if self.stability < 0.5 {
            return false;
        }

        // Verify validator hash
        let expected_hash = format!(
            "{:x}",
            Sha256::digest(
                format!("{}{}{}", self.achieved, self.stability, self.validation_steps).as_bytes()
            )
        );

        self.validator_hash == expected_hash
    }
}

/// Trait for resonance ledger implementations
pub trait ResonanceLedger: Send + Sync {
    /// Append an entry to the ledger
    fn append(&mut self, entry: LedgerEntry) -> crate::error::Result<String>;

    /// Get an entry by hash
    fn get(&self, hash: &str) -> Option<&LedgerEntry>;

    /// Get the latest entry
    fn latest(&self) -> Option<&LedgerEntry>;

    /// Get the genesis (first) entry
    fn genesis(&self) -> Option<&LedgerEntry>;

    /// Get total number of entries
    fn len(&self) -> usize;

    /// Check if ledger is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Verify chain integrity
    fn verify_chain(&self) -> bool;

    /// Query entries by minimum resonance
    fn query_by_resonance(&self, min_resonance: f64) -> Vec<&LedgerEntry>;

    /// Query entries by type
    fn query_by_type(&self, entry_type: &str) -> Vec<&LedgerEntry>;
}

/// In-memory implementation of ResonanceLedger
#[derive(Debug, Clone, Default)]
pub struct MemoryLedger {
    entries: Vec<LedgerEntry>,
    index: std::collections::HashMap<String, usize>,
}

impl MemoryLedger {
    /// Create a new empty ledger
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with genesis entry
    pub fn with_genesis(signature: Signature) -> Self {
        let mut ledger = Self::new();
        let genesis = LedgerEntry::new(
            "0000000000000000000000000000000000000000000000000000000000000000",
            signature,
            "genesis",
            "{}",
        );
        let _ = ledger.append(genesis);
        ledger
    }

    /// Get all entries
    pub fn entries(&self) -> &[LedgerEntry] {
        &self.entries
    }

    /// Export to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.entries).unwrap_or_default()
    }

    /// Import from JSON
    pub fn from_json(json: &str) -> crate::error::Result<Self> {
        let entries: Vec<LedgerEntry> = serde_json::from_str(json)
            .map_err(|e| crate::error::QopsError::Serialization(e.to_string()))?;

        let mut ledger = Self::new();
        for entry in entries {
            let hash = entry.hash.clone();
            ledger.entries.push(entry);
            ledger.index.insert(hash, ledger.entries.len() - 1);
        }

        Ok(ledger)
    }
}

impl ResonanceLedger for MemoryLedger {
    fn append(&mut self, entry: LedgerEntry) -> crate::error::Result<String> {
        // Verify hash is correct
        if !entry.verify_hash() {
            return Err(crate::error::QopsError::Validation(
                "Invalid entry hash".to_string(),
            ));
        }

        // Verify chain linkage
        if !self.entries.is_empty() {
            let latest = self.entries.last().unwrap();
            if entry.prev_hash != latest.hash {
                return Err(crate::error::QopsError::Validation(
                    "Invalid chain linkage".to_string(),
                ));
            }
        }

        let hash = entry.hash.clone();
        self.index.insert(hash.clone(), self.entries.len());
        self.entries.push(entry);

        Ok(hash)
    }

    fn get(&self, hash: &str) -> Option<&LedgerEntry> {
        self.index.get(hash).map(|&idx| &self.entries[idx])
    }

    fn latest(&self) -> Option<&LedgerEntry> {
        self.entries.last()
    }

    fn genesis(&self) -> Option<&LedgerEntry> {
        self.entries.first()
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn verify_chain(&self) -> bool {
        if self.entries.is_empty() {
            return true;
        }

        for i in 1..self.entries.len() {
            let current = &self.entries[i];
            let previous = &self.entries[i - 1];

            // Verify hash
            if !current.verify_hash() {
                return false;
            }

            // Verify linkage
            if current.prev_hash != previous.hash {
                return false;
            }
        }

        true
    }

    fn query_by_resonance(&self, min_resonance: f64) -> Vec<&LedgerEntry> {
        self.entries
            .iter()
            .filter(|e| e.resonance >= min_resonance)
            .collect()
    }

    fn query_by_type(&self, entry_type: &str) -> Vec<&LedgerEntry> {
        self.entries
            .iter()
            .filter(|e| e.entry_type == entry_type)
            .collect()
    }
}

/// Ledger statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerStats {
    pub total_entries: usize,
    pub average_resonance: f64,
    pub max_resonance: f64,
    pub min_resonance: f64,
    pub entries_with_proof: usize,
    pub entry_types: std::collections::HashMap<String, usize>,
}

impl LedgerStats {
    /// Compute stats from a ledger
    pub fn from_ledger<L: ResonanceLedger>(ledger: &L) -> Self {
        // Note: Cannot compute detailed stats from trait alone
        // Use from_memory_ledger for concrete type with full access
        Self {
            total_entries: ledger.len(),
            average_resonance: 0.0,
            max_resonance: 0.0,
            min_resonance: 0.0,
            entries_with_proof: 0,
            entry_types: std::collections::HashMap::new(),
        }
    }

    /// Compute stats from memory ledger (concrete type)
    pub fn from_memory_ledger(ledger: &MemoryLedger) -> Self {
        let mut entry_types = std::collections::HashMap::new();
        let mut resonances = Vec::new();
        let mut entries_with_proof = 0;

        for entry in ledger.entries() {
            *entry_types.entry(entry.entry_type.clone()).or_insert(0) += 1;
            resonances.push(entry.resonance);
            if entry.proof.is_some() {
                entries_with_proof += 1;
            }
        }

        let average_resonance = if resonances.is_empty() {
            0.0
        } else {
            resonances.iter().sum::<f64>() / resonances.len() as f64
        };

        Self {
            total_entries: ledger.len(),
            average_resonance,
            max_resonance: resonances.iter().cloned().fold(0.0, f64::max),
            min_resonance: resonances.iter().cloned().fold(1.0, f64::min),
            entries_with_proof,
            entry_types,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signature::Signature3D;

    #[test]
    fn test_ledger_entry() {
        let sig = Signature::D3(Signature3D::new(0.8, 0.7, 0.6));
        let entry = LedgerEntry::new("prev", sig, "test", "{}");

        assert!(entry.verify_hash());
        assert!(entry.resonance > 0.0);
    }

    #[test]
    fn test_proof_of_resonance() {
        let proof = ProofOfResonance::new(0.7, 0.8, 0.9, 10);

        assert!(proof.verify());
        assert!(proof.achieved >= proof.threshold);
    }

    #[test]
    fn test_memory_ledger() {
        let sig = Signature::D3(Signature3D::new(0.8, 0.7, 0.6));
        let mut ledger = MemoryLedger::with_genesis(sig);

        assert_eq!(ledger.len(), 1);
        assert!(ledger.verify_chain());

        // Add another entry
        let prev_hash = ledger.latest().unwrap().hash.clone();
        let sig2 = Signature::D3(Signature3D::new(0.9, 0.8, 0.7));
        let entry2 = LedgerEntry::new(&prev_hash, sig2, "operator", "{}");
        ledger.append(entry2).unwrap();

        assert_eq!(ledger.len(), 2);
        assert!(ledger.verify_chain());
    }

    #[test]
    fn test_ledger_query() {
        let mut ledger = MemoryLedger::new();

        // Add genesis
        let sig1 = Signature::D3(Signature3D::new(0.5, 0.5, 0.5));
        let genesis = LedgerEntry::new("0", sig1, "genesis", "{}");
        let prev = ledger.append(genesis).unwrap();

        // Add high resonance entry
        let sig2 = Signature::D3(Signature3D::new(0.9, 0.9, 0.9));
        let entry = LedgerEntry::new(&prev, sig2, "operator", "{}");
        ledger.append(entry).unwrap();

        let high_res = ledger.query_by_resonance(0.8);
        assert!(!high_res.is_empty());
    }
}
