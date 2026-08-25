#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode,
};

const ROOTS: [u64; 2] = [1_100_000, 1_200_000];
const MODULATION_DELAYS: [i64; 8] = [0, 1, 2, 3, 4, 5, 8, 12];
const EXPECTED_GATE_A_CASES: usize = 160;
const EXPECTED_DECAY_CASES: usize = 420;
const EXPECTED_GATE_B_CASES: usize = 160;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Control {
    AOnly,
    BOnly,
    AThenB,
    SubthresholdSource,
    NearbyPath,
    RepeatA,
    EqualActivityB,
    SharedFanout,
}

impl Control {
    const ALL: [Self; 8] = [
        Self::AOnly,
        Self::BOnly,
        Self::AThenB,
        Self::SubthresholdSource,
        Self::NearbyPath,
        Self::RepeatA,
        Self::EqualActivityB,
        Self::SharedFanout,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::AOnly => "a_only",
            Self::BOnly => "b_only",
            Self::AThenB => "a_then_b",
            Self::SubthresholdSource => "subthreshold_source",
            Self::NearbyPath => "nearby_path",
            Self::RepeatA => "repeat_a",
            Self::EqualActivityB => "equal_activity_b",
            Self::SharedFanout => "shared_fanout",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    a_level: u64,
    b_level: u64,
    a_contacts: Vec<u64>,
    b_contacts: Vec<u64>,
    a_updates: u64,
    b_updates: u64,
    causal_work: u64,
    final_tick: i64,
    pressure_phase: i64,
    body_hash: String,
    trace_hash: String,
    quiescent: bool,
}

struct Geometry {
    body: PlasticSubstrate,
    source_a: truelearner_core::CellId,
    source_b: truelearner_core::CellId,
    nearby_source: truelearner_core::CellId,
    target_a: truelearner_core::CellId,
    a: ArrowId,
    b: ArrowId,
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

fn drive(
    body: &mut PlasticSubstrate,
    from: truelearner_core::CellId,
    to: truelearner_core::CellId,
) -> ArrowId {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: 100,
        mode: TransmissionMode::Drive,
    })
}

fn modulation(
    from: truelearner_core::CellId,
    to: truelearner_core::CellId,
    body: &mut PlasticSubstrate,
) {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: 100,
        mode: TransmissionMode::Modulatory,
    });
}

fn independent_geometry(
    root: u64,
    phase: i64,
    mechanics: MechanicalConfig,
    subthreshold_a: bool,
) -> Geometry {
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root + 10), 12, 12, mechanics);
    body.set_physical_tracing(true);
    let source_a = add_cell(
        &mut body,
        root + 1,
        0,
        if subthreshold_a { 2 } else { 1 },
    );
    let target_a = add_cell(&mut body, root + 2, 10, 2);
    let source_b = add_cell(&mut body, root + 3, 20, 1);
    let target_b = add_cell(&mut body, root + 4, 30, 2);
    let nearby_source = add_cell(&mut body, root + 5, 40, 1);
    let nearby_target = add_cell(&mut body, root + 6, 50, 2);
    let a = drive(&mut body, source_a, target_a);
    let b = drive(&mut body, source_b, target_b);
    drive(&mut body, nearby_source, nearby_target);
    if phase > 0 {
        body.advance_time(phase);
    }
    Geometry {
        body,
        source_a,
        source_b,
        nearby_source,
        target_a,
        a,
        b,
    }
}

fn shared_geometry(root: u64, phase: i64, mechanics: MechanicalConfig) -> Geometry {
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root + 20), 10, 10, mechanics);
    body.set_physical_tracing(true);
    let source = add_cell(&mut body, root + 11, 0, 1);
    let target_a = add_cell(&mut body, root + 12, 10, 2);
    let target_b = add_cell(&mut body, root + 13, 20, 2);
    let nearby_source = add_cell(&mut body, root + 14, 30, 1);
    let a = drive(&mut body, source, target_a);
    let b = drive(&mut body, source, target_b);
    if phase > 0 {
        body.advance_time(phase);
    }
    Geometry {
        body,
        source_a: source,
        source_b: source,
        nearby_source,
        target_a,
        a,
        b,
    }
}

fn observe(
    body: &PlasticSubstrate,
    a: ArrowId,
    b: ArrowId,
    trace: &[PhysicalTransition],
    causal_work: u64,
) -> Observation {
    let mut a_contacts = Vec::new();
    let mut b_contacts = Vec::new();
    let mut a_updates = 0_u64;
    let mut b_updates = 0_u64;
    for transition in trace {
        match &transition.event {
            PhysicalEvent::ParticipationContact { arrow, level } if *arrow == a => {
                a_contacts.push(*level);
            }
            PhysicalEvent::ParticipationContact { arrow, level } if *arrow == b => {
                b_contacts.push(*level);
            }
            PhysicalEvent::Resistance { arrow, .. } if *arrow == a => {
                a_updates = a_updates.saturating_add(1);
            }
            PhysicalEvent::Resistance { arrow, .. } if *arrow == b => {
                b_updates = b_updates.saturating_add(1);
            }
            _ => {}
        }
    }
    Observation {
        a_level: body.path_participation(a),
        b_level: body.path_participation(b),
        a_contacts,
        b_contacts,
        a_updates,
        b_updates,
        causal_work,
        final_tick: body.clock().tick,
        pressure_phase: body.clock().pressure_phase(),
        body_hash: ContentHash::of(&body.canonical_body_bytes(1).unwrap()).to_string(),
        trace_hash: ContentHash::of(format!("{trace:?}").as_bytes()).to_string(),
        quiescent: true,
    }
}

fn admit(
    target: truelearner_core::CellId,
    geometry: &mut Geometry,
    tick: i64,
    origin: u64,
    impulse: i32,
    trace: &mut Vec<PhysicalTransition>,
    causal_work: &mut u64,
) {
    let result = geometry
        .body
        .arrive(&[input(target, tick, origin, impulse)], 99);
    assert!(result.naturally_quiescent);
    *causal_work = causal_work.saturating_add(result.work.physical_total());
    trace.extend(result.physical_trace);
}

fn run_gate_a(
    root: u64,
    phase: i64,
    control: Control,
    mechanics: MechanicalConfig,
) -> Observation {
    let mut geometry = if control == Control::SharedFanout {
        shared_geometry(root, phase, mechanics)
    } else {
        independent_geometry(
            root,
            phase,
            mechanics,
            control == Control::SubthresholdSource,
        )
    };
    let mut trace = Vec::new();
    let mut work = 0_u64;
    match control {
        Control::AOnly => admit(
            geometry.source_a,
            &mut geometry,
            phase,
            root + 101,
            1,
            &mut trace,
            &mut work,
        ),
        Control::BOnly => admit(
            geometry.source_b,
            &mut geometry,
            phase,
            root + 102,
            1,
            &mut trace,
            &mut work,
        ),
        Control::AThenB => {
            admit(
                geometry.source_a,
                &mut geometry,
                phase,
                root + 103,
                1,
                &mut trace,
                &mut work,
            );
            admit(
                geometry.source_b,
                &mut geometry,
                phase + 3,
                root + 104,
                1,
                &mut trace,
                &mut work,
            );
        }
        Control::SubthresholdSource => admit(
            geometry.source_a,
            &mut geometry,
            phase,
            root + 105,
            1,
            &mut trace,
            &mut work,
        ),
        Control::NearbyPath => admit(
            geometry.nearby_source,
            &mut geometry,
            phase,
            root + 106,
            1,
            &mut trace,
            &mut work,
        ),
        Control::RepeatA => {
            admit(
                geometry.source_a,
                &mut geometry,
                phase,
                root + 107,
                1,
                &mut trace,
                &mut work,
            );
            admit(
                geometry.source_a,
                &mut geometry,
                phase + 5,
                root + 108,
                1,
                &mut trace,
                &mut work,
            );
        }
        Control::EqualActivityB => {
            admit(
                geometry.source_b,
                &mut geometry,
                phase,
                root + 109,
                1,
                &mut trace,
                &mut work,
            );
            admit(
                geometry.source_b,
                &mut geometry,
                phase + 5,
                root + 110,
                1,
                &mut trace,
                &mut work,
            );
        }
        Control::SharedFanout => admit(
            geometry.source_a,
            &mut geometry,
            phase,
            root + 111,
            1,
            &mut trace,
            &mut work,
        ),
    }
    let final_tick = phase + 10;
    if geometry.body.clock().tick < final_tick {
        let elapsed = geometry.body.advance_time(final_tick);
        work = work.saturating_add(elapsed.physical_total());
    }
    observe(&geometry.body, geometry.a, geometry.b, &trace, work)
}

fn run_decay(root: u64, phase: i64, delay: i64, mechanics: MechanicalConfig) -> Observation {
    let mut geometry = independent_geometry(root, phase, mechanics, false);
    let mut trace = Vec::new();
    let mut work = 0_u64;
    admit(
        geometry.source_a,
        &mut geometry,
        phase,
        root + 201,
        1,
        &mut trace,
        &mut work,
    );
    if delay > 0 {
        let elapsed = geometry.body.advance_time(phase + delay);
        work = work.saturating_add(elapsed.physical_total());
    }
    observe(&geometry.body, geometry.a, geometry.b, &trace, work)
}

fn run_gate_b(root: u64, phase: i64, delay: i64, mechanics: MechanicalConfig) -> Observation {
    let mut geometry = shared_geometry(root, phase, mechanics);
    modulation(geometry.target_a, geometry.source_a, &mut geometry.body);
    let mut trace = Vec::new();
    let mut work = 0_u64;
    admit(
        geometry.source_a,
        &mut geometry,
        phase,
        root + 301,
        1,
        &mut trace,
        &mut work,
    );
    admit(
        geometry.target_a,
        &mut geometry,
        phase + delay,
        root + 302,
        2,
        &mut trace,
        &mut work,
    );
    observe(&geometry.body, geometry.a, geometry.b, &trace, work)
}

fn mechanics_name(mechanics: MechanicalConfig) -> &'static str {
    if mechanics == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn write_observation(
    csv: &mut String,
    fields: &[String],
    mechanics: MechanicalConfig,
    observation: &Observation,
) {
    let mut row = fields.to_vec();
    row.extend([
        mechanics_name(mechanics).to_owned(),
        observation.a_level.to_string(),
        observation.b_level.to_string(),
        observation
            .a_contacts
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join("|"),
        observation
            .b_contacts
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join("|"),
        observation.a_updates.to_string(),
        observation.b_updates.to_string(),
        observation.causal_work.to_string(),
        observation.final_tick.to_string(),
        observation.pressure_phase.to_string(),
        observation.body_hash.clone(),
        observation.trace_hash.clone(),
        u8::from(observation.quiescent).to_string(),
    ]);
    writeln!(csv, "{}", row.join(",")).unwrap();
}

fn write_checksums(output: &Path) {
    let mut sums = String::new();
    for name in ["gate_a.csv", "decay.csv", "gate_b.csv", "report.md"] {
        let bytes = fs::read(output.join(name)).unwrap();
        writeln!(sums, "{}  {name}", ContentHash::of(&bytes)).unwrap();
    }
    fs::write(output.join("SHA256SUMS"), sums).unwrap();
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/tc_ds1_continuous_participation_v1"));
    fs::create_dir_all(&output).unwrap();
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];

    let mut gate_a = String::from(
        "case_id,root,phase,control,mechanics,a_level,b_level,a_contacts,b_contacts,a_updates,b_updates,causal_work,final_tick,pressure_phase,body_hash,trace_hash,quiescent\n",
    );
    let mut gate_a_cases = 0_usize;
    for root in ROOTS {
        for phase in 0..10 {
            let mut a_only = None;
            let mut b_only = None;
            for control in Control::ALL {
                gate_a_cases += 1;
                let reference = run_gate_a(root, phase, control, mechanics[0]);
                let production = run_gate_a(root, phase, control, mechanics[1]);
                assert_eq!(production, reference);
                assert!(reference.quiescent);
                match control {
                    Control::AOnly => {
                        assert!(reference.a_level > 0 && reference.b_level == 0);
                        a_only = Some(reference.a_level);
                    }
                    Control::BOnly => {
                        assert!(reference.b_level > 0 && reference.a_level == 0);
                        b_only = Some(reference.b_level);
                    }
                    Control::AThenB => {
                        assert!(reference.a_level > 0 && reference.b_level > reference.a_level);
                    }
                    Control::SubthresholdSource | Control::NearbyPath => {
                        assert_eq!((reference.a_level, reference.b_level), (0, 0));
                    }
                    Control::RepeatA => {
                        assert!(reference.a_level > a_only.expect("A-only row precedes repeat"));
                        assert_eq!(reference.b_level, 0);
                    }
                    Control::EqualActivityB => {
                        assert!(reference.b_level > b_only.expect("B-only row precedes repeat"));
                        assert_eq!(reference.a_level, 0);
                    }
                    Control::SharedFanout => {
                        assert!(reference.a_level > 0);
                        assert_eq!(reference.a_level, reference.b_level);
                    }
                }
                for (kind, observation) in [
                    (mechanics[0], &reference),
                    (mechanics[1], &production),
                ] {
                    write_observation(
                        &mut gate_a,
                        &[
                            gate_a_cases.to_string(),
                            root.to_string(),
                            phase.to_string(),
                            control.name().to_owned(),
                        ],
                        kind,
                        observation,
                    );
                }
            }
        }
    }
    assert_eq!(gate_a_cases, EXPECTED_GATE_A_CASES);

    let mut decay = String::from(
        "case_id,root,phase,delay,mechanics,a_level,b_level,a_contacts,b_contacts,a_updates,b_updates,causal_work,final_tick,pressure_phase,body_hash,trace_hash,quiescent\n",
    );
    let mut decay_cases = 0_usize;
    let mut baseline_curve: Option<Vec<u64>> = None;
    for root in ROOTS {
        for phase in 0..10 {
            let mut curve = Vec::new();
            for delay in 0..=20 {
                decay_cases += 1;
                let reference = run_decay(root, phase, delay, mechanics[0]);
                let production = run_decay(root, phase, delay, mechanics[1]);
                assert_eq!(production, reference);
                assert!(reference.a_level > 0 && reference.b_level == 0);
                curve.push(reference.a_level);
                for (kind, observation) in [
                    (mechanics[0], &reference),
                    (mechanics[1], &production),
                ] {
                    write_observation(
                        &mut decay,
                        &[
                            decay_cases.to_string(),
                            root.to_string(),
                            phase.to_string(),
                            delay.to_string(),
                        ],
                        kind,
                        observation,
                    );
                }
            }
            assert!(curve.windows(2).all(|pair| pair[0] > pair[1]));
            assert!(curve.iter().copied().collect::<std::collections::BTreeSet<_>>().len() >= 3);
            if let Some(baseline) = &baseline_curve {
                assert_eq!(&curve, baseline);
            } else {
                baseline_curve = Some(curve);
            }
        }
    }
    assert_eq!(decay_cases, EXPECTED_DECAY_CASES);

    let mut gate_b = String::from(
        "case_id,root,phase,delay,mechanics,a_level,b_level,a_contacts,b_contacts,a_updates,b_updates,causal_work,final_tick,pressure_phase,body_hash,trace_hash,quiescent\n",
    );
    let mut gate_b_cases = 0_usize;
    let mut only_a_contact = 0_usize;
    let mut both_contact = 0_usize;
    for root in ROOTS {
        for phase in 0..10 {
            for delay in MODULATION_DELAYS {
                gate_b_cases += 1;
                let reference = run_gate_b(root, phase, delay, mechanics[0]);
                let production = run_gate_b(root, phase, delay, mechanics[1]);
                assert_eq!(production, reference);
                assert!(reference.quiescent);
                only_a_contact += usize::from(
                    !reference.a_contacts.is_empty() && reference.b_contacts.is_empty(),
                );
                both_contact += usize::from(
                    !reference.a_contacts.is_empty() && !reference.b_contacts.is_empty(),
                );
                for (kind, observation) in [
                    (mechanics[0], &reference),
                    (mechanics[1], &production),
                ] {
                    write_observation(
                        &mut gate_b,
                        &[
                            gate_b_cases.to_string(),
                            root.to_string(),
                            phase.to_string(),
                            delay.to_string(),
                        ],
                        kind,
                        observation,
                    );
                }
            }
        }
    }
    assert_eq!(gate_b_cases, EXPECTED_GATE_B_CASES);

    let gate_a_rows = gate_a_cases * mechanics.len();
    let decay_rows = decay_cases * mechanics.len();
    let gate_b_rows = gate_b_cases * mechanics.len();
    let gate_b_pass = only_a_contact == gate_b_cases;
    let report = format!(
        "# TC-DS1 continuous path participation result v1\n\n\
         - Gate A physical cases: `{gate_a_cases}/{EXPECTED_GATE_A_CASES}`\n\
         - Gate A mechanics rows: `{gate_a_rows}/{}`\n\
         - decay samples: `{decay_cases}/{EXPECTED_DECAY_CASES}` physical, `{decay_rows}/{}` mechanics\n\
         - decay curve: strictly graded through delays `0..20`\n\
         - Gate B physical cases: `{gate_b_cases}/{EXPECTED_GATE_B_CASES}`\n\
         - Gate B mechanics rows: `{gate_b_rows}/{}`\n\
         - only A contacted: `{only_a_contact}/{gate_b_cases}`\n\
         - both A and B contacted: `{both_contact}/{gate_b_cases}`\n\
         - Gate B desired discriminator: `{gate_b_pass}`\n\n\
         Gate A establishes path-local graded participation. Gate B is a\n\
         preregistered stop if source-local modulation contacts both paths.\n",
        EXPECTED_GATE_A_CASES * 2,
        EXPECTED_DECAY_CASES * 2,
        EXPECTED_GATE_B_CASES * 2,
    );
    fs::write(output.join("gate_a.csv"), gate_a).unwrap();
    fs::write(output.join("decay.csv"), decay).unwrap();
    fs::write(output.join("gate_b.csv"), gate_b).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!(
        "TC_DS1_COMPLETE gate_a=pass decay=graded gate_b={} cases={}",
        if gate_b_pass { "pass" } else { "negative" },
        gate_a_cases + decay_cases + gate_b_cases
    );
}
