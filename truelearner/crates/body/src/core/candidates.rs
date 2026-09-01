
#[derive(Clone, Debug)]
struct CandidatePath {
    surface: JunctionId,
    middle: JunctionRef,
    output: JunctionId,
    first: LinkRef,
    second: LinkRef,
    form: PathForm,
    at: Time,
    current_cause: Cause,
    return_cause: Option<Cause>,
    unanswered: bool,
    connected_start: usize,
    connected_end: usize,
    outcome: Option<Outcome>,
    participation: u64,
    participated_at: Time,
    output_participated: bool,
    outcome_source: Option<JunctionId>,
    progress_source: Option<JunctionId>,
    resisted_progress: bool,
    boundary_open: bool,
    boundary_inhibited: bool,
    strength: i64,
    drive: u16,
    stable_order: u32,
    continuation: ContinuationResult,
    executable: bool,
}

#[derive(Clone, Debug, Default)]
struct ContinuationResult {
    fresh_opportunity: Option<FreshOpportunityTrace>,
    reentries: Vec<ReentryTrace>,
    motif_reentries: Vec<MotifReentryTrace>,
    motif_routes: Option<Box<[MotifRouteTrace]>>,
    reentry_incidence_visits: u16,
    reentry_shortcut_hits: u16,
    reentry_failed: bool,
    motif_route_failed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LinkForm {
    delay: Time,
    impulse: Impulse,
    trigger: Trigger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PathForm {
    surface: Junction,
    first: LinkForm,
    second: LinkForm,
}

impl CandidatePath {
    const fn trace_path(&self) -> TracePath {
        TracePath {
            surface: self.surface,
            middle: self.middle,
            output: self.output,
            first: self.first,
            second: self.second,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ReadyChoice {
    winner: usize,
    basis: ChoiceBasis,
}

#[allow(clippy::too_many_arguments)]
fn form_and_choose<T: TraceSink>(
    body: ReactionView<'_>,
    facts: &mut [MomentFact],
    ready: &mut Vec<CandidatePath>,
    connected_outcomes: &mut Vec<JunctionId>,
    worlds: &mut Vec<usize>,
    winners: &mut Vec<ReadyChoice>,
    reentry: &mut ReentryScratch,
    change: &mut Change,
    construction: bool,
    trace: &mut T,
) {
    for fact in facts.iter_mut() {
        if !fact.boundary {
            continue;
        }

        let surface = fact.event.junction;
        let current_cause = if is_progress_source(body, surface) {
            0
        } else {
            fact.event.cause
        };
        let ready_start = ready.len();
        append_existing_ready_paths(
            body,
            fact.event,
            current_cause,
            fact.drive,
            ready,
            connected_outcomes,
        );
        fact.had_ready_path = ready.len() > ready_start;
        let mut next = body
            .arena
            .junction(surface)
            .and_then(|junction| junction.outgoing_head);
        while let Some(morphology_id) = next {
            let morphology = *body.arena.link(morphology_id).expect("live link");
            next = morphology.next;
            if morphology.impulse != 0
                || !body.arrows[morphology_id.slot()].is_drive()
                || !(1..=LOCAL_RADIUS as Time).contains(&morphology.delay)
            {
                continue;
            }
            for sign in [1_i8, -1_i8] {
                if ready_path_exists(body, &ready[ready_start..], morphology.to, sign) {
                    continue;
                }
                let middle = change.add_junction(Junction::integrating(1));
                let entry_trigger =
                    path_entry_trigger(body, surface, fact.event.before, fact.event.after);
                let first = change.add_link(
                    surface.into(),
                    middle.into(),
                    LinkSpec {
                        delay: morphology.delay,
                        impulse: 1,
                        trigger: entry_trigger,
                        state: ArrowState::entry(),
                    },
                );
                let second = change.add_link(
                    middle.into(),
                    morphology.to.into(),
                    LinkSpec {
                        delay: morphology.delay,
                        impulse: i32::from(sign),
                        trigger: Trigger::SourceFires,
                        state: ArrowState::drive(),
                    },
                );
                let connected_start = connected_outcomes.len();
                append_outcome_sources(body, morphology.to, connected_outcomes);
                let connected_end = connected_outcomes.len();
                ready.push(CandidatePath {
                    surface,
                    middle: middle.into(),
                    output: morphology.to,
                    first: first.into(),
                    second: second.into(),
                    form: PathForm {
                        surface: body
                            .arena
                            .junction(surface)
                            .expect("current surface")
                            .checkpoint_law(),
                        first: LinkForm {
                            delay: morphology.delay,
                            impulse: 1,
                            trigger: entry_trigger,
                        },
                        second: LinkForm {
                            delay: morphology.delay,
                            impulse: i32::from(sign),
                            trigger: Trigger::SourceFires,
                        },
                    },
                    at: fact.event.at,
                    current_cause,
                    return_cause: None,
                    unanswered: false,
                    connected_start,
                    connected_end,
                    outcome: None,
                    participation: 0,
                    participated_at: 0,
                    output_participated: false,
                    outcome_source: unique_witness_source(
                        body,
                        morphology.to,
                        WitnessKind::Closure {
                            offers_choice: true,
                        },
                    ),
                    progress_source: unique_witness_source(
                        body,
                        morphology.to,
                        WitnessKind::Progress,
                    ),
                    resisted_progress: false,
                    boundary_open: false,
                    boundary_inhibited: false,
                    strength: 1,
                    drive: fact.drive,
                    stable_order: u32::try_from(body.arena.link_count())
                        .unwrap_or(u32::MAX)
                        .saturating_add(second.0),
                    continuation: ContinuationResult::default(),
                    executable: path_is_executable(body, surface, false),
                });
            }
        }
    }

    mark_current_returns(body, facts, ready, connected_outcomes);
    mark_reentries(body, facts, ready, reentry, change, construction);
    mark_motif_reentries(body, ready, &reentry.present, construction);
    fill_ready_worlds(ready, connected_outcomes, worlds);
    for world in 0..ready.len() {
        if worlds[world] == world {
            if let Some(mut choice) = choose_ready(ready, worlds, world, construction) {
                if !matches!(
                    choice.basis,
                    ChoiceBasis::CurrentReturn
                        | ChoiceBasis::BoundaryRelease
                        | ChoiceBasis::RetainedProgress
                        | ChoiceBasis::UniqueReentry
                        | ChoiceBasis::UniqueMotifReentry
                ) {
                    if let Some((donor, fresh)) = fresh_opportunity(
                        body,
                        ready,
                        connected_outcomes,
                        worlds,
                        world,
                        construction,
                    ) {
                        ready[donor].continuation.fresh_opportunity = Some(fresh);
                        choice.winner = donor;
                        choice.basis = ChoiceBasis::FreshOpportunity;
                    }
                }
                winners.push(choice);
            }
        }
    }
    winners.sort_by_key(|choice| ready[choice.winner].surface);
    if T::ENABLED {
        for (index, candidate) in ready.iter().enumerate() {
            trace.record(TraceEvent::Candidate(CandidateTrace {
                at: candidate.at,
                cause: candidate.current_cause,
                group: worlds[index],
                path: candidate.trace_path(),
                connected_outcomes: connected_outcomes
                    [candidate.connected_start..candidate.connected_end]
                    .to_vec(),
                executable: candidate.executable,
                return_cause: candidate.return_cause,
                unanswered: candidate.unanswered,
                outcome: candidate.outcome,
                participation: candidate.participation,
                participated_at: candidate.participated_at,
                output_participated: candidate.output_participated,
                outcome_source: candidate.outcome_source,
                progress_source: candidate.progress_source,
                resisted_progress: candidate.resisted_progress,
                boundary_open: candidate.boundary_open,
                boundary_inhibited: candidate.boundary_inhibited,
                strength: candidate.strength,
                drive: candidate.drive,
                stable_order: candidate.stable_order,
                fresh_opportunity: candidate.continuation.fresh_opportunity,
                present_sources: reentry.present.clone(),
                reentries: candidate.continuation.reentries.clone(),
                motif_reentries: candidate.continuation.motif_reentries.clone(),
                motif_routes: candidate
                    .continuation.motif_routes
                    .as_deref()
                    .unwrap_or_default()
                    .to_vec(),
                reentry_incidence_visits: candidate.continuation.reentry_incidence_visits,
                reentry_shortcut_hits: candidate.continuation.reentry_shortcut_hits,
                reentry_failed: candidate.continuation.reentry_failed,
                motif_route_failed: candidate.continuation.motif_route_failed,
                new_path: matches!(candidate.first, LinkRef::New(_)),
            }));
        }
        for world in 0..ready.len() {
            if worlds[world] != world {
                continue;
            }
            let choice = winners.iter().find(|choice| worlds[choice.winner] == world);
            let at = ready
                .iter()
                .enumerate()
                .find(|(index, _)| worlds[*index] == world)
                .map_or(0, |(_, candidate)| candidate.at);
            trace.record(TraceEvent::Choice(ChoiceTrace {
                at,
                group: world,
                alternatives: worlds.iter().filter(|group| **group == world).count(),
                winner: choice.map(|choice| {
                    let winner = &ready[choice.winner];
                    winner.trace_path()
                }),
                basis: choice.map(|choice| choice.basis),
                construction,
                sent: choice.is_some() && !construction,
            }));
        }
    }
    if construction {
        return;
    }
    for choice in winners.iter() {
        let winner = &ready[choice.winner];
        let through = winner.continuation.fresh_opportunity.map_or_else(
            || {
                let path = match (winner.middle, winner.first, winner.second) {
                    (
                        JunctionRef::Existing(middle),
                        LinkRef::Existing(first),
                        LinkRef::Existing(second),
                    ) => Some(Path {
                        surface: winner.surface,
                        middle,
                        output: winner.output,
                        first,
                        second,
                    }),
                    _ => None,
                };
                path.and_then(|path| usable_composite(body, path))
                    .map_or(winner.first, LinkRef::Existing)
            },
            |fresh| fresh.through.into(),
        );
        change.send(through, winner.at, winner.current_cause);
        for (index, candidate) in ready.iter().enumerate() {
            if worlds[index] != worlds[choice.winner] || !candidate.boundary_inhibited {
                continue;
            }
            for link in [candidate.first, candidate.second] {
                change.change_link(link, LinkChange::ConsumeBoundaryInhibition);
            }
        }
        if winner.continuation.fresh_opportunity.is_none()
            && winner
                .outcome
                .is_some_and(|outcome| outcome.available_until_choice)
        {
            if let LinkRef::Existing(first) = winner.first {
                consume_path_outcome(body, change, first);
            }
        }
    }
}

fn append_existing_ready_paths(
    body: ReactionView<'_>,
    event: crate::physics::Event,
    current_cause: Cause,
    drive: u16,
    paths: &mut Vec<CandidatePath>,
    connected_outcomes: &mut Vec<JunctionId>,
) {
    let surface = event.junction;
    let mut next = body
        .arena
        .junction(surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(first_id) = next {
        let first = *body.arena.link(first_id).expect("live link");
        next = first.next;
        let first_memory = &body.arrows[first_id.slot()];
        if !first_memory.is_entry()
            || !opens(first.trigger, event.before, event.after)
        {
            continue;
        }
        let mut second = body
            .arena
            .junction(first.to)
            .and_then(|junction| junction.outgoing_head);
        while let Some(second_id) = second {
            let link = *body.arena.link(second_id).expect("live link");
            second = link.next;
            let memory = &body.arrows[second_id.slot()];
            if memory.is_drive() && memory.factors().is_none() && link.impulse != 0 {
                let connected_start = connected_outcomes.len();
                append_connected_outcomes(body, first.to, link.to, connected_outcomes);
                let connected_end = connected_outcomes.len();
                let outcome = memory.outcome();
                let occurrence = memory.occurrence();
                paths.push(CandidatePath {
                    surface,
                    middle: first.to.into(),
                    output: link.to,
                    first: first_id.into(),
                    second: second_id.into(),
                    form: PathForm {
                        surface: body
                            .arena
                            .junction(surface)
                            .expect("current surface")
                            .checkpoint_law(),
                        first: LinkForm {
                            delay: first.delay,
                            impulse: first.impulse,
                            trigger: first.trigger,
                        },
                        second: LinkForm {
                            delay: link.delay,
                            impulse: link.impulse,
                            trigger: link.trigger,
                        },
                    },
                    at: event.at,
                    current_cause,
                    return_cause: occurrence.map(|occurrence| occurrence.cause),
                    unanswered: path_has_open_return(body, first.to, link.to),
                    connected_start,
                    connected_end,
                    outcome,
                    participation: memory.participation(),
                    participated_at: occurrence.map_or(0, |occurrence| occurrence.at),
                    output_participated: false,
                    outcome_source: unique_witness_source(
                        body,
                        link.to,
                        WitnessKind::Closure {
                            offers_choice: true,
                        },
                    ),
                    progress_source: unique_witness_source(
                        body,
                        link.to,
                        WitnessKind::Progress,
                    ),
                    resisted_progress: false,
                    boundary_open: memory.participation() > 0 && !memory.boundary_closed(),
                    boundary_inhibited: memory.boundary_inhibited(),
                    strength: memory.strength(),
                    drive,
                    stable_order: second_id.slot() as u32,
                    continuation: ContinuationResult::default(),
                    executable: path_is_executable(body, surface, outcome.is_some()),
                });
            }
        }
    }
}

fn path_entry_trigger(
    body: ReactionView<'_>,
    surface: JunctionId,
    before: Impulse,
    after: Impulse,
) -> Trigger {
    if !body.arena.junction(surface).is_some_and(|junction| {
        matches!(
            junction.checkpoint_law().retention,
            Retention::Sampled { .. }
        )
    }) {
        return Trigger::SourceFires;
    }
    if after > before {
        Trigger::Rises
    } else if after < before {
        Trigger::Falls
    } else {
        Trigger::SourceFires
    }
}

fn mark_reentries(
    body: ReactionView<'_>,
    facts: &[MomentFact],
    paths: &mut [CandidatePath],
    scratch: &mut ReentryScratch,
    change: &mut Change,
    construction: bool,
) {
    if construction {
        return;
    }
    scratch.present.extend(
        facts
            .iter()
            .filter(|fact| {
                fact.boundary && matches!(fact.used, UsedPaths::None) && !fact.had_ready_path
            })
            .map(|fact| fact.event.junction),
    );
    if scratch.present.is_empty() {
        return;
    }
    for candidate in paths.iter_mut().filter(|candidate| candidate.executable) {
        let (JunctionRef::Existing(middle), LinkRef::Existing(first), LinkRef::Existing(second)) =
            (candidate.middle, candidate.first, candidate.second)
        else {
            continue;
        };
        let path = Path {
            surface: candidate.surface,
            middle,
            output: candidate.output,
            first,
            second,
        };
        scratch.clear_search();
        let mut visits = 0;
        let mut shortcut_hits = 0;
        match find_reentries(
            body,
            path,
            &scratch.present,
            &mut scratch.steps,
            &mut scratch.continuations,
            &mut scratch.compilation,
            &mut visits,
            &mut shortcut_hits,
        ) {
            Ok(found) => {
                for rehearsal in &scratch.compilation.rehearsals {
                    change.rehearse_reentry(
                        rehearsal.start,
                        rehearsal.condition,
                        rehearsal.routes.clone(),
                        rehearsal.dependencies.clone(),
                    );
                }
                candidate.continuation.reentries = found;
            }
            Err(()) => candidate.continuation.reentry_failed = true,
        }
        candidate.continuation.reentry_incidence_visits = visits;
        candidate.continuation.reentry_shortcut_hits = shortcut_hits;
    }
}
