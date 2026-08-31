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
    pub final_body_fingerprint: String,
    pub body_checkpoint_file: String,
    pub body_checkpoint_sha256: String,
    pub workstation_pose_checkpoint_file: Option<String>,
    pub workstation_pose_checkpoint_sha256: Option<String>,
    pub workstation_entry_checkpoint_file: Option<String>,
    pub workstation_entry_checkpoint_sha256: Option<String>,
    pub workstation_body_checkpoint_file: Option<String>,
    pub workstation_body_checkpoint_sha256: Option<String>,
    pub workstation_evidence_state: Option<academy_workstation_course::ScreenDeviceEvidenceState>,
    pub workstation_automaticity: Option<academy_workstation_course::RepeatedUseEvidence>,
    pub workstation_first_failure: Option<academy_workstation_course::WorkstationFailure>,
    pub workstation_retention_verdict: Option<academy_workstation_course::WorkstationVerdict>,
    pub workstation_retention_ladder: Vec<academy_workstation_course::WorkstationVerdict>,
    pub physical_work: u64,
    pub plasticity_updates: u64,
    pub transcript_file: String,
    pub transcript_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidencePaths {
    pub receipt: PathBuf,
    pub transcript: PathBuf,
    pub body_checkpoint: PathBuf,
    pub workstation_pose_checkpoint: Option<PathBuf>,
    pub workstation_entry_checkpoint: Option<PathBuf>,
    pub workstation_body_checkpoint: Option<PathBuf>,
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
    let checkpoint_sha256 = hex(&Sha256::digest(&run.body_checkpoint));
    let checkpoint_file = format!("body-checkpoint-{checkpoint_sha256}.bin");
    let (pose_checkpoint_file, pose_checkpoint_sha256) = run
        .workstation_pose_checkpoint
        .as_ref()
        .map(|checkpoint| {
            let digest = hex(&Sha256::digest(checkpoint));
            (format!("workstation-pose-checkpoint-{digest}.bin"), digest)
        })
        .unzip();
    let (entry_checkpoint_file, entry_checkpoint_sha256) = run
        .workstation_entry_checkpoint
        .as_ref()
        .map(|checkpoint| {
            let digest = hex(&Sha256::digest(checkpoint));
            (format!("workstation-entry-checkpoint-{digest}.bin"), digest)
        })
        .unzip();
    let (workstation_checkpoint_file, workstation_checkpoint_sha256) = run
        .workstation_course
        .as_ref()
        .map(|course| {
            let digest = hex(&Sha256::digest(&course.body_checkpoint));
            (format!("workstation-body-checkpoint-{digest}.bin"), digest)
        })
        .unzip();
    let restored = truelearner_workstation::WorkstationHarness::restore(
        truelearner_workstation::WorkstationCheckpoint::decode(&run.body_checkpoint)?,
    )?;
    if restored.read()?.body_fingerprint != run.final_body_fingerprint {
        return Err(BodyCourseError::Serialization(
            "final body checkpoint fingerprint differs".to_string(),
        ));
    }
    if let Some(course) = &run.workstation_course {
        let restored = truelearner_workstation::WorkstationHarness::restore(
            truelearner_workstation::WorkstationCheckpoint::decode(&course.body_checkpoint)?,
        )?;
        if restored.read()?.body_fingerprint != course.final_body_fingerprint {
            return Err(BodyCourseError::Serialization(
                "workstation body checkpoint fingerprint differs".to_string(),
            ));
        }
    }
    let receipt = CourseReceipt {
        schema: "body-course/v13".to_string(),
        seed: run.seed,
        courses: run.courses.clone(),
        acquired: run.acquired.clone(),
        capability_evidence: run.capability_evidence.clone(),
        first_failure: run.first_failure,
        experience_count: run.experiences.len(),
        exact_replay: run.exact_replay,
        final_body_fingerprint: run.final_body_fingerprint.clone(),
        body_checkpoint_file: checkpoint_file.clone(),
        body_checkpoint_sha256: checkpoint_sha256,
        workstation_pose_checkpoint_file: pose_checkpoint_file.clone(),
        workstation_pose_checkpoint_sha256: pose_checkpoint_sha256,
        workstation_entry_checkpoint_file: entry_checkpoint_file.clone(),
        workstation_entry_checkpoint_sha256: entry_checkpoint_sha256,
        workstation_body_checkpoint_file: workstation_checkpoint_file.clone(),
        workstation_body_checkpoint_sha256: workstation_checkpoint_sha256,
        workstation_evidence_state: run
            .workstation_course
            .as_ref()
            .map(|course| course.evidence_state),
        workstation_automaticity: run
            .workstation_course
            .as_ref()
            .map(|course| course.automaticity.clone()),
        workstation_first_failure: run
            .workstation_course
            .as_ref()
            .and_then(|course| course.first_failure),
        workstation_retention_verdict: run
            .workstation_retention
            .as_ref()
            .map(|experience| experience.verdict),
        workstation_retention_ladder: run
            .workstation_retention_ladder
            .iter()
            .map(|experience| experience.verdict)
            .collect(),
        physical_work: total_physical_work(run),
        plasticity_updates: total_plasticity_updates(run),
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
    fs::write(temporary.join(&checkpoint_file), &run.body_checkpoint)
        .map_err(|error| BodyCourseError::Io(error.to_string()))?;
    if let (Some(file), Some(checkpoint)) = (
        pose_checkpoint_file.as_ref(),
        run.workstation_pose_checkpoint.as_ref(),
    ) {
        fs::write(temporary.join(file), checkpoint)
            .map_err(|error| BodyCourseError::Io(error.to_string()))?;
    }
    if let (Some(file), Some(course)) = (
        workstation_checkpoint_file.as_ref(),
        run.workstation_course.as_ref(),
    ) {
        fs::write(temporary.join(file), &course.body_checkpoint)
            .map_err(|error| BodyCourseError::Io(error.to_string()))?;
    }
    if let (Some(file), Some(checkpoint)) = (
        entry_checkpoint_file.as_ref(),
        run.workstation_entry_checkpoint.as_ref(),
    ) {
        fs::write(temporary.join(file), checkpoint)
            .map_err(|error| BodyCourseError::Io(error.to_string()))?;
    }
    fs::write(temporary.join("receipt.json"), receipt_bytes)
        .map_err(|error| BodyCourseError::Io(error.to_string()))?;
    fs::rename(&temporary, output).map_err(|error| BodyCourseError::Io(error.to_string()))?;
    Ok(EvidencePaths {
        receipt: output.join("receipt.json"),
        transcript: output.join(transcript_file),
        body_checkpoint: output.join(checkpoint_file),
        workstation_pose_checkpoint: pose_checkpoint_file.map(|file| output.join(file)),
        workstation_entry_checkpoint: entry_checkpoint_file.map(|file| output.join(file)),
        workstation_body_checkpoint: workstation_checkpoint_file.map(|file| output.join(file)),
    })
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn total_physical_work(run: &CourseRun) -> u64 {
    let body = run.experiences.iter().fold(0_u64, |sum, experience| {
        sum.saturating_add(experience.physical_work)
    });
    let workstation = run
        .workstation_course
        .as_ref()
        .into_iter()
        .flat_map(|course| &course.experiences)
        .fold(0_u64, |sum, experience| {
            sum.saturating_add(experience.physical_work)
        });
    let retention = run
        .workstation_retention
        .as_ref()
        .map_or(0, |experience| experience.physical_work);
    let ladder = run
        .workstation_retention_ladder
        .iter()
        .fold(0_u64, |sum, experience| {
            sum.saturating_add(experience.physical_work)
        });
    body.saturating_add(workstation)
        .saturating_add(ladder)
        .saturating_add(retention)
}

fn total_plasticity_updates(run: &CourseRun) -> u64 {
    let body = run.experiences.iter().fold(0_u64, |sum, experience| {
        sum.saturating_add(experience.plasticity_updates)
    });
    let workstation = run
        .workstation_course
        .as_ref()
        .into_iter()
        .flat_map(|course| &course.experiences)
        .fold(0_u64, |sum, experience| {
            sum.saturating_add(experience.plasticity_updates)
        });
    let retention = run
        .workstation_retention
        .as_ref()
        .map_or(0, |experience| experience.plasticity_updates);
    let ladder = run
        .workstation_retention_ladder
        .iter()
        .fold(0_u64, |sum, experience| {
            sum.saturating_add(experience.plasticity_updates)
        });
    body.saturating_add(workstation)
        .saturating_add(ladder)
        .saturating_add(retention)
}
