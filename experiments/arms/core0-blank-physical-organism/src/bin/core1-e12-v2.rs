#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::env;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use academy_arc3::{spatial_context, Arc3Sensorimotor, ARC3_FRAME_PIXELS};
use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, Core0Profile, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode,
};

const HIGH_RESISTANCE: u32 = 10_000;
const MATERIAL_ONE: i64 = 1_i64 << 32;
const PROFILES: [(Core0Profile, &str); 3] = [
    (Core0Profile::B, "CORE1-A"),
    (Core0Profile::GenericExternal, "CORE1-B"),
    (Core0Profile::GenericActivity, "CORE1-C"),
];

#[derive(Clone, Copy)]
enum Consequence {
    Local,
    None,
    Wrong,
}

struct TrainingBody {
    body: PlasticSubstrate,
    ia: CellId,
    ib: CellId,
    unrelated: CellId,
    modulator: CellId,
    negative: [ArrowId; 2],
    unrelated_negative: ArrowId,
    positive: [ArrowId; 2],
}

struct TrainingResult {
    world: TrainingBody,
    coupling_curve: Vec<[i64; 2]>,
    probes: Vec<ProbeRow>,
    trace: Vec<PhysicalTransition>,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProbeRow {
    step: u32,
    coupling: [i64; 2],
    fires_a: usize,
    fires_b: usize,
    fires_ia: usize,
    fires_ib: usize,
    negative_incidences: usize,
    stable: bool,
    first_ceiling: bool,
    continuation_ceiling: bool,
    quiescent: bool,
    work: u64,
    trace: Vec<PhysicalTransition>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct E12Observation {
    pass: bool,
    coupling_curve: Vec<[i64; 2]>,
    probes: Vec<ProbeRow>,
    control_curves: Vec<Vec<[i64; 2]>>,
    primary_trace: Vec<PhysicalTransition>,
    final_coupling: [i64; 2],
    final_resistance: [u64; 2],
    work: u64,
    quiescent: bool,
    reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AcademyObservation {
    pass: bool,
    actions: Vec<Option<u8>>,
    updates: Vec<u64>,
    work: u64,
    tick: i64,
    quiescent: bool,
}

fn substrate(root: u64, profile: Core0Profile, mechanics: MechanicalConfig) -> PlasticSubstrate {
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

fn material_arrow(
    body: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    material: i64,
    delay: i64,
) -> ArrowId {
    let observer = i32::try_from(material / MATERIAL_ONE).unwrap_or_else(|_| {
        if material.is_negative() {
            i32::MIN
        } else {
            i32::MAX
        }
    });
    let id = arrow(body, from, to, observer, delay, TransmissionMode::Drive);
    assert!(body.set_core0_coupling_material(id, material));
    id
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

fn training_body(
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
    permuted: bool,
    consequence: Consequence,
) -> TrainingBody {
    let mut body = substrate(root, profile, mechanics);
    let a = cell(&mut body, root + 1, 0, 100);
    let b = cell(&mut body, root + 2, 100, 100);
    let (ia, ib, pa, pb, unrelated) = if permuted {
        let pb = cell(&mut body, root + 30, 40_000, 1);
        let unrelated = cell(&mut body, root + 50, 50_000, 1);
        let ib = cell(&mut body, root + 4, 20_000, 1);
        let pa = cell(&mut body, root + 20, 30_000, 1);
        let ia = cell(&mut body, root + 3, 10_000, 1);
        (ia, ib, pa, pb, unrelated)
    } else {
        (
            cell(&mut body, root + 3, 10_000, 1),
            cell(&mut body, root + 4, 20_000, 1),
            cell(&mut body, root + 20, 30_000, 1),
            cell(&mut body, root + 30, 40_000, 1),
            cell(&mut body, root + 50, 50_000, 1),
        )
    };
    let modulator = cell(&mut body, root + 60, 60_000, 1);
    let wrong = cell(&mut body, root + 70, 70_000, 100);
    let negative = [
        arrow(&mut body, ia, a, -2, 0, TransmissionMode::Drive),
        arrow(&mut body, ib, b, -2, 0, TransmissionMode::Drive),
    ];
    let unrelated_negative = arrow(&mut body, unrelated, a, -2, 0, TransmissionMode::Drive);
    let positive = [
        arrow(&mut body, pa, a, 2, 0, TransmissionMode::Drive),
        arrow(&mut body, pb, b, 2, 0, TransmissionMode::Drive),
    ];
    if !matches!(consequence, Consequence::None) {
        let targets = if matches!(consequence, Consequence::Wrong) {
            [wrong, wrong]
        } else {
            [ia, ib]
        };
        for target in targets {
            arrow(
                &mut body,
                modulator,
                target,
                2,
                0,
                TransmissionMode::Modulatory,
            );
        }
    }
    TrainingBody {
        body,
        ia,
        ib,
        unrelated,
        modulator,
        negative,
        unrelated_negative,
        positive,
    }
}

fn fire_count(trace: &[PhysicalTransition], cell: CellId) -> usize {
    trace
        .iter()
        .filter(|entry| matches!(entry.event, PhysicalEvent::Fire { cell: id } if id == cell))
        .count()
}

fn probe(
    world: &TrainingBody,
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
    step: u32,
) -> ProbeRow {
    let mut body = substrate(root, profile, mechanics);
    let a = cell(&mut body, root + 1, 0, 2);
    let b = cell(&mut body, root + 2, 100, 2);
    let ia = cell(&mut body, root + 3, 10_000, 1);
    let ib = cell(&mut body, root + 4, 20_000, 1);
    material_arrow(&mut body, a, b, 2 * MATERIAL_ONE, 1);
    material_arrow(&mut body, b, a, 2 * MATERIAL_ONE, 1);
    material_arrow(&mut body, a, ia, MATERIAL_ONE, 0);
    material_arrow(&mut body, b, ib, MATERIAL_ONE, 0);
    let coupling = world
        .negative
        .map(|id| world.body.core0_coupling_material(id));
    material_arrow(&mut body, ia, a, coupling[0], 0);
    material_arrow(&mut body, ib, b, coupling[1], 0);
    body.enter(SpikeInput {
        arrival_tick: 0,
        phase: 0,
        origin_physical: root + 90,
        target: a,
        impulse: 2,
    });
    let first = body.propagate_with_observation_ceiling(256);
    let first_ceiling = first.observation_ceiling_reached;
    let mut trace = first.run.physical_trace.clone();
    let mut work = first.run.work.physical_total();
    let mut quiescent = first.run.naturally_quiescent;
    let mut continuation_ceiling = false;
    if first_ceiling {
        let continuation = body.propagate_with_observation_ceiling(32);
        continuation_ceiling = continuation.observation_ceiling_reached;
        quiescent = continuation.run.naturally_quiescent;
        work = work.saturating_add(continuation.run.work.physical_total());
        trace.extend(continuation.run.physical_trace);
    }
    let fires_a = fire_count(&trace, a);
    let fires_b = fire_count(&trace, b);
    let fires_ia = fire_count(&trace, ia);
    let fires_ib = fire_count(&trace, ib);
    let negative_incidences = trace
        .iter()
        .filter(|entry| {
            matches!(
                entry.event,
                PhysicalEvent::MaterialDriveIncidence { impulse, .. } if impulse < 0
            )
        })
        .count();
    ProbeRow {
        step,
        coupling,
        fires_a,
        fires_b,
        fires_ia,
        fires_ib,
        negative_incidences,
        stable: quiescent && fires_a == 1 && fires_b == 1,
        first_ceiling,
        continuation_ceiling,
        quiescent,
        work,
        trace,
    }
}

fn train(
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
    permuted: bool,
    consequence: Consequence,
) -> TrainingResult {
    let mut world = training_body(profile, mechanics, root, permuted, consequence);
    let mut curve = vec![world
        .negative
        .map(|id| world.body.core0_coupling_material(id))];
    let mut probes = vec![probe(&world, profile, mechanics, root + 500_000, 0)];
    let mut trace = probes[0].trace.clone();
    let mut work = probes[0].work;
    let mut quiescent = true;
    for experience in 1..=6_u32 {
        let start = world.body.clock().tick.saturating_add(1);
        let traversal = pulse_many(
            &mut world.body,
            &[world.ia, world.ib, world.unrelated],
            start,
            root + 10_000 + u64::from(experience) * 1_000,
        );
        work = work.saturating_add(traversal.work.physical_total());
        quiescent &= traversal.naturally_quiescent;
        trace.extend(traversal.physical_trace);
        if !matches!(consequence, Consequence::None) {
            let closure = pulse(
                &mut world.body,
                world.modulator,
                start.saturating_add(12),
                root + 20_000 + u64::from(experience) * 1_000,
            );
            work = work.saturating_add(closure.work.physical_total());
            quiescent &= closure.naturally_quiescent;
            trace.extend(closure.physical_trace);
        } else {
            world.body.advance_time(start.saturating_add(12));
        }
        curve.push(
            world
                .negative
                .map(|id| world.body.core0_coupling_material(id)),
        );
        let observed = probe(
            &world,
            profile,
            mechanics,
            root + 500_000 + u64::from(experience) * 1_000,
            experience,
        );
        work = work.saturating_add(observed.work);
        trace.extend(observed.trace.clone());
        probes.push(observed);
    }
    TrainingResult {
        world,
        coupling_curve: curve,
        probes,
        trace,
        work,
        quiescent,
    }
}

fn e12(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> E12Observation {
    let primary_result = train(profile, mechanics, root, false, Consequence::Local);
    let permutation = train(
        profile,
        mechanics,
        root + 1_000_000,
        true,
        Consequence::Local,
    );
    let none = train(
        profile,
        mechanics,
        root + 2_000_000,
        false,
        Consequence::None,
    );
    let wrong = train(
        profile,
        mechanics,
        root + 3_000_000,
        false,
        Consequence::Wrong,
    );
    let TrainingResult {
        world: mut primary,
        coupling_curve: curve,
        probes,
        trace,
        mut work,
        mut quiescent,
    } = primary_result;
    let unrelated_before = primary
        .body
        .core0_coupling_material(primary.unrelated_negative);
    let positive_before = primary
        .positive
        .map(|id| primary.body.core0_coupling_material(id));
    let learned_before = primary
        .negative
        .map(|id| primary.body.core0_coupling_material(id));
    let tick = primary.body.clock().tick.saturating_add(1);
    let final_use = pulse_many(
        &mut primary.body,
        &[primary.ia, primary.ib, primary.unrelated],
        tick,
        root + 4_000_000,
    );
    work = work.saturating_add(final_use.work.physical_total());
    quiescent &= final_use.naturally_quiescent;
    let learned_after = primary
        .negative
        .map(|id| primary.body.core0_coupling_material(id));
    let monotonic = curve.windows(2).all(|pair| {
        pair[1][0].saturating_abs() > pair[0][0].saturating_abs()
            && pair[1][1].saturating_abs() > pair[0][1].saturating_abs()
    });
    let zero_active = !probes[0].stable && probes[0].first_ceiling;
    let two_plus_stable = probes.iter().skip(2).all(|probe| probe.stable);
    let controls = none
        .coupling_curve
        .iter()
        .all(|value| *value == none.coupling_curve[0])
        && wrong
            .coupling_curve
            .iter()
            .all(|value| *value == wrong.coupling_curve[0])
        && curve == permutation.coupling_curve
        && probes
            .iter()
            .map(|probe| probe.stable)
            .eq(permutation.probes.iter().map(|probe| probe.stable))
        && learned_before == learned_after
        && unrelated_before
            == primary
                .body
                .core0_coupling_material(primary.unrelated_negative)
        && positive_before
            == primary
                .positive
                .map(|id| primary.body.core0_coupling_material(id));
    let pass = monotonic
        && zero_active
        && two_plus_stable
        && controls
        && quiescent
        && permutation.quiescent
        && none.quiescent
        && wrong.quiescent;
    let reason = format!(
        "curve={} stable={} fires={}",
        curve
            .iter()
            .map(|pair| format!("{}:{}", pair[0], pair[1]))
            .collect::<Vec<_>>()
            .join("|"),
        probes
            .iter()
            .map(|probe| probe.stable.to_string())
            .collect::<Vec<_>>()
            .join("|"),
        probes
            .iter()
            .map(|probe| format!("{}:{}", probe.fires_a, probe.fires_b))
            .collect::<Vec<_>>()
            .join("|")
    );
    E12Observation {
        pass,
        coupling_curve: curve,
        probes,
        control_curves: vec![
            permutation.coupling_curve,
            none.coupling_curve,
            wrong.coupling_curve,
        ],
        primary_trace: trace,
        final_coupling: primary
            .negative
            .map(|id| primary.body.core0_coupling_material(id)),
        final_resistance: primary
            .negative
            .map(|id| primary.body.core0_resistance_material(id)),
        work,
        quiescent,
        reason,
    }
}

fn frames() -> Vec<Vec<u8>> {
    let mut frames = Vec::new();
    let mut contexts = BTreeSet::new();
    for nonce in 0_u16..u16::MAX {
        let mut candidate = vec![4_u8; ARC3_FRAME_PIXELS];
        candidate[0] = (nonce & 0x0f) as u8;
        candidate[1] = (nonce >> 4 & 0x0f) as u8;
        candidate[2] = (nonce >> 8 & 0x0f) as u8;
        candidate[3] = (nonce >> 12 & 0x0f) as u8;
        let context = spatial_context(&candidate).expect("valid frame");
        if contexts.insert(context) {
            frames.push(candidate);
            if frames.len() == 5 {
                break;
            }
        }
    }
    frames
}

fn academy(
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
    probe_all: bool,
) -> AcademyObservation {
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, profile).expect("Academy body");
    let frames = frames();
    let actions = [1_u8, 4, 2, 3];
    let mut observed = Vec::new();
    for (index, action) in actions.into_iter().enumerate() {
        observed.push(
            organism
                .observe(
                    frames[index].clone(),
                    &[1, 2, 3, 4],
                    Some(action),
                    index > 0,
                    false,
                    &[1, 2, 3, 4],
                )
                .expect("training observation"),
        );
    }
    observed.push(
        organism
            .observe(
                frames[4].clone(),
                &[1, 2, 3, 4],
                None,
                true,
                false,
                &[1, 2, 3, 4],
            )
            .expect("closure observation"),
    );
    let (actions, pass) = if probe_all {
        organism.clear_episode();
        let mut outputs = Vec::new();
        for frame in frames.iter().take(4) {
            outputs.push(
                organism
                    .observe(
                        frame.clone(),
                        &[1, 2, 3, 4],
                        None,
                        false,
                        false,
                        &[1, 2, 3, 4],
                    )
                    .expect("probe")
                    .action,
            );
            organism.clear_episode();
        }
        let pass = outputs == [Some(1), Some(4), Some(2), Some(3)];
        (outputs, pass)
    } else {
        let outputs = observed
            .iter()
            .take(4)
            .map(|entry| entry.action)
            .collect::<Vec<_>>();
        let pass = outputs == [Some(1), Some(4), Some(2), Some(3)]
            && observed
                .iter()
                .map(|entry| entry.plasticity_updates)
                .collect::<Vec<_>>()
                == [0, 1, 1, 1, 1];
        (outputs, pass)
    };
    AcademyObservation {
        pass: pass && observed.iter().all(|entry| entry.naturally_quiescent),
        actions,
        updates: observed
            .iter()
            .map(|entry| entry.plasticity_updates)
            .collect(),
        work: observed.iter().map(|entry| entry.physical_work).sum(),
        tick: observed.last().map_or(0, |entry| entry.physical_tick),
        quiescent: observed.iter().all(|entry| entry.naturally_quiescent),
    }
}

fn write_e12(
    csv: &mut BufWriter<File>,
    profile: &str,
    mechanics: &str,
    observation: &E12Observation,
) {
    writeln!(
        csv,
        "{profile},{mechanics},E12,{},{},{},{},{},{}",
        observation.pass,
        observation.quiescent,
        observation.work,
        observation
            .coupling_curve
            .iter()
            .map(|pair| format!("{}:{}", pair[0], pair[1]))
            .collect::<Vec<_>>()
            .join("|"),
        observation
            .probes
            .iter()
            .map(|probe| probe.stable.to_string())
            .collect::<Vec<_>>()
            .join("|"),
        observation.reason.replace(',', ";")
    )
    .expect("write E12 row");
    csv.flush().expect("flush E12 row");
}

fn write_academy(
    csv: &mut BufWriter<File>,
    profile: &str,
    mechanics: &str,
    gate: &str,
    observation: &AcademyObservation,
) {
    writeln!(
        csv,
        "{profile},{mechanics},{gate},{},{},{},actions={:?},updates={:?},",
        observation.pass,
        observation.quiescent,
        observation.work,
        observation.actions,
        observation.updates
    )
    .expect("write Academy row");
    csv.flush().expect("flush Academy row");
}

fn main() {
    eprintln!("CORE1_E12_V2_CONTINUATION_V1_EVIDENCE_SPENT");
    let destination = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_e12_v2_continuation_v1"));
    fs::create_dir_all(&destination).expect("create result directory");
    let mut csv = BufWriter::new(File::create(destination.join("matrix.csv")).expect("create CSV"));
    csv.write_all(
        b"profile,mechanics,gate,pass,quiescent,physical_work,coupling_curve,stable_curve,reason\n",
    )
    .expect("write CSV header");
    csv.flush().expect("flush CSV header");

    let mut all_e12_pass = true;
    let mut summary = String::from("# CORE1 E12 v2 continuation result\n\n| Profile | E12 | Replay | Mechanics |\n|---|---|---|---|\n");
    for (index, (profile, name)) in PROFILES.into_iter().enumerate() {
        let root = 15_000_000 + u64::try_from(index).unwrap_or(0) * 5_000_000;
        let reference = e12(profile, MechanicalConfig::REFERENCE, root);
        write_e12(&mut csv, name, "reference", &reference);
        let replay = e12(profile, MechanicalConfig::REFERENCE, root);
        write_e12(&mut csv, name, "replay", &replay);
        let production = e12(profile, MechanicalConfig::PRODUCTION, root);
        write_e12(&mut csv, name, "production", &production);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        let pass = reference.pass && replay_exact && mechanics_exact;
        all_e12_pass &= pass;
        writeln!(
            summary,
            "| {name} | {} | {replay_exact} | {mechanics_exact} |",
            reference.pass
        )
        .expect("write summary");
        fs::write(destination.join("summary.md"), &summary).expect("stream summary");
    }

    if all_e12_pass {
        summary.push_str("\n| Profile | E13 | E14 | Replay/mechanics exact |\n|---|---|---|---|\n");
        for (index, (profile, name)) in PROFILES.into_iter().enumerate() {
            let root = 40_000_000 + u64::try_from(index).unwrap_or(0) * 1_000_000;
            let e13_reference = academy(profile, MechanicalConfig::REFERENCE, root, true);
            write_academy(&mut csv, name, "reference", "E13", &e13_reference);
            let e13_replay = academy(profile, MechanicalConfig::REFERENCE, root, true);
            write_academy(&mut csv, name, "replay", "E13", &e13_replay);
            let e13_production = academy(profile, MechanicalConfig::PRODUCTION, root, true);
            write_academy(&mut csv, name, "production", "E13", &e13_production);
            let e13_exact = e13_reference == e13_replay && e13_reference == e13_production;

            let e14_reference =
                academy(profile, MechanicalConfig::REFERENCE, root + 500_000, false);
            write_academy(&mut csv, name, "reference", "E14", &e14_reference);
            let e14_replay = academy(profile, MechanicalConfig::REFERENCE, root + 500_000, false);
            write_academy(&mut csv, name, "replay", "E14", &e14_replay);
            let e14_production =
                academy(profile, MechanicalConfig::PRODUCTION, root + 500_000, false);
            write_academy(&mut csv, name, "production", "E14", &e14_production);
            let e14_exact = e14_reference == e14_replay && e14_reference == e14_production;
            writeln!(
                summary,
                "| {name} | {} | {} | {} |",
                e13_reference.pass,
                e14_reference.pass,
                e13_exact && e14_exact
            )
            .expect("write summary");
            fs::write(destination.join("summary.md"), &summary).expect("stream summary");
        }
    } else {
        summary
            .push_str("\nE13/E14 were not reached because E12 did not pass for every profile.\n");
        fs::write(destination.join("summary.md"), &summary).expect("write blocked summary");
    }
    println!("CORE1_E12_V2_CONTINUATION_COMPLETE e12_all_pass={all_e12_pass}");
}
