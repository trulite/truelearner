#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use truelearner_workstation::{
    BodyAxis, ContactSample, Eye, LightField, Point, Protocol, ResearchChoiceDiagnostic,
    ResearchHarnessConfig, ResearchOpportunityIncidence, ResearchRetinalSnapshot,
    ResearchTransitionOpportunity, ResearchVisualComposition, WorkstationError, WorkstationHarness,
    WorkstationStepObservation, WorldSample, BODY_MAX, TOUCH_SITES,
};

const SIDE: u16 = 65;
const CENTER: i16 = 512;
const STEPS: usize = 48;
const WORK_BOUND: u64 = 200_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepthBand {
    Far,
    Middle,
    Near,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetRelation {
    Outward,
    Inward,
}

impl TargetRelation {
    pub const ALL: [Self; 2] = [Self::Outward, Self::Inward];

    const fn targets(self, half: i16) -> [i16; 2] {
        match self {
            Self::Outward => [CENTER.saturating_sub(half), CENTER.saturating_add(half)],
            Self::Inward => [CENTER.saturating_add(half), CENTER.saturating_sub(half)],
        }
    }
}

impl DepthBand {
    pub const ALL: [Self; 3] = [Self::Far, Self::Middle, Self::Near];

    pub const fn half_disparity(self) -> i16 {
        match self {
            Self::Far => 16,
            Self::Middle => 48,
            Self::Near => 80,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlignmentArm {
    Minimal,
    FovealIdentityOnly,
    CenteredReturnOnly,
    FovealIdentityReturnComposition,
    Stable,
    Complete,
    CollapsedPlacement,
    NoVisualReturn,
    NoThresholdFactorization,
    Production,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlignmentStep {
    pub sequence: u64,
    pub left_before: i16,
    pub right_before: i16,
    pub left_after: i16,
    pub right_after: i16,
    pub left_error_before: u16,
    pub right_error_before: u16,
    pub left_error_after: u16,
    pub right_error_after: u16,
    pub left_improved: bool,
    pub right_improved: bool,
    pub horizontal_movements: usize,
    pub horizontal_effort_frames: usize,
    pub retinal_transitions: usize,
    pub changed_retinal_samples: usize,
    pub eligible_retinal_transitions: usize,
    pub active_eligible_retinal_transitions: usize,
    pub retinal_snapshot: Vec<ResearchRetinalSnapshot>,
    pub choice_diagnostics: Vec<ResearchChoiceDiagnostic>,
    pub naturally_quiescent: bool,
    pub physical_work: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BandTrace {
    pub relation: TargetRelation,
    pub band: DepthBand,
    pub target_x: [i16; 2],
    pub target_disparity: u16,
    pub initial_error: [u16; 2],
    pub minimum_error: [u16; 2],
    pub best_joint_error: [u16; 2],
    pub final_error: [u16; 2],
    /// Eye separation at the externally observed minimum joint target error.
    pub achieved_gaze_separation: u16,
    pub left_improvement_steps: usize,
    pub right_improvement_steps: usize,
    pub joint_improvement_steps: usize,
    pub joint_alignment_steps: usize,
    pub post_acquisition_hold_steps: usize,
    pub total_retinal_transitions: usize,
    pub naturally_quiescent: bool,
    pub physical_work: u64,
    pub replay_exact: bool,
    pub steps: Vec<AlignmentStep>,
}

impl BandTrace {
    pub fn both_eyes_improved(&self) -> bool {
        self.minimum_error[0] < self.initial_error[0]
            && self.minimum_error[1] < self.initial_error[1]
    }

    pub fn bounded_alignment_acquired(&self) -> bool {
        self.both_eyes_improved()
            && self.best_joint_error == [0, 0]
            && self.joint_alignment_steps > 0
            && self.total_retinal_transitions > 0
    }

    pub fn stable_fixation_acquired(&self) -> bool {
        let Some(first_alignment) = self
            .steps
            .iter()
            .position(|step| step.left_error_after == 0 && step.right_error_after == 0)
        else {
            return false;
        };
        self.bounded_alignment_acquired()
            && self.post_acquisition_hold_steps >= 8
            && self.steps[first_alignment + 1..first_alignment + 9]
                .iter()
                .all(|step| step.horizontal_effort_frames == 0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlignmentTrace {
    pub schema: String,
    pub arm: AlignmentArm,
    pub bands: Vec<BandTrace>,
}

impl AlignmentTrace {
    pub fn bounded_alignment_passes(&self) -> bool {
        self.bands.iter().all(|band| {
            band.bounded_alignment_acquired()
                && band.replay_exact
                && band.naturally_quiescent
                && band.physical_work <= WORK_BOUND
        }) && TargetRelation::ALL.into_iter().all(|relation| {
            self.bands
                .iter()
                .filter(|band| band.relation == relation)
                .collect::<Vec<_>>()
                .windows(2)
                .all(|pair| pair[0].achieved_gaze_separation < pair[1].achieved_gaze_separation)
        })
    }

    pub fn stable_fixation_passes(&self) -> bool {
        self.bounded_alignment_passes()
            && self.bands.iter().all(BandTrace::stable_fixation_acquired)
    }
}

pub fn run_alignment_arm(arm: AlignmentArm) -> Result<AlignmentTrace, WorkstationError> {
    let mut bands = Vec::with_capacity(DepthBand::ALL.len() * TargetRelation::ALL.len());
    for (relation_index, relation) in TargetRelation::ALL.into_iter().enumerate() {
        for (band_index, band) in DepthBand::ALL.into_iter().enumerate() {
            let seed = 91_000 + (relation_index * DepthBand::ALL.len() + band_index) as u64;
            bands.push(run_band(arm, relation, band, seed)?);
        }
    }
    Ok(AlignmentTrace {
        schema: "workstation-binocular-alignment/v1".to_string(),
        arm,
        bands,
    })
}

fn run_band(
    arm: AlignmentArm,
    relation: TargetRelation,
    band: DepthBand,
    seed: u64,
) -> Result<BandTrace, WorkstationError> {
    let visual = visual_composition(arm);
    let mut harness = new_harness(arm, seed, visual)?;
    let checkpoint_before = harness.save()?;
    let half = band.half_disparity();
    let targets = relation.targets(half);
    let sample = stereo_sample(targets, 255)?;
    let mut samples = Vec::with_capacity(STEPS);
    let mut observations = Vec::with_capacity(STEPS);
    let mut retinal_snapshots = Vec::with_capacity(STEPS);
    for _ in 0..STEPS {
        samples.push(sample.clone());
        retinal_snapshots.push(harness.research_retinal_snapshot(&sample));
        observations.push(harness.step(sample.clone())?);
    }
    let checkpoint_after = harness.save()?.canonical_bytes()?;
    let replay_exact = replay(
        arm,
        visual,
        checkpoint_before,
        &samples,
        &observations,
        &checkpoint_after,
    )?;
    Ok(project_band(
        relation,
        band,
        targets,
        observations,
        retinal_snapshots,
        replay_exact,
    ))
}

fn base_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity:
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDelta,
    }
}

fn visual_composition(arm: AlignmentArm) -> ResearchVisualComposition {
    let complete = ResearchVisualComposition::binocular_alignment();
    match arm {
        AlignmentArm::Minimal => complete.with_threshold_factorization(false),
        AlignmentArm::FovealIdentityOnly => complete
            .with_threshold_factorization(false)
            .with_foveal_identity_opportunity(true),
        AlignmentArm::CenteredReturnOnly => complete
            .with_threshold_factorization(false)
            .with_centered_movement_return(false),
        AlignmentArm::FovealIdentityReturnComposition => complete
            .with_threshold_factorization(false)
            .with_foveal_identity_opportunity(true)
            .with_centered_movement_return(false),
        AlignmentArm::Stable => ResearchVisualComposition::binocular_stable_fixation(),
        AlignmentArm::Complete => complete,
        AlignmentArm::CollapsedPlacement => complete
            .with_signed_placement(false)
            .with_threshold_factorization(false),
        AlignmentArm::NoVisualReturn => complete
            .with_movement_caused_return(false)
            .with_threshold_factorization(false),
        AlignmentArm::NoThresholdFactorization => complete.with_threshold_factorization(false),
        AlignmentArm::Production => ResearchVisualComposition::default(),
    }
}

fn new_harness(
    arm: AlignmentArm,
    seed: u64,
    visual: ResearchVisualComposition,
) -> Result<WorkstationHarness, WorkstationError> {
    if arm == AlignmentArm::Production {
        WorkstationHarness::new(seed)
    } else {
        WorkstationHarness::new_research_composed(seed, base_config(), visual)
    }
}

fn replay(
    arm: AlignmentArm,
    visual: ResearchVisualComposition,
    checkpoint_before: truelearner_workstation::WorkstationCheckpoint,
    samples: &[WorldSample],
    expected: &[WorkstationStepObservation],
    checkpoint_after: &[u8],
) -> Result<bool, WorkstationError> {
    let mut replay = if arm == AlignmentArm::Production {
        WorkstationHarness::restore(checkpoint_before)?
    } else {
        WorkstationHarness::restore_research_composed(checkpoint_before, base_config(), visual)?
    };
    let mut observed = Vec::with_capacity(samples.len());
    for sample in samples {
        observed.push(replay.step(sample.clone())?);
    }
    Ok(observed == expected && replay.save()?.canonical_bytes()? == checkpoint_after)
}

fn stereo_sample(targets: [i16; 2], light: u8) -> Result<WorldSample, WorkstationError> {
    let fields = [
        target_field(targets[0], light)?,
        target_field(targets[1], light)?,
    ];
    WorldSample::new(
        [fields[0].clone(), fields[1].clone()],
        [ContactSample::default(); TOUCH_SITES],
    )
}

fn target_field(target_x: i16, light: u8) -> Result<LightField, WorkstationError> {
    let mut pixels = vec![0; usize::from(SIDE) * usize::from(SIDE)];
    let target = Point::new(target_x, CENTER)?;
    let x = usize::try_from(i32::from(target.x()) * i32::from(SIDE - 1) / i32::from(BODY_MAX))
        .unwrap_or(0);
    for y in 0..usize::from(SIDE) {
        pixels[y * usize::from(SIDE) + x] = light;
    }
    LightField::new(SIDE, SIDE, pixels)
}

fn error(position: i16, target: i16) -> u16 {
    position.abs_diff(target)
}

fn project_band(
    relation: TargetRelation,
    band: DepthBand,
    targets: [i16; 2],
    observations: Vec<WorkstationStepObservation>,
    retinal_snapshots: Vec<Vec<ResearchRetinalSnapshot>>,
    replay_exact: bool,
) -> BandTrace {
    let initial = observations
        .first()
        .expect("the fixed horizon is non-empty")
        .state_before
        .clone();
    let initial_error = [
        error(initial.eye(Eye::Left).gaze().x(), targets[0]),
        error(initial.eye(Eye::Right).gaze().x(), targets[1]),
    ];
    let mut minimum_error = initial_error;
    let mut best_joint_error = initial_error;
    let mut left_improvement_steps = 0;
    let mut right_improvement_steps = 0;
    let mut joint_improvement_steps = 0;
    let mut joint_alignment_steps = 0;
    let mut total_retinal_transitions = 0;
    let mut naturally_quiescent = true;
    let mut physical_work = 0_u64;
    let mut achieved_gaze_separation = initial
        .eye(Eye::Left)
        .gaze()
        .x()
        .abs_diff(initial.eye(Eye::Right).gaze().x());
    let steps = observations
        .iter()
        .zip(&retinal_snapshots)
        .map(|(observation, snapshot)| {
            let left_before = observation.state_before.eye(Eye::Left).gaze().x();
            let right_before = observation.state_before.eye(Eye::Right).gaze().x();
            let left_after = observation.state_after.eye(Eye::Left).gaze().x();
            let right_after = observation.state_after.eye(Eye::Right).gaze().x();
            let left_error_before = error(left_before, targets[0]);
            let right_error_before = error(right_before, targets[1]);
            let left_error_after = error(left_after, targets[0]);
            let right_error_after = error(right_after, targets[1]);
            let left_improved = left_error_after < left_error_before;
            let right_improved = right_error_after < right_error_before;
            left_improvement_steps += usize::from(left_improved);
            right_improvement_steps += usize::from(right_improved);
            joint_improvement_steps += usize::from(left_improved && right_improved);
            joint_alignment_steps += usize::from(left_error_after == 0 && right_error_after == 0);
            minimum_error[0] = minimum_error[0].min(left_error_after);
            minimum_error[1] = minimum_error[1].min(right_error_after);
            if u32::from(left_error_after) + u32::from(right_error_after)
                < u32::from(best_joint_error[0]) + u32::from(best_joint_error[1])
            {
                best_joint_error = [left_error_after, right_error_after];
                achieved_gaze_separation = left_after.abs_diff(right_after);
            }
            total_retinal_transitions += observation.retinal_transitions.len();
            naturally_quiescent &= observation.naturally_quiescent;
            physical_work = physical_work.saturating_add(observation.metrics.physical_work);
            AlignmentStep {
                sequence: observation.sequence,
                left_before,
                right_before,
                left_after,
                right_after,
                left_error_before,
                right_error_before,
                left_error_after,
                right_error_after,
                left_improved,
                right_improved,
                horizontal_movements: observation
                    .movements
                    .iter()
                    .filter(|movement| {
                        movement.changed && matches!(movement.axis, BodyAxis::EyeHorizontal { .. })
                    })
                    .count(),
                horizontal_effort_frames: observation
                    .movements
                    .iter()
                    .filter(|movement| matches!(movement.axis, BodyAxis::EyeHorizontal { .. }))
                    .count(),
                retinal_transitions: observation.retinal_transitions.len(),
                changed_retinal_samples: snapshot
                    .iter()
                    .filter(|item| {
                        item.previous_bin
                            .is_some_and(|previous| previous != item.current_bin)
                    })
                    .count(),
                eligible_retinal_transitions: snapshot
                    .iter()
                    .filter(|item| item.eligible_transition())
                    .count(),
                active_eligible_retinal_transitions: snapshot
                    .iter()
                    .filter(|item| item.current_bin > 0 && item.eligible_transition())
                    .count(),
                retinal_snapshot: snapshot.clone(),
                choice_diagnostics: observation.choice_diagnostics.clone(),
                naturally_quiescent: observation.naturally_quiescent,
                physical_work: observation.metrics.physical_work,
            }
        })
        .collect::<Vec<_>>();
    let post_acquisition_hold_steps = steps
        .iter()
        .position(|step| step.left_error_after == 0 && step.right_error_after == 0)
        .map_or(0, |first| {
            steps[first + 1..]
                .iter()
                .take_while(|step| step.left_error_after == 0 && step.right_error_after == 0)
                .count()
        });
    let final_state = observations
        .last()
        .expect("the fixed horizon is non-empty")
        .state_after
        .clone();
    let final_error = [
        error(final_state.eye(Eye::Left).gaze().x(), targets[0]),
        error(final_state.eye(Eye::Right).gaze().x(), targets[1]),
    ];
    BandTrace {
        relation,
        band,
        target_x: targets,
        target_disparity: targets[0].abs_diff(targets[1]),
        initial_error,
        minimum_error,
        best_joint_error,
        final_error,
        achieved_gaze_separation,
        left_improvement_steps,
        right_improvement_steps,
        joint_improvement_steps,
        joint_alignment_steps,
        post_acquisition_hold_steps,
        total_retinal_transitions,
        naturally_quiescent,
        physical_work,
        replay_exact,
        steps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_product_acquires_mirrored_binocular_alignment() {
        let trace = run_alignment_arm(AlignmentArm::Minimal).unwrap();
        let summary = trace
            .bands
            .iter()
            .map(|band| {
                (
                    band.band,
                    band.initial_error,
                    band.best_joint_error,
                    band.final_error,
                    band.achieved_gaze_separation,
                )
            })
            .collect::<Vec<_>>();
        assert!(trace.bounded_alignment_passes(), "{summary:?}");
    }

    #[test]
    fn post_alignment_choice_projection_is_inert() {
        let trace = run_alignment_arm(AlignmentArm::Minimal).unwrap();
        assert!(trace.bands.iter().all(|band| band.replay_exact));
        assert!(trace
            .bands
            .iter()
            .flat_map(|band| &band.steps)
            .any(|step| !step.choice_diagnostics.is_empty()));
    }

    #[test]
    fn stable_fixation_holds_all_mirrored_relations() {
        let trace = run_alignment_arm(AlignmentArm::Stable).unwrap();
        let summary = trace
            .bands
            .iter()
            .map(|band| {
                (
                    band.relation,
                    band.band,
                    band.best_joint_error,
                    band.post_acquisition_hold_steps,
                    band.final_error,
                )
            })
            .collect::<Vec<_>>();
        assert!(trace.stable_fixation_passes(), "{summary:?}");
    }

    #[test]
    fn fixation_controls_preserve_exploration_and_eye_locality() {
        for arm in [AlignmentArm::Minimal, AlignmentArm::FovealIdentityOnly] {
            assert!(!run_alignment_arm(arm).unwrap().stable_fixation_passes());
        }

        let visual = ResearchVisualComposition::binocular_stable_fixation();
        let mut one_eye = new_harness(AlignmentArm::Stable, 92_001, visual).unwrap();
        let observation = one_eye
            .step(stereo_sample([CENTER, CENTER + 80], 255).unwrap())
            .unwrap();
        assert!(!observation.movements.iter().any(|movement| {
            matches!(movement.axis, BodyAxis::EyeHorizontal { eye: Eye::Left })
        }));
        assert!(observation.movements.iter().any(|movement| {
            movement.changed && matches!(movement.axis, BodyAxis::EyeHorizontal { eye: Eye::Right })
        }));

        let dark = WorldSample::new(
            [
                LightField::filled(SIDE, SIDE, 0).unwrap(),
                LightField::filled(SIDE, SIDE, 0).unwrap(),
            ],
            [ContactSample::default(); TOUCH_SITES],
        )
        .unwrap();
        let mut dark_harness = new_harness(AlignmentArm::Stable, 92_002, visual).unwrap();
        let dark_observation = dark_harness.step(dark).unwrap();
        assert!(Eye::ALL.into_iter().all(|eye| {
            dark_observation.movements.iter().any(|movement| {
                movement.changed
                    && matches!(movement.axis, BodyAxis::EyeHorizontal { eye: owner } if owner == eye)
            })
        }));

        let centered = stereo_sample([CENTER, CENTER], 255).unwrap();
        let mut centered_harness = new_harness(AlignmentArm::Stable, 92_003, visual).unwrap();
        let first = centered_harness.step(centered.clone()).unwrap();
        let second = centered_harness.step(centered).unwrap();
        assert!(first.choice_diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic,
                ResearchChoiceDiagnostic::Choice {
                    admitted_controls,
                    ..
                } if admitted_controls.iter().any(|control| {
                    matches!(control.axis(), BodyAxis::EyeHorizontal { .. })
                })
            )
        }));
        assert!(first
            .movements
            .iter()
            .chain(&second.movements)
            .all(|movement| !matches!(movement.axis, BodyAxis::EyeHorizontal { .. })));
        assert!(first
            .pending_transitions
            .iter()
            .all(|axis| { !matches!(axis, BodyAxis::EyeHorizontal { .. }) }));
        assert!(second.retinal_transitions.is_empty());
    }

    #[test]
    fn foveal_identity_is_eye_local() {
        fixation_controls_preserve_exploration_and_eye_locality();
    }

    #[test]
    fn removals_fail_at_declared_rungs() {
        for arm in [
            AlignmentArm::CollapsedPlacement,
            AlignmentArm::NoVisualReturn,
        ] {
            let trace = run_alignment_arm(arm).unwrap();
            assert!(
                !trace.bounded_alignment_passes(),
                "{arm:?} unexpectedly passed"
            );
        }

        let overcomposed = run_alignment_arm(AlignmentArm::Complete).unwrap();
        assert!(!overcomposed.bounded_alignment_passes());
    }

    #[test]
    fn binocular_controls_and_transfer_hold() {
        let complete = run_alignment_arm(AlignmentArm::Minimal).unwrap();
        assert!(complete.bands.iter().all(|band| {
            band.replay_exact
                && band.naturally_quiescent
                && band.total_retinal_transitions > 0
                && band.physical_work <= WORK_BOUND
        }));
        let production = run_alignment_arm(AlignmentArm::Production).unwrap();
        assert!(!production.bounded_alignment_passes());

        let wire = serde_json::to_string(&stereo_sample([432, 592], 255).unwrap()).unwrap();
        for forbidden in [
            "target",
            "depth",
            "disparity",
            "error",
            "direction",
            "verdict",
            "capability",
        ] {
            assert!(!wire.contains(forbidden), "leaked {forbidden}: {wire}");
        }
    }
}
