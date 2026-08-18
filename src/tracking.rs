use crate::inertia::{DynamicsConcept, GridTopology, Observation, Point};

type TrackId = usize;
type SensorId = usize;
type WorldTime = u64;

#[derive(Clone, Debug)]
struct Track {
    id: TrackId,
    observations: Vec<Observation>,
}

impl Track {
    fn predicted_point(
        &self,
        topology: &GridTopology,
        concept: DynamicsConcept,
        time: WorldTime,
    ) -> Option<Point> {
        match self.observations.as_slice() {
            [] => None,
            [only] => topology.point_of(only.sensor),
            observations => {
                let first = observations[observations.len() - 2];
                let second = observations[observations.len() - 1];
                if time == second.time {
                    topology.point_of(second.sensor)
                } else {
                    concept.predict_point(topology, first, second, time)
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Candidate {
    assignments: Vec<Option<usize>>,
    cost: u32,
}

#[derive(Debug, Default)]
struct FrameUpdate {
    assigned_points: Vec<(TrackId, Point)>,
    unmatched_tracks: usize,
    unmatched_detections: usize,
    ambiguous: bool,
}

impl FrameUpdate {
    fn assigned_point(&self, track: TrackId) -> Option<Point> {
        self.assigned_points
            .iter()
            .find_map(|&(id, point)| (id == track).then_some(point))
    }
}

struct Tracker {
    concept: DynamicsConcept,
    tracks: Vec<Track>,
    max_assignment_distance: u32,
    miss_penalty: u32,
    unexplained_detection_penalty: u32,
    last_time: Option<WorldTime>,
}

impl Tracker {
    fn new(concept: DynamicsConcept) -> Self {
        Self {
            concept,
            tracks: Vec::new(),
            max_assignment_distance: 2,
            miss_penalty: 10,
            unexplained_detection_penalty: 10,
            last_time: None,
        }
    }

    fn update(
        &mut self,
        topology: &GridTopology,
        time: WorldTime,
        detections: &[SensorId],
    ) -> FrameUpdate {
        if let Some(previous_time) = self.last_time {
            assert!(time > previous_time);
        }
        self.last_time = Some(time);

        if self.tracks.is_empty() {
            return self.initialize(topology, time, detections);
        }

        let detection_points: Vec<_> = detections
            .iter()
            .map(|&sensor| {
                topology
                    .point_of(sensor)
                    .expect("detection must belong to this topology")
            })
            .collect();
        let predictions: Vec<_> = self
            .tracks
            .iter()
            .map(|track| track.predicted_point(topology, self.concept, time))
            .collect();
        let mut candidates = Vec::new();
        let mut used = vec![false; detections.len()];
        let mut assignments = vec![None; self.tracks.len()];
        self.enumerate_assignments(
            0,
            &predictions,
            &detection_points,
            &mut used,
            &mut assignments,
            0,
            &mut candidates,
        );

        let minimum_cost = candidates
            .iter()
            .map(|candidate| candidate.cost)
            .min()
            .expect("the all-missed assignment always exists");
        let mut best = candidates
            .into_iter()
            .filter(|candidate| candidate.cost == minimum_cost);
        let chosen = best.next().expect("minimum candidate must exist");
        if best.next().is_some() {
            return FrameUpdate {
                unmatched_tracks: self.tracks.len(),
                unmatched_detections: detections.len(),
                ambiguous: true,
                ..FrameUpdate::default()
            };
        }

        let mut update = FrameUpdate::default();
        let mut used_detections = vec![false; detections.len()];
        for (track_index, detection_index) in chosen.assignments.into_iter().enumerate() {
            let Some(detection_index) = detection_index else {
                update.unmatched_tracks += 1;
                continue;
            };
            used_detections[detection_index] = true;
            let sensor = detections[detection_index];
            let point = detection_points[detection_index];
            self.tracks[track_index]
                .observations
                .push(Observation { time, sensor });
            update
                .assigned_points
                .push((self.tracks[track_index].id, point));
        }
        update.unmatched_detections = used_detections.iter().filter(|&&used| !used).count();
        update
    }

    fn initialize(
        &mut self,
        topology: &GridTopology,
        time: WorldTime,
        detections: &[SensorId],
    ) -> FrameUpdate {
        let mut ordered: Vec<_> = detections
            .iter()
            .map(|&sensor| {
                (
                    topology
                        .point_of(sensor)
                        .expect("detection must belong to this topology"),
                    sensor,
                )
            })
            .collect();
        ordered.sort_by_key(|(point, _)| (point.y, point.x));

        let mut update = FrameUpdate::default();
        for (id, (point, sensor)) in ordered.into_iter().enumerate() {
            self.tracks.push(Track {
                id,
                observations: vec![Observation { time, sensor }],
            });
            update.assigned_points.push((id, point));
        }
        update
    }

    #[allow(clippy::too_many_arguments)]
    fn enumerate_assignments(
        &self,
        track_index: usize,
        predictions: &[Option<Point>],
        detections: &[Point],
        used: &mut [bool],
        assignments: &mut [Option<usize>],
        cost: u32,
        candidates: &mut Vec<Candidate>,
    ) {
        if track_index == self.tracks.len() {
            let unexplained = used.iter().filter(|&&is_used| !is_used).count() as u32;
            candidates.push(Candidate {
                assignments: assignments.to_vec(),
                cost: cost + unexplained * self.unexplained_detection_penalty,
            });
            return;
        }

        assignments[track_index] = None;
        self.enumerate_assignments(
            track_index + 1,
            predictions,
            detections,
            used,
            assignments,
            cost + self.miss_penalty,
            candidates,
        );

        let Some(predicted) = predictions[track_index] else {
            return;
        };
        for (detection_index, &detected) in detections.iter().enumerate() {
            if used[detection_index] {
                continue;
            }
            let distance = predicted.x.abs_diff(detected.x) + predicted.y.abs_diff(detected.y);
            if distance > self.max_assignment_distance {
                continue;
            }

            used[detection_index] = true;
            assignments[track_index] = Some(detection_index);
            self.enumerate_assignments(
                track_index + 1,
                predictions,
                detections,
                used,
                assignments,
                cost + distance,
                candidates,
            );
            used[detection_index] = false;
            assignments[track_index] = None;
        }
    }
}

fn sensors(topology: &GridTopology, points: &[Point], reverse: bool) -> Vec<SensorId> {
    let mut sensors: Vec<_> = points
        .iter()
        .map(|&point| {
            topology
                .sensor_at(point)
                .expect("scenario point must be inside the topology")
        })
        .collect();
    if reverse {
        sensors.reverse();
    }
    sensors
}

fn occlusion_reassociation(concept: DynamicsConcept) -> bool {
    let topology = GridTopology::permuted(20, 20, 0x0cc1_0ded);
    let mut tracker = Tracker::new(concept);

    for time in 0..=2 {
        let points = [
            Point { x: 1 + time, y: 2 },
            Point {
                x: 15 - time,
                y: 10,
            },
        ];
        tracker.update(
            &topology,
            time as WorldTime,
            &sensors(&topology, &points, time % 2 == 0),
        );
    }
    for time in 3..=4 {
        let visible = [Point {
            x: 15 - time,
            y: 10,
        }];
        tracker.update(
            &topology,
            time as WorldTime,
            &sensors(&topology, &visible, false),
        );
    }

    let reappearance = [Point { x: 6, y: 2 }, Point { x: 10, y: 10 }];
    let update = tracker.update(&topology, 5, &sensors(&topology, &reappearance, true));
    !update.ambiguous
        && update.assigned_point(0) == Some(reappearance[0])
        && update.assigned_point(1) == Some(reappearance[1])
}

fn crossing_preserves_identity(concept: DynamicsConcept) -> bool {
    let topology = GridTopology::permuted(8, 4, 0xc20551);
    let mut tracker = Tracker::new(concept);
    let frames = [
        [Point { x: 1, y: 1 }, Point { x: 4, y: 1 }],
        [Point { x: 2, y: 1 }, Point { x: 3, y: 1 }],
        [Point { x: 3, y: 1 }, Point { x: 2, y: 1 }],
    ];

    let mut update = FrameUpdate::default();
    for (time, points) in frames.iter().enumerate() {
        update = tracker.update(
            &topology,
            time as WorldTime,
            &sensors(&topology, points, time % 2 == 1),
        );
    }

    !update.ambiguous
        && update.assigned_point(0) == Some(frames[2][0])
        && update.assigned_point(1) == Some(frames[2][1])
}

fn three_object_reassociation(concept: DynamicsConcept) -> bool {
    let topology = GridTopology::permuted(20, 20, 0x3b1ec7);
    let mut tracker = Tracker::new(concept);
    let frame0 = [
        Point { x: 1, y: 1 },
        Point { x: 10, y: 1 },
        Point { x: 1, y: 10 },
    ];
    let frame1 = [
        Point { x: 2, y: 1 },
        Point { x: 10, y: 2 },
        Point { x: 2, y: 11 },
    ];
    let frame2 = [Point { x: 3, y: 1 }, Point { x: 3, y: 12 }];
    let frame3 = [
        Point { x: 4, y: 1 },
        Point { x: 10, y: 4 },
        Point { x: 4, y: 13 },
    ];

    tracker.update(&topology, 0, &sensors(&topology, &frame0, true));
    tracker.update(&topology, 1, &sensors(&topology, &frame1, false));
    tracker.update(&topology, 2, &sensors(&topology, &frame2, true));
    let update = tracker.update(&topology, 3, &sensors(&topology, &frame3, true));

    !update.ambiguous
        && update.assigned_point(0) == Some(frame3[0])
        && update.assigned_point(1) == Some(frame3[1])
        && update.assigned_point(2) == Some(frame3[2])
}

fn symmetric_case_reports_ambiguity(concept: DynamicsConcept) -> bool {
    let topology = GridTopology::permuted(6, 4, 0xa8b190);
    let mut tracker = Tracker::new(concept);
    let initial = [Point { x: 1, y: 1 }, Point { x: 3, y: 1 }];
    let symmetric = [Point { x: 2, y: 0 }, Point { x: 2, y: 2 }];

    tracker.update(&topology, 0, &sensors(&topology, &initial, true));
    tracker
        .update(&topology, 1, &sensors(&topology, &symmetric, false))
        .ambiguous
}

fn teleports_remain_unmatched(concept: DynamicsConcept) -> bool {
    let topology = GridTopology::permuted(14, 14, 0x7e1e907);
    let mut tracker = Tracker::new(concept);
    let frame0 = [Point { x: 1, y: 1 }, Point { x: 5, y: 5 }];
    let frame1 = [Point { x: 2, y: 1 }, Point { x: 4, y: 5 }];
    let teleports = [Point { x: 12, y: 12 }, Point { x: 0, y: 12 }];

    tracker.update(&topology, 0, &sensors(&topology, &frame0, false));
    tracker.update(&topology, 1, &sensors(&topology, &frame1, true));
    let update = tracker.update(&topology, 2, &sensors(&topology, &teleports, false));

    !update.ambiguous
        && update.assigned_points.is_empty()
        && update.unmatched_tracks == 2
        && update.unmatched_detections == 2
}

#[derive(Debug)]
pub struct TrackingReport {
    pub occlusion_reassociation: bool,
    pub crossing_identity: bool,
    pub three_object_reassociation: bool,
    pub ambiguity_reported: bool,
    pub teleports_rejected: bool,
    pub passed: bool,
}

pub fn run_experiment(concept: DynamicsConcept) -> TrackingReport {
    let occlusion_reassociation = occlusion_reassociation(concept);
    let crossing_identity = crossing_preserves_identity(concept);
    let three_object_reassociation = three_object_reassociation(concept);
    let ambiguity_reported = symmetric_case_reports_ambiguity(concept);
    let teleports_rejected = teleports_remain_unmatched(concept);
    let passed = occlusion_reassociation
        && crossing_identity
        && three_object_reassociation
        && ambiguity_reported
        && teleports_rejected;

    TrackingReport {
        occlusion_reassociation,
        crossing_identity,
        three_object_reassociation,
        ambiguity_reported,
        teleports_rejected,
        passed,
    }
}

pub fn print_report(report: &TrackingReport) {
    println!("persistent multi-object tracking:");
    println!(
        "  two-object occlusion reassociation: {}",
        report.occlusion_reassociation
    );
    println!(
        "  crossing identity preserved: {}",
        report.crossing_identity
    );
    println!(
        "  three-object occlusion reassociation: {}",
        report.three_object_reassociation
    );
    println!(
        "  fundamentally symmetric case marked ambiguous: {}",
        report.ambiguity_reported
    );
    println!(
        "  teleport detections rejected as unrelated: {}",
        report.teleports_rejected
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inertia::learned_inertia_concept;

    #[test]
    fn reappearance_is_bound_to_the_original_track() {
        assert!(occlusion_reassociation(learned_inertia_concept()));
    }

    #[test]
    fn velocity_context_preserves_identity_during_a_crossing() {
        assert!(crossing_preserves_identity(learned_inertia_concept()));
    }

    #[test]
    fn three_objects_survive_one_objects_occlusion() {
        assert!(three_object_reassociation(learned_inertia_concept()));
    }

    #[test]
    fn symmetric_evidence_is_reported_as_ambiguous() {
        assert!(symmetric_case_reports_ambiguity(learned_inertia_concept()));
    }

    #[test]
    fn implausible_teleports_are_not_forced_into_existing_tracks() {
        assert!(teleports_remain_unmatched(learned_inertia_concept()));
    }
}
