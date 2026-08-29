use std::path::PathBuf;
use workstation_return_bearing_opportunity_composition::{
    capture_aligned_contact, capture_effect_receptor_contact,
    capture_intermediate_transition_contact, capture_output_specific_opposition,
    capture_sequential_opposition, continue_intermediate_contact_trace,
};

fn main() {
    let mut output = None;
    let mut sequential = false;
    let mut aligned = false;
    let mut transition = false;
    let mut effect = false;
    let mut input = None;
    let mut max_steps = 120;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--output" => output = args.next().map(PathBuf::from),
            "--sequential" => sequential = true,
            "--aligned" => aligned = true,
            "--transition" => transition = true,
            "--effect" => effect = true,
            "--input" => input = args.next().map(PathBuf::from),
            "--max-steps" => {
                max_steps = args
                    .next()
                    .expect("--max-steps value is required")
                    .parse()
                    .expect("--max-steps is an integer")
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    let path = output.expect("--output path is required");
    let rendered = if effect {
        serde_json::to_vec_pretty(
            &capture_effect_receptor_contact().expect("trace capture succeeds"),
        )
    } else if let Some(input) = input {
        let encoded = std::fs::read(input).expect("input trace reads");
        serde_json::to_vec_pretty(
            &continue_intermediate_contact_trace(&encoded, max_steps)
                .expect("trace continuation succeeds"),
        )
    } else if transition {
        serde_json::to_vec_pretty(
            &capture_intermediate_transition_contact().expect("trace capture succeeds"),
        )
    } else if aligned {
        serde_json::to_vec_pretty(&capture_aligned_contact().expect("trace capture succeeds"))
    } else if sequential {
        serde_json::to_vec_pretty(&capture_sequential_opposition().expect("trace capture succeeds"))
    } else {
        serde_json::to_vec_pretty(
            &capture_output_specific_opposition().expect("trace capture succeeds"),
        )
    }
    .expect("trace serializes");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("artifact directory is created");
    }
    std::fs::write(path, rendered).expect("artifact writes");
}
