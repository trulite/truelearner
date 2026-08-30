use std::{hint::black_box, time::Instant};
use truelearner_body::{Body, Junction};

const WAVES: u64 = 5_000_000;
const SAMPLES: usize = 9;

fn main() {
    run(100_000);
    let mut samples = [0.0; SAMPLES];
    for sample in &mut samples {
        *sample = run(WAVES);
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "waves_per_sample={WAVES} samples={SAMPLES} min_ns={:.2} median_ns={:.2} max_ns={:.2}",
        samples[0],
        samples[SAMPLES / 2],
        samples[SAMPLES - 1]
    );
}

fn run(waves: u64) -> f64 {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(1)).unwrap();
    let started = Instant::now();
    for at in 0..waves {
        body.input(at, junction, 1).unwrap();
        black_box(
            body.run(1, |change| {
                black_box(change);
            })
            .unwrap(),
        );
    }
    let elapsed = started.elapsed();
    elapsed.as_nanos() as f64 / waves as f64
}
