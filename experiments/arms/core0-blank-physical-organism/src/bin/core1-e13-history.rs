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
    reinforcement_action: Option<u8>,
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
) -> Result<academy_arc3::Arc3SensorimotorObservation, String> {
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
        .map_err(|error| error.to_string())
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
        let prime = organism
            .observe(
                frames[index].clone(),
                &AVAILABLE,
                None,
                false,
                false,
                &AVAILABLE,
            )
            .expect("first context prime");
        organism.clear_episode();
        let second_prime = organism
            .observe(
                frames[index].clone(),
                &AVAILABLE,
                None,
                false,
                false,
                &AVAILABLE,
            )
            .expect("second context prime");
        organism.clear_episode();
        organism
            .advance_gap(1)
            .expect("ordinary refractory recovery before history controls");
        let mut negative_branch = organism.clone();
        let negative = match history(&mut negative_branch, &frames[index], intended, -1) {
            Ok(observation) => observation,
            Err(error) => {
                return Run {
                    rows,
                    pass: false,
                    reason: format!(
                        "context {} action {intended} has no exercisable signed contact pair: {error}",
                        contexts[index]
                    ),
                };
            }
        };
        let positive = match history(&mut organism, &frames[index], intended, 1) {
            Ok(observation) => observation,
            Err(error) => {
                return Run {
                    rows,
                    pass: false,
                    reason: format!(
                        "context {} action {intended} positive history failed: {error}",
                        contexts[index]
                    ),
                };
            }
        };
        let material = organism
            .diagnostic_context(contexts[index], motor(intended))
            .expect("post-history material");
        rows.push(ContextResult {
            context: contexts[index],
            intended,
            prime_action: prime.action.or(second_prime.action),
            negative_early_action: negative.action,
            positive_early_action: positive.action,
            reinforcement_action: None,
            consequence_admitted: false,
            consequence_modulation: 0,
            consequence_updates: 0,
            probe_action: None,
            after_histories: material.clone(),
            after_consequence: material.clone(),
            after_probe: material,
            quiescent: prime.naturally_quiescent
                && second_prime.naturally_quiescent
                && negative.naturally_quiescent
                && positive.naturally_quiescent,
            work: prime
                .physical_work
                .saturating_add(second_prime.physical_work)
                .saturating_add(negative.physical_work)
                .saturating_add(positive.physical_work),
        });
        let consequence = organism
            .admit_previous_consequence()
            .expect("separate world consequence");
        let current = rows.last_mut().expect("current context result exists");
        current.consequence_admitted = consequence.admitted;
        current.consequence_modulation = consequence.modulatory_deliveries;
        current.consequence_updates = consequence.plasticity_updates;
        current.after_consequence = organism
            .diagnostic_context(contexts[index], motor(intended))
            .expect("consequence material");
        current.quiescent &= consequence.naturally_quiescent;
        current.work = current.work.saturating_add(consequence.physical_work);
        organism
            .advance_gap(1)
            .expect("ordinary recovery before fixed reinforcement");
        let reinforcement = match history(&mut organism, &frames[index], intended, 1) {
            Ok(observation) => observation,
            Err(error) => {
                return Run {
                    rows,
                    pass: false,
                    reason: format!(
                        "context {} action {intended} route is absent before fixed reinforcement: {error}",
                        contexts[index]
                    ),
                };
            }
        };
        let reinforced_consequence = organism
            .admit_previous_consequence()
            .expect("fixed reinforcement consequence");
        let current = rows.last_mut().expect("current context result exists");
        current.reinforcement_action = reinforcement.action;
        current.consequence_admitted &= reinforced_consequence.admitted;
        current.consequence_modulation = current
            .consequence_modulation
            .saturating_add(reinforced_consequence.modulatory_deliveries);
        current.consequence_updates = current
            .consequence_updates
            .saturating_add(reinforced_consequence.plasticity_updates);
        current.after_consequence = organism
            .diagnostic_context(contexts[index], motor(intended))
            .expect("reinforced consequence material");
        current.quiescent &=
            reinforcement.naturally_quiescent && reinforced_consequence.naturally_quiescent;
        current.work = current
            .work
            .saturating_add(reinforcement.physical_work)
            .saturating_add(reinforced_consequence.physical_work);
        organism
            .advance_gap(12)
            .expect("ordinary cleanup after fixed supported experiences");
    }

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
        if row.reinforcement_action != Some(row.intended) {
            pass = false;
            reason = "fixed second supported experience does not re-express the action".to_string();
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
            "{profile},{mechanics},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            row.context,
            row.intended,
            row.prime_action
                .map_or_else(|| "none".to_string(), |v| v.to_string()),
            row.negative_early_action
                .map_or_else(|| "none".to_string(), |v| v.to_string()),
            row.positive_early_action
                .map_or_else(|| "none".to_string(), |v| v.to_string()),
            row.reinforcement_action
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
    if args.iter().any(|arg| arg.starts_with("--preflight")) {
        let (profile, root) = if args.iter().any(|arg| arg == "--preflight-a") {
            (Core0Profile::B, 70_000_000)
        } else if args.iter().any(|arg| arg == "--preflight-c") {
            (Core0Profile::GenericActivity, 80_000_000)
        } else {
            (Core0Profile::GenericExternal, 75_000_000)
        };
        let result = run(profile, MechanicalConfig::REFERENCE, root);
        println!("{result:#?}");
        return;
    }

    eprintln!("CORE1_E13_PQLC_COMPOSITION_V3_EVIDENCE_SPENT");
    let destination = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_e13_pqlc_v3"));
    fs::create_dir_all(&destination).expect("create result directory");
    let mut csv =
        BufWriter::new(File::create(destination.join("matrix.csv")).expect("create matrix"));
    csv.write_all(b"profile,mechanics,context,intended,prime_action,negative_early_action,positive_early_action,reinforcement_action,consequence_admitted,modulatory_deliveries,plasticity_updates,probe_action,quiescent,physical_work,pass,reason,after_histories_material,after_consequence_material,after_probe_material\n").expect("write matrix header");
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
    println!("CORE1_E13_PQLC_COMPOSITION_V3_COMPLETE profiles=3 contexts=4");
}
