//! Development-only cumulative DS4 request/start linker over authoritative M3.
//!
//! The M3 cumulative port and P4 request-role mechanism are included from
//! byte-identical composition copies. This module closes only the physical
//! M3-completion-to-P4-selection edge and supplies development probes.

use crate::research_runtime::HarnessMode;
use std::collections::BTreeSet;

pub const PROTOCOL: &str = "ds4-cumulative-request-start-v1";
pub const AUTHORITATIVE_M3: &str = "ffcdfe8b36fc62348b7ebcb09aaf4797f6146ba8";
pub const FROZEN_P4_COMMIT: &str = "51cf918e0b6eda77ccef6386ff1150db42cea6fd";
pub const TARGET_FREEZE_COMMIT: &str = "fcfd0413da37eac6eeb1a49c6f387e090229e605";
pub const PROTOCOL_V2_COMMIT: &str = "0e866f6bdde3dc52442c1023cc247f20c18d38aa";
pub const FROZEN_M3_PORT_SHA256: &str =
    "c4fc7aca11a5925effeb5a84b90184a70da0f66da7c063d0f87ba46ca36addf3";
pub const FROZEN_M3_DEFINITIVE_SHA256: &str =
    "d4c3ea6e671d1812e35e34ef7fa46a77f6a577c9e6a1d77d2c30e7d570017840";
pub const FROZEN_M3_RESULT_CSV_SHA256: &str =
    "ac8c0a6c9b7badfa263ceb054ffe59c11162b1ca256c56cc6df5f0d378179401";
pub const FROZEN_M3_RESULT_MD_SHA256: &str =
    "ab77bd12b705b8620b6315260f8bb5b4df6efc961f1d20a0dd521af403e1ac5f";
pub const FROZEN_P4_SHA256: &str =
    "2dbde723b394bcb3d788c796aa1745cd1cea392a64ab61497bb97474866144b8";
pub const FROZEN_P4_RESULT_CSV_SHA256: &str =
    "b1e8ee07be2fa425e7ec5cdfee54ea77abde97d0160a77ca6b5b550126d46c5d";
pub const FROZEN_P4_RESULT_MD_SHA256: &str =
    "ced485b93072e4ffba6b1889175be130dcda27926b6b8685b4e6d3ade31cc2bd";
pub const FROZEN_TARGET_SHA256: &str =
    "f10f9d7b16106b6014767ff6188a6d556145ba3e5b4335e28de245c7622a7595";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "a1460c0d30f55edb16888ef4c93d119586cf24fe206cb3a7362c08cee5187e95";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EventFixture {
    Standard,
    Relabelled,
    MissingClose,
    InvalidTransition,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct EventActivity {
    completion_spikes: usize,
    generic_spans: usize,
    learned_uses: usize,
    physical_work: u64,
    chunks: usize,
    persistent_bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct EventState {
    chunks: usize,
    persistent_bytes: usize,
}

macro_rules! ds4_m3_access {
    () => {
        pub(super) struct Ds4EventGate {
            learner: frozen_ds3::GlueBoundary,
        }

        pub(super) fn ds4_event_gate(seed: u64, acquisition: usize) -> Option<Ds4EventGate> {
            let mut learner = frozen_ds3::glue_default_boundary();
            for episode in 0..acquisition {
                let stream = standard_stream(seed + episode as u64, RenderOptions::default())?;
                let evaluation =
                    frozen_ds3::glue_evaluate(&mut learner, &stream.observations, true);
                if !stream_legal(&stream)
                    || !exact_reconstruction(&evaluation, &stream)
                    || !consequence_parity(&evaluation, &stream)
                {
                    return None;
                }
            }
            Some(Ds4EventGate { learner })
        }

        pub(super) fn event_completion_activity(
            gate: &mut Ds4EventGate,
            seed: u64,
            fixture: super::EventFixture,
        ) -> super::EventActivity {
            let stream = match fixture {
                super::EventFixture::Standard => standard_stream(seed, RenderOptions::default()),
                super::EventFixture::Relabelled => standard_stream(
                    seed,
                    RenderOptions {
                        shape_xor: 0xA7,
                        consequence_delta: 31,
                        reverse_time: true,
                        relabel: true,
                        reverse_allocation: true,
                        ..RenderOptions::default()
                    },
                ),
                super::EventFixture::MissingClose => {
                    let mut stream = Stream::default();
                    let mut occurrences = seed ^ 0xD540_0001;
                    append_lifecycle(
                        &mut stream,
                        seed,
                        0,
                        2,
                        RenderOptions::default(),
                        &mut occurrences,
                    )
                    .then_some(stream)
                }
                super::EventFixture::InvalidTransition => {
                    let Some(mut stream) = standard_stream(seed, RenderOptions::default()) else {
                        return super::EventActivity::default();
                    };
                    if stream.observations.len() < 2 {
                        return super::EventActivity::default();
                    }
                    stream.observations[1].causal_link = CausalLink::Reset;
                    Some(stream)
                }
            };
            let Some(stream) = stream else {
                return super::EventActivity::default();
            };
            let evaluation =
                frozen_ds3::glue_evaluate(&mut gate.learner, &stream.observations, false);
            let learned_complete = !evaluation.spans.is_empty()
                && evaluation.used_learned == evaluation.spans.len()
                && exact_reconstruction(&evaluation, &stream)
                && consequence_parity(&evaluation, &stream);
            super::EventActivity {
                completion_spikes: if learned_complete {
                    evaluation.spans.len()
                } else {
                    0
                },
                generic_spans: evaluation.spans.len(),
                learned_uses: evaluation.used_learned,
                physical_work: stream.m2_work
                    + evaluation.work.generic_transition_checks
                    + evaluation.work.learned_signature_checks
                    + evaluation.work.completed_spans
                    + evaluation.work.propagated_consequences,
                chunks: frozen_ds3::glue_chunk_count(&gate.learner),
                persistent_bytes: frozen_ds3::glue_persistent_bytes(&gate.learner),
            }
        }

        pub(super) fn ds4_m3_source_ok() -> bool {
            source_audit().passed()
        }

        pub(super) fn ds4_event_state(gate: &Ds4EventGate) -> super::EventState {
            super::EventState {
                chunks: frozen_ds3::glue_chunk_count(&gate.learner),
                persistent_bytes: frozen_ds3::glue_persistent_bytes(&gate.learner),
            }
        }
    };
}

#[allow(dead_code)]
mod frozen_m3 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds4_m3_cumulative_event_boundary_port.rs"
    ));
    ds4_m3_access!();
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RequestStep {
    selection_activations: usize,
    execution_activations: usize,
    update_activations: usize,
    selected_from_occurrence: bool,
    pre_answer_trace: bool,
    functional: bool,
    explicit_answer: bool,
    queue_empty: bool,
    target_position: usize,
}

macro_rules! ds4_p4_access {
    () => {
        pub(super) struct Ds4RequestSession {
            learner: RequestRoleLearner,
            roles: LearnedRoles,
            program: ChosenProgram,
            identities: IdentitySource,
            rng: DeterministicRng,
            episode: usize,
        }

        pub(super) fn ds4_request_session(seed: usize) -> Ds4RequestSession {
            let (roles, program) = fixed_roles_and_program(seed);
            Ds4RequestSession {
                learner: RequestRoleLearner::new(40, 0xD540_1000 + seed as u64),
                roles,
                program,
                identities: IdentitySource::new(0xD540_2000 + seed as u64),
                rng: DeterministicRng::new(0xD540_3000 + seed as u64),
                episode: 0,
            }
        }

        // DS4_LINKER_START
        pub(super) fn activate_request_selection(
            session: &mut Ds4RequestSession,
            completion_spikes: usize,
            acquire: bool,
        ) -> super::RequestStep {
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
                if acquire {
                    RequestEncodingFamily::Training
                } else {
                    RequestEncodingFamily::Transferred
                },
            );
            if acquire {
                session.learner.observe(&encoded.occurrences);
            }
            if completion_spikes == 0 {
                return super::RequestStep {
                    target_position: encoded.target_position,
                    ..super::RequestStep::default()
                };
            }
            let choice = if acquire {
                session.learner.choose(&encoded.occurrences)
            } else {
                session.learner.evaluated(&encoded.occurrences)
            };
            let selected_from_occurrence = match choice.outcome {
                BindingOutcome::Answer(selected) => encoded
                    .occurrences
                    .iter()
                    .any(|occurrence| occurrence.identity == Some(selected)),
                BindingOutcome::NotFound | BindingOutcome::Ambiguous => false,
            };
            let selected = choice.pattern_cell.is_some();
            let run = execute_choice(&episode, &choice, session.roles, session.program);
            let functional = run.outcome == BindingOutcome::Answer(episode.answer)
                && run.explicit_answer
                && run.queue_empty;
            if acquire {
                session.learner.feedback(choice.pattern_cell, functional);
            }
            super::RequestStep {
                selection_activations: usize::from(selected),
                execution_activations: usize::from(selected),
                update_activations: usize::from(acquire && selected),
                selected_from_occurrence,
                pre_answer_trace: choice.pre_answer_trace,
                functional,
                explicit_answer: run.explicit_answer,
                queue_empty: run.queue_empty,
                target_position: encoded.target_position,
            }
        }
        // DS4_LINKER_END

        pub(super) fn ds4_symmetric_step(
            session: &mut Ds4RequestSession,
            completion_spikes: usize,
        ) -> super::RequestStep {
            let episode = chain_episode(&mut session.identities, &mut session.rng, 1);
            let encoded = request_encoding(
                &mut session.identities,
                &mut session.rng,
                episode.target_request,
                RequestEncodingFamily::Symmetric,
            );
            session.learner.observe(&encoded.occurrences);
            if completion_spikes == 0 {
                return super::RequestStep {
                    target_position: encoded.target_position,
                    ..super::RequestStep::default()
                };
            }
            let choice = session.learner.choose(&encoded.occurrences);
            let selected = choice.pattern_cell.is_some();
            let run = execute_choice(&episode, &choice, session.roles, session.program);
            let functional = run.outcome == BindingOutcome::Answer(episode.answer)
                && run.explicit_answer
                && run.queue_empty;
            session.learner.feedback(choice.pattern_cell, false);
            super::RequestStep {
                selection_activations: usize::from(selected),
                execution_activations: usize::from(selected),
                update_activations: usize::from(selected),
                selected_from_occurrence: matches!(choice.outcome, BindingOutcome::Answer(_)),
                pre_answer_trace: choice.pre_answer_trace,
                functional,
                explicit_answer: run.explicit_answer,
                queue_empty: run.queue_empty,
                target_position: encoded.target_position,
            }
        }

        pub(super) fn ds4_request_ready(session: &Ds4RequestSession) -> bool {
            session.learner.target_role(request_signature(0)).is_some()
        }

        pub(super) fn ds4_request_fingerprint(session: &Ds4RequestSession) -> u64 {
            session.learner.fingerprint()
        }

        pub(super) fn ds4_request_role_count(session: &Ds4RequestSession) -> usize {
            session.learner.consolidated_cells().len()
        }

        pub(super) fn ds4_p4_persistent_state_ok() -> bool {
            let source = include_str!("request_roles.rs");
            let persistent = source
                .split("struct RequestPattern {")
                .nth(1)
                .and_then(|tail| tail.split("struct RequestRoleLearner").next())
                .unwrap_or("");
            !persistent.is_empty()
                && !persistent.contains("OpaqueId")
                && !persistent.contains("receptor")
                && !persistent.contains("target_position")
        }
    };
}

#[allow(dead_code)]
mod frozen_p4 {
    include!(concat!(env!("OUT_DIR"), "/ds4_request_roles.rs"));
    ds4_p4_access!();
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub authoritative_m3: bool,
    pub m3_port_hash: bool,
    pub m3_definitive_hash: bool,
    pub m3_result_hashes: bool,
    pub p4_hash: bool,
    pub p4_result_hashes: bool,
    pub target_hash: bool,
    pub protocol_hash: bool,
    pub m3_source: bool,
    pub p4_persistent_state: bool,
    pub linker_boundary: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.authoritative_m3
            && self.m3_port_hash
            && self.m3_definitive_hash
            && self.m3_result_hashes
            && self.p4_hash
            && self.p4_result_hashes
            && self.target_hash
            && self.protocol_hash
            && self.m3_source
            && self.p4_persistent_state
            && self.linker_boundary
    }
}

fn source_audit() -> SourceAudit {
    let source = include_str!("ds4_cumulative_request_start_port.rs");
    let linker = source
        .split("// DS4_LINKER_START")
        .nth(1)
        .and_then(|tail| tail.split("// DS4_LINKER_END").next())
        .unwrap_or("");
    SourceAudit {
        authoritative_m3: AUTHORITATIVE_M3 == "ffcdfe8b36fc62348b7ebcb09aaf4797f6146ba8",
        m3_port_hash: env!("DS4_M3_PORT_SHA256") == FROZEN_M3_PORT_SHA256,
        m3_definitive_hash: env!("DS4_M3_DEFINITIVE_SHA256") == FROZEN_M3_DEFINITIVE_SHA256,
        m3_result_hashes: env!("DS4_M3_RESULT_CSV_SHA256") == FROZEN_M3_RESULT_CSV_SHA256
            && env!("DS4_M3_RESULT_MD_SHA256") == FROZEN_M3_RESULT_MD_SHA256,
        p4_hash: env!("DS4_P4_SHA256") == FROZEN_P4_SHA256,
        p4_result_hashes: env!("DS4_P4_RESULT_CSV_SHA256") == FROZEN_P4_RESULT_CSV_SHA256
            && env!("DS4_P4_RESULT_MD_SHA256") == FROZEN_P4_RESULT_MD_SHA256,
        target_hash: env!("DS4_TARGET_FREEZE_SHA256") == FROZEN_TARGET_SHA256,
        protocol_hash: env!("DS4_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        m3_source: frozen_m3::ds4_m3_source_ok(),
        p4_persistent_state: frozen_p4::ds4_p4_persistent_state_ok(),
        linker_boundary: !linker.is_empty()
            && linker.contains("completion_spikes")
            && !linker.contains("expected_answer")
            && !linker.contains("target_identity")
            && !linker.contains("start_opcode"),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeReport {
    pub label: String,
    pub protocol: String,
    pub claim_eligible: bool,
    pub path_exists: bool,
    pub source: SourceAudit,
    pub learned_m3_uses: usize,
    pub completion_activity: usize,
    pub request_selection_activations: usize,
    pub request_execution_activations: usize,
    pub request_update_activations: usize,
    pub selected_from_occurrence: bool,
    pub pre_answer_trace: bool,
    pub m3_physical_work: u64,
    pub no_event_selection: usize,
    pub no_event_execution: usize,
    pub no_event_update: usize,
    pub request_fingerprint_after: u64,
}

pub fn run_probe() -> ProbeReport {
    let source = source_audit();
    let mut event_gate = frozen_m3::ds4_event_gate(94_000, 2);
    let mut request = frozen_p4::ds4_request_session(94_000);
    let activity = event_gate
        .as_mut()
        .map_or_else(EventActivity::default, |gate| {
            frozen_m3::event_completion_activity(gate, 94_100, EventFixture::Standard)
        });
    let step =
        frozen_p4::activate_request_selection(&mut request, activity.completion_spikes, true);
    let no_event = frozen_p4::activate_request_selection(&mut request, 0, false);
    let path_exists = source.passed()
        && activity.learned_uses > 0
        && activity.completion_spikes > 0
        && step.selection_activations > 0
        && step.execution_activations > 0
        && step.update_activations > 0
        && step.selected_from_occurrence
        && step.pre_answer_trace
        && activity.physical_work > 0
        && no_event.selection_activations == 0
        && no_event.execution_activations == 0
        && no_event.update_activations == 0;
    ProbeReport {
        label: if path_exists {
            "DS4 PROBE PATH EXISTS".to_string()
        } else {
            "DS4 PROBE COLLAPSE".to_string()
        },
        protocol: PROTOCOL.to_string(),
        claim_eligible: false,
        path_exists,
        source,
        learned_m3_uses: activity.learned_uses,
        completion_activity: activity.completion_spikes,
        request_selection_activations: step.selection_activations,
        request_execution_activations: step.execution_activations,
        request_update_activations: step.update_activations,
        selected_from_occurrence: step.selected_from_occurrence,
        pre_answer_trace: step.pre_answer_trace,
        m3_physical_work: activity.physical_work,
        no_event_selection: no_event.selection_activations,
        no_event_execution: no_event.execution_activations,
        no_event_update: no_event.update_activations,
        request_fingerprint_after: frozen_p4::ds4_request_fingerprint(&request),
    }
}

const STAGES: [&str; 6] = [
    "P0 frozen-source audit",
    "P1 physical initiation path",
    "P2 learned request acquisition",
    "P3 held-out functional transfer",
    "P4 controls 1-12",
    "P5 determinism, work, and lifecycle",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlResult {
    pub number: usize,
    pub name: &'static str,
    pub passed: bool,
    pub diagnostic: String,
}

fn control(
    number: usize,
    name: &'static str,
    passed: bool,
    diagnostic: impl Into<String>,
) -> ControlResult {
    ControlResult {
        number,
        name,
        passed,
        diagnostic: diagnostic.into(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DevelopmentSnapshot {
    source: SourceAudit,
    learner_count: usize,
    ready_learners: usize,
    single_role_learners: usize,
    competence_episodes: Vec<usize>,
    m3_learned_uses: usize,
    completion_activity: usize,
    selection_activations: usize,
    execution_activations: usize,
    update_activations: usize,
    m3_physical_work: u64,
    held_out_correct: usize,
    held_out_total: usize,
    explicit_answers: usize,
    queues_empty: usize,
    request_positions: BTreeSet<usize>,
    p4_nonplastic: bool,
    m3_nonplastic: bool,
    acquisition_seeds: BTreeSet<u64>,
    held_out_seeds: BTreeSet<u64>,
    controls: Vec<ControlResult>,
}

fn development_snapshot(
    base_seed: u64,
    learner_count: usize,
    held_out_per_learner: usize,
) -> DevelopmentSnapshot {
    let source = source_audit();
    let mut ready_learners = 0usize;
    let mut single_role_learners = 0usize;
    let mut competence_episodes = Vec::new();
    let mut m3_learned_uses = 0usize;
    let mut completion_activity = 0usize;
    let mut selection_activations = 0usize;
    let mut execution_activations = 0usize;
    let mut update_activations = 0usize;
    let mut m3_physical_work = 0u64;
    let mut held_out_correct = 0usize;
    let held_out_total = learner_count * held_out_per_learner;
    let mut explicit_answers = 0usize;
    let mut queues_empty = 0usize;
    let mut request_positions = BTreeSet::new();
    let mut p4_nonplastic = true;
    let mut m3_nonplastic = true;
    let mut acquisition_seeds = BTreeSet::new();
    let mut held_out_seeds = BTreeSet::new();
    let mut learned_event_required = true;
    let mut subthreshold_rejected = true;
    let mut missing_close_rejected = true;
    let mut invalid_rejected_and_reentered = true;
    let mut relabelled_m3_transfer = true;
    let mut symmetric_rejected = true;
    let mut all_pre_answer = true;
    let mut all_selected_from_occurrence = true;
    let mut all_gates_available = true;

    for learner_index in 0..learner_count {
        let cell_seed = base_seed + learner_index as u64 * 10_000;
        acquisition_seeds.insert(cell_seed);
        acquisition_seeds.insert(cell_seed + 1);
        let Some(mut event_gate) = frozen_m3::ds4_event_gate(cell_seed, 2) else {
            all_gates_available = false;
            continue;
        };
        let mut request = frozen_p4::ds4_request_session(cell_seed as usize);

        let mut competence = None;
        for episode in 1..=4_000usize {
            let event_seed = cell_seed + 1_000 + episode as u64;
            acquisition_seeds.insert(event_seed);
            let activity = frozen_m3::event_completion_activity(
                &mut event_gate,
                event_seed,
                EventFixture::Standard,
            );
            let step = frozen_p4::activate_request_selection(
                &mut request,
                activity.completion_spikes,
                true,
            );
            m3_learned_uses += activity.learned_uses;
            completion_activity += activity.completion_spikes;
            selection_activations += step.selection_activations;
            execution_activations += step.execution_activations;
            update_activations += step.update_activations;
            m3_physical_work += activity.physical_work;
            if step.selection_activations > 0 {
                all_pre_answer &= step.pre_answer_trace;
                all_selected_from_occurrence &= step.selected_from_occurrence;
            }
            if frozen_p4::ds4_request_ready(&request) {
                competence = Some(episode);
                break;
            }
        }
        if let Some(episode) = competence {
            ready_learners += 1;
            competence_episodes.push(episode);
        }
        single_role_learners += usize::from(frozen_p4::ds4_request_role_count(&request) == 1);

        let p4_before = frozen_p4::ds4_request_fingerprint(&request);
        let m3_before = frozen_m3::ds4_event_state(&event_gate);
        for held_out in 0..held_out_per_learner {
            let event_seed = cell_seed + 100_000 + held_out as u64;
            held_out_seeds.insert(event_seed);
            let fixture = if held_out.is_multiple_of(2) {
                EventFixture::Standard
            } else {
                EventFixture::Relabelled
            };
            let activity =
                frozen_m3::event_completion_activity(&mut event_gate, event_seed, fixture);
            relabelled_m3_transfer &=
                activity.completion_spikes > 0 && activity.learned_uses == activity.generic_spans;
            let step = frozen_p4::activate_request_selection(
                &mut request,
                activity.completion_spikes,
                false,
            );
            request_positions.insert(step.target_position);
            held_out_correct += usize::from(step.functional);
            explicit_answers += usize::from(step.explicit_answer);
            queues_empty += usize::from(step.queue_empty);
            all_pre_answer &= step.pre_answer_trace;
            all_selected_from_occurrence &= step.selected_from_occurrence;
            m3_learned_uses += activity.learned_uses;
            completion_activity += activity.completion_spikes;
            selection_activations += step.selection_activations;
            execution_activations += step.execution_activations;
            m3_physical_work += activity.physical_work;
        }
        p4_nonplastic &= p4_before == frozen_p4::ds4_request_fingerprint(&request);
        m3_nonplastic &= m3_before == frozen_m3::ds4_event_state(&event_gate);

        let no_event_before = frozen_p4::ds4_request_fingerprint(&request);
        let no_event = frozen_p4::activate_request_selection(&mut request, 0, false);
        learned_event_required &= no_event.selection_activations == 0
            && no_event.execution_activations == 0
            && no_event.update_activations == 0
            && no_event_before == frozen_p4::ds4_request_fingerprint(&request);

        let mut subthreshold_gate = frozen_m3::ds4_event_gate(cell_seed + 200_000, 1);
        let subthreshold = subthreshold_gate
            .as_mut()
            .map_or_else(EventActivity::default, |gate| {
                frozen_m3::event_completion_activity(
                    gate,
                    cell_seed + 200_100,
                    EventFixture::Standard,
                )
            });
        let mut subthreshold_request =
            frozen_p4::ds4_request_session((cell_seed + 200_000) as usize);
        let subthreshold_step = frozen_p4::activate_request_selection(
            &mut subthreshold_request,
            subthreshold.completion_spikes,
            false,
        );
        subthreshold_rejected &= subthreshold.generic_spans > 0
            && subthreshold.learned_uses == 0
            && subthreshold.completion_spikes == 0
            && subthreshold_step.selection_activations == 0;

        let missing = frozen_m3::event_completion_activity(
            &mut event_gate,
            cell_seed + 210_000,
            EventFixture::MissingClose,
        );
        missing_close_rejected &= missing.generic_spans == 0 && missing.completion_spikes == 0;

        let invalid = frozen_m3::event_completion_activity(
            &mut event_gate,
            cell_seed + 220_000,
            EventFixture::InvalidTransition,
        );
        let reentry = frozen_m3::event_completion_activity(
            &mut event_gate,
            cell_seed + 220_001,
            EventFixture::Standard,
        );
        invalid_rejected_and_reentered &=
            invalid.completion_spikes == 0 && reentry.completion_spikes > 0;

        let Some(mut symmetric_gate) = frozen_m3::ds4_event_gate(cell_seed + 300_000, 2) else {
            all_gates_available = false;
            continue;
        };
        let mut symmetric_request = frozen_p4::ds4_request_session((cell_seed + 300_000) as usize);
        let mut symmetric_functional = 0usize;
        for episode in 0..128usize {
            let activity = frozen_m3::event_completion_activity(
                &mut symmetric_gate,
                cell_seed + 301_000 + episode as u64,
                EventFixture::Standard,
            );
            let step =
                frozen_p4::ds4_symmetric_step(&mut symmetric_request, activity.completion_spikes);
            symmetric_functional += usize::from(step.functional);
        }
        symmetric_rejected &= !frozen_p4::ds4_request_ready(&symmetric_request)
            && frozen_p4::ds4_request_role_count(&symmetric_request) == 0
            && symmetric_functional == 0;
    }

    let seed_disjoint = acquisition_seeds.is_disjoint(&held_out_seeds);
    let controls = vec![
        control(
            1,
            "learned-event-required",
            learned_event_required,
            "zero completion activity yields zero selection/execution/update",
        ),
        control(
            2,
            "subthreshold-m3",
            subthreshold_rejected,
            "generic spans with zero learned use do not activate P4",
        ),
        control(
            3,
            "missing-close",
            missing_close_rejected,
            "incomplete M3 candidates produce no completion activity",
        ),
        control(
            4,
            "invalid-transition-and-reentry",
            invalid_rejected_and_reentered,
            "invalid event is silent and a later valid event reenters",
        ),
        control(
            5,
            "fresh-m3-identities-and-allocation",
            relabelled_m3_transfer,
            "relabelled/reallocated M3 streams retain learned completion",
        ),
        control(
            6,
            "fresh-request-serialization",
            request_positions.len() == 6,
            format!("positions={:?}", request_positions),
        ),
        control(
            7,
            "symmetric-impossible-requests",
            symmetric_rejected,
            "identical request signatures form no stable role",
        ),
        control(
            8,
            "pre-answer-information-flow",
            all_pre_answer,
            "every selected trace precedes recurrence and terminal update",
        ),
        control(
            9,
            "no-separate-target-channel",
            all_selected_from_occurrence,
            "every selected opaque identity is recovered from an occurrence",
        ),
        control(
            10,
            "frozen-source-leak-audit",
            source.passed(),
            format!("source={source:?}"),
        ),
        control(
            11,
            "held-out-non-plasticity",
            p4_nonplastic && m3_nonplastic,
            format!("p4={p4_nonplastic} m3={m3_nonplastic}"),
        ),
    ];

    DevelopmentSnapshot {
        source,
        learner_count,
        ready_learners,
        single_role_learners,
        competence_episodes,
        m3_learned_uses,
        completion_activity,
        selection_activations,
        execution_activations,
        update_activations,
        m3_physical_work,
        held_out_correct,
        held_out_total,
        explicit_answers,
        queues_empty,
        request_positions,
        p4_nonplastic,
        m3_nonplastic,
        acquisition_seeds,
        held_out_seeds,
        controls: controls
            .into_iter()
            .chain(std::iter::once(control(
                12,
                "disjoint-development-population",
                seed_disjoint && all_gates_available,
                format!(
                    "acquisition={} held_out={} gates={all_gates_available}",
                    acquisition_seeds.len(),
                    held_out_seeds.len()
                ),
            )))
            .collect(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub development_ready: bool,
    pub m3_authoritative: bool,
    pub m4_exists: bool,
    pub source: SourceAudit,
    pub stages: [String; 6],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub learner_count: usize,
    pub ready_learners: usize,
    pub single_role_learners: usize,
    pub average_competence_episode_millis: u64,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub explicit_answers: usize,
    pub queues_empty: usize,
    pub request_positions: usize,
    pub m3_learned_uses: usize,
    pub completion_activity: usize,
    pub selection_activations: usize,
    pub execution_activations: usize,
    pub update_activations: usize,
    pub m3_physical_work: u64,
    pub duplicate_deterministic: bool,
    pub p4_nonplastic: bool,
    pub m3_nonplastic: bool,
    pub controls: Vec<ControlResult>,
}

fn forbidden_report() -> Report {
    Report {
        label: "DS4 CUMULATIVE DEFINITIVE FORBIDDEN".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "DEFINITIVE-FORBIDDEN".to_string(),
        claim_eligible: false,
        development_ready: false,
        m3_authoritative: true,
        m4_exists: false,
        source: source_audit(),
        stages: std::array::from_fn(|_| "BLOCKED".to_string()),
        first_collapse_stage: None,
        first_collapse: "separate definitive matrix preregistration required".to_string(),
        learner_count: 0,
        ready_learners: 0,
        single_role_learners: 0,
        average_competence_episode_millis: 0,
        held_out_correct: 0,
        held_out_total: 0,
        explicit_answers: 0,
        queues_empty: 0,
        request_positions: 0,
        m3_learned_uses: 0,
        completion_activity: 0,
        selection_activations: 0,
        execution_activations: 0,
        update_activations: 0,
        m3_physical_work: 0,
        duplicate_deterministic: false,
        p4_nonplastic: false,
        m3_nonplastic: false,
        controls: Vec::new(),
    }
}

pub fn run(mode: HarnessMode) -> Report {
    if mode == HarnessMode::Definitive {
        return forbidden_report();
    }
    let (mode_name, base_seed, learner_count, held_out) = match mode {
        HarnessMode::Micro => ("MICRO", 95_000, 2, 8),
        HarnessMode::Gate => ("GATE", 96_000, 6, 32),
        HarnessMode::Definitive => unreachable!(),
    };
    let mut first = development_snapshot(base_seed, learner_count, held_out);
    let second = development_snapshot(base_seed, learner_count, held_out);
    let duplicate_deterministic = first == second;
    if let Some(control) = first
        .controls
        .iter_mut()
        .find(|control| control.number == 12)
    {
        control.passed &= duplicate_deterministic;
        control.diagnostic = format!("{} duplicate={duplicate_deterministic}", control.diagnostic);
    }
    let source_ready = first.source.passed();
    let physical_path = first.m3_learned_uses > 0
        && first.completion_activity > 0
        && first.selection_activations > 0
        && first.execution_activations > 0
        && first.update_activations > 0;
    let request_ready = first.ready_learners == learner_count
        && first.single_role_learners == learner_count
        && first.competence_episodes.len() == learner_count;
    let functional_transfer = first.held_out_correct == first.held_out_total
        && first.explicit_answers == first.held_out_total
        && first.queues_empty == first.held_out_total;
    let controls_ready = first.controls.len() == 12
        && first.controls.iter().all(|control| control.passed)
        && duplicate_deterministic;
    let lifecycle_ready = duplicate_deterministic
        && first.m3_physical_work > 0
        && first.p4_nonplastic
        && first.m3_nonplastic
        && first.acquisition_seeds.is_disjoint(&first.held_out_seeds);
    let ready = [
        source_ready,
        physical_path,
        request_ready,
        functional_transfer,
        controls_ready,
        lifecycle_ready,
    ];
    let first_collapse_stage = ready.iter().position(|value| !value);
    let stages = std::array::from_fn(|stage| match first_collapse_stage {
        None => "READY".to_string(),
        Some(collapse) if stage < collapse => "READY".to_string(),
        Some(collapse) if stage == collapse => format!("COLLAPSE: {}", STAGES[stage]),
        Some(_) => "BLOCKED".to_string(),
    });
    let first_collapse = first_collapse_stage
        .map(|stage| {
            if stage == 4 {
                first
                    .controls
                    .iter()
                    .find(|control| !control.passed)
                    .map(|control| format!("P4/control {} {}", control.number, control.name))
                    .unwrap_or_else(|| STAGES[stage].to_string())
            } else {
                STAGES[stage].to_string()
            }
        })
        .unwrap_or_else(|| "NONE".to_string());
    let development_ready = first_collapse_stage.is_none();
    let competence_total = first.competence_episodes.iter().sum::<usize>() as u64;
    let competence_count = first.competence_episodes.len() as u64;
    Report {
        label: if development_ready {
            "DS4 CUMULATIVE DEVELOPMENT READY".to_string()
        } else {
            format!("DS4 CUMULATIVE COLLAPSE AT {first_collapse}")
        },
        protocol: PROTOCOL.to_string(),
        mode: mode_name.to_string(),
        claim_eligible: false,
        development_ready,
        m3_authoritative: true,
        m4_exists: false,
        source: first.source,
        stages,
        first_collapse_stage,
        first_collapse,
        learner_count,
        ready_learners: first.ready_learners,
        single_role_learners: first.single_role_learners,
        average_competence_episode_millis: if competence_count == 0 {
            0
        } else {
            competence_total * 1_000 / competence_count
        },
        held_out_correct: first.held_out_correct,
        held_out_total: first.held_out_total,
        explicit_answers: first.explicit_answers,
        queues_empty: first.queues_empty,
        request_positions: first.request_positions.len(),
        m3_learned_uses: first.m3_learned_uses,
        completion_activity: first.completion_activity,
        selection_activations: first.selection_activations,
        execution_activations: first.execution_activations,
        update_activations: first.update_activations,
        m3_physical_work: first.m3_physical_work,
        duplicate_deterministic,
        p4_nonplastic: first.p4_nonplastic,
        m3_nonplastic: first.m3_nonplastic,
        controls: first.controls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_closes_only_the_first_missing_edge() {
        let report = run_probe();
        assert!(!report.claim_eligible);
        assert!(report.path_exists, "{report:#?}");
        assert_eq!(report.no_event_selection, 0);
        assert_eq!(report.no_event_execution, 0);
        assert_eq!(report.no_event_update, 0);
    }

    #[test]
    fn micro_is_development_only_and_ordered() {
        let report = run(HarnessMode::Micro);
        assert!(!report.claim_eligible && report.m3_authoritative && !report.m4_exists);
        assert!(report.development_ready, "{report:#?}");
        assert_eq!(report.controls.len(), 12);
        assert_eq!(report.held_out_correct, report.held_out_total);
    }

    #[test]
    fn definitive_remains_locked() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.claim_eligible);
        assert!(!report.development_ready);
        assert!(!report.m4_exists);
    }
}
