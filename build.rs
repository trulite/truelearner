use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn composition_copy(source: &Path, destination: &Path) {
    let text = fs::read_to_string(source).expect("frozen composition source is readable");
    let mut body_started = false;
    let mut output = String::new();
    for line in text.split_inclusive('\n') {
        if !body_started
            && (line.trim().is_empty()
                || line.starts_with("//!")
                || line.starts_with("#![allow(dead_code)]"))
        {
            continue;
        }
        body_started = true;
        output.push_str(line);
    }
    fs::write(destination, output).expect("composition include copy is writable");
    println!("cargo:rerun-if-changed={}", source.display());
}

fn main() {
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo supplies OUT_DIR"));
    let bin_output = output.join("bin");
    fs::create_dir_all(&bin_output).expect("generated bin directory is writable");
    composition_copy(
        Path::new("src/ds_e0_anonymous_event_formation.rs"),
        &output.join("ds_e0_anonymous_event_formation.rs"),
    );
    composition_copy(
        Path::new("src/ds_a0_anonymous_boundary_action_formation.rs"),
        &output.join("ds_a0_anonymous_boundary_action_formation.rs"),
    );
    fs::copy(
        "src/bin/ds_a0_anonymous_boundary_action_formation.rs",
        bin_output.join("ds_a0_anonymous_boundary_action_formation.rs"),
    )
    .expect("frozen A0 runner audit copy is writable");
    println!("cargo:rerun-if-changed=src/bin/ds_a0_anonymous_boundary_action_formation.rs");
}
