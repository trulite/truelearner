#![forbid(unsafe_code)]

use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use academy_arc3::{
    spatial_context, Arc3ContextDiagnostic, Arc3Sensorimotor, Arc3TransientHistoryRequest,
    ARC3_FRAME_PIXELS,
};
use truelearner_core::{Core0Profile, MechanicalConfig};

const ACTIONS: [u8; 4] = [1, 2, 3, 4];
const SEEDS: usize = 8;
const MATERIAL_ONE: u64 = 1_u64 << 32;
const RELAX_NUMERATOR: u64 = 15;
const RELAX_DENOMINATOR: u64 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Arm {
    Variation,
    Refractory,
    Depression,
    Trace,
}

impl Arm {
    const ALL: [Self; 4] = [
        Self::Variation,
        Self::Refractory,
        Self::Depression,
        Self::Trace,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Variation => "V_variation",
            Self::Refractory => "R_refractory",
            Self::Depression => "D_depression",
            Self::Trace => "T_trace",
        }
    }
}

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
    attempted: [Vec<u8>; 2],
    participated: [Vec<u8>; 2],
    temporary_state: [Vec<u64>; 2],
    attempts_to_useful: [usize; 2],
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
    arms: Vec<(Arm, ArmObservation)>,
}

#[derive(Clone, Debug)]
struct RowBundle {
    reference: Row,
    replay: Row,
    production: Row,
}

#[derive(Clone, Debug, Default)]
struct TemporaryState {
    refractory: HashSet<u8>,
    depression: [u64; 4],
    trace: [u64; 4],
}

#[derive(Clone)]
struct Opportunity {
    organism: Option<Arc3Sensorimotor>,
    attempted: Vec<u8>,
    participated: Vec<u8>,
    temporary_state: Vec<u64>,
    attempts_to_useful: usize,
    first_tick: Option<i64>,
    work: u64,
    quiescent: bool,
}

fn frame() -> Vec<u8> {
    let mut contexts = BTreeSet::new();
    for nonce in 0_u16..u16::MAX {
        let mut candidate = vec![4_u8; ARC3_FRAME_PIXELS];
        candidate[0] = (nonce & 0x0f) as u8;
        candidate[1] = (nonce >> 4 & 0x0f) as u8;
        candidate[2] = (nonce >> 8 & 0x0f) as u8;
        candidate[3] = (nonce >> 12 & 0x0f) as u8;
        let context = spatial_context(&candidate).expect("valid E17 frame");
        if contexts.insert(context) {
            return candidate;
        }
    }
    panic!("one E17 frame must exist")
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

fn second_order(seed: usize, mut order: [u8; 4]) -> [u8; 4] {
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
        .unwrap_or_else(|error| panic!("E17 action {action} could not participate: {error}"))
}

fn recovered(mut organism: Arc3Sensorimotor) -> Arc3Sensorimotor {
    organism.clear_episode();
    organism
        .advance_gap(1)
        .expect("ordinary E17 interaction recovery");
    organism
}

fn unresolved(root: u64, mechanics: MechanicalConfig, pixels: &[u8]) -> Arc3Sensorimotor {
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, Core0Profile::GenericExternal)
            .expect("CORE1-B E17 body");
    for _ in 0..2 {
        let prime = organism
            .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
            .expect("E17 candidate prime");
        assert!(prime.action.is_none(), "E17 prime must remain unresolved");
        assert!(prime.naturally_quiescent, "E17 prime must quiesce");
        organism.clear_episode();
    }
    recovered(organism)
}

fn material(organism: &Arc3Sensorimotor, context: u16, action: u8) -> Material {
    let diagnostic = organism
        .diagnostic_context(context, action.saturating_sub(1))
        .expect("E17 route diagnostic");
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
        .expect("positive E17 outgoing route");
    let stem = diagnostic
        .links
        .iter()
        .filter(|link| link.role == "stem" && link.contact == outgoing.contact)
        .max_by_key(|link| (link.resistance, link.coupling))
        .expect("positive E17 route stem");
    Material {
        outgoing_coupling: outgoing.coupling,
        outgoing_resistance: outgoing.resistance,
        stem_coupling: stem.coupling,
        stem_resistance: stem.resistance,
    }
}

fn live_route_material(organism: &Arc3Sensorimotor, context: u16, action: u8) -> Option<u64> {
    let diagnostic = organism
        .diagnostic_context(context, action.saturating_sub(1))
        .ok()?;
    let stems = diagnostic
        .links
        .iter()
        .filter(|link| link.role == "stem" && link.coupling > 0)
        .filter_map(|link| link.contact)
        .collect::<Vec<_>>();
    let outgoing = diagnostic
        .links
        .iter()
        .filter(|link| {
            link.role == "outgoing"
                && link.coupling > 0
                && link.contact.is_some_and(|contact| stems.contains(&contact))
        })
        .map(|link| u64::try_from(link.coupling).unwrap_or(0))
        .max()?;
    Some(outgoing)
}

fn autonomous(organism: &Arc3Sensorimotor, pixels: &[u8]) -> Option<u8> {
    let mut probe = recovered(organism.clone());
    probe
        .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("E17 autonomous probe")
        .action
}

fn relax(value: u64, elapsed: i64) -> u64 {
    let mut relaxed = value;
    for _ in 0..elapsed.max(0) {
        relaxed = relaxed.saturating_mul(RELAX_NUMERATOR) / RELAX_DENOMINATOR;
    }
    relaxed
}

fn select_action(
    arm: Arm,
    organism: &Arc3Sensorimotor,
    context: u16,
    order: [u8; 4],
    state: &TemporaryState,
) -> Option<u8> {
    let live = order
        .into_iter()
        .filter_map(|action| live_route_material(organism, context, action).map(|c| (action, c)))
        .collect::<Vec<_>>();
    match arm {
        Arm::Variation => None,
        Arm::Refractory => live
            .into_iter()
            .filter(|(action, _)| !state.refractory.contains(action))
            .fold(None, |best, candidate| match best {
                Some((_, coupling)) if coupling >= candidate.1 => best,
                _ => Some(candidate),
            })
            .map(|(action, _)| action),
        Arm::Depression => live
            .into_iter()
            .filter_map(|(action, coupling)| {
                let index = usize::from(action.saturating_sub(1));
                let available = coupling.saturating_sub(state.depression[index]);
                (available >= MATERIAL_ONE).then_some((action, available))
            })
            .fold(None, |best, candidate| match best {
                Some((_, available)) if available >= candidate.1 => best,
                _ => Some(candidate),
            })
            .map(|(action, _)| action),
        Arm::Trace => {
            let greatest_coupling = live.iter().map(|(_, coupling)| *coupling).max()?;
            let minimum = live
                .iter()
                .filter(|(_, coupling)| *coupling == greatest_coupling)
                .map(|(action, _)| state.trace[usize::from(action.saturating_sub(1))])
                .min()?;
            if minimum != 0 {
                return None;
            }
            live.into_iter()
                .filter(|(_, coupling)| *coupling == greatest_coupling)
                .map(|(action, _)| action)
                .find(|action| state.trace[usize::from(action.saturating_sub(1))] == minimum)
        }
    }
}

fn update_state(arm: Arm, state: &mut TemporaryState, action: u8, coupling: u64) {
    let index = usize::from(action.saturating_sub(1));
    match arm {
        Arm::Variation => {}
        Arm::Refractory => {
            state.refractory.insert(action);
        }
        Arm::Depression => {
            state.depression[index] =
                state.depression[index].saturating_add(coupling / RELAX_DENOMINATOR);
        }
        Arm::Trace => {
            state.trace[index] = state.trace[index].saturating_add(MATERIAL_ONE);
        }
    }
}

fn state_observation(arm: Arm, state: &TemporaryState) -> Vec<u64> {
    match arm {
        Arm::Variation => Vec::new(),
        Arm::Refractory => ACTIONS
            .into_iter()
            .map(|action| u64::from(state.refractory.contains(&action)))
            .collect(),
        Arm::Depression => state.depression.to_vec(),
        Arm::Trace => state.trace.to_vec(),
    }
}

fn variation_opportunity(
    base: &Arc3Sensorimotor,
    pixels: &[u8],
    useful: u8,
    order: [u8; 4],
) -> Opportunity {
    let start_tick = base.snapshot().expect("E17 V start snapshot").physical_tick;
    let mut attempted = Vec::new();
    let mut participated = Vec::new();
    let mut useful_branch = None;
    let mut attempts_to_useful = 0;
    let mut first_tick = None;
    let mut work = 0_u64;
    let mut quiescent = true;
    for (index, action) in order.into_iter().enumerate() {
        let mut attempt = base.clone();
        let observation = history(&mut attempt, pixels, action);
        attempted.push(action);
        work = work.saturating_add(observation.physical_work);
        quiescent &= observation.naturally_quiescent;
        if let Some(actual) = observation.action {
            first_tick.get_or_insert(observation.physical_tick - start_tick);
            participated.push(actual);
            if actual == useful && useful_branch.is_none() {
                useful_branch = Some(attempt);
                attempts_to_useful = index + 1;
            }
        }
    }
    Opportunity {
        organism: useful_branch,
        attempted,
        participated,
        temporary_state: Vec::new(),
        attempts_to_useful,
        first_tick,
        work,
        quiescent,
    }
}

fn local_opportunity(
    arm: Arm,
    base: &Arc3Sensorimotor,
    pixels: &[u8],
    context: u16,
    useful: u8,
    order: [u8; 4],
) -> Opportunity {
    let start_tick = base
        .snapshot()
        .expect("E17 local start snapshot")
        .physical_tick;
    let mut organism = base.clone();
    let mut state = TemporaryState::default();
    let mut attempted = Vec::new();
    let mut participated = Vec::new();
    let mut attempts_to_useful = 0;
    let mut first_tick = None;
    let mut work = 0_u64;
    let mut quiescent = true;

    for trigger in 0..ACTIONS.len() {
        let Some(action) = select_action(arm, &organism, context, order, &state) else {
            break;
        };
        let coupling = live_route_material(&organism, context, action)
            .expect("selected E17 route remains live");
        let before_tick = organism
            .snapshot()
            .expect("E17 trigger snapshot")
            .physical_tick;
        attempted.push(action);
        let observation = organism
            .trigger_transient_continuation(pixels.to_vec(), action, &ACTIONS, 1)
            .unwrap_or_else(|error| panic!("E17 internal action {action} failed: {error}"));
        work = work.saturating_add(observation.physical_work);
        quiescent &= observation.naturally_quiescent;
        let after_tick = observation.physical_tick;
        let elapsed = after_tick.saturating_sub(before_tick);
        for value in &mut state.depression {
            *value = relax(*value, elapsed);
        }
        for value in &mut state.trace {
            *value = relax(*value, elapsed);
        }
        let Some(actual) = observation.action else {
            break;
        };
        participated.push(actual);
        first_tick.get_or_insert(after_tick - start_tick);
        update_state(arm, &mut state, actual, coupling);
        if actual == useful {
            attempts_to_useful = trigger + 1;
            return Opportunity {
                organism: Some(organism),
                attempted,
                participated,
                temporary_state: state_observation(arm, &state),
                attempts_to_useful,
                first_tick,
                work,
                quiescent,
            };
        }
    }

    Opportunity {
        organism: None,
        attempted,
        participated,
        temporary_state: state_observation(arm, &state),
        attempts_to_useful,
        first_tick,
        work,
        quiescent,
    }
}

fn run_arm(
    arm: Arm,
    initial: &Arc3Sensorimotor,
    pixels: &[u8],
    context: u16,
    useful: u8,
    schedules: [[u8; 4]; 2],
) -> ArmObservation {
    let mut base = initial.clone();
    let mut first_action = None;
    let mut ticks_to_first_action = None;
    let mut attempted: [Vec<u8>; 2] = std::array::from_fn(|_| Vec::new());
    let mut participated: [Vec<u8>; 2] = std::array::from_fn(|_| Vec::new());
    let mut temporary_state: [Vec<u64>; 2] = std::array::from_fn(|_| Vec::new());
    let mut attempts_to_useful = [0_usize; 2];
    let mut consequence_admitted = [false; 2];
    let mut consequence_modulation = [0_u64; 2];
    let mut consequence_updates = [0_u64; 2];
    let mut work = 0_u64;
    let mut quiescent = true;

    for opportunity_index in 0..2 {
        let opportunity = if arm == Arm::Variation {
            variation_opportunity(&base, pixels, useful, schedules[opportunity_index])
        } else {
            local_opportunity(
                arm,
                &base,
                pixels,
                context,
                useful,
                schedules[opportunity_index],
            )
        };
        if first_action.is_none() {
            first_action = opportunity.participated.first().copied();
            ticks_to_first_action = opportunity.first_tick;
        }
        attempted[opportunity_index] = opportunity.attempted;
        participated[opportunity_index] = opportunity.participated;
        temporary_state[opportunity_index] = opportunity.temporary_state;
        attempts_to_useful[opportunity_index] = opportunity.attempts_to_useful;
        work = work.saturating_add(opportunity.work);
        quiescent &= opportunity.quiescent;
        if let Some(mut branch) = opportunity.organism {
            let consequence = branch
                .admit_previous_consequence()
                .expect("ordinary E17 consequence");
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
        attempted,
        participated,
        temporary_state,
        attempts_to_useful,
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
    let context = spatial_context(&pixels).expect("E17 context");
    let order = permutation(seed);
    let useful = useful_action(seed, order);
    let initial = unresolved(root, mechanics, &pixels);
    let mut zero = initial.clone();
    let zero_observation = zero
        .observe(pixels.clone(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("E17 zero-initiation control");
    let schedules = [order, second_order(seed, order)];
    let arms = Arm::ALL
        .into_iter()
        .map(|arm| {
            let arm_schedules = if arm == Arm::Variation {
                schedules
            } else {
                [order, order]
            };
            (
                arm,
                run_arm(arm, &initial, &pixels, context, useful, arm_schedules),
            )
        })
        .collect();
    Row {
        seed,
        order,
        useful,
        zero_action: zero_observation.action,
        zero_updates: zero_observation.plasticity_updates,
        zero_modulation: zero_observation.modulatory_deliveries,
        zero_quiescent: zero_observation.naturally_quiescent,
        arms,
    }
}

fn tiny_sequence(arm: Arm, count: usize) -> Vec<usize> {
    let mut refractory = vec![false; count];
    let mut depression = vec![0_u64; count];
    let mut trace = vec![0_u64; count];
    let mut sequence = Vec::new();
    for _ in 0..count {
        let selected = match arm {
            Arm::Variation => (0..count).find(|index| !sequence.contains(index)),
            Arm::Refractory => (0..count).find(|index| !refractory[*index]),
            Arm::Depression => (0..count)
                .filter_map(|index| {
                    let available = MATERIAL_ONE.saturating_sub(depression[index]);
                    (available >= MATERIAL_ONE).then_some((index, available))
                })
                .fold(None, |best, candidate| match best {
                    Some((_, available)) if available >= candidate.1 => best,
                    _ => Some(candidate),
                })
                .map(|(index, _)| index),
            Arm::Trace => {
                let minimum = trace.iter().copied().min().unwrap_or(0);
                (minimum == 0)
                    .then(|| (0..count).find(|index| trace[*index] == minimum))
                    .flatten()
            }
        };
        let Some(selected) = selected else {
            break;
        };
        sequence.push(selected);
        match arm {
            Arm::Variation => {}
            Arm::Refractory => refractory[selected] = true,
            Arm::Depression => depression[selected] = MATERIAL_ONE / RELAX_DENOMINATOR,
            Arm::Trace => trace[selected] = MATERIAL_ONE,
        }
    }
    sequence
}

fn tiny_gate() -> Vec<(Arm, usize, bool, Vec<usize>)> {
    let mut rows = Vec::new();
    for arm in Arm::ALL {
        for count in [1_usize, 2, 4, 8, 16] {
            let sequence = tiny_sequence(arm, count);
            let expected = (0..count).collect::<Vec<_>>();
            rows.push((arm, count, sequence == expected, sequence));
        }
    }
    rows
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

fn write_rows(
    csv: &mut BufWriter<File>,
    row: &Row,
    mechanics: &str,
    replay_exact: bool,
    mechanics_exact: bool,
) {
    let zero_silent = row.zero_action.is_none()
        && row.zero_updates == 0
        && row.zero_modulation == 0
        && row.zero_quiescent;
    for (arm, observation) in &row.arms {
        writeln!(
            csv,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            row.seed,
            mechanics,
            arm.name(),
            list(row.order),
            row.useful,
            option(observation.first_action),
            observation
                .ticks_to_first_action
                .map_or_else(|| "none".to_string(), |tick| tick.to_string()),
            list(observation.attempted[0].iter()),
            list(observation.attempted[1].iter()),
            list(observation.participated[0].iter()),
            list(observation.participated[1].iter()),
            list(observation.temporary_state[0].iter()),
            list(observation.temporary_state[1].iter()),
            list(observation.attempts_to_useful),
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
            zero_silent,
            replay_exact,
            mechanics_exact,
            observation.final_action == Some(row.useful),
        )
        .expect("write E17 arm row");
    }
    csv.flush().expect("stream E17 rows");
}

fn bundle(seed: usize) -> RowBundle {
    let root = 96_000_000_u64.saturating_add(u64::try_from(seed).unwrap_or(0) * 100_000);
    RowBundle {
        reference: execute(seed, MechanicalConfig::REFERENCE, root),
        replay: execute(seed, MechanicalConfig::REFERENCE, root),
        production: execute(seed, MechanicalConfig::PRODUCTION, root),
    }
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    let tiny = tiny_gate();
    if arguments.get(1).is_some_and(|value| value == "--tiny") {
        for row in tiny {
            println!(
                "{} n={} pass={} sequence={:?}",
                row.0.name(),
                row.1,
                row.2,
                row.3
            );
        }
        return;
    }
    assert!(tiny.iter().all(|row| row.2), "E17 tiny gate failed");
    if arguments.get(1).is_some_and(|value| value == "--preflight") {
        println!("{:#?}", execute(7, MechanicalConfig::REFERENCE, 96_700_000));
        return;
    }
    if arguments
        .get(1)
        .is_some_and(|value| value == "--preflight-first")
    {
        println!("{:#?}", execute(0, MechanicalConfig::REFERENCE, 96_000_000));
        return;
    }

    eprintln!("CORE1_E17_SYMMETRY_BREAKING_TOURNAMENT_V2_EVIDENCE_SPENT");
    let destination = arguments.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/core1_e17_symmetry_breaking_tournament_v2")
    });
    fs::create_dir_all(&destination).expect("create E17 result directory");
    let mut tiny_csv =
        BufWriter::new(File::create(destination.join("tiny_matrix.csv")).expect("tiny"));
    writeln!(tiny_csv, "arm,route_count,pass,sequence").expect("tiny header");
    for (arm, count, pass, sequence) in &tiny {
        writeln!(
            tiny_csv,
            "{},{},{},{}",
            arm.name(),
            count,
            pass,
            list(sequence.iter())
        )
        .expect("tiny row");
    }
    tiny_csv.flush().expect("tiny flush");

    let (sender, receiver) = mpsc::channel();
    thread::scope(|scope| {
        for seed in 0..SEEDS {
            let sender = sender.clone();
            scope.spawn(move || {
                let result = bundle(seed);
                sender.send(result).expect("send E17 seed");
            });
        }
        drop(sender);
    });
    let mut bundles = receiver.into_iter().collect::<Vec<_>>();
    bundles.sort_by_key(|bundle| bundle.reference.seed);

    let mut csv = BufWriter::new(File::create(destination.join("matrix.csv")).expect("matrix"));
    writeln!(csv, "seed,mechanics,arm,order,useful,first_action,ticks_to_first_action,attempted_1,attempted_2,participated_1,participated_2,temporary_state_1,temporary_state_2,attempts_to_useful,consequence_admitted,consequence_modulation,consequence_updates,final_action,outgoing_coupling,stem_coupling,outgoing_resistance,stem_resistance,physical_work,quiescent,zero_silent,replay_exact,mechanics_exact,learned_useful").expect("matrix header");

    let mut learned = [0_usize; 4];
    let mut first = [0_usize; 4];
    let mut replay_all = true;
    let mut mechanics_all = true;
    let mut zero_all = true;
    for bundle in &bundles {
        let replay_exact = bundle.reference == bundle.replay;
        let mechanics_exact = bundle.reference == bundle.production;
        replay_all &= replay_exact;
        mechanics_all &= mechanics_exact;
        zero_all &= bundle.reference.zero_action.is_none()
            && bundle.reference.zero_updates == 0
            && bundle.reference.zero_modulation == 0
            && bundle.reference.zero_quiescent;
        for (index, (_, observation)) in bundle.reference.arms.iter().enumerate() {
            first[index] += usize::from(observation.first_action.is_some());
            learned[index] +=
                usize::from(observation.final_action == Some(bundle.reference.useful));
        }
        for (mechanics, row) in [
            ("reference", &bundle.reference),
            ("replay", &bundle.replay),
            ("production", &bundle.production),
        ] {
            write_rows(&mut csv, row, mechanics, replay_exact, mechanics_exact);
        }
        eprintln!(
            "E17 seed={} useful={} V={} R={} D={} T={} replay={} mechanics={}",
            bundle.reference.seed,
            bundle.reference.useful,
            option(bundle.reference.arms[0].1.final_action),
            option(bundle.reference.arms[1].1.final_action),
            option(bundle.reference.arms[2].1.final_action),
            option(bundle.reference.arms[3].1.final_action),
            replay_exact,
            mechanics_exact,
        );
    }

    let report = format!(
        "# CORE1 E17 local symmetry-breaking tournament v2\n\n| Arm | First action | Learned useful |\n|---|---:|---:|\n| V bounded variation | {}/{SEEDS} | {}/{SEEDS} |\n| R route refractory | {}/{SEEDS} | {}/{SEEDS} |\n| D efficacy depression | {}/{SEEDS} | {}/{SEEDS} |\n| T usage trace | {}/{SEEDS} | {}/{SEEDS} |\n\n- tiny gate: `20/20`\n- zero control exact: `{zero_all}`\n- exact replay: `{replay_all}`\n- Reference/Production exact: `{mechanics_all}`\n",
        first[0], learned[0], first[1], learned[1], first[2], learned[2], first[3],
        learned[3],
    );
    fs::write(destination.join("report.md"), report).expect("write E17 report");
    println!(
        "CORE1_E17_SYMMETRY_BREAKING_TOURNAMENT_V2_COMPLETE V={}|{} R={}|{} D={}|{} T={}|{} replay={} mechanics={}",
        first[0], learned[0], first[1], learned[1], first[2], learned[2], first[3],
        learned[3], replay_all, mechanics_all,
    );
}
