#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, ContentHash, MechanicalConfig, PhysicalTransition,
    PlasticSubstrate, SpikeInput, TransmissionMode,
};

const ROOTS: [u64; 2] = [1_500_000, 1_600_000];
const CURVE_DELAYS: [i64; 22] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1024,
];
const EXPECTED_CURVE_CASES: usize = 440;
const EXPECTED_CONTROL_CASES: usize = 180;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Control {
    PromptA,
    ReturnB,
    UnrelatedActivity,
    RepeatA,
    RepeatBWhileAWaits,
    SourceWithoutTraversal,
    SameTotalWrongPath,
    ContactFanout,
    LateReturn,
}

impl Control {
    const ALL: [Self; 9] = [
        Self::PromptA,
        Self::ReturnB,
        Self::UnrelatedActivity,
        Self::RepeatA,
        Self::RepeatBWhileAWaits,
        Self::SourceWithoutTraversal,
        Self::SameTotalWrongPath,
        Self::ContactFanout,
        Self::LateReturn,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::PromptA => "prompt_a",
            Self::ReturnB => "return_b",
            Self::UnrelatedActivity => "unrelated_activity",
            Self::RepeatA => "repeat_a",
            Self::RepeatBWhileAWaits => "repeat_b_while_a_waits",
            Self::SourceWithoutTraversal => "source_without_traversal",
            Self::SameTotalWrongPath => "same_total_wrong_path",
            Self::ContactFanout => "contact_fanout",
            Self::LateReturn => "late_return",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<PhysicalTransition>,
    a_participation: u64,
    b_participation: u64,
    a_support: u64,
    b_support: u64,
    physical_work: u64,
    final_tick: i64,
    pressure_phase: i64,
    body_hash: String,
    a_live: bool,
    b_live: bool,
    quiescent: bool,
}

struct Geometry {
    body: PlasticSubstrate,
    p: truelearner_core::CellId,
    ca: truelearner_core::CellId,
    cb: truelearner_core::CellId,
    x: truelearner_core::CellId,
    y: truelearner_core::CellId,
    unrelated_source: truelearner_core::CellId,
    a: ArrowId,
    b: ArrowId,
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
        resistance: 100_000,
    })
}

fn add_arrow(
    body: &mut PlasticSubstrate,
    from: truelearner_core::CellId,
    to: truelearner_core::CellId,
    mode: TransmissionMode,
) -> ArrowId {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: 100_000,
        mode,
    })
}

fn geometry(
    root: u64,
    phase: i64,
    mechanics: MechanicalConfig,
    control: Option<Control>,
    return_b: bool,
) -> Geometry {
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root + 20), 16, 32, mechanics);
    body.set_physical_tracing(true);
    let p = add_cell(
        &mut body,
        root + 1,
        0,
        if control == Some(Control::SourceWithoutTraversal) {
            2
        } else {
            1
        },
    );
    let ca = add_cell(&mut body, root + 2, 10, 1);
    let cb = add_cell(&mut body, root + 3, 20, 1);
    let x = add_cell(&mut body, root + 4, 30, 2);
    let y = add_cell(&mut body, root + 5, 40, 2);
    let unrelated_source = add_cell(&mut body, root + 6, 100, 1);
    let unrelated_target = add_cell(&mut body, root + 7, 110, 2);
    add_arrow(&mut body, p, ca, TransmissionMode::Drive);
    if control != Some(Control::ContactFanout) {
        add_arrow(&mut body, p, cb, TransmissionMode::Drive);
    }
    let a = add_arrow(&mut body, ca, x, TransmissionMode::Drive);
    let b = if control == Some(Control::ContactFanout) {
        add_arrow(&mut body, ca, y, TransmissionMode::Drive)
    } else {
        add_arrow(&mut body, cb, y, TransmissionMode::Drive)
    };
    add_arrow(
        &mut body,
        if return_b { y } else { x },
        if return_b { cb } else { ca },
        TransmissionMode::Modulatory,
    );
    add_arrow(
        &mut body,
        unrelated_source,
        unrelated_target,
        TransmissionMode::Drive,
    );
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
        unrelated_source,
        a,
        b,
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

fn advance(geometry: &mut Geometry, tick: i64, physical_work: &mut u64) {
    let work = geometry.body.advance_time(tick);
    *physical_work = physical_work.saturating_add(work.physical_total());
}

fn observe(geometry: Geometry, trace: Vec<PhysicalTransition>, physical_work: u64) -> Observation {
    let body = geometry.body.arena_body(1);
    let a_live = body
        .arrows
        .iter()
        .find(|arrow| arrow.id == geometry.a)
        .is_some_and(|arrow| arrow.live);
    let b_live = body
        .arrows
        .iter()
        .find(|arrow| arrow.id == geometry.b)
        .is_some_and(|arrow| arrow.live);
    Observation {
        a_participation: geometry.body.local_participation(geometry.a),
        b_participation: geometry.body.local_participation(geometry.b),
        a_support: geometry.body.local_plastic_support(geometry.a),
        b_support: geometry.body.local_plastic_support(geometry.b),
        physical_work,
        final_tick: geometry.body.clock().tick,
        pressure_phase: geometry.body.clock().pressure_phase(),
        body_hash: ContentHash::of(&geometry.body.canonical_body_bytes(1).unwrap()).to_string(),
        a_live,
        b_live,
        quiescent: true,
        trace,
    }
}

fn run_curve(root: u64, phase: i64, delay: i64, mechanics: MechanicalConfig) -> Observation {
    let mut geometry = geometry(root, phase, mechanics, None, false);
    let mut trace = Vec::new();
    let mut physical_work = 0_u64;
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
        &[input(
            x,
            phase + delay,
            root + 102,
            if delay == 0 { 1 } else { 2 },
        )],
        &mut trace,
        &mut physical_work,
    );
    observe(geometry, trace, physical_work)
}

fn run_control(
    root: u64,
    phase: i64,
    control: Control,
    mechanics: MechanicalConfig,
) -> Observation {
    if control == Control::PromptA {
        return run_curve(root, phase, 0, mechanics);
    }
    if control == Control::LateReturn {
        return run_curve(root, phase, 1024, mechanics);
    }
    let mut geometry = geometry(
        root,
        phase,
        mechanics,
        Some(control),
        control == Control::ReturnB,
    );
    let mut trace = Vec::new();
    let mut physical_work = 0_u64;
    match control {
        Control::ReturnB | Control::ContactFanout => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 201, 1)],
                &mut trace,
                &mut physical_work,
            );
            let target = if control == Control::ReturnB {
                geometry.y
            } else {
                geometry.x
            };
            admit(
                &mut geometry,
                &[input(target, phase, root + 202, 1)],
                &mut trace,
                &mut physical_work,
            );
        }
        Control::UnrelatedActivity => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 203, 1)],
                &mut trace,
                &mut physical_work,
            );
            let unrelated = geometry.unrelated_source;
            admit(
                &mut geometry,
                &[input(unrelated, phase + 5, root + 204, 1)],
                &mut trace,
                &mut physical_work,
            );
            advance(&mut geometry, phase + 10, &mut physical_work);
        }
        Control::RepeatA => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 205, 1)],
                &mut trace,
                &mut physical_work,
            );
            let ca = geometry.ca;
            admit(
                &mut geometry,
                &[input(ca, phase + 5, root + 206, 1)],
                &mut trace,
                &mut physical_work,
            );
            advance(&mut geometry, phase + 10, &mut physical_work);
        }
        Control::RepeatBWhileAWaits => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 207, 1)],
                &mut trace,
                &mut physical_work,
            );
            let cb = geometry.cb;
            admit(
                &mut geometry,
                &[input(cb, phase + 5, root + 208, 1)],
                &mut trace,
                &mut physical_work,
            );
            advance(&mut geometry, phase + 10, &mut physical_work);
        }
        Control::SourceWithoutTraversal => {
            let p = geometry.p;
            admit(
                &mut geometry,
                &[input(p, phase, root + 209, 1)],
                &mut trace,
                &mut physical_work,
            );
            advance(&mut geometry, phase + 10, &mut physical_work);
        }
        Control::SameTotalWrongPath => {
            let cb = geometry.cb;
            admit(
                &mut geometry,
                &[input(cb, phase, root + 210, 1)],
                &mut trace,
                &mut physical_work,
            );
            admit(
                &mut geometry,
                &[input(cb, phase + 5, root + 211, 1)],
                &mut trace,
                &mut physical_work,
            );
            advance(&mut geometry, phase + 10, &mut physical_work);
        }
        Control::PromptA | Control::LateReturn => unreachable!(),
    }
    observe(geometry, trace, physical_work)
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
    prefix: &[String],
    mechanics: MechanicalConfig,
    observation: &Observation,
) {
    let mut fields = prefix.to_vec();
    fields.extend([
        mechanics_name(mechanics).to_owned(),
        observation.a_participation.to_string(),
        observation.b_participation.to_string(),
        observation.a_support.to_string(),
        observation.b_support.to_string(),
        observation.physical_work.to_string(),
        observation.final_tick.to_string(),
        observation.pressure_phase.to_string(),
        ContentHash::of(format!("{:?}", observation.trace).as_bytes()).to_string(),
        observation.body_hash.clone(),
        u8::from(observation.a_live).to_string(),
        u8::from(observation.b_live).to_string(),
        u8::from(observation.quiescent).to_string(),
    ]);
    writeln!(csv, "{}", fields.join(",")).unwrap();
}

fn write_checksums(output: &Path) {
    let mut sums = String::new();
    for name in ["curve.csv", "controls.csv", "report.md"] {
        let bytes = fs::read(output.join(name)).unwrap();
        writeln!(sums, "{}  {name}", ContentHash::of(&bytes)).unwrap();
    }
    fs::write(output.join("SHA256SUMS"), sums).unwrap();
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/cpc1_local_temporal_participation_v1"));
    fs::create_dir_all(&output).unwrap();
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];
    let mut curve_csv = String::from(
        "case_id,root,initial_phase,delay,mechanics,a_participation,b_participation,a_support,b_support,physical_work,final_tick,pressure_phase,trace_hash,body_hash,a_live,b_live,quiescent\n",
    );
    let mut control_csv = String::from(
        "case_id,root,initial_phase,control,mechanics,a_participation,b_participation,a_support,b_support,physical_work,final_tick,pressure_phase,trace_hash,body_hash,a_live,b_live,quiescent\n",
    );
    let mut curve_cases = 0_usize;
    let mut control_cases = 0_usize;
    let mut baseline_curve: Option<Vec<u64>> = None;
    for root in ROOTS {
        for phase in 0..10 {
            let mut relative_curve = Vec::new();
            for delay in CURVE_DELAYS {
                curve_cases += 1;
                let reference = run_curve(root, phase, delay, mechanics[0]);
                let reference_replay = run_curve(root, phase, delay, mechanics[0]);
                assert_eq!(reference_replay, reference);
                let production = run_curve(root, phase, delay, mechanics[1]);
                let production_replay = run_curve(root, phase, delay, mechanics[1]);
                assert_eq!(production_replay, production);
                assert_eq!(production, reference);
                assert!(reference.a_live && reference.b_live && reference.quiescent);
                assert_eq!(reference.a_support, reference.a_participation);
                assert_eq!(reference.b_support, 0);
                if delay <= 20 {
                    assert!(reference.a_participation > 0);
                    relative_curve.push(reference.a_participation);
                } else {
                    assert_eq!(delay, 1024);
                    assert_eq!((reference.a_participation, reference.a_support), (0, 0));
                }
                for (kind, observation) in [(mechanics[0], &reference), (mechanics[1], &production)]
                {
                    write_observation(
                        &mut curve_csv,
                        &[
                            curve_cases.to_string(),
                            root.to_string(),
                            phase.to_string(),
                            delay.to_string(),
                        ],
                        kind,
                        observation,
                    );
                }
            }
            assert!(relative_curve.windows(2).all(|pair| pair[0] > pair[1]));
            assert!(
                relative_curve
                    .iter()
                    .copied()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
                    >= 3
            );
            if let Some(baseline) = &baseline_curve {
                assert_eq!(&relative_curve, baseline);
            } else {
                baseline_curve = Some(relative_curve.clone());
            }
            let single_at_10 = relative_curve[10];
            for control in Control::ALL {
                control_cases += 1;
                let reference = run_control(root, phase, control, mechanics[0]);
                let reference_replay = run_control(root, phase, control, mechanics[0]);
                assert_eq!(reference_replay, reference);
                let production = run_control(root, phase, control, mechanics[1]);
                let production_replay = run_control(root, phase, control, mechanics[1]);
                assert_eq!(production_replay, production);
                assert_eq!(production, reference);
                assert!(reference.a_live && reference.b_live && reference.quiescent);
                match control {
                    Control::PromptA => {
                        assert!(reference.a_support > 0);
                        assert_eq!(reference.b_support, 0);
                    }
                    Control::ReturnB => {
                        assert_eq!(reference.a_support, 0);
                        assert!(reference.b_support > 0);
                    }
                    Control::UnrelatedActivity => {
                        assert_eq!(reference.a_participation, single_at_10);
                        assert_eq!(reference.a_support, 0);
                    }
                    Control::RepeatA => {
                        assert!(reference.a_participation > single_at_10);
                        assert_eq!(reference.b_participation, single_at_10);
                    }
                    Control::RepeatBWhileAWaits => {
                        assert_eq!(reference.a_participation, single_at_10);
                        assert!(reference.b_participation > single_at_10);
                    }
                    Control::SourceWithoutTraversal => {
                        assert_eq!(
                            (reference.a_participation, reference.b_participation),
                            (0, 0)
                        );
                    }
                    Control::SameTotalWrongPath => {
                        assert_eq!(reference.a_participation, 0);
                        assert!(reference.b_participation > 0);
                    }
                    Control::ContactFanout => {
                        assert!(reference.a_support > 0);
                        assert_eq!(reference.a_support, reference.b_support);
                    }
                    Control::LateReturn => {
                        assert_eq!((reference.a_support, reference.b_support), (0, 0));
                    }
                }
                for (kind, observation) in [(mechanics[0], &reference), (mechanics[1], &production)]
                {
                    write_observation(
                        &mut control_csv,
                        &[
                            control_cases.to_string(),
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
    assert_eq!(curve_cases, EXPECTED_CURVE_CASES);
    assert_eq!(control_cases, EXPECTED_CONTROL_CASES);
    let report = format!(
        "# CPC1 local continuous temporal participation result v1\n\n\
         - curve physical cases: `{curve_cases}/{EXPECTED_CURVE_CASES}`\n\
         - curve mechanics rows: `{}/{}`\n\
         - control physical cases: `{control_cases}/{EXPECTED_CONTROL_CASES}`\n\
         - control mechanics rows: `{}/{}`\n\
         - total physical cases: `{}/{}`\n\
         - total mechanics rows: `{}/{}`\n\
         - exact same-mechanics fresh replay: `{}/{}` runs\n\
         - exact ordered Reference/Production retained histories: `{}/{}`\n\
         - graded positive curve: delays `0..20`\n\
         - naturally relaxed zero: delay `1024`\n\
         - local renewal and wrong-path controls: `pass`\n\
         - contact fan-out granularity: `pass`\n\
         - pressure or durable-resistance candidate interaction: `none`\n\n\
         Candidate plastic support is the arithmetic intersection of local\n\
         participation remaining at the contact and Modulatory arrival.\n",
        EXPECTED_CURVE_CASES * 2,
        EXPECTED_CURVE_CASES * 2,
        EXPECTED_CONTROL_CASES * 2,
        EXPECTED_CONTROL_CASES * 2,
        EXPECTED_CURVE_CASES + EXPECTED_CONTROL_CASES,
        EXPECTED_CURVE_CASES + EXPECTED_CONTROL_CASES,
        (EXPECTED_CURVE_CASES + EXPECTED_CONTROL_CASES) * 2,
        (EXPECTED_CURVE_CASES + EXPECTED_CONTROL_CASES) * 2,
        (EXPECTED_CURVE_CASES + EXPECTED_CONTROL_CASES) * 4,
        (EXPECTED_CURVE_CASES + EXPECTED_CONTROL_CASES) * 4,
        EXPECTED_CURVE_CASES + EXPECTED_CONTROL_CASES,
        EXPECTED_CURVE_CASES + EXPECTED_CONTROL_CASES,
    );
    fs::write(output.join("curve.csv"), curve_csv).unwrap();
    fs::write(output.join("controls.csv"), control_csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!(
        "CPC1_COMPLETE curve={curve_cases} controls={control_cases} total={}",
        curve_cases + control_cases
    );
}
