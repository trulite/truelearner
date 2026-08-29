use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CausalOriginResolution {
    Preserved,
    JunctionFallback,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReturnOriginDecision {
    AdmittedDirect,
    AdmittedLocal,
    RejectedAlreadyRemembered,
    RejectedAlreadyAdmittedThisMoment,
    RejectedBeforeReturnOpened,
    RejectedUnchangedSample,
    RejectedMissingLink,
    RejectedInactiveLink,
    RejectedWrongMode,
    RejectedMissingSource,
    RejectedMissingTarget,
    RejectedOriginNotFound,
    RejectedNonLocal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReversePathDecision {
    Consolidated,
    MissingReturnLink,
    OriginNotFound,
    OriginIsReturnSource,
    NoParticipatingActionPath,
    MissingActionLink,
    ZeroActionStrength,
    NoCompatibleReversePath,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CandidateOwnership {
    Organism,
    Owned(LearnerId),
    Ambiguous,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FreshOpportunityDecision {
    Admitted,
    RejectedRecentDonor,
    RejectedOwnerMismatch,
    RejectedNotSelected,
    RejectedBelowThreshold,
    RejectedRecipientHasReturn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LearnerOwnershipRelation {
    SameOwner,
    OrganismToRoot,
    RootToOrganism,
    ParentToChild,
    ChildToParent,
    Siblings,
    Unrelated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletedCycleState {
    NotApplicable,
    Missing,
    Stale,
    AmbiguousLatest,
    Unique,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NaturalCycleClosureDecision {
    NoTransition,
    NoMatchingPath,
    Ambiguous,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputChoiceBasis {
    CurrentTransition,
    CoherentEffect,
    CompletedCycle,
    FreshAlternative,
    Ordinary,
    RecentCohort,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputCompetitionBasis {
    ImmediateOrigin,
    CausalPathOrigin,
    CausalTopology,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputAdmission {
    pub target: JunctionId,
    pub owner: Option<LearnerId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PhysicalEvent {
    PhysicalIncidenceObserved {
        target: JunctionId,
        origin_physical: u64,
        incidence: PhysicalIncidence,
        causal_wave: u64,
    },
    CausalLineageMemberObserved {
        target: JunctionId,
        origin_physical: u64,
        mode: TransmissionMode,
        link: Option<LinkId>,
        generation: Option<u32>,
        causal_wave: u64,
    },
    DriveOriginObserved {
        target: JunctionId,
        origin_physical: u64,
        link: Option<LinkId>,
        generation: Option<u32>,
        causal_wave: u64,
    },
    DriveProvenanceObserved {
        source: Option<JunctionId>,
        target: JunctionId,
        source_physical: Option<u64>,
        target_physical: u64,
        source_region: Option<i16>,
        target_region: i16,
        link: Option<LinkId>,
        completes_path: bool,
        carried_origin: u64,
        origin_owner: Option<LearnerId>,
        path_owner: Option<LearnerId>,
        strength: i64,
        causal_wave: u64,
    },
    CausalOriginResolved {
        target: JunctionId,
        distinct_origins: u32,
        resolved_origin: u64,
        resolution: CausalOriginResolution,
        causal_wave: u64,
    },
    NaturalCycleClosureEvaluated {
        surface: JunctionId,
        matching_paths: u32,
        decision: NaturalCycleClosureDecision,
    },
    NaturalCycleClosed {
        surface: JunctionId,
        output: JunctionId,
        first: LinkId,
        second: LinkId,
    },
    DriveIncidence {
        target: JunctionId,
        arrivals: u32,
        impulse: i32,
        causal_wave: u64,
    },
    ModulatoryOriginObserved {
        target: JunctionId,
        origin_physical: u64,
        link: Option<LinkId>,
        generation: Option<u32>,
        causal_wave: u64,
    },
    ModulatoryIncidence {
        target: JunctionId,
        arrivals: u32,
        impulse: i32,
        causal_wave: u64,
    },
    MaterialDriveIncidence {
        target: JunctionId,
        impulse: i64,
        activation_after: i64,
        causal_wave: u64,
    },
    PathChosen {
        target: JunctionId,
        positive_strength: u64,
        negative_strength: u64,
        opportunity_active: bool,
        admitted_sign: i8,
    },
    OutputWaveFinished {
        target: JunctionId,
        activation: i64,
    },
    LinkStrengthened {
        link: LinkId,
        from: JunctionId,
        to: JunctionId,
        coupling_before: i32,
        coupling_after: i32,
    },
    Deliver {
        mode: TransmissionMode,
        target: JunctionId,
        impulse: i32,
    },
    Fire {
        junction: JunctionId,
    },
    Resistance {
        link: LinkId,
        before: u32,
        after: u32,
    },
    Coupling {
        link: LinkId,
        before: i32,
        after: i32,
    },
    Deallocate {
        link: LinkId,
    },
    JunctionDeallocate {
        junction: JunctionId,
        before_generation: u32,
        after_generation: u32,
    },
    JunctionProposal {
        junction: JunctionId,
        source: JunctionId,
        target: JunctionId,
    },
    Proposal {
        link: LinkId,
        from: JunctionId,
        to: JunctionId,
    },
    Output(Output),
    QualifiedLocalTraversal {
        link: LinkId,
    },
    ConsequenceRecorded {
        link: LinkId,
        junction: JunctionId,
    },
    OrganismConsequenceConsumed {
        target: JunctionId,
        link: LinkId,
        generation: u32,
        consequence_tick: i64,
    },
    ProprioceptiveOpportunity {
        owner: LearnerId,
        target: JunctionId,
        origin_physical: u64,
        admitted: bool,
    },
    LearnerConsequenceRecorded {
        owner: LearnerId,
        link: LinkId,
        generation: u32,
        tick: i64,
    },
    LearnerCandidatePreference {
        owner: LearnerId,
        target: JunctionId,
        consequence_tick: Option<i64>,
        admitted: bool,
    },
    SurfacePathStateObserved {
        surface: JunctionId,
        owner: Option<LearnerId>,
        complete_paths: u32,
        consequential_paths: u32,
    },
    OutputCandidateEvaluated {
        target: JunctionId,
        ownership: CandidateOwnership,
        path_inputs: u32,
        path_origins: Vec<u64>,
        distinct_path_origins: u32,
        distinct_path_owners: u32,
        positive_path_strength: u64,
        negative_path_strength: u64,
        opportunity: i64,
        supplied_opportunity: i64,
        admitted_drive: i64,
        projected_drive: i64,
        threshold: i64,
        consequence_tick: Option<i64>,
        unanswered_returns: u32,
        executable: bool,
    },
    OutputCompetitionComponent {
        target: JunctionId,
        outcome_source: Option<JunctionId>,
        component: Option<u64>,
        basis: OutputCompetitionBasis,
    },
    FreshOpportunityTransferred {
        donor: JunctionId,
        recipient: JunctionId,
        return_link: LinkId,
        owner: Option<LearnerId>,
        opportunity: i64,
    },
    FreshOpportunityEvaluated {
        donor: JunctionId,
        recipient: JunctionId,
        return_link: LinkId,
        return_owner: Option<LearnerId>,
        recipient_owner: Option<LearnerId>,
        ownership_relation: LearnerOwnershipRelation,
        decision: FreshOpportunityDecision,
    },
    PhysicalTransitionContinuationEvaluated {
        target: JunctionId,
        owner: Option<LearnerId>,
        current_owner_transition: bool,
        unanswered_returns: u32,
        admitted: bool,
    },
    CoherentEffectEvaluated {
        target: JunctionId,
        owner: Option<LearnerId>,
        latest_unanswered_opened_tick: Option<i64>,
        unanswered_returns: u32,
        admitted: bool,
    },
    CompletedCycleContinuationEvaluated {
        target: JunctionId,
        owner: Option<LearnerId>,
        consequence_tick: Option<i64>,
        consequence_witnesses: Vec<(LinkId, u32)>,
        unique_latest_tick: Option<i64>,
        crosses_ownership_view: bool,
        admitted: bool,
    },
    ConstructionContinuationConsumed {
        target: JunctionId,
        owner: LearnerId,
        link: LinkId,
        generation: u32,
        consequence_tick: i64,
    },
    OutputChoiceResolved {
        ordinary_target: JunctionId,
        current_transition_target: Option<JunctionId>,
        coherent_effect_target: Option<JunctionId>,
        completed_cycle_target: Option<JunctionId>,
        computed_winner_target: JunctionId,
        admitted: Vec<OutputAdmission>,
        computed_winner_basis: OutputChoiceBasis,
        admission_basis: OutputChoiceBasis,
        completed_cycle_state: CompletedCycleState,
        crosses_ownership_view: bool,
    },
    MixedOwnerCandidateResolved {
        target: JunctionId,
        owner_count: u32,
        executable_groups: u32,
        selected_owner: Option<LearnerId>,
        selected_path_inputs: u32,
    },
    CausalOriginCandidateResolved {
        target: JunctionId,
        origin_count: u32,
        executable_groups: u32,
        selected_origin: Option<u64>,
        selected_ownership: Option<CandidateOwnership>,
        selected_path_inputs: u32,
    },
    PropagationBudgetExhausted {
        moments: u64,
    },
    ReversePathConsolidated {
        source: JunctionId,
        output: JunctionId,
        link: LinkId,
    },
    ClosureEligibilityEvaluated {
        return_link: LinkId,
        origin_physical: u64,
        origin_birth_tick: i64,
        return_opened_tick: i64,
        eligible: bool,
    },
    PhysicalTransitionEligibilityEvaluated {
        return_link: LinkId,
        origin_physical: u64,
        transition_tick: Option<i64>,
        return_opened_tick: i64,
        eligible: bool,
    },
    ReturnCohortClosed {
        source: JunctionId,
        opened_tick: i64,
        link_count: u32,
    },
    BoundaryNoveltyEvaluated {
        parent: Option<LearnerId>,
        surface: JunctionId,
        output: JunctionId,
        proposed_members: u32,
        novel_members: u32,
        eligible: bool,
    },
    CandidateSelection {
        target: JunctionId,
        origin_scope: Option<u64>,
        consequence_tick: Option<i64>,
        admitted: bool,
    },
    ReturnSuperseded {
        link: LinkId,
    },
    ReturnScheduling {
        owner: Option<LearnerId>,
        link: LinkId,
        generation: u32,
        admitted: bool,
    },
    ReturnOriginAdmission {
        owner: Option<LearnerId>,
        link: LinkId,
        generation: u32,
        origin_physical: u64,
        admitted: bool,
    },
    ReturnOriginEvaluated {
        owner: Option<LearnerId>,
        link: LinkId,
        generation: u32,
        origin_physical: u64,
        source: Option<JunctionId>,
        target: Option<JunctionId>,
        origin: Option<JunctionId>,
        distance: Option<i32>,
        decision: ReturnOriginDecision,
    },
    ReversePathEvaluated {
        return_link: LinkId,
        origin_physical: u64,
        source: Option<JunctionId>,
        output: Option<JunctionId>,
        reverse_link: Option<LinkId>,
        decision: ReversePathDecision,
    },
    CausalClosureObserved {
        parent: Option<LearnerId>,
        surface: JunctionId,
        output: JunctionId,
        evidence: u32,
    },
    LearnerConstructed {
        learner: LearnerId,
        parent: Option<LearnerId>,
        surface: JunctionId,
        output: JunctionId,
        junction_count: u32,
        link_count: u32,
    },
}

impl PhysicalEvent {
    pub fn is_diagnostic(&self) -> bool {
        matches!(
            self,
            Self::PhysicalIncidenceObserved { .. }
                | Self::CausalLineageMemberObserved { .. }
                | Self::DriveOriginObserved { .. }
                | Self::DriveProvenanceObserved { .. }
                | Self::CausalOriginResolved { .. }
                | Self::NaturalCycleClosureEvaluated { .. }
                | Self::ModulatoryOriginObserved { .. }
                | Self::ReturnOriginEvaluated { .. }
                | Self::ReversePathEvaluated { .. }
                | Self::ClosureEligibilityEvaluated { .. }
                | Self::PhysicalTransitionEligibilityEvaluated { .. }
                | Self::ReturnCohortClosed { .. }
                | Self::BoundaryNoveltyEvaluated { .. }
                | Self::SurfacePathStateObserved { .. }
                | Self::OutputCandidateEvaluated { .. }
                | Self::OutputCompetitionComponent { .. }
                | Self::FreshOpportunityTransferred { .. }
                | Self::FreshOpportunityEvaluated { .. }
                | Self::MixedOwnerCandidateResolved { .. }
                | Self::CausalOriginCandidateResolved { .. }
                | Self::PropagationBudgetExhausted { .. }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalTransition {
    pub tick: i64,
    pub phase: i32,
    pub event: PhysicalEvent,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub(crate) total: u64,
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExecutionCost {
    pub queue_ops: u64,
    pub comparisons: u64,
    pub timing_wheel_minimum_key_comparisons: u64,
    pub timing_wheel_bucket_selection_comparisons: u64,
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

impl ExecutionCost {
    pub fn attributed_comparisons(&self) -> u64 {
        self.timing_wheel_minimum_key_comparisons
            .saturating_add(self.timing_wheel_bucket_selection_comparisons)
    }

    pub(crate) fn touch<T>(&mut self, count: usize) {
        self.bytes_touched = self.bytes_touched.saturating_add(
            u64::try_from(std::mem::size_of::<T>().saturating_mul(count)).unwrap_or(u64::MAX),
        );
    }

    pub(crate) fn observe_memory_bytes(&mut self, bytes: usize) {
        self.peak_memory_bytes = self
            .peak_memory_bytes
            .max(u64::try_from(bytes).unwrap_or(u64::MAX));
    }

    pub(crate) fn observe_frontier(&mut self, active: usize) {
        let active = u64::try_from(active).unwrap_or(u64::MAX);
        self.frontier_samples = self.frontier_samples.saturating_add(1);
        self.active_frontier_total = self.active_frontier_total.saturating_add(active);
        self.active_frontier_max = self.active_frontier_max.max(active);
    }

    pub(crate) fn observe_batch(&mut self, size: usize) {
        let size = u64::try_from(size).unwrap_or(u64::MAX);
        self.batches = self.batches.saturating_add(1);
        self.batched_items = self.batched_items.saturating_add(size);
        self.batch_max = self.batch_max.max(size);
        let bucket = match size {
            0 | 1 => 0,
            2 => 1,
            3..=4 => 2,
            5..=8 => 3,
            9..=16 => 4,
            17..=32 => 5,
            _ => 6,
        };
        self.batch_histogram[bucket] = self.batch_histogram[bucket].saturating_add(1);
    }
}

impl Work {
    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn physical_total(&self) -> u64 {
        let total = self
            .drive_deliveries
            .saturating_add(self.modulatory_deliveries)
            .saturating_add(self.local_return_updates)
            .saturating_add(self.local_structural_proposals)
            .saturating_add(self.physical_deallocations);
        let total = total.saturating_add(self.junction_deallocations);
        let total = total.saturating_add(self.local_junction_proposals);
        let total = total.saturating_add(self.qualified_local_traversals);
        let total = total.saturating_add(self.causal_closure_observations);
        total.saturating_add(self.learner_constructions)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunResult {
    pub outputs: Vec<Output>,
    pub work: Work,
    pub naturally_quiescent: bool,
    pub memory_bytes: usize,
    pub execution_cost: ExecutionCost,
    pub physical_trace: Vec<PhysicalTransition>,
}

impl RunResult {
    pub fn physical_diagnostics(&self) -> impl Iterator<Item = &PhysicalTransition> {
        self.physical_trace
            .iter()
            .filter(|transition| transition.event.is_diagnostic())
    }
}
