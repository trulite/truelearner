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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    turns: Vec<Arc3SensorimotorObservation>,
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
        let context = spatial_context(&candidate).expect("valid frozen ARC A2 frame");
        if contexts.insert(context) {
            frames.push(candidate);
            if frames.len() == 5 {
                break;
            }
        }
    }
    assert_eq!(frames.len(), 5, "five frozen ARC A2 frames");
    frames
}

fn execute(root: u64, mechanics: MechanicalConfig) -> Observation {
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, Core0Profile::GenericExternal)
            .expect("CORE1-B Academy body");
    let frames = frames();
    let mut turns = Vec::with_capacity(5);
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
                .expect("frozen ARC A2 teaching observation"),
        );
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
            .expect("frozen ARC A2 closing observation"),
    );
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
        && turns.iter().all(|turn| turn.naturally_quiescent);
    Observation { turns, pass }
}

fn list<T: ToString>(values: impl IntoIterator<Item = T>) -> String {
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn actions(observation: &Observation) -> String {
    list(observation.turns.iter().map(|turn| {
        turn.action
            .map_or_else(|| "none".to_string(), |action| action.to_string())
    }))
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.get(1).is_some_and(|value| value == "--check") {
        let contexts = frames()
            .iter()
            .map(|frame| spatial_context(frame).expect("valid frame"))
            .collect::<Vec<_>>();
        println!(
            "CORE1_E14_ARC_A2_V1_OBSERVER_OK contexts={}",
            list(contexts)
        );
        return;
    }

    eprintln!("CORE1_E14_ARC_A2_V1_EVIDENCE_SPENT");
    let destination = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_e14_arc_a2_v1"));
    fs::create_dir_all(&destination).expect("create E14 result directory");

    let root = 93_000_000;
    let reference = execute(root, MechanicalConfig::REFERENCE);
    let replay = execute(root, MechanicalConfig::REFERENCE);
    let production = execute(root, MechanicalConfig::PRODUCTION);
    let replay_exact = reference == replay;
    let mechanics_exact = reference == production;

    let mut csv = String::from(
        "mechanics,pass,actions,updates,modulatory_deliveries,physical_work,ticks,quiescent,body_fingerprints\n",
    );
    for (name, observation) in [
        ("reference", &reference),
        ("replay", &replay),
        ("production", &production),
    ] {
        writeln!(
            csv,
            "{name},{},{},{},{},{},{},{},{}",
            observation.pass,
            actions(observation),
            list(observation.turns.iter().map(|turn| turn.plasticity_updates)),
            list(
                observation
                    .turns
                    .iter()
                    .map(|turn| turn.modulatory_deliveries)
            ),
            list(observation.turns.iter().map(|turn| turn.physical_work)),
            list(observation.turns.iter().map(|turn| turn.physical_tick)),
            observation
                .turns
                .iter()
                .all(|turn| turn.naturally_quiescent),
            list(
                observation
                    .turns
                    .iter()
                    .map(|turn| turn.body_fingerprint.clone())
            ),
        )
        .expect("write E14 row");
    }
    fs::write(destination.join("matrix.csv"), csv).expect("write E14 matrix");

    let updates = list(reference.turns.iter().map(|turn| turn.plasticity_updates));
    let report = format!(
        "# CORE1-B E14 frozen ARC A2 result\n\n- pass: `{}`\n- actions: `{}`\n- updates: `{updates}`\n- exact replay: `{replay_exact}`\n- Reference/Production exact: `{mechanics_exact}`\n- natural quiescence: `{}`\n",
        reference.pass,
        actions(&reference),
        reference
            .turns
            .iter()
            .all(|turn| turn.naturally_quiescent),
    );
    fs::write(destination.join("report.md"), report).expect("write E14 report");

    println!(
        "CORE1_E14_ARC_A2_V1_COMPLETE pass={} replay_exact={} mechanics_exact={}",
        reference.pass, replay_exact, mechanics_exact
    );
}
