#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    CompletedCycleContinuationEvidence, EffectComposition, ExistingWitnessEvent,
    OutputChoiceResolutionEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{CompletedCycleState, JunctionId, LearnerId, OutputChoiceBasis, Protocol};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    SameTickConstructionComposition,
    TemporalAndLineageSelectivity,
}

impl Arm {
    pub const ALL: [Self; 2] = [
        Self::SameTickConstructionComposition,
        Self::TemporalAndLineageSelectivity,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::SameTickConstructionComposition => "same-tick-construction-composition",
            Self::TemporalAndLineageSelectivity => "temporal-and-lineage-selectivity",
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
struct ConstructionProjection {
    hand_step: usize,
    construction_tick: i64,
    learner: LearnerId,
    link: truelearner_core::LinkId,
    generation: u32,
    consequence_tick: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct HandSummary {
    protocol: Protocol,
    effect_composition: EffectComposition,
    actual_position_changes: usize,
    opposing_output_steps: usize,
    directions: BTreeSet<i8>,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    final_position: i16,
    completed_cycle_admissions: usize,
    cross_view_admissions: usize,
    propagation_budget_exhaustions: u64,
    stopped: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
}

impl HandSummary {
    fn exact_unchanged_hand(&self) -> bool {
        self.protocol == Protocol::RecursiveLearnerConstructionOutcomeComposition
            && self.effect_composition == EffectComposition::Batched
            && self.actual_position_changes == 12
            && self.opposing_output_steps == 4
            && self.directions == BTreeSet::from([-1, 1])
            && !self.reached_lower
            && !self.reached_upper
            && !self.escaped_lower
            && !self.escaped_upper
            && self.final_position == -2
            && self.completed_cycle_admissions == 9
            && self.cross_view_admissions == 2
            && self.propagation_budget_exhaustions == 0
            && !self.stopped
            && self.exact_replay
            && self.naturally_quiescent
    }
}

#[derive(Clone, Debug, Serialize)]
struct Evidence {
    summary: HandSummary,
    construction_projections: Vec<ConstructionProjection>,
    tick_twenty_three_target_eleven: Option<CompletedCycleContinuationEvidence>,
    tick_twenty_three_choice: Option<OutputChoiceResolutionEvidence>,
}

impl Evidence {
    fn same_tick_projection_survived(&self) -> bool {
        let projected = self.construction_projections.iter().any(|projection| {
            projection.hand_step == 2
                && projection.construction_tick == 16
                && projection.learner == LearnerId(2)
                && projection.consequence_tick == 16
        });
        let stale_target = self
            .tick_twenty_three_target_eleven
            .as_ref()
            .is_some_and(|effect| {
                effect.tick == 23
                    && effect.target == JunctionId(11)
                    && effect.owner == Some(LearnerId(2))
                    && effect.consequence_tick == Some(16)
                    && !effect.admitted
            });
        let unchanged_choice = self
            .tick_twenty_three_choice
            .as_ref()
            .is_some_and(|choice| {
                choice.tick == 23
                    && choice.completed_cycle_state == CompletedCycleState::Stale
                    && choice.admission_basis == OutputChoiceBasis::FreshAlternative
                    && choice.admitted
                        == vec![truelearner_core::OutputAdmission {
                            target: JunctionId(10),
                            owner: Some(LearnerId(2)),
                        }]
            });
        projected && stale_target && unchanged_choice && self.summary.exact_unchanged_hand()
    }

    fn selectivity_survived(&self) -> bool {
        !self.construction_projections.is_empty()
            && self
                .construction_projections
                .iter()
                .all(|projection| projection.construction_tick == projection.consequence_tick)
            && self.same_tick_projection_survived()
    }
}

fn measure() -> Evidence {
    let hand = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerConstructionOutcomeComposition,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let completed = hand
        .trajectory
        .iter()
        .flat_map(|step| step.completed_cycle_continuations.iter())
        .collect::<Vec<_>>();
    let construction_projections = hand
        .trajectory
        .iter()
        .flat_map(|step| {
            step.existing_witness_trace
                .iter()
                .enumerate()
                .flat_map(move |(index, entry)| {
                    let ExistingWitnessEvent::LearnerConstructed { learner, .. } = entry.event
                    else {
                        return Vec::new();
                    };
                    step.existing_witness_trace
                        .iter()
                        .skip(index.saturating_add(1))
                        .map_while(|following| match following.event {
                            ExistingWitnessEvent::LearnerConsequenceRecorded {
                                owner,
                                link,
                                generation,
                                consequence_tick,
                            } if owner == learner => Some(ConstructionProjection {
                                hand_step: step.index,
                                construction_tick: entry.tick,
                                learner,
                                link,
                                generation,
                                consequence_tick,
                            }),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
        })
        .collect::<Vec<_>>();
    let tick_twenty_three_target_eleven = completed
        .iter()
        .find(|effect| effect.tick == 23 && effect.target == JunctionId(11))
        .map(|effect| (*effect).clone());
    let tick_twenty_three_choice = hand
        .trajectory
        .iter()
        .flat_map(|step| step.output_choice_resolutions.iter())
        .find(|choice| {
            choice.tick == 23 && choice.admitted.iter().any(|a| a.target == JunctionId(10))
        })
        .cloned();
    let summary = HandSummary {
        protocol: hand.protocol,
        effect_composition: hand.effect_composition,
        actual_position_changes: hand.actual_position_changes,
        opposing_output_steps: hand
            .trajectory
            .iter()
            .filter(|step| {
                step.emitted_outputs.contains(&20_000) && step.emitted_outputs.contains(&20_001)
            })
            .count(),
        directions: hand.directions,
        reached_lower: hand.reached_lower,
        reached_upper: hand.reached_upper,
        escaped_lower: hand.escaped_lower,
        escaped_upper: hand.escaped_upper,
        final_position: hand.final_position,
        completed_cycle_admissions: completed.iter().filter(|effect| effect.admitted).count(),
        cross_view_admissions: completed
            .iter()
            .filter(|effect| effect.admitted && effect.crosses_ownership_view)
            .count(),
        propagation_budget_exhaustions: hand
            .trajectory
            .iter()
            .map(|step| step.propagation_budget_exhaustions)
            .sum(),
        stopped: hand.stopped,
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
    };
    Evidence {
        summary,
        construction_projections,
        tick_twenty_three_target_eleven,
        tick_twenty_three_choice,
    }
}

static EVIDENCE: OnceLock<Evidence> = OnceLock::new();

fn evidence() -> &'static Evidence {
    EVIDENCE.get_or_init(measure)
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

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    let survived = match arm {
        Arm::SameTickConstructionComposition => evidence.same_tick_projection_survived(),
        Arm::TemporalAndLineageSelectivity => evidence.selectivity_survived(),
    };
    let falsifier = match arm {
        Arm::SameTickConstructionComposition => {
            "same-tick construction did not produce tick-sixteen owner-local evidence and a tick-twenty-three Stale classification with unchanged choice"
        }
        Arm::TemporalAndLineageSelectivity => {
            "a construction projection refreshed time or the exact unchanged-hand and integrity controls failed"
        }
    };
    ProbeResult {
        schema: "hand-construction-outcome-composition/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations: serde_json::json!(evidence),
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: evidence.summary.exact_replay,
        naturally_quiescent: evidence.summary.naturally_quiescent,
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_projection_requires_the_original_tick() {
        let valid = ConstructionProjection {
            hand_step: 2,
            construction_tick: 16,
            learner: LearnerId(2),
            link: truelearner_core::LinkId(45),
            generation: 1,
            consequence_tick: 16,
        };
        assert_eq!(valid.construction_tick, valid.consequence_tick);
        let refreshed = ConstructionProjection {
            consequence_tick: 23,
            ..valid
        };
        assert_ne!(refreshed.construction_tick, refreshed.consequence_tick);
    }
}
