use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use organism_v0::reflected_program_discovery::{
    print_rp0b_report, rp0b_csv, rp0b_markdown, run_rp0b_experiment,
};

fn write_new(path: &Path, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap_or_else(|error| panic!("refuse to overwrite {}: {error}", path.display()));
    file.write_all(contents.as_bytes())
        .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

fn main() {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    assert!(arguments.len() <= 2, "expected [csv] [markdown]");
    let csv = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/rp0b_reflected_program_economics.csv"));
    let markdown = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/rp0b_reflected_program_economics.md"));
    assert!(!csv.exists(), "refuse to overwrite {}", csv.display());
    assert!(
        !markdown.exists(),
        "refuse to overwrite {}",
        markdown.display()
    );
    let report = run_rp0b_experiment();
    print_rp0b_report(&report);
    write_new(&csv, &rp0b_csv(&report));
    write_new(&markdown, &rp0b_markdown(&report));
    println!("wrote {} and {}", csv.display(), markdown.display());
}
