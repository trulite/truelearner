use crate::prelude::*;

const DEFAULT_JUNCTION_CAPACITY: u32 = 65_536;
const DEFAULT_LINK_CAPACITY: u32 = 262_144;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Body {
    pub(crate) arena: Arena,
    pub(crate) pending: Schedule,
    pub(crate) protocol: Protocol,
    pub(crate) tick: i64,
    pub(crate) next_serial: u64,
    pub(crate) pressure_tick: i64,
    pub(crate) trace_physics: bool,
    pub(crate) outcome_source: Option<JunctionId>,
    pub(crate) output_wave_open: bool,
}

// Body advances time. Junctions and links change inside its arena.
impl Body {
    pub(crate) fn elapse_to(
        &mut self,
        tick: i64,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
    ) {
        self.decay_links_to(tick, work, execution_cost, None, 0);
        self.retire_unlinked_junctions(tick, work, execution_cost, None, 0);
        self.elapse_activation_to(tick, execution_cost);
    }

    pub(crate) fn elapse_to_observed(
        &mut self,
        tick: i64,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        phase: i32,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) {
        self.decay_links_to(
            tick,
            work,
            execution_cost,
            Some(&mut *physical_trace),
            phase,
        );
        self.retire_unlinked_junctions(
            tick,
            work,
            execution_cost,
            Some(&mut *physical_trace),
            phase,
        );
        self.elapse_activation_to(tick, execution_cost);
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_JUNCTION_CAPACITY, DEFAULT_LINK_CAPACITY)
    }
}

impl Body {
    pub(crate) fn with_capacity(junction_capacity: u32, link_capacity: u32) -> Self {
        Self::from_arena(Arena::new(junction_capacity, link_capacity))
    }

    pub(crate) fn from_arena(arena: Arena) -> Self {
        Self {
            arena,
            pending: Schedule::new(0),
            protocol: Protocol::default(),
            tick: 0,
            next_serial: 0,
            pressure_tick: 0,
            trace_physics: false,
            outcome_source: None,
            output_wave_open: false,
        }
    }
}

impl Body {
    pub(crate) fn set_physical_tracing(&mut self, enabled: bool) {
        self.trace_physics = enabled;
    }

    pub(crate) fn protocol(&self) -> Protocol {
        self.protocol
    }

    pub(crate) fn set_protocol(&mut self, protocol: Protocol) {
        self.protocol = protocol;
    }

    pub(crate) fn clock(&self) -> PhysicalClock {
        PhysicalClock { tick: self.tick }
    }

    pub(crate) fn set_outcome_source(&mut self, source: JunctionId) {
        self.arena.require_junction(source);
        self.outcome_source = Some(source);
    }

    pub(crate) fn return_path_count(&self) -> usize {
        self.arena.return_links(self.outcome_source).len()
    }

    pub(crate) fn add_junction(&mut self, spec: Junction) -> JunctionId {
        self.arena.add_junction(spec, self.tick)
    }

    pub(crate) fn add_link(&mut self, spec: Link) -> LinkId {
        self.arena.add_link(spec)
    }

    pub(crate) fn set_link_trigger(&mut self, id: LinkId, trigger: TransmissionTrigger) {
        let slot = self.arena.link_slot(id).expect("link must resolve");
        self.arena.edit_link(slot.0, |link| link.trigger = trigger);
    }

    pub(crate) fn arrive(&mut self, inputs: &[Input], outward_region: i16) -> RunResult {
        for input in inputs {
            self.enter(*input);
        }
        let mut result = self.propagate();
        result
            .outputs
            .retain(|output| output.to_region == outward_region);
        result
    }

    pub(crate) fn advance_time(&mut self, tick: i64) -> Work {
        assert!(tick >= self.tick, "physical time cannot run backward");
        assert!(
            self.pending.is_empty(),
            "queued activity must propagate first"
        );
        let mut work = Work::default();
        let mut ignored = ExecutionCost::default();
        self.elapse_to(tick, &mut work, &mut ignored);
        self.tick = tick;
        work
    }

    pub(crate) fn working_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            .saturating_add(self.arena.allocated_bytes())
            .saturating_add(self.pending.memory_bytes())
    }
}
