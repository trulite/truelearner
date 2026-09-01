pub type Cause = u64;
pub type Cohort = u64;
const LOCAL_RADIUS: i32 = 2;
const AUTOMATIC_AFTER_EXACT_CLOSURES: u8 = 3;
const THOUGHT_SHORTCUT_AFTER_REHEARSALS: u8 = 3;
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
pub(crate) struct Automaticity {
    pub(crate) closure_maintenance: bool,
    witnesses: Vec<AutomaticWitness>,
    evidence: Vec<AutomaticEvidence>,
    reentry_epochs: Vec<u64>,
    thought_shortcuts: Vec<ThoughtShortcut>,
    pub(crate) generic_composites: bool,
    work: AutomaticityWork,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AutomaticityV7 {
    closure_maintenance: bool,
    witnesses: Vec<AutomaticWitness>,
    evidence: Vec<AutomaticEvidence>,
    generic_composites: bool,
    work: AutomaticityWork,
}

impl From<AutomaticityV7> for Automaticity {
    fn from(previous: AutomaticityV7) -> Self {
        Self {
            closure_maintenance: previous.closure_maintenance,
            witnesses: previous.witnesses,
            evidence: previous.evidence,
            reentry_epochs: Vec::new(),
            thought_shortcuts: Vec::new(),
            generic_composites: previous.generic_composites,
            work: previous.work,
        }
    }
}

impl Automaticity {
    pub(crate) fn remap(&mut self, junction_base: usize, link_base: usize) {
        for witness in &mut self.witnesses {
            witness.remap(junction_base, link_base);
        }
        for evidence in &mut self.evidence {
            evidence.owner = remap_link(evidence.owner, link_base);
            evidence.pair.remap_links(link_base);
        }
        let mut remapped_epochs = vec![0; junction_base];
        remapped_epochs.append(&mut self.reentry_epochs);
        self.reentry_epochs = remapped_epochs;
        for shortcut in &mut self.thought_shortcuts {
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
        self.thought_shortcuts
            .sort_unstable_by_key(|shortcut| (shortcut.start, shortcut.condition));
    }

    pub(crate) fn append(&mut self, mut other: Self) {
        self.closure_maintenance |= other.closure_maintenance;
        self.witnesses.append(&mut other.witnesses);
        self.evidence.append(&mut other.evidence);
        if self.reentry_epochs.len() < other.reentry_epochs.len() {
            self.reentry_epochs.resize(other.reentry_epochs.len(), 0);
        }
        for (slot, epoch) in other.reentry_epochs.into_iter().enumerate() {
            self.reentry_epochs[slot] = self.reentry_epochs[slot].max(epoch);
        }
        self.thought_shortcuts.append(&mut other.thought_shortcuts);
        self.thought_shortcuts
            .sort_unstable_by_key(|shortcut| (shortcut.start, shortcut.condition));
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

    fn reentry_epoch(&self, junction: JunctionId) -> u64 {
        self.reentry_epochs
            .get(junction.slot())
            .copied()
            .unwrap_or(0)
    }

    fn touch_reentry(&mut self, junction: JunctionId) {
        if self.reentry_epochs.len() <= junction.slot() {
            self.reentry_epochs.resize(junction.slot() + 1, 0);
        }
        self.reentry_epochs[junction.slot()] =
            self.reentry_epochs[junction.slot()].saturating_add(1);
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
        self.thought_shortcuts
            .binary_search_by_key(&(start, condition), |shortcut| {
                (shortcut.start, shortcut.condition)
            })
            .ok()
            .map(|index| &self.thought_shortcuts[index])
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

impl Path {
    const fn links(self) -> [LinkId; 2] {
        [self.first, self.second]
    }
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
pub struct Outcome {
    pub at: Time,
    pub caused_transition: bool,
    pub available_until_choice: bool,
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
    RememberSwitchedFrom {
        prior: LinkId,
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
    automaticity: Option<&'a Automaticity>,
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
            automaticity: None,
        }
    }

    pub(crate) const fn with_automaticity(
        arena: &'a Arena,
        link_memory: &'a [LinkMemory],
        returns: &'a ReturnIndex,
        automaticity: Option<&'a Automaticity>,
    ) -> Self {
        Self {
            arena,
            link_memory,
            returns,
            automaticity,
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
