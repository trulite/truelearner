const LOCAL_RADIUS: i32 = 2;
const AUTOMATIC_AFTER_EXACT_CLOSURES: u8 = 3;
const THOUGHT_SHORTCUT_AFTER_REHEARSALS: u8 = 3;
const MAX_REENTRY_DEPTH: usize = 16;
const MAX_REENTRY_INCIDENCE_VISITS: u16 = 256;

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
    fn remap(&mut self, junction_base: usize, link_base: usize) {
        self.returned = remap_link(self.returned, link_base);
        remap_path(&mut self.path, junction_base, link_base);
        for pair in &mut self.pairs {
            pair.remap_links(link_base);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct AutomaticEvidence {
    owner: LinkId,
    pair: AutomaticPair,
    exact_closures: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ReentryDependency {
    junction: JunctionId,
    epoch: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ThoughtShortcut {
    start: Path,
    condition: JunctionId,
    routes: Vec<ReentryTrace>,
    dependencies: Vec<ReentryDependency>,
    rehearsals: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Consolidation {
    pub(crate) closure_maintenance: bool,
    witnesses: Vec<AutomaticWitness>,
    evidence: Vec<AutomaticEvidence>,
    work: AutomaticityWork,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ReentryCache {
    epochs: Vec<u64>,
    shortcuts: Vec<ThoughtShortcut>,
}

impl Consolidation {
    pub(crate) fn remap(&mut self, junction_base: usize, link_base: usize) {
        for witness in &mut self.witnesses {
            witness.remap(junction_base, link_base);
        }
        for evidence in &mut self.evidence {
            evidence.owner = remap_link(evidence.owner, link_base);
            evidence.pair.remap_links(link_base);
        }
    }

    pub(crate) fn append(&mut self, mut other: Self) {
        self.closure_maintenance |= other.closure_maintenance;
        self.witnesses.append(&mut other.witnesses);
        self.evidence.append(&mut other.evidence);
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

impl ReentryCache {
    pub(crate) fn remap(&mut self, junction_base: usize, link_base: usize) {
        let mut remapped_epochs = vec![0; junction_base];
        remapped_epochs.append(&mut self.epochs);
        self.epochs = remapped_epochs;
        for shortcut in &mut self.shortcuts {
            remap_path(&mut shortcut.start, junction_base, link_base);
            shortcut.condition = remap_junction(shortcut.condition, junction_base);
            for route in &mut shortcut.routes {
                route.condition = remap_junction(route.condition, junction_base);
                for step in &mut route.steps {
                    remap_path(&mut step.path, junction_base, link_base);
                    step.returned_source = remap_junction(step.returned_source, junction_base);
                    step.outcome_witness = remap_link(step.outcome_witness, link_base);
                    step.outcome_target = remap_junction(step.outcome_target, junction_base);
                }
            }
            for dependency in &mut shortcut.dependencies {
                dependency.junction = remap_junction(dependency.junction, junction_base);
            }
        }
        self.shortcuts
            .sort_unstable_by_key(|shortcut| (shortcut.start, shortcut.condition));
    }

    pub(crate) fn append(&mut self, mut other: Self) {
        if self.epochs.len() < other.epochs.len() {
            self.epochs.resize(other.epochs.len(), 0);
        }
        for (slot, epoch) in other.epochs.into_iter().enumerate() {
            self.epochs[slot] = self.epochs[slot].max(epoch);
        }
        self.shortcuts.append(&mut other.shortcuts);
        self.shortcuts
            .sort_unstable_by_key(|shortcut| (shortcut.start, shortcut.condition));
    }

    fn reentry_epoch(&self, junction: JunctionId) -> u64 {
        self.epochs
            .get(junction.slot())
            .copied()
            .unwrap_or(0)
    }

    fn touch_reentry(&mut self, junction: JunctionId) {
        if self.epochs.len() <= junction.slot() {
            self.epochs.resize(junction.slot() + 1, 0);
        }
        self.epochs[junction.slot()] = self.epochs[junction.slot()].saturating_add(1);
    }

    fn shortcut_is_current(&self, shortcut: &ThoughtShortcut) -> bool {
        shortcut
            .dependencies
            .iter()
            .all(|dependency| self.reentry_epoch(dependency.junction) == dependency.epoch)
    }

    fn usable_thought_shortcut(
        &self,
        start: Path,
        condition: JunctionId,
    ) -> Option<&ThoughtShortcut> {
        self.shortcuts
            .binary_search_by_key(&(start, condition), |shortcut| {
                (shortcut.start, shortcut.condition)
            })
            .ok()
            .map(|index| &self.shortcuts[index])
            .filter(|shortcut| {
                shortcut.rehearsals >= THOUGHT_SHORTCUT_AFTER_REHEARSALS
                    && self.shortcut_is_current(shortcut)
            })
    }
}

fn remap_link(link: LinkId, base: usize) -> LinkId {
    LinkId::new(base + link.slot()).expect("validated attachment link identity")
}

fn remap_junction(junction: JunctionId, base: usize) -> JunctionId {
    JunctionId::new(base + junction.slot()).expect("validated attachment junction identity")
}

fn remap_path(path: &mut Path, junction_base: usize, link_base: usize) {
    path.surface = remap_junction(path.surface, junction_base);
    path.middle = remap_junction(path.middle, junction_base);
    path.output = remap_junction(path.output, junction_base);
    path.first = remap_link(path.first, link_base);
    path.second = remap_link(path.second, link_base);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReentryState {
    pub closed_steps: usize,
    pub thought_shortcuts: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ClosedStep {
    link: LinkId,
    path: Path,
    returned_source: JunctionId,
    outcome_witness: LinkId,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LinkSpec {
    pub delay: Time,
    pub impulse: Impulse,
    pub trigger: Trigger,
    pub state: ArrowState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LinkChange {
    Participated {
        cause: Cause,
        at: Time,
    },
    RememberOutcome {
        at: Time,
        available_until_choice: bool,
    },
    ConsumeOutcome,
    ClearOutcomeSelection,
    InhibitBoundaryChoice,
    ConsumeBoundaryInhibition,
    Strengthen {
        amount: i32,
    },
    RememberSwitchedFrom {
        prior: LinkId,
    },
    MarkAmbiguous {
        at: Time,
    },
    Retire,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Edit {
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
        motif_parent: Option<LinkId>,
        exact: bool,
        exclusive_source: bool,
        offers_choice: bool,
        at: Time,
    },
    RehearseReentry {
        start: Path,
        condition: JunctionId,
        routes: Vec<ReentryTrace>,
        dependencies: Vec<JunctionId>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Change {
    edits: Vec<Edit>,
    junctions: u32,
    links: u32,
}

impl Change {
    #[cfg(test)]
    pub fn empty() -> Self {
        Self::default()
    }

    fn add_junction(&mut self, spec: Junction) -> NewJunction {
        let id = NewJunction(self.junctions);
        self.junctions += 1;
        self.edits.push(Edit::AddJunction { new: id, spec });
        id
    }

    fn add_link(&mut self, from: JunctionRef, to: JunctionRef, spec: LinkSpec) -> NewLink {
        let id = NewLink(self.links);
        self.links += 1;
        self.edits.push(Edit::AddLink {
            new: id,
            from,
            to,
            spec,
        });
        id
    }

    fn send(&mut self, through: LinkRef, at: Time, cause: Cause) {
        self.edits.push(Edit::Send { through, at, cause });
    }

    fn change_link(&mut self, link: LinkRef, change: LinkChange) {
        self.edits.push(Edit::ChangeLink { link, change });
    }

    #[allow(clippy::too_many_arguments)]
    fn complete_return(
        &mut self,
        source: JunctionId,
        returned: LinkId,
        path: Path,
        outcome_witness: Option<LinkId>,
        motif_parent: Option<LinkId>,
        exact: bool,
        exclusive_source: bool,
        offers_choice: bool,
        at: Time,
    ) {
        self.edits.push(Edit::CompleteReturn {
            source,
            returned,
            path,
            outcome_witness,
            motif_parent,
            exact,
            exclusive_source,
            offers_choice,
            at,
        });
    }

    fn rehearse_reentry(
        &mut self,
        start: Path,
        condition: JunctionId,
        routes: Vec<ReentryTrace>,
        dependencies: Vec<JunctionId>,
    ) {
        self.edits.push(Edit::RehearseReentry {
            start,
            condition,
            routes,
            dependencies,
        });
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct ReentryRehearsal {
    start: Path,
    condition: JunctionId,
    routes: Vec<ReentryTrace>,
    dependencies: Vec<JunctionId>,
}

#[derive(Clone, Debug)]
struct ReentryFrame {
    start: Path,
    prefix_len: usize,
    found_start: usize,
    dependencies: Vec<JunctionId>,
}

#[derive(Clone, Debug, Default)]
struct ReentryCompilationScratch {
    frames: Vec<ReentryFrame>,
    rehearsals: Vec<ReentryRehearsal>,
}

#[derive(Clone, Debug, Default)]
struct ReentryScratch {
    present: Vec<JunctionId>,
    steps: Vec<ReentryStepTrace>,
    continuations: Vec<ReentryContinuation>,
    compilation: ReentryCompilationScratch,
}

impl ReentryScratch {
    fn clear(&mut self) {
        self.present.clear();
        self.steps.clear();
        self.continuations.clear();
        self.compilation.frames.clear();
        self.compilation.rehearsals.clear();
    }

    fn clear_search(&mut self) {
        self.steps.clear();
        self.continuations.clear();
        self.compilation.frames.clear();
        self.compilation.rehearsals.clear();
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
    ready: Vec<CandidatePath>,
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
    arrows: &'a [ArrowState],
    returns: &'a ReturnIndex,
    reentry: Option<&'a ReentryCache>,
}

impl<'a> ReactionView<'a> {
    pub(crate) const fn new(
        arena: &'a Arena,
        arrows: &'a [ArrowState],
        returns: &'a ReturnIndex,
    ) -> Self {
        Self {
            arena,
            arrows,
            returns,
            reentry: None,
        }
    }

    pub(crate) const fn with_reentry(
        arena: &'a Arena,
        arrows: &'a [ArrowState],
        returns: &'a ReturnIndex,
        reentry: Option<&'a ReentryCache>,
    ) -> Self {
        Self {
            arena,
            arrows,
            returns,
            reentry,
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
