#![forbid(unsafe_code)]

use serde::Serialize;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{
    Checkpoint, Harness, HarnessBuilder, Input, Junction, JunctionId, Link, Output, PhysicalEvent,
    Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const LOWER: i16 = -4;
const UPPER: i16 = 4;
const PRIMARY_STEPS: usize = 16;
const JUNCTION_CAPACITY: u32 = 16_384;
const LINK_CAPACITY: u32 = 65_536;
const FROZEN_PARENT: &str = include_str!(
    "../../../campaigns/hand-same-lineage-closure-renewal-v1/artifacts/hand-same-lineage-closure-renewal.json"
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    InheritedNegativeReference,
    LineagePrimitiveControls,
    HandCausalLineagePreservation,
    FirstTransitionLocalization,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::InheritedNegativeReference,
        Self::LineagePrimitiveControls,
        Self::HandCausalLineagePreservation,
        Self::FirstTransitionLocalization,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::InheritedNegativeReference => "inherited-negative-reference",
            Self::LineagePrimitiveControls => "lineage-primitive-controls",
            Self::HandCausalLineagePreservation => "hand-causal-lineage-preservation",
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
        schema: "hand-causal-lineage-preservation/v1",
        arm: arm.id(),
        outcome,
        observations,
        falsifier,
        exact_replay,
        naturally_quiescent,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct FrozenReference {
    outcome: String,
    first_divergence: String,
    delivered_surface_origins: Vec<u64>,
    admitted_origins: Vec<u64>,
    exact_replay: bool,
    naturally_quiescent: bool,
    intact: bool,
}

fn frozen_reference() -> FrozenReference {
    let artifact: serde_json::Value =
        serde_json::from_str(FROZEN_PARENT).expect("frozen parent artifact parses");
    let outcome = artifact["outcome"].as_str().unwrap_or_default().to_string();
    let observations = &artifact["observations"];
    let first_divergence = observations["first_divergence"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let round = &observations["suffix"]["renewal_round"];
    let delivered_surface_origins = values_u64(&round["delivered_surface_origins"]);
    let admitted_origins = round["origin_admissions"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|admission| admission["admitted"].as_bool() == Some(true))
        .filter_map(|admission| admission["origin_physical"].as_u64())
        .collect::<Vec<_>>();
    let exact_replay = artifact["exact_replay"].as_bool().unwrap_or(false);
    let naturally_quiescent = artifact["naturally_quiescent"].as_bool().unwrap_or(false);
    let intact = outcome == "falsified"
        && first_divergence == "delivered-surface-origin-admission"
        && delivered_surface_origins == [10_002, 10_001]
        && admitted_origins == [40_001]
        && exact_replay
        && naturally_quiescent;
    FrozenReference {
        outcome,
        first_divergence,
        delivered_surface_origins,
        admitted_origins,
        exact_replay,
        naturally_quiescent,
        intact,
    }
}

fn values_u64(value: &serde_json::Value) -> Vec<u64> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(serde_json::Value::as_u64)
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct PrimitiveEvidence {
    scalar_outputs: Vec<Output>,
    lineage_outputs: Vec<Output>,
    forward_members: Vec<u64>,
    reversed_members: Vec<u64>,
    duplicate_members: Vec<u64>,
    one_impulse: bool,
    order_independent: bool,
    duplicate_free: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn primitive_run(origins: [u64; 2], protocol: Protocol) -> (Run, Vec<u64>) {
    let mut builder = HarnessBuilder::with_capacity(16, 16, OUTWARD_REGION);
    builder.set_protocol(protocol);
    builder.set_physical_tracing(true);
    let merge = junction(&mut builder, 81_000, 0, 0, 2);
    let sink = junction(&mut builder, 81_010, 0, OUTWARD_REGION, 1);
    link(&mut builder, merge, sink, 0);
    let run = builder
        .build()
        .send(&[input(merge, 0, origins[0]), input(merge, 0, origins[1])]);
    let members = run
        .physical_diagnostics()
        .filter_map(|transition| match transition.event {
            PhysicalEvent::CausalLineageMemberObserved {
                target,
                origin_physical,
                mode: TransmissionMode::Drive,
                ..
            } if target == sink => Some(origin_physical),
            _ => None,
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    (run, members)
}

fn primitive_evidence() -> PrimitiveEvidence {
    let (scalar, _) = primitive_run([81_001, 81_002], Protocol::RecursiveLearnerConstruction);
    let (forward, forward_members) =
        primitive_run([81_001, 81_002], Protocol::RecursiveLearnerCausalLineage);
    let (reversed, reversed_members) =
        primitive_run([81_002, 81_001], Protocol::RecursiveLearnerCausalLineage);
    let (duplicate, duplicate_members) =
        primitive_run([81_001, 81_001], Protocol::RecursiveLearnerCausalLineage);
    let one_impulse = forward.outputs == scalar.outputs
        && forward.outputs.len() == 1
        && forward.outputs[0].impulse == 1;
    let order_independent = forward.outputs == reversed.outputs
        && forward_members == reversed_members
        && forward_members == [81_001, 81_002];
    let duplicate_free = duplicate_members == [81_001];
    let scalar_outputs = scalar.outputs.clone();
    let lineage_outputs = forward.outputs.clone();
    let naturally_quiescent = [scalar, forward, reversed, duplicate]
        .iter()
        .all(|run| run.naturally_quiescent);
    PrimitiveEvidence {
        scalar_outputs,
        lineage_outputs,
        forward_members,
        reversed_members,
        duplicate_members,
        one_impulse,
        order_independent,
        duplicate_free,
        naturally_quiescent,
        survived: one_impulse && order_independent && duplicate_free && naturally_quiescent,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct ClosureKey {
    parent: Option<u64>,
    surface: u64,
    output: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ClosureObservation {
    run: usize,
    key: ClosureKey,
    evidence: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ConstructionObservation {
    run: usize,
    learner: u64,
    key: ClosureKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ReturnEvaluation {
    origin_physical: u64,
    decision: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct RunObservation {
    run: usize,
    kind: &'static str,
    step: usize,
    delivered_surface_origins: Vec<u64>,
    output_physical: Vec<u64>,
    lineage_members_at_return: Vec<u64>,
    return_evaluations: Vec<ReturnEvaluation>,
    admitted_origins: Vec<u64>,
    reverse_sources: Vec<u64>,
    closure_count: usize,
    construction_count: usize,
    local_return_updates: u64,
    naturally_quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Movement {
    step: usize,
    before: i16,
    after: i16,
    direction: i8,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct HandTrace {
    runs: Vec<RunObservation>,
    movements: Vec<Movement>,
    closures: Vec<ClosureObservation>,
    constructions: Vec<ConstructionObservation>,
    final_position: i16,
    reached_lower: bool,
    reached_upper: bool,
    naturally_quiescent: bool,
}

#[derive(Clone)]
struct HandCheckpoint {
    harness: Checkpoint,
    position: i16,
    pending: Vec<usize>,
    step: usize,
}

struct HandWorld {
    harness: Harness,
    sensors: Vec<JunctionId>,
    sensor_physical: Vec<u64>,
    motors: [JunctionId; 2],
    motor_physical: [u64; 2],
    position: i16,
    pending: Vec<usize>,
    step: usize,
}

impl HandWorld {
    fn new() -> Self {
        let mut builder =
            HarnessBuilder::with_capacity(JUNCTION_CAPACITY, LINK_CAPACITY, OUTWARD_REGION);
        builder.set_protocol(Protocol::RecursiveLearnerCausalLineage);
        builder.set_physical_tracing(true);
        let anchor = junction(&mut builder, 90_000, 10_000, 0, 99);
        let sensor_physical = (0..9)
            .map(|channel| 10_000 + channel as u64)
            .collect::<Vec<_>>();
        let sensors = sensor_physical
            .iter()
            .map(|physical| {
                let sensor = junction(&mut builder, *physical, 10, 0, 1);
                link(&mut builder, anchor, sensor, 0);
                sensor
            })
            .collect::<Vec<_>>();
        let motor_physical = [20_000, 20_001];
        let motors = [
            junction(&mut builder, motor_physical[0], 9, 0, 2),
            junction(&mut builder, motor_physical[1], 11, 0, 2),
        ];
        let sinks = [
            junction(&mut builder, 30_000, 9, OUTWARD_REGION, 1),
            junction(&mut builder, 30_001, 11, OUTWARD_REGION, 1),
        ];
        for index in 0..2 {
            link(&mut builder, motors[index], sinks[index], 0);
        }
        let outcomes = [
            junction(&mut builder, 40_000, 1_000, 0, 1),
            junction(&mut builder, 40_001, 1_001, 0, 1),
        ];
        for outcome in outcomes {
            link(&mut builder, anchor, outcome, 0);
        }
        for sensor in &sensors {
            for outcome in outcomes {
                link(&mut builder, *sensor, outcome, 3);
            }
        }
        for index in 0..2 {
            builder.set_outcome_source_for_output(motors[index], outcomes[index]);
        }
        Self {
            harness: builder.build(),
            sensors,
            sensor_physical,
            motors,
            motor_physical,
            position: 0,
            pending: Vec::new(),
            step: 0,
        }
    }

    fn checkpoint(&self) -> HandCheckpoint {
        HandCheckpoint {
            harness: self.harness.save().expect("hand checkpoint saves"),
            position: self.position,
            pending: self.pending.clone(),
            step: self.step,
        }
    }

    fn restore(checkpoint: HandCheckpoint) -> Self {
        let mut world = Self::new();
        world.harness = Harness::restore(checkpoint.harness).expect("hand checkpoint restores");
        world.position = checkpoint.position;
        world.pending = checkpoint.pending;
        world.step = checkpoint.step;
        world
    }

    fn deliver_pending(&mut self) -> Option<(Run, Vec<u64>)> {
        if self.pending.is_empty() {
            return None;
        }
        let channels = std::mem::take(&mut self.pending);
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let origins = channels
            .iter()
            .map(|channel| self.sensor_physical[*channel])
            .collect::<Vec<_>>();
        let inputs = channels
            .into_iter()
            .map(|channel| input(self.sensors[channel], tick, self.sensor_physical[channel]))
            .collect::<Vec<_>>();
        Some((self.harness.send(&inputs), origins))
    }

    fn act(&mut self, prior_outputs: &[Output]) -> (Run, Movement) {
        let before = self.position;
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let mut inputs = active_channels(self.position)
            .into_iter()
            .map(|channel| input(self.sensors[channel], tick, self.sensor_physical[channel]))
            .collect::<Vec<_>>();
        for index in 0..2 {
            inputs.push(input(
                self.motors[index],
                tick.saturating_add(2),
                40_000 + index as u64,
            ));
        }
        let run = self.harness.send(&inputs);
        let mut effort = [0_i32; 2];
        for output in prior_outputs.iter().chain(&run.outputs) {
            if output.from_physical == self.motor_physical[0] {
                effort[0] = effort[0].saturating_add(output.impulse.abs());
            } else if output.from_physical == self.motor_physical[1] {
                effort[1] = effort[1].saturating_add(output.impulse.abs());
            }
        }
        let direction = match effort[1].cmp(&effort[0]) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };
        self.position = self
            .position
            .saturating_add(i16::from(direction))
            .clamp(LOWER, UPPER);
        self.pending = if self.position == before {
            Vec::new()
        } else {
            active_channels(self.position)
        };
        let movement = Movement {
            step: self.step,
            before,
            after: self.position,
            direction,
        };
        self.step += 1;
        (run, movement)
    }
}

fn observe_run(
    run_index: usize,
    kind: &'static str,
    step: usize,
    delivered_surface_origins: Vec<u64>,
    run: &Run,
    closures: &mut Vec<ClosureObservation>,
    constructions: &mut Vec<ConstructionObservation>,
) -> RunObservation {
    let delivered = delivered_surface_origins
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let lineage_members_at_return = run
        .physical_diagnostics()
        .filter_map(|transition| match transition.event {
            PhysicalEvent::CausalLineageMemberObserved {
                origin_physical,
                mode: TransmissionMode::Modulatory,
                ..
            } if delivered.contains(&origin_physical) => Some(origin_physical),
            _ => None,
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let return_evaluations = run
        .physical_diagnostics()
        .filter_map(|transition| match transition.event {
            PhysicalEvent::ReturnOriginEvaluated {
                origin_physical,
                decision,
                ..
            } => Some(ReturnEvaluation {
                origin_physical,
                decision: format!("{decision:?}"),
            }),
            _ => None,
        })
        .collect::<Vec<_>>();
    let admitted_origins = return_evaluations
        .iter()
        .filter(|evaluation| {
            matches!(
                evaluation.decision.as_str(),
                "AdmittedDirect" | "AdmittedLocal"
            )
        })
        .map(|evaluation| evaluation.origin_physical)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let reverse_sources = run
        .physical_trace
        .iter()
        .filter_map(|transition| match transition.event {
            PhysicalEvent::ReversePathConsolidated { source, .. } => Some(source.0),
            _ => None,
        })
        .collect::<Vec<_>>();
    let closure_count_before = closures.len();
    let construction_count_before = constructions.len();
    for transition in &run.physical_trace {
        match transition.event {
            PhysicalEvent::CausalClosureObserved {
                parent,
                surface,
                output,
                evidence,
            } => closures.push(ClosureObservation {
                run: run_index,
                key: ClosureKey {
                    parent: parent.map(|parent| parent.0),
                    surface: surface.0,
                    output: output.0,
                },
                evidence,
            }),
            PhysicalEvent::LearnerConstructed {
                learner,
                parent,
                surface,
                output,
                ..
            } => constructions.push(ConstructionObservation {
                run: run_index,
                learner: learner.0,
                key: ClosureKey {
                    parent: parent.map(|parent| parent.0),
                    surface: surface.0,
                    output: output.0,
                },
            }),
            _ => {}
        }
    }
    RunObservation {
        run: run_index,
        kind,
        step,
        delivered_surface_origins,
        output_physical: run
            .outputs
            .iter()
            .map(|output| output.from_physical)
            .collect(),
        lineage_members_at_return,
        return_evaluations,
        admitted_origins,
        reverse_sources,
        closure_count: closures.len().saturating_sub(closure_count_before),
        construction_count: constructions
            .len()
            .saturating_sub(construction_count_before),
        local_return_updates: run.work.local_return_updates,
        naturally_quiescent: run.naturally_quiescent,
    }
}

fn run_hand(world: &mut HandWorld) -> HandTrace {
    let mut runs = Vec::new();
    let mut movements = Vec::new();
    let mut closures = Vec::new();
    let mut constructions = Vec::new();
    let mut run_index = 0;
    while world.step < PRIMARY_STEPS {
        let mut prior_outputs = Vec::new();
        if let Some((delivery, origins)) = world.deliver_pending() {
            prior_outputs = delivery.outputs.clone();
            runs.push(observe_run(
                run_index,
                "delivery",
                world.step,
                origins,
                &delivery,
                &mut closures,
                &mut constructions,
            ));
            run_index += 1;
        }
        let (action, movement) = world.act(&prior_outputs);
        runs.push(observe_run(
            run_index,
            "action",
            movement.step,
            Vec::new(),
            &action,
            &mut closures,
            &mut constructions,
        ));
        movements.push(movement);
        run_index += 1;
    }
    HandTrace {
        naturally_quiescent: runs.iter().all(|run| run.naturally_quiescent),
        final_position: world.position,
        reached_lower: movements.iter().any(|movement| movement.after == LOWER),
        reached_upper: movements.iter().any(|movement| movement.after == UPPER),
        runs,
        movements,
        closures,
        constructions,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum TransitionStage {
    FrozenNegativeReference,
    PrimitiveLineage,
    FirstClosure,
    DeliveredSurfaceLineageAtReturn,
    DeliveredSurfaceAdmission,
    ReverseConsolidation,
    SameClosureEvidenceTwo,
    Construction,
    Replay,
    Quiescence,
    Complete,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct StageEvidence {
    stage: TransitionStage,
    survived: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct HandEvidence {
    trace: HandTrace,
    stages: Vec<StageEvidence>,
    first_failure: TransitionStage,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn stage_evidence(
    reference: &FrozenReference,
    primitive: &PrimitiveEvidence,
    trace: &HandTrace,
    exact_replay: bool,
) -> Vec<StageEvidence> {
    let first = trace.closures.iter().find(|closure| closure.evidence == 1);
    let delivered_runs = trace
        .runs
        .iter()
        .filter(|run| !run.delivered_surface_origins.is_empty())
        .collect::<Vec<_>>();
    let surface_lineage = delivered_runs.iter().any(|run| {
        run.delivered_surface_origins
            .iter()
            .any(|origin| run.lineage_members_at_return.contains(origin))
    });
    let surface_admission = delivered_runs.iter().any(|run| {
        run.delivered_surface_origins
            .iter()
            .any(|origin| run.admitted_origins.contains(origin))
    });
    let surface_reverse = delivered_runs.iter().any(|run| {
        !run.reverse_sources.is_empty()
            && run
                .delivered_surface_origins
                .iter()
                .any(|origin| run.admitted_origins.contains(origin))
    });
    let evidence_two = first.is_some_and(|first| {
        trace.closures.iter().any(|closure| {
            closure.run > first.run && closure.key == first.key && closure.evidence >= 2
        })
    });
    let construction = first.is_some_and(|first| {
        trace
            .constructions
            .iter()
            .any(|construction| construction.run > first.run && construction.key == first.key)
    });
    [
        (TransitionStage::FrozenNegativeReference, reference.intact),
        (TransitionStage::PrimitiveLineage, primitive.survived),
        (TransitionStage::FirstClosure, first.is_some()),
        (
            TransitionStage::DeliveredSurfaceLineageAtReturn,
            surface_lineage,
        ),
        (
            TransitionStage::DeliveredSurfaceAdmission,
            surface_admission,
        ),
        (TransitionStage::ReverseConsolidation, surface_reverse),
        (TransitionStage::SameClosureEvidenceTwo, evidence_two),
        (TransitionStage::Construction, construction),
        (TransitionStage::Replay, exact_replay),
        (TransitionStage::Quiescence, trace.naturally_quiescent),
    ]
    .into_iter()
    .map(|(stage, survived)| StageEvidence { stage, survived })
    .collect()
}

fn hand_evidence(reference: &FrozenReference, primitive: &PrimitiveEvidence) -> HandEvidence {
    let initial = HandWorld::new().checkpoint();
    let mut world = HandWorld::restore(initial.clone());
    let mut replay = HandWorld::restore(initial);
    let trace = run_hand(&mut world);
    let replayed = run_hand(&mut replay);
    let exact_replay = trace == replayed
        && world.harness.save().unwrap().canonical_bytes().unwrap()
            == replay.harness.save().unwrap().canonical_bytes().unwrap();
    let stages = stage_evidence(reference, primitive, &trace, exact_replay);
    let first_failure = stages
        .iter()
        .find(|stage| !stage.survived)
        .map_or(TransitionStage::Complete, |stage| stage.stage);
    let naturally_quiescent = trace.naturally_quiescent;
    HandEvidence {
        survived: first_failure == TransitionStage::Complete,
        trace,
        stages,
        first_failure,
        exact_replay,
        naturally_quiescent,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Evidence {
    frozen: FrozenReference,
    primitive: PrimitiveEvidence,
    hand: HandEvidence,
}

fn measure() -> Evidence {
    let frozen = frozen_reference();
    let primitive = primitive_evidence();
    let hand = hand_evidence(&frozen, &primitive);
    Evidence {
        frozen,
        primitive,
        hand,
    }
}

static EVIDENCE: OnceLock<Evidence> = OnceLock::new();

fn evidence() -> &'static Evidence {
    EVIDENCE.get_or_init(measure)
}

fn failure_text(stage: TransitionStage) -> &'static str {
    match stage {
        TransitionStage::FrozenNegativeReference => "the frozen negative reference changed",
        TransitionStage::PrimitiveLineage => "lineage changed impulse, order, or duplication",
        TransitionStage::FirstClosure => "the candidate produced no first causal closure",
        TransitionStage::DeliveredSurfaceLineageAtReturn => {
            "an actually delivered surface disappeared before modulatory return"
        }
        TransitionStage::DeliveredSurfaceAdmission => {
            "no actually delivered surface origin was admitted"
        }
        TransitionStage::ReverseConsolidation => {
            "surface admission produced no reverse consolidation"
        }
        TransitionStage::SameClosureEvidenceTwo => {
            "reverse consolidation did not renew the first closure at evidence two"
        }
        TransitionStage::Construction => "evidence two did not construct the learner",
        TransitionStage::Replay => "candidate checkpoint replay diverged",
        TransitionStage::Quiescence => "candidate did not settle naturally",
        TransitionStage::Complete => "the complete candidate survived every declared stage",
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    match arm {
        Arm::InheritedNegativeReference => result(
            arm,
            if evidence.frozen.intact {
                "survived"
            } else {
                "falsified"
            },
            serde_json::to_value(&evidence.frozen).expect("reference serializes"),
            (!evidence.frozen.intact)
                .then(|| "the immutable parent counterexample did not match".to_string()),
            evidence.frozen.exact_replay,
            evidence.frozen.naturally_quiescent,
        ),
        Arm::LineagePrimitiveControls => result(
            arm,
            if evidence.primitive.survived {
                "survived"
            } else {
                "falsified"
            },
            serde_json::to_value(&evidence.primitive).expect("primitive serializes"),
            (!evidence.primitive.survived)
                .then(|| "lineage failed impulse, order, duplicate, or quiescence control".into()),
            true,
            evidence.primitive.naturally_quiescent,
        ),
        Arm::HandCausalLineagePreservation => result(
            arm,
            if evidence.hand.survived {
                "survived"
            } else {
                "falsified"
            },
            serde_json::to_value(&evidence.hand).expect("hand evidence serializes"),
            (!evidence.hand.survived)
                .then(|| failure_text(evidence.hand.first_failure).to_string()),
            evidence.hand.exact_replay,
            evidence.hand.naturally_quiescent,
        ),
        Arm::FirstTransitionLocalization => {
            let interpretable = evidence.frozen.intact
                && evidence.primitive.survived
                && evidence.hand.exact_replay
                && evidence.hand.naturally_quiescent;
            result(
                arm,
                if interpretable {
                    "survived"
                } else {
                    "inconclusive"
                },
                serde_json::json!({
                    "stages": evidence.hand.stages,
                    "first_failure": evidence.hand.first_failure,
                    "explanation": failure_text(evidence.hand.first_failure),
                    "hand_trace": evidence.hand.trace,
                }),
                (!interpretable).then(|| {
                    "reference, primitive, replay, or quiescence blocked interpretation".into()
                }),
                evidence.hand.exact_replay,
                evidence.hand.naturally_quiescent,
            )
        }
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}

fn active_channels(position: i16) -> Vec<usize> {
    let mut active = vec![2];
    if position < 0 {
        active.push(0);
    } else if position > 0 {
        active.push(1);
    }
    if position == LOWER {
        active.push(3);
    }
    if position == UPPER {
        active.push(4);
    }
    active
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
    fn lineage_primitive_survives_controls() {
        let evidence = evidence();
        assert!(evidence.primitive.survived, "{:#?}", evidence.primitive);
    }

    #[test]
    fn complete_hand_candidate_is_totally_classified() {
        let evidence = evidence();
        assert!(evidence.hand.survived || evidence.hand.first_failure != TransitionStage::Complete);
        assert_eq!(
            evidence.hand.survived,
            evidence.hand.stages.iter().all(|stage| stage.survived)
        );
    }

    #[test]
    fn inherited_negative_reference_is_unchanged() {
        let evidence = evidence();
        assert!(evidence.frozen.intact, "{:#?}", evidence.frozen);
    }

    #[test]
    fn hand_candidate_replays_and_settles() {
        let evidence = evidence();
        assert!(evidence.hand.exact_replay);
        assert!(evidence.hand.naturally_quiescent);
    }
}
