
fn mark_current_returns(
    body: ReactionView<'_>,
    facts: &[MomentFact],
    paths: &mut [CandidatePath],
    connected_outcomes: &[JunctionId],
) {
    for fact in facts.iter().filter(|fact| {
        fact.boundary && matches!(fact.used, UsedPaths::None) && !fact.had_ready_path
    }) {
        let source = fact.event.junction;
        let output = latest_fresh_output(body, source).or_else(|| {
            scan_live_returns(body, source)
                .selected
                .filter(|returned| returned.opened_at <= fact.event.at)
                .map(|returned| returned.path.output)
        });
        let progress_output = unique_progress_output(paths, source);
        for path in paths.iter_mut() {
            let connected = &connected_outcomes[path.connected_start..path.connected_end];
            if connected.contains(&source) {
                path.output_participated |= output == Some(path.output);
            }
            if path.progress_source == Some(source) {
                path.resisted_progress |= progress_output == Some(path.output);
            }
        }
    }
}

fn unique_progress_output(
    paths: &[CandidatePath],
    source: JunctionId,
) -> Option<JunctionId> {
    let mut output = None;
    for path in paths.iter().filter(|path| {
        path.progress_source == Some(source)
            && path.return_present
            && path.participation > 0
    }) {
        match output {
            None => output = Some(path.output),
            Some(existing) if existing != path.output => return None,
            Some(_) => {}
        }
    }
    output
}

fn latest_fresh_output(body: ReactionView<'_>, source: JunctionId) -> Option<JunctionId> {
    let mut latest_fresh = None;
    let mut fresh_ambiguous = false;
    let mut latest_drive = None;
    let mut next = body
        .arena
        .junction(source)
        .and_then(|junction| junction.outgoing_head);
    while let Some(witness) = next {
        let link = body.arena.link(witness).expect("live link");
        next = link.next;
        let memory = &body.arrows[witness.slot()];
        if !closes_return(memory) {
            continue;
        }
        if let Some(occurrence) = memory.last_transmission() {
            match latest_fresh {
                None => latest_fresh = Some((occurrence.at, link.to)),
                Some((at, _)) if occurrence.at > at => {
                    latest_fresh = Some((occurrence.at, link.to));
                    fresh_ambiguous = false;
                }
                Some((at, output)) if occurrence.at == at && link.to != output => {
                    fresh_ambiguous = true;
                }
                Some(_) => {}
            }
        }
        for drive in body.arena.incoming(link.to) {
            let physical = body.arena.link(drive).expect("live link");
            let memory = &body.arrows[drive.slot()];
            if memory.is_drive()
                && physical.impulse != 0
                && memory.participation() > 0
            {
                let participated_at = memory.occurrence().map_or(0, |occurrence| occurrence.at);
                latest_drive = Some(latest_drive.map_or(participated_at, |at: Time| {
                    at.max(participated_at)
                }));
            }
        }
    }
    if fresh_ambiguous {
        return None;
    }
    let (fresh_at, output) = latest_fresh?;
    latest_drive
        .is_none_or(|drive_at| fresh_at > drive_at)
        .then_some(output)
}

fn fresh_opportunity(
    body: ReactionView<'_>,
    paths: &[CandidatePath],
    connected_outcomes: &[JunctionId],
    worlds: &[usize],
    world: usize,
    construction: bool,
) -> Option<(usize, FreshOpportunityTrace)> {
    if construction {
        return None;
    }
    let strongest_drive = paths
        .iter()
        .enumerate()
        .filter(|(index, path)| {
            worlds[*index] == world && path.executable && !path.boundary_inhibited
        })
        .map(|(_, path)| path.drive)
        .max()?;
    let mut donors = paths.iter().enumerate().filter(|(index, path)| {
        worlds[*index] == world
            && path.executable
            && !path.boundary_inhibited
            && path.drive == strongest_drive
            && path.unanswered
            && path.participation > 0
    });
    let (donor_index, donor) = donors.next()?;
    if donors.next().is_some() {
        return None;
    }
    if paths.iter().enumerate().any(|(index, path)| {
        worlds[index] == world
            && path.executable
            && !path.boundary_inhibited
            && path.drive == strongest_drive
            && path.surface != donor.surface
    }) {
        return None;
    }
    let mut selected = None;
    for source in &connected_outcomes[donor.connected_start..donor.connected_end] {
        let mut next = body
            .arena
            .junction(*source)
            .and_then(|junction| junction.outgoing_head);
        while let Some(witness) = next {
            let link = body.arena.link(witness).expect("live link");
            next = link.next;
            let memory = &body.arrows[witness.slot()];
            if !closes_return(memory)
                || link.to == donor.output
                || output_has_returned_path(body, link.to)
                || paths.iter().enumerate().any(|(index, path)| {
                    worlds[index] == world
                        && path.executable
                        && !path.boundary_inhibited
                        && path.output == link.to
                })
                || !outputs_are_local(body, donor.output, link.to)
            {
                continue;
            }
            let fresh = FreshOpportunityTrace {
                source: *source,
                output: link.to,
                through: witness,
            };
            if selected.is_none_or(|current: FreshOpportunityTrace| witness < current.through) {
                selected = Some(fresh);
            }
        }
    }
    selected.map(|fresh| (donor_index, fresh))
}

fn output_has_returned_path(body: ReactionView<'_>, output: JunctionId) -> bool {
    body.arena.incoming(output).any(|drive| {
        path_from_drive(body, drive).is_some()
            && body.arrows[drive.slot()].participation() > 0
            && body.arrows[drive.slot()]
                .outcome()
                .is_some_and(|outcome| outcome.changed_world)
    })
}

fn outputs_are_local(body: ReactionView<'_>, left: JunctionId, right: JunctionId) -> bool {
    body.arena.incoming(left).any(|incidence| {
        let link = body.arena.link(incidence).expect("live link");
        let memory = &body.arrows[incidence.slot()];
        memory.is_drive()
            && link.impulse == 0
            && (1..=LOCAL_RADIUS as Time).contains(&link.delay)
            && surface_reaches(body, link.from, right)
    })
}

fn surface_reaches(body: ReactionView<'_>, surface: JunctionId, output: JunctionId) -> bool {
    let mut next = body
        .arena
        .junction(surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(incidence) = next {
        let link = body.arena.link(incidence).expect("live link");
        next = link.next;
        let memory = &body.arrows[incidence.slot()];
        if memory.is_drive()
            && link.to == output
            && link.impulse == 0
            && (1..=LOCAL_RADIUS as Time).contains(&link.delay)
        {
            return true;
        }
    }
    false
}

fn ready_path_exists(
    body: ReactionView<'_>,
    ready: &[CandidatePath],
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
            if body.arrows[second_id.slot()].is_drive()
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

fn choose_ready<F>(
    paths: &[CandidatePath],
    worlds: &[usize],
    world: usize,
    construction: bool,
    fresh: F,
) -> Option<ReadyChoice>
where
    F: Fn() -> Option<(usize, FreshOpportunityTrace)>,
{
    if !construction {
        let mut inhibited = (0..paths.len()).filter(|index| {
            worlds[*index] == world && paths[*index].executable && paths[*index].boundary_inhibited
        });
        if let Some(first) = inhibited.next() {
            let source = paths[first].outcome_source?;
            if inhibited.any(|index| paths[index].outcome_source != Some(source)) {
                return None;
            }
            let has_uninhibited = (0..paths.len()).any(|index| {
                worlds[index] == world
                    && paths[index].executable
                    && !paths[index].boundary_inhibited
                    && paths[index].outcome_source == Some(source)
            });
            if has_uninhibited {
                if let Some(winner) = unique_output(
                    (0..paths.len()).filter(|index| {
                        worlds[*index] == world
                            && paths[*index].executable
                            && !paths[*index].boundary_inhibited
                            && paths[*index].outcome_source == Some(source)
                    }),
                    paths,
                ) {
                    return Some(warranted(
                        winner,
                        ChoiceWarrant::ReturnedConsequence,
                    ));
                }
            } else {
                let completed = latest_participated_output(
                    (0..paths.len()).filter(|index| {
                        worlds[*index] == world
                            && paths[*index].executable
                            && paths[*index].boundary_inhibited
                            && paths[*index].outcome_source == Some(source)
                    }),
                    paths,
                )?;
                return unique_output(
                    (0..paths.len()).filter(|index| {
                        worlds[*index] == world
                            && paths[*index].executable
                            && paths[*index].outcome_source == Some(source)
                            && paths[*index].output != completed
                    }),
                    paths,
                )
                .map(|winner| warranted(winner, ChoiceWarrant::ReturnedConsequence));
            }
        }
    }
    let eligible = |index: &usize| {
        worlds[*index] == world
            && (construction || (paths[*index].executable && !paths[*index].boundary_inhibited))
    };
    let strongest_drive = (0..paths.len())
        .filter(eligible)
        .map(|index| paths[index].drive)
        .max()?;
    let active = |index: &usize| eligible(index) && paths[*index].drive == strongest_drive;
    let latest_unanswered = latest_unanswered_output((0..paths.len()).filter(active), paths);
    let output_is_tried = |output| {
        (0..paths.len()).any(|index| {
            active(&index)
                && paths[index].output == output
                && (paths[index].participation > 0 || paths[index].output_participated)
        })
    };
    let has_tried_output =
        (0..paths.len()).any(|index| active(&index) && output_is_tried(paths[index].output));
    let has_untried_output =
        (0..paths.len()).any(|index| active(&index) && !output_is_tried(paths[index].output));
    let release_to_untried_output = has_tried_output && has_untried_output;

    if let Some(winner) = unique_ready((0..paths.len()).filter(eligible).filter(|index| {
        paths[*index].return_present
    })) {
        return Some(warranted(
            winner,
            ChoiceWarrant::ReturnedConsequence,
        ));
    }

    if let Some(winner) = unique_retained_progress((0..paths.len()).filter(active), paths) {
        return Some(warranted(
            winner,
            ChoiceWarrant::RetainedContinuation,
        ));
    }

    if release_to_untried_output {
        if let Some((winner, evidence)) = fresh() {
            return Some(warranted_fresh(winner, evidence));
        }
        if let Some(winner) = (0..paths.len())
            .filter(active)
            .filter(|index| !output_is_tried(paths[*index].output))
            .max_by_key(|index| ready_preference(&paths[*index]))
        {
            return Some(warranted(winner, ChoiceWarrant::Exploration));
        }
    }

    if let Some(winner) = unique_returned_output((0..paths.len()).filter(active), paths) {
        return Some(warranted(
            winner,
            ChoiceWarrant::ReturnedConsequence,
        ));
    }

    if let Some(winner) = unique_reentry((0..paths.len()).filter(active), paths) {
        return Some(warranted(winner, ChoiceWarrant::Reentry));
    }
    if let Some(winner) = unique_motif_reentry((0..paths.len()).filter(active), paths) {
        return Some(warranted(winner, ChoiceWarrant::Reentry));
    }

    if let Some((winner, evidence)) = fresh() {
        return Some(warranted_fresh(winner, evidence));
    }
    if let Some(winner) =
        unique_latest_ready(paths, worlds, world, strongest_drive, true, construction)
    {
        return Some(warranted(
            winner,
            ChoiceWarrant::RetainedContinuation,
        ));
    }

    if let Some(unanswered) = latest_unanswered {
        if let Some(winner) = (0..paths.len())
            .filter(active)
            .filter(|index| paths[*index].output != unanswered)
            .max_by_key(|index| ready_preference(&paths[*index]))
        {
            return Some(warranted(winner, ChoiceWarrant::Exploration));
        }
    }

    if let Some(winner) =
        unique_latest_ready(paths, worlds, world, strongest_drive, false, construction)
    {
        return Some(warranted(winner, ChoiceWarrant::LocalIncidence));
    }
    (0..paths.len())
        .filter(active)
        .max_by_key(|index| ready_preference(&paths[*index]))
        .map(|winner| warranted(winner, ChoiceWarrant::LocalIncidence))
}

const fn warranted(winner: usize, warrant: ChoiceWarrant) -> ReadyChoice {
    ReadyChoice {
        winner,
        warrant,
        fresh_through: None,
    }
}

const fn warranted_fresh(winner: usize, evidence: FreshOpportunityTrace) -> ReadyChoice {
    ReadyChoice {
        winner,
        warrant: ChoiceWarrant::RetainedContinuation,
        fresh_through: Some(evidence.through),
    }
}

fn ready_preference(path: &CandidatePath) -> (u64, i64, u16, Reverse<u32>) {
    (
        path.participation,
        path.strength,
        path.drive,
        Reverse(path.stable_order),
    )
}

fn unique_reentry(paths: impl Iterator<Item = usize>, ready: &[CandidatePath]) -> Option<usize> {
    let candidates = paths.collect::<Vec<_>>();
    if candidates
        .iter()
        .any(|index| ready[*index].continuation.reentry_failed || ready[*index].continuation.reentries.len() > 1)
    {
        return None;
    }
    unique_ready(
        candidates
            .into_iter()
            .filter(|index| ready[*index].continuation.reentries.len() == 1),
    )
}

fn unique_motif_reentry(paths: impl Iterator<Item = usize>, ready: &[CandidatePath]) -> Option<usize> {
    let candidates = paths.collect::<Vec<_>>();
    if candidates.iter().any(|index| ready[*index].continuation.reentry_failed) {
        return None;
    }
    if candidates
        .iter()
        .any(|index| ready[*index].continuation.motif_routes.is_some())
    {
        if candidates.iter().any(|index| {
            ready[*index].continuation.motif_route_failed
                || ready[*index]
                    .continuation.motif_routes
                    .as_ref()
                    .is_some_and(|routes| routes.len() > 1)
        }) {
            return None;
        }
        return unique_ready(candidates.into_iter().filter(|index| {
            ready[*index]
                .continuation.motif_routes
                .as_ref()
                .is_some_and(|routes| routes.len() == 1)
        }));
    }
    unique_ready(
        candidates
            .into_iter()
            .filter(|index| !ready[*index].continuation.motif_reentries.is_empty()),
    )
}

fn unique_returned_output(
    paths: impl Iterator<Item = usize>,
    ready: &[CandidatePath],
) -> Option<usize> {
    unique_output(
        paths.filter(|index| ready[*index].output_participated),
        ready,
    )
}

fn latest_unanswered_output(
    paths: impl Iterator<Item = usize>,
    ready: &[CandidatePath],
) -> Option<JunctionId> {
    latest_participated_output(paths.filter(|index| ready[*index].unanswered), ready)
}

fn latest_participated_output(
    paths: impl Iterator<Item = usize>,
    ready: &[CandidatePath],
) -> Option<JunctionId> {
    let mut latest = None;
    let mut output = None;
    let mut ambiguous = false;
    for index in paths.filter(|index| ready[*index].participation > 0) {
        let path = &ready[index];
        match latest {
            None => {
                latest = Some(path.participated_at);
                output = Some(path.output);
            }
            Some(at) if path.participated_at > at => {
                latest = Some(path.participated_at);
                output = Some(path.output);
                ambiguous = false;
            }
            Some(at) if path.participated_at == at && output != Some(path.output) => {
                ambiguous = true;
            }
            Some(_) => {}
        }
    }
    if ambiguous {
        None
    } else {
        output
    }
}

fn unique_output(paths: impl Iterator<Item = usize>, ready: &[CandidatePath]) -> Option<usize> {
    let mut output: Option<JunctionId> = None;
    let mut winner: Option<usize> = None;
    for index in paths {
        match output {
            None => output = Some(ready[index].output),
            Some(current) if current != ready[index].output => return None,
            Some(_) => {}
        }
        if winner.is_none_or(|current| {
            let path = &ready[index];
            let selected = &ready[current];
            (
                path.participation,
                path.strength,
                path.drive,
                Reverse(path.stable_order),
            ) > (
                selected.participation,
                selected.strength,
                selected.drive,
                Reverse(selected.stable_order),
            )
        }) {
            winner = Some(index);
        }
    }
    winner
}

fn unique_retained_progress(
    paths: impl Iterator<Item = usize>,
    ready: &[CandidatePath],
) -> Option<usize> {
    unique_output(
        paths.filter(|index| {
            ready[*index].resisted_progress
                && ready[*index].boundary_open
                && ready[*index].strength > 1
                && ready[*index].participation > 0
        }),
        ready,
    )
}

fn unique_witness_source(
    body: ReactionView<'_>,
    junction: JunctionId,
    kind: WitnessKind,
    incidence: &mut ChoiceIncidence,
) -> Option<JunctionId> {
    let key = (
        junction,
        match kind {
            WitnessKind::Progress => 0,
            WitnessKind::Closure {
                offers_choice: false,
            } => 1,
            WitnessKind::Closure {
                offers_choice: true,
            } => 2,
        },
    );
    if let Some(source) = incidence.witnesses.get(&key) {
        return *source;
    }
    let mut source = None;
    for witness in body.arena.incoming(junction) {
        let memory = &body.arrows[witness.slot()];
        if memory.witness_kind() != Some(kind) {
            continue;
        }
        let candidate = body.arena.link(witness).expect("live witness").from;
        match source {
            None => source = Some(candidate),
            Some(existing) if existing != candidate => {
                incidence.witnesses.insert(key, None);
                return None;
            }
            Some(_) => {}
        }
    }
    incidence.witnesses.insert(key, source);
    source
}

fn path_has_open_return(body: ReactionView<'_>, middle: JunctionId, output: JunctionId) -> bool {
    let mut next = body
        .arena
        .junction(output)
        .and_then(|junction| junction.outgoing_head);
    while let Some(link) = next {
        let physical = body.arena.link(link).expect("live link");
        next = physical.next;
        if physical.to == middle && body.arrows[link.slot()].open_return_data().is_some() {
            return true;
        }
    }
    false
}

fn unique_ready(mut paths: impl Iterator<Item = usize>) -> Option<usize> {
    let path = paths.next()?;
    paths.next().is_none().then_some(path)
}

fn unique_latest_ready(
    paths: &[CandidatePath],
    worlds: &[usize],
    world: usize,
    drive: u16,
    available_only: bool,
    construction: bool,
) -> Option<usize> {
    let latest = (0..paths.len())
        .filter(|index| {
            worlds[*index] == world
                && paths[*index].drive == drive
                && (construction || paths[*index].executable)
        })
        .filter_map(|index| {
            paths[index]
                .outcome
                .filter(|outcome| !available_only || outcome.available_until_choice)
        })
        .map(|outcome| outcome.at)
        .max()?;
    unique_ready((0..paths.len()).filter(|index| {
        if worlds[*index] != world
            || paths[*index].drive != drive
            || (!construction && !paths[*index].executable)
        {
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
    incidence: &mut ChoiceIncidence,
) {
    append_outcome_sources(body, middle, outcomes, incidence);
    append_outcome_sources(body, output, outcomes, incidence);
}

fn append_outcome_sources(
    body: ReactionView<'_>,
    junction: JunctionId,
    outcomes: &mut Vec<JunctionId>,
    incidence: &mut ChoiceIncidence,
) {
    let sources = incidence.outcome_sources.entry(junction).or_insert_with(|| {
        body.arena
            .incoming(junction)
            .filter(|link| {
                closes_return(&body.arrows[link.slot()])
            })
            .filter_map(|link| body.arena.link(link).map(|physical| physical.from))
            .collect()
    });
    outcomes.extend_from_slice(sources);
}

fn fill_ready_worlds(
    paths: &[CandidatePath],
    connected_outcomes: &[JunctionId],
    parents: &mut Vec<usize>,
) {
    parents.extend(0..paths.len());
    let mut surface_owner = BTreeMap::new();
    let mut outcome_owner = BTreeMap::new();
    for (index, path) in paths.iter().enumerate() {
        if let Some(owner) = surface_owner.insert(path.surface, index) {
            union(parents, owner, index);
        }
        for source in &connected_outcomes[path.connected_start..path.connected_end] {
            if let Some(owner) = outcome_owner.insert(*source, index) {
                union(parents, owner, index);
            }
        }
    }
    for index in 0..parents.len() {
        parents[index] = find(parents, index);
    }
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

fn consume_path_outcome(body: ReactionView<'_>, change: &mut Change, first: LinkId) {
    change.change_link(first.into(), LinkChange::ConsumeOutcome);
    let middle = body.arena.link(first).expect("live path entry").to;
    let mut next = body
        .arena
        .junction(middle)
        .and_then(|junction| junction.outgoing_head);
    while let Some(second) = next {
        let link = body.arena.link(second).expect("live link");
        next = link.next;
        if body.arrows[second.slot()].is_drive() && link.impulse != 0 {
            change.change_link(second.into(), LinkChange::ConsumeOutcome);
            break;
        }
    }
}
