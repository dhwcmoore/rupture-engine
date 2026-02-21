use rupture_engine::config::StateMachineConfig;
use rupture_engine::model::state_machine::StateMachine;
use rupture_engine::types::RuptureState;

fn default_cfg() -> StateMachineConfig {
    StateMachineConfig {
        rho_stressed: 0.60,
        rho_critical: 0.85,
        rho_rupture: 1.00,
        confirm_k: 3,
        confirm_m: 2,
        enable_recovery: true,
        recovery_hold: 2,
    }
}

#[test]
fn test_stable_below_threshold() {
    let cfg = default_cfg();
    let mut sm = StateMachine::new(cfg);
    let rhos = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    for (i, &rho) in rhos.iter().enumerate() {
        let (state, cand, conf) = sm.update(i, &format!("t{}", i), rho);
        assert_eq!(state, RuptureState::Stable);
        assert!(!cand);
        assert!(!conf);
    }
}

#[test]
fn test_stressed_state() {
    let cfg = default_cfg();
    let mut sm = StateMachine::new(cfg);
    let (state, _, _) = sm.update(0, "t0", 0.3);
    assert_eq!(state, RuptureState::Stable);
    let (state, _, _) = sm.update(1, "t1", 0.7);
    assert_eq!(state, RuptureState::Stressed);
}

#[test]
fn test_critical_state() {
    let cfg = default_cfg();
    let mut sm = StateMachine::new(cfg);
    let (state, _, _) = sm.update(0, "t0", 0.3);
    assert_eq!(state, RuptureState::Stable);
    let (state, _, _) = sm.update(1, "t1", 0.9);
    assert_eq!(state, RuptureState::Critical);
}

#[test]
fn test_single_spike_no_confirmation() {
    // A single bar above 1.0 then dropping below should not confirm for k=3, m=2.
    let cfg = default_cfg();
    let mut sm = StateMachine::new(cfg);

    // Build up to below threshold.
    sm.update(0, "t0", 0.5);
    let (state, cand, _) = sm.update(1, "t1", 1.2);
    assert_eq!(state, RuptureState::CandidateRupture);
    assert!(cand);

    // Next two bars below threshold.
    let (_, _, conf) = sm.update(2, "t2", 0.8);
    assert!(!conf);
    let (state, _, conf) = sm.update(3, "t3", 0.7);
    assert!(!conf);
    // State should have reverted after confirmation window completes.
    assert_ne!(state, RuptureState::ConfirmedRupture);

    // Only one event, and it should be unconfirmed.
    let events = sm.events();
    assert_eq!(events.len(), 1);
    assert!(events[0].confirmed_index.is_none());
}

#[test]
fn test_sustained_above_confirms() {
    // Sustained above 1.0 for k bars should confirm with m-of-k.
    let cfg = default_cfg();
    let mut sm = StateMachine::new(cfg);

    sm.update(0, "t0", 0.5);
    let (state, cand, _) = sm.update(1, "t1", 1.5);
    assert_eq!(state, RuptureState::CandidateRupture);
    assert!(cand);

    // Next bars above threshold (total of k=3 bars including the candidate).
    sm.update(2, "t2", 1.3);
    let (state, _, conf) = sm.update(3, "t3", 1.1);
    assert_eq!(state, RuptureState::ConfirmedRupture);
    assert!(conf);

    let events = sm.events();
    let confirmed: Vec<_> = events.iter().filter(|e| e.confirmed_index.is_some()).collect();
    assert_eq!(confirmed.len(), 1);
    assert_eq!(confirmed[0].candidate_index, 1);
}

#[test]
fn test_recovery_after_confirmed() {
    let cfg = default_cfg();
    let mut sm = StateMachine::new(cfg);

    // Enter confirmed rupture.
    sm.update(0, "t0", 0.5);
    sm.update(1, "t1", 1.5);
    sm.update(2, "t2", 1.3);
    sm.update(3, "t3", 1.1);

    // Now drop below rho_critical for recovery_hold bars.
    let (state, _, _) = sm.update(4, "t4", 0.7);
    assert_eq!(state, RuptureState::ConfirmedRupture);
    let (state, _, _) = sm.update(5, "t5", 0.6);
    assert_eq!(state, RuptureState::Recovery);

    // After recovery, should return to base state.
    let (state, _, _) = sm.update(6, "t6", 0.3);
    assert_eq!(state, RuptureState::Stable);
}

#[test]
fn test_flapping_case() {
    // Rho oscillates around 1.0. With m=2 of k=3, exactly 1-of-3 above should not confirm.
    let cfg = default_cfg();
    let mut sm = StateMachine::new(cfg);

    sm.update(0, "t0", 0.5);
    sm.update(1, "t1", 1.1); // candidate
    sm.update(2, "t2", 0.9); // below
    let (state, _, conf) = sm.update(3, "t3", 0.8); // below => 1-of-3, no confirm
    assert!(!conf);
    assert_ne!(state, RuptureState::ConfirmedRupture);
}
