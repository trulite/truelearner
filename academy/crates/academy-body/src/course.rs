use crate::world::FlatWorld;
use academy_workstation::{
    DeviceEvent, DeviceState, KeyId, WorkstationWorld, WorldError, WorldTransition, KEY_COUNT,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;
use truelearner_workstation::{
    BodyAxis, BodyMovement, Eye, LightField, MotorEffect, WorkstationError, WorkstationHarness,
    WorkstationStepObservation, WorldSample,
};

const STEPS_PER_EXPERIENCE: usize = 12;
const CONTACT_STEPS_PER_EXPERIENCE: usize = 16;
const DEMONSTRATION_STEPS_PER_EXPERIENCE: usize = 5;
const IMITATION_STEPS_PER_EXPERIENCE: usize = 6;
// The deepest diagnostic rung needs seven ordinary 16-unit palm steps from
// the frozen lesson checkpoint. This is exposure time, not an action hint.
const DEPTH_CONTROL_STEPS_PER_EXPERIENCE: usize = 7;
const PRACTICE_KEY_PRESS_DEPTH: i16 = 640;
const PRACTICE_KEY_RELEASE_DEPTH: i16 = 608;
const DEPTH_CONTROL_PRESS_DEPTHS: [i16; 6] = [640, 656, 672, 688, 704, 720];
const DEPTH_CONTROL_RELEASE_DEPTH: i16 = 608;
const PHYSICAL_WORK_BOUND: u64 = 2_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyCapability {
    GazeContingency,
    GazeControl,
    BinocularDepth,
    HandContingency,
    DigitSeparation,
    SelfWorld,
    Contact,
    VisualReach,
    TapHoldRelease,
    DragOpposition,
}

impl BodyCapability {
    pub const ORDER: [Self; 10] = [
        Self::GazeContingency,
        Self::GazeControl,
        Self::BinocularDepth,
        Self::HandContingency,
        Self::DigitSeparation,
        Self::SelfWorld,
        Self::Contact,
        Self::VisualReach,
        Self::TapHoldRelease,
        Self::DragOpposition,
    ];

    pub fn prerequisites(self) -> &'static [Self] {
        match self {
            Self::GazeContingency | Self::HandContingency => &[],
            Self::GazeControl => &[Self::GazeContingency],
            Self::BinocularDepth => &[Self::GazeControl],
            Self::DigitSeparation => &[Self::HandContingency],
            Self::SelfWorld => &[Self::BinocularDepth, Self::DigitSeparation],
            Self::Contact => &[Self::SelfWorld],
            Self::VisualReach => &[Self::Contact],
            Self::TapHoldRelease => &[Self::VisualReach],
            Self::DragOpposition => &[Self::TapHoldRelease],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyCourseKind {
    EyeControl,
    HandAndFingerControl,
    EyeHandCoordination,
    WorkstationContact,
}

impl BodyCourseKind {
    pub const ORDER: [Self; 4] = [
        Self::EyeControl,
        Self::HandAndFingerControl,
        Self::EyeHandCoordination,
        Self::WorkstationContact,
    ];

    pub const fn capabilities(self) -> &'static [BodyCapability] {
        match self {
            Self::EyeControl => &[
                BodyCapability::GazeContingency,
                BodyCapability::GazeControl,
                BodyCapability::BinocularDepth,
            ],
            Self::HandAndFingerControl => &[
                BodyCapability::HandContingency,
                BodyCapability::DigitSeparation,
            ],
            Self::EyeHandCoordination => &[BodyCapability::SelfWorld],
            Self::WorkstationContact => &[
                BodyCapability::Contact,
                BodyCapability::VisualReach,
                BodyCapability::TapHoldRelease,
                BodyCapability::DragOpposition,
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", content = "capability", rename_all = "snake_case")]
pub enum BodyCourseOutcome {
    Acquired,
    Failed(BodyCapability),
    NotReached,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyCourseProgress {
    pub course: BodyCourseKind,
    pub acquired: Vec<BodyCapability>,
    pub outcome: BodyCourseOutcome,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyExperienceMode {
    Demonstration,
    Development,
    DepthControl,
    Probe,
    Transfer,
    Retention,
    Control,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyVerdict {
    Presented,
    Passed,
    Failed,
    MissingExploration,
    BudgetExceeded,
    NotReached,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyExperience {
    pub id: String,
    pub capability: BodyCapability,
    pub mode: BodyExperienceMode,
    pub seed: u64,
    pub samples: Vec<WorldSample>,
    pub observations: Vec<WorkstationStepObservation>,
    pub world_observations: Vec<BodyWorldObservation>,
    pub key_press_depth: Option<i16>,
    pub key_release_depth: Option<i16>,
    pub checkpoint_before: Vec<u8>,
    pub checkpoint_after: Vec<u8>,
    pub durable_unchanged: bool,
    pub replay_exact: bool,
    pub verdict: BodyVerdict,
    pub physical_work: u64,
    pub plasticity_updates: u64,
    pub naturally_quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyWorldObservation {
    pub events: Vec<BodyWorldEvent>,
    pub boundary_parents: Vec<MotorEffect>,
    pub progress_parents: Vec<MotorEffect>,
    pub fingerprint: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyWorldCause {
    Organism,
    Demonstrator,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyWorldEvent {
    pub cause: BodyWorldCause,
    pub event: DeviceEvent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CourseRun {
    pub schema_version: u16,
    pub seed: u64,
    pub courses: Vec<BodyCourseProgress>,
    pub acquired: Vec<BodyCapability>,
    pub first_failure: Option<BodyCapability>,
    pub experiences: Vec<BodyExperience>,
    pub exact_replay: bool,
}

pub struct BodyCourse {
    seed: u64,
    harness: WorkstationHarness,
    acquired: BTreeSet<BodyCapability>,
    experiences: Vec<BodyExperience>,
}

impl BodyCourse {
    pub fn new(seed: u64) -> Result<Self, BodyCourseError> {
        Ok(Self {
            seed,
            harness: WorkstationHarness::new(seed)?,
            acquired: BTreeSet::new(),
            experiences: Vec::new(),
        })
    }

    pub fn checkpoint_bytes(&self) -> Result<Vec<u8>, BodyCourseError> {
        Ok(self.harness.save()?.canonical_bytes()?)
    }

    pub fn experiences(&self) -> &[BodyExperience] {
        &self.experiences
    }

    pub fn experience(
        &mut self,
        capability: BodyCapability,
        mode: BodyExperienceMode,
        seed: u64,
    ) -> Result<BodyExperience, BodyCourseError> {
        self.experience_with_key_depths(capability, mode, seed, None)
    }

    fn experience_with_key_depths(
        &mut self,
        capability: BodyCapability,
        mode: BodyExperienceMode,
        seed: u64,
        key_depths: Option<(i16, i16)>,
    ) -> Result<BodyExperience, BodyCourseError> {
        if !capability
            .prerequisites()
            .iter()
            .all(|required| self.acquired.contains(required))
        {
            return Err(BodyCourseError::Prerequisite(capability));
        }
        let durable_before = self.checkpoint_bytes()?;
        let checkpoint = truelearner_workstation::WorkstationCheckpoint::decode(&durable_before)?;
        let mut working = WorkstationHarness::restore(checkpoint)?;
        let mut world = ExperienceWorld::generated(seed, capability, mode, key_depths)?;
        let (key_press_depth, key_release_depth) = world.key_depths();
        let steps = steps_per_experience(capability, mode);
        let mut samples = Vec::with_capacity(steps);
        let mut observations = Vec::with_capacity(steps);
        let mut world_observations = Vec::with_capacity(steps);
        let mut physical_work = 0_u64;
        let mut plasticity_updates = 0_u64;
        let mut naturally_quiescent = true;
        let mut budget_exceeded = false;
        let mut boundary_parents = Vec::new();
        let mut progress_parents = Vec::new();
        let mut settle_next = false;
        for _ in 0..steps {
            let sample = world.sample(&working.read()?)?;
            let settling = settle_next;
            let observation = if settling {
                working.settle_with_causal_parents(
                    sample.clone(),
                    &boundary_parents,
                    &progress_parents,
                )?
            } else {
                working.step_with_causal_parents(
                    sample.clone(),
                    &boundary_parents,
                    &progress_parents,
                )?
            };
            let world_observation = world.advance(&observation)?;
            boundary_parents.clone_from(&world_observation.boundary_parents);
            progress_parents.clone_from(&world_observation.progress_parents);
            physical_work = physical_work.saturating_add(observation.metrics.physical_work);
            plasticity_updates =
                plasticity_updates.saturating_add(observation.metrics.plasticity_updates);
            naturally_quiescent &= observation.naturally_quiescent;
            samples.push(sample);
            observations.push(observation);
            world_observations.push(world_observation);
            if physical_work > PHYSICAL_WORK_BOUND {
                budget_exceeded = true;
                break;
            }
            if settling {
                break;
            }
            settle_next = capability == BodyCapability::TapHoldRelease
                && mode == BodyExperienceMode::Development
                && tap_hold_release_completed(&world_observations, BodyWorldCause::Organism);
        }
        let checkpoint_after = working.save()?.canonical_bytes()?;
        let mut verdict = if mode == BodyExperienceMode::Demonstration {
            if tap_hold_release_completed(&world_observations, BodyWorldCause::Demonstrator) {
                BodyVerdict::Presented
            } else {
                BodyVerdict::Failed
            }
        } else if mode == BodyExperienceMode::DepthControl {
            if key_press_completed(&world_observations, BodyWorldCause::Organism) {
                BodyVerdict::Passed
            } else {
                BodyVerdict::Failed
            }
        } else {
            evaluate(capability, &samples, &observations, &world_observations)
        };
        if budget_exceeded {
            verdict = BodyVerdict::BudgetExceeded;
        }
        if !naturally_quiescent {
            verdict = BodyVerdict::Failed;
        }
        let replay_exact = replay(
            capability,
            mode,
            seed,
            key_depths,
            ReplayEvidence {
                checkpoint_before: &durable_before,
                samples: &samples,
                observations: &observations,
                world_observations: &world_observations,
                checkpoint_after: &checkpoint_after,
            },
        )?;
        if !replay_exact {
            verdict = BodyVerdict::Failed;
        }
        if commits_learning(mode) {
            self.harness = working;
        }
        let durable_unchanged = if commits_learning(mode) {
            false
        } else {
            self.checkpoint_bytes()? == durable_before
        };
        let experience = BodyExperience {
            id: format!(
                "body-{:?}-{:?}-{:016x}-{:04}",
                capability,
                mode,
                seed,
                self.experiences.len()
            )
            .to_lowercase(),
            capability,
            mode,
            seed,
            samples,
            observations,
            world_observations,
            key_press_depth,
            key_release_depth,
            checkpoint_before: durable_before,
            checkpoint_after,
            durable_unchanged,
            replay_exact,
            verdict,
            physical_work,
            plasticity_updates,
            naturally_quiescent,
        };
        self.experiences.push(experience.clone());
        Ok(experience)
    }

    pub fn run(mut self) -> Result<CourseRun, BodyCourseError> {
        let mut first_failure = None;
        let mut courses = Vec::with_capacity(BodyCourseKind::ORDER.len());
        for course in BodyCourseKind::ORDER {
            let mut course_acquired = Vec::with_capacity(course.capabilities().len());
            let mut course_failure = None;
            let mut course_blocked = false;
            for capability in course.capabilities().iter().copied() {
                if !capability
                    .prerequisites()
                    .iter()
                    .all(|required| self.acquired.contains(required))
                {
                    course_blocked = true;
                    break;
                }
                let capability_index = BodyCapability::ORDER
                    .iter()
                    .position(|candidate| *candidate == capability)
                    .expect("course partition contains only ordered capabilities");
                let development_seed = self.seed.saturating_add(capability_index as u64 * 10 + 1);
                let checkpoint_before_lesson = self.checkpoint_bytes()?;
                let mut development_required = true;
                if capability == BodyCapability::TapHoldRelease {
                    let demonstration = self.experience(
                        capability,
                        BodyExperienceMode::Demonstration,
                        development_seed.saturating_add(500_000),
                    )?;
                    if demonstration.verdict != BodyVerdict::Presented {
                        self.restore_checkpoint(&checkpoint_before_lesson)?;
                        course_failure = Some(capability);
                        break;
                    }
                    let imitation = self.experience(
                        capability,
                        BodyExperienceMode::Control,
                        development_seed.saturating_add(750_000),
                    )?;
                    if imitation.verdict == BodyVerdict::Passed {
                        development_required = false;
                    } else {
                        self.restore_checkpoint(&checkpoint_before_lesson)?;
                        for press_depth in DEPTH_CONTROL_PRESS_DEPTHS {
                            let depth_control = self.experience_with_key_depths(
                                capability,
                                BodyExperienceMode::DepthControl,
                                development_seed.saturating_add(875_000),
                                Some((press_depth, DEPTH_CONTROL_RELEASE_DEPTH)),
                            )?;
                            if depth_control.verdict != BodyVerdict::Passed {
                                break;
                            }
                        }
                    }
                }
                if development_required {
                    let development = self.experience(
                        capability,
                        BodyExperienceMode::Development,
                        development_seed,
                    )?;
                    if matches!(
                        development.verdict,
                        BodyVerdict::MissingExploration | BodyVerdict::BudgetExceeded
                    ) {
                        self.restore_checkpoint(&checkpoint_before_lesson)?;
                        course_failure = Some(capability);
                        break;
                    }
                }
                if capability == BodyCapability::TapHoldRelease {
                    let reference = truelearner_workstation::WorkstationCheckpoint::decode(
                        &checkpoint_before_lesson,
                    )?;
                    self.harness.reposition_from_checkpoint(&reference)?;
                }
                let probe = self.experience(
                    capability,
                    BodyExperienceMode::Probe,
                    development_seed.saturating_add(1_000_000),
                )?;
                if probe.verdict != BodyVerdict::Passed || !probe.durable_unchanged {
                    self.restore_checkpoint(&checkpoint_before_lesson)?;
                    course_failure = Some(capability);
                    break;
                }
                self.acquired.insert(capability);
                course_acquired.push(capability);
            }
            let outcome = if course_blocked {
                BodyCourseOutcome::NotReached
            } else if let Some(capability) = course_failure {
                first_failure.get_or_insert(capability);
                BodyCourseOutcome::Failed(capability)
            } else {
                BodyCourseOutcome::Acquired
            };
            courses.push(BodyCourseProgress {
                course,
                acquired: course_acquired,
                outcome,
            });
        }
        Ok(CourseRun {
            schema_version: 7,
            seed: self.seed,
            courses,
            acquired: self.acquired.iter().copied().collect(),
            first_failure,
            exact_replay: self
                .experiences
                .iter()
                .all(|experience| experience.replay_exact),
            experiences: self.experiences,
        })
    }

    fn restore_checkpoint(&mut self, bytes: &[u8]) -> Result<(), BodyCourseError> {
        let checkpoint = truelearner_workstation::WorkstationCheckpoint::decode(bytes)?;
        self.harness = WorkstationHarness::restore(checkpoint)?;
        Ok(())
    }
}

enum ExperienceWorld {
    Flat(FlatWorld),
    Workstation {
        visual: FlatWorld,
        device: Box<WorkstationWorld>,
        demonstrator: Option<Demonstrator>,
    },
}

struct Demonstrator {
    key: KeyId,
    step: usize,
    pressed: bool,
}

impl ExperienceWorld {
    fn generated(
        seed: u64,
        capability: BodyCapability,
        mode: BodyExperienceMode,
        key_depths: Option<(i16, i16)>,
    ) -> Result<Self, BodyCourseError> {
        if capability == BodyCapability::TapHoldRelease {
            let device = match key_depths {
                Some((press, release)) => WorkstationWorld::new_with_key_depths(press, release)?,
                None if mode == BodyExperienceMode::Development => {
                    WorkstationWorld::new_with_key_depths(
                        PRACTICE_KEY_PRESS_DEPTH,
                        PRACTICE_KEY_RELEASE_DEPTH,
                    )?
                }
                None => WorkstationWorld::new()?,
            };
            Ok(Self::Workstation {
                visual: FlatWorld::generated(seed, capability),
                device: Box::new(device),
                demonstrator: (mode == BodyExperienceMode::Demonstration).then_some(Demonstrator {
                    key: KeyId(u16::try_from(seed % KEY_COUNT as u64).unwrap_or(0)),
                    step: 0,
                    pressed: false,
                }),
            })
        } else {
            Ok(Self::Flat(FlatWorld::generated(seed, capability)))
        }
    }

    fn sample(
        &mut self,
        body: &truelearner_workstation::WorkstationRead,
    ) -> Result<WorldSample, BodyCourseError> {
        match self {
            Self::Flat(world) => world.sample(body),
            Self::Workstation {
                visual,
                device,
                demonstrator,
            } => {
                let visual_sample = visual.sample(body)?;
                let fields = compact_button_fields(
                    &visual_sample,
                    device.device(),
                    demonstrator.as_ref().map(|actor| actor.pressed),
                )?;
                let contacts = if demonstrator.is_some() {
                    *visual_sample.contacts()
                } else {
                    device.contact_samples(&body.state)?
                };
                Ok(WorldSample::new(fields, contacts)?)
            }
        }
    }

    fn key_depths(&self) -> (Option<i16>, Option<i16>) {
        match self {
            Self::Flat(_) => (None, None),
            Self::Workstation { device, .. } => (
                Some(device.key_press_depth()),
                Some(device.key_release_depth()),
            ),
        }
    }

    fn advance(
        &mut self,
        observation: &WorkstationStepObservation,
    ) -> Result<BodyWorldObservation, BodyCourseError> {
        match self {
            Self::Flat(world) => Ok(BodyWorldObservation {
                events: Vec::new(),
                boundary_parents: Vec::new(),
                progress_parents: world.progress_parents(observation),
                fingerprint: None,
            }),
            Self::Workstation {
                device,
                demonstrator,
                ..
            } => {
                let (cause, transition) = if let Some(actor) = demonstrator {
                    let pressed_after = (0..=2).contains(&actor.step);
                    let events =
                        device.advance_external_key(actor.key, actor.pressed, pressed_after)?;
                    actor.pressed = pressed_after;
                    actor.step = actor.step.saturating_add(1);
                    (
                        BodyWorldCause::Demonstrator,
                        WorldTransition::external(events),
                    )
                } else {
                    (
                        BodyWorldCause::Organism,
                        device.advance_observation(observation),
                    )
                };
                Ok(BodyWorldObservation {
                    events: transition
                        .events
                        .into_iter()
                        .map(|event| BodyWorldEvent { cause, event })
                        .collect(),
                    boundary_parents: transition.boundary_parents,
                    progress_parents: transition.progress_parents,
                    fingerprint: Some(device.fingerprint()?),
                })
            }
        }
    }
}

fn compact_button_fields(
    sample: &WorldSample,
    device: &DeviceState,
    demonstrator_pressed: Option<bool>,
) -> Result<[LightField; 2], BodyCourseError> {
    Ok([
        compact_button_field(sample.eye(Eye::Left), device, demonstrator_pressed)?,
        compact_button_field(sample.eye(Eye::Right), device, demonstrator_pressed)?,
    ])
}

fn compact_button_field(
    base: &LightField,
    device: &DeviceState,
    demonstrator_pressed: Option<bool>,
) -> Result<LightField, BodyCourseError> {
    let width = usize::from(base.width());
    let height = usize::from(base.height());
    let mut pixels = base.pixels().to_vec();
    if width < 5 || height < 5 {
        return Ok(base.clone());
    }
    let pressed = device.keys_down().next().is_some();
    let activated = device.long_pressed_keys().next().is_some();
    let center = width / 2;
    let button_y = if pressed { height - 1 } else { height - 2 };
    for x in center - 1..=center + 1 {
        pixels[button_y * width + x] = if pressed { 180 } else { 112 };
    }
    pixels[width + width - 2] = if activated { 255 } else { 16 };
    if let Some(down) = demonstrator_pressed {
        let finger_y = if down { height - 2 } else { height - 4 };
        pixels[finger_y * width + center] = 240;
    }
    Ok(LightField::new(base.width(), base.height(), pixels)?)
}

const fn commits_learning(mode: BodyExperienceMode) -> bool {
    matches!(
        mode,
        BodyExperienceMode::Demonstration | BodyExperienceMode::Development
    )
}

const fn steps_per_experience(capability: BodyCapability, mode: BodyExperienceMode) -> usize {
    if matches!(mode, BodyExperienceMode::Demonstration) {
        DEMONSTRATION_STEPS_PER_EXPERIENCE
    } else if matches!(mode, BodyExperienceMode::DepthControl) {
        DEPTH_CONTROL_STEPS_PER_EXPERIENCE
    } else if matches!(
        (capability, mode),
        (BodyCapability::TapHoldRelease, BodyExperienceMode::Control)
    ) {
        IMITATION_STEPS_PER_EXPERIENCE
    } else if matches!(capability, BodyCapability::Contact) {
        CONTACT_STEPS_PER_EXPERIENCE
    } else {
        STEPS_PER_EXPERIENCE
    }
}

struct ReplayEvidence<'a> {
    checkpoint_before: &'a [u8],
    samples: &'a [WorldSample],
    observations: &'a [WorkstationStepObservation],
    world_observations: &'a [BodyWorldObservation],
    checkpoint_after: &'a [u8],
}

fn replay(
    capability: BodyCapability,
    mode: BodyExperienceMode,
    seed: u64,
    key_depths: Option<(i16, i16)>,
    evidence: ReplayEvidence<'_>,
) -> Result<bool, BodyCourseError> {
    if evidence.samples.len() != evidence.observations.len()
        || evidence.samples.len() != evidence.world_observations.len()
    {
        return Ok(false);
    }
    let checkpoint =
        truelearner_workstation::WorkstationCheckpoint::decode(evidence.checkpoint_before)?;
    let mut harness = WorkstationHarness::restore(checkpoint)?;
    let mut world = ExperienceWorld::generated(seed, capability, mode, key_depths)?;
    let mut boundary_parents = Vec::new();
    let mut progress_parents = Vec::new();
    for ((expected_sample, expected_observation), expected_world_observation) in evidence
        .samples
        .iter()
        .zip(evidence.observations)
        .zip(evidence.world_observations)
    {
        let sample = world.sample(&harness.read()?)?;
        if &sample != expected_sample {
            return Ok(false);
        }
        let observation = if expected_observation.opportunity_admitted {
            harness.step_with_causal_parents(sample, &boundary_parents, &progress_parents)?
        } else {
            harness.settle_with_causal_parents(sample, &boundary_parents, &progress_parents)?
        };
        let world_observation = world.advance(&observation)?;
        if &observation != expected_observation || &world_observation != expected_world_observation
        {
            return Ok(false);
        }
        boundary_parents.clone_from(&world_observation.boundary_parents);
        progress_parents.clone_from(&world_observation.progress_parents);
    }
    Ok(harness.save()?.canonical_bytes()? == evidence.checkpoint_after)
}

fn evaluate(
    capability: BodyCapability,
    samples: &[WorldSample],
    observations: &[WorkstationStepObservation],
    world_observations: &[BodyWorldObservation],
) -> BodyVerdict {
    if capability == BodyCapability::DigitSeparation {
        let any_movement = observations
            .iter()
            .flat_map(|observation| &observation.movements)
            .any(|movement| movement.changed);
        if !any_movement {
            return BodyVerdict::MissingExploration;
        }
        return if has_digit_separation(
            observations
                .iter()
                .map(|observation| observation.movements.as_slice()),
        ) {
            BodyVerdict::Passed
        } else {
            BodyVerdict::Failed
        };
    }
    if capability == BodyCapability::BinocularDepth {
        let opposing_eye_movements = observations
            .iter()
            .filter(|observation| opposing_horizontal_eye_movement(&observation.movements))
            .count();
        if !observations
            .iter()
            .flat_map(|observation| &observation.movements)
            .any(|movement| movement.changed)
        {
            return BodyVerdict::MissingExploration;
        }
        return binocular_depth_verdict(
            opposing_eye_movements,
            binocular_alignment_consequences(samples, observations),
        );
    }
    let movements = observations
        .iter()
        .flat_map(|observation| &observation.movements)
        .copied()
        .collect::<Vec<_>>();
    evaluate_physical(
        capability,
        &movements,
        samples,
        gaze_visual_consequences(samples, observations),
        world_observations,
    )
}

fn evaluate_physical(
    capability: BodyCapability,
    movements: &[BodyMovement],
    samples: &[WorldSample],
    gaze_visual_consequences: usize,
    world_observations: &[BodyWorldObservation],
) -> BodyVerdict {
    let movements = movements
        .iter()
        .filter(|movement| movement.changed)
        .collect::<Vec<_>>();
    if movements.is_empty() {
        return BodyVerdict::MissingExploration;
    }
    let gaze = movements
        .iter()
        .filter(|movement| is_gaze(movement.axis))
        .count();
    let hand = movements
        .iter()
        .filter(|movement| !is_gaze(movement.axis))
        .count();
    let fingers = movements
        .iter()
        .filter(|movement| matches!(movement.axis, BodyAxis::FingerFlexion { .. }))
        .count();
    let palm = movements
        .iter()
        .filter(|movement| {
            matches!(
                movement.axis,
                BodyAxis::PalmHorizontal | BodyAxis::PalmVertical
            )
        })
        .count();
    let opposition = movements
        .iter()
        .filter(|movement| matches!(movement.axis, BodyAxis::ThumbOpposition))
        .count();
    let contact = samples
        .iter()
        .any(|sample| sample.contacts().iter().any(|value| value.pressure() > 0));
    if capability == BodyCapability::GazeContingency {
        return gaze_contingency_verdict(gaze, gaze_visual_consequences);
    }
    let passed = match capability {
        BodyCapability::GazeContingency => unreachable!("handled above"),
        BodyCapability::GazeControl => gaze >= 2,
        BodyCapability::BinocularDepth => unreachable!("handled before movement flattening"),
        BodyCapability::HandContingency => hand >= 1,
        BodyCapability::DigitSeparation => unreachable!("handled before movement flattening"),
        BodyCapability::SelfWorld => gaze >= 1 && hand >= 1,
        BodyCapability::Contact => contact,
        BodyCapability::VisualReach => contact && gaze >= 1 && palm >= 1,
        BodyCapability::TapHoldRelease => {
            tap_hold_release_completed(world_observations, BodyWorldCause::Organism)
        }
        BodyCapability::DragOpposition => contact && palm >= 1 && fingers >= 1 && opposition >= 1,
    };
    if passed {
        BodyVerdict::Passed
    } else {
        BodyVerdict::Failed
    }
}

fn tap_hold_release_completed(
    world_observations: &[BodyWorldObservation],
    cause: BodyWorldCause,
) -> bool {
    world_observations
        .iter()
        .enumerate()
        .flat_map(|(step, observation)| {
            observation.events.iter().filter_map(move |event| match event {
                BodyWorldEvent {
                    cause: event_cause,
                    event: DeviceEvent::KeyPressed { key },
                } if *event_cause == cause => Some((step, *key)),
                _ => None,
            })
        })
        .any(|(pressed_at, key)| {
            let released_at = world_observations
                .iter()
                .enumerate()
                .skip(pressed_at + 1)
                .find_map(|(released_at, observation)| {
                    observation
                        .events
                        .iter()
                        .any(|event| {
                            event.cause == cause
                                && matches!(event.event, DeviceEvent::KeyReleased { key: released } if released == key)
                        })
                        .then_some(released_at)
                });
            released_at.is_some_and(|released_at| {
                world_observations
                    .iter()
                    .take(released_at)
                    .skip(pressed_at + 1)
                    .any(|observation| {
                        observation.events.iter().any(|event| {
                            event.cause == cause
                                && matches!(event.event, DeviceEvent::LongPressActivated { key: activated } if activated == key)
                        })
                    })
            })
        })
}

fn key_press_completed(world_observations: &[BodyWorldObservation], cause: BodyWorldCause) -> bool {
    world_observations
        .iter()
        .flat_map(|observation| &observation.events)
        .any(|event| event.cause == cause && matches!(event.event, DeviceEvent::KeyPressed { .. }))
}

fn binocular_alignment_consequences(
    samples: &[WorldSample],
    observations: &[WorkstationStepObservation],
) -> usize {
    samples
        .windows(2)
        .zip(observations)
        .filter(|(_, observation)| opposing_horizontal_eye_movement(&observation.movements))
        .filter(|(frames, _)| has_stereo_target(&frames[0]))
        .filter(|(frames, _)| {
            Eye::ALL
                .into_iter()
                .all(|eye| target_alignment_improved(frames[0].eye(eye), frames[1].eye(eye)))
        })
        .count()
}

fn binocular_depth_verdict(
    opposing_eye_movements: usize,
    binocular_alignment_consequences: usize,
) -> BodyVerdict {
    if opposing_eye_movements >= 2 && binocular_alignment_consequences >= 2 {
        BodyVerdict::Passed
    } else {
        BodyVerdict::Failed
    }
}

fn has_stereo_target(sample: &WorldSample) -> bool {
    matches!(
        (
            target_horizontal(sample.eye(Eye::Left)),
            target_horizontal(sample.eye(Eye::Right))
        ),
        (Some(left), Some(right)) if left != right
    )
}

fn target_horizontal(field: &LightField) -> Option<i16> {
    let column =
        field.pixels().iter().position(|pixel| *pixel == 255)? % usize::from(field.width());
    let denominator = i32::from(field.width().saturating_sub(1));
    if denominator == 0 {
        return Some(0);
    }
    let body_x =
        i32::try_from(column).ok()? * i32::from(truelearner_workstation::BODY_MAX) / denominator;
    i16::try_from(body_x).ok()
}

fn target_alignment_improved(before: &LightField, after: &LightField) -> bool {
    let center = (truelearner_workstation::BODY_MAX + 1) / 2;
    matches!(
        (target_horizontal(before), target_horizontal(after)),
        (Some(before), Some(after)) if after.abs_diff(center) < before.abs_diff(center)
    )
}

fn has_digit_separation<'a>(steps: impl IntoIterator<Item = &'a [BodyMovement]>) -> bool {
    let mut isolated = Vec::with_capacity(2);
    for movements in steps {
        let fingers = movements
            .iter()
            .filter(|movement| movement.changed)
            .filter_map(|movement| match movement.axis {
                axis @ BodyAxis::FingerFlexion { .. } => Some(axis),
                _ => None,
            })
            .collect::<Vec<_>>();
        if let [finger] = fingers.as_slice() {
            if !isolated.contains(finger) {
                isolated.push(*finger);
            }
        }
    }
    isolated.len() >= 2
}

fn gaze_visual_consequences(
    samples: &[WorldSample],
    observations: &[WorkstationStepObservation],
) -> usize {
    samples
        .windows(2)
        .zip(observations)
        .filter(|(frames, observation)| {
            Eye::ALL.into_iter().any(|eye| {
                observation
                    .movements
                    .iter()
                    .any(|movement| movement.changed && moves_eye(movement.axis, eye))
                    && frames[0].eye(eye) != frames[1].eye(eye)
            })
        })
        .count()
}

fn gaze_contingency_verdict(net_gaze_movements: usize, visual_consequences: usize) -> BodyVerdict {
    if net_gaze_movements >= 2 && visual_consequences >= 2 {
        BodyVerdict::Passed
    } else {
        BodyVerdict::Failed
    }
}

fn is_gaze(axis: BodyAxis) -> bool {
    matches!(
        axis,
        BodyAxis::EyeHorizontal { .. } | BodyAxis::EyeVertical { .. }
    )
}

fn moves_eye(axis: BodyAxis, eye: Eye) -> bool {
    matches!(
        axis,
        BodyAxis::EyeHorizontal { eye: moved } | BodyAxis::EyeVertical { eye: moved }
            if moved == eye
    )
}

fn opposing_horizontal_eye_movement(movements: &[BodyMovement]) -> bool {
    let velocity = |eye| {
        movements
            .iter()
            .find(|movement| movement.changed && movement.axis == BodyAxis::EyeHorizontal { eye })
            .map(|movement| movement.velocity)
    };
    matches!(
        (velocity(Eye::Left), velocity(Eye::Right)),
        (Some(left), Some(right)) if left.signum() == -right.signum()
    )
}

#[derive(Debug)]
pub enum BodyCourseError {
    Workstation(WorkstationError),
    ExternalWorld(WorldError),
    Prerequisite(BodyCapability),
    Io(String),
    Serialization(String),
    OutputExists(String),
}

impl fmt::Display for BodyCourseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workstation(error) => write!(formatter, "workstation Harness failed: {error}"),
            Self::ExternalWorld(error) => write!(formatter, "external world failed: {error}"),
            Self::Prerequisite(capability) => {
                write!(
                    formatter,
                    "body capability prerequisite is missing for {capability:?}"
                )
            }
            Self::Io(message) => write!(formatter, "body course I/O failed: {message}"),
            Self::Serialization(message) => {
                write!(formatter, "body course serialization failed: {message}")
            }
            Self::OutputExists(path) => {
                write!(formatter, "body course output already exists: {path}")
            }
        }
    }
}

impl std::error::Error for BodyCourseError {}

impl From<WorkstationError> for BodyCourseError {
    fn from(value: WorkstationError) -> Self {
        Self::Workstation(value)
    }
}

impl From<WorldError> for BodyCourseError {
    fn from(value: WorldError) -> Self {
        Self::ExternalWorld(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use truelearner_workstation::{
        verify_choice_laws, BodyControl, BodyTraceEvent, ChoiceBasis, Digit, Direction,
    };

    #[test]
    fn prerequisites_are_external_and_acyclic() {
        for (index, capability) in BodyCapability::ORDER.into_iter().enumerate() {
            assert!(capability
                .prerequisites()
                .iter()
                .all(|required| { BodyCapability::ORDER[..index].contains(required) }));
        }
    }

    #[test]
    fn contact_trace_preserves_every_choice_arrow() {
        let seed = 31_001;
        let mut course = BodyCourse::new(seed).unwrap();
        for capability in BodyCapability::ORDER[..6].iter().copied() {
            let capability_index = BodyCapability::ORDER
                .iter()
                .position(|candidate| *candidate == capability)
                .unwrap();
            let development_seed = seed + capability_index as u64 * 10 + 1;
            let development = course
                .experience(
                    capability,
                    BodyExperienceMode::Development,
                    development_seed,
                )
                .unwrap();
            assert_ne!(development.verdict, BodyVerdict::MissingExploration);
            let probe = course
                .experience(
                    capability,
                    BodyExperienceMode::Probe,
                    development_seed + 1_000_000,
                )
                .unwrap();
            assert_eq!(
                probe.verdict,
                BodyVerdict::Passed,
                "candidate regressed {capability:?}"
            );
            course.acquired.insert(capability);
        }

        let checkpoint = truelearner_workstation::WorkstationCheckpoint::decode(
            &course.checkpoint_bytes().unwrap(),
        )
        .unwrap();
        let mut harness = WorkstationHarness::restore(checkpoint).unwrap();
        let mut world = FlatWorld::generated(seed + 61, BodyCapability::Contact);
        let contact_steps =
            steps_per_experience(BodyCapability::Contact, BodyExperienceMode::Development);
        let mut steps = Vec::with_capacity(contact_steps);
        for _ in 0..contact_steps {
            let sample = world.sample(&harness.read().unwrap()).unwrap();
            let (observation, trace) = harness.step_traced(sample).unwrap();
            verify_choice_laws(&trace).unwrap();
            steps.push((observation, trace));
        }

        assert_eq!(steps.len(), contact_steps);
        assert!(steps.iter().all(|(observation, _)| {
            observation.movements.iter().any(|movement| {
                movement.axis == BodyAxis::PalmDepth && movement.changed && movement.net_impulse > 0
            })
        }));

        let development = course
            .experience(
                BodyCapability::Contact,
                BodyExperienceMode::Development,
                seed + 61,
            )
            .unwrap();
        assert_ne!(development.verdict, BodyVerdict::MissingExploration);
        let probe = course
            .experience(
                BodyCapability::Contact,
                BodyExperienceMode::Probe,
                seed + 1_000_061,
            )
            .unwrap();
        assert_eq!(probe.samples.len(), CONTACT_STEPS_PER_EXPERIENCE);
        assert_eq!(probe.verdict, BodyVerdict::Passed);
        assert!(probe.durable_unchanged);
        assert!(probe.replay_exact);
        assert!(probe.naturally_quiescent);
        assert!(probe.samples.iter().any(|sample| sample
            .contacts()
            .iter()
            .any(|contact| contact.pressure() > 0)));
    }

    #[test]
    fn world_ancestry_closure_and_local_release_compose_tap_hold_release() {
        let seed = 31_001;
        let mut course = BodyCourse::new(seed).unwrap();
        for capability in BodyCapability::ORDER[..8].iter().copied() {
            let capability_index = BodyCapability::ORDER
                .iter()
                .position(|candidate| *candidate == capability)
                .unwrap();
            let development_seed = seed + capability_index as u64 * 10 + 1;
            let development = course
                .experience(
                    capability,
                    BodyExperienceMode::Development,
                    development_seed,
                )
                .unwrap();
            assert!(
                !matches!(
                    development.verdict,
                    BodyVerdict::MissingExploration | BodyVerdict::BudgetExceeded
                ),
                "development stopped at {capability:?}"
            );
            let probe = course
                .experience(
                    capability,
                    BodyExperienceMode::Probe,
                    development_seed + 1_000_000,
                )
                .unwrap();
            assert_eq!(
                probe.verdict,
                BodyVerdict::Passed,
                "prerequisite probe failed at {capability:?}"
            );
            course.acquired.insert(capability);
        }

        let checkpoint = truelearner_workstation::WorkstationCheckpoint::decode(
            &course.checkpoint_bytes().unwrap(),
        )
        .unwrap();
        let development_seed = seed + 8 * 10 + 1;
        let lesson_pose = checkpoint.clone();
        let mut soft_harness = WorkstationHarness::restore(checkpoint.clone()).unwrap();
        let mut soft_world = ExperienceWorld::generated(
            development_seed + 875_000,
            BodyCapability::TapHoldRelease,
            BodyExperienceMode::DepthControl,
            Some((640, DEPTH_CONTROL_RELEASE_DEPTH)),
        )
        .unwrap();
        let mut soft_closed = false;
        let mut soft_choice_checked = false;
        let mut soft_boundary_parents = Vec::new();
        let mut soft_progress_parents = Vec::new();
        for _ in 0..3 {
            let closed_before_choice = soft_closed;
            let sample = soft_world.sample(&soft_harness.read().unwrap()).unwrap();
            let (observation, trace) = soft_harness
                .step_traced_with_causal_parents(
                    sample,
                    &soft_boundary_parents,
                    &soft_progress_parents,
                )
                .unwrap();
            verify_choice_laws(&trace).unwrap();
            let world_observation = soft_world.advance(&observation).unwrap();
            soft_boundary_parents.clone_from(&world_observation.boundary_parents);
            soft_progress_parents.clone_from(&world_observation.progress_parents);
            let before_depth = Digit::ALL
                .into_iter()
                .map(|digit| observation.state_before.hand().fingertip(digit).depth())
                .max()
                .unwrap();
            if before_depth == 640 {
                assert!(closed_before_choice);
                let candidates = trace
                    .iter()
                    .filter_map(|event| match event {
                        BodyTraceEvent::Candidate(candidate) => soft_harness
                            .control_for_trace_output(candidate.path.output)
                            .map(|control| (control, candidate)),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let forward = BodyControl::PalmDepth {
                    direction: Direction::Increase,
                };
                let forward_candidate = candidates
                    .iter()
                    .filter_map(|(control, candidate)| {
                        (*control == forward && candidate.participation > 0).then_some(*candidate)
                    })
                    .max_by_key(|candidate| (candidate.participation, candidate.strength))
                    .expect("soft closure retains the witnessed forward path");
                assert!(forward_candidate.executable);
                assert!(forward_candidate.resisted_progress);
                assert!(!forward_candidate.boundary_open);
                assert!(forward_candidate.boundary_inhibited);
                assert!(forward_candidate.outcome.is_none());
                assert!(forward_candidate.participation > 0);
                assert!(forward_candidate.strength > 1);
                let relevant_choice = trace
                    .iter()
                    .find_map(|event| match event {
                        BodyTraceEvent::Choice(choice)
                            if choice.group == forward_candidate.group =>
                        {
                            Some(choice)
                        }
                        _ => None,
                    })
                    .expect("soft forward candidate's choice is recorded");
                assert_eq!(relevant_choice.basis, Some(ChoiceBasis::BoundaryRelease));
                assert_eq!(
                    relevant_choice
                        .winner
                        .and_then(|path| soft_harness.control_for_trace_output(path.output)),
                    Some(BodyControl::PalmDepth {
                        direction: Direction::Decrease,
                    })
                );
                assert!(!observation
                    .crossings
                    .iter()
                    .any(|effect| effect.control == forward));
                soft_choice_checked = true;
            }
            soft_closed |= world_observation
                .events
                .iter()
                .any(|event| matches!(event.event, DeviceEvent::KeyPressed { .. }));
        }
        assert!(soft_choice_checked);

        let mut harness = WorkstationHarness::restore(checkpoint).unwrap();
        let mut world = ExperienceWorld::generated(
            development_seed + 875_000,
            BodyCapability::TapHoldRelease,
            BodyExperienceMode::DepthControl,
            Some((656, DEPTH_CONTROL_RELEASE_DEPTH)),
        )
        .unwrap();

        let mut first_progress_checked = false;
        let mut boundary_parents = Vec::new();
        let mut progress_parents = Vec::new();
        let mut boundary_return_closed = false;
        for _ in 0..DEPTH_CONTROL_STEPS_PER_EXPERIENCE {
            let sample = world.sample(&harness.read().unwrap()).unwrap();
            let had_boundary_parents = !boundary_parents.is_empty();
            let (observation, trace) = harness
                .step_traced_with_causal_parents(sample, &boundary_parents, &progress_parents)
                .unwrap();
            verify_choice_laws(&trace).unwrap();
            let world_observation = world.advance(&observation).unwrap();
            if had_boundary_parents {
                boundary_return_closed |= trace.iter().any(|event| {
                    matches!(
                        event,
                        BodyTraceEvent::Return(returned)
                            if returned.decision
                                == truelearner_workstation::BodyReturnDecision::Accepted
                    )
                });
            }
            boundary_parents.clone_from(&world_observation.boundary_parents);
            progress_parents.clone_from(&world_observation.progress_parents);
            let before_depth = Digit::ALL
                .into_iter()
                .map(|digit| observation.state_before.hand().fingertip(digit).depth())
                .max()
                .unwrap();
            let depth = Digit::ALL
                .into_iter()
                .map(|digit| observation.state_after.hand().fingertip(digit).depth())
                .max()
                .unwrap();
            let candidates = trace
                .iter()
                .filter_map(|event| match event {
                    BodyTraceEvent::Candidate(candidate) => harness
                        .control_for_trace_output(candidate.path.output)
                        .map(|control| (control, candidate)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let choices = trace
                .iter()
                .filter_map(|event| match event {
                    BodyTraceEvent::Choice(choice) => Some(choice),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if before_depth == 640 && !first_progress_checked {
                let forward = BodyControl::PalmDepth {
                    direction: Direction::Increase,
                };
                let forward_candidate = candidates
                    .iter()
                    .filter_map(|(control, candidate)| {
                        (*control == forward
                            && candidate.unanswered
                            && candidate.output_participated
                            && candidate.participation > 0)
                            .then_some(*candidate)
                    })
                    .max_by_key(|candidate| (candidate.participation, candidate.strength))
                    .expect("unanswered forward candidate remains physically present");
                assert!(forward_candidate.executable);
                assert!(forward_candidate.boundary_open);
                assert_eq!(forward_candidate.return_cause, Some(102));
                assert_eq!(forward_candidate.participation, 38);
                assert_eq!(forward_candidate.strength, 38);
                assert_eq!(forward_candidate.drive, 16);
                assert!(forward_candidate.resisted_progress);
                assert!(forward_candidate.outcome.is_some());

                let choice = choices
                    .iter()
                    .find(|choice| choice.group == forward_candidate.group)
                    .expect("forward candidate's choice group is recorded");
                assert_eq!(choice.basis, Some(ChoiceBasis::RetainedProgress));
                let winner_path = choice.winner.expect("choice has a physical winner");
                let (winner_control, winner_candidate) = candidates
                    .iter()
                    .find(|(_, candidate)| candidate.path == winner_path)
                    .copied()
                    .expect("winner is one of the recorded candidates");
                assert_eq!(winner_control, forward);
                assert!(winner_candidate.executable);
                assert!(winner_candidate.output_participated);
                assert!(winner_candidate.resisted_progress);
                assert_eq!(winner_candidate.participation, 38);
                assert_eq!(winner_candidate.strength, 38);
                assert_eq!(winner_candidate.drive, 16);
                assert!(observation
                    .returned_transitions
                    .contains(&BodyAxis::PalmDepth));
                assert!(observation
                    .crossings
                    .iter()
                    .any(|effect| effect.control == forward));
                assert!(world_observation
                    .events
                    .iter()
                    .any(|event| matches!(event.event, DeviceEvent::KeyPressed { .. })));
                assert_eq!(depth, 656);
                first_progress_checked = true;
            }
        }
        assert!(first_progress_checked);
        assert!(boundary_return_closed);

        let practice = course
            .experience(
                BodyCapability::TapHoldRelease,
                BodyExperienceMode::Development,
                development_seed,
            )
            .unwrap();
        assert_eq!(practice.verdict, BodyVerdict::Passed);
        assert!(practice
            .observations
            .last()
            .is_some_and(|observation| !observation.opportunity_admitted));
        course
            .harness
            .reposition_from_checkpoint(&lesson_pose)
            .unwrap();
        let probe = course
            .experience(
                BodyCapability::TapHoldRelease,
                BodyExperienceMode::Probe,
                development_seed + 1_000_000,
            )
            .unwrap();
        assert_eq!(probe.verdict, BodyVerdict::Passed);
        assert!(probe.durable_unchanged);
        assert!(probe.replay_exact);
        assert!(tap_hold_release_completed(
            &probe.world_observations,
            BodyWorldCause::Organism
        ));
    }

    #[test]
    fn courses_partition_the_capability_order_exactly() {
        let flattened = BodyCourseKind::ORDER
            .into_iter()
            .flat_map(BodyCourseKind::capabilities)
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(flattened, BodyCapability::ORDER);
        assert!(BodyCourseKind::ORDER
            .into_iter()
            .all(|course| !course.capabilities().is_empty()));
    }

    #[test]
    fn passive_world_change_is_not_self_movement() {
        let harness = WorkstationHarness::new(11).unwrap();
        let mut world = FlatWorld::passive(12);
        let first = world.sample(&harness.read().unwrap()).unwrap();
        let second = world.sample(&harness.read().unwrap()).unwrap();
        assert_ne!(first, second);
        assert_eq!(
            evaluate(BodyCapability::SelfWorld, &[first, second], &[], &[]),
            BodyVerdict::MissingExploration
        );
    }

    #[test]
    fn direction_is_not_an_evaluator_verdict() {
        assert_ne!(
            truelearner_workstation::Direction::Decrease,
            truelearner_workstation::Direction::Increase
        );
    }

    #[test]
    fn equal_opposing_effort_is_not_credited_as_movement() {
        let canceled = BodyMovement {
            axis: BodyAxis::EyeHorizontal { eye: Eye::Left },
            decrease_effort: 2,
            increase_effort: 2,
            net_impulse: 0,
            velocity: 0,
            changed: false,
        };
        assert_eq!(
            evaluate_physical(BodyCapability::GazeContingency, &[canceled], &[], 0, &[]),
            BodyVerdict::MissingExploration
        );
    }

    fn caused_world_observation(
        cause: BodyWorldCause,
        events: Vec<DeviceEvent>,
    ) -> BodyWorldObservation {
        BodyWorldObservation {
            events: events
                .into_iter()
                .map(|event| BodyWorldEvent { cause, event })
                .collect(),
            boundary_parents: Vec::new(),
            progress_parents: Vec::new(),
            fingerprint: None,
        }
    }

    fn world_observation(events: Vec<DeviceEvent>) -> BodyWorldObservation {
        caused_world_observation(BodyWorldCause::Organism, events)
    }

    #[test]
    fn tap_hold_release_requires_an_intervening_held_step() {
        let completed = [
            world_observation(vec![DeviceEvent::KeyPressed { key: 17 }]),
            world_observation(Vec::new()),
            world_observation(vec![DeviceEvent::LongPressActivated { key: 17 }]),
            world_observation(vec![DeviceEvent::KeyReleased { key: 17 }]),
        ];
        assert!(tap_hold_release_completed(
            &completed,
            BodyWorldCause::Organism
        ));

        let immediate = [
            world_observation(vec![DeviceEvent::KeyPressed { key: 17 }]),
            world_observation(vec![DeviceEvent::KeyReleased { key: 17 }]),
        ];
        assert!(!tap_hold_release_completed(
            &immediate,
            BodyWorldCause::Organism
        ));
    }

    #[test]
    fn tap_hold_release_requires_the_same_physical_key() {
        let different_keys = [
            world_observation(vec![DeviceEvent::KeyPressed { key: 17 }]),
            world_observation(vec![DeviceEvent::LongPressActivated { key: 17 }]),
            world_observation(vec![DeviceEvent::KeyReleased { key: 18 }]),
        ];
        assert!(!tap_hold_release_completed(
            &different_keys,
            BodyWorldCause::Organism
        ));
    }

    #[test]
    fn demonstrated_events_cannot_satisfy_organism_capability() {
        let demonstrated = [
            caused_world_observation(
                BodyWorldCause::Demonstrator,
                vec![DeviceEvent::KeyPressed { key: 17 }],
            ),
            caused_world_observation(
                BodyWorldCause::Demonstrator,
                vec![DeviceEvent::LongPressActivated { key: 17 }],
            ),
            caused_world_observation(
                BodyWorldCause::Demonstrator,
                vec![DeviceEvent::KeyReleased { key: 17 }],
            ),
        ];
        assert!(tap_hold_release_completed(
            &demonstrated,
            BodyWorldCause::Demonstrator
        ));
        assert!(!tap_hold_release_completed(
            &demonstrated,
            BodyWorldCause::Organism
        ));
    }

    #[test]
    fn demonstration_is_visible_exact_and_externally_attributed() {
        let mut course = BodyCourse::new(62_001).unwrap();
        course.acquired.insert(BodyCapability::VisualReach);
        let experience = course
            .experience(
                BodyCapability::TapHoldRelease,
                BodyExperienceMode::Demonstration,
                62_002,
            )
            .unwrap();

        assert_eq!(experience.verdict, BodyVerdict::Presented);
        assert_eq!(experience.samples.len(), DEMONSTRATION_STEPS_PER_EXPERIENCE);
        assert!(experience.replay_exact);
        assert_eq!(experience.key_press_depth, Some(720));
        assert!(experience
            .world_observations
            .iter()
            .flat_map(|observation| &observation.events)
            .all(|event| event.cause == BodyWorldCause::Demonstrator));
        assert!(experience.samples.windows(2).any(|frames| Eye::ALL
            .into_iter()
            .any(|eye| frames[0].eye(eye) != frames[1].eye(eye))));
        let wire = serde_json::to_string(&experience.samples).unwrap();
        for forbidden in [
            "demonstrator",
            "cause",
            "key",
            "long_press",
            "capability",
            "expected",
            "action",
        ] {
            assert!(!wire.contains(forbidden), "leaked {forbidden}: {wire}");
        }
    }

    #[test]
    fn tap_hold_release_does_not_credit_contact_or_depth_motion_alone() {
        assert_eq!(
            evaluate_physical(
                BodyCapability::TapHoldRelease,
                &[changed(BodyAxis::PalmDepth)],
                &[],
                0,
                &[],
            ),
            BodyVerdict::Failed
        );
    }

    #[test]
    fn tap_hold_release_replay_rejects_changed_world_evidence() {
        let mut course = BodyCourse::new(61_001).unwrap();
        course.acquired.insert(BodyCapability::VisualReach);
        let experience = course
            .experience(
                BodyCapability::TapHoldRelease,
                BodyExperienceMode::Probe,
                61_002,
            )
            .unwrap();
        assert!(experience.replay_exact);

        let mut changed_world = experience.world_observations.clone();
        changed_world[0].fingerprint = Some("changed-world".to_string());
        assert!(!replay(
            experience.capability,
            experience.mode,
            experience.seed,
            None,
            ReplayEvidence {
                checkpoint_before: &experience.checkpoint_before,
                samples: &experience.samples,
                observations: &experience.observations,
                world_observations: &changed_world,
                checkpoint_after: &experience.checkpoint_after,
            },
        )
        .unwrap());
    }

    #[test]
    fn gaze_contingency_requires_repeated_visual_consequences() {
        assert_eq!(
            gaze_contingency_verdict(1, 1),
            BodyVerdict::Failed,
            "one accidental movement is not a contingency"
        );
        assert_eq!(gaze_contingency_verdict(3, 0), BodyVerdict::Failed);
        assert_eq!(gaze_contingency_verdict(2, 2), BodyVerdict::Passed);
    }

    #[test]
    fn binocular_depth_requires_repeated_opposing_alignment_consequences() {
        assert_eq!(binocular_depth_verdict(0, 0), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(2, 0), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(1, 1), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(3, 1), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(2, 2), BodyVerdict::Passed);
    }

    #[test]
    fn binocular_alignment_is_measured_in_successive_retinal_frames() {
        let target_at = |column: usize| {
            let mut pixels = vec![0; 9];
            pixels[column] = 255;
            LightField::new(9, 1, pixels).unwrap()
        };

        assert!(target_alignment_improved(&target_at(1), &target_at(2)));
        assert!(!target_alignment_improved(&target_at(1), &target_at(1)));
        assert!(!target_alignment_improved(&target_at(1), &target_at(0)));
    }

    fn changed(axis: BodyAxis) -> BodyMovement {
        BodyMovement {
            axis,
            decrease_effort: 0,
            increase_effort: 1,
            net_impulse: 1,
            velocity: 1,
            changed: true,
        }
    }

    #[test]
    fn digit_separation_requires_two_distinct_isolated_fingers() {
        let all_together = truelearner_workstation::Digit::ALL
            .into_iter()
            .map(|digit| changed(BodyAxis::FingerFlexion { digit }))
            .collect::<Vec<_>>();
        assert!(!has_digit_separation([all_together.as_slice()]));

        let thumb = changed(BodyAxis::FingerFlexion {
            digit: truelearner_workstation::Digit::Thumb,
        });
        assert!(!has_digit_separation([&[thumb][..], &[thumb][..]]));

        let index = changed(BodyAxis::FingerFlexion {
            digit: truelearner_workstation::Digit::Index,
        });
        assert!(has_digit_separation([&[thumb][..], &[index][..]]));

        let two_together = [thumb, index];
        assert!(!has_digit_separation([two_together.as_slice()]));
    }

    #[test]
    fn binocular_coordination_requires_both_eyes_in_opposite_directions() {
        let left = changed(BodyAxis::EyeHorizontal { eye: Eye::Left });
        let mut right = changed(BodyAxis::EyeHorizontal { eye: Eye::Right });
        right.net_impulse = -1;
        right.velocity = -1;
        assert!(opposing_horizontal_eye_movement(&[left, right]));
        assert!(!opposing_horizontal_eye_movement(&[left]));
        assert!(!opposing_horizontal_eye_movement(&[
            left,
            changed(BodyAxis::EyeHorizontal { eye: Eye::Right })
        ]));
    }
}
