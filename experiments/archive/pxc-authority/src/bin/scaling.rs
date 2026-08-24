use std::env;
use std::fs;
use std::path::PathBuf;

use organism_v0::scaling;

fn main() {
    let mut output = None;
    let mut arguments = env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--output" => {
                output = arguments.next().map(PathBuf::from);
            }
            "--help" | "-h" => {
                println!("usage: scaling [--output PATH]");
                return;
            }
            _ => {
                eprintln!("unknown argument: {argument}");
                std::process::exit(2);
            }
        }
    }

    let report = scaling::run_experiment();
    println!("{}", report.summary());
    if let Some(path) = output {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("failed to create output directory");
        }
        fs::write(&path, report.to_csv()).expect("failed to write scaling CSV");
        println!("wrote {}", path.display());
    }
    if !report.passed {
        std::process::exit(1);
    }
}
