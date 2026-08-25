#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::env;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use academy_arc3::{
    spatial_context, Arc3ContextDiagnostic, Arc3Sensorimotor, Arc3TransientHistoryRequest,
    ARC3_FRAME_PIXELS,
};
use truelearner_core::{Core0Profile, MechanicalConfig};

const ACTIONS: [u8; 4] = [1, 2, 3, 4];
const SEEDS: usize = 8;
const STAGES: usize = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Material {
    outgoing_coupling: i64,
    outgoing_resistance: u64,
    stem_coupling: i64,
    stem_resistance: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: usize,
    order: [u8; 4],
    useful: u8,
    explored: Vec<u8>,
    preference: u8,
    h_actions: [Option<u8>; STAGES],
    c_actions: [Option<u8>; STAGES],
    random_actions: [Option<u8>; STAGES],
    random_choices: [u8; STAGES],
    consequences_admitted: [bool; 2],
    consequence_modulation: [u64; 2],
    consequence_updates: [u64; 2],
    materials: [Material; STAGES],
    quiescent: bool,
    work: u64,
}

fn frame() -> Vec<u8> {
    let mut contexts = BTreeSet::new();
    for nonce in 0_u16..u16::MAX {
        let mut candidate = vec![4_u8; ARC3_FRAME_PIXELS];
        candidate[0] = (nonce & 0x0f) as u8;
        candidate[1] = (nonce >> 4 & 0x0f) as u8;
        candidate[2] = (nonce >> 8 & 0x0f) as u8;
        candidate[3] = (nonce >> 12 & 0x0f) as u8;
        let context = spatial_context(&candidate).expect("valid E15 frame");
        if contexts.insert(context) {
            return candidate;
        }
    }
    panic!("one E15 frame must exist")
}

fn permutation(seed: usize) -> [u8; 4] {
    let mut order = ACTIONS;
    order.rotate_left(seed % ACTIONS.len());
    if seed & 4 != 0 {
        order.reverse();
    }
    order
}

fn useful_action(seed: usize, order: [u8; 4]) -> u8 {
    order[(seed / 2) % ACTIONS.len()]
}

fn random_action(seed: usize, stage: usize, order: [u8; 4]) -> u8 {
    order[(seed
        .saturating_mul(3)
        .saturating_add(stage)
        .saturating_add(1))
        % ACTIONS.len()]
}

fn history(
    organism: &mut Arc3Sensorimotor,
    pixels: &[u8],
    action: u8,
) -> Result<academy_arc3::Arc3SensorimotorObservation, String> {
    organism
        .observe_with_transient_history(Arc3TransientHistoryRequest {
            frame: pixels.to_vec(),
            available_actions: &ACTIONS,
            babble_action: action,
            support_previous: false,
            settle_pressure: false,
            action_map: &ACTIONS,
            early_material_sign: 1,
        })
        .map_err(|error| error.to_string())
}

fn recovered(mut organism: Arc3Sensorimotor) -> Arc3Sensorimotor {
    organism.clear_episode();
    organism
        .advance_gap(1)
        .expect("ordinary E15 probe recovery");
    organism
}

fn h_probe(snapshot: &Arc3Sensorimotor, pixels: &[u8], preference: u8) -> Option<u8> {
    let mut autonomous = recovered(snapshot.clone());
    let expressed = autonomous
        .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("history-preference autonomous probe")
        .action;
    if expressed.is_some() {
        return expressed;
    }
    let mut preferred = recovered(snapshot.clone());
    history(&mut preferred, pixels, preference)
        .ok()
        .and_then(|observation| observation.action)
}

fn c_probe(snapshot: &Arc3Sensorimotor, pixels: &[u8]) -> Option<u8> {
    let mut probe = recovered(snapshot.clone());
    probe
        .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("consequence-only probe")
        .action
}

fn random_probe(snapshot: &Arc3Sensorimotor, pixels: &[u8], action: u8) -> Option<u8> {
    let mut probe = recovered(snapshot.clone());
    history(&mut probe, pixels, action)
        .ok()
        .and_then(|observation| observation.action)
}

fn material(organism: &Arc3Sensorimotor, context: u16, action: u8) -> Material {
    let diagnostic = organism
        .diagnostic_context(context, action.saturating_sub(1))
        .expect("E15 useful-route diagnostic");
    material_from(&diagnostic)
}

fn material_from(diagnostic: &Arc3ContextDiagnostic) -> Material {
    let stem_contacts = diagnostic
        .links
        .iter()
        .filter(|link| link.role == "stem")
        .filter_map(|link| link.contact)
        .collect::<Vec<_>>();
    let outgoing = diagnostic
        .links
        .iter()
        .filter(|link| {
            link.role == "outgoing"
                && link.coupling > 0
                && link
                    .contact
                    .is_some_and(|contact| stem_contacts.contains(&contact))
        })
        .max_by_key(|link| (link.resistance, link.coupling))
        .expect("positive useful outgoing link");
    let stem = diagnostic
        .links
        .iter()
        .filter(|link| link.role == "stem" && link.contact == outgoing.contact)
        .max_by_key(|link| (link.resistance, link.coupling))
        .expect("useful contact stem");
    Material {
        outgoing_coupling: outgoing.coupling,
        outgoing_resistance: outgoing.resistance,
        stem_coupling: stem.coupling,
        stem_resistance: stem.resistance,
    }
}

fn execute(seed: usize, mechanics: MechanicalConfig, root: u64) -> Row {
    let pixels = frame();
    let context = spatial_context(&pixels).expect("E15 context");
    let order = permutation(seed);
    let useful = useful_action(seed, order);
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, Core0Profile::GenericExternal)
            .expect("CORE1-B E15 body");
    let mut quiescent = true;
    let mut work = 0_u64;

    for _ in 0..2 {
        let prime = organism
            .observe(pixels.clone(), &ACTIONS, None, false, false, &ACTIONS)
            .expect("E15 candidate prime");
        assert!(prime.action.is_none(), "E15 prime must remain unresolved");
        quiescent &= prime.naturally_quiescent;
        work = work.saturating_add(prime.physical_work);
        organism.clear_episode();
    }
    organism
        .advance_gap(1)
        .expect("ordinary E15 exploration recovery");

    let unresolved = organism;
    let mut explored = Vec::with_capacity(ACTIONS.len());
    let mut useful_branch = None;
    for action in order {
        let mut attempt = unresolved.clone();
        let observation = history(&mut attempt, &pixels, action)
            .unwrap_or_else(|error| panic!("seed {seed} action {action} history failed: {error}"));
        assert_eq!(
            observation.action,
            Some(action),
            "every fixed E15 exploratory action must execute"
        );
        explored.push(action);
        quiescent &= observation.naturally_quiescent;
        work = work.saturating_add(observation.physical_work);
        if action == useful {
            useful_branch = Some(attempt);
        }
    }

    let mut organism = useful_branch.expect("useful action explored once");
    let before_consequence = organism.clone();
    let first_consequence = organism
        .admit_previous_consequence()
        .expect("first ordinary E15 consequence");
    quiescent &= first_consequence.naturally_quiescent;
    work = work.saturating_add(first_consequence.physical_work);
    let after_one = organism.clone();

    organism
        .advance_gap(1)
        .expect("ordinary recovery before second consequence experience");
    let reinforcement = history(&mut organism, &pixels, useful)
        .unwrap_or_else(|error| panic!("seed {seed} useful reinforcement failed: {error}"));
    assert_eq!(reinforcement.action, Some(useful));
    quiescent &= reinforcement.naturally_quiescent;
    work = work.saturating_add(reinforcement.physical_work);
    let second_consequence = organism
        .admit_previous_consequence()
        .expect("second ordinary E15 consequence");
    quiescent &= second_consequence.naturally_quiescent;
    work = work.saturating_add(second_consequence.physical_work);
    let after_two = organism.clone();

    let snapshots = [&before_consequence, &after_one, &after_two];
    let h_snapshots = [&unresolved, &after_one, &after_two];
    let h_actions = std::array::from_fn(|stage| h_probe(h_snapshots[stage], &pixels, useful));
    let c_actions = std::array::from_fn(|stage| c_probe(snapshots[stage], &pixels));
    let random_choices = std::array::from_fn(|stage| random_action(seed, stage, order));
    let random_actions =
        std::array::from_fn(|stage| random_probe(&unresolved, &pixels, random_choices[stage]));
    let materials = std::array::from_fn(|stage| material(snapshots[stage], context, useful));

    Row {
        seed,
        order,
        useful,
        explored,
        preference: useful,
        h_actions,
        c_actions,
        random_actions,
        random_choices,
        consequences_admitted: [first_consequence.admitted, second_consequence.admitted],
        consequence_modulation: [
            first_consequence.modulatory_deliveries,
            second_consequence.modulatory_deliveries,
        ],
        consequence_updates: [
            first_consequence.plasticity_updates,
            second_consequence.plasticity_updates,
        ],
        materials,
        quiescent,
        work,
    }
}

fn option_list(values: [Option<u8>; STAGES]) -> String {
    values
        .into_iter()
        .map(|value| value.map_or_else(|| "none".to_string(), |action| action.to_string()))
        .collect::<Vec<_>>()
        .join("|")
}

fn value_list<T: ToString>(values: impl IntoIterator<Item = T>) -> String {
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.get(1).is_some_and(|value| value == "--preflight") {
        let row = execute(0, MechanicalConfig::REFERENCE, 94_000_000);
        println!("{row:#?}");
        return;
    }

    eprintln!("CORE1_E15_HISTORY_VS_CONSEQUENCE_V1_EVIDENCE_SPENT");
    let destination = arguments.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/core1_e15_history_vs_consequence_v1")
    });
    fs::create_dir_all(&destination).expect("create E15 result directory");
    let mut csv = BufWriter::new(File::create(destination.join("matrix.csv")).expect("matrix"));
    writeln!(csv, "seed,mechanics,order,useful,explored,preference,h_actions,c_actions,random_choices,random_actions,consequences_admitted,consequence_modulation,consequence_updates,outgoing_coupling,stem_coupling,outgoing_resistance,stem_resistance,quiescent,physical_work,replay_exact,mechanics_exact").expect("header");

    let mut h_success = [0_usize; STAGES];
    let mut c_success = [0_usize; STAGES];
    let mut random_success = [0_usize; STAGES];
    let mut replay_all = true;
    let mut mechanics_all = true;
    for seed in 0..SEEDS {
        let root = 94_000_000_u64.saturating_add(u64::try_from(seed).unwrap_or(0) * 100_000);
        let reference = execute(seed, MechanicalConfig::REFERENCE, root);
        let replay = execute(seed, MechanicalConfig::REFERENCE, root);
        let production = execute(seed, MechanicalConfig::PRODUCTION, root);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        replay_all &= replay_exact;
        mechanics_all &= mechanics_exact;
        for stage in 0..STAGES {
            h_success[stage] += usize::from(reference.h_actions[stage] == Some(reference.useful));
            c_success[stage] += usize::from(reference.c_actions[stage] == Some(reference.useful));
            random_success[stage] +=
                usize::from(reference.random_actions[stage] == Some(reference.useful));
        }
        for (name, row) in [
            ("reference", &reference),
            ("replay", &replay),
            ("production", &production),
        ] {
            writeln!(
                csv,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                row.seed,
                name,
                value_list(row.order),
                row.useful,
                value_list(&row.explored),
                row.preference,
                option_list(row.h_actions),
                option_list(row.c_actions),
                value_list(row.random_choices),
                option_list(row.random_actions),
                value_list(row.consequences_admitted),
                value_list(row.consequence_modulation),
                value_list(row.consequence_updates),
                value_list(row.materials.iter().map(|item| item.outgoing_coupling)),
                value_list(row.materials.iter().map(|item| item.stem_coupling)),
                value_list(row.materials.iter().map(|item| item.outgoing_resistance)),
                value_list(row.materials.iter().map(|item| item.stem_resistance)),
                row.quiescent,
                row.work,
                replay_exact,
                mechanics_exact,
            )
            .expect("E15 row");
            csv.flush().expect("stream E15 row");
        }
        eprintln!(
            "E15 seed={seed} useful={} H={} C={} random={} replay={} mechanics={}",
            reference.useful,
            option_list(reference.h_actions),
            option_list(reference.c_actions),
            option_list(reference.random_actions),
            replay_exact,
            mechanics_exact,
        );
    }

    let mut report = String::from("# CORE1 E15 history preference versus consequence-only result\n\n| Arm | Before consequence | After one | After two |\n|---|---:|---:|---:|\n");
    for (name, values) in [
        ("H history preference", h_success),
        ("C consequence only", c_success),
        ("Random control", random_success),
    ] {
        writeln!(
            report,
            "| {name} | {}/{} | {}/{} | {}/{} |",
            values[0], SEEDS, values[1], SEEDS, values[2], SEEDS
        )
        .expect("report row");
    }
    writeln!(
        report,
        "\n- exact replay: `{replay_all}`\n- Reference/Production exact: `{mechanics_all}`"
    )
    .expect("report footer");
    fs::write(destination.join("report.md"), report).expect("write E15 report");
    println!(
        "CORE1_E15_HISTORY_VS_CONSEQUENCE_V1_COMPLETE H={} C={} RANDOM={} replay={} mechanics={}",
        value_list(h_success),
        value_list(c_success),
        value_list(random_success),
        replay_all,
        mechanics_all,
    );
}
