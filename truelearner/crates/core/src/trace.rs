use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PhysicalEvent {
    DriveIncidence {
        target: JunctionId,
        arrivals: u32,
        impulse: i32,
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
    ReversePathConsolidated {
        source: JunctionId,
        output: JunctionId,
        link: LinkId,
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
