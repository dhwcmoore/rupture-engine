use crate::config::StateMachineConfig;
use crate::types::{RuptureEvent, RuptureState};

/// Deterministic state machine for rupture detection.
/// Transitions are driven by rho = S / (E + eps).
/// Candidate ruptures are detected at the first crossing of rho >= 1.0,
/// and confirmed using an m-of-k rule over subsequent bars.
pub struct StateMachine {
    cfg: StateMachineConfig,
    state: RuptureState,
    prev_rho: f64,

    // Candidate tracking.
    candidate_index: Option<usize>,
    candidate_timestamp: Option<String>,
    candidate_peak_rho: f64,
    confirm_buffer: Vec<bool>,

    // Recovery tracking.
    recovery_count: usize,

    // Event log.
    events: Vec<RuptureEvent>,
}

impl StateMachine {
    pub fn new(cfg: StateMachineConfig) -> Self {
        Self {
            cfg,
            state: RuptureState::Stable,
            prev_rho: 0.0,
            candidate_index: None,
            candidate_timestamp: None,
            candidate_peak_rho: 0.0,
            confirm_buffer: Vec::new(),
            recovery_count: 0,
            events: Vec::new(),
        }
    }

    /// Update the state machine with a new rho value. Returns the current state
    /// and flags indicating whether this bar is a candidate or confirmed rupture.
    pub fn update(
        &mut self,
        index: usize,
        timestamp: &str,
        rho: f64,
    ) -> (RuptureState, bool, bool) {
        let mut candidate_flag = false;
        let mut confirmed_flag = false;

        match self.state {
            RuptureState::Stable | RuptureState::Stressed | RuptureState::Critical => {
                if rho >= self.cfg.rho_rupture && self.prev_rho < self.cfg.rho_rupture {
                    // First crossing: enter candidate state.
                    self.state = RuptureState::CandidateRupture;
                    self.candidate_index = Some(index);
                    self.candidate_timestamp = Some(timestamp.to_string());
                    self.candidate_peak_rho = rho;
                    self.confirm_buffer.clear();
                    self.confirm_buffer.push(true); // The crossing bar itself counts.
                    candidate_flag = true;
                } else {
                    // Normal state assignment by rho level.
                    self.state = base_state(rho, &self.cfg);
                }
            }
            RuptureState::CandidateRupture => {
                // We are in the confirmation window.
                self.confirm_buffer
                    .push(rho >= self.cfg.rho_rupture);
                if rho > self.candidate_peak_rho {
                    self.candidate_peak_rho = rho;
                }

                if self.confirm_buffer.len() >= self.cfg.confirm_k {
                    let above_count = self.confirm_buffer.iter().filter(|&&b| b).count();
                    if above_count >= self.cfg.confirm_m {
                        // Confirmed rupture.
                        self.state = RuptureState::ConfirmedRupture;
                        confirmed_flag = true;
                        self.events.push(RuptureEvent {
                            candidate_index: self.candidate_index.unwrap_or(index),
                            candidate_timestamp: self
                                .candidate_timestamp
                                .clone()
                                .unwrap_or_default(),
                            confirmed_index: Some(index),
                            confirmed_timestamp: Some(timestamp.to_string()),
                            peak_rho: self.candidate_peak_rho,
                            confirmation_k: self.cfg.confirm_k,
                            confirmation_m: self.cfg.confirm_m,
                        });
                        self.recovery_count = 0;
                    } else {
                        // Failed confirmation: revert to base state.
                        self.events.push(RuptureEvent {
                            candidate_index: self.candidate_index.unwrap_or(index),
                            candidate_timestamp: self
                                .candidate_timestamp
                                .clone()
                                .unwrap_or_default(),
                            confirmed_index: None,
                            confirmed_timestamp: None,
                            peak_rho: self.candidate_peak_rho,
                            confirmation_k: self.cfg.confirm_k,
                            confirmation_m: self.cfg.confirm_m,
                        });
                        self.state = base_state(rho, &self.cfg);
                    }
                    self.candidate_index = None;
                    self.candidate_timestamp = None;
                    self.confirm_buffer.clear();
                }
            }
            RuptureState::ConfirmedRupture => {
                if self.cfg.enable_recovery && rho < self.cfg.rho_critical {
                    self.recovery_count += 1;
                    if self.recovery_count >= self.cfg.recovery_hold {
                        self.state = RuptureState::Recovery;
                        self.recovery_count = 0;
                    }
                } else {
                    self.recovery_count = 0;
                }
            }
            RuptureState::Recovery => {
                // Transition back to base state.
                self.state = base_state(rho, &self.cfg);
            }
        }

        self.prev_rho = rho;
        (self.state, candidate_flag, confirmed_flag)
    }

    /// Return the accumulated event log.
    pub fn events(&self) -> &[RuptureEvent] {
        &self.events
    }
}

/// Determine the base state from rho without candidate/confirmation logic.
fn base_state(rho: f64, cfg: &StateMachineConfig) -> RuptureState {
    if rho >= cfg.rho_rupture {
        RuptureState::Critical
    } else if rho >= cfg.rho_critical {
        RuptureState::Critical
    } else if rho >= cfg.rho_stressed {
        RuptureState::Stressed
    } else {
        RuptureState::Stable
    }
}
