#![forbid(unsafe_code)]

use cj_b_locally_gated_arrow::{
    ArrowSpec, ArrowSummary, CellId, CellSpec, Execution, SpikeInput, Substrate, WorkLedger,
};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

const CSV_PATH: &str = "results/cj0_b_locally_gated_arrow_probe_v1.csv";
const REPORT_PATH: &str = "results/cj0_b_locally_gated_arrow_probe_v1.md";
const CSV_STAGE: &str = "results/.cj0_b_locally_gated_arrow_probe_v1.csv.staging";
const REPORT_STAGE: &str = "results/.cj0_b_locally_gated_arrow_probe_v1.md.staging";

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
    consumed: u64,
    emitted: usize,
    source_firings: usize,
    output_firings: usize,
    quiescent: bool,
    work: u64,
}

struct Controls {
    too_late: usize,
    correlation_only: usize,
    no_return_reuse: usize,
    absent: usize,
    blocked: usize,
    stale: usize,
    ambiguity: usize,
    ambiguity_wrong: usize,
    quiescent: bool,
    work: u64,
}

struct Row {
    variant: Variant,
    initial_fingerprint: u64,
    acquired_fingerprint: u64,
    gap_fingerprint: u64,
    permanent_fingerprint: u64,
    training_ab: usize,
    training_cd: usize,
    training_consumed: u64,
    training_returns: u64,
    trained_ab: Observation,
    trained_cd: Observation,
    crossed_ad: Observation,
    crossed_cb: Observation,
    singleton_a: Observation,
    singleton_b: Observation,
    singleton_c: Observation,
    singleton_d: Observation,
    self_evidence: Observation,
    ab: ArrowSummary,
    cd: ArrowSummary,
    ad: ArrowSummary,
    cb: ArrowSummary,
    controls: Controls,
    duplicate_equal: bool,
    arrow_count: usize,
    persistent_bytes: usize,
    work: WorkLedger,
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
    let positions =
        [0, 100, 1, 2, 101, 102, -20, 80, 30, 31, 130, 131].map(|position| position * sign);
    let regions = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1];
    let thresholds = [3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1];
    let mut substrate = Substrate::new();
    let mut handles: [Option<CellId>; 12] = [None; 12];
    let mut order = (0..12).collect::<Vec<_>>();
    if variant.reverse_cells {
        order.reverse();
    }
    for role in order {
        let physical_offset = if variant.reverse_cells {
            12 - role
        } else {
            role + 1
        };
        handles[role] = Some(add_cell(
            &mut substrate,
            variant.namespace + u64::try_from(physical_offset).expect("small role"),
            positions[role],
            regions[role],
            thresholds[role],
        ));
    }
    let cells = handles.map(|handle| handle.expect("every physical role allocated"));
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
                to: sources[if index < 2 { 0 } else { 1 }],
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
            origin_physical: 0x100 + u64::try_from(contributor * 2 + index).expect("small"),
            target,
            impulse: 2,
        });
    }
}

fn enter_source(field: &mut Field, source: usize, tick: i64, direct: bool) {
    field.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 0x200 + u64::try_from(source).expect("small"),
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

fn observe(
    field: &mut Field,
    duo: Option<Duo>,
    source_only: Option<usize>,
) -> (Execution, Observation) {
    let tick = field.substrate.current_tick();
    match (duo, source_only) {
        (Some(active), None) => {
            enter_contributor(field, active.contributor(), tick);
            enter_source(field, active.source(), tick, false);
        }
        (None, Some(source)) => enter_source(field, source, tick, false),
        _ => panic!("one physical observation mode is required"),
    }
    let run = field.substrate.propagate();
    let source_ids = field
        .sources
        .map(|cell| field.substrate.cell_physical_id(cell));
    let site_ids = field
        .sites
        .map(|cell| field.substrate.cell_physical_id(cell));
    let emitted = run
        .transmissions
        .iter()
        .filter(|entry| entry.emitted)
        .count();
    let candidate_consumptions = run
        .transmissions
        .iter()
        .filter(|entry| {
            entry.emitted
                && entry.destination_state > 0
                && source_ids.contains(&entry.from_physical)
                && site_ids.contains(&entry.to_physical)
        })
        .count();
    let observation = Observation {
        crossings: run.crossings.len(),
        consumed: u64::try_from(candidate_consumptions).expect("candidate count fits u64"),
        emitted,
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
    };
    (run, observation)
}

fn observe_contributor_only(field: &mut Field, contributor: usize) -> Observation {
    let tick = field.substrate.current_tick();
    enter_contributor(field, contributor, tick);
    let run = field.substrate.propagate();
    Observation {
        crossings: run.crossings.len(),
        consumed: run.work.local_state_consumptions,
        emitted: run
            .transmissions
            .iter()
            .filter(|entry| entry.emitted)
            .count(),
        source_firings: 0,
        output_firings: run.trace.iter().filter(|entry| entry.fired).count(),
        quiescent: run.naturally_quiescent,
        work: run.work.total(),
    }
}

fn train_primary(field: &mut Field, work: &mut WorkLedger) -> (usize, usize, u64, u64) {
    let mut ab = 0;
    let mut cd = 0;
    let mut consumed = 0;
    let mut returns = 0;
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
            consumed += run.work.local_state_consumptions;
            returns += run.work.local_return_updates;
            work.absorb(&run.work);
        }
    }
    (ab, cd, consumed, returns)
}

fn train_one(field: &mut Field, duo: Duo, rounds: usize, spacing: i64) -> WorkLedger {
    let mut work = WorkLedger::default();
    for round in 0..rounds {
        let run = execute_duo(
            field,
            duo,
            i64::try_from(round).expect("small") * spacing,
            true,
        );
        work.absorb(&run.work);
    }
    work
}

fn run_controls(variant: Variant) -> Controls {
    let mut work_total = 0;
    let mut all_quiescent = true;

    let mut late = build_field(
        Variant {
            namespace: variant.namespace + 0x10_000,
            ..variant
        },
        1,
        true,
    );
    enter_source(&mut late, 0, 0, true);
    let late_a = late.substrate.propagate();
    enter_contributor(&mut late, 0, 2);
    let late_b = late.substrate.propagate();
    let too_late = late_a.crossings.len() + late_b.crossings.len();
    work_total += late_a.work.total() + late_b.work.total();
    all_quiescent &= late_a.naturally_quiescent && late_b.naturally_quiescent;

    let mut correlation = build_field(
        Variant {
            namespace: variant.namespace + 0x20_000,
            ..variant
        },
        1,
        true,
    );
    let mut correlation_only = 0;
    for tick in [0, 4, 8, 12] {
        enter_contributor(&mut correlation, 0, tick);
        let run = correlation.substrate.propagate();
        correlation_only += run.crossings.len();
        work_total += run.work.total();
        all_quiescent &= run.naturally_quiescent;
    }

    let mut no_return = build_field(
        Variant {
            namespace: variant.namespace + 0x30_000,
            ..variant
        },
        1,
        false,
    );
    let no_return_work = train_one(&mut no_return, Duo::Ab, 3, 6);
    work_total += no_return_work.total();
    let pressure = no_return.substrate.advance_time(50);
    work_total += pressure.total();
    let (_, no_return_observation) = observe(&mut no_return, Some(Duo::Ab), None);
    work_total += no_return_observation.work;
    all_quiescent &= no_return_observation.quiescent;

    let mut absent_field = build_field(
        Variant {
            namespace: variant.namespace + 0x40_000,
            ..variant
        },
        0,
        true,
    );
    let (_, absent_observation) = observe(&mut absent_field, Some(Duo::Ab), None);
    work_total += absent_observation.work;
    all_quiescent &= absent_observation.quiescent;

    let mut blocked_field = build_field(
        Variant {
            namespace: variant.namespace + 0x50_000,
            ..variant
        },
        1,
        true,
    );
    let blocked_pressure = blocked_field.substrate.advance_time(20);
    work_total += blocked_pressure.total();
    let (_, blocked_observation) = observe(&mut blocked_field, Some(Duo::Ab), None);
    work_total += blocked_observation.work;
    all_quiescent &= blocked_observation.quiescent;

    let mut stale_field = build_field(
        Variant {
            namespace: variant.namespace + 0x60_000,
            ..variant
        },
        1,
        true,
    );
    let stale_training = execute_duo(&mut stale_field, Duo::Ab, 0, true);
    work_total += stale_training.work.total();
    let stale_pressure = stale_field.substrate.advance_time(60);
    work_total += stale_pressure.total();
    let (_, stale_observation) = observe(&mut stale_field, Some(Duo::Ab), None);
    work_total += stale_observation.work;
    all_quiescent &= stale_training.naturally_quiescent && stale_observation.quiescent;

    let mut ambiguity_field = build_field(
        Variant {
            namespace: variant.namespace + 0x70_000,
            ..variant
        },
        1,
        true,
    );
    let ambiguity_training = train_one(&mut ambiguity_field, Duo::Ab, 8, 4);
    work_total += ambiguity_training.total();
    let ambiguity_pressure = ambiguity_field.substrate.advance_time(50);
    work_total += ambiguity_pressure.total();
    let tick = ambiguity_field.substrate.current_tick();
    enter_contributor(&mut ambiguity_field, 0, tick);
    enter_contributor(&mut ambiguity_field, 1, tick);
    enter_source(&mut ambiguity_field, 0, tick, false);
    let ambiguity_run = ambiguity_field.substrate.propagate();
    let ab_id = ambiguity_field
        .substrate
        .cell_physical_id(ambiguity_field.sites[0]);
    let ad_id = ambiguity_field
        .substrate
        .cell_physical_id(ambiguity_field.sites[1]);
    let ambiguity = ambiguity_run
        .trace
        .iter()
        .filter(|entry| entry.fired && entry.target_physical == ab_id)
        .count();
    let ambiguity_wrong = ambiguity_run
        .trace
        .iter()
        .filter(|entry| entry.fired && entry.target_physical == ad_id)
        .count();
    work_total += ambiguity_run.work.total();
    all_quiescent &= ambiguity_run.naturally_quiescent;

    Controls {
        too_late,
        correlation_only,
        no_return_reuse: no_return_observation.crossings,
        absent: absent_observation.crossings,
        blocked: blocked_observation.crossings,
        stale: stale_observation.crossings,
        ambiguity,
        ambiguity_wrong,
        quiescent: all_quiescent,
        work: work_total,
    }
}

fn run_row(variant: Variant) -> Row {
    let mut field = build_field(variant, 1, true);
    let initial_fingerprint = field.substrate.permanent_fingerprint();
    let mut work = WorkLedger::default();
    let (training_ab, training_cd, training_consumed, training_returns) =
        train_primary(&mut field, &mut work);
    let acquired_fingerprint = field.substrate.permanent_fingerprint();
    let gap_work = field.substrate.advance_time(50);
    work.absorb(&gap_work);
    let gap_fingerprint = field.substrate.complete_fingerprint();
    let permanent_fingerprint = field.substrate.permanent_fingerprint();

    let ab = field
        .substrate
        .arrows_between(field.sources[0], field.sites[0]);
    let ad = field
        .substrate
        .arrows_between(field.sources[0], field.sites[1]);
    let cd = field
        .substrate
        .arrows_between(field.sources[1], field.sites[2]);
    let cb = field
        .substrate
        .arrows_between(field.sources[1], field.sites[3]);

    let mut trained_ab_field = field.clone();
    let (duplicate_left, trained_ab) = observe(&mut trained_ab_field, Some(Duo::Ab), None);
    let mut duplicate_field = field.clone();
    let (duplicate_right, _) = observe(&mut duplicate_field, Some(Duo::Ab), None);
    let duplicate_equal = duplicate_left == duplicate_right
        && trained_ab_field.substrate.complete_fingerprint()
            == duplicate_field.substrate.complete_fingerprint();

    let mut trained_cd_field = field.clone();
    let (_, trained_cd) = observe(&mut trained_cd_field, Some(Duo::Cd), None);
    let mut crossed_ad_field = field.clone();
    let (_, crossed_ad) = observe(&mut crossed_ad_field, Some(Duo::Ad), None);
    let mut crossed_cb_field = field.clone();
    let (_, crossed_cb) = observe(&mut crossed_cb_field, Some(Duo::Cb), None);
    let mut singleton_a_field = field.clone();
    let (_, singleton_a) = observe(&mut singleton_a_field, None, Some(0));
    let mut singleton_c_field = field.clone();
    let (_, singleton_c) = observe(&mut singleton_c_field, None, Some(1));
    let mut singleton_b_field = field.clone();
    let singleton_b = observe_contributor_only(&mut singleton_b_field, 0);
    let mut singleton_d_field = field.clone();
    let singleton_d = observe_contributor_only(&mut singleton_d_field, 1);
    let mut self_field = field.clone();
    let (_, self_evidence) = observe(&mut self_field, None, Some(0));

    let controls = run_controls(variant);
    let all_observations = [
        &trained_ab,
        &trained_cd,
        &crossed_ad,
        &crossed_cb,
        &singleton_a,
        &singleton_b,
        &singleton_c,
        &singleton_d,
        &self_evidence,
    ];
    let observations_quiescent = all_observations.iter().all(|entry| entry.quiescent);
    let p0 = true;
    let p1 = training_ab == 8 && training_cd == 8;
    let p2 = training_consumed >= 16 && training_returns >= 16;
    let p3 = singleton_a.crossings == 0
        && singleton_b.crossings == 0
        && singleton_c.crossings == 0
        && singleton_d.crossings == 0
        && self_evidence.crossings == 0
        && self_evidence.consumed == 0;
    let p4 = trained_ab.crossings == 1
        && trained_cd.crossings == 1
        && trained_ab.output_firings == 1
        && trained_cd.output_firings == 1
        && trained_ab.source_firings == 1
        && trained_cd.source_firings == 1
        && trained_ab.emitted >= 4
        && trained_cd.emitted >= 4;
    let p5 = crossed_ad.crossings == 0
        && crossed_cb.crossings == 0
        && crossed_ad.output_firings == 0
        && crossed_cb.output_firings == 0;
    let p6 = controls.too_late == 0
        && controls.correlation_only == 0
        && controls.no_return_reuse == 0
        && controls.absent == 0
        && controls.blocked == 0
        && controls.stale == 0
        && controls.ambiguity == 1
        && controls.ambiguity_wrong == 0;
    let p7 = duplicate_equal;
    let p8 = observations_quiescent && controls.quiescent;
    let p9 = work.total() > 0 && controls.work > 0;
    let arrow_count = field.substrate.arrow_count();
    let persistent_bytes = field.substrate.persistent_bytes();
    Row {
        variant,
        initial_fingerprint,
        acquired_fingerprint,
        gap_fingerprint,
        permanent_fingerprint,
        training_ab,
        training_cd,
        training_consumed,
        training_returns,
        trained_ab,
        trained_cd,
        crossed_ad,
        crossed_cb,
        singleton_a,
        singleton_b,
        singleton_c,
        singleton_d,
        self_evidence,
        ab,
        cd,
        ad,
        cb,
        controls,
        duplicate_equal,
        arrow_count,
        persistent_bytes,
        work,
        clauses: [p0, p1, p2, p3, p4, p5, p6, p7, p8, p9],
    }
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "variant,namespace,initial_fp,acquired_fp,gap_fp,permanent_fp,training_ab,training_cd,training_consumed,training_returns,trained_ab_crossings,trained_ab_consumed,trained_ab_emitted,trained_ab_source_firings,trained_ab_output_firings,trained_cd_crossings,trained_cd_consumed,trained_cd_emitted,trained_cd_source_firings,trained_cd_output_firings,crossed_ad_crossings,crossed_ad_consumed,crossed_ad_output_firings,crossed_cb_crossings,crossed_cb_consumed,crossed_cb_output_firings,singleton_a,singleton_b,singleton_c,singleton_d,self_crossings,self_consumed,ab_records,ab_live,ab_resistance,ab_coupling,ab_generation,cd_records,cd_live,cd_resistance,cd_coupling,cd_generation,ad_records,ad_live,ad_resistance,ad_generation,cb_records,cb_live,cb_resistance,cb_generation,too_late,correlation_only,no_return_reuse,absent,blocked,stale,ambiguity,ambiguity_wrong,duplicate_equal,quiescent,arrow_count,persistent_bytes,main_work,control_work,P0,P1,P2,P3,P4,P5,P6,P7,P8,P9,row_pass\n",
    );
    for row in rows {
        let row_pass = row.clauses.iter().all(|value| *value);
        let mut values = vec![
            row.variant.name.to_string(),
            format!("{:#x}", row.variant.namespace),
            format!("{:#018x}", row.initial_fingerprint),
            format!("{:#018x}", row.acquired_fingerprint),
            format!("{:#018x}", row.gap_fingerprint),
            format!("{:#018x}", row.permanent_fingerprint),
            row.training_ab.to_string(),
            row.training_cd.to_string(),
            row.training_consumed.to_string(),
            row.training_returns.to_string(),
            row.trained_ab.crossings.to_string(),
            row.trained_ab.consumed.to_string(),
            row.trained_ab.emitted.to_string(),
            row.trained_ab.source_firings.to_string(),
            row.trained_ab.output_firings.to_string(),
            row.trained_cd.crossings.to_string(),
            row.trained_cd.consumed.to_string(),
            row.trained_cd.emitted.to_string(),
            row.trained_cd.source_firings.to_string(),
            row.trained_cd.output_firings.to_string(),
            row.crossed_ad.crossings.to_string(),
            row.crossed_ad.consumed.to_string(),
            row.crossed_ad.output_firings.to_string(),
            row.crossed_cb.crossings.to_string(),
            row.crossed_cb.consumed.to_string(),
            row.crossed_cb.output_firings.to_string(),
            row.singleton_a.crossings.to_string(),
            row.singleton_b.crossings.to_string(),
            row.singleton_c.crossings.to_string(),
            row.singleton_d.crossings.to_string(),
            row.self_evidence.crossings.to_string(),
            row.self_evidence.consumed.to_string(),
            row.ab.records.to_string(),
            row.ab.live.to_string(),
            row.ab.resistance_max.to_string(),
            row.ab.coupling_max.to_string(),
            row.ab.generation_max.to_string(),
            row.cd.records.to_string(),
            row.cd.live.to_string(),
            row.cd.resistance_max.to_string(),
            row.cd.coupling_max.to_string(),
            row.cd.generation_max.to_string(),
            row.ad.records.to_string(),
            row.ad.live.to_string(),
            row.ad.resistance_max.to_string(),
            row.ad.generation_max.to_string(),
            row.cb.records.to_string(),
            row.cb.live.to_string(),
            row.cb.resistance_max.to_string(),
            row.cb.generation_max.to_string(),
            row.controls.too_late.to_string(),
            row.controls.correlation_only.to_string(),
            row.controls.no_return_reuse.to_string(),
            row.controls.absent.to_string(),
            row.controls.blocked.to_string(),
            row.controls.stale.to_string(),
            row.controls.ambiguity.to_string(),
            row.controls.ambiguity_wrong.to_string(),
            row.duplicate_equal.to_string(),
            row.controls.quiescent.to_string(),
            row.arrow_count.to_string(),
            row.persistent_bytes.to_string(),
            row.work.total().to_string(),
            row.controls.work.to_string(),
        ];
        values.extend(row.clauses.iter().map(bool::to_string));
        values.push(row_pass.to_string());
        output.push_str(&values.join(","));
        output.push('\n');
    }
    output
}

fn report(rows: &[Row]) -> String {
    let passed = rows
        .iter()
        .filter(|row| row.clauses.iter().all(|bit| *bit))
        .count();
    let total_work: u64 = rows
        .iter()
        .map(|row| row.work.total() + row.controls.work)
        .sum();
    let outcome = if passed == rows.len() {
        "POSITIVE; MICRO ELIGIBLE BUT UNSPENT"
    } else {
        "FROZEN NEGATIVE; MICRO INELIGIBLE"
    };
    format!(
        "# CJ0 ARM CJ-B locally gated ARROW PROBE v1 result\n\nStatus: **{outcome}**.\n\n- conjunctive rows: `{passed}/{}`;\n- claims: `{}/{}`;\n- ledgered physical work: `{total_work}`;\n- authoritative PX0--PX2 source modified: `no`;\n- definitive/PX3/PX-C execution: `none`.\n\nThe candidate consumes only current decayed destination CELL state plus local ARROW coupling. A successful inspection clears that state, emits their numeric sum as an ordinary SPIKE, and exposes only the traversed ARROW to ordinary returned activity. No persistent field, contributor key, relation record, or logical operator is present.\n\nTrained held-out crossings by row are `{}`; crossed held-out crossings are `{}`; source/contributor singleton crossings are `{}`; self-evidence consumptions are `{}`.\n\nExact row data, stage fingerprints, structure, controls, clause bits, work, and storage are serialized in the companion CSV. This PROBE does not execute reversal, recursion, OR, GATE, definitive evidence, or authority.\n",
        rows.len(),
        rows.iter().map(|row| row.clauses.iter().filter(|bit| **bit).count()).sum::<usize>(),
        rows.len() * 10,
        rows.iter().map(|row| format!("{}|{}", row.trained_ab.crossings, row.trained_cd.crossings)).collect::<Vec<_>>().join(";"),
        rows.iter().map(|row| format!("{}|{}", row.crossed_ad.crossings, row.crossed_cb.crossings)).collect::<Vec<_>>().join(";"),
        rows.iter().map(|row| format!("{}|{}|{}|{}", row.singleton_a.crossings, row.singleton_b.crossings, row.singleton_c.crossings, row.singleton_d.crossings)).collect::<Vec<_>>().join(";"),
        rows.iter().map(|row| row.self_evidence.consumed.to_string()).collect::<Vec<_>>().join("|")
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
            return Err(format!(
                "refusing to overwrite existing evidence path {path}"
            ));
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

fn execute_probe() -> Result<bool, String> {
    let variants = [
        Variant {
            name: "normal",
            namespace: 0x9_b100_0000,
            mirror: false,
            reverse_cells: false,
            reverse_arrows: false,
            reverse_insertion: false,
        },
        Variant {
            name: "mirror",
            namespace: 0x9_b200_0000,
            mirror: true,
            reverse_cells: false,
            reverse_arrows: false,
            reverse_insertion: true,
        },
        Variant {
            name: "reverse_allocation",
            namespace: 0x9_b300_0000,
            mirror: false,
            reverse_cells: true,
            reverse_arrows: true,
            reverse_insertion: false,
        },
        Variant {
            name: "permuted_orientation",
            namespace: 0x9_b400_0000,
            mirror: true,
            reverse_cells: true,
            reverse_arrows: true,
            reverse_insertion: true,
        },
    ];
    println!("CJ0_B_LOCALLY_GATED_ARROW_PROBE_V1_EVIDENCE_SPENT");
    let rows = variants.into_iter().map(run_row).collect::<Vec<_>>();
    let passed = rows.iter().all(|row| row.clauses.iter().all(|bit| *bit));
    publish(&rows)?;
    Ok(passed)
}

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments == ["--preflight"] {
        println!(
            "{{\"arm\":\"CJ-B\",\"stage\":\"PROBE\",\"cells_entered\":0,\"artifacts_written\":0}}"
        );
        return ExitCode::SUCCESS;
    }
    if arguments != ["--probe"] {
        eprintln!("refusing execution: expected exactly --preflight or --probe");
        return ExitCode::from(2);
    }
    match execute_probe() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
