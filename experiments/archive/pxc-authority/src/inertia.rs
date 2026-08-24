use std::collections::BTreeMap;

type SensorId = usize;
type WorldTime = u64;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Delta {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn translated(self, delta: Delta) -> Self {
        Self {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }
}

impl Delta {
    fn plus(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn minus(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_usize(&mut self, upper_bound: usize) -> usize {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.state >> 32) as usize) % upper_bound
    }
}

pub struct GridTopology {
    width: i32,
    height: i32,
    coordinate_to_sensor: Vec<SensorId>,
    sensor_to_coordinate: Vec<Point>,
}

impl GridTopology {
    pub fn permuted(width: i32, height: i32, seed: u64) -> Self {
        assert!(width > 0 && height > 0);
        let sensor_count = (width * height) as usize;
        let mut coordinate_to_sensor: Vec<_> = (0..sensor_count).collect();
        let mut rng = Lcg::new(seed);

        for index in (1..sensor_count).rev() {
            let swap_with = rng.next_usize(index + 1);
            coordinate_to_sensor.swap(index, swap_with);
        }

        let mut sensor_to_coordinate = vec![Point::default(); sensor_count];
        for y in 0..height {
            for x in 0..width {
                let coordinate_index = (y * width + x) as usize;
                let sensor = coordinate_to_sensor[coordinate_index];
                sensor_to_coordinate[sensor] = Point { x, y };
            }
        }

        Self {
            width,
            height,
            coordinate_to_sensor,
            sensor_to_coordinate,
        }
    }

    pub fn sensor_at(&self, point: Point) -> Option<SensorId> {
        if point.x < 0 || point.y < 0 || point.x >= self.width || point.y >= self.height {
            return None;
        }
        let coordinate_index = (point.y * self.width + point.x) as usize;
        Some(self.coordinate_to_sensor[coordinate_index])
    }

    pub fn point_of(&self, sensor: SensorId) -> Option<Point> {
        self.sensor_to_coordinate.get(sensor).copied()
    }

    pub fn sensor_count(&self) -> usize {
        self.sensor_to_coordinate.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Observation {
    pub time: WorldTime,
    pub sensor: SensorId,
}

fn observation(topology: &GridTopology, time: WorldTime, point: Point) -> Observation {
    Observation {
        time,
        sensor: topology
            .sensor_at(point)
            .expect("trajectory must stay inside the sensor surface"),
    }
}

fn velocity_between(
    topology: &GridTopology,
    first: Observation,
    second: Observation,
) -> Option<Delta> {
    let elapsed = second.time.checked_sub(first.time)?;
    if elapsed == 0 || elapsed > i32::MAX as u64 {
        return None;
    }

    let first_point = topology.point_of(first.sensor)?;
    let second_point = topology.point_of(second.sensor)?;
    let elapsed = elapsed as i32;
    let dx = second_point.x - first_point.x;
    let dy = second_point.y - first_point.y;
    if dx % elapsed != 0 || dy % elapsed != 0 {
        return None;
    }

    Some(Delta {
        x: dx / elapsed,
        y: dy / elapsed,
    })
}

#[derive(Clone, Copy, Debug)]
pub struct DynamicsConcept {
    pub acceleration: Delta,
    pub support: u32,
    pub confidence: f64,
}

impl DynamicsConcept {
    pub fn predict_point(
        &self,
        topology: &GridTopology,
        first: Observation,
        second: Observation,
        target_time: WorldTime,
    ) -> Option<Point> {
        if target_time <= second.time {
            return None;
        }

        let mut point = topology.point_of(second.sensor)?;
        let mut velocity = velocity_between(topology, first, second)?;
        for _ in (second.time + 1)..=target_time {
            velocity = velocity.plus(self.acceleration);
            point = point.translated(velocity);
        }
        topology.sensor_at(point)?;
        Some(point)
    }

    pub fn predict_sensor(
        &self,
        topology: &GridTopology,
        first: Observation,
        second: Observation,
        target_time: WorldTime,
    ) -> Option<SensorId> {
        let point = self.predict_point(topology, first, second, target_time)?;
        topology.sensor_at(point)
    }
}

#[derive(Default)]
pub struct DynamicsLearner {
    acceleration_counts: BTreeMap<Delta, u32>,
    total_accelerations: u32,
}

impl DynamicsLearner {
    pub fn observe_episode(&mut self, topology: &GridTopology, episode: &[Observation]) {
        let mut velocities = Vec::with_capacity(episode.len().saturating_sub(1));
        for pair in episode.windows(2) {
            let Some(velocity) = velocity_between(topology, pair[0], pair[1]) else {
                return;
            };
            velocities.push(velocity);
        }

        for pair in velocities.windows(2) {
            let acceleration = pair[1].minus(pair[0]);
            *self.acceleration_counts.entry(acceleration).or_default() += 1;
            self.total_accelerations += 1;
        }
    }

    pub fn discover(
        &self,
        minimum_support: u32,
        minimum_confidence: f64,
    ) -> Option<DynamicsConcept> {
        let (&acceleration, &support) = self
            .acceleration_counts
            .iter()
            .max_by_key(|(_, count)| *count)?;
        if self.total_accelerations == 0 {
            return None;
        }

        let confidence = support as f64 / self.total_accelerations as f64;
        if support < minimum_support || confidence < minimum_confidence {
            return None;
        }

        Some(DynamicsConcept {
            acceleration,
            support,
            confidence,
        })
    }
}

fn constant_velocity_episode(
    topology: &GridTopology,
    start: Point,
    velocity: Delta,
    length: usize,
) -> Vec<Observation> {
    (0..length)
        .map(|time| {
            let point = Point {
                x: start.x + velocity.x * time as i32,
                y: start.y + velocity.y * time as i32,
            };
            observation(topology, time as WorldTime, point)
        })
        .collect()
}

pub fn learned_inertia_concept() -> DynamicsConcept {
    let topology = GridTopology::permuted(10, 10, 0x51a7e);
    let velocities = [
        Delta { x: 1, y: 0 },
        Delta { x: -1, y: 0 },
        Delta { x: 0, y: 1 },
        Delta { x: 0, y: -1 },
    ];
    let mut learner = DynamicsLearner::default();

    for episode_index in 0..32 {
        let velocity = velocities[episode_index % velocities.len()];
        let offset = 1 + (episode_index % 7) as i32;
        let start = match velocity {
            Delta { x: 1, y: 0 } => Point { x: 1, y: offset },
            Delta { x: -1, y: 0 } => Point { x: 8, y: offset },
            Delta { x: 0, y: 1 } => Point { x: offset, y: 1 },
            Delta { x: 0, y: -1 } => Point { x: offset, y: 8 },
            _ => unreachable!(),
        };
        learner.observe_episode(
            &topology,
            &constant_velocity_episode(&topology, start, velocity, 6),
        );
    }

    learner
        .discover(64, 0.9)
        .expect("constant trajectories should produce a stable dynamics concept")
}

fn random_walk_episode(topology: &GridTopology, rng: &mut Lcg, length: usize) -> Vec<Observation> {
    let directions = [
        Delta { x: 1, y: 0 },
        Delta { x: -1, y: 0 },
        Delta { x: 0, y: 1 },
        Delta { x: 0, y: -1 },
    ];
    let mut point = Point { x: 5, y: 5 };
    let mut episode = vec![observation(topology, 0, point)];

    for time in 1..length {
        let mut direction = directions[rng.next_usize(directions.len())];
        let mut next = point.translated(direction);
        if topology.sensor_at(next).is_none() {
            direction = Delta {
                x: -direction.x,
                y: -direction.y,
            };
            next = point.translated(direction);
        }
        point = next;
        episode.push(observation(topology, time as WorldTime, point));
    }

    episode
}

fn random_walk_concept() -> Option<DynamicsConcept> {
    let topology = GridTopology::permuted(11, 11, 0xbadc0de);
    let mut learner = DynamicsLearner::default();
    let mut rng = Lcg::new(0xc0ffee);
    for _ in 0..128 {
        learner.observe_episode(&topology, &random_walk_episode(&topology, &mut rng, 12));
    }
    learner.discover(64, 0.75)
}

#[derive(Debug)]
pub struct TransferCase {
    pub name: &'static str,
    pub velocity: Delta,
    pub hidden_ticks: u64,
    pub predicted_point: Option<Point>,
    pub actual_point: Point,
    pub predicted_sensor: Option<SensorId>,
    pub actual_sensor: SensorId,
}

impl TransferCase {
    fn passed(&self) -> bool {
        self.predicted_point == Some(self.actual_point)
            && self.predicted_sensor == Some(self.actual_sensor)
    }
}

#[derive(Debug)]
pub struct InertiaReport {
    pub concept: DynamicsConcept,
    pub cases: Vec<TransferCase>,
    pub random_control_concept: Option<DynamicsConcept>,
    pub passed: bool,
}

fn transfer_case(
    concept: DynamicsConcept,
    topology: &GridTopology,
    name: &'static str,
    start: Point,
    velocity: Delta,
    target_time: WorldTime,
) -> TransferCase {
    let first = observation(topology, 0, start);
    let second_point = start.translated(velocity);
    let second = observation(topology, 1, second_point);
    let actual_point = Point {
        x: start.x + velocity.x * target_time as i32,
        y: start.y + velocity.y * target_time as i32,
    };
    let actual_sensor = topology
        .sensor_at(actual_point)
        .expect("held-out trajectory must remain on the sensor surface");

    TransferCase {
        name,
        velocity,
        hidden_ticks: target_time - second.time - 1,
        predicted_point: concept.predict_point(topology, first, second, target_time),
        actual_point,
        predicted_sensor: concept.predict_sensor(topology, first, second, target_time),
        actual_sensor,
    }
}

pub fn run_experiment() -> InertiaReport {
    let concept = learned_inertia_concept();
    let transfer_topology = GridTopology::permuted(20, 20, 0xdecafbad);
    let cases = vec![
        transfer_case(
            concept,
            &transfer_topology,
            "unseen diagonal speed",
            Point { x: 1, y: 2 },
            Delta { x: 2, y: 1 },
            6,
        ),
        transfer_case(
            concept,
            &transfer_topology,
            "unseen oblique reverse",
            Point { x: 15, y: 1 },
            Delta { x: -1, y: 2 },
            6,
        ),
    ];
    let random_control_concept = random_walk_concept();
    let passed = concept.acceleration == Delta::default()
        && concept.confidence >= 0.9
        && cases.iter().all(TransferCase::passed)
        && random_control_concept.is_none();

    InertiaReport {
        concept,
        cases,
        random_control_concept,
        passed,
    }
}

pub fn print_report(report: &InertiaReport) {
    println!("learned higher-order dynamics:");
    println!(
        "  acceleration=({}, {}), support={}, confidence={:.1}%",
        report.concept.acceleration.x,
        report.concept.acceleration.y,
        report.concept.support,
        report.concept.confidence * 100.0
    );
    for case in &report.cases {
        println!(
            "  {}: velocity=({}, {}), hidden_ticks={}, predicted={:?}, actual={:?}, opaque_sensor_match={}",
            case.name,
            case.velocity.x,
            case.velocity.y,
            case.hidden_ticks,
            case.predicted_point,
            case.actual_point,
            case.predicted_sensor == Some(case.actual_sensor)
        );
    }
    println!(
        "  random-walk control concept: {}",
        if report.random_control_concept.is_some() {
            "false positive"
        } else {
            "none"
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permuted_topology_round_trips_opaque_sensor_ids() {
        let topology = GridTopology::permuted(7, 5, 123);
        for y in 0..5 {
            for x in 0..7 {
                let point = Point { x, y };
                let sensor = topology.sensor_at(point).unwrap();
                assert_eq!(topology.point_of(sensor), Some(point));
            }
        }
    }

    #[test]
    fn constant_motion_discovers_zero_acceleration() {
        let concept = learned_inertia_concept();

        assert_eq!(concept.acceleration, Delta::default());
        assert_eq!(concept.support, 128);
        assert_eq!(concept.confidence, 1.0);
    }

    #[test]
    fn learned_inertia_transfers_to_new_grid_directions_speeds_and_occlusions() {
        let report = run_experiment();

        assert!(report.cases.iter().all(TransferCase::passed));
        assert!(report.passed);
    }

    #[test]
    fn random_walks_do_not_produce_an_inertia_concept() {
        assert!(random_walk_concept().is_none());
    }
}
