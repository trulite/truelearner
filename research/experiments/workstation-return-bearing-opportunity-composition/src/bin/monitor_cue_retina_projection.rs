use academy_workstation::{WorkstationPresentation, WorkstationWorld};
use serde::Serialize;
use std::path::PathBuf;
use truelearner_workstation::{BODY_MAX, Eye, Point, WorkstationState, WorldSample};

const WORKSTATION_RETINA_OFFSETS: [(i16, i16); 12] = [
    (0, 0),
    (-512, -768),
    (-384, -768),
    (-256, -768),
    (-128, -768),
    (0, -768),
    (-384, -384),
    (-128, -384),
    (128, -384),
    (384, -384),
    (-160, 128),
    (160, 128),
];

#[derive(Serialize)]
struct Projection {
    source_trace: PathBuf,
    cue_a: char,
    cue_b: char,
    gazes: [[i16; 2]; 2],
    image_differs: [bool; 2],
    retina_a: Vec<u8>,
    retina_b: Vec<u8>,
    retina_differs: bool,
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let source_trace = PathBuf::from(
        args.next()
            .expect("usage: monitor_cue_retina_projection TRACE OUTPUT"),
    );
    let output = PathBuf::from(
        args.next()
            .expect("usage: monitor_cue_retina_projection TRACE OUTPUT"),
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
    let sample_a =
        WorkstationWorld::new_with_presentation(WorkstationPresentation::with_monitor_glyph(cue_a))
            .expect("cue A world builds")
            .sense(&state)
            .expect("cue A senses");
    let sample_b =
        WorkstationWorld::new_with_presentation(WorkstationPresentation::with_monitor_glyph(cue_b))
            .expect("cue B world builds")
            .sense(&state)
            .expect("cue B senses");
    let retina_a = workstation_retinal_features(&sample_a, &state);
    let retina_b = workstation_retinal_features(&sample_b, &state);
    let projection = Projection {
        source_trace: source_trace.clone(),
        cue_a,
        cue_b,
        gazes: Eye::ALL.map(|eye| {
            let gaze = state.eye(eye).gaze();
            [gaze.x(), gaze.y()]
        }),
        image_differs: Eye::ALL.map(|eye| sample_a.eye(eye) != sample_b.eye(eye)),
        retina_differs: retina_a != retina_b,
        retina_a,
        retina_b,
    };
    let encoded = serde_json::to_vec_pretty(&projection).expect("projection serializes");
    std::fs::write(output, encoded).expect("projection writes");
}

fn workstation_retinal_features(sample: &WorldSample, state: &WorkstationState) -> Vec<u8> {
    Eye::ALL
        .into_iter()
        .flat_map(|eye| {
            let gaze = state.eye(eye).gaze();
            WORKSTATION_RETINA_OFFSETS.into_iter().map(move |(dx, dy)| {
                let point = Point::new(
                    gaze.x().saturating_add(dx).clamp(0, BODY_MAX),
                    gaze.y().saturating_add(dy).clamp(0, BODY_MAX),
                )
                .expect("clamped retinal point is valid");
                sample.eye(eye).sample(point)
            })
        })
        .collect()
}
