use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use organism_v0::reflected_program_discovery::grounding::{
    print_rg0a_report, rg0a_csv, rg0a_markdown, run_rg0a_harness,
};
use organism_v0::research_runtime::HarnessMode;

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
    let mode = match arguments.first().map(String::as_str) {
        Some("--micro") => {
            arguments.remove(0);
            HarnessMode::Micro
        }
        Some("--gate") => {
            arguments.remove(0);
            HarnessMode::Gate
        }
        Some("--definitive") => {
            arguments.remove(0);
            HarnessMode::Definitive
        }
        _ => panic!("expected --micro, --gate, or --definitive"),
    };
    if mode != HarnessMode::Definitive {
        assert!(
            arguments.is_empty(),
            "development modes never write result artifacts"
        );
        let report = run_rg0a_harness(mode);
        print_rg0a_report(&report);
        assert!(report.qualitative_passed, "development harness failed");
        return;
    }
    assert!(arguments.len() <= 2, "expected [csv] [markdown]");
    let csv = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/rg0a_reflected_grounding.csv"));
    let markdown = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/rg0a_reflected_grounding.md"));
    assert!(!csv.exists(), "refuse to overwrite {}", csv.display());
    assert!(
        !markdown.exists(),
        "refuse to overwrite {}",
        markdown.display()
    );
    let report = run_rg0a_harness(mode);
    print_rg0a_report(&report);
    write_new(&csv, &rg0a_csv(&report));
    write_new(&markdown, &rg0a_markdown(&report));
    println!("wrote {} and {}", csv.display(), markdown.display());
}
