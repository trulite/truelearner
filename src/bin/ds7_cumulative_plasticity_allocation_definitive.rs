use organism_v0::ds7_cumulative_plasticity_allocation_definitive::{
    csv, markdown, run_definitive, source_preflight,
};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

const CSV_PATH: &str = "results/ds7_cumulative_plasticity_allocation_definitive.csv";
const MD_PATH: &str = "results/ds7_cumulative_plasticity_allocation_definitive.md";

fn outputs_absent() -> bool {
    !Path::new(CSV_PATH).exists() && !Path::new(MD_PATH).exists()
}

fn create_new(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("definitive result path must not already exist");
    file.write_all(contents.as_bytes())
        .expect("definitive result must be fully written before process exit");
    file.sync_all()
        .expect("definitive result must be durable before process exit");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, mode] if mode == "--audit" => {
            let audit = source_preflight(outputs_absent());
            println!("{audit:#?}");
            if !audit.passed {
                std::process::exit(2);
            }
        }
        [_, mode] if mode == "--definitive" => {
            if !outputs_absent() {
                eprintln!("refusing definitive run: a write-once result path exists");
                std::process::exit(2);
            }
            let report = run_definitive(true);
            let csv = csv(&report);
            let markdown = markdown(&report);
            create_new(CSV_PATH, &csv);
            create_new(MD_PATH, &markdown);
            print!("{markdown}");
            if !report.passed {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!(
                "usage: ds7_cumulative_plasticity_allocation_definitive [--audit|--definitive]"
            );
            std::process::exit(2);
        }
    }
}
