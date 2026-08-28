#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    EffectComposition, ExistingWitnessEvent, ReflectedHandProtocolEvidence,
    run_reflected_hand_bounded,
};
use serde::Serialize;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{
    CompletedCycleState, JunctionId, LearnerId, LinkId, OutputAdmission, OutputChoiceBasis,
    Protocol,
};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    ExistingTraceRetention,
    ConstructionBoundaryCycleWitness,
}

impl Arm {
    pub const ALL: [Self; 2] = [
        Self::ExistingTraceRetention,
        Self::ConstructionBoundaryCycleWitness,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::ExistingTraceRetention => "existing-trace-retention",
            Self::ConstructionBoundaryCycleWitness => "construction-boundary-cycle-witness",
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
struct HandSummary {
    protocol: Protocol,
    effect_composition: EffectComposition,
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

impl HandSummary {
    fn exact_parent(&self) -> bool {
        self.protocol == Protocol::RecursiveLearnerCompletedCycle
            && self.effect_composition == EffectComposition::Batched
            && self.actual_position_changes == 12
            && self.opposing_output_steps == 4
            && self.final_position == -2
            && !self.reached_lower
            && !self.reached_upper
            && !self.escaped_lower
            && !self.escaped_upper
            && self.completed_cycle_admissions == 9
            && self.cross_view_admissions == 2
            && self.output_choice_resolutions > 0
            && self.propagation_budget_exhaustions == 0
            && !self.stopped
            && self.exact_replay
            && self.naturally_quiescent
    }
}

fn summarize(hand: &ReflectedHandProtocolEvidence) -> HandSummary {
    HandSummary {
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
        cross_view_admissions: hand
            .trajectory
            .iter()
            .flat_map(|step| &step.completed_cycle_continuations)
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct TracePoint {
    hand_step: usize,
    tick: i64,
    phase: i32,
    event: ExistingWitnessEvent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ChoicePoint {
    hand_step: usize,
    tick: i64,
    phase: i32,
    admitted: Vec<OutputAdmission>,
    admission_basis: OutputChoiceBasis,
    completed_cycle_state: CompletedCycleState,
}

impl ChoicePoint {
    fn unique(&self) -> Option<OutputAdmission> {
        (self.admitted.len() == 1).then(|| self.admitted[0])
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ConstructionPoint {
    hand_step: usize,
    tick: i64,
    phase: i32,
    learner: LearnerId,
    parent: Option<LearnerId>,
    surface: JunctionId,
    output: JunctionId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct MissingOwnerPreference {
    tick: i64,
    phase: i32,
    owner: LearnerId,
    target: JunctionId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ParticipationPoint {
    tick: i64,
    phase: i32,
    causal_wave: u64,
    target: JunctionId,
    completes_path: bool,
    observed_generations: Vec<u32>,
    same_generation: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct LearnerWritePoint {
    tick: i64,
    phase: i32,
    owner: LearnerId,
    generation: u32,
    consequence_tick: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct WitnessLinkEvidence {
    link: LinkId,
    consequence_tick: i64,
    observed_generations_at_consequence: Vec<u32>,
    unique_generation: Option<u32>,
    deallocated_at: Vec<(i64, i32)>,
    live_through_failure: bool,
    failure_participation: Vec<ParticipationPoint>,
    matching_fresh_owner_writes: Vec<LearnerWritePoint>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WitnessVerdict {
    OwnerProjectionGap,
    PhysicalWitnessDeallocated,
    PhysicalWitnessNotParticipating,
    InsufficientExistingTrace,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct WitnessAnalysis {
    verdict: WitnessVerdict,
    reason: String,
    previous_choice: Option<ChoicePoint>,
    first_changed_choice: Option<ChoicePoint>,
    completed_consequence_tick: Option<i64>,
    witness_target: Option<JunctionId>,
    fresh_owner: Option<LearnerId>,
    constructions: Vec<ConstructionPoint>,
    missing_owner_preferences: Vec<MissingOwnerPreference>,
    witness_links: Vec<WitnessLinkEvidence>,
    decisive_slice: Vec<TracePoint>,
}

fn flatten_trace(hand: &ReflectedHandProtocolEvidence) -> Vec<TracePoint> {
    hand.trajectory
        .iter()
        .flat_map(|step| {
            step.existing_witness_trace
                .iter()
                .map(move |entry| TracePoint {
                    hand_step: step.index,
                    tick: entry.tick,
                    phase: entry.phase,
                    event: entry.event.clone(),
                })
        })
        .collect()
}

fn choices(trace: &[TracePoint]) -> Vec<ChoicePoint> {
    trace
        .iter()
        .filter_map(|point| match &point.event {
            ExistingWitnessEvent::OutputChoiceResolved(choice) => Some(ChoicePoint {
                hand_step: point.hand_step,
                tick: point.tick,
                phase: point.phase,
                admitted: choice.admitted.clone(),
                admission_basis: choice.admission_basis,
                completed_cycle_state: choice.completed_cycle_state,
            }),
            _ => None,
        })
        .collect()
}

fn generations_at(
    trace: &[TracePoint],
    link: LinkId,
    tick: i64,
    causal_wave: Option<u64>,
) -> Vec<u32> {
    let mut generations = BTreeSet::new();
    for point in trace {
        match &point.event {
            ExistingWitnessEvent::CausalLineageMemberObserved {
                link: Some(observed_link),
                generation: Some(generation),
                causal_wave: observed_wave,
                ..
            } if point.tick == tick
                && *observed_link == link
                && causal_wave.is_none_or(|wave| wave == *observed_wave) =>
            {
                generations.insert(*generation);
            }
            ExistingWitnessEvent::ReturnScheduling {
                link: observed_link,
                generation,
                ..
            } if point.tick == tick && *observed_link == link && causal_wave.is_none() => {
                generations.insert(*generation);
            }
            _ => {}
        }
    }
    generations.into_iter().collect()
}

fn analyze_trace(trace: &[TracePoint]) -> WitnessAnalysis {
    let ordered_choices = choices(trace);
    let changed_pair = ordered_choices.windows(2).find_map(|window| {
        let previous = window[0].unique()?;
        let current = window[1].unique()?;
        (previous.owner != current.owner && previous.target != current.target)
            .then(|| (window[0].clone(), window[1].clone(), previous, current))
    });

    let Some((previous_choice, changed_choice, previous_admission, changed_admission)) =
        changed_pair
    else {
        return WitnessAnalysis {
            verdict: WitnessVerdict::InsufficientExistingTrace,
            reason: "no single-target ownership change that also changes target".to_owned(),
            previous_choice: None,
            first_changed_choice: None,
            completed_consequence_tick: None,
            witness_target: None,
            fresh_owner: None,
            constructions: Vec::new(),
            missing_owner_preferences: Vec::new(),
            witness_links: Vec::new(),
            decisive_slice: trace.to_vec(),
        };
    };
    let consequence_tick = trace.iter().find_map(|point| match &point.event {
        ExistingWitnessEvent::CompletedCycleContinuationEvaluated(effect)
            if point.tick == previous_choice.tick
                && effect.target == previous_admission.target
                && effect.admitted =>
        {
            effect.consequence_tick
        }
        _ => None,
    });
    let slice_start = consequence_tick.unwrap_or(previous_choice.tick);
    let decisive_slice = trace
        .iter()
        .filter(|point| point.tick >= slice_start && point.tick <= changed_choice.tick)
        .cloned()
        .collect::<Vec<_>>();
    let constructions = decisive_slice
        .iter()
        .filter_map(|point| match point.event {
            ExistingWitnessEvent::LearnerConstructed {
                learner,
                parent,
                surface,
                output,
                ..
            } => Some(ConstructionPoint {
                hand_step: point.hand_step,
                tick: point.tick,
                phase: point.phase,
                learner,
                parent,
                surface,
                output,
            }),
            _ => None,
        })
        .collect::<Vec<_>>();
    let fresh_owner = changed_admission.owner;
    let missing_owner_preferences = fresh_owner
        .map(|owner| {
            decisive_slice
                .iter()
                .filter_map(|point| match point.event {
                    ExistingWitnessEvent::LearnerCandidatePreference {
                        owner: observed_owner,
                        target,
                        consequence_tick: None,
                        ..
                    } if observed_owner == owner => Some(MissingOwnerPreference {
                        tick: point.tick,
                        phase: point.phase,
                        owner,
                        target,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let witness_ids = consequence_tick
        .map(|tick| {
            trace
                .iter()
                .filter_map(|point| match point.event {
                    ExistingWitnessEvent::ConsequenceRecorded { link, junction }
                        if point.tick == tick && junction == previous_admission.target =>
                    {
                        Some(link)
                    }
                    _ => None,
                })
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let witness_links = witness_ids
        .into_iter()
        .map(|link| {
            let consequence_tick = consequence_tick.expect("witnesses require a consequence tick");
            let observed_generations_at_consequence =
                generations_at(trace, link, consequence_tick, None);
            let unique_generation = (observed_generations_at_consequence.len() == 1)
                .then(|| observed_generations_at_consequence[0]);
            let deallocated_at = decisive_slice
                .iter()
                .filter_map(|point| match point.event {
                    ExistingWitnessEvent::LinkDeallocated { link: observed }
                        if observed == link =>
                    {
                        Some((point.tick, point.phase))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            let live_through_failure = deallocated_at.is_empty();
            let failure_participation = decisive_slice
                .iter()
                .filter_map(|point| match &point.event {
                    ExistingWitnessEvent::DriveProvenanceObserved(drive)
                        if point.tick == changed_choice.tick && drive.link == Some(link) =>
                    {
                        let observed_generations =
                            generations_at(trace, link, point.tick, Some(drive.causal_wave));
                        Some(ParticipationPoint {
                            tick: point.tick,
                            phase: point.phase,
                            causal_wave: drive.causal_wave,
                            target: drive.target,
                            completes_path: drive.completes_path,
                            same_generation: unique_generation.is_some()
                                && unique_generation
                                    == (observed_generations.len() == 1)
                                        .then(|| observed_generations[0]),
                            observed_generations,
                        })
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            let matching_fresh_owner_writes = fresh_owner
                .map(|owner| {
                    decisive_slice
                        .iter()
                        .filter_map(|point| match point.event {
                            ExistingWitnessEvent::LearnerConsequenceRecorded {
                                owner: observed_owner,
                                link: observed_link,
                                generation,
                                consequence_tick,
                            } if observed_owner == owner
                                && observed_link == link
                                && unique_generation == Some(generation) =>
                            {
                                Some(LearnerWritePoint {
                                    tick: point.tick,
                                    phase: point.phase,
                                    owner,
                                    generation,
                                    consequence_tick,
                                })
                            }
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            WitnessLinkEvidence {
                link,
                consequence_tick,
                observed_generations_at_consequence,
                unique_generation,
                deallocated_at,
                live_through_failure,
                failure_participation,
                matching_fresh_owner_writes,
            }
        })
        .collect::<Vec<_>>();

    let constructed_fresh_owner = fresh_owner.is_some_and(|owner| {
        constructions
            .iter()
            .any(|construction| construction.learner == owner)
    });
    let all_deallocated =
        !witness_links.is_empty() && witness_links.iter().all(|link| !link.live_through_failure);
    let live_links = witness_links
        .iter()
        .filter(|link| link.live_through_failure)
        .collect::<Vec<_>>();
    let exact_participant = live_links.iter().any(|link| {
        link.unique_generation.is_some()
            && link
                .failure_participation
                .iter()
                .any(|participation| participation.same_generation)
    });
    let matching_private_write = live_links
        .iter()
        .any(|link| !link.matching_fresh_owner_writes.is_empty());
    let all_live_generations_known = !live_links.is_empty()
        && live_links
            .iter()
            .all(|link| link.unique_generation.is_some());

    let (verdict, reason) = if consequence_tick.is_none() || witness_links.is_empty() {
        (
            WitnessVerdict::InsufficientExistingTrace,
            "the accepted completed-cycle consequence tick or its physical link is absent"
                .to_owned(),
        )
    } else if all_deallocated {
        (
            WitnessVerdict::PhysicalWitnessDeallocated,
            "every consequence-bearing physical link was deallocated before the changed choice"
                .to_owned(),
        )
    } else if !all_live_generations_known {
        (
            WitnessVerdict::InsufficientExistingTrace,
            "the retained lineage does not identify one physical generation for every live witness"
                .to_owned(),
        )
    } else if !exact_participant {
        (
            WitnessVerdict::PhysicalWitnessNotParticipating,
            "a live witness remains, but the same link generation does not participate at the changed choice"
                .to_owned(),
        )
    } else if constructed_fresh_owner
        && !missing_owner_preferences.is_empty()
        && !matching_private_write
    {
        (
            WitnessVerdict::OwnerProjectionGap,
            "the same live physical witness generation participates after construction while the fresh owner reports no consequence and has no matching private write"
                .to_owned(),
        )
    } else {
        (
            WitnessVerdict::InsufficientExistingTrace,
            "the existing events do not jointly prove physical survival, fresh-owner absence, and construction"
                .to_owned(),
        )
    };

    WitnessAnalysis {
        verdict,
        reason,
        previous_choice: Some(previous_choice),
        first_changed_choice: Some(changed_choice),
        completed_consequence_tick: consequence_tick,
        witness_target: Some(previous_admission.target),
        fresh_owner,
        constructions,
        missing_owner_preferences,
        witness_links,
        decisive_slice,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct RetentionControl {
    ordered_within_steps: bool,
    consequence_writes_match: bool,
    constructions_match: bool,
    choices_match: bool,
    retained_events: usize,
}

impl RetentionControl {
    fn survived(&self) -> bool {
        self.ordered_within_steps
            && self.consequence_writes_match
            && self.constructions_match
            && self.choices_match
            && self.retained_events > 0
    }
}

fn retention_control(hand: &ReflectedHandProtocolEvidence) -> RetentionControl {
    RetentionControl {
        ordered_within_steps: hand.trajectory.iter().all(|step| {
            step.existing_witness_trace.windows(2).all(|events| {
                (events[0].tick, events[0].phase) <= (events[1].tick, events[1].phase)
            })
        }),
        consequence_writes_match: hand.trajectory.iter().all(|step| {
            step.consequence_writes.iter().all(|write| {
                step.existing_witness_trace.iter().any(|entry| {
                    entry.tick == write.tick
                        && matches!(
                            entry.event,
                            ExistingWitnessEvent::ConsequenceRecorded { link, junction }
                                if link == write.link && junction == write.junction
                        )
                })
            })
        }),
        constructions_match: hand.trajectory.iter().all(|step| {
            step.existing_witness_trace
                .iter()
                .filter(|entry| {
                    matches!(entry.event, ExistingWitnessEvent::LearnerConstructed { .. })
                })
                .count() as u64
                == step.constructions
        }),
        choices_match: hand.trajectory.iter().all(|step| {
            step.existing_witness_trace
                .iter()
                .filter(|entry| {
                    matches!(entry.event, ExistingWitnessEvent::OutputChoiceResolved(_))
                })
                .count()
                == step.output_choice_resolutions.len()
        }),
        retained_events: hand
            .trajectory
            .iter()
            .map(|step| step.existing_witness_trace.len())
            .sum(),
    }
}

#[derive(Clone, Debug)]
struct Evidence {
    summary: HandSummary,
    retention: RetentionControl,
    analysis: WitnessAnalysis,
}

fn measure() -> Evidence {
    let hand = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerCompletedCycle,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let trace = flatten_trace(&hand);
    Evidence {
        summary: summarize(&hand),
        retention: retention_control(&hand),
        analysis: analyze_trace(&trace),
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

fn exact_first_change(analysis: &WitnessAnalysis) -> bool {
    let Some(previous) = analysis
        .previous_choice
        .as_ref()
        .and_then(ChoicePoint::unique)
    else {
        return false;
    };
    let Some(current_choice) = &analysis.first_changed_choice else {
        return false;
    };
    let Some(current) = current_choice.unique() else {
        return false;
    };
    previous.target == JunctionId(11)
        && current_choice.hand_step == 3
        && current_choice.tick == 23
        && current.target == JunctionId(10)
        && current.owner == Some(LearnerId(2))
        && current_choice.admission_basis == OutputChoiceBasis::FreshAlternative
        && current_choice.completed_cycle_state == CompletedCycleState::Missing
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    let parent_intact = evidence.summary.exact_parent()
        && evidence.retention.survived()
        && exact_first_change(&evidence.analysis);
    let survived = match arm {
        Arm::ExistingTraceRetention => parent_intact,
        Arm::ConstructionBoundaryCycleWitness => {
            parent_intact && evidence.analysis.verdict != WitnessVerdict::InsufficientExistingTrace
        }
    };
    let falsifier = match arm {
        Arm::ExistingTraceRetention => {
            "retention changed or failed to reproduce the prior hand, first arrow change, or reduced evidence"
        }
        Arm::ConstructionBoundaryCycleWitness => {
            "the existing trace could not classify the witness lifecycle and owner projection"
        }
    };
    ProbeResult {
        schema: "hand-construction-cycle-witness/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations: serde_json::json!({
            "frozen_parent_summary": evidence.summary,
            "retention_control": evidence.retention,
            "witness_analysis": evidence.analysis,
        }),
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
    use developmental_hand_construction_admission::{
        CompletedCycleContinuationEvidence, DriveProvenanceEvidence, OutputChoiceResolutionEvidence,
    };
    use truelearner_core::TransmissionMode;

    fn point(tick: i64, event: ExistingWitnessEvent) -> TracePoint {
        TracePoint {
            hand_step: usize::from(tick >= 3),
            tick,
            phase: 0,
            event,
        }
    }

    fn choice(tick: i64, target: u64, owner: Option<u64>, basis: OutputChoiceBasis) -> TracePoint {
        point(
            tick,
            ExistingWitnessEvent::OutputChoiceResolved(OutputChoiceResolutionEvidence {
                tick: 0,
                phase: 0,
                ordinary_target: JunctionId(target),
                current_transition_target: None,
                coherent_effect_target: None,
                completed_cycle_target: (basis == OutputChoiceBasis::CompletedCycle)
                    .then_some(JunctionId(target)),
                computed_winner_target: JunctionId(target),
                admitted: vec![OutputAdmission {
                    target: JunctionId(target),
                    owner: owner.map(LearnerId),
                }],
                computed_winner_basis: basis,
                admission_basis: basis,
                completed_cycle_state: if basis == OutputChoiceBasis::CompletedCycle {
                    CompletedCycleState::Unique
                } else {
                    CompletedCycleState::Missing
                },
                crosses_ownership_view: owner.is_some(),
            }),
        )
    }

    fn base_trace() -> Vec<TracePoint> {
        vec![
            point(
                1,
                ExistingWitnessEvent::ConsequenceRecorded {
                    link: LinkId(7),
                    junction: JunctionId(11),
                },
            ),
            point(
                1,
                ExistingWitnessEvent::CausalLineageMemberObserved {
                    target: JunctionId(11),
                    origin_physical: 1,
                    mode: TransmissionMode::Drive,
                    link: Some(LinkId(7)),
                    generation: Some(3),
                    causal_wave: 1,
                },
            ),
            point(
                2,
                ExistingWitnessEvent::CompletedCycleContinuationEvaluated(
                    CompletedCycleContinuationEvidence {
                        tick: 0,
                        target: JunctionId(11),
                        owner: None,
                        consequence_tick: Some(1),
                        consequence_witnesses: vec![(LinkId(7), 3)],
                        unique_latest_tick: Some(1),
                        crosses_ownership_view: false,
                        admitted: true,
                    },
                ),
            ),
            choice(2, 11, None, OutputChoiceBasis::CompletedCycle),
            point(
                2,
                ExistingWitnessEvent::LearnerConstructed {
                    learner: LearnerId(2),
                    parent: Some(LearnerId(1)),
                    surface: JunctionId(20),
                    output: JunctionId(10),
                    junction_count: 1,
                    link_count: 1,
                },
            ),
            point(
                3,
                ExistingWitnessEvent::LearnerCandidatePreference {
                    owner: LearnerId(2),
                    target: JunctionId(11),
                    consequence_tick: None,
                    admitted: false,
                },
            ),
            point(
                3,
                ExistingWitnessEvent::CausalLineageMemberObserved {
                    target: JunctionId(11),
                    origin_physical: 1,
                    mode: TransmissionMode::Drive,
                    link: Some(LinkId(7)),
                    generation: Some(3),
                    causal_wave: 9,
                },
            ),
            point(
                3,
                ExistingWitnessEvent::DriveProvenanceObserved(DriveProvenanceEvidence {
                    ordinal: 0,
                    tick: 0,
                    phase: 0,
                    causal_wave: 9,
                    source: Some(JunctionId(20)),
                    target: JunctionId(11),
                    source_physical: Some(1),
                    target_physical: 2,
                    source_region: Some(0),
                    target_region: 0,
                    is_motor: true,
                    link: Some(LinkId(7)),
                    completes_path: true,
                    carried_origin: 1,
                    origin_owner: None,
                    path_owner: Some(LearnerId(2)),
                    strength: 1,
                }),
            ),
            choice(3, 10, Some(2), OutputChoiceBasis::FreshAlternative),
        ]
    }

    #[test]
    fn witness_verdict_distinguishes_all_registered_outcomes() {
        let owner_gap = analyze_trace(&base_trace());
        assert_eq!(owner_gap.verdict, WitnessVerdict::OwnerProjectionGap);

        let mut deallocated = base_trace();
        deallocated.insert(
            deallocated.len() - 1,
            point(3, ExistingWitnessEvent::LinkDeallocated { link: LinkId(7) }),
        );
        assert_eq!(
            analyze_trace(&deallocated).verdict,
            WitnessVerdict::PhysicalWitnessDeallocated
        );

        let non_participating = base_trace()
            .into_iter()
            .filter(|point| {
                !matches!(
                    point.event,
                    ExistingWitnessEvent::DriveProvenanceObserved(_)
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            analyze_trace(&non_participating).verdict,
            WitnessVerdict::PhysicalWitnessNotParticipating
        );

        let insufficient = base_trace()
            .into_iter()
            .filter(|point| {
                !matches!(
                    point.event,
                    ExistingWitnessEvent::ConsequenceRecorded { .. }
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            analyze_trace(&insufficient).verdict,
            WitnessVerdict::InsufficientExistingTrace
        );
    }
}
