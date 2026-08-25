#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    SameJunction,
    ThresholdPlusOnePlusOne,
    ThresholdPlusTwoMinusOne,
    ThresholdPlusTwoPlusOne,
    PreexistingState,
    DifferentJunctions,
    ParallelArrows,
    ZeroDelayChain,
    ZeroDelayFanoutMerge,
    ZeroDelayCycle,
}

impl Family {
    const ALL: [Self; 10] = [
        Self::SameJunction,
        Self::ThresholdPlusOnePlusOne,
        Self::ThresholdPlusTwoMinusOne,
        Self::ThresholdPlusTwoPlusOne,
        Self::PreexistingState,
        Self::DifferentJunctions,
        Self::ParallelArrows,
        Self::ZeroDelayChain,
        Self::ZeroDelayFanoutMerge,
        Self::ZeroDelayCycle,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::SameJunction => "same_junction_plus2_minus1",
            Self::ThresholdPlusOnePlusOne => "threshold_plus1_plus1",
            Self::ThresholdPlusTwoMinusOne => "threshold_plus2_minus1",
            Self::ThresholdPlusTwoPlusOne => "threshold_plus2_plus1",
            Self::PreexistingState => "preexisting_state",
            Self::DifferentJunctions => "different_junctions",
            Self::ParallelArrows => "parallel_arrows",
            Self::ZeroDelayChain => "zero_delay_chain",
            Self::ZeroDelayFanoutMerge => "zero_delay_fanout_merge",
            Self::ZeroDelayCycle => "zero_delay_cycle",
        }
    }

    fn expected_fires(self) -> &'static [(&'static str, usize)] {
        match self {
            Self::SameJunction | Self::ThresholdPlusTwoMinusOne => &[("target", 0)],
            Self::ThresholdPlusOnePlusOne | Self::ThresholdPlusTwoPlusOne => &[("target", 1)],
            Self::PreexistingState => &[("target", 1)],
            Self::DifferentJunctions => &[("left", 1), ("right", 1)],
            Self::ParallelArrows => &[("source", 1), ("target", 0)],
            Self::ZeroDelayChain => &[("a", 1), ("b", 1)],
            Self::ZeroDelayFanoutMerge => &[("a", 1), ("b", 1), ("c", 1), ("d", 1)],
            Self::ZeroDelayCycle => &[("a", 1), ("b", 1)],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Permutation {
    Identity,
    ReverseInput,
    ReverseCellInsertion,
    ReverseArrowInsertion,
    ReversePhysicalNames,
    RandomPhysicalNames,
}

impl Permutation {
    const ALL: [Self; 6] = [
        Self::Identity,
        Self::ReverseInput,
        Self::ReverseCellInsertion,
        Self::ReverseArrowInsertion,
        Self::ReversePhysicalNames,
        Self::RandomPhysicalNames,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::ReverseInput => "reverse_input",
            Self::ReverseCellInsertion => "reverse_cell_insertion",
            Self::ReverseArrowInsertion => "reverse_arrow_insertion",
            Self::ReversePhysicalNames => "reverse_physical_names",
            Self::RandomPhysicalNames => "random_physical_names",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<String>,
    work: Work,
    tick: i64,
    durable_cells: Vec<String>,
    durable_arrows: Vec<String>,
    transient_cells: Vec<String>,
    pending: usize,
    loads: usize,
    quiescent: bool,
    fires: BTreeMap<&'static str, usize>,
}

struct LogicalWorld {
    body: PlasticSubstrate,
    cells: BTreeMap<&'static str, CellId>,
    cell_names: BTreeMap<CellId, &'static str>,
    arrow_names: BTreeMap<ArrowId, &'static str>,
}

#[derive(Clone, Copy)]
struct CellDef {
    name: &'static str,
    position: i32,
    threshold: i32,
}

#[derive(Clone, Copy)]
struct ArrowDef {
    name: &'static str,
    from: &'static str,
    to: &'static str,
    coupling: i32,
}

fn definitions(family: Family) -> (Vec<CellDef>, Vec<ArrowDef>) {
    let cells = match family {
        Family::SameJunction
        | Family::ThresholdPlusOnePlusOne
        | Family::ThresholdPlusTwoMinusOne
        | Family::ThresholdPlusTwoPlusOne
        | Family::PreexistingState => vec![CellDef {
            name: "target",
            position: 0,
            threshold: match family {
                Family::PreexistingState => 3,
                _ => 2,
            },
        }],
        Family::DifferentJunctions => vec![
            CellDef {
                name: "left",
                position: 0,
                threshold: 1,
            },
            CellDef {
                name: "right",
                position: 10,
                threshold: 1,
            },
        ],
        Family::ParallelArrows => vec![
            CellDef {
                name: "source",
                position: 0,
                threshold: 1,
            },
            CellDef {
                name: "target",
                position: 10,
                threshold: 2,
            },
        ],
        Family::ZeroDelayChain | Family::ZeroDelayCycle => vec![
            CellDef {
                name: "a",
                position: 0,
                threshold: 1,
            },
            CellDef {
                name: "b",
                position: 10,
                threshold: 1,
            },
        ],
        Family::ZeroDelayFanoutMerge => vec![
            CellDef {
                name: "a",
                position: 0,
                threshold: 1,
            },
            CellDef {
                name: "b",
                position: 10,
                threshold: 1,
            },
            CellDef {
                name: "c",
                position: 20,
                threshold: 1,
            },
            CellDef {
                name: "d",
                position: 30,
                threshold: 2,
            },
        ],
    };
    let arrows = match family {
        Family::ParallelArrows => vec![
            ArrowDef {
                name: "positive",
                from: "source",
                to: "target",
                coupling: 2,
            },
            ArrowDef {
                name: "negative",
                from: "source",
                to: "target",
                coupling: -1,
            },
        ],
        Family::ZeroDelayChain => vec![ArrowDef {
            name: "a_to_b",
            from: "a",
            to: "b",
            coupling: 1,
        }],
        Family::ZeroDelayCycle => vec![
            ArrowDef {
                name: "a_to_b",
                from: "a",
                to: "b",
                coupling: 1,
            },
            ArrowDef {
                name: "b_to_a",
                from: "b",
                to: "a",
                coupling: 1,
            },
        ],
        Family::ZeroDelayFanoutMerge => vec![
            ArrowDef {
                name: "a_to_b",
                from: "a",
                to: "b",
                coupling: 1,
            },
            ArrowDef {
                name: "a_to_c",
                from: "a",
                to: "c",
                coupling: 1,
            },
            ArrowDef {
                name: "b_to_d",
                from: "b",
                to: "d",
                coupling: 1,
            },
            ArrowDef {
                name: "c_to_d",
                from: "c",
                to: "d",
                coupling: 1,
            },
        ],
        _ => Vec::new(),
    };
    (cells, arrows)
}

fn physical_names(count: usize, permutation: Permutation) -> Vec<u64> {
    let mut values = (0..count)
        .map(|index| 10_000 + u64::try_from(index).unwrap() * 101)
        .collect::<Vec<_>>();
    match permutation {
        Permutation::ReversePhysicalNames => values.reverse(),
        Permutation::RandomPhysicalNames if count > 1 => values.rotate_left(1),
        _ => {}
    }
    values
}

fn build(family: Family, permutation: Permutation, mechanics: MechanicalConfig) -> LogicalWorld {
    let (cell_defs, mut arrow_defs) = definitions(family);
    let physical = physical_names(cell_defs.len(), permutation);
    let mut insertion = (0..cell_defs.len()).collect::<Vec<_>>();
    if permutation == Permutation::ReverseCellInsertion {
        insertion.reverse();
    }
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(60_000), 32, 32, mechanics);
    body.set_physical_tracing(true);
    let mut cells = BTreeMap::new();
    let mut cell_names = BTreeMap::new();
    for index in insertion {
        let def = cell_defs[index];
        let id = body.add_cell(CellSpec {
            physical_id: physical[index],
            position: def.position,
            region: 0,
            threshold: def.threshold,
            resistance: 1_000,
        });
        cells.insert(def.name, id);
        cell_names.insert(id, def.name);
    }
    if permutation == Permutation::ReverseArrowInsertion {
        arrow_defs.reverse();
    }
    let mut arrow_names = BTreeMap::new();
    for def in arrow_defs {
        let id = body.add_arrow(ArrowSpec {
            from: cells[def.from],
            to: cells[def.to],
            delay: 0,
            phase: 0,
            coupling: def.coupling,
            resistance: 1_000,
            mode: TransmissionMode::Drive,
        });
        arrow_names.insert(id, def.name);
    }
    LogicalWorld {
        body,
        cells,
        cell_names,
        arrow_names,
    }
}

fn inputs(family: Family) -> Vec<(&'static str, i64, i32, i32)> {
    match family {
        Family::SameJunction | Family::ThresholdPlusTwoMinusOne => {
            vec![("target", 0, 0, 2), ("target", 0, 0, -1)]
        }
        Family::ThresholdPlusOnePlusOne => {
            vec![("target", 0, 0, 1), ("target", 0, 0, 1)]
        }
        Family::ThresholdPlusTwoPlusOne => {
            vec![("target", 0, 0, 2), ("target", 0, 0, 1)]
        }
        Family::DifferentJunctions => {
            vec![("left", 0, 0, 1), ("right", 0, 0, 1)]
        }
        Family::ParallelArrows => vec![("source", 0, 0, 1)],
        Family::ZeroDelayChain | Family::ZeroDelayFanoutMerge | Family::ZeroDelayCycle => {
            vec![("a", 0, 0, 1)]
        }
        Family::PreexistingState => vec![
            ("target", 0, 0, 1),
            ("target", 0, 1, 2),
            ("target", 0, 1, -1),
            ("target", 0, 2, 1),
        ],
    }
}

fn admit(
    world: &mut LogicalWorld,
    family: Family,
    permutation: Permutation,
) -> Vec<PhysicalTransition> {
    let mut admitted = inputs(family);
    if permutation == Permutation::ReverseInput {
        admitted.reverse();
    }
    for (serial, (target, tick, phase, impulse)) in admitted.into_iter().enumerate() {
        world.body.enter(SpikeInput {
            arrival_tick: tick,
            phase,
            origin_physical: 900_000 + u64::try_from(serial).unwrap(),
            target: world.cells[target],
            impulse,
        });
    }
    Vec::new()
}

fn logical_event(world: &LogicalWorld, event: &PhysicalEvent) -> String {
    let cell = |id: CellId| world.cell_names.get(&id).copied().unwrap_or("generated");
    let arrow = |id: ArrowId| {
        world
            .arrow_names
            .get(&id)
            .copied()
            .unwrap_or("generated_link")
    };
    match event {
        PhysicalEvent::DriveIncidence {
            target,
            arrivals,
            impulse,
            causal_wave,
        } => format!(
            "INCIDENCE:{}:arrivals={arrivals}:impulse={impulse}:wave={causal_wave}",
            cell(*target)
        ),
        PhysicalEvent::Deliver {
            mode,
            target,
            impulse,
        } => format!("DELIVER:{mode:?}:{}:{impulse}", cell(*target)),
        PhysicalEvent::Fire { cell: fired } => format!("FIRE:{}", cell(*fired)),
        PhysicalEvent::Resistance {
            arrow: id,
            before,
            after,
        } => format!("RESISTANCE:{}:{before}:{after}", arrow(*id)),
        PhysicalEvent::Deallocate { arrow: id } => format!("DEALLOCATE:{}", arrow(*id)),
        PhysicalEvent::Proposal {
            arrow: id,
            from,
            to,
        } => format!("PROPOSAL:{}:{}:{}", arrow(*id), cell(*from), cell(*to)),
        PhysicalEvent::Crossing(crossing) => format!(
            "CROSSING:{}:{}:{}",
            crossing.from_physical, crossing.to_physical, crossing.impulse
        ),
        PhysicalEvent::QualifiedLocalTraversal { arrow: id } => format!("QLP:{}", arrow(*id)),
    }
}

fn normalize_trace(world: &LogicalWorld, trace: &[PhysicalTransition]) -> Vec<String> {
    let mut waves: BTreeMap<(i64, i32, u64), (Vec<String>, Vec<String>, Vec<String>)> =
        BTreeMap::new();
    let mut current_wave = None;
    for transition in trace {
        match &transition.event {
            PhysicalEvent::DriveIncidence { causal_wave, .. } => {
                let key = (transition.tick, transition.phase, *causal_wave);
                waves
                    .entry(key)
                    .or_default()
                    .0
                    .push(logical_event(world, &transition.event));
                current_wave = Some(key);
            }
            PhysicalEvent::Fire { .. } => waves
                .get_mut(&current_wave.expect("every SI0 fire follows a Drive wave"))
                .expect("current SI0 wave must exist")
                .1
                .push(logical_event(world, &transition.event)),
            _ => waves
                .get_mut(&current_wave.expect("every SI0 effect follows a Drive wave"))
                .expect("current SI0 wave must exist")
                .2
                .push(logical_event(world, &transition.event)),
        }
    }
    waves
        .into_iter()
        .map(|((tick, phase, wave), (mut incidences, mut fires, mut effects))| {
            incidences.sort();
            fires.sort();
            effects.sort();
            format!(
                "{tick}:{phase}:wave={wave}:INCIDENCES=[{}]:FIRES=[{}]:EFFECTS=[{}]",
                incidences.join("|"),
                fires.join("|"),
                effects.join("|")
            )
        })
        .collect()
}

fn read_u32(bytes: &[u8], offset: &mut usize) -> u32 {
    let value = u32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    value
}

fn read_i32(bytes: &[u8], offset: &mut usize) -> i32 {
    let value = i32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    value
}

fn read_u64(bytes: &[u8], offset: &mut usize) -> u64 {
    let value = u64::from_le_bytes(bytes[*offset..*offset + 8].try_into().unwrap());
    *offset += 8;
    value
}

fn read_i64(bytes: &[u8], offset: &mut usize) -> i64 {
    let value = i64::from_le_bytes(bytes[*offset..*offset + 8].try_into().unwrap());
    *offset += 8;
    value
}

fn normalized_transient(
    world: &LogicalWorld,
    checkpoint: &[u8],
    tick: i64,
) -> (Vec<String>, usize, usize) {
    assert_eq!(&checkpoint[..8], b"TLLIVE01");
    let mut offset = 10;
    assert_eq!(read_i64(checkpoint, &mut offset), tick);
    let _next_serial = read_u64(checkpoint, &mut offset);
    let manifest_len = usize::try_from(read_u64(checkpoint, &mut offset)).unwrap();
    let body_len = usize::try_from(read_u64(checkpoint, &mut offset)).unwrap();
    let cell_count = usize::try_from(read_u32(checkpoint, &mut offset)).unwrap();
    let arrow_count = usize::try_from(read_u32(checkpoint, &mut offset)).unwrap();
    let pending = usize::try_from(read_u32(checkpoint, &mut offset)).unwrap();
    let loads = usize::try_from(read_u32(checkpoint, &mut offset)).unwrap();
    let _payload_len = read_u64(checkpoint, &mut offset);
    offset += 32 + manifest_len + body_len;
    let mut cells = Vec::new();
    for _ in 0..cell_count {
        let id = CellId(read_u64(checkpoint, &mut offset));
        let state = read_i32(checkpoint, &mut offset);
        let last_update = read_i64(checkpoint, &mut offset);
        let refractory_until = read_i64(checkpoint, &mut offset);
        cells.push(format!(
            "{}:state={state}:age={}:refractory={}",
            world.cell_names[&id],
            if state == 0 {
                0
            } else {
                tick.saturating_sub(last_update)
            },
            refractory_until.saturating_sub(tick).max(0)
        ));
    }
    offset += arrow_count * (8 + 8 + 8 + 8 + 1);
    assert!(offset <= checkpoint.len());
    cells.sort();
    (cells, pending, loads)
}

fn observe(family: Family, permutation: Permutation, mechanics: MechanicalConfig) -> Observation {
    let mut world = build(family, permutation, mechanics);
    let mut prefix_trace = admit(&mut world, family, permutation);
    let run = world.body.propagate();
    prefix_trace.extend(run.physical_trace.clone());
    let trace = normalize_trace(&world, &prefix_trace);
    let tick = world.body.clock().tick;
    let arena = world.body.arena_body(1);
    let mut durable_cells = arena
        .cells
        .iter()
        .map(|cell| {
            format!(
                "{}:pos={}:threshold={}:resistance={}:generation={}:live={}",
                world.cell_names[&cell.id],
                cell.position,
                cell.threshold,
                cell.resistance,
                cell.generation.0,
                cell.live
            )
        })
        .collect::<Vec<_>>();
    durable_cells.sort();
    let mut durable_arrows = arena
        .arrows
        .iter()
        .map(|arrow| {
            format!(
                "{}:{}->{}:delay={}:phase={}:coupling={}:resistance={}:generation={}:live={}",
                world.arrow_names[&arrow.id],
                world.cell_names[&arrow.from.id],
                world.cell_names[&arrow.to.id],
                arrow.delay,
                arrow.phase,
                arrow.coupling,
                arrow.resistance,
                arrow.generation.0,
                arrow.live
            )
        })
        .collect::<Vec<_>>();
    durable_arrows.sort();
    let checkpoint = world
        .body
        .live_checkpoint(1)
        .unwrap()
        .canonical_bytes()
        .unwrap();
    let (transient_cells, pending, loads) = normalized_transient(&world, &checkpoint, tick);
    let mut fires = BTreeMap::new();
    for name in world.cells.keys() {
        let id = world.cells[name];
        let count = prefix_trace
            .iter()
            .filter(
                |transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == id),
            )
            .count();
        fires.insert(*name, count);
    }
    Observation {
        trace,
        work: run.work,
        tick,
        durable_cells,
        durable_arrows,
        transient_cells,
        pending,
        loads,
        quiescent: run.naturally_quiescent,
        fires,
    }
}

fn expected(family: Family, observation: &Observation) -> bool {
    family
        .expected_fires()
        .iter()
        .all(|(name, count)| observation.fires.get(name).copied().unwrap_or(0) == *count)
        && observation.quiescent
        && observation.pending == 0
        && observation.loads == 0
}

fn mechanics_name(mechanics: MechanicalConfig) -> &'static str {
    if mechanics == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn main() {
    let output = env::args_os().nth(1).map(PathBuf::from).unwrap_or_else(|| {
            PathBuf::from("experiments/results/si0_simultaneous_local_incidence_v2")
    });
    fs::create_dir_all(&output).unwrap();
    let mut csv = String::from(
        "family,permutation,mechanics,replay_equal,cross_equal,baseline_equal,expected_firing,physical_work,tick,quiescent,pending,loads,trace_hash,case_pass\n",
    );
    let mut rows = 0usize;
    let mut passed = 0usize;
    let mut first_divergence = String::new();
    let mut maximum_work = 0_u64;
    for family in Family::ALL {
        let baseline_reference =
            observe(family, Permutation::Identity, MechanicalConfig::REFERENCE);
        let baseline_production =
            observe(family, Permutation::Identity, MechanicalConfig::PRODUCTION);
        for permutation in Permutation::ALL {
            let reference = observe(family, permutation, MechanicalConfig::REFERENCE);
            let reference_replay = observe(family, permutation, MechanicalConfig::REFERENCE);
            let production = observe(family, permutation, MechanicalConfig::PRODUCTION);
            let production_replay = observe(family, permutation, MechanicalConfig::PRODUCTION);
            let cross_equal = reference == production;
            for (mechanics, observation, replay, baseline) in [
                (
                    MechanicalConfig::REFERENCE,
                    &reference,
                    &reference_replay,
                    &baseline_reference,
                ),
                (
                    MechanicalConfig::PRODUCTION,
                    &production,
                    &production_replay,
                    &baseline_production,
                ),
            ] {
                rows += 1;
                let replay_equal = observation == replay;
                let baseline_equal = observation == baseline;
                let expected_firing = expected(family, observation);
                let case_pass = replay_equal && cross_equal && baseline_equal && expected_firing;
                passed += usize::from(case_pass);
                maximum_work = maximum_work.max(observation.work.total());
                if !case_pass && first_divergence.is_empty() {
                    first_divergence = format!(
                        "family={} permutation={} mechanics={} replay={} cross={} baseline={} expected={} fires={:?} trace={:?}",
                        family.name(),
                        permutation.name(),
                        mechanics_name(mechanics),
                        replay_equal,
                        cross_equal,
                        baseline_equal,
                        expected_firing,
                        observation.fires,
                        observation.trace,
                    );
                }
                writeln!(
                    csv,
                    "{},{},{},{replay_equal},{cross_equal},{baseline_equal},{expected_firing},{},{},{},{},{},{},{case_pass}",
                    family.name(),
                    permutation.name(),
                    mechanics_name(mechanics),
                    observation.work.total(),
                    observation.tick,
                    observation.quiescent,
                    observation.pending,
                    observation.loads,
                    ContentHash::of(format!("{:?}", observation.trace).as_bytes()),
                )
                .unwrap();
            }
        }
    }
    fs::write(output.join("matrix.csv"), &csv).unwrap();
    let verdict = if passed == rows { "PASS" } else { "NEGATIVE" };
    let report = format!(
        "# SI0 simultaneous local incidence v2\n\n\
         - families: `{}`\n\
         - rows passing: `{passed}/{rows}`\n\
         - verdict: `{verdict}`\n\
         - maximum PhysicalWork: `{maximum_work}`\n\
         - first divergence: `{}`\n",
        Family::ALL.len(),
        if first_divergence.is_empty() {
            "none"
        } else {
            &first_divergence
        }
    );
    fs::write(output.join("report.md"), report).unwrap();
    assert_eq!(passed, rows, "SI0 simultaneous incidence matrix failed");
    println!("SI0_SIMULTANEOUS_LOCAL_INCIDENCE_POSITIVE_V2");
}
