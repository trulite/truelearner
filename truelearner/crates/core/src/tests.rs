use crate::prelude::*;

// These worlds follow the physical story in algo.md. Each always-on world
// protects one step; ignored worlds hold important time and causal boundaries.

fn input(target: JunctionId, tick: i64) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 7,
        target,
        impulse: 1,
    }
}

fn junction(
    body: &mut Body,
    physical_id: u64,
    position: i32,
    region: i16,
    threshold: i32,
) -> JunctionId {
    body.add_junction(Junction {
        physical_id,
        position,
        region,
        threshold,
        resistance: u32::MAX,
    })
}

fn link(
    body: &mut Body,
    from: JunctionId,
    to: JunctionId,
    delay: i64,
    strength: i32,
    life: u32,
    mode: TransmissionMode,
) -> LinkId {
    body.add_link(Link {
        from,
        to,
        delay,
        phase: 0,
        coupling: strength,
        resistance: life,
        mode,
    })
}

struct PathWorld {
    body: Body,
    input: JunctionId,
    opportunity: JunctionId,
    outcome: JunctionId,
}

impl PathWorld {
    fn new(output_distance: i32) -> Self {
        let mut body = Body::with_capacity(ArenaId(25), 32, 64);
        body.set_physical_tracing(true);
        let input = junction(&mut body, 25_000, 0, 0, 1);
        let opportunity = junction(&mut body, 25_001, output_distance, 0, 2);
        let output = junction(&mut body, 25_002, output_distance + 1, 1, 1);
        let outcome = junction(&mut body, 25_003, 50, 0, 1);
        let anchor = junction(&mut body, 25_004, 100, 0, 99);
        for target in [input, outcome] {
            link(
                &mut body,
                anchor,
                target,
                0,
                1,
                u32::MAX,
                TransmissionMode::Drive,
            );
        }
        link(
            &mut body,
            opportunity,
            output,
            0,
            1,
            u32::MAX,
            TransmissionMode::Drive,
        );
        body.set_outcome_source(outcome);
        Self {
            body,
            input,
            opportunity,
            outcome,
        }
    }

    fn use_path(&mut self) -> RunResult {
        self.body
            .arrive(&[input(self.input, 0), input(self.opportunity, 1)], 1)
    }

    fn return_outcome(&mut self, delay: i64) -> RunResult {
        let tick = self.body.tick.saturating_add(delay);
        self.body.advance_time(tick);
        self.body.arrive(&[input(self.outcome, tick)], 1)
    }
}

struct DeepWorld {
    body: Body,
    input: JunctionId,
    outcome: JunctionId,
    used: Vec<LinkId>,
    returns: Vec<LinkId>,
}

impl DeepWorld {
    fn new(depth: usize, life: u32) -> Self {
        assert!(depth > 0);
        let mut body = Body::with_capacity(ArenaId(26), 64, 128);
        body.set_physical_tracing(true);
        let input = junction(&mut body, 26_000, 0, 0, 1);
        let path = (0..depth)
            .map(|step| junction(&mut body, 26_001 + step as u64, 10 + step as i32 * 10, 0, 1))
            .collect::<Vec<_>>();
        let output_junction = junction(&mut body, 26_100, 1000, 0, 1);
        let output = junction(&mut body, 26_101, 1010, 1, 1);
        let outcome = junction(&mut body, 26_102, 2000, 0, 1);

        let mut used = Vec::with_capacity(depth + 1);
        let mut from = input;
        for to in path.iter().copied().chain([output_junction]) {
            used.push(link(
                &mut body,
                from,
                to,
                0,
                1,
                life,
                TransmissionMode::Drive,
            ));
            from = to;
        }
        link(
            &mut body,
            output_junction,
            output,
            0,
            1,
            u32::MAX,
            TransmissionMode::Drive,
        );
        link(
            &mut body,
            outcome,
            output_junction,
            0,
            1,
            u32::MAX,
            TransmissionMode::Modulatory,
        );

        let mut returns = Vec::with_capacity(depth + 1);
        let mut from = output_junction;
        for to in path.iter().rev().copied().chain([input]) {
            let id = link(
                &mut body,
                from,
                to,
                0,
                1,
                u32::MAX,
                TransmissionMode::Modulatory,
            );
            let slot = body.arena.link_slot(id).unwrap();
            body.arena.edit_link(slot.0, |state| {
                state.trigger = TransmissionTrigger::QualifiedLocalParticipation;
            });
            returns.push(id);
            from = to;
        }

        Self {
            body,
            input,
            outcome,
            used,
            returns,
        }
    }

    fn fire_input(&mut self) -> RunResult {
        self.body.arrive(&[input(self.input, self.body.tick)], 1)
    }

    fn return_outcome(&mut self) -> RunResult {
        self.body.arrive(&[input(self.outcome, self.body.tick)], 1)
    }
}

#[test]
fn input_without_near_output_forms_no_path() {
    let mut world = PathWorld::new(LOCAL_VARIATION_RADIUS + 1);
    let silence = world.body.propagate();
    assert!(silence.outputs.is_empty());
    assert!(world.body.arena.paths().is_empty());

    let input_only = world.body.arrive(&[input(world.input, 0)], 1);
    assert!(input_only.outputs.is_empty());
    assert!(world.body.arena.paths().is_empty());
    assert_eq!(input_only.work.local_structural_proposals, 0);
}

#[test]
fn input_forms_complete_signed_paths() {
    let mut world = PathWorld::new(1);
    assert!(world.body.arena.paths().is_empty());

    let run = world.use_path();
    let paths = world.body.arena.paths();
    let signs = paths
        .iter()
        .map(|path| world.body.link_strength(path.second).signum())
        .collect::<HashSet<_>>();
    assert_eq!(paths.len(), 2);
    assert_eq!(signs, HashSet::from([-1, 1]));
    assert_eq!(run.work.local_junction_proposals, 2);
    assert_eq!(run.work.local_structural_proposals, 4);
    assert!(run.naturally_quiescent);
}

#[test]
fn junction_chooses_one_path_and_output_fires() {
    let mut world = PathWorld::new(1);
    let run = world.use_path();
    let choice = run.physical_trace.iter().find_map(|step| match step.event {
        PhysicalEvent::PathChosen {
            positive_strength,
            negative_strength,
            opportunity_active,
            admitted_sign,
            ..
        } => Some((
            positive_strength,
            negative_strength,
            opportunity_active,
            admitted_sign,
        )),
        _ => None,
    });
    assert_eq!(choice, Some((UNIT_U64, UNIT_U64, true, 1)));
    assert_eq!(run.outputs.len(), 1);
    assert_eq!(world.body.return_path_count(), 1);
}

#[test]
fn used_path_holds_return_until_later_outcome() {
    let mut world = PathWorld::new(1);
    world.use_path();
    assert_eq!(world.body.return_path_count(), 1);

    let returned = world.return_outcome(20);
    assert!(returned.work.modulatory_deliveries > 0);
    assert_eq!(returned.work.local_return_updates, 2);
    assert_eq!(world.body.return_path_count(), 0);
    assert!(returned.naturally_quiescent);
}

#[test]
fn outcome_returns_through_every_used_link() {
    let mut world = DeepWorld::new(8, u32::MAX);
    let fired = world.fire_input();
    assert_eq!(fired.outputs.len(), 1);
    assert!(world.used.iter().all(|link| world.body.link_use(*link) > 0));

    let returned = world.return_outcome();
    let return_count = returned
        .physical_trace
        .iter()
        .filter(|step| {
            matches!(
                step.event,
                PhysicalEvent::QualifiedLocalTraversal { link }
                    if world.returns.contains(&link)
            )
        })
        .count();
    assert_eq!(return_count, world.returns.len());
    assert!(world
        .used
        .iter()
        .all(|link| world.body.link_strength(*link) > UNIT));
    assert!(returned.naturally_quiescent);
}

#[test]
fn strengthened_path_fires_on_later_input() {
    let mut world = PathWorld::new(1);
    world.use_path();
    world.return_outcome(1);

    let later = world
        .body
        .arrive(&[input(world.input, world.body.tick + 1)], 1);
    assert_eq!(later.outputs.len(), 1);
    assert!(later.naturally_quiescent);
}

#[test]
fn checkpoint_restores_the_same_body_and_time() {
    let mut world = PathWorld::new(1);
    world.body.set_physical_tracing(false);
    let core = Core::new(world.body, 1);
    let checkpoint = core.save(6).unwrap();
    let bytes = checkpoint.canonical_bytes().unwrap();
    let decoded = Checkpoint::decode(&bytes).unwrap();
    assert_eq!(decoded.canonical_bytes().unwrap(), bytes);
    assert_eq!(Core::restore(decoded).unwrap(), core);
}

#[test]
#[ignore = "adversarial time boundary"]
fn outcome_strengthens_used_links_only_before_the_time_boundary() {
    let mut inside = DeepWorld::new(8, 1);
    inside.fire_input();
    inside.body.advance_time(inside.body.tick + 9);
    assert!(inside.return_outcome().work.local_return_updates > 0);

    let mut outside = DeepWorld::new(8, 1);
    outside.fire_input();
    outside.body.advance_time(outside.body.tick + 10);
    outside.return_outcome();
    assert!(outside
        .used
        .iter()
        .all(|link| outside.body.link_strength(*link) == UNIT));
}

#[test]
#[ignore = "adversarial causal boundary"]
fn outcome_does_not_form_a_missing_return() {
    let mut world = PathWorld::new(1);
    world.use_path();
    let returned = world.body.arena.return_links(Some(world.outcome));
    assert_eq!(returned.len(), 1);
    world.body.return_outcome(returned[0]);

    let outcome = world.return_outcome(1);
    assert_eq!(outcome.work.modulatory_deliveries, 0);
    assert_eq!(outcome.work.local_return_updates, 0);
}
