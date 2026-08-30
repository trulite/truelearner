//! What changes because of one observed physical event.

use crate::{
    arena::Arena, engine::PhysicalMoment, Body, BuildError, Impulse, Junction, JunctionId, Link,
    LinkId, RunError, Time, Trigger,
};
use std::{cmp::Reverse, collections::BTreeSet};

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

#[derive(Clone, Debug, Default)]
pub(crate) struct ReactionScratch {
    facts: Vec<MomentFact>,
    ready: Vec<ReadyPath>,
    connected_outcomes: Vec<JunctionId>,
    worlds: Vec<usize>,
    winners: Vec<usize>,
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
    moment.changes.iter().any(|recorded| {
        !matches!(recorded.used, UsedPaths::None)
            || recorded.boundary && boundary_can_react(body, recorded.event.junction)
    })
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

pub(crate) fn react_into(
    body: ReactionView<'_>,
    moment: &PhysicalMoment,
    scratch: &mut ReactionScratch,
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
    form_and_choose(
        body,
        &mut scratch.facts,
        &mut scratch.ready,
        &mut scratch.connected_outcomes,
        &mut scratch.worlds,
        &mut scratch.winners,
        &mut scratch.change,
    );
    record_used_outputs(&scratch.facts, &mut scratch.change);
    record_returned_outcomes(body, &scratch.facts, &mut scratch.change);
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

fn record_returned_outcomes(body: ReactionView<'_>, facts: &[MomentFact], change: &mut Change) {
    for fact in facts {
        if !fact.boundary || fact.had_ready_path || !matches!(fact.used, UsedPaths::None) {
            continue;
        }

        let selected = select_live_return(body, fact.event.cause);
        if let Some(returned) = selected.filter(|returned| returned.opened_at <= fact.event.at) {
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
            let witness = change.new_link();
            change.push(Edit::AddLink {
                new: witness,
                from: fact.event.junction.into(),
                to: returned.path.middle.into(),
                spec: LinkSpec {
                    delay: 0,
                    impulse: 0,
                    trigger: Trigger::SourceFires,
                    role: LinkRole::OutcomeWitness,
                },
            });
        }
        for entry in body.live_returns {
            if open_return(body, *entry).is_some() {
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

fn select_live_return(body: ReactionView<'_>, cause: Cause) -> Option<OpenReturn> {
    let mut only = None;
    let mut total = 0_usize;
    let mut exact = None;
    let mut exact_total = 0_usize;
    for entry in body.live_returns {
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
    match (exact_total, total) {
        (1, _) => exact,
        (0, 1) => only,
        _ => None,
    }
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
    first: LinkRef,
    at: Time,
    current_cause: Cause,
    return_cause: Option<Cause>,
    connected_start: usize,
    connected_end: usize,
    outcome: Option<Outcome>,
    participation: u64,
    strength: i64,
    stable_order: u32,
}

fn form_and_choose(
    body: ReactionView<'_>,
    facts: &mut [MomentFact],
    ready: &mut Vec<ReadyPath>,
    connected_outcomes: &mut Vec<JunctionId>,
    worlds: &mut Vec<usize>,
    winners: &mut Vec<usize>,
    change: &mut Change,
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
                    first: first.into(),
                    at: fact.event.at,
                    current_cause: fact.event.cause,
                    return_cause: None,
                    connected_start,
                    connected_end,
                    outcome: None,
                    participation: 0,
                    strength: 1,
                    stable_order: u32::try_from(body.arena.link_count())
                        .unwrap_or(u32::MAX)
                        .saturating_add(second.0),
                });
            }
        }
    }

    fill_ready_worlds(ready, connected_outcomes, worlds);
    for world in 0..ready.len() {
        if worlds[world] == world {
            if let Some(winner) = choose_ready(ready, worlds, world) {
                winners.push(winner);
            }
        }
    }
    winners.sort_by_key(|winner| ready[*winner].surface);
    for winner in winners.iter().map(|index| &ready[*index]) {
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
                paths.push(ReadyPath {
                    surface,
                    first: first_id.into(),
                    at,
                    current_cause,
                    return_cause: (memory.participation > 0).then_some(memory.cause),
                    connected_start,
                    connected_end,
                    outcome: memory.outcome_at.map(|at| Outcome {
                        at,
                        caused_transition: true,
                        available_until_choice: memory.outcome_available,
                    }),
                    participation: memory.participation,
                    strength: memory.strength,
                    stable_order: second_id.slot() as u32,
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

fn choose_ready(paths: &[ReadyPath], worlds: &[usize], world: usize) -> Option<usize> {
    let in_world = |index: &usize| worlds[*index] == world;
    unique_ready((0..paths.len()).filter(in_world).filter(|index| {
        let path = &paths[*index];
        path.return_cause.is_some() && path.return_cause == Some(path.current_cause)
    }))
    .or_else(|| unique_latest_ready(paths, worlds, world, true))
    .or_else(|| unique_latest_ready(paths, worlds, world, false))
    .or_else(|| {
        (0..paths.len()).filter(in_world).max_by_key(|index| {
            let path = &paths[*index];
            (
                path.participation,
                path.strength,
                Reverse(path.stable_order),
            )
        })
    })
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
) -> Option<usize> {
    let latest = (0..paths.len())
        .filter(|index| worlds[*index] == world)
        .filter_map(|index| {
            paths[index]
                .outcome
                .filter(|outcome| !available_only || outcome.available_until_choice)
        })
        .map(|outcome| outcome.at)
        .max()?;
    unique_ready((0..paths.len()).filter(|index| {
        if worlds[*index] != world {
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
        self.apply_reusing(&mut change, &mut applied)?;
        Ok(applied)
    }

    pub(crate) fn apply_reusing(
        &mut self,
        change: &mut Change,
        applied: &mut Applied,
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
                            memory.strength = memory.strength.saturating_add(i64::from(amount));
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
}
