//! Development-only cumulative DS4 request/start linker over authoritative M3.
//!
//! The M3 cumulative port and P4 request-role mechanism are included from
//! byte-identical composition copies. This module closes only the physical
//! M3-completion-to-P4-selection edge and supplies development probes.

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
                target_position: encoded.target_position,
            }
        }
        // DS4_LINKER_END

        pub(super) fn ds4_request_ready(session: &Ds4RequestSession) -> bool {
            session.learner.target_role(request_signature(0)).is_some()
        }

        pub(super) fn ds4_request_fingerprint(session: &Ds4RequestSession) -> u64 {
            session.learner.fingerprint()
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
    let activity = event_gate.as_mut().map_or_else(EventActivity::default, |gate| {
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
}
