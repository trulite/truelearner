use hand_construction_outcome_composition::{Arm, run, run_all};
use std::path::PathBuf;

fn main() {
    let mut arm = None;
    let mut output_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--arm" => arm = args.next(),
            "--all" => {}
            "--output-dir" => output_dir = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    let results = arm.map_or_else(run_all, |selected| {
        let selected = selected.parse::<Arm>().expect("unknown arm");
        vec![(selected, run(selected))]
    });
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
