#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use academy_arc3::{
    spatial_context, Arc3CandidateLinkDiagnostic, Arc3ContextDiagnostic, Arc3Sensorimotor,
    ARC3_FRAME_PIXELS,
};
use truelearner_core::{Core0Profile, MechanicalConfig};

const PROFILES: [(Core0Profile, &str); 3] = [
    (Core0Profile::B, "CORE1-A"),
    (Core0Profile::GenericExternal, "CORE1-B"),
    (Core0Profile::GenericActivity, "CORE1-C"),
];
const ACTIONS: [u8; 4] = [1, 4, 2, 3];

#[derive(Clone, Debug, PartialEq, Eq)]
struct ContextRow {
    index: usize,
    context: u16,
    action: u8,
    motor: u8,
    context_activity: bool,
    direct_positive: usize,
    direct_negative: usize,
    contact_positive: usize,
    contact_negative: usize,
    topology_exists: bool,
    candidate_traversed: bool,
    motor_crossing: Option<u8>,
    outward_crossings: usize,
    world_changed: bool,
    support_admitted: bool,
    modulatory_deliveries: u64,
    participation_at_consequence: u64,
    material_changed: bool,
    plasticity_updates: u64,
    fresh_action: Option<u8>,
    fresh_participation: u64,
    quiescent: bool,
    work: u64,
    before: Arc3ContextDiagnostic,
    after: Arc3ContextDiagnostic,
    after_consequence: Arc3ContextDiagnostic,
    probe: Arc3ContextDiagnostic,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DiagnosticRun {
    rows: Vec<ContextRow>,
    class: &'static str,
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

fn link_totals(snapshot: &Arc3ContextDiagnostic) -> (usize, usize, usize, usize) {
    let mut direct_positive = 0;
    let mut direct_negative = 0;
    let mut contact_positive = 0;
    let mut contact_negative = 0;
    for link in &snapshot.links {
        let direct = link.role == "direct";
        match (direct, link.coupling.is_positive()) {
            (true, true) => direct_positive += 1,
            (true, false) => direct_negative += 1,
            (false, true) if link.role == "outgoing" => contact_positive += 1,
            (false, false) if link.role == "outgoing" => contact_negative += 1,
            _ => {}
        }
    }
    (
        direct_positive,
        direct_negative,
        contact_positive,
        contact_negative,
    )
}

fn participation(snapshot: &Arc3ContextDiagnostic) -> u64 {
    snapshot.links.iter().map(|link| link.participation).sum()
}

fn material_changed(before: &Arc3ContextDiagnostic, after: &Arc3ContextDiagnostic) -> bool {
    before.links.iter().any(|left| {
        after.links.iter().any(|right| {
            left.arrow == right.arrow
                && (left.coupling != right.coupling || left.resistance != right.resistance)
        })
    })
}

fn classify(rows: &[ContextRow]) -> (&'static str, String) {
    if rows.iter().any(|row| !row.topology_exists) {
        return ("A", "candidate action topology does not appear".to_string());
    }
    if rows.iter().any(|row| !row.candidate_traversed) {
        return (
            "B",
            "candidate action topology appears but does not traverse".to_string(),
        );
    }
    if rows.iter().any(|row| row.motor_crossing.is_none()) {
        return (
            "B",
            "candidate action topology traverses but does not produce motor output".to_string(),
        );
    }
    if rows
        .iter()
        .any(|row| !row.world_changed || !row.support_admitted)
    {
        return (
            "C",
            "action occurs but consequence is not admitted".to_string(),
        );
    }
    if rows
        .iter()
        .any(|row| row.modulatory_deliveries == 0 || row.participation_at_consequence == 0)
    {
        return (
            "D",
            "consequence does not meet remaining participation".to_string(),
        );
    }
    if rows.iter().any(|row| !row.material_changed) {
        return (
            "D",
            "consequence returns but material does not change".to_string(),
        );
    }
    if rows.iter().any(|row| row.fresh_action.is_none()) {
        return (
            "E",
            "material changes but fresh context does not re-execute".to_string(),
        );
    }
    (
        "POSITIVE",
        "complete context-action chain closes".to_string(),
    )
}

fn run(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> DiagnosticRun {
    let frames = frames();
    let contexts = frames
        .iter()
        .take(4)
        .map(|frame| spatial_context(frame).expect("valid context"))
        .collect::<Vec<_>>();
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, profile).expect("Academy body");
    let before = contexts
        .iter()
        .zip(ACTIONS)
        .map(|(context, action)| {
            organism
                .diagnostic_context(*context, motor(action))
                .expect("before diagnostic")
        })
        .collect::<Vec<_>>();
    let mut observations = Vec::new();
    let mut after = Vec::new();
    let mut after_consequence = vec![None; 4];
    for (index, action) in ACTIONS.into_iter().enumerate() {
        let observed = organism
            .observe(
                frames[index].clone(),
                &[1, 2, 3, 4],
                Some(action),
                index > 0,
                false,
                &[1, 2, 3, 4],
            )
            .expect("training observation");
        observations.push(observed);
        after.push(
            organism
                .diagnostic_context(contexts[index], motor(action))
                .expect("after diagnostic"),
        );
        if index > 0 {
            after_consequence[index - 1] = Some(
                organism
                    .diagnostic_context(contexts[index - 1], motor(ACTIONS[index - 1]))
                    .expect("consequence diagnostic"),
            );
        }
    }
    let closure = organism
        .observe(
            frames[4].clone(),
            &[1, 2, 3, 4],
            None,
            true,
            false,
            &[1, 2, 3, 4],
        )
        .expect("closure observation");
    after_consequence[3] = Some(
        organism
            .diagnostic_context(contexts[3], motor(ACTIONS[3]))
            .expect("final consequence diagnostic"),
    );
    let mut consequence = observations.iter().skip(1).cloned().collect::<Vec<_>>();
    consequence.push(closure);

    organism.clear_episode();
    let mut probe_observations = Vec::new();
    let mut probe_snapshots = Vec::new();
    for (index, action) in ACTIONS.into_iter().enumerate() {
        let observed = organism
            .observe(
                frames[index].clone(),
                &[1, 2, 3, 4],
                None,
                false,
                false,
                &[1, 2, 3, 4],
            )
            .expect("probe observation");
        probe_observations.push(observed);
        probe_snapshots.push(
            organism
                .diagnostic_context(contexts[index], motor(action))
                .expect("probe diagnostic"),
        );
        organism.clear_episode();
    }

    let rows = (0..4)
        .map(|index| {
            let after_consequence = after_consequence[index]
                .clone()
                .expect("consequence snapshot complete");
            let (direct_positive, direct_negative, contact_positive, contact_negative) =
                link_totals(&after[index]);
            ContextRow {
                index,
                context: contexts[index],
                action: ACTIONS[index],
                motor: motor(ACTIONS[index]),
                context_activity: observations[index].physical_work > 0,
                direct_positive,
                direct_negative,
                contact_positive,
                contact_negative,
                topology_exists: !after[index].links.is_empty(),
                candidate_traversed: participation(&after[index]) > participation(&before[index]),
                motor_crossing: observations[index].motor_crossing,
                outward_crossings: observations[index].outward_crossings,
                world_changed: consequence[index].frame_changed == Some(true),
                support_admitted: consequence[index].support_admitted,
                modulatory_deliveries: consequence[index].modulatory_deliveries,
                participation_at_consequence: participation(&after_consequence),
                material_changed: material_changed(&after[index], &after_consequence),
                plasticity_updates: consequence[index].plasticity_updates,
                fresh_action: probe_observations[index].action,
                fresh_participation: participation(&probe_snapshots[index]),
                quiescent: observations[index].naturally_quiescent
                    && consequence[index].naturally_quiescent
                    && probe_observations[index].naturally_quiescent,
                work: observations[index]
                    .physical_work
                    .saturating_add(consequence[index].physical_work)
                    .saturating_add(probe_observations[index].physical_work),
                before: before[index].clone(),
                after: after[index].clone(),
                after_consequence,
                probe: probe_snapshots[index].clone(),
            }
        })
        .collect::<Vec<_>>();
    let (class, reason) = classify(&rows);
    DiagnosticRun {
        rows,
        class,
        reason,
    }
}

fn write_contexts(csv: &mut BufWriter<File>, profile: &str, mechanics: &str, run: &DiagnosticRun) {
    for row in &run.rows {
        writeln!(
            csv,
            "{profile},{mechanics},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            row.index,
            row.context,
            row.action,
            row.motor,
            row.context_activity,
            row.direct_positive,
            row.direct_negative,
            row.contact_positive,
            row.contact_negative,
            row.topology_exists,
            row.candidate_traversed,
            row.motor_crossing.map_or_else(|| "none".to_string(), |value| value.to_string()),
            row.outward_crossings,
            row.world_changed,
            row.support_admitted,
            row.modulatory_deliveries,
            row.participation_at_consequence,
            row.material_changed,
            row.plasticity_updates,
            row.fresh_action.map_or_else(|| "none".to_string(), |value| value.to_string()),
            row.fresh_participation,
            row.quiescent,
            row.work,
            run.class,
            run.reason.replace(',', ";")
        )
        .expect("write context row");
        csv.flush().expect("flush context row");
    }
}

fn write_links(csv: &mut BufWriter<File>, profile: &str, mechanics: &str, rows: &[ContextRow]) {
    for row in rows {
        for (stage, snapshot) in [
            ("before", &row.before),
            ("after", &row.after),
            ("consequence", &row.after_consequence),
            ("probe", &row.probe),
        ] {
            for Arc3CandidateLinkDiagnostic {
                role,
                contact,
                arrow,
                coupling,
                resistance,
                participation,
            } in &snapshot.links
            {
                writeln!(
                    csv,
                    "{profile},{mechanics},{},{},{stage},{role},{},{},{coupling},{resistance},{participation}",
                    row.index,
                    row.context,
                    contact.map_or_else(|| "none".to_string(), |value| value.0.to_string()),
                    arrow.0
                )
                .expect("write link row");
                csv.flush().expect("flush link row");
            }
        }
    }
}

fn main() {
    eprintln!("CORE1_E13D_V1_EVIDENCE_SPENT");
    let destination = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_e13d_v1"));
    fs::create_dir_all(&destination).expect("create result directory");
    let mut contexts =
        BufWriter::new(File::create(destination.join("contexts.csv")).expect("create context CSV"));
    contexts.write_all(b"profile,mechanics,index,context,intended_action,motor,context_activity,direct_positive,direct_negative,contact_positive,contact_negative,topology_exists,candidate_traversed,motor_crossing,outward_crossings,world_changed,support_admitted,modulatory_deliveries,participation_at_consequence,material_changed,plasticity_updates,fresh_action,fresh_participation,quiescent,physical_work,class,reason\n").expect("write context header");
    let mut links =
        BufWriter::new(File::create(destination.join("links.csv")).expect("create link CSV"));
    links.write_all(b"profile,mechanics,index,context,stage,role,contact,arrow,coupling,resistance,participation\n").expect("write link header");
    let mut summary = String::from("# CORE1-E13D result\n\n| Profile | Class | First broken link | Replay exact | Mechanics exact |\n|---|---|---|---|---|\n");
    for (index, (profile, name)) in PROFILES.into_iter().enumerate() {
        let root = 60_000_000 + u64::try_from(index).unwrap_or(0) * 1_000_000;
        let reference = run(profile, MechanicalConfig::REFERENCE, root);
        write_contexts(&mut contexts, name, "reference", &reference);
        write_links(&mut links, name, "reference", &reference.rows);
        let replay = run(profile, MechanicalConfig::REFERENCE, root);
        write_contexts(&mut contexts, name, "replay", &replay);
        write_links(&mut links, name, "replay", &replay.rows);
        let production = run(profile, MechanicalConfig::PRODUCTION, root);
        write_contexts(&mut contexts, name, "production", &production);
        write_links(&mut links, name, "production", &production.rows);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        use std::fmt::Write as _;
        writeln!(
            summary,
            "| {name} | {} | {} | {replay_exact} | {mechanics_exact} |",
            reference.class, reference.reason
        )
        .expect("write summary row");
        fs::write(destination.join("summary.md"), &summary).expect("stream summary");
    }
    println!("CORE1_E13D_COMPLETE profiles=3 contexts=4");
}
