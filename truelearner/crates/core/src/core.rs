//! The algorithm and its public contract.

pub use crate::checkpoint::{Checkpoint, CheckpointError};
pub use crate::identity::{JunctionId, LearnerId, LinkId};
pub use crate::junction::Junction;
pub use crate::link::{Link, TransmissionMode, TransmissionTrigger};
pub use crate::schedule::PhysicalClock;
pub use crate::trace::{
    CandidateOwnership, CausalOriginResolution, CompletedCycleState, ExecutionCost,
    FreshOpportunityDecision, LearnerOwnershipRelation, OutputAdmission, OutputChoiceBasis,
    OutputCompetitionBasis, PhysicalEvent, PhysicalTransition, ReturnOriginDecision,
    ReversePathDecision, RunResult as Run, Work,
};

use crate::body::Body;
use crate::junction::JunctionState;
use crate::schedule::{CausalLineage, Firing};
use crate::trace::RunResult;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub(crate) struct RunState {
    pub outputs: Vec<Output>,
    pub work: Work,
    pub cost: ExecutionCost,
    pub trace: Vec<PhysicalTransition>,
}

pub(crate) struct Moment {
    pub phase: i32,
    pub causal: u64,
    pub incidences: Vec<Incidence>,
}

pub(crate) struct Incidence {
    pub junction: JunctionId,
    pub inputs: Vec<Firing>,
    pub outcomes: Vec<Firing>,
    pub supplied_opportunity: i64,
}

pub(crate) struct Fired {
    pub junction: JunctionId,
    pub state: JunctionState,
    pub external: bool,
    pub boundary_effect: bool,
    pub causal_origin: u64,
    pub causal_lineage: Option<CausalLineage>,
}

/// A coherent choice of physical implementation for the algorithm.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    #[default]
    Physical,
    UnansweredReturnDeferral,
    UnansweredReturnReplacement,
    SensorimotorCandidate,
    SensorimotorSynthesis,
    RecursiveLearnerConstruction,
    RecursiveLearnerCausalLineage,
    RecursiveLearnerConsequenceBornClosure,
    RecursiveLearnerConsequenceCohortClosure,
    RecursiveLearnerEligibleReturnClosure,
    RecursiveLearnerBoundaryNovelty,
    RecursiveLearnerOwnerFactorization,
    RecursiveLearnerCausalOriginFactorization,
    RecursiveLearnerRegionalPathClosure,
    RecursiveLearnerBoundaryEffectTerminal,
    RecursiveLearnerConsequenceBornReturn,
    RecursiveLearnerPhysicalTransitionReturn,
    RecursiveLearnerFreshOpportunity,
    RecursiveLearnerRootFreshOpportunity,
    RecursiveLearnerTransitionContinuation,
    RecursiveLearnerCoherentEffect,
    RecursiveLearnerCompletedCycle,
    RecursiveLearnerConstructionOutcomeComposition,
    RecursiveLearnerBoundedConstructionContinuation,
    RecursiveLearnerReturnBearingContinuation,
    RecursiveLearnerCausalOriginProductComposition,
    RecursiveLearnerCausalPathProductComposition,
    RecursiveLearnerCausalTopologyOutputComposition,
    RecursiveLearnerCausalTopologyOpportunityComposition,
    RecursiveLearnerCausalTopologyProductComposition,
}

impl Protocol {
    pub(crate) fn is_sensorimotor(self) -> bool {
        matches!(
            self,
            Self::SensorimotorCandidate
                | Self::SensorimotorSynthesis
                | Self::RecursiveLearnerConstruction
                | Self::RecursiveLearnerCausalLineage
                | Self::RecursiveLearnerConsequenceBornClosure
                | Self::RecursiveLearnerConsequenceCohortClosure
                | Self::RecursiveLearnerEligibleReturnClosure
                | Self::RecursiveLearnerBoundaryNovelty
                | Self::RecursiveLearnerOwnerFactorization
                | Self::RecursiveLearnerCausalOriginFactorization
                | Self::RecursiveLearnerRegionalPathClosure
                | Self::RecursiveLearnerBoundaryEffectTerminal
                | Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn consolidates_reverse_paths(self) -> bool {
        matches!(
            self,
            Self::SensorimotorSynthesis
                | Self::RecursiveLearnerConstruction
                | Self::RecursiveLearnerCausalLineage
                | Self::RecursiveLearnerConsequenceBornClosure
                | Self::RecursiveLearnerConsequenceCohortClosure
                | Self::RecursiveLearnerEligibleReturnClosure
                | Self::RecursiveLearnerBoundaryNovelty
                | Self::RecursiveLearnerOwnerFactorization
                | Self::RecursiveLearnerCausalOriginFactorization
                | Self::RecursiveLearnerRegionalPathClosure
                | Self::RecursiveLearnerBoundaryEffectTerminal
                | Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn integrates_current_opportunity(self) -> bool {
        self.consolidates_reverse_paths()
    }

    pub(crate) fn constructs_learners(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerConstruction
                | Self::RecursiveLearnerCausalLineage
                | Self::RecursiveLearnerConsequenceBornClosure
                | Self::RecursiveLearnerConsequenceCohortClosure
                | Self::RecursiveLearnerEligibleReturnClosure
                | Self::RecursiveLearnerBoundaryNovelty
                | Self::RecursiveLearnerOwnerFactorization
                | Self::RecursiveLearnerCausalOriginFactorization
                | Self::RecursiveLearnerRegionalPathClosure
                | Self::RecursiveLearnerBoundaryEffectTerminal
                | Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn preserves_causal_lineage(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerCausalLineage
                | Self::RecursiveLearnerConsequenceBornClosure
                | Self::RecursiveLearnerConsequenceCohortClosure
                | Self::RecursiveLearnerEligibleReturnClosure
                | Self::RecursiveLearnerBoundaryNovelty
                | Self::RecursiveLearnerOwnerFactorization
                | Self::RecursiveLearnerCausalOriginFactorization
                | Self::RecursiveLearnerRegionalPathClosure
                | Self::RecursiveLearnerBoundaryEffectTerminal
                | Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn requires_consequence_born_closure(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerConsequenceBornClosure
                | Self::RecursiveLearnerConsequenceCohortClosure
                | Self::RecursiveLearnerEligibleReturnClosure
                | Self::RecursiveLearnerBoundaryNovelty
                | Self::RecursiveLearnerOwnerFactorization
                | Self::RecursiveLearnerCausalOriginFactorization
                | Self::RecursiveLearnerRegionalPathClosure
                | Self::RecursiveLearnerBoundaryEffectTerminal
                | Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn closes_return_cohort(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerConsequenceCohortClosure
                | Self::RecursiveLearnerEligibleReturnClosure
                | Self::RecursiveLearnerBoundaryNovelty
                | Self::RecursiveLearnerOwnerFactorization
                | Self::RecursiveLearnerCausalOriginFactorization
                | Self::RecursiveLearnerRegionalPathClosure
                | Self::RecursiveLearnerBoundaryEffectTerminal
                | Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn prioritizes_eligible_returns(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerEligibleReturnClosure
                | Self::RecursiveLearnerBoundaryNovelty
                | Self::RecursiveLearnerOwnerFactorization
                | Self::RecursiveLearnerCausalOriginFactorization
                | Self::RecursiveLearnerRegionalPathClosure
                | Self::RecursiveLearnerBoundaryEffectTerminal
                | Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn requires_boundary_novelty(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerBoundaryNovelty
                | Self::RecursiveLearnerOwnerFactorization
                | Self::RecursiveLearnerCausalOriginFactorization
                | Self::RecursiveLearnerRegionalPathClosure
                | Self::RecursiveLearnerBoundaryEffectTerminal
                | Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn factors_candidate_owners(self) -> bool {
        self == Self::RecursiveLearnerOwnerFactorization
    }

    pub(crate) fn factors_candidate_origins(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerCausalOriginFactorization
                | Self::RecursiveLearnerRegionalPathClosure
                | Self::RecursiveLearnerBoundaryEffectTerminal
                | Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn requires_regional_path_closure(self) -> bool {
        self == Self::RecursiveLearnerRegionalPathClosure
    }

    pub(crate) fn terminates_boundary_effect_formation(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerBoundaryEffectTerminal
                | Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn requires_consequence_born_return(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerConsequenceBornReturn
                | Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn requires_physical_transition_return(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerPhysicalTransitionReturn
                | Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn supplies_fresh_opportunity(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerFreshOpportunity
                | Self::RecursiveLearnerRootFreshOpportunity
                | Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub fn admits_fresh_opportunity_relation(self, relation: LearnerOwnershipRelation) -> bool {
        match self {
            Self::RecursiveLearnerFreshOpportunity => {
                relation == LearnerOwnershipRelation::SameOwner
            }
            Self::RecursiveLearnerRootFreshOpportunity
            | Self::RecursiveLearnerTransitionContinuation
            | Self::RecursiveLearnerCoherentEffect
            | Self::RecursiveLearnerCompletedCycle
            | Self::RecursiveLearnerConstructionOutcomeComposition
            | Self::RecursiveLearnerBoundedConstructionContinuation
            | Self::RecursiveLearnerReturnBearingContinuation
            | Self::RecursiveLearnerCausalOriginProductComposition
            | Self::RecursiveLearnerCausalPathProductComposition
            | Self::RecursiveLearnerCausalTopologyOutputComposition
            | Self::RecursiveLearnerCausalTopologyOpportunityComposition
            | Self::RecursiveLearnerCausalTopologyProductComposition => matches!(
                relation,
                LearnerOwnershipRelation::SameOwner | LearnerOwnershipRelation::OrganismToRoot
            ),
            _ => false,
        }
    }

    pub(crate) fn continues_current_physical_transition(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerTransitionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn admits_return_bearing_continuation(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn composes_independent_output_products(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn uses_causal_path_output_products(self) -> bool {
        self == Self::RecursiveLearnerCausalPathProductComposition
    }

    pub(crate) fn uses_causal_topology_output_products(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn uses_causal_topology_opportunity_products(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn coheres_recent_unanswered_effect(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerCoherentEffect
                | Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn composes_completed_physical_cycle(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerCompletedCycle
                | Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn composes_construction_outcome(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerConstructionOutcomeComposition
                | Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }

    pub(crate) fn holds_construction_outcome_for_first_choice(self) -> bool {
        matches!(
            self,
            Self::RecursiveLearnerBoundedConstructionContinuation
                | Self::RecursiveLearnerReturnBearingContinuation
                | Self::RecursiveLearnerCausalOriginProductComposition
                | Self::RecursiveLearnerCausalPathProductComposition
                | Self::RecursiveLearnerCausalTopologyOutputComposition
                | Self::RecursiveLearnerCausalTopologyOpportunityComposition
                | Self::RecursiveLearnerCausalTopologyProductComposition
        )
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Bindings {
    pub(crate) start: fn(&mut Body) -> RunState,
    pub(crate) links_meet: fn(&mut Body, &mut RunState) -> Option<Moment>,
    pub(crate) choose: fn(&mut Body, &mut Moment, &mut RunState),
    pub(crate) outcome_returns: fn(&mut Body, &Moment, &mut RunState),
    pub(crate) strengthen: fn(&mut Body, &Moment, &mut RunState),
    pub(crate) fire_junction: fn(&mut Body, Incidence, &Moment, &mut RunState) -> Option<Fired>,
    pub(crate) form_paths: fn(&mut Body, &Fired, &Moment, &mut RunState),
    pub(crate) fire_output: fn(&mut Body, Fired, &Moment, &mut RunState),
    pub(crate) hold: fn(&mut Body, &mut RunState),
    pub(crate) finish: fn(&mut Body, RunState) -> RunResult,
}

pub(crate) fn run(body: &mut Body, protocol: Bindings) -> RunResult {
    run_with_limit(body, protocol, None)
}

pub(crate) fn run_bounded(body: &mut Body, protocol: Bindings, max_moments: u64) -> RunResult {
    run_with_limit(body, protocol, Some(max_moments))
}

fn run_with_limit(body: &mut Body, protocol: Bindings, max_moments: Option<u64>) -> RunResult {
    let mut run = (protocol.start)(body);
    let mut moments = 0_u64;
    // One loop is one physical moment. The full story in algo.md unfolds
    // across moments as input, output, and later outcome reach junctions.
    while !body.pending.is_empty() {
        if max_moments.is_some_and(|limit| moments >= limit) {
            if body.trace_physics {
                run.trace.push(PhysicalTransition {
                    tick: body.tick,
                    phase: 0,
                    event: PhysicalEvent::PropagationBudgetExhausted { moments },
                });
            }
            break;
        }
        let Some(mut moment) = (protocol.links_meet)(body, &mut run) else {
            break;
        };
        moments = moments.saturating_add(1);
        (protocol.choose)(body, &mut moment, &mut run);
        (protocol.outcome_returns)(body, &moment, &mut run);
        (protocol.strengthen)(body, &moment, &mut run);
        let time = Moment {
            phase: moment.phase,
            causal: moment.causal,
            incidences: Vec::new(),
        };
        for incidence in moment.incidences {
            if let Some(fired) = (protocol.fire_junction)(body, incidence, &time, &mut run) {
                (protocol.form_paths)(body, &fired, &time, &mut run);
                // New paths and strengthened paths execute by the same rule.
                (protocol.fire_output)(body, fired, &time, &mut run);
            }
        }
        (protocol.hold)(body, &mut run);
    }
    (protocol.finish)(body, run)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub arrival_tick: i64,
    pub phase: i32,
    pub origin_physical: u64,
    pub target: JunctionId,
    pub impulse: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicalIncidence {
    #[default]
    Sample,
    Transition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhysicalInput {
    pub input: Input,
    pub incidence: PhysicalIncidence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    pub tick: i64,
    pub from_physical: u64,
    pub to_physical: u64,
    pub from_region: i16,
    pub to_region: i16,
    pub impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JunctionObservation {
    pub id: JunctionId,
    pub physical_id: u64,
    pub position: i32,
    pub region: i16,
    pub threshold: i32,
    pub resistance: u32,
    pub live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkObservation {
    pub id: LinkId,
    pub from: JunctionId,
    pub to: JunctionId,
    pub delay: i64,
    pub phase: i32,
    pub mode: TransmissionMode,
    pub coupling: i32,
    pub resistance: u32,
    pub strength: i64,
    pub life: u64,
    pub participation: u64,
    pub last_consequence_tick: Option<i64>,
    pub return_origins: Vec<u64>,
    pub live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LearnerObservation {
    pub id: LearnerId,
    pub parent: Option<LearnerId>,
    pub surface: JunctionId,
    pub output: JunctionId,
    pub junctions: Vec<JunctionId>,
    pub links: Vec<LinkId>,
}

pub fn classify_learner_ownership_relation(
    donor: Option<LearnerId>,
    recipient: Option<LearnerId>,
    learners: &[LearnerObservation],
) -> LearnerOwnershipRelation {
    classify_learner_ownership_relation_with(donor, recipient, |owner| {
        learners
            .iter()
            .find(|learner| learner.id == owner)
            .map(|learner| learner.parent)
    })
}

pub(crate) fn classify_learner_ownership_relation_with(
    donor: Option<LearnerId>,
    recipient: Option<LearnerId>,
    mut parent_of: impl FnMut(LearnerId) -> Option<Option<LearnerId>>,
) -> LearnerOwnershipRelation {
    match (donor, recipient) {
        (None, None) => LearnerOwnershipRelation::SameOwner,
        (Some(donor), Some(recipient)) if donor == recipient => {
            if parent_of(donor).is_some() {
                LearnerOwnershipRelation::SameOwner
            } else {
                LearnerOwnershipRelation::Unrelated
            }
        }
        (None, Some(recipient)) => match parent_of(recipient) {
            Some(None) => LearnerOwnershipRelation::OrganismToRoot,
            Some(Some(_)) | None => LearnerOwnershipRelation::Unrelated,
        },
        (Some(donor), None) => match parent_of(donor) {
            Some(None) => LearnerOwnershipRelation::RootToOrganism,
            Some(Some(_)) | None => LearnerOwnershipRelation::Unrelated,
        },
        (Some(donor), Some(recipient)) => {
            let donor_parent = parent_of(donor);
            let recipient_parent = parent_of(recipient);
            if recipient_parent == Some(Some(donor)) {
                LearnerOwnershipRelation::ParentToChild
            } else if donor_parent == Some(Some(recipient)) {
                LearnerOwnershipRelation::ChildToParent
            } else if donor_parent.is_some()
                && donor_parent == recipient_parent
                && donor_parent.flatten().is_some()
            {
                LearnerOwnershipRelation::Siblings
            } else {
                LearnerOwnershipRelation::Unrelated
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HarnessObservation {
    pub clock: PhysicalClock,
    pub protocol: Protocol,
    pub return_path_count: usize,
    pub resident_bytes: usize,
    pub junctions: Vec<JunctionObservation>,
    pub links: Vec<LinkObservation>,
    pub learners: Vec<LearnerObservation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HarnessBuilder {
    body: Body,
    outward_region: i16,
}

impl HarnessBuilder {
    pub fn with_capacity(junction_capacity: u32, link_capacity: u32, outward_region: i16) -> Self {
        Self {
            body: Body::with_capacity(junction_capacity, link_capacity),
            outward_region,
        }
    }

    pub fn set_physical_tracing(&mut self, enabled: bool) {
        self.body.set_physical_tracing(enabled);
    }

    pub fn set_protocol(&mut self, protocol: Protocol) {
        self.body.set_protocol(protocol);
    }

    pub fn add_junction(&mut self, spec: Junction) -> JunctionId {
        self.body.add_junction(spec)
    }

    pub fn add_link(&mut self, spec: Link) -> LinkId {
        self.body.add_link(spec)
    }

    pub fn set_link_trigger(&mut self, link: LinkId, trigger: TransmissionTrigger) {
        self.body.set_link_trigger(link, trigger);
    }

    pub fn set_outcome_source(&mut self, source: JunctionId) {
        self.body.set_outcome_source(source);
    }

    pub fn set_outcome_source_for_output(&mut self, output: JunctionId, source: JunctionId) {
        self.body.set_outcome_source_for_output(output, source);
    }

    pub fn build(self) -> Harness {
        Harness {
            body: self.body,
            outward_region: self.outward_region,
        }
    }
}

impl HarnessObservation {
    pub fn fingerprint(&self) -> [u8; 32] {
        let mut hash = Sha256::new();
        hash.update(b"truelearner-harness-observation-v1");
        for junction in &self.junctions {
            hash.update(junction.id.0.to_le_bytes());
            hash.update(junction.physical_id.to_le_bytes());
            hash.update(junction.position.to_le_bytes());
            hash.update(junction.region.to_le_bytes());
            hash.update(junction.threshold.to_le_bytes());
            hash.update(junction.resistance.to_le_bytes());
            hash.update([u8::from(junction.live)]);
        }
        for link in &self.links {
            hash.update(link.id.0.to_le_bytes());
            hash.update(link.from.0.to_le_bytes());
            hash.update(link.to.0.to_le_bytes());
            hash.update(link.delay.to_le_bytes());
            hash.update(link.phase.to_le_bytes());
            hash.update([match link.mode {
                TransmissionMode::Drive => 0,
                TransmissionMode::Modulatory => 1,
            }]);
            hash.update(link.coupling.to_le_bytes());
            hash.update(link.resistance.to_le_bytes());
            hash.update(link.strength.to_le_bytes());
            hash.update(link.life.to_le_bytes());
            hash.update(link.participation.to_le_bytes());
            hash.update(link.last_consequence_tick.unwrap_or(i64::MIN).to_le_bytes());
            hash.update(
                u64::try_from(link.return_origins.len())
                    .unwrap_or(u64::MAX)
                    .to_le_bytes(),
            );
            for origin in &link.return_origins {
                hash.update(origin.to_le_bytes());
            }
            hash.update([u8::from(link.live)]);
        }
        for learner in &self.learners {
            hash.update(learner.id.0.to_le_bytes());
            hash.update(learner.parent.unwrap_or_default().0.to_le_bytes());
            hash.update([u8::from(learner.parent.is_some())]);
            hash.update(learner.surface.0.to_le_bytes());
            hash.update(learner.output.0.to_le_bytes());
            for junction in &learner.junctions {
                hash.update(junction.0.to_le_bytes());
            }
            for link in &learner.links {
                hash.update(link.0.to_le_bytes());
            }
        }
        hash.finalize().into()
    }

    pub fn junction(&self, id: JunctionId) -> Option<&JunctionObservation> {
        self.junctions.iter().find(|junction| junction.id == id)
    }

    pub fn link(&self, id: LinkId) -> Option<&LinkObservation> {
        self.links.iter().find(|link| link.id == id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Harness {
    body: Body,
    outward_region: i16,
}

impl Harness {
    pub fn send(&mut self, inputs: &[Input]) -> Run {
        let mut next = self.body.clone();
        let mut run = next.arrive(inputs, self.outward_region);
        run.outputs
            .retain(|output| output.to_region == self.outward_region);
        self.body = next;
        run
    }

    pub fn send_physical(&mut self, inputs: &[PhysicalInput]) -> Run {
        let mut next = self.body.clone();
        let run = next.arrive_physical(inputs, self.outward_region);
        self.body = next;
        run
    }

    pub fn send_bounded(&mut self, inputs: &[Input], max_moments: u64) -> Run {
        let mut next = self.body.clone();
        for input in inputs {
            next.enter(*input);
        }
        let mut run = next.propagate_bounded(max_moments);
        run.outputs
            .retain(|output| output.to_region == self.outward_region);
        self.body = next;
        run
    }

    pub fn send_physical_bounded(&mut self, inputs: &[PhysicalInput], max_moments: u64) -> Run {
        let mut next = self.body.clone();
        for input in inputs {
            next.enter_physical(*input);
        }
        let mut run = next.propagate_bounded(max_moments);
        run.outputs
            .retain(|output| output.to_region == self.outward_region);
        self.body = next;
        run
    }

    pub fn advance_to(&mut self, tick: i64) -> Work {
        self.body.advance_time(tick)
    }

    pub fn read(&self) -> HarnessObservation {
        let junctions = self
            .body
            .arena
            .junctions
            .iter()
            .map(|junction| JunctionObservation {
                id: junction.id,
                physical_id: junction.physical_id,
                position: junction.position,
                region: junction.region,
                threshold: junction.threshold,
                resistance: junction.resistance,
                live: junction.live,
            })
            .collect();
        let links = self
            .body
            .arena
            .links
            .iter()
            .map(|link| LinkObservation {
                id: link.id,
                from: link.from,
                to: link.to,
                delay: link.delay,
                phase: link.phase,
                mode: link.mode,
                coupling: link.coupling,
                resistance: link.resistance,
                strength: self.body.arena.strength[link.id.0 as usize],
                life: self.body.arena.life[link.id.0 as usize],
                participation: link.participation_level,
                last_consequence_tick: link.last_consequence_tick,
                return_origins: link.return_origins.clone(),
                live: link.live,
            })
            .collect();
        let learners = self
            .body
            .learners
            .iter()
            .map(|learner| LearnerObservation {
                id: learner.id,
                parent: learner.parent,
                surface: learner.surface,
                output: learner.output,
                junctions: learner.junctions.clone(),
                links: learner.links.clone(),
            })
            .collect();
        HarnessObservation {
            clock: self.body.clock(),
            protocol: self.body.protocol(),
            return_path_count: self.body.return_path_count(),
            resident_bytes: self.body.arena.memory_bytes(),
            junctions,
            links,
            learners,
        }
    }

    pub fn save(&self) -> Result<Checkpoint, CheckpointError> {
        Ok(Checkpoint::new(self.body.snapshot()?, self.outward_region))
    }

    pub fn restore(checkpoint: Checkpoint) -> Result<Self, CheckpointError> {
        let (body, outward_region) = checkpoint.open();
        Ok(Self {
            body: Body::from_snapshot(body)?,
            outward_region,
        })
    }
}
