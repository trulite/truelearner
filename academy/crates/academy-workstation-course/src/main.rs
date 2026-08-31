use academy_workstation_course::{write_workstation_evidence, WorkstationCourse};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let mut seed = 41_001_u64;
    let mut body_checkpoint = None;
    let mut pose_checkpoint = None;
    let mut output = None;
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--seed" => {
                index += 1;
                seed = arguments
                    .get(index)
                    .ok_or("--seed requires a value")?
                    .parse()?;
            }
            "--body-checkpoint" => {
                index += 1;
                body_checkpoint = Some(PathBuf::from(
                    arguments
                        .get(index)
                        .ok_or("--body-checkpoint requires a value")?,
                ));
            }
            "--pose-checkpoint" => {
                index += 1;
                pose_checkpoint = Some(PathBuf::from(
                    arguments
                        .get(index)
                        .ok_or("--pose-checkpoint requires a value")?,
                ));
            }
            "--output" => {
                index += 1;
                output = Some(PathBuf::from(
                    arguments.get(index).ok_or("--output requires a value")?,
                ));
            }
            other => return Err(format!("unexpected argument {other:?}").into()),
        }
        index += 1;
    }
    let body_checkpoint = body_checkpoint.ok_or("--body-checkpoint is required")?;
    let pose_checkpoint = pose_checkpoint.ok_or("--pose-checkpoint is required")?;
    let output = output.ok_or("--output is required")?;
    let body = fs::read(body_checkpoint)?;
    let pose = fs::read(pose_checkpoint)?;
    let run = WorkstationCourse::restore(seed, &body, &pose)?.run()?;
    let paths = write_workstation_evidence(&output, &run)?;
    println!(
        "WORKSTATION_COURSE_COMPLETE state={:?} first_failure={:?} exact_replay={} receipt={} checkpoint={}",
        run.evidence_state,
        run.first_failure,
        run.exact_replay,
        paths.receipt.display(),
        paths.body_checkpoint.display()
    );
    Ok(())
}
