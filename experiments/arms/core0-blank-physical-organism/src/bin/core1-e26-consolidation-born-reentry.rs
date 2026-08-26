#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use academy_arc3::{
    spatial_context, Arc3Sensorimotor, Arc3SensorimotorObservation, ARC3_FRAME_PIXELS,
};
use truelearner_core::{Core0Profile, MechanicalConfig};

const ACTIONS: [u8; 4] = [1, 4, 2, 3];
const ACTION_MAP: [u8; 4] = [1, 2, 3, 4];
const ROOT_BASE: u64 = 93_000_000;
const SEEDS: [u64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

#[derive(Clone, Debug, PartialEq, Eq)]
struct CreationGate {
    turns: [Arc3SensorimotorObservation; 2],
    reentries: [usize; 2],
    returns: [usize; 2],
    pending: [usize; 2],
    pass: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FullObservation {
    turns: Vec<Arc3SensorimotorObservation>,
    reentries: Vec<usize>,
    returns: Vec<usize>,
    pending: Vec<usize>,
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

fn organism(root: u64, mechanics: MechanicalConfig) -> Arc3Sensorimotor {
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, Core0Profile::GenericExternal)
            .expect("CORE1-B E26 body");
    organism.enable_atomic_route_closure();
    organism.enable_local_signed_gating();
    organism.enable_motor_integration_window();
    organism.enable_consolidation_reentry();
    organism
}

fn creation_gate(root: u64, mechanics: MechanicalConfig) -> CreationGate {
    let frames = frames();
    let mut organism = organism(root, mechanics);
    let first = organism
        .observe(
            frames[0].clone(),
            &ACTION_MAP,
            Some(1),
            false,
            false,
            &ACTION_MAP,
        )
        .expect("E26 first teaching observation");
    let first_reentries = organism.consolidation_reentry_count();
    let first_returns = organism.temporary_credit_return_count();
    let first_pending = organism.used_pending_count();
    let second = organism
        .observe(
            frames[1].clone(),
            &ACTION_MAP,
            Some(4),
            true,
            false,
            &ACTION_MAP,
        )
        .expect("E26 second teaching observation");
    let second_reentries = organism.consolidation_reentry_count();
    let second_returns = organism.temporary_credit_return_count();
    let second_pending = organism.used_pending_count();
    let pass = first.action == Some(1)
        && first.modulatory_deliveries == 0
        && first.plasticity_updates == 0
        && first_reentries == 0
        && first_returns == 1
        && first_pending == 0
        && second.action == Some(4)
        && second.modulatory_deliveries > 0
        && second.plasticity_updates > 0
        && second_reentries == 1
        && second_returns == 1
        && second_pending == 0
        && first.naturally_quiescent
        && second.naturally_quiescent;
    CreationGate {
        turns: [first, second],
        reentries: [first_reentries, second_reentries],
        returns: [first_returns, second_returns],
        pending: [first_pending, second_pending],
        pass,
    }
}

fn recover(mut organism: Arc3Sensorimotor) -> Arc3Sensorimotor {
    organism.clear_episode();
    organism.advance_gap(1).expect("ordinary E26 recovery");
    organism
}

fn full(root: u64, mechanics: MechanicalConfig) -> FullObservation {
    let frames = frames();
    let mut organism = organism(root, mechanics);
    let mut turns = Vec::with_capacity(5);
    let mut reentries = Vec::with_capacity(5);
    let mut returns = Vec::with_capacity(5);
    let mut pending = Vec::with_capacity(5);
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
                .expect("E26 frozen teaching observation"),
        );
        reentries.push(organism.consolidation_reentry_count());
        returns.push(organism.temporary_credit_return_count());
        pending.push(organism.used_pending_count());
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
            .expect("E26 frozen closing observation"),
    );
    reentries.push(organism.consolidation_reentry_count());
    returns.push(organism.temporary_credit_return_count());
    pending.push(organism.used_pending_count());

    let probes = frames
        .iter()
        .take(4)
        .map(|frame| {
            recover(organism.clone())
                .observe(frame.clone(), &ACTION_MAP, None, false, false, &ACTION_MAP)
                .expect("E26 autonomous probe")
                .action
        })
        .collect::<Vec<_>>();
    let teaching_actions = turns
        .iter()
        .take(4)
        .map(|turn| turn.action)
        .collect::<Vec<_>>();
    let pass = teaching_actions == [Some(1), Some(4), Some(2), Some(3)]
        && turns
            .iter()
            .skip(1)
            .all(|turn| turn.modulatory_deliveries > 0 && turn.plasticity_updates > 0)
        && reentries == [0, 1, 2, 3, 4]
        && returns == [1, 1, 1, 1, 0]
        && pending.iter().all(|count| *count == 0)
        && turns.iter().all(|turn| turn.naturally_quiescent)
        && probes == [Some(1), Some(4), Some(2), Some(3)];
    FullObservation {
        turns,
        reentries,
        returns,
        pending,
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
    if arguments
        .get(1)
        .is_some_and(|argument| argument == "--check")
    {
        println!(
            "CORE1_E26_CONSOLIDATION_BORN_REENTRY_V1_OBSERVER_OK roots={}",
            list(SEEDS.map(|seed| ROOT_BASE + seed))
        );
        return;
    }

    eprintln!("CORE1_E26_CONSOLIDATION_BORN_REENTRY_V1_EVIDENCE_SPENT");
    let destination = arguments.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/core1_e26_consolidation_born_reentry_v1")
    });
    fs::create_dir_all(&destination).expect("create E26 result directory");

    let mut gate_csv = String::from(
        "root,mechanics,actions,modulatory,updates,reentries,returns,used_pending,quiescent,pass,replay_exact,mechanics_exact\n",
    );
    let mut gate_passes = 0;
    let mut gate_exact = true;
    for seed in SEEDS {
        let root = ROOT_BASE + seed;
        let reference = creation_gate(root, MechanicalConfig::REFERENCE);
        let replay = creation_gate(root, MechanicalConfig::REFERENCE);
        let production = creation_gate(root, MechanicalConfig::PRODUCTION);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        let exact = replay_exact && mechanics_exact;
        gate_passes += usize::from(reference.pass && exact);
        gate_exact &= exact;
        for (mechanics, gate) in [
            ("reference", &reference),
            ("replay", &replay),
            ("production", &production),
        ] {
            writeln!(
                gate_csv,
                "{root},{mechanics},{},{},{},{},{},{},{},{},{},{}",
                options(gate.turns.iter().map(|turn| turn.action)),
                list(gate.turns.iter().map(|turn| turn.modulatory_deliveries)),
                list(gate.turns.iter().map(|turn| turn.plasticity_updates)),
                list(gate.reentries),
                list(gate.returns),
                list(gate.pending),
                gate.turns.iter().all(|turn| turn.naturally_quiescent),
                gate.pass,
                replay_exact,
                mechanics_exact,
            )
            .expect("write E26 creation gate");
        }
    }
    fs::write(destination.join("creation_gate.csv"), gate_csv).expect("write E26 creation gate");

    if gate_passes != SEEDS.len() || !gate_exact {
        let report = format!(
            "# CORE1 E26 consolidation-born re-entry result\n\n- status: `FAILED_AT_CREATION_GATE`\n- creation gate: `{gate_passes}/8`\n- exact replay/mechanics: `{gate_exact}`\n- full regimen run: `false`\n"
        );
        fs::write(destination.join("report.md"), report).expect("write E26 stopped report");
        println!(
            "CORE1_E26_CONSOLIDATION_BORN_REENTRY_V1_STOPPED gate={gate_passes}/8 exact={gate_exact}"
        );
        return;
    }

    let mut full_csv = String::from(
        "root,actions,modulatory,updates,reentries,returns,used_pending,probes,quiescent,pass,replay_exact,mechanics_exact\n",
    );
    let mut full_passes = 0;
    let mut full_exact = true;
    for seed in SEEDS {
        let root = ROOT_BASE + seed;
        let reference = full(root, MechanicalConfig::REFERENCE);
        let replay = full(root, MechanicalConfig::REFERENCE);
        let production = full(root, MechanicalConfig::PRODUCTION);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        let exact = replay_exact && mechanics_exact;
        full_passes += usize::from(reference.pass && exact);
        full_exact &= exact;
        writeln!(
            full_csv,
            "{root},{},{},{},{},{},{},{},{},{},{},{}",
            options(reference.turns.iter().take(4).map(|turn| turn.action)),
            list(
                reference
                    .turns
                    .iter()
                    .map(|turn| turn.modulatory_deliveries)
            ),
            list(reference.turns.iter().map(|turn| turn.plasticity_updates)),
            list(&reference.reentries),
            list(&reference.returns),
            list(&reference.pending),
            options(reference.probes.iter().copied()),
            reference.turns.iter().all(|turn| turn.naturally_quiescent),
            reference.pass,
            replay_exact,
            mechanics_exact,
        )
        .expect("write E26 full matrix");
    }
    fs::write(destination.join("full.csv"), full_csv).expect("write E26 full matrix");

    let pass = full_passes == SEEDS.len() && full_exact;
    let status = if pass {
        "PASSED_8_OF_8"
    } else {
        "FALSIFIED_AFTER_CREATION"
    };
    let report = format!(
        "# CORE1 E26 consolidation-born re-entry result\n\n- status: `{status}`\n- frozen E25 autonomous baseline: `0/8`\n- creation gate: `{gate_passes}/8`\n- full autonomous solve: `{full_passes}/8`\n- exact replay/mechanics: `{}`\n",
        gate_exact && full_exact,
    );
    fs::write(destination.join("report.md"), report).expect("write E26 report");
    println!(
        "CORE1_E26_CONSOLIDATION_BORN_REENTRY_V1_COMPLETE status={status} creation={gate_passes}/8 autonomous={full_passes}/8 exact={}",
        gate_exact && full_exact,
    );
}
