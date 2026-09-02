//! Small physical setup and observation helpers for body-level laws.

use crate::{
    attach, calibrate, Arrival, Body, Join, Junction, JunctionId, Link, LinkId, OpenBody,
    PhysicalEvent, Residual, Run, Time, Trigger, WitnessKind,
};

#[derive(Clone, Copy)]
pub struct Motor {
    pub opportunity: JunctionId,
    pub effect: JunctionId,
}

pub struct Trace {
    pub run: Run,
    pub events: Vec<PhysicalEvent>,
}

pub fn integrating(body: &mut Body, threshold: i32) -> JunctionId {
    body.add_junction(Junction::integrating(threshold)).unwrap()
}

pub fn calibrated(normal: i32, reading: i32) -> i32 {
    let mut normalizer = calibrate(normal, |normal: &i32, reading: &i32| {
        Residual::new(normal.abs_diff(*reading))
    });
    normalizer.step(Some(reading)).unwrap().amount() as i32
}

pub fn reading(target: JunctionId, normal: i32, value: i32) -> Arrival {
    Arrival::new(target, calibrated(normal, value))
}

pub fn attach_sensor(
    body: &mut Body,
    sensor: Junction,
    nearby_outputs: &[(JunctionId, u64)],
) -> JunctionId {
    let mut part = Body::default();
    let local = part.add_junction(sensor).unwrap();
    let part = OpenBody::new(part, vec![local]).unwrap();
    let port = part.port(0).unwrap();
    let joins = nearby_outputs
        .iter()
        .map(|(output, distance)| Join::into_host(*output, port, *distance, 0))
        .collect::<Vec<_>>();
    attach(body, part, &joins).unwrap().port(port).unwrap()
}

pub fn motor(body: &mut Body) -> Motor {
    let opportunity = integrating(body, 2);
    let effect = integrating(body, 1);
    let crossing = body
        .add_link(Link::new(opportunity, effect, 0, 1))
        .expect("validated motor crossing");
    body.mark_boundary_drive(crossing)
        .expect("new motor crossing exists");
    Motor {
        opportunity,
        effect,
    }
}

pub fn attach_outcome_component(
    body: &mut Body,
    source: JunctionId,
    motor_opportunities: impl IntoIterator<Item = JunctionId>,
) {
    for opportunity in motor_opportunities {
        let link = body
            .add_link(Link::new(source, opportunity, 0, 1))
            .expect("validated outcome component");
        body.mark_witness(
            link,
            WitnessKind::Closure {
                offers_choice: true,
            },
        )
        .expect("new outcome link exists");
    }
}

pub fn attach_boundary_component(
    body: &mut Body,
    source: JunctionId,
    motor_opportunities: impl IntoIterator<Item = JunctionId>,
) {
    for opportunity in motor_opportunities {
        let link = body
            .add_link(Link::new(source, opportunity, 0, 1))
            .expect("validated boundary component");
        body.mark_witness(
            link,
            WitnessKind::Closure {
                offers_choice: false,
            },
        )
        .expect("new boundary link exists");
    }
}

/// Attaches a physical progress source to candidate motor outputs without
/// making that source capable of closing their temporary return paths.
pub fn attach_progress_component(
    body: &mut Body,
    source: JunctionId,
    motor_opportunities: impl IntoIterator<Item = JunctionId>,
) {
    for opportunity in motor_opportunities {
        let link = body
            .add_link(Link::new(source, opportunity, 0, 1))
            .expect("validated progress component");
        body.mark_witness(link, WitnessKind::Progress)
            .expect("new progress link exists");
    }
}

/// Attaches a learnable link from a source to one target: born with a
/// whisper of impulse, so it fires and can participate in a path, and the
/// learner's strengthening laws potentiate it from there — like LTP, the
/// connection exists weakly and grows with use. A zero-impulse link could
/// never fire at any strength, since transmission multiplies impulse by
/// strength; one that fires can grow. The link stays an ordinary drive, so
/// transmission carries it and closures can return through it.
pub fn attach_learnable_link(
    body: &mut Body,
    source: JunctionId,
    target: JunctionId,
    delay: Time,
    trigger: Trigger,
) -> LinkId {
    let link = body
        .add_link(Link::new(source, target, delay, 1).when(trigger))
        .expect("validated learnable link");
    body.mark_locally_plastic(link)
        .expect("new learnable link exists");
    link
}

pub fn schedule(body: &mut Body, at: u64, arrivals: &[Arrival]) {
    body.inputs(at, arrivals).unwrap();
}

pub fn finish(body: &mut Body) -> Trace {
    let mut events = Vec::new();
    let run = body.run(256, |event| events.push(event)).unwrap();
    assert!(body.is_quiet());
    Trace { run, events }
}

pub fn event_count(events: &[PhysicalEvent], junction: JunctionId) -> usize {
    events
        .iter()
        .filter(|event| event.junction == junction)
        .count()
}

pub fn effect(events: &[PhysicalEvent], motors: &[Motor]) -> Vec<usize> {
    events
        .iter()
        .filter_map(|event| {
            motors
                .iter()
                .position(|motor| motor.effect == event.junction)
        })
        .collect()
}

pub fn physical_trace(events: &[PhysicalEvent]) -> Vec<(u64, JunctionId, i32, i32)> {
    events
        .iter()
        .map(|event| (event.at, event.junction, event.before, event.after))
        .collect()
}
