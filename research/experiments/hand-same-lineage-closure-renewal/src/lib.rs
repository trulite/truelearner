#![forbid(unsafe_code)]

use serde::Serialize;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{
    Checkpoint, Harness, HarnessBuilder, HarnessObservation, Input, Junction, JunctionId, Link,
    LinkId, Output, PhysicalEvent, Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const LOWER: i16 = -4;
const UPPER: i16 = 4;
const PRIMARY_STEPS: usize = 16;
const JUNCTION_CAPACITY: u32 = 16_384;
const LINK_CAPACITY: u32 = 65_536;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    InheritedIntegrityControl,
    ScriptedTwoCycleReference,
    HandFirstClosureReference,
    HandSameLineageClosureRenewal,
    FirstTransitionLocalization,
}

impl Arm {
    pub const ALL: [Self; 5] = [
        Self::InheritedIntegrityControl,
        Self::ScriptedTwoCycleReference,
        Self::HandFirstClosureReference,
        Self::HandSameLineageClosureRenewal,
        Self::FirstTransitionLocalization,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::InheritedIntegrityControl => "inherited-integrity-control",
            Self::ScriptedTwoCycleReference => "scripted-two-cycle-reference",
            Self::HandFirstClosureReference => "hand-first-closure-reference",
            Self::HandSameLineageClosureRenewal => "hand-same-lineage-closure-renewal",
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
    replay: bool,
    quiet: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "hand-same-lineage-closure-renewal/v1",
        arm: arm.id(),
        outcome,
        observations,
        falsifier,
        exact_replay: replay,
        naturally_quiescent: quiet,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct ClosureKey {
    parent: Option<u64>,
    surface: u64,
    output: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct ReturnToken {
    owner: Option<u64>,
    link: u64,
    generation: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ScheduledReturn {
    token: ReturnToken,
    admitted: bool,
    phase: SchedulePhase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum SchedulePhase {
    Action,
    Consequence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct OriginAdmission {
    token: ReturnToken,
    origin_physical: u64,
    admitted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct LinkState {
    link: u64,
    exists: bool,
    live: bool,
    modulatory: bool,
    from: Option<u64>,
    to: Option<u64>,
    life: Option<u64>,
    participation: Option<u64>,
    return_origins: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ReverseConsolidation {
    source: u64,
    output: u64,
    link: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ClosureObservation {
    key: ClosureKey,
    evidence: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ConstructionObservation {
    learner: u64,
    key: ClosureKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct RoundTrace {
    action_output_physical: Vec<u64>,
    participating_links: Vec<u64>,
    participating_links_at_consequence: Vec<LinkState>,
    scheduled_returns: Vec<ScheduledReturn>,
    scheduled_links_at_consequence: Vec<LinkState>,
    delivered_surface_origins: Vec<u64>,
    origin_admissions: Vec<OriginAdmission>,
    delivered_surface_admissions: Vec<OriginAdmission>,
    strengthened_links: Vec<u64>,
    reverse_consolidations: Vec<ReverseConsolidation>,
    closures: Vec<ClosureObservation>,
    constructions: Vec<ConstructionObservation>,
    naturally_quiescent: bool,
}

impl RoundTrace {
    fn capture(
        before_action: &HarnessObservation,
        action: &Run,
        before_consequence: &HarnessObservation,
        consequence: &Run,
        after_consequence: &HarnessObservation,
        delivered_surface_origins: Vec<u64>,
    ) -> Self {
        let action_output_physical = action
            .outputs
            .iter()
            .map(|output| output.from_physical)
            .collect::<Vec<_>>();
        let participating_links = before_consequence
            .links
            .iter()
            .filter_map(|after| {
                let before = before_action
                    .links
                    .iter()
                    .find(|before| before.id == after.id);
                (after.live
                    && after.participation > 0
                    && before.is_none_or(|before| before.participation != after.participation))
                .then_some(after.id.0)
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let schedules = |run: &Run, phase| {
            run.physical_trace
                .iter()
                .filter_map(move |transition| match transition.event {
                    PhysicalEvent::ReturnScheduling {
                        owner,
                        link,
                        generation,
                        admitted,
                    } => Some(ScheduledReturn {
                        token: ReturnToken {
                            owner: owner.map(|owner| owner.0),
                            link: link.0,
                            generation,
                        },
                        admitted,
                        phase,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        let scheduled_returns = schedules(action, SchedulePhase::Action)
            .into_iter()
            .chain(schedules(consequence, SchedulePhase::Consequence))
            .collect::<Vec<_>>();
        let participating_links_at_consequence = participating_links
            .iter()
            .map(|link| link_state(before_consequence, LinkId(*link)))
            .collect();
        let scheduled_links_at_consequence = scheduled_returns
            .iter()
            .map(|scheduled| {
                let observation = match scheduled.phase {
                    SchedulePhase::Action => before_consequence,
                    SchedulePhase::Consequence => after_consequence,
                };
                link_state(observation, LinkId(scheduled.token.link))
            })
            .collect();
        let origin_admissions = consequence
            .physical_trace
            .iter()
            .filter_map(|transition| match transition.event {
                PhysicalEvent::ReturnOriginAdmission {
                    owner,
                    link,
                    generation,
                    origin_physical,
                    admitted,
                } => Some(OriginAdmission {
                    token: ReturnToken {
                        owner: owner.map(|owner| owner.0),
                        link: link.0,
                        generation,
                    },
                    origin_physical,
                    admitted,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();
        let delivered_surface_admissions =
            surface_origin_admissions(&delivered_surface_origins, &origin_admissions);
        let strengthened_links = consequence
            .physical_trace
            .iter()
            .filter_map(|transition| match transition.event {
                PhysicalEvent::LinkStrengthened { link, .. } => Some(link.0),
                _ => None,
            })
            .collect();
        let reverse_consolidations = consequence
            .physical_trace
            .iter()
            .filter_map(|transition| match transition.event {
                PhysicalEvent::ReversePathConsolidated {
                    source,
                    output,
                    link,
                } => Some(ReverseConsolidation {
                    source: source.0,
                    output: output.0,
                    link: link.0,
                }),
                _ => None,
            })
            .collect();
        let closures = closure_observations(consequence);
        let constructions = consequence
            .physical_trace
            .iter()
            .filter_map(|transition| match transition.event {
                PhysicalEvent::LearnerConstructed {
                    learner,
                    parent,
                    surface,
                    output,
                    ..
                } => Some(ConstructionObservation {
                    learner: learner.0,
                    key: ClosureKey {
                        parent: parent.map(|parent| parent.0),
                        surface: surface.0,
                        output: output.0,
                    },
                }),
                _ => None,
            })
            .collect();
        Self {
            action_output_physical,
            participating_links,
            participating_links_at_consequence,
            scheduled_returns,
            scheduled_links_at_consequence,
            delivered_surface_origins,
            origin_admissions,
            delivered_surface_admissions,
            strengthened_links,
            reverse_consolidations,
            closures,
            constructions,
            naturally_quiescent: action.naturally_quiescent && consequence.naturally_quiescent,
        }
    }
}

fn surface_origin_admissions(
    delivered_surface_origins: &[u64],
    admissions: &[OriginAdmission],
) -> Vec<OriginAdmission> {
    let delivered = delivered_surface_origins
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    admissions
        .iter()
        .filter(|admission| admission.admitted && delivered.contains(&admission.origin_physical))
        .cloned()
        .collect()
}

fn link_state(observation: &HarnessObservation, id: LinkId) -> LinkState {
    observation.links.iter().find(|link| link.id == id).map_or(
        LinkState {
            link: id.0,
            exists: false,
            live: false,
            modulatory: false,
            from: None,
            to: None,
            life: None,
            participation: None,
            return_origins: Vec::new(),
        },
        |link| LinkState {
            link: id.0,
            exists: true,
            live: link.live,
            modulatory: link.mode == TransmissionMode::Modulatory,
            from: Some(link.from.0),
            to: Some(link.to.0),
            life: Some(link.life),
            participation: Some(link.participation),
            return_origins: link.return_origins.clone(),
        },
    )
}

fn closure_observations(run: &Run) -> Vec<ClosureObservation> {
    run.physical_trace
        .iter()
        .filter_map(|transition| match transition.event {
            PhysicalEvent::CausalClosureObserved {
                parent,
                surface,
                output,
                evidence,
            } => Some(ClosureObservation {
                key: ClosureKey {
                    parent: parent.map(|parent| parent.0),
                    surface: surface.0,
                    output: output.0,
                },
                evidence,
            }),
            _ => None,
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum TransitionStage {
    FirstClosure,
    FreshOutputParticipation,
    ParticipatingTraversal,
    FreshReturnScheduling,
    ReturnLiveAtConsequence,
    DeliveredSurfaceOriginAdmission,
    ReverseConsolidation,
    SameClosureIdentity,
    EvidenceTwo,
    Construction,
    Complete,
}

const ORDERED_STAGES: [TransitionStage; 10] = [
    TransitionStage::FirstClosure,
    TransitionStage::FreshOutputParticipation,
    TransitionStage::ParticipatingTraversal,
    TransitionStage::FreshReturnScheduling,
    TransitionStage::ReturnLiveAtConsequence,
    TransitionStage::DeliveredSurfaceOriginAdmission,
    TransitionStage::ReverseConsolidation,
    TransitionStage::SameClosureIdentity,
    TransitionStage::EvidenceTwo,
    TransitionStage::Construction,
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct StageEvidence {
    stage: TransitionStage,
    scripted: bool,
    hand: bool,
}

fn round_predicates(first: Option<ClosureKey>, round: Option<&RoundTrace>) -> [bool; 10] {
    let Some(first) = first else {
        return [false; 10];
    };
    let Some(round) = round else {
        return [
            true, false, false, false, false, false, false, false, false, false,
        ];
    };
    let same = round.closures.iter().any(|closure| closure.key == first);
    [
        true,
        !round.action_output_physical.is_empty(),
        !round.participating_links.is_empty(),
        !round.scheduled_returns.is_empty(),
        round
            .scheduled_links_at_consequence
            .iter()
            .any(|link| link.exists && link.live && link.modulatory),
        !round.delivered_surface_admissions.is_empty(),
        !round.reverse_consolidations.is_empty(),
        same,
        round
            .closures
            .iter()
            .any(|closure| closure.key == first && closure.evidence >= 2),
        round
            .constructions
            .iter()
            .any(|construction| construction.key == first),
    ]
}

fn compare_stages(
    scripted_first: Option<ClosureKey>,
    scripted: Option<&RoundTrace>,
    hand_first: Option<ClosureKey>,
    hand: Option<&RoundTrace>,
) -> Vec<StageEvidence> {
    let scripted = round_predicates(scripted_first, scripted);
    let hand = round_predicates(hand_first, hand);
    ORDERED_STAGES
        .into_iter()
        .enumerate()
        .map(|(index, stage)| StageEvidence {
            stage,
            scripted: scripted[index],
            hand: hand[index],
        })
        .collect()
}

fn first_divergence(stages: &[StageEvidence]) -> TransitionStage {
    stages
        .iter()
        .find(|stage| stage.scripted && !stage.hand)
        .map_or(TransitionStage::Complete, |stage| stage.stage)
}

struct ScriptedWorld {
    harness: Harness,
    action: JunctionId,
    surface: JunctionId,
    motor: JunctionId,
}

impl ScriptedWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_protocol(Protocol::RecursiveLearnerConstruction);
        builder.set_physical_tracing(true);
        let action = junction(&mut builder, 75_000, 0, 0, 1);
        let surface = junction(&mut builder, 75_001, 2, 0, 1);
        let motor = junction(&mut builder, 75_010, 1, 0, 2);
        let sink = junction(&mut builder, 75_011, 1, OUTWARD_REGION, 1);
        let outcome = junction(&mut builder, 75_012, 50, 0, 1);
        let anchor = junction(&mut builder, 75_013, 100, 0, 99);
        for target in [action, surface, outcome] {
            link(&mut builder, anchor, target, 0);
        }
        link(&mut builder, surface, outcome, 3);
        link(&mut builder, motor, sink, 0);
        builder.set_outcome_source_for_output(motor, outcome);
        Self {
            harness: builder.build(),
            action,
            surface,
            motor,
        }
    }

    fn restore(
        checkpoint: Checkpoint,
        action: JunctionId,
        surface: JunctionId,
        motor: JunctionId,
    ) -> Self {
        Self {
            harness: Harness::restore(checkpoint).expect("scripted checkpoint restores"),
            action,
            surface,
            motor,
        }
    }

    fn round(&mut self) -> (RoundTrace, Run) {
        let before_action = self.harness.read();
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let action = self.harness.send(&[
            input(self.action, tick, 75_000),
            input(self.motor, tick.saturating_add(2), 75_010),
        ]);
        let before = self.harness.read();
        let tick = before.clock.tick.saturating_add(1);
        let consequence = self.harness.send(&[input(self.surface, tick, 75_001)]);
        let after_consequence = self.harness.read();
        let trace = RoundTrace::capture(
            &before_action,
            &action,
            &before,
            &consequence,
            &after_consequence,
            vec![75_001],
        );
        (trace, consequence)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ScriptedEvidence {
    first_closure: Option<ClosureObservation>,
    second_round: RoundTrace,
    exact_replay: bool,
    naturally_quiescent: bool,
}

fn scripted_evidence() -> ScriptedEvidence {
    let mut world = ScriptedWorld::new();
    let (first_round, first_consequence) = world.round();
    let first_closure = first_round.closures.first().cloned();
    let checkpoint = world.harness.save().expect("scripted checkpoint saves");
    let mut replay = ScriptedWorld::restore(checkpoint, world.action, world.surface, world.motor);
    let (second_round, second_consequence) = world.round();
    let (replayed_round, replayed_consequence) = replay.round();
    let exact_replay = second_round == replayed_round
        && second_consequence == replayed_consequence
        && world.harness.save().unwrap().canonical_bytes().unwrap()
            == replay.harness.save().unwrap().canonical_bytes().unwrap();
    ScriptedEvidence {
        first_closure,
        naturally_quiescent: first_consequence.naturally_quiescent
            && second_round.naturally_quiescent,
        second_round,
        exact_replay,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct AttemptSummary {
    step: usize,
    position_before: i16,
    position_after: i16,
    direction: i8,
    output_physical: Vec<u64>,
    scheduled_returns: usize,
    naturally_quiescent: bool,
}

struct ActionAttempt {
    summary: AttemptSummary,
    before_action: HarnessObservation,
    run: Run,
}

impl HandWorld {
    fn new() -> Self {
        let mut builder =
            HarnessBuilder::with_capacity(JUNCTION_CAPACITY, LINK_CAPACITY, OUTWARD_REGION);
        builder.set_protocol(Protocol::RecursiveLearnerConstruction);
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

    fn act(&mut self, prior_outputs: &[Output]) -> ActionAttempt {
        let position_before = self.position;
        let before_action = self.harness.read();
        let tick = before_action.clock.tick.saturating_add(1);
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
        self.pending = if self.position == position_before {
            Vec::new()
        } else {
            active_channels(self.position)
        };
        let scheduled_returns = run
            .physical_trace
            .iter()
            .filter(|transition| {
                matches!(
                    transition.event,
                    PhysicalEvent::ReturnScheduling { admitted: true, .. }
                )
            })
            .count();
        let summary = AttemptSummary {
            step: self.step,
            position_before,
            position_after: self.position,
            direction,
            output_physical: prior_outputs
                .iter()
                .chain(&run.outputs)
                .map(|output| output.from_physical)
                .collect(),
            scheduled_returns,
            naturally_quiescent: run.naturally_quiescent,
        };
        self.step += 1;
        ActionAttempt {
            summary,
            before_action,
            run,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct InterveningDelivery {
    before_step: usize,
    surface_origins: Vec<u64>,
    admissions: usize,
    reverse_consolidations: usize,
    closures: Vec<ClosureObservation>,
    naturally_quiescent: bool,
}

fn summarize_delivery(before_step: usize, run: &Run, origins: Vec<u64>) -> InterveningDelivery {
    InterveningDelivery {
        before_step,
        surface_origins: origins,
        admissions: run
            .physical_trace
            .iter()
            .filter(|transition| {
                matches!(
                    transition.event,
                    PhysicalEvent::ReturnOriginAdmission { admitted: true, .. }
                )
            })
            .count(),
        reverse_consolidations: run
            .physical_trace
            .iter()
            .filter(|transition| {
                matches!(
                    transition.event,
                    PhysicalEvent::ReversePathConsolidated { .. }
                )
            })
            .count(),
        closures: closure_observations(run),
        naturally_quiescent: run.naturally_quiescent,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct HandSuffix {
    intervening_deliveries: Vec<InterveningDelivery>,
    skipped_attempts: Vec<AttemptSummary>,
    selected_attempt: Option<AttemptSummary>,
    renewal_round: Option<RoundTrace>,
    final_step: usize,
    naturally_quiescent: bool,
}

fn hand_suffix(world: &mut HandWorld) -> HandSuffix {
    let mut intervening_deliveries = Vec::new();
    let mut skipped_attempts = Vec::new();
    let mut quiet = true;
    while world.step < PRIMARY_STEPS {
        let mut prior_outputs = Vec::new();
        if let Some((delivery, origins)) = world.deliver_pending() {
            quiet &= delivery.naturally_quiescent;
            prior_outputs = delivery.outputs.clone();
            intervening_deliveries.push(summarize_delivery(world.step, &delivery, origins));
        }
        let attempt = world.act(&prior_outputs);
        quiet &= attempt.run.naturally_quiescent;
        if attempt.summary.direction == 0 {
            skipped_attempts.push(attempt.summary);
            continue;
        }
        let selected_attempt = attempt.summary;
        if world.step >= PRIMARY_STEPS || world.pending.is_empty() {
            return HandSuffix {
                intervening_deliveries,
                skipped_attempts,
                selected_attempt: Some(selected_attempt),
                renewal_round: None,
                final_step: world.step,
                naturally_quiescent: quiet,
            };
        }
        let before = world.harness.read();
        let (consequence, origins) = world
            .deliver_pending()
            .expect("selected movement leaves an actual pending surface");
        let after_consequence = world.harness.read();
        quiet &= consequence.naturally_quiescent;
        let renewal_round = RoundTrace::capture(
            &attempt.before_action,
            &attempt.run,
            &before,
            &consequence,
            &after_consequence,
            origins,
        );
        return HandSuffix {
            intervening_deliveries,
            skipped_attempts,
            selected_attempt: Some(selected_attempt),
            renewal_round: Some(renewal_round),
            final_step: world.step,
            naturally_quiescent: quiet,
        };
    }
    HandSuffix {
        intervening_deliveries,
        skipped_attempts,
        selected_attempt: None,
        renewal_round: None,
        final_step: world.step,
        naturally_quiescent: quiet,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct HandEvidence {
    first_attempt: AttemptSummary,
    first_closure: Option<ClosureObservation>,
    suffix: HandSuffix,
    exact_replay: bool,
    naturally_quiescent: bool,
}

fn hand_evidence() -> HandEvidence {
    let mut world = HandWorld::new();
    let first = world.act(&[]);
    let first_closure = closure_observations(&first.run).into_iter().next();
    let checkpoint = world.checkpoint();
    let mut replay = HandWorld::restore(checkpoint.clone());
    let suffix = hand_suffix(&mut world);
    let replayed = hand_suffix(&mut replay);
    let exact_replay = suffix == replayed
        && world.harness.save().unwrap().canonical_bytes().unwrap()
            == replay.harness.save().unwrap().canonical_bytes().unwrap();
    HandEvidence {
        first_attempt: first.summary,
        first_closure,
        naturally_quiescent: first.run.naturally_quiescent && suffix.naturally_quiescent,
        suffix,
        exact_replay,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct IntegrityEvidence {
    parent_hand_outcome: &'static str,
    parent_hand_closures: u64,
    parent_hand_constructions: u64,
    parent_reentry_outcome: &'static str,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn integrity_evidence() -> IntegrityEvidence {
    use developmental_hand_construction_admission::{Arm as HandArm, run as hand_run};
    use recursive_learner_fresh_memory::{Arm as MemoryArm, run as memory_run};
    let hand = hand_run(HandArm::TruthfulHandConstructionAdmission);
    let memory = memory_run(MemoryArm::ReturnReentryComposition);
    let events = &hand.observations["trial"]["aggregate"]["events"];
    let parent_hand_closures = events["closure_observations"].as_u64().unwrap_or(0);
    let parent_hand_constructions = events["constructions"].as_u64().unwrap_or(0);
    let exact_replay = hand.exact_replay && memory.exact_replay;
    let naturally_quiescent = hand.naturally_quiescent && memory.naturally_quiescent;
    let survived = hand.outcome == "falsified"
        && parent_hand_closures == 1
        && parent_hand_constructions == 0
        && memory.outcome == "survived"
        && exact_replay
        && naturally_quiescent;
    IntegrityEvidence {
        parent_hand_outcome: hand.outcome,
        parent_hand_closures,
        parent_hand_constructions,
        parent_reentry_outcome: memory.outcome,
        exact_replay,
        naturally_quiescent,
        survived,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Evidence {
    integrity: IntegrityEvidence,
    scripted: ScriptedEvidence,
    hand: HandEvidence,
    stages: Vec<StageEvidence>,
    first_divergence: TransitionStage,
}

fn measure() -> Evidence {
    let integrity = integrity_evidence();
    let scripted = scripted_evidence();
    let hand = hand_evidence();
    let stages = compare_stages(
        scripted.first_closure.as_ref().map(|closure| closure.key),
        Some(&scripted.second_round),
        hand.first_closure.as_ref().map(|closure| closure.key),
        hand.suffix.renewal_round.as_ref(),
    );
    let first_divergence = first_divergence(&stages);
    Evidence {
        integrity,
        scripted,
        hand,
        stages,
        first_divergence,
    }
}

static EVIDENCE: OnceLock<Evidence> = OnceLock::new();

fn evidence() -> &'static Evidence {
    EVIDENCE.get_or_init(measure)
}

fn scripted_survived(evidence: &Evidence) -> bool {
    round_predicates(
        evidence
            .scripted
            .first_closure
            .as_ref()
            .map(|closure| closure.key),
        Some(&evidence.scripted.second_round),
    )
    .into_iter()
    .all(|predicate| predicate)
        && evidence.scripted.exact_replay
        && evidence.scripted.naturally_quiescent
}

fn hand_first_survived(evidence: &Evidence) -> bool {
    evidence.integrity.survived
        && evidence.hand.first_attempt.step == 0
        && evidence.hand.first_attempt.position_before == 0
        && evidence.hand.first_attempt.position_after == 1
        && evidence
            .hand
            .first_closure
            .as_ref()
            .is_some_and(|closure| closure.evidence == 1)
        && evidence.hand.exact_replay
        && evidence.hand.naturally_quiescent
}

fn renewal_survived(evidence: &Evidence) -> bool {
    hand_first_survived(evidence)
        && round_predicates(
            evidence
                .hand
                .first_closure
                .as_ref()
                .map(|closure| closure.key),
            evidence.hand.suffix.renewal_round.as_ref(),
        )
        .into_iter()
        .all(|predicate| predicate)
}

fn divergence_explanation(stage: TransitionStage) -> &'static str {
    match stage {
        TransitionStage::FirstClosure => "the hand did not preserve the frozen first closure",
        TransitionStage::FreshOutputParticipation => {
            "no later actual movement round emitted a motor output"
        }
        TransitionStage::ParticipatingTraversal => {
            "the later motor output had no qualified participating traversal"
        }
        TransitionStage::FreshReturnScheduling => {
            "the later participating output scheduled no admitted fresh return"
        }
        TransitionStage::ReturnLiveAtConsequence => {
            "the fresh scheduled return was no longer a live modulatory link when its surface arrived"
        }
        TransitionStage::DeliveredSurfaceOriginAdmission => {
            "the live return admitted an outcome origin but not either actually delivered post-movement surface origin"
        }
        TransitionStage::ReverseConsolidation => {
            "an actually delivered surface origin was admitted but produced no reverse consolidation"
        }
        TransitionStage::SameClosureIdentity => {
            "reverse consolidation did not reproduce the first closure's parent, surface, and output identity"
        }
        TransitionStage::EvidenceTwo => {
            "the same closure identity did not advance from evidence one to evidence two"
        }
        TransitionStage::Construction => {
            "evidence two did not produce the threshold construction event"
        }
        TransitionStage::Complete => {
            "the hand matched every scripted transition through construction"
        }
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    let replay = evidence.integrity.exact_replay
        && evidence.scripted.exact_replay
        && evidence.hand.exact_replay;
    let quiet = evidence.integrity.naturally_quiescent
        && evidence.scripted.naturally_quiescent
        && evidence.hand.naturally_quiescent;
    match arm {
        Arm::InheritedIntegrityControl => result(
            arm,
            if evidence.integrity.survived {
                "survived"
            } else {
                "falsified"
            },
            serde_json::to_value(&evidence.integrity).expect("integrity serializes"),
            (!evidence.integrity.survived)
                .then(|| "a frozen parent classification changed".to_string()),
            evidence.integrity.exact_replay,
            evidence.integrity.naturally_quiescent,
        ),
        Arm::ScriptedTwoCycleReference => {
            let survived = evidence.integrity.survived && scripted_survived(evidence);
            result(
                arm,
                if survived { "survived" } else { "falsified" },
                serde_json::to_value(&evidence.scripted).expect("scripted evidence serializes"),
                (!survived).then(|| {
                    "the established scripted second round did not preserve every transition through construction"
                        .to_string()
                }),
                evidence.scripted.exact_replay,
                evidence.scripted.naturally_quiescent,
            )
        }
        Arm::HandFirstClosureReference => {
            let survived = hand_first_survived(evidence);
            result(
                arm,
                if survived { "survived" } else { "falsified" },
                serde_json::json!({
                    "first_attempt": evidence.hand.first_attempt,
                    "first_closure": evidence.hand.first_closure,
                    "parent": evidence.integrity,
                }),
                (!survived).then(|| "the hand's frozen evidence-one prefix changed".to_string()),
                evidence.hand.exact_replay,
                evidence.hand.naturally_quiescent,
            )
        }
        Arm::HandSameLineageClosureRenewal => {
            let survived = renewal_survived(evidence);
            result(
                arm,
                if survived { "survived" } else { "falsified" },
                serde_json::json!({
                    "first_closure": evidence.hand.first_closure,
                    "suffix": evidence.hand.suffix,
                    "stages": evidence.stages,
                    "first_divergence": evidence.first_divergence,
                    "explanation": divergence_explanation(evidence.first_divergence),
                }),
                (!survived).then(|| divergence_explanation(evidence.first_divergence).to_string()),
                evidence.hand.exact_replay,
                evidence.hand.naturally_quiescent,
            )
        }
        Arm::FirstTransitionLocalization => {
            let survived = evidence.integrity.survived
                && scripted_survived(evidence)
                && hand_first_survived(evidence)
                && replay
                && quiet;
            result(
                arm,
                if survived { "survived" } else { "inconclusive" },
                serde_json::json!({
                    "ordered_comparison": evidence.stages,
                    "first_divergence": evidence.first_divergence,
                    "explanation": divergence_explanation(evidence.first_divergence),
                    "scripted_round": evidence.scripted.second_round,
                    "hand_round": evidence.hand.suffix.renewal_round,
                    "intervening_deliveries": evidence.hand.suffix.intervening_deliveries,
                    "skipped_attempts": evidence.hand.suffix.skipped_attempts,
                }),
                (!survived).then(|| {
                    "a frozen reference, replay, or quiescence gate prevented interpretation"
                        .to_string()
                }),
                replay,
                quiet,
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
    fn transition_order_is_total() {
        assert_eq!(ORDERED_STAGES.len(), 10);
        let stages = ORDERED_STAGES
            .into_iter()
            .map(|stage| StageEvidence {
                stage,
                scripted: true,
                hand: stage != TransitionStage::DeliveredSurfaceOriginAdmission,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            first_divergence(&stages),
            TransitionStage::DeliveredSurfaceOriginAdmission
        );
    }

    #[test]
    fn scripted_second_round_reaches_evidence_two() {
        let evidence = evidence();
        assert!(scripted_survived(evidence), "{:#?}", evidence.scripted);
    }

    #[test]
    fn hand_first_closure_matches_frozen_reference() {
        let evidence = evidence();
        assert!(hand_first_survived(evidence), "{:#?}", evidence.hand);
    }

    #[test]
    fn hand_round_uses_only_actual_movement_and_surface() {
        let evidence = evidence();
        let selected = evidence
            .hand
            .suffix
            .selected_attempt
            .as_ref()
            .expect("a later movement is selected");
        let round = evidence
            .hand
            .suffix
            .renewal_round
            .as_ref()
            .expect("selected movement receives its actual surface");
        assert_ne!(selected.position_before, selected.position_after);
        assert!(!round.delivered_surface_origins.is_empty());
        assert!(
            round
                .delivered_surface_origins
                .iter()
                .all(|origin| (10_000..10_009).contains(origin))
        );
        assert!(
            evidence
                .hand
                .suffix
                .skipped_attempts
                .iter()
                .all(|attempt| attempt.direction == 0)
        );
    }

    #[test]
    fn admission_requires_an_actually_delivered_surface_origin() {
        let admissions = vec![
            OriginAdmission {
                token: ReturnToken {
                    owner: None,
                    link: 7,
                    generation: 3,
                },
                origin_physical: 40_001,
                admitted: true,
            },
            OriginAdmission {
                token: ReturnToken {
                    owner: None,
                    link: 8,
                    generation: 2,
                },
                origin_physical: 99_999,
                admitted: true,
            },
            OriginAdmission {
                token: ReturnToken {
                    owner: None,
                    link: 9,
                    generation: 1,
                },
                origin_physical: 10_002,
                admitted: true,
            },
        ];
        let matched = surface_origin_admissions(&[10_001, 10_002], &admissions);
        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0].origin_physical, 10_002);
    }

    #[test]
    fn renewal_and_localization_follow_frozen_predicates() {
        let renewal = run(Arm::HandSameLineageClosureRenewal);
        let localization = run(Arm::FirstTransitionLocalization);
        assert!(matches!(renewal.outcome, "survived" | "falsified"));
        assert_eq!(localization.outcome, "survived", "{localization:#?}");
        assert!(renewal.exact_replay);
        assert!(renewal.naturally_quiescent);
    }

    #[test]
    fn every_declared_arm_has_a_result() {
        for arm in Arm::ALL {
            let result = run(arm);
            assert_eq!(result.arm, arm.id());
            assert!(result.exact_replay, "{result:#?}");
            assert!(result.naturally_quiescent, "{result:#?}");
        }
    }
}
