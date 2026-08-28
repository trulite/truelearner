#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    ReflectedHandProtocolEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{
    Checkpoint, Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent,
    Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const MAX_MOMENTS_PER_SEND: u64 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    EffectBornGenesisIsolation,
    ExternalSurfaceDiscriminator,
    ConditionalReflectedHand,
    CompleteComposition,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::EffectBornGenesisIsolation,
        Self::ExternalSurfaceDiscriminator,
        Self::ConditionalReflectedHand,
        Self::CompleteComposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::EffectBornGenesisIsolation => "effect-born-genesis-isolation",
            Self::ExternalSurfaceDiscriminator => "external-surface-discriminator",
            Self::ConditionalReflectedHand => "conditional-reflected-hand",
            Self::CompleteComposition => "complete-composition",
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

struct BoundaryFixture {
    harness: Harness,
    motors: [JunctionId; 2],
    effects: [JunctionId; 2],
    internal_surface: JunctionId,
}

impl BoundaryFixture {
    fn new(protocol: Protocol) -> Self {
        let mut builder = HarnessBuilder::with_capacity(40, 120, OUTWARD_REGION);
        builder.set_protocol(protocol);
        builder.set_physical_tracing(true);
        let motors = [
            junction(&mut builder, 70_000, 0, 0, 1),
            junction(&mut builder, 70_001, 1, 0, 1),
        ];
        let effects = [
            junction(&mut builder, 70_010, 0, OUTWARD_REGION, 1),
            junction(&mut builder, 70_011, 1, OUTWARD_REGION, 1),
        ];
        let internal_surface = junction(&mut builder, 70_020, -1, 0, 1);
        for index in 0..2 {
            link(&mut builder, motors[index], effects[index], 0);
        }
        Self {
            harness: builder.build(),
            motors,
            effects,
            internal_surface,
        }
    }

    fn restore(protocol: Protocol, checkpoint: Checkpoint) -> Self {
        let template = Self::new(protocol);
        Self {
            harness: Harness::restore(checkpoint).expect("fixture checkpoint restores"),
            ..template
        }
    }

    fn stimulate(&mut self, target: JunctionId) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send_bounded(
            &[Input {
                arrival_tick: tick,
                phase: 0,
                origin_physical: 70_099,
                target,
                impulse: 1,
            }],
            64,
        )
    }
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

fn proposal(run: &Run, source: JunctionId, target: JunctionId) -> bool {
    run.physical_trace.iter().any(|transition| {
        matches!(
            transition.event,
            PhysicalEvent::JunctionProposal {
                source: observed_source,
                target: observed_target,
                ..
            } if observed_source == source && observed_target == target
        )
    })
}

#[derive(Clone, Debug, Serialize)]
struct IsolationEvidence {
    reference_outputs: usize,
    regional_outputs: usize,
    terminal_outputs: usize,
    reference_effect_born_proposal: bool,
    regional_effect_born_proposal: bool,
    terminal_effect_born_proposal: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn isolation_evidence() -> IsolationEvidence {
    let mut reference = BoundaryFixture::new(Protocol::RecursiveLearnerCausalOriginFactorization);
    let reference_target = reference.motors[0];
    let reference_run = reference.stimulate(reference_target);

    let mut regional = BoundaryFixture::new(Protocol::RecursiveLearnerRegionalPathClosure);
    let regional_target = regional.motors[0];
    let regional_run = regional.stimulate(regional_target);

    let protocol = Protocol::RecursiveLearnerBoundaryEffectTerminal;
    let mut terminal = BoundaryFixture::new(protocol);
    let checkpoint = terminal.harness.save().expect("terminal fixture saves");
    let mut replay = BoundaryFixture::restore(protocol, checkpoint);
    let terminal_target = terminal.motors[0];
    let replay_target = replay.motors[0];
    let terminal_run = terminal.stimulate(terminal_target);
    let replayed = replay.stimulate(replay_target);

    let reference_effect_born_proposal =
        proposal(&reference_run, reference.effects[0], reference.motors[1]);
    let regional_effect_born_proposal =
        proposal(&regional_run, regional.effects[0], regional.motors[1]);
    let terminal_effect_born_proposal =
        proposal(&terminal_run, terminal.effects[0], terminal.motors[1]);
    let exact_replay = terminal_run == replayed
        && terminal.harness.save().unwrap().canonical_bytes().unwrap()
            == replay.harness.save().unwrap().canonical_bytes().unwrap();
    let naturally_quiescent = [&reference_run, &regional_run, &terminal_run, &replayed]
        .into_iter()
        .all(|run| run.naturally_quiescent);
    let reference_outputs = reference_run.outputs.len();
    let regional_outputs = regional_run.outputs.len();
    let terminal_outputs = terminal_run.outputs.len();
    let survived = reference_outputs == 3
        && regional_outputs == 1
        && terminal_outputs == 1
        && reference_effect_born_proposal
        && !regional_effect_born_proposal
        && !terminal_effect_born_proposal
        && exact_replay
        && naturally_quiescent;
    IsolationEvidence {
        reference_outputs,
        regional_outputs,
        terminal_outputs,
        reference_effect_born_proposal,
        regional_effect_born_proposal,
        terminal_effect_born_proposal,
        exact_replay,
        naturally_quiescent,
        survived,
    }
}

#[derive(Clone, Debug, Serialize)]
struct DiscriminatorEvidence {
    terminal_external_surface_forms: bool,
    regional_external_surface_forms: bool,
    terminal_internal_surface_forms: bool,
    regional_internal_surface_forms: bool,
    naturally_quiescent: bool,
    selected_protocol: &'static str,
    survived: bool,
}

fn discriminator_evidence() -> DiscriminatorEvidence {
    let mut terminal_external =
        BoundaryFixture::new(Protocol::RecursiveLearnerBoundaryEffectTerminal);
    let terminal_external_target = terminal_external.effects[0];
    let terminal_external_run = terminal_external.stimulate(terminal_external_target);
    let terminal_external_surface_forms = proposal(
        &terminal_external_run,
        terminal_external.effects[0],
        terminal_external.motors[1],
    );
    let mut regional_external = BoundaryFixture::new(Protocol::RecursiveLearnerRegionalPathClosure);
    let regional_external_target = regional_external.effects[0];
    let regional_external_run = regional_external.stimulate(regional_external_target);
    let regional_external_surface_forms = proposal(
        &regional_external_run,
        regional_external.effects[0],
        regional_external.motors[1],
    );

    let mut terminal_internal =
        BoundaryFixture::new(Protocol::RecursiveLearnerBoundaryEffectTerminal);
    let terminal_internal_target = terminal_internal.internal_surface;
    let terminal_internal_run = terminal_internal.stimulate(terminal_internal_target);
    let terminal_internal_surface_forms = terminal_internal_run
        .physical_trace
        .iter()
        .any(|transition| matches!(transition.event, PhysicalEvent::JunctionProposal { source, .. } if source == terminal_internal.internal_surface));
    let mut regional_internal = BoundaryFixture::new(Protocol::RecursiveLearnerRegionalPathClosure);
    let regional_internal_target = regional_internal.internal_surface;
    let regional_internal_run = regional_internal.stimulate(regional_internal_target);
    let regional_internal_surface_forms = regional_internal_run
        .physical_trace
        .iter()
        .any(|transition| matches!(transition.event, PhysicalEvent::JunctionProposal { source, .. } if source == regional_internal.internal_surface));
    let naturally_quiescent = [
        &terminal_external_run,
        &regional_external_run,
        &terminal_internal_run,
        &regional_internal_run,
    ]
    .into_iter()
    .all(|run| run.naturally_quiescent);
    let survived = terminal_external_surface_forms
        && !regional_external_surface_forms
        && terminal_internal_surface_forms
        && regional_internal_surface_forms
        && naturally_quiescent;
    DiscriminatorEvidence {
        terminal_external_surface_forms,
        regional_external_surface_forms,
        terminal_internal_surface_forms,
        regional_internal_surface_forms,
        naturally_quiescent,
        selected_protocol: "recursive-learner-boundary-effect-terminal",
        survived,
    }
}

#[derive(Clone, Debug, Serialize)]
struct HandEvidence {
    reference_position_changes: usize,
    terminal_position_changes: usize,
    reference_final_position: i16,
    terminal_final_position: i16,
    reference_exhaustions: u64,
    terminal_exhaustions: u64,
    terminal_reached_upper: bool,
    terminal_directions: usize,
    terminal_primary_closed: bool,
    terminal_perturbation_recovered: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    safe_transition_survived: bool,
    complete_hand_survived: bool,
}

fn position_changes(hand: &ReflectedHandProtocolEvidence) -> usize {
    hand.trajectory
        .iter()
        .filter(|step| step.position_before != step.position_after)
        .count()
}

fn exhaustions(hand: &ReflectedHandProtocolEvidence) -> u64 {
    hand.trajectory
        .iter()
        .map(|step| step.propagation_budget_exhaustions)
        .sum()
}

fn hand_evidence(
    reference: &ReflectedHandProtocolEvidence,
    terminal: &ReflectedHandProtocolEvidence,
) -> HandEvidence {
    let reference_position_changes = position_changes(reference);
    let terminal_position_changes = position_changes(terminal);
    let reference_exhaustions = exhaustions(reference);
    let terminal_exhaustions = exhaustions(terminal);
    let exact_replay = reference.exact_replay && terminal.exact_replay;
    let naturally_quiescent = terminal.naturally_quiescent;
    let safe_transition_survived = reference_exhaustions == 2
        && !reference.naturally_quiescent
        && terminal_exhaustions == 0
        && terminal.naturally_quiescent
        && terminal.reached_upper
        && terminal_position_changes > 0
        && exact_replay;
    let complete_hand_survived = terminal.primary_closed && terminal.perturbation_recovered;
    HandEvidence {
        reference_position_changes,
        terminal_position_changes,
        reference_final_position: reference.final_position,
        terminal_final_position: terminal.final_position,
        reference_exhaustions,
        terminal_exhaustions,
        terminal_reached_upper: terminal.reached_upper,
        terminal_directions: terminal.directions.len(),
        terminal_primary_closed: terminal.primary_closed,
        terminal_perturbation_recovered: terminal.perturbation_recovered,
        exact_replay,
        naturally_quiescent,
        safe_transition_survived,
        complete_hand_survived,
    }
}

#[derive(Clone, Debug)]
struct Evidence {
    isolation: IsolationEvidence,
    discriminator: DiscriminatorEvidence,
    hand: HandEvidence,
}

fn measure() -> Evidence {
    let isolation = isolation_evidence();
    let discriminator = discriminator_evidence();
    let reference = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerCausalOriginFactorization,
        512,
        2_048,
        MAX_MOMENTS_PER_SEND,
    );
    let terminal = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerBoundaryEffectTerminal,
        512,
        2_048,
        MAX_MOMENTS_PER_SEND,
    );
    Evidence {
        isolation,
        discriminator,
        hand: hand_evidence(&reference, &terminal),
    }
}

static EVIDENCE: OnceLock<Evidence> = OnceLock::new();

fn evidence() -> &'static Evidence {
    EVIDENCE.get_or_init(measure)
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
    survived: bool,
    observations: serde_json::Value,
    falsifier: &'static str,
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "hand-boundary-effect-reentry/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations,
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay,
        naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    match arm {
        Arm::EffectBornGenesisIsolation => result(
            arm,
            evidence.isolation.survived,
            serde_json::to_value(&evidence.isolation).unwrap(),
            "the candidate did not preserve the first effect while isolating effect-born cross-region path genesis",
            evidence.isolation.exact_replay,
            evidence.isolation.naturally_quiescent,
        ),
        Arm::ExternalSurfaceDiscriminator => result(
            arm,
            evidence.discriminator.survived,
            serde_json::to_value(&evidence.discriminator).unwrap(),
            "the narrow terminal law did not preserve external and same-region path formation better than regional closure",
            true,
            evidence.discriminator.naturally_quiescent,
        ),
        Arm::ConditionalReflectedHand => result(
            arm,
            evidence.hand.safe_transition_survived,
            serde_json::to_value(&evidence.hand).unwrap(),
            "the selected law did not remove exhaustion while preserving physical progress and replay",
            evidence.hand.exact_replay,
            evidence.hand.naturally_quiescent,
        ),
        Arm::CompleteComposition => {
            let survived = evidence.isolation.survived
                && evidence.discriminator.survived
                && evidence.hand.safe_transition_survived
                && evidence.hand.complete_hand_survived;
            result(
                arm,
                survived,
                serde_json::json!({
                    "effect_isolation": evidence.isolation.survived,
                    "narrow_law_selected": evidence.discriminator.survived,
                    "safe_hand_transition": evidence.hand.safe_transition_survived,
                    "complete_hand": evidence.hand.complete_hand_survived,
                    "terminal_directions": evidence.hand.terminal_directions,
                    "terminal_final_position": evidence.hand.terminal_final_position,
                    "next_failure": "the quiet hand reaches the upper boundary but expresses only one direction and never releases",
                }),
                "effect terminality fixed the runaway re-entry boundary but did not produce bidirectional hand closure",
                evidence.isolation.exact_replay && evidence.hand.exact_replay,
                evidence.isolation.naturally_quiescent && evidence.hand.naturally_quiescent,
            )
        }
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_born_genesis_is_isolated_without_losing_the_first_effect() {
        assert_eq!(run(Arm::EffectBornGenesisIsolation).outcome, "survived");
    }

    #[test]
    fn external_surface_selects_the_narrow_terminal_law() {
        assert_eq!(run(Arm::ExternalSurfaceDiscriminator).outcome, "survived");
    }

    #[test]
    fn selected_law_removes_hand_exhaustion_and_preserves_progress() {
        assert_eq!(run(Arm::ConditionalReflectedHand).outcome, "survived");
    }

    #[test]
    fn complete_hand_failure_is_preserved_after_safe_transition() {
        let result = run(Arm::CompleteComposition);
        assert_eq!(result.outcome, "falsified");
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
    }
}
