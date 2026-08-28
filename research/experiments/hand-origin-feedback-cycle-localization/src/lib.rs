#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    DriveProvenanceEvidence, ReflectedHandProtocolEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{JunctionId, LearnerId, LinkId, Protocol};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const MIN_CYCLE_TRAVERSALS: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    OriginPathProvenanceDisagreement,
    StepEightFeedbackCycle,
    MatchedBoundaryControl,
    CompleteLocalization,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::OriginPathProvenanceDisagreement,
        Self::StepEightFeedbackCycle,
        Self::MatchedBoundaryControl,
        Self::CompleteLocalization,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::OriginPathProvenanceDisagreement => "origin-path-provenance-disagreement",
            Self::StepEightFeedbackCycle => "step-eight-feedback-cycle",
            Self::MatchedBoundaryControl => "matched-boundary-control",
            Self::CompleteLocalization => "complete-localization",
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EdgeKey {
    source: JunctionId,
    target: JunctionId,
    source_physical: u64,
    target_physical: u64,
    source_region: i16,
    target_region: i16,
    link: LinkId,
    completes_path: bool,
    carried_origin: u64,
    origin_owner: Option<LearnerId>,
    path_owner: Option<LearnerId>,
}

impl EdgeKey {
    fn from_event(event: &DriveProvenanceEvidence) -> Option<Self> {
        Some(Self {
            source: event.source?,
            target: event.target,
            source_physical: event.source_physical?,
            target_physical: event.target_physical,
            source_region: event.source_region?,
            target_region: event.target_region,
            link: event.link?,
            completes_path: event.completes_path,
            carried_origin: event.carried_origin,
            origin_owner: event.origin_owner,
            path_owner: event.path_owner,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CycleEdgeEvidence {
    pub source: JunctionId,
    pub target: JunctionId,
    pub source_physical: u64,
    pub target_physical: u64,
    pub source_region: i16,
    pub target_region: i16,
    pub link: LinkId,
    pub completes_path: bool,
    pub carried_origin: u64,
    pub origin_owner: Option<LearnerId>,
    pub path_owner: Option<LearnerId>,
    pub occurrences: usize,
    pub first_ordinal: u64,
    pub first_tick: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CycleEvidence {
    pub step: usize,
    pub edge_count: usize,
    pub carried_origin: u64,
    pub origin_owner: Option<LearnerId>,
    pub minimum_edge_occurrences: usize,
    pub first_ordinal: u64,
    pub first_tick: i64,
    pub last_tick: i64,
    pub outward_crossings: usize,
    pub inward_reentries: usize,
    pub graph_contiguous: bool,
    pub graph_closed: bool,
    pub edges: Vec<CycleEdgeEvidence>,
}

fn search_cycle(
    current: JunctionId,
    goal: JunctionId,
    adjacency: &BTreeMap<JunctionId, Vec<EdgeKey>>,
    visited: &mut BTreeSet<JunctionId>,
    path: &mut Vec<EdgeKey>,
    maximum_edges: usize,
) -> bool {
    if current == goal {
        return true;
    }
    if path.len() >= maximum_edges || !visited.insert(current) {
        return false;
    }
    if let Some(edges) = adjacency.get(&current) {
        for edge in edges {
            path.push(edge.clone());
            if search_cycle(edge.target, goal, adjacency, visited, path, maximum_edges) {
                return true;
            }
            path.pop();
        }
    }
    visited.remove(&current);
    false
}

fn shortest_repeated_cycle(
    step: usize,
    events: &[DriveProvenanceEvidence],
) -> Option<CycleEvidence> {
    let mut occurrences = BTreeMap::<EdgeKey, Vec<&DriveProvenanceEvidence>>::new();
    for event in events {
        if let Some(edge) = EdgeKey::from_event(event) {
            occurrences.entry(edge).or_default().push(event);
        }
    }
    let repeated = occurrences
        .iter()
        .filter(|(_, events)| events.len() >= MIN_CYCLE_TRAVERSALS)
        .map(|(edge, _)| edge.clone())
        .collect::<Vec<_>>();
    let mut best: Option<Vec<EdgeKey>> = None;
    for origin in repeated
        .iter()
        .map(|edge| edge.carried_origin)
        .collect::<BTreeSet<_>>()
    {
        let origin_edges = repeated
            .iter()
            .filter(|edge| edge.carried_origin == origin)
            .cloned()
            .collect::<Vec<_>>();
        let mut adjacency = BTreeMap::<JunctionId, Vec<EdgeKey>>::new();
        for edge in &origin_edges {
            adjacency.entry(edge.source).or_default().push(edge.clone());
        }
        for edges in adjacency.values_mut() {
            edges.sort();
        }
        for first in &origin_edges {
            let mut path = vec![first.clone()];
            let mut visited = BTreeSet::from([first.source]);
            if search_cycle(
                first.target,
                first.source,
                &adjacency,
                &mut visited,
                &mut path,
                origin_edges.len(),
            ) && best
                .as_ref()
                .is_none_or(|known| (path.len(), &path) < (known.len(), known))
            {
                best = Some(path);
            }
        }
    }
    let edges = best?;
    let evidence = edges
        .iter()
        .map(|edge| {
            let observed = &occurrences[edge];
            let first = observed
                .iter()
                .min_by_key(|event| event.ordinal)
                .expect("repeated edge has an observation");
            CycleEdgeEvidence {
                source: edge.source,
                target: edge.target,
                source_physical: edge.source_physical,
                target_physical: edge.target_physical,
                source_region: edge.source_region,
                target_region: edge.target_region,
                link: edge.link,
                completes_path: edge.completes_path,
                carried_origin: edge.carried_origin,
                origin_owner: edge.origin_owner,
                path_owner: edge.path_owner,
                occurrences: observed.len(),
                first_ordinal: first.ordinal,
                first_tick: first.tick,
            }
        })
        .collect::<Vec<_>>();
    let graph_contiguous = evidence
        .windows(2)
        .all(|pair| pair[0].target == pair[1].source);
    let graph_closed = evidence
        .first()
        .zip(evidence.last())
        .is_some_and(|(first, last)| last.target == first.source);
    Some(CycleEvidence {
        step,
        edge_count: evidence.len(),
        carried_origin: evidence[0].carried_origin,
        origin_owner: evidence[0].origin_owner,
        minimum_edge_occurrences: evidence
            .iter()
            .map(|edge| edge.occurrences)
            .min()
            .unwrap_or(0),
        first_ordinal: evidence
            .iter()
            .map(|edge| edge.first_ordinal)
            .min()
            .unwrap_or(0),
        first_tick: evidence
            .iter()
            .map(|edge| edge.first_tick)
            .min()
            .unwrap_or(0),
        last_tick: events.iter().map(|event| event.tick).max().unwrap_or(0),
        outward_crossings: evidence
            .iter()
            .filter(|edge| edge.source_region == 0 && edge.target_region != 0)
            .count(),
        inward_reentries: evidence
            .iter()
            .filter(|edge| edge.source_region != 0 && edge.target_region == 0)
            .count(),
        graph_contiguous,
        graph_closed,
        edges: evidence,
    })
}

#[derive(Clone, Debug, Serialize)]
struct ProvenanceDisagreementEvidence {
    completing_inputs: usize,
    owner_disagreements: usize,
    owned_path_unowned_origin: usize,
    first_disagreement: Option<DriveProvenanceEvidence>,
    exact_replay: bool,
    survived: bool,
}

fn provenance_disagreement(hand: &ReflectedHandProtocolEvidence) -> ProvenanceDisagreementEvidence {
    let completing = hand
        .trajectory
        .iter()
        .flat_map(|step| &step.drive_provenance)
        .filter(|event| event.completes_path)
        .collect::<Vec<_>>();
    let disagreements = completing
        .iter()
        .copied()
        .filter(|event| event.path_owner != event.origin_owner)
        .collect::<Vec<_>>();
    let owned_path_unowned_origin = disagreements
        .iter()
        .filter(|event| event.path_owner.is_some() && event.origin_owner.is_none())
        .count();
    let owner_disagreements = disagreements.len();
    ProvenanceDisagreementEvidence {
        completing_inputs: completing.len(),
        owner_disagreements,
        owned_path_unowned_origin,
        first_disagreement: disagreements.first().cloned().cloned(),
        exact_replay: hand.exact_replay,
        survived: owner_disagreements > 0 && owned_path_unowned_origin > 0 && hand.exact_replay,
    }
}

#[derive(Clone, Debug, Serialize)]
struct FeedbackCycleEvidence {
    first_exhaustion_step: Option<usize>,
    provenance_events: usize,
    motor_candidates: usize,
    cycle: Option<CycleEvidence>,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn feedback_cycle(hand: &ReflectedHandProtocolEvidence) -> FeedbackCycleEvidence {
    let exhaustion = hand
        .trajectory
        .iter()
        .find(|step| step.propagation_budget_exhaustions > 0);
    let cycle =
        exhaustion.and_then(|step| shortest_repeated_cycle(step.index, &step.drive_provenance));
    let survived = cycle.as_ref().is_some_and(|cycle| {
        cycle.graph_contiguous
            && cycle.graph_closed
            && cycle.minimum_edge_occurrences >= MIN_CYCLE_TRAVERSALS
            && cycle.outward_crossings > 0
            && cycle.inward_reentries > 0
    }) && hand.exact_replay
        && !hand.naturally_quiescent;
    FeedbackCycleEvidence {
        first_exhaustion_step: exhaustion.map(|step| step.index),
        provenance_events: exhaustion.map_or(0, |step| step.drive_provenance.len()),
        motor_candidates: exhaustion.map_or(0, |step| {
            step.output_candidates
                .iter()
                .filter(|candidate| candidate.is_motor)
                .count()
        }),
        cycle,
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
        survived,
    }
}

#[derive(Clone, Debug, Serialize)]
struct MatchedControlEvidence {
    reference_changed_steps: usize,
    candidate_changed_steps: usize,
    reference_final_position: i16,
    candidate_final_position: i16,
    reference_exhaustions: u64,
    candidate_exhaustions: u64,
    reference_repeated_cycles: usize,
    reference_exact_replay: bool,
    candidate_exact_replay: bool,
    reference_naturally_quiescent: bool,
    candidate_naturally_quiescent: bool,
    frozen_candidate_behavior: bool,
    survived: bool,
}

fn exhaustion_count(hand: &ReflectedHandProtocolEvidence) -> u64 {
    hand.trajectory
        .iter()
        .map(|step| step.propagation_budget_exhaustions)
        .sum()
}

fn matched_control(
    reference: &ReflectedHandProtocolEvidence,
    candidate: &ReflectedHandProtocolEvidence,
) -> MatchedControlEvidence {
    let reference_repeated_cycles = reference
        .trajectory
        .iter()
        .filter(|step| shortest_repeated_cycle(step.index, &step.drive_provenance).is_some())
        .count();
    let reference_exhaustions = exhaustion_count(reference);
    let candidate_exhaustions = exhaustion_count(candidate);
    let frozen_candidate_behavior = candidate.changed_steps == 8
        && candidate.final_position == 4
        && candidate_exhaustions == 2
        && candidate.exact_replay
        && !candidate.naturally_quiescent;
    let survived = reference.changed_steps == 3
        && reference.final_position == -1
        && reference_exhaustions == 0
        && reference_repeated_cycles == 0
        && reference.exact_replay
        && reference.naturally_quiescent
        && frozen_candidate_behavior;
    MatchedControlEvidence {
        reference_changed_steps: reference.changed_steps,
        candidate_changed_steps: candidate.changed_steps,
        reference_final_position: reference.final_position,
        candidate_final_position: candidate.final_position,
        reference_exhaustions,
        candidate_exhaustions,
        reference_repeated_cycles,
        reference_exact_replay: reference.exact_replay,
        candidate_exact_replay: candidate.exact_replay,
        reference_naturally_quiescent: reference.naturally_quiescent,
        candidate_naturally_quiescent: candidate.naturally_quiescent,
        frozen_candidate_behavior,
        survived,
    }
}

#[derive(Clone, Debug)]
struct Evidence {
    provenance: ProvenanceDisagreementEvidence,
    cycle: FeedbackCycleEvidence,
    control: MatchedControlEvidence,
}

fn measure() -> Evidence {
    let reference = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerBoundaryNovelty,
        512,
        2_048,
        MAX_MOMENTS_PER_SEND,
    );
    let candidate = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerCausalOriginFactorization,
        512,
        2_048,
        MAX_MOMENTS_PER_SEND,
    );
    Evidence {
        provenance: provenance_disagreement(&candidate),
        cycle: feedback_cycle(&candidate),
        control: matched_control(&reference, &candidate),
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

fn result(
    arm: Arm,
    survived: bool,
    observations: serde_json::Value,
    falsifier: &'static str,
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "hand-origin-feedback-cycle-localization/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations,
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay,
        naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    match arm {
        Arm::OriginPathProvenanceDisagreement => result(
            arm,
            evidence.provenance.survived,
            serde_json::to_value(&evidence.provenance).unwrap(),
            "the trace did not expose an owned completing path carrying an unowned physical origin",
            evidence.provenance.exact_replay,
            evidence.control.candidate_naturally_quiescent,
        ),
        Arm::StepEightFeedbackCycle => result(
            arm,
            evidence.cycle.survived,
            serde_json::to_value(&evidence.cycle).unwrap(),
            "no closed repeated physical cycle crossing outward and back inward was localized before exhaustion",
            evidence.cycle.exact_replay,
            evidence.cycle.naturally_quiescent,
        ),
        Arm::MatchedBoundaryControl => result(
            arm,
            evidence.control.survived,
            serde_json::to_value(&evidence.control).unwrap(),
            "instrumentation changed frozen behavior or the matched boundary reference also exhausted or cycled",
            evidence.control.reference_exact_replay && evidence.control.candidate_exact_replay,
            evidence.control.reference_naturally_quiescent,
        ),
        Arm::CompleteLocalization => {
            let survived = evidence.provenance.survived
                && evidence.cycle.survived
                && evidence.control.survived;
            result(
                arm,
                survived,
                serde_json::json!({
                    "provenance_disagreement_localized": evidence.provenance.survived,
                    "feedback_cycle_localized": evidence.cycle.survived,
                    "matched_control_preserved": evidence.control.survived,
                    "cycle": evidence.cycle.cycle,
                    "next_discriminator": "test whether outward-effect participation in new inward paths is the exact re-entry boundary, using unrelated output worlds before another hand solve",
                }),
                "the diagnostic did not jointly preserve behavior and localize provenance disagreement plus the closed cycle",
                evidence.provenance.exact_replay
                    && evidence.cycle.exact_replay
                    && evidence.control.reference_exact_replay,
                evidence.control.reference_naturally_quiescent,
            )
        }
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_path_provenance_disagreement_is_localized() {
        assert_eq!(
            run(Arm::OriginPathProvenanceDisagreement).outcome,
            "survived"
        );
    }

    #[test]
    fn step_eight_closed_feedback_cycle_is_localized() {
        let result = run(Arm::StepEightFeedbackCycle);
        assert_eq!(result.outcome, "survived");
        assert!(result.exact_replay);
        assert!(!result.naturally_quiescent);
    }

    #[test]
    fn matched_boundary_reference_stays_quiet_and_behavior_is_frozen() {
        let result = run(Arm::MatchedBoundaryControl);
        assert_eq!(result.outcome, "survived");
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
    }

    #[test]
    fn complete_localization_is_compact_and_deterministic() {
        let first = run(Arm::CompleteLocalization);
        let second = run(Arm::CompleteLocalization);
        assert_eq!(first.outcome, "survived");
        assert_eq!(
            serde_json::to_string(&first).unwrap(),
            serde_json::to_string(&second).unwrap()
        );
    }
}
