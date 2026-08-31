#![forbid(unsafe_code)]

use academy_arc3::{Arc3CapstoneAgent, Arc3CapstoneCommand, Arc3CapstoneResponse};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

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
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 || arguments[0] != "--body-checkpoint" {
        return Err("usage: academy-arc3-capstone-agent --body-checkpoint PATH".into());
    }
    let checkpoint_path = PathBuf::from(&arguments[1]);
    let checkpoint = std::fs::read(&checkpoint_path).map_err(|error| {
        format!(
            "could not read body checkpoint {}: {error}",
            checkpoint_path.display()
        )
    })?;

    let mut agent = Arc3CapstoneAgent::restore(&checkpoint)?;
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
