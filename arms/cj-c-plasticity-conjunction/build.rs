use std::env;
use std::fs;
use std::path::PathBuf;

const HEADER: &str = "#![forbid(unsafe_code)]\n//! Experimental substrate-native CELL/ARROW/SPIKE physics for PX0.\n//!\n//! Active state contains only cells, arrows, spikes, and local physical\n//! timing. The module contains no evaluator types and has no dependency on the\n//! historical mechanism suite.\n";

fn main() {
    let source_path = "../../crates/px0-physical-correspondence/src/lib.rs";
    println!("cargo:rerun-if-changed={source_path}");
    let source = fs::read_to_string(source_path).expect("read frozen authority source");
    let body = source
        .strip_prefix(HEADER)
        .expect("frozen authority header must remain exact");
    let destination = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("authority.rs");
    fs::write(destination, body).expect("write mechanically included authority source");
}
