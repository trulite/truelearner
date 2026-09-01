use super::*;
use crate::{
    harness::{attach_outcome_component, attach_sensor, finish, motor, reading, schedule},
    Arrival, Junction, TraceEvent,
};

fn body_with_open_return() -> (Body, crate::JunctionId) {
    let mut body = Body::default();
    let motor = motor(&mut body);
    let sensor = attach_sensor(
        &mut body,
        Junction::integrating(1),
        &[(motor.opportunity, 1)],
    );
    let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
    attach_outcome_component(&mut body, outcome, [motor.opportunity]);
    schedule(&mut body, 0, &[Arrival::caused(outcome, 0, 0)]);
    finish(&mut body);
    schedule(&mut body, 1, &[reading(sensor, 0, 1, 1)]);
    schedule(&mut body, 2, &[Arrival::caused(motor.opportunity, 1, 1)]);
    finish(&mut body);
    (body, outcome)
}

#[test]
fn checkpoint_restores_the_exact_next_wave() {
    let (body, outcome) = body_with_open_return();
    let bytes = body.checkpoint().unwrap().canonical_bytes().unwrap();
    let mut plain = body;
    let mut restored = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();
    let mut histories = Vec::new();
    for candidate in [&mut plain, &mut restored] {
        candidate
            .inputs(candidate.now() + 1, &[Arrival::caused(outcome, 1, 1)])
            .unwrap();
        let mut physical = Vec::new();
        let mut trace = Vec::<TraceEvent>::new();
        candidate
            .run_traced(8, |event| physical.push(event), |event| trace.push(event))
            .unwrap();
        histories.push((physical, trace));
    }
    assert_eq!(histories[0], histories[1]);
    assert_eq!(
        plain.checkpoint().unwrap().canonical_bytes().unwrap(),
        restored.checkpoint().unwrap().canonical_bytes().unwrap()
    );
}

#[test]
fn corruption_and_nonquiet_save_fail_closed() {
    let (mut body, outcome) = body_with_open_return();
    let bytes = body.checkpoint().unwrap().canonical_bytes().unwrap();
    body.input(body.now() + 1, outcome, 1).unwrap();
    assert_eq!(body.checkpoint(), Err(BodyCheckpointError::BodyNotQuiet));

    let mut corrupt = bytes.clone();
    corrupt[HEADER_LEN] ^= 1;
    assert_eq!(
        BodyCheckpoint::decode(&corrupt),
        Err(BodyCheckpointError::Checksum)
    );
    assert_eq!(
        BodyCheckpoint::decode(&bytes[..HEADER_LEN - 1]),
        Err(BodyCheckpointError::Truncated)
    );

    let mut obsolete = bytes;
    obsolete[8..10].copy_from_slice(&8_u16.to_le_bytes());
    assert_eq!(
        BodyCheckpoint::decode(&obsolete),
        Err(BodyCheckpointError::UnsupportedVersion(8))
    );
}
