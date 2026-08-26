#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use academy_arc3::{
    spatial_context, Arc3ContextDiagnostic, Arc3Sensorimotor, Arc3SensorimotorObservation,
    ARC3_FRAME_PIXELS,
};
use truelearner_core::{CellId, Core0Profile, MechanicalConfig, PhysicalEvent, PhysicalTransition};

const ACTIONS: [u8; 4] = [1, 4, 2, 3];
const ACTION_MAP: [u8; 4] = [1, 2, 3, 4];
const ROOT: u64 = 93_000_000;

#[derive(Clone, Debug, PartialEq, Eq)]
struct FirstTurn {
    observation: Arc3SensorimotorObservation,
    before: Arc3ContextDiagnostic,
    after: Arc3ContextDiagnostic,
    trace: Vec<PhysicalTransition>,
    used_pending: usize,
    credit_returns: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FullObservation {
    turns: Vec<Arc3SensorimotorObservation>,
    pending: Vec<usize>,
    credit_returns: Vec<usize>,
    probes: Vec<Option<u8>>,
    pass: bool,
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
        let context = spatial_context(&candidate).expect("valid frozen E14 frame");
        if contexts.insert(context) {
            frames.push(candidate);
            if frames.len() == 5 {
                break;
            }
        }
    }
    assert_eq!(frames.len(), 5, "five frozen E14 frames");
    frames
}

fn organism(mechanics: MechanicalConfig) -> Arc3Sensorimotor {
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(ROOT, mechanics, Core0Profile::GenericExternal)
            .expect("CORE1-B E24 body");
    organism.enable_atomic_route_closure();
    organism
}

fn complete_contacts(diagnostic: &Arc3ContextDiagnostic) -> BTreeSet<CellId> {
    let stems = diagnostic
        .links
        .iter()
        .filter(|link| link.role == "stem")
        .filter_map(|link| link.contact)
        .collect::<BTreeSet<_>>();
    let outgoings = diagnostic
        .links
        .iter()
        .filter(|link| link.role == "outgoing")
        .filter_map(|link| link.contact)
        .collect::<BTreeSet<_>>();
    stems.intersection(&outgoings).copied().collect()
}

fn signed_pairs(diagnostic: &Arc3ContextDiagnostic, sign: i64) -> usize {
    let contacts = complete_contacts(diagnostic);
    contacts
        .iter()
        .filter(|contact| {
            diagnostic.links.iter().any(|link| {
                link.role == "outgoing"
                    && link.contact == Some(**contact)
                    && link.coupling.signum() == sign
            })
        })
        .count()
}

fn paired_proposals(turn: &FirstTurn) -> usize {
    let proposed_contacts = turn
        .trace
        .iter()
        .filter_map(|transition| match transition.event {
            PhysicalEvent::CellProposal {
                cell,
                source,
                target,
            } if source == turn.after.source && target == turn.after.target => Some(cell),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    proposed_contacts
        .into_iter()
        .filter(|contact| {
            let stem = turn.trace.iter().any(|transition| {
                matches!(
                    transition.event,
                    PhysicalEvent::Proposal { from, to, .. }
                        if from == turn.after.source && to == *contact
                )
            });
            let outgoing = turn.trace.iter().any(|transition| {
                matches!(
                    transition.event,
                    PhysicalEvent::Proposal { from, to, .. }
                        if from == *contact && to == turn.after.target
                )
            });
            stem && outgoing
        })
        .count()
}

fn fire_count(trace: &[PhysicalTransition], cell: CellId) -> usize {
    trace
        .iter()
        .filter(|transition| {
            matches!(transition.event, PhysicalEvent::Fire { cell: fired } if fired == cell)
        })
        .count()
}

fn first_turn(mechanics: MechanicalConfig) -> FirstTurn {
    let pixels = frames().remove(0);
    let context = spatial_context(&pixels).expect("E24 context");
    let mut organism = organism(mechanics);
    let before = organism
        .diagnostic_context(context, 0)
        .expect("E24 before diagnostic");
    let observation = organism
        .observe(pixels, &ACTION_MAP, Some(1), false, false, &ACTION_MAP)
        .expect("E24 first frozen E14 turn");
    let trace = organism.last_action_physical_trace().to_vec();
    let after = organism
        .diagnostic_context(context, 0)
        .expect("E24 after diagnostic");
    FirstTurn {
        observation,
        before,
        after,
        trace,
        used_pending: organism.used_pending_count(),
        credit_returns: organism.temporary_credit_return_count(),
    }
}

fn recover(mut organism: Arc3Sensorimotor) -> Arc3Sensorimotor {
    organism.clear_episode();
    organism.advance_gap(1).expect("ordinary E24 recovery");
    organism
}

fn full(mechanics: MechanicalConfig) -> FullObservation {
    let frames = frames();
    let mut organism = organism(mechanics);
    let mut turns = Vec::with_capacity(5);
    let mut pending = Vec::with_capacity(5);
    let mut credit_returns = Vec::with_capacity(5);
    for (index, action) in ACTIONS.into_iter().enumerate() {
        turns.push(
            organism
                .observe(
                    frames[index].clone(),
                    &ACTION_MAP,
                    Some(action),
                    index > 0,
                    false,
                    &ACTION_MAP,
                )
                .expect("E24 frozen E14 teaching observation"),
        );
        pending.push(organism.used_pending_count());
        credit_returns.push(organism.temporary_credit_return_count());
    }
    turns.push(
        organism
            .observe(
                frames[4].clone(),
                &ACTION_MAP,
                None,
                true,
                false,
                &ACTION_MAP,
            )
            .expect("E24 frozen E14 closing observation"),
    );
    pending.push(organism.used_pending_count());
    credit_returns.push(organism.temporary_credit_return_count());

    let probes = frames
        .iter()
        .take(4)
        .map(|frame| {
            recover(organism.clone())
                .observe(frame.clone(), &ACTION_MAP, None, false, false, &ACTION_MAP)
                .expect("E24 autonomous probe")
                .action
        })
        .collect::<Vec<_>>();
    let actions = turns
        .iter()
        .take(4)
        .map(|turn| turn.action)
        .collect::<Vec<_>>();
    let updates = turns
        .iter()
        .map(|turn| turn.plasticity_updates)
        .collect::<Vec<_>>();
    let pass = actions == [Some(1), Some(4), Some(2), Some(3)]
        && updates == [0, 1, 1, 1, 1]
        && turns
            .iter()
            .skip(1)
            .any(|turn| turn.modulatory_deliveries > 0)
        && turns.iter().all(|turn| turn.naturally_quiescent)
        && pending.iter().all(|count| *count == 0)
        && probes == [Some(1), Some(4), Some(2), Some(3)];
    FullObservation {
        turns,
        pending,
        credit_returns,
        probes,
        pass,
    }
}

fn list<T: ToString>(values: impl IntoIterator<Item = T>) -> String {
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn options(values: impl IntoIterator<Item = Option<u8>>) -> String {
    list(
        values
            .into_iter()
            .map(|value| value.map_or_else(|| "none".to_string(), |action| action.to_string())),
    )
}

fn write_preflight(
    csv: &mut String,
    mechanics: &str,
    turn: &FirstTurn,
    replay_exact: bool,
    mechanics_exact: bool,
) {
    writeln!(
        csv,
        "{mechanics},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        turn.observation.context,
        fire_count(&turn.trace, turn.after.source),
        fire_count(&turn.trace, turn.after.context_trace),
        fire_count(&turn.trace, turn.after.babbler),
        complete_contacts(&turn.before).len(),
        complete_contacts(&turn.after).len(),
        paired_proposals(turn),
        signed_pairs(&turn.after, 1),
        signed_pairs(&turn.after, -1),
        turn.observation
            .action
            .map_or_else(|| "none".to_string(), |action| action.to_string()),
        turn.observation.modulatory_deliveries,
        turn.observation.plasticity_updates,
        turn.used_pending,
        turn.credit_returns,
        turn.observation.physical_work,
        turn.observation.physical_tick,
        turn.observation.naturally_quiescent,
        turn.trace.len(),
        replay_exact,
        mechanics_exact,
    )
    .expect("write E24 preflight");
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.get(1).is_some_and(|value| value == "--check") {
        println!(
            "CORE1_E24_ATOMIC_ROUTE_CLOSURE_V1_OBSERVER_OK root={ROOT} actions={}",
            list(ACTIONS)
        );
        return;
    }

    eprintln!("CORE1_E24_ATOMIC_ROUTE_CLOSURE_V1_EVIDENCE_SPENT");
    let destination = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_e24_atomic_route_closure_v1"));
    fs::create_dir_all(&destination).expect("create E24 result directory");

    let reference = first_turn(MechanicalConfig::REFERENCE);
    let replay = first_turn(MechanicalConfig::REFERENCE);
    let production = first_turn(MechanicalConfig::PRODUCTION);
    let replay_exact = reference == replay;
    let mechanics_exact = reference == production;
    let formation = paired_proposals(&reference) > 0
        && !complete_contacts(&reference.after).is_empty()
        && signed_pairs(&reference.after, 1) > 0
        && signed_pairs(&reference.after, -1) > 0
        && reference.used_pending == 0
        && reference.observation.naturally_quiescent
        && replay_exact
        && mechanics_exact;
    let participation = formation && reference.observation.action.is_some();

    let mut preflight = String::from(
        "mechanics,context,source_fires,context_trace_fires,babbler_fires,pairs_before,pairs_after,paired_proposals,positive_pairs,negative_pairs,action,modulatory,updates,used_pending,credit_returns,physical_work,physical_tick,quiescent,trace_events,replay_exact,mechanics_exact\n",
    );
    for (mechanics, turn) in [
        ("reference", &reference),
        ("replay", &replay),
        ("production", &production),
    ] {
        write_preflight(
            &mut preflight,
            mechanics,
            turn,
            replay_exact,
            mechanics_exact,
        );
    }
    fs::write(destination.join("preflight.csv"), preflight).expect("write E24 preflight");

    if !participation {
        let status = if formation {
            "FALSIFIED_AT_GATE_2"
        } else {
            "FAILED_AT_GATE_1"
        };
        let report = format!(
            "# CORE1 E24 atomic route closure result\n\n- status: `{status}`\n- formation: `{formation}`\n- paired proposals: `{}`\n- complete pairs after: `{}`\n- positive/negative pairs: `{}/{}`\n- first action: `{}`\n- Modulatory deliveries: `{}`\n- PQLC updates: `{}`\n- exact replay: `{replay_exact}`\n- Reference/Production exact: `{mechanics_exact}`\n- natural quiescence: `{}`\n- Gate 3 run: `false`\n",
            paired_proposals(&reference),
            complete_contacts(&reference.after).len(),
            signed_pairs(&reference.after, 1),
            signed_pairs(&reference.after, -1),
            options([reference.observation.action]),
            reference.observation.modulatory_deliveries,
            reference.observation.plasticity_updates,
            reference.observation.naturally_quiescent,
        );
        fs::write(destination.join("report.md"), report).expect("write E24 stopped report");
        println!(
            "CORE1_E24_ATOMIC_ROUTE_CLOSURE_V1_STOPPED status={status} formation={formation} pairs={} action={} replay={} mechanics={}",
            complete_contacts(&reference.after).len(),
            options([reference.observation.action]),
            replay_exact,
            mechanics_exact,
        );
        return;
    }

    let full_reference = full(MechanicalConfig::REFERENCE);
    let full_replay = full(MechanicalConfig::REFERENCE);
    let full_production = full(MechanicalConfig::PRODUCTION);
    let full_replay_exact = full_reference == full_replay;
    let full_mechanics_exact = full_reference == full_production;
    let pass = full_reference.pass && full_replay_exact && full_mechanics_exact;
    let mut matrix = String::from(
        "mechanics,pass,actions,updates,modulatory,pending,credit_returns,probes,quiescent,replay_exact,mechanics_exact\n",
    );
    for (mechanics, observation) in [
        ("reference", &full_reference),
        ("replay", &full_replay),
        ("production", &full_production),
    ] {
        writeln!(
            matrix,
            "{mechanics},{},{},{},{},{},{},{},{},{},{}",
            observation.pass,
            options(observation.turns.iter().map(|turn| turn.action)),
            list(observation.turns.iter().map(|turn| turn.plasticity_updates)),
            list(
                observation
                    .turns
                    .iter()
                    .map(|turn| turn.modulatory_deliveries)
            ),
            list(&observation.pending),
            list(&observation.credit_returns),
            options(observation.probes.iter().copied()),
            observation
                .turns
                .iter()
                .all(|turn| turn.naturally_quiescent),
            full_replay_exact,
            full_mechanics_exact,
        )
        .expect("write E24 matrix");
    }
    fs::write(destination.join("matrix.csv"), matrix).expect("write E24 matrix");
    let report = format!(
        "# CORE1 E24 atomic route closure result\n\n- status: `{}`\n- formation: `true`\n- participation: `true`\n- actions: `{}`\n- updates: `{}`\n- probes: `{}`\n- exact replay: `{full_replay_exact}`\n- Reference/Production exact: `{full_mechanics_exact}`\n",
        if pass { "POSITIVE" } else { "NEGATIVE_AT_GATE_3" },
        options(full_reference.turns.iter().map(|turn| turn.action)),
        list(
            full_reference
                .turns
                .iter()
                .map(|turn| turn.plasticity_updates)
        ),
        options(full_reference.probes.iter().copied()),
    );
    fs::write(destination.join("report.md"), report).expect("write E24 report");
    println!(
        "CORE1_E24_ATOMIC_ROUTE_CLOSURE_V1_COMPLETE pass={pass} replay={} mechanics={}",
        full_replay_exact, full_mechanics_exact,
    );
}
