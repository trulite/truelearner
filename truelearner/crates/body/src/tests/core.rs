use super::*;
use crate::harness::{
    attach_boundary_component, attach_outcome_component, attach_sensor, finish, motor, reading,
    schedule,
};
use crate::{verify_choice_contract, Arrival};
use proptest::prelude::*;

fn participate_arrow(state: &mut ArrowState, times: u64, cause: Cause, at: Time) {
    for _ in 0..times {
        state.participate(Occurrence { cause, at });
    }
}

fn confirm_arrow(state: &mut ArrowState, times: u8) {
    for _ in 0..times {
        state.increment_exact_closures();
    }
}

fn close_arrow(
    body: &mut Body,
    returned: LinkId,
    path: Path,
    cause: Cause,
    source: JunctionId,
    witness: LinkId,
) {
    body.arrows[returned.slot()] = ArrowState::open_return(path, cause, 0);
    assert!(body.arrows[returned.slot()].close_return(0, ClosedSupport { source, witness }, None,));
}

fn candidate_path(drive: u16, stable_order: u32) -> CandidatePath {
    let surface = JunctionId::new(0).unwrap();
    let output = JunctionId::new(stable_order as usize + 1).unwrap();
    CandidatePath {
        surface,
        middle: JunctionId::new(stable_order as usize + 3).unwrap().into(),
        output,
        first: LinkId::new(stable_order as usize * 2).unwrap().into(),
        second: LinkId::new(stable_order as usize * 2 + 1).unwrap().into(),
        form: PathForm {
            surface: Junction::integrating(1),
            first: LinkForm {
                delay: 1,
                impulse: 1,
                trigger: Trigger::SourceFires,
            },
            second: LinkForm {
                delay: 1,
                impulse: 1,
                trigger: Trigger::SourceFires,
            },
        },
        at: 1,
        current_cause: 1,
        return_cause: None,
        unanswered: false,
        connected_start: 0,
        connected_end: 0,
        outcome: None,
        participation: 0,
        participated_at: 0,
        output_participated: false,
        outcome_source: None,
        progress_source: None,
        resisted_progress: false,
        boundary_open: false,
        boundary_inhibited: false,
        strength: 1,
        drive,
        stable_order,
        continuation: ContinuationResult::default(),
        executable: true,
    }
}

#[test]
fn current_normalized_drive_breaks_an_unlearned_choice_tie() {
    let paths = [candidate_path(512, 0), candidate_path(513, 1)];

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 1);
    assert_eq!(choice.warrant, ChoiceWarrant::LocalIncidence);
}

fn witnessed_reentry(path: &CandidatePath, condition: JunctionId) -> ReentryTrace {
    let (JunctionRef::Existing(middle), LinkRef::Existing(first), LinkRef::Existing(second)) =
        (path.middle, path.first, path.second)
    else {
        unreachable!()
    };
    ReentryTrace {
        condition,
        steps: vec![ReentryStepTrace {
            path: Path {
                surface: path.surface,
                middle,
                output: path.output,
                first,
                second,
            },
            returned_source: condition,
            outcome_witness: LinkId::new(20).unwrap(),
            outcome_target: path.output,
        }],
    }
}

fn closed_reentry_chain(length: usize, dormant_prefix: bool) -> (Body, Vec<Path>, JunctionId) {
    assert!(length > 0);
    let mut body = Body::default();
    if dormant_prefix {
        let from = body.add_junction(Junction::integrating(1)).unwrap();
        let to = body.add_junction(Junction::integrating(1)).unwrap();
        body.add_link(Link::new(from, to, 1, 1)).unwrap();
    }
    let surfaces = (0..=length)
        .map(|_| body.add_junction(Junction::integrating(1)).unwrap())
        .collect::<Vec<_>>();
    let mut paths = Vec::with_capacity(length);
    for index in 0..length {
        let middle = body.add_junction(Junction::integrating(1)).unwrap();
        let output = body.add_junction(Junction::integrating(1)).unwrap();
        let first = body
            .add_link(Link::new(surfaces[index], middle, 1, 1))
            .unwrap();
        body.mark_path_entry(first).unwrap();
        let second = body.add_link(Link::new(middle, output, 1, 1)).unwrap();
        participate_arrow(&mut body.arrows[first.slot()], 1, 1, 0);
        participate_arrow(&mut body.arrows[second.slot()], 1, 1, 0);
        let witness = body
            .add_link(Link::new(surfaces[index + 1], output, 0, 1))
            .unwrap();
        body.mark_witness(
            witness,
            WitnessKind::Closure {
                offers_choice: true,
            },
        )
        .unwrap();
        let returned = body.add_link(Link::new(output, middle, 0, 0)).unwrap();
        let path = Path {
            surface: surfaces[index],
            middle,
            output,
            first,
            second,
        };
        close_arrow(&mut body, returned, path, 1, surfaces[index + 1], witness);
        paths.push(path);
    }
    (body, paths, surfaces[length])
}

fn retain_confirmed_shortcuts(body: &mut Body, paths: &[Path]) -> Vec<LinkId> {
    let mut trace = NoTrace;
    for path in paths {
        confirm_arrow(
            &mut body.arrows[path.first.slot()],
            AUTOMATIC_AFTER_EXACT_CLOSURES,
        );
        confirm_arrow(
            &mut body.arrows[path.second.slot()],
            AUTOMATIC_AFTER_EXACT_CLOSURES,
        );
        retain_composite_direct(body, *path, 1, &mut trace);
    }
    paths
        .iter()
        .map(|path| {
            let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
            usable_composite(view, *path).expect("confirmed path has a valid shortcut")
        })
        .collect()
}

fn inspect_reentry(
    body: &Body,
    first: Path,
    present: JunctionId,
) -> (Result<Vec<ReentryTrace>, ()>, u16) {
    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    let mut steps = Vec::new();
    let mut continuations = Vec::new();
    let mut compilation = ReentryCompilationScratch::default();
    let mut incidence_visits = 0;
    let mut shortcut_hits = 0;
    let found = find_reentries(
        view,
        first,
        &[present],
        &mut steps,
        &mut continuations,
        &mut compilation,
        &mut incidence_visits,
        &mut shortcut_hits,
    );
    (found, incidence_visits)
}

fn append_closed_reentry_step(
    body: &mut Body,
    surface: JunctionId,
    returned_source: JunctionId,
) -> Path {
    let middle = body.add_junction(Junction::integrating(1)).unwrap();
    let output = body.add_junction(Junction::integrating(1)).unwrap();
    let first = body.add_link(Link::new(surface, middle, 1, 1)).unwrap();
    body.mark_path_entry(first).unwrap();
    let second = body.add_link(Link::new(middle, output, 1, 1)).unwrap();
    participate_arrow(&mut body.arrows[first.slot()], 1, 1, 0);
    participate_arrow(&mut body.arrows[second.slot()], 1, 1, 0);
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
    let returned = body.add_link(Link::new(output, middle, 0, 0)).unwrap();
    let path = Path {
        surface,
        middle,
        output,
        first,
        second,
    };
    close_arrow(body, returned, path, 1, returned_source, witness);
    path
}

fn append_unclosed_reentry_step(body: &mut Body, surface: JunctionId) -> Path {
    let middle = body.add_junction(Junction::integrating(1)).unwrap();
    let output = body.add_junction(Junction::integrating(1)).unwrap();
    let first = body.add_link(Link::new(surface, middle, 1, 1)).unwrap();
    body.mark_path_entry(first).unwrap();
    let second = body.add_link(Link::new(middle, output, 1, 1)).unwrap();
    participate_arrow(&mut body.arrows[first.slot()], 1, 1, 0);
    participate_arrow(&mut body.arrows[second.slot()], 1, 1, 0);
    Path {
        surface,
        middle,
        output,
        first,
        second,
    }
}

fn think_reentry(
    body: &mut Body,
    first: Path,
    present: JunctionId,
) -> (Result<Vec<ReentryTrace>, ()>, u16, u16) {
    let (found, rehearsals, visits, shortcut_hits) = {
        let view = ReactionView::with_reentry(
            &body.arena,
            &body.arrows,
            &body.returns,
            body.reentry.as_deref(),
        );
        let mut steps = Vec::new();
        let mut continuations = Vec::new();
        let mut compilation = ReentryCompilationScratch::default();
        let mut incidence_visits = 0;
        let mut shortcut_hits = 0;
        let found = find_reentries(
            view,
            first,
            &[present],
            &mut steps,
            &mut continuations,
            &mut compilation,
            &mut incidence_visits,
            &mut shortcut_hits,
        );
        (
            found,
            compilation.rehearsals,
            incidence_visits,
            shortcut_hits,
        )
    };
    if found.is_ok() {
        let mut change = Change::empty();
        for rehearsal in rehearsals {
            change.rehearse_reentry(
                rehearsal.start,
                rehearsal.condition,
                rehearsal.routes,
                rehearsal.dependencies,
            );
        }
        body.apply(change).unwrap();
    }
    (found, visits, shortcut_hits)
}

fn rehearse_through_reaction(
    body: &mut Body,
    first: Path,
    present: JunctionId,
) -> (Vec<ReentryTrace>, u16, u16) {
    let (routes, visits, shortcut_hits, change) = {
        let view = ReactionView::with_reentry(
            &body.arena,
            &body.arrows,
            &body.returns,
            body.reentry.as_deref(),
        );
        let event = crate::physics::Event {
            at: 7,
            junction: first.surface,
            arrivals: 1,
            impulse: 1,
            before: 0,
            after: 1,
            cause: 1,
        };
        let mut paths = Vec::new();
        let mut connected = Vec::new();
        append_existing_ready_paths(view, event, 1, 1, &mut paths, &mut connected);
        let facts = [MomentFact {
            event: crate::physics::Event {
                junction: present,
                ..event
            },
            drive: 1,
            boundary: true,
            used: UsedPaths::None,
            had_ready_path: false,
        }];
        let mut change = Change::empty();
        mark_reentries(
            view,
            &facts,
            &mut paths,
            &mut ReentryScratch::default(),
            &mut change,
            false,
        );
        let path = paths
            .into_iter()
            .find(|candidate| candidate.first == first.first.into())
            .unwrap();
        (
            path.continuation.reentries,
            path.continuation.reentry_incidence_visits,
            path.continuation.reentry_shortcut_hits,
            change,
        )
    };
    body.apply(change).unwrap();
    (routes, visits, shortcut_hits)
}

#[test]
fn actual_current_return_precedes_unique_reentry() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    paths[0].continuation.reentries =
        vec![witnessed_reentry(&paths[0], JunctionId::new(20).unwrap())];
    paths[1].return_cause = Some(paths[1].current_cause);

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 1);
    assert_eq!(choice.warrant, ChoiceWarrant::ReturnedConsequence);
}

#[test]
fn actual_retained_progress_precedes_unique_reentry() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    paths[0].continuation.reentries =
        vec![witnessed_reentry(&paths[0], JunctionId::new(20).unwrap())];
    paths[1].resisted_progress = true;
    paths[1].boundary_open = true;
    paths[1].strength = 2;
    paths[1].participation = 1;

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 1);
    assert_eq!(choice.warrant, ChoiceWarrant::RetainedContinuation);
}

#[test]
fn one_local_resolver_orders_fresh_opportunity_without_a_second_choice() {
    let paths = [candidate_path(512, 0), candidate_path(512, 1)];
    let evidence = FreshOpportunityTrace {
        source: paths[0].surface,
        output: paths[0].output,
        through: LinkId::new(30).unwrap(),
    };
    let fresh = choose_ready(&paths, &[0, 0], 0, false, || Some((0, evidence))).unwrap();
    assert_eq!(fresh.winner, 0);
    assert_eq!(fresh.warrant, ChoiceWarrant::RetainedContinuation);
    assert_eq!(fresh.fresh_through, Some(evidence.through));

    let mut current = paths.clone();
    current[1].return_cause = Some(current[1].current_cause);
    let current = choose_ready(&current, &[0, 0], 0, false, || Some((0, evidence))).unwrap();
    assert_eq!(current.winner, 1);
    assert_eq!(current.warrant, ChoiceWarrant::ReturnedConsequence);
    assert_eq!(current.fresh_through, None);

    let mut progressing = paths.clone();
    progressing[1].resisted_progress = true;
    progressing[1].boundary_open = true;
    progressing[1].strength = 2;
    progressing[1].participation = 1;
    let progressing =
        choose_ready(&progressing, &[0, 0], 0, false, || Some((0, evidence))).unwrap();
    assert_eq!(progressing.winner, 1);
    assert_eq!(progressing.warrant, ChoiceWarrant::RetainedContinuation);
    assert_eq!(progressing.fresh_through, None);

    let mut reentering = paths;
    for path in &mut reentering {
        path.participation = 1;
    }
    reentering[1].continuation.reentries = vec![witnessed_reentry(
        &reentering[1],
        JunctionId::new(20).unwrap(),
    )];
    let reentering = choose_ready(&reentering, &[0, 0], 0, false, || Some((0, evidence))).unwrap();
    assert_eq!(reentering.winner, 1);
    assert_eq!(reentering.warrant, ChoiceWarrant::Reentry);
    assert_eq!(reentering.fresh_through, None);
}

#[test]
fn current_choice_surface_is_not_a_present_reentry_condition() {
    let mut body = Body::default();
    let surface = body.add_junction(Junction::integrating(1)).unwrap();
    let middle = body.add_junction(Junction::integrating(1)).unwrap();
    let output = body.add_junction(Junction::integrating(1)).unwrap();
    let first = body.add_link(Link::new(surface, middle, 1, 1)).unwrap();
    body.mark_path_entry(first).unwrap();
    let second = body.add_link(Link::new(middle, output, 1, 1)).unwrap();
    participate_arrow(&mut body.arrows[first.slot()], 1, 1, 0);
    participate_arrow(&mut body.arrows[second.slot()], 1, 1, 0);
    let witness = body.add_link(Link::new(surface, output, 0, 1)).unwrap();
    body.mark_witness(
        witness,
        WitnessKind::Closure {
            offers_choice: true,
        },
    )
    .unwrap();
    let returned = body.add_link(Link::new(output, middle, 0, 0)).unwrap();
    close_arrow(
        &mut body,
        returned,
        Path {
            surface,
            middle,
            output,
            first,
            second,
        },
        1,
        surface,
        witness,
    );
    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    let mut paths = Vec::new();
    let mut connected = Vec::new();
    let event = crate::physics::Event {
        at: 7,
        junction: surface,
        arrivals: 1,
        impulse: 1,
        before: 0,
        after: 1,
        cause: 1,
    };
    append_existing_ready_paths(view, event, 1, 1, &mut paths, &mut connected);
    let facts = [MomentFact {
        event,
        drive: 1,
        boundary: true,
        used: UsedPaths::None,
        had_ready_path: true,
    }];

    mark_reentries(
        view,
        &facts,
        &mut paths,
        &mut ReentryScratch::default(),
        &mut Change::empty(),
        false,
    );

    assert!(paths
        .iter()
        .all(|path| path.continuation.reentries.is_empty()));
}

#[test]
fn cyclic_closed_steps_fail_reentry_closed() {
    let mut body = Body::default();
    let surfaces = [
        body.add_junction(Junction::integrating(1)).unwrap(),
        body.add_junction(Junction::integrating(1)).unwrap(),
    ];
    let middles = [
        body.add_junction(Junction::integrating(1)).unwrap(),
        body.add_junction(Junction::integrating(1)).unwrap(),
    ];
    let outputs = [
        body.add_junction(Junction::integrating(1)).unwrap(),
        body.add_junction(Junction::integrating(1)).unwrap(),
    ];
    let mut paths = Vec::new();
    for index in 0..2 {
        let first = body
            .add_link(Link::new(surfaces[index], middles[index], 1, 1))
            .unwrap();
        body.mark_path_entry(first).unwrap();
        let second = body
            .add_link(Link::new(middles[index], outputs[index], 1, 1))
            .unwrap();
        participate_arrow(&mut body.arrows[first.slot()], 1, 1, 0);
        participate_arrow(&mut body.arrows[second.slot()], 1, 1, 0);
        let witness = body
            .add_link(Link::new(surfaces[1 - index], outputs[index], 0, 1))
            .unwrap();
        body.mark_witness(
            witness,
            WitnessKind::Closure {
                offers_choice: true,
            },
        )
        .unwrap();
        let returned = body
            .add_link(Link::new(outputs[index], middles[index], 0, 0))
            .unwrap();
        let path = Path {
            surface: surfaces[index],
            middle: middles[index],
            output: outputs[index],
            first,
            second,
        };
        close_arrow(&mut body, returned, path, 1, surfaces[1 - index], witness);
        paths.push(path);
    }
    let present = body.add_junction(Junction::integrating(1)).unwrap();
    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);

    let mut steps = Vec::new();
    let mut continuations = Vec::new();
    let mut compilation = ReentryCompilationScratch::default();
    let mut incidence_visits = 0;
    let mut shortcut_hits = 0;
    assert_eq!(
        find_reentries(
            view,
            paths[0],
            &[present],
            &mut steps,
            &mut continuations,
            &mut compilation,
            &mut incidence_visits,
            &mut shortcut_hits,
        ),
        Err(())
    );
}

#[test]
fn confirmed_shortcuts_extend_foresight_without_a_larger_depth_lifetime() {
    let (mut body, paths, present) = closed_reentry_chain(MAX_REENTRY_DEPTH + 1, false);
    assert_eq!(inspect_reentry(&body, paths[0], present).0, Err(()));

    let shortcuts = retain_confirmed_shortcuts(&mut body, &paths);
    let (found, _) = inspect_reentry(&body, paths[0], present);

    let found = found.expect("confirmed compression extends the same wave");
    let [reentry] = found.as_slice() else {
        panic!("one compressed route should reach the present condition");
    };
    assert_eq!(reentry.condition, present);
    assert_eq!(reentry.steps.len(), MAX_REENTRY_DEPTH + 1);
    assert_eq!(shortcuts.len(), paths.len());
}

#[test]
fn one_confirmed_shortcut_extends_foresight_by_one_physical_step() {
    let (mut body, paths, present) = closed_reentry_chain(MAX_REENTRY_DEPTH + 1, false);
    assert_eq!(inspect_reentry(&body, paths[0], present).0, Err(()));

    retain_confirmed_shortcuts(&mut body, &paths[MAX_REENTRY_DEPTH..]);

    assert!(inspect_reentry(&body, paths[0], present)
        .0
        .is_ok_and(|found| found.len() == 1));
}

#[test]
fn unconfirmed_inspection_never_creates_its_own_foresight() {
    let (body, paths, present) = closed_reentry_chain(MAX_REENTRY_DEPTH + 1, false);
    let before = body.checkpoint().unwrap().canonical_bytes().unwrap();

    for _ in 0..3 {
        assert_eq!(inspect_reentry(&body, paths[0], present).0, Err(()));
    }

    assert_eq!(
        body.checkpoint().unwrap().canonical_bytes().unwrap(),
        before
    );
    assert!(body.arrows.iter().all(|memory| memory.factors().is_none()));
}

#[test]
fn repeated_thought_permanently_compiles_without_inventing_causal_evidence() {
    let (mut body, paths, present) = closed_reentry_chain(6, false);
    let closed_before = body.reentry_state().closed_steps;
    let link_count_before = body.arena.link_count();
    let strengths_before = body
        .arrows
        .iter()
        .map(ArrowState::strength)
        .collect::<Vec<_>>();

    let (expected, full_visits, first_hits) = think_reentry(&mut body, paths[0], present);
    assert_eq!(first_hits, 0);
    assert_eq!(body.reentry_state().thought_shortcuts, 0);
    let (_, second_visits, second_hits) = think_reentry(&mut body, paths[0], present);
    assert_eq!((second_visits, second_hits), (full_visits, 0));
    assert_eq!(body.reentry_state().thought_shortcuts, 0);
    let (_, third_visits, third_hits) = think_reentry(&mut body, paths[0], present);
    assert_eq!((third_visits, third_hits), (full_visits, 0));
    assert_eq!(body.reentry_state().thought_shortcuts, paths.len());

    let (compiled, compiled_visits, compiled_hits) = think_reentry(&mut body, paths[0], present);
    assert_eq!(compiled, expected);
    assert_eq!(compiled_hits, 1);
    assert!(compiled_visits < full_visits);
    assert_eq!(body.reentry_state().closed_steps, closed_before);
    assert_eq!(body.arena.link_count(), link_count_before);
    assert_eq!(
        body.arrows
            .iter()
            .map(ArrowState::strength)
            .collect::<Vec<_>>(),
        strengths_before
    );
}

#[test]
fn ordinary_reaction_compiles_the_same_repeated_internal_path() {
    let (mut body, paths, present) = closed_reentry_chain(4, false);
    let first = rehearse_through_reaction(&mut body, paths[0], present);
    assert_eq!(first.0.len(), 1);
    assert_eq!(first.2, 0);
    assert_eq!(rehearse_through_reaction(&mut body, paths[0], present).2, 0);
    assert_eq!(rehearse_through_reaction(&mut body, paths[0], present).2, 0);

    let compiled = rehearse_through_reaction(&mut body, paths[0], present);

    assert_eq!(compiled.0, first.0);
    assert!(compiled.1 < first.1);
    assert_eq!(compiled.2, 1);
}

#[test]
fn different_beginnings_compile_and_reuse_their_shared_causal_tail() {
    let (mut body, tail, present) = closed_reentry_chain(3, false);
    let starts = (0..3)
        .map(|_| body.add_junction(Junction::integrating(1)).unwrap())
        .collect::<Vec<_>>();
    let prefixes = starts
        .iter()
        .map(|surface| append_closed_reentry_step(&mut body, *surface, tail[0].surface))
        .collect::<Vec<_>>();
    let (expected, full_visits) = inspect_reentry(&body, prefixes[2], present);

    assert_eq!(think_reentry(&mut body, prefixes[0], present).2, 0);
    assert_eq!(think_reentry(&mut body, prefixes[1], present).2, 0);
    assert_eq!(think_reentry(&mut body, prefixes[1], present).2, 0);
    assert_eq!(body.reentry_state().thought_shortcuts, tail.len());

    let (reused, visits, shortcut_hits) = think_reentry(&mut body, prefixes[2], present);

    assert_eq!(reused, expected);
    assert_eq!(shortcut_hits, 1);
    assert!(visits < full_visits);
    assert_eq!(reused.unwrap()[0].steps.len(), tail.len() + 1);
}

#[test]
fn a_compiled_tail_never_invents_a_missing_beginning() {
    let (mut body, tail, present) = closed_reentry_chain(3, false);
    let first = body.add_junction(Junction::integrating(1)).unwrap();
    let second = body.add_junction(Junction::integrating(1)).unwrap();
    let missing = body.add_junction(Junction::integrating(1)).unwrap();
    let first = append_closed_reentry_step(&mut body, first, tail[0].surface);
    let second = append_closed_reentry_step(&mut body, second, tail[0].surface);
    let missing = append_unclosed_reentry_step(&mut body, missing);
    assert_eq!(think_reentry(&mut body, first, present).2, 0);
    assert_eq!(think_reentry(&mut body, second, present).2, 0);
    assert_eq!(think_reentry(&mut body, second, present).2, 0);

    let (found, _, shortcut_hits) = think_reentry(&mut body, missing, present);

    assert!(found.unwrap().is_empty());
    assert_eq!(shortcut_hits, 0);
}

#[test]
fn changing_one_beginning_does_not_invalidate_its_shared_tail() {
    let (mut body, tail, present) = closed_reentry_chain(3, false);
    let starts = (0..3)
        .map(|_| body.add_junction(Junction::integrating(1)).unwrap())
        .collect::<Vec<_>>();
    let prefixes = starts
        .iter()
        .map(|surface| append_closed_reentry_step(&mut body, *surface, tail[0].surface))
        .collect::<Vec<_>>();
    assert_eq!(think_reentry(&mut body, prefixes[0], present).2, 0);
    assert_eq!(think_reentry(&mut body, prefixes[1], present).2, 0);
    assert_eq!(think_reentry(&mut body, prefixes[1], present).2, 0);

    body.set_link_impulse(prefixes[0].second, 2).unwrap();
    let (_, _, shortcut_hits) = think_reentry(&mut body, prefixes[2], present);

    assert_eq!(shortcut_hits, 1);
}

#[test]
fn compiled_thought_survives_checkpoint_restore() {
    let (mut body, paths, present) = closed_reentry_chain(5, false);
    for _ in 0..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
        assert!(think_reentry(&mut body, paths[0], present).0.is_ok());
    }
    let bytes = body.checkpoint().unwrap().canonical_bytes().unwrap();
    let mut restored = crate::BodyCheckpoint::decode(&bytes)
        .unwrap()
        .restore()
        .unwrap();

    let (_, visits, shortcut_hits) = think_reentry(&mut restored, paths[0], present);

    assert_eq!(shortcut_hits, 1);
    assert_eq!(visits, 0);
    assert_eq!(restored.reentry_state().thought_shortcuts, paths.len());
}

#[test]
fn a_new_possible_branch_invalidates_compiled_thought_and_preserves_ambiguity() {
    let (mut body, paths, present) = closed_reentry_chain(4, false);
    for _ in 0..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
        assert_eq!(
            think_reentry(&mut body, paths[0], present).0.unwrap().len(),
            1
        );
    }
    assert_eq!(think_reentry(&mut body, paths[0], present).2, 1);

    append_closed_reentry_step(&mut body, paths[1].surface, present);
    let (found, _, shortcut_hits) = think_reentry(&mut body, paths[0], present);

    assert_eq!(shortcut_hits, 0);
    assert_eq!(found.unwrap().len(), 2);
}

#[test]
fn changed_causal_support_invalidates_only_the_dependent_compiled_thought() {
    let (mut body, paths, present) = closed_reentry_chain(4, false);
    for _ in 0..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
        assert!(think_reentry(&mut body, paths[0], present).0.is_ok());
    }
    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    let witness = closed_steps(view)
        .find(|step| step.path == paths[2])
        .unwrap()
        .outcome_witness;

    body.mark_witness(
        witness,
        WitnessKind::Closure {
            offers_choice: false,
        },
    )
    .unwrap();
    let (found, _, shortcut_hits) = think_reentry(&mut body, paths[0], present);

    assert_eq!(shortcut_hits, 0);
    assert!(found.unwrap().is_empty());
    assert_eq!(body.reentry_state().thought_shortcuts, 1);
    assert_eq!(think_reentry(&mut body, paths[3], present).2, 1);
}

#[test]
fn changed_membership_invalidates_the_path_whose_executability_it_changes() {
    let (mut body, paths, present) = closed_reentry_chain(4, false);
    for _ in 0..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
        assert!(think_reentry(&mut body, paths[0], present).0.is_ok());
    }
    assert_eq!(think_reentry(&mut body, paths[0], present).2, 1);

    let parent = body.add_junction(Junction::integrating(1)).unwrap();
    let membership = body
        .add_link(Link::new(parent, paths[0].surface, 0, 1))
        .unwrap();
    body.replace_arrow_state(membership, ArrowState::membership())
        .unwrap();
    assert_eq!(body.reentry_state().thought_shortcuts, paths.len() - 1);
    let (found, visits, shortcut_hits) = think_reentry(&mut body, paths[0], present);

    assert!(visits > 0);
    assert_eq!(shortcut_hits, 1);
    assert_eq!(found.unwrap().len(), 1);
    for _ in 1..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
        assert!(think_reentry(&mut body, paths[0], present).0.is_ok());
    }
    assert_eq!(think_reentry(&mut body, paths[0], present).2, 1);

    let mut change = Change::empty();
    change.change_link(membership.into(), LinkChange::Retire);
    body.apply(change).unwrap();
    assert_eq!(body.reentry_state().thought_shortcuts, paths.len() - 1);
    let (found, visits, shortcut_hits) = think_reentry(&mut body, paths[0], present);

    assert!(visits > 0);
    assert_eq!(shortcut_hits, 1);
    assert_eq!(found.unwrap().len(), 1);
}

#[test]
fn repeated_real_confirmation_of_the_same_support_keeps_compiled_thought() {
    let (mut body, paths, present) = closed_reentry_chain(4, false);
    for _ in 0..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
        assert!(think_reentry(&mut body, paths[0], present).0.is_ok());
    }
    let confirmed = {
        let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
        closed_steps(view)
            .find(|step| step.path == paths[1])
            .unwrap()
    };
    let epochs_before = body.reentry.as_ref().unwrap().epochs.clone();
    let returned = body
        .add_link_untracked(Link::new(paths[1].output, paths[1].middle, 0, 0))
        .unwrap();
    body.arrows[returned.slot()] = ArrowState::open_return(paths[1], 9, 0);
    assert_eq!(body.reentry.as_ref().unwrap().epochs, epochs_before);

    let support = body.retained_closed_support(
        returned,
        confirmed.returned_source,
        confirmed.path,
        Some(confirmed.outcome_witness),
        true,
        true,
    );
    body.arrows[returned.slot()].close_return(1, support.unwrap(), None);
    assert_eq!(body.reentry.as_ref().unwrap().epochs, epochs_before);
    let (found, _, shortcut_hits) = think_reentry(&mut body, paths[0], present);

    assert_eq!(shortcut_hits, 1);
    assert_eq!(found.unwrap().len(), 1);
    assert_eq!(body.reentry_state().thought_shortcuts, paths.len());
}

#[test]
fn compiled_ambiguous_thought_never_becomes_a_unique_future() {
    let (mut body, paths, present) = closed_reentry_chain(4, false);
    append_closed_reentry_step(&mut body, paths[1].surface, present);
    for _ in 0..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
        assert_eq!(
            think_reentry(&mut body, paths[0], present).0.unwrap().len(),
            2
        );
    }

    let (found, _, shortcut_hits) = think_reentry(&mut body, paths[0], present);

    assert_eq!(shortcut_hits, 1);
    assert_eq!(found.unwrap().len(), 2);
}

#[test]
fn disconnected_change_does_not_invalidate_compiled_thought() {
    let (mut body, paths, present) = closed_reentry_chain(4, false);
    for _ in 0..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
        assert!(think_reentry(&mut body, paths[0], present).0.is_ok());
    }
    let from = body.add_junction(Junction::integrating(1)).unwrap();
    let to = body.add_junction(Junction::integrating(1)).unwrap();
    body.add_link(Link::new(from, to, 1, 1)).unwrap();

    let (_, _, shortcut_hits) = think_reentry(&mut body, paths[0], present);

    assert_eq!(shortcut_hits, 1);
}

#[test]
fn attached_compiled_thought_keeps_its_remapped_physical_dependencies() {
    let (mut part, paths, present) = closed_reentry_chain(4, false);
    for _ in 0..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
        assert!(think_reentry(&mut part, paths[0], present).0.is_ok());
    }
    let mut host = Body::default();
    let dormant_from = host.add_junction(Junction::integrating(1)).unwrap();
    let dormant_to = host.add_junction(Junction::integrating(1)).unwrap();
    host.add_link(Link::new(dormant_from, dormant_to, 1, 1))
        .unwrap();
    let junction_base = host.arena.junction_count();
    let link_base = host.arena.link_count();
    let mut remapped_start = paths[0];
    remap_path(&mut remapped_start, junction_base, link_base);
    let remapped_present = remap_junction(present, junction_base);
    let part = crate::OpenBody::new(part, vec![present]).unwrap();

    crate::attach(&mut host, part, &[]).unwrap();
    let (_, visits, shortcut_hits) = think_reentry(&mut host, remapped_start, remapped_present);

    assert_eq!(shortcut_hits, 1);
    assert_eq!(visits, 0);
    assert_eq!(host.reentry_state().thought_shortcuts, paths.len());
}

#[test]
fn disconnected_compiled_thoughts_do_not_change_local_receipt_or_graph_work() {
    let (mut host, paths, present) = closed_reentry_chain(4, false);
    for _ in 0..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
        assert!(think_reentry(&mut host, paths[0], present).0.is_ok());
    }
    let expected = think_reentry(&mut host, paths[0], present);
    for _ in 0..12 {
        let (mut part, part_paths, part_present) = closed_reentry_chain(4, false);
        for _ in 0..THOUGHT_SHORTCUT_AFTER_REHEARSALS {
            assert!(think_reentry(&mut part, part_paths[0], part_present)
                .0
                .is_ok());
        }
        let part = crate::OpenBody::new(part, vec![part_present]).unwrap();
        crate::attach(&mut host, part, &[]).unwrap();
    }

    let observed = think_reentry(&mut host, paths[0], present);

    assert_eq!(observed, expected);
}

#[test]
fn changing_one_confirmed_parent_falls_back_to_detailed_foresight() {
    let (mut body, paths, present) = closed_reentry_chain(MAX_REENTRY_DEPTH + 1, false);
    let shortcuts = retain_confirmed_shortcuts(&mut body, &paths);
    assert!(inspect_reentry(&body, paths[0], present)
        .0
        .is_ok_and(|found| found.len() == 1));

    let changed = MAX_REENTRY_DEPTH / 2;
    body.set_link_impulse(shortcuts[changed], 0).unwrap();

    assert!(inspect_reentry(&body, paths[0], present)
        .0
        .is_ok_and(|found| found.len() == 1));
    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    assert!(usable_composite(view, paths[changed]).is_none());
}

#[test]
fn changing_an_intermediate_consequence_stops_compressed_foresight() {
    let (mut body, paths, present) = closed_reentry_chain(MAX_REENTRY_DEPTH + 1, false);
    retain_confirmed_shortcuts(&mut body, &paths);
    let changed = MAX_REENTRY_DEPTH / 2;
    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    let witness = closed_steps(view)
        .find(|step| step.path == paths[changed])
        .expect("changed step has exact retained support")
        .outcome_witness;

    body.mark_witness(
        witness,
        WitnessKind::Closure {
            offers_choice: false,
        },
    )
    .unwrap();

    assert!(inspect_reentry(&body, paths[0], present)
        .0
        .is_ok_and(|found| found.is_empty()));
}

#[test]
fn compressed_foresight_ignores_identity_and_disconnected_construction() {
    let episode = |dormant_prefix| {
        let (mut body, paths, present) =
            closed_reentry_chain(MAX_REENTRY_DEPTH + 1, dormant_prefix);
        retain_confirmed_shortcuts(&mut body, &paths);
        let (found, visits) = inspect_reentry(&body, paths[0], present);
        (
            found
                .expect("renamed compressed route remains inspectable")
                .into_iter()
                .map(|reentry| reentry.steps.len())
                .collect::<Vec<_>>(),
            visits,
        )
    };

    assert_eq!(episode(false), episode(true));
}

#[test]
fn confirmed_compression_never_removes_the_incidence_safety_ceiling() {
    let (mut body, paths, present) =
        closed_reentry_chain(MAX_REENTRY_INCIDENCE_VISITS as usize, false);
    retain_confirmed_shortcuts(&mut body, &paths);

    let (found, visits) = inspect_reentry(&body, paths[0], present);

    assert_eq!(found, Err(()));
    assert_eq!(visits, MAX_REENTRY_INCIDENCE_VISITS);
}

#[test]
fn old_outcome_cannot_override_a_stronger_current_surface() {
    let mut paths = [candidate_path(44, 0), candidate_path(1_023, 1)];
    paths[0].outcome = Some(Outcome {
        at: 1,
        caused_transition: true,
        available_until_choice: true,
    });

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 1);
}

#[test]
fn equal_current_drive_preserves_physical_stable_order() {
    let paths = [candidate_path(512, 0), candidate_path(512, 1)];

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 0);
}

#[test]
fn a_participating_output_continues_its_current_return_after_both_outputs_were_tried() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    paths[0].output_participated = true;
    paths[1].participation = 1;

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 0);
    assert_eq!(choice.warrant, ChoiceWarrant::ReturnedConsequence);
}

#[test]
fn one_retained_progressing_output_precedes_untried_release() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    paths[0].resisted_progress = true;
    paths[0].boundary_open = true;
    paths[0].participation = 1;
    paths[0].strength = 2;

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 0);
    assert_eq!(choice.warrant, ChoiceWarrant::RetainedContinuation);
}

#[test]
fn unanswered_without_fresh_progress_releases_to_the_untried_output() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    paths[0].unanswered = true;
    paths[0].participation = 1;

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 1);
    assert_eq!(choice.warrant, ChoiceWarrant::Exploration);
}

#[test]
fn a_newer_unanswered_reuse_releases_an_older_success_to_a_tried_alternative() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    paths[0].participation = 2;
    paths[0].participated_at = 20;
    paths[0].unanswered = true;
    paths[0].outcome = Some(Outcome {
        at: 10,
        caused_transition: true,
        available_until_choice: false,
    });
    paths[0].strength = 3;
    paths[1].participation = 1;
    paths[1].participated_at = 15;

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 1);
    assert_eq!(choice.warrant, ChoiceWarrant::Exploration);
}

#[test]
fn an_exact_return_precedes_unanswered_output_release() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    paths[0].participation = 2;
    paths[0].participated_at = 20;
    paths[0].unanswered = true;
    paths[0].return_cause = Some(paths[0].current_cause);
    paths[0].outcome = Some(Outcome {
        at: 10,
        caused_transition: true,
        available_until_choice: false,
    });
    paths[1].participation = 1;
    paths[1].participated_at = 15;

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 0);
    assert_eq!(choice.warrant, ChoiceWarrant::ReturnedConsequence);
}

#[test]
fn a_lone_unanswered_output_does_not_invent_an_alternative() {
    let mut path = candidate_path(512, 0);
    path.participation = 2;
    path.participated_at = 20;
    path.unanswered = true;
    path.outcome = Some(Outcome {
        at: 10,
        caused_transition: true,
        available_until_choice: false,
    });

    let choice = choose_ready(&[path], &[0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 0);
    assert_eq!(choice.warrant, ChoiceWarrant::LocalIncidence);
}

#[test]
fn simultaneous_unanswered_outputs_make_no_unique_release_claim() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    for path in &mut paths {
        path.participation = 1;
        path.participated_at = 20;
        path.unanswered = true;
    }

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 0);
    assert_eq!(choice.warrant, ChoiceWarrant::LocalIncidence);
}

#[test]
fn unclosed_exploration_cannot_claim_continuation() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    paths[0].unanswered = true;
    paths[0].resisted_progress = true;
    paths[0].boundary_open = true;
    paths[0].participation = 1;

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 1);
    assert_eq!(choice.warrant, ChoiceWarrant::Exploration);
}

#[test]
fn retained_output_with_fresh_progress_continues_after_ordinary_outcome() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    paths[0].resisted_progress = true;
    paths[0].boundary_open = true;
    paths[0].participation = 1;
    paths[0].strength = 2;

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 0);
    assert_eq!(choice.warrant, ChoiceWarrant::RetainedContinuation);
}

#[test]
fn several_progressing_outputs_receive_no_continuation_precedence() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    for path in &mut paths {
        path.unanswered = true;
        path.resisted_progress = true;
        path.boundary_open = true;
        path.participation = 1;
        path.strength = 2;
    }

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_ne!(choice.warrant, ChoiceWarrant::RetainedContinuation);
}

#[test]
fn boundary_closed_output_cannot_claim_progress_continuation() {
    let mut paths = [candidate_path(512, 0), candidate_path(512, 1)];
    paths[0].resisted_progress = true;
    paths[0].participation = 1;
    paths[0].strength = 2;

    let choice = choose_ready(&paths, &[0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 1);
    assert_ne!(choice.warrant, ChoiceWarrant::RetainedContinuation);
}

#[test]
fn boundary_completion_releases_only_the_local_antagonist() {
    let mut paths = [
        candidate_path(512, 0),
        candidate_path(512, 1),
        candidate_path(900, 2),
    ];
    let local = JunctionId::new(20).unwrap();
    paths[0].outcome_source = Some(local);
    paths[0].boundary_inhibited = true;
    paths[1].outcome_source = Some(local);
    paths[2].outcome_source = Some(JunctionId::new(21).unwrap());
    paths[2].participation = 100;
    paths[2].strength = 100;

    let choice = choose_ready(&paths, &[0, 0, 0], 0, false, || None).unwrap();

    assert_eq!(choice.winner, 1);
    assert_eq!(choice.warrant, ChoiceWarrant::ReturnedConsequence);
}

#[test]
fn simultaneous_boundary_components_make_no_local_release_claim() {
    let mut paths = [
        candidate_path(512, 0),
        candidate_path(512, 1),
        candidate_path(512, 2),
    ];
    paths[0].outcome_source = Some(JunctionId::new(20).unwrap());
    paths[0].boundary_inhibited = true;
    paths[1].outcome_source = Some(JunctionId::new(21).unwrap());
    paths[1].boundary_inhibited = true;
    paths[2].outcome_source = Some(JunctionId::new(20).unwrap());

    assert!(choose_ready(&paths, &[0, 0, 0], 0, false, || None).is_none());
}

#[test]
fn a_later_path_action_supersedes_a_stale_fresh_witness() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(1)).unwrap();
    let middle = body.add_junction(Junction::integrating(1)).unwrap();
    let outputs: [JunctionId; 2] =
        std::array::from_fn(|_| body.add_junction(Junction::integrating(1)).unwrap());
    let witnesses = outputs.map(|output| {
        let witness = body.add_link(Link::new(source, output, 0, 1)).unwrap();
        body.mark_witness(
            witness,
            WitnessKind::Closure {
                offers_choice: true,
            },
        )
        .unwrap();
        witness
    });
    let drive = body.add_link(Link::new(middle, outputs[0], 1, 1)).unwrap();
    participate_arrow(&mut body.arrows[drive.slot()], 1, 1, 10);
    body.arrows[witnesses[1].slot()].record_transmission(Occurrence { cause: 2, at: 20 });

    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    assert_eq!(latest_fresh_output(view, source), Some(outputs[1]));

    participate_arrow(&mut body.arrows[drive.slot()], 1, 1, 21);
    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    assert_eq!(latest_fresh_output(view, source), None);
}

#[test]
fn simultaneous_fresh_outputs_claim_no_single_return() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(1)).unwrap();
    let outputs: [JunctionId; 2] =
        std::array::from_fn(|_| body.add_junction(Junction::integrating(1)).unwrap());
    for output in outputs {
        let witness = body.add_link(Link::new(source, output, 0, 1)).unwrap();
        body.mark_witness(
            witness,
            WitnessKind::Closure {
                offers_choice: true,
            },
        )
        .unwrap();
        body.arrows[witness.slot()].record_transmission(Occurrence { cause: 1, at: 20 });
    }

    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    assert_eq!(latest_fresh_output(view, source), None);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn every_ready_choice_satisfies_the_offline_laws(
        left_drive in 0_u16..=1_023,
        right_drive in 0_u16..=1_023,
        left_participation in 0_u64..4,
        right_participation in 0_u64..4,
        left_participated_at in 0_u64..4,
        right_participated_at in 0_u64..4,
        left_unanswered in any::<bool>(),
        right_unanswered in any::<bool>(),
        left_output_participated in any::<bool>(),
        right_output_participated in any::<bool>(),
        left_strength in -2_i64..=2,
        right_strength in -2_i64..=2,
        left_outcome_at in prop::option::of(0_u64..4),
        right_outcome_at in prop::option::of(0_u64..4),
        left_available in any::<bool>(),
        right_available in any::<bool>(),
        left_returns in any::<bool>(),
        right_returns in any::<bool>(),
        left_executable in any::<bool>(),
        right_executable in any::<bool>(),
        construction in any::<bool>(),
    ) {
        let mut paths = [candidate_path(left_drive, 0), candidate_path(right_drive, 1)];
        paths[0].participation = left_participation;
        paths[1].participation = right_participation;
        paths[0].participated_at = left_participated_at;
        paths[1].participated_at = right_participated_at;
        paths[0].unanswered = left_unanswered;
        paths[1].unanswered = right_unanswered;
        paths[0].output_participated = left_output_participated;
        paths[1].output_participated = right_output_participated;
        paths[0].strength = left_strength;
        paths[1].strength = right_strength;
        paths[0].outcome = left_outcome_at.map(|at| Outcome {
            at,
            caused_transition: true,
            available_until_choice: left_available,
        });
        paths[1].outcome = right_outcome_at.map(|at| Outcome {
            at,
            caused_transition: true,
            available_until_choice: right_available,
        });
        paths[0].return_cause = left_returns.then_some(paths[0].current_cause);
        paths[1].return_cause = right_returns.then_some(paths[1].current_cause);
        paths[0].executable = left_executable;
        paths[1].executable = right_executable;

        let selected = choose_ready(&paths, &[0, 0], 0, construction, || None);
        let candidates = paths.iter().map(|path| CandidateTrace {
            at: path.at,
            cause: path.current_cause,
            group: 0,
            path: path.trace_path(),
            connected_outcomes: Vec::new(),
            executable: path.executable,
            return_cause: path.return_cause,
            unanswered: path.unanswered,
            outcome: path.outcome,
            participation: path.participation,
            participated_at: path.participated_at,
            output_participated: path.output_participated,
            outcome_source: path.outcome_source,
            progress_source: path.progress_source,
            resisted_progress: path.resisted_progress,
            boundary_open: path.boundary_open,
            boundary_inhibited: path.boundary_inhibited,
            strength: path.strength,
            drive: path.drive,
            stable_order: path.stable_order,
            fresh_opportunity: None,
            present_sources: Vec::new(),
            reentries: path.continuation.reentries.clone(),
            motif_reentries: path.continuation.motif_reentries.clone(),
            motif_routes: path
                .continuation.motif_routes
                .as_deref()
                .unwrap_or_default()
                .to_vec(),
            reentry_incidence_visits: path.continuation.reentry_incidence_visits,
            reentry_shortcut_hits: path.continuation.reentry_shortcut_hits,
            reentry_failed: path.continuation.reentry_failed,
            motif_route_failed: path.continuation.motif_route_failed,
            new_path: false,
        });
        let mut events = candidates.map(TraceEvent::Candidate).collect::<Vec<_>>();
        events.push(TraceEvent::Choice(ChoiceTrace {
            at: 1,
            group: 0,
            alternatives: paths.len(),
            winner: selected.map(|choice| paths[choice.winner].trace_path()),
            warrant: selected.map(|choice| choice.warrant),
            construction,
            sent: selected.is_some() && !construction,
        }));

        prop_assert_eq!(verify_choice_contract(&events), Ok(()));
    }
}

fn membership(body: &mut Body, parent: JunctionId, member: JunctionId) {
    let link = body.add_link(Link::new(parent, member, 0, 1)).unwrap();
    body.replace_arrow_state(link, ArrowState::membership())
        .unwrap();
}

#[test]
fn membership_parent_resolution_composes_recursively() {
    let mut body = Body::default();
    let members: [JunctionId; 4] =
        std::array::from_fn(|_| body.add_junction(Junction::integrating(1)).unwrap());
    let root = body.add_junction(Junction::integrating(1)).unwrap();
    membership(&mut body, root, members[0]);
    membership(&mut body, root, members[1]);
    let child = body.add_junction(Junction::integrating(1)).unwrap();
    membership(&mut body, child, root);
    membership(&mut body, child, members[2]);

    let view = ReactionView::new(&body.arena, &body.arrows, &body.returns);
    let mut scratch = ConstructionScratch::default();
    assert_eq!(
        resolve_membership_parent(
            view,
            &members[..2],
            &mut scratch.candidates,
            &mut scratch.stack,
            &mut scratch.visited,
            &mut scratch.leaves,
            &mut scratch.parent_members,
        ),
        MembershipParent::Existing(root)
    );
    assert_eq!(
        resolve_membership_parent(
            view,
            &members[..3],
            &mut scratch.candidates,
            &mut scratch.stack,
            &mut scratch.visited,
            &mut scratch.leaves,
            &mut scratch.parent_members,
        ),
        MembershipParent::Existing(child)
    );
    assert_eq!(scratch.parent_members.len(), 3);
    assert!(members[..3]
        .iter()
        .all(|member| scratch.parent_members.contains(member)));
}

#[test]
fn live_return_count_tracks_open_returns_and_retirement() {
    let mut body = Body::default();
    let from = body.add_junction(Junction::integrating(1)).unwrap();
    let to = body.add_junction(Junction::integrating(1)).unwrap();
    let first = body.add_link(Link::new(from, to, 0, 0)).unwrap();
    let second = body.add_link(Link::new(from, to, 0, 0)).unwrap();
    let third = body.add_link(Link::new(from, to, 0, 0)).unwrap();

    let path = Path {
        surface: from,
        middle: to,
        output: from,
        first,
        second,
    };
    body.replace_arrow_state(second, ArrowState::open_return(path, 9, 0))
        .unwrap();
    body.replace_arrow_state(first, ArrowState::open_return(path, 3, 0))
        .unwrap();
    body.replace_arrow_state(third, ArrowState::open_return(path, 9, 0))
        .unwrap();

    assert_eq!(body.returns.live_count, 3);
    assert_eq!(body.clone().returns.live_count, body.returns.live_count);

    let mut change = Change::empty();
    change.change_link(second.into(), LinkChange::Retire);
    body.apply(change).unwrap();
    body.replace_arrow_state(first, ArrowState::drive())
        .unwrap();

    assert_eq!(body.returns.live_count, 1);
    assert!(body.arrows[third.slot()].active());
}

#[test]
fn live_returns_are_indexed_by_their_own_outcome_source() {
    let mut body = Body::default();
    let mut outcomes = Vec::new();
    for index in 0..2_u64 {
        let motor = motor(&mut body);
        let sensor = attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        );
        let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        schedule(&mut body, index * 4, &[reading(outcome, 0, 0, 0)]);
        finish(&mut body);
        schedule(
            &mut body,
            1 + index * 4,
            &[reading(sensor, 0, 1, index + 1)],
        );
        schedule(
            &mut body,
            2 + index * 4,
            &[Arrival::caused(motor.opportunity, 1, index + 1)],
        );
        finish(&mut body);
        outcomes.push(outcome);
    }

    assert_eq!(body.returns.live_count, 2);
    assert_eq!(body.returns.by_source[outcomes[0].slot()].len(), 1);
    assert_eq!(body.returns.by_source[outcomes[1].slot()].len(), 1);
    assert_ne!(
        body.returns.by_source[outcomes[0].slot()][0].link,
        body.returns.by_source[outcomes[1].slot()][0].link
    );
    assert_eq!(
        body.returns.by_source.iter().map(Vec::len).sum::<usize>(),
        2
    );
}

#[test]
fn completing_a_shared_return_retires_it_from_every_outcome_source() {
    let mut body = Body::default();
    let motor = motor(&mut body);
    let sensor = attach_sensor(
        &mut body,
        Junction::integrating(1),
        &[(motor.opportunity, 1)],
    );
    let outcomes: [JunctionId; 2] = std::array::from_fn(|_| {
        let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        outcome
    });
    schedule(
        &mut body,
        0,
        &outcomes.map(|outcome| reading(outcome, 0, 0, 0)),
    );
    finish(&mut body);
    schedule(&mut body, 1, &[reading(sensor, 0, 1, 1)]);
    schedule(&mut body, 2, &[Arrival::caused(motor.opportunity, 1, 1)]);
    finish(&mut body);

    assert_eq!(body.returns.live_count, 1);
    assert!(outcomes.iter().all(|outcome| {
        let returns = &body.returns.by_source[outcome.slot()];
        returns.len() == 1 && !returns[0].exclusive_source
    }));

    schedule(&mut body, 3, &[Arrival::caused(outcomes[0], 1, 2)]);
    finish(&mut body);

    assert_eq!(body.returns.live_count, 0);
    assert!(outcomes
        .iter()
        .all(|outcome| body.returns.by_source[outcome.slot()].is_empty()));
}

#[test]
fn accepted_return_updates_memory_without_growing_morphology() {
    let mut body = Body::default();
    let motor = motor(&mut body);
    let sensor = attach_sensor(
        &mut body,
        Junction::integrating(1),
        &[(motor.opportunity, 1)],
    );
    let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
    attach_outcome_component(&mut body, outcome, [motor.opportunity]);
    schedule(&mut body, 0, &[reading(outcome, 0, 0, 0)]);
    finish(&mut body);
    schedule(&mut body, 1, &[reading(sensor, 0, 1, 1)]);
    schedule(&mut body, 2, &[Arrival::caused(motor.opportunity, 1, 1)]);
    finish(&mut body);

    let links_before_return = body.arena.link_count();
    schedule(&mut body, 20, &[Arrival::caused(outcome, 1, 2)]);
    finish(&mut body);

    assert_eq!(body.arena.link_count(), links_before_return);
}

#[test]
fn a_motor_gate_arrival_re_presents_one_returned_unfinished_path() {
    let mut body = Body::default();
    let motor = motor(&mut body);
    let surface = attach_sensor(
        &mut body,
        Junction::integrating(1),
        &[(motor.opportunity, 1)],
    );
    let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
    attach_outcome_component(&mut body, outcome, [motor.opportunity]);
    schedule(&mut body, 0, &[reading(outcome, 0, 0, 0)]);
    finish(&mut body);

    schedule(&mut body, 10, &[reading(surface, 0, 1, 1)]);
    schedule(&mut body, 11, &[Arrival::caused(motor.opportunity, 1, 1)]);
    assert_eq!(
        crate::harness::event_count(&finish(&mut body).events, motor.effect),
        1
    );
    schedule(&mut body, 12, &[reading(outcome, 0, 1, 1)]);
    finish(&mut body);

    let links_before = body.arena.link_count();
    schedule(&mut body, 20, &[Arrival::caused(motor.opportunity, 1, 2)]);
    let recurrence = finish(&mut body);

    assert_eq!(
        crate::harness::event_count(&recurrence.events, motor.effect),
        1
    );
    assert_eq!(body.arena.link_count(), links_before + 1);
    assert_eq!(body.returns.live_count, 1);
}

fn return_motor_path(
    body: &mut Body,
    motor: crate::harness::Motor,
    surface: JunctionId,
    outcome: JunctionId,
    at: Time,
    cause: Cause,
) {
    schedule(body, at, &[reading(surface, 0, 1, cause)]);
    schedule(
        body,
        at + 1,
        &[Arrival::caused(motor.opportunity, 1, cause)],
    );
    finish(body);
    schedule(body, at + 2, &[reading(outcome, 0, 1, cause)]);
    finish(body);
}

#[test]
fn an_unreturned_path_cannot_recur_from_a_motor_gate_arrival() {
    let mut body = Body::default();
    let motor = motor(&mut body);
    let surface = attach_sensor(
        &mut body,
        Junction::integrating(1),
        &[(motor.opportunity, 1)],
    );
    schedule(&mut body, 10, &[reading(surface, 0, 1, 1)]);
    schedule(&mut body, 11, &[Arrival::caused(motor.opportunity, 1, 1)]);
    finish(&mut body);

    schedule(&mut body, 20, &[Arrival::caused(motor.opportunity, 1, 2)]);
    let recurrence = finish(&mut body);

    assert_eq!(
        crate::harness::event_count(&recurrence.events, motor.effect),
        0
    );
}

#[test]
fn a_competition_component_re_presents_its_least_recent_exact_component() {
    let mut body = Body::default();
    let motors = [motor(&mut body), motor(&mut body)];
    let surfaces = motors.map(|motor| {
        attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        )
    });
    let outcomes = motors.map(|motor| {
        let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        outcome
    });
    schedule(
        &mut body,
        0,
        &outcomes.map(|outcome| reading(outcome, 0, 0, 0)),
    );
    finish(&mut body);
    return_motor_path(&mut body, motors[0], surfaces[0], outcomes[0], 10, 1);
    return_motor_path(&mut body, motors[1], surfaces[1], outcomes[1], 20, 2);
    let competition = attach_sensor(&mut body, Junction::integrating(1), &[]);
    attach_boundary_component(
        &mut body,
        competition,
        motors.map(|motor| motor.opportunity),
    );

    schedule(
        &mut body,
        30,
        &motors.map(|motor| Arrival::caused(motor.opportunity, 1, 3)),
    );
    let recurrence = finish(&mut body);

    assert_eq!(
        motors.map(|motor| crate::harness::event_count(&recurrence.events, motor.effect)),
        [1, 0]
    );
}

#[test]
fn equal_oldest_exact_components_preserve_ambiguity() {
    let mut body = Body::default();
    let motors = [motor(&mut body), motor(&mut body)];
    let surfaces = motors.map(|motor| {
        attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        )
    });
    let outcomes = motors.map(|motor| {
        let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        outcome
    });
    schedule(
        &mut body,
        0,
        &outcomes.map(|outcome| reading(outcome, 0, 0, 0)),
    );
    finish(&mut body);
    schedule(
        &mut body,
        10,
        &surfaces.map(|surface| reading(surface, 0, 1, 1)),
    );
    schedule(
        &mut body,
        11,
        &motors.map(|motor| Arrival::caused(motor.opportunity, 1, 1)),
    );
    finish(&mut body);
    schedule(
        &mut body,
        12,
        &outcomes.map(|outcome| reading(outcome, 0, 1, 1)),
    );
    finish(&mut body);
    let competition = attach_sensor(&mut body, Junction::integrating(1), &[]);
    attach_boundary_component(
        &mut body,
        competition,
        motors.map(|motor| motor.opportunity),
    );

    schedule(
        &mut body,
        20,
        &motors.map(|motor| Arrival::caused(motor.opportunity, 1, 2)),
    );
    let recurrence = finish(&mut body);

    assert!(motors
        .iter()
        .all(|motor| { crate::harness::event_count(&recurrence.events, motor.effect) == 0 }));
}

#[test]
fn independent_exact_components_recur_as_a_product() {
    let mut body = Body::default();
    let motors = [motor(&mut body), motor(&mut body)];
    let surfaces = motors.map(|motor| {
        attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        )
    });
    let outcomes = motors.map(|motor| {
        let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        outcome
    });
    schedule(
        &mut body,
        0,
        &outcomes.map(|outcome| reading(outcome, 0, 0, 0)),
    );
    finish(&mut body);
    return_motor_path(&mut body, motors[0], surfaces[0], outcomes[0], 10, 1);
    return_motor_path(&mut body, motors[1], surfaces[1], outcomes[1], 20, 2);

    schedule(
        &mut body,
        30,
        &motors.map(|motor| Arrival::caused(motor.opportunity, 1, 3)),
    );
    let recurrence = finish(&mut body);

    assert_eq!(
        motors.map(|motor| crate::harness::event_count(&recurrence.events, motor.effect)),
        [1, 1]
    );
}

#[test]
fn a_current_path_from_the_same_surface_suppresses_dormant_recurrence() {
    let mut body = Body::default();
    let motor = motor(&mut body);
    let surface = attach_sensor(
        &mut body,
        Junction::integrating(1),
        &[(motor.opportunity, 1)],
    );
    let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
    attach_outcome_component(&mut body, outcome, [motor.opportunity]);
    schedule(&mut body, 0, &[reading(outcome, 0, 0, 0)]);
    finish(&mut body);
    return_motor_path(&mut body, motor, surface, outcome, 10, 1);

    let path = body
        .arena
        .incoming(motor.opportunity)
        .find_map(|drive| {
            path_from_drive(
                ReactionView::new(&body.arena, &body.arrows, &body.returns),
                drive,
            )
        })
        .expect("returned path");
    body.arrows[path.first.slot()].record_transmission(Occurrence { cause: 2, at: 20 });

    schedule(&mut body, 20, &[Arrival::caused(motor.opportunity, 1, 2)]);
    let answered = finish(&mut body);

    assert_eq!(
        crate::harness::event_count(&answered.events, motor.effect),
        0
    );
}

#[test]
fn a_current_path_in_the_same_competition_component_suppresses_dormant_recurrence() {
    let mut body = Body::default();
    let motors = [motor(&mut body), motor(&mut body)];
    let surfaces = motors.map(|motor| {
        attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        )
    });
    let outcomes = motors.map(|motor| {
        let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        outcome
    });
    let competition = attach_sensor(&mut body, Junction::integrating(1), &[]);
    attach_boundary_component(
        &mut body,
        competition,
        motors.map(|motor| motor.opportunity),
    );
    schedule(
        &mut body,
        0,
        &outcomes.map(|outcome| reading(outcome, 0, 0, 0)),
    );
    finish(&mut body);
    return_motor_path(&mut body, motors[0], surfaces[0], outcomes[0], 10, 1);
    return_motor_path(&mut body, motors[1], surfaces[1], outcomes[1], 20, 2);

    let current = body
        .arena
        .incoming(motors[0].opportunity)
        .find_map(|drive| {
            path_from_drive(
                ReactionView::new(&body.arena, &body.arrows, &body.returns),
                drive,
            )
        })
        .expect("current path");
    for link in [current.first, current.second] {
        body.arrows[link.slot()].record_transmission(Occurrence { cause: 3, at: 30 });
    }

    schedule(
        &mut body,
        30,
        &[Arrival::caused(motors[1].opportunity, 1, 3)],
    );
    let answered = finish(&mut body);

    assert_eq!(
        crate::harness::event_count(&answered.events, motors[1].effect),
        0
    );
}

#[test]
fn a_returned_output_is_not_a_fresh_opportunity() {
    let mut body = Body::default();
    let motor = motor(&mut body);
    let surface = attach_sensor(
        &mut body,
        Junction::integrating(1),
        &[(motor.opportunity, 1)],
    );
    let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
    attach_outcome_component(&mut body, outcome, [motor.opportunity]);
    schedule(&mut body, 0, &[reading(outcome, 0, 0, 0)]);
    finish(&mut body);

    assert!(!output_has_returned_path(
        ReactionView::new(&body.arena, &body.arrows, &body.returns),
        motor.opportunity
    ));
    return_motor_path(&mut body, motor, surface, outcome, 10, 1);
    assert!(output_has_returned_path(
        ReactionView::new(&body.arena, &body.arrows, &body.returns),
        motor.opportunity
    ));
}
