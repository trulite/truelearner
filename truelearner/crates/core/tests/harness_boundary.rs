use std::collections::{BTreeSet, HashSet};
use truelearner_core::{
    classify_learner_ownership_relation, CandidateOwnership, CausalOriginResolution, Checkpoint,
    CheckpointError, CompletedCycleState, Harness, HarnessBuilder, Input, Junction, JunctionId,
    LearnerId, LearnerObservation, LearnerOwnershipRelation, Link, LinkId, OutputChoiceBasis,
    PhysicalEvent, PhysicalIncidence, PhysicalInput, Protocol, ReturnOriginDecision,
    ReversePathDecision, Run, TransmissionMode, TransmissionTrigger,
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

    fn transition_outcome_for(&mut self, output: u64, delay: i64) -> Run {
        let index = self
            .physical_outputs
            .iter()
            .position(|physical| *physical == output)
            .expect("output belongs to the local pair");
        let tick = self.harness.read().clock.tick.saturating_add(delay);
        self.harness.advance_to(tick);
        self.harness.send_physical(&[PhysicalInput {
            input: Input {
                origin_physical: 28_000,
                ..input(self.outcomes[index], tick)
            },
            incidence: PhysicalIncidence::Transition,
        }])
    }

    fn recall(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[input(self.input, tick)])
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
fn bounded_fresh_opportunity_exposes_one_balanced_local_candidate() {
    let mut candidate = LocalOutcomePairWorld::new(Protocol::RecursiveLearnerFreshOpportunity);
    let first = candidate.stimulate().outputs[0].from_physical;
    let returned = candidate.transition_outcome_for(first, 1);
    assert!(returned.work.local_return_updates > 0, "{returned:#?}");
    assert_eq!(candidate.stimulate().outputs[0].from_physical, first);
    candidate
        .harness
        .advance_to(candidate.harness.read().clock.tick.saturating_add(10));
    let alternative = candidate.recall();

    assert_eq!(alternative.outputs.len(), 1, "{alternative:#?}");
    assert_ne!(alternative.outputs[0].from_physical, first);
    let transfer =
        alternative
            .physical_trace
            .iter()
            .find_map(|transition| match transition.event {
                PhysicalEvent::FreshOpportunityTransferred {
                    donor,
                    recipient,
                    return_link,
                    owner,
                    opportunity,
                } => Some((donor, recipient, return_link, owner, opportunity)),
                _ => None,
            });
    let (donor, recipient, return_link, owner, opportunity) =
        transfer.expect("one fresh opportunity transfers");
    assert_ne!(donor, recipient);
    assert!(owner.is_none());
    assert_eq!(opportunity, i64::try_from(UNIT_U64).unwrap());
    assert!(alternative.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnSuperseded { link } if link == return_link
    )));
    assert!(alternative.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::OutputCandidateEvaluated {
            target,
            positive_path_strength: UNIT_U64,
            negative_path_strength: UNIT_U64,
            supplied_opportunity,
            executable: true,
            ..
        } if target == recipient && supplied_opportunity == i64::try_from(UNIT_U64).unwrap()
    )));
    assert!(alternative.naturally_quiescent);
    candidate
        .harness
        .advance_to(candidate.harness.read().clock.tick.saturating_add(10));
    let repeated = candidate.recall();
    assert!(!repeated.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::FreshOpportunityTransferred { return_link: observed, .. }
            if observed == return_link
    )));
}

#[test]
fn opportunity_owner_relation_partitions_actual_ancestry() {
    let learner = |id: u64, parent: Option<u64>| LearnerObservation {
        id: LearnerId(id),
        parent: parent.map(LearnerId),
        surface: JunctionId(0),
        output: JunctionId(0),
        junctions: Vec::new(),
        links: Vec::new(),
    };
    let learners = [
        learner(1, None),
        learner(2, None),
        learner(3, Some(1)),
        learner(4, Some(1)),
        learner(5, Some(3)),
    ];
    let classify = |donor: Option<u64>, recipient: Option<u64>| {
        classify_learner_ownership_relation(
            donor.map(LearnerId),
            recipient.map(LearnerId),
            &learners,
        )
    };

    assert_eq!(classify(None, None), LearnerOwnershipRelation::SameOwner);
    assert_eq!(
        classify(Some(1), Some(1)),
        LearnerOwnershipRelation::SameOwner
    );
    assert_eq!(
        classify(None, Some(1)),
        LearnerOwnershipRelation::OrganismToRoot
    );
    assert_eq!(
        classify(Some(1), None),
        LearnerOwnershipRelation::RootToOrganism
    );
    assert_eq!(
        classify(Some(1), Some(3)),
        LearnerOwnershipRelation::ParentToChild
    );
    assert_eq!(
        classify(Some(3), Some(1)),
        LearnerOwnershipRelation::ChildToParent
    );
    assert_eq!(
        classify(Some(3), Some(4)),
        LearnerOwnershipRelation::Siblings
    );
    assert_eq!(
        classify(Some(1), Some(2)),
        LearnerOwnershipRelation::Unrelated
    );
    assert_eq!(classify(None, Some(5)), LearnerOwnershipRelation::Unrelated);
    assert_eq!(
        classify(Some(9), Some(9)),
        LearnerOwnershipRelation::Unrelated
    );
}

#[test]
fn organism_root_fresh_opportunity_is_protocol_scoped() {
    let strict = Protocol::RecursiveLearnerFreshOpportunity;
    let root = Protocol::RecursiveLearnerRootFreshOpportunity;
    let parent = Protocol::RecursiveLearnerPhysicalTransitionReturn;

    for relation in [
        LearnerOwnershipRelation::SameOwner,
        LearnerOwnershipRelation::OrganismToRoot,
        LearnerOwnershipRelation::RootToOrganism,
        LearnerOwnershipRelation::ParentToChild,
        LearnerOwnershipRelation::ChildToParent,
        LearnerOwnershipRelation::Siblings,
        LearnerOwnershipRelation::Unrelated,
    ] {
        assert_eq!(
            strict.admits_fresh_opportunity_relation(relation),
            relation == LearnerOwnershipRelation::SameOwner
        );
        assert_eq!(
            root.admits_fresh_opportunity_relation(relation),
            matches!(
                relation,
                LearnerOwnershipRelation::SameOwner | LearnerOwnershipRelation::OrganismToRoot
            )
        );
        assert!(!parent.admits_fresh_opportunity_relation(relation));
    }

    let mut candidate = LocalOutcomePairWorld::new(root);
    let first = candidate.stimulate().outputs[0].from_physical;
    candidate.transition_outcome_for(first, 1);
    candidate.stimulate();
    candidate
        .harness
        .advance_to(candidate.harness.read().clock.tick.saturating_add(10));
    let alternative = candidate.recall();
    assert_ne!(alternative.outputs[0].from_physical, first);
    assert!(alternative.naturally_quiescent);
}

#[test]
fn physical_transition_continuation_is_cumulative_and_protocol_scoped() {
    let protocol = Protocol::RecursiveLearnerTransitionContinuation;
    assert!(protocol.admits_fresh_opportunity_relation(LearnerOwnershipRelation::SameOwner));
    assert!(protocol.admits_fresh_opportunity_relation(LearnerOwnershipRelation::OrganismToRoot));
    assert!(!protocol.admits_fresh_opportunity_relation(LearnerOwnershipRelation::Siblings));

    let mut world = LocalOutcomePairWorld::new(protocol);
    let first = world.stimulate().outputs[0].from_physical;
    world.transition_outcome_for(first, 1);
    world.stimulate();
    world
        .harness
        .advance_to(world.harness.read().clock.tick.saturating_add(10));
    let alternative = world.recall();
    assert_ne!(alternative.outputs[0].from_physical, first);
    assert!(alternative.naturally_quiescent);
}

#[test]
fn coherent_unresolved_effect_holds_once_then_releases() {
    let mut coherent = LocalOutcomePairWorld::new(Protocol::RecursiveLearnerCoherentEffect);
    let first = coherent.stimulate().outputs[0].from_physical;
    coherent.transition_outcome_for(first, 1);
    assert_eq!(coherent.stimulate().outputs[0].from_physical, first);
    let held = coherent.stimulate();
    assert_eq!(held.outputs[0].from_physical, first, "{held:#?}");
    assert!(held.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::CoherentEffectEvaluated {
            latest_unanswered_opened_tick: Some(_),
            unanswered_returns: 1,
            admitted: true,
            ..
        }
    )));

    coherent
        .harness
        .advance_to(coherent.harness.read().clock.tick.saturating_add(10));
    let released = coherent.stimulate();
    assert_ne!(released.outputs[0].from_physical, first, "{released:#?}");

    let mut parent = LocalOutcomePairWorld::new(Protocol::RecursiveLearnerRootFreshOpportunity);
    let parent_first = parent.stimulate().outputs[0].from_physical;
    parent.transition_outcome_for(parent_first, 1);
    assert_eq!(parent.stimulate().outputs[0].from_physical, parent_first);
    assert_ne!(parent.stimulate().outputs[0].from_physical, parent_first);
    assert!(!parent
        .stimulate()
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::CoherentEffectEvaluated { .. }
        )));
    assert!(held.naturally_quiescent && released.naturally_quiescent);
}

#[test]
fn completed_cycle_composes_truthful_returns_but_not_samples() {
    let mut world = LocalOutcomePairWorld::new(Protocol::RecursiveLearnerCompletedCycle);
    let first = world.stimulate().outputs[0].from_physical;

    let first_return = world.transition_outcome_for(first, 1);
    assert!(first_return.naturally_quiescent);
    assert!(first_return
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::PhysicalTransitionEligibilityEvaluated { eligible: true, .. }
        )));
    let first_successor = world.stimulate();
    assert_eq!(
        first_successor.outputs[0].from_physical, first,
        "{:#?}",
        first_successor.physical_trace
    );

    world.transition_outcome_for(first, 1);
    let second_successor = world.stimulate();
    assert_eq!(second_successor.outputs[0].from_physical, first);

    let rejected_sample = world.outcome_for(first, 1);
    assert!(rejected_sample
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::PhysicalTransitionEligibilityEvaluated {
                eligible: false,
                ..
            }
        )));
    world
        .harness
        .advance_to(world.harness.read().clock.tick.saturating_add(10));
    let released = world.stimulate();
    assert_ne!(released.outputs[0].from_physical, first, "{released:#?}");

    let mut parent = LocalOutcomePairWorld::new(Protocol::RecursiveLearnerCoherentEffect);
    let parent_first = parent.stimulate().outputs[0].from_physical;
    parent.transition_outcome_for(parent_first, 1);
    assert!(!parent
        .stimulate()
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::CompletedCycleContinuationEvaluated { .. }
        )));
    assert!(first_successor.naturally_quiescent);
    assert!(second_successor.naturally_quiescent);
    assert!(released.naturally_quiescent);
}

#[test]
fn output_choice_resolution_matches_existing_admissions() {
    let mut world = LocalOutcomePairWorld::new(Protocol::RecursiveLearnerCompletedCycle);
    let first = world.stimulate().outputs[0].from_physical;
    world.transition_outcome_for(first, 1);
    world.stimulate();
    let observed = world.stimulate();

    let resolutions = observed
        .physical_trace
        .iter()
        .filter_map(|transition| match &transition.event {
            PhysicalEvent::OutputChoiceResolved {
                admitted,
                admission_basis,
                completed_cycle_state,
                ..
            } => Some((
                transition.tick,
                transition.phase,
                admitted,
                *admission_basis,
                *completed_cycle_state,
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(!resolutions.is_empty());
    for (tick, phase, admitted, basis, completed_state) in resolutions {
        let existing = observed
            .physical_trace
            .iter()
            .filter(|transition| transition.tick == tick && transition.phase == phase)
            .filter_map(|transition| match transition.event {
                PhysicalEvent::CandidateSelection {
                    target,
                    admitted: true,
                    ..
                } => Some(target),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        let grouped = admitted
            .iter()
            .map(|admission| admission.target)
            .collect::<BTreeSet<_>>();
        assert_eq!(grouped, existing);
        assert!(matches!(
            basis,
            OutputChoiceBasis::CoherentEffect
                | OutputChoiceBasis::CompletedCycle
                | OutputChoiceBasis::RecentCohort
                | OutputChoiceBasis::Ordinary
        ));
        assert!(matches!(
            completed_state,
            CompletedCycleState::Missing
                | CompletedCycleState::Unique
                | CompletedCycleState::AmbiguousLatest
                | CompletedCycleState::Stale
        ));
    }
}

#[test]
fn bounded_fresh_opportunity_controls_answered_old_protocol_and_reflection() {
    let prepare = |protocol| {
        let mut world = LocalOutcomePairWorld::new(protocol);
        let first = world.stimulate().outputs[0].from_physical;
        world.transition_outcome_for(first, 1);
        assert_eq!(world.stimulate().outputs[0].from_physical, first);
        (world, first)
    };

    let (mut answered, answered_first) = prepare(Protocol::RecursiveLearnerFreshOpportunity);
    let (mut answered_parent, _) = prepare(Protocol::RecursiveLearnerPhysicalTransitionReturn);
    answered.transition_outcome_for(answered_first, 1);
    answered_parent.transition_outcome_for(answered_first, 1);
    answered
        .harness
        .advance_to(answered.harness.read().clock.tick.saturating_add(10));
    answered_parent
        .harness
        .advance_to(answered_parent.harness.read().clock.tick.saturating_add(10));
    let answered_run = answered.recall();
    let answered_parent_run = answered_parent.recall();
    assert_eq!(answered_run.outputs, answered_parent_run.outputs);
    assert!(!answered_run
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::FreshOpportunityTransferred { .. }
        )));

    let (mut parent, parent_first) = prepare(Protocol::RecursiveLearnerPhysicalTransitionReturn);
    parent
        .harness
        .advance_to(parent.harness.read().clock.tick.saturating_add(10));
    let parent_run = parent.recall();
    assert_eq!(parent_run.outputs[0].from_physical, parent_first);
    assert!(!parent_run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::FreshOpportunityTransferred { .. }
    )));

    let (mut reflected, reflected_first) = {
        let mut world = LocalOutcomePairWorld::with_positions(
            Protocol::RecursiveLearnerFreshOpportunity,
            [1, -1],
        );
        let first = world.stimulate().outputs[0].from_physical;
        world.transition_outcome_for(first, 1);
        world.stimulate();
        (world, first)
    };
    reflected
        .harness
        .advance_to(reflected.harness.read().clock.tick.saturating_add(10));
    let reflected_run = reflected.recall();
    assert_ne!(reflected_run.outputs[0].from_physical, reflected_first);
    assert!(reflected_run.naturally_quiescent);
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

fn boundary_novelty_sibling_world(
    second_position: i32,
) -> (Harness, JunctionId, [JunctionId; 2], JunctionId) {
    boundary_novelty_sibling_world_with_protocol(
        second_position,
        Protocol::RecursiveLearnerBoundaryNovelty,
    )
}

fn boundary_novelty_sibling_world_with_protocol(
    second_position: i32,
    protocol: Protocol,
) -> (Harness, JunctionId, [JunctionId; 2], JunctionId) {
    let mut builder = HarnessBuilder::with_capacity(96, 256, OUTWARD_REGION);
    builder.set_protocol(protocol);
    builder.set_physical_tracing(true);
    let action = junction(&mut builder, 39_000, 0, 0, 1);
    let surfaces = [
        junction(&mut builder, 39_001, 2, 0, 1),
        junction(&mut builder, 39_002, second_position, 0, 1),
    ];
    let motor = junction(&mut builder, 39_010, 1, 0, 2);
    let sink = junction(&mut builder, 39_011, 1, OUTWARD_REGION, 1);
    let outcome = junction(&mut builder, 39_012, 50, 0, 1);
    let anchor = junction(&mut builder, 39_013, 100, 0, 99);
    for target in [action, surfaces[0], surfaces[1], outcome] {
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
    for surface in surfaces {
        link(
            &mut builder,
            surface,
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
    (builder.build(), action, surfaces, motor)
}

fn observe_boundary_sibling_closure(
    harness: &mut Harness,
    action: JunctionId,
    surface: JunctionId,
    surface_physical: u64,
    motor: JunctionId,
) -> Run {
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 39_000,
            target: action,
            impulse: 1,
        },
        Input {
            arrival_tick: tick.saturating_add(2),
            phase: 0,
            origin_physical: 39_010,
            target: motor,
            impulse: 1,
        },
    ]);
    send_physical(harness, surface, surface_physical)
}

fn recursive_control_world() -> (
    Harness,
    JunctionId,
    JunctionId,
    JunctionId,
    JunctionId,
    JunctionId,
    JunctionId,
    JunctionId,
) {
    recursive_control_world_with_protocol(Protocol::RecursiveLearnerConstruction)
}

fn recursive_control_world_with_protocol(
    protocol: Protocol,
) -> (
    Harness,
    JunctionId,
    JunctionId,
    JunctionId,
    JunctionId,
    JunctionId,
    JunctionId,
    JunctionId,
) {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
    builder.set_protocol(protocol);
    builder.set_physical_tracing(true);
    let action = junction(&mut builder, 36_000, 0, 0, 1);
    let surface = junction(&mut builder, 36_001, 2, 0, 1);
    let unrelated = junction(&mut builder, 36_002, 20, 0, 1);
    let motor = junction(&mut builder, 36_010, 1, 0, 3);
    let sink = junction(&mut builder, 36_011, 1, OUTWARD_REGION, 1);
    let outcome = junction(&mut builder, 36_012, 50, 0, 1);
    let anchor = junction(&mut builder, 36_013, 100, 0, 99);
    let controlled = junction(&mut builder, 36_020, 30, 0, 2);
    let controlled_sink = junction(&mut builder, 36_021, 30, OUTWARD_REGION, 1);
    let controlled_path = junction(&mut builder, 36_022, 2, 0, 1);
    let controlled_outcome = junction(&mut builder, 36_023, 60, 0, 1);
    for target in [action, surface, unrelated, outcome, controlled_outcome] {
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
    link(
        &mut builder,
        surface,
        controlled_path,
        0,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    link(
        &mut builder,
        controlled_path,
        controlled,
        2,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    link(
        &mut builder,
        controlled,
        controlled_sink,
        0,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    builder.set_outcome_source_for_output(motor, outcome);
    builder.set_outcome_source_for_output(controlled, controlled_outcome);
    (
        builder.build(),
        action,
        surface,
        unrelated,
        motor,
        outcome,
        controlled,
        controlled_outcome,
    )
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

fn observe_recursive_closure(
    harness: &mut Harness,
    action: JunctionId,
    surface: JunctionId,
    motor: JunctionId,
) -> Run {
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_000,
            target: action,
            impulse: 1,
        },
        Input {
            arrival_tick: tick.saturating_add(2),
            phase: 0,
            origin_physical: 35_010,
            target: motor,
            impulse: 1,
        },
    ]);
    send_physical(harness, surface, 35_001)
}

fn observe_recursive_transition_closure(
    harness: &mut Harness,
    action: JunctionId,
    surface: JunctionId,
    motor: JunctionId,
) -> Run {
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_000,
            target: action,
            impulse: 1,
        },
        Input {
            arrival_tick: tick.saturating_add(2),
            phase: 0,
            origin_physical: 35_010,
            target: motor,
            impulse: 1,
        },
    ]);
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send_physical(&[PhysicalInput {
        input: Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_001,
            target: surface,
            impulse: 1,
        },
        incidence: PhysicalIncidence::Transition,
    }])
}

fn observe_control_closure(
    harness: &mut Harness,
    action: JunctionId,
    surface: JunctionId,
    motor: JunctionId,
) -> Run {
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 36_000,
            target: action,
            impulse: 1,
        },
        Input {
            arrival_tick: tick.saturating_add(2),
            phase: 0,
            origin_physical: 36_010,
            target: motor,
            impulse: 2,
        },
    ]);
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 36_001,
        target: surface,
        impulse: 1,
    }])
}

fn stimulate_recursive_control(
    harness: &mut Harness,
    surface: JunctionId,
    controlled: JunctionId,
) -> Run {
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 36_001,
            target: surface,
            impulse: 1,
        },
        Input {
            arrival_tick: tick.saturating_add(2),
            phase: 0,
            origin_physical: 36_001,
            target: controlled,
            impulse: 1,
        },
    ])
}

fn return_recursive_control(harness: &mut Harness, outcome: JunctionId) -> Run {
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 36_001,
        target: outcome,
        impulse: 1,
    }])
}

fn expire_recursive_returns(harness: &mut Harness, unrelated: JunctionId) -> Run {
    let tick = harness.read().clock.tick.saturating_add(64);
    harness.send(&[Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 35_002,
        target: unrelated,
        impulse: 1,
    }])
}

fn owned_return_admission(run: &Run, admitted: bool) -> (LearnerId, LinkId, u32) {
    run.physical_trace
        .iter()
        .find_map(|transition| match transition.event {
            PhysicalEvent::ReturnOriginAdmission {
                owner: Some(owner),
                link,
                generation,
                admitted: observed,
                ..
            } if observed == admitted => Some((owner, link, generation)),
            _ => None,
        })
        .expect("run contains the expected owner-local return admission")
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
    assert_eq!(duplicate.work.causal_closure_observations, 1);
    assert_eq!(duplicate.work.learner_constructions, 0);
    assert!(duplicate.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::CausalClosureObserved {
            parent: Some(parent),
            evidence: 1,
            ..
        } if parent == observed.learners[0].id
    )));
    assert_eq!(harness.read().learners.len(), 1);
    assert!(duplicate.naturally_quiescent);
}

#[test]
fn construction_outcome_composition_preserves_the_same_tick_exact_lineage() {
    let protocol = Protocol::RecursiveLearnerConstructionOutcomeComposition;
    let (mut candidate, action, surface, _, motor) = recursive_learner_world(protocol);

    let first = observe_recursive_transition_closure(&mut candidate, action, surface, motor);
    assert_eq!(first.work.learner_constructions, 0);
    let checkpoint = candidate.save().expect("composition checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("composition checkpoint restores");

    let constructed = observe_recursive_transition_closure(&mut candidate, action, surface, motor);
    let replayed = observe_recursive_transition_closure(&mut replay, action, surface, motor);
    assert_eq!(constructed, replayed);
    let (construction_index, construction_tick, learner) = constructed
        .physical_trace
        .iter()
        .enumerate()
        .find_map(|(index, transition)| match transition.event {
            PhysicalEvent::LearnerConstructed { learner, .. } => {
                Some((index, transition.tick, learner))
            }
            _ => None,
        })
        .expect("the second closure constructs a learner");
    let learner_links = candidate
        .read()
        .learners
        .into_iter()
        .find(|state| state.id == learner)
        .expect("constructed learner is observable")
        .links;
    let projected = constructed
        .physical_trace
        .iter()
        .skip(construction_index.saturating_add(1))
        .take_while(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::LearnerConsequenceRecorded { owner, .. } if owner == learner
            )
        })
        .filter_map(|transition| match transition.event {
            PhysicalEvent::LearnerConsequenceRecorded {
                owner,
                link,
                generation,
                tick,
            } if owner == learner => Some((transition.tick, link, generation, tick)),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert!(!projected.is_empty(), "{:#?}", constructed.physical_trace);
    assert!(
        projected
            .iter()
            .all(|(event_tick, link, _, remembered_tick)| {
                *event_tick == construction_tick
                    && *remembered_tick == construction_tick
                    && learner_links.contains(link)
                    && constructed.physical_trace.iter().any(|transition| {
                        transition.tick == construction_tick
                            && (matches!(
                                transition.event,
                                PhysicalEvent::ConsequenceRecorded { link: written, .. }
                                    if written == *link
                            ) || matches!(
                                transition.event,
                                PhysicalEvent::ReversePathConsolidated { link: written, .. }
                                    if written == *link
                            ))
                    })
            }),
        "projected {projected:?}, construction tick {construction_tick}, learner links {learner_links:?}"
    );
    assert_eq!(
        candidate.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );
    assert!(constructed.naturally_quiescent);

    let (mut parent, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerCompletedCycle);
    observe_recursive_transition_closure(&mut parent, action, surface, motor);
    let parent_constructed =
        observe_recursive_transition_closure(&mut parent, action, surface, motor);
    let parent_learner = parent_constructed
        .physical_trace
        .iter()
        .find_map(|transition| match transition.event {
            PhysicalEvent::LearnerConstructed { learner, .. } => Some(learner),
            _ => None,
        })
        .expect("the parent protocol constructs the same boundary");
    assert!(!parent_constructed.physical_trace.iter().any(|transition| {
        matches!(
            transition.event,
            PhysicalEvent::LearnerConsequenceRecorded { owner, .. }
                if owner == parent_learner
        )
    }));
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

#[test]
fn recursive_learner_children_start_with_fresh_return_memory() {
    let (mut harness, action, surface, unrelated, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConstruction);

    let root_first = observe_recursive_closure(&mut harness, action, surface, motor);
    assert_eq!(root_first.work.causal_closure_observations, 1);
    let root_second = observe_recursive_closure(&mut harness, action, surface, motor);
    assert_eq!(root_second.work.learner_constructions, 1);
    let root = harness.read().learners[0].id;
    let parent_return_history = harness
        .read()
        .links
        .iter()
        .map(|link| (link.id, link.return_origins.clone()))
        .collect::<Vec<_>>();

    let child_first = observe_recursive_closure(&mut harness, action, surface, motor);
    assert_eq!(child_first.work.causal_closure_observations, 1);
    assert_eq!(child_first.work.learner_constructions, 0);
    assert!(child_first.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::CausalClosureObserved {
            parent: Some(parent),
            evidence: 1,
            ..
        } if parent == root
    )));
    assert!(child_first.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnScheduling {
            owner: Some(owner),
            admitted: true,
            ..
        } if owner == root
    )));
    let (first_owner, return_link, first_generation) = owned_return_admission(&child_first, true);
    assert_eq!(first_owner, root);
    let after_child = harness.read();
    for (link, return_origins) in &parent_return_history {
        assert_eq!(
            after_child
                .links
                .iter()
                .find(|state| state.id == *link)
                .map(|state| &state.return_origins),
            Some(return_origins),
            "child-local return admission changed parent history for {link:?}"
        );
    }

    let checkpoint = harness.save().expect("child return memory saves");
    let mut replay = Harness::restore(checkpoint).expect("child return memory restores");
    let duplicate = observe_recursive_closure(&mut harness, action, surface, motor);
    let duplicate_replayed = observe_recursive_closure(&mut replay, action, surface, motor);
    assert_eq!(duplicate, duplicate_replayed);
    assert_eq!(duplicate.work.causal_closure_observations, 1);
    assert_eq!(duplicate.work.learner_constructions, 1);
    assert_eq!(
        owned_return_admission(&duplicate, false),
        (root, return_link, first_generation)
    );
    let (second_owner, second_link, second_generation) = owned_return_admission(&duplicate, true);
    assert_eq!(second_owner, root);
    assert_ne!(
        (second_link, second_generation),
        (return_link, first_generation)
    );
    assert_eq!(harness.read().learners.len(), 2);
    assert_eq!(
        harness.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );

    let grandchild_first = observe_recursive_closure(&mut harness, action, surface, motor);
    assert_eq!(grandchild_first.work.causal_closure_observations, 1);
    assert_eq!(grandchild_first.work.learner_constructions, 0);
    let (grandchild_owner, grandchild_link, grandchild_generation) =
        owned_return_admission(&grandchild_first, true);
    assert_eq!(grandchild_owner, harness.read().learners[1].id);
    let grandchild_duplicate = observe_recursive_closure(&mut harness, action, surface, motor);
    assert_eq!(grandchild_duplicate.work.causal_closure_observations, 1);
    assert_eq!(grandchild_duplicate.work.learner_constructions, 1);
    assert_eq!(
        owned_return_admission(&grandchild_duplicate, false),
        (grandchild_owner, grandchild_link, grandchild_generation)
    );
    let unrelated_return = send_physical(&mut harness, unrelated, 35_002);
    assert_eq!(unrelated_return.work.causal_closure_observations, 0);

    let renewal = expire_recursive_returns(&mut harness, unrelated);
    assert_eq!(renewal.work.causal_closure_observations, 0);
    let renewed = observe_recursive_closure(&mut harness, action, surface, motor);
    assert_eq!(renewed.work.causal_closure_observations, 1);
    assert_eq!(renewed.work.learner_constructions, 0);
    let (renewed_owner, _, renewed_generation) = owned_return_admission(&renewed, true);
    assert_eq!(renewed_owner, harness.read().learners[2].id);
    assert!(renewed_generation > 1);

    let learners = harness.read().learners;
    assert_eq!(learners.len(), 3);
    assert_eq!(learners[0].parent, None);
    assert_eq!(learners[1].parent, Some(learners[0].id));
    assert_eq!(learners[2].parent, Some(learners[1].id));
    assert!(learners
        .iter()
        .all(|learner| learner.junctions.contains(&surface)));
    assert!(duplicate.naturally_quiescent);
    assert!(grandchild_duplicate.naturally_quiescent);
    assert!(unrelated_return.naturally_quiescent);
}

#[test]
fn recursive_learner_proprioceptive_opportunity_is_current_and_owner_local() {
    let (mut harness, action, surface, unrelated, motor, _, controlled, _) =
        recursive_control_world();
    assert_eq!(
        observe_control_closure(&mut harness, action, surface, motor)
            .work
            .causal_closure_observations,
        1
    );
    assert_eq!(
        observe_control_closure(&mut harness, action, surface, motor)
            .work
            .learner_constructions,
        1
    );
    let root = harness.read().learners[0].id;
    let checkpoint = harness.save().expect("proprioceptive checkpoint saves");

    let tick = harness.read().clock.tick.saturating_add(1);
    let current = harness.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 36_001,
            target: surface,
            impulse: 1,
        },
        Input {
            arrival_tick: tick.saturating_add(2),
            phase: 0,
            origin_physical: 36_001,
            target: controlled,
            impulse: 1,
        },
    ]);
    assert!(current
        .outputs
        .iter()
        .any(|output| output.from_physical == 36_020));

    let mut absent = Harness::restore(checkpoint.clone()).expect("checkpoint restores");
    let tick = absent.read().clock.tick.saturating_add(1);
    let without = absent.send(&[Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 36_001,
        target: surface,
        impulse: 1,
    }]);
    assert!(without
        .outputs
        .iter()
        .all(|output| output.from_physical != 36_020));

    let mut shifted = Harness::restore(checkpoint.clone()).expect("checkpoint restores");
    let tick = shifted.read().clock.tick.saturating_add(1);
    let early = shifted.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 36_001,
            target: controlled,
            impulse: 1,
        },
        Input {
            arrival_tick: tick.saturating_add(1),
            phase: 0,
            origin_physical: 36_001,
            target: surface,
            impulse: 1,
        },
    ]);
    assert!(early
        .outputs
        .iter()
        .all(|output| output.from_physical != 36_020));

    let mut other = Harness::restore(checkpoint).expect("checkpoint restores");
    let tick = other.read().clock.tick.saturating_add(1);
    let unrelated_run = other.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 36_001,
            target: surface,
            impulse: 1,
        },
        Input {
            arrival_tick: tick.saturating_add(2),
            phase: 0,
            origin_physical: 36_002,
            target: controlled,
            impulse: 1,
        },
    ]);
    assert!(
        unrelated_run
            .outputs
            .iter()
            .all(|output| output.from_physical != 36_020),
        "{:#?}",
        unrelated_run.physical_trace
    );
    assert!(current.naturally_quiescent);
    assert!(without.naturally_quiescent);
    assert!(early.naturally_quiescent);
    assert!(unrelated_run.naturally_quiescent);
    assert_eq!(harness.read().learners[0].id, root);
    assert_eq!(other.read().learners[0].id, root);
    assert_ne!(unrelated, surface);
}

#[test]
fn recursive_learner_proprioceptive_consequence_is_private_recent_and_replayable() {
    let (mut harness, action, surface, _, motor, motor_outcome, controlled, controlled_outcome) =
        recursive_control_world();
    observe_control_closure(&mut harness, action, surface, motor);
    assert_eq!(
        observe_control_closure(&mut harness, action, surface, motor)
            .work
            .learner_constructions,
        1
    );
    let root = harness.read().learners[0].id;

    let trained_run = stimulate_recursive_control(&mut harness, surface, controlled);
    assert_eq!(trained_run.outputs.len(), 1);
    let trained = trained_run.outputs[0].from_physical;
    let trained_outcome = if trained == 36_010 {
        motor_outcome
    } else {
        assert_eq!(trained, 36_020);
        controlled_outcome
    };
    let consequence = return_recursive_control(&mut harness, trained_outcome);
    let consequence_tick = consequence
        .physical_trace
        .iter()
        .find_map(|transition| match transition.event {
            PhysicalEvent::LearnerConsequenceRecorded { owner, tick, .. } if owner == root => {
                Some(tick)
            }
            _ => None,
        })
        .expect("accepted root return records private consequence");
    let checkpoint = harness.save().expect("private consequence saves");
    let checkpoint_bytes = checkpoint.canonical_bytes().expect("checkpoint encodes");

    let physical_outputs = [36_010, 36_020];
    let predicted_tick = harness.read().clock.tick.saturating_add(2);
    let neutral = physical_outputs[predicted_tick.rem_euclid(2) as usize];
    if neutral == trained {
        harness.advance_to(harness.read().clock.tick.saturating_add(1));
    }
    let preferred = stimulate_recursive_control(&mut harness, surface, controlled);
    assert_eq!(preferred.outputs[0].from_physical, trained);
    assert!(preferred.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::LearnerCandidatePreference {
            owner,
            consequence_tick: Some(tick),
            admitted: true,
            ..
        } if owner == root && tick == consequence_tick
    )));

    let mut replay = Harness::restore(checkpoint.clone()).expect("private consequence restores");
    assert_eq!(
        replay.save().unwrap().canonical_bytes().unwrap(),
        checkpoint_bytes
    );
    let mut direct = Harness::restore(checkpoint.clone()).expect("comparison restores");
    assert_eq!(
        stimulate_recursive_control(&mut replay, surface, controlled),
        stimulate_recursive_control(&mut direct, surface, controlled)
    );

    let mut released_runs = Vec::new();
    for offset in 6..10 {
        let mut released =
            Harness::restore(checkpoint.clone()).expect("release checkpoint restores");
        released.advance_to(released.read().clock.tick.saturating_add(offset));
        released_runs.push(stimulate_recursive_control(
            &mut released,
            surface,
            controlled,
        ));
    }
    assert!(released_runs
        .iter()
        .any(|run| run.outputs[0].from_physical != trained));
    assert!(released_runs
        .iter()
        .all(|run| run.physical_trace.iter().any(|transition| matches!(
            transition.event,
            PhysicalEvent::LearnerCandidatePreference {
                owner,
                consequence_tick: Some(tick),
                ..
            } if owner == root
                && transition.tick.saturating_sub(tick) > 4
        ))));
    assert!(preferred.naturally_quiescent);
    assert!(released_runs.iter().all(|run| run.naturally_quiescent));
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

#[test]
fn physical_diagnostics_explain_origin_resolution_and_return_decisions() {
    let (mut merged, _, surface, _, _) =
        recursive_learner_world(Protocol::RecursiveLearnerConstruction);
    let merged_tick = merged.read().clock.tick.saturating_add(1);
    let merged_run = merged.send(&[
        Input {
            arrival_tick: merged_tick,
            phase: 0,
            origin_physical: 35_001,
            target: surface,
            impulse: 1,
        },
        Input {
            arrival_tick: merged_tick,
            phase: 0,
            origin_physical: 35_002,
            target: surface,
            impulse: 1,
        },
    ]);
    let observed_origins = merged_run
        .physical_diagnostics()
        .filter_map(|transition| match transition.event {
            PhysicalEvent::DriveOriginObserved {
                target,
                origin_physical,
                ..
            } if target == surface => Some(origin_physical),
            _ => None,
        })
        .collect::<HashSet<_>>();
    assert_eq!(observed_origins, HashSet::from([35_001, 35_002]));
    assert!(merged_run.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::CausalOriginResolved {
            target,
            distinct_origins: 2,
            resolved_origin: 35_001,
            resolution: CausalOriginResolution::JunctionFallback,
            ..
        } if target == surface
    )));

    let (mut admitted, action, admitted_surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConstruction);
    admitted.send(&[input(action, 0), input(motor, 2)]);
    let admitted_run = send_physical(&mut admitted, admitted_surface, 35_001);
    assert!(admitted_run
        .physical_diagnostics()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::ModulatoryOriginObserved {
                origin_physical: 35_001,
                ..
            }
        )));
    assert!(admitted_run
        .physical_diagnostics()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::ReturnOriginEvaluated {
                origin_physical: 35_001,
                decision: ReturnOriginDecision::AdmittedLocal,
                ..
            }
        )));

    let (mut rejected, action, _, unrelated, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConstruction);
    rejected.send(&[input(action, 0), input(motor, 2)]);
    let rejected_run = send_physical(&mut rejected, unrelated, 35_002);
    assert!(rejected_run
        .physical_diagnostics()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::ReturnOriginEvaluated {
                origin_physical: 35_002,
                decision: ReturnOriginDecision::RejectedNonLocal,
                distance: Some(distance),
                ..
            } if distance > LOCAL_VARIATION_RADIUS
        )));
}

#[test]
fn physical_diagnostics_explain_reverse_path_failures() {
    let (mut harness, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConstruction);
    harness.send(&[input(action, 0), input(motor, 2)]);

    let run = send_physical(&mut harness, surface, 35_012);
    assert!(run.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnOriginEvaluated {
            origin_physical: 35_012,
            decision: ReturnOriginDecision::AdmittedDirect,
            ..
        }
    )));
    assert!(run.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReversePathEvaluated {
            origin_physical: 35_012,
            decision: ReversePathDecision::OriginIsReturnSource,
            ..
        }
    )));
    assert!(!run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReversePathConsolidated { .. }
    )));
}

#[test]
fn consequence_born_return_rejects_preopening_origin_without_writing() {
    let protocol = Protocol::RecursiveLearnerConsequenceBornReturn;
    let (mut harness, action, surface, _, motor) = recursive_learner_world(protocol);
    let run = harness.send(&[
        Input {
            arrival_tick: 0,
            phase: 0,
            origin_physical: 35_001,
            target: surface,
            impulse: 1,
        },
        input(action, 0),
        input(motor, 2),
    ]);

    assert!(run.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnOriginEvaluated {
            origin_physical: 35_001,
            decision: ReturnOriginDecision::RejectedBeforeReturnOpened,
            ..
        }
    )));
    assert!(run.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ClosureEligibilityEvaluated {
            origin_physical: 35_001,
            eligible: false,
            ..
        }
    )));
    assert!(!run
        .physical_trace
        .iter()
        .any(|transition| matches!(transition.event, PhysicalEvent::ConsequenceRecorded { .. })));
    assert_eq!(run.work.local_return_updates, 0);
    assert!(run.naturally_quiescent);
}

#[test]
fn consequence_born_return_preserves_genuinely_later_local_return() {
    let protocol = Protocol::RecursiveLearnerConsequenceBornReturn;
    let (mut harness, action, surface, _, motor) = recursive_learner_world(protocol);
    let used = harness.send(&[input(action, 0), input(motor, 2)]);
    let checkpoint = harness.save().expect("consequence-born return saves");
    let mut replay = Harness::restore(checkpoint).expect("consequence-born return restores");
    let tick = harness.read().clock.tick.saturating_add(1);
    let input = Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 35_001,
        target: surface,
        impulse: 1,
    };
    let returned = harness.send(&[input]);
    let replayed = replay.send(&[input]);

    assert_eq!(returned, replayed);
    assert!(returned.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnOriginEvaluated {
            origin_physical: 35_001,
            decision: ReturnOriginDecision::AdmittedLocal,
            ..
        }
    )));
    assert!(returned
        .physical_trace
        .iter()
        .any(|transition| matches!(transition.event, PhysicalEvent::ConsequenceRecorded { .. })));
    assert!(returned.work.local_return_updates > 0);
    assert!(used.naturally_quiescent && returned.naturally_quiescent);
}

#[test]
fn physical_transition_return_rejects_later_sample_but_admits_matched_transition() {
    let protocol = Protocol::RecursiveLearnerPhysicalTransitionReturn;
    let (mut sample, action, surface, _, motor) = recursive_learner_world(protocol);
    sample.send(&[input(action, 0), input(motor, 2)]);
    let checkpoint = sample.save().expect("transition return fixture saves");
    let mut transitioned = Harness::restore(checkpoint).expect("transition fixture restores");
    let tick = sample.read().clock.tick.saturating_add(1);
    let later = Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 35_001,
        target: surface,
        impulse: 1,
    };

    let sampled = sample.send_physical(&[PhysicalInput {
        input: later,
        incidence: PhysicalIncidence::Sample,
    }]);
    let changed = transitioned.send_physical(&[PhysicalInput {
        input: later,
        incidence: PhysicalIncidence::Transition,
    }]);

    assert!(sampled.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnOriginEvaluated {
            origin_physical: 35_001,
            decision: ReturnOriginDecision::RejectedUnchangedSample,
            ..
        }
    )));
    assert!(!sampled
        .physical_trace
        .iter()
        .any(|transition| matches!(transition.event, PhysicalEvent::ConsequenceRecorded { .. })));
    assert_eq!(sampled.work.local_return_updates, 0);

    assert!(changed.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnOriginEvaluated {
            origin_physical: 35_001,
            decision: ReturnOriginDecision::AdmittedLocal,
            ..
        }
    )));
    assert!(changed
        .physical_trace
        .iter()
        .any(|transition| matches!(transition.event, PhysicalEvent::ConsequenceRecorded { .. })));
    assert!(changed.work.local_return_updates > 0);
    assert!(sampled.naturally_quiescent && changed.naturally_quiescent);
}

#[test]
fn physical_transition_return_preserves_sampling_for_old_protocols_and_replay() {
    let (mut ordinary, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConsequenceBornReturn);
    ordinary.send(&[input(action, 0), input(motor, 2)]);
    let checkpoint = ordinary.save().expect("old protocol fixture saves");
    let mut marked = Harness::restore(checkpoint).expect("old protocol fixture restores");
    let tick = ordinary.read().clock.tick.saturating_add(1);
    let later = Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 35_001,
        target: surface,
        impulse: 1,
    };

    let sampled = ordinary.send(&[later]);
    let transitioned = marked.send_physical(&[PhysicalInput {
        input: later,
        incidence: PhysicalIncidence::Transition,
    }]);

    assert_eq!(sampled.outputs, transitioned.outputs);
    assert_eq!(sampled.work, transitioned.work);
    assert_eq!(ordinary.read(), marked.read());
    assert!(transitioned
        .physical_diagnostics()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::PhysicalIncidenceObserved {
                origin_physical: 35_001,
                incidence: PhysicalIncidence::Transition,
                ..
            }
        )));

    let checkpoint = marked.save().expect("transition lineage saves");
    let restored = Harness::restore(checkpoint).expect("transition lineage restores");
    assert_eq!(
        marked.save().unwrap().canonical_bytes().unwrap(),
        restored.save().unwrap().canonical_bytes().unwrap()
    );
}

#[test]
fn physical_diagnostics_are_opt_in_pure_and_replayable() {
    let mut traced = PathWorld::with_protocol(1, true, Protocol::SensorimotorSynthesis);
    let mut untraced = PathWorld::with_protocol(1, false, Protocol::SensorimotorSynthesis);
    let traced_run = traced.use_path();
    let untraced_run = untraced.use_path();

    assert_eq!(traced_run.outputs, untraced_run.outputs);
    assert_eq!(traced_run.work, untraced_run.work);
    assert_eq!(traced_run.execution_cost, untraced_run.execution_cost);
    assert_eq!(
        traced_run.naturally_quiescent,
        untraced_run.naturally_quiescent
    );
    assert_eq!(traced.harness.read(), untraced.harness.read());
    assert!(untraced_run.physical_trace.is_empty());
    assert!(traced_run.physical_diagnostics().next().is_some());
    assert!(traced_run
        .physical_diagnostics()
        .all(|transition| transition.event.is_diagnostic()));

    let checkpoint = traced.harness.save().expect("diagnostic checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("diagnostic checkpoint restores");
    let tick = traced.harness.read().clock.tick.saturating_add(1);
    let consequence = [Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 25_003,
        target: traced.outcome,
        impulse: 1,
    }];
    assert_eq!(traced.harness.send(&consequence), replay.send(&consequence));
}

fn lineage_merge_run(protocol: Protocol, origins: [u64; 2]) -> (Harness, Run, JunctionId) {
    let mut builder = HarnessBuilder::with_capacity(16, 16, OUTWARD_REGION);
    builder.set_protocol(protocol);
    builder.set_physical_tracing(true);
    let merge = junction(&mut builder, 38_000, 0, 0, 2);
    let sink = junction(&mut builder, 38_010, 0, OUTWARD_REGION, 1);
    link(
        &mut builder,
        merge,
        sink,
        0,
        1,
        u32::MAX,
        TransmissionMode::Drive,
    );
    let mut harness = builder.build();
    let run = harness.send(&[
        Input {
            arrival_tick: 0,
            phase: 0,
            origin_physical: origins[0],
            target: merge,
            impulse: 1,
        },
        Input {
            arrival_tick: 0,
            phase: 0,
            origin_physical: origins[1],
            target: merge,
            impulse: 1,
        },
    ]);
    (harness, run, sink)
}

fn lineage_return_run(
    origins_reversed: bool,
) -> (Harness, Run, JunctionId, JunctionId, JunctionId) {
    let (mut harness, action, surface, unrelated, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerCausalLineage);
    harness.send(&[input(action, 0), input(motor, 2)]);
    let tick = harness.read().clock.tick.saturating_add(1);
    let mut inputs = [
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_001,
            target: surface,
            impulse: 1,
        },
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_002,
            target: unrelated,
            impulse: 1,
        },
    ];
    if origins_reversed {
        inputs.reverse();
    }
    let run = harness.send(&inputs);
    (harness, run, surface, unrelated, motor)
}

#[test]
fn causal_lineage_candidate_preserves_actual_origins_without_changing_impulse() {
    let (_, scalar, _) =
        lineage_merge_run(Protocol::RecursiveLearnerConstruction, [38_001, 38_002]);
    let (_, lineage, sink) =
        lineage_merge_run(Protocol::RecursiveLearnerCausalLineage, [38_001, 38_002]);
    assert_eq!(lineage.outputs, scalar.outputs);
    assert_eq!(lineage.outputs.len(), 1);
    assert_eq!(lineage.outputs[0].impulse, 1);
    let members = lineage
        .physical_diagnostics()
        .filter_map(|transition| match transition.event {
            PhysicalEvent::CausalLineageMemberObserved {
                target,
                origin_physical,
                mode: TransmissionMode::Drive,
                ..
            } if target == sink => Some(origin_physical),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(members, BTreeSet::from([38_001, 38_002]));

    let (_, returned, _, _, _) = lineage_return_run(false);
    assert!(returned.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnOriginEvaluated {
            origin_physical: 35_001,
            decision: ReturnOriginDecision::AdmittedLocal,
            ..
        }
    )));
    assert!(returned.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnOriginEvaluated {
            origin_physical: 35_002,
            decision: ReturnOriginDecision::RejectedNonLocal,
            ..
        }
    )));
}

#[test]
fn causal_lineage_candidate_is_order_independent_and_duplicate_free() {
    let (forward, forward_run, _, _, _) = lineage_return_run(false);
    let (reverse, reverse_run, _, _, _) = lineage_return_run(true);
    let decisions = |run: &Run| {
        run.physical_diagnostics()
            .filter_map(|transition| match transition.event {
                PhysicalEvent::ReturnOriginEvaluated {
                    origin_physical,
                    decision,
                    ..
                } => Some((origin_physical, decision)),
                _ => None,
            })
            .collect::<BTreeSet<_>>()
    };
    assert_eq!(decisions(&forward_run), decisions(&reverse_run));
    assert_eq!(forward_run.outputs, reverse_run.outputs);
    assert_eq!(forward.read(), reverse.read());

    let (mut duplicate, action, surface, unrelated, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerCausalLineage);
    duplicate.send(&[input(action, 0), input(motor, 2)]);
    let tick = duplicate.read().clock.tick.saturating_add(1);
    let duplicate_run = duplicate.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_001,
            target: surface,
            impulse: 1,
        },
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_001,
            target: unrelated,
            impulse: 1,
        },
    ]);
    let admitted = duplicate_run
        .physical_trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::ReturnOriginAdmission {
                    origin_physical: 35_001,
                    admitted: true,
                    ..
                }
            )
        })
        .count();
    assert_eq!(admitted, 1);
}

#[test]
fn causal_lineage_candidate_preserves_replay_and_old_protocol() {
    let (mut candidate, action, surface, unrelated, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerCausalLineage);
    candidate.send(&[input(action, 0), input(motor, 2)]);
    let checkpoint = candidate.save().expect("lineage checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("lineage checkpoint restores");
    let tick = candidate.read().clock.tick.saturating_add(1);
    let consequence = [
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_001,
            target: surface,
            impulse: 1,
        },
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_002,
            target: unrelated,
            impulse: 1,
        },
    ];
    assert_eq!(candidate.send(&consequence), replay.send(&consequence));
    assert_eq!(
        candidate.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );

    let (_, scalar, sink) =
        lineage_merge_run(Protocol::RecursiveLearnerConstruction, [38_001, 38_002]);
    assert!(scalar.physical_diagnostics().any(|transition| matches!(
        transition.event,
        PhysicalEvent::CausalOriginResolved {
            target,
            resolved_origin: 38_000,
            resolution: CausalOriginResolution::JunctionFallback,
            ..
        } if target != sink
    )));
    assert!(!scalar.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::CausalLineageMemberObserved { .. }
    )));
}

fn consequence_born_coactive_round(
    harness: &mut Harness,
    action: JunctionId,
    surface: JunctionId,
    motor: JunctionId,
    surface_offset: i64,
) -> Run {
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_000,
            target: action,
            impulse: 1,
        },
        Input {
            arrival_tick: tick.saturating_add(surface_offset),
            phase: 0,
            origin_physical: 35_001,
            target: surface,
            impulse: 1,
        },
        Input {
            arrival_tick: tick.saturating_add(2),
            phase: 0,
            origin_physical: 35_010,
            target: motor,
            impulse: 1,
        },
    ])
}

#[test]
fn consequence_born_temporal_only_rejects_first_coactivity_but_reuses_old_sibling() {
    let (mut old, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerCausalLineage);
    let old_first = consequence_born_coactive_round(&mut old, action, surface, motor, 0);
    let old_second = consequence_born_coactive_round(&mut old, action, surface, motor, 0);
    assert_eq!(old_first.work.causal_closure_observations, 1);
    assert_eq!(old_second.work.learner_constructions, 1);

    let (mut candidate, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConsequenceBornClosure);
    let coactive_first = consequence_born_coactive_round(&mut candidate, action, surface, motor, 0);
    let coactive_second =
        consequence_born_coactive_round(&mut candidate, action, surface, motor, 0);
    assert_eq!(coactive_first.work.causal_closure_observations, 0);
    assert_eq!(coactive_second.work.causal_closure_observations, 1);
    assert_eq!(coactive_second.work.learner_constructions, 0);
    assert!(coactive_first
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::ClosureEligibilityEvaluated {
                origin_physical: 35_001,
                eligible: false,
                ..
            }
        )));
    assert!(coactive_second
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::ClosureEligibilityEvaluated {
                origin_physical: 35_001,
                eligible: true,
                ..
            }
        )));
    assert!(coactive_first
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::ReversePathConsolidated { source, .. } if source == surface
        )));

    let (mut delayed, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConsequenceBornClosure);
    delayed.send(&[input(action, 0), input(motor, 2)]);
    let first = send_physical(&mut delayed, surface, 35_001);
    let tick = delayed.read().clock.tick.saturating_add(1);
    delayed.send(&[input(action, tick), input(motor, tick.saturating_add(2))]);
    let second = send_physical(&mut delayed, surface, 35_001);
    assert_eq!(first.work.causal_closure_observations, 1);
    assert_eq!(first.work.learner_constructions, 0);
    assert_eq!(second.work.causal_closure_observations, 0);
    assert_eq!(second.work.learner_constructions, 0);
    assert!(second.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ClosureEligibilityEvaluated {
            origin_physical: 35_001,
            eligible: false,
            ..
        }
    )));
    assert!(coactive_first.naturally_quiescent);
    assert!(coactive_second.naturally_quiescent);
    assert!(first.naturally_quiescent);
    assert!(second.naturally_quiescent);
}

#[test]
fn consequence_born_cohort_rejects_repeated_coactivity_and_keeps_delayed_consequence() {
    let (mut candidate, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConsequenceCohortClosure);
    let first = consequence_born_coactive_round(&mut candidate, action, surface, motor, 0);
    let second = consequence_born_coactive_round(&mut candidate, action, surface, motor, 0);
    assert_eq!(first.work.causal_closure_observations, 0);
    assert_eq!(second.work.causal_closure_observations, 0);
    assert_eq!(second.work.learner_constructions, 0);
    assert!(first.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnCohortClosed { link_count, .. } if link_count > 0
    )));

    let (mut delayed, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConsequenceCohortClosure);
    let first_action = delayed.send(&[input(action, 0), input(motor, 2)]);
    let first_return = send_physical(&mut delayed, surface, 35_001);
    let tick = delayed.read().clock.tick.saturating_add(1);
    let second_action = delayed.send(&[input(action, tick), input(motor, tick.saturating_add(2))]);
    let second_return = send_physical(&mut delayed, surface, 35_001);
    assert_eq!(first_return.work.causal_closure_observations, 1);
    assert_eq!(first_return.work.learner_constructions, 0);
    assert_eq!(second_return.work.causal_closure_observations, 0);
    assert_eq!(second_return.work.learner_constructions, 0);
    assert!([first_action, first_return, second_action, second_return]
        .iter()
        .all(|run| run.naturally_quiescent));
}

#[test]
fn consequence_born_eligible_return_priority_completes_temporal_cohort_composition() {
    let (mut coactive, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerEligibleReturnClosure);
    let first = consequence_born_coactive_round(&mut coactive, action, surface, motor, 0);
    let second = consequence_born_coactive_round(&mut coactive, action, surface, motor, 0);
    assert_eq!(first.work.causal_closure_observations, 0);
    assert_eq!(second.work.causal_closure_observations, 0);
    assert_eq!(second.work.learner_constructions, 0);

    let (mut delayed, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerEligibleReturnClosure);
    delayed.send(&[input(action, 0), input(motor, 2)]);
    let first = send_physical(&mut delayed, surface, 35_001);
    let tick = delayed.read().clock.tick.saturating_add(1);
    delayed.send(&[input(action, tick), input(motor, tick.saturating_add(2))]);
    let checkpoint = delayed.save().expect("eligible-return checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("eligible-return checkpoint restores");
    let second = send_physical(&mut delayed, surface, 35_001);
    let replayed = send_physical(&mut replay, surface, 35_001);
    assert_eq!(second, replayed);
    assert_eq!(first.work.causal_closure_observations, 1);
    assert_eq!(second.work.causal_closure_observations, 1);
    assert_eq!(second.work.learner_constructions, 1);
    assert!(second.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ClosureEligibilityEvaluated {
            origin_physical: 35_001,
            eligible: true,
            ..
        }
    )));
    assert_eq!(
        delayed.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );
    assert!(first.naturally_quiescent);
    assert!(second.naturally_quiescent);
}

#[test]
fn consequence_born_closure_is_strict_duplicate_free_and_replays() {
    let (mut candidate, action, surface, unrelated, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerConsequenceBornClosure);
    let equal = consequence_born_coactive_round(&mut candidate, action, surface, motor, 2);
    assert_eq!(equal.work.causal_closure_observations, 0);
    assert!(equal.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ClosureEligibilityEvaluated {
            origin_physical: 35_001,
            origin_birth_tick,
            return_opened_tick,
            eligible: false,
            ..
        } if origin_birth_tick <= return_opened_tick
    )));

    let checkpoint = candidate.save().expect("temporal checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("temporal checkpoint restores");
    let tick = candidate.read().clock.tick.saturating_add(1);
    let duplicate = [
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_001,
            target: surface,
            impulse: 1,
        },
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 35_001,
            target: unrelated,
            impulse: 1,
        },
    ];
    let run = candidate.send(&duplicate);
    let replayed = replay.send(&duplicate);
    assert_eq!(run, replayed);
    assert!(run.work.causal_closure_observations <= 1);
    assert_eq!(run.work.learner_constructions, 0);
    assert_eq!(
        candidate.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );
    assert!(run.naturally_quiescent);
}

#[test]
fn boundary_novelty_rejects_exact_owned_repetition_and_replays() {
    let (mut candidate, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerBoundaryNovelty);
    let first = observe_recursive_closure(&mut candidate, action, surface, motor);
    let second = observe_recursive_closure(&mut candidate, action, surface, motor);
    assert_eq!(first.work.causal_closure_observations, 1);
    assert_eq!(second.work.learner_constructions, 1);
    assert_eq!(candidate.read().learners.len(), 1);

    let checkpoint = candidate.save().expect("boundary checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("boundary checkpoint restores");
    for _ in 0..10 {
        let rejected = observe_recursive_closure(&mut candidate, action, surface, motor);
        let replayed = observe_recursive_closure(&mut replay, action, surface, motor);
        assert_eq!(rejected, replayed);
        assert_eq!(rejected.work.causal_closure_observations, 0);
        assert_eq!(rejected.work.learner_constructions, 0);
        assert!(rejected.physical_trace.iter().any(|transition| matches!(
            transition.event,
            PhysicalEvent::BoundaryNoveltyEvaluated {
                parent: Some(_),
                novel_members: 0,
                eligible: false,
                ..
            }
        )));
    }
    assert_eq!(candidate.read().learners.len(), 1);
    assert_eq!(
        candidate.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );
}

#[test]
fn boundary_novelty_allows_adjacent_expansion_with_a_new_physical_member() {
    let (mut candidate, action, surface, _, motor, _, controlled, controlled_outcome) =
        recursive_control_world_with_protocol(Protocol::RecursiveLearnerBoundaryNovelty);
    observe_control_closure(&mut candidate, action, surface, motor);
    let root = observe_control_closure(&mut candidate, action, surface, motor);
    assert_eq!(root.work.learner_constructions, 1);
    let root_id = candidate.read().learners[0].id;

    for expected in [0, 1] {
        let action_run = stimulate_recursive_control(&mut candidate, surface, controlled);
        let consequence = return_recursive_control(&mut candidate, controlled_outcome);
        assert!(action_run.naturally_quiescent);
        assert_eq!(consequence.work.learner_constructions, expected);
    }
    let observed = candidate.read();
    assert_eq!(observed.learners.len(), 2);
    assert_eq!(observed.learners[1].parent, Some(root_id));
    assert_eq!(observed.learners[1].output, controlled);
}

#[test]
fn boundary_novelty_constructs_distinct_root_siblings_in_either_order() {
    let mut observed_sets = Vec::new();
    for order in [[0usize, 1usize], [1usize, 0usize]] {
        let (world, action, surfaces, motor) = boundary_novelty_sibling_world(2);
        let checkpoint = world.save().expect("sibling checkpoint saves");
        let mut candidate = Harness::restore(checkpoint.clone()).expect("candidate restores");
        let mut replay = Harness::restore(checkpoint).expect("replay restores");
        for index in order {
            let physical = 39_001 + u64::try_from(index).unwrap();
            for _ in 0..2 {
                let run = observe_boundary_sibling_closure(
                    &mut candidate,
                    action,
                    surfaces[index],
                    physical,
                    motor,
                );
                let replayed = observe_boundary_sibling_closure(
                    &mut replay,
                    action,
                    surfaces[index],
                    physical,
                    motor,
                );
                assert_eq!(run, replayed);
            }
        }
        let observed = candidate.read();
        assert_eq!(observed.learners.len(), 2);
        assert!(observed
            .learners
            .iter()
            .all(|learner| learner.parent.is_none()));
        let learned_surfaces = observed
            .learners
            .iter()
            .map(|learner| learner.surface)
            .collect::<BTreeSet<_>>();
        assert_eq!(learned_surfaces, surfaces.into_iter().collect());
        assert_eq!(
            candidate.save().unwrap().canonical_bytes().unwrap(),
            replay.save().unwrap().canonical_bytes().unwrap()
        );
        observed_sets.push(learned_surfaces);
    }
    assert_eq!(observed_sets[0], observed_sets[1]);
}

#[test]
fn boundary_novelty_preserves_inherited_distance_three_return_rejection() {
    let (mut candidate, action, surfaces, motor) = boundary_novelty_sibling_world(3);
    for _ in 0..2 {
        observe_boundary_sibling_closure(&mut candidate, action, surfaces[0], 39_001, motor);
    }
    assert_eq!(candidate.read().learners.len(), 1);

    let first =
        observe_boundary_sibling_closure(&mut candidate, action, surfaces[1], 39_002, motor);
    let second =
        observe_boundary_sibling_closure(&mut candidate, action, surfaces[1], 39_002, motor);
    assert_eq!(first.work.causal_closure_observations, 1);
    assert_eq!(second.work.causal_closure_observations, 0);
    assert_eq!(second.work.learner_constructions, 0);
    assert!(second.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::ReturnOriginEvaluated {
            distance: Some(3),
            decision: ReturnOriginDecision::RejectedNonLocal,
            ..
        }
    )));
    assert_eq!(candidate.read().learners.len(), 1);
}

#[test]
fn autonomous_reuse_diagnostics_are_observational_only() {
    let mut traced = PathWorld::with_protocol(1, true, Protocol::SensorimotorCandidate);
    let mut untraced = PathWorld::with_protocol(1, false, Protocol::SensorimotorCandidate);
    let traced_run = traced.use_path();
    let untraced_run = untraced.use_path();

    assert_eq!(traced_run.outputs, untraced_run.outputs);
    assert_eq!(traced_run.work, untraced_run.work);
    assert_eq!(traced_run.execution_cost, untraced_run.execution_cost);
    assert_eq!(
        traced_run.naturally_quiescent,
        untraced_run.naturally_quiescent
    );
    assert_eq!(traced.harness.read(), untraced.harness.read());
    assert!(traced_run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::OutputCandidateEvaluated {
            ownership: CandidateOwnership::Organism,
            executable: true,
            ..
        }
    )));
    assert!(traced_run
        .physical_trace
        .iter()
        .filter(|transition| matches!(
            transition.event,
            PhysicalEvent::OutputCandidateEvaluated { .. }
        ))
        .all(|transition| transition.event.is_diagnostic()));
}

#[test]
fn pre_executability_diagnostics_report_signed_drive_and_live_returns_observationally() {
    let mut traced = PathWorld::with_protocol(1, true, Protocol::SensorimotorCandidate);
    let mut untraced = PathWorld::with_protocol(1, false, Protocol::SensorimotorCandidate);
    let traced_run = traced.use_path();
    let untraced_run = untraced.use_path();

    assert_eq!(traced_run.outputs, untraced_run.outputs);
    assert_eq!(traced_run.work, untraced_run.work);
    assert_eq!(traced_run.execution_cost, untraced_run.execution_cost);
    assert_eq!(traced.harness.read(), untraced.harness.read());
    assert!(traced_run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::OutputCandidateEvaluated {
            positive_path_strength: UNIT_U64,
            negative_path_strength: UNIT_U64,
            opportunity,
            unanswered_returns: 0,
            executable: true,
            ..
        } if opportunity > 0
    )));

    let mut returned = LocalOutcomePairWorld::new(Protocol::SensorimotorCandidate);
    let first = returned.stimulate().outputs[0].from_physical;
    returned.outcome_for(first, 1);
    returned.stimulate();
    let pending = returned.stimulate();
    assert!(
        pending.physical_trace.iter().any(|transition| matches!(
            transition.event,
            PhysicalEvent::OutputCandidateEvaluated {
                positive_path_strength,
                negative_path_strength,
                unanswered_returns,
                executable: true,
                ..
            } if positive_path_strength > 0
                && negative_path_strength > 0
                && unanswered_returns > 0
        )),
        "{:#?}",
        pending.physical_trace
    );
    assert!(pending.naturally_quiescent);
}

#[test]
fn autonomous_reuse_diagnostics_order_owned_surface_path_and_candidate() {
    let (mut candidate, action, surface, _, motor) =
        recursive_learner_world(Protocol::RecursiveLearnerBoundaryNovelty);
    observe_recursive_closure(&mut candidate, action, surface, motor);
    observe_recursive_closure(&mut candidate, action, surface, motor);
    assert_eq!(candidate.read().learners.len(), 1);

    let run = send_physical(&mut candidate, surface, 35_001);
    let surface_index = run
        .physical_trace
        .iter()
        .position(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::SurfacePathStateObserved {
                    surface: observed,
                    owner: Some(_),
                    complete_paths,
                    ..
                } if observed == surface && complete_paths > 0
            )
        })
        .expect("owned surface path state is observed");
    let candidate_index = run
        .physical_trace
        .iter()
        .position(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::OutputCandidateEvaluated {
                    ownership: CandidateOwnership::Owned(_),
                    path_inputs,
                    ..
                } if path_inputs > 0
            )
        })
        .expect("owned output candidate is evaluated");
    assert!(surface_index < candidate_index);
    assert!(run.naturally_quiescent);
}

#[test]
fn drive_provenance_reports_path_owner_separately_from_carried_origin_owner() {
    let protocol = Protocol::RecursiveLearnerCausalOriginFactorization;
    let (mut candidate, surfaces) = prepared_mixed_owner_world(protocol);
    let owner = candidate
        .read()
        .learners
        .into_iter()
        .find(|learner| learner.surface == surfaces[0])
        .expect("first surface has an owner")
        .id;
    let checkpoint = candidate.save().expect("provenance checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("provenance replay restores");
    let tick = candidate.read().clock.tick.saturating_add(1);
    let inputs = [Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 39_099,
        target: surfaces[0],
        impulse: 1,
    }];
    let run = candidate.send(&inputs);
    let replayed = replay.send(&inputs);

    assert_eq!(run, replayed);
    let provenance = run
        .physical_trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::DriveProvenanceObserved { .. }
            )
        })
        .collect::<Vec<_>>();
    assert!(
        run.physical_trace.iter().any(|transition| matches!(
            transition.event,
        PhysicalEvent::DriveProvenanceObserved {
            source: Some(_),
                link: Some(_),
                completes_path: true,
                carried_origin: 39_099,
                origin_owner: None,
            path_owner: Some(observed),
            ..
        } if observed == owner
        )),
        "{provenance:#?}"
    );
    assert!(run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::DriveProvenanceObserved {
            source: None,
            link: None,
            completes_path: false,
            carried_origin: 39_099,
            origin_owner: None,
            path_owner: None,
            ..
        }
    )));
    assert_eq!(
        candidate.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );
    assert!(run.naturally_quiescent);
}

fn prepared_mixed_owner_world(protocol: Protocol) -> (Harness, [JunctionId; 2]) {
    let (mut harness, action, surfaces, motor) =
        boundary_novelty_sibling_world_with_protocol(2, protocol);
    for (index, surface) in surfaces.iter().enumerate() {
        for _ in 0..2 {
            observe_boundary_sibling_closure(
                &mut harness,
                action,
                *surface,
                39_001 + u64::try_from(index).unwrap(),
                motor,
            );
        }
    }
    assert_eq!(harness.read().learners.len(), 2);
    for _ in 0..2 {
        observe_boundary_sibling_closure(&mut harness, action, surfaces[0], 39_001, motor);
    }
    (harness, surfaces)
}

fn send_mixed_owner_surfaces(harness: &mut Harness, surfaces: [JunctionId; 2]) -> Run {
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 39_001,
            target: surfaces[0],
            impulse: 1,
        },
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 39_002,
            target: surfaces[1],
            impulse: 1,
        },
    ])
}

struct OpportunityOriginWorld {
    harness: Harness,
    inputs: [JunctionId; 2],
    opportunities: [JunctionId; 2],
}

impl OpportunityOriginWorld {
    fn new(protocol: Protocol) -> Self {
        let positions = [0, 10];
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_protocol(protocol);
        builder.set_physical_tracing(true);
        let inputs = std::array::from_fn(|index| {
            junction(
                &mut builder,
                40_000 + u64::try_from(index).unwrap(),
                positions[index],
                0,
                1,
            )
        });
        let opportunities = std::array::from_fn(|index| {
            junction(
                &mut builder,
                40_010 + u64::try_from(index).unwrap(),
                positions[index] + 1,
                0,
                2,
            )
        });
        let sinks: [JunctionId; 2] = std::array::from_fn(|index| {
            junction(
                &mut builder,
                40_020 + u64::try_from(index).unwrap(),
                positions[index] + 1,
                OUTWARD_REGION,
                1,
            )
        });
        let outcomes: [JunctionId; 2] = std::array::from_fn(|index| {
            junction(
                &mut builder,
                40_030 + u64::try_from(index).unwrap(),
                100 + i32::try_from(index).unwrap() * 10,
                0,
                1,
            )
        });
        for index in 0..2 {
            let anchor = junction(
                &mut builder,
                40_040 + u64::try_from(index).unwrap(),
                1_000 + i32::try_from(index).unwrap(),
                0,
                99,
            );
            for target in [inputs[index], outcomes[index]] {
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
            inputs,
            opportunities,
        }
    }

    fn stimulate(&mut self, opportunity_origins: [u64; 2]) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            Input {
                arrival_tick: tick,
                phase: 0,
                origin_physical: 40_100,
                target: self.inputs[0],
                impulse: 1,
            },
            Input {
                arrival_tick: tick,
                phase: 0,
                origin_physical: 40_101,
                target: self.inputs[1],
                impulse: 1,
            },
            Input {
                arrival_tick: tick + 1,
                phase: 1,
                origin_physical: opportunity_origins[0],
                target: self.opportunities[0],
                impulse: 1,
            },
            Input {
                arrival_tick: tick + 1,
                phase: 1,
                origin_physical: opportunity_origins[1],
                target: self.opportunities[1],
                impulse: 1,
            },
        ])
    }
}

#[test]
fn shared_opportunity_origin_is_already_a_bounded_choice() {
    let protocol = Protocol::RecursiveLearnerCausalTopologyProductComposition;
    let mut world = OpportunityOriginWorld::new(protocol);
    let run = world.stimulate([40_200, 40_200]);

    assert_eq!(run.outputs.len(), 1, "{:#?}", run.physical_trace);
    assert!(run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::CandidateSelection {
            origin_scope: Some(40_200),
            admitted: false,
            ..
        }
    )));
    assert!(run.naturally_quiescent);
}

#[test]
fn distinct_opportunity_origins_still_compose_across_topology() {
    let protocol = Protocol::RecursiveLearnerCausalTopologyProductComposition;
    let mut world = OpportunityOriginWorld::new(protocol);
    let run = world.stimulate([40_200, 40_201]);

    assert_eq!(run.outputs.len(), 2, "{:#?}", run.physical_trace);
    assert!(run.naturally_quiescent);
}

#[test]
fn owner_local_factorization_turns_one_unique_private_group_into_one_effect() {
    let (mut reference, surfaces) =
        prepared_mixed_owner_world(Protocol::RecursiveLearnerBoundaryNovelty);
    let rejected = send_mixed_owner_surfaces(&mut reference, surfaces);
    assert!(!rejected
        .outputs
        .iter()
        .any(|output| output.from_physical == 39_010));
    assert!(rejected.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::OutputCandidateEvaluated {
            ownership: CandidateOwnership::Ambiguous,
            executable: false,
            ..
        }
    )));

    let (mut candidate, surfaces) =
        prepared_mixed_owner_world(Protocol::RecursiveLearnerOwnerFactorization);
    let checkpoint = candidate.save().expect("factorization checkpoint saves");
    let mut replay = Harness::restore(checkpoint.clone()).expect("factorization replay restores");
    let mut reordered = Harness::restore(checkpoint).expect("factorization checkpoint restores");
    let selected = send_mixed_owner_surfaces(&mut candidate, surfaces);
    let replayed = send_mixed_owner_surfaces(&mut replay, surfaces);
    let tick = reordered.read().clock.tick.saturating_add(1);
    let reversed = reordered.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 39_002,
            target: surfaces[1],
            impulse: 1,
        },
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 39_001,
            target: surfaces[0],
            impulse: 1,
        },
    ]);
    let motor_outputs = |run: &Run| {
        run.outputs
            .iter()
            .filter(|output| output.from_physical == 39_010)
            .count()
    };
    assert_eq!(motor_outputs(&selected), 1);
    assert_eq!(selected, replayed);
    assert_eq!(motor_outputs(&reversed), 1);
    assert_eq!(selected.outputs, reversed.outputs);
    assert!(selected.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::MixedOwnerCandidateResolved {
            owner_count: 2,
            executable_groups,
            selected_owner: Some(_),
            selected_path_inputs,
            ..
        } if executable_groups > 0 && selected_path_inputs > 0
    )));
    assert_eq!(
        candidate.read().learners.len(),
        reordered.read().learners.len()
    );
    assert_eq!(candidate.read().links.len(), reordered.read().links.len());
    assert_eq!(
        candidate.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );
    assert!(selected.naturally_quiescent);
    assert!(reversed.naturally_quiescent);
}

#[test]
fn causal_origin_ownership_factorization_selects_one_origin_and_replays() {
    let protocol = Protocol::RecursiveLearnerCausalOriginFactorization;
    let (mut candidate, surfaces) = prepared_mixed_owner_world(protocol);
    let checkpoint = candidate.save().expect("origin checkpoint saves");
    let mut replay = Harness::restore(checkpoint.clone()).expect("origin replay restores");
    let mut reordered = Harness::restore(checkpoint).expect("origin reorder restores");
    let selected = send_mixed_owner_surfaces(&mut candidate, surfaces);
    let replayed = send_mixed_owner_surfaces(&mut replay, surfaces);
    let tick = reordered.read().clock.tick.saturating_add(1);
    let reversed = reordered.send(&[
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 39_002,
            target: surfaces[1],
            impulse: 1,
        },
        Input {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 39_001,
            target: surfaces[0],
            impulse: 1,
        },
    ]);
    let motor_outputs = |run: &Run| {
        run.outputs
            .iter()
            .filter(|output| output.from_physical == 39_010)
            .count()
    };
    assert_eq!(motor_outputs(&selected), 1);
    assert_eq!(motor_outputs(&reversed), 1);
    assert_eq!(selected.outputs, reversed.outputs);
    assert_eq!(selected, replayed);
    assert!(selected.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::CausalOriginCandidateResolved {
            origin_count: 2,
            selected_origin: Some(_),
            selected_path_inputs,
            ..
        } if selected_path_inputs > 0
    )));
    assert_eq!(
        candidate.save().unwrap().canonical_bytes().unwrap(),
        replay.save().unwrap().canonical_bytes().unwrap()
    );
    assert!(selected.naturally_quiescent);
    assert!(reversed.naturally_quiescent);
}

#[test]
fn causal_origin_ownership_factorization_bounded_send_reports_non_quiescence() {
    let protocol = Protocol::RecursiveLearnerCausalOriginFactorization;
    let world = PathWorld::with_protocol(1, true, protocol);
    let path_input = world.input;
    let checkpoint = world.harness.save().expect("path world saves");
    let mut ordinary = Harness::restore(checkpoint.clone()).expect("ordinary world restores");
    let mut bounded = Harness::restore(checkpoint.clone()).expect("bounded world restores");
    let mut exhausted = Harness::restore(checkpoint).expect("exhausted world restores");
    let inputs = [input(path_input, 0)];

    let ordinary_run = ordinary.send(&inputs);
    let bounded_run = bounded.send_bounded(&inputs, 100);
    let exhausted_run = exhausted.send_bounded(&inputs, 0);

    assert_eq!(ordinary_run, bounded_run);
    assert_eq!(
        ordinary.save().unwrap().canonical_bytes().unwrap(),
        bounded.save().unwrap().canonical_bytes().unwrap()
    );
    assert!(!exhausted_run.naturally_quiescent);
    assert!(exhausted_run
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::PropagationBudgetExhausted { moments: 0 }
        )));
}

struct BoundaryFeedbackWorld {
    harness: Harness,
    motors: [JunctionId; 2],
    effects: [JunctionId; 2],
}

impl BoundaryFeedbackWorld {
    fn new(protocol: Protocol) -> Self {
        let mut builder = HarnessBuilder::with_capacity(32, 96, OUTWARD_REGION);
        builder.set_protocol(protocol);
        builder.set_physical_tracing(true);
        let motors = [
            junction(&mut builder, 70_000, 0, 0, 1),
            junction(&mut builder, 70_001, 1, 0, 1),
        ];
        let effects = [
            junction(&mut builder, 70_010, 0, OUTWARD_REGION, 1),
            junction(&mut builder, 70_011, 1, OUTWARD_REGION, 1),
        ];
        for index in 0..2 {
            link(
                &mut builder,
                motors[index],
                effects[index],
                0,
                1,
                u32::MAX,
                TransmissionMode::Drive,
            );
        }
        Self {
            harness: builder.build(),
            motors,
            effects,
        }
    }

    fn stimulate_motor(&mut self, max_moments: u64) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness
            .send_bounded(&[input(self.motors[0], tick)], max_moments)
    }

    fn stimulate_effect(&mut self, max_moments: u64) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness
            .send_bounded(&[input(self.effects[0], tick)], max_moments)
    }
}

#[test]
fn boundary_effect_reentry_reference_exposes_cross_region_path_genesis() {
    let mut reference =
        BoundaryFeedbackWorld::new(Protocol::RecursiveLearnerCausalOriginFactorization);
    let run = reference.stimulate_motor(64);
    assert!(run.naturally_quiescent);
    assert_eq!(run.outputs.len(), 3);
    assert!(run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::JunctionProposal { source, target, .. }
            if source == reference.effects[0] && target == reference.motors[1]
    )));
}

#[test]
fn boundary_effect_reentry_terminal_preserves_effect_but_blocks_effect_born_genesis() {
    let protocol = Protocol::RecursiveLearnerBoundaryEffectTerminal;
    let mut candidate = BoundaryFeedbackWorld::new(protocol);
    let checkpoint = candidate.harness.save().expect("terminal world saves");
    let mut replay = BoundaryFeedbackWorld {
        harness: Harness::restore(checkpoint).expect("terminal world restores"),
        motors: candidate.motors,
        effects: candidate.effects,
    };
    let run = candidate.stimulate_motor(64);
    let replayed = replay.stimulate_motor(64);

    assert_eq!(run, replayed);
    assert!(run.naturally_quiescent);
    assert_eq!(run.outputs.len(), 1);
    assert!(!run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::JunctionProposal { source, target, .. }
            if source == candidate.effects[0] && target == candidate.motors[1]
    )));
    assert_eq!(
        candidate.harness.save().unwrap().canonical_bytes().unwrap(),
        replay.harness.save().unwrap().canonical_bytes().unwrap()
    );
}

#[test]
fn boundary_effect_reentry_external_surface_discriminates_terminal_from_regional_law() {
    let mut terminal = BoundaryFeedbackWorld::new(Protocol::RecursiveLearnerBoundaryEffectTerminal);
    let terminal_run = terminal.stimulate_effect(64);
    assert!(terminal_run.naturally_quiescent);
    assert!(terminal_run
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::JunctionProposal { source, target, .. }
                if source == terminal.effects[0] && target == terminal.motors[1]
        )));

    let mut regional = BoundaryFeedbackWorld::new(Protocol::RecursiveLearnerRegionalPathClosure);
    let regional_run = regional.stimulate_effect(64);
    assert!(regional_run.naturally_quiescent);
    assert!(!regional_run
        .physical_trace
        .iter()
        .any(|transition| matches!(
            transition.event,
            PhysicalEvent::JunctionProposal { source, target, .. }
                if source == regional.effects[0] && target == regional.motors[1]
        )));
}

#[test]
fn boundary_effect_reentry_candidates_preserve_same_region_path_formation() {
    for protocol in [
        Protocol::RecursiveLearnerRegionalPathClosure,
        Protocol::RecursiveLearnerBoundaryEffectTerminal,
    ] {
        let mut world = PathWorld::with_protocol(1, true, protocol);
        let run = world.use_path();
        assert_eq!(run.outputs.len(), 1);
        assert!(run.naturally_quiescent);
    }
}
