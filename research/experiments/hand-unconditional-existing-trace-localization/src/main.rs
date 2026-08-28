use hand_unconditional_existing_trace_localization::run_all;
use std::path::PathBuf;

fn main() {
    let mut output_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--output-dir" => output_dir = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    let output_dir = output_dir.expect("use --output-dir <directory>");
    std::fs::create_dir_all(&output_dir).expect("output directory is created");
    for (arm, result) in run_all() {
        let rendered = serde_json::to_string_pretty(&result).expect("result serializes");
        std::fs::write(output_dir.join(format!("{}.json", arm.id())), rendered)
            .expect("artifact writes");
    }
}
