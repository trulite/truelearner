use std::collections::{HashMap, VecDeque};

pub type Token = u16;
type CellId = usize;
type ArrowId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArrowUse {
    JoinParent,
    JoinToken,
    Predict,
}

#[derive(Clone, Debug)]
struct Cell {
    state: u32,
    threshold: u32,
    depth: usize,
    outgoing: Vec<ArrowId>,
}

#[derive(Clone, Debug)]
struct Arrow {
    from: CellId,
    to: CellId,
    strength: u32,
    use_: ArrowUse,
}

#[derive(Clone, Copy, Debug)]
struct Spike {
    from: CellId,
    to: CellId,
    value: u32,
}

#[derive(Clone, Copy, Debug)]
struct Join {
    child: CellId,
    parent_arrow: ArrowId,
    token_arrow: ArrowId,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LearnerMetrics {
    pub absorbed_tokens: u64,
    pub cells: usize,
    pub arrows: usize,
    pub learned_pattern_cells: usize,
    pub prediction_arrows: usize,
    pub spikes: u64,
    pub activity_limit_hits: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbsorbResult {
    pub prediction_before_input: Option<Token>,
    pub spikes: u64,
}

/// One sequence learner built from cells, arrows, and queued spikes.
///
/// The learner receives no task identifier. Its only supplied structure is
/// that tokens arrive in order. A recruited cell joins one previously active
/// pattern cell with the current token receptor. Prediction arrows grow from
/// active pattern cells to token receptors.
#[derive(Clone, Debug)]
pub struct UnifiedLearner {
    token_count: usize,
    max_pattern_depth: usize,
    activity_limit: usize,
    root: CellId,
    cells: Vec<Cell>,
    arrows: Vec<Arrow>,
    joins: HashMap<(CellId, CellId), Join>,
    predictions: HashMap<(CellId, CellId), ArrowId>,
    active_patterns: Vec<CellId>,
    queue: VecDeque<Spike>,
    last_prediction: Option<Token>,
    total_spikes: u64,
    absorbed_tokens: u64,
    activity_limit_hits: u64,
}

impl UnifiedLearner {
    pub fn new(token_count: usize, max_pattern_depth: usize, activity_limit: usize) -> Self {
        assert!(token_count > 1);
        assert!(token_count <= Token::MAX as usize + 1);
        assert!(max_pattern_depth > 0);
        assert!(activity_limit > 0);

        let mut cells = Vec::with_capacity(token_count + 1);
        for _ in 0..token_count {
            cells.push(Cell {
                state: 0,
                threshold: u32::MAX,
                depth: 0,
                outgoing: Vec::new(),
            });
        }
        let root = cells.len();
        cells.push(Cell {
            state: 0,
            threshold: 1,
            depth: 0,
            outgoing: Vec::new(),
        });

        Self {
            token_count,
            max_pattern_depth,
            activity_limit,
            root,
            cells,
            arrows: Vec::new(),
            joins: HashMap::new(),
            predictions: HashMap::new(),
            active_patterns: Vec::new(),
            queue: VecDeque::new(),
            last_prediction: None,
            total_spikes: 0,
            absorbed_tokens: 0,
            activity_limit_hits: 0,
        }
    }

    /// Clears recent activity while preserving every learned cell and arrow.
    pub fn reset_activity(&mut self) {
        self.active_patterns.clear();
        self.queue.clear();
        self.last_prediction = None;
    }

    pub fn answer(&self) -> Option<Token> {
        self.last_prediction
    }

    pub fn absorb(&mut self, token: Token) -> AbsorbResult {
        let receptor = self.receptor(token);
        let prediction_before_input = self.last_prediction;
        self.learn_successor(receptor);

        let mut parents = Vec::with_capacity(self.active_patterns.len() + 1);
        parents.push(self.root);
        parents.extend(
            self.active_patterns
                .iter()
                .copied()
                .filter(|&cell| self.cells[cell].depth < self.max_pattern_depth),
        );

        let joins: Vec<_> = parents
            .iter()
            .copied()
            .map(|parent| self.get_or_recruit_join(parent, receptor))
            .collect();
        let before_spikes = self.total_spikes;
        self.active_patterns = self.activate_joins(&joins);
        self.last_prediction = self.predict_from_active_patterns();
        self.absorbed_tokens += 1;

        AbsorbResult {
            prediction_before_input,
            spikes: self.total_spikes - before_spikes,
        }
    }

    pub fn metrics(&self) -> LearnerMetrics {
        LearnerMetrics {
            absorbed_tokens: self.absorbed_tokens,
            cells: self.cells.len(),
            arrows: self.arrows.len(),
            learned_pattern_cells: self.cells.len() - self.token_count - 1,
            prediction_arrows: self.predictions.len(),
            spikes: self.total_spikes,
            activity_limit_hits: self.activity_limit_hits,
        }
    }

    fn receptor(&self, token: Token) -> CellId {
        let receptor = token as usize;
        assert!(
            receptor < self.token_count,
            "token {token} is outside the configured receptor range"
        );
        receptor
    }

    fn add_cell(&mut self, threshold: u32, depth: usize) -> CellId {
        let id = self.cells.len();
        self.cells.push(Cell {
            state: 0,
            threshold,
            depth,
            outgoing: Vec::new(),
        });
        id
    }

    fn add_arrow(&mut self, from: CellId, to: CellId, strength: u32, use_: ArrowUse) -> ArrowId {
        let id = self.arrows.len();
        self.arrows.push(Arrow {
            from,
            to,
            strength,
            use_,
        });
        self.cells[from].outgoing.push(id);
        id
    }

    fn get_or_recruit_join(&mut self, parent: CellId, receptor: CellId) -> Join {
        if let Some(join) = self.joins.get(&(parent, receptor)) {
            return *join;
        }

        let depth = self.cells[parent].depth + 1;
        let child = self.add_cell(2, depth);
        let parent_arrow = self.add_arrow(parent, child, 1, ArrowUse::JoinParent);
        let token_arrow = self.add_arrow(receptor, child, 1, ArrowUse::JoinToken);
        let join = Join {
            child,
            parent_arrow,
            token_arrow,
        };
        self.joins.insert((parent, receptor), join);
        join
    }

    fn learn_successor(&mut self, receptor: CellId) {
        let sources = self.active_patterns.clone();
        for source in sources {
            if let Some(&arrow_id) = self.predictions.get(&(source, receptor)) {
                self.arrows[arrow_id].strength = self.arrows[arrow_id].strength.saturating_add(1);
            } else {
                let arrow_id = self.add_arrow(source, receptor, 1, ArrowUse::Predict);
                self.predictions.insert((source, receptor), arrow_id);
            }
        }
    }

    fn queue_arrow(&mut self, arrow_id: ArrowId) {
        let arrow = &self.arrows[arrow_id];
        self.queue.push_back(Spike {
            from: arrow.from,
            to: arrow.to,
            value: arrow.strength,
        });
    }

    fn activate_joins(&mut self, joins: &[Join]) -> Vec<CellId> {
        let mut touched = Vec::with_capacity(joins.len());
        for join in joins {
            debug_assert_eq!(self.arrows[join.parent_arrow].use_, ArrowUse::JoinParent);
            debug_assert_eq!(self.arrows[join.token_arrow].use_, ArrowUse::JoinToken);
            self.queue_arrow(join.parent_arrow);
            self.queue_arrow(join.token_arrow);
            touched.push(join.child);
        }

        let mut active = Vec::with_capacity(joins.len());
        let mut processed = 0;
        while let Some(spike) = self.queue.pop_front() {
            if processed == self.activity_limit {
                self.activity_limit_hits += 1;
                self.queue.clear();
                break;
            }
            processed += 1;
            self.total_spikes += 1;
            debug_assert!(spike.from < self.cells.len());
            let cell = &mut self.cells[spike.to];
            cell.state = cell.state.saturating_add(spike.value);
            if cell.state >= cell.threshold {
                cell.state = 0;
                active.push(spike.to);
            }
        }
        for cell in touched {
            self.cells[cell].state = 0;
        }
        active.sort_unstable_by_key(|&cell| self.cells[cell].depth);
        active
    }

    fn predict_from_active_patterns(&mut self) -> Option<Token> {
        let depth = self
            .active_patterns
            .iter()
            .filter(|&&cell| {
                self.cells[cell]
                    .outgoing
                    .iter()
                    .any(|&arrow| self.arrows[arrow].use_ == ArrowUse::Predict)
            })
            .map(|&cell| self.cells[cell].depth)
            .max()?;

        let sources: Vec<_> = self
            .active_patterns
            .iter()
            .copied()
            .filter(|&cell| self.cells[cell].depth == depth)
            .collect();
        let mut touched = Vec::new();
        for source in sources {
            let outgoing = self.cells[source].outgoing.clone();
            for arrow_id in outgoing {
                if self.arrows[arrow_id].use_ == ArrowUse::Predict {
                    touched.push(self.arrows[arrow_id].to);
                    self.queue_arrow(arrow_id);
                }
            }
        }

        let mut processed = 0;
        while let Some(spike) = self.queue.pop_front() {
            if processed == self.activity_limit {
                self.activity_limit_hits += 1;
                self.queue.clear();
                break;
            }
            processed += 1;
            self.total_spikes += 1;
            self.cells[spike.to].state = self.cells[spike.to].state.saturating_add(spike.value);
        }

        touched.sort_unstable();
        touched.dedup();
        let mut best = None;
        let mut tied = false;
        for receptor in touched.iter().copied() {
            let strength = self.cells[receptor].state;
            match best {
                None => {
                    best = Some((receptor, strength));
                    tied = false;
                }
                Some((_, best_strength)) if strength > best_strength => {
                    best = Some((receptor, strength));
                    tied = false;
                }
                Some((_, best_strength)) if strength == best_strength => {
                    tied = true;
                }
                Some(_) => {}
            }
        }
        for receptor in touched {
            self.cells[receptor].state = 0;
        }

        if tied {
            None
        } else {
            best.map(|(receptor, _)| receptor as Token)
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

fn induction_probe(
    learner: &mut UnifiedLearner,
    trials: usize,
    sequence_len: usize,
    seed: u64,
) -> (usize, usize) {
    let mut rng = DeterministicRng::new(seed);
    let mut correct = 0;
    let mut total = 0;

    for _ in 0..trials {
        let sequence: Vec<_> = (0..sequence_len).map(|_| rng.token(0, 64)).collect();
        learner.reset_activity();
        for &token in &sequence {
            learner.absorb(token);
        }

        learner.absorb(251);
        learner.absorb(sequence[0]);
        for &expected in &sequence[1..] {
            correct += usize::from(learner.answer() == Some(expected));
            total += 1;
            learner.absorb(expected);
        }
    }
    (correct, total)
}

fn train_key_values(learner: &mut UnifiedLearner) -> Vec<(Token, Token)> {
    let pairs: Vec<_> = (0..32).map(|index| (64 + index, 96 + index)).collect();
    learner.reset_activity();
    for &(key, value) in &pairs {
        learner.absorb(250);
        learner.absorb(key);
        learner.absorb(value);
    }
    pairs
}

fn recall_pairs(learner: &UnifiedLearner, pairs: &[(Token, Token)]) -> usize {
    pairs
        .iter()
        .filter(|&&(key, value)| {
            let mut query = learner.clone();
            query.reset_activity();
            query.absorb(key);
            query.answer() == Some(value)
        })
        .count()
}

fn needle_probe(learner: &mut UnifiedLearner, length: usize, seed: u64) -> (usize, usize) {
    let facts = [(128, 129), (130, 131), (132, 133)];
    let placements = [length / 10, length / 2, length * 9 / 10];
    let mut rng = DeterministicRng::new(seed);
    learner.reset_activity();

    let mut fact_index = 0;
    let mut position = 0;
    while position < length {
        if fact_index < facts.len() && position == placements[fact_index] {
            learner.absorb(facts[fact_index].0);
            learner.absorb(facts[fact_index].1);
            fact_index += 1;
            position += 2;
        } else {
            learner.absorb(rng.token(134, 116));
            position += 1;
        }
    }

    let correct = recall_pairs(learner, &facts);
    (correct, facts.len())
}

#[derive(Clone, Debug)]
pub struct UnifiedReport {
    pub induction_correct: usize,
    pub induction_total: usize,
    pub short_context_correct: usize,
    pub short_context_total: usize,
    pub recalled_pairs: usize,
    pub total_pairs: usize,
    pub recalled_needles: usize,
    pub total_needles: usize,
    pub remapped_control_correct: usize,
    pub unknown_query_rejected: bool,
    pub metrics: LearnerMetrics,
    pub passed: bool,
}

impl UnifiedReport {
    pub fn induction_accuracy(&self) -> f64 {
        self.induction_correct as f64 / self.induction_total as f64
    }

    pub fn short_context_accuracy(&self) -> f64 {
        self.short_context_correct as f64 / self.short_context_total as f64
    }
}

pub fn run_experiment() -> UnifiedReport {
    let mut learner = UnifiedLearner::new(256, 8, 4_096);
    let (induction_correct, induction_total) = induction_probe(&mut learner, 24, 64, 0x1600_0001);

    let mut short_context = UnifiedLearner::new(256, 1, 4_096);
    let (short_context_correct, short_context_total) =
        induction_probe(&mut short_context, 24, 64, 0x1600_0001);

    let pairs = train_key_values(&mut learner);
    let recalled_pairs = recall_pairs(&learner, &pairs);
    let (recalled_needles, total_needles) = needle_probe(&mut learner, 8_192, 0x1600_0002);

    let remapped: Vec<_> = pairs
        .iter()
        .enumerate()
        .map(|(index, &(key, _))| (key, pairs[(index + 1) % pairs.len()].1))
        .collect();
    let remapped_control_correct = recall_pairs(&learner, &remapped);

    let mut unknown_query = learner.clone();
    unknown_query.reset_activity();
    unknown_query.absorb(252);
    let unknown_query_rejected = unknown_query.answer().is_none();
    let metrics = learner.metrics();

    let induction_accuracy = induction_correct as f64 / induction_total as f64;
    let short_context_accuracy = short_context_correct as f64 / short_context_total as f64;
    let passed = induction_accuracy >= 0.97
        && induction_accuracy >= short_context_accuracy + 0.20
        && recalled_pairs == pairs.len()
        && recalled_needles == total_needles
        && remapped_control_correct == 0
        && unknown_query_rejected
        && metrics.activity_limit_hits == 0;

    UnifiedReport {
        induction_correct,
        induction_total,
        short_context_correct,
        short_context_total,
        recalled_pairs,
        total_pairs: pairs.len(),
        recalled_needles,
        total_needles,
        remapped_control_correct,
        unknown_query_rejected,
        metrics,
        passed,
    }
}

pub fn print_report(report: &UnifiedReport) {
    println!("v16 one-learner integration:");
    println!(
        "  induction copying: {}/{} ({:.1}%), one-token context control: {:.1}%",
        report.induction_correct,
        report.induction_total,
        report.induction_accuracy() * 100.0,
        report.short_context_accuracy() * 100.0
    );
    println!(
        "  many-key recall: {}/{}, deep needles: {}/{}",
        report.recalled_pairs, report.total_pairs, report.recalled_needles, report.total_needles
    );
    println!(
        "  remapped control correct: {}, unknown query rejected: {}",
        report.remapped_control_correct, report.unknown_query_rejected
    );
    println!(
        "  learned pattern cells: {}, arrows: {}, spikes: {}, activity limit hits: {}",
        report.metrics.learned_pattern_cells,
        report.metrics.arrows,
        report.metrics.spikes,
        report.metrics.activity_limit_hits
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v16_cells_and_arrows_learn_a_successor() {
        let mut learner = UnifiedLearner::new(256, 4, 128);
        learner.absorb(10);
        learner.absorb(20);
        learner.reset_activity();
        learner.absorb(10);

        assert_eq!(learner.answer(), Some(20));
        assert!(learner.metrics().learned_pattern_cells >= 2);
        assert!(learner.metrics().prediction_arrows >= 1);
    }

    #[test]
    fn v16_hierarchical_patterns_beat_one_token_memory_on_induction() {
        let report = run_experiment();

        assert!(report.induction_accuracy() >= 0.97);
        assert!(
            report.induction_accuracy() >= report.short_context_accuracy() + 0.20,
            "full={:.3}, short={:.3}",
            report.induction_accuracy(),
            report.short_context_accuracy()
        );
    }

    #[test]
    fn v16_one_learner_recalls_many_pairs_and_deep_needles() {
        let report = run_experiment();

        assert_eq!(report.recalled_pairs, report.total_pairs);
        assert_eq!(report.recalled_needles, report.total_needles);
    }

    #[test]
    fn v16_controls_reject_remapping_and_unknown_queries() {
        let report = run_experiment();

        assert_eq!(report.remapped_control_correct, 0);
        assert!(report.unknown_query_rejected);
        assert_eq!(report.metrics.activity_limit_hits, 0);
        assert!(report.passed);
    }
}
