use crate::{
    BodyCapability, BodyCapabilityEvidence, BodyCourseError, BodyCourseProgress, CourseRun,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CourseReceipt {
    pub schema: String,
    pub seed: u64,
    pub courses: Vec<BodyCourseProgress>,
    pub acquired: Vec<BodyCapability>,
    pub capability_evidence: Vec<BodyCapabilityEvidence>,
    pub first_failure: Option<BodyCapability>,
    pub experience_count: usize,
    pub exact_replay: bool,
    pub physical_work: u64,
    pub plasticity_updates: u64,
    pub transcript_file: String,
    pub transcript_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidencePaths {
    pub receipt: PathBuf,
    pub transcript: PathBuf,
}

pub fn write_course_evidence(
    output: &Path,
    run: &CourseRun,
) -> Result<EvidencePaths, BodyCourseError> {
    if output.exists() {
        return Err(BodyCourseError::OutputExists(output.display().to_string()));
    }
    let transcript = serde_json::to_vec_pretty(run)
        .map_err(|error| BodyCourseError::Serialization(error.to_string()))?;
    let transcript_sha256 = hex(&Sha256::digest(&transcript));
    let transcript_file = format!("transcript-{transcript_sha256}.json");
    let receipt = CourseReceipt {
        schema: "body-course/v9".to_string(),
        seed: run.seed,
        courses: run.courses.clone(),
        acquired: run.acquired.clone(),
        capability_evidence: run.capability_evidence.clone(),
        first_failure: run.first_failure,
        experience_count: run.experiences.len(),
        exact_replay: run.exact_replay,
        physical_work: run.experiences.iter().fold(0, |sum, experience| {
            sum.saturating_add(experience.physical_work)
        }),
        plasticity_updates: run.experiences.iter().fold(0, |sum, experience| {
            sum.saturating_add(experience.plasticity_updates)
        }),
        transcript_file: transcript_file.clone(),
        transcript_sha256,
    };
    let receipt_bytes = serde_json::to_vec_pretty(&receipt)
        .map_err(|error| BodyCourseError::Serialization(error.to_string()))?;
    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|error| BodyCourseError::Io(error.to_string()))?;
    let name = output
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("body-course");
    let temporary = parent.join(format!(".{name}-{}", std::process::id()));
    if temporary.exists() {
        fs::remove_dir_all(&temporary).map_err(|error| BodyCourseError::Io(error.to_string()))?;
    }
    fs::create_dir(&temporary).map_err(|error| BodyCourseError::Io(error.to_string()))?;
    fs::write(temporary.join(&transcript_file), &transcript)
        .map_err(|error| BodyCourseError::Io(error.to_string()))?;
    fs::write(temporary.join("receipt.json"), receipt_bytes)
        .map_err(|error| BodyCourseError::Io(error.to_string()))?;
    fs::rename(&temporary, output).map_err(|error| BodyCourseError::Io(error.to_string()))?;
    Ok(EvidencePaths {
        receipt: output.join("receipt.json"),
        transcript: output.join(transcript_file),
    })
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
