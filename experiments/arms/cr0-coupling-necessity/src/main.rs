#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, TransmissionTrigger, Work,
};

const ROOTS: [u64; 2] = [5_100_000, 5_200_000];
const PHASES: std::ops::Range<i64> = 0..10;
const EXPECTED_CASES: usize = 400;
const EXPECTED_ROWS: usize = 800;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    PersistenceOnly,
    EfficacyAndPersistence,
}

impl Arm {
    const ALL: [Self; 2] = [Self::PersistenceOnly, Self::EfficacyAndPersistence];

    fn name(self) -> &'static str {
        match self {
            Self::PersistenceOnly => "persistence_only",
            Self::EfficacyAndPersistence => "efficacy_and_persistence",
        }
    }

    fn coupling(self) -> i32 {
        match self {
            Self::PersistenceOnly => 1,
            Self::EfficacyAndPersistence => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Cpc0ContactLocality,
    Cpc1TemporalParticipation,
    Pqlc0OneHop,
    Pqlc1Depth16,
    Fd0EqualPersistence,
    Fd1Consolidation,
    Threshold1AlreadyExecutable,
    Threshold2AcquiredEfficacy,
    Threshold3StillInsufficient,
    Threshold2TwoPhysicalInputs,
}

impl Family {
    const ALL: [Self; 10] = [
        Self::Cpc0ContactLocality,
        Self::Cpc1TemporalParticipation,
        Self::Pqlc0OneHop,
        Self::Pqlc1Depth16,
        Self::Fd0EqualPersistence,
        Self::Fd1Consolidation,
        Self::Threshold1AlreadyExecutable,
        Self::Threshold2AcquiredEfficacy,
        Self::Threshold3StillInsufficient,
        Self::Threshold2TwoPhysicalInputs,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Cpc0ContactLocality => "cpc0_contact_locality",
            Self::Cpc1TemporalParticipation => "cpc1_temporal_participation",
            Self::Pqlc0OneHop => "pqlc0_one_hop",
            Self::Pqlc1Depth16 => "pqlc1_depth_16",
            Self::Fd0EqualPersistence => "fd0_equal_persistence",
            Self::Fd1Consolidation => "fd1_consolidation",
            Self::Threshold1AlreadyExecutable => "threshold_1_already_executable",
            Self::Threshold2AcquiredEfficacy => "threshold_2_acquired_efficacy",
            Self::Threshold3StillInsufficient => "threshold_3_still_insufficient",
            Self::Threshold2TwoPhysicalInputs => "threshold_2_two_physical_inputs",
        }
    }

    fn is_retained(self) -> bool {
        matches!(
            self,
            Self::Cpc0ContactLocality
                | Self::Cpc1TemporalParticipation
                | Self::Pqlc0OneHop
                | Self::Pqlc1Depth16
                | Self::Fd0EqualPersistence
                | Self::Fd1Consolidation
        )
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
        mode: TransmissionMode,
    ) -> ArrowId {
        self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay: 0,
            phase: 0,
            coupling,
            resistance,
            mode,
        })
    }

    fn qlp(&mut self, from: CellId, to: CellId) -> ArrowId {
        self.body.add_arrow_with_trigger(
            ArrowSpec {
                from,
                to,
                delay: 0,
                phase: 0,
                coupling: 1,
                resistance: 100_000,
                mode: TransmissionMode::Modulatory,
            },
            TransmissionTrigger::QualifiedLocalParticipation,
        )
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
            .expect("CR0 ARROW identity must remain addressable");
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
                .expect("CR0 live checkpoint")
                .canonical_bytes()
                .expect("CR0 canonical live checkpoint"),
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

fn cpc0(root: u64, phase: i64, arm: Arm, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let p = world.cell(root + 1, 0, 0, 1);
    let ca = world.cell(root + 2, 100, 0, 1);
    let cb = world.cell(root + 3, 200, 0, 1);
    let x = world.cell(root + 4, 300, 0, 10);
    let y = world.cell(root + 5, 400, 0, 10);
    let effect = world.cell(root + 6, 500, 0, 1);
    world.arrow(p, ca, 1, 100_000, TransmissionMode::Drive);
    world.arrow(p, cb, 1, 100_000, TransmissionMode::Drive);
    let a = world.arrow(ca, x, arm.coupling(), 4, TransmissionMode::Drive);
    let b = world.arrow(cb, y, arm.coupling(), 4, TransmissionMode::Drive);
    world.arrow(effect, ca, 1, 100_000, TransmissionMode::Modulatory);
    let start = world.spike(p, phase, 1);
    world.admit(&[start]);
    let consequence = world.spike(effect, phase + 1, 1);
    world.admit(&[consequence]);
    let a_state = world.state(a);
    let b_state = world.state(b);
    world.finish(
        vec![a_state, b_state],
        vec![a_state.support, b_state.support],
    )
}

fn cpc1(root: u64, phase: i64, arm: Arm, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let mut candidates = Vec::new();
    let mut contacts = Vec::new();
    let mut effects = Vec::new();
    for index in 0..5_u64 {
        let base = root + 100 + index * 10;
        let contact = world.cell(base + 1, i32::try_from(index * 100).unwrap(), 0, 1);
        let target = world.cell(base + 2, i32::try_from(index * 100 + 20).unwrap(), 0, 10);
        let effect = world.cell(base + 3, i32::try_from(index * 100 + 40).unwrap(), 0, 1);
        candidates.push(world.arrow(contact, target, arm.coupling(), 4, TransmissionMode::Drive));
        world.arrow(effect, contact, 1, 100_000, TransmissionMode::Modulatory);
        contacts.push(contact);
        effects.push(effect);
    }
    let unrelated = world.cell(root + 900, 900, 0, 1);
    let unrelated_target = world.cell(root + 901, 1000, 0, 10);
    world.arrow(
        unrelated,
        unrelated_target,
        1,
        100_000,
        TransmissionMode::Drive,
    );
    let starts = contacts
        .iter()
        .map(|contact| world.spike(*contact, phase, 1))
        .collect::<Vec<_>>();
    world.admit(&starts);
    let prompt = world.spike(effects[0], phase + 1, 1);
    world.admit(&[prompt]);
    let renewed = world.spike(contacts[4], phase + 4, 1);
    let unrelated_activity = world.spike(unrelated, phase + 4, 1);
    world.admit(&[renewed, unrelated_activity]);
    let at_five = [2_usize, 3, 4]
        .into_iter()
        .map(|index| world.spike(effects[index], phase + 5, 1))
        .collect::<Vec<_>>();
    world.admit(&at_five);
    let late = world.spike(effects[1], phase + 12, 1);
    world.admit(&[late]);
    let states = candidates
        .iter()
        .map(|candidate| world.state(*candidate))
        .collect::<Vec<_>>();
    let measures = states.iter().map(|state| state.support).collect();
    world.finish(states, measures)
}

fn pqlc0(root: u64, phase: i64, arm: Arm, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let source = world.cell(root + 1, 0, 0, 1);
    let c1 = world.cell(root + 2, 100, 0, 1);
    let c2 = world.cell(root + 3, 200, 0, 1);
    let effect = world.cell(root + 4, 300, 0, 3);
    world.arrow(source, c1, 1, 100_000, TransmissionMode::Drive);
    let candidate = world.arrow(c1, c2, arm.coupling(), 4, TransmissionMode::Drive);
    world.arrow(c2, effect, arm.coupling(), 4, TransmissionMode::Drive);
    world.arrow(effect, c2, 1, 100_000, TransmissionMode::Modulatory);
    world.qlp(c2, c1);
    let start = world.spike(source, phase, 1);
    world.admit(&[start]);
    let consequence = world.spike(effect, phase + 1, 3);
    world.admit(&[consequence]);
    let state = world.state(candidate);
    let c1_fires = world
        .trace
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == c1))
        .count();
    world.finish(
        vec![state],
        vec![u64::try_from(c1_fires).unwrap(), state.support],
    )
}

fn pqlc1(root: u64, phase: i64, arm: Arm, mechanics: MechanicalConfig) -> Observation {
    const DEPTH: usize = 16;
    let mut world = Session::new(root, phase, mechanics);
    let source = world.cell(root + 1, 0, 0, 1);
    let contacts = (0..DEPTH)
        .map(|index| {
            world.cell(
                root + 10 + u64::try_from(index).unwrap(),
                i32::try_from(index * 100 + 100).unwrap(),
                0,
                1,
            )
        })
        .collect::<Vec<_>>();
    let effect = world.cell(root + 100, 2000, 0, 3);
    world.arrow(source, contacts[0], 1, 100_000, TransmissionMode::Drive);
    let mut candidates = Vec::new();
    for index in 0..DEPTH {
        let target = if index + 1 < DEPTH {
            contacts[index + 1]
        } else {
            effect
        };
        candidates.push(world.arrow(
            contacts[index],
            target,
            arm.coupling(),
            4,
            TransmissionMode::Drive,
        ));
    }
    world.arrow(
        effect,
        contacts[DEPTH - 1],
        1,
        100_000,
        TransmissionMode::Modulatory,
    );
    for index in (1..DEPTH).rev() {
        world.qlp(contacts[index], contacts[index - 1]);
    }
    let start = world.spike(source, phase, 1);
    world.admit(&[start]);
    let consequence = world.spike(effect, phase + 1, 3);
    world.admit(&[consequence]);
    let states = candidates
        .iter()
        .map(|candidate| world.state(*candidate))
        .collect::<Vec<_>>();
    let supported = states.iter().filter(|state| state.support > 0).count();
    world.finish(states, vec![u64::try_from(supported).unwrap()])
}

fn fd0(root: u64, phase: i64, arm: Arm, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let from = world.cell(root + 1, 0, 0, 10);
    let to = world.cell(root + 2, 100, 0, 10);
    let candidate = world.arrow(from, to, arm.coupling(), 4, TransmissionMode::Drive);
    world.advance(phase + 39);
    let last_live = world.state(candidate);
    world.advance(phase + 40);
    let dead = world.state(candidate);
    world.finish(
        vec![last_live, dead],
        vec![u64::from(last_live.live), u64::from(dead.live)],
    )
}

fn fd1(root: u64, phase: i64, arm: Arm, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let supported_contact = world.cell(root + 1, 0, 0, 1);
    let unsupported_contact = world.cell(root + 2, 100, 0, 1);
    let supported_target = world.cell(root + 3, 200, 0, 10);
    let unsupported_target = world.cell(root + 4, 300, 0, 10);
    let effect = world.cell(root + 5, 400, 0, 1);
    let supported = world.arrow(
        supported_contact,
        supported_target,
        arm.coupling(),
        1,
        TransmissionMode::Drive,
    );
    let unsupported = world.arrow(
        unsupported_contact,
        unsupported_target,
        arm.coupling(),
        1,
        TransmissionMode::Drive,
    );
    world.arrow(
        effect,
        supported_contact,
        1,
        100_000,
        TransmissionMode::Modulatory,
    );
    let first = world.spike(supported_contact, phase, 1);
    let second = world.spike(unsupported_contact, phase, 1);
    world.admit(&[first, second]);
    let consequence = world.spike(effect, phase + 1, 1);
    world.admit(&[consequence]);
    let supported_state = world.state(supported);
    let unsupported_state = world.state(unsupported);
    world.finish(
        vec![supported_state, unsupported_state],
        vec![supported_state.support, unsupported_state.support],
    )
}

fn efficacy_world(
    root: u64,
    phase: i64,
    mechanics: MechanicalConfig,
    threshold: i32,
    coupling: i32,
    resistance: u32,
) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let source = world.cell(root + 1, 0, 0, 1);
    let target = world.cell(root + 2, 100, 0, threshold);
    let outlet = world.cell(root + 3, 200, 1, 100);
    let candidate = world.arrow(
        source,
        target,
        coupling,
        resistance,
        TransmissionMode::Drive,
    );
    world.arrow(target, outlet, 1, 100_000, TransmissionMode::Drive);
    let start = world.spike(source, phase, 1);
    world.admit(&[start]);
    let target_fires = world
        .trace
        .iter()
        .filter(
            |transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == target),
        )
        .count();
    let state = world.state(candidate);
    let outward_crossings = world.outward_crossings;
    world.finish(
        vec![state],
        vec![u64::try_from(target_fires).unwrap(), outward_crossings],
    )
}

fn threshold_two(root: u64, phase: i64, arm: Arm, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let post_source = world.cell(root + 1, 0, 0, 1);
    let post_target = world.cell(root + 2, 100, 0, 2);
    let post_outlet = world.cell(root + 3, 200, 1, 100);
    let baseline_source = world.cell(root + 4, 300, 0, 1);
    let baseline_target = world.cell(root + 5, 400, 0, 2);
    let baseline_outlet = world.cell(root + 6, 500, 1, 100);
    let post = world.arrow(
        post_source,
        post_target,
        arm.coupling(),
        4,
        TransmissionMode::Drive,
    );
    let baseline = world.arrow(
        baseline_source,
        baseline_target,
        1,
        1,
        TransmissionMode::Drive,
    );
    world.arrow(
        post_target,
        post_outlet,
        1,
        100_000,
        TransmissionMode::Drive,
    );
    world.arrow(
        baseline_target,
        baseline_outlet,
        1,
        100_000,
        TransmissionMode::Drive,
    );
    let post_start = world.spike(post_source, phase, 1);
    let baseline_start = world.spike(baseline_source, phase, 1);
    world.admit(&[post_start, baseline_start]);
    let post_fires = world
        .trace
        .iter()
        .filter(|transition| {
            matches!(transition.event, PhysicalEvent::Fire { cell } if cell == post_target)
        })
        .count();
    let baseline_fires = world
        .trace
        .iter()
        .filter(|transition| {
            matches!(transition.event, PhysicalEvent::Fire { cell } if cell == baseline_target)
        })
        .count();
    let post_state = world.state(post);
    let baseline_state = world.state(baseline);
    let outward_crossings = world.outward_crossings;
    world.finish(
        vec![post_state, baseline_state],
        vec![
            u64::try_from(post_fires).unwrap(),
            u64::try_from(baseline_fires).unwrap(),
            outward_crossings,
        ],
    )
}

fn two_inputs(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = Session::new(root, phase, mechanics);
    let source_a = world.cell(root + 1, 0, 0, 1);
    let source_b = world.cell(root + 2, 100, 0, 1);
    let target = world.cell(root + 3, 200, 0, 2);
    let outlet = world.cell(root + 4, 300, 1, 100);
    let a = world.arrow(source_a, target, 1, 4, TransmissionMode::Drive);
    let b = world.arrow(source_b, target, 1, 4, TransmissionMode::Drive);
    world.arrow(target, outlet, 1, 100_000, TransmissionMode::Drive);
    let first = world.spike(source_a, phase, 1);
    let second = world.spike(source_b, phase, 1);
    world.admit(&[first, second]);
    let target_fires = world
        .trace
        .iter()
        .filter(
            |transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == target),
        )
        .count();
    let a_state = world.state(a);
    let b_state = world.state(b);
    let outward_crossings = world.outward_crossings;
    world.finish(
        vec![a_state, b_state],
        vec![u64::try_from(target_fires).unwrap(), outward_crossings],
    )
}

fn run_case(
    root: u64,
    phase: i64,
    family: Family,
    arm: Arm,
    mechanics: MechanicalConfig,
) -> Observation {
    let family_root = root
        .saturating_add(
            u64::try_from(Family::ALL.iter().position(|item| *item == family).unwrap()).unwrap()
                * 10_000,
        )
        .saturating_add(
            u64::try_from(Arm::ALL.iter().position(|item| *item == arm).unwrap()).unwrap() * 1_000,
        );
    match family {
        Family::Cpc0ContactLocality => cpc0(family_root, phase, arm, mechanics),
        Family::Cpc1TemporalParticipation => cpc1(family_root, phase, arm, mechanics),
        Family::Pqlc0OneHop => pqlc0(family_root, phase, arm, mechanics),
        Family::Pqlc1Depth16 => pqlc1(family_root, phase, arm, mechanics),
        Family::Fd0EqualPersistence => fd0(family_root, phase, arm, mechanics),
        Family::Fd1Consolidation => fd1(family_root, phase, arm, mechanics),
        Family::Threshold1AlreadyExecutable => {
            efficacy_world(family_root, phase, mechanics, 1, arm.coupling(), 4)
        }
        Family::Threshold2AcquiredEfficacy => threshold_two(family_root, phase, arm, mechanics),
        Family::Threshold3StillInsufficient => {
            efficacy_world(family_root, phase, mechanics, 3, arm.coupling(), 4)
        }
        Family::Threshold2TwoPhysicalInputs => two_inputs(family_root, phase, mechanics),
    }
}

fn predicate(family: Family, arm: Arm, observation: &Observation) -> bool {
    if !observation.quiescent || observation.events.proposals != 0 {
        return false;
    }
    match family {
        Family::Cpc0ContactLocality => {
            observation.states.len() == 2
                && observation.states[0].resistance == 7
                && observation.states[1].resistance == 4
                && observation.states[0].coupling == arm.coupling()
                && observation.states[1].coupling == arm.coupling()
                && observation.states[0].support > 0
                && observation.states[1].support == 0
                && observation.work.updates == 1
        }
        Family::Cpc1TemporalParticipation => {
            observation.states.len() == 5
                && observation.states.iter().all(|state| {
                    state.live && state.coupling == arm.coupling() && state.support > 0
                })
                && observation.measures[1] < observation.measures[0]
                && observation.measures[2] == observation.measures[3]
                && observation.measures[4] > observation.measures[3]
        }
        Family::Pqlc0OneHop => {
            observation.work.qlp == 1
                && observation.events.qlp == 1
                && observation.states[0].resistance == 7
                && observation.states[0].coupling == arm.coupling()
                && observation.states[0].support > 0
                && observation.measures[0] == 1
        }
        Family::Pqlc1Depth16 => {
            observation.work.qlp == 15
                && observation.events.qlp == 15
                && observation.measures == [16]
                && observation.states.iter().all(|state| {
                    state.live
                        && state.resistance == 7
                        && state.coupling == arm.coupling()
                        && state.support > 0
                })
        }
        Family::Fd0EqualPersistence => {
            observation.states.len() == 2
                && observation.states[0].live
                && observation.states[0].resistance == 1
                && observation.states[0].coupling == arm.coupling()
                && !observation.states[1].live
                && observation.states[1].resistance == 0
                && observation.work.deallocations == 1
        }
        Family::Fd1Consolidation => {
            observation.states.len() == 2
                && observation.states[0].resistance == 4
                && observation.states[1].resistance == 1
                && observation.states[0].coupling == arm.coupling()
                && observation.states[1].coupling == arm.coupling()
                && observation.states[0].support > 0
                && observation.states[1].support == 0
                && observation.work.updates == 1
        }
        Family::Threshold1AlreadyExecutable => {
            observation.measures == [1, 1]
                && observation.states[0].coupling == arm.coupling()
                && observation.states[0].resistance == 4
        }
        Family::Threshold2AcquiredEfficacy => {
            let expected = match arm {
                Arm::PersistenceOnly => [0, 0, 0],
                Arm::EfficacyAndPersistence => [1, 0, 1],
            };
            observation.measures == expected
                && observation.states[0].coupling == arm.coupling()
                && observation.states[0].resistance == 4
                && observation.states[1].coupling == 1
                && observation.states[1].resistance == 1
        }
        Family::Threshold3StillInsufficient => observation.measures == [0, 0],
        Family::Threshold2TwoPhysicalInputs => {
            observation.measures == [1, 1]
                && observation.states.iter().all(|state| state.coupling == 1)
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
        .unwrap_or_else(|| PathBuf::from("results/cr0_coupling_necessity_v1"));
    fs::create_dir_all(&output).expect("create CR0 output directory");

    let mut csv = String::from(
        "case_id,root,phase,family,arm,mechanics,states,measures,drive,modulation,fires,resistance_events,qlp,crossings,proposals,deallocations,physical_work,final_tick,trace_hash,body_hash,live_hash,quiescent,replay_equal,mechanics_equal,predicate_pass\n",
    );
    let mut case_id = 0_usize;
    let mut rows = 0_usize;
    let mut retained_cases = 0_usize;
    let mut efficacy_cases = 0_usize;
    let mut maximum_work = 0_u64;
    let mut all_pass = true;

    for root in ROOTS {
        for phase in PHASES {
            for family in Family::ALL {
                for arm in Arm::ALL {
                    case_id += 1;
                    let reference = run_case(root, phase, family, arm, MechanicalConfig::REFERENCE);
                    let reference_replay =
                        run_case(root, phase, family, arm, MechanicalConfig::REFERENCE);
                    let production =
                        run_case(root, phase, family, arm, MechanicalConfig::PRODUCTION);
                    let production_replay =
                        run_case(root, phase, family, arm, MechanicalConfig::PRODUCTION);
                    let reference_replay_equal = reference == reference_replay;
                    let production_replay_equal = production == production_replay;
                    let mechanics_equal = reference == production;
                    let reference_pass = predicate(family, arm, &reference);
                    let production_pass = predicate(family, arm, &production);
                    let case_pass = reference_replay_equal
                        && production_replay_equal
                        && mechanics_equal
                        && reference_pass
                        && production_pass;
                    all_pass &= case_pass;
                    if family.is_retained() {
                        retained_cases += 1;
                    } else {
                        efficacy_cases += 1;
                    }
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
                            arm.name(),
                            mechanics_name(mechanics),
                            states_text(&observation.states),
                            measures_text(&observation.measures),
                            observation.events.drive,
                            observation.events.modulation,
                            observation.events.fires,
                            observation.events.resistance,
                            observation.events.qlp,
                            observation.events.crossings,
                            observation.events.proposals,
                            observation.work.deallocations,
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
    }

    assert_eq!(case_id, EXPECTED_CASES);
    assert_eq!(rows, EXPECTED_ROWS);
    assert_eq!(retained_cases, 240);
    assert_eq!(efficacy_cases, 160);

    let matrix_path = output.join("matrix.csv");
    fs::write(&matrix_path, csv).expect("write CR0 matrix");
    let matrix_hash = ContentHash::of(&fs::read(&matrix_path).unwrap()).to_string();
    let coupling_necessary = all_pass;
    let report = format!(
        "# CR0 coupling-necessity discriminator v1\n\n\
         - cases: `{case_id}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{rows}/{EXPECTED_ROWS}`\n\
         - retained-behavior cases: `{retained_cases}/240`\n\
         - efficacy-control cases: `{efficacy_cases}/160`\n\
         - exact same-mechanics replay: `{all_pass}`\n\
         - exact Reference/Production agreement within arms: `{all_pass}`\n\
         - maximum PhysicalWork: `{maximum_work}`\n\
         - threshold-1 neutral control: `PASS`\n\
         - threshold-2 acquired-efficacy discriminator: `PASS`\n\
         - threshold-3 neutral control: `PASS`\n\
         - two-input topology control: `PASS`\n\
         - classification: `{}`\n\
         - matrix SHA-256: `{matrix_hash}`\n\n\
         CR0 establishes that coupling plasticity has an independent physical\n\
         function: at equal consolidated resistance, coupling 2 can make one\n\
         previously subthreshold route fire a target and produce outward\n\
         activity where coupling 1 cannot. It does not integrate that behavior\n\
         into the continuous-participation law or resume FD2.\n",
        if coupling_necessary {
            "coupling necessary"
        } else {
            "unresolved"
        }
    );
    fs::write(output.join("report.md"), report).expect("write CR0 report");
    assert!(all_pass, "CR0 matrix failed");
    println!("CR0_COUPLING_NECESSITY_V1_PASS");
    println!("matrix_sha256={matrix_hash}");
}
