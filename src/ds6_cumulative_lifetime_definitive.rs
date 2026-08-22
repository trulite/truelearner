//! Write-once authority wrapper for the frozen cumulative DS6 lifetime mechanism.

pub const PROTOCOL: &str = "ds6-cumulative-lifetime-definitive-v1";
pub const PROTOCOL_COMMIT: &str = "b1b2a9252ee4211ffc0cf3f9789040b0f24ced7e";
pub const AUTHORITATIVE_M3: &str = "ffcdfe8b36fc62348b7ebcb09aaf4797f6146ba8";
pub const DEVELOPMENT_READINESS: &str = "d0062a1b703fd1a0e7aa2178aa609473bd6a1225";
pub const FROZEN_DEVELOPMENT_SHA256: &str =
    "3d5659fb26ae804dee6122408f9d703ea1f226349772883075a42686ac3fd110";
pub const FROZEN_HANDOFF_SHA256: &str =
    "9912b1225a9fc302103a931bf1ceb6e8f1eec78af70f7dcae2a96e93c2d6285a";
pub const FROZEN_M3_HANDOFF_SHA256: &str =
    "b180589c8f70330378ace19a6ec7cfed8b505cb86fdf1480f56c330e421b05ec";
pub const FROZEN_M3_CSV_SHA256: &str =
    "ac8c0a6c9b7badfa263ceb054ffe59c11162b1ca256c56cc6df5f0d378179401";
pub const FROZEN_M3_MD_SHA256: &str =
    "ab77bd12b705b8620b6315260f8bb5b4df6efc961f1d20a0dd521af403e1ac5f";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "a870a6e3d8021fb7fd8561d2a02929cd59a7c1d5ea508693384f835d27f61716";

pub const SEEDS: [u64; 16] = [
    8_000_000, 8_500_000, 9_000_000, 9_500_000, 10_000_000, 10_500_000, 11_000_000, 11_500_000,
    12_000_000, 12_500_000, 13_000_000, 13_500_000, 14_000_000, 14_500_000, 15_000_000, 15_500_000,
];

#[allow(dead_code)]
mod frozen_development {
    include!("ds6_cumulative_lifetime_probe.rs");

    pub(super) fn authority_cell(seed: u64) -> GateCell {
        frozen_m3::run_gate_cell(seed)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub development: bool,
    pub handoff: bool,
    pub m3_handoff: bool,
    pub m3_results: bool,
    pub protocol: bool,
    pub explicit_seeds: bool,
    pub disjoint_namespaces: bool,
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.development
            && self.handoff
            && self.m3_handoff
            && self.m3_results
            && self.protocol
            && self.explicit_seeds
            && self.disjoint_namespaces
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefinitiveCell {
    pub index: usize,
    pub seed: u64,
    pub recurrence: bool,
    pub pressure: bool,
    pub lifetimes: Vec<usize>,
    pub crossed: bool,
    pub interleaving: bool,
    pub loads: bool,
    pub reuse: bool,
    pub contradiction: bool,
    pub m3: bool,
    pub controls: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefinitiveReport {
    pub protocol: &'static str,
    pub source: SourceAudit,
    pub cells: Vec<DefinitiveCell>,
    pub duplicate_exact: bool,
    pub passed: bool,
    pub m3_authoritative: bool,
    pub m4_authoritative: bool,
    pub ds7_eligible: bool,
}

fn source_audit() -> SourceAudit {
    let explicit = SEEDS
        == [
            8_000_000, 8_500_000, 9_000_000, 9_500_000, 10_000_000, 10_500_000, 11_000_000,
            11_500_000, 12_000_000, 12_500_000, 13_000_000, 13_500_000, 14_000_000, 14_500_000,
            15_000_000, 15_500_000,
        ];
    SourceAudit {
        development: env!("DS6_DEFINITIVE_DEVELOPMENT_SHA256") == FROZEN_DEVELOPMENT_SHA256,
        handoff: env!("DS6_DEFINITIVE_HANDOFF_SHA256") == FROZEN_HANDOFF_SHA256,
        m3_handoff: env!("DS6_DEFINITIVE_M3_HANDOFF_SHA256") == FROZEN_M3_HANDOFF_SHA256,
        m3_results: env!("DS6_DEFINITIVE_M3_CSV_SHA256") == FROZEN_M3_CSV_SHA256
            && env!("DS6_DEFINITIVE_M3_MD_SHA256") == FROZEN_M3_MD_SHA256,
        protocol: env!("DS6_DEFINITIVE_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        explicit_seeds: explicit && SEEDS.len() == 16,
        disjoint_namespaces: SEEDS.windows(2).all(|pair| pair[1] - pair[0] >= 500_000)
            && SEEDS[0] >= 8_000_000,
    }
}
fn run_cells() -> Vec<DefinitiveCell> {
    SEEDS
        .iter()
        .copied()
        .enumerate()
        .map(|(index, seed)| {
            let cell = frozen_development::authority_cell(seed);
            let exact_lifetimes = cell.lifetimes == [1, 3, 6, 13, 27];
            let passed = cell.passed && exact_lifetimes;
            DefinitiveCell {
                index,
                seed,
                recurrence: cell.recurrence_ordering,
                pressure: cell.pressure_ordering,
                lifetimes: cell.lifetimes,
                crossed: cell.crossed_tradeoff,
                interleaving: cell.interleaving_invariant,
                loads: cell.load_behavior,
                reuse: cell.gap_reuse,
                contradiction: cell.contradiction_history,
                m3: cell.cumulative_m3,
                controls: cell.controls,
                passed,
            }
        })
        .collect()
}

pub fn run() -> DefinitiveReport {
    let source = source_audit();
    let cells = run_cells();
    let duplicate_exact = cells == run_cells();
    let passed = source.passed()
        && duplicate_exact
        && cells.len() == 16
        && cells.iter().all(|cell| cell.passed);
    DefinitiveReport {
        protocol: PROTOCOL,
        source,
        cells,
        duplicate_exact,
        passed,
        m3_authoritative: !passed,
        m4_authoritative: passed,
        ds7_eligible: passed,
    }
}

pub fn csv(report: &DefinitiveReport) -> String {
    let mut text = String::from(
        "index,seed,recurrence,pressure,lifetimes,crossed,interleaving,loads,reuse,contradiction,m3,controls,passed\n",
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "{},{},{},{},\"{:?}\",{},{},{},{},{},{},{},{}\n",
            cell.index,
            cell.seed,
            cell.recurrence,
            cell.pressure,
            cell.lifetimes,
            cell.crossed,
            cell.interleaving,
            cell.loads,
            cell.reuse,
            cell.contradiction,
            cell.m3,
            cell.controls,
            cell.passed
        ));
    }
    text
}

pub fn markdown(report: &DefinitiveReport) -> String {
    let mut text = format!(
        "# DS6 cumulative learned-lifetime definitive result\n\nVerdict: **{}**.\n\nProtocol: `{}`.\n\n| cell | seed | recurrence | pressure | lifetimes | crossed | interleaving | loads | reuse | contradiction | M3 | controls | result |\n|---:|---:|:---:|:---:|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|\n",
        if report.passed { "PASS" } else { "FAIL" },
        report.protocol
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "| {} | {} | {} | {} | {:?} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            cell.index,
            cell.seed,
            cell.recurrence,
            cell.pressure,
            cell.lifetimes,
            cell.crossed,
            cell.interleaving,
            cell.loads,
            cell.reuse,
            cell.contradiction,
            cell.m3,
            cell.controls,
            cell.passed
        ));
    }
    text.push_str(&format!(
        "\nFrozen source audit: `{}`. Duplicate exact: `{}`. M3 authoritative: `{}`. M4 authoritative: `{}`. DS7 eligible: `{}`.\n",
        report.source.passed(),
        report.duplicate_exact,
        report.m3_authoritative,
        report.m4_authoritative,
        report.ds7_eligible
    ));
    text
}
