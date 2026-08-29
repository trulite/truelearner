use crate::checkpoint::WorkstationCheckpoint;
use crate::state::{ActuatorFrame, BodyControl, Direction};
use crate::{
    AxisProprioception, BodyAxis, BodyMovement, ContactSample, Digit, Eye, WorkstationError,
    WorkstationState, WorldSample, AXIS_COUNT, BODY_MAX,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_core::{
    Harness, HarnessBuilder, Input, JunctionId, Output, PhysicalIncidence, PhysicalInput, Protocol,
};
#[cfg(feature = "research")]
use truelearner_core::{PhysicalEvent, PhysicalTransition};
#[cfg(any(feature = "research", test))]
use truelearner_embodiment::Quantizer;
use truelearner_embodiment::{
    bounded_magnitude, signed_channels, Availability, CommandCollector, DriveSpec, Driver,
    EffectMode, FocusProfile, JunctionSpec, OpportunityGate, OpposedEffort, Port, SpatialField,
    Wiring,
};

const OUTWARD_REGION: i16 = 1;
const RETINA_FEATURES_PER_EYE: usize = 12;
const RETINA_FEATURES: usize = RETINA_FEATURES_PER_EYE * 2;
#[cfg(feature = "research")]
pub const RESEARCH_RETINA_FEATURE_COUNT: usize = RETINA_FEATURES;
const FOCUSED_REFINEMENT_DEPTH: usize = 7;
const FOCUSED_REGIONS_PER_EYE: usize = 57;
const FOCUSED_BITS_PER_REGION: usize = u32::BITS as usize;
const FOCUSED_FACTORS_PER_EYE: usize = FOCUSED_REGIONS_PER_EYE * FOCUSED_BITS_PER_REGION;
const FOCUSED_RECEPTOR_FEATURES: usize = FOCUSED_FACTORS_PER_EYE * 2;
#[cfg(feature = "research")]
pub const FOCUSED_RECEPTOR_FEATURE_COUNT: usize = FOCUSED_RECEPTOR_FEATURES;
const FOCUSED_RECEPTOR_VALUES: usize = 3;
const EXTERNAL_FEATURE_COUNT: usize = RETINA_FEATURES + crate::TOUCH_SITES;
const RECEPTORS_PER_AXIS: usize = 9;
const FEATURE_COUNT: usize = EXTERNAL_FEATURE_COUNT + AXIS_COUNT * RECEPTORS_PER_AXIS;
const BINS: usize = 4;
const CONTROL_COUNT: usize = AXIS_COUNT * 2;
#[cfg(any(feature = "research", test))]
const RETINAL_QUANTIZER: Quantizer = match Quantizer::new(64, BINS as u16) {
    Ok(quantizer) => quantizer,
    Err(_) => panic!("retinal quantizer constants are valid"),
};
const AXIS_POSITION_BASE: i32 = 10;
const AXIS_POSITION_STRIDE: i32 = 8;
const SENSOR_PHYSICAL_BASE: u64 = 10_000_000;
const CONTROL_PHYSICAL_BASE: u64 = 20_000_000;
const SINK_PHYSICAL_BASE: u64 = 30_000_000;
const OUTCOME_PHYSICAL_BASE: u64 = 40_000_000;
const ANCHOR_PHYSICAL_BASE: u64 = 41_000_000;
const VISUAL_REACH_RELAY_PHYSICAL_BASE: u64 = 42_000_000;
const EXTERNAL_PHYSICAL_BASE: u64 = 50_000_000;
const FOCUSED_SENSOR_PHYSICAL_BASE: u64 = 60_000_000;
const FOCUSED_RELAY_PHYSICAL_BASE: u64 = 70_000_000;
const FOCUSED_SIGNAL_ORIGIN_BASE: u64 = 80_000_000;

#[cfg(feature = "research")]
pub fn research_focused_feature_for_origin(origin_physical: u64) -> Option<usize> {
    let feature = origin_physical.checked_sub(FOCUSED_SIGNAL_ORIGIN_BASE)?;
    let feature = usize::try_from(feature).ok()?;
    (feature < FOCUSED_RECEPTOR_FEATURES).then_some(feature)
}
const RETINA_OFFSETS: [(i16, i16); RETINA_FEATURES_PER_EYE] = [
    (0, 0),
    (8, 0),
    (-8, 0),
    (0, 8),
    (0, -8),
    (24, 24),
    (-24, 24),
    (24, -24),
    (-24, -24),
    (128, 0),
    (-128, 0),
    (0, 128),
];
#[cfg(feature = "research")]
const WIDE_RETINA_OFFSETS: [(i16, i16); RETINA_FEATURES_PER_EYE] = [
    (-384, -128),
    (-128, -128),
    (128, -128),
    (384, -128),
    (-384, 128),
    (-128, 128),
    (128, 128),
    (384, 128),
    (-384, 384),
    (-128, 384),
    (128, 384),
    (384, 384),
];
#[cfg(feature = "research")]
const RETINOTOPIC_RETINA_OFFSETS: [(i16, i16); RETINA_FEATURES_PER_EYE] = [
    (-384, -128),
    (-160, -128),
    (160, -128),
    (384, -128),
    (-384, 128),
    (-160, 128),
    (160, 128),
    (384, 128),
    (-384, 384),
    (-160, 384),
    (160, 384),
    (384, 384),
];
#[cfg(feature = "research")]
const FOVEAL_REACH_RETINA_OFFSETS: [(i16, i16); RETINA_FEATURES_PER_EYE] = [
    (-384, -128),
    (-160, -128),
    (0, 0),
    (160, -128),
    (384, -128),
    (-384, 128),
    (-160, 128),
    (160, 128),
    (384, 128),
    (-160, 384),
    (160, 384),
    (0, 384),
];
#[cfg(feature = "research")]
const BINOCULAR_HORIZONTAL_RETINA_OFFSETS: [(i16, i16); RETINA_FEATURES_PER_EYE] = [
    (0, 0),
    (-16, 0),
    (16, 0),
    (-32, 0),
    (32, 0),
    (-48, 0),
    (48, 0),
    (-64, 0),
    (64, 0),
    (-80, 0),
    (80, 0),
    (0, 32),
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Sites {
    sensors: Vec<[JunctionId; BINS]>,
    motors: Vec<JunctionId>,
    outcomes: Vec<JunctionId>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct RetinalState {
    initialized: bool,
    bins: [u8; RETINA_FEATURES],
    gazes: [crate::Point; 2],
}

impl RetinalState {
    #[cfg(any(feature = "research", test))]
    fn observe(
        &mut self,
        bins: [u8; RETINA_FEATURES],
        gazes: [crate::Point; 2],
    ) -> [bool; RETINA_FEATURES] {
        let changed = if self.initialized {
            std::array::from_fn(|feature| self.bins[feature] != bins[feature])
        } else {
            [false; RETINA_FEATURES]
        };
        self.initialized = true;
        self.bins = bins;
        self.gazes = gazes;
        changed
    }

    #[cfg(feature = "research")]
    fn stable_eyes(
        self,
        sample: &WorldSample,
        retina_offsets: &[(i16, i16); RETINA_FEATURES_PER_EYE],
    ) -> [bool; 2] {
        if !self.initialized {
            return [false; 2];
        }
        let current_at_old_gaze =
            retinal_bins(retinal_features_at(sample, self.gazes, retina_offsets));
        std::array::from_fn(|eye| {
            let start = eye * RETINA_FEATURES_PER_EYE;
            self.bins[start..start + RETINA_FEATURES_PER_EYE]
                == current_at_old_gaze[start..start + RETINA_FEATURES_PER_EYE]
        })
    }

    fn bin(self, feature: usize) -> Option<u8> {
        self.initialized.then_some(self.bins[feature])
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResearchFocusedActionProjection {
    #[default]
    Isolated,
    PalmHorizontal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FocusedVision {
    pub(crate) sensors: Vec<[JunctionId; FOCUSED_RECEPTOR_VALUES]>,
    pub(crate) relays: Vec<JunctionId>,
    pub(crate) previous: Option<Vec<u8>>,
    pub(crate) action_projection: ResearchFocusedActionProjection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FocusedChange {
    feature: usize,
    to: u8,
}

struct FocusedObservation {
    inputs: Vec<PhysicalInput>,
    changes: Vec<FocusedChange>,
    active_regions: [usize; 2],
}

impl FocusedVision {
    fn new(
        sensors: Vec<[JunctionId; FOCUSED_RECEPTOR_VALUES]>,
        relays: Vec<JunctionId>,
        action_projection: ResearchFocusedActionProjection,
    ) -> Self {
        Self {
            sensors,
            relays,
            previous: None,
            action_projection,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), WorkstationError> {
        if self.sensors.len() != FOCUSED_RECEPTOR_FEATURES
            || self.relays.len() != FOCUSED_RECEPTOR_FEATURES
            || self
                .previous
                .as_ref()
                .is_some_and(|previous| previous.len() != FOCUSED_RECEPTOR_FEATURES)
            || self.previous.as_ref().is_some_and(|previous| {
                previous
                    .iter()
                    .any(|value| *value >= FOCUSED_RECEPTOR_VALUES as u8)
            })
        {
            return Err(WorkstationError::InvalidCheckpoint);
        }
        Ok(())
    }

    fn observe(
        &mut self,
        sample: &WorldSample,
        state: &WorkstationState,
        tick: i64,
    ) -> Result<FocusedObservation, WorkstationError> {
        let (current, active_regions) = focused_receptor_values(sample, state)?;
        debug_assert_eq!(current.len(), FOCUSED_RECEPTOR_FEATURES);
        let changes = self.previous.as_ref().map_or_else(Vec::new, |previous| {
            previous
                .iter()
                .copied()
                .zip(current.iter().copied())
                .enumerate()
                .filter_map(|(feature, (from, to))| {
                    (from != to).then_some(FocusedChange { feature, to })
                })
                .collect::<Vec<_>>()
        });
        let inputs = changes
            .iter()
            .map(|change| PhysicalInput {
                input: Input {
                    arrival_tick: tick,
                    phase: i32::try_from(change.feature).unwrap_or(i32::MAX),
                    origin_physical: FOCUSED_SIGNAL_ORIGIN_BASE
                        .saturating_add(u64::try_from(change.feature).unwrap_or(u64::MAX)),
                    target: self.sensors[change.feature][usize::from(change.to)],
                    impulse: 1,
                },
                incidence: PhysicalIncidence::Transition,
            })
            .collect();
        self.previous = Some(current);
        Ok(FocusedObservation {
            inputs,
            changes,
            active_regions,
        })
    }
}

impl Sites {
    pub(crate) fn validate(&self) -> Result<(), WorkstationError> {
        if self.sensors.len() == FEATURE_COUNT
            && self.motors.len() == CONTROL_COUNT
            && matches!(self.outcomes.len(), AXIS_COUNT | CONTROL_COUNT)
        {
            Ok(())
        } else {
            Err(WorkstationError::InvalidCheckpoint)
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepMetrics {
    pub physical_work: u64,
    pub drive_deliveries: u64,
    pub modulatory_deliveries: u64,
    pub plasticity_updates: u64,
    pub structural_proposals: u64,
    pub junction_proposals: u64,
    pub resident_bytes: usize,
}

impl StepMetrics {
    fn add_run(&mut self, run: &truelearner_core::Run) {
        self.physical_work = self.physical_work.saturating_add(run.work.physical_total());
        self.drive_deliveries = self
            .drive_deliveries
            .saturating_add(run.work.drive_deliveries);
        self.modulatory_deliveries = self
            .modulatory_deliveries
            .saturating_add(run.work.modulatory_deliveries);
        self.plasticity_updates = self
            .plasticity_updates
            .saturating_add(run.work.local_return_updates);
        self.structural_proposals = self
            .structural_proposals
            .saturating_add(run.work.local_structural_proposals);
        self.junction_proposals = self
            .junction_proposals
            .saturating_add(run.work.local_junction_proposals);
        self.resident_bytes = self.resident_bytes.max(run.memory_bytes);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationStepObservation {
    pub sequence: u64,
    pub state_before: WorkstationState,
    pub state_after: WorkstationState,
    pub pose_changed: bool,
    pub admitted_inputs: usize,
    pub crossings: Vec<Output>,
    pub movements: Vec<BodyMovement>,
    pub returned_transitions: Vec<BodyAxis>,
    pub pending_transitions: Vec<BodyAxis>,
    pub metrics: StepMetrics,
    pub naturally_quiescent: bool,
    pub body_fingerprint: String,
    #[cfg(feature = "research")]
    pub learner_fingerprint: String,
    pub physical_tick: i64,
    #[cfg(feature = "research")]
    pub choice_diagnostics: Vec<ResearchChoiceDiagnostic>,
    #[cfg(feature = "research")]
    pub retinal_transitions: Vec<ResearchRetinalTransition>,
    #[cfg(feature = "research")]
    pub focused_vision: ResearchFocusedVisionObservation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationRead {
    pub state: WorkstationState,
    pub body_fingerprint: String,
    #[cfg(feature = "research")]
    pub learner_fingerprint: String,
    pub physical_tick: i64,
    pub return_path_count: usize,
    pub resident_bytes: usize,
    pub pending_transitions: Vec<BodyAxis>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkstationHarness {
    boundary: Harness,
    state: WorkstationState,
    sites: Sites,
    sequence: u64,
    pending_transitions: [bool; AXIS_COUNT],
    retinal_state: RetinalState,
    focused_vision: Option<FocusedVision>,
    #[cfg(feature = "research")]
    opportunity_incidence: ResearchOpportunityIncidence,
    #[cfg(feature = "research")]
    transition_opportunity: ResearchTransitionOpportunity,
    #[cfg(feature = "research")]
    visual_composition: ResearchVisualComposition,
}

#[cfg(feature = "research")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResearchFocusedVisionObservation {
    pub enabled: bool,
    pub factor_count: usize,
    pub active_regions: [usize; 2],
    pub admitted_transitions: usize,
    pub changed_features: Vec<usize>,
    pub sparse_retinal_inputs: usize,
}

#[cfg(feature = "research")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResearchOpportunityIncidence {
    Independent,
    SharedWave,
}

#[cfg(feature = "research")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResearchRetinalLayout {
    #[default]
    Inherited,
    BinocularHorizontal,
}

#[cfg(feature = "research")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResearchVisualComposition {
    pub layout: ResearchRetinalLayout,
    pub signed_placement: bool,
    pub movement_caused_return: bool,
    pub threshold_factorization: bool,
    pub foveal_identity_opportunity: bool,
    pub centered_movement_return: bool,
    pub foveal_identity_effect: bool,
    pub focused_sensor_field: bool,
    pub focused_action_projection: ResearchFocusedActionProjection,
}

#[cfg(feature = "research")]
impl Default for ResearchVisualComposition {
    fn default() -> Self {
        Self {
            layout: ResearchRetinalLayout::Inherited,
            signed_placement: false,
            movement_caused_return: false,
            threshold_factorization: false,
            foveal_identity_opportunity: false,
            centered_movement_return: true,
            foveal_identity_effect: false,
            focused_sensor_field: false,
            focused_action_projection: ResearchFocusedActionProjection::Isolated,
        }
    }
}

#[cfg(feature = "research")]
impl ResearchVisualComposition {
    pub const fn binocular_alignment() -> Self {
        Self {
            layout: ResearchRetinalLayout::BinocularHorizontal,
            signed_placement: true,
            movement_caused_return: true,
            threshold_factorization: true,
            foveal_identity_opportunity: false,
            centered_movement_return: true,
            foveal_identity_effect: false,
            focused_sensor_field: false,
            focused_action_projection: ResearchFocusedActionProjection::Isolated,
        }
    }

    pub const fn binocular_stable_fixation() -> Self {
        Self::binocular_alignment()
            .with_threshold_factorization(false)
            .with_foveal_identity_effect(true)
    }

    pub const fn with_signed_placement(mut self, enabled: bool) -> Self {
        self.signed_placement = enabled;
        self
    }

    pub const fn with_movement_caused_return(mut self, enabled: bool) -> Self {
        self.movement_caused_return = enabled;
        self
    }

    pub const fn with_threshold_factorization(mut self, enabled: bool) -> Self {
        self.threshold_factorization = enabled;
        self
    }

    pub const fn with_foveal_identity_opportunity(mut self, enabled: bool) -> Self {
        self.foveal_identity_opportunity = enabled;
        self
    }

    pub const fn with_centered_movement_return(mut self, enabled: bool) -> Self {
        self.centered_movement_return = enabled;
        self
    }

    pub const fn with_foveal_identity_effect(mut self, enabled: bool) -> Self {
        self.foveal_identity_effect = enabled;
        self
    }

    pub const fn with_focused_sensor_field(mut self, enabled: bool) -> Self {
        self.focused_sensor_field = enabled;
        self
    }

    pub const fn with_focused_action_projection(
        mut self,
        projection: ResearchFocusedActionProjection,
    ) -> Self {
        self.focused_action_projection = projection;
        self
    }

    fn sparse_retinal_effects(self) -> bool {
        self.layout != ResearchRetinalLayout::Inherited
            || self.signed_placement
            || self.movement_caused_return
            || self.threshold_factorization
            || self.foveal_identity_opportunity
            || !self.centered_movement_return
            || self.foveal_identity_effect
    }
}

#[cfg(feature = "research")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResearchTransitionOpportunity {
    GenericOnly,
    LocalAfterTransition,
    ComposedWithReturn,
    OutputSpecificComposedReturn,
    OutputSpecificLocalAfterTransition,
    OutputSpecificProprioceptiveReturn,
    OutputSpecificProprioceptiveSequential,
    OutputSpecificProprioceptiveSequentialAligned,
    OutputSpecificProprioceptiveSequentialAlignedTransition,
    OutputSpecificProprioceptiveSequentialAlignedEffect,
    OutputSpecificProprioceptiveSequentialAlignedDelta,
    OutputSpecificProprioceptiveSequentialAlignedCausalDelta,
    OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent,
    OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina,
    OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition,
    OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic,
    OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude,
    OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds,
    OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach,
    OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach,
}

#[cfg(feature = "research")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResearchHarnessConfig {
    pub protocol: Protocol,
    pub opportunity_incidence: ResearchOpportunityIncidence,
    pub transition_opportunity: ResearchTransitionOpportunity,
}

#[cfg(feature = "research")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResearchRetinalTransition {
    pub feature: usize,
    pub axis: BodyAxis,
    pub from_bin: u8,
    pub to_bin: u8,
    pub origin_physical: u64,
}

#[cfg(feature = "research")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResearchRetinalSnapshot {
    pub feature: usize,
    pub axis: BodyAxis,
    pub previous_bin: Option<u8>,
    pub current_bin: u8,
    pub stable_at_previous_gaze: bool,
    pub pending_axis_return: bool,
    pub output_origin: Option<u64>,
}

#[cfg(feature = "research")]
impl ResearchRetinalSnapshot {
    pub fn eligible_transition(self) -> bool {
        self.previous_bin.is_some_and(|bin| bin != self.current_bin)
            && self.stable_at_previous_gaze
            && self.pending_axis_return
            && self.output_origin.is_some()
    }
}

#[cfg(feature = "research")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "diagnostic", rename_all = "snake_case")]
pub enum ResearchChoiceDiagnostic {
    Candidate {
        tick: i64,
        phase: i32,
        control: BodyControl,
        ownership: String,
        path_inputs: u32,
        path_origins: Vec<u64>,
        positive_path_strength: u64,
        negative_path_strength: u64,
        opportunity: i64,
        supplied_opportunity: i64,
        admitted_drive: i64,
        projected_drive: i64,
        threshold: i64,
        consequence_tick: Option<i64>,
        unanswered_returns: u32,
        executable: bool,
    },
    TransitionContinuation {
        tick: i64,
        phase: i32,
        control: BodyControl,
        current_owner_transition: bool,
        unanswered_returns: u32,
        admitted: bool,
    },
    ConsequenceRecorded {
        tick: i64,
        phase: i32,
        link: u64,
        junction: u64,
    },
    ConsequenceConsumed {
        tick: i64,
        phase: i32,
        control: BodyControl,
        link: u64,
        generation: u32,
        consequence_tick: i64,
    },
    CompletedCycle {
        tick: i64,
        phase: i32,
        control: BodyControl,
        consequence_tick: Option<i64>,
        consequence_witnesses: Vec<(u64, u32)>,
        unique_latest_tick: Option<i64>,
        admitted: bool,
    },
    Choice {
        tick: i64,
        phase: i32,
        ordinary_control: Option<BodyControl>,
        current_transition_control: Option<BodyControl>,
        computed_winner_control: Option<BodyControl>,
        admitted_controls: Vec<BodyControl>,
        computed_winner_basis: String,
        admission_basis: String,
    },
}

impl WorkstationHarness {
    pub fn new(_seed: u64) -> Result<Self, WorkstationError> {
        let (boundary, sites, focused_vision) = build_harness(
            Protocol::RecursiveLearnerCausalTopologyProductComposition,
            false,
            false,
            false,
            false,
            &RETINA_OFFSETS,
            false,
            ResearchFocusedActionProjection::Isolated,
        );
        Ok(Self {
            boundary,
            state: WorkstationState::default(),
            sites,
            sequence: 0,
            pending_transitions: [false; AXIS_COUNT],
            retinal_state: RetinalState::default(),
            focused_vision,
            #[cfg(feature = "research")]
            opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
            #[cfg(feature = "research")]
            transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
            #[cfg(feature = "research")]
            visual_composition: ResearchVisualComposition::default(),
        })
    }

    #[cfg(feature = "research")]
    fn new_with(
        protocol: Protocol,
        opportunity_incidence: ResearchOpportunityIncidence,
        transition_opportunity: ResearchTransitionOpportunity,
        visual_composition: ResearchVisualComposition,
    ) -> Result<Self, WorkstationError> {
        if (!visual_composition.focused_sensor_field
            && visual_composition.focused_action_projection
                != ResearchFocusedActionProjection::Isolated)
            || (visual_composition.focused_sensor_field
                && (visual_composition.sparse_retinal_effects()
                    || transition_uses_sparse_retina(transition_opportunity)))
        {
            return Err(WorkstationError::InvalidState);
        }
        let output_specific = matches!(
            transition_opportunity,
            ResearchTransitionOpportunity::OutputSpecificComposedReturn
                | ResearchTransitionOpportunity::OutputSpecificLocalAfterTransition
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveReturn
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequential
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAligned
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedTransition
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedEffect
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedDelta
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDelta
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        );
        let couple_palm_translation = matches!(
            transition_opportunity,
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
        );
        if visual_composition.focused_action_projection
            == ResearchFocusedActionProjection::PalmHorizontal
            && (!output_specific || !couple_palm_translation)
        {
            return Err(WorkstationError::InvalidState);
        }
        let retina_offsets = if visual_composition.layout == ResearchRetinalLayout::BinocularHorizontal {
            &BINOCULAR_HORIZONTAL_RETINA_OFFSETS
        } else if transition_opportunity
            == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        {
            &FOVEAL_REACH_RETINA_OFFSETS
        } else if transition_opportunity
            == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
            || transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
            || transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
            || transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
        {
            &RETINOTOPIC_RETINA_OFFSETS
        } else if transition_opportunity
            == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
            || transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
            || transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
            || transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
            || transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
        {
            &WIDE_RETINA_OFFSETS
        } else {
            &RETINA_OFFSETS
        };
        let (boundary, sites, focused_vision) = build_harness(
            protocol,
            output_specific,
            couple_palm_translation,
            visual_composition.signed_placement
                || matches!(
                transition_opportunity,
                ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                    | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                    | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                    | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                    | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
                ),
            matches!(
                transition_opportunity,
                ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                    | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
            ),
            retina_offsets,
            visual_composition.focused_sensor_field,
            visual_composition.focused_action_projection,
        );
        Ok(Self {
            boundary,
            state: WorkstationState::default(),
            sites,
            sequence: 0,
            pending_transitions: [false; AXIS_COUNT],
            retinal_state: RetinalState::default(),
            focused_vision,
            opportunity_incidence,
            transition_opportunity,
            visual_composition,
        })
    }

    #[cfg(feature = "research")]
    pub fn new_research(
        _seed: u64,
        config: ResearchHarnessConfig,
    ) -> Result<Self, WorkstationError> {
        Self::new_with(
            config.protocol,
            config.opportunity_incidence,
            config.transition_opportunity,
            ResearchVisualComposition::default(),
        )
    }

    #[cfg(feature = "research")]
    pub fn new_research_composed(
        _seed: u64,
        config: ResearchHarnessConfig,
        visual_composition: ResearchVisualComposition,
    ) -> Result<Self, WorkstationError> {
        Self::new_with(
            config.protocol,
            config.opportunity_incidence,
            config.transition_opportunity,
            visual_composition,
        )
    }

    pub fn step(
        &mut self,
        sample: WorldSample,
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        sample.validate()?;
        let mut next = self.clone();
        let state_before = next.state.clone();
        let returned_transitions = next.pending_axes();
        let mut metrics = StepMetrics::default();
        let mut naturally_quiescent = true;
        let mut crossings = Vec::new();
        let mut movements = Vec::new();
        let mut current_movements = Vec::new();
        let mut admitted_inputs = 0;
        #[cfg(feature = "research")]
        let mut choice_diagnostics = Vec::new();
        #[cfg(feature = "research")]
        let mut retinal_transitions = Vec::new();
        let focused_enabled = next.focused_vision.is_some();
        #[cfg(feature = "research")]
        let pre_return_retina = if next.visual_composition.movement_caused_return {
            let before = next.retinal_state;
            let gazes = Eye::ALL.map(|eye| next.state.eye(eye).gaze());
            let values = retinal_features_at(&sample, gazes, next.retina_offsets());
            let bins = retinal_bins(values);
            let stable_eyes = before.stable_eyes(&sample, next.retina_offsets());
            let output_origins = retinal_output_origins(before, gazes, next.retina_offsets());
            let deltas = next.retinal_state.observe(bins, gazes);
            Some((before, values, stable_eyes, deltas, output_origins))
        } else {
            None
        };
        #[cfg(feature = "research")]
        let foveal_active = pre_return_retina.map_or([false; 2], |(_, values, _, _, _)| {
            Self::foveal_active_eyes(&values)
        });
        #[cfg(feature = "research")]
        let foveal_effect_identity = if next.visual_composition.foveal_identity_effect {
            foveal_active
        } else {
            [false; 2]
        };
        #[cfg(not(feature = "research"))]
        let foveal_effect_identity = [false; 2];

        if !returned_transitions.is_empty() {
            let tick = next.boundary.read().clock.tick.saturating_add(1);
            let returns = if next.returns_through_proprioception() {
                let values = sensory_features(&sample, &next.state, next.retina_offsets());
                let proprioception = next.state.proprioception();
                let mut returns = Vec::new();
                for (order, axis) in returned_transitions.iter().enumerate() {
                    let first = EXTERNAL_FEATURE_COUNT + axis.index() * RECEPTORS_PER_AXIS;
                    let delta = proprioceptor_delta_for(proprioception[axis.index()]);
                    let phase = 30_000_i32.saturating_add(i32::try_from(order).unwrap_or(0));
                    for (offset, value) in values[first..first + RECEPTORS_PER_AXIS]
                        .iter()
                        .enumerate()
                        .filter(|(_, value)| **value > 0)
                        .filter(|(offset, _)| {
                            !next.effect_receptor_transitions() || effect_receptor_offset(*offset)
                        })
                        .filter(|(offset, _)| !next.delta_receptor_transitions() || delta[*offset])
                    {
                        let feature = first + offset;
                        let bin = usize::from(*value / 64).min(BINS - 1);
                        let origin_physical = if next.causal_delta_receptor_transitions() {
                            let Some(origin) = next.return_outcome_origin(*axis) else {
                                continue;
                            };
                            origin
                        } else {
                            SENSOR_PHYSICAL_BASE.saturating_add(
                                u64::try_from(feature.saturating_mul(BINS).saturating_add(bin))
                                    .unwrap_or(0),
                            )
                        };
                        returns.push(PhysicalInput {
                            input: Input {
                                arrival_tick: tick,
                                phase,
                                origin_physical,
                                target: next.sites.sensors[feature][bin],
                                impulse: 1,
                            },
                            incidence: PhysicalIncidence::Transition,
                        });
                    }
                }
                returns
            } else {
                returned_transitions
                    .iter()
                    .enumerate()
                    .filter_map(|(order, axis)| {
                        Some(PhysicalInput {
                            input: Input {
                                arrival_tick: tick,
                                phase: 30_000_i32.saturating_add(i32::try_from(order).unwrap_or(0)),
                                origin_physical: transition_origin(next.sequence, *axis),
                                target: next.return_target(*axis)?,
                                impulse: 1,
                            },
                            incidence: PhysicalIncidence::Transition,
                        })
                    })
                    .collect::<Vec<_>>()
            };
            #[cfg(feature = "research")]
            let returns = if matches!(
                next.transition_opportunity,
                ResearchTransitionOpportunity::ComposedWithReturn
                    | ResearchTransitionOpportunity::OutputSpecificComposedReturn
            ) {
                let mut returns = returns;
                let opportunity_tick = tick.saturating_add(1);
                for (order, axis) in returned_transitions.iter().enumerate() {
                    let first_motor = axis.index() * 2;
                    let phase = 30_000_i32.saturating_add(i32::try_from(order).unwrap_or(0));
                    let origin_physical = transition_opportunity_origin(next.sequence, *axis);
                    for target in &next.sites.motors[first_motor..first_motor + 2] {
                        returns.push(PhysicalInput {
                            input: Input {
                                arrival_tick: opportunity_tick,
                                phase,
                                origin_physical,
                                target: *target,
                                impulse: 1,
                            },
                            incidence: PhysicalIncidence::Sample,
                        });
                    }
                }
                returns
            } else {
                returns
            };
            admitted_inputs += returns.len();
            let returned = next.boundary.send_physical(&returns);
            metrics.add_run(&returned);
            naturally_quiescent &= returned.naturally_quiescent;
            #[cfg(feature = "research")]
            choice_diagnostics.extend(project_choice_diagnostics(
                &returned.physical_trace,
                &next.sites,
            ));
            if next.sequential_effect_composition() {
                movements.extend(
                    next.integrate_output_moments(&returned.outputs, foveal_effect_identity)?,
                );
            }
            crossings.extend(returned.outputs);
        }

        let features = sensory_features(&sample, &next.state, next.retina_offsets());
        #[cfg(feature = "research")]
        let features = {
            let mut features = features;
            if let Some((_, values, _, _, _)) = pre_return_retina {
                features[..RETINA_FEATURES].copy_from_slice(&values);
            }
            features
        };
        #[cfg(feature = "research")]
        let (retinal_before, stable_eyes, retinal_deltas, retinal_outcome_origins) =
            if let Some((before, _, stable, deltas, origins)) = pre_return_retina {
                (before, stable, deltas, origins)
            } else {
                let before = next.retinal_state;
                let gazes = Eye::ALL.map(|eye| next.state.eye(eye).gaze());
                let bins = retinal_bins(
                    features[..RETINA_FEATURES]
                        .try_into()
                        .expect("retinal feature count is fixed"),
                );
                let stable = if next.visual_receptor_transitions() {
                    before.stable_eyes(&sample, next.retina_offsets())
                } else {
                    [false; 2]
                };
                let deltas = if next.visual_receptor_transitions() {
                    next.retinal_state.observe(bins, gazes)
                } else {
                    [false; RETINA_FEATURES]
                };
                (before, stable, deltas, [None; RETINA_FEATURES])
            };
        #[cfg(not(feature = "research"))]
        let retinal_before = next.retinal_state;
        #[cfg(not(feature = "research"))]
        let stable_eyes = [false; 2];
        #[cfg(not(feature = "research"))]
        let retinal_deltas = [false; RETINA_FEATURES];
        #[cfg(not(feature = "research"))]
        let retinal_outcome_origins = [None; RETINA_FEATURES];
        let tick = next.boundary.read().clock.tick.saturating_add(1);
        let focused_observation = if let Some(focused_vision) = next.focused_vision.as_mut() {
            focused_vision.observe(&sample, &next.state, tick)?
        } else {
            FocusedObservation {
                inputs: Vec::new(),
                changes: Vec::new(),
                active_regions: [0; 2],
            }
        };
        #[cfg(not(feature = "research"))]
        let _ = (
            &focused_observation.changes,
            focused_observation.active_regions,
        );
        let transitioned_axes: [bool; AXIS_COUNT] = std::array::from_fn(|index| {
            movements
                .iter()
                .any(|movement| movement.axis.index() == index && movement.changed)
        });
        let proprioceptor_deltas = next.state.proprioception().map(proprioceptor_delta_for);
        let mut crossed_retinal_thresholds = Vec::new();
        let mut inputs = features
            .iter()
            .enumerate()
            .filter(|(feature, _)| !focused_enabled || *feature >= RETINA_FEATURES)
            .filter(|(_, value)| **value > 0)
            .map(|(feature, value)| {
                let bin = usize::from(*value / 64).min(BINS - 1);
                let axis = receptor_axis(feature, next.retina_offsets());
                let outcome_origin = retinal_outcome_origins
                    .get(feature)
                    .copied()
                    .flatten()
                    .or_else(|| next.return_outcome_origin(axis));
                let proprioceptor_transition = next.carries_intermediate_transition()
                    && feature >= EXTERNAL_FEATURE_COUNT
                    && transitioned_axes[axis.index()]
                    && (!next.effect_receptor_transitions()
                        || effect_receptor_offset(
                            (feature - EXTERNAL_FEATURE_COUNT) % RECEPTORS_PER_AXIS,
                        ))
                    && (!next.delta_receptor_transitions()
                        || proprioceptor_deltas[axis.index()]
                            [(feature - EXTERNAL_FEATURE_COUNT) % RECEPTORS_PER_AXIS])
                    && (!next.causal_delta_receptor_transitions() || outcome_origin.is_some());
                let retinal_transition = next.visual_receptor_transitions()
                    && feature < RETINA_FEATURES
                    && next.returns_centered_movement(feature)
                    && retinal_deltas[feature]
                    && stable_eyes[feature / RETINA_FEATURES_PER_EYE]
                    && returned_transitions.contains(&axis)
                    && outcome_origin.is_some();
                let transitioned = proprioceptor_transition || retinal_transition;
                if retinal_transition && next.factorizes_retinal_thresholds() {
                    let from_bin = retinal_before
                        .bin(feature)
                        .expect("retinal transition has retained state");
                    let to_bin = u8::try_from(bin).unwrap_or(u8::MAX);
                    let crossed = if from_bin < to_bin {
                        (from_bin.saturating_add(1)..to_bin).collect::<Vec<_>>()
                    } else {
                        (to_bin.saturating_add(1)..from_bin)
                            .rev()
                            .collect::<Vec<_>>()
                    };
                    for crossed_bin in crossed {
                        crossed_retinal_thresholds.push(PhysicalInput {
                            input: Input {
                                arrival_tick: tick,
                                phase: i32::try_from(feature).unwrap_or(0),
                                origin_physical: EXTERNAL_PHYSICAL_BASE
                                    .saturating_add(next.sequence.saturating_mul(10_000))
                                    .saturating_add(u64::try_from(feature).unwrap_or(0)),
                                target: next.sites.sensors[feature][usize::from(crossed_bin)],
                                impulse: 1,
                            },
                            incidence: PhysicalIncidence::Sample,
                        });
                    }
                }
                #[cfg(feature = "research")]
                if retinal_transition {
                    retinal_transitions.push(ResearchRetinalTransition {
                        feature,
                        axis,
                        from_bin: retinal_before
                            .bin(feature)
                            .expect("stable retained retina is initialized"),
                        to_bin: u8::try_from(bin).unwrap_or(u8::MAX),
                        origin_physical: outcome_origin
                            .expect("retinal transition has an exact output outcome"),
                    });
                }
                let stable_proprioceptor =
                    next.delta_receptor_transitions() && feature >= EXTERNAL_FEATURE_COUNT;
                let impulse = if retinal_transition && next.preserves_retinal_transition_magnitude()
                {
                    i32::from(
                        retinal_before
                            .bin(feature)
                            .expect("retinal transition has retained state")
                            .abs_diff(u8::try_from(bin).unwrap_or(u8::MAX)),
                    )
                    .max(1)
                } else {
                    1
                };
                let receptor_origin = SENSOR_PHYSICAL_BASE.saturating_add(
                    u64::try_from(feature.saturating_mul(BINS).saturating_add(bin)).unwrap_or(0),
                );
                PhysicalInput {
                    input: Input {
                        arrival_tick: tick,
                        phase: i32::try_from(feature).unwrap_or(0),
                        origin_physical: if transitioned && next.causal_delta_receptor_transitions()
                        {
                            outcome_origin.unwrap_or(receptor_origin)
                        } else if transitioned || stable_proprioceptor {
                            receptor_origin
                        } else {
                            EXTERNAL_PHYSICAL_BASE
                                .saturating_add(next.sequence.saturating_mul(10_000))
                                .saturating_add(u64::try_from(feature).unwrap_or(0))
                        },
                        target: next.sites.sensors[feature][bin],
                        impulse,
                    },
                    incidence: if transitioned {
                        PhysicalIncidence::Transition
                    } else {
                        PhysicalIncidence::Sample
                    },
                }
            })
            .collect::<Vec<_>>();
        #[cfg(feature = "research")]
        let sparse_retinal_inputs = if focused_enabled {
            0
        } else {
            features[..RETINA_FEATURES]
                .iter()
                .filter(|value| **value > 0)
                .count()
        };
        inputs.append(&mut crossed_retinal_thresholds);
        inputs.extend(focused_observation.inputs);
        let opportunity_tick = tick.saturating_add(next.opportunity_delay());
        #[cfg(feature = "research")]
        let foveal_identity = if next.visual_composition.foveal_identity_opportunity {
            foveal_active
        } else {
            [false; 2]
        };
        let mut opportunity_gate = OpportunityGate;
        inputs.extend(
            next.sites
                .motors
                .iter()
                .enumerate()
                .filter_map(|(index, target)| {
                    #[cfg(feature = "research")]
                    let control = control(index);
                    #[cfg(feature = "research")]
                    let open = match control.axis() {
                        BodyAxis::EyeHorizontal { eye } => !foveal_identity[eye.index()],
                        _ => true,
                    };
                    #[cfg(not(feature = "research"))]
                    let open = true;
                    let (phase, origin_offset) = next.opportunity_coordinates(index);
                    opportunity_gate.step((
                        open,
                        PhysicalInput {
                            input: Input {
                                arrival_tick: opportunity_tick,
                                phase,
                                origin_physical: EXTERNAL_PHYSICAL_BASE
                                    .saturating_add(next.sequence.saturating_mul(10_000))
                                    .saturating_add(origin_offset),
                                target: *target,
                                impulse: 1,
                            },
                            incidence: PhysicalIncidence::Sample,
                        },
                    ))
                }),
        );
        #[cfg(feature = "research")]
        if matches!(
            next.transition_opportunity,
            ResearchTransitionOpportunity::LocalAfterTransition
                | ResearchTransitionOpportunity::OutputSpecificLocalAfterTransition
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveReturn
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequential
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAligned
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedTransition
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedEffect
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedDelta
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDelta
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        ) {
            for (order, axis) in returned_transitions.iter().enumerate() {
                let first_motor = axis.index() * 2;
                let phase = 30_000_i32.saturating_add(i32::try_from(order).unwrap_or(0));
                let origin_physical = transition_origin(next.sequence, *axis);
                for target in &next.sites.motors[first_motor..first_motor + 2] {
                    inputs.push(PhysicalInput {
                        input: Input {
                            arrival_tick: opportunity_tick,
                            phase,
                            origin_physical,
                            target: *target,
                            impulse: 1,
                        },
                        incidence: PhysicalIncidence::Sample,
                    });
                }
            }
        }
        admitted_inputs += inputs.len();
        let run = if next.carries_intermediate_transition() {
            next.boundary.send_physical(&inputs)
        } else {
            let ordinary = inputs.iter().map(|input| input.input).collect::<Vec<_>>();
            next.boundary.send(&ordinary)
        };
        metrics.add_run(&run);
        naturally_quiescent &= run.naturally_quiescent;
        #[cfg(feature = "research")]
        choice_diagnostics.extend(project_choice_diagnostics(&run.physical_trace, &next.sites));
        if next.sequential_effect_composition() {
            current_movements =
                next.integrate_output_moments(&run.outputs, foveal_effect_identity)?;
            movements.extend(current_movements.iter().copied());
        }
        crossings.extend(run.outputs);

        if !next.sequential_effect_composition() {
            movements = next.integrate_outputs(&crossings, foveal_effect_identity)?;
        }
        let pose_changed = !next.state.same_pose(&state_before);
        next.pending_transitions = [false; AXIS_COUNT];
        let pending_movements = if next.sequential_effect_composition() {
            &current_movements
        } else {
            &movements
        };
        for movement in pending_movements {
            if movement.changed {
                next.pending_transitions[movement.axis.index()] = true;
            }
        }
        next.sequence = next.sequence.saturating_add(1);
        let observation = WorkstationStepObservation {
            sequence: self.sequence,
            state_before,
            state_after: next.state.clone(),
            pose_changed,
            admitted_inputs,
            crossings,
            movements,
            returned_transitions,
            pending_transitions: next.pending_axes(),
            metrics,
            naturally_quiescent,
            body_fingerprint: next.fingerprint()?,
            #[cfg(feature = "research")]
            learner_fingerprint: next.learner_fingerprint()?,
            physical_tick: next.boundary.read().clock.tick,
            #[cfg(feature = "research")]
            choice_diagnostics,
            #[cfg(feature = "research")]
            retinal_transitions,
            #[cfg(feature = "research")]
            focused_vision: ResearchFocusedVisionObservation {
                enabled: focused_enabled,
                factor_count: usize::from(focused_enabled) * FOCUSED_RECEPTOR_FEATURES,
                active_regions: focused_observation.active_regions,
                admitted_transitions: focused_observation.changes.len(),
                changed_features: focused_observation
                    .changes
                    .into_iter()
                    .map(|change| change.feature)
                    .collect(),
                sparse_retinal_inputs,
            },
        };
        *self = next;
        Ok(observation)
    }

    pub fn read(&self) -> Result<WorkstationRead, WorkstationError> {
        let observation = self.boundary.read();
        Ok(WorkstationRead {
            state: self.state.clone(),
            body_fingerprint: self.fingerprint()?,
            #[cfg(feature = "research")]
            learner_fingerprint: self.learner_fingerprint()?,
            physical_tick: observation.clock.tick,
            return_path_count: observation.return_path_count,
            resident_bytes: observation.resident_bytes,
            pending_transitions: self.pending_axes(),
        })
    }

    #[cfg(feature = "research")]
    pub fn research_retinal_snapshot(&self, sample: &WorldSample) -> Vec<ResearchRetinalSnapshot> {
        let gazes = Eye::ALL.map(|eye| self.state.eye(eye).gaze());
        let bins = retinal_bins(retinal_features_at(sample, gazes, self.retina_offsets()));
        let stable_eyes = self
            .retinal_state
            .stable_eyes(sample, self.retina_offsets());
        let output_origins =
            retinal_output_origins(self.retinal_state, gazes, self.retina_offsets());
        (0..RETINA_FEATURES)
            .map(|feature| {
                let axis = receptor_axis(feature, self.retina_offsets());
                ResearchRetinalSnapshot {
                    feature,
                    axis,
                    previous_bin: self.retinal_state.bin(feature),
                    current_bin: bins[feature],
                    stable_at_previous_gaze: stable_eyes[feature / RETINA_FEATURES_PER_EYE],
                    pending_axis_return: self.pending_transitions[axis.index()],
                    output_origin: output_origins[feature],
                }
            })
            .collect()
    }

    pub fn save(&self) -> Result<WorkstationCheckpoint, WorkstationError> {
        let core = self
            .boundary
            .save()
            .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?
            .canonical_bytes()
            .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
        Ok(WorkstationCheckpoint::new(
            core,
            self.state.clone(),
            self.sites.clone(),
            self.sequence,
            self.pending_transitions,
            self.retinal_state,
            self.focused_vision.clone(),
        ))
    }

    pub fn restore(checkpoint: WorkstationCheckpoint) -> Result<Self, WorkstationError> {
        let payload = checkpoint.open()?;
        let core = truelearner_core::Checkpoint::decode(&payload.core)
            .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
        let boundary = Harness::restore(core)
            .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
        for sensors in &payload.sites.sensors {
            for sensor in sensors {
                if boundary.read().junction(*sensor).is_none() {
                    return Err(WorkstationError::InvalidCheckpoint);
                }
            }
        }
        for site in payload.sites.motors.iter().chain(&payload.sites.outcomes) {
            if boundary.read().junction(*site).is_none() {
                return Err(WorkstationError::InvalidCheckpoint);
            }
        }
        if let Some(focused_vision) = &payload.focused_vision {
            focused_vision.validate()?;
            for site in focused_vision
                .sensors
                .iter()
                .flatten()
                .chain(&focused_vision.relays)
            {
                if boundary.read().junction(*site).is_none() {
                    return Err(WorkstationError::InvalidCheckpoint);
                }
            }
        }
        Ok(Self {
            boundary,
            state: payload.state,
            sites: payload.sites,
            sequence: payload.sequence,
            pending_transitions: payload.pending_transitions,
            retinal_state: payload.retinal_state,
            focused_vision: payload.focused_vision,
            #[cfg(feature = "research")]
            opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
            #[cfg(feature = "research")]
            transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
            #[cfg(feature = "research")]
            visual_composition: ResearchVisualComposition::default(),
        })
    }

    #[cfg(feature = "research")]
    pub fn restore_research(
        checkpoint: WorkstationCheckpoint,
        opportunity_incidence: ResearchOpportunityIncidence,
    ) -> Result<Self, WorkstationError> {
        let mut restored = Self::restore(checkpoint)?;
        if restored.focused_vision.is_some() {
            return Err(WorkstationError::InvalidState);
        }
        restored.opportunity_incidence = opportunity_incidence;
        Ok(restored)
    }

    #[cfg(feature = "research")]
    pub fn restore_research_config(
        checkpoint: WorkstationCheckpoint,
        config: ResearchHarnessConfig,
    ) -> Result<Self, WorkstationError> {
        let mut restored = Self::restore(checkpoint)?;
        if restored.focused_vision.is_some() {
            return Err(WorkstationError::InvalidState);
        }
        restored.opportunity_incidence = config.opportunity_incidence;
        restored.transition_opportunity = config.transition_opportunity;
        restored.visual_composition = ResearchVisualComposition::default();
        Ok(restored)
    }

    #[cfg(feature = "research")]
    pub fn restore_research_composed(
        checkpoint: WorkstationCheckpoint,
        config: ResearchHarnessConfig,
        visual_composition: ResearchVisualComposition,
    ) -> Result<Self, WorkstationError> {
        let mut restored = Self::restore(checkpoint)?;
        if restored.focused_vision.is_some() != visual_composition.focused_sensor_field
            || (visual_composition.focused_sensor_field
                && (visual_composition.sparse_retinal_effects()
                    || transition_uses_sparse_retina(config.transition_opportunity)))
            || restored.focused_vision.as_ref().is_some_and(|vision| {
                vision.action_projection != visual_composition.focused_action_projection
            })
        {
            return Err(WorkstationError::InvalidState);
        }
        restored.opportunity_incidence = config.opportunity_incidence;
        restored.transition_opportunity = config.transition_opportunity;
        restored.visual_composition = visual_composition;
        Ok(restored)
    }

    fn opportunity_coordinates(&self, _index: usize) -> (i32, u64) {
        #[cfg(feature = "research")]
        if self.aligned_opportunity() {
            return (0, 5_000);
        }
        #[cfg(feature = "research")]
        if self.opportunity_incidence == ResearchOpportunityIncidence::Independent {
            return (
                20_000_i32.saturating_add(i32::try_from(_index).unwrap_or(0)),
                5_000_u64.saturating_add(u64::try_from(_index).unwrap_or(0)),
            );
        }

        (20_000, 5_000)
    }

    fn opportunity_delay(&self) -> i64 {
        #[cfg(feature = "research")]
        if self.aligned_opportunity() {
            return 2;
        }
        1
    }

    #[cfg(feature = "research")]
    fn aligned_opportunity(&self) -> bool {
        matches!(
            self.transition_opportunity,
            ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAligned
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedTransition
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedEffect
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedDelta
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDelta
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        )
    }

    fn retina_offsets(&self) -> &'static [(i16, i16); RETINA_FEATURES_PER_EYE] {
        #[cfg(feature = "research")]
        if self.visual_composition.layout == ResearchRetinalLayout::BinocularHorizontal {
            return &BINOCULAR_HORIZONTAL_RETINA_OFFSETS;
        } else if self.transition_opportunity
            == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        {
            return &FOVEAL_REACH_RETINA_OFFSETS;
        } else if self.transition_opportunity
            == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
            || self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
            || self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
            || self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
        {
            return &RETINOTOPIC_RETINA_OFFSETS;
        } else if self.transition_opportunity
            == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
            || self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
        {
            return &WIDE_RETINA_OFFSETS;
        }
        &RETINA_OFFSETS
    }

    fn visual_receptor_transitions(&self) -> bool {
        #[cfg(feature = "research")]
        {
            self.visual_composition.movement_caused_return
                || self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        }
        #[cfg(not(feature = "research"))]
        {
            false
        }
    }

    fn preserves_retinal_transition_magnitude(&self) -> bool {
        #[cfg(feature = "research")]
        {
            self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
        }
        #[cfg(not(feature = "research"))]
        {
            false
        }
    }

    fn factorizes_retinal_thresholds(&self) -> bool {
        #[cfg(feature = "research")]
        {
            self.visual_composition.threshold_factorization
                || self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        }
        #[cfg(not(feature = "research"))]
        {
            false
        }
    }

    fn returns_centered_movement(&self, feature: usize) -> bool {
        #[cfg(feature = "research")]
        {
            self.visual_composition.centered_movement_return
                || !feature.is_multiple_of(RETINA_FEATURES_PER_EYE)
        }
        #[cfg(not(feature = "research"))]
        {
            let _ = feature;
            true
        }
    }

    fn pending_axes(&self) -> Vec<BodyAxis> {
        BodyAxis::ALL
            .into_iter()
            .filter(|axis| self.pending_transitions[axis.index()])
            .collect()
    }

    #[cfg(feature = "research")]
    fn foveal_active_eyes(features: &[u8; RETINA_FEATURES]) -> [bool; 2] {
        Eye::ALL.map(|eye| {
            let value = features[eye.index() * RETINA_FEATURES_PER_EYE];
            usize::from(RETINAL_QUANTIZER.bin(u16::from(value))) == BINS - 1
        })
    }

    fn return_target(&self, axis: BodyAxis) -> Option<JunctionId> {
        if self.sites.outcomes.len() == AXIS_COUNT {
            return self.sites.outcomes.get(axis.index()).copied();
        }
        let effort = self.state.proprioception()[axis.index()];
        let direction = match (effort.decrease_effort > 0, effort.increase_effort > 0) {
            (true, false) => Direction::Decrease,
            (false, true) => Direction::Increase,
            _ => return None,
        };
        self.sites
            .outcomes
            .get(axis.index() * 2 + usize::from(direction == Direction::Increase))
            .copied()
    }

    fn returns_through_proprioception(&self) -> bool {
        #[cfg(feature = "research")]
        {
            self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveReturn
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequential
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAligned
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedTransition
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedEffect
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedDelta
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDelta
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        }
        #[cfg(not(feature = "research"))]
        {
            false
        }
    }

    fn sequential_effect_composition(&self) -> bool {
        #[cfg(feature = "research")]
        {
            self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequential
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAligned
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedTransition
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedEffect
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedDelta
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDelta
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        }
        #[cfg(not(feature = "research"))]
        {
            false
        }
    }

    fn carries_intermediate_transition(&self) -> bool {
        #[cfg(feature = "research")]
        {
            self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedTransition
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedEffect
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedDelta
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDelta
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        }
        #[cfg(not(feature = "research"))]
        {
            false
        }
    }

    fn effect_receptor_transitions(&self) -> bool {
        #[cfg(feature = "research")]
        {
            self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedEffect
        }
        #[cfg(not(feature = "research"))]
        {
            false
        }
    }

    fn delta_receptor_transitions(&self) -> bool {
        #[cfg(feature = "research")]
        {
            self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedDelta
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDelta
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        }
        #[cfg(not(feature = "research"))]
        {
            false
        }
    }

    fn causal_delta_receptor_transitions(&self) -> bool {
        #[cfg(feature = "research")]
        {
            self.transition_opportunity
                == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDelta
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponent
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
                || self.transition_opportunity
                    == ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
        }
        #[cfg(not(feature = "research"))]
        {
            false
        }
    }

    fn return_outcome_origin(&self, axis: BodyAxis) -> Option<u64> {
        if self.sites.outcomes.len() != CONTROL_COUNT {
            return None;
        }
        let effort = self.state.proprioception()[axis.index()];
        let direction = match (effort.decrease_effort > 0, effort.increase_effort > 0) {
            (true, false) => Direction::Decrease,
            (false, true) => Direction::Increase,
            _ => return None,
        };
        Some(
            OUTCOME_PHYSICAL_BASE
                .saturating_add(u64::try_from(axis.index() * 2).unwrap_or(0))
                .saturating_add(u64::from(direction == Direction::Increase)),
        )
    }

    fn integrate_outputs(
        &mut self,
        outputs: &[Output],
        foveal_identity: [bool; 2],
    ) -> Result<Vec<BodyMovement>, WorkstationError> {
        let mut commands = CommandCollector::<OpposedEffort, AXIS_COUNT>::new();
        for crossing in outputs {
            let index = crossing
                .from_physical
                .checked_sub(CONTROL_PHYSICAL_BASE)
                .and_then(|value| usize::try_from(value).ok())
                .filter(|index| *index < CONTROL_COUNT)
                .ok_or(WorkstationError::UnknownOutput(crossing.from_physical))?;
            let control = control(index);
            let impulse = u16::try_from(crossing.impulse.unsigned_abs())
                .unwrap_or(u16::MAX)
                .min(BODY_MAX as u16);
            let command = match control.direction() {
                Direction::Decrease => OpposedEffort::new(impulse, 0),
                Direction::Increase => OpposedEffort::new(0, impulse),
            };
            commands
                .add(
                    Port(u32::try_from(control.axis().index()).unwrap_or(0)),
                    command,
                    |left, right| left.combine_bounded(right, BODY_MAX as u16),
                )
                .map_err(|_| WorkstationError::UnknownOutput(crossing.from_physical))?;
        }

        let mut commands = commands.finish();
        for eye in Eye::ALL {
            let axis = BodyAxis::EyeHorizontal { eye };
            let mode = if foveal_identity[eye.index()] {
                EffectMode::Identity
            } else {
                EffectMode::Apply
            };
            commands
                .constrain(Port(u32::try_from(axis.index()).unwrap_or(0)), mode)
                .map_err(|_| WorkstationError::InvalidState)?;
        }
        let frame = ActuatorFrame {
            axes: commands.into_commands().map(Option::unwrap_or_default),
        };
        Ok(self.state.integrate(frame))
    }

    fn integrate_output_moments(
        &mut self,
        outputs: &[Output],
        foveal_identity: [bool; 2],
    ) -> Result<Vec<BodyMovement>, WorkstationError> {
        debug_assert!(outputs.windows(2).all(|pair| pair[0].tick <= pair[1].tick));
        let mut movements = Vec::new();
        let mut first = 0;
        while first < outputs.len() {
            let tick = outputs[first].tick;
            let end = outputs[first..]
                .iter()
                .position(|output| output.tick != tick)
                .map_or(outputs.len(), |offset| first + offset);
            movements.extend(self.integrate_outputs(&outputs[first..end], foveal_identity)?);
            first = end;
        }
        Ok(movements)
    }

    fn fingerprint(&self) -> Result<String, WorkstationError> {
        let digest = Sha256::digest(self.save()?.canonical_bytes()?);
        Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
    }

    #[cfg(feature = "research")]
    fn learner_fingerprint(&self) -> Result<String, WorkstationError> {
        let checkpoint = self
            .boundary
            .save()
            .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
        let digest = Sha256::digest(
            checkpoint
                .canonical_bytes()
                .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?,
        );
        Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
    }
}

fn transition_origin(sequence: u64, axis: BodyAxis) -> u64 {
    EXTERNAL_PHYSICAL_BASE
        .saturating_add(sequence.saturating_mul(10_000))
        .saturating_add(9_000)
        .saturating_add(u64::try_from(axis.index()).unwrap_or(0))
}

#[cfg(feature = "research")]
fn transition_opportunity_origin(sequence: u64, axis: BodyAxis) -> u64 {
    EXTERNAL_PHYSICAL_BASE
        .saturating_add(sequence.saturating_mul(10_000))
        .saturating_add(9_500)
        .saturating_add(u64::try_from(axis.index()).unwrap_or(0))
}

#[cfg(feature = "research")]
fn project_choice_diagnostics(
    trace: &[PhysicalTransition],
    sites: &Sites,
) -> Vec<ResearchChoiceDiagnostic> {
    trace
        .iter()
        .filter_map(|transition| match &transition.event {
            PhysicalEvent::OutputCandidateEvaluated {
                target,
                ownership,
                path_inputs,
                path_origins,
                positive_path_strength,
                negative_path_strength,
                opportunity,
                supplied_opportunity,
                admitted_drive,
                projected_drive,
                threshold,
                consequence_tick,
                unanswered_returns,
                executable,
                ..
            } => Some(ResearchChoiceDiagnostic::Candidate {
                tick: transition.tick,
                phase: transition.phase,
                control: control_for_target(sites, *target)?,
                ownership: format!("{ownership:?}"),
                path_inputs: *path_inputs,
                path_origins: path_origins.clone(),
                positive_path_strength: *positive_path_strength,
                negative_path_strength: *negative_path_strength,
                opportunity: *opportunity,
                supplied_opportunity: *supplied_opportunity,
                admitted_drive: *admitted_drive,
                projected_drive: *projected_drive,
                threshold: *threshold,
                consequence_tick: *consequence_tick,
                unanswered_returns: *unanswered_returns,
                executable: *executable,
            }),
            PhysicalEvent::PhysicalTransitionContinuationEvaluated {
                target,
                current_owner_transition,
                unanswered_returns,
                admitted,
                ..
            } => Some(ResearchChoiceDiagnostic::TransitionContinuation {
                tick: transition.tick,
                phase: transition.phase,
                control: control_for_target(sites, *target)?,
                current_owner_transition: *current_owner_transition,
                unanswered_returns: *unanswered_returns,
                admitted: *admitted,
            }),
            PhysicalEvent::ConsequenceRecorded { link, junction } => {
                Some(ResearchChoiceDiagnostic::ConsequenceRecorded {
                    tick: transition.tick,
                    phase: transition.phase,
                    link: link.0,
                    junction: junction.0,
                })
            }
            PhysicalEvent::OrganismConsequenceConsumed {
                target,
                link,
                generation,
                consequence_tick,
            } => Some(ResearchChoiceDiagnostic::ConsequenceConsumed {
                tick: transition.tick,
                phase: transition.phase,
                control: control_for_target(sites, *target)?,
                link: link.0,
                generation: *generation,
                consequence_tick: *consequence_tick,
            }),
            PhysicalEvent::CompletedCycleContinuationEvaluated {
                target,
                consequence_tick,
                consequence_witnesses,
                unique_latest_tick,
                admitted,
                ..
            } => Some(ResearchChoiceDiagnostic::CompletedCycle {
                tick: transition.tick,
                phase: transition.phase,
                control: control_for_target(sites, *target)?,
                consequence_tick: *consequence_tick,
                consequence_witnesses: consequence_witnesses
                    .iter()
                    .map(|(link, generation)| (link.0, *generation))
                    .collect(),
                unique_latest_tick: *unique_latest_tick,
                admitted: *admitted,
            }),
            PhysicalEvent::OutputChoiceResolved {
                ordinary_target,
                current_transition_target,
                computed_winner_target,
                admitted,
                computed_winner_basis,
                admission_basis,
                ..
            } => Some(ResearchChoiceDiagnostic::Choice {
                tick: transition.tick,
                phase: transition.phase,
                ordinary_control: control_for_target(sites, *ordinary_target),
                current_transition_control: current_transition_target
                    .and_then(|target| control_for_target(sites, target)),
                computed_winner_control: control_for_target(sites, *computed_winner_target),
                admitted_controls: admitted
                    .iter()
                    .filter_map(|admission| control_for_target(sites, admission.target))
                    .collect(),
                computed_winner_basis: format!("{computed_winner_basis:?}"),
                admission_basis: format!("{admission_basis:?}"),
            }),
            _ => None,
        })
        .collect()
}

#[cfg(feature = "research")]
fn control_for_target(sites: &Sites, target: JunctionId) -> Option<BodyControl> {
    sites
        .motors
        .iter()
        .position(|candidate| *candidate == target)
        .map(control)
}

fn build_harness(
    protocol: Protocol,
    output_specific: bool,
    couple_palm_translation: bool,
    retinotopic: bool,
    visual_reach: bool,
    retina_offsets: &[(i16, i16); RETINA_FEATURES_PER_EYE],
    focused_sensor_field: bool,
    focused_action_projection: ResearchFocusedActionProjection,
) -> (Harness, Sites, Option<FocusedVision>) {
    let (junction_capacity, link_capacity) = if focused_sensor_field {
        (
            32_768,
            if focused_action_projection == ResearchFocusedActionProjection::PalmHorizontal {
                65_536
            } else {
                32_768
            },
        )
    } else {
        (8_192, 16_384)
    };
    let mut builder =
        HarnessBuilder::with_capacity(junction_capacity, link_capacity, OUTWARD_REGION);
    builder.set_protocol(protocol);
    builder.set_physical_tracing(true);
    let mut wiring = Wiring::new(&mut builder);

    let motors = wiring.actuator_bank(
        CONTROL_COUNT,
        CONTROL_PHYSICAL_BASE,
        SINK_PHYSICAL_BASE,
        |index, physical_id| {
            let position = motor_position(index, couple_palm_translation);
            JunctionSpec::ordinary(physical_id, position, 0, 2)
        },
        |index, physical_id| {
            let position = motor_position(index, couple_palm_translation);
            JunctionSpec::ordinary(physical_id, position, OUTWARD_REGION, 1)
        },
        DriveSpec::ordinary(1),
    );
    let sensors = wiring.receptor_bank::<BINS>(
        FEATURE_COUNT,
        SENSOR_PHYSICAL_BASE,
        |feature, _, physical_id| {
            let position = receptor_position(
                feature,
                couple_palm_translation,
                retinotopic,
                retina_offsets,
            );
            JunctionSpec::ordinary(physical_id, position, 0, 1)
        },
    );
    let focused_vision = focused_sensor_field.then(|| {
        let sensors = wiring.receptor_bank::<FOCUSED_RECEPTOR_VALUES>(
            FOCUSED_RECEPTOR_FEATURES,
            FOCUSED_SENSOR_PHYSICAL_BASE,
            |feature, _, physical_id| {
                JunctionSpec::ordinary(physical_id, focused_feature_position(feature), 0, 1)
            },
        );
        let relays = wiring.junction_bank(
            FOCUSED_RECEPTOR_FEATURES,
            FOCUSED_RELAY_PHYSICAL_BASE,
            |feature, physical_id| {
                let position = if focused_action_projection
                    == ResearchFocusedActionProjection::PalmHorizontal
                {
                    axis_position(BodyAxis::PalmHorizontal)
                } else {
                    focused_feature_position(feature).saturating_add(1)
                };
                JunctionSpec::ordinary(physical_id, position, 0, 1)
            },
        );
        for (targets, relay) in sensors.iter().zip(&relays) {
            for target in targets {
                wiring.drive(*target, *relay, DriveSpec::ordinary(1));
            }
        }
        FocusedVision::new(sensors, relays, focused_action_projection)
    });
    let visual_reach_relays: Vec<Option<[JunctionId; BINS]>> = if visual_reach {
        (0..RETINA_FEATURES)
            .map(|feature| {
                visual_reach_relay_position(feature, retina_offsets).map(|position| {
                    std::array::from_fn(|bin| {
                        let relay = wiring.junction(JunctionSpec::ordinary(
                            VISUAL_REACH_RELAY_PHYSICAL_BASE
                                + u64::try_from(feature.saturating_mul(BINS).saturating_add(bin))
                                    .unwrap_or(0),
                            position,
                            0,
                            1,
                        ));
                        wiring.drive(sensors[feature][bin], relay, DriveSpec::ordinary(1));
                        relay
                    })
                })
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let outcome_count = if output_specific {
        CONTROL_COUNT
    } else {
        AXIS_COUNT
    };
    let outcomes = wiring.junction_bank(
        outcome_count,
        OUTCOME_PHYSICAL_BASE,
        |index, physical_id| {
            let axis = BodyAxis::ALL[index / usize::from(output_specific).saturating_add(1)];
            JunctionSpec::ordinary(
                physical_id,
                2_000 + i32::try_from(axis.index()).unwrap_or(0) * 8,
                0,
                1,
            )
        },
    );

    let palm_anchor = couple_palm_translation.then(|| {
        wiring.junction(JunctionSpec::ordinary(
            ANCHOR_PHYSICAL_BASE + u64::try_from(BodyAxis::PalmHorizontal.index()).unwrap_or(0),
            3_000 + i32::try_from(BodyAxis::PalmHorizontal.index()).unwrap_or(0) * 8,
            0,
            99,
        ))
    });
    for axis in BodyAxis::ALL {
        let anchor = match palm_anchor {
            Some(anchor) if is_palm_translation(axis) => anchor,
            _ => wiring.junction(JunctionSpec::ordinary(
                ANCHOR_PHYSICAL_BASE + u64::try_from(axis.index()).unwrap_or(0),
                3_000 + i32::try_from(axis.index()).unwrap_or(0) * 8,
                0,
                99,
            )),
        };
        let outcome_start = if output_specific {
            axis.index() * 2
        } else {
            axis.index()
        };
        let outcome_end = outcome_start + if output_specific { 2 } else { 1 };
        for outcome in &outcomes[outcome_start..outcome_end] {
            wiring.drive(anchor, *outcome, DriveSpec::ordinary(1));
        }
        for (feature, bins) in sensors.iter().enumerate() {
            if receptor_axis(feature, retina_offsets) == axis {
                for sensor in bins {
                    wiring.drive(anchor, *sensor, DriveSpec::ordinary(1));
                    if output_specific {
                        for outcome in &outcomes[outcome_start..outcome_end] {
                            wiring.drive(*sensor, *outcome, DriveSpec::ordinary(1));
                        }
                    }
                }
            }
        }
        if axis == BodyAxis::PalmHorizontal {
            for relay in visual_reach_relays.iter().flatten().flatten() {
                wiring.drive(anchor, *relay, DriveSpec::ordinary(1));
                if output_specific {
                    for outcome in &outcomes[outcome_start..outcome_end] {
                        wiring.drive(*relay, *outcome, DriveSpec::ordinary(1));
                    }
                }
            }
            if let Some(focused) = focused_vision.as_ref().filter(|focused| {
                focused.action_projection == ResearchFocusedActionProjection::PalmHorizontal
            }) {
                for relay in &focused.relays {
                    wiring.drive(anchor, *relay, DriveSpec::ordinary(1));
                    for outcome in &outcomes[outcome_start..outcome_end] {
                        wiring.drive(*relay, *outcome, DriveSpec::ordinary(1));
                    }
                }
            }
        }
        let first_motor = axis.index() * 2;
        wiring.bind_output(motors[first_motor], outcomes[outcome_start]);
        wiring.bind_output(
            motors[first_motor + 1],
            outcomes[if output_specific {
                outcome_start + 1
            } else {
                outcome_start
            }],
        );
    }
    (
        builder.build(),
        Sites {
            sensors,
            motors,
            outcomes,
        },
        focused_vision,
    )
}

fn focused_feature_position(feature: usize) -> i32 {
    10_000_i32.saturating_add(i32::try_from(feature).unwrap_or(i32::MAX).saturating_mul(4))
}

const fn is_palm_translation(axis: BodyAxis) -> bool {
    matches!(
        axis,
        BodyAxis::PalmHorizontal | BodyAxis::PalmVertical | BodyAxis::PalmDepth
    )
}

fn motor_position(index: usize, couple_palm_translation: bool) -> i32 {
    let control = control(index);
    let center = if couple_palm_translation && is_palm_translation(control.axis()) {
        axis_position(BodyAxis::PalmHorizontal)
    } else {
        axis_position(control.axis())
    };
    match control.direction() {
        Direction::Decrease => center - 1,
        Direction::Increase => center + 1,
    }
}

fn receptor_position(
    feature: usize,
    couple_palm_translation: bool,
    retinotopic: bool,
    retina_offsets: &[(i16, i16); RETINA_FEATURES_PER_EYE],
) -> i32 {
    let axis = receptor_axis(feature, retina_offsets);
    let center = if couple_palm_translation && is_palm_translation(axis) {
        axis_position(BodyAxis::PalmHorizontal)
    } else {
        axis_position(axis)
    };
    if !retinotopic || feature >= RETINA_FEATURES {
        return center;
    }

    let (dx, dy) = retina_offsets[feature % RETINA_FEATURES_PER_EYE];
    let dominant_offset = if dx.unsigned_abs() >= dy.unsigned_abs() {
        dx
    } else {
        dy
    };
    center.saturating_add(i32::from(dominant_offset.signum()) * 2)
}

fn visual_reach_relay_position(
    feature: usize,
    retina_offsets: &[(i16, i16); RETINA_FEATURES_PER_EYE],
) -> Option<i32> {
    if feature >= RETINA_FEATURES {
        return None;
    }
    let (dx, dy) = retina_offsets[feature % RETINA_FEATURES_PER_EYE];
    if dx == 0 || dx.unsigned_abs() < dy.unsigned_abs() {
        return None;
    }
    Some(
        axis_position(BodyAxis::PalmHorizontal)
            .saturating_add(i32::from(dx.signum()).saturating_mul(2)),
    )
}

fn axis_position(axis: BodyAxis) -> i32 {
    AXIS_POSITION_BASE
        .saturating_add(i32::try_from(axis.index()).unwrap_or(0) * AXIS_POSITION_STRIDE)
}

fn receptor_axis(
    feature: usize,
    retina_offsets: &[(i16, i16); RETINA_FEATURES_PER_EYE],
) -> BodyAxis {
    debug_assert!(feature < FEATURE_COUNT);
    if feature < RETINA_FEATURES {
        let eye = Eye::ALL[feature / RETINA_FEATURES_PER_EYE];
        let (dx, dy) = retina_offsets[feature % retina_offsets.len()];
        return if dx.unsigned_abs() >= dy.unsigned_abs() {
            BodyAxis::EyeHorizontal { eye }
        } else {
            BodyAxis::EyeVertical { eye }
        };
    }
    if feature < EXTERNAL_FEATURE_COUNT {
        let contact = feature - RETINA_FEATURES;
        return if contact == 0 {
            BodyAxis::PalmDepth
        } else {
            BodyAxis::FingerFlexion {
                digit: Digit::ALL[contact - 1],
            }
        };
    }
    BodyAxis::ALL[(feature - EXTERNAL_FEATURE_COUNT) / RECEPTORS_PER_AXIS]
}

fn sensory_features(
    sample: &WorldSample,
    state: &WorkstationState,
    retina_offsets: &[(i16, i16); RETINA_FEATURES_PER_EYE],
) -> [u8; FEATURE_COUNT] {
    let mut values = [0_u8; FEATURE_COUNT];
    let gazes = Eye::ALL.map(|eye| state.eye(eye).gaze());
    let retinal = retinal_features_at(sample, gazes, retina_offsets);
    values[..RETINA_FEATURES].copy_from_slice(&retinal);
    let mut cursor = RETINA_FEATURES;
    for contact in sample.contacts() {
        values[cursor] = contact_value(*contact);
        cursor += 1;
    }
    debug_assert_eq!(cursor, EXTERNAL_FEATURE_COUNT);
    for sense in state.proprioception() {
        for value in signed_channels(sense.position / 4)
            .into_iter()
            .chain([if sense.position == 0 { u8::MAX } else { 0 }])
            .chain(signed_channels(sense.velocity))
            .chain([
                bounded_magnitude(sense.decrease_effort),
                bounded_magnitude(sense.increase_effort),
                if sense.at_lower_limit { u8::MAX } else { 0 },
                if sense.at_upper_limit { u8::MAX } else { 0 },
            ])
        {
            values[cursor] = value;
            cursor += 1;
        }
    }
    debug_assert_eq!(cursor, FEATURE_COUNT);
    values
}

fn retinal_features_at(
    sample: &WorldSample,
    gazes: [crate::Point; 2],
    retina_offsets: &[(i16, i16); RETINA_FEATURES_PER_EYE],
) -> [u8; RETINA_FEATURES] {
    let mut values = [0; RETINA_FEATURES];
    let mut cursor = 0;
    for eye in Eye::ALL {
        let focus = gazes[eye.index()];
        for (dx, dy) in *retina_offsets {
            values[cursor] = sample.eye(eye).sample(focus.offset(dx, dy));
            cursor += 1;
        }
    }
    debug_assert_eq!(cursor, RETINA_FEATURES);
    values
}

fn focused_receptor_values(
    sample: &WorldSample,
    state: &WorkstationState,
) -> Result<(Vec<u8>, [usize; 2]), WorkstationError> {
    let profile = FocusProfile::<2>::new(FOCUSED_REFINEMENT_DEPTH, 1)
        .map_err(|_| WorkstationError::InvalidState)?;
    debug_assert_eq!(profile.region_bound(), FOCUSED_REGIONS_PER_EYE);
    let mut values = Vec::with_capacity(FOCUSED_RECEPTOR_FEATURES);
    let mut active_regions = [0; 2];
    for eye in Eye::ALL {
        let image = sample.eye(eye);
        let shape = [usize::from(image.height()), usize::from(image.width())];
        let field = SpatialField::new(
            shape,
            image
                .pixels()
                .iter()
                .copied()
                .map(Availability::Available)
                .collect(),
        )
        .map_err(|_| WorkstationError::InvalidState)?;
        let gaze = state.eye(eye).gaze();
        let focus = profile
            .focuses(
                shape,
                [[
                    scale_focus(gaze.y(), shape[0]),
                    scale_focus(gaze.x(), shape[1]),
                ]],
            )
            .map_err(|_| WorkstationError::InvalidState)?;
        let frame = field
            .focus_partition(focus)
            .map_err(|_| WorkstationError::InvalidState)?
            .transduce_complete(0_u64, u64::from, u64::saturating_add)
            .map(|sum| u32::try_from(sum).expect("bounded light field sum fits u32"))
            .into_receptor_frame();
        active_regions[eye.index()] = frame.active_region_count();
        for slot in frame.slots() {
            for bit in 0..FOCUSED_BITS_PER_REGION {
                values.push(match slot {
                    Availability::Unavailable => 0,
                    Availability::Available(value) if (*value >> bit) & 1 == 0 => 1,
                    Availability::Available(_) => 2,
                });
            }
        }
    }
    debug_assert_eq!(values.len(), FOCUSED_RECEPTOR_FEATURES);
    Ok((values, active_regions))
}

fn scale_focus(position: i16, extent: usize) -> usize {
    usize::try_from(position)
        .unwrap_or(0)
        .saturating_mul(extent.saturating_sub(1))
        / usize::try_from(BODY_MAX).unwrap_or(1)
}

#[cfg(feature = "research")]
const fn transition_uses_sparse_retina(transition: ResearchTransitionOpportunity) -> bool {
    matches!(
        transition,
        ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetina
            | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransition
            | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopic
            | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicMagnitude
            | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholds
            | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsVisualReach
            | ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach
    )
}

#[cfg(any(feature = "research", test))]
fn retinal_bins(values: [u8; RETINA_FEATURES]) -> [u8; RETINA_FEATURES] {
    values.map(|value| u8::try_from(RETINAL_QUANTIZER.bin(u16::from(value))).unwrap_or(u8::MAX))
}

#[cfg(feature = "research")]
fn retinal_output_origins(
    before: RetinalState,
    current_gazes: [crate::Point; 2],
    retina_offsets: &[(i16, i16); RETINA_FEATURES_PER_EYE],
) -> [Option<u64>; RETINA_FEATURES] {
    if !before.initialized {
        return [None; RETINA_FEATURES];
    }
    std::array::from_fn(|feature| {
        let eye = Eye::ALL[feature / RETINA_FEATURES_PER_EYE];
        let axis = receptor_axis(feature, retina_offsets);
        let change = match axis {
            BodyAxis::EyeHorizontal { .. } => current_gazes[eye.index()]
                .x()
                .cmp(&before.gazes[eye.index()].x()),
            BodyAxis::EyeVertical { .. } => current_gazes[eye.index()]
                .y()
                .cmp(&before.gazes[eye.index()].y()),
            _ => return None,
        };
        let direction = match change {
            std::cmp::Ordering::Less => Direction::Decrease,
            std::cmp::Ordering::Greater => Direction::Increase,
            std::cmp::Ordering::Equal => return None,
        };
        Some(
            OUTCOME_PHYSICAL_BASE
                .saturating_add(u64::try_from(axis.index() * 2).unwrap_or(0))
                .saturating_add(u64::from(direction == Direction::Increase)),
        )
    })
}

#[cfg(feature = "research")]
pub fn research_retinal_features(
    sample: &WorldSample,
    state: &WorkstationState,
) -> [u8; RESEARCH_RETINA_FEATURE_COUNT] {
    research_retinal_features_with(sample, state, &RETINA_OFFSETS)
}

#[cfg(feature = "research")]
pub fn research_wide_retinal_features(
    sample: &WorldSample,
    state: &WorkstationState,
) -> [u8; RESEARCH_RETINA_FEATURE_COUNT] {
    research_retinal_features_with(sample, state, &WIDE_RETINA_OFFSETS)
}

#[cfg(feature = "research")]
pub fn research_retinotopic_retinal_features(
    sample: &WorldSample,
    state: &WorkstationState,
) -> [u8; RESEARCH_RETINA_FEATURE_COUNT] {
    research_retinal_features_with(sample, state, &RETINOTOPIC_RETINA_OFFSETS)
}

#[cfg(feature = "research")]
pub fn research_foveal_reach_retinal_features(
    sample: &WorldSample,
    state: &WorkstationState,
) -> [u8; RESEARCH_RETINA_FEATURE_COUNT] {
    research_retinal_features_with(sample, state, &FOVEAL_REACH_RETINA_OFFSETS)
}

#[cfg(feature = "research")]
fn research_retinal_features_with(
    sample: &WorldSample,
    state: &WorkstationState,
    retina_offsets: &[(i16, i16); RETINA_FEATURES_PER_EYE],
) -> [u8; RESEARCH_RETINA_FEATURE_COUNT] {
    let features = sensory_features(sample, state, retina_offsets);
    features[..RETINA_FEATURES]
        .try_into()
        .expect("retinal feature count is fixed")
}

const fn effect_receptor_offset(offset: usize) -> bool {
    matches!(offset, 3..=6)
}

fn proprioceptor_delta_for(sense: AxisProprioception) -> [bool; RECEPTORS_PER_AXIS] {
    proprioceptor_delta(
        sense.position,
        sense.velocity,
        sense.decrease_effort,
        sense.increase_effort,
        sense.at_lower_limit,
        sense.at_upper_limit,
    )
}

fn proprioceptor_delta(
    position: i16,
    velocity: i16,
    decrease_effort: u16,
    increase_effort: u16,
    at_lower_limit: bool,
    at_upper_limit: bool,
) -> [bool; RECEPTORS_PER_AXIS] {
    let before_position = position.saturating_sub(velocity);
    let before_signed = signed_channels(before_position / 4);
    let current_signed = signed_channels(position / 4);
    let before_position_values = [
        before_signed[0],
        before_signed[1],
        if before_position == 0 { u8::MAX } else { 0 },
    ];
    let current_position_values = [
        current_signed[0],
        current_signed[1],
        if position == 0 { u8::MAX } else { 0 },
    ];
    let velocity_values = signed_channels(velocity);
    let current = [
        current_position_values[0],
        current_position_values[1],
        current_position_values[2],
        velocity_values[0],
        velocity_values[1],
        bounded_magnitude(decrease_effort),
        bounded_magnitude(increase_effort),
        if at_lower_limit { u8::MAX } else { 0 },
        if at_upper_limit { u8::MAX } else { 0 },
    ];
    std::array::from_fn(|offset| {
        if current[offset] == 0 {
            return false;
        }
        match offset {
            0..=2 => {
                before_position_values[offset] == 0
                    || before_position_values[offset] / 64 != current[offset] / 64
            }
            3..=6 => true,
            7..=8 => velocity != 0,
            _ => false,
        }
    })
}

fn contact_value(contact: ContactSample) -> u8 {
    let pressure = contact.pressure() / 4;
    let slip = contact.slip().unsigned_abs() / 4;
    u8::try_from(pressure.saturating_add(slip).min(u16::from(u8::MAX))).unwrap_or(u8::MAX)
}

fn control(index: usize) -> BodyControl {
    let axis = BodyAxis::ALL[index / 2];
    let direction = if index.is_multiple_of(2) {
        Direction::Decrease
    } else {
        Direction::Increase
    };
    control_for_axis(axis, direction)
}

const fn control_for_axis(axis: BodyAxis, direction: Direction) -> BodyControl {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_transition_uses_only_velocity_and_effort_factors() {
        let selected = (0..RECEPTORS_PER_AXIS)
            .filter(|offset| effect_receptor_offset(*offset))
            .collect::<Vec<_>>();
        assert_eq!(selected, vec![3, 4, 5, 6]);
    }

    #[test]
    fn proprioceptor_delta_selects_only_changed_active_bins() {
        assert_eq!(
            selected_offsets(proprioceptor_delta(16, 16, 0, 1, false, false)),
            vec![1, 4, 6]
        );
        assert_eq!(
            selected_offsets(proprioceptor_delta(32, 16, 0, 1, false, false)),
            vec![4, 6]
        );
        assert_eq!(
            selected_offsets(proprioceptor_delta(511, 3, 0, 1, false, true)),
            vec![4, 6, 8]
        );
    }

    #[test]
    fn retinal_delta_requires_a_changed_bin() {
        let mut state = RetinalState::default();
        let first = [1; RETINA_FEATURES];
        let gazes = [crate::Point::default(); 2];
        assert_eq!(state.observe(first, gazes), [false; RETINA_FEATURES]);
        assert_eq!(state.observe(first, gazes), [false; RETINA_FEATURES]);

        let mut changed = first;
        changed[5] = 3;
        let delta = state.observe(changed, gazes);
        assert!(delta[5]);
        assert_eq!(delta.iter().filter(|changed| **changed).count(), 1);
        assert_eq!(state.observe(changed, gazes), [false; RETINA_FEATURES]);

        let mut same_value_bin = [192; RETINA_FEATURES];
        same_value_bin[5] = 193;
        assert_eq!(retinal_bins(same_value_bin), [3; RETINA_FEATURES]);
    }

    #[test]
    fn focused_driver_emits_only_actual_receptor_changes() {
        let (_, _, focused) = build_harness(
            Protocol::RecursiveLearnerCausalTopologyProductComposition,
            false,
            false,
            false,
            false,
            &RETINA_OFFSETS,
            true,
            ResearchFocusedActionProjection::Isolated,
        );
        let mut focused = focused.unwrap();
        let state = WorkstationState::default();
        let blank = dark_sample();
        let initial = focused.observe(&blank, &state, 1).unwrap();
        assert!(initial.inputs.is_empty());
        assert_eq!(initial.active_regions, [1, 1]);
        assert!(focused
            .observe(&blank, &state, 2)
            .unwrap()
            .inputs
            .is_empty());

        let light = WorldSample::new(
            [
                crate::LightField::filled(1, 1, u8::MAX).unwrap(),
                crate::LightField::filled(1, 1, u8::MAX).unwrap(),
            ],
            [ContactSample::default(); crate::TOUCH_SITES],
        )
        .unwrap();
        let changed = focused.observe(&light, &state, 3).unwrap();
        assert_eq!(changed.inputs.len(), 16);
        assert_eq!(changed.changes.len(), 16);
        assert!(changed
            .inputs
            .iter()
            .all(|input| input.incidence == PhysicalIncidence::Transition));
        assert!(focused
            .observe(&light, &state, 4)
            .unwrap()
            .inputs
            .is_empty());
    }

    #[test]
    fn focused_driver_bounds_edge_gaze_and_rejects_corrupt_state() {
        assert_eq!(scale_focus(0, 37), 0);
        assert_eq!(scale_focus(BODY_MAX, 37), 36);

        let (_, _, focused) = build_harness(
            Protocol::RecursiveLearnerCausalTopologyProductComposition,
            false,
            false,
            false,
            false,
            &RETINA_OFFSETS,
            true,
            ResearchFocusedActionProjection::Isolated,
        );
        let mut focused = focused.unwrap();
        focused.previous = Some(vec![
            FOCUSED_RECEPTOR_VALUES as u8;
            FOCUSED_RECEPTOR_FEATURES
        ]);
        assert_eq!(focused.validate(), Err(WorkstationError::InvalidCheckpoint));
    }

    #[test]
    fn focused_action_projection_is_symmetric_inside_one_palm_component() {
        let (boundary, sites, focused) = build_harness(
            Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
            true,
            true,
            false,
            false,
            &RETINA_OFFSETS,
            true,
            ResearchFocusedActionProjection::PalmHorizontal,
        );
        let focused = focused.unwrap();
        let observation = boundary.read();
        let anchor_physical =
            ANCHOR_PHYSICAL_BASE + u64::try_from(BodyAxis::PalmHorizontal.index()).unwrap_or(0);
        let anchor = observation
            .junctions
            .iter()
            .find(|junction| junction.physical_id == anchor_physical)
            .unwrap()
            .id;
        let outcome_start = BodyAxis::PalmHorizontal.index() * 2;
        let palm_outcomes = &sites.outcomes[outcome_start..outcome_start + 2];

        for relay in &focused.relays {
            assert_eq!(
                observation.junction(*relay).unwrap().position,
                axis_position(BodyAxis::PalmHorizontal)
            );
            assert_eq!(
                observation
                    .links
                    .iter()
                    .filter(|link| link.from == anchor && link.to == *relay)
                    .count(),
                1
            );
            let targets = observation
                .links
                .iter()
                .filter(|link| link.from == *relay && palm_outcomes.contains(&link.to))
                .map(|link| link.to)
                .collect::<BTreeSet<_>>();
            assert_eq!(targets, palm_outcomes.iter().copied().collect());
        }
    }

    fn selected_offsets(selected: [bool; RECEPTORS_PER_AXIS]) -> Vec<usize> {
        selected
            .into_iter()
            .enumerate()
            .filter_map(|(offset, selected)| selected.then_some(offset))
            .collect()
    }
    use std::collections::BTreeSet;

    fn dark_sample() -> WorldSample {
        WorldSample::new(
            [
                crate::LightField::filled(1, 1, 0).unwrap(),
                crate::LightField::filled(1, 1, 0).unwrap(),
            ],
            [ContactSample::default(); crate::TOUCH_SITES],
        )
        .unwrap()
    }

    #[test]
    fn every_control_maps_to_one_axis_and_the_two_directions() {
        for axis in BodyAxis::ALL {
            let pair = [control(axis.index() * 2), control(axis.index() * 2 + 1)];
            assert_eq!(pair.map(BodyControl::axis), [axis, axis]);
            assert_eq!(
                pair.map(BodyControl::direction),
                [Direction::Decrease, Direction::Increase]
            );
        }
    }

    #[test]
    fn each_axis_has_one_distinct_local_outcome_component() {
        let (boundary, sites, _) = build_harness(
            Protocol::RecursiveLearnerCausalTopologyProductComposition,
            false,
            false,
            false,
            false,
            &RETINA_OFFSETS,
            false,
            ResearchFocusedActionProjection::Isolated,
        );
        assert_eq!(
            boundary.read().protocol,
            Protocol::RecursiveLearnerCausalTopologyProductComposition
        );
        assert_eq!(sites.outcomes.len(), AXIS_COUNT);
        assert_eq!(
            sites
                .outcomes
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            AXIS_COUNT
        );
    }

    #[test]
    fn every_receptor_is_local_to_one_anatomical_axis() {
        for feature in 0..FEATURE_COUNT {
            let axis = receptor_axis(feature, &RETINA_OFFSETS);
            let receptor = receptor_position(feature, false, false, &RETINA_OFFSETS);
            let local = (0..CONTROL_COUNT)
                .filter(|index| motor_position(*index, false).abs_diff(receptor) <= 2)
                .collect::<Vec<_>>();
            assert_eq!(local.len(), 2, "feature {feature} local motors {local:?}");
            assert!(local.iter().all(|index| control(*index).axis() == axis));
        }
    }

    #[test]
    #[cfg(feature = "research")]
    fn retinotopy_preserves_retinal_sign_as_one_local_direction() {
        for feature in 0..RETINA_FEATURES {
            let axis = receptor_axis(feature, &RETINOTOPIC_RETINA_OFFSETS);
            let receptor = receptor_position(feature, false, true, &RETINOTOPIC_RETINA_OFFSETS);
            let local = (0..CONTROL_COUNT)
                .filter(|index| motor_position(*index, false).abs_diff(receptor) <= 2)
                .collect::<Vec<_>>();
            assert_eq!(local.len(), 1, "feature {feature} local motors {local:?}");
            assert_eq!(control(local[0]).axis(), axis);
            let (dx, dy) = RETINOTOPIC_RETINA_OFFSETS[feature % RETINA_FEATURES_PER_EYE];
            let offset = if dx.unsigned_abs() >= dy.unsigned_abs() {
                dx
            } else {
                dy
            };
            let expected = if offset.is_negative() {
                Direction::Decrease
            } else {
                Direction::Increase
            };
            assert_eq!(control(local[0]).direction(), expected);
        }
    }

    #[test]
    #[cfg(feature = "research")]
    fn visual_reach_projection_preserves_sign_without_duplicating_input() {
        for feature in 0..RETINA_FEATURES {
            let Some(position) = visual_reach_relay_position(feature, &RETINOTOPIC_RETINA_OFFSETS)
            else {
                continue;
            };
            let local = (0..CONTROL_COUNT)
                .filter(|index| motor_position(*index, false).abs_diff(position) <= 2)
                .collect::<Vec<_>>();
            assert_eq!(local.len(), 1, "feature {feature} local motors {local:?}");
            let control = control(local[0]);
            assert_eq!(control.axis(), BodyAxis::PalmHorizontal);
            let (dx, _) = RETINOTOPIC_RETINA_OFFSETS[feature % RETINA_FEATURES_PER_EYE];
            let expected = if dx.is_negative() {
                Direction::Decrease
            } else {
                Direction::Increase
            };
            assert_eq!(control.direction(), expected);
        }
    }

    #[test]
    fn neutral_proprioception_keeps_every_axis_physically_present() {
        let features = sensory_features(
            &dark_sample(),
            &WorkstationState::default(),
            &RETINA_OFFSETS,
        );
        for axis in BodyAxis::ALL {
            let first = EXTERNAL_FEATURE_COUNT + axis.index() * RECEPTORS_PER_AXIS;
            assert_eq!(&features[first..first + 3], &[0, 0, u8::MAX]);
        }
    }
}
