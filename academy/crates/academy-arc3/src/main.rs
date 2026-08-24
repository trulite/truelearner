#![forbid(unsafe_code)]

use academy_arc3::Arc3Recording;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: academy-arc3-ingest RECORDING.jsonl [FINAL.png]")?;
    let recording = Arc3Recording::read_jsonl(&source)?;
    let destination = std::env::args_os().nth(2).map(PathBuf::from);
    if let Some(destination) = destination {
        std::fs::write(destination, recording.final_surface(8)?.png_bytes()?)?;
    }
    println!(
        "ARC3_RECORDING_OK game={} turns={} actions={:?}",
        recording.metadata.game_id,
        recording.observations.len(),
        recording.metadata.available_actions
    );
    Ok(())
}
