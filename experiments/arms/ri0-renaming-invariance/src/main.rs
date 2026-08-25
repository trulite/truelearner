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
    TwoSource,
    ParallelArrow,
}

impl Family {
    const ALL: [Self; 2] = [Self::TwoSource, Self::ParallelArrow];

    fn name(self) -> &'static str {
        match self {
            Self::TwoSource => "same_tick_two_source",
            Self::ParallelArrow => "same_source_parallel_arrow",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Permutation {
    Identity,
    ReversePhysical,
    RandomPhysical,
    SwapCompetingPhysical,
    SwapUnrelatedPhysical,
    ReverseCellInsertion,
    ReverseArrowInsertion,
}

impl Permutation {
    const ALL: [Self; 7] = [
        Self::Identity,
        Self::ReversePhysical,
        Self::RandomPhysical,
        Self::SwapCompetingPhysical,
        Self::SwapUnrelatedPhysical,
        Self::ReverseCellInsertion,
        Self::ReverseArrowInsertion,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::ReversePhysical => "reverse_physical",
            Self::RandomPhysical => "random_physical",
            Self::SwapCompetingPhysical => "swap_competing_physical",
            Self::SwapUnrelatedPhysical => "swap_unrelated_physical",
            Self::ReverseCellInsertion => "reverse_cell_insertion",
            Self::ReverseArrowInsertion => "reverse_arrow_insertion",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<String>,
    work: Work,
    tick: i64,
    cells: Vec<String>,
    arrows: Vec<String>,
    transient: Vec<String>,
    pending_count: usize,
    load_count: usize,
    quiescent: bool,
    target_fires: usize,
}

struct LogicalWorld {
    body: PlasticSubstrate,
    cells: BTreeMap<&'static str, CellId>,
    cell_names: BTreeMap<CellId, &'static str>,
    physical_names: BTreeMap<u64, &'static str>,
    arrow_names: BTreeMap<ArrowId, &'static str>,
}

fn physical_values(permutation: Permutation) -> [u64; 4] {
    match permutation {
        Permutation::Identity
        | Permutation::ReverseCellInsertion
        | Permutation::ReverseArrowInsertion => [101, 202, 303, 404],
        Permutation::ReversePhysical => [404, 303, 202, 101],
        Permutation::RandomPhysical => [303, 101, 404, 202],
        Permutation::SwapCompetingPhysical => [202, 101, 303, 404],
        Permutation::SwapUnrelatedPhysical => [101, 202, 404, 303],
    }
}

fn build(family: Family, permutation: Permutation, mechanics: MechanicalConfig) -> LogicalWorld {
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(9_100_000), 8, 8, mechanics);
    body.set_physical_tracing(true);
    let logical = match family {
        Family::TwoSource => ["positive", "negative", "target", "unrelated"],
        Family::ParallelArrow => ["source", "target", "unrelated_a", "unrelated_b"],
    };
    let physical = physical_values(permutation);
    let insertion = if permutation == Permutation::ReverseCellInsertion {
        [3usize, 2, 1, 0]
    } else {
        [0usize, 1, 2, 3]
    };
    let mut cells = BTreeMap::new();
    let mut cell_names = BTreeMap::new();
    let mut physical_names = BTreeMap::new();
    for index in insertion {
        let threshold = match (family, logical[index]) {
            (Family::TwoSource, "target") | (Family::ParallelArrow, "target") => 1,
            (_, "unrelated") | (_, "unrelated_a") | (_, "unrelated_b") => 100,
            _ => 1,
        };
        let id = body.add_cell(CellSpec {
            physical_id: physical[index],
            position: i32::try_from(index).unwrap() * 10,
            region: 0,
            threshold,
            resistance: 500,
        });
        cells.insert(logical[index], id);
        cell_names.insert(id, logical[index]);
        physical_names.insert(physical[index], logical[index]);
    }
    let mut arrow_names = BTreeMap::new();
    let arrows: Vec<(&str, &str, i32, &'static str)> = match family {
        Family::TwoSource => vec![
            ("positive", "target", 1, "positive_link"),
            ("negative", "target", -1, "negative_link"),
        ],
        Family::ParallelArrow => {
            if permutation == Permutation::ReverseArrowInsertion {
                vec![
                    ("source", "target", -1, "negative_link"),
                    ("source", "target", 1, "positive_link"),
                ]
            } else {
                vec![
                    ("source", "target", 1, "positive_link"),
                    ("source", "target", -1, "negative_link"),
                ]
            }
        }
    };
    for (from, to, coupling, name) in arrows {
        let id = body.add_arrow(ArrowSpec {
            from: cells[from],
            to: cells[to],
            delay: 1,
            phase: 0,
            coupling,
            resistance: 500,
            mode: TransmissionMode::Drive,
        });
        arrow_names.insert(id, name);
    }
    LogicalWorld {
        body,
        cells,
        cell_names,
        physical_names,
        arrow_names,
    }
}

fn normalize_transition(world: &LogicalWorld, transition: &PhysicalTransition) -> String {
    let cell = |id: CellId| world.cell_names.get(&id).copied().unwrap_or("generated");
    let arrow = |id: ArrowId| {
        world
            .arrow_names
            .get(&id)
            .copied()
            .unwrap_or("generated_link")
    };
    let event = match &transition.event {
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
        PhysicalEvent::CellDeallocate {
            cell: id,
            before_generation,
            after_generation,
        } => format!(
            "CELL_DEALLOCATE:{}:{}:{}",
            cell(*id),
            before_generation.0,
            after_generation.0
        ),
        PhysicalEvent::CellProposal {
            cell: proposed,
            source,
            target,
        } => format!(
            "CELL_PROPOSAL:{}:{}:{}",
            cell(*proposed),
            cell(*source),
            cell(*target)
        ),
        PhysicalEvent::Proposal {
            arrow: id,
            from,
            to,
        } => format!("PROPOSAL:{}:{}:{}", arrow(*id), cell(*from), cell(*to)),
        PhysicalEvent::Crossing(crossing) => format!(
            "CROSSING:{}:{}:{}",
            world
                .physical_names
                .get(&crossing.from_physical)
                .copied()
                .unwrap_or("external"),
            world
                .physical_names
                .get(&crossing.to_physical)
                .copied()
                .unwrap_or("external"),
            crossing.impulse
        ),
        PhysicalEvent::QualifiedLocalTraversal { arrow: id } => {
            format!("QLP:{}", arrow(*id))
        }
    };
    format!("{}:{}:{event}", transition.tick, transition.phase)
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
    let checkpoint_tick = read_i64(checkpoint, &mut offset);
    assert_eq!(checkpoint_tick, tick);
    let _next_serial = read_u64(checkpoint, &mut offset);
    let manifest_len = usize::try_from(read_u64(checkpoint, &mut offset)).unwrap();
    let body_len = usize::try_from(read_u64(checkpoint, &mut offset)).unwrap();
    let cell_count = usize::try_from(read_u32(checkpoint, &mut offset)).unwrap();
    let arrow_count = usize::try_from(read_u32(checkpoint, &mut offset)).unwrap();
    let pending_count = usize::try_from(read_u32(checkpoint, &mut offset)).unwrap();
    let load_count = usize::try_from(read_u32(checkpoint, &mut offset)).unwrap();
    let _payload_len = read_u64(checkpoint, &mut offset);
    offset += 32 + manifest_len + body_len;
    let mut transient = Vec::new();
    for _ in 0..cell_count {
        let id = CellId(read_u64(checkpoint, &mut offset));
        let state = read_i32(checkpoint, &mut offset);
        let last_update = read_i64(checkpoint, &mut offset);
        let refractory_until = read_i64(checkpoint, &mut offset);
        let decay_load = read_u64(checkpoint, &mut offset);
        let name = world.cell_names[&id];
        let state_age = if state == 0 {
            0
        } else {
            tick.saturating_sub(last_update)
        };
        let refractory_remaining = refractory_until.saturating_sub(tick).max(0);
        transient.push(format!(
            "CELL:{name}:state={state}:state_age={state_age}:refractory={refractory_remaining}:decay={decay_load}"
        ));
    }
    for _ in 0..arrow_count {
        let id = ArrowId(read_u64(checkpoint, &mut offset));
        let participation = read_u64(checkpoint, &mut offset);
        let support = read_u64(checkpoint, &mut offset);
        let decay = read_u64(checkpoint, &mut offset);
        let trigger = checkpoint[offset];
        offset += 1;
        transient.push(format!(
            "ARROW:{}:participation={participation}:support={support}:decay={decay}:trigger={trigger}",
            world.arrow_names[&id]
        ));
    }
    transient.sort();
    (transient, pending_count, load_count)
}

fn observe(family: Family, permutation: Permutation, mechanics: MechanicalConfig) -> Observation {
    let mut world = build(family, permutation, mechanics);
    match family {
        Family::TwoSource => {
            world.body.enter(SpikeInput {
                arrival_tick: 0,
                phase: 0,
                origin_physical: 90_001,
                target: world.cells["positive"],
                impulse: 1,
            });
            world.body.enter(SpikeInput {
                arrival_tick: 0,
                phase: 0,
                origin_physical: 90_002,
                target: world.cells["negative"],
                impulse: 1,
            });
        }
        Family::ParallelArrow => world.body.enter(SpikeInput {
            arrival_tick: 0,
            phase: 0,
            origin_physical: 90_001,
            target: world.cells["source"],
            impulse: 1,
        }),
    }
    let run = world.body.propagate();
    let tick = world.body.clock().tick;
    let trace = run
        .physical_trace
        .iter()
        .map(|transition| normalize_transition(&world, transition))
        .collect::<Vec<_>>();
    let target = world.cells["target"];
    let target_fires = run
        .physical_trace
        .iter()
        .filter(
            |transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == target),
        )
        .count();
    let arena = world.body.arena_body(1);
    let mut cells = arena
        .cells
        .iter()
        .map(|cell| {
            format!(
                "{}:gen={}:pos={}:threshold={}:resistance={}:live={}",
                world.cell_names[&cell.id],
                cell.generation.0,
                cell.position,
                cell.threshold,
                cell.resistance,
                cell.live
            )
        })
        .collect::<Vec<_>>();
    cells.sort();
    let mut arrows = arena
        .arrows
        .iter()
        .map(|arrow| {
            format!(
                "{}:{}->{}:gen={}:delay={}:phase={}:coupling={}:resistance={}:live={}",
                world.arrow_names[&arrow.id],
                world.cell_names[&arrow.from.id],
                world.cell_names[&arrow.to.id],
                arrow.generation.0,
                arrow.delay,
                arrow.phase,
                arrow.coupling,
                arrow.resistance,
                arrow.live
            )
        })
        .collect::<Vec<_>>();
    arrows.sort();
    let checkpoint = world
        .body
        .live_checkpoint(1)
        .unwrap()
        .canonical_bytes()
        .unwrap();
    let (transient, pending_count, load_count) = normalized_transient(&world, &checkpoint, tick);
    Observation {
        trace,
        work: run.work,
        tick,
        cells,
        arrows,
        transient,
        pending_count,
        load_count,
        quiescent: run.naturally_quiescent,
        target_fires,
    }
}

fn mechanics_name(mechanics: MechanicalConfig) -> &'static str {
    if mechanics == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/ri0_renaming_invariance_v1"));
    fs::create_dir_all(&output).unwrap();
    let mut csv = String::from(
        "family,permutation,mechanics,replay_equal,cross_equal,baseline_equal,target_fires,trace_hash,physical_work,tick,quiescent,pending,loads,case_pass\n",
    );
    let mut total = 0usize;
    let mut passed = 0usize;
    let mut first_divergence = String::new();
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
                total += 1;
                let replay_equal = observation == replay;
                let baseline_equal = observation == baseline;
                let case_pass = replay_equal && cross_equal && baseline_equal;
                passed += usize::from(case_pass);
                if !case_pass && first_divergence.is_empty() {
                    let trace_index = observation
                        .trace
                        .iter()
                        .zip(&baseline.trace)
                        .position(|(left, right)| left != right)
                        .unwrap_or(observation.trace.len().min(baseline.trace.len()));
                    first_divergence = format!(
                        "family={} permutation={} mechanics={} trace_index={} baseline_fires={} candidate_fires={} baseline_event={:?} candidate_event={:?}",
                        family.name(),
                        permutation.name(),
                        mechanics_name(mechanics),
                        trace_index,
                        baseline.target_fires,
                        observation.target_fires,
                        baseline.trace.get(trace_index),
                        observation.trace.get(trace_index),
                    );
                }
                writeln!(
                    csv,
                    "{},{},{},{replay_equal},{cross_equal},{baseline_equal},{},{},{},{},{},{},{},{case_pass}",
                    family.name(),
                    permutation.name(),
                    mechanics_name(mechanics),
                    observation.target_fires,
                    ContentHash::of(format!("{:?}", observation.trace).as_bytes()),
                    observation.work.physical_total(),
                    observation.tick,
                    observation.quiescent,
                    observation.pending_count,
                    observation.load_count,
                )
                .unwrap();
            }
        }
    }
    fs::write(output.join("matrix.csv"), &csv).unwrap();
    let verdict = if passed == total {
        "PASS"
    } else {
        "REAL_NEGATIVE"
    };
    let report = format!(
        "# RI0 opaque-identity renaming invariance v1\n\n\
         - rows passing: `{passed}/{total}`\n\
         - verdict: `{verdict}`\n\
         - first divergence: `{}`\n",
        if first_divergence.is_empty() {
            "none"
        } else {
            &first_divergence
        }
    );
    fs::write(output.join("report.md"), report).unwrap();
    assert_eq!(passed, total, "RI0 renaming changed physical history");
    println!("RI0_RENAMING_INVARIANCE_POSITIVE_V1");
}
trait WorkTotal {
    fn physical_total(self) -> u64;
}

impl WorkTotal for Work {
    fn physical_total(self) -> u64 {
        self.total()
    }
}
