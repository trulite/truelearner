use super::*;
use crate::{
    harness::{attach_outcome_component, attach_sensor, finish, motor, reading, schedule},
    Arrival, Junction, TraceEvent, Trigger,
};

#[derive(Serialize)]
enum VersionEightTrigger {
    SourceFires,
    RisesThrough(i32),
    FallsThrough(i32),
}

#[derive(Serialize)]
struct VersionEightLink {
    from: crate::JunctionId,
    to: crate::JunctionId,
    delay: u64,
    impulse: i32,
    trigger: VersionEightTrigger,
}

#[derive(Serialize)]
struct VersionEightLinkRecord {
    law: VersionEightLink,
    memory: LinkMemory,
}

#[derive(Serialize)]
struct VersionEightPayload {
    now: u64,
    junctions: Vec<JunctionRecord>,
    links: Vec<VersionEightLinkRecord>,
    automaticity: Option<Box<Automaticity>>,
}

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
fn version_eight_threshold_triggers_keep_their_checkpoint_meaning() {
    let first = crate::JunctionId::new(0).unwrap();
    let second = crate::JunctionId::new(1).unwrap();
    let payload = VersionEightPayload {
        now: 0,
        junctions: vec![
            JunctionRecord {
                law: Junction::sampled(100),
                stamp: 0,
                value: 0,
                sampled_known: false,
            },
            JunctionRecord {
                law: Junction::integrating(1),
                stamp: 0,
                value: 0,
                sampled_known: false,
            },
        ],
        links: vec![
            VersionEightLinkRecord {
                law: VersionEightLink {
                    from: first,
                    to: second,
                    delay: 0,
                    impulse: 1,
                    trigger: VersionEightTrigger::SourceFires,
                },
                memory: LinkMemory::default(),
            },
            VersionEightLinkRecord {
                law: VersionEightLink {
                    from: first,
                    to: second,
                    delay: 1,
                    impulse: 1,
                    trigger: VersionEightTrigger::RisesThrough(5),
                },
                memory: LinkMemory::default(),
            },
            VersionEightLinkRecord {
                law: VersionEightLink {
                    from: first,
                    to: second,
                    delay: 2,
                    impulse: -1,
                    trigger: VersionEightTrigger::FallsThrough(-5),
                },
                memory: LinkMemory::default(),
            },
        ],
        automaticity: None,
    };
    let payload = options().serialize(&payload).unwrap();
    let mut bytes = Vec::with_capacity(HEADER_LEN + payload.len());
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&VERSION.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    bytes.extend_from_slice(&Sha256::digest(&payload));
    bytes.extend_from_slice(&payload);

    let body = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();
    let triggers = body
        .arena
        .links()
        .map(|link| link.checkpoint_law().trigger)
        .collect::<Vec<_>>();

    assert_eq!(
        triggers,
        [
            Trigger::SourceFires,
            Trigger::RisesThrough(5),
            Trigger::FallsThrough(-5)
        ]
    );
}

#[test]
fn version_seven_checkpoint_restores_with_empty_dependent_thought_state() {
    let payload = PayloadV7 {
        now: 0,
        junctions: Vec::new(),
        links: Vec::new(),
        automaticity: Some(Box::new(AutomaticityV7::default())),
    };
    let payload = options().serialize(&payload).unwrap();
    let mut bytes = Vec::with_capacity(HEADER_LEN + payload.len());
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&PREVIOUS_VERSION.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    bytes.extend_from_slice(&Sha256::digest(&payload));
    bytes.extend_from_slice(&payload);

    let body = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();

    assert_eq!(body.reentry_state().thought_shortcuts, 0);
    assert_eq!(body.automaticity_work(), crate::AutomaticityWork::default());
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
}
