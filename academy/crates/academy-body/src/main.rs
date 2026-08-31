use academy_body::{write_course_evidence, BodyCourse};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let mut seed = 31_001_u64;
    let mut output = None;
    let mut with_workstation = false;
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
            "--output" => {
                index += 1;
                output = Some(PathBuf::from(
                    arguments.get(index).ok_or("--output requires a value")?,
                ));
            }
            "--with-workstation" => with_workstation = true,
            other => return Err(format!("unexpected argument {other:?}").into()),
        }
        index += 1;
    }
    let output = output.ok_or("--output is required")?;
    let course = BodyCourse::new(seed)?;
    let run = if with_workstation {
        course.run_with_workstation_course()?
    } else {
        course.run()?
    };
    let paths = write_course_evidence(&output, &run)?;
    let pose_checkpoint = paths
        .workstation_pose_checkpoint
        .as_ref()
        .map_or_else(|| "none".to_string(), |path| path.display().to_string());
    let entry_checkpoint = paths
        .workstation_entry_checkpoint
        .as_ref()
        .map_or_else(|| "none".to_string(), |path| path.display().to_string());
    let workstation_checkpoint = paths
        .workstation_body_checkpoint
        .as_ref()
        .map_or_else(|| "none".to_string(), |path| path.display().to_string());
    println!(
        "BODY_COURSE_OK acquired={} first_failure={:?} exact_replay={} receipt={} checkpoint={} pose_checkpoint={} entry_checkpoint={} workstation_checkpoint={}",
        run.acquired.len(),
        run.first_failure,
        run.exact_replay,
        paths.receipt.display(),
        paths.body_checkpoint.display(),
        pose_checkpoint,
        entry_checkpoint,
        workstation_checkpoint
    );
    Ok(())
}
