use std::collections::VecDeque;
use std::fmt::Write as _;
use std::time::Instant;

use crate::generality::{
    simulate_effect, Frame, RelationalTopology, RepresentationLearner, StructuralEffect,
    WorkMetrics,
};

#[derive(Clone, Debug)]
pub struct ScalePoint {
    pub size: usize,
    pub work: u64,
    pub wall_nanoseconds: u128,
}

#[derive(Clone, Debug)]
pub struct CascadePoint {
    pub branching_ratio: f64,
    pub mean_spikes: f64,
    pub theoretical_mean: Option<f64>,
    pub relative_error: Option<f64>,
    pub runaway_fraction: f64,
}

#[derive(Clone, Debug)]
pub struct CapacityPoint {
    pub associations: usize,
    pub slots: usize,
    pub load: f64,
    pub accuracy: f64,
}

#[derive(Debug)]
pub struct ScalingReport {
    pub data_points: Vec<ScalePoint>,
    pub context_points: Vec<ScalePoint>,
    pub topology_points: Vec<ScalePoint>,
    pub data_exponent: f64,
    pub context_exponent: f64,
    pub topology_exponent: f64,
    pub cascade_points: Vec<CascadePoint>,
    pub capacity_points: Vec<CapacityPoint>,
    pub maximum_subcritical_error: f64,
    pub supercritical_runaway_fraction: f64,
    pub passed: bool,
}

impl ScalingReport {
    pub fn to_csv(&self) -> String {
        let mut output = String::from(
            "probe,size,work,wall_nanoseconds,branching_ratio,theoretical_mean,relative_error,runaway_fraction,slots,load,accuracy\n",
        );
        for point in &self.data_points {
            let _ = writeln!(
                output,
                "data,{},{},{},,,,,,,",
                point.size, point.work, point.wall_nanoseconds
            );
        }
        for point in &self.context_points {
            let _ = writeln!(
                output,
                "context,{},{},{},,,,,,,",
                point.size, point.work, point.wall_nanoseconds
            );
        }
        for point in &self.topology_points {
            let _ = writeln!(
                output,
                "topology,{},{},{},,,,,,,",
                point.size, point.work, point.wall_nanoseconds
            );
        }
        for point in &self.cascade_points {
            let _ = writeln!(
                output,
                "cascade,,,,{:.4},{},{},{:.6},,,",
                point.branching_ratio,
                point
                    .theoretical_mean
                    .map_or_else(String::new, |value| format!("{value:.6}")),
                point
                    .relative_error
                    .map_or_else(String::new, |value| format!("{value:.6}")),
                point.runaway_fraction
            );
        }
        for point in &self.capacity_points {
            let _ = writeln!(
                output,
                "capacity,{},,,,,,,{},{:.4},{:.6}",
                point.associations, point.slots, point.load, point.accuracy
            );
        }
        output
    }

    pub fn summary(&self) -> String {
        format!(
            "v14.5 scaling: data exponent={:.3}, context exponent={:.3}, topology exponent={:.3}, max subcritical cascade error={:.1}%, supercritical runaway={:.1}%, capacity accuracy {:.1}% -> {:.1}%, passed={}",
            self.data_exponent,
            self.context_exponent,
            self.topology_exponent,
            self.maximum_subcritical_error * 100.0,
            self.supercritical_runaway_fraction * 100.0,
            self.capacity_points.first().map_or(0.0, |point| point.accuracy) * 100.0,
            self.capacity_points.last().map_or(0.0, |point| point.accuracy) * 100.0,
            self.passed
        )
    }
}

fn total_work(metrics: WorkMetrics) -> u64 {
    metrics.hypothesis_evaluations + metrics.sensor_visits + metrics.experiment_states_considered
}

fn data_sweep() -> Vec<ScalePoint> {
    let topology = RelationalTopology::permuted_grid(16, 16, 0x1450_0001);
    let sensor = topology.interior_sensors()[0];
    let before = Frame::singleton(sensor);
    let effect = StructuralEffect::FollowPort(1);
    let after = simulate_effect(&topology, &before, effect);

    [256, 1_024, 4_096, 16_384]
        .into_iter()
        .map(|size| {
            let mut learner = RepresentationLearner::new([1], &topology);
            let mut metrics = WorkMetrics::default();
            let started = Instant::now();
            for _ in 0..size {
                assert!(learner.observe_measured(&topology, &before, 1, &after, &mut metrics));
            }
            assert_eq!(learner.effect(1), Some(effect));
            ScalePoint {
                size,
                work: total_work(metrics),
                wall_nanoseconds: started.elapsed().as_nanos(),
            }
        })
        .collect()
}

fn context_sweep() -> Vec<ScalePoint> {
    let topology = RelationalTopology::permuted_grid(64, 64, 0x1450_0002);
    let interior = topology.interior_sensors();
    let effect = StructuralEffect::FollowPort(1);

    [1, 4, 16, 64, 256]
        .into_iter()
        .map(|size| {
            let before = Frame::from_sensors(interior.iter().copied().take(size));
            let after = simulate_effect(&topology, &before, effect);
            let mut learner = RepresentationLearner::new([2], &topology);
            let mut metrics = WorkMetrics::default();
            let started = Instant::now();
            for _ in 0..1_024 {
                assert!(learner.observe_measured(&topology, &before, 2, &after, &mut metrics));
            }
            assert_eq!(learner.effect(2), Some(effect));
            ScalePoint {
                size,
                work: metrics.sensor_visits,
                wall_nanoseconds: started.elapsed().as_nanos(),
            }
        })
        .collect()
}

fn topology_sweep() -> Vec<ScalePoint> {
    [8, 16, 32, 64]
        .into_iter()
        .map(|side| {
            let topology = RelationalTopology::permuted_grid(side, side, 0x1450_1000 + side as u64);
            let learner = RepresentationLearner::new([3, 5, 7, 11, 13], &topology);
            let mut metrics = WorkMetrics::default();
            let started = Instant::now();
            assert!(learner
                .choose_experiment_measured(&topology, &mut metrics)
                .is_some());
            ScalePoint {
                size: topology.sensor_count(),
                work: total_work(metrics),
                wall_nanoseconds: started.elapsed().as_nanos(),
            }
        })
        .collect()
}

fn fit_power_exponent(points: &[ScalePoint]) -> f64 {
    let count = points.len() as f64;
    let mean_x = points
        .iter()
        .map(|point| (point.size as f64).ln())
        .sum::<f64>()
        / count;
    let mean_y = points
        .iter()
        .map(|point| (point.work as f64).ln())
        .sum::<f64>()
        / count;
    let covariance = points
        .iter()
        .map(|point| ((point.size as f64).ln() - mean_x) * ((point.work as f64).ln() - mean_y))
        .sum::<f64>();
    let variance = points
        .iter()
        .map(|point| ((point.size as f64).ln() - mean_x).powi(2))
        .sum::<f64>();
    covariance / variance
}

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_unit(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.state >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn cascade_point(branching_ratio: f64, trials: usize, spike_cap: usize) -> CascadePoint {
    const FANOUT: usize = 4;
    let probability = branching_ratio / FANOUT as f64;
    let mut rng = Lcg::new(0x1450_ca5c ^ branching_ratio.to_bits());
    let mut total_spikes = 0u64;
    let mut runaway = 0usize;

    for _ in 0..trials {
        let mut queue = VecDeque::from([()]);
        let mut processed = 0usize;
        while queue.pop_front().is_some() && processed < spike_cap {
            processed += 1;
            for _ in 0..FANOUT {
                if rng.next_unit() < probability {
                    queue.push_back(());
                }
            }
        }
        if !queue.is_empty() {
            runaway += 1;
        }
        total_spikes += processed as u64;
    }

    let mean_spikes = total_spikes as f64 / trials as f64;
    let theoretical_mean = (branching_ratio < 1.0).then_some(1.0 / (1.0 - branching_ratio));
    let relative_error = theoretical_mean.map(|theory| (mean_spikes - theory).abs() / theory);
    CascadePoint {
        branching_ratio,
        mean_spikes,
        theoretical_mean,
        relative_error,
        runaway_fraction: runaway as f64 / trials as f64,
    }
}

fn cascade_sweep() -> Vec<CascadePoint> {
    let mut points: Vec<_> = [0.25, 0.5, 0.75, 0.9]
        .into_iter()
        .map(|ratio| cascade_point(ratio, 20_000, 100_000))
        .collect();
    points.push(cascade_point(1.1, 2_000, 10_000));
    points
}

struct BoundedAssociativeMemory {
    slots: Vec<Option<(u64, u64)>>,
}

impl BoundedAssociativeMemory {
    fn new(capacity: usize) -> Self {
        Self {
            slots: vec![None; capacity],
        }
    }

    fn insert(&mut self, key: u64, value: u64) {
        let slot = mix64(key) as usize % self.slots.len();
        self.slots[slot] = Some((key, value));
    }

    fn get(&self, key: u64) -> Option<u64> {
        let slot = mix64(key) as usize % self.slots.len();
        self.slots[slot]
            .filter(|(stored_key, _)| *stored_key == key)
            .map(|(_, value)| value)
    }
}

fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn capacity_sweep() -> Vec<CapacityPoint> {
    const SLOTS: usize = 4_096;
    [(1, 4), (1, 2), (1, 1), (2, 1), (4, 1)]
        .into_iter()
        .map(|(numerator, denominator)| {
            let associations = SLOTS * numerator / denominator;
            let mut memory = BoundedAssociativeMemory::new(SLOTS);
            for key in 0..associations as u64 {
                memory.insert(key, mix64(key ^ 0x1450_5eed));
            }
            let correct = (0..associations as u64)
                .filter(|&key| memory.get(key) == Some(mix64(key ^ 0x1450_5eed)))
                .count();
            CapacityPoint {
                associations,
                slots: SLOTS,
                load: associations as f64 / SLOTS as f64,
                accuracy: correct as f64 / associations as f64,
            }
        })
        .collect()
}

pub fn run_experiment() -> ScalingReport {
    let data_points = data_sweep();
    let context_points = context_sweep();
    let topology_points = topology_sweep();
    let data_exponent = fit_power_exponent(&data_points);
    let context_exponent = fit_power_exponent(&context_points);
    let topology_exponent = fit_power_exponent(&topology_points);
    let cascade_points = cascade_sweep();
    let maximum_subcritical_error = cascade_points
        .iter()
        .filter_map(|point| point.relative_error)
        .fold(0.0, f64::max);
    let supercritical_runaway_fraction = cascade_points
        .iter()
        .find(|point| point.branching_ratio > 1.0)
        .map_or(0.0, |point| point.runaway_fraction);
    let capacity_points = capacity_sweep();
    let capacity_monotonic = capacity_points
        .windows(2)
        .all(|pair| pair[1].accuracy <= pair[0].accuracy);
    let passed = (0.98..=1.02).contains(&data_exponent)
        && (0.98..=1.02).contains(&context_exponent)
        && (0.98..=1.02).contains(&topology_exponent)
        && maximum_subcritical_error < 0.08
        && supercritical_runaway_fraction > 0.02
        && capacity_monotonic
        && capacity_points
            .first()
            .is_some_and(|point| point.accuracy > 0.8)
        && capacity_points
            .last()
            .is_some_and(|point| point.accuracy < 0.35);

    ScalingReport {
        data_points,
        context_points,
        topology_points,
        data_exponent,
        context_exponent,
        topology_exponent,
        cascade_points,
        capacity_points,
        maximum_subcritical_error,
        supercritical_runaway_fraction,
        passed,
    }
}

pub fn print_report(report: &ScalingReport) {
    println!("v14.5 scaling harness:");
    println!(
        "  deterministic work exponents: data={:.3}, active-context={:.3}, topology-search={:.3}",
        report.data_exponent, report.context_exponent, report.topology_exponent
    );
    println!(
        "  cascade model: max subcritical error={:.1}%, supercritical runaway={:.1}%",
        report.maximum_subcritical_error * 100.0,
        report.supercritical_runaway_fraction * 100.0
    );
    println!(
        "  bounded associative accuracy: {:.1}% at {:.2}x load -> {:.1}% at {:.2}x load",
        report
            .capacity_points
            .first()
            .map_or(0.0, |point| point.accuracy)
            * 100.0,
        report
            .capacity_points
            .first()
            .map_or(0.0, |point| point.load),
        report
            .capacity_points
            .last()
            .map_or(0.0, |point| point.accuracy)
            * 100.0,
        report
            .capacity_points
            .last()
            .map_or(0.0, |point| point.load)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v14_5_training_work_is_linear_in_data_context_and_topology() {
        let report = run_experiment();

        assert!((0.98..=1.02).contains(&report.data_exponent));
        assert!((0.98..=1.02).contains(&report.context_exponent));
        assert!((0.98..=1.02).contains(&report.topology_exponent));
    }

    #[test]
    fn v14_5_subcritical_cascades_match_theory_and_supercritical_cascades_run_away() {
        let report = run_experiment();

        assert!(report.maximum_subcritical_error < 0.08);
        assert!(report.supercritical_runaway_fraction > 0.02);
    }

    #[test]
    fn v14_5_associative_accuracy_exposes_a_capacity_knee() {
        let report = run_experiment();

        assert!(report.capacity_points[0].accuracy > 0.8);
        assert!(report.capacity_points.last().unwrap().accuracy < 0.35);
        assert!(report
            .capacity_points
            .windows(2)
            .all(|pair| pair[1].accuracy <= pair[0].accuracy));
    }

    #[test]
    fn v14_5_complete_scaling_experiment_passes() {
        assert!(run_experiment().passed);
    }
}
