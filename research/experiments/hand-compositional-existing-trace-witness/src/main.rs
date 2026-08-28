use hand_compositional_existing_trace_witness::{analyze_source, analyze_source_all};
use std::path::PathBuf;

fn main() {
    let mut source = None;
    let mut output = None;
    let mut output_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--source" => source = args.next().map(PathBuf::from),
            "--output" => output = args.next().map(PathBuf::from),
            "--output-dir" => output_dir = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    let source = source.expect("use --source <artifact>");
    if let Some(directory) = output_dir {
        std::fs::create_dir_all(&directory).expect("output directory is created");
        for result in analyze_source_all(&source) {
            let rendered = serde_json::to_string_pretty(&result).expect("result serializes");
            std::fs::write(directory.join(format!("{}.json", result.arm)), rendered)
                .expect("artifact writes");
        }
        return;
    }
    let result = analyze_source(&source);
    let rendered = serde_json::to_string_pretty(&result).expect("result serializes");
    if let Some(path) = output {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("output parent is created");
        }
        std::fs::write(path, rendered).expect("artifact writes");
    } else {
        println!("{rendered}");
    }
}
