use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use organism_v0::reflected_program_discovery::{
    print_rp0a_report, rp0a_csv, rp0a_markdown, run_rp0a_experiment,
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
    let mut arguments = env::args().skip(1).collect::<Vec<_>>();
    let smoke = arguments
        .first()
        .is_some_and(|argument| argument == "--smoke");
    if smoke {
        arguments.remove(0);
    }
    assert!(arguments.len() <= 2, "expected [--smoke] [csv] [markdown]");
    let csv = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/rp0a_reflected_program_discovery.csv"));
    let markdown = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/rp0a_reflected_program_discovery.md"));
    assert!(!csv.exists(), "refuse to overwrite {}", csv.display());
    assert!(
        !markdown.exists(),
        "refuse to overwrite {}",
        markdown.display()
    );
    let report = run_rp0a_experiment(smoke);
    print_rp0a_report(&report);
    write_new(&csv, &rp0a_csv(&report));
    write_new(&markdown, &rp0a_markdown(&report));
    println!("wrote {} and {}", csv.display(), markdown.display());
}
