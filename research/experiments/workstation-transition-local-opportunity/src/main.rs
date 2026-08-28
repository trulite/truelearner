use std::path::PathBuf;
use workstation_transition_local_opportunity::run;

fn main() {
    let mut output = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--output" => output = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    let rendered = serde_json::to_string_pretty(&run().expect("candidate run succeeds"))
        .expect("candidate evidence serializes");
    if let Some(path) = output {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("artifact directory is created");
        }
        std::fs::write(path, rendered).expect("artifact writes");
    } else {
        println!("{rendered}");
    }
}
