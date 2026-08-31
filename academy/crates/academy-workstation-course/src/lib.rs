#![forbid(unsafe_code)]
//! Generic workstation development and frozen capability evidence.

mod course;
mod evidence;

pub use course::{
    RepeatedUseEvidence, RepeatedUseEvidenceState, ScreenDeviceEvidenceState, WorkstationCourse,
    WorkstationCourseError, WorkstationCourseRun, WorkstationExperience, WorkstationExperienceMode,
    WorkstationFailure, WorkstationStep, WorkstationVerdict,
};
pub use evidence::{write_workstation_evidence, WorkstationEvidencePaths, WorkstationReceipt};
