//! Write-once definitive authority over the byte-frozen post-M6 DS4 mechanism.

pub const PROTOCOL: &str = "post-m6-ds4-arrival-initiation-definitive-v1";
pub const PROTOCOL_COMMIT: &str = "6ba5deee16c37362f99833d1c5afb53b4dee2a2f";
pub const READINESS_COMMIT: &str = "7c39cb4306c61b8e67119c91c3f9e3802dab9a39";
pub const MECHANISM_COMMIT: &str = "e11a6a7927d86bbe0fff5f55942a8b418de39de1";
pub const AUTHORITATIVE_M6: &str = "aa4e22efd8a65b7694956a53cfaa970582695215";

pub const SEEDS: [u64; 16] = [
    700_123_457,
    710_123_457,
    720_123_457,
    730_123_457,
    740_123_457,
    750_123_457,
    760_123_457,
    770_123_457,
    780_123_457,
    790_123_457,
    800_123_457,
    810_123_457,
    820_123_457,
    830_123_457,
    840_123_457,
    850_123_457,
];
pub const HELD_OUT_PER_CELL: usize = 64;
pub const CELL_NAMESPACE: u64 = 6_300_000;

const PROTOCOL_SHA256: &str = "d454a9521fc05416f62155830914fa29de8e1358fb5fe70ce0bca0a0a613412e";
const MECHANISM_SHA256: &str = "67e170f12d7b7649a0a291ddfc16cd80e4b5c15564b65cd09c884f3e52b9ac5b";
const DEVELOPMENT_RUNNER_SHA256: &str =
    "0a7bc106fe5135c04c4e62aed8de77d50ba7b0756ea51548d391ce33c00796e2";
const GATE_RESULT_SHA256: &str = "d16b22930e18f5c58b6490ce24f41fc260c5f0afb84b589fb6a5c27ba73b97e0";
const READINESS_SHA256: &str = "db788154859f1b1038533738f4700caf00eb41e80ff6fb060a2a7d08ce0155c0";
const OLD_NEGATIVE_AUDIT_SHA256: &str =
    "009eb190df8df3faf327abc8567c0074e6fba76de76def4f8bacf14fc1ae7a56";

const M0_CSV_SHA256: &str = "7883f71918d48c4c622d7cd2d9dd7561f5954f7287f8bc6abb535f5a9f994a55";
const M0_MD_SHA256: &str = "a788106462498dd7581fcbd324d6fbc71a1ca0a46c3390a4d289ae180731edad";
const M1_CSV_SHA256: &str = "fede145a50bc059ffcd19a26dc65763843a83b1644c89bd44a3b27e8cd7cea27";
const M1_MD_SHA256: &str = "5971c78eb2688e9fb2d31e59b8d835000ece4923b84aef57d6a1ef48f9295bea";
const M2_CSV_SHA256: &str = "68d6dd31ca15e206b382f3ef6592804882eecfe09efd6696b8ed403dc6304159";
const M2_MD_SHA256: &str = "a67f6d0acc2dc01a8456922153b64b97ad2493f2e7bf50b4d5268533a134f2e1";
const M3_CSV_SHA256: &str = "ac8c0a6c9b7badfa263ceb054ffe59c11162b1ca256c56cc6df5f0d378179401";
const M3_MD_SHA256: &str = "ab77bd12b705b8620b6315260f8bb5b4df6efc961f1d20a0dd521af403e1ac5f";
const M4_CSV_SHA256: &str = "5c4a2e2b021a26a4cc2161202dd9a62205d426ba361f90a69d00ceb3df470a83";
const M4_MD_SHA256: &str = "c418f6b5fb5f8f3f83e385c75cd23fb8c88def3650ff80a13a491d0979944768";
const M5_CSV_SHA256: &str = "86d9f6e3a8ab4ad5c242e0d7c619d8eda99e0da47faff623f26c8c6835b9a99a";
const M5_MD_SHA256: &str = "a336633c73565261d357a67ca02df3047ffcaf88488153bb2f43b621818ba5f0";
const M6_CSV_SHA256: &str = "0cb9ba779fca1899cf030d30358fe9354cfb7b2cccf87f32df3f6ea9ddfe91e4";
const M6_MD_SHA256: &str = "6a5d938c3e021344b00f3a559593fee860b5f6cceb777c409ad8d59a2dd71872";

#[allow(dead_code)]
mod frozen_development {
    include!(concat!(
        env!("OUT_DIR"),
        "/post_m6_ds4_arrival_initiation_frozen.rs"
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
        pub(super) learned_event_activity: usize,
        pub(super) selections: usize,
        pub(super) recurrences: usize,
        pub(super) consequences: usize,
        pub(super) updates: usize,
        pub(super) credit_observations: u64,
        pub(super) held_out_correct: usize,
        pub(super) held_out_total: usize,
        pub(super) explicit: usize,
        pub(super) quiescent: usize,
        pub(super) positions: usize,
        pub(super) physical_work: u64,
        pub(super) m3_nonplastic: bool,
        pub(super) p4_nonplastic: bool,
        pub(super) m6_nonplastic: bool,
        pub(super) namespaces_disjoint: bool,
        pub(super) controls: [bool; 12],
        pub(super) missing_stale_invalid_execution: bool,
    }

    pub(super) fn authority_source_audit() -> (bool, bool, bool) {
        let audit = source_audit();
        (
            audit.passed(),
            audit.information_boundary,
            audit.lane_isolated,
        )
    }

    fn silent(step: frozen_request::Step) -> bool {
        step.selected == 0
            && step.executed == 0
            && step.recurrence == 0
            && step.consequences == 0
            && step.updates == 0
    }

    fn missing_stale_invalid_execution(seed: u64) -> bool {
        use frozen_pre_m6_ds4::ArrivalFixture;

        let control_seed = seed + 400_000;
        let Some(mut event_gate) = frozen_pre_m6_ds4::arrival_gate(control_seed, 2) else {
            return false;
        };
        let mut request = frozen_request::session(control_seed as usize);
        let mut credit = frozen_m6::credit_gate(control_seed);
        for episode in 1..=4_000usize {
            let activity = frozen_pre_m6_ds4::arrival_activity(
                &mut event_gate,
                control_seed + 10_000 + episode as u64,
                ArrivalFixture::Standard,
            );
            let _ =
                frozen_request::step(&mut request, &mut credit, activity.completion, true, false);
            if frozen_request::ready(&request) {
                break;
            }
        }
        if !frozen_request::ready(&request) || frozen_request::roles(&request) != 1 {
            return false;
        }

        let stale = frozen_request::step(&mut request, &mut credit, 0, false, false);
        let missing = frozen_pre_m6_ds4::arrival_activity(
            &mut event_gate,
            control_seed + 20_000,
            ArrivalFixture::MissingClose,
        );
        let missing_step =
            frozen_request::step(&mut request, &mut credit, missing.completion, false, false);
        let invalid = frozen_pre_m6_ds4::arrival_activity(
            &mut event_gate,
            control_seed + 30_000,
            ArrivalFixture::InvalidTransition,
        );
        let invalid_step =
            frozen_request::step(&mut request, &mut credit, invalid.completion, false, false);
        let reentry = frozen_pre_m6_ds4::arrival_activity(
            &mut event_gate,
            control_seed + 30_001,
            ArrivalFixture::Relabelled,
        );
        let reentry_step =
            frozen_request::step(&mut request, &mut credit, reentry.completion, false, false);

        silent(stale)
            && missing.completion == 0
            && silent(missing_step)
            && invalid.completion == 0
            && silent(invalid_step)
            && reentry.completion > 0
            && reentry_step.selected == 1
            && reentry_step.executed == 1
            && reentry_step.recurrence == 1
            && reentry_step.from_occurrence
            && reentry_step.functional
            && reentry_step.explicit
            && reentry_step.quiescent
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
            learned_event_activity: observed.learned_event_activity,
            selections: observed.selections,
            recurrences: observed.recurrences,
            consequences: observed.consequences,
            updates: observed.updates,
            credit_observations: observed.credit_observations,
            held_out_correct: observed.held_out_correct,
            held_out_total: observed.held_out_total,
            explicit: observed.explicit,
            quiescent: observed.quiescent,
            positions: observed.positions.len(),
            physical_work: observed.physical_work,
            m3_nonplastic: observed.m3_nonplastic,
            p4_nonplastic: observed.p4_nonplastic,
            m6_nonplastic: observed.m6_nonplastic,
            namespaces_disjoint: observed.acquisition.is_disjoint(&observed.held_out),
            controls,
            missing_stale_invalid_execution: missing_stale_invalid_execution(seed),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub exact_frozen_mechanism: bool,
    pub exact_development_lineage: bool,
    pub exact_original_negative: bool,
    pub exact_m0_m6_artifacts: bool,
    pub source_order_and_information_flow: bool,
    pub lane_b_isolated: bool,
    pub protocol_and_commits: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.exact_frozen_mechanism
            && self.exact_development_lineage
            && self.exact_original_negative
            && self.exact_m0_m6_artifacts
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
    pub development_namespaces_disjoint: bool,
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
    pub learned_event_activity: usize,
    pub selections: usize,
    pub recurrences: usize,
    pub consequences: usize,
    pub updates: usize,
    pub credit_observations: u64,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub explicit: usize,
    pub quiescent: usize,
    pub positions: usize,
    pub physical_work: u64,
    pub controls_passed: usize,
    pub p0_source_authority: bool,
    pub p1_arrival_event_paths: bool,
    pub p2_recurrence_consequence: bool,
    pub p3_single_anonymous_role: bool,
    pub p4_held_out_transfer: bool,
    pub p5_nonplastic_lifecycle: bool,
    pub p6_controls: bool,
    pub p7_duplicate_exact: bool,
    pub first_collapse: &'static str,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimSummary {
    pub blank_single_role_acquisition: bool,
    pub physical_arrival_and_event_required: bool,
    pub recurrence_and_delayed_consequence_required: bool,
    pub invalid_paths_silent: bool,
    pub held_out_correct_explicit_quiescent: bool,
    pub identity_layout_allocation_serialization_transfer: bool,
    pub evaluator_blind_information_flow: bool,
    pub m3_p4_m6_nonplastic: bool,
    pub frozen_artifacts_exact: bool,
    pub lane_b_absent: bool,
    pub duplicate_physical_execution_exact: bool,
}

impl ClaimSummary {
    pub fn passed(&self) -> bool {
        self.blank_single_role_acquisition
            && self.physical_arrival_and_event_required
            && self.recurrence_and_delayed_consequence_required
            && self.invalid_paths_silent
            && self.held_out_correct_explicit_quiescent
            && self.identity_layout_allocation_serialization_transfer
            && self.evaluator_blind_information_flow
            && self.m3_p4_m6_nonplastic
            && self.frozen_artifacts_exact
            && self.lane_b_absent
            && self.duplicate_physical_execution_exact
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefinitiveReport {
    pub protocol: &'static str,
    pub preflight: Preflight,
    pub claims: ClaimSummary,
    pub cells: Vec<DefinitiveCell>,
    pub passed: bool,
    pub m6_authoritative: bool,
    pub m7_authoritative: bool,
    pub successor_definitive_positive: bool,
    pub ds5_development_eligible: bool,
}

fn marked<'a>(source: &'a str, begin: &str, end: &str) -> &'a str {
    source
        .split(begin)
        .nth(1)
        .and_then(|tail| tail.split(end).next())
        .unwrap_or_default()
}

fn source_order_and_information_flow() -> bool {
    let mechanism = include_str!("post_m6_ds4_arrival_initiation.rs");
    let request = marked(
        mechanism,
        "// POST_M6_DS4_REQUEST_LINKER_BEGIN",
        "// POST_M6_DS4_REQUEST_LINKER_END",
    );
    let consequence = marked(
        mechanism,
        "// POST_M6_DS4_GATE_BEGIN",
        "// POST_M6_DS4_GATE_END",
    );
    let after_request = mechanism
        .split("// POST_M6_DS4_REQUEST_LINKER_END")
        .nth(1)
        .unwrap_or_default();
    let authority = include_str!("post_m6_ds4_arrival_initiation_definitive.rs");
    let authority_call = marked(
        authority,
        "// POST_M6_DS4_AUTHORITY_CELL_CALL_BEGIN",
        "// POST_M6_DS4_AUTHORITY_CELL_CALL_END",
    );
    let forbidden = [
        ["func", "tional"].concat(),
        ["episode", ".answer"].concat(),
        ["expected", "_answer"].concat(),
        ["expected", "_trace"].concat(),
        ["semantic", "_credit"].concat(),
        ["start", "_flag"].concat(),
        ["target", "_identity"].concat(),
        ["target", "_channel"].concat(),
        ["rew", "ard"].concat(),
        ["lo", "ss"].concat(),
        ["ss", "a"].concat(),
    ];
    let request_clean = forbidden.iter().all(|word| !request.contains(word));
    let consequence_clean = forbidden.iter().all(|word| !consequence.contains(word));
    let authority_clean = forbidden.iter().all(|word| !authority_call.contains(word));
    let request_execution = request.find("execute_choice");
    let request_differential = request.find("apply_recurrence");
    let request_feedback = request.find("learner.feedback(choice.pattern_cell, true)");
    let consequence_execution = consequence.find("gate.path.execute(edge)");
    let recurrence_check = consequence.find("if !recurrent_activity");
    let consequence_apply = consequence.find("gate.learner.apply");
    request_clean
        && consequence_clean
        && authority_clean
        && request.matches("apply_recurrence").count() == 1
        && request
            .matches("learner.feedback(choice.pattern_cell, true)")
            .count()
            == 1
        && matches!(
            (request_execution, request_differential, request_feedback),
            (Some(execution), Some(differential), Some(feedback))
                if execution < differential && differential < feedback
        )
        && matches!(
            (consequence_execution, recurrence_check, consequence_apply),
            (Some(execution), Some(recurrence), Some(apply))
                if execution < recurrence && recurrence < apply
        )
        && consequence.matches("gate.learner.apply").count() == 1
        && after_request.contains("run.outcome == BindingOutcome::Answer(episode.answer)")
        && authority_call
            .matches("frozen_development::authority_observation")
            .count()
            == 1
        && authority_call
            .contains("frozen_development::authority_observation(seed, HELD_OUT_PER_CELL)")
}

fn exact_m0_m6_artifacts() -> bool {
    [
        (env!("POST_M6_DS4_DEFINITIVE_M0_CSV_SHA256"), M0_CSV_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M0_MD_SHA256"), M0_MD_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M1_CSV_SHA256"), M1_CSV_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M1_MD_SHA256"), M1_MD_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M2_CSV_SHA256"), M2_CSV_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M2_MD_SHA256"), M2_MD_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M3_CSV_SHA256"), M3_CSV_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M3_MD_SHA256"), M3_MD_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M4_CSV_SHA256"), M4_CSV_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M4_MD_SHA256"), M4_MD_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M5_CSV_SHA256"), M5_CSV_SHA256),
        (env!("POST_M6_DS4_DEFINITIVE_M5_MD_SHA256"), M5_MD_SHA256),
        (env!("POST_M6_DS4_M6_CSV_SHA256"), M6_CSV_SHA256),
        (env!("POST_M6_DS4_M6_MD_SHA256"), M6_MD_SHA256),
    ]
    .iter()
    .all(|(observed, expected)| observed == expected)
}

fn source_audit() -> SourceAudit {
    let frozen = frozen_development::authority_source_audit();
    SourceAudit {
        exact_frozen_mechanism: env!("POST_M6_DS4_DEFINITIVE_MECHANISM_SHA256") == MECHANISM_SHA256
            && frozen.0,
        exact_development_lineage: env!("POST_M6_DS4_DEFINITIVE_DEVELOPMENT_RUNNER_SHA256")
            == DEVELOPMENT_RUNNER_SHA256
            && env!("POST_M6_DS4_DEFINITIVE_GATE_RESULT_SHA256") == GATE_RESULT_SHA256
            && env!("POST_M6_DS4_DEFINITIVE_READINESS_SHA256") == READINESS_SHA256,
        exact_original_negative: env!("POST_M6_DS4_OLD_NEGATIVE_CSV_SHA256")
            == frozen_development::FROZEN_OLD_NEGATIVE_CSV_SHA256
            && env!("POST_M6_DS4_OLD_NEGATIVE_MD_SHA256")
                == frozen_development::FROZEN_OLD_NEGATIVE_MD_SHA256
            && env!("POST_M6_DS4_DEFINITIVE_OLD_NEGATIVE_AUDIT_SHA256")
                == OLD_NEGATIVE_AUDIT_SHA256,
        exact_m0_m6_artifacts: exact_m0_m6_artifacts(),
        source_order_and_information_flow: source_order_and_information_flow() && frozen.1,
        lane_b_isolated: frozen.2,
        protocol_and_commits: env!("POST_M6_DS4_DEFINITIVE_PROTOCOL_SHA256") == PROTOCOL_SHA256
            && PROTOCOL_COMMIT == "6ba5deee16c37362f99833d1c5afb53b4dee2a2f"
            && READINESS_COMMIT == "7c39cb4306c61b8e67119c91c3f9e3802dab9a39"
            && MECHANISM_COMMIT == "e11a6a7927d86bbe0fff5f55942a8b418de39de1"
            && AUTHORITATIVE_M6 == "aa4e22efd8a65b7694956a53cfaa970582695215",
    }
}

pub fn source_preflight(outputs_absent: bool, staging_absent: bool) -> Preflight {
    let source = source_audit();
    let explicit_matrix = SEEDS
        == [
            700_123_457,
            710_123_457,
            720_123_457,
            730_123_457,
            740_123_457,
            750_123_457,
            760_123_457,
            770_123_457,
            780_123_457,
            790_123_457,
            800_123_457,
            810_123_457,
            820_123_457,
            830_123_457,
            840_123_457,
            850_123_457,
        ]
        && SEEDS.len() == 16;
    let derived_namespaces_disjoint = SEEDS.windows(2).all(|pair| {
        pair[0] + CELL_NAMESPACE <= pair[1]
            && pair[1] - pair[0] == 10_000_000
            && pair[0] + 6_200_128 < pair[0] + CELL_NAMESPACE
    });
    let development_namespaces_disjoint = SEEDS[0] > 152_700_000;
    let held_out_fixed = HELD_OUT_PER_CELL == 64;
    let passed = source.passed()
        && explicit_matrix
        && derived_namespaces_disjoint
        && development_namespaces_disjoint
        && held_out_fixed
        && outputs_absent
        && staging_absent;
    Preflight {
        source,
        explicit_matrix,
        derived_namespaces_disjoint,
        development_namespaces_disjoint,
        held_out_fixed,
        outputs_absent,
        staging_absent,
        passed,
    }
}

// POST_M6_DS4_AUTHORITY_CELL_CALL_BEGIN
fn frozen_authority_cell(seed: u64) -> frozen_development::AuthorityObservation {
    frozen_development::authority_observation(seed, HELD_OUT_PER_CELL)
}
// POST_M6_DS4_AUTHORITY_CELL_CALL_END

fn run_cell(index: usize, seed: u64, source: &SourceAudit) -> DefinitiveCell {
    let first = frozen_authority_cell(seed);
    let second = frozen_authority_cell(seed);
    let duplicate_exact = first == second;
    let mut controls = first.controls;
    controls[2] &= first.missing_stale_invalid_execution;
    controls[8] &= first.positions == 6;
    controls[10] &= source.source_order_and_information_flow && source.lane_b_isolated;
    controls[11] &= duplicate_exact;

    let p0 =
        source.passed() && first.source_passed && first.information_boundary && first.lane_isolated;
    let p1 = first.learned_event_activity > 0
        && controls[0]
        && controls[1]
        && controls[2]
        && first.missing_stale_invalid_execution;
    let p2 = first.selections > 0
        && first.recurrences > 0
        && first.consequences > 0
        && first.updates > 0
        && first.credit_observations > 0
        && first.physical_work > 0
        && controls[3..8].iter().all(|passed| *passed);
    let p3 = first.learners == 1
        && first.ready == 1
        && first.single_roles == 1
        && first.competence.len() == 1
        && first.competence[0] <= 4_000
        && controls[9];
    let p4 = first.held_out_total == HELD_OUT_PER_CELL
        && first.held_out_correct == HELD_OUT_PER_CELL
        && first.explicit == HELD_OUT_PER_CELL
        && first.quiescent == HELD_OUT_PER_CELL
        && first.positions == 6
        && controls[8];
    let p5 = first.m3_nonplastic
        && first.p4_nonplastic
        && first.m6_nonplastic
        && first.namespaces_disjoint
        && first.physical_work > 0;
    let p6 = controls.iter().all(|passed| *passed);
    let p7 = duplicate_exact;
    let stages = [p0, p1, p2, p3, p4, p5, p6, p7];
    let first_collapse = stages
        .iter()
        .position(|passed| !passed)
        .map_or("NONE", |stage| {
            [
                "P0 source and authority",
                "P1 physical arrival and learned event requirement",
                "P2 recurrence and delayed consequence requirement",
                "P3 exactly one anonymous initiation role",
                "P4 held-out function and transfer",
                "P5 non-plasticity and lifecycle",
                "P6 twelve controls",
                "P7 exact duplicate physical execution",
            ][stage]
        });
    DefinitiveCell {
        index,
        seed,
        competence_episode: first.competence.first().copied().unwrap_or(0),
        learned_event_activity: first.learned_event_activity,
        selections: first.selections,
        recurrences: first.recurrences,
        consequences: first.consequences,
        updates: first.updates,
        credit_observations: first.credit_observations,
        held_out_correct: first.held_out_correct,
        held_out_total: first.held_out_total,
        explicit: first.explicit,
        quiescent: first.quiescent,
        positions: first.positions,
        physical_work: first.physical_work,
        controls_passed: controls.iter().filter(|passed| **passed).count(),
        p0_source_authority: p0,
        p1_arrival_event_paths: p1,
        p2_recurrence_consequence: p2,
        p3_single_anonymous_role: p3,
        p4_held_out_transfer: p4,
        p5_nonplastic_lifecycle: p5,
        p6_controls: p6,
        p7_duplicate_exact: p7,
        first_collapse,
        passed: stages.iter().all(|passed| *passed),
    }
}

fn summarize(preflight: &Preflight, cells: &[DefinitiveCell]) -> ClaimSummary {
    ClaimSummary {
        blank_single_role_acquisition: cells.iter().all(|cell| cell.p3_single_anonymous_role),
        physical_arrival_and_event_required: cells.iter().all(|cell| cell.p1_arrival_event_paths),
        recurrence_and_delayed_consequence_required: cells
            .iter()
            .all(|cell| cell.p2_recurrence_consequence),
        invalid_paths_silent: cells
            .iter()
            .all(|cell| cell.p1_arrival_event_paths && cell.controls_passed == 12),
        held_out_correct_explicit_quiescent: cells.iter().all(|cell| cell.p4_held_out_transfer),
        identity_layout_allocation_serialization_transfer: cells
            .iter()
            .all(|cell| cell.positions == 6 && cell.p4_held_out_transfer),
        evaluator_blind_information_flow: preflight.source.source_order_and_information_flow,
        m3_p4_m6_nonplastic: cells.iter().all(|cell| cell.p5_nonplastic_lifecycle),
        frozen_artifacts_exact: preflight.source.exact_frozen_mechanism
            && preflight.source.exact_development_lineage
            && preflight.source.exact_original_negative
            && preflight.source.exact_m0_m6_artifacts,
        lane_b_absent: preflight.source.lane_b_isolated,
        duplicate_physical_execution_exact: cells.iter().all(|cell| cell.p7_duplicate_exact),
    }
}

pub fn run_definitive(outputs_absent: bool, staging_absent: bool) -> DefinitiveReport {
    let preflight = source_preflight(outputs_absent, staging_absent);
    assert!(
        preflight.passed,
        "definitive preflight must pass before any learner or cell"
    );
    eprintln!("POST_M6_DS4_DEFINITIVE_EVIDENCE_SPENT");
    let mut cells = Vec::with_capacity(SEEDS.len());
    for seed in SEEDS {
        let index = cells.len();
        cells.push(run_cell(index, seed, &preflight.source));
    }
    let claims = summarize(&preflight, &cells);
    let passed = cells.len() == 16
        && cells.iter().all(|cell| cell.passed)
        && cells.iter().map(|cell| cell.held_out_total).sum::<usize>() == 1_024
        && cells.iter().map(|cell| cell.controls_passed).sum::<usize>() == 192
        && claims.passed();
    DefinitiveReport {
        protocol: PROTOCOL,
        preflight,
        claims,
        cells,
        passed,
        m6_authoritative: !passed,
        m7_authoritative: passed,
        successor_definitive_positive: passed,
        ds5_development_eligible: passed,
    }
}

pub fn csv(report: &DefinitiveReport) -> String {
    let mut text = String::from(
        "index,seed,competence_episode,learned_event_activity,selections,recurrences,consequences,updates,credit_observations,held_out_correct,held_out_total,explicit,quiescent,positions,physical_work,controls_passed,p0_source_authority,p1_arrival_event_paths,p2_recurrence_consequence,p3_single_anonymous_role,p4_held_out_transfer,p5_nonplastic_lifecycle,p6_controls,p7_duplicate_exact,first_collapse,passed\n",
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            cell.index,
            cell.seed,
            cell.competence_episode,
            cell.learned_event_activity,
            cell.selections,
            cell.recurrences,
            cell.consequences,
            cell.updates,
            cell.credit_observations,
            cell.held_out_correct,
            cell.held_out_total,
            cell.explicit,
            cell.quiescent,
            cell.positions,
            cell.physical_work,
            cell.controls_passed,
            cell.p0_source_authority,
            cell.p1_arrival_event_paths,
            cell.p2_recurrence_consequence,
            cell.p3_single_anonymous_role,
            cell.p4_held_out_transfer,
            cell.p5_nonplastic_lifecycle,
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
        "# Post-M6 DS4 arrival-initiation definitive result\n\nVerdict: **{}**.\n\nProtocol: `{}`. Definitive cells: `{}/16`. Held-out correct/explicit/quiescent: `{}/{}/{}` of `1024`. Controls: `{}/192`.\n\n## Definitive conjunction\n\n| claim | pass |\n|---|:---:|\n| blank learners acquire exactly one anonymous initiation role | {} |\n| physical arrival and learned event required | {} |\n| recurrent delayed physical consequence required | {} |\n| missing, stale, and invalid paths remain silent | {} |\n| held-out correct, explicit, and naturally quiescent | {} |\n| identity/layout/allocation/serialization transfer | {} |\n| evaluator-blind post-execution information flow | {} |\n| M3/P4/M6 held-out non-plastic | {} |\n| original negative, development stages, and M0-M6 exact | {} |\n| Lane-B SSA absent | {} |\n| duplicate physical executions exact | {} |\n\n## Cells\n\n| cell | seed | competence | event | selection/recurrence | consequence/update/observations | held-out | explicit | quiescent | positions | controls | P0..P7 | first collapse | result |\n|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|:---:|\n",
        if report.passed { "PASS" } else { "FAIL" },
        report.protocol,
        report.cells.iter().filter(|cell| cell.passed).count(),
        report.cells.iter().map(|cell| cell.held_out_correct).sum::<usize>(),
        report.cells.iter().map(|cell| cell.explicit).sum::<usize>(),
        report.cells.iter().map(|cell| cell.quiescent).sum::<usize>(),
        report.cells.iter().map(|cell| cell.controls_passed).sum::<usize>(),
        claims.blank_single_role_acquisition,
        claims.physical_arrival_and_event_required,
        claims.recurrence_and_delayed_consequence_required,
        claims.invalid_paths_silent,
        claims.held_out_correct_explicit_quiescent,
        claims.identity_layout_allocation_serialization_transfer,
        claims.evaluator_blind_information_flow,
        claims.m3_p4_m6_nonplastic,
        claims.frozen_artifacts_exact,
        claims.lane_b_absent,
        claims.duplicate_physical_execution_exact,
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "| {} | {} | {} | {} | {}/{} | {}/{}/{} | {}/{} | {} | {} | {}/6 | {}/12 | {}/{}/{}/{}/{}/{}/{}/{} | {} | {} |\n",
            cell.index,
            cell.seed,
            cell.competence_episode,
            cell.learned_event_activity,
            cell.selections,
            cell.recurrences,
            cell.consequences,
            cell.updates,
            cell.credit_observations,
            cell.held_out_correct,
            cell.held_out_total,
            cell.explicit,
            cell.quiescent,
            cell.positions,
            cell.controls_passed,
            cell.p0_source_authority,
            cell.p1_arrival_event_paths,
            cell.p2_recurrence_consequence,
            cell.p3_single_anonymous_role,
            cell.p4_held_out_transfer,
            cell.p5_nonplastic_lifecycle,
            cell.p6_controls,
            cell.p7_duplicate_exact,
            cell.first_collapse,
            cell.passed,
        ));
    }
    text.push_str(&format!(
        "\nM6 authoritative: `{}`. M7 authoritative: `{}`. Post-M6 DS4 successor definitive positive: `{}`. DS5 finish/output development eligible: `{}`.\n\nProgram-priority decisions remain outside evidentiary scope.\n",
        report.m6_authoritative,
        report.m7_authoritative,
        report.successor_definitive_positive,
        report.ds5_development_eligible,
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
        assert!(audit.development_namespaces_disjoint);
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
