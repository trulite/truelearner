use std::path::PathBuf;
use workstation_digit_separation::{Arm, run};

fn main() {
    let mut arm = None;
    let mut output = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--arm" => arm = args.next(),
            "--output" => output = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    let arm = arm
        .as_deref()
        .expect("use --arm <id>")
        .parse::<Arm>()
        .expect("unknown arm");
    let rendered = serde_json::to_string_pretty(&run(arm)).expect("result serializes");
    if let Some(path) = output {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("artifact directory is created");
        }
        std::fs::write(path, rendered).expect("artifact writes");
    } else {
        println!("{rendered}");
    }
}
