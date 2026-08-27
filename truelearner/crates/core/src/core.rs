//! The algorithm and its public contract.

pub use crate::checkpoint::{Checkpoint, CheckpointError};
pub use crate::identity::{JunctionId, LearnerId, LinkId};
pub use crate::junction::Junction;
pub use crate::link::{Link, TransmissionMode, TransmissionTrigger};
pub use crate::schedule::PhysicalClock;
pub use crate::trace::{ExecutionCost, PhysicalEvent, PhysicalTransition, RunResult as Run, Work};

use crate::body::Body;
use crate::junction::JunctionState;
use crate::schedule::Firing;
use crate::trace::RunResult;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub(crate) struct RunState {
    pub outputs: Vec<Output>,
    pub work: Work,
    pub cost: ExecutionCost,
    pub trace: Vec<PhysicalTransition>,
}

pub(crate) struct Moment {
    pub phase: i32,
    pub causal: u64,
    pub incidences: Vec<Incidence>,
}

pub(crate) struct Incidence {
    pub junction: JunctionId,
    pub inputs: Vec<Firing>,
    pub outcomes: Vec<Firing>,
}

pub(crate) struct Fired {
    pub junction: JunctionId,
    pub state: JunctionState,
    pub external: bool,
    pub causal_origin: u64,
}

/// A coherent choice of physical implementation for the algorithm.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    #[default]
    Physical,
    UnansweredReturnDeferral,
    UnansweredReturnReplacement,
    SensorimotorCandidate,
    SensorimotorSynthesis,
    RecursiveLearnerConstruction,
}

impl Protocol {
    pub(crate) fn is_sensorimotor(self) -> bool {
        matches!(
            self,
            Self::SensorimotorCandidate
                | Self::SensorimotorSynthesis
                | Self::RecursiveLearnerConstruction
        )
    }

    pub(crate) fn consolidates_reverse_paths(self) -> bool {
        matches!(
            self,
            Self::SensorimotorSynthesis | Self::RecursiveLearnerConstruction
        )
    }

    pub(crate) fn integrates_current_opportunity(self) -> bool {
        self.consolidates_reverse_paths()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Bindings {
    pub(crate) start: fn(&mut Body) -> RunState,
    pub(crate) links_meet: fn(&mut Body, &mut RunState) -> Option<Moment>,
    pub(crate) choose: fn(&mut Body, &mut Moment, &mut RunState),
    pub(crate) outcome_returns: fn(&mut Body, &Moment, &mut RunState),
    pub(crate) strengthen: fn(&mut Body, &Moment, &mut RunState),
    pub(crate) fire_junction: fn(&mut Body, Incidence, &Moment, &mut RunState) -> Option<Fired>,
    pub(crate) form_paths: fn(&mut Body, &Fired, &Moment, &mut RunState),
    pub(crate) fire_output: fn(&mut Body, Fired, &Moment, &mut RunState),
    pub(crate) hold: fn(&mut Body, &mut RunState),
    pub(crate) finish: fn(&mut Body, RunState) -> RunResult,
}

pub(crate) fn run(body: &mut Body, protocol: Bindings) -> RunResult {
    let mut run = (protocol.start)(body);
    // One loop is one physical moment. The full story in algo.md unfolds
    // across moments as input, output, and later outcome reach junctions.
    while let Some(mut moment) = (protocol.links_meet)(body, &mut run) {
        (protocol.choose)(body, &mut moment, &mut run);
        (protocol.outcome_returns)(body, &moment, &mut run);
        (protocol.strengthen)(body, &moment, &mut run);
        let time = Moment {
            phase: moment.phase,
            causal: moment.causal,
            incidences: Vec::new(),
        };
        for incidence in moment.incidences {
            if let Some(fired) = (protocol.fire_junction)(body, incidence, &time, &mut run) {
                (protocol.form_paths)(body, &fired, &time, &mut run);
                // New paths and strengthened paths execute by the same rule.
                (protocol.fire_output)(body, fired, &time, &mut run);
            }
        }
        (protocol.hold)(body, &mut run);
    }
    (protocol.finish)(body, run)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub arrival_tick: i64,
    pub phase: i32,
    pub origin_physical: u64,
    pub target: JunctionId,
    pub impulse: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    pub tick: i64,
    pub from_physical: u64,
    pub to_physical: u64,
    pub from_region: i16,
    pub to_region: i16,
    pub impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JunctionObservation {
    pub id: JunctionId,
    pub physical_id: u64,
    pub position: i32,
    pub region: i16,
    pub threshold: i32,
    pub resistance: u32,
    pub live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkObservation {
    pub id: LinkId,
    pub from: JunctionId,
    pub to: JunctionId,
    pub delay: i64,
    pub phase: i32,
    pub mode: TransmissionMode,
    pub coupling: i32,
    pub resistance: u32,
    pub strength: i64,
    pub life: u64,
    pub participation: u64,
    pub last_consequence_tick: Option<i64>,
    pub return_origins: Vec<u64>,
    pub live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LearnerObservation {
    pub id: LearnerId,
    pub parent: Option<LearnerId>,
    pub surface: JunctionId,
    pub output: JunctionId,
    pub junctions: Vec<JunctionId>,
    pub links: Vec<LinkId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HarnessObservation {
    pub clock: PhysicalClock,
    pub protocol: Protocol,
    pub return_path_count: usize,
    pub resident_bytes: usize,
    pub junctions: Vec<JunctionObservation>,
    pub links: Vec<LinkObservation>,
    pub learners: Vec<LearnerObservation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HarnessBuilder {
    body: Body,
    outward_region: i16,
}

impl HarnessBuilder {
    pub fn with_capacity(junction_capacity: u32, link_capacity: u32, outward_region: i16) -> Self {
        Self {
            body: Body::with_capacity(junction_capacity, link_capacity),
            outward_region,
        }
    }

    pub fn set_physical_tracing(&mut self, enabled: bool) {
        self.body.set_physical_tracing(enabled);
    }

    pub fn set_protocol(&mut self, protocol: Protocol) {
        self.body.set_protocol(protocol);
    }

    pub fn add_junction(&mut self, spec: Junction) -> JunctionId {
        self.body.add_junction(spec)
    }

    pub fn add_link(&mut self, spec: Link) -> LinkId {
        self.body.add_link(spec)
    }

    pub fn set_link_trigger(&mut self, link: LinkId, trigger: TransmissionTrigger) {
        self.body.set_link_trigger(link, trigger);
    }

    pub fn set_outcome_source(&mut self, source: JunctionId) {
        self.body.set_outcome_source(source);
    }

    pub fn set_outcome_source_for_output(&mut self, output: JunctionId, source: JunctionId) {
        self.body.set_outcome_source_for_output(output, source);
    }

    pub fn build(self) -> Harness {
        Harness {
            body: self.body,
            outward_region: self.outward_region,
        }
    }
}

impl HarnessObservation {
    pub fn fingerprint(&self) -> [u8; 32] {
        let mut hash = Sha256::new();
        hash.update(b"truelearner-harness-observation-v1");
        for junction in &self.junctions {
            hash.update(junction.id.0.to_le_bytes());
            hash.update(junction.physical_id.to_le_bytes());
            hash.update(junction.position.to_le_bytes());
            hash.update(junction.region.to_le_bytes());
            hash.update(junction.threshold.to_le_bytes());
            hash.update(junction.resistance.to_le_bytes());
            hash.update([u8::from(junction.live)]);
        }
        for link in &self.links {
            hash.update(link.id.0.to_le_bytes());
            hash.update(link.from.0.to_le_bytes());
            hash.update(link.to.0.to_le_bytes());
            hash.update(link.delay.to_le_bytes());
            hash.update(link.phase.to_le_bytes());
            hash.update([match link.mode {
                TransmissionMode::Drive => 0,
                TransmissionMode::Modulatory => 1,
            }]);
            hash.update(link.coupling.to_le_bytes());
            hash.update(link.resistance.to_le_bytes());
            hash.update(link.strength.to_le_bytes());
            hash.update(link.life.to_le_bytes());
            hash.update(link.participation.to_le_bytes());
            hash.update(link.last_consequence_tick.unwrap_or(i64::MIN).to_le_bytes());
            hash.update(
                u64::try_from(link.return_origins.len())
                    .unwrap_or(u64::MAX)
                    .to_le_bytes(),
            );
            for origin in &link.return_origins {
                hash.update(origin.to_le_bytes());
            }
            hash.update([u8::from(link.live)]);
        }
        for learner in &self.learners {
            hash.update(learner.id.0.to_le_bytes());
            hash.update(learner.parent.unwrap_or_default().0.to_le_bytes());
            hash.update([u8::from(learner.parent.is_some())]);
            hash.update(learner.surface.0.to_le_bytes());
            hash.update(learner.output.0.to_le_bytes());
            for junction in &learner.junctions {
                hash.update(junction.0.to_le_bytes());
            }
            for link in &learner.links {
                hash.update(link.0.to_le_bytes());
            }
        }
        hash.finalize().into()
    }

    pub fn junction(&self, id: JunctionId) -> Option<&JunctionObservation> {
        self.junctions.iter().find(|junction| junction.id == id)
    }

    pub fn link(&self, id: LinkId) -> Option<&LinkObservation> {
        self.links.iter().find(|link| link.id == id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Harness {
    body: Body,
    outward_region: i16,
}

impl Harness {
    pub fn send(&mut self, inputs: &[Input]) -> Run {
        let mut next = self.body.clone();
        let mut run = next.arrive(inputs, self.outward_region);
        run.outputs
            .retain(|output| output.to_region == self.outward_region);
        self.body = next;
        run
    }

    pub fn advance_to(&mut self, tick: i64) -> Work {
        self.body.advance_time(tick)
    }

    pub fn read(&self) -> HarnessObservation {
        let junctions = self
            .body
            .arena
            .junctions
            .iter()
            .map(|junction| JunctionObservation {
                id: junction.id,
                physical_id: junction.physical_id,
                position: junction.position,
                region: junction.region,
                threshold: junction.threshold,
                resistance: junction.resistance,
                live: junction.live,
            })
            .collect();
        let links = self
            .body
            .arena
            .links
            .iter()
            .map(|link| LinkObservation {
                id: link.id,
                from: link.from,
                to: link.to,
                delay: link.delay,
                phase: link.phase,
                mode: link.mode,
                coupling: link.coupling,
                resistance: link.resistance,
                strength: self.body.arena.strength[link.id.0 as usize],
                life: self.body.arena.life[link.id.0 as usize],
                participation: link.participation_level,
                last_consequence_tick: link.last_consequence_tick,
                return_origins: link.return_origins.clone(),
                live: link.live,
            })
            .collect();
        let learners = self
            .body
            .learners
            .iter()
            .map(|learner| LearnerObservation {
                id: learner.id,
                parent: learner.parent,
                surface: learner.surface,
                output: learner.output,
                junctions: learner.junctions.clone(),
                links: learner.links.clone(),
            })
            .collect();
        HarnessObservation {
            clock: self.body.clock(),
            protocol: self.body.protocol(),
            return_path_count: self.body.return_path_count(),
            resident_bytes: self.body.arena.memory_bytes(),
            junctions,
            links,
            learners,
        }
    }

    pub fn save(&self) -> Result<Checkpoint, CheckpointError> {
        Ok(Checkpoint::new(self.body.snapshot()?, self.outward_region))
    }

    pub fn restore(checkpoint: Checkpoint) -> Result<Self, CheckpointError> {
        let (body, outward_region) = checkpoint.open();
        Ok(Self {
            body: Body::from_snapshot(body)?,
            outward_region,
        })
    }
}
