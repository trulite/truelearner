use developmental_hand_multi_joint::{PredecessorBytes, run_all_with_protocol};
use std::path::{Path, PathBuf};
use truelearner_core::Protocol;

fn read(path: &Path, name: &str) -> Vec<u8> {
    std::fs::read(path.join(name)).unwrap_or_else(|error| panic!("{name} reads: {error}"))
}

fn main() {
    let mut predecessor_dir = None;
    let mut output_dir = None;
    let mut protocol = Protocol::RecursiveLearnerReturnBearingContinuation;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--predecessor-dir" => predecessor_dir = args.next().map(PathBuf::from),
            "--output-dir" => output_dir = args.next().map(PathBuf::from),
            "--protocol" => {
                protocol = match args.next().as_deref() {
                    Some("return-bearing") => Protocol::RecursiveLearnerReturnBearingContinuation,
                    Some("immediate-origin-product") => {
                        Protocol::RecursiveLearnerCausalOriginProductComposition
                    }
                    Some("causal-path-product") => {
                        Protocol::RecursiveLearnerCausalPathProductComposition
                    }
                    Some("causal-topology-output-only") => {
                        Protocol::RecursiveLearnerCausalTopologyOutputComposition
                    }
                    Some("causal-topology-opportunity-only") => {
                        Protocol::RecursiveLearnerCausalTopologyOpportunityComposition
                    }
                    Some("causal-topology-product") => {
                        Protocol::RecursiveLearnerCausalTopologyProductComposition
                    }
                    Some(other) => panic!("unknown protocol: {other}"),
                    None => panic!("--protocol requires a value"),
                }
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    let predecessor_dir = predecessor_dir.expect("use --predecessor-dir <campaign directory>");
    let output_dir = output_dir.expect("use --output-dir <directory>");
    let complete = read(&predecessor_dir, "artifacts/complete-one-joint.json");
    let convergence = read(&predecessor_dir, "convergence.toml");
    let predecessor = PredecessorBytes {
        complete_one_joint: &complete,
        convergence: &convergence,
    };
    std::fs::create_dir_all(&output_dir).expect("output directory is created");
    for (arm, result) in run_all_with_protocol(&predecessor, protocol) {
        let rendered = serde_json::to_string_pretty(&result).expect("result serializes");
        std::fs::write(output_dir.join(format!("{}.json", arm.id())), rendered)
            .expect("artifact writes");
    }
}
