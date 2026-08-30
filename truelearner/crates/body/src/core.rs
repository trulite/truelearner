//! What changes because of one observed physical event.

use crate::{
    arena::Arena,
    engine::PhysicalMoment,
    trace::{
        CandidateTrace, ChoiceBasis, ChoiceTrace, NoTrace, ReturnDecision, ReturnTrace,
        StrengthTrace, TraceEvent, TracePath, TraceSink,
    },
    Body, BuildError, Impulse, Junction, JunctionId, Link, LinkId, RunError, Time, Trigger,
};
use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap},
};

pub type Cause = u64;
pub type Cohort = u64;
pub type Boundary = u32;
const LOCAL_RADIUS: i32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Path {
    pub surface: JunctionId,
    pub middle: JunctionId,
    pub output: JunctionId,
    pub first: LinkId,
    pub second: LinkId,
}

impl Path {
    const fn links(self) -> [LinkId; 2] {
        [self.first, self.second]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub at: Time,
    pub caused_transition: bool,
    pub available_until_choice: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Owner {
    Organism,
    Learner { id: Boundary, root: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Candidate<'a> {
    pub path: Path,
    pub connected_outcomes: &'a [JunctionId],
    pub owner: Owner,
    pub opportunity_from: Option<Owner>,
    pub executable: bool,
    pub return_cause: Option<Cause>,
    pub unanswered: bool,
    pub outcome: Option<Outcome>,
    pub participation: u64,
    pub strength: i64,
    pub stable_order: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Surface<'a> {
    pub id: JunctionId,
    pub external: bool,
    pub maximally_resistant: bool,
    pub output: bool,
    pub learned_intermediate: bool,
    pub boundary_effect: bool,
    pub nearby_outputs: &'a [(JunctionId, i32)],
    pub existing_paths: &'a [(JunctionId, i8)],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsedPath {
    pub path: Path,
    pub cause: Cause,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReturnedOutcome<'a> {
    pub used: UsedPath,
    pub return_links: &'a [LinkId],
    pub cohort: Cohort,
    pub return_opened_at: Time,
    pub transition_at: Option<Time>,
}

#[derive(Clone, Copy, Debug)]
pub struct Closure<'a> {
    pub parent: Option<Boundary>,
    pub parent_members: &'a [JunctionId],
    pub participating_members: &'a [JunctionId],
    pub live_witness: Option<(Path, Time)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CyclePath {
    pub used: UsedPath,
    pub participated: bool,
    pub opened_at: Time,
    pub transition_at: Option<Time>,
}

#[derive(Clone, Copy, Debug)]
pub enum Event<'a> {
    SurfaceFired(Surface<'a>),
    PathsReady {
        candidates: &'a [Candidate<'a>],
        current_transition: Option<Cause>,
    },
    OutputFired(UsedPath),
    OutcomesReturned(&'a [ReturnedOutcome<'a>]),
    BoundaryClosed(Closure<'a>),
    NaturalCycle(&'a [CyclePath]),
    Quiet,
}

#[derive(Clone, Copy, Debug)]
pub struct Context<'a> {
    pub at: Time,
    pub event: Event<'a>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NewJunction(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NewLink(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JunctionRef {
    Existing(JunctionId),
    New(NewJunction),
}

impl From<JunctionId> for JunctionRef {
    fn from(value: JunctionId) -> Self {
        Self::Existing(value)
    }
}

impl From<NewJunction> for JunctionRef {
    fn from(value: NewJunction) -> Self {
        Self::New(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkRef {
    Existing(LinkId),
    New(NewLink),
}

impl From<LinkId> for LinkRef {
    fn from(value: LinkId) -> Self {
        Self::Existing(value)
    }
}

impl From<NewLink> for LinkRef {
    fn from(value: NewLink) -> Self {
        Self::New(value)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LinkRole {
    #[default]
    Drive,
    PathEntry,
    Return {
        cause: Cause,
        cohort: Cohort,
    },
    OutcomeWitness,
    Membership,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkSpec {
    pub delay: Time,
    pub impulse: Impulse,
    pub trigger: Trigger,
    pub role: LinkRole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkChange {
    Participated {
        cause: Cause,
        at: Time,
    },
    RememberOutcome {
        at: Time,
        available_until_choice: bool,
    },
    ConsumeOutcome,
    Strengthen {
        amount: i32,
    },
    Retire,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Edit {
    AddJunction {
        new: NewJunction,
        spec: Junction,
    },
    AddLink {
        new: NewLink,
        from: JunctionRef,
        to: JunctionRef,
        spec: LinkSpec,
    },
    Send {
        through: LinkRef,
        at: Time,
        cause: Cause,
    },
    ChangeLink {
        link: LinkRef,
        change: LinkChange,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Change {
    edits: Vec<Edit>,
    junctions: u32,
    links: u32,
}

impl Change {
    pub fn empty() -> Self {
        Self::default()
    }

    fn new_junction(&mut self) -> NewJunction {
        let id = NewJunction(self.junctions);
        self.junctions += 1;
        id
    }

    fn new_link(&mut self) -> NewLink {
        let id = NewLink(self.links);
        self.links += 1;
        id
    }

    fn push(&mut self, edit: Edit) {
        self.edits.push(edit);
    }

    fn clear(&mut self) {
        self.edits.clear();
        self.junctions = 0;
        self.links = 0;
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Trace {
    pub event: &'static str,
    pub requested_edits: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reaction {
    pub change: Change,
    pub trace: Trace,
}

pub fn react_event(context: Context<'_>) -> Reaction {
    let (event, change) = match context.event {
        Event::SurfaceFired(surface) => ("surface-fired", surface_fired(surface)),
        Event::PathsReady {
            candidates,
            current_transition,
        } => (
            "paths-ready",
            paths_ready(context.at, candidates, current_transition),
        ),
        Event::OutputFired(used) => ("output-fired", output_fired(context.at, used)),
        Event::OutcomesReturned(outcomes) => ("outcomes-returned", outcomes_returned(outcomes)),
        Event::BoundaryClosed(closure) => ("boundary-closed", boundary_closed(context.at, closure)),
        Event::NaturalCycle(paths) => ("natural-cycle", natural_cycle(paths)),
        Event::Quiet => ("quiet", Change::empty()),
    };
    let trace = Trace {
        event,
        requested_edits: change.edits.len(),
    };
    Reaction { change, trace }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum UsedPaths {
    #[default]
    None,
    One(Path),
    Many,
}

impl UsedPaths {
    pub(crate) fn include(&mut self, path: Option<Path>) {
        let Some(path) = path else {
            return;
        };
        *self = match *self {
            Self::None => Self::One(path),
            Self::One(_) | Self::Many => Self::Many,
        };
    }
}

#[derive(Clone, Copy, Debug)]
struct MomentFact {
    event: crate::physics::Event,
    boundary: bool,
    used: UsedPaths,
    had_ready_path: bool,
}

#[derive(Clone, Copy, Debug)]
struct ConstructionFact {
    cause: Cause,
    junction: JunctionId,
    consequence: bool,
}

#[derive(Clone, Debug, Default)]
struct ConstructionScratch {
    counts: HashMap<Cause, usize>,
    passive_counts: HashMap<Cause, usize>,
    facts: Vec<ConstructionFact>,
    members: Vec<JunctionId>,
    consequences: Vec<JunctionId>,
    candidates: Vec<JunctionId>,
    stack: Vec<JunctionId>,
    visited: Vec<JunctionId>,
    leaves: Vec<JunctionId>,
    parent_members: Vec<JunctionId>,
}

impl ConstructionScratch {
    fn clear(&mut self) {
        self.counts.clear();
        self.passive_counts.clear();
        self.facts.clear();
        self.members.clear();
        self.consequences.clear();
        self.candidates.clear();
        self.stack.clear();
        self.visited.clear();
        self.leaves.clear();
        self.parent_members.clear();
    }
}

#[derive(Clone, Copy, Debug)]
struct DetectedClosure {
    at: Time,
    parent: Option<JunctionId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MembershipParent {
    Root,
    Existing(JunctionId),
    Ambiguous,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ReactionScratch {
    facts: Vec<MomentFact>,
    ready: Vec<ReadyPath>,
    connected_outcomes: Vec<JunctionId>,
    worlds: Vec<usize>,
    winners: Vec<ReadyChoice>,
    construction: ConstructionScratch,
    pub(crate) change: Change,
    pub(crate) applied: Applied,
}

#[derive(Clone, Copy)]
pub(crate) struct ReactionView<'a> {
    arena: &'a Arena,
    link_memory: &'a [LinkMemory],
    live_returns: &'a [ReturnEntry],
}

impl<'a> ReactionView<'a> {
    pub(crate) const fn new(
        arena: &'a Arena,
        link_memory: &'a [LinkMemory],
        live_returns: &'a [ReturnEntry],
    ) -> Self {
        Self {
            arena,
            link_memory,
            live_returns,
        }
    }
}

impl ReactionScratch {
    pub(crate) fn clear(&mut self) {
        self.facts.clear();
        self.ready.clear();
        self.connected_outcomes.clear();
        self.worlds.clear();
        self.winners.clear();
        self.construction.clear();
        self.change.clear();
        self.applied.junctions.clear();
        self.applied.links.clear();
    }

    #[cfg(test)]
    pub(crate) fn is_clear(&self) -> bool {
        self.facts.is_empty()
            && self.ready.is_empty()
            && self.connected_outcomes.is_empty()
            && self.worlds.is_empty()
            && self.winners.is_empty()
            && self.construction.counts.is_empty()
            && self.construction.facts.is_empty()
            && self.construction.passive_counts.is_empty()
            && self.construction.members.is_empty()
            && self.construction.consequences.is_empty()
            && self.construction.candidates.is_empty()
            && self.construction.stack.is_empty()
            && self.construction.visited.is_empty()
            && self.construction.leaves.is_empty()
            && self.construction.parent_members.is_empty()
            && self.change.is_empty()
            && self.applied.junctions.is_empty()
            && self.applied.links.is_empty()
    }

    #[cfg(test)]
    pub(crate) fn fact_capacity(&self) -> usize {
        self.facts.capacity()
    }
}

pub(crate) fn reaction_needed(body: ReactionView<'_>, moment: &PhysicalMoment) -> bool {
    let mut boundary_changes = 0_usize;
    for recorded in &moment.changes {
        if !matches!(recorded.used, UsedPaths::None)
            || recorded.boundary && boundary_can_react(body, recorded.event.junction)
        {
            return true;
        }
        if recorded.boundary && recorded.event.cause != 0 {
            boundary_changes += 1;
            if boundary_changes >= 2 {
                return true;
            }
        }
    }
    false
}

fn boundary_can_react(body: ReactionView<'_>, surface: JunctionId) -> bool {
    if !body.live_returns.is_empty() {
        return true;
    }
    let mut next = body
        .arena
        .junction(surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link_id) = next {
        let link = body.arena.link(link_id).expect("live link");
        let memory = &body.link_memory[link_id.slot()];
        if (memory.live && memory.role == LinkRole::PathEntry)
            || (memory.role == LinkRole::Drive
                && link.impulse == 0
                && (1..=LOCAL_RADIUS as Time).contains(&link.delay))
        {
            return true;
        }
        next = link.next;
    }
    false
}

fn is_outcome_source(body: ReactionView<'_>, junction: JunctionId) -> bool {
    let mut next = body
        .arena
        .junction(junction)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link_id) = next {
        let link = body.arena.link(link_id).expect("live link");
        let memory = &body.link_memory[link_id.slot()];
        if memory.live && memory.role == LinkRole::OutcomeWitness {
            return true;
        }
        next = link.next;
    }
    false
}

fn is_membership_link(body: ReactionView<'_>, link: LinkId) -> bool {
    body.link_memory[link.slot()].live && body.link_memory[link.slot()].role == LinkRole::Membership
}

fn has_membership_children(body: ReactionView<'_>, junction: JunctionId) -> bool {
    let mut next = body
        .arena
        .junction(junction)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link_id) = next {
        let link = body.arena.link(link_id).expect("live link");
        if is_membership_link(body, link_id) {
            return true;
        }
        next = link.next;
    }
    false
}

fn member_is_owned(body: ReactionView<'_>, member: JunctionId) -> bool {
    body.arena
        .incoming(member)
        .any(|link| is_membership_link(body, link))
}

fn append_membership_ancestors(
    body: ReactionView<'_>,
    member: JunctionId,
    candidates: &mut Vec<JunctionId>,
    stack: &mut Vec<JunctionId>,
    visited: &mut Vec<JunctionId>,
) {
    stack.push(member);
    while let Some(junction) = stack.pop() {
        for link_id in body.arena.incoming(junction) {
            if !is_membership_link(body, link_id) {
                continue;
            }
            let parent = body.arena.link(link_id).expect("live link").from;
            if visited.contains(&parent) {
                continue;
            }
            visited.push(parent);
            candidates.push(parent);
            stack.push(parent);
        }
    }
}

fn collect_membership_leaves(
    body: ReactionView<'_>,
    boundary: JunctionId,
    stack: &mut Vec<JunctionId>,
    visited: &mut Vec<JunctionId>,
    leaves: &mut Vec<JunctionId>,
) {
    stack.clear();
    visited.clear();
    leaves.clear();
    stack.push(boundary);
    while let Some(junction) = stack.pop() {
        if visited.contains(&junction) {
            continue;
        }
        visited.push(junction);
        let mut next = body
            .arena
            .junction(junction)
            .and_then(|slot| slot.outgoing_head);
        while let Some(link_id) = next {
            let link = body.arena.link(link_id).expect("live link");
            next = link.next;
            if !is_membership_link(body, link_id) {
                continue;
            }
            if has_membership_children(body, link.to) {
                stack.push(link.to);
            } else if !leaves.contains(&link.to) {
                leaves.push(link.to);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn resolve_membership_parent(
    body: ReactionView<'_>,
    members: &[JunctionId],
    candidates: &mut Vec<JunctionId>,
    stack: &mut Vec<JunctionId>,
    visited: &mut Vec<JunctionId>,
    leaves: &mut Vec<JunctionId>,
    parent_members: &mut Vec<JunctionId>,
) -> MembershipParent {
    candidates.clear();
    stack.clear();
    visited.clear();
    parent_members.clear();
    for member in members {
        append_membership_ancestors(body, *member, candidates, stack, visited);
    }
    if candidates.is_empty() {
        return MembershipParent::Root;
    }

    let mut best = None;
    let mut best_size = 0_usize;
    for candidate in candidates.iter().copied() {
        collect_membership_leaves(body, candidate, stack, visited, leaves);
        let complete = leaves.iter().all(|leaf| members.contains(leaf));
        let contains_owned = members
            .iter()
            .all(|member| !member_is_owned(body, *member) || leaves.contains(member));
        if !complete || !contains_owned {
            continue;
        }
        if leaves.len() < best_size {
            continue;
        }
        if leaves.len() == best_size && best.is_some() && best != Some(candidate) {
            return MembershipParent::Ambiguous;
        }
        best = Some(candidate);
        best_size = leaves.len();
        parent_members.clear();
        parent_members.extend_from_slice(leaves);
    }

    best.map_or(MembershipParent::Ambiguous, MembershipParent::Existing)
}

fn detect_closure(
    body: ReactionView<'_>,
    moment: &PhysicalMoment,
    scratch: &mut ConstructionScratch,
) -> Option<DetectedClosure> {
    scratch.clear();
    let mut at = None;
    for change in &moment.changes {
        let event = change.event;
        if !change.boundary || event.cause == 0 {
            continue;
        }
        at = Some(event.at);
        let consequence = is_outcome_source(body, event.junction);
        if !consequence {
            *scratch.counts.entry(event.cause).or_default() += 1;
            if !boundary_can_react(body, event.junction) {
                *scratch.passive_counts.entry(event.cause).or_default() += 1;
            }
        }
        scratch.facts.push(ConstructionFact {
            cause: event.cause,
            junction: event.junction,
            consequence,
        });
    }

    let mut causes = scratch
        .counts
        .iter()
        .filter(|(cause, count)| {
            **count >= 2 && scratch.passive_counts.get(cause).copied().unwrap_or(0) > 0
        })
        .map(|(cause, _)| *cause);
    let cause = causes.next()?;
    if causes.next().is_some() {
        return None;
    }
    for fact in &scratch.facts {
        if fact.cause != cause {
            continue;
        }
        if fact.consequence {
            scratch.consequences.push(fact.junction);
        } else {
            scratch.members.push(fact.junction);
        }
    }

    let parent = resolve_membership_parent(
        body,
        &scratch.members,
        &mut scratch.candidates,
        &mut scratch.stack,
        &mut scratch.visited,
        &mut scratch.leaves,
        &mut scratch.parent_members,
    );
    let parent = match parent {
        MembershipParent::Root => None,
        MembershipParent::Existing(parent) => Some(parent),
        MembershipParent::Ambiguous => return None,
    };
    Some(DetectedClosure { at: at?, parent })
}

fn direct_membership_parent(
    body: ReactionView<'_>,
    member: JunctionId,
) -> Result<Option<JunctionId>, ()> {
    let mut parent = None;
    for link_id in body.arena.incoming(member) {
        if !is_membership_link(body, link_id) {
            continue;
        }
        let found = body.arena.link(link_id).expect("live link").from;
        if parent.is_some_and(|existing| existing != found) {
            return Err(());
        }
        parent = Some(found);
    }
    Ok(parent)
}

fn path_is_executable(body: ReactionView<'_>, surface: JunctionId, has_outcome: bool) -> bool {
    match direct_membership_parent(body, surface) {
        Ok(None) => true,
        Ok(Some(parent)) => !member_is_owned(body, parent) && has_outcome,
        Err(()) => false,
    }
}

pub(crate) fn react_into<T: TraceSink>(
    body: ReactionView<'_>,
    moment: &PhysicalMoment,
    scratch: &mut ReactionScratch,
    trace: &mut T,
) {
    scratch.clear();
    scratch
        .facts
        .extend(moment.changes.iter().map(|recorded| MomentFact {
            event: recorded.event,
            boundary: recorded.boundary,
            used: recorded.used,
            had_ready_path: false,
        }));
    let construction = detect_closure(body, moment, &mut scratch.construction);
    form_and_choose(
        body,
        &mut scratch.facts,
        &mut scratch.ready,
        &mut scratch.connected_outcomes,
        &mut scratch.worlds,
        &mut scratch.winners,
        &mut scratch.change,
        construction.is_some(),
        trace,
    );
    if let Some(closure) = construction {
        construct_membership(
            closure,
            &scratch.construction.members,
            &scratch.construction.parent_members,
            &mut scratch.change,
        );
        remember_construction_outcomes(
            closure.at,
            &scratch.construction.members,
            &scratch.construction.parent_members,
            &scratch.construction.consequences,
            &scratch.ready,
            &scratch.connected_outcomes,
            &scratch.winners,
            &mut scratch.change,
        );
        return;
    }
    record_used_outputs(&scratch.facts, &mut scratch.change);
    record_returned_outcomes(body, &scratch.facts, &mut scratch.change, trace);
}

fn construct_membership(
    closure: DetectedClosure,
    members: &[JunctionId],
    parent_members: &[JunctionId],
    change: &mut Change,
) {
    if members.iter().all(|member| parent_members.contains(member)) {
        return;
    }
    let boundary = change.new_junction();
    change.push(Edit::AddJunction {
        new: boundary,
        spec: Junction::integrating(1),
    });
    if let Some(parent) = closure.parent {
        let membership = change.new_link();
        change.push(Edit::AddLink {
            new: membership,
            from: boundary.into(),
            to: parent.into(),
            spec: LinkSpec {
                delay: 0,
                impulse: 1,
                trigger: Trigger::SourceFires,
                role: LinkRole::Membership,
            },
        });
    }
    for member in members {
        if parent_members.contains(member) {
            continue;
        }
        let membership = change.new_link();
        change.push(Edit::AddLink {
            new: membership,
            from: boundary.into(),
            to: (*member).into(),
            spec: LinkSpec {
                delay: 0,
                impulse: 1,
                trigger: Trigger::SourceFires,
                role: LinkRole::Membership,
            },
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn remember_construction_outcomes(
    at: Time,
    members: &[JunctionId],
    parent_members: &[JunctionId],
    consequences: &[JunctionId],
    ready: &[ReadyPath],
    connected_outcomes: &[JunctionId],
    winners: &[ReadyChoice],
    change: &mut Change,
) {
    if consequences.is_empty() {
        return;
    }
    for winner in winners.iter().map(|choice| &ready[choice.winner]) {
        if !members.contains(&winner.surface) || parent_members.contains(&winner.surface) {
            continue;
        }
        let connected = &connected_outcomes[winner.connected_start..winner.connected_end];
        if !consequences
            .iter()
            .any(|consequence| connected.contains(consequence))
        {
            continue;
        }
        for link in [winner.first, winner.second] {
            change.push(Edit::ChangeLink {
                link,
                change: LinkChange::RememberOutcome {
                    at,
                    available_until_choice: true,
                },
            });
        }
    }
}

fn record_used_outputs(facts: &[MomentFact], change: &mut Change) {
    for fact in facts {
        let UsedPaths::One(path) = fact.used else {
            continue;
        };
        for link in path.links() {
            change.push(Edit::ChangeLink {
                link: link.into(),
                change: LinkChange::Participated {
                    cause: fact.event.cause,
                    at: fact.event.at,
                },
            });
        }
        let returned = change.new_link();
        change.push(Edit::AddLink {
            new: returned,
            from: path.output.into(),
            to: path.middle.into(),
            spec: LinkSpec {
                delay: 0,
                impulse: 0,
                trigger: Trigger::SourceFires,
                role: LinkRole::Return {
                    cause: fact.event.cause,
                    cohort: fact.event.cause,
                },
            },
        });
        change.push(Edit::ChangeLink {
            link: returned.into(),
            change: LinkChange::Participated {
                cause: fact.event.cause,
                at: fact.event.at,
            },
        });
    }
}

fn record_returned_outcomes<T: TraceSink>(
    body: ReactionView<'_>,
    facts: &[MomentFact],
    change: &mut Change,
    trace: &mut T,
) {
    for fact in facts {
        if !fact.boundary
            || !matches!(fact.used, UsedPaths::None)
            || !is_outcome_source(body, fact.event.junction)
        {
            continue;
        }
        if fact.had_ready_path {
            if T::ENABLED {
                trace.record(TraceEvent::Return(ReturnTrace {
                    at: fact.event.at,
                    source: fact.event.junction,
                    incoming_cause: fact.event.cause,
                    path: None,
                    return_cause: None,
                    return_opened_at: None,
                    open_paths: component_return_count(body, fact.event.junction),
                    exact_paths: 0,
                    decision: ReturnDecision::BlockedByReadyPath,
                }));
            }
            continue;
        }

        let selected = select_live_return(body, fact.event.junction, fact.event.cause);
        let decision = match selected.selected {
            Some(returned) if returned.opened_at <= fact.event.at => ReturnDecision::Accepted,
            Some(_) => ReturnDecision::BeforeReturnOpened,
            None if selected.total == 0 => ReturnDecision::NoOpenPath,
            None => ReturnDecision::Ambiguous,
        };
        if T::ENABLED {
            trace.record(TraceEvent::Return(ReturnTrace {
                at: fact.event.at,
                source: fact.event.junction,
                incoming_cause: fact.event.cause,
                path: selected.selected.map(|returned| returned.path),
                return_cause: selected.selected.map(|returned| returned.cause),
                return_opened_at: selected.selected.map(|returned| returned.opened_at),
                open_paths: selected.total,
                exact_paths: selected.exact_total,
                decision,
            }));
        }
        if let Some(returned) = selected
            .selected
            .filter(|returned| returned.opened_at <= fact.event.at)
        {
            for link in returned.path.links() {
                change.push(Edit::ChangeLink {
                    link: link.into(),
                    change: LinkChange::RememberOutcome {
                        at: fact.event.at,
                        available_until_choice: true,
                    },
                });
                change.push(Edit::ChangeLink {
                    link: link.into(),
                    change: LinkChange::Strengthen { amount: 1 },
                });
            }
        }
        for entry in body.live_returns {
            let Some(returned) = open_return(body, *entry) else {
                continue;
            };
            if return_is_connected(body, fact.event.junction, returned) {
                change.push(Edit::ChangeLink {
                    link: entry.link.into(),
                    change: LinkChange::Retire,
                });
            }
        }
    }
}

#[derive(Clone, Copy)]
struct OpenReturn {
    path: Path,
    cause: Cause,
    opened_at: Time,
}

#[derive(Clone, Copy)]
struct ReturnSelection {
    selected: Option<OpenReturn>,
    total: usize,
    exact_total: usize,
}

fn select_live_return(body: ReactionView<'_>, source: JunctionId, cause: Cause) -> ReturnSelection {
    let mut only = None;
    let mut total = 0_usize;
    let mut exact = None;
    let mut exact_total = 0_usize;
    for entry in body.live_returns {
        let Some(returned) = open_return(body, *entry) else {
            continue;
        };
        if !return_is_connected(body, source, returned) {
            continue;
        }
        total += 1;
        only = Some(returned);
        if returned.cause == cause {
            exact_total += 1;
            exact = Some(returned);
        }
    }
    let selected = match (exact_total, total) {
        (1, _) => exact,
        (0, 1) => only,
        _ => None,
    };
    ReturnSelection {
        selected,
        total,
        exact_total,
    }
}

fn component_return_count(body: ReactionView<'_>, source: JunctionId) -> usize {
    body.live_returns
        .iter()
        .filter_map(|entry| open_return(body, *entry))
        .filter(|returned| return_is_connected(body, source, *returned))
        .count()
}

fn return_is_connected(body: ReactionView<'_>, source: JunctionId, returned: OpenReturn) -> bool {
    outcome_source_enters(body, source, returned.path.middle)
        || outcome_source_enters(body, source, returned.path.output)
}

fn outcome_source_enters(body: ReactionView<'_>, source: JunctionId, junction: JunctionId) -> bool {
    body.arena.incoming(junction).any(|link| {
        body.link_memory[link.slot()].live
            && body.link_memory[link.slot()].role == LinkRole::OutcomeWitness
            && body
                .arena
                .link(link)
                .is_some_and(|physical| physical.from == source)
    })
}

fn open_return(body: ReactionView<'_>, entry: ReturnEntry) -> Option<OpenReturn> {
    let memory = body.link_memory.get(entry.link.slot())?;
    let LinkRole::Return { cause, .. } = memory.role else {
        return None;
    };
    if !memory.live {
        return None;
    }
    debug_assert_eq!(cause, entry.cause);
    let physical = body
        .arena
        .link(entry.link)
        .expect("indexed live return link");
    let second = outgoing_drive_to(body, physical.to, physical.from)?;
    let path = path_from_drive(body, second)?;
    Some(OpenReturn {
        path,
        cause,
        opened_at: memory.participated_at,
    })
}

fn outgoing_drive_to(
    body: ReactionView<'_>,
    middle: JunctionId,
    output: JunctionId,
) -> Option<LinkId> {
    let mut next = body
        .arena
        .junction(middle)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link) = next {
        let physical = body.arena.link(link).expect("live link");
        if body.link_memory[link.slot()].live
            && body.link_memory[link.slot()].role == LinkRole::Drive
            && physical.to == output
            && physical.impulse != 0
        {
            return Some(link);
        }
        next = physical.next;
    }
    None
}

pub(crate) fn path_from_drive(body: ReactionView<'_>, second: LinkId) -> Option<Path> {
    let drive = body.arena.link(second)?;
    let memory = body.link_memory.get(second.slot())?;
    if !memory.live || memory.role != LinkRole::Drive || drive.impulse == 0 {
        return None;
    }
    let first = body.arena.incoming(drive.from).find(|first| {
        body.link_memory[first.slot()].live
            && body.link_memory[first.slot()].role == LinkRole::PathEntry
    })?;
    let entry = body.arena.link(first)?;
    Some(Path {
        surface: entry.from,
        middle: drive.from,
        output: drive.to,
        first,
        second,
    })
}

#[derive(Clone, Debug)]
struct ReadyPath {
    surface: JunctionId,
    middle: JunctionRef,
    output: JunctionId,
    first: LinkRef,
    second: LinkRef,
    at: Time,
    current_cause: Cause,
    return_cause: Option<Cause>,
    unanswered: bool,
    connected_start: usize,
    connected_end: usize,
    outcome: Option<Outcome>,
    participation: u64,
    strength: i64,
    stable_order: u32,
    executable: bool,
}

impl ReadyPath {
    const fn trace_path(&self) -> TracePath {
        TracePath {
            surface: self.surface,
            middle: self.middle,
            output: self.output,
            first: self.first,
            second: self.second,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ReadyChoice {
    winner: usize,
    basis: ChoiceBasis,
}

#[allow(clippy::too_many_arguments)]
fn form_and_choose<T: TraceSink>(
    body: ReactionView<'_>,
    facts: &mut [MomentFact],
    ready: &mut Vec<ReadyPath>,
    connected_outcomes: &mut Vec<JunctionId>,
    worlds: &mut Vec<usize>,
    winners: &mut Vec<ReadyChoice>,
    change: &mut Change,
    construction: bool,
    trace: &mut T,
) {
    for fact in facts.iter_mut() {
        if !fact.boundary {
            continue;
        }

        let surface = fact.event.junction;
        let ready_start = ready.len();
        append_existing_ready_paths(
            body,
            surface,
            fact.event.at,
            fact.event.cause,
            ready,
            connected_outcomes,
        );
        fact.had_ready_path = ready.len() > ready_start;
        let mut next = body
            .arena
            .junction(surface)
            .and_then(|junction| junction.outgoing_head);
        while let Some(morphology_id) = next {
            let morphology = *body.arena.link(morphology_id).expect("live link");
            next = morphology.next;
            if morphology.impulse != 0
                || body.link_memory[morphology_id.slot()].role != LinkRole::Drive
                || !(1..=LOCAL_RADIUS as Time).contains(&morphology.delay)
            {
                continue;
            }
            for sign in [1_i8, -1_i8] {
                if ready_path_exists(body, &ready[ready_start..], morphology.to, sign) {
                    continue;
                }
                let middle = change.new_junction();
                change.push(Edit::AddJunction {
                    new: middle,
                    spec: Junction::integrating(1),
                });
                let first = change.new_link();
                change.push(Edit::AddLink {
                    new: first,
                    from: surface.into(),
                    to: middle.into(),
                    spec: LinkSpec {
                        delay: morphology.delay,
                        impulse: 1,
                        trigger: Trigger::SourceFires,
                        role: LinkRole::PathEntry,
                    },
                });
                let second = change.new_link();
                change.push(Edit::AddLink {
                    new: second,
                    from: middle.into(),
                    to: morphology.to.into(),
                    spec: LinkSpec {
                        delay: morphology.delay,
                        impulse: i32::from(sign),
                        trigger: Trigger::SourceFires,
                        role: LinkRole::Drive,
                    },
                });
                let connected_start = connected_outcomes.len();
                append_outcome_sources(body, morphology.to, connected_outcomes);
                let connected_end = connected_outcomes.len();
                ready.push(ReadyPath {
                    surface,
                    middle: middle.into(),
                    output: morphology.to,
                    first: first.into(),
                    second: second.into(),
                    at: fact.event.at,
                    current_cause: fact.event.cause,
                    return_cause: None,
                    unanswered: false,
                    connected_start,
                    connected_end,
                    outcome: None,
                    participation: 0,
                    strength: 1,
                    stable_order: u32::try_from(body.arena.link_count())
                        .unwrap_or(u32::MAX)
                        .saturating_add(second.0),
                    executable: path_is_executable(body, surface, false),
                });
            }
        }
    }

    fill_ready_worlds(ready, connected_outcomes, worlds);
    for world in 0..ready.len() {
        if worlds[world] == world {
            if let Some(winner) = choose_ready(ready, worlds, world, construction) {
                winners.push(winner);
            }
        }
    }
    winners.sort_by_key(|choice| ready[choice.winner].surface);
    if T::ENABLED {
        for (index, candidate) in ready.iter().enumerate() {
            trace.record(TraceEvent::Candidate(CandidateTrace {
                at: candidate.at,
                cause: candidate.current_cause,
                group: worlds[index],
                path: candidate.trace_path(),
                connected_outcomes: connected_outcomes
                    [candidate.connected_start..candidate.connected_end]
                    .to_vec(),
                executable: candidate.executable,
                return_cause: candidate.return_cause,
                unanswered: candidate.unanswered,
                outcome: candidate.outcome,
                participation: candidate.participation,
                strength: candidate.strength,
                new_path: matches!(candidate.first, LinkRef::New(_)),
            }));
        }
        for world in 0..ready.len() {
            if worlds[world] != world {
                continue;
            }
            let choice = winners.iter().find(|choice| worlds[choice.winner] == world);
            let at = ready
                .iter()
                .enumerate()
                .find(|(index, _)| worlds[*index] == world)
                .map_or(0, |(_, candidate)| candidate.at);
            trace.record(TraceEvent::Choice(ChoiceTrace {
                at,
                group: world,
                alternatives: worlds.iter().filter(|group| **group == world).count(),
                winner: choice.map(|choice| {
                    let winner = &ready[choice.winner];
                    winner.trace_path()
                }),
                basis: choice.map(|choice| choice.basis),
                sent: choice.is_some() && !construction,
            }));
        }
    }
    if construction {
        return;
    }
    for winner in winners.iter().map(|choice| &ready[choice.winner]) {
        change.push(Edit::Send {
            through: winner.first,
            at: winner.at,
            cause: winner.current_cause,
        });
        if winner
            .outcome
            .is_some_and(|outcome| outcome.available_until_choice)
        {
            if let LinkRef::Existing(first) = winner.first {
                consume_path_outcome(body, change, first);
            }
        }
    }
}

fn append_existing_ready_paths(
    body: ReactionView<'_>,
    surface: JunctionId,
    at: Time,
    current_cause: Cause,
    paths: &mut Vec<ReadyPath>,
    connected_outcomes: &mut Vec<JunctionId>,
) {
    let mut next = body
        .arena
        .junction(surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(first_id) = next {
        let first = *body.arena.link(first_id).expect("live link");
        next = first.next;
        let first_memory = &body.link_memory[first_id.slot()];
        if !first_memory.live || first_memory.role != LinkRole::PathEntry {
            continue;
        }
        let mut second = body
            .arena
            .junction(first.to)
            .and_then(|junction| junction.outgoing_head);
        while let Some(second_id) = second {
            let link = *body.arena.link(second_id).expect("live link");
            second = link.next;
            let memory = &body.link_memory[second_id.slot()];
            if memory.live && memory.role == LinkRole::Drive && link.impulse != 0 {
                let connected_start = connected_outcomes.len();
                append_connected_outcomes(body, first.to, link.to, connected_outcomes);
                let connected_end = connected_outcomes.len();
                let outcome = memory.outcome_at.map(|at| Outcome {
                    at,
                    caused_transition: true,
                    available_until_choice: memory.outcome_available,
                });
                paths.push(ReadyPath {
                    surface,
                    middle: first.to.into(),
                    output: link.to,
                    first: first_id.into(),
                    second: second_id.into(),
                    at,
                    current_cause,
                    return_cause: (memory.participation > 0).then_some(memory.cause),
                    unanswered: path_has_open_return(body, first.to, link.to),
                    connected_start,
                    connected_end,
                    outcome,
                    participation: memory.participation,
                    strength: memory.strength,
                    stable_order: second_id.slot() as u32,
                    executable: path_is_executable(body, surface, outcome.is_some()),
                });
            }
        }
    }
}

fn ready_path_exists(
    body: ReactionView<'_>,
    ready: &[ReadyPath],
    output: JunctionId,
    sign: i8,
) -> bool {
    ready.iter().any(|path| {
        let LinkRef::Existing(first) = path.first else {
            return false;
        };
        let middle = body.arena.link(first).expect("live path entry").to;
        let mut second = body
            .arena
            .junction(middle)
            .and_then(|junction| junction.outgoing_head);
        while let Some(second_id) = second {
            let link = body.arena.link(second_id).expect("live link");
            if body.link_memory[second_id.slot()].role == LinkRole::Drive
                && link.to == output
                && link.impulse.signum() == i32::from(sign)
            {
                return true;
            }
            second = link.next;
        }
        false
    })
}

fn choose_ready(
    paths: &[ReadyPath],
    worlds: &[usize],
    world: usize,
    construction: bool,
) -> Option<ReadyChoice> {
    let in_world =
        |index: &usize| worlds[*index] == world && (construction || paths[*index].executable);
    let output_is_unanswered = |output| {
        (0..paths.len()).any(|index| {
            in_world(&index) && paths[index].output == output && paths[index].unanswered
        })
    };
    let has_unanswered_output =
        (0..paths.len()).any(|index| in_world(&index) && output_is_unanswered(paths[index].output));
    let has_other_output = (0..paths.len())
        .any(|index| in_world(&index) && !output_is_unanswered(paths[index].output));
    let release_unanswered_output = has_unanswered_output && has_other_output;
    unique_ready((0..paths.len()).filter(in_world).filter(|index| {
        let path = &paths[*index];
        path.return_cause.is_some() && path.return_cause == Some(path.current_cause)
    }))
    .map(|winner| ReadyChoice {
        winner,
        basis: ChoiceBasis::CurrentReturn,
    })
    .or_else(|| {
        unique_latest_ready(paths, worlds, world, true, construction).map(|winner| ReadyChoice {
            winner,
            basis: ChoiceBasis::AvailableOutcome,
        })
    })
    .or_else(|| {
        unique_latest_ready(paths, worlds, world, false, construction).map(|winner| ReadyChoice {
            winner,
            basis: ChoiceBasis::LatestOutcome,
        })
    })
    .or_else(|| {
        (0..paths.len())
            .filter(in_world)
            .filter(|index| {
                !release_unanswered_output || !output_is_unanswered(paths[*index].output)
            })
            .max_by_key(|index| {
                let path = &paths[*index];
                (
                    path.participation,
                    path.strength,
                    Reverse(path.stable_order),
                )
            })
            .map(|winner| ReadyChoice {
                winner,
                basis: if release_unanswered_output {
                    ChoiceBasis::UnansweredOutputRelease
                } else {
                    ChoiceBasis::ParticipationAndStrength
                },
            })
    })
}

fn path_has_open_return(body: ReactionView<'_>, middle: JunctionId, output: JunctionId) -> bool {
    let mut next = body
        .arena
        .junction(output)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link) = next {
        let physical = body.arena.link(link).expect("live link");
        next = physical.next;
        if physical.to == middle
            && body.link_memory[link.slot()].live
            && matches!(body.link_memory[link.slot()].role, LinkRole::Return { .. })
        {
            return true;
        }
    }
    false
}

fn unique_ready(mut paths: impl Iterator<Item = usize>) -> Option<usize> {
    let path = paths.next()?;
    paths.next().is_none().then_some(path)
}

fn unique_latest_ready(
    paths: &[ReadyPath],
    worlds: &[usize],
    world: usize,
    available_only: bool,
    construction: bool,
) -> Option<usize> {
    let latest = (0..paths.len())
        .filter(|index| worlds[*index] == world && (construction || paths[*index].executable))
        .filter_map(|index| {
            paths[index]
                .outcome
                .filter(|outcome| !available_only || outcome.available_until_choice)
        })
        .map(|outcome| outcome.at)
        .max()?;
    unique_ready((0..paths.len()).filter(|index| {
        if worlds[*index] != world || (!construction && !paths[*index].executable) {
            return false;
        }
        let path = &paths[*index];
        path.outcome.is_some_and(|outcome| {
            outcome.at == latest && (!available_only || outcome.available_until_choice)
        })
    }))
}

fn append_connected_outcomes(
    body: ReactionView<'_>,
    middle: JunctionId,
    output: JunctionId,
    outcomes: &mut Vec<JunctionId>,
) {
    append_outcome_sources(body, middle, outcomes);
    append_outcome_sources(body, output, outcomes);
}

fn append_outcome_sources(
    body: ReactionView<'_>,
    junction: JunctionId,
    outcomes: &mut Vec<JunctionId>,
) {
    outcomes.extend(
        body.arena
            .incoming(junction)
            .filter(|link| {
                body.link_memory[link.slot()].live
                    && body.link_memory[link.slot()].role == LinkRole::OutcomeWitness
            })
            .filter_map(|link| body.arena.link(link).map(|physical| physical.from)),
    );
}

fn fill_ready_worlds(
    paths: &[ReadyPath],
    connected_outcomes: &[JunctionId],
    parents: &mut Vec<usize>,
) {
    parents.extend(0..paths.len());
    for right in 0..paths.len() {
        for left in 0..right {
            let same_surface = paths[left].surface == paths[right].surface;
            let left_outcomes =
                &connected_outcomes[paths[left].connected_start..paths[left].connected_end];
            let right_outcomes =
                &connected_outcomes[paths[right].connected_start..paths[right].connected_end];
            let connected_outcome = left_outcomes
                .iter()
                .any(|source| right_outcomes.contains(source));
            if same_surface || connected_outcome {
                union(parents, left, right);
            }
        }
    }
    for index in 0..parents.len() {
        parents[index] = find(parents, index);
    }
}

fn consume_path_outcome(body: ReactionView<'_>, change: &mut Change, first: LinkId) {
    change.push(Edit::ChangeLink {
        link: first.into(),
        change: LinkChange::ConsumeOutcome,
    });
    let middle = body.arena.link(first).expect("live path entry").to;
    let mut next = body
        .arena
        .junction(middle)
        .and_then(|junction| junction.outgoing_head);
    while let Some(second) = next {
        let link = body.arena.link(second).expect("live link");
        next = link.next;
        if body.link_memory[second.slot()].role == LinkRole::Drive && link.impulse != 0 {
            change.push(Edit::ChangeLink {
                link: second.into(),
                change: LinkChange::ConsumeOutcome,
            });
            break;
        }
    }
}

fn surface_fired(surface: Surface<'_>) -> Change {
    let mut change = Change::empty();
    if surface.boundary_effect
        || surface.output
        || surface.learned_intermediate
        || !(surface.external || surface.maximally_resistant)
    {
        return change;
    }
    for &(output, distance) in surface.nearby_outputs {
        if !(1..=LOCAL_RADIUS).contains(&distance) {
            continue;
        }
        for sign in [1_i8, -1_i8] {
            if surface.existing_paths.contains(&(output, sign)) {
                continue;
            }
            let middle = change.new_junction();
            change.push(Edit::AddJunction {
                new: middle,
                spec: Junction::integrating(1),
            });
            for (from, to, impulse, role) in [
                (surface.id.into(), middle.into(), 1, LinkRole::PathEntry),
                (
                    middle.into(),
                    output.into(),
                    i32::from(sign),
                    LinkRole::Drive,
                ),
            ] {
                let new = change.new_link();
                change.push(Edit::AddLink {
                    new,
                    from,
                    to,
                    spec: LinkSpec {
                        delay: distance as Time,
                        impulse,
                        trigger: Trigger::SourceFires,
                        role,
                    },
                });
            }
        }
    }
    change
}

fn paths_ready(
    at: Time,
    candidates: &[Candidate<'_>],
    current_transition: Option<Cause>,
) -> Change {
    let mut change = Change::empty();
    let worlds = connected_worlds(candidates);
    for world in 0..candidates.len() {
        if worlds[world] != world {
            continue;
        }
        let local = candidates
            .iter()
            .zip(&worlds)
            .filter_map(|(candidate, root)| (*root == world).then_some(candidate))
            .collect::<Vec<_>>();
        let Some(winner) = choose_one(&local, current_transition) else {
            continue;
        };
        change.push(Edit::Send {
            through: winner.path.first.into(),
            at,
            cause: current_transition.unwrap_or_default(),
        });
        if winner
            .outcome
            .is_some_and(|outcome| outcome.available_until_choice)
        {
            for link in winner.path.links() {
                change.push(Edit::ChangeLink {
                    link: link.into(),
                    change: LinkChange::ConsumeOutcome,
                });
            }
        }
    }
    change
}

fn output_fired(at: Time, used: UsedPath) -> Change {
    let mut change = Change::empty();
    for link in used.path.links() {
        change.push(Edit::ChangeLink {
            link: link.into(),
            change: LinkChange::Participated {
                cause: used.cause,
                at,
            },
        });
    }
    let new = change.new_link();
    change.push(Edit::AddLink {
        new,
        from: used.path.output.into(),
        to: used.path.middle.into(),
        spec: LinkSpec {
            delay: 0,
            impulse: 1,
            trigger: Trigger::SourceFires,
            role: LinkRole::Return {
                cause: used.cause,
                cohort: used.cause,
            },
        },
    });
    change
}

fn outcomes_returned(outcomes: &[ReturnedOutcome<'_>]) -> Change {
    let mut change = Change::empty();
    let mut accepted = BTreeSet::new();
    let mut returns = BTreeSet::new();
    for outcome in outcomes {
        returns.extend(outcome.return_links.iter().copied());
        let Some(at) = outcome.transition_at else {
            continue;
        };
        if at <= outcome.return_opened_at || !accepted.insert(outcome.used.cause) {
            continue;
        }
        for link in outcome.used.path.links() {
            change.push(Edit::ChangeLink {
                link: link.into(),
                change: LinkChange::RememberOutcome {
                    at,
                    available_until_choice: true,
                },
            });
            change.push(Edit::ChangeLink {
                link: link.into(),
                change: LinkChange::Strengthen { amount: 1 },
            });
        }
    }
    for link in returns {
        change.push(Edit::ChangeLink {
            link: link.into(),
            change: LinkChange::Retire,
        });
    }
    change
}

fn boundary_closed(at: Time, closure: Closure<'_>) -> Change {
    let mut change = Change::empty();
    let members = closure
        .participating_members
        .iter()
        .copied()
        .filter(|member| !closure.parent_members.contains(member))
        .collect::<Vec<_>>();
    if members.is_empty() {
        return change;
    }
    let boundary = change.new_junction();
    change.push(Edit::AddJunction {
        new: boundary,
        spec: Junction::integrating(1),
    });
    for member in members {
        let new = change.new_link();
        change.push(Edit::AddLink {
            new,
            from: boundary.into(),
            to: member.into(),
            spec: LinkSpec {
                delay: 0,
                impulse: 1,
                trigger: Trigger::SourceFires,
                role: LinkRole::Membership,
            },
        });
    }
    if let Some((path, outcome_at)) = closure.live_witness {
        if outcome_at == at {
            for link in path.links() {
                change.push(Edit::ChangeLink {
                    link: link.into(),
                    change: LinkChange::RememberOutcome {
                        at,
                        available_until_choice: true,
                    },
                });
            }
        }
    }
    change
}

fn natural_cycle(paths: &[CyclePath]) -> Change {
    let mut matches = paths.iter().filter(|path| {
        path.participated
            && path
                .transition_at
                .is_some_and(|transition| transition > path.opened_at)
    });
    let Some(path) = matches.next() else {
        return Change::empty();
    };
    if matches.next().is_some() {
        return Change::empty();
    }
    let mut change = Change::empty();
    let at = path.transition_at.expect("matched transition");
    for link in path.used.path.links() {
        change.push(Edit::ChangeLink {
            link: link.into(),
            change: LinkChange::RememberOutcome {
                at,
                available_until_choice: true,
            },
        });
        change.push(Edit::ChangeLink {
            link: link.into(),
            change: LinkChange::Strengthen { amount: 1 },
        });
    }
    change
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ReturnEntry {
    pub(crate) cause: Cause,
    pub(crate) link: LinkId,
}

impl ReturnEntry {
    fn remapped(self, link_base: usize) -> Self {
        Self {
            cause: self.cause,
            link: LinkId::new(link_base + self.link.slot())
                .expect("validated attachment return identity"),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct LinkMemory {
    pub(crate) live: bool,
    pub(crate) transmitted: bool,
    pub(crate) role: LinkRole,
    pub(crate) participation: u64,
    pub(crate) cause: Cause,
    pub(crate) participated_at: Time,
    pub(crate) outcome_at: Option<Time>,
    pub(crate) outcome_available: bool,
    pub(crate) strength: i64,
}

impl Default for LinkMemory {
    fn default() -> Self {
        Self {
            live: true,
            transmitted: false,
            role: LinkRole::Drive,
            participation: 0,
            cause: 0,
            participated_at: 0,
            outcome_at: None,
            outcome_available: false,
            strength: 1,
        }
    }
}

impl LinkMemory {
    pub(crate) fn record_transmission(&mut self, cause: Cause, at: Time) {
        self.transmitted = true;
        self.cause = cause;
        self.participated_at = at;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApplyError {
    Build(BuildError),
    Run(RunError),
    UnknownJunction(JunctionId),
    UnknownLink(LinkId),
    ForwardJunction(NewJunction),
    ForwardLink(NewLink),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Applied {
    pub junctions: Vec<JunctionId>,
    pub links: Vec<LinkId>,
}

impl Body {
    fn insert_live_return(&mut self, link: LinkId, role: LinkRole) {
        let LinkRole::Return { cause, .. } = role else {
            return;
        };
        let entry = ReturnEntry { cause, link };
        match self.live_returns.binary_search(&entry) {
            Ok(_) => {}
            Err(index) => self.live_returns.insert(index, entry),
        }
    }

    fn remove_live_return(&mut self, link: LinkId, role: LinkRole) {
        let LinkRole::Return { cause, .. } = role else {
            return;
        };
        let entry = ReturnEntry { cause, link };
        if let Ok(index) = self.live_returns.binary_search(&entry) {
            self.live_returns.remove(index);
        } else {
            debug_assert!(false, "live return index drift");
        }
    }

    pub(crate) fn append_live_returns(
        &mut self,
        returns: impl IntoIterator<Item = ReturnEntry>,
        link_base: usize,
    ) {
        self.live_returns
            .extend(returns.into_iter().map(|entry| entry.remapped(link_base)));
        self.live_returns.sort_unstable();
        debug_assert!(self
            .live_returns
            .windows(2)
            .all(|entries| entries[0] != entries[1]));
    }

    pub fn set_link_impulse(&mut self, link: LinkId, impulse: Impulse) -> Result<(), ApplyError> {
        let physical = self
            .arena
            .link_mut(link)
            .ok_or(ApplyError::UnknownLink(link))?;
        physical.impulse = impulse;
        Ok(())
    }

    pub fn set_link_role(&mut self, link: LinkId, role: LinkRole) -> Result<(), ApplyError> {
        let memory = self
            .link_memory
            .get(link.slot())
            .ok_or(ApplyError::UnknownLink(link))?;
        let live = memory.live;
        let previous = memory.role;
        if live {
            self.remove_live_return(link, previous);
        }
        self.link_memory[link.slot()].role = role;
        if live {
            self.insert_live_return(link, role);
        }
        Ok(())
    }

    pub fn apply(&mut self, mut change: Change) -> Result<Applied, ApplyError> {
        let mut applied = Applied::default();
        let at = self.now();
        self.apply_reusing(&mut change, &mut applied, at, &mut NoTrace)?;
        Ok(applied)
    }

    pub(crate) fn apply_reusing<T: TraceSink>(
        &mut self,
        change: &mut Change,
        applied: &mut Applied,
        at: Time,
        trace: &mut T,
    ) -> Result<(), ApplyError> {
        applied.junctions.clear();
        applied.links.clear();
        let mut junctions = 0_usize;
        let mut links = 0_usize;
        for edit in &change.edits {
            match edit {
                Edit::AddJunction { new, .. } => {
                    if new.0 as usize != junctions {
                        return Err(ApplyError::ForwardJunction(*new));
                    }
                    junctions += 1;
                }
                Edit::AddLink { new, from, to, .. } => {
                    if new.0 as usize != links {
                        return Err(ApplyError::ForwardLink(*new));
                    }
                    validate_junction(self, *from, junctions)?;
                    validate_junction(self, *to, junctions)?;
                    links += 1;
                }
                Edit::Send { through, .. } | Edit::ChangeLink { link: through, .. } => {
                    validate_link(self, *through, links)?;
                }
            }
        }
        if !self.arena.has_junction_capacity(junctions) {
            return Err(ApplyError::Build(BuildError::CapacityExhausted));
        }
        if !self.arena.has_link_capacity(links) {
            return Err(ApplyError::Build(BuildError::CapacityExhausted));
        }

        for edit in change.edits.drain(..) {
            match edit {
                Edit::AddJunction { new, spec } => {
                    if new.0 as usize != applied.junctions.len() {
                        return Err(ApplyError::ForwardJunction(new));
                    }
                    let id = self.add_junction(spec).map_err(ApplyError::Build)?;
                    applied.junctions.push(id);
                }
                Edit::AddLink {
                    new,
                    from,
                    to,
                    spec,
                } => {
                    if new.0 as usize != applied.links.len() {
                        return Err(ApplyError::ForwardLink(new));
                    }
                    let from = resolve_junction(from, applied)?;
                    let to = resolve_junction(to, applied)?;
                    let id = self
                        .add_link(Link::new(from, to, spec.delay, spec.impulse).when(spec.trigger))
                        .map_err(ApplyError::Build)?;
                    self.set_link_role(id, spec.role)?;
                    applied.links.push(id);
                }
                Edit::Send { through, at, cause } => {
                    let link = resolve_link(through, applied)?;
                    self.send_through(at, link, cause)
                        .map_err(ApplyError::Run)?;
                }
                Edit::ChangeLink { link, change } => {
                    let link = resolve_link(link, applied)?;
                    if change == LinkChange::Retire {
                        let memory = self
                            .link_memory
                            .get(link.slot())
                            .ok_or(ApplyError::UnknownLink(link))?;
                        let was_live = memory.live;
                        let role = memory.role;
                        if was_live {
                            self.remove_live_return(link, role);
                            self.link_memory[link.slot()].live = false;
                        }
                        continue;
                    }
                    let memory = self
                        .link_memory
                        .get_mut(link.slot())
                        .ok_or(ApplyError::UnknownLink(link))?;
                    match change {
                        LinkChange::Participated { cause, at } => {
                            memory.participation = memory.participation.saturating_add(1);
                            memory.cause = cause;
                            memory.participated_at = at;
                        }
                        LinkChange::RememberOutcome {
                            at,
                            available_until_choice,
                        } => {
                            memory.outcome_at = Some(at);
                            memory.outcome_available = available_until_choice;
                        }
                        LinkChange::ConsumeOutcome => memory.outcome_available = false,
                        LinkChange::Strengthen { amount } => {
                            let before = memory.strength;
                            let after = before.saturating_add(i64::from(amount));
                            memory.strength = after;
                            if T::ENABLED {
                                trace.record(TraceEvent::Strengthened(StrengthTrace {
                                    at,
                                    link,
                                    before,
                                    after,
                                }));
                            }
                        }
                        LinkChange::Retire => unreachable!("retirement handled before mutation"),
                    }
                }
            }
        }
        Ok(())
    }
}

fn validate_junction(
    body: &Body,
    reference: JunctionRef,
    new_count: usize,
) -> Result<(), ApplyError> {
    match reference {
        JunctionRef::Existing(id) => body.arena.require(id).map_err(ApplyError::UnknownJunction),
        JunctionRef::New(id) if (id.0 as usize) < new_count => Ok(()),
        JunctionRef::New(id) => Err(ApplyError::ForwardJunction(id)),
    }
}

fn validate_link(body: &Body, reference: LinkRef, new_count: usize) -> Result<(), ApplyError> {
    match reference {
        LinkRef::Existing(id) if body.arena.link(id).is_some() => Ok(()),
        LinkRef::Existing(id) => Err(ApplyError::UnknownLink(id)),
        LinkRef::New(id) if (id.0 as usize) < new_count => Ok(()),
        LinkRef::New(id) => Err(ApplyError::ForwardLink(id)),
    }
}

fn resolve_junction(reference: JunctionRef, applied: &Applied) -> Result<JunctionId, ApplyError> {
    match reference {
        JunctionRef::Existing(id) => Ok(id),
        JunctionRef::New(id) => applied
            .junctions
            .get(id.0 as usize)
            .copied()
            .ok_or(ApplyError::ForwardJunction(id)),
    }
}

fn resolve_link(reference: LinkRef, applied: &Applied) -> Result<LinkId, ApplyError> {
    match reference {
        LinkRef::Existing(id) => Ok(id),
        LinkRef::New(id) => applied
            .links
            .get(id.0 as usize)
            .copied()
            .ok_or(ApplyError::ForwardLink(id)),
    }
}

fn connected_worlds(candidates: &[Candidate<'_>]) -> Vec<usize> {
    let mut parents = (0..candidates.len()).collect::<Vec<_>>();
    for right in 0..candidates.len() {
        for left in 0..right {
            let a = candidates[left].connected_outcomes;
            let b = candidates[right].connected_outcomes;
            if a.is_empty() || b.is_empty() || a.iter().any(|source| b.contains(source)) {
                union(&mut parents, left, right);
            }
        }
    }
    (0..parents.len())
        .map(|index| find(&mut parents, index))
        .collect()
}

fn find(parents: &mut [usize], mut index: usize) -> usize {
    while parents[index] != index {
        parents[index] = parents[parents[index]];
        index = parents[index];
    }
    index
}

fn union(parents: &mut [usize], left: usize, right: usize) {
    let left = find(parents, left);
    let right = find(parents, right);
    let root = left.min(right);
    parents[left] = root;
    parents[right] = root;
}

fn choose_one<'a>(
    candidates: &'a [&'a Candidate<'_>],
    current_transition: Option<Cause>,
) -> Option<&'a Candidate<'a>> {
    let executable = |candidate: &&Candidate| candidate.executable;
    unique(
        candidates
            .iter()
            .copied()
            .filter(executable)
            .filter(|candidate| {
                candidate.return_cause.is_some() && candidate.return_cause == current_transition
            }),
    )
    .or_else(|| {
        unique_latest(candidates, |outcome| {
            outcome.caused_transition && outcome.available_until_choice
        })
    })
    .or_else(|| unique_latest(candidates, |_| true))
    .or_else(|| {
        unique(
            candidates
                .iter()
                .copied()
                .filter(executable)
                .filter(|candidate| {
                    candidate.unanswered
                        && match (candidate.opportunity_from, candidate.owner) {
                            (Some(from), to) if from == to => true,
                            (Some(Owner::Organism), Owner::Learner { root: true, .. }) => true,
                            _ => false,
                        }
                }),
        )
    })
    .or_else(|| {
        candidates
            .iter()
            .copied()
            .filter(executable)
            .max_by_key(|candidate| {
                (
                    candidate.participation,
                    candidate.strength,
                    Reverse(candidate.stable_order),
                )
            })
    })
}

fn unique<'a>(mut values: impl Iterator<Item = &'a Candidate<'a>>) -> Option<&'a Candidate<'a>> {
    let value = values.next()?;
    values.next().is_none().then_some(value)
}

fn unique_latest<'a>(
    candidates: &'a [&Candidate<'a>],
    accepts: impl Fn(Outcome) -> bool,
) -> Option<&'a Candidate<'a>> {
    let latest = candidates
        .iter()
        .copied()
        .filter(|candidate| candidate.executable)
        .filter_map(|candidate| candidate.outcome.filter(|outcome| accepts(*outcome)))
        .map(|outcome| outcome.at)
        .max()?;
    unique(candidates.iter().copied().filter(|candidate| {
        candidate.executable
            && candidate
                .outcome
                .is_some_and(|outcome| accepts(outcome) && outcome.at == latest)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harness::{
        attach_outcome_component, attach_sensor, finish, motor, reading, schedule,
    };
    use crate::Arrival;

    fn membership(body: &mut Body, parent: JunctionId, member: JunctionId) {
        let link = body.add_link(Link::new(parent, member, 0, 1)).unwrap();
        body.set_link_role(link, LinkRole::Membership).unwrap();
    }

    #[test]
    fn membership_parent_resolution_composes_recursively() {
        let mut body = Body::default();
        let members: [JunctionId; 4] =
            std::array::from_fn(|_| body.add_junction(Junction::integrating(1)).unwrap());
        let root = body.add_junction(Junction::integrating(1)).unwrap();
        membership(&mut body, root, members[0]);
        membership(&mut body, root, members[1]);
        let child = body.add_junction(Junction::integrating(1)).unwrap();
        membership(&mut body, child, root);
        membership(&mut body, child, members[2]);

        let view = ReactionView::new(&body.arena, &body.link_memory, &body.live_returns);
        let mut scratch = ConstructionScratch::default();
        assert_eq!(
            resolve_membership_parent(
                view,
                &members[..2],
                &mut scratch.candidates,
                &mut scratch.stack,
                &mut scratch.visited,
                &mut scratch.leaves,
                &mut scratch.parent_members,
            ),
            MembershipParent::Existing(root)
        );
        assert_eq!(
            resolve_membership_parent(
                view,
                &members[..3],
                &mut scratch.candidates,
                &mut scratch.stack,
                &mut scratch.visited,
                &mut scratch.leaves,
                &mut scratch.parent_members,
            ),
            MembershipParent::Existing(child)
        );
        assert_eq!(scratch.parent_members.len(), 3);
        assert!(members[..3]
            .iter()
            .all(|member| scratch.parent_members.contains(member)));
    }

    #[test]
    fn live_return_index_tracks_roles_order_and_retirement() {
        let mut body = Body::default();
        let from = body.add_junction(Junction::integrating(1)).unwrap();
        let to = body.add_junction(Junction::integrating(1)).unwrap();
        let first = body.add_link(Link::new(from, to, 0, 0)).unwrap();
        let second = body.add_link(Link::new(from, to, 0, 0)).unwrap();
        let third = body.add_link(Link::new(from, to, 0, 0)).unwrap();

        body.set_link_role(
            second,
            LinkRole::Return {
                cause: 9,
                cohort: 9,
            },
        )
        .unwrap();
        body.set_link_role(
            first,
            LinkRole::Return {
                cause: 3,
                cohort: 3,
            },
        )
        .unwrap();
        body.set_link_role(
            third,
            LinkRole::Return {
                cause: 9,
                cohort: 9,
            },
        )
        .unwrap();

        assert_eq!(
            body.live_returns,
            [
                ReturnEntry {
                    cause: 3,
                    link: first,
                },
                ReturnEntry {
                    cause: 9,
                    link: second,
                },
                ReturnEntry {
                    cause: 9,
                    link: third,
                },
            ]
        );
        assert_eq!(body.clone().live_returns, body.live_returns);

        let mut change = Change::empty();
        change.push(Edit::ChangeLink {
            link: second.into(),
            change: LinkChange::Retire,
        });
        body.apply(change).unwrap();
        body.set_link_role(first, LinkRole::Drive).unwrap();

        assert_eq!(
            body.live_returns,
            [ReturnEntry {
                cause: 9,
                link: third,
            }]
        );
    }

    #[test]
    fn accepted_return_updates_memory_without_growing_morphology() {
        let mut body = Body::default();
        let motor = motor(&mut body);
        let sensor = attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        );
        let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        schedule(&mut body, 0, &[reading(outcome, 0, 0, 0)]);
        finish(&mut body);
        schedule(&mut body, 1, &[reading(sensor, 0, 1, 1)]);
        schedule(&mut body, 2, &[Arrival::caused(motor.opportunity, 1, 1)]);
        finish(&mut body);

        let links_before_return = body.arena.link_count();
        schedule(&mut body, 20, &[Arrival::caused(outcome, 1, 2)]);
        finish(&mut body);

        assert_eq!(body.arena.link_count(), links_before_return);
    }
}
