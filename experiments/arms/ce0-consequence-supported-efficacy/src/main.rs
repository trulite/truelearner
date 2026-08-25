#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [5_500_000, 5_600_000];
const PHASES: std::ops::Range<i64> = 0..10;
const PARTICIPATION_QUANTUM: u64 = 1_u64 << 32;
const EXPECTED_CASES: usize = 200;
const EXPECTED_ROWS: usize = 400;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    NoConsequence,
    RepeatedUseOnly,
    PromptConsequence,
    WrongAndLate,
    GradedMagnitude,
    RepeatedSupported,
    ThresholdFamily,
    EqualPersistenceEfficacy,
    FanoutLocality,
    RecurrentStability,
}

impl Family {
    const ALL: [Self; 10] = [
        Self::NoConsequence,
        Self::RepeatedUseOnly,
        Self::PromptConsequence,
        Self::WrongAndLate,
        Self::GradedMagnitude,
        Self::RepeatedSupported,
        Self::ThresholdFamily,
        Self::EqualPersistenceEfficacy,
        Self::FanoutLocality,
        Self::RecurrentStability,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::NoConsequence => "no_consequence",
            Self::RepeatedUseOnly => "repeated_use_only",
            Self::PromptConsequence => "prompt_consequence",
            Self::WrongAndLate => "wrong_and_late",
            Self::GradedMagnitude => "graded_magnitude",
            Self::RepeatedSupported => "repeated_supported",
            Self::ThresholdFamily => "threshold_family",
            Self::EqualPersistenceEfficacy => "equal_persistence_efficacy",
            Self::FanoutLocality => "fanout_locality",
            Self::RecurrentStability => "recurrent_stability",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    updates: u64,
    proposals: u64,
    deallocations: u64,
    qlp: u64,
}

impl WorkTotals {
    fn add(&mut self, work: Work) {
        self.physical = self.physical.saturating_add(work.physical_total());
        self.drive = self.drive.saturating_add(work.drive_deliveries);
        self.modulation = self.modulation.saturating_add(work.modulatory_deliveries);
        self.updates = self.updates.saturating_add(work.local_return_updates);
        self.proposals = self
            .proposals
            .saturating_add(work.local_structural_proposals);
        self.deallocations = self
            .deallocations
            .saturating_add(work.physical_deallocations);
        self.qlp = self.qlp.saturating_add(work.qualified_local_traversals);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ArrowState {
    live: bool,
    resistance: u32,
    coupling: i32,
    participation: u64,
    support: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct EventCounts {
    drive: u64,
    modulation: u64,
    fires: u64,
    resistance: u64,
    coupling: u64,
    proposals: u64,
    deallocations: u64,
    crossings: u64,
    qlp: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<PhysicalTransition>,
    states: Vec<ArrowState>,
    measures: Vec<u64>,
    events: EventCounts,
    work: WorkTotals,
    final_tick: i64,
    body_hash: String,
    live_hash: String,
    quiescent: bool,
}

impl Observation {
    fn physical_eq(&self, other: &Self) -> bool {
        self.trace == other.trace
            && self.states == other.states
            && self.measures == other.measures
            && self.events == other.events
            && self.work == other.work
            && self.final_tick == other.final_tick
            && self.body_hash == other.body_hash
            && self.quiescent == other.quiescent
    }
}

struct Session {
    body: PlasticSubstrate,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    outward_crossings: u64,
    quiescent: bool,
    next_origin: u64,
}

impl Session {
    fn new(root: u64, phase: i64, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 256, 1024, mechanics);
        body.set_physical_tracing(true);
        if phase > 0 {
            body.advance_time(phase);
        }
        Self {
            body,
            trace: Vec::new(),
            work: WorkTotals::default(),
            outward_crossings: 0,
            quiescent: true,
            next_origin: root + 900_000,
        }
    }

    fn cell(&mut self, physical_id: u64, position: i32, region: i16, threshold: i32) -> CellId {
        self.body.add_cell(CellSpec {
            physical_id,
            position,
            region,
            threshold,
            resistance: 100_000,
        })
    }

    fn arrow(
        &mut self,
        from: CellId,
        to: CellId,
        coupling: i32,
        resistance: u32,
        delay: i64,
        mode: TransmissionMode,
    ) -> ArrowId {
        self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay,
            phase: 0,
            coupling,
            resistance,
            mode,
        })
    }

    fn drive(&mut self, from: CellId, to: CellId, coupling: i32, resistance: u32) -> ArrowId {
        self.arrow(from, to, coupling, resistance, 0, TransmissionMode::Drive)
    }

    fn modulation(&mut self, from: CellId, to: CellId) -> ArrowId {
        self.arrow(from, to, 1, 100_000, 0, TransmissionMode::Modulatory)
    }

    fn spike(&mut self, target: CellId, tick: i64, impulse: i32) -> SpikeInput {
        let input = SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: self.next_origin,
            target,
            impulse,
        };
        self.next_origin = self.next_origin.saturating_add(1);
        input
    }

    fn admit(&mut self, inputs: &[SpikeInput]) {
        let result = self.body.arrive(inputs, 1);
        self.outward_crossings = self
            .outward_crossings
            .saturating_add(u64::try_from(result.crossings.len()).unwrap_or(u64::MAX));
        self.work.add(result.work);
        self.quiescent &= result.naturally_quiescent;
        self.trace.extend(result.physical_trace);
    }

    fn advance(&mut self, tick: i64) {
        self.work.add(self.body.advance_time(tick));
    }

    fn state(&self, id: ArrowId) -> ArrowState {
        let durable = self
            .body
            .arena_body(1)
            .arrows
            .into_iter()
            .find(|arrow| arrow.id == id)
            .expect("CE0 ARROW identity must remain addressable");
        ArrowState {
            live: durable.live,
            resistance: durable.resistance,
            coupling: durable.coupling,
            participation: self.body.local_participation(id),
            support: self.body.local_plastic_support(id),
        }
    }

    fn finish(self, states: Vec<ArrowState>, measures: Vec<u64>) -> Observation {
        let events = count_events(&self.trace);
        let body_hash = ContentHash::of(&self.body.canonical_body_bytes(1).unwrap()).to_string();
        let live_hash = ContentHash::of(
            &self
                .body
                .live_checkpoint(1)
                .expect("CE0 live checkpoint")
                .canonical_bytes()
                .expect("CE0 canonical live checkpoint"),
        )
        .to_string();
        Observation {
            trace: self.trace,
            states,
            measures,
            events,
            work: self.work,
            final_tick: self.body.clock().tick,
            body_hash,
            live_hash,
            quiescent: self.quiescent,
        }
    }
}

fn count_events(trace: &[PhysicalTransition]) -> EventCounts {
    let mut counts = EventCounts::default();
    for transition in trace {
        match transition.event {
            PhysicalEvent::Deliver {
                mode: TransmissionMode::Drive,
                ..
            } => counts.drive = counts.drive.saturating_add(1),
            PhysicalEvent::Deliver {
                mode: TransmissionMode::Modulatory,
                ..
            } => counts.modulation = counts.modulation.saturating_add(1),
            PhysicalEvent::Fire { .. } => counts.fires = counts.fires.saturating_add(1),
            PhysicalEvent::Resistance { .. } => {
                counts.resistance = counts.resistance.saturating_add(1)
            }
            PhysicalEvent::Coupling { .. } => {
                counts.coupling = counts.coupling.saturating_add(1)
            }
            PhysicalEvent::Proposal { .. } => counts.proposals = counts.proposals.saturating_add(1),
            PhysicalEvent::Deallocate { .. } => {
                counts.deallocations = counts.deallocations.saturating_add(1)
            }
            PhysicalEvent::Crossing(_) => counts.crossings = counts.crossings.saturating_add(1),
            PhysicalEvent::QualifiedLocalTraversal { .. } => {
                counts.qlp = counts.qlp.saturating_add(1)
            }
        }
    }
    counts
}

fn local_route(
    world: &mut Session,
    root: u64,
    resistance: u32,
    threshold: i32,
) -> (CellId, CellId, CellId, ArrowId) {
    let contact = world.cell(root + 1, 0, 0, 1);
    let target = world.cell(root + 2, 100, 0, threshold);
    let effect = world.cell(root + 3, 200, 0, 1);
    let candidate = world.drive(contact, target, 1, resistance);
    world.modulation(effect, contact);
    (contact, target, effect, candidate)
}

fn no_consequence(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let (contact, _, _, candidate) = local_route(&mut world, root, 100_000, 100);
    let start = world.spike(contact, phase, 1);
    world.admit(&[start]);
    let state = world.state(candidate);
    world.finish(vec![state], vec![state.support])
}

fn repeated_use(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let (contact, _, _, candidate) = local_route(&mut world, root, 100_000, 100);
    for offset in 0..8_i64 {
        let start = world.spike(contact, phase + offset, 1);
        world.admit(&[start]);
    }
    let state = world.state(candidate);
    world.finish(vec![state], vec![state.support])
}

fn prompt(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let (contact, _, effect, candidate) = local_route(&mut world, root, 100_000, 100);
    let start = world.spike(contact, phase, 1);
    world.admit(&[start]);
    let consequence = world.spike(effect, phase, 1);
    world.admit(&[consequence]);
    let state = world.state(candidate);
    world.finish(vec![state], vec![state.support])
}

fn wrong_and_late(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let (a_contact, _, a_effect, a) = local_route(&mut world, root, 100_000, 100);
    let (b_contact, _, b_effect, b) = local_route(&mut world, root + 100, 100_000, 100);
    let a_start = world.spike(a_contact, phase, 1);
    world.admit(&[a_start]);
    let wrong = world.spike(b_effect, phase + 1, 1);
    world.admit(&[wrong]);
    let b_start = world.spike(b_contact, phase + 2, 1);
    world.admit(&[b_start]);
    world.advance(phase + 1_026);
    let late = world.spike(a_effect, phase + 1_026, 1);
    world.admit(&[late]);
    let a_state = world.state(a);
    let b_state = world.state(b);
    world.finish(
        vec![a_state, b_state],
        vec![a_state.support, b_state.support],
    )
}

fn graded(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let (weak_contact, _, weak_effect, weak) = local_route(&mut world, root, 100_000, 100);
    let (full_contact, _, full_effect, full) =
        local_route(&mut world, root + 100, 100_000, 100);
    let (strong_contact, _, strong_effect, strong) =
        local_route(&mut world, root + 200, 100_000, 100);

    let weak_start = world.spike(weak_contact, phase, 1);
    let full_start = world.spike(full_contact, phase, 1);
    world.admit(&[weak_start, full_start]);
    let full_return = world.spike(full_effect, phase, 1);
    world.admit(&[full_return]);
    let weak_return = world.spike(weak_effect, phase + 12, 1);
    world.admit(&[weak_return]);

    for offset in 0..3_i64 {
        let start = world.spike(strong_contact, phase + 20 + offset, 1);
        world.admit(&[start]);
    }
    let strong_return = world.spike(strong_effect, phase + 22, 1);
    world.admit(&[strong_return]);

    let weak_state = world.state(weak);
    let full_state = world.state(full);
    let strong_state = world.state(strong);
    world.finish(
        vec![weak_state, full_state, strong_state],
        vec![weak_state.support, full_state.support, strong_state.support],
    )
}

fn repeated_supported(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let (contact, _, effect, candidate) = local_route(&mut world, root, 100_000, 100);
    let mut couplings = Vec::new();
    for offset in [0_i64, 1_024, 2_048] {
        let start = world.spike(contact, phase + offset, 1);
        world.admit(&[start]);
        let consequence = world.spike(effect, phase + offset, 1);
        world.admit(&[consequence]);
        couplings.push(u64::try_from(world.state(candidate).coupling).unwrap());
    }
    let state = world.state(candidate);
    world.finish(vec![state], couplings)
}

fn threshold_route(
    world: &mut Session,
    root: u64,
    threshold: i32,
    supports: usize,
    start_tick: i64,
) -> (ArrowId, u64) {
    let source = world.cell(root + 1, i32::try_from(root % 100_000).unwrap(), 0, 1);
    let target = world.cell(
        root + 2,
        i32::try_from(root % 100_000).unwrap() + 100,
        0,
        threshold,
    );
    let outlet = world.cell(
        root + 3,
        i32::try_from(root % 100_000).unwrap() + 200,
        1,
        100,
    );
    let effect = world.cell(
        root + 4,
        i32::try_from(root % 100_000).unwrap() + 300,
        0,
        1,
    );
    let candidate = world.drive(source, target, 1, 100_000);
    world.drive(target, outlet, 1, 100_000);
    world.modulation(effect, source);
    for index in 0..supports {
        let tick = start_tick + i64::try_from(index).unwrap() * 1_024;
        let start = world.spike(source, tick, 1);
        world.admit(&[start]);
        let consequence = world.spike(effect, tick, 1);
        world.admit(&[consequence]);
    }
    let probe_tick = start_tick + i64::try_from(supports.max(1)).unwrap() * 1_024;
    let before = world.trace.len();
    let probe = world.spike(source, probe_tick, 1);
    world.admit(&[probe]);
    let target_fires = world.trace[before..]
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == target))
        .count();
    (candidate, u64::try_from(target_fires).unwrap())
}

fn threshold_family(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let (t1, t1_fires) = threshold_route(&mut world, root, 1, 0, phase);
    let (t2, t2_fires) = threshold_route(&mut world, root + 100, 2, 1, phase + 3_000);
    let (t3, t3_fires) = threshold_route(&mut world, root + 200, 3, 2, phase + 6_000);

    let source_a = world.cell(root + 400, 40_000, 0, 1);
    let source_b = world.cell(root + 401, 40_100, 0, 1);
    let target = world.cell(root + 402, 40_200, 0, 2);
    let a = world.drive(source_a, target, 1, 100_000);
    let b = world.drive(source_b, target, 1, 100_000);
    let before = world.trace.len();
    let first = world.spike(source_a, phase + 10_000, 1);
    let second = world.spike(source_b, phase + 10_000, 1);
    world.admit(&[first, second]);
    let two_input_fires = world.trace[before..]
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == target))
        .count();

    let states = vec![world.state(t1), world.state(t2), world.state(t3), world.state(a), world.state(b)];
    world.finish(
        states,
        vec![t1_fires, t2_fires, t3_fires, u64::try_from(two_input_fires).unwrap()],
    )
}

fn equal_persistence(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let source_a = world.cell(root + 1, 0, 0, 1);
    let source_b = world.cell(root + 2, 300, 0, 1);
    let target_a = world.cell(root + 3, 100, 0, 2);
    let target_b = world.cell(root + 4, 400, 0, 2);
    let a = world.drive(source_a, target_a, 1, 4);
    let b = world.drive(source_b, target_b, 2, 4);
    let first = world.spike(source_a, phase, 1);
    let second = world.spike(source_b, phase, 1);
    world.admit(&[first, second]);
    let a_fires = world.trace.iter().filter(|transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == target_a)).count();
    let b_fires = world.trace.iter().filter(|transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == target_b)).count();
    world.advance(phase + 39);
    let a_before = world.state(a);
    let b_before = world.state(b);
    world.advance(phase + 40);
    let a_after = world.state(a);
    let b_after = world.state(b);
    world.finish(
        vec![a_before, b_before, a_after, b_after],
        vec![u64::try_from(a_fires).unwrap(), u64::try_from(b_fires).unwrap()],
    )
}

fn fanout(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let contact = world.cell(root + 1, 0, 0, 1);
    let target_a = world.cell(root + 2, 100, 0, 100);
    let target_b = world.cell(root + 3, 200, 0, 100);
    let effect = world.cell(root + 4, 300, 0, 1);
    let a = world.drive(contact, target_a, 1, 100_000);
    let b = world.drive(contact, target_b, 1, 100_000);
    world.modulation(effect, contact);

    let other_contact = world.cell(root + 5, 500, 0, 1);
    let other_target = world.cell(root + 6, 600, 0, 100);
    let other = world.drive(other_contact, other_target, 1, 100_000);

    let start = world.spike(contact, phase, 1);
    world.admit(&[start]);
    let consequence = world.spike(effect, phase, 1);
    world.admit(&[consequence]);
    let a_state = world.state(a);
    let b_state = world.state(b);
    let other_state = world.state(other);
    world.finish(
        vec![a_state, b_state, other_state],
        vec![a_state.support, b_state.support, other_state.support],
    )
}

fn recurrent(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let a = world.cell(root + 1, 0, 0, 2);
    let b = world.cell(root + 2, 100, 0, 2);
    let effect_a = world.cell(root + 3, 300, 0, 1);
    let effect_b = world.cell(root + 4, 400, 0, 1);
    let ab = world.arrow(a, b, 1, 4, 1, TransmissionMode::Drive);
    let ba = world.arrow(b, a, 1, 4, 1, TransmissionMode::Drive);
    world.modulation(effect_a, a);
    world.modulation(effect_b, b);

    let pre_start = world.spike(a, phase, 2);
    world.admit(&[pre_start]);
    let pre_refires = world.trace.iter().filter(|transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == a)).count();

    let train_a = world.spike(a, phase + 3, 2);
    let return_a = world.spike(effect_a, phase + 3, 1);
    world.admit(&[train_a, return_a]);
    let train_b = world.spike(b, phase + 6, 2);
    let return_b = world.spike(effect_b, phase + 6, 1);
    world.admit(&[train_b, return_b]);

    let before_probe = world.trace.len();
    let probe = world.spike(a, phase + 9, 2);
    world.admit(&[probe]);
    let probe_a_fires = world.trace[before_probe..]
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == a))
        .count();
    let probe_b_fires = world.trace[before_probe..]
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == b))
        .count();
    let ab_state = world.state(ab);
    let ba_state = world.state(ba);
    world.finish(
        vec![ab_state, ba_state],
        vec![
            u64::try_from(pre_refires).unwrap(),
            u64::try_from(probe_a_fires).unwrap(),
            u64::try_from(probe_b_fires).unwrap(),
        ],
    )
}

fn run_case(root: u64, phase: i64, family: Family, mechanics: MechanicalConfig) -> Observation {
    let family_root = root.saturating_add(
        u64::try_from(Family::ALL.iter().position(|item| *item == family).unwrap()).unwrap()
            * 100_000,
    );
    match family {
        Family::NoConsequence => no_consequence(family_root, phase, mechanics),
        Family::RepeatedUseOnly => repeated_use(family_root, phase, mechanics),
        Family::PromptConsequence => prompt(family_root, phase, mechanics),
        Family::WrongAndLate => wrong_and_late(family_root, phase, mechanics),
        Family::GradedMagnitude => graded(family_root, phase, mechanics),
        Family::RepeatedSupported => repeated_supported(family_root, phase, mechanics),
        Family::ThresholdFamily => threshold_family(family_root, phase, mechanics),
        Family::EqualPersistenceEfficacy => equal_persistence(family_root, phase, mechanics),
        Family::FanoutLocality => fanout(family_root, phase, mechanics),
        Family::RecurrentStability => recurrent(family_root, phase, mechanics),
    }
}

fn predicate(family: Family, observation: &Observation) -> bool {
    if !observation.quiescent || observation.events.proposals != 0 {
        return false;
    }
    match family {
        Family::NoConsequence | Family::RepeatedUseOnly => {
            observation.states == [ArrowState {
                live: true,
                resistance: observation.states[0].resistance,
                coupling: 1,
                participation: observation.states[0].participation,
                support: 0,
            }]
                && observation.events.coupling == 0
                && observation.work.updates == 0
        }
        Family::PromptConsequence => {
            observation.states.len() == 1
                && observation.states[0].coupling == 2
                && observation.states[0].resistance == 100_003
                && observation.states[0].support == PARTICIPATION_QUANTUM
                && observation.events.coupling == 1
                && observation.work.updates == 1
        }
        Family::WrongAndLate => {
            observation.states.iter().all(|state| state.coupling == 1)
                && observation.states.iter().all(|state| state.support == 0)
                && observation.events.coupling == 0
        }
        Family::GradedMagnitude => {
            observation.states.len() == 3
                && observation.states[0].coupling == 1
                && observation.states[1].coupling == 2
                && observation.states[2].coupling == 3
                && observation.measures[0] < PARTICIPATION_QUANTUM
                && observation.measures[1] == PARTICIPATION_QUANTUM
                && observation.measures[2] >= PARTICIPATION_QUANTUM.saturating_mul(2)
                && observation.events.coupling == 2
        }
        Family::RepeatedSupported => {
            observation.measures == [2, 3, 4]
                && observation.states[0].coupling == 4
                && observation.events.coupling == 3
        }
        Family::ThresholdFamily => {
            observation.measures == [1, 1, 1, 1]
                && observation.states[0].coupling == 1
                && observation.states[1].coupling == 2
                && observation.states[2].coupling == 3
                && observation.states[3].coupling == 1
                && observation.states[4].coupling == 1
        }
        Family::EqualPersistenceEfficacy => {
            observation.measures == [0, 1]
                && observation.states[0].live
                && observation.states[1].live
                && observation.states[0].resistance == observation.states[1].resistance
                && observation.states[0].coupling == 1
                && observation.states[1].coupling == 2
                && !observation.states[2].live
                && !observation.states[3].live
        }
        Family::FanoutLocality => {
            observation.states[0].coupling == 2
                && observation.states[1].coupling == 2
                && observation.states[2].coupling == 1
                && observation.measures[0] == PARTICIPATION_QUANTUM
                && observation.measures[1] == PARTICIPATION_QUANTUM
                && observation.measures[2] == 0
                && observation.events.coupling == 2
        }
        Family::RecurrentStability => {
            observation.states[0].coupling == 2
                && observation.states[1].coupling == 2
                && observation.measures[0] == 1
                && observation.measures[1] == 1
                && observation.measures[2] == 1
        }
    }
}

fn mechanics_name(mechanics: MechanicalConfig) -> &'static str {
    if mechanics == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn trace_hash(trace: &[PhysicalTransition]) -> String {
    ContentHash::of(format!("{trace:?}").as_bytes()).to_string()
}

fn states_text(states: &[ArrowState]) -> String {
    states
        .iter()
        .map(|state| {
            format!(
                "{}/{}/{}/{}/{}",
                u8::from(state.live),
                state.resistance,
                state.coupling,
                state.participation,
                state.support
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn measures_text(measures: &[u64]) -> String {
    measures
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/ce0_consequence_supported_efficacy_v1"));
    fs::create_dir_all(&output).expect("create CE0 output directory");

    let mut csv = String::from(
        "case_id,root,phase,family,mechanics,states,measures,drive,modulation,fires,resistance_events,coupling_events,qlp,crossings,proposals,deallocations,physical_work,final_tick,trace_hash,body_hash,live_hash,quiescent,replay_equal,mechanics_equal,predicate_pass,case_pass\n",
    );
    let mut case_id = 0_usize;
    let mut rows = 0_usize;
    let mut maximum_work = 0_u64;
    let mut all_pass = true;
    let mut all_replay = true;
    let mut all_mechanics = true;
    let mut all_predicates = true;

    for root in ROOTS {
        for phase in PHASES {
            for family in Family::ALL {
                case_id += 1;
                let reference = run_case(root, phase, family, MechanicalConfig::REFERENCE);
                let reference_replay = run_case(root, phase, family, MechanicalConfig::REFERENCE);
                let production = run_case(root, phase, family, MechanicalConfig::PRODUCTION);
                let production_replay = run_case(root, phase, family, MechanicalConfig::PRODUCTION);
                let reference_replay_equal = reference == reference_replay;
                let production_replay_equal = production == production_replay;
                let mechanics_equal = reference.physical_eq(&production);
                let reference_pass = predicate(family, &reference);
                let production_pass = predicate(family, &production);
                let case_pass = reference_replay_equal
                    && production_replay_equal
                    && mechanics_equal
                    && reference_pass
                    && production_pass;
                all_pass &= case_pass;
                all_replay &= reference_replay_equal && production_replay_equal;
                all_mechanics &= mechanics_equal;
                all_predicates &= reference_pass && production_pass;

                for (mechanics, observation, replay_equal, predicate_pass) in [
                    (
                        MechanicalConfig::REFERENCE,
                        &reference,
                        reference_replay_equal,
                        reference_pass,
                    ),
                    (
                        MechanicalConfig::PRODUCTION,
                        &production,
                        production_replay_equal,
                        production_pass,
                    ),
                ] {
                    rows += 1;
                    maximum_work = maximum_work.max(observation.work.physical);
                    writeln!(
                        csv,
                        "{case_id},{root},{phase},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        family.name(),
                        mechanics_name(mechanics),
                        states_text(&observation.states),
                        measures_text(&observation.measures),
                        observation.events.drive,
                        observation.events.modulation,
                        observation.events.fires,
                        observation.events.resistance,
                        observation.events.coupling,
                        observation.events.qlp,
                        observation.events.crossings,
                        observation.events.proposals,
                        observation.events.deallocations,
                        observation.work.physical,
                        observation.final_tick,
                        trace_hash(&observation.trace),
                        observation.body_hash,
                        observation.live_hash,
                        observation.quiescent,
                        replay_equal,
                        mechanics_equal,
                        predicate_pass,
                        case_pass,
                    )
                    .unwrap();
                }
            }
        }
    }

    assert_eq!(case_id, EXPECTED_CASES);
    assert_eq!(rows, EXPECTED_ROWS);
    let matrix_path = output.join("matrix.csv");
    fs::write(&matrix_path, csv).expect("write CE0 matrix");
    let matrix_hash = ContentHash::of(&fs::read(&matrix_path).unwrap()).to_string();
    let claim = if all_pass {
        "CE0 establishes a general local consequence-supported efficacy law."
    } else {
        "CE0 is an immutable negative: at least one frozen condition failed."
    };
    let report = format!(
        "# CE0 consequence-supported efficacy v1\n\n\
         - cases: `{case_id}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{rows}/{EXPECTED_ROWS}`\n\
         - exact same-mechanics replay: `{all_replay}`\n\
         - exact Reference/Production physical agreement: `{all_mechanics}`\n\
         - all frozen predicates: `{all_predicates}`\n\
         - maximum PhysicalWork: `{maximum_work}`\n\
         - matrix SHA-256: `{matrix_hash}`\n\n\
         {claim}\n"
    );
    fs::write(output.join("report.md"), report).expect("write CE0 report");
    assert!(all_pass, "CE0 matrix failed");
    println!("CE0_CONSEQUENCE_SUPPORTED_EFFICACY_V1_PASS");
    println!("matrix_sha256={matrix_hash}");
}

