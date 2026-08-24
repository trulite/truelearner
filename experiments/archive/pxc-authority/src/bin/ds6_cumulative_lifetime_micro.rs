use organism_v0::ds6_cumulative_lifetime_probe::{render_micro, run_micro};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let report = run_micro();
    let rendered = render_micro(&report);
    let path = "results/ds6_cumulative_lifetime_micro.md";
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("MICRO result path must not already exist");
    file.write_all(rendered.as_bytes())
        .expect("MICRO result is written atomically before exit");
    print!("{rendered}");
    if !report.passed {
        std::process::exit(1);
    }
}
