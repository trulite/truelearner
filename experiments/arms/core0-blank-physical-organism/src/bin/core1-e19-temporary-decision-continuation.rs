#![forbid(unsafe_code)]

use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::mem::size_of;
use std::path::PathBuf;

use academy_arc3::{
    spatial_context, Arc3DecisionOpenness, Arc3Sensorimotor, Arc3SensorimotorObservation,
    ARC3_FRAME_PIXELS,
};
use truelearner_core::{Core0Profile, MechanicalConfig};

const ACTIONS: [u8; 4] = [1, 2, 3, 4];
const SEEDS: usize = 8;
const MAX_COMPLETIONS: usize = ACTIONS.len();

#[derive(Clone, Debug, PartialEq, Eq)]
struct Attempt {
    attempted: u8,
    participated: Option<u8>,
    completion_delivered: bool,
    completion_work: u64,
    open_before_completion: Arc3DecisionOpenness,
    open_after_completion: Arc3DecisionOpenness,
    reactivation_eligible: bool,
    reactivated: bool,
    regenerated: Vec<u8>,
    consequence_admitted: bool,
    consequence_updates: u64,
    open_after_consequence: Arc3DecisionOpenness,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Opportunity {
    attempts: Vec<Attempt>,
    encountered_useful: bool,
    completion_count: usize,
    reactivation_count: usize,
    consequence_updates: u64,
    open_at_start: Arc3DecisionOpenness,
    open_at_end: Arc3DecisionOpenness,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Controls {
    closed_silent: bool,
    missing_completion_one_attempt: bool,
    missing_completion_no_reactivation: bool,
    closed_completion_delivered: bool,
    closed_completion_ineligible: bool,
    failed_completion_zero_updates: bool,
    completion_preserves_open: bool,
    consequence_closes: bool,
    closed_after_success_completion_ineligible: bool,
    marker_schema_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    seed: usize,
    order: [u8; 4],
    useful: u8,
    first_action: Option<u8>,
    opportunities: [Opportunity; 2],
    final_action: Option<u8>,
    controls: Controls,
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
        let context = spatial_context(&candidate).expect("valid E19 frame");
        if contexts.insert(context) {
            return candidate;
        }
    }
    panic!("one E19 frame must exist")
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

fn recovered(mut organism: Arc3Sensorimotor) -> Arc3Sensorimotor {
    organism.clear_episode();
    organism
        .advance_gap(1)
        .expect("ordinary E19 interaction recovery");
    organism
}

fn unresolved(root: u64, mechanics: MechanicalConfig, pixels: &[u8]) -> Arc3Sensorimotor {
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, Core0Profile::GenericExternal)
            .expect("CORE1-B E19 body");
    for _ in 0..2 {
        let prime = organism
            .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
            .expect("E19 candidate prime");
        assert!(prime.action.is_none(), "E19 prime must remain unresolved");
        assert!(prime.naturally_quiescent, "E19 prime must quiesce");
        organism.clear_episode();
    }
    recovered(organism)
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
    diagnostic
        .links
        .iter()
        .filter(|link| {
            link.role == "outgoing"
                && link.coupling > 0
                && link.contact.is_some_and(|contact| stems.contains(&contact))
        })
        .map(|link| u64::try_from(link.coupling).unwrap_or(0))
        .max()
}

fn live_actions(organism: &Arc3Sensorimotor, context: u16) -> Vec<u8> {
    ACTIONS
        .into_iter()
        .filter(|action| live_route_material(organism, context, *action).is_some())
        .collect()
}

fn select_action(
    organism: &Arc3Sensorimotor,
    context: u16,
    order: [u8; 4],
    refractory: &HashSet<u8>,
) -> Option<u8> {
    order
        .into_iter()
        .filter(|action| !refractory.contains(action))
        .filter_map(|action| live_route_material(organism, context, action).map(|c| (action, c)))
        .fold(None, |best, candidate| match best {
            Some((_, coupling)) if coupling >= candidate.1 => best,
            _ => Some(candidate),
        })
        .map(|(action, _)| action)
}

fn autonomous(organism: &Arc3Sensorimotor, pixels: &[u8]) -> Option<u8> {
    let mut probe = recovered(organism.clone());
    probe
        .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("E19 autonomous probe")
        .action
}

fn trigger(
    organism: &mut Arc3Sensorimotor,
    pixels: &[u8],
    action: u8,
) -> Arc3SensorimotorObservation {
    organism
        .trigger_transient_continuation(pixels.to_vec(), action, &ACTIONS, 1)
        .unwrap_or_else(|error| panic!("E19 internal action {action} failed: {error}"))
}

fn run_opportunity(
    base: &Arc3Sensorimotor,
    pixels: &[u8],
    context: u16,
    useful: u8,
    order: [u8; 4],
) -> (Opportunity, Option<Arc3Sensorimotor>) {
    let mut organism = base.clone();
    organism
        .open_decision_interaction(context)
        .expect("install and OPEN E19 interaction");
    let open_at_start = organism.decision_openness();
    let initial = organism
        .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("E19 initial junction activation");
    let mut pending_action = initial.action;
    let mut refractory = HashSet::new();
    let mut attempts = Vec::new();
    let mut completion_count = 0_usize;
    let mut reactivation_count = 0_usize;
    let mut consequence_updates = 0_u64;
    let mut work = initial.physical_work;
    let mut quiescent = initial.naturally_quiescent;

    while completion_count < MAX_COMPLETIONS {
        let selected = pending_action
            .take()
            .or_else(|| select_action(&organism, context, order, &refractory));
        let Some(selected) = selected else {
            break;
        };
        let action_observation = if initial.action == Some(selected) && attempts.is_empty() {
            initial.clone()
        } else {
            trigger(&mut organism, pixels, selected)
        };
        if !(initial.action == Some(selected) && attempts.is_empty()) {
            work = work.saturating_add(action_observation.physical_work);
            quiescent &= action_observation.naturally_quiescent;
        }
        let Some(participated) = action_observation.action else {
            break;
        };
        refractory.insert(participated);
        completion_count += 1;
        let live_before_return = live_actions(&organism, context);
        let open_before_completion = organism.decision_openness();
        let completion = organism
            .return_decision_completion()
            .expect("E19 positive completion return");
        work = work.saturating_add(completion.physical_work);
        quiescent &= completion.naturally_quiescent;
        let open_after_completion = organism.decision_openness();
        let mut attempt = Attempt {
            attempted: selected,
            participated: Some(participated),
            completion_delivered: completion.delivered,
            completion_work: completion.physical_work,
            open_before_completion,
            open_after_completion,
            reactivation_eligible: completion.reactivation_eligible,
            reactivated: false,
            regenerated: Vec::new(),
            consequence_admitted: false,
            consequence_updates: 0,
            open_after_consequence: open_after_completion,
        };
        if completion.reactivation_eligible {
            attempt.reactivated = true;
            reactivation_count += 1;
        }
        if participated == useful {
            let consequence = organism
                .admit_open_decision_consequence()
                .expect("ordinary E19 consequence");
            work = work.saturating_add(consequence.physical_work);
            quiescent &= consequence.naturally_quiescent;
            consequence_updates = consequence.plasticity_updates;
            attempt.consequence_admitted = consequence.admitted;
            attempt.consequence_updates = consequence.plasticity_updates;
            attempt.open_after_consequence = organism.decision_openness();
            attempts.push(attempt);
            return (
                Opportunity {
                    attempts,
                    encountered_useful: true,
                    completion_count,
                    reactivation_count,
                    consequence_updates,
                    open_at_start,
                    open_at_end: organism.decision_openness(),
                    work,
                    quiescent,
                },
                Some(organism),
            );
        }
        if completion.reactivation_eligible {
            let after = live_actions(&organism, context);
            attempt.regenerated = after
                .into_iter()
                .filter(|action| !live_before_return.contains(action))
                .collect();
        }
        attempts.push(attempt);
        if !completion.reactivation_eligible {
            break;
        }
    }

    (
        Opportunity {
            attempts,
            encountered_useful: false,
            completion_count,
            reactivation_count,
            consequence_updates,
            open_at_start,
            open_at_end: organism.decision_openness(),
            work,
            quiescent,
        },
        None,
    )
}

fn controls(
    initial: &Arc3Sensorimotor,
    pixels: &[u8],
    context: u16,
    order: [u8; 4],
    learned: &Arc3Sensorimotor,
    opportunities: &[Opportunity; 2],
) -> Controls {
    let mut closed = initial.clone();
    let closed_observation = closed
        .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("E19 CLOSED control");

    let mut missing = initial.clone();
    missing
        .open_decision_interaction(context)
        .expect("install and OPEN missing-completion interaction");
    let selected = select_action(&missing, context, order, &HashSet::new())
        .expect("E19 missing-completion route");
    let missing_observation = trigger(&mut missing, pixels, selected);
    let missing_attempts = usize::from(missing_observation.action.is_some());

    let mut closed_completion = initial.clone();
    closed_completion
        .install_decision_completion_junction(context)
        .expect("install CLOSED completion junction");
    let closed_return = closed_completion
        .return_decision_completion()
        .expect("E19 CLOSED completion control");

    let failed_attempts = opportunities
        .iter()
        .flat_map(|opportunity| &opportunity.attempts)
        .filter(|attempt| !attempt.consequence_admitted)
        .collect::<Vec<_>>();
    let successful_attempts = opportunities
        .iter()
        .flat_map(|opportunity| &opportunity.attempts)
        .filter(|attempt| attempt.consequence_admitted)
        .collect::<Vec<_>>();

    let mut closed_after_success = learned.clone();
    closed_after_success
        .install_decision_completion_junction(context)
        .expect("install post-success CLOSED completion junction");
    let after_success_return = closed_after_success
        .return_decision_completion()
        .expect("E19 post-success CLOSED completion");

    Controls {
        closed_silent: closed.decision_openness() == Arc3DecisionOpenness::Closed
            && closed_observation.action.is_none(),
        missing_completion_one_attempt: missing_attempts == 1,
        missing_completion_no_reactivation: missing.decision_openness()
            == Arc3DecisionOpenness::Open,
        closed_completion_delivered: closed_return.delivered && closed_return.physical_work > 0,
        closed_completion_ineligible: !closed_return.reactivation_eligible,
        failed_completion_zero_updates: !failed_attempts.is_empty()
            && failed_attempts.iter().all(|attempt| {
                attempt.completion_delivered
                    && attempt.consequence_updates == 0
                    && !attempt.consequence_admitted
            }),
        completion_preserves_open: failed_attempts.iter().all(|attempt| {
            attempt.open_before_completion == Arc3DecisionOpenness::Open
                && attempt.open_after_completion == Arc3DecisionOpenness::Open
        }),
        consequence_closes: successful_attempts.len() == 2
            && successful_attempts.iter().all(|attempt| {
                attempt.open_after_completion == Arc3DecisionOpenness::Open
                    && attempt.open_after_consequence == Arc3DecisionOpenness::Closed
                    && attempt.consequence_updates > 0
            }),
        closed_after_success_completion_ineligible: after_success_return.delivered
            && !after_success_return.reactivation_eligible
            && closed_after_success.decision_openness() == Arc3DecisionOpenness::Closed,
        marker_schema_exact: size_of::<Arc3DecisionOpenness>() == 1,
    }
}

fn execute(seed: usize, mechanics: MechanicalConfig, root: u64) -> Observation {
    let pixels = frame();
    let context = spatial_context(&pixels).expect("E19 context");
    let order = permutation(seed);
    let useful = useful_action(seed, order);
    let initial = unresolved(root, mechanics, &pixels);
    let mut base = initial.clone();
    let mut opportunities = Vec::new();
    let mut first_action = None;
    let mut work = 0_u64;
    let mut quiescent = true;
    for _ in 0..2 {
        let (opportunity, next) = run_opportunity(&base, &pixels, context, useful, order);
        if first_action.is_none() {
            first_action = opportunity
                .attempts
                .first()
                .and_then(|attempt| attempt.participated);
        }
        work = work.saturating_add(opportunity.work);
        quiescent &= opportunity.quiescent;
        opportunities.push(opportunity);
        let Some(next) = next else {
            break;
        };
        base = recovered(next);
    }
    while opportunities.len() < 2 {
        opportunities.push(Opportunity {
            attempts: Vec::new(),
            encountered_useful: false,
            completion_count: 0,
            reactivation_count: 0,
            consequence_updates: 0,
            open_at_start: Arc3DecisionOpenness::Closed,
            open_at_end: Arc3DecisionOpenness::Closed,
            work: 0,
            quiescent: true,
        });
    }
    let opportunities: [Opportunity; 2] = opportunities
        .try_into()
        .expect("exactly two E19 opportunities");
    let final_action = autonomous(&base, &pixels);
    let controls = controls(&initial, &pixels, context, order, &base, &opportunities);
    Observation {
        seed,
        order,
        useful,
        first_action,
        opportunities,
        final_action,
        controls,
        work,
        quiescent,
    }
}

fn bools(controls: &Controls) -> [bool; 10] {
    [
        controls.closed_silent,
        controls.missing_completion_one_attempt,
        controls.missing_completion_no_reactivation,
        controls.closed_completion_delivered,
        controls.closed_completion_ineligible,
        controls.failed_completion_zero_updates,
        controls.completion_preserves_open,
        controls.consequence_closes,
        controls.closed_after_success_completion_ineligible,
        controls.marker_schema_exact,
    ]
}

fn option(value: Option<u8>) -> String {
    value.map_or_else(|| "none".to_string(), |value| value.to_string())
}

fn list<T: ToString>(values: impl IntoIterator<Item = T>) -> String {
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn openness(value: Arc3DecisionOpenness) -> &'static str {
    match value {
        Arc3DecisionOpenness::Closed => "CLOSED",
        Arc3DecisionOpenness::Open => "OPEN",
    }
}

fn attempts(opportunity: &Opportunity) -> String {
    list(opportunity.attempts.iter().map(|attempt| attempt.attempted))
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.get(1).is_some_and(|value| value == "--preflight") {
        let seed = arguments
            .get(2)
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(7);
        let mechanics = if arguments.get(3).is_some_and(|value| value == "production") {
            MechanicalConfig::PRODUCTION
        } else {
            MechanicalConfig::REFERENCE
        };
        println!(
            "{:#?}",
            execute(seed, mechanics, 119_000_000 + seed as u64 * 100_000)
        );
        return;
    }

    eprintln!("CORE1_E19_TEMPORARY_DECISION_CONTINUATION_V1_EVIDENCE_SPENT");
    let destination = arguments.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/core1_e19_temporary_decision_continuation_v1")
    });
    fs::create_dir_all(&destination).expect("create E19 results");
    let mut csv = BufWriter::new(File::create(destination.join("matrix.csv")).expect("matrix"));
    writeln!(csv, "seed,mechanics,order,useful,first_action,attempted_1,attempted_2,completions_1,completions_2,reactivations_1,reactivations_2,updates_1,updates_2,open_start_1,open_end_1,open_start_2,open_end_2,final_action,work,quiescent,controls,replay_exact,mechanics_exact,passed").expect("header");

    let mut first = 0_usize;
    let mut encountered = 0_usize;
    let mut learned = 0_usize;
    let mut replay_all = true;
    let mut mechanics_all = true;
    let mut quiescent_all = true;
    let mut controls_all = true;
    let mut bounded_all = true;
    for seed in 0..SEEDS {
        let root = 119_000_000_u64 + seed as u64 * 100_000;
        let reference = execute(seed, MechanicalConfig::REFERENCE, root);
        let replay = execute(seed, MechanicalConfig::REFERENCE, root);
        let production = execute(seed, MechanicalConfig::PRODUCTION, root);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        let row_controls = bools(&reference.controls).into_iter().all(|gate| gate);
        let bounded = reference
            .opportunities
            .iter()
            .all(|opportunity| opportunity.completion_count <= MAX_COMPLETIONS);
        let row_passed = reference.first_action.is_some()
            && reference
                .opportunities
                .iter()
                .all(|opportunity| opportunity.encountered_useful)
            && reference.final_action == Some(reference.useful)
            && row_controls
            && bounded
            && reference.quiescent
            && replay_exact
            && mechanics_exact;
        first += usize::from(reference.first_action.is_some());
        encountered += usize::from(
            reference
                .opportunities
                .iter()
                .all(|opportunity| opportunity.encountered_useful),
        );
        learned += usize::from(reference.final_action == Some(reference.useful));
        replay_all &= replay_exact;
        mechanics_all &= mechanics_exact;
        quiescent_all &= reference.quiescent;
        controls_all &= row_controls;
        bounded_all &= bounded;
        for (mechanics, observation) in [
            ("reference", &reference),
            ("replay", &replay),
            ("production", &production),
        ] {
            writeln!(
                csv,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                seed,
                mechanics,
                list(observation.order),
                observation.useful,
                option(observation.first_action),
                attempts(&observation.opportunities[0]),
                attempts(&observation.opportunities[1]),
                observation.opportunities[0].completion_count,
                observation.opportunities[1].completion_count,
                observation.opportunities[0].reactivation_count,
                observation.opportunities[1].reactivation_count,
                observation.opportunities[0].consequence_updates,
                observation.opportunities[1].consequence_updates,
                openness(observation.opportunities[0].open_at_start),
                openness(observation.opportunities[0].open_at_end),
                openness(observation.opportunities[1].open_at_start),
                openness(observation.opportunities[1].open_at_end),
                option(observation.final_action),
                observation.work,
                observation.quiescent,
                list(bools(&observation.controls).map(u8::from)),
                replay_exact,
                mechanics_exact,
                row_passed,
            )
            .expect("row");
        }
        eprintln!(
            "E19 seed={} useful={} first={} attempts={}/{} final={} controls={} replay={} mechanics={} pass={}",
            seed,
            reference.useful,
            option(reference.first_action),
            attempts(&reference.opportunities[0]),
            attempts(&reference.opportunities[1]),
            option(reference.final_action),
            row_controls,
            replay_exact,
            mechanics_exact,
            row_passed,
        );
    }
    csv.flush().expect("flush E19 matrix");

    let passed = first == SEEDS
        && encountered == SEEDS
        && learned == SEEDS
        && replay_all
        && mechanics_all
        && quiescent_all
        && controls_all
        && bounded_all;
    let report = format!(
        "# CORE1 E19 temporary decision continuation v1\n\n- first physical action: `{first}/{SEEDS}`\n- useful route encountered in both opportunities: `{encountered}/{SEEDS}`\n- useful policy learned: `{learned}/{SEEDS}`\n- bounded attempts: `{bounded_all}`\n- mandatory controls: `{controls_all}`\n- exact replay: `{replay_all}`\n- Reference/Production exact: `{mechanics_all}`\n- natural quiescence: `{quiescent_all}`\n- primary matrix: `{passed}`\n"
    );
    fs::write(destination.join("report.md"), report).expect("write E19 report");
    println!(
        "CORE1_E19_TEMPORARY_DECISION_CONTINUATION_V1_COMPLETE first={first}/{SEEDS} encountered={encountered}/{SEEDS} learned={learned}/{SEEDS} controls={controls_all} bounded={bounded_all} replay={replay_all} mechanics={mechanics_all} quiescent={quiescent_all} pass={passed}"
    );
    assert!(passed, "E19 primary matrix failed; inspect streamed rows");
}
