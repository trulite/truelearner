use std::{hint::black_box, time::Duration, time::Instant};
use truelearner_body::{
    harness::{attach_outcome_component, attach_sensor, finish, motor, reading, schedule},
    Arrival, Body, Junction, JunctionId, ReturnDecision, TraceEvent,
};

const BODIES_PER_BATCH: usize = 64;
const WAVES: usize = 10_000;
const SAMPLES: usize = 9;

fn main() {
    verify_measured_wave();
    sample(BODIES_PER_BATCH * 2);
    let mut samples = [0.0; SAMPLES];
    for result in &mut samples {
        *result = sample(WAVES);
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "warm_bodies_per_batch={BODIES_PER_BATCH} waves_per_sample={WAVES} samples={SAMPLES} min_ns={:.2} median_ns={:.2} max_ns={:.2}",
        samples[0],
        samples[SAMPLES / 2],
        samples[SAMPLES - 1]
    );
}

fn sample(waves: usize) -> f64 {
    let mut elapsed = Duration::ZERO;
    let mut remaining = waves;
    while remaining > 0 {
        let batch = remaining.min(BODIES_PER_BATCH);
        let mut bodies = (0..batch).map(|_| open_two_returns()).collect::<Vec<_>>();
        let started = Instant::now();
        for (body, outcome) in &mut bodies {
            admit_return(body, *outcome);
            black_box(
                body.run(8, |event| {
                    black_box(event);
                })
                .unwrap(),
            );
        }
        elapsed += started.elapsed();
        remaining -= batch;
    }
    elapsed.as_nanos() as f64 / waves as f64
}

fn verify_measured_wave() {
    let (mut body, outcome) = open_two_returns();
    let mut accepted = 0;
    let mut strengthened = 0;
    admit_return(&mut body, outcome);
    body.run_traced(
        8,
        |_| {},
        |event| match event {
            TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted => {
                accepted += 1;
            }
            TraceEvent::Strengthened(_) => strengthened += 1,
            _ => {}
        },
    )
    .unwrap();
    assert_eq!(accepted, 1);
    assert_eq!(strengthened, 2);
}

fn admit_return(body: &mut Body, outcome: JunctionId) {
    body.inputs(body.now() + 1, &[Arrival::caused(outcome, 1, 1)])
        .unwrap();
}

fn open_two_returns() -> (Body, JunctionId) {
    let mut body = Body::default();
    let mut outcomes = Vec::new();
    for index in 0..2_u64 {
        let motor = motor(&mut body);
        let surface = attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        );
        let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        schedule(&mut body, index * 4, &[reading(outcome, 0, 0, 0)]);
        finish(&mut body);
        schedule(
            &mut body,
            1 + index * 4,
            &[reading(surface, 0, 1, index + 1)],
        );
        schedule(
            &mut body,
            2 + index * 4,
            &[Arrival::caused(motor.opportunity, 1, index + 1)],
        );
        finish(&mut body);
        outcomes.push(outcome);
    }
    (body, outcomes[0])
}
