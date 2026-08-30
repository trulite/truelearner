use std::{hint::black_box, time::Instant};
use truelearner_body::{
    harness::{attach_outcome_component, attach_sensor, finish, motor, reading, schedule},
    Arrival, Body, Junction, JunctionId,
};

const WAVES: usize = 10_000;
const SAMPLES: usize = 9;

fn main() {
    sample(100);
    let mut samples = [0.0; SAMPLES];
    for result in &mut samples {
        *result = sample(WAVES);
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "waves_per_sample={WAVES} samples={SAMPLES} min_ns={:.2} median_ns={:.2} max_ns={:.2}",
        samples[0],
        samples[SAMPLES / 2],
        samples[SAMPLES - 1]
    );
}

fn sample(waves: usize) -> f64 {
    let (template, outcome) = open_two_returns();
    let mut bodies = vec![template; waves];
    let started = Instant::now();
    for body in &mut bodies {
        body.input(20, outcome, 1).unwrap();
        black_box(
            body.run(8, |event| {
                black_box(event);
            })
            .unwrap(),
        );
    }
    started.elapsed().as_nanos() as f64 / waves as f64
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
