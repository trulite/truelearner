//! Screen-use rungs on the target app. Aimed tap first.
use academy_workstation2::{TargetApp, Workstation2, Workstation2Session};
use serde::{Deserialize, Serialize};
use truelearner_workstation::{BodyAxis, Eye, WorkstationCheckpoint, WorkstationError};

pub const PROBE_SEEDS: usize = 3;
pub const MIN_TAPS: u32 = 20;
pub const CHANCE_MARGIN: f64 = 3.0;
/// The big toy for development: half the screen, so the first hit presents
/// within a few dozen steps and the consequence loop can find it.
pub const BIG_TARGET_SIDE: i16 = 512;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TapEvidence {
    pub steps: usize,
    pub taps: u32,
    pub target_taps: u32,
    pub hits: u32,
    pub chance: f64,
    pub gaze_changes: usize,
    /// Steps in which either retina carried target brightness.
    pub target_seen_steps: usize,
    /// Steps in which the target sat on the left fovea.
    pub target_foveal_steps: usize,
    /// Steps in which either retina carried the hand.
    pub hand_seen_steps: usize,
    /// Steps in which the hand sat on the left fovea.
    pub hand_foveal_steps: usize,
    pub contact_steps: usize,
    pub physical_work: u64,
    pub naturally_quiescent: bool,
    pub palm_x: (i16, i16),
    pub palm_y: (i16, i16),
    pub depth: (i16, i16),
}

impl TapEvidence {
    pub fn rate(&self) -> f64 {
        if self.taps == 0 {
            0.0
        } else {
            f64::from(self.target_taps) / f64::from(self.taps)
        }
    }

    /// Far above chance: enough taps and a rate at least CHANCE_MARGIN times
    /// the target's share of the screen.
    pub fn above_chance(&self) -> bool {
        self.taps >= MIN_TAPS && self.rate() >= CHANCE_MARGIN * self.chance
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RungState {
    Unknown,
    Emerging,
    Acquired,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AimedTapRun {
    pub development: TapEvidence,
    pub probes: Vec<TapEvidence>,
    pub blind_controls: Vec<TapEvidence>,
    pub exact_replay: bool,
    pub state: RungState,
}

impl AimedTapRun {
    /// The killing controls stayed at chance: no blind run beat it.
    pub fn controls_quiet(&self) -> bool {
        self.blind_controls
            .iter()
            .all(|control| !control.above_chance())
    }
}

pub struct ScreenUseCourse {
    steps_per_phase: usize,
}

impl ScreenUseCourse {
    pub fn new(steps_per_phase: usize) -> Self {
        Self {
            steps_per_phase: steps_per_phase.clamp(1, 4096),
        }
    }

    /// The whole rung from one checkpoint: big-toy development, then honest
    /// shrinking probes and blind controls from the developed body.
    pub fn aimed_tap(
        &self,
        checkpoint: WorkstationCheckpoint,
        seed: u64,
    ) -> Result<AimedTapRun, WorkstationError> {
        let (development, after) = run_development_phase(
            checkpoint.clone(),
            TargetApp::lit_with_side(seed, BIG_TARGET_SIDE),
            self.steps_per_phase,
        )?;
        let (replay, _) = run_development_phase(
            checkpoint,
            TargetApp::lit_with_side(seed, BIG_TARGET_SIDE),
            self.steps_per_phase,
        )?;
        let exact_replay = development == replay;
        let mut run = probe_aimed_tap(after, seed, development, self.steps_per_phase)?;
        run.exact_replay = exact_replay;
        Ok(run)
    }
}

/// The big-toy development phase: a target half the screen, so the first
/// hit presents within a few dozen steps and the consequence loop can find
/// it.
pub fn run_development_phase(
    checkpoint: WorkstationCheckpoint,
    app: TargetApp,
    steps: usize,
) -> Result<(TapEvidence, WorkstationCheckpoint), WorkstationError> {
    run_target_phase(checkpoint, app, steps)
}

/// The honest probes from a developed checkpoint: standard-size targets at
/// fresh positions, plus the invisible-target control that must stay at
/// chance.
pub fn probe_aimed_tap(
    developed: WorkstationCheckpoint,
    seed: u64,
    development: TapEvidence,
    steps: usize,
) -> Result<AimedTapRun, WorkstationError> {
    let mut probes = Vec::with_capacity(PROBE_SEEDS);
    let mut blind_controls = Vec::with_capacity(PROBE_SEEDS);
    for index in 0..PROBE_SEEDS as u64 {
        let probe_seed = seed.wrapping_add(1_000_003 * (index + 1));
        probes.push(run_target_phase(developed.clone(), TargetApp::lit(probe_seed), steps)?.0);
        blind_controls.push(
            run_target_phase(developed.clone(), TargetApp::lit(probe_seed).blind(), steps)?.0,
        );
    }
    let passed = probes.iter().filter(|probe| probe.above_chance()).count();
    let controls_quiet = blind_controls.iter().all(|control| !control.above_chance());
    let state = match (development.above_chance(), passed, controls_quiet) {
        (_, n, true) if n == PROBE_SEEDS => RungState::Acquired,
        (true, _, _) | (_, 1.., _) => RungState::Emerging,
        _ => RungState::Unknown,
    };
    Ok(AimedTapRun {
        development,
        probes,
        blind_controls,
        exact_replay: false,
        state,
    })
}

fn run_target_phase(
    checkpoint: WorkstationCheckpoint,
    app: TargetApp,
    steps: usize,
) -> Result<(TapEvidence, WorkstationCheckpoint), WorkstationError> {
    let mut session = Workstation2Session::with_world(checkpoint, Workstation2::with_target(app))?;
    let mut evidence = TapEvidence {
        steps,
        taps: 0,
        target_taps: 0,
        hits: 0,
        chance: 0.0,
        gaze_changes: 0,
        target_seen_steps: 0,
        target_foveal_steps: 0,
        hand_seen_steps: 0,
        hand_foveal_steps: 0,
        contact_steps: 0,
        physical_work: 0,
        naturally_quiescent: true,
        palm_x: (i16::MAX, i16::MIN),
        palm_y: (i16::MAX, i16::MIN),
        depth: (i16::MAX, i16::MIN),
    };
    let mut chance_sum = 0.0;
    for _ in 0..steps {
        chance_sum += session.world().target().map_or(0.0, TargetApp::chance);
        let observation = session.step()?;
        evidence.gaze_changes += observation
            .body
            .movements
            .iter()
            .filter(|movement| {
                movement.changed
                    && matches!(
                        movement.axis,
                        BodyAxis::EyeHorizontal { .. } | BodyAxis::EyeVertical { .. }
                    )
            })
            .count();
        let band = session.world().target().map(|app| app.layout().target_band);
        let sees = |value: u8| {
            Eye::ALL
                .into_iter()
                .any(|eye| observation.sample.eye(eye).pixels().contains(&value))
        };
        let left = observation.sample.eye(Eye::Left).pixels();
        let fovea = left[left.len() / 2];
        evidence.target_seen_steps += usize::from(band.is_some_and(sees));
        evidence.target_foveal_steps += usize::from(band == Some(fovea));
        evidence.hand_seen_steps += usize::from(sees(8));
        evidence.hand_foveal_steps += usize::from(fovea == 8);
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
        let palm = observation.body.state_after.hand().palm();
        evidence.palm_x = (
            evidence.palm_x.0.min(palm.x()),
            evidence.palm_x.1.max(palm.x()),
        );
        evidence.palm_y = (
            evidence.palm_y.0.min(palm.y()),
            evidence.palm_y.1.max(palm.y()),
        );
        evidence.depth = (
            evidence.depth.0.min(palm.depth()),
            evidence.depth.1.max(palm.depth()),
        );
    }
    let app = session.world().target().expect("target app");
    evidence.taps = app.taps();
    evidence.target_taps = app.target_taps();
    evidence.hits = app.hits();
    evidence.chance = chance_sum / steps as f64;
    Ok((evidence, session.body_checkpoint()?))
}

// ---------------------------------------------------------------------------
// The screen-use rungs beyond aimed tap. Each rung has a development phase
// that presents its consequence, fresh probes that discard mutation, and a
// control that would stay at chance if the claim were luck.
// ---------------------------------------------------------------------------

use academy_workstation2::ScreenPoint;

/// The rungs after aimed tap, in ladder order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RungKind {
    LiveKey,
    DeadKey,
    Scan,
    QuietHand,
    Sequence,
    Drag,
}

impl RungKind {
    pub const ALL: [Self; 6] = [
        Self::LiveKey,
        Self::DeadKey,
        Self::Scan,
        Self::QuietHand,
        Self::Sequence,
        Self::Drag,
    ];
}

/// Everything a rung measures, per phase. Fields a rung does not use stay
/// at their zero value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RungEvidence {
    pub steps: usize,
    pub contact_steps: usize,
    pub contact_steps_first_half: usize,
    pub contact_steps_second_half: usize,
    pub taps: u32,
    pub taps_first_half: u32,
    pub taps_second_half: u32,
    pub target_taps: u32,
    pub decoy_taps: u32,
    pub hits: u32,
    pub ab_pairs: u32,
    pub ba_pairs: u32,
    pub drag_attempts: u32,
    pub drag_hits: u32,
    pub death_step: Option<usize>,
    pub taps_at_death: u32,
    pub first_target_gaze_step: Option<usize>,
    pub first_target_tap_step: Option<usize>,
    pub goal_chance: f64,
    pub physical_work: u64,
    pub naturally_quiescent: bool,
}

impl RungEvidence {
    fn blank(steps: usize) -> Self {
        Self {
            steps,
            contact_steps: 0,
            contact_steps_first_half: 0,
            contact_steps_second_half: 0,
            taps: 0,
            taps_first_half: 0,
            taps_second_half: 0,
            target_taps: 0,
            decoy_taps: 0,
            hits: 0,
            ab_pairs: 0,
            ba_pairs: 0,
            drag_attempts: 0,
            drag_hits: 0,
            death_step: None,
            taps_at_death: 0,
            first_target_gaze_step: None,
            first_target_tap_step: None,
            goal_chance: 0.0,
            physical_work: 0,
            naturally_quiescent: true,
        }
    }

    /// The live-key claim: the reactive rectangle takes at least twice the
    /// taps of the inert one, with enough taps to count.
    pub fn prefers_the_reactive_key(&self) -> bool {
        self.target_taps >= 10 && self.target_taps >= 2 * self.decoy_taps
    }

    /// The dead-key claim: after the target went dead, taps on it fell to
    /// at most half the alive rate.
    pub fn abandons_the_dead_key(&self) -> bool {
        let Some(death) = self.death_step else {
            return false;
        };
        let alive_steps = (death + 1).max(1);
        let dead_steps = self.steps.saturating_sub(death + 1).max(1);
        let alive_rate = f64::from(self.taps_at_death.max(1)) / alive_steps as f64;
        let dead_taps = self.taps.saturating_sub(self.taps_at_death);
        let dead_rate = f64::from(dead_taps) / dead_steps as f64;
        dead_rate <= alive_rate / 2.0
    }

    /// The alive-control claim: a target that keeps reacting keeps its tap
    /// rate; the fall in the dead probe is due to death, not fatigue.
    pub fn keeps_tapping_a_live_key(&self) -> bool {
        let second = f64::from(self.taps_second_half);
        let first = f64::from(self.taps_first_half);
        second >= first / 2.0
    }

    /// The scan claim: the eyes reach the target before the first tap
    /// lands on it. Look, then act.
    pub fn looks_before_acting(&self) -> bool {
        match (self.first_target_gaze_step, self.first_target_tap_step) {
            (Some(gaze), Some(tap)) => gaze < tap,
            (Some(_), None) => true,
            _ => false,
        }
    }

    /// The quiet-hand claim: on a blank screen the palm stays far off the
    /// screen compared with a lit one.
    pub fn stays_quiet(&self, lit_contact_steps: usize) -> bool {
        lit_contact_steps > 0
            && self.contact_steps * 4 < lit_contact_steps
            && self.contact_steps_second_half <= self.contact_steps_first_half + 2
    }

    /// The sequence claim: the rewarded order exceeds its reverse, with
    /// enough completions to count.
    pub fn follows_the_order(&self) -> bool {
        self.ab_pairs >= 5 && self.ab_pairs > self.ba_pairs
    }

    /// The drag claim: releases land on the goal far above its share of
    /// the screen, with enough attempts to count.
    pub fn drags_to_the_goal(&self) -> bool {
        self.drag_attempts >= 5
            && f64::from(self.drag_hits)
                >= CHANCE_MARGIN * self.goal_chance * f64::from(self.drag_attempts)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RungRun {
    pub development: Option<RungEvidence>,
    pub probes: Vec<RungEvidence>,
    pub controls: Vec<RungEvidence>,
    pub state: RungState,
}

/// The development app that presents a rung's consequence, if it has one.
pub fn development_app(kind: RungKind, seed: u64) -> Option<TargetApp> {
    match kind {
        // The big-toy live key: the pair abuts, so midpoint taps still land
        // on a key and the reactive/inert asymmetry presents. The value
        // links can then earn their strength.
        RungKind::LiveKey => Some(TargetApp::dual_with_side(seed, BIG_TARGET_SIDE)),
        RungKind::DeadKey => Some(TargetApp::lit(seed).dies_after(5)),
        RungKind::Scan => None,
        RungKind::QuietHand => None,
        RungKind::Sequence => Some(TargetApp::sequence(seed)),
        RungKind::Drag => Some(TargetApp::drag(seed)),
    }
}

fn probe_app(kind: RungKind, seed: u64) -> TargetApp {
    match kind {
        RungKind::LiveKey => TargetApp::dual(seed),
        RungKind::DeadKey => TargetApp::lit(seed).dies_after(5),
        RungKind::Scan => TargetApp::lit(seed),
        RungKind::QuietHand => TargetApp::blank(seed),
        RungKind::Sequence => TargetApp::sequence(seed),
        RungKind::Drag => TargetApp::drag(seed),
    }
}

fn control_app(kind: RungKind, seed: u64) -> TargetApp {
    match kind {
        RungKind::LiveKey => TargetApp::dual(seed).swapped(),
        RungKind::DeadKey => TargetApp::lit(seed),
        RungKind::Scan => TargetApp::lit(seed).blind(),
        RungKind::QuietHand => TargetApp::lit(seed),
        RungKind::Sequence => TargetApp::sequence(seed).reversed(),
        RungKind::Drag => TargetApp::drag(seed).blind(),
    }
}

fn probe_claim(kind: RungKind, evidence: &RungEvidence) -> bool {
    match kind {
        RungKind::LiveKey => evidence.prefers_the_reactive_key(),
        RungKind::DeadKey => evidence.abandons_the_dead_key(),
        RungKind::Scan => evidence.looks_before_acting(),
        RungKind::QuietHand => true, // measured against the lit control below
        RungKind::Sequence => evidence.follows_the_order(),
        RungKind::Drag => evidence.drags_to_the_goal(),
    }
}

fn control_claim(kind: RungKind, control: &RungEvidence, probe: &RungEvidence) -> bool {
    match kind {
        RungKind::LiveKey => control.prefers_the_reactive_key(),
        RungKind::DeadKey => control.keeps_tapping_a_live_key(),
        RungKind::Scan => match (probe.first_target_gaze_step, control.first_target_gaze_step) {
            (Some(probe_step), Some(control_step)) => probe_step < control_step,
            (Some(_), None) => true,
            _ => false,
        },
        RungKind::QuietHand => probe.stays_quiet(control.contact_steps),
        RungKind::Sequence => !(control.ab_pairs >= 5 && control.ab_pairs > control.ba_pairs),
        RungKind::Drag => !control.drags_to_the_goal(),
    }
}

/// All probes and controls for one rung, from a developed checkpoint that
/// the probes never mutate.
pub fn probe_rung(
    kind: RungKind,
    developed: &WorkstationCheckpoint,
    seed: u64,
    steps: usize,
) -> Result<RungRun, WorkstationError> {
    let mut probes = Vec::with_capacity(PROBE_SEEDS);
    let mut controls = Vec::with_capacity(PROBE_SEEDS);
    for index in 0..PROBE_SEEDS as u64 {
        let probe_seed = seed.wrapping_add(1_000_003 * (index + 1));
        probes.push(run_rung_phase(developed.clone(), probe_app(kind, probe_seed), steps)?.0);
        controls.push(run_rung_phase(developed.clone(), control_app(kind, probe_seed), steps)?.0);
    }
    let passed = probes
        .iter()
        .zip(&controls)
        .filter(|(probe, control)| probe_claim(kind, probe) && control_claim(kind, control, probe))
        .count();
    let state = match passed {
        n if n == PROBE_SEEDS => RungState::Acquired,
        1.. => RungState::Emerging,
        _ => RungState::Unknown,
    };
    Ok(RungRun {
        development: None,
        probes,
        controls,
        state,
    })
}

pub fn run_rung_phase(
    checkpoint: WorkstationCheckpoint,
    app: TargetApp,
    steps: usize,
) -> Result<(RungEvidence, WorkstationCheckpoint), WorkstationError> {
    let mut evidence = RungEvidence::blank(steps);
    let goal_chance = app.goal_chance();
    let mut session = Workstation2Session::with_world(checkpoint, Workstation2::with_target(app))?;
    let mut was_reactive = session
        .world()
        .target()
        .is_some_and(|t| t.layout().reactive);
    for step in 0..steps {
        let layout = session.world().target().map(|t| t.layout());
        let taps_before = session.world().target().map_or(0, |t| t.taps());
        let target_taps_before = session
            .world()
            .target()
            .map_or(0, |target| target.target_taps());
        let observation = session.step()?;
        let app = session.world().target().expect("target app");
        let total_taps = app.taps();
        if total_taps > taps_before {
            let gained = total_taps - taps_before;
            if step < steps / 2 {
                evidence.taps_first_half += gained;
            } else {
                evidence.taps_second_half += gained;
            }
        }
        let gaze = observation.body.state_after.eye(Eye::Left).gaze();
        let in_target = layout.is_some_and(|l| {
            l.target.is_some_and(|rect| {
                rect.contains(ScreenPoint {
                    x: gaze.x(),
                    y: gaze.y(),
                })
            })
        });
        if evidence.first_target_gaze_step.is_none() && in_target {
            evidence.first_target_gaze_step = Some(step);
        }
        if evidence.first_target_tap_step.is_none() && app.target_taps() > target_taps_before {
            evidence.first_target_tap_step = Some(step);
        }
        let reactive_now = app.layout().reactive;
        if evidence.death_step.is_none() && was_reactive && !reactive_now {
            evidence.death_step = Some(step);
            evidence.taps_at_death = total_taps;
        }
        was_reactive = reactive_now;
        let touching = observation
            .sample
            .contacts()
            .iter()
            .any(|contact| contact.pressure() > 0);
        evidence.contact_steps += usize::from(touching);
        if step < steps / 2 {
            evidence.contact_steps_first_half += usize::from(touching);
        } else {
            evidence.contact_steps_second_half += usize::from(touching);
        }
        evidence.physical_work = evidence
            .physical_work
            .saturating_add(observation.body.metrics.physical_work);
        evidence.naturally_quiescent &= observation.body.naturally_quiescent;
    }
    if let Some(app) = session.world().target() {
        evidence.taps = app.taps();
        evidence.target_taps = app.target_taps();
        evidence.decoy_taps = app.decoy_taps();
        evidence.hits = app.hits();
        evidence.ab_pairs = app.ab_pairs();
        evidence.ba_pairs = app.ba_pairs();
        evidence.drag_attempts = app.drag_attempts();
        evidence.drag_hits = app.drag_hits();
    }
    evidence.goal_chance = goal_chance;
    // Half-split taps from the per-step deltas recorded above.
    Ok((evidence, session.body_checkpoint()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_requires_target_gaze_before_target_tap() {
        let mut evidence = RungEvidence::blank(64);
        evidence.first_target_gaze_step = Some(7);
        evidence.first_target_tap_step = Some(12);
        assert!(evidence.looks_before_acting());

        evidence.first_target_gaze_step = Some(13);
        assert!(!evidence.looks_before_acting());
    }

    #[test]
    fn scan_control_compares_when_the_hidden_target_is_found() {
        let mut probe = RungEvidence::blank(64);
        probe.first_target_gaze_step = Some(7);
        let mut control = RungEvidence::blank(64);

        assert!(control_claim(RungKind::Scan, &control, &probe));
        control.first_target_gaze_step = Some(31);
        assert!(control_claim(RungKind::Scan, &control, &probe));
        control.first_target_gaze_step = Some(3);
        assert!(!control_claim(RungKind::Scan, &control, &probe));
    }
}
