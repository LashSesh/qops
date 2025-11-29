//! Hierarchical Directed Acyclic Graph (HDAG) for Execution
//!
//! The HDAG wraps the Genesis + TRITON + Resonance Pipeline inside a
//! directed acyclic execution graph, enabling:
//! - Multi-stage operator compilation
//! - Hierarchical execution ordering
//! - Artifact generation and tracking

use crate::coordinates::Coord5D;
use crate::operators::{Operator5D, OperatorFamily, OperatorType, CompilationOperator, CompilationMode};
use crate::artifact::{HypercubeArtifact, ArtifactType, ArtifactMetadata};
use crate::error::{HypercubeError, Result};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// State of an HDAG node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HDAGNodeState {
    /// Not yet ready (dependencies not met)
    Pending,
    /// Ready to execute
    Ready,
    /// Currently executing
    Executing,
    /// Execution complete
    Completed,
    /// Execution failed
    Failed,
    /// Skipped (optional node)
    Skipped,
}

/// Type of HDAG node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HDAGNodeType {
    /// Input node (data source)
    Input,
    /// Operator application node
    Operator,
    /// Compilation node (Ξ operator)
    Compilation,
    /// Merge node (combines multiple inputs)
    Merge,
    /// Fork node (splits to multiple outputs)
    Fork,
    /// Output node (artifact generation)
    Output,
    /// Checkpoint node (saves state)
    Checkpoint,
    /// Conditional node (branches based on condition)
    Conditional,
}

/// A node in the HDAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDAGNode {
    /// Unique identifier
    pub id: String,
    /// Node name
    pub name: String,
    /// Node type
    pub node_type: HDAGNodeType,
    /// Current state
    pub state: HDAGNodeState,
    /// Associated operator type (for Operator nodes)
    pub operator_type: Option<OperatorType>,
    /// Input coordinate (if applicable)
    pub input: Option<Coord5D>,
    /// Output coordinate (after execution)
    pub output: Option<Coord5D>,
    /// Execution priority (lower = higher priority)
    pub priority: i32,
    /// Is node optional (can be skipped)
    pub optional: bool,
    /// Execution time in milliseconds
    pub execution_time_ms: Option<u64>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Node metadata
    pub metadata: HDAGNodeMetadata,
}

/// Node metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HDAGNodeMetadata {
    pub created_at: Option<String>,
    pub executed_at: Option<String>,
    pub resonance_before: f64,
    pub resonance_after: f64,
    pub tags: Vec<String>,
}

impl HDAGNode {
    /// Create a new HDAG node
    pub fn new(name: &str, node_type: HDAGNodeType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            node_type,
            state: HDAGNodeState::Pending,
            operator_type: None,
            input: None,
            output: None,
            priority: 0,
            optional: false,
            execution_time_ms: None,
            error: None,
            metadata: HDAGNodeMetadata::default(),
        }
    }

    /// Create an input node
    pub fn input(name: &str, coord: Coord5D) -> Self {
        let mut node = Self::new(name, HDAGNodeType::Input);
        node.input = Some(coord);
        node.output = Some(coord);
        node.state = HDAGNodeState::Completed; // Input nodes start completed
        node
    }

    /// Create an operator node
    pub fn operator(name: &str, op_type: OperatorType) -> Self {
        let mut node = Self::new(name, HDAGNodeType::Operator);
        node.operator_type = Some(op_type);
        node
    }

    /// Create a compilation node
    pub fn compilation(name: &str) -> Self {
        let mut node = Self::new(name, HDAGNodeType::Compilation);
        node.operator_type = Some(OperatorType::Xi);
        node
    }

    /// Create an output node
    pub fn output(name: &str) -> Self {
        Self::new(name, HDAGNodeType::Output)
    }

    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set as optional
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Check if node can execute
    pub fn can_execute(&self) -> bool {
        self.state == HDAGNodeState::Ready
    }

    /// Mark as ready
    pub fn mark_ready(&mut self) {
        if self.state == HDAGNodeState::Pending {
            self.state = HDAGNodeState::Ready;
        }
    }

    /// Mark as executing
    pub fn mark_executing(&mut self) {
        self.state = HDAGNodeState::Executing;
        self.metadata.executed_at = Some(chrono::Utc::now().to_rfc3339());
    }

    /// Mark as completed
    pub fn mark_completed(&mut self, output: Coord5D, time_ms: u64) {
        self.state = HDAGNodeState::Completed;
        self.output = Some(output);
        self.execution_time_ms = Some(time_ms);
        self.metadata.resonance_after = output.resonance();
    }

    /// Mark as failed
    pub fn mark_failed(&mut self, error: &str) {
        self.state = HDAGNodeState::Failed;
        self.error = Some(error.to_string());
    }
}

/// Type of HDAG edge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HDAGEdgeType {
    /// Data flow edge
    Data,
    /// Control flow edge
    Control,
    /// Dependency edge (must complete before)
    Dependency,
}

/// An edge in the HDAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDAGEdge {
    /// Edge type
    pub edge_type: HDAGEdgeType,
    /// Weight/priority
    pub weight: f64,
    /// Label
    pub label: Option<String>,
}

impl HDAGEdge {
    /// Create a data edge
    pub fn data() -> Self {
        Self {
            edge_type: HDAGEdgeType::Data,
            weight: 1.0,
            label: None,
        }
    }

    /// Create a control edge
    pub fn control() -> Self {
        Self {
            edge_type: HDAGEdgeType::Control,
            weight: 0.0,
            label: None,
        }
    }

    /// Create a dependency edge
    pub fn dependency() -> Self {
        Self {
            edge_type: HDAGEdgeType::Dependency,
            weight: 0.0,
            label: None,
        }
    }

    /// Set label
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
}

impl Default for HDAGEdge {
    fn default() -> Self {
        Self::data()
    }
}

/// The Hierarchical Directed Acyclic Graph
///
/// Note: The `operators` field contains `Box<dyn Operator5D>` which cannot be cloned.
/// When cloning this struct, the operators HashMap will be empty. The HDAG can still
/// execute using the built-in operator application logic based on `operator_type`.
pub struct HDAG {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// The underlying directed graph
    graph: DiGraph<HDAGNode, HDAGEdge>,
    /// Node ID to index mapping
    node_index_map: HashMap<String, NodeIndex>,
    /// Operator instances for execution (not cloneable/debuggable)
    #[allow(dead_code)]
    operators: HashMap<String, Box<dyn Operator5D>>,
    /// Generated artifacts
    artifacts: Vec<HypercubeArtifact>,
    /// Execution order (topological sort)
    execution_order: Vec<NodeIndex>,
    /// Current execution position
    current_position: usize,
}

impl std::fmt::Debug for HDAG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HDAG")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("node_count", &self.graph.node_count())
            .field("edge_count", &self.graph.edge_count())
            .field("operators_count", &self.operators.len())
            .field("artifacts_count", &self.artifacts.len())
            .field("current_position", &self.current_position)
            .finish()
    }
}

impl Clone for HDAG {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            graph: self.graph.clone(),
            node_index_map: self.node_index_map.clone(),
            operators: HashMap::new(), // Operators are not cloned
            artifacts: self.artifacts.clone(),
            execution_order: self.execution_order.clone(),
            current_position: self.current_position,
        }
    }
}

impl HDAG {
    /// Create a new HDAG
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            graph: DiGraph::new(),
            node_index_map: HashMap::new(),
            operators: HashMap::new(),
            artifacts: Vec::new(),
            execution_order: Vec::new(),
            current_position: 0,
        }
    }

    /// Add a node to the HDAG
    pub fn add_node(&mut self, node: HDAGNode) -> String {
        let id = node.id.clone();
        let idx = self.graph.add_node(node);
        self.node_index_map.insert(id.clone(), idx);
        id
    }

    /// Add an edge between nodes
    pub fn add_edge(&mut self, from_id: &str, to_id: &str, edge: HDAGEdge) -> Result<()> {
        let from_idx = self.node_index_map.get(from_id)
            .ok_or_else(|| HypercubeError::VertexNotFound(from_id.to_string()))?;
        let to_idx = self.node_index_map.get(to_id)
            .ok_or_else(|| HypercubeError::VertexNotFound(to_id.to_string()))?;

        self.graph.add_edge(*from_idx, *to_idx, edge);
        Ok(())
    }

    /// Get a node by ID
    pub fn get_node(&self, id: &str) -> Option<&HDAGNode> {
        self.node_index_map.get(id)
            .and_then(|idx| self.graph.node_weight(*idx))
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut HDAGNode> {
        self.node_index_map.get(id)
            .and_then(|idx| self.graph.node_weight_mut(*idx))
    }

    /// Compute topological execution order
    pub fn compute_execution_order(&mut self) -> Result<()> {
        match toposort(&self.graph, None) {
            Ok(order) => {
                self.execution_order = order;
                self.current_position = 0;
                Ok(())
            }
            Err(_) => Err(HypercubeError::CycleDetected),
        }
    }

    /// Update node readiness based on dependencies
    fn update_readiness(&mut self) {
        for idx in self.execution_order.clone() {
            let deps_complete = self.graph
                .neighbors_directed(idx, petgraph::Direction::Incoming)
                .all(|dep_idx| {
                    self.graph.node_weight(dep_idx)
                        .map(|n| n.state == HDAGNodeState::Completed || n.state == HDAGNodeState::Skipped)
                        .unwrap_or(false)
                });

            if deps_complete {
                if let Some(node) = self.graph.node_weight_mut(idx) {
                    node.mark_ready();
                }
            }
        }
    }

    /// Get the next node to execute
    pub fn next_executable(&self) -> Option<&HDAGNode> {
        for idx in &self.execution_order[self.current_position..] {
            if let Some(node) = self.graph.node_weight(*idx) {
                if node.can_execute() {
                    return Some(node);
                }
            }
        }
        None
    }

    /// Get inputs from predecessor nodes
    fn get_inputs(&self, node_id: &str) -> Vec<Coord5D> {
        let idx = match self.node_index_map.get(node_id) {
            Some(i) => *i,
            None => return Vec::new(),
        };

        self.graph
            .neighbors_directed(idx, petgraph::Direction::Incoming)
            .filter_map(|pred_idx| {
                self.graph.node_weight(pred_idx)
                    .and_then(|n| n.output)
            })
            .collect()
    }

    /// Execute a single node
    fn execute_node(&mut self, node_id: &str) -> Result<Coord5D> {
        let inputs = self.get_inputs(node_id);

        let node = self.get_node(node_id)
            .ok_or_else(|| HypercubeError::VertexNotFound(node_id.to_string()))?;

        let node_type = node.node_type;
        let operator_type = node.operator_type;
        let node_input = node.input;

        // Determine input coordinate
        let input = if !inputs.is_empty() {
            // Average of all inputs for merge
            let mut sum = Coord5D::origin();
            for inp in &inputs {
                sum = sum.add(inp);
            }
            sum.scale(1.0 / inputs.len() as f64)
        } else {
            node_input.unwrap_or(Coord5D::center())
        };

        // Execute based on node type
        let output = match node_type {
            HDAGNodeType::Input => {
                node_input.unwrap_or(input)
            }
            HDAGNodeType::Operator => {
                self.apply_operator(operator_type, &input)?
            }
            HDAGNodeType::Compilation => {
                let compiler = CompilationOperator::new(CompilationMode::Balanced);
                compiler.apply(&input)
            }
            HDAGNodeType::Merge => {
                // Already computed as average
                input
            }
            HDAGNodeType::Fork => {
                // Pass through
                input
            }
            HDAGNodeType::Output => {
                // Generate artifact
                input
            }
            HDAGNodeType::Checkpoint => {
                input
            }
            HDAGNodeType::Conditional => {
                // For now, pass through
                input
            }
        };

        Ok(output)
    }

    /// Apply an operator
    fn apply_operator(&self, op_type: Option<OperatorType>, input: &Coord5D) -> Result<Coord5D> {
        use crate::operators::*;

        let output = match op_type {
            Some(OperatorType::DK) => {
                DoubleKickOperator::default().apply(input)
            }
            Some(OperatorType::SW) => {
                SwapWaveOperator::default().apply(input)
            }
            Some(OperatorType::PI) => {
                PhaseIntegrationOperator::default().apply(input)
            }
            Some(OperatorType::WT) => {
                WeightTransformOperator::default().apply(input)
            }
            Some(OperatorType::Xi) => {
                CompilationOperator::default().apply(input)
            }
            Some(OperatorType::Identity) | None => {
                *input
            }
            Some(OperatorType::Composite) => {
                // Apply all operators in sequence
                let mut result = *input;
                result = DoubleKickOperator::default().apply(&result);
                result = SwapWaveOperator::default().apply(&result);
                result = PhaseIntegrationOperator::default().apply(&result);
                result = WeightTransformOperator::default().apply(&result);
                result
            }
        };

        Ok(output)
    }

    /// Check if execution is complete
    pub fn is_complete(&self) -> bool {
        self.graph.node_weights().all(|n| {
            matches!(n.state, HDAGNodeState::Completed | HDAGNodeState::Skipped | HDAGNodeState::Failed)
        })
    }

    /// Get all artifacts
    pub fn artifacts(&self) -> &[HypercubeArtifact] {
        &self.artifacts
    }

    /// Get execution progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.execution_order.is_empty() {
            return 1.0;
        }

        let completed = self.graph.node_weights()
            .filter(|n| matches!(n.state, HDAGNodeState::Completed | HDAGNodeState::Skipped))
            .count();

        completed as f64 / self.graph.node_count() as f64
    }

    /// Get all nodes
    pub fn nodes(&self) -> impl Iterator<Item = &HDAGNode> {
        self.graph.node_weights()
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get edge count
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Create a standard pipeline HDAG
    /// Input -> DK -> SW -> PI -> WT -> Compilation -> Output
    pub fn standard_pipeline(seed: Coord5D) -> Self {
        let mut hdag = Self::new("Standard Pipeline");

        let input_id = hdag.add_node(HDAGNode::input("Input", seed));
        let dk_id = hdag.add_node(HDAGNode::operator("Double Kick", OperatorType::DK));
        let sw_id = hdag.add_node(HDAGNode::operator("Swap Wave", OperatorType::SW));
        let pi_id = hdag.add_node(HDAGNode::operator("Phase Integration", OperatorType::PI));
        let wt_id = hdag.add_node(HDAGNode::operator("Weight Transform", OperatorType::WT));
        let compile_id = hdag.add_node(HDAGNode::compilation("Compile"));
        let output_id = hdag.add_node(HDAGNode::output("Output"));

        hdag.add_edge(&input_id, &dk_id, HDAGEdge::data()).ok();
        hdag.add_edge(&dk_id, &sw_id, HDAGEdge::data()).ok();
        hdag.add_edge(&sw_id, &pi_id, HDAGEdge::data()).ok();
        hdag.add_edge(&pi_id, &wt_id, HDAGEdge::data()).ok();
        hdag.add_edge(&wt_id, &compile_id, HDAGEdge::data()).ok();
        hdag.add_edge(&compile_id, &output_id, HDAGEdge::data()).ok();

        hdag.compute_execution_order().ok();
        hdag
    }

    /// Create a parallel branches HDAG
    pub fn parallel_branches(seed: Coord5D) -> Self {
        let mut hdag = Self::new("Parallel Branches");

        let input_id = hdag.add_node(HDAGNode::input("Input", seed));

        // Fork
        let fork_id = hdag.add_node(HDAGNode::new("Fork", HDAGNodeType::Fork));

        // Branch 1: DK -> SW
        let dk_id = hdag.add_node(HDAGNode::operator("DK Branch", OperatorType::DK));
        let sw_id = hdag.add_node(HDAGNode::operator("SW Branch", OperatorType::SW));

        // Branch 2: PI -> WT
        let pi_id = hdag.add_node(HDAGNode::operator("PI Branch", OperatorType::PI));
        let wt_id = hdag.add_node(HDAGNode::operator("WT Branch", OperatorType::WT));

        // Merge
        let merge_id = hdag.add_node(HDAGNode::new("Merge", HDAGNodeType::Merge));

        // Output
        let compile_id = hdag.add_node(HDAGNode::compilation("Compile"));
        let output_id = hdag.add_node(HDAGNode::output("Output"));

        // Edges
        hdag.add_edge(&input_id, &fork_id, HDAGEdge::data()).ok();

        // Branch 1
        hdag.add_edge(&fork_id, &dk_id, HDAGEdge::data().with_label("branch1")).ok();
        hdag.add_edge(&dk_id, &sw_id, HDAGEdge::data()).ok();
        hdag.add_edge(&sw_id, &merge_id, HDAGEdge::data()).ok();

        // Branch 2
        hdag.add_edge(&fork_id, &pi_id, HDAGEdge::data().with_label("branch2")).ok();
        hdag.add_edge(&pi_id, &wt_id, HDAGEdge::data()).ok();
        hdag.add_edge(&wt_id, &merge_id, HDAGEdge::data()).ok();

        // Output
        hdag.add_edge(&merge_id, &compile_id, HDAGEdge::data()).ok();
        hdag.add_edge(&compile_id, &output_id, HDAGEdge::data()).ok();

        hdag.compute_execution_order().ok();
        hdag
    }
}

/// HDAG Executor for running the graph
pub struct HDAGExecutor {
    hdag: HDAG,
}

/// Result of HDAG execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Final output coordinate
    pub output: Coord5D,
    /// Final resonance
    pub resonance: f64,
    /// Total execution time in ms
    pub total_time_ms: u64,
    /// Number of nodes executed
    pub nodes_executed: usize,
    /// Number of nodes failed
    pub nodes_failed: usize,
    /// Generated artifacts
    pub artifact_count: usize,
}

impl HDAGExecutor {
    /// Create a new executor
    pub fn new(hdag: HDAG) -> Self {
        Self { hdag }
    }

    /// Execute the entire HDAG
    pub fn execute(&mut self) -> Result<ExecutionResult> {
        let start = std::time::Instant::now();
        let mut nodes_executed = 0;
        let mut nodes_failed = 0;
        let mut last_output = Coord5D::center();

        self.hdag.compute_execution_order()?;
        self.hdag.update_readiness();

        while !self.hdag.is_complete() {
            // Find next executable node
            let next_id = {
                let next = self.hdag.next_executable();
                match next {
                    Some(n) => n.id.clone(),
                    None => break,
                }
            };

            // Mark as executing
            if let Some(node) = self.hdag.get_node_mut(&next_id) {
                node.mark_executing();
            }

            // Execute
            let node_start = std::time::Instant::now();
            let result = self.hdag.execute_node(&next_id);
            let node_time = node_start.elapsed().as_millis() as u64;

            match result {
                Ok(output) => {
                    last_output = output;
                    if let Some(node) = self.hdag.get_node_mut(&next_id) {
                        node.mark_completed(output, node_time);
                    }
                    nodes_executed += 1;

                    // Generate artifact for output nodes
                    if self.hdag.get_node(&next_id)
                        .map(|n| n.node_type == HDAGNodeType::Output)
                        .unwrap_or(false)
                    {
                        let artifact = HypercubeArtifact::new(
                            "output",
                            ArtifactType::CompiledFamily,
                            output,
                        );
                        self.hdag.artifacts.push(artifact);
                    }
                }
                Err(e) => {
                    if let Some(node) = self.hdag.get_node_mut(&next_id) {
                        if node.optional {
                            node.state = HDAGNodeState::Skipped;
                        } else {
                            node.mark_failed(&e.to_string());
                            nodes_failed += 1;
                        }
                    }
                }
            }

            // Update readiness
            self.hdag.update_readiness();
            self.hdag.current_position += 1;
        }

        let total_time = start.elapsed().as_millis() as u64;

        Ok(ExecutionResult {
            output: last_output,
            resonance: last_output.resonance(),
            total_time_ms: total_time,
            nodes_executed,
            nodes_failed,
            artifact_count: self.hdag.artifacts.len(),
        })
    }

    /// Get the underlying HDAG
    pub fn hdag(&self) -> &HDAG {
        &self.hdag
    }

    /// Take the HDAG (consuming self)
    pub fn into_hdag(self) -> HDAG {
        self.hdag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdag_creation() {
        let hdag = HDAG::new("test");
        assert_eq!(hdag.node_count(), 0);
        assert_eq!(hdag.edge_count(), 0);
    }

    #[test]
    fn test_standard_pipeline() {
        let seed = Coord5D::center();
        let hdag = HDAG::standard_pipeline(seed);

        assert_eq!(hdag.node_count(), 7);
        assert!(hdag.edge_count() >= 6);
    }

    #[test]
    fn test_hdag_execution() {
        let seed = Coord5D::new(0.5, 0.5, 0.5, 0.5, 0.5);
        let hdag = HDAG::standard_pipeline(seed);
        let mut executor = HDAGExecutor::new(hdag);

        let result = executor.execute().unwrap();
        assert!(result.resonance > 0.0);
        assert!(result.nodes_executed > 0);
    }

    #[test]
    fn test_parallel_branches() {
        let seed = Coord5D::center();
        let hdag = HDAG::parallel_branches(seed);
        let mut executor = HDAGExecutor::new(hdag);

        let result = executor.execute().unwrap();
        assert!(result.resonance > 0.0);
    }
}
