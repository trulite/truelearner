#![forbid(unsafe_code)]

use academy_arc3::{Arc3AgentCommand, Arc3AgentResponse, Arc3Sensorimotor};
use std::io::{self, BufRead, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().collect::<Vec<_>>();
    let seed = arguments
        .get(1)
        .map(|value| value.parse::<u64>())
        .transpose()?
        .unwrap_or(205);
    let spatial = arguments.get(2).map(String::as_str) == Some("spatial");
    if let Some(argument) = arguments.get(3) {
        return Err(format!("unexpected argument {argument:?}").into());
    }
    let mut organism = if spatial {
        Arc3Sensorimotor::new_spatial(seed)?
    } else {
        Arc3Sensorimotor::new(seed)?
    };
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let command = serde_json::from_str::<Arc3AgentCommand>(&line);
        let response = match command {
            Ok(command) => match organism.handle(command) {
                Ok(Some(response)) => Some(response),
                Ok(None) => break,
                Err(error) => Some(Arc3AgentResponse::Error {
                    message: error.to_string(),
                }),
            },
            Err(error) => Some(Arc3AgentResponse::Error {
                message: format!("invalid agent command: {error}"),
            }),
        };
        if let Some(response) = response {
            serde_json::to_writer(&mut stdout, &response)?;
            stdout.write_all(b"\n")?;
            stdout.flush()?;
        }
    }
    Ok(())
}
