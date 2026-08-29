use academy_workstation::{WorkstationPresentation, WorkstationWorld};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use truelearner_embodiment::{
    Availability, FocusProfile, FocusedField, FocusedPartition, SpatialField,
};
use truelearner_workstation::{BODY_MAX, Eye, LightField, WorkstationState};

const REFINEMENT_DEPTH: usize = 7;
const GRID_INTERVALS: usize = 4;

#[derive(Serialize)]
struct Projection {
    schema: &'static str,
    source_trace: PathBuf,
    source_sha256: String,
    cue_a: char,
    cue_b: char,
    field_shape: [usize; 2],
    profile: ProfileSummary,
    eyes: [EyeProjection; 2],
}

#[derive(Serialize)]
struct ProfileSummary {
    refinement_depth: usize,
    maximum_foci: usize,
    region_bound: usize,
    grid_intervals: usize,
}

#[derive(Serialize)]
struct EyeProjection {
    eye: Eye,
    physical_gaze: [i16; 2],
    raster_focus: [usize; 2],
    actual_region_count: usize,
    actual_focused_region_shape: [usize; 2],
    exact_reassembly_a: bool,
    exact_reassembly_b: bool,
    same_cue_replay_equal: bool,
    blank_control_differs: bool,
    actual_cue_pair_differs: bool,
    actual_differing_region_count: usize,
    no_focus_region_count: usize,
    no_focus_cue_pair_differs: bool,
    grid_focus_count: usize,
    grid_focuses_distinguishing_cues: usize,
    maximum_grid_region_count: usize,
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let source_trace = PathBuf::from(
        args.next()
            .expect("usage: focused_sensor_field_projection TRACE OUTPUT"),
    );
    let output = PathBuf::from(
        args.next()
            .expect("usage: focused_sensor_field_projection TRACE OUTPUT"),
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
    let sample_a = sample(&state, WorkstationPresentation::with_monitor_glyph(cue_a));
    let sample_b = sample(&state, WorkstationPresentation::with_monitor_glyph(cue_b));
    let sample_a_again = sample(&state, WorkstationPresentation::with_monitor_glyph(cue_a));
    let blank = sample(&state, WorkstationPresentation::default());
    let profile = FocusProfile::new(REFINEMENT_DEPTH, 1).expect("profile is bounded");
    let field_shape = [
        usize::from(sample_a.eye(Eye::Left).height()),
        usize::from(sample_a.eye(Eye::Left).width()),
    ];
    let eyes = Eye::ALL.map(|eye| {
        project_eye(
            eye,
            &state,
            sample_a.eye(eye),
            sample_b.eye(eye),
            sample_a_again.eye(eye),
            blank.eye(eye),
            profile,
        )
    });

    let projection = Projection {
        schema: "workstation-focused-sensor-field-projection/v1",
        source_trace,
        source_sha256: hex_digest(&source),
        cue_a,
        cue_b,
        field_shape,
        profile: ProfileSummary {
            refinement_depth: profile.refinement_depth(),
            maximum_foci: profile.maximum_foci(),
            region_bound: profile.region_bound(),
            grid_intervals: GRID_INTERVALS,
        },
        eyes,
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

fn project_eye(
    eye: Eye,
    state: &WorkstationState,
    image_a: &LightField,
    image_b: &LightField,
    image_a_again: &LightField,
    blank: &LightField,
    profile: FocusProfile<2>,
) -> EyeProjection {
    let field_a = field(image_a);
    let field_b = field(image_b);
    let field_a_again = field(image_a_again);
    let blank = field(blank);
    let gaze = state.eye(eye).gaze();
    let physical_gaze = [gaze.x(), gaze.y()];
    let raster_focus = [
        scale_focus(gaze.y(), field_a.shape()[0]),
        scale_focus(gaze.x(), field_a.shape()[1]),
    ];
    let focus = profile
        .focuses(field_a.shape(), [raster_focus])
        .expect("physical gaze is inside the raster");
    let partition_a = field_a
        .clone()
        .focus_partition(focus.clone())
        .expect("focus belongs to cue A field");
    let partition_b = field_b
        .clone()
        .focus_partition(focus.clone())
        .expect("focus belongs to cue B field");
    let partition_a_again = field_a_again
        .focus_partition(focus.clone())
        .expect("focus belongs to repeated cue field");
    let blank_partition = blank
        .focus_partition(focus)
        .expect("focus belongs to blank field");
    let actual_focused_region_shape = focused_region_shape(&partition_a, raster_focus);
    let actual_region_count = partition_a.regions().len();
    let exact_reassembly_a = partition_a.clone().reassemble() == field_a;
    let exact_reassembly_b = partition_b.clone().reassemble() == field_b;
    let focused_a = transduce(partition_a);
    let focused_b = transduce(partition_b);
    let focused_a_again = transduce(partition_a_again);
    let focused_blank = transduce(blank_partition);

    let empty = profile
        .focuses(field_a.shape(), [])
        .expect("empty focus is valid");
    let coarse_a = transduce(
        field_a
            .clone()
            .focus_partition(empty.clone())
            .expect("empty focus belongs to cue A field"),
    );
    let coarse_b = transduce(
        field_b
            .clone()
            .focus_partition(empty)
            .expect("empty focus belongs to cue B field"),
    );

    let mut grid_focus_count = 0;
    let mut grid_focuses_distinguishing_cues = 0;
    let mut maximum_grid_region_count = 0;
    for row_index in 0..=GRID_INTERVALS {
        for column_index in 0..=GRID_INTERVALS {
            let grid_focus = [
                grid_coordinate(field_a.shape()[0], row_index),
                grid_coordinate(field_a.shape()[1], column_index),
            ];
            let grid = profile
                .focuses(field_a.shape(), [grid_focus])
                .expect("regular grid focus is valid");
            let grid_a = transduce(
                field_a
                    .clone()
                    .focus_partition(grid.clone())
                    .expect("grid focus belongs to cue A field"),
            );
            let grid_b = transduce(
                field_b
                    .clone()
                    .focus_partition(grid)
                    .expect("grid focus belongs to cue B field"),
            );
            grid_focus_count += 1;
            grid_focuses_distinguishing_cues += usize::from(grid_a != grid_b);
            maximum_grid_region_count = maximum_grid_region_count.max(grid_a.regions().len());
        }
    }

    EyeProjection {
        eye,
        physical_gaze,
        raster_focus,
        actual_region_count,
        actual_focused_region_shape,
        exact_reassembly_a,
        exact_reassembly_b,
        same_cue_replay_equal: focused_a == focused_a_again,
        blank_control_differs: focused_a != focused_blank,
        actual_cue_pair_differs: focused_a != focused_b,
        actual_differing_region_count: differing_regions(&focused_a, &focused_b),
        no_focus_region_count: coarse_a.regions().len(),
        no_focus_cue_pair_differs: coarse_a != coarse_b,
        grid_focus_count,
        grid_focuses_distinguishing_cues,
        maximum_grid_region_count,
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

fn transduce(partition: FocusedPartition<u8, 2>) -> FocusedField<u64, 2> {
    partition.transduce_complete(0_u64, u64::from, u64::saturating_add)
}

fn scale_focus(position: i16, extent: usize) -> usize {
    usize::try_from(position)
        .unwrap_or(0)
        .saturating_mul(extent.saturating_sub(1))
        / usize::try_from(BODY_MAX).unwrap_or(1)
}

fn grid_coordinate(extent: usize, index: usize) -> usize {
    index.saturating_mul(extent.saturating_sub(1)) / GRID_INTERVALS
}

fn focused_region_shape(partition: &FocusedPartition<u8, 2>, focus: [usize; 2]) -> [usize; 2] {
    partition
        .regions()
        .iter()
        .find(|region| {
            let origin = region.origin();
            let shape = region.shape();
            (0..2).all(|axis| {
                focus[axis] >= origin[axis]
                    && focus[axis] < origin[axis].saturating_add(shape[axis])
            })
        })
        .expect("one focused region contains the physical focus")
        .shape()
}

fn differing_regions(left: &FocusedField<u64, 2>, right: &FocusedField<u64, 2>) -> usize {
    left.regions()
        .iter()
        .zip(right.regions())
        .filter(|(left, right)| left != right)
        .count()
}

fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
