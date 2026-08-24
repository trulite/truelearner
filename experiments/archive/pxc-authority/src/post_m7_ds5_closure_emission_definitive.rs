//! Write-once definitive authority over the byte-frozen post-M7 DS5 mechanism.

pub const PROTOCOL: &str = "post-m7-ds5-closure-emission-definitive-v1";
pub const PROTOCOL_COMMIT: &str = "11b885c375a723fe08443f0078a261c73af9cd74";
pub const READINESS_COMMIT: &str = "0bc7def607a7e1c576d6d07dc97cf25dc9e77446";
pub const MECHANISM_COMMIT: &str = "70768779f0018102f181c892b0753c690ce4225e";
pub const AUTHORITATIVE_M7: &str = "b607ed52f640a3e202da3cc6b73ac58b180caf83";

pub const SEEDS: [u64; 16] = [
    1_000_123_457,
    1_010_123_457,
    1_020_123_457,
    1_030_123_457,
    1_040_123_457,
    1_050_123_457,
    1_060_123_457,
    1_070_123_457,
    1_080_123_457,
    1_090_123_457,
    1_100_123_457,
    1_110_123_457,
    1_120_123_457,
    1_130_123_457,
    1_140_123_457,
    1_150_123_457,
];
pub const HELD_OUT_PER_CELL: usize = 64;
pub const CELL_NAMESPACE: u64 = 8_000_000;
const MAX_DIRECT_DERIVED_OFFSET: u64 = 7_100_000;

const PROTOCOL_SHA256: &str = "4b48e347fcad07df2323d7044c71af01e74db58d988e2fead2bec658badc54cc";
const MECHANISM_SHA256: &str = "6a8590b904403dfa880198f7acf8daf843864dee1a2ea0230d964d928f4076d1";
const DEVELOPMENT_RUNNER_SHA256: &str =
    "20955a945984f04c57e2abc97254d19dcc1c0de829acf19450dcf163d69de213";
const DEVELOPMENT_PROTOCOL_SHA256: &str =
    "140d2392263359c666f364e1923f956a4b2b09e4107a0fd2f7f8f469d97be154";
const PROBE_V1_SHA256: &str = "1e2faea63db4b165a4e35b3f9fdccccb373eecbe4b5c173102612593cf4716c4";
const PROBE_RETRY_SHA256: &str = "c947e50e77fb56d696a53eb1b4f125f5710405fc708e1071d0056579cb52a085";
const MICRO_SHA256: &str = "21c716c87b364e4611d11773d0ff4a914e0d19325ce3b90084be146d8c891e2c";
const GATE_V1_SHA256: &str = "99f228f32a86c3f48b36d3e10a908955152ca7e8aa826e78c603e537676db876";
const GATE_V2_SHA256: &str = "6d5b1a66d5064f80237a58fedd8aca35688a46b70f1879e4f09c6a34fd88e028";
const READINESS_SHA256: &str = "c470736b6be4fea5e3081f403e7a1ebb89a3e6104f06e652719dd8d4df5db013";

const M7_SOURCE_SHA256: &str = "67e170f12d7b7649a0a291ddfc16cd80e4b5c15564b65cd09c884f3e52b9ac5b";
const V20_SHA256: &str = "8a17e7a5fda9519ad0d4a9d29d04d2434dd5b5ee857e74c1296c5f8b3b06f897";
const V21_SHA256: &str = "85230e7b6b0d669a3b2e163f3e281975c9fbd5d98709b923efff418d36ff9f1a";
const M3_PORT_SHA256: &str = "c4fc7aca11a5925effeb5a84b90184a70da0f66da7c063d0f87ba46ca36addf3";
const M3_FIXTURE_SHA256: &str = "b65b28256d58c184b41bf2ff8d383c99593e6d812480751684209dce1d82f99a";
const P4_SHA256: &str = "2dbde723b394bcb3d788c796aa1745cd1cea392a64ab61497bb97474866144b8";
const M4_SHA256: &str = "3d5659fb26ae804dee6122408f9d703ea1f226349772883075a42686ac3fd110";
const M5_SHA256: &str = "e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7";
const M6_SHA256: &str = "11b4229122b3e0788ca30c55579b91ffe07461de9a138860690134565fcf2ed6";
const BOUNDARY_SHA256: &str = "3eb802f394a225a4ad7f0938b4a672723da2c1303ff95e805423de8161057527";
const CLOSURE_SHA256: &str = "860e89304e86f254dd02a5aa35cf63cc240af160039b4166fa0cb5856dacb84a";
const RETURN_SHA256: &str = "f17afa482bf345eb680463f7418b6b6c2553cd78eab9b4dbfce74f7ca1483d51";
const V20_RESULT_SHA256: &str = "468d50db53ed7451f2680621b85067a046fccba3c8ef19097dabaec22a3806b4";
const V21A_RESULT_SHA256: &str = "0f4a30c378aba506492351588412aab71ba6c174ef358d2d0758c82ec87cfc20";
const V21B_RESULT_SHA256: &str = "ca4f2ffb8b77ac237bfce19d66d21820d26d34b727bbf95262003dffd93ad300";

const AUTHORITY_ARTIFACT_HASHES: [(&str, &str); 17] = [
    (
        env!("POST_M7_DS5_DEFINITIVE_M0_CSV_SHA256"),
        "7883f71918d48c4c622d7cd2d9dd7561f5954f7287f8bc6abb535f5a9f994a55",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M0_MD_SHA256"),
        "a788106462498dd7581fcbd324d6fbc71a1ca0a46c3390a4d289ae180731edad",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M1_CSV_SHA256"),
        "fede145a50bc059ffcd19a26dc65763843a83b1644c89bd44a3b27e8cd7cea27",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M1_MD_SHA256"),
        "5971c78eb2688e9fb2d31e59b8d835000ece4923b84aef57d6a1ef48f9295bea",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M2_CSV_SHA256"),
        "68d6dd31ca15e206b382f3ef6592804882eecfe09efd6696b8ed403dc6304159",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M2_MD_SHA256"),
        "a67f6d0acc2dc01a8456922153b64b97ad2493f2e7bf50b4d5268533a134f2e1",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M3_CSV_SHA256"),
        "ac8c0a6c9b7badfa263ceb054ffe59c11162b1ca256c56cc6df5f0d378179401",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M3_MD_SHA256"),
        "ab77bd12b705b8620b6315260f8bb5b4df6efc961f1d20a0dd521af403e1ac5f",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M4_CSV_SHA256"),
        "5c4a2e2b021a26a4cc2161202dd9a62205d426ba361f90a69d00ceb3df470a83",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M4_MD_SHA256"),
        "c418f6b5fb5f8f3f83e385c75cd23fb8c88def3650ff80a13a491d0979944768",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M5_CSV_SHA256"),
        "86d9f6e3a8ab4ad5c242e0d7c619d8eda99e0da47faff623f26c8c6835b9a99a",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M5_MD_SHA256"),
        "a336633c73565261d357a67ca02df3047ffcaf88488153bb2f43b621818ba5f0",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M6_CSV_SHA256"),
        "0cb9ba779fca1899cf030d30358fe9354cfb7b2cccf87f32df3f6ea9ddfe91e4",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M6_MD_SHA256"),
        "6a5d938c3e021344b00f3a559593fee860b5f6cceb777c409ad8d59a2dd71872",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M7_CSV_SHA256"),
        "13619c786471b34f5dc9da914c4a0f454bab8d95a87142ce6c9e35808f3dd91a",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M7_MD_SHA256"),
        "d1f4d3dc6c944b8ab146a121b0fb0df7d6270b3d4363ca6d4e18b8b53925b1cd",
    ),
    (
        env!("POST_M7_DS5_DEFINITIVE_M7_HANDOFF_SHA256"),
        "b4a9012f8fbbb1fa8fdfd36921a82e162c73f4c2175c809bd48c0dae78e45520",
    ),
];

#[allow(dead_code)]
mod frozen_development {
    include!(concat!(
        env!("OUT_DIR"),
        "/post_m7_ds5_closure_emission_frozen.rs"
    ));

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct AuthorityObservation {
        pub(super) source_passed: bool,
        pub(super) information_boundary: bool,
        pub(super) lane_isolated: bool,
        pub(super) learners: usize,
        pub(super) ready: usize,
        pub(super) single_roles: usize,
        pub(super) competence: Vec<usize>,
        pub(super) m7_competence: Vec<usize>,
        pub(super) m7_activations: usize,
        pub(super) selections: usize,
        pub(super) consequences: usize,
        pub(super) updates: usize,
        pub(super) observations: u64,
        pub(super) crossings: usize,
        pub(super) held_out_correct: usize,
        pub(super) held_out_total: usize,
        pub(super) quiescent: usize,
        pub(super) positions: usize,
        pub(super) depths: usize,
        pub(super) route_handles: usize,
        pub(super) allocation_layouts: usize,
        pub(super) physical_work: u64,
        pub(super) m7_nonplastic: bool,
        pub(super) closure_nonplastic: bool,
        pub(super) temporary_erased: bool,
        pub(super) namespaces_disjoint: bool,
        pub(super) controls: [bool; 12],
    }

    pub(super) fn authority_source_audit() -> (bool, bool, bool) {
        let audit = source_audit();
        (
            audit.passed(),
            audit.information_boundary,
            audit.lane_isolated,
        )
    }

    pub(super) fn authority_observation(
        seed: u64,
        held_out_per_learner: usize,
    ) -> AuthorityObservation {
        let observed = snapshot(&[seed], held_out_per_learner);
        let mut controls = [false; 12];
        if observed.controls.len() == controls.len() {
            for (target, control) in controls.iter_mut().zip(&observed.controls) {
                *target = control.passed;
            }
        }
        AuthorityObservation {
            source_passed: observed.source.passed(),
            information_boundary: observed.source.information_boundary,
            lane_isolated: observed.source.lane_isolated,
            learners: observed.learners,
            ready: observed.ready,
            single_roles: observed.single_roles,
            competence: observed.competence,
            m7_competence: observed.m7_competence,
            m7_activations: observed.m7_activations,
            selections: observed.selections,
            consequences: observed.consequences,
            updates: observed.updates,
            observations: observed.observations,
            crossings: observed.crossings,
            held_out_correct: observed.correct,
            held_out_total: observed.total,
            quiescent: observed.quiescent,
            positions: observed.positions.len(),
            depths: observed.depths.len(),
            route_handles: observed.route_handles.len(),
            allocation_layouts: observed.allocation_layouts.len(),
            physical_work: observed.physical_work,
            m7_nonplastic: observed.m7_nonplastic,
            closure_nonplastic: observed.closure_nonplastic,
            temporary_erased: observed.temporary_erased,
            namespaces_disjoint: observed.namespaces.is_disjoint(&observed.held_out),
            controls,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub exact_frozen_mechanism: bool,
    pub exact_development_lineage: bool,
    pub exact_reused_parts: bool,
    pub exact_m0_m7_artifacts: bool,
    pub source_order_and_information_flow: bool,
    pub lane_b_isolated: bool,
    pub protocol_and_commits: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.exact_frozen_mechanism
            && self.exact_development_lineage
            && self.exact_reused_parts
            && self.exact_m0_m7_artifacts
            && self.source_order_and_information_flow
            && self.lane_b_isolated
            && self.protocol_and_commits
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Preflight {
    pub source: SourceAudit,
    pub explicit_matrix: bool,
    pub derived_namespaces_disjoint: bool,
    pub prior_namespaces_disjoint: bool,
    pub held_out_fixed: bool,
    pub outputs_absent: bool,
    pub staging_absent: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefinitiveCell {
    pub index: usize,
    pub seed: u64,
    pub competence_episode: usize,
    pub m7_competence_episode: usize,
    pub m7_activations: usize,
    pub selections: usize,
    pub consequences: usize,
    pub updates: usize,
    pub m6_observations: u64,
    pub crossings: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub quiescent: usize,
    pub positions: usize,
    pub depths: usize,
    pub route_handles: usize,
    pub allocation_layouts: usize,
    pub physical_work: u64,
    pub controls_passed: usize,
    pub p0_source_authority: bool,
    pub p1_initiation_structural_closure: bool,
    pub p2_single_closure_role: bool,
    pub p3_exact_current_crossing: bool,
    pub p4_quiescent_lifecycle: bool,
    pub p5_transfer: bool,
    pub p6_controls: bool,
    pub p7_duplicate_exact: bool,
    pub first_collapse: &'static str,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimSummary {
    pub blank_single_role_acquisition: bool,
    pub frozen_m7_initiation_required: bool,
    pub successor_prevents_premature_crossing: bool,
    pub closure_activates_without_semantic_metadata: bool,
    pub current_value_crosses_exactly_once: bool,
    pub natural_queue_drain_without_cutoff: bool,
    pub invalid_incomplete_blocked_paths_silent: bool,
    pub branch_cycle_cannot_complete: bool,
    pub m6_delayed_history_controls: bool,
    pub duplicate_zero_hop_and_finite_structure: bool,
    pub transfer_and_blank_start: bool,
    pub held_out_nonplastic_and_erased: bool,
    pub frozen_evaluator_boundary_and_lane_isolation: bool,
}

impl ClaimSummary {
    pub fn passed(&self) -> bool {
        self.blank_single_role_acquisition
            && self.frozen_m7_initiation_required
            && self.successor_prevents_premature_crossing
            && self.closure_activates_without_semantic_metadata
            && self.current_value_crosses_exactly_once
            && self.natural_queue_drain_without_cutoff
            && self.invalid_incomplete_blocked_paths_silent
            && self.branch_cycle_cannot_complete
            && self.m6_delayed_history_controls
            && self.duplicate_zero_hop_and_finite_structure
            && self.transfer_and_blank_start
            && self.held_out_nonplastic_and_erased
            && self.frozen_evaluator_boundary_and_lane_isolation
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefinitiveReport {
    pub protocol: &'static str,
    pub preflight: Preflight,
    pub claims: ClaimSummary,
    pub cells: Vec<DefinitiveCell>,
    pub passed: bool,
    pub m7_authoritative: bool,
    pub m8_authoritative: bool,
    pub successor_definitive_positive: bool,
    pub cognitive_desupply_complete: bool,
    pub ds9_created: bool,
}

fn marked<'a>(source: &'a str, begin: &str, end: &str) -> &'a str {
    source
        .split(begin)
        .nth(1)
        .and_then(|tail| tail.split(end).next())
        .unwrap_or_default()
}

fn marked_last<'a>(source: &'a str, begin: &str, end: &str) -> &'a str {
    source
        .rsplit(begin)
        .next()
        .and_then(|tail| tail.split(end).next())
        .unwrap_or_default()
}

fn source_order_and_information_flow() -> bool {
    let mechanism = include_str!("post_m7_ds5_closure_emission.rs");
    let linker = marked(
        mechanism,
        "// POST_M7_DS5_M6_GATE_BEGIN",
        "// POST_M7_DS5_M6_GATE_END",
    );
    let execute = mechanism
        .split("fn execute(")
        .nth(1)
        .and_then(|tail| tail.split("pub struct SourceAudit").next())
        .unwrap_or_default();
    let evaluator = mechanism
        .split("let run = execute(")
        .nth(2)
        .unwrap_or_default();
    let authority = include_str!("post_m7_ds5_closure_emission_definitive.rs");
    let authority_call = marked_last(
        authority,
        "// POST_M7_DS5_AUTHORITY_CELL_CALL_BEGIN",
        "// POST_M7_DS5_AUTHORITY_CELL_CALL_END",
    );
    let forbidden = [
        ["expected", "_answer"].concat(),
        ["episode", ".terminal"].concat(),
        ["semantic", "_credit"].concat(),
        ["finish", "_route"].concat(),
        ["no_", "result"].concat(),
        ["answer", "_identity"].concat(),
        ["remaining", "_steps"].concat(),
        ["target", "_depth"].concat(),
        ["rew", "ard"].concat(),
        ["lo", "ss"].concat(),
    ];
    let linker_clean = forbidden.iter().all(|word| !linker.contains(word));
    let authority_clean = forbidden.iter().all(|word| !authority_call.contains(word));
    let consequence = linker.find("let consequence = acquire && connected");
    let differential = linker.find("frozen_cumulative::differential");
    let feedback = linker.find("session.learner.feedback");
    let zero_branch = execute.find("0 =>");
    let successor_branch = execute.find("1 =>");
    let crossing = execute.find("step.crossing");
    let comparison = evaluator.find("run.crossings == [episode.terminal]");
    linker_clean
        && authority_clean
        && linker.matches("frozen_cumulative::differential").count() == 1
        && linker.matches("session.learner.feedback").count() == 1
        && matches!(
            (consequence, differential, feedback),
            (Some(consequence), Some(differential), Some(feedback))
                if consequence < differential && differential < feedback
        )
        && matches!(
            (zero_branch, crossing, successor_branch),
            (Some(zero), Some(crossing), Some(successor))
                if zero < crossing && crossing < successor
        )
        && comparison.is_some()
        && authority_call
            .matches("frozen_development::authority_observation")
            .count()
            == 1
        && authority_call
            .contains("frozen_development::authority_observation(seed, HELD_OUT_PER_CELL)")
}

fn exact_reused_parts() -> bool {
    [
        (
            env!("POST_M7_DS5_DEFINITIVE_M7_SOURCE_SHA256"),
            M7_SOURCE_SHA256,
        ),
        (env!("POST_M7_DS5_DEFINITIVE_V20_SHA256"), V20_SHA256),
        (env!("POST_M7_DS5_DEFINITIVE_V21_SHA256"), V21_SHA256),
        (
            env!("POST_M7_DS5_DEFINITIVE_M3_PORT_SHA256"),
            M3_PORT_SHA256,
        ),
        (
            env!("POST_M7_DS5_DEFINITIVE_M3_FIXTURE_SHA256"),
            M3_FIXTURE_SHA256,
        ),
        (env!("POST_M7_DS5_DEFINITIVE_P4_SHA256"), P4_SHA256),
        (env!("POST_M7_DS5_DEFINITIVE_M4_SHA256"), M4_SHA256),
        (env!("POST_M7_DS5_DEFINITIVE_M5_SHA256"), M5_SHA256),
        (env!("POST_M7_DS5_DEFINITIVE_M6_SHA256"), M6_SHA256),
        (
            env!("POST_M7_DS5_DEFINITIVE_BOUNDARY_SHA256"),
            BOUNDARY_SHA256,
        ),
        (
            env!("POST_M7_DS5_DEFINITIVE_CLOSURE_SHA256"),
            CLOSURE_SHA256,
        ),
        (env!("POST_M7_DS5_DEFINITIVE_RETURN_SHA256"), RETURN_SHA256),
        (
            env!("POST_M7_DS5_DEFINITIVE_V20_RESULT_SHA256"),
            V20_RESULT_SHA256,
        ),
        (
            env!("POST_M7_DS5_DEFINITIVE_V21A_RESULT_SHA256"),
            V21A_RESULT_SHA256,
        ),
        (
            env!("POST_M7_DS5_DEFINITIVE_V21B_RESULT_SHA256"),
            V21B_RESULT_SHA256,
        ),
    ]
    .iter()
    .all(|(observed, expected)| observed == expected)
}

fn source_audit() -> SourceAudit {
    let frozen = frozen_development::authority_source_audit();
    SourceAudit {
        exact_frozen_mechanism: env!("POST_M7_DS5_DEFINITIVE_MECHANISM_SHA256") == MECHANISM_SHA256
            && frozen.0,
        exact_development_lineage: env!("POST_M7_DS5_DEFINITIVE_DEVELOPMENT_RUNNER_SHA256")
            == DEVELOPMENT_RUNNER_SHA256
            && env!("POST_M7_DS5_DEFINITIVE_DEVELOPMENT_PROTOCOL_SHA256")
                == DEVELOPMENT_PROTOCOL_SHA256
            && env!("POST_M7_DS5_DEFINITIVE_PROBE_V1_SHA256") == PROBE_V1_SHA256
            && env!("POST_M7_DS5_DEFINITIVE_PROBE_RETRY_SHA256") == PROBE_RETRY_SHA256
            && env!("POST_M7_DS5_DEFINITIVE_MICRO_SHA256") == MICRO_SHA256
            && env!("POST_M7_DS5_DEFINITIVE_GATE_V1_SHA256") == GATE_V1_SHA256
            && env!("POST_M7_DS5_DEFINITIVE_GATE_V2_SHA256") == GATE_V2_SHA256
            && env!("POST_M7_DS5_DEFINITIVE_READINESS_SHA256") == READINESS_SHA256,
        exact_reused_parts: exact_reused_parts(),
        exact_m0_m7_artifacts: AUTHORITY_ARTIFACT_HASHES
            .iter()
            .all(|(observed, expected)| observed == expected),
        source_order_and_information_flow: source_order_and_information_flow() && frozen.1,
        lane_b_isolated: frozen.2,
        protocol_and_commits: env!("POST_M7_DS5_DEFINITIVE_PROTOCOL_SHA256") == PROTOCOL_SHA256
            && PROTOCOL_COMMIT == "11b885c375a723fe08443f0078a261c73af9cd74"
            && READINESS_COMMIT == "0bc7def607a7e1c576d6d07dc97cf25dc9e77446"
            && MECHANISM_COMMIT == "70768779f0018102f181c892b0753c690ce4225e"
            && AUTHORITATIVE_M7 == "b607ed52f640a3e202da3cc6b73ac58b180caf83",
    }
}

pub fn source_preflight(outputs_absent: bool, staging_absent: bool) -> Preflight {
    let source = source_audit();
    let explicit_matrix = SEEDS
        == [
            1_000_123_457,
            1_010_123_457,
            1_020_123_457,
            1_030_123_457,
            1_040_123_457,
            1_050_123_457,
            1_060_123_457,
            1_070_123_457,
            1_080_123_457,
            1_090_123_457,
            1_100_123_457,
            1_110_123_457,
            1_120_123_457,
            1_130_123_457,
            1_140_123_457,
            1_150_123_457,
        ]
        && SEEDS.len() == 16;
    let derived_namespaces_disjoint = MAX_DIRECT_DERIVED_OFFSET < CELL_NAMESPACE
        && SEEDS
            .windows(2)
            .all(|pair| pair[0] + CELL_NAMESPACE <= pair[1] && pair[1] - pair[0] == 10_000_000);
    let prior_namespaces_disjoint = SEEDS[0] > 894_000_000;
    let held_out_fixed = HELD_OUT_PER_CELL == 64;
    let passed = source.passed()
        && explicit_matrix
        && derived_namespaces_disjoint
        && prior_namespaces_disjoint
        && held_out_fixed
        && outputs_absent
        && staging_absent;
    Preflight {
        source,
        explicit_matrix,
        derived_namespaces_disjoint,
        prior_namespaces_disjoint,
        held_out_fixed,
        outputs_absent,
        staging_absent,
        passed,
    }
}

// POST_M7_DS5_AUTHORITY_CELL_CALL_BEGIN
fn frozen_authority_cell(seed: u64) -> frozen_development::AuthorityObservation {
    frozen_development::authority_observation(seed, HELD_OUT_PER_CELL)
}
// POST_M7_DS5_AUTHORITY_CELL_CALL_END

fn run_cell(index: usize, seed: u64, source: &SourceAudit) -> DefinitiveCell {
    let first = frozen_authority_cell(seed);
    let second = frozen_authority_cell(seed);
    let duplicate_exact = first == second;
    let mut controls = first.controls;
    controls[2] &= controls[0] && source.source_order_and_information_flow;
    controls[8] &= first.positions == 6
        && first.depths == 7
        && first.route_handles == HELD_OUT_PER_CELL
        && first.allocation_layouts == 2;
    controls[9] &= source.source_order_and_information_flow;
    controls[10] &= first.m7_nonplastic
        && first.closure_nonplastic
        && first.temporary_erased
        && first.namespaces_disjoint;
    controls[11] &=
        source.exact_m0_m7_artifacts && source.exact_development_lineage && source.lane_b_isolated;

    let p0 =
        source.passed() && first.source_passed && first.information_boundary && first.lane_isolated;
    let p1 = first.m7_activations > 0 && controls[0] && controls[1];
    let p2 = first.learners == 1
        && first.ready == 1
        && first.single_roles == 1
        && first.competence.len() == 1
        && first.competence[0] <= 4_000
        && first.m7_competence.len() == 1
        && first.m7_competence[0] <= 4_000
        && first.selections > 0
        && first.consequences > 0
        && first.updates > 0
        && first.observations > 0;
    let p3 = first.held_out_total == HELD_OUT_PER_CELL
        && first.held_out_correct == HELD_OUT_PER_CELL
        && first.route_handles == HELD_OUT_PER_CELL;
    let p4 = first.quiescent == HELD_OUT_PER_CELL
        && first.physical_work > 0
        && first.m7_nonplastic
        && first.closure_nonplastic
        && first.temporary_erased
        && first.namespaces_disjoint;
    let p5 = first.positions == 6
        && first.depths == 7
        && first.allocation_layouts == 2
        && controls[6]
        && controls[7]
        && controls[8];
    let p6 = controls.iter().all(|passed| *passed);
    let p7 = duplicate_exact;
    let stages = [p0, p1, p2, p3, p4, p5, p6, p7];
    let first_collapse = stages
        .iter()
        .position(|passed| !passed)
        .map_or("NONE", |stage| {
            [
                "P0 source and authority",
                "P1 learned initiation and structural closure",
                "P2 one anonymous closure-emission role",
                "P3 exact current-value boundary crossing",
                "P4 natural quiescence and lifecycle",
                "P5 identity/depth/layout/position transfer",
                "P6 twelve controls",
                "P7 exact duplicate blank-start execution",
            ][stage]
        });
    DefinitiveCell {
        index,
        seed,
        competence_episode: first.competence.first().copied().unwrap_or(0),
        m7_competence_episode: first.m7_competence.first().copied().unwrap_or(0),
        m7_activations: first.m7_activations,
        selections: first.selections,
        consequences: first.consequences,
        updates: first.updates,
        m6_observations: first.observations,
        crossings: first.crossings,
        held_out_correct: first.held_out_correct,
        held_out_total: first.held_out_total,
        quiescent: first.quiescent,
        positions: first.positions,
        depths: first.depths,
        route_handles: first.route_handles,
        allocation_layouts: first.allocation_layouts,
        physical_work: first.physical_work,
        controls_passed: controls.iter().filter(|passed| **passed).count(),
        p0_source_authority: p0,
        p1_initiation_structural_closure: p1,
        p2_single_closure_role: p2,
        p3_exact_current_crossing: p3,
        p4_quiescent_lifecycle: p4,
        p5_transfer: p5,
        p6_controls: p6,
        p7_duplicate_exact: p7,
        first_collapse,
        passed: stages.iter().all(|passed| *passed),
    }
}

fn summarize(preflight: &Preflight, cells: &[DefinitiveCell]) -> ClaimSummary {
    ClaimSummary {
        blank_single_role_acquisition: cells.iter().all(|cell| cell.p2_single_closure_role),
        frozen_m7_initiation_required: cells
            .iter()
            .all(|cell| cell.p1_initiation_structural_closure && cell.m7_activations > 0),
        successor_prevents_premature_crossing: cells
            .iter()
            .all(|cell| cell.p1_initiation_structural_closure && cell.controls_passed == 12),
        closure_activates_without_semantic_metadata: cells
            .iter()
            .all(|cell| cell.p2_single_closure_role)
            && preflight.source.source_order_and_information_flow,
        current_value_crosses_exactly_once: cells.iter().all(|cell| cell.p3_exact_current_crossing),
        natural_queue_drain_without_cutoff: cells.iter().all(|cell| cell.p4_quiescent_lifecycle),
        invalid_incomplete_blocked_paths_silent: cells
            .iter()
            .all(|cell| cell.p6_controls && cell.controls_passed == 12),
        branch_cycle_cannot_complete: cells
            .iter()
            .all(|cell| cell.p6_controls && cell.controls_passed == 12),
        m6_delayed_history_controls: cells
            .iter()
            .all(|cell| cell.p2_single_closure_role && cell.p6_controls),
        duplicate_zero_hop_and_finite_structure: cells
            .iter()
            .all(|cell| cell.p5_transfer && cell.p7_duplicate_exact),
        transfer_and_blank_start: cells.iter().all(|cell| cell.p5_transfer),
        held_out_nonplastic_and_erased: cells.iter().all(|cell| cell.p4_quiescent_lifecycle),
        frozen_evaluator_boundary_and_lane_isolation: preflight.source.passed(),
    }
}

pub fn run_definitive(outputs_absent: bool, staging_absent: bool) -> DefinitiveReport {
    let preflight = source_preflight(outputs_absent, staging_absent);
    assert!(
        preflight.passed,
        "definitive preflight must pass before any learner or cell"
    );
    eprintln!("POST_M7_DS5_DEFINITIVE_EVIDENCE_SPENT");
    let mut cells = Vec::with_capacity(SEEDS.len());
    for seed in SEEDS {
        let index = cells.len();
        cells.push(run_cell(index, seed, &preflight.source));
    }
    let claims = summarize(&preflight, &cells);
    let passed = cells.len() == 16
        && cells.iter().all(|cell| cell.passed)
        && cells
            .iter()
            .map(|cell| cell.held_out_correct)
            .sum::<usize>()
            == 1_024
        && cells.iter().map(|cell| cell.quiescent).sum::<usize>() == 1_024
        && cells.iter().map(|cell| cell.route_handles).sum::<usize>() == 1_024
        && cells.iter().map(|cell| cell.controls_passed).sum::<usize>() == 192
        && claims.passed();
    DefinitiveReport {
        protocol: PROTOCOL,
        preflight,
        claims,
        cells,
        passed,
        m7_authoritative: !passed,
        m8_authoritative: passed,
        successor_definitive_positive: passed,
        cognitive_desupply_complete: passed,
        ds9_created: false,
    }
}

pub fn csv(report: &DefinitiveReport) -> String {
    let mut text = String::from(
        "index,seed,competence_episode,m7_competence_episode,m7_activations,selections,consequences,updates,m6_observations,crossings,held_out_correct,held_out_total,quiescent,positions,depths,route_handles,allocation_layouts,physical_work,controls_passed,p0_source_authority,p1_initiation_structural_closure,p2_single_closure_role,p3_exact_current_crossing,p4_quiescent_lifecycle,p5_transfer,p6_controls,p7_duplicate_exact,first_collapse,passed\n",
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            cell.index,
            cell.seed,
            cell.competence_episode,
            cell.m7_competence_episode,
            cell.m7_activations,
            cell.selections,
            cell.consequences,
            cell.updates,
            cell.m6_observations,
            cell.crossings,
            cell.held_out_correct,
            cell.held_out_total,
            cell.quiescent,
            cell.positions,
            cell.depths,
            cell.route_handles,
            cell.allocation_layouts,
            cell.physical_work,
            cell.controls_passed,
            cell.p0_source_authority,
            cell.p1_initiation_structural_closure,
            cell.p2_single_closure_role,
            cell.p3_exact_current_crossing,
            cell.p4_quiescent_lifecycle,
            cell.p5_transfer,
            cell.p6_controls,
            cell.p7_duplicate_exact,
            cell.first_collapse,
            cell.passed,
        ));
    }
    text
}

pub fn markdown(report: &DefinitiveReport) -> String {
    let claims = &report.claims;
    let mut text = format!(
        "# Post-M7 DS5 closure-emission definitive result\n\nVerdict: **{}**.\n\nProtocol: `{}`. Definitive cells: `{}/16`. Held-out exact current values / natural quiescence / route handles: `{}/{}/{}` of `1024`. Controls: `{}/192`.\n\n## Definitive conjunction\n\n| claim | pass |\n|---|:---:|\n| blank learners acquire exactly one anonymous closure-emission role | {} |\n| frozen M7 initiation is required | {} |\n| successor activity prevents premature crossing | {} |\n| structural closure activates without semantic finality metadata | {} |\n| current local value crosses exactly once | {} |\n| ordinary queue drains naturally without cutoff | {} |\n| invalid, incomplete, stale, and blocked paths are silent | {} |\n| branch ambiguity and cycles cannot falsely complete | {} |\n| frozen M6 delayed-history controls hold | {} |\n| duplicate, zero-hop, and finite chains follow structure | {} |\n| identity/depth/layout/allocation/position transfer and blank starts hold | {} |\n| held-out state is non-plastic and temporary state erased | {} |\n| frozen evaluator boundary and Lane-B isolation hold | {} |\n\n## Cells\n\n| cell | seed | competence M7/closure | M7 activation | selection | consequence/update/observation | held-out | quiescent | positions/depths/allocations | handles | controls | P0..P7 | first collapse | result |\n|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|:---:|\n",
        if report.passed { "PASS" } else { "FAIL" },
        report.protocol,
        report.cells.iter().filter(|cell| cell.passed).count(),
        report.cells.iter().map(|cell| cell.held_out_correct).sum::<usize>(),
        report.cells.iter().map(|cell| cell.quiescent).sum::<usize>(),
        report.cells.iter().map(|cell| cell.route_handles).sum::<usize>(),
        report.cells.iter().map(|cell| cell.controls_passed).sum::<usize>(),
        claims.blank_single_role_acquisition,
        claims.frozen_m7_initiation_required,
        claims.successor_prevents_premature_crossing,
        claims.closure_activates_without_semantic_metadata,
        claims.current_value_crosses_exactly_once,
        claims.natural_queue_drain_without_cutoff,
        claims.invalid_incomplete_blocked_paths_silent,
        claims.branch_cycle_cannot_complete,
        claims.m6_delayed_history_controls,
        claims.duplicate_zero_hop_and_finite_structure,
        claims.transfer_and_blank_start,
        claims.held_out_nonplastic_and_erased,
        claims.frozen_evaluator_boundary_and_lane_isolation,
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "| {} | {} | {}/{} | {} | {} | {}/{}/{} | {}/{} | {} | {}/{}/{} | {} | {}/12 | {}/{}/{}/{}/{}/{}/{}/{} | {} | {} |\n",
            cell.index,
            cell.seed,
            cell.m7_competence_episode,
            cell.competence_episode,
            cell.m7_activations,
            cell.selections,
            cell.consequences,
            cell.updates,
            cell.m6_observations,
            cell.held_out_correct,
            cell.held_out_total,
            cell.quiescent,
            cell.positions,
            cell.depths,
            cell.allocation_layouts,
            cell.route_handles,
            cell.controls_passed,
            cell.p0_source_authority,
            cell.p1_initiation_structural_closure,
            cell.p2_single_closure_role,
            cell.p3_exact_current_crossing,
            cell.p4_quiescent_lifecycle,
            cell.p5_transfer,
            cell.p6_controls,
            cell.p7_duplicate_exact,
            cell.first_collapse,
            cell.passed,
        ));
    }
    text.push_str(&format!(
        "\nM7 authoritative: `{}`. M8 authoritative: `{}`. Post-M7 DS5 successor definitive positive: `{}`. Cognitive de-supply ladder complete: `{}`. DS9 created: `{}`.\n\nPost-M8 substrate-contract/code-consolidation eligibility and program-priority decisions remain outside evidentiary scope.\n",
        report.m7_authoritative,
        report.m8_authoritative,
        report.successor_definitive_positive,
        report.cognitive_desupply_complete,
        report.ds9_created,
    ));
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_cell_preflight_is_exact_and_fresh() {
        let audit = source_preflight(true, true);
        assert!(audit.source.passed(), "{audit:#?}");
        assert!(audit.explicit_matrix);
        assert!(audit.derived_namespaces_disjoint);
        assert!(audit.prior_namespaces_disjoint);
        assert!(audit.held_out_fixed);
        assert!(audit.outputs_absent);
        assert!(audit.staging_absent);
        assert!(audit.passed);
    }

    #[test]
    fn no_cell_preflight_refuses_occupied_paths() {
        assert!(!source_preflight(false, true).passed);
        assert!(!source_preflight(true, false).passed);
        assert!(!source_preflight(false, false).passed);
    }
}
