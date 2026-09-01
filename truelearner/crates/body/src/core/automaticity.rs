const MAX_AUTOMATIC_COMPOSITE_DEPTH: usize = 32;

impl Body {
    pub fn reentry_state(&self) -> ReentryState {
        ReentryState {
            closed_steps: self
                .link_memory
                .iter()
                .filter(|memory| memory.closed_support().is_some())
                .count(),
            thought_shortcuts: self.automaticity.as_ref().map_or(0, |automaticity| {
                automaticity
                    .thought_shortcuts
                    .iter()
                    .filter(|shortcut| {
                        shortcut.rehearsals >= THOUGHT_SHORTCUT_AFTER_REHEARSALS
                            && automaticity.shortcut_is_current(shortcut)
                    })
                    .count()
            }),
        }
    }

    pub(crate) fn touch_reentry_junctions(
        &mut self,
        junctions: impl IntoIterator<Item = JunctionId>,
    ) {
        let Some(automaticity) = &mut self.automaticity else {
            return;
        };
        for junction in junctions {
            automaticity.touch_reentry(junction);
        }
    }

    fn touch_path_entries_from(&mut self, surface: JunctionId) {
        let mut middles = Vec::new();
        let mut next = self
            .arena
            .junction(surface)
            .and_then(|junction| junction.outgoing_head);
        while let Some(link) = next {
            let physical = self.arena.link(link).expect("live path incidence");
            next = physical.next;
            if self.link_memory[link.slot()].live
                && self.link_memory[link.slot()].role == LinkRole::PathEntry
            {
                middles.push(physical.to);
            }
        }
        self.touch_reentry_junctions(middles);
    }

    fn rehearse_reentry(
        &mut self,
        start: Path,
        condition: JunctionId,
        routes: Vec<ReentryTrace>,
        mut dependencies: Vec<JunctionId>,
    ) {
        dependencies.sort_unstable();
        dependencies.dedup();
        let automaticity = self.automaticity_mut();
        let captured = dependencies
            .into_iter()
            .map(|junction| ReentryDependency {
                junction,
                epoch: automaticity.reentry_epoch(junction),
            })
            .collect::<Vec<_>>();
        match automaticity
            .thought_shortcuts
            .binary_search_by_key(&(start, condition), |shortcut| {
                (shortcut.start, shortcut.condition)
            }) {
            Ok(index) => {
                let current =
                    automaticity.shortcut_is_current(&automaticity.thought_shortcuts[index]);
                let shortcut = &mut automaticity.thought_shortcuts[index];
                let same_dependencies = shortcut
                    .dependencies
                    .iter()
                    .map(|dependency| dependency.junction)
                    .eq(captured.iter().map(|dependency| dependency.junction));
                if current && shortcut.routes == routes && same_dependencies {
                    shortcut.rehearsals = shortcut.rehearsals.saturating_add(1);
                } else {
                    shortcut.routes = routes;
                    shortcut.dependencies = captured;
                    shortcut.rehearsals = 1;
                }
            }
            Err(index) => automaticity.thought_shortcuts.insert(
                index,
                ThoughtShortcut {
                    start,
                    condition,
                    routes,
                    dependencies: captured,
                    rehearsals: 1,
                },
            ),
        }
    }

    fn invalidate_closed_step(&mut self, path: Path) {
        self.invalidate_closed_step_with_epoch(path, true);
    }

    fn invalidate_closed_step_with_epoch(&mut self, path: Path, touch_epoch: bool) {
        let view = ReactionView::new(&self.arena, &self.link_memory, &self.returns);
        let invalid = closed_steps(view)
            .filter(|step| step.path == path)
            .map(|step| step.link)
            .collect::<Vec<_>>();
        for link in invalid {
            self.link_memory[link.slot()].outcome_available = false;
        }
        if touch_epoch {
            self.touch_reentry_junctions([path.surface, path.middle, path.output]);
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
        let same_existing_support = if replaces_existing {
            let view = ReactionView::new(&self.arena, &self.link_memory, &self.returns);
            let mut matching = closed_steps(view)
                .filter(|step| step.path == path)
                .map(|step| (step.returned_source, step.outcome_witness));
            matching.next() == Some(support) && matching.all(|existing| existing == support)
        } else {
            false
        };
        if replaces_existing {
            self.invalidate_closed_step_with_epoch(path, !same_existing_support);
        }
        if self.link_memory[returned.slot()].stored_support() != Some(support) {
            self.link_memory[returned.slot()].remember_closed_support(source, outcome_witness);
            if !same_existing_support {
                let witness = self.arena.link(outcome_witness).copied();
                self.touch_reentry_junctions(
                    [path.surface, path.middle, path.output, source]
                        .into_iter()
                        .chain(witness.into_iter().flat_map(|link| [link.from, link.to])),
                );
            }
        }
    }

    fn prune_reentry(&mut self) {
        let view = ReactionView::new(&self.arena, &self.link_memory, &self.returns);
        let invalid = closed_steps(view)
            .filter(|step| closed_step_is_valid(view, *step).is_none())
            .map(|step| (step.link, step.path))
            .collect::<Vec<_>>();
        for (link, path) in invalid {
            self.link_memory[link.slot()].outcome_available = false;
            self.touch_reentry_junctions([path.surface, path.middle, path.output]);
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
