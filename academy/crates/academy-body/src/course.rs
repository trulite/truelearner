use crate::world::FlatWorld;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;
use truelearner_human::{
    BodyAxis, BodyMovement, HumanError, HumanHarness, HumanStepObservation, LightField, Point,
    Side, WorldSample,
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
    DragPinch,
    Bimanual,
}

impl BodyCapability {
    pub const ORDER: [Self; 11] = [
        Self::GazeContingency,
        Self::GazeControl,
        Self::BinocularDepth,
        Self::HandContingency,
        Self::DigitSeparation,
        Self::SelfWorld,
        Self::Contact,
        Self::VisualReach,
        Self::TapHoldRelease,
        Self::DragPinch,
        Self::Bimanual,
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
            Self::DragPinch => &[Self::TapHoldRelease],
            Self::Bimanual => &[Self::DragPinch],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyCourseKind {
    EyeControl,
    HandAndFingerControl,
    EyeHandCoordination,
    TouchGuidedManipulation,
    TwoHandCoordination,
}

impl BodyCourseKind {
    pub const ORDER: [Self; 5] = [
        Self::EyeControl,
        Self::HandAndFingerControl,
        Self::EyeHandCoordination,
        Self::TouchGuidedManipulation,
        Self::TwoHandCoordination,
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
            Self::TouchGuidedManipulation => &[
                BodyCapability::Contact,
                BodyCapability::VisualReach,
                BodyCapability::TapHoldRelease,
                BodyCapability::DragPinch,
            ],
            Self::TwoHandCoordination => &[BodyCapability::Bimanual],
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
    pub observations: Vec<HumanStepObservation>,
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
    harness: HumanHarness,
    acquired: BTreeSet<BodyCapability>,
    experiences: Vec<BodyExperience>,
}

impl BodyCourse {
    pub fn new(seed: u64) -> Result<Self, BodyCourseError> {
        Ok(Self {
            seed,
            harness: HumanHarness::new(seed)?,
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
        let checkpoint = truelearner_human::HumanCheckpoint::decode(&durable_before)?;
        let mut working = HumanHarness::restore(checkpoint)?;
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
            schema_version: 2,
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
        let checkpoint = truelearner_human::HumanCheckpoint::decode(bytes)?;
        self.harness = HumanHarness::restore(checkpoint)?;
        Ok(())
    }
}

fn replay(
    checkpoint_before: &[u8],
    samples: &[WorldSample],
    expected: &[HumanStepObservation],
    checkpoint_after: &[u8],
) -> Result<bool, BodyCourseError> {
    let checkpoint = truelearner_human::HumanCheckpoint::decode(checkpoint_before)?;
    let mut harness = HumanHarness::restore(checkpoint)?;
    let mut observed = Vec::with_capacity(samples.len());
    for sample in samples {
        observed.push(harness.step(sample.clone())?);
    }
    Ok(observed == expected && harness.save()?.canonical_bytes()? == checkpoint_after)
}

fn evaluate(
    capability: BodyCapability,
    samples: &[WorldSample],
    observations: &[HumanStepObservation],
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
        let vergence_movements = observations
            .iter()
            .flat_map(|observation| &observation.movements)
            .filter(|movement| movement.changed && movement.axis == BodyAxis::Vergence)
            .count();
        if !observations
            .iter()
            .flat_map(|observation| &observation.movements)
            .any(|movement| movement.changed)
        {
            return BodyVerdict::MissingExploration;
        }
        return binocular_depth_verdict(
            vergence_movements,
            binocular_visual_consequences(samples, observations),
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
        .filter(|movement| side_of(movement.axis).is_some())
        .count();
    let fingers = movements
        .iter()
        .filter(|movement| matches!(movement.axis, BodyAxis::FingerFlexion { .. }))
        .count();
    let force = movements
        .iter()
        .filter(|movement| matches!(movement.axis, BodyAxis::ContactForce { .. }))
        .count();
    let palm = movements
        .iter()
        .filter(|movement| {
            matches!(
                movement.axis,
                BodyAxis::PalmHorizontal { .. } | BodyAxis::PalmVertical { .. }
            )
        })
        .count();
    let contact = samples.iter().any(|sample| {
        [Side::Left, Side::Right].into_iter().any(|side| {
            sample
                .contacts(side)
                .iter()
                .any(|value| value.pressure() > 0)
        })
    });
    let left = movements
        .iter()
        .any(|movement| side_of(movement.axis) == Some(Side::Left));
    let right = movements
        .iter()
        .any(|movement| side_of(movement.axis) == Some(Side::Right));
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
        BodyCapability::TapHoldRelease => contact && force >= 1,
        BodyCapability::DragPinch => contact && palm >= 1 && fingers >= 1,
        BodyCapability::Bimanual => contact && left && right,
    };
    if passed {
        BodyVerdict::Passed
    } else {
        BodyVerdict::Failed
    }
}

fn binocular_visual_consequences(
    samples: &[WorldSample],
    observations: &[HumanStepObservation],
) -> usize {
    samples
        .iter()
        .zip(observations)
        .filter(|(_, observation)| {
            observation
                .movements
                .iter()
                .any(|movement| movement.changed && movement.axis == BodyAxis::Vergence)
        })
        .filter(|(sample, _)| has_stereo_target(sample))
        .filter(|(sample, observation)| {
            [Side::Left, Side::Right].into_iter().all(|side| {
                focus_changes_light(
                    sample.eye(side),
                    observation.state_before.eyes().focus(side),
                    observation.state_after.eyes().focus(side),
                )
            })
        })
        .count()
}

fn binocular_depth_verdict(
    vergence_movements: usize,
    binocular_visual_consequences: usize,
) -> BodyVerdict {
    if vergence_movements >= 2 && binocular_visual_consequences >= 2 {
        BodyVerdict::Passed
    } else {
        BodyVerdict::Failed
    }
}

fn has_stereo_target(sample: &WorldSample) -> bool {
    let target_column = |side| {
        let field = sample.eye(side);
        field
            .pixels()
            .iter()
            .position(|pixel| *pixel == 255)
            .map(|index| index % usize::from(field.width()))
    };
    matches!(
        (target_column(Side::Left), target_column(Side::Right)),
        (Some(left), Some(right)) if left != right
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
    observations: &[HumanStepObservation],
) -> usize {
    samples
        .iter()
        .zip(observations)
        .filter(|(_, observation)| {
            observation
                .movements
                .iter()
                .any(|movement| movement.changed && is_gaze(movement.axis))
        })
        .filter(|(sample, observation)| {
            [Side::Left, Side::Right].into_iter().any(|side| {
                focus_changes_light(
                    sample.eye(side),
                    observation.state_before.eyes().focus(side),
                    observation.state_after.eyes().focus(side),
                )
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

fn focus_changes_light(field: &LightField, before: Point, after: Point) -> bool {
    before != after && field.sample(before) != field.sample(after)
}

fn is_gaze(axis: BodyAxis) -> bool {
    matches!(
        axis,
        BodyAxis::GazeHorizontal | BodyAxis::GazeVertical | BodyAxis::Vergence
    )
}

fn side_of(axis: BodyAxis) -> Option<Side> {
    match axis {
        BodyAxis::PalmHorizontal { side }
        | BodyAxis::PalmVertical { side }
        | BodyAxis::Wrist { side }
        | BodyAxis::ContactForce { side }
        | BodyAxis::Spread { side }
        | BodyAxis::ThumbOpposition { side }
        | BodyAxis::FingerFlexion { side, .. } => Some(side),
        BodyAxis::GazeHorizontal | BodyAxis::GazeVertical | BodyAxis::Vergence => None,
    }
}

#[derive(Debug)]
pub enum BodyCourseError {
    Human(HumanError),
    Prerequisite(BodyCapability),
    Io(String),
    Serialization(String),
    OutputExists(String),
}

impl fmt::Display for BodyCourseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Human(error) => write!(formatter, "human Harness failed: {error}"),
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

impl From<HumanError> for BodyCourseError {
    fn from(value: HumanError) -> Self {
        Self::Human(value)
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
        let harness = HumanHarness::new(11).unwrap();
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
            truelearner_human::Direction::Decrease,
            truelearner_human::Direction::Increase
        );
    }

    #[test]
    fn equal_opposing_effort_is_not_credited_as_movement() {
        let canceled = BodyMovement {
            axis: BodyAxis::GazeHorizontal,
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
    fn binocular_depth_requires_repeated_two_eye_vergence_consequences() {
        assert_eq!(binocular_depth_verdict(0, 0), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(2, 0), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(1, 1), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(3, 1), BodyVerdict::Failed);
        assert_eq!(binocular_depth_verdict(2, 2), BodyVerdict::Passed);
    }

    #[test]
    fn focus_change_must_change_actual_light() {
        use truelearner_human::BODY_MAX;

        let uniform = LightField::filled(2, 1, 7).unwrap();
        assert!(!focus_changes_light(
            &uniform,
            Point::new(0, 0).unwrap(),
            Point::new(BODY_MAX, 0).unwrap()
        ));
        let varied = LightField::new(2, 1, vec![0, 255]).unwrap();
        assert!(focus_changes_light(
            &varied,
            Point::new(0, 0).unwrap(),
            Point::new(BODY_MAX, 0).unwrap()
        ));
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
        let all_together = [Side::Left, Side::Right]
            .into_iter()
            .flat_map(|side| {
                truelearner_human::Digit::ALL
                    .into_iter()
                    .map(move |digit| changed(BodyAxis::FingerFlexion { side, digit }))
            })
            .collect::<Vec<_>>();
        assert!(!has_digit_separation([all_together.as_slice()]));

        let thumb = changed(BodyAxis::FingerFlexion {
            side: Side::Left,
            digit: truelearner_human::Digit::Thumb,
        });
        assert!(!has_digit_separation([&[thumb][..], &[thumb][..]]));

        let index = changed(BodyAxis::FingerFlexion {
            side: Side::Left,
            digit: truelearner_human::Digit::Index,
        });
        assert!(has_digit_separation([&[thumb][..], &[index][..]]));

        let two_together = [thumb, index];
        assert!(!has_digit_separation([two_together.as_slice()]));
    }
}
