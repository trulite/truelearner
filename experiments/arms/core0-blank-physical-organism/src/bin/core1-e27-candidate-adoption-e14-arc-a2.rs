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
    reentries: Vec<usize>,
    executables: Vec<usize>,
    returns: Vec<usize>,
    pending: Vec<usize>,
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

fn candidate() -> Arc3Sensorimotor {
    let mut organism = Arc3Sensorimotor::new_spatial_with_profile(
        ROOT,
        MechanicalConfig::REFERENCE,
        Core0Profile::GenericExternal,
    )
    .expect("full CORE1-B Academy body");
    organism.enable_atomic_route_closure();
    organism.enable_local_signed_gating();
    organism.enable_motor_integration_window();
    organism.enable_consolidation_reentry();
    organism.enable_consolidation_executability();
    organism
}

fn execute(mut organism: Arc3Sensorimotor) -> Observation {
    let frames = frames();
    let mut turns = Vec::with_capacity(5);
    let mut reentries = Vec::with_capacity(5);
    let mut executables = Vec::with_capacity(5);
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
                .expect("frozen ARC A2 teaching observation"),
        );
        reentries.push(organism.consolidation_reentry_count());
        executables.push(organism.consolidation_executable_count());
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
            .expect("frozen ARC A2 closing observation"),
    );
    reentries.push(organism.consolidation_reentry_count());
    executables.push(organism.consolidation_executable_count());
    returns.push(organism.temporary_credit_return_count());
    pending.push(organism.used_pending_count());

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
    Observation {
        turns,
        reentries,
        executables,
        returns,
        pending,
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
            "CORE1_E27_ADOPTION_E14_ARC_A2_V1_OBSERVER_OK root={ROOT} contexts={}",
            list(contexts)
        );
        return;
    }

    eprintln!("CORE1_E27_ADOPTION_E14_ARC_A2_V1_EVIDENCE_SPENT");
    let started = Instant::now();
    let destination = arguments.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/core1_e27_candidate_adoption_e14_arc_a2_v1")
    });
    fs::create_dir_all(&destination).expect("create E27 adoption result directory");

    let base = candidate();
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
        "mechanics,pass,actions,updates,modulatory_deliveries,reentries,executables,returns,used_pending,physical_work,ticks,quiescent,body_fingerprints\n",
    );
    for (name, observation) in [
        ("reference", &reference),
        ("replay", &replay),
        ("production", &production),
    ] {
        writeln!(
            csv,
            "{name},{},{},{},{},{},{},{},{},{},{},{},{}",
            observation.pass,
            actions(observation),
            list(observation.turns.iter().map(|turn| turn.plasticity_updates)),
            list(
                observation
                    .turns
                    .iter()
                    .map(|turn| turn.modulatory_deliveries)
            ),
            list(&observation.reentries),
            list(&observation.executables),
            list(&observation.returns),
            list(&observation.pending),
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
        .expect("write E27 adoption row");
    }
    fs::write(destination.join("matrix.csv"), csv).expect("write E27 adoption matrix");

    let updates = list(reference.turns.iter().map(|turn| turn.plasticity_updates));
    let elapsed_ms = started.elapsed().as_millis();
    let report = format!(
        "# CORE1 E27 candidate adoption on unchanged E14 ARC A2\n\n- unchanged E14 pass: `{}`\n- actions: `{}`\n- updates: `{updates}`\n- exact replay: `{replay_exact}`\n- Reference/Production exact: `{mechanics_exact}`\n- natural quiescence: `{}`\n- elapsed milliseconds: `{elapsed_ms}`\n",
        reference.pass,
        actions(&reference),
        reference
            .turns
            .iter()
            .all(|turn| turn.naturally_quiescent),
    );
    fs::write(destination.join("report.md"), report).expect("write E27 adoption report");

    println!(
        "CORE1_E27_ADOPTION_E14_ARC_A2_V1_COMPLETE pass={} replay_exact={} mechanics_exact={} elapsed_ms={elapsed_ms}",
        reference.pass, replay_exact, mechanics_exact
    );
}
