use super::foveal_visual_reach_config;
use academy_workstation::{
    DeviceEvent, KeyId, SessionCheckpoint, SessionObservation, WorkstationPresentation,
    WorkstationSession, WorldError, WorldGeometry,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use truelearner_workstation::{
    BodyMovement, Digit, Eye, HandPoint, Point, ResearchChoiceDiagnostic, ResearchHarnessConfig,
    ResearchRuntimeAttachmentObservation,
};
use workstation_contact_contingency::EVIDENCE_SEED;

const MAX_RESIDENT_BYTES: usize = 512 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct CompactStep {
    sequence: u64,
    projection_sha256: String,
    image_sha256: [String; 2],
    focused_field_sha256: String,
    focused_active_regions: [usize; 2],
    learner_fingerprint: String,
    candidate_sha256: String,
    choice_sha256: String,
    consequence_recorded: bool,
    movements: Vec<BodyMovement>,
    eye_gaze: [Point; 2],
    fingertips: [HandPoint; 5],
    contact_pressure: Vec<u16>,
    device_events: Vec<DeviceEvent>,
    keys_down: Vec<u16>,
    runtime_attachment: ResearchRuntimeAttachmentObservation,
    naturally_quiescent: bool,
    physical_work: u64,
    resident_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ProbeTrace {
    presentation: &'static str,
    glyph: Option<char>,
    intended_key: Option<u16>,
    exact_replay: bool,
    replayed_steps: usize,
    steps: Vec<CompactStep>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeAttachedStage {
    Development,
    DevelopmentReplay,
    LearnedA,
    LearnedAReplay,
    LearnedB,
    LearnedBReplay,
    Blank,
    BlankReplay,
    Unlearned,
    UnlearnedReplay,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RuntimeAttachedProgress {
    pub stage: RuntimeAttachedStage,
    pub stage_step: usize,
    pub stage_budget: usize,
    pub completed_steps: usize,
    pub planned_max_steps: usize,
    pub replay: bool,
    pub physical_work: u64,
    pub resident_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RuntimeAttachedExecution {
    pub planned_max_steps: usize,
    pub completed_steps: usize,
    pub primary_steps: usize,
    pub replay_steps: usize,
}

struct ProgressTracker<'a> {
    planned_max_steps: usize,
    completed_steps: usize,
    primary_steps: usize,
    replay_steps: usize,
    report: &'a mut dyn FnMut(RuntimeAttachedProgress),
}

impl ProgressTracker<'_> {
    fn record(
        &mut self,
        stage: RuntimeAttachedStage,
        stage_step: usize,
        stage_budget: usize,
        replay: bool,
        observation: &SessionObservation,
    ) {
        self.absorb_one(RuntimeAttachedProgress {
            stage,
            stage_step,
            stage_budget,
            completed_steps: 0,
            planned_max_steps: self.planned_max_steps,
            replay,
            physical_work: observation.body.metrics.physical_work,
            resident_bytes: observation.body.metrics.resident_bytes,
        });
    }

    fn absorb(&mut self, progress: Vec<RuntimeAttachedProgress>) {
        for progress in progress {
            self.absorb_one(progress);
        }
    }

    fn absorb_one(&mut self, mut progress: RuntimeAttachedProgress) {
        self.completed_steps = self.completed_steps.saturating_add(1);
        if progress.replay {
            self.replay_steps = self.replay_steps.saturating_add(1);
        } else {
            self.primary_steps = self.primary_steps.saturating_add(1);
        }
        progress.completed_steps = self.completed_steps;
        progress.planned_max_steps = self.planned_max_steps;
        (self.report)(progress);
    }

    fn execution(&self) -> RuntimeAttachedExecution {
        RuntimeAttachedExecution {
            planned_max_steps: self.planned_max_steps,
            completed_steps: self.completed_steps,
            primary_steps: self.primary_steps,
            replay_steps: self.replay_steps,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RuntimeAttachedLadder {
    world_presentation: Option<u64>,
    self_caused_visible_outcome: Option<u64>,
    key_specific_association: Option<u64>,
    cue_first_path_reuse: Option<u64>,
    cue_first_executable_choice: Option<u64>,
    cue_continuity_across_gaze: Option<u64>,
    active_keyboard_search: Option<u64>,
    cross_surface_correspondence: Option<u64>,
    closed_visual_reach: Option<u64>,
    single_finger_touch: Option<u64>,
    intended_key_press: Option<u64>,
    visible_press_consequence: Option<u64>,
    intended_key_release: Option<u64>,
    natural_quiet: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RuntimeAttachedControls {
    construction_inert: bool,
    paired_initial_body_equality: bool,
    paired_initial_device_equality: bool,
    paired_initial_learner_equality: bool,
    blank_distinct_from_learned: bool,
    unlearned_distinct_from_learned: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    runtime_attachment_every_step: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RuntimeAttachedCompleteTrace {
    schema: &'static str,
    seed: u64,
    development_budget: usize,
    probe_budget: usize,
    resident_limit_bytes: usize,
    execution: RuntimeAttachedExecution,
    cost_limit_hit: bool,
    learned_glyphs: Vec<char>,
    source_keys: Vec<u16>,
    ladder: RuntimeAttachedLadder,
    controls: RuntimeAttachedControls,
    frontier: &'static str,
    development: Vec<CompactStep>,
    cue_a: ProbeTrace,
    cue_b: ProbeTrace,
    blank: ProbeTrace,
    unlearned: ProbeTrace,
}

pub fn capture_runtime_attached_complete_candidate(
    development_budget: usize,
    probe_budget: usize,
) -> Result<RuntimeAttachedCompleteTrace, WorldError> {
    capture_runtime_attached_complete_candidate_with_progress(
        development_budget,
        probe_budget,
        |_| {},
    )
}

pub fn capture_runtime_attached_complete_candidate_with_progress(
    development_budget: usize,
    probe_budget: usize,
    mut report: impl FnMut(RuntimeAttachedProgress),
) -> Result<RuntimeAttachedCompleteTrace, WorldError> {
    if development_budget == 0
        || development_budget > 240
        || probe_budget == 0
        || probe_budget > 120
    {
        return Err(WorldError::InvalidRecording);
    }
    let control_budget = probe_budget.min(8);
    let planned_max_steps = development_budget
        .saturating_mul(2)
        .saturating_add(probe_budget.saturating_mul(2))
        .saturating_add(control_budget.saturating_mul(2))
        .saturating_add(4);
    let mut progress = ProgressTracker {
        planned_max_steps,
        completed_steps: 0,
        primary_steps: 0,
        replay_steps: 0,
        report: &mut report,
    };
    let config = foveal_visual_reach_config();
    let geometry = WorldGeometry::standard_ansi_104()?;
    let mut session = WorkstationSession::new_research_runtime_attached_workstation(
        EVIDENCE_SEED,
        config,
        WorkstationPresentation::default(),
    )?;
    let initial = session.read()?;
    let construction_inert = initial.sequence == 0
        && initial.body.physical_tick == 0
        && initial.body.pending_transitions.is_empty();
    let development_checkpoint = session.save()?;
    let mut development_observations = Vec::with_capacity(development_budget);
    let mut development = Vec::with_capacity(development_budget);
    let mut learned_glyphs = Vec::new();
    let mut source_keys = Vec::new();
    let mut all_released_after_pair = false;

    for index in 0..development_budget {
        let observation = session.step()?;
        progress.record(
            RuntimeAttachedStage::Development,
            index.saturating_add(1),
            development_budget,
            false,
            &observation,
        );
        for event in &observation.device_events {
            if let DeviceEvent::KeyPressed { key } = event
                && let Some(key_geometry) = geometry.key(KeyId(*key))
            {
                let mut characters = key_geometry.label.chars();
                if let Some(glyph) = characters.next()
                    && characters.next().is_none()
                    && glyph.is_ascii_graphic()
                    && !learned_glyphs.contains(&glyph)
                {
                    learned_glyphs.push(glyph);
                    source_keys.push(*key);
                }
            }
        }
        all_released_after_pair =
            learned_glyphs.len() >= 2 && observation.device_after.keys_down().next().is_none();
        let compact = compact_step(&observation);
        development.push(compact);
        development_observations.push(observation);
        if all_released_after_pair
            || development
                .last()
                .is_some_and(|step| step.resident_bytes >= MAX_RESIDENT_BYTES)
        {
            break;
        }
    }
    let development_final = session.save()?;
    let base_checkpoint = development_final.clone();
    let base_read = session.read()?;
    drop(session);
    let glyph_a = learned_glyphs.first().copied();
    let glyph_b = learned_glyphs.get(1).copied();
    let key_a = source_keys.first().copied();
    let key_b = source_keys.get(1).copied();
    let dependent_probe_budget = if all_released_after_pair {
        probe_budget
    } else {
        1
    };
    let planned = progress.planned_max_steps;
    let (development_replay, cue_a, cue_b, blank, unlearned, read_a, read_b) =
        std::thread::scope(|scope| {
            let development = scope.spawn(move || {
                replay_observations_with_progress(
                    development_checkpoint,
                    config,
                    &development_observations,
                    &development_final,
                    RuntimeAttachedStage::DevelopmentReplay,
                    planned,
                )
            });
            let learned_a_checkpoint = base_checkpoint.clone();
            let learned_a = scope.spawn(move || {
                run_probe_with_progress(
                    learned_a_checkpoint,
                    config,
                    ProbeSpec {
                        presentation: "learned_a",
                        glyph: glyph_a.or(Some('?')),
                        intended_key: key_a,
                        steps: dependent_probe_budget,
                        stage: RuntimeAttachedStage::LearnedA,
                        replay_stage: RuntimeAttachedStage::LearnedAReplay,
                    },
                    planned,
                )
            });
            let learned_b_checkpoint = base_checkpoint.clone();
            let learned_b = scope.spawn(move || {
                run_probe_with_progress(
                    learned_b_checkpoint,
                    config,
                    ProbeSpec {
                        presentation: "learned_b",
                        glyph: glyph_b.or(Some('!')),
                        intended_key: key_b,
                        steps: dependent_probe_budget,
                        stage: RuntimeAttachedStage::LearnedB,
                        replay_stage: RuntimeAttachedStage::LearnedBReplay,
                    },
                    planned,
                )
            });
            let blank_checkpoint = base_checkpoint.clone();
            let blank = scope.spawn(move || {
                run_probe_with_progress(
                    blank_checkpoint,
                    config,
                    ProbeSpec {
                        presentation: "blank",
                        glyph: None,
                        intended_key: None,
                        steps: control_budget,
                        stage: RuntimeAttachedStage::Blank,
                        replay_stage: RuntimeAttachedStage::BlankReplay,
                    },
                    planned,
                )
            });
            let unlearned_checkpoint = base_checkpoint.clone();
            let unlearned = scope.spawn(move || {
                run_probe_with_progress(
                    unlearned_checkpoint,
                    config,
                    ProbeSpec {
                        presentation: "unlearned",
                        glyph: Some('?'),
                        intended_key: None,
                        steps: control_budget,
                        stage: RuntimeAttachedStage::Unlearned,
                        replay_stage: RuntimeAttachedStage::UnlearnedReplay,
                    },
                    planned,
                )
            });
            let paired_a_checkpoint = base_checkpoint.clone();
            let paired_a = scope.spawn(move || {
                let mut session =
                    WorkstationSession::restore_research_runtime_attached_workstation(
                        paired_a_checkpoint,
                        config,
                    )?;
                if let Some(glyph) = glyph_a.or(Some('?')) {
                    session.set_presentation(WorkstationPresentation::with_monitor_glyph(glyph))?;
                }
                session.read()
            });
            let paired_b_checkpoint = base_checkpoint.clone();
            let paired_b = scope.spawn(move || {
                let mut session =
                    WorkstationSession::restore_research_runtime_attached_workstation(
                        paired_b_checkpoint,
                        config,
                    )?;
                if let Some(glyph) = glyph_b.or(Some('!')) {
                    session.set_presentation(WorkstationPresentation::with_monitor_glyph(glyph))?;
                }
                session.read()
            });
            let development = development
                .join()
                .map_err(|_| WorldError::InvalidRecording)??;
            let learned_a = learned_a
                .join()
                .map_err(|_| WorldError::InvalidRecording)??;
            let learned_b = learned_b
                .join()
                .map_err(|_| WorldError::InvalidRecording)??;
            let blank = blank.join().map_err(|_| WorldError::InvalidRecording)??;
            let unlearned = unlearned
                .join()
                .map_err(|_| WorldError::InvalidRecording)??;
            let paired_a = paired_a
                .join()
                .map_err(|_| WorldError::InvalidRecording)??;
            let paired_b = paired_b
                .join()
                .map_err(|_| WorldError::InvalidRecording)??;
            Ok::<_, WorldError>((
                development,
                learned_a,
                learned_b,
                blank,
                unlearned,
                paired_a,
                paired_b,
            ))
        })?;
    progress.absorb(development_replay.1);
    progress.absorb(cue_a.1);
    progress.absorb(cue_b.1);
    progress.absorb(blank.1);
    progress.absorb(unlearned.1);
    let development_replay = development_replay.0;
    let cue_a = cue_a.0;
    let cue_b = cue_b.0;
    let blank = blank.0;
    let unlearned = unlearned.0;

    let ladder = ladder(
        &geometry,
        &development,
        &cue_a.steps,
        &cue_b.steps,
        key_a,
        key_b,
        learned_glyphs.len() >= 2 && all_released_after_pair,
    );
    let frontier = frontier(&ladder);
    let exact_replay = development_replay
        && cue_a.exact_replay
        && cue_b.exact_replay
        && blank.exact_replay
        && unlearned.exact_replay;
    let all_steps = development
        .iter()
        .chain(&cue_a.steps)
        .chain(&cue_b.steps)
        .chain(&blank.steps)
        .chain(&unlearned.steps);
    let (naturally_quiescent, runtime_attachment_every_step) =
        all_steps.fold((true, true), |(quiet, attached), step| {
            (
                quiet && step.naturally_quiescent,
                attached && step.runtime_attachment.enabled,
            )
        });
    let blank_distinct_from_learned = cue_a
        .steps
        .first()
        .zip(blank.steps.first())
        .is_some_and(|(learned, blank)| learned.image_sha256 != blank.image_sha256);
    let unlearned_distinct_from_learned = cue_a
        .steps
        .first()
        .zip(unlearned.steps.first())
        .is_some_and(|(learned, unlearned)| learned.image_sha256 != unlearned.image_sha256);

    Ok(RuntimeAttachedCompleteTrace {
        schema: "workstation-runtime-attached-complete-candidate/v3",
        seed: EVIDENCE_SEED,
        development_budget,
        probe_budget,
        resident_limit_bytes: MAX_RESIDENT_BYTES,
        execution: progress.execution(),
        cost_limit_hit: development
            .last()
            .is_some_and(|step| step.resident_bytes >= MAX_RESIDENT_BYTES),
        learned_glyphs,
        source_keys,
        ladder,
        controls: RuntimeAttachedControls {
            construction_inert,
            paired_initial_body_equality: read_a.body.state == read_b.body.state,
            paired_initial_device_equality: read_a.device == read_b.device,
            paired_initial_learner_equality: read_a.body.learner_fingerprint
                == read_b.body.learner_fingerprint
                && read_a.body.learner_fingerprint == base_read.body.learner_fingerprint,
            blank_distinct_from_learned,
            unlearned_distinct_from_learned,
            exact_replay,
            naturally_quiescent,
            runtime_attachment_every_step,
        },
        frontier,
        development,
        cue_a,
        cue_b,
        blank,
        unlearned,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProbeSpec {
    presentation: &'static str,
    glyph: Option<char>,
    intended_key: Option<u16>,
    steps: usize,
    stage: RuntimeAttachedStage,
    replay_stage: RuntimeAttachedStage,
}

fn run_probe_with_progress(
    checkpoint: SessionCheckpoint,
    config: ResearchHarnessConfig,
    spec: ProbeSpec,
    planned_max_steps: usize,
) -> Result<(ProbeTrace, Vec<RuntimeAttachedProgress>), WorldError> {
    let mut reported = Vec::new();
    let trace = {
        let mut report = |progress| reported.push(progress);
        let mut tracker = ProgressTracker {
            planned_max_steps,
            completed_steps: 0,
            primary_steps: 0,
            replay_steps: 0,
            report: &mut report,
        };
        run_probe(checkpoint, config, spec, &mut tracker)?
    };
    Ok((trace, reported))
}

fn replay_observations_with_progress(
    checkpoint: SessionCheckpoint,
    config: ResearchHarnessConfig,
    expected: &[SessionObservation],
    final_checkpoint: &SessionCheckpoint,
    stage: RuntimeAttachedStage,
    planned_max_steps: usize,
) -> Result<(bool, Vec<RuntimeAttachedProgress>), WorldError> {
    let mut reported = Vec::new();
    let exact = {
        let mut report = |progress| reported.push(progress);
        let mut tracker = ProgressTracker {
            planned_max_steps,
            completed_steps: 0,
            primary_steps: 0,
            replay_steps: 0,
            report: &mut report,
        };
        replay_observations(
            checkpoint,
            config,
            expected,
            final_checkpoint,
            stage,
            &mut tracker,
        )?
    };
    Ok((exact, reported))
}

fn run_probe(
    checkpoint: SessionCheckpoint,
    config: ResearchHarnessConfig,
    spec: ProbeSpec,
    progress: &mut ProgressTracker<'_>,
) -> Result<ProbeTrace, WorldError> {
    let mut session =
        WorkstationSession::restore_research_runtime_attached_workstation(checkpoint, config)?;
    session.set_presentation(match spec.glyph {
        Some(glyph) => WorkstationPresentation::with_monitor_glyph(glyph),
        None => WorkstationPresentation::default(),
    })?;
    let replay_checkpoint = session.save()?;
    let mut compact = Vec::with_capacity(spec.steps);
    let mut first_checkpoint = None;
    let mut first_observation = None;
    let (replayed, replay_checkpoint_after) = std::thread::scope(|scope| {
        let replay = scope.spawn(move || {
            let mut replay = WorkstationSession::restore_research_runtime_attached_workstation(
                replay_checkpoint,
                config,
            )?;
            let observation = replay.step()?;
            Ok::<_, WorldError>((observation, replay.save()?))
        });
        for index in 0..spec.steps {
            let observation = session.step()?;
            progress.record(
                spec.stage,
                index.saturating_add(1),
                spec.steps,
                false,
                &observation,
            );
            let step = compact_step(&observation);
            compact.push(step);
            if index == 0 {
                first_checkpoint = Some(session.save()?);
                first_observation = Some(observation);
            }
            if compact
                .last()
                .is_some_and(|step| step.resident_bytes >= MAX_RESIDENT_BYTES)
            {
                break;
            }
        }
        replay.join().map_err(|_| WorldError::InvalidRecording)?
    })?;
    let expected = first_observation.ok_or(WorldError::InvalidRecording)?;
    let expected_checkpoint = first_checkpoint.ok_or(WorldError::InvalidRecording)?;
    progress.record(spec.replay_stage, 1, 1, true, &replayed);
    let exact_replay = replayed == expected && replay_checkpoint_after == expected_checkpoint;
    Ok(ProbeTrace {
        presentation: spec.presentation,
        glyph: spec.glyph,
        intended_key: spec.intended_key,
        exact_replay,
        replayed_steps: 1,
        steps: compact,
    })
}

fn replay_observations(
    checkpoint: SessionCheckpoint,
    config: ResearchHarnessConfig,
    expected: &[SessionObservation],
    final_checkpoint: &SessionCheckpoint,
    stage: RuntimeAttachedStage,
    progress: &mut ProgressTracker<'_>,
) -> Result<bool, WorldError> {
    let mut replay =
        WorkstationSession::restore_research_runtime_attached_workstation(checkpoint, config)?;
    let mut exact = true;
    let replay_steps = expected.len();
    for (index, expected) in expected.iter().enumerate() {
        let observation = replay.step()?;
        progress.record(
            stage,
            index.saturating_add(1),
            replay_steps,
            true,
            &observation,
        );
        exact &= observation == *expected;
    }
    exact &= replay.save()? == *final_checkpoint;
    Ok(exact)
}

fn compact_step(observation: &SessionObservation) -> CompactStep {
    let candidates = observation
        .body
        .choice_diagnostics
        .iter()
        .filter(|diagnostic| matches!(diagnostic, ResearchChoiceDiagnostic::Candidate { .. }))
        .collect::<Vec<_>>();
    let choices = observation
        .body
        .choice_diagnostics
        .iter()
        .filter(|diagnostic| matches!(diagnostic, ResearchChoiceDiagnostic::Choice { .. }))
        .collect::<Vec<_>>();
    let mut compact = CompactStep {
        sequence: observation.sequence,
        projection_sha256: String::new(),
        image_sha256: Eye::ALL.map(|eye| digest(observation.sample.eye(eye).pixels())),
        focused_field_sha256: digest(
            &bincode::serialize(&observation.body.focused_vision.changes).unwrap_or_default(),
        ),
        focused_active_regions: observation.body.focused_vision.active_regions,
        learner_fingerprint: observation.body.learner_fingerprint.clone(),
        candidate_sha256: digest(&bincode::serialize(&candidates).unwrap_or_default()),
        choice_sha256: digest(&bincode::serialize(&choices).unwrap_or_default()),
        consequence_recorded: observation
            .body
            .choice_diagnostics
            .iter()
            .any(|diagnostic| {
                matches!(
                    diagnostic,
                    ResearchChoiceDiagnostic::ConsequenceRecorded { .. }
                )
            }),
        movements: observation.body.movements.clone(),
        eye_gaze: Eye::ALL.map(|eye| observation.body.state_after.eye(eye).gaze()),
        fingertips: Digit::ALL.map(|digit| observation.body.state_after.hand().fingertip(digit)),
        contact_pressure: observation
            .sample
            .contacts()
            .iter()
            .map(|contact| contact.pressure())
            .collect(),
        device_events: observation.device_events.clone(),
        keys_down: observation
            .device_after
            .keys_down()
            .map(|key| key.0)
            .collect(),
        runtime_attachment: observation.body.runtime_attachment.clone(),
        naturally_quiescent: observation.body.naturally_quiescent,
        physical_work: observation.body.metrics.physical_work,
        resident_bytes: observation.body.metrics.resident_bytes,
    };
    compact.projection_sha256 = digest(&bincode::serialize(&compact).unwrap_or_default());
    compact
}

fn ladder(
    geometry: &WorldGeometry,
    development: &[CompactStep],
    cue_a: &[CompactStep],
    cue_b: &[CompactStep],
    key_a: Option<u16>,
    key_b: Option<u16>,
    developed_pair: bool,
) -> RuntimeAttachedLadder {
    let first_text = development.iter().position(|step| {
        step.device_events
            .iter()
            .any(|event| matches!(event, DeviceEvent::TextChanged))
    });
    let self_caused_visible_outcome = first_text.and_then(|index| {
        let before = development.get(index)?;
        development
            .iter()
            .skip(index.saturating_add(1))
            .find(|step| step.image_sha256 != before.image_sha256)
            .map(|step| step.sequence)
    });
    let key_specific_association = developed_pair
        .then(|| development_association(development, key_a, key_b))
        .flatten();
    let world_presentation = first_divergence(cue_a, cue_b, |step| &step.image_sha256);
    let cue_retina = world_presentation.and_then(|after| {
        first_divergence_after(cue_a, cue_b, after, |step| &step.focused_field_sha256)
    });
    let cue_learner = cue_retina.and_then(|after| {
        first_divergence_after(cue_a, cue_b, after, |step| &step.learner_fingerprint)
    });
    let cue_first_path_reuse = cue_learner.and_then(|after| {
        first_divergence_after(cue_a, cue_b, after, |step| &step.candidate_sha256)
    });
    let cue_first_executable_choice = cue_first_path_reuse
        .and_then(|after| first_divergence_after(cue_a, cue_b, after, |step| &step.choice_sha256));
    let cue_continuity_across_gaze = cue_first_executable_choice
        .and_then(|after| first_divergence_after(cue_a, cue_b, after, |step| &step.movements));
    let active_keyboard_search = cue_continuity_across_gaze.and_then(|after| {
        both_branch_first_after(cue_a, cue_b, after, |step, _| {
            step.eye_gaze
                .iter()
                .any(|gaze| geometry.keyboard.contains_xy(gaze.x(), gaze.y()))
        })
    });
    let cross_surface_correspondence = active_keyboard_search.and_then(|after| {
        both_key_first_after(geometry, cue_a, cue_b, key_a, key_b, after, |step, key| {
            step.eye_gaze
                .iter()
                .any(|gaze| key.rect.contains_xy(gaze.x(), gaze.y()))
        })
    });
    let closed_visual_reach = cross_surface_correspondence.and_then(|after| {
        both_key_first_after(geometry, cue_a, cue_b, key_a, key_b, after, |step, key| {
            step.fingertips
                .iter()
                .any(|tip| key.rect.contains_xy(tip.x(), tip.y()))
        })
    });
    let single_finger_touch = closed_visual_reach.and_then(|after| {
        both_branch_first_after(cue_a, cue_b, after, |step, _| {
            step.contact_pressure
                .iter()
                .filter(|pressure| **pressure > 0)
                .count()
                == 1
        })
    });
    let intended_key_press = single_finger_touch.and_then(|after| {
        both_key_first_after(geometry, cue_a, cue_b, key_a, key_b, after, |step, key| {
            step.device_events.iter().any(
                |event| matches!(event, DeviceEvent::KeyPressed { key: pressed } if *pressed == key.id.0),
            )
        })
    });
    let visible_press_consequence = intended_key_press.and_then(|after| {
        both_branch_first_after(cue_a, cue_b, after, |step, _| {
            step.device_events
                .iter()
                .any(|event| matches!(event, DeviceEvent::TextChanged))
        })
    });
    let intended_key_release = visible_press_consequence.and_then(|after| {
        both_key_first_after(geometry, cue_a, cue_b, key_a, key_b, after, |step, key| {
            step.device_events.iter().any(
                |event| matches!(event, DeviceEvent::KeyReleased { key: released } if *released == key.id.0),
            )
        })
    });
    let natural_quiet = intended_key_release.and_then(|after| {
        both_branch_first_after(cue_a, cue_b, after, |step, _| {
            step.naturally_quiescent && step.keys_down.is_empty()
        })
    });

    RuntimeAttachedLadder {
        world_presentation,
        self_caused_visible_outcome,
        key_specific_association,
        cue_first_path_reuse,
        cue_first_executable_choice,
        cue_continuity_across_gaze,
        active_keyboard_search,
        cross_surface_correspondence,
        closed_visual_reach,
        single_finger_touch,
        intended_key_press,
        visible_press_consequence,
        intended_key_release,
        natural_quiet,
    }
}

fn frontier(ladder: &RuntimeAttachedLadder) -> &'static str {
    for (name, observed) in [
        ("world_presentation", ladder.world_presentation),
        (
            "self_caused_visible_outcome",
            ladder.self_caused_visible_outcome,
        ),
        ("key_specific_association", ladder.key_specific_association),
        ("cue_first_path_reuse", ladder.cue_first_path_reuse),
        (
            "cue_first_executable_choice",
            ladder.cue_first_executable_choice,
        ),
        (
            "cue_continuity_across_gaze",
            ladder.cue_continuity_across_gaze,
        ),
        ("active_keyboard_search", ladder.active_keyboard_search),
        (
            "cross_surface_correspondence",
            ladder.cross_surface_correspondence,
        ),
        ("closed_visual_reach", ladder.closed_visual_reach),
        ("single_finger_touch", ladder.single_finger_touch),
        ("intended_key_press", ladder.intended_key_press),
        (
            "visible_press_consequence",
            ladder.visible_press_consequence,
        ),
        ("intended_key_release", ladder.intended_key_release),
        ("natural_quiet", ladder.natural_quiet),
    ] {
        if observed.is_none() {
            return name;
        }
    }
    "complete"
}

fn first_divergence<T: PartialEq + ?Sized>(
    left: &[CompactStep],
    right: &[CompactStep],
    field: impl Fn(&CompactStep) -> &T,
) -> Option<u64> {
    left.iter()
        .zip(right)
        .find(|(left, right)| field(left) != field(right))
        .map(|(left, _)| left.sequence)
}

fn first_divergence_after<T: PartialEq + ?Sized>(
    left: &[CompactStep],
    right: &[CompactStep],
    after: u64,
    field: impl Fn(&CompactStep) -> &T,
) -> Option<u64> {
    left.iter()
        .zip(right)
        .find(|(left, right)| left.sequence >= after && field(left) != field(right))
        .map(|(left, _)| left.sequence)
}

fn development_association(
    development: &[CompactStep],
    left_key: Option<u16>,
    right_key: Option<u16>,
) -> Option<u64> {
    [left_key?, right_key?]
        .into_iter()
        .map(|key| {
            let pressed = development.iter().find(|step| {
                step.device_events.iter().any(
                    |event| matches!(event, DeviceEvent::KeyPressed { key: pressed } if *pressed == key),
                )
            })?;
            development
                .iter()
                .find(|step| step.sequence >= pressed.sequence && step.consequence_recorded)
                .map(|step| step.sequence)
        })
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .max()
}

fn both_branch_first_after(
    left: &[CompactStep],
    right: &[CompactStep],
    after: u64,
    predicate: impl Fn(&CompactStep, usize) -> bool,
) -> Option<u64> {
    let left = left
        .iter()
        .find(|step| step.sequence >= after && predicate(step, 0))?
        .sequence;
    let right = right
        .iter()
        .find(|step| step.sequence >= after && predicate(step, 1))?
        .sequence;
    Some(left.max(right))
}

fn both_key_first_after(
    geometry: &WorldGeometry,
    left: &[CompactStep],
    right: &[CompactStep],
    left_key: Option<u16>,
    right_key: Option<u16>,
    after: u64,
    predicate: impl Fn(&CompactStep, &academy_workstation::Key) -> bool,
) -> Option<u64> {
    let left_key = geometry.key(KeyId(left_key?))?;
    let right_key = geometry.key(KeyId(right_key?))?;
    let left = left
        .iter()
        .find(|step| step.sequence >= after && predicate(step, left_key))?
        .sequence;
    let right = right
        .iter()
        .find(|step| step.sequence >= after && predicate(step, right_key))?
        .sequence;
    Some(left.max(right))
}

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_attached_complete_candidate_retains_first_broken_arrow() {
        let started = std::time::Instant::now();
        let mut progress = Vec::new();
        let trace = capture_runtime_attached_complete_candidate_with_progress(2, 1, |step| {
            progress.push(step)
        })
        .unwrap();
        assert!(trace.controls.construction_inert);
        assert!(trace.controls.exact_replay);
        assert!(trace.controls.naturally_quiescent);
        assert!(trace.controls.runtime_attachment_every_step);
        assert_eq!(trace.execution.planned_max_steps, 12);
        assert_eq!(trace.execution.completed_steps, 12);
        assert_eq!(trace.execution.primary_steps, 6);
        assert_eq!(trace.execution.replay_steps, 6);
        assert_eq!(progress.len(), trace.execution.completed_steps);
        assert!(
            progress
                .iter()
                .enumerate()
                .all(|(index, step)| step.completed_steps == index + 1)
        );
        assert!(
            progress
                .iter()
                .all(|step| step.planned_max_steps == trace.execution.planned_max_steps)
        );
        assert_eq!(trace.cue_a.replayed_steps, 1);
        assert_eq!(trace.cue_b.replayed_steps, 1);
        assert_eq!(trace.blank.replayed_steps, 1);
        assert_eq!(trace.unlearned.replayed_steps, 1);
        assert!(!trace.frontier.is_empty());
        let encoded = serde_json::to_vec(&trace).unwrap();
        assert!(encoded.len() < 1_000_000);
        if !cfg!(debug_assertions) {
            assert!(
                started.elapsed() < std::time::Duration::from_millis(500),
                "warm complete fixture must finish in under 0.5 seconds; took {:?}",
                started.elapsed()
            );
        }
    }
}
