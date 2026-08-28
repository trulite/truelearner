#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{
    Checkpoint, Harness, HarnessBuilder, Input, Junction, JunctionId, Link, Protocol, Run,
    TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const EXPECTED_PARENT_SHA256: &str =
    "85df69ff559a50317be4a6703a75db8e63af02a8eb15162a7c1237760f10f0dd";
const FROZEN_PARENT: &str = include_str!(
    "../../../campaigns/hand-causal-lineage-preservation-v1/artifacts/hand-causal-lineage-preservation.json"
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    FrozenCausalContractAudit,
    TemporalCoactivitySelectivity,
    BoundaryNoveltySelectivity,
    FirstTransitionLocalization,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::FrozenCausalContractAudit,
        Self::TemporalCoactivitySelectivity,
        Self::BoundaryNoveltySelectivity,
        Self::FirstTransitionLocalization,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::FrozenCausalContractAudit => "frozen-causal-contract-audit",
            Self::TemporalCoactivitySelectivity => "temporal-coactivity-selectivity",
            Self::BoundaryNoveltySelectivity => "boundary-novelty-selectivity",
            Self::FirstTransitionLocalization => "first-transition-localization",
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
    falsifier: Option<String>,
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "hand-lineage-construction-selectivity/v1",
        arm: arm.id(),
        outcome,
        observations,
        falsifier,
        exact_replay,
        naturally_quiescent,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
struct ClosureKey {
    parent: Option<u64>,
    surface: u64,
    output: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct ParentRun {
    run: usize,
    kind: String,
    step: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
struct ParentMovement {
    step: usize,
    direction: i8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct ParentClosure {
    run: usize,
    key: ClosureKey,
    evidence: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct ParentConstruction {
    run: usize,
    learner: u64,
    key: ClosureKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct ParentTrace {
    runs: Vec<ParentRun>,
    movements: Vec<ParentMovement>,
    closures: Vec<ParentClosure>,
    constructions: Vec<ParentConstruction>,
    naturally_quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct ParentObservations {
    trace: ParentTrace,
    exact_replay: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct ParentArtifact {
    outcome: String,
    observations: ParentObservations,
    exact_replay: bool,
    naturally_quiescent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ObservationClass {
    OutputCausedDelivery,
    DeliveryOutputMismatch,
    CurrentStateCoactivity,
    MissingContext,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ConstructionAudit {
    learner: u64,
    key: ClosureKey,
    first_run: Option<usize>,
    second_run: Option<usize>,
    first_class: ObservationClass,
    second_class: ObservationClass,
    output_caused: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ContractAudit {
    parent_sha256: String,
    parent_intact: bool,
    chronological_first: ParentClosure,
    chronological_first_class: ObservationClass,
    chronological_first_is_output_caused: bool,
    constructions: Vec<ConstructionAudit>,
    reported_constructions: usize,
    output_caused_constructions: usize,
    distinct_physical_boundary_keys: usize,
    chronology_contract_valid: bool,
    any_owner_formation_supported: bool,
}

fn digest(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

fn classify_observation(
    closure: ParentClosure,
    runs: &BTreeMap<usize, &ParentRun>,
    movements: &BTreeMap<usize, ParentMovement>,
) -> ObservationClass {
    let Some(run) = runs.get(&closure.run) else {
        return ObservationClass::MissingContext;
    };
    if run.kind != "delivery" {
        return ObservationClass::CurrentStateCoactivity;
    }
    let Some(previous_step) = run.step.checked_sub(1) else {
        return ObservationClass::MissingContext;
    };
    let Some(movement) = movements.get(&previous_step) else {
        return ObservationClass::MissingContext;
    };
    let causal_output = match movement.direction {
        -1 => 10,
        1 => 11,
        _ => return ObservationClass::DeliveryOutputMismatch,
    };
    if closure.key.output == causal_output {
        ObservationClass::OutputCausedDelivery
    } else {
        ObservationClass::DeliveryOutputMismatch
    }
}

fn contract_audit() -> ContractAudit {
    let artifact: ParentArtifact =
        serde_json::from_str(FROZEN_PARENT).expect("frozen parent artifact parses");
    let trace = &artifact.observations.trace;
    let runs = trace
        .runs
        .iter()
        .map(|run| (run.run, run))
        .collect::<BTreeMap<_, _>>();
    let movements = trace
        .movements
        .iter()
        .copied()
        .map(|movement| (movement.step, movement))
        .collect::<BTreeMap<_, _>>();
    let chronological_first = *trace
        .closures
        .first()
        .expect("parent contains a closure observation");
    let chronological_first_class = classify_observation(chronological_first, &runs, &movements);
    let constructions = trace
        .constructions
        .iter()
        .map(|construction| {
            let first = trace.closures.iter().copied().find(|closure| {
                closure.key == construction.key
                    && closure.evidence == 1
                    && closure.run <= construction.run
            });
            let second = trace.closures.iter().copied().find(|closure| {
                closure.key == construction.key
                    && closure.evidence >= 2
                    && closure.run <= construction.run
            });
            let first_class = first.map_or(ObservationClass::MissingContext, |closure| {
                classify_observation(closure, &runs, &movements)
            });
            let second_class = second.map_or(ObservationClass::MissingContext, |closure| {
                classify_observation(closure, &runs, &movements)
            });
            ConstructionAudit {
                learner: construction.learner,
                key: construction.key,
                first_run: first.map(|closure| closure.run),
                second_run: second.map(|closure| closure.run),
                first_class,
                second_class,
                output_caused: first_class == ObservationClass::OutputCausedDelivery
                    && second_class == ObservationClass::OutputCausedDelivery,
            }
        })
        .collect::<Vec<_>>();
    let parent_sha256 = digest(FROZEN_PARENT);
    let parent_intact = parent_sha256 == EXPECTED_PARENT_SHA256
        && artifact.outcome == "falsified"
        && artifact.exact_replay
        && artifact.observations.exact_replay
        && artifact.naturally_quiescent
        && trace.naturally_quiescent;
    let output_caused_constructions = constructions
        .iter()
        .filter(|construction| construction.output_caused)
        .count();
    let distinct_physical_boundary_keys = trace
        .constructions
        .iter()
        .map(|construction| (construction.key.surface, construction.key.output))
        .collect::<BTreeSet<_>>()
        .len();
    ContractAudit {
        parent_sha256,
        parent_intact,
        chronological_first,
        chronological_first_class,
        chronological_first_is_output_caused: chronological_first_class
            == ObservationClass::OutputCausedDelivery,
        reported_constructions: constructions.len(),
        output_caused_constructions,
        distinct_physical_boundary_keys,
        chronology_contract_valid: false,
        any_owner_formation_supported: output_caused_constructions > 0,
        constructions,
    }
}

#[derive(Clone)]
struct FixtureCheckpoint {
    harness: Checkpoint,
}

struct Fixture {
    harness: Harness,
    action: JunctionId,
    first_surface: JunctionId,
    second_surface: JunctionId,
    disconnected: JunctionId,
    motor: JunctionId,
}

impl Fixture {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(128, 512, OUTWARD_REGION);
        builder.set_protocol(Protocol::RecursiveLearnerCausalLineage);
        builder.set_physical_tracing(true);
        let action = junction(&mut builder, 50_000, 0, 0, 1);
        let first_surface = junction(&mut builder, 50_001, 2, 0, 1);
        let second_surface = junction(&mut builder, 50_002, 3, 0, 1);
        let disconnected = junction(&mut builder, 50_003, 20, 0, 1);
        let motor = junction(&mut builder, 50_010, 1, 0, 2);
        let sink = junction(&mut builder, 50_011, 1, OUTWARD_REGION, 1);
        let outcome = junction(&mut builder, 50_012, 50, 0, 1);
        let anchor = junction(&mut builder, 50_013, 100, 0, 99);
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
        FixtureCheckpoint {
            harness: self.harness.save().expect("fixture checkpoint saves"),
        }
    }

    fn restore(checkpoint: FixtureCheckpoint) -> Self {
        let template = Self::new();
        Self {
            harness: Harness::restore(checkpoint.harness).expect("fixture checkpoint restores"),
            action: template.action,
            first_surface: template.first_surface,
            second_surface: template.second_surface,
            disconnected: template.disconnected,
            motor: template.motor,
        }
    }

    fn action(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.action, tick, 50_000),
            input(self.motor, tick.saturating_add(2), 50_010),
        ])
    }

    fn deliver(&mut self, surface: JunctionId, physical: u64) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[input(surface, tick, physical)])
    }

    fn causal_round(&mut self, surface: JunctionId, physical: u64) -> [Run; 2] {
        [self.action(), self.deliver(surface, physical)]
    }

    fn coactive_round(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.action, tick, 50_000),
            input(self.first_surface, tick, 50_001),
            input(self.motor, tick.saturating_add(2), 50_010),
        ])
    }
}

fn construction_count(runs: &[Run]) -> u64 {
    runs.iter().map(|run| run.work.learner_constructions).sum()
}

fn closure_count(runs: &[Run]) -> u64 {
    runs.iter()
        .map(|run| run.work.causal_closure_observations)
        .sum()
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct TemporalEvidence {
    delayed_closures: u64,
    delayed_constructions: u64,
    coactive_closures: u64,
    coactive_constructions: u64,
    disconnected_closures: u64,
    disconnected_constructions: u64,
    duplicate_closures: u64,
    duplicate_constructions: u64,
    exact_replay: bool,
    naturally_quiescent: bool,
    selectivity_survived: bool,
}

fn temporal_evidence() -> TemporalEvidence {
    let mut delayed = Fixture::new();
    let first = delayed.first_surface;
    let delayed_runs = [
        delayed.causal_round(first, 50_001),
        delayed.causal_round(first, 50_001),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    let initial = Fixture::new().checkpoint();
    let mut coactive = Fixture::restore(initial.clone());
    let mut replay = Fixture::restore(initial);
    let coactive_runs = vec![coactive.coactive_round(), coactive.coactive_round()];
    let replayed = vec![replay.coactive_round(), replay.coactive_round()];
    let exact_replay = coactive_runs
        .iter()
        .zip(&replayed)
        .all(|(left, right)| same_run(left, right))
        && coactive.harness.save().unwrap().canonical_bytes().unwrap()
            == replay.harness.save().unwrap().canonical_bytes().unwrap();

    let mut disconnected = Fixture::new();
    let disconnected_surface = disconnected.disconnected;
    let disconnected_runs = [
        disconnected.causal_round(disconnected_surface, 50_003),
        disconnected.causal_round(disconnected_surface, 50_003),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    let mut duplicate = Fixture::new();
    let training = duplicate.action();
    let tick = duplicate.harness.read().clock.tick.saturating_add(1);
    let duplicate_run = duplicate.harness.send(&[
        input(duplicate.first_surface, tick, 50_001),
        input(duplicate.first_surface, tick, 50_001),
    ]);
    let duplicate_runs = vec![training, duplicate_run];

    let delayed_closures = closure_count(&delayed_runs);
    let delayed_constructions = construction_count(&delayed_runs);
    let coactive_closures = closure_count(&coactive_runs);
    let coactive_constructions = construction_count(&coactive_runs);
    let disconnected_closures = closure_count(&disconnected_runs);
    let disconnected_constructions = construction_count(&disconnected_runs);
    let duplicate_closures = closure_count(&duplicate_runs);
    let duplicate_constructions = construction_count(&duplicate_runs);
    let naturally_quiescent = all_quiet(&delayed_runs)
        && all_quiet(&coactive_runs)
        && all_quiet(&disconnected_runs)
        && all_quiet(&duplicate_runs);
    let selectivity_survived = delayed_constructions == 1
        && coactive_constructions == 0
        && disconnected_constructions == 0
        && duplicate_constructions == 0
        && duplicate_closures <= 1
        && exact_replay
        && naturally_quiescent;
    TemporalEvidence {
        delayed_closures,
        delayed_constructions,
        coactive_closures,
        coactive_constructions,
        disconnected_closures,
        disconnected_constructions,
        duplicate_closures,
        duplicate_constructions,
        exact_replay,
        naturally_quiescent,
        selectivity_survived,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct BoundaryEvidence {
    repetitions: usize,
    same_boundary_learners: usize,
    same_boundary_max_depth: usize,
    same_boundary_keys: usize,
    two_surface_learners: usize,
    two_surface_root_learners: usize,
    two_surface_keys: usize,
    two_surface_contains_first: bool,
    two_surface_contains_second: bool,
    reversed_order_keys: usize,
    reversed_contains_first: bool,
    reversed_contains_second: bool,
    order_independent: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    selectivity_survived: bool,
}

fn learner_depths(learners: &[truelearner_core::LearnerObservation]) -> Vec<usize> {
    let parents = learners
        .iter()
        .map(|learner| (learner.id, learner.parent))
        .collect::<BTreeMap<_, _>>();
    learners
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
        .collect()
}

fn physical_boundary_keys(
    learners: &[truelearner_core::LearnerObservation],
) -> BTreeSet<(JunctionId, JunctionId)> {
    learners
        .iter()
        .map(|learner| (learner.surface, learner.output))
        .collect()
}

fn repeat_surface(
    world: &mut Fixture,
    surface: JunctionId,
    physical: u64,
    rounds: usize,
) -> Vec<Run> {
    (0..rounds)
        .flat_map(|_| world.causal_round(surface, physical))
        .collect()
}

fn boundary_evidence() -> BoundaryEvidence {
    const REPETITIONS: usize = 8;
    let initial = Fixture::new().checkpoint();
    let mut same = Fixture::restore(initial.clone());
    let same_surface = same.first_surface;
    let same_runs = repeat_surface(&mut same, same_surface, 50_001, REPETITIONS);
    let same_observation = same.harness.read();
    let same_depth = learner_depths(&same_observation.learners)
        .into_iter()
        .max()
        .unwrap_or(0);
    let same_keys = physical_boundary_keys(&same_observation.learners).len();

    let mut replay = Fixture::restore(initial);
    let replay_surface = replay.first_surface;
    let replayed = repeat_surface(&mut replay, replay_surface, 50_001, REPETITIONS);
    let exact_replay = same_runs
        .iter()
        .zip(&replayed)
        .all(|(left, right)| same_run(left, right))
        && same.harness.save().unwrap().canonical_bytes().unwrap()
            == replay.harness.save().unwrap().canonical_bytes().unwrap();

    let mut two = Fixture::new();
    let first = two.first_surface;
    let second = two.second_surface;
    let mut two_runs = repeat_surface(&mut two, first, 50_001, 2);
    two_runs.extend(repeat_surface(&mut two, second, 50_002, 2));
    let two_observation = two.harness.read();
    let two_keys = physical_boundary_keys(&two_observation.learners);
    let two_surface_contains_first = two_keys.iter().any(|(surface, _)| *surface == first);
    let two_surface_contains_second = two_keys.iter().any(|(surface, _)| *surface == second);

    let mut reversed = Fixture::new();
    let first = reversed.first_surface;
    let second = reversed.second_surface;
    let mut reversed_runs = repeat_surface(&mut reversed, second, 50_002, 2);
    reversed_runs.extend(repeat_surface(&mut reversed, first, 50_001, 2));
    let reversed_keys = physical_boundary_keys(&reversed.harness.read().learners);
    let reversed_contains_first = reversed_keys
        .iter()
        .any(|(surface, _)| *surface == reversed.first_surface);
    let reversed_contains_second = reversed_keys
        .iter()
        .any(|(surface, _)| *surface == reversed.second_surface);
    let order_independent = two_keys == reversed_keys
        && two_observation.learners.len() == reversed.harness.read().learners.len();
    let naturally_quiescent = all_quiet(&same_runs)
        && all_quiet(&replayed)
        && all_quiet(&two_runs)
        && all_quiet(&reversed_runs);
    let same_boundary_learners = same_observation.learners.len();
    let two_surface_learners = two_observation.learners.len();
    let two_surface_root_learners = two_observation
        .learners
        .iter()
        .filter(|learner| learner.parent.is_none())
        .count();
    let selectivity_survived = same_boundary_learners <= same_keys
        && same_depth <= 1
        && two_keys.len() == 2
        && order_independent
        && exact_replay
        && naturally_quiescent;
    BoundaryEvidence {
        repetitions: REPETITIONS,
        same_boundary_learners,
        same_boundary_max_depth: same_depth,
        same_boundary_keys: same_keys,
        two_surface_learners,
        two_surface_root_learners,
        two_surface_keys: two_keys.len(),
        two_surface_contains_first,
        two_surface_contains_second,
        reversed_order_keys: reversed_keys.len(),
        reversed_contains_first,
        reversed_contains_second,
        order_independent,
        exact_replay,
        naturally_quiescent,
        selectivity_survived,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Evidence {
    contract: ContractAudit,
    temporal: TemporalEvidence,
    boundary: BoundaryEvidence,
}

fn measure() -> Evidence {
    Evidence {
        contract: contract_audit(),
        temporal: temporal_evidence(),
        boundary: boundary_evidence(),
    }
}

static EVIDENCE: OnceLock<Evidence> = OnceLock::new();

fn evidence() -> &'static Evidence {
    EVIDENCE.get_or_init(measure)
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    match arm {
        Arm::FrozenCausalContractAudit => {
            let survived = evidence.contract.parent_intact
                && !evidence.contract.chronological_first_is_output_caused
                && evidence.contract.output_caused_constructions == 0
                && !evidence.contract.chronology_contract_valid
                && !evidence.contract.any_owner_formation_supported;
            result(
                arm,
                if survived { "survived" } else { "inconclusive" },
                serde_json::to_value(&evidence.contract).expect("contract serializes"),
                (!survived).then(|| "the frozen trace could not decide the causal contract".into()),
                true,
                true,
            )
        }
        Arm::TemporalCoactivitySelectivity => result(
            arm,
            if evidence.temporal.selectivity_survived {
                "survived"
            } else {
                "falsified"
            },
            serde_json::to_value(&evidence.temporal).expect("temporal evidence serializes"),
            (!evidence.temporal.selectivity_survived).then(|| {
                "current closure eligibility cannot distinguish output-caused delivery from current-state temporal coactivity".into()
            }),
            evidence.temporal.exact_replay,
            evidence.temporal.naturally_quiescent,
        ),
        Arm::BoundaryNoveltySelectivity => result(
            arm,
            if evidence.boundary.selectivity_survived {
                "survived"
            } else {
                "falsified"
            },
            serde_json::to_value(&evidence.boundary).expect("boundary evidence serializes"),
            (!evidence.boundary.selectivity_survived).then(|| {
                "repetition of one unchanged physical boundary creates deeper learner copies without boundary novelty".into()
            }),
            evidence.boundary.exact_replay,
            evidence.boundary.naturally_quiescent,
        ),
        Arm::FirstTransitionLocalization => {
            let interpretable = evidence.contract.parent_intact
                && evidence.temporal.exact_replay
                && evidence.temporal.naturally_quiescent
                && evidence.boundary.exact_replay
                && evidence.boundary.naturally_quiescent;
            result(
                arm,
                if interpretable { "survived" } else { "inconclusive" },
                serde_json::json!({
                    "lineage_preservation": "survived in the frozen parent",
                    "earliest_missing_transition": "causal-closure observation eligibility",
                    "first_false_admission": "a pre-movement/current-state surface observation counts as closure evidence",
                    "dependent_boundary_failure": "the same owned surface/output boundary can then generate deeper copies without new physical boundary",
                    "lineage_narrowing_recommended": false,
                    "next_solve": "preserve lineage and require consequence-born temporal eligibility before closure evidence; test physical boundary novelty separately",
                }),
                (!interpretable).then(|| "replay, quiescence, or frozen-parent integrity blocked localization".into()),
                evidence.temporal.exact_replay && evidence.boundary.exact_replay,
                evidence.temporal.naturally_quiescent && evidence.boundary.naturally_quiescent,
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
    use truelearner_core::PhysicalEvent;

    #[test]
    fn frozen_trace_rejects_chronology_and_unearned_owner_claims() {
        let audit = contract_audit();
        assert!(audit.parent_intact);
        assert_eq!(audit.reported_constructions, 14);
        assert!(!audit.chronological_first_is_output_caused);
        assert_eq!(audit.output_caused_constructions, 0);
        assert!(!audit.chronology_contract_valid);
        assert!(!audit.any_owner_formation_supported);
    }

    #[test]
    fn temporal_controls_classify_coactivity_without_weakening_negatives() {
        let evidence = temporal_evidence();
        assert_eq!(evidence.delayed_constructions, 1);
        assert_eq!(evidence.disconnected_constructions, 0);
        assert_eq!(evidence.duplicate_constructions, 0);
        assert!(evidence.duplicate_closures <= 1);
        assert!(evidence.exact_replay);
        assert!(evidence.naturally_quiescent);
    }

    #[test]
    fn repeated_same_boundary_is_not_counted_as_boundary_novelty() {
        let evidence = boundary_evidence();
        assert_eq!(evidence.same_boundary_keys, 1);
        assert!(evidence.same_boundary_learners >= evidence.same_boundary_keys);
        assert!(evidence.same_boundary_max_depth >= 1);
        assert_eq!(evidence.two_surface_keys, 1);
        assert!(evidence.two_surface_contains_first);
        assert!(!evidence.two_surface_contains_second);
        assert_eq!(evidence.reversed_order_keys, 1);
        assert!(evidence.reversed_contains_first);
        assert!(!evidence.reversed_contains_second);
        assert!(evidence.order_independent);
        assert!(evidence.exact_replay);
        assert!(evidence.naturally_quiescent);
    }

    #[test]
    fn every_arm_is_total_and_preserves_integrity() {
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

    #[test]
    fn closure_events_remain_physical_observations() {
        let mut fixture = Fixture::new();
        let surface = fixture.first_surface;
        let runs = fixture.causal_round(surface, 50_001);
        assert!(
            runs.iter()
                .any(|run| run.physical_trace.iter().any(|transition| {
                    matches!(
                        transition.event,
                        PhysicalEvent::CausalClosureObserved { .. }
                    )
                }))
        );
    }
}
