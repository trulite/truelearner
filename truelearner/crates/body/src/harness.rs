//! Small physical setup and observation helpers for body-level laws.

use crate::{
    attach, calibrate, Arrival, Body, Join, Junction, JunctionId, Link, LinkRole, OpenBody,
    PhysicalEvent, Residual, Run,
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

pub fn reading(target: JunctionId, normal: i32, value: i32, cause: u64) -> Arrival {
    Arrival::caused(target, calibrated(normal, value), cause)
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
    let mut part = Body::default();
    let local_effect = integrating(&mut part, 1);
    let part = OpenBody::new(part, vec![local_effect]).unwrap();
    let port = part.port(0).unwrap();
    let effect = attach(body, part, &[Join::into_part(opportunity, port, 0, 1)])
        .unwrap()
        .port(port)
        .unwrap();
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
        body.set_link_role(link, LinkRole::OutcomeWitness)
            .expect("new outcome link exists");
    }
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

pub fn physical_trace(events: &[PhysicalEvent]) -> Vec<(u64, JunctionId, i32, i32, u64)> {
    events
        .iter()
        .map(|event| {
            (
                event.at,
                event.junction,
                event.before,
                event.after,
                event.cause,
            )
        })
        .collect()
}
