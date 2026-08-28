#![forbid(unsafe_code)]

use serde::Serialize;
use std::any::Any;
use std::collections::BTreeSet;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::str::FromStr;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use truelearner_core::{
    CandidateOwnership, Checkpoint, CompletedCycleState, ExecutionCost, FreshOpportunityDecision,
    Harness, HarnessBuilder, Input, Junction, JunctionId, LearnerId, LearnerOwnershipRelation,
    Link, LinkId, Output, OutputAdmission, OutputChoiceBasis, PhysicalEvent, PhysicalIncidence,
    PhysicalInput, Protocol, ReturnOriginDecision, Run, TransmissionMode, Work,
};

const OUTWARD_REGION: i16 = 1;
const LOWER: i16 = -4;
const UPPER: i16 = 4;
const PRIMARY_STEPS: usize = 16;
const DEVELOPMENT_STEPS: usize = 8;
const RECOVERY_STEPS: usize = 16;
const JUNCTION_CAPACITY: u32 = 16_384;
const LINK_CAPACITY: u32 = 65_536;
const WARM_LIMIT: Duration = Duration::from_secs(10);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    InheritedIntegrityControl,
    FrozenTruthfulRecursiveReference,
    SymmetricSurfacePathOnly,
    PhysicalSurfaceDeliveryOnly,
    TruthfulHandConstructionAdmission,
    ChildFreshnessBoundary,
    ReflectedJointRetry,
    SurfacePathDeliveryFactorialLocalization,
}

impl Arm {
    pub const ALL: [Self; 8] = [
        Self::TruthfulHandConstructionAdmission,
        Self::InheritedIntegrityControl,
        Self::FrozenTruthfulRecursiveReference,
        Self::SymmetricSurfacePathOnly,
        Self::PhysicalSurfaceDeliveryOnly,
        Self::ChildFreshnessBoundary,
        Self::ReflectedJointRetry,
        Self::SurfacePathDeliveryFactorialLocalization,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::InheritedIntegrityControl => "inherited-integrity-control",
            Self::FrozenTruthfulRecursiveReference => "frozen-truthful-recursive-reference",
            Self::SymmetricSurfacePathOnly => "symmetric-surface-path-only",
            Self::PhysicalSurfaceDeliveryOnly => "physical-surface-delivery-only",
            Self::TruthfulHandConstructionAdmission => "truthful-hand-construction-admission",
            Self::ChildFreshnessBoundary => "child-freshness-boundary",
            Self::ReflectedJointRetry => "reflected-joint-retry",
            Self::SurfacePathDeliveryFactorialLocalization => {
                "surface-path-delivery-factorial-localization"
            }
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
    outcome: &'static str,
    observations: serde_json::Value,
    falsifier: Option<String>,
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "developmental-hand-construction-admission/v1",
        arm: arm.id(),
        outcome,
        observations,
        falsifier,
        exact_replay,
        naturally_quiescent,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum SurfacePath {
    Absent,
    Symmetric,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ReturnDelivery {
    DirectOutcome,
    PhysicalSurface,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
struct AdmissionCell {
    surface_path: SurfacePath,
    return_delivery: ReturnDelivery,
}

impl AdmissionCell {
    const FROZEN: Self = Self {
        surface_path: SurfacePath::Absent,
        return_delivery: ReturnDelivery::DirectOutcome,
    };
    const PATH_ONLY: Self = Self {
        surface_path: SurfacePath::Symmetric,
        return_delivery: ReturnDelivery::DirectOutcome,
    };
    const DELIVERY_ONLY: Self = Self {
        surface_path: SurfacePath::Absent,
        return_delivery: ReturnDelivery::PhysicalSurface,
    };
    const COMPLETE: Self = Self {
        surface_path: SurfacePath::Symmetric,
        return_delivery: ReturnDelivery::PhysicalSurface,
    };
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EffectComposition {
    #[default]
    Batched,
    QuiescentPhaseSequential,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PendingReturn {
    Direct { target: JunctionId, origin: u64 },
    Surfaces(Vec<usize>),
}

#[derive(Clone)]
struct WorldCheckpoint {
    harness: Checkpoint,
    position: i16,
    pending: Vec<PendingReturn>,
    sequence: u64,
    max_moments: Option<u64>,
    effect_composition: EffectComposition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum PhysicalStop {
    JunctionCapacity,
    LinkCapacity,
    WarmRuntime,
}

fn panic_text(payload: &(dyn Any + Send)) -> Option<&str> {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
}

fn classify_capacity(payload: &(dyn Any + Send)) -> Option<PhysicalStop> {
    match panic_text(payload) {
        Some("arena has no free junction slot") => Some(PhysicalStop::JunctionCapacity),
        Some("arena has no free link identity") => Some(PhysicalStop::LinkCapacity),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
enum OwnerEvent {
    Admission {
        owner: LearnerId,
        admitted: bool,
    },
    Write {
        owner: LearnerId,
    },
    Read {
        owner: LearnerId,
        consequential: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SurfacePathEvidence {
    pub surface: JunctionId,
    pub owner: Option<LearnerId>,
    pub complete_paths: u32,
    pub consequential_paths: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OutputCandidateEvidence {
    pub target: JunctionId,
    pub is_motor: bool,
    pub ownership: CandidateOwnership,
    pub path_inputs: u32,
    pub distinct_path_origins: u32,
    pub distinct_path_owners: u32,
    pub positive_path_strength: u64,
    pub negative_path_strength: u64,
    pub opportunity: i64,
    pub supplied_opportunity: i64,
    pub admitted_drive: i64,
    pub projected_drive: i64,
    pub threshold: i64,
    pub consequence_tick: Option<i64>,
    pub unanswered_returns: u32,
    pub executable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FreshOpportunityEvidence {
    pub tick: i64,
    pub donor: JunctionId,
    pub recipient: JunctionId,
    pub return_link: LinkId,
    pub owner: Option<LearnerId>,
    pub opportunity: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FreshOpportunityEvaluationEvidence {
    pub tick: i64,
    pub donor: JunctionId,
    pub recipient: JunctionId,
    pub return_link: LinkId,
    pub return_owner: Option<LearnerId>,
    pub recipient_owner: Option<LearnerId>,
    pub ownership_relation: LearnerOwnershipRelation,
    pub decision: FreshOpportunityDecision,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PhysicalTransitionContinuationEvidence {
    pub tick: i64,
    pub target: JunctionId,
    pub owner: Option<LearnerId>,
    pub current_owner_transition: bool,
    pub unanswered_returns: u32,
    pub admitted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CoherentEffectEvidence {
    pub tick: i64,
    pub target: JunctionId,
    pub owner: Option<LearnerId>,
    pub latest_unanswered_opened_tick: Option<i64>,
    pub unanswered_returns: u32,
    pub admitted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CompletedCycleContinuationEvidence {
    pub tick: i64,
    pub target: JunctionId,
    pub owner: Option<LearnerId>,
    pub consequence_tick: Option<i64>,
    pub consequence_witnesses: Vec<(LinkId, u32)>,
    pub unique_latest_tick: Option<i64>,
    pub crosses_ownership_view: bool,
    pub admitted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OutputChoiceResolutionEvidence {
    pub tick: i64,
    pub phase: i32,
    pub ordinary_target: JunctionId,
    pub current_transition_target: Option<JunctionId>,
    pub coherent_effect_target: Option<JunctionId>,
    pub completed_cycle_target: Option<JunctionId>,
    pub computed_winner_target: JunctionId,
    pub admitted: Vec<OutputAdmission>,
    pub computed_winner_basis: OutputChoiceBasis,
    pub admission_basis: OutputChoiceBasis,
    pub completed_cycle_state: CompletedCycleState,
    pub crosses_ownership_view: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "event", content = "evidence")]
pub enum ExistingWitnessEvent {
    CausalLineageMemberObserved {
        target: JunctionId,
        origin_physical: u64,
        mode: TransmissionMode,
        link: Option<LinkId>,
        generation: Option<u32>,
        causal_wave: u64,
    },
    DriveProvenanceObserved(DriveProvenanceEvidence),
    LinkDeallocated {
        link: LinkId,
    },
    QualifiedLocalTraversal {
        link: LinkId,
    },
    ConsequenceRecorded {
        link: LinkId,
        junction: JunctionId,
    },
    LearnerConsequenceRecorded {
        owner: LearnerId,
        link: LinkId,
        generation: u32,
        consequence_tick: i64,
    },
    LearnerConstructed {
        learner: LearnerId,
        parent: Option<LearnerId>,
        surface: JunctionId,
        output: JunctionId,
        junction_count: u32,
        link_count: u32,
    },
    LearnerCandidatePreference {
        owner: LearnerId,
        target: JunctionId,
        consequence_tick: Option<i64>,
        admitted: bool,
    },
    SurfacePathStateObserved(SurfacePathEvidence),
    OutputCandidateEvaluated(OutputCandidateEvidence),
    ReturnScheduling {
        owner: Option<LearnerId>,
        link: LinkId,
        generation: u32,
        admitted: bool,
    },
    ReturnOriginEvaluated(ReturnOriginEvidence),
    CandidateSelection(CandidateSelectionEvidence),
    FreshOpportunityTransferred(FreshOpportunityEvidence),
    PhysicalTransitionEligibilityEvaluated(PhysicalTransitionEligibilityEvidence),
    CompletedCycleContinuationEvaluated(CompletedCycleContinuationEvidence),
    ConstructionContinuationConsumed {
        target: JunctionId,
        owner: LearnerId,
        link: LinkId,
        generation: u32,
        consequence_tick: i64,
    },
    OutputChoiceResolved(OutputChoiceResolutionEvidence),
    ReturnSuperseded {
        link: LinkId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExistingWitnessTraceEntry {
    pub tick: i64,
    pub phase: i32,
    pub event: ExistingWitnessEvent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DriveProvenanceEvidence {
    pub ordinal: u64,
    pub tick: i64,
    pub phase: i32,
    pub causal_wave: u64,
    pub source: Option<JunctionId>,
    pub target: JunctionId,
    pub source_physical: Option<u64>,
    pub target_physical: u64,
    pub source_region: Option<i16>,
    pub target_region: i16,
    pub is_motor: bool,
    pub link: Option<LinkId>,
    pub completes_path: bool,
    pub carried_origin: u64,
    pub origin_owner: Option<LearnerId>,
    pub path_owner: Option<LearnerId>,
    pub strength: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CausalOriginSelectionEvidence {
    pub target: JunctionId,
    pub is_motor: bool,
    pub origin_count: u32,
    pub executable_groups: u32,
    pub selected_origin: Option<u64>,
    pub selected_ownership: Option<CandidateOwnership>,
    pub selected_path_inputs: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CandidateSelectionEvidence {
    pub target: JunctionId,
    pub is_motor: bool,
    pub origin_scope: Option<u64>,
    pub consequence_tick: Option<i64>,
    pub admitted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReturnOriginEvidence {
    pub tick: i64,
    pub owner: Option<LearnerId>,
    pub link: LinkId,
    pub origin_physical: u64,
    pub source: Option<JunctionId>,
    pub target: Option<JunctionId>,
    pub origin: Option<JunctionId>,
    pub distance: Option<i32>,
    pub decision: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsequenceWriteEvidence {
    pub tick: i64,
    pub junction: JunctionId,
    pub link: LinkId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ClosureEligibilityEvidence {
    pub tick: i64,
    pub return_link: LinkId,
    pub origin_physical: u64,
    pub origin_birth_tick: i64,
    pub return_opened_tick: i64,
    pub eligible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PhysicalIncidenceEvidence {
    pub tick: i64,
    pub target: JunctionId,
    pub origin_physical: u64,
    pub incidence: PhysicalIncidence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PhysicalTransitionEligibilityEvidence {
    pub tick: i64,
    pub return_link: LinkId,
    pub origin_physical: u64,
    pub transition_tick: Option<i64>,
    pub return_opened_tick: i64,
    pub eligible: bool,
}

fn return_origin_decision(decision: ReturnOriginDecision) -> &'static str {
    match decision {
        ReturnOriginDecision::AdmittedDirect => "admitted-direct",
        ReturnOriginDecision::AdmittedLocal => "admitted-local",
        ReturnOriginDecision::RejectedAlreadyRemembered => "rejected-already-remembered",
        ReturnOriginDecision::RejectedAlreadyAdmittedThisMoment => {
            "rejected-already-admitted-this-moment"
        }
        ReturnOriginDecision::RejectedBeforeReturnOpened => "rejected-before-return-opened",
        ReturnOriginDecision::RejectedUnchangedSample => "rejected-unchanged-sample",
        ReturnOriginDecision::RejectedMissingLink => "rejected-missing-link",
        ReturnOriginDecision::RejectedInactiveLink => "rejected-inactive-link",
        ReturnOriginDecision::RejectedWrongMode => "rejected-wrong-mode",
        ReturnOriginDecision::RejectedMissingSource => "rejected-missing-source",
        ReturnOriginDecision::RejectedMissingTarget => "rejected-missing-target",
        ReturnOriginDecision::RejectedOriginNotFound => "rejected-origin-not-found",
        ReturnOriginDecision::RejectedNonLocal => "rejected-non-local",
    }
}

fn existing_witness_event(
    event: &PhysicalEvent,
    motors: [JunctionId; 2],
) -> Option<ExistingWitnessEvent> {
    match event {
        PhysicalEvent::CausalLineageMemberObserved {
            target,
            origin_physical,
            mode,
            link,
            generation,
            causal_wave,
        } => Some(ExistingWitnessEvent::CausalLineageMemberObserved {
            target: *target,
            origin_physical: *origin_physical,
            mode: *mode,
            link: *link,
            generation: *generation,
            causal_wave: *causal_wave,
        }),
        PhysicalEvent::DriveProvenanceObserved {
            source,
            target,
            source_physical,
            target_physical,
            source_region,
            target_region,
            link,
            completes_path,
            carried_origin,
            origin_owner,
            path_owner,
            strength,
            causal_wave,
        } => Some(ExistingWitnessEvent::DriveProvenanceObserved(
            DriveProvenanceEvidence {
                ordinal: 0,
                tick: 0,
                phase: 0,
                causal_wave: *causal_wave,
                source: *source,
                target: *target,
                source_physical: *source_physical,
                target_physical: *target_physical,
                source_region: *source_region,
                target_region: *target_region,
                is_motor: motors.contains(target),
                link: *link,
                completes_path: *completes_path,
                carried_origin: *carried_origin,
                origin_owner: *origin_owner,
                path_owner: *path_owner,
                strength: *strength,
            },
        )),
        PhysicalEvent::Deallocate { link } => {
            Some(ExistingWitnessEvent::LinkDeallocated { link: *link })
        }
        PhysicalEvent::QualifiedLocalTraversal { link } => {
            Some(ExistingWitnessEvent::QualifiedLocalTraversal { link: *link })
        }
        PhysicalEvent::ConsequenceRecorded { link, junction } => {
            Some(ExistingWitnessEvent::ConsequenceRecorded {
                link: *link,
                junction: *junction,
            })
        }
        PhysicalEvent::LearnerConsequenceRecorded {
            owner,
            link,
            generation,
            tick,
        } => Some(ExistingWitnessEvent::LearnerConsequenceRecorded {
            owner: *owner,
            link: *link,
            generation: *generation,
            consequence_tick: *tick,
        }),
        PhysicalEvent::LearnerConstructed {
            learner,
            parent,
            surface,
            output,
            junction_count,
            link_count,
        } => Some(ExistingWitnessEvent::LearnerConstructed {
            learner: *learner,
            parent: *parent,
            surface: *surface,
            output: *output,
            junction_count: *junction_count,
            link_count: *link_count,
        }),
        PhysicalEvent::LearnerCandidatePreference {
            owner,
            target,
            consequence_tick,
            admitted,
        } => Some(ExistingWitnessEvent::LearnerCandidatePreference {
            owner: *owner,
            target: *target,
            consequence_tick: *consequence_tick,
            admitted: *admitted,
        }),
        PhysicalEvent::SurfacePathStateObserved {
            surface,
            owner,
            complete_paths,
            consequential_paths,
        } => Some(ExistingWitnessEvent::SurfacePathStateObserved(
            SurfacePathEvidence {
                surface: *surface,
                owner: *owner,
                complete_paths: *complete_paths,
                consequential_paths: *consequential_paths,
            },
        )),
        PhysicalEvent::OutputCandidateEvaluated {
            target,
            ownership,
            path_inputs,
            distinct_path_origins,
            distinct_path_owners,
            positive_path_strength,
            negative_path_strength,
            opportunity,
            supplied_opportunity,
            admitted_drive,
            projected_drive,
            threshold,
            consequence_tick,
            unanswered_returns,
            executable,
        } => Some(ExistingWitnessEvent::OutputCandidateEvaluated(
            OutputCandidateEvidence {
                target: *target,
                is_motor: motors.contains(target),
                ownership: *ownership,
                path_inputs: *path_inputs,
                distinct_path_origins: *distinct_path_origins,
                distinct_path_owners: *distinct_path_owners,
                positive_path_strength: *positive_path_strength,
                negative_path_strength: *negative_path_strength,
                opportunity: *opportunity,
                supplied_opportunity: *supplied_opportunity,
                admitted_drive: *admitted_drive,
                projected_drive: *projected_drive,
                threshold: *threshold,
                consequence_tick: *consequence_tick,
                unanswered_returns: *unanswered_returns,
                executable: *executable,
            },
        )),
        PhysicalEvent::ReturnScheduling {
            owner,
            link,
            generation,
            admitted,
        } => Some(ExistingWitnessEvent::ReturnScheduling {
            owner: *owner,
            link: *link,
            generation: *generation,
            admitted: *admitted,
        }),
        PhysicalEvent::ReturnOriginEvaluated {
            owner,
            link,
            origin_physical,
            source,
            target,
            origin,
            distance,
            decision,
            ..
        } => Some(ExistingWitnessEvent::ReturnOriginEvaluated(
            ReturnOriginEvidence {
                tick: 0,
                owner: *owner,
                link: *link,
                origin_physical: *origin_physical,
                source: *source,
                target: *target,
                origin: *origin,
                distance: *distance,
                decision: return_origin_decision(*decision),
            },
        )),
        PhysicalEvent::CandidateSelection {
            target,
            origin_scope,
            consequence_tick,
            admitted,
        } => Some(ExistingWitnessEvent::CandidateSelection(
            CandidateSelectionEvidence {
                target: *target,
                is_motor: motors.contains(target),
                origin_scope: *origin_scope,
                consequence_tick: *consequence_tick,
                admitted: *admitted,
            },
        )),
        PhysicalEvent::FreshOpportunityTransferred {
            donor,
            recipient,
            return_link,
            owner,
            opportunity,
        } => Some(ExistingWitnessEvent::FreshOpportunityTransferred(
            FreshOpportunityEvidence {
                tick: 0,
                donor: *donor,
                recipient: *recipient,
                return_link: *return_link,
                owner: *owner,
                opportunity: *opportunity,
            },
        )),
        PhysicalEvent::PhysicalTransitionEligibilityEvaluated {
            return_link,
            origin_physical,
            transition_tick,
            return_opened_tick,
            eligible,
        } => Some(
            ExistingWitnessEvent::PhysicalTransitionEligibilityEvaluated(
                PhysicalTransitionEligibilityEvidence {
                    tick: 0,
                    return_link: *return_link,
                    origin_physical: *origin_physical,
                    transition_tick: *transition_tick,
                    return_opened_tick: *return_opened_tick,
                    eligible: *eligible,
                },
            ),
        ),
        PhysicalEvent::CompletedCycleContinuationEvaluated {
            target,
            owner,
            consequence_tick,
            consequence_witnesses,
            unique_latest_tick,
            crosses_ownership_view,
            admitted,
        } => Some(ExistingWitnessEvent::CompletedCycleContinuationEvaluated(
            CompletedCycleContinuationEvidence {
                tick: 0,
                target: *target,
                owner: *owner,
                consequence_tick: *consequence_tick,
                consequence_witnesses: consequence_witnesses.clone(),
                unique_latest_tick: *unique_latest_tick,
                crosses_ownership_view: *crosses_ownership_view,
                admitted: *admitted,
            },
        )),
        PhysicalEvent::ConstructionContinuationConsumed {
            target,
            owner,
            link,
            generation,
            consequence_tick,
        } => Some(ExistingWitnessEvent::ConstructionContinuationConsumed {
            target: *target,
            owner: *owner,
            link: *link,
            generation: *generation,
            consequence_tick: *consequence_tick,
        }),
        PhysicalEvent::OutputChoiceResolved {
            ordinary_target,
            current_transition_target,
            coherent_effect_target,
            completed_cycle_target,
            computed_winner_target,
            admitted,
            computed_winner_basis,
            admission_basis,
            completed_cycle_state,
            crosses_ownership_view,
        } => Some(ExistingWitnessEvent::OutputChoiceResolved(
            OutputChoiceResolutionEvidence {
                tick: 0,
                phase: 0,
                ordinary_target: *ordinary_target,
                current_transition_target: *current_transition_target,
                coherent_effect_target: *coherent_effect_target,
                completed_cycle_target: *completed_cycle_target,
                computed_winner_target: *computed_winner_target,
                admitted: admitted.clone(),
                computed_winner_basis: *computed_winner_basis,
                admission_basis: *admission_basis,
                completed_cycle_state: *completed_cycle_state,
                crosses_ownership_view: *crosses_ownership_view,
            },
        )),
        PhysicalEvent::ReturnSuperseded { link } => {
            Some(ExistingWitnessEvent::ReturnSuperseded { link: *link })
        }
        _ => None,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
struct EventSummary {
    drive_incidence: u64,
    return_scheduling: u64,
    return_admissions: u64,
    rejected_returns: u64,
    reverse_consolidations: u64,
    closure_observations: u64,
    constructions: u64,
    owner_writes: u64,
    owner_reads: u64,
    consequential_owner_reads: u64,
    #[serde(skip)]
    boundary_novelty_checks: u64,
    #[serde(skip)]
    boundary_novelty_rejections: u64,
    #[serde(skip)]
    surface_paths: Vec<SurfacePathEvidence>,
    #[serde(skip)]
    output_candidates: Vec<OutputCandidateEvidence>,
    #[serde(skip)]
    fresh_opportunities: Vec<FreshOpportunityEvidence>,
    #[serde(skip)]
    fresh_opportunity_evaluations: Vec<FreshOpportunityEvaluationEvidence>,
    #[serde(skip)]
    physical_transition_continuations: Vec<PhysicalTransitionContinuationEvidence>,
    #[serde(skip)]
    coherent_effects: Vec<CoherentEffectEvidence>,
    #[serde(skip)]
    completed_cycle_continuations: Vec<CompletedCycleContinuationEvidence>,
    #[serde(skip)]
    output_choice_resolutions: Vec<OutputChoiceResolutionEvidence>,
    #[serde(skip)]
    existing_witness_trace: Vec<ExistingWitnessTraceEntry>,
    #[serde(skip)]
    superseded_returns: Vec<LinkId>,
    #[serde(skip)]
    drive_provenance: Vec<DriveProvenanceEvidence>,
    #[serde(skip)]
    causal_origin_selection: Vec<CausalOriginSelectionEvidence>,
    #[serde(skip)]
    candidate_selection: Vec<CandidateSelectionEvidence>,
    #[serde(skip)]
    return_origins: Vec<ReturnOriginEvidence>,
    #[serde(skip)]
    consequence_writes: Vec<ConsequenceWriteEvidence>,
    #[serde(skip)]
    closure_eligibility: Vec<ClosureEligibilityEvidence>,
    #[serde(skip)]
    physical_incidences: Vec<PhysicalIncidenceEvidence>,
    #[serde(skip)]
    transition_eligibility: Vec<PhysicalTransitionEligibilityEvidence>,
    #[serde(skip)]
    mixed_owner_checks: u64,
    #[serde(skip)]
    mixed_owner_selections: u64,
    #[serde(skip)]
    causal_origin_checks: u64,
    #[serde(skip)]
    causal_origin_selections: u64,
    #[serde(skip)]
    propagation_budget_exhaustions: u64,
    closure_evidence: Vec<u32>,
    constructed_learners: Vec<LearnerId>,
    owner_events: Vec<OwnerEvent>,
}

impl EventSummary {
    fn observe(&mut self, run: &Run, motors: [JunctionId; 2]) {
        for transition in &run.physical_trace {
            if let Some(event) = existing_witness_event(&transition.event, motors) {
                self.existing_witness_trace.push(ExistingWitnessTraceEntry {
                    tick: transition.tick,
                    phase: transition.phase,
                    event,
                });
            }
            match transition.event {
                PhysicalEvent::PhysicalIncidenceObserved {
                    target,
                    origin_physical,
                    incidence,
                    ..
                } => self.physical_incidences.push(PhysicalIncidenceEvidence {
                    tick: transition.tick,
                    target,
                    origin_physical,
                    incidence,
                }),
                PhysicalEvent::DriveIncidence { .. } => self.drive_incidence += 1,
                PhysicalEvent::ReturnScheduling { .. } => self.return_scheduling += 1,
                PhysicalEvent::ReturnOriginAdmission {
                    owner, admitted, ..
                } => {
                    if admitted {
                        self.return_admissions += 1;
                    } else {
                        self.rejected_returns += 1;
                    }
                    if let Some(owner) = owner {
                        self.owner_events
                            .push(OwnerEvent::Admission { owner, admitted });
                    }
                }
                PhysicalEvent::ReversePathConsolidated { .. } => {
                    self.reverse_consolidations += 1;
                }
                PhysicalEvent::CausalClosureObserved { evidence, .. } => {
                    self.closure_observations += 1;
                    self.closure_evidence.push(evidence);
                }
                PhysicalEvent::LearnerConstructed { learner, .. } => {
                    self.constructions += 1;
                    self.constructed_learners.push(learner);
                }
                PhysicalEvent::BoundaryNoveltyEvaluated { eligible, .. } => {
                    self.boundary_novelty_checks += 1;
                    self.boundary_novelty_rejections += u64::from(!eligible);
                }
                PhysicalEvent::LearnerConsequenceRecorded { owner, .. } => {
                    self.owner_writes += 1;
                    self.owner_events.push(OwnerEvent::Write { owner });
                }
                PhysicalEvent::LearnerCandidatePreference {
                    owner,
                    consequence_tick,
                    ..
                } => {
                    self.owner_reads += 1;
                    let consequential = consequence_tick.is_some();
                    self.consequential_owner_reads += u64::from(consequential);
                    self.owner_events.push(OwnerEvent::Read {
                        owner,
                        consequential,
                    });
                }
                PhysicalEvent::SurfacePathStateObserved {
                    surface,
                    owner,
                    complete_paths,
                    consequential_paths,
                } => self.surface_paths.push(SurfacePathEvidence {
                    surface,
                    owner,
                    complete_paths,
                    consequential_paths,
                }),
                PhysicalEvent::OutputCandidateEvaluated {
                    target,
                    ownership,
                    path_inputs,
                    distinct_path_origins,
                    distinct_path_owners,
                    positive_path_strength,
                    negative_path_strength,
                    opportunity,
                    supplied_opportunity,
                    admitted_drive,
                    projected_drive,
                    threshold,
                    consequence_tick,
                    unanswered_returns,
                    executable,
                } => self.output_candidates.push(OutputCandidateEvidence {
                    target,
                    is_motor: motors.contains(&target),
                    ownership,
                    path_inputs,
                    distinct_path_origins,
                    distinct_path_owners,
                    positive_path_strength,
                    negative_path_strength,
                    opportunity,
                    supplied_opportunity,
                    admitted_drive,
                    projected_drive,
                    threshold,
                    consequence_tick,
                    unanswered_returns,
                    executable,
                }),
                PhysicalEvent::FreshOpportunityTransferred {
                    donor,
                    recipient,
                    return_link,
                    owner,
                    opportunity,
                } => self.fresh_opportunities.push(FreshOpportunityEvidence {
                    tick: transition.tick,
                    donor,
                    recipient,
                    return_link,
                    owner,
                    opportunity,
                }),
                PhysicalEvent::FreshOpportunityEvaluated {
                    donor,
                    recipient,
                    return_link,
                    return_owner,
                    recipient_owner,
                    ownership_relation,
                    decision,
                } => self
                    .fresh_opportunity_evaluations
                    .push(FreshOpportunityEvaluationEvidence {
                        tick: transition.tick,
                        donor,
                        recipient,
                        return_link,
                        return_owner,
                        recipient_owner,
                        ownership_relation,
                        decision,
                    }),
                PhysicalEvent::PhysicalTransitionContinuationEvaluated {
                    target,
                    owner,
                    current_owner_transition,
                    unanswered_returns,
                    admitted,
                } => self.physical_transition_continuations.push(
                    PhysicalTransitionContinuationEvidence {
                        tick: transition.tick,
                        target,
                        owner,
                        current_owner_transition,
                        unanswered_returns,
                        admitted,
                    },
                ),
                PhysicalEvent::CoherentEffectEvaluated {
                    target,
                    owner,
                    latest_unanswered_opened_tick,
                    unanswered_returns,
                    admitted,
                } => self.coherent_effects.push(CoherentEffectEvidence {
                    tick: transition.tick,
                    target,
                    owner,
                    latest_unanswered_opened_tick,
                    unanswered_returns,
                    admitted,
                }),
                PhysicalEvent::CompletedCycleContinuationEvaluated {
                    target,
                    owner,
                    consequence_tick,
                    ref consequence_witnesses,
                    unique_latest_tick,
                    crosses_ownership_view,
                    admitted,
                } => self
                    .completed_cycle_continuations
                    .push(CompletedCycleContinuationEvidence {
                        tick: transition.tick,
                        target,
                        owner,
                        consequence_tick,
                        consequence_witnesses: consequence_witnesses.clone(),
                        unique_latest_tick,
                        crosses_ownership_view,
                        admitted,
                    }),
                PhysicalEvent::OutputChoiceResolved {
                    ordinary_target,
                    current_transition_target,
                    coherent_effect_target,
                    completed_cycle_target,
                    computed_winner_target,
                    ref admitted,
                    computed_winner_basis,
                    admission_basis,
                    completed_cycle_state,
                    crosses_ownership_view,
                } => self
                    .output_choice_resolutions
                    .push(OutputChoiceResolutionEvidence {
                        tick: transition.tick,
                        phase: transition.phase,
                        ordinary_target,
                        current_transition_target,
                        coherent_effect_target,
                        completed_cycle_target,
                        computed_winner_target,
                        admitted: admitted.clone(),
                        computed_winner_basis,
                        admission_basis,
                        completed_cycle_state,
                        crosses_ownership_view,
                    }),
                PhysicalEvent::ReturnSuperseded { link } => {
                    self.superseded_returns.push(link);
                }
                PhysicalEvent::DriveProvenanceObserved {
                    source,
                    target,
                    source_physical,
                    target_physical,
                    source_region,
                    target_region,
                    link,
                    completes_path,
                    carried_origin,
                    origin_owner,
                    path_owner,
                    strength,
                    causal_wave,
                } => self.drive_provenance.push(DriveProvenanceEvidence {
                    ordinal: u64::try_from(self.drive_provenance.len()).unwrap_or(u64::MAX),
                    tick: transition.tick,
                    phase: transition.phase,
                    causal_wave,
                    source,
                    target,
                    source_physical,
                    target_physical,
                    source_region,
                    target_region,
                    is_motor: motors.contains(&target),
                    link,
                    completes_path,
                    carried_origin,
                    origin_owner,
                    path_owner,
                    strength,
                }),
                PhysicalEvent::MixedOwnerCandidateResolved { selected_owner, .. } => {
                    self.mixed_owner_checks += 1;
                    self.mixed_owner_selections += u64::from(selected_owner.is_some());
                }
                PhysicalEvent::CausalOriginCandidateResolved {
                    target,
                    origin_count,
                    executable_groups,
                    selected_origin,
                    selected_ownership,
                    selected_path_inputs,
                } => {
                    self.causal_origin_checks += 1;
                    self.causal_origin_selections += u64::from(selected_origin.is_some());
                    self.causal_origin_selection
                        .push(CausalOriginSelectionEvidence {
                            target,
                            is_motor: motors.contains(&target),
                            origin_count,
                            executable_groups,
                            selected_origin,
                            selected_ownership,
                            selected_path_inputs,
                        });
                }
                PhysicalEvent::CandidateSelection {
                    target,
                    origin_scope,
                    consequence_tick,
                    admitted,
                } => self.candidate_selection.push(CandidateSelectionEvidence {
                    target,
                    is_motor: motors.contains(&target),
                    origin_scope,
                    consequence_tick,
                    admitted,
                }),
                PhysicalEvent::ReturnOriginEvaluated {
                    owner,
                    link,
                    origin_physical,
                    source,
                    target,
                    origin,
                    distance,
                    decision,
                    ..
                } => self.return_origins.push(ReturnOriginEvidence {
                    tick: transition.tick,
                    owner,
                    link,
                    origin_physical,
                    source,
                    target,
                    origin,
                    distance,
                    decision: return_origin_decision(decision),
                }),
                PhysicalEvent::ConsequenceRecorded { link, junction } => {
                    self.consequence_writes.push(ConsequenceWriteEvidence {
                        tick: transition.tick,
                        junction,
                        link,
                    });
                }
                PhysicalEvent::ClosureEligibilityEvaluated {
                    return_link,
                    origin_physical,
                    origin_birth_tick,
                    return_opened_tick,
                    eligible,
                } => self.closure_eligibility.push(ClosureEligibilityEvidence {
                    tick: transition.tick,
                    return_link,
                    origin_physical,
                    origin_birth_tick,
                    return_opened_tick,
                    eligible,
                }),
                PhysicalEvent::PhysicalTransitionEligibilityEvaluated {
                    return_link,
                    origin_physical,
                    transition_tick,
                    return_opened_tick,
                    eligible,
                } => self
                    .transition_eligibility
                    .push(PhysicalTransitionEligibilityEvidence {
                        tick: transition.tick,
                        return_link,
                        origin_physical,
                        transition_tick,
                        return_opened_tick,
                        eligible,
                    }),
                PhysicalEvent::PropagationBudgetExhausted { .. } => {
                    self.propagation_budget_exhaustions += 1;
                }
                _ => {}
            }
        }
    }

    fn merge(&mut self, other: &Self) {
        self.drive_incidence += other.drive_incidence;
        self.return_scheduling += other.return_scheduling;
        self.return_admissions += other.return_admissions;
        self.rejected_returns += other.rejected_returns;
        self.reverse_consolidations += other.reverse_consolidations;
        self.closure_observations += other.closure_observations;
        self.constructions += other.constructions;
        self.owner_writes += other.owner_writes;
        self.owner_reads += other.owner_reads;
        self.consequential_owner_reads += other.consequential_owner_reads;
        self.boundary_novelty_checks += other.boundary_novelty_checks;
        self.boundary_novelty_rejections += other.boundary_novelty_rejections;
        self.surface_paths.extend_from_slice(&other.surface_paths);
        self.output_candidates
            .extend_from_slice(&other.output_candidates);
        self.fresh_opportunities
            .extend_from_slice(&other.fresh_opportunities);
        self.fresh_opportunity_evaluations
            .extend_from_slice(&other.fresh_opportunity_evaluations);
        self.physical_transition_continuations
            .extend_from_slice(&other.physical_transition_continuations);
        self.coherent_effects
            .extend_from_slice(&other.coherent_effects);
        self.completed_cycle_continuations
            .extend_from_slice(&other.completed_cycle_continuations);
        self.output_choice_resolutions
            .extend_from_slice(&other.output_choice_resolutions);
        self.existing_witness_trace
            .extend_from_slice(&other.existing_witness_trace);
        self.superseded_returns
            .extend_from_slice(&other.superseded_returns);
        let ordinal_offset = u64::try_from(self.drive_provenance.len()).unwrap_or(u64::MAX);
        self.drive_provenance
            .extend(other.drive_provenance.iter().cloned().map(|mut evidence| {
                evidence.ordinal = evidence.ordinal.saturating_add(ordinal_offset);
                evidence
            }));
        self.causal_origin_selection
            .extend_from_slice(&other.causal_origin_selection);
        self.candidate_selection
            .extend_from_slice(&other.candidate_selection);
        self.return_origins.extend_from_slice(&other.return_origins);
        self.consequence_writes
            .extend_from_slice(&other.consequence_writes);
        self.closure_eligibility
            .extend_from_slice(&other.closure_eligibility);
        self.physical_incidences
            .extend_from_slice(&other.physical_incidences);
        self.transition_eligibility
            .extend_from_slice(&other.transition_eligibility);
        self.mixed_owner_checks += other.mixed_owner_checks;
        self.mixed_owner_selections += other.mixed_owner_selections;
        self.causal_origin_checks += other.causal_origin_checks;
        self.causal_origin_selections += other.causal_origin_selections;
        self.propagation_budget_exhaustions += other.propagation_budget_exhaustions;
        self.closure_evidence
            .extend_from_slice(&other.closure_evidence);
        self.constructed_learners
            .extend_from_slice(&other.constructed_learners);
        self.owner_events.extend_from_slice(&other.owner_events);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct JointStep {
    index: usize,
    position_before: i16,
    position_after: i16,
    delivered_surface_origins: Vec<u64>,
    direction: i8,
    phase_directions: Vec<i8>,
    actual_position_changes: usize,
    emitted_outputs: Vec<u64>,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    learners: usize,
    junctions: usize,
    links: usize,
    naturally_quiescent: bool,
    comparisons: u64,
    scans: u64,
    work: WorkEvidence,
    execution_cost: ExecutionCostEvidence,
    phase_work: Vec<ReflectedHandPhaseWorkEvidence>,
    events: EventSummary,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct WorkEvidence {
    pub total: u64,
    pub physical_total: u64,
    pub drive_deliveries: u64,
    pub modulatory_deliveries: u64,
    pub local_return_updates: u64,
    pub local_structural_proposals: u64,
    pub physical_deallocations: u64,
    pub junction_deallocations: u64,
    pub local_junction_proposals: u64,
    pub qualified_local_traversals: u64,
    pub causal_closure_observations: u64,
    pub learner_constructions: u64,
}

impl WorkEvidence {
    fn observe(&mut self, work: &Work) {
        self.total = self.total.saturating_add(work.total());
        self.physical_total = self.physical_total.saturating_add(work.physical_total());
        self.drive_deliveries = self.drive_deliveries.saturating_add(work.drive_deliveries);
        self.modulatory_deliveries = self
            .modulatory_deliveries
            .saturating_add(work.modulatory_deliveries);
        self.local_return_updates = self
            .local_return_updates
            .saturating_add(work.local_return_updates);
        self.local_structural_proposals = self
            .local_structural_proposals
            .saturating_add(work.local_structural_proposals);
        self.physical_deallocations = self
            .physical_deallocations
            .saturating_add(work.physical_deallocations);
        self.junction_deallocations = self
            .junction_deallocations
            .saturating_add(work.junction_deallocations);
        self.local_junction_proposals = self
            .local_junction_proposals
            .saturating_add(work.local_junction_proposals);
        self.qualified_local_traversals = self
            .qualified_local_traversals
            .saturating_add(work.qualified_local_traversals);
        self.causal_closure_observations = self
            .causal_closure_observations
            .saturating_add(work.causal_closure_observations);
        self.learner_constructions = self
            .learner_constructions
            .saturating_add(work.learner_constructions);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ExecutionCostEvidence {
    pub queue_ops: u64,
    pub comparisons: u64,
    pub timing_wheel_minimum_key_comparisons: u64,
    pub timing_wheel_bucket_selection_comparisons: u64,
    pub attributed_comparisons: u64,
    pub comparisons_reconciled: bool,
    pub scans: u64,
    pub allocations: u64,
    pub bytes_touched: u64,
    pub peak_memory_bytes: u64,
    pub adjacency_accesses: u64,
    pub frontier_samples: u64,
    pub active_frontier_total: u64,
    pub active_frontier_max: u64,
    pub batches: u64,
    pub batched_items: u64,
    pub batch_max: u64,
    pub batch_histogram: [u64; 7],
    pub batch_fallback_zero_delay: u64,
    pub arena_lookups: u64,
    pub arena_hops: u64,
    pub active_arena_samples: u64,
    pub active_arena_total: u64,
    pub active_arena_max: u64,
    pub local_structural_scans: u64,
}

impl ExecutionCostEvidence {
    fn observe(&mut self, cost: &ExecutionCost) {
        self.queue_ops = self.queue_ops.saturating_add(cost.queue_ops);
        self.comparisons = self.comparisons.saturating_add(cost.comparisons);
        self.timing_wheel_minimum_key_comparisons = self
            .timing_wheel_minimum_key_comparisons
            .saturating_add(cost.timing_wheel_minimum_key_comparisons);
        self.timing_wheel_bucket_selection_comparisons = self
            .timing_wheel_bucket_selection_comparisons
            .saturating_add(cost.timing_wheel_bucket_selection_comparisons);
        self.attributed_comparisons = self
            .timing_wheel_minimum_key_comparisons
            .saturating_add(self.timing_wheel_bucket_selection_comparisons);
        self.comparisons_reconciled = self.attributed_comparisons == self.comparisons;
        self.scans = self.scans.saturating_add(cost.scans);
        self.allocations = self.allocations.saturating_add(cost.allocations);
        self.bytes_touched = self.bytes_touched.saturating_add(cost.bytes_touched);
        self.peak_memory_bytes = self.peak_memory_bytes.max(cost.peak_memory_bytes);
        self.adjacency_accesses = self
            .adjacency_accesses
            .saturating_add(cost.adjacency_accesses);
        self.frontier_samples = self.frontier_samples.saturating_add(cost.frontier_samples);
        self.active_frontier_total = self
            .active_frontier_total
            .saturating_add(cost.active_frontier_total);
        self.active_frontier_max = self.active_frontier_max.max(cost.active_frontier_max);
        self.batches = self.batches.saturating_add(cost.batches);
        self.batched_items = self.batched_items.saturating_add(cost.batched_items);
        self.batch_max = self.batch_max.max(cost.batch_max);
        for (observed, increment) in self.batch_histogram.iter_mut().zip(cost.batch_histogram) {
            *observed = observed.saturating_add(increment);
        }
        self.batch_fallback_zero_delay = self
            .batch_fallback_zero_delay
            .saturating_add(cost.batch_fallback_zero_delay);
        self.arena_lookups = self.arena_lookups.saturating_add(cost.arena_lookups);
        self.arena_hops = self.arena_hops.saturating_add(cost.arena_hops);
        self.active_arena_samples = self
            .active_arena_samples
            .saturating_add(cost.active_arena_samples);
        self.active_arena_total = self
            .active_arena_total
            .saturating_add(cost.active_arena_total);
        self.active_arena_max = self.active_arena_max.max(cost.active_arena_max);
        self.local_structural_scans = self
            .local_structural_scans
            .saturating_add(cost.local_structural_scans);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReflectedHandPhase {
    PendingReturn,
    CurrentInput,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReflectedHandPhaseWorkEvidence {
    pub phase: ReflectedHandPhase,
    pub input_count: usize,
    pub output_count: usize,
    pub emitted_outputs: Vec<u64>,
    pub trace_event_count: usize,
    pub naturally_quiescent: bool,
    pub work: WorkEvidence,
    pub execution_cost: ExecutionCostEvidence,
}

fn phase_work_evidence(
    phase: ReflectedHandPhase,
    input_count: usize,
    run: &Run,
) -> ReflectedHandPhaseWorkEvidence {
    let mut work = WorkEvidence::default();
    work.observe(&run.work);
    let mut execution_cost = ExecutionCostEvidence::default();
    execution_cost.observe(&run.execution_cost);
    ReflectedHandPhaseWorkEvidence {
        phase,
        input_count,
        output_count: run.outputs.len(),
        emitted_outputs: run
            .outputs
            .iter()
            .map(|output| output.from_physical)
            .collect(),
        trace_event_count: run.physical_trace.len(),
        naturally_quiescent: run.naturally_quiescent,
        work,
        execution_cost,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
struct TrialAggregate {
    steps: usize,
    changed_steps: usize,
    actual_position_changes: usize,
    directions: BTreeSet<i8>,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    maximum_same_direction_run: usize,
    final_position: i16,
    learners: usize,
    junctions: usize,
    links: usize,
    comparisons: u64,
    scans: u64,
    delivered_surface_origins: BTreeSet<u64>,
    events: EventSummary,
}

impl TrialAggregate {
    fn from_history(history: &[JointStep]) -> Self {
        let mut aggregate = Self {
            steps: history.len(),
            final_position: history.last().map_or(0, |step| step.position_after),
            learners: history.last().map_or(0, |step| step.learners),
            junctions: history.last().map_or(0, |step| step.junctions),
            links: history.last().map_or(0, |step| step.links),
            ..Self::default()
        };
        let mut previous_direction = 0;
        let mut run = 0;
        for step in history {
            if step.direction != 0 {
                aggregate.changed_steps += 1;
                if previous_direction == step.direction {
                    run += 1;
                } else {
                    previous_direction = step.direction;
                    run = 1;
                }
                aggregate.maximum_same_direction_run =
                    aggregate.maximum_same_direction_run.max(run);
            }
            aggregate.actual_position_changes += step.actual_position_changes;
            aggregate.directions.extend(
                step.phase_directions
                    .iter()
                    .copied()
                    .filter(|direction| *direction != 0),
            );
            aggregate.reached_lower |= step.reached_lower;
            aggregate.reached_upper |= step.reached_upper;
            aggregate.escaped_lower |= step.escaped_lower;
            aggregate.escaped_upper |= step.escaped_upper;
            aggregate.comparisons += step.comparisons;
            aggregate.scans += step.scans;
            aggregate
                .delivered_surface_origins
                .extend(step.delivered_surface_origins.iter().copied());
            aggregate.events.merge(&step.events);
        }
        aggregate
    }

    fn closes_joint(&self) -> bool {
        self.steps == PRIMARY_STEPS
            && self.directions.len() == 2
            && self.reached_lower
            && self.reached_upper
            && self.escaped_lower
            && self.escaped_upper
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct CompletedTrial {
    history: Vec<JointStep>,
    aggregate: TrialAggregate,
    exact_replay: bool,
    naturally_quiescent: bool,
    elapsed_millis: u128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct StoppedTrial {
    completed_prefix: Vec<JointStep>,
    aggregate: TrialAggregate,
    stop: PhysicalStop,
    stopped_step: usize,
    exact_replay: bool,
    naturally_quiescent: bool,
    elapsed_millis: u128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
enum TrialOutcome {
    Completed(CompletedTrial),
    Stopped(StoppedTrial),
}

impl TrialOutcome {
    fn aggregate(&self) -> &TrialAggregate {
        match self {
            Self::Completed(trial) => &trial.aggregate,
            Self::Stopped(trial) => &trial.aggregate,
        }
    }

    fn history(&self) -> &[JointStep] {
        match self {
            Self::Completed(trial) => &trial.history,
            Self::Stopped(trial) => &trial.completed_prefix,
        }
    }

    fn stop(&self) -> Option<PhysicalStop> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(trial) => Some(trial.stop),
        }
    }

    fn exact_replay(&self) -> bool {
        match self {
            Self::Completed(trial) => trial.exact_replay,
            Self::Stopped(trial) => trial.exact_replay,
        }
    }

    fn naturally_quiescent(&self) -> bool {
        match self {
            Self::Completed(trial) => trial.naturally_quiescent,
            Self::Stopped(trial) => trial.naturally_quiescent,
        }
    }
}

struct JointWorld {
    harness: Harness,
    cell: AdmissionCell,
    sensors: Vec<JunctionId>,
    sensor_physical: Vec<u64>,
    motors: [JunctionId; 2],
    motor_physical: [u64; 2],
    outcomes: [JunctionId; 2],
    outcome_physical: [u64; 2],
    position: i16,
    pending: Vec<PendingReturn>,
    sequence: u64,
    max_moments: Option<u64>,
    effect_composition: EffectComposition,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct AppliedEffect {
    direction: i8,
    actual_position_changes: usize,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
}

impl JointWorld {
    fn new(cell: AdmissionCell) -> Self {
        Self::with_capacity(cell, JUNCTION_CAPACITY, LINK_CAPACITY)
    }

    fn with_capacity(cell: AdmissionCell, junction_capacity: u32, link_capacity: u32) -> Self {
        Self::with_capacity_and_protocol(
            cell,
            junction_capacity,
            link_capacity,
            Protocol::RecursiveLearnerConstruction,
        )
    }

    fn with_capacity_and_protocol(
        cell: AdmissionCell,
        junction_capacity: u32,
        link_capacity: u32,
        protocol: Protocol,
    ) -> Self {
        Self::with_capacity_protocol_and_limit(
            cell,
            junction_capacity,
            link_capacity,
            protocol,
            None,
        )
    }

    fn with_capacity_protocol_and_limit(
        cell: AdmissionCell,
        junction_capacity: u32,
        link_capacity: u32,
        protocol: Protocol,
        max_moments: Option<u64>,
    ) -> Self {
        Self::with_capacity_protocol_limit_and_effect_composition(
            cell,
            junction_capacity,
            link_capacity,
            protocol,
            max_moments,
            EffectComposition::Batched,
        )
    }

    fn with_capacity_protocol_limit_and_effect_composition(
        cell: AdmissionCell,
        junction_capacity: u32,
        link_capacity: u32,
        protocol: Protocol,
        max_moments: Option<u64>,
        effect_composition: EffectComposition,
    ) -> Self {
        let mut builder =
            HarnessBuilder::with_capacity(junction_capacity, link_capacity, OUTWARD_REGION);
        builder.set_protocol(protocol);
        builder.set_physical_tracing(true);
        let anchor = add_junction(&mut builder, 90_000, 10_000, 0, 99);
        let sensor_physical = (0..9)
            .map(|channel| 10_000 + channel as u64)
            .collect::<Vec<_>>();
        let sensors = sensor_physical
            .iter()
            .map(|physical| {
                let sensor = add_junction(&mut builder, *physical, 10, 0, 1);
                add_link(&mut builder, anchor, sensor, 0);
                sensor
            })
            .collect::<Vec<_>>();
        let motor_physical = [20_000, 20_001];
        let motors = [
            add_junction(&mut builder, motor_physical[0], 9, 0, 2),
            add_junction(&mut builder, motor_physical[1], 11, 0, 2),
        ];
        let sinks = [
            add_junction(&mut builder, 30_000, 9, OUTWARD_REGION, 1),
            add_junction(&mut builder, 30_001, 11, OUTWARD_REGION, 1),
        ];
        for index in 0..2 {
            add_link(&mut builder, motors[index], sinks[index], 0);
        }
        let outcome_physical = [40_000, 40_001];
        let outcomes = [
            add_junction(&mut builder, outcome_physical[0], 1_000, 0, 1),
            add_junction(&mut builder, outcome_physical[1], 1_001, 0, 1),
        ];
        for outcome in outcomes {
            add_link(&mut builder, anchor, outcome, 0);
        }
        if cell.surface_path == SurfacePath::Symmetric {
            for sensor in &sensors {
                for outcome in outcomes {
                    add_link(&mut builder, *sensor, outcome, 3);
                }
            }
        }
        for index in 0..2 {
            builder.set_outcome_source_for_output(motors[index], outcomes[index]);
        }
        Self {
            harness: builder.build(),
            cell,
            sensors,
            sensor_physical,
            motors,
            motor_physical,
            outcomes,
            outcome_physical,
            position: 0,
            pending: Vec::new(),
            sequence: 0,
            max_moments,
            effect_composition,
        }
    }

    fn checkpoint(&self) -> WorldCheckpoint {
        WorldCheckpoint {
            harness: self.harness.save().expect("joint checkpoint saves"),
            position: self.position,
            pending: self.pending.clone(),
            sequence: self.sequence,
            max_moments: self.max_moments,
            effect_composition: self.effect_composition,
        }
    }

    fn restore(cell: AdmissionCell, checkpoint: WorldCheckpoint) -> Self {
        let mut world = Self::new(cell);
        world.harness = Harness::restore(checkpoint.harness).expect("joint checkpoint restores");
        world.position = checkpoint.position;
        world.pending = checkpoint.pending;
        world.sequence = checkpoint.sequence;
        world.max_moments = checkpoint.max_moments;
        world.effect_composition = checkpoint.effect_composition;
        world
    }

    fn active_channels(&self) -> Vec<usize> {
        active_channels(self.position)
    }

    fn checked_send(&mut self, inputs: &[Input]) -> Result<Run, PhysicalStop> {
        let max_moments = self.max_moments;
        match catch_unwind(AssertUnwindSafe(|| match max_moments {
            Some(limit) => self.harness.send_bounded(inputs, limit),
            None => self.harness.send(inputs),
        })) {
            Ok(run) => Ok(run),
            Err(payload) => match classify_capacity(payload.as_ref()) {
                Some(stop) => Err(stop),
                None => resume_unwind(payload),
            },
        }
    }

    fn checked_send_physical(&mut self, inputs: &[PhysicalInput]) -> Result<Run, PhysicalStop> {
        let max_moments = self.max_moments;
        match catch_unwind(AssertUnwindSafe(|| match max_moments {
            Some(limit) => self.harness.send_physical_bounded(inputs, limit),
            None => self.harness.send_physical(inputs),
        })) {
            Ok(run) => Ok(run),
            Err(payload) => match classify_capacity(payload.as_ref()) {
                Some(stop) => Err(stop),
                None => resume_unwind(payload),
            },
        }
    }

    fn deliver_pending(&mut self) -> Result<(Run, Vec<u64>, usize), PhysicalStop> {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let pending = self.pending.clone();
        let mut surface_origins = Vec::new();
        let inputs = pending
            .into_iter()
            .flat_map(|pending| match pending {
                PendingReturn::Direct { target, origin } => {
                    vec![PhysicalInput {
                        input: physical_input(target, tick, origin),
                        incidence: PhysicalIncidence::Transition,
                    }]
                }
                PendingReturn::Surfaces(channels) => channels
                    .into_iter()
                    .map(|channel| {
                        let origin = self.sensor_physical[channel];
                        surface_origins.push(origin);
                        PhysicalInput {
                            input: physical_input(self.sensors[channel], tick, origin),
                            incidence: PhysicalIncidence::Transition,
                        }
                    })
                    .collect(),
            })
            .collect::<Vec<_>>();
        let input_count = inputs.len();
        let run = self.checked_send_physical(&inputs)?;
        self.pending.clear();
        Ok((run, surface_origins, input_count))
    }

    fn apply_outputs(&mut self, outputs: &[Output]) -> AppliedEffect {
        let mut effort = [0_i32; 2];
        for output in outputs {
            if output.from_physical == self.motor_physical[0] {
                effort[0] = effort[0].saturating_add(output.impulse.abs());
            } else if output.from_physical == self.motor_physical[1] {
                effort[1] = effort[1].saturating_add(output.impulse.abs());
            }
        }
        let direction = match effort[1].cmp(&effort[0]) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };
        let before = self.position;
        let next = before
            .saturating_add(i16::from(direction))
            .clamp(LOWER, UPPER);
        let effect = AppliedEffect {
            direction,
            actual_position_changes: usize::from(next != before),
            reached_lower: next == LOWER && before != LOWER,
            reached_upper: next == UPPER && before != UPPER,
            escaped_lower: before == LOWER && next > LOWER,
            escaped_upper: before == UPPER && next < UPPER,
        };
        self.position = next;
        if next != before {
            match self.cell.return_delivery {
                ReturnDelivery::DirectOutcome => {
                    let index = usize::from(direction > 0);
                    self.pending.push(PendingReturn::Direct {
                        target: self.outcomes[index],
                        origin: self.outcome_physical[index],
                    });
                }
                ReturnDelivery::PhysicalSurface => {
                    self.pending
                        .push(PendingReturn::Surfaces(active_channels(next)));
                }
            }
        }
        effect
    }

    fn step(&mut self) -> Result<JointStep, PhysicalStop> {
        let position_before = self.position;
        let mut quiet = true;
        let mut outputs = Vec::new();
        let mut events = EventSummary::default();
        let mut comparisons = 0;
        let mut scans = 0;
        let mut work = WorkEvidence::default();
        let mut execution_cost = ExecutionCostEvidence::default();
        let mut phase_work = Vec::new();
        let mut delivered_surface_origins = Vec::new();
        let mut phase_effects = Vec::new();

        if !self.pending.is_empty() {
            let (returned, origins, input_count) = self.deliver_pending()?;
            quiet &= returned.naturally_quiescent;
            events.observe(&returned, self.motors);
            comparisons += returned.execution_cost.comparisons;
            scans += returned.execution_cost.scans;
            work.observe(&returned.work);
            execution_cost.observe(&returned.execution_cost);
            phase_work.push(phase_work_evidence(
                ReflectedHandPhase::PendingReturn,
                input_count,
                &returned,
            ));
            if self.effect_composition == EffectComposition::QuiescentPhaseSequential {
                phase_effects.push(self.apply_outputs(&returned.outputs));
            }
            outputs.extend(returned.outputs);
            delivered_surface_origins = origins;
        }

        let tick = self.harness.read().clock.tick.saturating_add(1);
        let mut inputs = self
            .active_channels()
            .into_iter()
            .map(|channel| {
                physical_input(self.sensors[channel], tick, self.sensor_physical[channel])
            })
            .collect::<Vec<_>>();
        for index in 0..2 {
            inputs.push(physical_input(
                self.motors[index],
                tick.saturating_add(2),
                self.outcome_physical[index],
            ));
        }
        let input_count = inputs.len();
        let moved = self.checked_send(&inputs)?;
        quiet &= moved.naturally_quiescent;
        events.observe(&moved, self.motors);
        comparisons += moved.execution_cost.comparisons;
        scans += moved.execution_cost.scans;
        work.observe(&moved.work);
        execution_cost.observe(&moved.execution_cost);
        phase_work.push(phase_work_evidence(
            ReflectedHandPhase::CurrentInput,
            input_count,
            &moved,
        ));
        if self.effect_composition == EffectComposition::QuiescentPhaseSequential {
            phase_effects.push(self.apply_outputs(&moved.outputs));
        }
        outputs.extend(moved.outputs);

        let emitted_outputs = outputs
            .iter()
            .map(|output| output.from_physical)
            .collect::<Vec<_>>();
        if self.effect_composition == EffectComposition::Batched {
            phase_effects.push(self.apply_outputs(&outputs));
        }
        let direction = phase_effects
            .iter()
            .rev()
            .find_map(|effect| (effect.direction != 0).then_some(effect.direction))
            .unwrap_or(0);
        let reached_lower = phase_effects.iter().any(|effect| effect.reached_lower);
        let reached_upper = phase_effects.iter().any(|effect| effect.reached_upper);
        let escaped_lower = phase_effects.iter().any(|effect| effect.escaped_lower);
        let escaped_upper = phase_effects.iter().any(|effect| effect.escaped_upper);
        let actual_position_changes = phase_effects
            .iter()
            .map(|effect| effect.actual_position_changes)
            .sum();
        let observation = self.harness.read();
        let index = usize::try_from(self.sequence).unwrap_or(usize::MAX);
        self.sequence = self.sequence.saturating_add(1);
        Ok(JointStep {
            index,
            position_before,
            position_after: self.position,
            delivered_surface_origins,
            direction,
            phase_directions: phase_effects
                .iter()
                .map(|effect| effect.direction)
                .collect(),
            actual_position_changes,
            emitted_outputs,
            reached_lower,
            reached_upper,
            escaped_lower,
            escaped_upper,
            learners: observation.learners.len(),
            junctions: observation.junctions.len(),
            links: observation.links.len(),
            naturally_quiescent: quiet,
            comparisons,
            scans,
            work,
            execution_cost,
            phase_work,
            events,
        })
    }
}

fn active_channels(position: i16) -> Vec<usize> {
    let mut active = vec![2];
    if position < 0 {
        active.push(0);
    } else if position > 0 {
        active.push(1);
    }
    if position == LOWER {
        active.push(3);
    }
    if position == UPPER {
        active.push(4);
    }
    active
}

fn add_junction(
    builder: &mut HarnessBuilder,
    physical_id: u64,
    position: i32,
    region: i16,
    threshold: i32,
) -> JunctionId {
    builder.add_junction(Junction {
        physical_id,
        position,
        region,
        threshold,
        resistance: u32::MAX,
    })
}

fn add_link(builder: &mut HarnessBuilder, from: JunctionId, to: JunctionId, delay: i64) {
    builder.add_link(Link {
        from,
        to,
        delay,
        phase: 0,
        coupling: 1,
        resistance: u32::MAX,
        mode: TransmissionMode::Drive,
    });
}

fn physical_input(target: JunctionId, tick: i64, origin_physical: u64) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical,
        target,
        impulse: 1,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Execution {
    history: Vec<JointStep>,
    stop: Option<PhysicalStop>,
    stopped_step: usize,
    canonical_bytes: Vec<u8>,
}

fn execute(world: &mut JointWorld, steps: usize) -> Execution {
    let mut history = Vec::new();
    for step in 0..steps {
        match world.step() {
            Ok(observed) => history.push(observed),
            Err(stop) => {
                return Execution {
                    history,
                    stop: Some(stop),
                    stopped_step: step,
                    canonical_bytes: world
                        .harness
                        .save()
                        .and_then(|checkpoint| checkpoint.canonical_bytes())
                        .expect("stopped checkpoint encodes"),
                };
            }
        }
    }
    Execution {
        history,
        stop: None,
        stopped_step: steps,
        canonical_bytes: world
            .harness
            .save()
            .and_then(|checkpoint| checkpoint.canonical_bytes())
            .expect("completed checkpoint encodes"),
    }
}

fn trial_with_capacity(
    cell: AdmissionCell,
    junction_capacity: u32,
    link_capacity: u32,
) -> TrialOutcome {
    trial_with_capacity_and_protocol(
        cell,
        junction_capacity,
        link_capacity,
        Protocol::RecursiveLearnerConstruction,
    )
}

fn trial_with_capacity_and_protocol(
    cell: AdmissionCell,
    junction_capacity: u32,
    link_capacity: u32,
    protocol: Protocol,
) -> TrialOutcome {
    trial_with_capacity_protocol_and_limit(cell, junction_capacity, link_capacity, protocol, None)
}

fn trial_with_capacity_protocol_and_limit(
    cell: AdmissionCell,
    junction_capacity: u32,
    link_capacity: u32,
    protocol: Protocol,
    max_moments: Option<u64>,
) -> TrialOutcome {
    trial_with_capacity_protocol_limit_and_effect_composition(
        cell,
        junction_capacity,
        link_capacity,
        protocol,
        max_moments,
        EffectComposition::Batched,
    )
}

fn trial_with_capacity_protocol_limit_and_effect_composition(
    cell: AdmissionCell,
    junction_capacity: u32,
    link_capacity: u32,
    protocol: Protocol,
    max_moments: Option<u64>,
    effect_composition: EffectComposition,
) -> TrialOutcome {
    let started = Instant::now();
    let mut direct = JointWorld::with_capacity_protocol_limit_and_effect_composition(
        cell,
        junction_capacity,
        link_capacity,
        protocol,
        max_moments,
        effect_composition,
    );
    let checkpoint = direct.checkpoint();
    let direct_execution = execute(&mut direct, PRIMARY_STEPS);
    let mut replay = JointWorld::restore(cell, checkpoint);
    let replay_execution = execute(&mut replay, PRIMARY_STEPS);
    let elapsed = started.elapsed();
    let exact_replay = direct_execution == replay_execution;
    let naturally_quiescent = direct_execution
        .history
        .iter()
        .all(|step| step.naturally_quiescent);
    let aggregate = TrialAggregate::from_history(&direct_execution.history);
    if elapsed >= WARM_LIMIT {
        return TrialOutcome::Stopped(StoppedTrial {
            completed_prefix: direct_execution.history,
            aggregate,
            stop: PhysicalStop::WarmRuntime,
            stopped_step: direct_execution.stopped_step,
            exact_replay,
            naturally_quiescent,
            elapsed_millis: elapsed.as_millis(),
        });
    }
    match direct_execution.stop {
        Some(stop) => TrialOutcome::Stopped(StoppedTrial {
            completed_prefix: direct_execution.history,
            aggregate,
            stop,
            stopped_step: direct_execution.stopped_step,
            exact_replay,
            naturally_quiescent,
            elapsed_millis: elapsed.as_millis(),
        }),
        None => TrialOutcome::Completed(CompletedTrial {
            history: direct_execution.history,
            aggregate,
            exact_replay,
            naturally_quiescent,
            elapsed_millis: elapsed.as_millis(),
        }),
    }
}

fn trial(cell: AdmissionCell) -> TrialOutcome {
    trial_with_capacity(cell, JUNCTION_CAPACITY, LINK_CAPACITY)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct PerturbationTrial {
    imposed_position: i16,
    left_upper: bool,
    reached_lower: bool,
    left_lower: bool,
    both_signs: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    stop: Option<PhysicalStop>,
}

fn execute_perturbation(imposed: i16) -> PerturbationTrial {
    execute_perturbation_with_protocol(imposed, Protocol::RecursiveLearnerConstruction)
}

fn execute_perturbation_with_protocol(imposed: i16, protocol: Protocol) -> PerturbationTrial {
    execute_perturbation_with_protocol_and_capacity(
        imposed,
        protocol,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
    )
}

fn execute_perturbation_with_protocol_and_capacity(
    imposed: i16,
    protocol: Protocol,
    junction_capacity: u32,
    link_capacity: u32,
) -> PerturbationTrial {
    execute_perturbation_with_protocol_capacity_and_effect_composition(
        imposed,
        protocol,
        junction_capacity,
        link_capacity,
        EffectComposition::Batched,
    )
}

fn execute_perturbation_with_protocol_capacity_and_effect_composition(
    imposed: i16,
    protocol: Protocol,
    junction_capacity: u32,
    link_capacity: u32,
    effect_composition: EffectComposition,
) -> PerturbationTrial {
    fn once_with_protocol(
        imposed: i16,
        protocol: Protocol,
        junction_capacity: u32,
        link_capacity: u32,
        effect_composition: EffectComposition,
    ) -> (Execution, Execution) {
        let mut world = JointWorld::with_capacity_protocol_limit_and_effect_composition(
            AdmissionCell::COMPLETE,
            junction_capacity,
            link_capacity,
            protocol,
            None,
            effect_composition,
        );
        let development = execute(&mut world, DEVELOPMENT_STEPS);
        if development.stop.is_none() {
            world.position = imposed;
            world.pending.clear();
        }
        let recovery = if development.stop.is_none() {
            execute(&mut world, RECOVERY_STEPS)
        } else {
            Execution {
                history: Vec::new(),
                stop: development.stop,
                stopped_step: 0,
                canonical_bytes: development.canonical_bytes.clone(),
            }
        };
        (development, recovery)
    }
    let started = Instant::now();
    let run = |imposed| {
        once_with_protocol(
            imposed,
            protocol,
            junction_capacity,
            link_capacity,
            effect_composition,
        )
    };
    let (development, recovery) = run(imposed);
    let (replayed_development, replayed_recovery) = run(imposed);
    let aggregate = TrialAggregate::from_history(&recovery.history);
    let stop = if started.elapsed() >= WARM_LIMIT {
        Some(PhysicalStop::WarmRuntime)
    } else {
        development.stop.or(recovery.stop)
    };
    PerturbationTrial {
        imposed_position: imposed,
        left_upper: recovery
            .history
            .iter()
            .any(|step| step.position_before == UPPER && step.position_after < UPPER),
        reached_lower: aggregate.reached_lower,
        left_lower: aggregate.escaped_lower,
        both_signs: aggregate.directions.len() == 2,
        exact_replay: development == replayed_development && recovery == replayed_recovery,
        naturally_quiescent: development
            .history
            .iter()
            .chain(&recovery.history)
            .all(|step| step.naturally_quiescent),
        stop,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReflectedHandProtocolEvidence {
    pub protocol: Protocol,
    pub effect_composition: EffectComposition,
    pub steps: usize,
    pub changed_steps: usize,
    pub actual_position_changes: usize,
    pub comparisons: u64,
    pub scans: u64,
    pub directions: BTreeSet<i8>,
    pub reached_lower: bool,
    pub reached_upper: bool,
    pub escaped_lower: bool,
    pub escaped_upper: bool,
    pub final_position: i16,
    pub learners: usize,
    pub closure_observations: u64,
    pub constructions: u64,
    pub primary_closed: bool,
    pub perturbation_recovered: bool,
    pub stopped: bool,
    pub exact_replay: bool,
    pub naturally_quiescent: bool,
    pub trajectory: Vec<ReflectedHandStepEvidence>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReflectedHandStepEvidence {
    pub index: usize,
    pub position_before: i16,
    pub position_after: i16,
    pub direction: i8,
    pub phase_directions: Vec<i8>,
    pub actual_position_changes: usize,
    pub comparisons: u64,
    pub scans: u64,
    pub work: WorkEvidence,
    pub execution_cost: ExecutionCostEvidence,
    pub phase_work: Vec<ReflectedHandPhaseWorkEvidence>,
    pub emitted_outputs: Vec<u64>,
    pub delivered_surface_count: usize,
    pub learners: usize,
    pub junctions: usize,
    pub links: usize,
    pub return_scheduling: u64,
    pub return_admissions: u64,
    pub rejected_returns: u64,
    pub reverse_consolidations: u64,
    pub closure_observations: u64,
    pub constructions: u64,
    pub boundary_novelty_checks: u64,
    pub boundary_novelty_rejections: u64,
    pub owner_writes: u64,
    pub owner_reads: u64,
    pub consequential_owner_reads: u64,
    pub surface_paths: Vec<SurfacePathEvidence>,
    pub output_candidates: Vec<OutputCandidateEvidence>,
    pub fresh_opportunities: Vec<FreshOpportunityEvidence>,
    pub fresh_opportunity_evaluations: Vec<FreshOpportunityEvaluationEvidence>,
    pub physical_transition_continuations: Vec<PhysicalTransitionContinuationEvidence>,
    pub coherent_effects: Vec<CoherentEffectEvidence>,
    pub completed_cycle_continuations: Vec<CompletedCycleContinuationEvidence>,
    pub output_choice_resolutions: Vec<OutputChoiceResolutionEvidence>,
    pub existing_witness_trace: Vec<ExistingWitnessTraceEntry>,
    pub superseded_returns: Vec<LinkId>,
    pub drive_provenance: Vec<DriveProvenanceEvidence>,
    pub causal_origin_selection: Vec<CausalOriginSelectionEvidence>,
    pub candidate_selection: Vec<CandidateSelectionEvidence>,
    pub return_origins: Vec<ReturnOriginEvidence>,
    pub consequence_writes: Vec<ConsequenceWriteEvidence>,
    pub closure_eligibility: Vec<ClosureEligibilityEvidence>,
    pub physical_incidences: Vec<PhysicalIncidenceEvidence>,
    pub transition_eligibility: Vec<PhysicalTransitionEligibilityEvidence>,
    pub mixed_owner_checks: u64,
    pub mixed_owner_selections: u64,
    pub causal_origin_checks: u64,
    pub causal_origin_selections: u64,
    pub propagation_budget_exhaustions: u64,
}

pub fn run_reflected_hand_with_protocol(protocol: Protocol) -> ReflectedHandProtocolEvidence {
    run_reflected_hand_with_protocol_and_capacity(protocol, JUNCTION_CAPACITY, LINK_CAPACITY)
}

pub fn run_reflected_hand_with_protocol_and_capacity(
    protocol: Protocol,
    junction_capacity: u32,
    link_capacity: u32,
) -> ReflectedHandProtocolEvidence {
    run_reflected_hand_with_protocol_capacity_and_limit(
        protocol,
        junction_capacity,
        link_capacity,
        None,
        EffectComposition::Batched,
    )
}

pub fn run_reflected_hand_bounded(
    protocol: Protocol,
    junction_capacity: u32,
    link_capacity: u32,
    max_moments_per_send: u64,
) -> ReflectedHandProtocolEvidence {
    run_reflected_hand_bounded_with_effect_composition(
        protocol,
        junction_capacity,
        link_capacity,
        max_moments_per_send,
        EffectComposition::Batched,
    )
}

pub fn run_reflected_hand_bounded_with_effect_composition(
    protocol: Protocol,
    junction_capacity: u32,
    link_capacity: u32,
    max_moments_per_send: u64,
    effect_composition: EffectComposition,
) -> ReflectedHandProtocolEvidence {
    run_reflected_hand_with_protocol_capacity_and_limit(
        protocol,
        junction_capacity,
        link_capacity,
        Some(max_moments_per_send),
        effect_composition,
    )
}

fn run_reflected_hand_with_protocol_capacity_and_limit(
    protocol: Protocol,
    junction_capacity: u32,
    link_capacity: u32,
    max_moments: Option<u64>,
    effect_composition: EffectComposition,
) -> ReflectedHandProtocolEvidence {
    let primary = trial_with_capacity_protocol_limit_and_effect_composition(
        AdmissionCell::COMPLETE,
        junction_capacity,
        link_capacity,
        protocol,
        max_moments,
        effect_composition,
    );
    let trajectory = primary
        .history()
        .iter()
        .map(|step| ReflectedHandStepEvidence {
            index: step.index,
            position_before: step.position_before,
            position_after: step.position_after,
            direction: step.direction,
            phase_directions: step.phase_directions.clone(),
            actual_position_changes: step.actual_position_changes,
            comparisons: step.comparisons,
            scans: step.scans,
            work: step.work,
            execution_cost: step.execution_cost.clone(),
            phase_work: step.phase_work.clone(),
            emitted_outputs: step.emitted_outputs.clone(),
            delivered_surface_count: step.delivered_surface_origins.len(),
            learners: step.learners,
            junctions: step.junctions,
            links: step.links,
            return_scheduling: step.events.return_scheduling,
            return_admissions: step.events.return_admissions,
            rejected_returns: step.events.rejected_returns,
            reverse_consolidations: step.events.reverse_consolidations,
            closure_observations: step.events.closure_observations,
            constructions: step.events.constructions,
            boundary_novelty_checks: step.events.boundary_novelty_checks,
            boundary_novelty_rejections: step.events.boundary_novelty_rejections,
            owner_writes: step.events.owner_writes,
            owner_reads: step.events.owner_reads,
            consequential_owner_reads: step.events.consequential_owner_reads,
            surface_paths: step.events.surface_paths.clone(),
            output_candidates: step.events.output_candidates.clone(),
            fresh_opportunities: step.events.fresh_opportunities.clone(),
            fresh_opportunity_evaluations: step.events.fresh_opportunity_evaluations.clone(),
            physical_transition_continuations: step
                .events
                .physical_transition_continuations
                .clone(),
            coherent_effects: step.events.coherent_effects.clone(),
            completed_cycle_continuations: step.events.completed_cycle_continuations.clone(),
            output_choice_resolutions: step.events.output_choice_resolutions.clone(),
            existing_witness_trace: step.events.existing_witness_trace.clone(),
            superseded_returns: step.events.superseded_returns.clone(),
            drive_provenance: step.events.drive_provenance.clone(),
            causal_origin_selection: step.events.causal_origin_selection.clone(),
            candidate_selection: step.events.candidate_selection.clone(),
            return_origins: step.events.return_origins.clone(),
            consequence_writes: step.events.consequence_writes.clone(),
            closure_eligibility: step.events.closure_eligibility.clone(),
            physical_incidences: step.events.physical_incidences.clone(),
            transition_eligibility: step.events.transition_eligibility.clone(),
            mixed_owner_checks: step.events.mixed_owner_checks,
            mixed_owner_selections: step.events.mixed_owner_selections,
            causal_origin_checks: step.events.causal_origin_checks,
            causal_origin_selections: step.events.causal_origin_selections,
            propagation_budget_exhaustions: step.events.propagation_budget_exhaustions,
        })
        .collect();
    let aggregate = primary.aggregate();
    let primary_closed = aggregate.closes_joint();
    let perturbation = primary_closed.then(|| {
        execute_perturbation_with_protocol_capacity_and_effect_composition(
            UPPER,
            protocol,
            junction_capacity,
            link_capacity,
            effect_composition,
        )
    });
    ReflectedHandProtocolEvidence {
        protocol,
        effect_composition,
        steps: aggregate.steps,
        changed_steps: aggregate.changed_steps,
        actual_position_changes: aggregate.actual_position_changes,
        comparisons: aggregate.comparisons,
        scans: aggregate.scans,
        directions: aggregate.directions.clone(),
        reached_lower: aggregate.reached_lower,
        reached_upper: aggregate.reached_upper,
        escaped_lower: aggregate.escaped_lower,
        escaped_upper: aggregate.escaped_upper,
        final_position: aggregate.final_position,
        learners: aggregate.learners,
        closure_observations: aggregate.events.closure_observations,
        constructions: aggregate.events.constructions,
        primary_closed,
        perturbation_recovered: perturbation
            .as_ref()
            .is_some_and(PerturbationTrial::recovered),
        stopped: primary.stop().is_some()
            || perturbation.as_ref().is_some_and(|run| run.stop.is_some()),
        exact_replay: primary.exact_replay()
            && perturbation.as_ref().is_none_or(|run| run.exact_replay),
        naturally_quiescent: primary.naturally_quiescent()
            && perturbation
                .as_ref()
                .is_none_or(|run| run.naturally_quiescent),
        trajectory,
    }
}

impl PerturbationTrial {
    fn recovered(&self) -> bool {
        self.stop.is_none()
            && self.left_upper
            && self.reached_lower
            && self.left_lower
            && self.both_signs
            && self.exact_replay
            && self.naturally_quiescent
    }
}

#[derive(Clone, Debug, Serialize)]
struct IntegrityEvidence {
    return_reentry_outcome: &'static str,
    return_reentry_replay: bool,
    return_reentry_quiescent: bool,
    frozen_reference_exact: bool,
    survived: bool,
}

fn frozen_reference_matches(outcome: &TrialOutcome) -> bool {
    let aggregate = outcome.aggregate();
    outcome.stop().is_none()
        && aggregate.steps == 16
        && aggregate.changed_steps == 15
        && aggregate.directions == BTreeSet::from([-1, 1])
        && !aggregate.reached_lower
        && aggregate.reached_upper
        && !aggregate.escaped_lower
        && aggregate.escaped_upper
        && aggregate.final_position == 3
        && aggregate.learners == 0
        && aggregate.events.closure_observations == 0
        && aggregate.events.constructions == 0
        && outcome.exact_replay()
        && outcome.naturally_quiescent()
}

fn integrity(frozen: &TrialOutcome) -> IntegrityEvidence {
    use recursive_learner_fresh_memory::{Arm as ParentArm, run as parent_run};
    let parent = parent_run(ParentArm::ReturnReentryComposition);
    let frozen_reference_exact = frozen_reference_matches(frozen);
    let survived = parent.outcome == "survived"
        && parent.exact_replay
        && parent.naturally_quiescent
        && frozen_reference_exact;
    IntegrityEvidence {
        return_reentry_outcome: parent.outcome,
        return_reentry_replay: parent.exact_replay,
        return_reentry_quiescent: parent.naturally_quiescent,
        frozen_reference_exact,
        survived,
    }
}

fn construction_survived(outcome: &TrialOutcome) -> bool {
    let events = &outcome.aggregate().events;
    outcome.stop().is_none()
        && events.closure_observations >= 2
        && events.constructions >= 1
        && events
            .closure_evidence
            .iter()
            .any(|evidence| *evidence >= 2)
        && outcome.exact_replay()
        && outcome.naturally_quiescent()
}

#[derive(Clone, Debug, Serialize)]
struct FreshnessEvidence {
    child: Option<LearnerId>,
    first_owner_return_admitted: bool,
    consequential_read_before_own_write: bool,
    own_write_observed: bool,
    survived: bool,
}

fn freshness(outcome: &TrialOutcome) -> FreshnessEvidence {
    let events = &outcome.aggregate().events;
    let child = events.constructed_learners.first().copied();
    let Some(child) = child else {
        return FreshnessEvidence {
            child: None,
            first_owner_return_admitted: false,
            consequential_read_before_own_write: false,
            own_write_observed: false,
            survived: false,
        };
    };
    let first_owner_return_admitted = events.owner_events.iter().find_map(|event| match event {
        OwnerEvent::Admission { owner, admitted } if *owner == child => Some(*admitted),
        _ => None,
    }) == Some(true);
    let write_index = events
        .owner_events
        .iter()
        .position(|event| matches!(event, OwnerEvent::Write { owner } if *owner == child));
    let consequential_read_before_own_write =
        events
            .owner_events
            .iter()
            .enumerate()
            .any(|(index, event)| {
                matches!(
                    event,
                    OwnerEvent::Read {
                        owner,
                        consequential: true,
                    } if *owner == child && write_index.is_none_or(|write| index < write)
                )
            });
    let survived = first_owner_return_admitted && !consequential_read_before_own_write;
    FreshnessEvidence {
        child: Some(child),
        first_owner_return_admitted,
        consequential_read_before_own_write,
        own_write_observed: write_index.is_some(),
        survived,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum TransitionStage {
    SurfaceIncidence,
    OutcomeTraversal,
    ReturnScheduling,
    OriginAdmission,
    ReverseConsolidation,
    CausalClosure,
    Construction,
    FreshOwnerAdmission,
    PrivateWrite,
    PrivateRead,
    Continuation,
    Reversal,
    Release,
    PhysicalCost,
    Complete,
}

fn incomplete_transition(outcome: &TrialOutcome) -> (TransitionStage, &'static str) {
    if outcome.stop().is_some() {
        return (
            TransitionStage::PhysicalCost,
            "the complete cell hit a fixed physical stop",
        );
    }
    let aggregate = outcome.aggregate();
    let events = &aggregate.events;
    if aggregate.delivered_surface_origins.is_empty() {
        (
            TransitionStage::SurfaceIncidence,
            "no post-movement surface was delivered",
        )
    } else if events.drive_incidence == 0 {
        (
            TransitionStage::OutcomeTraversal,
            "surface incidence produced no drive traversal",
        )
    } else if events.return_scheduling == 0 {
        (
            TransitionStage::ReturnScheduling,
            "no emitted action scheduled a live return",
        )
    } else if events.return_admissions == 0 {
        (
            TransitionStage::OriginAdmission,
            "no physical return origin was admitted",
        )
    } else if events.reverse_consolidations == 0 {
        (
            TransitionStage::ReverseConsolidation,
            "admission produced no reverse consolidation",
        )
    } else if events.closure_observations == 0 {
        (
            TransitionStage::CausalClosure,
            "reverse consolidation produced no closure",
        )
    } else if events.constructions == 0 {
        (
            TransitionStage::Construction,
            "closure evidence did not construct a child",
        )
    } else {
        let fresh = freshness(outcome);
        if !fresh.first_owner_return_admitted {
            (
                TransitionStage::FreshOwnerAdmission,
                "the child admitted no fresh own return",
            )
        } else if !fresh.own_write_observed {
            (
                TransitionStage::PrivateWrite,
                "the child received no private consequence write",
            )
        } else if events.consequential_owner_reads == 0 {
            (
                TransitionStage::PrivateRead,
                "later choice did not read private consequence",
            )
        } else if aggregate.maximum_same_direction_run < 4 {
            (
                TransitionStage::Continuation,
                "movement did not sustain boundary travel",
            )
        } else if aggregate.directions.len() < 2 {
            (
                TransitionStage::Reversal,
                "the opposite sign did not emerge",
            )
        } else if (aggregate.reached_lower && !aggregate.escaped_lower)
            || (aggregate.reached_upper && !aggregate.escaped_upper)
        {
            (TransitionStage::Release, "a reached boundary was not left")
        } else {
            (
                TransitionStage::Complete,
                "the declared construction chain completed",
            )
        }
    }
}

fn first_difference(left: &TrialOutcome, right: &TrialOutcome) -> Option<usize> {
    left.history()
        .iter()
        .zip(right.history())
        .position(|(left, right)| {
            left.position_after != right.position_after
                || left.direction != right.direction
                || left.emitted_outputs != right.emitted_outputs
                || left.events != right.events
        })
        .or_else(|| {
            (left.stop() != right.stop()).then_some(left.history().len().min(right.history().len()))
        })
}

fn differs(left: &TrialOutcome, right: &TrialOutcome) -> bool {
    first_difference(left, right).is_some()
        || left.aggregate().final_position != right.aggregate().final_position
        || left.aggregate().learners != right.aggregate().learners
}

#[derive(Clone, Debug, Serialize)]
struct Localization {
    path_effect_direct: bool,
    path_effect_physical: bool,
    delivery_effect_absent: bool,
    delivery_effect_symmetric: bool,
    interaction_observed: bool,
    first_path_difference_direct: Option<usize>,
    first_delivery_difference_symmetric: Option<usize>,
    earliest_incomplete_transition: TransitionStage,
    explanation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct AdmissionEvidence {
    integrity: IntegrityEvidence,
    frozen: TrialOutcome,
    path_only: TrialOutcome,
    delivery_only: TrialOutcome,
    complete: TrialOutcome,
    freshness: FreshnessEvidence,
    perturbation: Option<PerturbationTrial>,
    localization: Localization,
}

fn measure() -> AdmissionEvidence {
    let complete = trial(AdmissionCell::COMPLETE);
    let (frozen, path_only, delivery_only) = std::thread::scope(|scope| {
        let frozen = scope.spawn(|| trial(AdmissionCell::FROZEN));
        let path_only = scope.spawn(|| trial(AdmissionCell::PATH_ONLY));
        let delivery_only = scope.spawn(|| trial(AdmissionCell::DELIVERY_ONLY));
        (
            frozen.join().expect("frozen cell completes"),
            path_only.join().expect("path-only cell completes"),
            delivery_only.join().expect("delivery-only cell completes"),
        )
    });
    let integrity = integrity(&frozen);
    let freshness = freshness(&complete);
    let perturbation = (construction_survived(&complete)
        && freshness.survived
        && complete.aggregate().closes_joint())
    .then(|| execute_perturbation(UPPER));
    let path_effect_direct = differs(&frozen, &path_only);
    let path_effect_physical = differs(&delivery_only, &complete);
    let delivery_effect_absent = differs(&frozen, &delivery_only);
    let delivery_effect_symmetric = differs(&path_only, &complete);
    let (earliest_incomplete_transition, explanation) = incomplete_transition(&complete);
    let localization = Localization {
        path_effect_direct,
        path_effect_physical,
        delivery_effect_absent,
        delivery_effect_symmetric,
        interaction_observed: path_effect_direct != path_effect_physical
            || delivery_effect_absent != delivery_effect_symmetric,
        first_path_difference_direct: first_difference(&frozen, &path_only),
        first_delivery_difference_symmetric: first_difference(&path_only, &complete),
        earliest_incomplete_transition,
        explanation,
    };
    AdmissionEvidence {
        integrity,
        frozen,
        path_only,
        delivery_only,
        complete,
        freshness,
        perturbation,
        localization,
    }
}

static EVIDENCE: OnceLock<AdmissionEvidence> = OnceLock::new();

fn evidence() -> &'static AdmissionEvidence {
    EVIDENCE.get_or_init(measure)
}

fn integrity_result(evidence: &AdmissionEvidence) -> ProbeResult {
    result(
        Arm::InheritedIntegrityControl,
        if evidence.integrity.survived {
            "survived"
        } else {
            "falsified"
        },
        serde_json::to_value(&evidence.integrity).expect("integrity serializes"),
        (!evidence.integrity.survived).then(|| "an imported parent reference changed".to_string()),
        evidence.integrity.survived,
        evidence.integrity.survived,
    )
}

fn cell_result(
    arm: Arm,
    evidence: &AdmissionEvidence,
    outcome: &TrialOutcome,
    extra_predicate: bool,
    falsifier: &'static str,
) -> ProbeResult {
    if !evidence.integrity.survived {
        return result(
            arm,
            "inconclusive",
            serde_json::json!({"integrity": "failed"}),
            Some("the integrity prerequisite failed".to_string()),
            false,
            false,
        );
    }
    let survived = outcome.stop().is_none()
        && outcome.exact_replay()
        && outcome.naturally_quiescent()
        && extra_predicate;
    result(
        arm,
        if survived { "survived" } else { "falsified" },
        serde_json::to_value(outcome).expect("cell serializes"),
        (!survived).then(|| falsifier.to_string()),
        outcome.exact_replay(),
        outcome.naturally_quiescent(),
    )
}

fn construction_result(evidence: &AdmissionEvidence) -> ProbeResult {
    let survived = evidence.integrity.survived && construction_survived(&evidence.complete);
    result(
        Arm::TruthfulHandConstructionAdmission,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "cell": AdmissionCell::COMPLETE,
            "trial": evidence.complete,
            "construction_admitted": construction_survived(&evidence.complete),
        }),
        (!survived).then(|| {
            "the complete physical-reentry cell did not construct a bounded causal child"
                .to_string()
        }),
        evidence.complete.exact_replay(),
        evidence.complete.naturally_quiescent(),
    )
}

fn freshness_result(evidence: &AdmissionEvidence) -> ProbeResult {
    if !construction_survived(&evidence.complete) {
        return result(
            Arm::ChildFreshnessBoundary,
            "inconclusive",
            serde_json::json!({"construction": "failed", "freshness": evidence.freshness}),
            Some("child freshness was not run because bounded construction failed".to_string()),
            evidence.complete.exact_replay(),
            evidence.complete.naturally_quiescent(),
        );
    }
    result(
        Arm::ChildFreshnessBoundary,
        if evidence.freshness.survived {
            "survived"
        } else {
            "falsified"
        },
        serde_json::to_value(&evidence.freshness).expect("freshness serializes"),
        (!evidence.freshness.survived).then(|| {
            "the constructed child did not expose a fresh owner-local boundary".to_string()
        }),
        evidence.complete.exact_replay(),
        evidence.complete.naturally_quiescent(),
    )
}

fn joint_result(evidence: &AdmissionEvidence) -> ProbeResult {
    if !construction_survived(&evidence.complete) || !evidence.freshness.survived {
        return result(
            Arm::ReflectedJointRetry,
            "inconclusive",
            serde_json::json!({
                "construction": construction_survived(&evidence.complete),
                "freshness": evidence.freshness.survived,
                "perturbation": null,
            }),
            Some("the joint retry was gated by construction or freshness failure".to_string()),
            evidence.complete.exact_replay(),
            evidence.complete.naturally_quiescent(),
        );
    }
    let primary_closed = evidence.complete.aggregate().closes_joint();
    let recovered = evidence
        .perturbation
        .as_ref()
        .is_some_and(PerturbationTrial::recovered);
    let survived = primary_closed && recovered;
    result(
        Arm::ReflectedJointRetry,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "primary": evidence.complete,
            "primary_closed": primary_closed,
            "perturbation": evidence.perturbation,
            "perturbation_recovered": recovered,
        }),
        (!survived).then(|| {
            if !primary_closed {
                "the constructed hand did not reach and leave both limits in sixteen steps"
            } else {
                "the primary survivor did not recover from the fixed perturbation"
            }
            .to_string()
        }),
        evidence.complete.exact_replay()
            && evidence
                .perturbation
                .as_ref()
                .is_none_or(|trial| trial.exact_replay),
        evidence.complete.naturally_quiescent()
            && evidence
                .perturbation
                .as_ref()
                .is_none_or(|trial| trial.naturally_quiescent),
    )
}

fn localization_result(evidence: &AdmissionEvidence) -> ProbeResult {
    let replay = [
        &evidence.frozen,
        &evidence.path_only,
        &evidence.delivery_only,
        &evidence.complete,
    ]
    .into_iter()
    .all(TrialOutcome::exact_replay);
    let quiet = [
        &evidence.frozen,
        &evidence.path_only,
        &evidence.delivery_only,
        &evidence.complete,
    ]
    .into_iter()
    .all(TrialOutcome::naturally_quiescent);
    let survived = evidence.integrity.survived && replay;
    result(
        Arm::SurfacePathDeliveryFactorialLocalization,
        if survived { "survived" } else { "inconclusive" },
        serde_json::json!({
            "localization": evidence.localization,
            "frozen": evidence.frozen.aggregate(),
            "path_only": evidence.path_only.aggregate(),
            "delivery_only": evidence.delivery_only.aggregate(),
            "complete": evidence.complete.aggregate(),
            "stops": {
                "frozen": evidence.frozen.stop(),
                "path_only": evidence.path_only.stop(),
                "delivery_only": evidence.delivery_only.stop(),
                "complete": evidence.complete.stop(),
            },
        }),
        (!survived).then(|| "the factorial lineage or replay was not interpretable".to_string()),
        replay,
        quiet,
    )
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    match arm {
        Arm::InheritedIntegrityControl => integrity_result(evidence),
        Arm::FrozenTruthfulRecursiveReference => cell_result(
            arm,
            evidence,
            &evidence.frozen,
            frozen_reference_matches(&evidence.frozen),
            "the frozen truthful-recursive hand trace changed",
        ),
        Arm::SymmetricSurfacePathOnly => cell_result(
            arm,
            evidence,
            &evidence.path_only,
            true,
            "the symmetric-path-only control hit a physical or software predicate",
        ),
        Arm::PhysicalSurfaceDeliveryOnly => cell_result(
            arm,
            evidence,
            &evidence.delivery_only,
            evidence.delivery_only.aggregate().events.constructions == 0,
            "physical delivery without a surface path constructed falsely or hit a stop",
        ),
        Arm::TruthfulHandConstructionAdmission => construction_result(evidence),
        Arm::ChildFreshnessBoundary => freshness_result(evidence),
        Arm::ReflectedJointRetry => joint_result(evidence),
        Arm::SurfacePathDeliveryFactorialLocalization => localization_result(evidence),
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admission_cells_change_exactly_one_axis() {
        assert_eq!(
            AdmissionCell::FROZEN.surface_path,
            AdmissionCell::DELIVERY_ONLY.surface_path
        );
        assert_ne!(
            AdmissionCell::FROZEN.return_delivery,
            AdmissionCell::DELIVERY_ONLY.return_delivery
        );
        assert_eq!(
            AdmissionCell::FROZEN.return_delivery,
            AdmissionCell::PATH_ONLY.return_delivery
        );
        assert_ne!(
            AdmissionCell::FROZEN.surface_path,
            AdmissionCell::PATH_ONLY.surface_path
        );
    }

    #[test]
    fn frozen_truthful_recursive_reference_is_exact() {
        let observed = run(Arm::FrozenTruthfulRecursiveReference);
        assert_eq!(observed.outcome, "survived", "{observed:#?}");
    }

    #[test]
    fn symmetric_surface_paths_are_direction_blind() {
        let world = JointWorld::new(AdmissionCell::PATH_ONLY);
        let observation = world.harness.read();
        for sensor in &world.sensors {
            let targets = observation
                .links
                .iter()
                .filter(|link| link.from == *sensor && world.outcomes.contains(&link.to))
                .map(|link| (link.to, link.delay, link.coupling, link.resistance))
                .collect::<BTreeSet<_>>();
            assert_eq!(targets.len(), 2);
            assert!(world.outcomes.iter().all(|outcome| {
                targets.iter().any(|(target, delay, coupling, resistance)| {
                    target == outcome && *delay == 3 && *coupling == 1 && *resistance == u32::MAX
                })
            }));
        }
    }

    #[test]
    fn physical_delivery_uses_only_active_surface_origins() {
        let measured = evidence();
        for step in measured.complete.history() {
            let expected = active_channels(step.position_before)
                .into_iter()
                .map(|channel| 10_000 + channel as u64)
                .collect::<BTreeSet<_>>();
            assert!(
                step.delivered_surface_origins
                    .iter()
                    .all(|origin| expected.contains(origin))
            );
        }
    }

    #[test]
    fn construction_and_dependent_gates_follow_frozen_predicates() {
        let measured = evidence();
        let construction = run(Arm::TruthfulHandConstructionAdmission);
        assert_eq!(
            construction.outcome == "survived",
            construction_survived(&measured.complete)
        );
        let fresh = run(Arm::ChildFreshnessBoundary);
        if construction_survived(&measured.complete) {
            assert_eq!(fresh.outcome == "survived", measured.freshness.survived);
        } else {
            assert_eq!(fresh.outcome, "inconclusive");
        }
        let joint = run(Arm::ReflectedJointRetry);
        if !construction_survived(&measured.complete) || !measured.freshness.survived {
            assert_eq!(joint.outcome, "inconclusive");
        }
    }

    #[test]
    fn duplicate_disconnected_and_single_closure_controls_construct_none() {
        let measured = evidence();
        assert_eq!(measured.delivery_only.aggregate().events.constructions, 0);
        let closures = &measured.complete.aggregate().events.closure_evidence;
        if closures.contains(&1) {
            let first_construction = measured
                .complete
                .history()
                .iter()
                .position(|step| step.events.constructions > 0);
            let first_closure = measured
                .complete
                .history()
                .iter()
                .position(|step| step.events.closure_evidence.contains(&1));
            assert!(first_construction.is_none_or(|construction| {
                first_closure.is_some_and(|closure| construction > closure)
            }));
        }
        assert!(measured.complete.aggregate().events.rejected_returns > 0 || closures.is_empty());
    }

    #[test]
    fn factorial_localizes_first_incomplete_transition() {
        let observed = run(Arm::SurfacePathDeliveryFactorialLocalization);
        assert_eq!(observed.outcome, "survived", "{observed:#?}");
        assert!(
            observed.observations["localization"]["explanation"]
                .as_str()
                .is_some_and(|text| !text.is_empty())
        );
    }

    #[test]
    fn fixed_perturbation_is_conditional_and_external() {
        let measured = evidence();
        let should_run = construction_survived(&measured.complete)
            && measured.freshness.survived
            && measured.complete.aggregate().closes_joint();
        assert_eq!(measured.perturbation.is_some(), should_run);
        if let Some(perturbation) = &measured.perturbation {
            assert_eq!(perturbation.imposed_position, UPPER);
        }
    }

    #[test]
    fn inherited_integrity_control() {
        let observed = run(Arm::InheritedIntegrityControl);
        assert_eq!(observed.outcome, "survived", "{observed:#?}");
    }

    #[test]
    fn causal_origin_selection_preserves_existing_trace_fields() {
        let hand = run_reflected_hand_bounded(
            Protocol::RecursiveLearnerBoundaryEffectTerminal,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            256,
        );
        let selections = hand
            .trajectory
            .iter()
            .flat_map(|step| &step.causal_origin_selection)
            .filter(|selection| selection.is_motor)
            .collect::<Vec<_>>();
        assert!(!selections.is_empty());
        assert!(selections.iter().all(|selection| {
            selection.executable_groups <= selection.origin_count
                && selection.selected_origin.is_some() == selection.selected_ownership.is_some()
                && selection.selected_origin.is_some() == (selection.selected_path_inputs > 0)
        }));
        assert_eq!(
            selections
                .iter()
                .filter(|selection| selection.selected_origin.is_some())
                .count() as u64,
            hand.trajectory
                .iter()
                .map(|step| step.causal_origin_selections)
                .sum::<u64>()
        );
    }

    #[test]
    fn explicit_batched_effect_composition_is_the_frozen_reference() {
        let inherited = run_reflected_hand_bounded(
            Protocol::RecursiveLearnerRootFreshOpportunity,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            256,
        );
        let explicit = run_reflected_hand_bounded_with_effect_composition(
            Protocol::RecursiveLearnerRootFreshOpportunity,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            256,
            EffectComposition::Batched,
        );
        assert_eq!(inherited, explicit);
    }

    #[test]
    fn reflected_hand_step_evidence_retains_topology() {
        let observed = run_reflected_hand_bounded(
            Protocol::RecursiveLearnerConstructionOutcomeComposition,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            256,
        );

        assert!(observed.trajectory.iter().all(|step| step.junctions > 0));
        assert!(observed.trajectory.iter().all(|step| step.links > 0));
        assert_eq!(
            observed.trajectory.last().map(|step| step.learners),
            Some(observed.learners)
        );
    }

    #[test]
    fn reflected_hand_phase_work_preserves_step_totals() {
        let observed = run_reflected_hand_bounded(
            Protocol::RecursiveLearnerBoundedConstructionContinuation,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            256,
        );

        for step in &observed.trajectory {
            assert_eq!(
                step.phase_work
                    .iter()
                    .map(|phase| phase.execution_cost.comparisons)
                    .sum::<u64>(),
                step.comparisons
            );
            assert_eq!(
                step.phase_work
                    .iter()
                    .map(|phase| phase.execution_cost.scans)
                    .sum::<u64>(),
                step.scans
            );
            assert_eq!(
                step.phase_work
                    .iter()
                    .map(|phase| phase.work.total)
                    .sum::<u64>(),
                step.work.total
            );
            assert_eq!(
                step.phase_work
                    .iter()
                    .map(|phase| phase.work.physical_total)
                    .sum::<u64>(),
                step.work.physical_total
            );
            assert!(step.execution_cost.comparisons_reconciled);
            assert!(
                step.phase_work
                    .iter()
                    .all(|phase| phase.execution_cost.comparisons_reconciled)
            );
        }
    }

    #[test]
    fn reflected_hand_return_bearing_continuation_closes_the_joint() {
        let observed = run_reflected_hand_bounded(
            Protocol::RecursiveLearnerReturnBearingContinuation,
            512,
            2_048,
            256,
        );
        assert!(
            observed.primary_closed,
            "trajectory: {:?}",
            observed
                .trajectory
                .iter()
                .map(|step| (step.position_before, step.position_after))
                .collect::<Vec<_>>()
        );
        assert!(observed.perturbation_recovered);
        assert!(observed.exact_replay);
        assert!(observed.naturally_quiescent);
        assert!(!observed.stopped);
    }

    #[test]
    fn completed_cycle_is_visible_in_the_official_batched_hand() {
        let observed = run_reflected_hand_bounded(
            Protocol::RecursiveLearnerCompletedCycle,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            256,
        );
        let completed = observed
            .trajectory
            .iter()
            .flat_map(|step| &step.completed_cycle_continuations)
            .collect::<Vec<_>>();

        assert_eq!(observed.effect_composition, EffectComposition::Batched);
        assert!(observed.exact_replay);
        assert!(observed.naturally_quiescent);
        assert!(!observed.stopped);
        assert!(!completed.is_empty());
        assert!(completed.iter().any(|decision| {
            decision.crosses_ownership_view
                && decision.admitted
                && decision.consequence_tick == decision.unique_latest_tick
        }));
    }

    #[test]
    fn output_choice_resolution_preserves_the_completed_cycle_hand() {
        let observed = run_reflected_hand_bounded(
            Protocol::RecursiveLearnerCompletedCycle,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            256,
        );
        let choices = observed
            .trajectory
            .iter()
            .flat_map(|step| &step.output_choice_resolutions)
            .collect::<Vec<_>>();
        let completed = observed
            .trajectory
            .iter()
            .flat_map(|step| &step.completed_cycle_continuations)
            .collect::<Vec<_>>();
        let opposing_output_steps = observed
            .trajectory
            .iter()
            .filter(|step| {
                step.emitted_outputs.contains(&20_000) && step.emitted_outputs.contains(&20_001)
            })
            .count();
        let propagation_exhaustions = observed
            .trajectory
            .iter()
            .map(|step| step.propagation_budget_exhaustions)
            .sum::<u64>();

        assert!(!choices.is_empty());
        assert!(choices.iter().all(|choice| !choice.admitted.is_empty()));
        assert_eq!(observed.actual_position_changes, 12);
        assert_eq!(opposing_output_steps, 4);
        assert_eq!(observed.final_position, -2);
        assert!(!observed.reached_lower && !observed.reached_upper);
        assert!(!observed.escaped_lower && !observed.escaped_upper);
        assert_eq!(completed.iter().filter(|effect| effect.admitted).count(), 9);
        assert_eq!(
            completed
                .iter()
                .filter(|effect| effect.admitted && effect.crosses_ownership_view)
                .count(),
            2
        );
        assert_eq!(propagation_exhaustions, 0);
        assert!(observed.exact_replay);
        assert!(observed.naturally_quiescent);
        assert!(!observed.stopped);
    }

    #[test]
    fn existing_witness_trace_preserves_existing_events_and_hand() {
        let observed = run_reflected_hand_bounded(
            Protocol::RecursiveLearnerCompletedCycle,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            256,
        );
        let witness_events = observed
            .trajectory
            .iter()
            .flat_map(|step| &step.existing_witness_trace)
            .collect::<Vec<_>>();
        let opposing_output_steps = observed
            .trajectory
            .iter()
            .filter(|step| {
                step.emitted_outputs.contains(&20_000) && step.emitted_outputs.contains(&20_001)
            })
            .count();

        assert!(!witness_events.is_empty());
        for step in &observed.trajectory {
            assert!(step.existing_witness_trace.windows(2).all(|events| {
                (events[0].tick, events[0].phase) <= (events[1].tick, events[1].phase)
            }));
            for write in &step.consequence_writes {
                assert!(step.existing_witness_trace.iter().any(|entry| {
                    entry.tick == write.tick
                        && matches!(
                            entry.event,
                            ExistingWitnessEvent::ConsequenceRecorded { link, junction }
                                if link == write.link && junction == write.junction
                        )
                }));
            }
            assert_eq!(
                step.existing_witness_trace
                    .iter()
                    .filter(|entry| matches!(
                        entry.event,
                        ExistingWitnessEvent::LearnerConstructed { .. }
                    ))
                    .count() as u64,
                step.constructions
            );
            assert_eq!(
                step.existing_witness_trace
                    .iter()
                    .filter(|entry| matches!(
                        entry.event,
                        ExistingWitnessEvent::OutputChoiceResolved(_)
                    ))
                    .count(),
                step.output_choice_resolutions.len()
            );
        }
        assert_eq!(observed.actual_position_changes, 12);
        assert_eq!(opposing_output_steps, 4);
        assert_eq!(observed.final_position, -2);
        assert!(!observed.reached_lower && !observed.reached_upper);
        assert!(!observed.escaped_lower && !observed.escaped_upper);
        assert!(observed.exact_replay);
        assert!(observed.naturally_quiescent);
        assert!(!observed.stopped);
    }

    #[test]
    fn sequential_effect_composition_records_each_quiescent_world_arrow() {
        let observed = run_reflected_hand_bounded_with_effect_composition(
            Protocol::RecursiveLearnerRootFreshOpportunity,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            256,
            EffectComposition::QuiescentPhaseSequential,
        );
        assert!(observed.exact_replay);
        assert!(observed.naturally_quiescent);
        assert_eq!(
            observed.actual_position_changes,
            observed
                .trajectory
                .iter()
                .map(|step| step.actual_position_changes)
                .sum::<usize>()
        );
        assert!(
            observed
                .trajectory
                .iter()
                .all(|step| (1..=2).contains(&step.phase_directions.len()))
        );
    }

    #[test]
    fn physical_surface_capacity_exhaustion_is_typed_and_replayable() {
        let observed = trial_with_capacity(AdmissionCell::COMPLETE, 64, 64);
        assert!(matches!(
            observed.stop(),
            Some(PhysicalStop::JunctionCapacity | PhysicalStop::LinkCapacity)
        ));
        assert!(observed.exact_replay());
    }

    #[test]
    fn mirrored_perturbation_is_held_out_and_deterministic_when_primary_closes() {
        let measured = evidence();
        if construction_survived(&measured.complete)
            && measured.freshness.survived
            && measured.complete.aggregate().closes_joint()
        {
            let first = execute_perturbation(LOWER);
            let second = execute_perturbation(LOWER);
            assert_eq!(first, second);
        }
    }
}
