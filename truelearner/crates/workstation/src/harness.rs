use crate::checkpoint::WorkstationCheckpoint;
use crate::state::{ActuatorFrame, BodyControl, Direction};
use crate::{
    BodyAxis, BodyMovement, Eye, Point, WorkstationError, WorkstationState, WorldSample,
    AXIS_COUNT, BODY_MAX, TOUCH_SITES,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_body::{
    harness::{
        attach_boundary_component, attach_learnable_link, attach_outcome_component,
        attach_progress_component, attach_sensor, motor,
    },
    Arrival, AutomaticityWork, Body, BodyCheckpoint, BodyCheckpointError, Junction, JunctionId,
    Link, LinkId, Run, TraceEvent as BodyTraceEvent, Trigger, Work,
};

const CONTROL_COUNT: usize = AXIS_COUNT * 2;
const RECEPTOR_SIDE: usize = 9;
const RECEPTORS_PER_EYE: usize = RECEPTOR_SIDE * RECEPTOR_SIDE;
const VISUAL_SENSOR_COUNT: usize = Eye::ALL.len() * RECEPTORS_PER_EYE;
const SALIENCE_COUNT: usize = Eye::ALL.len() * RECEPTORS_PER_EYE;
const CONTACT_FIELDS: usize = 2;
const PROPRIOCEPTIVE_FIELDS: usize = 6;
const SENSOR_COUNT: usize = VISUAL_SENSOR_COUNT
    + SALIENCE_COUNT
    + TOUCH_SITES * CONTACT_FIELDS
    + AXIS_COUNT * PROPRIOCEPTIVE_FIELDS;
const SENSOR_LIFETIME: u64 = u64::MAX;
const SENSOR_PRIME: i32 = i32::MIN;
/// Retinal light at or above this floor counts as salient — the one shared
/// signal of "what stands out" that the foveation and pre-reach reflexes and
/// the learner all read. Just above mid-range: above the rendered hand of
/// the body course (palm 96, fingertips 128) and every background, below
/// every application's target bands. The floor belongs to the organism's
/// retina, not to any course or application.
const SALIENCE_FLOOR: u8 = 129;
/// The drive stops pushing once the palm sits this close to the salience
/// centroid. The centroid is a mean over lit receptors, so it is accurate
/// to a few world units; a small deadzone lands the palm well inside any
/// target the reflex can see.
const REACH_DEADZONE: i32 = 32;
/// Receptors this close to the organism's own palm are its own hand, never
/// a target. The mask is the hand's true visual size: a real occluder, not
/// a zone — a larger mask would hide the very target the palm reaches once
/// it arrives, and the reach would orbit it forever.
const REACH_HAND_MASK: i32 = 40;
const LIGHT_RANGE: u32 = u8::MAX as u32;
const BODY_RANGE: u32 = BODY_MAX as u32;
const SIGNED_BODY_RANGE: u32 = BODY_RANGE * 2;
const COMPETITION_COMPONENTS: usize = 4;
const OUTCOME_COMPONENTS: usize = AXIS_COUNT;
const MOMENT_LIMIT: usize = 512;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Handles {
    pub(crate) vision: [Vec<JunctionId>; 2],
    /// One salience cell per receptor: tonic cells that fire while their
    /// receptor is lit above the salience floor. The foveation reflex is
    /// wired from them at attach time.
    pub(crate) salience: [Vec<JunctionId>; 2],
    /// The learnable value link from each receptor's light sensor onto its
    /// salience cell: zero impulse at birth, so the reflexes see only raw
    /// brightness until the learner's own consequence history strengthens
    /// it. Value written onto salience — top-down attention, earned. Stored
    /// by link slot for checkpointing; `value_link` reconstructs the id.
    pub(crate) value: [Vec<u32>; 2],
    pub(crate) contacts: [[JunctionId; CONTACT_FIELDS]; TOUCH_SITES],
    pub(crate) proprioception: [[JunctionId; PROPRIOCEPTIVE_FIELDS]; AXIS_COUNT],
    pub(crate) exploration: [JunctionId; COMPETITION_COMPONENTS],
    pub(crate) competition_outcomes: [JunctionId; COMPETITION_COMPONENTS],
    pub(crate) outcomes: [JunctionId; OUTCOME_COMPONENTS],
    pub(crate) resisted_progress: [JunctionId; AXIS_COUNT],
    pub(crate) opportunities: Vec<JunctionId>,
    pub(crate) outward: Vec<(JunctionId, BodyControl)>,
}

impl Handles {
    /// The learnable value link of one receptor, reconstructed from its
    /// stored slot.
    pub(crate) fn value_link(&self, eye: Eye, receptor: usize) -> Option<LinkId> {
        LinkId::new(self.value[eye.index()][receptor].saturating_sub(1) as usize)
    }

    fn valid_for(&self, body: &Body) -> bool {
        if self
            .vision
            .iter()
            .any(|receptors| receptors.len() != RECEPTORS_PER_EYE)
            || self
                .salience
                .iter()
                .any(|cells| cells.len() != RECEPTORS_PER_EYE)
            || self.opportunities.len() != CONTROL_COUNT
            || self.outward.len() != CONTROL_COUNT
        {
            return false;
        }
        let controls_are_canonical = self
            .outward
            .iter()
            .map(|(_, body_control)| *body_control)
            .eq(BodyAxis::ALL.into_iter().flat_map(|axis| {
                [Direction::Decrease, Direction::Increase]
                    .into_iter()
                    .map(move |direction| control(axis, direction))
            }));
        let value_links_valid = Eye::ALL.into_iter().all(|eye| {
            (0..RECEPTORS_PER_EYE).all(|receptor| {
                self.value_link(eye, receptor)
                    .is_some_and(|_| self.value[eye.index()][receptor] > 0)
                    && self.salience[eye.index()].len() == RECEPTORS_PER_EYE
            })
        });
        controls_are_canonical
            && value_links_valid
            && self
                .vision
                .iter()
                .flatten()
                .chain(self.salience.iter().flatten())
                .chain(self.contacts.iter().flatten())
                .chain(self.proprioception.iter().flatten())
                .chain(&self.exploration)
                .chain(&self.competition_outcomes)
                .chain(&self.outcomes)
                .chain(&self.resisted_progress)
                .chain(&self.opportunities)
                .chain(self.outward.iter().map(|(junction, _)| junction))
                .all(|junction| body.held(*junction).is_some())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotorEffect {
    pub at: u64,
    pub control: BodyControl,
    pub impulse: i32,
    pub cause: u64,
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
    pub physical_trace_events: u64,
}

impl StepMetrics {
    fn from_run(run: Run, resident_bytes: usize, physical_trace_events: u64) -> Self {
        Self {
            physical_work: total_work(run.work),
            drive_deliveries: run.work.emissions,
            modulatory_deliveries: 0,
            plasticity_updates: run.work.changes,
            structural_proposals: run.work.changes,
            junction_proposals: 0,
            resident_bytes,
            physical_trace_events,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationStepObservation {
    pub sequence: u64,
    pub state_before: WorkstationState,
    pub state_after: WorkstationState,
    pub pose_changed: bool,
    pub admitted_inputs: usize,
    pub opportunity_admitted: bool,
    pub boundary_parents: Vec<MotorEffect>,
    pub progress_parents: Vec<MotorEffect>,
    pub crossings: Vec<MotorEffect>,
    /// Crossings that pushed an axis into its own joint stop without moving.
    /// Each is a completed boundary whose exact parent is that crossing.
    pub joint_stops: Vec<MotorEffect>,
    pub movements: Vec<BodyMovement>,
    pub returned_transitions: Vec<BodyAxis>,
    pub pending_transitions: Vec<BodyAxis>,
    pub metrics: StepMetrics,
    pub naturally_quiescent: bool,
    pub body_fingerprint: String,
    pub physical_tick: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationRead {
    pub state: WorkstationState,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub return_path_count: usize,
    pub resident_bytes: usize,
    pub pending_transitions: Vec<BodyAxis>,
}

#[derive(Clone, Debug)]
pub struct WorkstationHarness {
    body: Body,
    handles: Handles,
    state: WorkstationState,
    sequence: u64,
    physical_tick: u64,
    pending_transitions: [bool; AXIS_COUNT],
    pending_stops: Vec<MotorEffect>,
    /// Sustained-reach strain per planar axis, signed by aim direction: the
    /// insistent-reaching integral. While a planar aim persists, the reach
    /// pulse grows; when the aim clears or flips, the strain resets.
    reach_strain: [i32; 2],
    /// Insistent-vergence strain: while the fusion error persists, the
    /// vergence pulse grows; fused or absent, it resets.
    vergence_strain: i32,
    history: Vec<WorldSample>,
}

impl PartialEq for WorkstationHarness {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.sequence == other.sequence
            && self.physical_tick == other.physical_tick
            && self.pending_transitions == other.pending_transitions
            && self.pending_stops == other.pending_stops
            && self.reach_strain == other.reach_strain
            && self.vergence_strain == other.vergence_strain
            && self.history == other.history
    }
}

impl Eq for WorkstationHarness {}

impl WorkstationHarness {
    pub fn new(_seed: u64) -> Result<Self, WorkstationError> {
        Self::fresh()
    }

    fn fresh() -> Result<Self, WorkstationError> {
        let mut body = Body::default();
        body.reserve(
            COMPETITION_COMPONENTS * 2
                + OUTCOME_COMPONENTS
                + AXIS_COUNT
                + SENSOR_COUNT
                + CONTROL_COUNT * 2,
            1_024,
        );
        let mut opportunities = Vec::with_capacity(CONTROL_COUNT);
        let mut outward = Vec::with_capacity(CONTROL_COUNT);
        for axis in BodyAxis::ALL {
            for direction in [Direction::Decrease, Direction::Increase] {
                let attached = motor(&mut body);
                opportunities.push(attached.opportunity);
                outward.push((attached.effect, control(axis, direction)));
            }
        }
        let exploration = std::array::from_fn(|component| {
            let nearby = BodyAxis::ALL
                .into_iter()
                .filter(|axis| competition_component(*axis) == component)
                .flat_map(|axis| axis_nearness(&opportunities, axis))
                .collect::<Vec<_>>();
            attach_sensor(&mut body, Junction::integrating(1), &nearby)
        });
        let mut prime = Vec::with_capacity(SENSOR_COUNT);
        let mut vision = Eye::ALL.map(|_| Vec::with_capacity(RECEPTORS_PER_EYE));
        let mut salience = Eye::ALL.map(|_| Vec::with_capacity(RECEPTORS_PER_EYE));
        let mut value = Eye::ALL.map(|_| vec![0_u32; RECEPTORS_PER_EYE]);
        for eye in Eye::ALL {
            // The receptor index names three parallel structures: the
            // vision sensor, the salience cell, and the value link between
            // them.
            #[allow(clippy::needless_range_loop)]
            for receptor in 0..RECEPTORS_PER_EYE {
                let nearby = eye_palm_nearness(&opportunities, eye, receptor);
                let sensor = attach_sampled_sensor(&mut body, LIGHT_RANGE, &nearby, &mut prime);
                vision[eye.index()].push(sensor);
                // The birthright foveation reflex: a tonic salience cell per
                // receptor, wired to the eye opportunities that move gaze
                // toward this receptor. The arc's impulse reaches the motor
                // threshold alone, so a seen target wins the crossing the
                // step it appears; a centered stimulus drives both sides
                // equally, so balance — not a constant — terminates the
                // pull, and only summed effort above two can outvote it.
                let cell = attach_sensor(&mut body, Junction::integrating(1), &[]);
                for opportunity in eye_foveation_drive(&opportunities, eye, receptor) {
                    body.add_link(Link::new(cell, opportunity, 1, 2))
                        .expect("validated foveation drive arc");
                }
                salience[eye.index()].push(cell);
                // The learnable value link: this receptor's light sensor onto
                // its own salience cell, a whisper of impulse at birth. It
                // fires only when the receptor brightens through the
                // salience floor — the same events the reflexes answer — so
                // dark and quiet scenes carry nothing and the exploration
                // clock stays exact. A receptor whose light has paid can be
                // strengthened, becoming effectively brighter to the
                // reflexes: top-down attention, earned from the learner's
                // own consequence history.
                let link = attach_learnable_link(
                    &mut body,
                    sensor,
                    cell,
                    1,
                    Trigger::RisesThrough(i32::from(SALIENCE_FLOOR)),
                );
                value[eye.index()][receptor] = (link.slot() as u32)
                    .checked_add(1)
                    .expect("link slot is bounded");
            }
        }
        let contacts = std::array::from_fn(|_site| {
            let pressure_nearby = contact_pressure_nearness(&opportunities);
            let slip_nearby = contact_slip_nearness(&opportunities);
            [
                attach_sampled_sensor(&mut body, BODY_RANGE, &pressure_nearby, &mut prime),
                attach_sampled_sensor(&mut body, SIGNED_BODY_RANGE, &slip_nearby, &mut prime),
            ]
        });
        let proprioception = BodyAxis::ALL.map(|axis| {
            let nearby = axis_nearness(&opportunities, axis);
            [
                attach_sampled_sensor(&mut body, axis_range(axis), &nearby, &mut prime),
                attach_sampled_sensor(&mut body, axis_range(axis), &nearby, &mut prime),
                attach_sampled_sensor(&mut body, BODY_RANGE, &nearby, &mut prime),
                attach_sampled_sensor(&mut body, BODY_RANGE, &nearby, &mut prime),
                attach_sampled_sensor(&mut body, 1, &nearby, &mut prime),
                attach_sampled_sensor(&mut body, 1, &nearby, &mut prime),
            ]
        });
        let competition_outcomes =
            std::array::from_fn(|_| attach_sensor(&mut body, Junction::integrating(1), &[]));
        for axis in BodyAxis::ALL {
            let start = axis.index() * 2;
            attach_boundary_component(
                &mut body,
                competition_outcomes[competition_component(axis)],
                opportunities[start..start + 2].iter().copied(),
            );
        }
        let outcomes =
            std::array::from_fn(|_| attach_sensor(&mut body, Junction::integrating(1), &[]));
        for axis in BodyAxis::ALL {
            let start = axis.index() * 2;
            attach_outcome_component(
                &mut body,
                outcomes[axis.index()],
                opportunities[start..start + 2].iter().copied(),
            );
        }
        let resisted_progress =
            std::array::from_fn(|_| attach_sensor(&mut body, Junction::integrating(1), &[]));
        for axis in BodyAxis::ALL {
            let start = axis.index() * 2;
            attach_progress_component(
                &mut body,
                resisted_progress[axis.index()],
                opportunities[start..start + 2].iter().copied(),
            );
        }
        body.inputs(0, &prime).map_err(body_error)?;
        body.run(MOMENT_LIMIT, |_| {}).map_err(body_error)?;
        Ok(Self {
            body,
            handles: Handles {
                vision,
                salience,
                value,
                contacts,
                proprioception,
                exploration,
                competition_outcomes,
                outcomes,
                resisted_progress,
                opportunities,
                outward,
            },
            state: WorkstationState::default(),
            sequence: 0,
            physical_tick: 0,
            pending_transitions: [false; AXIS_COUNT],
            pending_stops: Vec::new(),
            reach_strain: [0, 0],
            vergence_strain: 0,
            history: Vec::new(),
        })
    }

    pub fn step(
        &mut self,
        sample: WorldSample,
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        self.step_with_boundary_parents(sample, &[])
    }

    pub fn step_with_boundary_parents(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        self.step_with_causal_parents(sample, boundary_parents, &[])
    }

    pub fn step_with_causal_parents(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
        progress_parents: &[MotorEffect],
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        let (next, observation) =
            self.transition_with_causal_parents(sample, boundary_parents, progress_parents)?;
        *self = next;
        Ok(observation)
    }

    /// Admits a world sample without opening a fresh chance to move. Existing
    /// retained paths may still react to the sample through ordinary physics.
    pub fn observe(
        &mut self,
        sample: WorldSample,
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        self.observe_with_causal_parents(sample, &[], &[])
    }

    pub fn observe_with_causal_parents(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
        progress_parents: &[MotorEffect],
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        sample.validate()?;
        let mut next = self.clone();
        let observation = next.step_in_place_with_trace(
            sample,
            boundary_parents,
            progress_parents,
            true,
            false,
            None,
        )?;
        *self = next;
        Ok(observation)
    }

    /// Admits returned boundary effects without ordinary sensory input or a
    /// fresh chance to move. This lets an already-caused world return settle
    /// before a checkpoint is frozen.
    pub fn settle_with_boundary_parents(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        self.settle_with_causal_parents(sample, boundary_parents, &[])
    }

    pub fn settle_with_causal_parents(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
        progress_parents: &[MotorEffect],
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        sample.validate()?;
        let mut next = self.clone();
        let observation = next.step_in_place_with_trace(
            sample,
            boundary_parents,
            progress_parents,
            false,
            false,
            None,
        )?;
        *self = next;
        Ok(observation)
    }

    /// Observer-equivalent form of [`Self::settle_with_boundary_parents`] that
    /// also records the body's physical trace.
    pub fn settle_traced_with_boundary_parents(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
    ) -> Result<(WorkstationStepObservation, Vec<BodyTraceEvent>), WorkstationError> {
        self.settle_traced_with_causal_parents(sample, boundary_parents, &[])
    }

    /// Observer-equivalent form of [`Self::settle_with_causal_parents`] that
    /// also records the body's physical trace.
    pub fn settle_traced_with_causal_parents(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
        progress_parents: &[MotorEffect],
    ) -> Result<(WorkstationStepObservation, Vec<BodyTraceEvent>), WorkstationError> {
        sample.validate()?;
        let mut next = self.clone();
        let mut trace = Vec::new();
        let observation = next.step_in_place_with_trace(
            sample,
            boundary_parents,
            progress_parents,
            false,
            false,
            Some(&mut trace),
        )?;
        *self = next;
        Ok((observation, trace))
    }

    pub fn step_traced(
        &mut self,
        sample: WorldSample,
    ) -> Result<(WorkstationStepObservation, Vec<BodyTraceEvent>), WorkstationError> {
        self.step_traced_with_boundary_parents(sample, &[])
    }

    pub fn step_traced_with_boundary_parents(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
    ) -> Result<(WorkstationStepObservation, Vec<BodyTraceEvent>), WorkstationError> {
        self.step_traced_with_causal_parents(sample, boundary_parents, &[])
    }

    pub fn step_traced_with_causal_parents(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
        progress_parents: &[MotorEffect],
    ) -> Result<(WorkstationStepObservation, Vec<BodyTraceEvent>), WorkstationError> {
        sample.validate()?;
        let mut next = self.clone();
        let mut trace = Vec::new();
        let observation = next.step_in_place_with_trace(
            sample,
            boundary_parents,
            progress_parents,
            true,
            true,
            Some(&mut trace),
        )?;
        *self = next;
        Ok((observation, trace))
    }

    pub fn transition(
        &self,
        sample: WorldSample,
    ) -> Result<(Self, WorkstationStepObservation), WorkstationError> {
        self.transition_with_boundary_parents(sample, &[])
    }

    pub fn transition_with_boundary_parents(
        &self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
    ) -> Result<(Self, WorkstationStepObservation), WorkstationError> {
        self.transition_with_causal_parents(sample, boundary_parents, &[])
    }

    pub fn transition_with_causal_parents(
        &self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
        progress_parents: &[MotorEffect],
    ) -> Result<(Self, WorkstationStepObservation), WorkstationError> {
        sample.validate()?;
        let mut next = self.clone();
        let observation = next.step_in_place(sample, boundary_parents, progress_parents)?;
        Ok((next, observation))
    }

    fn step_in_place(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
        progress_parents: &[MotorEffect],
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        self.step_in_place_with_trace(sample, boundary_parents, progress_parents, true, true, None)
    }

    fn step_in_place_with_trace(
        &mut self,
        sample: WorldSample,
        boundary_parents: &[MotorEffect],
        progress_parents: &[MotorEffect],
        admit_sensory: bool,
        admit_opportunity: bool,
        trace: Option<&mut Vec<BodyTraceEvent>>,
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        sample.validate()?;
        let state_before = self.state.clone();
        let returned_transitions = if admit_sensory {
            self.pending_axes()
        } else {
            Vec::new()
        };
        let cause = self.sequence.saturating_add(1);
        let at = self.physical_tick.saturating_add(1);
        // A joint stop met on the previous step is a completed boundary of the
        // body itself; it joins the world's boundary parents for this wave.
        let stops = std::mem::take(&mut self.pending_stops);
        let parents = boundary_parents
            .iter()
            .copied()
            .chain(stops)
            .collect::<Vec<_>>();
        let boundary_wave = self.boundary_wave(&parents);
        let sensory_at = at.saturating_add(u64::from(!boundary_wave.is_empty()));
        if !boundary_wave.is_empty() {
            self.body.inputs(at, &boundary_wave).map_err(body_error)?;
        }
        let mut first_wave = if admit_sensory {
            self.sensory_wave(&sample, cause)
        } else {
            Vec::new()
        };
        first_wave.extend(self.resisted_progress_wave(progress_parents));
        let mut returned_components = [false; OUTCOME_COMPONENTS];
        for axis in &returned_transitions {
            returned_components[outcome_component(*axis)] = true;
        }
        for (component, returned) in returned_components.into_iter().enumerate() {
            if returned {
                first_wave.push(Arrival::caused(self.handles.outcomes[component], 1, cause));
            }
        }
        if !first_wave.is_empty() {
            self.body
                .inputs(sensory_at, &first_wave)
                .map_err(body_error)?;
        }
        let opportunity_at = sensory_at.saturating_add(1);
        let opportunity_wave = if admit_opportunity {
            let exploration = self.handles.exploration
                [usize::try_from(self.sequence).unwrap_or(0) % COMPETITION_COMPONENTS];
            let mut wave = self
                .handles
                .opportunities
                .iter()
                .chain(std::iter::once(&exploration))
                .copied()
                .map(|target| Arrival::caused(target, 1, cause))
                .collect::<Vec<_>>();
            wave.extend(self.vergence_wave(&sample, cause));
            wave.extend(self.reach_depth_wave(&sample, cause));
            wave
        } else {
            Vec::new()
        };
        if !opportunity_wave.is_empty() {
            self.body
                .inputs(opportunity_at, &opportunity_wave)
                .map_err(body_error)?;
        }

        let outward = &self.handles.outward;
        let mut crossings = Vec::new();
        let mut latest = opportunity_at;
        let mut observe = |event: truelearner_body::PhysicalEvent| {
            latest = latest.max(event.at);
            if let Some((_, control)) = outward
                .iter()
                .find(|(junction, _)| *junction == event.junction)
            {
                crossings.push(MotorEffect {
                    at: event.at,
                    control: *control,
                    impulse: event
                        .impulse
                        .clamp(i64::from(i32::MIN), i64::from(i32::MAX))
                        as i32,
                    cause: event.cause,
                });
            }
        };
        let run = match trace {
            Some(trace) => self
                .body
                .run_traced(MOMENT_LIMIT, &mut observe, |event| trace.push(event)),
            None => self.body.run(MOMENT_LIMIT, &mut observe),
        }
        .map_err(body_error)?;
        self.physical_tick = latest.max(self.body.now());

        let mut frame = ActuatorFrame::default();
        for effect in &crossings {
            let effort = effect
                .impulse
                .unsigned_abs()
                .min(u32::from(BODY_MAX as u16)) as u16;
            frame.activate(effect.control.axis(), effect.control.direction(), effort);
        }
        apply_contact_reaction(&sample, &mut frame);
        apply_arm_recoil(&sample, &mut frame);
        self.apply_pre_reach(&sample, &mut frame);
        apply_ocular_drift(&sample, &self.state, &mut frame);
        let movements = self.state.integrate(frame);
        let joint_stops = joint_stops(&self.state, &crossings, &movements);
        self.pending_stops.clone_from(&joint_stops);
        self.pending_transitions = std::array::from_fn(|index| {
            movements
                .iter()
                .any(|movement| movement.axis.index() == index && movement.changed)
        });
        let pending_transitions = self.pending_axes();
        let pose_changed = !self.state.same_pose(&state_before);
        let sequence = self.sequence;
        self.sequence = self.sequence.saturating_add(1);
        if admit_sensory {
            self.history.push(sample);
        }
        let resident_bytes = self.resident_bytes();
        let body_fingerprint = self.fingerprint()?;
        let metrics = StepMetrics::from_run(
            run,
            resident_bytes,
            u64::try_from(crossings.len()).unwrap_or(u64::MAX),
        );
        Ok(WorkstationStepObservation {
            sequence,
            state_before,
            state_after: self.state.clone(),
            pose_changed,
            admitted_inputs: boundary_wave
                .len()
                .saturating_add(first_wave.len())
                .saturating_add(opportunity_wave.len()),
            opportunity_admitted: admit_opportunity,
            boundary_parents: boundary_parents.to_vec(),
            progress_parents: progress_parents.to_vec(),
            crossings,
            joint_stops,
            movements,
            returned_transitions,
            pending_transitions,
            metrics,
            naturally_quiescent: self.body.is_quiet(),
            body_fingerprint,
            physical_tick: i64::try_from(self.physical_tick).unwrap_or(i64::MAX),
        })
    }

    fn boundary_wave(&self, parents: &[MotorEffect]) -> Vec<Arrival> {
        let mut causes: [Vec<u64>; COMPETITION_COMPONENTS] = std::array::from_fn(|_| Vec::new());
        for parent in parents {
            let component = competition_component(parent.control.axis());
            if !causes[component].contains(&parent.cause) {
                causes[component].push(parent.cause);
            }
        }
        causes
            .into_iter()
            .enumerate()
            .filter_map(|(component, causes)| {
                let cause = match causes.as_slice() {
                    [] => return None,
                    [cause] => *cause,
                    _ => 0,
                };
                Some(Arrival::caused(
                    self.handles.competition_outcomes[component],
                    1,
                    cause,
                ))
            })
            .collect()
    }

    fn resisted_progress_wave(&self, parents: &[MotorEffect]) -> Vec<Arrival> {
        BodyAxis::ALL
            .into_iter()
            .filter_map(|axis| {
                let mut cause = 0;
                for parent in parents
                    .iter()
                    .filter(|parent| parent.control.axis() == axis)
                {
                    if cause == 0 {
                        cause = parent.cause;
                    } else if cause != parent.cause {
                        return None;
                    }
                }
                (cause != 0).then_some(())?;
                Some(Arrival::caused(
                    self.handles.resisted_progress[axis.index()],
                    1,
                    cause,
                ))
            })
            .collect()
    }

    fn sensory_wave(&self, sample: &WorldSample, cause: u64) -> Vec<Arrival> {
        let mut wave = Vec::with_capacity(SENSOR_COUNT);
        for eye in Eye::ALL {
            for (receptor, target) in self.handles.vision[eye.index()].iter().copied().enumerate() {
                wave.push(Arrival::caused(
                    target,
                    i32::from(sample.eye(eye).sample(receptor_position(receptor))),
                    cause,
                ));
            }
            // Salience cells fire while their receptor is lit above the
            // floor: tonic retina, not change-only. Below the floor they get
            // no arrival, so a quiet scene costs nothing.
            for (receptor, cell) in self.handles.salience[eye.index()]
                .iter()
                .copied()
                .enumerate()
            {
                let light = sample.eye(eye).sample(receptor_position(receptor));
                if light > SALIENCE_FLOOR {
                    wave.push(Arrival::caused(
                        cell,
                        i32::from(light - SALIENCE_FLOOR),
                        cause,
                    ));
                }
            }
        }
        for (site, contact) in sample.contacts().iter().copied().enumerate() {
            let [pressure, slip] = self.handles.contacts[site];
            wave.push(Arrival::caused(
                pressure,
                i32::from(contact.pressure()),
                cause,
            ));
            wave.push(Arrival::caused(slip, i32::from(contact.slip()), cause));
        }
        for sense in self.state.proprioception() {
            let [position, velocity, decrease, increase, lower, upper] =
                self.handles.proprioception[sense.axis.index()];
            wave.extend([
                Arrival::caused(position, i32::from(sense.position), cause),
                Arrival::caused(velocity, i32::from(sense.velocity), cause),
                Arrival::caused(decrease, i32::from(sense.decrease_effort), cause),
                Arrival::caused(increase, i32::from(sense.increase_effort), cause),
                Arrival::caused(lower, i32::from(sense.at_lower_limit), cause),
                Arrival::caused(upper, i32::from(sense.at_upper_limit), cause),
            ]);
        }
        debug_assert!(wave.len() <= SENSOR_COUNT);
        wave
    }

    /// The world position of the brightness-weighted centroid of what this
    /// eye sees above the salience floor, or `None` when the eye sees only
    /// dim light or its own hand. Each lit receptor votes with its
    /// brightness above the floor, so a brighter patch pulls harder than a
    /// dim one and two distinct things select the brighter instead of
    /// averaging into the empty space between them. The organism's own
    /// palm never counts as a target.
    fn reach_target(&self, sample: &WorldSample, eye: Eye) -> Option<(i32, i32)> {
        let gaze = self.state.eye(eye).gaze();
        let field = sample.eye(eye);
        let center = receptor_position(RECEPTOR_SIDE * 4 + 4);
        let mut sum_x = 0_i64;
        let mut sum_y = 0_i64;
        let mut weight_sum = 0_i64;
        for receptor in 0..RECEPTORS_PER_EYE {
            let light = field.sample(receptor_position(receptor));
            if light < SALIENCE_FLOOR {
                continue;
            }
            let weight = i64::from(light - SALIENCE_FLOOR);
            let position = receptor_position(receptor);
            let world_x = i32::from(gaze.x()) + i32::from(position.x()) - i32::from(center.x());
            let world_y = i32::from(gaze.y()) + i32::from(position.y()) - i32::from(center.y());
            if self.own_hand_at(world_x, world_y) {
                continue;
            }
            sum_x += i64::from(world_x) * weight;
            sum_y += i64::from(world_y) * weight;
            weight_sum += weight;
        }
        (weight_sum > 0).then(|| ((sum_x / weight_sum) as i32, (sum_y / weight_sum) as i32))
    }

    fn own_hand_at(&self, x: i32, y: i32) -> bool {
        let palm = self.state.hand().palm();
        (i32::from(palm.x()) - x).abs() <= REACH_HAND_MASK
            && (i32::from(palm.y()) - y).abs() <= REACH_HAND_MASK
    }

    /// The pre-reach depth extension: while the eyes see a salient target
    /// and the palm is not in contact, the arm extends toward what is seen.
    /// This is infant pre-reaching — a seen thing invites arm extension —
    /// computed from the organism's own retina and touch alone. Contact
    /// terminates it: the screen's resistance stops the push, exactly as
    /// balance terminates orienting, so the pulse never fights a surface.
    fn reach_depth_wave(&self, sample: &WorldSample, cause: u64) -> Vec<Arrival> {
        if sample
            .contacts()
            .iter()
            .any(|contact| contact.pressure() > 0)
        {
            return Vec::new();
        }
        let sees_target = Eye::ALL
            .into_iter()
            .any(|eye| self.reach_target(sample, eye).is_some());
        if !sees_target {
            return Vec::new();
        }
        vec![Arrival::caused(
            self.handles.opportunities[BodyAxis::PalmDepth.index() * 2 + 1],
            2,
            cause,
        )]
    }

    /// The yoked vergence controller: when both eyes see salient pulls in
    /// opposing horizontal directions — the vergence geometry — both eyes'
    /// horizontal opportunities are pulsed in the same step, every step,
    /// until each target sits within one receptor pitch of its fovea. Real
    /// vergence is brainstem-yoked: the eyes converge together or not at
    /// all, and the choice machinery would otherwise serialize them. The
    /// pulse shares the opportunity wave's cause, so each eye crosses
    /// without waiting for a choice. A single centered or shared target
    /// commands nothing here, so ordinary gaze exploration stays with the
    /// learner.
    fn vergence_wave(&mut self, sample: &WorldSample, cause: u64) -> Vec<Arrival> {
        let mut aims = [0_i32; 2];
        let mut fused = true;
        for eye in Eye::ALL {
            let Some(target) = self.reach_target(sample, eye) else {
                continue;
            };
            // Any horizontal misalignment counts: the stereo course places
            // its far targets a single receptor pitch off-center, and real
            // vergence tracks to exact coincidence.
            let dx = target.0 - i32::from(self.state.eye(eye).gaze().x());
            if dx != 0 {
                aims[eye.index()] = dx.signum();
                fused = false;
            }
        }
        // Insistent vergence: while the vergence error persists, the pulse
        // grows each step, so no learned habit can stalemate the fusion —
        // the same strain law as the pre-reach. Fused or absent, the strain
        // resets.
        if fused {
            self.vergence_strain = 0;
            return Vec::new();
        }
        if self.vergence_strain >= 32 {
            self.vergence_strain = 32;
        } else {
            self.vergence_strain += 1;
        }
        let mut wave = Vec::new();
        if aims[0] != 0 && aims[0] == -aims[1] {
            for eye in Eye::ALL {
                let direction = usize::from(aims[eye.index()] > 0);
                wave.push(Arrival::caused(
                    self.handles.opportunities
                        [BodyAxis::EyeHorizontal { eye }.index() * 2 + direction],
                    2_i32.saturating_add(self.vergence_strain.min(28)),
                    cause,
                ));
            }
        }
        wave
    }

    /// The pre-reach as an equilibrium-point shift: the arm's resting
    /// posture moves toward the salience centroid of what the eyes see.
    /// Like the ocular drift and the arm recoil, this is frame-level
    /// physics, not a chosen crossing — the learner's habits still sum
    /// their effort against it, and a sustained aim recruits more drive
    /// each step, like an infant straining toward a toy, so no fixed habit
    /// can stalemate the reach. The strain is reflex machinery: it resets
    /// when the aim clears or flips.
    fn apply_pre_reach(&mut self, sample: &WorldSample, frame: &mut ActuatorFrame) {
        let palm = self.state.hand().palm();
        let mut horizontal = 0_i32;
        let mut vertical = 0_i32;
        for eye in Eye::ALL {
            let Some(target) = self.reach_target(sample, eye) else {
                continue;
            };
            let dx = target.0 - i32::from(palm.x());
            let dy = target.1 - i32::from(palm.y());
            if dx.abs() > REACH_DEADZONE {
                horizontal += dx.signum();
            }
            if dy.abs() > REACH_DEADZONE {
                vertical += dy.signum();
            }
        }
        let aims = [horizontal.clamp(-1, 1), vertical.clamp(-1, 1)];
        for (index, axis) in [BodyAxis::PalmHorizontal, BodyAxis::PalmVertical]
            .into_iter()
            .enumerate()
        {
            if aims[index] == 0 {
                self.reach_strain[index] = 0;
                continue;
            }
            let flipped = self.reach_strain[index].signum() != 0
                && self.reach_strain[index].signum() != aims[index];
            if flipped {
                self.reach_strain[index] = aims[index];
            } else {
                self.reach_strain[index] = self.reach_strain[index]
                    .saturating_add(aims[index])
                    .clamp(-32, 32);
            }
            let direction = if aims[index] > 0 {
                Direction::Increase
            } else {
                Direction::Decrease
            };
            frame.activate(
                axis,
                direction,
                4_i32.saturating_add(self.reach_strain[index].abs().min(28)) as u16,
            );
        }
    }

    pub const fn state(&self) -> &WorkstationState {
        &self.state
    }

    pub fn read(&self) -> Result<WorkstationRead, WorkstationError> {
        Ok(WorkstationRead {
            state: self.state.clone(),
            body_fingerprint: self.fingerprint()?,
            physical_tick: i64::try_from(self.physical_tick).unwrap_or(i64::MAX),
            return_path_count: 0,
            resident_bytes: self.resident_bytes(),
            pending_transitions: self.pending_axes(),
        })
    }

    /// Observer-only accounting for repeated closed physical composition.
    /// This value is never fed back into action selection.
    pub fn automaticity_work(&self) -> AutomaticityWork {
        self.body.automaticity_work()
    }

    /// Maps an observer trace's strengthened link back to the receptor
    /// whose value link it is, if it is one: `(eye, receptor)`. Observer
    /// only — it changes nothing, so a trace can name where the learner
    /// wrote value onto salience.
    pub fn receptor_for_value_link(&self, link: LinkId) -> Option<(Eye, usize)> {
        for eye in Eye::ALL {
            for receptor in 0..RECEPTORS_PER_EYE {
                if self.handles.value_link(eye, receptor) == Some(link) {
                    return Some((eye, receptor));
                }
            }
        }
        None
    }

    /// Maps an observer trace's outward junction back to the physical control
    /// owned by this harness. It does not change the body or admit input.
    pub fn control_for_trace_output(&self, output: JunctionId) -> Option<BodyControl> {
        self.handles
            .opportunities
            .iter()
            .position(|junction| *junction == output)
            .and_then(|index| self.handles.outward.get(index).map(|(_, control)| *control))
            .or_else(|| {
                self.handles
                    .outward
                    .iter()
                    .find_map(|(junction, control)| (*junction == output).then_some(*control))
            })
    }

    pub fn save(&self) -> Result<WorkstationCheckpoint, WorkstationError> {
        if !self.body.is_quiet() {
            return Err(WorkstationError::InvalidCheckpoint);
        }
        let body = self
            .body
            .checkpoint()
            .and_then(|checkpoint| checkpoint.canonical_bytes())
            .map_err(body_checkpoint_error)?;
        Ok(WorkstationCheckpoint::new(
            body,
            self.handles.clone(),
            self.state.clone(),
            self.sequence,
            self.physical_tick,
            self.pending_transitions,
            self.pending_stops.clone(),
            self.reach_strain,
            self.vergence_strain,
            self.history.clone(),
        ))
    }

    pub fn restore(checkpoint: WorkstationCheckpoint) -> Result<Self, WorkstationError> {
        let payload = checkpoint.open();
        let body = BodyCheckpoint::decode(&payload.body)
            .and_then(BodyCheckpoint::restore)
            .map_err(body_checkpoint_error)?;
        if !payload.handles.valid_for(&body) {
            return Err(WorkstationError::InvalidCheckpoint);
        }
        Ok(Self {
            body,
            handles: payload.handles,
            state: payload.state,
            sequence: payload.sequence,
            physical_tick: payload.physical_tick,
            pending_transitions: payload.pending_transitions,
            pending_stops: payload.pending_stops,
            reach_strain: payload.reach_strain,
            vergence_strain: payload.vergence_strain,
            history: payload.history,
        })
    }

    /// Repositions the physical workstation body to a frozen reference pose
    /// while preserving the learned body, causal time, and durable topology.
    /// The next admitted sample makes the intervention physically observable.
    pub fn reposition_from_checkpoint(
        &mut self,
        checkpoint: &WorkstationCheckpoint,
    ) -> Result<(), WorkstationError> {
        if !self.body.is_quiet() {
            return Err(WorkstationError::InvalidCheckpoint);
        }
        let reference = checkpoint.clone().open();
        self.state = reference.state;
        self.pending_transitions = reference.pending_transitions;
        self.pending_stops = reference.pending_stops;
        self.reach_strain = reference.reach_strain;
        self.vergence_strain = reference.vergence_strain;
        self.history = reference.history;
        Ok(())
    }

    /// Applies a small external displacement to a quiet body. No learner
    /// output participates, so the next sensory sample witnesses the change
    /// without attributing it to an organism action.
    pub fn perturb_body(
        &mut self,
        control: BodyControl,
        impulse: u16,
    ) -> Result<bool, WorkstationError> {
        if !self.body.is_quiet() || impulse == 0 {
            return Err(WorkstationError::InvalidCheckpoint);
        }
        let mut frame = ActuatorFrame::default();
        frame.activate(control.axis(), control.direction(), impulse);
        Ok(self
            .state
            .integrate(frame)
            .into_iter()
            .any(|movement| movement.changed))
    }

    fn pending_axes(&self) -> Vec<BodyAxis> {
        BodyAxis::ALL
            .into_iter()
            .filter(|axis| self.pending_transitions[axis.index()])
            .collect()
    }

    fn resident_bytes(&self) -> usize {
        std::mem::size_of::<Self>().saturating_add(
            self.history
                .iter()
                .map(|sample| bincode::serialized_size(sample).unwrap_or(0) as usize)
                .sum::<usize>(),
        )
    }

    fn fingerprint(&self) -> Result<String, WorkstationError> {
        let mut digest = Sha256::new();
        digest.update(b"truelearner-compact-workstation-v1");
        digest.update(
            bincode::serialize(&self.history).map_err(|_| WorkstationError::InvalidCheckpoint)?,
        );
        digest.update(
            bincode::serialize(&self.state).map_err(|_| WorkstationError::InvalidCheckpoint)?,
        );
        Ok(digest
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }
}

/// A crossing that pushes an axis against its own joint stop moves nothing.
/// The stop is a one-sided reaction of the body, and that crossing is the
/// exact parent of the completed boundary.
fn joint_stops(
    state: &WorkstationState,
    crossings: &[MotorEffect],
    movements: &[BodyMovement],
) -> Vec<MotorEffect> {
    crossings
        .iter()
        .copied()
        .filter(|crossing| {
            let axis = crossing.control.axis();
            let moved = movements
                .iter()
                .any(|movement| movement.axis == axis && movement.changed);
            let (at_lower_limit, at_upper_limit) = state.limits(axis);
            !moved
                && match crossing.control.direction() {
                    Direction::Decrease => at_lower_limit,
                    Direction::Increase => at_upper_limit,
                }
        })
        .collect()
}

fn apply_contact_reaction(sample: &WorldSample, frame: &mut ActuatorFrame) {
    if sample
        .contacts()
        .iter()
        .any(|contact| contact.pressure() == BODY_MAX as u16)
    {
        frame.resist_increase(BodyAxis::PalmDepth, BODY_MAX as u16);
    }
}

/// The arm is elastic: with the extension drive stopped by contact, the
/// un-driven arm recoils toward its resting length. Press, lift, press
/// again — the tap cycle is the equilibrium of the extension reflex and
/// arm elasticity, not a learned trick. Like the surface reaction this is
/// frame-level physics: it crosses nothing and strengthens nothing.
fn apply_arm_recoil(sample: &WorldSample, frame: &mut ActuatorFrame) {
    if sample
        .contacts()
        .iter()
        .any(|contact| contact.pressure() > 0)
    {
        frame.activate(BodyAxis::PalmDepth, Direction::Decrease, 1);
    }
}

/// The gaze integrator leaks: when the whole retina is dark — the eyes
/// staring past the screen's edge — an external drift carries the eyes back
/// toward the primary position, straight ahead. Like the contact reaction,
/// this is frame-level physics, not a chosen crossing, so it returns no
/// consequence and strengthens nothing. Any salience makes the foveation
/// reflex hold the eye, so lit scenes never drift.
fn apply_ocular_drift(sample: &WorldSample, state: &WorkstationState, frame: &mut ActuatorFrame) {
    let dark = Eye::ALL.into_iter().all(|eye| {
        (0..RECEPTORS_PER_EYE)
            .all(|receptor| sample.eye(eye).sample(receptor_position(receptor)) <= SALIENCE_FLOOR)
    });
    if !dark {
        return;
    }
    let center = (BODY_MAX + 1) / 2;
    for eye in Eye::ALL {
        let gaze = state.eye(eye).gaze();
        let mut push = |axis: BodyAxis, position: i16| {
            if position > center {
                frame.activate(axis, Direction::Decrease, 8);
            } else if position < center {
                frame.activate(axis, Direction::Increase, 8);
            }
        };
        push(BodyAxis::EyeHorizontal { eye }, gaze.x());
        push(BodyAxis::EyeVertical { eye }, gaze.y());
    }
}

const fn control(axis: BodyAxis, direction: Direction) -> BodyControl {
    BodyControl::new(axis, direction)
}

const fn outcome_component(axis: BodyAxis) -> usize {
    axis.index()
}

const fn competition_component(axis: BodyAxis) -> usize {
    match axis {
        BodyAxis::EyeHorizontal { eye: Eye::Left } | BodyAxis::EyeVertical { eye: Eye::Left } => 0,
        BodyAxis::EyeHorizontal { eye: Eye::Right } | BodyAxis::EyeVertical { eye: Eye::Right } => {
            1
        }
        BodyAxis::PalmHorizontal | BodyAxis::PalmVertical => 2,
        BodyAxis::PalmDepth => 3,
    }
}

const fn total_work(work: Work) -> u64 {
    work.arrivals
        .saturating_add(work.meetings)
        .saturating_add(work.changes)
        .saturating_add(work.link_visits)
        .saturating_add(work.emissions)
}

fn body_error(error: truelearner_body::RunError) -> WorkstationError {
    error.into()
}

fn body_checkpoint_error(error: BodyCheckpointError) -> WorkstationError {
    error.into()
}

fn attach_sampled_sensor(
    body: &mut Body,
    range: u32,
    nearby: &[(JunctionId, u64)],
    prime: &mut Vec<Arrival>,
) -> JunctionId {
    let sensor = attach_sensor(body, Junction::sampled_in(SENSOR_LIFETIME, range), nearby);
    prime.push(Arrival::new(sensor, SENSOR_PRIME));
    sensor
}

/// The eye opportunities a salient receptor commands: the movements that
/// move gaze toward that receptor. The exact foveal center commands
/// nothing, so a centered stimulus leaves the eye still.
fn eye_foveation_drive(opportunities: &[JunctionId], eye: Eye, receptor: usize) -> Vec<JunctionId> {
    let column = receptor % RECEPTOR_SIDE;
    let row = receptor / RECEPTOR_SIDE;
    let center = RECEPTOR_SIDE / 2;
    let mut drive = Vec::with_capacity(2);
    let mut push = |axis: BodyAxis, position: usize| match position.cmp(&center) {
        std::cmp::Ordering::Less => drive.push(opportunities[axis.index() * 2]),
        std::cmp::Ordering::Greater => drive.push(opportunities[axis.index() * 2 + 1]),
        std::cmp::Ordering::Equal => {}
    };
    push(BodyAxis::EyeHorizontal { eye }, column);
    push(BodyAxis::EyeVertical { eye }, row);
    drive
}

/// A retinal receptor is near its displaced eye axes, and near planar palm
/// transport in both directions. The eye nearness is direction-specific
/// because a receptor's position names the eye movement that centers it.
/// Palm transport gets both directions equally: a lit patch says where the
/// palm could go relative to the gaze, but only a consequence can say which
/// way, because the palm and the gaze move independently. Every join here
/// carries zero impulse, so this is a learnable path, never a driven one.
fn eye_palm_nearness(
    opportunities: &[JunctionId],
    eye: Eye,
    receptor: usize,
) -> Vec<(JunctionId, u64)> {
    let mut nearby = eye_nearness(opportunities, eye, receptor);
    nearby.extend(axis_nearness(opportunities, BodyAxis::PalmHorizontal));
    nearby.extend(axis_nearness(opportunities, BodyAxis::PalmVertical));
    nearby
}

const fn axis_range(_axis: BodyAxis) -> u32 {
    BODY_RANGE
}

fn axis_nearness(opportunities: &[JunctionId], axis: BodyAxis) -> Vec<(JunctionId, u64)> {
    let start = axis.index() * 2;
    vec![(opportunities[start], 1), (opportunities[start + 1], 1)]
}

fn eye_nearness(opportunities: &[JunctionId], eye: Eye, receptor: usize) -> Vec<(JunctionId, u64)> {
    let mut nearby = Vec::with_capacity(4);
    let column = receptor % RECEPTOR_SIDE;
    let row = receptor / RECEPTOR_SIDE;
    let center = RECEPTOR_SIDE / 2;
    // Preserve neutral four-way exploration only at the exact foveal center.
    if column == center && row == center {
        for axis in [
            BodyAxis::EyeHorizontal { eye },
            BodyAxis::EyeVertical { eye },
        ] {
            let start = axis.index() * 2;
            nearby.extend([(opportunities[start], 1), (opportunities[start + 1], 1)]);
        }
        return nearby;
    }
    extend_directional_nearness(
        &mut nearby,
        opportunities,
        BodyAxis::EyeHorizontal { eye },
        column,
    );
    extend_directional_nearness(
        &mut nearby,
        opportunities,
        BodyAxis::EyeVertical { eye },
        row,
    );
    nearby
}

fn extend_directional_nearness(
    nearby: &mut Vec<(JunctionId, u64)>,
    opportunities: &[JunctionId],
    axis: BodyAxis,
    position: usize,
) {
    let center = RECEPTOR_SIDE / 2;
    let mut push = |direction| {
        let offset = usize::from(direction == Direction::Increase);
        nearby.push((opportunities[axis.index() * 2 + offset], 1));
    };
    match position.cmp(&center) {
        std::cmp::Ordering::Less => push(Direction::Decrease),
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Greater => push(Direction::Increase),
    }
}

/// The one contact surface of the undifferentiated hand: its pressure is
/// near palm depth, its slip is near planar palm transport.
fn contact_pressure_nearness(opportunities: &[JunctionId]) -> Vec<(JunctionId, u64)> {
    axis_nearness(opportunities, BodyAxis::PalmDepth)
}

fn contact_slip_nearness(opportunities: &[JunctionId]) -> Vec<(JunctionId, u64)> {
    [BodyAxis::PalmHorizontal, BodyAxis::PalmVertical]
        .into_iter()
        .flat_map(|axis| axis_nearness(opportunities, axis))
        .collect()
}

fn receptor_position(receptor: usize) -> Point {
    let column = receptor % RECEPTOR_SIDE;
    let row = receptor / RECEPTOR_SIDE;
    let coordinate = |index: usize| {
        let numerator = index * BODY_MAX as usize + RECEPTOR_SIDE - 2;
        i16::try_from(numerator / (RECEPTOR_SIDE - 1)).expect("receptor position is bounded")
    };
    Point::new(coordinate(column), coordinate(row)).expect("receptor position is bounded")
}

#[cfg(test)]
fn sensory_values(wave: &[Arrival]) -> Vec<i32> {
    wave.iter().map(|arrival| arrival.impulse).collect()
}

#[cfg(test)]
fn visual_range(eye: Eye) -> std::ops::Range<usize> {
    let start = eye.index() * RECEPTORS_PER_EYE;
    start..start + RECEPTORS_PER_EYE
}

#[cfg(test)]
fn contact_range() -> std::ops::Range<usize> {
    VISUAL_SENSOR_COUNT..VISUAL_SENSOR_COUNT + TOUCH_SITES * CONTACT_FIELDS
}

#[cfg(test)]
fn proprioception_range() -> std::ops::Range<usize> {
    let start = VISUAL_SENSOR_COUNT + TOUCH_SITES * CONTACT_FIELDS;
    start..start + AXIS_COUNT * PROPRIOCEPTIVE_FIELDS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContactSample, LightField, TOUCH_SITES};
    use truelearner_body::ChoiceWarrant;

    fn sample() -> WorldSample {
        WorldSample::new(
            [
                LightField::filled(3, 3, 1).unwrap(),
                LightField::filled(3, 3, 2).unwrap(),
            ],
            [ContactSample::default(); TOUCH_SITES],
        )
        .unwrap()
    }

    fn field(value: u8) -> LightField {
        LightField::filled(9, 9, value).unwrap()
    }

    fn centered(value: u8) -> LightField {
        let mut pixels = vec![0; 9 * 9];
        pixels[4 * 9 + 4] = value;
        LightField::new(9, 9, pixels).unwrap()
    }

    fn with_fields(
        left: LightField,
        right: LightField,
        contacts: [ContactSample; TOUCH_SITES],
    ) -> WorldSample {
        WorldSample::new([left, right], contacts).unwrap()
    }

    #[test]
    fn only_maximal_support_pressure_cancels_inward_depth() {
        let axis = BodyAxis::PalmDepth;
        let movement_at_pressure = |pressure| {
            let mut contacts = [ContactSample::default(); TOUCH_SITES];
            contacts[0] = ContactSample::new(pressure, 0).unwrap();
            let sample = with_fields(field(0), field(0), contacts);
            let mut frame = ActuatorFrame::default();
            frame.activate(axis, Direction::Increase, 7);
            apply_contact_reaction(&sample, &mut frame);
            WorkstationState::default().integrate(frame)
        };

        let soft = movement_at_pressure(1);
        assert!(soft[0].changed);
        assert_eq!(soft[0].net_impulse, 7);

        let rigid = movement_at_pressure(BODY_MAX as u16);
        assert!(!rigid[0].changed);
        assert_eq!(rigid[0].decrease_effort, 7);
        assert_eq!(rigid[0].increase_effort, 7);
        assert_eq!(rigid[0].net_impulse, 0);
    }

    #[test]
    fn target_intensity_is_an_ordinary_local_reading() {
        let harness = WorkstationHarness::new(1).unwrap();
        let high = harness.sensory_wave(
            &with_fields(
                centered(255),
                field(0),
                [ContactSample::default(); TOUCH_SITES],
            ),
            1,
        );
        let lower = harness.sensory_wave(
            &with_fields(
                centered(254),
                field(0),
                [ContactSample::default(); TOUCH_SITES],
            ),
            1,
        );

        assert_eq!(
            high.iter()
                .map(|arrival| arrival.target)
                .collect::<Vec<_>>(),
            lower
                .iter()
                .map(|arrival| arrival.target)
                .collect::<Vec<_>>()
        );
        let differences = sensory_values(&high)
            .into_iter()
            .zip(sensory_values(&lower))
            .enumerate()
            .filter_map(|(index, (left, right))| (left != right).then_some(index))
            .collect::<Vec<_>>();
        // The local vision reading and the local salience feed both change,
        // by the same local amount: the tonic channel is an ordinary local
        // transduction of the same pixel, not a second sensor.
        assert_eq!(differences, [RECEPTORS_PER_EYE / 2, RECEPTORS_PER_EYE]);
    }

    #[test]
    fn progress_requires_one_explicit_causal_parent_per_axis() {
        let harness = WorkstationHarness::new(1).unwrap();
        let parent = MotorEffect {
            at: 7,
            control: BodyControl::new(BodyAxis::PalmDepth, Direction::Increase),
            impulse: 1,
            cause: 41,
        };

        assert_eq!(
            harness.resisted_progress_wave(&[parent]),
            [Arrival::caused(
                harness.handles.resisted_progress[BodyAxis::PalmDepth.index()],
                1,
                41,
            )]
        );
        assert!(harness
            .resisted_progress_wave(&[
                parent,
                MotorEffect {
                    cause: 42,
                    ..parent
                },
            ])
            .is_empty());
        assert!(harness.resisted_progress_wave(&[]).is_empty());
    }

    #[test]
    fn palm_depth_and_planar_transport_are_independent_products() {
        assert_ne!(
            competition_component(BodyAxis::PalmDepth),
            competition_component(BodyAxis::PalmHorizontal)
        );
    }

    #[test]
    fn the_one_contact_surface_does_not_bridge_depth_and_planar_products() {
        let harness = WorkstationHarness::new(1).unwrap();
        let pressure = contact_pressure_nearness(&harness.handles.opportunities);
        let slip = contact_slip_nearness(&harness.handles.opportunities);
        let reaches = |nearby: &[(JunctionId, u64)], axis: BodyAxis| {
            let start = axis.index() * 2;
            harness.handles.opportunities[start..start + 2]
                .iter()
                .any(|junction| nearby.iter().any(|(target, _)| target == junction))
        };

        assert!(reaches(&pressure, BodyAxis::PalmDepth));
        assert!(!reaches(&pressure, BodyAxis::PalmHorizontal));
        assert!(!reaches(&pressure, BodyAxis::PalmVertical));
        assert!(!reaches(&slip, BodyAxis::PalmDepth));
        assert!(reaches(&slip, BodyAxis::PalmHorizontal));
        assert!(reaches(&slip, BodyAxis::PalmVertical));
    }

    #[test]
    fn the_external_clock_explores_one_physical_component_at_a_time() {
        let stable = with_fields(field(0), field(0), [ContactSample::default(); TOUCH_SITES]);
        for expected in 0..COMPETITION_COMPONENTS {
            let mut harness = WorkstationHarness::new(1).unwrap();
            harness.observe(stable.clone()).unwrap();
            while usize::try_from(harness.sequence).unwrap() % COMPETITION_COMPONENTS != expected {
                let quiet = harness.observe(stable.clone()).unwrap();
                assert!(quiet.crossings.is_empty());
            }
            let explored = harness.step(stable.clone()).unwrap();
            assert_eq!(
                explored
                    .crossings
                    .iter()
                    .filter(|effect| competition_component(effect.control.axis()) == expected)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn returned_external_exploration_reaches_planar_hand_transport() {
        let mut harness = WorkstationHarness::new(1).unwrap();
        let stable = with_fields(field(0), field(0), [ContactSample::default(); TOUCH_SITES]);
        harness.observe(stable.clone()).unwrap();

        let mut controls = Vec::new();
        for _ in 0..CONTROL_COUNT {
            controls.extend(
                harness
                    .step(stable.clone())
                    .unwrap()
                    .crossings
                    .into_iter()
                    .map(|effect| effect.control),
            );
        }

        assert!(controls.iter().any(|control| matches!(
            control.axis(),
            BodyAxis::PalmHorizontal | BodyAxis::PalmVertical
        )));
    }

    #[test]
    fn returned_external_exploration_reaches_palm_depth_increase() {
        let mut harness = WorkstationHarness::new(1).unwrap();
        let stable = with_fields(field(0), field(0), [ContactSample::default(); TOUCH_SITES]);
        harness.observe(stable.clone()).unwrap();

        let controls = (0..CONTROL_COUNT)
            .flat_map(|_| harness.step(stable.clone()).unwrap().crossings)
            .map(|effect| effect.control)
            .collect::<Vec<_>>();

        assert!(controls.contains(&BodyControl::new(BodyAxis::PalmDepth, Direction::Increase,)));
    }

    #[test]
    fn a_crossing_into_a_joint_stop_releases_its_antagonist() {
        let mut harness = WorkstationHarness::new(1).unwrap();
        let stable = with_fields(field(0), field(0), [ContactSample::default(); TOUCH_SITES]);
        harness.observe(stable.clone()).unwrap();
        let increase = BodyControl::new(BodyAxis::PalmHorizontal, Direction::Increase);
        let decrease = BodyControl::new(BodyAxis::PalmHorizontal, Direction::Decrease);
        // The palm is rate-limited, so an external push reaches the joint
        // stop over several perturbations.
        for _ in 0..32 {
            harness.perturb_body(increase, 64).unwrap();
        }
        assert_eq!(harness.state().hand().palm().x(), BODY_MAX);

        // Find the first outward push that meets the joint stop without moving.
        let mut stop = None;
        for _ in 0..CONTROL_COUNT * 4 {
            let observation = harness.step(stable.clone()).unwrap();
            let pushed = observation
                .crossings
                .iter()
                .find(|effect| effect.control == increase)
                .copied();
            if observation.state_before.hand().palm().x() == BODY_MAX
                && observation.state_after.hand().palm().x() == BODY_MAX
                && pushed.is_some()
            {
                assert_eq!(
                    observation.joint_stops,
                    pushed.into_iter().collect::<Vec<_>>()
                );
                stop = pushed;
                break;
            }
            assert!(observation.joint_stops.is_empty());
        }
        let stop = stop.expect("the palm pushes into its joint stop");

        // The stop is a completed boundary with that exact push as parent, so
        // the next wave releases the palm's antagonist as a returned consequence.
        let (observation, trace) = harness.step_traced(stable).unwrap();
        let released = trace.iter().any(|event| match event {
            BodyTraceEvent::Choice(choice) => {
                choice.warrant == Some(ChoiceWarrant::ReturnedConsequence)
                    && choice
                        .winner
                        .and_then(|winner| harness.control_for_trace_output(winner.output))
                        == Some(decrease)
            }
            _ => false,
        });
        assert!(
            released,
            "no returned-consequence release after stop {stop:?}"
        );
        assert!(observation
            .crossings
            .iter()
            .any(|effect| effect.control == decrease));
        assert!(observation.state_after.hand().palm().x() < BODY_MAX);
    }

    #[test]
    fn retinal_position_is_near_only_its_displaced_eye_axes() {
        let harness = WorkstationHarness::new(1).unwrap();
        let opportunities = &harness.handles.opportunities;
        let nearby = |axis: BodyAxis, direction: Direction| {
            let offset = usize::from(direction == Direction::Increase);
            (opportunities[axis.index() * 2 + offset], 1)
        };
        let horizontal = BodyAxis::EyeHorizontal { eye: Eye::Left };
        let vertical = BodyAxis::EyeVertical { eye: Eye::Left };
        let left = eye_nearness(opportunities, Eye::Left, RECEPTOR_SIDE * 4);
        let upper = eye_nearness(opportunities, Eye::Left, 4);
        let center = eye_nearness(opportunities, Eye::Left, RECEPTOR_SIDE * 4 + 4);
        let lower_right = eye_nearness(opportunities, Eye::Left, RECEPTORS_PER_EYE - 1);

        assert_eq!(left, vec![nearby(horizontal, Direction::Decrease)]);
        assert_eq!(upper, vec![nearby(vertical, Direction::Decrease)]);
        assert_eq!(
            center,
            vec![
                nearby(horizontal, Direction::Decrease),
                nearby(horizontal, Direction::Increase),
                nearby(vertical, Direction::Decrease),
                nearby(vertical, Direction::Increase),
            ]
        );
        assert_eq!(
            lower_right,
            vec![
                nearby(horizontal, Direction::Increase),
                nearby(vertical, Direction::Increase),
            ]
        );
    }

    #[test]
    fn retinal_receptors_reach_palm_transport_in_both_directions() {
        // The missing physics for aimed reaching, isolated to bare structure:
        // a lit patch on the retina must be a learnable candidate for steering
        // planar palm transport, in both directions equally. The eye axes stay
        // direction-specific; only the consequence can teach the palm which
        // way, because the palm and the gaze move independently.
        let harness = WorkstationHarness::new(1).unwrap();
        let opportunities = &harness.handles.opportunities;
        let nearby = |axis: BodyAxis, direction: Direction| {
            let offset = usize::from(direction == Direction::Increase);
            (opportunities[axis.index() * 2 + offset], 1)
        };
        for receptor in 0..RECEPTORS_PER_EYE {
            let near = eye_palm_nearness(opportunities, Eye::Left, receptor);
            let eye_axes = eye_nearness(opportunities, Eye::Left, receptor);
            assert!(near.starts_with(&eye_axes));
            for axis in [BodyAxis::PalmHorizontal, BodyAxis::PalmVertical] {
                for direction in [Direction::Decrease, Direction::Increase] {
                    assert!(
                        near.contains(&nearby(axis, direction)),
                        "receptor {receptor} does not reach {axis:?} {direction:?}"
                    );
                }
            }
        }
    }

    /// The field one eye would see from a bright vertical bar fixed at
    /// `world_x`, given the current gaze: the bar rides the retina as gaze
    /// moves, exactly as a rendered world does. `band` is the bar's
    /// brightness, so a dim mover can be drawn below the salience floor.
    fn bar_field(gaze: Point, world_x: i16, band: u8) -> LightField {
        let center = receptor_position(RECEPTOR_SIDE * 4 + 4);
        let mut pixels = vec![0_u8; RECEPTORS_PER_EYE];
        for (receptor, pixel) in pixels.iter_mut().enumerate() {
            let position = receptor_position(receptor);
            let here = gaze.x() + position.x() - center.x();
            if (here - world_x).abs() <= 128 {
                *pixel = band;
            }
        }
        LightField::new(RECEPTOR_SIDE as u16, RECEPTOR_SIDE as u16, pixels).unwrap()
    }

    #[test]
    fn a_static_off_fovea_bar_pulls_gaze_from_rest() {
        // The binocular probe failure isolated to bare physics: with a purely
        // change-driven retina, a resting body on a static scene is blind, so
        // a persistently off-fovea target can sit there forever. The tonic
        // salience channel and the foveation arc must pull gaze to the bar
        // from rest, with the learner otherwise occupied with the hand.
        let mut harness = WorkstationHarness::new(1).unwrap();
        let bar_x = 896_i16;
        let bar = |gaze: Point| bar_field(gaze, bar_x, 230);
        harness
            .observe(with_fields(
                bar(harness.state().eye(Eye::Left).gaze()),
                bar(harness.state().eye(Eye::Right).gaze()),
                [ContactSample::default(); TOUCH_SITES],
            ))
            .unwrap();
        for _ in 0..96 {
            let gaze_left = harness.state().eye(Eye::Left).gaze();
            let gaze_right = harness.state().eye(Eye::Right).gaze();
            harness
                .step(with_fields(
                    bar(gaze_left),
                    bar(gaze_right),
                    [ContactSample::default(); TOUCH_SITES],
                ))
                .unwrap();
        }
        let gaze = harness.state().eye(Eye::Left).gaze().x();
        assert!(
            (gaze - bar_x).abs() <= 128,
            "gaze {gaze} never foveated the static bar at {bar_x}"
        );
    }

    #[test]
    fn a_foveated_bar_is_quiet_by_balance() {
        // Termination is balance, not a constant: a centered bar drives both
        // directions equally, so the eyes settle still and stay there.
        let mut harness = WorkstationHarness::new(1).unwrap();
        let bar = |gaze: Point| bar_field(gaze, gaze.x(), 230);
        harness
            .observe(with_fields(
                bar(harness.state().eye(Eye::Left).gaze()),
                bar(harness.state().eye(Eye::Right).gaze()),
                [ContactSample::default(); TOUCH_SITES],
            ))
            .unwrap();
        for _ in 0..64 {
            let gaze_left = harness.state().eye(Eye::Left).gaze();
            let gaze_right = harness.state().eye(Eye::Right).gaze();
            harness
                .step(with_fields(
                    bar(gaze_left),
                    bar(gaze_right),
                    [ContactSample::default(); TOUCH_SITES],
                ))
                .unwrap();
        }
        let gaze = harness.state().eye(Eye::Left).gaze().x();
        let drift = (gaze - 512).abs();
        assert!(drift <= 128, "a centered bar drifted {drift} away");
    }

    #[test]
    fn below_the_salience_floor_the_reflexes_receive_nothing() {
        // The rendered hand of the body course renders at 96/128, below the
        // floor. No salience cell may feed and no reach aim may fire for it:
        // only what stands out reaches the reflexes. The learner still sees
        // the dim pixels through its ordinary light sensors — its freedom is
        // untouched — but the birthright arcs stay silent.
        let mut harness = WorkstationHarness::new(1).unwrap();
        let gaze = harness.state().eye(Eye::Left).gaze();
        let dim = bar_field(gaze, 832, 96);
        let sample = with_fields(
            bar_field(gaze, 832, 96),
            dim,
            [ContactSample::default(); TOUCH_SITES],
        );
        let wave = harness.sensory_wave(&sample, 1);
        let salience_cells: Vec<JunctionId> =
            harness.handles.salience.iter().flatten().copied().collect();
        assert!(wave
            .iter()
            .all(|arrival| !salience_cells.contains(&arrival.target)));
        let mut frame = ActuatorFrame::default();
        let before = frame.clone();
        harness.apply_pre_reach(&sample, &mut frame);
        assert_eq!(frame, before, "the pre-reach moved the arm below the floor");
    }

    #[test]
    fn value_links_carry_signal_into_the_salience_cells() {
        // The value channel through the public surface: in the live-key
        // scene (target jumps on hit, so receptor light changes), the
        // phasic value links must transmit those changes into the salience
        // cells. Signal through the channel is the precondition for the
        // learner ever writing value onto salience.
        let mut harness = WorkstationHarness::new(1).unwrap();
        let value_links: Vec<truelearner_body::LinkId> = Eye::ALL
            .into_iter()
            .flat_map(|eye| {
                (0..RECEPTORS_PER_EYE)
                    .filter_map(|r| harness.handles.value_link(eye, r))
                    .collect::<Vec<_>>()
            })
            .collect();
        // The live-key scene, honestly: the target jumps on each hit, so
        // receptor light changes every jump — the phasic value channel
        // fires on change, and the jumps give it change to carry.
        let mut rng = 1_i16;
        let mut jump_x = 600_i16;
        let bar = |gaze: Point, x: i16| bar_field(gaze, x, 230);
        let mut contacts = [ContactSample::default(); TOUCH_SITES];
        let mut strengthened_value_links = 0_usize;
        let mut value_arrivals = 0_usize;
        harness
            .observe(with_fields(
                bar(harness.state().eye(Eye::Left).gaze(), jump_x),
                bar(harness.state().eye(Eye::Right).gaze(), jump_x),
                contacts,
            ))
            .unwrap();
        for step in 0..64 {
            let depth = harness.state().hand().palm().depth();
            let contact_now = depth >= 600;
            contacts[0] = ContactSample::new(u16::from(contact_now) * 1_023, 0).unwrap();
            // A hit jumps the target, as the live-key world does.
            if contact_now && step % 2 == 0 {
                rng = rng.wrapping_mul(5).wrapping_add(1);
                jump_x = 200 + rng.rem_euclid(624);
            }
            let gaze_left = harness.state().eye(Eye::Left).gaze();
            let gaze_right = harness.state().eye(Eye::Right).gaze();
            let sample = with_fields(bar(gaze_left, jump_x), bar(gaze_right, jump_x), contacts);
            let (_observation, trace) = harness.step_traced(sample).unwrap();
            for event in &trace {
                match event {
                    BodyTraceEvent::Strengthened(strength) => {
                        if value_links.contains(&strength.link) {
                            strengthened_value_links += 1;
                        }
                    }
                    BodyTraceEvent::Arrival(arrival) => {
                        if let Some(via) = arrival.via {
                            if value_links.contains(&via) {
                                value_arrivals += 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        // Light changes must travel through the value channel, and returned
        // local activity must potentiate that redundant route.
        assert!(
            value_arrivals > 0,
            "the value channel is silent: {value_arrivals} arrivals"
        );
        assert!(
            strengthened_value_links > 0,
            "returned local activity did not strengthen a value link"
        );
    }

    #[test]
    fn value_links_survive_the_checkpoint_round_trip() {
        // Pre-release checkpoint honesty: the value links are part of the
        // body's structure, so save and restore must preserve every one,
        // and a restored body must keep behaving identically.
        let mut harness = WorkstationHarness::new(1).unwrap();
        let before: Vec<Option<LinkId>> = Eye::ALL
            .into_iter()
            .flat_map(|eye| {
                (0..RECEPTORS_PER_EYE)
                    .map(|r| harness.handles.value_link(eye, r))
                    .collect::<Vec<_>>()
            })
            .collect();
        let bar = |gaze: Point| bar_field(gaze, 832, 230);
        harness
            .observe(with_fields(
                bar(harness.state().eye(Eye::Left).gaze()),
                bar(harness.state().eye(Eye::Right).gaze()),
                [ContactSample::default(); TOUCH_SITES],
            ))
            .unwrap();
        for _ in 0..8 {
            let gaze_left = harness.state().eye(Eye::Left).gaze();
            let gaze_right = harness.state().eye(Eye::Right).gaze();
            harness
                .step(with_fields(
                    bar(gaze_left),
                    bar(gaze_right),
                    [ContactSample::default(); TOUCH_SITES],
                ))
                .unwrap();
        }
        let checkpoint = harness.save().unwrap();
        let restored = WorkstationHarness::restore(checkpoint).unwrap();
        let after: Vec<Option<LinkId>> = Eye::ALL
            .into_iter()
            .flat_map(|eye| {
                (0..RECEPTORS_PER_EYE)
                    .map(|r| restored.handles.value_link(eye, r))
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(before, after);
        assert_eq!(
            restored.state().eye(Eye::Left).gaze().x(),
            harness.state().eye(Eye::Left).gaze().x()
        );
        // The observer mapping agrees on both sides.
        for eye in Eye::ALL {
            let link = restored.handles.value_link(eye, 0).unwrap();
            assert_eq!(restored.receptor_for_value_link(link), Some((eye, 0)));
        }
    }

    #[test]
    fn every_receptor_has_a_learnable_value_link_onto_its_salience_cell() {
        // Top-down attention, isolated to bare structure: each retinal
        // receptor's light sensor carries a zero-impulse learnable link onto
        // its own salience cell. At birth it changes nothing — the reflexes
        // see only raw brightness — but the learner's strengthening laws can
        // raise it, so a receptor whose light has paid becomes effectively
        // brighter to the reflexes. Value written onto salience, built only
        // from the learner's own consequence history.
        let harness = WorkstationHarness::new(1).unwrap();
        for eye in Eye::ALL {
            for receptor in 0..RECEPTORS_PER_EYE {
                assert!(
                    harness.handles.value_link(eye, receptor).is_some(),
                    "receptor {receptor} of the {eye:?} eye has no value link"
                );
            }
        }
        // A fresh body behaves exactly as before the links existed: they
        // carry zero impulse, so a body with them foveates the same static
        // bar a body without them would.
        let mut harness = WorkstationHarness::new(1).unwrap();
        let bar = |gaze: Point| bar_field(gaze, 832, 230);
        harness
            .observe(with_fields(
                bar(harness.state().eye(Eye::Left).gaze()),
                bar(harness.state().eye(Eye::Right).gaze()),
                [ContactSample::default(); TOUCH_SITES],
            ))
            .unwrap();
        for _ in 0..96 {
            let gaze_left = harness.state().eye(Eye::Left).gaze();
            let gaze_right = harness.state().eye(Eye::Right).gaze();
            harness
                .step(with_fields(
                    bar(gaze_left),
                    bar(gaze_right),
                    [ContactSample::default(); TOUCH_SITES],
                ))
                .unwrap();
        }
        let gaze = harness.state().eye(Eye::Left).gaze().x();
        assert!(
            (gaze - 832).abs() <= 128,
            "the value links changed newborn behavior: gaze {gaze}"
        );
    }

    #[test]
    fn vergence_moves_both_eyes_together() {
        // The binocular rung evidence: each eye foveates its own stereo
        // target, but the choice machinery serializes the eyes, so no step
        // ever shows both eyes moving in opposition. Real vergence is
        // yoked: the eyes converge together or not at all. The vergence
        // controller must deliver same-step opposing movements.
        let mut harness = WorkstationHarness::new(1).unwrap();
        let left_bar = |gaze: Point| bar_field(gaze, 896, 230);
        let right_bar = |gaze: Point| bar_field(gaze, 128, 230);
        harness
            .observe(with_fields(
                left_bar(harness.state().eye(Eye::Left).gaze()),
                right_bar(harness.state().eye(Eye::Right).gaze()),
                [ContactSample::default(); TOUCH_SITES],
            ))
            .unwrap();
        let mut opposing_steps = 0_usize;
        for _ in 0..24 {
            let gaze_left = harness.state().eye(Eye::Left).gaze();
            let gaze_right = harness.state().eye(Eye::Right).gaze();
            let observation = harness
                .step(with_fields(
                    left_bar(gaze_left),
                    right_bar(gaze_right),
                    [ContactSample::default(); TOUCH_SITES],
                ))
                .unwrap();
            let left_moved = observation.movements.iter().any(|movement| {
                movement.changed && movement.axis == BodyAxis::EyeHorizontal { eye: Eye::Left }
            });
            let right_moved = observation.movements.iter().any(|movement| {
                movement.changed
                    && movement.axis == BodyAxis::EyeHorizontal { eye: Eye::Right }
                    && movement.net_impulse.signum()
                        != observation
                            .movements
                            .iter()
                            .find(|movement| {
                                movement.axis == BodyAxis::EyeHorizontal { eye: Eye::Left }
                            })
                            .map(|movement| movement.net_impulse.signum())
                            .unwrap_or(0)
            });
            opposing_steps += usize::from(left_moved && right_moved);
        }
        assert!(
            opposing_steps >= 2,
            "only {opposing_steps} same-step opposing vergence movements"
        );
    }

    #[test]
    fn a_lit_patch_pulls_the_palm_toward_it() {
        // The missing physics for aimed reaching, isolated to bare physics:
        // a bright patch on the organism's own retina must drive planar palm
        // transport toward its world position. The palm starts left of the
        // bar; a body without the drive never closes the gap.
        let mut harness = WorkstationHarness::new(1).unwrap();
        let blob_x = harness
            .state()
            .hand()
            .palm()
            .x()
            .saturating_add(320)
            .min(896);
        let bar = |gaze: Point| bar_field(gaze, blob_x, 230);
        harness
            .observe(with_fields(
                bar(harness.state().eye(Eye::Left).gaze()),
                bar(harness.state().eye(Eye::Right).gaze()),
                [ContactSample::default(); TOUCH_SITES],
            ))
            .unwrap();
        for _ in 0..96 {
            let gaze_left = harness.state().eye(Eye::Left).gaze();
            let gaze_right = harness.state().eye(Eye::Right).gaze();
            harness
                .step(with_fields(
                    bar(gaze_left),
                    bar(gaze_right),
                    [ContactSample::default(); TOUCH_SITES],
                ))
                .unwrap();
        }
        let palm_x = harness.state().hand().palm().x();
        assert!(
            palm_x >= blob_x - 160,
            "palm {palm_x} never reached the lit patch at {blob_x}"
        );
    }

    #[test]
    fn the_reach_selects_the_brighter_of_two_patches() {
        // The live-key rung isolated to bare physics: two lit patches — a
        // bright one and a dim one — must pull the palm into the bright
        // one, not into the empty midpoint between them. A reach that
        // averages cannot choose; a reach weighted by brightness can.
        let mut harness = WorkstationHarness::new(1).unwrap();
        let bright_x = 704_i16;
        let dim_x = 320_i16;
        let field = |gaze: Point| {
            let mut pixels = vec![0_u8; RECEPTORS_PER_EYE];
            for (receptor, pixel) in pixels.iter_mut().enumerate() {
                let position = receptor_position(receptor);
                let center = receptor_position(RECEPTOR_SIDE * 4 + 4);
                let here = gaze.x() + position.x() - center.x();
                if (here - bright_x).abs() <= 128 {
                    *pixel = 230;
                } else if (here - dim_x).abs() <= 128 {
                    *pixel = 140;
                }
            }
            LightField::new(RECEPTOR_SIDE as u16, RECEPTOR_SIDE as u16, pixels).unwrap()
        };
        harness
            .observe(with_fields(
                field(harness.state().eye(Eye::Left).gaze()),
                field(harness.state().eye(Eye::Right).gaze()),
                [ContactSample::default(); TOUCH_SITES],
            ))
            .unwrap();
        for _ in 0..96 {
            let gaze_left = harness.state().eye(Eye::Left).gaze();
            let gaze_right = harness.state().eye(Eye::Right).gaze();
            harness
                .step(with_fields(
                    field(gaze_left),
                    field(gaze_right),
                    [ContactSample::default(); TOUCH_SITES],
                ))
                .unwrap();
        }
        let palm_x = harness.state().hand().palm().x();
        assert!(
            (palm_x - bright_x).abs() <= 160,
            "palm {palm_x} chose neither: not the bright patch at {bright_x}"
        );
    }

    #[test]
    fn changing_the_right_eye_changes_only_right_receptors() {
        let harness = WorkstationHarness::new(2).unwrap();
        let before = sensory_values(&harness.sensory_wave(
            &with_fields(field(1), field(2), [ContactSample::default(); TOUCH_SITES]),
            1,
        ));
        let after = sensory_values(&harness.sensory_wave(
            &with_fields(field(1), field(3), [ContactSample::default(); TOUCH_SITES]),
            1,
        ));

        assert_eq!(
            &before[visual_range(Eye::Left)],
            &after[visual_range(Eye::Left)]
        );
        assert_ne!(
            &before[visual_range(Eye::Right)],
            &after[visual_range(Eye::Right)]
        );
        assert_eq!(&before[contact_range()], &after[contact_range()]);
        assert_eq!(
            &before[proprioception_range()],
            &after[proprioception_range()]
        );
    }

    #[test]
    fn changing_the_touch_reading_changes_only_the_contact_reading() {
        let harness = WorkstationHarness::new(3).unwrap();
        let mut contacts = [ContactSample::default(); TOUCH_SITES];
        contacts[0] = ContactSample::new(7, -3).unwrap();
        let before = sensory_values(&harness.sensory_wave(
            &with_fields(field(1), field(2), [ContactSample::default(); TOUCH_SITES]),
            1,
        ));
        let after =
            sensory_values(&harness.sensory_wave(&with_fields(field(1), field(2), contacts), 1));
        let differences = before
            .iter()
            .zip(&after)
            .enumerate()
            .filter_map(|(index, (left, right))| (left != right).then_some(index))
            .collect::<Vec<_>>();
        assert_eq!(differences, contact_range().collect::<Vec<_>>());
    }

    #[test]
    fn transition_is_transactional_and_restore_replays() {
        let harness = WorkstationHarness::new(1).unwrap();
        let (mut candidate, expected) = harness.transition(sample()).unwrap();
        assert_eq!(harness.read().unwrap().physical_tick, 0);
        assert!(expected.naturally_quiescent);

        let checkpoint = candidate.save().unwrap();
        let next_expected = candidate.step(sample()).unwrap();
        let mut restored = WorkstationHarness::restore(checkpoint).unwrap();
        assert_eq!(restored.step(sample()).unwrap(), next_expected);
    }

    #[test]
    fn observation_admits_sensation_without_a_fresh_opportunity() {
        let mut harness = WorkstationHarness::new(5).unwrap();
        let observation = harness.observe(sample()).unwrap();

        assert!(observation.admitted_inputs > 0);
        assert!(!observation.opportunity_admitted);
        assert!(observation.naturally_quiescent);
    }

    #[test]
    fn external_perturbation_changes_pose_without_changing_the_learner() {
        let mut harness = WorkstationHarness::new(6).unwrap();
        let before = harness.save().unwrap().open();
        let x_before = before.state.hand().palm().x();

        assert!(harness
            .perturb_body(
                BodyControl::new(BodyAxis::PalmHorizontal, Direction::Increase),
                1,
            )
            .unwrap());

        let after = harness.save().unwrap().open();
        assert_eq!(after.state.hand().palm().x(), x_before + 16);
        assert_eq!(after.body, before.body);
        assert_eq!(after.handles, before.handles);
        assert_eq!(after.sequence, before.sequence);
        assert_eq!(after.physical_tick, before.physical_tick);
        assert_eq!(after.pending_transitions, before.pending_transitions);
        assert_eq!(after.history, before.history);
    }

    #[test]
    fn traced_step_preserves_the_body_and_its_continuation() {
        let mut plain = WorkstationHarness::new(4).unwrap();
        let mut traced = plain.clone();

        let plain_observation = plain.step(sample()).unwrap();
        let (traced_observation, trace) = traced.step_traced(sample()).unwrap();

        assert_eq!(plain_observation, traced_observation);
        assert_eq!(format!("{:?}", plain.body), format!("{:?}", traced.body));
        assert!(trace
            .iter()
            .any(|event| matches!(event, BodyTraceEvent::Choice(_))));
        assert!(matches!(trace.last(), Some(BodyTraceEvent::Quiet(_))));
        crate::verify_choice_contract(&trace).unwrap();

        assert_eq!(
            plain.step(sample()).unwrap(),
            traced.step(sample()).unwrap()
        );
        assert_eq!(format!("{:?}", plain.body), format!("{:?}", traced.body));
    }

    #[test]
    fn traced_settlement_preserves_the_body_and_its_continuation() {
        let mut plain = WorkstationHarness::new(14).unwrap();
        let parent = (0..12)
            .find_map(|_| plain.step(sample()).unwrap().crossings.first().copied())
            .expect("the generic workstation exposes an outward crossing");
        let mut traced = plain.clone();

        let plain_observation = plain
            .settle_with_boundary_parents(sample(), &[parent])
            .unwrap();
        let (traced_observation, trace) = traced
            .settle_traced_with_boundary_parents(sample(), &[parent])
            .unwrap();

        assert_eq!(plain_observation, traced_observation);
        assert_eq!(format!("{:?}", plain.body), format!("{:?}", traced.body));
        assert!(matches!(trace.last(), Some(BodyTraceEvent::Quiet(_))));
        crate::verify_choice_contract(&trace).unwrap();
        assert_eq!(
            plain.step(sample()).unwrap(),
            traced.step(sample()).unwrap()
        );
    }

    #[test]
    fn returned_movement_fires_only_its_exact_axis_witness() {
        let mut harness = WorkstationHarness::new(5).unwrap();
        let mut outward = harness.step(sample()).unwrap();
        for _ in 0..3 {
            if !outward.pending_transitions.is_empty() {
                break;
            }
            outward = harness.step(sample()).unwrap();
        }
        assert!(!outward.pending_transitions.is_empty());

        let expected_axes = outward.pending_transitions;
        let (returned, trace) = harness.step_traced(sample()).unwrap();
        assert_eq!(returned.returned_transitions, expected_axes);

        let transitioned = trace
            .iter()
            .filter_map(|event| match event {
                BodyTraceEvent::Transition(event) => Some(event.junction),
                _ => None,
            })
            .collect::<Vec<_>>();
        let expected_witnesses = expected_axes
            .iter()
            .map(|axis| harness.handles.outcomes[axis.index()])
            .collect::<Vec<_>>();
        let observed_witnesses = harness
            .handles
            .outcomes
            .iter()
            .copied()
            .filter(|witness| transitioned.contains(witness))
            .collect::<Vec<_>>();

        assert_eq!(observed_witnesses, expected_witnesses);
        assert!(harness
            .handles
            .competition_outcomes
            .iter()
            .all(|witness| !transitioned.contains(witness)));
    }
}
