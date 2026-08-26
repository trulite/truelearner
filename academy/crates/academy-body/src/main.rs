use academy_body::{write_course_evidence, BodyCourse};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let mut seed = 31_001_u64;
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
    let output = output.ok_or("--output is required")?;
    let run = BodyCourse::new(seed)?.run()?;
    let paths = write_course_evidence(&output, &run)?;
    println!(
        "BODY_COURSE_OK acquired={} first_failure={:?} exact_replay={} receipt={}",
        run.acquired.len(),
        run.first_failure,
        run.exact_replay,
        paths.receipt.display()
    );
    Ok(())
}
