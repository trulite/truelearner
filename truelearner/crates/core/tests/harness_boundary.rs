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
        Self::with_protocol(output_distance, tracing, Protocol::Physical)
    }

    fn candidate(output_distance: i32) -> Self {
        Self::with_protocol(output_distance, false, Protocol::SensorimotorCandidate)
    }

    fn with_protocol(output_distance: i32, tracing: bool, protocol: Protocol) -> Self {
        let mut builder = HarnessBuilder::with_capacity(32, 64, OUTWARD_REGION);
        builder.set_physical_tracing(tracing);
        builder.set_protocol(protocol);
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

struct LocalOutcomePairWorld {
    harness: Harness,
    input: JunctionId,
    opportunities: [JunctionId; 2],
    outcomes: [JunctionId; 2],
    physical_outputs: [u64; 2],
}

#[derive(Clone, Copy)]
enum OutcomeWiring {
    Global,
    Local,
    Shuffled,
}

struct CausalPairWorld {
    harness: Harness,
    inputs: [JunctionId; 2],
    opportunities: [JunctionId; 2],
    outcomes: [JunctionId; 2],
}

impl CausalPairWorld {
    fn new(wiring: OutcomeWiring, reflected: bool) -> Self {
        let positions = if reflected { [10, 0] } else { [0, 10] };
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_physical_tracing(true);
        let inputs: [JunctionId; 2] = std::array::from_fn(|index| {
            junction(&mut builder, 29_000 + index as u64, positions[index], 0, 1)
        });
        let opportunities: [JunctionId; 2] = std::array::from_fn(|index| {
            junction(
                &mut builder,
                29_010 + index as u64,
                positions[index] + 1,
                0,
                2,
            )
        });
        let sinks: [JunctionId; 2] = std::array::from_fn(|index| {
            junction(
                &mut builder,
                29_020 + index as u64,
                positions[index] + 1,
                OUTWARD_REGION,
                1,
            )
        });
        let outcomes: [JunctionId; 2] = std::array::from_fn(|index| {
            junction(
                &mut builder,
                29_030 + index as u64,
                100 + index as i32 * 10,
                0,
                1,
            )
        });
        let anchor = junction(&mut builder, 29_040, 1_000, 0, 99);
        for target in inputs.into_iter().chain(outcomes) {
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
        for index in 0..2 {
            link(
                &mut builder,
                opportunities[index],
                sinks[index],
                0,
                1,
                u32::MAX,
                TransmissionMode::Drive,
            );
        }
        match wiring {
            OutcomeWiring::Global => builder.set_outcome_source(outcomes[0]),
            OutcomeWiring::Local => {
                for index in 0..2 {
                    builder.set_outcome_source_for_output(opportunities[index], outcomes[index]);
                }
            }
            OutcomeWiring::Shuffled => {
                for index in 0..2 {
                    builder
                        .set_outcome_source_for_output(opportunities[index], outcomes[1 - index]);
                }
            }
        }
        Self {
            harness: builder.build(),
            inputs,
            opportunities,
            outcomes,
        }
    }

    fn participate(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.inputs[0], tick),
            input(self.inputs[1], tick),
            input(self.opportunities[0], tick.saturating_add(1)),
            input(self.opportunities[1], tick.saturating_add(1)),
        ])
    }

    fn return_first_physical_consequence(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[input(self.outcomes[0], tick)])
    }

    fn used_second_strengths(&self) -> [i64; 2] {
        let observation = self.harness.read();
        std::array::from_fn(|index| {
            observation
                .links
                .iter()
                .filter(|link| {
                    link.live && link.to == self.opportunities[index] && link.participation > 0
                })
                .map(|link| link.strength)
                .max()
                .expect("each participating output has one used second link")
        })
    }
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

impl LocalOutcomePairWorld {
    fn new(protocol: Protocol) -> Self {
        Self::with_positions(protocol, [-1, 1])
    }

    fn with_positions(protocol: Protocol, positions: [i32; 2]) -> Self {
        let mut builder = HarnessBuilder::with_capacity(48, 96, OUTWARD_REGION);
        builder.set_physical_tracing(true);
        builder.set_protocol(protocol);
        let input = junction(&mut builder, 28_000, 0, 0, 1);
        let physical_outputs = [28_001, 28_002];
        let opportunities = [
            junction(&mut builder, physical_outputs[0], positions[0], 0, 2),
            junction(&mut builder, physical_outputs[1], positions[1], 0, 2),
        ];
        let sinks = [
            junction(&mut builder, 28_003, positions[0], OUTWARD_REGION, 1),
            junction(&mut builder, 28_004, positions[1], OUTWARD_REGION, 1),
        ];
        let outcomes = [
            junction(&mut builder, 28_005, 50, 0, 1),
            junction(&mut builder, 28_006, 60, 0, 1),
        ];
        let anchor = junction(&mut builder, 28_007, 100, 0, 99);
        for target in [input, outcomes[0], outcomes[1]] {
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
        for index in 0..2 {
            link(
                &mut builder,
                opportunities[index],
                sinks[index],
                0,
                1,
                u32::MAX,
                TransmissionMode::Drive,
            );
            builder.set_outcome_source_for_output(opportunities[index], outcomes[index]);
        }
        Self {
            harness: builder.build(),
            input,
            opportunities,
            outcomes,
            physical_outputs,
        }
    }

    fn stimulate(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.input, tick),
            input(self.opportunities[0], tick.saturating_add(1)),
            input(self.opportunities[1], tick.saturating_add(1)),
        ])
    }

    fn outcome_for(&mut self, output: u64, delay: i64) -> Run {
        let index = self
            .physical_outputs
            .iter()
            .position(|physical| *physical == output)
            .expect("output belongs to the local pair");
        let tick = self.harness.read().clock.tick.saturating_add(delay);
        self.harness.advance_to(tick);
        self.harness.send(&[input(self.outcomes[index], tick)])
    }

    fn used_strength(&self, output: u64) -> i64 {
        let index = self
            .physical_outputs
            .iter()
            .position(|physical| *physical == output)
            .expect("output belongs to the local pair");
        self.harness
            .read()
            .links
            .into_iter()
            .filter(|link| {
                link.live && link.to == self.opportunities[index] && link.participation > 0
            })
            .map(|link| link.strength)
            .max()
            .expect("participating output has a used path")
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
fn unanswered_local_return_deferral_exposes_one_neighbor() {
    let mut world = LocalOutcomePairWorld::new(Protocol::UnansweredReturnDeferral);
    let first = world.stimulate().outputs[0].from_physical;
    assert_eq!(world.outcome_for(first, 1).work.local_return_updates, 2);
    let reused = world.stimulate().outputs[0].from_physical;
    let alternative = world.stimulate();

    assert_eq!(reused, first);
    assert_eq!(alternative.outputs.len(), 1);
    assert_ne!(alternative.outputs[0].from_physical, reused);
    assert_eq!(world.harness.read().return_path_count, 2);
    assert!(alternative.naturally_quiescent);
}

#[test]
fn unanswered_local_return_replacement_closes_only_the_displaced_return() {
    let mut world = LocalOutcomePairWorld::new(Protocol::UnansweredReturnReplacement);
    let first = world.stimulate().outputs[0].from_physical;
    world.outcome_for(first, 1);
    let reused = world.stimulate().outputs[0].from_physical;
    let before = world.used_strength(reused);
    let alternative = world.stimulate();
    let fresh = alternative.outputs[0].from_physical;

    assert_eq!(reused, first);
    assert_ne!(fresh, reused);
    assert_eq!(world.harness.read().return_path_count, 1);
    assert!(alternative
        .physical_trace
        .iter()
        .any(|transition| matches!(transition.event, PhysicalEvent::ReturnSuperseded { .. })));
    let returned = world.outcome_for(fresh, 1);
    assert_eq!(returned.work.local_return_updates, 2);
    assert_eq!(world.used_strength(reused), before);
    assert_eq!(world.harness.read().return_path_count, 0);
}

#[test]
fn unanswered_local_return_keeps_a_valid_delay_without_competition() {
    let mut world = LocalOutcomePairWorld::new(Protocol::UnansweredReturnReplacement);
    let first = world.stimulate().outputs[0].from_physical;
    let before = world.used_strength(first);
    let returned = world.outcome_for(first, 20);

    assert_eq!(returned.work.local_return_updates, 2);
    assert!(world.used_strength(first) > before);
    assert_eq!(world.harness.read().return_path_count, 0);
}

#[test]
fn unanswered_local_return_protocol_reflects_and_survives_checkpoint() {
    let mut ordinary =
        LocalOutcomePairWorld::with_positions(Protocol::UnansweredReturnReplacement, [-1, 1]);
    let mut reflected =
        LocalOutcomePairWorld::with_positions(Protocol::UnansweredReturnReplacement, [1, -1]);
    let ordinary_first = ordinary.stimulate().outputs[0].from_physical;
    let reflected_first = reflected.stimulate().outputs[0].from_physical;
    ordinary.outcome_for(ordinary_first, 1);
    reflected.outcome_for(reflected_first, 1);
    ordinary.stimulate();
    reflected.stimulate();
    let ordinary_alternative = ordinary.stimulate().outputs[0].from_physical;
    let reflected_alternative = reflected.stimulate().outputs[0].from_physical;

    let checkpoint = ordinary.harness.save().expect("checkpoint saves");
    let restored = Harness::restore(checkpoint).expect("checkpoint restores");
    assert_eq!(restored.read(), ordinary.harness.read());
    assert_eq!(
        restored.read().protocol,
        Protocol::UnansweredReturnReplacement
    );
    assert_ne!(ordinary_first, reflected_first);
    assert_eq!(ordinary_first, reflected_alternative);
    assert_eq!(reflected_first, ordinary_alternative);
}

#[test]
fn unanswered_local_return_replacement_does_not_suppress_a_far_output() {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
    builder.set_protocol(Protocol::UnansweredReturnReplacement);
    let local_input = junction(&mut builder, 28_100, 0, 0, 1);
    let local_motors = [
        junction(&mut builder, 28_101, -1, 0, 2),
        junction(&mut builder, 28_102, 1, 0, 2),
    ];
    let local_sinks = [
        junction(&mut builder, 28_103, -1, OUTWARD_REGION, 1),
        junction(&mut builder, 28_104, 1, OUTWARD_REGION, 1),
    ];
    let local_outcomes = [
        junction(&mut builder, 28_105, 50, 0, 1),
        junction(&mut builder, 28_106, 60, 0, 1),
    ];
    let far_input = junction(&mut builder, 28_107, 20, 0, 1);
    let far_motor = junction(&mut builder, 28_108, 21, 0, 2);
    let far_sink = junction(&mut builder, 28_109, 21, OUTWARD_REGION, 1);
    let anchor = junction(&mut builder, 28_110, 100, 0, 99);
    for target in [local_input, far_input, local_outcomes[0], local_outcomes[1]] {
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
    for index in 0..2 {
        link(
            &mut builder,
            local_motors[index],
            local_sinks[index],
            0,
            1,
            u32::MAX,
            TransmissionMode::Drive,
        );
        builder.set_outcome_source_for_output(local_motors[index], local_outcomes[index]);
    }
    link(
        &mut builder,
        far_motor,
        far_sink,
        0,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    let mut harness = builder.build();
    let stimulate = |harness: &Harness| {
        let tick = harness.read().clock.tick.saturating_add(1);
        [
            input(local_input, tick),
            input(far_input, tick),
            input(local_motors[0], tick + 1),
            input(local_motors[1], tick + 1),
            input(far_motor, tick + 1),
        ]
    };
    let first = harness.send(&stimulate(&harness));
    let local_first = first
        .outputs
        .iter()
        .find(|output| output.from_physical != 28_108)
        .expect("one local output fires")
        .from_physical;
    let local_outcome = if local_first == 28_101 {
        local_outcomes[0]
    } else {
        local_outcomes[1]
    };
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[input(local_outcome, tick)]);
    harness.send(&stimulate(&harness));
    let alternative = harness.send(&stimulate(&harness));

    assert_eq!(alternative.outputs.len(), 2);
    assert!(alternative
        .outputs
        .iter()
        .any(|output| output.from_physical == 28_108));
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
fn causal_local_outcome_global_reference_credits_both_participating_paths() {
    let mut world = CausalPairWorld::new(OutcomeWiring::Global, false);
    assert_eq!(world.participate().outputs.len(), 2);
    let before = world.used_second_strengths();
    let returned = world.return_first_physical_consequence();
    let after = world.used_second_strengths();

    assert!(returned.work.local_return_updates >= 4);
    assert!(after[0] > before[0]);
    assert!(after[1] > before[1]);
}

#[test]
fn causal_local_outcome_credits_only_the_physically_wired_path() {
    for reflected in [false, true] {
        let mut world = CausalPairWorld::new(OutcomeWiring::Local, reflected);
        assert_eq!(world.participate().outputs.len(), 2);
        let before = world.used_second_strengths();
        assert_eq!(world.harness.read().return_path_count, 2);
        let checkpoint = world.harness.save().unwrap();

        let returned = world.return_first_physical_consequence();
        let after = world.used_second_strengths();
        assert!(returned.work.local_return_updates >= 2);
        assert!(after[0] > before[0]);
        assert_eq!(after[1], before[1]);
        assert_eq!(world.harness.read().return_path_count, 1);
        assert!(returned.naturally_quiescent);

        let mut restored = Harness::restore(checkpoint).unwrap();
        let tick = restored.read().clock.tick.saturating_add(1);
        let replayed = restored.send(&[input(world.outcomes[0], tick)]);
        assert_eq!(returned.outputs, replayed.outputs);
        assert_eq!(returned.work, replayed.work);
        assert_eq!(returned.naturally_quiescent, replayed.naturally_quiescent);
        assert_eq!(
            world.harness.save().unwrap().canonical_bytes().unwrap(),
            restored.save().unwrap().canonical_bytes().unwrap()
        );
    }
}

#[test]
fn causal_local_outcome_shuffled_wiring_credits_the_wrong_path() {
    let mut world = CausalPairWorld::new(OutcomeWiring::Shuffled, false);
    assert_eq!(world.participate().outputs.len(), 2);
    let before = world.used_second_strengths();
    world.return_first_physical_consequence();
    let after = world.used_second_strengths();

    assert_eq!(after[0], before[0]);
    assert!(after[1] > before[1]);
}

#[test]
#[should_panic(expected = "an output may have only one local outcome source")]
fn causal_local_outcome_rejects_duplicate_output_wiring() {
    let mut builder = HarnessBuilder::with_capacity(8, 8, OUTWARD_REGION);
    let motor = junction(&mut builder, 29_100, 0, 0, 1);
    let sink = junction(&mut builder, 29_101, 0, OUTWARD_REGION, 1);
    let first = junction(&mut builder, 29_102, 10, 0, 1);
    let second = junction(&mut builder, 29_103, 20, 0, 1);
    link(
        &mut builder,
        motor,
        sink,
        0,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    builder.set_outcome_source_for_output(motor, first);
    builder.set_outcome_source_for_output(motor, second);
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
fn sensorimotor_candidate_preserves_valid_return_expires_stale_return_and_replays() {
    let mut valid = PathWorld::with_protocol(1, true, Protocol::SensorimotorCandidate);
    let used = valid.use_path();
    assert_eq!(used.outputs.len(), 1);
    let tick = valid.harness.read().clock.tick.saturating_add(20);
    valid.harness.advance_to(tick);
    let checkpoint = valid.harness.save().unwrap();
    let mut replay = Harness::restore(checkpoint.clone()).unwrap();
    let consequence = [Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 25_003,
        target: valid.outcome,
        impulse: 1,
    }];
    let observed = valid.harness.send(&consequence);
    let replayed = replay.send(&consequence);
    assert_eq!(observed, replayed);
    assert!(observed.work.local_return_updates > 0);
    assert!(observed
        .physical_trace
        .iter()
        .any(|transition| matches!(transition.event, PhysicalEvent::ConsequenceRecorded { .. })));
    assert!(valid
        .harness
        .read()
        .links
        .iter()
        .any(|link| link.last_consequence_tick == Some(tick)));
    assert_eq!(
        valid.harness.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );

    let mut stale = PathWorld::candidate(1);
    stale.use_path();
    let tick = stale.harness.read().clock.tick.saturating_add(200);
    stale.harness.advance_to(tick);
    let returned = stale.harness.send(&[Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 25_003,
        target: stale.outcome,
        impulse: 1,
    }]);
    assert_eq!(returned.work.local_return_updates, 0);
    assert_eq!(stale.harness.read().return_path_count, 0);
    assert!(returned.naturally_quiescent);
}

#[test]
fn sensorimotor_candidate_uses_current_proprioception_to_cross_motor_threshold() {
    let mut builder = HarnessBuilder::with_capacity(32, 64, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorSynthesis);
    let source = junction(&mut builder, 31_000, 0, 0, 1);
    let motor = junction(&mut builder, 31_001, 1, 0, 2);
    let sink = junction(&mut builder, 31_002, 1, OUTWARD_REGION, 1);
    let anchor = junction(&mut builder, 31_003, 100, 0, 99);
    link(
        &mut builder,
        anchor,
        source,
        0,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    link(
        &mut builder,
        motor,
        sink,
        0,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    let mut harness = builder.build();

    let run = harness.send(&[input(source, 0), input(motor, 2)]);

    assert_eq!(run.outputs.len(), 1);
    assert_eq!(run.outputs[0].from_physical, 31_001);
    assert!(run.naturally_quiescent);
}

#[test]
fn sensorimotor_candidate_consolidates_actual_surface_consequence_for_reverse_execution() {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorSynthesis);
    let action = junction(&mut builder, 32_000, 0, 0, 1);
    let surface = junction(&mut builder, 32_001, 2, 0, 1);
    let unrelated = junction(&mut builder, 32_002, 20, 0, 1);
    let motor = junction(&mut builder, 32_010, 1, 0, 2);
    let sink = junction(&mut builder, 32_011, 1, OUTWARD_REGION, 1);
    let outcome = junction(&mut builder, 32_012, 50, 0, 1);
    let anchor = junction(&mut builder, 32_013, 100, 0, 99);
    for target in [action, surface, unrelated, outcome] {
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
        surface,
        outcome,
        3,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    link(
        &mut builder,
        unrelated,
        outcome,
        3,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    link(
        &mut builder,
        motor,
        sink,
        0,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    builder.set_outcome_source_for_output(motor, outcome);
    let mut harness = builder.build();
    harness.send(&[input(action, 0), input(motor, 2)]);
    let surface_tick = harness.read().clock.tick.saturating_add(1);
    let consequence = harness.send(&[Input {
        arrival_tick: surface_tick,
        phase: 0,
        origin_physical: 32_001,
        target: surface,
        impulse: 1,
    }]);
    assert!(consequence.work.local_return_updates > 0);
    let checkpoint = harness.save().expect("surface checkpoint saves");

    let recall_tick = harness.read().clock.tick.saturating_add(1);
    let recalled = harness.send(&[Input {
        arrival_tick: recall_tick,
        phase: 0,
        origin_physical: 32_001,
        target: surface,
        impulse: 1,
    }]);
    let mut control = Harness::restore(checkpoint).expect("surface checkpoint restores");
    let unrelated_run = control.send(&[Input {
        arrival_tick: recall_tick,
        phase: 0,
        origin_physical: 32_002,
        target: unrelated,
        impulse: 1,
    }]);

    assert!(recalled
        .outputs
        .iter()
        .any(|output| output.from_physical == 32_010));
    assert!(unrelated_run.outputs.is_empty());
    assert!(recalled.naturally_quiescent);
    assert!(unrelated_run.naturally_quiescent);
}

fn recursive_learner_world(
    protocol: Protocol,
) -> (Harness, JunctionId, JunctionId, JunctionId, JunctionId) {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
    builder.set_protocol(protocol);
    builder.set_physical_tracing(true);
    let action = junction(&mut builder, 35_000, 0, 0, 1);
    let surface = junction(&mut builder, 35_001, 2, 0, 1);
    let unrelated = junction(&mut builder, 35_002, 20, 0, 1);
    let motor = junction(&mut builder, 35_010, 1, 0, 2);
    let sink = junction(&mut builder, 35_011, 1, OUTWARD_REGION, 1);
    let outcome = junction(&mut builder, 35_012, 50, 0, 1);
    let anchor = junction(&mut builder, 35_013, 100, 0, 99);
    for target in [action, surface, unrelated, outcome] {
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
    for source in [surface, unrelated] {
        link(
            &mut builder,
            source,
            outcome,
            3,
            1,
            u32::MAX,
            TransmissionMode::Drive,
        );
    }
    link(
        &mut builder,
        motor,
        sink,
        0,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    builder.set_outcome_source_for_output(motor, outcome);
    (builder.build(), action, surface, unrelated, motor)
}

fn send_physical(harness: &mut Harness, target: JunctionId, physical: u64) -> Run {
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: physical,
        target,
        impulse: 1,
    }])
}

#[test]
fn recursive_learner_constructs_once_from_repeated_actual_closure_and_replays() {
    let (mut harness, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConstruction);
    harness.send(&[input(action, 0), input(motor, 2)]);

    let first = send_physical(&mut harness, surface, 35_001);
    assert_eq!(first.work.causal_closure_observations, 1);
    assert_eq!(first.work.learner_constructions, 0);
    assert!(harness.read().learners.is_empty());

    let checkpoint = harness.save().expect("closure checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("closure checkpoint restores");
    let retrain_tick = harness.read().clock.tick.saturating_add(1);
    let retrain = [
        Input {
            arrival_tick: retrain_tick,
            phase: 0,
            origin_physical: 35_000,
            target: action,
            impulse: 1,
        },
        Input {
            arrival_tick: retrain_tick.saturating_add(2),
            phase: 0,
            origin_physical: 35_010,
            target: motor,
            impulse: 1,
        },
    ];
    assert_eq!(harness.send(&retrain), replay.send(&retrain));
    let second = send_physical(&mut harness, surface, 35_001);
    let replayed = send_physical(&mut replay, surface, 35_001);
    assert_eq!(second, replayed);
    assert_eq!(second.work.causal_closure_observations, 1);
    assert_eq!(second.work.learner_constructions, 1);
    assert!(second.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::LearnerConstructed { parent: None, .. }
    )));
    let observed = harness.read();
    assert_eq!(observed.learners.len(), 1);
    assert_eq!(observed.learners[0].parent, None);
    assert!(observed.learners[0].junctions.contains(&surface));
    assert!(observed.learners[0].junctions.contains(&motor));
    assert_eq!(
        harness.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );

    let duplicate = send_physical(&mut harness, surface, 35_001);
    assert_eq!(duplicate.work.causal_closure_observations, 0);
    assert_eq!(duplicate.work.learner_constructions, 0);
    assert_eq!(harness.read().learners.len(), 1);
    assert!(duplicate.naturally_quiescent);
}

#[test]
fn recursive_learner_rejects_noncausal_and_single_closure_controls() {
    let (mut harness, action, surface, unrelated, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConstruction);
    harness.send(&[input(action, 0), input(motor, 2)]);
    let one = send_physical(&mut harness, surface, 35_001);
    assert_eq!(one.work.causal_closure_observations, 1);
    let unrelated_run = send_physical(&mut harness, unrelated, 35_002);
    assert_eq!(unrelated_run.work.causal_closure_observations, 0);
    assert!(harness.read().learners.is_empty());

    let (mut accepted, action, surface, _, motor) =
        recursive_learner_world(Protocol::SensorimotorSynthesis);
    accepted.send(&[input(action, 0), input(motor, 2)]);
    send_physical(&mut accepted, surface, 35_001);
    send_physical(&mut accepted, surface, 35_001);
    assert!(accepted.read().learners.is_empty());
}

fn candidate_structural_run(dormant_outputs: usize) -> Run {
    let capacity = u32::try_from(dormant_outputs.saturating_mul(2).saturating_add(32)).unwrap();
    let mut builder =
        HarnessBuilder::with_capacity(capacity, capacity.saturating_mul(4), OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorSynthesis);
    let source = junction(&mut builder, 33_000, 0, 0, 1);
    let local = junction(&mut builder, 33_001, 1, 0, 2);
    let local_sink = junction(&mut builder, 33_002, 1, OUTWARD_REGION, 1);
    link(
        &mut builder,
        local,
        local_sink,
        0,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    for index in 0..dormant_outputs {
        let position = 100_i32.saturating_add(i32::try_from(index).unwrap());
        let physical = 34_000_u64.saturating_add(u64::try_from(index).unwrap());
        let output = junction(&mut builder, physical, position, 0, 2);
        let sink = junction(
            &mut builder,
            physical.saturating_add(10_000),
            position,
            OUTWARD_REGION,
            1,
        );
        link(
            &mut builder,
            output,
            sink,
            0,
            1,
            u32::MAX,
            TransmissionMode::Drive,
        );
    }
    builder.build().send(&[input(source, 0)])
}

#[test]
fn sensorimotor_candidate_structural_work_follows_the_local_source_neighborhood() {
    let small = candidate_structural_run(4);
    let large = candidate_structural_run(1_024);

    assert_eq!(small.work.local_junction_proposals, 2);
    assert_eq!(large.work.local_junction_proposals, 2);
    assert_eq!(
        small.execution_cost.local_structural_scans,
        large.execution_cost.local_structural_scans
    );
    assert!(large.execution_cost.local_structural_scans <= 8);
    assert!(small.naturally_quiescent);
    assert!(large.naturally_quiescent);
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
