use organism_v0::ds8_cumulative_semantic_credit_definitive::{
    csv, markdown, run_definitive, source_preflight,
};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;

const CSV_PATH: &str = "results/ds8_cumulative_semantic_credit_definitive.csv";
const MD_PATH: &str = "results/ds8_cumulative_semantic_credit_definitive.md";
const CSV_STAGE: &str = "results/.ds8_cumulative_semantic_credit_definitive.csv.staging";
const MD_STAGE: &str = "results/.ds8_cumulative_semantic_credit_definitive.md.staging";

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
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    File::open(parent)?.sync_all()
}

fn publish_staged(stage: &Path, final_path: &Path) -> io::Result<()> {
    fs::hard_link(stage, final_path)
}

fn publish_pair_atomic_write_once(csv_text: &str, markdown_text: &str) -> io::Result<()> {
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
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, mode] if mode == "--audit" => {
            let audit = source_preflight(outputs_absent(), staging_absent());
            println!("{audit:#?}");
            if !audit.passed {
                std::process::exit(2);
            }
        }
        [_, mode] if mode == "--definitive" => {
            if !outputs_absent() || !staging_absent() {
                eprintln!("refusing definitive run: a write-once final or staging path exists");
                std::process::exit(2);
            }
            let report = run_definitive(true, true);
            let csv_text = csv(&report);
            let markdown_text = markdown(&report);
            publish_pair_atomic_write_once(&csv_text, &markdown_text)
                .expect("definitive artifacts must publish atomically without replacement");
            print!("{markdown_text}");
            if !report.passed {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("usage: ds8_cumulative_semantic_credit_definitive [--audit|--definitive]");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temporary_paths() -> (PathBuf, PathBuf, PathBuf) {
        let directory = std::env::temp_dir().join(format!(
            "ds8-definitive-publication-refusal-{}",
            std::process::id()
        ));
        fs::create_dir(&directory).expect("fresh test directory is created once");
        let stage = directory.join("result.staging");
        let final_path = directory.join("result.csv");
        (directory, stage, final_path)
    }

    #[test]
    fn atomic_publication_is_create_new_and_refuses_replacement() {
        let (directory, stage, final_path) = temporary_paths();
        stage_create_new(&stage, "first\n").expect("first staging create succeeds");
        publish_staged(&stage, &final_path).expect("first atomic publish succeeds");
        fs::remove_file(&stage).expect("published staging link is removable");
        stage_create_new(&stage, "second\n").expect("second staging create succeeds");
        let error = publish_staged(&stage, &final_path).expect_err("replacement is refused");
        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(fs::read_to_string(&final_path).unwrap(), "first\n");
        fs::remove_dir_all(directory).expect("bounded temporary test directory is removable");
    }
}
