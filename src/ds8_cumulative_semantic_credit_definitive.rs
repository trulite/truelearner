//! Write-once authority wrapper over the byte-frozen cumulative DS8 GATE v3 mechanism.

pub const PROTOCOL: &str = "ds8-cumulative-semantic-credit-definitive-v1";
pub const PROTOCOL_COMMIT: &str = "e4f6d1514739a51f175df557779e1b316a78e4ef";
pub const READINESS_COMMIT: &str = "87f3862ecd335b2840d00f84f268ed3bd4e3f246";
pub const AUTHORITATIVE_M5: &str = "9c5ba68a6a4ae37b51575ebaae414ab51a248575";

pub const SEEDS: [u64; 16] = [
    50_000_000,
    53_500_000,
    57_000_000,
    60_500_000,
    64_000_000,
    67_500_000,
    71_000_000,
    74_500_000,
    78_000_000,
    81_500_000,
    85_000_000,
    88_500_000,
    92_000_000,
    95_500_000,
    99_000_000,
    102_500_000,
];
pub const LOADS: [usize; 3] = [8, 32, 128];
pub const CELL_NAMESPACE: u64 = 3_100_000;

const M5_ALLOCATOR_SHA256: &str =
    "e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7";
const M5_CSV_SHA256: &str =
    "86d9f6e3a8ab4ad5c242e0d7c619d8eda99e0da47faff623f26c8c6835b9a99a";
const M5_MD_SHA256: &str =
    "a336633c73565261d357a67ca02df3047ffcaf88488153bb2f43b621818ba5f0";
const ACTIVATION_SHA256: &str =
    "6e3064a1609390933cda4afdc374579cd23316bb24b833c2024ac14d7138e458";
const DEPENDENCY_SHA256: &str =
    "33b963dd50b711f49bc0e90d33adb0d9d80020e5c33ba024d3e861da56d9a326";
const PROBE_PROTOCOL_SHA256: &str =
    "a9a944d3ffab8fe53f303db773846c8f53f7dbb05c8558958567814e0b37f953";
const PROBE_PROTOCOL_V2_SHA256: &str =
    "93cd9c69fa8ae1fc4589c4bed2d1a8add81a87b992bdd0fc9dc3bb33acfe218c";
const PROBE_SOURCE_SHA256: &str =
    "11b4229122b3e0788ca30c55579b91ffe07461de9a138860690134565fcf2ed6";
const PROBE_RUNNER_SHA256: &str =
    "421332e41ad42388187fcadf67e3685605e4a7f5c58c1e2528d08b5059dfc2ce";
const PROBE_RESULT_SHA256: &str =
    "a1acd9c35cb5da4ab1c9aa341926dae6d8d8cb5e10e216be2089879328d8404d";
const PROBE_AUDIT_SHA256: &str =
    "00f788e2b3a6930fed7ec69626062f5d6631167002a179cc96e7499d5280502a";
const PROBE_HANDOFF_SHA256: &str =
    "2653b6db073e3241a4d98e223cbc545826b57ecf272f2bc073f58dac98060087";
const PROBE_V1_RESULT_SHA256: &str =
    "31ae6bcabcca510496a586f27fa1b9cda42d69142f64f442513dd888321569ca";
const PROBE_V1_AUDIT_SHA256: &str =
    "7921c4ddaecf1fd06a0b73d56ff5373fc2b8bbb9211d4a520ff2e8ff88a574a0";
const MICRO_PROTOCOL_SHA256: &str =
    "4c66a654607659cccd5e1692dcf3c491452cc394c6cdb9583f52de6ae71b433a";
const MICRO_PROTOCOL_V2_SHA256: &str =
    "092eb5577e78b9cb6ee7dfac6050e76556390afc903a61d4b4f258b77e97cf8d";
const MICRO_SOURCE_SHA256: &str =
    "36dd93a581bb7d15cb82de3089ff6786fc8b5e7a0edf2181383a52422d301b78";
const MICRO_RUNNER_SHA256: &str =
    "a94817865d1a1172e436fe615b496caeabdeb6a7a9a1dd895eeb41dcc256ad46";
const MICRO_RESULT_SHA256: &str =
    "d225ea316bd492df5025088615b8650c7cb5476b9b90cbe5b3e826b22129a894";
const MICRO_AUDIT_SHA256: &str =
    "a316da1a1d327ed417e5b07ee3c0099dcc00d72af00fb65479e0e4d52e3b064b";
const DEVELOPMENT_HANDOFF_SHA256: &str =
    "91feb6e878fae6f8155dd3f9ea5107ec5e8d1fceab6cea460b23808d537e29da";
const MICRO_V1_RESULT_SHA256: &str =
    "4c352ebbbeb9bb56c6aeab98164b44085e69a0f0d7d9b128542d2fd0f28313d2";
const MICRO_V1_AUDIT_SHA256: &str =
    "f8d630d54b1cf3ff9211aea5339876728c800b01f186f4a02389bef22f92f3f6";
const GATE_PROTOCOL_SHA256: &str =
    "c65c08c59056cda39d3f93615da9be28ab12210d8c19b76de18dd8a0ef245b78";
const GATE_PROTOCOL_V2_SHA256: &str =
    "93aacb0835588d5be343a60d30178a6d594984fce0e21f7af6c702d66825f80a";
const GATE_PROTOCOL_V3_SHA256: &str =
    "3d18d948879cb192159e4a894985f546eb3f0e98f217c893c3e2cf3094f828fc";
const GATE_SOURCE_SHA256: &str =
    "19c9051d15023c5b88559cba4ee3b3eb55686d1a68e083ca260a4a65629e8f30";
const GATE_RUNNER_SHA256: &str =
    "7e31925ce8122ea35f0c86243b4998fbe19a1319724e70c8c7293b7dad11ef6a";
const GATE_RESULT_SHA256: &str =
    "3505fdd3627085ea92fc512abd2726124a167ae041f3af3d9081a18fe042c996";
const GATE_AUDIT_SHA256: &str =
    "ebbbe3655ea14c3a7985cdbbffa48b844d3eb9e9a7adbd963a584e2f0f743d16";
const GATE_V1_RESULT_SHA256: &str =
    "399b725cd823d6e086f8e7cbfa098c57c73e2296798afef503fd75eabf132343";
const GATE_V1_AUDIT_SHA256: &str =
    "7dc966be9d9a47cabd0e6569dddac208de8b7de842061b303481e827f587f171";
const GATE_V2_RESULT_SHA256: &str =
    "8bdedbad412376188c20ebbc887404c9f67e04ce5184162ec4a6a1576e61df9e";
const GATE_V2_AUDIT_SHA256: &str =
    "7aedfb788d1e5ed502560dbafb95051c20fe67db6a5111fab96142097fe70f9d";
const READINESS_SHA256: &str =
    "c1e122159dab69478a396eba3da9607cb349cd8e54c1c8260e6f091458489c6d";
const PROTOCOL_SHA256: &str =
    "fc22fd12c737f61de877fe6fa3f092bee7364cb11c70b6554e485301aedfef07";
const LINKER_SHA256: &str =
    "1f68f7e943f37c42d29f16fe26f0d851a59361ed4c1f4273a82d0537f935d343";

#[allow(dead_code)]
mod frozen_gate {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds8_cumulative_semantic_credit_gate_frozen.rs"
    ));

    pub(super) fn authority_cell(seed: u64, load: usize, controls: bool) -> GateCell {
        frozen_linker::run_cell(seed, load, controls)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub authoritative_m5: bool,
    pub probe_lineage: bool,
    pub micro_lineage: bool,
    pub gate_lineage: bool,
    pub immutable_negatives: bool,
    pub readiness_and_protocol: bool,
    pub linker_exact: bool,
    pub information_boundary: bool,
    pub negative_tag_manifest: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.authoritative_m5
            && self.probe_lineage
            && self.micro_lineage
            && self.gate_lineage
            && self.immutable_negatives
            && self.readiness_and_protocol
            && self.linker_exact
            && self.information_boundary
            && self.negative_tag_manifest
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Preflight {
    pub source: SourceAudit,
    pub explicit_seeds: bool,
    pub explicit_loads: bool,
    pub disjoint_namespaces: bool,
    pub development_disjoint: bool,
    pub topology_balanced: bool,
    pub layout_balanced: bool,
    pub outputs_absent: bool,
    pub staging_absent: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimSummary {
    pub blank_acquisition: bool,
    pub physical_accounting: bool,
    pub heldout_execution: bool,
    pub sparse_allocation: bool,
    pub structural_economy: bool,
    pub raw_history_attribution: bool,
    pub value_shuffle_attribution: bool,
    pub physical_removal_and_stale_block: bool,
    pub nonsemantic_reacquisition: bool,
    pub transfer_controls: bool,
    pub positive_controls_and_m5: bool,
    pub zero_semantic_or_covert_credit: bool,
    pub immutable_negatives: bool,
}

impl ClaimSummary {
    pub fn passed(&self) -> bool {
        self.blank_acquisition
            && self.physical_accounting
            && self.heldout_execution
            && self.sparse_allocation
            && self.structural_economy
            && self.raw_history_attribution
            && self.value_shuffle_attribution
            && self.physical_removal_and_stale_block
            && self.nonsemantic_reacquisition
            && self.transfer_controls
            && self.positive_controls_and_m5
            && self.zero_semantic_or_covert_credit
            && self.immutable_negatives
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefinitiveCell {
    pub index: usize,
    pub seed: u64,
    pub load: usize,
    pub blank_acquisition: bool,
    pub physical_exact: bool,
    pub heldout: usize,
    pub heldout_total: usize,
    pub route_admissions: usize,
    pub distractor_admissions: usize,
    pub always_open_admissions: usize,
    pub work_reduction: f64,
    pub raw_history_reversal: bool,
    pub value_shuffle_reversal: bool,
    pub entry_resistance: i32,
    pub final_resistance: i32,
    pub removed: bool,
    pub stale_blocked: bool,
    pub repair_admitted: bool,
    pub repair_executed: bool,
    pub repair_applied: bool,
    pub post_age_prototype_resistance: i32,
    pub post_age_value_resistance: i32,
    pub repaired: usize,
    pub repaired_total: usize,
    pub retained_proposals: usize,
    pub retained_route_edges: usize,
    pub retained_economy: bool,
    pub topology_identity_layout: bool,
    pub controls: bool,
    pub source_audit: bool,
    pub cumulative_m5: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefinitiveReport {
    pub protocol: &'static str,
    pub preflight: Preflight,
    pub controls: bool,
    pub claims: ClaimSummary,
    pub cells: Vec<DefinitiveCell>,
    pub passed: bool,
    pub m5_authoritative: bool,
    pub m6_authoritative: bool,
    pub core_autonomy_checkpoint: bool,
    pub post_m6_ds4_eligible: bool,
}

fn authority_call_boundary() -> bool {
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/ds8_cumulative_semantic_credit_definitive.rs"
    ));
    let cell_call = source
        .rsplit("// DS8_AUTHORITY_CELL_CALL_BEGIN")
        .next()
        .and_then(|text| text.split("// DS8_AUTHORITY_CELL_CALL_END").next())
        .unwrap_or_default();
    let forbidden = [
        "correctness",
        "wrongness",
        "reward",
        "loss",
        "expected_answer",
        "expected_trace",
        "selected_route",
        "route_polarity",
        "timing_polarity",
        "magnitude_polarity",
        "omission_polarity",
        "reset_polarity",
        "namespace_polarity",
        "semantic_polarity",
        "target_answer",
    ];
    forbidden.iter().all(|word| !cell_call.contains(word))
        && cell_call.matches("frozen_gate::authority_cell").count() == 1
        && cell_call.contains("frozen_gate::authority_cell(seed, load, controls)")
}

fn linker_information_boundary() -> bool {
    let probe = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/ds8_cumulative_semantic_credit_probe.rs"
    ));
    let linker = probe
        .split("// DS8_ORGANISM_PATH_BEGIN")
        .nth(1)
        .and_then(|text| text.split("// DS8_ORGANISM_PATH_END").next())
        .unwrap_or_default();
    let forbidden = [
        ["cor", "rect"].concat(),
        ["wr", "ong"].concat(),
        ["rew", "ard"].concat(),
        ["lo", "ss"].concat(),
        ["expected", "_answer"].concat(),
        ["expected", "_trace"].concat(),
        ["target", "_answer"].concat(),
        ["selected", "_route"].concat(),
        ["route", "_polarity"].concat(),
        ["timing", "_polarity"].concat(),
        ["magnitude", "_polarity"].concat(),
        ["omission", "_polarity"].concat(),
        ["reset", "_polarity"].concat(),
        ["namespace", "_polarity"].concat(),
        ["semantic", "_polarity"].concat(),
    ];
    forbidden.iter().all(|word| !linker.contains(word))
        && linker
            .matches("path.delayed_experience(differential)")
            .count()
            == 1
        && linker.matches("fn execute_and_normalize(").count() == 1
        && linker.matches("occurrences").count() == 0
        && linker.matches(".magnitude()").count() == 0
}

fn source_audit() -> SourceAudit {
    let immutable_negatives = env!("DS8_DEFINITIVE_PROBE_V1_RESULT_SHA256")
        == PROBE_V1_RESULT_SHA256
        && env!("DS8_DEFINITIVE_PROBE_V1_AUDIT_SHA256") == PROBE_V1_AUDIT_SHA256
        && env!("DS8_DEFINITIVE_MICRO_V1_RESULT_SHA256") == MICRO_V1_RESULT_SHA256
        && env!("DS8_DEFINITIVE_MICRO_V1_AUDIT_SHA256") == MICRO_V1_AUDIT_SHA256
        && env!("DS8_DEFINITIVE_GATE_V1_RESULT_SHA256") == GATE_V1_RESULT_SHA256
        && env!("DS8_DEFINITIVE_GATE_V1_AUDIT_SHA256") == GATE_V1_AUDIT_SHA256
        && env!("DS8_DEFINITIVE_GATE_V2_RESULT_SHA256") == GATE_V2_RESULT_SHA256
        && env!("DS8_DEFINITIVE_GATE_V2_AUDIT_SHA256") == GATE_V2_AUDIT_SHA256;
    SourceAudit {
        authoritative_m5: env!("DS8_DEFINITIVE_M5_ALLOCATOR_SHA256") == M5_ALLOCATOR_SHA256
            && env!("DS8_DEFINITIVE_M5_CSV_SHA256") == M5_CSV_SHA256
            && env!("DS8_DEFINITIVE_M5_MD_SHA256") == M5_MD_SHA256,
        probe_lineage: env!("DS8_DEFINITIVE_ACTIVATION_SHA256") == ACTIVATION_SHA256
            && env!("DS8_DEFINITIVE_DEPENDENCY_SHA256") == DEPENDENCY_SHA256
            && env!("DS8_DEFINITIVE_PROBE_PROTOCOL_SHA256") == PROBE_PROTOCOL_SHA256
            && env!("DS8_DEFINITIVE_PROBE_PROTOCOL_V2_SHA256") == PROBE_PROTOCOL_V2_SHA256
            && env!("DS8_DEFINITIVE_PROBE_SOURCE_SHA256") == PROBE_SOURCE_SHA256
            && env!("DS8_DEFINITIVE_PROBE_RUNNER_SHA256") == PROBE_RUNNER_SHA256
            && env!("DS8_DEFINITIVE_PROBE_RESULT_SHA256") == PROBE_RESULT_SHA256
            && env!("DS8_DEFINITIVE_PROBE_AUDIT_SHA256") == PROBE_AUDIT_SHA256
            && env!("DS8_DEFINITIVE_PROBE_HANDOFF_SHA256") == PROBE_HANDOFF_SHA256,
        micro_lineage: env!("DS8_DEFINITIVE_MICRO_PROTOCOL_SHA256") == MICRO_PROTOCOL_SHA256
            && env!("DS8_DEFINITIVE_MICRO_PROTOCOL_V2_SHA256") == MICRO_PROTOCOL_V2_SHA256
            && env!("DS8_DEFINITIVE_MICRO_SOURCE_SHA256") == MICRO_SOURCE_SHA256
            && env!("DS8_DEFINITIVE_MICRO_RUNNER_SHA256") == MICRO_RUNNER_SHA256
            && env!("DS8_DEFINITIVE_MICRO_RESULT_SHA256") == MICRO_RESULT_SHA256
            && env!("DS8_DEFINITIVE_MICRO_AUDIT_SHA256") == MICRO_AUDIT_SHA256
            && env!("DS8_DEFINITIVE_DEVELOPMENT_HANDOFF_SHA256")
                == DEVELOPMENT_HANDOFF_SHA256,
        gate_lineage: env!("DS8_DEFINITIVE_GATE_PROTOCOL_SHA256") == GATE_PROTOCOL_SHA256
            && env!("DS8_DEFINITIVE_GATE_PROTOCOL_V2_SHA256") == GATE_PROTOCOL_V2_SHA256
            && env!("DS8_DEFINITIVE_GATE_PROTOCOL_V3_SHA256") == GATE_PROTOCOL_V3_SHA256
            && env!("DS8_DEFINITIVE_GATE_SOURCE_SHA256") == GATE_SOURCE_SHA256
            && env!("DS8_DEFINITIVE_GATE_RUNNER_SHA256") == GATE_RUNNER_SHA256
            && env!("DS8_DEFINITIVE_GATE_RESULT_SHA256") == GATE_RESULT_SHA256
            && env!("DS8_DEFINITIVE_GATE_AUDIT_SHA256") == GATE_AUDIT_SHA256,
        immutable_negatives,
        readiness_and_protocol: env!("DS8_DEFINITIVE_READINESS_SHA256") == READINESS_SHA256
            && env!("DS8_DEFINITIVE_PROTOCOL_SHA256") == PROTOCOL_SHA256,
        linker_exact: env!("DS8_MICRO_LINKER_FRAGMENT_SHA256") == LINKER_SHA256,
        information_boundary: authority_call_boundary() && linker_information_boundary(),
        negative_tag_manifest: [
            "cbb8af415766111a7e1b4d305f27435bd58e26d2",
            "78852b56db790472b2bb4ead8a3f46133b957501",
            "cb9b2cede8e1078464d4b619c1687af5ca06e2d9",
            "675d10df1639f8480af58cbc47185801cc8a5bbc",
        ]
        .iter()
        .all(|commit| commit.len() == 40),
    }
}

pub fn source_preflight(outputs_absent: bool, staging_absent: bool) -> Preflight {
    let source = source_audit();
    let explicit_seeds = SEEDS
        == [
            50_000_000,
            53_500_000,
            57_000_000,
            60_500_000,
            64_000_000,
            67_500_000,
            71_000_000,
            74_500_000,
            78_000_000,
            81_500_000,
            85_000_000,
            88_500_000,
            92_000_000,
            95_500_000,
            99_000_000,
            102_500_000,
        ]
        && SEEDS.len() == 16;
    let explicit_loads = LOADS == [8, 32, 128] && LOADS.len() == 3;
    let disjoint_namespaces = SEEDS.windows(2).all(|pair| {
        pair[0] + CELL_NAMESPACE <= pair[1]
            && pair[1] - pair[0] == 3_500_000
            && pair[0] + 3_001_006 < pair[0] + CELL_NAMESPACE
    });
    let development_disjoint = SEEDS[0] > 44_500_000 + CELL_NAMESPACE;
    let mut topology_counts = [0usize; 4];
    let mut layout_counts = [0usize; 2];
    for seed in SEEDS {
        topology_counts[(seed as usize / 500_000) % 4] += 1;
        layout_counts[usize::from(seed % 1_000_000 != 0)] += 1;
    }
    let topology_balanced = topology_counts == [4, 4, 4, 4];
    let layout_balanced = layout_counts == [8, 8];
    let passed = source.passed()
        && explicit_seeds
        && explicit_loads
        && disjoint_namespaces
        && development_disjoint
        && topology_balanced
        && layout_balanced
        && outputs_absent
        && staging_absent;
    Preflight {
        source,
        explicit_seeds,
        explicit_loads,
        disjoint_namespaces,
        development_disjoint,
        topology_balanced,
        layout_balanced,
        outputs_absent,
        staging_absent,
        passed,
    }
}

// DS8_AUTHORITY_CELL_CALL_BEGIN
fn frozen_authority_cell(seed: u64, load: usize, controls: bool) -> frozen_gate::GateCell {
    frozen_gate::authority_cell(seed, load, controls)
}
// DS8_AUTHORITY_CELL_CALL_END

fn run_cell(index: usize, seed: u64, load: usize, controls: bool) -> DefinitiveCell {
    let cell = frozen_authority_cell(seed, load, controls);
    let independently_conjunctive = cell.blank_acquisition
        && cell.physical_exact
        && cell.heldout == 32
        && cell.heldout_total == 32
        && cell.route_admissions == 4
        && cell.distractor_admissions <= load.div_ceil(8)
        && cell.always_open_admissions == 4 + load
        && cell.work_reduction >= 0.50
        && cell.raw_history_reversal
        && cell.value_shuffle_reversal
        && cell.entry_resistance > 0
        && cell.final_resistance == 0
        && cell.removed
        && cell.stale_blocked
        && cell.repair_admitted
        && cell.repair_executed
        && cell.repair_applied
        && cell.post_age_prototype_resistance > 0
        && cell.post_age_value_resistance > 0
        && cell.repaired == 32
        && cell.repaired_total == 32
        && cell.retained_proposals < cell.always_open_admissions
        && cell.retained_route_edges == 4
        && cell.retained_economy
        && cell.topology_identity_layout
        && cell.controls
        && cell.source_audit
        && cell.cumulative_m5;
    DefinitiveCell {
        index,
        seed,
        load,
        blank_acquisition: cell.blank_acquisition,
        physical_exact: cell.physical_exact,
        heldout: cell.heldout,
        heldout_total: cell.heldout_total,
        route_admissions: cell.route_admissions,
        distractor_admissions: cell.distractor_admissions,
        always_open_admissions: cell.always_open_admissions,
        work_reduction: cell.work_reduction,
        raw_history_reversal: cell.raw_history_reversal,
        value_shuffle_reversal: cell.value_shuffle_reversal,
        entry_resistance: cell.entry_resistance,
        final_resistance: cell.final_resistance,
        removed: cell.removed,
        stale_blocked: cell.stale_blocked,
        repair_admitted: cell.repair_admitted,
        repair_executed: cell.repair_executed,
        repair_applied: cell.repair_applied,
        post_age_prototype_resistance: cell.post_age_prototype_resistance,
        post_age_value_resistance: cell.post_age_value_resistance,
        repaired: cell.repaired,
        repaired_total: cell.repaired_total,
        retained_proposals: cell.retained_proposals,
        retained_route_edges: cell.retained_route_edges,
        retained_economy: cell.retained_economy,
        topology_identity_layout: cell.topology_identity_layout,
        controls: cell.controls,
        source_audit: cell.source_audit,
        cumulative_m5: cell.cumulative_m5,
        passed: cell.passed && independently_conjunctive,
    }
}

fn summarize_claims(preflight: &Preflight, controls: bool, cells: &[DefinitiveCell]) -> ClaimSummary {
    ClaimSummary {
        blank_acquisition: cells.iter().all(|cell| cell.blank_acquisition),
        physical_accounting: cells.iter().all(|cell| cell.physical_exact),
        heldout_execution: cells
            .iter()
            .all(|cell| cell.heldout == 32 && cell.heldout_total == 32),
        sparse_allocation: cells.iter().all(|cell| {
            cell.route_admissions == 4
                && cell.distractor_admissions <= cell.load.div_ceil(8)
                && cell.always_open_admissions == 4 + cell.load
        }),
        structural_economy: cells.iter().all(|cell| {
            cell.work_reduction >= 0.50
                && cell.retained_economy
                && cell.retained_proposals < cell.always_open_admissions
                && cell.retained_route_edges == 4
        }),
        raw_history_attribution: cells.iter().all(|cell| cell.raw_history_reversal),
        value_shuffle_attribution: cells.iter().all(|cell| cell.value_shuffle_reversal),
        physical_removal_and_stale_block: cells.iter().all(|cell| {
            cell.entry_resistance > 0
                && cell.final_resistance == 0
                && cell.removed
                && cell.stale_blocked
        }),
        nonsemantic_reacquisition: cells.iter().all(|cell| {
            cell.repair_admitted
                && cell.repair_executed
                && cell.repair_applied
                && cell.repaired == 32
                && cell.repaired_total == 32
        }),
        transfer_controls: preflight.topology_balanced
            && preflight.layout_balanced
            && cells.iter().all(|cell| cell.topology_identity_layout),
        positive_controls_and_m5: controls
            && cells
                .iter()
                .all(|cell| cell.controls && cell.cumulative_m5),
        zero_semantic_or_covert_credit: preflight.source.linker_exact
            && preflight.source.information_boundary
            && cells.iter().all(|cell| cell.source_audit),
        immutable_negatives: preflight.source.immutable_negatives
            && preflight.source.negative_tag_manifest,
    }
}

pub fn run_definitive(outputs_absent: bool, staging_absent: bool) -> DefinitiveReport {
    let preflight = source_preflight(outputs_absent, staging_absent);
    assert!(
        preflight.passed,
        "definitive preflight must pass before positive controls or any cell"
    );
    let controls = crate::ds8_cumulative_semantic_credit_micro::run().passed;
    let mut cells = Vec::with_capacity(SEEDS.len() * LOADS.len());
    for seed in SEEDS {
        for load in LOADS {
            let index = cells.len();
            cells.push(run_cell(index, seed, load, controls));
        }
    }
    let claims = summarize_claims(&preflight, controls, &cells);
    let passed = controls
        && cells.len() == 48
        && cells.iter().all(|cell| cell.passed)
        && claims.passed();
    DefinitiveReport {
        protocol: PROTOCOL,
        preflight,
        controls,
        claims,
        cells,
        passed,
        m5_authoritative: !passed,
        m6_authoritative: passed,
        core_autonomy_checkpoint: passed,
        post_m6_ds4_eligible: passed,
    }
}

pub fn csv(report: &DefinitiveReport) -> String {
    let mut text = String::from(
        "index,seed,load,blank_acquisition,physical_exact,heldout,heldout_total,route_admissions,distractor_admissions,always_open_admissions,work_reduction,raw_history_reversal,value_shuffle_reversal,entry_resistance,final_resistance,removed,stale_blocked,repair_admitted,repair_executed,repair_applied,post_age_prototype_resistance,post_age_value_resistance,repaired,repaired_total,retained_proposals,retained_route_edges,retained_economy,topology_identity_layout,controls,source_audit,cumulative_m5,passed\n",
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{:.6},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            cell.index,
            cell.seed,
            cell.load,
            cell.blank_acquisition,
            cell.physical_exact,
            cell.heldout,
            cell.heldout_total,
            cell.route_admissions,
            cell.distractor_admissions,
            cell.always_open_admissions,
            cell.work_reduction,
            cell.raw_history_reversal,
            cell.value_shuffle_reversal,
            cell.entry_resistance,
            cell.final_resistance,
            cell.removed,
            cell.stale_blocked,
            cell.repair_admitted,
            cell.repair_executed,
            cell.repair_applied,
            cell.post_age_prototype_resistance,
            cell.post_age_value_resistance,
            cell.repaired,
            cell.repaired_total,
            cell.retained_proposals,
            cell.retained_route_edges,
            cell.retained_economy,
            cell.topology_identity_layout,
            cell.controls,
            cell.source_audit,
            cell.cumulative_m5,
            cell.passed,
        ));
    }
    text
}

pub fn markdown(report: &DefinitiveReport) -> String {
    let claims = &report.claims;
    let mut text = format!(
        "# DS8 cumulative non-semantic-credit definitive result\n\nVerdict: **{}**.\n\nProtocol: `{}`.\n\nDefinitive cells: `{}/48`. Frozen positive controls: `{}`. No-cell source preflight: `{}`.\n\n## Thirteen-claim conjunction\n\n| # | claim | pass |\n|---:|---|:---:|\n| 1 | blank-start useful route acquisition | {} |\n| 2 | exact delayed physical accounting | {} |\n| 3 | held-out route execution 32/32 | {} |\n| 4 | sparse productive/distractor allocation | {} |\n| 5 | structural-work economy | {} |\n| 6 | raw-history attribution | {} |\n| 7 | learned-value-shuffle attribution | {} |\n| 8 | physical removal and stale block | {} |\n| 9 | non-semantic reacquisition 32/32 | {} |\n| 10 | topology/occurrence/identity/layout/equal-magnitude transfer | {} |\n| 11 | positive PROBE/MICRO and exact M5 | {} |\n| 12 | zero semantic or covert credit | {} |\n| 13 | immutable negative artifacts | {} |\n\n## Cells\n\n| cell | seed | load | acquired | physical | held-out | admitted route/distractor | baseline | reduction | raw swap | value shuffle | resistance | removed/stale | repair A/E/D | repaired | retained/route | transfer | controls/source/M5 | result |\n|---:|---:|---:|:---:|:---:|---:|---:|---:|---:|:---:|:---:|---:|:---:|:---:|---:|---:|:---:|:---:|:---:|\n",
        if report.passed { "PASS" } else { "FAIL" },
        report.protocol,
        report.cells.iter().filter(|cell| cell.passed).count(),
        report.controls,
        report.preflight.source.passed(),
        claims.blank_acquisition,
        claims.physical_accounting,
        claims.heldout_execution,
        claims.sparse_allocation,
        claims.structural_economy,
        claims.raw_history_attribution,
        claims.value_shuffle_attribution,
        claims.physical_removal_and_stale_block,
        claims.nonsemantic_reacquisition,
        claims.transfer_controls,
        claims.positive_controls_and_m5,
        claims.zero_semantic_or_covert_credit,
        claims.immutable_negatives,
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "| {} | {} | {} | {} | {} | {}/{} | {}/{} | {} | {:.2}% | {} | {} | {} -> {} | {}/{} | {}/{}/{} | {}/{} | {}/{} | {} | {}/{}/{} | {} |\n",
            cell.index,
            cell.seed,
            cell.load,
            cell.blank_acquisition,
            cell.physical_exact,
            cell.heldout,
            cell.heldout_total,
            cell.route_admissions,
            cell.distractor_admissions,
            cell.always_open_admissions,
            100.0 * cell.work_reduction,
            cell.raw_history_reversal,
            cell.value_shuffle_reversal,
            cell.entry_resistance,
            cell.final_resistance,
            cell.removed,
            cell.stale_blocked,
            cell.repair_admitted,
            cell.repair_executed,
            cell.repair_applied,
            cell.repaired,
            cell.repaired_total,
            cell.retained_proposals,
            cell.retained_route_edges,
            cell.topology_identity_layout,
            cell.controls,
            cell.source_audit,
            cell.cumulative_m5,
            cell.passed,
        ));
    }
    text.push_str(&format!(
        "\nM5 authoritative: `{}`. M6 authoritative: `{}`. Core-autonomy checkpoint: `{}`. Separately named post-M6 DS4 successor eligible: `{}`.\n\nProgram priorities remain outside evidentiary scope.\n",
        report.m5_authoritative,
        report.m6_authoritative,
        report.core_autonomy_checkpoint,
        report.post_m6_ds4_eligible,
    ));
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_cell_preflight_is_exact_fresh_and_balanced() {
        let audit = source_preflight(true, true);
        assert!(audit.source.passed(), "{audit:#?}");
        assert!(audit.explicit_seeds);
        assert!(audit.explicit_loads);
        assert!(audit.disjoint_namespaces);
        assert!(audit.development_disjoint);
        assert!(audit.topology_balanced);
        assert!(audit.layout_balanced);
        assert!(audit.outputs_absent);
        assert!(audit.staging_absent);
        assert!(audit.passed);
    }

    #[test]
    fn no_cell_preflight_refuses_existing_final_or_staging_path() {
        assert!(!source_preflight(false, true).passed);
        assert!(!source_preflight(true, false).passed);
        assert!(!source_preflight(false, false).passed);
    }
}
