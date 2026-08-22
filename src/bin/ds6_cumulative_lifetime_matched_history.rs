use organism_v0::ds6_cumulative_lifetime_probe::{render_matched, run_matched};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let report = run_matched();
    let rendered = render_matched(&report);
    let path = "results/ds6_cumulative_lifetime_matched_history.md";
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("matched-history result path must not already exist");
    file.write_all(rendered.as_bytes())
        .expect("matched-history result is written before exit");
    print!("{rendered}");
    if !report.passed {
        std::process::exit(1);
    }
}
