use crate::{WorkstationCourseError, WorkstationCourseRun, WorkstationFailure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationReceipt {
    pub schema: String,
    pub seed: u64,
    pub capability: String,
    pub evidence_state: crate::ScreenDeviceEvidenceState,
    pub first_failure: Option<WorkstationFailure>,
    pub experience_count: usize,
    pub exact_replay: bool,
    pub input_body_checkpoint_sha256: String,
    pub workstation_pose_checkpoint_sha256: String,
    pub final_body_fingerprint: String,
    pub body_checkpoint_file: String,
    pub body_checkpoint_sha256: String,
    pub physical_work: u64,
    pub plasticity_updates: u64,
    pub transcript_file: String,
    pub transcript_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkstationEvidencePaths {
    pub receipt: PathBuf,
    pub transcript: PathBuf,
    pub body_checkpoint: PathBuf,
}

pub fn write_workstation_evidence(
    output: &Path,
    run: &WorkstationCourseRun,
) -> Result<WorkstationEvidencePaths, WorkstationCourseError> {
    if output.exists() {
        return Err(WorkstationCourseError::OutputExists(
            output.display().to_string(),
        ));
    }
    let transcript = serde_json::to_vec_pretty(run)
        .map_err(|error| WorkstationCourseError::Serialization(error.to_string()))?;
    let transcript_sha256 = hex(&Sha256::digest(&transcript));
    let transcript_file = format!("transcript-{transcript_sha256}.json");
    let checkpoint_sha256 = hex(&Sha256::digest(&run.body_checkpoint));
    let checkpoint_file = format!("body-checkpoint-{checkpoint_sha256}.bin");
    let restored = truelearner_workstation::WorkstationHarness::restore(
        truelearner_workstation::WorkstationCheckpoint::decode(&run.body_checkpoint)?,
    )?;
    if restored.read()?.body_fingerprint != run.final_body_fingerprint {
        return Err(WorkstationCourseError::InvalidEvidence(
            "final body checkpoint fingerprint differs".to_string(),
        ));
    }
    let receipt = WorkstationReceipt {
        schema: "workstation-course/v1".to_string(),
        seed: run.seed,
        capability: run.capability.clone(),
        evidence_state: run.evidence_state,
        first_failure: run.first_failure,
        experience_count: run.experiences.len(),
        exact_replay: run.exact_replay,
        input_body_checkpoint_sha256: run.input_body_checkpoint_sha256.clone(),
        workstation_pose_checkpoint_sha256: run.workstation_pose_checkpoint_sha256.clone(),
        final_body_fingerprint: run.final_body_fingerprint.clone(),
        body_checkpoint_file: checkpoint_file.clone(),
        body_checkpoint_sha256: checkpoint_sha256,
        physical_work: run.experiences.iter().fold(0_u64, |sum, experience| {
            sum.saturating_add(experience.physical_work)
        }),
        plasticity_updates: run.experiences.iter().fold(0_u64, |sum, experience| {
            sum.saturating_add(experience.plasticity_updates)
        }),
        transcript_file: transcript_file.clone(),
        transcript_sha256,
    };
    let receipt_bytes = serde_json::to_vec_pretty(&receipt)
        .map_err(|error| WorkstationCourseError::Serialization(error.to_string()))?;
    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|error| WorkstationCourseError::Io(error.to_string()))?;
    let name = output
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("workstation-course");
    let temporary = parent.join(format!(".{name}-{}", std::process::id()));
    if temporary.exists() {
        fs::remove_dir_all(&temporary)
            .map_err(|error| WorkstationCourseError::Io(error.to_string()))?;
    }
    fs::create_dir(&temporary).map_err(|error| WorkstationCourseError::Io(error.to_string()))?;
    fs::write(temporary.join(&transcript_file), &transcript)
        .map_err(|error| WorkstationCourseError::Io(error.to_string()))?;
    fs::write(temporary.join(&checkpoint_file), &run.body_checkpoint)
        .map_err(|error| WorkstationCourseError::Io(error.to_string()))?;
    fs::write(temporary.join("receipt.json"), receipt_bytes)
        .map_err(|error| WorkstationCourseError::Io(error.to_string()))?;
    fs::rename(&temporary, output)
        .map_err(|error| WorkstationCourseError::Io(error.to_string()))?;
    Ok(WorkstationEvidencePaths {
        receipt: output.join("receipt.json"),
        transcript: output.join(transcript_file),
        body_checkpoint: output.join(checkpoint_file),
    })
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
