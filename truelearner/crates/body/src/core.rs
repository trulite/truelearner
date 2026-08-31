//! What changes because of one observed physical event.

use crate::{
    arena::Arena,
    engine::PhysicalMoment,
    trace::{
        CandidateTrace, ChoiceBasis, ChoiceTrace, FreshOpportunityTrace, NoTrace, ReentryStepTrace,
        ReentryTrace, ReturnCandidateTrace, ReturnDecision, ReturnTrace, StrengthTrace, TraceEvent,
        TracePath, TraceSink,
    },
    Body, BuildError, Impulse, Junction, JunctionId, Link, LinkId, Retention, RunError, Time,
    Trigger,
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap},
};

pub type Cause = u64;
pub type Cohort = u64;
pub type Boundary = u32;
const LOCAL_RADIUS: i32 = 2;
const AUTOMATIC_AFTER_EXACT_CLOSURES: u8 = 3;
const MAX_REENTRY_DEPTH: usize = 16;
const MAX_REENTRY_INCIDENCE_VISITS: u16 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Path {
    pub surface: JunctionId,
    pub middle: JunctionId,
    pub output: JunctionId,
    pub first: LinkId,
    pub second: LinkId,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomaticityWork {
    pub pair_observations: u64,
    pub exact_closure_updates: u64,
    pub composites_formed: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomaticityState {
    pub open_witnesses: usize,
    pub candidate_pairs: usize,
    pub has_recursive_composites: bool,
}

impl AutomaticityWork {
    pub const fn total(self) -> u64 {
        self.pair_observations
            .saturating_add(self.exact_closure_updates)
            .saturating_add(self.composites_formed)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct AutomaticPair {
    first: LinkId,
    second: LinkId,
}

impl AutomaticPair {
    fn remap_links(&mut self, base: usize) {
        self.first = remap_link(self.first, base);
        self.second = remap_link(self.second, base);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct AutomaticWitness {
    returned: LinkId,
    path: Path,
    cause: Cause,
    pairs: Vec<AutomaticPair>,
}

impl AutomaticWitness {
    fn remap_links(&mut self, base: usize) {
        self.returned = remap_link(self.returned, base);
        self.path.first = remap_link(self.path.first, base);
        self.path.second = remap_link(self.path.second, base);
        for pair in &mut self.pairs {
            pair.remap_links(base);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct AutomaticEvidence {
    owner: LinkId,
    pair: AutomaticPair,
    exact_closures: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Automaticity {
    pub(crate) closure_maintenance: bool,
    witnesses: Vec<AutomaticWitness>,
    evidence: Vec<AutomaticEvidence>,
    pub(crate) generic_composites: bool,
    work: AutomaticityWork,
}

impl Automaticity {
    pub(crate) fn remap_links(&mut self, base: usize) {
        for witness in &mut self.witnesses {
            witness.remap_links(base);
        }
        for evidence in &mut self.evidence {
            evidence.owner = remap_link(evidence.owner, base);
            evidence.pair.remap_links(base);
        }
    }

    pub(crate) fn append(&mut self, mut other: Self) {
        self.closure_maintenance |= other.closure_maintenance;
        self.witnesses.append(&mut other.witnesses);
        self.evidence.append(&mut other.evidence);
        self.generic_composites |= other.generic_composites;
        self.work.pair_observations = self
            .work
            .pair_observations
            .saturating_add(other.work.pair_observations);
        self.work.exact_closure_updates = self
            .work
            .exact_closure_updates
            .saturating_add(other.work.exact_closure_updates);
        self.work.composites_formed = self
            .work
            .composites_formed
            .saturating_add(other.work.composites_formed);
    }
}

fn remap_link(link: LinkId, base: usize) -> LinkId {
    LinkId::new(base + link.slot()).expect("validated attachment link identity")
}

impl Path {
    const fn links(self) -> [LinkId; 2] {
        [self.first, self.second]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReentryState {
    pub closed_steps: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ClosedStep {
    link: LinkId,
    path: Path,
    returned_source: JunctionId,
    outcome_witness: LinkId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct NewJunction(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct NewLink(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkRole {
    #[default]
    Drive,
    PathEntry,
    /// A repeatedly closed two-link path retained as one ordinary physical
    /// occurrence. The parents remain the causal support and are used again
    /// whenever the composite reaches an output.
    Composite {
        first: LinkId,
        second: LinkId,
    },
    Return {
        cause: Cause,
        cohort: Cohort,
    },
    OutcomeWitness,
    Membership,
    /// Incidence from a physical progress source to an output. Unlike an
    /// outcome witness, this can identify an open path without closing it.
    ProgressWitness,
    /// A world-boundary return can close and strengthen a path, but does not
    /// itself offer that path as the next action.
    BoundaryWitness,
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
    LearnOutcome {
        at: Time,
        available_until_choice: bool,
        strength: i32,
    },
    ConsumeOutcome,
    ClearOutcomeSelection,
    InhibitBoundaryChoice,
    ConsumeBoundaryInhibition,
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
    CompleteReturn {
        source: JunctionId,
        returned: LinkId,
        path: Path,
        outcome_witness: Option<LinkId>,
        exact: bool,
        exclusive_source: bool,
        offers_choice: bool,
        at: Time,
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
    drive: u16,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ReentryContinuation {
    path: Path,
}

#[derive(Clone, Debug, Default)]
struct ReentryScratch {
    present: Vec<JunctionId>,
    steps: Vec<ReentryStepTrace>,
    continuations: Vec<ReentryContinuation>,
}

impl ReentryScratch {
    fn clear(&mut self) {
        self.present.clear();
        self.steps.clear();
        self.continuations.clear();
    }

    fn clear_search(&mut self) {
        self.steps.clear();
        self.continuations.clear();
    }
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
    reentry: ReentryScratch,
    construction: ConstructionScratch,
    pub(crate) change: Change,
    pub(crate) applied: Applied,
}

#[derive(Clone, Copy)]
pub(crate) struct ReactionView<'a> {
    arena: &'a Arena,
    link_memory: &'a [LinkMemory],
    returns: &'a ReturnIndex,
}

impl<'a> ReactionView<'a> {
    pub(crate) const fn new(
        arena: &'a Arena,
        link_memory: &'a [LinkMemory],
        returns: &'a ReturnIndex,
    ) -> Self {
        Self {
            arena,
            link_memory,
            returns,
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
        self.reentry.clear();
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
    if body.returns.live_count != 0 {
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
        if memory.live && closes_return(memory.role) {
            return true;
        }
        next = link.next;
    }
    false
}

fn is_progress_source(body: ReactionView<'_>, junction: JunctionId) -> bool {
    let mut next = body
        .arena
        .junction(junction)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link) = next {
        let physical = body.arena.link(link).expect("live progress incidence");
        next = physical.next;
        if body.link_memory[link.slot()].live
            && body.link_memory[link.slot()].role == LinkRole::ProgressWitness
        {
            return true;
        }
    }
    false
}

const fn closes_return(role: LinkRole) -> bool {
    matches!(role, LinkRole::OutcomeWitness | LinkRole::BoundaryWitness)
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
    if let Some(fact) = return_only_fact(body, moment) {
        record_returned_outcome(body, &fact, &mut scratch.change, trace);
        return;
    }
    scratch
        .facts
        .extend(moment.changes.iter().map(|recorded| MomentFact {
            event: recorded.event,
            drive: event_drive(body, recorded.event),
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
        &mut scratch.reentry,
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

fn return_only_fact(body: ReactionView<'_>, moment: &PhysicalMoment) -> Option<MomentFact> {
    let [recorded] = moment.changes.as_slice() else {
        return None;
    };
    if !recorded.boundary || !matches!(recorded.used, UsedPaths::None) {
        return None;
    }
    let source = recorded.event.junction;
    if body
        .returns
        .by_source
        .get(source.slot())
        .is_none_or(Vec::is_empty)
        || surface_may_choose(body, source)
    {
        return None;
    }
    Some(MomentFact {
        event: recorded.event,
        drive: event_drive(body, recorded.event),
        boundary: true,
        used: UsedPaths::None,
        had_ready_path: false,
    })
}

fn event_drive(body: ReactionView<'_>, event: crate::physics::Event) -> u16 {
    body.arena
        .junction(event.junction)
        .expect("live event junction")
        .drive(event.before, event.after)
}

#[inline(always)]
fn surface_may_choose(body: ReactionView<'_>, surface: JunctionId) -> bool {
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
        record_returned_outcome(body, fact, change, trace);
    }
}

fn record_returned_outcome<T: TraceSink>(
    body: ReactionView<'_>,
    fact: &MomentFact,
    change: &mut Change,
    trace: &mut T,
) {
    let selected = scan_live_returns(body, fact.event.junction, fact.event.cause);
    // One exact physical event may finish the preceding path while starting
    // an already learned next path. A merely coincident ready surface cannot.
    if fact.had_ready_path && selected.exact_total != 1 {
        if T::ENABLED {
            let candidates = trace_return_candidates(body, fact.event.junction);
            trace.record(TraceEvent::Return(ReturnTrace {
                at: fact.event.at,
                source: fact.event.junction,
                incoming_cause: fact.event.cause,
                path: None,
                return_cause: None,
                return_opened_at: None,
                offers_choice: None,
                open_paths: selected.total,
                exact_paths: selected.exact_total,
                candidates,
                decision: ReturnDecision::BlockedByReadyPath,
            }));
        }
        return;
    }

    let decision = match selected.selected {
        Some(returned) if returned.opened_at <= fact.event.at => ReturnDecision::Accepted,
        Some(_) => ReturnDecision::BeforeReturnOpened,
        None if selected.total == 0 => ReturnDecision::NoOpenPath,
        None => ReturnDecision::Ambiguous,
    };
    if T::ENABLED {
        let candidates = trace_return_candidates(body, fact.event.junction);
        trace.record(TraceEvent::Return(ReturnTrace {
            at: fact.event.at,
            source: fact.event.junction,
            incoming_cause: fact.event.cause,
            path: selected.selected.map(|returned| returned.path),
            return_cause: selected.selected.map(|returned| returned.cause),
            return_opened_at: selected.selected.map(|returned| returned.opened_at),
            offers_choice: selected.selected.map(|returned| returned.offers_choice),
            open_paths: selected.total,
            exact_paths: selected.exact_total,
            candidates,
            decision,
        }));
    }
    let accepted = selected
        .selected
        .filter(|returned| returned.opened_at <= fact.event.at);
    if let Some(returned) = accepted.filter(|returned| !returned.offers_choice) {
        clear_output_selection(body, returned.path.output, change);
    }
    for entry in body
        .returns
        .by_source
        .get(fact.event.junction.slot())
        .into_iter()
        .flatten()
        .filter(|entry| open_return(body, **entry).is_some())
    {
        if accepted.is_some_and(|returned| returned.link == entry.link) {
            let returned = accepted.expect("selected accepted return exists");
            change.push(Edit::CompleteReturn {
                source: fact.event.junction,
                returned: entry.link,
                path: entry.path,
                outcome_witness: unique_outcome_witness(body, fact.event.junction, entry.path)
                    .map(|(witness, _)| witness),
                exact: returned.cause == fact.event.cause,
                exclusive_source: entry.exclusive_source,
                offers_choice: entry.offers_choice,
                at: fact.event.at,
            });
            retain_composite_after_return(
                body,
                entry.path,
                returned.cause == fact.event.cause,
                change,
            );
        } else {
            change.push(Edit::ChangeLink {
                link: entry.link.into(),
                change: LinkChange::Retire,
            });
        }
    }
}

fn retain_composite_after_return(
    body: ReactionView<'_>,
    path: Path,
    exact: bool,
    change: &mut Change,
) {
    if let Some(composite) = composite_with_parents(body, path) {
        change.push(Edit::ChangeLink {
            link: composite.into(),
            change: LinkChange::Strengthen { amount: 1 },
        });
        return;
    }
    let exact_closures = body.link_memory[path.first.slot()].exact_closures;
    if !exact
        || exact_closures.saturating_add(1) < AUTOMATIC_AFTER_EXACT_CLOSURES
        || !path_can_be_composed(body, path)
    {
        return;
    }
    let first = body.arena.link(path.first).expect("validated path entry");
    let second = body.arena.link(path.second).expect("validated path drive");
    let composite = change.new_link();
    change.push(Edit::AddLink {
        new: composite,
        from: path.surface.into(),
        to: path.output.into(),
        spec: LinkSpec {
            delay: first.delay + second.delay,
            impulse: second.impulse,
            trigger: Trigger::SourceFires,
            role: LinkRole::Composite {
                first: path.first,
                second: path.second,
            },
        },
    });
    let parent_strength = body.link_memory[path.second.slot()].strength;
    let amount = i32::try_from(parent_strength).unwrap_or(i32::MAX);
    change.push(Edit::ChangeLink {
        link: composite.into(),
        change: LinkChange::Strengthen { amount },
    });
}

const MAX_AUTOMATIC_COMPOSITE_DEPTH: usize = 32;

impl Body {
    pub fn reentry_state(&self) -> ReentryState {
        ReentryState {
            closed_steps: self
                .link_memory
                .iter()
                .filter(|memory| memory.closed_support().is_some())
                .count(),
        }
    }

    fn invalidate_closed_step(&mut self, path: Path) {
        let view = ReactionView::new(&self.arena, &self.link_memory, &self.returns);
        let invalid = closed_steps(view)
            .filter(|step| step.path == path)
            .map(|step| step.link)
            .collect::<Vec<_>>();
        for link in invalid {
            self.link_memory[link.slot()].outcome_available = false;
        }
    }

    #[inline(always)]
    fn retain_closed_step(
        &mut self,
        returned: LinkId,
        source: JunctionId,
        path: Path,
        outcome_witness: Option<LinkId>,
        exact: bool,
        replaces_existing: bool,
    ) {
        if !exact {
            if replaces_existing {
                self.invalidate_closed_step(path);
            }
            self.link_memory[returned.slot()].outcome_available = false;
            return;
        }
        let Some(outcome_witness) = outcome_witness else {
            if replaces_existing {
                self.invalidate_closed_step(path);
            }
            self.link_memory[returned.slot()].outcome_available = false;
            return;
        };
        let support = (source, outcome_witness);
        if replaces_existing {
            self.invalidate_closed_step(path);
        }
        if self.link_memory[returned.slot()].stored_support() != Some(support) {
            self.link_memory[returned.slot()].remember_closed_support(source, outcome_witness);
        }
    }

    fn prune_reentry(&mut self) {
        let view = ReactionView::new(&self.arena, &self.link_memory, &self.returns);
        let invalid = closed_steps(view)
            .filter(|step| closed_step_is_valid(view, *step).is_none())
            .map(|step| step.link)
            .collect::<Vec<_>>();
        for link in invalid {
            self.link_memory[link.slot()].outcome_available = false;
        }
    }

    fn automaticity_mut(&mut self) -> &mut Automaticity {
        self.automaticity
            .get_or_insert_with(|| Box::new(Automaticity::default()))
    }

    pub fn automaticity_work(&self) -> AutomaticityWork {
        self.automaticity
            .as_ref()
            .map_or(AutomaticityWork::default(), |automaticity| {
                automaticity.work
            })
    }

    pub fn automaticity_state(&self) -> AutomaticityState {
        self.automaticity
            .as_ref()
            .map_or(AutomaticityState::default(), |automaticity| {
                AutomaticityState {
                    open_witnesses: automaticity.witnesses.len(),
                    candidate_pairs: automaticity.evidence.len(),
                    has_recursive_composites: automaticity.generic_composites,
                }
            })
    }

    #[inline(always)]
    fn needs_automatic_closure(&self) -> bool {
        self.automaticity
            .as_ref()
            .is_some_and(|automaticity| automaticity.closure_maintenance)
    }

    fn refresh_automatic_closure(&mut self) {
        if let Some(automaticity) = &mut self.automaticity {
            automaticity.closure_maintenance =
                !automaticity.witnesses.is_empty() || !automaticity.evidence.is_empty();
        }
    }

    pub(crate) fn observe_automatic_pair(&mut self, first: LinkId, second: LinkId, cause: Cause) {
        if cause == 0 || first == second || !automatic_segment_role(self, first) {
            return;
        }
        if !automatic_segment_role(self, second) {
            return;
        }
        let pair = AutomaticPair { first, second };
        let Some((returned, path)) = self.automatic_witness_for_pair(pair, cause) else {
            return;
        };
        let index = self
            .automaticity
            .as_ref()
            .and_then(|automaticity| {
                automaticity
                    .witnesses
                    .iter()
                    .position(|witness| witness.returned == returned)
            })
            .unwrap_or_else(|| {
                let automaticity = self.automaticity_mut();
                let index = automaticity.witnesses.len();
                automaticity.witnesses.push(AutomaticWitness {
                    returned,
                    path,
                    cause,
                    pairs: Vec::new(),
                });
                automaticity.closure_maintenance = true;
                index
            });
        let automaticity = self.automaticity_mut();
        let witness = &mut automaticity.witnesses[index];
        if witness.pairs.contains(&pair) {
            return;
        }
        witness.pairs.push(pair);
        automaticity.work.pair_observations = automaticity.work.pair_observations.saturating_add(1);
    }

    fn automatic_witness_for_pair(
        &self,
        pair: AutomaticPair,
        cause: Cause,
    ) -> Option<(LinkId, Path)> {
        if let Some(automaticity) = &self.automaticity {
            let mut continuing = automaticity.witnesses.iter().filter(|witness| {
                witness.cause == cause
                    && (witness.pairs.contains(&pair)
                        || witness
                            .pairs
                            .last()
                            .is_some_and(|previous| previous.second == pair.first))
            });
            if let Some(witness) = continuing.next() {
                if continuing.next().is_none() {
                    return Some((witness.returned, witness.path));
                }
                return None;
            }
        }

        let root = self.arena.link(pair.first)?.from;
        let view = ReactionView::new(&self.arena, &self.link_memory, &self.returns);
        let mut roots = self.returns.by_source.iter().flatten().filter_map(|entry| {
            let returned = open_return(view, *entry)?;
            (returned.cause == cause
                && self.automatic_root_descends_from(returned.path.output, root, cause))
            .then_some((returned.link, returned.path))
        });
        let root = roots.next()?;
        roots.next().is_none().then_some(root)
    }

    fn automatic_root_descends_from(
        &self,
        output: JunctionId,
        root: JunctionId,
        cause: Cause,
    ) -> bool {
        output == root
            || self.arena.incoming(root).any(|link| {
                let physical = self.arena.link(link).expect("live boundary incidence");
                let memory = &self.link_memory[link.slot()];
                physical.from == output
                    && memory.live
                    && memory.is_boundary_drive()
                    && memory.transmitted
                    && memory.cause == cause
            })
    }

    fn expire_automatic_witness(&mut self, returned: LinkId) {
        let Some(automaticity) = &mut self.automaticity else {
            return;
        };
        automaticity
            .witnesses
            .retain(|witness| witness.returned != returned);
        self.refresh_automatic_closure();
    }

    fn complete_automatic_witness(&mut self, returned: LinkId, path: Path, exact: bool) {
        if self.automaticity.is_none() {
            return;
        }
        if exact
            && self
                .automaticity
                .as_ref()
                .is_some_and(|automaticity| !automaticity.evidence.is_empty())
        {
            let evidence = std::mem::take(
                &mut self
                    .automaticity
                    .as_mut()
                    .expect("checked automaticity")
                    .evidence,
            );
            let retained = evidence
                .into_iter()
                .filter(|evidence| {
                    evidence.owner != path.first || automatic_pair_is_valid(self, evidence.pair)
                })
                .collect();
            self.automaticity
                .as_mut()
                .expect("checked automaticity")
                .evidence = retained;
        }
        let Some(index) = self
            .automaticity
            .as_ref()
            .expect("checked automaticity")
            .witnesses
            .iter()
            .position(|witness| witness.returned == returned)
        else {
            self.refresh_automatic_closure();
            return;
        };
        let witness = self
            .automaticity
            .as_mut()
            .expect("checked automaticity")
            .witnesses
            .remove(index);
        if !exact {
            self.refresh_automatic_closure();
            return;
        }

        let mut ready = Vec::new();
        for pair in witness.pairs {
            if !automatic_pair_is_valid(self, pair) {
                continue;
            }
            let automaticity = self.automaticity_mut();
            automaticity.work.exact_closure_updates =
                automaticity.work.exact_closure_updates.saturating_add(1);
            if automatic_composite_with_parents(self, pair).is_some() {
                continue;
            }
            if let Some(evidence) = self
                .automaticity_mut()
                .evidence
                .iter_mut()
                .find(|evidence| evidence.owner == witness.path.first && evidence.pair == pair)
            {
                evidence.exact_closures = evidence.exact_closures.saturating_add(1);
                if evidence.exact_closures >= AUTOMATIC_AFTER_EXACT_CLOSURES {
                    ready.push(pair);
                }
            } else {
                self.automaticity_mut().evidence.push(AutomaticEvidence {
                    owner: witness.path.first,
                    pair,
                    exact_closures: 1,
                });
            }
        }
        for pair in ready {
            if self.retain_automatic_pair(pair) {
                self.automaticity_mut().evidence.retain(|evidence| {
                    evidence.owner != witness.path.first || evidence.pair != pair
                });
            }
        }
        self.refresh_automatic_closure();
    }

    fn retain_automatic_pair(&mut self, pair: AutomaticPair) -> bool {
        if !automatic_pair_is_valid(self, pair)
            || automatic_composite_with_parents(self, pair).is_some()
        {
            return false;
        }
        let first = *self
            .arena
            .link(pair.first)
            .expect("validated automatic parent");
        let second = *self
            .arena
            .link(pair.second)
            .expect("validated automatic parent");
        let Some(delay) = first.delay.checked_add(second.delay) else {
            return false;
        };
        let Ok(composite) = self.add_link(Link::new(first.from, second.to, delay, second.impulse))
        else {
            return false;
        };
        self.set_link_role(
            composite,
            LinkRole::Composite {
                first: pair.first,
                second: pair.second,
            },
        )
        .expect("new automatic composite exists");
        self.link_memory[composite.slot()].strength = self.link_memory[pair.second.slot()].strength;
        true
    }

    pub(crate) fn preferred_automatic_drive(&self, drive: LinkId) -> LinkId {
        let Some(root) = self.arena.link(drive) else {
            return drive;
        };
        let mut next = self
            .arena
            .junction(root.from)
            .and_then(|junction| junction.outgoing_head);
        let mut best = None;
        let mut best_leaves = 0_usize;
        let mut ambiguous = false;
        while let Some(candidate) = next {
            let physical = self
                .arena
                .link(candidate)
                .expect("live automatic incidence");
            next = physical.next;
            if candidate == drive
                || !matches!(
                    self.link_memory[candidate.slot()].role,
                    LinkRole::Composite { .. }
                )
                || automatic_leftmost(self, candidate, 0) != Some(drive)
                || !automatic_segment_is_valid(self, candidate, 0)
            {
                continue;
            }
            let Some(leaves) = automatic_leaf_count(self, candidate, 0) else {
                continue;
            };
            if leaves > best_leaves {
                best = Some(candidate);
                best_leaves = leaves;
                ambiguous = false;
            } else if leaves == best_leaves && best != Some(candidate) {
                ambiguous = true;
            }
        }
        if ambiguous {
            drive
        } else {
            best.unwrap_or(drive)
        }
    }
}

fn automatic_segment_role(body: &Body, link: LinkId) -> bool {
    body.link_memory.get(link.slot()).is_some_and(|memory| {
        memory.live
            && !memory.is_boundary_drive()
            && matches!(memory.role, LinkRole::Drive | LinkRole::Composite { .. })
    })
}

fn automatic_leftmost(body: &Body, link: LinkId, depth: usize) -> Option<LinkId> {
    if depth >= MAX_AUTOMATIC_COMPOSITE_DEPTH {
        return None;
    }
    match body.link_memory.get(link.slot())?.role {
        LinkRole::Drive => Some(link),
        LinkRole::Composite { first, .. } => automatic_leftmost(body, first, depth + 1),
        _ => None,
    }
}

fn automatic_rightmost(body: &Body, link: LinkId, depth: usize) -> Option<LinkId> {
    if depth >= MAX_AUTOMATIC_COMPOSITE_DEPTH {
        return None;
    }
    match body.link_memory.get(link.slot())?.role {
        LinkRole::Drive => Some(link),
        LinkRole::Composite { second, .. } => automatic_rightmost(body, second, depth + 1),
        _ => None,
    }
}

fn automatic_leaf_count(body: &Body, link: LinkId, depth: usize) -> Option<usize> {
    if depth >= MAX_AUTOMATIC_COMPOSITE_DEPTH {
        return None;
    }
    match body.link_memory.get(link.slot())?.role {
        LinkRole::Drive => Some(1),
        LinkRole::Composite { first, second } => automatic_leaf_count(body, first, depth + 1)?
            .checked_add(automatic_leaf_count(body, second, depth + 1)?),
        _ => None,
    }
}

fn automatic_pair_is_valid(body: &Body, pair: AutomaticPair) -> bool {
    automatic_pair_is_valid_at(body, pair, 0)
}

fn automatic_segment_is_valid(body: &Body, link: LinkId, depth: usize) -> bool {
    if depth >= MAX_AUTOMATIC_COMPOSITE_DEPTH || !automatic_segment_role(body, link) {
        return false;
    }
    let physical = body.arena.link(link).expect("live automatic segment");
    match body.link_memory[link.slot()].role {
        LinkRole::Drive => physical.impulse != 0,
        LinkRole::Composite { first, second } => {
            let pair = AutomaticPair { first, second };
            let Some(left) = body.arena.link(first) else {
                return false;
            };
            let Some(right) = body.arena.link(second) else {
                return false;
            };
            automatic_segment_is_valid(body, first, depth + 1)
                && automatic_segment_is_valid(body, second, depth + 1)
                && automatic_pair_is_valid_at(body, pair, depth + 1)
                && physical.from == left.from
                && physical.to == right.to
                && left.delay.checked_add(right.delay) == Some(physical.delay)
                && physical.impulse == right.impulse
                && physical.trigger == Trigger::SourceFires
                && body.link_memory[link.slot()].strength
                    == body.link_memory[second.slot()].strength
        }
        _ => false,
    }
}

fn automatic_pair_is_valid_at(body: &Body, pair: AutomaticPair, depth: usize) -> bool {
    if depth >= MAX_AUTOMATIC_COMPOSITE_DEPTH {
        return false;
    }
    let Some(first) = body.arena.link(pair.first) else {
        return false;
    };
    let Some(second) = body.arena.link(pair.second) else {
        return false;
    };
    if !automatic_segment_is_valid(body, pair.first, depth)
        || !automatic_segment_is_valid(body, pair.second, depth)
        || first.to != second.from
        || first.trigger != Trigger::SourceFires
        || second.trigger != Trigger::SourceFires
        || first.delay.checked_add(second.delay).is_none()
    {
        return false;
    }
    let middle = first.to;
    let Some(junction) = body.arena.junction(middle) else {
        return false;
    };
    if !matches!(junction.checkpoint_law().retention, Retention::Integrating) {
        return false;
    }
    let delivered =
        i64::from(first.impulse).saturating_mul(body.link_memory[pair.first.slot()].strength);
    if delivered < i64::from(junction.checkpoint_law().threshold) {
        return false;
    }
    let Some(incoming) = automatic_rightmost(body, pair.first, depth) else {
        return false;
    };
    let Some(outgoing) = automatic_leftmost(body, pair.second, depth) else {
        return false;
    };
    if body.arena.incoming(middle).any(|link| {
        body.link_memory[link.slot()].live
            && !matches!(
                body.link_memory[link.slot()].role,
                LinkRole::Composite { .. }
            )
            && link != incoming
    }) {
        return false;
    }
    let mut next = junction.outgoing_head;
    while let Some(link) = next {
        let physical = body.arena.link(link).expect("live automatic incidence");
        next = physical.next;
        if body.link_memory[link.slot()].live
            && !matches!(
                body.link_memory[link.slot()].role,
                LinkRole::Composite { .. }
            )
            && link != outgoing
        {
            return false;
        }
    }
    true
}

fn automatic_composite_with_parents(body: &Body, pair: AutomaticPair) -> Option<LinkId> {
    let from = body.arena.link(pair.first)?.from;
    let mut next = body
        .arena
        .junction(from)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link) = next {
        let physical = body.arena.link(link).expect("live automatic incidence");
        next = physical.next;
        if body.link_memory[link.slot()].live
            && body.link_memory[link.slot()].role
                == (LinkRole::Composite {
                    first: pair.first,
                    second: pair.second,
                })
        {
            return Some(link);
        }
    }
    None
}

#[inline(always)]
pub(crate) fn try_complete_single_return<T: TraceSink>(
    body: &mut Body,
    event: crate::physics::Event,
    trace: &mut T,
) -> bool {
    let source = event.junction;
    let Some([entry]) = body.returns.by_source.get(source.slot()).map(Vec::as_slice) else {
        return false;
    };
    let entry = *entry;
    let view = ReactionView::new(&body.arena, &body.link_memory, &body.returns);
    if surface_may_choose(view, source) {
        return false;
    }
    let Some(returned) = open_return(view, entry) else {
        return false;
    };
    if returned.opened_at > event.at {
        return false;
    }
    let exact = returned.cause == event.cause;
    debug_assert!(body.arena.link(returned.link).is_some());
    debug_assert!(returned
        .path
        .links()
        .into_iter()
        .all(|link| body.arena.link(link).is_some()));
    if T::ENABLED {
        trace.record(TraceEvent::Return(ReturnTrace {
            at: event.at,
            source,
            incoming_cause: event.cause,
            path: Some(returned.path),
            return_cause: Some(returned.cause),
            return_opened_at: Some(returned.opened_at),
            offers_choice: Some(returned.offers_choice),
            open_paths: 1,
            exact_paths: usize::from(returned.cause == event.cause),
            candidates: vec![ReturnCandidateTrace {
                path: returned.path,
                cause: returned.cause,
                opened_at: returned.opened_at,
            }],
            decision: ReturnDecision::Accepted,
        }));
    }
    if !returned.offers_choice {
        clear_output_selection_direct(body, returned.path.output);
    }
    if body.needs_automatic_closure() {
        body.complete_automatic_witness(returned.link, returned.path, exact);
    }
    if entry.exclusive_source {
        body.returns.live_count -= 1;
        body.returns.by_source[source.slot()].clear();
    } else {
        body.remove_live_return_with_path(returned.link, Some(returned.path));
    }
    body.link_memory[returned.link.slot()].live = false;
    let exact_closures = {
        let memory = &mut body.link_memory[returned.path.first.slot()];
        if exact {
            memory.exact_closures = memory.exact_closures.saturating_add(1);
        }
        memory.exact_closures
    };
    for link in returned.path.links() {
        let memory = &mut body.link_memory[link.slot()];
        memory.outcome_at = returned.offers_choice.then_some(event.at);
        memory.outcome_available = returned.offers_choice;
        memory.boundary_closed |= !returned.offers_choice;
        let before = memory.strength;
        let after = before.saturating_add(1);
        memory.strength = after;
        if T::ENABLED {
            trace.record(TraceEvent::Strengthened(StrengthTrace {
                at: event.at,
                link,
                before,
                after,
            }));
        }
    }
    if !exact || exact_closures > 1 {
        let support = body.link_memory[returned.link.slot()].stored_support();
        body.retain_closed_step(
            returned.link,
            source,
            returned.path,
            support
                .filter(|(support_source, _)| *support_source == source)
                .map(|(_, witness)| witness),
            exact,
            exact_closures > u8::from(exact),
        );
    }
    if exact_closures >= AUTOMATIC_AFTER_EXACT_CLOSURES {
        retain_composite_direct(body, returned.path, event.at, trace);
    }
    true
}

fn retain_composite_direct<T: TraceSink>(body: &mut Body, path: Path, at: Time, trace: &mut T) {
    let view = ReactionView::new(&body.arena, &body.link_memory, &body.returns);
    if let Some(composite) = composite_with_parents(view, path) {
        let memory = &mut body.link_memory[composite.slot()];
        let before = memory.strength;
        let after = before.saturating_add(1);
        memory.strength = after;
        if T::ENABLED {
            trace.record(TraceEvent::Strengthened(StrengthTrace {
                at,
                link: composite,
                before,
                after,
            }));
        }
        return;
    }
    if !path_can_be_composed(view, path) {
        return;
    }
    let first = *body.arena.link(path.first).expect("validated path entry");
    let second = *body.arena.link(path.second).expect("validated path drive");
    let parent_strength = body.link_memory[path.second.slot()].strength;
    let Ok(composite) = body.add_link(Link::new(
        path.surface,
        path.output,
        first.delay + second.delay,
        second.impulse,
    )) else {
        return;
    };
    body.set_link_role(
        composite,
        LinkRole::Composite {
            first: path.first,
            second: path.second,
        },
    )
    .expect("new composite link exists");
    let memory = &mut body.link_memory[composite.slot()];
    let before = memory.strength;
    memory.strength = parent_strength;
    if T::ENABLED {
        trace.record(TraceEvent::Strengthened(StrengthTrace {
            at,
            link: composite,
            before,
            after: parent_strength,
        }));
    }
}

fn clear_output_selection_direct(body: &mut Body, output: JunctionId) {
    let (arena, link_memory) = (&body.arena, &mut body.link_memory);
    for second in arena.incoming(output) {
        let physical = arena.link(second).expect("live output incidence");
        let memory = &mut link_memory[second.slot()];
        if !memory.live || memory.role != LinkRole::Drive || physical.impulse == 0 {
            continue;
        }
        memory.outcome_at = None;
        memory.outcome_available = false;
        memory.boundary_inhibited = true;
        for first in arena.incoming(physical.from) {
            let memory = &mut link_memory[first.slot()];
            if memory.live && memory.role == LinkRole::PathEntry {
                memory.outcome_at = None;
                memory.outcome_available = false;
                memory.boundary_inhibited = true;
            }
        }
    }
}

fn clear_output_selection(body: ReactionView<'_>, output: JunctionId, change: &mut Change) {
    for second in body.arena.incoming(output) {
        let physical = body.arena.link(second).expect("live output incidence");
        let memory = &body.link_memory[second.slot()];
        if !memory.live || memory.role != LinkRole::Drive || physical.impulse == 0 {
            continue;
        }
        change.push(Edit::ChangeLink {
            link: second.into(),
            change: LinkChange::ClearOutcomeSelection,
        });
        change.push(Edit::ChangeLink {
            link: second.into(),
            change: LinkChange::InhibitBoundaryChoice,
        });
        for first in body.arena.incoming(physical.from) {
            if body.link_memory[first.slot()].live
                && body.link_memory[first.slot()].role == LinkRole::PathEntry
            {
                change.push(Edit::ChangeLink {
                    link: first.into(),
                    change: LinkChange::ClearOutcomeSelection,
                });
                change.push(Edit::ChangeLink {
                    link: first.into(),
                    change: LinkChange::InhibitBoundaryChoice,
                });
            }
        }
    }
}

#[derive(Clone, Copy)]
struct OpenReturn {
    link: LinkId,
    path: Path,
    cause: Cause,
    opened_at: Time,
    offers_choice: bool,
}

#[derive(Clone, Copy)]
struct ReturnSelection {
    selected: Option<OpenReturn>,
    total: usize,
    exact_total: usize,
}

fn scan_live_returns(body: ReactionView<'_>, source: JunctionId, cause: Cause) -> ReturnSelection {
    let mut only = None;
    let mut total = 0_usize;
    let mut exact = None;
    let mut exact_total = 0_usize;
    let Some(entries) = body.returns.by_source.get(source.slot()) else {
        return ReturnSelection {
            selected: None,
            total: 0,
            exact_total: 0,
        };
    };
    for entry in entries {
        let Some(returned) = open_return(body, *entry) else {
            continue;
        };
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

fn trace_return_candidates(
    body: ReactionView<'_>,
    source: JunctionId,
) -> Vec<ReturnCandidateTrace> {
    body.returns
        .by_source
        .get(source.slot())
        .into_iter()
        .flatten()
        .filter_map(|entry| open_return(body, *entry))
        .map(|returned| ReturnCandidateTrace {
            path: returned.path,
            cause: returned.cause,
            opened_at: returned.opened_at,
        })
        .collect()
}

#[inline(always)]
fn open_return(body: ReactionView<'_>, entry: ReturnEntry) -> Option<OpenReturn> {
    let memory = body.link_memory.get(entry.link.slot())?;
    let LinkRole::Return { cause, .. } = memory.role else {
        return None;
    };
    if !memory.live {
        return None;
    }
    debug_assert_eq!(cause, entry.cause);
    Some(OpenReturn {
        link: entry.link,
        path: entry.path,
        cause,
        opened_at: memory.participated_at,
        offers_choice: entry.offers_choice,
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
    if !memory.live {
        return None;
    }
    if let LinkRole::Composite {
        first,
        second: parent_second,
    } = memory.role
    {
        let path = path_from_links(body, first, parent_second)?;
        return composite_is_valid(body, second, path).then_some(path);
    }
    if memory.role != LinkRole::Drive || drive.impulse == 0 {
        return None;
    }
    let first = body.arena.incoming(drive.from).find(|first| {
        body.link_memory[first.slot()].live
            && body.link_memory[first.slot()].role == LinkRole::PathEntry
    })?;
    path_from_links(body, first, second)
}

fn path_from_links(body: ReactionView<'_>, first: LinkId, second: LinkId) -> Option<Path> {
    let entry = body.arena.link(first)?;
    let drive = body.arena.link(second)?;
    if !body.link_memory.get(first.slot())?.live
        || body.link_memory[first.slot()].role != LinkRole::PathEntry
        || !body.link_memory.get(second.slot())?.live
        || body.link_memory[second.slot()].role != LinkRole::Drive
        || entry.to != drive.from
        || entry.impulse == 0
        || drive.impulse == 0
    {
        return None;
    }
    Some(Path {
        surface: entry.from,
        middle: drive.from,
        output: drive.to,
        first,
        second,
    })
}

fn composite_with_parents(body: ReactionView<'_>, path: Path) -> Option<LinkId> {
    let mut next = body
        .arena
        .junction(path.surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link) = next {
        let physical = body.arena.link(link).expect("live link");
        next = physical.next;
        if body.link_memory[link.slot()].live
            && body.link_memory[link.slot()].role
                == (LinkRole::Composite {
                    first: path.first,
                    second: path.second,
                })
        {
            return Some(link);
        }
    }
    None
}

fn path_middle_is_transparent(body: ReactionView<'_>, path: Path) -> bool {
    let mut next = body
        .arena
        .junction(path.middle)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link) = next {
        let physical = body.arena.link(link).expect("live link");
        next = physical.next;
        if link != path.second
            && body.link_memory[link.slot()].live
            && body.link_memory[link.slot()].role == LinkRole::Drive
        {
            return false;
        }
    }
    true
}

fn path_can_be_composed(body: ReactionView<'_>, path: Path) -> bool {
    let Some(first) = body.arena.link(path.first) else {
        return false;
    };
    let Some(second) = body.arena.link(path.second) else {
        return false;
    };
    path_from_links(body, path.first, path.second) == Some(path)
        && first.trigger == Trigger::SourceFires
        && second.trigger == Trigger::SourceFires
        && first.delay.checked_add(second.delay).is_some()
        && path_middle_is_transparent(body, path)
}

fn composite_is_valid(body: ReactionView<'_>, composite: LinkId, path: Path) -> bool {
    if !path_can_be_composed(body, path) {
        return false;
    }
    let Some(link) = body.arena.link(composite) else {
        return false;
    };
    let Some(first) = body.arena.link(path.first) else {
        return false;
    };
    let Some(second) = body.arena.link(path.second) else {
        return false;
    };
    body.link_memory
        .get(composite.slot())
        .is_some_and(|memory| {
            memory.live
                && memory.role
                    == (LinkRole::Composite {
                        first: path.first,
                        second: path.second,
                    })
        })
        && link.from == path.surface
        && link.to == path.output
        && link.delay == first.delay.saturating_add(second.delay)
        && link.impulse == second.impulse
        && link.trigger == Trigger::SourceFires
}

fn usable_composite(body: ReactionView<'_>, path: Path) -> Option<LinkId> {
    composite_with_parents(body, path).filter(|link| composite_is_valid(body, *link, path))
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
    participated_at: Time,
    output_participated: bool,
    outcome_source: Option<JunctionId>,
    progress_source: Option<JunctionId>,
    resisted_progress: bool,
    boundary_open: bool,
    boundary_inhibited: bool,
    strength: i64,
    drive: u16,
    stable_order: u32,
    fresh_opportunity: Option<FreshOpportunityTrace>,
    reentries: Vec<ReentryTrace>,
    reentry_incidence_visits: u16,
    reentry_failed: bool,
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
    reentry: &mut ReentryScratch,
    change: &mut Change,
    construction: bool,
    trace: &mut T,
) {
    for fact in facts.iter_mut() {
        if !fact.boundary {
            continue;
        }

        let surface = fact.event.junction;
        let current_cause = if is_progress_source(body, surface) {
            0
        } else {
            fact.event.cause
        };
        let ready_start = ready.len();
        append_existing_ready_paths(
            body,
            surface,
            fact.event.at,
            current_cause,
            fact.drive,
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
                    current_cause,
                    return_cause: None,
                    unanswered: false,
                    connected_start,
                    connected_end,
                    outcome: None,
                    participation: 0,
                    participated_at: 0,
                    output_participated: false,
                    outcome_source: unique_witness_source(
                        body,
                        morphology.to,
                        LinkRole::OutcomeWitness,
                    ),
                    progress_source: unique_witness_source(
                        body,
                        morphology.to,
                        LinkRole::ProgressWitness,
                    ),
                    resisted_progress: false,
                    boundary_open: false,
                    boundary_inhibited: false,
                    strength: 1,
                    drive: fact.drive,
                    stable_order: u32::try_from(body.arena.link_count())
                        .unwrap_or(u32::MAX)
                        .saturating_add(second.0),
                    fresh_opportunity: None,
                    reentries: Vec::new(),
                    reentry_incidence_visits: 0,
                    reentry_failed: false,
                    executable: path_is_executable(body, surface, false),
                });
            }
        }
    }

    mark_current_returns(body, facts, ready, connected_outcomes);
    mark_reentries(body, facts, ready, reentry, construction);
    fill_ready_worlds(ready, connected_outcomes, worlds);
    for world in 0..ready.len() {
        if worlds[world] == world {
            if let Some(mut choice) = choose_ready(ready, worlds, world, construction) {
                if !matches!(
                    choice.basis,
                    ChoiceBasis::CurrentReturn
                        | ChoiceBasis::BoundaryRelease
                        | ChoiceBasis::RetainedProgress
                        | ChoiceBasis::UniqueReentry
                ) {
                    if let Some((donor, fresh)) = fresh_opportunity(
                        body,
                        ready,
                        connected_outcomes,
                        worlds,
                        world,
                        construction,
                    ) {
                        ready[donor].fresh_opportunity = Some(fresh);
                        choice.winner = donor;
                        choice.basis = ChoiceBasis::FreshOpportunity;
                    }
                }
                winners.push(choice);
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
                participated_at: candidate.participated_at,
                output_participated: candidate.output_participated,
                outcome_source: candidate.outcome_source,
                progress_source: candidate.progress_source,
                resisted_progress: candidate.resisted_progress,
                boundary_open: candidate.boundary_open,
                boundary_inhibited: candidate.boundary_inhibited,
                strength: candidate.strength,
                drive: candidate.drive,
                stable_order: candidate.stable_order,
                fresh_opportunity: candidate.fresh_opportunity,
                present_sources: reentry.present.clone(),
                reentries: candidate.reentries.clone(),
                reentry_incidence_visits: candidate.reentry_incidence_visits,
                reentry_failed: candidate.reentry_failed,
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
                construction,
                sent: choice.is_some() && !construction,
            }));
        }
    }
    if construction {
        return;
    }
    for choice in winners.iter() {
        let winner = &ready[choice.winner];
        let through = winner.fresh_opportunity.map_or_else(
            || {
                let path = match (winner.middle, winner.first, winner.second) {
                    (
                        JunctionRef::Existing(middle),
                        LinkRef::Existing(first),
                        LinkRef::Existing(second),
                    ) => Some(Path {
                        surface: winner.surface,
                        middle,
                        output: winner.output,
                        first,
                        second,
                    }),
                    _ => None,
                };
                path.and_then(|path| usable_composite(body, path))
                    .map_or(winner.first, LinkRef::Existing)
            },
            |fresh| fresh.through.into(),
        );
        change.push(Edit::Send {
            through,
            at: winner.at,
            cause: winner.current_cause,
        });
        for (index, candidate) in ready.iter().enumerate() {
            if worlds[index] != worlds[choice.winner] || !candidate.boundary_inhibited {
                continue;
            }
            for link in [candidate.first, candidate.second] {
                change.push(Edit::ChangeLink {
                    link,
                    change: LinkChange::ConsumeBoundaryInhibition,
                });
            }
        }
        if winner.fresh_opportunity.is_none()
            && winner
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
    drive: u16,
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
                    participated_at: memory.participated_at,
                    output_participated: false,
                    outcome_source: unique_witness_source(body, link.to, LinkRole::OutcomeWitness),
                    progress_source: unique_witness_source(
                        body,
                        link.to,
                        LinkRole::ProgressWitness,
                    ),
                    resisted_progress: false,
                    boundary_open: memory.participation > 0 && !memory.boundary_closed,
                    boundary_inhibited: memory.boundary_inhibited,
                    strength: memory.strength,
                    drive,
                    stable_order: second_id.slot() as u32,
                    fresh_opportunity: None,
                    reentries: Vec::new(),
                    reentry_incidence_visits: 0,
                    reentry_failed: false,
                    executable: path_is_executable(body, surface, outcome.is_some()),
                });
            }
        }
    }
}

fn mark_reentries(
    body: ReactionView<'_>,
    facts: &[MomentFact],
    paths: &mut [ReadyPath],
    scratch: &mut ReentryScratch,
    construction: bool,
) {
    if construction {
        return;
    }
    scratch.present.extend(
        facts
            .iter()
            .filter(|fact| {
                fact.boundary && matches!(fact.used, UsedPaths::None) && !fact.had_ready_path
            })
            .map(|fact| fact.event.junction),
    );
    if scratch.present.is_empty() {
        return;
    }
    for candidate in paths.iter_mut().filter(|candidate| candidate.executable) {
        let (JunctionRef::Existing(middle), LinkRef::Existing(first), LinkRef::Existing(second)) =
            (candidate.middle, candidate.first, candidate.second)
        else {
            continue;
        };
        let path = Path {
            surface: candidate.surface,
            middle,
            output: candidate.output,
            first,
            second,
        };
        scratch.clear_search();
        let mut visits = 0;
        match find_reentries(
            body,
            path,
            &scratch.present,
            &mut scratch.steps,
            &mut scratch.continuations,
            &mut visits,
        ) {
            Ok(found) => candidate.reentries = found,
            Err(()) => candidate.reentry_failed = true,
        }
        candidate.reentry_incidence_visits = visits;
    }
}

fn find_reentries(
    body: ReactionView<'_>,
    path: Path,
    present: &[JunctionId],
    steps: &mut Vec<ReentryStepTrace>,
    continuations: &mut Vec<ReentryContinuation>,
    incidence_visits: &mut u16,
) -> Result<Vec<ReentryTrace>, ()> {
    let mut search = ReentrySearch {
        body,
        present,
        steps,
        continuations,
        incidence_visits,
        found: Vec::new(),
    };
    search.search(path, 0)?;
    Ok(search.found)
}

struct ReentrySearch<'body, 'scratch> {
    body: ReactionView<'body>,
    present: &'scratch [JunctionId],
    steps: &'scratch mut Vec<ReentryStepTrace>,
    continuations: &'scratch mut Vec<ReentryContinuation>,
    incidence_visits: &'scratch mut u16,
    found: Vec<ReentryTrace>,
}

impl ReentrySearch<'_, '_> {
    fn search(&mut self, path: Path, depth: usize) -> Result<(), ()> {
        if depth >= MAX_REENTRY_DEPTH || self.steps.iter().any(|step| step.path == path) {
            return Err(());
        }
        let mut next = self
            .body
            .arena
            .junction(path.output)
            .and_then(|junction| junction.outgoing_head);
        while let Some(link) = next {
            visit_reentry_incidence(self.incidence_visits)?;
            let physical = *self.body.arena.link(link).expect("live retained incidence");
            next = physical.next;
            if physical.to != path.middle {
                continue;
            }
            let Some((returned_source, outcome_witness)) =
                self.body.link_memory[link.slot()].closed_support()
            else {
                continue;
            };
            let retained = ClosedStep {
                link,
                path,
                returned_source,
                outcome_witness,
            };
            let Some(outcome_target) =
                closed_step_is_valid_for_reentry(self.body, retained, self.incidence_visits)?
            else {
                return Err(());
            };
            self.steps.push(ReentryStepTrace {
                path,
                returned_source,
                outcome_witness,
                outcome_target,
            });
            if self.present.contains(&returned_source) {
                self.found.push(ReentryTrace {
                    condition: returned_source,
                    steps: self.steps.clone(),
                });
            } else {
                let start = self.continuations.len();
                append_reentry_continuations(
                    self.body,
                    returned_source,
                    self.continuations,
                    self.incidence_visits,
                )?;
                let end = self.continuations.len();
                for index in start..end {
                    let continuation = self.continuations[index];
                    self.search(continuation.path, depth + 1)?;
                }
                self.continuations.truncate(start);
            }
            self.steps.pop();
        }
        Ok(())
    }
}

fn visit_reentry_incidence(incidence_visits: &mut u16) -> Result<(), ()> {
    if *incidence_visits >= MAX_REENTRY_INCIDENCE_VISITS {
        return Err(());
    }
    *incidence_visits += 1;
    Ok(())
}

fn append_reentry_continuations(
    body: ReactionView<'_>,
    surface: JunctionId,
    continuations: &mut Vec<ReentryContinuation>,
    incidence_visits: &mut u16,
) -> Result<(), ()> {
    let start = continuations.len();
    let mut next = body
        .arena
        .junction(surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(first_id) = next {
        visit_reentry_incidence(incidence_visits)?;
        let first = *body.arena.link(first_id).expect("live path incidence");
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
            visit_reentry_incidence(incidence_visits)?;
            let drive = *body.arena.link(second_id).expect("live drive incidence");
            second = drive.next;
            let memory = &body.link_memory[second_id.slot()];
            if !memory.live || memory.role != LinkRole::Drive || drive.impulse == 0 {
                continue;
            }
            let path = Path {
                surface,
                middle: first.to,
                output: drive.to,
                first: first_id,
                second: second_id,
            };
            if path_from_links(body, first_id, second_id) != Some(path)
                || continuations[start..]
                    .iter()
                    .any(|continuation| continuation.path == path)
            {
                continue;
            }
            continuations.push(ReentryContinuation { path });
        }
    }
    Ok(())
}

fn closed_step_is_valid_for_reentry(
    body: ReactionView<'_>,
    step: ClosedStep,
    incidence_visits: &mut u16,
) -> Result<Option<JunctionId>, ()> {
    if path_from_links(body, step.path.first, step.path.second) != Some(step.path)
        || body.link_memory[step.path.first.slot()].participation == 0
        || body.link_memory[step.path.second.slot()].participation == 0
        || !path_is_executable_for_reentry(body, step.path.surface, incidence_visits)?
    {
        return Ok(None);
    }
    let Some((witness, target)) = unique_outcome_witness_for_reentry(
        body,
        step.returned_source,
        step.path,
        incidence_visits,
    )?
    else {
        return Ok(None);
    };
    Ok((witness == step.outcome_witness).then_some(target))
}

fn path_is_executable_for_reentry(
    body: ReactionView<'_>,
    surface: JunctionId,
    incidence_visits: &mut u16,
) -> Result<bool, ()> {
    let mut parent = None;
    for link in body.arena.incoming(surface) {
        visit_reentry_incidence(incidence_visits)?;
        if !is_membership_link(body, link) {
            continue;
        }
        let found = body
            .arena
            .link(link)
            .expect("live membership incidence")
            .from;
        if parent.is_some_and(|existing| existing != found) {
            return Ok(false);
        }
        parent = Some(found);
    }
    let Some(parent) = parent else {
        return Ok(true);
    };
    for link in body.arena.incoming(parent) {
        visit_reentry_incidence(incidence_visits)?;
        if is_membership_link(body, link) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn unique_outcome_witness_for_reentry(
    body: ReactionView<'_>,
    source: JunctionId,
    path: Path,
    incidence_visits: &mut u16,
) -> Result<Option<(LinkId, JunctionId)>, ()> {
    let mut selected = None;
    for target in [path.middle, path.output] {
        for witness in body.arena.incoming(target) {
            visit_reentry_incidence(incidence_visits)?;
            let physical = body.arena.link(witness).expect("live outcome incidence");
            let memory = &body.link_memory[witness.slot()];
            if !memory.live || memory.role != LinkRole::OutcomeWitness || physical.from != source {
                continue;
            }
            if selected.is_some() {
                return Ok(None);
            }
            selected = Some((witness, target));
        }
    }
    Ok(selected)
}

fn closed_steps(body: ReactionView<'_>) -> impl Iterator<Item = ClosedStep> + '_ {
    (0..body.link_memory.len()).filter_map(move |slot| {
        let link = LinkId::new(slot)?;
        closed_step(body, link)
    })
}

fn closed_step(body: ReactionView<'_>, link: LinkId) -> Option<ClosedStep> {
    let (returned_source, outcome_witness) = body.link_memory.get(link.slot())?.closed_support()?;
    let physical = body.arena.link(link)?;
    let second = outgoing_drive_to(body, physical.to, physical.from)?;
    let path = path_from_drive(body, second)?;
    (path.middle == physical.to && path.output == physical.from).then_some(ClosedStep {
        link,
        path,
        returned_source,
        outcome_witness,
    })
}

fn closed_step_is_valid(body: ReactionView<'_>, step: ClosedStep) -> Option<JunctionId> {
    if path_from_links(body, step.path.first, step.path.second) != Some(step.path)
        || body.link_memory[step.path.first.slot()].participation == 0
        || body.link_memory[step.path.second.slot()].participation == 0
        || !path_is_executable(body, step.path.surface, true)
    {
        return None;
    }
    let (witness, target) = unique_outcome_witness(body, step.returned_source, step.path)?;
    (witness == step.outcome_witness).then_some(target)
}

fn unique_outcome_witness(
    body: ReactionView<'_>,
    source: JunctionId,
    path: Path,
) -> Option<(LinkId, JunctionId)> {
    let mut selected = None;
    for target in [path.middle, path.output] {
        for witness in body.arena.incoming(target) {
            let physical = body.arena.link(witness).expect("live outcome support");
            let memory = &body.link_memory[witness.slot()];
            if !memory.live || memory.role != LinkRole::OutcomeWitness || physical.from != source {
                continue;
            }
            if selected.is_some() {
                return None;
            }
            selected = Some((witness, target));
        }
    }
    selected
}

fn mark_current_returns(
    body: ReactionView<'_>,
    facts: &[MomentFact],
    paths: &mut [ReadyPath],
    connected_outcomes: &[JunctionId],
) {
    for fact in facts.iter().filter(|fact| {
        fact.boundary && matches!(fact.used, UsedPaths::None) && !fact.had_ready_path
    }) {
        let source = fact.event.junction;
        let output = latest_fresh_output(body, source).or_else(|| {
            scan_live_returns(body, source, fact.event.cause)
                .selected
                .filter(|returned| returned.opened_at <= fact.event.at)
                .map(|returned| returned.path.output)
        });
        let progress_output = unique_progress_output(paths, source, fact.event.cause);
        for path in paths.iter_mut() {
            let connected = &connected_outcomes[path.connected_start..path.connected_end];
            if connected.contains(&source) {
                path.output_participated |= output == Some(path.output);
            }
            if path.progress_source == Some(source) {
                path.resisted_progress |= progress_output == Some(path.output);
            }
        }
    }
}

fn unique_progress_output(
    paths: &[ReadyPath],
    source: JunctionId,
    cause: Cause,
) -> Option<JunctionId> {
    if cause == 0 {
        return None;
    }
    let mut output = None;
    for path in paths.iter().filter(|path| {
        path.progress_source == Some(source)
            && path.return_cause == Some(cause)
            && path.participation > 0
    }) {
        match output {
            None => output = Some(path.output),
            Some(existing) if existing != path.output => return None,
            Some(_) => {}
        }
    }
    output
}

fn latest_fresh_output(body: ReactionView<'_>, source: JunctionId) -> Option<JunctionId> {
    let mut latest_fresh = None;
    let mut fresh_ambiguous = false;
    let mut latest_drive = None;
    let mut next = body
        .arena
        .junction(source)
        .and_then(|junction| junction.outgoing_head);
    while let Some(witness) = next {
        let link = body.arena.link(witness).expect("live link");
        next = link.next;
        let memory = &body.link_memory[witness.slot()];
        if !memory.live || !closes_return(memory.role) {
            continue;
        }
        if memory.transmitted {
            match latest_fresh {
                None => latest_fresh = Some((memory.participated_at, link.to)),
                Some((at, _)) if memory.participated_at > at => {
                    latest_fresh = Some((memory.participated_at, link.to));
                    fresh_ambiguous = false;
                }
                Some((at, output)) if memory.participated_at == at && link.to != output => {
                    fresh_ambiguous = true;
                }
                Some(_) => {}
            }
        }
        for drive in body.arena.incoming(link.to) {
            let physical = body.arena.link(drive).expect("live link");
            let memory = &body.link_memory[drive.slot()];
            if memory.live
                && memory.role == LinkRole::Drive
                && physical.impulse != 0
                && memory.participation > 0
            {
                latest_drive = Some(latest_drive.map_or(memory.participated_at, |at: Time| {
                    at.max(memory.participated_at)
                }));
            }
        }
    }
    if fresh_ambiguous {
        return None;
    }
    let (fresh_at, output) = latest_fresh?;
    latest_drive
        .is_none_or(|drive_at| fresh_at > drive_at)
        .then_some(output)
}

fn fresh_opportunity(
    body: ReactionView<'_>,
    paths: &[ReadyPath],
    connected_outcomes: &[JunctionId],
    worlds: &[usize],
    world: usize,
    construction: bool,
) -> Option<(usize, FreshOpportunityTrace)> {
    if construction {
        return None;
    }
    let strongest_drive = paths
        .iter()
        .enumerate()
        .filter(|(index, path)| {
            worlds[*index] == world && path.executable && !path.boundary_inhibited
        })
        .map(|(_, path)| path.drive)
        .max()?;
    let mut donors = paths.iter().enumerate().filter(|(index, path)| {
        worlds[*index] == world
            && path.executable
            && !path.boundary_inhibited
            && path.drive == strongest_drive
            && path.unanswered
            && path.participation > 0
            && path.current_cause != 0
    });
    let (donor_index, donor) = donors.next()?;
    if donors.next().is_some() {
        return None;
    }
    if paths.iter().enumerate().any(|(index, path)| {
        worlds[index] == world
            && path.executable
            && !path.boundary_inhibited
            && path.drive == strongest_drive
            && path.surface != donor.surface
    }) {
        return None;
    }
    let mut selected = None;
    for source in &connected_outcomes[donor.connected_start..donor.connected_end] {
        let mut next = body
            .arena
            .junction(*source)
            .and_then(|junction| junction.outgoing_head);
        while let Some(witness) = next {
            let link = body.arena.link(witness).expect("live link");
            next = link.next;
            let memory = &body.link_memory[witness.slot()];
            if !memory.live
                || !closes_return(memory.role)
                || link.to == donor.output
                || paths.iter().enumerate().any(|(index, path)| {
                    worlds[index] == world
                        && path.executable
                        && !path.boundary_inhibited
                        && path.output == link.to
                })
                || !outputs_are_local(body, donor.output, link.to)
            {
                continue;
            }
            let fresh = FreshOpportunityTrace {
                source: *source,
                output: link.to,
                through: witness,
            };
            if selected.is_none_or(|current: FreshOpportunityTrace| witness < current.through) {
                selected = Some(fresh);
            }
        }
    }
    selected.map(|fresh| (donor_index, fresh))
}

fn outputs_are_local(body: ReactionView<'_>, left: JunctionId, right: JunctionId) -> bool {
    body.arena.incoming(left).any(|incidence| {
        let link = body.arena.link(incidence).expect("live link");
        let memory = &body.link_memory[incidence.slot()];
        memory.live
            && memory.role == LinkRole::Drive
            && link.impulse == 0
            && (1..=LOCAL_RADIUS as Time).contains(&link.delay)
            && surface_reaches(body, link.from, right)
    })
}

fn surface_reaches(body: ReactionView<'_>, surface: JunctionId, output: JunctionId) -> bool {
    let mut next = body
        .arena
        .junction(surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(incidence) = next {
        let link = body.arena.link(incidence).expect("live link");
        next = link.next;
        let memory = &body.link_memory[incidence.slot()];
        if memory.live
            && memory.role == LinkRole::Drive
            && link.to == output
            && link.impulse == 0
            && (1..=LOCAL_RADIUS as Time).contains(&link.delay)
        {
            return true;
        }
    }
    false
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
    if !construction {
        let mut inhibited = (0..paths.len()).filter(|index| {
            worlds[*index] == world && paths[*index].executable && paths[*index].boundary_inhibited
        });
        if let Some(first) = inhibited.next() {
            let source = paths[first].outcome_source?;
            if inhibited.any(|index| paths[index].outcome_source != Some(source)) {
                return None;
            }
            return unique_output(
                (0..paths.len()).filter(|index| {
                    worlds[*index] == world
                        && paths[*index].executable
                        && !paths[*index].boundary_inhibited
                        && paths[*index].outcome_source == Some(source)
                }),
                paths,
            )
            .map(|winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::BoundaryRelease,
            });
        }
    }
    let eligible = |index: &usize| {
        worlds[*index] == world
            && (construction || (paths[*index].executable && !paths[*index].boundary_inhibited))
    };
    let strongest_drive = (0..paths.len())
        .filter(eligible)
        .map(|index| paths[index].drive)
        .max()?;
    let active = |index: &usize| eligible(index) && paths[*index].drive == strongest_drive;
    let latest_unanswered = latest_unanswered_output((0..paths.len()).filter(active), paths);
    let output_is_tried = |output| {
        (0..paths.len()).any(|index| {
            active(&index)
                && paths[index].output == output
                && (paths[index].participation > 0 || paths[index].output_participated)
        })
    };
    let has_tried_output =
        (0..paths.len()).any(|index| active(&index) && output_is_tried(paths[index].output));
    let has_untried_output =
        (0..paths.len()).any(|index| active(&index) && !output_is_tried(paths[index].output));
    let release_to_untried_output = has_tried_output && has_untried_output;
    unique_ready((0..paths.len()).filter(eligible).filter(|index| {
        let path = &paths[*index];
        path.return_cause.is_some() && path.return_cause == Some(path.current_cause)
    }))
    .map(|winner| ReadyChoice {
        winner,
        basis: ChoiceBasis::CurrentReturn,
    })
    .or_else(|| {
        unique_retained_progress((0..paths.len()).filter(active), paths).map(|winner| ReadyChoice {
            winner,
            basis: ChoiceBasis::RetainedProgress,
        })
    })
    .or_else(|| {
        release_to_untried_output
            .then(|| {
                (0..paths.len())
                    .filter(active)
                    .filter(|index| !output_is_tried(paths[*index].output))
                    .max_by_key(|index| {
                        let path = &paths[*index];
                        (
                            path.participation,
                            path.strength,
                            path.drive,
                            Reverse(path.stable_order),
                        )
                    })
            })
            .flatten()
            .map(|winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::UntriedOutputRelease,
            })
    })
    .or_else(|| {
        unique_returned_output((0..paths.len()).filter(active), paths).map(|winner| ReadyChoice {
            winner,
            basis: ChoiceBasis::CurrentReturn,
        })
    })
    .or_else(|| {
        unique_reentry((0..paths.len()).filter(active), paths).map(|winner| ReadyChoice {
            winner,
            basis: ChoiceBasis::UniqueReentry,
        })
    })
    .or_else(|| {
        unique_latest_ready(paths, worlds, world, strongest_drive, true, construction).map(
            |winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::AvailableOutcome,
            },
        )
    })
    .or_else(|| {
        let unanswered = latest_unanswered?;
        (0..paths.len())
            .filter(active)
            .filter(|index| paths[*index].output != unanswered)
            .max_by_key(|index| {
                let path = &paths[*index];
                (
                    path.participation,
                    path.strength,
                    path.drive,
                    Reverse(path.stable_order),
                )
            })
            .map(|winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::UnansweredOutputRelease,
            })
    })
    .or_else(|| {
        unique_latest_ready(paths, worlds, world, strongest_drive, false, construction).map(
            |winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::LatestOutcome,
            },
        )
    })
    .or_else(|| {
        (0..paths.len())
            .filter(active)
            .max_by_key(|index| {
                let path = &paths[*index];
                (
                    path.participation,
                    path.strength,
                    path.drive,
                    Reverse(path.stable_order),
                )
            })
            .map(|winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::ParticipationStrengthAndDrive,
            })
    })
}

fn unique_reentry(paths: impl Iterator<Item = usize>, ready: &[ReadyPath]) -> Option<usize> {
    let candidates = paths.collect::<Vec<_>>();
    if candidates
        .iter()
        .any(|index| ready[*index].reentry_failed || ready[*index].reentries.len() > 1)
    {
        return None;
    }
    unique_ready(
        candidates
            .into_iter()
            .filter(|index| ready[*index].reentries.len() == 1),
    )
}

fn unique_returned_output(
    paths: impl Iterator<Item = usize>,
    ready: &[ReadyPath],
) -> Option<usize> {
    unique_output(
        paths.filter(|index| ready[*index].output_participated),
        ready,
    )
}

fn latest_unanswered_output(
    paths: impl Iterator<Item = usize>,
    ready: &[ReadyPath],
) -> Option<JunctionId> {
    let mut latest = None;
    let mut output = None;
    let mut ambiguous = false;
    for index in paths.filter(|index| ready[*index].unanswered && ready[*index].participation > 0) {
        let path = &ready[index];
        match latest {
            None => {
                latest = Some(path.participated_at);
                output = Some(path.output);
            }
            Some(at) if path.participated_at > at => {
                latest = Some(path.participated_at);
                output = Some(path.output);
                ambiguous = false;
            }
            Some(at) if path.participated_at == at && output != Some(path.output) => {
                ambiguous = true;
            }
            Some(_) => {}
        }
    }
    if ambiguous {
        None
    } else {
        output
    }
}

fn unique_output(paths: impl Iterator<Item = usize>, ready: &[ReadyPath]) -> Option<usize> {
    let mut output: Option<JunctionId> = None;
    let mut winner: Option<usize> = None;
    for index in paths {
        match output {
            None => output = Some(ready[index].output),
            Some(current) if current != ready[index].output => return None,
            Some(_) => {}
        }
        if winner.is_none_or(|current| {
            let path = &ready[index];
            let selected = &ready[current];
            (
                path.participation,
                path.strength,
                path.drive,
                Reverse(path.stable_order),
            ) > (
                selected.participation,
                selected.strength,
                selected.drive,
                Reverse(selected.stable_order),
            )
        }) {
            winner = Some(index);
        }
    }
    winner
}

fn unique_retained_progress(
    paths: impl Iterator<Item = usize>,
    ready: &[ReadyPath],
) -> Option<usize> {
    unique_output(
        paths.filter(|index| {
            ready[*index].resisted_progress
                && ready[*index].boundary_open
                && ready[*index].strength > 1
                && ready[*index].participation > 0
        }),
        ready,
    )
}

fn unique_witness_source(
    body: ReactionView<'_>,
    junction: JunctionId,
    role: LinkRole,
) -> Option<JunctionId> {
    let mut source = None;
    for witness in body.arena.incoming(junction) {
        let memory = &body.link_memory[witness.slot()];
        if !memory.live || memory.role != role {
            continue;
        }
        let candidate = body.arena.link(witness).expect("live witness").from;
        match source {
            None => source = Some(candidate),
            Some(existing) if existing != candidate => return None,
            Some(_) => {}
        }
    }
    source
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
    drive: u16,
    available_only: bool,
    construction: bool,
) -> Option<usize> {
    let latest = (0..paths.len())
        .filter(|index| {
            worlds[*index] == world
                && paths[*index].drive == drive
                && (construction || paths[*index].executable)
        })
        .filter_map(|index| {
            paths[index]
                .outcome
                .filter(|outcome| !available_only || outcome.available_until_choice)
        })
        .map(|outcome| outcome.at)
        .max()?;
    unique_ready((0..paths.len()).filter(|index| {
        if worlds[*index] != world
            || paths[*index].drive != drive
            || (!construction && !paths[*index].executable)
        {
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
                    && closes_return(body.link_memory[link.slot()].role)
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
    pub(crate) path: Path,
    pub(crate) exclusive_source: bool,
    pub(crate) offers_choice: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ReturnIndex {
    pub(crate) by_source: Vec<Vec<ReturnEntry>>,
    pub(crate) live_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LinkMemory {
    pub(crate) live: bool,
    pub(crate) transmitted: bool,
    pub(crate) role: LinkRole,
    pub(crate) participation: u64,
    pub(crate) cause: Cause,
    pub(crate) participated_at: Time,
    pub(crate) outcome_at: Option<Time>,
    pub(crate) outcome_available: bool,
    pub(crate) boundary_closed: bool,
    pub(crate) boundary_inhibited: bool,
    pub(crate) boundary_drive: bool,
    pub(crate) exact_closures: u8,
    pub(crate) strength: i64,
}

const fn return_topology_role(role: LinkRole) -> bool {
    matches!(
        role,
        LinkRole::Drive
            | LinkRole::PathEntry
            | LinkRole::OutcomeWitness
            | LinkRole::BoundaryWitness
    )
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
            boundary_closed: false,
            boundary_inhibited: false,
            boundary_drive: false,
            exact_closures: 0,
            strength: 1,
        }
    }
}

impl LinkMemory {
    pub(crate) const fn is_boundary_drive(&self) -> bool {
        self.boundary_drive
    }

    pub(crate) fn record_transmission(&mut self, cause: Cause, at: Time) {
        self.transmitted = true;
        self.cause = cause;
        self.participated_at = at;
    }

    pub(crate) fn remap_links(&mut self, base: usize) {
        if let LinkRole::Composite { first, second } = self.role {
            self.role = LinkRole::Composite {
                first: remap_link(first, base),
                second: remap_link(second, base),
            };
        }
        if let Some((source, witness)) = self.closed_support() {
            self.remember_closed_support(source, remap_link(witness, base));
        }
    }

    pub(crate) fn remap_junctions(&mut self, base: usize) {
        if let Some((source, witness)) = self.closed_support() {
            let source = JunctionId::new(base + source.slot())
                .expect("validated attachment junction identity");
            self.remember_closed_support(source, witness);
        }
    }

    #[inline(always)]
    fn remember_closed_support(&mut self, source: JunctionId, witness: LinkId) {
        // A retired Return link no longer uses strength or outcome availability.
        // Reuse those cold fields for its sparse exact support instead of adding
        // state to every hot drive link.
        let source = u32::try_from(source.slot()).expect("junction identity fits in u32");
        let witness = u32::try_from(witness.slot()).expect("link identity fits in u32");
        self.strength = ((u64::from(source) << 32) | u64::from(witness)) as i64;
        self.outcome_available = true;
    }

    #[inline(always)]
    fn stored_support(&self) -> Option<(JunctionId, LinkId)> {
        if !self.outcome_available || !matches!(self.role, LinkRole::Return { .. }) {
            return None;
        }
        let packed = self.strength as u64;
        let source = JunctionId::new((packed >> 32) as usize)?;
        let witness = LinkId::new((packed as u32) as usize)?;
        Some((source, witness))
    }

    #[inline(always)]
    fn closed_support(&self) -> Option<(JunctionId, LinkId)> {
        (!self.live).then(|| self.stored_support()).flatten()
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
    pub(crate) fn rebuild_live_returns(&mut self) {
        for returns in &mut self.returns.by_source {
            returns.clear();
        }
        self.returns.live_count = 0;
        for slot in 0..self.link_memory.len() {
            let memory = &self.link_memory[slot];
            let live = memory.live;
            let role = memory.role;
            let LinkRole::Return { .. } = role else {
                continue;
            };
            if !live {
                continue;
            }
            let link = LinkId::new(slot).expect("validated link identity");
            self.insert_live_return(link, role);
        }
    }

    fn insert_live_return(&mut self, link: LinkId, role: LinkRole) {
        let LinkRole::Return { cause, .. } = role else {
            return;
        };
        self.returns.live_count += 1;
        let view = ReactionView::new(&self.arena, &self.link_memory, &self.returns);
        let Some(physical) = self.arena.link(link) else {
            return;
        };
        let Some(second) = outgoing_drive_to(view, physical.to, physical.from) else {
            return;
        };
        let Some(path) = path_from_drive(view, second) else {
            return;
        };
        let mut only_source = None;
        let mut multiple_sources = false;
        for target in [path.middle, path.output] {
            for witness in self.arena.incoming(target) {
                if !self.link_memory[witness.slot()].live
                    || !closes_return(self.link_memory[witness.slot()].role)
                {
                    continue;
                }
                let source = self.arena.link(witness).expect("live outcome witness").from;
                if let Some(existing) = only_source {
                    multiple_sources |= existing != source;
                } else {
                    only_source = Some(source);
                }
            }
        }
        let exclusive_source = only_source.is_some() && !multiple_sources;
        let mut sources = Vec::new();
        for target in [path.middle, path.output] {
            for witness in self.arena.incoming(target) {
                let role = self.link_memory[witness.slot()].role;
                if !self.link_memory[witness.slot()].live || !closes_return(role) {
                    continue;
                }
                let source = self.arena.link(witness).expect("live outcome witness").from;
                sources.push((source, role));
            }
        }
        let prepared = only_source.and_then(|source| {
            let view = ReactionView::new(&self.arena, &self.link_memory, &self.returns);
            unique_outcome_witness(view, source, path).map(|(witness, _)| (source, witness))
        });
        if let Some((source, witness)) = prepared {
            self.link_memory[link.slot()].remember_closed_support(source, witness);
        } else {
            self.link_memory[link.slot()].outcome_available = false;
        }
        for (source, role) in sources {
            let entry = ReturnEntry {
                cause,
                link,
                path,
                exclusive_source,
                offers_choice: role == LinkRole::OutcomeWitness,
            };
            let returns = &mut self.returns.by_source[source.slot()];
            if let Err(index) = returns.binary_search(&entry) {
                returns.insert(index, entry);
            }
        }
    }

    fn remove_live_return(&mut self, link: LinkId, role: LinkRole) {
        let LinkRole::Return { .. } = role else {
            return;
        };
        self.expire_automatic_witness(link);
        let view = ReactionView::new(&self.arena, &self.link_memory, &self.returns);
        let path = self.arena.link(link).and_then(|physical| {
            outgoing_drive_to(view, physical.to, physical.from)
                .and_then(|second| path_from_drive(view, second))
        });
        self.remove_live_return_with_path(link, path);
    }

    fn remove_live_return_with_path(&mut self, link: LinkId, path: Option<Path>) {
        self.expire_automatic_witness(link);
        if let Some(path) = path {
            self.invalidate_closed_step(path);
        }
        debug_assert!(self.returns.live_count > 0, "live return count drift");
        self.returns.live_count -= 1;
        let (arena, link_memory, returns_by_source) =
            (&self.arena, &self.link_memory, &mut self.returns.by_source);
        let Some(path) = path else {
            for returns in returns_by_source {
                returns.retain(|entry| entry.link != link);
            }
            return;
        };
        for target in [path.middle, path.output] {
            for witness in arena.incoming(target) {
                if !link_memory[witness.slot()].live
                    || !closes_return(link_memory[witness.slot()].role)
                {
                    continue;
                }
                let source = arena.link(witness).expect("live outcome witness").from;
                returns_by_source[source.slot()].retain(|entry| entry.link != link);
            }
        }
    }

    fn remove_exclusive_live_return(&mut self, source: JunctionId, link: LinkId) {
        self.expire_automatic_witness(link);
        debug_assert!(self.returns.live_count > 0, "live return count drift");
        self.returns.live_count -= 1;
        let returns = &mut self.returns.by_source[source.slot()];
        let before = returns.len();
        returns.retain(|entry| entry.link != link);
        debug_assert_eq!(returns.len() + 1, before, "live return index drift");
    }

    pub fn set_link_impulse(&mut self, link: LinkId, impulse: Impulse) -> Result<(), ApplyError> {
        let physical = self
            .arena
            .link_mut(link)
            .ok_or(ApplyError::UnknownLink(link))?;
        physical.impulse = impulse;
        if self.returns.live_count != 0 {
            self.rebuild_live_returns();
        }
        self.prune_reentry();
        Ok(())
    }

    /// Marks an ordinary transmitting link as an outward body/world crossing.
    /// The link still drives normally, but internal automaticity may not erase
    /// it or compose across it.
    pub(crate) fn mark_boundary_drive(&mut self, link: LinkId) -> Result<(), ApplyError> {
        let memory = self
            .link_memory
            .get_mut(link.slot())
            .ok_or(ApplyError::UnknownLink(link))?;
        memory.boundary_drive = true;
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
        if !matches!(previous, LinkRole::Composite { .. })
            && matches!(role, LinkRole::Composite { .. })
        {
            let automaticity = self.automaticity_mut();
            automaticity.work.composites_formed =
                automaticity.work.composites_formed.saturating_add(1);
            if automatic_leftmost(self, link, 0)
                .is_some_and(|root| self.link_memory[root.slot()].role == LinkRole::Drive)
            {
                self.automaticity_mut().generic_composites = true;
            }
        }
        if live {
            self.insert_live_return(link, role);
        }
        if live
            && self.returns.live_count != 0
            && previous != role
            && (return_topology_role(previous) || return_topology_role(role))
        {
            self.rebuild_live_returns();
        }
        if !matches!(previous, LinkRole::Return { .. }) || !matches!(role, LinkRole::Return { .. })
        {
            self.prune_reentry();
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
                Edit::CompleteReturn { returned, path, .. } => {
                    validate_link(self, (*returned).into(), links)?;
                    for link in path.links() {
                        validate_link(self, link.into(), links)?;
                    }
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
                Edit::CompleteReturn {
                    source,
                    returned,
                    path,
                    outcome_witness,
                    exact,
                    exclusive_source,
                    offers_choice,
                    at,
                } => {
                    let memory = self
                        .link_memory
                        .get(returned.slot())
                        .ok_or(ApplyError::UnknownLink(returned))?;
                    let was_live = memory.live;
                    let role = memory.role;
                    if self.needs_automatic_closure() {
                        self.complete_automatic_witness(returned, path, exact);
                    }
                    if was_live {
                        debug_assert!(matches!(role, LinkRole::Return { .. }));
                        if matches!(role, LinkRole::Return { .. }) {
                            if exclusive_source {
                                self.remove_exclusive_live_return(source, returned);
                            } else {
                                self.remove_live_return_with_path(returned, Some(path));
                            }
                        }
                        self.link_memory[returned.slot()].live = false;
                    }
                    let mut exact_closures = self.link_memory[path.first.slot()].exact_closures;
                    if exact {
                        let memory = self
                            .link_memory
                            .get_mut(path.first.slot())
                            .ok_or(ApplyError::UnknownLink(path.first))?;
                        memory.exact_closures = memory.exact_closures.saturating_add(1);
                        exact_closures = memory.exact_closures;
                    }
                    for link in path.links() {
                        let memory = self
                            .link_memory
                            .get_mut(link.slot())
                            .ok_or(ApplyError::UnknownLink(link))?;
                        memory.outcome_at = offers_choice.then_some(at);
                        memory.outcome_available = offers_choice;
                        memory.boundary_closed |= !offers_choice;
                        let before = memory.strength;
                        let after = before.saturating_add(1);
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
                    self.retain_closed_step(
                        returned,
                        source,
                        path,
                        outcome_witness,
                        exact,
                        exclusive_source && exact_closures > u8::from(exact),
                    );
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
                            self.link_memory[link.slot()].outcome_available = false;
                            if self.returns.live_count != 0 && return_topology_role(role) {
                                self.rebuild_live_returns();
                            }
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
                            memory.boundary_closed = false;
                        }
                        LinkChange::RememberOutcome {
                            at,
                            available_until_choice,
                        } => {
                            memory.outcome_at = Some(at);
                            memory.outcome_available = available_until_choice;
                        }
                        LinkChange::LearnOutcome {
                            at,
                            available_until_choice,
                            strength,
                        } => {
                            memory.outcome_at = Some(at);
                            memory.outcome_available = available_until_choice;
                            let before = memory.strength;
                            let after = before.saturating_add(i64::from(strength));
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
                        LinkChange::ConsumeOutcome => memory.outcome_available = false,
                        LinkChange::ClearOutcomeSelection => {
                            memory.outcome_at = None;
                            memory.outcome_available = false;
                        }
                        LinkChange::InhibitBoundaryChoice => {
                            memory.boundary_inhibited = true;
                        }
                        LinkChange::ConsumeBoundaryInhibition => {
                            memory.boundary_inhibited = false;
                        }
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
    use crate::{verify_choice_laws, Arrival};
    use proptest::prelude::*;

    fn ready_path(drive: u16, stable_order: u32) -> ReadyPath {
        let surface = JunctionId::new(0).unwrap();
        let output = JunctionId::new(stable_order as usize + 1).unwrap();
        ReadyPath {
            surface,
            middle: JunctionId::new(stable_order as usize + 3).unwrap().into(),
            output,
            first: LinkId::new(stable_order as usize * 2).unwrap().into(),
            second: LinkId::new(stable_order as usize * 2 + 1).unwrap().into(),
            at: 1,
            current_cause: 1,
            return_cause: None,
            unanswered: false,
            connected_start: 0,
            connected_end: 0,
            outcome: None,
            participation: 0,
            participated_at: 0,
            output_participated: false,
            outcome_source: None,
            progress_source: None,
            resisted_progress: false,
            boundary_open: false,
            boundary_inhibited: false,
            strength: 1,
            drive,
            stable_order,
            fresh_opportunity: None,
            reentries: Vec::new(),
            reentry_incidence_visits: 0,
            reentry_failed: false,
            executable: true,
        }
    }

    #[test]
    fn current_normalized_drive_breaks_an_unlearned_choice_tie() {
        let paths = [ready_path(512, 0), ready_path(513, 1)];

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 1);
        assert_eq!(choice.basis, ChoiceBasis::ParticipationStrengthAndDrive);
    }

    fn witnessed_reentry(path: &ReadyPath, condition: JunctionId) -> ReentryTrace {
        let (JunctionRef::Existing(middle), LinkRef::Existing(first), LinkRef::Existing(second)) =
            (path.middle, path.first, path.second)
        else {
            unreachable!()
        };
        ReentryTrace {
            condition,
            steps: vec![ReentryStepTrace {
                path: Path {
                    surface: path.surface,
                    middle,
                    output: path.output,
                    first,
                    second,
                },
                returned_source: condition,
                outcome_witness: LinkId::new(20).unwrap(),
                outcome_target: path.output,
            }],
        }
    }

    #[test]
    fn actual_current_return_precedes_unique_reentry() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        paths[0].reentries = vec![witnessed_reentry(&paths[0], JunctionId::new(20).unwrap())];
        paths[1].return_cause = Some(paths[1].current_cause);

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 1);
        assert_eq!(choice.basis, ChoiceBasis::CurrentReturn);
    }

    #[test]
    fn actual_retained_progress_precedes_unique_reentry() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        paths[0].reentries = vec![witnessed_reentry(&paths[0], JunctionId::new(20).unwrap())];
        paths[1].resisted_progress = true;
        paths[1].boundary_open = true;
        paths[1].strength = 2;
        paths[1].participation = 1;

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 1);
        assert_eq!(choice.basis, ChoiceBasis::RetainedProgress);
    }

    #[test]
    fn current_choice_surface_is_not_a_present_reentry_condition() {
        let mut body = Body::default();
        let surface = body.add_junction(Junction::integrating(1)).unwrap();
        let middle = body.add_junction(Junction::integrating(1)).unwrap();
        let output = body.add_junction(Junction::integrating(1)).unwrap();
        let first = body.add_link(Link::new(surface, middle, 1, 1)).unwrap();
        body.set_link_role(first, LinkRole::PathEntry).unwrap();
        let second = body.add_link(Link::new(middle, output, 1, 1)).unwrap();
        body.link_memory[first.slot()].participation = 1;
        body.link_memory[second.slot()].participation = 1;
        let witness = body.add_link(Link::new(surface, output, 0, 1)).unwrap();
        body.set_link_role(witness, LinkRole::OutcomeWitness)
            .unwrap();
        let returned = body.add_link(Link::new(output, middle, 0, 0)).unwrap();
        body.set_link_role(
            returned,
            LinkRole::Return {
                cause: 1,
                cohort: 1,
            },
        )
        .unwrap();
        body.link_memory[returned.slot()].live = false;
        body.link_memory[returned.slot()].remember_closed_support(surface, witness);
        let view = ReactionView::new(&body.arena, &body.link_memory, &body.returns);
        let mut paths = Vec::new();
        let mut connected = Vec::new();
        append_existing_ready_paths(view, surface, 7, 1, 1, &mut paths, &mut connected);
        let facts = [MomentFact {
            event: crate::physics::Event {
                at: 7,
                junction: surface,
                arrivals: 1,
                impulse: 1,
                before: 0,
                after: 1,
                cause: 1,
            },
            drive: 1,
            boundary: true,
            used: UsedPaths::None,
            had_ready_path: true,
        }];

        mark_reentries(
            view,
            &facts,
            &mut paths,
            &mut ReentryScratch::default(),
            false,
        );

        assert!(paths.iter().all(|path| path.reentries.is_empty()));
    }

    #[test]
    fn cyclic_closed_steps_fail_reentry_closed() {
        let mut body = Body::default();
        let surfaces = [
            body.add_junction(Junction::integrating(1)).unwrap(),
            body.add_junction(Junction::integrating(1)).unwrap(),
        ];
        let middles = [
            body.add_junction(Junction::integrating(1)).unwrap(),
            body.add_junction(Junction::integrating(1)).unwrap(),
        ];
        let outputs = [
            body.add_junction(Junction::integrating(1)).unwrap(),
            body.add_junction(Junction::integrating(1)).unwrap(),
        ];
        let mut paths = Vec::new();
        for index in 0..2 {
            let first = body
                .add_link(Link::new(surfaces[index], middles[index], 1, 1))
                .unwrap();
            body.set_link_role(first, LinkRole::PathEntry).unwrap();
            let second = body
                .add_link(Link::new(middles[index], outputs[index], 1, 1))
                .unwrap();
            body.link_memory[first.slot()].participation = 1;
            body.link_memory[second.slot()].participation = 1;
            let witness = body
                .add_link(Link::new(surfaces[1 - index], outputs[index], 0, 1))
                .unwrap();
            body.set_link_role(witness, LinkRole::OutcomeWitness)
                .unwrap();
            let returned = body
                .add_link(Link::new(outputs[index], middles[index], 0, 0))
                .unwrap();
            body.set_link_role(
                returned,
                LinkRole::Return {
                    cause: 1,
                    cohort: 1,
                },
            )
            .unwrap();
            body.link_memory[returned.slot()].live = false;
            body.link_memory[returned.slot()].remember_closed_support(surfaces[1 - index], witness);
            paths.push(Path {
                surface: surfaces[index],
                middle: middles[index],
                output: outputs[index],
                first,
                second,
            });
        }
        let present = body.add_junction(Junction::integrating(1)).unwrap();
        let view = ReactionView::new(&body.arena, &body.link_memory, &body.returns);

        let mut steps = Vec::new();
        let mut continuations = Vec::new();
        let mut incidence_visits = 0;
        assert_eq!(
            find_reentries(
                view,
                paths[0],
                &[present],
                &mut steps,
                &mut continuations,
                &mut incidence_visits,
            ),
            Err(())
        );
    }

    #[test]
    fn old_outcome_cannot_override_a_stronger_current_surface() {
        let mut paths = [ready_path(44, 0), ready_path(1_023, 1)];
        paths[0].outcome = Some(Outcome {
            at: 1,
            caused_transition: true,
            available_until_choice: true,
        });

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 1);
    }

    #[test]
    fn equal_current_drive_preserves_physical_stable_order() {
        let paths = [ready_path(512, 0), ready_path(512, 1)];

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 0);
    }

    #[test]
    fn a_participating_output_continues_its_current_return_after_both_outputs_were_tried() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        paths[0].output_participated = true;
        paths[1].participation = 1;

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 0);
        assert_eq!(choice.basis, ChoiceBasis::CurrentReturn);
    }

    #[test]
    fn one_retained_progressing_output_precedes_untried_release() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        paths[0].resisted_progress = true;
        paths[0].boundary_open = true;
        paths[0].participation = 1;
        paths[0].strength = 2;

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 0);
        assert_eq!(choice.basis, ChoiceBasis::RetainedProgress);
    }

    #[test]
    fn unanswered_without_fresh_progress_releases_to_the_untried_output() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        paths[0].unanswered = true;
        paths[0].participation = 1;

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 1);
        assert_eq!(choice.basis, ChoiceBasis::UntriedOutputRelease);
    }

    #[test]
    fn a_newer_unanswered_reuse_releases_an_older_success_to_a_tried_alternative() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        paths[0].participation = 2;
        paths[0].participated_at = 20;
        paths[0].unanswered = true;
        paths[0].outcome = Some(Outcome {
            at: 10,
            caused_transition: true,
            available_until_choice: false,
        });
        paths[0].strength = 3;
        paths[1].participation = 1;
        paths[1].participated_at = 15;

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 1);
        assert_eq!(choice.basis, ChoiceBasis::UnansweredOutputRelease);
    }

    #[test]
    fn an_exact_return_precedes_unanswered_output_release() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        paths[0].participation = 2;
        paths[0].participated_at = 20;
        paths[0].unanswered = true;
        paths[0].return_cause = Some(paths[0].current_cause);
        paths[0].outcome = Some(Outcome {
            at: 10,
            caused_transition: true,
            available_until_choice: false,
        });
        paths[1].participation = 1;
        paths[1].participated_at = 15;

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 0);
        assert_eq!(choice.basis, ChoiceBasis::CurrentReturn);
    }

    #[test]
    fn a_lone_unanswered_output_does_not_invent_an_alternative() {
        let mut path = ready_path(512, 0);
        path.participation = 2;
        path.participated_at = 20;
        path.unanswered = true;
        path.outcome = Some(Outcome {
            at: 10,
            caused_transition: true,
            available_until_choice: false,
        });

        let choice = choose_ready(&[path], &[0], 0, false).unwrap();

        assert_eq!(choice.winner, 0);
        assert_eq!(choice.basis, ChoiceBasis::LatestOutcome);
    }

    #[test]
    fn simultaneous_unanswered_outputs_make_no_unique_release_claim() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        for path in &mut paths {
            path.participation = 1;
            path.participated_at = 20;
            path.unanswered = true;
        }

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 0);
        assert_eq!(choice.basis, ChoiceBasis::ParticipationStrengthAndDrive);
    }

    #[test]
    fn unclosed_exploration_cannot_claim_continuation() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        paths[0].unanswered = true;
        paths[0].resisted_progress = true;
        paths[0].boundary_open = true;
        paths[0].participation = 1;

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 1);
        assert_eq!(choice.basis, ChoiceBasis::UntriedOutputRelease);
    }

    #[test]
    fn retained_output_with_fresh_progress_continues_after_ordinary_outcome() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        paths[0].resisted_progress = true;
        paths[0].boundary_open = true;
        paths[0].participation = 1;
        paths[0].strength = 2;

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 0);
        assert_eq!(choice.basis, ChoiceBasis::RetainedProgress);
    }

    #[test]
    fn several_progressing_outputs_receive_no_continuation_precedence() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        for path in &mut paths {
            path.unanswered = true;
            path.resisted_progress = true;
            path.boundary_open = true;
            path.participation = 1;
            path.strength = 2;
        }

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_ne!(choice.basis, ChoiceBasis::RetainedProgress);
    }

    #[test]
    fn boundary_closed_output_cannot_claim_progress_continuation() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1)];
        paths[0].resisted_progress = true;
        paths[0].participation = 1;
        paths[0].strength = 2;

        let choice = choose_ready(&paths, &[0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 1);
        assert_ne!(choice.basis, ChoiceBasis::RetainedProgress);
    }

    #[test]
    fn boundary_completion_releases_only_the_local_antagonist() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1), ready_path(900, 2)];
        let local = JunctionId::new(20).unwrap();
        paths[0].outcome_source = Some(local);
        paths[0].boundary_inhibited = true;
        paths[1].outcome_source = Some(local);
        paths[2].outcome_source = Some(JunctionId::new(21).unwrap());
        paths[2].participation = 100;
        paths[2].strength = 100;

        let choice = choose_ready(&paths, &[0, 0, 0], 0, false).unwrap();

        assert_eq!(choice.winner, 1);
        assert_eq!(choice.basis, ChoiceBasis::BoundaryRelease);
    }

    #[test]
    fn simultaneous_boundary_components_make_no_local_release_claim() {
        let mut paths = [ready_path(512, 0), ready_path(512, 1), ready_path(512, 2)];
        paths[0].outcome_source = Some(JunctionId::new(20).unwrap());
        paths[0].boundary_inhibited = true;
        paths[1].outcome_source = Some(JunctionId::new(21).unwrap());
        paths[1].boundary_inhibited = true;
        paths[2].outcome_source = Some(JunctionId::new(20).unwrap());

        assert!(choose_ready(&paths, &[0, 0, 0], 0, false).is_none());
    }

    #[test]
    fn a_later_path_action_supersedes_a_stale_fresh_witness() {
        let mut body = Body::default();
        let source = body.add_junction(Junction::integrating(1)).unwrap();
        let middle = body.add_junction(Junction::integrating(1)).unwrap();
        let outputs: [JunctionId; 2] =
            std::array::from_fn(|_| body.add_junction(Junction::integrating(1)).unwrap());
        let witnesses = outputs.map(|output| {
            let witness = body.add_link(Link::new(source, output, 0, 1)).unwrap();
            body.set_link_role(witness, LinkRole::OutcomeWitness)
                .unwrap();
            witness
        });
        let drive = body.add_link(Link::new(middle, outputs[0], 1, 1)).unwrap();
        body.link_memory[drive.slot()].participation = 1;
        body.link_memory[drive.slot()].participated_at = 10;
        body.link_memory[witnesses[1].slot()].record_transmission(2, 20);

        let view = ReactionView::new(&body.arena, &body.link_memory, &body.returns);
        assert_eq!(latest_fresh_output(view, source), Some(outputs[1]));

        body.link_memory[drive.slot()].participated_at = 21;
        let view = ReactionView::new(&body.arena, &body.link_memory, &body.returns);
        assert_eq!(latest_fresh_output(view, source), None);
    }

    #[test]
    fn simultaneous_fresh_outputs_claim_no_single_return() {
        let mut body = Body::default();
        let source = body.add_junction(Junction::integrating(1)).unwrap();
        let outputs: [JunctionId; 2] =
            std::array::from_fn(|_| body.add_junction(Junction::integrating(1)).unwrap());
        for output in outputs {
            let witness = body.add_link(Link::new(source, output, 0, 1)).unwrap();
            body.set_link_role(witness, LinkRole::OutcomeWitness)
                .unwrap();
            body.link_memory[witness.slot()].record_transmission(1, 20);
        }

        let view = ReactionView::new(&body.arena, &body.link_memory, &body.returns);
        assert_eq!(latest_fresh_output(view, source), None);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn every_ready_choice_satisfies_the_offline_laws(
            left_drive in 0_u16..=1_023,
            right_drive in 0_u16..=1_023,
            left_participation in 0_u64..4,
            right_participation in 0_u64..4,
            left_participated_at in 0_u64..4,
            right_participated_at in 0_u64..4,
            left_unanswered in any::<bool>(),
            right_unanswered in any::<bool>(),
            left_output_participated in any::<bool>(),
            right_output_participated in any::<bool>(),
            left_strength in -2_i64..=2,
            right_strength in -2_i64..=2,
            left_outcome_at in prop::option::of(0_u64..4),
            right_outcome_at in prop::option::of(0_u64..4),
            left_available in any::<bool>(),
            right_available in any::<bool>(),
            left_returns in any::<bool>(),
            right_returns in any::<bool>(),
            left_executable in any::<bool>(),
            right_executable in any::<bool>(),
            construction in any::<bool>(),
        ) {
            let mut paths = [ready_path(left_drive, 0), ready_path(right_drive, 1)];
            paths[0].participation = left_participation;
            paths[1].participation = right_participation;
            paths[0].participated_at = left_participated_at;
            paths[1].participated_at = right_participated_at;
            paths[0].unanswered = left_unanswered;
            paths[1].unanswered = right_unanswered;
            paths[0].output_participated = left_output_participated;
            paths[1].output_participated = right_output_participated;
            paths[0].strength = left_strength;
            paths[1].strength = right_strength;
            paths[0].outcome = left_outcome_at.map(|at| Outcome {
                at,
                caused_transition: true,
                available_until_choice: left_available,
            });
            paths[1].outcome = right_outcome_at.map(|at| Outcome {
                at,
                caused_transition: true,
                available_until_choice: right_available,
            });
            paths[0].return_cause = left_returns.then_some(paths[0].current_cause);
            paths[1].return_cause = right_returns.then_some(paths[1].current_cause);
            paths[0].executable = left_executable;
            paths[1].executable = right_executable;

            let selected = choose_ready(&paths, &[0, 0], 0, construction);
            let candidates = paths.iter().map(|path| CandidateTrace {
                at: path.at,
                cause: path.current_cause,
                group: 0,
                path: path.trace_path(),
                connected_outcomes: Vec::new(),
                executable: path.executable,
                return_cause: path.return_cause,
                unanswered: path.unanswered,
                outcome: path.outcome,
                participation: path.participation,
                participated_at: path.participated_at,
                output_participated: path.output_participated,
                outcome_source: path.outcome_source,
                progress_source: path.progress_source,
                resisted_progress: path.resisted_progress,
                boundary_open: path.boundary_open,
                boundary_inhibited: path.boundary_inhibited,
                strength: path.strength,
                drive: path.drive,
                stable_order: path.stable_order,
                fresh_opportunity: path.fresh_opportunity,
                present_sources: Vec::new(),
                reentries: path.reentries.clone(),
                reentry_incidence_visits: path.reentry_incidence_visits,
                reentry_failed: path.reentry_failed,
                new_path: false,
            });
            let mut events = candidates.map(TraceEvent::Candidate).collect::<Vec<_>>();
            events.push(TraceEvent::Choice(ChoiceTrace {
                at: 1,
                group: 0,
                alternatives: paths.len(),
                winner: selected.map(|choice| paths[choice.winner].trace_path()),
                basis: selected.map(|choice| choice.basis),
                construction,
                sent: selected.is_some() && !construction,
            }));

            prop_assert_eq!(verify_choice_laws(&events), Ok(()));
        }
    }

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

        let view = ReactionView::new(&body.arena, &body.link_memory, &body.returns);
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
    fn live_return_count_tracks_roles_and_retirement() {
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

        assert_eq!(body.returns.live_count, 3);
        assert_eq!(body.clone().returns.live_count, body.returns.live_count);

        let mut change = Change::empty();
        change.push(Edit::ChangeLink {
            link: second.into(),
            change: LinkChange::Retire,
        });
        body.apply(change).unwrap();
        body.set_link_role(first, LinkRole::Drive).unwrap();

        assert_eq!(body.returns.live_count, 1);
        assert!(body.link_memory[third.slot()].live);
    }

    #[test]
    fn live_returns_are_indexed_by_their_own_outcome_source() {
        let mut body = Body::default();
        let mut outcomes = Vec::new();
        for index in 0..2_u64 {
            let motor = motor(&mut body);
            let sensor = attach_sensor(
                &mut body,
                Junction::integrating(1),
                &[(motor.opportunity, 1)],
            );
            let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
            attach_outcome_component(&mut body, outcome, [motor.opportunity]);
            schedule(&mut body, index * 4, &[reading(outcome, 0, 0, 0)]);
            finish(&mut body);
            schedule(
                &mut body,
                1 + index * 4,
                &[reading(sensor, 0, 1, index + 1)],
            );
            schedule(
                &mut body,
                2 + index * 4,
                &[Arrival::caused(motor.opportunity, 1, index + 1)],
            );
            finish(&mut body);
            outcomes.push(outcome);
        }

        assert_eq!(body.returns.live_count, 2);
        assert_eq!(body.returns.by_source[outcomes[0].slot()].len(), 1);
        assert_eq!(body.returns.by_source[outcomes[1].slot()].len(), 1);
        assert_ne!(
            body.returns.by_source[outcomes[0].slot()][0].link,
            body.returns.by_source[outcomes[1].slot()][0].link
        );
        assert_eq!(
            body.returns.by_source.iter().map(Vec::len).sum::<usize>(),
            2
        );
    }

    #[test]
    fn completing_a_shared_return_retires_it_from_every_outcome_source() {
        let mut body = Body::default();
        let motor = motor(&mut body);
        let sensor = attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        );
        let outcomes: [JunctionId; 2] = std::array::from_fn(|_| {
            let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
            attach_outcome_component(&mut body, outcome, [motor.opportunity]);
            outcome
        });
        schedule(
            &mut body,
            0,
            &outcomes.map(|outcome| reading(outcome, 0, 0, 0)),
        );
        finish(&mut body);
        schedule(&mut body, 1, &[reading(sensor, 0, 1, 1)]);
        schedule(&mut body, 2, &[Arrival::caused(motor.opportunity, 1, 1)]);
        finish(&mut body);

        assert_eq!(body.returns.live_count, 1);
        assert!(outcomes.iter().all(|outcome| {
            let returns = &body.returns.by_source[outcome.slot()];
            returns.len() == 1 && !returns[0].exclusive_source
        }));

        schedule(&mut body, 3, &[Arrival::caused(outcomes[0], 1, 2)]);
        finish(&mut body);

        assert_eq!(body.returns.live_count, 0);
        assert!(outcomes
            .iter()
            .all(|outcome| body.returns.by_source[outcome.slot()].is_empty()));
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
