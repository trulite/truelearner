#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    EffectComposition, ExistingWitnessEvent, ExistingWitnessTraceEntry,
    OutputChoiceResolutionEvidence, ReflectedHandProtocolEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::collections::BTreeSet;
use truelearner_core::{
    CompletedCycleState, JunctionId, LearnerId, LinkId, OutputAdmission, OutputChoiceBasis,
    Protocol,
};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    FullSnapshotControl,
    TickFortySevenLinkComposition,
}

impl Arm {
    pub const ALL: [Self; 2] = [
        Self::FullSnapshotControl,
        Self::TickFortySevenLinkComposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::FullSnapshotControl => "full-snapshot-control",
            Self::TickFortySevenLinkComposition => "tick-forty-seven-link-composition",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Verdict {
    Composed,
    ChoiceMismatch,
    MissingConstruction,
    NoCompletingProjection,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Projection {
    link: LinkId,
    generation: u32,
    consequence_tick: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Localization {
    verdict: Verdict,
    choice: Option<OutputChoiceResolutionEvidence>,
    projections: Vec<Projection>,
    completing_links: Vec<(LinkId, u32)>,
    reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Summary {
    steps: usize,
    actual_position_changes: usize,
    opposing_output_steps: usize,
    final_position: i16,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    completed_cycle_admissions: usize,
    output_choice_resolutions: usize,
    propagation_budget_exhaustions: u64,
    stopped: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
}

impl Summary {
    fn exact(&self) -> bool {
        self.steps == 16
            && self.actual_position_changes == 12
            && self.opposing_output_steps == 4
            && self.final_position == -2
            && !self.reached_lower
            && !self.reached_upper
            && !self.escaped_lower
            && !self.escaped_upper
            && self.completed_cycle_admissions == 10
            && self.output_choice_resolutions == 24
            && self.propagation_budget_exhaustions == 0
            && !self.stopped
            && self.exact_replay
            && self.naturally_quiescent
    }
}

fn expected_choice(choice: &OutputChoiceResolutionEvidence) -> bool {
    choice.tick == 47
        && choice.phase == 0
        && choice.admitted
            == vec![OutputAdmission {
                target: JunctionId(10),
                owner: Some(LearnerId(3)),
            }]
        && choice.admission_basis == OutputChoiceBasis::CompletedCycle
        && choice.completed_cycle_state == CompletedCycleState::Unique
}

fn localize(
    choices: &[OutputChoiceResolutionEvidence],
    trace: &[ExistingWitnessTraceEntry],
) -> Localization {
    let choice = choices
        .iter()
        .find(|choice| choice.tick == 47 && choice.phase == 0)
        .cloned();
    if !choice.as_ref().is_some_and(expected_choice) {
        return Localization {
            verdict: Verdict::ChoiceMismatch,
            choice,
            projections: Vec::new(),
            completing_links: Vec::new(),
            reason: "tick-forty-seven choice is not learner-three target-ten Unique under CompletedCycle"
                .to_owned(),
        };
    }

    let constructed = trace.iter().any(|entry| {
        entry.tick == 44
            && matches!(
                entry.event,
                ExistingWitnessEvent::LearnerConstructed {
                    learner: LearnerId(3),
                    ..
                }
            )
    });
    let projections = trace
        .iter()
        .filter_map(|entry| match entry.event {
            ExistingWitnessEvent::LearnerConsequenceRecorded {
                owner: LearnerId(3),
                link,
                generation,
                consequence_tick: 44,
            } if entry.tick == 44 => Some(Projection {
                link,
                generation,
                consequence_tick: 44,
            }),
            _ => None,
        })
        .collect::<Vec<_>>();
    if !constructed || projections.is_empty() {
        return Localization {
            verdict: Verdict::MissingConstruction,
            choice,
            projections,
            completing_links: Vec::new(),
            reason: "learner-three construction or its tick-forty-four projections are missing"
                .to_owned(),
        };
    }

    let completing_drives = trace
        .iter()
        .filter_map(|entry| match &entry.event {
            ExistingWitnessEvent::DriveProvenanceObserved(drive)
                if entry.tick == 47 && drive.target == JunctionId(10) && drive.completes_path =>
            {
                drive.link
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let lineage = trace
        .iter()
        .filter_map(|entry| match entry.event {
            ExistingWitnessEvent::CausalLineageMemberObserved {
                target: JunctionId(10),
                link: Some(link),
                generation: Some(generation),
                ..
            } if entry.tick == 47 => Some((link, generation)),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let completing_links = projections
        .iter()
        .filter(|projection| {
            completing_drives.contains(&projection.link)
                && lineage.contains(&(projection.link, projection.generation))
        })
        .map(|projection| (projection.link, projection.generation))
        .collect::<Vec<_>>();
    Localization {
        verdict: if completing_links.is_empty() {
            Verdict::NoCompletingProjection
        } else {
            Verdict::Composed
        },
        choice,
        projections,
        reason: if completing_links.is_empty() {
            "no tick-forty-four projection composes by same link and generation into target ten"
        } else {
            "the named tick-forty-four projections compose by same link and generation into target ten"
        }
        .to_owned(),
        completing_links,
    }
}

fn summarize(hand: &ReflectedHandProtocolEvidence) -> Summary {
    Summary {
        steps: hand.steps,
        actual_position_changes: hand.actual_position_changes,
        opposing_output_steps: hand
            .trajectory
            .iter()
            .filter(|step| {
                step.emitted_outputs.contains(&20_000) && step.emitted_outputs.contains(&20_001)
            })
            .count(),
        final_position: hand.final_position,
        reached_lower: hand.reached_lower,
        reached_upper: hand.reached_upper,
        escaped_lower: hand.escaped_lower,
        escaped_upper: hand.escaped_upper,
        completed_cycle_admissions: hand
            .trajectory
            .iter()
            .flat_map(|step| &step.completed_cycle_continuations)
            .filter(|effect| effect.admitted)
            .count(),
        output_choice_resolutions: hand
            .trajectory
            .iter()
            .map(|step| step.output_choice_resolutions.len())
            .sum(),
        propagation_budget_exhaustions: hand
            .trajectory
            .iter()
            .map(|step| step.propagation_budget_exhaustions)
            .sum(),
        stopped: hand.stopped,
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
    }
}

#[derive(Clone, Debug, Serialize)]
struct Evidence {
    summary: Summary,
    localization: Localization,
    snapshot: ReflectedHandProtocolEvidence,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProbeResult {
    schema: &'static str,
    pub arm: &'static str,
    pub outcome: &'static str,
    pub observations: serde_json::Value,
    pub falsifier: Option<String>,
    pub exact_replay: bool,
    pub naturally_quiescent: bool,
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    let snapshot = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerConstructionOutcomeComposition,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    debug_assert_eq!(snapshot.effect_composition, EffectComposition::Batched);
    let choices = snapshot
        .trajectory
        .iter()
        .flat_map(|step| step.output_choice_resolutions.iter().cloned())
        .collect::<Vec<_>>();
    let trace = snapshot
        .trajectory
        .iter()
        .flat_map(|step| step.existing_witness_trace.iter().cloned())
        .collect::<Vec<_>>();
    let evidence = Evidence {
        summary: summarize(&snapshot),
        localization: localize(&choices, &trace),
        snapshot,
    };
    Arm::ALL
        .into_iter()
        .map(|arm| {
            let survived = match arm {
                Arm::FullSnapshotControl => evidence.summary.exact(),
                Arm::TickFortySevenLinkComposition => {
                    evidence.summary.exact() && evidence.localization.verdict == Verdict::Composed
                }
            };
            let falsifier = match arm {
                Arm::FullSnapshotControl => "the unconditional snapshot or exact hand summary changed",
                Arm::TickFortySevenLinkComposition => {
                    "no same-generation tick-forty-four projection completed learner-three target ten at tick forty-seven"
                }
            };
            (
                arm,
                ProbeResult {
                    schema: "hand-unconditional-existing-trace-localization/v1",
                    arm: arm.id(),
                    outcome: if survived { "survived" } else { "falsified" },
                    observations: serde_json::to_value(&evidence).expect("evidence serializes"),
                    falsifier: (!survived).then(|| falsifier.to_owned()),
                    exact_replay: evidence.summary.exact_replay,
                    naturally_quiescent: evidence.summary.naturally_quiescent,
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use developmental_hand_construction_admission::DriveProvenanceEvidence;
    use truelearner_core::TransmissionMode;

    fn choice() -> OutputChoiceResolutionEvidence {
        OutputChoiceResolutionEvidence {
            tick: 47,
            phase: 0,
            ordinary_target: JunctionId(11),
            current_transition_target: None,
            coherent_effect_target: None,
            completed_cycle_target: Some(JunctionId(10)),
            computed_winner_target: JunctionId(10),
            admitted: vec![OutputAdmission {
                target: JunctionId(10),
                owner: Some(LearnerId(3)),
            }],
            computed_winner_basis: OutputChoiceBasis::CompletedCycle,
            admission_basis: OutputChoiceBasis::CompletedCycle,
            completed_cycle_state: CompletedCycleState::Unique,
            crosses_ownership_view: false,
        }
    }

    fn entry(tick: i64, event: ExistingWitnessEvent) -> ExistingWitnessTraceEntry {
        ExistingWitnessTraceEntry {
            tick,
            phase: 0,
            event,
        }
    }

    fn composed_trace() -> Vec<ExistingWitnessTraceEntry> {
        vec![
            entry(
                44,
                ExistingWitnessEvent::LearnerConstructed {
                    learner: LearnerId(3),
                    parent: Some(LearnerId(2)),
                    surface: JunctionId(20),
                    output: JunctionId(10),
                    junction_count: 1,
                    link_count: 1,
                },
            ),
            entry(
                44,
                ExistingWitnessEvent::LearnerConsequenceRecorded {
                    owner: LearnerId(3),
                    link: LinkId(34),
                    generation: 3,
                    consequence_tick: 44,
                },
            ),
            entry(
                47,
                ExistingWitnessEvent::DriveProvenanceObserved(DriveProvenanceEvidence {
                    ordinal: 0,
                    tick: 47,
                    phase: 0,
                    causal_wave: 0,
                    source: Some(JunctionId(20)),
                    target: JunctionId(10),
                    source_physical: Some(40_000),
                    target_physical: 20_000,
                    source_region: Some(0),
                    target_region: 0,
                    is_motor: true,
                    link: Some(LinkId(34)),
                    completes_path: true,
                    carried_origin: 10_000,
                    origin_owner: Some(LearnerId(3)),
                    path_owner: Some(LearnerId(3)),
                    strength: 1,
                }),
            ),
            entry(
                47,
                ExistingWitnessEvent::CausalLineageMemberObserved {
                    target: JunctionId(10),
                    origin_physical: 10_000,
                    mode: TransmissionMode::Drive,
                    link: Some(LinkId(34)),
                    generation: Some(3),
                    causal_wave: 0,
                },
            ),
        ]
    }

    #[test]
    fn composes_only_the_same_link_and_generation() {
        let localized = localize(&[choice()], &composed_trace());
        assert_eq!(localized.verdict, Verdict::Composed);
        assert_eq!(localized.completing_links, vec![(LinkId(34), 3)]);
    }

    #[test]
    fn rejects_wrong_generation_without_losing_projection() {
        let mut trace = composed_trace();
        if let ExistingWitnessEvent::CausalLineageMemberObserved { generation, .. } =
            &mut trace[3].event
        {
            *generation = Some(4);
        }
        let localized = localize(&[choice()], &trace);
        assert_eq!(localized.verdict, Verdict::NoCompletingProjection);
        assert_eq!(localized.projections.len(), 1);
        assert!(localized.completing_links.is_empty());
    }

    #[test]
    fn rejects_choice_change_before_physical_composition() {
        let mut changed = choice();
        changed.admitted[0].target = JunctionId(11);
        let localized = localize(&[changed], &composed_trace());
        assert_eq!(localized.verdict, Verdict::ChoiceMismatch);
    }
}
