//! Holistic Resonance Architecture
//!
//! Implements the Kosmokrator–Chronokrator–Pfauenthron decision framework:
//! - **Kosmokrator**: Exclusion axis via Proof-of-Resonance (PoR)
//! - **Chronokrator**: Expansion axis via resonance channel dynamics
//! - **Pfauenthron/Monolith**: Decision core via Mandorla convergence
//!
//! The matrix only outputs action vector E(t) when:
//! - PoR(t) = true (Kosmokrator passed)
//! - D_total(t) > Θ(t) (Chronokrator spike)
//!
//! Reference: Sebastian Klemm, "Holistische Resonanzarchitektur"

use crate::{Signature5D, resonance_5d};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// ============================================================================
// GENESIS STAGES
// ============================================================================

/// Mining pipeline stage enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenesisStage {
    /// Initial operator discovery
    Discovery,
    /// Kosmokrator exclusion filter
    KosmokratorFilter,
    /// Chronokrator expansion
    ChronokratorExpansion,
    /// Pfauenthron/Monolith finalization
    PfauenthronCollapse,
    /// Complete
    Finalized,
}

impl Default for GenesisStage {
    fn default() -> Self {
        Self::Discovery
    }
}

impl GenesisStage {
    /// Get the color associated with this stage (for visualization)
    pub fn color(&self) -> &'static str {
        match self {
            Self::Discovery => "#4a90d9",       // Blue
            Self::KosmokratorFilter => "#00bfff", // Deep sky blue
            Self::ChronokratorExpansion => "#9932cc", // Violet
            Self::PfauenthronCollapse => "#ffd700", // Gold
            Self::Finalized => "#00ff00",      // Green
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Discovery => "Discovery",
            Self::KosmokratorFilter => "Kosmokrator Filter",
            Self::ChronokratorExpansion => "Chronokrator Expansion",
            Self::PfauenthronCollapse => "Pfauenthron Collapse",
            Self::Finalized => "Finalized",
        }
    }
}

// ============================================================================
// KOSMOKRATOR - EXCLUSION AXIS
// ============================================================================

/// Configuration for Kosmokrator filter stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KosmokratorConfig {
    /// Resonance threshold κ* for PoR acceptance
    pub kappa_threshold: f64,
    /// Maximum allowed fluctuation ε for stability
    pub epsilon: f64,
    /// Telescope operator focus factor γ
    pub telescope_gamma: f64,
    /// Minimum stability window (timesteps)
    pub stability_window: usize,
    /// Enable exclusion logging
    pub log_exclusions: bool,
}

impl Default for KosmokratorConfig {
    fn default() -> Self {
        Self {
            kappa_threshold: 0.7,
            epsilon: 0.05,
            telescope_gamma: 0.8,
            stability_window: 5,
            log_exclusions: true,
        }
    }
}

/// Proof-of-Resonance result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfResonanceResult {
    /// Whether PoR passed
    pub passed: bool,
    /// Coherence measure κ(t)
    pub kappa: f64,
    /// Rate of change |dκ/dt|
    pub kappa_derivative: f64,
    /// Stability score
    pub stability: f64,
    /// Timestamp
    pub timestamp: f64,
}

/// Kosmokrator filter state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KosmokratorState {
    /// Configuration
    pub config: KosmokratorConfig,
    /// Phase history for coherence computation
    pub phase_history: Vec<f64>,
    /// Kappa history
    pub kappa_history: Vec<f64>,
    /// Filtered (surviving) operators
    pub survivors: Vec<OperatorCandidate>,
    /// Excluded operators
    pub excluded: Vec<OperatorCandidate>,
    /// Current PoR status
    pub por_status: Option<ProofOfResonanceResult>,
    /// Total candidates processed
    pub total_processed: usize,
}

/// An operator candidate being evaluated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorCandidate {
    /// Unique identifier
    pub id: String,
    /// 5D signature
    pub signature: Signature5D,
    /// Phase angle θ
    pub phase: f64,
    /// Resonance score
    pub resonance: f64,
    /// Stability score
    pub stability: f64,
    /// Whether it's in a Mandorla zone
    pub is_mandorla: bool,
    /// Node index in topology
    pub node_index: usize,
    /// Discovery timestamp
    pub discovered_at: f64,
}

impl KosmokratorState {
    /// Create new Kosmokrator state
    pub fn new(config: KosmokratorConfig) -> Self {
        Self {
            config,
            phase_history: Vec::new(),
            kappa_history: Vec::new(),
            survivors: Vec::new(),
            excluded: Vec::new(),
            por_status: None,
            total_processed: 0,
        }
    }

    /// Compute coherence measure κ(t) using exp(iθ) averaging
    /// κ(t) = |1/N Σ exp(iθⱼ(t))|
    pub fn compute_kappa(&self, phases: &[f64]) -> f64 {
        if phases.is_empty() {
            return 0.0;
        }

        let n = phases.len() as f64;
        let sum_real: f64 = phases.iter().map(|&theta| theta.cos()).sum();
        let sum_imag: f64 = phases.iter().map(|&theta| theta.sin()).sum();

        ((sum_real / n).powi(2) + (sum_imag / n).powi(2)).sqrt()
    }

    /// Compute Proof-of-Resonance
    /// PoR(t) = (κ(t) ≥ κ* ∧ |dκ/dt| ≤ ε)
    pub fn compute_por(&mut self, current_phases: &[f64], t: f64) -> ProofOfResonanceResult {
        let kappa = self.compute_kappa(current_phases);
        self.kappa_history.push(kappa);

        // Compute derivative
        let kappa_derivative = if self.kappa_history.len() >= 2 {
            let n = self.kappa_history.len();
            (self.kappa_history[n - 1] - self.kappa_history[n - 2]).abs()
        } else {
            0.0
        };

        // Check PoR conditions
        let passed = kappa >= self.config.kappa_threshold
            && kappa_derivative <= self.config.epsilon;

        // Compute stability over window
        let stability = if self.kappa_history.len() >= self.config.stability_window {
            let window: Vec<_> = self.kappa_history
                .iter()
                .rev()
                .take(self.config.stability_window)
                .collect();
            let mean: f64 = window.iter().copied().copied().sum::<f64>() / window.len() as f64;
            let variance: f64 = window.iter()
                .map(|&&k| (k - mean).powi(2))
                .sum::<f64>() / window.len() as f64;
            1.0 - variance.sqrt().min(1.0)
        } else {
            0.5
        };

        let result = ProofOfResonanceResult {
            passed,
            kappa,
            kappa_derivative,
            stability,
            timestamp: t,
        };

        self.por_status = Some(result.clone());
        result
    }

    /// Apply Telescope Operator to focus on stable attractors
    /// T_γ(Φ) = arg max_{x∈Φ} (|dκ/dt(x,t)| < ε)
    pub fn apply_telescope(&mut self, candidates: &[OperatorCandidate]) -> Vec<OperatorCandidate> {
        let gamma = self.config.telescope_gamma;

        candidates.iter()
            .filter(|c| {
                // Focus on high-resonance, stable candidates
                c.resonance >= gamma * self.config.kappa_threshold
                    && c.stability >= gamma
            })
            .cloned()
            .collect()
    }

    /// Process candidates through Kosmokrator filter
    pub fn filter(&mut self, candidates: Vec<OperatorCandidate>, t: f64) -> Vec<OperatorCandidate> {
        self.total_processed += candidates.len();

        // Extract phases and compute PoR
        let phases: Vec<f64> = candidates.iter().map(|c| c.phase).collect();
        let por = self.compute_por(&phases, t);

        if !por.passed {
            // All candidates excluded when PoR fails
            self.excluded.extend(candidates.clone());
            return Vec::new();
        }

        // Apply telescope operator to focus
        let focused = self.apply_telescope(&candidates);

        // Separate survivors and excluded
        for c in &candidates {
            if focused.iter().any(|f| f.id == c.id) {
                self.survivors.push(c.clone());
            } else {
                self.excluded.push(c.clone());
            }
        }

        focused
    }

    /// Get filter statistics
    pub fn stats(&self) -> KosmokratorStats {
        KosmokratorStats {
            total_processed: self.total_processed,
            survivors: self.survivors.len(),
            excluded: self.excluded.len(),
            exclusion_rate: if self.total_processed > 0 {
                self.excluded.len() as f64 / self.total_processed as f64
            } else {
                0.0
            },
            current_kappa: self.kappa_history.last().copied().unwrap_or(0.0),
            por_passed: self.por_status.as_ref().map(|p| p.passed).unwrap_or(false),
        }
    }
}

/// Kosmokrator statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KosmokratorStats {
    pub total_processed: usize,
    pub survivors: usize,
    pub excluded: usize,
    pub exclusion_rate: f64,
    pub current_kappa: f64,
    pub por_passed: bool,
}

// ============================================================================
// CHRONOKRATOR - EXPANSION AXIS
// ============================================================================

/// Configuration for Chronokrator expansion stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronokratorConfig {
    /// Number of resonance channels
    pub num_channels: usize,
    /// Base threshold Θ₀
    pub base_threshold: f64,
    /// Threshold adaptation rate
    pub threshold_adaptation: f64,
    /// Central oscillator frequency
    pub omega_central: f64,
    /// Enable Exkalibration vector computation
    pub compute_exkalibration: bool,
}

impl Default for ChronokratorConfig {
    fn default() -> Self {
        Self {
            num_channels: 4,
            base_threshold: 0.75,
            threshold_adaptation: 0.1,
            omega_central: 1.0,
            compute_exkalibration: true,
        }
    }
}

/// A resonance channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceChannel {
    /// Channel index
    pub index: usize,
    /// ψ - semantic density
    pub psi: f64,
    /// ρ - structural coherence
    pub rho: f64,
    /// ω - rhythmic phase
    pub omega: f64,
    /// Λ - envelope function
    pub lambda: f64,
    /// Channel phase φ
    pub phi: f64,
    /// Circular frequency
    pub omega_circ: f64,
}

impl ResonanceChannel {
    /// Create new channel
    pub fn new(index: usize, sig: &Signature5D) -> Self {
        Self {
            index,
            psi: sig.psi,
            rho: sig.rho,
            omega: sig.omega,
            lambda: 0.5 * (1.0 + (sig.chi * PI).sin()),
            phi: sig.eta * 2.0 * PI,
            omega_circ: 1.0 + 0.1 * index as f64,
        }
    }

    /// Compute channel dynamics D_i(t) = ψ_i(t) · ρ_i(t) · ω_i(t) · Λ_i(t)
    pub fn compute_dynamics(&self, t: f64) -> f64 {
        let envelope = 0.5 * (1.0 + (self.omega_circ * t + self.phi).sin());
        self.psi * self.rho * self.omega * envelope
    }
}

/// Exkalibration vector E(t) = ∇_{ψ,ρ,ω} Φ(t)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExkalibrationVector {
    /// Gradient components
    pub gradient: [f64; 5],
    /// Magnitude
    pub magnitude: f64,
    /// Direction (normalized)
    pub direction: [f64; 5],
    /// Timestamp
    pub timestamp: f64,
    /// Is valid (threshold exceeded)
    pub valid: bool,
}

/// Chronokrator expansion state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronokratorState {
    /// Configuration
    pub config: ChronokratorConfig,
    /// Active resonance channels
    pub channels: Vec<ResonanceChannel>,
    /// D_total history
    pub d_total_history: Vec<f64>,
    /// Threshold Θ(t) history
    pub threshold_history: Vec<f64>,
    /// Exkalibration vectors
    pub exkalibration_history: Vec<ExkalibrationVector>,
    /// Current time
    pub current_t: f64,
    /// Spike events
    pub spike_events: Vec<SpikeEvent>,
}

/// A threshold spike event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikeEvent {
    pub timestamp: f64,
    pub d_total: f64,
    pub threshold: f64,
    pub exkalibration: ExkalibrationVector,
}

impl ChronokratorState {
    /// Create new Chronokrator state
    pub fn new(config: ChronokratorConfig) -> Self {
        Self {
            config,
            channels: Vec::new(),
            d_total_history: Vec::new(),
            threshold_history: Vec::new(),
            exkalibration_history: Vec::new(),
            current_t: 0.0,
            spike_events: Vec::new(),
        }
    }

    /// Initialize channels from operator candidates
    pub fn init_channels(&mut self, candidates: &[OperatorCandidate]) {
        self.channels.clear();
        for (i, c) in candidates.iter().take(self.config.num_channels).enumerate() {
            self.channels.push(ResonanceChannel::new(i, &c.signature));
        }
    }

    /// Compute central oscillator Ω(t)
    pub fn compute_omega(&self, t: f64) -> f64 {
        0.5 * (1.0 + (self.config.omega_central * t).sin())
    }

    /// Compute total dynamics D_total(t) = (∏ D_i(t)) · Ω(t)
    pub fn compute_d_total(&self, t: f64) -> f64 {
        if self.channels.is_empty() {
            return 0.0;
        }

        let channel_product: f64 = self.channels
            .iter()
            .map(|c| c.compute_dynamics(t))
            .product();

        let omega = self.compute_omega(t);

        // Use geometric mean for stability
        let n = self.channels.len() as f64;
        channel_product.powf(1.0 / n) * omega
    }

    /// Compute adaptive threshold Θ(t)
    pub fn compute_threshold(&self, t: f64) -> f64 {
        let base = self.config.base_threshold;

        // Adapt based on recent history
        if self.d_total_history.len() >= 10 {
            let recent: Vec<_> = self.d_total_history.iter().rev().take(10).collect();
            let mean: f64 = recent.iter().copied().copied().sum::<f64>() / recent.len() as f64;
            base * (1.0 + self.config.threshold_adaptation * (mean - 0.5))
        } else {
            base
        }
    }

    /// Compute Exkalibration vector E(t) = ∇_{ψ,ρ,ω} Φ(t)
    pub fn compute_exkalibration(&self, candidates: &[OperatorCandidate], t: f64) -> ExkalibrationVector {
        if candidates.is_empty() {
            return ExkalibrationVector {
                gradient: [0.0; 5],
                magnitude: 0.0,
                direction: [0.0; 5],
                timestamp: t,
                valid: false,
            };
        }

        // Compute gradient as weighted average of candidate directions
        let mut gradient = [0.0; 5];
        let mut total_weight = 0.0;

        for c in candidates {
            let weight = c.resonance;
            let sig = &c.signature;
            gradient[0] += weight * sig.psi;
            gradient[1] += weight * sig.rho;
            gradient[2] += weight * sig.omega;
            gradient[3] += weight * sig.chi;
            gradient[4] += weight * sig.eta;
            total_weight += weight;
        }

        if total_weight > 0.0 {
            for g in &mut gradient {
                *g /= total_weight;
            }
        }

        let magnitude = gradient.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
        let direction = if magnitude > 0.0 {
            gradient.map(|x| x / magnitude)
        } else {
            [0.0; 5]
        };

        let d_total = self.d_total_history.last().copied().unwrap_or(0.0);
        let threshold = self.threshold_history.last().copied().unwrap_or(self.config.base_threshold);

        ExkalibrationVector {
            gradient,
            magnitude,
            direction,
            timestamp: t,
            valid: d_total > threshold,
        }
    }

    /// Process expansion step
    pub fn expand(&mut self, candidates: &[OperatorCandidate], t: f64) -> Option<ExkalibrationVector> {
        self.current_t = t;

        // Initialize channels if needed
        if self.channels.is_empty() && !candidates.is_empty() {
            self.init_channels(candidates);
        }

        // Compute dynamics
        let d_total = self.compute_d_total(t);
        let threshold = self.compute_threshold(t);

        self.d_total_history.push(d_total);
        self.threshold_history.push(threshold);

        // Check trigger condition: D_total(t) > Θ(t)
        if d_total > threshold && self.config.compute_exkalibration {
            let exkal = self.compute_exkalibration(candidates, t);

            if exkal.valid {
                self.spike_events.push(SpikeEvent {
                    timestamp: t,
                    d_total,
                    threshold,
                    exkalibration: exkal.clone(),
                });
                self.exkalibration_history.push(exkal.clone());
                return Some(exkal);
            }
        }

        None
    }

    /// Get expansion statistics
    pub fn stats(&self) -> ChronokratorStats {
        let avg_d_total = if !self.d_total_history.is_empty() {
            self.d_total_history.iter().sum::<f64>() / self.d_total_history.len() as f64
        } else {
            0.0
        };

        ChronokratorStats {
            num_channels: self.channels.len(),
            current_d_total: self.d_total_history.last().copied().unwrap_or(0.0),
            current_threshold: self.threshold_history.last().copied().unwrap_or(0.0),
            avg_d_total,
            spike_count: self.spike_events.len(),
            current_t: self.current_t,
        }
    }
}

/// Chronokrator statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronokratorStats {
    pub num_channels: usize,
    pub current_d_total: f64,
    pub current_threshold: f64,
    pub avg_d_total: f64,
    pub spike_count: usize,
    pub current_t: f64,
}

// ============================================================================
// PFAUENTHRON / MONOLITH - DECISION CORE
// ============================================================================

/// Configuration for Pfauenthron collapse stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfauenthronConfig {
    /// Mandorla convergence threshold η
    pub mandorla_threshold: f64,
    /// Convergence derivative threshold
    pub convergence_epsilon: f64,
    /// Number of Ophanim nodes
    pub num_ophanim: usize,
    /// Enable Monolith emission
    pub emit_monolith: bool,
}

impl Default for PfauenthronConfig {
    fn default() -> Self {
        Self {
            mandorla_threshold: 0.8,
            convergence_epsilon: 0.01,
            num_ophanim: 4,
            emit_monolith: true,
        }
    }
}

/// An Ophanim resonance emitter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ophanim {
    /// Index
    pub index: usize,
    /// Local resonance signature O_i(t) = (ψ_i(t), ρ_i(t), ω_i(t))
    pub signature: Signature5D,
    /// Phase φ_i
    pub phase: f64,
    /// Envelope λ_i(t)
    pub lambda: f64,
}

impl Ophanim {
    /// Create from operator candidate
    pub fn from_candidate(index: usize, c: &OperatorCandidate) -> Self {
        Self {
            index,
            signature: c.signature,
            phase: c.phase,
            lambda: 0.5 * (1.0 + (c.signature.chi * PI).sin()),
        }
    }

    /// Compute contribution to Konus
    pub fn konus_contribution(&self, t: f64, konus_phase: f64) -> f64 {
        let phase_diff = self.phase - konus_phase;
        self.lambda * phase_diff.cos()
    }
}

/// Mandorla convergence field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandorlaField {
    /// Perception vector P_Gabriel
    pub perception: [f64; 5],
    /// Intention vector I_Oriphiel
    pub intention: [f64; 5],
    /// Convergence score S_Mandorla(t) = P · I
    pub convergence_score: f64,
    /// Score derivative
    pub score_derivative: f64,
    /// Is converged
    pub is_converged: bool,
}

/// Monolith action output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monolith {
    /// Exkalibration vector E(t) ∈ R^5
    pub exkalibration: [f64; 5],
    /// Magnitude
    pub magnitude: f64,
    /// Emergence timestamp
    pub timestamp: f64,
    /// Source Mandorla score
    pub mandorla_score: f64,
    /// Operator family assignments
    pub family_ids: Vec<String>,
}

/// Pfauenthron decision core state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfauenthronState {
    /// Configuration
    pub config: PfauenthronConfig,
    /// Ophanim array (O.P.H.A.N.)
    pub ophanim: Vec<Ophanim>,
    /// Konus phase
    pub konus_phase: f64,
    /// Konus envelope Ω(t)
    pub konus_omega: f64,
    /// Mandorla field history
    pub mandorla_history: Vec<MandorlaField>,
    /// Emitted Monoliths
    pub monoliths: Vec<Monolith>,
    /// Finalized operator families
    pub finalized_families: Vec<FinalizedFamily>,
}

/// A finalized operator family
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizedFamily {
    /// Family identifier
    pub id: String,
    /// Family name
    pub name: String,
    /// Centroid signature
    pub centroid: Signature5D,
    /// Member count
    pub member_count: usize,
    /// Average resonance
    pub avg_resonance: f64,
    /// Mandorla convergence score
    pub mandorla_score: f64,
    /// Is high-quality
    pub is_high_quality: bool,
    /// Is stable
    pub is_stable: bool,
}

impl PfauenthronState {
    /// Create new Pfauenthron state
    pub fn new(config: PfauenthronConfig) -> Self {
        Self {
            config,
            ophanim: Vec::new(),
            konus_phase: 0.0,
            konus_omega: 0.0,
            mandorla_history: Vec::new(),
            monoliths: Vec::new(),
            finalized_families: Vec::new(),
        }
    }

    /// Initialize Ophanim array from candidates
    pub fn init_ophanim(&mut self, candidates: &[OperatorCandidate]) {
        self.ophanim.clear();
        for (i, c) in candidates.iter().take(self.config.num_ophanim).enumerate() {
            self.ophanim.push(Ophanim::from_candidate(i, c));
        }
    }

    /// Compute Konus envelope Ω(t) = Σ λ_i(t) cos(φ_i(t) - φ_Konus(t))
    pub fn compute_konus(&self, t: f64) -> f64 {
        if self.ophanim.is_empty() {
            return 0.0;
        }

        self.ophanim.iter()
            .map(|o| o.konus_contribution(t, self.konus_phase))
            .sum()
    }

    /// Compute Mandorla convergence S_Mandorla(t) = P_Gabriel · I_Oriphiel
    pub fn compute_mandorla(&mut self, candidates: &[OperatorCandidate], exkal: &ExkalibrationVector, t: f64) -> MandorlaField {
        // Perception vector: average of candidate signatures
        let mut perception = [0.0; 5];
        if !candidates.is_empty() {
            for c in candidates {
                let sig = &c.signature;
                perception[0] += sig.psi;
                perception[1] += sig.rho;
                perception[2] += sig.omega;
                perception[3] += sig.chi;
                perception[4] += sig.eta;
            }
            let n = candidates.len() as f64;
            for p in &mut perception {
                *p /= n;
            }
        }

        // Intention vector: Exkalibration direction
        let intention = exkal.direction;

        // Convergence score: dot product
        let convergence_score: f64 = perception.iter()
            .zip(intention.iter())
            .map(|(p, i)| p * i)
            .sum();

        // Compute derivative
        let score_derivative = if let Some(last) = self.mandorla_history.last() {
            (convergence_score - last.convergence_score).abs()
        } else {
            0.0
        };

        // Check convergence: |S| ≥ η ∧ |dS/dt| ≈ 0
        let is_converged = convergence_score.abs() >= self.config.mandorla_threshold
            && score_derivative <= self.config.convergence_epsilon;

        let field = MandorlaField {
            perception,
            intention,
            convergence_score,
            score_derivative,
            is_converged,
        };

        self.mandorla_history.push(field.clone());
        field
    }

    /// Attempt collapse to Monolith
    pub fn collapse(&mut self, candidates: &[OperatorCandidate], exkal: &ExkalibrationVector, t: f64) -> Option<Monolith> {
        // Initialize Ophanim if needed
        if self.ophanim.is_empty() && !candidates.is_empty() {
            self.init_ophanim(candidates);
        }

        // Update Konus
        self.konus_omega = self.compute_konus(t);
        self.konus_phase += 0.1; // Advance phase

        // Compute Mandorla
        let mandorla = self.compute_mandorla(candidates, exkal, t);

        if mandorla.is_converged && self.config.emit_monolith {
            // Create Monolith
            let monolith = Monolith {
                exkalibration: exkal.gradient,
                magnitude: exkal.magnitude,
                timestamp: t,
                mandorla_score: mandorla.convergence_score,
                family_ids: Vec::new(),
            };

            self.monoliths.push(monolith.clone());
            Some(monolith)
        } else {
            None
        }
    }

    /// Finalize operator families
    pub fn finalize_families(&mut self, candidates: &[OperatorCandidate]) {
        // Group candidates into families based on signature similarity
        let mut families: Vec<Vec<&OperatorCandidate>> = Vec::new();
        let threshold = 0.2;

        for c in candidates {
            let mut found = false;
            for family in &mut families {
                if let Some(first) = family.first() {
                    let dist = c.signature.distance(&first.signature);
                    if dist < threshold {
                        family.push(c);
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                families.push(vec![c]);
            }
        }

        // Convert to finalized families
        self.finalized_families.clear();
        for (i, family) in families.iter().enumerate() {
            if family.is_empty() {
                continue;
            }

            // Compute centroid
            let mut centroid = Signature5D::zero();
            let n = family.len() as f64;
            for c in family {
                centroid.psi += c.signature.psi / n;
                centroid.rho += c.signature.rho / n;
                centroid.omega += c.signature.omega / n;
                centroid.chi += c.signature.chi / n;
                centroid.eta += c.signature.eta / n;
            }

            let avg_resonance: f64 = family.iter().map(|c| c.resonance).sum::<f64>() / n;
            let mandorla_score = self.mandorla_history.last()
                .map(|m| m.convergence_score)
                .unwrap_or(0.0);

            self.finalized_families.push(FinalizedFamily {
                id: format!("family_{}", i),
                name: format!("Family {}", i + 1),
                centroid,
                member_count: family.len(),
                avg_resonance,
                mandorla_score,
                is_high_quality: centroid.psi >= 0.7,
                is_stable: centroid.rho >= 0.7,
            });
        }
    }

    /// Get Pfauenthron statistics
    pub fn stats(&self) -> PfauenthronStats {
        let current_mandorla = self.mandorla_history.last()
            .map(|m| m.convergence_score)
            .unwrap_or(0.0);

        PfauenthronStats {
            num_ophanim: self.ophanim.len(),
            konus_omega: self.konus_omega,
            current_mandorla,
            monolith_count: self.monoliths.len(),
            family_count: self.finalized_families.len(),
            is_converged: self.mandorla_history.last()
                .map(|m| m.is_converged)
                .unwrap_or(false),
        }
    }
}

/// Pfauenthron statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PfauenthronStats {
    pub num_ophanim: usize,
    pub konus_omega: f64,
    pub current_mandorla: f64,
    pub monolith_count: usize,
    pub family_count: usize,
    pub is_converged: bool,
}

// ============================================================================
// HOLISTIC MATRIX - UNIFIED PIPELINE
// ============================================================================

/// Holistic Resonance Matrix configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolisticConfig {
    pub kosmokrator: KosmokratorConfig,
    pub chronokrator: ChronokratorConfig,
    pub pfauenthron: PfauenthronConfig,
}

impl Default for HolisticConfig {
    fn default() -> Self {
        Self {
            kosmokrator: KosmokratorConfig::default(),
            chronokrator: ChronokratorConfig::default(),
            pfauenthron: PfauenthronConfig::default(),
        }
    }
}

/// Matrix output: M(t) = E(t) if PoR(t) ∧ D_total(t) > Θ(t), else ∅
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixOutput {
    /// Whether output is valid
    pub valid: bool,
    /// Exkalibration vector (if valid)
    pub exkalibration: Option<ExkalibrationVector>,
    /// Monolith (if collapsed)
    pub monolith: Option<Monolith>,
    /// Current stage
    pub stage: GenesisStage,
    /// Timestamp
    pub timestamp: f64,
}

/// The Holistic Resonance Matrix
///
/// Combines Kosmokrator (exclusion), Chronokrator (expansion), and
/// Pfauenthron (finalization) into a unified decision pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolisticMatrix {
    pub config: HolisticConfig,
    pub kosmokrator: KosmokratorState,
    pub chronokrator: ChronokratorState,
    pub pfauenthron: PfauenthronState,
    pub current_stage: GenesisStage,
    pub output_history: Vec<MatrixOutput>,
}

impl HolisticMatrix {
    /// Create new Holistic Matrix
    pub fn new(config: HolisticConfig) -> Self {
        Self {
            kosmokrator: KosmokratorState::new(config.kosmokrator.clone()),
            chronokrator: ChronokratorState::new(config.chronokrator.clone()),
            pfauenthron: PfauenthronState::new(config.pfauenthron.clone()),
            config,
            current_stage: GenesisStage::Discovery,
            output_history: Vec::new(),
        }
    }

    /// Process candidates through the full pipeline
    /// M(t) = { E(t) if PoR(t) = true ∧ D_total(t) > Θ(t), ∅ otherwise }
    pub fn process(&mut self, candidates: Vec<OperatorCandidate>, t: f64) -> MatrixOutput {
        // Stage 1: Kosmokrator Filter (Exclusion)
        self.current_stage = GenesisStage::KosmokratorFilter;
        let survivors = self.kosmokrator.filter(candidates, t);

        if survivors.is_empty() {
            return self.emit_empty(t);
        }

        // Stage 2: Chronokrator Expansion
        self.current_stage = GenesisStage::ChronokratorExpansion;
        let exkal = self.chronokrator.expand(&survivors, t);

        let exkal = match exkal {
            Some(e) if e.valid => e,
            _ => return self.emit_empty(t),
        };

        // Stage 3: Pfauenthron Collapse
        self.current_stage = GenesisStage::PfauenthronCollapse;
        let monolith = self.pfauenthron.collapse(&survivors, &exkal, t);

        // Finalize families
        self.pfauenthron.finalize_families(&survivors);
        self.current_stage = GenesisStage::Finalized;

        let output = MatrixOutput {
            valid: true,
            exkalibration: Some(exkal),
            monolith,
            stage: self.current_stage,
            timestamp: t,
        };

        self.output_history.push(output.clone());
        output
    }

    fn emit_empty(&mut self, t: f64) -> MatrixOutput {
        let output = MatrixOutput {
            valid: false,
            exkalibration: None,
            monolith: None,
            stage: self.current_stage,
            timestamp: t,
        };
        self.output_history.push(output.clone());
        output
    }

    /// Get combined statistics
    pub fn stats(&self) -> HolisticStats {
        HolisticStats {
            current_stage: self.current_stage,
            kosmokrator: self.kosmokrator.stats(),
            chronokrator: self.chronokrator.stats(),
            pfauenthron: self.pfauenthron.stats(),
            valid_outputs: self.output_history.iter().filter(|o| o.valid).count(),
            total_outputs: self.output_history.len(),
        }
    }

    /// Get finalized families
    pub fn families(&self) -> &[FinalizedFamily] {
        &self.pfauenthron.finalized_families
    }

    /// Reset the matrix for a new session
    pub fn reset(&mut self) {
        self.kosmokrator = KosmokratorState::new(self.config.kosmokrator.clone());
        self.chronokrator = ChronokratorState::new(self.config.chronokrator.clone());
        self.pfauenthron = PfauenthronState::new(self.config.pfauenthron.clone());
        self.current_stage = GenesisStage::Discovery;
        self.output_history.clear();
    }
}

/// Combined statistics for the Holistic Matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolisticStats {
    pub current_stage: GenesisStage,
    pub kosmokrator: KosmokratorStats,
    pub chronokrator: ChronokratorStats,
    pub pfauenthron: PfauenthronStats,
    pub valid_outputs: usize,
    pub total_outputs: usize,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_candidate(id: &str, resonance: f64) -> OperatorCandidate {
        OperatorCandidate {
            id: id.to_string(),
            signature: Signature5D::new(resonance, resonance, resonance, 0.5, 0.3),
            phase: resonance * PI,
            resonance,
            stability: resonance,
            is_mandorla: resonance >= 0.85,
            node_index: 0,
            discovered_at: 0.0,
        }
    }

    #[test]
    fn test_kosmokrator_kappa() {
        let state = KosmokratorState::new(KosmokratorConfig::default());

        // Perfectly aligned phases -> κ = 1
        let phases = vec![0.0, 0.0, 0.0, 0.0];
        assert!((state.compute_kappa(&phases) - 1.0).abs() < 0.01);

        // Opposite phases -> κ = 0
        let phases = vec![0.0, PI, 0.0, PI];
        assert!(state.compute_kappa(&phases) < 0.01);
    }

    #[test]
    fn test_kosmokrator_filter() {
        let mut state = KosmokratorState::new(KosmokratorConfig {
            kappa_threshold: 0.5,
            ..Default::default()
        });

        let candidates: Vec<_> = (0..5)
            .map(|i| create_test_candidate(&format!("op_{}", i), 0.8))
            .collect();

        let survivors = state.filter(candidates, 1.0);
        assert!(!survivors.is_empty());
    }

    #[test]
    fn test_chronokrator_dynamics() {
        let mut state = ChronokratorState::new(ChronokratorConfig::default());

        let candidates: Vec<_> = (0..4)
            .map(|i| create_test_candidate(&format!("op_{}", i), 0.8))
            .collect();

        state.init_channels(&candidates);
        let d_total = state.compute_d_total(1.0);
        assert!(d_total > 0.0);
    }

    #[test]
    fn test_pfauenthron_mandorla() {
        let mut state = PfauenthronState::new(PfauenthronConfig::default());

        let candidates: Vec<_> = (0..4)
            .map(|i| create_test_candidate(&format!("op_{}", i), 0.9))
            .collect();

        let exkal = ExkalibrationVector {
            gradient: [0.9, 0.8, 0.7, 0.6, 0.5],
            magnitude: 1.5,
            direction: [0.6, 0.53, 0.47, 0.4, 0.33],
            timestamp: 1.0,
            valid: true,
        };

        let mandorla = state.compute_mandorla(&candidates, &exkal, 1.0);
        assert!(mandorla.convergence_score > 0.0);
    }

    #[test]
    fn test_holistic_matrix() {
        let config = HolisticConfig {
            kosmokrator: KosmokratorConfig {
                kappa_threshold: 0.3,
                ..Default::default()
            },
            chronokrator: ChronokratorConfig {
                base_threshold: 0.1,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut matrix = HolisticMatrix::new(config);

        let candidates: Vec<_> = (0..10)
            .map(|i| create_test_candidate(&format!("op_{}", i), 0.85))
            .collect();

        let output = matrix.process(candidates, 1.0);
        // Output validity depends on convergence criteria
        assert!(matrix.stats().total_outputs == 1);
    }
}
