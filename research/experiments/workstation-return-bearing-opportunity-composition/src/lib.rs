#![forbid(unsafe_code)]

mod runtime_attached_complete;

pub use runtime_attached_complete::{
    RuntimeAttachedCompleteTrace, RuntimeAttachedExecution, RuntimeAttachedProgress,
    RuntimeAttachedStage, capture_runtime_attached_complete_candidate,
    capture_runtime_attached_complete_candidate_with_progress,
};

use academy_workstation::{
    DeviceEvent, KeyId, SessionObservation, WorkstationPresentation, WorkstationSession,
    WorldError, WorldGeometry,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_workstation::{
    BodyAxis, BodyControl, BodyMovement, Direction, Eye, Protocol, ResearchChoiceDiagnostic,
    ResearchHarnessConfig, ResearchOpportunityIncidence, ResearchRetinalTransition,
    ResearchTransitionOpportunity, WorkstationState, research_foveal_reach_retinal_features,
    research_retinotopic_retinal_features, research_wide_retinal_features,
};
use workstation_contact_contingency::{EVIDENCE_SEED, EVIDENCE_STEPS, project_observations};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ContinuationSummary {
    returned_components: u64,
    emitted_composed_inputs: u64,
    contact_relevant_returns: u64,
    contact_relevant_same_component_movements: u64,
    contact_relevant_same_direction_movements: u64,
    opposing_effort_steps: u64,
    admitted_current_transition_steps: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ComposedWitness {
    sequence: u64,
    axis: BodyAxis,
    control: BodyControl,
    prior_movement: BodyMovement,
    current_movement: BodyMovement,
    candidate: ResearchChoiceDiagnostic,
    continuation: ResearchChoiceDiagnostic,
    choice: ResearchChoiceDiagnostic,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CandidateEvidence {
    schema: &'static str,
    outcome: &'static str,
    seed: u64,
    steps: usize,
    exact_replay: bool,
    trace_sha256: String,
    continuation: ContinuationSummary,
    first_composed_witness: Option<ComposedWitness>,
    contact: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OutputSpecificOppositionTrace {
    schema: &'static str,
    seed: u64,
    observed_steps: usize,
    first_opposition_after_current_transition: Option<u64>,
    exact_replay: bool,
    naturally_quiescent: bool,
    observations: Vec<SessionObservation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlignedContactTrace {
    schema: String,
    seed: u64,
    observed_steps: usize,
    first_contact: Option<u64>,
    exact_replay: bool,
    naturally_quiescent: bool,
    observations: Vec<SessionObservation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VisibleKeyStep {
    sequence: u64,
    image_sha256: [String; 2],
    retina: Vec<u8>,
    body_fingerprint: String,
    learner_fingerprint: String,
    state_after: WorkstationState,
    movements: Vec<BodyMovement>,
    retinal_transitions: Vec<ResearchRetinalTransition>,
    choice_diagnostics: Vec<ResearchChoiceDiagnostic>,
    device_events: Vec<DeviceEvent>,
    keys_down: Vec<u16>,
    naturally_quiescent: bool,
    physical_work: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VisibleKeyIntentionTrace {
    schema: &'static str,
    seed: u64,
    steps: usize,
    key_a: u16,
    key_b: u16,
    first_image_divergence: Option<u64>,
    first_retinal_divergence: Option<u64>,
    first_learner_divergence: Option<u64>,
    first_retinal_transition_a: Option<u64>,
    first_retinal_transition_b: Option<u64>,
    first_candidate_divergence: Option<u64>,
    first_choice_divergence: Option<u64>,
    first_movement_divergence: Option<u64>,
    first_body_state_divergence: Option<u64>,
    first_key_a_press: Option<u64>,
    first_key_b_press: Option<u64>,
    off_target_presses_a: Vec<(u64, u16)>,
    off_target_presses_b: Vec<(u64, u16)>,
    exact_initial_body_equality: bool,
    run_a: Vec<VisibleKeyStep>,
    run_b: Vec<VisibleKeyStep>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MonitorCueReuseTrace {
    schema: &'static str,
    seed: u64,
    development_steps: usize,
    probe_steps: usize,
    learned_glyphs: Vec<char>,
    source_keys: Vec<u16>,
    first_text_change: Option<u64>,
    first_monitor_image_after_text: Option<u64>,
    first_retina_change_after_text: Option<u64>,
    first_learner_change_after_text: Option<u64>,
    first_consequence_recorded_after_text: Option<u64>,
    paired_initial_body_equality: bool,
    paired_initial_device_equality: bool,
    paired_initial_learner_equality: bool,
    first_probe_image_divergence: Option<u64>,
    first_probe_retinal_divergence: Option<u64>,
    first_probe_learner_divergence: Option<u64>,
    first_probe_candidate_divergence: Option<u64>,
    first_probe_choice_divergence: Option<u64>,
    first_probe_movement_divergence: Option<u64>,
    exact_replay: bool,
    naturally_quiescent: bool,
    frontier: &'static str,
    development: Vec<VisibleKeyStep>,
    cue_a: Vec<VisibleKeyStep>,
    cue_b: Vec<VisibleKeyStep>,
    blank: Vec<VisibleKeyStep>,
    unlearned: Vec<VisibleKeyStep>,
}

pub fn capture_monitor_cue_action_outcome_reuse(
    development_steps: usize,
    probe_steps: usize,
) -> Result<MonitorCueReuseTrace, WorldError> {
    if development_steps == 0 || development_steps > 240 || probe_steps == 0 || probe_steps > 120 {
        return Err(WorldError::InvalidRecording);
    }
    let config = threshold_retinotopic_visual_transition_config();
    let geometry = WorldGeometry::standard_ansi_104()?;
    let mut development_session = WorkstationSession::new_research_with_presentation(
        EVIDENCE_SEED,
        config,
        WorkstationPresentation::default(),
    )?;
    let development_checkpoint = development_session.save()?;
    let mut development_observations = Vec::with_capacity(development_steps);
    let mut source_keys = Vec::new();
    let mut learned_glyphs = Vec::new();
    let mut first_text_change = None;
    let mut all_keys_released_after_pair = false;

    for _ in 0..development_steps {
        let observation = development_session.step()?;
        for event in &observation.device_events {
            if let DeviceEvent::KeyPressed { key } = event {
                let Some(key_geometry) = geometry.key(KeyId(*key)) else {
                    continue;
                };
                let mut characters = key_geometry.label.chars();
                let Some(glyph) = characters.next() else {
                    continue;
                };
                if characters.next().is_none()
                    && glyph.is_ascii_graphic()
                    && !learned_glyphs.contains(&glyph)
                {
                    source_keys.push(*key);
                    learned_glyphs.push(glyph);
                }
            }
            if matches!(event, DeviceEvent::TextChanged) {
                first_text_change.get_or_insert(observation.sequence);
            }
        }
        all_keys_released_after_pair =
            learned_glyphs.len() >= 2 && observation.device_after.keys_down().next().is_none();
        development_observations.push(observation);
        if all_keys_released_after_pair {
            break;
        }
    }

    let mut development_replay =
        WorkstationSession::restore_research_config(development_checkpoint, config)?;
    let mut exact_replay = true;
    for expected in &development_observations {
        exact_replay &= development_replay.step()? == *expected;
    }
    exact_replay &= development_replay.save()? == development_session.save()?;

    let development = development_observations
        .iter()
        .map(|observation| visible_key_step(observation, true, false))
        .collect::<Vec<_>>();
    let text_index = first_text_change.and_then(|sequence| {
        development
            .iter()
            .position(|step| step.sequence == sequence)
    });
    let first_monitor_image_after_text =
        first_changed_after(&development, text_index, |step| &step.image_sha256);
    let first_retina_change_after_text =
        first_changed_after(&development, text_index, |step| &step.retina);
    let first_learner_change_after_text =
        first_changed_after(&development, text_index, |step| &step.learner_fingerprint);
    let first_consequence_recorded_after_text = text_index.and_then(|index| {
        development
            .iter()
            .skip(index.saturating_add(1))
            .find(|step| {
                step.choice_diagnostics.iter().any(|diagnostic| {
                    matches!(
                        diagnostic,
                        ResearchChoiceDiagnostic::ConsequenceRecorded { .. }
                    )
                })
            })
            .map(|step| step.sequence)
    });

    let base_checkpoint = development_session.save()?;
    let base_read = development_session.read()?;
    let glyph_a = learned_glyphs.first().copied();
    let glyph_b = learned_glyphs.get(1).copied();
    let (cue_a, replay_a) = capture_monitor_probe(
        base_checkpoint.clone(),
        config,
        glyph_a.map(WorkstationPresentation::with_monitor_glyph),
        probe_steps,
    )?;
    let (cue_b, replay_b) = capture_monitor_probe(
        base_checkpoint.clone(),
        config,
        glyph_b.map(WorkstationPresentation::with_monitor_glyph),
        probe_steps,
    )?;
    let (blank, replay_blank) = capture_monitor_probe(
        base_checkpoint.clone(),
        config,
        Some(WorkstationPresentation::default()),
        probe_steps,
    )?;
    let (unlearned, replay_unlearned) = capture_monitor_probe(
        base_checkpoint.clone(),
        config,
        Some(WorkstationPresentation::with_monitor_glyph('?')),
        probe_steps,
    )?;
    exact_replay &= replay_a && replay_b && replay_blank && replay_unlearned;

    let mut session_a =
        WorkstationSession::restore_research_config(base_checkpoint.clone(), config)?;
    let mut session_b = WorkstationSession::restore_research_config(base_checkpoint, config)?;
    if let Some(glyph) = glyph_a {
        session_a.set_presentation(WorkstationPresentation::with_monitor_glyph(glyph))?;
    }
    if let Some(glyph) = glyph_b {
        session_b.set_presentation(WorkstationPresentation::with_monitor_glyph(glyph))?;
    }
    let read_a = session_a.read()?;
    let read_b = session_b.read()?;
    let paired_initial_body_equality = read_a.body.state == read_b.body.state;
    let paired_initial_device_equality = read_a.device == read_b.device;
    let paired_initial_learner_equality = read_a.body.learner_fingerprint
        == read_b.body.learner_fingerprint
        && read_a.body.learner_fingerprint == base_read.body.learner_fingerprint;

    let first_probe_image_divergence =
        first_pair_divergence(&cue_a, &cue_b, |step| &step.image_sha256);
    let first_probe_retinal_divergence = first_pair_divergence(&cue_a, &cue_b, |step| &step.retina);
    let first_probe_learner_divergence =
        first_pair_divergence(&cue_a, &cue_b, |step| &step.learner_fingerprint);
    let first_probe_candidate_divergence =
        first_diagnostic_divergence(&cue_a, &cue_b, |diagnostic| {
            matches!(diagnostic, ResearchChoiceDiagnostic::Candidate { .. })
        });
    let first_probe_choice_divergence = first_diagnostic_divergence(&cue_a, &cue_b, |diagnostic| {
        matches!(diagnostic, ResearchChoiceDiagnostic::Choice { .. })
    });
    let first_probe_movement_divergence =
        first_pair_divergence(&cue_a, &cue_b, |step| &step.movements);
    let naturally_quiescent = development.iter().all(|step| step.naturally_quiescent)
        && [&cue_a, &cue_b, &blank, &unlearned]
            .into_iter()
            .flatten()
            .all(|step| step.naturally_quiescent);

    let frontier = if learned_glyphs.len() < 2 || !all_keys_released_after_pair {
        "development_key_pair"
    } else if first_monitor_image_after_text.is_none() {
        "monitor_image_return"
    } else if first_retina_change_after_text.is_none() {
        "development_retinal_return"
    } else if first_consequence_recorded_after_text.is_none() {
        "action_outcome_return"
    } else if first_probe_image_divergence.is_none() {
        "cue_render"
    } else if first_probe_retinal_divergence.is_none() {
        "cue_retina"
    } else if first_probe_learner_divergence.is_none() {
        "cue_learner"
    } else if first_probe_candidate_divergence.is_none() {
        "cue_path_reuse"
    } else if first_probe_choice_divergence.is_none() {
        "cue_executable_choice"
    } else if first_probe_movement_divergence.is_none() {
        "cue_outward_effect"
    } else {
        "keyboard_search"
    };

    Ok(MonitorCueReuseTrace {
        schema: "workstation-monitor-cue-reuse-trace/v1",
        seed: EVIDENCE_SEED,
        development_steps: development.len(),
        probe_steps,
        learned_glyphs,
        source_keys,
        first_text_change,
        first_monitor_image_after_text,
        first_retina_change_after_text,
        first_learner_change_after_text,
        first_consequence_recorded_after_text,
        paired_initial_body_equality,
        paired_initial_device_equality,
        paired_initial_learner_equality,
        first_probe_image_divergence,
        first_probe_retinal_divergence,
        first_probe_learner_divergence,
        first_probe_candidate_divergence,
        first_probe_choice_divergence,
        first_probe_movement_divergence,
        exact_replay,
        naturally_quiescent,
        frontier,
        development,
        cue_a,
        cue_b,
        blank,
        unlearned,
    })
}

fn capture_monitor_probe(
    checkpoint: academy_workstation::SessionCheckpoint,
    config: ResearchHarnessConfig,
    presentation: Option<WorkstationPresentation>,
    steps: usize,
) -> Result<(Vec<VisibleKeyStep>, bool), WorldError> {
    let mut session = WorkstationSession::restore_research_config(checkpoint, config)?;
    if let Some(presentation) = presentation {
        session.set_presentation(presentation)?;
    }
    let checkpoint = session.save()?;
    let mut observations = Vec::with_capacity(steps);
    for _ in 0..steps {
        observations.push(session.step()?);
    }
    let mut replay = WorkstationSession::restore_research_config(checkpoint, config)?;
    let mut exact = true;
    for expected in &observations {
        exact &= replay.step()? == *expected;
    }
    exact &= replay.save()? == session.save()?;
    Ok((
        observations
            .iter()
            .map(|observation| visible_key_step(observation, true, false))
            .collect(),
        exact,
    ))
}

fn first_changed_after<T: PartialEq + ?Sized>(
    steps: &[VisibleKeyStep],
    start: Option<usize>,
    field: impl Fn(&VisibleKeyStep) -> &T,
) -> Option<u64> {
    let index = start?;
    let before = steps.get(index)?;
    steps
        .iter()
        .skip(index.saturating_add(1))
        .find(|step| field(step) != field(before))
        .map(|step| step.sequence)
}

fn first_pair_divergence<T: PartialEq + ?Sized>(
    left: &[VisibleKeyStep],
    right: &[VisibleKeyStep],
    field: impl Fn(&VisibleKeyStep) -> &T,
) -> Option<u64> {
    left.iter()
        .zip(right)
        .find(|(left, right)| field(left) != field(right))
        .map(|(left, _)| left.sequence)
}

fn first_diagnostic_divergence(
    left: &[VisibleKeyStep],
    right: &[VisibleKeyStep],
    include: impl Fn(&ResearchChoiceDiagnostic) -> bool + Copy,
) -> Option<u64> {
    left.iter()
        .zip(right)
        .find(|(left, right)| {
            projected_diagnostics(left, include) != projected_diagnostics(right, include)
        })
        .map(|(left, _)| left.sequence)
}

pub fn capture_visible_key_intention_pair(
    steps: usize,
) -> Result<VisibleKeyIntentionTrace, WorldError> {
    capture_visible_key_pair(
        steps,
        "workstation-visible-key-intention-trace/v1",
        visible_key_intention_config(),
        KeyId(19),
        KeyId(27),
    )
}

pub fn capture_visual_transition_pair(
    steps: usize,
) -> Result<VisibleKeyIntentionTrace, WorldError> {
    capture_visible_key_pair(
        steps,
        "workstation-visual-transition-return-trace/v1",
        visual_transition_config(),
        KeyId(19),
        KeyId(27),
    )
}

pub fn capture_visual_transition_opposition_pair(
    steps: usize,
) -> Result<VisibleKeyIntentionTrace, WorldError> {
    capture_visible_key_pair(
        steps,
        "workstation-visual-transition-opposition-trace/v1",
        visual_transition_config(),
        KeyId(19),
        KeyId(79),
    )
}

pub fn capture_retinotopic_visual_transition_pair(
    steps: usize,
) -> Result<VisibleKeyIntentionTrace, WorldError> {
    capture_visible_key_pair(
        steps,
        "workstation-retinotopic-visual-transition-trace/v1",
        retinotopic_visual_transition_config(),
        KeyId(26),
        KeyId(87),
    )
}

pub fn capture_magnitude_retinotopic_visual_transition_pair(
    steps: usize,
) -> Result<VisibleKeyIntentionTrace, WorldError> {
    capture_visible_key_pair(
        steps,
        "workstation-magnitude-retinotopic-visual-transition-trace/v1",
        magnitude_retinotopic_visual_transition_config(),
        KeyId(26),
        KeyId(87),
    )
}

pub fn capture_threshold_retinotopic_visual_transition_pair(
    steps: usize,
) -> Result<VisibleKeyIntentionTrace, WorldError> {
    capture_visible_key_pair(
        steps,
        "workstation-threshold-retinotopic-visual-transition-trace/v1",
        threshold_retinotopic_visual_transition_config(),
        KeyId(26),
        KeyId(87),
    )
}

pub fn capture_visual_reach_pair(steps: usize) -> Result<VisibleKeyIntentionTrace, WorldError> {
    capture_visible_key_pair(
        steps,
        "workstation-visual-reach-trace/v1",
        visual_reach_config(),
        KeyId(26),
        KeyId(87),
    )
}

pub fn capture_foveal_visual_reach_pair(
    steps: usize,
) -> Result<VisibleKeyIntentionTrace, WorldError> {
    capture_visible_key_pair(
        steps,
        "workstation-foveal-visual-reach-trace/v1",
        foveal_visual_reach_config(),
        KeyId(26),
        KeyId(87),
    )
}

fn capture_visible_key_pair(
    steps: usize,
    schema: &'static str,
    config: ResearchHarnessConfig,
    key_a: KeyId,
    key_b: KeyId,
) -> Result<VisibleKeyIntentionTrace, WorldError> {
    if steps == 0 || steps > 240 {
        return Err(WorldError::InvalidRecording);
    }
    let mut session_a = WorkstationSession::new_research_with_presentation(
        EVIDENCE_SEED,
        config,
        WorkstationPresentation::with_illuminated_key(key_a),
    )?;
    let mut session_b = WorkstationSession::new_research_with_presentation(
        EVIDENCE_SEED,
        config,
        WorkstationPresentation::with_illuminated_key(key_b),
    )?;
    let exact_initial_body_equality = session_a.read()?.body == session_b.read()?.body;
    let mut trace = VisibleKeyIntentionTrace {
        schema,
        seed: EVIDENCE_SEED,
        steps,
        key_a: key_a.0,
        key_b: key_b.0,
        first_image_divergence: None,
        first_retinal_divergence: None,
        first_learner_divergence: None,
        first_retinal_transition_a: None,
        first_retinal_transition_b: None,
        first_candidate_divergence: None,
        first_choice_divergence: None,
        first_movement_divergence: None,
        first_body_state_divergence: None,
        first_key_a_press: None,
        first_key_b_press: None,
        off_target_presses_a: Vec::new(),
        off_target_presses_b: Vec::new(),
        exact_initial_body_equality,
        run_a: Vec::with_capacity(steps),
        run_b: Vec::with_capacity(steps),
    };

    for _ in 0..steps {
        let observation_a = session_a.step()?;
        let observation_b = session_b.step()?;
        let retinotopic = matches!(
            config.transition_opportunity,
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        );
        let foveal = config.transition_opportunity
            == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach;
        let step_a = visible_key_step(&observation_a, retinotopic, foveal);
        let step_b = visible_key_step(&observation_b, retinotopic, foveal);
        let sequence = observation_a.sequence;
        debug_assert_eq!(sequence, observation_b.sequence);
        if step_a.image_sha256 != step_b.image_sha256 {
            trace.first_image_divergence.get_or_insert(sequence);
        }
        if step_a.retina != step_b.retina {
            trace.first_retinal_divergence.get_or_insert(sequence);
        }
        if step_a.learner_fingerprint != step_b.learner_fingerprint {
            trace.first_learner_divergence.get_or_insert(sequence);
        }
        if !step_a.retinal_transitions.is_empty() {
            trace.first_retinal_transition_a.get_or_insert(sequence);
        }
        if !step_b.retinal_transitions.is_empty() {
            trace.first_retinal_transition_b.get_or_insert(sequence);
        }
        if projected_diagnostics(&step_a, |diagnostic| {
            matches!(diagnostic, ResearchChoiceDiagnostic::Candidate { .. })
        }) != projected_diagnostics(&step_b, |diagnostic| {
            matches!(diagnostic, ResearchChoiceDiagnostic::Candidate { .. })
        }) {
            trace.first_candidate_divergence.get_or_insert(sequence);
        }
        if projected_diagnostics(&step_a, |diagnostic| {
            matches!(diagnostic, ResearchChoiceDiagnostic::Choice { .. })
        }) != projected_diagnostics(&step_b, |diagnostic| {
            matches!(diagnostic, ResearchChoiceDiagnostic::Choice { .. })
        }) {
            trace.first_choice_divergence.get_or_insert(sequence);
        }
        if step_a.movements != step_b.movements {
            trace.first_movement_divergence.get_or_insert(sequence);
        }
        if step_a.state_after != step_b.state_after {
            trace.first_body_state_divergence.get_or_insert(sequence);
        }
        collect_key_events(
            &observation_a,
            key_a,
            &mut trace.first_key_a_press,
            &mut trace.off_target_presses_a,
        );
        collect_key_events(
            &observation_b,
            key_b,
            &mut trace.first_key_b_press,
            &mut trace.off_target_presses_b,
        );
        trace.run_a.push(step_a);
        trace.run_b.push(step_b);
    }
    Ok(trace)
}

fn projected_diagnostics(
    step: &VisibleKeyStep,
    include: impl Fn(&ResearchChoiceDiagnostic) -> bool,
) -> Vec<&ResearchChoiceDiagnostic> {
    step.choice_diagnostics
        .iter()
        .filter(|diagnostic| include(diagnostic))
        .collect()
}

fn visible_key_step(
    observation: &SessionObservation,
    retinotopic: bool,
    foveal: bool,
) -> VisibleKeyStep {
    let retina = if foveal {
        research_foveal_reach_retinal_features(&observation.sample, &observation.body.state_before)
    } else if retinotopic {
        research_retinotopic_retinal_features(&observation.sample, &observation.body.state_before)
    } else {
        research_wide_retinal_features(&observation.sample, &observation.body.state_before)
    };
    VisibleKeyStep {
        sequence: observation.sequence,
        image_sha256: Eye::ALL.map(|eye| hex_digest(observation.sample.eye(eye).pixels())),
        retina: retina.to_vec(),
        body_fingerprint: observation.body.body_fingerprint.clone(),
        learner_fingerprint: observation.body.learner_fingerprint.clone(),
        state_after: observation.body.state_after.clone(),
        movements: observation.body.movements.clone(),
        retinal_transitions: observation.body.retinal_transitions.clone(),
        choice_diagnostics: observation.body.choice_diagnostics.clone(),
        device_events: observation.device_events.clone(),
        keys_down: observation
            .device_after
            .keys_down()
            .map(|key| key.0)
            .collect(),
        naturally_quiescent: observation.body.naturally_quiescent,
        physical_work: observation.body.metrics.physical_work,
    }
}

fn collect_key_events(
    observation: &SessionObservation,
    intended: KeyId,
    first_intended_press: &mut Option<u64>,
    off_target_presses: &mut Vec<(u64, u16)>,
) {
    for event in &observation.device_events {
        if let DeviceEvent::KeyPressed { key } = event {
            if *key == intended.0 {
                first_intended_press.get_or_insert(observation.sequence);
            } else {
                off_target_presses.push((observation.sequence, *key));
            }
        }
    }
}

pub fn continue_intermediate_contact_trace(
    encoded: &[u8],
    max_steps: usize,
) -> Result<AlignedContactTrace, WorldError> {
    let mut trace: AlignedContactTrace =
        serde_json::from_slice(encoded).map_err(|_| WorldError::InvalidRecording)?;
    if trace.schema != "workstation-intermediate-transition-contact-trace/v1"
        || trace.seed != EVIDENCE_SEED
        || !trace.exact_replay
        || !trace.naturally_quiescent
        || max_steps <= trace.observations.len()
        || max_steps > EVIDENCE_STEPS
    {
        return Err(WorldError::InvalidRecording);
    }

    let config = intermediate_transition_config();
    let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config)?;
    for expected in &trace.observations {
        if session.step()? != *expected {
            return Err(WorldError::RecordingReplayDiverged(expected.sequence));
        }
    }
    let mut duplicate = session.clone();
    while trace.observations.len() < max_steps && trace.first_contact.is_none() {
        let observation = session.step()?;
        let repeated = duplicate.step()?;
        if repeated != observation {
            return Err(WorldError::RecordingReplayDiverged(observation.sequence));
        }
        if observation
            .sample
            .contacts()
            .iter()
            .any(|contact| contact.pressure() > 0)
        {
            trace.first_contact = Some(observation.sequence);
        }
        trace.naturally_quiescent &= observation.body.naturally_quiescent;
        trace.observations.push(observation);
    }
    trace.observed_steps = trace.observations.len();
    trace.exact_replay &= session.save()? == duplicate.save()?;
    Ok(trace)
}

pub fn run() -> Result<CandidateEvidence, WorldError> {
    let config = candidate_config();
    let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config)?;
    let checkpoint = session.save()?;
    let mut observations = Vec::with_capacity(EVIDENCE_STEPS);
    for _ in 0..EVIDENCE_STEPS {
        observations.push(session.step()?);
    }

    let mut replay = WorkstationSession::restore_research_config(checkpoint, config)?;
    let mut exact_replay = true;
    for expected in &observations {
        exact_replay &= replay.step()? == *expected;
    }
    exact_replay &= replay.save()? == session.save()?;

    let trace = serde_json::to_vec(&observations).map_err(|_| WorldError::InvalidRecording)?;
    let trace_sha256 = hex_digest(&trace);
    let contact = project_observations(
        EVIDENCE_SEED,
        trace_sha256.clone(),
        &observations,
        exact_replay,
    )?;
    let contact = serde_json::to_value(contact).map_err(|_| WorldError::InvalidRecording)?;
    let continuation = summarize(&observations);
    let first_composed_witness = find_composed_witness(&observations);
    let separated = contact["five_finger_steps"] == 0;
    let work_bounded = contact["max_step_work"]
        .as_u64()
        .is_some_and(|work| work <= 2_000);
    let local_solve = first_composed_witness.is_some()
        && continuation.contact_relevant_same_direction_movements > 63
        && continuation.opposing_effort_steps == 0;
    let outcome = if exact_replay && separated && work_bounded && local_solve {
        "local-solve-passed"
    } else {
        "falsified"
    };

    Ok(CandidateEvidence {
        schema: "workstation-return-bearing-opportunity-composition/v1",
        outcome,
        seed: EVIDENCE_SEED,
        steps: EVIDENCE_STEPS,
        exact_replay,
        trace_sha256,
        continuation,
        first_composed_witness,
        contact,
    })
}

pub fn capture_output_specific_opposition() -> Result<OutputSpecificOppositionTrace, WorldError> {
    capture_opposition(
        output_specific_config(),
        "workstation-output-specific-opposition-trace/v1",
    )
}

pub fn capture_sequential_opposition() -> Result<OutputSpecificOppositionTrace, WorldError> {
    capture_opposition(
        sequential_output_specific_config(),
        "workstation-sequential-opposition-trace/v1",
    )
}

pub fn capture_aligned_contact() -> Result<AlignedContactTrace, WorldError> {
    capture_contact(
        aligned_output_specific_config(),
        "workstation-aligned-contact-trace/v1",
        48,
    )
}

pub fn capture_intermediate_transition_contact() -> Result<AlignedContactTrace, WorldError> {
    capture_contact(
        intermediate_transition_config(),
        "workstation-intermediate-transition-contact-trace/v1",
        48,
    )
}

pub fn capture_effect_receptor_contact() -> Result<AlignedContactTrace, WorldError> {
    capture_contact(
        effect_receptor_config(),
        "workstation-effect-receptor-contact-trace/v1",
        EVIDENCE_STEPS,
    )
}

fn capture_contact(
    config: ResearchHarnessConfig,
    schema: &'static str,
    max_steps: usize,
) -> Result<AlignedContactTrace, WorldError> {
    let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config)?;
    let checkpoint = session.save()?;
    let mut observations = Vec::new();
    let mut first_contact = None;
    for _ in 0..max_steps {
        let observation = session.step()?;
        if observation
            .sample
            .contacts()
            .iter()
            .any(|contact| contact.pressure() > 0)
        {
            first_contact = Some(observation.sequence);
        }
        observations.push(observation);
        if first_contact.is_some() {
            break;
        }
    }

    let mut replay = WorkstationSession::restore_research_config(checkpoint, config)?;
    let mut exact_replay = true;
    for expected in &observations {
        exact_replay &= replay.step()? == *expected;
    }
    let naturally_quiescent = observations
        .iter()
        .all(|observation| observation.body.naturally_quiescent);
    Ok(AlignedContactTrace {
        schema: schema.to_owned(),
        seed: EVIDENCE_SEED,
        observed_steps: observations.len(),
        first_contact,
        exact_replay,
        naturally_quiescent,
        observations,
    })
}

fn capture_opposition(
    config: ResearchHarnessConfig,
    schema: &'static str,
) -> Result<OutputSpecificOppositionTrace, WorldError> {
    let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config)?;
    let checkpoint = session.save()?;
    let mut observations = Vec::new();
    let mut saw_current_transition = false;
    let mut first_opposition = None;
    for _ in 0..48 {
        let observation = session.step()?;
        saw_current_transition |= observation
            .body
            .choice_diagnostics
            .iter()
            .any(|diagnostic| {
                matches!(
                    diagnostic,
                    ResearchChoiceDiagnostic::TransitionContinuation {
                        current_owner_transition: true,
                        admitted: true,
                        ..
                    }
                )
            });
        let opposing = observation
            .body
            .movements
            .iter()
            .any(|movement| movement.decrease_effort > 0 && movement.increase_effort > 0);
        if saw_current_transition && opposing {
            first_opposition = Some(observation.sequence);
        }
        observations.push(observation);
        if first_opposition.is_some() {
            break;
        }
    }

    let mut replay = WorkstationSession::restore_research_config(checkpoint, config)?;
    let mut exact_replay = true;
    for expected in &observations {
        exact_replay &= replay.step()? == *expected;
    }
    let naturally_quiescent = observations
        .iter()
        .all(|observation| observation.body.naturally_quiescent);
    Ok(OutputSpecificOppositionTrace {
        schema,
        seed: EVIDENCE_SEED,
        observed_steps: observations.len(),
        first_opposition_after_current_transition: first_opposition,
        exact_replay,
        naturally_quiescent,
        observations,
    })
}

fn candidate_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::ComposedWithReturn,
    }
}

fn output_specific_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveReturn,
    }
}

fn sequential_output_specific_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity:
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequential,
    }
}

fn aligned_output_specific_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity:
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAligned,
    }
}

fn intermediate_transition_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity:
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedTransition,
    }
}

fn effect_receptor_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity:
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedEffect,
    }
}

#[cfg(test)]
fn receptor_delta_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity:
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedDelta,
    }
}

#[cfg(test)]
fn causal_delta_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity:
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDelta,
    }
}

#[cfg(test)]
fn causal_delta_palm_component_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent,
    }
}

#[cfg(test)]
fn causal_delta_palm_component_outcome_lifetime_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol:
            Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent,
    }
}

fn visible_key_intention_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina,
    }
}

fn visual_transition_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition,
    }
}

fn retinotopic_visual_transition_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic,
    }
}

fn magnitude_retinotopic_visual_transition_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude,
    }
}

fn threshold_retinotopic_visual_transition_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds,
    }
}

fn visual_reach_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach,
    }
}

fn foveal_visual_reach_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach,
    }
}

fn summarize(observations: &[SessionObservation]) -> ContinuationSummary {
    let mut summary = ContinuationSummary {
        returned_components: 0,
        emitted_composed_inputs: 0,
        contact_relevant_returns: 0,
        contact_relevant_same_component_movements: 0,
        contact_relevant_same_direction_movements: 0,
        opposing_effort_steps: 0,
        admitted_current_transition_steps: 0,
    };
    for (index, observation) in observations.iter().enumerate() {
        let returned = &observation.body.returned_transitions;
        summary.returned_components += as_u64(returned.len());
        summary.emitted_composed_inputs += as_u64(returned.len()).saturating_mul(2);
        summary.opposing_effort_steps += u64::from(
            observation
                .body
                .movements
                .iter()
                .any(|movement| movement.decrease_effort > 0 && movement.increase_effort > 0),
        );
        summary.admitted_current_transition_steps += u64::from(
            observation
                .body
                .choice_diagnostics
                .iter()
                .any(|diagnostic| {
                    matches!(
                        diagnostic,
                        ResearchChoiceDiagnostic::TransitionContinuation {
                            current_owner_transition: true,
                            admitted: true,
                            ..
                        }
                    )
                }),
        );
        for axis in returned
            .iter()
            .copied()
            .filter(|axis| contact_relevant(*axis))
        {
            summary.contact_relevant_returns += 1;
            let current = changed_movement(observation, axis);
            summary.contact_relevant_same_component_movements += u64::from(current.is_some());
            let prior = index
                .checked_sub(1)
                .and_then(|prior| changed_movement(&observations[prior], axis));
            summary.contact_relevant_same_direction_movements +=
                u64::from(current.zip(prior).is_some_and(|(current, prior)| {
                    current.net_impulse.signum() == prior.net_impulse.signum()
                }));
        }
    }
    summary
}

fn find_composed_witness(observations: &[SessionObservation]) -> Option<ComposedWitness> {
    for (index, current) in observations.iter().enumerate().skip(1) {
        let prior = &observations[index - 1];
        for axis in current
            .body
            .returned_transitions
            .iter()
            .copied()
            .filter(|axis| contact_relevant(*axis))
        {
            let prior_movement = changed_movement(prior, axis)?;
            let current_movement = changed_movement(current, axis)?;
            if current_movement.net_impulse.signum() != prior_movement.net_impulse.signum() {
                continue;
            }
            let control = control_for(
                axis,
                if prior_movement.net_impulse.is_negative() {
                    Direction::Decrease
                } else {
                    Direction::Increase
                },
            );
            let candidate = current.body.choice_diagnostics.iter().find(|diagnostic| {
                matches!(diagnostic, ResearchChoiceDiagnostic::Candidate { control: candidate, path_inputs, executable: true, .. } if *candidate == control && *path_inputs > 0)
            })?;
            let continuation = current.body.choice_diagnostics.iter().find(|diagnostic| {
                matches!(diagnostic, ResearchChoiceDiagnostic::TransitionContinuation { control: candidate, current_owner_transition: true, admitted: true, .. } if *candidate == control)
            })?;
            let choice = current.body.choice_diagnostics.iter().find(|diagnostic| {
                matches!(diagnostic, ResearchChoiceDiagnostic::Choice { current_transition_control: Some(candidate), admitted_controls, .. } if *candidate == control && admitted_controls.as_slice() == [control])
            })?;
            return Some(ComposedWitness {
                sequence: current.sequence,
                axis,
                control,
                prior_movement,
                current_movement,
                candidate: candidate.clone(),
                continuation: continuation.clone(),
                choice: choice.clone(),
            });
        }
    }
    None
}

fn changed_movement(observation: &SessionObservation, axis: BodyAxis) -> Option<BodyMovement> {
    observation
        .body
        .movements
        .iter()
        .find(|movement| movement.changed && movement.axis == axis)
        .copied()
}

fn contact_relevant(axis: BodyAxis) -> bool {
    matches!(axis, BodyAxis::PalmDepth | BodyAxis::FingerFlexion { .. })
}

fn control_for(axis: BodyAxis, direction: Direction) -> BodyControl {
    match axis {
        BodyAxis::EyeHorizontal { eye } => BodyControl::EyeHorizontal { eye, direction },
        BodyAxis::EyeVertical { eye } => BodyControl::EyeVertical { eye, direction },
        BodyAxis::PalmHorizontal => BodyControl::PalmHorizontal { direction },
        BodyAxis::PalmVertical => BodyControl::PalmVertical { direction },
        BodyAxis::PalmDepth => BodyControl::PalmDepth { direction },
        BodyAxis::Wrist => BodyControl::Wrist { direction },
        BodyAxis::Spread => BodyControl::Spread { direction },
        BodyAxis::ThumbOpposition => BodyControl::ThumbOpposition { direction },
        BodyAxis::FingerFlexion { digit } => BodyControl::FingerFlexion { digit, direction },
    }
}

fn as_u64(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn co_scheduled_return_and_opportunity_do_not_form_a_current_arrow() {
        let config = candidate_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = session.save().unwrap();
        let mut observations = Vec::new();
        for _ in 0..48 {
            observations.push(session.step().unwrap());
            if find_composed_witness(&observations).is_some() {
                break;
            }
        }
        let summary = summarize(&observations);
        assert!(find_composed_witness(&observations).is_none());
        assert_eq!(summary.contact_relevant_same_component_movements, 0);
        assert_eq!(summary.admitted_current_transition_steps, 0);

        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        for expected in observations {
            assert_eq!(replay.step().unwrap(), expected);
        }
    }

    #[test]
    fn output_specific_return_reaches_current_choice_but_not_one_arrow() {
        let config = output_specific_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = session.save().unwrap();
        let mut observations = Vec::new();
        for _ in 0..48 {
            observations.push(session.step().unwrap());
            let summary = summarize(&observations);
            if summary.admitted_current_transition_steps > 0 && summary.opposing_effort_steps > 0 {
                break;
            }
        }
        let summary = summarize(&observations);
        assert!(find_composed_witness(&observations).is_none());
        assert!(summary.admitted_current_transition_steps > 0);
        assert!(summary.opposing_effort_steps > 0);

        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        for expected in observations {
            assert_eq!(replay.step().unwrap(), expected);
        }
    }

    #[test]
    fn sequential_world_effects_preserve_two_ordered_arrows() {
        let config = sequential_output_specific_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = session.save().unwrap();
        let mut observations = Vec::new();
        for _ in 0..48 {
            observations.push(session.step().unwrap());
            let summary = summarize(&observations);
            let ordered_opposition = observations.iter().any(|observation| {
                observation.body.movements.windows(2).any(|movements| {
                    movements[0].axis == movements[1].axis
                        && movements[0].changed
                        && movements[1].changed
                        && movements[0].net_impulse.signum() == -movements[1].net_impulse.signum()
                })
            });
            if summary.admitted_current_transition_steps > 0 && ordered_opposition {
                break;
            }
        }

        let summary = summarize(&observations);
        assert!(summary.admitted_current_transition_steps > 0);
        assert_eq!(summary.opposing_effort_steps, 0);
        assert!(observations.iter().any(|observation| {
            observation.body.movements.windows(2).any(|movements| {
                movements[0].axis == movements[1].axis
                    && movements[0].changed
                    && movements[1].changed
                    && movements[0].net_impulse.signum() == -movements[1].net_impulse.signum()
            })
        }));

        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        for expected in observations {
            assert_eq!(replay.step().unwrap(), expected);
        }
    }

    #[test]
    fn aligned_generic_opportunity_starts_a_silent_component() {
        let mut parent =
            WorkstationSession::new_research(EVIDENCE_SEED, sequential_output_specific_config())
                .unwrap();
        let config = aligned_output_specific_config();
        let mut candidate = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = candidate.save().unwrap();
        let mut parent_observations = Vec::new();
        let mut candidate_observations = Vec::new();
        for _ in 0..4 {
            parent_observations.push(parent.step().unwrap());
            candidate_observations.push(candidate.step().unwrap());
        }

        let palm_depth_executable = |observations: &[SessionObservation]| {
            observations
                .iter()
                .flat_map(|observation| &observation.body.choice_diagnostics)
                .filter(|diagnostic| {
                    matches!(
                        diagnostic,
                        ResearchChoiceDiagnostic::Candidate {
                            control: BodyControl::PalmDepth { .. },
                            executable: true,
                            ..
                        }
                    )
                })
                .count()
        };
        let palm_depth_movements = |observations: &[SessionObservation]| {
            observations
                .iter()
                .flat_map(|observation| &observation.body.movements)
                .filter(|movement| movement.axis == BodyAxis::PalmDepth && movement.changed)
                .count()
        };
        assert_eq!(palm_depth_executable(&parent_observations), 0);
        assert_eq!(palm_depth_movements(&parent_observations), 0);
        assert!(palm_depth_executable(&candidate_observations) > 0);
        assert!(palm_depth_movements(&candidate_observations) > 0);

        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        for expected in candidate_observations {
            assert_eq!(replay.step().unwrap(), expected);
        }
    }

    #[test]
    fn intermediate_proprioception_keeps_the_current_physical_arrow() {
        let mut parent =
            WorkstationSession::new_research(EVIDENCE_SEED, aligned_output_specific_config())
                .unwrap();
        let config = intermediate_transition_config();
        let mut candidate = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = candidate.save().unwrap();
        let parent_observations = [parent.step().unwrap(), parent.step().unwrap()];
        let candidate_observations = [candidate.step().unwrap(), candidate.step().unwrap()];

        let parent_second = &parent_observations[1];
        assert_eq!(
            parent_second.body.state_before.hand().palm().depth(),
            parent_second.body.state_after.hand().palm().depth()
        );
        let candidate_second = &candidate_observations[1];
        let palm_diagnostics = candidate_second
            .body
            .choice_diagnostics
            .iter()
            .filter(|diagnostic| match diagnostic {
                ResearchChoiceDiagnostic::Candidate { control, .. }
                | ResearchChoiceDiagnostic::TransitionContinuation { control, .. } => {
                    control.axis() == BodyAxis::PalmDepth
                }
                ResearchChoiceDiagnostic::Choice {
                    ordinary_control,
                    current_transition_control,
                    computed_winner_control,
                    ..
                } => ordinary_control
                    .or(*current_transition_control)
                    .or(*computed_winner_control)
                    .is_some_and(|control| control.axis() == BodyAxis::PalmDepth),
                ResearchChoiceDiagnostic::CompletedCycle { control, .. } => {
                    control.axis() == BodyAxis::PalmDepth
                }
                ResearchChoiceDiagnostic::ConsequenceRecorded { .. }
                | ResearchChoiceDiagnostic::ConsequenceConsumed { .. } => false,
            })
            .collect::<Vec<_>>();
        assert!(
            candidate_second.body.state_after.hand().palm().depth()
                > candidate_second.body.state_before.hand().palm().depth(),
            "movements={:#?}\ndiagnostics={:#?}",
            candidate_second.body.movements,
            palm_diagnostics
        );
        let palm_depth_movements = candidate_second
            .body
            .movements
            .iter()
            .filter(|movement| movement.axis == BodyAxis::PalmDepth && movement.changed)
            .collect::<Vec<_>>();
        assert_eq!(palm_depth_movements.len(), 2);
        assert!(palm_depth_movements.iter().all(|movement| {
            movement.increase_effort > 0
                && movement.decrease_effort == 0
                && movement.net_impulse > 0
        }));
        assert!(
            candidate_second
                .body
                .choice_diagnostics
                .iter()
                .any(|diagnostic| matches!(
                    diagnostic,
                    ResearchChoiceDiagnostic::Candidate {
                        control: BodyControl::PalmDepth {
                            direction: Direction::Increase
                        },
                        ownership,
                        consequence_tick: Some(_),
                        executable: true,
                        ..
                    } if ownership.starts_with("Owned(")
                ))
        );
        assert!(
            !candidate_second
                .body
                .choice_diagnostics
                .iter()
                .any(|diagnostic| matches!(
                    diagnostic,
                    ResearchChoiceDiagnostic::Choice {
                        computed_winner_control: Some(BodyControl::PalmDepth {
                            direction: Direction::Decrease
                        }),
                        computed_winner_basis,
                        ..
                    } if computed_winner_basis == "FreshAlternative"
                ))
        );

        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        for expected in candidate_observations {
            assert_eq!(replay.step().unwrap(), expected);
        }
    }

    #[test]
    fn effect_receptor_return_stays_near_origin() {
        let config = effect_receptor_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = session.save().unwrap();
        let mut observations = Vec::new();
        for _ in 0..4 {
            observations.push(session.step().unwrap());
        }

        let depth_trajectory = observations
            .iter()
            .map(|observation| observation.body.state_after.hand().palm().depth())
            .collect::<Vec<_>>();
        assert!(
            depth_trajectory
                .iter()
                .all(|depth| (240..=288).contains(depth))
        );
        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        for expected in observations {
            assert_eq!(replay.step().unwrap(), expected);
        }
    }

    #[test]
    fn stable_sample_identity_does_not_make_an_owned_delta_arrow() {
        let config = receptor_delta_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = session.save().unwrap();
        let mut observations = Vec::new();
        for _ in 0..2 {
            observations.push(session.step().unwrap());
        }

        assert_eq!(
            observations[1].body.state_after.hand().palm().depth(),
            observations[1].body.state_before.hand().palm().depth()
        );
        assert!(
            observations[1]
                .body
                .choice_diagnostics
                .iter()
                .any(|diagnostic| matches!(
                    diagnostic,
                    ResearchChoiceDiagnostic::Choice {
                        computed_winner_control: Some(BodyControl::PalmDepth {
                            direction: Direction::Decrease
                        }),
                        computed_winner_basis,
                        ..
                    } if computed_winner_basis == "FreshAlternative"
                ))
        );
        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        for expected in observations {
            assert_eq!(replay.step().unwrap(), expected);
        }
    }

    #[test]
    fn causal_delta_carries_the_output_arrow_through_the_receptor() {
        let config = causal_delta_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = session.save().unwrap();
        let first = session.step().unwrap();
        let second = session.step().unwrap();

        assert!(
            first.body.state_after.hand().palm().depth()
                > first.body.state_before.hand().palm().depth()
        );
        assert!(
            second.body.state_after.hand().palm().depth()
                > second.body.state_before.hand().palm().depth(),
            "movements={:#?}\ndiagnostics={:#?}",
            second.body.movements,
            second.body.choice_diagnostics
        );
        assert!(second.body.choice_diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic,
                ResearchChoiceDiagnostic::Choice {
                    computed_winner_control: Some(BodyControl::PalmDepth {
                        direction: Direction::Increase
                    }),
                    computed_winner_basis,
                    ..
                } if computed_winner_basis == "Ordinary"
            )
        }));
        assert!(!second.body.choice_diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic,
                ResearchChoiceDiagnostic::Choice {
                    computed_winner_control: Some(BodyControl::PalmDepth {
                        direction: Direction::Decrease
                    }),
                    computed_winner_basis,
                    ..
                } if computed_winner_basis == "FreshAlternative"
            )
        }));

        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        assert_eq!(replay.step().unwrap(), first);
        assert_eq!(replay.step().unwrap(), second);
    }

    #[test]
    fn causal_delta_releases_the_first_upper_boundary() {
        let config = causal_delta_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = session.save().unwrap();
        let mut observations = Vec::new();
        let mut reached_upper = false;
        let mut released_upper = false;
        for _ in 0..64 {
            let observation = session.step().unwrap();
            let before = observation.body.state_before.hand().palm().depth();
            let after = observation.body.state_after.hand().palm().depth();
            reached_upper |= after == truelearner_workstation::BODY_MAX;
            released_upper |= before == truelearner_workstation::BODY_MAX && after < before;
            observations.push(observation);
            if released_upper {
                break;
            }
        }

        let trajectory = observations
            .iter()
            .map(|observation| observation.body.state_after.hand().palm().depth())
            .collect::<Vec<_>>();
        assert!(reached_upper, "trajectory={trajectory:?}");
        assert!(released_upper, "trajectory={trajectory:?}");
        assert!(
            observations
                .iter()
                .all(|observation| observation.body.naturally_quiescent)
        );

        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        for expected in observations {
            assert_eq!(replay.step().unwrap(), expected);
        }
    }

    #[test]
    fn causal_delta_alone_misses_the_surface_in_a_corner_cycle() {
        let config = causal_delta_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let mut trajectory = Vec::new();
        let mut first_contact = None;
        for _ in 0..120 {
            let observation = session.step().unwrap();
            let palm = observation.body.state_after.hand().palm();
            trajectory.push((palm.x(), palm.y(), palm.depth()));
            if observation
                .sample
                .contacts()
                .iter()
                .any(|contact| contact.pressure() > 0)
            {
                first_contact = Some(observation.sequence);
                break;
            }
        }

        assert!(first_contact.is_none());
        let tail = &trajectory[trajectory.len() - 6..];
        assert_eq!(&tail[..3], &tail[3..]);
        assert_eq!(
            tail[..3].iter().copied().collect::<BTreeSet<_>>(),
            BTreeSet::from([(1023, 1007, 1023), (1007, 1023, 1023), (1023, 1023, 1007),])
        );
    }

    #[test]
    fn palm_component_makes_translation_axes_compete_locally() {
        let config = causal_delta_palm_component_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let observation = session.step().unwrap();
        let changed_translation_axes = observation
            .body
            .movements
            .iter()
            .filter(|movement| {
                movement.changed
                    && matches!(
                        movement.axis,
                        BodyAxis::PalmHorizontal | BodyAxis::PalmVertical | BodyAxis::PalmDepth
                    )
            })
            .count();

        assert_eq!(
            changed_translation_axes, 1,
            "movements={:#?}\ncrossings={:#?}",
            observation.body.movements, observation.body.crossings
        );
    }

    #[test]
    fn palm_component_misses_contact_after_a_lower_depth_cycle() {
        let config = causal_delta_palm_component_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = session.save().unwrap();
        let mut observations = Vec::new();
        let mut first_contact = None;
        for _ in 0..120 {
            let observation = session.step().unwrap();
            if observation
                .sample
                .contacts()
                .iter()
                .any(|contact| contact.pressure() > 0)
            {
                first_contact = Some(observation.sequence);
            }
            observations.push(observation);
            if first_contact.is_some() {
                break;
            }
        }

        let trajectory = observations
            .iter()
            .map(|observation| {
                let palm = observation.body.state_after.hand().palm();
                (palm.x(), palm.y(), palm.depth())
            })
            .collect::<Vec<_>>();
        assert!(first_contact.is_none());
        let tail = &trajectory[trajectory.len() - 6..];
        assert_eq!(&tail[..3], &tail[3..]);
        assert_eq!(
            tail[..3]
                .iter()
                .filter(|position| **position == (16, 800, 0))
                .count(),
            2
        );
        assert!(tail[..3].contains(&(16, 800, 16)));
        assert!(
            observations
                .iter()
                .all(|observation| observation.body.naturally_quiescent)
        );

        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        for expected in observations {
            assert_eq!(replay.step().unwrap(), expected);
        }
    }

    #[test]
    fn palm_component_outcome_survives_but_expires_before_first_executable_choice() {
        let config = causal_delta_palm_component_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let mut release = None;
        for _ in 0..120 {
            let observation = session.step().unwrap();
            let before = observation.body.state_before.hand().palm().depth();
            let after = observation.body.state_after.hand().palm().depth();
            if before == 0 && after > before {
                release = Some(observation);
                break;
            }
        }
        let release = release.expect("the retained arm releases lower depth once");
        let continuation = session.step().unwrap();
        let palm_diagnostics = continuation
            .body
            .choice_diagnostics
            .iter()
            .filter(|diagnostic| match diagnostic {
                ResearchChoiceDiagnostic::Candidate { control, .. }
                | ResearchChoiceDiagnostic::TransitionContinuation { control, .. } => {
                    control.axis() == BodyAxis::PalmDepth
                }
                ResearchChoiceDiagnostic::Choice {
                    ordinary_control,
                    current_transition_control,
                    computed_winner_control,
                    ..
                } => ordinary_control
                    .iter()
                    .chain(current_transition_control.iter())
                    .chain(computed_winner_control.iter())
                    .any(|control| control.axis() == BodyAxis::PalmDepth),
                ResearchChoiceDiagnostic::CompletedCycle { control, .. } => {
                    control.axis() == BodyAxis::PalmDepth
                }
                ResearchChoiceDiagnostic::ConsequenceRecorded { .. }
                | ResearchChoiceDiagnostic::ConsequenceConsumed { .. } => false,
            })
            .collect::<Vec<_>>();
        let composition_diagnostics = release
            .body
            .choice_diagnostics
            .iter()
            .chain(continuation.body.choice_diagnostics.iter())
            .filter(|diagnostic| match diagnostic {
                ResearchChoiceDiagnostic::ConsequenceRecorded { tick, .. } => {
                    (424..=429).contains(tick)
                }
                ResearchChoiceDiagnostic::CompletedCycle { tick, control, .. } => {
                    *tick == 429 && control.axis() == BodyAxis::PalmDepth
                }
                _ => false,
            })
            .collect::<Vec<_>>();

        assert_eq!(release.body.state_after.hand().palm().depth(), 16);
        assert_eq!(continuation.body.state_after.hand().palm().depth(), 0);
        assert!(composition_diagnostics.iter().any(|diagnostic| matches!(
            diagnostic,
            ResearchChoiceDiagnostic::ConsequenceRecorded {
                tick: 424,
                link: 2509,
                ..
            }
        )));
        assert!(composition_diagnostics.iter().any(|diagnostic| matches!(
            diagnostic,
            ResearchChoiceDiagnostic::CompletedCycle {
                tick: 429,
                control: BodyControl::PalmDepth {
                    direction: Direction::Increase
                },
                consequence_tick: Some(424),
                consequence_witnesses,
                unique_latest_tick: None,
                admitted: false,
                ..
            } if consequence_witnesses == &vec![(2509, 24)]
        )));
        assert!(palm_diagnostics.iter().any(|diagnostic| matches!(
            diagnostic,
            ResearchChoiceDiagnostic::Choice {
                tick: 429,
                computed_winner_control: Some(BodyControl::PalmDepth {
                    direction: Direction::Decrease
                }),
                computed_winner_basis,
                ..
            } if computed_winner_basis == "Ordinary"
        )));
    }

    #[test]
    fn first_choice_lifetime_presses_and_releases_real_keys() {
        let config = causal_delta_palm_component_outcome_lifetime_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let mut trajectory = Vec::new();
        let mut states = Vec::new();
        let mut first_contact = None;
        let mut contact_pose = None;
        let mut contact_pressures = None;
        let mut contact_checkpoint = None;
        let mut contact_observation = None;
        let mut first_device_event = None;
        let mut first_device_text = None;
        let mut first_key_release = None;
        let mut all_keys_released = None;
        let mut release_checkpoint = None;
        let mut release_observation = None;
        let mut first_cycle = None;
        let mut consumed = 0_u64;
        for _ in 0..120 {
            let observation = session.step().unwrap();
            let consumed_this_step = observation
                .body
                .choice_diagnostics
                .iter()
                .filter(|diagnostic| {
                    matches!(
                        diagnostic,
                        ResearchChoiceDiagnostic::ConsequenceConsumed { .. }
                    )
                })
                .count();
            consumed =
                consumed.saturating_add(u64::try_from(consumed_this_step).unwrap_or(u64::MAX));
            let palm = observation.body.state_after.hand().palm();
            trajectory.push((palm.x(), palm.y(), palm.depth()));
            if first_contact.is_none()
                && observation
                    .sample
                    .contacts()
                    .iter()
                    .any(|contact| contact.pressure() > 0)
            {
                first_contact = Some(observation.sequence);
                contact_pose = trajectory.last().copied();
                contact_pressures = Some(
                    observation
                        .sample
                        .contacts()
                        .iter()
                        .map(|contact| contact.pressure())
                        .collect::<Vec<_>>(),
                );
                contact_observation = Some(observation.clone());
            }
            if first_device_event.is_none() && !observation.device_events.is_empty() {
                first_device_text = Some(observation.device_after.text().to_owned());
                first_device_event =
                    Some((observation.sequence, observation.device_events.clone()));
            }
            if observation
                .device_events
                .iter()
                .any(|event| matches!(event, DeviceEvent::KeyReleased { .. }))
            {
                first_key_release.get_or_insert_with(|| {
                    (observation.sequence, observation.device_events.clone())
                });
            }
            if first_device_event.is_some() && observation.device_after.keys_down().next().is_none()
            {
                all_keys_released = Some(observation.sequence);
                release_observation = Some(observation);
                break;
            }
            if let Some(start) = states
                .iter()
                .position(|state| state == &observation.session_fingerprint)
            {
                first_cycle = Some((start, observation.sequence));
                break;
            }
            states.push(observation.session_fingerprint);
            if trajectory.len() == 19 {
                contact_checkpoint = Some(session.save().unwrap());
            }
            if trajectory.len() == 61 {
                release_checkpoint = Some(session.save().unwrap());
            }
        }

        assert!(consumed > 0, "the candidate never consumed an outcome");
        assert_eq!(
            first_contact,
            Some(19),
            "first_cycle={first_cycle:?}; trajectory={trajectory:#?}"
        );
        assert_eq!(contact_pose, Some((528, 768, 560)));
        assert_eq!(contact_pressures, Some(vec![0, 0, 8, 8, 8, 8]));
        assert!(first_cycle.is_none());
        assert_eq!(
            first_device_event,
            Some((
                26,
                vec![
                    DeviceEvent::KeyPressed { key: 42 },
                    DeviceEvent::KeyPressed { key: 43 },
                    DeviceEvent::KeyPressed { key: 80 },
                    DeviceEvent::KeyPressed { key: 81 },
                    DeviceEvent::TextChanged,
                ]
            ))
        );
        assert_eq!(first_device_text.as_deref(), Some("]\\"));
        assert_eq!(
            first_key_release,
            Some((
                29,
                vec![
                    DeviceEvent::KeyReleased { key: 43 },
                    DeviceEvent::KeyReleased { key: 80 },
                    DeviceEvent::KeyReleased { key: 81 },
                    DeviceEvent::KeyPressed { key: 91 },
                    DeviceEvent::TextChanged,
                ]
            ))
        );
        assert_eq!(all_keys_released, Some(61));

        let mut replay = WorkstationSession::restore_research_config(
            contact_checkpoint.expect("pre-contact checkpoint is retained"),
            config,
        )
        .unwrap();
        assert_eq!(
            replay.step().unwrap(),
            contact_observation.expect("contact observation is retained")
        );
        let mut replay = WorkstationSession::restore_research_config(
            release_checkpoint.expect("pre-release checkpoint is retained"),
            config,
        )
        .unwrap();
        assert_eq!(
            replay.step().unwrap(),
            release_observation.expect("release observation is retained")
        );
    }

    #[test]
    fn monitor_cue_action_outcome_reuse_retains_the_first_broken_arrow() {
        let trace = capture_monitor_cue_action_outcome_reuse(64, 12).unwrap();
        assert!(trace.exact_replay);
        assert!(trace.naturally_quiescent);
        assert!(trace.paired_initial_body_equality);
        assert!(trace.paired_initial_device_equality);
        assert!(trace.paired_initial_learner_equality);
        assert_eq!(trace.development_steps, trace.development.len());
        assert_eq!(trace.cue_a.len(), 12);
        assert_eq!(trace.cue_b.len(), 12);
        assert_eq!(trace.blank.len(), 12);
        assert_eq!(trace.unlearned.len(), 12);
        assert!(matches!(
            trace.frontier,
            "development_key_pair"
                | "monitor_image_return"
                | "development_retinal_return"
                | "action_outcome_return"
                | "cue_render"
                | "cue_retina"
                | "cue_learner"
                | "cue_path_reuse"
                | "cue_executable_choice"
                | "cue_outward_effect"
                | "keyboard_search"
        ));
        if trace.learned_glyphs.len() >= 2 {
            assert!(trace.first_text_change.is_some());
            assert!(trace.first_monitor_image_after_text.is_some());
            assert!(trace.first_probe_image_divergence.is_some());
        }
    }

    #[test]
    fn visible_key_pair_localizes_the_first_failed_map() {
        let trace = capture_visible_key_intention_pair(12).unwrap();
        assert!(trace.exact_initial_body_equality);
        assert_eq!(trace.first_image_divergence, Some(0));
        assert_eq!(trace.first_retinal_divergence, Some(1));
        assert_eq!(trace.first_learner_divergence, Some(1));
        assert_eq!(trace.first_choice_divergence, None);
        assert_eq!(trace.first_movement_divergence, None);
        assert_eq!(trace.first_body_state_divergence, None);
        assert_eq!(trace.first_key_a_press, None);
        assert_eq!(trace.first_key_b_press, None);
    }

    #[test]
    fn visual_transition_preserves_the_candidate_to_choice_falsifier() {
        let trace = capture_visual_transition_opposition_pair(12).unwrap();
        assert_eq!(trace.first_retinal_divergence, Some(1));
        assert_eq!(trace.first_retinal_transition_a, Some(1));
        assert_eq!(trace.first_retinal_transition_b, Some(1));
        assert_eq!(trace.first_candidate_divergence, Some(3));
        assert_eq!(trace.first_choice_divergence, None);
        assert_eq!(trace.first_movement_divergence, None);
    }

    #[test]
    fn retinotopic_visual_transition_preserves_the_magnitude_falsifier() {
        let trace = capture_retinotopic_visual_transition_pair(12).unwrap();
        assert_eq!(trace.first_retinal_divergence, Some(1));
        assert_eq!(trace.first_retinal_transition_a, Some(1));
        assert_eq!(trace.first_retinal_transition_b, Some(1));
        assert_eq!(trace.first_candidate_divergence, None);
        assert_eq!(trace.first_choice_divergence, None);
        assert_eq!(trace.first_movement_divergence, None);
    }

    #[test]
    fn impulse_magnitude_preserves_the_threshold_falsifier() {
        let trace = capture_magnitude_retinotopic_visual_transition_pair(12).unwrap();
        assert_eq!(trace.first_retinal_divergence, Some(1));
        assert_eq!(trace.first_retinal_transition_a, Some(1));
        assert_eq!(trace.first_retinal_transition_b, Some(1));
        assert_eq!(trace.first_learner_divergence, Some(1));
        assert_eq!(trace.first_candidate_divergence, None);
        assert_eq!(trace.first_choice_divergence, None);
        assert_eq!(trace.first_movement_divergence, None);
    }

    #[test]
    fn threshold_retinotopic_visual_transition_reaches_an_executable_choice() {
        let trace = capture_threshold_retinotopic_visual_transition_pair(12).unwrap();
        assert_eq!(trace.first_retinal_divergence, Some(1));
        assert_eq!(trace.first_retinal_transition_a, Some(1));
        assert_eq!(trace.first_retinal_transition_b, Some(1));
        assert!(
            trace.first_choice_divergence.is_some() || trace.first_movement_divergence.is_some(),
            "threshold-preserving retinotopic transition never reached choice: {trace:#?}"
        );
        assert!(
            trace
                .run_a
                .iter()
                .zip(&trace.run_b)
                .all(|(a, b)| a.state_after.hand().palm() == b.state_after.hand().palm())
        );
    }

    #[test]
    fn visual_reach_moves_eyes_and_palm_toward_opposite_real_keys() {
        let trace = capture_visual_reach_pair(12).unwrap();
        assert_eq!(trace.first_retinal_transition_a, Some(1));
        assert_eq!(trace.first_retinal_transition_b, Some(1));
        for eye in Eye::ALL {
            assert!(trace.run_a.iter().any(|step| {
                step.movements.iter().any(|movement| {
                    movement.axis == BodyAxis::EyeHorizontal { eye } && movement.net_impulse < 0
                })
            }));
            assert!(trace.run_b.iter().any(|step| {
                step.movements.iter().any(|movement| {
                    movement.axis == BodyAxis::EyeHorizontal { eye } && movement.net_impulse > 0
                })
            }));
        }
        assert!(trace.run_a.iter().any(|step| {
            step.movements.iter().any(|movement| {
                movement.axis == BodyAxis::PalmHorizontal && movement.net_impulse < 0
            })
        }));
        assert!(trace.run_b.iter().any(|step| {
            step.movements.iter().any(|movement| {
                movement.axis == BodyAxis::PalmHorizontal && movement.net_impulse > 0
            })
        }));
    }

    #[test]
    fn foveal_visibility_alone_does_not_close_the_reach() {
        let trace = capture_foveal_visual_reach_pair(64).unwrap();
        let geometry = WorldGeometry::standard_ansi_104().unwrap();
        for (run, key) in [(&trace.run_a, KeyId(26)), (&trace.run_b, KeyId(87))] {
            let rect = geometry.key(key).unwrap().rect;
            assert!(run.iter().any(|step| {
                let x = step.state_after.hand().palm().x();
                x >= rect.x && x < rect.right()
            }));
            assert!(run.iter().any(|step| {
                let x = step.state_after.hand().palm().x();
                x == 0 || x == truelearner_workstation::BODY_MAX
            }));
        }
    }
}
