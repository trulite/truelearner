#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent, Protocol, Run,
    TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arm {
    CausalLocalCredit,
    DelayedReturnLifetime,
    LocalAlternativeExposure,
    DisplacedReturnClosure,
    ConsequenceSupportedContinuation,
    NoConsequenceRelease,
    ReversibleLocalCompetition,
    SparseSharedOpportunity,
    CoalitionSeparateCredit,
    DelayedMultisurfaceAlignment,
    ComposedOutputSequence,
    ScaleSelectivity,
    ConsolidatedReuse,
    OneJointComposition,
}

impl Arm {
    pub const ALL: [Self; 14] = [
        Self::CausalLocalCredit,
        Self::DelayedReturnLifetime,
        Self::LocalAlternativeExposure,
        Self::DisplacedReturnClosure,
        Self::ConsequenceSupportedContinuation,
        Self::NoConsequenceRelease,
        Self::ReversibleLocalCompetition,
        Self::SparseSharedOpportunity,
        Self::CoalitionSeparateCredit,
        Self::DelayedMultisurfaceAlignment,
        Self::ComposedOutputSequence,
        Self::ScaleSelectivity,
        Self::ConsolidatedReuse,
        Self::OneJointComposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::CausalLocalCredit => "causal-local-credit",
            Self::DelayedReturnLifetime => "delayed-return-lifetime",
            Self::LocalAlternativeExposure => "local-alternative-exposure",
            Self::DisplacedReturnClosure => "displaced-return-closure",
            Self::ConsequenceSupportedContinuation => "consequence-supported-continuation",
            Self::NoConsequenceRelease => "no-consequence-release",
            Self::ReversibleLocalCompetition => "reversible-local-competition",
            Self::SparseSharedOpportunity => "sparse-shared-opportunity",
            Self::CoalitionSeparateCredit => "coalition-separate-credit",
            Self::DelayedMultisurfaceAlignment => "delayed-multisurface-alignment",
            Self::ComposedOutputSequence => "composed-output-sequence",
            Self::ScaleSelectivity => "scale-selectivity",
            Self::ConsolidatedReuse => "consolidated-reuse",
            Self::OneJointComposition => "one-joint-composition",
        }
    }

    const fn prediction(self) -> &'static str {
        match self {
            Self::CausalLocalCredit => "one consequence credits only its causal output path",
            Self::DelayedReturnLifetime => "a used path remains exactly creditable after 20 ticks",
            Self::LocalAlternativeExposure => {
                "an unanswered winner yields to one fresh local neighbor"
            }
            Self::DisplacedReturnClosure => {
                "replacement closes only the displaced unanswered return"
            }
            Self::ConsequenceSupportedContinuation => {
                "one real consequence keeps the fresh route eligible for bounded reuse"
            }
            Self::NoConsequenceRelease => {
                "a dominant route releases after participation without consequence"
            }
            Self::ReversibleLocalCompetition => {
                "later contrary consequence can reverse a prior local advantage"
            }
            Self::SparseSharedOpportunity => {
                "one shared opportunity activates bounded distant motor structure"
            }
            Self::CoalitionSeparateCredit => {
                "simultaneous outputs retain consequence-specific causal credit"
            }
            Self::DelayedMultisurfaceAlignment => {
                "one action can retain distinguishable delayed consequences from two surfaces"
            }
            Self::ComposedOutputSequence => {
                "one recalled output's physical consequence invokes a second learned output"
            }
            Self::ScaleSelectivity => {
                "active output and work stay bounded as available surface grows"
            }
            Self::ConsolidatedReuse => {
                "one learned route recalls without activating dormant learned routes"
            }
            Self::OneJointComposition => {
                "the surviving local laws compose into bidirectional one-joint recovery"
            }
        }
    }
}

impl FromStr for Arm {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|arm| arm.id() == value)
            .ok_or_else(|| format!("unknown arm {value:?}"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeResult {
    pub schema: String,
    pub arm: String,
    pub outcome: String,
    pub prediction: String,
    pub observations: serde_json::Value,
    pub falsifier: Option<String>,
    pub exact_replay: bool,
    pub naturally_quiescent: bool,
}

pub fn run(arm: Arm) -> ProbeResult {
    match arm {
        Arm::CausalLocalCredit => causal_local_credit(),
        Arm::DelayedReturnLifetime => delayed_return_lifetime(),
        Arm::LocalAlternativeExposure => local_alternative_exposure(),
        Arm::DisplacedReturnClosure => displaced_return_closure(),
        Arm::ConsequenceSupportedContinuation => consequence_supported_continuation(),
        Arm::NoConsequenceRelease => no_consequence_release(),
        Arm::ReversibleLocalCompetition => reversible_local_competition(),
        Arm::SparseSharedOpportunity => sparse_shared_opportunity(),
        Arm::CoalitionSeparateCredit => coalition_separate_credit(),
        Arm::DelayedMultisurfaceAlignment => delayed_multisurface_alignment(),
        Arm::ComposedOutputSequence => composed_output_sequence(),
        Arm::ScaleSelectivity => scale_selectivity(),
        Arm::ConsolidatedReuse => consolidated_reuse(),
        Arm::OneJointComposition => one_joint_composition(),
    }
}

fn finish(
    arm: Arm,
    survived: bool,
    observations: serde_json::Value,
    falsifier: &str,
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    let passed = survived && exact_replay && naturally_quiescent;
    ProbeResult {
        schema: "sensorimotor-dependency-sweep/v1".to_string(),
        arm: arm.id().to_string(),
        outcome: if passed { "survived" } else { "falsified" }.to_string(),
        prediction: arm.prediction().to_string(),
        observations,
        falsifier: (!passed).then(|| {
            if !exact_replay {
                "exact checkpoint replay differed".to_string()
            } else if !naturally_quiescent {
                "the fixture did not quiesce naturally".to_string()
            } else {
                falsifier.to_string()
            }
        }),
        exact_replay,
        naturally_quiescent,
    }
}

fn physical_input(target: JunctionId, tick: i64, origin: u64) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target,
        impulse: 1,
    }
}

fn add_junction(
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

fn add_link(builder: &mut HarnessBuilder, from: JunctionId, to: JunctionId) {
    builder.add_link(Link {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: u32::MAX,
        mode: TransmissionMode::Drive,
    });
}

fn canonical(harness: &Harness) -> Vec<u8> {
    harness
        .save()
        .and_then(|checkpoint| checkpoint.canonical_bytes())
        .expect("fixture checkpoint is canonical")
}

fn same_replay(left: &Run, right: &Run) -> bool {
    left.outputs == right.outputs
        && left.work == right.work
        && left.naturally_quiescent == right.naturally_quiescent
        && left.memory_bytes == right.memory_bytes
        && left.execution_cost == right.execution_cost
}

fn replay_send(harness: &mut Harness, inputs: &[Input]) -> (Run, bool) {
    let checkpoint = harness.save().expect("fixture checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("fixture checkpoint restores");
    let observed = harness.send(inputs);
    let replayed = replay.send(inputs);
    let exact = same_replay(&observed, &replayed) && canonical(harness) == canonical(&replay);
    (observed, exact)
}

fn used_strength(harness: &Harness, motor: JunctionId) -> Option<i64> {
    harness
        .read()
        .links
        .into_iter()
        .filter(|link| link.live && link.to == motor && link.participation > 0)
        .map(|link| link.strength)
        .max()
}

struct ParallelWorld {
    harness: Harness,
    sources: [JunctionId; 2],
    motors: [JunctionId; 2],
    outcomes: [JunctionId; 2],
}

impl ParallelWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_physical_tracing(true);
        let sources = [
            add_junction(&mut builder, 1_000, 0, 0, 1),
            add_junction(&mut builder, 1_001, 10, 0, 1),
        ];
        let motors = [
            add_junction(&mut builder, 1_010, 1, 0, 2),
            add_junction(&mut builder, 1_011, 11, 0, 2),
        ];
        let sinks = [
            add_junction(&mut builder, 1_020, 1, OUTWARD_REGION, 1),
            add_junction(&mut builder, 1_021, 11, OUTWARD_REGION, 1),
        ];
        let outcomes = [
            add_junction(&mut builder, 1_030, 50, 0, 1),
            add_junction(&mut builder, 1_031, 60, 0, 1),
        ];
        let anchor = add_junction(&mut builder, 1_040, 1_000, 0, 99);
        for target in sources.into_iter().chain(outcomes) {
            add_link(&mut builder, anchor, target);
        }
        for index in 0..2 {
            add_link(&mut builder, motors[index], sinks[index]);
            builder.set_outcome_source_for_output(motors[index], outcomes[index]);
        }
        Self {
            harness: builder.build(),
            sources,
            motors,
            outcomes,
        }
    }

    fn participate(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            physical_input(self.sources[0], tick, 10_000 + tick as u64),
            physical_input(self.sources[1], tick, 20_000 + tick as u64),
            physical_input(self.motors[0], tick + 1, 30_000 + tick as u64),
            physical_input(self.motors[1], tick + 1, 40_000 + tick as u64),
        ])
    }

    fn strengths(&self) -> [i64; 2] {
        std::array::from_fn(|index| {
            used_strength(&self.harness, self.motors[index]).expect("parallel path participates")
        })
    }
}

fn causal_local_credit() -> ProbeResult {
    let arm = Arm::CausalLocalCredit;
    let mut world = ParallelWorld::new();
    let participated = world.participate();
    let before = world.strengths();
    let tick = world.harness.read().clock.tick.saturating_add(1);
    let consequence = [physical_input(world.outcomes[0], tick, 50_000)];
    let (returned, exact_replay) = replay_send(&mut world.harness, &consequence);
    let after = world.strengths();
    let survived = participated.outputs.len() == 2
        && after[0] > before[0]
        && after[1] == before[1]
        && returned.work.local_return_updates >= 2;
    finish(
        arm,
        survived,
        serde_json::json!({
            "participating_outputs": participated.outputs.len(),
            "strength_before": before,
            "strength_after": after,
            "local_return_updates": returned.work.local_return_updates,
        }),
        "the consequence did not credit exactly its causal path",
        exact_replay,
        participated.naturally_quiescent && returned.naturally_quiescent,
    )
}

struct SingleWorld {
    harness: Harness,
    source: JunctionId,
    motor: JunctionId,
    outcome: JunctionId,
}

impl SingleWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(32, 64, OUTWARD_REGION);
        let source = add_junction(&mut builder, 2_000, 0, 0, 1);
        let motor = add_junction(&mut builder, 2_001, 1, 0, 2);
        let sink = add_junction(&mut builder, 2_002, 1, OUTWARD_REGION, 1);
        let outcome = add_junction(&mut builder, 2_003, 50, 0, 1);
        let anchor = add_junction(&mut builder, 2_004, 1_000, 0, 99);
        add_link(&mut builder, anchor, source);
        add_link(&mut builder, anchor, outcome);
        add_link(&mut builder, motor, sink);
        builder.set_outcome_source_for_output(motor, outcome);
        Self {
            harness: builder.build(),
            source,
            motor,
            outcome,
        }
    }

    fn participate(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            physical_input(self.source, tick, 60_000 + tick as u64),
            physical_input(self.motor, tick + 1, 70_000 + tick as u64),
        ])
    }
}

fn delayed_return_lifetime() -> ProbeResult {
    let arm = Arm::DelayedReturnLifetime;
    let mut world = SingleWorld::new();
    let participated = world.participate();
    let before = used_strength(&world.harness, world.motor).expect("single path participates");
    let delayed_tick = world.harness.read().clock.tick.saturating_add(20);
    world.harness.advance_to(delayed_tick);
    let consequence = [physical_input(world.outcome, delayed_tick, 80_000)];
    let (returned, exact_replay) = replay_send(&mut world.harness, &consequence);
    let after = used_strength(&world.harness, world.motor).expect("held path remains live");
    let remaining_returns = world.harness.read().return_path_count;
    let survived = participated.outputs.len() == 1
        && after > before
        && returned.work.local_return_updates == 2
        && remaining_returns == 0;
    finish(
        arm,
        survived,
        serde_json::json!({
            "delay_ticks": 20,
            "strength_before": before,
            "strength_after": after,
            "local_return_updates": returned.work.local_return_updates,
            "remaining_returns": remaining_returns,
        }),
        "the valid delayed consequence did not return and close exactly",
        exact_replay,
        participated.naturally_quiescent && returned.naturally_quiescent,
    )
}

struct PairWorld {
    harness: Harness,
    source: JunctionId,
    motors: [JunctionId; 2],
    outcomes: [JunctionId; 2],
    physical_outputs: [u64; 2],
}

impl PairWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_physical_tracing(true);
        builder.set_protocol(Protocol::UnansweredReturnReplacement);
        let source = add_junction(&mut builder, 3_000, 0, 0, 1);
        let physical_outputs = [3_010, 3_011];
        let motors = [
            add_junction(&mut builder, physical_outputs[0], -1, 0, 2),
            add_junction(&mut builder, physical_outputs[1], 1, 0, 2),
        ];
        let sinks = [
            add_junction(&mut builder, 3_020, -1, OUTWARD_REGION, 1),
            add_junction(&mut builder, 3_021, 1, OUTWARD_REGION, 1),
        ];
        let outcomes = [
            add_junction(&mut builder, 3_030, 50, 0, 1),
            add_junction(&mut builder, 3_031, 60, 0, 1),
        ];
        let anchor = add_junction(&mut builder, 3_040, 1_000, 0, 99);
        for target in [source, outcomes[0], outcomes[1]] {
            add_link(&mut builder, anchor, target);
        }
        for index in 0..2 {
            add_link(&mut builder, motors[index], sinks[index]);
            builder.set_outcome_source_for_output(motors[index], outcomes[index]);
        }
        Self {
            harness: builder.build(),
            source,
            motors,
            outcomes,
            physical_outputs,
        }
    }

    fn stimulate(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            physical_input(self.source, tick, 90_000 + tick as u64),
            physical_input(self.motors[0], tick + 1, 91_000 + tick as u64),
            physical_input(self.motors[1], tick + 1, 92_000 + tick as u64),
        ])
    }

    fn outcome_for(&mut self, physical_output: u64) -> Run {
        let index = self.output_index(physical_output);
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[physical_input(
            self.outcomes[index],
            tick,
            93_000 + tick as u64,
        )])
    }

    fn output_index(&self, physical_output: u64) -> usize {
        self.physical_outputs
            .iter()
            .position(|candidate| *candidate == physical_output)
            .expect("output belongs to pair fixture")
    }

    fn prime_alternative(&mut self) -> (u64, u64, Vec<Run>) {
        let first = self.stimulate();
        let first_output = first.outputs[0].from_physical;
        let returned = self.outcome_for(first_output);
        let reused = self.stimulate();
        let prior = reused.outputs[0].from_physical;
        let alternative = self.stimulate();
        let fresh = alternative.outputs[0].from_physical;
        (prior, fresh, vec![first, returned, reused, alternative])
    }
}

fn local_alternative_exposure() -> ProbeResult {
    let arm = Arm::LocalAlternativeExposure;
    let mut world = PairWorld::new();
    let (prior, fresh, runs) = world.prime_alternative();
    let naturally_quiescent = runs.iter().all(|run| run.naturally_quiescent);
    let checkpoint = world.harness.save().expect("fixture checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("fixture checkpoint restores");
    let tick = world.harness.read().clock.tick.saturating_add(1);
    let inputs = [
        physical_input(world.source, tick, 94_000 + tick as u64),
        physical_input(world.motors[0], tick + 1, 95_000 + tick as u64),
        physical_input(world.motors[1], tick + 1, 96_000 + tick as u64),
    ];
    let observed = world.harness.send(&inputs);
    let replayed = replay.send(&inputs);
    let exact_replay =
        same_replay(&observed, &replayed) && canonical(&world.harness) == canonical(&replay);
    let survived = prior != fresh && runs[3].outputs.len() == 1;
    finish(
        arm,
        survived,
        serde_json::json!({
            "prior_output": prior,
            "fresh_output": fresh,
            "fresh_outputs": runs[3].outputs.len(),
        }),
        "the unanswered winner did not expose exactly one fresh neighbor",
        exact_replay,
        naturally_quiescent && observed.naturally_quiescent,
    )
}

fn displaced_return_closure() -> ProbeResult {
    let arm = Arm::DisplacedReturnClosure;
    let mut world = PairWorld::new();
    let (prior, fresh, runs) = world.prime_alternative();
    let alternative = &runs[3];
    let superseded = alternative
        .physical_trace
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::ReturnSuperseded { .. }))
        .count();
    let remaining_returns = world.harness.read().return_path_count;
    let tick = world.harness.read().clock.tick.saturating_add(1);
    let inputs = [physical_input(
        world.outcomes[world.output_index(fresh)],
        tick,
        97_000,
    )];
    let (returned, exact_replay) = replay_send(&mut world.harness, &inputs);
    let survived = prior != fresh
        && superseded == 1
        && remaining_returns == 1
        && returned.work.local_return_updates == 2
        && world.harness.read().return_path_count == 0;
    finish(
        arm,
        survived,
        serde_json::json!({
            "prior_output": prior,
            "fresh_output": fresh,
            "superseded_returns": superseded,
            "returns_before_fresh_consequence": remaining_returns,
            "fresh_return_updates": returned.work.local_return_updates,
            "returns_after_fresh_consequence": world.harness.read().return_path_count,
        }),
        "replacement did not close only the displaced unanswered return",
        exact_replay,
        runs.iter().all(|run| run.naturally_quiescent) && returned.naturally_quiescent,
    )
}

fn consequence_supported_continuation() -> ProbeResult {
    let arm = Arm::ConsequenceSupportedContinuation;
    let mut world = PairWorld::new();
    let (prior, fresh, mut runs) = world.prime_alternative();
    runs.push(world.outcome_for(fresh));
    let checkpoint = world.harness.save().expect("continuation checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("continuation checkpoint restores");
    let source = world.source;
    let motors = world.motors;
    let fresh_outcome = world.outcomes[world.output_index(fresh)];
    let continue_route = |harness: &mut Harness| {
        let mut winners = Vec::new();
        let mut observed = Vec::new();
        for cycle in 0..4 {
            let tick = harness.read().clock.tick.saturating_add(1);
            let stimulus = harness.send(&[
                physical_input(source, tick, 100_000 + cycle),
                physical_input(motors[0], tick + 1, 101_000 + cycle),
                physical_input(motors[1], tick + 1, 102_000 + cycle),
            ]);
            let winner = stimulus.outputs.first().map(|output| output.from_physical);
            winners.push(winner);
            observed.push(stimulus);
            if winner == Some(fresh) {
                let tick = harness.read().clock.tick.saturating_add(1);
                observed.push(harness.send(&[physical_input(
                    fresh_outcome,
                    tick,
                    103_000 + cycle,
                )]));
            }
        }
        (winners, observed)
    };
    let (winners, continued) = continue_route(&mut world.harness);
    let (replayed_winners, replayed) = continue_route(&mut replay);
    let exact_replay = winners == replayed_winners
        && continued
            .iter()
            .zip(&replayed)
            .all(|(left, right)| same_replay(left, right))
        && canonical(&world.harness) == canonical(&replay);
    let survived =
        prior != fresh && winners.len() == 4 && winners.iter().all(|winner| *winner == Some(fresh));
    finish(
        arm,
        survived,
        serde_json::json!({
            "prior_output": prior,
            "fresh_output": fresh,
            "continuation_outputs": winners,
            "consecutive_fresh_cycles": winners.iter().take_while(|winner| **winner == Some(fresh)).count(),
        }),
        "the fresh consequential route did not survive four consequence-renewed cycles",
        exact_replay,
        runs.iter()
            .chain(&continued)
            .all(|run| run.naturally_quiescent),
    )
}

struct ReversibleWorld {
    harness: Harness,
    sources: [JunctionId; 2],
    motors: [JunctionId; 2],
    outcomes: [JunctionId; 2],
    physical_outputs: [u64; 2],
}

impl ReversibleWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_protocol(Protocol::UnansweredReturnReplacement);
        let sources = [
            add_junction(&mut builder, 4_000, -2, 0, 1),
            add_junction(&mut builder, 4_001, 2, 0, 1),
        ];
        let physical_outputs = [4_010, 4_011];
        let motors = [
            add_junction(&mut builder, physical_outputs[0], -1, 0, 2),
            add_junction(&mut builder, physical_outputs[1], 1, 0, 2),
        ];
        let sinks = [
            add_junction(&mut builder, 4_020, -1, OUTWARD_REGION, 1),
            add_junction(&mut builder, 4_021, 1, OUTWARD_REGION, 1),
        ];
        let outcomes = [
            add_junction(&mut builder, 4_030, 50, 0, 1),
            add_junction(&mut builder, 4_031, 60, 0, 1),
        ];
        let anchor = add_junction(&mut builder, 4_040, 1_000, 0, 99);
        for target in sources.into_iter().chain(outcomes) {
            add_link(&mut builder, anchor, target);
        }
        for index in 0..2 {
            add_link(&mut builder, motors[index], sinks[index]);
            builder.set_outcome_source_for_output(motors[index], outcomes[index]);
        }
        Self {
            harness: builder.build(),
            sources,
            motors,
            outcomes,
            physical_outputs,
        }
    }

    fn train(&mut self, index: usize, repetitions: usize) -> Vec<Run> {
        let mut runs = Vec::new();
        for _ in 0..repetitions {
            let tick = self.harness.read().clock.tick.saturating_add(1);
            runs.push(self.harness.send(&[
                physical_input(self.sources[index], tick, 110_000 + tick as u64),
                physical_input(self.motors[index], tick + 1, 120_000 + tick as u64),
            ]));
            let tick = self.harness.read().clock.tick.saturating_add(1);
            runs.push(self.harness.send(&[physical_input(
                self.outcomes[index],
                tick,
                130_000 + tick as u64,
            )]));
        }
        runs
    }

    fn compete_inputs(&self) -> Vec<Input> {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        vec![
            physical_input(self.sources[0], tick, 140_000 + tick as u64),
            physical_input(self.sources[1], tick, 150_000 + tick as u64),
            physical_input(self.motors[0], tick + 1, 160_000 + tick as u64),
            physical_input(self.motors[1], tick + 1, 170_000 + tick as u64),
        ]
    }
}

fn no_consequence_release() -> ProbeResult {
    let arm = Arm::NoConsequenceRelease;
    let mut world = ReversibleWorld::new();
    let mut runs = world.train(0, 2);
    let first = world.harness.send(&world.compete_inputs());
    let dominant = first.outputs.first().map(|output| output.from_physical);
    let inputs = world.compete_inputs();
    let (released, exact_replay) = replay_send(&mut world.harness, &inputs);
    let next = released.outputs.first().map(|output| output.from_physical);
    runs.push(first);
    let survived = dominant == Some(world.physical_outputs[0])
        && next == Some(world.physical_outputs[1])
        && released.outputs.len() == 1;
    finish(
        arm,
        survived,
        serde_json::json!({
            "dominant_output": dominant,
            "output_after_withheld_consequence": next,
            "released_to_fresh": next == Some(world.physical_outputs[1]),
        }),
        "the unanswered dominant route did not release to one fresh competitor",
        exact_replay,
        runs.iter().all(|run| run.naturally_quiescent) && released.naturally_quiescent,
    )
}

fn reversible_local_competition() -> ProbeResult {
    let arm = Arm::ReversibleLocalCompetition;
    let mut world = ReversibleWorld::new();
    let mut runs = world.train(0, 1);
    let first_strength = used_strength(&world.harness, world.motors[0]).unwrap_or(0);
    runs.extend(world.train(1, 3));
    let later_strength = used_strength(&world.harness, world.motors[1]).unwrap_or(0);
    let inputs = world.compete_inputs();
    let (competition, exact_replay) = replay_send(&mut world.harness, &inputs);
    let winner = competition
        .outputs
        .first()
        .map(|output| output.from_physical);
    let survived = later_strength > first_strength
        && winner == Some(world.physical_outputs[1])
        && competition.outputs.len() == 1;
    finish(
        arm,
        survived,
        serde_json::json!({
            "initial_route_strength": first_strength,
            "contrary_route_strength": later_strength,
            "winner": winner,
            "reversed": winner == Some(world.physical_outputs[1]),
        }),
        "later contrary consequence did not reverse the earlier local advantage",
        exact_replay,
        runs.iter().all(|run| run.naturally_quiescent) && competition.naturally_quiescent,
    )
}

struct FanoutObservation {
    outputs: usize,
    work: u64,
    active_frontier: u64,
    resident_bytes: usize,
    exact_replay: bool,
    naturally_quiescent: bool,
}

fn fanout(size: usize, shared: bool) -> FanoutObservation {
    let capacity = u32::try_from(size.saturating_mul(3).saturating_add(8)).unwrap_or(u32::MAX);
    let mut builder =
        HarnessBuilder::with_capacity(capacity, capacity.saturating_mul(2), OUTWARD_REGION);
    let root = add_junction(&mut builder, 5_000, -100, 0, 1);
    let mut motors = Vec::with_capacity(size);
    for index in 0..size {
        let position = i32::try_from(index).unwrap_or(i32::MAX).saturating_mul(10);
        let motor = add_junction(&mut builder, 5_100 + index as u64, position, 0, 1);
        let sink = add_junction(
            &mut builder,
            6_100 + index as u64,
            position,
            OUTWARD_REGION,
            1,
        );
        add_link(&mut builder, motor, sink);
        add_link(&mut builder, root, motor);
        motors.push(motor);
    }
    let mut harness = builder.build();
    let tick = harness.read().clock.tick.saturating_add(1);
    let inputs = if shared {
        vec![physical_input(root, tick, 180_000)]
    } else {
        motors
            .iter()
            .enumerate()
            .map(|(index, motor)| physical_input(*motor, tick, 181_000 + index as u64))
            .collect()
    };
    let (run, exact_replay) = replay_send(&mut harness, &inputs);
    FanoutObservation {
        outputs: run.outputs.len(),
        work: run.work.total(),
        active_frontier: run.execution_cost.active_frontier_max,
        resident_bytes: harness.read().resident_bytes,
        exact_replay,
        naturally_quiescent: run.naturally_quiescent,
    }
}

fn sparse_shared_opportunity() -> ProbeResult {
    let arm = Arm::SparseSharedOpportunity;
    let shared = fanout(8, true);
    let independent = fanout(8, false);
    let survived = shared.outputs <= 1 && independent.outputs == 8;
    finish(
        arm,
        survived,
        serde_json::json!({
            "available_outputs": 8,
            "shared_root_outputs": shared.outputs,
            "independently_driven_outputs": independent.outputs,
            "shared_work": shared.work,
            "independent_far_outputs_preserved": independent.outputs == 8,
        }),
        "one shared opportunity broadly coactivated every distant output",
        shared.exact_replay && independent.exact_replay,
        shared.naturally_quiescent && independent.naturally_quiescent,
    )
}

fn coalition_separate_credit() -> ProbeResult {
    let arm = Arm::CoalitionSeparateCredit;
    let mut world = ParallelWorld::new();
    let participated = world.participate();
    let before = world.strengths();
    let checkpoint = world.harness.save().expect("coalition checkpoint saves");
    let mut first = Harness::restore(checkpoint.clone()).expect("coalition checkpoint restores");
    let mut second = Harness::restore(checkpoint.clone()).expect("coalition checkpoint restores");
    let mut replay = Harness::restore(checkpoint).expect("coalition checkpoint restores");
    let tick = first.read().clock.tick.saturating_add(1);
    let first_input = [physical_input(world.outcomes[0], tick, 190_000)];
    let second_input = [physical_input(world.outcomes[1], tick, 191_000)];
    let first_run = first.send(&first_input);
    let replayed = replay.send(&first_input);
    let second_run = second.send(&second_input);
    let strengths = |harness: &Harness| {
        std::array::from_fn::<_, 2, _>(|index| {
            used_strength(harness, world.motors[index]).expect("coalition path participates")
        })
    };
    let after_first = strengths(&first);
    let after_second = strengths(&second);
    let exact_replay =
        same_replay(&first_run, &replayed) && canonical(&first) == canonical(&replay);
    let survived = participated.outputs.len() == 2
        && after_first[0] > before[0]
        && after_first[1] == before[1]
        && after_second[0] == before[0]
        && after_second[1] > before[1];
    finish(
        arm,
        survived,
        serde_json::json!({
            "coalition_outputs": participated.outputs.len(),
            "strength_before": before,
            "after_first_consequence": after_first,
            "after_second_consequence": after_second,
        }),
        "coalition consequences did not retain separate causal credit",
        exact_replay,
        participated.naturally_quiescent
            && first_run.naturally_quiescent
            && second_run.naturally_quiescent,
    )
}

fn delayed_multisurface_alignment() -> ProbeResult {
    let arm = Arm::DelayedMultisurfaceAlignment;
    let mut builder = HarnessBuilder::with_capacity(32, 64, OUTWARD_REGION);
    let source = add_junction(&mut builder, 7_000, 0, 0, 1);
    let motor = add_junction(&mut builder, 7_001, 1, 0, 2);
    let sink = add_junction(&mut builder, 7_002, 1, OUTWARD_REGION, 1);
    let outcome_hub = add_junction(&mut builder, 7_003, 50, 0, 1);
    let surfaces = [
        add_junction(&mut builder, 7_004, 100, 0, 1),
        add_junction(&mut builder, 7_005, 110, 0, 1),
    ];
    let anchor = add_junction(&mut builder, 7_006, 1_000, 0, 99);
    add_link(&mut builder, motor, sink);
    add_link(&mut builder, surfaces[0], outcome_hub);
    add_link(&mut builder, surfaces[1], outcome_hub);
    for target in [source, surfaces[0], surfaces[1], outcome_hub] {
        add_link(&mut builder, anchor, target);
    }
    builder.set_outcome_source_for_output(motor, outcome_hub);
    let mut harness = builder.build();
    let tick = harness.read().clock.tick.saturating_add(1);
    let participated = harness.send(&[
        physical_input(source, tick, 200_000),
        physical_input(motor, tick + 1, 200_001),
    ]);
    let before = used_strength(&harness, motor).expect("multisurface path participates");
    let checkpoint = harness.save().expect("multisurface checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("multisurface checkpoint restores");
    let deliver = |body: &mut Harness| {
        let first_tick = body.read().clock.tick.saturating_add(5);
        body.advance_to(first_tick);
        let first = body.send(&[physical_input(surfaces[0], first_tick, 201_000)]);
        let after_first = used_strength(body, motor).expect("multisurface path remains readable");
        let second_tick = body.read().clock.tick.saturating_add(5);
        body.advance_to(second_tick);
        let second = body.send(&[physical_input(surfaces[1], second_tick, 202_000)]);
        let after_second = used_strength(body, motor).unwrap_or(after_first);
        (first, second, after_first, after_second)
    };
    let (first, second, after_first, after_second) = deliver(&mut harness);
    let (first_replay, second_replay, replay_after_first, replay_after_second) =
        deliver(&mut replay);
    let exact_replay = same_replay(&first, &first_replay)
        && same_replay(&second, &second_replay)
        && after_first == replay_after_first
        && after_second == replay_after_second
        && canonical(&harness) == canonical(&replay);
    let survived = after_first > before
        && after_second > after_first
        && first.work.local_return_updates > 0
        && second.work.local_return_updates > 0;
    finish(
        arm,
        survived,
        serde_json::json!({
            "physical_surfaces": 2,
            "shared_local_outcome_hub": true,
            "strength_before": before,
            "strength_after_first_surface": after_first,
            "strength_after_second_surface": after_second,
            "first_surface_return_updates": first.work.local_return_updates,
            "second_surface_return_updates": second.work.local_return_updates,
        }),
        "the first delayed surface closed the action return before the second surface arrived",
        exact_replay,
        participated.naturally_quiescent && first.naturally_quiescent && second.naturally_quiescent,
    )
}

struct SequenceWorld {
    harness: Harness,
    first_input: JunctionId,
    motors: [JunctionId; 2],
    consequences: [JunctionId; 2],
    physical_outputs: [u64; 2],
}

impl SequenceWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        let first_input = add_junction(&mut builder, 8_000, 0, 0, 1);
        let physical_outputs = [8_010, 8_011];
        let motors = [
            add_junction(&mut builder, physical_outputs[0], 1, 0, 2),
            add_junction(&mut builder, physical_outputs[1], 11, 0, 2),
        ];
        let sinks = [
            add_junction(&mut builder, 8_020, 1, OUTWARD_REGION, 1),
            add_junction(&mut builder, 8_021, 11, OUTWARD_REGION, 1),
        ];
        let consequences = [
            add_junction(&mut builder, 8_030, 10, 0, 1),
            add_junction(&mut builder, 8_031, 50, 0, 1),
        ];
        let anchor = add_junction(&mut builder, 8_040, 1_000, 0, 99);
        for target in [first_input, consequences[0], consequences[1]] {
            add_link(&mut builder, anchor, target);
        }
        for index in 0..2 {
            add_link(&mut builder, motors[index], sinks[index]);
            builder.set_outcome_source_for_output(motors[index], consequences[index]);
        }
        Self {
            harness: builder.build(),
            first_input,
            motors,
            consequences,
            physical_outputs,
        }
    }

    fn train(&mut self) -> Vec<Run> {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let first = self.harness.send(&[
            physical_input(self.first_input, tick, 210_000),
            physical_input(self.motors[0], tick + 1, 210_001),
        ]);
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let second = self.harness.send(&[
            physical_input(self.consequences[0], tick, 210_002),
            physical_input(self.motors[1], tick + 1, 210_003),
        ]);
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let returned = self
            .harness
            .send(&[physical_input(self.consequences[1], tick, 210_004)]);
        vec![first, second, returned]
    }

    fn recall(harness: &mut Harness, first_input: JunctionId, consequence: JunctionId) -> Vec<Run> {
        let tick = harness.read().clock.tick.saturating_add(1);
        let first = harness.send(&[physical_input(first_input, tick, 220_000)]);
        let second = if first.outputs.is_empty() {
            Run {
                outputs: Vec::new(),
                work: Default::default(),
                naturally_quiescent: true,
                memory_bytes: first.memory_bytes,
                execution_cost: Default::default(),
                physical_trace: Vec::new(),
            }
        } else {
            let tick = harness.read().clock.tick.saturating_add(1);
            harness.send(&[physical_input(consequence, tick, 220_001)])
        };
        vec![first, second]
    }
}

fn composed_output_sequence() -> ProbeResult {
    let arm = Arm::ComposedOutputSequence;
    let mut world = SequenceWorld::new();
    let training = world.train();
    let checkpoint = world.harness.save().expect("sequence checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("sequence checkpoint restores");
    let recalled =
        SequenceWorld::recall(&mut world.harness, world.first_input, world.consequences[0]);
    let replayed = SequenceWorld::recall(&mut replay, world.first_input, world.consequences[0]);
    let exact_replay = recalled == replayed && canonical(&world.harness) == canonical(&replay);
    let first = recalled[0]
        .outputs
        .first()
        .map(|output| output.from_physical);
    let second = recalled[1]
        .outputs
        .first()
        .map(|output| output.from_physical);
    let survived = first == Some(world.physical_outputs[0])
        && second == Some(world.physical_outputs[1])
        && recalled.iter().all(|run| run.outputs.len() == 1);
    finish(
        arm,
        survived,
        serde_json::json!({
            "recalled_first_output": first,
            "recalled_second_output": second,
            "direct_motor_inputs_during_recall": 0,
        }),
        "recall stopped before the second learned output",
        exact_replay,
        training.iter().all(|run| run.naturally_quiescent)
            && recalled.iter().all(|run| run.naturally_quiescent),
    )
}

fn scale_selectivity() -> ProbeResult {
    let arm = Arm::ScaleSelectivity;
    let small = fanout(4, true);
    let large = fanout(64, true);
    let output_bounded = large.outputs <= small.outputs;
    let work_bounded = large.work <= small.work.saturating_mul(4);
    let survived = output_bounded && work_bounded;
    finish(
        arm,
        survived,
        serde_json::json!({
            "small_available": 4,
            "small_outputs": small.outputs,
            "small_work": small.work,
            "small_active_frontier": small.active_frontier,
            "small_resident_bytes": small.resident_bytes,
            "large_available": 64,
            "large_outputs": large.outputs,
            "large_work": large.work,
            "large_active_frontier": large.active_frontier,
            "large_resident_bytes": large.resident_bytes,
            "output_bounded": output_bounded,
            "work_bounded": work_bounded,
        }),
        "active output or work scaled with the full available surface",
        small.exact_replay && large.exact_replay,
        small.naturally_quiescent && large.naturally_quiescent,
    )
}

fn consolidated_reuse() -> ProbeResult {
    let arm = Arm::ConsolidatedReuse;
    const ROUTES: usize = 64;
    let mut builder = HarnessBuilder::with_capacity(512, 1_024, OUTWARD_REGION);
    let mut sources = Vec::with_capacity(ROUTES);
    let mut motors = Vec::with_capacity(ROUTES);
    let mut outcomes = Vec::with_capacity(ROUTES);
    let mut physical_outputs = Vec::with_capacity(ROUTES);
    let anchor = add_junction(&mut builder, 9_000, 10_000, 0, 99);
    for index in 0..ROUTES {
        let center = i32::try_from(index).unwrap_or(i32::MAX).saturating_mul(10);
        let source = add_junction(&mut builder, 9_100 + index as u64, center, 0, 1);
        let physical = 9_200 + index as u64;
        let motor = add_junction(&mut builder, physical, center + 1, 0, 2);
        let sink = add_junction(
            &mut builder,
            9_300 + index as u64,
            center + 1,
            OUTWARD_REGION,
            1,
        );
        let outcome = add_junction(&mut builder, 9_400 + index as u64, 1_000 + center, 0, 1);
        add_link(&mut builder, anchor, source);
        add_link(&mut builder, anchor, outcome);
        add_link(&mut builder, motor, sink);
        builder.set_outcome_source_for_output(motor, outcome);
        sources.push(source);
        motors.push(motor);
        outcomes.push(outcome);
        physical_outputs.push(physical);
    }
    let mut harness = builder.build();
    let tick = harness.read().clock.tick.saturating_add(1);
    let mut training_inputs = Vec::with_capacity(ROUTES * 2);
    for index in 0..ROUTES {
        training_inputs.push(physical_input(sources[index], tick, 230_000 + index as u64));
        training_inputs.push(physical_input(
            motors[index],
            tick + 1,
            231_000 + index as u64,
        ));
    }
    let participated = harness.send(&training_inputs);
    let tick = harness.read().clock.tick.saturating_add(1);
    let consequence_inputs = outcomes
        .iter()
        .enumerate()
        .map(|(index, outcome)| physical_input(*outcome, tick, 232_000 + index as u64))
        .collect::<Vec<_>>();
    let returned = harness.send(&consequence_inputs);
    let selected = 3;
    let tick = harness.read().clock.tick.saturating_add(1);
    let recall_inputs = [physical_input(sources[selected], tick, 233_000)];
    let (recalled, exact_replay) = replay_send(&mut harness, &recall_inputs);
    let recalled_outputs = recalled
        .outputs
        .iter()
        .map(|output| output.from_physical)
        .collect::<Vec<_>>();
    let survived =
        participated.outputs.len() == ROUTES && recalled_outputs == [physical_outputs[selected]];
    finish(
        arm,
        survived,
        serde_json::json!({
            "trained_routes": ROUTES,
            "training_outputs": participated.outputs.len(),
            "selected_route": selected,
            "recalled_outputs": recalled_outputs,
            "unrelated_outputs": recalled.outputs.len().saturating_sub(1),
        }),
        "one-route recall was absent or activated an unrelated learned route",
        exact_replay,
        participated.naturally_quiescent
            && returned.naturally_quiescent
            && recalled.naturally_quiescent,
    )
}

fn one_joint_composition() -> ProbeResult {
    let arm = Arm::OneJointComposition;
    let predecessor =
        sensorimotor_emergence::run(sensorimotor_emergence::Arm::LocalReturnReplacement, true);
    let single = predecessor
        .stages
        .iter()
        .find(|stage| stage.stage == "single_joint")
        .expect("frozen predecessor contains the one-joint stage");
    let survived = single.status == sensorimotor_emergence::StageStatus::Passed;
    finish(
        arm,
        survived,
        serde_json::json!({
            "predecessor_arm": predecessor.arm,
            "single_joint_status": single.status,
            "single_joint_observations": single.observations,
            "later_stages": "not_run_after_one_joint_failure",
        }),
        "the isolated local laws did not compose into recovery from both joint limits",
        predecessor.exact_replay,
        predecessor.naturally_quiescent,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_integrity(arm: Arm) -> ProbeResult {
        let result = run(arm);
        assert_eq!(result.arm, arm.id());
        assert!(matches!(result.outcome.as_str(), "survived" | "falsified"));
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
        result
    }

    #[test]
    fn probe_causal_local_credit() {
        assert_eq!(assert_integrity(Arm::CausalLocalCredit).outcome, "survived");
    }

    #[test]
    fn probe_delayed_return_lifetime() {
        assert_eq!(
            assert_integrity(Arm::DelayedReturnLifetime).outcome,
            "survived"
        );
    }

    #[test]
    fn probe_local_alternative_exposure() {
        assert_eq!(
            assert_integrity(Arm::LocalAlternativeExposure).outcome,
            "survived"
        );
    }

    #[test]
    fn probe_displaced_return_closure() {
        assert_eq!(
            assert_integrity(Arm::DisplacedReturnClosure).outcome,
            "survived"
        );
    }

    #[test]
    fn probe_consequence_supported_continuation() {
        assert_integrity(Arm::ConsequenceSupportedContinuation);
    }

    #[test]
    fn probe_no_consequence_release() {
        assert_integrity(Arm::NoConsequenceRelease);
    }

    #[test]
    fn probe_reversible_local_competition() {
        assert_integrity(Arm::ReversibleLocalCompetition);
    }

    #[test]
    fn probe_sparse_shared_opportunity() {
        assert_integrity(Arm::SparseSharedOpportunity);
    }

    #[test]
    fn probe_coalition_separate_credit() {
        assert_integrity(Arm::CoalitionSeparateCredit);
    }

    #[test]
    fn probe_delayed_multisurface_alignment() {
        assert_integrity(Arm::DelayedMultisurfaceAlignment);
    }

    #[test]
    fn probe_composed_output_sequence() {
        assert_integrity(Arm::ComposedOutputSequence);
    }

    #[test]
    fn probe_scale_selectivity() {
        assert_integrity(Arm::ScaleSelectivity);
    }

    #[test]
    fn probe_consolidated_reuse() {
        assert_integrity(Arm::ConsolidatedReuse);
    }

    #[test]
    fn probe_one_joint_composition() {
        assert_integrity(Arm::OneJointComposition);
    }
}
