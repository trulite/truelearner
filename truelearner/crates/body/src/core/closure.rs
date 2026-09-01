
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
    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    if surface_may_choose(view, source) {
        return false;
    }
    let Some(returned) = open_return(view, entry) else {
        return false;
    };
    if body.arrows[returned.link.slot()].switched_from().is_some() {
        return false;
    }
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
    let mut exact_closures = body.arrows[returned.path.first.slot()].exact_closures();
    for (index, link) in returned.path.links().into_iter().enumerate() {
        let memory = &mut body.arrows[link.slot()];
        let (closures, before, after) = memory
            .learn_closure(event.at, returned.offers_choice, exact && index == 0)
            .unwrap_or((0, 1, 1));
        if index == 0 {
            exact_closures = closures;
        }
        if T::ENABLED {
            trace.record(TraceEvent::Strengthened(StrengthTrace {
                at: event.at,
                link,
                before,
                after,
            }));
        }
    }
    if let Some(support) = body.retained_closed_support(
        returned.link,
        source,
        returned.path,
        entry.outcome_witness,
        exact,
        exact_closures > u8::from(exact),
    ) {
        body.arrows[returned.link.slot()].close_return(event.at, support, None);
    }
    if exact_closures >= AUTOMATIC_AFTER_EXACT_CLOSURES {
        retain_composite_direct(body, returned.path, event.at, trace);
    }
    true
}

fn retain_composite_direct<T: TraceSink>(body: &mut Body, path: Path, at: Time, trace: &mut T) {
    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    if let Some(composite) = composite_with_parents(view, path) {
        let memory = &mut body.arrows[composite.slot()];
        let (before, after) = memory.strengthen(1).unwrap_or((1, 1));
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
    let parent_strength = body.arrows[path.second.slot()].strength();
    let Ok(composite) = body.add_link(
        Link::new(
            path.surface,
            path.output,
            first.delay + second.delay,
            second.impulse,
        )
        .when(first.trigger),
    ) else {
        return;
    };
    body.arrows[composite.slot()].retain_factors([path.first, path.second]);
    body.consolidation_mut().work.composites_formed = body
        .consolidation_mut()
        .work
        .composites_formed
        .saturating_add(1);
    let memory = &mut body.arrows[composite.slot()];
    let before = memory.strength();
    memory.strengthen(parent_strength.saturating_sub(before));
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
    let (arena, arrows) = (&body.arena, &mut body.arrows);
    for second in arena.incoming(output) {
        let physical = arena.link(second).expect("live output incidence");
        let memory = &mut arrows[second.slot()];
        if !memory.is_drive() || memory.factors().is_some() || physical.impulse == 0 {
            continue;
        }
        memory.clear_outcome();
        memory.inhibit_boundary();
        for first in arena.incoming(physical.from) {
            let memory = &mut arrows[first.slot()];
            if memory.is_entry() {
                memory.clear_outcome();
                memory.inhibit_boundary();
            }
        }
    }
}

fn clear_output_selection(body: ReactionView<'_>, output: JunctionId, change: &mut Change) {
    for second in body.arena.incoming(output) {
        let physical = body.arena.link(second).expect("live output incidence");
        let memory = &body.arrows[second.slot()];
        if !memory.is_drive() || memory.factors().is_some() || physical.impulse == 0 {
            continue;
        }
        change.change_link(second.into(), LinkChange::ClearOutcomeSelection);
        change.change_link(second.into(), LinkChange::InhibitBoundaryChoice);
        for first in body.arena.incoming(physical.from) {
            if body.arrows[first.slot()].is_entry() {
                change.change_link(first.into(), LinkChange::ClearOutcomeSelection);
                change.change_link(first.into(), LinkChange::InhibitBoundaryChoice);
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
    let memory = body.arrows.get(entry.link.slot())?;
    let (path, cause, opened_at, _) = memory.open_return_data()?;
    debug_assert_eq!(cause, entry.cause);
    debug_assert_eq!(path, entry.path);
    Some(OpenReturn {
        link: entry.link,
        path: entry.path,
        cause,
        opened_at,
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
        if body.arrows[link.slot()].is_drive()
            && body.arrows[link.slot()].factors().is_none()
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
    let memory = body.arrows.get(second.slot())?;
    if !memory.is_drive() {
        return None;
    }
    if let Some([first, parent_second]) = memory.factors() {
        let path = path_from_links(body, first, parent_second)?;
        return composite_is_valid(body, second, path).then_some(path);
    }
    if drive.impulse == 0 {
        return None;
    }
    let first = body.arena.incoming(drive.from).find(|first| {
        body.arrows[first.slot()].is_entry()
    })?;
    path_from_links(body, first, second)
}

fn path_from_links(body: ReactionView<'_>, first: LinkId, second: LinkId) -> Option<Path> {
    let entry = body.arena.link(first)?;
    let drive = body.arena.link(second)?;
    if !body.arrows.get(first.slot())?.is_entry()
        || !body.arrows.get(second.slot())?.is_drive()
        || body.arrows[second.slot()].factors().is_some()
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
        if body.arrows[link.slot()].active()
            && body.arrows[link.slot()].factors() == Some([path.first, path.second])
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
            && body.arrows[link.slot()].is_drive()
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
        && matches!(
            first.trigger,
            Trigger::SourceFires | Trigger::Rises | Trigger::Falls
        )
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
    body.arrows
        .get(composite.slot())
        .is_some_and(|memory| {
            memory.active() && memory.factors() == Some([path.first, path.second])
        })
        && link.from == path.surface
        && link.to == path.output
        && link.delay == first.delay.saturating_add(second.delay)
        && link.impulse == second.impulse
        && link.trigger == first.trigger
}

fn usable_composite(body: ReactionView<'_>, path: Path) -> Option<LinkId> {
    composite_with_parents(body, path).filter(|link| composite_is_valid(body, *link, path))
}

fn usable_composite_for_reentry(
    body: ReactionView<'_>,
    path: Path,
    incidence_visits: &mut u16,
) -> Result<bool, ()> {
    let mut next = body
        .arena
        .junction(path.surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(composite) = next {
        visit_reentry_incidence(incidence_visits)?;
        let physical = body.arena.link(composite).expect("live shortcut incidence");
        next = physical.next;
        if body.arrows[composite.slot()].active()
            && body.arrows[composite.slot()].factors() == Some([path.first, path.second])
        {
            return composite_is_valid_for_reentry(body, composite, path, incidence_visits);
        }
    }
    Ok(false)
}

fn composite_is_valid_for_reentry(
    body: ReactionView<'_>,
    composite: LinkId,
    path: Path,
    incidence_visits: &mut u16,
) -> Result<bool, ()> {
    let Some(first) = body.arena.link(path.first) else {
        return Ok(false);
    };
    let Some(second) = body.arena.link(path.second) else {
        return Ok(false);
    };
    if path_from_links(body, path.first, path.second) != Some(path)
        || !matches!(
            first.trigger,
            Trigger::SourceFires | Trigger::Rises | Trigger::Falls
        )
        || second.trigger != Trigger::SourceFires
        || first.delay.checked_add(second.delay).is_none()
    {
        return Ok(false);
    }
    let mut next = body
        .arena
        .junction(path.middle)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link) = next {
        visit_reentry_incidence(incidence_visits)?;
        let physical = body.arena.link(link).expect("live path-middle incidence");
        next = physical.next;
        if link != path.second
            && body.arrows[link.slot()].is_drive()
        {
            return Ok(false);
        }
    }
    let Some(physical) = body.arena.link(composite) else {
        return Ok(false);
    };
    Ok(body
        .arrows
        .get(composite.slot())
        .is_some_and(|memory| {
            memory.active() && memory.factors() == Some([path.first, path.second])
        })
        && physical.from == path.surface
        && physical.to == path.output
        && physical.delay == first.delay.saturating_add(second.delay)
        && physical.impulse == second.impulse
        && physical.trigger == first.trigger)
}
