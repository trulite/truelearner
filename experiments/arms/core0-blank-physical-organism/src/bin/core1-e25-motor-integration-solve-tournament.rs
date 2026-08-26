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
const ROOT_BASE: u64 = 93_000_000;
const SEEDS: [u64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    G,
    W,
    Gw,
}

impl Arm {
    const ALL: [Self; 3] = [Self::G, Self::W, Self::Gw];

    const fn label(self) -> &'static str {
        match self {
            Self::G => "G",
            Self::W => "W",
            Self::Gw => "G+W",
        }
    }

    const fn gating(self) -> bool {
        matches!(self, Self::G | Self::Gw)
    }

    const fn window(self) -> bool {
        matches!(self, Self::W | Self::Gw)
    }
}

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

#[derive(Clone, Debug)]
struct ArmSummary {
    arm: Arm,
    gate1_passes: usize,
    gate1_exact: bool,
    full_passes: usize,
    full_exact: bool,
    advanced: bool,
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

fn organism(root: u64, mechanics: MechanicalConfig, arm: Arm) -> Arc3Sensorimotor {
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, Core0Profile::GenericExternal)
            .expect("CORE1-B E25 body");
    organism.enable_atomic_route_closure();
    if arm.gating() {
        organism.enable_local_signed_gating();
    }
    if arm.window() {
        organism.enable_motor_integration_window();
    }
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

fn gate_counts(trace: &[PhysicalTransition]) -> (usize, usize, usize, usize) {
    let mut opportunity = 0;
    let mut positive = 0;
    let mut negative = 0;
    let mut none = 0;
    for transition in trace {
        if let PhysicalEvent::SignedGateCompetition {
            opportunity_active,
            admitted_sign,
            ..
        } = transition.event
        {
            opportunity += usize::from(opportunity_active);
            match admitted_sign {
                1 => positive += 1,
                -1 => negative += 1,
                _ => none += 1,
            }
        }
    }
    (opportunity, positive, negative, none)
}

fn integration_closes(trace: &[PhysicalTransition]) -> usize {
    trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::IntegrationWindowClosed { .. }
            )
        })
        .count()
}

fn motor_material(trace: &[PhysicalTransition], motor: CellId) -> String {
    list(
        trace
            .iter()
            .filter_map(|transition| match transition.event {
                PhysicalEvent::MaterialDriveIncidence {
                    target,
                    impulse,
                    activation_after,
                    ..
                } if target == motor => Some(format!(
                    "{}:{}:{}",
                    transition.tick, impulse, activation_after
                )),
                _ => None,
            }),
    )
}

fn first_turn(root: u64, mechanics: MechanicalConfig, arm: Arm) -> FirstTurn {
    let pixels = frames().remove(0);
    let context = spatial_context(&pixels).expect("E25 context");
    let mut organism = organism(root, mechanics, arm);
    let before = organism
        .diagnostic_context(context, 0)
        .expect("E25 before diagnostic");
    let observation = organism
        .observe(pixels, &ACTION_MAP, Some(1), false, false, &ACTION_MAP)
        .expect("E25 first frozen E14 turn");
    let trace = organism.last_action_physical_trace().to_vec();
    let after = organism
        .diagnostic_context(context, 0)
        .expect("E25 after diagnostic");
    FirstTurn {
        observation,
        before,
        after,
        trace,
        used_pending: organism.used_pending_count(),
        credit_returns: organism.temporary_credit_return_count(),
    }
}

fn formation_pass(turn: &FirstTurn) -> bool {
    paired_proposals(turn) > 0
        && !complete_contacts(&turn.after).is_empty()
        && signed_pairs(&turn.after, 1) > 0
        && signed_pairs(&turn.after, -1) > 0
        && turn.used_pending == 0
        && turn.observation.naturally_quiescent
}

fn gate1_pass(turn: &FirstTurn) -> bool {
    formation_pass(turn) && turn.observation.action == Some(1)
}

fn recover(mut organism: Arc3Sensorimotor) -> Arc3Sensorimotor {
    organism.clear_episode();
    organism.advance_gap(1).expect("ordinary E25 recovery");
    organism
}

fn full(root: u64, mechanics: MechanicalConfig, arm: Arm) -> FullObservation {
    let frames = frames();
    let mut organism = organism(root, mechanics, arm);
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
                .expect("E25 frozen E14 teaching observation"),
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
            .expect("E25 frozen E14 closing observation"),
    );
    pending.push(organism.used_pending_count());
    credit_returns.push(organism.temporary_credit_return_count());

    let probes = frames
        .iter()
        .take(4)
        .map(|frame| {
            recover(organism.clone())
                .observe(frame.clone(), &ACTION_MAP, None, false, false, &ACTION_MAP)
                .expect("E25 autonomous probe")
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
        && credit_returns.iter().take(4).all(|count| *count == 1)
        && credit_returns.last() == Some(&0)
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

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.get(1).is_some_and(|value| value == "--check") {
        println!(
            "CORE1_E25_MOTOR_INTEGRATION_SOLVE_TOURNAMENT_V1_OBSERVER_OK roots={} arms=G|W|G+W",
            list(SEEDS.map(|seed| ROOT_BASE + seed))
        );
        return;
    }

    eprintln!("CORE1_E25_MOTOR_INTEGRATION_SOLVE_TOURNAMENT_V1_EVIDENCE_SPENT");
    let destination = arguments.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/core1_e25_motor_integration_solve_tournament_v1")
    });
    fs::create_dir_all(&destination).expect("create E25 result directory");

    let mut preflight = String::from(
        "arm,root,mechanics,context,source_fires,context_trace_fires,babbler_fires,pairs_before,pairs_after,paired_proposals,positive_pairs,negative_pairs,gate_opportunities,gate_positive,gate_negative,gate_none,integration_closes,motor_material,action,modulatory,updates,used_pending,credit_returns,physical_work,physical_tick,quiescent,replay_exact,mechanics_exact,formation_pass,gate1_pass\n",
    );
    let mut full_csv = String::from(
        "arm,root,actions,updates,modulatory,credit_returns,used_pending,probes,quiescent,pass,replay_exact,mechanics_exact\n",
    );
    let mut summaries = Vec::new();

    for arm in Arm::ALL {
        let mut gate1_passes = 0;
        let mut gate1_exact = true;
        for seed in SEEDS {
            let root = ROOT_BASE + seed;
            let reference = first_turn(root, MechanicalConfig::REFERENCE, arm);
            let replay = first_turn(root, MechanicalConfig::REFERENCE, arm);
            let production = first_turn(root, MechanicalConfig::PRODUCTION, arm);
            let replay_exact = reference == replay;
            let mechanics_exact = reference == production;
            let exact = replay_exact && mechanics_exact;
            let pass = gate1_pass(&reference) && exact;
            gate1_passes += usize::from(pass);
            gate1_exact &= exact;
            for (mechanics, turn) in [
                ("reference", &reference),
                ("replay", &replay),
                ("production", &production),
            ] {
                let (gate_opportunities, gate_positive, gate_negative, gate_none) =
                    gate_counts(&turn.trace);
                writeln!(
                    preflight,
                    "{},{root},{mechanics},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                    arm.label(),
                    turn.observation.context,
                    fire_count(&turn.trace, turn.after.source),
                    fire_count(&turn.trace, turn.after.context_trace),
                    fire_count(&turn.trace, turn.after.babbler),
                    complete_contacts(&turn.before).len(),
                    complete_contacts(&turn.after).len(),
                    paired_proposals(turn),
                    signed_pairs(&turn.after, 1),
                    signed_pairs(&turn.after, -1),
                    gate_opportunities,
                    gate_positive,
                    gate_negative,
                    gate_none,
                    integration_closes(&turn.trace),
                    motor_material(&turn.trace, turn.after.target),
                    options([turn.observation.action]),
                    turn.observation.modulatory_deliveries,
                    turn.observation.plasticity_updates,
                    turn.used_pending,
                    turn.credit_returns,
                    turn.observation.physical_work,
                    turn.observation.physical_tick,
                    turn.observation.naturally_quiescent,
                    replay_exact,
                    mechanics_exact,
                    formation_pass(turn),
                    gate1_pass(turn),
                )
                .expect("write E25 preflight");
            }
        }

        let advanced = gate1_passes == SEEDS.len() && gate1_exact;
        let mut full_passes = 0;
        let mut full_exact = true;
        if advanced {
            for seed in SEEDS {
                let root = ROOT_BASE + seed;
                let reference = full(root, MechanicalConfig::REFERENCE, arm);
                let replay = full(root, MechanicalConfig::REFERENCE, arm);
                let production = full(root, MechanicalConfig::PRODUCTION, arm);
                let replay_exact = reference == replay;
                let mechanics_exact = reference == production;
                let exact = replay_exact && mechanics_exact;
                let pass = reference.pass && exact;
                full_passes += usize::from(pass);
                full_exact &= exact;
                writeln!(
                    full_csv,
                    "{},{root},{},{},{},{},{},{},{},{},{},{}",
                    arm.label(),
                    options(reference.turns.iter().take(4).map(|turn| turn.action)),
                    list(reference.turns.iter().map(|turn| turn.plasticity_updates)),
                    list(
                        reference
                            .turns
                            .iter()
                            .map(|turn| turn.modulatory_deliveries)
                    ),
                    list(&reference.credit_returns),
                    list(&reference.pending),
                    options(reference.probes.iter().copied()),
                    reference.turns.iter().all(|turn| turn.naturally_quiescent),
                    reference.pass,
                    replay_exact,
                    mechanics_exact,
                )
                .expect("write E25 full matrix");
            }
        }
        summaries.push(ArmSummary {
            arm,
            gate1_passes,
            gate1_exact,
            full_passes,
            full_exact,
            advanced,
        });
    }

    fs::write(destination.join("preflight.csv"), preflight).expect("write E25 preflight");
    fs::write(destination.join("full.csv"), full_csv).expect("write E25 full matrix");

    let solved = summaries
        .iter()
        .filter(|summary| summary.full_passes == SEEDS.len() && summary.full_exact)
        .map(|summary| summary.arm.label())
        .collect::<Vec<_>>();
    let interpretation = match solved.as_slice() {
        ["G+W"] => "ONLY_G_PLUS_W_PASSES_JOINTLY_CAUSAL",
        [] => "ALL_FAIL_COMBINED_HYPOTHESIS_WRONG",
        ["G"] => "G_ALONE_SUFFICIENT",
        ["W"] => "W_ALONE_SUFFICIENT",
        _ => "MULTIPLE_SOLVES_COMPARE_SMALLER_LAW",
    };
    let mut report = format!(
        "# CORE1 E25 motor integration solve tournament result\n\n- status: `{interpretation}`\n- frozen E24 baseline: `formation yes | action none | PQLC 0`\n- roots: `{}`\n\n## Arms\n\n",
        list(SEEDS.map(|seed| ROOT_BASE + seed)),
    );
    for summary in &summaries {
        writeln!(
            report,
            "- {}: Gate 1 `{}/8` (exact `{}`); full chain `{}/8` (exact `{}`); advanced `{}`",
            summary.arm.label(),
            summary.gate1_passes,
            summary.gate1_exact,
            summary.full_passes,
            summary.full_exact,
            summary.advanced,
        )
        .expect("write E25 report arm");
    }
    writeln!(
        report,
        "\nOnly an `8/8` full-chain arm is a solve. Gate-1-negative arms were not run through consequence or autonomy.\n"
    )
    .expect("write E25 report ending");
    fs::write(destination.join("report.md"), report).expect("write E25 report");

    println!(
        "CORE1_E25_MOTOR_INTEGRATION_SOLVE_TOURNAMENT_V1_COMPLETE status={interpretation} solved={} gate1={}",
        if solved.is_empty() { "none".to_string() } else { solved.join("|") },
        list(summaries.iter().map(|summary| format!(
            "{}:{}/8",
            summary.arm.label(),
            summary.gate1_passes
        ))),
    );
}
