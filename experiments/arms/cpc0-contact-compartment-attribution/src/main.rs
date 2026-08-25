#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, ContentHash, MechanicalConfig, PhysicalEvent, PhysicalTransition,
    PlasticSubstrate, SpikeInput, TransmissionMode,
};

const ROOTS: [u64; 2] = [1_300_000, 1_400_000];
const EXPECTED_CASES: usize = 220;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    OldSourceLocal,
    ContactA,
    ContactB,
    ANotTraversed,
    DriveAtA,
    LateModulation,
    DenseDistractors,
    SwappedIdentity,
    ReverseOrder,
    TimingOffset,
    ContactFanout,
}

impl Scenario {
    const ALL: [Self; 11] = [
        Self::OldSourceLocal,
        Self::ContactA,
        Self::ContactB,
        Self::ANotTraversed,
        Self::DriveAtA,
        Self::LateModulation,
        Self::DenseDistractors,
        Self::SwappedIdentity,
        Self::ReverseOrder,
        Self::TimingOffset,
        Self::ContactFanout,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::OldSourceLocal => "old_source_local",
            Self::ContactA => "c0_contact_a",
            Self::ContactB => "c1_contact_b",
            Self::ANotTraversed => "c2_a_not_traversed",
            Self::DriveAtA => "c3_drive_at_ca",
            Self::LateModulation => "c4_late_modulation",
            Self::DenseDistractors => "c5_dense_distractors",
            Self::SwappedIdentity => "c6_swapped_identity",
            Self::ReverseOrder => "c6_reverse_order",
            Self::TimingOffset => "c6_timing_offset",
            Self::ContactFanout => "c7_contact_fanout",
        }
    }

    fn expected_updates(self) -> (u64, u64) {
        match self {
            Self::OldSourceLocal | Self::ContactFanout => (1, 1),
            Self::ContactA
            | Self::DenseDistractors
            | Self::SwappedIdentity
            | Self::ReverseOrder
            | Self::TimingOffset => (1, 0),
            Self::ContactB => (0, 1),
            Self::ANotTraversed | Self::DriveAtA | Self::LateModulation => (0, 0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ArrowState {
    resistance: u32,
    coupling: i32,
    live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<PhysicalTransition>,
    a_updates: u64,
    b_updates: u64,
    a_initial: ArrowState,
    b_initial: ArrowState,
    a_final: ArrowState,
    b_final: ArrowState,
    drive_deliveries: u64,
    modulatory_deliveries: u64,
    fires: u64,
    eligible: u64,
    resistance_events: u64,
    proposals: u64,
    deallocations: u64,
    crossings: u64,
    ca_fires: u64,
    physical_work: u64,
    final_tick: i64,
    pressure_phase: i64,
    body_hash: String,
    quiescent: bool,
}

struct Geometry {
    body: PlasticSubstrate,
    p: truelearner_core::CellId,
    ca: truelearner_core::CellId,
    cb: truelearner_core::CellId,
    x: truelearner_core::CellId,
    y: truelearner_core::CellId,
    a: ArrowId,
    b: ArrowId,
    distractors: Vec<truelearner_core::CellId>,
}

fn add_cell(
    body: &mut PlasticSubstrate,
    physical_id: u64,
    position: i32,
    threshold: i32,
) -> truelearner_core::CellId {
    body.add_cell(truelearner_core::CellSpec {
        physical_id,
        position,
        region: 0,
        threshold,
        resistance: 100,
    })
}

fn add_arrow(
    body: &mut PlasticSubstrate,
    from: truelearner_core::CellId,
    to: truelearner_core::CellId,
    delay: i64,
    resistance: u32,
    mode: TransmissionMode,
) -> ArrowId {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling: 1,
        resistance,
        mode,
    })
}

fn old_geometry(root: u64, phase: i64, mechanics: MechanicalConfig) -> Geometry {
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root + 10), 16, 32, mechanics);
    body.set_physical_tracing(true);
    let p = add_cell(&mut body, root + 1, 0, 1);
    let x = add_cell(&mut body, root + 2, 10, 2);
    let y = add_cell(&mut body, root + 3, 20, 2);
    let a = add_arrow(&mut body, p, x, 0, 4, TransmissionMode::Drive);
    let b = add_arrow(&mut body, p, y, 0, 4, TransmissionMode::Drive);
    add_arrow(&mut body, x, p, 0, 100, TransmissionMode::Modulatory);
    if phase > 0 {
        body.advance_time(phase);
    }
    Geometry {
        body,
        p,
        ca: p,
        cb: p,
        x,
        y,
        a,
        b,
        distractors: Vec::new(),
    }
}

fn contact_geometry(
    root: u64,
    phase: i64,
    scenario: Scenario,
    mechanics: MechanicalConfig,
) -> Geometry {
    let dense = scenario == Scenario::DenseDistractors;
    let mut body = PlasticSubstrate::with_mechanics(
        ArenaId(root + 20),
        if dense { 64 } else { 16 },
        if dense { 128 } else { 32 },
        mechanics,
    );
    body.set_physical_tracing(true);
    let p = add_cell(&mut body, root + 1, 0, 1);
    let swapped = scenario == Scenario::SwappedIdentity;
    let reverse = scenario == Scenario::ReverseOrder;
    let (ca_physical, cb_physical) = if swapped {
        (root + 13, root + 12)
    } else {
        (root + 12, root + 13)
    };
    let (ca, cb, x, y) = if reverse {
        let cb = add_cell(&mut body, cb_physical, 20, 1);
        let y = add_cell(&mut body, root + 15, 40, 2);
        let ca = add_cell(&mut body, ca_physical, 10, 1);
        let x = add_cell(&mut body, root + 14, 30, 2);
        (ca, cb, x, y)
    } else {
        let ca = add_cell(&mut body, ca_physical, 10, 1);
        let x = add_cell(&mut body, root + 14, 30, 2);
        let cb = add_cell(&mut body, cb_physical, 20, 1);
        let y = add_cell(&mut body, root + 15, 40, 2);
        (ca, cb, x, y)
    };

    let timing_offset = scenario == Scenario::TimingOffset;
    if reverse {
        add_arrow(&mut body, p, cb, 0, 100, TransmissionMode::Drive);
        add_arrow(&mut body, p, ca, 0, 100, TransmissionMode::Drive);
    } else {
        add_arrow(&mut body, p, ca, 0, 100, TransmissionMode::Drive);
        add_arrow(
            &mut body,
            p,
            cb,
            if timing_offset { 2 } else { 0 },
            100,
            TransmissionMode::Drive,
        );
    }

    let (a, b) = if scenario == Scenario::ContactFanout {
        let a = add_arrow(&mut body, ca, x, 0, 4, TransmissionMode::Drive);
        let b = add_arrow(&mut body, ca, y, 0, 4, TransmissionMode::Drive);
        (a, b)
    } else if reverse {
        let b = add_arrow(&mut body, cb, y, 0, 4, TransmissionMode::Drive);
        let a = add_arrow(&mut body, ca, x, 0, 4, TransmissionMode::Drive);
        (a, b)
    } else {
        let a = add_arrow(&mut body, ca, x, 0, 4, TransmissionMode::Drive);
        let b = add_arrow(&mut body, cb, y, 0, 4, TransmissionMode::Drive);
        (a, b)
    };

    match scenario {
        Scenario::ContactB => {
            add_arrow(&mut body, y, cb, 0, 100, TransmissionMode::Modulatory);
        }
        Scenario::DriveAtA => {}
        _ => {
            add_arrow(&mut body, x, ca, 0, 100, TransmissionMode::Modulatory);
        }
    }

    let mut distractors = Vec::new();
    if dense {
        for index in 0..32_u64 {
            let source = add_cell(
                &mut body,
                root + 1_000 + index,
                1_000 + i32::try_from(index).unwrap() * 10,
                1,
            );
            add_arrow(&mut body, source, ca, 0, 100, TransmissionMode::Drive);
            distractors.push(source);
        }
    }
    if phase > 0 {
        body.advance_time(phase);
    }
    Geometry {
        body,
        p,
        ca,
        cb,
        x,
        y,
        a,
        b,
        distractors,
    }
}

fn input(target: truelearner_core::CellId, tick: i64, origin: u64, impulse: i32) -> SpikeInput {
    SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target,
        impulse,
    }
}

fn arrow_state(body: &PlasticSubstrate, id: ArrowId) -> ArrowState {
    let arrow = body
        .arena_body(1)
        .arrows
        .into_iter()
        .find(|arrow| arrow.id == id)
        .expect("candidate ARROW must remain addressable");
    ArrowState {
        resistance: arrow.resistance,
        coupling: arrow.coupling,
        live: arrow.live,
    }
}

fn admit(
    geometry: &mut Geometry,
    inputs: &[SpikeInput],
    trace: &mut Vec<PhysicalTransition>,
    physical_work: &mut u64,
) {
    let result = geometry.body.arrive(inputs, 99);
    assert!(result.naturally_quiescent);
    *physical_work = physical_work.saturating_add(result.work.physical_total());
    trace.extend(result.physical_trace);
}

fn execute(root: u64, phase: i64, scenario: Scenario, mechanics: MechanicalConfig) -> Observation {
    let mut geometry = if scenario == Scenario::OldSourceLocal {
        old_geometry(root, phase, mechanics)
    } else {
        contact_geometry(root, phase, scenario, mechanics)
    };
    let a_initial = arrow_state(&geometry.body, geometry.a);
    let b_initial = arrow_state(&geometry.body, geometry.b);
    let mut trace = Vec::new();
    let mut physical_work = 0_u64;

    match scenario {
        Scenario::OldSourceLocal => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 101, 1)],
                &mut trace,
                &mut physical_work,
            );
            let x = geometry.x;
            admit(
                &mut geometry,
                &[input(x, phase, root + 102, 1)],
                &mut trace,
                &mut physical_work,
            );
        }
        Scenario::ANotTraversed => {
            let cb = geometry.cb;
            admit(
                &mut geometry,
                &[input(cb, phase, root + 103, 1)],
                &mut trace,
                &mut physical_work,
            );
            let x = geometry.x;
            admit(
                &mut geometry,
                &[input(x, phase, root + 104, 2)],
                &mut trace,
                &mut physical_work,
            );
        }
        Scenario::DriveAtA => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 105, 1)],
                &mut trace,
                &mut physical_work,
            );
            let ca = geometry.ca;
            admit(
                &mut geometry,
                &[input(ca, phase + 1, root + 106, 1)],
                &mut trace,
                &mut physical_work,
            );
        }
        Scenario::LateModulation => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 107, 1)],
                &mut trace,
                &mut physical_work,
            );
            let x = geometry.x;
            admit(
                &mut geometry,
                &[input(x, phase + 5, root + 108, 2)],
                &mut trace,
                &mut physical_work,
            );
        }
        Scenario::DenseDistractors => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 109, 1)],
                &mut trace,
                &mut physical_work,
            );
            let inputs = geometry
                .distractors
                .iter()
                .enumerate()
                .map(|(index, target)| {
                    input(
                        *target,
                        phase + 1,
                        root + 2_000 + u64::try_from(index).unwrap(),
                        1,
                    )
                })
                .collect::<Vec<_>>();
            admit(&mut geometry, &inputs, &mut trace, &mut physical_work);
            let x = geometry.x;
            admit(
                &mut geometry,
                &[input(x, phase + 1, root + 110, 2)],
                &mut trace,
                &mut physical_work,
            );
        }
        Scenario::TimingOffset => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 111, 1)],
                &mut trace,
                &mut physical_work,
            );
            let x = geometry.x;
            admit(
                &mut geometry,
                &[input(x, phase + 2, root + 112, 2)],
                &mut trace,
                &mut physical_work,
            );
        }
        Scenario::ContactB => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 113, 1)],
                &mut trace,
                &mut physical_work,
            );
            let y = geometry.y;
            admit(
                &mut geometry,
                &[input(y, phase, root + 114, 1)],
                &mut trace,
                &mut physical_work,
            );
        }
        Scenario::ContactA
        | Scenario::SwappedIdentity
        | Scenario::ReverseOrder
        | Scenario::ContactFanout => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 115, 1)],
                &mut trace,
                &mut physical_work,
            );
            let x = geometry.x;
            admit(
                &mut geometry,
                &[input(x, phase, root + 116, 1)],
                &mut trace,
                &mut physical_work,
            );
        }
    }

    let mut drive_deliveries = 0_u64;
    let mut modulatory_deliveries = 0_u64;
    let mut fires = 0_u64;
    let mut eligible = 0_u64;
    let mut resistance_events = 0_u64;
    let mut proposals = 0_u64;
    let mut deallocations = 0_u64;
    let mut crossings = 0_u64;
    let mut ca_fires = 0_u64;
    let mut a_updates = 0_u64;
    let mut b_updates = 0_u64;
    for transition in &trace {
        match transition.event {
            PhysicalEvent::Deliver {
                mode: TransmissionMode::Drive,
                ..
            } => drive_deliveries = drive_deliveries.saturating_add(1),
            PhysicalEvent::Deliver {
                mode: TransmissionMode::Modulatory,
                ..
            } => modulatory_deliveries = modulatory_deliveries.saturating_add(1),
            PhysicalEvent::Fire { cell } => {
                fires = fires.saturating_add(1);
                ca_fires = ca_fires.saturating_add(u64::from(cell == geometry.ca));
            }
            PhysicalEvent::Eligible { .. } => eligible = eligible.saturating_add(1),
            PhysicalEvent::Resistance {
                arrow,
                before,
                after,
            } => {
                resistance_events = resistance_events.saturating_add(1);
                if after > before && arrow == geometry.a {
                    a_updates = a_updates.saturating_add(1);
                }
                if after > before && arrow == geometry.b {
                    b_updates = b_updates.saturating_add(1);
                }
            }
            PhysicalEvent::Proposal { .. } => proposals = proposals.saturating_add(1),
            PhysicalEvent::Deallocate { .. } => deallocations = deallocations.saturating_add(1),
            PhysicalEvent::Crossing(_) => crossings = crossings.saturating_add(1),
        }
    }

    Observation {
        a_updates,
        b_updates,
        a_initial,
        b_initial,
        a_final: arrow_state(&geometry.body, geometry.a),
        b_final: arrow_state(&geometry.body, geometry.b),
        drive_deliveries,
        modulatory_deliveries,
        fires,
        eligible,
        resistance_events,
        proposals,
        deallocations,
        crossings,
        ca_fires,
        physical_work,
        final_tick: geometry.body.clock().tick,
        pressure_phase: geometry.body.clock().pressure_phase(),
        body_hash: ContentHash::of(&geometry.body.canonical_body_bytes(1).unwrap()).to_string(),
        quiescent: true,
        trace,
    }
}

fn mechanics_name(mechanics: MechanicalConfig) -> &'static str {
    if mechanics == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn assert_scenario(scenario: Scenario, observation: &Observation) {
    assert_eq!(
        (observation.a_updates, observation.b_updates),
        scenario.expected_updates()
    );
    assert!(observation.quiescent);
    assert_eq!(observation.crossings, 0);
    assert_eq!(observation.deallocations, 0);
    assert!(observation.a_initial.live && observation.b_initial.live);
    assert!(observation.a_final.live && observation.b_final.live);
    if scenario.expected_updates().0 == 1 {
        assert_eq!(observation.a_final.resistance, 7);
        assert_eq!(observation.a_final.coupling, 2);
    }
    if scenario.expected_updates().1 == 1 {
        assert_eq!(observation.b_final.resistance, 7);
        assert_eq!(observation.b_final.coupling, 2);
    }
    if scenario == Scenario::DriveAtA {
        assert!(observation.ca_fires >= 2);
        assert_eq!(observation.modulatory_deliveries, 0);
    }
    if scenario == Scenario::LateModulation {
        assert!(observation.modulatory_deliveries >= 1);
        assert_eq!(observation.resistance_events, 0);
    }
    if scenario == Scenario::OldSourceLocal {
        assert_eq!((observation.a_updates, observation.b_updates), (1, 1));
    }
    if scenario == Scenario::ContactFanout {
        assert_eq!((observation.a_updates, observation.b_updates), (1, 1));
    }
}

fn write_row(
    csv: &mut String,
    case_id: usize,
    root: u64,
    phase: i64,
    scenario: Scenario,
    mechanics: MechanicalConfig,
    observation: &Observation,
) {
    writeln!(
        csv,
        "{case_id},{root},{phase},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        scenario.name(),
        mechanics_name(mechanics),
        observation.a_updates,
        observation.b_updates,
        observation.a_initial.resistance,
        observation.a_final.resistance,
        observation.a_initial.coupling,
        observation.a_final.coupling,
        u8::from(observation.a_final.live),
        observation.b_initial.resistance,
        observation.b_final.resistance,
        observation.b_initial.coupling,
        observation.b_final.coupling,
        u8::from(observation.b_final.live),
        observation.drive_deliveries,
        observation.modulatory_deliveries,
        observation.fires,
        observation.eligible,
        observation.resistance_events,
        observation.proposals,
        observation.deallocations,
        observation.crossings,
        observation.ca_fires,
        observation.physical_work,
        observation.final_tick,
        observation.pressure_phase,
        ContentHash::of(format!("{:?}", observation.trace).as_bytes()),
        observation.body_hash,
        u8::from(observation.quiescent),
    )
    .unwrap();
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
        .unwrap_or_else(|| PathBuf::from("results/cpc0_contact_compartment_v1"));
    fs::create_dir_all(&output).unwrap();
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];
    let mut csv = String::from(
        "case_id,root,initial_phase,scenario,mechanics,a_updates,b_updates,a_initial_resistance,a_final_resistance,a_initial_coupling,a_final_coupling,a_live,b_initial_resistance,b_final_resistance,b_initial_coupling,b_final_coupling,b_live,drive_deliveries,modulatory_deliveries,fires,eligible,resistance_events,proposals,deallocations,crossings,ca_fires,physical_work,final_tick,pressure_phase,trace_hash,body_hash,quiescent\n",
    );
    let mut cases = 0_usize;
    let mut old_alias_cases = 0_usize;
    let mut contact_specific_cases = 0_usize;
    let mut fanout_alias_cases = 0_usize;
    for root in ROOTS {
        for phase in 0..10 {
            for scenario in Scenario::ALL {
                cases += 1;
                let reference = execute(root, phase, scenario, mechanics[0]);
                let reference_replay = execute(root, phase, scenario, mechanics[0]);
                assert_eq!(reference_replay, reference);
                let production = execute(root, phase, scenario, mechanics[1]);
                let production_replay = execute(root, phase, scenario, mechanics[1]);
                assert_eq!(production_replay, production);
                assert_eq!(production, reference);
                assert_scenario(scenario, &reference);
                old_alias_cases += usize::from(
                    scenario == Scenario::OldSourceLocal
                        && (reference.a_updates, reference.b_updates) == (1, 1),
                );
                contact_specific_cases += usize::from(
                    matches!(
                        scenario,
                        Scenario::ContactA
                            | Scenario::ContactB
                            | Scenario::DenseDistractors
                            | Scenario::SwappedIdentity
                            | Scenario::ReverseOrder
                            | Scenario::TimingOffset
                    ) && (reference.a_updates, reference.b_updates) == scenario.expected_updates(),
                );
                fanout_alias_cases += usize::from(
                    scenario == Scenario::ContactFanout
                        && (reference.a_updates, reference.b_updates) == (1, 1),
                );
                for (kind, observation) in [(mechanics[0], &reference), (mechanics[1], &production)]
                {
                    write_row(&mut csv, cases, root, phase, scenario, kind, observation);
                }
            }
        }
    }
    assert_eq!(cases, EXPECTED_CASES);
    assert_eq!(old_alias_cases, 20);
    assert_eq!(contact_specific_cases, 120);
    assert_eq!(fanout_alias_cases, 20);

    let report = format!(
        "# CPC0 contact-compartment spatial attribution result v1\n\n\
         - physical cases: `{cases}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{}/{}`\n\
         - exact same-mechanics replay: `{}/{}`\n\
         - exact Reference/Production transition histories: `{cases}/{EXPECTED_CASES}`\n\
         - old source-local alias controls: `{old_alias_cases}/20`\n\
         - contact-specific attribution controls: `{contact_specific_cases}/120`\n\
         - contact fan-out granularity controls: `{fanout_alias_cases}/20`\n\
         - natural quiescence: `{}/{}` mechanics rows\n\
         - runtime or substrate-law changes: `0`\n\n\
         Ordinary CELL/ARROW topology changes attribution resolution under the\n\
         unchanged LR-C law. A compartment with two eligible outgoing ARROWs\n\
         credits both, so specificity is limited by physical granularity.\n",
        EXPECTED_CASES * 2,
        EXPECTED_CASES * 2,
        EXPECTED_CASES * 4,
        EXPECTED_CASES * 4,
        EXPECTED_CASES * 2,
        EXPECTED_CASES * 2,
    );
    fs::write(output.join("matrix.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!("CPC0_COMPLETE cases={cases} rows={}", EXPECTED_CASES * 2);
}
