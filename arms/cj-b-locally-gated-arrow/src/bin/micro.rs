#![forbid(unsafe_code)]

use cj_b_locally_gated_arrow::{
    ArrowSpec, ArrowSummary, CellId, CellSpec, Execution, SpikeInput, Substrate, WorkLedger,
};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

const CSV_PATH: &str = "results/cj0_b_locally_gated_arrow_micro_v1.csv";
const REPORT_PATH: &str = "results/cj0_b_locally_gated_arrow_micro_v1.md";
const CSV_STAGE: &str = "results/.cj0_b_locally_gated_arrow_micro_v1.csv.staging";
const REPORT_STAGE: &str = "results/.cj0_b_locally_gated_arrow_micro_v1.md.staging";

#[derive(Clone, Copy)]
enum Duo {
    Ab,
    Cd,
    Ad,
    Cb,
}

impl Duo {
    fn source(self) -> usize {
        match self {
            Self::Ab | Self::Ad => 0,
            Self::Cd | Self::Cb => 1,
        }
    }

    fn contributor(self) -> usize {
        match self {
            Self::Ab | Self::Cb => 0,
            Self::Cd | Self::Ad => 1,
        }
    }
}

#[derive(Clone, Copy)]
struct Variant {
    name: &'static str,
    namespace: u64,
    mirror: bool,
    reverse_cells: bool,
    reverse_arrows: bool,
    reverse_insertion: bool,
}

#[derive(Clone)]
struct Field {
    substrate: Substrate,
    sources: [CellId; 2],
    sites: [CellId; 4],
    drivers: [CellId; 2],
    reverse_insertion: bool,
}

#[derive(Default)]
struct Observation {
    crossings: usize,
    candidate_consumptions: usize,
    source_firings: usize,
    output_firings: usize,
    quiescent: bool,
    work: u64,
}

struct Bootstrap {
    initially_dead: bool,
    changed_ad: usize,
    changed_cb: usize,
    no_return_ad: usize,
    no_return_cb: usize,
    new_ad: ArrowSummary,
    new_cb: ArrowSummary,
    quiescent: bool,
    work: u64,
}

struct Row {
    variant: Variant,
    initial_fp: u64,
    old_acquired_fp: u64,
    old_gap_fp: u64,
    changed_acquired_fp: u64,
    changed_gap_fp: u64,
    permanent_fp: u64,
    old_training_ab: usize,
    old_training_cd: usize,
    changed_training_ad: usize,
    changed_training_cb: usize,
    old_before_ab: Observation,
    old_before_cd: Observation,
    new_after_ad: Observation,
    new_after_cb: Observation,
    old_after_ab: Observation,
    old_after_cd: Observation,
    old_late_ab: Observation,
    old_late_cd: Observation,
    source_alone: Observation,
    contributor_alone: Observation,
    old_ab: ArrowSummary,
    old_cd: ArrowSummary,
    new_ad: ArrowSummary,
    new_cb: ArrowSummary,
    bootstrap: Bootstrap,
    duplicate_equal: bool,
    proposals: u64,
    deallocations: u64,
    arrow_count: usize,
    persistent_bytes: usize,
    total_work: u64,
    clauses: [bool; 10],
}

fn add_cell(
    substrate: &mut Substrate,
    physical_id: u64,
    position: i32,
    region: i16,
    threshold: i32,
) -> CellId {
    substrate.add_cell(CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 1_000,
    })
}

fn build_field(variant: Variant, candidate_resistance: u32, include_return: bool) -> Field {
    let sign = if variant.mirror { -1 } else { 1 };
    let positions = [0, 100, 1, 2, 101, 102, -20, 80, 30, 31, 130, 131].map(|value| value * sign);
    let regions = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1];
    let thresholds = [3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1];
    let mut substrate = Substrate::new();
    let mut handles: [Option<CellId>; 12] = [None; 12];
    let mut order = (0..12).collect::<Vec<_>>();
    if variant.reverse_cells {
        order.reverse();
    }
    for role in order {
        let offset = if variant.reverse_cells {
            12 - role
        } else {
            role + 1
        };
        handles[role] = Some(add_cell(
            &mut substrate,
            variant.namespace + u64::try_from(offset).expect("small role"),
            positions[role],
            regions[role],
            thresholds[role],
        ));
    }
    let cells = handles.map(|entry| entry.expect("complete numeric field"));
    let sources = [cells[0], cells[1]];
    let sites = [cells[2], cells[3], cells[4], cells[5]];
    let drivers = [cells[6], cells[7]];
    let outward = [cells[8], cells[9], cells[10], cells[11]];
    let mut arrows = Vec::new();
    for (source, site) in [
        (sources[0], sites[0]),
        (sources[0], sites[1]),
        (sources[1], sites[2]),
        (sources[1], sites[3]),
    ] {
        arrows.push(ArrowSpec {
            from: source,
            to: site,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: candidate_resistance,
        });
    }
    for (driver, source) in drivers.into_iter().zip(sources) {
        arrows.push(ArrowSpec {
            from: driver,
            to: source,
            delay: 0,
            phase: 0,
            coupling: 3,
            resistance: 1_000,
        });
    }
    for index in 0..4 {
        if include_return {
            arrows.push(ArrowSpec {
                from: sites[index],
                to: sources[usize::from(index >= 2)],
                delay: 0,
                phase: 1,
                coupling: 3,
                resistance: 1_000,
            });
        }
        arrows.push(ArrowSpec {
            from: sites[index],
            to: outward[index],
            delay: 0,
            phase: 2,
            coupling: 1,
            resistance: 1_000,
        });
    }
    if variant.reverse_arrows {
        arrows.reverse();
    }
    for arrow in arrows {
        substrate.add_arrow(arrow);
    }
    Field {
        substrate,
        sources,
        sites,
        drivers,
        reverse_insertion: variant.reverse_insertion,
    }
}

fn enter_contributor(field: &mut Field, contributor: usize, tick: i64) {
    let mut targets = if contributor == 0 {
        [field.sites[0], field.sites[3]]
    } else {
        [field.sites[1], field.sites[2]]
    };
    if field.reverse_insertion {
        targets.reverse();
    }
    for (index, target) in targets.into_iter().enumerate() {
        field.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: -10,
            origin_physical: 0x310 + u64::try_from(contributor * 2 + index).expect("small"),
            target,
            impulse: 2,
        });
    }
}

fn enter_source(field: &mut Field, source: usize, tick: i64, direct: bool) {
    field.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 0x320 + u64::try_from(source).expect("small"),
        target: if direct {
            field.sources[source]
        } else {
            field.drivers[source]
        },
        impulse: 3,
    });
}

fn execute_duo(field: &mut Field, duo: Duo, tick: i64, direct: bool) -> Execution {
    if field.reverse_insertion {
        enter_source(field, duo.source(), tick, direct);
        enter_contributor(field, duo.contributor(), tick);
    } else {
        enter_contributor(field, duo.contributor(), tick);
        enter_source(field, duo.source(), tick, direct);
    }
    field.substrate.propagate()
}

fn observation(field: &Field, run: &Execution) -> Observation {
    let source_ids = field
        .sources
        .map(|cell| field.substrate.cell_physical_id(cell));
    let site_ids = field
        .sites
        .map(|cell| field.substrate.cell_physical_id(cell));
    Observation {
        crossings: run.crossings.len(),
        candidate_consumptions: run
            .transmissions
            .iter()
            .filter(|entry| {
                entry.emitted
                    && entry.destination_state > 0
                    && source_ids.contains(&entry.from_physical)
                    && site_ids.contains(&entry.to_physical)
            })
            .count(),
        source_firings: run
            .trace
            .iter()
            .filter(|entry| entry.fired && source_ids.contains(&entry.target_physical))
            .count(),
        output_firings: run
            .trace
            .iter()
            .filter(|entry| entry.fired && site_ids.contains(&entry.target_physical))
            .count(),
        quiescent: run.naturally_quiescent,
        work: run.work.total(),
    }
}

fn observe_duo(field: &mut Field, duo: Duo) -> (Execution, Observation) {
    let tick = field.substrate.current_tick();
    let run = execute_duo(field, duo, tick, false);
    let summary = observation(field, &run);
    (run, summary)
}

fn observe_source(field: &mut Field, source: usize) -> Observation {
    let tick = field.substrate.current_tick();
    enter_source(field, source, tick, false);
    let run = field.substrate.propagate();
    observation(field, &run)
}

fn observe_contributor(field: &mut Field, contributor: usize) -> Observation {
    let tick = field.substrate.current_tick();
    enter_contributor(field, contributor, tick);
    let run = field.substrate.propagate();
    observation(field, &run)
}

fn train_initial(field: &mut Field, work: &mut WorkLedger) -> (usize, usize) {
    let mut ab = 0;
    let mut cd = 0;
    for round in 0..8 {
        let base = i64::from(round * 4);
        let order = if round % 2 == 0 {
            [(Duo::Ab, base), (Duo::Cd, base + 2)]
        } else {
            [(Duo::Cd, base), (Duo::Ab, base + 2)]
        };
        for (duo, tick) in order {
            let run = execute_duo(field, duo, tick, true);
            ab += usize::from(matches!(duo, Duo::Ab) && run.crossings.len() == 1);
            cd += usize::from(matches!(duo, Duo::Cd) && run.crossings.len() == 1);
            work.absorb(&run.work);
        }
    }
    (ab, cd)
}

fn train_changed(
    field: &mut Field,
    start: i64,
    rounds: usize,
    work: &mut WorkLedger,
) -> (usize, usize) {
    let mut ad = 0;
    let mut cb = 0;
    for round in 0..rounds {
        let base = start + i64::try_from(round).expect("small") * 10;
        let order = if round % 2 == 0 {
            [(Duo::Ad, base), (Duo::Cb, base + 4)]
        } else {
            [(Duo::Cb, base), (Duo::Ad, base + 4)]
        };
        for (duo, tick) in order {
            let run = execute_duo(field, duo, tick, true);
            ad += usize::from(matches!(duo, Duo::Ad) && run.crossings.len() == 1);
            cb += usize::from(matches!(duo, Duo::Cb) && run.crossings.len() == 1);
            work.absorb(&run.work);
        }
    }
    (ad, cb)
}

fn bootstrap(variant: Variant) -> Bootstrap {
    let mut field = build_field(
        Variant {
            namespace: variant.namespace + 0x10_000,
            ..variant
        },
        1,
        true,
    );
    let mut work = WorkLedger::default();
    let pressure = field.substrate.advance_time(30);
    work.absorb(&pressure);
    let initially_dead = [
        (field.sources[0], field.sites[0]),
        (field.sources[0], field.sites[1]),
        (field.sources[1], field.sites[2]),
        (field.sources[1], field.sites[3]),
    ]
    .iter()
    .all(|(from, to)| field.substrate.arrows_between(*from, *to).live == 0);
    let _ = train_changed(&mut field, 30, 8, &mut work);
    let pressure = field.substrate.advance_time(140);
    work.absorb(&pressure);
    let new_ad = field
        .substrate
        .arrows_between(field.sources[0], field.sites[1]);
    let new_cb = field
        .substrate
        .arrows_between(field.sources[1], field.sites[3]);
    let mut ad_field = field.clone();
    let (_, ad) = observe_duo(&mut ad_field, Duo::Ad);
    let mut cb_field = field.clone();
    let (_, cb) = observe_duo(&mut cb_field, Duo::Cb);

    let mut no_return = build_field(
        Variant {
            namespace: variant.namespace + 0x20_000,
            ..variant
        },
        1,
        false,
    );
    let pressure = no_return.substrate.advance_time(30);
    work.absorb(&pressure);
    let _ = train_changed(&mut no_return, 30, 8, &mut work);
    let pressure = no_return.substrate.advance_time(140);
    work.absorb(&pressure);
    let mut no_return_ad_field = no_return.clone();
    let (_, no_return_ad) = observe_duo(&mut no_return_ad_field, Duo::Ad);
    let mut no_return_cb_field = no_return.clone();
    let (_, no_return_cb) = observe_duo(&mut no_return_cb_field, Duo::Cb);

    Bootstrap {
        initially_dead,
        changed_ad: ad.crossings,
        changed_cb: cb.crossings,
        no_return_ad: no_return_ad.crossings,
        no_return_cb: no_return_cb.crossings,
        new_ad,
        new_cb,
        quiescent: ad.quiescent && cb.quiescent && no_return_ad.quiescent && no_return_cb.quiescent,
        work: work.total() + ad.work + cb.work + no_return_ad.work + no_return_cb.work,
    }
}

fn run_row(variant: Variant) -> Row {
    let mut field = build_field(variant, 1, true);
    let initial_fp = field.substrate.permanent_fingerprint();
    let mut work = WorkLedger::default();
    let (old_training_ab, old_training_cd) = train_initial(&mut field, &mut work);
    let old_acquired_fp = field.substrate.permanent_fingerprint();
    let gap = field.substrate.advance_time(50);
    work.absorb(&gap);
    let old_gap_fp = field.substrate.complete_fingerprint();

    let mut old_before_ab_field = field.clone();
    let (_, old_before_ab) = observe_duo(&mut old_before_ab_field, Duo::Ab);
    let mut old_before_cd_field = field.clone();
    let (_, old_before_cd) = observe_duo(&mut old_before_cd_field, Duo::Cd);

    let (changed_training_ad, changed_training_cb) = train_changed(&mut field, 50, 20, &mut work);
    let changed_acquired_fp = field.substrate.permanent_fingerprint();
    let gap = field.substrate.advance_time(280);
    work.absorb(&gap);
    let changed_gap_fp = field.substrate.complete_fingerprint();
    let permanent_fp = field.substrate.permanent_fingerprint();

    let old_ab = field
        .substrate
        .arrows_between(field.sources[0], field.sites[0]);
    let old_cd = field
        .substrate
        .arrows_between(field.sources[1], field.sites[2]);
    let new_ad = field
        .substrate
        .arrows_between(field.sources[0], field.sites[1]);
    let new_cb = field
        .substrate
        .arrows_between(field.sources[1], field.sites[3]);

    let mut new_after_ad_field = field.clone();
    let (duplicate_left, new_after_ad) = observe_duo(&mut new_after_ad_field, Duo::Ad);
    let mut duplicate_field = field.clone();
    let (duplicate_right, _) = observe_duo(&mut duplicate_field, Duo::Ad);
    let duplicate_equal = duplicate_left == duplicate_right
        && new_after_ad_field.substrate.complete_fingerprint()
            == duplicate_field.substrate.complete_fingerprint();
    let mut new_after_cb_field = field.clone();
    let (_, new_after_cb) = observe_duo(&mut new_after_cb_field, Duo::Cb);
    let mut old_after_ab_field = field.clone();
    let (_, old_after_ab) = observe_duo(&mut old_after_ab_field, Duo::Ab);
    let mut old_after_cd_field = field.clone();
    let (_, old_after_cd) = observe_duo(&mut old_after_cd_field, Duo::Cd);

    let mut late = field.clone();
    let late_gap = late.substrate.advance_time(330);
    let mut old_late_ab_field = late.clone();
    let (_, old_late_ab) = observe_duo(&mut old_late_ab_field, Duo::Ab);
    let mut old_late_cd_field = late.clone();
    let (_, old_late_cd) = observe_duo(&mut old_late_cd_field, Duo::Cd);
    let mut source_field = field.clone();
    let source_alone = observe_source(&mut source_field, 0);
    let mut contributor_field = field.clone();
    let contributor_alone = observe_contributor(&mut contributor_field, 1);
    let bootstrap = bootstrap(variant);

    let observations = [
        &old_before_ab,
        &old_before_cd,
        &new_after_ad,
        &new_after_cb,
        &old_after_ab,
        &old_after_cd,
        &old_late_ab,
        &old_late_cd,
        &source_alone,
        &contributor_alone,
    ];
    let observation_work: u64 = observations.iter().map(|entry| entry.work).sum();
    let quiescent = observations.iter().all(|entry| entry.quiescent) && bootstrap.quiescent;
    let finite_recurrence = observations
        .iter()
        .all(|entry| entry.source_firings <= 2 && entry.output_firings <= 1);
    let p0 = true;
    let p1 = old_training_ab == 8
        && old_training_cd == 8
        && old_before_ab.crossings == 1
        && old_before_cd.crossings == 1;
    let p2 = changed_training_ad == 20 && changed_training_cb == 20;
    let p3 = old_ab.live == 0 && old_cd.live == 0;
    let p4 = new_ad.live == 1
        && new_cb.live == 1
        && new_ad.coupling_max == 2
        && new_cb.coupling_max == 2
        && new_after_ad.crossings == 1
        && new_after_cb.crossings == 1
        && new_after_ad.output_firings == 1
        && new_after_cb.output_firings == 1;
    let p5 = old_after_ab.crossings == 0
        && old_after_cd.crossings == 0
        && old_late_ab.crossings == 0
        && old_late_cd.crossings == 0;
    let p6 = bootstrap.initially_dead
        && bootstrap.changed_ad == 1
        && bootstrap.changed_cb == 1
        && bootstrap.no_return_ad == 0
        && bootstrap.no_return_cb == 0
        && bootstrap.new_ad.live == 1
        && bootstrap.new_cb.live == 1;
    let p7 = source_alone.crossings == 0
        && source_alone.candidate_consumptions == 0
        && source_alone.output_firings == 0
        && contributor_alone.crossings == 0
        && contributor_alone.candidate_consumptions == 0
        && contributor_alone.output_firings == 0;
    let p8 = duplicate_equal && quiescent && finite_recurrence;
    let p9 = work.total() > 0 && bootstrap.work > 0;
    let proposals = work.local_structural_proposals;
    let deallocations = work.physical_deallocations;
    let arrow_count = field.substrate.arrow_count();
    let persistent_bytes = field.substrate.persistent_bytes();
    let total_work = work.total() + observation_work + late_gap.total() + bootstrap.work;
    Row {
        variant,
        initial_fp,
        old_acquired_fp,
        old_gap_fp,
        changed_acquired_fp,
        changed_gap_fp,
        permanent_fp,
        old_training_ab,
        old_training_cd,
        changed_training_ad,
        changed_training_cb,
        old_before_ab,
        old_before_cd,
        new_after_ad,
        new_after_cb,
        old_after_ab,
        old_after_cd,
        old_late_ab,
        old_late_cd,
        source_alone,
        contributor_alone,
        old_ab,
        old_cd,
        new_ad,
        new_cb,
        bootstrap,
        duplicate_equal,
        proposals,
        deallocations,
        arrow_count,
        persistent_bytes,
        total_work,
        clauses: [p0, p1, p2, p3, p4, p5, p6, p7, p8, p9],
    }
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "variant,namespace,initial_fp,old_acquired_fp,old_gap_fp,changed_acquired_fp,changed_gap_fp,permanent_fp,old_training_ab,old_training_cd,changed_training_ad,changed_training_cb,old_before_ab,old_before_cd,new_after_ad,new_after_cb,old_after_ab,old_after_cd,old_late_ab,old_late_cd,source_alone,source_consumptions,contributor_alone,contributor_consumptions,old_ab_records,old_ab_live,old_ab_resistance,old_ab_generation,old_cd_records,old_cd_live,old_cd_resistance,old_cd_generation,new_ad_records,new_ad_live,new_ad_resistance,new_ad_coupling,new_ad_generation,new_cb_records,new_cb_live,new_cb_resistance,new_cb_coupling,new_cb_generation,bootstrap_initially_dead,bootstrap_ad,bootstrap_cb,bootstrap_no_return_ad,bootstrap_no_return_cb,bootstrap_ad_live,bootstrap_ad_resistance,bootstrap_cb_live,bootstrap_cb_resistance,duplicate_equal,proposals,deallocations,arrow_count,persistent_bytes,total_work,P0,P1,P2,P3,P4,P5,P6,P7,P8,P9,row_pass\n",
    );
    for row in rows {
        let mut values = vec![
            row.variant.name.to_string(),
            format!("{:#x}", row.variant.namespace),
            format!("{:#018x}", row.initial_fp),
            format!("{:#018x}", row.old_acquired_fp),
            format!("{:#018x}", row.old_gap_fp),
            format!("{:#018x}", row.changed_acquired_fp),
            format!("{:#018x}", row.changed_gap_fp),
            format!("{:#018x}", row.permanent_fp),
            row.old_training_ab.to_string(),
            row.old_training_cd.to_string(),
            row.changed_training_ad.to_string(),
            row.changed_training_cb.to_string(),
            row.old_before_ab.crossings.to_string(),
            row.old_before_cd.crossings.to_string(),
            row.new_after_ad.crossings.to_string(),
            row.new_after_cb.crossings.to_string(),
            row.old_after_ab.crossings.to_string(),
            row.old_after_cd.crossings.to_string(),
            row.old_late_ab.crossings.to_string(),
            row.old_late_cd.crossings.to_string(),
            row.source_alone.crossings.to_string(),
            row.source_alone.candidate_consumptions.to_string(),
            row.contributor_alone.crossings.to_string(),
            row.contributor_alone.candidate_consumptions.to_string(),
            row.old_ab.records.to_string(),
            row.old_ab.live.to_string(),
            row.old_ab.resistance_max.to_string(),
            row.old_ab.generation_max.to_string(),
            row.old_cd.records.to_string(),
            row.old_cd.live.to_string(),
            row.old_cd.resistance_max.to_string(),
            row.old_cd.generation_max.to_string(),
            row.new_ad.records.to_string(),
            row.new_ad.live.to_string(),
            row.new_ad.resistance_max.to_string(),
            row.new_ad.coupling_max.to_string(),
            row.new_ad.generation_max.to_string(),
            row.new_cb.records.to_string(),
            row.new_cb.live.to_string(),
            row.new_cb.resistance_max.to_string(),
            row.new_cb.coupling_max.to_string(),
            row.new_cb.generation_max.to_string(),
            row.bootstrap.initially_dead.to_string(),
            row.bootstrap.changed_ad.to_string(),
            row.bootstrap.changed_cb.to_string(),
            row.bootstrap.no_return_ad.to_string(),
            row.bootstrap.no_return_cb.to_string(),
            row.bootstrap.new_ad.live.to_string(),
            row.bootstrap.new_ad.resistance_max.to_string(),
            row.bootstrap.new_cb.live.to_string(),
            row.bootstrap.new_cb.resistance_max.to_string(),
            row.duplicate_equal.to_string(),
            row.proposals.to_string(),
            row.deallocations.to_string(),
            row.arrow_count.to_string(),
            row.persistent_bytes.to_string(),
            row.total_work.to_string(),
        ];
        values.extend(row.clauses.iter().map(bool::to_string));
        values.push(row.clauses.iter().all(|value| *value).to_string());
        output.push_str(&values.join(","));
        output.push('\n');
    }
    output
}

fn report(rows: &[Row]) -> String {
    let passed = rows
        .iter()
        .filter(|row| row.clauses.iter().all(|value| *value))
        .count();
    let claims = rows
        .iter()
        .map(|row| row.clauses.iter().filter(|value| **value).count())
        .sum::<usize>();
    let work = rows.iter().map(|row| row.total_work).sum::<u64>();
    let status = if passed == rows.len() {
        "POSITIVE; GATE ELIGIBLE BUT UNSPENT"
    } else {
        "FROZEN NEGATIVE; GATE INELIGIBLE"
    };
    format!(
        "# CJ0 ARM CJ-B locally gated ARROW MICRO v1 result\n\nStatus: **{status}**.\n\n- conjunctive rows: `{passed}/{}`;\n- claims: `{claims}/{}`;\n- ledgered physical work: `{work}`;\n- authoritative/PX3/PX3-R bytes changed: `no`;\n- GATE/definitive/PX3/PX-C execution: `none`.\n\nOld held-out A+B/C+D before change: `{}`. New held-out A+D/C+B after change: `{}`. Old immediate/late held-out after change: `{}`. Full-deallocation bootstrap new/no-return: `{}`.\n\nOrdinary pressure and generic local proposals alone removed old support and formed changed support; no invalidation, relation-change signal, historical lookup, or new persistent variable exists. Exact fingerprints, structures, generations, controls, replay, work, storage, and clause bits are in the companion CSV.\n",
        rows.len(),
        rows.len() * 10,
        rows.iter().map(|row| format!("{}|{}", row.old_before_ab.crossings, row.old_before_cd.crossings)).collect::<Vec<_>>().join(";"),
        rows.iter().map(|row| format!("{}|{}", row.new_after_ad.crossings, row.new_after_cb.crossings)).collect::<Vec<_>>().join(";"),
        rows.iter().map(|row| format!("{}|{}|{}|{}", row.old_after_ab.crossings, row.old_after_cd.crossings, row.old_late_ab.crossings, row.old_late_cd.crossings)).collect::<Vec<_>>().join(";"),
        rows.iter().map(|row| format!("{}|{} / {}|{}", row.bootstrap.changed_ad, row.bootstrap.changed_cb, row.bootstrap.no_return_ad, row.bootstrap.no_return_cb)).collect::<Vec<_>>().join(";")
    )
}

fn create_stage(path: &str, contents: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("cannot create {path}: {error}"))?;
    file.write_all(contents.as_bytes())
        .map_err(|error| format!("cannot write {path}: {error}"))?;
    file.sync_all()
        .map_err(|error| format!("cannot sync {path}: {error}"))
}

fn publish(rows: &[Row]) -> Result<(), String> {
    for path in [CSV_PATH, REPORT_PATH, CSV_STAGE, REPORT_STAGE] {
        if Path::new(path).exists() {
            return Err(format!("refusing to overwrite {path}"));
        }
    }
    create_stage(CSV_STAGE, &csv(rows))?;
    if let Err(error) = create_stage(REPORT_STAGE, &report(rows)) {
        let _ = fs::remove_file(CSV_STAGE);
        return Err(error);
    }
    fs::rename(CSV_STAGE, CSV_PATH).map_err(|error| format!("publish CSV failed: {error}"))?;
    fs::rename(REPORT_STAGE, REPORT_PATH)
        .map_err(|error| format!("publish report failed: {error}"))?;
    Ok(())
}

fn execute_micro() -> Result<bool, String> {
    let variants = [
        Variant {
            name: "normal",
            namespace: 0x9_b510_0000,
            mirror: false,
            reverse_cells: false,
            reverse_arrows: false,
            reverse_insertion: false,
        },
        Variant {
            name: "mirror",
            namespace: 0x9_b520_0000,
            mirror: true,
            reverse_cells: false,
            reverse_arrows: false,
            reverse_insertion: true,
        },
        Variant {
            name: "reverse_allocation",
            namespace: 0x9_b530_0000,
            mirror: false,
            reverse_cells: true,
            reverse_arrows: true,
            reverse_insertion: false,
        },
        Variant {
            name: "permuted_orientation",
            namespace: 0x9_b540_0000,
            mirror: true,
            reverse_cells: true,
            reverse_arrows: true,
            reverse_insertion: true,
        },
    ];
    println!("CJ0_B_LOCALLY_GATED_ARROW_MICRO_V1_EVIDENCE_SPENT");
    let rows = variants.into_iter().map(run_row).collect::<Vec<_>>();
    let passed = rows
        .iter()
        .all(|row| row.clauses.iter().all(|value| *value));
    publish(&rows)?;
    Ok(passed)
}

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments == ["--preflight"] {
        println!(
            "{{\"arm\":\"CJ-B\",\"stage\":\"MICRO\",\"cells_entered\":0,\"artifacts_written\":0}}"
        );
        return ExitCode::SUCCESS;
    }
    if arguments != ["--micro"] {
        eprintln!("refusing execution: expected exactly --preflight or --micro");
        return ExitCode::from(2);
    }
    match execute_micro() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
