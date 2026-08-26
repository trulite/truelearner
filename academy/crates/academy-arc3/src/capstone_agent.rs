#![forbid(unsafe_code)]

use academy_arc3::{Arc3CapstoneAgent, Arc3CapstoneCommand, Arc3CapstoneResponse};
use std::io::{self, BufRead, Write};

fn write_response(
    stdout: &mut impl Write,
    response: &Arc3CapstoneResponse,
) -> Result<(), Box<dyn std::error::Error>> {
    serde_json::to_writer(&mut *stdout, response)?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().collect::<Vec<_>>();
    let seed = arguments
        .get(1)
        .map(|value| value.parse::<u64>())
        .transpose()?
        .unwrap_or(205);
    if let Some(argument) = arguments.get(2) {
        return Err(format!("unexpected argument {argument:?}").into());
    }

    let mut agent = Arc3CapstoneAgent::new(seed)?;
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    write_response(&mut stdout, &Arc3CapstoneResponse::Ready(agent.snapshot()?))?;

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let response = match serde_json::from_str::<Arc3CapstoneCommand>(&line) {
            Ok(command) => match agent.handle(command) {
                Ok(Some(response)) => response,
                Ok(None) => break,
                Err(error) => Arc3CapstoneResponse::Error {
                    message: error.to_string(),
                },
            },
            Err(error) => Arc3CapstoneResponse::Error {
                message: format!("invalid capstone command: {error}"),
            },
        };
        write_response(&mut stdout, &response)?;
    }
    Ok(())
}
