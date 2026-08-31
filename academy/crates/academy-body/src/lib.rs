#![forbid(unsafe_code)]
//! Headless development and evidence for visual-touch body discovery.

mod course;
mod evidence;
mod world;

pub use course::{
    BodyCapability, BodyCourse, BodyCourseError, BodyCourseKind, BodyCourseOutcome,
    BodyCourseProgress, BodyExperience, BodyExperienceMode, BodyVerdict, BodyWorldCause,
    BodyWorldEvent, BodyWorldObservation, CourseRun,
};
pub use evidence::{write_course_evidence, CourseReceipt, EvidencePaths};
