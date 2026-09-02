use crate::world::FlatWorld;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;
use truelearner_workstation::{
    BodyAxis, BodyControl, BodyMovement, Eye, LightField, WorkstationError, WorkstationHarness,
    WorkstationStepObservation, WorldSample,
};

const STEPS_PER_EXPERIENCE: usize = 12;
const PHYSICAL_WORK_BOUND: u64 = 2_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyCapability {
    GazeContingency,
    GazeControl,
    BinocularDepth,
    HandContingency,
    SelfWorld,
}

impl BodyCapability {
    pub const ORDER: [Self; 5] = [
        Self::GazeContingency,
        Self::GazeControl,
        Self::BinocularDepth,
        Self::HandContingency,
        Self::SelfWorld,
    ];

    pub fn prerequisites(self) -> &'static [Self] {
        match self {
            Self::GazeContingency | Self::HandContingency => &[],
            Self::GazeControl => &[Self::GazeContingency],
            Self::BinocularDepth => &[Self::GazeControl],
            Self::SelfWorld => &[Self::BinocularDepth, Self::HandContingency],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyCourseKind {
    EyeControl,
    HandControl,
    EyeHandCoordination,
}

impl BodyCourseKind {
    pub const ORDER: [Self; 3] = [
        Self::EyeControl,
        Self::HandControl,
        Self::EyeHandCoordination,
    ];

    pub const fn capabilities(self) -> &'static [BodyCapability] {
        match self {
            Self::EyeControl => &[
                BodyCapability::GazeContingency,
                BodyCapability::GazeControl,
                BodyCapability::BinocularDepth,
            ],
            Self::HandControl => &[BodyCapability::HandContingency],
            Self::EyeHandCoordination => &[BodyCapability::SelfWorld],
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
    Development,
    Probe,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyVerdict {
    Passed,
    Failed,
    MissingExploration,
    BudgetExceeded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyEvidenceState {
    Unknown,
    Emerging,
    Acquired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyCapabilityEvidence {
    pub capability: BodyCapability,
    pub state: BodyEvidenceState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyPerturbation {
    pub control: BodyControl,
    pub impulse: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyExperience {
    pub id: String,
    pub capability: BodyCapability,
    pub mode: BodyExperienceMode,
    pub seed: u64,
    pub samples: Vec<WorldSample>,
    pub observations: Vec<WorkstationStepObservation>,
    pub perturbation: Option<BodyPerturbation>,
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
pub struct CourseRun {
    pub schema_version: u16,
    pub seed: u64,
    pub courses: Vec<BodyCourseProgress>,
    pub acquired: Vec<BodyCapability>,
    pub capability_evidence: Vec<BodyCapabilityEvidence>,
    pub first_failure: Option<BodyCapability>,
    pub experiences: Vec<BodyExperience>,
    pub exact_replay: bool,
    pub final_body_fingerprint: String,
    #[serde(skip)]
    pub body_checkpoint: Vec<u8>,
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
        self.experience_with_setup(capability, mode, seed, None)
    }

    #[cfg(test)]
    fn experience_with_perturbation(
        &mut self,
        capability: BodyCapability,
        mode: BodyExperienceMode,
        seed: u64,
        perturbation: BodyPerturbation,
    ) -> Result<BodyExperience, BodyCourseError> {
        self.experience_with_setup(capability, mode, seed, Some(perturbation))
    }

    fn experience_with_setup(
        &mut self,
        capability: BodyCapability,
        mode: BodyExperienceMode,
        seed: u64,
        perturbation: Option<BodyPerturbation>,
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
        if let Some(perturbation) = perturbation {
            if !working.perturb_body(perturbation.control, perturbation.impulse)? {
                return Err(BodyCourseError::Workstation(
                    WorkstationError::InvalidCheckpoint,
                ));
            }
        }
        let mut world = FlatWorld::generated(seed, capability);
        let mut samples = Vec::with_capacity(STEPS_PER_EXPERIENCE);
        let mut observations = Vec::with_capacity(STEPS_PER_EXPERIENCE);
        let mut physical_work = 0_u64;
        let mut plasticity_updates = 0_u64;
        let mut naturally_quiescent = true;
        let mut budget_exceeded = false;
        for _ in 0..STEPS_PER_EXPERIENCE {
            let sample = world.sample(&working.read()?)?;
            let observation = working.step_with_causal_parents(sample.clone(), &[], &[])?;
            physical_work = physical_work.saturating_add(observation.metrics.physical_work);
            plasticity_updates =
                plasticity_updates.saturating_add(observation.metrics.plasticity_updates);
            naturally_quiescent &= observation.naturally_quiescent;
            samples.push(sample);
            observations.push(observation);
            if physical_work > PHYSICAL_WORK_BOUND {
                budget_exceeded = true;
                break;
            }
        }
        let checkpoint_after = working.save()?.canonical_bytes()?;
        let mut verdict = evaluate(capability, &samples, &observations);
        if budget_exceeded {
            verdict = BodyVerdict::BudgetExceeded;
        }
        if !naturally_quiescent {
            verdict = BodyVerdict::Failed;
        }
        let replay_exact = replay(
            capability,
            seed,
            perturbation,
            ReplayEvidence {
                checkpoint_before: &durable_before,
                samples: &samples,
                observations: &observations,
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
            perturbation,
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

    /// Teaches the pointer-body courses: eye control, hand control, and
    /// eye-hand coordination.
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
        let capability_evidence = capability_evidence(&self.experiences, &self.acquired);
        let body_checkpoint = self.checkpoint_bytes()?;
        let final_body_fingerprint = self.harness.read()?.body_fingerprint;
        let exact_replay = self
            .experiences
            .iter()
            .all(|experience| experience.replay_exact);
        Ok(CourseRun {
            schema_version: 15,
            seed: self.seed,
            courses,
            acquired: self.acquired.iter().copied().collect(),
            capability_evidence,
            first_failure,
            exact_replay,
            experiences: self.experiences,
            final_body_fingerprint,
            body_checkpoint,
        })
    }

    fn restore_checkpoint(&mut self, bytes: &[u8]) -> Result<(), BodyCourseError> {
        let checkpoint = truelearner_workstation::WorkstationCheckpoint::decode(bytes)?;
        self.harness = WorkstationHarness::restore(checkpoint)?;
        Ok(())
    }
}

const fn commits_learning(mode: BodyExperienceMode) -> bool {
    matches!(mode, BodyExperienceMode::Development)
}

struct ReplayEvidence<'a> {
    checkpoint_before: &'a [u8],
    samples: &'a [WorldSample],
    observations: &'a [WorkstationStepObservation],
    checkpoint_after: &'a [u8],
}

fn replay(
    capability: BodyCapability,
    seed: u64,
    perturbation: Option<BodyPerturbation>,
    evidence: ReplayEvidence<'_>,
) -> Result<bool, BodyCourseError> {
    if evidence.samples.len() != evidence.observations.len() {
        return Ok(false);
    }
    let checkpoint =
        truelearner_workstation::WorkstationCheckpoint::decode(evidence.checkpoint_before)?;
    let mut harness = WorkstationHarness::restore(checkpoint)?;
    if let Some(perturbation) = perturbation {
        if !harness.perturb_body(perturbation.control, perturbation.impulse)? {
            return Ok(false);
        }
    }
    let mut world = FlatWorld::generated(seed, capability);
    for (expected_sample, expected_observation) in
        evidence.samples.iter().zip(evidence.observations.iter())
    {
        let sample = world.sample(&harness.read()?)?;
        if &sample != expected_sample {
            return Ok(false);
        }
        let observation = harness.step_with_causal_parents(sample, &[], &[])?;
        if &observation != expected_observation {
            return Ok(false);
        }
    }
    Ok(harness.save()?.canonical_bytes()? == evidence.checkpoint_after)
}

fn evaluate(
    capability: BodyCapability,
    samples: &[WorldSample],
    observations: &[WorkstationStepObservation],
) -> BodyVerdict {
    if capability == BodyCapability::BinocularDepth {
        if !observations
            .iter()
            .flat_map(|observation| &observation.movements)
            .any(|movement| movement.changed)
        {
            return BodyVerdict::MissingExploration;
        }
        return binocular_depth_verdict(
            binocular_fusion_steps(samples),
            binocular_alignment_improvements(samples),
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
        gaze_visual_consequences(samples, observations),
    )
}

fn evaluate_physical(
    capability: BodyCapability,
    movements: &[BodyMovement],
    gaze_visual_consequences: usize,
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
    let passed = match capability {
        BodyCapability::GazeContingency => {
            return gaze_contingency_verdict(gaze, gaze_visual_consequences)
        }
        BodyCapability::GazeControl => gaze >= 2,
        BodyCapability::HandContingency => hand >= 1,
        BodyCapability::SelfWorld => gaze >= 1 && hand >= 1,
        BodyCapability::BinocularDepth => unreachable!("handled before movement flattening"),
    };
    if passed {
        BodyVerdict::Passed
    } else {
        BodyVerdict::Failed
    }
}

fn capability_evidence(
    experiences: &[BodyExperience],
    acquired: &BTreeSet<BodyCapability>,
) -> Vec<BodyCapabilityEvidence> {
    BodyCapability::ORDER
        .into_iter()
        .map(|capability| {
            let passed = |mode| {
                experiences.iter().any(|experience| {
                    experience.capability == capability
                        && experience.mode == mode
                        && experience.verdict == BodyVerdict::Passed
                        && experience.replay_exact
                        && experience.naturally_quiescent
                        && (commits_learning(mode) || experience.durable_unchanged)
                })
            };
            let state = if acquired.contains(&capability) {
                BodyEvidenceState::Acquired
            } else if passed(BodyExperienceMode::Development) {
                BodyEvidenceState::Emerging
            } else {
                BodyEvidenceState::Unknown
            };
            BodyCapabilityEvidence { capability, state }
        })
        .collect()
}

/// Steps in which both eyes hold their stereo targets on their foveas at
/// once: binocular fusion, the outcome the capability claims. The rung's
/// world places two distinct targets by construction, so seeing a target in
/// each eye is the given; the claim is both foveated at once. The birthright
/// vergence controller converges the eyes, often one eye per step, so the
/// honest claim is achieved and held fusion, not a same-step motion
/// signature.
fn binocular_fusion_steps(samples: &[WorldSample]) -> usize {
    let center = (truelearner_workstation::BODY_MAX + 1) / 2;
    samples
        .iter()
        .filter(|sample| {
            Eye::ALL.into_iter().all(|eye| {
                target_horizontal(sample.eye(eye)).is_some_and(|x| (x - center).abs() <= 128)
            })
        })
        .count()
}

/// Steps in which at least one eye's stereo alignment strictly improved:
/// active vergence, so fusion is reached by the body and not an accident of
/// a frozen pose.
fn binocular_alignment_improvements(samples: &[WorldSample]) -> usize {
    let center = (truelearner_workstation::BODY_MAX + 1) / 2;
    samples
        .windows(2)
        .filter(|frames| has_stereo_target(&frames[0]))
        .filter(|frames| {
            Eye::ALL.into_iter().any(|eye| {
                matches!(
                    (
                        target_horizontal(frames[0].eye(eye)),
                        target_horizontal(frames[1].eye(eye)),
                    ),
                    (Some(before), Some(after)) if (after - center).abs() < (before - center).abs()
                )
            })
        })
        .count()
}

fn binocular_depth_verdict(
    binocular_fusion_steps: usize,
    binocular_alignment_improvements: usize,
) -> BodyVerdict {
    if binocular_fusion_steps >= 2 && binocular_alignment_improvements >= 2 {
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

#[derive(Debug)]
pub enum BodyCourseError {
    Workstation(WorkstationError),
    Prerequisite(BodyCapability),
    Io(String),
    Serialization(String),
    OutputExists(String),
}

impl fmt::Display for BodyCourseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workstation(error) => write!(formatter, "workstation Harness failed: {error}"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use truelearner_workstation::{
        verify_choice_contract, BodyTraceEvent, ContactSample, Direction,
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
            evaluate(BodyCapability::SelfWorld, &[first, second], &[]),
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
            evaluate_physical(BodyCapability::GazeContingency, &[canceled], 0),
            BodyVerdict::MissingExploration
        );
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
    fn binocular_depth_requires_held_fusion_and_active_convergence() {
        assert_eq!(binocular_depth_verdict(0, 0), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(2, 0), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(1, 1), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(3, 1), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(2, 2), BodyVerdict::Passed);
    }

    #[test]
    fn a_frozen_pose_is_not_exploration() {
        // The pointer body has a birthright vergence controller, so fusion
        // can appear without any chosen movement. The exploration gate
        // still guards the claim: a frozen pose over stereo targets is
        // MissingExploration, not a demonstration of vergence.
        let target_at = |column: usize| {
            let mut pixels = vec![0_u8; 81];
            for row in 0..9 {
                pixels[row * 9 + column] = 255;
            }
            LightField::new(9, 9, pixels).unwrap()
        };
        let samples = (0..12)
            .map(|_| {
                WorldSample::new([target_at(4), target_at(4)], [ContactSample::default()]).unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            evaluate(BodyCapability::BinocularDepth, &samples, &[]),
            BodyVerdict::MissingExploration
        );
    }

    #[test]
    fn fusion_is_measured_in_simultaneous_foveal_frames() {
        let target_at = |column: usize| {
            let mut pixels = vec![0; 9];
            pixels[column] = 255;
            LightField::new(9, 1, pixels).unwrap()
        };
        let sample_with = |left: usize, right: usize| {
            WorldSample::new(
                [target_at(left), target_at(right)],
                [ContactSample::default()],
            )
            .unwrap()
        };

        // Both eyes hold their stereo targets on the fovea at once.
        let fused = sample_with(4, 4);
        assert_eq!(binocular_fusion_steps(std::slice::from_ref(&fused)), 1);
        // One receptor pitch off-center is still foveal at receptor
        // resolution; two pitches is not.
        assert_eq!(binocular_fusion_steps(&[sample_with(4, 5)]), 1);
        let lagging = sample_with(4, 6);
        assert_eq!(binocular_fusion_steps(std::slice::from_ref(&lagging)), 0);
        // Convergence in action counts an improvement step, fusion or not.
        let steps = [lagging, fused];
        assert_eq!(binocular_alignment_improvements(&steps), 1);
        assert_eq!(binocular_fusion_steps(&steps), 1);
    }

    #[test]
    fn replay_rejects_tampered_evidence() {
        let mut course = BodyCourse::new(61_001).unwrap();
        let experience = course
            .experience(
                BodyCapability::GazeContingency,
                BodyExperienceMode::Development,
                61_002,
            )
            .unwrap();
        assert!(experience.replay_exact);

        let mut changed_observations = experience.observations.clone();
        changed_observations.pop();
        assert!(!replay(
            experience.capability,
            experience.seed,
            experience.perturbation,
            ReplayEvidence {
                checkpoint_before: &experience.checkpoint_before,
                samples: &experience.samples,
                observations: &changed_observations,
                checkpoint_after: &experience.checkpoint_after,
            },
        )
        .unwrap());

        let mut changed_samples = experience.samples.clone();
        changed_samples.pop();
        assert!(!replay(
            experience.capability,
            experience.seed,
            experience.perturbation,
            ReplayEvidence {
                checkpoint_before: &experience.checkpoint_before,
                samples: &changed_samples,
                observations: &experience.observations,
                checkpoint_after: &experience.checkpoint_after,
            },
        )
        .unwrap());
    }

    #[test]
    fn perturbed_probe_replays_exactly_and_discards() {
        let mut course = BodyCourse::new(63_001).unwrap();
        let development = course
            .experience(
                BodyCapability::GazeContingency,
                BodyExperienceMode::Development,
                63_002,
            )
            .unwrap();
        assert_eq!(development.verdict, BodyVerdict::Passed);
        let durable = course.checkpoint_bytes().unwrap();

        let probe = course
            .experience_with_perturbation(
                BodyCapability::GazeContingency,
                BodyExperienceMode::Probe,
                63_003,
                BodyPerturbation {
                    control: BodyControl::new(
                        BodyAxis::EyeVertical { eye: Eye::Left },
                        Direction::Increase,
                    ),
                    impulse: 1,
                },
            )
            .unwrap();
        assert!(probe.replay_exact);
        assert!(probe.durable_unchanged);
        assert_eq!(course.checkpoint_bytes().unwrap(), durable);
    }

    #[test]
    fn course_steps_preserve_every_choice_arrow() {
        let seed = 31_001;
        let mut course = BodyCourse::new(seed).unwrap();
        for capability in BodyCapability::ORDER {
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
            assert_ne!(
                development.verdict,
                BodyVerdict::MissingExploration,
                "development stopped at {capability:?}"
            );

            let checkpoint = truelearner_workstation::WorkstationCheckpoint::decode(
                &development.checkpoint_before,
            )
            .unwrap();
            let mut harness = WorkstationHarness::restore(checkpoint).unwrap();
            let mut world = FlatWorld::generated(development.seed, capability);
            for expected in &development.observations {
                let sample = world.sample(&harness.read().unwrap()).unwrap();
                let (observation, trace) = harness
                    .step_traced_with_causal_parents(sample, &[], &[])
                    .unwrap();
                verify_choice_contract(&trace).unwrap();
                assert_eq!(&observation, expected);
                assert!(trace
                    .iter()
                    .any(|event| matches!(event, BodyTraceEvent::Choice(_))));
            }

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
    }
}
