use academy_workstation::{WorkstationPresentation, WorkstationWorld};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use truelearner_core::{
    Checkpoint, Harness, HarnessBuilder, Input, JunctionId, PhysicalIncidence, PhysicalInput,
    Protocol,
};
use truelearner_embodiment::{
    Availability, ChangeDetector, DriveSpec, DriverBank, FocusProfile, FocusedReceptorFrame,
    Incidence, JunctionSpec, Origin, Signal, SpatialField, Wiring,
};
use truelearner_workstation::{BODY_MAX, Eye, LightField, WorkstationState};

const REFINEMENT_DEPTH: usize = 7;
const BITS_PER_REGION: usize = u32::BITS as usize;
const OUTWARD_REGION: i16 = 1;
const SENSOR_PHYSICAL_BASE: u64 = 400_000;
const RELAY_PHYSICAL_BASE: u64 = 500_000;
const SIGNAL_ORIGIN_BASE: u64 = 600_000;

#[derive(Serialize)]
struct Projection {
    schema: &'static str,
    source_trace: PathBuf,
    source_sha256: String,
    cue_a: char,
    cue_b: char,
    field_shape: [usize; 2],
    region_bound_per_eye: usize,
    bits_per_region: usize,
    fixed_feature_count: usize,
    receptor_junction_count: usize,
    fixture_total_junctions: usize,
    fixture_total_links: usize,
    focused: PairProjection,
    no_focus: PairProjection,
    controls: Controls,
}

#[derive(Serialize)]
struct PairProjection {
    active_regions_blank: [usize; 2],
    active_regions_a: [usize; 2],
    active_regions_b: [usize; 2],
    frame_a_differs_from_b: bool,
    changed_features_a: usize,
    changed_features_b: usize,
    changed_feature_sets_differ: bool,
    branches_differ_in_core: bool,
    cue_a: BranchSummary,
    cue_b: BranchSummary,
    exact_replay_a: bool,
    exact_replay_b: bool,
    reversed_evaluation_order_equal: bool,
    same_cue_transition_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct BranchSummary {
    admitted_inputs: usize,
    naturally_quiescent: bool,
    outputs: usize,
    physical_work: u64,
    drive_deliveries: u64,
    learner_constructions: u64,
    participating_links: usize,
    learner_count: usize,
    resident_bytes: usize,
    core_fingerprint: String,
}

#[derive(Serialize)]
struct Controls {
    initial_core_fingerprint: String,
    initial_resident_bytes: usize,
    no_admission_preserves_core: bool,
    blank_initialization_is_all_sample: bool,
    focused_input_count_within_bound: bool,
    no_focus_also_distinguishes_cues: bool,
}

#[derive(Clone)]
struct SceneFrame {
    features: Vec<Availability<bool>>,
    active_regions: [usize; 2],
}

#[derive(Clone)]
struct ReceptorFixture {
    checkpoint: Checkpoint,
    targets: Vec<[JunctionId; 3]>,
    initial_core_fingerprint: String,
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let source_trace = PathBuf::from(
        args.next()
            .expect("usage: focused_receptor_participation TRACE OUTPUT"),
    );
    let output = PathBuf::from(
        args.next()
            .expect("usage: focused_receptor_participation TRACE OUTPUT"),
    );
    assert!(args.next().is_none(), "too many arguments");

    let source = std::fs::read(&source_trace).expect("source trace reads");
    let value: serde_json::Value = serde_json::from_slice(&source).expect("trace parses");
    let state: WorkstationState = serde_json::from_value(
        value["development"]
            .as_array()
            .and_then(|steps| steps.last())
            .expect("trace has development")
            .get("state_after")
            .expect("development retains state")
            .clone(),
    )
    .expect("state parses");
    let glyphs = value["learned_glyphs"]
        .as_array()
        .expect("trace has learned glyphs");
    let cue = |index: usize| {
        glyphs[index]
            .as_str()
            .and_then(|glyph| glyph.chars().next())
            .expect("glyph is one character")
    };
    let cue_a = cue(0);
    let cue_b = cue(1);
    let blank_sample = sample(&state, WorkstationPresentation::default());
    let sample_a = sample(&state, WorkstationPresentation::with_monitor_glyph(cue_a));
    let sample_b = sample(&state, WorkstationPresentation::with_monitor_glyph(cue_b));
    let profile = FocusProfile::new(REFINEMENT_DEPTH, 1).expect("profile is bounded");
    let blank = scene_frame(&blank_sample, &state, profile, true);
    let frame_a = scene_frame(&sample_a, &state, profile, true);
    let frame_b = scene_frame(&sample_b, &state, profile, true);
    let no_focus_blank = scene_frame(&blank_sample, &state, profile, false);
    let no_focus_a = scene_frame(&sample_a, &state, profile, false);
    let no_focus_b = scene_frame(&sample_b, &state, profile, false);
    let feature_count = blank.features.len();
    assert_eq!(feature_count, 2 * profile.region_bound() * BITS_PER_REGION);
    let fixture = receptor_fixture(feature_count);

    let (focused, focused_blank_is_sample) = pair_projection(&fixture, &blank, &frame_a, &frame_b);
    let (no_focus, no_focus_blank_is_sample) =
        pair_projection(&fixture, &no_focus_blank, &no_focus_a, &no_focus_b);
    let no_admission_a = restore_fingerprint(&fixture.checkpoint);
    let no_admission_b = restore_fingerprint(&fixture.checkpoint);
    let projection = Projection {
        schema: "workstation-focused-receptor-participation/v1",
        source_trace,
        source_sha256: hex_digest(&source),
        cue_a,
        cue_b,
        field_shape: [
            usize::from(sample_a.eye(Eye::Left).height()),
            usize::from(sample_a.eye(Eye::Left).width()),
        ],
        region_bound_per_eye: profile.region_bound(),
        bits_per_region: BITS_PER_REGION,
        fixed_feature_count: feature_count,
        receptor_junction_count: feature_count * 3,
        fixture_total_junctions: fixture_junction_count(&fixture.checkpoint),
        fixture_total_links: fixture_link_count(&fixture.checkpoint),
        controls: Controls {
            initial_core_fingerprint: fixture.initial_core_fingerprint.clone(),
            initial_resident_bytes: fixture_resident_bytes(&fixture.checkpoint),
            no_admission_preserves_core: no_admission_a == fixture.initial_core_fingerprint
                && no_admission_b == fixture.initial_core_fingerprint,
            blank_initialization_is_all_sample: focused_blank_is_sample && no_focus_blank_is_sample,
            focused_input_count_within_bound: focused.cue_a.admitted_inputs <= feature_count
                && focused.cue_b.admitted_inputs <= feature_count,
            no_focus_also_distinguishes_cues: no_focus.branches_differ_in_core,
        },
        focused,
        no_focus,
    };
    let encoded = serde_json::to_vec_pretty(&projection).expect("projection serializes");
    std::fs::write(output, encoded).expect("projection writes");
}

fn sample(
    state: &WorkstationState,
    presentation: WorkstationPresentation,
) -> truelearner_workstation::WorldSample {
    WorkstationWorld::new_with_presentation(presentation)
        .expect("cue world builds")
        .sense(state)
        .expect("cue world senses")
}

fn scene_frame(
    sample: &truelearner_workstation::WorldSample,
    state: &WorkstationState,
    profile: FocusProfile<2>,
    focused: bool,
) -> SceneFrame {
    let mut features = Vec::with_capacity(2 * profile.region_bound() * BITS_PER_REGION);
    let mut active_regions = [0; 2];
    for (eye_index, eye) in Eye::ALL.into_iter().enumerate() {
        let image = sample.eye(eye);
        let field = field(image);
        let focus = if focused {
            let gaze = state.eye(eye).gaze();
            let raster_focus = [
                scale_focus(gaze.y(), field.shape()[0]),
                scale_focus(gaze.x(), field.shape()[1]),
            ];
            profile
                .focuses(field.shape(), [raster_focus])
                .expect("physical gaze is inside the raster")
        } else {
            profile
                .focuses(field.shape(), [])
                .expect("empty focus is valid")
        };
        let frame = field
            .focus_partition(focus)
            .expect("focus belongs to rendered field")
            .transduce_complete(0_u64, u64::from, u64::saturating_add)
            .map(|sum| u32::try_from(sum).expect("rendered eye sum fits u32"))
            .into_receptor_frame();
        active_regions[eye_index] = frame.active_region_count();
        append_binary_features(&mut features, frame);
    }
    SceneFrame {
        features,
        active_regions,
    }
}

fn field(image: &LightField) -> SpatialField<u8, 2> {
    SpatialField::new(
        [usize::from(image.height()), usize::from(image.width())],
        image
            .pixels()
            .iter()
            .copied()
            .map(Availability::Available)
            .collect(),
    )
    .expect("rendered image has a valid field shape")
}

fn append_binary_features(
    destination: &mut Vec<Availability<bool>>,
    frame: FocusedReceptorFrame<u32, 2>,
) {
    for slot in frame.slots() {
        for bit in 0..BITS_PER_REGION {
            destination.push(match slot {
                Availability::Available(value) => Availability::Available((value >> bit) & 1 == 1),
                Availability::Unavailable => Availability::Unavailable,
            });
        }
    }
}

fn pair_projection(
    fixture: &ReceptorFixture,
    blank: &SceneFrame,
    cue_a: &SceneFrame,
    cue_b: &SceneFrame,
) -> (PairProjection, bool) {
    let mut seed = DriverBank::new(vec![ChangeDetector::default(); blank.features.len()]);
    let blank_observation = seed
        .step(signals(&blank.features))
        .expect("blank frame has fixed arity");
    let blank_is_sample = blank_observation
        .iter()
        .all(|signal| signal.incidence == Incidence::Sample);

    let mut detector_a = seed.clone();
    let transitions_a = transitions(
        detector_a
            .step(signals(&cue_a.features))
            .expect("cue A frame has fixed arity"),
    );
    let same_cue_transition_count = transitions(
        detector_a
            .step(signals(&cue_a.features))
            .expect("repeated cue A frame has fixed arity"),
    )
    .len();
    let mut detector_b = seed.clone();
    let transitions_b = transitions(
        detector_b
            .step(signals(&cue_b.features))
            .expect("cue B frame has fixed arity"),
    );

    let cue_a_summary = run_branch(fixture, &transitions_a);
    let cue_b_summary = run_branch(fixture, &transitions_b);
    let replay_a = run_branch(fixture, &transitions_a);
    let replay_b = run_branch(fixture, &transitions_b);

    let mut reverse_b = seed.clone();
    let reversed_b = transitions(
        reverse_b
            .step(signals(&cue_b.features))
            .expect("reverse cue B frame has fixed arity"),
    );
    let mut reverse_a = seed;
    let reversed_a = transitions(
        reverse_a
            .step(signals(&cue_a.features))
            .expect("reverse cue A frame has fixed arity"),
    );

    (
        PairProjection {
            active_regions_blank: blank.active_regions,
            active_regions_a: cue_a.active_regions,
            active_regions_b: cue_b.active_regions,
            frame_a_differs_from_b: cue_a.features != cue_b.features,
            changed_features_a: transitions_a.len(),
            changed_features_b: transitions_b.len(),
            changed_feature_sets_differ: transitions_a != transitions_b,
            branches_differ_in_core: cue_a_summary.core_fingerprint
                != cue_b_summary.core_fingerprint,
            cue_a: cue_a_summary.clone(),
            cue_b: cue_b_summary.clone(),
            exact_replay_a: cue_a_summary == replay_a,
            exact_replay_b: cue_b_summary == replay_b,
            reversed_evaluation_order_equal: transitions_a == reversed_a
                && transitions_b == reversed_b,
            same_cue_transition_count,
        },
        blank_is_sample,
    )
}

fn signals(features: &[Availability<bool>]) -> Vec<Signal<Availability<bool>>> {
    features
        .iter()
        .copied()
        .enumerate()
        .map(|(feature, value)| {
            Signal::new(
                Origin(
                    SIGNAL_ORIGIN_BASE.saturating_add(u64::try_from(feature).unwrap_or(u64::MAX)),
                ),
                Incidence::Sample,
                value,
            )
        })
        .collect()
}

fn transitions(signals: Vec<Signal<Availability<bool>>>) -> Vec<Signal<Availability<bool>>> {
    signals
        .into_iter()
        .filter(|signal| signal.incidence == Incidence::Transition)
        .collect()
}

fn receptor_fixture(feature_count: usize) -> ReceptorFixture {
    let junction_capacity = u32::try_from(feature_count.saturating_mul(4))
        .expect("focused receptor fixture junction capacity fits u32");
    let link_capacity = u32::try_from(feature_count.saturating_mul(3))
        .expect("focused receptor fixture link capacity fits u32");
    let mut builder =
        HarnessBuilder::with_capacity(junction_capacity, link_capacity, OUTWARD_REGION);
    builder.set_protocol(Protocol::RecursiveLearnerCausalTopologyProductComposition);
    let mut wiring = Wiring::new(&mut builder);
    let targets = wiring.receptor_bank::<3>(
        feature_count,
        SENSOR_PHYSICAL_BASE,
        |feature, _, physical_id| {
            JunctionSpec::ordinary(physical_id, feature_position(feature), 0, 1)
        },
    );
    let relays = wiring.junction_bank(
        feature_count,
        RELAY_PHYSICAL_BASE,
        |feature, physical_id| {
            JunctionSpec::ordinary(
                physical_id,
                feature_position(feature).saturating_add(1),
                0,
                1,
            )
        },
    );
    for (feature_targets, relay) in targets.iter().zip(relays) {
        for target in feature_targets {
            wiring.drive(*target, relay, DriveSpec::ordinary(1));
        }
    }
    let harness = builder.build();
    let initial_core_fingerprint = core_fingerprint(&harness);
    let checkpoint = harness.save().expect("receptor fixture saves");
    ReceptorFixture {
        checkpoint,
        targets,
        initial_core_fingerprint,
    }
}

fn feature_position(feature: usize) -> i32 {
    i32::try_from(feature)
        .expect("focused receptor feature position fits i32")
        .saturating_mul(8)
}

fn run_branch(
    fixture: &ReceptorFixture,
    transitions: &[Signal<Availability<bool>>],
) -> BranchSummary {
    let mut harness = Harness::restore(fixture.checkpoint.clone()).expect("fixture restores");
    let tick = harness.read().clock.tick.saturating_add(1);
    let inputs = transitions
        .iter()
        .map(|signal| {
            let feature = usize::try_from(signal.origin.0.saturating_sub(SIGNAL_ORIGIN_BASE))
                .expect("signal feature fits usize");
            let value = match signal.value {
                Availability::Unavailable => 0,
                Availability::Available(false) => 1,
                Availability::Available(true) => 2,
            };
            PhysicalInput {
                input: Input {
                    arrival_tick: tick,
                    phase: 0,
                    origin_physical: signal.origin.0,
                    target: fixture.targets[feature][value],
                    impulse: 1,
                },
                incidence: PhysicalIncidence::Transition,
            }
        })
        .collect::<Vec<_>>();
    let run = harness.send_physical(&inputs);
    let observation = harness.read();
    BranchSummary {
        admitted_inputs: inputs.len(),
        naturally_quiescent: run.naturally_quiescent,
        outputs: run.outputs.len(),
        physical_work: run.work.physical_total(),
        drive_deliveries: run.work.drive_deliveries,
        learner_constructions: run.work.learner_constructions,
        participating_links: observation
            .links
            .iter()
            .filter(|link| link.participation > 0)
            .count(),
        learner_count: observation.learners.len(),
        resident_bytes: observation.resident_bytes,
        core_fingerprint: hex_digest(&observation.fingerprint()),
    }
}

fn restore_fingerprint(checkpoint: &Checkpoint) -> String {
    let harness = Harness::restore(checkpoint.clone()).expect("fixture restores");
    core_fingerprint(&harness)
}

fn fixture_junction_count(checkpoint: &Checkpoint) -> usize {
    Harness::restore(checkpoint.clone())
        .expect("fixture restores")
        .read()
        .junctions
        .len()
}

fn fixture_link_count(checkpoint: &Checkpoint) -> usize {
    Harness::restore(checkpoint.clone())
        .expect("fixture restores")
        .read()
        .links
        .len()
}

fn fixture_resident_bytes(checkpoint: &Checkpoint) -> usize {
    Harness::restore(checkpoint.clone())
        .expect("fixture restores")
        .read()
        .resident_bytes
}

fn core_fingerprint(harness: &Harness) -> String {
    hex_digest(&harness.read().fingerprint())
}

fn scale_focus(position: i16, extent: usize) -> usize {
    usize::try_from(position)
        .unwrap_or(0)
        .saturating_mul(extent.saturating_sub(1))
        / usize::try_from(BODY_MAX).unwrap_or(1)
}

fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
