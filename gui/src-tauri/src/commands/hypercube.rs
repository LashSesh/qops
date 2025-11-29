//! Hypercube Tauri commands
//!
//! Commands for hypercube operations, HDAG execution, and 5D coordinate manipulation.

use super::{
    Coord5DDto, HypercubeStatsDto, CompilationResultDto,
    HDAGInfoDto, HDAGNodeDto, HDAGEdgeDto, HDAGExecutionResultDto,
    HypercubeSessionResultDto,
};
use qops_hypercube::{
    Hypercube, HypercubeConfig, CubeExpansionRule,
    HypercubeCompiler, CompilationConfig,
    HDAG, HDAGExecutor,
    HypercubeSession, SessionConfig,
    Coord5D,
};

// ============================================================================
// Conversion helpers
// ============================================================================

fn coord5d_to_dto(c: &Coord5D) -> Coord5DDto {
    Coord5DDto {
        psi: c.psi,
        rho: c.rho,
        omega: c.omega,
        chi: c.chi,
        eta: c.eta,
    }
}

fn dto_to_coord5d(dto: &Coord5DDto) -> Coord5D {
    Coord5D::new(dto.psi, dto.rho, dto.omega, dto.chi, dto.eta)
}

// ============================================================================
// Hypercube commands
// ============================================================================

/// Compile a hypercube from a seed coordinate
#[tauri::command]
pub fn compile_hypercube(
    seed_psi: f64,
    seed_rho: f64,
    seed_omega: f64,
    seed_chi: f64,
    seed_eta: f64,
    iterations: usize,
    use_triton: bool,
) -> Result<CompilationResultDto, String> {
    let seed = Coord5D::new(seed_psi, seed_rho, seed_omega, seed_chi, seed_eta);

    let expansion_rule = if use_triton {
        CubeExpansionRule::Triton
    } else {
        CubeExpansionRule::ResonanceGuided
    };

    let config = HypercubeConfig {
        max_depth: iterations,
        expansion_rule,
        ..Default::default()
    };

    let mut cube = Hypercube::new("gui_cube", config);

    // Expand
    for _ in 0..iterations {
        let _ = cube.expand_step();
    }

    // Compile
    let compile_config = CompilationConfig::default();
    let mut compiler = HypercubeCompiler::new(compile_config);
    let result = compiler.compile(&mut cube)
        .map_err(|e| format!("Compilation failed: {}", e))?;

    Ok(CompilationResultDto {
        output: coord5d_to_dto(&result.output),
        resonance: result.resonance,
        iterations: result.iterations,
        threshold_met: result.threshold_met,
        artifact_count: result.artifacts.len(),
    })
}

/// Expand a hypercube step by step
#[tauri::command]
pub fn expand_cube_step(
    current_vertices: usize,
    expansion_rule: String,
    iterations: usize,
) -> Result<HypercubeStatsDto, String> {
    let rule = match expansion_rule.as_str() {
        "lattice" => CubeExpansionRule::Lattice,
        "resonance" => CubeExpansionRule::ResonanceGuided,
        "triton" => CubeExpansionRule::Triton,
        "operator" => CubeExpansionRule::OperatorDriven,
        "random" => CubeExpansionRule::Random,
        "hybrid" => CubeExpansionRule::HybridTriton,
        _ => CubeExpansionRule::Triton,
    };

    let config = HypercubeConfig {
        max_depth: iterations,
        expansion_rule: rule,
        ..Default::default()
    };

    let mut cube = Hypercube::new("expansion_cube", config);

    for _ in 0..iterations {
        let _ = cube.expand_step();
    }

    Ok(HypercubeStatsDto {
        total_vertices: cube.stats.total_vertices,
        total_edges: cube.stats.total_edges,
        max_depth_reached: cube.stats.max_depth_reached,
        best_resonance: cube.best_resonance,
        avg_resonance: cube.stats.avg_resonance,
    })
}

/// Get hypercube info
#[tauri::command]
pub fn get_hypercube_info() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "name": "Hypercube-HDAG 5D Framework",
        "version": "0.1.0",
        "dimensions": 5,
        "coordinates": ["psi", "rho", "omega", "chi", "eta"],
        "operators": ["DK", "SW", "PI", "WT", "Xi"],
        "expansion_rules": ["lattice", "resonance", "triton", "operator", "random", "hybrid"],
        "features": [
            "Self-compiling cubes",
            "HDAG execution",
            "5D operator families",
            "TRITON integration",
            "Resonance-guided expansion"
        ]
    }))
}

// ============================================================================
// HDAG commands
// ============================================================================

/// Execute an HDAG pipeline
#[tauri::command]
pub fn hdag_execute(
    pipeline_type: String,
    seed_psi: f64,
    seed_rho: f64,
    seed_omega: f64,
    seed_chi: f64,
    seed_eta: f64,
) -> Result<HDAGExecutionResultDto, String> {
    let seed = Coord5D::new(seed_psi, seed_rho, seed_omega, seed_chi, seed_eta);

    let hdag = match pipeline_type.as_str() {
        "parallel" => HDAG::parallel_branches(seed),
        _ => HDAG::standard_pipeline(seed),
    };

    let mut executor = HDAGExecutor::new(hdag);
    let result = executor.execute()
        .map_err(|e| format!("HDAG execution failed: {}", e))?;

    Ok(HDAGExecutionResultDto {
        output: coord5d_to_dto(&result.output),
        resonance: result.resonance,
        nodes_executed: result.nodes_executed,
        nodes_failed: result.nodes_failed,
        total_time_ms: result.total_time_ms,
        artifact_count: result.artifact_count,
    })
}

/// Get HDAG structure info
#[tauri::command]
pub fn get_hdag_info(pipeline_type: String) -> Result<HDAGInfoDto, String> {
    let seed = Coord5D::center();

    let hdag = match pipeline_type.as_str() {
        "parallel" => HDAG::parallel_branches(seed),
        _ => HDAG::standard_pipeline(seed),
    };

    // Note: standard_pipeline and parallel_branches already call compute_execution_order internally

    let nodes: Vec<HDAGNodeDto> = hdag.nodes().map(|n| {
        HDAGNodeDto {
            id: n.id.clone(),
            name: n.name.clone(),
            node_type: format!("{:?}", n.node_type),
            input: n.input.as_ref().map(|c| coord5d_to_dto(c)),
            output: n.output.as_ref().map(|c| coord5d_to_dto(c)),
        }
    }).collect();

    // HDAG doesn't expose edges directly, so we return an empty list
    // The edge structure is internal to the DAG
    let edges: Vec<HDAGEdgeDto> = Vec::new();

    Ok(HDAGInfoDto {
        name: hdag.name.clone(),
        nodes,
        edges,
        execution_order: Vec::new(), // execution_order is private
    })
}

// ============================================================================
// Session commands
// ============================================================================

/// Run a full hypercube session
///
/// Note: The seed parameters are kept for future API compatibility when custom
/// seed support is added to the Hypercube API. Currently they are ignored.
#[tauri::command]
pub fn run_hypercube_session(
    preset: String,
    _seed_psi: Option<f64>,
    _seed_rho: Option<f64>,
    _seed_omega: Option<f64>,
    _seed_chi: Option<f64>,
    _seed_eta: Option<f64>,
) -> Result<HypercubeSessionResultDto, String> {
    let config = match preset.as_str() {
        "quick" => SessionConfig::quick(),
        "thorough" => SessionConfig::thorough(),
        "research" => SessionConfig::research(),
        _ => SessionConfig::default(),
    };

    let mut session = HypercubeSession::new(config);

    let result = session.run()
        .map_err(|e| format!("Session failed: {}", e))?;

    Ok(HypercubeSessionResultDto {
        session_id: result.session_id,
        state: format!("{:?}", result.state),
        best_coordinate: coord5d_to_dto(&result.best_coordinate),
        best_resonance: result.best_resonance,
        compilation_result: result.compilation_result.map(|cr| CompilationResultDto {
            output: coord5d_to_dto(&cr.output),
            resonance: cr.resonance,
            iterations: cr.iterations,
            threshold_met: cr.threshold_met,
            artifact_count: cr.artifacts.len(),
        }),
        total_time_ms: result.total_time_ms,
        expansion_steps: result.expansion_steps,
        total_vertices: result.total_vertices,
        artifact_count: result.artifact_count,
    })
}

/// Get available session presets
#[tauri::command]
pub fn get_hypercube_presets() -> Result<Vec<serde_json::Value>, String> {
    Ok(vec![
        serde_json::json!({
            "name": "quick",
            "description": "Fast exploration with 3 expansion steps",
            "max_depth": 3,
            "expansion_rule": "Triton"
        }),
        serde_json::json!({
            "name": "thorough",
            "description": "Thorough exploration with 10 expansion steps",
            "max_depth": 10,
            "expansion_rule": "HybridTriton"
        }),
        serde_json::json!({
            "name": "research",
            "description": "Research-grade exploration with 20 steps",
            "max_depth": 20,
            "expansion_rule": "OperatorDriven"
        }),
    ])
}
