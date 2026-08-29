use truelearner_core::{
    AttachmentSite, Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent,
    PhysicalIncidence, PhysicalInput, Protocol, Run, TransmissionMode,
};
use truelearner_embodiment::{
    calibrate, Availability, Driver, Incidence as SensorIncidence, Origin, PhysicalTraceComponent,
    PhysicalTraceSpec, PhysicalTraceSpecError, Residual, Signal,
};

const TRACE_LIFETIME: u32 = 8;

fn harness() -> Harness {
    let mut builder = HarnessBuilder::with_capacity(2_048, 8_192, 1);
    builder.set_physical_tracing(true);
    builder.set_protocol(Protocol::SensorimotorSynthesis);
    builder.build()
}

fn fixed_junction(
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

fn fixed_link(builder: &mut HarnessBuilder, from: JunctionId, to: JunctionId) {
    builder.add_link(Link {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: u32::MAX,
        mode: TransmissionMode::Drive,
    });
}

fn attach_trace(
    harness: &mut Harness,
    thresholds: Vec<i32>,
) -> (PhysicalTraceComponent, truelearner_core::PhysicalAttachment) {
    let trace = PhysicalTraceSpec::new(thresholds, TRACE_LIFETIME)
        .unwrap()
        .build();
    let attachment = harness
        .attach_physical(AttachmentSite::new(0, 0), trace.component())
        .unwrap();
    (trace, attachment)
}

fn junction_for_origin(harness: &Harness, origin: u64) -> JunctionId {
    harness
        .read()
        .junctions
        .iter()
        .find(|junction| junction.physical_id == origin)
        .unwrap()
        .id
}

fn fired(run: &Run, junction: JunctionId) -> bool {
    run.physical_trace
        .iter()
        .any(|transition| matches!(transition.event, PhysicalEvent::Fire { junction: fired } if fired == junction))
}

fn sample(
    harness: &mut Harness,
    trace: &PhysicalTraceComponent,
    attachment: &truelearner_core::PhysicalAttachment,
    tick: i64,
    value: i32,
) -> Run {
    let inputs = trace
        .sample_inputs(attachment, tick, value, PhysicalIncidence::Sample)
        .unwrap();
    harness.send_physical(&inputs)
}

#[test]
fn trace_spec_rejects_shapes_that_cannot_be_physical_memory() {
    assert_eq!(
        PhysicalTraceSpec::new(Vec::new(), TRACE_LIFETIME),
        Err(PhysicalTraceSpecError::NoThresholds)
    );
    assert_eq!(
        PhysicalTraceSpec::new(vec![1], 0),
        Err(PhysicalTraceSpecError::InvalidLifetime)
    );
    assert_eq!(
        PhysicalTraceSpec::new(vec![1], i32::MAX as u32),
        Err(PhysicalTraceSpecError::InvalidLifetime)
    );
    assert_eq!(
        PhysicalTraceSpec::new(vec![2, 2], TRACE_LIFETIME),
        Err(PhysicalTraceSpecError::ThresholdsNotIncreasing)
    );
}

#[test]
fn trace_initializes_then_reports_only_real_rise_and_fall() {
    let mut body = harness();
    let (trace, attachment) = attach_trace(&mut body, vec![5]);
    let rise = junction_for_origin(&body, trace.rise_origin(&attachment, 0).unwrap());
    let fall = junction_for_origin(&body, trace.fall_origin(&attachment, 0).unwrap());

    let initial = sample(&mut body, &trace, &attachment, 1, 2);
    assert!(initial.naturally_quiescent);
    assert!(!fired(&initial, rise));
    assert!(!fired(&initial, fall));

    let equal_low = sample(&mut body, &trace, &attachment, 2, 2);
    assert!(equal_low.naturally_quiescent);
    assert!(!fired(&equal_low, rise));
    assert!(!fired(&equal_low, fall));

    let rising = sample(&mut body, &trace, &attachment, 3, 7);
    assert!(rising.naturally_quiescent);
    assert!(fired(&rising, rise));
    assert!(!fired(&rising, fall));

    let equal_high = sample(&mut body, &trace, &attachment, 4, 7);
    assert!(equal_high.naturally_quiescent);
    assert!(!fired(&equal_high, rise));
    assert!(!fired(&equal_high, fall));

    let falling = sample(&mut body, &trace, &attachment, 5, 1);
    assert!(falling.naturally_quiescent);
    assert!(!fired(&falling, rise));
    assert!(fired(&falling, fall));
}

#[test]
fn trace_forgets_at_its_declared_physical_lifetime() {
    let mut remembered = harness();
    let (trace, attachment) = attach_trace(&mut remembered, vec![5]);
    let fall = junction_for_origin(&remembered, trace.fall_origin(&attachment, 0).unwrap());
    sample(&mut remembered, &trace, &attachment, 1, 9);
    let before_expiry = sample(
        &mut remembered,
        &trace,
        &attachment,
        i64::from(TRACE_LIFETIME),
        1,
    );
    assert!(fired(&before_expiry, fall));

    let mut forgotten = harness();
    let (trace, attachment) = attach_trace(&mut forgotten, vec![5]);
    let fall = junction_for_origin(&forgotten, trace.fall_origin(&attachment, 0).unwrap());
    sample(&mut forgotten, &trace, &attachment, 1, 9);
    let at_expiry = sample(
        &mut forgotten,
        &trace,
        &attachment,
        1_i64.saturating_add(i64::from(TRACE_LIFETIME)),
        1,
    );
    assert!(!fired(&at_expiry, fall));
}

#[test]
fn trace_checkpoint_replays_the_same_next_comparison() {
    let mut original = harness();
    let (trace, attachment) = attach_trace(&mut original, vec![5]);
    sample(&mut original, &trace, &attachment, 1, 2);
    let checkpoint = original.save().unwrap();
    let mut replay = Harness::restore(checkpoint).unwrap();

    let original_run = sample(&mut original, &trace, &attachment, 2, 8);
    let replay_run = sample(&mut replay, &trace, &attachment, 2, 8);

    assert_eq!(replay_run, original_run);
    assert_eq!(replay.read(), original.read());
}

#[test]
fn trace_transduction_preserves_the_real_physical_cause() {
    const ACTION_ORIGIN: u64 = 77_777;
    let mut body = harness();
    let (trace, attachment) = attach_trace(&mut body, vec![5]);
    sample(&mut body, &trace, &attachment, 1, 2);
    let inputs = trace
        .sample_inputs_from(
            &attachment,
            2,
            8,
            PhysicalIncidence::Transition,
            ACTION_ORIGIN,
        )
        .unwrap();
    let run = body.send_physical(&inputs);

    assert!(run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::PhysicalIncidenceObserved {
            origin_physical: ACTION_ORIGIN,
            incidence: PhysicalIncidence::Transition,
            ..
        }
    )));
}

#[test]
fn trace_factored_change_surface_preserves_rise_fall_and_quiet() {
    let mut body = harness();
    let trace = PhysicalTraceSpec::new(vec![5], TRACE_LIFETIME)
        .unwrap()
        .build_factored_change();
    let attachment = body
        .attach_physical(AttachmentSite::new(0, 0), trace.component())
        .unwrap();
    let rise = junction_for_origin(&body, trace.rise_origin(&attachment, 0).unwrap());
    let fall = junction_for_origin(&body, trace.fall_origin(&attachment, 0).unwrap());
    let change = junction_for_origin(&body, trace.change_origin(&attachment, 0).unwrap());

    let initial = sample(&mut body, &trace, &attachment, 1, 2);
    assert!(!fired(&initial, change));
    let rising = sample(&mut body, &trace, &attachment, 2, 8);
    assert!(fired(&rising, rise));
    assert!(fired(&rising, change));
    let equal = sample(&mut body, &trace, &attachment, 3, 8);
    assert!(!fired(&equal, change));
    let falling = sample(&mut body, &trace, &attachment, 4, 2);
    assert!(fired(&falling, fall));
    assert!(fired(&falling, change));
    assert!(falling.naturally_quiescent);
}

#[test]
fn trace_unified_change_surface_preserves_all_threshold_changes() {
    let mut body = harness();
    let trace = PhysicalTraceSpec::new(vec![-2, -1, 2, 3], TRACE_LIFETIME)
        .unwrap()
        .build_unified_change();
    let attachment = body
        .attach_physical(AttachmentSite::new(0, 0), trace.component())
        .unwrap();
    let origins = (0..4)
        .map(|index| trace.change_origin(&attachment, index).unwrap())
        .collect::<Vec<_>>();
    assert!(origins.windows(2).all(|pair| pair[0] == pair[1]));
    let change = junction_for_origin(&body, origins[0]);

    sample(&mut body, &trace, &attachment, 1, 0);
    assert!(fired(&sample(&mut body, &trace, &attachment, 2, 3), change));
    assert!(fired(&sample(&mut body, &trace, &attachment, 3, 2), change));
    assert!(fired(&sample(&mut body, &trace, &attachment, 4, 1), change));
    assert!(!fired(
        &sample(&mut body, &trace, &attachment, 5, 1),
        change
    ));
}

#[test]
fn calibration_trace_materializes_drive_fall_return_and_zero_identity() {
    let mut body = harness();
    let trace = PhysicalTraceSpec::new(vec![1, 2, 3], TRACE_LIFETIME)
        .unwrap()
        .build_calibration();
    let attachment = body
        .attach_physical(AttachmentSite::new(0, 0), trace.component())
        .unwrap();
    let drive = junction_for_origin(&body, trace.drive_origin(&attachment).unwrap());
    let high_rise = junction_for_origin(&body, trace.rise_origin(&attachment, 2).unwrap());
    let high_fall = junction_for_origin(&body, trace.fall_origin(&attachment, 2).unwrap());

    let quiet_inputs = trace
        .sample_inputs(&attachment, 1, 0, PhysicalIncidence::Sample)
        .unwrap();
    assert_eq!(quiet_inputs.len(), 1, "zero adds only the trace sample");
    let quiet = body.send_physical(&quiet_inputs);
    assert!(!fired(&quiet, drive));

    let active_inputs = trace
        .sample_inputs(&attachment, 2, 3, PhysicalIncidence::Transition)
        .unwrap();
    assert_eq!(
        active_inputs.len(),
        5,
        "three crossed thresholds, one trace sample, and one drive"
    );
    assert_eq!(
        active_inputs.last().unwrap().incidence,
        PhysicalIncidence::Sample,
        "persistent drive is not itself a returned physical change"
    );
    let active = body.send_physical(&active_inputs);
    assert!(fired(&active, high_rise));
    assert!(fired(&active, drive));

    let held = sample(&mut body, &trace, &attachment, 3, 3);
    assert!(!fired(&held, high_rise));
    assert!(!fired(&held, high_fall));
    assert!(fired(&held, drive), "unresolved residual remains a drive");

    let falling = body.send_physical(
        &trace
            .sample_inputs(&attachment, 4, 2, PhysicalIncidence::Transition)
            .unwrap(),
    );
    assert!(fired(&falling, high_fall));
    assert!(
        fired(&falling, drive),
        "a decrease returns to the drive surface"
    );

    let rising_again = body.send_physical(
        &trace
            .sample_inputs(&attachment, 5, 3, PhysicalIncidence::Transition)
            .unwrap(),
    );
    assert!(fired(&rising_again, high_rise));
    assert!(!fired(&rising_again, high_fall));
    assert!(fired(&rising_again, drive));

    let final_return_inputs = trace
        .sample_inputs(&attachment, 6, 0, PhysicalIncidence::Transition)
        .unwrap();
    assert_eq!(
        final_return_inputs.len(),
        1,
        "zero contributes no fresh drive; only physical falls may return"
    );
}

#[test]
fn trace_tracks_multiple_thresholds_without_collapsing_them() {
    let mut body = harness();
    let (trace, attachment) = attach_trace(&mut body, vec![3, 6, 9]);
    let rises = (0..3)
        .map(|index| junction_for_origin(&body, trace.rise_origin(&attachment, index).unwrap()))
        .collect::<Vec<_>>();
    let falls = (0..3)
        .map(|index| junction_for_origin(&body, trace.fall_origin(&attachment, index).unwrap()))
        .collect::<Vec<_>>();

    sample(&mut body, &trace, &attachment, 1, 2);
    let all_rise = sample(&mut body, &trace, &attachment, 2, 10);
    assert!(rises.iter().all(|junction| fired(&all_rise, *junction)));
    assert!(falls.iter().all(|junction| !fired(&all_rise, *junction)));

    let partial_fall = sample(&mut body, &trace, &attachment, 3, 5);
    assert!(!fired(&partial_fall, falls[0]));
    assert!(fired(&partial_fall, falls[1]));
    assert!(fired(&partial_fall, falls[2]));
}

#[test]
fn modality_labels_do_not_change_the_physical_construction() {
    let modalities = [
        "luminance",
        "contrast",
        "pressure",
        "slip",
        "depth",
        "sound",
        "position",
        "velocity",
        "effort",
        "availability",
        "held-out-field",
    ];

    for modality in modalities {
        let mut body = harness();
        let (trace, attachment) = attach_trace(&mut body, vec![5]);
        let rise = junction_for_origin(&body, trace.rise_origin(&attachment, 0).unwrap());
        sample(&mut body, &trace, &attachment, 1, 2);
        let run = sample(&mut body, &trace, &attachment, 2, 8);
        assert!(fired(&run, rise), "{modality} did not transfer");
    }
}

#[test]
fn composition_keeps_attached_sensor_memories_independent() {
    let mut body = harness();
    let (left, left_attachment) = attach_trace(&mut body, vec![5]);
    let (right, right_attachment) = attach_trace(&mut body, vec![5]);
    let left_rise = junction_for_origin(&body, left.rise_origin(&left_attachment, 0).unwrap());
    let right_rise = junction_for_origin(&body, right.rise_origin(&right_attachment, 0).unwrap());

    let mut initial = left
        .sample_inputs(&left_attachment, 1, 2, PhysicalIncidence::Sample)
        .unwrap();
    initial.extend(
        right
            .sample_inputs(&right_attachment, 1, 2, PhysicalIncidence::Sample)
            .unwrap(),
    );
    body.send_physical(&initial);

    let run = sample(&mut body, &left, &left_attachment, 2, 8);
    assert!(fired(&run, left_rise));
    assert!(!fired(&run, right_rise));
}

#[test]
fn attachment_order_does_not_change_each_sensor_truth_table() {
    fn run(reverse: bool) -> [(bool, bool); 2] {
        let mut body = harness();
        let left = PhysicalTraceSpec::new(vec![5], TRACE_LIFETIME)
            .unwrap()
            .build();
        let right = left.clone();
        let (left_attachment, right_attachment) = if reverse {
            let right_attachment = body
                .attach_physical(AttachmentSite::new(10, 0), right.component())
                .unwrap();
            let left_attachment = body
                .attach_physical(AttachmentSite::new(-10, 0), left.component())
                .unwrap();
            (left_attachment, right_attachment)
        } else {
            let left_attachment = body
                .attach_physical(AttachmentSite::new(-10, 0), left.component())
                .unwrap();
            let right_attachment = body
                .attach_physical(AttachmentSite::new(10, 0), right.component())
                .unwrap();
            (left_attachment, right_attachment)
        };
        let left_rise = junction_for_origin(&body, left.rise_origin(&left_attachment, 0).unwrap());
        let left_fall = junction_for_origin(&body, left.fall_origin(&left_attachment, 0).unwrap());
        let right_rise =
            junction_for_origin(&body, right.rise_origin(&right_attachment, 0).unwrap());
        let right_fall =
            junction_for_origin(&body, right.fall_origin(&right_attachment, 0).unwrap());

        let mut initial = left
            .sample_inputs(&left_attachment, 1, 2, PhysicalIncidence::Sample)
            .unwrap();
        initial.extend(
            right
                .sample_inputs(&right_attachment, 1, 8, PhysicalIncidence::Sample)
                .unwrap(),
        );
        body.send_physical(&initial);

        let mut changed = left
            .sample_inputs(&left_attachment, 2, 8, PhysicalIncidence::Sample)
            .unwrap();
        changed.extend(
            right
                .sample_inputs(&right_attachment, 2, 2, PhysicalIncidence::Sample)
                .unwrap(),
        );
        let run = body.send_physical(&changed);
        [
            (fired(&run, left_rise), fired(&run, left_fall)),
            (fired(&run, right_rise), fired(&run, right_fall)),
        ]
    }

    assert_eq!(run(false), [(true, false), (false, true)]);
    assert_eq!(run(true), run(false));
}

#[test]
fn reconnect_and_replacement_do_not_invent_shared_history() {
    let mut body = harness();
    let (old, old_attachment) = attach_trace(&mut body, vec![5]);
    let old_rise = junction_for_origin(&body, old.rise_origin(&old_attachment, 0).unwrap());
    sample(&mut body, &old, &old_attachment, 1, 2);

    let reconnected = sample(&mut body, &old, &old_attachment, 3, 8);
    assert!(fired(&reconnected, old_rise));

    let (replacement, replacement_attachment) = attach_trace(&mut body, vec![5]);
    let replacement_rise = junction_for_origin(
        &body,
        replacement.rise_origin(&replacement_attachment, 0).unwrap(),
    );
    let first_replacement = sample(&mut body, &replacement, &replacement_attachment, 4, 8);
    assert!(!fired(&first_replacement, replacement_rise));
    assert!(!fired(&first_replacement, old_rise));
}

fn one_active_trace_with_dormant_replacements(dormant: usize) -> Run {
    let mut body = harness();
    let (active, active_attachment) = attach_trace(&mut body, vec![5]);
    for _ in 0..dormant {
        attach_trace(&mut body, vec![5]);
    }
    sample(&mut body, &active, &active_attachment, 1, 2);
    sample(&mut body, &active, &active_attachment, 2, 8)
}

#[test]
fn composition_active_work_does_not_follow_dormant_replacements() {
    let small = one_active_trace_with_dormant_replacements(1);
    let large = one_active_trace_with_dormant_replacements(64);

    assert_eq!(large.work, small.work);
    assert_eq!(
        large.execution_cost.local_structural_scans,
        small.execution_cost.local_structural_scans
    );
    assert_eq!(
        large.execution_cost.active_frontier_total,
        small.execution_cost.active_frontier_total
    );
    assert!(large.memory_bytes > small.memory_bytes);
    assert!(large.naturally_quiescent);
}

#[test]
fn attachment_ordinary_physics_assimilates_a_disturbed_sensor_without_a_motor_map() {
    let mut builder = HarnessBuilder::with_capacity(512, 2_048, 1);
    builder.set_physical_tracing(true);
    builder.set_protocol(Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime);
    let anchor = fixed_junction(&mut builder, 90_000, 10_000, 0, 99);
    let motors = [
        fixed_junction(&mut builder, 20_000, 9, 0, 2),
        fixed_junction(&mut builder, 20_001, 11, 0, 2),
    ];
    let sinks = [
        fixed_junction(&mut builder, 30_000, 9, 1, 1),
        fixed_junction(&mut builder, 30_001, 11, 1, 1),
    ];
    let outcomes = [
        fixed_junction(&mut builder, 40_000, 1_000, 0, 1),
        fixed_junction(&mut builder, 40_001, 1_001, 0, 1),
    ];
    for index in 0..2 {
        fixed_link(&mut builder, motors[index], sinks[index]);
        fixed_link(&mut builder, anchor, outcomes[index]);
        builder.set_outcome_source_for_output(motors[index], outcomes[index]);
    }
    let mut body = builder.build();
    let trace = PhysicalTraceSpec::new(vec![0], TRACE_LIFETIME)
        .unwrap()
        .build();
    let attachment = body
        .attach_physical(AttachmentSite::new(10, 0), trace.component())
        .unwrap();
    let rise = junction_for_origin(&body, trace.rise_origin(&attachment, 0).unwrap());
    let fixed_links = body.read().links.len();

    sample(&mut body, &trace, &attachment, 1, -1);
    let mut disturbed = trace
        .sample_inputs(&attachment, 2, 1, PhysicalIncidence::Transition)
        .unwrap();
    disturbed.extend(
        motors
            .iter()
            .enumerate()
            .map(|(index, motor)| PhysicalInput {
                input: Input {
                    target: *motor,
                    arrival_tick: 4,
                    phase: 0,
                    impulse: 1,
                    origin_physical: 40_000_u64.saturating_add(u64::try_from(index).unwrap()),
                },
                incidence: PhysicalIncidence::Sample,
            }),
    );
    let first_action = body.send_physical(&disturbed);

    assert!(fired(&first_action, rise));
    assert!(first_action.naturally_quiescent);
    assert!(
        !first_action.outputs.is_empty(),
        "{:#?}",
        first_action.physical_trace
    );
    assert!(body.read().links.len() > fixed_links);
    assert!(
        first_action
            .physical_trace
            .iter()
            .any(|transition| matches!(
                transition.event,
                PhysicalEvent::JunctionProposal { source, target, .. }
                    if source == rise && motors.contains(&target)
            )),
        "links: {:#?}\ntrace: {:#?}",
        body.read().links,
        first_action.physical_trace
    );

    let action = first_action.outputs[0];
    let returned = trace
        .sample_inputs_from(
            &attachment,
            body.read().clock.tick.saturating_add(1),
            -1,
            PhysicalIncidence::Transition,
            action.from_physical,
        )
        .unwrap();
    let return_run = body.send_physical(&returned);
    assert!(return_run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::PhysicalIncidenceObserved { origin_physical, .. }
            if origin_physical == action.from_physical
    )));
    assert!(return_run.naturally_quiescent);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RegulationRun {
    positions: Vec<i32>,
    directions: Vec<i32>,
    output_origins: Vec<Vec<u64>>,
    consequence_observed: Vec<bool>,
    decision_trace: Vec<Vec<String>>,
    returned_origins_preserved: bool,
    naturally_quiescent: bool,
}

fn regulation_body() -> (
    Harness,
    PhysicalTraceComponent,
    truelearner_core::PhysicalAttachment,
    [JunctionId; 2],
) {
    regulation_body_variant(false, false)
}

fn regulation_body_variant(
    factored_change: bool,
    local_outcomes: bool,
) -> (
    Harness,
    PhysicalTraceComponent,
    truelearner_core::PhysicalAttachment,
    [JunctionId; 2],
) {
    regulation_body_custom(
        (-4..=4).collect(),
        if factored_change { 1 } else { 0 },
        local_outcomes,
    )
}

fn regulation_body_custom(
    thresholds: Vec<i32>,
    change_factor: u8,
    local_outcomes: bool,
) -> (
    Harness,
    PhysicalTraceComponent,
    truelearner_core::PhysicalAttachment,
    [JunctionId; 2],
) {
    regulation_body_custom_protocol(
        thresholds,
        change_factor,
        local_outcomes,
        Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
    )
}

fn regulation_body_custom_protocol(
    thresholds: Vec<i32>,
    change_factor: u8,
    local_outcomes: bool,
    protocol: Protocol,
) -> (
    Harness,
    PhysicalTraceComponent,
    truelearner_core::PhysicalAttachment,
    [JunctionId; 2],
) {
    let mut builder = HarnessBuilder::with_capacity(1_024, 4_096, 1);
    builder.set_physical_tracing(true);
    builder.set_protocol(protocol);
    let anchor = fixed_junction(&mut builder, 90_000, 10_000, 0, 99);
    let motors = [
        fixed_junction(&mut builder, 20_000, 9, 0, 2),
        fixed_junction(&mut builder, 20_001, 11, 0, 2),
    ];
    for (index, motor) in motors.iter().enumerate() {
        let offset = u64::try_from(index).unwrap();
        let sink = fixed_junction(
            &mut builder,
            30_000_u64.saturating_add(offset),
            9_i32.saturating_add(i32::try_from(index.saturating_mul(2)).unwrap()),
            1,
            1,
        );
        let outcome = fixed_junction(
            &mut builder,
            40_000_u64.saturating_add(offset),
            if local_outcomes {
                10
            } else {
                1_000_i32.saturating_add(i32::try_from(index).unwrap())
            },
            0,
            1,
        );
        fixed_link(&mut builder, *motor, sink);
        fixed_link(&mut builder, anchor, outcome);
        builder.set_outcome_source_for_output(*motor, outcome);
    }
    let mut body = builder.build();
    let trace_spec = PhysicalTraceSpec::new(thresholds, TRACE_LIFETIME).unwrap();
    let trace = match change_factor {
        0 => trace_spec.build(),
        1 => trace_spec.build_factored_change(),
        2 => trace_spec.build_unified_change(),
        3 => trace_spec.build_calibration(),
        _ => unreachable!("test change factor is closed"),
    };
    let attachment = body
        .attach_physical(AttachmentSite::new(10, 0), trace.component())
        .unwrap();
    sample(&mut body, &trace, &attachment, 1, 0);
    (body, trace, attachment, motors)
}

fn regulate_from(
    mut body: Harness,
    trace: &PhysicalTraceComponent,
    attachment: &truelearner_core::PhysicalAttachment,
    motors: [JunctionId; 2],
    disturbance: i32,
) -> RegulationRun {
    let mut position = disturbance;
    let mut previous = 0;
    let mut cause = None;
    let mut positions = Vec::new();
    let mut directions = Vec::new();
    let mut output_origins = Vec::new();
    let mut consequence_observed = Vec::new();
    let mut decision_trace = Vec::new();
    let mut returned_origins_preserved = true;
    let mut naturally_quiescent = true;

    for _ in 0..32 {
        let tick = body.read().clock.tick.saturating_add(1);
        let incidence = if position == previous {
            PhysicalIncidence::Sample
        } else {
            PhysicalIncidence::Transition
        };
        let expected_origin =
            cause.unwrap_or_else(|| attachment.port(0).unwrap().origin_physical());
        let mut inputs = trace
            .sample_inputs_from(attachment, tick, position, incidence, expected_origin)
            .unwrap();
        inputs.extend(
            motors
                .iter()
                .enumerate()
                .map(|(index, motor)| PhysicalInput {
                    input: Input {
                        target: *motor,
                        arrival_tick: tick.saturating_add(2),
                        phase: 0,
                        impulse: 1,
                        origin_physical: 40_000_u64.saturating_add(u64::try_from(index).unwrap()),
                    },
                    incidence: PhysicalIncidence::Sample,
                }),
        );
        let run = body.send_physical(&inputs);
        output_origins.push(
            run.outputs
                .iter()
                .map(|output| output.from_physical)
                .collect(),
        );
        consequence_observed.push(run.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::ConsequenceRecorded { .. }
                    | PhysicalEvent::LearnerConsequenceRecorded { .. }
                    | PhysicalEvent::OutputCandidateEvaluated {
                        consequence_tick: Some(_),
                        ..
                    }
            )
        }));
        decision_trace.push(
            run.physical_trace
                .iter()
                .filter(|transition| {
                    matches!(
                        transition.event,
                        PhysicalEvent::ReturnOriginAdmission { .. }
                            | PhysicalEvent::ConsequenceRecorded { .. }
                            | PhysicalEvent::LearnerConsequenceRecorded { .. }
                            | PhysicalEvent::LearnerCandidatePreference { .. }
                            | PhysicalEvent::OutputCandidateEvaluated { .. }
                            | PhysicalEvent::CandidateSelection { .. }
                            | PhysicalEvent::FreshOpportunityTransferred { .. }
                            | PhysicalEvent::PhysicalTransitionContinuationEvaluated { .. }
                            | PhysicalEvent::CoherentEffectEvaluated { .. }
                            | PhysicalEvent::CompletedCycleContinuationEvaluated { .. }
                            | PhysicalEvent::OutputChoiceResolved { .. }
                            | PhysicalEvent::Output(_)
                    )
                })
                .map(|transition| format!("{transition:?}"))
                .collect(),
        );
        naturally_quiescent &= run.naturally_quiescent;
        if cause.is_some() {
            returned_origins_preserved &= run.physical_trace.iter().any(|transition| matches!(
                transition.event,
                PhysicalEvent::PhysicalIncidenceObserved { origin_physical, incidence: PhysicalIncidence::Transition, .. }
                    if origin_physical == expected_origin
            ));
        }

        let effort = [20_000_u64, 20_001_u64].map(|physical| {
            run.outputs
                .iter()
                .filter(|output| output.from_physical == physical)
                .map(|output| output.impulse.abs())
                .sum::<i32>()
        });
        let direction = match effort[1].cmp(&effort[0]) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };
        previous = position;
        let next = position.saturating_add(direction).clamp(-4, 4);
        cause = (next != position).then_some(if direction < 0 { 20_000 } else { 20_001 });
        position = next;
        positions.push(position);
        directions.push(direction);
    }

    RegulationRun {
        positions,
        directions,
        output_origins,
        consequence_observed,
        decision_trace,
        returned_origins_preserved,
        naturally_quiescent,
    }
}

fn held_central_region(positions: &[i32]) -> bool {
    positions
        .windows(4)
        .any(|window| window.iter().all(|position| (-1..=1).contains(position)))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CalibrationBand {
    low: i32,
    high: i32,
}

impl CalibrationBand {
    fn contains(self, value: i32) -> bool {
        (self.low..=self.high).contains(&value)
    }
}

fn regulation_residual(body: &CalibrationBand, value: &i32) -> Residual {
    let amount = if *value < body.low {
        body.low.abs_diff(*value)
    } else if *value > body.high {
        value.abs_diff(body.high)
    } else {
        0
    };
    Residual::new(amount)
}

fn regulation_residual_amount(body: CalibrationBand, value: i32) -> u32 {
    regulation_residual(&body, &value).amount()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum CalibrationAblation {
    #[default]
    Complete,
    PersistentDriveAfterFirst,
    DirectionalChange,
    ZeroIdentity,
}

fn regulate_calibrated_from(
    body: Harness,
    trace: &PhysicalTraceComponent,
    attachment: &truelearner_core::PhysicalAttachment,
    motors: [JunctionId; 2],
    disturbance: i32,
    normal: CalibrationBand,
    motor_effects: [i32; 2],
) -> RegulationRun {
    regulate_calibrated_with(
        body,
        trace,
        attachment,
        motors,
        disturbance,
        normal,
        motor_effects,
        CalibrationAblation::Complete,
    )
}

#[allow(clippy::too_many_arguments)]
fn regulate_calibrated_with(
    mut body: Harness,
    trace: &PhysicalTraceComponent,
    attachment: &truelearner_core::PhysicalAttachment,
    motors: [JunctionId; 2],
    disturbance: i32,
    calibration_context: CalibrationBand,
    motor_effects: [i32; 2],
    ablation: CalibrationAblation,
) -> RegulationRun {
    let mut normalizer = calibrate(calibration_context, regulation_residual);
    let mut position = disturbance;
    let mut previous = calibration_context.low;
    let mut cause = None;
    let mut positions = Vec::new();
    let mut directions = Vec::new();
    let mut output_origins = Vec::new();
    let mut consequence_observed = Vec::new();
    let mut decision_trace = Vec::new();
    let mut returned_origins_preserved = true;
    let mut naturally_quiescent = true;

    for observation in 0..32 {
        let tick = body.read().clock.tick.saturating_add(1);
        let sensor_incidence = if position == previous {
            SensorIncidence::Sample
        } else {
            SensorIncidence::Transition
        };
        let expected_origin =
            cause.unwrap_or_else(|| attachment.port(0).unwrap().origin_physical());
        let normalized = normalizer.step(Signal::new(
            Origin(expected_origin),
            sensor_incidence,
            Availability::Available(position),
        ));
        assert_eq!(normalized.origin, Origin(expected_origin));
        let residual = match normalized.value {
            Availability::Available(residual) => residual,
            Availability::Unavailable => unreachable!("the regulation sensor remained available"),
        };
        let physical_incidence = match normalized.incidence {
            SensorIncidence::Sample => PhysicalIncidence::Sample,
            SensorIncidence::Transition => PhysicalIncidence::Transition,
        };
        let mut inputs = trace
            .sample_inputs_from(
                attachment,
                tick,
                i32::try_from(residual.amount()).expect("bounded residual fits i32"),
                physical_incidence,
                expected_origin,
            )
            .unwrap();
        match ablation {
            CalibrationAblation::Complete => {}
            CalibrationAblation::PersistentDriveAfterFirst if observation > 0 => {
                inputs.retain(|input| input.input.phase != 7);
            }
            CalibrationAblation::DirectionalChange => {
                for input in &mut inputs {
                    if input.input.phase == 7 {
                        input.incidence = physical_incidence;
                    }
                }
            }
            CalibrationAblation::ZeroIdentity if residual.is_quiet() => {
                let drive = attachment
                    .port(attachment.len().saturating_sub(1))
                    .expect("calibration attachment retains its drive port");
                inputs.push(PhysicalInput {
                    input: Input {
                        target: drive.target(),
                        arrival_tick: tick,
                        phase: 7,
                        impulse: 1,
                        origin_physical: expected_origin,
                    },
                    incidence: PhysicalIncidence::Sample,
                });
            }
            CalibrationAblation::PersistentDriveAfterFirst | CalibrationAblation::ZeroIdentity => {}
        }
        inputs.extend(
            motors
                .iter()
                .enumerate()
                .map(|(index, motor)| PhysicalInput {
                    input: Input {
                        target: *motor,
                        arrival_tick: tick.saturating_add(2),
                        phase: 0,
                        impulse: 1,
                        origin_physical: 40_000_u64.saturating_add(u64::try_from(index).unwrap()),
                    },
                    incidence: PhysicalIncidence::Sample,
                }),
        );
        let run = body.send_physical(&inputs);
        output_origins.push(
            run.outputs
                .iter()
                .map(|output| output.from_physical)
                .collect(),
        );
        consequence_observed.push(run.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::ConsequenceRecorded { .. }
                    | PhysicalEvent::LearnerConsequenceRecorded { .. }
                    | PhysicalEvent::NaturalCycleClosed { .. }
            )
        }));
        decision_trace.push(
            run.physical_trace
                .iter()
                .filter(|transition| {
                    matches!(
                        transition.event,
                        PhysicalEvent::NaturalCycleClosureEvaluated { .. }
                            | PhysicalEvent::NaturalCycleClosed { .. }
                            | PhysicalEvent::ReturnOriginAdmission { .. }
                            | PhysicalEvent::ConsequenceRecorded { .. }
                            | PhysicalEvent::LearnerConsequenceRecorded { .. }
                            | PhysicalEvent::OutputCandidateEvaluated { .. }
                            | PhysicalEvent::CandidateSelection { .. }
                            | PhysicalEvent::OutputChoiceResolved { .. }
                            | PhysicalEvent::Output(_)
                    )
                })
                .map(|transition| format!("{transition:?}"))
                .collect(),
        );
        naturally_quiescent &= run.naturally_quiescent;
        if cause.is_some() {
            returned_origins_preserved &= run.physical_trace.iter().any(|transition| matches!(
                transition.event,
                PhysicalEvent::PhysicalIncidenceObserved { origin_physical, incidence: PhysicalIncidence::Transition, .. }
                    if origin_physical == expected_origin
            ));
        }

        let effort = [20_000_u64, 20_001_u64].map(|physical| {
            run.outputs
                .iter()
                .filter(|output| output.from_physical == physical)
                .map(|output| output.impulse.abs())
                .sum::<i32>()
        });
        let net_effect = effort[0]
            .saturating_mul(motor_effects[0])
            .saturating_add(effort[1].saturating_mul(motor_effects[1]));
        let direction = net_effect.signum();
        previous = position;
        let next = position.saturating_add(direction).clamp(-4, 4);
        cause = match (next != position, effort[0] > 0, effort[1] > 0) {
            (true, true, false) => Some(20_000),
            (true, false, true) => Some(20_001),
            _ => None,
        };
        position = next;
        positions.push(position);
        directions.push(direction);
    }

    RegulationRun {
        positions,
        directions,
        output_origins,
        consequence_observed,
        decision_trace,
        returned_origins_preserved,
        naturally_quiescent,
    }
}

fn held_in_band(positions: &[i32], normal: CalibrationBand) -> bool {
    positions
        .windows(4)
        .any(|window| window.iter().all(|position| normal.contains(*position)))
}

fn assert_calibration_controls(
    name: &str,
    disturbance: i32,
    normal: CalibrationBand,
    run: &RegulationRun,
) {
    let observed_positions = std::iter::once(disturbance)
        .chain(
            run.positions
                .iter()
                .copied()
                .take(run.positions.len().saturating_sub(1)),
        )
        .collect::<Vec<_>>();
    for index in 1..observed_positions.len() {
        let before = regulation_residual_amount(normal, observed_positions[index - 1]);
        let current = regulation_residual_amount(normal, observed_positions[index]);
        if current >= before {
            assert!(
                !run.consequence_observed[index],
                "{name} credited a same-or-larger residual at observation {index}: {before} -> {current}; decisions={:#?}",
                run.decision_trace[index]
            );
        } else if current > 0 {
            assert!(
                run.output_origins[index].is_empty(),
                "{name} acted before the smaller-residual return closed at observation {index}"
            );
        }
        if current == 0 {
            assert!(
                run.output_origins[index].is_empty(),
                "{name} acted from the residual identity at observation {index}"
            );
        }
    }
}

fn calibrated_regulation_body() -> (
    Harness,
    PhysicalTraceComponent,
    truelearner_core::PhysicalAttachment,
    [JunctionId; 2],
) {
    regulation_body_custom_protocol(
        vec![1, 2, 3, 4, 5],
        3,
        false,
        Protocol::RecursiveLearnerCausalTopologyProductCompositionNaturalCycleClosure,
    )
}

#[test]
fn regulation_parent_reproduces_first_reversal() {
    let (body, trace, attachment, motors) = regulation_body_variant(false, false);
    let observed = regulate_from(body, &trace, &attachment, motors, 3);

    assert_eq!(&observed.positions[..2], &[2, 3]);
    assert_eq!(observed.output_origins[0], vec![20_000]);
    assert_eq!(observed.output_origins[1], vec![20_001]);
    assert!(!observed.consequence_observed[1]);
    assert!(observed.returned_origins_preserved);
    assert!(observed.naturally_quiescent);
}

#[test]
#[ignore = "frozen negative: outcome placement did not close the return"]
fn regulation_outcome_boundary_attachment_closes_first_return() {
    let (body, trace, attachment, motors) = regulation_body_variant(false, true);
    let observed = regulate_from(body, &trace, &attachment, motors, 3);

    assert_eq!(observed.output_origins[0], vec![20_000]);
    assert!(
        observed.consequence_observed[1],
        "first two decisions: {:#?}",
        &observed.decision_trace[..2]
    );
    assert_eq!(observed.output_origins[1], vec![20_000]);
    assert!(observed.returned_origins_preserved);
    assert!(observed.naturally_quiescent);
}

#[test]
#[ignore = "frozen negative: shared surface continued without consequence write"]
fn regulation_shared_change_surface_closes_first_return() {
    let (body, trace, attachment, motors) = regulation_body_variant(true, false);
    let observed = regulate_from(body, &trace, &attachment, motors, 3);

    assert_eq!(observed.output_origins[0], vec![20_000]);
    assert!(
        observed.consequence_observed[1],
        "first two decisions: {:#?}",
        &observed.decision_trace[..2]
    );
    assert_eq!(observed.output_origins[1], vec![20_000]);
    assert!(observed.returned_origins_preserved);
    assert!(observed.naturally_quiescent);
}

#[test]
#[ignore = "frozen negative: split trace reverses before local regulation"]
fn regulation_runtime_attached_scalar_returns_from_both_disturbances() {
    let (body, trace, attachment, motors) = regulation_body();
    let checkpoint = body.save().unwrap();

    let positive = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        3,
    );
    let positive_replay = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        3,
    );
    let negative = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
    );
    let negative_replay = regulate_from(
        Harness::restore(checkpoint).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
    );

    assert_eq!(positive_replay, positive);
    assert_eq!(negative_replay, negative);
    assert!(positive.returned_origins_preserved);
    assert!(negative.returned_origins_preserved);
    assert!(positive.naturally_quiescent);
    assert!(negative.naturally_quiescent);
    assert!(
        held_central_region(&positive.positions),
        "positive: {positive:#?}"
    );
    assert!(
        held_central_region(&negative.positions),
        "negative: {negative:#?}"
    );
    assert!(positive
        .directions
        .iter()
        .chain(&negative.directions)
        .any(|direction| *direction < 0));
    assert!(positive
        .directions
        .iter()
        .chain(&negative.directions)
        .any(|direction| *direction > 0));
}

#[test]
#[ignore = "frozen negative: per-threshold factor lost continuation at the next threshold"]
fn regulation_factored_change_returns_from_both_disturbances() {
    let (body, trace, attachment, motors) = regulation_body_variant(true, false);
    let checkpoint = body.save().unwrap();
    let positive = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        3,
    );
    let positive_replay = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        3,
    );
    let negative = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
    );
    let negative_replay = regulate_from(
        Harness::restore(checkpoint).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
    );

    assert_eq!(positive_replay, positive);
    assert_eq!(negative_replay, negative);
    assert!(positive.returned_origins_preserved);
    assert!(negative.returned_origins_preserved);
    assert!(positive.naturally_quiescent);
    assert!(negative.naturally_quiescent);
    assert!(
        held_central_region(&positive.positions),
        "positive: {positive:#?}"
    );
    assert!(
        held_central_region(&negative.positions),
        "negative: {negative:#?}"
    );
    assert!(positive
        .directions
        .iter()
        .chain(&negative.directions)
        .any(|direction| *direction < 0));
    assert!(positive
        .directions
        .iter()
        .chain(&negative.directions)
        .any(|direction| *direction > 0));
}

#[test]
#[ignore = "frozen negative: rest entry fired another action instead of becoming identity"]
fn regulation_unified_homeostatic_surface_returns_from_both_disturbances() {
    let (body, trace, attachment, motors) = regulation_body_custom(vec![-2, -1, 2, 3], 2, false);
    let checkpoint = body.save().unwrap();
    let positive = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        3,
    );
    let positive_replay = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        3,
    );
    let negative = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
    );
    let negative_replay = regulate_from(
        Harness::restore(checkpoint).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
    );

    assert_eq!(positive_replay, positive);
    assert_eq!(negative_replay, negative);
    assert!(positive.returned_origins_preserved);
    assert!(negative.returned_origins_preserved);
    assert!(positive.naturally_quiescent);
    assert!(negative.naturally_quiescent);
    assert!(
        held_central_region(&positive.positions),
        "positive: {positive:#?}"
    );
    assert!(
        held_central_region(&negative.positions),
        "negative: {negative:#?}"
    );
    assert!(positive
        .directions
        .iter()
        .chain(&negative.directions)
        .any(|direction| *direction < 0));
    assert!(positive
        .directions
        .iter()
        .chain(&negative.directions)
        .any(|direction| *direction > 0));
}

#[test]
#[ignore = "frozen negative: exact closure stalls at the first still-disturbed state"]
fn regulation_attachment_natural_closure() {
    let (body, trace, attachment, motors) = regulation_body_custom_protocol(
        vec![-2, -1, 2, 3],
        2,
        false,
        Protocol::RecursiveLearnerCausalTopologyProductCompositionNaturalCycleClosure,
    );
    let checkpoint = body.save().unwrap();
    let positive = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        3,
    );
    let positive_replay = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        3,
    );
    let negative = regulate_from(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
    );
    let negative_replay = regulate_from(
        Harness::restore(checkpoint).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
    );

    assert_eq!(positive_replay, positive);
    assert_eq!(negative_replay, negative);
    assert!(positive.returned_origins_preserved);
    assert!(negative.returned_origins_preserved);
    assert!(positive.naturally_quiescent);
    assert!(negative.naturally_quiescent);
    assert!(
        held_central_region(&positive.positions) && held_central_region(&negative.positions),
        "positive positions: {:?}, outputs: {:?}, consequence: {:?}; negative positions: {:?}, outputs: {:?}, consequence: {:?}",
        positive.positions,
        positive.output_origins,
        positive.consequence_observed,
        negative.positions,
        negative.output_origins,
        negative.consequence_observed,
    );
    assert!(positive
        .directions
        .iter()
        .chain(&negative.directions)
        .any(|direction| *direction < 0));
    assert!(positive
        .directions
        .iter()
        .chain(&negative.directions)
        .any(|direction| *direction > 0));
}

#[test]
fn regulation_body_curried_calibration() {
    let (body, trace, attachment, motors) = calibrated_regulation_body();
    let checkpoint = body.save().unwrap();
    let centered = CalibrationBand { low: -1, high: 1 };
    let shifted = CalibrationBand { low: 1, high: 2 };
    let cases = [
        ("center-positive", 3, centered, [-1, 1]),
        ("center-negative", -3, centered, [-1, 1]),
        ("reflected-positive", 3, centered, [1, -1]),
        ("reflected-negative", -3, centered, [1, -1]),
        ("shifted-positive", 4, shifted, [-1, 1]),
        ("shifted-negative", -2, shifted, [-1, 1]),
    ];
    let mut observed = Vec::new();

    for (name, disturbance, normal, motor_effects) in cases {
        let run = regulate_calibrated_from(
            Harness::restore(checkpoint.clone()).unwrap(),
            &trace,
            &attachment,
            motors,
            disturbance,
            normal,
            motor_effects,
        );
        let replay = regulate_calibrated_from(
            Harness::restore(checkpoint.clone()).unwrap(),
            &trace,
            &attachment,
            motors,
            disturbance,
            normal,
            motor_effects,
        );

        assert_eq!(replay, run, "{name} did not replay exactly");
        assert!(
            run.returned_origins_preserved,
            "{name} lost the action that physically caused a sensor return"
        );
        assert!(run.naturally_quiescent, "{name} did not quiesce per step");
        assert!(
            held_in_band(&run.positions, normal),
            "{name} did not hold its body relation: positions={:?}, outputs={:?}, consequence={:?}",
            run.positions,
            run.output_origins,
            run.consequence_observed,
        );
        assert_calibration_controls(name, disturbance, normal, &run);
        observed.push(run);
    }

    assert!(observed
        .iter()
        .flat_map(|run| &run.directions)
        .any(|direction| *direction < 0));
    assert!(observed
        .iter()
        .flat_map(|run| &run.directions)
        .any(|direction| *direction > 0));
}

#[test]
#[ignore = "frozen ablation: fixed context must fail a disjoint shifted body norm"]
fn calibration_ablation_fixed_context() {
    let (body, trace, attachment, motors) = calibrated_regulation_body();
    let checkpoint = body.save().unwrap();
    let centered = CalibrationBand { low: -1, high: 1 };
    let shifted = CalibrationBand { low: 2, high: 3 };
    let complete = regulate_calibrated_with(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        4,
        shifted,
        [-1, 1],
        CalibrationAblation::Complete,
    );
    let removed = regulate_calibrated_with(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        4,
        centered,
        [-1, 1],
        CalibrationAblation::Complete,
    );
    let replay = regulate_calibrated_with(
        Harness::restore(checkpoint).unwrap(),
        &trace,
        &attachment,
        motors,
        4,
        centered,
        [-1, 1],
        CalibrationAblation::Complete,
    );

    assert_eq!(replay, removed);
    assert!(complete.naturally_quiescent);
    assert!(removed.naturally_quiescent);
    assert!(held_in_band(&complete.positions, shifted));
    assert!(
        !held_in_band(&removed.positions, shifted),
        "fixed context unexpectedly held shifted normal: {removed:#?}"
    );
    assert!(held_in_band(&removed.positions, centered));
}

#[test]
#[ignore = "frozen ablation: one-shot drive must stall after the first useful closure"]
fn calibration_ablation_persistent_drive() {
    let (body, trace, attachment, motors) = calibrated_regulation_body();
    let checkpoint = body.save().unwrap();
    let centered = CalibrationBand { low: -1, high: 1 };
    let removed = regulate_calibrated_with(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        3,
        centered,
        [-1, 1],
        CalibrationAblation::PersistentDriveAfterFirst,
    );
    let replay = regulate_calibrated_with(
        Harness::restore(checkpoint).unwrap(),
        &trace,
        &attachment,
        motors,
        3,
        centered,
        [-1, 1],
        CalibrationAblation::PersistentDriveAfterFirst,
    );

    assert_eq!(replay, removed);
    assert!(removed.naturally_quiescent);
    assert_eq!(removed.positions[0], 2);
    assert!(removed.positions[1..].iter().all(|position| *position == 2));
    assert!(removed.consequence_observed[1]);
    assert!(!held_in_band(&removed.positions, centered));
}

#[test]
#[ignore = "frozen ablation: merged rise and fall must falsely close a worsening residual"]
fn calibration_ablation_directional_change() {
    let (body, trace, attachment, motors) = calibrated_regulation_body();
    let checkpoint = body.save().unwrap();
    let centered = CalibrationBand { low: -1, high: 1 };
    let complete = regulate_calibrated_with(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
        centered,
        [-1, 1],
        CalibrationAblation::Complete,
    );
    let removed = regulate_calibrated_with(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
        centered,
        [-1, 1],
        CalibrationAblation::DirectionalChange,
    );
    let replay = regulate_calibrated_with(
        Harness::restore(checkpoint).unwrap(),
        &trace,
        &attachment,
        motors,
        -3,
        centered,
        [-1, 1],
        CalibrationAblation::DirectionalChange,
    );

    assert_eq!(replay, removed);
    assert!(complete.naturally_quiescent);
    assert!(removed.naturally_quiescent);
    assert_eq!(complete.positions[0], -4);
    assert_eq!(removed.positions[0], -4);
    assert!(!complete.consequence_observed[1]);
    assert!(
        removed.consequence_observed[1],
        "worsening residual did not falsely close: {removed:#?}"
    );
    assert_eq!(regulation_residual_amount(centered, -3), 2);
    assert_eq!(regulation_residual_amount(centered, -4), 3);
}

#[test]
#[ignore = "frozen ablation: drive at zero must act from an initially normal body"]
fn calibration_ablation_zero_identity() {
    let (body, trace, attachment, motors) = calibrated_regulation_body();
    let checkpoint = body.save().unwrap();
    let centered = CalibrationBand { low: -1, high: 1 };
    let complete = regulate_calibrated_with(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        0,
        centered,
        [-1, 1],
        CalibrationAblation::Complete,
    );
    let removed = regulate_calibrated_with(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        0,
        centered,
        [-1, 1],
        CalibrationAblation::ZeroIdentity,
    );
    let replay = regulate_calibrated_with(
        Harness::restore(checkpoint).unwrap(),
        &trace,
        &attachment,
        motors,
        0,
        centered,
        [-1, 1],
        CalibrationAblation::ZeroIdentity,
    );

    assert_eq!(replay, removed);
    assert!(complete.naturally_quiescent);
    assert!(removed.naturally_quiescent);
    assert!(complete.positions.iter().all(|position| *position == 0));
    assert!(complete.output_origins.iter().all(Vec::is_empty));
    assert!(
        removed.positions.iter().any(|position| *position != 0)
            || removed
                .output_origins
                .iter()
                .any(|outputs| !outputs.is_empty()),
        "driven zero unexpectedly remained identity: {removed:#?}"
    );
}

fn final_four_in_band(run: &RegulationRun, normal: CalibrationBand) -> bool {
    run.positions
        .iter()
        .rev()
        .take(4)
        .all(|position| normal.contains(*position))
}

#[test]
#[ignore = "frozen successor reference: complete calibration must finish in shifted context"]
fn calibration_terminal_shifted_context_reference() {
    let (body, trace, attachment, motors) = calibrated_regulation_body();
    let checkpoint = body.save().unwrap();
    let shifted = CalibrationBand { low: 2, high: 3 };
    let complete = regulate_calibrated_with(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        4,
        shifted,
        [-1, 1],
        CalibrationAblation::Complete,
    );
    let replay = regulate_calibrated_with(
        Harness::restore(checkpoint).unwrap(),
        &trace,
        &attachment,
        motors,
        4,
        shifted,
        [-1, 1],
        CalibrationAblation::Complete,
    );

    assert_eq!(replay, complete);
    assert!(complete.naturally_quiescent);
    assert!(
        final_four_in_band(&complete, shifted),
        "complete terminal positions left shifted context: {:?}",
        &complete.positions[complete.positions.len().saturating_sub(4)..]
    );
}

#[test]
#[ignore = "frozen successor: fixed calibration must finish outside the disjoint body norm"]
fn calibration_ablation_fixed_context_terminal_residence() {
    let (body, trace, attachment, motors) = calibrated_regulation_body();
    let checkpoint = body.save().unwrap();
    let centered = CalibrationBand { low: -1, high: 1 };
    let shifted = CalibrationBand { low: 2, high: 3 };
    let removed = regulate_calibrated_with(
        Harness::restore(checkpoint.clone()).unwrap(),
        &trace,
        &attachment,
        motors,
        4,
        centered,
        [-1, 1],
        CalibrationAblation::Complete,
    );
    let replay = regulate_calibrated_with(
        Harness::restore(checkpoint).unwrap(),
        &trace,
        &attachment,
        motors,
        4,
        centered,
        [-1, 1],
        CalibrationAblation::Complete,
    );

    assert_eq!(replay, removed);
    assert!(removed.naturally_quiescent);
    assert!(!final_four_in_band(&removed, shifted));
    assert!(final_four_in_band(&removed, centered));
    assert_eq!(
        &removed.positions[removed.positions.len().saturating_sub(4)..],
        &[1, 1, 1, 1]
    );
}
