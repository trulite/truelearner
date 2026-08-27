use sensorimotor_emergence::{run, Arm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let mut arm = None;
    let mut full = false;
    let mut output = None;
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--arm" => {
                index += 1;
                arm = Some(match arguments.get(index).map(String::as_str) {
                    Some("global-return") => Arm::GlobalReturn,
                    Some("causal-local-return") => Arm::CausalLocalReturn,
                    Some("shuffled-local-return") => Arm::ShuffledLocalReturn,
                    Some("causal-local-reference") => Arm::CausalLocalReference,
                    Some("local-return-deferral") => Arm::LocalReturnDeferral,
                    Some("local-return-replacement") => Arm::LocalReturnReplacement,
                    Some("shuffled-local-reference") => Arm::ShuffledLocalReference,
                    other => return Err(format!("unknown arm {other:?}").into()),
                });
            }
            "--suite" => {
                index += 1;
                full = arguments.get(index).map(String::as_str) == Some("full");
                if !full {
                    return Err("--suite requires full".into());
                }
            }
            "--output" => {
                index += 1;
                output = arguments.get(index).cloned();
            }
            other => return Err(format!("unexpected argument {other:?}").into()),
        }
        index += 1;
    }
    let result = run(arm.ok_or("--arm is required")?, full);
    let bytes = serde_json::to_vec_pretty(&result)?;
    if let Some(path) = output {
        std::fs::write(path, bytes)?;
    } else {
        println!("{}", String::from_utf8(bytes)?);
    }
    Ok(())
}
