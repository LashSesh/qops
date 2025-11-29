//! Slot Lattice Structure
//!
//! Multi-dimensional grid of interconnected slots.

use crate::slot::{Slot, SlotConfig, SlotValue, SlotSymbol};
use crate::entropy::EntropyMapper;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Lattice configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeConfig {
    /// Number of rows
    pub rows: usize,
    /// Number of columns
    pub cols: usize,
    /// Slot configuration to use
    pub slot_config: SlotConfig,
    /// Connect adjacent slots
    pub connect_adjacent: bool,
    /// Connect diagonal slots
    pub connect_diagonal: bool,
    /// Wrap around edges (toroidal)
    pub wrap_around: bool,
}

impl Default for LatticeConfig {
    fn default() -> Self {
        Self {
            rows: 3,
            cols: 5,
            slot_config: SlotConfig::default(),
            connect_adjacent: true,
            connect_diagonal: false,
            wrap_around: false,
        }
    }
}

impl LatticeConfig {
    /// Create a standard 3x5 lattice (like classic slots)
    pub fn classic() -> Self {
        Self::default()
    }

    /// Create a 5x5 lattice for 5D mapping
    pub fn pentagonal() -> Self {
        Self {
            rows: 5,
            cols: 5,
            ..Default::default()
        }
    }

    /// Create a single row
    pub fn single_row(cols: usize) -> Self {
        Self {
            rows: 1,
            cols,
            ..Default::default()
        }
    }
}

/// A node in the lattice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeNode {
    /// Row position
    pub row: usize,
    /// Column position
    pub col: usize,
    /// The slot at this position
    pub slot: Slot,
}

impl LatticeNode {
    /// Create a new lattice node
    pub fn new(row: usize, col: usize, slot: Slot) -> Self {
        Self { row, col, slot }
    }

    /// Get position as tuple
    pub fn position(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

/// An edge connecting two lattice nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeEdge {
    /// Source position
    pub from: (usize, usize),
    /// Target position
    pub to: (usize, usize),
    /// Edge weight (connection strength)
    pub weight: f64,
    /// Is diagonal connection
    pub diagonal: bool,
}

impl LatticeEdge {
    /// Create a new edge
    pub fn new(from: (usize, usize), to: (usize, usize), diagonal: bool) -> Self {
        Self {
            from,
            to,
            weight: 1.0,
            diagonal,
        }
    }
}

/// The slot lattice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotLattice {
    /// Unique identifier
    pub id: String,
    /// Configuration
    pub config: LatticeConfig,
    /// Nodes indexed by (row, col)
    nodes: HashMap<(usize, usize), LatticeNode>,
    /// Edges
    edges: Vec<LatticeEdge>,
    /// Current spin result
    pub current_result: Option<LatticeResult>,
    /// History of results
    pub history: Vec<LatticeResult>,
}

/// Result of a lattice spin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeResult {
    /// Values for each position
    pub values: HashMap<(usize, usize), SlotValue>,
    /// Total score
    pub total_score: f64,
    /// Winning lines (if any)
    pub winning_lines: Vec<WinLine>,
    /// Bonus activated
    pub bonus_activated: bool,
    /// Timestamp
    pub timestamp: String,
}

/// A winning line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinLine {
    /// Positions in the line
    pub positions: Vec<(usize, usize)>,
    /// Symbol that won
    pub symbol: SlotSymbol,
    /// Multiplier
    pub multiplier: f64,
    /// Score for this line
    pub score: f64,
}

impl SlotLattice {
    /// Create a new slot lattice
    pub fn new(config: LatticeConfig) -> Self {
        let mut lattice = Self {
            id: Uuid::new_v4().to_string(),
            config: config.clone(),
            nodes: HashMap::new(),
            edges: Vec::new(),
            current_result: None,
            history: Vec::new(),
        };

        lattice.initialize();
        lattice
    }

    /// Create with default config
    pub fn default_lattice() -> Self {
        Self::new(LatticeConfig::default())
    }

    /// Initialize the lattice
    fn initialize(&mut self) {
        // Create nodes
        for row in 0..self.config.rows {
            for col in 0..self.config.cols {
                let name = format!("slot_{}_{}", row, col);
                let mut slot = Slot::new(&name, self.config.slot_config.clone());
                slot.position = Some((row, col));

                let node = LatticeNode::new(row, col, slot);
                self.nodes.insert((row, col), node);
            }
        }

        // Create edges
        self.create_edges();
    }

    /// Create connections between nodes
    fn create_edges(&mut self) {
        let rows = self.config.rows;
        let cols = self.config.cols;

        for row in 0..rows {
            for col in 0..cols {
                // Horizontal connections
                if self.config.connect_adjacent {
                    if col + 1 < cols {
                        self.edges.push(LatticeEdge::new((row, col), (row, col + 1), false));
                    } else if self.config.wrap_around {
                        self.edges.push(LatticeEdge::new((row, col), (row, 0), false));
                    }

                    // Vertical connections
                    if row + 1 < rows {
                        self.edges.push(LatticeEdge::new((row, col), (row + 1, col), false));
                    } else if self.config.wrap_around {
                        self.edges.push(LatticeEdge::new((row, col), (0, col), false));
                    }
                }

                // Diagonal connections
                if self.config.connect_diagonal {
                    if row + 1 < rows && col + 1 < cols {
                        self.edges.push(LatticeEdge::new((row, col), (row + 1, col + 1), true));
                    }
                    if row + 1 < rows && col > 0 {
                        self.edges.push(LatticeEdge::new((row, col), (row + 1, col - 1), true));
                    }
                }
            }
        }
    }

    /// Get a node
    pub fn get_node(&self, row: usize, col: usize) -> Option<&LatticeNode> {
        self.nodes.get(&(row, col))
    }

    /// Get a mutable node
    pub fn get_node_mut(&mut self, row: usize, col: usize) -> Option<&mut LatticeNode> {
        self.nodes.get_mut(&(row, col))
    }

    /// Spin all unlocked slots
    pub fn spin(&mut self, entropy_mapper: &mut EntropyMapper) -> Result<LatticeResult> {
        let mut values = HashMap::new();

        // Spin each slot
        for ((row, col), node) in &mut self.nodes {
            node.slot.spin();
            let entropy = entropy_mapper.generate();
            let value = node.slot.stop(entropy);
            values.insert((*row, *col), value);
        }

        // Check for winning lines
        let winning_lines = self.check_winning_lines(&values);

        // Calculate total score
        let mut total_score: f64 = values.values()
            .map(|v| v.effective_value())
            .sum();

        // Add winning line bonuses
        for line in &winning_lines {
            total_score += line.score;
        }

        // Check for bonus
        let bonus_activated = values.values().any(|v| {
            matches!(v.symbol, SlotSymbol::Star | SlotSymbol::Diamond)
        });

        let result = LatticeResult {
            values,
            total_score,
            winning_lines,
            bonus_activated,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.current_result = Some(result.clone());
        self.history.push(result.clone());

        Ok(result)
    }

    /// Check for winning lines
    fn check_winning_lines(&self, values: &HashMap<(usize, usize), SlotValue>) -> Vec<WinLine> {
        let mut wins = Vec::new();

        // Check horizontal lines
        for row in 0..self.config.rows {
            if let Some(line) = self.check_horizontal_line(row, values) {
                wins.push(line);
            }
        }

        // Check vertical lines
        for col in 0..self.config.cols {
            if let Some(line) = self.check_vertical_line(col, values) {
                wins.push(line);
            }
        }

        // Check diagonals if grid is square
        if self.config.rows == self.config.cols {
            if let Some(line) = self.check_diagonal(values, true) {
                wins.push(line);
            }
            if let Some(line) = self.check_diagonal(values, false) {
                wins.push(line);
            }
        }

        wins
    }

    /// Check a horizontal line
    fn check_horizontal_line(&self, row: usize, values: &HashMap<(usize, usize), SlotValue>) -> Option<WinLine> {
        let cols = self.config.cols;
        if cols < 3 {
            return None;
        }

        // Get first symbol
        let first = values.get(&(row, 0))?.symbol;

        // Count matching symbols
        let mut count = 1;
        let mut positions = vec![(row, 0)];

        for col in 1..cols {
            let val = values.get(&(row, col))?;
            if val.symbol == first || matches!(val.symbol, SlotSymbol::Star) {
                count += 1;
                positions.push((row, col));
            } else {
                break;
            }
        }

        // Need at least 3 matching
        if count >= 3 {
            let multiplier = 1.0 + (count as f64 - 3.0) * 0.5;
            let score = first.base_value() * multiplier * count as f64;

            Some(WinLine {
                positions,
                symbol: first,
                multiplier,
                score,
            })
        } else {
            None
        }
    }

    /// Check a vertical line
    fn check_vertical_line(&self, col: usize, values: &HashMap<(usize, usize), SlotValue>) -> Option<WinLine> {
        let rows = self.config.rows;
        if rows < 3 {
            return None;
        }

        let first = values.get(&(0, col))?.symbol;
        let mut count = 1;
        let mut positions = vec![(0, col)];

        for row in 1..rows {
            let val = values.get(&(row, col))?;
            if val.symbol == first || matches!(val.symbol, SlotSymbol::Star) {
                count += 1;
                positions.push((row, col));
            } else {
                break;
            }
        }

        if count >= 3 {
            let multiplier = 1.0 + (count as f64 - 3.0) * 0.5;
            let score = first.base_value() * multiplier * count as f64;

            Some(WinLine {
                positions,
                symbol: first,
                multiplier,
                score,
            })
        } else {
            None
        }
    }

    /// Check a diagonal line
    fn check_diagonal(&self, values: &HashMap<(usize, usize), SlotValue>, main: bool) -> Option<WinLine> {
        let size = self.config.rows.min(self.config.cols);
        if size < 3 {
            return None;
        }

        let get_pos = |i: usize| -> (usize, usize) {
            if main {
                (i, i)
            } else {
                (i, size - 1 - i)
            }
        };

        let first = values.get(&get_pos(0))?.symbol;
        let mut count = 1;
        let mut positions = vec![get_pos(0)];

        for i in 1..size {
            let pos = get_pos(i);
            let val = values.get(&pos)?;
            if val.symbol == first || matches!(val.symbol, SlotSymbol::Star) {
                count += 1;
                positions.push(pos);
            } else {
                break;
            }
        }

        if count >= 3 {
            let multiplier = 1.5 + (count as f64 - 3.0) * 0.5; // Diagonals worth more
            let score = first.base_value() * multiplier * count as f64;

            Some(WinLine {
                positions,
                symbol: first,
                multiplier,
                score,
            })
        } else {
            None
        }
    }

    /// Get all node values as 5D coordinate (for resonance computation)
    pub fn to_coord5d(&self) -> [f64; 5] {
        let mut sums = [0.0; 5];
        let mut counts = [0; 5];

        for node in self.nodes.values() {
            if let Some(dim) = node.slot.value.symbol.to_dimension() {
                sums[dim] += node.slot.value.value;
                counts[dim] += 1;
            }
        }

        // Average values per dimension
        for i in 0..5 {
            if counts[i] > 0 {
                sums[i] /= counts[i] as f64;
            } else {
                sums[i] = 0.5; // Default
            }
        }

        sums
    }

    /// Get average resonance
    pub fn average_resonance(&self) -> f64 {
        let coord = self.to_coord5d();
        // R(v) = 0.4·ψ + 0.3·ρ + 0.3·ω + 0.05·χ - 0.05·η
        0.4 * coord[0] + 0.3 * coord[1] + 0.3 * coord[2] + 0.05 * coord[3] - 0.05 * coord[4]
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get all nodes
    pub fn nodes(&self) -> impl Iterator<Item = &LatticeNode> {
        self.nodes.values()
    }

    /// Reset all slots
    pub fn reset(&mut self) {
        for node in self.nodes.values_mut() {
            node.slot.reset();
        }
        self.current_result = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::EntropyConfig;

    #[test]
    fn test_lattice_creation() {
        let lattice = SlotLattice::default_lattice();
        assert_eq!(lattice.node_count(), 15); // 3x5
    }

    #[test]
    fn test_lattice_spin() {
        let mut lattice = SlotLattice::default_lattice();
        let mut mapper = EntropyMapper::new(EntropyConfig::default());

        let result = lattice.spin(&mut mapper).unwrap();
        assert_eq!(result.values.len(), 15);
        assert!(result.total_score >= 0.0);
    }

    #[test]
    fn test_coord5d() {
        let lattice = SlotLattice::default_lattice();
        let coord = lattice.to_coord5d();

        assert_eq!(coord.len(), 5);
        assert!(coord.iter().all(|&v| v >= 0.0 && v <= 1.0));
    }
}
