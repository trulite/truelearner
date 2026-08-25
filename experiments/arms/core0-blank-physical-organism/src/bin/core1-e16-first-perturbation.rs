#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::env;
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Material {
    outgoing_coupling: i64,
    outgoing_resistance: u64,
    stem_coupling: i64,
    stem_resistance: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArmObservation {
    first_action: Option<u8>,
    ticks_to_first_action: Option<i64>,
    attempts_to_useful: [usize; 2],
    motor_participations: [usize; 2],
    evidence: [bool; 2],
    consequence_admitted: [bool; 2],
    consequence_modulation: [u64; 2],
    consequence_updates: [u64; 2],
    final_action: Option<u8>,
    material: Material,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: usize,
    order: [u8; 4],
    useful: u8,
    zero_action: Option<u8>,
    zero_updates: u64,
    zero_modulation: u64,
    zero_quiescent: bool,
    self_trigger: ArmObservation,
    spontaneous: ArmObservation,
}

fn frame() -> Vec<u8> {
    let mut contexts = BTreeSet::new();
    for nonce in 0_u16..u16::MAX {
        let mut candidate = vec![4_u8; ARC3_FRAME_PIXELS];
        candidate[0] = (nonce & 0x0f) as u8;
        candidate[1] = (nonce >> 4 & 0x0f) as u8;
        candidate[2] = (nonce >> 8 & 0x0f) as u8;
        candidate[3] = (nonce >> 12 & 0x0f) as u8;
        let context = spatial_context(&candidate).expect("valid E16 frame");
        if contexts.insert(context) {
            return candidate;
        }
    }
    panic!("one E16 frame must exist")
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

fn second_variation_order(seed: usize, mut order: [u8; 4]) -> [u8; 4] {
    order.rotate_left((seed + 1) % ACTIONS.len());
    order
}

fn history(
    organism: &mut Arc3Sensorimotor,
    pixels: &[u8],
    action: u8,
) -> academy_arc3::Arc3SensorimotorObservation {
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
        .unwrap_or_else(|error| panic!("E16 action {action} could not participate: {error}"))
}

fn recovered(mut organism: Arc3Sensorimotor) -> Arc3Sensorimotor {
    organism.clear_episode();
    organism
        .advance_gap(1)
        .expect("ordinary E16 episode recovery");
    organism
}

fn unresolved(root: u64, mechanics: MechanicalConfig, pixels: &[u8]) -> Arc3Sensorimotor {
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, Core0Profile::GenericExternal)
            .expect("CORE1-B E16 body");
    for _ in 0..2 {
        let prime = organism
            .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
            .expect("E16 candidate prime");
        assert!(prime.action.is_none(), "E16 prime must remain unresolved");
        assert!(prime.naturally_quiescent, "E16 prime must quiesce");
        organism.clear_episode();
    }
    recovered(organism)
}

fn material(organism: &Arc3Sensorimotor, context: u16, action: u8) -> Material {
    let diagnostic = organism
        .diagnostic_context(context, action.saturating_sub(1))
        .expect("E16 useful-route diagnostic");
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
        .expect("positive E16 useful outgoing link");
    let stem = diagnostic
        .links
        .iter()
        .filter(|link| link.role == "stem" && link.contact == outgoing.contact)
        .max_by_key(|link| (link.resistance, link.coupling))
        .expect("E16 useful stem");
    Material {
        outgoing_coupling: outgoing.coupling,
        outgoing_resistance: outgoing.resistance,
        stem_coupling: stem.coupling,
        stem_resistance: stem.resistance,
    }
}

fn autonomous(organism: &Arc3Sensorimotor, pixels: &[u8]) -> Option<u8> {
    let mut probe = recovered(organism.clone());
    probe
        .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("E16 autonomous probe")
        .action
}

fn opportunity(
    base: &Arc3Sensorimotor,
    pixels: &[u8],
    actions: [u8; 4],
    useful: u8,
) -> (
    Vec<academy_arc3::Arc3SensorimotorObservation>,
    Option<Arc3Sensorimotor>,
    usize,
) {
    let mut observations = Vec::with_capacity(actions.len());
    let mut useful_branch = None;
    let mut attempts_to_useful = 0;
    for (index, action) in actions.into_iter().enumerate() {
        let mut attempt = base.clone();
        let observation = history(&mut attempt, pixels, action);
        if observation.action == Some(useful) && useful_branch.is_none() {
            useful_branch = Some(attempt);
            attempts_to_useful = index + 1;
        }
        observations.push(observation);
    }
    (observations, useful_branch, attempts_to_useful)
}

fn run_arm(
    initial: &Arc3Sensorimotor,
    pixels: &[u8],
    context: u16,
    useful: u8,
    schedules: [[u8; 4]; 2],
) -> ArmObservation {
    let mut base = initial.clone();
    let starting_tick = base
        .snapshot()
        .expect("E16 arm start snapshot")
        .physical_tick;
    let mut first_action = None;
    let mut ticks_to_first_action = None;
    let mut attempts_to_useful = [0_usize; 2];
    let mut motor_participations = [0_usize; 2];
    let mut evidence = [false; 2];
    let mut consequence_admitted = [false; 2];
    let mut consequence_modulation = [0_u64; 2];
    let mut consequence_updates = [0_u64; 2];
    let mut work = 0_u64;
    let mut quiescent = true;

    for opportunity_index in 0..2 {
        let (observations, useful_branch, useful_attempt) =
            opportunity(&base, pixels, schedules[opportunity_index], useful);
        motor_participations[opportunity_index] = observations
            .iter()
            .filter(|observation| observation.action.is_some())
            .count();
        if first_action.is_none() {
            first_action = observations
                .first()
                .and_then(|observation| observation.action);
            ticks_to_first_action = observations
                .first()
                .map(|observation| observation.physical_tick - starting_tick);
        }
        for observation in observations {
            work = work.saturating_add(observation.physical_work);
            quiescent &= observation.naturally_quiescent;
        }
        attempts_to_useful[opportunity_index] = useful_attempt;
        evidence[opportunity_index] = useful_branch.is_some();
        if let Some(mut branch) = useful_branch {
            let consequence = branch
                .admit_previous_consequence()
                .expect("ordinary E16 consequence");
            consequence_admitted[opportunity_index] = consequence.admitted;
            consequence_modulation[opportunity_index] = consequence.modulatory_deliveries;
            consequence_updates[opportunity_index] = consequence.plasticity_updates;
            work = work.saturating_add(consequence.physical_work);
            quiescent &= consequence.naturally_quiescent;
            base = recovered(branch);
        }
    }

    ArmObservation {
        first_action,
        ticks_to_first_action,
        attempts_to_useful,
        motor_participations,
        evidence,
        consequence_admitted,
        consequence_modulation,
        consequence_updates,
        final_action: autonomous(&base, pixels),
        material: material(&base, context, useful),
        work,
        quiescent,
    }
}

fn execute(seed: usize, mechanics: MechanicalConfig, root: u64) -> Row {
    let pixels = frame();
    let context = spatial_context(&pixels).expect("E16 context");
    let order = permutation(seed);
    let useful = useful_action(seed, order);
    let initial = unresolved(root, mechanics, &pixels);

    let mut zero = initial.clone();
    let zero_observation = zero
        .observe(pixels.clone(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("zero-initiation control");

    let self_action = order[0];
    let self_schedule = [self_action; 4];
    let self_trigger = run_arm(
        &initial,
        &pixels,
        context,
        useful,
        [self_schedule, self_schedule],
    );
    let spontaneous = run_arm(
        &initial,
        &pixels,
        context,
        useful,
        [order, second_variation_order(seed, order)],
    );

    Row {
        seed,
        order,
        useful,
        zero_action: zero_observation.action,
        zero_updates: zero_observation.plasticity_updates,
        zero_modulation: zero_observation.modulatory_deliveries,
        zero_quiescent: zero_observation.naturally_quiescent,
        self_trigger,
        spontaneous,
    }
}

fn option(value: Option<u8>) -> String {
    value.map_or_else(|| "none".to_string(), |action| action.to_string())
}

fn list<T: ToString>(values: impl IntoIterator<Item = T>) -> String {
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn write_arm(
    csv: &mut BufWriter<File>,
    row: &Row,
    mechanics: &str,
    arm: &str,
    observation: &ArmObservation,
    replay_exact: bool,
    mechanics_exact: bool,
) {
    writeln!(
        csv,
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        row.seed,
        mechanics,
        arm,
        list(row.order),
        row.useful,
        option(observation.first_action),
        observation
            .ticks_to_first_action
            .map_or_else(|| "none".to_string(), |tick| tick.to_string()),
        list(observation.attempts_to_useful),
        list(observation.motor_participations),
        list(observation.evidence),
        list(observation.consequence_admitted),
        list(observation.consequence_modulation),
        list(observation.consequence_updates),
        option(observation.final_action),
        observation.material.outgoing_coupling,
        observation.material.stem_coupling,
        observation.material.outgoing_resistance,
        observation.material.stem_resistance,
        observation.work,
        observation.quiescent,
        row.zero_action.is_none()
            && row.zero_updates == 0
            && row.zero_modulation == 0
            && row.zero_quiescent,
        row.zero_updates,
        row.zero_modulation,
        replay_exact,
        mechanics_exact,
    )
    .expect("write E16 arm row");
    csv.flush().expect("stream E16 arm row");
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.get(1).is_some_and(|value| value == "--preflight") {
        let row = execute(2, MechanicalConfig::REFERENCE, 95_200_000);
        println!("{row:#?}");
        return;
    }

    eprintln!("CORE1_E16_FIRST_PERTURBATION_V1_EVIDENCE_SPENT");
    let destination = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_e16_first_perturbation_v1"));
    fs::create_dir_all(&destination).expect("create E16 result directory");
    let mut csv = BufWriter::new(File::create(destination.join("matrix.csv")).expect("matrix"));
    writeln!(csv, "seed,mechanics,arm,order,useful,first_action,ticks_to_first_action,attempts_to_useful,motor_participations,evidence,consequence_admitted,consequence_modulation,consequence_updates,final_action,outgoing_coupling,stem_coupling,outgoing_resistance,stem_resistance,physical_work,quiescent,zero_silent,zero_updates,zero_modulation,replay_exact,mechanics_exact").expect("header");

    let mut self_first = 0_usize;
    let mut self_learned = 0_usize;
    let mut variation_first = 0_usize;
    let mut variation_learned = 0_usize;
    let mut zero_silent = 0_usize;
    let mut replay_all = true;
    let mut mechanics_all = true;
    for seed in 0..SEEDS {
        let root = 95_000_000_u64.saturating_add(u64::try_from(seed).unwrap_or(0) * 100_000);
        let reference = execute(seed, MechanicalConfig::REFERENCE, root);
        let replay = execute(seed, MechanicalConfig::REFERENCE, root);
        let production = execute(seed, MechanicalConfig::PRODUCTION, root);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        replay_all &= replay_exact;
        mechanics_all &= mechanics_exact;
        zero_silent += usize::from(
            reference.zero_action.is_none()
                && reference.zero_updates == 0
                && reference.zero_modulation == 0
                && reference.zero_quiescent,
        );
        self_first += usize::from(reference.self_trigger.first_action.is_some());
        self_learned += usize::from(reference.self_trigger.final_action == Some(reference.useful));
        variation_first += usize::from(reference.spontaneous.first_action.is_some());
        variation_learned +=
            usize::from(reference.spontaneous.final_action == Some(reference.useful));

        for (mechanics, row) in [
            ("reference", &reference),
            ("replay", &replay),
            ("production", &production),
        ] {
            write_arm(
                &mut csv,
                row,
                mechanics,
                "self_trigger",
                &row.self_trigger,
                replay_exact,
                mechanics_exact,
            );
            write_arm(
                &mut csv,
                row,
                mechanics,
                "spontaneous",
                &row.spontaneous,
                replay_exact,
                mechanics_exact,
            );
        }
        eprintln!(
            "E16 seed={seed} useful={} zero={} S_first={} S_final={} V_first={} V_final={} replay={} mechanics={}",
            reference.useful,
            option(reference.zero_action),
            option(reference.self_trigger.first_action),
            option(reference.self_trigger.final_action),
            option(reference.spontaneous.first_action),
            option(reference.spontaneous.final_action),
            replay_exact,
            mechanics_exact,
        );
    }

    let report = format!(
        "# CORE1 E16 first perturbation result\n\n| Arm | First action | Learned useful action |\n|---|---:|---:|\n| Zero initiation | 0/{SEEDS} | 0/{SEEDS} |\n| S self-trigger | {self_first}/{SEEDS} | {self_learned}/{SEEDS} |\n| V spontaneous variation | {variation_first}/{SEEDS} | {variation_learned}/{SEEDS} |\n\n- zero controls exact: `{zero_silent}/{SEEDS}`\n- exact replay: `{replay_all}`\n- Reference/Production exact: `{mechanics_all}`\n"
    );
    fs::write(destination.join("report.md"), report).expect("write E16 report");
    println!(
        "CORE1_E16_FIRST_PERTURBATION_V1_COMPLETE zero={zero_silent} S={self_first}|{self_learned} V={variation_first}|{variation_learned} replay={replay_all} mechanics={mechanics_all}"
    );
}
