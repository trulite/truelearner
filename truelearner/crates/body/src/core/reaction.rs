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
    moment.boundary_arrivals().any(|(_, target, _)| {
        !moment
            .changes
            .iter()
            .any(|change| change.event.junction == target)
            && is_motor_gate(body, target)
    })
}

fn is_motor_gate(body: ReactionView<'_>, junction: JunctionId) -> bool {
    let mut next = body
        .arena
        .junction(junction)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link) = next {
        let physical = body.arena.link(link).expect("live motor incidence");
        if body.arrows[link.slot()].is_drive()
            && body.arrows[link.slot()].boundary_crossing()
        {
            return true;
        }
        next = physical.next;
    }
    false
}

fn boundary_can_react(body: ReactionView<'_>, surface: JunctionId) -> bool {
    body.returns.live_count != 0 || surface_may_choose(body, surface)
}

fn is_outcome_source(body: ReactionView<'_>, junction: JunctionId) -> bool {
    let mut next = body
        .arena
        .junction(junction)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link_id) = next {
        let link = body.arena.link(link_id).expect("live link");
        let memory = &body.arrows[link_id.slot()];
        if closes_return(memory) {
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
        if body.arrows[link.slot()].witness_kind() == Some(WitnessKind::Progress) {
            return true;
        }
    }
    false
}

const fn closes_return(state: &ArrowState) -> bool {
    matches!(state.witness_kind(), Some(WitnessKind::Closure { .. }))
}

fn is_membership_link(body: ReactionView<'_>, link: LinkId) -> bool {
    body.arrows[link.slot()].is_membership()
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
        moment,
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
    record_used_outputs(body, &scratch.facts, &mut scratch.change);
    construct_caused_reentry_memberships(
        body,
        &scratch.facts,
        &scratch.ready,
        &scratch.winners,
        &mut scratch.construction,
        &mut scratch.change,
    );
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
        let memory = &body.arrows[link_id.slot()];
        if memory.is_entry()
            || (memory.is_drive()
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
    let boundary = change.add_junction(Junction::integrating(1));
    if let Some(parent) = closure.parent {
        change.add_link(
            boundary.into(),
            parent.into(),
            LinkSpec {
                delay: 0,
                impulse: 1,
                trigger: Trigger::SourceFires,
                state: ArrowState::membership(),
            },
        );
    }
    for member in members {
        if parent_members.contains(member) {
            continue;
        }
        change.add_link(
            boundary.into(),
            (*member).into(),
            LinkSpec {
                delay: 0,
                impulse: 1,
                trigger: Trigger::SourceFires,
                state: ArrowState::membership(),
            },
        );
    }
}

fn construct_caused_reentry_memberships(
    body: ReactionView<'_>,
    facts: &[MomentFact],
    ready: &[CandidatePath],
    winners: &[ReadyChoice],
    scratch: &mut ConstructionScratch,
    change: &mut Change,
) {
    for choice in winners
        .iter()
        .filter(|choice| choice.basis == ChoiceBasis::UniqueReentry)
    {
        let candidate = &ready[choice.winner];
        let [reentry] = candidate.continuation.reentries.as_slice() else {
            continue;
        };
        let mut returning = facts.iter().filter(|fact| {
            fact.boundary
                && matches!(fact.used, UsedPaths::None)
                && fact.event.junction == reentry.condition
        });
        let Some(fact) = returning.next() else {
            continue;
        };
        if returning.next().is_some() {
            continue;
        }
        let selected = scan_live_returns(body, reentry.condition, fact.event.cause);
        let Some(returned) = selected.selected.filter(|returned| {
            selected.exact_total == 1
                && returned.cause == fact.event.cause
                && returned.opened_at <= fact.event.at
        }) else {
            continue;
        };
        let (JunctionRef::Existing(middle), LinkRef::Existing(first), LinkRef::Existing(second)) =
            (candidate.middle, candidate.first, candidate.second)
        else {
            continue;
        };
        let candidate_path = Path {
            surface: candidate.surface,
            middle,
            output: candidate.output,
            first,
            second,
        };
        if reentry.steps.first().map(|step| step.path) != Some(candidate_path)
            || returned.path.surface == candidate.surface
        {
            continue;
        }

        scratch.clear();
        scratch
            .members
            .extend([candidate.surface, returned.path.surface]);
        scratch.members.sort_unstable();
        scratch.members.dedup();
        if scratch.members.len() != 2 {
            continue;
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
            MembershipParent::Ambiguous => continue,
        };
        construct_membership(
            DetectedClosure {
                at: fact.event.at,
                parent,
            },
            &scratch.members,
            &scratch.parent_members,
            change,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn remember_construction_outcomes(
    at: Time,
    members: &[JunctionId],
    parent_members: &[JunctionId],
    consequences: &[JunctionId],
    ready: &[CandidatePath],
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
            change.change_link(
                link,
                LinkChange::RememberOutcome {
                    at,
                    available_until_choice: true,
                },
            );
        }
    }
}

fn record_used_outputs(body: ReactionView<'_>, facts: &[MomentFact], change: &mut Change) {
    for fact in facts {
        let UsedPaths::One(path) = fact.used else {
            continue;
        };
        for link in path.links() {
            change.change_link(
                link.into(),
                LinkChange::Participated {
                    cause: fact.event.cause,
                    at: fact.event.at,
                },
            );
        }
        let returned = change.add_link(
            path.output.into(),
            path.middle.into(),
            LinkSpec {
                delay: 0,
                impulse: 0,
                trigger: Trigger::SourceFires,
                state: ArrowState::open_return(path, fact.event.cause, fact.event.at),
            },
        );
        change.change_link(
            returned.into(),
            LinkChange::Participated {
                cause: fact.event.cause,
                at: fact.event.at,
            },
        );
        if let Some(prior) = unique_prior_unclosed_sibling_before(body, path, fact.event.at) {
            change.change_link(
                returned.into(),
                LinkChange::RememberSwitchedFrom {
                    prior: prior.second,
                },
            );
        }
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
                decision: ReturnDecision::BlockedByCandidatePath,
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
    let motif_parent = accepted
        .filter(|returned| returned.cause == fact.event.cause)
        .filter(|returned| body.arrows[returned.link.slot()].switched_from().is_some())
        .and_then(|returned| matching_return_motif(body, returned, fact.event.junction))
        .map(|closed| closed.link);
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
            change.complete_return(
                fact.event.junction,
                entry.link,
                entry.path,
                entry.outcome_witness,
                motif_parent,
                returned.cause == fact.event.cause,
                entry.exclusive_source,
                entry.offers_choice,
                fact.event.at,
            );
            retain_composite_after_return(
                body,
                entry.path,
                returned.cause == fact.event.cause,
                change,
            );
        } else {
            change.change_link(
                entry.link.into(),
                if decision == ReturnDecision::Ambiguous {
                    LinkChange::MarkAmbiguous { at: fact.event.at }
                } else {
                    LinkChange::Retire
                },
            );
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
        change.change_link(composite.into(), LinkChange::Strengthen { amount: 1 });
        return;
    }
    let exact_closures = body.arrows[path.first.slot()].exact_closures();
    if !exact
        || exact_closures.saturating_add(1) < AUTOMATIC_AFTER_EXACT_CLOSURES
        || !path_can_be_composed(body, path)
    {
        return;
    }
    let first = body.arena.link(path.first).expect("validated path entry");
    let second = body.arena.link(path.second).expect("validated path drive");
    let composite = change.add_link(
        path.surface.into(),
        path.output.into(),
        LinkSpec {
            delay: first.delay + second.delay,
            impulse: second.impulse,
            trigger: first.trigger,
            state: {
                let mut state = ArrowState::drive();
                let retained = state.retain_factors([path.first, path.second]);
                debug_assert!(retained);
                state
            },
        },
    );
    let parent_strength = body.arrows[path.second.slot()].strength();
    let amount = i32::try_from(parent_strength).unwrap_or(i32::MAX);
    change.change_link(composite.into(), LinkChange::Strengthen { amount });
}
