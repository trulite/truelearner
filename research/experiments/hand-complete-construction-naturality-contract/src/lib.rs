#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    CompletedCycleContinuationEvidence, EffectComposition, ExistingWitnessEvent,
    ExistingWitnessTraceEntry, ReflectedHandProtocolEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use truelearner_core::{JunctionId, LearnerId, LinkId, Protocol};

const PARENT_SHA256: &str = "e4c38011cc9ff198474f553f72b6e4e7b366f3201a83fc114acd7c38a20e04d3";
const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    ExactWitnessInertness,
    CompleteConstructionNaturality,
    SelectivityIntegrationControl,
}

impl Arm {
    pub const ALL: [Self; 3] = [
        Self::ExactWitnessInertness,
        Self::CompleteConstructionNaturality,
        Self::SelectivityIntegrationControl,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::ExactWitnessInertness => "exact-witness-inertness",
            Self::CompleteConstructionNaturality => "complete-construction-naturality",
            Self::SelectivityIntegrationControl => "selectivity-integration-control",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Projection {
    construction_tick: i64,
    owner: LearnerId,
    link: LinkId,
    generation: u32,
    consequence_tick: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum FactorizationVerdict {
    Composed,
    MissingWitness,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ConstructionBornAdmission {
    tick: i64,
    target: JunctionId,
    owner: LearnerId,
    consequence_tick: i64,
    consequence_witnesses: Vec<(LinkId, u32)>,
    projected_links: Vec<(LinkId, u32)>,
    composed_links: Vec<(LinkId, u32)>,
    verdict: FactorizationVerdict,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ParentControl {
    sha256: String,
    expected_sha256: &'static str,
    choice_count: usize,
    unique_count: usize,
    survived: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Summary {
    protocol: Protocol,
    effect_composition: EffectComposition,
    steps: usize,
    actual_position_changes: usize,
    opposing_output_steps: usize,
    final_position: i16,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    completed_cycle_admissions: usize,
    cross_view_admissions: usize,
    output_choice_resolutions: usize,
    propagation_budget_exhaustions: u64,
    stopped: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
}

impl Summary {
    fn exact(&self) -> bool {
        self.protocol == Protocol::RecursiveLearnerConstructionOutcomeComposition
            && self.effect_composition == EffectComposition::Batched
            && self.steps == 16
            && self.actual_position_changes == 12
            && self.opposing_output_steps == 4
            && self.final_position == -2
            && !self.reached_lower
            && !self.reached_upper
            && !self.escaped_lower
            && !self.escaped_upper
            && self.completed_cycle_admissions == 10
            && self.cross_view_admissions == 2
            && self.output_choice_resolutions == 24
            && self.propagation_budget_exhaustions == 0
            && !self.stopped
            && self.exact_replay
            && self.naturally_quiescent
    }
}

fn parent_control(bytes: &[u8]) -> ParentControl {
    let sha256 = format!("{:x}", Sha256::digest(bytes));
    let parsed = serde_json::from_slice::<serde_json::Value>(bytes).ok();
    let choices = parsed
        .as_ref()
        .and_then(|value| value.pointer("/observations/naturality/ordered_choices"))
        .and_then(serde_json::Value::as_array);
    let choice_count = choices.map_or(0, Vec::len);
    let unique_count = choices.map_or(0, |choices| {
        choices
            .iter()
            .filter(|choice| {
                choice
                    .get("completed_cycle_state")
                    .and_then(serde_json::Value::as_str)
                    == Some("Unique")
            })
            .count()
    });
    ParentControl {
        survived: sha256 == PARENT_SHA256 && choice_count == 23 && unique_count == 9,
        sha256,
        expected_sha256: PARENT_SHA256,
        choice_count,
        unique_count,
    }
}

fn construction_projections(trace: &[ExistingWitnessTraceEntry]) -> Vec<Projection> {
    trace
        .iter()
        .enumerate()
        .flat_map(|(index, entry)| {
            let ExistingWitnessEvent::LearnerConstructed { learner, .. } = entry.event else {
                return Vec::new();
            };
            trace
                .iter()
                .skip(index.saturating_add(1))
                .map_while(|following| match following.event {
                    ExistingWitnessEvent::LearnerConsequenceRecorded {
                        owner,
                        link,
                        generation,
                        consequence_tick,
                    } if owner == learner => Some(Projection {
                        construction_tick: entry.tick,
                        owner,
                        link,
                        generation,
                        consequence_tick,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn factor_construction_born(
    completed: &[CompletedCycleContinuationEvidence],
    projections: &[Projection],
) -> Vec<ConstructionBornAdmission> {
    let construction_keys = projections
        .iter()
        .map(|projection| (projection.owner, projection.construction_tick))
        .collect::<BTreeSet<_>>();
    completed
        .iter()
        .filter(|effect| effect.admitted)
        .filter_map(|effect| {
            let owner = effect.owner?;
            let consequence_tick = effect.consequence_tick?;
            construction_keys
                .contains(&(owner, consequence_tick))
                .then_some((effect, owner, consequence_tick))
        })
        .map(|(effect, owner, consequence_tick)| {
            let projected_links = projections
                .iter()
                .filter(|projection| {
                    projection.owner == owner
                        && projection.construction_tick == consequence_tick
                        && projection.consequence_tick == consequence_tick
                })
                .map(|projection| (projection.link, projection.generation))
                .collect::<Vec<_>>();
            let composed_links = effect
                .consequence_witnesses
                .iter()
                .copied()
                .filter(|witness| projected_links.contains(witness))
                .collect::<Vec<_>>();
            ConstructionBornAdmission {
                tick: effect.tick,
                target: effect.target,
                owner,
                consequence_tick,
                consequence_witnesses: effect.consequence_witnesses.clone(),
                projected_links,
                verdict: if composed_links.is_empty() {
                    FactorizationVerdict::MissingWitness
                } else {
                    FactorizationVerdict::Composed
                },
                composed_links,
            }
        })
        .collect()
}

fn summarize(hand: &ReflectedHandProtocolEvidence) -> Summary {
    let completed = hand
        .trajectory
        .iter()
        .flat_map(|step| &step.completed_cycle_continuations)
        .collect::<Vec<_>>();
    Summary {
        protocol: hand.protocol,
        effect_composition: hand.effect_composition,
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
        completed_cycle_admissions: completed.iter().filter(|effect| effect.admitted).count(),
        cross_view_admissions: completed
            .iter()
            .filter(|effect| effect.admitted && effect.crosses_ownership_view)
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
    parent_control: ParentControl,
    summary: Summary,
    projections: Vec<Projection>,
    construction_born_admissions: Vec<ConstructionBornAdmission>,
    snapshot: ReflectedHandProtocolEvidence,
}

impl Evidence {
    fn contract_survived(&self) -> bool {
        let ticks = self
            .construction_born_admissions
            .iter()
            .map(|admission| admission.tick)
            .collect::<Vec<_>>();
        self.parent_control.survived
            && self.summary.exact()
            && ticks == [47, 95, 103]
            && self
                .construction_born_admissions
                .iter()
                .all(|admission| admission.verdict == FactorizationVerdict::Composed)
            && self.construction_born_admissions.iter().any(|admission| {
                admission.tick == 47 && admission.composed_links.contains(&(LinkId(34), 3))
            })
            && self.construction_born_admissions.iter().any(|admission| {
                admission.tick == 95 && admission.composed_links.contains(&(LinkId(46), 7))
            })
    }

    fn selectivity_survived(&self) -> bool {
        self.contract_survived()
            && self
                .projections
                .iter()
                .all(|projection| projection.construction_tick == projection.consequence_tick)
            && self.construction_born_admissions.iter().all(|admission| {
                !admission.composed_links.is_empty()
                    && admission
                        .composed_links
                        .iter()
                        .all(|link| admission.projected_links.contains(link))
            })
    }
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

pub fn run_all(parent_bytes: &[u8]) -> Vec<(Arm, ProbeResult)> {
    let snapshot = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerConstructionOutcomeComposition,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let trace = snapshot
        .trajectory
        .iter()
        .flat_map(|step| step.existing_witness_trace.iter().cloned())
        .collect::<Vec<_>>();
    let completed = snapshot
        .trajectory
        .iter()
        .flat_map(|step| step.completed_cycle_continuations.iter().cloned())
        .collect::<Vec<_>>();
    let projections = construction_projections(&trace);
    let construction_born_admissions = factor_construction_born(&completed, &projections);
    let evidence = Evidence {
        parent_control: parent_control(parent_bytes),
        summary: summarize(&snapshot),
        projections,
        construction_born_admissions,
        snapshot,
    };
    Arm::ALL
        .into_iter()
        .map(|arm| {
            let survived = match arm {
                Arm::ExactWitnessInertness => {
                    evidence.parent_control.survived && evidence.summary.exact()
                }
                Arm::CompleteConstructionNaturality => evidence.contract_survived(),
                Arm::SelectivityIntegrationControl => evidence.selectivity_survived(),
            };
            let falsifier = match arm {
                Arm::ExactWitnessInertness => {
                    "exact witness diagnostics changed the revised hand or integrity summary"
                }
                Arm::CompleteConstructionNaturality => {
                    "a construction-born completed-cycle admission lacked an exact projected witness"
                }
                Arm::SelectivityIntegrationControl => {
                    "a projection changed time or a composed witness escaped its exact projected set"
                }
            };
            (
                arm,
                ProbeResult {
                    schema: "hand-complete-construction-naturality-contract/v1",
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

    fn projection(tick: i64, owner: u64, link: u64, generation: u32) -> Projection {
        Projection {
            construction_tick: tick,
            owner: LearnerId(owner),
            link: LinkId(link),
            generation,
            consequence_tick: tick,
        }
    }

    fn admission(
        tick: i64,
        owner: u64,
        consequence_tick: i64,
        witnesses: Vec<(LinkId, u32)>,
    ) -> CompletedCycleContinuationEvidence {
        CompletedCycleContinuationEvidence {
            tick,
            target: JunctionId(10),
            owner: Some(LearnerId(owner)),
            consequence_tick: Some(consequence_tick),
            consequence_witnesses: witnesses,
            unique_latest_tick: Some(consequence_tick),
            crosses_ownership_view: false,
            admitted: true,
        }
    }

    #[test]
    fn every_construction_born_admission_factors_through_exact_projection() {
        let projections = [
            projection(44, 3, 34, 3),
            projection(92, 4, 46, 7),
            projection(100, 5, 34, 3),
        ];
        let admissions = [
            admission(47, 3, 44, vec![(LinkId(34), 3)]),
            admission(95, 4, 92, vec![(LinkId(46), 7)]),
            admission(103, 5, 100, vec![(LinkId(34), 3)]),
        ];
        let factored = factor_construction_born(&admissions, &projections);
        assert_eq!(factored.len(), 3);
        assert!(
            factored
                .iter()
                .all(|admission| admission.verdict == FactorizationVerdict::Composed)
        );
    }

    #[test]
    fn wrong_generation_is_a_typed_missing_witness() {
        let factored = factor_construction_born(
            &[admission(47, 3, 44, vec![(LinkId(34), 4)])],
            &[projection(44, 3, 34, 3)],
        );
        assert_eq!(factored[0].verdict, FactorizationVerdict::MissingWitness);
        assert!(factored[0].composed_links.is_empty());
    }

    #[test]
    fn ordinary_later_cycle_is_not_construction_born() {
        let factored = factor_construction_born(
            &[admission(55, 3, 52, vec![(LinkId(34), 3)])],
            &[projection(44, 3, 34, 3)],
        );
        assert!(factored.is_empty());
    }
}
