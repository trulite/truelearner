use crate::world::FlatWorld;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;
use truelearner_workstation::{
    BodyAxis, BodyMovement, Eye, LightField, WorkstationError, WorkstationHarness,
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
    Development,
    Probe,
    Transfer,
    Retention,
    Control,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyVerdict {
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
        let mut world = FlatWorld::generated(seed, capability);
        let mut samples = Vec::with_capacity(STEPS_PER_EXPERIENCE);
        let mut observations = Vec::with_capacity(STEPS_PER_EXPERIENCE);
        let mut physical_work = 0_u64;
        let mut plasticity_updates = 0_u64;
        let mut naturally_quiescent = true;
        let mut budget_exceeded = false;
        for _ in 0..STEPS_PER_EXPERIENCE {
            let sample = world.sample(&working.read()?)?;
            let observation = working.step(sample.clone())?;
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
        let replay_exact = replay(&durable_before, &samples, &observations, &checkpoint_after)?;
        if !replay_exact {
            verdict = BodyVerdict::Failed;
        }
        if mode == BodyExperienceMode::Development {
            self.harness = working;
        }
        let durable_unchanged = if mode == BodyExperienceMode::Development {
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
        Ok(CourseRun {
            schema_version: 3,
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

fn replay(
    checkpoint_before: &[u8],
    samples: &[WorldSample],
    expected: &[WorkstationStepObservation],
    checkpoint_after: &[u8],
) -> Result<bool, BodyCourseError> {
    let checkpoint = truelearner_workstation::WorkstationCheckpoint::decode(checkpoint_before)?;
    let mut harness = WorkstationHarness::restore(checkpoint)?;
    let mut observed = Vec::with_capacity(samples.len());
    for sample in samples {
        observed.push(harness.step(sample.clone())?);
    }
    Ok(observed == expected && harness.save()?.canonical_bytes()? == checkpoint_after)
}

fn evaluate(
    capability: BodyCapability,
    samples: &[WorldSample],
    observations: &[WorkstationStepObservation],
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
    )
}

fn evaluate_physical(
    capability: BodyCapability,
    movements: &[BodyMovement],
    samples: &[WorldSample],
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
    let fingers = movements
        .iter()
        .filter(|movement| matches!(movement.axis, BodyAxis::FingerFlexion { .. }))
        .count();
    let depth = movements
        .iter()
        .filter(|movement| matches!(movement.axis, BodyAxis::PalmDepth))
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
        BodyCapability::TapHoldRelease => contact && (depth >= 1 || fingers >= 1),
        BodyCapability::DragOpposition => contact && palm >= 1 && fingers >= 1 && opposition >= 1,
    };
    if passed {
        BodyVerdict::Passed
    } else {
        BodyVerdict::Failed
    }
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
            evaluate_physical(BodyCapability::GazeContingency, &[canceled], &[], 0),
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
