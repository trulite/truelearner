use std::collections::{HashMap, HashSet};

use crate::unified::{Token, UnifiedLearner};

const TOKEN_COUNT: usize = 256;
const MAX_PATTERN_DEPTH: usize = 8;
const ACTIVITY_LIMIT: usize = 4_096;
const EDGE_MARKER: Token = 250;
const QUERY_MARKER: Token = 251;
const ANSWER_MARKER: Token = 252;
const END_MARKER: Token = 253;

#[derive(Clone, Debug)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.state
    }

    fn shuffle<T>(&mut self, values: &mut [T]) {
        for index in (1..values.len()).rev() {
            let selected = (self.next_u64() as usize) % (index + 1);
            values.swap(index, selected);
        }
    }
}

#[derive(Clone, Debug)]
struct ChainEpisode {
    edges: Vec<(Token, Token)>,
    query: Token,
    answer: Token,
}

impl ChainEpisode {
    fn prompt(&self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(self.edges.len() * 3 + 2);
        for &(from, to) in &self.edges {
            tokens.extend([EDGE_MARKER, from, to]);
        }
        tokens.extend([QUERY_MARKER, self.query]);
        tokens
    }

    fn training_stream(&self) -> Vec<Token> {
        let mut tokens = self.prompt();
        tokens.extend([ANSWER_MARKER, self.answer, END_MARKER]);
        tokens
    }
}

fn make_episode(
    rng: &mut DeterministicRng,
    symbol_start: Token,
    symbol_count: Token,
    chain_links: usize,
    distractor_links: usize,
) -> ChainEpisode {
    let needed = chain_links + 1 + distractor_links + 1;
    assert!(needed <= symbol_count as usize);
    let mut symbols: Vec<_> = (symbol_start..symbol_start + symbol_count).collect();
    rng.shuffle(&mut symbols);

    let main = &symbols[..=chain_links];
    let distractor_start = chain_links + 1;
    let distractor_end = distractor_start + distractor_links;
    let distractor = &symbols[distractor_start..=distractor_end];
    let mut edges: Vec<_> = main
        .windows(2)
        .chain(distractor.windows(2))
        .map(|pair| (pair[0], pair[1]))
        .collect();
    rng.shuffle(&mut edges);

    ChainEpisode {
        edges,
        query: main[0],
        answer: main[chain_links],
    }
}

fn training_episodes(count: usize) -> Vec<ChainEpisode> {
    let mut rng = DeterministicRng::new(0x1800_0001);
    (0..count)
        .map(|index| make_episode(&mut rng, 0, 96, 2 + index % 3, 2))
        .collect()
}

fn held_out_episodes() -> Vec<(usize, ChainEpisode)> {
    let mut rng = DeterministicRng::new(0x1800_0002);
    let mut episodes = Vec::new();
    for depth in [5, 8, 16, 32] {
        for _ in 0..8 {
            episodes.push((depth, make_episode(&mut rng, 96, 144, depth, 2)));
        }
    }
    episodes
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WalkOutcome {
    Answer(Token),
    Ambiguous,
    Cycle,
    Unknown,
}

fn walk(episode: &ChainEpisode) -> (WalkOutcome, usize) {
    let mut outgoing: HashMap<Token, Vec<Token>> = HashMap::new();
    let mut known = HashSet::new();
    for &(from, to) in &episode.edges {
        outgoing.entry(from).or_default().push(to);
        known.insert(from);
        known.insert(to);
    }
    if !known.contains(&episode.query) {
        return (WalkOutcome::Unknown, 0);
    }

    let mut current = episode.query;
    let mut visited = HashSet::new();
    let mut work = 0;
    loop {
        if !visited.insert(current) {
            return (WalkOutcome::Cycle, work);
        }
        work += 1;
        match outgoing.get(&current).map(Vec::as_slice).unwrap_or(&[]) {
            [] => return (WalkOutcome::Answer(current), work),
            [next] => current = *next,
            _ => return (WalkOutcome::Ambiguous, work),
        }
    }
}

#[derive(Clone, Debug)]
struct ContextLookup {
    max_depth: usize,
    history: Vec<Token>,
    predictions: HashMap<Vec<Token>, HashMap<Token, u32>>,
    last_prediction: Option<Token>,
    work: u64,
}

impl ContextLookup {
    fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            history: Vec::new(),
            predictions: HashMap::new(),
            last_prediction: None,
            work: 0,
        }
    }

    fn reset(&mut self) {
        self.history.clear();
        self.last_prediction = None;
    }

    fn absorb(&mut self, token: Token, learn: bool) {
        if learn {
            for length in 1..=self.history.len().min(self.max_depth) {
                self.work += 1;
                let context = self.history[self.history.len() - length..].to_vec();
                *self
                    .predictions
                    .entry(context)
                    .or_default()
                    .entry(token)
                    .or_default() += 1;
            }
        }
        self.history.push(token);
        if self.history.len() > self.max_depth {
            self.history.remove(0);
        }
        self.last_prediction = self.predict();
    }

    fn predict(&mut self) -> Option<Token> {
        for length in (1..=self.history.len()).rev() {
            self.work += 1;
            let context = &self.history[self.history.len() - length..];
            let Some(predictions) = self.predictions.get(context) else {
                continue;
            };
            let mut best = None;
            let mut tied = false;
            for (&token, &strength) in predictions {
                match best {
                    None => {
                        best = Some((token, strength));
                        tied = false;
                    }
                    Some((_, best_strength)) if strength > best_strength => {
                        best = Some((token, strength));
                        tied = false;
                    }
                    Some((_, best_strength)) if strength == best_strength => tied = true,
                    Some(_) => {}
                }
            }
            return if tied {
                None
            } else {
                best.map(|(token, _)| token)
            };
        }
        None
    }

    fn answer(&self) -> Option<Token> {
        self.last_prediction
    }

    fn contexts(&self) -> usize {
        self.predictions.len()
    }

    fn links(&self) -> usize {
        self.predictions.values().map(HashMap::len).sum()
    }
}

fn train_memories(episodes: &[ChainEpisode]) -> (UnifiedLearner, ContextLookup) {
    let mut learner = UnifiedLearner::new(TOKEN_COUNT, MAX_PATTERN_DEPTH, ACTIVITY_LIMIT);
    let mut lookup = ContextLookup::new(MAX_PATTERN_DEPTH);
    for episode in episodes {
        learner.reset_activity();
        lookup.reset();
        for token in episode.training_stream() {
            learner.absorb(token);
            lookup.absorb(token, true);
        }
    }
    (learner, lookup)
}

fn query_learner(learner: &UnifiedLearner, episode: &ChainEpisode) -> (Option<Token>, u64) {
    let mut query = learner.clone();
    query.reset_activity();
    let before_spikes = query.metrics().spikes;
    for token in episode.prompt() {
        query.absorb_without_learning(token);
    }
    query.absorb_without_learning(ANSWER_MARKER);
    (query.answer(), query.metrics().spikes - before_spikes)
}

fn query_lookup(lookup: &ContextLookup, episode: &ChainEpisode) -> Option<Token> {
    let mut query = lookup.clone();
    query.reset();
    for token in episode.prompt() {
        query.absorb(token, false);
    }
    query.absorb(ANSWER_MARKER, false);
    query.answer()
}

#[derive(Clone, Debug)]
pub struct DepthResult {
    pub depth: usize,
    pub episodes: usize,
    pub learner_correct: usize,
    pub trie_correct: usize,
    pub walker_correct: usize,
    pub learner_spikes_per_query: f64,
    pub walker_steps_per_query: f64,
}

#[derive(Clone, Debug)]
pub struct CompositionGrowthPoint {
    pub training_examples: usize,
    pub learner_patterns: usize,
    pub learner_arrows: usize,
    pub trie_contexts: usize,
    pub trie_links: usize,
}

#[derive(Clone, Debug)]
pub struct CompositionReport {
    pub depths: Vec<DepthResult>,
    pub growth: Vec<CompositionGrowthPoint>,
    pub branch_control: bool,
    pub cycle_control: bool,
    pub missing_query_control: bool,
    pub learner_control_answers: usize,
    pub permanent_patterns_added_during_test: usize,
    pub permanent_arrows_added_during_test: usize,
    pub training_symbols_disjoint_from_test: bool,
    pub probe_valid: bool,
    pub composition_discovered: bool,
}

pub fn run_experiment() -> CompositionReport {
    let training = training_episodes(128);
    let (learner, lookup) = train_memories(&training);
    let before = learner.metrics();
    let held_out = held_out_episodes();
    let mut depth_results = Vec::new();

    for depth in [5, 8, 16, 32] {
        let episodes: Vec<_> = held_out
            .iter()
            .filter(|(episode_depth, _)| *episode_depth == depth)
            .map(|(_, episode)| episode)
            .collect();
        let mut learner_correct = 0;
        let mut trie_correct = 0;
        let mut walker_correct = 0;
        let mut learner_spikes = 0;
        let mut walker_steps = 0;

        for episode in &episodes {
            let (prediction, spikes) = query_learner(&learner, episode);
            learner_spikes += spikes;
            learner_correct += usize::from(prediction == Some(episode.answer));
            trie_correct += usize::from(query_lookup(&lookup, episode) == Some(episode.answer));

            let (outcome, steps) = walk(episode);
            walker_steps += steps;
            walker_correct += usize::from(outcome == WalkOutcome::Answer(episode.answer));
        }

        depth_results.push(DepthResult {
            depth,
            episodes: episodes.len(),
            learner_correct,
            trie_correct,
            walker_correct,
            learner_spikes_per_query: learner_spikes as f64 / episodes.len() as f64,
            walker_steps_per_query: walker_steps as f64 / episodes.len() as f64,
        });
    }

    let after = learner.metrics();
    let mut control_rng = DeterministicRng::new(0x1800_0003);
    let base_control = make_episode(&mut control_rng, 96, 144, 5, 0);

    let mut branch = base_control.clone();
    let extra = (96..240)
        .find(|symbol| {
            !branch
                .edges
                .iter()
                .any(|&(from, to)| from == *symbol || to == *symbol)
        })
        .unwrap();
    branch.edges.push((branch.query, extra));
    let branch_control = walk(&branch).0 == WalkOutcome::Ambiguous;

    let mut cycle = base_control.clone();
    cycle.edges.push((cycle.answer, cycle.query));
    let cycle_control = walk(&cycle).0 == WalkOutcome::Cycle;

    let mut missing = base_control;
    missing.query = 249;
    let missing_query_control = walk(&missing).0 == WalkOutcome::Unknown;
    let learner_control_answers = [&branch, &cycle, &missing]
        .into_iter()
        .filter(|episode| query_learner(&learner, episode).0.is_some())
        .count();

    let growth = [16, 32, 64, 128]
        .into_iter()
        .map(|count| {
            let (learner, trie) = train_memories(&training[..count]);
            let metrics = learner.metrics();
            CompositionGrowthPoint {
                training_examples: count,
                learner_patterns: metrics.learned_pattern_cells,
                learner_arrows: metrics.arrows,
                trie_contexts: trie.contexts(),
                trie_links: trie.links(),
            }
        })
        .collect::<Vec<_>>();

    let training_symbols_disjoint_from_test = training
        .iter()
        .all(|episode| episode.edges.iter().all(|&(from, to)| from < 96 && to < 96))
        && held_out.iter().all(|(_, episode)| {
            episode
                .edges
                .iter()
                .all(|&(from, to)| (96..240).contains(&from) && (96..240).contains(&to))
        });
    let walker_passed = depth_results
        .iter()
        .all(|result| result.walker_correct == result.episodes);
    let learner_accuracy = depth_results
        .iter()
        .map(|result| result.learner_correct)
        .sum::<usize>() as f64
        / depth_results
            .iter()
            .map(|result| result.episodes)
            .sum::<usize>() as f64;
    let trie_accuracy = depth_results
        .iter()
        .map(|result| result.trie_correct)
        .sum::<usize>() as f64
        / depth_results
            .iter()
            .map(|result| result.episodes)
            .sum::<usize>() as f64;
    let memory_grows_with_examples = growth
        .windows(2)
        .all(|pair| pair[1].learner_patterns > pair[0].learner_patterns);
    let permanent_patterns_added_during_test =
        after.learned_pattern_cells - before.learned_pattern_cells;
    let permanent_arrows_added_during_test = after.arrows - before.arrows;
    let probe_valid = walker_passed
        && branch_control
        && cycle_control
        && missing_query_control
        && training_symbols_disjoint_from_test
        && permanent_patterns_added_during_test == 0
        && permanent_arrows_added_during_test == 0
        && memory_grows_with_examples;
    let composition_discovered = learner_accuracy >= 0.95 && trie_accuracy <= 0.10 && probe_valid;

    CompositionReport {
        depths: depth_results,
        growth,
        branch_control,
        cycle_control,
        missing_query_control,
        learner_control_answers,
        permanent_patterns_added_during_test,
        permanent_arrows_added_during_test,
        training_symbols_disjoint_from_test,
        probe_valid,
        composition_discovered,
    }
}

pub fn print_report(report: &CompositionReport) {
    println!("v18 renaming-invariant composition:");
    for result in &report.depths {
        println!(
            "  depth {}: learner={}/{}, trie={}/{}, walker={}/{}, learner spikes/query={:.1}, walker steps/query={:.1}",
            result.depth,
            result.learner_correct,
            result.episodes,
            result.trie_correct,
            result.episodes,
            result.walker_correct,
            result.episodes,
            result.learner_spikes_per_query,
            result.walker_steps_per_query
        );
    }
    print!("  permanent growth examples:patterns/arrows/trie-contexts:");
    for point in &report.growth {
        print!(
            " {}:{}/{}/{}",
            point.training_examples,
            point.learner_patterns,
            point.learner_arrows,
            point.trie_contexts
        );
    }
    println!();
    println!(
        "  controls branch/cycle/missing={}/{}/{}, learner fabricated answers on {}/3 controls, test graph growth patterns/arrows={}/{}",
        report.branch_control,
        report.cycle_control,
        report.missing_query_control,
        report.learner_control_answers,
        report.permanent_patterns_added_during_test,
        report.permanent_arrows_added_during_test
    );
    println!(
        "  probe valid={}, reusable composition discovered={}",
        report.probe_valid, report.composition_discovered
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v18_dataset_is_solvable_at_unseen_depths() {
        let report = run_experiment();

        assert!(report
            .depths
            .iter()
            .all(|result| result.walker_correct == result.episodes));
        assert!(report.training_symbols_disjoint_from_test);
    }

    #[test]
    fn v18_unchanged_learner_and_trie_do_not_transfer_composition() {
        let report = run_experiment();
        let learner_correct: usize = report
            .depths
            .iter()
            .map(|result| result.learner_correct)
            .sum();
        let trie_correct: usize = report.depths.iter().map(|result| result.trie_correct).sum();
        let total: usize = report.depths.iter().map(|result| result.episodes).sum();

        assert!(learner_correct * 10 <= total);
        assert!(trie_correct * 10 <= total);
        assert!(!report.composition_discovered);
    }

    #[test]
    fn v18_evaluation_is_read_only_and_memory_keeps_growing_with_examples() {
        let report = run_experiment();

        assert_eq!(report.permanent_patterns_added_during_test, 0);
        assert_eq!(report.permanent_arrows_added_during_test, 0);
        assert!(report
            .growth
            .windows(2)
            .all(|pair| pair[1].learner_patterns > pair[0].learner_patterns));
    }

    #[test]
    fn v18_controls_reject_branch_cycle_and_missing_query() {
        let report = run_experiment();

        assert!(report.branch_control);
        assert!(report.cycle_control);
        assert!(report.missing_query_control);
        assert!(report.learner_control_answers > 0);
        assert!(report.probe_valid);
    }
}
