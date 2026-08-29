use academy_workstation::{WorkstationPresentation, WorkstationWorld};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use truelearner_embodiment::{Availability, LocalBlock, LocalBlockShape, SpatialField};
use truelearner_workstation::{Eye, LightField, WorkstationState};

const BLOCK_SHAPE: [usize; 2] = [8, 8];

#[derive(Serialize)]
struct Projection {
    schema: &'static str,
    source_trace: PathBuf,
    source_sha256: String,
    cue_a: char,
    cue_b: char,
    gazes: [[i16; 2]; 2],
    field_shape: [usize; 2],
    block_shape: [usize; 2],
    eyes: [EyeProjection; 2],
}

#[derive(Serialize)]
struct EyeProjection {
    eye: Eye,
    image_a_sha256: String,
    image_b_sha256: String,
    images_differ: bool,
    exact_reassembly_a: bool,
    exact_reassembly_b: bool,
    same_cue_replay_equal: bool,
    blank_control_differs: bool,
    block_count: usize,
    differing_block_count: usize,
    first_differing_block_origin: Option<[usize; 2]>,
    first_differing_block_a_sha256: Option<String>,
    first_differing_block_b_sha256: Option<String>,
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let source_trace = PathBuf::from(
        args.next()
            .expect("usage: monitor_cue_local_field_projection TRACE OUTPUT"),
    );
    let output = PathBuf::from(
        args.next()
            .expect("usage: monitor_cue_local_field_projection TRACE OUTPUT"),
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
    let eyes = Eye::ALL.map(|eye| {
        project_eye(
            eye,
            sample_a.eye(eye),
            sample_b.eye(eye),
            sample_a_again.eye(eye),
            blank.eye(eye),
        )
    });

    let projection = Projection {
        schema: "workstation-monitor-cue-local-field-projection/v1",
        source_trace: source_trace.clone(),
        source_sha256: hex_digest(&source),
        cue_a,
        cue_b,
        gazes: Eye::ALL.map(|eye| {
            let gaze = state.eye(eye).gaze();
            [gaze.x(), gaze.y()]
        }),
        field_shape: [
            usize::from(sample_a.eye(Eye::Left).height()),
            usize::from(sample_a.eye(Eye::Left).width()),
        ],
        block_shape: BLOCK_SHAPE,
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
    image_a: &LightField,
    image_b: &LightField,
    image_a_again: &LightField,
    blank: &LightField,
) -> EyeProjection {
    let field_a = field(image_a);
    let field_b = field(image_b);
    let field_a_again = field(image_a_again);
    let blank = field(blank);
    let shape = LocalBlockShape::new(BLOCK_SHAPE).expect("block shape is non-zero");
    let blocks_a = field_a.clone().factor_local(shape);
    let blocks_b = field_b.clone().factor_local(shape);
    let blocks_a_again = field_a_again.factor_local(shape);
    let blank_blocks = blank.factor_local(shape);
    let differing = blocks_a
        .blocks()
        .iter()
        .zip(blocks_b.blocks())
        .filter(|(left, right)| left != right)
        .collect::<Vec<_>>();
    let first = differing.first().copied();

    EyeProjection {
        eye,
        image_a_sha256: hex_digest(image_a.pixels()),
        image_b_sha256: hex_digest(image_b.pixels()),
        images_differ: image_a != image_b,
        exact_reassembly_a: blocks_a.clone().reassemble() == field_a,
        exact_reassembly_b: blocks_b.clone().reassemble() == field_b,
        same_cue_replay_equal: blocks_a == blocks_a_again,
        blank_control_differs: blocks_a != blank_blocks,
        block_count: blocks_a.blocks().len(),
        differing_block_count: differing.len(),
        first_differing_block_origin: first.map(|(left, _)| left.origin()),
        first_differing_block_a_sha256: first.map(|(left, _)| block_digest(left)),
        first_differing_block_b_sha256: first.map(|(_, right)| block_digest(right)),
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

fn block_digest(block: &LocalBlock<u8, 2>) -> String {
    let values = block
        .cells()
        .iter()
        .map(|cell| match cell {
            Availability::Available(value) => *value,
            Availability::Unavailable => 0,
        })
        .collect::<Vec<_>>();
    hex_digest(&values)
}

fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
