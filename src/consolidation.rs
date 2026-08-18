use std::collections::HashMap;

use crate::unified::{Token, UnifiedLearner};

const TOKEN_COUNT: usize = 256;
const MAX_PATTERN_DEPTH: usize = 8;
const ACTIVITY_LIMIT: usize = 4_096;
const PAIR_DELIMITER: Token = 250;
const COPY_DELIMITER: Token = 251;
const UNKNOWN_TOKEN: Token = 252;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Behavior {
    induction_correct: usize,
    induction_total: usize,
    pair_correct: usize,
    pair_total: usize,
    needle_correct: usize,
    needle_total: usize,
    remapped_correct: usize,
    unknown_rejected: bool,
    work: u64,
    active_patterns: u64,
    absorbed_for_evaluation: u64,
}

impl Behavior {
    fn induction_accuracy(self) -> f64 {
        self.induction_correct as f64 / self.induction_total as f64
    }

    fn average_work(self) -> f64 {
        self.work as f64 / self.absorbed_for_evaluation as f64
    }

    fn average_active_patterns(self) -> f64 {
        self.active_patterns as f64 / self.absorbed_for_evaluation as f64
    }

    fn preserves(self, reference: Self) -> bool {
        self.induction_accuracy() + 0.01 >= reference.induction_accuracy()
            && self.pair_correct == reference.pair_correct
            && self.needle_correct == reference.needle_correct
            && self.remapped_correct == reference.remapped_correct
            && self.unknown_rejected == reference.unknown_rejected
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct MemorySize {
    structures: usize,
    links: usize,
    work: u64,
    active_patterns: usize,
}

trait SequenceMemory: Clone {
    fn reset(&mut self);
    fn absorb_learning(&mut self, token: Token);
    fn absorb_read_only(&mut self, token: Token);
    fn answer(&self) -> Option<Token>;
    fn size(&self) -> MemorySize;
}

impl SequenceMemory for UnifiedLearner {
    fn reset(&mut self) {
        self.reset_activity();
    }

    fn absorb_learning(&mut self, token: Token) {
        self.absorb(token);
    }

    fn absorb_read_only(&mut self, token: Token) {
        self.absorb_without_learning(token);
    }

    fn answer(&self) -> Option<Token> {
        self.answer()
    }

    fn size(&self) -> MemorySize {
        let metrics = self.metrics();
        MemorySize {
            structures: metrics.learned_pattern_cells,
            links: metrics.arrows,
            work: metrics.spikes,
            active_patterns: self.active_pattern_count(),
        }
    }
}

#[derive(Clone, Debug)]
struct TrieNode {
    depth: usize,
    activations: u32,
    children: HashMap<Token, usize>,
    predictions: HashMap<Token, u32>,
}

#[derive(Clone, Debug)]
struct ContextTrie {
    max_depth: usize,
    nodes: Vec<TrieNode>,
    active: Vec<usize>,
    last_prediction: Option<Token>,
    lookups: u64,
}

impl ContextTrie {
    fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            nodes: vec![TrieNode {
                depth: 0,
                activations: 0,
                children: HashMap::new(),
                predictions: HashMap::new(),
            }],
            active: Vec::new(),
            last_prediction: None,
            lookups: 0,
        }
    }

    fn absorb_internal(&mut self, token: Token, learn: bool) {
        if learn {
            for &node in &self.active {
                self.lookups += 1;
                *self.nodes[node].predictions.entry(token).or_default() += 1;
            }
        }

        let mut parents = Vec::with_capacity(self.active.len() + 1);
        parents.push(0);
        parents.extend(
            self.active
                .iter()
                .copied()
                .filter(|&node| self.nodes[node].depth < self.max_depth),
        );

        let mut next = Vec::with_capacity(parents.len());
        for parent in parents {
            self.lookups += 1;
            let child = if let Some(&child) = self.nodes[parent].children.get(&token) {
                Some(child)
            } else if learn {
                let child = self.nodes.len();
                let depth = self.nodes[parent].depth + 1;
                self.nodes.push(TrieNode {
                    depth,
                    activations: 0,
                    children: HashMap::new(),
                    predictions: HashMap::new(),
                });
                self.nodes[parent].children.insert(token, child);
                Some(child)
            } else {
                None
            };
            if let Some(child) = child {
                self.nodes[child].activations = self.nodes[child].activations.saturating_add(1);
                next.push(child);
            }
        }
        self.active = next;
        self.last_prediction = self.predict();
    }

    fn predict(&mut self) -> Option<Token> {
        let node = self
            .active
            .iter()
            .filter(|&&node| !self.nodes[node].predictions.is_empty())
            .max_by_key(|&&node| self.nodes[node].depth)
            .copied()?;
        self.lookups += self.nodes[node].predictions.len() as u64;

        let mut best = None;
        let mut tied = false;
        for (&token, &strength) in &self.nodes[node].predictions {
            match best {
                None => {
                    best = Some((token, strength));
                    tied = false;
                }
                Some((_, best_strength)) if strength > best_strength => {
                    best = Some((token, strength));
                    tied = false;
                }
                Some((_, best_strength)) if strength == best_strength => {
                    tied = true;
                }
                Some(_) => {}
            }
        }
        if tied {
            None
        } else {
            best.map(|(token, _)| token)
        }
    }
}

impl SequenceMemory for ContextTrie {
    fn reset(&mut self) {
        self.active.clear();
        self.last_prediction = None;
    }

    fn absorb_learning(&mut self, token: Token) {
        self.absorb_internal(token, true);
    }

    fn absorb_read_only(&mut self, token: Token) {
        self.absorb_internal(token, false);
    }

    fn answer(&self) -> Option<Token> {
        self.last_prediction
    }

    fn size(&self) -> MemorySize {
        let child_links: usize = self.nodes.iter().map(|node| node.children.len()).sum();
        let predictions: usize = self.nodes.iter().map(|node| node.predictions.len()).sum();
        MemorySize {
            structures: self.nodes.len() - 1,
            links: child_links + predictions,
            work: self.lookups,
            active_patterns: self.active.len(),
        }
    }
}

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

    fn token(&mut self, start: Token, count: Token) -> Token {
        start + ((self.next_u64() >> 32) % count as u64) as Token
    }
}

#[derive(Clone, Debug)]
struct Curriculum {
    sequences: Vec<Vec<Token>>,
    pairs: Vec<(Token, Token)>,
    needle_stream: Vec<Token>,
    facts: Vec<(Token, Token)>,
}

impl Curriculum {
    fn new(sequence_trials: usize, sequence_len: usize, noise_len: usize, seed: u64) -> Self {
        let mut rng = DeterministicRng::new(seed);
        let sequences = (0..sequence_trials)
            .map(|_| (0..sequence_len).map(|_| rng.token(0, 64)).collect())
            .collect();
        let pairs = (0..32).map(|index| (64 + index, 96 + index)).collect();
        let facts = vec![(128, 129), (130, 131), (132, 133)];
        let placements = [noise_len / 10, noise_len / 2, noise_len * 9 / 10];
        let mut needle_stream = Vec::with_capacity(noise_len);
        let mut fact_index = 0;
        let mut position = 0;
        while position < noise_len {
            if fact_index < facts.len() && position == placements[fact_index] {
                needle_stream.push(facts[fact_index].0);
                needle_stream.push(facts[fact_index].1);
                fact_index += 1;
                position += 2;
            } else {
                needle_stream.push(rng.token(134, 116));
                position += 1;
            }
        }
        Self {
            sequences,
            pairs,
            needle_stream,
            facts,
        }
    }

    fn experience_tokens(&self) -> usize {
        let induction = self
            .sequences
            .iter()
            .map(|sequence| sequence.len() * 2 + 1)
            .sum::<usize>();
        let associations = self.pairs.len() * 3;
        let queries = self.pairs.len() + self.facts.len() + self.pairs.len() + 1;
        induction + associations + self.needle_stream.len() + queries
    }

    fn replay_tokens(&self) -> usize {
        self.sequences.iter().map(Vec::len).sum::<usize>()
            + self.pairs.len()
            + self.facts.len()
            + self.pairs.len()
            + 1
    }
}

fn absorb_and_measure<M: SequenceMemory>(
    memory: &mut M,
    token: Token,
    learn: bool,
    behavior: &mut Behavior,
) {
    let before = memory.size().work;
    if learn {
        memory.absorb_learning(token);
    } else {
        memory.absorb_read_only(token);
    }
    let after = memory.size();
    behavior.work += after.work - before;
    behavior.active_patterns += after.active_patterns as u64;
    behavior.absorbed_for_evaluation += 1;
}

fn query_pairs<M: SequenceMemory>(
    memory: &mut M,
    pairs: &[(Token, Token)],
    learn: bool,
    behavior: &mut Behavior,
) -> usize {
    pairs
        .iter()
        .filter(|&&(key, value)| {
            memory.reset();
            absorb_and_measure(memory, key, learn, behavior);
            memory.answer() == Some(value)
        })
        .count()
}

fn train_and_mark<M: SequenceMemory>(memory: &mut M, curriculum: &Curriculum) -> Behavior {
    let mut behavior = Behavior::default();
    for sequence in &curriculum.sequences {
        memory.reset();
        for &token in sequence {
            memory.absorb_learning(token);
        }
        memory.absorb_learning(COPY_DELIMITER);
        memory.absorb_learning(sequence[0]);
        for &expected in &sequence[1..] {
            behavior.induction_correct += usize::from(memory.answer() == Some(expected));
            behavior.induction_total += 1;
            memory.absorb_learning(expected);
        }
    }

    memory.reset();
    for &(key, value) in &curriculum.pairs {
        memory.absorb_learning(PAIR_DELIMITER);
        memory.absorb_learning(key);
        memory.absorb_learning(value);
    }

    memory.reset();
    for &token in &curriculum.needle_stream {
        memory.absorb_learning(token);
    }

    behavior.pair_total = curriculum.pairs.len();
    behavior.pair_correct = query_pairs(memory, &curriculum.pairs, true, &mut behavior);
    behavior.needle_total = curriculum.facts.len();
    behavior.needle_correct = query_pairs(memory, &curriculum.facts, true, &mut behavior);
    let remapped: Vec<_> = curriculum
        .pairs
        .iter()
        .enumerate()
        .map(|(index, &(key, _))| {
            (
                key,
                curriculum.pairs[(index + 1) % curriculum.pairs.len()].1,
            )
        })
        .collect();
    behavior.remapped_correct = query_pairs(memory, &remapped, true, &mut behavior);
    memory.reset();
    absorb_and_measure(memory, UNKNOWN_TOKEN, true, &mut behavior);
    behavior.unknown_rejected = memory.answer().is_none();
    behavior
}

fn evaluate_read_only<M: SequenceMemory>(memory: &M, curriculum: &Curriculum) -> Behavior {
    let mut memory = memory.clone();
    let mut behavior = Behavior::default();

    for sequence in &curriculum.sequences {
        memory.reset();
        absorb_and_measure(&mut memory, sequence[0], false, &mut behavior);
        for &expected in &sequence[1..] {
            behavior.induction_correct += usize::from(memory.answer() == Some(expected));
            behavior.induction_total += 1;
            absorb_and_measure(&mut memory, expected, false, &mut behavior);
        }
    }

    behavior.pair_total = curriculum.pairs.len();
    behavior.pair_correct = query_pairs(&mut memory, &curriculum.pairs, false, &mut behavior);
    behavior.needle_total = curriculum.facts.len();
    behavior.needle_correct = query_pairs(&mut memory, &curriculum.facts, false, &mut behavior);
    let remapped: Vec<_> = curriculum
        .pairs
        .iter()
        .enumerate()
        .map(|(index, &(key, _))| {
            (
                key,
                curriculum.pairs[(index + 1) % curriculum.pairs.len()].1,
            )
        })
        .collect();
    behavior.remapped_correct = query_pairs(&mut memory, &remapped, false, &mut behavior);
    memory.reset();
    absorb_and_measure(&mut memory, UNKNOWN_TOKEN, false, &mut behavior);
    behavior.unknown_rejected = memory.answer().is_none();
    behavior
}

#[derive(Clone, Debug)]
pub struct ConsolidationScalePoint {
    pub experience_tokens: usize,
    pub raw_patterns: usize,
    pub consolidated_patterns: usize,
    pub consolidated_accuracy: f64,
}

#[derive(Clone, Debug)]
pub struct ConsolidationReport {
    pub event_accuracy_before: f64,
    pub event_accuracy_after: f64,
    pub trie_accuracy: f64,
    pub event_pairs: usize,
    pub consolidated_pairs: usize,
    pub trie_pairs: usize,
    pub event_needles: usize,
    pub consolidated_needles: usize,
    pub trie_needles: usize,
    pub patterns_before: usize,
    pub patterns_after: usize,
    pub trie_nodes: usize,
    pub arrows_before: usize,
    pub arrows_after: usize,
    pub trie_links: usize,
    pub work_per_query_before: f64,
    pub work_per_query_after: f64,
    pub active_per_query_before: f64,
    pub active_per_query_after: f64,
    pub random_control_accuracy: f64,
    pub random_control_patterns: usize,
    pub rest_replay_tokens: usize,
    pub retained_replay_tokens: usize,
    pub rewrite_accepted: bool,
    pub scaling: Vec<ConsolidationScalePoint>,
    pub passed: bool,
}

fn train_unified(curriculum: &Curriculum) -> (UnifiedLearner, Behavior) {
    let mut learner = UnifiedLearner::new(TOKEN_COUNT, MAX_PATTERN_DEPTH, ACTIVITY_LIMIT);
    let behavior = train_and_mark(&mut learner, curriculum);
    (learner, behavior)
}

fn scaling_sweep() -> Vec<ConsolidationScalePoint> {
    [1, 2, 4, 8, 16]
        .into_iter()
        .map(|factor| {
            let curriculum = Curriculum::new(4, 64, factor * 1_024, 0x1700_1000);
            let (learner, _) = train_unified(&curriculum);
            let raw_patterns = learner.metrics().learned_pattern_cells;
            let mut consolidated = learner.clone();
            consolidated.consolidate_recurring(2);
            let behavior = evaluate_read_only(&consolidated, &curriculum);
            ConsolidationScalePoint {
                experience_tokens: curriculum.experience_tokens(),
                raw_patterns,
                consolidated_patterns: consolidated.metrics().learned_pattern_cells,
                consolidated_accuracy: behavior.induction_accuracy(),
            }
        })
        .collect()
}

pub fn run_experiment() -> ConsolidationReport {
    let curriculum = Curriculum::new(24, 64, 8_192, 0x1700_0001);
    let (learner, _) = train_unified(&curriculum);
    let before_behavior = evaluate_read_only(&learner, &curriculum);
    let before_size = learner.size();

    let mut trie = ContextTrie::new(MAX_PATTERN_DEPTH);
    train_and_mark(&mut trie, &curriculum);
    let trie_behavior = evaluate_read_only(&trie, &curriculum);
    let trie_size = trie.size();

    let mut candidate = learner.clone();
    let consolidation = candidate.consolidate_recurring(2);
    let after_behavior = evaluate_read_only(&candidate, &curriculum);
    let after_size = candidate.size();
    let rewrite_accepted = after_behavior.preserves(before_behavior);

    let mut random_control = candidate.clone();
    random_control.scramble_prediction_targets(0x1700_dead);
    let random_behavior = evaluate_read_only(&random_control, &curriculum);
    let random_size = random_control.size();

    let scaling = scaling_sweep();
    let first = &scaling[0];
    let last = scaling.last().expect("scaling sweep is non-empty");
    let raw_growth = last.raw_patterns as f64 / first.raw_patterns as f64;
    let consolidated_growth =
        last.consolidated_patterns as f64 / first.consolidated_patterns as f64;
    let memory_reduction = consolidation.patterns_removed as f64 / before_size.structures as f64;

    let passed = rewrite_accepted
        && before_behavior.induction_accuracy() >= 0.97
        && (trie_behavior.induction_accuracy() - before_behavior.induction_accuracy()).abs()
            <= 0.01
        && trie_behavior.pair_correct == before_behavior.pair_correct
        && trie_behavior.needle_correct == before_behavior.needle_correct
        && memory_reduction >= 0.60
        && after_size.links * 2 < before_size.links
        && random_behavior.induction_accuracy() + 0.20 < after_behavior.induction_accuracy()
        && raw_growth >= 6.0
        && consolidated_growth <= 4.0
        && scaling
            .iter()
            .all(|point| point.consolidated_accuracy >= 0.95);

    ConsolidationReport {
        event_accuracy_before: before_behavior.induction_accuracy(),
        event_accuracy_after: after_behavior.induction_accuracy(),
        trie_accuracy: trie_behavior.induction_accuracy(),
        event_pairs: before_behavior.pair_correct,
        consolidated_pairs: after_behavior.pair_correct,
        trie_pairs: trie_behavior.pair_correct,
        event_needles: before_behavior.needle_correct,
        consolidated_needles: after_behavior.needle_correct,
        trie_needles: trie_behavior.needle_correct,
        patterns_before: before_size.structures,
        patterns_after: after_size.structures,
        trie_nodes: trie_size.structures,
        arrows_before: before_size.links,
        arrows_after: after_size.links,
        trie_links: trie_size.links,
        work_per_query_before: before_behavior.average_work(),
        work_per_query_after: after_behavior.average_work(),
        active_per_query_before: before_behavior.average_active_patterns(),
        active_per_query_after: after_behavior.average_active_patterns(),
        random_control_accuracy: random_behavior.induction_accuracy(),
        random_control_patterns: random_size.structures,
        rest_replay_tokens: curriculum.replay_tokens(),
        retained_replay_tokens: 0,
        rewrite_accepted,
        scaling,
        passed,
    }
}

pub fn print_report(report: &ConsolidationReport) {
    println!("v17 consolidation:");
    println!(
        "  induction: event={:.1}%, trie={:.1}%, after rest={:.1}%",
        report.event_accuracy_before * 100.0,
        report.trie_accuracy * 100.0,
        report.event_accuracy_after * 100.0
    );
    println!(
        "  recall: pairs event/after/trie={}/{}/{}, needles={}/{}/{}",
        report.event_pairs,
        report.consolidated_pairs,
        report.trie_pairs,
        report.event_needles,
        report.consolidated_needles,
        report.trie_needles
    );
    println!(
        "  patterns: {} -> {}, trie nodes={}; links: {} -> {}, trie links={}",
        report.patterns_before,
        report.patterns_after,
        report.trie_nodes,
        report.arrows_before,
        report.arrows_after,
        report.trie_links
    );
    println!(
        "  work/query: {:.1} -> {:.1}, active patterns/query: {:.1} -> {:.1}",
        report.work_per_query_before,
        report.work_per_query_after,
        report.active_per_query_before,
        report.active_per_query_after
    );
    println!(
        "  arbitrary rewiring control: {:.1}% with {} patterns; rewrite accepted={}",
        report.random_control_accuracy * 100.0,
        report.random_control_patterns,
        report.rewrite_accepted
    );
    println!(
        "  rest replay tokens={}, retained after rest={}",
        report.rest_replay_tokens, report.retained_replay_tokens
    );
    print!("  scaling patterns raw/consolidated:");
    for point in &report.scaling {
        print!(
            " {}:{}/{}",
            point.experience_tokens, point.raw_patterns, point.consolidated_patterns
        );
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v17_plain_trie_matches_the_unconsolidated_learner() {
        let report = run_experiment();

        assert!((report.event_accuracy_before - report.trie_accuracy).abs() <= 0.01);
        assert_eq!(report.event_pairs, report.trie_pairs);
        assert_eq!(report.event_needles, report.trie_needles);
    }

    #[test]
    fn v17_rest_preserves_behavior_with_less_structure() {
        let report = run_experiment();

        assert!(report.rewrite_accepted);
        assert!(report.event_accuracy_after + 0.01 >= report.event_accuracy_before);
        assert_eq!(report.event_pairs, report.consolidated_pairs);
        assert_eq!(report.event_needles, report.consolidated_needles);
        assert!(report.patterns_after * 2 < report.patterns_before);
        assert!(report.arrows_after * 2 < report.arrows_before);
    }

    #[test]
    fn v17_arbitrary_rewiring_does_not_preserve_induction() {
        let report = run_experiment();

        assert!(
            report.random_control_accuracy + 0.20 < report.event_accuracy_after,
            "random={:.3}, recurring={:.3}",
            report.random_control_accuracy,
            report.event_accuracy_after
        );
    }

    #[test]
    fn v17_consolidated_memory_grows_slower_than_raw_memory() {
        let report = run_experiment();
        let first = &report.scaling[0];
        let last = report.scaling.last().unwrap();
        let raw_growth = last.raw_patterns as f64 / first.raw_patterns as f64;
        let consolidated_growth =
            last.consolidated_patterns as f64 / first.consolidated_patterns as f64;

        assert!(raw_growth >= 6.0);
        assert!(consolidated_growth <= 4.0);
        assert!(report
            .scaling
            .iter()
            .all(|point| point.consolidated_accuracy >= 0.95));
        assert!(report.passed);
    }
}
