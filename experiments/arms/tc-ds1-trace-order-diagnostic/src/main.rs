#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode,
};

const ROOT: u64 = 1_100_000;

#[derive(Debug)]
struct DiagnosticRun {
    trace: Vec<PhysicalTransition>,
    a: ArrowId,
    b: ArrowId,
    a_level: u64,
    b_level: u64,
    a_contacts: Vec<u64>,
    b_contacts: Vec<u64>,
    a_updates: u64,
    b_updates: u64,
    causal_work: u64,
    body_hash: String,
    final_tick: i64,
    pressure_phase: i64,
    quiescent: bool,
}

fn add_cell(body: &mut PlasticSubstrate, physical_id: u64, position: i32, threshold: i32) -> truelearner_core::CellId {
    body.add_cell(truelearner_core::CellSpec {
        physical_id,
        position,
        region: 0,
        threshold,
        resistance: 100,
    })
}

fn add_arrow(
    body: &mut PlasticSubstrate,
    from: truelearner_core::CellId,
    to: truelearner_core::CellId,
    mode: TransmissionMode,
) -> ArrowId {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: 100,
        mode,
    })
}

fn input(target: truelearner_core::CellId, origin: u64, impulse: i32) -> SpikeInput {
    SpikeInput {
        arrival_tick: 0,
        phase: 0,
        origin_physical: origin,
        target,
        impulse,
    }
}

fn execute(mechanics: MechanicalConfig) -> DiagnosticRun {
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(ROOT + 20), 10, 10, mechanics);
    body.set_physical_tracing(true);
    let source = add_cell(&mut body, ROOT + 11, 0, 1);
    let target_a = add_cell(&mut body, ROOT + 12, 10, 2);
    let target_b = add_cell(&mut body, ROOT + 13, 20, 2);
    add_cell(&mut body, ROOT + 14, 30, 1);
    let a = add_arrow(&mut body, source, target_a, TransmissionMode::Drive);
    let b = add_arrow(&mut body, source, target_b, TransmissionMode::Drive);
    add_arrow(&mut body, target_a, source, TransmissionMode::Modulatory);

    let mut trace = Vec::new();
    let first = body.arrive(&[input(source, ROOT + 301, 1)], 99);
    assert!(first.naturally_quiescent);
    let mut causal_work = first.work.physical_total();
    trace.extend(first.physical_trace);
    let second = body.arrive(&[input(target_a, ROOT + 302, 2)], 99);
    assert!(second.naturally_quiescent);
    causal_work = causal_work.saturating_add(second.work.physical_total());
    trace.extend(second.physical_trace);

    let mut a_contacts = Vec::new();
    let mut b_contacts = Vec::new();
    let mut a_updates = 0_u64;
    let mut b_updates = 0_u64;
    for transition in &trace {
        match &transition.event {
            PhysicalEvent::ParticipationContact { arrow, level } if *arrow == a => {
                a_contacts.push(*level);
            }
            PhysicalEvent::ParticipationContact { arrow, level } if *arrow == b => {
                b_contacts.push(*level);
            }
            PhysicalEvent::Resistance { arrow, .. } if *arrow == a => {
                a_updates = a_updates.saturating_add(1);
            }
            PhysicalEvent::Resistance { arrow, .. } if *arrow == b => {
                b_updates = b_updates.saturating_add(1);
            }
            _ => {}
        }
    }

    DiagnosticRun {
        a_level: body.path_participation(a),
        b_level: body.path_participation(b),
        a_contacts,
        b_contacts,
        a_updates,
        b_updates,
        causal_work,
        body_hash: ContentHash::of(&body.canonical_body_bytes(1).unwrap()).to_string(),
        final_tick: body.clock().tick,
        pressure_phase: body.clock().pressure_phase(),
        quiescent: true,
        trace,
        a,
        b,
    }
}

fn variant(event: &PhysicalEvent) -> &'static str {
    match event {
        PhysicalEvent::Deliver { .. } => "Deliver",
        PhysicalEvent::Fire { .. } => "Fire",
        PhysicalEvent::Eligible { .. } => "Eligible",
        PhysicalEvent::Participation { .. } => "Participation",
        PhysicalEvent::ParticipationContact { .. } => "ParticipationContact",
        PhysicalEvent::Resistance { .. } => "Resistance",
        PhysicalEvent::Deallocate { .. } => "Deallocate",
        PhysicalEvent::Proposal { .. } => "Proposal",
        PhysicalEvent::Crossing(_) => "Crossing",
    }
}

fn line(transition: &PhysicalTransition) -> String {
    format!(
        "{}|{}|{}|{:?}",
        transition.tick,
        transition.phase,
        variant(&transition.event),
        transition.event
    )
}

fn retained(event: &PhysicalEvent) -> bool {
    !matches!(
        event,
        PhysicalEvent::Participation { .. } | PhysicalEvent::ParticipationContact { .. }
    )
}

fn hash_lines(lines: &[String]) -> String {
    ContentHash::of(lines.join("\n").as_bytes()).to_string()
}

fn hashes(trace: &[PhysicalTransition], retained_only: bool) -> (String, String, Vec<String>) {
    let ordered = trace
        .iter()
        .filter(|transition| !retained_only || retained(&transition.event))
        .map(line)
        .collect::<Vec<_>>();
    let mut canonical = ordered.clone();
    canonical.sort();
    (hash_lines(&ordered), hash_lines(&canonical), ordered)
}

fn counts(trace: &[PhysicalTransition]) -> BTreeMap<&'static str, usize> {
    let mut counts = BTreeMap::new();
    for transition in trace {
        *counts.entry(variant(&transition.event)).or_default() += 1;
    }
    counts
}

fn write_checksums(output: &Path) {
    let mut sums = String::new();
    for name in ["transitions.csv", "report.md"] {
        let bytes = fs::read(output.join(name)).unwrap();
        writeln!(sums, "{}  {name}", ContentHash::of(&bytes)).unwrap();
    }
    fs::write(output.join("SHA256SUMS"), sums).unwrap();
}

fn main() {
    let output = env::args_os().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("results/tc_ds1_trace_order_negative_diagnostic_v1")
    });
    fs::create_dir_all(&output).unwrap();

    let reference = execute(MechanicalConfig::REFERENCE);
    let production = execute(MechanicalConfig::PRODUCTION);
    let (reference_ordered_hash, reference_multiset_hash, reference_lines) =
        hashes(&reference.trace, false);
    let (production_ordered_hash, production_multiset_hash, production_lines) =
        hashes(&production.trace, false);
    let (reference_retained_ordered_hash, reference_retained_multiset_hash, _) =
        hashes(&reference.trace, true);
    let (production_retained_ordered_hash, production_retained_multiset_hash, _) =
        hashes(&production.trace, true);

    let first_divergence = reference_lines
        .iter()
        .zip(&production_lines)
        .position(|(left, right)| left != right)
        .or_else(|| (reference_lines.len() != production_lines.len()).then_some(reference_lines.len().min(production_lines.len())));

    assert_eq!(reference.a, production.a);
    assert_eq!(reference.b, production.b);
    assert_eq!(reference.a_level, production.a_level);
    assert_eq!(reference.b_level, production.b_level);
    assert_eq!(reference.a_contacts, production.a_contacts);
    assert_eq!(reference.b_contacts, production.b_contacts);
    assert_eq!(reference.a_updates, production.a_updates);
    assert_eq!(reference.b_updates, production.b_updates);
    assert_eq!(reference.causal_work, production.causal_work);
    assert_eq!(reference.body_hash, production.body_hash);
    assert_eq!(reference.final_tick, production.final_tick);
    assert_eq!(reference.pressure_phase, production.pressure_phase);
    assert_eq!(reference.quiescent, production.quiescent);
    assert_eq!(reference_multiset_hash, production_multiset_hash);

    let mut csv = String::from("mechanics,index,tick,phase,variant,event\n");
    for (mechanics, trace) in [
        ("reference", &reference.trace),
        ("production", &production.trace),
    ] {
        for (index, transition) in trace.iter().enumerate() {
            writeln!(
                csv,
                "{mechanics},{index},{},{},{},\"{:?}\"",
                transition.tick,
                transition.phase,
                variant(&transition.event),
                transition.event
            )
            .unwrap();
        }
    }

    let divergent_index = first_divergence.expect("frozen negative must diverge in order");
    let report = format!(
        "# TC-DS1 trace-order negative diagnostic v1\n\n\
         - root / pressure phase / return delay: `{ROOT} / 0 / 0`\n\
         - Reference trace length: `{}`\n\
         - Production trace length: `{}`\n\
         - first ordered divergence: `{divergent_index}`\n\
         - Reference event: `{}`\n\
         - Production event: `{}`\n\
         - Reference ordered hash: `{reference_ordered_hash}`\n\
         - Production ordered hash: `{production_ordered_hash}`\n\
         - full multiset hashes equal: `{}`\n\
         - retained ordered hashes equal: `{}`\n\
         - retained multiset hashes equal: `{}`\n\
         - Reference counts: `{:?}`\n\
         - Production counts: `{:?}`\n\
         - final body/work/clock/quiescence equal: `true`\n\
         - A/B contacts: `{:?} / {:?}`\n\
         - A/B updates: `{}/{}`\n\n\
         Classification: identical complete transition multisets and retained\n\
         future state, with a mechanics-dependent ordering difference.\n",
        reference.trace.len(),
        production.trace.len(),
        reference_lines[divergent_index],
        production_lines[divergent_index],
        reference_multiset_hash == production_multiset_hash,
        reference_retained_ordered_hash == production_retained_ordered_hash,
        reference_retained_multiset_hash == production_retained_multiset_hash,
        counts(&reference.trace),
        counts(&production.trace),
        reference.a_contacts,
        reference.b_contacts,
        reference.a_updates,
        reference.b_updates,
    );
    fs::write(output.join("transitions.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!("TC_DS1_TRACE_DIAGNOSTIC_COMPLETE first_divergence={divergent_index}");
}
