#![forbid(unsafe_code)]

use academy_workstation_review::record_and_render;
use std::path::PathBuf;

fn main() {
    match arguments().and_then(|arguments| {
        record_and_render(&arguments.destination, arguments.seed, arguments.steps)
            .map_err(|error| error.to_string())
    }) {
        Ok(manifest) => println!(
            "WORKSTATION_REVIEW_READY steps={} replay_exact={} video={}",
            manifest.step_count, manifest.replay_exact, manifest.video_file
        ),
        Err(error) => {
            eprintln!("Workstation recording failed: {error}");
            std::process::exit(1);
        }
    }
}

struct Arguments {
    destination: PathBuf,
    seed: u64,
    steps: usize,
}

fn arguments() -> Result<Arguments, String> {
    let mut values = std::env::args().skip(1);
    let destination = values.next().map(PathBuf::from).ok_or_else(usage)?;
    let mut seed = 82_001;
    let mut steps = 48;
    while let Some(flag) = values.next() {
        let value = values.next().ok_or_else(usage)?;
        match flag.as_str() {
            "--seed" => seed = value.parse().map_err(|_| usage())?,
            "--steps" => steps = value.parse().map_err(|_| usage())?,
            _ => return Err(usage()),
        }
    }
    Ok(Arguments {
        destination,
        seed,
        steps,
    })
}

fn usage() -> String {
    "usage: academy-workstation-record DESTINATION [--steps N] [--seed N]".to_string()
}
