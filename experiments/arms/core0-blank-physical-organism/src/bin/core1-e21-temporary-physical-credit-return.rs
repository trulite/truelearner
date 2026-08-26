#![forbid(unsafe_code)]

use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
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
    returns_after_route: usize,
    passive_pending_after_route: usize,
    completion_delivered: bool,
    variation_cycles: u8,
    returns_after_completion: usize,
    passive_pending_after_completion: usize,
    consequence_admitted: bool,
    modulatory: u64,
    updates: u64,
    returns_after_consequence: usize,
    open_after_completion: Arc3DecisionOpenness,
    open_after_consequence: Arc3DecisionOpenness,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Opportunity {
    attempts: Vec<Attempt>,
    encountered_useful: bool,
    updates: u64,
    modulatory: u64,
    open_at_end: Arc3DecisionOpenness,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Controls {
    closed_silent_no_return: bool,
    missing_completion_one_attempt: bool,
    missing_completion_return_survives: bool,
    closed_completion_delivered: bool,
    closed_completion_ineligible: bool,
    closed_completion_zero_cycles: bool,
    completion_preserves_returns: bool,
    passive_pending_zero: bool,
    consequence_closes: bool,
    consequence_clears_returns: bool,
    post_success_completion_ineligible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    seed: usize,
    order: [u8; 4],
    useful: u8,
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
        let context = spatial_context(&candidate).expect("valid E21 frame");
        if contexts.insert(context) {
            return candidate;
        }
    }
    panic!("one E21 frame must exist")
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
        .expect("ordinary E21 interaction recovery");
    organism
}

fn unresolved(root: u64, mechanics: MechanicalConfig, pixels: &[u8]) -> Arc3Sensorimotor {
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, Core0Profile::GenericExternal)
            .expect("CORE1-B E21 body");
    for _ in 0..2 {
        let prime = organism
            .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
            .expect("E21 candidate prime");
        assert!(prime.action.is_none(), "E21 prime must remain unresolved");
        assert!(prime.naturally_quiescent, "E21 prime must quiesce");
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

fn trigger(
    organism: &mut Arc3Sensorimotor,
    pixels: &[u8],
    action: u8,
) -> Arc3SensorimotorObservation {
    organism
        .trigger_transient_continuation(pixels.to_vec(), action, &ACTIONS, 1)
        .unwrap_or_else(|error| panic!("E21 internal action {action} failed: {error}"))
}

fn autonomous(organism: &Arc3Sensorimotor, pixels: &[u8]) -> Option<u8> {
    let mut probe = recovered(organism.clone());
    probe
        .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("E21 autonomous probe")
        .action
}

fn run_opportunity(
    base: &Arc3Sensorimotor,
    pixels: &[u8],
    context: u16,
    useful: u8,
    order: [u8; 4],
) -> (Opportunity, Option<Arc3Sensorimotor>) {
    let mut organism = base.clone();
    organism.enable_physical_credit_return();
    organism
        .open_decision_interaction(context)
        .expect("install and OPEN E21 interaction");
    let initial = organism
        .observe_open_decision(pixels.to_vec(), &ACTIONS, &ACTIONS)
        .expect("E21 initial junction activation");
    let mut pending_action = initial.action;
    let mut refractory = HashSet::new();
    let mut attempts = Vec::new();
    let mut work = initial.physical_work;
    let mut quiescent = initial.naturally_quiescent;

    while attempts.len() < MAX_COMPLETIONS {
        let selected = pending_action
            .take()
            .or_else(|| select_action(&organism, context, order, &refractory));
        let Some(selected) = selected else {
            break;
        };
        let action = if initial.action == Some(selected) && attempts.is_empty() {
            initial.clone()
        } else {
            let action = trigger(&mut organism, pixels, selected);
            work = work.saturating_add(action.physical_work);
            quiescent &= action.naturally_quiescent;
            action
        };
        let Some(participated) = action.action else {
            break;
        };
        refractory.insert(participated);
        let returns_after_route = organism.temporary_credit_return_count();
        let passive_pending_after_route = organism.used_pending_count();
        let completion = organism
            .return_decision_completion()
            .expect("E21 positive completion return");
        work = work.saturating_add(completion.physical_work);
        quiescent &= completion.naturally_quiescent;
        let returns_after_completion = organism.temporary_credit_return_count();
        let passive_pending_after_completion = organism.used_pending_count();
        let open_after_completion = organism.decision_openness();
        let mut attempt = Attempt {
            attempted: selected,
            participated: Some(participated),
            returns_after_route,
            passive_pending_after_route,
            completion_delivered: completion.delivered,
            variation_cycles: completion.variation_cycles,
            returns_after_completion,
            passive_pending_after_completion,
            consequence_admitted: false,
            modulatory: 0,
            updates: 0,
            returns_after_consequence: returns_after_completion,
            open_after_completion,
            open_after_consequence: open_after_completion,
        };
        if participated == useful {
            let consequence = organism
                .admit_open_decision_consequence()
                .expect("ordinary E21 consequence");
            work = work.saturating_add(consequence.physical_work);
            quiescent &= consequence.naturally_quiescent;
            attempt.consequence_admitted = consequence.admitted;
            attempt.modulatory = consequence.modulatory_deliveries;
            attempt.updates = consequence.plasticity_updates;
            attempt.returns_after_consequence = organism.temporary_credit_return_count();
            attempt.open_after_consequence = organism.decision_openness();
            let updates = consequence.plasticity_updates;
            let modulatory = consequence.modulatory_deliveries;
            attempts.push(attempt);
            return (
                Opportunity {
                    attempts,
                    encountered_useful: true,
                    updates,
                    modulatory,
                    open_at_end: organism.decision_openness(),
                    work,
                    quiescent,
                },
                Some(organism),
            );
        }
        attempts.push(attempt);
    }

    (
        Opportunity {
            attempts,
            encountered_useful: false,
            updates: 0,
            modulatory: 0,
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
    closed.enable_physical_credit_return();
    let closed_observation = closed
        .observe(pixels.to_vec(), &ACTIONS, None, false, false, &ACTIONS)
        .expect("E21 CLOSED silence");

    let mut missing = initial.clone();
    missing.enable_physical_credit_return();
    missing
        .open_decision_interaction(context)
        .expect("E21 missing-completion OPEN");
    let selected = select_action(&missing, context, order, &HashSet::new())
        .expect("E21 missing-completion route");
    let missing_action = trigger(&mut missing, pixels, selected);

    let mut closed_completion = initial.clone();
    closed_completion.enable_physical_credit_return();
    closed_completion
        .install_decision_completion_junction(context)
        .expect("E21 CLOSED completion junction");
    let closed_return = closed_completion
        .return_decision_completion()
        .expect("E21 CLOSED completion");

    let attempts = opportunities
        .iter()
        .flat_map(|opportunity| &opportunity.attempts)
        .collect::<Vec<_>>();
    let successes = attempts
        .iter()
        .filter(|attempt| attempt.consequence_admitted)
        .collect::<Vec<_>>();

    let mut closed_after_success = learned.clone();
    let after_success = closed_after_success
        .return_decision_completion()
        .expect("E21 post-success completion");

    Controls {
        closed_silent_no_return: closed_observation.action.is_none()
            && closed.temporary_credit_return_count() == 0,
        missing_completion_one_attempt: missing_action.action.is_some(),
        missing_completion_return_survives: missing.temporary_credit_return_count() > 0
            && missing.decision_openness() == Arc3DecisionOpenness::Open,
        closed_completion_delivered: closed_return.delivered,
        closed_completion_ineligible: !closed_return.reactivation_eligible,
        closed_completion_zero_cycles: closed_return.variation_cycles == 0,
        completion_preserves_returns: attempts.iter().all(|attempt| {
            attempt.returns_after_route > 0
                && attempt.returns_after_completion == attempt.returns_after_route
        }),
        passive_pending_zero: attempts.iter().all(|attempt| {
            attempt.passive_pending_after_route == 0
                && attempt.passive_pending_after_completion == 0
        }),
        consequence_closes: successes.len() == 2
            && successes.iter().all(|attempt| {
                attempt.open_after_completion == Arc3DecisionOpenness::Open
                    && attempt.open_after_consequence == Arc3DecisionOpenness::Closed
            }),
        consequence_clears_returns: successes.len() == 2
            && successes
                .iter()
                .all(|attempt| attempt.returns_after_consequence == 0),
        post_success_completion_ineligible: after_success.delivered
            && !after_success.reactivation_eligible
            && after_success.variation_cycles == 0,
    }
}

fn execute(seed: usize, mechanics: MechanicalConfig, root: u64) -> Observation {
    let pixels = frame();
    let context = spatial_context(&pixels).expect("E21 context");
    let order = permutation(seed);
    let useful = useful_action(seed, order);
    let initial = unresolved(root, mechanics, &pixels);
    let mut base = initial.clone();
    let mut opportunities = Vec::new();
    let mut work = 0_u64;
    let mut quiescent = true;
    for _ in 0..2 {
        let (opportunity, next) = run_opportunity(&base, &pixels, context, useful, order);
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
            updates: 0,
            modulatory: 0,
            open_at_end: Arc3DecisionOpenness::Closed,
            work: 0,
            quiescent: true,
        });
    }
    let opportunities: [Opportunity; 2] = opportunities
        .try_into()
        .expect("exactly two E21 opportunities");
    let final_action = autonomous(&base, &pixels);
    let controls = controls(&initial, &pixels, context, order, &base, &opportunities);
    Observation {
        seed,
        order,
        useful,
        opportunities,
        final_action,
        controls,
        work,
        quiescent,
    }
}

fn gates(controls: &Controls) -> [bool; 11] {
    [
        controls.closed_silent_no_return,
        controls.missing_completion_one_attempt,
        controls.missing_completion_return_survives,
        controls.closed_completion_delivered,
        controls.closed_completion_ineligible,
        controls.closed_completion_zero_cycles,
        controls.completion_preserves_returns,
        controls.passive_pending_zero,
        controls.consequence_closes,
        controls.consequence_clears_returns,
        controls.post_success_completion_ineligible,
    ]
}

fn attempts(opportunity: &Opportunity) -> String {
    opportunity
        .attempts
        .iter()
        .map(|attempt| attempt.attempted.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn row_pass(observation: &Observation) -> bool {
    observation.opportunities.iter().all(|opportunity| {
        opportunity.encountered_useful
            && opportunity.attempts.len() <= MAX_COMPLETIONS
            && opportunity.modulatory > 0
            && opportunity.updates > 0
            && opportunity.open_at_end == Arc3DecisionOpenness::Closed
            && opportunity.quiescent
    }) && observation.final_action == Some(observation.useful)
        && gates(&observation.controls).into_iter().all(|gate| gate)
        && observation.quiescent
}

fn exact(seed: usize) -> (Observation, bool, bool, bool) {
    let root = 121_000_000_u64 + seed as u64 * 100_000;
    let reference = execute(seed, MechanicalConfig::REFERENCE, root);
    let replay = execute(seed, MechanicalConfig::REFERENCE, root);
    let production = execute(seed, MechanicalConfig::PRODUCTION, root);
    let replay_exact = reference == replay;
    let mechanics_exact = reference == production;
    let passed = row_pass(&reference) && replay_exact && mechanics_exact;
    (reference, replay_exact, mechanics_exact, passed)
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.get(1).is_some_and(|value| value == "--preflight") {
        let seed = arguments
            .get(2)
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(0);
        let mechanics = if arguments.get(3).is_some_and(|value| value == "production") {
            MechanicalConfig::PRODUCTION
        } else {
            MechanicalConfig::REFERENCE
        };
        println!(
            "{:#?}",
            execute(seed, mechanics, 121_000_000 + seed as u64 * 100_000)
        );
        return;
    }

    let (credit, credit_replay, credit_mechanics, credit_pass) = exact(0);
    eprintln!(
        "E21 P2={} mod={}|{} updates={}|{} final={:?} replay={} mechanics={}",
        credit_pass,
        credit.opportunities[0].modulatory,
        credit.opportunities[1].modulatory,
        credit.opportunities[0].updates,
        credit.opportunities[1].updates,
        credit.final_action,
        credit_replay,
        credit_mechanics
    );
    assert!(credit_pass, "E21 P2 failed; P1 and matrix ineligible");

    let (hard, hard_replay, hard_mechanics, hard_pass) = exact(7);
    eprintln!(
        "E21 P1={} attempts={}/{} replay={} mechanics={}",
        hard_pass,
        attempts(&hard.opportunities[0]),
        attempts(&hard.opportunities[1]),
        hard_replay,
        hard_mechanics
    );
    assert!(hard_pass, "E21 P1 failed; matrix ineligible");

    eprintln!("CORE1_E21_TEMPORARY_PHYSICAL_CREDIT_RETURN_V1_EVIDENCE_SPENT");
    let destination = arguments.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/core1_e21_temporary_physical_credit_return_v1")
    });
    fs::create_dir_all(&destination).expect("create E21 results");
    let mut csv = BufWriter::new(File::create(destination.join("matrix.csv")).expect("matrix"));
    writeln!(csv, "seed,mechanics,order,useful,attempts_1,attempts_2,modulatory_1,modulatory_2,updates_1,updates_2,final_action,work,quiescent,controls,replay_exact,mechanics_exact,passed").expect("header");

    let mut passed_rows = 0_usize;
    let mut replay_all = true;
    let mut mechanics_all = true;
    for seed in 0..SEEDS {
        let root = 121_000_000_u64 + seed as u64 * 100_000;
        let reference = execute(seed, MechanicalConfig::REFERENCE, root);
        let replay = execute(seed, MechanicalConfig::REFERENCE, root);
        let production = execute(seed, MechanicalConfig::PRODUCTION, root);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        let passed = row_pass(&reference) && replay_exact && mechanics_exact;
        passed_rows += usize::from(passed);
        replay_all &= replay_exact;
        mechanics_all &= mechanics_exact;
        for (mechanics, observation) in [
            ("reference", &reference),
            ("replay", &replay),
            ("production", &production),
        ] {
            writeln!(
                csv,
                "{},{},{},{},{},{},{},{},{},{},{:?},{},{},{},{},{},{}",
                seed,
                mechanics,
                observation
                    .order
                    .iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join("|"),
                observation.useful,
                attempts(&observation.opportunities[0]),
                attempts(&observation.opportunities[1]),
                observation.opportunities[0].modulatory,
                observation.opportunities[1].modulatory,
                observation.opportunities[0].updates,
                observation.opportunities[1].updates,
                observation.final_action,
                observation.work,
                observation.quiescent,
                gates(&observation.controls)
                    .map(u8::from)
                    .iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join("|"),
                replay_exact,
                mechanics_exact,
                passed,
            )
            .expect("row");
        }
        eprintln!(
            "E21 seed={} useful={} attempts={}/{} mod={}/{} updates={}/{} final={:?} replay={} mechanics={} pass={}",
            seed,
            reference.useful,
            attempts(&reference.opportunities[0]),
            attempts(&reference.opportunities[1]),
            reference.opportunities[0].modulatory,
            reference.opportunities[1].modulatory,
            reference.opportunities[0].updates,
            reference.opportunities[1].updates,
            reference.final_action,
            replay_exact,
            mechanics_exact,
            passed
        );
    }
    csv.flush().expect("flush E21 matrix");
    let passed = passed_rows == SEEDS && replay_all && mechanics_all;
    let report = format!(
        "# CORE1 E21 temporary physical credit return v1\n\n- primary seeds: `{passed_rows}/{SEEDS}`\n- exact replay: `{replay_all}`\n- Reference/Production exact: `{mechanics_all}`\n- primary matrix: `{passed}`\n"
    );
    fs::write(destination.join("report.md"), report).expect("write E21 report");
    println!(
        "CORE1_E21_TEMPORARY_PHYSICAL_CREDIT_RETURN_V1_COMPLETE seeds={passed_rows}/{SEEDS} replay={replay_all} mechanics={mechanics_all} pass={passed}"
    );
    assert!(passed, "E21 primary matrix failed; inspect streamed rows");
}
