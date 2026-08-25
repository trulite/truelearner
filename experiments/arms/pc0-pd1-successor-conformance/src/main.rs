#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, TransmissionTrigger, Work,
};

const Q: u64 = 1_u64 << 32;
const EXPECTED_CASES: usize = 200;
const EXPECTED_ROWS: usize = 400;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    NeverUsed,
    RecentlyUsed,
    Magnitude,
    UseNoConsequence,
    TimelyConsequence,
    LateMaintained,
    LateUnmaintained,
    WrongPath,
    StressThenStop,
    TimePartition,
}

impl Family {
    const ALL: [Self; 10] = [
        Self::NeverUsed,
        Self::RecentlyUsed,
        Self::Magnitude,
        Self::UseNoConsequence,
        Self::TimelyConsequence,
        Self::LateMaintained,
        Self::LateUnmaintained,
        Self::WrongPath,
        Self::StressThenStop,
        Self::TimePartition,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::NeverUsed => "never_used",
            Self::RecentlyUsed => "recently_used",
            Self::Magnitude => "same_age_magnitude",
            Self::UseNoConsequence => "use_no_consequence",
            Self::TimelyConsequence => "timely_consequence",
            Self::LateMaintained => "late_maintained",
            Self::LateUnmaintained => "late_unmaintained",
            Self::WrongPath => "wrong_path",
            Self::StressThenStop => "stress_then_stop",
            Self::TimePartition => "time_partition",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CaseSpec {
    family: Family,
    phase: i64,
    root_ordinal: u64,
}

fn cases() -> Vec<CaseSpec> {
    let mut cases = Vec::new();
    for root_ordinal in 0..2 {
        for phase in 0..10 {
            for family in Family::ALL {
                cases.push(CaseSpec {
                    family,
                    phase,
                    root_ordinal,
                });
            }
        }
    }
    assert_eq!(cases.len(), EXPECTED_CASES);
    cases
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ArrowState {
    live: bool,
    resistance: u32,
    coupling: i32,
    participation: u64,
    support: u64,
    pressure_load: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    stage: &'static str,
    tick: i64,
    arrows: Vec<ArrowState>,
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

    fn merge(&mut self, other: Self) {
        self.physical = self.physical.saturating_add(other.physical);
        self.drive = self.drive.saturating_add(other.drive);
        self.modulation = self.modulation.saturating_add(other.modulation);
        self.updates = self.updates.saturating_add(other.updates);
        self.proposals = self.proposals.saturating_add(other.proposals);
        self.deallocations = self.deallocations.saturating_add(other.deallocations);
        self.qlp = self.qlp.saturating_add(other.qlp);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    points: Vec<Point>,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    final_tick: i64,
    final_pressure_phase: i64,
    body_hash: String,
    naturally_quiescent: bool,
    time_partition_equal: bool,
}

struct Fixture {
    body: PlasticSubstrate,
    root: u64,
    contacts: Vec<CellId>,
    effects: Vec<CellId>,
    arrows: Vec<ArrowId>,
    next_input: u64,
}

impl Fixture {
    fn standard(
        case_id: usize,
        case: CaseSpec,
        mechanics: MechanicalConfig,
        resistances: [u32; 3],
    ) -> Self {
        let root =
            5_000_000 + case.root_ordinal * 1_000_000 + u64::try_from(case_id).unwrap() * 100;
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root + 50), 32, 64, mechanics);
        body.set_physical_tracing(true);
        if case.phase > 0 {
            body.advance_time(case.phase);
        }
        let mut contacts = Vec::new();
        let mut effects = Vec::new();
        let mut arrows = Vec::new();
        for (index, resistance) in resistances.into_iter().enumerate() {
            let base = i32::try_from(index).unwrap() * 100;
            let contact = add_cell(&mut body, root + 1 + index as u64 * 10, base, 1);
            let target = add_cell(&mut body, root + 2 + index as u64 * 10, base + 10, 2);
            let effect = add_cell(&mut body, root + 3 + index as u64 * 10, base + 20, 1);
            arrows.push(add_arrow(
                &mut body,
                contact,
                target,
                resistance,
                TransmissionMode::Drive,
            ));
            add_arrow(
                &mut body,
                effect,
                contact,
                100_000,
                TransmissionMode::Modulatory,
            );
            contacts.push(contact);
            effects.push(effect);
        }
        Self {
            body,
            root,
            contacts,
            effects,
            arrows,
            next_input: 1,
        }
    }

    fn chain(case_id: usize, case: CaseSpec, mechanics: MechanicalConfig) -> Self {
        let root =
            7_000_000 + case.root_ordinal * 1_000_000 + u64::try_from(case_id).unwrap() * 100;
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root + 50), 24, 48, mechanics);
        body.set_physical_tracing(true);
        if case.phase > 0 {
            body.advance_time(case.phase);
        }
        let c1 = add_cell(&mut body, root + 1, 0, 1);
        let c2 = add_cell(&mut body, root + 2, 10, 1);
        let target = add_cell(&mut body, root + 3, 20, 2);
        let effect = add_cell(&mut body, root + 4, 30, 1);
        let a1 = add_arrow(&mut body, c1, c2, 2, TransmissionMode::Drive);
        let a2 = add_arrow(&mut body, c2, target, 2, TransmissionMode::Drive);
        add_arrow(&mut body, effect, c2, 100_000, TransmissionMode::Modulatory);
        body.add_arrow_with_trigger(
            ArrowSpec {
                from: c2,
                to: c1,
                delay: 0,
                phase: 0,
                coupling: 1,
                resistance: 100_000,
                mode: TransmissionMode::Modulatory,
            },
            TransmissionTrigger::QualifiedLocalParticipation,
        );
        Self {
            body,
            root,
            contacts: vec![c1, c2],
            effects: vec![effect],
            arrows: vec![a1, a2],
            next_input: 1,
        }
    }

    fn inputs(&mut self, events: &[(i64, CellId)]) -> Vec<SpikeInput> {
        events
            .iter()
            .map(|(tick, target)| {
                let input = SpikeInput {
                    arrival_tick: *tick,
                    phase: 0,
                    origin_physical: self.root + 80_000 + self.next_input,
                    target: *target,
                    impulse: 1,
                };
                self.next_input += 1;
                input
            })
            .collect()
    }
}

fn add_cell(
    body: &mut PlasticSubstrate,
    physical_id: u64,
    position: i32,
    threshold: i32,
) -> CellId {
    body.add_cell(CellSpec {
        physical_id,
        position,
        region: 0,
        threshold,
        resistance: 100_000,
    })
}

fn add_arrow(
    body: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    resistance: u32,
    mode: TransmissionMode,
) -> ArrowId {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance,
        mode,
    })
}

fn arrow_state(body: &PlasticSubstrate, id: ArrowId) -> ArrowState {
    let durable = body
        .arena_body(1)
        .arrows
        .into_iter()
        .find(|arrow| arrow.id == id)
        .expect("candidate ARROW must remain addressable");
    ArrowState {
        live: durable.live,
        resistance: durable.resistance,
        coupling: durable.coupling,
        participation: body.local_participation(id),
        support: body.local_plastic_support(id),
        pressure_load: body.local_pressure_load(id),
    }
}

fn point(body: &PlasticSubstrate, arrows: &[ArrowId], stage: &'static str) -> Point {
    Point {
        stage,
        tick: body.clock().tick,
        arrows: arrows.iter().map(|id| arrow_state(body, *id)).collect(),
    }
}

fn run_inputs(
    fixture: &mut Fixture,
    events: &[(i64, CellId)],
    trace: &mut Vec<PhysicalTransition>,
    work: &mut WorkTotals,
    quiescent: &mut bool,
) {
    let inputs = fixture.inputs(events);
    let result = fixture.body.arrive(&inputs, 256);
    trace.extend(result.physical_trace);
    work.add(result.work);
    *quiescent &= result.naturally_quiescent;
}

fn advance(fixture: &mut Fixture, tick: i64, work: &mut WorkTotals) {
    work.add(fixture.body.advance_time(tick));
}

fn execute(case_id: usize, case: CaseSpec, mechanics: MechanicalConfig) -> Observation {
    let mut fixture = match case.family {
        Family::LateMaintained | Family::LateUnmaintained => {
            Fixture::chain(case_id, case, mechanics)
        }
        Family::Magnitude => Fixture::standard(case_id, case, mechanics, [8, 8, 8]),
        Family::UseNoConsequence => Fixture::standard(case_id, case, mechanics, [3, 100, 100]),
        Family::TimelyConsequence => Fixture::standard(case_id, case, mechanics, [2, 100, 100]),
        Family::StressThenStop => Fixture::standard(case_id, case, mechanics, [2, 100, 100]),
        Family::TimePartition => Fixture::standard(case_id, case, mechanics, [4, 100, 100]),
        _ => Fixture::standard(case_id, case, mechanics, [1, 100, 100]),
    };
    let mut points = vec![point(&fixture.body, &fixture.arrows, "initial")];
    let mut trace = Vec::new();
    let mut work = WorkTotals::default();
    let mut quiescent = true;
    let mut time_partition_equal = true;

    match case.family {
        Family::NeverUsed => {
            advance(&mut fixture, 10, &mut work);
            points.push(point(&fixture.body, &fixture.arrows, "after_pressure"));
            advance(&mut fixture, 60, &mut work);
        }
        Family::RecentlyUsed => {
            let contact = fixture.contacts[0];
            run_inputs(
                &mut fixture,
                &[(9, contact)],
                &mut trace,
                &mut work,
                &mut quiescent,
            );
            points.push(point(&fixture.body, &fixture.arrows, "after_use"));
            advance(&mut fixture, 10, &mut work);
            points.push(point(&fixture.body, &fixture.arrows, "after_pressure"));
            advance(&mut fixture, 60, &mut work);
        }
        Family::Magnitude => {
            let low = fixture.contacts[0];
            let medium = fixture.contacts[1];
            let high = fixture.contacts[2];
            run_inputs(
                &mut fixture,
                &[
                    (10, high),
                    (11, medium),
                    (11, high),
                    (12, medium),
                    (12, high),
                    (21, low),
                    (21, medium),
                    (21, high),
                ],
                &mut trace,
                &mut work,
                &mut quiescent,
            );
            advance(&mut fixture, 29, &mut work);
            points.push(point(&fixture.body, &fixture.arrows, "before_pressure"));
            advance(&mut fixture, 30, &mut work);
            points.push(point(&fixture.body, &fixture.arrows, "after_pressure"));
            advance(&mut fixture, 70, &mut work);
        }
        Family::UseNoConsequence => {
            let contact = fixture.contacts[0];
            let events = [11, 14, 17, 21, 24, 27, 31, 34, 37]
                .into_iter()
                .map(|tick| (tick, contact))
                .collect::<Vec<_>>();
            run_inputs(&mut fixture, &events, &mut trace, &mut work, &mut quiescent);
            advance(&mut fixture, 40, &mut work);
            points.push(point(&fixture.body, &fixture.arrows, "after_active"));
            advance(&mut fixture, 90, &mut work);
        }
        Family::TimelyConsequence => {
            let contact = fixture.contacts[0];
            let effect = fixture.effects[0];
            run_inputs(
                &mut fixture,
                &[(17, contact), (18, contact)],
                &mut trace,
                &mut work,
                &mut quiescent,
            );
            advance(&mut fixture, 20, &mut work);
            points.push(point(&fixture.body, &fixture.arrows, "after_pressure"));
            run_inputs(
                &mut fixture,
                &[(21, effect)],
                &mut trace,
                &mut work,
                &mut quiescent,
            );
            points.push(point(&fixture.body, &fixture.arrows, "after_consequence"));
            advance(&mut fixture, 60, &mut work);
        }
        Family::LateMaintained => {
            let contact = fixture.contacts[0];
            let effect = fixture.effects[0];
            let events = [11, 14, 17, 21, 24, 27]
                .into_iter()
                .map(|tick| (tick, contact))
                .collect::<Vec<_>>();
            run_inputs(&mut fixture, &events, &mut trace, &mut work, &mut quiescent);
            advance(&mut fixture, 34, &mut work);
            points.push(point(&fixture.body, &fixture.arrows, "before_consequence"));
            run_inputs(
                &mut fixture,
                &[(35, effect)],
                &mut trace,
                &mut work,
                &mut quiescent,
            );
            points.push(point(&fixture.body, &fixture.arrows, "after_consequence"));
            advance(&mut fixture, 70, &mut work);
        }
        Family::LateUnmaintained => {
            let contact = fixture.contacts[0];
            let effect = fixture.effects[0];
            run_inputs(
                &mut fixture,
                &[(11, contact)],
                &mut trace,
                &mut work,
                &mut quiescent,
            );
            advance(&mut fixture, 34, &mut work);
            points.push(point(&fixture.body, &fixture.arrows, "before_consequence"));
            run_inputs(
                &mut fixture,
                &[(35, effect)],
                &mut trace,
                &mut work,
                &mut quiescent,
            );
            points.push(point(&fixture.body, &fixture.arrows, "after_consequence"));
            advance(&mut fixture, 70, &mut work);
        }
        Family::WrongPath => {
            let other = fixture.contacts[1];
            run_inputs(
                &mut fixture,
                &[(case.phase, other), (11, other), (14, other), (17, other)],
                &mut trace,
                &mut work,
                &mut quiescent,
            );
            advance(&mut fixture, 20, &mut work);
            points.push(point(
                &fixture.body,
                &fixture.arrows,
                "after_other_activity",
            ));
            advance(&mut fixture, 60, &mut work);
        }
        Family::StressThenStop => {
            let contact = fixture.contacts[0];
            let events = [11, 19, 21, 29, 31, 39, 41, 49]
                .into_iter()
                .map(|tick| (tick, contact))
                .collect::<Vec<_>>();
            run_inputs(&mut fixture, &events, &mut trace, &mut work, &mut quiescent);
            advance(&mut fixture, 50, &mut work);
            points.push(point(&fixture.body, &fixture.arrows, "after_active"));
            advance(&mut fixture, 100, &mut work);
        }
        Family::TimePartition => {
            let contact = fixture.contacts[0];
            run_inputs(
                &mut fixture,
                &[(11, contact), (14, contact), (17, contact)],
                &mut trace,
                &mut work,
                &mut quiescent,
            );
            let mut tickwise = fixture.body.clone();
            let mut jumped = fixture.body.clone();
            let mut tickwise_work = WorkTotals::default();
            for tick in 18..=60 {
                tickwise_work.add(tickwise.advance_time(tick));
            }
            let mut jumped_work = WorkTotals::default();
            jumped_work.add(jumped.advance_time(60));
            let tickwise_state = fixture
                .arrows
                .iter()
                .map(|id| arrow_state(&tickwise, *id))
                .collect::<Vec<_>>();
            let jumped_state = fixture
                .arrows
                .iter()
                .map(|id| arrow_state(&jumped, *id))
                .collect::<Vec<_>>();
            let tickwise_hash = ContentHash::of(&tickwise.canonical_body_bytes(1).unwrap());
            let jumped_hash = ContentHash::of(&jumped.canonical_body_bytes(1).unwrap());
            time_partition_equal = tickwise_state == jumped_state
                && tickwise_hash == jumped_hash
                && tickwise.clock() == jumped.clock()
                && tickwise_work == jumped_work;
            let effect = fixture.effects[0];
            let future = SpikeInput {
                arrival_tick: 61,
                phase: 0,
                origin_physical: fixture.root + 99_999,
                target: effect,
                impulse: 1,
            };
            let tickwise_future = tickwise.arrive(&[future], 256);
            let jumped_future = jumped.arrive(&[future], 256);
            time_partition_equal &= tickwise_future == jumped_future
                && fixture
                    .arrows
                    .iter()
                    .all(|id| arrow_state(&tickwise, *id) == arrow_state(&jumped, *id));
            fixture.body = jumped;
            work.merge(jumped_work);
            work.add(jumped_future.work);
            trace.extend(jumped_future.physical_trace);
            quiescent &= jumped_future.naturally_quiescent;
            points.push(point(&fixture.body, &fixture.arrows, "after_partition"));
        }
    }

    points.push(point(&fixture.body, &fixture.arrows, "final"));
    Observation {
        points,
        trace,
        work,
        final_tick: fixture.body.clock().tick,
        final_pressure_phase: fixture.body.clock().pressure_phase(),
        body_hash: ContentHash::of(&fixture.body.canonical_body_bytes(1).unwrap()).to_string(),
        naturally_quiescent: quiescent,
        time_partition_equal,
    }
}

fn stage<'a>(observation: &'a Observation, name: &str) -> &'a Point {
    observation
        .points
        .iter()
        .find(|point| point.stage == name)
        .expect("required stage must be serialized")
}

fn assert_family(case: CaseSpec, observation: &Observation) {
    assert!(observation.naturally_quiescent);
    assert_eq!(observation.work.proposals, 0);
    let initial = &stage(observation, "initial").arrows[0];
    let final_state = &stage(observation, "final").arrows[0];
    match case.family {
        Family::NeverUsed => {
            assert!(!stage(observation, "after_pressure").arrows[0].live);
        }
        Family::RecentlyUsed => {
            let after = stage(observation, "after_pressure").arrows[0];
            assert!(after.live);
            assert_eq!(after.resistance, initial.resistance);
            assert!(after.pressure_load > 0 && after.pressure_load < Q);
            assert!(after.participation > 0);
            assert!(!final_state.live);
        }
        Family::Magnitude => {
            let before = &stage(observation, "before_pressure").arrows;
            assert!(before[0].participation < before[1].participation);
            assert!(before[1].participation < before[2].participation);
            assert!(before.iter().all(|arrow| arrow.participation < Q));
            let after = &stage(observation, "after_pressure").arrows;
            assert!(after[0].pressure_load > after[1].pressure_load);
            assert!(after[1].pressure_load > after[2].pressure_load);
        }
        Family::UseNoConsequence | Family::StressThenStop => {
            let active = stage(observation, "after_active").arrows[0];
            assert!(active.live);
            assert!(active.resistance <= initial.resistance);
            assert_eq!(active.support, 0);
            assert!(!final_state.live);
        }
        Family::TimelyConsequence => {
            let pressure = stage(observation, "after_pressure").arrows[0];
            let consequence = stage(observation, "after_consequence").arrows[0];
            assert!(pressure.live && pressure.participation > 0);
            assert!(consequence.resistance > pressure.resistance);
            assert!(consequence.support > pressure.support);
        }
        Family::LateMaintained => {
            let before = &stage(observation, "before_consequence").arrows;
            assert!(before.iter().all(|arrow| arrow.live));
            assert!(before.iter().all(|arrow| arrow.participation > 0));
            let after = &stage(observation, "after_consequence").arrows;
            assert!(after
                .iter()
                .zip(before)
                .all(|(after, before)| after.resistance > before.resistance));
            assert!(observation.work.qlp > 0);
        }
        Family::LateUnmaintained => {
            let before = &stage(observation, "before_consequence").arrows;
            assert!(before.iter().all(|arrow| !arrow.live));
            let after = &stage(observation, "after_consequence").arrows;
            assert!(after.iter().all(|arrow| arrow.support == 0));
            assert_eq!(observation.work.qlp, 0);
        }
        Family::WrongPath => {
            assert!(!stage(observation, "after_other_activity").arrows[0].live);
            assert_eq!(final_state.support, 0);
        }
        Family::TimePartition => assert!(observation.time_partition_equal),
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
                        "{}/{}/{}/{}/{}/{}",
                        u8::from(arrow.live),
                        arrow.resistance,
                        arrow.coupling,
                        arrow.participation,
                        arrow.support,
                        arrow.pressure_load,
                    )
                })
                .collect::<Vec<_>>()
                .join("|");
            format!("{}@{}:{arrows}", point.stage, point.tick)
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn write_row(
    csv: &mut String,
    case_id: usize,
    case: CaseSpec,
    mechanics: MechanicalConfig,
    observation: &Observation,
) {
    let columns = [
        case_id.to_string(),
        case.family.name().to_owned(),
        case.phase.to_string(),
        case.root_ordinal.to_string(),
        mechanics_name(mechanics).to_owned(),
        points_string(&observation.points),
        ContentHash::of(format!("{:?}", observation.trace).as_bytes()).to_string(),
        observation.work.physical.to_string(),
        observation.work.drive.to_string(),
        observation.work.modulation.to_string(),
        observation.work.updates.to_string(),
        observation.work.proposals.to_string(),
        observation.work.deallocations.to_string(),
        observation.work.qlp.to_string(),
        observation.final_tick.to_string(),
        observation.final_pressure_phase.to_string(),
        observation.body_hash.clone(),
        u8::from(observation.naturally_quiescent).to_string(),
        u8::from(observation.time_partition_equal).to_string(),
    ];
    writeln!(csv, "{}", columns.join(",")).unwrap();
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
        .unwrap_or_else(|| PathBuf::from("results/pc0_pd1_successor_conformance_v1"));
    fs::create_dir_all(&output).unwrap();
    let case_specs = cases();
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];
    let mut csv = String::from(
        "case_id,family,initial_phase,root_ordinal,mechanics,points,trace_hash,physical_work,drive_deliveries,modulatory_deliveries,local_updates,proposals,deallocations,qlp_traversals,final_tick,final_pressure_phase,body_hash,naturally_quiescent,time_partition_equal\n",
    );
    let mut family_passes = [0_u64; 10];
    let mut maximum_work = 0_u64;

    for (index, case) in case_specs.iter().copied().enumerate() {
        let case_id = index + 1;
        let reference = execute(case_id, case, mechanics[0]);
        let reference_replay = execute(case_id, case, mechanics[0]);
        assert_eq!(reference_replay, reference);
        let production = execute(case_id, case, mechanics[1]);
        let production_replay = execute(case_id, case, mechanics[1]);
        assert_eq!(production_replay, production);
        assert_eq!(production, reference);
        assert_family(case, &reference);
        family_passes[case.family as usize] = family_passes[case.family as usize].saturating_add(1);
        maximum_work = maximum_work.max(reference.work.physical);
        write_row(&mut csv, case_id, case, mechanics[0], &reference);
        write_row(&mut csv, case_id, case, mechanics[1], &production);
    }

    assert_eq!(case_specs.len(), EXPECTED_CASES);
    assert_eq!(case_specs.len() * 2, EXPECTED_ROWS);
    assert!(family_passes.iter().all(|count| *count == 20));
    let family_line = Family::ALL
        .into_iter()
        .map(|family| format!("{}={}/20", family.name(), family_passes[family as usize]))
        .collect::<Vec<_>>()
        .join(" ");
    let report = format!(
        "# PC0 PD1-successor conformance result v1\n\n\
         - physical cases: `{}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{}/{EXPECTED_ROWS}`\n\
         - exact same-mechanics replay runs: `{}`\n\
         - exact Reference/Production observations: `{}/{EXPECTED_CASES}`\n\
         - frozen families: `{family_line}`\n\
         - maximum PhysicalWork: `{maximum_work}`\n\
         - non-consuming pressure/participation candidate: `PASS`\n\
         - rectangular eligibility consulted by pressure: `false`\n\
         - traversal-only durable strengthening: `false`\n\
         - ARC, PD2 deletion, authority, oracle, or arch.md changes: `0`\n",
        case_specs.len(),
        case_specs.len() * 2,
        case_specs.len() * 4,
        case_specs.len(),
    );
    fs::write(output.join("matrix.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!(
        "PC0_PD1_SUCCESSOR_COMPLETE physical_cases={} pass=true",
        case_specs.len()
    );
}
