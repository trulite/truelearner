use academy_workstation::{
    KeyId, SessionCheckpoint, SessionObservation, WorkstationPresentation, WorkstationSession,
    WorldError,
};
use serde::Serialize;
use std::path::PathBuf;
use truelearner_core::Output;
use truelearner_workstation::{
    BodyAxis, BodyControl, BodyMovement, Protocol, ResearchChoiceDiagnostic,
    ResearchFocusedActionProjection, ResearchHarnessConfig, ResearchOpportunityIncidence,
    ResearchTransitionOpportunity, ResearchVisualComposition, research_focused_feature_for_origin,
};

const SEED: u64 = 91_026_087;
const KEY_A: u16 = 26;
const KEY_B: u16 = 87;
const PROTOCOL_SHA256: &str = "d21bfc97658a61eea946051c2b3b973f911aa36c5dbf03d01aacdf1b390d1788";

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct CandidateWitness {
    tick: i64,
    phase: i32,
    control: BodyControl,
    ownership: String,
    path_inputs: u32,
    path_origins: Vec<u64>,
    focused_features: Vec<usize>,
    admitted_drive: i64,
    projected_drive: i64,
    threshold: i64,
    executable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ChoiceWitness {
    tick: i64,
    phase: i32,
    ordinary_control: Option<BodyControl>,
    current_transition_control: Option<BodyControl>,
    computed_winner_control: Option<BodyControl>,
    admitted_controls: Vec<BodyControl>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ActionStep {
    sequence: u64,
    focused_changed_features: Vec<usize>,
    focused_admitted_transitions: usize,
    admitted_inputs: usize,
    learner_fingerprint: String,
    body_fingerprint: String,
    palm_candidates: Vec<CandidateWitness>,
    choices: Vec<ChoiceWitness>,
    outputs: Vec<Output>,
    palm_movements: Vec<BodyMovement>,
    physical_work: u64,
    resident_bytes: usize,
    naturally_quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct BranchEvidence {
    key: u16,
    steps: Vec<ActionStep>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Divergences {
    focused_receptor: Option<u64>,
    learner: Option<u64>,
    candidate: Option<u64>,
    choice: Option<u64>,
    output: Option<u64>,
    palm_movement: Option<u64>,
}

impl Divergences {
    fn first_action(&self) -> Option<u64> {
        [self.candidate, self.choice, self.output, self.palm_movement]
            .into_iter()
            .flatten()
            .min()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ArmEvidence {
    projection: &'static str,
    base_return_path_count: usize,
    base_resident_bytes: usize,
    seed_focused_transitions: usize,
    paired_initial_body_state_equal: bool,
    paired_initial_device_equal: bool,
    paired_initial_learner_equal: bool,
    semantic_firewall: bool,
    exact_replay_a: bool,
    exact_replay_b: bool,
    reversed_evaluation_order_equal: bool,
    naturally_quiescent: bool,
    first_focused_origin_candidate: Option<(u64, CandidateWitness)>,
    divergences: Divergences,
    cue_a: BranchEvidence,
    cue_b: BranchEvidence,
}

#[derive(Serialize)]
struct Controls {
    one_changed_factor_one_admission_unit_test: &'static str,
    symmetric_two_outcome_projection_unit_test: &'static str,
    version_three_focused_checkpoint_migrates_as_isolated: bool,
    same_cue_repeat_is_exact_replay: bool,
    reversed_branch_evaluation_is_equal: bool,
    isolated_removes_focused_origin_candidate: bool,
    cue_branches_start_equal_inside_each_arm: bool,
    no_evaluator_semantics_enter_organism_sample: bool,
}

#[derive(Serialize)]
struct Evidence {
    schema: &'static str,
    protocol_sha256: &'static str,
    outcome: &'static str,
    frontier: &'static str,
    claim_limit: &'static str,
    seed: u64,
    steps: usize,
    keys: [u16; 2],
    complete: ArmEvidence,
    isolated: ArmEvidence,
    controls: Controls,
}

struct CapturedBranch {
    checkpoint: SessionCheckpoint,
    observations: Vec<SessionObservation>,
    evidence: BranchEvidence,
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let output = PathBuf::from(
        args.next()
            .expect("usage: focused_receptor_action_participation OUTPUT [STEPS]"),
    );
    let steps = args
        .next()
        .map(|value| {
            value
                .to_string_lossy()
                .parse::<usize>()
                .expect("STEPS is an integer")
        })
        .unwrap_or(16);
    assert!(steps > 0 && steps <= 16, "STEPS must be in 1..=16");
    assert!(args.next().is_none(), "too many arguments");

    let complete =
        run_arm(ResearchFocusedActionProjection::PalmHorizontal, steps).expect("complete arm runs");
    let isolated =
        run_arm(ResearchFocusedActionProjection::Isolated, steps).expect("isolated arm runs");
    let complete_action = complete.divergences.first_action();
    let receptor_before_action = complete
        .divergences
        .focused_receptor
        .zip(complete_action)
        .is_some_and(|(receptor, action)| receptor <= action);
    let complete_supported = complete.first_focused_origin_candidate.is_some()
        && complete_action.is_some()
        && receptor_before_action;
    let isolated_control = isolated.first_focused_origin_candidate.is_none()
        && isolated.divergences.focused_receptor.is_some()
        && isolated.divergences.learner.is_some();
    let controls_pass = complete.exact_replay_a
        && complete.exact_replay_b
        && isolated.exact_replay_a
        && isolated.exact_replay_b
        && complete.reversed_evaluation_order_equal
        && isolated.reversed_evaluation_order_equal
        && complete.naturally_quiescent
        && isolated.naturally_quiescent
        && complete.paired_initial_body_state_equal
        && complete.paired_initial_device_equal
        && complete.paired_initial_learner_equal
        && isolated.paired_initial_body_state_equal
        && isolated.paired_initial_device_equal
        && isolated.paired_initial_learner_equal
        && complete.semantic_firewall
        && isolated.semantic_firewall;
    let outcome = if complete_supported && isolated_control && controls_pass {
        "supported"
    } else {
        "falsified"
    };
    let frontier = if !controls_pass {
        "control_integrity"
    } else if complete.divergences.focused_receptor.is_none() {
        "focused_receptor_distinction"
    } else if complete.divergences.learner.is_none() {
        "focused_learner_distinction"
    } else if complete.first_focused_origin_candidate.is_none() {
        "focused_origin_candidate"
    } else if complete_action.is_none() {
        "cue_dependent_action_rung"
    } else if !receptor_before_action {
        "causal_order"
    } else if !isolated_control {
        "projection_removal_control"
    } else {
        "key_directed_reaching"
    };
    let evidence = Evidence {
        schema: "workstation-focused-receptor-action-participation/v1",
        protocol_sha256: PROTOCOL_SHA256,
        outcome,
        frontier,
        claim_limit: "action participation only; not intended key selection, reaching, pressing, or Production authority",
        seed: SEED,
        steps,
        keys: [KEY_A, KEY_B],
        controls: Controls {
            one_changed_factor_one_admission_unit_test: "focused_action_projection_adds_links_without_adding_visual_inputs",
            symmetric_two_outcome_projection_unit_test: "focused_action_projection_is_symmetric_inside_one_palm_component",
            version_three_focused_checkpoint_migrates_as_isolated: true,
            same_cue_repeat_is_exact_replay: complete.exact_replay_a
                && complete.exact_replay_b
                && isolated.exact_replay_a
                && isolated.exact_replay_b,
            reversed_branch_evaluation_is_equal: complete.reversed_evaluation_order_equal
                && isolated.reversed_evaluation_order_equal,
            isolated_removes_focused_origin_candidate: isolated
                .first_focused_origin_candidate
                .is_none(),
            cue_branches_start_equal_inside_each_arm: complete.paired_initial_body_state_equal
                && complete.paired_initial_device_equal
                && complete.paired_initial_learner_equal
                && isolated.paired_initial_body_state_equal
                && isolated.paired_initial_device_equal
                && isolated.paired_initial_learner_equal,
            no_evaluator_semantics_enter_organism_sample: complete.semantic_firewall
                && isolated.semantic_firewall,
        },
        complete,
        isolated,
    };
    std::fs::write(
        output,
        serde_json::to_vec_pretty(&evidence).expect("evidence serializes"),
    )
    .expect("evidence writes");
}

fn run_arm(
    projection: ResearchFocusedActionProjection,
    steps: usize,
) -> Result<ArmEvidence, WorldError> {
    let config = config();
    let visual = ResearchVisualComposition::default()
        .with_focused_sensor_field(true)
        .with_focused_action_projection(projection);
    let mut seed = WorkstationSession::new_research_composed_with_presentation(
        SEED,
        config,
        visual,
        WorkstationPresentation::default(),
    )?;
    let seed_observation = seed.step()?;
    let base_read = seed.read()?;
    let base = seed.save()?;
    let mut branch_a = prepare_branch(base.clone(), config, visual, KEY_A)?;
    let mut branch_b = prepare_branch(base, config, visual, KEY_B)?;
    let read_a = branch_a.read()?;
    let read_b = branch_b.read()?;
    let paired_initial_body_state_equal = read_a.body.state == read_b.body.state;
    let paired_initial_device_equal = read_a.device == read_b.device;
    let paired_initial_learner_equal =
        read_a.body.learner_fingerprint == read_b.body.learner_fingerprint;
    let captured_a = capture_branch(&mut branch_a, KEY_A, steps)?;
    let captured_b = capture_branch(&mut branch_b, KEY_B, steps)?;

    // Replay B before A, deliberately reversing the original A-before-B evaluation order.
    let replay_b = replay_branch(
        captured_b.checkpoint.clone(),
        config,
        visual,
        &captured_b.observations,
    )?;
    let replay_a = replay_branch(
        captured_a.checkpoint.clone(),
        config,
        visual,
        &captured_a.observations,
    )?;
    let reversed_evaluation_order_equal = replay_a && replay_b;
    let semantic_firewall = captured_a
        .observations
        .iter()
        .chain(&captured_b.observations)
        .all(sample_has_no_evaluator_semantics);
    let divergences = divergences(&captured_a.evidence.steps, &captured_b.evidence.steps);
    let first_focused_origin_candidate = captured_a
        .evidence
        .steps
        .iter()
        .chain(&captured_b.evidence.steps)
        .find_map(|step| {
            step.palm_candidates
                .iter()
                .find(|candidate| candidate.executable && !candidate.focused_features.is_empty())
                .cloned()
                .map(|candidate| (step.sequence, candidate))
        });
    let naturally_quiescent = captured_a
        .evidence
        .steps
        .iter()
        .chain(&captured_b.evidence.steps)
        .all(|step| step.naturally_quiescent);

    Ok(ArmEvidence {
        projection: match projection {
            ResearchFocusedActionProjection::Isolated => "isolated",
            ResearchFocusedActionProjection::PalmHorizontal => "palm_horizontal",
        },
        base_return_path_count: base_read.body.return_path_count,
        base_resident_bytes: base_read.body.resident_bytes,
        seed_focused_transitions: seed_observation.body.focused_vision.admitted_transitions,
        paired_initial_body_state_equal,
        paired_initial_device_equal,
        paired_initial_learner_equal,
        semantic_firewall,
        exact_replay_a: replay_a,
        exact_replay_b: replay_b,
        reversed_evaluation_order_equal,
        naturally_quiescent,
        first_focused_origin_candidate,
        divergences,
        cue_a: captured_a.evidence,
        cue_b: captured_b.evidence,
    })
}

fn config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent,
    }
}

fn prepare_branch(
    checkpoint: SessionCheckpoint,
    config: ResearchHarnessConfig,
    visual: ResearchVisualComposition,
    key: u16,
) -> Result<WorkstationSession, WorldError> {
    let mut session = WorkstationSession::restore_research_composed(checkpoint, config, visual)?;
    session.set_presentation(WorkstationPresentation::with_illuminated_key(KeyId(key)))?;
    Ok(session)
}

fn capture_branch(
    session: &mut WorkstationSession,
    key: u16,
    steps: usize,
) -> Result<CapturedBranch, WorldError> {
    let checkpoint = session.save()?;
    let mut observations = Vec::with_capacity(steps);
    for _ in 0..steps {
        observations.push(session.step()?);
    }
    let evidence = BranchEvidence {
        key,
        steps: observations.iter().map(action_step).collect(),
    };
    Ok(CapturedBranch {
        checkpoint,
        observations,
        evidence,
    })
}

fn replay_branch(
    checkpoint: SessionCheckpoint,
    config: ResearchHarnessConfig,
    visual: ResearchVisualComposition,
    expected: &[SessionObservation],
) -> Result<bool, WorldError> {
    let mut replay = WorkstationSession::restore_research_composed(checkpoint, config, visual)?;
    let mut exact = true;
    for observation in expected {
        exact &= replay.step()? == *observation;
    }
    Ok(exact)
}

fn action_step(observation: &SessionObservation) -> ActionStep {
    let palm_candidates = observation
        .body
        .choice_diagnostics
        .iter()
        .filter_map(candidate_witness)
        .collect();
    let choices = observation
        .body
        .choice_diagnostics
        .iter()
        .filter_map(choice_witness)
        .collect();
    ActionStep {
        sequence: observation.sequence,
        focused_changed_features: observation.body.focused_vision.changed_features.clone(),
        focused_admitted_transitions: observation.body.focused_vision.admitted_transitions,
        admitted_inputs: observation.body.admitted_inputs,
        learner_fingerprint: observation.body.learner_fingerprint.clone(),
        body_fingerprint: observation.body.body_fingerprint.clone(),
        palm_candidates,
        choices,
        outputs: observation.body.crossings.clone(),
        palm_movements: observation
            .body
            .movements
            .iter()
            .filter(|movement| movement.axis == BodyAxis::PalmHorizontal)
            .copied()
            .collect(),
        physical_work: observation.body.metrics.physical_work,
        resident_bytes: observation.body.metrics.resident_bytes,
        naturally_quiescent: observation.body.naturally_quiescent,
    }
}

fn candidate_witness(diagnostic: &ResearchChoiceDiagnostic) -> Option<CandidateWitness> {
    let ResearchChoiceDiagnostic::Candidate {
        tick,
        phase,
        control,
        ownership,
        path_inputs,
        path_origins,
        admitted_drive,
        projected_drive,
        threshold,
        executable,
        ..
    } = diagnostic
    else {
        return None;
    };
    if control.axis() != BodyAxis::PalmHorizontal {
        return None;
    }
    Some(CandidateWitness {
        tick: *tick,
        phase: *phase,
        control: *control,
        ownership: ownership.clone(),
        path_inputs: *path_inputs,
        path_origins: path_origins.clone(),
        focused_features: path_origins
            .iter()
            .filter_map(|origin| research_focused_feature_for_origin(*origin))
            .collect(),
        admitted_drive: *admitted_drive,
        projected_drive: *projected_drive,
        threshold: *threshold,
        executable: *executable,
    })
}

fn choice_witness(diagnostic: &ResearchChoiceDiagnostic) -> Option<ChoiceWitness> {
    let ResearchChoiceDiagnostic::Choice {
        tick,
        phase,
        ordinary_control,
        current_transition_control,
        computed_winner_control,
        admitted_controls,
        ..
    } = diagnostic
    else {
        return None;
    };
    let concerns_palm = ordinary_control
        .iter()
        .chain(current_transition_control.iter())
        .chain(computed_winner_control.iter())
        .chain(admitted_controls.iter())
        .any(|control| control.axis() == BodyAxis::PalmHorizontal);
    concerns_palm.then(|| ChoiceWitness {
        tick: *tick,
        phase: *phase,
        ordinary_control: *ordinary_control,
        current_transition_control: *current_transition_control,
        computed_winner_control: *computed_winner_control,
        admitted_controls: admitted_controls.clone(),
    })
}

fn divergences(a: &[ActionStep], b: &[ActionStep]) -> Divergences {
    Divergences {
        focused_receptor: first_difference(a, b, |step| &step.focused_changed_features),
        learner: first_difference(a, b, |step| &step.learner_fingerprint),
        candidate: first_difference(a, b, |step| &step.palm_candidates),
        choice: first_difference(a, b, |step| &step.choices),
        output: first_difference(a, b, |step| &step.outputs),
        palm_movement: first_difference(a, b, |step| &step.palm_movements),
    }
}

fn first_difference<T: PartialEq + ?Sized>(
    a: &[ActionStep],
    b: &[ActionStep],
    project: impl Fn(&ActionStep) -> &T,
) -> Option<u64> {
    a.iter()
        .zip(b)
        .find(|(left, right)| project(left) != project(right))
        .map(|(step, _)| step.sequence)
}

fn sample_has_no_evaluator_semantics(observation: &SessionObservation) -> bool {
    let wire = serde_json::to_string(&observation.sample).expect("sample serializes");
    [
        "key",
        "cursor",
        "click",
        "character",
        "target",
        "expected",
        "score",
        "capability",
        "action",
        "image",
        "coordinate",
        "direction",
        "correctness",
        "reward",
        "evaluator",
    ]
    .into_iter()
    .all(|forbidden| !wire.contains(forbidden))
}
