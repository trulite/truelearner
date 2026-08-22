//! Write-once authority wrapper over the byte-frozen cumulative DS7 GATE v3 mechanism.

pub const PROTOCOL: &str = "ds7-cumulative-plasticity-allocation-definitive-v1";
pub const PROTOCOL_COMMIT: &str = "be8d6ade88c7a9ace32c5f705b333a60627cc415";
pub const READINESS_COMMIT: &str = "e7d153f334ddf2997c338d55a8aa63ffcc55a7e8";
pub const AUTHORITATIVE_M4: &str = "8db47281a7c9c97cbb52ced6fc3dcff0e7efa9b2";

pub const SEEDS: [u64; 16] = [
    30_000_000, 30_500_000, 31_000_000, 31_500_000, 32_000_000, 32_500_000, 33_000_000, 33_500_000,
    34_000_000, 34_500_000, 35_000_000, 35_500_000, 36_000_000, 36_500_000, 37_000_000, 37_500_000,
];
pub const LOADS: [usize; 3] = [8, 32, 128];
pub const CELL_NAMESPACE: u64 = 400_000;

const TARGET_SHA256: &str = "f10f9d7b16106b6014767ff6188a6d556145ba3e5b4335e28de245c7622a7595";
const ORDER_SHA256: &str = "609dc63ab8051316703899717fc30861d7a700d0ec60f205fa6d687ad478616d";
const M4_HANDOFF_SHA256: &str = "35bb33bddb014a08a0f1602520290bc1704032aefa3762052331434c88ef66ba";
const M4_CSV_SHA256: &str = "5c4a2e2b021a26a4cc2161202dd9a62205d426ba361f90a69d00ceb3df470a83";
const M4_MD_SHA256: &str = "c418f6b5fb5f8f3f83e385c75cd23fb8c88def3650ff80a13a491d0979944768";
const M4_SOURCE_SHA256: &str = "3d5659fb26ae804dee6122408f9d703ea1f226349772883075a42686ac3fd110";
const ACTIVATION_SHA256: &str = "f7290c939c54b78986596e937d4932335f995766f292c613eb522f552bb3e892";
const DEPENDENCY_SHA256: &str = "40305dd998b5fe80db9d4fcceee154288ea523f90bc57f26079025a7492b2509";
const MANIFEST_SHA256: &str = "138827ef5e9d761cd7cb58a672ed4a4776618d5eb56f5f4e88852ce18cb67504";
const P2_SHA256: &str = "704f757888d9b3bc89a5a3f5387f3422efb9dd4c746e3784506411b1da763b15";
const PROBE_PROTOCOL_SHA256: &str =
    "78109beadb9b96164d7a259f88cb73710a10fd6cdf773b053922743c5d8c7044";
const ALLOCATOR_SHA256: &str = "e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7";
const PROBE_RUNNER_SHA256: &str =
    "74b2e93e439d5d07cb80a079b762dcd2cca7646bae94a70c930c0663b14b2305";
const PROBE_RESULT_SHA256: &str =
    "93c11d30b9f3ebfbf6de68a315b3410275acc4031186eaa7e747a81f78a8ded4";
const PROBE_AUDIT_SHA256: &str = "b6d9f11fcb26d9702d95db989991558f3f86ca99dfe360b597b09a2b84d474f2";
const MICRO_PROTOCOL_SHA256: &str =
    "1648b4a0e19a13c918a3c92ad221c93b7ae145a34c6e960e57fd9d0f91506eb9";
const MICRO_SOURCE_SHA256: &str =
    "3fd46245614c17476b8fa44887e4a710d0c358ecc4f7ebe7da5fbdb5bb9459ec";
const MICRO_RUNNER_SHA256: &str =
    "e5042196e2f8c6dc238e17fc43c765ba9b612bf9ba0c0a5080be1005c3126114";
const MICRO_RESULT_SHA256: &str =
    "fa3d0510e0d821d32b2e3dd8c5bc9f357fca653f6e96f7d9d6b8fa19e61a377b";
const MICRO_AUDIT_SHA256: &str = "bc4e2cd797fb31133b315d8844301ca9734835176713cced0b049508475e84ff";
const DEVELOPMENT_HANDOFF_SHA256: &str =
    "e2cd6556b31c15457e6e0accf6b1369d0140378138fc063c417372da09fb3a1c";
const HISTORICAL_V2_PROTOCOL_SHA256: &str =
    "7f03c573fa6d5ad32f0401a1c8c260a844e9f943998a97fcc9a213a7636b61e3";
const HISTORICAL_V2_SOURCE_SHA256: &str =
    "519b7049da1f3d412132860e8e21f48731186b2675ea5a35a4bdc2da2a09098e";
const HISTORICAL_V2_RUNNER_SHA256: &str =
    "c2adb04e707acc6f375b66d29a1a28a06b060d21b1a5ac51820bdb29d7800a32";
const V2_RESULT_SHA256: &str = "cc0278c7476f50c505d7b8813c326203467b6b8b4e17c07f03188891750fccc9";
const V2_AUDIT_SHA256: &str = "9dbc561d7ec25ad9308df8285454e3b2a0f3c4dfcbb8502b1c1f98cd2cb2e58b";
const COLLAPSE_SHA256: &str = "10d35f4e0c29ead317ab3bd7254a83752877de582cb6f321b2676f187709e477";
const GATE_PROTOCOL_SHA256: &str =
    "324d328ed1ec1f20edfa3e5372a5fcefcca37d973e7b033f50cd2a0d26cfc9f5";
const GATE_SOURCE_SHA256: &str = "abaedd16717543270c5ed0ef2c8a16e3a4c0fed0215764443948c36d4adfa297";
const GATE_RUNNER_SHA256: &str = "606f1a3900f0f251da090ac9cfada39e35e5bfd0db301f0f945d4a3408cad97c";
const GATE_RESULT_SHA256: &str = "195827830258b36586d2311dc14b636b0d9b19404ce73cf2e5f2905e8f464baa";
const GATE_AUDIT_SHA256: &str = "50c82f70fb081c43991fff17ee7685aea451220454344a209fad10c3fae78f9b";
const READINESS_SHA256: &str = "41f1104f6b8fdf8d54a0b3420d3ede7899e1e606ad08ba7d7f48e62895e945fc";
const PROTOCOL_SHA256: &str = "85887e29737732fd98c7b578560b31cf0874c08210cd6e915c7e6fe06bd67f3f";

#[allow(dead_code)]
mod frozen_gate {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds7_cumulative_plasticity_allocation_gate_frozen.rs"
    ));

    pub(super) fn authority_cell(seed: u64, load: usize) -> GateCell {
        frozen_allocator::run_gate_cell(seed, load)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub target_and_order: bool,
    pub authoritative_m4: bool,
    pub activation_and_dependency: bool,
    pub probe_lineage: bool,
    pub micro_lineage: bool,
    pub immutable_v2_negative: bool,
    pub v2_historical_manifest: bool,
    pub gate_v3_lineage: bool,
    pub readiness_and_protocol: bool,
    pub wrapper_information_boundary: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.target_and_order
            && self.authoritative_m4
            && self.activation_and_dependency
            && self.probe_lineage
            && self.micro_lineage
            && self.immutable_v2_negative
            && self.v2_historical_manifest
            && self.gate_v3_lineage
            && self.readiness_and_protocol
            && self.wrapper_information_boundary
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Preflight {
    pub source: SourceAudit,
    pub explicit_seeds: bool,
    pub explicit_loads: bool,
    pub disjoint_namespaces: bool,
    pub development_disjoint: bool,
    pub outputs_absent: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefinitiveCell {
    pub index: usize,
    pub seed: u64,
    pub load: usize,
    pub blank_acquisition: bool,
    pub heldout_correct: usize,
    pub heldout_total: usize,
    pub productive_admissions: usize,
    pub distractor_admissions: usize,
    pub always_open_admissions: usize,
    pub admission_reduction: f64,
    pub shuffled_reversal: bool,
    pub entry_resistance_correct: i32,
    pub entry_resistance_shuffled: i32,
    pub final_resistance_correct: i32,
    pub final_resistance_shuffled: i32,
    pub removed_correct: bool,
    pub removed_shuffled: bool,
    pub stale_blocked: bool,
    pub repaired_correct: usize,
    pub repaired_total: usize,
    pub shuffled_repair_blocked: bool,
    pub retained_economy: bool,
    pub controls: bool,
    pub source_audit: bool,
    pub cumulative_m4: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefinitiveReport {
    pub protocol: &'static str,
    pub preflight: Preflight,
    pub cells: Vec<DefinitiveCell>,
    pub passed: bool,
    pub m4_authoritative: bool,
    pub m5_authoritative: bool,
    pub ds8_eligible: bool,
}

fn wrapper_information_boundary() -> bool {
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/ds7_cumulative_plasticity_allocation_definitive.rs"
    ));
    let cell_call = source
        .split("// DS7_AUTHORITY_CELL_CALL_BEGIN")
        .nth(1)
        .and_then(|text| text.split("// DS7_AUTHORITY_CELL_CALL_END").next())
        .unwrap_or_default();
    let forbidden = [
        "PlasticUnit",
        "EndpointKind",
        "sensory_class",
        "internal_class",
        "irrelevant_class",
        "LEARN_HERE",
        "candidate_list",
        "target_site",
        "proposal_site",
        "encounter_class",
        "productive_class",
        "distractor_class",
    ];
    forbidden.iter().all(|word| !cell_call.contains(word))
        && cell_call.contains("frozen_gate::authority_cell(seed, load)")
}

fn source_audit() -> SourceAudit {
    SourceAudit {
        target_and_order: env!("DS7_DEFINITIVE_TARGET_SHA256") == TARGET_SHA256
            && env!("DS7_DEFINITIVE_ORDER_SHA256") == ORDER_SHA256,
        authoritative_m4: env!("DS7_DEFINITIVE_M4_HANDOFF_SHA256") == M4_HANDOFF_SHA256
            && env!("DS7_DEFINITIVE_M4_CSV_SHA256") == M4_CSV_SHA256
            && env!("DS7_DEFINITIVE_M4_MD_SHA256") == M4_MD_SHA256
            && env!("DS7_DEFINITIVE_M4_SOURCE_SHA256") == M4_SOURCE_SHA256,
        activation_and_dependency: env!("DS7_DEFINITIVE_ACTIVATION_SHA256") == ACTIVATION_SHA256
            && env!("DS7_DEFINITIVE_DEPENDENCY_SHA256") == DEPENDENCY_SHA256
            && env!("DS7_DEFINITIVE_MANIFEST_SHA256") == MANIFEST_SHA256
            && env!("DS7_DEFINITIVE_P2_SHA256") == P2_SHA256,
        probe_lineage: env!("DS7_DEFINITIVE_PROBE_PROTOCOL_SHA256") == PROBE_PROTOCOL_SHA256
            && env!("DS7_DEFINITIVE_ALLOCATOR_SHA256") == ALLOCATOR_SHA256
            && env!("DS7_DEFINITIVE_PROBE_RUNNER_SHA256") == PROBE_RUNNER_SHA256
            && env!("DS7_DEFINITIVE_PROBE_RESULT_SHA256") == PROBE_RESULT_SHA256
            && env!("DS7_DEFINITIVE_PROBE_AUDIT_SHA256") == PROBE_AUDIT_SHA256,
        micro_lineage: env!("DS7_DEFINITIVE_MICRO_PROTOCOL_SHA256") == MICRO_PROTOCOL_SHA256
            && env!("DS7_DEFINITIVE_MICRO_SOURCE_SHA256") == MICRO_SOURCE_SHA256
            && env!("DS7_DEFINITIVE_MICRO_RUNNER_SHA256") == MICRO_RUNNER_SHA256
            && env!("DS7_DEFINITIVE_MICRO_RESULT_SHA256") == MICRO_RESULT_SHA256
            && env!("DS7_DEFINITIVE_MICRO_AUDIT_SHA256") == MICRO_AUDIT_SHA256
            && env!("DS7_DEFINITIVE_DEVELOPMENT_HANDOFF_SHA256") == DEVELOPMENT_HANDOFF_SHA256,
        immutable_v2_negative: env!("DS7_DEFINITIVE_V2_RESULT_SHA256") == V2_RESULT_SHA256
            && env!("DS7_DEFINITIVE_V2_AUDIT_SHA256") == V2_AUDIT_SHA256
            && env!("DS7_DEFINITIVE_COLLAPSE_SHA256") == COLLAPSE_SHA256,
        v2_historical_manifest: HISTORICAL_V2_PROTOCOL_SHA256
            == "7f03c573fa6d5ad32f0401a1c8c260a844e9f943998a97fcc9a213a7636b61e3"
            && HISTORICAL_V2_SOURCE_SHA256
                == "519b7049da1f3d412132860e8e21f48731186b2675ea5a35a4bdc2da2a09098e"
            && HISTORICAL_V2_RUNNER_SHA256
                == "c2adb04e707acc6f375b66d29a1a28a06b060d21b1a5ac51820bdb29d7800a32",
        gate_v3_lineage: env!("DS7_DEFINITIVE_GATE_PROTOCOL_SHA256") == GATE_PROTOCOL_SHA256
            && env!("DS7_DEFINITIVE_GATE_SOURCE_SHA256") == GATE_SOURCE_SHA256
            && env!("DS7_DEFINITIVE_GATE_RUNNER_SHA256") == GATE_RUNNER_SHA256
            && env!("DS7_DEFINITIVE_GATE_RESULT_SHA256") == GATE_RESULT_SHA256
            && env!("DS7_DEFINITIVE_GATE_AUDIT_SHA256") == GATE_AUDIT_SHA256,
        readiness_and_protocol: env!("DS7_DEFINITIVE_READINESS_SHA256") == READINESS_SHA256
            && env!("DS7_DEFINITIVE_PROTOCOL_SHA256") == PROTOCOL_SHA256,
        wrapper_information_boundary: wrapper_information_boundary(),
    }
}

pub fn source_preflight(outputs_absent: bool) -> Preflight {
    let source = source_audit();
    let explicit_seeds = SEEDS
        == [
            30_000_000, 30_500_000, 31_000_000, 31_500_000, 32_000_000, 32_500_000, 33_000_000,
            33_500_000, 34_000_000, 34_500_000, 35_000_000, 35_500_000, 36_000_000, 36_500_000,
            37_000_000, 37_500_000,
        ]
        && SEEDS.len() == 16;
    let explicit_loads = LOADS == [8, 32, 128] && LOADS.len() == 3;
    let disjoint_namespaces = SEEDS.windows(2).all(|pair| {
        pair[0] + CELL_NAMESPACE <= pair[1]
            && pair[1] - pair[0] == 500_000
            && pair[0] + 300_005 < pair[0] + CELL_NAMESPACE
    });
    let development_disjoint = SEEDS[0] > 24_500_000 + CELL_NAMESPACE;
    let passed = source.passed()
        && explicit_seeds
        && explicit_loads
        && disjoint_namespaces
        && development_disjoint
        && outputs_absent;
    Preflight {
        source,
        explicit_seeds,
        explicit_loads,
        disjoint_namespaces,
        development_disjoint,
        outputs_absent,
        passed,
    }
}

// DS7_AUTHORITY_CELL_CALL_BEGIN
fn run_cell(index: usize, seed: u64, load: usize) -> DefinitiveCell {
    let cell = frozen_gate::authority_cell(seed, load);
    let distractor_bound = load.div_ceil(8);
    let independently_conjunctive = cell.blank_acquisition
        && cell.heldout_correct == 32
        && cell.heldout_total == 32
        && cell.productive_admissions == 4
        && cell.distractor_admissions <= distractor_bound
        && cell.always_open_admissions == 4 + load
        && cell.admission_reduction >= 0.50
        && cell.shuffled_reversal
        && cell.entry_resistance_correct == 105
        && cell.entry_resistance_shuffled == 105
        && cell.final_resistance_correct == 0
        && cell.final_resistance_shuffled == 0
        && cell.removed_correct
        && cell.removed_shuffled
        && cell.stale_blocked
        && cell.repaired_correct == 32
        && cell.repaired_total == 32
        && cell.shuffled_repair_blocked
        && cell.retained_economy
        && cell.controls
        && cell.source_audit
        && cell.cumulative_m4;
    DefinitiveCell {
        index,
        seed,
        load,
        blank_acquisition: cell.blank_acquisition,
        heldout_correct: cell.heldout_correct,
        heldout_total: cell.heldout_total,
        productive_admissions: cell.productive_admissions,
        distractor_admissions: cell.distractor_admissions,
        always_open_admissions: cell.always_open_admissions,
        admission_reduction: cell.admission_reduction,
        shuffled_reversal: cell.shuffled_reversal,
        entry_resistance_correct: cell.entry_resistance_correct,
        entry_resistance_shuffled: cell.entry_resistance_shuffled,
        final_resistance_correct: cell.final_resistance_correct,
        final_resistance_shuffled: cell.final_resistance_shuffled,
        removed_correct: cell.removed_correct,
        removed_shuffled: cell.removed_shuffled,
        stale_blocked: cell.stale_blocked,
        repaired_correct: cell.repaired_correct,
        repaired_total: cell.repaired_total,
        shuffled_repair_blocked: cell.shuffled_repair_blocked,
        retained_economy: cell.retained_economy,
        controls: cell.controls,
        source_audit: cell.source_audit,
        cumulative_m4: cell.cumulative_m4,
        passed: cell.passed && independently_conjunctive,
    }
}
// DS7_AUTHORITY_CELL_CALL_END

pub fn run_definitive(outputs_absent: bool) -> DefinitiveReport {
    let preflight = source_preflight(outputs_absent);
    assert!(
        preflight.passed,
        "definitive preflight must pass before any cell"
    );
    let mut cells = Vec::with_capacity(SEEDS.len() * LOADS.len());
    for seed in SEEDS {
        for load in LOADS {
            let index = cells.len();
            cells.push(run_cell(index, seed, load));
        }
    }
    let passed = cells.len() == 48 && cells.iter().all(|cell| cell.passed);
    DefinitiveReport {
        protocol: PROTOCOL,
        preflight,
        cells,
        passed,
        m4_authoritative: !passed,
        m5_authoritative: passed,
        ds8_eligible: passed,
    }
}

pub fn csv(report: &DefinitiveReport) -> String {
    let mut text = String::from(
        "index,seed,load,acquisition,heldout_correct,heldout_total,productive_admissions,distractor_admissions,always_open_admissions,admission_reduction,shuffled_reversal,entry_resistance_correct,entry_resistance_shuffled,final_resistance_correct,final_resistance_shuffled,removed_correct,removed_shuffled,stale_blocked,repaired_correct,repaired_total,shuffled_repair_blocked,retained_economy,controls,source_audit,cumulative_m4,passed\n",
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{:.6},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            cell.index,
            cell.seed,
            cell.load,
            cell.blank_acquisition,
            cell.heldout_correct,
            cell.heldout_total,
            cell.productive_admissions,
            cell.distractor_admissions,
            cell.always_open_admissions,
            cell.admission_reduction,
            cell.shuffled_reversal,
            cell.entry_resistance_correct,
            cell.entry_resistance_shuffled,
            cell.final_resistance_correct,
            cell.final_resistance_shuffled,
            cell.removed_correct,
            cell.removed_shuffled,
            cell.stale_blocked,
            cell.repaired_correct,
            cell.repaired_total,
            cell.shuffled_repair_blocked,
            cell.retained_economy,
            cell.controls,
            cell.source_audit,
            cell.cumulative_m4,
            cell.passed,
        ));
    }
    text
}

pub fn markdown(report: &DefinitiveReport) -> String {
    let mut text = format!(
        "# DS7 cumulative learned plasticity-allocation definitive result\n\nVerdict: **{}**.\n\nProtocol: `{}`.\n\nCells: `{}/48`. Frozen source preflight: `{}`.\n\n| cell | seed | load | acquisition | held-out | admitted P/N | baseline | reduction | shuffled | resistance entry/final | removed | stale | repaired | shuffled repair blocked | economy | controls | source | M4 | result |\n|---:|---:|---:|:---:|---:|---:|---:|---:|:---:|---:|:---:|:---:|---:|:---:|:---:|:---:|:---:|:---:|:---:|\n",
        if report.passed { "PASS" } else { "FAIL" },
        report.protocol,
        report.cells.iter().filter(|cell| cell.passed).count(),
        report.preflight.source.passed(),
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "| {} | {} | {} | {} | {}/{} | {}/{} | {} | {:.2}% | {} | {}/{} -> {}/{} | {}/{} | {} | {}/{} | {} | {} | {} | {} | {} | {} |\n",
            cell.index,
            cell.seed,
            cell.load,
            cell.blank_acquisition,
            cell.heldout_correct,
            cell.heldout_total,
            cell.productive_admissions,
            cell.distractor_admissions,
            cell.always_open_admissions,
            100.0 * cell.admission_reduction,
            cell.shuffled_reversal,
            cell.entry_resistance_correct,
            cell.entry_resistance_shuffled,
            cell.final_resistance_correct,
            cell.final_resistance_shuffled,
            cell.removed_correct,
            cell.removed_shuffled,
            cell.stale_blocked,
            cell.repaired_correct,
            cell.repaired_total,
            cell.shuffled_repair_blocked,
            cell.retained_economy,
            cell.controls,
            cell.source_audit,
            cell.cumulative_m4,
            cell.passed,
        ));
    }
    text.push_str(&format!(
        "\nM4 authoritative: `{}`. M5 authoritative: `{}`. DS8 cumulative eligible: `{}`.\n",
        report.m4_authoritative, report.m5_authoritative, report.ds8_eligible
    ));
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_cell_preflight_is_exact_and_disjoint() {
        let audit = source_preflight(true);
        assert!(audit.source.passed(), "{audit:#?}");
        assert!(audit.explicit_seeds);
        assert!(audit.explicit_loads);
        assert!(audit.disjoint_namespaces);
        assert!(audit.development_disjoint);
        assert!(audit.outputs_absent);
        assert!(audit.passed);
    }

    #[test]
    fn no_cell_preflight_refuses_existing_output() {
        let audit = source_preflight(false);
        assert!(!audit.outputs_absent);
        assert!(!audit.passed);
    }
}
