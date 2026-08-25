#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use academy_arc3::{
    spatial_context, Arc3ContextDiagnostic, Arc3Sensorimotor, Arc3TransientHistoryRequest,
    ARC3_FRAME_PIXELS,
};
use truelearner_core::{Core0Profile, MechanicalConfig};

const PROFILES: [(Core0Profile, &str); 3] = [
    (Core0Profile::B, "CORE1-A"),
    (Core0Profile::GenericExternal, "CORE1-B"),
    (Core0Profile::GenericActivity, "CORE1-C"),
];
const ACTIONS: [u8; 4] = [1, 4, 2, 3];
const AVAILABLE: [u8; 4] = [1, 2, 3, 4];

#[derive(Clone, Debug, PartialEq, Eq)]
struct ContextResult {
    context: u16,
    intended: u8,
    prime_action: Option<u8>,
    negative_early_action: Option<u8>,
    positive_early_action: Option<u8>,
    consequence_admitted: bool,
    consequence_modulation: u64,
    consequence_updates: u64,
    probe_action: Option<u8>,
    after_histories: Arc3ContextDiagnostic,
    after_consequence: Arc3ContextDiagnostic,
    after_probe: Arc3ContextDiagnostic,
    quiescent: bool,
    work: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Run {
    rows: Vec<ContextResult>,
    pass: bool,
    reason: String,
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
        let context = spatial_context(&candidate).expect("valid frame");
        if contexts.insert(context) {
            frames.push(candidate);
            if frames.len() == 5 {
                break;
            }
        }
    }
    frames
}

fn motor(action: u8) -> u8 {
    action.saturating_sub(1)
}

fn history(
    organism: &mut Arc3Sensorimotor,
    frame: &[u8],
    action: u8,
    early_material_sign: i8,
) -> academy_arc3::Arc3SensorimotorObservation {
    organism
        .observe_with_transient_history(Arc3TransientHistoryRequest {
            frame: frame.to_vec(),
            available_actions: &AVAILABLE,
            babble_action: action,
            support_previous: false,
            settle_pressure: false,
            action_map: &AVAILABLE,
            early_material_sign,
        })
        .expect("transient-history observation")
}

fn run(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Run {
    let frames = frames();
    let contexts = frames
        .iter()
        .take(4)
        .map(|frame| spatial_context(frame).expect("valid context"))
        .collect::<Vec<_>>();
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, profile).expect("Academy body");
    let mut rows: Vec<ContextResult> = Vec::new();
    for (index, intended) in ACTIONS.into_iter().enumerate() {
        if index > 0 {
            let transition = organism
                .observe(
                    frames[index].clone(),
                    &AVAILABLE,
                    None,
                    true,
                    false,
                    &AVAILABLE,
                )
                .expect("world consequence transition");
            let preceding = rows
                .get_mut(index - 1)
                .expect("preceding context result exists");
            preceding.consequence_admitted = transition.support_admitted;
            preceding.consequence_modulation = transition.modulatory_deliveries;
            preceding.consequence_updates = transition.plasticity_updates;
            preceding.after_consequence = organism
                .diagnostic_context(contexts[index - 1], motor(ACTIONS[index - 1]))
                .expect("preceding consequence material");
            preceding.quiescent &= transition.naturally_quiescent;
            preceding.work = preceding.work.saturating_add(transition.physical_work);
            organism
                .advance_gap(12)
                .expect("ordinary local forgetting gap");
        }
        let prime = organism
            .observe(
                frames[index].clone(),
                &AVAILABLE,
                None,
                false,
                false,
                &AVAILABLE,
            )
            .expect("context prime");
        organism.clear_episode();
        let negative = history(&mut organism, &frames[index], intended, -1);
        let positive = history(&mut organism, &frames[index], intended, 1);
        let material = organism
            .diagnostic_context(contexts[index], motor(intended))
            .expect("post-history material");
        rows.push(ContextResult {
            context: contexts[index],
            intended,
            prime_action: prime.action,
            negative_early_action: negative.action,
            positive_early_action: positive.action,
            consequence_admitted: false,
            consequence_modulation: 0,
            consequence_updates: 0,
            probe_action: None,
            after_histories: material.clone(),
            after_consequence: material.clone(),
            after_probe: material,
            quiescent: prime.naturally_quiescent
                && negative.naturally_quiescent
                && positive.naturally_quiescent,
            work: prime
                .physical_work
                .saturating_add(negative.physical_work)
                .saturating_add(positive.physical_work),
        });
    }
    let closure = organism
        .observe(frames[4].clone(), &AVAILABLE, None, true, false, &AVAILABLE)
        .expect("final consequence");
    let last = rows.last_mut().expect("four context rows");
    last.consequence_admitted = closure.support_admitted;
    last.consequence_modulation = closure.modulatory_deliveries;
    last.consequence_updates = closure.plasticity_updates;
    last.after_consequence = organism
        .diagnostic_context(contexts[3], motor(ACTIONS[3]))
        .expect("final consequence material");
    last.quiescent &= closure.naturally_quiescent;
    last.work = last.work.saturating_add(closure.physical_work);
    organism
        .advance_gap(12)
        .expect("final ordinary local forgetting gap");

    organism.clear_episode();
    for (index, intended) in ACTIONS.into_iter().enumerate() {
        let probe = organism
            .observe(
                frames[index].clone(),
                &AVAILABLE,
                None,
                false,
                false,
                &AVAILABLE,
            )
            .expect("fresh context probe");
        rows[index].probe_action = probe.action;
        rows[index].after_probe = organism
            .diagnostic_context(contexts[index], motor(intended))
            .expect("probe material");
        rows[index].quiescent &= probe.naturally_quiescent;
        rows[index].work = rows[index].work.saturating_add(probe.physical_work);
        organism.clear_episode();
    }

    let mut pass = true;
    let mut reason = "complete history-resolved E13 chain closes".to_string();
    for row in &rows {
        if row.prime_action.is_some() {
            pass = false;
            reason = "candidate-generation prime emits an unintended action".to_string();
            break;
        }
        if row.negative_early_action.is_some() || row.positive_early_action != Some(row.intended) {
            pass = false;
            reason =
                "fixed early/late history pair does not resolve the intended action".to_string();
            break;
        }
        if !row.consequence_admitted
            || row.consequence_modulation == 0
            || row.consequence_updates == 0
        {
            pass = false;
            reason = "resolved action does not close through ordinary consequence".to_string();
            break;
        }
        if row.probe_action != Some(row.intended) {
            pass = false;
            reason = "supported route does not re-execute on a fresh context".to_string();
            break;
        }
        if !row.quiescent {
            pass = false;
            reason = "history-resolved context does not quiesce naturally".to_string();
            break;
        }
    }
    Run { rows, pass, reason }
}

fn material_summary(snapshot: &Arc3ContextDiagnostic) -> String {
    snapshot
        .links
        .iter()
        .map(|link| {
            format!(
                "{}:{}:{}:{}",
                link.role, link.coupling, link.resistance, link.participation
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn write_rows(csv: &mut BufWriter<File>, profile: &str, mechanics: &str, run: &Run) {
    for row in &run.rows {
        writeln!(
            csv,
            "{profile},{mechanics},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            row.context,
            row.intended,
            row.prime_action
                .map_or_else(|| "none".to_string(), |v| v.to_string()),
            row.negative_early_action
                .map_or_else(|| "none".to_string(), |v| v.to_string()),
            row.positive_early_action
                .map_or_else(|| "none".to_string(), |v| v.to_string()),
            row.consequence_admitted,
            row.consequence_modulation,
            row.consequence_updates,
            row.probe_action
                .map_or_else(|| "none".to_string(), |v| v.to_string()),
            row.quiescent,
            row.work,
            run.pass,
            run.reason.replace(',', ";"),
            material_summary(&row.after_histories),
            material_summary(&row.after_consequence),
            material_summary(&row.after_probe)
        )
        .expect("write E13 row");
        csv.flush().expect("flush E13 row");
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == "--preflight") {
        let result = run(Core0Profile::B, MechanicalConfig::REFERENCE, 70_000_000);
        println!("{result:#?}");
        return;
    }

    eprintln!("CORE1_E13_HISTORY_COMPOSITION_V1_EVIDENCE_SPENT");
    let destination = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_e13_history_v1"));
    fs::create_dir_all(&destination).expect("create result directory");
    let mut csv =
        BufWriter::new(File::create(destination.join("matrix.csv")).expect("create matrix"));
    csv.write_all(b"profile,mechanics,context,intended,prime_action,negative_early_action,positive_early_action,consequence_admitted,modulatory_deliveries,plasticity_updates,probe_action,quiescent,physical_work,pass,reason,after_histories_material,after_consequence_material,after_probe_material\n").expect("write matrix header");
    let mut summary = String::from(
        "# CORE1 E13 history-composition result\n\n| Profile | E13 | Replay | Mechanics | First result |\n|---|---|---|---|---|\n",
    );
    for (index, (profile, name)) in PROFILES.into_iter().enumerate() {
        let root = 70_000_000 + u64::try_from(index).unwrap_or(0) * 5_000_000;
        let reference = run(profile, MechanicalConfig::REFERENCE, root);
        write_rows(&mut csv, name, "reference", &reference);
        let replay = run(profile, MechanicalConfig::REFERENCE, root);
        write_rows(&mut csv, name, "replay", &replay);
        let production = run(profile, MechanicalConfig::PRODUCTION, root);
        write_rows(&mut csv, name, "production", &production);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        use std::fmt::Write as _;
        writeln!(
            summary,
            "| {name} | {} | {replay_exact} | {mechanics_exact} | {} |",
            reference.pass, reference.reason
        )
        .expect("write summary row");
        fs::write(destination.join("summary.md"), &summary).expect("stream summary");
    }
    println!("CORE1_E13_HISTORY_COMPOSITION_COMPLETE profiles=3 contexts=4");
}
