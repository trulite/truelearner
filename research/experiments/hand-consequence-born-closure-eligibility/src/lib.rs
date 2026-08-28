#![forbid(unsafe_code)]

use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;
use truelearner_core::{
    Checkpoint, Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent,
    Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const EXPECTED_PARENT_SHA256: &str =
    "9d1eb1461931fa6159e3955d39edd814e43531f266c2be66c184b2f0df2a8915";
const FROZEN_PARENT: &str =
    include_str!("../../../campaigns/hand-lineage-construction-selectivity-v1/convergence.toml");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    InheritedSelectivityReference,
    TemporalOnlyCounterexample,
    TemporalCohortCounterexample,
    EligibleReturnComposition,
    BoundaryNoveltyAfterTemporalGate,
}

impl Arm {
    pub const ALL: [Self; 5] = [
        Self::InheritedSelectivityReference,
        Self::TemporalOnlyCounterexample,
        Self::TemporalCohortCounterexample,
        Self::EligibleReturnComposition,
        Self::BoundaryNoveltyAfterTemporalGate,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::InheritedSelectivityReference => "inherited-selectivity-reference",
            Self::TemporalOnlyCounterexample => "temporal-only-counterexample",
            Self::TemporalCohortCounterexample => "temporal-cohort-counterexample",
            Self::EligibleReturnComposition => "eligible-return-composition",
            Self::BoundaryNoveltyAfterTemporalGate => "boundary-novelty-after-temporal-gate",
        }
    }
}

impl FromStr for Arm {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|arm| arm.id() == value)
            .ok_or(())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ProbeResult {
    schema: &'static str,
    pub arm: &'static str,
    pub outcome: &'static str,
    pub observations: serde_json::Value,
    pub falsifier: Option<String>,
    pub exact_replay: bool,
    pub naturally_quiescent: bool,
}

fn result(
    arm: Arm,
    outcome: &'static str,
    observations: serde_json::Value,
    falsifier: Option<&str>,
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "hand-consequence-born-closure-eligibility/v1",
        arm: arm.id(),
        outcome,
        observations,
        falsifier: falsifier.map(str::to_owned),
        exact_replay,
        naturally_quiescent,
    }
}

#[derive(Clone)]
struct FixtureCheckpoint(Checkpoint);

struct Fixture {
    harness: Harness,
    action: JunctionId,
    first_surface: JunctionId,
    second_surface: JunctionId,
    disconnected: JunctionId,
    motor: JunctionId,
}

impl Fixture {
    fn new(protocol: Protocol) -> Self {
        let mut builder = HarnessBuilder::with_capacity(192, 768, OUTWARD_REGION);
        builder.set_protocol(protocol);
        builder.set_physical_tracing(true);
        let action = junction(&mut builder, 60_000, 0, 0, 1);
        let first_surface = junction(&mut builder, 60_001, 2, 0, 1);
        let second_surface = junction(&mut builder, 60_002, 3, 0, 1);
        let disconnected = junction(&mut builder, 60_003, 20, 0, 1);
        let motor = junction(&mut builder, 60_010, 1, 0, 2);
        let sink = junction(&mut builder, 60_011, 1, OUTWARD_REGION, 1);
        let outcome = junction(&mut builder, 60_012, 50, 0, 1);
        let anchor = junction(&mut builder, 60_013, 100, 0, 99);
        for target in [action, first_surface, second_surface, disconnected, outcome] {
            link(&mut builder, anchor, target, 0);
        }
        for source in [first_surface, second_surface] {
            link(&mut builder, source, outcome, 3);
        }
        link(&mut builder, motor, sink, 0);
        builder.set_outcome_source_for_output(motor, outcome);
        Self {
            harness: builder.build(),
            action,
            first_surface,
            second_surface,
            disconnected,
            motor,
        }
    }

    fn checkpoint(&self) -> FixtureCheckpoint {
        FixtureCheckpoint(self.harness.save().expect("fixture checkpoint saves"))
    }

    fn restore(protocol: Protocol, checkpoint: FixtureCheckpoint) -> Self {
        let template = Self::new(protocol);
        Self {
            harness: Harness::restore(checkpoint.0).expect("fixture checkpoint restores"),
            ..template
        }
    }

    fn action(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.action, tick, 60_000),
            input(self.motor, tick.saturating_add(2), 60_010),
        ])
    }

    fn deliver(&mut self, surface: JunctionId, physical: u64) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[input(surface, tick, physical)])
    }

    fn delayed_round(&mut self, surface: JunctionId, physical: u64) -> [Run; 2] {
        [self.action(), self.deliver(surface, physical)]
    }

    fn coactive_round(&mut self, surface_offset: i64) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.action, tick, 60_000),
            input(
                self.first_surface,
                tick.saturating_add(surface_offset),
                60_001,
            ),
            input(self.motor, tick.saturating_add(2), 60_010),
        ])
    }

    fn deliver_both(&mut self, reversed: bool) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let first = input(self.first_surface, tick, 60_001);
        let second = input(self.second_surface, tick, 60_002);
        let inputs = if reversed {
            [second, first]
        } else {
            [first, second]
        };
        self.harness.send(&inputs)
    }
}

fn counts(runs: &[Run]) -> (u64, u64) {
    runs.iter().fold((0, 0), |(closures, constructions), run| {
        (
            closures + run.work.causal_closure_observations,
            constructions + run.work.learner_constructions,
        )
    })
}

fn all_quiet(runs: &[Run]) -> bool {
    runs.iter().all(|run| run.naturally_quiescent)
}

fn same_run(left: &Run, right: &Run) -> bool {
    left.outputs == right.outputs
        && left.work == right.work
        && left.execution_cost == right.execution_cost
        && left.physical_trace == right.physical_trace
        && left.naturally_quiescent == right.naturally_quiescent
}

fn two_delayed(protocol: Protocol) -> (Fixture, Vec<Run>) {
    let mut fixture = Fixture::new(protocol);
    let surface = fixture.first_surface;
    let runs = [
        fixture.delayed_round(surface, 60_001),
        fixture.delayed_round(surface, 60_001),
    ]
    .into_iter()
    .flatten()
    .collect();
    (fixture, runs)
}

#[derive(Clone, Debug, Serialize)]
struct PartialEvidence {
    delayed_closures: u64,
    delayed_constructions: u64,
    coactive_closures: u64,
    coactive_constructions: u64,
    replay: bool,
    quiescent: bool,
    survived: bool,
}

fn partial_evidence(protocol: Protocol, temporal_only: bool) -> PartialEvidence {
    let (_, delayed) = two_delayed(protocol);
    let initial = Fixture::new(protocol).checkpoint();
    let mut coactive = Fixture::restore(protocol, initial.clone());
    let mut replay = Fixture::restore(protocol, initial);
    let coactive_runs = vec![coactive.coactive_round(0), coactive.coactive_round(0)];
    let replayed = vec![replay.coactive_round(0), replay.coactive_round(0)];
    let exact_replay = coactive_runs
        .iter()
        .zip(&replayed)
        .all(|(left, right)| same_run(left, right));
    let (delayed_closures, delayed_constructions) = counts(&delayed);
    let (coactive_closures, coactive_constructions) = counts(&coactive_runs);
    let survived = delayed_closures == 2
        && delayed_constructions == 1
        && coactive_closures == 0
        && coactive_constructions == 0
        && exact_replay
        && all_quiet(&delayed)
        && all_quiet(&coactive_runs);
    let expected_counterexample = if temporal_only {
        delayed_closures == 1 && coactive_closures == 1
    } else {
        delayed_closures == 1 && coactive_closures == 0
    };
    debug_assert!(survived || expected_counterexample);
    PartialEvidence {
        delayed_closures,
        delayed_constructions,
        coactive_closures,
        coactive_constructions,
        replay: exact_replay,
        quiescent: all_quiet(&delayed) && all_quiet(&coactive_runs),
        survived,
    }
}

#[derive(Clone, Debug, Serialize)]
struct CompositionEvidence {
    delayed_closures: u64,
    delayed_constructions: u64,
    coactive_closures: u64,
    equal_tick_closures: u64,
    disconnected_closures: u64,
    duplicate_closures: u64,
    duplicate_constructions: u64,
    stale_closures: u64,
    simultaneous_surface_closures: u64,
    simultaneous_surface_constructions: u64,
    simultaneous_eligible_origins: usize,
    simultaneous_consolidated_surfaces: usize,
    simultaneous_order_stable: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn composition_evidence() -> CompositionEvidence {
    let protocol = Protocol::RecursiveLearnerEligibleReturnClosure;
    let (delayed_fixture, delayed) = two_delayed(protocol);

    let mut coactive = Fixture::new(protocol);
    let coactive_runs = vec![coactive.coactive_round(0), coactive.coactive_round(0)];
    let mut equal = Fixture::new(protocol);
    let equal_runs = vec![equal.coactive_round(2), equal.coactive_round(2)];

    let mut disconnected = Fixture::new(protocol);
    let disconnected_target = disconnected.disconnected;
    let disconnected_runs = [
        disconnected.delayed_round(disconnected_target, 60_003),
        disconnected.delayed_round(disconnected_target, 60_003),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    let mut duplicate = Fixture::new(protocol);
    let mut duplicate_runs = vec![duplicate.action()];
    let tick = duplicate.harness.read().clock.tick.saturating_add(1);
    duplicate_runs.push(duplicate.harness.send(&[
        input(duplicate.first_surface, tick, 60_001),
        input(duplicate.first_surface, tick, 60_001),
    ]));

    let mut stale = Fixture::new(protocol);
    let mut stale_runs = vec![stale.action()];
    let tick = stale.harness.read().clock.tick.saturating_add(128);
    stale_runs.push(
        stale
            .harness
            .send(&[input(stale.disconnected, tick, 60_003)]),
    );
    stale_runs.push(stale.deliver(stale.first_surface, 60_001));

    let mut simultaneous = Fixture::new(protocol);
    let simultaneous_runs = vec![
        simultaneous.action(),
        simultaneous.deliver_both(false),
        simultaneous.action(),
        simultaneous.deliver_both(false),
    ];
    let mut simultaneous_reversed = Fixture::new(protocol);
    let simultaneous_reversed_runs = vec![
        simultaneous_reversed.action(),
        simultaneous_reversed.deliver_both(true),
        simultaneous_reversed.action(),
        simultaneous_reversed.deliver_both(true),
    ];

    let checkpoint = Fixture::new(protocol).checkpoint();
    let mut replay_left = Fixture::restore(protocol, checkpoint.clone());
    let mut replay_right = Fixture::restore(protocol, checkpoint);
    let left_surface = replay_left.first_surface;
    let right_surface = replay_right.first_surface;
    let left = [
        replay_left.delayed_round(left_surface, 60_001),
        replay_left.delayed_round(left_surface, 60_001),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    let right = [
        replay_right.delayed_round(right_surface, 60_001),
        replay_right.delayed_round(right_surface, 60_001),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    let exact_replay = left.iter().zip(&right).all(|(a, b)| same_run(a, b))
        && replay_left
            .harness
            .save()
            .expect("left saves")
            .canonical_bytes()
            .expect("left serializes")
            == replay_right
                .harness
                .save()
                .expect("right saves")
                .canonical_bytes()
                .expect("right serializes");

    let (delayed_closures, delayed_constructions) = counts(&delayed);
    let (coactive_closures, _) = counts(&coactive_runs);
    let (equal_tick_closures, _) = counts(&equal_runs);
    let (disconnected_closures, _) = counts(&disconnected_runs);
    let (duplicate_closures, duplicate_constructions) = counts(&duplicate_runs);
    let (stale_closures, _) = counts(&stale_runs);
    let (simultaneous_surface_closures, simultaneous_surface_constructions) =
        counts(&simultaneous_runs);
    let simultaneous_eligible_origins = simultaneous_runs
        .iter()
        .flat_map(|run| &run.physical_trace)
        .filter_map(|transition| match transition.event {
            PhysicalEvent::ClosureEligibilityEvaluated {
                origin_physical,
                eligible: true,
                ..
            } => Some(origin_physical),
            _ => None,
        })
        .collect::<BTreeSet<_>>()
        .len();
    let simultaneous_consolidated_surfaces = simultaneous_runs
        .iter()
        .flat_map(|run| &run.physical_trace)
        .filter_map(|transition| match transition.event {
            PhysicalEvent::ReversePathConsolidated { source, .. } => Some(source),
            _ => None,
        })
        .collect::<BTreeSet<_>>()
        .len();
    let simultaneous_surfaces = simultaneous_runs
        .iter()
        .flat_map(|run| &run.physical_trace)
        .filter_map(|transition| match transition.event {
            PhysicalEvent::ReversePathConsolidated { source, .. } => Some(source),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let reversed_surfaces = simultaneous_reversed_runs
        .iter()
        .flat_map(|run| &run.physical_trace)
        .filter_map(|transition| match transition.event {
            PhysicalEvent::ReversePathConsolidated { source, .. } => Some(source),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let simultaneous_order_stable = counts(&simultaneous_reversed_runs)
        == (
            simultaneous_surface_closures,
            simultaneous_surface_constructions,
        )
        && simultaneous_surfaces == reversed_surfaces;
    let naturally_quiescent = [
        delayed.as_slice(),
        coactive_runs.as_slice(),
        equal_runs.as_slice(),
        disconnected_runs.as_slice(),
        duplicate_runs.as_slice(),
        stale_runs.as_slice(),
        simultaneous_runs.as_slice(),
        simultaneous_reversed_runs.as_slice(),
        left.as_slice(),
        right.as_slice(),
    ]
    .into_iter()
    .all(all_quiet);
    let survived = delayed_closures == 2
        && delayed_constructions == 1
        && coactive_closures == 0
        && equal_tick_closures == 0
        && disconnected_closures == 0
        && duplicate_closures <= 1
        && duplicate_constructions == 0
        && stale_closures == 0
        && simultaneous_surface_closures == 2
        && simultaneous_surface_constructions == 1
        && simultaneous_eligible_origins == 1
        && simultaneous_consolidated_surfaces == 1
        && simultaneous_order_stable
        && exact_replay
        && naturally_quiescent;
    let _ = delayed_fixture;
    CompositionEvidence {
        delayed_closures,
        delayed_constructions,
        coactive_closures,
        equal_tick_closures,
        disconnected_closures,
        duplicate_closures,
        duplicate_constructions,
        stale_closures,
        simultaneous_surface_closures,
        simultaneous_surface_constructions,
        simultaneous_eligible_origins,
        simultaneous_consolidated_surfaces,
        simultaneous_order_stable,
        exact_replay,
        naturally_quiescent,
        survived,
    }
}

fn repeat_surface(
    fixture: &mut Fixture,
    surface: JunctionId,
    physical: u64,
    rounds: usize,
) -> Vec<Run> {
    (0..rounds)
        .flat_map(|_| fixture.delayed_round(surface, physical))
        .collect()
}

fn boundary_keys(fixture: &Fixture) -> BTreeSet<(JunctionId, JunctionId)> {
    fixture
        .harness
        .read()
        .learners
        .iter()
        .map(|learner| (learner.surface, learner.output))
        .collect()
}

fn max_depth(fixture: &Fixture) -> usize {
    let observation = fixture.harness.read();
    let parents = observation
        .learners
        .iter()
        .map(|learner| (learner.id, learner.parent))
        .collect::<BTreeMap<_, _>>();
    observation
        .learners
        .iter()
        .map(|learner| {
            let mut depth = 1;
            let mut parent = learner.parent;
            while let Some(id) = parent {
                depth += 1;
                parent = parents.get(&id).copied().flatten();
            }
            depth
        })
        .max()
        .unwrap_or(0)
}

#[derive(Clone, Debug, Serialize)]
struct BoundaryEvidence {
    gate_survived: bool,
    repeated_rounds: usize,
    repeated_learners: usize,
    repeated_keys: usize,
    repeated_max_depth: usize,
    two_surface_keys: usize,
    two_surface_contains_both: bool,
    reversed_keys: usize,
    order_independent: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn boundary_evidence(gate: &CompositionEvidence) -> BoundaryEvidence {
    const ROUNDS: usize = 8;
    let protocol = Protocol::RecursiveLearnerEligibleReturnClosure;
    let initial = Fixture::new(protocol).checkpoint();
    let mut repeated = Fixture::restore(protocol, initial.clone());
    let surface = repeated.first_surface;
    let repeated_runs = repeat_surface(&mut repeated, surface, 60_001, ROUNDS);
    let mut replay = Fixture::restore(protocol, initial);
    let surface = replay.first_surface;
    let replay_runs = repeat_surface(&mut replay, surface, 60_001, ROUNDS);
    let exact_replay = repeated_runs
        .iter()
        .zip(&replay_runs)
        .all(|(left, right)| same_run(left, right));

    let mut two = Fixture::new(protocol);
    let first = two.first_surface;
    let second = two.second_surface;
    let mut two_runs = repeat_surface(&mut two, first, 60_001, 2);
    two_runs.extend(repeat_surface(&mut two, second, 60_002, 2));
    let two_keys = boundary_keys(&two);
    let two_surface_contains_both = two_keys.iter().any(|(surface, _)| *surface == first)
        && two_keys.iter().any(|(surface, _)| *surface == second);

    let mut reversed = Fixture::new(protocol);
    let first = reversed.first_surface;
    let second = reversed.second_surface;
    let mut reversed_runs = repeat_surface(&mut reversed, second, 60_002, 2);
    reversed_runs.extend(repeat_surface(&mut reversed, first, 60_001, 2));
    let reversed_keys = boundary_keys(&reversed);
    let order_independent = two_keys == reversed_keys
        && two.harness.read().learners.len() == reversed.harness.read().learners.len();
    let naturally_quiescent = all_quiet(&repeated_runs)
        && all_quiet(&replay_runs)
        && all_quiet(&two_runs)
        && all_quiet(&reversed_runs);
    let repeated_keys = boundary_keys(&repeated).len();
    let repeated_learners = repeated.harness.read().learners.len();
    let repeated_max_depth = max_depth(&repeated);
    let survived = gate.survived
        && repeated_learners <= repeated_keys
        && repeated_max_depth <= 1
        && two_keys.len() == 2
        && two_surface_contains_both
        && order_independent
        && exact_replay
        && naturally_quiescent;
    BoundaryEvidence {
        gate_survived: gate.survived,
        repeated_rounds: ROUNDS,
        repeated_learners,
        repeated_keys,
        repeated_max_depth,
        two_surface_keys: two_keys.len(),
        two_surface_contains_both,
        reversed_keys: reversed_keys.len(),
        order_independent,
        exact_replay,
        naturally_quiescent,
        survived,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    match arm {
        Arm::InheritedSelectivityReference => {
            let digest = format!("{:x}", Sha256::digest(FROZEN_PARENT.as_bytes()));
            let survived = digest == EXPECTED_PARENT_SHA256
                && FROZEN_PARENT.contains("temporal-coactivity-selectivity")
                && FROZEN_PARENT.contains("boundary-novelty-selectivity");
            result(
                arm,
                if survived { "survived" } else { "inconclusive" },
                serde_json::json!({
                    "parent_sha256": digest,
                    "parent_intact": survived,
                    "inherited_first_failure": "causal-closure observation eligibility",
                    "dependent_failure": "physical boundary novelty",
                }),
                (!survived).then_some("the immutable parent convergence changed"),
                true,
                true,
            )
        }
        Arm::TemporalOnlyCounterexample => {
            let evidence = partial_evidence(Protocol::RecursiveLearnerConsequenceBornClosure, true);
            result(
                arm,
                if evidence.survived { "survived" } else { "falsified" },
                serde_json::to_value(&evidence).expect("partial evidence serializes"),
                (!evidence.survived).then_some(
                    "an unanswered sibling return crosses output cohorts, while a new ineligible return can suppress an older eligible return",
                ),
                evidence.replay,
                evidence.quiescent,
            )
        }
        Arm::TemporalCohortCounterexample => {
            let evidence =
                partial_evidence(Protocol::RecursiveLearnerConsequenceCohortClosure, false);
            result(
                arm,
                if evidence.survived { "survived" } else { "falsified" },
                serde_json::to_value(&evidence).expect("cohort evidence serializes"),
                (!evidence.survived).then_some(
                    "a newly opened ineligible return is processed before the valid older return and consumes the origin",
                ),
                evidence.replay,
                evidence.quiescent,
            )
        }
        Arm::EligibleReturnComposition => {
            let evidence = composition_evidence();
            result(
                arm,
                if evidence.survived { "survived" } else { "falsified" },
                serde_json::to_value(&evidence).expect("composition evidence serializes"),
                (!evidence.survived).then_some(
                    "the temporal, cohort, and eligible-first composition failed a positive or negative control",
                ),
                evidence.exact_replay,
                evidence.naturally_quiescent,
            )
        }
        Arm::BoundaryNoveltyAfterTemporalGate => {
            let gate = composition_evidence();
            let evidence = boundary_evidence(&gate);
            result(
                arm,
                if evidence.survived { "survived" } else { "falsified" },
                serde_json::to_value(&evidence).expect("boundary evidence serializes"),
                (!evidence.survived).then_some(
                    "after temporal selectivity passes, repeated ownership still creates depth or distinct sibling boundaries remain undiscoverable",
                ),
                evidence.exact_replay,
                evidence.naturally_quiescent,
            )
        }
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}

fn junction(
    builder: &mut HarnessBuilder,
    physical_id: u64,
    position: i32,
    region: i16,
    threshold: i32,
) -> JunctionId {
    builder.add_junction(Junction {
        physical_id,
        position,
        region,
        threshold,
        resistance: u32::MAX,
    })
}

fn link(builder: &mut HarnessBuilder, from: JunctionId, to: JunctionId, delay: i64) {
    builder.add_link(Link {
        from,
        to,
        delay,
        phase: 0,
        coupling: 1,
        resistance: u32::MAX,
        mode: TransmissionMode::Drive,
    });
}

fn input(target: JunctionId, tick: i64, origin_physical: u64) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical,
        target,
        impulse: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_mechanisms_preserve_their_exact_counterexamples() {
        let temporal = partial_evidence(Protocol::RecursiveLearnerConsequenceBornClosure, true);
        assert_eq!(
            (temporal.delayed_closures, temporal.coactive_closures),
            (1, 1)
        );
        assert!(!temporal.survived);
        let cohort = partial_evidence(Protocol::RecursiveLearnerConsequenceCohortClosure, false);
        assert_eq!((cohort.delayed_closures, cohort.coactive_closures), (1, 0));
        assert!(!cohort.survived);
    }

    #[test]
    fn complete_composition_passes_positive_negative_and_replay_controls() {
        let evidence = composition_evidence();
        assert!(evidence.survived, "{evidence:#?}");
    }

    #[test]
    fn boundary_probe_runs_only_after_the_temporal_gate_survives() {
        let gate = composition_evidence();
        assert!(gate.survived);
        let evidence = boundary_evidence(&gate);
        assert!(evidence.gate_survived);
        assert!(evidence.exact_replay);
        assert!(evidence.naturally_quiescent);
    }

    #[test]
    fn every_arm_is_total_and_quiescent() {
        for arm in Arm::ALL {
            let result = run(arm);
            assert_eq!(result.arm, arm.id());
            assert!(matches!(
                result.outcome,
                "survived" | "falsified" | "inconclusive"
            ));
            assert!(result.exact_replay);
            assert!(result.naturally_quiescent);
        }
    }
}
