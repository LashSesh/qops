//! Slots Engine Session
//!
//! Session management for the QSlots engine.

use crate::lattice::{SlotLattice, LatticeResult};
use crate::miner::{SequenceMiner, MinerConfig, MiningResult, MinedSequence};
use crate::entropy::{EntropyMapper, EntropyConfig};
use crate::topology::{SlotTopology, TopologyType};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotsSessionConfig {
    /// Session name
    pub name: String,
    /// Topology to use
    pub topology: TopologyType,
    /// Miner configuration
    pub miner_config: MinerConfig,
    /// Entropy configuration
    pub entropy_config: EntropyConfig,
    /// Auto-mine on spin
    pub auto_mine: bool,
    /// Number of spins before mining
    pub spins_before_mine: usize,
    /// Target resonance
    pub target_resonance: f64,
}

impl Default for SlotsSessionConfig {
    fn default() -> Self {
        Self {
            name: "slots_session".to_string(),
            topology: TopologyType::Classic,
            miner_config: MinerConfig::default(),
            entropy_config: EntropyConfig::default(),
            auto_mine: true,
            spins_before_mine: 10,
            target_resonance: 0.8,
        }
    }
}

impl SlotsSessionConfig {
    /// Quick session
    pub fn quick() -> Self {
        Self {
            name: "quick_slots".to_string(),
            miner_config: MinerConfig::quick(),
            spins_before_mine: 5,
            ..Default::default()
        }
    }

    /// Research session
    pub fn research() -> Self {
        Self {
            name: "research_slots".to_string(),
            topology: TopologyType::Pentagonal,
            miner_config: MinerConfig::thorough(),
            entropy_config: EntropyConfig::resonance_optimized(),
            spins_before_mine: 20,
            ..Default::default()
        }
    }
}

/// Session state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    /// Session created
    Created,
    /// Spinning
    Spinning,
    /// Mining
    Mining,
    /// Complete
    Completed,
    /// Error
    Error,
}

/// Session result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotsSessionResult {
    /// Session ID
    pub session_id: String,
    /// Final state
    pub state: SessionState,
    /// Number of spins performed
    pub spin_count: usize,
    /// Spin results
    pub spin_results: Vec<LatticeResult>,
    /// Mining result (if performed)
    pub mining_result: Option<MiningResult>,
    /// Best sequence found
    pub best_sequence: Option<MinedSequence>,
    /// Best resonance
    pub best_resonance: f64,
    /// Total session time (ms)
    pub total_time_ms: u64,
}

/// Slots engine session
pub struct SlotsSession {
    /// Session ID
    pub id: String,
    /// Configuration
    pub config: SlotsSessionConfig,
    /// Current state
    pub state: SessionState,
    /// The lattice
    lattice: SlotLattice,
    /// Entropy mapper
    entropy_mapper: EntropyMapper,
    /// Miner
    miner: SequenceMiner,
    /// Spin results
    spin_results: Vec<LatticeResult>,
    /// Mining result
    mining_result: Option<MiningResult>,
    /// Best sequence
    best_sequence: Option<MinedSequence>,
    /// Start time
    start_time: std::time::Instant,
}

impl SlotsSession {
    /// Create a new session
    pub fn new(config: SlotsSessionConfig) -> Self {
        let topology = SlotTopology::new(config.topology);
        let lattice_config = topology.to_lattice_config();
        let lattice = SlotLattice::new(lattice_config);

        let entropy_mapper = EntropyMapper::new(config.entropy_config.clone());
        let miner = SequenceMiner::new(config.miner_config.clone());

        Self {
            id: Uuid::new_v4().to_string(),
            config,
            state: SessionState::Created,
            lattice,
            entropy_mapper,
            miner,
            spin_results: Vec::new(),
            mining_result: None,
            best_sequence: None,
            start_time: std::time::Instant::now(),
        }
    }

    /// Create default session
    pub fn default_session() -> Self {
        Self::new(SlotsSessionConfig::default())
    }

    /// Run the full session
    pub fn run(&mut self) -> Result<SlotsSessionResult> {
        self.start_time = std::time::Instant::now();

        // Spin phase
        self.run_spins(self.config.spins_before_mine)?;

        // Mining phase
        if self.config.auto_mine {
            self.run_mining()?;
        }

        self.state = SessionState::Completed;
        Ok(self.build_result())
    }

    /// Run spin phase
    pub fn run_spins(&mut self, count: usize) -> Result<()> {
        self.state = SessionState::Spinning;

        for _ in 0..count {
            let result = self.lattice.spin(&mut self.entropy_mapper)?;
            self.spin_results.push(result);
        }

        Ok(())
    }

    /// Perform a single spin
    pub fn spin(&mut self) -> Result<LatticeResult> {
        self.state = SessionState::Spinning;
        let result = self.lattice.spin(&mut self.entropy_mapper)?;
        self.spin_results.push(result.clone());
        Ok(result)
    }

    /// Run mining phase
    pub fn run_mining(&mut self) -> Result<MiningResult> {
        self.state = SessionState::Mining;
        let result = self.miner.mine()?;

        self.best_sequence = Some(result.best_sequence.clone());
        self.mining_result = Some(result.clone());

        Ok(result)
    }

    /// Get the lattice
    pub fn lattice(&self) -> &SlotLattice {
        &self.lattice
    }

    /// Get mutable lattice
    pub fn lattice_mut(&mut self) -> &mut SlotLattice {
        &mut self.lattice
    }

    /// Get spin results
    pub fn spin_results(&self) -> &[LatticeResult] {
        &self.spin_results
    }

    /// Get mining result
    pub fn mining_result(&self) -> Option<&MiningResult> {
        self.mining_result.as_ref()
    }

    /// Get best sequence
    pub fn best_sequence(&self) -> Option<&MinedSequence> {
        self.best_sequence.as_ref()
    }

    /// Get best resonance
    pub fn best_resonance(&self) -> f64 {
        self.best_sequence
            .as_ref()
            .map(|s| s.resonance)
            .unwrap_or(0.0)
    }

    /// Build session result
    fn build_result(&self) -> SlotsSessionResult {
        SlotsSessionResult {
            session_id: self.id.clone(),
            state: self.state,
            spin_count: self.spin_results.len(),
            spin_results: self.spin_results.clone(),
            mining_result: self.mining_result.clone(),
            best_sequence: self.best_sequence.clone(),
            best_resonance: self.best_resonance(),
            total_time_ms: self.start_time.elapsed().as_millis() as u64,
        }
    }

    /// Reset session
    pub fn reset(&mut self) {
        self.lattice.reset();
        self.spin_results.clear();
        self.mining_result = None;
        self.best_sequence = None;
        self.state = SessionState::Created;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = SlotsSession::default_session();
        assert_eq!(session.state, SessionState::Created);
    }

    #[test]
    fn test_session_spin() {
        let mut session = SlotsSession::default_session();
        let result = session.spin().unwrap();

        assert_eq!(session.spin_results.len(), 1);
        assert!(!result.values.is_empty());
    }

    #[test]
    fn test_full_session() {
        let mut session = SlotsSession::new(SlotsSessionConfig::quick());
        let result = session.run().unwrap();

        assert_eq!(result.state, SessionState::Completed);
        assert!(result.spin_count > 0);
    }
}
