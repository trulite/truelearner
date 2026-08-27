use serde::Serialize;
use std::collections::BTreeSet;
use std::str::FromStr;
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, Link, Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    CompleteCandidateReference,
    PhysicalContinuity,
    ReverseSurfaceExecution,
    FirstMotorOutput,
    OneJointControl,
    RepeatedAxisControl,
    DigitSeparation,
    BinocularDepth,
    VocalAuditoryControl,
    FullBodyComposition,
    DownwardAblations,
}

impl Arm {
    pub const ALL: [Self; 11] = [
        Self::CompleteCandidateReference,
        Self::PhysicalContinuity,
        Self::ReverseSurfaceExecution,
        Self::FirstMotorOutput,
        Self::OneJointControl,
        Self::RepeatedAxisControl,
        Self::DigitSeparation,
        Self::BinocularDepth,
        Self::VocalAuditoryControl,
        Self::FullBodyComposition,
        Self::DownwardAblations,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::CompleteCandidateReference => "complete-candidate-reference",
            Self::PhysicalContinuity => "physical-continuity",
            Self::ReverseSurfaceExecution => "reverse-surface-execution",
            Self::FirstMotorOutput => "first-motor-output",
            Self::OneJointControl => "one-joint-control",
            Self::RepeatedAxisControl => "repeated-axis-control",
            Self::DigitSeparation => "digit-separation",
            Self::BinocularDepth => "binocular-depth",
            Self::VocalAuditoryControl => "vocal-auditory-control",
            Self::FullBodyComposition => "full-body-composition",
            Self::DownwardAblations => "downward-ablations",
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

pub fn run(arm: Arm) -> ProbeResult {
    match arm {
        Arm::CompleteCandidateReference => complete_candidate_reference(),
        Arm::PhysicalContinuity => physical_continuity(),
        Arm::ReverseSurfaceExecution => reverse_surface_execution(),
        Arm::FirstMotorOutput => first_motor_output(),
        Arm::OneJointControl => after(arm, Arm::FirstMotorOutput, one_joint_control),
        Arm::RepeatedAxisControl => after(arm, Arm::OneJointControl, repeated_axis_control),
        Arm::DigitSeparation => after(arm, Arm::OneJointControl, digit_separation),
        Arm::BinocularDepth => after(arm, Arm::OneJointControl, binocular_depth),
        Arm::VocalAuditoryControl => after(arm, Arm::OneJointControl, vocal_auditory_control),
        Arm::FullBodyComposition => {
            let parents = [
                run(Arm::RepeatedAxisControl),
                run(Arm::DigitSeparation),
                run(Arm::BinocularDepth),
                run(Arm::VocalAuditoryControl),
            ];
            if parents.iter().all(|result| result.outcome == "survived") {
                full_body_composition()
            } else {
                inconclusive(
                    arm,
                    serde_json::json!({"parents": parents.iter().map(|result| (result.arm, result.outcome)).collect::<Vec<_>>() }),
                    "an upward body prerequisite failed",
                )
            }
        }
        Arm::DownwardAblations => {
            let joint = run(Arm::OneJointControl);
            if joint.outcome != "survived" {
                return inconclusive(
                    arm,
                    serde_json::json!({"one_joint": joint.outcome}),
                    "downward ablation is forbidden before full-body success",
                );
            }
            let full = run(Arm::FullBodyComposition);
            if full.outcome == "survived" {
                inconclusive(
                    arm,
                    serde_json::json!({"full_body": "survived"}),
                    "the full survivor must be frozen before removal implementations are admitted",
                )
            } else {
                inconclusive(
                    arm,
                    serde_json::json!({"full_body": full.outcome}),
                    "downward ablation is forbidden before full-body success",
                )
            }
        }
    }
}

fn after(arm: Arm, prerequisite: Arm, probe: fn() -> ProbeResult) -> ProbeResult {
    let parent = run(prerequisite);
    if parent.outcome == "survived" {
        probe()
    } else {
        inconclusive(
            arm,
            serde_json::json!({"prerequisite": prerequisite.id(), "outcome": parent.outcome}),
            "an upward prerequisite failed",
        )
    }
}

fn finish(
    arm: Arm,
    survived: bool,
    observations: serde_json::Value,
    falsifier: &str,
    replay: bool,
    quiet: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "sensorimotor-synthesis-ladder/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations,
        falsifier: (!survived).then(|| falsifier.to_string()),
        exact_replay: replay,
        naturally_quiescent: quiet,
    }
}

fn inconclusive(arm: Arm, observations: serde_json::Value, reason: &str) -> ProbeResult {
    ProbeResult {
        schema: "sensorimotor-synthesis-ladder/v1",
        arm: arm.id(),
        outcome: "inconclusive",
        observations,
        falsifier: Some(reason.to_string()),
        exact_replay: true,
        naturally_quiescent: true,
    }
}

fn complete_candidate_reference() -> ProbeResult {
    use sensorimotor_participation_continuity::Arm as Parent;
    let checks = Parent::ALL
        .into_iter()
        .map(sensorimotor_participation_continuity::run)
        .collect::<Vec<_>>();
    let expected = [
        "survived",
        "falsified",
        "survived",
        "falsified",
        "falsified",
        "survived",
        "inconclusive",
        "inconclusive",
        "inconclusive",
        "inconclusive",
    ];
    finish(
        Arm::CompleteCandidateReference,
        checks
            .iter()
            .zip(expected)
            .all(|(result, expected)| result.outcome == expected),
        serde_json::json!({"parent": checks.iter().map(|result| (result.arm.as_str(), result.outcome.as_str())).collect::<Vec<_>>() }),
        "a frozen parent classification changed",
        checks.iter().all(|result| result.exact_replay),
        checks.iter().all(|result| result.naturally_quiescent),
    )
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

fn input(target: JunctionId, tick: i64, origin: u64) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target,
        impulse: 1,
    }
}

fn same_run(left: &Run, right: &Run) -> bool {
    left.outputs == right.outputs
        && left.work == right.work
        && left.execution_cost == right.execution_cost
        && left.naturally_quiescent == right.naturally_quiescent
}

fn replay_send(harness: &mut Harness, inputs: &[Input]) -> (Run, bool) {
    let checkpoint = harness.save().expect("checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("checkpoint restores");
    let observed = harness.send(inputs);
    let replayed = replay.send(inputs);
    let exact = same_run(&observed, &replayed)
        && harness.save().unwrap().canonical_bytes().unwrap()
            == replay.save().unwrap().canonical_bytes().unwrap();
    (observed, exact)
}

fn structural_trial(dormant: usize) -> (u64, u64, bool) {
    let capacity = u32::try_from(dormant.saturating_mul(2).saturating_add(32)).unwrap();
    let mut builder = HarnessBuilder::with_capacity(capacity, capacity * 4, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorSynthesis);
    let source = junction(&mut builder, 100, 0, 0, 1);
    let motor = junction(&mut builder, 101, 1, 0, 2);
    let sink = junction(&mut builder, 102, 1, OUTWARD_REGION, 1);
    link(&mut builder, motor, sink, 0);
    for index in 0..dormant {
        let position = 100 + i32::try_from(index).unwrap();
        let output = junction(&mut builder, 1_000 + index as u64, position, 0, 2);
        let sink = junction(
            &mut builder,
            10_000 + index as u64,
            position,
            OUTWARD_REGION,
            1,
        );
        link(&mut builder, output, sink, 0);
    }
    let mut harness = builder.build();
    let (run, replay) = replay_send(&mut harness, &[input(source, 0, 100)]);
    (
        run.work.local_junction_proposals,
        run.execution_cost.local_structural_scans,
        replay && run.naturally_quiescent,
    )
}

fn physical_continuity() -> ProbeResult {
    let sizes = [4, 64, 1_024]
        .into_iter()
        .map(structural_trial)
        .collect::<Vec<_>>();
    let scans = sizes.iter().map(|trial| trial.1).collect::<BTreeSet<_>>();
    finish(
        Arm::PhysicalContinuity,
        sizes.iter().all(|trial| trial.0 == 2 && trial.2) && scans.len() == 1,
        serde_json::json!({"sizes": [4, 64, 1024], "trials": sizes}),
        "source-local structural work or exact replay changed with dormant topology",
        sizes.iter().all(|trial| trial.2),
        sizes.iter().all(|trial| trial.2),
    )
}

fn surface_world() -> (Harness, JunctionId, [JunctionId; 2], JunctionId, JunctionId) {
    let mut builder = HarnessBuilder::with_capacity(128, 512, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorSynthesis);
    let action = junction(&mut builder, 20_000, 0, 0, 1);
    let surfaces = [
        junction(&mut builder, 20_001, 0, 0, 1),
        junction(&mut builder, 20_002, 2, 0, 1),
    ];
    let unrelated = junction(&mut builder, 20_003, 20, 0, 1);
    let motor = junction(&mut builder, 20_010, 1, 0, 2);
    let sink = junction(&mut builder, 20_011, 1, OUTWARD_REGION, 1);
    let outcome = junction(&mut builder, 20_012, 50, 0, 1);
    let anchor = junction(&mut builder, 20_013, 100, 0, 99);
    for target in [action, surfaces[0], surfaces[1], unrelated, outcome] {
        link(&mut builder, anchor, target, 0);
    }
    for surface in surfaces.into_iter().chain([unrelated]) {
        link(&mut builder, surface, outcome, 3);
    }
    link(&mut builder, motor, sink, 0);
    builder.set_outcome_source_for_output(motor, outcome);
    (builder.build(), action, surfaces, unrelated, motor)
}

fn surface_order(reverse: bool) -> (bool, bool, bool, serde_json::Value) {
    let (mut harness, action, surfaces, unrelated, motor) = surface_world();
    let trained = harness.send(&[input(action, 0, 20_000), input(motor, 2, 20_010)]);
    let order = if reverse {
        [surfaces[1], surfaces[0]]
    } else {
        surfaces
    };
    let mut replay = true;
    for source in order {
        let tick = harness.read().clock.tick + 1;
        let physical = harness.read().junction(source).unwrap().physical_id;
        let (_, exact) = replay_send(&mut harness, &[input(source, tick, physical)]);
        replay &= exact;
    }
    let checkpoint = harness.save().unwrap();
    let recall = |source: JunctionId| {
        let mut trial = Harness::restore(checkpoint.clone()).unwrap();
        let tick = trial.read().clock.tick + 1;
        let physical = trial.read().junction(source).unwrap().physical_id;
        let run = trial.send(&[input(source, tick, physical)]);
        (
            run.outputs
                .iter()
                .any(|output| output.from_physical == 20_010),
            run.naturally_quiescent,
        )
    };
    let recalled = [recall(surfaces[0]), recall(surfaces[1])];
    let rejected = recall(unrelated);
    (
        recalled.iter().all(|trial| trial.0) && !rejected.0,
        replay,
        trained.naturally_quiescent && recalled.iter().all(|trial| trial.1) && rejected.1,
        serde_json::json!({"recalled": [recalled[0].0, recalled[1].0], "unrelated": rejected.0}),
    )
}

fn reverse_surface_execution() -> ProbeResult {
    let forward = surface_order(false);
    let reverse = surface_order(true);
    finish(
        Arm::ReverseSurfaceExecution,
        forward.0 && reverse.0,
        serde_json::json!({"forward": forward.3, "reverse": reverse.3}),
        "a useful surface did not execute the action or an unrelated surface did",
        forward.1 && reverse.1,
        forward.2 && reverse.2,
    )
}

fn stage_probe(
    arm: Arm,
    name: &str,
    axes: usize,
    surface: sensorimotor_emergence::CandidateSurface,
) -> ProbeResult {
    let stage = sensorimotor_emergence::run_candidate_control(name, axes, surface);
    finish(
        arm,
        stage.status == sensorimotor_emergence::StageStatus::Passed,
        serde_json::to_value(&stage).expect("stage serializes"),
        stage
            .falsifier
            .as_deref()
            .unwrap_or("the upward stage did not pass"),
        stage.observations["exact_replay"]
            .as_bool()
            .unwrap_or(false),
        stage.observations["naturally_quiescent"]
            .as_bool()
            .unwrap_or(false),
    )
}

fn first_motor_output() -> ProbeResult {
    let stage = sensorimotor_emergence::run_candidate_control_steps(
        "first_motor_output",
        1,
        sensorimotor_emergence::CandidateSurface::Proprioceptive,
        4,
    );
    let changed = stage.observations["changed_steps"].as_u64().unwrap_or(0);
    finish(
        Arm::FirstMotorOutput,
        changed > 0,
        serde_json::to_value(&stage).expect("stage serializes"),
        "the complete candidate emitted no motor output in four steps",
        stage.observations["exact_replay"]
            .as_bool()
            .unwrap_or(false),
        stage.observations["naturally_quiescent"]
            .as_bool()
            .unwrap_or(false),
    )
}

fn one_joint_control() -> ProbeResult {
    let stage = sensorimotor_emergence::run_candidate_control_steps(
        "one_joint_control",
        1,
        sensorimotor_emergence::CandidateSurface::Proprioceptive,
        16,
    );
    finish(
        Arm::OneJointControl,
        stage.status == sensorimotor_emergence::StageStatus::Passed,
        serde_json::to_value(&stage).expect("stage serializes"),
        stage
            .falsifier
            .as_deref()
            .unwrap_or("the one-joint preflight did not pass"),
        stage.observations["exact_replay"]
            .as_bool()
            .unwrap_or(false),
        stage.observations["naturally_quiescent"]
            .as_bool()
            .unwrap_or(false),
    )
}

fn repeated_axis_control() -> ProbeResult {
    stage_probe(
        Arm::RepeatedAxisControl,
        "repeated_axis_control",
        4,
        sensorimotor_emergence::CandidateSurface::Proprioceptive,
    )
}

fn digit_separation() -> ProbeResult {
    stage_probe(
        Arm::DigitSeparation,
        "digit_separation",
        10,
        sensorimotor_emergence::CandidateSurface::Proprioceptive,
    )
}

fn binocular_depth() -> ProbeResult {
    stage_probe(
        Arm::BinocularDepth,
        "binocular_depth",
        1,
        sensorimotor_emergence::CandidateSurface::Binocular,
    )
}

fn vocal_auditory_control() -> ProbeResult {
    stage_probe(
        Arm::VocalAuditoryControl,
        "vocal_auditory_control",
        4,
        sensorimotor_emergence::CandidateSurface::VocalAuditory,
    )
}

fn full_body_composition() -> ProbeResult {
    stage_probe(
        Arm::FullBodyComposition,
        "full_body_composition",
        7,
        sensorimotor_emergence::CandidateSurface::Composition,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_candidate_climbs_the_primitive_rungs() {
        for arm in [
            Arm::CompleteCandidateReference,
            Arm::PhysicalContinuity,
            Arm::ReverseSurfaceExecution,
            Arm::FirstMotorOutput,
        ] {
            let result = run(arm);
            assert_eq!(result.arm, arm.id());
            assert_eq!(result.outcome, "survived");
            assert!(result.exact_replay);
            assert!(result.naturally_quiescent);
        }
    }

    #[test]
    fn downward_ablation_is_conditional_on_full_success() {
        let joint = run(Arm::OneJointControl);
        let ablation = run(Arm::DownwardAblations);
        if joint.outcome != "survived" {
            assert_eq!(ablation.outcome, "inconclusive");
            assert!(ablation.falsifier.unwrap().contains("forbidden"));
        }
    }
}
