use hand_construction_projection_admission_retention::{Arm, run_all};
use std::path::PathBuf;

fn main() {
    let mut arm = None;
    let mut parent = None;
    let mut output_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--arm" => arm = args.next(),
            "--all" => {}
            "--parent" => parent = args.next().map(PathBuf::from),
            "--output-dir" => output_dir = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    let parent = parent.expect("use --parent <immutable artifact>");
    let parent_bytes = std::fs::read(parent).expect("immutable parent artifact reads");
    let mut results = run_all(&parent_bytes);
    if let Some(selected) = arm {
        let selected = selected.parse::<Arm>().expect("unknown arm");
        results.retain(|(arm, _)| *arm == selected);
    }
    if let Some(directory) = &output_dir {
        std::fs::create_dir_all(directory).expect("output directory is created");
    }
    for (selected, result) in results {
        let rendered = serde_json::to_string_pretty(&result).expect("result serializes");
        if let Some(directory) = &output_dir {
            std::fs::write(directory.join(format!("{}.json", selected.id())), &rendered)
                .expect("artifact writes");
        } else {
            println!("{rendered}");
        }
    }
}
