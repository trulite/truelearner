#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use academy_arc3::{
    spatial_context, Arc3CandidateLinkDiagnostic, Arc3ContextDiagnostic, Arc3Sensorimotor,
    Arc3SensorimotorObservation, Arc3TransientHistoryRequest, ARC3_FRAME_PIXELS,
};
use truelearner_core::{
    ArrowId, CellId, Core0Profile, MechanicalConfig, PhysicalEvent, PhysicalTransition,
    TransmissionMode,
};

const ACTIONS: [u8; 4] = [1, 2, 3, 4];
const E14_ROOT: u64 = 93_000_000;
const E16_ROOT: u64 = 95_000_000;

#[derive(Clone, Debug, PartialEq, Eq)]
struct LaneObservation {
    lane: &'static str,
    context: u16,
    context_present: bool,
    contact_live: bool,
    route_eligible_at_perturbation: bool,
    drive_emitted: bool,
    route_participates: bool,
    pre_contact_pairs: usize,
    post_contact_pairs: usize,
    trace_proposed_pairs: usize,
    pre_eligible_pairs: usize,
    eligible_pairs_before_babbler: usize,
    source_fires: usize,
    context_trace_fires: usize,
    babbler_fires: usize,
    motor_fires: usize,
    babbler_fire_tick: Option<i64>,
    babbler_fire_phase: Option<i32>,
    stem_participation_delta: u64,
    outgoing_participation_delta: u64,
    drive_deliveries_to_contacts: usize,
    drive_deliveries_to_motor: usize,
    action: Option<u8>,
    outward_crossings: usize,
    physical_work: u64,
    physical_tick: i64,
    naturally_quiescent: bool,
    trace: Vec<PhysicalTransition>,
}

fn first_frame() -> Vec<u8> {
    let mut contexts = BTreeSet::new();
    for nonce in 0_u16..u16::MAX {
        let mut candidate = vec![4_u8; ARC3_FRAME_PIXELS];
        candidate[0] = (nonce & 0x0f) as u8;
        candidate[1] = (nonce >> 4 & 0x0f) as u8;
        candidate[2] = (nonce >> 8 & 0x0f) as u8;
        candidate[3] = (nonce >> 12 & 0x0f) as u8;
        let context = spatial_context(&candidate).expect("valid E14/E16 frame");
        if contexts.insert(context) {
            return candidate;
        }
    }
    panic!("one E14/E16 frame must exist")
}

fn pair_contacts(diagnostic: &Arc3ContextDiagnostic, eligible: bool) -> BTreeSet<CellId> {
    let mut stems = BTreeSet::new();
    let mut outgoings = BTreeSet::new();
    for link in &diagnostic.links {
        let Some(contact) = link.contact else {
            continue;
        };
        let acceptable = !eligible || (link.coupling > 0 && link.resistance > 0);
        if !acceptable {
            continue;
        }
        match link.role {
            "stem" => {
                stems.insert(contact);
            }
            "outgoing" => {
                outgoings.insert(contact);
            }
            _ => {}
        }
    }
    stems.intersection(&outgoings).copied().collect()
}

fn participation_by_arrow(diagnostic: &Arc3ContextDiagnostic) -> BTreeMap<ArrowId, u64> {
    diagnostic
        .links
        .iter()
        .map(|link| (link.arrow, link.participation))
        .collect()
}

fn participation_delta(
    role: &str,
    before: &BTreeMap<ArrowId, u64>,
    after: &Arc3ContextDiagnostic,
) -> u64 {
    after
        .links
        .iter()
        .filter(|link| link.role == role)
        .map(|link| {
            link.participation
                .saturating_sub(before.get(&link.arrow).copied().unwrap_or(0))
        })
        .sum()
}

fn fire_count(trace: &[PhysicalTransition], cell: CellId) -> usize {
    trace
        .iter()
        .filter(|transition| {
            matches!(transition.event, PhysicalEvent::Fire { cell: fired } if fired == cell)
        })
        .count()
}

fn first_fire(trace: &[PhysicalTransition], cell: CellId) -> Option<(usize, &PhysicalTransition)> {
    trace.iter().enumerate().find(|(_, transition)| {
        matches!(transition.event, PhysicalEvent::Fire { cell: fired } if fired == cell)
    })
}

fn proposed_pair_positions(
    diagnostic: &Arc3ContextDiagnostic,
    trace: &[PhysicalTransition],
) -> BTreeMap<CellId, (Option<usize>, Option<usize>)> {
    let mut positions = BTreeMap::<CellId, (Option<usize>, Option<usize>)>::new();
    for (index, transition) in trace.iter().enumerate() {
        let PhysicalEvent::Proposal { from, to, .. } = transition.event else {
            continue;
        };
        if from == diagnostic.source {
            positions.entry(to).or_default().0 = Some(index);
        }
        if to == diagnostic.target {
            positions.entry(from).or_default().1 = Some(index);
        }
    }
    positions
}

fn links_for_contact(
    diagnostic: &Arc3ContextDiagnostic,
    contact: CellId,
) -> (
    Vec<&Arc3CandidateLinkDiagnostic>,
    Vec<&Arc3CandidateLinkDiagnostic>,
) {
    let stems = diagnostic
        .links
        .iter()
        .filter(|link| link.role == "stem" && link.contact == Some(contact))
        .collect();
    let outgoings = diagnostic
        .links
        .iter()
        .filter(|link| link.role == "outgoing" && link.contact == Some(contact))
        .collect();
    (stems, outgoings)
}

fn eligible_pairs_before_babbler(
    before: &Arc3ContextDiagnostic,
    after: &Arc3ContextDiagnostic,
    trace: &[PhysicalTransition],
) -> usize {
    let Some((babbler_index, _)) = first_fire(trace, after.babbler) else {
        return 0;
    };
    let before_arrows = before
        .links
        .iter()
        .map(|link| link.arrow)
        .collect::<BTreeSet<_>>();
    let proposal_indices = trace
        .iter()
        .enumerate()
        .filter_map(|(index, transition)| match transition.event {
            PhysicalEvent::Proposal { arrow, .. } => Some((arrow, index)),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();

    pair_contacts(after, true)
        .into_iter()
        .filter(|contact| {
            let (stems, outgoings) = links_for_contact(after, *contact);
            let available = |link: &&Arc3CandidateLinkDiagnostic| {
                before_arrows.contains(&link.arrow)
                    || proposal_indices
                        .get(&link.arrow)
                        .is_some_and(|index| *index < babbler_index)
            };
            stems.iter().any(available) && outgoings.iter().any(available)
        })
        .count()
}

fn summarize(
    lane: &'static str,
    expected_context: u16,
    before: Arc3ContextDiagnostic,
    observation: Arc3SensorimotorObservation,
    after: Arc3ContextDiagnostic,
    trace: Vec<PhysicalTransition>,
) -> LaneObservation {
    let before_participation = participation_by_arrow(&before);
    let stem_participation_delta = participation_delta("stem", &before_participation, &after);
    let outgoing_participation_delta =
        participation_delta("outgoing", &before_participation, &after);
    let post_contacts = pair_contacts(&after, false);
    let proposal_positions = proposed_pair_positions(&after, &trace);
    let trace_proposed_pairs = proposal_positions
        .values()
        .filter(|(stem, outgoing)| stem.is_some() && outgoing.is_some())
        .count();
    let eligible_pairs_before_babbler = eligible_pairs_before_babbler(&before, &after, &trace);
    let (babbler_fire_tick, babbler_fire_phase) = first_fire(&trace, after.babbler)
        .map(|(_, transition)| (Some(transition.tick), Some(transition.phase)))
        .unwrap_or((None, None));
    let drive_deliveries_to_contacts = trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::Deliver {
                    mode: TransmissionMode::Drive,
                    target,
                    ..
                } if post_contacts.contains(&target)
            )
        })
        .count();
    let drive_deliveries_to_motor = trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::Deliver {
                    mode: TransmissionMode::Drive,
                    target,
                    ..
                } if target == after.target
            )
        })
        .count();
    let source_fires = fire_count(&trace, after.source);
    let context_trace_fires = fire_count(&trace, after.context_trace);
    let babbler_fires = fire_count(&trace, after.babbler);
    let motor_fires = fire_count(&trace, after.target);
    let pre_contact_pairs = pair_contacts(&before, false).len();
    let post_contact_pairs = post_contacts.len();
    let pre_eligible_pairs = pair_contacts(&before, true).len();

    LaneObservation {
        lane,
        context: observation.context,
        context_present: observation.context == expected_context
            && (source_fires > 0 || context_trace_fires > 0),
        contact_live: pre_contact_pairs > 0 || post_contact_pairs > 0 || trace_proposed_pairs > 0,
        route_eligible_at_perturbation: eligible_pairs_before_babbler > 0,
        drive_emitted: stem_participation_delta > 0
            || outgoing_participation_delta > 0
            || drive_deliveries_to_contacts > 0,
        route_participates: observation.motor_crossing.is_some(),
        pre_contact_pairs,
        post_contact_pairs,
        trace_proposed_pairs,
        pre_eligible_pairs,
        eligible_pairs_before_babbler,
        source_fires,
        context_trace_fires,
        babbler_fires,
        motor_fires,
        babbler_fire_tick,
        babbler_fire_phase,
        stem_participation_delta,
        outgoing_participation_delta,
        drive_deliveries_to_contacts,
        drive_deliveries_to_motor,
        action: observation.action,
        outward_crossings: observation.outward_crossings,
        physical_work: observation.physical_work,
        physical_tick: observation.physical_tick,
        naturally_quiescent: observation.naturally_quiescent,
        trace,
    }
}

fn e14_lane(mechanics: MechanicalConfig) -> LaneObservation {
    let pixels = first_frame();
    let context = spatial_context(&pixels).expect("E14 context");
    let mut organism = Arc3Sensorimotor::new_spatial_with_profile(
        E14_ROOT,
        mechanics,
        Core0Profile::GenericExternal,
    )
    .expect("E14 body");
    let before = organism
        .diagnostic_context(context, 0)
        .expect("E14 pre-admission diagnostic");
    let observation = organism
        .observe(pixels, &ACTIONS, Some(1), false, false, &ACTIONS)
        .expect("unchanged E14 first teaching turn");
    let trace = organism.last_action_physical_trace().to_vec();
    let after = organism
        .diagnostic_context(context, 0)
        .expect("E14 post-admission diagnostic");
    summarize("e14", context, before, observation, after, trace)
}

fn e16_lane(mechanics: MechanicalConfig) -> LaneObservation {
    let pixels = first_frame();
    let context = spatial_context(&pixels).expect("E16 context");
    let mut organism = Arc3Sensorimotor::new_spatial_with_profile(
        E16_ROOT,
        mechanics,
        Core0Profile::GenericExternal,
    )
    .expect("E16 body");
    for _ in 0..2 {
        let prime = organism
            .observe(pixels.clone(), &ACTIONS, None, false, false, &ACTIONS)
            .expect("unchanged E16 unresolved prime");
        assert!(prime.action.is_none(), "E16 prime must remain unresolved");
        organism.clear_episode();
    }
    organism
        .advance_gap(1)
        .expect("unchanged E16 episode recovery");
    let before = organism
        .diagnostic_context(context, 0)
        .expect("E16 pre-admission diagnostic");
    let observation = organism
        .observe_with_transient_history(Arc3TransientHistoryRequest {
            frame: pixels,
            available_actions: &ACTIONS,
            babble_action: 1,
            support_previous: false,
            settle_pressure: false,
            action_map: &ACTIONS,
            early_material_sign: 1,
        })
        .expect("unchanged E16 first participation");
    let trace = organism.last_action_physical_trace().to_vec();
    let after = organism
        .diagnostic_context(context, 0)
        .expect("E16 post-admission diagnostic");
    summarize("e16", context, before, observation, after, trace)
}

fn first_divergence(e14: &LaneObservation, e16: &LaneObservation) -> &'static str {
    for (name, negative, positive) in [
        ("context_present", e14.context_present, e16.context_present),
        ("contact_live", e14.contact_live, e16.contact_live),
        (
            "route_eligible_at_perturbation",
            e14.route_eligible_at_perturbation,
            e16.route_eligible_at_perturbation,
        ),
        ("drive_emitted", e14.drive_emitted, e16.drive_emitted),
        (
            "route_participates",
            e14.route_participates,
            e16.route_participates,
        ),
    ] {
        if negative != positive {
            return name;
        }
    }
    "none"
}

fn option<T: ToString>(value: Option<T>) -> String {
    value.map_or_else(|| "none".to_string(), |value| value.to_string())
}

fn write_row(
    csv: &mut String,
    mechanics: &str,
    observation: &LaneObservation,
    divergence: &str,
    replay_exact: bool,
    mechanics_exact: bool,
) {
    writeln!(
        csv,
        "{mechanics},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        observation.lane,
        observation.context,
        observation.context_present,
        observation.contact_live,
        observation.route_eligible_at_perturbation,
        observation.drive_emitted,
        observation.route_participates,
        observation.pre_contact_pairs,
        observation.post_contact_pairs,
        observation.trace_proposed_pairs,
        observation.pre_eligible_pairs,
        observation.eligible_pairs_before_babbler,
        observation.source_fires,
        observation.context_trace_fires,
        observation.babbler_fires,
        observation.motor_fires,
        option(observation.babbler_fire_tick),
        option(observation.babbler_fire_phase),
        observation.stem_participation_delta,
        observation.outgoing_participation_delta,
        observation.drive_deliveries_to_contacts,
        observation.drive_deliveries_to_motor,
        option(observation.action),
        observation.outward_crossings,
        observation.physical_work,
        observation.physical_tick,
        observation.naturally_quiescent,
        observation.trace.len(),
        divergence,
        replay_exact,
        mechanics_exact,
        observation.context_present && observation.naturally_quiescent,
    )
    .expect("write E23 row");
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.get(1).is_some_and(|value| value == "--check") {
        println!(
            "CORE1_E23_FIRST_PARTICIPATION_DIFFERENTIAL_AUDIT_V1_OBSERVER_OK e14_root={E14_ROOT} e16_root={E16_ROOT} action=1"
        );
        return;
    }

    eprintln!("CORE1_E23_FIRST_PARTICIPATION_DIFFERENTIAL_AUDIT_V1_EVIDENCE_SPENT");
    let destination = arguments.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/core1_e23_first_participation_differential_audit_v1")
    });
    fs::create_dir_all(&destination).expect("create E23 result directory");

    let reference = (
        e14_lane(MechanicalConfig::REFERENCE),
        e16_lane(MechanicalConfig::REFERENCE),
    );
    let replay = (
        e14_lane(MechanicalConfig::REFERENCE),
        e16_lane(MechanicalConfig::REFERENCE),
    );
    let production = (
        e14_lane(MechanicalConfig::PRODUCTION),
        e16_lane(MechanicalConfig::PRODUCTION),
    );
    let replay_exact = reference == replay;
    let mechanics_exact = reference == production;
    let divergence = first_divergence(&reference.0, &reference.1);

    let mut csv = String::from(
        "mechanics,lane,context,context_present,contact_live,route_eligible_at_perturbation,drive_emitted,route_participates,pre_contact_pairs,post_contact_pairs,trace_proposed_pairs,pre_eligible_pairs,eligible_pairs_before_babbler,source_fires,context_trace_fires,babbler_fires,motor_fires,babbler_fire_tick,babbler_fire_phase,stem_participation_delta,outgoing_participation_delta,drive_deliveries_to_contacts,drive_deliveries_to_motor,action,outward_crossings,physical_work,physical_tick,quiescent,trace_events,first_divergence,replay_exact,mechanics_exact,valid\n",
    );
    for (mechanics, lanes) in [
        ("reference", &reference),
        ("replay", &replay),
        ("production", &production),
    ] {
        write_row(
            &mut csv,
            mechanics,
            &lanes.0,
            divergence,
            replay_exact,
            mechanics_exact,
        );
        write_row(
            &mut csv,
            mechanics,
            &lanes.1,
            divergence,
            replay_exact,
            mechanics_exact,
        );
    }
    fs::write(destination.join("matrix.csv"), csv).expect("write E23 matrix");

    let valid = reference.0.context_present
        && reference.1.context_present
        && !reference.0.route_participates
        && reference.1.route_participates
        && reference.0.naturally_quiescent
        && reference.1.naturally_quiescent
        && replay_exact
        && mechanics_exact
        && divergence != "none";
    let report = format!(
        "# CORE1 E23 first-participation differential audit\n\n- first divergence: `{divergence}`\n- E14 stages: `{}|{}|{}|{}|{}`\n- E16 stages: `{}|{}|{}|{}|{}`\n- E14 pre/post contact pairs: `{}/{}`\n- E16 pre/post contact pairs: `{}/{}`\n- E14 action: `{}`\n- E16 action: `{}`\n- exact replay: `{replay_exact}`\n- Reference/Production exact: `{mechanics_exact}`\n- valid localization: `{valid}`\n",
        reference.0.context_present,
        reference.0.contact_live,
        reference.0.route_eligible_at_perturbation,
        reference.0.drive_emitted,
        reference.0.route_participates,
        reference.1.context_present,
        reference.1.contact_live,
        reference.1.route_eligible_at_perturbation,
        reference.1.drive_emitted,
        reference.1.route_participates,
        reference.0.pre_contact_pairs,
        reference.0.post_contact_pairs,
        reference.1.pre_contact_pairs,
        reference.1.post_contact_pairs,
        option(reference.0.action),
        option(reference.1.action),
    );
    fs::write(destination.join("report.md"), report).expect("write E23 report");

    println!(
        "CORE1_E23_FIRST_PARTICIPATION_DIFFERENTIAL_AUDIT_V1_COMPLETE first_divergence={divergence} e14={} e16={} replay={} mechanics={} valid={}",
        option(reference.0.action),
        option(reference.1.action),
        replay_exact,
        mechanics_exact,
        valid,
    );
}
