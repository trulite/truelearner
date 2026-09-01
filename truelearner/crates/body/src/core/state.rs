
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
        let motif_parent = self.motif_parent();
        let switched_from = self.switched_from();
        if let LinkRole::Composite { first, second } = self.role {
            self.role = LinkRole::Composite {
                first: remap_link(first, base),
                second: remap_link(second, base),
            };
        }
        if let Some((source, witness)) = self.closed_support() {
            self.remember_closed_support(source, remap_link(witness, base));
        }
        if let Some(parent) = motif_parent {
            self.remember_motif_parent(remap_link(parent, base));
        }
        if let Some(prior) = switched_from {
            self.remember_switched_from(remap_link(prior, base));
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

    fn remember_motif_parent(&mut self, parent: LinkId) {
        debug_assert!(!self.live && matches!(self.role, LinkRole::Return { .. }));
        self.participation = parent.slot() as u64;
        self.boundary_inhibited = true;
    }

    pub(crate) fn motif_parent(&self) -> Option<LinkId> {
        if self.closed_support().is_none() || !self.boundary_inhibited {
            return None;
        }
        usize::try_from(self.participation)
            .ok()
            .and_then(LinkId::new)
    }

    fn remember_switched_from(&mut self, prior: LinkId) {
        debug_assert!(self.live && matches!(self.role, LinkRole::Return { .. }));
        self.participation = prior.slot() as u64;
        self.boundary_inhibited = true;
    }

    fn switched_from(&self) -> Option<LinkId> {
        if !self.live || !matches!(self.role, LinkRole::Return { .. }) || !self.boundary_inhibited {
            return None;
        }
        usize::try_from(self.participation)
            .ok()
            .and_then(LinkId::new)
    }

    fn clear_switched_from(&mut self) {
        if self.live && matches!(self.role, LinkRole::Return { .. }) {
            self.boundary_inhibited = false;
        }
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
        let (from, to, changed, relevant) = {
            let physical = self.arena.link(link).ok_or(ApplyError::UnknownLink(link))?;
            let memory = self
                .link_memory
                .get(link.slot())
                .ok_or(ApplyError::UnknownLink(link))?;
            (
                physical.from,
                physical.to,
                physical.impulse != impulse,
                matches!(memory.role, LinkRole::PathEntry | LinkRole::Drive),
            )
        };
        let physical = self
            .arena
            .link_mut(link)
            .ok_or(ApplyError::UnknownLink(link))?;
        physical.impulse = impulse;
        if changed && relevant {
            self.touch_reentry_junctions([from, to]);
        }
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
        let physical = *self.arena.link(link).ok_or(ApplyError::UnknownLink(link))?;
        if live {
            self.remove_live_return(link, previous);
        }
        self.link_memory[link.slot()].role = role;
        let transient_return =
            matches!(role, LinkRole::Return { .. }) && physical.delay == 0 && physical.impulse == 0;
        let reentry_role = |role| {
            matches!(
                role,
                LinkRole::PathEntry
                    | LinkRole::Drive
                    | LinkRole::OutcomeWitness
                    | LinkRole::Membership
            )
        };
        if previous != role && !transient_return && (reentry_role(previous) || reentry_role(role)) {
            self.touch_reentry_junctions([physical.from, physical.to]);
        }
        if previous != role && (previous == LinkRole::Membership || role == LinkRole::Membership) {
            self.touch_path_entries_from(physical.to);
        }
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
                Edit::Send { through, .. } => {
                    validate_link(self, *through, links)?;
                }
                Edit::ChangeLink { link, change } => {
                    validate_link(self, *link, links)?;
                    if let LinkChange::RememberSwitchedFrom { prior } = change {
                        validate_link(self, (*prior).into(), links)?;
                    }
                }
                Edit::CompleteReturn {
                    returned,
                    path,
                    motif_parent,
                    ..
                } => {
                    validate_link(self, (*returned).into(), links)?;
                    for link in path.links() {
                        validate_link(self, link.into(), links)?;
                    }
                    if let Some(parent) = motif_parent {
                        validate_link(self, (*parent).into(), links)?;
                    }
                }
                Edit::RehearseReentry {
                    start,
                    condition,
                    routes,
                    dependencies,
                } => {
                    for junction in [start.surface, start.middle, start.output, *condition]
                        .into_iter()
                        .chain(dependencies.iter().copied())
                        .chain(routes.iter().flat_map(|route| {
                            std::iter::once(route.condition).chain(route.steps.iter().flat_map(
                                |step| {
                                    [
                                        step.path.surface,
                                        step.path.middle,
                                        step.path.output,
                                        step.returned_source,
                                        step.outcome_target,
                                    ]
                                },
                            ))
                        }))
                    {
                        if self.arena.junction(junction).is_none() {
                            return Err(ApplyError::UnknownJunction(junction));
                        }
                    }
                    for link in start
                        .links()
                        .into_iter()
                        .chain(routes.iter().flat_map(|route| {
                            route.steps.iter().flat_map(|step| {
                                [step.path.first, step.path.second, step.outcome_witness]
                            })
                        }))
                    {
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
                        .add_link_untracked(
                            Link::new(from, to, spec.delay, spec.impulse).when(spec.trigger),
                        )
                        .map_err(ApplyError::Build)?;
                    self.set_link_role(id, spec.role)?;
                    if spec.role == LinkRole::Drive {
                        self.touch_reentry_junctions([from, to]);
                    }
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
                    motif_parent,
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
                        self.link_memory[returned.slot()].clear_switched_from();
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
                    if self.link_memory[returned.slot()].closed_support().is_some() {
                        if let Some(parent) = motif_parent {
                            self.link_memory[returned.slot()].remember_motif_parent(parent);
                        }
                    }
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
                        let physical =
                            *self.arena.link(link).ok_or(ApplyError::UnknownLink(link))?;
                        if was_live {
                            self.remove_live_return(link, role);
                            self.link_memory[link.slot()].live = false;
                            self.link_memory[link.slot()].outcome_available = false;
                            if self.returns.live_count != 0 && return_topology_role(role) {
                                self.rebuild_live_returns();
                            }
                            if matches!(
                                role,
                                LinkRole::PathEntry
                                    | LinkRole::Drive
                                    | LinkRole::OutcomeWitness
                                    | LinkRole::Membership
                            ) {
                                self.touch_reentry_junctions([physical.from, physical.to]);
                            }
                            if role == LinkRole::Membership {
                                self.touch_path_entries_from(physical.to);
                            }
                        }
                        continue;
                    }
                    let physical = *self.arena.link(link).ok_or(ApplyError::UnknownLink(link))?;
                    let mut touch_reentry = false;
                    let memory = self
                        .link_memory
                        .get_mut(link.slot())
                        .ok_or(ApplyError::UnknownLink(link))?;
                    match change {
                        LinkChange::Participated { cause, at } => {
                            touch_reentry = memory.participation == 0;
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
                        LinkChange::RememberSwitchedFrom { prior } => {
                            memory.remember_switched_from(prior);
                        }
                        LinkChange::Retire => unreachable!("retirement handled before mutation"),
                    }
                    if touch_reentry {
                        self.touch_reentry_junctions([physical.from, physical.to]);
                    }
                }
                Edit::RehearseReentry {
                    start,
                    condition,
                    routes,
                    dependencies,
                } => self.rehearse_reentry(start, condition, routes, dependencies),
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
