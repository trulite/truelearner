#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig, ObservedRun,
    PhysicalEvent, PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [5_900_000, 6_000_000];
const PHASES: std::ops::Range<i64> = 0..10;
const OBSERVATION_CEILING: u64 = 256;
const CONTINUATION_CEILING: u64 = 32;
const RESISTANCE: u32 = 1_000_000;
const EXPECTED_CASES: usize = 440;
const EXPECTED_ROWS: usize = 880;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Family {
    BaselineReciprocal,
    MainLocalH16AlternatingPhase,
    Chain8Delay1H16,
    Chain8Delay0H16,
    WrongPathH16,
    UntraversedH16,
    ReciprocalMixedDelayH16,
    ReciprocalDelay1H16,
    ReciprocalDelay2H16,
    ReciprocalDelay3H16,
    Cycle3Delay1H16,
    Cycle4Delay1H16,
    Cycle8Delay1H16,
    ReciprocalC1T1H16,
    ReciprocalC3T3H16,
    SubthresholdC2T3,
    StrengthH1,
    StrengthH2,
    StrengthH3,
    StrengthH4,
    StrengthH8,
    TwoLoopsOneInhibited,
}

impl Family {
    const ALL: [Self; 22] = [
        Self::BaselineReciprocal,
        Self::MainLocalH16AlternatingPhase,
        Self::Chain8Delay1H16,
        Self::Chain8Delay0H16,
        Self::WrongPathH16,
        Self::UntraversedH16,
        Self::ReciprocalMixedDelayH16,
        Self::ReciprocalDelay1H16,
        Self::ReciprocalDelay2H16,
        Self::ReciprocalDelay3H16,
        Self::Cycle3Delay1H16,
        Self::Cycle4Delay1H16,
        Self::Cycle8Delay1H16,
        Self::ReciprocalC1T1H16,
        Self::ReciprocalC3T3H16,
        Self::SubthresholdC2T3,
        Self::StrengthH1,
        Self::StrengthH2,
        Self::StrengthH3,
        Self::StrengthH4,
        Self::StrengthH8,
        Self::TwoLoopsOneInhibited,
    ];

    const STRENGTH_SWEEP: [Self; 5] = [
        Self::StrengthH1,
        Self::StrengthH2,
        Self::StrengthH3,
        Self::StrengthH4,
        Self::StrengthH8,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::BaselineReciprocal => "baseline_reciprocal",
            Self::MainLocalH16AlternatingPhase => "main_local_h16_alternating_phase",
            Self::Chain8Delay1H16 => "chain_8_delay_1_h16",
            Self::Chain8Delay0H16 => "chain_8_delay_0_h16",
            Self::WrongPathH16 => "wrong_path_h16",
            Self::UntraversedH16 => "untraversed_h16",
            Self::ReciprocalMixedDelayH16 => "reciprocal_mixed_delay_h16",
            Self::ReciprocalDelay1H16 => "reciprocal_delay_1_h16",
            Self::ReciprocalDelay2H16 => "reciprocal_delay_2_h16",
            Self::ReciprocalDelay3H16 => "reciprocal_delay_3_h16",
            Self::Cycle3Delay1H16 => "cycle_3_delay_1_h16",
            Self::Cycle4Delay1H16 => "cycle_4_delay_1_h16",
            Self::Cycle8Delay1H16 => "cycle_8_delay_1_h16",
            Self::ReciprocalC1T1H16 => "reciprocal_c1_t1_h16",
            Self::ReciprocalC3T3H16 => "reciprocal_c3_t3_h16",
            Self::SubthresholdC2T3 => "subthreshold_c2_t3",
            Self::StrengthH1 => "strength_h1",
            Self::StrengthH2 => "strength_h2",
            Self::StrengthH3 => "strength_h3",
            Self::StrengthH4 => "strength_h4",
            Self::StrengthH8 => "strength_h8",
            Self::TwoLoopsOneInhibited => "two_loops_one_inhibited",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Topology {
    Cycle,
    Chain,
    TwoCycles,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Feedback {
    None,
    Local {
        strength: i32,
        relay_threshold: i32,
        alternating_phase: bool,
    },
    WrongPath {
        strength: i32,
    },
    FirstCycleOnly {
        strength: i32,
    },
}

#[derive(Clone, Debug)]
struct Geometry {
    topology: Topology,
    cells: usize,
    coupling: i32,
    threshold: i32,
    delays: Vec<i64>,
    feedback: Feedback,
}

impl Geometry {
    fn for_family(family: Family) -> Self {
        match family {
            Family::BaselineReciprocal => Self::cycle(2, 2, 2, vec![1, 1], Feedback::None),
            Family::MainLocalH16AlternatingPhase => Self::cycle(
                2,
                2,
                2,
                vec![1, 1],
                Feedback::Local {
                    strength: 16,
                    relay_threshold: 1,
                    alternating_phase: true,
                },
            ),
            Family::Chain8Delay1H16 => Self::chain(8, 1),
            Family::Chain8Delay0H16 => Self::chain(8, 0),
            Family::WrongPathH16 => {
                Self::cycle(2, 2, 2, vec![1, 1], Feedback::WrongPath { strength: 16 })
            }
            Family::UntraversedH16 => Self::cycle(
                2,
                2,
                2,
                vec![1, 1],
                Feedback::Local {
                    strength: 16,
                    relay_threshold: 2,
                    alternating_phase: false,
                },
            ),
            Family::ReciprocalMixedDelayH16 => Self::local_cycle(2, 2, 2, vec![0, 1], 16),
            Family::ReciprocalDelay1H16 => Self::local_cycle(2, 2, 2, vec![1, 1], 16),
            Family::ReciprocalDelay2H16 => Self::local_cycle(2, 2, 2, vec![2, 2], 16),
            Family::ReciprocalDelay3H16 => Self::local_cycle(2, 2, 2, vec![3, 3], 16),
            Family::Cycle3Delay1H16 => Self::local_cycle(3, 2, 2, vec![1; 3], 16),
            Family::Cycle4Delay1H16 => Self::local_cycle(4, 2, 2, vec![1; 4], 16),
            Family::Cycle8Delay1H16 => Self::local_cycle(8, 2, 2, vec![1; 8], 16),
            Family::ReciprocalC1T1H16 => Self::local_cycle(2, 1, 1, vec![1, 1], 16),
            Family::ReciprocalC3T3H16 => Self::local_cycle(2, 3, 3, vec![1, 1], 16),
            Family::SubthresholdC2T3 => Self::cycle(2, 2, 3, vec![1, 1], Feedback::None),
            Family::StrengthH1 => Self::local_cycle(2, 2, 2, vec![1, 1], 1),
            Family::StrengthH2 => Self::local_cycle(2, 2, 2, vec![1, 1], 2),
            Family::StrengthH3 => Self::local_cycle(2, 2, 2, vec![1, 1], 3),
            Family::StrengthH4 => Self::local_cycle(2, 2, 2, vec![1, 1], 4),
            Family::StrengthH8 => Self::local_cycle(2, 2, 2, vec![1, 1], 8),
            Family::TwoLoopsOneInhibited => Self {
                topology: Topology::TwoCycles,
                cells: 4,
                coupling: 2,
                threshold: 2,
                delays: vec![1; 4],
                feedback: Feedback::FirstCycleOnly { strength: 16 },
            },
        }
    }

    fn cycle(
        cells: usize,
        coupling: i32,
        threshold: i32,
        delays: Vec<i64>,
        feedback: Feedback,
    ) -> Self {
        Self {
            topology: Topology::Cycle,
            cells,
            coupling,
            threshold,
            delays,
            feedback,
        }
    }

    fn local_cycle(
        cells: usize,
        coupling: i32,
        threshold: i32,
        delays: Vec<i64>,
        strength: i32,
    ) -> Self {
        Self::cycle(
            cells,
            coupling,
            threshold,
            delays,
            Feedback::Local {
                strength,
                relay_threshold: 1,
                alternating_phase: false,
            },
        )
    }

    fn chain(cells: usize, delay: i64) -> Self {
        Self {
            topology: Topology::Chain,
            cells,
            coupling: 2,
            threshold: 2,
            delays: vec![delay; cells - 1],
            feedback: Feedback::Local {
                strength: 16,
                relay_threshold: 1,
                alternating_phase: false,
            },
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ActivityClass {
    Dies,
    Periodic,
    PersistentNonperiodic,
    Growing,
}

impl ActivityClass {
    fn name(self) -> &'static str {
        match self {
            Self::Dies => "dies",
            Self::Periodic => "periodic",
            Self::PersistentNonperiodic => "persistent_nonperiodic",
            Self::Growing => "growing",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<PhysicalTransition>,
    continuation_trace: Vec<PhysicalTransition>,
    states: Vec<ArrowState>,
    expected_couplings: Vec<i32>,
    excit_fire_counts: Vec<u64>,
    inhibitor_firings: u64,
    first_firings: u64,
    continuation_firings: u64,
    excitatory_traversals: u64,
    relay_traversals: u64,
    negative_traversals: u64,
    work: WorkTotals,
    scheduled_deliveries: u64,
    continuation_deliveries: u64,
    final_tick: i64,
    body_hash: String,
    live_hash: String,
    naturally_quiescent: bool,
    ceiling_reached: bool,
    continuation_ceiling_reached: bool,
    activity_class: ActivityClass,
    period_firings: u64,
    period_ticks: i64,
    unbounded_match: bool,
}

impl Observation {
    fn physical_eq(&self, other: &Self) -> bool {
        self.trace == other.trace
            && self.continuation_trace == other.continuation_trace
            && self.states == other.states
            && self.expected_couplings == other.expected_couplings
            && self.excit_fire_counts == other.excit_fire_counts
            && self.inhibitor_firings == other.inhibitor_firings
            && self.first_firings == other.first_firings
            && self.continuation_firings == other.continuation_firings
            && self.excitatory_traversals == other.excitatory_traversals
            && self.relay_traversals == other.relay_traversals
            && self.negative_traversals == other.negative_traversals
            && self.work == other.work
            && self.scheduled_deliveries == other.scheduled_deliveries
            && self.continuation_deliveries == other.continuation_deliveries
            && self.final_tick == other.final_tick
            && self.body_hash == other.body_hash
            && self.naturally_quiescent == other.naturally_quiescent
            && self.ceiling_reached == other.ceiling_reached
            && self.continuation_ceiling_reached == other.continuation_ceiling_reached
            && self.activity_class == other.activity_class
            && self.period_firings == other.period_firings
            && self.period_ticks == other.period_ticks
            && self.unbounded_match == other.unbounded_match
    }
}

struct BuiltWorld {
    body: PlasticSubstrate,
    excit_cells: Vec<CellId>,
    inhibitor_cells: Vec<CellId>,
    arrows: Vec<ArrowId>,
    expected_couplings: Vec<i32>,
    initial_inputs: usize,
}

fn add_arrow(
    body: &mut PlasticSubstrate,
    arrows: &mut Vec<ArrowId>,
    expected_couplings: &mut Vec<i32>,
    from: CellId,
    to: CellId,
    coupling: i32,
    delay: i64,
    phase: i32,
) {
    arrows.push(body.add_arrow(ArrowSpec {
        from,
        to,
        delay,
        phase,
        coupling,
        resistance: RESISTANCE,
        mode: TransmissionMode::Drive,
    }));
    expected_couplings.push(coupling);
}

fn build_world(
    root: u64,
    absolute_phase: i64,
    family: Family,
    mechanics: MechanicalConfig,
) -> BuiltWorld {
    let geometry = Geometry::for_family(family);
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 128, 256, mechanics);
    body.set_physical_tracing(true);
    if absolute_phase > 0 {
        body.advance_time(absolute_phase);
    }
    let excit_cells = (0..geometry.cells)
        .map(|index| {
            body.add_cell(CellSpec {
                physical_id: root + u64::try_from(index).unwrap() + 1,
                position: i32::try_from(index * 100).unwrap(),
                region: 0,
                threshold: geometry.threshold,
                resistance: 100_000,
            })
        })
        .collect::<Vec<_>>();
    let mut arrows = Vec::new();
    let mut expected_couplings = Vec::new();
    match geometry.topology {
        Topology::Cycle => {
            for index in 0..geometry.cells {
                add_arrow(
                    &mut body,
                    &mut arrows,
                    &mut expected_couplings,
                    excit_cells[index],
                    excit_cells[(index + 1) % geometry.cells],
                    geometry.coupling,
                    geometry.delays[index],
                    0,
                );
            }
        }
        Topology::Chain => {
            for index in 0..geometry.cells - 1 {
                add_arrow(
                    &mut body,
                    &mut arrows,
                    &mut expected_couplings,
                    excit_cells[index],
                    excit_cells[index + 1],
                    geometry.coupling,
                    geometry.delays[index],
                    0,
                );
            }
        }
        Topology::TwoCycles => {
            for (from, to, delay) in [(0, 1, 1), (1, 0, 1), (2, 3, 1), (3, 2, 1)] {
                add_arrow(
                    &mut body,
                    &mut arrows,
                    &mut expected_couplings,
                    excit_cells[from],
                    excit_cells[to],
                    geometry.coupling,
                    delay,
                    0,
                );
            }
        }
    }

    let feedback_indices = match geometry.feedback {
        Feedback::None => Vec::new(),
        Feedback::FirstCycleOnly { .. } => vec![0, 1],
        _ => (0..geometry.cells).collect::<Vec<_>>(),
    };
    let mut inhibitor_cells = Vec::new();
    for index in feedback_indices {
        let (strength, relay_threshold, alternating_phase, wrong_path) = match geometry.feedback {
            Feedback::Local {
                strength,
                relay_threshold,
                alternating_phase,
            } => (strength, relay_threshold, alternating_phase, false),
            Feedback::WrongPath { strength } => (strength, 1, false, true),
            Feedback::FirstCycleOnly { strength } => (strength, 1, false, false),
            Feedback::None => unreachable!(),
        };
        let inhibitor = body.add_cell(CellSpec {
            physical_id: root + 10_000 + u64::try_from(index).unwrap(),
            position: 10_000 + i32::try_from(index * 100).unwrap(),
            region: 0,
            threshold: relay_threshold,
            resistance: 100_000,
        });
        inhibitor_cells.push(inhibitor);
        let relay_phase = if alternating_phase && index % 2 == 0 {
            1
        } else {
            0
        };
        add_arrow(
            &mut body,
            &mut arrows,
            &mut expected_couplings,
            excit_cells[index],
            inhibitor,
            1,
            0,
            relay_phase,
        );
        let negative_target = if wrong_path {
            body.add_cell(CellSpec {
                physical_id: root + 20_000 + u64::try_from(index).unwrap(),
                position: 20_000 + i32::try_from(index * 100).unwrap(),
                region: 0,
                threshold: 100,
                resistance: 100_000,
            })
        } else {
            excit_cells[index]
        };
        add_arrow(
            &mut body,
            &mut arrows,
            &mut expected_couplings,
            inhibitor,
            negative_target,
            -strength,
            0,
            relay_phase.saturating_add(1),
        );
    }

    let initial_targets = if geometry.topology == Topology::TwoCycles {
        vec![excit_cells[0], excit_cells[2]]
    } else {
        vec![excit_cells[0]]
    };
    for (serial, target) in initial_targets.iter().enumerate() {
        body.enter(SpikeInput {
            arrival_tick: absolute_phase,
            phase: 0,
            origin_physical: root + 900_000 + u64::try_from(serial).unwrap(),
            target: *target,
            impulse: geometry.threshold,
        });
    }
    BuiltWorld {
        body,
        excit_cells,
        inhibitor_cells,
        arrows,
        expected_couplings,
        initial_inputs: initial_targets.len(),
    }
}

fn fire_counts(trace: &[PhysicalTransition], cells: &[CellId]) -> Vec<u64> {
    let mut counts = vec![0_u64; cells.len()];
    for transition in trace {
        if let PhysicalEvent::Fire { cell } = transition.event {
            if let Some(index) = cells.iter().position(|candidate| *candidate == cell) {
                counts[index] = counts[index].saturating_add(1);
            }
        }
    }
    counts
}

fn firing_sequence(trace: &[PhysicalTransition], cells: &[CellId]) -> Vec<(usize, i64)> {
    trace
        .iter()
        .filter_map(|transition| {
            let PhysicalEvent::Fire { cell } = transition.event else {
                return None;
            };
            let index = cells.iter().position(|candidate| *candidate == cell)?;
            Some((index, transition.tick))
        })
        .collect()
}

fn periodicity(
    first: &[PhysicalTransition],
    continuation: &[PhysicalTransition],
    cells: &[CellId],
) -> Option<(u64, i64)> {
    let mut sequence = firing_sequence(first, cells);
    sequence.extend(firing_sequence(continuation, cells));
    for period in 1..=8_usize {
        if sequence.len() < period * 3 {
            continue;
        }
        let tail = &sequence[sequence.len() - period * 3..];
        let a = &tail[..period];
        let b = &tail[period..period * 2];
        let c = &tail[period * 2..];
        let same_cells = a
            .iter()
            .zip(b)
            .zip(c)
            .all(|((left, middle), right)| left.0 == middle.0 && middle.0 == right.0);
        let ab = b[0].1.saturating_sub(a[0].1);
        let bc = c[0].1.saturating_sub(b[0].1);
        if same_cells && ab > 0 && ab == bc {
            return Some((u64::try_from(period).unwrap(), ab));
        }
    }
    None
}

fn classify(
    first: &ObservedRun,
    continuation: Option<&ObservedRun>,
    first_trace: &[PhysicalTransition],
    continuation_trace: &[PhysicalTransition],
    cells: &[CellId],
) -> (ActivityClass, u64, i64) {
    if first.run.naturally_quiescent
        || continuation.is_some_and(|next| next.run.naturally_quiescent)
    {
        return (ActivityClass::Dies, 0, 0);
    }
    if let Some((firings, ticks)) = periodicity(first_trace, continuation_trace, cells) {
        return (ActivityClass::Periodic, firings, ticks);
    }
    let first_fires = firing_sequence(first_trace, cells).len();
    let continuation_fires = firing_sequence(continuation_trace, cells).len();
    let growing = continuation.is_some_and(|next| {
        next.observation_ceiling_reached
            && continuation_fires.saturating_mul(usize::try_from(OBSERVATION_CEILING).unwrap())
                > first_fires.saturating_mul(usize::try_from(CONTINUATION_CEILING).unwrap())
    });
    if growing {
        (ActivityClass::Growing, 0, 0)
    } else {
        (ActivityClass::PersistentNonperiodic, 0, 0)
    }
}

fn count_signed_deliveries(
    trace: &[PhysicalTransition],
    inhibitor_cells: &[CellId],
    initial_inputs: usize,
) -> (u64, u64, u64) {
    let mut positive = 0_u64;
    let mut relay = 0_u64;
    let mut negative = 0_u64;
    for transition in trace {
        let PhysicalEvent::Deliver {
            mode: TransmissionMode::Drive,
            target,
            impulse,
        } = transition.event
        else {
            continue;
        };
        if impulse < 0 {
            negative = negative.saturating_add(1);
        } else if inhibitor_cells.contains(&target) {
            relay = relay.saturating_add(1);
        } else {
            positive = positive.saturating_add(1);
        }
    }
    (
        positive.saturating_sub(u64::try_from(initial_inputs).unwrap()),
        relay,
        negative,
    )
}

fn run_case(
    root: u64,
    absolute_phase: i64,
    family: Family,
    mechanics: MechanicalConfig,
) -> Observation {
    let family_root = root.saturating_add(
        u64::try_from(Family::ALL.iter().position(|item| *item == family).unwrap()).unwrap()
            * 100_000,
    );
    let mut world = build_world(family_root, absolute_phase, family, mechanics);
    let first = world
        .body
        .propagate_with_observation_ceiling(OBSERVATION_CEILING);
    let first_trace = first.run.physical_trace.clone();
    let first_firings =
        u64::try_from(firing_sequence(&first_trace, &world.excit_cells).len()).unwrap();
    let mut work = WorkTotals::default();
    work.add(first.run.work);
    let continuation = if first.observation_ceiling_reached {
        let next = world
            .body
            .propagate_with_observation_ceiling(CONTINUATION_CEILING);
        work.add(next.run.work);
        Some(next)
    } else {
        None
    };
    let continuation_trace = continuation
        .as_ref()
        .map_or_else(Vec::new, |next| next.run.physical_trace.clone());
    let continuation_firings =
        u64::try_from(firing_sequence(&continuation_trace, &world.excit_cells).len()).unwrap();
    let mut combined_trace = first_trace.clone();
    combined_trace.extend(continuation_trace.clone());
    let excit_fire_counts = fire_counts(&combined_trace, &world.excit_cells);
    let inhibitor_firings = fire_counts(&combined_trace, &world.inhibitor_cells)
        .into_iter()
        .sum();
    let (excitatory_traversals, relay_traversals, negative_traversals) = count_signed_deliveries(
        &combined_trace,
        &world.inhibitor_cells,
        world.initial_inputs,
    );
    let (activity_class, period_firings, period_ticks) = classify(
        &first,
        continuation.as_ref(),
        &first_trace,
        &continuation_trace,
        &world.excit_cells,
    );
    let states = world
        .arrows
        .iter()
        .map(|id| {
            let arrow = world
                .body
                .arena_body(1)
                .arrows
                .into_iter()
                .find(|candidate| candidate.id == *id)
                .expect("RS1 ARROW must remain addressable");
            ArrowState {
                live: arrow.live,
                resistance: arrow.resistance,
                coupling: arrow.coupling,
            }
        })
        .collect::<Vec<_>>();
    let body_hash = ContentHash::of(&world.body.canonical_body_bytes(1).unwrap()).to_string();
    let live_hash = ContentHash::of(
        &world
            .body
            .live_checkpoint(1)
            .expect("RS1 live checkpoint")
            .canonical_bytes()
            .expect("RS1 canonical live checkpoint"),
    )
    .to_string();
    let naturally_quiescent = continuation
        .as_ref()
        .map_or(first.run.naturally_quiescent, |next| {
            next.run.naturally_quiescent
        });
    let continuation_deliveries = continuation
        .as_ref()
        .map_or(0, |next| next.scheduled_deliveries);
    let continuation_ceiling_reached = continuation
        .as_ref()
        .is_some_and(|next| next.observation_ceiling_reached);

    let unbounded_match = if first.run.naturally_quiescent {
        let mut ordinary = build_world(family_root, absolute_phase, family, mechanics);
        let ordinary_run = ordinary.body.propagate();
        ordinary_run.physical_trace == first_trace
            && ordinary_run.work == first.run.work
            && ordinary_run.naturally_quiescent
            && ordinary.body.clock() == world.body.clock()
            && ordinary.body.canonical_body_bytes(1).unwrap()
                == world.body.canonical_body_bytes(1).unwrap()
    } else {
        true
    };

    Observation {
        trace: first_trace,
        continuation_trace,
        states,
        expected_couplings: world.expected_couplings,
        excit_fire_counts,
        inhibitor_firings,
        first_firings,
        continuation_firings,
        excitatory_traversals,
        relay_traversals,
        negative_traversals,
        work,
        scheduled_deliveries: first.scheduled_deliveries,
        continuation_deliveries,
        final_tick: world.body.clock().tick,
        body_hash,
        live_hash,
        naturally_quiescent,
        ceiling_reached: first.observation_ceiling_reached,
        continuation_ceiling_reached,
        activity_class,
        period_firings,
        period_ticks,
        unbounded_match,
    }
}

fn settled_cycle(observation: &Observation, cells: usize) -> bool {
    observation.activity_class == ActivityClass::Dies
        && observation.excit_fire_counts == vec![1; cells]
        && observation.negative_traversals == u64::try_from(cells).unwrap()
}

fn predicate(family: Family, observation: &Observation) -> bool {
    let static_body = observation.states.len() == observation.expected_couplings.len()
        && observation
            .states
            .iter()
            .zip(&observation.expected_couplings)
            .all(|(state, expected)| {
                state.live
                    && state.coupling == *expected
                    && state.resistance > 999_000
                    && state.resistance <= RESISTANCE
            });
    let bounded_state = (observation.naturally_quiescent
        && ((!observation.ceiling_reached && observation.continuation_deliveries == 0)
            || (observation.ceiling_reached
                && !observation.continuation_ceiling_reached
                && observation.continuation_deliveries > 0)))
        || (!observation.naturally_quiescent
            && observation.ceiling_reached
            && observation.continuation_ceiling_reached
            && observation.scheduled_deliveries == OBSERVATION_CEILING
            && observation.continuation_deliveries == CONTINUATION_CEILING);
    let base = static_body
        && bounded_state
        && observation.work.modulation == 0
        && observation.work.updates == 0
        && observation.work.proposals == 0
        && observation.work.deallocations == 0
        && observation.work.qlp == 0
        && observation.unbounded_match;
    if !base {
        return false;
    }
    match family {
        Family::BaselineReciprocal => {
            observation.activity_class == ActivityClass::Periodic
                && observation.negative_traversals == 0
        }
        Family::MainLocalH16AlternatingPhase
        | Family::ReciprocalMixedDelayH16
        | Family::ReciprocalDelay1H16
        | Family::ReciprocalDelay2H16
        | Family::ReciprocalDelay3H16
        | Family::ReciprocalC1T1H16
        | Family::ReciprocalC3T3H16 => settled_cycle(observation, 2),
        Family::Cycle3Delay1H16 => settled_cycle(observation, 3),
        Family::Cycle4Delay1H16 => settled_cycle(observation, 4),
        Family::Cycle8Delay1H16 => settled_cycle(observation, 8),
        Family::Chain8Delay1H16 | Family::Chain8Delay0H16 => {
            observation.activity_class == ActivityClass::Dies
                && observation.excit_fire_counts == vec![1; 8]
                && observation.negative_traversals == 8
        }
        Family::WrongPathH16 => {
            observation.activity_class == ActivityClass::Periodic
                && observation.negative_traversals > 0
        }
        Family::UntraversedH16 => {
            observation.activity_class == ActivityClass::Periodic
                && observation.relay_traversals > 0
                && observation.negative_traversals == 0
        }
        Family::SubthresholdC2T3 => {
            observation.activity_class == ActivityClass::Dies
                && observation.excit_fire_counts == [1, 0]
                && observation.negative_traversals == 0
        }
        Family::StrengthH1
        | Family::StrengthH2
        | Family::StrengthH3
        | Family::StrengthH4
        | Family::StrengthH8 => true,
        Family::TwoLoopsOneInhibited => {
            observation.activity_class == ActivityClass::Periodic
                && observation.excit_fire_counts[0] == 1
                && observation.excit_fire_counts[1] == 1
                && observation.excit_fire_counts[2] > 50
                && observation.excit_fire_counts[3] > 50
                && observation.negative_traversals == 2
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
                "{}/{}/{}",
                u8::from(state.live),
                state.resistance,
                state.coupling
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn counts_text(counts: &[u64]) -> String {
    counts
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/rs1_inhibitory_topology_v1"));
    fs::create_dir_all(&output).expect("create RS1 output directory");
    let mut csv = String::from(
        "case_id,root,absolute_phase,family,mechanics,class,period_firings,period_ticks,first_firings,continuation_firings,excitatory_fire_counts,inhibitor_firings,excitatory_traversals,relay_traversals,negative_traversals,scheduled_deliveries,continuation_deliveries,physical_work,final_tick,states,naturally_quiescent,ceiling_reached,continuation_ceiling_reached,unbounded_match,trace_hash,continuation_trace_hash,body_hash,live_hash,replay_equal,mechanics_equal,predicate_pass,case_pass\n",
    );
    let mut case_id = 0_usize;
    let mut rows = 0_usize;
    let mut maximum_work = 0_u64;
    let mut all_replay = true;
    let mut all_mechanics = true;
    let mut all_predicates = true;
    let mut all_pass = true;
    let mut classifications = BTreeMap::<Family, BTreeSet<ActivityClass>>::new();

    for root in ROOTS {
        for absolute_phase in PHASES {
            for family in Family::ALL {
                case_id += 1;
                let reference = run_case(root, absolute_phase, family, MechanicalConfig::REFERENCE);
                let reference_replay =
                    run_case(root, absolute_phase, family, MechanicalConfig::REFERENCE);
                let production =
                    run_case(root, absolute_phase, family, MechanicalConfig::PRODUCTION);
                let production_replay =
                    run_case(root, absolute_phase, family, MechanicalConfig::PRODUCTION);
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
                all_replay &= reference_replay_equal && production_replay_equal;
                all_mechanics &= mechanics_equal;
                all_predicates &= reference_pass && production_pass;
                all_pass &= case_pass;
                classifications
                    .entry(family)
                    .or_default()
                    .insert(reference.activity_class);

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
                        "{case_id},{root},{absolute_phase},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        family.name(),
                        mechanics_name(mechanics),
                        observation.activity_class.name(),
                        observation.period_firings,
                        observation.period_ticks,
                        observation.first_firings,
                        observation.continuation_firings,
                        counts_text(&observation.excit_fire_counts),
                        observation.inhibitor_firings,
                        observation.excitatory_traversals,
                        observation.relay_traversals,
                        observation.negative_traversals,
                        observation.scheduled_deliveries,
                        observation.continuation_deliveries,
                        observation.work.physical,
                        observation.final_tick,
                        states_text(&observation.states),
                        observation.naturally_quiescent,
                        observation.ceiling_reached,
                        observation.continuation_ceiling_reached,
                        observation.unbounded_match,
                        trace_hash(&observation.trace),
                        trace_hash(&observation.continuation_trace),
                        observation.body_hash,
                        observation.live_hash,
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

    let phase_invariant = classifications.values().all(|classes| classes.len() == 1);
    let strength_classes = Family::STRENGTH_SWEEP
        .iter()
        .map(|family| {
            (
                family.name(),
                classifications
                    .get(family)
                    .expect("RS1 strength family must be classified")
                    .iter()
                    .map(|class| class.name())
                    .collect::<Vec<_>>()
                    .join("|"),
            )
        })
        .collect::<Vec<_>>();
    all_pass &= phase_invariant;

    assert_eq!(case_id, EXPECTED_CASES);
    assert_eq!(rows, EXPECTED_ROWS);
    let matrix_path = output.join("matrix.csv");
    fs::write(&matrix_path, csv).expect("write RS1 matrix");
    let matrix_hash = ContentHash::of(&fs::read(&matrix_path).unwrap()).to_string();
    let classification = if all_pass {
        "RS1 positive — ordinary inhibitory topology is sufficient"
    } else {
        "RS1 negative — frozen topology sufficiency gate failed"
    };
    let mut family_summary = String::new();
    for family in Family::ALL {
        let classes = classifications
            .get(&family)
            .expect("every RS1 family must have a classification");
        writeln!(
            family_summary,
            "- `{}`: `{}`",
            family.name(),
            classes
                .iter()
                .map(|class| class.name())
                .collect::<Vec<_>>()
                .join("|")
        )
        .unwrap();
    }
    let strength_summary = strength_classes
        .iter()
        .map(|(family, class)| format!("- `{family}`: `{class}`"))
        .collect::<Vec<_>>()
        .join("\n");
    let report = format!(
        "# RS1 inhibitory topology sufficiency v1\n\n\
         - cases: `{case_id}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{rows}/{EXPECTED_ROWS}`\n\
         - exact same-mechanics replay: `{all_replay}`\n\
         - exact Reference/Production physical agreement: `{all_mechanics}`\n\
         - all frozen predicates: `{all_predicates}`\n\
         - absolute-phase invariant classification: `{phase_invariant}`\n\
         - maximum PhysicalWork: `{maximum_work}`\n\
         - classification: `{classification}`\n\
         - matrix SHA-256: `{matrix_hash}`\n\n\
         ## Strength sweep\n\n{strength_summary}\n\n\
         ## Family classes\n\n{family_summary}\n"
    );
    fs::write(output.join("report.md"), report).expect("write RS1 report");
    assert!(all_pass, "RS1 inhibitory topology gate failed");
    println!("RS1_INHIBITORY_TOPOLOGY_SUFFICIENCY_V1_PASS");
    println!("classification={classification}");
    println!("matrix_sha256={matrix_hash}");
}
