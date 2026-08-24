use organism_v0::ds6_cumulative_lifetime_definitive::{csv, markdown, run};
use std::fs::OpenOptions;
use std::io::Write;

fn create_new(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("definitive result path must not already exist");
    file.write_all(contents.as_bytes())
        .expect("definitive result is written before exit");
}

fn main() {
    let report = run();
    let csv = csv(&report);
    let markdown = markdown(&report);
    create_new("results/ds6_cumulative_lifetime_definitive.csv", &csv);
    create_new("results/ds6_cumulative_lifetime_definitive.md", &markdown);
    print!("{markdown}");
    if !report.passed {
        std::process::exit(1);
    }
}
