use std::collections::{BTreeMap, BTreeSet, VecDeque};

type CellId = usize;
type ConnectionId = usize;

const ACTIVE_STRENGTH: f64 = 0.2;
const COMPRESSION_SUPPORT: u32 = 3;

#[derive(Clone, Copy, Debug)]
enum LearningMode {
    PredictionOnly,
    ActivityOnly,
    Combined,
}

#[derive(Clone, Debug)]
struct Connection {
    from: CellId,
    to: CellId,
    strength: f64,
    minimum_source_visit: usize,
    compressed: bool,
}

#[derive(Clone, Debug)]
struct Event {
    cell: CellId,
    path: Vec<ConnectionId>,
}

#[derive(Clone, Debug)]
struct Episode {
    correct: bool,
    spikes: usize,
    runaway: bool,
    connection_uses: Vec<usize>,
    first_success_path: Vec<ConnectionId>,
}

#[derive(Clone, Copy, Debug)]
struct Route {
    cue: CellId,
    output: CellId,
}

struct StabilizingNetwork {
    connections: Vec<Connection>,
    outgoing: Vec<Vec<ConnectionId>>,
    output_cells: BTreeSet<CellId>,
    successful_routes: BTreeMap<(CellId, CellId), u32>,
    spike_limit: usize,
}

impl StabilizingNetwork {
    fn new(cell_count: usize, output_cells: impl IntoIterator<Item = CellId>) -> Self {
        Self {
            connections: Vec::new(),
            outgoing: vec![Vec::new(); cell_count],
            output_cells: output_cells.into_iter().collect(),
            successful_routes: BTreeMap::new(),
            spike_limit: 256,
        }
    }

    fn add_connection(
        &mut self,
        from: CellId,
        to: CellId,
        strength: f64,
        minimum_source_visit: usize,
        compressed: bool,
    ) -> ConnectionId {
        let id = self.connections.len();
        self.connections.push(Connection {
            from,
            to,
            strength,
            minimum_source_visit,
            compressed,
        });
        self.outgoing[from].push(id);
        id
    }

    fn has_compressed_route(&self, cue: CellId, output: CellId) -> bool {
        self.connections.iter().any(|connection| {
            connection.from == cue
                && connection.to == output
                && connection.compressed
                && connection.strength >= ACTIVE_STRENGTH
        })
    }

    fn compressed_count(&self) -> usize {
        self.connections
            .iter()
            .filter(|connection| connection.compressed && connection.strength >= ACTIVE_STRENGTH)
            .count()
    }

    fn run(&self, route: Route) -> Episode {
        let mut queue = VecDeque::from([Event {
            cell: route.cue,
            path: Vec::new(),
        }]);
        let mut cell_visits = vec![0usize; self.outgoing.len()];
        let mut connection_uses = vec![0usize; self.connections.len()];
        let mut first_success_path = Vec::new();
        let mut prediction = None;
        let mut spikes = 0;

        while let Some(event) = queue.pop_front() {
            if spikes == self.spike_limit {
                queue.push_front(event);
                break;
            }
            spikes += 1;
            cell_visits[event.cell] += 1;

            if prediction.is_none() && self.output_cells.contains(&event.cell) {
                prediction = Some(event.cell);
                if event.cell == route.output {
                    first_success_path = event.path.clone();
                }
            }

            let mut outgoing = self.outgoing[event.cell].clone();
            outgoing.sort_by(|left, right| {
                self.connections[*right]
                    .strength
                    .total_cmp(&self.connections[*left].strength)
                    .then_with(|| left.cmp(right))
            });
            for connection_id in outgoing {
                let connection = &self.connections[connection_id];
                if connection.strength < ACTIVE_STRENGTH
                    || cell_visits[event.cell] < connection.minimum_source_visit
                {
                    continue;
                }
                connection_uses[connection_id] += 1;
                let mut path = event.path.clone();
                path.push(connection_id);
                queue.push_back(Event {
                    cell: connection.to,
                    path,
                });
            }
        }

        Episode {
            correct: prediction == Some(route.output),
            spikes,
            runaway: !queue.is_empty(),
            connection_uses,
            first_success_path,
        }
    }

    fn train(&mut self, route: Route, mode: LearningMode) -> Episode {
        let episode = self.run(route);
        if episode.correct && !matches!(mode, LearningMode::ActivityOnly) {
            let support = self
                .successful_routes
                .entry((route.cue, route.output))
                .or_default();
            *support += 1;
            if *support >= COMPRESSION_SUPPORT
                && !self.has_compressed_route(route.cue, route.output)
            {
                self.add_connection(route.cue, route.output, 2.0, 1, true);
            }
        }

        let mut path_uses = vec![0usize; self.connections.len()];
        for &connection_id in &episode.first_success_path {
            if connection_id < path_uses.len() {
                path_uses[connection_id] += 1;
            }
        }
        let compressed_path_used = episode
            .first_success_path
            .iter()
            .any(|&connection_id| self.connections[connection_id].compressed);

        for (connection_id, &uses) in episode.connection_uses.iter().enumerate() {
            if uses == 0 {
                continue;
            }
            let successful_uses = path_uses[connection_id];
            let connection = &mut self.connections[connection_id];
            match mode {
                LearningMode::PredictionOnly => {
                    if episode.correct && successful_uses > 0 {
                        connection.strength += 0.03 * successful_uses as f64;
                    }
                }
                LearningMode::ActivityOnly => {
                    connection.strength -= 0.25 * uses.min(4) as f64;
                }
                LearningMode::Combined => {
                    if episode.correct && successful_uses > 0 {
                        connection.strength += 0.03 * successful_uses as f64;
                    }
                    let repeated_uses = uses.saturating_sub(successful_uses);
                    connection.strength -= 0.002 * repeated_uses as f64;
                    if compressed_path_used && successful_uses == 0 {
                        connection.strength -= 0.35;
                    }
                }
            }
            connection.strength = connection.strength.clamp(-1.0, 3.0);
        }
        episode
    }
}

fn build_network(route_count: usize) -> (StabilizingNetwork, Vec<Route>) {
    let cells_per_route = 6;
    let spare_cells = 2;
    let cell_count = route_count * cells_per_route + spare_cells;
    let routes: Vec<_> = (0..route_count)
        .map(|index| {
            let base = index * cells_per_route;
            Route {
                cue: base,
                output: base + 3,
            }
        })
        .collect();
    let mut network = StabilizingNetwork::new(cell_count, routes.iter().map(|route| route.output));

    for (index, route) in routes.iter().enumerate() {
        let base = index * cells_per_route;
        let first = base + 1;
        let second = base + 2;
        let noise_first = base + 4;
        let noise_second = base + 5;

        network.add_connection(route.cue, first, 1.0, 1, false);
        network.add_connection(first, second, 1.0, 1, false);
        network.add_connection(second, first, 1.0, 1, false);
        network.add_connection(second, route.output, 1.0, 2, false);

        network.add_connection(route.cue, noise_first, 0.8, 1, false);
        network.add_connection(noise_first, noise_second, 1.0, 1, false);
        network.add_connection(noise_second, noise_first, 1.0, 1, false);
    }
    (network, routes)
}

fn train_routes(
    network: &mut StabilizingNetwork,
    routes: &[Route],
    mode: LearningMode,
    rounds: usize,
) -> usize {
    let mut spikes = 0;
    for _ in 0..rounds {
        for &route in routes {
            spikes += network.train(route, mode).spikes;
        }
    }
    spikes
}

fn evaluate_routes(network: &StabilizingNetwork, routes: &[Route]) -> (f64, f64, usize) {
    let episodes: Vec<_> = routes.iter().map(|&route| network.run(route)).collect();
    let accuracy =
        episodes.iter().filter(|episode| episode.correct).count() as f64 / episodes.len() as f64;
    let runaway_rate =
        episodes.iter().filter(|episode| episode.runaway).count() as f64 / episodes.len() as f64;
    let mut spike_counts: Vec<_> = episodes.iter().map(|episode| episode.spikes).collect();
    spike_counts.sort_unstable();
    let median_spikes = spike_counts[spike_counts.len() / 2];
    (accuracy, runaway_rate, median_spikes)
}

fn perturb_and_recover(
    network: &mut StabilizingNetwork,
    route: Route,
    first_spare_cell: CellId,
) -> (bool, usize, bool) {
    let second_spare_cell = first_spare_cell + 1;
    network.add_connection(route.cue, first_spare_cell, 1.0, 1, false);
    network.add_connection(first_spare_cell, second_spare_cell, 1.0, 1, false);
    network.add_connection(second_spare_cell, first_spare_cell, 1.0, 1, false);
    let destabilized = network.run(route).runaway;

    for sample in 1..=8 {
        network.train(route, LearningMode::Combined);
        let episode = network.run(route);
        if episode.correct && !episode.runaway {
            return (destabilized, sample, true);
        }
    }
    (destabilized, 8, false)
}

#[derive(Debug)]
pub struct StabilityReport {
    pub prediction_only_accuracy: f64,
    pub prediction_only_runaway_rate: f64,
    pub activity_only_accuracy: f64,
    pub activity_only_runaway_rate: f64,
    pub combined_accuracy: f64,
    pub combined_runaway_rate: f64,
    pub initial_median_spikes: usize,
    pub final_median_spikes: usize,
    pub compressed_concepts: usize,
    pub destabilized_by_new_loop: bool,
    pub recovery_samples: usize,
    pub recovered_after_perturbation: bool,
    pub passed: bool,
}

pub fn run_experiment() -> StabilityReport {
    let (initial_network, routes) = build_network(2);
    let (_, initial_runaway_rate, initial_median_spikes) =
        evaluate_routes(&initial_network, &routes);
    assert_eq!(initial_runaway_rate, 1.0);

    let (mut prediction_only, _) = build_network(2);
    train_routes(
        &mut prediction_only,
        &routes,
        LearningMode::PredictionOnly,
        10,
    );
    let (prediction_only_accuracy, prediction_only_runaway_rate, _) =
        evaluate_routes(&prediction_only, &routes);

    let (mut activity_only, _) = build_network(2);
    train_routes(&mut activity_only, &routes, LearningMode::ActivityOnly, 10);
    let (activity_only_accuracy, activity_only_runaway_rate, _) =
        evaluate_routes(&activity_only, &routes);

    let (mut combined, _) = build_network(2);
    train_routes(&mut combined, &routes, LearningMode::Combined, 12);
    let (combined_accuracy, combined_runaway_rate, final_median_spikes) =
        evaluate_routes(&combined, &routes);
    let compressed_concepts = combined.compressed_count();
    let first_spare_cell = routes.len() * 6;
    let (destabilized_by_new_loop, recovery_samples, recovered_after_perturbation) =
        perturb_and_recover(&mut combined, routes[0], first_spare_cell);

    let passed = prediction_only_accuracy == 1.0
        && prediction_only_runaway_rate == 1.0
        && activity_only_accuracy == 0.0
        && activity_only_runaway_rate == 0.0
        && combined_accuracy == 1.0
        && combined_runaway_rate == 0.0
        && final_median_spikes <= 3
        && initial_median_spikes >= 128
        && compressed_concepts == routes.len()
        && destabilized_by_new_loop
        && recovered_after_perturbation
        && recovery_samples <= 4;

    StabilityReport {
        prediction_only_accuracy,
        prediction_only_runaway_rate,
        activity_only_accuracy,
        activity_only_runaway_rate,
        combined_accuracy,
        combined_runaway_rate,
        initial_median_spikes,
        final_median_spikes,
        compressed_concepts,
        destabilized_by_new_loop,
        recovery_samples,
        recovered_after_perturbation,
        passed,
    }
}

#[derive(Clone, Debug)]
pub struct StabilizationScalePoint {
    pub routes: usize,
    pub training_spikes: u64,
    pub final_spikes_per_route: f64,
    pub final_runaway_rate: f64,
}

pub fn scaling_sweep() -> Vec<StabilizationScalePoint> {
    [1, 2, 4, 8, 16]
        .into_iter()
        .map(|route_count| {
            let (mut network, routes) = build_network(route_count);
            let training_spikes =
                train_routes(&mut network, &routes, LearningMode::Combined, 12) as u64;
            let (_, final_runaway_rate, _) = evaluate_routes(&network, &routes);
            let total_final_spikes: usize =
                routes.iter().map(|&route| network.run(route).spikes).sum();
            StabilizationScalePoint {
                routes: route_count,
                training_spikes,
                final_spikes_per_route: total_final_spikes as f64 / route_count as f64,
                final_runaway_rate,
            }
        })
        .collect()
}

pub fn print_report(report: &StabilityReport) {
    println!("v14.6 learned self-stabilization:");
    println!(
        "  prediction pressure only: accuracy={:.1}%, runaway={:.1}%",
        report.prediction_only_accuracy * 100.0,
        report.prediction_only_runaway_rate * 100.0
    );
    println!(
        "  activity pressure only: accuracy={:.1}%, runaway={:.1}%",
        report.activity_only_accuracy * 100.0,
        report.activity_only_runaway_rate * 100.0
    );
    println!(
        "  combined learning: accuracy={:.1}%, runaway={:.1}%, median spikes={} -> {}, compressed concepts={}",
        report.combined_accuracy * 100.0,
        report.combined_runaway_rate * 100.0,
        report.initial_median_spikes,
        report.final_median_spikes,
        report.compressed_concepts
    );
    println!(
        "  new unstable loop detected={}, recovered={} after {} learning samples",
        report.destabilized_by_new_loop,
        report.recovered_after_perturbation,
        report.recovery_samples
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v14_6_prediction_pressure_alone_keeps_accurate_runaway_activity() {
        let report = run_experiment();

        assert_eq!(report.prediction_only_accuracy, 1.0);
        assert_eq!(report.prediction_only_runaway_rate, 1.0);
    }

    #[test]
    fn v14_6_activity_pressure_alone_learns_silence_not_useful_compression() {
        let report = run_experiment();

        assert_eq!(report.activity_only_accuracy, 0.0);
        assert_eq!(report.activity_only_runaway_rate, 0.0);
    }

    #[test]
    fn v14_6_combined_learning_compresses_and_settles() {
        let report = run_experiment();

        assert_eq!(report.combined_accuracy, 1.0);
        assert_eq!(report.combined_runaway_rate, 0.0);
        assert!(report.final_median_spikes <= 3);
        assert_eq!(report.compressed_concepts, 2);
    }

    #[test]
    fn v14_6_recovers_after_a_new_unstable_loop_is_added() {
        let report = run_experiment();

        assert!(report.destabilized_by_new_loop);
        assert!(report.recovered_after_perturbation);
        assert!(report.recovery_samples <= 4);
        assert!(report.passed);
    }
}
