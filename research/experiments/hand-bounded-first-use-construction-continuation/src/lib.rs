#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    CompletedCycleContinuationEvidence, EffectComposition, ExistingWitnessEvent,
    ExistingWitnessTraceEntry, OutputChoiceResolutionEvidence, ReflectedHandProtocolEvidence,
    run_reflected_hand_bounded,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use truelearner_core::{
    CompletedCycleState, JunctionId, LearnerId, LinkId, OutputChoiceBasis, Protocol,
};

const PARENT_SHA256: &str = "7d1d9d51d3291a03d59df16948cd8313b3324eb30c91b058d438dbda6b3aaf26";
const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    BoundedFirstUseFixtures,
    ReflectedHandFirstWall,
    ParentAndLifetimeControls,
}

impl Arm {
    pub const ALL: [Self; 3] = [
        Self::BoundedFirstUseFixtures,
        Self::ReflectedHandFirstWall,
        Self::ParentAndLifetimeControls,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::BoundedFirstUseFixtures => "bounded-first-use-fixtures",
            Self::ReflectedHandFirstWall => "reflected-hand-first-wall",
            Self::ParentAndLifetimeControls => "parent-and-lifetime-controls",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct Consumption {
    tick: i64,
    target: JunctionId,
    owner: LearnerId,
    link: LinkId,
    generation: u32,
    consequence_tick: i64,
}

#[derive(Clone, Debug, Serialize)]
struct ParentControl {
    artifact_sha256: String,
    expected_sha256: &'static str,
    artifact_survived: bool,
    live_summary_survived: bool,
}

#[derive(Clone, Debug, Serialize)]
struct Summary {
    protocol: Protocol,
    effect_composition: EffectComposition,
    positions: Vec<(i16, i16)>,
    actual_position_changes: usize,
    final_position: i16,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    comparisons: u64,
    scans: u64,
    propagation_budget_exhaustions: u64,
    stopped: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
}

#[derive(Clone, Debug, Serialize)]
struct FirstWall {
    step: usize,
    tick: i64,
    position_before: i16,
    position_after: i16,
    choice: Option<OutputChoiceResolutionEvidence>,
    target_evidence: Option<CompletedCycleContinuationEvidence>,
    exact_consumption: Option<Consumption>,
    survived: bool,
}

#[derive(Clone, Debug, Serialize)]
struct Evidence {
    parent_control: ParentControl,
    parent_summary: Summary,
    candidate_summary: Summary,
    first_wall: FirstWall,
    consumptions: Vec<Consumption>,
    duplicate_consumptions: Vec<Consumption>,
    stale_reuses_after_consumption: Vec<(i64, JunctionId, i64)>,
    work_bounded: bool,
    snapshot: ReflectedHandProtocolEvidence,
}

fn summarize(hand: &ReflectedHandProtocolEvidence) -> Summary {
    Summary {
        protocol: hand.protocol,
        effect_composition: hand.effect_composition,
        positions: hand
            .trajectory
            .iter()
            .map(|step| (step.position_before, step.position_after))
            .collect(),
        actual_position_changes: hand.actual_position_changes,
        final_position: hand.final_position,
        reached_lower: hand.reached_lower,
        reached_upper: hand.reached_upper,
        escaped_lower: hand.escaped_lower,
        escaped_upper: hand.escaped_upper,
        comparisons: hand.comparisons,
        scans: hand.scans,
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

fn parent_summary_survived(summary: &Summary) -> bool {
    summary.protocol == Protocol::RecursiveLearnerConstructionOutcomeComposition
        && summary.effect_composition == EffectComposition::Batched
        && summary.positions
            == [
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 2),
                (2, 2),
                (2, 3),
                (3, 2),
                (2, 2),
                (2, 1),
                (1, 1),
                (1, 0),
                (0, -1),
                (-1, -2),
                (-2, -3),
                (-3, -2),
                (-2, -2),
            ]
        && summary.actual_position_changes == 12
        && summary.final_position == -2
        && !summary.reached_lower
        && !summary.reached_upper
        && !summary.escaped_lower
        && !summary.escaped_upper
        && summary.propagation_budget_exhaustions == 0
        && !summary.stopped
        && summary.exact_replay
        && summary.naturally_quiescent
}

fn consumptions(trace: &[ExistingWitnessTraceEntry]) -> Vec<Consumption> {
    trace
        .iter()
        .filter_map(|entry| match entry.event {
            ExistingWitnessEvent::ConstructionContinuationConsumed {
                target,
                owner,
                link,
                generation,
                consequence_tick,
            } => Some(Consumption {
                tick: entry.tick,
                target,
                owner,
                link,
                generation,
                consequence_tick,
            }),
            _ => None,
        })
        .collect()
}

fn duplicate_consumptions(consumed: &[Consumption]) -> Vec<Consumption> {
    let mut seen = BTreeSet::new();
    consumed
        .iter()
        .filter(|consumption| {
            !seen.insert((
                consumption.owner,
                consumption.link,
                consumption.generation,
                consumption.consequence_tick,
            ))
        })
        .cloned()
        .collect()
}

fn stale_reuses(
    consumed: &[Consumption],
    completed: &[CompletedCycleContinuationEvidence],
) -> Vec<(i64, JunctionId, i64)> {
    completed
        .iter()
        .filter(|effect| effect.admitted)
        .filter_map(|effect| {
            let owner = effect.owner?;
            let consequence_tick = effect.consequence_tick?;
            consumed
                .iter()
                .any(|consumption| {
                    consumption.tick < effect.tick
                        && consumption.owner == owner
                        && consumption.consequence_tick == consequence_tick
                        && effect
                            .consequence_witnesses
                            .contains(&(consumption.link, consumption.generation))
                        && effect.tick.saturating_sub(consequence_tick) > 4
                })
                .then_some((effect.tick, effect.target, consequence_tick))
        })
        .collect()
}

fn first_wall(hand: &ReflectedHandProtocolEvidence, consumed: &[Consumption]) -> FirstWall {
    let step = hand.trajectory.get(3);
    let choice = step.and_then(|step| {
        step.output_choice_resolutions
            .iter()
            .find(|choice| choice.tick == 23 && choice.phase == 0)
            .cloned()
    });
    let target_evidence = step.and_then(|step| {
        step.completed_cycle_continuations
            .iter()
            .find(|effect| effect.tick == 23 && effect.target == JunctionId(11))
            .cloned()
    });
    let exact_consumption = consumed
        .iter()
        .find(|consumption| {
            consumption.tick == 23
                && consumption.target == JunctionId(11)
                && consumption.owner == LearnerId(2)
                && consumption.link == LinkId(36)
                && consumption.generation == 1
                && consumption.consequence_tick == 16
        })
        .cloned();
    let position_before = step.map_or(i16::MIN, |step| step.position_before);
    let position_after = step.map_or(i16::MIN, |step| step.position_after);
    let survived = position_before == 3
        && position_after == 4
        && choice.as_ref().is_some_and(|choice| {
            choice.completed_cycle_target == Some(JunctionId(11))
                && choice.computed_winner_target == JunctionId(11)
                && choice.computed_winner_basis == OutputChoiceBasis::CompletedCycle
                && choice.completed_cycle_state == CompletedCycleState::Unique
        })
        && target_evidence.as_ref().is_some_and(|effect| {
            effect.admitted
                && effect.consequence_tick == Some(16)
                && effect.unique_latest_tick == Some(16)
                && effect.consequence_witnesses.contains(&(LinkId(36), 1))
        })
        && exact_consumption.is_some();
    FirstWall {
        step: 3,
        tick: 23,
        position_before,
        position_after,
        choice,
        target_evidence,
        exact_consumption,
        survived,
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
    let parent = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerConstructionOutcomeComposition,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let snapshot = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerBoundedConstructionContinuation,
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
    let consumptions = consumptions(&trace);
    let duplicate_consumptions = duplicate_consumptions(&consumptions);
    let stale_reuses_after_consumption = stale_reuses(&consumptions, &completed);
    let parent_summary = summarize(&parent);
    let candidate_summary = summarize(&snapshot);
    let artifact_sha256 = format!("{:x}", Sha256::digest(parent_bytes));
    let parent_control = ParentControl {
        artifact_survived: artifact_sha256 == PARENT_SHA256,
        live_summary_survived: parent_summary_survived(&parent_summary),
        artifact_sha256,
        expected_sha256: PARENT_SHA256,
    };
    let first_wall = first_wall(&snapshot, &consumptions);
    let work_bounded = candidate_summary.comparisons
        <= parent_summary.comparisons.saturating_add(256)
        && candidate_summary.scans <= parent_summary.scans.saturating_add(256);
    let evidence = Evidence {
        parent_control,
        parent_summary,
        candidate_summary,
        first_wall,
        consumptions,
        duplicate_consumptions,
        stale_reuses_after_consumption,
        work_bounded,
        snapshot,
    };

    Arm::ALL
        .into_iter()
        .map(|arm| {
            let integrity = evidence.candidate_summary.exact_replay
                && evidence.candidate_summary.naturally_quiescent
                && !evidence.candidate_summary.stopped
                && evidence.candidate_summary.propagation_budget_exhaustions == 0
                && evidence.work_bounded;
            let survived = match arm {
                Arm::BoundedFirstUseFixtures => {
                    !evidence.consumptions.is_empty()
                        && evidence.duplicate_consumptions.is_empty()
                        && evidence.stale_reuses_after_consumption.is_empty()
                        && integrity
                }
                Arm::ReflectedHandFirstWall => {
                    evidence.first_wall.survived
                        && evidence.candidate_summary.reached_upper
                        && integrity
                }
                Arm::ParentAndLifetimeControls => {
                    evidence.parent_control.artifact_survived
                        && evidence.parent_control.live_summary_survived
                        && evidence.duplicate_consumptions.is_empty()
                        && evidence.stale_reuses_after_consumption.is_empty()
                }
            };
            let falsifier = match arm {
                Arm::BoundedFirstUseFixtures => {
                    "held construction consequence was missing, reused, duplicated, or unbounded"
                }
                Arm::ReflectedHandFirstWall => {
                    "the exact tick-twenty-three arrow did not continue from plus three to plus four"
                }
                Arm::ParentAndLifetimeControls => {
                    "the immutable or live parent changed, or ordinary lifetime boundaries weakened"
                }
            };
            (
                arm,
                ProbeResult {
                    schema: "hand-bounded-first-use-construction-continuation/v1",
                    arm: arm.id(),
                    outcome: if survived { "survived" } else { "falsified" },
                    observations: serde_json::to_value(&evidence).expect("evidence serializes"),
                    falsifier: (!survived).then(|| falsifier.to_owned()),
                    exact_replay: evidence.candidate_summary.exact_replay,
                    naturally_quiescent: evidence.candidate_summary.naturally_quiescent,
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn consumption(tick: i64) -> Consumption {
        Consumption {
            tick,
            target: JunctionId(11),
            owner: LearnerId(2),
            link: LinkId(36),
            generation: 1,
            consequence_tick: 16,
        }
    }

    #[test]
    fn duplicate_use_is_detected_by_exact_memory_identity() {
        assert!(duplicate_consumptions(&[consumption(23)]).is_empty());
        assert_eq!(
            duplicate_consumptions(&[consumption(23), consumption(27)]),
            vec![consumption(27)]
        );
    }

    #[test]
    fn immutable_parent_digest_is_frozen() {
        assert_ne!(format!("{:x}", Sha256::digest(b"changed")), PARENT_SHA256);
    }
}
