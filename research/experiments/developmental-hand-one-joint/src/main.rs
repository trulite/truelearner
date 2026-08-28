use developmental_hand_one_joint::Arm;
use std::path::PathBuf;

fn main() {
    let mut arm = None;
    let mut all = false;
    let mut output_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--arm" => arm = args.next(),
            "--all" => all = true,
            "--output-dir" => output_dir = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    if let Some(directory) = &output_dir {
        std::fs::create_dir_all(directory).expect("output directory is created");
    }
    let results = if all {
        developmental_hand_one_joint::run_all()
    } else {
        let selected = arm
            .as_deref()
            .expect("use --arm <id> or --all")
            .parse::<Arm>()
            .expect("unknown arm");
        vec![(selected, developmental_hand_one_joint::run(selected))]
    };
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
