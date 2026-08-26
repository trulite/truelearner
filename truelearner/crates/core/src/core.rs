//! The algorithm and its public contract.

pub use crate::body::Body;
pub use crate::checkpoint::{Checkpoint, CheckpointError};
pub use crate::junction::{Junction, JunctionSlot};
pub use crate::link::{Link, LinkSlot, TransmissionMode, TransmissionTrigger};
pub use crate::schedule::PhysicalClock;
pub use crate::trace::{ExecutionCost, PhysicalEvent, PhysicalTransition, RunResult as Run, Work};
pub use truelearner_arena_format::{
    ArenaId, ArrowId as LinkId, CellId as JunctionId, ContentHash, Generation,
};

use crate::junction::JunctionState;
use crate::schedule::Firing;
use crate::trace::RunResult;
use serde::{Deserialize, Serialize};

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
}

/// A coherent choice of physical implementation for the algorithm.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    #[default]
    Physical,
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
pub enum CoreError {
    WrongOutwardRegion { configured: i16, requested: i16 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Core {
    body: Body,
    outward_region: i16,
}

impl Core {
    pub fn new(body: Body, outward_region: i16) -> Self {
        Self {
            body,
            outward_region,
        }
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn fire(&mut self, inputs: &[Input]) -> Run {
        let mut next = self.body.clone();
        let mut run = next.arrive(inputs, self.outward_region);
        run.outputs
            .retain(|output| output.to_region == self.outward_region);
        self.body = next;
        run
    }

    pub fn arrive(&mut self, inputs: &[Input], outward_region: i16) -> Result<Run, CoreError> {
        if outward_region != self.outward_region {
            return Err(CoreError::WrongOutwardRegion {
                configured: self.outward_region,
                requested: outward_region,
            });
        }
        Ok(self.fire(inputs))
    }

    pub fn advance_time(&mut self, tick: i64) -> Work {
        self.body.advance_time(tick)
    }

    pub fn set_outcome_source(&mut self, source: JunctionId) {
        self.body.set_outcome_source(source);
    }

    pub fn return_path_count(&self) -> usize {
        self.body.return_path_count()
    }

    pub fn save(&self, body_version: u64) -> Result<Checkpoint, CheckpointError> {
        Ok(Checkpoint::new(
            self.body.snapshot(body_version)?,
            self.outward_region,
        ))
    }

    pub fn restore(checkpoint: Checkpoint) -> Result<Self, CheckpointError> {
        let (body, outward_region) = checkpoint.open();
        Ok(Self {
            body: Body::from_snapshot(body)?,
            outward_region,
        })
    }
}
