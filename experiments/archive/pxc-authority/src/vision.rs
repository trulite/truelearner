use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::inertia::{GridTopology, Point};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ShapeKey {
    points: Vec<Point>,
}

impl ShapeKey {
    fn from_points(points: &[Point]) -> Self {
        let minimum_x = points.iter().map(|point| point.x).min().unwrap_or(0);
        let minimum_y = points.iter().map(|point| point.y).min().unwrap_or(0);
        let mut normalized: Vec<_> = points
            .iter()
            .map(|point| Point {
                x: point.x - minimum_x,
                y: point.y - minimum_y,
            })
            .collect();
        normalized.sort();
        normalized.dedup();
        Self { points: normalized }
    }

    fn similarity(&self, other: &Self) -> f64 {
        let intersection = self
            .points
            .iter()
            .filter(|point| other.points.binary_search(point).is_ok())
            .count();
        let union = self.points.len() + other.points.len() - intersection;
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }
}

#[derive(Clone, Debug)]
struct Region {
    anchor: Point,
    shape: ShapeKey,
}

#[derive(Clone, Debug)]
struct RawFrame {
    pixels: Vec<u8>,
}

impl RawFrame {
    fn blank(topology: &GridTopology) -> Self {
        Self {
            pixels: vec![0; topology.sensor_count()],
        }
    }

    fn paint(&mut self, topology: &GridTopology, shape: &ShapeKey, origin: Point) {
        for relative in &shape.points {
            let point = Point {
                x: origin.x + relative.x,
                y: origin.y + relative.y,
            };
            let sensor = topology
                .sensor_at(point)
                .expect("painted shape must stay inside the sensor surface");
            self.pixels[sensor] = 1;
        }
    }

    fn active(&self, sensor: usize) -> bool {
        self.pixels.get(sensor).copied().unwrap_or(0) != 0
    }
}

fn connected_regions(topology: &GridTopology, frame: &RawFrame) -> Vec<Region> {
    let mut remaining = BTreeSet::new();
    for sensor in 0..topology.sensor_count() {
        if frame.active(sensor) {
            remaining.insert(
                topology
                    .point_of(sensor)
                    .expect("sensor must belong to topology"),
            );
        }
    }

    let mut regions = Vec::new();
    while let Some(start) = remaining.pop_first() {
        let mut queue = VecDeque::from([start]);
        let mut points = vec![start];

        while let Some(point) = queue.pop_front() {
            let neighbors = [
                Point {
                    x: point.x - 1,
                    y: point.y,
                },
                Point {
                    x: point.x + 1,
                    y: point.y,
                },
                Point {
                    x: point.x,
                    y: point.y - 1,
                },
                Point {
                    x: point.x,
                    y: point.y + 1,
                },
            ];
            for neighbor in neighbors {
                if remaining.remove(&neighbor) {
                    points.push(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        let anchor = Point {
            x: points.iter().map(|point| point.x).min().unwrap(),
            y: points.iter().map(|point| point.y).min().unwrap(),
        };
        regions.push(Region {
            anchor,
            shape: ShapeKey::from_points(&points),
        });
    }
    regions
}

#[derive(Clone, Debug)]
struct VisualConcept {
    shape: ShapeKey,
    support: u32,
}

#[derive(Default)]
struct VisualLearner {
    occurrences: BTreeMap<ShapeKey, u32>,
}

impl VisualLearner {
    fn observe(&mut self, topology: &GridTopology, frame: &RawFrame) {
        for region in connected_regions(topology, frame) {
            if (5..=24).contains(&region.shape.points.len()) {
                *self.occurrences.entry(region.shape).or_default() += 1;
            }
        }
    }

    fn discover(&self, minimum_support: u32) -> Vec<VisualConcept> {
        self.occurrences
            .iter()
            .filter(|(_, support)| **support >= minimum_support)
            .map(|(shape, &support)| VisualConcept {
                shape: shape.clone(),
                support,
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Recognition {
    concept_index: usize,
    anchor: Point,
    similarity: f64,
}

fn recognize(
    topology: &GridTopology,
    frame: &RawFrame,
    concepts: &[VisualConcept],
    minimum_similarity: f64,
) -> Vec<Recognition> {
    let mut recognitions = Vec::new();
    for region in connected_regions(topology, frame) {
        let mut scored: Vec<_> = concepts
            .iter()
            .enumerate()
            .map(|(index, concept)| (index, region.shape.similarity(&concept.shape)))
            .collect();
        scored.sort_by(|left, right| right.1.total_cmp(&left.1));
        let Some(&(concept_index, similarity)) = scored.first() else {
            continue;
        };
        let tied = scored
            .get(1)
            .is_some_and(|(_, second)| (*second - similarity).abs() < f64::EPSILON);
        if similarity >= minimum_similarity && !tied {
            recognitions.push(Recognition {
                concept_index,
                anchor: region.anchor,
                similarity,
            });
        }
    }
    recognitions
}

fn rectangle_shape() -> ShapeKey {
    ShapeKey::from_points(&[
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
        Point { x: 2, y: 1 },
    ])
}

fn l_shape() -> ShapeKey {
    ShapeKey::from_points(&[
        Point { x: 0, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: 2 },
        Point { x: 1, y: 2 },
        Point { x: 2, y: 2 },
    ])
}

fn partial_rectangle_shape() -> ShapeKey {
    ShapeKey::from_points(&[
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
    ])
}

fn novel_t_shape() -> ShapeKey {
    ShapeKey::from_points(&[
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 1, y: 2 },
    ])
}

fn training_concepts() -> Vec<VisualConcept> {
    let topology = GridTopology::permuted(12, 10, 0x51a9e);
    let rectangle = rectangle_shape();
    let l_shape = l_shape();
    let mut learner = VisualLearner::default();

    for frame_index in 0..32 {
        let mut frame = RawFrame::blank(&topology);
        if frame_index % 11 != 0 {
            frame.paint(
                &topology,
                &rectangle,
                Point {
                    x: 1 + (frame_index % 3),
                    y: 1 + ((frame_index / 3) % 2),
                },
            );
        }
        if frame_index % 13 != 0 {
            frame.paint(
                &topology,
                &l_shape,
                Point {
                    x: 7 + (frame_index % 2),
                    y: 4 + ((frame_index / 2) % 2),
                },
            );
        }
        learner.observe(&topology, &frame);
    }

    learner.discover(20)
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

fn noise_concepts() -> Vec<VisualConcept> {
    let topology = GridTopology::permuted(12, 10, 0x9015e);
    let mut learner = VisualLearner::default();
    let mut rng = Lcg::new(0xdecafbad);

    for _ in 0..96 {
        let mut frame = RawFrame::blank(&topology);
        for sensor in 0..topology.sensor_count() {
            frame.pixels[sensor] = (rng.next_usize(100) < 18) as u8;
        }
        learner.observe(&topology, &frame);
    }
    learner.discover(20)
}

#[derive(Debug)]
pub struct VisionReport {
    pub concept_count: usize,
    pub rectangle_support: u32,
    pub l_support: u32,
    pub translated_recognitions: usize,
    pub partial_occlusion_recognized: bool,
    pub novel_shape_rejected: bool,
    pub noise_concept_count: usize,
    pub passed: bool,
}

pub fn run_experiment() -> VisionReport {
    let concepts = training_concepts();
    let rectangle = rectangle_shape();
    let l_shape = l_shape();
    let rectangle_support = concepts
        .iter()
        .find(|concept| concept.shape == rectangle)
        .map_or(0, |concept| concept.support);
    let l_support = concepts
        .iter()
        .find(|concept| concept.shape == l_shape)
        .map_or(0, |concept| concept.support);

    let transfer_topology = GridTopology::permuted(18, 14, 0x7a11_5fed);
    let mut translated = RawFrame::blank(&transfer_topology);
    translated.paint(&transfer_topology, &rectangle, Point { x: 12, y: 2 });
    translated.paint(&transfer_topology, &l_shape, Point { x: 2, y: 8 });
    let translated_results = recognize(&transfer_topology, &translated, &concepts, 0.8);
    let translated_recognitions = translated_results
        .iter()
        .filter(|recognition| {
            recognition.similarity == 1.0
                && concepts
                    .get(recognition.concept_index)
                    .is_some_and(|concept| concept.shape == rectangle || concept.shape == l_shape)
        })
        .count();

    let mut partial = RawFrame::blank(&transfer_topology);
    partial.paint(
        &transfer_topology,
        &partial_rectangle_shape(),
        Point { x: 7, y: 6 },
    );
    let partial_results = recognize(&transfer_topology, &partial, &concepts, 0.8);
    let partial_occlusion_recognized = partial_results.iter().any(|recognition| {
        recognition.anchor == Point { x: 7, y: 6 }
            && concepts
                .get(recognition.concept_index)
                .is_some_and(|concept| concept.shape == rectangle)
    });

    let mut novel = RawFrame::blank(&transfer_topology);
    novel.paint(&transfer_topology, &novel_t_shape(), Point { x: 5, y: 5 });
    let novel_shape_rejected = recognize(&transfer_topology, &novel, &concepts, 0.8).is_empty();
    let noise_concept_count = noise_concepts().len();
    let passed = concepts.len() == 2
        && rectangle_support >= 20
        && l_support >= 20
        && translated_recognitions == 2
        && partial_occlusion_recognized
        && novel_shape_rejected
        && noise_concept_count == 0;

    VisionReport {
        concept_count: concepts.len(),
        rectangle_support,
        l_support,
        translated_recognitions,
        partial_occlusion_recognized,
        novel_shape_rejected,
        noise_concept_count,
        passed,
    }
}

pub fn print_report(report: &VisionReport) {
    println!("raw-frame object-template discovery:");
    println!(
        "  recurring concepts discovered: {} (supports: {}, {})",
        report.concept_count, report.rectangle_support, report.l_support
    );
    println!(
        "  exact recognition on new grid and sensor permutation: {}/2",
        report.translated_recognitions
    );
    println!(
        "  partial-occlusion recognition: {}",
        report.partial_occlusion_recognized
    );
    println!("  novel shape rejected: {}", report.novel_shape_rejected);
    println!(
        "  high-support concepts from random visual noise: {}",
        report.noise_concept_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_raw_regions_become_two_unlabeled_shape_concepts() {
        let concepts = training_concepts();

        assert_eq!(concepts.len(), 2);
        assert!(concepts
            .iter()
            .any(|concept| concept.shape == rectangle_shape()));
        assert!(concepts.iter().any(|concept| concept.shape == l_shape()));
    }

    #[test]
    fn templates_transfer_across_position_grid_size_and_sensor_ids() {
        let report = run_experiment();

        assert_eq!(report.translated_recognitions, 2);
        assert!(report.passed);
    }

    #[test]
    fn partial_object_is_recognized_but_novel_shape_is_rejected() {
        let report = run_experiment();

        assert!(report.partial_occlusion_recognized);
        assert!(report.novel_shape_rejected);
    }

    #[test]
    fn random_visual_noise_does_not_become_a_stable_object_template() {
        assert!(noise_concepts().is_empty());
    }
}
