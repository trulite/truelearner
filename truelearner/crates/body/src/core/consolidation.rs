const MAX_AUTOMATIC_COMPOSITE_DEPTH: usize = 32;

impl Body {
    pub fn reentry_state(&self) -> ReentryState {
        ReentryState {
            closed_steps: self
                .arrows
                .iter()
                .filter(|memory| memory.closed_support().is_some())
                .count(),
            thought_shortcuts: self.reentry.as_ref().map_or(0, |reentry| {
                reentry
                    .shortcuts
                    .iter()
                    .filter(|shortcut| {
                        shortcut.rehearsals >= THOUGHT_SHORTCUT_AFTER_REHEARSALS
                            && reentry.shortcut_is_current(shortcut)
                    })
                    .count()
            }),
        }
    }

    pub(crate) fn touch_reentry_junctions(
        &mut self,
        junctions: impl IntoIterator<Item = JunctionId>,
    ) {
        let Some(reentry) = &mut self.reentry else {
            return;
        };
        for junction in junctions {
            reentry.touch_reentry(junction);
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
            if self.arrows[link.slot()].is_entry() {
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
        let reentry = self.reentry_mut();
        let captured = dependencies
            .into_iter()
            .map(|junction| ReentryDependency {
                junction,
                epoch: reentry.reentry_epoch(junction),
            })
            .collect::<Vec<_>>();
        match reentry
            .shortcuts
            .binary_search_by_key(&(start, condition), |shortcut| {
                (shortcut.start, shortcut.condition)
            }) {
            Ok(index) => {
                let current = reentry.shortcut_is_current(&reentry.shortcuts[index]);
                let shortcut = &mut reentry.shortcuts[index];
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
            Err(index) => reentry.shortcuts.insert(
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
        let view = ReactionView::new(&self.arena, &self.arrows, &self.returns);
        let invalid = closed_steps(view)
            .filter(|step| step.path == path)
            .map(|step| step.link)
            .collect::<Vec<_>>();
        for link in invalid {
            self.arrows[link.slot()].deactivate();
        }
        if touch_epoch {
            self.touch_reentry_junctions([path.surface, path.middle, path.output]);
        }
    }

    #[inline(always)]
    fn retained_closed_support(
        &mut self,
        returned: LinkId,
        source: JunctionId,
        path: Path,
        outcome_witness: Option<LinkId>,
        replaces_existing: bool,
    ) -> Option<ClosedSupport> {
        let Some(outcome_witness) = outcome_witness else {
            if replaces_existing {
                self.invalidate_closed_step(path);
            }
            self.arrows[returned.slot()].expire_return();
            return None;
        };
        let support = ClosedSupport {
            source,
            witness: outcome_witness,
        };
        let same_existing_support = if replaces_existing {
            let view = ReactionView::new(&self.arena, &self.arrows, &self.returns);
            let mut matching = closed_steps(view)
                .filter(|step| step.path == path)
                .map(|step| ClosedSupport {
                    source: step.returned_source,
                    witness: step.outcome_witness,
                });
            matching.next() == Some(support) && matching.all(|existing| existing == support)
        } else {
            false
        };
        if replaces_existing {
            self.invalidate_closed_step_with_epoch(path, !same_existing_support);
        }
        if !same_existing_support && self.reentry.is_some() {
            let witness = self.arena.link(outcome_witness).copied();
            self.touch_reentry_junctions(
                [path.surface, path.middle, path.output, source]
                    .into_iter()
                    .chain(witness.into_iter().flat_map(|link| [link.from, link.to])),
            );
        }
        Some(support)
    }

    fn prune_reentry(&mut self) {
        let view = ReactionView::new(&self.arena, &self.arrows, &self.returns);
        let invalid = closed_steps(view)
            .filter(|step| closed_step_is_valid(view, *step).is_none())
            .map(|step| (step.link, step.path))
            .collect::<Vec<_>>();
        for (link, path) in invalid {
            self.arrows[link.slot()].deactivate();
            self.touch_reentry_junctions([path.surface, path.middle, path.output]);
        }
    }

    fn consolidation_mut(&mut self) -> &mut Consolidation {
        self.consolidation
            .get_or_insert_with(|| Box::new(Consolidation::default()))
    }

    fn reentry_mut(&mut self) -> &mut ReentryCache {
        self.reentry
            .get_or_insert_with(|| Box::new(ReentryCache::default()))
    }

    pub fn automaticity_work(&self) -> AutomaticityWork {
        self.consolidation
            .as_ref()
            .map_or(AutomaticityWork::default(), |automaticity| {
                automaticity.work
            })
    }

    pub fn automaticity_state(&self) -> AutomaticityState {
        self.consolidation
            .as_ref()
            .map_or(AutomaticityState::default(), |automaticity| {
                AutomaticityState {
                    open_witnesses: automaticity.witnesses.len(),
                    candidate_pairs: automaticity.evidence.len(),
                    has_recursive_composites: self.has_composites,
                }
            })
    }

    #[inline(always)]
    fn needs_automatic_closure(&self) -> bool {
        self.consolidation
            .as_ref()
            .is_some_and(|automaticity| automaticity.closure_maintenance)
    }

    fn refresh_automatic_closure(&mut self) {
        if let Some(automaticity) = &mut self.consolidation {
            automaticity.closure_maintenance =
                !automaticity.witnesses.is_empty() || !automaticity.evidence.is_empty();
        }
    }

    pub(crate) fn observe_automatic_pair(&mut self, first: LinkId, second: LinkId, at: Time) {
        if first == second || !automatic_segment_role(self, first) {
            return;
        }
        if !automatic_segment_role(self, second) {
            return;
        }
        let pair = AutomaticPair { first, second };
        let Some((returned, path)) = self.automatic_witness_for_pair(pair, at) else {
            return;
        };
        let index = self
            .consolidation
            .as_ref()
            .and_then(|automaticity| {
                automaticity
                    .witnesses
                    .iter()
                    .position(|witness| witness.returned == returned)
            })
            .unwrap_or_else(|| {
                let automaticity = self.consolidation_mut();
                let index = automaticity.witnesses.len();
                automaticity.witnesses.push(AutomaticWitness {
                    returned,
                    path,
                    pairs: Vec::new(),
                });
                automaticity.closure_maintenance = true;
                index
            });
        let automaticity = self.consolidation_mut();
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
        at: Time,
    ) -> Option<(LinkId, Path)> {
        if let Some(automaticity) = &self.consolidation {
            let mut continuing = automaticity.witnesses.iter().filter(|witness| {
                witness.pairs.contains(&pair)
                    || witness
                        .pairs
                        .last()
                        .is_some_and(|previous| previous.second == pair.first)
            });
            if let Some(witness) = continuing.next() {
                if continuing.next().is_none() {
                    return Some((witness.returned, witness.path));
                }
                return None;
            }
        }

        let root = self.arena.link(pair.first)?.from;
        let view = ReactionView::new(&self.arena, &self.arrows, &self.returns);
        let mut roots = self.returns.by_source.iter().flatten().filter_map(|entry| {
            let returned = open_return(view, *entry)?;
            self.automatic_root_descends_from(returned.path.output, root, at)
            .then_some((returned.link, returned.path))
        });
        let root = roots.next()?;
        roots.next().is_none().then_some(root)
    }

    fn automatic_root_descends_from(
        &self,
        output: JunctionId,
        root: JunctionId,
        at: Time,
    ) -> bool {
        output == root
            || self.arena.incoming(root).any(|link| {
                let physical = self.arena.link(link).expect("live boundary incidence");
                let memory = &self.arrows[link.slot()];
                physical.from == output
                    && memory.active()
                    && memory.boundary_crossing()
                    && memory
                        .last_transmission()
                        .is_some_and(|occurrence| {
                            at.saturating_sub(occurrence.at) <= LOCAL_PLASTICITY_WINDOW
                        })
            })
    }

    fn expire_automatic_witness(&mut self, returned: LinkId) {
        let Some(automaticity) = &mut self.consolidation else {
            return;
        };
        automaticity
            .witnesses
            .retain(|witness| witness.returned != returned);
        self.refresh_automatic_closure();
    }

    fn complete_automatic_witness(&mut self, returned: LinkId, path: Path) {
        if self.consolidation.is_none() {
            return;
        }
        if self
                .consolidation
                .as_ref()
                .is_some_and(|automaticity| !automaticity.evidence.is_empty())
        {
            let evidence = std::mem::take(
                &mut self
                    .consolidation
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
            self.consolidation
                .as_mut()
                .expect("checked automaticity")
                .evidence = retained;
        }
        let Some(index) = self
            .consolidation
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
            .consolidation
            .as_mut()
            .expect("checked automaticity")
            .witnesses
            .remove(index);
        let mut ready = Vec::new();
        for pair in witness.pairs {
            if !automatic_pair_is_valid(self, pair) {
                continue;
            }
            let automaticity = self.consolidation_mut();
            automaticity.work.supported_closure_updates = automaticity
                .work
                .supported_closure_updates
                .saturating_add(1);
            if automatic_composite_with_parents(self, pair).is_some() {
                continue;
            }
            if let Some(evidence) = self
                .consolidation_mut()
                .evidence
                .iter_mut()
                .find(|evidence| evidence.owner == witness.path.first && evidence.pair == pair)
            {
                evidence.supported_closures = evidence.supported_closures.saturating_add(1);
                if evidence.supported_closures >= AUTOMATIC_AFTER_SUPPORTED_CLOSURES {
                    ready.push(pair);
                }
            } else {
                self.consolidation_mut().evidence.push(AutomaticEvidence {
                    owner: witness.path.first,
                    pair,
                    supported_closures: 1,
                });
            }
        }
        for pair in ready {
            if self.retain_automatic_pair(pair) {
                self.consolidation_mut().evidence.retain(|evidence| {
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
        self.arrows[composite.slot()].retain_factors([pair.first, pair.second]);
        let parent_strength = self.arrows[pair.second.slot()].strength();
        self.arrows[composite.slot()].strengthen(parent_strength.saturating_sub(1));
        self.consolidation_mut().work.composites_formed = self
            .consolidation_mut()
            .work
            .composites_formed
            .saturating_add(1);
        if automatic_leftmost(self, composite, 0)
            .is_some_and(|root| self.arrows[root.slot()].factors().is_none())
        {
            self.has_composites = true;
        }
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
                || self.arrows[candidate.slot()].factors().is_none()
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
    body.arrows.get(link.slot()).is_some_and(|memory| {
        memory.is_drive() && !memory.boundary_crossing()
    })
}

fn automatic_leftmost(body: &Body, link: LinkId, depth: usize) -> Option<LinkId> {
    if depth >= MAX_AUTOMATIC_COMPOSITE_DEPTH {
        return None;
    }
    let memory = body.arrows.get(link.slot())?;
    match memory.factors() {
        None if memory.is_drive() => Some(link),
        Some([first, _]) => automatic_leftmost(body, first, depth + 1),
        _ => None,
    }
}

fn automatic_rightmost(body: &Body, link: LinkId, depth: usize) -> Option<LinkId> {
    if depth >= MAX_AUTOMATIC_COMPOSITE_DEPTH {
        return None;
    }
    let memory = body.arrows.get(link.slot())?;
    match memory.factors() {
        None if memory.is_drive() => Some(link),
        Some([_, second]) => automatic_rightmost(body, second, depth + 1),
        _ => None,
    }
}

fn automatic_leaf_count(body: &Body, link: LinkId, depth: usize) -> Option<usize> {
    if depth >= MAX_AUTOMATIC_COMPOSITE_DEPTH {
        return None;
    }
    let memory = body.arrows.get(link.slot())?;
    match memory.factors() {
        None if memory.is_drive() => Some(1),
        Some([first, second]) => automatic_leaf_count(body, first, depth + 1)?
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
    match body.arrows[link.slot()].factors() {
        None => physical.impulse != 0,
        Some([first, second]) => {
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
                && body.arrows[link.slot()].strength()
                    == body.arrows[second.slot()].strength()
        }
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
        i64::from(first.impulse).saturating_mul(body.arrows[pair.first.slot()].strength());
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
        body.arrows[link.slot()].active()
            && body.arrows[link.slot()].factors().is_none()
            && link != incoming
    }) {
        return false;
    }
    let mut next = junction.outgoing_head;
    while let Some(link) = next {
        let physical = body.arena.link(link).expect("live automatic incidence");
        next = physical.next;
        if body.arrows[link.slot()].active()
            && body.arrows[link.slot()].factors().is_none()
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
        if body.arrows[link.slot()].active()
            && body.arrows[link.slot()].factors() == Some([pair.first, pair.second])
        {
            return Some(link);
        }
    }
    None
}
