use std::collections::HashSet;
use truelearner_core::{
    Checkpoint, CheckpointError, Harness, HarnessBuilder, Input, Junction, JunctionId, Link,
    LinkId, PhysicalEvent, Protocol, Run, TransmissionMode, TransmissionTrigger,
};

const OUTWARD_REGION: i16 = 1;
const LOCAL_VARIATION_RADIUS: i32 = 2;
const UNIT: i32 = 1;
const UNIT_U64: u64 = 1_u64 << 32;

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
    builder: &mut HarnessBuilder,
    physical_id: u64,
    position: i32,
    region: i16,
    threshold: i32,
) -> JunctionId {
    builder.add_junction(Junction {
        physical_id,
        position,
        region,
        threshold,
        resistance: u32::MAX,
    })
}

fn link(
    builder: &mut HarnessBuilder,
    from: JunctionId,
    to: JunctionId,
    delay: i64,
    coupling: i32,
    resistance: u32,
    mode: TransmissionMode,
) -> LinkId {
    builder.add_link(Link {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode,
    })
}

struct PathWorld {
    harness: Harness,
    input: JunctionId,
    opportunity: JunctionId,
    outcome: JunctionId,
}

impl PathWorld {
    fn new(output_distance: i32) -> Self {
        Self::with_tracing(output_distance, true)
    }

    fn with_tracing(output_distance: i32, tracing: bool) -> Self {
        let mut builder = HarnessBuilder::with_capacity(32, 64, OUTWARD_REGION);
        builder.set_physical_tracing(tracing);
        let input = junction(&mut builder, 25_000, 0, 0, 1);
        let opportunity = junction(&mut builder, 25_001, output_distance, 0, 2);
        let output = junction(&mut builder, 25_002, output_distance + 1, 1, 1);
        let outcome = junction(&mut builder, 25_003, 50, 0, 1);
        let anchor = junction(&mut builder, 25_004, 100, 0, 99);
        for target in [input, outcome] {
            link(
                &mut builder,
                anchor,
                target,
                0,
                1,
                u32::MAX,
                TransmissionMode::Drive,
            );
        }
        link(
            &mut builder,
            opportunity,
            output,
            0,
            1,
            u32::MAX,
            TransmissionMode::Drive,
        );
        builder.set_outcome_source(outcome);
        Self {
            harness: builder.build(),
            input,
            opportunity,
            outcome,
        }
    }

    fn use_path(&mut self) -> Run {
        self.harness
            .send(&[input(self.input, 0), input(self.opportunity, 1)])
    }

    fn return_outcome(&mut self, delay: i64) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(delay);
        self.harness.advance_to(tick);
        self.harness.send(&[input(self.outcome, tick)])
    }

    fn path_couplings(&self) -> Vec<i32> {
        self.harness
            .read()
            .links
            .into_iter()
            .filter(|link| link.live && link.to == self.opportunity)
            .map(|link| link.coupling)
            .collect()
    }
}

struct DeepWorld {
    harness: Harness,
    input: JunctionId,
    outcome: JunctionId,
    used: Vec<LinkId>,
    returns: Vec<LinkId>,
}

struct LocalPairWorld {
    harness: Harness,
    input: JunctionId,
    opportunities: [JunctionId; 2],
    outcome: JunctionId,
}

impl LocalPairWorld {
    fn new() -> Self {
        Self::with_positions([-1, 1])
    }

    fn with_positions(positions: [i32; 2]) -> Self {
        let mut builder = HarnessBuilder::with_capacity(32, 64, OUTWARD_REGION);
        let input = junction(&mut builder, 27_000, 0, 0, 1);
        let left = junction(&mut builder, 27_001, positions[0], 0, 2);
        let right = junction(&mut builder, 27_002, positions[1], 0, 2);
        let left_sink = junction(&mut builder, 27_003, positions[0], 1, 1);
        let right_sink = junction(&mut builder, 27_004, positions[1], 1, 1);
        let outcome = junction(&mut builder, 27_005, 50, 0, 1);
        let anchor = junction(&mut builder, 27_006, 100, 0, 99);
        for target in [input, outcome] {
            link(
                &mut builder,
                anchor,
                target,
                0,
                1,
                u32::MAX,
                TransmissionMode::Drive,
            );
        }
        for (motor, sink) in [(left, left_sink), (right, right_sink)] {
            link(
                &mut builder,
                motor,
                sink,
                0,
                1,
                u32::MAX,
                TransmissionMode::Drive,
            );
        }
        builder.set_outcome_source(outcome);
        Self {
            harness: builder.build(),
            input,
            opportunities: [left, right],
            outcome,
        }
    }

    fn stimulate(&mut self) -> Run {
        let input_tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.input, input_tick),
            input(self.opportunities[0], input_tick.saturating_add(1)),
            input(self.opportunities[1], input_tick.saturating_add(1)),
        ])
    }

    fn outcome(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[input(self.outcome, tick)])
    }
}

impl DeepWorld {
    fn new(depth: usize, resistance: u32) -> Self {
        assert!(depth > 0);
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_physical_tracing(true);
        let input = junction(&mut builder, 26_000, 0, 0, 1);
        let path = (0..depth)
            .map(|step| {
                junction(
                    &mut builder,
                    26_001 + step as u64,
                    10 + step as i32 * 10,
                    0,
                    1,
                )
            })
            .collect::<Vec<_>>();
        let output_junction = junction(&mut builder, 26_100, 1000, 0, 1);
        let output = junction(&mut builder, 26_101, 1010, 1, 1);
        let outcome = junction(&mut builder, 26_102, 2000, 0, 1);

        let mut used = Vec::with_capacity(depth + 1);
        let mut from = input;
        for to in path.iter().copied().chain([output_junction]) {
            used.push(link(
                &mut builder,
                from,
                to,
                0,
                1,
                resistance,
                TransmissionMode::Drive,
            ));
            from = to;
        }
        link(
            &mut builder,
            output_junction,
            output,
            0,
            1,
            u32::MAX,
            TransmissionMode::Drive,
        );
        link(
            &mut builder,
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
                &mut builder,
                from,
                to,
                0,
                1,
                u32::MAX,
                TransmissionMode::Modulatory,
            );
            builder.set_link_trigger(id, TransmissionTrigger::QualifiedLocalParticipation);
            returns.push(id);
            from = to;
        }
        builder.set_outcome_source(outcome);

        Self {
            harness: builder.build(),
            input,
            outcome,
            used,
            returns,
        }
    }

    fn fire_input(&mut self) -> Run {
        let tick = self.harness.read().clock.tick;
        self.harness.send(&[input(self.input, tick)])
    }

    fn return_outcome(&mut self) -> Run {
        let tick = self.harness.read().clock.tick;
        self.harness.send(&[input(self.outcome, tick)])
    }

    fn used_couplings(&self) -> Vec<i32> {
        let observation = self.harness.read();
        self.used
            .iter()
            .map(|id| {
                observation
                    .link(*id)
                    .expect("used link remains observable")
                    .coupling
            })
            .collect()
    }
}

#[test]
fn input_without_near_output_forms_no_path() {
    let mut world = PathWorld::new(LOCAL_VARIATION_RADIUS + 1);
    let silence = world.harness.send(&[]);
    assert!(silence.outputs.is_empty());
    assert!(world.path_couplings().is_empty());

    let input_only = world.harness.send(&[input(world.input, 0)]);
    assert!(input_only.outputs.is_empty());
    assert!(world.path_couplings().is_empty());
    assert_eq!(input_only.work.local_structural_proposals, 0);
}

#[test]
fn input_forms_complete_signed_paths() {
    let mut world = PathWorld::new(1);
    assert!(world.path_couplings().is_empty());

    let run = world.use_path();
    let couplings = world.path_couplings();
    let signs = couplings
        .iter()
        .map(|value| value.signum())
        .collect::<HashSet<_>>();
    assert_eq!(couplings.len(), 2);
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
    assert_eq!(world.harness.read().return_path_count, 1);
}

#[test]
fn local_motor_competition_explores_then_reuses_consequence() {
    let mut without_outcome = LocalPairWorld::new();
    let first = without_outcome.stimulate();
    let first_observation = without_outcome.harness.read();
    let second = without_outcome.stimulate();
    assert_eq!(first.outputs.len(), 1);
    assert_eq!(second.outputs.len(), 1);
    assert_ne!(
        first.outputs[0].from_physical,
        second.outputs[0].from_physical
    );
    let winner = first_observation
        .junctions
        .iter()
        .find(|junction| junction.physical_id == first.outputs[0].from_physical)
        .unwrap()
        .id;
    let loser = without_outcome
        .opportunities
        .into_iter()
        .find(|opportunity| *opportunity != winner)
        .unwrap();
    let loser_seconds = first_observation
        .links
        .iter()
        .filter(|link| link.live && link.to == loser)
        .collect::<Vec<_>>();
    assert!(loser_seconds.iter().all(|link| link.participation == 0));
    assert!(loser_seconds.iter().any(|second| {
        first_observation
            .links
            .iter()
            .any(|first| first.live && first.to == second.from && first.participation > 0)
    }));

    let mut with_outcome = LocalPairWorld::new();
    let consequential = with_outcome.stimulate();
    assert_eq!(consequential.outputs.len(), 1);
    assert_eq!(with_outcome.harness.read().return_path_count, 1);
    assert!(with_outcome.outcome().work.local_return_updates > 0);
    let reused = with_outcome.stimulate();
    assert_eq!(reused.outputs.len(), 1);
    assert_eq!(
        consequential.outputs[0].from_physical,
        reused.outputs[0].from_physical
    );

    let mut reflected = LocalPairWorld::with_positions([1, -1]);
    let reflected_first = reflected.stimulate();
    let reflected_second = reflected.stimulate();
    assert_eq!(reflected_first.outputs.len(), 1);
    assert_eq!(reflected_second.outputs.len(), 1);
    assert_ne!(
        reflected_first.outputs[0].from_physical,
        reflected_second.outputs[0].from_physical
    );

    let checkpoint = reflected.harness.save().unwrap();
    let mut restored = Harness::restore(checkpoint).unwrap();
    let input_tick = restored.read().clock.tick.saturating_add(1);
    let inputs = [
        input(reflected.input, input_tick),
        input(reflected.opportunities[0], input_tick.saturating_add(1)),
        input(reflected.opportunities[1], input_tick.saturating_add(1)),
    ];
    assert_eq!(reflected.harness.send(&inputs), restored.send(&inputs));
    assert_eq!(
        reflected.harness.save().unwrap().canonical_bytes().unwrap(),
        restored.save().unwrap().canonical_bytes().unwrap()
    );
}

#[test]
fn local_motor_competition_does_not_suppress_far_outputs() {
    let mut builder = HarnessBuilder::with_capacity(32, 64, OUTWARD_REGION);
    let left_input = junction(&mut builder, 28_000, 0, 0, 1);
    let left_motor = junction(&mut builder, 28_001, 1, 0, 2);
    let left_sink = junction(&mut builder, 28_002, 1, 1, 1);
    let right_input = junction(&mut builder, 28_003, 10, 0, 1);
    let right_motor = junction(&mut builder, 28_004, 11, 0, 2);
    let right_sink = junction(&mut builder, 28_005, 11, 1, 1);
    let anchor = junction(&mut builder, 28_006, 100, 0, 99);
    for target in [left_input, right_input] {
        link(
            &mut builder,
            anchor,
            target,
            0,
            1,
            u32::MAX,
            TransmissionMode::Drive,
        );
    }
    for (motor, sink) in [(left_motor, left_sink), (right_motor, right_sink)] {
        link(
            &mut builder,
            motor,
            sink,
            0,
            1,
            u32::MAX,
            TransmissionMode::Drive,
        );
    }
    let mut harness = builder.build();
    let run = harness.send(&[
        input(left_input, 1),
        input(right_input, 1),
        input(left_motor, 2),
        input(right_motor, 2),
    ]);
    assert_eq!(run.outputs.len(), 2);
    assert!(run.naturally_quiescent);
}

#[test]
fn used_path_holds_return_until_later_outcome() {
    let mut world = PathWorld::new(1);
    world.use_path();
    assert_eq!(world.harness.read().return_path_count, 1);

    let returned = world.return_outcome(20);
    assert!(returned.work.modulatory_deliveries > 0);
    assert_eq!(returned.work.local_return_updates, 2);
    assert_eq!(world.harness.read().return_path_count, 0);
    assert!(returned.naturally_quiescent);
}

#[test]
fn outcome_returns_through_every_used_link() {
    let mut world = DeepWorld::new(8, u32::MAX);
    let fired = world.fire_input();
    assert_eq!(fired.outputs.len(), 1);

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
        .used_couplings()
        .into_iter()
        .all(|coupling| coupling > UNIT));
    assert!(returned.naturally_quiescent);
}

#[test]
fn strengthened_path_fires_on_later_input() {
    let mut world = PathWorld::new(1);
    world.use_path();
    world.return_outcome(1);

    let tick = world.harness.read().clock.tick.saturating_add(1);
    let later = world.harness.send(&[input(world.input, tick)]);
    assert_eq!(later.outputs.len(), 1);
    assert!(later.naturally_quiescent);
}

#[test]
fn checkpoint_and_reads_preserve_body_time_and_protocol() {
    let mut world = PathWorld::with_tracing(1, false);
    world.use_path();
    let first_read = world.harness.read();
    assert_eq!(world.harness.read(), first_read);
    assert_eq!(first_read.protocol, Protocol::Physical);

    let checkpoint = world.harness.save().unwrap();
    let bytes = checkpoint.canonical_bytes().unwrap();
    let decoded = Checkpoint::decode(&bytes).unwrap();
    assert_eq!(decoded.canonical_bytes().unwrap(), bytes);
    assert!(matches!(
        Checkpoint::decode(&bytes[..bytes.len() - 1]),
        Err(CheckpointError::Truncated)
    ));
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(matches!(
        Checkpoint::decode(&trailing),
        Err(CheckpointError::TrailingBytes)
    ));
    let mut corrupt = bytes.clone();
    corrupt[50] ^= 1;
    assert!(matches!(
        Checkpoint::decode(&corrupt),
        Err(CheckpointError::Checksum)
    ));
    let restored = Harness::restore(decoded).unwrap();
    assert_eq!(restored, world.harness);
    assert_eq!(restored.read(), first_read);

    let tick = restored.read().clock.tick.saturating_add(1);
    let mut without_read = restored.clone();
    let mut with_read = restored;
    let _ = with_read.read();
    assert_eq!(
        without_read.send(&[input(world.input, tick)]),
        with_read.send(&[input(world.input, tick)])
    );
    assert_eq!(
        without_read.save().unwrap().canonical_bytes().unwrap(),
        with_read.save().unwrap().canonical_bytes().unwrap()
    );
}

#[test]
#[ignore = "adversarial time boundary"]
fn outcome_strengthens_used_links_only_before_the_time_boundary() {
    let mut inside = DeepWorld::new(8, 1);
    inside.fire_input();
    let inside_tick = inside.harness.read().clock.tick.saturating_add(9);
    inside.harness.advance_to(inside_tick);
    assert!(inside.return_outcome().work.local_return_updates > 0);

    let mut outside = DeepWorld::new(8, 1);
    outside.fire_input();
    let outside_tick = outside.harness.read().clock.tick.saturating_add(10);
    outside.harness.advance_to(outside_tick);
    outside.return_outcome();
    assert!(outside
        .used_couplings()
        .into_iter()
        .all(|coupling| coupling == UNIT));
}

#[test]
#[ignore = "adversarial causal boundary"]
fn outcome_does_not_form_a_missing_return() {
    let mut world = PathWorld::new(1);
    world.use_path();
    let first = world.return_outcome(1);
    assert!(first.work.modulatory_deliveries > 0);
    assert_eq!(world.harness.read().return_path_count, 0);

    let second = world.return_outcome(1);
    assert_eq!(second.work.modulatory_deliveries, 0);
    assert_eq!(second.work.local_return_updates, 0);
}
