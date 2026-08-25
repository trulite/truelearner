#![forbid(unsafe_code)]

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, Core0Profile, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode,
};

const HIGH_RESISTANCE: u32 = 10_000;
const MATERIAL_ONE: i64 = 1_i64 << 32;
const RB0_BOUNDARY: i64 = MATERIAL_ONE * 5 / 2;
const PROBE_COUNTS: [u32; 7] = [0, 1, 2, 4, 8, 16, 32];
const PROFILES: [(Core0Profile, &str); 3] = [
    (Core0Profile::B, "CORE1-A"),
    (Core0Profile::GenericExternal, "CORE1-B"),
    (Core0Profile::GenericActivity, "CORE1-C"),
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct DiagnosticRow {
    kind: &'static str,
    step: u32,
    route: u8,
    exists_before: bool,
    exists_after: bool,
    traversed: bool,
    participation_after_traversal: u64,
    participation_before_consequence: u64,
    modulation_at_contact: bool,
    coupling_before: i64,
    coupling_after: i64,
    resistance_before: u64,
    resistance_after: u64,
    qlp_traversals: u64,
    inhibitor_reexecutes: bool,
    rb0_boundary_reached: bool,
    fires_a: usize,
    fires_b: usize,
    settled: bool,
    ceiling_reached: bool,
    quiescent: bool,
    work: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Sample {
    row: DiagnosticRow,
    trace: Vec<PhysicalTransition>,
}

struct E12Body {
    body: PlasticSubstrate,
    a: CellId,
    b: CellId,
    ia: CellId,
    ib: CellId,
    unrelated: CellId,
    modulator: CellId,
    negative: [ArrowId; 2],
}

fn body(root: u64, profile: Core0Profile, mechanics: MechanicalConfig) -> PlasticSubstrate {
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 512, 2048, mechanics);
    body.set_core0_profile(profile);
    body.set_physical_tracing(true);
    body
}

fn cell(body: &mut PlasticSubstrate, physical: u64, position: i32, threshold: i32) -> CellId {
    body.add_cell(CellSpec {
        physical_id: physical,
        position,
        region: 0,
        threshold,
        resistance: HIGH_RESISTANCE,
    })
}

fn arrow(
    body: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    coupling: i32,
    delay: i64,
    mode: TransmissionMode,
) -> ArrowId {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: HIGH_RESISTANCE,
        mode,
    })
}

fn pulse_many(
    body: &mut PlasticSubstrate,
    targets: &[CellId],
    tick: i64,
    origin: u64,
) -> truelearner_core::RunResult {
    let inputs = targets
        .iter()
        .enumerate()
        .map(|(index, target)| SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: origin.saturating_add(u64::try_from(index).unwrap_or(u64::MAX)),
            target: *target,
            impulse: 1,
        })
        .collect::<Vec<_>>();
    body.arrive(&inputs, i16::MAX)
}

fn pulse(
    body: &mut PlasticSubstrate,
    target: CellId,
    tick: i64,
    origin: u64,
) -> truelearner_core::RunResult {
    body.arrive(
        &[SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: origin,
            target,
            impulse: 1,
        }],
        i16::MAX,
    )
}

fn e12_body(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> E12Body {
    let mut body = body(root, profile, mechanics);
    let a = cell(&mut body, root + 1, 0, 100);
    let b = cell(&mut body, root + 2, 100, 100);
    let ia = cell(&mut body, root + 3, 10_000, 1);
    let ib = cell(&mut body, root + 4, 20_000, 1);
    let pa = cell(&mut body, root + 20, 30_000, 1);
    let pb = cell(&mut body, root + 30, 40_000, 1);
    let unrelated = cell(&mut body, root + 50, 50_000, 1);
    let modulator = cell(&mut body, root + 60, 60_000, 1);
    let negative = [
        arrow(&mut body, ia, a, -2, 0, TransmissionMode::Drive),
        arrow(&mut body, ib, b, -2, 0, TransmissionMode::Drive),
    ];
    arrow(&mut body, unrelated, a, -2, 0, TransmissionMode::Drive);
    arrow(&mut body, pa, a, 2, 0, TransmissionMode::Drive);
    arrow(&mut body, pb, b, 2, 0, TransmissionMode::Drive);
    arrow(&mut body, modulator, ia, 2, 0, TransmissionMode::Modulatory);
    arrow(&mut body, modulator, ib, 2, 0, TransmissionMode::Modulatory);
    E12Body {
        body,
        a,
        b,
        ia,
        ib,
        unrelated,
        modulator,
        negative,
    }
}

fn fires(trace: &[PhysicalTransition], cell: CellId) -> usize {
    trace
        .iter()
        .filter(|entry| matches!(entry.event, PhysicalEvent::Fire { cell: id } if id == cell))
        .count()
}

fn modulation_at(trace: &[PhysicalTransition], cell: CellId) -> bool {
    trace.iter().any(|entry| {
        matches!(
            entry.event,
            PhysicalEvent::ModulatoryIncidence { target, .. } if target == cell
        )
    })
}

fn qlp_count(trace: &[PhysicalTransition]) -> u64 {
    u64::try_from(
        trace
            .iter()
            .filter(|entry| matches!(entry.event, PhysicalEvent::QualifiedLocalTraversal { .. }))
            .count(),
    )
    .unwrap_or(u64::MAX)
}

fn negative_incidence(trace: &[PhysicalTransition], target: CellId) -> bool {
    trace.iter().any(|entry| {
        matches!(
            entry.event,
            PhysicalEvent::MaterialDriveIncidence { target: observed, impulse, .. }
                if observed == target && impulse < 0
        )
    })
}

fn live(body: &PlasticSubstrate, arrow: ArrowId) -> bool {
    body.arena_body(1)
        .arrows
        .iter()
        .any(|candidate| candidate.id == arrow && candidate.live)
}

fn probe(world: &E12Body, step: u32) -> (Vec<Sample>, Vec<PhysicalTransition>) {
    let mut body = world.body.clone();
    for (from, to, coupling) in [
        (world.a, world.b, 2),
        (world.b, world.a, 2),
        (world.a, world.ia, 1),
        (world.b, world.ib, 1),
    ] {
        arrow(
            &mut body,
            from,
            to,
            coupling,
            if coupling == 2 { 1 } else { 0 },
            TransmissionMode::Drive,
        );
    }
    body.enter(SpikeInput {
        arrival_tick: body.clock().tick.saturating_add(1),
        phase: 0,
        origin_physical: 98_000_000,
        target: world.a,
        impulse: 2,
    });
    let first = body.propagate_with_observation_ceiling(256);
    let mut trace = first.run.physical_trace.clone();
    let mut work = first.run.work.physical_total();
    let mut quiescent = first.run.naturally_quiescent;
    let mut ceiling_reached = first.observation_ceiling_reached;
    if ceiling_reached {
        let continuation = body.propagate_with_observation_ceiling(32);
        trace.extend(continuation.run.physical_trace);
        work = work.saturating_add(continuation.run.work.physical_total());
        quiescent = continuation.run.naturally_quiescent;
        ceiling_reached |= continuation.observation_ceiling_reached;
    }
    let fires_a = fires(&trace, world.a);
    let fires_b = fires(&trace, world.b);
    let settled = quiescent && fires_a == 1 && fires_b == 1;
    let mut samples = Vec::new();
    for route in 0..2_usize {
        let id = world.negative[route];
        let contact = if route == 0 { world.ia } else { world.ib };
        let target = if route == 0 { world.a } else { world.b };
        let coupling = world.body.core0_coupling_material(id);
        samples.push(Sample {
            row: DiagnosticRow {
                kind: "probe",
                step,
                route: u8::try_from(route).unwrap_or(u8::MAX),
                exists_before: live(&world.body, id),
                exists_after: live(&body, id),
                traversed: false,
                participation_after_traversal: world.body.local_participation(id),
                participation_before_consequence: 0,
                modulation_at_contact: false,
                coupling_before: coupling,
                coupling_after: coupling,
                resistance_before: world.body.core0_resistance_material(id),
                resistance_after: world.body.core0_resistance_material(id),
                qlp_traversals: qlp_count(&trace),
                inhibitor_reexecutes: fires(&trace, contact) > 0
                    && negative_incidence(&trace, target),
                rb0_boundary_reached: coupling.saturating_abs() >= RB0_BOUNDARY,
                fires_a,
                fires_b,
                settled,
                ceiling_reached,
                quiescent,
                work,
            },
            trace: trace.clone(),
        });
    }
    (samples, trace)
}

fn train_once(world: &mut E12Body, experience: u32, root: u64) -> Vec<Sample> {
    let exists_before = world.negative.map(|id| live(&world.body, id));
    let start = world.body.clock().tick.saturating_add(1);
    let traversal = pulse_many(
        &mut world.body,
        &[world.ia, world.ib, world.unrelated],
        start,
        root + 10_000 + u64::from(experience) * 1_000,
    );
    let participation_after = world.negative.map(|id| world.body.local_participation(id));
    let consequence_tick = start.saturating_add(12);
    let mut aged = world.body.clone();
    aged.advance_time(consequence_tick);
    let participation_at_consequence = world.negative.map(|id| aged.local_participation(id));
    let coupling_pre_consequence = world
        .negative
        .map(|id| world.body.core0_coupling_material(id));
    let resistance_pre_consequence = world
        .negative
        .map(|id| world.body.core0_resistance_material(id));
    let closure = pulse(
        &mut world.body,
        world.modulator,
        consequence_tick,
        root + 20_000 + u64::from(experience) * 1_000,
    );
    let mut combined_trace = traversal.physical_trace.clone();
    combined_trace.extend(closure.physical_trace.clone());
    let mut samples = Vec::new();
    for route in 0..2_usize {
        let id = world.negative[route];
        let contact = if route == 0 { world.ia } else { world.ib };
        samples.push(Sample {
            row: DiagnosticRow {
                kind: "training",
                step: experience,
                route: u8::try_from(route).unwrap_or(u8::MAX),
                exists_before: exists_before[route],
                exists_after: live(&world.body, id),
                traversed: fires(&traversal.physical_trace, contact) > 0
                    && participation_after[route] > 0,
                participation_after_traversal: participation_after[route],
                participation_before_consequence: participation_at_consequence[route],
                modulation_at_contact: modulation_at(&closure.physical_trace, contact),
                coupling_before: coupling_pre_consequence[route],
                coupling_after: world.body.core0_coupling_material(id),
                resistance_before: resistance_pre_consequence[route],
                resistance_after: world.body.core0_resistance_material(id),
                qlp_traversals: qlp_count(&closure.physical_trace),
                inhibitor_reexecutes: false,
                rb0_boundary_reached: world.body.core0_coupling_material(id).saturating_abs()
                    >= RB0_BOUNDARY,
                fires_a: 0,
                fires_b: 0,
                settled: false,
                ceiling_reached: false,
                quiescent: traversal.naturally_quiescent && closure.naturally_quiescent,
                work: traversal
                    .work
                    .physical_total()
                    .saturating_add(closure.work.physical_total()),
            },
            trace: combined_trace.clone(),
        });
    }
    samples
}

fn csv_header() -> &'static str {
    "profile,mechanics,kind,step,route,exists_before,exists_after,traversed,participation_after_traversal,participation_before_consequence,modulation_at_contact,coupling_before,coupling_after,resistance_before,resistance_after,qlp_traversals,inhibitor_reexecutes,rb0_boundary_reached,fires_a,fires_b,settled,ceiling_reached,quiescent,physical_work\n"
}

fn write_sample(output: &mut BufWriter<File>, profile: &str, mechanics: &str, sample: &Sample) {
    let row = &sample.row;
    writeln!(
        output,
        "{profile},{mechanics},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        row.kind,
        row.step,
        row.route,
        row.exists_before,
        row.exists_after,
        row.traversed,
        row.participation_after_traversal,
        row.participation_before_consequence,
        row.modulation_at_contact,
        row.coupling_before,
        row.coupling_after,
        row.resistance_before,
        row.resistance_after,
        row.qlp_traversals,
        row.inhibitor_reexecutes,
        row.rb0_boundary_reached,
        row.fires_a,
        row.fires_b,
        row.settled,
        row.ceiling_reached,
        row.quiescent,
        row.work
    )
    .expect("write diagnostic row");
    output.flush().expect("flush diagnostic row");
}

fn run_profile(
    profile: Core0Profile,
    profile_name: &str,
    mechanics: MechanicalConfig,
    mechanics_name: &str,
    root: u64,
    output: &mut BufWriter<File>,
) -> Vec<Sample> {
    let mut world = e12_body(profile, mechanics, root);
    let mut samples = Vec::new();
    let (initial, _) = probe(&world, 0);
    for sample in initial {
        write_sample(output, profile_name, mechanics_name, &sample);
        samples.push(sample);
    }
    for experience in 1..=32_u32 {
        for sample in train_once(&mut world, experience, root) {
            write_sample(output, profile_name, mechanics_name, &sample);
            samples.push(sample);
        }
        if PROBE_COUNTS.contains(&experience) {
            let (probe_samples, _) = probe(&world, experience);
            for sample in probe_samples {
                write_sample(output, profile_name, mechanics_name, &sample);
                samples.push(sample);
            }
        }
    }
    samples
}

fn classify(samples: &[Sample]) -> (&'static str, String) {
    let training = samples
        .iter()
        .filter(|sample| sample.row.kind == "training")
        .collect::<Vec<_>>();
    let probes = samples
        .iter()
        .filter(|sample| sample.row.kind == "probe")
        .collect::<Vec<_>>();
    if training.iter().any(|sample| !sample.row.exists_before) {
        return ("A", "inhibitory topology is absent".to_string());
    }
    if training.iter().any(|sample| !sample.row.traversed) {
        return ("A", "inhibitory route does not traverse".to_string());
    }
    if training
        .iter()
        .any(|sample| sample.row.participation_before_consequence == 0)
    {
        return ("B", "participation is gone before consequence".to_string());
    }
    if training
        .iter()
        .any(|sample| !sample.row.modulation_at_contact)
    {
        return ("C", "Modulation does not reach the contact".to_string());
    }
    if training
        .iter()
        .all(|sample| sample.row.coupling_after == sample.row.coupling_before)
    {
        return ("C", "efficacy never changes".to_string());
    }
    if probes.iter().all(|sample| !sample.row.rb0_boundary_reached) {
        return ("D", "efficacy never reaches the RB0 boundary".to_string());
    }
    if probes
        .iter()
        .filter(|sample| sample.row.rb0_boundary_reached)
        .any(|sample| !sample.row.inhibitor_reexecutes || !sample.row.settled)
    {
        return (
            "E",
            "boundary-strength inhibitor fails to execute or settle".to_string(),
        );
    }
    ("POSITIVE", "complete E12 causal chain observed".to_string())
}

fn append_summary(
    destination: &Path,
    profile: &str,
    class: &str,
    reason: &str,
    replay_exact: bool,
    mechanics_exact: bool,
    samples: &[Sample],
) {
    let mut output = OpenOptions::new()
        .create(true)
        .append(true)
        .open(destination.join("summary.md"))
        .expect("open diagnostic summary");
    let first_boundary = samples
        .iter()
        .find(|sample| sample.row.kind == "probe" && sample.row.rb0_boundary_reached)
        .map(|sample| sample.row.step.to_string())
        .unwrap_or_else(|| "none".to_string());
    let first_settled = samples
        .iter()
        .find(|sample| sample.row.kind == "probe" && sample.row.settled)
        .map(|sample| sample.row.step.to_string())
        .unwrap_or_else(|| "none".to_string());
    writeln!(
        output,
        "| {profile} | {class} | {reason} | {first_boundary} | {first_settled} | {replay_exact} | {mechanics_exact} |"
    )
    .expect("write diagnostic summary");
    output.flush().expect("flush diagnostic summary");
}

fn main() {
    eprintln!("CORE1_E12D_V1_EVIDENCE_SPENT");
    let destination = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_e12d_v1"));
    fs::create_dir_all(&destination).expect("create diagnostic result directory");
    let mut csv =
        BufWriter::new(File::create(destination.join("rows.csv")).expect("create diagnostic CSV"));
    csv.write_all(csv_header().as_bytes())
        .expect("write diagnostic header");
    csv.flush().expect("flush diagnostic header");
    fs::write(
        destination.join("summary.md"),
        "# CORE1-E12D result\n\n| Profile | Class | First broken link | First RB0 boundary | First settled probe | Replay exact | Mechanics exact |\n|---|---|---|---:|---:|---|---|\n",
    )
    .expect("initialize diagnostic summary");

    for (index, (profile, name)) in PROFILES.into_iter().enumerate() {
        let root = 9_000_000 + u64::try_from(index).unwrap_or(0) * 1_000_000;
        let reference = run_profile(
            profile,
            name,
            MechanicalConfig::REFERENCE,
            "reference",
            root,
            &mut csv,
        );
        let replay = run_profile(
            profile,
            name,
            MechanicalConfig::REFERENCE,
            "replay",
            root,
            &mut csv,
        );
        let production = run_profile(
            profile,
            name,
            MechanicalConfig::PRODUCTION,
            "production",
            root,
            &mut csv,
        );
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        let (class, reason) = classify(&reference);
        append_summary(
            &destination,
            name,
            class,
            &reason,
            replay_exact,
            mechanics_exact,
            &reference,
        );
    }
    println!("CORE1_E12D_COMPLETE profiles=3 probe_counts=0|1|2|4|8|16|32");
}
