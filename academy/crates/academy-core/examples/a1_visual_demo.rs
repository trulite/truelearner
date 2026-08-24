use academy_core::{A1ProbeFamily, GenuineTeachingLab, TeachingCase};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(error) = run() {
        eprintln!("A1 visual demo failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut arguments = std::env::args().skip(1);
    let output = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("output/a1-visual-demo"));
    let name = arguments.next().unwrap_or_else(|| "Momo".to_string());
    fs::create_dir_all(&output).map_err(|error| error.to_string())?;

    let mut lab = GenuineTeachingLab::new(TeachingCase::named_creature(205, &name))
        .map_err(|error| error.to_string())?;
    let teaching = lab.teach_supported().map_err(|error| error.to_string())?;
    write_surface(
        &output.join("01-teaching-world.png"),
        &teaching
            .shared_world_surface
            .png_bytes()
            .map_err(|error| error.to_string())?,
    )?;
    write_surface(
        &output.join("02-teaching-response.png"),
        &teaching
            .organism_surface
            .png_bytes()
            .map_err(|error| error.to_string())?,
    )?;

    let probe = lab
        .probe(A1ProbeFamily::LearnedRelation)
        .map_err(|error| error.to_string())?;
    write_surface(
        &output.join("03-fresh-probe-world.png"),
        &probe
            .shared_world_surface
            .png_bytes()
            .map_err(|error| error.to_string())?,
    )?;
    write_surface(
        &output.join("04-learned-response.png"),
        &probe
            .organism_surface
            .png_bytes()
            .map_err(|error| error.to_string())?,
    )?;

    let replay = lab.replay(&probe.id).map_err(|error| error.to_string())?;
    println!(
        "A1_VISUAL_DEMO_OK name={} teaching_updates={} probe_crossings={} probe_updates={} replay_exact={} output={}",
        name,
        teaching.observation.plasticity_updates,
        probe.observation.outward_relation_crossings,
        probe.observation.plasticity_updates,
        replay.exact,
        output.display(),
    );
    Ok(())
}

fn write_surface(path: &Path, bytes: &[u8]) -> Result<(), String> {
    fs::write(path, bytes).map_err(|error| error.to_string())
}
