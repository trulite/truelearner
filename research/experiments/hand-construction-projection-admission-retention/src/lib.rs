#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    CompletedCycleContinuationEvidence, EffectComposition, ExistingWitnessEvent,
    ReflectedHandProtocolEvidence, run_reflected_hand_bounded,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::str::FromStr;
use truelearner_core::{
    CompletedCycleState, JunctionId, LearnerId, LinkId, OutputAdmission, OutputChoiceBasis,
    Protocol,
};

const PARENT_SHA256: &str = "e4c38011cc9ff198474f553f72b6e4e7b366f3201a83fc114acd7c38a20e04d3";
const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    ImmutableParentChoiceControl,
    FirstAddedAdmissionLocalization,
}

impl Arm {
    pub const ALL: [Self; 2] = [
        Self::ImmutableParentChoiceControl,
        Self::FirstAddedAdmissionLocalization,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::ImmutableParentChoiceControl => "immutable-parent-choice-control",
            Self::FirstAddedAdmissionLocalization => "first-added-admission-localization",
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ChoicePoint {
    hand_step: usize,
    ordinal: usize,
    tick: i64,
    phase: i32,
    admitted: Vec<OutputAdmission>,
    admission_basis: OutputChoiceBasis,
    completed_cycle_state: CompletedCycleState,
    crosses_ownership_view: bool,
}

#[derive(Deserialize)]
struct ParentEnvelope {
    observations: ParentObservations,
}

#[derive(Deserialize)]
struct ParentObservations {
    naturality: ParentNaturality,
}

#[derive(Deserialize)]
struct ParentNaturality {
    ordered_choices: Vec<ChoicePoint>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ConstructionProjection {
    hand_step: usize,
    construction_tick: i64,
    learner: LearnerId,
    link: LinkId,
    generation: u32,
    consequence_tick: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct TracePoint {
    hand_step: usize,
    tick: i64,
    phase: i32,
    event: ExistingWitnessEvent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum LocalizationVerdict {
    FirstAddedAdmission,
    NoAddedAdmission,
    ChoiceIdentityMismatch,
    InsufficientPhysicalComposition,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Localization {
    verdict: LocalizationVerdict,
    parent_choice: Option<ChoicePoint>,
    candidate_choice: Option<ChoicePoint>,
    completed_cycle: Option<CompletedCycleContinuationEvidence>,
    construction_tick: Option<i64>,
    projected_links: Vec<(LinkId, u32)>,
    completing_links: Vec<(LinkId, u32)>,
    decisive_slice: Vec<TracePoint>,
    reason: String,
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
    fn exact_candidate(&self) -> bool {
        self.protocol == Protocol::RecursiveLearnerConstructionOutcomeComposition
            && self.effect_composition == EffectComposition::Batched
            && self.actual_position_changes == 12
            && self.opposing_output_steps == 4
            && self.final_position == -2
            && !self.reached_lower
            && !self.reached_upper
            && !self.escaped_lower
            && !self.escaped_upper
            && self.completed_cycle_admissions == 10
            && self.cross_view_admissions == 2
            && self.output_choice_resolutions == 23
            && self.propagation_budget_exhaustions == 0
            && !self.stopped
            && self.exact_replay
            && self.naturally_quiescent
    }
}

#[derive(Clone, Debug, Serialize)]
struct Evidence {
    parent_control: ParentControl,
    summary: HandSummary,
    parent_choices: Vec<ChoicePoint>,
    candidate_choices: Vec<ChoicePoint>,
    construction_projections: Vec<ConstructionProjection>,
    localization: Localization,
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn parse_parent(bytes: &[u8]) -> Result<Vec<ChoicePoint>, serde_json::Error> {
    serde_json::from_slice::<ParentEnvelope>(bytes)
        .map(|envelope| envelope.observations.naturality.ordered_choices)
}

fn candidate_choices(hand: &ReflectedHandProtocolEvidence) -> Vec<ChoicePoint> {
    hand.trajectory
        .iter()
        .flat_map(|step| {
            step.output_choice_resolutions
                .iter()
                .map(move |choice| (step.index, choice))
        })
        .enumerate()
        .map(|(ordinal, (hand_step, choice))| ChoicePoint {
            hand_step,
            ordinal,
            tick: choice.tick,
            phase: choice.phase,
            admitted: choice.admitted.clone(),
            admission_basis: choice.admission_basis,
            completed_cycle_state: choice.completed_cycle_state,
            crosses_ownership_view: choice.crosses_ownership_view,
        })
        .collect()
}

fn flatten_trace(hand: &ReflectedHandProtocolEvidence) -> Vec<TracePoint> {
    hand.trajectory
        .iter()
        .flat_map(|step| {
            step.existing_witness_trace
                .iter()
                .cloned()
                .map(|entry| TracePoint {
                    hand_step: step.index,
                    tick: entry.tick,
                    phase: entry.phase,
                    event: entry.event,
                })
        })
        .collect()
}

fn construction_projections(trace: &[TracePoint]) -> Vec<ConstructionProjection> {
    trace
        .iter()
        .enumerate()
        .flat_map(|(index, point)| {
            let ExistingWitnessEvent::LearnerConstructed { learner, .. } = point.event else {
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
                    } if owner == learner => Some(ConstructionProjection {
                        hand_step: point.hand_step,
                        construction_tick: point.tick,
                        learner,
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

fn same_choice_identity(parent: &ChoicePoint, candidate: &ChoicePoint) -> bool {
    parent.ordinal == candidate.ordinal
        && parent.hand_step == candidate.hand_step
        && parent.tick == candidate.tick
        && parent.phase == candidate.phase
        && parent.admitted == candidate.admitted
}

fn first_added_admission(
    parent: &[ChoicePoint],
    candidate: &[ChoicePoint],
) -> Result<Option<usize>, usize> {
    if parent.len() != candidate.len() {
        return Err(parent.len().min(candidate.len()));
    }
    for (index, (old, new)) in parent.iter().zip(candidate).enumerate() {
        if !same_choice_identity(old, new) {
            return Err(index);
        }
        if old.completed_cycle_state != CompletedCycleState::Unique
            && new.completed_cycle_state == CompletedCycleState::Unique
        {
            return Ok(Some(index));
        }
    }
    Ok(None)
}

fn completing_projected_links(
    trace: &[TracePoint],
    projections: &[ConstructionProjection],
    completed: &CompletedCycleContinuationEvidence,
) -> Vec<(LinkId, u32)> {
    let Some(owner) = completed.owner else {
        return Vec::new();
    };
    let Some(consequence_tick) = completed.consequence_tick else {
        return Vec::new();
    };
    let eligible = projections
        .iter()
        .filter(|projection| {
            projection.learner == owner
                && projection.consequence_tick == consequence_tick
                && projection.construction_tick <= completed.tick
        })
        .map(|projection| (projection.link, projection.generation))
        .collect::<BTreeSet<_>>();
    let physically_completing = trace
        .iter()
        .filter_map(|point| match &point.event {
            ExistingWitnessEvent::DriveProvenanceObserved(drive)
                if point.tick == completed.tick
                    && drive.target == completed.target
                    && drive.completes_path =>
            {
                drive.link
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    eligible
        .into_iter()
        .filter(|(link, generation)| {
            physically_completing.contains(link)
                && trace.iter().any(|point| {
                    point.tick == completed.tick
                        && matches!(
                            point.event,
                            ExistingWitnessEvent::CausalLineageMemberObserved {
                                target,
                                link: Some(observed),
                                generation: Some(observed_generation),
                                ..
                            } if target == completed.target
                                && observed == *link
                                && observed_generation == *generation
                        )
                })
        })
        .collect()
}

fn localize(
    parent: &[ChoicePoint],
    candidate: &[ChoicePoint],
    completed: &[CompletedCycleContinuationEvidence],
    projections: &[ConstructionProjection],
    trace: &[TracePoint],
) -> Localization {
    let index = match first_added_admission(parent, candidate) {
        Err(index) => {
            return Localization {
                verdict: LocalizationVerdict::ChoiceIdentityMismatch,
                parent_choice: parent.get(index).cloned(),
                candidate_choice: candidate.get(index).cloned(),
                completed_cycle: None,
                construction_tick: None,
                projected_links: Vec::new(),
                completing_links: Vec::new(),
                decisive_slice: Vec::new(),
                reason: "choice identity changed before a comparable added admission".to_owned(),
            };
        }
        Ok(None) => {
            return Localization {
                verdict: LocalizationVerdict::NoAddedAdmission,
                parent_choice: None,
                candidate_choice: None,
                completed_cycle: None,
                construction_tick: None,
                projected_links: Vec::new(),
                completing_links: Vec::new(),
                decisive_slice: Vec::new(),
                reason: "candidate contains no parent non-Unique to candidate Unique transition"
                    .to_owned(),
            };
        }
        Ok(Some(index)) => index,
    };
    let parent_choice = parent[index].clone();
    let candidate_choice = candidate[index].clone();
    let completed_cycle = completed
        .iter()
        .find(|effect| effect.tick == candidate_choice.tick && effect.admitted)
        .cloned();
    let Some(completed_cycle) = completed_cycle else {
        return Localization {
            verdict: LocalizationVerdict::InsufficientPhysicalComposition,
            parent_choice: Some(parent_choice),
            candidate_choice: Some(candidate_choice),
            completed_cycle: None,
            construction_tick: None,
            projected_links: Vec::new(),
            completing_links: Vec::new(),
            decisive_slice: Vec::new(),
            reason: "the added Unique choice has no retained admitted completed-cycle event"
                .to_owned(),
        };
    };
    let relevant_projections = projections
        .iter()
        .filter(|projection| {
            completed_cycle.owner == Some(projection.learner)
                && completed_cycle.consequence_tick == Some(projection.consequence_tick)
                && projection.construction_tick <= completed_cycle.tick
        })
        .cloned()
        .collect::<Vec<_>>();
    let construction_tick = relevant_projections
        .iter()
        .map(|projection| projection.construction_tick)
        .max();
    let projected_links = relevant_projections
        .iter()
        .filter(|projection| Some(projection.construction_tick) == construction_tick)
        .map(|projection| (projection.link, projection.generation))
        .collect::<Vec<_>>();
    let completing_links =
        completing_projected_links(trace, &relevant_projections, &completed_cycle);
    let decisive_slice = construction_tick.map_or_else(Vec::new, |construction_tick| {
        trace
            .iter()
            .filter(|point| point.tick >= construction_tick && point.tick <= completed_cycle.tick)
            .cloned()
            .collect()
    });
    let composed = !projected_links.is_empty() && !completing_links.is_empty();
    Localization {
        verdict: if composed {
            LocalizationVerdict::FirstAddedAdmission
        } else {
            LocalizationVerdict::InsufficientPhysicalComposition
        },
        parent_choice: Some(parent_choice),
        candidate_choice: Some(candidate_choice),
        completed_cycle: Some(completed_cycle),
        construction_tick,
        projected_links,
        completing_links,
        decisive_slice,
        reason: if composed {
            "the first added admission composes from a same-tick construction projection through a same-generation link that completes the target"
        } else {
            "the choice divergence is located but the retained trace does not compose a projected same-generation link to its target"
        }
        .to_owned(),
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
            .flat_map(|step| step.completed_cycle_continuations.iter())
            .filter(|effect| effect.admitted)
            .count(),
        cross_view_admissions: hand
            .trajectory
            .iter()
            .flat_map(|step| step.completed_cycle_continuations.iter())
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

fn measure(parent_bytes: &[u8]) -> Evidence {
    let parent_hash = sha256(parent_bytes);
    let parent_choices = parse_parent(parent_bytes).unwrap_or_default();
    let parent_control = ParentControl {
        survived: parent_hash == PARENT_SHA256
            && parent_choices.len() == 23
            && parent_choices
                .iter()
                .filter(|choice| choice.completed_cycle_state == CompletedCycleState::Unique)
                .count()
                == 9,
        sha256: parent_hash,
        expected_sha256: PARENT_SHA256,
        choice_count: parent_choices.len(),
        unique_count: parent_choices
            .iter()
            .filter(|choice| choice.completed_cycle_state == CompletedCycleState::Unique)
            .count(),
    };
    let hand = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerConstructionOutcomeComposition,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let candidate_choices = candidate_choices(&hand);
    let completed = hand
        .trajectory
        .iter()
        .flat_map(|step| step.completed_cycle_continuations.iter().cloned())
        .collect::<Vec<_>>();
    let trace = flatten_trace(&hand);
    let construction_projections = construction_projections(&trace);
    let localization = localize(
        &parent_choices,
        &candidate_choices,
        &completed,
        &construction_projections,
        &trace,
    );
    Evidence {
        parent_control,
        summary: summarize(&hand),
        parent_choices,
        candidate_choices,
        construction_projections,
        localization,
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

fn result(arm: Arm, evidence: &Evidence) -> ProbeResult {
    let localized_exactly = evidence.localization.verdict
        == LocalizationVerdict::FirstAddedAdmission
        && evidence
            .localization
            .candidate_choice
            .as_ref()
            .is_some_and(|choice| {
                choice.ordinal == 7
                    && choice.hand_step == 6
                    && choice.tick == 47
                    && choice.admitted
                        == vec![OutputAdmission {
                            target: JunctionId(11),
                            owner: Some(LearnerId(3)),
                        }]
            })
        && evidence.localization.construction_tick == Some(44)
        && evidence
            .localization
            .completed_cycle
            .as_ref()
            .is_some_and(|effect| {
                effect.owner == Some(LearnerId(3))
                    && effect.target == JunctionId(11)
                    && effect.consequence_tick == Some(44)
                    && effect.admitted
            });
    let survived = match arm {
        Arm::ImmutableParentChoiceControl => {
            evidence.parent_control.survived && evidence.summary.exact_candidate()
        }
        Arm::FirstAddedAdmissionLocalization => {
            evidence.parent_control.survived
                && evidence.summary.exact_candidate()
                && localized_exactly
        }
    };
    let falsifier = match arm {
        Arm::ImmutableParentChoiceControl => {
            "the immutable parent choices or exact candidate integrity summary changed"
        }
        Arm::FirstAddedAdmissionLocalization => {
            "the first added admission was not tick-forty-seven learner-three target eleven from a composed tick-forty-four projection"
        }
    };
    ProbeResult {
        schema: "hand-construction-projection-admission-retention/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations: serde_json::json!(evidence),
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: evidence.summary.exact_replay,
        naturally_quiescent: evidence.summary.naturally_quiescent,
    }
}

pub fn run_all(parent_bytes: &[u8]) -> Vec<(Arm, ProbeResult)> {
    let evidence = measure(parent_bytes);
    Arm::ALL
        .into_iter()
        .map(|arm| (arm, result(arm, &evidence)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use developmental_hand_construction_admission::DriveProvenanceEvidence;
    use truelearner_core::TransmissionMode;

    fn choice(ordinal: usize, state: CompletedCycleState, target: u64) -> ChoicePoint {
        ChoicePoint {
            hand_step: ordinal,
            ordinal,
            tick: i64::try_from(ordinal).unwrap(),
            phase: 0,
            admitted: vec![OutputAdmission {
                target: JunctionId(target),
                owner: Some(LearnerId(3)),
            }],
            admission_basis: OutputChoiceBasis::Ordinary,
            completed_cycle_state: state,
            crosses_ownership_view: false,
        }
    }

    #[test]
    fn comparison_finds_only_the_first_added_unique_state() {
        let parent = [
            choice(0, CompletedCycleState::Missing, 10),
            choice(1, CompletedCycleState::Stale, 11),
            choice(2, CompletedCycleState::Missing, 11),
        ];
        let candidate = [
            choice(0, CompletedCycleState::Stale, 10),
            choice(1, CompletedCycleState::Stale, 11),
            choice(2, CompletedCycleState::Unique, 11),
        ];
        assert_eq!(first_added_admission(&parent, &candidate), Ok(Some(2)));
        assert_eq!(first_added_admission(&parent, &parent), Ok(None));
    }

    #[test]
    fn comparison_rejects_choice_identity_change() {
        let parent = [choice(0, CompletedCycleState::Missing, 10)];
        let candidate = [choice(0, CompletedCycleState::Unique, 11)];
        assert_eq!(first_added_admission(&parent, &candidate), Err(0));
    }

    #[test]
    fn physical_composition_requires_projected_generation_and_completed_drive() {
        let completed = CompletedCycleContinuationEvidence {
            tick: 47,
            target: JunctionId(11),
            owner: Some(LearnerId(3)),
            consequence_tick: Some(44),
            consequence_witnesses: vec![(LinkId(34), 3)],
            unique_latest_tick: Some(44),
            crosses_ownership_view: false,
            admitted: true,
        };
        let projections = vec![ConstructionProjection {
            hand_step: 6,
            construction_tick: 44,
            learner: LearnerId(3),
            link: LinkId(34),
            generation: 3,
            consequence_tick: 44,
        }];
        let drive = DriveProvenanceEvidence {
            ordinal: 0,
            tick: 47,
            phase: 0,
            causal_wave: 0,
            source: Some(JunctionId(20)),
            target: JunctionId(11),
            source_physical: Some(40_000),
            target_physical: 20_001,
            source_region: Some(0),
            target_region: 0,
            is_motor: true,
            link: Some(LinkId(34)),
            completes_path: true,
            carried_origin: 10_001,
            origin_owner: Some(LearnerId(3)),
            path_owner: Some(LearnerId(3)),
            strength: 1,
        };
        let trace = vec![
            TracePoint {
                hand_step: 6,
                tick: 47,
                phase: 0,
                event: ExistingWitnessEvent::DriveProvenanceObserved(drive),
            },
            TracePoint {
                hand_step: 6,
                tick: 47,
                phase: 0,
                event: ExistingWitnessEvent::CausalLineageMemberObserved {
                    target: JunctionId(11),
                    origin_physical: 10_001,
                    mode: TransmissionMode::Drive,
                    link: Some(LinkId(34)),
                    generation: Some(3),
                    causal_wave: 0,
                },
            },
        ];

        assert_eq!(
            completing_projected_links(&trace, &projections, &completed),
            vec![(LinkId(34), 3)]
        );
        assert!(completing_projected_links(&[], &projections, &completed).is_empty());
    }

    #[test]
    fn immutable_parent_parser_reads_real_ordered_choices() {
        let bytes = include_bytes!(
            "../../../campaigns/hand-completed-cycle-naturality-v1/artifacts/completed-cycle-first-arrow-change.json"
        );
        let choices = parse_parent(bytes).expect("immutable parent parses");
        assert_eq!(sha256(bytes), PARENT_SHA256);
        assert_eq!(choices.len(), 23);
        assert_eq!(
            choices
                .iter()
                .filter(|choice| choice.completed_cycle_state == CompletedCycleState::Unique)
                .count(),
            9
        );
    }
}
