use std::{hint::black_box, time::Instant};
use truelearner_body::{Body, Junction, Link};

const JUNCTIONS: usize = 500_000;
const SAMPLES: usize = 7;

fn main() {
    grow(10_000, true);
    let unreserved = samples(false);
    let reserved = samples(true);
    println!(
        "junctions={JUNCTIONS} links={} samples={SAMPLES} unreserved_median_ns={:.2} reserved_median_ns={:.2}",
        JUNCTIONS - 1,
        unreserved[SAMPLES / 2],
        reserved[SAMPLES / 2]
    );
}

fn samples(reserve: bool) -> [f64; SAMPLES] {
    let mut results = [0.0; SAMPLES];
    for result in &mut results {
        *result = grow(JUNCTIONS, reserve);
    }
    results.sort_by(f64::total_cmp);
    results
}

fn grow(junctions: usize, reserve: bool) -> f64 {
    let mut body = Body::default();
    if reserve {
        body.reserve(junctions, junctions.saturating_sub(1));
    }
    let started = Instant::now();
    let mut previous = body.add_junction(Junction::integrating(1)).unwrap();
    for _ in 1..junctions {
        let next = body.add_junction(Junction::integrating(1)).unwrap();
        body.add_link(Link::new(previous, next, 0, 1)).unwrap();
        previous = next;
    }
    black_box(body);
    started.elapsed().as_nanos() as f64 / junctions as f64
}
