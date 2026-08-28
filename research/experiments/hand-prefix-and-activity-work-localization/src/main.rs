use hand_prefix_and_activity_work_localization::{PredecessorBytes, run_all};
use std::path::{Path, PathBuf};

fn read(path: &Path, name: &str) -> Vec<u8> {
    std::fs::read(path.join(name)).unwrap_or_else(|error| panic!("{name} reads: {error}"))
}

fn main() {
    let mut predecessor_dir = None;
    let mut output_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--predecessor-dir" => predecessor_dir = args.next().map(PathBuf::from),
            "--output-dir" => output_dir = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    let predecessor_dir = predecessor_dir.expect("use --predecessor-dir <campaign directory>");
    let output_dir = output_dir.expect("use --output-dir <directory>");
    let fixtures = read(
        &predecessor_dir,
        "artifacts/bounded-first-use-fixtures.json",
    );
    let first_wall = read(&predecessor_dir, "artifacts/reflected-hand-first-wall.json");
    let controls = read(
        &predecessor_dir,
        "artifacts/parent-and-lifetime-controls.json",
    );
    let evidence = read(&predecessor_dir, "evidence.json");
    let adjudication = read(&predecessor_dir, "adjudication.toml");
    let convergence = read(&predecessor_dir, "convergence.toml");
    let predecessor = PredecessorBytes {
        fixtures: &fixtures,
        first_wall: &first_wall,
        controls: &controls,
        evidence: &evidence,
        adjudication: &adjudication,
        convergence: &convergence,
    };

    std::fs::create_dir_all(&output_dir).expect("output directory is created");
    for (arm, result) in run_all(&predecessor) {
        let rendered = serde_json::to_string_pretty(&result).expect("result serializes");
        std::fs::write(output_dir.join(format!("{}.json", arm.id())), rendered)
            .expect("artifact writes");
    }
}
