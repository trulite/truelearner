use organism_v0::ds6_cumulative_lifetime_probe::{render_gate, run_gate};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let report = run_gate();
    let rendered = render_gate(&report);
    let path = "results/ds6_cumulative_lifetime_gate.md";
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("DS6 GATE result path must not already exist");
    file.write_all(rendered.as_bytes())
        .expect("DS6 GATE result is written before exit");
    print!("{rendered}");
    if !report.passed {
        std::process::exit(1);
    }
}
