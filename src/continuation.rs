use std::collections::{HashSet, VecDeque};

use crate::binding::{BindingOutcome, IdentitySource, OpaqueId};
use crate::iteration::{frozen_iterable_operation, FrozenIterableOperation};

const LOOKUP_CELL_ID: usize = 1;
const APPLY_CELL_ID: usize = 6;
const CURRENT_CELL_ID: usize = 7;
const RESULT_CELL_ID: usize = 8;
const READ_CELL_ID: usize = 9;
const START_CELL_ID: usize = 10;
const QUIET_CELL_ID: usize = 11;
const NO_RESULT_CELL_ID: usize = 12;
const EXPLICIT_ANSWER_CELL_ID: usize = 13;
const CLEAR_CELL_ID: usize = 14;

const START_TO_APPLY_ARROW_ID: usize = 7;
const APPLY_TO_LOOKUP_ARROW_ID: usize = 8;
const LOOKUP_TO_RESULT_ARROW_ID: usize = 9;
const SELF_ARROW_OFFSET: usize = 10;
const LOOKUP_TO_NO_RESULT_ARROW_ID: usize = 13;
const FINISH_ARROW_OFFSET: usize = 14;

#[derive(Clone, Debug)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.state
    }

    fn shuffle<T>(&mut self, values: &mut [T]) {
        for index in (1..values.len()).rev() {
            let selected = (self.next_u64() as usize) % (index + 1);
            values.swap(index, selected);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SelfRoute {
    ApplyAgain,
    ReadNow,
    BecomeQuiet,
}

#[derive(Clone, Debug)]
struct SelfArrow {
    route: SelfRoute,
    strength: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FinishRoute {
    AnswerCurrent,
    ApplyAgain,
    Clear,
    BecomeQuiet,
}

#[derive(Clone, Debug)]
struct FinishArrow {
    route: FinishRoute,
    strength: i32,
}

#[derive(Clone, Debug)]
struct FinishControl {
    arrows: Vec<FinishArrow>,
    training_examples: u64,
}

impl FinishControl {
    fn new() -> Self {
        Self {
            arrows: [
                FinishRoute::AnswerCurrent,
                FinishRoute::ApplyAgain,
                FinishRoute::Clear,
                FinishRoute::BecomeQuiet,
            ]
            .into_iter()
            .map(|route| FinishArrow { route, strength: 0 })
            .collect(),
            training_examples: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RuntimeCell {
    Collect,
    Finalize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellRef {
    External,
    Permanent(usize),
    Temporary(usize),
    Runtime(RuntimeCell),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SpikePayload {
    None,
    Compare { last: bool },
    Identity(OpaqueId),
}

#[derive(Clone, Copy, Debug)]
struct QueuedSpike {
    from: CellRef,
    to: CellRef,
    arrow_id: Option<usize>,
    payload: SpikePayload,
    external: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ContinuationTrace {
    apply_cell_id: usize,
    lookup_arrow_id: usize,
    feedback_arrow_id: usize,
    self_arrow_id: usize,
    current: OpaqueId,
    internal_spikes_for_step: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SemanticEvent {
    Current(OpaqueId),
    Lookup(OpaqueId),
    NoResult,
    FinishArrow(usize),
    Answer(OpaqueId),
}

#[derive(Clone, Debug)]
struct AutonomousContinuation {
    operation: FrozenIterableOperation,
    self_arrows: Vec<SelfArrow>,
    finish_control: Option<FinishControl>,
    training_examples: u64,
    episode_query: Option<OpaqueId>,
    current: Option<OpaqueId>,
    result: Option<OpaqueId>,
    fault: Option<BindingOutcome>,
    emitted_answer: Option<OpaqueId>,
    finish_answer: Option<OpaqueId>,
    lookup_input: Option<OpaqueId>,
    lookup_outputs: HashSet<OpaqueId>,
    queue: VecDeque<QueuedSpike>,
    trace: Vec<ContinuationTrace>,
    semantic_events: Vec<SemanticEvent>,
    external_start_spikes: usize,
    internal_spikes: usize,
    delivered_spikes: usize,
    apply_activations: usize,
    completed_lookups: usize,
    last_trace_internal_spikes: usize,
    cutoff_reached: bool,
    discarded_at_cutoff: usize,
    activity_limit_hit: bool,
    route_override: Option<SelfRoute>,
    finish_route_override: Option<FinishRoute>,
}

impl AutonomousContinuation {
    fn new(operation: FrozenIterableOperation) -> Self {
        assert_eq!(operation.permanent_cells, 10);
        assert_eq!(operation.permanent_arrows, 7);
        Self {
            operation,
            self_arrows: [
                SelfRoute::ApplyAgain,
                SelfRoute::ReadNow,
                SelfRoute::BecomeQuiet,
            ]
            .into_iter()
            .map(|route| SelfArrow { route, strength: 0 })
            .collect(),
            finish_control: None,
            training_examples: 0,
            episode_query: None,
            current: None,
            result: None,
            fault: None,
            emitted_answer: None,
            finish_answer: None,
            lookup_input: None,
            lookup_outputs: HashSet::new(),
            queue: VecDeque::new(),
            trace: Vec::new(),
            semantic_events: Vec::new(),
            external_start_spikes: 0,
            internal_spikes: 0,
            delivered_spikes: 0,
            apply_activations: 0,
            completed_lookups: 0,
            last_trace_internal_spikes: 0,
            cutoff_reached: false,
            discarded_at_cutoff: 0,
            activity_limit_hit: false,
            route_override: None,
            finish_route_override: None,
        }
    }

    fn with_supplied_route(operation: FrozenIterableOperation, route: SelfRoute) -> Self {
        let mut learner = Self::new(operation);
        for arrow in &mut learner.self_arrows {
            arrow.strength = if arrow.route == route { 1 } else { -1 };
        }
        learner
    }

    fn enable_finish_learning(mut self) -> Self {
        self.finish_control = Some(FinishControl::new());
        self
    }

    fn with_supplied_finish(mut self) -> Self {
        let mut control = FinishControl::new();
        for arrow in &mut control.arrows {
            arrow.strength = if arrow.route == FinishRoute::AnswerCurrent {
                1
            } else {
                -1
            };
        }
        self.finish_control = Some(control);
        self
    }

    fn begin_episode(&mut self, query: OpaqueId) {
        self.erase_temporary();
        self.operation.lookup.begin_episode();
        self.episode_query = Some(query);
        self.current = Some(query);
    }

    fn observe_relation(&mut self, left: OpaqueId, right: OpaqueId) {
        self.operation.lookup.observe_relation(left, right);
    }

    /// The host contributes exactly one external event. All later events are
    /// emitted by permanent or temporary cells through the same queue.
    fn start_and_settle(&mut self, successful_lookup_cutoff: usize) -> RunResult {
        self.run(Some(successful_lookup_cutoff), None, None, 1_000_000)
    }

    fn start_and_finish(&mut self) -> RunResult {
        self.run(None, None, None, 100_000)
    }

    fn start_and_finish_with_limit(&mut self, activity_limit: usize) -> RunResult {
        self.run(None, None, None, activity_limit)
    }

    fn run(
        &mut self,
        successful_lookup_cutoff: Option<usize>,
        route_override: Option<SelfRoute>,
        finish_route_override: Option<FinishRoute>,
        activity_limit: usize,
    ) -> RunResult {
        if let Some(cutoff) = successful_lookup_cutoff {
            assert!(cutoff > 0);
        }
        self.reset_execution();
        self.route_override = route_override;
        self.finish_route_override = finish_route_override;
        self.external_start_spikes = 1;
        self.queue.push_back(QueuedSpike {
            from: CellRef::External,
            to: CellRef::Permanent(START_CELL_ID),
            arrow_id: None,
            payload: SpikePayload::None,
            external: true,
        });

        while let Some(spike) = self.queue.pop_front() {
            self.delivered_spikes += 1;
            self.activate(spike);
            if let Some(cutoff) = successful_lookup_cutoff {
                if self.completed_lookups >= cutoff {
                    self.cutoff_reached = true;
                    self.discarded_at_cutoff = self.queue.len();
                    self.queue = VecDeque::new();
                    break;
                }
            }
            if self.internal_spikes >= activity_limit {
                self.activity_limit_hit = true;
                self.queue = VecDeque::new();
                break;
            }
        }
        self.route_override = None;
        self.finish_route_override = None;
        let outcome = if successful_lookup_cutoff.is_some() {
            self.read_current()
        } else {
            self.explicit_outcome()
        };

        RunResult {
            outcome,
            explicit_answer_emitted: self.finish_answer.is_some(),
            completed_lookups: self.completed_lookups,
            external_start_spikes: self.external_start_spikes,
            internal_spikes: self.internal_spikes,
            delivered_spikes: self.delivered_spikes,
            apply_activations: self.apply_activations,
            cutoff_reached: self.cutoff_reached,
            discarded_at_cutoff: self.discarded_at_cutoff,
            activity_limit_hit: self.activity_limit_hit,
            remaining_queued_spikes: self.queue.len(),
            trace: self.trace.clone(),
            semantic_events: self.semantic_events.clone(),
        }
    }

    fn activate(&mut self, spike: QueuedSpike) {
        debug_assert_eq!(spike.external, matches!(spike.from, CellRef::External));
        match spike.to {
            CellRef::Permanent(START_CELL_ID) => {
                self.enqueue_internal(
                    CellRef::Permanent(START_CELL_ID),
                    CellRef::Permanent(APPLY_CELL_ID),
                    Some(START_TO_APPLY_ARROW_ID),
                    SpikePayload::None,
                );
            }
            CellRef::Permanent(APPLY_CELL_ID) => {
                debug_assert!(
                    spike.arrow_id == Some(START_TO_APPLY_ARROW_ID)
                        || spike.arrow_id == Some(SELF_ARROW_OFFSET)
                        || spike.arrow_id == Some(FINISH_ARROW_OFFSET + 1)
                );
                self.apply_activations += 1;
                self.enqueue_internal(
                    CellRef::Permanent(APPLY_CELL_ID),
                    CellRef::Permanent(LOOKUP_CELL_ID),
                    Some(APPLY_TO_LOOKUP_ARROW_ID),
                    SpikePayload::None,
                );
            }
            CellRef::Permanent(LOOKUP_CELL_ID) => {
                debug_assert_eq!(spike.arrow_id, Some(APPLY_TO_LOOKUP_ARROW_ID));
                self.begin_queued_lookup();
            }
            CellRef::Temporary(cell_id) => self.compare_temporary_cell(cell_id, spike.payload),
            CellRef::Runtime(RuntimeCell::Collect) => {
                let SpikePayload::Identity(identity) = spike.payload else {
                    unreachable!("collect receives an identity")
                };
                self.lookup_outputs.insert(identity);
            }
            CellRef::Runtime(RuntimeCell::Finalize) => self.finalize_lookup(),
            CellRef::Permanent(RESULT_CELL_ID) => {
                debug_assert_eq!(spike.arrow_id, Some(LOOKUP_TO_RESULT_ARROW_ID));
                let SpikePayload::Identity(identity) = spike.payload else {
                    unreachable!("result receives an identity")
                };
                self.result = Some(identity);
                self.enqueue_internal(
                    CellRef::Permanent(RESULT_CELL_ID),
                    CellRef::Permanent(CURRENT_CELL_ID),
                    Some(self.operation.feedback_arrow_id),
                    SpikePayload::Identity(identity),
                );
            }
            CellRef::Permanent(CURRENT_CELL_ID) => {
                debug_assert_eq!(spike.arrow_id, Some(self.operation.feedback_arrow_id));
                let SpikePayload::Identity(identity) = spike.payload else {
                    unreachable!("current receives an identity")
                };
                self.activate_current(identity);
            }
            CellRef::Permanent(NO_RESULT_CELL_ID) => {
                debug_assert_eq!(spike.arrow_id, Some(LOOKUP_TO_NO_RESULT_ARROW_ID));
                self.activate_no_result();
            }
            CellRef::Permanent(EXPLICIT_ANSWER_CELL_ID) => {
                debug_assert_eq!(spike.arrow_id, Some(FINISH_ARROW_OFFSET));
                let SpikePayload::Identity(identity) = spike.payload else {
                    unreachable!("explicit answer receives an identity")
                };
                self.emitted_answer = Some(identity);
                self.finish_answer = Some(identity);
                self.semantic_events.push(SemanticEvent::Answer(identity));
            }
            CellRef::Permanent(CLEAR_CELL_ID) => {
                self.current = None;
            }
            CellRef::Permanent(READ_CELL_ID) => {
                self.emitted_answer = self.current;
            }
            CellRef::Permanent(QUIET_CELL_ID) => {}
            CellRef::Permanent(other) => panic!("unknown permanent cell {other}"),
            CellRef::External => unreachable!("external is never a spike destination"),
        }
    }

    fn begin_queued_lookup(&mut self) {
        let Some(current) = self.current else {
            self.fault = Some(BindingOutcome::NotFound);
            return;
        };
        self.lookup_input = Some(current);
        self.semantic_events.push(SemanticEvent::Lookup(current));
        self.lookup_outputs.clear();
        let identity_cells = self.operation.lookup.temporary_identity_cells();
        if identity_cells.is_empty() {
            self.enqueue_internal(
                CellRef::Permanent(LOOKUP_CELL_ID),
                CellRef::Runtime(RuntimeCell::Finalize),
                None,
                SpikePayload::None,
            );
            return;
        }
        let last_index = identity_cells.len() - 1;
        for (index, cell_id) in identity_cells.into_iter().enumerate() {
            self.enqueue_internal(
                CellRef::Permanent(LOOKUP_CELL_ID),
                CellRef::Temporary(cell_id),
                None,
                SpikePayload::Compare {
                    last: index == last_index,
                },
            );
        }
    }

    fn compare_temporary_cell(&mut self, cell_id: usize, payload: SpikePayload) {
        let SpikePayload::Compare { last } = payload else {
            unreachable!("temporary identity receives a comparison")
        };
        let query = self.lookup_input.expect("lookup has a current identity");
        if let Some(output) = self
            .operation
            .lookup
            .compare_temporary_identity(query, cell_id)
        {
            self.enqueue_internal(
                CellRef::Temporary(cell_id),
                CellRef::Runtime(RuntimeCell::Collect),
                self.operation.lookup.selected_route_id(),
                SpikePayload::Identity(output),
            );
        }
        if last {
            self.enqueue_internal(
                CellRef::Temporary(cell_id),
                CellRef::Runtime(RuntimeCell::Finalize),
                None,
                SpikePayload::None,
            );
        }
    }

    fn finalize_lookup(&mut self) {
        match self.lookup_outputs.len() {
            0 if self.finish_control.is_some() => self.enqueue_internal(
                CellRef::Permanent(LOOKUP_CELL_ID),
                CellRef::Permanent(NO_RESULT_CELL_ID),
                Some(LOOKUP_TO_NO_RESULT_ARROW_ID),
                SpikePayload::None,
            ),
            0 => self.fault = Some(BindingOutcome::NotFound),
            1 => {
                let result = *self.lookup_outputs.iter().next().unwrap();
                self.enqueue_internal(
                    CellRef::Permanent(LOOKUP_CELL_ID),
                    CellRef::Permanent(RESULT_CELL_ID),
                    Some(LOOKUP_TO_RESULT_ARROW_ID),
                    SpikePayload::Identity(result),
                );
            }
            _ => self.fault = Some(BindingOutcome::Ambiguous),
        }
    }

    fn activate_current(&mut self, identity: OpaqueId) {
        self.current = Some(identity);
        self.semantic_events.push(SemanticEvent::Current(identity));
        self.completed_lookups += 1;
        let (self_arrow_id, route) = self.selected_or_overridden_route();
        match route {
            Some(SelfRoute::ApplyAgain) => self.enqueue_internal(
                CellRef::Permanent(CURRENT_CELL_ID),
                CellRef::Permanent(APPLY_CELL_ID),
                self_arrow_id,
                SpikePayload::None,
            ),
            Some(SelfRoute::ReadNow) => self.enqueue_internal(
                CellRef::Permanent(CURRENT_CELL_ID),
                CellRef::Permanent(READ_CELL_ID),
                self_arrow_id,
                SpikePayload::None,
            ),
            Some(SelfRoute::BecomeQuiet) => self.enqueue_internal(
                CellRef::Permanent(CURRENT_CELL_ID),
                CellRef::Permanent(QUIET_CELL_ID),
                self_arrow_id,
                SpikePayload::None,
            ),
            None => {}
        }

        self.trace.push(ContinuationTrace {
            apply_cell_id: APPLY_CELL_ID,
            lookup_arrow_id: self
                .operation
                .lookup
                .selected_route_id()
                .expect("frozen v19 lookup remains selected"),
            feedback_arrow_id: self.operation.feedback_arrow_id,
            self_arrow_id: self_arrow_id.unwrap_or(usize::MAX),
            current: identity,
            internal_spikes_for_step: self.internal_spikes - self.last_trace_internal_spikes,
        });
        self.last_trace_internal_spikes = self.internal_spikes;
    }

    fn activate_no_result(&mut self) {
        self.semantic_events.push(SemanticEvent::NoResult);
        let (arrow_id, route) = self.selected_or_overridden_finish_route();
        let Some(route) = route else {
            return;
        };
        let arrow_id = arrow_id.expect("selected finish route has an arrow");
        self.semantic_events
            .push(SemanticEvent::FinishArrow(arrow_id));
        match route {
            FinishRoute::AnswerCurrent => {
                let Some(current) = self.current else {
                    return;
                };
                self.enqueue_internal(
                    CellRef::Permanent(NO_RESULT_CELL_ID),
                    CellRef::Permanent(EXPLICIT_ANSWER_CELL_ID),
                    Some(arrow_id),
                    SpikePayload::Identity(current),
                );
            }
            FinishRoute::ApplyAgain => self.enqueue_internal(
                CellRef::Permanent(NO_RESULT_CELL_ID),
                CellRef::Permanent(APPLY_CELL_ID),
                Some(arrow_id),
                SpikePayload::None,
            ),
            FinishRoute::Clear => self.enqueue_internal(
                CellRef::Permanent(NO_RESULT_CELL_ID),
                CellRef::Permanent(CLEAR_CELL_ID),
                Some(arrow_id),
                SpikePayload::None,
            ),
            FinishRoute::BecomeQuiet => self.enqueue_internal(
                CellRef::Permanent(NO_RESULT_CELL_ID),
                CellRef::Permanent(QUIET_CELL_ID),
                Some(arrow_id),
                SpikePayload::None,
            ),
        }
    }

    fn selected_or_overridden_route(&self) -> (Option<usize>, Option<SelfRoute>) {
        if let Some(route) = self.route_override {
            let index = self
                .self_arrows
                .iter()
                .position(|arrow| arrow.route == route)
                .unwrap();
            return (Some(SELF_ARROW_OFFSET + index), Some(route));
        }
        self.selected_self_route()
            .map_or((None, None), |(index, route)| {
                (Some(SELF_ARROW_OFFSET + index), Some(route))
            })
    }

    fn selected_self_route(&self) -> Option<(usize, SelfRoute)> {
        let mut best: Option<(usize, &SelfArrow)> = None;
        let mut tied = false;
        for (index, arrow) in self.self_arrows.iter().enumerate() {
            match best {
                None => {
                    best = Some((index, arrow));
                    tied = false;
                }
                Some((_, current)) if arrow.strength > current.strength => {
                    best = Some((index, arrow));
                    tied = false;
                }
                Some((_, current)) if arrow.strength == current.strength => tied = true,
                Some(_) => {}
            }
        }
        if tied {
            None
        } else {
            best.filter(|(_, arrow)| arrow.strength > 0)
                .map(|(index, arrow)| (index, arrow.route))
        }
    }

    fn selected_or_overridden_finish_route(&self) -> (Option<usize>, Option<FinishRoute>) {
        if let Some(route) = self.finish_route_override {
            let index = self
                .finish_control
                .as_ref()
                .expect("finish override requires finish control")
                .arrows
                .iter()
                .position(|arrow| arrow.route == route)
                .unwrap();
            return (Some(FINISH_ARROW_OFFSET + index), Some(route));
        }
        self.selected_finish_route()
            .map_or((None, None), |(index, route)| {
                (Some(FINISH_ARROW_OFFSET + index), Some(route))
            })
    }

    fn selected_finish_route(&self) -> Option<(usize, FinishRoute)> {
        let control = self.finish_control.as_ref()?;
        let mut best: Option<(usize, &FinishArrow)> = None;
        let mut tied = false;
        for (index, arrow) in control.arrows.iter().enumerate() {
            match best {
                None => {
                    best = Some((index, arrow));
                    tied = false;
                }
                Some((_, current)) if arrow.strength > current.strength => {
                    best = Some((index, arrow));
                    tied = false;
                }
                Some((_, current)) if arrow.strength == current.strength => tied = true,
                Some(_) => {}
            }
        }
        if tied {
            None
        } else {
            best.filter(|(_, arrow)| arrow.strength > 0)
                .map(|(index, arrow)| (index, arrow.route))
        }
    }

    /// Supervision supplies only the expected identity after two successful
    /// lookups. Each candidate is evaluated by one external start and the
    /// queued runtime, never by repeated host calls to lookup.
    fn learn_from_terminal(&mut self, correct: BindingOutcome, cutoff: usize) {
        let outcomes: Vec<_> = self
            .self_arrows
            .iter()
            .map(|arrow| {
                let mut candidate = self.clone();
                candidate
                    .run(Some(cutoff), Some(arrow.route), None, 1_000_000)
                    .outcome
            })
            .collect();
        for (arrow, outcome) in self.self_arrows.iter_mut().zip(outcomes) {
            if outcome == correct {
                arrow.strength = arrow.strength.saturating_add(1);
            } else {
                arrow.strength = arrow.strength.saturating_sub(1);
            }
        }
        self.training_examples += 1;
    }

    fn learn_finish_from_terminal(&mut self, correct: BindingOutcome) {
        let routes: Vec<_> = self
            .finish_control
            .as_ref()
            .expect("finish learning requires finish candidates")
            .arrows
            .iter()
            .map(|arrow| arrow.route)
            .collect();
        let outcomes: Vec<_> = routes
            .iter()
            .map(|&route| {
                let mut candidate = self.clone();
                candidate.run(None, None, Some(route), 512).outcome
            })
            .collect();
        let control = self.finish_control.as_mut().unwrap();
        for (arrow, outcome) in control.arrows.iter_mut().zip(outcomes) {
            if outcome == correct {
                arrow.strength = arrow.strength.saturating_add(1);
            } else {
                arrow.strength = arrow.strength.saturating_sub(1);
            }
        }
        control.training_examples += 1;
    }

    fn enqueue_internal(
        &mut self,
        from: CellRef,
        to: CellRef,
        arrow_id: Option<usize>,
        payload: SpikePayload,
    ) {
        self.internal_spikes += 1;
        self.queue.push_back(QueuedSpike {
            from,
            to,
            arrow_id,
            payload,
            external: false,
        });
    }

    fn read_current(&self) -> BindingOutcome {
        if let Some(fault) = self.fault {
            return fault;
        }
        self.emitted_answer
            .or(self.current)
            .map_or(BindingOutcome::NotFound, BindingOutcome::Answer)
    }

    fn explicit_outcome(&self) -> BindingOutcome {
        if self.fault == Some(BindingOutcome::Ambiguous) {
            return BindingOutcome::Ambiguous;
        }
        self.finish_answer
            .map_or(BindingOutcome::NotFound, BindingOutcome::Answer)
    }

    fn reset_execution(&mut self) {
        self.current = self.episode_query;
        self.result = None;
        self.fault = None;
        self.emitted_answer = None;
        self.finish_answer = None;
        self.lookup_input = None;
        self.lookup_outputs = HashSet::new();
        self.queue = VecDeque::new();
        self.trace = Vec::new();
        self.semantic_events = Vec::new();
        self.external_start_spikes = 0;
        self.internal_spikes = 0;
        self.delivered_spikes = 0;
        self.apply_activations = 0;
        self.completed_lookups = 0;
        self.last_trace_internal_spikes = 0;
        self.cutoff_reached = false;
        self.discarded_at_cutoff = 0;
        self.activity_limit_hit = false;
        self.route_override = None;
        self.finish_route_override = None;
    }

    fn erase_temporary(&mut self) {
        self.operation.lookup.erase_temporary();
        self.episode_query = None;
        self.current = None;
        self.result = None;
        self.fault = None;
        self.emitted_answer = None;
        self.finish_answer = None;
        self.lookup_input = None;
        self.lookup_outputs = HashSet::new();
        self.queue = VecDeque::new();
        self.trace = Vec::new();
        self.semantic_events = Vec::new();
        self.route_override = None;
        self.finish_route_override = None;
    }

    fn temporary_counts(&self) -> (usize, usize) {
        let (lookup_cells, lookup_arrows) = self.operation.lookup.temporary_counts();
        let working_cells = if self.episode_query.is_some() {
            2 + usize::from(self.current.is_some()) + usize::from(self.result.is_some())
        } else {
            0
        };
        (lookup_cells + working_cells, lookup_arrows)
    }

    fn temporary_capacities(&self) -> (usize, usize, usize, usize, usize, usize, usize) {
        let (cells, arrows, relations) = self.operation.lookup.temporary_capacities();
        (
            cells,
            arrows,
            relations,
            self.lookup_outputs.capacity(),
            self.queue.capacity(),
            self.trace.capacity(),
            self.semantic_events.capacity(),
        )
    }

    fn permanent_counts(&self) -> (usize, usize) {
        let base = (
            self.operation.permanent_cells + 2,
            self.operation.permanent_arrows + 3 + self.self_arrows.len(),
        );
        if let Some(control) = &self.finish_control {
            (base.0 + 3, base.1 + 1 + control.arrows.len())
        } else {
            base
        }
    }

    fn continuation_fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        fingerprint_mix(&mut hash, self.operation.permanent_fingerprint);
        fingerprint_mix(&mut hash, self.training_examples);
        for cell in [
            START_CELL_ID,
            QUIET_CELL_ID,
            APPLY_CELL_ID,
            CURRENT_CELL_ID,
            RESULT_CELL_ID,
            READ_CELL_ID,
        ] {
            fingerprint_mix(&mut hash, cell as u64);
        }
        for arrow in [
            START_TO_APPLY_ARROW_ID,
            APPLY_TO_LOOKUP_ARROW_ID,
            LOOKUP_TO_RESULT_ARROW_ID,
        ] {
            fingerprint_mix(&mut hash, arrow as u64);
        }
        for arrow in &self.self_arrows {
            fingerprint_mix(&mut hash, arrow.route as u64);
            fingerprint_mix(&mut hash, arrow.strength as i64 as u64);
        }
        hash
    }

    fn permanent_fingerprint(&self) -> u64 {
        let mut hash = self.continuation_fingerprint();
        if let Some(control) = &self.finish_control {
            fingerprint_mix(&mut hash, control.training_examples);
            for cell in [NO_RESULT_CELL_ID, EXPLICIT_ANSWER_CELL_ID, CLEAR_CELL_ID] {
                fingerprint_mix(&mut hash, cell as u64);
            }
            fingerprint_mix(&mut hash, LOOKUP_TO_NO_RESULT_ARROW_ID as u64);
            for arrow in &control.arrows {
                fingerprint_mix(&mut hash, arrow.route as u64);
                fingerprint_mix(&mut hash, arrow.strength as i64 as u64);
            }
        }
        hash
    }

    fn route_strengths(&self) -> Vec<(String, i32)> {
        self.self_arrows
            .iter()
            .map(|arrow| (format!("{:?}", arrow.route), arrow.strength))
            .collect()
    }

    fn finish_route_strengths(&self) -> Vec<(String, i32)> {
        self.finish_control
            .as_ref()
            .map(|control| {
                control
                    .arrows
                    .iter()
                    .map(|arrow| (format!("{:?}", arrow.route), arrow.strength))
                    .collect()
            })
            .unwrap_or_default()
    }
}

fn fingerprint_mix(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(0x100_0000_01b3);
    }
}

#[derive(Clone, Debug)]
struct ContinuationEpisode {
    relations: Vec<(OpaqueId, OpaqueId)>,
    chain: Vec<OpaqueId>,
    query: OpaqueId,
    distinct_identities: usize,
}

impl ContinuationEpisode {
    fn answer_after(&self, successful_lookups: usize) -> BindingOutcome {
        BindingOutcome::Answer(self.chain[successful_lookups])
    }
}

fn chain_episode(
    identities: &mut IdentitySource,
    rng: &mut DeterministicRng,
    chain_depth: usize,
    relation_count: usize,
) -> ContinuationEpisode {
    assert!(chain_depth > 0);
    assert!(relation_count >= chain_depth);
    let chain: Vec<_> = (0..=chain_depth).map(|_| identities.issue()).collect();
    let mut relations: Vec<_> = chain.windows(2).map(|pair| (pair[0], pair[1])).collect();
    for _ in chain_depth..relation_count {
        relations.push((identities.issue(), identities.issue()));
    }
    rng.shuffle(&mut relations);
    ContinuationEpisode {
        relations,
        query: chain[0],
        chain,
        distinct_identities: chain_depth + 1 + (relation_count - chain_depth) * 2,
    }
}

fn present(learner: &mut AutonomousContinuation, episode: &ContinuationEpisode) {
    learner.begin_episode(episode.query);
    for &(left, right) in &episode.relations {
        learner.observe_relation(left, right);
    }
}

fn frozen_autonomous_continuation() -> AutonomousContinuation {
    let mut learner = AutonomousContinuation::new(frozen_iterable_operation());
    let mut identities = IdentitySource::new(0x210a_f001);
    let mut rng = DeterministicRng::new(0x210a_f002);
    for _ in 0..32 {
        let episode = chain_episode(&mut identities, &mut rng, 2, 10);
        present(&mut learner, &episode);
        let _proposal = learner.start_and_settle(2);
        learner.learn_from_terminal(episode.answer_after(2), 2);
        learner.erase_temporary();
    }
    let (_, route) = learner
        .selected_self_route()
        .expect("v21a self-trigger must be learned before freezing");
    assert_eq!(route, SelfRoute::ApplyAgain);
    learner
}

fn v18_style_episode(
    identities: &mut IdentitySource,
    rng: &mut DeterministicRng,
    depth: usize,
) -> ContinuationEpisode {
    let chain: Vec<_> = (0..=depth).map(|_| identities.issue()).collect();
    let distractor = [identities.issue(), identities.issue(), identities.issue()];
    let mut relations: Vec<_> = chain.windows(2).map(|pair| (pair[0], pair[1])).collect();
    relations.extend(distractor.windows(2).map(|pair| (pair[0], pair[1])));
    rng.shuffle(&mut relations);
    ContinuationEpisode {
        relations,
        query: chain[0],
        chain,
        distinct_identities: depth + 4,
    }
}

#[derive(Clone, Debug)]
struct RunResult {
    outcome: BindingOutcome,
    explicit_answer_emitted: bool,
    completed_lookups: usize,
    external_start_spikes: usize,
    internal_spikes: usize,
    delivered_spikes: usize,
    apply_activations: usize,
    cutoff_reached: bool,
    discarded_at_cutoff: usize,
    activity_limit_hit: bool,
    remaining_queued_spikes: usize,
    trace: Vec<ContinuationTrace>,
    semantic_events: Vec<SemanticEvent>,
}

fn evaluate(
    learner: &mut AutonomousContinuation,
    episode: &ContinuationEpisode,
    cutoff: usize,
) -> (bool, RunResult, usize, usize) {
    present(learner, episode);
    let before = learner.temporary_counts();
    let run = learner.start_and_settle(cutoff);
    let after = learner.temporary_counts();
    let peak_cells = before.0.max(after.0);
    let peak_arrows = before.1.max(after.1);
    let correct = run.outcome == episode.answer_after(cutoff);
    learner.erase_temporary();
    assert_eq!(learner.temporary_counts(), (0, 0));
    assert_eq!(learner.temporary_capacities(), (0, 0, 0, 0, 0, 0, 0));
    (correct, run, peak_cells, peak_arrows)
}

#[derive(Clone, Debug)]
pub struct ContinuationCheckpoint {
    pub training_episodes: usize,
    pub permanent_cells: usize,
    pub permanent_arrows: usize,
    pub validation_correct: usize,
    pub validation_total: usize,
}

#[derive(Clone, Debug)]
pub struct ContinuationDepth {
    pub cutoff: usize,
    pub learner_correct: usize,
    pub supplied_self_trigger_correct: usize,
    pub no_self_trigger_correct: usize,
    pub total: usize,
    pub average_internal_spikes: f64,
    pub average_delivered_spikes: f64,
    pub average_apply_activations: f64,
}

#[derive(Clone, Debug)]
pub struct ContinuationReport {
    pub checkpoints: Vec<ContinuationCheckpoint>,
    pub depth_results: Vec<ContinuationDepth>,
    pub permanent_cells: usize,
    pub permanent_arrows: usize,
    pub route_strengths: Vec<(String, i32)>,
    pub held_out_distinct_identities: usize,
    pub external_start_is_always_one: bool,
    pub trace_reuses_apply_cell: bool,
    pub trace_reuses_lookup_arrow: bool,
    pub trace_reuses_feedback_arrow: bool,
    pub trace_reuses_self_arrow: bool,
    pub trace_apply_cell_id: usize,
    pub trace_lookup_arrow_id: usize,
    pub trace_feedback_arrow_id: usize,
    pub trace_self_arrow_id: usize,
    pub trace_current_is_correct: bool,
    pub trace_step_work_consistent: bool,
    pub apply_activations_match_cutoff: bool,
    pub cutoff_discards_one_self_trigger: bool,
    pub internal_work_is_linear: bool,
    pub branch_is_ambiguous: bool,
    pub duplicate_is_single_result: bool,
    pub peak_temporary_cells: usize,
    pub peak_temporary_arrows: usize,
    pub residual_temporary_cells: usize,
    pub residual_temporary_arrows: usize,
    pub temporary_capacity_released: bool,
    pub permanent_fingerprint_unchanged: bool,
    pub activity_limit_hits: usize,
    pub passed: bool,
}

fn control_episodes(identities: &mut IdentitySource) -> (ContinuationEpisode, ContinuationEpisode) {
    let a = identities.issue();
    let b = identities.issue();
    let c = identities.issue();
    let d = identities.issue();
    let branch = ContinuationEpisode {
        relations: vec![(a, b), (b, c), (b, d)],
        chain: vec![a, b, c],
        query: a,
        distinct_identities: 4,
    };
    let duplicate = ContinuationEpisode {
        relations: vec![(a, b), (b, c), (b, c), (d, identities.issue())],
        chain: vec![a, b, c],
        query: a,
        distinct_identities: 5,
    };
    (branch, duplicate)
}

pub fn run_experiment() -> ContinuationReport {
    let frozen_operation = frozen_iterable_operation();
    let mut learner = AutonomousContinuation::new(frozen_operation.clone());
    let mut training_ids = IdentitySource::new(0x210a_0001);
    let mut training_rng = DeterministicRng::new(0x210a_0002);
    let mut validation_ids = IdentitySource::new(0x210a_1001);
    let mut validation_rng = DeterministicRng::new(0x210a_1002);
    let checkpoints_at = [10, 100, 1_000];
    let mut checkpoints = Vec::new();

    for episode_index in 1..=1_000 {
        let episode = chain_episode(&mut training_ids, &mut training_rng, 2, 10);
        present(&mut learner, &episode);
        let _proposal = learner.start_and_settle(2);
        learner.learn_from_terminal(episode.answer_after(2), 2);
        learner.erase_temporary();

        if checkpoints_at.contains(&episode_index) {
            let fingerprint = learner.permanent_fingerprint();
            let correct = (0..100)
                .filter(|_| {
                    let episode = chain_episode(&mut validation_ids, &mut validation_rng, 4, 10);
                    evaluate(&mut learner, &episode, 2).0
                })
                .count();
            assert_eq!(fingerprint, learner.permanent_fingerprint());
            let (permanent_cells, permanent_arrows) = learner.permanent_counts();
            checkpoints.push(ContinuationCheckpoint {
                training_episodes: episode_index,
                permanent_cells,
                permanent_arrows,
                validation_correct: correct,
                validation_total: 100,
            });
        }
    }

    let fingerprint_before = learner.permanent_fingerprint();
    let mut supplied = AutonomousContinuation::with_supplied_route(
        frozen_operation.clone(),
        SelfRoute::ApplyAgain,
    );
    let mut no_self =
        AutonomousContinuation::with_supplied_route(frozen_operation, SelfRoute::BecomeQuiet);
    let mut held_out_ids = IdentitySource::new(0x210a_2001);
    let mut held_out_rng = DeterministicRng::new(0x210a_2002);
    let mut depth_results = Vec::new();
    let mut held_out_distinct_identities = 0;
    let mut peak_temporary_cells = 0;
    let mut peak_temporary_arrows = 0;
    let mut activity_limit_hits = 0;
    let mut all_runs = Vec::new();
    let mut representative_episode = None;
    let mut representative_trace = Vec::new();

    for cutoff in [1, 2, 4, 8, 16, 32] {
        let total = 200;
        let mut learner_correct = 0;
        let mut supplied_self_trigger_correct = 0;
        let mut no_self_trigger_correct = 0;
        let mut internal_spikes = 0;
        let mut delivered_spikes = 0;
        let mut apply_activations = 0;

        for episode_index in 0..total {
            // All held-out depths use the same episode size. This isolates
            // execution depth from the amount of temporary relation memory.
            let episode = chain_episode(&mut held_out_ids, &mut held_out_rng, 40, 48);
            held_out_distinct_identities += episode.distinct_identities;
            let (correct, run, cells, arrows) = evaluate(&mut learner, &episode, cutoff);
            learner_correct += usize::from(correct);
            peak_temporary_cells = peak_temporary_cells.max(cells);
            peak_temporary_arrows = peak_temporary_arrows.max(arrows);
            internal_spikes += run.internal_spikes;
            delivered_spikes += run.delivered_spikes;
            apply_activations += run.apply_activations;
            activity_limit_hits += usize::from(run.activity_limit_hit);
            all_runs.push(run.clone());

            supplied_self_trigger_correct +=
                usize::from(evaluate(&mut supplied, &episode, cutoff).0);
            no_self_trigger_correct += usize::from(evaluate(&mut no_self, &episode, cutoff).0);

            if cutoff == 32 && episode_index == 0 {
                representative_episode = Some(episode);
                representative_trace = run.trace;
            }
        }

        depth_results.push(ContinuationDepth {
            cutoff,
            learner_correct,
            supplied_self_trigger_correct,
            no_self_trigger_correct,
            total,
            average_internal_spikes: internal_spikes as f64 / total as f64,
            average_delivered_spikes: delivered_spikes as f64 / total as f64,
            average_apply_activations: apply_activations as f64 / total as f64,
        });
    }

    let representative_episode = representative_episode.unwrap();
    let lookup_arrow_id = representative_trace[0].lookup_arrow_id;
    let feedback_arrow_id = representative_trace[0].feedback_arrow_id;
    let self_arrow_id = representative_trace[0].self_arrow_id;
    let trace_reuses_apply_cell = representative_trace
        .iter()
        .all(|step| step.apply_cell_id == APPLY_CELL_ID);
    let trace_reuses_lookup_arrow = representative_trace
        .iter()
        .all(|step| step.lookup_arrow_id == lookup_arrow_id);
    let trace_reuses_feedback_arrow = representative_trace
        .iter()
        .all(|step| step.feedback_arrow_id == feedback_arrow_id);
    let trace_reuses_self_arrow = representative_trace
        .iter()
        .all(|step| step.self_arrow_id == self_arrow_id);
    let trace_current_is_correct = representative_trace
        .iter()
        .enumerate()
        .all(|(index, step)| step.current == representative_episode.chain[index + 1]);
    let trace_step_work_consistent = representative_trace[0].internal_spikes_for_step
        == representative_trace[1].internal_spikes_for_step + 1
        && representative_trace[1..]
            .windows(2)
            .all(|pair| pair[0].internal_spikes_for_step == pair[1].internal_spikes_for_step);
    let external_start_is_always_one = all_runs.iter().all(|run| run.external_start_spikes == 1);
    let apply_activations_match_cutoff = all_runs
        .iter()
        .all(|run| run.apply_activations == run.completed_lookups);
    let cutoff_discards_one_self_trigger = all_runs.iter().all(|run| run.discarded_at_cutoff == 1);
    let internal_work_is_linear = depth_results.windows(2).all(|pair| {
        let first = &pair[0];
        let second = &pair[1];
        let per_step_first = (first.average_internal_spikes - 1.0) / first.cutoff as f64;
        let per_step_second = (second.average_internal_spikes - 1.0) / second.cutoff as f64;
        (per_step_first - per_step_second).abs() < f64::EPSILON
    });

    let mut control_ids = IdentitySource::new(0x210a_3001);
    let (branch, duplicate) = control_episodes(&mut control_ids);
    present(&mut learner, &branch);
    let branch_run = learner.start_and_settle(2);
    let branch_is_ambiguous = branch_run.outcome == BindingOutcome::Ambiguous;
    learner.erase_temporary();
    present(&mut learner, &duplicate);
    let duplicate_run = learner.start_and_settle(2);
    let duplicate_is_single_result = duplicate_run.outcome == duplicate.answer_after(2);
    learner.erase_temporary();

    let (residual_temporary_cells, residual_temporary_arrows) = learner.temporary_counts();
    let temporary_capacity_released = learner.temporary_capacities() == (0, 0, 0, 0, 0, 0, 0);
    let permanent_fingerprint_unchanged = fingerprint_before == learner.permanent_fingerprint();
    let (permanent_cells, permanent_arrows) = learner.permanent_counts();
    let route_strengths = learner.route_strengths();
    let checkpoint_accuracy = checkpoints
        .iter()
        .all(|point| point.validation_correct == point.validation_total);
    let structure_plateaued = checkpoints.windows(2).all(|pair| {
        pair[0].permanent_cells == pair[1].permanent_cells
            && pair[0].permanent_arrows == pair[1].permanent_arrows
    });
    let learner_all_depths = depth_results
        .iter()
        .all(|point| point.learner_correct == point.total);
    let supplied_all_depths = depth_results
        .iter()
        .all(|point| point.supplied_self_trigger_correct == point.total);
    let no_self_signature = depth_results.iter().all(|point| {
        if point.cutoff == 1 {
            point.no_self_trigger_correct == point.total
        } else {
            point.no_self_trigger_correct == 0
        }
    });
    let every_cutoff_reached = all_runs
        .iter()
        .all(|run| run.cutoff_reached && run.remaining_queued_spikes == 0);
    let passed = checkpoint_accuracy
        && structure_plateaued
        && learner_all_depths
        && supplied_all_depths
        && no_self_signature
        && external_start_is_always_one
        && every_cutoff_reached
        && trace_reuses_apply_cell
        && trace_reuses_lookup_arrow
        && trace_reuses_feedback_arrow
        && trace_reuses_self_arrow
        && trace_current_is_correct
        && trace_step_work_consistent
        && apply_activations_match_cutoff
        && cutoff_discards_one_self_trigger
        && internal_work_is_linear
        && branch_is_ambiguous
        && duplicate_is_single_result
        && residual_temporary_cells == 0
        && residual_temporary_arrows == 0
        && temporary_capacity_released
        && permanent_fingerprint_unchanged
        && activity_limit_hits == 0;

    ContinuationReport {
        checkpoints,
        depth_results,
        permanent_cells,
        permanent_arrows,
        route_strengths,
        held_out_distinct_identities,
        external_start_is_always_one,
        trace_reuses_apply_cell,
        trace_reuses_lookup_arrow,
        trace_reuses_feedback_arrow,
        trace_reuses_self_arrow,
        trace_apply_cell_id: APPLY_CELL_ID,
        trace_lookup_arrow_id: lookup_arrow_id,
        trace_feedback_arrow_id: feedback_arrow_id,
        trace_self_arrow_id: self_arrow_id,
        trace_current_is_correct,
        trace_step_work_consistent,
        apply_activations_match_cutoff,
        cutoff_discards_one_self_trigger,
        internal_work_is_linear,
        branch_is_ambiguous,
        duplicate_is_single_result,
        peak_temporary_cells,
        peak_temporary_arrows,
        residual_temporary_cells,
        residual_temporary_arrows,
        temporary_capacity_released,
        permanent_fingerprint_unchanged,
        activity_limit_hits,
        passed,
    }
}

pub fn print_report(report: &ContinuationReport) {
    println!("v21a autonomous continuation:");
    print!("  checkpoints episodes:cells/arrows/accuracy:");
    for checkpoint in &report.checkpoints {
        print!(
            " {}:{}/{}/{}/{}",
            checkpoint.training_episodes,
            checkpoint.permanent_cells,
            checkpoint.permanent_arrows,
            checkpoint.validation_correct,
            checkpoint.validation_total
        );
    }
    println!();
    for depth in &report.depth_results {
        println!(
            "  cutoff {} learner/supplied/no-self={}/{}, {}/{}, {}/{}, internal spikes={:.1}, apply activations={:.1}",
            depth.cutoff,
            depth.learner_correct,
            depth.total,
            depth.supplied_self_trigger_correct,
            depth.total,
            depth.no_self_trigger_correct,
            depth.total,
            depth.average_internal_spikes,
            depth.average_apply_activations
        );
    }
    println!(
        "  one external start={}, route ids apply/lookup/feedback/self={}/{}/{}/{}, reused={}/{}/{}/{}, current trace={}, cutoff discards one self-trigger={}, linear work={}",
        report.external_start_is_always_one,
        report.trace_apply_cell_id,
        report.trace_lookup_arrow_id,
        report.trace_feedback_arrow_id,
        report.trace_self_arrow_id,
        report.trace_reuses_apply_cell,
        report.trace_reuses_lookup_arrow,
        report.trace_reuses_feedback_arrow,
        report.trace_reuses_self_arrow,
        report.trace_current_is_correct,
        report.cutoff_discards_one_self_trigger,
        report.internal_work_is_linear
    );
    println!(
        "  permanent cells/arrows={}/{}, held-out identities={}, branch ambiguous={}, duplicate single={}, activity-limit hits={}",
        report.permanent_cells,
        report.permanent_arrows,
        report.held_out_distinct_identities,
        report.branch_is_ambiguous,
        report.duplicate_is_single_result,
        report.activity_limit_hits
    );
}

fn evaluate_finish(
    learner: &mut AutonomousContinuation,
    episode: &ContinuationEpisode,
) -> (bool, RunResult, usize, usize) {
    present(learner, episode);
    let before = learner.temporary_counts();
    let run = learner.start_and_finish();
    let after = learner.temporary_counts();
    let peak_cells = before.0.max(after.0);
    let peak_arrows = before.1.max(after.1);
    let expected = episode.answer_after(episode.chain.len() - 1);
    let correct = run.explicit_answer_emitted && run.outcome == expected;
    learner.erase_temporary();
    assert_eq!(learner.temporary_counts(), (0, 0));
    assert_eq!(learner.temporary_capacities(), (0, 0, 0, 0, 0, 0, 0));
    (correct, run, peak_cells, peak_arrows)
}

#[derive(Clone, Debug)]
pub struct FinishCheckpoint {
    pub training_episodes: usize,
    pub permanent_cells: usize,
    pub permanent_arrows: usize,
    pub validation_correct: usize,
    pub validation_total: usize,
}

#[derive(Clone, Debug)]
pub struct FinishDepth {
    pub depth: usize,
    pub learner_correct: usize,
    pub supplied_finish_correct: usize,
    pub no_finish_correct: usize,
    pub total: usize,
    pub average_internal_spikes: f64,
    pub average_apply_activations: f64,
}

#[derive(Clone, Debug)]
pub struct WorkingSetPoint {
    pub relations: usize,
    pub average_internal_spikes: f64,
}

#[derive(Clone, Debug)]
pub struct FinishReport {
    pub checkpoints: Vec<FinishCheckpoint>,
    pub depth_results: Vec<FinishDepth>,
    pub working_set: Vec<WorkingSetPoint>,
    pub permanent_cells: usize,
    pub permanent_arrows: usize,
    pub finish_route_strengths: Vec<(String, i32)>,
    pub held_out_distinct_identities: usize,
    pub explicit_answer_only: bool,
    pub natural_queue_empty: bool,
    pub no_cutoff_used: bool,
    pub success_path_reuses_v21a_routes: bool,
    pub final_trace_is_exact: bool,
    pub finish_arrow_id: usize,
    pub depth_work_is_linear: bool,
    pub working_set_work_is_linear: bool,
    pub branch_is_ambiguous_without_answer: bool,
    pub duplicate_is_single_answer: bool,
    pub zero_hop_answers_query: bool,
    pub cycle_emits_no_answer: bool,
    pub cycle_hits_safety_limit: bool,
    pub v18_distribution_correct: usize,
    pub v18_distribution_total: usize,
    pub continuation_fingerprint_unchanged: bool,
    pub permanent_fingerprint_unchanged: bool,
    pub residual_temporary_cells: usize,
    pub residual_temporary_arrows: usize,
    pub temporary_capacity_released: bool,
    pub activity_limit_hits: usize,
    pub passed: bool,
}

pub fn run_finish_experiment() -> FinishReport {
    let frozen_continuation = frozen_autonomous_continuation();
    let continuation_fingerprint = frozen_continuation.continuation_fingerprint();
    let mut learner = frozen_continuation.clone().enable_finish_learning();
    let mut training_ids = IdentitySource::new(0x210b_0001);
    let mut training_rng = DeterministicRng::new(0x210b_0002);
    let mut validation_ids = IdentitySource::new(0x210b_1001);
    let mut validation_rng = DeterministicRng::new(0x210b_1002);
    let checkpoints_at = [10, 100, 1_000];
    let mut checkpoints = Vec::new();

    for episode_index in 1..=1_000 {
        let depth = 1 + (episode_index - 1) % 4;
        let episode = chain_episode(&mut training_ids, &mut training_rng, depth, 10);
        present(&mut learner, &episode);
        let _proposal = learner.start_and_finish();
        learner.learn_finish_from_terminal(episode.answer_after(depth));
        learner.erase_temporary();

        if checkpoints_at.contains(&episode_index) {
            let fingerprint = learner.permanent_fingerprint();
            let correct = (0..100)
                .filter(|index| {
                    let depth = 1 + index % 4;
                    let episode =
                        chain_episode(&mut validation_ids, &mut validation_rng, depth, 10);
                    evaluate_finish(&mut learner, &episode).0
                })
                .count();
            assert_eq!(fingerprint, learner.permanent_fingerprint());
            let (permanent_cells, permanent_arrows) = learner.permanent_counts();
            checkpoints.push(FinishCheckpoint {
                training_episodes: episode_index,
                permanent_cells,
                permanent_arrows,
                validation_correct: correct,
                validation_total: 100,
            });
        }
    }

    let permanent_fingerprint = learner.permanent_fingerprint();
    let mut supplied = frozen_continuation.clone().with_supplied_finish();
    let mut no_finish = frozen_continuation;
    let mut held_out_ids = IdentitySource::new(0x210b_2001);
    let mut held_out_rng = DeterministicRng::new(0x210b_2002);
    let mut depth_results = Vec::new();
    let mut held_out_distinct_identities = 0;
    let mut explicit_answer_only = true;
    let mut natural_queue_empty = true;
    let mut no_cutoff_used = true;
    let mut activity_limit_hits = 0;
    let mut representative_episode = None;
    let mut representative_run = None;

    for depth in [5, 8, 16, 32] {
        let total = 200;
        let mut learner_correct = 0;
        let mut supplied_finish_correct = 0;
        let mut no_finish_correct = 0;
        let mut internal_spikes = 0;
        let mut apply_activations = 0;

        for episode_index in 0..total {
            // Main-chain depth changes while total relations stay fixed.
            let episode = chain_episode(&mut held_out_ids, &mut held_out_rng, depth, 40);
            held_out_distinct_identities += episode.distinct_identities;
            let (correct, run, _, _) = evaluate_finish(&mut learner, &episode);
            learner_correct += usize::from(correct);
            internal_spikes += run.internal_spikes;
            apply_activations += run.apply_activations;
            explicit_answer_only &= run.explicit_answer_emitted;
            natural_queue_empty &= run.remaining_queued_spikes == 0;
            no_cutoff_used &= !run.cutoff_reached;
            activity_limit_hits += usize::from(run.activity_limit_hit);

            supplied_finish_correct += usize::from(evaluate_finish(&mut supplied, &episode).0);
            no_finish_correct += usize::from(evaluate_finish(&mut no_finish, &episode).0);

            if depth == 32 && episode_index == 0 {
                representative_episode = Some(episode);
                representative_run = Some(run);
            }
        }

        depth_results.push(FinishDepth {
            depth,
            learner_correct,
            supplied_finish_correct,
            no_finish_correct,
            total,
            average_internal_spikes: internal_spikes as f64 / total as f64,
            average_apply_activations: apply_activations as f64 / total as f64,
        });
    }

    let representative_episode = representative_episode.unwrap();
    let representative_run = representative_run.unwrap();
    let terminal_identity = *representative_episode.chain.last().unwrap();
    let final_events = &representative_run.semantic_events
        [representative_run.semantic_events.len().saturating_sub(5)..];
    let finish_arrow_id = FINISH_ARROW_OFFSET;
    let final_trace_is_exact = final_events
        == [
            SemanticEvent::Current(terminal_identity),
            SemanticEvent::Lookup(terminal_identity),
            SemanticEvent::NoResult,
            SemanticEvent::FinishArrow(finish_arrow_id),
            SemanticEvent::Answer(terminal_identity),
        ];
    let answer_events = representative_run
        .semantic_events
        .iter()
        .filter(|event| matches!(event, SemanticEvent::Answer(_)))
        .count();
    explicit_answer_only &= answer_events == 1;
    let success_path_reuses_v21a_routes = representative_run.trace.iter().all(|step| {
        step.apply_cell_id == APPLY_CELL_ID
            && step.lookup_arrow_id == 1
            && step.feedback_arrow_id == 4
            && step.self_arrow_id == 10
    });
    let depth_work_is_linear = depth_results.windows(2).all(|pair| {
        let first = &pair[0];
        let second = &pair[1];
        let first_slope = (first.average_internal_spikes - 85.0) / first.depth as f64;
        let second_slope = (second.average_internal_spikes - 85.0) / second.depth as f64;
        (first_slope - second_slope).abs() < f64::EPSILON
    });

    let mut working_set_ids = IdentitySource::new(0x210b_4001);
    let mut working_set_rng = DeterministicRng::new(0x210b_4002);
    let mut working_set = Vec::new();
    for relations in [8, 16, 32, 64, 128] {
        let total = 50;
        let mut spikes = 0;
        for _ in 0..total {
            let episode = chain_episode(&mut working_set_ids, &mut working_set_rng, 8, relations);
            let (correct, run, _, _) = evaluate_finish(&mut learner, &episode);
            assert!(correct);
            spikes += run.internal_spikes;
        }
        working_set.push(WorkingSetPoint {
            relations,
            average_internal_spikes: spikes as f64 / total as f64,
        });
    }
    let working_set_work_is_linear = working_set.windows(2).all(|pair| {
        let relation_delta = (pair[1].relations - pair[0].relations) as f64;
        let spike_delta = pair[1].average_internal_spikes - pair[0].average_internal_spikes;
        (spike_delta / relation_delta - 18.0).abs() < f64::EPSILON
    });

    let mut control_ids = IdentitySource::new(0x210b_3001);
    let (branch, duplicate) = control_episodes(&mut control_ids);
    present(&mut learner, &branch);
    let branch_run = learner.start_and_finish();
    let branch_is_ambiguous_without_answer =
        branch_run.outcome == BindingOutcome::Ambiguous && !branch_run.explicit_answer_emitted;
    learner.erase_temporary();

    let duplicate_is_single_answer = evaluate_finish(&mut learner, &duplicate).0;

    let zero_query = control_ids.issue();
    let zero_hop = ContinuationEpisode {
        relations: Vec::new(),
        chain: vec![zero_query],
        query: zero_query,
        distinct_identities: 1,
    };
    let zero_hop_answers_query = evaluate_finish(&mut learner, &zero_hop).0;

    let cycle_a = control_ids.issue();
    let cycle_b = control_ids.issue();
    let cycle = ContinuationEpisode {
        relations: vec![(cycle_a, cycle_b), (cycle_b, cycle_a)],
        chain: vec![cycle_a, cycle_b],
        query: cycle_a,
        distinct_identities: 2,
    };
    present(&mut learner, &cycle);
    let cycle_run = learner.start_and_finish_with_limit(512);
    let cycle_emits_no_answer = !cycle_run.explicit_answer_emitted;
    let cycle_hits_safety_limit = cycle_run.activity_limit_hit;
    learner.erase_temporary();

    let mut v18_ids = IdentitySource::new(0x210b_5001);
    let mut v18_rng = DeterministicRng::new(0x210b_5002);
    let mut v18_distribution_correct = 0;
    let mut v18_distribution_total = 0;
    for depth in [5, 8, 16, 32] {
        for _ in 0..8 {
            let episode = v18_style_episode(&mut v18_ids, &mut v18_rng, depth);
            v18_distribution_correct += usize::from(evaluate_finish(&mut learner, &episode).0);
            v18_distribution_total += 1;
        }
    }

    let (residual_temporary_cells, residual_temporary_arrows) = learner.temporary_counts();
    let temporary_capacity_released = learner.temporary_capacities() == (0, 0, 0, 0, 0, 0, 0);
    let continuation_fingerprint_unchanged =
        continuation_fingerprint == learner.continuation_fingerprint();
    let permanent_fingerprint_unchanged = permanent_fingerprint == learner.permanent_fingerprint();
    let (permanent_cells, permanent_arrows) = learner.permanent_counts();
    let finish_route_strengths = learner.finish_route_strengths();
    let checkpoint_accuracy = checkpoints
        .iter()
        .all(|point| point.validation_correct == point.validation_total);
    let structure_plateaued = checkpoints.windows(2).all(|pair| {
        pair[0].permanent_cells == pair[1].permanent_cells
            && pair[0].permanent_arrows == pair[1].permanent_arrows
    });
    let learned_all_depths = depth_results
        .iter()
        .all(|point| point.learner_correct == point.total);
    let supplied_all_depths = depth_results
        .iter()
        .all(|point| point.supplied_finish_correct == point.total);
    let no_finish_never_answers = depth_results
        .iter()
        .all(|point| point.no_finish_correct == 0);
    let passed = checkpoint_accuracy
        && structure_plateaued
        && learned_all_depths
        && supplied_all_depths
        && no_finish_never_answers
        && explicit_answer_only
        && natural_queue_empty
        && no_cutoff_used
        && success_path_reuses_v21a_routes
        && final_trace_is_exact
        && depth_work_is_linear
        && working_set_work_is_linear
        && branch_is_ambiguous_without_answer
        && duplicate_is_single_answer
        && zero_hop_answers_query
        && cycle_emits_no_answer
        && cycle_hits_safety_limit
        && v18_distribution_correct == v18_distribution_total
        && continuation_fingerprint_unchanged
        && permanent_fingerprint_unchanged
        && residual_temporary_cells == 0
        && residual_temporary_arrows == 0
        && temporary_capacity_released
        && activity_limit_hits == 0;

    FinishReport {
        checkpoints,
        depth_results,
        working_set,
        permanent_cells,
        permanent_arrows,
        finish_route_strengths,
        held_out_distinct_identities,
        explicit_answer_only,
        natural_queue_empty,
        no_cutoff_used,
        success_path_reuses_v21a_routes,
        final_trace_is_exact,
        finish_arrow_id,
        depth_work_is_linear,
        working_set_work_is_linear,
        branch_is_ambiguous_without_answer,
        duplicate_is_single_answer,
        zero_hop_answers_query,
        cycle_emits_no_answer,
        cycle_hits_safety_limit,
        v18_distribution_correct,
        v18_distribution_total,
        continuation_fingerprint_unchanged,
        permanent_fingerprint_unchanged,
        residual_temporary_cells,
        residual_temporary_arrows,
        temporary_capacity_released,
        activity_limit_hits,
        passed,
    }
}

pub fn print_finish_report(report: &FinishReport) {
    println!("v21b learned finish:");
    print!("  checkpoints episodes:cells/arrows/accuracy:");
    for checkpoint in &report.checkpoints {
        print!(
            " {}:{}/{}/{}/{}",
            checkpoint.training_episodes,
            checkpoint.permanent_cells,
            checkpoint.permanent_arrows,
            checkpoint.validation_correct,
            checkpoint.validation_total
        );
    }
    println!();
    for depth in &report.depth_results {
        println!(
            "  depth {} learner/supplied/no-finish={}/{}, {}/{}, {}/{}, internal spikes={:.1}, apply activations={:.1}",
            depth.depth,
            depth.learner_correct,
            depth.total,
            depth.supplied_finish_correct,
            depth.total,
            depth.no_finish_correct,
            depth.total,
            depth.average_internal_spikes,
            depth.average_apply_activations
        );
    }
    print!("  working-set relations:spikes:");
    for point in &report.working_set {
        print!(" {}:{:.1}", point.relations, point.average_internal_spikes);
    }
    println!();
    println!(
        "  explicit answer={}, natural empty queue={}, no cutoff={}, reused v21a path={}, exact final trace={}, finish arrow={}",
        report.explicit_answer_only,
        report.natural_queue_empty,
        report.no_cutoff_used,
        report.success_path_reuses_v21a_routes,
        report.final_trace_is_exact,
        report.finish_arrow_id
    );
    println!(
        "  permanent cells/arrows={}/{}, v18 distribution={}/{}, branch ambiguous={}, duplicate={}, zero-hop={}, cycle no-answer/limit={}/{}, activity-limit hits={}",
        report.permanent_cells,
        report.permanent_arrows,
        report.v18_distribution_correct,
        report.v18_distribution_total,
        report.branch_is_ambiguous_without_answer,
        report.duplicate_is_single_answer,
        report.zero_hop_answers_query,
        report.cycle_emits_no_answer,
        report.cycle_hits_safety_limit,
        report.activity_limit_hits
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v21a_selects_self_trigger_from_terminal_supervision() {
        let report = run_experiment();
        let selected = report
            .route_strengths
            .iter()
            .find(|(route, _)| route == "ApplyAgain")
            .unwrap()
            .1;
        let strongest_alternative = report
            .route_strengths
            .iter()
            .filter(|(route, _)| route != "ApplyAgain")
            .map(|(_, strength)| *strength)
            .max()
            .unwrap();

        assert!(selected > strongest_alternative);
        assert!(report
            .checkpoints
            .iter()
            .all(|point| point.validation_correct == point.validation_total));
    }

    #[test]
    fn v21a_one_start_generates_all_later_applications_inside_the_queue() {
        let report = run_experiment();

        assert!(report.external_start_is_always_one);
        assert!(report.apply_activations_match_cutoff);
        assert!(report
            .depth_results
            .iter()
            .all(|point| point.learner_correct == point.total));
        assert!(report.trace_reuses_apply_cell);
        assert!(report.trace_reuses_lookup_arrow);
        assert!(report.trace_reuses_feedback_arrow);
        assert!(report.trace_reuses_self_arrow);
        assert!(report.trace_current_is_correct);
        assert!(report.trace_step_work_consistent);
        assert!(report.cutoff_discards_one_self_trigger);
    }

    #[test]
    fn v21a_distinguishes_learned_continuation_from_v20_without_self_trigger() {
        let report = run_experiment();

        for point in &report.depth_results {
            assert_eq!(point.supplied_self_trigger_correct, point.total);
            if point.cutoff == 1 {
                assert_eq!(point.no_self_trigger_correct, point.total);
            } else {
                assert_eq!(point.no_self_trigger_correct, 0);
            }
        }
    }

    #[test]
    fn v21a_fixed_knowledge_buys_more_transient_computation_at_greater_depth() {
        let report = run_experiment();

        assert!(report.internal_work_is_linear);
        assert!(report
            .depth_results
            .windows(2)
            .all(|pair| pair[1].average_internal_spikes > pair[0].average_internal_spikes));
        assert_eq!(report.activity_limit_hits, 0);
        assert!(report.permanent_fingerprint_unchanged);
        assert!(report.temporary_capacity_released);
    }

    #[test]
    fn v21a_local_ambiguity_and_duplicate_controls_remain_correct() {
        let report = run_experiment();

        assert!(report.branch_is_ambiguous);
        assert!(report.duplicate_is_single_result);
        assert!(report.passed);
    }

    #[test]
    fn v21b_selects_explicit_finish_from_terminal_supervision() {
        let report = run_finish_experiment();
        let selected = report
            .finish_route_strengths
            .iter()
            .find(|(route, _)| route == "AnswerCurrent")
            .unwrap()
            .1;
        let strongest_alternative = report
            .finish_route_strengths
            .iter()
            .filter(|(route, _)| route != "AnswerCurrent")
            .map(|(_, strength)| *strength)
            .max()
            .unwrap();

        assert!(selected > strongest_alternative);
        assert!(report
            .checkpoints
            .iter()
            .all(|point| point.validation_correct == point.validation_total));
    }

    #[test]
    fn v21b_finishes_naturally_with_an_explicit_answer_at_unseen_depths() {
        let report = run_finish_experiment();

        assert!(report
            .depth_results
            .iter()
            .all(|point| point.learner_correct == point.total));
        assert!(report.explicit_answer_only);
        assert!(report.natural_queue_empty);
        assert!(report.no_cutoff_used);
        assert!(report.success_path_reuses_v21a_routes);
        assert!(report.final_trace_is_exact);
    }

    #[test]
    fn v21b_preserves_depth_and_working_set_scaling_axes() {
        let report = run_finish_experiment();

        assert!(report.depth_work_is_linear);
        assert!(report.working_set_work_is_linear);
        assert!(report
            .depth_results
            .windows(2)
            .all(|pair| pair[1].average_internal_spikes > pair[0].average_internal_spikes));
        assert!(report
            .working_set
            .windows(2)
            .all(|pair| pair[1].average_internal_spikes > pair[0].average_internal_spikes));
    }

    #[test]
    fn v21b_controls_distinguish_finish_ambiguity_duplicates_and_cycles() {
        let report = run_finish_experiment();

        assert!(report.branch_is_ambiguous_without_answer);
        assert!(report.duplicate_is_single_answer);
        assert!(report.zero_hop_answers_query);
        assert!(report.cycle_emits_no_answer);
        assert!(report.cycle_hits_safety_limit);
    }

    #[test]
    fn v21b_recomposes_the_v18_distribution_without_changing_frozen_routes() {
        let report = run_finish_experiment();

        assert_eq!(
            report.v18_distribution_correct,
            report.v18_distribution_total
        );
        assert_eq!(report.v18_distribution_total, 32);
        assert!(report.continuation_fingerprint_unchanged);
        assert!(report.permanent_fingerprint_unchanged);
        assert!(report.temporary_capacity_released);
        assert!(report.passed);
    }
}
