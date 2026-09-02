#![forbid(unsafe_code)]
//! Development and fresh-probe evidence for the touchscreen workstation.
//!
//! The ladder a pointer body climbs before ARC: the eyes find and hold a
//! lit target, the arm reaches a screen that starts within easy reach and
//! then sits at the ordinary distance, and taps land on the target far
//! above chance from a big-toy development phase with honest shrinking
//! probes. Every claim is measured in a fresh probe that discards
//! mutation; development phases exist to present the consequence, never to
//! be the claim.

pub mod screen_use;

use academy_workstation2::{TargetApp, Workstation2, Workstation2Session};
use serde::{Deserialize, Serialize};
use truelearner_workstation::{Eye, WorkstationCheckpoint, WorkstationError};

const MAX_STEPS: usize = 512;
/// The screen depth during Touch development: just past the palm's resting
/// depth, so the contact consequence presents within a few excursions. The
/// toy is placed within the baby's reach.
const CLOSE_SCREEN_DEPTH: i16 = 304;
/// The ordinary screen depth every probe uses.
const ORDINARY_SCREEN_DEPTH: i16 = academy_workstation2::CONTACT_DEPTH;
/// The big toy for AimedTap development: half the screen.
const BIG_TARGET_SIDE: i16 = screen_use::BIG_TARGET_SIDE;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Capability {
    Gaze,
    Touch,
    AimedTap,
    LiveKey,
    DeadKey,
    Scan,
    QuietHand,
    Sequence,
    Drag,
}

impl Capability {
    pub const ALL: [Self; 9] = [
        Self::Gaze,
        Self::Touch,
        Self::AimedTap,
        Self::LiveKey,
        Self::DeadKey,
        Self::Scan,
        Self::QuietHand,
        Self::Sequence,
        Self::Drag,
    ];

    /// Generic screen capabilities required before an ARC application may
    /// use this body. Drag is an independent manipulation capability.
    pub const ARC_PREREQUISITES: [Self; 8] = [
        Self::Gaze,
        Self::Touch,
        Self::AimedTap,
        Self::LiveKey,
        Self::DeadKey,
        Self::Scan,
        Self::QuietHand,
        Self::Sequence,
    ];

    fn rung(self) -> Option<screen_use::RungKind> {
        match self {
            Self::LiveKey => Some(screen_use::RungKind::LiveKey),
            Self::DeadKey => Some(screen_use::RungKind::DeadKey),
            Self::Scan => Some(screen_use::RungKind::Scan),
            Self::QuietHand => Some(screen_use::RungKind::QuietHand),
            Self::Sequence => Some(screen_use::RungKind::Sequence),
            Self::Drag => Some(screen_use::RungKind::Drag),
            Self::Gaze | Self::Touch | Self::AimedTap => None,
        }
    }
}

/// One screen-use rung's measured outcome.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RungOutcome {
    pub kind: screen_use::RungKind,
    pub run: screen_use::RungRun,
}

pub use screen_use::RungState as EvidenceState;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseEvidence {
    pub steps: usize,
    /// Steps in which the target sat on the left fovea.
    pub foveal_steps: usize,
    /// Steps with fingertip contact on the screen.
    pub contact_steps: usize,
    pub taps: u32,
    pub target_taps: u32,
    pub chance: f64,
    pub physical_work: u64,
    pub naturally_quiescent: bool,
}

impl PhaseEvidence {
    pub fn gaze_acquired(&self) -> bool {
        self.foveal_steps >= self.steps / 3
    }

    pub fn touch_acquired(&self) -> bool {
        self.contact_steps >= self.steps / 16
    }

    pub fn rate(&self) -> f64 {
        if self.taps == 0 {
            0.0
        } else {
            f64::from(self.target_taps) / f64::from(self.taps)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CourseRun {
    pub gaze: PhaseEvidence,
    pub touch: PhaseEvidence,
    pub aimed_tap: screen_use::AimedTapRun,
    pub rungs: Vec<RungOutcome>,
    pub exact_replay: bool,
    pub first_failure: Option<Capability>,
}

impl CourseRun {
    pub fn state(&self, capability: Capability) -> EvidenceState {
        match capability {
            Capability::Gaze => acquired(self.gaze.gaze_acquired()),
            Capability::Touch => acquired(self.touch.touch_acquired()),
            Capability::AimedTap => self.aimed_tap.state,
            capability => {
                let kind = capability.rung().expect("screen-use capability");
                self.rungs
                    .iter()
                    .find(|outcome| outcome.kind == kind)
                    .expect("course records every screen-use rung")
                    .run
                    .state
            }
        }
    }

    pub fn arc_prerequisites_acquired(&self) -> bool {
        Capability::ARC_PREREQUISITES
            .into_iter()
            .all(|capability| self.state(capability) == EvidenceState::Acquired)
    }
}

fn acquired(observed: bool) -> EvidenceState {
    if observed {
        EvidenceState::Acquired
    } else {
        EvidenceState::Unknown
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Workstation2Course {
    steps_per_phase: usize,
}

impl Workstation2Course {
    pub fn new(steps_per_phase: usize) -> Self {
        Self {
            steps_per_phase: steps_per_phase.clamp(1, MAX_STEPS),
        }
    }

    pub fn run(
        self,
        checkpoint: WorkstationCheckpoint,
        seed: u64,
    ) -> Result<CourseRun, WorkstationError> {
        self.run_with_diagnostic_checkpoint(checkpoint, seed)
            .map(|(run, _)| run)
    }

    /// Run the course and retain its developed checkpoint for diagnostic use.
    /// The checkpoint carries no capability claim; callers must inspect
    /// `first_failure` and keep later applications behind their own gates.
    pub fn run_with_diagnostic_checkpoint(
        self,
        checkpoint: WorkstationCheckpoint,
        seed: u64,
    ) -> Result<(CourseRun, WorkstationCheckpoint), WorkstationError> {
        // The development ladder: gaze phase, then the close-screen touch
        // phase, then the big-toy tap phase. Each phase presents its
        // consequence to the learner; the phases train each other.
        let (gaze_development, after_gaze) = run_phase(
            checkpoint.clone(),
            TargetApp::lit(seed),
            ORDINARY_SCREEN_DEPTH,
            self.steps_per_phase,
        )?;
        let (gaze_replay, _) = run_phase(
            checkpoint,
            TargetApp::lit(seed),
            ORDINARY_SCREEN_DEPTH,
            self.steps_per_phase,
        )?;
        let exact_replay = gaze_development == gaze_replay;
        let (_touch_development, after_touch) = run_phase(
            after_gaze,
            TargetApp::lit(seed.wrapping_add(2_000_003)),
            CLOSE_SCREEN_DEPTH,
            self.steps_per_phase,
        )?;
        let (tap_development, mut developed) = screen_use::run_development_phase(
            after_touch.clone(),
            TargetApp::lit_with_side(seed, BIG_TARGET_SIDE),
            self.steps_per_phase,
        )?;
        let (tap_replay, _) = screen_use::run_development_phase(
            after_touch,
            TargetApp::lit_with_side(seed, BIG_TARGET_SIDE),
            self.steps_per_phase,
        )?;
        let tap_exact_replay = tap_development == tap_replay;

        // Probe the acquired foundations before a later rung can change the
        // body. Every probe discards its own mutation.
        let gaze = run_phase(
            developed.clone(),
            TargetApp::lit(seed.wrapping_add(1_000_003)),
            ORDINARY_SCREEN_DEPTH,
            self.steps_per_phase,
        )?
        .0;
        let touch = run_phase(
            developed.clone(),
            TargetApp::lit(seed.wrapping_add(3_000_003)),
            ORDINARY_SCREEN_DEPTH,
            self.steps_per_phase,
        )?
        .0;
        let mut aimed_tap = screen_use::probe_aimed_tap(
            developed.clone(),
            seed,
            tap_development,
            self.steps_per_phase,
        )?;
        aimed_tap.exact_replay = tap_exact_replay;

        // Develop and probe one frontier at a time. A failed rung blocks all
        // later development, while its own developed checkpoint is retained
        // for focused diagnosis.
        let mut rungs = Vec::with_capacity(screen_use::RungKind::ALL.len());
        let mut blocked = !gaze.gaze_acquired()
            || !touch.touch_acquired()
            || aimed_tap.state != EvidenceState::Acquired
            || !aimed_tap.controls_quiet();
        for (index, kind) in (0_u64..).zip(screen_use::RungKind::ALL) {
            let observer_control = matches!(
                kind,
                screen_use::RungKind::Scan | screen_use::RungKind::QuietHand
            );
            if blocked && !observer_control {
                rungs.push(RungOutcome {
                    kind,
                    run: screen_use::RungRun {
                        development: None,
                        probes: Vec::new(),
                        controls: Vec::new(),
                        state: EvidenceState::Unknown,
                    },
                });
                continue;
            }
            let development = match screen_use::development_app(
                kind,
                seed.wrapping_add(4_000_003 + index * 250_000),
            ) {
                Some(app) => {
                    let (evidence, after) =
                        screen_use::run_rung_phase(developed, app, self.steps_per_phase)?;
                    developed = after;
                    Some(evidence)
                }
                None => None,
            };
            let mut run = screen_use::probe_rung(kind, &developed, seed, self.steps_per_phase)?;
            run.development = development;
            if !observer_control {
                blocked = run.state != EvidenceState::Acquired;
            }
            rungs.push(RungOutcome { kind, run });
        }

        let mut run = CourseRun {
            gaze,
            touch,
            aimed_tap,
            rungs,
            exact_replay,
            first_failure: None,
        };
        run.first_failure = Capability::ALL
            .into_iter()
            .find(|capability| run.state(*capability) != EvidenceState::Acquired)
            .or_else(|| (!run.aimed_tap.controls_quiet()).then_some(Capability::AimedTap));
        Ok((run, developed))
    }
}

fn run_phase(
    checkpoint: WorkstationCheckpoint,
    app: TargetApp,
    contact_depth: i16,
    steps: usize,
) -> Result<(PhaseEvidence, WorkstationCheckpoint), WorkstationError> {
    let target_band = app.layout().target_band;
    let mut session = Workstation2Session::with_world(
        checkpoint,
        Workstation2::with_target_at_depth(app, contact_depth),
    )?;
    let mut evidence = PhaseEvidence {
        steps,
        foveal_steps: 0,
        contact_steps: 0,
        taps: 0,
        target_taps: 0,
        chance: 0.0,
        physical_work: 0,
        naturally_quiescent: true,
    };
    let mut chance_sum = 0.0_f64;
    for _ in 0..steps {
        chance_sum += session.world().target().map_or(0.0, TargetApp::chance);
        let observation = session.step()?;
        let left = observation.sample.eye(Eye::Left).foveal().pixels();
        let fovea = left[left.len() / 2];
        evidence.foveal_steps += usize::from(fovea == target_band);
        evidence.contact_steps += usize::from(
            observation
                .sample
                .contacts()
                .iter()
                .any(|contact| contact.pressure() > 0),
        );
        evidence.physical_work = evidence
            .physical_work
            .saturating_add(observation.body.metrics.physical_work);
        evidence.naturally_quiescent &= observation.body.naturally_quiescent;
    }
    if let Some(app) = session.world().target() {
        evidence.taps = app.taps();
        evidence.target_taps = app.target_taps();
    }
    evidence.chance = chance_sum / steps as f64;
    Ok((evidence, session.body_checkpoint()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;
    use truelearner_workstation::WorkstationHarness;

    fn completed_course() -> &'static CourseRun {
        static RUN: OnceLock<CourseRun> = OnceLock::new();
        RUN.get_or_init(|| {
            let checkpoint = WorkstationHarness::new(11).unwrap().save().unwrap();
            Workstation2Course::new(256).run(checkpoint, 11).unwrap()
        })
    }

    #[test]
    fn arc_prerequisites_end_before_drag() {
        assert_eq!(
            Capability::ARC_PREREQUISITES.last(),
            Some(&Capability::Sequence)
        );
        assert!(!Capability::ARC_PREREQUISITES.contains(&Capability::Drag));
    }

    #[test]
    fn a_fresh_body_acquires_separate_looking_and_approach() {
        let run = completed_course();

        assert!(run.exact_replay);
        assert!(run.gaze.naturally_quiescent);
        assert!(run.touch.naturally_quiescent);
        // Foundations and observer controls remain acquired under local,
        // identifier-free coincidence.
        for capability in [
            Capability::Gaze,
            Capability::Touch,
            Capability::AimedTap,
            Capability::LiveKey,
            Capability::DeadKey,
            Capability::Sequence,
            Capability::Scan,
            Capability::QuietHand,
        ] {
            assert_eq!(
                run.state(capability),
                EvidenceState::Acquired,
                "{capability:?}: gaze={:?} touch={:?}",
                run.gaze,
                run.touch
            );
        }
        assert_eq!(run.state(Capability::Drag), EvidenceState::Unknown);
        assert_eq!(run.first_failure, Some(Capability::Drag));
        assert!(run.arc_prerequisites_acquired());
        // The killing control for the tap rung stays at chance.
        assert!(run.aimed_tap.controls_quiet());
    }

    #[test]
    fn the_learning_frontier_names_its_evidence() {
        let run = completed_course();
        let live_key = run
            .rungs
            .iter()
            .find(|outcome| outcome.kind == screen_use::RungKind::LiveKey)
            .unwrap();
        assert_eq!(live_key.run.state, EvidenceState::Acquired);
        let passing_pairs = live_key
            .run
            .probes
            .iter()
            .zip(&live_key.run.controls)
            .filter(|(probe, control)| {
                probe.prefers_the_reactive_key() && control.prefers_the_reactive_key()
            })
            .count();
        assert_eq!(passing_pairs, live_key.run.probes.len());
        let dead_key = run
            .rungs
            .iter()
            .find(|outcome| outcome.kind == screen_use::RungKind::DeadKey)
            .unwrap();
        assert_eq!(dead_key.run.state, EvidenceState::Acquired);
        assert!(dead_key.run.development.is_some());
        assert_eq!(dead_key.run.probes.len(), screen_use::PROBE_SEEDS);
        assert!(dead_key
            .run
            .probes
            .iter()
            .all(screen_use::RungEvidence::abandons_the_dead_key));
        assert!(dead_key
            .run
            .controls
            .iter()
            .all(screen_use::RungEvidence::keeps_tapping_a_live_key));
    }
}
