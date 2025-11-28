//! Application state management

use qops_circuits::Circuit;
use qops_genesis::MetatronCube;
use qops_seraphic::SeraphicCalibrator;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

/// Global application state
pub struct AppState {
    /// Stored circuits by ID
    pub circuits: Mutex<HashMap<String, Circuit>>,

    /// Current S7 topology instance
    pub s7_topology: Mutex<Option<MetatronCube>>,

    /// Seraphic calibrator instance
    pub calibrator: Mutex<Option<SeraphicCalibrator>>,

    /// Experiment history
    pub experiment_history: Mutex<Vec<ExperimentRecord>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            circuits: Mutex::new(HashMap::new()),
            s7_topology: Mutex::new(None),
            calibrator: Mutex::new(None),
            experiment_history: Mutex::new(Vec::new()),
        }
    }

    /// Generate a new unique ID
    pub fn new_id() -> String {
        Uuid::new_v4().to_string()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Record of an experiment run
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ExperimentRecord {
    pub id: String,
    pub name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub algorithm: String,
    pub parameters: serde_json::Value,
    pub result: serde_json::Value,
}
