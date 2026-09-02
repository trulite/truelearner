
#[inline(always)]
pub(crate) fn try_complete_single_return<T: TraceSink>(
    body: &mut Body,
    event: crate::physics::Event,
    local_plasticity: Option<(&mut Vec<JunctionId>, &mut Vec<LinkId>)>,
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
            path: Some(returned.path),
            return_opened_at: Some(returned.opened_at),
            offers_choice: Some(returned.offers_choice),
            open_paths: 1,
            candidates: vec![ReturnCandidateTrace {
                path: returned.path,
                opened_at: returned.opened_at,
            }],
            decision: ReturnDecision::Accepted,
        }));
    }
    if !returned.offers_choice {
        clear_output_selection_direct(body, returned.path.output);
    }
    if body.needs_automatic_closure() {
        body.complete_automatic_witness(returned.link, returned.path);
    }
    if entry.exclusive_source {
        body.returns.live_count -= 1;
        body.returns.by_source[source.slot()].clear();
    } else {
        body.remove_live_return_with_path(returned.link, Some(returned.path));
    }
    let mut supported_closures = body.arrows[returned.path.first.slot()].supported_closures();
    for (index, link) in returned.path.links().into_iter().enumerate() {
        let memory = &mut body.arrows[link.slot()];
        let (closures, before, after) = memory
            .learn_closure(event.at, returned.offers_choice)
            .unwrap_or((0, 1, 1));
        if index == 0 {
            supported_closures = closures;
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
    if body.has_local_plasticity {
        let (local_junctions, local_eligible) =
            local_plasticity.expect("plastic body supplies local reaction scratch");
        strengthen_recent_local_inputs_direct(
            body,
            returned.path,
            event.at,
            local_junctions,
            local_eligible,
            trace,
        );
    }
    if let Some(support) = body.retained_closed_support(
        returned.link,
        source,
        returned.path,
        entry.outcome_witness,
        supported_closures > 1,
    ) {
        body.arrows[returned.link.slot()].close_return(event.at, support, None);
    }
    if supported_closures >= AUTOMATIC_AFTER_SUPPORTED_CLOSURES {
        retain_composite_direct(body, returned.path, event.at, trace);
    }
    true
}

fn strengthen_recent_local_inputs_direct<T: TraceSink>(
    body: &mut Body,
    returned: Path,
    at: Time,
    local_junctions: &mut Vec<JunctionId>,
    local_eligible: &mut Vec<LinkId>,
    trace: &mut T,
) {
    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    local_eligible.clear();
    append_recent_local_inputs(view, returned, at, local_junctions, local_eligible);
    for link in local_eligible.iter().copied() {
        let (before, after) = body.arrows[link.slot()].strengthen(1).unwrap_or((1, 1));
        if T::ENABLED {
            trace.record(TraceEvent::Strengthened(StrengthTrace {
                at,
                link,
                before,
                after,
            }));
        }
    }
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
    opened_at: Time,
    offers_choice: bool,
}

#[derive(Clone, Copy)]
struct ReturnSelection {
    selected: Option<OpenReturn>,
    total: usize,
}

fn scan_live_returns(body: ReactionView<'_>, source: JunctionId) -> ReturnSelection {
    let mut only = None;
    let mut total = 0_usize;
    let mut latest_at = None;
    let mut latest_count = 0_usize;
    let Some(entries) = body.returns.by_source.get(source.slot()) else {
        return ReturnSelection {
            selected: None,
            total: 0,
        };
    };
    for entry in entries {
        let Some(returned) = open_return(body, *entry) else {
            continue;
        };
        total += 1;
        match latest_at {
            None => {
                latest_at = Some(returned.opened_at);
                latest_count = 1;
                only = Some(returned);
            }
            Some(at) if returned.opened_at > at => {
                latest_at = Some(returned.opened_at);
                latest_count = 1;
                only = Some(returned);
            }
            Some(at) if returned.opened_at == at => {
                latest_count += 1;
                only = None;
            }
            Some(_) => {}
        }
    }
    ReturnSelection {
        selected: (latest_count == 1).then_some(only).flatten(),
        total,
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
            opened_at: returned.opened_at,
        })
        .collect()
}

#[inline(always)]
fn open_return(body: ReactionView<'_>, entry: ReturnEntry) -> Option<OpenReturn> {
    let memory = body.arrows.get(entry.link.slot())?;
    let (path, opened_at, _) = memory.open_return_data()?;
    debug_assert_eq!(path, entry.path);
    Some(OpenReturn {
        link: entry.link,
        path: entry.path,
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

#[cfg(test)]
mod return_selection_tests {
    use super::*;

    fn open_path(body: &mut Body, returned_source: JunctionId, opened_at: Time) -> Path {
        let surface = body.add_junction(Junction::integrating(1)).unwrap();
        let middle = body.add_junction(Junction::integrating(1)).unwrap();
        let output = body.add_junction(Junction::integrating(1)).unwrap();
        let first = body.add_link(Link::new(surface, middle, 1, 1)).unwrap();
        let second = body.add_link(Link::new(middle, output, 1, 1)).unwrap();
        let witness = body
            .add_link(Link::new(returned_source, output, 0, 1))
            .unwrap();
        body.mark_witness(
            witness,
            WitnessKind::Closure {
                offers_choice: true,
            },
        )
        .unwrap();
        let path = Path {
            surface,
            middle,
            output,
            first,
            second,
        };
        let returned = body.add_link(Link::new(output, surface, 0, 0)).unwrap();
        body.replace_arrow_state(returned, ArrowState::open_return(path, opened_at))
            .unwrap();
        path
    }

    #[test]
    fn newest_local_trace_is_unique_and_an_equal_time_tie_is_ambiguous() {
        let mut body = Body::default();
        let returned_source = body.add_junction(Junction::integrating(1)).unwrap();
        let older = open_path(&mut body, returned_source, 9);
        let newer = open_path(&mut body, returned_source, 10);
        let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
        let selected = scan_live_returns(view, returned_source);
        assert_eq!(selected.total, 2);
        assert_eq!(selected.selected.map(|returned| returned.path), Some(newer));

        let tied_return = body
            .arrows
            .iter()
            .enumerate()
            .find_map(|(slot, memory)| {
                memory
                    .open_return_data()
                    .filter(|(path, _, _)| *path == older)
                    .and_then(|_| LinkId::new(slot))
            })
            .unwrap();
        body.replace_arrow_state(tied_return, ArrowState::open_return(older, 10))
            .unwrap();
        let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
        let tied = scan_live_returns(view, returned_source);
        assert_eq!(tied.total, 2);
        assert!(tied.selected.is_none());
    }

    #[test]
    fn a_tied_return_strengthens_recent_links_in_both_local_cones() {
        let mut body = Body::default();
        let returned_source = body.add_junction(Junction::integrating(1)).unwrap();
        let left = open_path(&mut body, returned_source, 10);
        let right = open_path(&mut body, returned_source, 10);
        let left_source = body.add_junction(Junction::integrating(1)).unwrap();
        let right_source = body.add_junction(Junction::integrating(1)).unwrap();
        let left_eligible = body
            .add_link(Link::new(left_source, left.surface, 1, 1))
            .unwrap();
        let right_eligible = body
            .add_link(Link::new(right_source, right.surface, 1, 1))
            .unwrap();
        body.mark_locally_plastic(left_eligible).unwrap();
        body.mark_locally_plastic(right_eligible).unwrap();
        body.inputs(
            10,
            &[
                crate::Arrival::new(left_source, 1),
                crate::Arrival::new(right_source, 1),
            ],
        )
        .unwrap();
        body.run(32, |_| {}).unwrap();

        body.input(14, returned_source, 1).unwrap();
        let mut trace = Vec::new();
        body.run_traced(32, |_| {}, |event| trace.push(event))
            .unwrap();

        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned) if returned.decision == ReturnDecision::Ambiguous
        )));
        assert_eq!(body.arrows[left_eligible.slot()].strength(), 2);
        assert_eq!(body.arrows[right_eligible.slot()].strength(), 2);
    }
}
