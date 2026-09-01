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
    verify_choice_laws, BodyControl, BodyLinkId, BodyTraceEvent, Direction, WorkstationCheckpoint,
    WorkstationHarness, WorkstationStepObservation,
};

const PRACTICE_PRESS_DEPTH: i16 = 640;
const PRACTICE_RELEASE_DEPTH: i16 = 608;
const DEMONSTRATION_STEPS: usize = 5;
const EXPERIENCE_STEPS: usize = 16;
const AUTOMATICITY_DEVELOPMENT_USES: usize = 7;
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
    AutomaticityBaseline,
    AutomaticityDevelopment,
    AutomaticityInterference,
    AutomaticityProbe,
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
pub enum RepeatedUseEvidenceState {
    Unknown,
    Emerging,
    Automatic,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepeatedUseEvidence {
    pub state: RepeatedUseEvidenceState,
    pub closed_development_uses: usize,
    pub screen_closed_composites: usize,
    pub reused_composites: usize,
    pub baseline_physical_work: u64,
    pub automatic_physical_work: u64,
    pub saved_physical_work_per_use: u64,
    pub formation_work: u64,
    pub break_even_uses: u64,
    pub same_external_trace: bool,
    pub no_return_control: bool,
    pub interference_survived: bool,
    pub checkpoint_retained: bool,
    pub exact_replay: bool,
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
    pub automaticity_work_before: u64,
    pub automaticity_work_after: u64,
    pub screen_closed_composite_links: Vec<BodyLinkId>,
    pub retained_composite_links_traversed: Vec<BodyLinkId>,
    pub retained_composite_traversal_steps: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationCourseRun {
    pub schema_version: u16,
    pub seed: u64,
    pub capability: String,
    pub evidence_state: ScreenDeviceEvidenceState,
    pub automaticity: RepeatedUseEvidence,
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

        let (action_development, action_body) = self.experience_observing_automaticity(
            &initial,
            WorkstationExperienceMode::ActionOnlyDevelopment,
            self.seed.saturating_add(5),
            true,
            &[],
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
        let automaticity_probe_seed = self.seed.saturating_add(90);
        let (automaticity_baseline, _) = self.experience_observing_automaticity(
            &developed,
            WorkstationExperienceMode::AutomaticityBaseline,
            automaticity_probe_seed,
            false,
            &[],
        )?;
        experiences.push(automaticity_baseline.clone());

        let mut repeated = developed;
        let mut automaticity_development = Vec::with_capacity(AUTOMATICITY_DEVELOPMENT_USES);
        let mut learned_composite_links = Vec::new();
        for repetition in 0..AUTOMATICITY_DEVELOPMENT_USES {
            let (experience, next) = self.experience_observing_automaticity(
                &repeated,
                WorkstationExperienceMode::AutomaticityDevelopment,
                self.seed
                    .saturating_add(100 + u64::try_from(repetition).unwrap_or_default()),
                true,
                &learned_composite_links,
            )?;
            for link in &experience.screen_closed_composite_links {
                if !learned_composite_links.contains(link) {
                    learned_composite_links.push(*link);
                }
            }
            automaticity_development.push(experience.clone());
            experiences.push(experience);
            repeated = next;
        }
        let (automaticity_interference, developed) = self.experience(
            &repeated,
            WorkstationExperienceMode::AutomaticityInterference,
            self.seed.saturating_add(180),
            true,
        )?;
        experiences.push(automaticity_interference.clone());
        let (automaticity_probe, _) = self.experience_observing_automaticity(
            &developed,
            WorkstationExperienceMode::AutomaticityProbe,
            automaticity_probe_seed,
            false,
            &learned_composite_links,
        )?;
        experiences.push(automaticity_probe.clone());
        let automaticity = repeated_use_evidence(
            &automaticity_baseline,
            &automaticity_development,
            &automaticity_probe,
            &automaticity_interference,
            &action_development,
            &action_probe,
        );
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
            schema_version: 2,
            seed: self.seed,
            capability: "screen_device_screen_contingency".to_string(),
            evidence_state,
            automaticity,
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
        self.experience_internal(learned_checkpoint, mode, seed, commits_learning, None)
    }

    fn experience_observing_automaticity(
        &self,
        learned_checkpoint: &[u8],
        mode: WorkstationExperienceMode,
        seed: u64,
        commits_learning: bool,
        retained_links: &[BodyLinkId],
    ) -> Result<(WorkstationExperience, Vec<u8>), WorkstationCourseError> {
        self.experience_internal(
            learned_checkpoint,
            mode,
            seed,
            commits_learning,
            Some(retained_links),
        )
    }

    fn experience_internal(
        &self,
        learned_checkpoint: &[u8],
        mode: WorkstationExperienceMode,
        seed: u64,
        commits_learning: bool,
        retained_links: Option<&[BodyLinkId]>,
    ) -> Result<(WorkstationExperience, Vec<u8>), WorkstationCourseError> {
        let (first, replay) = std::thread::scope(|scope| {
            let replay = scope.spawn(|| {
                execute(
                    learned_checkpoint,
                    &self.pose_checkpoint,
                    mode,
                    seed,
                    retained_links,
                )
            });
            let first = execute(
                learned_checkpoint,
                &self.pose_checkpoint,
                mode,
                seed,
                retained_links,
            )?;
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
    automaticity_work_before: u64,
    automaticity_work_after: u64,
    screen_closed_composite_links: Vec<BodyLinkId>,
    retained_composite_links_traversed: Vec<BodyLinkId>,
    retained_composite_traversal_steps: Vec<usize>,
}

fn execute(
    learned_checkpoint: &[u8],
    pose_checkpoint: &[u8],
    mode: WorkstationExperienceMode,
    seed: u64,
    retained_links: Option<&[BodyLinkId]>,
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
        WorkstationExperienceMode::ActionOnlyDevelopment
            | WorkstationExperienceMode::Development
            | WorkstationExperienceMode::AutomaticityDevelopment
    ) {
        (PRACTICE_PRESS_DEPTH, PRACTICE_RELEASE_DEPTH)
    } else {
        (KEY_PRESS_DEPTH, KEY_RELEASE_DEPTH)
    };
    let response = match mode {
        WorkstationExperienceMode::PassiveDevelopment
        | WorkstationExperienceMode::AutomaticityInterference => Response::Passive,
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
    let automaticity_work_before = checkpoint_automaticity_work(&checkpoint_before)?;
    let mut automaticity_work_after = automaticity_work_before;
    let mut known_links = retained_links.unwrap_or_default().to_vec();
    let mut screen_closed_composite_links = Vec::new();
    let mut retained_composite_links_traversed = Vec::new();
    let mut retained_composite_traversal_steps = Vec::new();
    if external_shift != 0 {
        let frame_sha256 = digest(current_frame.pixels());
        let observation = session.settle()?;
        steps.push(compact_step(observation, frame_sha256)?);
    }
    for index in 0..step_count {
        let frame_sha256 = digest(current_frame.pixels());
        let settling = settle_next;
        let (observation, trace) = if retained_links.is_some() {
            let traced = if settling {
                session.settle_traced()?
            } else {
                session.step_traced()?
            };
            verify_choice_laws(&traced.1).map_err(|violation| {
                WorkstationCourseError::InvalidEvidence(format!(
                    "automaticity trace violates choice law: {violation:?}"
                ))
            })?;
            traced
        } else if mode == WorkstationExperienceMode::AutomaticityInterference {
            (session.observe()?, Vec::new())
        } else if settling {
            (session.settle()?, Vec::new())
        } else {
            let observation = match mode {
                WorkstationExperienceMode::Demonstration => {
                    session.step_with_external_key(demonstrator_key, index <= 2)?
                }
                WorkstationExperienceMode::PassiveDevelopment
                | WorkstationExperienceMode::AutomaticityInterference => {
                    session.step_with_external_key(demonstrator_key, false)?
                }
                _ => session.step()?,
            };
            (observation, Vec::new())
        };
        let key_pressed = observation
            .device_events
            .iter()
            .any(|event| matches!(event, DeviceEvent::KeyPressed { .. }));
        let key_released = observation
            .device_events
            .iter()
            .any(|event| matches!(event, DeviceEvent::KeyReleased { .. }));
        let step = compact_step(observation, frame_sha256)?;
        let returned_screen = steps
            .last()
            .is_some_and(|before| returned_screen_change(before, &step));
        if retained_links.is_some() {
            let mut traversed_at_step = false;
            for event in &trace {
                let BodyTraceEvent::Arrival(arrival) = event else {
                    continue;
                };
                if let Some(link) = arrival.via {
                    if known_links.contains(&link)
                        && !retained_composite_links_traversed.contains(&link)
                    {
                        retained_composite_links_traversed.push(link);
                    }
                    traversed_at_step |= known_links.contains(&link);
                }
            }
            if traversed_at_step {
                retained_composite_traversal_steps.push(steps.len());
            }
            let work = session_automaticity_work(&session)?;
            let formed = work.saturating_sub(automaticity_work_after);
            if returned_screen && formed > 0 {
                for link in formed_composite_links(&trace, formed)? {
                    if !screen_closed_composite_links.contains(&link) {
                        screen_closed_composite_links.push(link);
                    }
                    if !known_links.contains(&link) {
                        known_links.push(link);
                    }
                }
            }
            automaticity_work_after = work;
        }
        steps.push(step);
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
    let checkpoint_after = session.body_checkpoint()?.canonical_bytes()?;
    if retained_links.is_none() {
        automaticity_work_after = checkpoint_automaticity_work(&checkpoint_after)?;
    }
    Ok(Execution {
        checkpoint_before,
        checkpoint_after,
        steps,
        automaticity_work_before,
        automaticity_work_after,
        screen_closed_composite_links,
        retained_composite_links_traversed,
        retained_composite_traversal_steps,
    })
}

fn checkpoint_automaticity_work(checkpoint: &[u8]) -> Result<u64, WorkstationCourseError> {
    let checkpoint = WorkstationCheckpoint::decode(checkpoint)?;
    Ok(WorkstationHarness::restore(checkpoint)?
        .automaticity_work()
        .total())
}

fn session_automaticity_work(session: &WorkstationSession) -> Result<u64, WorkstationCourseError> {
    Ok(WorkstationHarness::restore(session.body_checkpoint()?)?
        .automaticity_work()
        .total())
}

fn formed_composite_links(
    trace: &[BodyTraceEvent],
    formed: u64,
) -> Result<Vec<BodyLinkId>, WorkstationCourseError> {
    let mut parent_links = Vec::new();
    for event in trace {
        let BodyTraceEvent::Return(returned) = event else {
            continue;
        };
        if let Some(path) = returned.path {
            for link in [path.first, path.second] {
                if !parent_links.contains(&link) {
                    parent_links.push(link);
                }
            }
        }
    }
    let mut candidates = trace
        .iter()
        .filter_map(|event| match event {
            BodyTraceEvent::Strengthened(strengthened)
                if !parent_links.contains(&strengthened.link) =>
            {
                Some(strengthened.link)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    candidates.dedup();
    let formed = usize::try_from(formed).map_err(|_| {
        WorkstationCourseError::InvalidEvidence(
            "automaticity formation count does not fit this platform".to_string(),
        )
    })?;
    if candidates.len() < formed {
        return Err(WorkstationCourseError::InvalidEvidence(
            "automaticity formation lacks a strengthened retained link".to_string(),
        ));
    }
    Ok(candidates.split_off(candidates.len() - formed))
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
            BodyControl::new(
                truelearner_workstation::BodyAxis::PalmHorizontal,
                if horizontal_shift.is_positive() {
                    Direction::Increase
                } else {
                    Direction::Decrease
                },
            ),
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
        WorkstationExperienceMode::AutomaticityInterference => {
            if screen_changes > 0 && unique_returned_screen_changes == 0 {
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
        WorkstationExperienceMode::ActionOnlyDevelopment
            | WorkstationExperienceMode::Development
            | WorkstationExperienceMode::AutomaticityDevelopment
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
        automaticity_work_before: execution.automaticity_work_before,
        automaticity_work_after: execution.automaticity_work_after,
        screen_closed_composite_links: execution.screen_closed_composite_links.clone(),
        retained_composite_links_traversed: execution.retained_composite_links_traversed.clone(),
        retained_composite_traversal_steps: execution.retained_composite_traversal_steps.clone(),
    })
}

fn repeated_use_evidence(
    baseline: &WorkstationExperience,
    development: &[WorkstationExperience],
    probe: &WorkstationExperience,
    interference: &WorkstationExperience,
    no_return_development: &WorkstationExperience,
    no_return_probe: &WorkstationExperience,
) -> RepeatedUseEvidence {
    let closed_development_uses = development
        .iter()
        .filter(|experience| {
            experience.verdict == WorkstationVerdict::Passed
                && experience.unique_returned_screen_changes > 0
                && experience.replay_exact
                && experience.naturally_quiescent
        })
        .count();
    let mut screen_closed_links = development
        .iter()
        .flat_map(|experience| experience.screen_closed_composite_links.iter().copied())
        .collect::<Vec<_>>();
    screen_closed_links.sort_unstable();
    screen_closed_links.dedup();
    let mut reused_links = probe
        .retained_composite_links_traversed
        .iter()
        .copied()
        .filter(|link| screen_closed_links.contains(link))
        .collect::<Vec<_>>();
    reused_links.sort_unstable();
    reused_links.dedup();
    let formation_work = development
        .first()
        .zip(development.last())
        .map_or(0, |(first, last)| {
            last.automaticity_work_after
                .saturating_sub(first.automaticity_work_before)
        });
    let saved_physical_work_per_use = baseline.physical_work.saturating_sub(probe.physical_work);
    let break_even_uses = if saved_physical_work_per_use == 0 {
        u64::MAX
    } else {
        formation_work.div_ceil(saved_physical_work_per_use)
    };
    let same_external_trace = same_external_trace(baseline, probe);
    let no_return_control = no_return_development.verdict == WorkstationVerdict::Presented
        && no_return_development.screen_changes == 0
        && no_return_development.unique_returned_screen_changes == 0
        && no_return_development
            .screen_closed_composite_links
            .is_empty()
        && no_return_probe.verdict == WorkstationVerdict::Failed;
    let interference_survived = interference.verdict == WorkstationVerdict::Presented
        && interference.screen_changes > 0
        && interference.unique_returned_screen_changes == 0
        && interference.screen_closed_composite_links.is_empty()
        && interference.automaticity_work_before == interference.automaticity_work_after
        && interference.replay_exact
        && interference.naturally_quiescent;
    let exact_replay = baseline.replay_exact
        && development.iter().all(|experience| experience.replay_exact)
        && interference.replay_exact
        && probe.replay_exact;
    let checkpoint_retained = probe.mutation_discarded && !reused_links.is_empty();
    let automatic = closed_development_uses == development.len()
        && !screen_closed_links.is_empty()
        && !reused_links.is_empty()
        && probe.verdict == WorkstationVerdict::Passed
        && probe.naturally_quiescent
        && same_external_trace
        && saved_physical_work_per_use > 0
        && break_even_uses <= 1
        && no_return_control
        && interference_survived
        && checkpoint_retained
        && exact_replay;
    let state = if automatic {
        RepeatedUseEvidenceState::Automatic
    } else if !screen_closed_links.is_empty() || saved_physical_work_per_use > 0 {
        RepeatedUseEvidenceState::Emerging
    } else {
        RepeatedUseEvidenceState::Unknown
    };
    RepeatedUseEvidence {
        state,
        closed_development_uses,
        screen_closed_composites: screen_closed_links.len(),
        reused_composites: reused_links.len(),
        baseline_physical_work: baseline.physical_work,
        automatic_physical_work: probe.physical_work,
        saved_physical_work_per_use,
        formation_work,
        break_even_uses,
        same_external_trace,
        no_return_control,
        interference_survived,
        checkpoint_retained,
        exact_replay,
    }
}

fn same_external_trace(baseline: &WorkstationExperience, probe: &WorkstationExperience) -> bool {
    if baseline.steps.len() != probe.steps.len()
        || baseline.screen_changes != probe.screen_changes
        || baseline.unique_returned_screen_changes != probe.unique_returned_screen_changes
    {
        return false;
    }
    let baseline_events = baseline
        .steps
        .iter()
        .enumerate()
        .flat_map(|(step, observation)| {
            observation
                .device_events
                .iter()
                .cloned()
                .map(move |event| (step, event))
        })
        .collect::<Vec<_>>();
    let probe_events = probe
        .steps
        .iter()
        .enumerate()
        .flat_map(|(step, observation)| {
            observation
                .device_events
                .iter()
                .cloned()
                .map(move |event| (step, event))
        })
        .collect::<Vec<_>>();
    if baseline_events != probe_events {
        return false;
    }
    returned_screen_parent_trace(baseline) == returned_screen_parent_trace(probe)
}

fn returned_screen_parent_trace(
    experience: &WorkstationExperience,
) -> Vec<(usize, BodyControl, i32, u64)> {
    experience
        .steps
        .windows(2)
        .enumerate()
        .filter_map(|(step, pair)| {
            let [before, after] = pair else {
                return None;
            };
            if !returned_screen_change(before, after) {
                return None;
            }
            let [parent] = after.body.boundary_parents.as_slice() else {
                return None;
            };
            let origin = before.body.crossings.iter().map(|effect| effect.at).min()?;
            Some((
                step,
                parent.control,
                parent.impulse,
                parent.at.saturating_sub(origin),
            ))
        })
        .collect()
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
            control: BodyControl::new(
                truelearner_workstation::BodyAxis::PalmDepth,
                Direction::Increase,
            ),
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
