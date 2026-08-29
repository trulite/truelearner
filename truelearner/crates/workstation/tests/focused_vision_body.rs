#![cfg(feature = "research")]

use truelearner_workstation::{
    ContactSample, LightField, Protocol, ResearchFocusedActionProjection, ResearchHarnessConfig,
    ResearchOpportunityIncidence, ResearchTransitionOpportunity, ResearchVisualComposition,
    WorkstationHarness, WorldSample, FOCUSED_RECEPTOR_FEATURE_COUNT, TOUCH_SITES,
};

fn field(width: u16, height: u16, lit_x: u16, lit_y: u16, value: u8) -> LightField {
    let mut pixels = vec![0; usize::from(width) * usize::from(height)];
    pixels[usize::from(lit_y) * usize::from(width) + usize::from(lit_x)] = value;
    LightField::new(width, height, pixels).unwrap()
}

fn scene(left: (u16, u16), right: (u16, u16), value: u8) -> WorldSample {
    WorldSample::new(
        [
            field(33, 35, left.0, left.1, value),
            field(37, 31, right.0, right.1, value),
        ],
        [ContactSample::default(); TOUCH_SITES],
    )
    .unwrap()
}

fn body() -> WorkstationHarness {
    WorkstationHarness::new_research_composed(
        701,
        ResearchHarnessConfig {
            protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
            opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
            transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
        },
        ResearchVisualComposition::default().with_focused_sensor_field(true),
    )
    .unwrap()
}

#[test]
fn focused_vision_is_an_owned_workstation_organ_and_not_a_second_sparse_view() {
    let blank = scene((0, 0), (0, 0), 0);
    let cue_a = scene((16, 17), (18, 15), 255);
    let cue_b = scene((3, 4), (31, 25), 192);
    let mut body = body();

    let initial = body.step(blank).unwrap();
    assert!(initial.focused_vision.enabled);
    assert_eq!(
        initial.focused_vision.factor_count,
        FOCUSED_RECEPTOR_FEATURE_COUNT
    );
    assert_eq!(initial.focused_vision.admitted_transitions, 0);
    assert_eq!(initial.focused_vision.sparse_retinal_inputs, 0);

    let checkpoint = body.save().unwrap();
    let mut branch_a = WorkstationHarness::restore_research_composed(
        checkpoint.clone(),
        ResearchHarnessConfig {
            protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
            opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
            transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
        },
        ResearchVisualComposition::default().with_focused_sensor_field(true),
    )
    .unwrap();
    let mut branch_b = branch_a.clone();
    let observed_a = branch_a.step(cue_a.clone()).unwrap();
    let observed_b = branch_b.step(cue_b).unwrap();

    assert!(observed_a.focused_vision.admitted_transitions > 0);
    assert!(observed_b.focused_vision.admitted_transitions > 0);
    assert_eq!(observed_a.focused_vision.sparse_retinal_inputs, 0);
    assert_eq!(observed_b.focused_vision.sparse_retinal_inputs, 0);
    assert_ne!(
        observed_a.focused_vision.changed_features,
        observed_b.focused_vision.changed_features
    );
    assert_ne!(
        observed_a.learner_fingerprint,
        observed_b.learner_fingerprint
    );

    let mut replay = WorkstationHarness::restore_research_composed(
        checkpoint,
        ResearchHarnessConfig {
            protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
            opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
            transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
        },
        ResearchVisualComposition::default().with_focused_sensor_field(true),
    )
    .unwrap();
    assert_eq!(replay.step(cue_a).unwrap(), observed_a);
}

#[test]
fn normal_workstation_body_keeps_its_sparse_retina() {
    let mut body = WorkstationHarness::new(702).unwrap();
    let observed = body.step(scene((16, 17), (18, 15), 255)).unwrap();

    assert!(!observed.focused_vision.enabled);
    assert_eq!(observed.focused_vision.admitted_transitions, 0);
    assert!(observed.focused_vision.sparse_retinal_inputs > 0);
}

#[test]
fn focused_mode_rejects_sparse_retinal_effect_composition() {
    let result = WorkstationHarness::new_research_composed(
        703,
        ResearchHarnessConfig {
            protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
            opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
            transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
        },
        ResearchVisualComposition::binocular_alignment().with_focused_sensor_field(true),
    );

    assert!(result.is_err());
}

#[test]
fn focused_action_projection_adds_links_without_adding_visual_inputs() {
    let config = ResearchHarnessConfig {
        protocol:
            Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity:
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent,
    };
    let isolated = ResearchVisualComposition::default().with_focused_sensor_field(true);
    let projected =
        isolated.with_focused_action_projection(ResearchFocusedActionProjection::PalmHorizontal);
    let mut isolated_body =
        WorkstationHarness::new_research_composed(704, config, isolated).unwrap();
    let mut projected_body =
        WorkstationHarness::new_research_composed(704, config, projected).unwrap();
    let blank = scene((0, 0), (0, 0), 0);
    let cue = scene((16, 17), (18, 15), 255);

    let isolated_seed = isolated_body.step(blank.clone()).unwrap();
    let projected_seed = projected_body.step(blank).unwrap();
    assert_eq!(isolated_seed.focused_vision.admitted_transitions, 0);
    assert_eq!(projected_seed.focused_vision.admitted_transitions, 0);
    assert_eq!(
        isolated_seed.admitted_inputs,
        projected_seed.admitted_inputs
    );

    let isolated_cue = isolated_body.step(cue.clone()).unwrap();
    let projected_cue = projected_body.step(cue).unwrap();
    assert_eq!(
        isolated_cue.focused_vision.changed_features,
        projected_cue.focused_vision.changed_features
    );
    assert_eq!(
        isolated_cue.focused_vision.admitted_transitions,
        projected_cue.focused_vision.admitted_transitions
    );
    assert_eq!(isolated_cue.admitted_inputs, projected_cue.admitted_inputs);
}

#[test]
fn focused_action_projection_requires_a_focused_output_specific_palm_body() {
    let projection = ResearchVisualComposition::default()
        .with_focused_action_projection(ResearchFocusedActionProjection::PalmHorizontal);
    let generic = ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
    };
    assert!(WorkstationHarness::new_research_composed(705, generic, projection).is_err());

    let focused = projection.with_focused_sensor_field(true);
    assert!(WorkstationHarness::new_research_composed(705, generic, focused).is_err());
}
