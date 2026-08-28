#![forbid(unsafe_code)]

use academy_workstation::{SessionObservation, WorkstationSession};
use serde::Serialize;
use std::collections::BTreeMap;
use std::str::FromStr;
use truelearner_workstation::{
    BodyAxis, Protocol, ResearchHarnessConfig, ResearchOpportunityIncidence,
    ResearchTransitionOpportunity,
};

const STEPS: usize = 48;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    ParentReference,
    SharedIncidence,
    AdoptedDefault,
    WaveSparseNeutral,
    ComposedWaveSparse,
}

impl Arm {
    pub const ALL: [Self; 5] = [
        Self::ParentReference,
        Self::SharedIncidence,
        Self::AdoptedDefault,
        Self::WaveSparseNeutral,
        Self::ComposedWaveSparse,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::ParentReference => "parent-reference",
            Self::SharedIncidence => "shared-incidence",
            Self::AdoptedDefault => "adopted-default",
            Self::WaveSparseNeutral => "wave-sparse-neutral",
            Self::ComposedWaveSparse => "composed-wave-sparse",
        }
    }
}

impl FromStr for Arm {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|arm| arm.id() == value)
            .ok_or(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MovementSummary {
    steps: usize,
    pose_changed_steps: u64,
    isolated_finger_steps: u64,
    five_finger_steps: u64,
    moved_fingers: Vec<String>,
    changed_axis_steps: BTreeMap<String, u64>,
    output_crossings: u64,
    device_events: u64,
    max_step_work: u64,
    final_body_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ProbeResult {
    schema: &'static str,
    pub arm: &'static str,
    pub outcome: &'static str,
    parent_revision: &'static str,
    protocol_sha256: &'static str,
    observations: serde_json::Value,
    falsifier: Option<String>,
    exact_replay: bool,
    naturally_quiescent: bool,
}

pub fn run(arm: Arm) -> ProbeResult {
    match arm {
        Arm::ParentReference => run_workstation(arm, SessionMode::IndependentResearch),
        Arm::SharedIncidence => run_workstation(arm, SessionMode::SharedResearch),
        Arm::AdoptedDefault => run_workstation(arm, SessionMode::Default),
        Arm::WaveSparseNeutral | Arm::ComposedWaveSparse => ProbeResult {
            schema: "workstation-digit-separation/v1",
            arm: arm.id(),
            outcome: "superseded",
            parent_revision: "00ef061f34b6e8a5e14bdce25a14a39f85f787f3",
            protocol_sha256: "39d10a0a9578f98d2555b789b00aa8dfd1cf411a38788f3a017b315c39a91e22",
            observations: serde_json::json!({
                "reason": "stopped after the smaller shared-incidence arm satisfied the frozen full-morphology predicate",
                "new_core_law_added": false,
            }),
            falsifier: None,
            exact_replay: true,
            naturally_quiescent: true,
        },
    }
}

#[derive(Clone, Copy)]
enum SessionMode {
    Default,
    IndependentResearch,
    SharedResearch,
}

fn run_workstation(arm: Arm, mode: SessionMode) -> ProbeResult {
    let seed = 82_001;
    let config = ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: match mode {
            SessionMode::IndependentResearch => ResearchOpportunityIncidence::Independent,
            SessionMode::Default | SessionMode::SharedResearch => {
                ResearchOpportunityIncidence::SharedWave
            }
        },
        transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
    };
    let mut session = match mode {
        SessionMode::Default => WorkstationSession::new(seed).expect("default session builds"),
        SessionMode::IndependentResearch | SessionMode::SharedResearch => {
            WorkstationSession::new_research(seed, config).expect("research session builds")
        }
    };
    let checkpoint = session.save().expect("initial checkpoint saves");
    let observations = run_steps(&mut session);
    let mut replay = match mode {
        SessionMode::Default => {
            WorkstationSession::restore(checkpoint).expect("default replay restores")
        }
        SessionMode::IndependentResearch => WorkstationSession::restore_research(
            checkpoint,
            ResearchOpportunityIncidence::Independent,
        )
        .expect("independent replay restores"),
        SessionMode::SharedResearch => WorkstationSession::restore_research(
            checkpoint,
            ResearchOpportunityIncidence::SharedWave,
        )
        .expect("shared replay restores"),
    };
    let replayed = run_steps(&mut replay);
    let exact_replay = observations == replayed && session.save().ok() == replay.save().ok();
    let naturally_quiescent = observations
        .iter()
        .all(|observation| observation.body.naturally_quiescent);
    let summary = summarize(&observations, &session);
    let reference_intact = summary.isolated_finger_steps == 0 && summary.five_finger_steps == 46;
    let shared = matches!(mode, SessionMode::Default | SessionMode::SharedResearch);
    let shared_passed = summary.isolated_finger_steps > 0
        && summary.moved_fingers.len() >= 2
        && summary.five_finger_steps == 0;
    let passed = if shared {
        shared_passed
    } else {
        reference_intact
    } && exact_replay
        && naturally_quiescent;
    ProbeResult {
        schema: "workstation-digit-separation/v1",
        arm: arm.id(),
        outcome: if shared && passed {
            "passed"
        } else if !shared && passed {
            "reference-reproduced"
        } else {
            "falsified"
        },
        parent_revision: "00ef061f34b6e8a5e14bdce25a14a39f85f787f3",
        protocol_sha256: "39d10a0a9578f98d2555b789b00aa8dfd1cf411a38788f3a017b315c39a91e22",
        observations: serde_json::to_value(&summary).expect("summary serializes"),
        falsifier: (!passed).then(|| {
            if shared {
                "shared incidence did not separate the full workstation morphology"
            } else {
                "parent reference counts drifted"
            }
            .to_string()
        }),
        exact_replay,
        naturally_quiescent,
    }
}

fn run_steps(session: &mut WorkstationSession) -> Vec<SessionObservation> {
    (0..STEPS)
        .map(|_| session.step().expect("workstation step succeeds"))
        .collect()
}

fn summarize(observations: &[SessionObservation], session: &WorkstationSession) -> MovementSummary {
    let mut pose_changed_steps = 0_u64;
    let mut isolated_finger_steps = 0_u64;
    let mut five_finger_steps = 0_u64;
    let mut moved_fingers = Vec::new();
    let mut changed_axis_steps = BTreeMap::<String, u64>::new();
    let mut output_crossings = 0_u64;
    let mut device_events = 0_u64;
    let mut max_step_work = 0_u64;
    for observation in observations {
        pose_changed_steps += u64::from(observation.body.pose_changed);
        output_crossings = output_crossings
            .saturating_add(u64::try_from(observation.body.crossings.len()).unwrap_or(u64::MAX));
        device_events = device_events
            .saturating_add(u64::try_from(observation.device_events.len()).unwrap_or(u64::MAX));
        max_step_work = max_step_work.max(observation.body.metrics.physical_work);
        let changed = observation
            .body
            .movements
            .iter()
            .filter(|movement| movement.changed)
            .collect::<Vec<_>>();
        for movement in &changed {
            *changed_axis_steps
                .entry(format!("{:?}", movement.axis))
                .or_default() += 1;
        }
        let fingers = changed
            .iter()
            .filter_map(|movement| match movement.axis {
                BodyAxis::FingerFlexion { digit } => Some(format!("{digit:?}")),
                _ => None,
            })
            .collect::<Vec<_>>();
        if fingers.len() == 1 {
            isolated_finger_steps += 1;
            if !moved_fingers.contains(&fingers[0]) {
                moved_fingers.push(fingers[0].clone());
            }
        }
        five_finger_steps += u64::from(fingers.len() == 5);
    }
    moved_fingers.sort();
    MovementSummary {
        steps: observations.len(),
        pose_changed_steps,
        isolated_finger_steps,
        five_finger_steps,
        moved_fingers,
        changed_axis_steps,
        output_crossings,
        device_events,
        max_step_work,
        final_body_fingerprint: session
            .read()
            .expect("final session reads")
            .body
            .body_fingerprint,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "full 48-step evidence run"]
    fn parent_reference_reproduces_frozen_counts() {
        let result = run(Arm::ParentReference);
        assert_eq!(result.outcome, "reference-reproduced");
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
    }

    #[test]
    #[ignore = "full 48-step evidence run"]
    fn shared_incidence_is_the_smallest_complete_solve() {
        let result = run(Arm::SharedIncidence);
        assert_eq!(result.outcome, "passed", "{:#?}", result.observations);
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
    }

    #[test]
    #[ignore = "full 48-step adoption evidence run"]
    fn adopted_default_reproduces_the_authorized_shared_result() {
        let result = run(Arm::AdoptedDefault);
        assert_eq!(result.outcome, "passed", "{:#?}", result.observations);
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
    }

    #[test]
    fn larger_candidate_arms_stop_after_smaller_solve() {
        for arm in [Arm::WaveSparseNeutral, Arm::ComposedWaveSparse] {
            let result = run(arm);
            assert_eq!(result.outcome, "superseded");
        }
    }
}
