#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowRef, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig,
    PhysicalEvent, PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [4_100_000, 4_200_000];
const PHASES: std::ops::Range<i64> = 0..10;
const EXPECTED_CASES: usize = 100;
const EXPECTED_ROWS: usize = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    WeakLifetime,
    ResistanceLifetime,
    TraversalIndependence,
    TimePartition,
    StaleDelivery,
}

impl Family {
    const ALL: [Self; 5] = [
        Self::WeakLifetime,
        Self::ResistanceLifetime,
        Self::TraversalIndependence,
        Self::TimePartition,
        Self::StaleDelivery,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::WeakLifetime => "phase_free_weak_lifetime",
            Self::ResistanceLifetime => "resistance_scales_lifetime",
            Self::TraversalIndependence => "traversal_cannot_alter_forgetting",
            Self::TimePartition => "host_time_partition_invariance",
            Self::StaleDelivery => "local_death_blocks_stale_delivery",
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
    decay_load: u64,
    participation: u64,
    generation_resolves: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    age: i64,
    tick: i64,
    arrows: Vec<ArrowState>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    points: Vec<Point>,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    body_hash: String,
    final_tick: i64,
    naturally_quiescent: bool,
    partition_equal: bool,
    target_fires: u64,
}

struct World {
    body: PlasticSubstrate,
    origin: i64,
    arrows: Vec<(ArrowId, ArrowRef)>,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    naturally_quiescent: bool,
}

impl World {
    fn new(root: u64, phase: i64, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 64, 128, mechanics);
        body.set_physical_tracing(true);
        body.advance_time(phase);
        Self {
            body,
            origin: phase,
            arrows: Vec::new(),
            trace: Vec::new(),
            work: WorkTotals::default(),
            naturally_quiescent: true,
        }
    }

    fn cell(&mut self, physical_id: u64, position: i32, threshold: i32) -> CellId {
        self.body.add_cell(CellSpec {
            physical_id,
            position,
            region: 0,
            threshold,
            resistance: 10_000,
        })
    }

    fn arrow(&mut self, from: CellId, to: CellId, resistance: u32, delay: i64) -> ArrowId {
        let id = self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay,
            phase: 0,
            coupling: 1,
            resistance,
            mode: TransmissionMode::Drive,
        });
        self.arrows.push((id, self.body.arrow_reference(id)));
        id
    }

    fn advance_age(&mut self, age: i64) {
        self.work
            .add(self.body.advance_time(self.origin.saturating_add(age)));
    }

    fn arrive(&mut self, target: CellId, age: i64, origin_physical: u64) {
        let result = self.body.arrive(
            &[SpikeInput {
                arrival_tick: self.origin.saturating_add(age),
                phase: 0,
                origin_physical,
                target,
                impulse: 1,
            }],
            256,
        );
        self.trace.extend(result.physical_trace);
        self.work.add(result.work);
        self.naturally_quiescent &= result.naturally_quiescent;
    }

    fn point(&self, age: i64) -> Point {
        let durable = self.body.arena_body(1);
        Point {
            age,
            tick: self.body.clock().tick,
            arrows: self
                .arrows
                .iter()
                .map(|(id, reference)| {
                    let arrow = durable
                        .arrows
                        .iter()
                        .find(|arrow| arrow.id == *id)
                        .expect("ARROW identity remains addressable");
                    ArrowState {
                        live: arrow.live,
                        resistance: arrow.resistance,
                        decay_load: self.body.local_decay_load(*id),
                        participation: self.body.local_participation(*id),
                        generation_resolves: self.body.resolve_arrow(*reference).is_some(),
                    }
                })
                .collect(),
        }
    }

    fn finish(
        self,
        points: Vec<Point>,
        partition_equal: bool,
        target: Option<CellId>,
    ) -> Observation {
        let target_fires = target.map_or(0, |target| {
            self.trace
                .iter()
                .filter(|transition| {
                    matches!(transition.event, PhysicalEvent::Fire { cell } if cell == target)
                })
                .count() as u64
        });
        Observation {
            points,
            trace: self.trace,
            work: self.work,
            body_hash: ContentHash::of(&self.body.canonical_body_bytes(1).unwrap()).to_string(),
            final_tick: self.body.clock().tick,
            naturally_quiescent: self.naturally_quiescent,
            partition_equal,
            target_fires,
        }
    }
}

fn simple_arrows(root: u64, phase: i64, mechanics: MechanicalConfig, resistances: &[u32]) -> World {
    let mut world = World::new(root, phase, mechanics);
    for (index, resistance) in resistances.iter().copied().enumerate() {
        let offset = i32::try_from(index).unwrap_or(i32::MAX).saturating_mul(200);
        let source = world.cell(root + 100 + index as u64, offset, 1);
        let target = world.cell(root + 200 + index as u64, offset.saturating_add(100), 100);
        world.arrow(source, target, resistance, 0);
    }
    world
}

fn weak_lifetime(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = simple_arrows(root, phase, mechanics, &[1]);
    let mut points = vec![world.point(0)];
    for age in [1, 9, 10] {
        world.advance_age(age);
        points.push(world.point(age));
    }
    world.finish(points, true, None)
}

fn resistance_lifetime(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = simple_arrows(root, phase, mechanics, &[1, 2, 4]);
    let mut points = vec![world.point(0)];
    for age in [9, 10, 19, 20, 39, 40] {
        world.advance_age(age);
        points.push(world.point(age));
    }
    world.finish(points, true, None)
}

fn traversal_independence(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let source_a = world.cell(root + 100, 0, 1);
    let target_a = world.cell(root + 200, 100, 100);
    let source_b = world.cell(root + 300, 300, 1);
    let target_b = world.cell(root + 400, 400, 100);
    world.arrow(source_a, target_a, 4, 0);
    world.arrow(source_b, target_b, 4, 0);
    let mut points = vec![world.point(0)];
    for age in [1, 3, 5, 7] {
        world.arrive(source_a, age, root + 900 + age as u64);
    }
    world.advance_age(9);
    points.push(world.point(9));
    world.advance_age(39);
    points.push(world.point(39));
    world.advance_age(40);
    points.push(world.point(40));
    world.finish(points, true, None)
}

fn partition_invariance(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut tickwise = simple_arrows(root, phase, mechanics, &[4]);
    let mut jumped = simple_arrows(root, phase, mechanics, &[4]);
    let mut tickwise_points = vec![tickwise.point(0)];
    for age in 1..=37 {
        tickwise.advance_age(age);
    }
    tickwise_points.push(tickwise.point(37));
    for age in [7, 19, 37] {
        jumped.advance_age(age);
    }
    let state_equal = tickwise.point(37) == jumped.point(37)
        && tickwise.work == jumped.work
        && tickwise.body.clock() == jumped.body.clock()
        && tickwise.body.canonical_body_bytes(1).unwrap()
            == jumped.body.canonical_body_bytes(1).unwrap();
    tickwise.advance_age(40);
    jumped.advance_age(40);
    let future_equal = tickwise.point(40) == jumped.point(40)
        && tickwise.work == jumped.work
        && tickwise.body.canonical_body_bytes(1).unwrap()
            == jumped.body.canonical_body_bytes(1).unwrap();
    tickwise_points.push(tickwise.point(40));
    tickwise.finish(tickwise_points, state_equal && future_equal, None)
}

fn stale_delivery(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let source = world.cell(root + 100, 0, 1);
    let target = world.cell(root + 200, 100, 1);
    world.arrow(source, target, 1, 15);
    let mut points = vec![world.point(0)];
    world.arrive(source, 0, root + 900);
    points.push(world.point(15));
    world.finish(points, true, Some(target))
}

fn execute(root: u64, phase: i64, family: Family, mechanics: MechanicalConfig) -> Observation {
    match family {
        Family::WeakLifetime => weak_lifetime(root, phase, mechanics),
        Family::ResistanceLifetime => resistance_lifetime(root, phase, mechanics),
        Family::TraversalIndependence => traversal_independence(root, phase, mechanics),
        Family::TimePartition => partition_invariance(root, phase, mechanics),
        Family::StaleDelivery => stale_delivery(root, phase, mechanics),
    }
}

fn point(observation: &Observation, age: i64) -> &Point {
    observation
        .points
        .iter()
        .find(|point| point.age == age)
        .expect("required local age is serialized")
}

fn predicate(family: Family, observation: &Observation) -> bool {
    if !observation.naturally_quiescent
        || observation.work.modulation != 0
        || observation.work.updates != 0
        || observation.work.proposals != 0
        || observation.work.qlp != 0
    {
        return false;
    }
    match family {
        Family::WeakLifetime => {
            point(observation, 0).arrows[0]
                == ArrowState {
                    live: true,
                    resistance: 1,
                    decay_load: 0,
                    participation: 0,
                    generation_resolves: true,
                }
                && point(observation, 1).arrows[0].resistance == 1
                && point(observation, 1).arrows[0].decay_load == 1
                && point(observation, 9).arrows[0].resistance == 1
                && point(observation, 9).arrows[0].decay_load == 9
                && !point(observation, 10).arrows[0].live
                && point(observation, 10).arrows[0].resistance == 0
                && !point(observation, 10).arrows[0].generation_resolves
                && observation.work.deallocations == 1
        }
        Family::ResistanceLifetime => {
            let at_10 = &point(observation, 10).arrows;
            let at_20 = &point(observation, 20).arrows;
            let at_40 = &point(observation, 40).arrows;
            !at_10[0].live
                && at_10[1].resistance == 1
                && at_10[2].resistance == 3
                && !at_20[1].live
                && at_20[2].resistance == 2
                && !at_40[2].live
                && observation.work.deallocations == 3
        }
        Family::TraversalIndependence => {
            for age in [0, 9, 39, 40] {
                let arrows = &point(observation, age).arrows;
                if arrows[0].live != arrows[1].live
                    || arrows[0].resistance != arrows[1].resistance
                    || arrows[0].decay_load != arrows[1].decay_load
                {
                    return false;
                }
            }
            point(observation, 9).arrows[0].participation > 0
                && point(observation, 9).arrows[1].participation == 0
                && observation.work.updates == 0
                && observation.work.deallocations == 2
        }
        Family::TimePartition => {
            observation.partition_equal
                && point(observation, 37).arrows[0].resistance == 1
                && point(observation, 37).arrows[0].decay_load == 7
                && !point(observation, 40).arrows[0].live
        }
        Family::StaleDelivery => {
            !point(observation, 15).arrows[0].live
                && observation.target_fires == 0
                && observation.work.drive == 1
                && observation.work.deallocations == 1
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

fn points_string(points: &[Point]) -> String {
    points
        .iter()
        .map(|point| {
            let arrows = point
                .arrows
                .iter()
                .map(|arrow| {
                    format!(
                        "{}/{}/{}/{}/{}",
                        u8::from(arrow.live),
                        arrow.resistance,
                        arrow.decay_load,
                        arrow.participation,
                        u8::from(arrow.generation_resolves),
                    )
                })
                .collect::<Vec<_>>()
                .join("|");
            format!("{}@{}:{arrows}", point.age, point.tick)
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn write_checksums(output: &Path) {
    let mut sums = String::new();
    for name in ["matrix.csv", "report.md"] {
        let bytes = fs::read(output.join(name)).unwrap();
        writeln!(sums, "{}  {name}", ContentHash::of(&bytes)).unwrap();
    }
    fs::write(output.join("SHA256SUMS"), sums).unwrap();
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/fd0_phase_free_local_forgetting_v1"));
    fs::create_dir_all(&output).unwrap();
    let mut csv = String::from(
        "case_id,root,creation_phase,family,mechanics,points,trace_hash,physical_work,drive,modulation,updates,proposals,deallocations,qlp,final_tick,body_hash,quiescent,partition_equal,target_fires,predicate_pass\n",
    );
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];
    let mut cases = 0_usize;
    let mut family_passes = [0_u64; 5];
    let mut maximum_work = 0_u64;

    for root in ROOTS {
        for phase in PHASES {
            for family in Family::ALL {
                cases += 1;
                let reference = execute(root, phase, family, mechanics[0]);
                let reference_replay = execute(root, phase, family, mechanics[0]);
                assert_eq!(reference_replay, reference);
                let production = execute(root, phase, family, mechanics[1]);
                let production_replay = execute(root, phase, family, mechanics[1]);
                assert_eq!(production_replay, production);
                assert_eq!(production, reference);
                let passed = predicate(family, &reference);
                assert!(passed, "FD0 family failed: {}", family.name());
                family_passes[family as usize] = family_passes[family as usize].saturating_add(1);
                maximum_work = maximum_work.max(reference.work.physical);
                for (kind, observation) in [(mechanics[0], &reference), (mechanics[1], &production)]
                {
                    writeln!(
                        csv,
                        "{cases},{root},{phase},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        family.name(),
                        mechanics_name(kind),
                        points_string(&observation.points),
                        ContentHash::of(format!("{:?}", observation.trace).as_bytes()),
                        observation.work.physical,
                        observation.work.drive,
                        observation.work.modulation,
                        observation.work.updates,
                        observation.work.proposals,
                        observation.work.deallocations,
                        observation.work.qlp,
                        observation.final_tick,
                        observation.body_hash,
                        u8::from(observation.naturally_quiescent),
                        u8::from(observation.partition_equal),
                        observation.target_fires,
                        u8::from(passed),
                    )
                    .unwrap();
                }
            }
        }
    }

    assert_eq!(cases, EXPECTED_CASES);
    assert_eq!(cases * 2, EXPECTED_ROWS);
    assert!(family_passes.iter().all(|count| *count == 20));
    let family_line = Family::ALL
        .into_iter()
        .map(|family| format!("{}={}/20", family.name(), family_passes[family as usize]))
        .collect::<Vec<_>>()
        .join(" ");
    let report = format!(
        "# FD0 phase-free local forgetting result v1\n\n\
         - physical cases: `{cases}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{}/{EXPECTED_ROWS}`\n\
         - exact same-mechanics replay runs: `{}`\n\
         - exact Reference/Production observations: `{cases}/{EXPECTED_CASES}`\n\
         - frozen families: `{family_line}`\n\
         - maximum PhysicalWork: `{maximum_work}`\n\
         - absolute creation phases: `0..9`\n\
         - local death ages for resistance 1/2/4: `10/20/40`\n\
         - traversal alters durable forgetting: `false`\n\
         - global forgetting epoch consulted: `false`\n\
         - resource competition, FD1, ARC, authority, oracle, arch.md: `0`\n",
        cases * 2,
        cases * 4,
    );
    fs::write(output.join("matrix.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!("FD0_COMPLETE physical_cases={cases} pass=true");
}
