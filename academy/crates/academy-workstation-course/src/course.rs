use academy_workstation::{
    DeviceEvent, DeviceState, KeyId, MonitorFrame, SessionObservation, WorkstationPresentation,
    WorkstationSession, KEY_COUNT, KEY_PRESS_DEPTH, KEY_RELEASE_DEPTH,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
#[cfg(test)]
use truelearner_workstation::MotorEffect;
use truelearner_workstation::{
    BodyControl, Direction, WorkstationCheckpoint, WorkstationHarness, WorkstationStepObservation,
};

const PRACTICE_PRESS_DEPTH: i16 = 640;
const PRACTICE_RELEASE_DEPTH: i16 = 608;
const DEMONSTRATION_STEPS: usize = 5;
const EXPERIENCE_STEPS: usize = 16;
const PASSIVE_CHANGE_STEP: usize = 2;
// One palm-horizontal impulse moves this morphology by 16 coordinate units.
const TRANSFER_IMPULSE: i16 = 1;
const PHYSICAL_WORK_BOUND: u64 = 2_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkstationExperienceMode {
    Demonstration,
    ImitationControl,
    PassiveDevelopment,
    PassiveProbe,
    ActionOnlyDevelopment,
    ActionOnlyProbe,
    Development,
    NormalDepthProbe,
    Probe,
    Transfer,
    FreshBodyControl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkstationVerdict {
    Presented,
    Passed,
    Failed,
    BudgetExceeded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScreenDeviceEvidenceState {
    Unknown,
    Emerging,
    Acquired,
    General,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkstationFailure {
    Demonstration,
    PassiveControl,
    ActionOnlyControl,
    Development,
    NormalDepthProbe,
    Probe,
    Transfer,
    FreshBodyControl,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationStep {
    pub sequence: u64,
    pub application_frame_sha256: String,
    pub sample_sha256: String,
    pub body: WorkstationStepObservation,
    pub device_events: Vec<DeviceEvent>,
    pub device_after: DeviceState,
    pub world_fingerprint: String,
    pub session_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationExperience {
    pub id: String,
    pub mode: WorkstationExperienceMode,
    pub seed: u64,
    pub key_press_depth: i16,
    pub key_release_depth: i16,
    pub external_pose_impulse: i16,
    pub steps: Vec<WorkstationStep>,
    pub organism_key_events: usize,
    pub external_key_events: usize,
    pub screen_changes: usize,
    pub unique_returned_screen_changes: usize,
    pub checkpoint_before_sha256: String,
    pub checkpoint_after_sha256: String,
    pub replay_exact: bool,
    pub mutation_discarded: bool,
    pub verdict: WorkstationVerdict,
    pub physical_work: u64,
    pub plasticity_updates: u64,
    pub naturally_quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationCourseRun {
    pub schema_version: u16,
    pub seed: u64,
    pub capability: String,
    pub evidence_state: ScreenDeviceEvidenceState,
    pub first_failure: Option<WorkstationFailure>,
    pub experiences: Vec<WorkstationExperience>,
    pub exact_replay: bool,
    pub input_body_checkpoint_sha256: String,
    pub workstation_pose_checkpoint_sha256: String,
    pub final_body_fingerprint: String,
    #[serde(skip)]
    pub body_checkpoint: Vec<u8>,
}

pub struct WorkstationCourse {
    seed: u64,
    body_checkpoint: Vec<u8>,
    pose_checkpoint: Vec<u8>,
}

impl WorkstationCourse {
    pub fn restore(
        seed: u64,
        body_checkpoint: &[u8],
        pose_checkpoint: &[u8],
    ) -> Result<Self, WorkstationCourseError> {
        let body = WorkstationCheckpoint::decode(body_checkpoint)?;
        let pose = WorkstationCheckpoint::decode(pose_checkpoint)?;
        WorkstationHarness::restore(body)?;
        WorkstationHarness::restore(pose)?;
        Ok(Self {
            seed,
            body_checkpoint: body_checkpoint.to_vec(),
            pose_checkpoint: pose_checkpoint.to_vec(),
        })
    }

    pub fn run(self) -> Result<WorkstationCourseRun, WorkstationCourseError> {
        let initial = self.body_checkpoint.clone();
        let mut experiences = Vec::new();

        let (demonstration, demonstrated) = self.experience(
            &initial,
            WorkstationExperienceMode::Demonstration,
            self.seed.saturating_add(1),
            true,
        )?;
        experiences.push(demonstration.clone());
        let (imitation, _) = self.experience(
            &demonstrated,
            WorkstationExperienceMode::ImitationControl,
            self.seed.saturating_add(2),
            false,
        )?;
        experiences.push(imitation.clone());

        let (passive_development, passive_body) = self.experience(
            &initial,
            WorkstationExperienceMode::PassiveDevelopment,
            self.seed.saturating_add(3),
            true,
        )?;
        experiences.push(passive_development);
        let (passive_probe, _) = self.experience(
            &passive_body,
            WorkstationExperienceMode::PassiveProbe,
            self.seed.saturating_add(4),
            false,
        )?;
        experiences.push(passive_probe.clone());

        let (action_development, action_body) = self.experience(
            &initial,
            WorkstationExperienceMode::ActionOnlyDevelopment,
            self.seed.saturating_add(5),
            true,
        )?;
        experiences.push(action_development.clone());
        let (action_probe, _) = self.experience(
            &action_body,
            WorkstationExperienceMode::ActionOnlyProbe,
            self.seed.saturating_add(6),
            false,
        )?;
        experiences.push(action_probe.clone());

        let (development, developed) = if imitation.verdict == WorkstationVerdict::Passed {
            (None, demonstrated)
        } else {
            let (development, developed) = self.experience(
                &initial,
                WorkstationExperienceMode::Development,
                self.seed.saturating_add(7),
                true,
            )?;
            experiences.push(development.clone());
            (Some(development), developed)
        };
        let (normal_depth_probe, _) = self.experience(
            &developed,
            WorkstationExperienceMode::NormalDepthProbe,
            self.seed.saturating_add(7),
            false,
        )?;
        experiences.push(normal_depth_probe.clone());
        let (probe, _) = self.experience(
            &developed,
            WorkstationExperienceMode::Probe,
            self.seed.saturating_add(8),
            false,
        )?;
        experiences.push(probe.clone());
        let (transfer, _) = self.experience(
            &developed,
            WorkstationExperienceMode::Transfer,
            self.seed.saturating_add(9),
            false,
        )?;
        experiences.push(transfer.clone());
        let (fresh, _) = self.experience(
            &initial,
            WorkstationExperienceMode::FreshBodyControl,
            self.seed.saturating_add(10),
            false,
        )?;
        experiences.push(fresh.clone());

        let demonstration_ok = demonstration.verdict == WorkstationVerdict::Presented;
        let passive_control_ok = passive_probe.verdict == WorkstationVerdict::Failed;
        let action_control_ok = action_development.verdict == WorkstationVerdict::Presented
            && action_probe.verdict == WorkstationVerdict::Failed;
        let development_ok = imitation.verdict == WorkstationVerdict::Passed
            || development
                .as_ref()
                .is_some_and(|experience| experience.verdict == WorkstationVerdict::Passed);
        let normal_depth_ok = normal_depth_probe.verdict == WorkstationVerdict::Passed;
        let probe_ok = probe.verdict == WorkstationVerdict::Passed;
        let transfer_ok = transfer.verdict == WorkstationVerdict::Passed;
        let fresh_control_ok = fresh.verdict == WorkstationVerdict::Failed;
        let first_failure = if !demonstration_ok {
            Some(WorkstationFailure::Demonstration)
        } else if !passive_control_ok {
            Some(WorkstationFailure::PassiveControl)
        } else if !action_control_ok {
            Some(WorkstationFailure::ActionOnlyControl)
        } else if !development_ok {
            Some(WorkstationFailure::Development)
        } else if !normal_depth_ok {
            Some(WorkstationFailure::NormalDepthProbe)
        } else if !probe_ok {
            Some(WorkstationFailure::Probe)
        } else if !transfer_ok {
            Some(WorkstationFailure::Transfer)
        } else if !fresh_control_ok {
            Some(WorkstationFailure::FreshBodyControl)
        } else {
            None
        };
        let evidence_state = if !development_ok || !normal_depth_ok {
            ScreenDeviceEvidenceState::Unknown
        } else if !probe_ok {
            ScreenDeviceEvidenceState::Emerging
        } else if !transfer_ok || !passive_control_ok || !action_control_ok || !fresh_control_ok {
            ScreenDeviceEvidenceState::Acquired
        } else {
            ScreenDeviceEvidenceState::General
        };
        let final_checkpoint = positioned_checkpoint(&developed, &self.pose_checkpoint, 0)?;
        let final_harness =
            WorkstationHarness::restore(WorkstationCheckpoint::decode(&final_checkpoint)?)?;
        let final_body_fingerprint = final_harness.read()?.body_fingerprint;
        Ok(WorkstationCourseRun {
            schema_version: 1,
            seed: self.seed,
            capability: "screen_device_screen_contingency".to_string(),
            evidence_state,
            first_failure,
            exact_replay: experiences.iter().all(|experience| experience.replay_exact),
            experiences,
            input_body_checkpoint_sha256: digest(&initial),
            workstation_pose_checkpoint_sha256: digest(&self.pose_checkpoint),
            final_body_fingerprint,
            body_checkpoint: final_checkpoint,
        })
    }

    pub fn retention_probe(
        seed: u64,
        body_checkpoint: &[u8],
        pose_checkpoint: &[u8],
    ) -> Result<WorkstationExperience, WorkstationCourseError> {
        let course = Self::restore(seed, body_checkpoint, pose_checkpoint)?;
        let (experience, _) = course.experience(
            body_checkpoint,
            WorkstationExperienceMode::Probe,
            seed,
            false,
        )?;
        Ok(experience)
    }

    fn experience(
        &self,
        learned_checkpoint: &[u8],
        mode: WorkstationExperienceMode,
        seed: u64,
        commits_learning: bool,
    ) -> Result<(WorkstationExperience, Vec<u8>), WorkstationCourseError> {
        let (first, replay) = std::thread::scope(|scope| {
            let replay =
                scope.spawn(|| execute(learned_checkpoint, &self.pose_checkpoint, mode, seed));
            let first = execute(learned_checkpoint, &self.pose_checkpoint, mode, seed)?;
            let replay = replay.join().map_err(|_| {
                WorkstationCourseError::InvalidEvidence(
                    "workstation replay worker panicked".to_string(),
                )
            })??;
            Ok::<_, WorkstationCourseError>((first, replay))
        })?;
        let replay_exact = first == replay;
        let mut experience = assess(mode, seed, commits_learning, &first)?;
        experience.replay_exact = replay_exact;
        if !replay_exact {
            experience.verdict = WorkstationVerdict::Failed;
        }
        Ok((experience, first.checkpoint_after))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Response {
    OnKey,
    Passive,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Execution {
    checkpoint_before: Vec<u8>,
    checkpoint_after: Vec<u8>,
    steps: Vec<WorkstationStep>,
}

fn execute(
    learned_checkpoint: &[u8],
    pose_checkpoint: &[u8],
    mode: WorkstationExperienceMode,
    seed: u64,
) -> Result<Execution, WorkstationCourseError> {
    let external_shift = if mode == WorkstationExperienceMode::Transfer {
        TRANSFER_IMPULSE
    } else {
        0
    };
    let checkpoint_before = if mode == WorkstationExperienceMode::FreshBodyControl {
        let fresh = WorkstationHarness::new(seed)?.save()?.canonical_bytes()?;
        positioned_checkpoint(&fresh, pose_checkpoint, external_shift)?
    } else {
        positioned_checkpoint(learned_checkpoint, pose_checkpoint, external_shift)?
    };
    let (key_press_depth, key_release_depth) = if matches!(
        mode,
        WorkstationExperienceMode::ActionOnlyDevelopment | WorkstationExperienceMode::Development
    ) {
        (PRACTICE_PRESS_DEPTH, PRACTICE_RELEASE_DEPTH)
    } else {
        (KEY_PRESS_DEPTH, KEY_RELEASE_DEPTH)
    };
    let response = match mode {
        WorkstationExperienceMode::PassiveDevelopment => Response::Passive,
        WorkstationExperienceMode::ActionOnlyDevelopment => Response::None,
        _ => Response::OnKey,
    };
    let initial_frame = generated_frame(seed, false)?;
    let changed_frame = generated_frame(seed, true)?;
    let mut current_frame = initial_frame.clone();
    let presentation = WorkstationPresentation::with_monitor_frame(initial_frame);
    let checkpoint = WorkstationCheckpoint::decode(&checkpoint_before)?;
    let mut session = WorkstationSession::from_body_checkpoint_with_key_depths(
        checkpoint,
        presentation,
        key_press_depth,
        key_release_depth,
    )?;
    let step_count = if mode == WorkstationExperienceMode::Demonstration {
        DEMONSTRATION_STEPS
    } else {
        EXPERIENCE_STEPS
    };
    let demonstrator_key = KeyId(u16::try_from(seed % KEY_COUNT as u64).unwrap_or(0));
    let mut changed = false;
    let mut settle_next = false;
    let mut steps = Vec::with_capacity(step_count);
    if external_shift != 0 {
        let frame_sha256 = digest(current_frame.pixels());
        let observation = session.settle()?;
        steps.push(compact_step(observation, frame_sha256)?);
    }
    for index in 0..step_count {
        let frame_sha256 = digest(current_frame.pixels());
        let settling = settle_next;
        let observation = if settling {
            session.settle()?
        } else {
            match mode {
                WorkstationExperienceMode::Demonstration => {
                    session.step_with_external_key(demonstrator_key, index <= 2)?
                }
                WorkstationExperienceMode::PassiveDevelopment => {
                    session.step_with_external_key(demonstrator_key, false)?
                }
                _ => session.step()?,
            }
        };
        let key_pressed = observation
            .device_events
            .iter()
            .any(|event| matches!(event, DeviceEvent::KeyPressed { .. }));
        let key_released = observation
            .device_events
            .iter()
            .any(|event| matches!(event, DeviceEvent::KeyReleased { .. }));
        steps.push(compact_step(observation, frame_sha256)?);
        if settling {
            break;
        }
        let should_change = !changed
            && (response == Response::OnKey && key_pressed
                || response == Response::Passive && index == PASSIVE_CHANGE_STEP);
        if should_change {
            current_frame = changed_frame.clone();
            session.set_presentation(WorkstationPresentation::with_monitor_frame(
                changed_frame.clone(),
            ))?;
            changed = true;
        }
        if changed
            && key_released
            && response == Response::OnKey
            && mode != WorkstationExperienceMode::Demonstration
        {
            settle_next = true;
        }
    }
    Ok(Execution {
        checkpoint_before,
        checkpoint_after: session.body_checkpoint()?.canonical_bytes()?,
        steps,
    })
}

fn positioned_checkpoint(
    learned_checkpoint: &[u8],
    pose_checkpoint: &[u8],
    horizontal_shift: i16,
) -> Result<Vec<u8>, WorkstationCourseError> {
    let learned = WorkstationCheckpoint::decode(learned_checkpoint)?;
    let pose = WorkstationCheckpoint::decode(pose_checkpoint)?;
    let mut harness = WorkstationHarness::restore(learned)?;
    harness.reposition_from_checkpoint(&pose)?;
    if horizontal_shift != 0
        && !harness.perturb_body(
            BodyControl::PalmHorizontal {
                direction: if horizontal_shift.is_positive() {
                    Direction::Increase
                } else {
                    Direction::Decrease
                },
            },
            horizontal_shift.unsigned_abs(),
        )?
    {
        return Err(WorkstationCourseError::InvalidEvidence(
            "transfer pose did not move".to_string(),
        ));
    }
    Ok(harness.save()?.canonical_bytes()?)
}

fn generated_frame(seed: u64, changed: bool) -> Result<MonitorFrame, WorkstationCourseError> {
    let mut state = seed ^ if changed { 0xa076_1d64_78bd_642f } else { 0 };
    let mut pixels = Vec::with_capacity(64);
    for index in 0..64 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let bit = (state >> (index % 17)) & 1;
        pixels.push(if bit == 0 { 24 } else { 232 });
    }
    Ok(MonitorFrame::new(8, 8, pixels)?)
}

fn compact_step(
    observation: SessionObservation,
    application_frame_sha256: String,
) -> Result<WorkstationStep, WorkstationCourseError> {
    let sample = bincode::serialize(&observation.sample)
        .map_err(|error| WorkstationCourseError::Serialization(error.to_string()))?;
    Ok(WorkstationStep {
        sequence: observation.sequence,
        application_frame_sha256,
        sample_sha256: digest(&sample),
        body: observation.body,
        device_events: observation.device_events,
        device_after: observation.device_after,
        world_fingerprint: observation.world_fingerprint,
        session_fingerprint: observation.session_fingerprint,
    })
}

fn assess(
    mode: WorkstationExperienceMode,
    seed: u64,
    commits_learning: bool,
    execution: &Execution,
) -> Result<WorkstationExperience, WorkstationCourseError> {
    let external = mode == WorkstationExperienceMode::Demonstration;
    let key_events = execution
        .steps
        .iter()
        .map(|step| {
            step.device_events
                .iter()
                .filter(|event| matches!(event, DeviceEvent::KeyPressed { .. }))
                .count()
        })
        .sum::<usize>();
    let screen_changes = execution
        .steps
        .windows(2)
        .filter(|pair| pair[0].application_frame_sha256 != pair[1].application_frame_sha256)
        .count();
    let unique_returned_screen_changes = if external {
        0
    } else {
        execution
            .steps
            .windows(2)
            .filter(|pair| returned_screen_change(&pair[0], &pair[1]))
            .count()
    };
    let physical_work = execution.steps.iter().fold(0_u64, |sum, step| {
        sum.saturating_add(step.body.metrics.physical_work)
    });
    let plasticity_updates = execution.steps.iter().fold(0_u64, |sum, step| {
        sum.saturating_add(step.body.metrics.plasticity_updates)
    });
    let naturally_quiescent = execution
        .steps
        .iter()
        .all(|step| step.body.naturally_quiescent);
    let mut verdict = match mode {
        WorkstationExperienceMode::Demonstration => {
            if key_events > 0
                && screen_changes > 0
                && execution
                    .steps
                    .iter()
                    .all(|step| step.body.boundary_parents.is_empty())
            {
                WorkstationVerdict::Presented
            } else {
                WorkstationVerdict::Failed
            }
        }
        WorkstationExperienceMode::PassiveDevelopment => {
            if key_events == 0 && screen_changes > 0 {
                WorkstationVerdict::Presented
            } else {
                WorkstationVerdict::Failed
            }
        }
        WorkstationExperienceMode::ActionOnlyDevelopment => {
            if key_events > 0 && screen_changes == 0 {
                WorkstationVerdict::Presented
            } else {
                WorkstationVerdict::Failed
            }
        }
        _ => {
            if unique_returned_screen_changes > 0 {
                WorkstationVerdict::Passed
            } else {
                WorkstationVerdict::Failed
            }
        }
    };
    if physical_work > PHYSICAL_WORK_BOUND {
        verdict = WorkstationVerdict::BudgetExceeded;
    } else if !naturally_quiescent {
        verdict = WorkstationVerdict::Failed;
    }
    let (key_press_depth, key_release_depth) = if matches!(
        mode,
        WorkstationExperienceMode::ActionOnlyDevelopment | WorkstationExperienceMode::Development
    ) {
        (PRACTICE_PRESS_DEPTH, PRACTICE_RELEASE_DEPTH)
    } else {
        (KEY_PRESS_DEPTH, KEY_RELEASE_DEPTH)
    };
    Ok(WorkstationExperience {
        id: format!("workstation-{mode:?}-{seed:016x}").to_lowercase(),
        mode,
        seed,
        key_press_depth,
        key_release_depth,
        external_pose_impulse: if mode == WorkstationExperienceMode::Transfer {
            TRANSFER_IMPULSE
        } else {
            0
        },
        steps: execution.steps.clone(),
        organism_key_events: if external { 0 } else { key_events },
        external_key_events: if external { key_events } else { 0 },
        screen_changes,
        unique_returned_screen_changes,
        checkpoint_before_sha256: digest(&execution.checkpoint_before),
        checkpoint_after_sha256: digest(&execution.checkpoint_after),
        replay_exact: false,
        mutation_discarded: !commits_learning,
        verdict,
        physical_work,
        plasticity_updates,
        naturally_quiescent,
    })
}

fn returned_screen_change(before: &WorkstationStep, after: &WorkstationStep) -> bool {
    let pressed = before
        .device_events
        .iter()
        .any(|event| matches!(event, DeviceEvent::KeyPressed { .. }));
    let frame_changed = before.application_frame_sha256 != after.application_frame_sha256;
    matches!(after.body.boundary_parents.as_slice(), [parent]
        if pressed && frame_changed && before.body.crossings.contains(parent))
}

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkstationCourseError {
    Workstation(String),
    Serialization(String),
    InvalidEvidence(String),
    OutputExists(String),
    Io(String),
}

impl fmt::Display for WorkstationCourseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workstation(message) => write!(formatter, "workstation failed: {message}"),
            Self::Serialization(message) => write!(formatter, "serialization failed: {message}"),
            Self::InvalidEvidence(message) => write!(formatter, "invalid evidence: {message}"),
            Self::OutputExists(path) => write!(formatter, "output already exists: {path}"),
            Self::Io(message) => write!(formatter, "I/O failed: {message}"),
        }
    }
}

impl std::error::Error for WorkstationCourseError {}

impl From<academy_workstation::WorldError> for WorkstationCourseError {
    fn from(value: academy_workstation::WorldError) -> Self {
        Self::Workstation(value.to_string())
    }
}

impl From<truelearner_workstation::WorkstationError> for WorkstationCourseError {
    fn from(value: truelearner_workstation::WorkstationError) -> Self {
        Self::Workstation(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_application_frames_are_deterministic_and_distinct() {
        let before = generated_frame(91_001, false).unwrap();
        let replay = generated_frame(91_001, false).unwrap();
        let after = generated_frame(91_001, true).unwrap();
        assert_eq!(before, replay);
        assert_ne!(before, after);
    }

    #[test]
    fn returned_screen_change_requires_the_exact_preceding_crossing() {
        let parent = MotorEffect {
            at: 1,
            control: BodyControl::PalmDepth {
                direction: Direction::Increase,
            },
            impulse: 16,
            cause: 2,
        };
        let mut harness = WorkstationHarness::new(91_002).unwrap();
        let sample = academy_workstation::WorkstationWorld::new()
            .unwrap()
            .sense(harness.state())
            .unwrap();
        let body = harness.step(sample).unwrap();
        let step =
            |frame: &str, crossings: Vec<MotorEffect>, parents: Vec<MotorEffect>| WorkstationStep {
                sequence: 0,
                application_frame_sha256: frame.to_string(),
                sample_sha256: String::new(),
                body: WorkstationStepObservation {
                    crossings,
                    boundary_parents: parents,
                    ..body.clone()
                },
                device_events: vec![DeviceEvent::KeyPressed { key: 17 }],
                device_after: DeviceState::default(),
                world_fingerprint: String::new(),
                session_fingerprint: String::new(),
            };
        let before = step("a", vec![parent], Vec::new());
        let after = step("b", Vec::new(), vec![parent]);
        assert!(returned_screen_change(&before, &after));
        let ambiguous = step("b", Vec::new(), vec![parent, parent]);
        assert!(!returned_screen_change(&before, &ambiguous));
    }
}
