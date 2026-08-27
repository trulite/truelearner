use sensorimotor_composition_sweep::{run, Arm};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let mut arm = None;
    let mut all = false;
    let mut output_dir = None;
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--arm" => {
                index += 1;
                arm = Some(
                    arguments
                        .get(index)
                        .ok_or("--arm requires an id")?
                        .parse()?,
                );
            }
            "--all" => all = true,
            "--output-dir" => {
                index += 1;
                output_dir = Some(PathBuf::from(
                    arguments.get(index).ok_or("--output-dir requires a path")?,
                ));
            }
            other => return Err(format!("unexpected argument {other:?}").into()),
        }
        index += 1;
    }
    if all == arm.is_some() {
        return Err("select exactly one of --all or --arm".into());
    }
    let arms = arm.map_or_else(|| Arm::ALL.to_vec(), |selected| vec![selected]);
    let results = arms.into_iter().map(run).collect::<Vec<_>>();
    if let Some(directory) = output_dir {
        std::fs::create_dir_all(&directory)?;
        for result in &results {
            std::fs::write(
                directory.join(format!("{}.json", result.arm)),
                serde_json::to_vec_pretty(result)?,
            )?;
        }
    } else if results.len() == 1 {
        println!("{}", serde_json::to_string_pretty(&results[0])?);
    } else {
        println!("{}", serde_json::to_string_pretty(&results)?);
    }
    Ok(())
}
