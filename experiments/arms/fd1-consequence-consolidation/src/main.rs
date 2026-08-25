#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowRef, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [4_300_000, 4_400_000];
const PHASES: std::ops::Range<i64> = 0..10;
const EXPECTED_CASES: usize = 140;
const EXPECTED_ROWS: usize = 280;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Unused,
    UseOnly,
    OneConsequence,
    EquivalentFuture,
    WrongPath,
    LateConsequence,
    RepeatedConsequence,
}

impl Family {
    const ALL: [Self; 7] = [
        Self::Unused,
        Self::UseOnly,
        Self::OneConsequence,
        Self::EquivalentFuture,
        Self::WrongPath,
        Self::LateConsequence,
        Self::RepeatedConsequence,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Unused => "unused_proposal",
            Self::UseOnly => "repeated_use_no_consequence",
            Self::OneConsequence => "one_qualified_consequence",
            Self::EquivalentFuture => "same_durable_state_same_future",
            Self::WrongPath => "wrong_path_modulation",
            Self::LateConsequence => "late_consequence_no_resurrection",
            Self::RepeatedConsequence => "repeated_supported_consequence",
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
    support: u64,
    generation_resolves: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    stage: &'static str,
    age: i64,
    tick: i64,
    state: ArrowState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    points: Vec<Point>,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    body_hash: String,
    final_tick: i64,
    naturally_quiescent: bool,
    equivalent_future: bool,
}

struct World {
    body: PlasticSubstrate,
    origin: i64,
    contact: CellId,
    target: CellId,
    candidate: ArrowId,
    candidate_ref: ArrowRef,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    naturally_quiescent: bool,
}

impl World {
    fn new(root: u64, phase: i64, mechanics: MechanicalConfig, with_return: bool) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 32, 64, mechanics);
        body.set_physical_tracing(true);
        body.advance_time(phase);
        let contact = body.add_cell(CellSpec {
            physical_id: root + 10,
            position: 0,
            region: 0,
            threshold: 1,
            resistance: 10_000,
        });
        let target = body.add_cell(CellSpec {
            physical_id: root + 20,
            position: 100,
            region: 0,
            threshold: 100,
            resistance: 10_000,
        });
        let candidate = body.add_arrow(ArrowSpec {
            from: contact,
            to: target,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: 1,
            mode: TransmissionMode::Drive,
        });
        let candidate_ref = body.arrow_reference(candidate);
        if with_return {
            let effect = body.add_cell(CellSpec {
                physical_id: root + 30,
                position: 300,
                region: 0,
                threshold: 1,
                resistance: 10_000,
            });
            body.add_arrow(ArrowSpec {
                from: effect,
                to: contact,
                delay: 0,
                phase: 1,
                coupling: 1,
                resistance: 100,
                mode: TransmissionMode::Modulatory,
            });
        }
        Self {
            body,
            origin: phase,
            contact,
            target,
            candidate,
            candidate_ref,
            trace: Vec::new(),
            work: WorkTotals::default(),
            naturally_quiescent: true,
        }
    }

    fn effect(&self) -> CellId {
        self.body
            .arena_body(1)
            .cells
            .iter()
            .find(|cell| cell.physical_id % 100 == 30)
            .map(|cell| cell.id)
            .expect("return fixture has an effect CELL")
    }

    fn advance(&mut self, age: i64) {
        self.work
            .add(self.body.advance_time(self.origin.saturating_add(age)));
    }

    fn input(&mut self, cell: CellId, age: i64, origin: u64) {
        let result = self.body.arrive(
            &[SpikeInput {
                arrival_tick: self.origin.saturating_add(age),
                phase: 0,
                origin_physical: origin,
                target: cell,
                impulse: 1,
            }],
            256,
        );
        self.trace.extend(result.physical_trace);
        self.work.add(result.work);
        self.naturally_quiescent &= result.naturally_quiescent;
    }

    fn traverse(&mut self, age: i64, ordinal: u64) {
        self.input(self.contact, age, ordinal);
    }

    fn consequence(&mut self, age: i64, ordinal: u64) {
        self.input(self.effect(), age, ordinal);
    }

    fn state(&self) -> ArrowState {
        let durable = self
            .body
            .arena_body(1)
            .arrows
            .into_iter()
            .find(|arrow| arrow.id == self.candidate)
            .expect("candidate remains addressable");
        ArrowState {
            live: durable.live,
            resistance: durable.resistance,
            decay_load: self.body.local_decay_load(self.candidate),
            participation: self.body.local_participation(self.candidate),
            support: self.body.local_plastic_support(self.candidate),
            generation_resolves: self.body.resolve_arrow(self.candidate_ref).is_some(),
        }
    }

    fn point(&self, stage: &'static str, age: i64) -> Point {
        Point {
            stage,
            age,
            tick: self.body.clock().tick,
            state: self.state(),
        }
    }

    fn finish(self, points: Vec<Point>, equivalent_future: bool) -> Observation {
        Observation {
            points,
            trace: self.trace,
            work: self.work,
            body_hash: ContentHash::of(&self.body.canonical_body_bytes(1).unwrap()).to_string(),
            final_tick: self.body.clock().tick,
            naturally_quiescent: self.naturally_quiescent,
            equivalent_future,
        }
    }
}

fn unused(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, false);
    let mut points = vec![world.point("initial", 0)];
    world.advance(9);
    points.push(world.point("age_9", 9));
    world.advance(10);
    points.push(world.point("death", 10));
    world.finish(points, true)
}

fn use_only(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, false);
    let mut points = vec![world.point("initial", 0)];
    for age in [1, 3, 5, 7] {
        world.traverse(age, root + 900 + age as u64);
    }
    world.advance(9);
    points.push(world.point("age_9", 9));
    world.advance(10);
    points.push(world.point("death", 10));
    world.finish(points, true)
}

fn one_consequence(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, true);
    let mut points = vec![world.point("initial", 0)];
    world.traverse(5, root + 901);
    points.push(world.point("before_consequence", 5));
    world.consequence(5, root + 902);
    points.push(world.point("after_consequence", 5));
    world.advance(44);
    points.push(world.point("last_live", 44));
    world.advance(45);
    points.push(world.point("death", 45));
    world.finish(points, true)
}

fn equivalent_future(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut early = World::new(root, phase, mechanics, true);
    early.traverse(1, root + 901);
    early.consequence(1, root + 902);
    let early_after = early.state();
    let early_work = early.work.physical;
    early.advance(40);
    let early_last = early.state();
    let early_future_work = early.work.physical.saturating_sub(early_work);
    early.advance(41);
    let early_dead = early.state();

    let mut late = World::new(root, phase, mechanics, true);
    late.traverse(9, root + 903);
    late.consequence(9, root + 904);
    let late_after = late.state();
    let late_work = late.work.physical;
    late.advance(48);
    let late_last = late.state();
    let late_future_work = late.work.physical.saturating_sub(late_work);
    late.advance(49);
    let late_dead = late.state();

    let equal = early_after == late_after
        && early_last == late_last
        && early_dead == late_dead
        && early_future_work == late_future_work;
    let points = vec![
        late.point("after_consequence", 9),
        late.point("last_live", 48),
        late.point("death", 49),
    ];
    late.finish(points, equal)
}

fn wrong_path(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, false);
    let wrong_contact = world.body.add_cell(CellSpec {
        physical_id: root + 40,
        position: 400,
        region: 0,
        threshold: 1,
        resistance: 10_000,
    });
    let wrong_target = world.body.add_cell(CellSpec {
        physical_id: root + 50,
        position: 500,
        region: 0,
        threshold: 100,
        resistance: 10_000,
    });
    world.body.add_arrow(ArrowSpec {
        from: wrong_contact,
        to: wrong_target,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: 100,
        mode: TransmissionMode::Drive,
    });
    let effect = world.body.add_cell(CellSpec {
        physical_id: root + 60,
        position: 600,
        region: 0,
        threshold: 1,
        resistance: 10_000,
    });
    world.body.add_arrow(ArrowSpec {
        from: effect,
        to: wrong_contact,
        delay: 0,
        phase: 1,
        coupling: 1,
        resistance: 100,
        mode: TransmissionMode::Modulatory,
    });
    let mut points = vec![world.point("initial", 0)];
    world.traverse(5, root + 901);
    points.push(world.point("before_wrong_return", 5));
    world.input(effect, 5, root + 902);
    points.push(world.point("after_wrong_return", 5));
    world.advance(10);
    points.push(world.point("death", 10));
    world.finish(points, true)
}

fn late_consequence(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, true);
    let mut points = vec![world.point("initial", 0)];
    world.traverse(1, root + 901);
    world.advance(10);
    points.push(world.point("dead_before_return", 10));
    world.consequence(11, root + 902);
    points.push(world.point("after_late_return", 11));
    world.finish(points, true)
}

fn repeated_consequence(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, true);
    let mut points = vec![world.point("initial", 0)];
    world.traverse(5, root + 901);
    world.consequence(5, root + 902);
    points.push(world.point("after_first", 5));
    world.traverse(6, root + 903);
    world.consequence(6, root + 904);
    points.push(world.point("after_second", 6));
    world.advance(75);
    points.push(world.point("last_live", 75));
    world.advance(76);
    points.push(world.point("death", 76));
    world.finish(points, true)
}

fn execute(root: u64, phase: i64, family: Family, mechanics: MechanicalConfig) -> Observation {
    match family {
        Family::Unused => unused(root, phase, mechanics),
        Family::UseOnly => use_only(root, phase, mechanics),
        Family::OneConsequence => one_consequence(root, phase, mechanics),
        Family::EquivalentFuture => equivalent_future(root, phase, mechanics),
        Family::WrongPath => wrong_path(root, phase, mechanics),
        Family::LateConsequence => late_consequence(root, phase, mechanics),
        Family::RepeatedConsequence => repeated_consequence(root, phase, mechanics),
    }
}

fn point<'a>(observation: &'a Observation, stage: &str) -> &'a Point {
    observation
        .points
        .iter()
        .find(|point| point.stage == stage)
        .expect("required stage is serialized")
}

fn predicate(family: Family, observation: &Observation) -> bool {
    if !observation.naturally_quiescent
        || observation.work.proposals != 0
        || observation.work.qlp != 0
    {
        return false;
    }
    match family {
        Family::Unused => {
            point(observation, "age_9").state.resistance == 1
                && point(observation, "age_9").state.decay_load == 9
                && !point(observation, "death").state.live
                && observation.work.updates == 0
        }
        Family::UseOnly => {
            point(observation, "age_9").state.participation > 0
                && point(observation, "age_9").state.resistance == 1
                && point(observation, "age_9").state.decay_load == 9
                && !point(observation, "death").state.live
                && observation.work.updates == 0
        }
        Family::OneConsequence => {
            let before = point(observation, "before_consequence").state;
            let after = point(observation, "after_consequence").state;
            before.resistance == 1
                && before.decay_load == 5
                && before.participation > 0
                && after.resistance == 4
                && after.decay_load == 0
                && after.support > 0
                && point(observation, "last_live").state.live
                && point(observation, "last_live").state.resistance == 1
                && point(observation, "last_live").state.decay_load == 9
                && !point(observation, "death").state.live
                && observation.work.updates == 1
        }
        Family::EquivalentFuture => {
            observation.equivalent_future
                && point(observation, "after_consequence").state.resistance == 4
                && point(observation, "after_consequence").state.decay_load == 0
                && point(observation, "last_live").state.resistance == 1
                && !point(observation, "death").state.live
        }
        Family::WrongPath => {
            let before = point(observation, "before_wrong_return").state;
            let after = point(observation, "after_wrong_return").state;
            before.resistance == 1
                && before.decay_load == 5
                && before.participation > 0
                && after == before
                && !point(observation, "death").state.live
                && observation.work.updates == 0
        }
        Family::LateConsequence => {
            let dead = point(observation, "dead_before_return").state;
            let after = point(observation, "after_late_return").state;
            !dead.live
                && dead.resistance == 0
                && !dead.generation_resolves
                && after == dead
                && observation.work.updates == 0
        }
        Family::RepeatedConsequence => {
            let first = point(observation, "after_first").state;
            let second = point(observation, "after_second").state;
            first.resistance == 4
                && first.decay_load == 0
                && second.resistance == 7
                && second.decay_load == 0
                && second.support > first.support
                && point(observation, "last_live").state.resistance == 1
                && point(observation, "last_live").state.decay_load == 9
                && !point(observation, "death").state.live
                && observation.work.updates == 2
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
            format!(
                "{}@{}/{}:{}/{}/{}/{}/{}/{}",
                point.stage,
                point.age,
                point.tick,
                u8::from(point.state.live),
                point.state.resistance,
                point.state.decay_load,
                point.state.participation,
                point.state.support,
                u8::from(point.state.generation_resolves),
            )
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
        .unwrap_or_else(|| PathBuf::from("results/fd1_consequence_consolidation_v1"));
    fs::create_dir_all(&output).unwrap();
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];
    let mut csv = String::from(
        "case_id,root,creation_phase,family,mechanics,points,trace_hash,physical_work,drive,modulation,updates,proposals,deallocations,qlp,final_tick,body_hash,quiescent,equivalent_future,predicate_pass\n",
    );
    let mut cases = 0_usize;
    let mut family_passes = [0_u64; 7];
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
                assert!(passed, "FD1 family failed: {}", family.name());
                family_passes[family as usize] = family_passes[family as usize].saturating_add(1);
                maximum_work = maximum_work.max(reference.work.physical);
                for (kind, observation) in [(mechanics[0], &reference), (mechanics[1], &production)]
                {
                    writeln!(
                        csv,
                        "{cases},{root},{phase},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
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
                        u8::from(observation.equivalent_future),
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
        "# FD1 consequence consolidation result v1\n\n\
         - physical cases: `{cases}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{}/{EXPECTED_ROWS}`\n\
         - exact same-mechanics replay runs: `{}`\n\
         - exact Reference/Production observations: `{cases}/{EXPECTED_CASES}`\n\
         - frozen families: `{family_line}`\n\
         - maximum PhysicalWork: `{maximum_work}`\n\
         - one consequence resistance/lifetime: `1->4 / death age 45`\n\
         - two consequences resistance/lifetime: `1->4->7 / death age 76`\n\
         - traversal-only consolidation: `false`\n\
         - wrong-path or stale resurrection: `false`\n\
         - permanent learned class: `false`\n\
         - RC0, CPC/PQLC replay, ARC, authority, oracle, arch.md: `0`\n",
        cases * 2,
        cases * 4,
    );
    fs::write(output.join("matrix.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!("FD1_COMPLETE physical_cases={cases} pass=true");
}
