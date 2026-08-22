//! Development-only post-M6 DS4 physical-arrival initiation successor.

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "post-m6-ds4-arrival-initiation-v2";
pub const AUTHORITATIVE_M6: &str = "aa4e22efd8a65b7694956a53cfaa970582695215";
pub const PROBE_V1_SEED: u64 = 140_000_000;
pub const PROBE_RETRY_SEED: u64 = 141_000_000;
pub const MICRO_SEEDS: [u64; 2] = [142_000_000, 142_500_000];
pub const GATE_SEEDS: [u64; 6] = [
    144_000_000,
    144_500_000,
    145_000_000,
    145_500_000,
    146_000_000,
    146_500_000,
];
pub const FROZEN_PROTOCOL_SHA256: &str =
    "01c47af6fe1be9dc1e48a4b81a94e194df36d1631e1d650c8f2e94284bd42d6b";
pub const FROZEN_M6_CSV_SHA256: &str =
    "0cb9ba779fca1899cf030d30358fe9354cfb7b2cccf87f32df3f6ea9ddfe91e4";
pub const FROZEN_M6_MD_SHA256: &str =
    "6a5d938c3e021344b00f3a559593fee860b5f6cceb777c409ad8d59a2dd71872";
pub const FROZEN_M6_HANDOFF_SHA256: &str =
    "6cdd015d6b20f10a95f26c33dfe30ceb834b2663f9912926752fa1fb204c9ca9";
pub const FROZEN_OLD_NEGATIVE_CSV_SHA256: &str =
    "c6b626650fc199a8ebe2feae8115a8b27071088f963f919393f55e24fbe44a3a";
pub const FROZEN_OLD_NEGATIVE_MD_SHA256: &str =
    "97f1fc665e03be1ccd398dcdf34fbb262c525aabc23a812c8740c350dd890659";
pub const FROZEN_PROBE_V1_RESULT_SHA256: &str =
    "46dac216ed3977ec8d12821af1b5b69d93f6932e20364c4d4c1b5809b3fba9c1";
pub const FROZEN_PROBE_V1_AUDIT_SHA256: &str =
    "660ba16162153ed909e3c28476282647bd0b3d8b8414663575efe8795db4d504";
pub const FROZEN_PROBE_RETRY_RESULT_SHA256: &str =
    "4720e0558fb0241ec893fa451c097c384d7e246a1efe3488a375e3077415f7fa";
pub const FROZEN_PROBE_HANDOFF_SHA256: &str =
    "bef1c81f4dc48e1fcbe463d1518dad6fec14a5c32fa43f1b4ac82146a56abb07";
pub const FROZEN_MICRO_RESULT_SHA256: &str =
    "c77de17b00a1e60a5e512d7544c09735d3b84bd3f46a9b3793e215072c6504a8";
pub const FROZEN_MICRO_HANDOFF_SHA256: &str =
    "d3c3535bfdb28476ab5d5401b245b34984a1311bc03cd68248406bb99ae0b17f";
pub const FROZEN_M3_SHA256: &str =
    "c4fc7aca11a5925effeb5a84b90184a70da0f66da7c063d0f87ba46ca36addf3";
pub const FROZEN_P4_SHA256: &str =
    "2dbde723b394bcb3d788c796aa1745cd1cea392a64ab61497bb97474866144b8";
pub const FROZEN_M5_SHA256: &str =
    "e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7";
pub const FROZEN_M6_LINKER_SHA256: &str =
    "1f68f7e943f37c42d29f16fe26f0d851a59361ed4c1f4273a82d0537f935d343";

#[allow(dead_code)]
mod frozen_pre_m6_ds4 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds4_cumulative_request_start_port.rs"
    ));

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(super) struct ArrivalActivity {
        pub(super) completion: usize,
        pub(super) generic: usize,
        pub(super) learned: usize,
        pub(super) work: u64,
        pub(super) chunks: usize,
        pub(super) bytes: usize,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(super) enum ArrivalFixture {
        Standard,
        Relabelled,
        MissingClose,
        InvalidTransition,
    }

    pub(super) struct ArrivalGate(frozen_m3::Ds4EventGate);

    pub(super) fn arrival_gate(seed: u64, acquisition: usize) -> Option<ArrivalGate> {
        frozen_m3::ds4_event_gate(seed, acquisition).map(ArrivalGate)
    }

    pub(super) fn arrival_activity(
        gate: &mut ArrivalGate,
        seed: u64,
        fixture: ArrivalFixture,
    ) -> ArrivalActivity {
        let fixture = match fixture {
            ArrivalFixture::Standard => EventFixture::Standard,
            ArrivalFixture::Relabelled => EventFixture::Relabelled,
            ArrivalFixture::MissingClose => EventFixture::MissingClose,
            ArrivalFixture::InvalidTransition => EventFixture::InvalidTransition,
        };
        let activity = frozen_m3::event_completion_activity(&mut gate.0, seed, fixture);
        ArrivalActivity {
            completion: activity.completion_spikes,
            generic: activity.generic_spans,
            learned: activity.learned_uses,
            work: activity.physical_work,
            chunks: activity.chunks,
            bytes: activity.persistent_bytes,
        }
    }

    pub(super) fn arrival_state(gate: &ArrivalGate) -> (usize, usize) {
        let state = frozen_m3::ds4_event_state(&gate.0);
        (state.chunks, state.persistent_bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeV1Report {
    pub protocol: &'static str,
    pub seed: u64,
    pub claim_eligible: bool,
    pub expected_negative: bool,
    pub exact_m6: bool,
    pub immutable_old_negative: bool,
    pub protocol_frozen: bool,
    pub physical_arrival_path: bool,
    pub learned_event_activity: usize,
    pub occurrence_selections: usize,
    pub semantic_feedback_calls: usize,
    pub m6_differential_links: usize,
    pub lawful_updates: usize,
    pub first_collapse: &'static str,
}

pub fn run_probe_v1() -> ProbeV1Report {
    let old = frozen_pre_m6_ds4::run_probe();
    let old_source = include_str!("ds4_cumulative_request_start_port.rs");
    let linker = old_source
        .split("// DS4_LINKER_START")
        .nth(1)
        .and_then(|tail| tail.split("// DS4_LINKER_END").next())
        .unwrap_or_default();
    let semantic_feedback_calls = linker
        .matches("learner.feedback(choice.pattern_cell, functional)")
        .count();
    let m6_differential_links = linker.matches("delayed_experience(differential)").count();
    let exact_m6 = AUTHORITATIVE_M6 == "aa4e22efd8a65b7694956a53cfaa970582695215"
        && env!("POST_M6_DS4_M6_CSV_SHA256") == FROZEN_M6_CSV_SHA256
        && env!("POST_M6_DS4_M6_MD_SHA256") == FROZEN_M6_MD_SHA256
        && env!("POST_M6_DS4_M6_HANDOFF_SHA256") == FROZEN_M6_HANDOFF_SHA256;
    let immutable_old_negative = env!("POST_M6_DS4_OLD_NEGATIVE_CSV_SHA256")
        == FROZEN_OLD_NEGATIVE_CSV_SHA256
        && env!("POST_M6_DS4_OLD_NEGATIVE_MD_SHA256") == FROZEN_OLD_NEGATIVE_MD_SHA256;
    let protocol_frozen = env!("POST_M6_DS4_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256;
    let physical_arrival_path = old.path_exists
        && old.learned_m3_uses > 0
        && old.completion_activity > 0
        && old.request_selection_activations > 0;
    let lawful_updates = if semantic_feedback_calls == 0 && m6_differential_links == 1 {
        old.request_update_activations
    } else {
        0
    };
    let expected_negative = exact_m6
        && immutable_old_negative
        && protocol_frozen
        && physical_arrival_path
        && semantic_feedback_calls == 1
        && m6_differential_links == 0
        && lawful_updates == 0;
    ProbeV1Report {
        protocol: PROTOCOL,
        seed: PROBE_V1_SEED,
        claim_eligible: false,
        expected_negative,
        exact_m6,
        immutable_old_negative,
        protocol_frozen,
        physical_arrival_path,
        learned_event_activity: old.completion_activity,
        occurrence_selections: old.request_selection_activations,
        semantic_feedback_calls,
        m6_differential_links,
        lawful_updates,
        first_collapse: "P4 semantic terminal credit; M6 differential-to-active-trace edge absent",
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct CreditControls {
    no_recurrence: bool,
    immediate_rejected: bool,
    absent_eligibility: bool,
    equal_and_shuffled_abstain: bool,
    swapped_history_reverses: bool,
}

impl CreditControls {
    fn passed(self) -> bool {
        self.no_recurrence
            && self.immediate_rejected
            && self.absent_eligibility
            && self.equal_and_shuffled_abstain
            && self.swapped_history_reverses
    }
}

#[allow(dead_code)]
mod frozen_m6 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds7_cumulative_plasticity_targeting_probe_frozen.rs"
    ));

    const MINIMUM_DELAY: u8 = 2;
    const RECURRENT_SUPPORT: u16 = 4;
    const MINIMUM_MARGIN: u16 = 2;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RawConsequence {
        occurrences: [u64; 3],
        ticks: [u8; 3],
        arrows: [[u8; 2]; 2],
        root: u8,
    }

    include!(concat!(
        env!("OUT_DIR"),
        "/ds8_cumulative_semantic_credit_linker_frozen.rs"
    ));

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct CreditGate {
        path: PlasticityPath,
        learner: ConsequenceLearner,
        seed: u64,
        episode: usize,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(super) struct CreditState {
        pub(super) prototypes: usize,
        pub(super) values: usize,
        pub(super) proposals: usize,
        pub(super) observations: u64,
        pub(super) updates: usize,
    }

    fn raw(seed: u64, episode: usize, variant: usize, immediate: bool) -> RawConsequence {
        let base = seed
            .wrapping_mul(1_000_003)
            .wrapping_add(episode as u64 * 53)
            .wrapping_add(1 << 33);
        let (root, arrows) = match variant % 4 {
            0 => (0, [[0, 1], [1, 2]]),
            1 => (0, [[0, 2], [2, 1]]),
            2 => (1, [[1, 0], [0, 2]]),
            _ => (2, [[2, 1], [1, 0]]),
        };
        let first_tick = if immediate { 1 } else { MINIMUM_DELAY };
        RawConsequence {
            occurrences: [base, base + 1, base + 2],
            ticks: [first_tick, first_tick + 1, first_tick + 2],
            arrows,
            root,
        }
    }

    fn productive(seed: u64, ordinal: u64, reverse: bool) -> PhysicalEncounter {
        let base = seed + 10_000 + ordinal * 2;
        if reverse {
            pattern_p(base, base + 1, 91, 90)
        } else {
            pattern_p(base, base + 1, 40, 41)
        }
    }

    fn contrast(seed: u64, ordinal: u64, reverse: bool) -> PhysicalEncounter {
        let base = seed + 20_000 + ordinal * 2;
        if reverse {
            pattern_n(base, base + 1, 101, 100)
        } else {
            pattern_n(base, base + 1, 50, 51)
        }
    }

    pub(super) fn credit_gate(seed: u64) -> CreditGate {
        CreditGate {
            path: PlasticityPath::default(),
            learner: ConsequenceLearner::default(),
            seed,
            episode: 0,
        }
    }

    // POST_M6_DS4_GATE_BEGIN
    pub(super) fn apply_recurrence(gate: &mut CreditGate, recurrent_activity: bool) -> bool {
        let ordinal = gate.episode as u64;
        gate.episode += 1;
        let active_encounter = productive(gate.seed, ordinal, ordinal.is_multiple_of(2));
        let active = active_encounter.snapshot();
        let other = contrast(gate.seed, 0, false).snapshot();
        let Some(edge) = gate.path.encounter(active_encounter) else {
            return false;
        };
        if !gate.path.execute(edge) {
            return false;
        }
        if !recurrent_activity {
            gate.path.eligibility = None;
            return false;
        }
        gate.learner.apply(
            &mut gate.path,
            active,
            other,
            raw(gate.seed + 100_000, gate.episode, 0, false),
        )
    }
    // POST_M6_DS4_GATE_END

    pub(super) fn credit_state(gate: &CreditGate) -> CreditState {
        CreditState {
            prototypes: gate.path.encoder.records.len(),
            values: gate.path.values.len(),
            proposals: gate.path.proposals.len(),
            observations: gate.learner.work.observations,
            updates: gate
                .path
                .values
                .values()
                .map(|value| value.support + value.rejection)
                .sum(),
        }
    }

    fn direction_for(seed: u64, stable_first: bool, shuffled: bool) -> Option<EncounterSnapshot> {
        let first = productive(seed, 0, false).snapshot();
        let second = contrast(seed, 0, false).snapshot();
        let mut learner = ConsequenceLearner::default();
        for episode in 0..8usize {
            let first_variant = if stable_first && !shuffled {
                0
            } else {
                episode % 4
            };
            let second_variant = if !stable_first && !shuffled {
                0
            } else if shuffled {
                (episode + 2) % 4
            } else {
                episode % 4
            };
            let _ = learner.observe(
                first,
                raw(seed + 200_000, episode * 2, first_variant, false),
            );
            let _ = learner.observe(
                second,
                raw(seed + 200_000, episode * 2 + 1, second_variant, false),
            );
        }
        learner.direction([first, second])
    }

    pub(super) fn controls(seed: u64) -> super::CreditControls {
        let mut no_recurrence_gate = credit_gate(seed);
        let no_recurrence = !apply_recurrence(&mut no_recurrence_gate, false)
            && credit_state(&no_recurrence_gate).observations == 0;

        let mut immediate_path = PlasticityPath::default();
        let mut immediate_learner = ConsequenceLearner::default();
        let immediate_encounter = productive(seed + 1_000_000, 0, false);
        let immediate_active = immediate_encounter.snapshot();
        let immediate_other = contrast(seed + 1_000_000, 0, false).snapshot();
        let immediate_edge = immediate_path
            .encounter(immediate_encounter)
            .expect("blank M5 exploration admits the immediate control");
        let immediate_executed = immediate_path.execute(immediate_edge);
        let immediate_rejected = immediate_executed
            && !immediate_learner.apply(
                &mut immediate_path,
                immediate_active,
                immediate_other,
                raw(seed + 1_100_000, 0, 0, true),
            );

        let mut absent_path = PlasticityPath::default();
        let mut absent_learner = ConsequenceLearner::default();
        let absent_eligibility = !absent_learner.apply(
            &mut absent_path,
            productive(seed + 2_000_000, 0, false).snapshot(),
            contrast(seed + 2_000_000, 0, false).snapshot(),
            raw(seed + 2_100_000, 0, 0, false),
        );

        let first = productive(seed + 3_000_000, 0, false).snapshot();
        let second = contrast(seed + 3_000_000, 0, false).snapshot();
        let equal = {
            let mut learner = ConsequenceLearner::default();
            for episode in 0..8usize {
                let _ = learner.observe(first, raw(seed + 3_100_000, episode * 2, 0, false));
                let _ = learner.observe(second, raw(seed + 3_100_000, episode * 2 + 1, 1, false));
            }
            learner.direction([first, second]).is_none()
        };
        let shuffled = direction_for(seed + 4_000_000, true, true).is_none();
        let normal = direction_for(seed + 5_000_000, true, false)
            == Some(productive(seed + 5_000_000, 0, false).snapshot());
        let swapped = direction_for(seed + 6_000_000, false, false)
            == Some(contrast(seed + 6_000_000, 0, false).snapshot());

        super::CreditControls {
            no_recurrence,
            immediate_rejected,
            absent_eligibility,
            equal_and_shuffled_abstain: equal && shuffled,
            swapped_history_reverses: normal && swapped,
        }
    }
}

#[allow(dead_code)]
mod frozen_request {
    include!(concat!(env!("OUT_DIR"), "/ds4_request_roles.rs"));

    #[derive(Clone, Debug)]
    pub(super) struct Session {
        learner: RequestRoleLearner,
        roles: LearnedRoles,
        program: ChosenProgram,
        identities: IdentitySource,
        rng: DeterministicRng,
        episode: usize,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(super) struct Step {
        pub(super) selected: usize,
        pub(super) executed: usize,
        pub(super) recurrence: usize,
        pub(super) consequences: usize,
        pub(super) updates: usize,
        pub(super) from_occurrence: bool,
        pub(super) pre_output_trace: bool,
        pub(super) functional: bool,
        pub(super) explicit: bool,
        pub(super) quiescent: bool,
        pub(super) position: usize,
    }

    pub(super) fn session(seed: usize) -> Session {
        let (roles, program) = fixed_roles_and_program(seed);
        Session {
            learner: RequestRoleLearner::new(40, 0xA741_1000 + seed as u64),
            roles,
            program,
            identities: IdentitySource::new(0xA741_2000 + seed as u64),
            rng: DeterministicRng::new(0xA741_3000 + seed as u64),
            episode: 0,
        }
    }

    pub(super) fn step(
        session: &mut Session,
        credit: &mut super::frozen_m6::CreditGate,
        completion_activity: usize,
        acquire: bool,
        symmetric: bool,
    ) -> Step {
        let depth = if acquire {
            1 + session.episode % 4
        } else {
            [5, 8, 16, 32][session.episode % 4]
        };
        session.episode += 1;
        let episode = chain_episode(&mut session.identities, &mut session.rng, depth);
        let encoded = request_encoding(
            &mut session.identities,
            &mut session.rng,
            episode.target_request,
            if symmetric {
                RequestEncodingFamily::Symmetric
            } else if acquire {
                RequestEncodingFamily::Training
            } else {
                RequestEncodingFamily::Transferred
            },
        );
        if acquire {
            session.learner.observe(&encoded.occurrences);
        }
        if completion_activity == 0 {
            return Step {
                position: encoded.target_position,
                ..Step::default()
            };
        }

        // POST_M6_DS4_REQUEST_LINKER_BEGIN
        let choice = if acquire {
            session.learner.choose(&encoded.occurrences)
        } else {
            session.learner.evaluated(&encoded.occurrences)
        };
        let from_occurrence = match choice.outcome {
            BindingOutcome::Answer(selected) => encoded
                .occurrences
                .iter()
                .any(|occurrence| occurrence.identity == Some(selected)),
            BindingOutcome::NotFound | BindingOutcome::Ambiguous => false,
        };
        let selected = choice.pattern_cell.is_some();
        let run = execute_choice(&episode, &choice, session.roles, session.program);
        let recurrent_activity = run.used.len() > 2;
        let differential =
            acquire && selected && super::frozen_m6::apply_recurrence(credit, recurrent_activity);
        if differential {
            session.learner.feedback(choice.pattern_cell, true);
        }
        // POST_M6_DS4_REQUEST_LINKER_END

        let functional = run.outcome == BindingOutcome::Answer(episode.answer)
            && run.explicit_answer
            && run.queue_empty;
        Step {
            selected: usize::from(selected),
            executed: usize::from(selected),
            recurrence: usize::from(recurrent_activity),
            consequences: usize::from(acquire && recurrent_activity),
            updates: usize::from(differential),
            from_occurrence,
            pre_output_trace: choice.pre_answer_trace,
            functional,
            explicit: run.explicit_answer,
            quiescent: run.queue_empty,
            position: encoded.target_position,
        }
    }

    pub(super) fn ready(session: &Session) -> bool {
        session.learner.target_role(request_signature(0)).is_some()
    }

    pub(super) fn roles(session: &Session) -> usize {
        session.learner.consolidated_cells().len()
    }

    pub(super) fn fingerprint(session: &Session) -> u64 {
        session.learner.fingerprint()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub exact_m6: bool,
    pub immutable_old_negative: bool,
    pub frozen_probe_v1_negative: bool,
    pub frozen_probe_retry: bool,
    pub frozen_micro: bool,
    pub frozen_mechanisms: bool,
    pub protocol_frozen: bool,
    pub linker_exact: bool,
    pub information_boundary: bool,
    pub lane_isolated: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.exact_m6
            && self.immutable_old_negative
            && self.frozen_probe_v1_negative
            && self.frozen_probe_retry
            && self.frozen_micro
            && self.frozen_mechanisms
            && self.protocol_frozen
            && self.linker_exact
            && self.information_boundary
            && self.lane_isolated
    }
}

fn source_audit() -> SourceAudit {
    let source = include_str!("post_m6_ds4_arrival_initiation.rs");
    let request_linker = source
        .split("// POST_M6_DS4_REQUEST_LINKER_BEGIN")
        .nth(1)
        .and_then(|tail| tail.split("// POST_M6_DS4_REQUEST_LINKER_END").next())
        .unwrap_or_default();
    let m6_linker = source
        .split("// POST_M6_DS4_GATE_BEGIN")
        .nth(1)
        .and_then(|tail| tail.split("// POST_M6_DS4_GATE_END").next())
        .unwrap_or_default();
    let boundary_forbidden = [
        ["func", "tional"].concat(),
        ["episode", ".answer"].concat(),
        ["expected", "_answer"].concat(),
        ["semantic", "_credit"].concat(),
        ["start", "_flag"].concat(),
        ["target", "_identity"].concat(),
    ];
    let m6 = crate::ds8_cumulative_semantic_credit_definitive::source_preflight(true, true);
    SourceAudit {
        exact_m6: AUTHORITATIVE_M6 == "aa4e22efd8a65b7694956a53cfaa970582695215"
            && env!("POST_M6_DS4_M6_CSV_SHA256") == FROZEN_M6_CSV_SHA256
            && env!("POST_M6_DS4_M6_MD_SHA256") == FROZEN_M6_MD_SHA256
            && env!("POST_M6_DS4_M6_HANDOFF_SHA256") == FROZEN_M6_HANDOFF_SHA256
            && m6.source.passed(),
        immutable_old_negative: env!("POST_M6_DS4_OLD_NEGATIVE_CSV_SHA256")
            == FROZEN_OLD_NEGATIVE_CSV_SHA256
            && env!("POST_M6_DS4_OLD_NEGATIVE_MD_SHA256") == FROZEN_OLD_NEGATIVE_MD_SHA256,
        frozen_probe_v1_negative: env!("POST_M6_DS4_PROBE_V1_RESULT_SHA256")
            == FROZEN_PROBE_V1_RESULT_SHA256
            && env!("POST_M6_DS4_PROBE_V1_AUDIT_SHA256") == FROZEN_PROBE_V1_AUDIT_SHA256,
        frozen_probe_retry: env!("POST_M6_DS4_PROBE_RETRY_RESULT_SHA256")
            == FROZEN_PROBE_RETRY_RESULT_SHA256
            && env!("POST_M6_DS4_PROBE_HANDOFF_SHA256") == FROZEN_PROBE_HANDOFF_SHA256,
        frozen_micro: env!("POST_M6_DS4_MICRO_RESULT_SHA256") == FROZEN_MICRO_RESULT_SHA256
            && env!("POST_M6_DS4_MICRO_HANDOFF_SHA256") == FROZEN_MICRO_HANDOFF_SHA256,
        frozen_mechanisms: env!("DS4_M3_PORT_SHA256") == FROZEN_M3_SHA256
            && env!("DS4_P4_SHA256") == FROZEN_P4_SHA256
            && env!("DS8_M5_ALLOCATOR_SHA256") == FROZEN_M5_SHA256
            && env!("DS8_MICRO_LINKER_FRAGMENT_SHA256") == FROZEN_M6_LINKER_SHA256,
        protocol_frozen: env!("POST_M6_DS4_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        linker_exact: !request_linker.is_empty()
            && !m6_linker.is_empty()
            && request_linker.matches("apply_recurrence").count() == 1
            && request_linker
                .matches("learner.feedback(choice.pattern_cell, true)")
                .count()
                == 1
            && m6_linker.matches("learner.apply").count() == 1,
        information_boundary: boundary_forbidden
            .iter()
            .all(|item| !request_linker.contains(item) && !m6_linker.contains(item)),
        lane_isolated: !request_linker.contains("ssa")
            && !m6_linker.contains("ssa")
            && !request_linker.contains("stochastic")
            && !m6_linker.contains("stochastic"),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Control {
    pub number: usize,
    pub name: &'static str,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Snapshot {
    source: SourceAudit,
    learners: usize,
    ready: usize,
    single_roles: usize,
    competence: Vec<usize>,
    learned_event_activity: usize,
    selections: usize,
    recurrences: usize,
    consequences: usize,
    updates: usize,
    credit_observations: u64,
    held_out_correct: usize,
    held_out_total: usize,
    explicit: usize,
    quiescent: usize,
    positions: std::collections::BTreeSet<usize>,
    physical_work: u64,
    m3_nonplastic: bool,
    p4_nonplastic: bool,
    m6_nonplastic: bool,
    acquisition: std::collections::BTreeSet<u64>,
    held_out: std::collections::BTreeSet<u64>,
    controls: Vec<Control>,
}

fn snapshot(seeds: &[u64], held_out_per_learner: usize) -> Snapshot {
    use frozen_pre_m6_ds4::{ArrivalActivity, ArrivalFixture};

    let source = source_audit();
    let mut ready = 0;
    let mut single_roles = 0;
    let mut competence = Vec::new();
    let mut learned_event_activity = 0;
    let mut selections = 0;
    let mut recurrences = 0;
    let mut consequences = 0;
    let mut updates = 0;
    let mut credit_observations = 0;
    let mut held_out_correct = 0;
    let held_out_total = seeds.len() * held_out_per_learner;
    let mut explicit = 0;
    let mut quiescent = 0;
    let mut positions = std::collections::BTreeSet::new();
    let mut physical_work = 0;
    let mut m3_nonplastic = true;
    let mut p4_nonplastic = true;
    let mut m6_nonplastic = true;
    let mut acquisition = std::collections::BTreeSet::new();
    let mut held_out = std::collections::BTreeSet::new();
    let mut no_arrival = true;
    let mut subthreshold = true;
    let mut invalid_and_reentry = true;
    let mut fresh_transfer = true;
    let mut occurrence_only = true;
    let mut symmetric_rejected = true;

    for &seed in seeds {
        acquisition.insert(seed);
        acquisition.insert(seed + 1);
        let Some(mut event_gate) = frozen_pre_m6_ds4::arrival_gate(seed, 2) else {
            continue;
        };
        let mut request = frozen_request::session(seed as usize);
        let mut credit = frozen_m6::credit_gate(seed);
        let mut learned_at = None;
        for episode in 1..=4_000usize {
            let event_seed = seed + 1_000 + episode as u64;
            acquisition.insert(event_seed);
            let activity = frozen_pre_m6_ds4::arrival_activity(
                &mut event_gate,
                event_seed,
                ArrivalFixture::Standard,
            );
            let step =
                frozen_request::step(&mut request, &mut credit, activity.completion, true, false);
            learned_event_activity += activity.learned;
            selections += step.selected;
            recurrences += step.recurrence;
            consequences += step.consequences;
            updates += step.updates;
            physical_work += activity.work;
            occurrence_only &= step.selected == 0 || step.from_occurrence;
            if frozen_request::ready(&request) {
                learned_at = Some(episode);
                break;
            }
        }
        if let Some(episode) = learned_at {
            ready += 1;
            competence.push(episode);
        }
        single_roles += usize::from(frozen_request::roles(&request) == 1);
        credit_observations += frozen_m6::credit_state(&credit).observations;

        let before_m3 = frozen_pre_m6_ds4::arrival_state(&event_gate);
        let before_p4 = frozen_request::fingerprint(&request);
        let before_m6 = frozen_m6::credit_state(&credit);
        for held_index in 0..held_out_per_learner {
            let event_seed = seed + 100_000 + held_index as u64;
            held_out.insert(event_seed);
            let fixture = if held_index.is_multiple_of(2) {
                ArrivalFixture::Standard
            } else {
                ArrivalFixture::Relabelled
            };
            let activity =
                frozen_pre_m6_ds4::arrival_activity(&mut event_gate, event_seed, fixture);
            fresh_transfer &= activity.completion > 0 && activity.learned == activity.generic;
            let step =
                frozen_request::step(&mut request, &mut credit, activity.completion, false, false);
            held_out_correct += usize::from(step.functional);
            explicit += usize::from(step.explicit);
            quiescent += usize::from(step.quiescent);
            positions.insert(step.position);
            occurrence_only &= step.from_occurrence;
            learned_event_activity += activity.learned;
            selections += step.selected;
            recurrences += step.recurrence;
            physical_work += activity.work;
        }
        m3_nonplastic &= before_m3 == frozen_pre_m6_ds4::arrival_state(&event_gate);
        p4_nonplastic &= before_p4 == frozen_request::fingerprint(&request);
        m6_nonplastic &= before_m6 == frozen_m6::credit_state(&credit);

        let no_arrival_step = frozen_request::step(&mut request, &mut credit, 0, false, false);
        no_arrival &= no_arrival_step.selected == 0
            && no_arrival_step.recurrence == 0
            && no_arrival_step.consequences == 0
            && no_arrival_step.updates == 0;

        let mut weak_gate = frozen_pre_m6_ds4::arrival_gate(seed + 200_000, 1);
        let weak = weak_gate
            .as_mut()
            .map_or(ArrivalActivity::default(), |gate| {
                frozen_pre_m6_ds4::arrival_activity(gate, seed + 200_100, ArrivalFixture::Standard)
            });
        let mut weak_request = frozen_request::session((seed + 200_000) as usize);
        let mut weak_credit = frozen_m6::credit_gate(seed + 200_000);
        let weak_step = frozen_request::step(
            &mut weak_request,
            &mut weak_credit,
            weak.completion,
            false,
            false,
        );
        subthreshold &= weak.generic > 0
            && weak.learned == 0
            && weak.completion == 0
            && weak_step.selected == 0
            && weak_step.updates == 0;

        let missing = frozen_pre_m6_ds4::arrival_activity(
            &mut event_gate,
            seed + 210_000,
            ArrivalFixture::MissingClose,
        );
        let invalid = frozen_pre_m6_ds4::arrival_activity(
            &mut event_gate,
            seed + 220_000,
            ArrivalFixture::InvalidTransition,
        );
        let reentry = frozen_pre_m6_ds4::arrival_activity(
            &mut event_gate,
            seed + 220_001,
            ArrivalFixture::Standard,
        );
        invalid_and_reentry &=
            missing.completion == 0 && invalid.completion == 0 && reentry.completion > 0;

        let Some(mut symmetric_event) = frozen_pre_m6_ds4::arrival_gate(seed + 300_000, 2) else {
            symmetric_rejected = false;
            continue;
        };
        let mut symmetric_request = frozen_request::session((seed + 300_000) as usize);
        let mut symmetric_credit = frozen_m6::credit_gate(seed + 300_000);
        for episode in 0..128usize {
            let activity = frozen_pre_m6_ds4::arrival_activity(
                &mut symmetric_event,
                seed + 301_000 + episode as u64,
                ArrivalFixture::Standard,
            );
            let _ = frozen_request::step(
                &mut symmetric_request,
                &mut symmetric_credit,
                activity.completion,
                true,
                true,
            );
        }
        symmetric_rejected &= !frozen_request::ready(&symmetric_request)
            && frozen_request::roles(&symmetric_request) == 0
            && frozen_m6::credit_state(&symmetric_credit).observations == 0;
    }

    let credit_controls = frozen_m6::controls(seeds.first().copied().unwrap_or(PROBE_RETRY_SEED));
    let controls = vec![
        Control {
            number: 1,
            name: "no-arrival",
            passed: no_arrival,
        },
        Control {
            number: 2,
            name: "learned-event-required",
            passed: subthreshold,
        },
        Control {
            number: 3,
            name: "missing-invalid-reentry",
            passed: invalid_and_reentry,
        },
        Control {
            number: 4,
            name: "no-recurrence-no-consequence",
            passed: credit_controls.no_recurrence,
        },
        Control {
            number: 5,
            name: "immediate-rejected",
            passed: credit_controls.immediate_rejected,
        },
        Control {
            number: 6,
            name: "active-eligibility-required",
            passed: credit_controls.absent_eligibility,
        },
        Control {
            number: 7,
            name: "equal-shuffled-abstain",
            passed: credit_controls.equal_and_shuffled_abstain,
        },
        Control {
            number: 8,
            name: "raw-history-swap",
            passed: credit_controls.swapped_history_reverses,
        },
        Control {
            number: 9,
            name: "fresh-identity-layout-allocation",
            passed: fresh_transfer,
        },
        Control {
            number: 10,
            name: "occurrence-only-selection",
            passed: occurrence_only && symmetric_rejected,
        },
        Control {
            number: 11,
            name: "source-information-lane-boundary",
            passed: source.passed(),
        },
        Control {
            number: 12,
            name: "nonplastic-disjoint-deterministic",
            passed: m3_nonplastic
                && p4_nonplastic
                && m6_nonplastic
                && acquisition.is_disjoint(&held_out)
                && credit_controls.passed(),
        },
    ];

    Snapshot {
        source,
        learners: seeds.len(),
        ready,
        single_roles,
        competence,
        learned_event_activity,
        selections,
        recurrences,
        consequences,
        updates,
        credit_observations,
        held_out_correct,
        held_out_total,
        explicit,
        quiescent,
        positions,
        physical_work,
        m3_nonplastic,
        p4_nonplastic,
        m6_nonplastic,
        acquisition,
        held_out,
        controls,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DevelopmentReport {
    pub protocol: &'static str,
    pub mode: &'static str,
    pub claim_eligible: bool,
    pub development_ready: bool,
    pub m6_authoritative: bool,
    pub m7_exists: bool,
    pub ds5_eligible: bool,
    pub source: SourceAudit,
    pub first_collapse: &'static str,
    pub learners: usize,
    pub ready_learners: usize,
    pub single_role_learners: usize,
    pub average_competence_millis: u64,
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
    pub m3_nonplastic: bool,
    pub p4_nonplastic: bool,
    pub m6_nonplastic: bool,
    pub duplicate_exact: bool,
    pub controls: Vec<Control>,
}

fn report(mode: &'static str, seeds: &[u64], held_out: usize, full: bool) -> DevelopmentReport {
    let first = snapshot(seeds, held_out);
    let second = snapshot(seeds, held_out);
    let duplicate_exact = first == second;
    let source_ready = first.source.passed();
    let physical_path = first.learned_event_activity > 0
        && first.selections > 0
        && first.recurrences > 0
        && first.consequences > 0
        && first.credit_observations > 0
        && first.updates > 0;
    let acquisition_ready = first.ready == first.learners
        && first.single_roles == first.learners
        && first.competence.len() == first.learners;
    let transfer_ready = !full
        || (first.held_out_correct == first.held_out_total
            && first.explicit == first.held_out_total
            && first.quiescent == first.held_out_total
            && first.positions.len() == 6);
    let controls_ready = !full
        || (first.controls.len() == 12 && first.controls.iter().all(|control| control.passed));
    let lifecycle_ready = duplicate_exact
        && first.physical_work > 0
        && first.m3_nonplastic
        && first.p4_nonplastic
        && first.m6_nonplastic
        && first.acquisition.is_disjoint(&first.held_out);
    let readiness = [
        source_ready,
        physical_path,
        acquisition_ready,
        transfer_ready,
        controls_ready,
        lifecycle_ready,
    ];
    let first_collapse = readiness
        .iter()
        .position(|ready| !ready)
        .map_or("NONE", |index| {
            [
                "P0 exact M6/source/negative authority",
                "P1-P4 physical initiation and M6 active-trace path",
                "P5 occurrence-role acquisition",
                "P6 held-out transfer",
                "P7 controls",
                "P7 lifecycle/determinism",
            ][index]
        });
    let competence_sum = first.competence.iter().sum::<usize>() as u64;
    let competence_count = first.competence.len() as u64;
    DevelopmentReport {
        protocol: PROTOCOL,
        mode,
        claim_eligible: false,
        development_ready: readiness.iter().all(|ready| *ready),
        m6_authoritative: true,
        m7_exists: false,
        ds5_eligible: false,
        source: first.source,
        first_collapse,
        learners: first.learners,
        ready_learners: first.ready,
        single_role_learners: first.single_roles,
        average_competence_millis: (competence_sum * 1_000)
            .checked_div(competence_count)
            .unwrap_or(0),
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
        positions: first.positions.len(),
        physical_work: first.physical_work,
        m3_nonplastic: first.m3_nonplastic,
        p4_nonplastic: first.p4_nonplastic,
        m6_nonplastic: first.m6_nonplastic,
        duplicate_exact,
        controls: first.controls,
    }
}

pub fn run_probe_retry() -> DevelopmentReport {
    report("PROBE-RETRY", &[PROBE_RETRY_SEED], 0, false)
}

pub fn run_development(mode: HarnessMode) -> DevelopmentReport {
    match mode {
        HarnessMode::Micro => report("MICRO", &MICRO_SEEDS, 8, true),
        HarnessMode::Gate => report("GATE", &GATE_SEEDS, 32, true),
        HarnessMode::Definitive => DevelopmentReport {
            protocol: PROTOCOL,
            mode: "DEFINITIVE-FORBIDDEN",
            claim_eligible: false,
            development_ready: false,
            m6_authoritative: true,
            m7_exists: false,
            ds5_eligible: false,
            source: source_audit(),
            first_collapse: "definitive rejected before learner or seed construction",
            learners: 0,
            ready_learners: 0,
            single_role_learners: 0,
            average_competence_millis: 0,
            learned_event_activity: 0,
            selections: 0,
            recurrences: 0,
            consequences: 0,
            updates: 0,
            credit_observations: 0,
            held_out_correct: 0,
            held_out_total: 0,
            explicit: 0,
            quiescent: 0,
            positions: 0,
            physical_work: 0,
            m3_nonplastic: false,
            p4_nonplastic: false,
            m6_nonplastic: false,
            duplicate_exact: false,
            controls: Vec::new(),
        },
    }
}

pub fn definitive_rejected() -> bool {
    let report = run_development(HarnessMode::Definitive);
    report.mode == "DEFINITIVE-FORBIDDEN"
        && report.learners == 0
        && !report.claim_eligible
        && !report.development_ready
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_v1_freezes_the_expected_first_collapse() {
        let report = run_probe_v1();
        assert!(report.expected_negative, "{report:#?}");
        assert!(report.physical_arrival_path);
        assert_eq!(report.semantic_feedback_calls, 1);
        assert_eq!(report.m6_differential_links, 0);
        assert_eq!(report.lawful_updates, 0);
        assert!(!report.claim_eligible);
    }

    #[test]
    fn definitive_is_inert() {
        assert!(definitive_rejected());
    }

    #[test]
    fn probe_retry_closes_only_the_preregistered_gate() {
        let report = run_probe_retry();
        assert!(report.development_ready, "{report:#?}");
        assert!(report.updates > 0);
        assert!(report.credit_observations > 0);
        assert!(!report.claim_eligible && report.m6_authoritative && !report.m7_exists);
    }

    #[test]
    fn micro_is_development_only_and_conjunctive() {
        let report = run_development(HarnessMode::Micro);
        assert!(report.development_ready, "{report:#?}");
        assert_eq!(report.held_out_correct, report.held_out_total);
        assert!(report.controls.iter().all(|control| control.passed));
        assert!(!report.claim_eligible && !report.ds5_eligible);
    }
}
