use std::{hint::black_box, time::Instant};
use truelearner_body::{Body, Junction, Link};

const LINKS: usize = 1_024;
const EPISODES: u64 = 10_000;
const SAMPLES: usize = 7;

fn main() {
    sample(100);
    let mut samples = [0.0; SAMPLES];
    for result in &mut samples {
        *result = sample(EPISODES);
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "links={LINKS} episodes={EPISODES} samples={SAMPLES} min_ns={:.2} median_ns={:.2} max_ns={:.2}",
        samples[0],
        samples[SAMPLES / 2],
        samples[SAMPLES - 1]
    );
}

fn sample(episodes: u64) -> f64 {
    let mut body = Body::default();
    body.reserve(LINKS + 1, LINKS);
    let first = body.add_junction(Junction::integrating(1)).unwrap();
    let mut previous = first;
    for _ in 0..LINKS {
        let next = body.add_junction(Junction::integrating(1)).unwrap();
        body.add_link(Link::new(previous, next, 0, 1)).unwrap();
        previous = next;
    }

    let started = Instant::now();
    for at in 0..episodes {
        body.input(at, first, 1).unwrap();
        black_box(
            body.run(LINKS + 1, |change| {
                black_box(change);
            })
            .unwrap(),
        );
    }
    started.elapsed().as_nanos() as f64 / (episodes as f64 * LINKS as f64)
}
