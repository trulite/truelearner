#![allow(dead_code)]

#[path = "../ssa0_3_precommit_support_definitive.rs"]
mod authority;

use authority::{csv, markdown, run_definitive, source_preflight};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;

const CSV_PATH: &str = "results/ssa0_3_precommit_support_definitive_v1.csv";
const MD_PATH: &str = "results/ssa0_3_precommit_support_definitive_v1.md";
const CSV_STAGE: &str = "results/.ssa0_3_precommit_support_definitive_v1.csv.staging";
const MD_STAGE: &str = "results/.ssa0_3_precommit_support_definitive_v1.md.staging";

fn outputs_absent() -> bool {
    !Path::new(CSV_PATH).exists() && !Path::new(MD_PATH).exists()
}

fn staging_absent() -> bool {
    !Path::new(CSV_STAGE).exists() && !Path::new(MD_STAGE).exists()
}

fn stage_create_new(path: &Path, contents: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()
}

fn sync_parent(path: &Path) -> io::Result<()> {
    File::open(path.parent().unwrap_or_else(|| Path::new(".")))?.sync_all()
}

fn publish_staged(stage: &Path, final_path: &Path) -> io::Result<()> {
    fs::hard_link(stage, final_path)
}

fn publish_pair_write_once(csv_text: &str, markdown_text: &str) -> io::Result<()> {
    let csv_stage = Path::new(CSV_STAGE);
    let md_stage = Path::new(MD_STAGE);
    let csv_path = Path::new(CSV_PATH);
    let md_path = Path::new(MD_PATH);
    stage_create_new(csv_stage, csv_text)?;
    stage_create_new(md_stage, markdown_text)?;
    sync_parent(csv_stage)?;
    publish_staged(csv_stage, csv_path)?;
    publish_staged(md_stage, md_path)?;
    sync_parent(csv_path)?;
    fs::remove_file(csv_stage)?;
    fs::remove_file(md_stage)?;
    sync_parent(csv_path)
}

fn main() {
    let arguments = std::env::args().collect::<Vec<_>>();
    match arguments.as_slice() {
        [_, mode] if mode == "--audit" => {
            let audit = source_preflight(outputs_absent(), staging_absent());
            println!("{audit:#?}");
            if !audit.passed {
                std::process::exit(2);
            }
        }
        [_, mode] if mode == "--definitive" => {
            if !outputs_absent() || !staging_absent() {
                eprintln!("refusing definitive run: a fixed final or staging path exists");
                std::process::exit(2);
            }
            let preflight = source_preflight(true, true);
            if !preflight.passed {
                eprintln!("refusing definitive run: zero-cell preflight failed");
                std::process::exit(2);
            }
            let report = run_definitive(preflight);
            let csv_text = csv(&report);
            let markdown_text = markdown(&report);
            publish_pair_write_once(&csv_text, &markdown_text)
                .expect("fixed definitive artifacts must publish once without replacement");
            print!("{markdown_text}");
            if !report.passed {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("usage: ssa0_3_precommit_support_definitive [--audit|--definitive]");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_paths() -> (PathBuf, PathBuf, PathBuf) {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time follows epoch")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "ssa0-3-definitive-publication-refusal-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&directory).expect("bounded fresh test directory is created once");
        let stage = directory.join("result.staging");
        let final_path = directory.join("result.csv");
        (directory, stage, final_path)
    }

    #[test]
    fn atomic_publication_is_create_new_and_refuses_replacement() {
        let (directory, stage, final_path) = temporary_paths();
        stage_create_new(&stage, "first\n").expect("first create-new stage succeeds");
        publish_staged(&stage, &final_path).expect("first atomic publication succeeds");
        fs::remove_file(&stage).expect("published staging link is removable");
        stage_create_new(&stage, "second\n").expect("second create-new stage succeeds");
        let error = publish_staged(&stage, &final_path).expect_err("replacement is refused");
        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(fs::read_to_string(&final_path).unwrap(), "first\n");
        fs::remove_file(&stage).expect("bounded staging file is removed");
        fs::remove_file(&final_path).expect("bounded final file is removed");
        fs::remove_dir(directory).expect("empty bounded directory is removed");
    }
}
