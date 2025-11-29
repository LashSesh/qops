//! Ledger for recording irreversible transformations.
//!
//! Per Section 7.1 (L7), the Ledger/Governance Layer records irreversible
//! transformations B → A, including roles, approvals, and parameters.

use crate::error::{KernelError, Result};
use crate::state::CoreSignature;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use uuid::Uuid;

/// Ledger record representing a single transformation entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerRecord {
    /// Unique record ID
    pub id: String,
    /// Sequence number
    pub sequence: u64,
    /// Transformation entry
    pub entry: TransformationEntry,
    /// Hash of this record
    pub hash: String,
    /// Hash of previous record (chain)
    pub prev_hash: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Transformation entry details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationEntry {
    /// Source blueprint ID
    pub blueprint_id: String,
    /// Generated artefact ID
    pub artefact_id: String,
    /// Transformation timestamp
    pub timestamp: DateTime<Utc>,
    /// Parameters used in transformation
    pub parameters: HashMap<String, serde_json::Value>,
    /// Final resonance score
    pub resonance_score: f64,
    /// Constraints that were applied
    pub constraints: Vec<String>,
}

/// Trait for ledger implementations
pub trait KernelLedger: Send + Sync {
    /// Record a transformation
    fn record_transformation(&mut self, entry: TransformationEntry) -> Result<LedgerRecord>;

    /// Get record by ID
    fn get_record(&self, id: &str) -> Result<Option<LedgerRecord>>;

    /// Get all records for a blueprint
    fn get_blueprint_records(&self, blueprint_id: &str) -> Result<Vec<LedgerRecord>>;

    /// Get all records for an artefact
    fn get_artefact_records(&self, artefact_id: &str) -> Result<Vec<LedgerRecord>>;

    /// Get the latest record
    fn get_latest(&self) -> Result<Option<LedgerRecord>>;

    /// Get total record count
    fn count(&self) -> Result<u64>;

    /// Verify chain integrity
    fn verify_integrity(&self) -> Result<bool>;

    /// Export ledger to JSON
    fn export(&self) -> Result<serde_json::Value>;
}

/// In-memory ledger implementation
#[derive(Debug, Clone, Default)]
pub struct MemoryLedger {
    /// Records stored in memory
    records: Vec<LedgerRecord>,
    /// Index by blueprint ID
    blueprint_index: HashMap<String, Vec<usize>>,
    /// Index by artefact ID
    artefact_index: HashMap<String, Vec<usize>>,
}

impl MemoryLedger {
    /// Create a new memory ledger
    pub fn new() -> Self {
        Self::default()
    }

    /// Compute hash for a record
    fn compute_hash(entry: &TransformationEntry, prev_hash: Option<&str>, sequence: u64) -> String {
        let mut hasher = Sha256::new();

        hasher.update(entry.blueprint_id.as_bytes());
        hasher.update(entry.artefact_id.as_bytes());
        hasher.update(entry.timestamp.to_rfc3339().as_bytes());
        hasher.update(entry.resonance_score.to_be_bytes());
        hasher.update(sequence.to_be_bytes());

        if let Some(prev) = prev_hash {
            hasher.update(prev.as_bytes());
        }

        format!("{:x}", hasher.finalize())
    }
}

impl KernelLedger for MemoryLedger {
    fn record_transformation(&mut self, entry: TransformationEntry) -> Result<LedgerRecord> {
        let sequence = self.records.len() as u64;
        let prev_hash = self.records.last().map(|r| r.hash.clone());
        let hash = Self::compute_hash(&entry, prev_hash.as_deref(), sequence);

        let record = LedgerRecord {
            id: Uuid::new_v4().to_string(),
            sequence,
            entry: entry.clone(),
            hash,
            prev_hash,
            timestamp: Utc::now(),
        };

        let idx = self.records.len();

        // Update indices
        self.blueprint_index
            .entry(entry.blueprint_id.clone())
            .or_default()
            .push(idx);
        self.artefact_index
            .entry(entry.artefact_id.clone())
            .or_default()
            .push(idx);

        self.records.push(record.clone());

        Ok(record)
    }

    fn get_record(&self, id: &str) -> Result<Option<LedgerRecord>> {
        Ok(self.records.iter().find(|r| r.id == id).cloned())
    }

    fn get_blueprint_records(&self, blueprint_id: &str) -> Result<Vec<LedgerRecord>> {
        let indices = self.blueprint_index.get(blueprint_id);
        match indices {
            Some(idxs) => Ok(idxs.iter().map(|&i| self.records[i].clone()).collect()),
            None => Ok(Vec::new()),
        }
    }

    fn get_artefact_records(&self, artefact_id: &str) -> Result<Vec<LedgerRecord>> {
        let indices = self.artefact_index.get(artefact_id);
        match indices {
            Some(idxs) => Ok(idxs.iter().map(|&i| self.records[i].clone()).collect()),
            None => Ok(Vec::new()),
        }
    }

    fn get_latest(&self) -> Result<Option<LedgerRecord>> {
        Ok(self.records.last().cloned())
    }

    fn count(&self) -> Result<u64> {
        Ok(self.records.len() as u64)
    }

    fn verify_integrity(&self) -> Result<bool> {
        if self.records.is_empty() {
            return Ok(true);
        }

        for (i, record) in self.records.iter().enumerate() {
            let expected_prev = if i > 0 {
                Some(self.records[i - 1].hash.as_str())
            } else {
                None
            };

            if record.prev_hash.as_deref() != expected_prev {
                return Ok(false);
            }

            let computed_hash =
                Self::compute_hash(&record.entry, record.prev_hash.as_deref(), record.sequence);

            if computed_hash != record.hash {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn export(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(&self.records)?)
    }
}

/// File-based ledger implementation
pub struct FileLedger {
    /// Path to ledger file
    path: PathBuf,
    /// In-memory cache
    cache: MemoryLedger,
}

impl FileLedger {
    /// Create or open a file ledger
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut ledger = Self {
            path: path.clone(),
            cache: MemoryLedger::new(),
        };

        // Load existing records if file exists
        if path.exists() {
            ledger.load()?;
        }

        Ok(ledger)
    }

    /// Load records from file
    fn load(&mut self) -> Result<()> {
        let file = fs::File::open(&self.path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                let record: LedgerRecord = serde_json::from_str(&line)?;

                // Update indices
                let idx = self.cache.records.len();
                self.cache
                    .blueprint_index
                    .entry(record.entry.blueprint_id.clone())
                    .or_default()
                    .push(idx);
                self.cache
                    .artefact_index
                    .entry(record.entry.artefact_id.clone())
                    .or_default()
                    .push(idx);

                self.cache.records.push(record);
            }
        }

        Ok(())
    }

    /// Append a record to file
    fn append(&self, record: &LedgerRecord) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let json = serde_json::to_string(record)?;
        writeln!(file, "{}", json)?;

        Ok(())
    }
}

impl KernelLedger for FileLedger {
    fn record_transformation(&mut self, entry: TransformationEntry) -> Result<LedgerRecord> {
        let record = self.cache.record_transformation(entry)?;
        self.append(&record)?;
        Ok(record)
    }

    fn get_record(&self, id: &str) -> Result<Option<LedgerRecord>> {
        self.cache.get_record(id)
    }

    fn get_blueprint_records(&self, blueprint_id: &str) -> Result<Vec<LedgerRecord>> {
        self.cache.get_blueprint_records(blueprint_id)
    }

    fn get_artefact_records(&self, artefact_id: &str) -> Result<Vec<LedgerRecord>> {
        self.cache.get_artefact_records(artefact_id)
    }

    fn get_latest(&self) -> Result<Option<LedgerRecord>> {
        self.cache.get_latest()
    }

    fn count(&self) -> Result<u64> {
        self.cache.count()
    }

    fn verify_integrity(&self) -> Result<bool> {
        self.cache.verify_integrity()
    }

    fn export(&self) -> Result<serde_json::Value> {
        self.cache.export()
    }
}

/// Ledger query builder
pub struct LedgerQuery<'a> {
    ledger: &'a dyn KernelLedger,
    blueprint_filter: Option<String>,
    artefact_filter: Option<String>,
    min_resonance: Option<f64>,
    max_resonance: Option<f64>,
    from_time: Option<DateTime<Utc>>,
    to_time: Option<DateTime<Utc>>,
    limit: Option<usize>,
}

impl<'a> LedgerQuery<'a> {
    /// Create a new query
    pub fn new(ledger: &'a dyn KernelLedger) -> Self {
        Self {
            ledger,
            blueprint_filter: None,
            artefact_filter: None,
            min_resonance: None,
            max_resonance: None,
            from_time: None,
            to_time: None,
            limit: None,
        }
    }

    /// Filter by blueprint ID
    pub fn blueprint(mut self, id: &str) -> Self {
        self.blueprint_filter = Some(id.to_string());
        self
    }

    /// Filter by artefact ID
    pub fn artefact(mut self, id: &str) -> Self {
        self.artefact_filter = Some(id.to_string());
        self
    }

    /// Filter by minimum resonance
    pub fn min_resonance(mut self, min: f64) -> Self {
        self.min_resonance = Some(min);
        self
    }

    /// Filter by maximum resonance
    pub fn max_resonance(mut self, max: f64) -> Self {
        self.max_resonance = Some(max);
        self
    }

    /// Limit results
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Execute query
    pub fn execute(&self) -> Result<Vec<LedgerRecord>> {
        // Start with all records or filtered by blueprint/artefact
        let records = if let Some(bp_id) = &self.blueprint_filter {
            self.ledger.get_blueprint_records(bp_id)?
        } else if let Some(art_id) = &self.artefact_filter {
            self.ledger.get_artefact_records(art_id)?
        } else {
            // Get all records (inefficient for file ledger, but works)
            let export = self.ledger.export()?;
            serde_json::from_value(export)?
        };

        // Apply filters
        let filtered: Vec<LedgerRecord> = records
            .into_iter()
            .filter(|r| {
                if let Some(min) = self.min_resonance {
                    if r.entry.resonance_score < min {
                        return false;
                    }
                }
                if let Some(max) = self.max_resonance {
                    if r.entry.resonance_score > max {
                        return false;
                    }
                }
                if let Some(from) = self.from_time {
                    if r.entry.timestamp < from {
                        return false;
                    }
                }
                if let Some(to) = self.to_time {
                    if r.entry.timestamp > to {
                        return false;
                    }
                }
                true
            })
            .collect();

        // Apply limit
        if let Some(n) = self.limit {
            Ok(filtered.into_iter().take(n).collect())
        } else {
            Ok(filtered)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_memory_ledger() {
        let mut ledger = MemoryLedger::new();

        let entry = TransformationEntry {
            blueprint_id: "bp_1".to_string(),
            artefact_id: "art_1".to_string(),
            timestamp: Utc::now(),
            parameters: HashMap::new(),
            resonance_score: 0.8,
            constraints: vec!["min_resonance".to_string()],
        };

        let record = ledger.record_transformation(entry).unwrap();
        assert!(!record.id.is_empty());
        assert_eq!(record.sequence, 0);

        let retrieved = ledger.get_record(&record.id).unwrap();
        assert!(retrieved.is_some());

        assert!(ledger.verify_integrity().unwrap());
    }

    #[test]
    fn test_ledger_chain() {
        let mut ledger = MemoryLedger::new();

        // Add multiple records
        for i in 0..5 {
            let entry = TransformationEntry {
                blueprint_id: format!("bp_{}", i),
                artefact_id: format!("art_{}", i),
                timestamp: Utc::now(),
                parameters: HashMap::new(),
                resonance_score: 0.5 + (i as f64) * 0.1,
                constraints: Vec::new(),
            };
            ledger.record_transformation(entry).unwrap();
        }

        assert_eq!(ledger.count().unwrap(), 5);
        assert!(ledger.verify_integrity().unwrap());

        // Check chain
        let latest = ledger.get_latest().unwrap().unwrap();
        assert_eq!(latest.sequence, 4);
        assert!(latest.prev_hash.is_some());
    }

    #[test]
    fn test_file_ledger() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("ledger.jsonl");

        {
            let mut ledger = FileLedger::new(path.clone()).unwrap();

            let entry = TransformationEntry {
                blueprint_id: "bp_file".to_string(),
                artefact_id: "art_file".to_string(),
                timestamp: Utc::now(),
                parameters: HashMap::new(),
                resonance_score: 0.75,
                constraints: Vec::new(),
            };

            ledger.record_transformation(entry).unwrap();
        }

        // Reload and verify
        let ledger = FileLedger::new(path).unwrap();
        assert_eq!(ledger.count().unwrap(), 1);
        assert!(ledger.verify_integrity().unwrap());
    }

    #[test]
    fn test_ledger_query() {
        let mut ledger = MemoryLedger::new();

        for i in 0..10 {
            let entry = TransformationEntry {
                blueprint_id: format!("bp_{}", i % 3),
                artefact_id: format!("art_{}", i),
                timestamp: Utc::now(),
                parameters: HashMap::new(),
                resonance_score: 0.3 + (i as f64) * 0.07,
                constraints: Vec::new(),
            };
            ledger.record_transformation(entry).unwrap();
        }

        // Query by blueprint
        let query = LedgerQuery::new(&ledger).blueprint("bp_0");
        let results = query.execute().unwrap();
        assert!(!results.is_empty());

        // Query by resonance
        let query = LedgerQuery::new(&ledger).min_resonance(0.7);
        let results = query.execute().unwrap();
        for r in &results {
            assert!(r.entry.resonance_score >= 0.7);
        }
    }
}
