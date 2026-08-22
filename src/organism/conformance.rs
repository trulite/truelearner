//! Post-M8 conformance surfaces.
//!
//! The affordance fixture is an evaluator around ordinary substrate events;
//! it does not enter route information into the substrate. The cumulative
//! fingerprint calls only the frozen development GATE, never a spent
//! definitive runner.

use std::collections::BTreeMap;

use super::substrate::{ArrowSpec, CellId, CellSpec, Execution, SpikeInput, Substrate};
use crate::post_m7_ds5_closure_emission;
use crate::research_runtime::HarnessMode;

const THRESHOLD: i32 = 4;
const INHIBITION: i32 = -64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Delivery {
    tick: i32,
    phase: i32,
    impulse: i32,
}

impl Delivery {
    const fn at(tick: i32) -> Self {
        Self {
            tick,
            phase: 0,
            impulse: 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EvaluatedExecution {
    realized: Option<usize>,
    execution: Execution,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AffordanceFingerprint {
    pub exact_complete_state_replay: bool,
    pub early_support_changes_trajectory: bool,
    pub late_support_is_inert: bool,
    pub same_count_different_early_support_differs: bool,
    pub different_count_same_early_support_matches: bool,
    pub impulse_and_spacing_are_physical: bool,
    pub mirrored: bool,
    pub allocation_independent: bool,
    pub blocked_and_stale_paths_cannot_win: bool,
    pub naturally_quiescent: bool,
}

impl AffordanceFingerprint {
    pub fn passed(&self) -> bool {
        self.exact_complete_state_replay
            && self.early_support_changes_trajectory
            && self.late_support_is_inert
            && self.same_count_different_early_support_differs
            && self.different_count_same_early_support_matches
            && self.impulse_and_spacing_are_physical
            && self.mirrored
            && self.allocation_independent
            && self.blocked_and_stale_paths_cannot_win
            && self.naturally_quiescent
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct M8Fingerprint {
    pub development_ready: bool,
    pub first_collapse: &'static str,
    pub learners: usize,
    pub ready_learners: usize,
    pub single_role_learners: usize,
    pub m7_activations: usize,
    pub selections: usize,
    pub consequences: usize,
    pub updates: usize,
    pub observations: u64,
    pub crossings: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub quiescent: usize,
    pub positions: usize,
    pub depths: usize,
    pub route_handles: usize,
    pub allocation_layouts: usize,
    pub physical_work: u64,
    pub controls_passed: usize,
    pub duplicate_exact: bool,
    pub held_out_nonplastic: bool,
    pub temporary_erased: bool,
    pub source_exact: bool,
}

impl M8Fingerprint {
    pub fn matches_frozen_gate(&self) -> bool {
        self.development_ready
            && self.first_collapse == "NONE"
            && self.learners == 6
            && self.ready_learners == 6
            && self.single_role_learners == 6
            && self.m7_activations == 352
            && self.selections == 160
            && self.consequences == 36
            && self.updates == 18
            && self.observations == 36
            && self.crossings == 228
            && self.held_out_correct == 192
            && self.held_out_total == 192
            && self.quiescent == 192
            && self.positions == 6
            && self.depths == 7
            && self.route_handles == 192
            && self.allocation_layouts == 2
            && self.physical_work == 391_628
            && self.controls_passed == 12
            && self.duplicate_exact
            && self.held_out_nonplastic
            && self.temporary_erased
            && self.source_exact
    }
}

pub fn replay_m8_gate() -> M8Fingerprint {
    let report = post_m7_ds5_closure_emission::run_development(HarnessMode::Gate);
    M8Fingerprint {
        development_ready: report.development_ready,
        first_collapse: report.first_collapse,
        learners: report.learners,
        ready_learners: report.ready_learners,
        single_role_learners: report.single_role_learners,
        m7_activations: report.m7_activations,
        selections: report.selections,
        consequences: report.consequences,
        updates: report.updates,
        observations: report.m6_observations,
        crossings: report.crossings,
        held_out_correct: report.held_out_correct,
        held_out_total: report.held_out_total,
        quiescent: report.natural_quiescence,
        positions: report.positions,
        depths: report.depths,
        route_handles: report.route_handles,
        allocation_layouts: report.allocation_layouts,
        physical_work: report.physical_work,
        controls_passed: report
            .controls
            .iter()
            .filter(|control| control.passed)
            .count(),
        duplicate_exact: report.duplicate_exact,
        held_out_nonplastic: report.m7_nonplastic && report.closure_nonplastic,
        temporary_erased: report.temporary_erased,
        source_exact: report.source.passed(),
    }
}

pub fn replay_affordance_law() -> AffordanceFingerprint {
    let baseline = [
        vec![
            Delivery::at(1),
            Delivery::at(3),
            Delivery::at(5),
            Delivery::at(7),
        ],
        vec![
            Delivery::at(1),
            Delivery::at(3),
            Delivery::at(5),
            Delivery::at(7),
        ],
    ];
    let exact_first = run(baseline.clone(), false, false, [false; 2], [false; 2]);
    let exact_second = run(baseline.clone(), false, false, [false; 2], [false; 2]);

    let mut early = baseline.clone();
    early[1].push(Delivery::at(6));
    let early = run(early, false, false, [false; 2], [false; 2]);

    let mut late = baseline.clone();
    late[1].push(Delivery::at(8));
    let late = run(late.clone(), false, false, [false; 2], [false; 2]);

    let mut more_late = baseline.clone();
    more_late[1].extend([Delivery::at(8), Delivery::at(9), Delivery::at(10)]);
    let more_late = run(more_late, false, false, [false; 2], [false; 2]);

    let mut strong_early = baseline.clone();
    strong_early[1].push(Delivery {
        tick: 5,
        phase: 50,
        impulse: 2,
    });
    let strong_early = run(strong_early, false, false, [false; 2], [false; 2]);
    let mut spaced_early = baseline.clone();
    spaced_early[1].push(Delivery::at(2));
    let spaced_early = run(spaced_early, false, false, [false; 2], [false; 2]);

    let mirrored_baseline = run(baseline.clone(), true, false, [false; 2], [false; 2]);
    let mut mirrored_early_deliveries = baseline.clone();
    mirrored_early_deliveries[0].push(Delivery::at(6));
    let mirrored_early = run(
        mirrored_early_deliveries,
        true,
        false,
        [false; 2],
        [false; 2],
    );
    let allocation_permuted = run(baseline.clone(), false, true, [false; 2], [false; 2]);

    let blocked = run(baseline.clone(), false, false, [true, false], [false; 2]);
    let stale = run(baseline, false, false, [false; 2], [true, false]);

    AffordanceFingerprint {
        exact_complete_state_replay: exact_first == exact_second,
        early_support_changes_trajectory: exact_first.realized == Some(0)
            && early.realized == Some(1),
        late_support_is_inert: same_realization_and_effect_trace(&exact_first, &late),
        same_count_different_early_support_differs: early.realized != late.realized,
        different_count_same_early_support_matches: late.realized == more_late.realized,
        impulse_and_spacing_are_physical: strong_early.realized == Some(1)
            && spaced_early.realized == Some(1),
        mirrored: mirrored_baseline.realized == Some(1) && mirrored_early.realized == Some(0),
        allocation_independent: exact_first.realized == allocation_permuted.realized
            && exact_first.execution.trace == allocation_permuted.execution.trace,
        blocked_and_stale_paths_cannot_win: blocked.realized == Some(1)
            && stale.realized == Some(1),
        naturally_quiescent: [
            &exact_first,
            &early,
            &late,
            &more_late,
            &strong_early,
            &spaced_early,
            &mirrored_baseline,
            &mirrored_early,
            &allocation_permuted,
            &blocked,
            &stale,
        ]
        .iter()
        .all(|run| run.execution.naturally_quiescent),
    }
}

fn same_realization_and_effect_trace(
    left: &EvaluatedExecution,
    right: &EvaluatedExecution,
) -> bool {
    left.realized == right.realized
        && left
            .execution
            .fired
            .iter()
            .filter(|physical| **physical == 40 || **physical == 50)
            .eq(right
                .execution
                .fired
                .iter()
                .filter(|physical| **physical == 40 || **physical == 50))
}

fn run(
    deliveries: [Vec<Delivery>; 2],
    mirrored: bool,
    reverse_allocation: bool,
    blocked: [bool; 2],
    stale: [bool; 2],
) -> EvaluatedExecution {
    let source = 10;
    let contenders = if mirrored { [30, 20] } else { [20, 30] };
    let effects = if mirrored { [50, 40] } else { [40, 50] };
    let mut cells = vec![(source, 1, 0)];
    cells.extend(contenders.iter().copied().map(|id| (id, THRESHOLD, 0)));
    cells.extend(effects.iter().copied().map(|id| (id, 1, 1)));
    for (route, route_deliveries) in deliveries.iter().enumerate() {
        for ordinal in 0..route_deliveries.len() {
            let physical = if mirrored {
                200 - route as u64 * 100 + ordinal as u64
            } else {
                100 + route as u64 * 100 + ordinal as u64
            };
            cells.push((physical, 1, 0));
        }
    }
    if reverse_allocation {
        cells.reverse();
    }

    let mut substrate = Substrate::new();
    let mut ids = BTreeMap::new();
    for (physical_id, threshold, region) in cells {
        let id = substrate.add_cell(CellSpec {
            physical_id,
            position: physical_id as i32,
            region,
            threshold,
            state: 0,
            generation: 1,
            resistance: 16,
        });
        ids.insert(physical_id, id);
    }
    let id = |physical: u64, ids: &BTreeMap<u64, CellId>| ids[&physical];
    let mut arrows = Vec::new();
    for (route, route_deliveries) in deliveries.iter().enumerate() {
        for (ordinal, delivery) in route_deliveries.iter().enumerate() {
            let relay = if mirrored {
                200 - route as u64 * 100 + ordinal as u64
            } else {
                100 + route as u64 * 100 + ordinal as u64
            };
            let resistance = if blocked[route] { 0 } else { 16 };
            let generation = if stale[route] { 2 } else { 1 };
            arrows.push(ArrowSpec {
                from: id(source, &ids),
                to: id(relay, &ids),
                delay: delivery.tick,
                transient_delay: 0,
                phase: 0,
                coupling: 1,
                generation,
                resistance,
            });
            arrows.push(ArrowSpec {
                from: id(relay, &ids),
                to: id(contenders[route], &ids),
                delay: 0,
                transient_delay: 0,
                phase: delivery.phase,
                coupling: delivery.impulse,
                generation,
                resistance,
            });
        }
    }
    for route in 0..2 {
        arrows.push(ArrowSpec {
            from: id(contenders[route], &ids),
            to: id(effects[route], &ids),
            delay: 0,
            transient_delay: 0,
            phase: 0,
            coupling: 1,
            generation: 1,
            resistance: 16,
        });
        arrows.push(ArrowSpec {
            from: id(contenders[route], &ids),
            to: id(contenders[1 - route], &ids),
            delay: 0,
            transient_delay: 0,
            phase: -100,
            coupling: INHIBITION,
            generation: 1,
            resistance: 16,
        });
    }
    if reverse_allocation {
        arrows.reverse();
    }
    for arrow in arrows {
        substrate.add_arrow(arrow);
    }
    substrate.enter(SpikeInput {
        arrival_tick: 0,
        phase: 0,
        origin_physical: 1,
        target: id(source, &ids),
        impulse: 1,
    });
    let execution = substrate.propagate();
    let route = effects
        .iter()
        .position(|effect| execution.fired.contains(effect));
    EvaluatedExecution {
        realized: route,
        execution,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_substrate_replays_the_authoritative_causal_window() {
        let fingerprint = replay_affordance_law();
        assert!(fingerprint.passed(), "{fingerprint:#?}");
    }

    #[test]
    fn definitive_surface_remains_refused() {
        assert!(post_m7_ds5_closure_emission::definitive_rejected());
    }

    #[test]
    #[ignore = "explicit six-seed cumulative cleanup replay"]
    fn cumulative_m8_gate_matches_frozen_fingerprint() {
        let fingerprint = replay_m8_gate();
        assert!(fingerprint.matches_frozen_gate(), "{fingerprint:#?}");
    }
}
