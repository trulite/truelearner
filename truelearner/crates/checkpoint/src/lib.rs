#![forbid(unsafe_code)]
//! In-memory checkpoints taken across a quiet cut in body activity.

use truelearner_body::{Body, PhysicalEvent, Run, RunError};

/// A private, process-local copy of a naturally quiet body.
#[derive(Clone, Debug)]
pub struct Checkpoint {
    body: Body,
}

impl Checkpoint {
    /// Produces an independent continuation from the captured quiet body.
    pub fn restore(&self) -> Body {
        self.body.clone()
    }

    /// Consumes the checkpoint when only one continuation is needed.
    pub fn into_body(self) -> Body {
        self.body
    }
}

/// The checkpoint and the physical work required to reach its quiet cut.
#[derive(Clone, Debug)]
pub struct Capture {
    pub checkpoint: Checkpoint,
    pub drain: Run,
}

/// Holds exclusive body access, completes existing activity, then copies the
/// quiet body. The borrow is the input gate: safe callers cannot supply another
/// input until this function returns.
pub fn capture(
    body: &mut Body,
    moment_limit: usize,
    observe: impl FnMut(PhysicalEvent),
) -> Result<Capture, RunError> {
    let drain = body.run(moment_limit, observe)?;
    debug_assert!(body.is_quiet());
    Ok(Capture {
        checkpoint: Checkpoint { body: body.clone() },
        drain,
    })
}
