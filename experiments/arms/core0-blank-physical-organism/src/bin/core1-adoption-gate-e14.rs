#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Instant;

use academy_arc3::{
    spatial_context, Arc3Sensorimotor, Arc3SensorimotorObservation, ARC3_FRAME_PIXELS,
};
use truelearner_core::{Core0Profile, MechanicalConfig};

const ACTIONS: [u8; 4] = [1, 4, 2, 3];
const ACTION_MAP: [u8; 4] = [1, 2, 3, 4];
const ROOT: u64 = 93_000_000;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    turns: Vec<Arc3SensorimotorObservation>,
    autonomous_actions: Vec<Option<u8>>,
    returns: Vec<usize>,
    behavioral_frontier_closed: bool,
    legacy_contract_pass: bool,
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

fn organism() -> Arc3Sensorimotor {
    Arc3Sensorimotor::new_spatial_with_profile(
        ROOT,
        MechanicalConfig::REFERENCE,
        Core0Profile::GenericExternal,
    )
    .expect("full CORE1-B Academy body")
}

fn execute(mut organism: Arc3Sensorimotor) -> Observation {
    let frames = frames();
    let mut turns = Vec::with_capacity(5);
    let mut returns = Vec::with_capacity(5);
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
        returns.push(organism.return_path_count());
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
    returns.push(organism.return_path_count());

    let actions = turns
        .iter()
        .take(4)
        .map(|turn| turn.action)
        .collect::<Vec<_>>();
    let updates = turns
        .iter()
        .map(|turn| turn.plasticity_updates)
        .collect::<Vec<_>>();
    let autonomous_actions = frames
        .iter()
        .take(4)
        .map(|pixels| {
            let mut probe = organism.clone();
            probe.clear_episode();
            probe.advance_gap(1).expect("autonomous revisit gap");
            probe
                .observe(pixels.clone(), &ACTION_MAP, None, false, false, &ACTION_MAP)
                .expect("autonomous revisit")
                .action
        })
        .collect::<Vec<_>>();
    let naturally_quiescent = turns.iter().all(|turn| turn.naturally_quiescent);
    let behavioral_frontier_closed = actions == [Some(1), Some(4), Some(2), Some(3)]
        && updates == [0, 2, 2, 2, 2]
        && autonomous_actions == [Some(1), Some(4), Some(2), Some(3)]
        && naturally_quiescent;
    let legacy_contract_pass = actions == [Some(1), Some(4), Some(2), Some(3)]
        && updates == [0, 1, 1, 1, 1]
        && turns.iter().all(|turn| turn.naturally_quiescent);
    Observation {
        turns,
        autonomous_actions,
        returns,
        behavioral_frontier_closed,
        legacy_contract_pass,
    }
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
            "CORE1_ADOPTION_GATE_E14_V1_OBSERVER_OK root={ROOT} contexts={}",
            list(contexts)
        );
        return;
    }

    eprintln!("CORE1_ADOPTION_GATE_E14_V1_EVIDENCE_SPENT");
    let started = Instant::now();
    let destination = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_adoption_gate_e14_v1"));
    fs::create_dir_all(&destination).expect("create CORE1 adoption result directory");

    let base = organism();
    let reference_organism = base.clone();
    let replay_organism = base.clone();
    let mut production_organism = base;
    production_organism.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
    let (reference, replay, production) = thread::scope(|scope| {
        let reference = scope.spawn(|| execute(reference_organism));
        let replay = scope.spawn(|| execute(replay_organism));
        let production = scope.spawn(|| execute(production_organism));
        (
            reference.join().expect("Reference E14 execution"),
            replay.join().expect("Reference replay E14 execution"),
            production.join().expect("Production E14 execution"),
        )
    });
    let replay_exact = reference == replay;
    let mechanics_exact = reference == production;

    let mut csv = String::from(
        "mechanics,behavioral_frontier_closed,legacy_contract_pass,actions,autonomous_actions,updates,modulatory_deliveries,returns,physical_work,ticks,quiescent,body_fingerprints\n",
    );
    for (name, observation) in [
        ("reference", &reference),
        ("replay", &replay),
        ("production", &production),
    ] {
        writeln!(
            csv,
            "{name},{},{},{},{},{},{},{},{},{},{},{}",
            observation.behavioral_frontier_closed,
            observation.legacy_contract_pass,
            actions(observation),
            list(observation.autonomous_actions.iter().map(|action| {
                action.map_or_else(|| "none".to_string(), |action| action.to_string())
            })),
            list(observation.turns.iter().map(|turn| turn.plasticity_updates)),
            list(
                observation
                    .turns
                    .iter()
                    .map(|turn| turn.modulatory_deliveries)
            ),
            list(&observation.returns),
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
        .expect("write CORE1 adoption row");
    }
    fs::write(destination.join("matrix.csv"), csv).expect("write CORE1 adoption matrix");

    let updates = list(reference.turns.iter().map(|turn| turn.plasticity_updates));
    let elapsed_ms = started.elapsed().as_millis();
    let report = format!(
        "# CORE1 adoption gate on unchanged E14 ARC A2\n\n- behavioral frontier closed: `{}`\n- legacy one-update contract pass: `{}`\n- actions: `{}`\n- autonomous revisit: `{}`\n- updates: `{updates}`\n- exact replay: `{replay_exact}`\n- Reference/Production exact: `{mechanics_exact}`\n- natural quiescence: `{}`\n- elapsed milliseconds: `{elapsed_ms}`\n",
        reference.behavioral_frontier_closed,
        reference.legacy_contract_pass,
        actions(&reference),
        list(reference.autonomous_actions.iter().map(|action| action
            .map_or_else(|| "none".to_string(), |action| action.to_string()))),
        reference
            .turns
            .iter()
            .all(|turn| turn.naturally_quiescent),
    );
    fs::write(destination.join("report.md"), report).expect("write CORE1 adoption report");

    println!(
        "CORE1_ADOPTION_GATE_E14_V1_COMPLETE behavioral_frontier_closed={} legacy_contract_pass={} replay_exact={} mechanics_exact={} elapsed_ms={elapsed_ms}",
        reference.behavioral_frontier_closed,
        reference.legacy_contract_pass,
        replay_exact,
        mechanics_exact
    );
}
