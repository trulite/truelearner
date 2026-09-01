
fn mark_current_returns(
    body: ReactionView<'_>,
    facts: &[MomentFact],
    paths: &mut [ReadyPath],
    connected_outcomes: &[JunctionId],
) {
    for fact in facts.iter().filter(|fact| {
        fact.boundary && matches!(fact.used, UsedPaths::None) && !fact.had_ready_path
    }) {
        let source = fact.event.junction;
        let output = latest_fresh_output(body, source).or_else(|| {
            scan_live_returns(body, source, fact.event.cause)
                .selected
                .filter(|returned| returned.opened_at <= fact.event.at)
                .map(|returned| returned.path.output)
        });
        let progress_output = unique_progress_output(paths, source, fact.event.cause);
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
    paths: &[ReadyPath],
    source: JunctionId,
    cause: Cause,
) -> Option<JunctionId> {
    if cause == 0 {
        return None;
    }
    let mut output = None;
    for path in paths.iter().filter(|path| {
        path.progress_source == Some(source)
            && path.return_cause == Some(cause)
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
        let memory = &body.link_memory[witness.slot()];
        if !memory.live || !closes_return(memory.role) {
            continue;
        }
        if memory.transmitted {
            match latest_fresh {
                None => latest_fresh = Some((memory.participated_at, link.to)),
                Some((at, _)) if memory.participated_at > at => {
                    latest_fresh = Some((memory.participated_at, link.to));
                    fresh_ambiguous = false;
                }
                Some((at, output)) if memory.participated_at == at && link.to != output => {
                    fresh_ambiguous = true;
                }
                Some(_) => {}
            }
        }
        for drive in body.arena.incoming(link.to) {
            let physical = body.arena.link(drive).expect("live link");
            let memory = &body.link_memory[drive.slot()];
            if memory.live
                && memory.role == LinkRole::Drive
                && physical.impulse != 0
                && memory.participation > 0
            {
                latest_drive = Some(latest_drive.map_or(memory.participated_at, |at: Time| {
                    at.max(memory.participated_at)
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
    paths: &[ReadyPath],
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
            && path.current_cause != 0
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
            let memory = &body.link_memory[witness.slot()];
            if !memory.live
                || !closes_return(memory.role)
                || link.to == donor.output
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

fn outputs_are_local(body: ReactionView<'_>, left: JunctionId, right: JunctionId) -> bool {
    body.arena.incoming(left).any(|incidence| {
        let link = body.arena.link(incidence).expect("live link");
        let memory = &body.link_memory[incidence.slot()];
        memory.live
            && memory.role == LinkRole::Drive
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
        let memory = &body.link_memory[incidence.slot()];
        if memory.live
            && memory.role == LinkRole::Drive
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
    ready: &[ReadyPath],
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
            if body.link_memory[second_id.slot()].role == LinkRole::Drive
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

fn choose_ready(
    paths: &[ReadyPath],
    worlds: &[usize],
    world: usize,
    construction: bool,
) -> Option<ReadyChoice> {
    if !construction {
        let mut inhibited = (0..paths.len()).filter(|index| {
            worlds[*index] == world && paths[*index].executable && paths[*index].boundary_inhibited
        });
        if let Some(first) = inhibited.next() {
            let source = paths[first].outcome_source?;
            if inhibited.any(|index| paths[index].outcome_source != Some(source)) {
                return None;
            }
            return unique_output(
                (0..paths.len()).filter(|index| {
                    worlds[*index] == world
                        && paths[*index].executable
                        && !paths[*index].boundary_inhibited
                        && paths[*index].outcome_source == Some(source)
                }),
                paths,
            )
            .map(|winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::BoundaryRelease,
            });
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
    unique_ready((0..paths.len()).filter(eligible).filter(|index| {
        let path = &paths[*index];
        path.return_cause.is_some() && path.return_cause == Some(path.current_cause)
    }))
    .map(|winner| ReadyChoice {
        winner,
        basis: ChoiceBasis::CurrentReturn,
    })
    .or_else(|| {
        unique_retained_progress((0..paths.len()).filter(active), paths).map(|winner| ReadyChoice {
            winner,
            basis: ChoiceBasis::RetainedProgress,
        })
    })
    .or_else(|| {
        release_to_untried_output
            .then(|| {
                (0..paths.len())
                    .filter(active)
                    .filter(|index| !output_is_tried(paths[*index].output))
                    .max_by_key(|index| {
                        let path = &paths[*index];
                        (
                            path.participation,
                            path.strength,
                            path.drive,
                            Reverse(path.stable_order),
                        )
                    })
            })
            .flatten()
            .map(|winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::UntriedOutputRelease,
            })
    })
    .or_else(|| {
        unique_returned_output((0..paths.len()).filter(active), paths).map(|winner| ReadyChoice {
            winner,
            basis: ChoiceBasis::CurrentReturn,
        })
    })
    .or_else(|| {
        unique_reentry((0..paths.len()).filter(active), paths).map(|winner| ReadyChoice {
            winner,
            basis: ChoiceBasis::UniqueReentry,
        })
    })
    .or_else(|| {
        unique_motif_reentry((0..paths.len()).filter(active), paths).map(|winner| ReadyChoice {
            winner,
            basis: ChoiceBasis::UniqueMotifReentry,
        })
    })
    .or_else(|| {
        unique_latest_ready(paths, worlds, world, strongest_drive, true, construction).map(
            |winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::AvailableOutcome,
            },
        )
    })
    .or_else(|| {
        let unanswered = latest_unanswered?;
        (0..paths.len())
            .filter(active)
            .filter(|index| paths[*index].output != unanswered)
            .max_by_key(|index| ready_preference(&paths[*index]))
            .map(|winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::UnansweredOutputRelease,
            })
    })
    .or_else(|| {
        unique_latest_ready(paths, worlds, world, strongest_drive, false, construction).map(
            |winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::LatestOutcome,
            },
        )
    })
    .or_else(|| {
        (0..paths.len())
            .filter(active)
            .max_by_key(|index| ready_preference(&paths[*index]))
            .map(|winner| ReadyChoice {
                winner,
                basis: ChoiceBasis::ParticipationStrengthAndDrive,
            })
    })
}

fn ready_preference(path: &ReadyPath) -> (u64, i64, u16, Reverse<u32>) {
    (
        path.participation,
        path.strength,
        path.drive,
        Reverse(path.stable_order),
    )
}

fn unique_reentry(paths: impl Iterator<Item = usize>, ready: &[ReadyPath]) -> Option<usize> {
    let candidates = paths.collect::<Vec<_>>();
    if candidates
        .iter()
        .any(|index| ready[*index].reentry_failed || ready[*index].reentries.len() > 1)
    {
        return None;
    }
    unique_ready(
        candidates
            .into_iter()
            .filter(|index| ready[*index].reentries.len() == 1),
    )
}

fn unique_motif_reentry(paths: impl Iterator<Item = usize>, ready: &[ReadyPath]) -> Option<usize> {
    let candidates = paths.collect::<Vec<_>>();
    if candidates.iter().any(|index| ready[*index].reentry_failed) {
        return None;
    }
    if candidates
        .iter()
        .any(|index| ready[*index].motif_routes.is_some())
    {
        if candidates.iter().any(|index| {
            ready[*index].motif_route_failed
                || ready[*index]
                    .motif_routes
                    .as_ref()
                    .is_some_and(|routes| routes.len() > 1)
        }) {
            return None;
        }
        return unique_ready(candidates.into_iter().filter(|index| {
            ready[*index]
                .motif_routes
                .as_ref()
                .is_some_and(|routes| routes.len() == 1)
        }));
    }
    unique_ready(
        candidates
            .into_iter()
            .filter(|index| !ready[*index].motif_reentries.is_empty()),
    )
}

fn unique_returned_output(
    paths: impl Iterator<Item = usize>,
    ready: &[ReadyPath],
) -> Option<usize> {
    unique_output(
        paths.filter(|index| ready[*index].output_participated),
        ready,
    )
}

fn latest_unanswered_output(
    paths: impl Iterator<Item = usize>,
    ready: &[ReadyPath],
) -> Option<JunctionId> {
    let mut latest = None;
    let mut output = None;
    let mut ambiguous = false;
    for index in paths.filter(|index| ready[*index].unanswered && ready[*index].participation > 0) {
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

fn unique_output(paths: impl Iterator<Item = usize>, ready: &[ReadyPath]) -> Option<usize> {
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
    ready: &[ReadyPath],
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
    role: LinkRole,
) -> Option<JunctionId> {
    let mut source = None;
    for witness in body.arena.incoming(junction) {
        let memory = &body.link_memory[witness.slot()];
        if !memory.live || memory.role != role {
            continue;
        }
        let candidate = body.arena.link(witness).expect("live witness").from;
        match source {
            None => source = Some(candidate),
            Some(existing) if existing != candidate => return None,
            Some(_) => {}
        }
    }
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
        if physical.to == middle
            && body.link_memory[link.slot()].live
            && matches!(body.link_memory[link.slot()].role, LinkRole::Return { .. })
        {
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
    paths: &[ReadyPath],
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
) {
    append_outcome_sources(body, middle, outcomes);
    append_outcome_sources(body, output, outcomes);
}

fn append_outcome_sources(
    body: ReactionView<'_>,
    junction: JunctionId,
    outcomes: &mut Vec<JunctionId>,
) {
    outcomes.extend(
        body.arena
            .incoming(junction)
            .filter(|link| {
                body.link_memory[link.slot()].live
                    && closes_return(body.link_memory[link.slot()].role)
            })
            .filter_map(|link| body.arena.link(link).map(|physical| physical.from)),
    );
}

fn fill_ready_worlds(
    paths: &[ReadyPath],
    connected_outcomes: &[JunctionId],
    parents: &mut Vec<usize>,
) {
    parents.extend(0..paths.len());
    for right in 0..paths.len() {
        for left in 0..right {
            let same_surface = paths[left].surface == paths[right].surface;
            let left_outcomes =
                &connected_outcomes[paths[left].connected_start..paths[left].connected_end];
            let right_outcomes =
                &connected_outcomes[paths[right].connected_start..paths[right].connected_end];
            let connected_outcome = left_outcomes
                .iter()
                .any(|source| right_outcomes.contains(source));
            if same_surface || connected_outcome {
                union(parents, left, right);
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
    change.push(Edit::ChangeLink {
        link: first.into(),
        change: LinkChange::ConsumeOutcome,
    });
    let middle = body.arena.link(first).expect("live path entry").to;
    let mut next = body
        .arena
        .junction(middle)
        .and_then(|junction| junction.outgoing_head);
    while let Some(second) = next {
        let link = body.arena.link(second).expect("live link");
        next = link.next;
        if body.link_memory[second.slot()].role == LinkRole::Drive && link.impulse != 0 {
            change.push(Edit::ChangeLink {
                link: second.into(),
                change: LinkChange::ConsumeOutcome,
            });
            break;
        }
    }
}
