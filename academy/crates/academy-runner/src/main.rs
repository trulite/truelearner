#![forbid(unsafe_code)]

use academy_episodes::generate_a1_gallery;
use std::path::PathBuf;

fn main() {
    let destination = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("output/academy-episodes"));
    match generate_a1_gallery(&destination) {
        Ok(catalog) => println!(
            "ACADEMY_EPISODES_READY count={} destination={}",
            catalog.episodes.len(),
            destination.display()
        ),
        Err(error) => {
            eprintln!("Academy episode run failed: {error}");
            std::process::exit(1);
        }
    }
}
