//! Slot Topology System
//!
//! Defines different topology configurations for slot lattices.

use crate::lattice::LatticeConfig;
use serde::{Deserialize, Serialize};

/// Type of topology
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyType {
    /// Linear (1D row)
    Linear,
    /// Classic grid (3x5)
    Classic,
    /// Square (NxN)
    Square,
    /// Pentagonal (5x5 for 5D mapping)
    Pentagonal,
    /// Hexagonal layout
    Hexagonal,
    /// Circular/Ring
    Ring,
    /// Metatron-inspired (13 positions)
    Metatron,
    /// Custom
    Custom,
}

/// Topology metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopologyMetrics {
    /// Total node count
    pub node_count: usize,
    /// Total edge count
    pub edge_count: usize,
    /// Average degree (connections per node)
    pub avg_degree: f64,
    /// Is connected
    pub connected: bool,
    /// Diameter (longest shortest path)
    pub diameter: usize,
    /// Has cycles
    pub has_cycles: bool,
}

/// Slot topology generator
#[derive(Debug, Clone)]
pub struct SlotTopology {
    topology_type: TopologyType,
    params: TopologyParams,
}

/// Topology parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyParams {
    /// Size parameter (varies by type)
    pub size: usize,
    /// Secondary size (for rectangular)
    pub size2: Option<usize>,
    /// Connect neighbors
    pub connect_neighbors: bool,
    /// Connect diagonals
    pub connect_diagonals: bool,
    /// Wrap edges
    pub wrap: bool,
}

impl Default for TopologyParams {
    fn default() -> Self {
        Self {
            size: 5,
            size2: None,
            connect_neighbors: true,
            connect_diagonals: false,
            wrap: false,
        }
    }
}

impl SlotTopology {
    /// Create a new topology
    pub fn new(topology_type: TopologyType) -> Self {
        Self {
            topology_type,
            params: TopologyParams::default(),
        }
    }

    /// Create with parameters
    pub fn with_params(topology_type: TopologyType, params: TopologyParams) -> Self {
        Self { topology_type, params }
    }

    /// Set size
    pub fn with_size(mut self, size: usize) -> Self {
        self.params.size = size;
        self
    }

    /// Set secondary size
    pub fn with_size2(mut self, size2: usize) -> Self {
        self.params.size2 = Some(size2);
        self
    }

    /// Enable wrapping
    pub fn with_wrap(mut self) -> Self {
        self.params.wrap = true;
        self
    }

    /// Enable diagonals
    pub fn with_diagonals(mut self) -> Self {
        self.params.connect_diagonals = true;
        self
    }

    /// Generate lattice configuration
    pub fn to_lattice_config(&self) -> LatticeConfig {
        match self.topology_type {
            TopologyType::Linear => LatticeConfig {
                rows: 1,
                cols: self.params.size,
                connect_adjacent: self.params.connect_neighbors,
                connect_diagonal: false,
                wrap_around: self.params.wrap,
                ..Default::default()
            },
            TopologyType::Classic => LatticeConfig {
                rows: 3,
                cols: 5,
                connect_adjacent: true,
                connect_diagonal: false,
                wrap_around: false,
                ..Default::default()
            },
            TopologyType::Square => LatticeConfig {
                rows: self.params.size,
                cols: self.params.size,
                connect_adjacent: self.params.connect_neighbors,
                connect_diagonal: self.params.connect_diagonals,
                wrap_around: self.params.wrap,
                ..Default::default()
            },
            TopologyType::Pentagonal => LatticeConfig {
                rows: 5,
                cols: 5,
                connect_adjacent: true,
                connect_diagonal: true,
                wrap_around: false,
                ..Default::default()
            },
            TopologyType::Hexagonal => {
                // Approximate hex with offset grid
                LatticeConfig {
                    rows: self.params.size,
                    cols: self.params.size2.unwrap_or(self.params.size + 1),
                    connect_adjacent: true,
                    connect_diagonal: true, // Approximate hex connections
                    wrap_around: false,
                    ..Default::default()
                }
            }
            TopologyType::Ring => LatticeConfig {
                rows: 1,
                cols: self.params.size,
                connect_adjacent: true,
                connect_diagonal: false,
                wrap_around: true,
                ..Default::default()
            },
            TopologyType::Metatron => {
                // 13 positions: 1 center + 6 hexagon + 6 outer
                // Approximate with 3x5 grid (15 positions, close enough)
                // Or use 4x4 - 3 = 13 for actual count
                LatticeConfig {
                    rows: 4,
                    cols: 4,
                    connect_adjacent: true,
                    connect_diagonal: true,
                    wrap_around: false,
                    ..Default::default()
                }
            }
            TopologyType::Custom => LatticeConfig {
                rows: self.params.size,
                cols: self.params.size2.unwrap_or(self.params.size),
                connect_adjacent: self.params.connect_neighbors,
                connect_diagonal: self.params.connect_diagonals,
                wrap_around: self.params.wrap,
                ..Default::default()
            },
        }
    }

    /// Calculate metrics for this topology
    pub fn metrics(&self) -> TopologyMetrics {
        let config = self.to_lattice_config();
        let node_count = config.rows * config.cols;

        // Calculate edges
        let mut edge_count = 0;

        // Horizontal edges
        if config.connect_adjacent {
            edge_count += config.rows * (config.cols - 1);
            if config.wrap_around {
                edge_count += config.rows; // wrap edges
            }
        }

        // Vertical edges
        if config.connect_adjacent && config.rows > 1 {
            edge_count += (config.rows - 1) * config.cols;
            if config.wrap_around {
                edge_count += config.cols;
            }
        }

        // Diagonal edges
        if config.connect_diagonal && config.rows > 1 {
            edge_count += 2 * (config.rows - 1) * (config.cols - 1);
        }

        let avg_degree = if node_count > 0 {
            2.0 * edge_count as f64 / node_count as f64
        } else {
            0.0
        };

        // Simplified diameter estimate
        let diameter = if config.wrap_around {
            (config.rows.max(config.cols) + 1) / 2
        } else {
            config.rows + config.cols - 2
        };

        TopologyMetrics {
            node_count,
            edge_count,
            avg_degree,
            connected: edge_count > 0,
            diameter,
            has_cycles: config.wrap_around || (config.connect_diagonal && config.rows > 1 && config.cols > 1),
        }
    }

    /// Get topology type
    pub fn topology_type(&self) -> TopologyType {
        self.topology_type
    }
}

/// Predefined topologies
impl SlotTopology {
    /// Standard 5-reel slot
    pub fn classic_5reel() -> Self {
        Self::new(TopologyType::Classic)
    }

    /// 5D mapping topology
    pub fn pentagonal_5d() -> Self {
        Self::new(TopologyType::Pentagonal)
    }

    /// Metatron cube inspired
    pub fn metatron() -> Self {
        Self::new(TopologyType::Metatron)
    }

    /// Simple ring
    pub fn ring(size: usize) -> Self {
        Self::new(TopologyType::Ring).with_size(size)
    }

    /// Square grid
    pub fn square(size: usize) -> Self {
        Self::new(TopologyType::Square).with_size(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_topology() {
        let topo = SlotTopology::classic_5reel();
        let config = topo.to_lattice_config();

        assert_eq!(config.rows, 3);
        assert_eq!(config.cols, 5);
    }

    #[test]
    fn test_pentagonal_topology() {
        let topo = SlotTopology::pentagonal_5d();
        let metrics = topo.metrics();

        assert_eq!(metrics.node_count, 25);
        assert!(metrics.connected);
    }

    #[test]
    fn test_ring_topology() {
        let topo = SlotTopology::ring(8);
        let metrics = topo.metrics();

        assert_eq!(metrics.node_count, 8);
        assert!(metrics.has_cycles);
    }

    #[test]
    fn test_topology_metrics() {
        let topo = SlotTopology::square(4).with_diagonals();
        let metrics = topo.metrics();

        assert_eq!(metrics.node_count, 16);
        assert!(metrics.edge_count > 16); // Has diagonals
    }
}
