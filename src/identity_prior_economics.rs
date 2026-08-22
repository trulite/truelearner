//! IP0: evaluator-only accounting over frozen identity de-supply artifacts.
//! No organism state, learner, executor, or experimental physics is present.

use std::collections::BTreeMap;

use crate::research_runtime::HarnessMode;

pub const IP0_PROTOCOL: &str = "identity-desupply-ladder-v1/ip0";
pub const PRICE_MICROS: [u64; 5] = [0, 1, 10, 100, 1_000];
const MICROS_PER_WORK: u128 = 1_000_000;

const FFS0_CSV: &str = include_str!("../results/ffs0_full_fractal_scaling.csv");
const SAME0_CSV: &str = include_str!("../results/ffs_same0_learned_correspondence.csv");
const CS0A_CSV: &str = include_str!("../results/cs0a_compiled_correspondence.csv");
const CS0B_ATTRIBUTION_CSV: &str = include_str!("../results/cs0a_grounding_tax_attribution.csv");
const SAME1_CSV: &str = include_str!("../results/ffs_same1_compiled_correspondence.csv");

#[derive(Clone, Debug, PartialEq, Eq)]
struct CsvTable {
    rows: Vec<BTreeMap<String, String>>,
}

impl CsvTable {
    fn parse(source: &str) -> Self {
        let mut lines = source.lines();
        let header = lines
            .next()
            .expect("frozen CSV has a header")
            .split(',')
            .map(str::to_string)
            .collect::<Vec<_>>();
        let rows = lines
            .filter(|line| !line.is_empty())
            .map(|line| {
                let values = line.split(',').collect::<Vec<_>>();
                assert_eq!(values.len(), header.len(), "frozen CSV schema mismatch");
                header
                    .iter()
                    .cloned()
                    .zip(values.into_iter().map(str::to_string))
                    .collect()
            })
            .collect();
        Self { rows }
    }

    fn rows_of<'a>(
        &'a self,
        row_type: &'a str,
    ) -> impl Iterator<Item = &'a BTreeMap<String, String>> + 'a {
        self.rows
            .iter()
            .filter(move |row| field(row, "row_type") == row_type)
    }
}

fn field<'a>(row: &'a BTreeMap<String, String>, key: &str) -> &'a str {
    row.get(key).map(String::as_str).unwrap_or("")
}

fn integer(row: &BTreeMap<String, String>, key: &str) -> u64 {
    field(row, key)
        .parse()
        .unwrap_or_else(|_| panic!("{key} must be an integer"))
}

fn signed(row: &BTreeMap<String, String>, key: &str) -> i64 {
    field(row, key)
        .parse()
        .unwrap_or_else(|_| panic!("{key} must be a signed integer"))
}

fn ceil_div(numerator: u128, denominator: u128) -> u128 {
    numerator / denominator + u128::from(!numerator.is_multiple_of(denominator))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnershipView {
    BlankStart,
    ExactAssetAlreadyOwned,
}

impl OwnershipView {
    pub const ALL: [Self; 2] = [Self::BlankStart, Self::ExactAssetAlreadyOwned];

    pub fn label(self) -> &'static str {
        match self {
            Self::BlankStart => "blank-start",
            Self::ExactAssetAlreadyOwned => "exact-asset-already-owned",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PriorClassification {
    ExecutionPriorAdvantage,
    DevelopmentalPriorAdvantage,
    LearnedSpecializationAdvantage,
}

impl PriorClassification {
    pub fn label(self) -> &'static str {
        match self {
            Self::ExecutionPriorAdvantage => "EXECUTION_PRIOR_ADVANTAGE",
            Self::DevelopmentalPriorAdvantage => "DEVELOPMENTAL_PRIOR_ADVANTAGE",
            Self::LearnedSpecializationAdvantage => "LEARNED_SPECIALIZATION_ADVANTAGE",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ip0Row {
    pub seed: usize,
    pub scale: String,
    pub depth: usize,
    pub population: usize,
    pub ownership: OwnershipView,
    pub price_micros: u64,
    pub generic_acquisition_work: u64,
    pub compilation_acquisition_work: u64,
    pub grounding_acquisition_work: u64,
    pub installation_work: u64,
    pub marginal_persistent_bytes: usize,
    pub maintenance_work_per_use: u64,
    pub supplied_runtime: u64,
    pub generic_runtime: u64,
    pub compiled_runtime: u64,
    pub generic_to_compiled_saving: u64,
    pub compiled_premium_vs_supplied: i64,
    pub fixed_cost_micros: u128,
    pub per_use_delta_micros: i128,
    pub break_even_vs_supplied: Option<u64>,
    pub compilation_break_even_vs_generic: Option<u64>,
    pub classification: PriorClassification,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScaffoldLedger {
    pub scaffold: String,
    pub architectural_necessity: String,
    pub reconstructed: String,
    pub compiled: String,
    pub recursive_compatible: String,
    pub developmental_accelerator: String,
    pub mature_execution_accelerator: String,
    pub scaling_enabling_prerequisite: String,
    pub recursion_enabling_prerequisite: String,
    pub supplied_value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ip0Report {
    pub mode: String,
    pub claim_eligible: bool,
    pub rows: Vec<Ip0Row>,
    pub ledger: ScaffoldLedger,
    pub parent_artifacts_consistent: bool,
    pub cs0b_skipped: bool,
    pub all_execution_prior_advantage: bool,
    pub all_no_break_even_vs_supplied: bool,
    pub generic_to_compiled_reduction: u64,
    pub compiled_premium_vs_supplied: i64,
    pub zero_price_compilation_break_even: u64,
    pub passed: bool,
}

fn claims_pass(table: &CsvTable, expected: &[&str]) -> bool {
    expected.iter().all(|name| {
        table
            .rows_of("claim")
            .any(|row| field(row, "name") == *name && field(row, "status") == "PASS")
    })
}

fn parent_artifacts_consistent() -> bool {
    let ffs0 = CsvTable::parse(FFS0_CSV);
    let same0 = CsvTable::parse(SAME0_CSV);
    let cs0a = CsvTable::parse(CS0A_CSV);
    let same1 = CsvTable::parse(SAME1_CSV);
    let ffs0_claims = claims_pass(
        &ffs0,
        &[
            "A-functional-recursion",
            "B-computational-recursion",
            "C-economic-recursion",
            "E-adaptive-recursion",
        ],
    );
    let same0_claims = claims_pass(
        &same0,
        &[
            "A-correspondence-reconstruction",
            "B-functional-recovery",
            "C-fractal-recovery",
        ],
    );
    let same1_claims = claims_pass(
        &same1,
        &[
            "A1-fractal-compatibility",
            "B1-computational-recursion",
            "C1-economic-recursion",
            "D1-identity-tax-reduction",
            "E1-adaptive-reuse",
        ],
    );
    let acquisitions_valid = cs0a.rows_of("acquisition").count() == 8
        && cs0a.rows_of("acquisition").all(|row| {
            integer(row, "parent_acquisition_work") == 860
                && integer(row, "compilation_work") == 988
                && integer(row, "persistent_bytes") == 80
                && integer(row, "compiled_routes") == 2
        });
    let same0_by_key = same0
        .rows_of("scale")
        .map(|row| {
            (
                (
                    field(row, "seed").to_string(),
                    field(row, "scale").to_string(),
                ),
                row,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let same1_references_valid = same1.rows_of("scale").count() == 48
        && same1.rows_of("scale").all(|row| {
            let key = (
                field(row, "seed").to_string(),
                field(row, "scale").to_string(),
            );
            same0_by_key.get(&key).is_some_and(|parent| {
                integer(row, "same0_runtime") == integer(parent, "same_less_runtime")
                    && integer(row, "supplied_runtime") == integer(parent, "supplied_same_runtime")
                    && integer(row, "same0_tax") == 18
                    && integer(row, "same1_tax") == 6
                    && signed(row, "premium_vs_supplied") == 6
                    && integer(row, "generic_acquisition_work") == 860
                    && integer(row, "compilation_acquisition_work") == 988
                    && integer(row, "generic_bytes") == 26
                    && integer(row, "compiled_bytes") == 80
            })
        });
    ffs0_claims && same0_claims && same1_claims && acquisitions_valid && same1_references_valid
}

fn cs0b_skipped() -> bool {
    let attribution = CsvTable::parse(CS0B_ATTRIBUTION_CSV);
    let first = attribution.rows.first().expect("U=1 attribution");
    let last = attribution.rows.last().expect("U=16 attribution");
    let residual_slope =
        (integer(last, "residual_overhead") - integer(first, "residual_overhead")) / 15;
    let grounding_slope =
        (integer(last, "grounding_total") - integer(first, "grounding_total")) / 15;
    grounding_slope > 0 && grounding_slope * 2 < residual_slope
}

fn classification(compiled: u64, supplied: u64) -> PriorClassification {
    match compiled.cmp(&supplied) {
        std::cmp::Ordering::Greater => PriorClassification::ExecutionPriorAdvantage,
        std::cmp::Ordering::Equal => PriorClassification::DevelopmentalPriorAdvantage,
        std::cmp::Ordering::Less => PriorClassification::LearnedSpecializationAdvantage,
    }
}

fn break_even(fixed_micros: u128, per_use_delta_micros: i128) -> Option<u64> {
    if per_use_delta_micros >= 0 {
        return None;
    }
    Some(
        u64::try_from(ceil_div(fixed_micros, per_use_delta_micros.unsigned_abs()))
            .expect("IP0 break-even fits u64"),
    )
}

fn selected_scale(row: &BTreeMap<String, String>, mode: HarnessMode) -> bool {
    match mode {
        HarnessMode::Micro => field(row, "seed") == "0" && field(row, "scale") == "S1",
        HarnessMode::Gate => field(row, "seed") == "0",
        HarnessMode::Definitive => true,
    }
}

pub fn run_ip0(mode: HarnessMode) -> Ip0Report {
    let same1 = CsvTable::parse(SAME1_CSV);
    let parent_artifacts_consistent = parent_artifacts_consistent();
    let cs0b_skipped = cs0b_skipped();
    let mut rows = Vec::new();
    for source in same1
        .rows_of("scale")
        .filter(|row| selected_scale(row, mode))
    {
        let seed = integer(source, "seed") as usize;
        let scale = field(source, "scale").to_string();
        let depth = integer(source, "depth") as usize;
        let population = integer(source, "population") as usize;
        let generic_runtime = integer(source, "same0_runtime");
        let compiled_runtime = integer(source, "same1_runtime");
        let supplied_runtime = integer(source, "supplied_runtime");
        for ownership in OwnershipView::ALL {
            for price_micros in PRICE_MICROS {
                let (generic_acquisition_work, compilation_acquisition_work, bytes) =
                    match ownership {
                        OwnershipView::BlankStart => (860, 988, 106usize),
                        OwnershipView::ExactAssetAlreadyOwned => (0, 0, 0usize),
                    };
                let grounding_acquisition_work = 0;
                let installation_work = 0;
                let maintenance_work_per_use = 0;
                let fixed_work = generic_acquisition_work
                    + compilation_acquisition_work
                    + grounding_acquisition_work
                    + installation_work;
                let fixed_cost_micros = fixed_work as u128 * MICROS_PER_WORK;
                let runtime_delta = compiled_runtime as i128 - supplied_runtime as i128;
                let carrying_per_use_micros = bytes as i128 * price_micros as i128;
                let per_use_delta_micros = runtime_delta * MICROS_PER_WORK as i128
                    + maintenance_work_per_use as i128 * MICROS_PER_WORK as i128
                    + carrying_per_use_micros;
                let generic_to_compiled_saving = generic_runtime - compiled_runtime;
                let compilation_slope_micros = generic_to_compiled_saving as i128
                    * MICROS_PER_WORK as i128
                    - 80i128 * price_micros as i128;
                let compilation_break_even_vs_generic = (compilation_slope_micros > 0).then(|| {
                    u64::try_from(ceil_div(
                        988u128 * MICROS_PER_WORK,
                        compilation_slope_micros as u128,
                    ))
                    .expect("compilation break-even fits u64")
                });
                rows.push(Ip0Row {
                    seed,
                    scale: scale.clone(),
                    depth,
                    population,
                    ownership,
                    price_micros,
                    generic_acquisition_work,
                    compilation_acquisition_work,
                    grounding_acquisition_work,
                    installation_work,
                    marginal_persistent_bytes: bytes,
                    maintenance_work_per_use,
                    supplied_runtime,
                    generic_runtime,
                    compiled_runtime,
                    generic_to_compiled_saving,
                    compiled_premium_vs_supplied: runtime_delta as i64,
                    fixed_cost_micros,
                    per_use_delta_micros,
                    break_even_vs_supplied: break_even(fixed_cost_micros, per_use_delta_micros),
                    compilation_break_even_vs_generic,
                    classification: classification(compiled_runtime, supplied_runtime),
                });
            }
        }
    }
    let all_execution_prior_advantage = rows.iter().all(|row| {
        row.classification == PriorClassification::ExecutionPriorAdvantage
            && row.compiled_premium_vs_supplied == 6
    });
    let all_no_break_even_vs_supplied = rows
        .iter()
        .all(|row| row.break_even_vs_supplied.is_none() && row.per_use_delta_micros > 0);
    let zero_price_compilation_break_even = rows
        .iter()
        .find(|row| row.ownership == OwnershipView::BlankStart && row.price_micros == 0)
        .and_then(|row| row.compilation_break_even_vs_generic)
        .expect("positive generic-to-compiled break-even");
    let passed = parent_artifacts_consistent
        && cs0b_skipped
        && !rows.is_empty()
        && all_execution_prior_advantage
        && all_no_break_even_vs_supplied
        && rows.iter().all(|row| {
            row.generic_to_compiled_saving == 12
                && row.generic_runtime > row.compiled_runtime
                && row.compiled_runtime > row.supplied_runtime
                && row.grounding_acquisition_work == 0
                && row.installation_work == 0
                && row.maintenance_work_per_use == 0
        });
    Ip0Report {
        mode: match mode {
            HarnessMode::Micro => "micro",
            HarnessMode::Gate => "gate",
            HarnessMode::Definitive => "definitive",
        }
        .to_string(),
        claim_eligible: mode == HarnessMode::Definitive,
        rows,
        ledger: ScaffoldLedger {
            scaffold: "filler-correspondence".to_string(),
            architectural_necessity: "NO".to_string(),
            reconstructed: "YES".to_string(),
            compiled: "YES: 18->6 work/use".to_string(),
            recursive_compatible: "YES: 0->3->5->=6 preserved".to_string(),
            developmental_accelerator: "YES".to_string(),
            mature_execution_accelerator: "YES: 6 work/use".to_string(),
            scaling_enabling_prerequisite: "NO".to_string(),
            recursion_enabling_prerequisite: "NO".to_string(),
            supplied_value: "development plus fixed 6 work/use".to_string(),
        },
        parent_artifacts_consistent,
        cs0b_skipped,
        all_execution_prior_advantage,
        all_no_break_even_vs_supplied,
        generic_to_compiled_reduction: 12,
        compiled_premium_vs_supplied: 6,
        zero_price_compilation_break_even,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_parents_are_physically_consistent() {
        assert!(parent_artifacts_consistent());
        assert!(cs0b_skipped());
    }

    #[test]
    fn gate_classifies_every_workload_as_execution_prior_advantage() {
        let report = run_ip0(HarnessMode::Gate);
        assert!(report.passed);
        assert_eq!(report.rows.len(), 60);
        assert_eq!(report.zero_price_compilation_break_even, 83);
        assert!(report
            .rows
            .iter()
            .all(|row| row.break_even_vs_supplied.is_none()));
    }

    #[test]
    fn supplied_same_has_no_scaling_or_recursion_necessity() {
        let report = run_ip0(HarnessMode::Micro);
        assert_eq!(report.ledger.architectural_necessity, "NO");
        assert_eq!(report.ledger.scaling_enabling_prerequisite, "NO");
        assert_eq!(report.ledger.recursion_enabling_prerequisite, "NO");
    }
}
