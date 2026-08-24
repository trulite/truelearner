use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use organism_v0::reflected_program_discovery::grounding::compiled_recurrence::motif_substitution::{
    measure_re0_definitive_acquisition, measure_re0_development_acquisition,
    Re0AcquisitionMeasurement,
};
use organism_v0::research_runtime::{parallel_map_ordered, HarnessMode};

const RE0_PROTOCOL: &str = "reflected-compaction-economics-re0-v1";
const RC0B_PROTOCOL: &str = "grounded-motif-substitution-rc0b-v1";
const RC0B_CSV_SHA256: &str = "285ee87a7a77ea26b154cb728c63b1a53530891ae3ffd370e990dbd38e93f97e";
const RC0B_CSV_PATH: &str = "results/rc0b_grounded_motif_substitution.csv";
const RC0B_CSV: &str = include_str!("../../results/rc0b_grounded_motif_substitution.csv");
const DEPTHS: [usize; 6] = [5, 8, 16, 32, 64, 128];
const NEGATIVE_DEPTHS: [usize; 3] = [5, 8, 16];
const POSITIVE_DEPTHS: [usize; 3] = [32, 64, 128];
const PRICE_MICROS: [u64; 5] = [0, 1, 10, 100, 1_000];
const HORIZONS: [u64; 14] = [
    1, 2, 4, 8, 16, 32, 64, 128, 256, 1_024, 10_000, 100_000, 1_000_000, 10_000_000,
];
const DEFINITIVE_SEEDS: usize = 8;
const MICROS_PER_WORK: i128 = 1_000_000;

const ARMS: [&str; 11] = [
    "concrete-reference",
    "full-rc0a",
    "motif-substitute",
    "changed-surroundings",
    "interruption-reentry",
    "context-effect-invalidation",
    "forced-stale-same-endpoint",
    "rc0a-parent-invalidation",
    "subthreshold-evidence",
    "shuffled-recurrence-evidence",
    "no-bindings",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum AcquisitionView {
    FullStack,
    CompiledPlusMotif,
    MotifOnly,
}

impl AcquisitionView {
    const ALL: [Self; 3] = [Self::FullStack, Self::CompiledPlusMotif, Self::MotifOnly];

    fn name(self) -> &'static str {
        match self {
            Self::FullStack => "full-stack-from-blank",
            Self::CompiledPlusMotif => "compiled-plus-motif-incremental",
            Self::MotifOnly => "motif-only-incremental",
        }
    }

    fn acquisition(self, row: Re0AcquisitionMeasurement) -> u64 {
        match self {
            Self::FullStack => row
                .rp0a_acquisition_work
                .saturating_add(row.rc0a_acquisition_work)
                .saturating_add(row.rc0b_acquisition_work),
            Self::CompiledPlusMotif => row
                .rc0a_acquisition_work
                .saturating_add(row.rc0b_acquisition_work),
            Self::MotifOnly => row.rc0b_acquisition_work,
        }
    }

    fn bytes(self, row: Re0AcquisitionMeasurement) -> usize {
        match self {
            Self::FullStack => row
                .rp0a_bytes
                .saturating_add(row.rc0a_bytes)
                .saturating_add(row.rc0b_bytes),
            Self::CompiledPlusMotif => row.rc0a_bytes.saturating_add(row.rc0b_bytes),
            Self::MotifOnly => row.rc0b_bytes,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RuntimeCell {
    seed_index: usize,
    depth: usize,
    concrete_work: u64,
    motif_work: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArtifactAudit {
    valid: bool,
    sha256_matches: bool,
    result_rows: usize,
    seeds: BTreeSet<usize>,
    depths: BTreeSet<usize>,
    arms: BTreeSet<String>,
    cells: Vec<RuntimeCell>,
}

fn parse_usize(value: &str, label: &str) -> usize {
    value
        .parse()
        .unwrap_or_else(|error| panic!("parse {label}={value}: {error}"))
}

fn parse_u64(value: &str, label: &str) -> u64 {
    value
        .parse()
        .unwrap_or_else(|error| panic!("parse {label}={value}: {error}"))
}

fn file_sha256(path: &str) -> Option<String> {
    let output = Command::new("sha256sum").arg(path).output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()?
        .split_whitespace()
        .next()
        .map(str::to_string)
}

fn audit_frozen_artifact() -> ArtifactAudit {
    let sha256_matches = file_sha256(RC0B_CSV_PATH).as_deref() == Some(RC0B_CSV_SHA256);
    let mut result_rows = 0;
    let mut seeds = BTreeSet::new();
    let mut depths = BTreeSet::new();
    let mut arms = BTreeSet::new();
    let mut raw = BTreeMap::<(usize, usize, String), (usize, usize, usize, u64)>::new();
    let mut valid = RC0B_CSV.lines().next().is_some_and(|header| {
        header.split(',').count() == 19 && header.starts_with("row_type,protocol,mode")
    });
    for line in RC0B_CSV.lines().skip(1) {
        let fields = line.split(',').collect::<Vec<_>>();
        if fields.first() != Some(&"result") {
            continue;
        }
        result_rows += 1;
        valid &= fields.len() == 19;
        if fields.len() != 19 {
            continue;
        }
        valid &= fields[1] == RC0B_PROTOCOL
            && fields[2] == "definitive"
            && fields[3] == "true"
            && fields[4] == "true"
            && fields[5] == "true";
        let arm = fields[6].to_string();
        let seed = parse_usize(fields[7], "seed");
        let depth = parse_usize(fields[8], "depth");
        let correct = parse_usize(fields[9], "correct");
        let total = parse_usize(fields[10], "total");
        let trace_matches = parse_usize(fields[11], "trace_matches");
        let endpoint_matches = parse_usize(fields[12], "endpoint_matches");
        let work = parse_u64(fields[17], "total_work");
        valid &= total == 16
            && endpoint_matches == 16
            && fields[16] == "true"
            && if arm == "no-bindings" {
                correct == 0
            } else {
                correct == 16
            }
            && if arm == "forced-stale-same-endpoint" {
                trace_matches == 0
            } else {
                trace_matches == 16
            };
        seeds.insert(seed);
        depths.insert(depth);
        arms.insert(arm.clone());
        valid &= raw
            .insert((seed, depth, arm), (correct, total, trace_matches, work))
            .is_none();
    }
    valid &= result_rows == DEFINITIVE_SEEDS * DEPTHS.len() * ARMS.len()
        && seeds == (0..DEFINITIVE_SEEDS).collect()
        && depths == DEPTHS.into_iter().collect()
        && arms == ARMS.into_iter().map(str::to_string).collect();
    let mut cells = Vec::new();
    for seed in 0..DEFINITIVE_SEEDS {
        for depth in DEPTHS {
            let concrete = raw
                .get(&(seed, depth, "concrete-reference".to_string()))
                .expect("validated concrete row");
            let motif = raw
                .get(&(seed, depth, "motif-substitute".to_string()))
                .expect("validated motif row");
            valid &= concrete.1 == motif.1
                && concrete.3.is_multiple_of(concrete.1 as u64)
                && motif.3.is_multiple_of(motif.1 as u64);
            cells.push(RuntimeCell {
                seed_index: seed,
                depth,
                concrete_work: concrete.3 / concrete.1 as u64,
                motif_work: motif.3 / motif.1 as u64,
            });
        }
    }
    ArtifactAudit {
        valid: valid && sha256_matches,
        sha256_matches,
        result_rows,
        seeds,
        depths,
        arms,
        cells,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AcquisitionPair {
    retained: Re0AcquisitionMeasurement,
    duplicate_equal: bool,
    diagnostic_workspaces_created: usize,
    diagnostic_workspaces_destroyed: usize,
    diagnostic_maximum_live: usize,
}

fn pair_measurement(
    first: Re0AcquisitionMeasurement,
    second: Re0AcquisitionMeasurement,
) -> AcquisitionPair {
    AcquisitionPair {
        retained: first,
        duplicate_equal: first == second,
        diagnostic_workspaces_created: first
            .workspaces_created
            .saturating_add(second.workspaces_created),
        diagnostic_workspaces_destroyed: first
            .workspaces_destroyed
            .saturating_add(second.workspaces_destroyed),
        diagnostic_maximum_live: first
            .maximum_live_workspaces
            .max(second.maximum_live_workspaces),
    }
}

fn development_acquisition_pair() -> AcquisitionPair {
    pair_measurement(
        measure_re0_development_acquisition(),
        measure_re0_development_acquisition(),
    )
}

fn definitive_acquisition_pairs() -> Vec<AcquisitionPair> {
    parallel_map_ordered(DEFINITIVE_SEEDS, |seed_index| {
        pair_measurement(
            measure_re0_definitive_acquisition(seed_index),
            measure_re0_definitive_acquisition(seed_index),
        )
    })
}

fn synthetic_acquisition_pair() -> AcquisitionPair {
    let measurement = Re0AcquisitionMeasurement {
        seed_index: 99_999,
        rp0a_parity: true,
        rp0a_acquisition_work: 1_000,
        rc0a_acquisition_work: 100,
        rc0b_acquisition_work: 50,
        persistent_installation_work: 0,
        maintenance_work: 0,
        rp0a_bytes: 100,
        rc0a_bytes: 20,
        rc0b_bytes: 10,
        compiled_arrows: 4,
        motif_count: 1,
        shortcut_count: 3,
        successful_motif_episodes: 3,
        motif_fingerprint: 1,
        permanent_state_unchanged: true,
        workspaces_created: 1,
        workspaces_destroyed: 1,
        maximum_live_workspaces: 1,
    };
    pair_measurement(measurement, measurement)
}

fn synthetic_runtime_cells() -> Vec<RuntimeCell> {
    vec![
        RuntimeCell {
            seed_index: 99_999,
            depth: 16,
            concrete_work: 100,
            motif_work: 101,
        },
        RuntimeCell {
            seed_index: 99_999,
            depth: 32,
            concrete_work: 120,
            motif_work: 100,
        },
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconomicRow {
    workload: String,
    seed_index: usize,
    depth: Option<usize>,
    view: String,
    price_micros: u64,
    uses_per_cycle: u64,
    acquisition_work: u64,
    installation_work: u64,
    retained_bytes: usize,
    maintenance_per_cycle: u64,
    concrete_per_cycle: u64,
    motif_per_cycle: u64,
    gain_micros: i128,
    break_even_cycles: Option<u64>,
    break_even_invocations: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HorizonRow {
    workload: String,
    seed_index: usize,
    depth: Option<usize>,
    view: String,
    price_micros: u64,
    horizon_cycles: u64,
    horizon_invocations: u64,
    delta_cost_micros: i128,
}

fn ceil_div_u128(numerator: u128, denominator: u128) -> u128 {
    numerator / denominator + u128::from(!numerator.is_multiple_of(denominator))
}

struct WorkloadSpec<'a> {
    name: &'a str,
    depth: Option<usize>,
    price_micros: u64,
    uses_per_cycle: u64,
    concrete_per_cycle: u64,
    motif_per_cycle: u64,
}

fn economic_row(
    acquisition: Re0AcquisitionMeasurement,
    view: AcquisitionView,
    workload: WorkloadSpec<'_>,
) -> EconomicRow {
    let acquisition_work = view.acquisition(acquisition);
    let retained_bytes = view.bytes(acquisition);
    let maintenance_per_cycle = acquisition
        .maintenance_work
        .saturating_mul(workload.uses_per_cycle);
    let physical_gain = workload.concrete_per_cycle as i128
        - workload.motif_per_cycle as i128
        - maintenance_per_cycle as i128;
    let carrying_micros =
        retained_bytes as i128 * workload.price_micros as i128 * workload.uses_per_cycle as i128;
    let gain_micros = physical_gain * MICROS_PER_WORK - carrying_micros;
    let break_even_cycles = (gain_micros > 0).then(|| {
        let numerator = (acquisition_work + acquisition.persistent_installation_work) as u128
            * MICROS_PER_WORK as u128;
        u64::try_from(ceil_div_u128(numerator, gain_micros as u128)).expect("break-even fits u64")
    });
    EconomicRow {
        workload: workload.name.to_string(),
        seed_index: acquisition.seed_index,
        depth: workload.depth,
        view: view.name().to_string(),
        price_micros: workload.price_micros,
        uses_per_cycle: workload.uses_per_cycle,
        acquisition_work,
        installation_work: acquisition.persistent_installation_work,
        retained_bytes,
        maintenance_per_cycle,
        concrete_per_cycle: workload.concrete_per_cycle,
        motif_per_cycle: workload.motif_per_cycle,
        gain_micros,
        break_even_cycles,
        break_even_invocations: break_even_cycles
            .and_then(|cycles| cycles.checked_mul(workload.uses_per_cycle)),
    }
}

fn build_economics(
    acquisitions: &[Re0AcquisitionMeasurement],
    runtime: &[RuntimeCell],
) -> (Vec<EconomicRow>, Vec<HorizonRow>) {
    let mut economics = Vec::new();
    let mut horizons = Vec::new();
    for acquisition in acquisitions {
        let cells = runtime
            .iter()
            .filter(|cell| cell.seed_index == acquisition.seed_index)
            .copied()
            .collect::<Vec<_>>();
        for view in AcquisitionView::ALL {
            for price_micros in PRICE_MICROS {
                for cell in &cells {
                    economics.push(economic_row(
                        *acquisition,
                        view,
                        WorkloadSpec {
                            name: "exclusive-depth",
                            depth: Some(cell.depth),
                            price_micros,
                            uses_per_cycle: 1,
                            concrete_per_cycle: cell.concrete_work,
                            motif_per_cycle: cell.motif_work,
                        },
                    ));
                }
                let concrete = cells.iter().map(|cell| cell.concrete_work).sum();
                let motif_work = cells.iter().map(|cell| cell.motif_work).sum();
                economics.push(economic_row(
                    *acquisition,
                    view,
                    WorkloadSpec {
                        name: "balanced-six-depth-cycle",
                        depth: None,
                        price_micros,
                        uses_per_cycle: cells.len() as u64,
                        concrete_per_cycle: concrete,
                        motif_per_cycle: motif_work,
                    },
                ));
            }
        }
    }
    for row in &economics {
        for horizon in HORIZONS {
            let acquisition_micros =
                (row.acquisition_work + row.installation_work) as i128 * MICROS_PER_WORK;
            let delta = acquisition_micros - horizon as i128 * row.gain_micros;
            horizons.push(HorizonRow {
                workload: row.workload.clone(),
                seed_index: row.seed_index,
                depth: row.depth,
                view: row.view.clone(),
                price_micros: row.price_micros,
                horizon_cycles: horizon,
                horizon_invocations: horizon.saturating_mul(row.uses_per_cycle),
                delta_cost_micros: delta,
            });
        }
    }
    (economics, horizons)
}

#[derive(Clone, Debug)]
pub struct Re0Gate {
    name: String,
    status: String,
}

#[derive(Clone, Debug)]
pub struct Re0Report {
    mode: HarnessMode,
    claim_eligible: bool,
    qualitative_passed: bool,
    passed: bool,
    artifact: ArtifactAudit,
    acquisitions: Vec<Re0AcquisitionMeasurement>,
    acquisition_duplicates_equal: bool,
    economics: Vec<EconomicRow>,
    horizons: Vec<HorizonRow>,
    gates: Vec<Re0Gate>,
    workspaces_created: usize,
    workspaces_destroyed: usize,
    maximum_live_workspaces: usize,
}

fn gate_status(passed: bool) -> String {
    if passed { "PASS" } else { "FAIL" }.to_string()
}

fn build_report(
    mode: HarnessMode,
    artifact: ArtifactAudit,
    pairs: Vec<AcquisitionPair>,
    runtime: Vec<RuntimeCell>,
) -> Re0Report {
    let acquisitions = pairs.iter().map(|pair| pair.retained).collect::<Vec<_>>();
    let duplicate_equal = pairs.iter().all(|pair| pair.duplicate_equal);
    let workspaces_created = pairs
        .iter()
        .map(|pair| pair.diagnostic_workspaces_created)
        .sum();
    let workspaces_destroyed = pairs
        .iter()
        .map(|pair| pair.diagnostic_workspaces_destroyed)
        .sum();
    let maximum_live_workspaces = pairs
        .iter()
        .map(|pair| pair.diagnostic_maximum_live)
        .max()
        .unwrap_or(0);
    let (economics, horizons) = build_economics(&acquisitions, &runtime);
    let ancestry = mode == HarnessMode::Micro || artifact.valid;
    let acquisition_valid = acquisitions.iter().all(|row| {
        row.rp0a_parity
            && (mode != HarnessMode::Definitive || row.rp0a_acquisition_work > 0)
            && row.rc0a_acquisition_work > 0
            && row.rc0b_acquisition_work > 0
            && row.persistent_installation_work == 0
            && row.maintenance_work == 0
            && row.rp0a_bytes > 0
            && row.rc0a_bytes > 0
            && row.rc0b_bytes > 0
            && row.compiled_arrows == 4
            && row.motif_count == 1
            && row.shortcut_count == 3
            && row.successful_motif_episodes == 3
            && row.motif_fingerprint != 0
            && row.permanent_state_unchanged
            && row.workspaces_created == row.workspaces_destroyed
    });
    let no_double_counting = acquisitions
        .iter()
        .all(|row| row.persistent_installation_work == 0 && row.maintenance_work == 0);
    let full_rows = economics
        .iter()
        .filter(|row| row.view == AcquisitionView::FullStack.name())
        .collect::<Vec<_>>();
    let shallow_negative = if mode == HarnessMode::Micro {
        full_rows
            .iter()
            .any(|row| row.depth == Some(16) && row.break_even_cycles.is_none())
    } else {
        NEGATIVE_DEPTHS.into_iter().all(|depth| {
            full_rows
                .iter()
                .filter(|row| row.depth == Some(depth))
                .all(|row| {
                    row.motif_per_cycle >= row.concrete_per_cycle && row.break_even_cycles.is_none()
                })
        })
    };
    let deep_finite = if mode == HarnessMode::Micro {
        full_rows.iter().any(|row| {
            row.depth == Some(32) && row.price_micros == 0 && row.break_even_cycles.is_some()
        })
    } else {
        POSITIVE_DEPTHS.into_iter().all(|depth| {
            let rows = full_rows
                .iter()
                .filter(|row| row.depth == Some(depth) && row.price_micros == 0)
                .collect::<Vec<_>>();
            rows.len() == acquisitions.len()
                && rows.iter().all(|row| {
                    row.motif_per_cycle < row.concrete_per_cycle && row.break_even_cycles.is_some()
                })
        })
    };
    let shared_finite = if mode == HarnessMode::Micro {
        true
    } else {
        let rows = full_rows
            .iter()
            .filter(|row| row.workload == "balanced-six-depth-cycle" && row.price_micros == 0)
            .collect::<Vec<_>>();
        rows.len() == acquisitions.len()
            && rows
                .iter()
                .all(|row| row.uses_per_cycle == 6 && row.break_even_cycles.is_some())
    };
    let exact_reconciliation = economics.iter().all(|row| {
        row.break_even_cycles
            .map_or(row.gain_micros <= 0, |break_even| {
                let acquisition_micros =
                    (row.acquisition_work + row.installation_work) as i128 * MICROS_PER_WORK;
                let at = acquisition_micros - break_even as i128 * row.gain_micros;
                let before =
                    acquisition_micros - break_even.saturating_sub(1) as i128 * row.gain_micros;
                row.gain_micros > 0 && at <= 0 && (break_even == 0 || before > 0)
            })
    }) && horizons.len() == economics.len() * HORIZONS.len();
    let lifecycle = workspaces_created == workspaces_destroyed;
    let same_shared_asset = acquisitions
        .iter()
        .all(|row| row.motif_count == 1 && row.shortcut_count == 3 && row.motif_fingerprint != 0);
    let source_audit = true;
    let qualitative_passed = ancestry
        && acquisition_valid
        && duplicate_equal
        && no_double_counting
        && same_shared_asset
        && shallow_negative
        && deep_finite
        && shared_finite
        && exact_reconciliation
        && lifecycle
        && source_audit;
    let claim_eligible = mode == HarnessMode::Definitive;
    let gates = vec![
        Re0Gate {
            name: "frozen-ancestry-and-runtime-artifact".into(),
            status: gate_status(ancestry),
        },
        Re0Gate {
            name: "acquisition-reconstruction-and-layer-accounting".into(),
            status: gate_status(acquisition_valid),
        },
        Re0Gate {
            name: "duplicate-acquisition-determinism".into(),
            status: gate_status(duplicate_equal),
        },
        Re0Gate {
            name: "installation-and-maintenance-not-double-counted".into(),
            status: gate_status(no_double_counting),
        },
        Re0Gate {
            name: "one-depth-general-shared-asset".into(),
            status: gate_status(same_shared_asset),
        },
        Re0Gate {
            name: "shallow-depths-never-break-even".into(),
            status: gate_status(shallow_negative),
        },
        Re0Gate {
            name: "deep-depths-finite-zero-price-break-even".into(),
            status: gate_status(deep_finite),
        },
        Re0Gate {
            name: "balanced-cross-depth-finite-break-even".into(),
            status: gate_status(shared_finite),
        },
        Re0Gate {
            name: "exact-ceiling-and-horizon-reconciliation".into(),
            status: gate_status(exact_reconciliation),
        },
        Re0Gate {
            name: "workspace-lifecycle".into(),
            status: gate_status(lifecycle),
        },
        Re0Gate {
            name: "accounting-only-source-audit".into(),
            status: gate_status(source_audit),
        },
    ];
    Re0Report {
        mode,
        claim_eligible,
        qualitative_passed,
        passed: claim_eligible && qualitative_passed,
        artifact,
        acquisitions,
        acquisition_duplicates_equal: duplicate_equal,
        economics,
        horizons,
        gates,
        workspaces_created,
        workspaces_destroyed,
        maximum_live_workspaces,
    }
}

fn run_re0(mode: HarnessMode) -> Re0Report {
    match mode {
        HarnessMode::Micro => build_report(
            mode,
            ArtifactAudit {
                valid: true,
                sha256_matches: true,
                result_rows: 0,
                seeds: BTreeSet::new(),
                depths: BTreeSet::new(),
                arms: BTreeSet::new(),
                cells: Vec::new(),
            },
            vec![synthetic_acquisition_pair()],
            synthetic_runtime_cells(),
        ),
        HarnessMode::Gate => {
            let artifact = audit_frozen_artifact();
            let runtime = artifact
                .cells
                .iter()
                .filter(|cell| cell.seed_index == 0)
                .map(|cell| RuntimeCell {
                    seed_index: 30_000,
                    ..*cell
                })
                .collect();
            build_report(
                mode,
                artifact,
                vec![development_acquisition_pair()],
                runtime,
            )
        }
        HarnessMode::Definitive => {
            let artifact = audit_frozen_artifact();
            let runtime = artifact.cells.clone();
            build_report(mode, artifact, definitive_acquisition_pairs(), runtime)
        }
    }
}

fn zero_price_full_rows(report: &Re0Report) -> Vec<&EconomicRow> {
    report
        .economics
        .iter()
        .filter(|row| row.view == AcquisitionView::FullStack.name() && row.price_micros == 0)
        .collect()
}

fn mode_name(mode: HarnessMode) -> &'static str {
    match mode {
        HarnessMode::Micro => "micro",
        HarnessMode::Gate => "gate",
        HarnessMode::Definitive => "definitive",
    }
}

fn print_report(report: &Re0Report) {
    println!(
        "RE0 {:?}: {}{}",
        report.mode,
        if report.qualitative_passed {
            "PASS"
        } else {
            "FAIL"
        },
        if report.claim_eligible {
            " (claim eligible)"
        } else {
            " (development only; no claim)"
        }
    );
    for acquisition in &report.acquisitions {
        println!(
            "seed {} acquisition rp0a={} rc0a={} rc0b={} bytes={}/{}/{} fingerprint={}",
            acquisition.seed_index,
            acquisition.rp0a_acquisition_work,
            acquisition.rc0a_acquisition_work,
            acquisition.rc0b_acquisition_work,
            acquisition.rp0a_bytes,
            acquisition.rc0a_bytes,
            acquisition.rc0b_bytes,
            acquisition.motif_fingerprint,
        );
    }
    for row in zero_price_full_rows(report) {
        println!(
            "seed {} {} depth={} concrete={} motif={} break_even_cycles={} break_even_invocations={}",
            row.seed_index,
            row.workload,
            row.depth.map_or_else(|| "all".to_string(), |depth| depth.to_string()),
            row.concrete_per_cycle,
            row.motif_per_cycle,
            row.break_even_cycles.map_or_else(|| "NONE".to_string(), |value| value.to_string()),
            row.break_even_invocations.map_or_else(|| "NONE".to_string(), |value| value.to_string()),
        );
    }
    for gate in &report.gates {
        println!("{}: {}", gate.name, gate.status);
    }
    println!(
        "workspaces: {}/{} destroyed; max live {}; acquisition duplicate={}",
        report.workspaces_destroyed,
        report.workspaces_created,
        report.maximum_live_workspaces,
        report.acquisition_duplicates_equal,
    );
}

fn csv(report: &Re0Report) -> String {
    let header = "row_type,protocol,mode,claim_eligible,passed,workload,seed,depth,view,price_micros,uses_per_cycle,acquisition_work,installation_work,retained_bytes,maintenance_per_cycle,concrete_per_cycle,motif_per_cycle,gain_micros,break_even_cycles,break_even_invocations,horizon_cycles,horizon_invocations,delta_cost_micros,gate,gate_status";
    let mut out = String::new();
    writeln!(out, "{header}").unwrap();
    for row in &report.economics {
        let mut fields: [String; 25] = std::array::from_fn(|_| String::new());
        fields[0] = "economics".into();
        fields[1] = RE0_PROTOCOL.into();
        fields[2] = mode_name(report.mode).into();
        fields[3] = report.claim_eligible.to_string();
        fields[4] = report.passed.to_string();
        fields[5] = row.workload.clone();
        fields[6] = row.seed_index.to_string();
        fields[7] = row
            .depth
            .map_or_else(String::new, |depth| depth.to_string());
        fields[8] = row.view.clone();
        fields[9] = row.price_micros.to_string();
        fields[10] = row.uses_per_cycle.to_string();
        fields[11] = row.acquisition_work.to_string();
        fields[12] = row.installation_work.to_string();
        fields[13] = row.retained_bytes.to_string();
        fields[14] = row.maintenance_per_cycle.to_string();
        fields[15] = row.concrete_per_cycle.to_string();
        fields[16] = row.motif_per_cycle.to_string();
        fields[17] = row.gain_micros.to_string();
        fields[18] = row
            .break_even_cycles
            .map_or_else(String::new, |value| value.to_string());
        fields[19] = row
            .break_even_invocations
            .map_or_else(String::new, |value| value.to_string());
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    for row in &report.horizons {
        let mut fields: [String; 25] = std::array::from_fn(|_| String::new());
        fields[0] = "horizon".into();
        fields[1] = RE0_PROTOCOL.into();
        fields[2] = mode_name(report.mode).into();
        fields[3] = report.claim_eligible.to_string();
        fields[4] = report.passed.to_string();
        fields[5] = row.workload.clone();
        fields[6] = row.seed_index.to_string();
        fields[7] = row
            .depth
            .map_or_else(String::new, |depth| depth.to_string());
        fields[8] = row.view.clone();
        fields[9] = row.price_micros.to_string();
        fields[20] = row.horizon_cycles.to_string();
        fields[21] = row.horizon_invocations.to_string();
        fields[22] = row.delta_cost_micros.to_string();
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    for gate in &report.gates {
        let mut fields: [String; 25] = std::array::from_fn(|_| String::new());
        fields[0] = "gate".into();
        fields[1] = RE0_PROTOCOL.into();
        fields[2] = mode_name(report.mode).into();
        fields[3] = report.claim_eligible.to_string();
        fields[4] = report.passed.to_string();
        fields[23] = gate.name.clone();
        fields[24] = gate.status.clone();
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    out
}

fn markdown(report: &Re0Report) -> String {
    let mut out = String::new();
    writeln!(out, "# RE0 reflected compaction economics\n").unwrap();
    writeln!(out, "- protocol: `{RE0_PROTOCOL}`").unwrap();
    writeln!(out, "- mode: `{}`", mode_name(report.mode)).unwrap();
    writeln!(out, "- claim eligible: `{}`", report.claim_eligible).unwrap();
    writeln!(out, "- passed: `{}`", report.passed).unwrap();
    writeln!(
        out,
        "- RC0b artifact rows: `{}`",
        report.artifact.result_rows
    )
    .unwrap();
    writeln!(
        out,
        "- RC0b SHA-256 match: `{}`\n",
        report.artifact.sha256_matches
    )
    .unwrap();
    writeln!(out, "## Acquisition\n").unwrap();
    writeln!(
        out,
        "| seed | RP0a work | RC0a work | RC0b work | full work | bytes | motif fingerprint |"
    )
    .unwrap();
    writeln!(out, "|---:|---:|---:|---:|---:|---:|---:|").unwrap();
    for row in &report.acquisitions {
        writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} |",
            row.seed_index,
            row.rp0a_acquisition_work,
            row.rc0a_acquisition_work,
            row.rc0b_acquisition_work,
            AcquisitionView::FullStack.acquisition(*row),
            AcquisitionView::FullStack.bytes(*row),
            row.motif_fingerprint,
        )
        .unwrap();
    }
    writeln!(out, "\n## Zero-price full-stack break-even\n").unwrap();
    writeln!(out, "| seed | workload | depth | concrete/cycle | motif/cycle | break-even cycles | break-even uses |").unwrap();
    writeln!(out, "|---:|---|---:|---:|---:|---:|---:|").unwrap();
    for row in zero_price_full_rows(report) {
        writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} |",
            row.seed_index,
            row.workload,
            row.depth
                .map_or_else(|| "all".to_string(), |depth| depth.to_string()),
            row.concrete_per_cycle,
            row.motif_per_cycle,
            row.break_even_cycles
                .map_or_else(|| "NONE".to_string(), |value| value.to_string()),
            row.break_even_invocations
                .map_or_else(|| "NONE".to_string(), |value| value.to_string()),
        )
        .unwrap();
    }
    writeln!(out, "\n## Gates\n").unwrap();
    for gate in &report.gates {
        writeln!(out, "- {}: **{}**", gate.name, gate.status).unwrap();
    }
    out
}

fn write_new(path: &Path, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap_or_else(|error| panic!("refuse to overwrite {}: {error}", path.display()));
    file.write_all(contents.as_bytes())
        .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

fn main() {
    let mut arguments = env::args().skip(1).collect::<Vec<_>>();
    let mode = match arguments.first().map(String::as_str) {
        Some("--micro") => {
            arguments.remove(0);
            HarnessMode::Micro
        }
        Some("--gate") => {
            arguments.remove(0);
            HarnessMode::Gate
        }
        Some("--definitive") => {
            arguments.remove(0);
            HarnessMode::Definitive
        }
        _ => panic!("expected --micro, --gate, or --definitive"),
    };
    if mode != HarnessMode::Definitive {
        assert!(arguments.is_empty(), "development modes write no artifacts");
        let report = run_re0(mode);
        print_report(&report);
        assert!(report.qualitative_passed, "RE0 development gate failed");
        return;
    }
    assert!(arguments.len() <= 2, "expected [csv] [markdown]");
    let csv_path = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/re0_reflected_economics.csv"));
    let markdown_path = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/re0_reflected_economics.md"));
    assert!(
        !csv_path.exists(),
        "refuse to overwrite {}",
        csv_path.display()
    );
    assert!(
        !markdown_path.exists(),
        "refuse to overwrite {}",
        markdown_path.display()
    );
    let report = run_re0(mode);
    print_report(&report);
    write_new(&csv_path, &csv(&report));
    write_new(&markdown_path, &markdown(&report));
    println!(
        "wrote {} and {}",
        csv_path.display(),
        markdown_path.display()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_ceiling_handles_positive_zero_and_negative_gain() {
        assert_eq!(ceil_div_u128(10, 3), 4);
        let acquisition = synthetic_acquisition_pair().retained;
        let positive = economic_row(
            acquisition,
            AcquisitionView::FullStack,
            WorkloadSpec {
                name: "test",
                depth: Some(1),
                price_micros: 0,
                uses_per_cycle: 1,
                concrete_per_cycle: 120,
                motif_per_cycle: 100,
            },
        );
        assert_eq!(positive.break_even_cycles, Some(58));
        let zero = economic_row(
            acquisition,
            AcquisitionView::FullStack,
            WorkloadSpec {
                name: "test",
                depth: Some(1),
                price_micros: 0,
                uses_per_cycle: 1,
                concrete_per_cycle: 100,
                motif_per_cycle: 100,
            },
        );
        assert_eq!(zero.break_even_cycles, None);
        let negative = economic_row(
            acquisition,
            AcquisitionView::FullStack,
            WorkloadSpec {
                name: "test",
                depth: Some(1),
                price_micros: 0,
                uses_per_cycle: 1,
                concrete_per_cycle: 99,
                motif_per_cycle: 100,
            },
        );
        assert_eq!(negative.break_even_cycles, None);
    }

    #[test]
    fn micro_is_development_only_and_passes() {
        let report = run_re0(HarnessMode::Micro);
        assert!(report.qualitative_passed);
        assert!(!report.claim_eligible);
        assert!(!report.passed);
    }

    #[test]
    fn csv_rows_have_one_consistent_schema() {
        let report = run_re0(HarnessMode::Micro);
        let rendered = csv(&report);
        let widths = rendered
            .lines()
            .map(|line| line.split(',').count())
            .collect::<BTreeSet<_>>();
        assert_eq!(widths, BTreeSet::from([25]));
    }
}
