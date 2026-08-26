#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use academy_arc3::{
    spatial_context, Arc3Sensorimotor, Arc3SensorimotorObservation, ARC3_FRAME_PIXELS,
};
use truelearner_core::{Core0Profile, MechanicalConfig, PhysicalEvent};

const ACTIONS: [u8; 4] = [1, 4, 2, 3];
const ACTION_MAP: [u8; 4] = [1, 2, 3, 4];
const CONTEXTS: usize = 5;
const ROOT_BASE: u64 = 95_000_000;
const SEEDS: [u64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Baseline,
    Executable,
}

impl Arm {
    const fn label(self) -> &'static str {
        match self {
            Self::Baseline => "B",
            Self::Executable => "X",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    turns: Vec<Arc3SensorimotorObservation>,
    reentries: Vec<usize>,
    executables: Vec<usize>,
    returns: Vec<usize>,
    pending: Vec<usize>,
    probes: Vec<Option<u8>>,
    traversals: Vec<usize>,
    probe_quiescence: Vec<bool>,
    pass: bool,
}

fn frames() -> Vec<Vec<u8>> {
    let mut by_context = BTreeMap::new();
    for nonce in 0_u32..=u32::MAX {
        let mut candidate = vec![4_u8; ARC3_FRAME_PIXELS];
        candidate[0] = (nonce & 0x0f) as u8;
        candidate[1] = (nonce >> 4 & 0x0f) as u8;
        candidate[2] = (nonce >> 8 & 0x0f) as u8;
        candidate[3] = (nonce >> 12 & 0x0f) as u8;
        candidate[4] = (nonce >> 16 & 0x0f) as u8;
        candidate[5] = (nonce >> 20 & 0x0f) as u8;
        candidate[6] = (nonce >> 24 & 0x0f) as u8;
        candidate[7] = (nonce >> 28 & 0x0f) as u8;
        let context = spatial_context(&candidate).expect("valid compact E27 frame");
        if usize::from(context) < CONTEXTS {
            by_context.entry(context).or_insert(candidate);
            if by_context.len() == CONTEXTS {
                break;
            }
        }
    }
    assert_eq!(by_context.len(), CONTEXTS, "five compact E27 contexts");
    by_context.into_values().collect()
}

fn organism(root: u64, mechanics: MechanicalConfig, arm: Arm) -> Arc3Sensorimotor {
    let mut organism = Arc3Sensorimotor::new_spatial_fixture_with_profile(
        root,
        mechanics,
        CONTEXTS,
        Core0Profile::GenericExternal,
    )
    .expect("compact CORE1-B E27 body");
    organism.enable_atomic_route_closure();
    organism.enable_local_signed_gating();
    organism.enable_motor_integration_window();
    organism.enable_consolidation_reentry();
    if arm == Arm::Executable {
        organism.enable_consolidation_executability();
    }
    organism
}

fn recover(mut organism: Arc3Sensorimotor) -> Arc3Sensorimotor {
    organism.clear_episode();
    organism
        .advance_gap(1)
        .expect("ordinary compact E27 recovery");
    organism
}

fn executable_traversals(organism: &Arc3Sensorimotor) -> usize {
    organism
        .last_action_physical_trace()
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::ConsolidatedExecution { .. }
            )
        })
        .count()
}

fn run(root: u64, mechanics: MechanicalConfig, arm: Arm) -> Observation {
    let frames = frames();
    let mut organism = organism(root, mechanics, arm);
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
                .expect("compact E27 teaching observation"),
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
            .expect("compact E27 closing observation"),
    );
    reentries.push(organism.consolidation_reentry_count());
    executables.push(organism.consolidation_executable_count());
    returns.push(organism.temporary_credit_return_count());
    pending.push(organism.used_pending_count());

    let mut probes = Vec::with_capacity(4);
    let mut traversals = Vec::with_capacity(4);
    let mut probe_quiescence = Vec::with_capacity(4);
    for frame in frames.iter().take(4) {
        let mut probe_organism = recover(organism.clone());
        let probe = probe_organism
            .observe(frame.clone(), &ACTION_MAP, None, false, false, &ACTION_MAP)
            .expect("compact E27 autonomous probe");
        probes.push(probe.action);
        traversals.push(executable_traversals(&probe_organism));
        probe_quiescence.push(probe.naturally_quiescent);
    }

    let common =
        turns
            .iter()
            .take(4)
            .map(|turn| turn.action)
            .eq([Some(1), Some(4), Some(2), Some(3)])
            && turns
                .iter()
                .skip(1)
                .all(|turn| turn.modulatory_deliveries > 0 && turn.plasticity_updates > 0)
            && reentries == [0, 1, 2, 3, 4]
            && returns == [1, 1, 1, 1, 0]
            && pending.iter().all(|count| *count == 0)
            && turns.iter().all(|turn| turn.naturally_quiescent)
            && probe_quiescence.iter().all(|value| *value);
    let arm_pass = match arm {
        Arm::Baseline => {
            executables.iter().all(|count| *count == 0)
                && probes.iter().all(Option::is_none)
                && traversals.iter().all(|count| *count == 0)
        }
        Arm::Executable => {
            executables == [0, 2, 4, 6, 8]
                && probes == [Some(1), Some(4), Some(2), Some(3)]
                && traversals.iter().all(|count| *count >= 2)
        }
    };
    Observation {
        turns,
        reentries,
        executables,
        returns,
        pending,
        probes,
        traversals,
        probe_quiescence,
        pass: common && arm_pass,
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

fn append_row(
    csv: &mut String,
    root: u64,
    mechanics: &str,
    arm: Arm,
    observation: &Observation,
    replay_exact: bool,
    mechanics_exact: bool,
) {
    writeln!(
        csv,
        "{root},{},{mechanics},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        arm.label(),
        options(observation.turns.iter().take(4).map(|turn| turn.action)),
        list(
            observation
                .turns
                .iter()
                .map(|turn| turn.modulatory_deliveries)
        ),
        list(observation.turns.iter().map(|turn| turn.plasticity_updates)),
        list(&observation.reentries),
        list(&observation.executables),
        list(&observation.returns),
        list(&observation.pending),
        options(observation.probes.iter().copied()),
        list(&observation.traversals),
        observation
            .turns
            .iter()
            .all(|turn| turn.naturally_quiescent)
            && observation.probe_quiescence.iter().all(|value| *value),
        observation.pass,
        replay_exact,
        mechanics_exact,
    )
    .expect("write compact E27 matrix");
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments
        .get(1)
        .is_some_and(|argument| argument == "--smoke")
    {
        let started = Instant::now();
        let baseline = run(94_999_999, MechanicalConfig::REFERENCE, Arm::Baseline);
        let candidate = run(94_999_999, MechanicalConfig::REFERENCE, Arm::Executable);
        assert!(baseline.pass, "compact E26 smoke baseline must reproduce");
        assert!(candidate.pass, "compact E27 smoke candidate must solve");
        println!(
            "CORE1_E27_CONSOLIDATION_BORN_EXECUTABILITY_V2_SMOKE_OK elapsed_ms={}",
            started.elapsed().as_millis()
        );
        return;
    }
    if arguments
        .get(1)
        .is_some_and(|argument| argument == "--check")
    {
        println!(
            "CORE1_E27_CONSOLIDATION_BORN_EXECUTABILITY_V2_OBSERVER_OK roots={} contexts={CONTEXTS}",
            list(SEEDS.map(|seed| ROOT_BASE + seed))
        );
        return;
    }

    eprintln!("CORE1_E27_CONSOLIDATION_BORN_EXECUTABILITY_V2_EVIDENCE_SPENT");
    let started = Instant::now();
    let destination = arguments.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/core1_e27_consolidation_born_executability_v2")
    });
    fs::create_dir_all(&destination).expect("create compact E27 result directory");

    let mut csv = String::from(
        "root,arm,mechanics,actions,modulatory,updates,reentries,executables,returns,used_pending,probes,traversals,quiescent,pass,replay_exact,mechanics_exact\n",
    );
    let mut baseline_passes = 0;
    let mut baseline_exact = true;
    for seed in SEEDS {
        let root = ROOT_BASE + seed;
        let reference = run(root, MechanicalConfig::REFERENCE, Arm::Baseline);
        let replay = run(root, MechanicalConfig::REFERENCE, Arm::Baseline);
        let production = run(root, MechanicalConfig::PRODUCTION, Arm::Baseline);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        let exact = replay_exact && mechanics_exact;
        baseline_passes += usize::from(reference.pass && exact);
        baseline_exact &= exact;
        append_row(
            &mut csv,
            root,
            "reference",
            Arm::Baseline,
            &reference,
            replay_exact,
            mechanics_exact,
        );
        append_row(
            &mut csv,
            root,
            "replay",
            Arm::Baseline,
            &replay,
            replay_exact,
            mechanics_exact,
        );
        append_row(
            &mut csv,
            root,
            "production",
            Arm::Baseline,
            &production,
            replay_exact,
            mechanics_exact,
        );
    }
    if baseline_passes != SEEDS.len() || !baseline_exact {
        fs::write(destination.join("matrix.csv"), csv).expect("write invalid v2 matrix");
        let elapsed_ms = started.elapsed().as_millis();
        let report = format!(
            "# CORE1 E27 consolidation-born executability v2 result\n\n- status: `INVALID_BASELINE_MISMATCH`\n- compact E26 baseline: `{baseline_passes}/8`\n- candidate run: `false`\n- exact replay/mechanics: `{baseline_exact}`\n- harness elapsed milliseconds: `{elapsed_ms}`\n"
        );
        fs::write(destination.join("report.md"), report).expect("write invalid v2 report");
        println!(
            "CORE1_E27_CONSOLIDATION_BORN_EXECUTABILITY_V2_STOPPED baseline={baseline_passes}/8 exact={baseline_exact} elapsed_ms={elapsed_ms}"
        );
        return;
    }

    let mut candidate_passes = 0;
    let mut candidate_exact = true;
    for seed in SEEDS {
        let root = ROOT_BASE + seed;
        let reference = run(root, MechanicalConfig::REFERENCE, Arm::Executable);
        let replay = run(root, MechanicalConfig::REFERENCE, Arm::Executable);
        let production = run(root, MechanicalConfig::PRODUCTION, Arm::Executable);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        let exact = replay_exact && mechanics_exact;
        candidate_passes += usize::from(reference.pass && exact);
        candidate_exact &= exact;
        append_row(
            &mut csv,
            root,
            "reference",
            Arm::Executable,
            &reference,
            replay_exact,
            mechanics_exact,
        );
        append_row(
            &mut csv,
            root,
            "replay",
            Arm::Executable,
            &replay,
            replay_exact,
            mechanics_exact,
        );
        append_row(
            &mut csv,
            root,
            "production",
            Arm::Executable,
            &production,
            replay_exact,
            mechanics_exact,
        );
    }
    fs::write(destination.join("matrix.csv"), csv).expect("write compact E27 matrix");
    let elapsed_ms = started.elapsed().as_millis();
    let exact = baseline_exact && candidate_exact;
    let status = if candidate_passes == SEEDS.len() && exact {
        "PASSED_8_OF_8"
    } else {
        "FALSIFIED_AFTER_VALID_BASELINE"
    };
    let report = format!(
        "# CORE1 E27 consolidation-born executability v2 result\n\n- status: `{status}`\n- compact E26 baseline: `{baseline_passes}/8`\n- E27 autonomous solve: `{candidate_passes}/8`\n- exact replay/mechanics: `{exact}`\n- harness elapsed milliseconds: `{elapsed_ms}`\n"
    );
    fs::write(destination.join("report.md"), report).expect("write compact E27 report");
    println!(
        "CORE1_E27_CONSOLIDATION_BORN_EXECUTABILITY_V2_COMPLETE status={status} baseline={baseline_passes}/8 autonomous={candidate_passes}/8 exact={exact} elapsed_ms={elapsed_ms}"
    );
}
