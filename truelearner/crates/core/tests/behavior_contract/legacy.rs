use std::collections::BTreeMap;
use truelearner_behavior_contract::{
    Adapter, Effect, Episode, InputTarget, Morphology, MotorId, Observation, Retention, SensorId,
};
use truelearner_core::{
    Checkpoint, CheckpointError, Harness, HarnessBuilder, Input, Junction, JunctionId, Link,
    PhysicalEvent, PhysicalIncidence, PhysicalInput, Protocol, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const FIRST_SENSOR_PHYSICAL: u64 = 100_000;
const FIRST_MOTOR_PHYSICAL: u64 = 200_000;
const ANCHOR_PHYSICAL: u64 = 99_999;
const DEFAULT_MOTOR_POSITION: i32 = 1_000_000;
const CONSEQUENCE_SENSOR: SensorId = SensorId(1);

#[derive(Clone, Copy, Debug)]
pub enum LegacyProfile {
    Physical,
}

impl LegacyProfile {
    fn protocol(self) -> Protocol {
        match self {
            Self::Physical => Protocol::Physical,
        }
    }

    fn consequence_sensor(self) -> SensorId {
        match self {
            Self::Physical => CONSEQUENCE_SENSOR,
        }
    }
}

#[derive(Clone)]
pub struct LegacyAdapter {
    profile: LegacyProfile,
    tracing: bool,
}

impl LegacyAdapter {
    pub const fn new(profile: LegacyProfile, tracing: bool) -> Self {
        Self { profile, tracing }
    }
}

pub struct LegacyOrganism {
    harness: Harness,
    sensors: BTreeMap<SensorId, JunctionId>,
    motors: BTreeMap<MotorId, JunctionId>,
    outward: BTreeMap<u64, MotorId>,
}

#[derive(Clone)]
pub struct LegacyCheckpoint {
    checkpoint: Checkpoint,
    sensors: BTreeMap<SensorId, JunctionId>,
    motors: BTreeMap<MotorId, JunctionId>,
    outward: BTreeMap<u64, MotorId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LegacyError {
    TimeOverflow,
    CapacityOverflow,
    UnknownSensor(SensorId),
    UnknownMotor(MotorId),
    Checkpoint(CheckpointError),
}

impl Adapter for LegacyAdapter {
    type Organism = LegacyOrganism;
    type Checkpoint = LegacyCheckpoint;
    type Error = LegacyError;

    fn build(&self, morphology: &Morphology) -> Result<Self::Organism, Self::Error> {
        let node_capacity = morphology
            .sensors
            .len()
            .checked_add(morphology.motors.len().saturating_mul(2))
            .and_then(|count| count.checked_add(65))
            .and_then(|count| u32::try_from(count).ok())
            .ok_or(LegacyError::CapacityOverflow)?;
        let link_capacity = morphology
            .sensors
            .len()
            .checked_add(morphology.motors.len())
            .and_then(|count| count.checked_add(256))
            .and_then(|count| u32::try_from(count).ok())
            .ok_or(LegacyError::CapacityOverflow)?;
        let mut builder =
            HarnessBuilder::with_capacity(node_capacity, link_capacity, OUTWARD_REGION);
        builder.set_protocol(self.profile.protocol());
        builder.set_physical_tracing(self.tracing);

        let mut sensors = BTreeMap::new();
        for sensor in &morphology.sensors {
            let threshold = match sensor.retention {
                Retention::Integrating { threshold } => threshold,
                Retention::Sampled { .. } => 1,
            };
            let id = builder.add_junction(Junction {
                physical_id: sensor_physical(sensor.id),
                position: sensor_position(sensor.id),
                region: 0,
                threshold,
                resistance: u32::MAX,
            });
            sensors.insert(sensor.id, id);
        }

        let positions = motor_positions(morphology);
        let mut motors = BTreeMap::new();
        let mut outward = BTreeMap::new();
        for motor in &morphology.motors {
            let position = positions[&motor.id];
            let opportunity = builder.add_junction(Junction {
                physical_id: motor_input_physical(motor.id),
                position,
                region: 0,
                threshold: 2,
                resistance: u32::MAX,
            });
            let effect_physical = motor_effect_physical(motor.id);
            let effect = builder.add_junction(Junction {
                physical_id: effect_physical,
                position: position.saturating_add(1),
                region: OUTWARD_REGION,
                threshold: 1,
                resistance: u32::MAX,
            });
            builder.add_link(Link {
                from: opportunity,
                to: effect,
                delay: 0,
                phase: 0,
                coupling: 1,
                resistance: u32::MAX,
                mode: TransmissionMode::Drive,
            });
            motors.insert(motor.id, opportunity);
            outward.insert(effect_physical, motor.id);
        }

        if !sensors.is_empty() {
            let anchor = builder.add_junction(Junction {
                physical_id: ANCHOR_PHYSICAL,
                position: i32::MAX,
                region: 0,
                threshold: i32::MAX,
                resistance: u32::MAX,
            });
            for sensor in sensors.values().copied() {
                builder.add_link(Link {
                    from: anchor,
                    to: sensor,
                    delay: 0,
                    phase: 0,
                    coupling: 1,
                    resistance: u32::MAX,
                    mode: TransmissionMode::Drive,
                });
            }
        }
        if let Some(consequence) = sensors.get(&self.profile.consequence_sensor()) {
            builder.set_outcome_source(*consequence);
        }

        Ok(LegacyOrganism {
            harness: builder.build(),
            sensors,
            motors,
            outward,
        })
    }

    fn run(
        &self,
        organism: &mut Self::Organism,
        episode: &Episode,
    ) -> Result<Observation, Self::Error> {
        let inputs = episode
            .inputs
            .iter()
            .map(|input| {
                let (target, incidence) = match input.target {
                    InputTarget::Sensor(sensor) => (
                        *organism
                            .sensors
                            .get(&sensor)
                            .ok_or(LegacyError::UnknownSensor(sensor))?,
                        PhysicalIncidence::Transition,
                    ),
                    InputTarget::Motor(motor) => (
                        *organism
                            .motors
                            .get(&motor)
                            .ok_or(LegacyError::UnknownMotor(motor))?,
                        PhysicalIncidence::Sample,
                    ),
                };
                Ok(PhysicalInput {
                    input: Input {
                        arrival_tick: i64::try_from(input.at)
                            .map_err(|_| LegacyError::TimeOverflow)?,
                        phase: 0,
                        origin_physical: input.cause,
                        target,
                        impulse: input.impulse,
                    },
                    incidence,
                })
            })
            .collect::<Result<Vec<_>, LegacyError>>()?;
        let run = organism
            .harness
            .send_physical_bounded(&inputs, episode.moment_limit);
        let unique_cause = episode
            .inputs
            .first()
            .map(|input| input.cause)
            .filter(|cause| episode.inputs.iter().all(|input| input.cause == *cause));
        let effects = run
            .outputs
            .iter()
            .filter_map(|output| {
                let motor = organism.outward.get(&output.to_physical).copied()?;
                let cause = unique_cause
                    .or_else(|| {
                        carried_origin(&run.physical_trace, output.to_physical, output.tick)
                    })
                    .unwrap_or_default();
                Some(Effect {
                    at: u64::try_from(output.tick).ok()?,
                    motor,
                    impulse: output.impulse,
                    cause,
                })
            })
            .collect();
        Ok(Observation {
            effects,
            quiet: run.naturally_quiescent,
        })
    }

    fn save(&self, organism: &Self::Organism) -> Result<Self::Checkpoint, Self::Error> {
        Ok(LegacyCheckpoint {
            checkpoint: organism.harness.save().map_err(LegacyError::Checkpoint)?,
            sensors: organism.sensors.clone(),
            motors: organism.motors.clone(),
            outward: organism.outward.clone(),
        })
    }

    fn restore(&self, checkpoint: &Self::Checkpoint) -> Result<Self::Organism, Self::Error> {
        Ok(LegacyOrganism {
            harness: Harness::restore(checkpoint.checkpoint.clone())
                .map_err(LegacyError::Checkpoint)?,
            sensors: checkpoint.sensors.clone(),
            motors: checkpoint.motors.clone(),
            outward: checkpoint.outward.clone(),
        })
    }
}

fn motor_positions(morphology: &Morphology) -> BTreeMap<MotorId, i32> {
    let mut positions = morphology
        .motors
        .iter()
        .map(|motor| {
            (
                motor.id,
                DEFAULT_MOTOR_POSITION.saturating_add(i32::from(motor.id.0).saturating_mul(100)),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for nearby in &morphology.nearby {
        positions.insert(
            nearby.motor,
            sensor_position(nearby.sensor)
                .saturating_add(i32::try_from(nearby.distance).unwrap_or(i32::MAX)),
        );
    }
    positions
}

const fn sensor_position(sensor: SensorId) -> i32 {
    sensor.0 as i32 * 100
}

const fn sensor_physical(sensor: SensorId) -> u64 {
    FIRST_SENSOR_PHYSICAL + sensor.0 as u64
}

const fn motor_input_physical(motor: MotorId) -> u64 {
    FIRST_MOTOR_PHYSICAL + motor.0 as u64 * 2
}

const fn motor_effect_physical(motor: MotorId) -> u64 {
    motor_input_physical(motor) + 1
}

fn carried_origin(
    trace: &[truelearner_core::PhysicalTransition],
    target_physical: u64,
    tick: i64,
) -> Option<u64> {
    trace.iter().rev().find_map(|transition| {
        if transition.tick != tick {
            return None;
        }
        match transition.event {
            PhysicalEvent::DriveProvenanceObserved {
                target_physical: target,
                carried_origin,
                ..
            } if target == target_physical => Some(carried_origin),
            _ => None,
        }
    })
}
