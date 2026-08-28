use hand_bounded_first_use_construction_continuation::run_all;
use std::path::PathBuf;

fn main() {
    let mut parent = None;
    let mut output_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--parent" => parent = args.next().map(PathBuf::from),
            "--output-dir" => output_dir = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    let parent = parent.expect("use --parent <immutable artifact>");
    let parent_bytes = std::fs::read(parent).expect("immutable parent reads");
    let output_dir = output_dir.expect("use --output-dir <directory>");
    std::fs::create_dir_all(&output_dir).expect("output directory is created");
    for (arm, result) in run_all(&parent_bytes) {
        let rendered = serde_json::to_string_pretty(&result).expect("result serializes");
        std::fs::write(output_dir.join(format!("{}.json", arm.id())), rendered)
            .expect("artifact writes");
    }
}
