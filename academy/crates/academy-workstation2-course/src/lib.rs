#![forbid(unsafe_code)]
//! Development and fresh-probe evidence for the touchscreen workstation.

use academy_workstation2::{DeviceEvent, Workstation2Observation, Workstation2Session};
use serde::{Deserialize, Serialize};
use truelearner_workstation::{BodyAxis, WorkstationCheckpoint, WorkstationError};

const MAX_STEPS: usize = 512;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Capability {
    Gaze,
    Touch,
    VirtualKey,
    Pinch,
}

impl Capability {
    pub const ALL: [Self; 4] = [Self::Gaze, Self::Touch, Self::VirtualKey, Self::Pinch];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceState {
    Unknown,
    Emerging,
    Acquired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhaseEvidence {
    pub steps: usize,
    pub gaze_changes: usize,
    pub touch_starts: usize,
    pub virtual_key_changes: usize,
    pub pinch_changes: usize,
    pub physical_work: u64,
    pub naturally_quiescent: bool,
    pub final_body_fingerprint: String,
}

impl PhaseEvidence {
    pub fn observed(&self, capability: Capability) -> bool {
        match capability {
            Capability::Gaze => self.gaze_changes > 0,
            Capability::Touch => self.touch_starts > 0,
            Capability::VirtualKey => self.virtual_key_changes > 0,
            Capability::Pinch => self.pinch_changes > 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CourseRun {
    pub development: PhaseEvidence,
    pub shifted_probe: PhaseEvidence,
    pub probe_keyboard_shift: i16,
    pub exact_replay: bool,
    pub first_failure: Option<Capability>,
}

impl CourseRun {
    pub fn state(&self, capability: Capability) -> EvidenceState {
        match (
            self.development.observed(capability),
            self.shifted_probe.observed(capability),
        ) {
            (false, _) => EvidenceState::Unknown,
            (true, false) => EvidenceState::Emerging,
            (true, true) => EvidenceState::Acquired,
        }
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
        let shift = if seed & 1 == 0 { 96 } else { -96 };
        let development = run_phase(checkpoint.clone(), 0, self.steps_per_phase)?;
        let replay = run_phase(checkpoint, 0, self.steps_per_phase)?;
        let exact_replay = development.observations == replay.observations;
        let shifted_probe = run_phase(development.checkpoint_after, shift, self.steps_per_phase)?;
        let first_failure = Capability::ALL.into_iter().find(|capability| {
            !development.evidence.observed(*capability)
                || !shifted_probe.evidence.observed(*capability)
        });
        Ok(CourseRun {
            development: development.evidence,
            shifted_probe: shifted_probe.evidence,
            probe_keyboard_shift: shift,
            exact_replay,
            first_failure,
        })
    }
}

struct PhaseRun {
    evidence: PhaseEvidence,
    checkpoint_after: WorkstationCheckpoint,
    observations: Vec<Workstation2Observation>,
}

fn run_phase(
    checkpoint: WorkstationCheckpoint,
    keyboard_shift: i16,
    steps: usize,
) -> Result<PhaseRun, WorkstationError> {
    let mut session = Workstation2Session::from_checkpoint(checkpoint, keyboard_shift)?;
    let mut observations = Vec::with_capacity(steps);
    let mut gaze_changes = 0;
    let mut touch_starts = 0;
    let mut virtual_key_changes = 0;
    let mut pinch_changes = 0;
    let mut physical_work = 0_u64;
    let mut naturally_quiescent = true;
    let mut prior_text = String::new();
    let mut prior_scale = 128;
    for _ in 0..steps {
        let observation = session.step()?;
        gaze_changes += observation
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
        touch_starts += observation
            .device_events
            .iter()
            .filter(|event| matches!(event, DeviceEvent::TouchStarted { .. }))
            .count();
        virtual_key_changes += usize::from(observation.text != prior_text);
        pinch_changes += usize::from(observation.scale != prior_scale);
        physical_work = physical_work.saturating_add(observation.body.metrics.physical_work);
        naturally_quiescent &= observation.body.naturally_quiescent;
        prior_text.clone_from(&observation.text);
        prior_scale = observation.scale;
        observations.push(observation);
    }
    let final_body_fingerprint = observations
        .last()
        .map(|observation| observation.body.body_fingerprint.clone())
        .unwrap_or_default();
    let checkpoint_after = session.body_checkpoint()?;
    Ok(PhaseRun {
        evidence: PhaseEvidence {
            steps,
            gaze_changes,
            touch_starts,
            virtual_key_changes,
            pinch_changes,
            physical_work,
            naturally_quiescent,
            final_body_fingerprint,
        },
        checkpoint_after,
        observations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use truelearner_workstation::WorkstationHarness;

    #[test]
    fn course_replays_and_reports_the_first_honest_frontier() {
        let checkpoint = WorkstationHarness::new(11).unwrap().save().unwrap();
        let run = Workstation2Course::new(24).run(checkpoint, 11).unwrap();

        assert!(run.exact_replay);
        assert!(run.development.naturally_quiescent);
        assert!(run.shifted_probe.naturally_quiescent);
        assert!(run.first_failure.is_some());
    }

    #[test]
    fn a_fresh_body_acquires_every_rung_by_sweeping() {
        let checkpoint = WorkstationHarness::new(11).unwrap().save().unwrap();
        let run = Workstation2Course::new(256).run(checkpoint, 11).unwrap();

        assert!(run.exact_replay);
        assert_eq!(run.first_failure, None);
        for capability in Capability::ALL {
            assert_eq!(
                run.state(capability),
                EvidenceState::Acquired,
                "{capability:?}"
            );
        }
    }

    #[test]
    fn shifted_probe_uses_a_fresh_external_layout() {
        let checkpoint = WorkstationHarness::new(12).unwrap().save().unwrap();
        let run = Workstation2Course::new(8).run(checkpoint, 12).unwrap();

        assert_eq!(run.probe_keyboard_shift, 96);
    }
}
