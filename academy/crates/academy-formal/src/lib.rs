#![forbid(unsafe_code)]
//! Causally inert invocation of the pinned Lean closure checker.

mod trace_projection;

pub use trace_projection::{
    project_ambiguous_return, project_closed_boundary_return, project_closed_return,
    ClosureProjection, TraceProjectionError,
};

use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub const REQUEST_SCHEMA: &str = "truelearner-causal-check/v1";
pub const RECEIPT_SCHEMA: &str = "truelearner-causal-receipt/v1";

const MAX_EVENTS: usize = 4_096;
const MAX_WITNESSES: usize = 1_024;
const MAX_PARENTS_PER_EVENT: usize = 64;
const MAX_SUPPORT_PER_WITNESS: usize = 256;
const MAX_REQUEST_BYTES: usize = 1_048_576;
const MAX_RECEIPT_BYTES: usize = 1_048_576;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CausalEvent {
    pub id: u64,
    pub time: u64,
    pub parents: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosureWitness {
    pub id: u64,
    pub crossing: u64,
    pub support: Vec<u64>,
    pub opened_at: u64,
    pub expires_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CausalClaim {
    pub resolution: String,
    pub witness: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CausalCheckRequest {
    pub schema: String,
    pub events: Vec<CausalEvent>,
    pub witnesses: Vec<ClosureWitness>,
    pub returned: u64,
    pub claim: CausalClaim,
}

impl CausalCheckRequest {
    pub fn new(
        events: Vec<CausalEvent>,
        witnesses: Vec<ClosureWitness>,
        returned: u64,
        claim: CausalClaim,
    ) -> Self {
        Self {
            schema: REQUEST_SCHEMA.to_string(),
            events,
            witnesses,
            returned,
            claim,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CausalCheckReceipt {
    pub schema: String,
    pub accepted: bool,
    pub resolution: String,
    pub witness: Option<u64>,
    pub explanations: Vec<u64>,
    pub persistent_links: Vec<u64>,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeanChecker {
    executable: PathBuf,
}

impl LeanChecker {
    pub fn new(executable: impl Into<PathBuf>) -> Self {
        Self {
            executable: executable.into(),
        }
    }

    pub fn executable(&self) -> &Path {
        &self.executable
    }

    /// Checks already-frozen observer evidence in a separate process.
    ///
    /// The checker receives no harness or organism handle, and its result is
    /// never returned to the learner.
    pub fn check(
        &self,
        request: &CausalCheckRequest,
    ) -> Result<CausalCheckReceipt, FormalCheckError> {
        validate_bounds(request)?;
        let request = serde_json::to_vec(request)
            .map_err(|error| FormalCheckError::Serialization(error.to_string()))?;
        if request.len() > MAX_REQUEST_BYTES {
            return Err(FormalCheckError::RequestTooLarge);
        }

        let mut child = Command::new(&self.executable)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| FormalCheckError::Spawn(error.to_string()))?;
        child
            .stdin
            .take()
            .ok_or(FormalCheckError::MissingStdin)?
            .write_all(&request)
            .map_err(|error| FormalCheckError::Io(error.to_string()))?;
        let output = child
            .wait_with_output()
            .map_err(|error| FormalCheckError::Io(error.to_string()))?;
        if !output.status.success() {
            return Err(FormalCheckError::CheckerFailed {
                status: output.status.code(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }
        if output.stdout.len() > MAX_RECEIPT_BYTES {
            return Err(FormalCheckError::ReceiptTooLarge);
        }
        let receipt: CausalCheckReceipt = serde_json::from_slice(&output.stdout)
            .map_err(|error| FormalCheckError::InvalidReceipt(error.to_string()))?;
        if receipt.schema != RECEIPT_SCHEMA {
            return Err(FormalCheckError::WrongReceiptSchema(receipt.schema));
        }
        Ok(receipt)
    }
}

fn validate_bounds(request: &CausalCheckRequest) -> Result<(), FormalCheckError> {
    if request.events.len() > MAX_EVENTS
        || request
            .events
            .iter()
            .any(|event| event.parents.len() > MAX_PARENTS_PER_EVENT)
        || request.witnesses.len() > MAX_WITNESSES
        || request
            .witnesses
            .iter()
            .any(|witness| witness.support.len() > MAX_SUPPORT_PER_WITNESS)
    {
        return Err(FormalCheckError::RequestTooLarge);
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormalCheckError {
    RequestTooLarge,
    ReceiptTooLarge,
    Serialization(String),
    Spawn(String),
    MissingStdin,
    Io(String),
    CheckerFailed { status: Option<i32>, stderr: String },
    InvalidReceipt(String),
    WrongReceiptSchema(String),
}

impl fmt::Display for FormalCheckError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestTooLarge => formatter.write_str("causal check request exceeds its bound"),
            Self::ReceiptTooLarge => formatter.write_str("Lean receipt exceeds its bound"),
            Self::Serialization(message) => {
                write!(formatter, "cannot encode causal request: {message}")
            }
            Self::Spawn(message) => write!(formatter, "cannot start Lean checker: {message}"),
            Self::MissingStdin => formatter.write_str("Lean checker stdin is unavailable"),
            Self::Io(message) => write!(formatter, "Lean checker I/O failed: {message}"),
            Self::CheckerFailed { status, stderr } => {
                write!(formatter, "Lean checker exited with {status:?}: {stderr}")
            }
            Self::InvalidReceipt(message) => write!(formatter, "invalid Lean receipt: {message}"),
            Self::WrongReceiptSchema(schema) => {
                write!(formatter, "unsupported Lean receipt schema: {schema}")
            }
        }
    }
}

impl std::error::Error for FormalCheckError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_uses_the_pinned_wire_schema() {
        let request = CausalCheckRequest::new(
            vec![CausalEvent {
                id: 1,
                time: 10,
                parents: Vec::new(),
            }],
            vec![ClosureWitness {
                id: 7,
                crossing: 1,
                support: vec![11, 12],
                opened_at: 10,
                expires_at: 20,
            }],
            1,
            CausalClaim {
                resolution: "no_claim".to_string(),
                witness: None,
            },
        );

        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["schema"], REQUEST_SCHEMA);
        assert_eq!(value["witnesses"][0]["openedAt"], 10);
        assert_eq!(value["witnesses"][0]["expiresAt"], 20);
    }

    #[test]
    fn oversized_evidence_is_rejected_before_process_start() {
        let request = CausalCheckRequest::new(
            (0..=MAX_EVENTS)
                .map(|id| CausalEvent {
                    id: id as u64,
                    time: id as u64,
                    parents: Vec::new(),
                })
                .collect(),
            Vec::new(),
            0,
            CausalClaim {
                resolution: "no_claim".to_string(),
                witness: None,
            },
        );

        assert_eq!(
            LeanChecker::new("does-not-exist").check(&request),
            Err(FormalCheckError::RequestTooLarge)
        );
    }
}
