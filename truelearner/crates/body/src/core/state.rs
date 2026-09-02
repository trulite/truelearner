
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ReturnEntry {
    pub(crate) cause: Cause,
    pub(crate) link: LinkId,
    pub(crate) path: Path,
    pub(crate) exclusive_source: bool,
    pub(crate) offers_choice: bool,
    pub(crate) outcome_witness: Option<LinkId>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ReturnIndex {
    pub(crate) by_source: Vec<Vec<ReturnEntry>>,
    pub(crate) live_count: usize,
}

const fn touches_reentry(state: &ArrowState) -> bool {
    state.is_entry()
        || state.is_drive()
        || state.is_membership()
        || matches!(state.witness_kind(), Some(WitnessKind::Closure { offers_choice: true }))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApplyError {
    Build(BuildError),
    Run(RunError),
    UnknownJunction(JunctionId),
    UnknownLink(LinkId),
    InvalidLinkRole(LinkId),
    ForwardJunction(NewJunction),
    ForwardLink(NewLink),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Applied {
    pub junctions: Vec<JunctionId>,
    pub links: Vec<LinkId>,
}

impl Body {
    pub(crate) fn rebuild_live_returns(&mut self) {
        for returns in &mut self.returns.by_source {
            returns.clear();
        }
        self.returns.live_count = 0;
        for slot in 0..self.arrows.len() {
            if self.arrows[slot].open_return_data().is_none() {
                continue;
            }
            let link = LinkId::new(slot).expect("validated link identity");
            self.insert_live_return(link);
        }
    }

    fn insert_live_return(&mut self, link: LinkId) {
        let Some((path, cause, _, _)) = self.arrows[link.slot()].open_return_data() else {
            return;
        };
        self.returns.live_count += 1;
        let mut only_source = None;
        let mut multiple_sources = false;
        for target in [path.middle, path.output] {
            for witness in self.arena.incoming(target) {
                if !closes_return(&self.arrows[witness.slot()]) {
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
                let state = &self.arrows[witness.slot()];
                let Some(WitnessKind::Closure { offers_choice }) = state.witness_kind() else {
                    continue;
                };
                let source = self.arena.link(witness).expect("live outcome witness").from;
                sources.push((source, offers_choice));
            }
        }
        for (source, offers_choice) in sources {
            let view = ReactionView::new(&self.arena, &self.arrows, &self.returns);
            let entry = ReturnEntry {
                cause,
                link,
                path,
                exclusive_source,
                offers_choice,
                outcome_witness: unique_outcome_witness(view, source, path)
                    .map(|(witness, _)| witness),
            };
            let returns = &mut self.returns.by_source[source.slot()];
            if let Err(index) = returns.binary_search(&entry) {
                returns.insert(index, entry);
            }
        }
    }

    fn remove_live_return(&mut self, link: LinkId) {
        let Some((path, _, _, _)) = self.arrows[link.slot()].open_return_data() else {
            return;
        };
        self.expire_automatic_witness(link);
        self.remove_live_return_with_path(link, Some(path));
    }

    fn remove_live_return_with_path(&mut self, link: LinkId, path: Option<Path>) {
        self.expire_automatic_witness(link);
        if let Some(path) = path {
            self.invalidate_closed_step(path);
        }
        debug_assert!(self.returns.live_count > 0, "live return count drift");
        self.returns.live_count -= 1;
        let (arena, arrows, returns_by_source) =
            (&self.arena, &self.arrows, &mut self.returns.by_source);
        let Some(path) = path else {
            for returns in returns_by_source {
                returns.retain(|entry| entry.link != link);
            }
            return;
        };
        for target in [path.middle, path.output] {
            for witness in arena.incoming(target) {
                if !closes_return(&arrows[witness.slot()]) {
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
                .arrows
                .get(link.slot())
                .ok_or(ApplyError::UnknownLink(link))?;
            (
                physical.from,
                physical.to,
                physical.impulse != impulse,
                memory.is_entry() || memory.is_drive(),
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

    /// Allows a propagation link to learn from a returned consequence in its
    /// recent local backward cone. Exact path learning does not require this.
    pub fn mark_locally_plastic(&mut self, link: LinkId) -> Result<(), ApplyError> {
        let memory = self
            .arrows
            .get_mut(link.slot())
            .ok_or(ApplyError::UnknownLink(link))?;
        if memory.mark_locally_plastic() {
            Ok(())
        } else {
            Err(ApplyError::InvalidLinkRole(link))
        }
    }

    /// Marks an ordinary transmitting link as an outward body/world crossing.
    /// The link still drives normally, but internal automaticity may not erase
    /// it or compose across it.
    pub(crate) fn mark_boundary_drive(&mut self, link: LinkId) -> Result<(), ApplyError> {
        let memory = self
            .arrows
            .get_mut(link.slot())
            .ok_or(ApplyError::UnknownLink(link))?;
        memory.mark_boundary_crossing();
        Ok(())
    }

    pub(crate) fn replace_arrow_state(
        &mut self,
        link: LinkId,
        state: ArrowState,
    ) -> Result<(), ApplyError> {
        let previous = *self
            .arrows
            .get(link.slot())
            .ok_or(ApplyError::UnknownLink(link))?;
        let physical = *self.arena.link(link).ok_or(ApplyError::UnknownLink(link))?;
        if previous.open_return_data().is_some() {
            self.remove_live_return(link);
        }
        self.arrows[link.slot()] = state;
        let transient_return = state.open_return_data().is_some()
            && physical.delay == 0
            && physical.impulse == 0;
        if previous != state
            && !transient_return
            && (touches_reentry(&previous) || touches_reentry(&state))
        {
            self.touch_reentry_junctions([physical.from, physical.to]);
        }
        if previous != state && (previous.is_membership() || state.is_membership()) {
            self.touch_path_entries_from(physical.to);
        }
        if previous.factors().is_none() && state.factors().is_some() {
            let consolidation = self.consolidation_mut();
            consolidation.work.composites_formed =
                consolidation.work.composites_formed.saturating_add(1);
            if automatic_leftmost(self, link, 0)
                .is_some_and(|root| self.arrows[root.slot()].is_drive())
            {
                self.has_composites = true;
            }
        }
        if state.open_return_data().is_some() {
            self.insert_live_return(link);
        }
        if self.returns.live_count != 0
            && previous != state
            && (touches_reentry(&previous) || touches_reentry(&state))
        {
            self.rebuild_live_returns();
        }
        if previous.open_return_data().is_none() || state.open_return_data().is_none() {
            self.prune_reentry();
        }
        Ok(())
    }

    pub fn mark_path_entry(&mut self, link: LinkId) -> Result<(), ApplyError> {
        self.replace_arrow_state(link, ArrowState::entry())
    }

    pub(crate) fn mark_witness(
        &mut self,
        link: LinkId,
        kind: WitnessKind,
    ) -> Result<(), ApplyError> {
        self.replace_arrow_state(link, ArrowState::witness(kind))
    }

    #[cfg(test)]
    pub(crate) fn apply(&mut self, mut change: Change) -> Result<Applied, ApplyError> {
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
                    self.replace_arrow_state(id, spec.state)?;
                    if spec.state.is_drive() && spec.state.factors().is_none() {
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
                    let was_open = self
                        .arrows
                        .get(returned.slot())
                        .ok_or(ApplyError::UnknownLink(returned))?
                        .open_return_data()
                        .is_some();
                    if self.needs_automatic_closure() {
                        self.complete_automatic_witness(returned, path, exact);
                    }
                    if was_open {
                        if exclusive_source {
                            self.remove_exclusive_live_return(source, returned);
                        } else {
                            self.remove_live_return_with_path(returned, Some(path));
                        }
                    }
                    let mut exact_closures = self.arrows[path.first.slot()].exact_closures();
                    for (index, link) in path.links().into_iter().enumerate() {
                        let memory = self
                            .arrows
                            .get_mut(link.slot())
                            .ok_or(ApplyError::UnknownLink(link))?;
                        let (closures, before, after) = memory
                            .learn_closure(at, offers_choice, exact && index == 0)
                            .unwrap_or((0, 1, 1));
                        if index == 0 {
                            exact_closures = closures;
                        }
                        if T::ENABLED {
                            trace.record(TraceEvent::Strengthened(StrengthTrace {
                                at,
                                link,
                                before,
                                after,
                            }));
                        }
                    }
                    if let Some(support) = self.retained_closed_support(
                        returned,
                        source,
                        path,
                        outcome_witness,
                        exact,
                        exclusive_source && exact_closures > u8::from(exact),
                    ) {
                        self.arrows[returned.slot()].close_return(at, support, motif_parent);
                    }
                }
                Edit::ChangeLink { link, change } => {
                    let link = resolve_link(link, applied)?;
                    if let LinkChange::MarkAmbiguous { at } = change {
                        let was_open = self
                            .arrows
                            .get(link.slot())
                            .ok_or(ApplyError::UnknownLink(link))?
                            .open_return_data()
                            .is_some();
                        if was_open {
                            self.remove_live_return(link);
                            self.arrows[link.slot()].mark_ambiguous(at);
                        }
                        continue;
                    }
                    if change == LinkChange::Retire {
                        let memory = self
                            .arrows
                            .get(link.slot())
                            .ok_or(ApplyError::UnknownLink(link))?;
                        let was_active = memory.active();
                        let was_open = memory.open_return_data().is_some();
                        let touched_reentry = touches_reentry(memory);
                        let was_membership = memory.is_membership();
                        let physical =
                            *self.arena.link(link).ok_or(ApplyError::UnknownLink(link))?;
                        if was_active {
                            if was_open {
                                self.remove_live_return(link);
                                self.arrows[link.slot()].expire_return();
                            } else {
                                self.arrows[link.slot()].deactivate();
                                self.arrows[link.slot()].clear_outcome();
                            }
                            if self.returns.live_count != 0 && touched_reentry {
                                self.rebuild_live_returns();
                            }
                            if touched_reentry {
                                self.touch_reentry_junctions([physical.from, physical.to]);
                            }
                            if was_membership {
                                self.touch_path_entries_from(physical.to);
                            }
                        }
                        continue;
                    }
                    let physical = *self.arena.link(link).ok_or(ApplyError::UnknownLink(link))?;
                    let mut touch_reentry = false;
                    let memory = self
                        .arrows
                        .get_mut(link.slot())
                        .ok_or(ApplyError::UnknownLink(link))?;
                    match change {
                        LinkChange::Participated { cause, at } => {
                            touch_reentry = memory.participate(Occurrence { cause, at });
                        }
                        LinkChange::RememberOutcome {
                            at,
                            available_until_choice,
                        } => {
                            memory.remember_outcome(Outcome {
                                at,
                                caused_transition: false,
                                available_until_choice,
                            });
                        }
                        LinkChange::ConsumeOutcome => {
                            memory.consume_outcome();
                        }
                        LinkChange::ClearOutcomeSelection => {
                            memory.clear_outcome();
                        }
                        LinkChange::InhibitBoundaryChoice => {
                            memory.inhibit_boundary();
                        }
                        LinkChange::ConsumeBoundaryInhibition => {
                            memory.consume_boundary_inhibition();
                        }
                        LinkChange::Strengthen { amount } => {
                            let (before, after) = memory
                                .strengthen(i64::from(amount))
                                .unwrap_or((1, 1));
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
                        LinkChange::MarkAmbiguous { .. } => {
                            unreachable!("ambiguity handled before ordinary mutation")
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
