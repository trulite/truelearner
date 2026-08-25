#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode,
};

const Q: u64 = 1_u64 << 32;
const ROOTS: [u64; 2] = [3_100_000, 3_200_000];
const PHASES: std::ops::Range<i32> = 0..10;
const EXPECTED_CASES: usize = 120;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    GradedPressure,
    PressureBeforeModulation,
    PressureAfterModulation,
    NoConsequence,
    WrongPath,
    RepeatedUse,
}

impl Family {
    const ALL: [Self; 6] = [
        Self::GradedPressure,
        Self::PressureBeforeModulation,
        Self::PressureAfterModulation,
        Self::NoConsequence,
        Self::WrongPath,
        Self::RepeatedUse,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::GradedPressure => "graded_pressure",
            Self::PressureBeforeModulation => "pressure_before_modulation",
            Self::PressureAfterModulation => "pressure_after_modulation",
            Self::NoConsequence => "no_consequence",
            Self::WrongPath => "wrong_path",
            Self::RepeatedUse => "repeated_use",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Totals {
    physical: u64,
    drive: u64,
    modulation: u64,
    updates: u64,
    deallocations: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    participation: Vec<u64>,
    support: Vec<u64>,
    pressure_load: Vec<u64>,
    resistance: Vec<u32>,
    live: Vec<bool>,
    effective_pressure: Vec<u64>,
    trace: Vec<PhysicalTransition>,
    work: Totals,
    tick: i64,
    body_hash: String,
    quiescent: bool,
}

struct World {
    body: PlasticSubstrate,
    arrows: Vec<ArrowId>,
    initial_resistance: Vec<u32>,
    trace: Vec<PhysicalTransition>,
    work: Totals,
    quiescent: bool,
}

impl World {
    fn new(root: u64, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 32, 64, mechanics);
        body.set_physical_tracing(true);
        Self {
            body,
            arrows: Vec::new(),
            initial_resistance: Vec::new(),
            trace: Vec::new(),
            work: Totals::default(),
            quiescent: true,
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

    fn arrow(
        &mut self,
        from: CellId,
        to: CellId,
        resistance: u32,
        mode: TransmissionMode,
    ) -> ArrowId {
        self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance,
            mode,
        })
    }

    fn candidate(&mut self, from: CellId, to: CellId, resistance: u32) -> ArrowId {
        let arrow = self.arrow(from, to, resistance, TransmissionMode::Drive);
        self.arrows.push(arrow);
        self.initial_resistance.push(resistance);
        arrow
    }

    fn arrive(&mut self, target: CellId, tick: i64, phase: i32, impulse: i32, origin: u64) {
        let result = self.body.arrive(
            &[SpikeInput {
                arrival_tick: tick,
                phase,
                origin_physical: origin,
                target,
                impulse,
            }],
            99,
        );
        self.work.physical = self
            .work
            .physical
            .saturating_add(result.work.physical_total());
        self.work.drive = self.work.drive.saturating_add(result.work.drive_deliveries);
        self.work.modulation = self
            .work
            .modulation
            .saturating_add(result.work.modulatory_deliveries);
        self.work.updates = self
            .work
            .updates
            .saturating_add(result.work.local_return_updates);
        self.work.deallocations = self
            .work
            .deallocations
            .saturating_add(result.work.physical_deallocations);
        self.quiescent &= result.naturally_quiescent;
        self.trace.extend(result.physical_trace);
    }

    fn advance(&mut self, tick: i64) {
        let work = self.body.advance_time(tick);
        self.work.physical = self.work.physical.saturating_add(work.physical_total());
        self.work.updates = self.work.updates.saturating_add(work.local_return_updates);
        self.work.deallocations = self
            .work
            .deallocations
            .saturating_add(work.physical_deallocations);
    }

    fn observe(self) -> Observation {
        let durable = self.body.arena_body(1);
        let mut participation = Vec::new();
        let mut support = Vec::new();
        let mut pressure_load = Vec::new();
        let mut resistance = Vec::new();
        let mut live = Vec::new();
        let mut effective_pressure = Vec::new();
        for (index, id) in self.arrows.iter().copied().enumerate() {
            participation.push(self.body.local_participation(id));
            support.push(self.body.local_plastic_support(id));
            pressure_load.push(self.body.local_pressure_load(id));
            let arrow = durable
                .arrows
                .iter()
                .find(|arrow| arrow.id == id)
                .expect("candidate ARROW must remain addressable");
            resistance.push(arrow.resistance);
            live.push(arrow.live);
            effective_pressure.push(
                u64::from(self.initial_resistance[index].saturating_sub(arrow.resistance))
                    .saturating_mul(Q)
                    .saturating_add(self.body.local_pressure_load(id)),
            );
        }
        Observation {
            participation,
            support,
            pressure_load,
            resistance,
            live,
            effective_pressure,
            trace: self.trace,
            work: self.work,
            tick: self.body.clock().tick,
            body_hash: ContentHash::of(&self.body.canonical_body_bytes(1).unwrap()).to_string(),
            quiescent: self.quiescent,
        }
    }
}

fn relax(mut value: u64, ticks: usize) -> u64 {
    for _ in 0..ticks {
        value = value.saturating_mul(15) / 16;
    }
    value
}

fn graded(root: u64, phase: i32, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, mechanics);
    let mut contacts = Vec::new();
    for index in 0..4 {
        let contact = world.cell(root + 10 + index, (index as i32) * 100, 1);
        let target = world.cell(root + 20 + index, (index as i32) * 100 + 20, 2);
        world.candidate(contact, target, 10);
        contacts.push(contact);
    }
    world.arrive(contacts[1], 1, phase, 1, root + 101);
    world.arrive(contacts[2], 5, phase, 1, root + 102);
    world.arrive(contacts[3], 9, phase, 1, root + 103);
    world.advance(10);
    world.observe()
}

fn pressure_and_modulation(
    root: u64,
    phase: i32,
    mechanics: MechanicalConfig,
    pressure_first: bool,
) -> Observation {
    let mut world = World::new(root, mechanics);
    let contact = world.cell(root + 1, 0, 1);
    let effect = world.cell(root + 2, 20, 2);
    let source = world.cell(root + 3, 40, 1);
    world.candidate(contact, effect, 1);
    world.arrow(effect, contact, 100, TransmissionMode::Modulatory);
    world.arrow(source, contact, 100, TransmissionMode::Drive);
    if pressure_first {
        world.arrive(source, 9, phase, 1, root + 201);
        world.arrive(effect, 10, phase - 5, 2, root + 202);
    } else {
        world.arrive(source, 8, phase, 1, root + 203);
        world.arrive(effect, 9, phase - 5, 2, root + 204);
        world.advance(10);
    }
    world.observe()
}

fn no_consequence(root: u64, phase: i32, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, mechanics);
    let contact = world.cell(root + 1, 0, 1);
    let target = world.cell(root + 2, 20, 2);
    world.candidate(contact, target, 1);
    world.arrive(contact, 9, phase, 1, root + 301);
    world.advance(500);
    world.observe()
}

fn wrong_path(root: u64, phase: i32, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, mechanics);
    let a = world.cell(root + 1, 0, 1);
    let xa = world.cell(root + 2, 20, 2);
    let b = world.cell(root + 3, 100, 1);
    let xb = world.cell(root + 4, 120, 2);
    let effect = world.cell(root + 5, 200, 1);
    world.candidate(a, xa, 1);
    world.candidate(b, xb, 1);
    world.arrow(effect, a, 100, TransmissionMode::Modulatory);
    world.arrive(b, 9, phase, 1, root + 401);
    world.arrive(effect, 10, phase - 5, 1, root + 402);
    world.observe()
}

fn repeated_use(root: u64, phase: i32, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, mechanics);
    let contact = world.cell(root + 1, 0, 1);
    let target = world.cell(root + 2, 20, 2);
    world.candidate(contact, target, 1);
    for (index, tick) in [1, 6, 11, 16, 21, 26].into_iter().enumerate() {
        world.arrive(contact, tick, phase, 1, root + 500 + index as u64);
    }
    assert_eq!(world.body.arena_body(1).arrows[0].resistance, 1);
    world.advance(500);
    world.observe()
}

fn run(root: u64, phase: i32, family: Family, mechanics: MechanicalConfig) -> Observation {
    match family {
        Family::GradedPressure => graded(root, phase, mechanics),
        Family::PressureBeforeModulation => {
            pressure_and_modulation(root, phase, mechanics, true)
        }
        Family::PressureAfterModulation => {
            pressure_and_modulation(root, phase, mechanics, false)
        }
        Family::NoConsequence => no_consequence(root, phase, mechanics),
        Family::WrongPath => wrong_path(root, phase, mechanics),
        Family::RepeatedUse => repeated_use(root, phase, mechanics),
    }
}

fn predicate(family: Family, observation: &Observation) -> bool {
    if !observation.quiescent {
        return false;
    }
    match family {
        Family::GradedPressure => {
            observation.participation
                == vec![0, relax(Q, 9), relax(Q, 5), relax(Q, 1)]
                && observation.effective_pressure.windows(2).all(|pair| pair[0] > pair[1])
                && observation.effective_pressure[0] == Q
                && observation.support.iter().all(|value| *value == 0)
        }
        Family::PressureBeforeModulation | Family::PressureAfterModulation => {
            observation.participation[0] > 0
                && observation.support[0] > 0
                && observation.resistance[0] == 4
                && observation.live[0]
                && observation.work.updates == 1
        }
        Family::NoConsequence | Family::RepeatedUse => {
            observation.support[0] == 0
                && observation.resistance[0] == 0
                && !observation.live[0]
                && observation.work.updates == 0
        }
        Family::WrongPath => {
            observation.support == vec![0, 0]
                && !observation.live[0]
                && observation.live[1]
                && observation.work.updates == 0
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

fn vector<T: ToString>(values: &[T]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
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
        .unwrap_or_else(|| PathBuf::from("results/pc0_pressure_closure_compatibility_v1"));
    fs::create_dir_all(&output).unwrap();
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];
    let mut csv = String::from(
        "case_id,root,phase,family,mechanics,participation,support,pressure_load,resistance,live,effective_pressure,physical_work,drive,modulation,updates,deallocations,tick,body_hash,quiescent,predicate_pass\n",
    );
    let mut cases = 0_usize;
    let mut passed = 0_usize;
    let mut family_pass = [true; 6];
    let mut max_work = 0_u64;
    for (family_index, family) in Family::ALL.into_iter().enumerate() {
        for root in ROOTS {
            for phase in PHASES {
                cases += 1;
                let reference = run(root, phase, family, mechanics[0]);
                assert_eq!(run(root, phase, family, mechanics[0]), reference);
                let production = run(root, phase, family, mechanics[1]);
                assert_eq!(run(root, phase, family, mechanics[1]), production);
                assert_eq!(production, reference);
                let pass = predicate(family, &reference);
                family_pass[family_index] &= pass;
                passed += usize::from(pass);
                max_work = max_work.max(reference.work.physical);
                for (kind, observation) in [(mechanics[0], &reference), (mechanics[1], &production)] {
                    writeln!(
                        csv,
                        "{cases},{root},{phase},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        family.name(),
                        mechanics_name(kind),
                        vector(&observation.participation),
                        vector(&observation.support),
                        vector(&observation.pressure_load),
                        vector(&observation.resistance),
                        vector(&observation.live),
                        vector(&observation.effective_pressure),
                        observation.work.physical,
                        observation.work.drive,
                        observation.work.modulation,
                        observation.work.updates,
                        observation.work.deallocations,
                        observation.tick,
                        observation.body_hash,
                        u8::from(observation.quiescent),
                        u8::from(pass),
                    )
                    .unwrap();
                }
            }
        }
    }
    assert_eq!(cases, EXPECTED_CASES);
    assert_eq!(passed, cases);
    assert!(family_pass.iter().all(|value| *value));
    let report = format!(
        "# PC0 pressure / closure compatibility result v1\n\n\
         - physical cases: `{cases}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{}/{}`\n\
         - same-mechanics exact replays: `{}/{}` runs\n\
         - exact Reference/Production histories: `{cases}/{EXPECTED_CASES}`\n\
         - predicate-positive cases: `{passed}/{cases}`\n\
         - family-complete signature: `{family_pass:?}`\n\
         - maximum PhysicalWork: `{max_work}`\n\
         - eligibility, timer, pressure exception, or new state: `0`\n\
         - ARC or authority execution: `0`\n",
        cases * 2,
        cases * 2,
        cases * 4,
        cases * 4,
    );
    fs::write(output.join("matrix.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!("PC0_COMPLETE cases={cases} passed={passed} max_work={max_work}");
}
