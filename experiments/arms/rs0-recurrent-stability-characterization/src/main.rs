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

const ROOTS: [u64; 2] = [5_700_000, 5_800_000];
const PHASES: std::ops::Range<i64> = 0..10;
const OBSERVATION_CEILING: u64 = 256;
const CONTINUATION_CEILING: u64 = 32;
const RESISTANCE: u32 = 1_000_000;
const EXPECTED_CASES: usize = 400;
const EXPECTED_ROWS: usize = 800;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Family {
    OneWayC2D1T2,
    ReciprocalC1D1T2,
    ReciprocalC1D1T1,
    ReciprocalC2D0T2,
    ReciprocalC2D0T2AlternatingPhase,
    ReciprocalC2D1T2,
    ReciprocalC2D2T2,
    ReciprocalC2D3T2,
    ReciprocalC2MixedD0D1T2,
    ReciprocalC2D1T2AlternatingPhase,
    ReciprocalC2D1T1,
    ReciprocalC2D1T3,
    Cycle3C2D1T2,
    Cycle4C2D1T2,
    Cycle8C2D1T2,
    Cycle3C2D0T2,
    Cycle4C2D0T2,
    Cycle8C2D0T2,
    Chain8C2D1T2,
    Chain8C2D0T2,
}

impl Family {
    const ALL: [Self; 20] = [
        Self::OneWayC2D1T2,
        Self::ReciprocalC1D1T2,
        Self::ReciprocalC1D1T1,
        Self::ReciprocalC2D0T2,
        Self::ReciprocalC2D0T2AlternatingPhase,
        Self::ReciprocalC2D1T2,
        Self::ReciprocalC2D2T2,
        Self::ReciprocalC2D3T2,
        Self::ReciprocalC2MixedD0D1T2,
        Self::ReciprocalC2D1T2AlternatingPhase,
        Self::ReciprocalC2D1T1,
        Self::ReciprocalC2D1T3,
        Self::Cycle3C2D1T2,
        Self::Cycle4C2D1T2,
        Self::Cycle8C2D1T2,
        Self::Cycle3C2D0T2,
        Self::Cycle4C2D0T2,
        Self::Cycle8C2D0T2,
        Self::Chain8C2D1T2,
        Self::Chain8C2D0T2,
    ];

    const PERSISTENCE_DISCRIMINATORS: [Self; 10] = [
        Self::ReciprocalC1D1T1,
        Self::ReciprocalC2D1T2,
        Self::ReciprocalC2D2T2,
        Self::ReciprocalC2D3T2,
        Self::ReciprocalC2MixedD0D1T2,
        Self::ReciprocalC2D1T2AlternatingPhase,
        Self::ReciprocalC2D1T1,
        Self::Cycle3C2D1T2,
        Self::Cycle4C2D1T2,
        Self::Cycle8C2D1T2,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::OneWayC2D1T2 => "one_way_c2_d1_t2",
            Self::ReciprocalC1D1T2 => "reciprocal_c1_d1_t2",
            Self::ReciprocalC1D1T1 => "reciprocal_c1_d1_t1",
            Self::ReciprocalC2D0T2 => "reciprocal_c2_d0_t2",
            Self::ReciprocalC2D0T2AlternatingPhase => {
                "reciprocal_c2_d0_t2_alternating_phase"
            }
            Self::ReciprocalC2D1T2 => "reciprocal_c2_d1_t2",
            Self::ReciprocalC2D2T2 => "reciprocal_c2_d2_t2",
            Self::ReciprocalC2D3T2 => "reciprocal_c2_d3_t2",
            Self::ReciprocalC2MixedD0D1T2 => "reciprocal_c2_mixed_d0_d1_t2",
            Self::ReciprocalC2D1T2AlternatingPhase => {
                "reciprocal_c2_d1_t2_alternating_phase"
            }
            Self::ReciprocalC2D1T1 => "reciprocal_c2_d1_t1",
            Self::ReciprocalC2D1T3 => "reciprocal_c2_d1_t3",
            Self::Cycle3C2D1T2 => "cycle_3_c2_d1_t2",
            Self::Cycle4C2D1T2 => "cycle_4_c2_d1_t2",
            Self::Cycle8C2D1T2 => "cycle_8_c2_d1_t2",
            Self::Cycle3C2D0T2 => "cycle_3_c2_d0_t2",
            Self::Cycle4C2D0T2 => "cycle_4_c2_d0_t2",
            Self::Cycle8C2D0T2 => "cycle_8_c2_d0_t2",
            Self::Chain8C2D1T2 => "chain_8_c2_d1_t2",
            Self::Chain8C2D0T2 => "chain_8_c2_d0_t2",
        }
    }

    fn cycle_len(self) -> Option<usize> {
        match self {
            Self::ReciprocalC1D1T2
            | Self::ReciprocalC1D1T1
            | Self::ReciprocalC2D0T2
            | Self::ReciprocalC2D0T2AlternatingPhase
            | Self::ReciprocalC2D1T2
            | Self::ReciprocalC2D2T2
            | Self::ReciprocalC2D3T2
            | Self::ReciprocalC2MixedD0D1T2
            | Self::ReciprocalC2D1T2AlternatingPhase
            | Self::ReciprocalC2D1T1
            | Self::ReciprocalC2D1T3 => Some(2),
            Self::Cycle3C2D1T2 | Self::Cycle3C2D0T2 => Some(3),
            Self::Cycle4C2D1T2 | Self::Cycle4C2D0T2 => Some(4),
            Self::Cycle8C2D1T2 | Self::Cycle8C2D0T2 => Some(8),
            Self::OneWayC2D1T2 | Self::Chain8C2D1T2 | Self::Chain8C2D0T2 => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Topology {
    OneWay,
    Cycle,
    Chain,
}

#[derive(Clone, Debug)]
struct Geometry {
    topology: Topology,
    cells: usize,
    coupling: i32,
    threshold: i32,
    delays: Vec<i64>,
    phases: Vec<i32>,
}

impl Geometry {
    fn for_family(family: Family) -> Self {
        match family {
            Family::OneWayC2D1T2 => Self::one_way(2, 2, 1, 0),
            Family::ReciprocalC1D1T2 => Self::cycle(2, 1, 2, vec![1, 1], vec![0, 0]),
            Family::ReciprocalC1D1T1 => Self::cycle(2, 1, 1, vec![1, 1], vec![0, 0]),
            Family::ReciprocalC2D0T2 => Self::cycle(2, 2, 2, vec![0, 0], vec![0, 0]),
            Family::ReciprocalC2D0T2AlternatingPhase => {
                Self::cycle(2, 2, 2, vec![0, 0], vec![0, 1])
            }
            Family::ReciprocalC2D1T2 => Self::cycle(2, 2, 2, vec![1, 1], vec![0, 0]),
            Family::ReciprocalC2D2T2 => Self::cycle(2, 2, 2, vec![2, 2], vec![0, 0]),
            Family::ReciprocalC2D3T2 => Self::cycle(2, 2, 2, vec![3, 3], vec![0, 0]),
            Family::ReciprocalC2MixedD0D1T2 => {
                Self::cycle(2, 2, 2, vec![0, 1], vec![0, 0])
            }
            Family::ReciprocalC2D1T2AlternatingPhase => {
                Self::cycle(2, 2, 2, vec![1, 1], vec![0, 1])
            }
            Family::ReciprocalC2D1T1 => Self::cycle(2, 2, 1, vec![1, 1], vec![0, 0]),
            Family::ReciprocalC2D1T3 => Self::cycle(2, 2, 3, vec![1, 1], vec![0, 0]),
            Family::Cycle3C2D1T2 => Self::uniform_cycle(3, 1),
            Family::Cycle4C2D1T2 => Self::uniform_cycle(4, 1),
            Family::Cycle8C2D1T2 => Self::uniform_cycle(8, 1),
            Family::Cycle3C2D0T2 => Self::uniform_cycle(3, 0),
            Family::Cycle4C2D0T2 => Self::uniform_cycle(4, 0),
            Family::Cycle8C2D0T2 => Self::uniform_cycle(8, 0),
            Family::Chain8C2D1T2 => Self::chain(8, 1),
            Family::Chain8C2D0T2 => Self::chain(8, 0),
        }
    }

    fn one_way(coupling: i32, threshold: i32, delay: i64, phase: i32) -> Self {
        Self {
            topology: Topology::OneWay,
            cells: 2,
            coupling,
            threshold,
            delays: vec![delay],
            phases: vec![phase],
        }
    }

    fn cycle(
        cells: usize,
        coupling: i32,
        threshold: i32,
        delays: Vec<i64>,
        phases: Vec<i32>,
    ) -> Self {
        Self {
            topology: Topology::Cycle,
            cells,
            coupling,
            threshold,
            delays,
            phases,
        }
    }

    fn uniform_cycle(cells: usize, delay: i64) -> Self {
        Self::cycle(cells, 2, 2, vec![delay; cells], vec![0; cells])
    }

    fn chain(cells: usize, delay: i64) -> Self {
        Self {
            topology: Topology::Chain,
            cells,
            coupling: 2,
            threshold: 2,
            delays: vec![delay; cells - 1],
            phases: vec![0; cells - 1],
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
    fire_counts: Vec<u64>,
    first_firings: u64,
    continuation_firings: u64,
    traversals: u64,
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
            && self.fire_counts == other.fire_counts
            && self.first_firings == other.first_firings
            && self.continuation_firings == other.continuation_firings
            && self.traversals == other.traversals
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
    cells: Vec<CellId>,
    arrows: Vec<ArrowId>,
}

fn build_world(
    root: u64,
    absolute_phase: i64,
    family: Family,
    mechanics: MechanicalConfig,
) -> BuiltWorld {
    let geometry = Geometry::for_family(family);
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 64, 64, mechanics);
    body.set_physical_tracing(true);
    if absolute_phase > 0 {
        body.advance_time(absolute_phase);
    }
    let cells = (0..geometry.cells)
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
    let arrow_count = match geometry.topology {
        Topology::OneWay | Topology::Chain => geometry.cells - 1,
        Topology::Cycle => geometry.cells,
    };
    let arrows = (0..arrow_count)
        .map(|index| {
            let to_index = if geometry.topology == Topology::Cycle {
                (index + 1) % geometry.cells
            } else {
                index + 1
            };
            body.add_arrow(ArrowSpec {
                from: cells[index],
                to: cells[to_index],
                delay: geometry.delays[index],
                phase: geometry.phases[index],
                coupling: geometry.coupling,
                resistance: RESISTANCE,
                mode: TransmissionMode::Drive,
            })
        })
        .collect::<Vec<_>>();
    body.enter(SpikeInput {
        arrival_tick: absolute_phase,
        phase: 0,
        origin_physical: root + 900_000,
        target: cells[0],
        impulse: geometry.threshold,
    });
    BuiltWorld {
        body,
        cells,
        arrows,
    }
}

fn count_fires(trace: &[PhysicalTransition], cells: &[CellId]) -> Vec<u64> {
    let mut counts = vec![0_u64; cells.len()];
    for transition in trace {
        if let PhysicalEvent::Fire { cell } = transition.event {
            let index = cells
                .iter()
                .position(|candidate| *candidate == cell)
                .expect("RS0 firing must belong to the frozen geometry");
            counts[index] = counts[index].saturating_add(1);
        }
    }
    counts
}

fn fire_sequence(trace: &[PhysicalTransition], cells: &[CellId]) -> Vec<(usize, i64)> {
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
    cycle_len: Option<usize>,
) -> Option<(u64, i64)> {
    let cycle_len = cycle_len?;
    let mut sequence = fire_sequence(first, cells);
    sequence.extend(fire_sequence(continuation, cells));
    if sequence.len() < cycle_len.saturating_mul(3) {
        return None;
    }
    let tail = &sequence[sequence.len() - cycle_len * 3..];
    let a = &tail[..cycle_len];
    let b = &tail[cycle_len..cycle_len * 2];
    let c = &tail[cycle_len * 2..];
    let same_cells = a
        .iter()
        .zip(b)
        .zip(c)
        .all(|((left, middle), right)| left.0 == middle.0 && middle.0 == right.0);
    let period_ab = b[0].1.saturating_sub(a[0].1);
    let period_bc = c[0].1.saturating_sub(b[0].1);
    if same_cells && period_ab > 0 && period_ab == period_bc {
        Some((u64::try_from(cycle_len).unwrap(), period_ab))
    } else {
        None
    }
}

fn classify(
    first: &ObservedRun,
    continuation: Option<&ObservedRun>,
    first_trace: &[PhysicalTransition],
    continuation_trace: &[PhysicalTransition],
    cells: &[CellId],
    cycle_len: Option<usize>,
) -> (ActivityClass, u64, i64) {
    if first.run.naturally_quiescent {
        return (ActivityClass::Dies, 0, 0);
    }
    if continuation.is_some_and(|next| next.run.naturally_quiescent) {
        return (ActivityClass::Dies, 0, 0);
    }
    if let Some((period_firings, period_ticks)) =
        periodicity(first_trace, continuation_trace, cells, cycle_len)
    {
        return (ActivityClass::Periodic, period_firings, period_ticks);
    }
    let first_fires = fire_sequence(first_trace, cells).len();
    let continuation_fires = fire_sequence(continuation_trace, cells).len();
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

fn run_case(
    root: u64,
    absolute_phase: i64,
    family: Family,
    mechanics: MechanicalConfig,
) -> Observation {
    let family_root = root.saturating_add(
        u64::try_from(Family::ALL.iter().position(|item| *item == family).unwrap()).unwrap()
            * 10_000,
    );
    let mut world = build_world(family_root, absolute_phase, family, mechanics);
    let first = world
        .body
        .propagate_with_observation_ceiling(OBSERVATION_CEILING);
    let first_trace = first.run.physical_trace.clone();
    let first_firings = u64::try_from(fire_sequence(&first_trace, &world.cells).len()).unwrap();
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
        u64::try_from(fire_sequence(&continuation_trace, &world.cells).len()).unwrap();
    let mut combined_trace = first_trace.clone();
    combined_trace.extend(continuation_trace.clone());
    let fire_counts = count_fires(&combined_trace, &world.cells);
    let drive_deliveries = combined_trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::Deliver {
                    mode: TransmissionMode::Drive,
                    ..
                }
            )
        })
        .count();
    let traversals = u64::try_from(drive_deliveries.saturating_sub(1)).unwrap();
    let (activity_class, period_firings, period_ticks) = classify(
        &first,
        continuation.as_ref(),
        &first_trace,
        &continuation_trace,
        &world.cells,
        family.cycle_len(),
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
                .expect("RS0 ARROW must remain addressable");
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
            .expect("RS0 live checkpoint")
            .canonical_bytes()
            .expect("RS0 canonical live checkpoint"),
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
        fire_counts,
        first_firings,
        continuation_firings,
        traversals,
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

fn predicate(family: Family, observation: &Observation) -> bool {
    let geometry = Geometry::for_family(family);
    let static_body = observation.states.iter().all(|state| {
        state.live
            && state.coupling == geometry.coupling
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
        Family::OneWayC2D1T2 => {
            observation.activity_class == ActivityClass::Dies
                && observation.first_firings == 2
                && observation.traversals == 1
        }
        Family::ReciprocalC1D1T2 => {
            observation.activity_class == ActivityClass::Dies
                && observation.first_firings == 1
                && observation.traversals == 1
        }
        Family::Chain8C2D1T2 | Family::Chain8C2D0T2 => {
            observation.activity_class == ActivityClass::Dies
                && observation.first_firings == 8
                && observation.traversals == 7
        }
        _ => true,
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
        .unwrap_or_else(|| PathBuf::from("results/rs0_recurrent_stability_v1"));
    fs::create_dir_all(&output).expect("create RS0 output directory");
    let mut csv = String::from(
        "case_id,root,absolute_phase,family,mechanics,class,period_firings,period_ticks,first_firings,continuation_firings,fire_counts,traversals,scheduled_deliveries,continuation_deliveries,physical_work,final_tick,states,naturally_quiescent,ceiling_reached,continuation_ceiling_reached,unbounded_match,trace_hash,continuation_trace_hash,body_hash,live_hash,replay_equal,mechanics_equal,predicate_pass,case_pass\n",
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
                        "{case_id},{root},{absolute_phase},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        family.name(),
                        mechanics_name(mechanics),
                        observation.activity_class.name(),
                        observation.period_firings,
                        observation.period_ticks,
                        observation.first_firings,
                        observation.continuation_firings,
                        counts_text(&observation.fire_counts),
                        observation.traversals,
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
    let persistent_discriminators = Family::PERSISTENCE_DISCRIMINATORS
        .iter()
        .filter(|family| {
            classifications
                .get(family)
                .is_some_and(|classes| classes == &BTreeSet::from([ActivityClass::Periodic]))
        })
        .count();
    let classification = if persistent_discriminators == Family::PERSISTENCE_DISCRIMINATORS.len()
    {
        "B — strong recurrence generally persistent"
    } else if persistent_discriminators <= 2 {
        "A — broad existing stable regime"
    } else {
        "C — timing-sensitive or razor-thin stability"
    };
    all_pass &= phase_invariant;

    assert_eq!(case_id, EXPECTED_CASES);
    assert_eq!(rows, EXPECTED_ROWS);
    let matrix_path = output.join("matrix.csv");
    fs::write(&matrix_path, csv).expect("write RS0 matrix");
    let matrix_hash = ContentHash::of(&fs::read(&matrix_path).unwrap()).to_string();
    let mut family_summary = String::new();
    for family in Family::ALL {
        let classes = classifications
            .get(&family)
            .expect("every RS0 family must have a classification");
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
    let report = format!(
        "# RS0 recurrent stability characterization v1\n\n\
         - cases: `{case_id}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{rows}/{EXPECTED_ROWS}`\n\
         - exact same-mechanics replay: `{all_replay}`\n\
         - exact Reference/Production physical agreement: `{all_mechanics}`\n\
         - all frozen predicates: `{all_predicates}`\n\
         - absolute-phase invariant classification: `{phase_invariant}`\n\
         - persistent discriminators: `{persistent_discriminators}/{}`\n\
         - maximum PhysicalWork: `{maximum_work}`\n\
         - classification: `{classification}`\n\
         - matrix SHA-256: `{matrix_hash}`\n\n\
         ## Family classes\n\n{family_summary}\n",
        Family::PERSISTENCE_DISCRIMINATORS.len()
    );
    fs::write(output.join("report.md"), report).expect("write RS0 report");
    assert!(all_pass, "RS0 characterization gate failed");
    println!("RS0_RECURRENT_STABILITY_CHARACTERIZATION_V1_PASS");
    println!("classification={classification}");
    println!("matrix_sha256={matrix_hash}");
}
