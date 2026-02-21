use crate::config::Config;
use crate::features::{compute_acceleration, compute_normed, compute_returns};
use crate::model::capacity::CapacityEstimator;
use crate::model::combine::soft_max_combine;
use crate::model::memory::MemoryKernel;
use crate::model::residuals::compute_residuals;
use crate::model::state_machine::StateMachine;
use crate::types::{Bar, EngineOutputRow, RuptureEvent};

/// Run the full engine pipeline on a sequence of bars and return the per-bar
/// output rows and the event log.
pub fn run_engine(bars: &[Bar], config: &Config) -> (Vec<EngineOutputRow>, Vec<RuptureEvent>) {
    let returns = compute_returns(bars, config.features.use_log_returns);
    let accel = compute_acceleration(&returns);
    let normed = compute_normed(
        bars,
        &returns,
        &accel,
        config.windows.robust_scale_n,
        config.robust.mad_scale_factor,
        config.numerics.eps,
    );

    let mut memory = MemoryKernel::new(config.windows.memory_k, config.memory.alpha);
    let mut capacity = CapacityEstimator::new(
        config.windows.capacity_l,
        config.capacity.q,
        config.capacity.smooth,
        config.capacity.lambda,
        config.capacity.e_min,
    );
    let mut state_machine = StateMachine::new(config.state_machine.clone());

    let mut rows = Vec::with_capacity(bars.len());

    for i in 0..bars.len() {
        let mut res = compute_residuals(&normed[i], &config.residuals, &config.numerics);
        res.r = soft_max_combine(res.r_vol, res.r_liq, res.r_acc, config.combine.tau);

        let strain = memory.push_and_accumulate(res.r);
        let cap = capacity.update(strain);
        let rho = strain / (cap + config.numerics.eps);

        let (state, candidate_flag, confirmed_flag) =
            state_machine.update(i, &bars[i].ts, rho);

        rows.push(EngineOutputRow {
            timestamp: bars[i].ts.clone(),
            close: bars[i].close,
            volume: bars[i].volume,
            r_vol: res.r_vol,
            r_liq: res.r_liq,
            r_acc: res.r_acc,
            r_combined: res.r,
            strain,
            capacity: cap,
            rho,
            state: state.to_string(),
            candidate_flag,
            confirmed_flag,
        });
    }

    let events = state_machine.events().to_vec();
    (rows, events)
}
