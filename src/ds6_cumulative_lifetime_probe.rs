//! Development-only DS6 path-existence diagnostic over frozen M3 event physics.

pub const PROTOCOL: &str = "ds6-cumulative-lifetime-probe-v1";
pub const PROTOCOL_COMMIT: &str = "24e658ad84b88100ac81ad76ab17035b755d6687";
pub const AUTHORITATIVE_M3: &str = "ffcdfe8b36fc62348b7ebcb09aaf4797f6146ba8";
pub const FROZEN_M3_SHA256: &str =
    "a8d8fe060b497c7a6b5f9a5a88b7ed2292dc8a729a8781f599547b6027efc0a0";
pub const FROZEN_TARGET_SHA256: &str =
    "f10f9d7b16106b6014767ff6188a6d556145ba3e5b4335e28de245c7622a7595";
pub const FROZEN_ORDER_SHA256: &str =
    "609dc63ab8051316703899717fc30861d7a700d0ec60f205fa6d687ad478616d";
pub const FROZEN_AUDIT_SHA256: &str =
    "5d896fad16a4a38847de470a6a69f4cea5cd6f4fee5e71900a31d125be45b983";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "dd2fa0cf33acde8592be5c92e31f2aa3a883ebff222eb10af95d7c9dc2ad6ead";
pub const PROBE_SEED: u64 = 106_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    pub name: &'static str,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArmReport {
    pub arm: &'static str,
    pub physical_path: bool,
    pub passed: bool,
    pub first_collapse: &'static str,
    pub checks: Vec<Check>,
    pub final_records: usize,
    pub raw_records: usize,
    pub diagnostic: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeReport {
    pub protocol: &'static str,
    pub seed: u64,
    pub arms: Vec<ArmReport>,
    pub passing_arms: usize,
    pub selected_arm: Option<&'static str>,
    pub scientific_ambiguity: bool,
    pub diagnostic_complete: bool,
    pub duplicate_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MicroCell {
    pub seed: u64,
    pub history_ordered: bool,
    pub useful_survived: usize,
    pub useful_total: usize,
    pub oneoffs_removed: usize,
    pub oneoffs_total: usize,
    pub short_gap_survived_and_strengthened: bool,
    pub long_gap_disappeared: bool,
    pub contradiction_lost_advantage: bool,
    pub stale_path_blocked: bool,
    pub reacquired: usize,
    pub fresh_layout_exact: bool,
    pub economy: bool,
    pub controls: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MicroReport {
    pub protocol: &'static str,
    pub cells: Vec<MicroCell>,
    pub duplicate_exact: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchedCell {
    pub seed: u64,
    pub recurrence_strengths: Vec<i32>,
    pub disuse_strengths: Vec<i32>,
    pub high_long_strength: i32,
    pub low_short_strength: i32,
    pub reuse_delta: i32,
    pub fresh_layout_exact: bool,
    pub controls: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchedReport {
    pub protocol: &'static str,
    pub cells: Vec<MatchedCell>,
    pub duplicate_exact: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateCell {
    pub seed: u64,
    pub recurrence_ordering: bool,
    pub pressure_ordering: bool,
    pub lifetimes: Vec<usize>,
    pub dynamic_lifetime: bool,
    pub crossed_tradeoff: bool,
    pub interleaving_invariant: bool,
    pub load_behavior: bool,
    pub gap_reuse: bool,
    pub contradiction_history: bool,
    pub cumulative_m3: bool,
    pub controls: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateReport {
    pub protocol: &'static str,
    pub cells: Vec<GateCell>,
    pub duplicate_exact: bool,
    pub passed: bool,
}

macro_rules! ds6_m3_access {
    () => {
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        struct ScalarLifecycle {
            records: BTreeMap<ChunkSignature, i32>,
            completed: usize,
            pressure_enabled: bool,
        }

        impl ScalarLifecycle {
            fn new(pressure_enabled: bool) -> Self {
                Self {
                    pressure_enabled,
                    ..Self::default()
                }
            }

            fn observe(&mut self, key: ChunkSignature) {
                if let Some(strength) = self.records.get_mut(&key) {
                    // Frozen parts-bin successful recurrence credit.
                    *strength += 2;
                } else {
                    self.records.insert(key, 1);
                }
                self.completed += 1;
                // Four ordinary completed event propagations are the physical
                // clock. There is no rest/session/cleanup call.
                if self.pressure_enabled && self.completed % 4 == 0 {
                    for strength in self.records.values_mut() {
                        *strength -= 1;
                    }
                    self.records.retain(|_, strength| *strength > 0);
                }
            }

            fn available(&self, key: &ChunkSignature) -> bool {
                self.records.contains_key(key)
            }

            fn bytes(&self) -> usize {
                self.records
                    .keys()
                    .map(|key| 2 * key.roles.len() + 2 + std::mem::size_of::<i32>())
                    .sum()
            }

            fn strength(&self, key: &ChunkSignature) -> i32 {
                self.records.get(key).copied().unwrap_or(0)
            }
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        struct KeepAll {
            records: BTreeMap<ChunkSignature, i32>,
        }

        impl KeepAll {
            fn observe(&mut self, key: ChunkSignature) {
                *self.records.entry(key).or_default() += 1;
            }
        }

        fn event(seed: u64, relation: u8) -> (ChunkSignature, bool) {
            let stream = fixture(seed, &[3], &[7, 7, 7], relation);
            let key = signature(&stream.observations);
            let mut generic = BoundaryLearner::default();
            let evaluation = generic.evaluate(&stream.observations, false);
            (key, evaluation.spans == supplied(&stream))
        }

        fn event_len(seed: u64, relation: u8, length: usize) -> (ChunkSignature, bool) {
            let shapes = vec![7; length];
            let stream = fixture(seed, &[length], &shapes, relation);
            let key = signature(&stream.observations);
            let mut generic = BoundaryLearner::default();
            let evaluation = generic.evaluate(&stream.observations, false);
            (key, evaluation.spans == supplied(&stream))
        }

        fn broken_event(seed: u64, relation: u8) -> (ChunkSignature, bool) {
            let mut stream = fixture(seed, &[3], &[7, 7, 7], relation);
            let old = signature(&stream.observations);
            stream.observations[1].causal_link = CausalLink::Reset;
            let changed = signature(&stream.observations);
            let mut generic = BoundaryLearner::default();
            let evaluation = generic.evaluate(&stream.observations, false);
            (changed, old != signature(&stream.observations) && evaluation.spans.is_empty())
        }

        fn check(name: &'static str, passed: bool) -> super::Check {
            super::Check { name, passed }
        }

        fn scalar_probe(pressure_enabled: bool) -> (ScalarLifecycle, KeepAll, Vec<super::Check>) {
            let mut life = ScalarLifecycle::new(pressure_enabled);
            let mut raw = KeepAll::default();
            let mut exact = true;

            for offset in 0..4 {
                for relation in [10u8, 11u8] {
                    let (key, reconstructed) =
                        event(super::PROBE_SEED + offset * 10 + relation as u64, relation);
                    exact &= reconstructed;
                    life.observe(key.clone());
                    raw.observe(key);
                }
            }

            let useful = [event(super::PROBE_SEED + 900, 10).0, event(super::PROBE_SEED + 901, 11).0];

            for ordinal in 0..8u8 {
                let relation = 20 + ordinal;
                let (key, reconstructed) =
                    event(super::PROBE_SEED + 1_000 + ordinal as u64, relation);
                exact &= reconstructed;
                life.observe(key.clone());
                raw.observe(key);
            }

            for offset in 0..4 {
                let (key, reconstructed) = event(super::PROBE_SEED + 2_000 + offset, 30);
                exact &= reconstructed;
                life.observe(key.clone());
                raw.observe(key);
            }
            let old_changed = event(super::PROBE_SEED + 2_100, 30).0;
            let (changed, broken_reopens) = broken_event(super::PROBE_SEED + 2_101, 30);
            let stale_blocked = changed != old_changed && !life.available(&changed);

            for ordinal in 0..4u8 {
                let relation = 40 + ordinal;
                let (key, reconstructed) =
                    event(super::PROBE_SEED + 3_000 + ordinal as u64, relation);
                exact &= reconstructed;
                life.observe(key.clone());
                raw.observe(key);
            }

            let useful_persist = useful.iter().all(|key| life.available(key));
            let oneoffs_removed = (20..28).all(|relation| {
                let key = event(super::PROBE_SEED + 4_000 + relation as u64, relation).0;
                !life.available(&key)
            });

            for (ordinal, relation) in [10u8, 11u8].into_iter().enumerate() {
                let (key, reconstructed) =
                    event(super::PROBE_SEED + 5_000 + ordinal as u64, relation);
                exact &= reconstructed && life.available(&key);
                life.observe(key.clone());
                raw.observe(key);
            }

            let relearn_key = event(super::PROBE_SEED + 6_000, 20).0;
            let absent_before_relearn = !life.available(&relearn_key);
            for offset in 0..4 {
                let (key, reconstructed) = event(super::PROBE_SEED + 6_010 + offset, 20);
                exact &= reconstructed;
                life.observe(key.clone());
                raw.observe(key);
            }
            let relearned = absent_before_relearn && life.available(&relearn_key);

            let no_tiers = true;
            let economy = life.records.len() < raw.records.len() && life.bytes() > 0;
            let source = source_ok();
            let checks = vec![
                check("source/information audit", source),
                check("single scalar lifecycle", no_tiers),
                check("useful persistence", useful_persist && exact),
                check("one-off removal", oneoffs_removed),
                check("contradiction response", stale_blocked && broken_reopens),
                check("relearning", relearned),
                check("economy/lifecycle", economy),
            ];
            (life, raw, checks)
        }

        pub(super) fn run_arm_a() -> super::ArmReport {
            let (life, raw, mut checks) = scalar_probe(true);
            let duplicate = scalar_probe(true);
            let exact_duplicate = life == duplicate.0 && raw == duplicate.1 && checks == duplicate.2;
            checks.push(check("determinism", exact_duplicate));

            let no_pressure = scalar_probe(false).0;
            let pressure_control = no_pressure.records.len() > life.records.len();
            let no_recurrence = {
                let mut shuffled = ScalarLifecycle::new(true);
                for ordinal in 0..16u8 {
                    shuffled.observe(event(super::PROBE_SEED + 7_000 + ordinal as u64, 80 + ordinal).0);
                }
                !shuffled.available(&event(super::PROBE_SEED + 8_000, 80).0)
            };
            let controls = pressure_control && no_recurrence && raw.records.len() > life.records.len();
            checks.push(check("frozen controls", controls));

            let first = checks.iter().find(|row| !row.passed).map_or("NONE", |row| row.name);
            let passed = checks.iter().all(|row| row.passed);
            super::ArmReport {
                arm: "A recurrence/use competition",
                physical_path: true,
                passed,
                first_collapse: first,
                checks,
                final_records: life.records.len(),
                raw_records: raw.records.len(),
                diagnostic: "one anonymous scalar record; recurrence/use opposes ordinary periodic pressure",
            }
        }

        pub(super) fn source_ok() -> bool {
            source_audit()
                && env!("DS6_M3_SHA256") == super::FROZEN_M3_SHA256
                && env!("DS6_TARGET_SHA256") == super::FROZEN_TARGET_SHA256
                && env!("DS6_ORDER_SHA256") == super::FROZEN_ORDER_SHA256
                && env!("DS6_AUDIT_SHA256") == super::FROZEN_AUDIT_SHA256
                && env!("DS6_PROTOCOL_SHA256") == super::FROZEN_PROTOCOL_SHA256
        }

        fn pressure_activity(life: &mut ScalarLifecycle, seed: u64, count: u8) {
            for ordinal in 0..count {
                life.observe(event(seed + ordinal as u64, 100 + ordinal).0);
            }
        }

        fn pressure_ticks(life: &mut ScalarLifecycle, seed: u64, ticks: usize) {
            let target = life.completed / 4 + ticks;
            let mut ordinal = 0u8;
            while life.completed / 4 < target {
                life.observe(event(seed + ordinal as u64, 120 + ordinal).0);
                ordinal += 1;
            }
        }

        fn recurrence(life: &mut ScalarLifecycle, seed: u64, relation: u8, count: usize) {
            for ordinal in 0..count {
                life.observe(event(seed + ordinal as u64, relation).0);
            }
        }

        pub(super) fn run_micro_cell(seed: u64, reverse: bool) -> super::MicroCell {
            let mut life = ScalarLifecycle::new(true);
            let mut raw = KeepAll::default();
            let mut schedule = vec![(10u8, 4usize), (11, 6), (12, 8), (13, 10)];
            if reverse {
                schedule.reverse();
            }
            for (relation, count) in &schedule {
                for ordinal in 0..*count {
                    let key = event(seed + *relation as u64 * 100 + ordinal as u64, *relation).0;
                    life.observe(key.clone());
                    raw.observe(key);
                }
            }
            let useful = (10u8..14)
                .map(|relation| event(seed + 20_000 + relation as u64, relation).0)
                .collect::<Vec<_>>();
            let before = useful
                .iter()
                .map(|key| life.strength(key))
                .collect::<Vec<_>>();
            for relation in 20u8..36 {
                let key = event(seed + 30_000 + relation as u64, relation).0;
                life.observe(key.clone());
                raw.observe(key);
            }
            let useful_survived = useful.iter().filter(|key| life.available(key)).count();
            let history_ordered = before.windows(2).all(|pair| pair[0] <= pair[1]);
            let oneoffs_removed = (20u8..36)
                .filter(|relation| {
                    !life.available(&event(seed + 31_000 + *relation as u64, *relation).0)
                })
                .count();

            let mut short = ScalarLifecycle::new(true);
            recurrence(&mut short, seed + 40_000, 40, 4);
            let gap_key = event(seed + 41_000, 40).0;
            pressure_activity(&mut short, seed + 42_000, 12);
            let short_before = short.strength(&gap_key);
            recurrence(&mut short, seed + 43_000, 40, 1);
            let short_gap_survived_and_strengthened =
                short_before > 0 && short.strength(&gap_key) > short_before;

            let mut long = ScalarLifecycle::new(true);
            recurrence(&mut long, seed + 44_000, 41, 4);
            let long_key = event(seed + 45_000, 41).0;
            pressure_activity(&mut long, seed + 46_000, 24);
            let long_gap_disappeared = !long.available(&long_key);

            let mut contradicted = ScalarLifecycle::new(true);
            let mut matched = ScalarLifecycle::new(true);
            recurrence(&mut contradicted, seed + 50_000, 50, 4);
            recurrence(&mut matched, seed + 50_000, 50, 4);
            let original = event(seed + 51_000, 50).0;
            let (changed, broken) = broken_event(seed + 51_001, 50);
            pressure_activity(&mut contradicted, seed + 52_000, 8);
            recurrence(&mut matched, seed + 53_000, 50, 4);
            let contradiction_lost_advantage =
                contradicted.strength(&original) < matched.strength(&original);
            let stale_path_blocked = changed != original && !contradicted.available(&changed) && broken;

            let removed = [20u8, 21u8];
            let mut reacquired = 0;
            for relation in removed {
                let key = event(seed + 60_000 + relation as u64, relation).0;
                if !life.available(&key) {
                    recurrence(&mut life, seed + 61_000 + relation as u64 * 10, relation, 4);
                    reacquired += usize::from(life.available(&key));
                }
            }

            let mut no_pressure = ScalarLifecycle::new(false);
            for relation in 20u8..36 {
                no_pressure.observe(event(seed + 70_000 + relation as u64, relation).0);
            }
            let controls = no_pressure.records.len() == 16 && oneoffs_removed == 16;
            let fresh_layout_exact = useful.iter().all(|key| {
                let relation = key.relation;
                *key == event(seed ^ 0x5a5a_5a5a, relation).0
            });
            let economy = life.records.len() < raw.records.len() && life.bytes() > 0;
            let passed = history_ordered
                && useful_survived == useful.len()
                && oneoffs_removed == 16
                && short_gap_survived_and_strengthened
                && long_gap_disappeared
                && contradiction_lost_advantage
                && stale_path_blocked
                && reacquired == 2
                && fresh_layout_exact
                && economy
                && controls;
            super::MicroCell {
                seed,
                history_ordered,
                useful_survived,
                useful_total: useful.len(),
                oneoffs_removed,
                oneoffs_total: 16,
                short_gap_survived_and_strengthened,
                long_gap_disappeared,
                contradiction_lost_advantage,
                stale_path_blocked,
                reacquired,
                fresh_layout_exact,
                economy,
                controls,
                passed,
            }
        }

        pub(super) fn run_matched_cell(seed: u64, reverse: bool) -> super::MatchedCell {
            let counts = [1usize, 2, 4, 8];
            let mut recurrence_strengths = Vec::new();
            let mut fresh_layout_exact = source_ok();
            for (index, count) in counts.into_iter().enumerate() {
                let relation = 10 + index as u8;
                let mut life = ScalarLifecycle::new(true);
                recurrence(&mut life, seed + index as u64 * 1_000, relation, count);
                let key = event(seed + 20_000 + index as u64, relation).0;
                pressure_ticks(&mut life, seed + 21_000 + index as u64 * 100, 4);
                recurrence_strengths.push(life.strength(&key));
                let relabelled = event((seed ^ 0x6d6d_0000) + index as u64, relation).0;
                fresh_layout_exact &= key == relabelled;
            }

            let gaps = [2usize, 4, 8, 12];
            let mut disuse_strengths = Vec::new();
            for (index, ticks) in gaps.into_iter().enumerate() {
                let relation = 20 + index as u8;
                let mut life = ScalarLifecycle::new(true);
                recurrence(&mut life, seed + 30_000 + index as u64 * 1_000, relation, 6);
                let key = event(seed + 31_000 + index as u64, relation).0;
                pressure_ticks(&mut life, seed + 32_000 + index as u64 * 100, ticks);
                disuse_strengths.push(life.strength(&key));
            }

            let mut high_long = ScalarLifecycle::new(true);
            recurrence(&mut high_long, seed + 40_000, 40, 8);
            let high_key = event(seed + 40_100, 40).0;
            pressure_ticks(&mut high_long, seed + 41_000, 8);
            let high_long_strength = high_long.strength(&high_key);

            let mut low_short = ScalarLifecycle::new(true);
            recurrence(&mut low_short, seed + 42_000, 41, 2);
            let low_key = event(seed + 42_100, 41).0;
            pressure_ticks(&mut low_short, seed + 43_000, 2);
            let low_short_strength = low_short.strength(&low_key);

            let mut reused = ScalarLifecycle::new(true);
            recurrence(&mut reused, seed + 44_000, 42, 6);
            let reuse_key = event(seed + 44_100, 42).0;
            pressure_ticks(&mut reused, seed + 45_000, 4);
            let before_reuse = reused.strength(&reuse_key);
            recurrence(&mut reused, seed + 46_000, 42, 1);
            let reuse_delta = reused.strength(&reuse_key) - before_reuse;

            let mut oneoffs = ScalarLifecycle::new(true);
            for relation in 50u8..58 {
                oneoffs.observe(event(seed + 50_000 + relation as u64, relation).0);
            }
            let oneoffs_removed = (50u8..58).all(|relation| {
                !oneoffs.available(&event(seed + 51_000 + relation as u64, relation).0)
            });
            recurrence(&mut oneoffs, seed + 52_000, 50, 4);
            let reacquired = oneoffs.available(&event(seed + 52_100, 50).0);

            let mut contradicted = ScalarLifecycle::new(true);
            recurrence(&mut contradicted, seed + 53_000, 60, 4);
            let original = event(seed + 53_100, 60).0;
            let (changed, broken) = broken_event(seed + 53_101, 60);
            let stale_blocked = changed != original && !contradicted.available(&changed) && broken;

            let mut no_pressure = ScalarLifecycle::new(false);
            for relation in 70u8..78 {
                no_pressure.observe(event(seed + 54_000 + relation as u64, relation).0);
            }
            let controls = oneoffs_removed
                && reacquired
                && stale_blocked
                && no_pressure.records.len() == 8
                && oneoffs.bytes() > 0;

            if reverse {
                fresh_layout_exact &= event(seed ^ 0xffff, 90).0 == event(seed, 90).0;
            }
            let recurrence_ordered = recurrence_strengths[0] <= recurrence_strengths[1]
                && recurrence_strengths[1] < recurrence_strengths[2]
                && recurrence_strengths[2] < recurrence_strengths[3];
            let disuse_ordered = disuse_strengths.windows(2).all(|pair| pair[0] > pair[1])
                && disuse_strengths[3] == 0;
            let interaction_exact = high_long_strength == 5 && low_short_strength == 1;
            let passed = recurrence_ordered
                && disuse_ordered
                && interaction_exact
                && reuse_delta == 2
                && fresh_layout_exact
                && controls;
            super::MatchedCell {
                seed,
                recurrence_strengths,
                disuse_strengths,
                high_long_strength,
                low_short_strength,
                reuse_delta,
                fresh_layout_exact,
                controls,
                passed,
            }
        }

        fn strength_after(seed: u64, recurrences: usize, ticks: usize) -> i32 {
            let mut life = ScalarLifecycle::new(true);
            recurrence(&mut life, seed, 10, recurrences);
            let key = event(seed + 900, 10).0;
            pressure_ticks(&mut life, seed + 1_000, ticks);
            life.strength(&key)
        }

        fn deallocation_ticks(seed: u64, recurrences: usize) -> usize {
            let mut life = ScalarLifecycle::new(true);
            recurrence(&mut life, seed, 11, recurrences);
            let key = event(seed + 900, 11).0;
            let mut ticks = 0;
            while life.available(&key) && ticks < 64 {
                pressure_ticks(&mut life, seed + 2_000 + ticks as u64 * 10, 1);
                ticks += 1;
            }
            ticks
        }

        pub(super) fn run_gate_cell(seed: u64) -> super::GateCell {
            let recurrence_counts = [1usize, 2, 4, 8, 16];
            let pressure_counts = [0usize, 2, 4, 8, 12, 16];
            let mut grid = Vec::new();
            for (row, recurrences) in recurrence_counts.into_iter().enumerate() {
                let strengths = pressure_counts
                    .into_iter()
                    .enumerate()
                    .map(|(column, ticks)| {
                        strength_after(
                            seed + row as u64 * 10_000 + column as u64 * 500,
                            recurrences,
                            ticks,
                        )
                    })
                    .collect::<Vec<_>>();
                grid.push(strengths);
            }
            let recurrence_ordering = (0..pressure_counts.len()).all(|column| {
                let values = grid.iter().map(|row| row[column]).collect::<Vec<_>>();
                values.windows(2).all(|pair| pair[0] <= pair[1])
                    && values[1..].windows(2).all(|pair| {
                        pair[0] == 0 || pair[1] == 0 || pair[0] < pair[1]
                    })
            });
            let pressure_ordering = grid.iter().all(|row| {
                row.windows(2).all(|pair| pair[0] >= pair[1])
                    && row.windows(2).all(|pair| pair[0] == 0 || pair[0] > pair[1])
            });
            let lifetimes = recurrence_counts
                .into_iter()
                .enumerate()
                .map(|(index, count)| deallocation_ticks(seed + 100_000 + index as u64 * 5_000, count))
                .collect::<Vec<_>>();
            let dynamic_lifetime = lifetimes.windows(2).all(|pair| pair[0] <= pair[1])
                && lifetimes[1..].windows(2).all(|pair| pair[0] < pair[1]);

            let crossed = [
                (strength_after(seed + 130_000, 4, 8), strength_after(seed + 131_000, 2, 2)),
                (strength_after(seed + 132_000, 8, 16), strength_after(seed + 133_000, 4, 4)),
                (strength_after(seed + 134_000, 16, 16), strength_after(seed + 135_000, 8, 16)),
            ];
            let crossed_tradeoff = crossed
                .iter()
                .all(|(left, right)| (*left == 0) != (*right == 0));

            let mut ascending = ScalarLifecycle::new(true);
            let mut descending = ScalarLifecycle::new(true);
            recurrence(&mut ascending, seed + 140_000, 70, 8);
            recurrence(&mut descending, seed + 140_000, 70, 8);
            for ordinal in 0..32u8 {
                ascending.observe(event(seed + 141_000 + ordinal as u64, 80 + ordinal).0);
                descending.observe(event(seed + 141_000 + ordinal as u64, 111 - ordinal).0);
            }
            let target = event(seed + 142_000, 70).0;
            let interleaving_invariant = ascending.strength(&target) == descending.strength(&target)
                && ascending.records.len() == descending.records.len();

            let mut load_behavior = true;
            for (index, load) in [8u8, 32, 128].into_iter().enumerate() {
                let mut life = ScalarLifecycle::new(true);
                recurrence(&mut life, seed + 150_000 + index as u64 * 10_000, 71, 8);
                let load_target = event(seed + 151_000 + index as u64, 71).0;
                for ordinal in 0..load {
                    life.observe(event(seed + 152_000 + ordinal as u64, 80 + ordinal).0);
                }
                let expected = (13 - i32::from(load) / 4).max(0);
                load_behavior &= life.strength(&load_target) == expected;
                load_behavior &= (0..load).all(|ordinal| {
                    !life.available(&event(seed + 153_000 + ordinal as u64, 80 + ordinal).0)
                });
            }

            let mut gap = ScalarLifecycle::new(true);
            recurrence(&mut gap, seed + 160_000, 72, 8);
            let gap_key = event(seed + 160_900, 72).0;
            let mut gap_reuse = true;
            for (index, ticks) in [2usize, 4, 8].into_iter().enumerate() {
                let mut branch = gap.clone();
                pressure_ticks(&mut branch, seed + 161_000 + index as u64 * 1_000, ticks);
                let before = branch.strength(&gap_key);
                recurrence(&mut branch, seed + 162_000 + index as u64 * 1_000, 72, 1);
                gap_reuse &= before > 0 && branch.strength(&gap_key) == before + 2;
            }
            let mut removed = ScalarLifecycle::new(true);
            recurrence(&mut removed, seed + 163_000, 73, 2);
            let removed_key = event(seed + 163_900, 73).0;
            pressure_ticks(&mut removed, seed + 164_000, 4);
            gap_reuse &= !removed.available(&removed_key);
            recurrence(&mut removed, seed + 165_000, 73, 1);
            gap_reuse &= removed.strength(&removed_key) == 1;

            let old_key = event_len(seed + 170_000, 74, 3).0;
            let changed_key = event_len(seed + 170_001, 74, 2).0;
            let mut old_base = ScalarLifecycle::new(true);
            for ordinal in 0..8 {
                old_base.observe(event_len(seed + 171_000 + ordinal, 74, 3).0);
            }
            let mut old_strengths = Vec::new();
            let mut changed_strengths = Vec::new();
            let mut contradiction_history = old_key != changed_key;
            for (index, exposure) in [0usize, 2, 4, 8, 16].into_iter().enumerate() {
                let mut branch = old_base.clone();
                for ordinal in 0..exposure {
                    branch.observe(event_len(seed + 172_000 + ordinal as u64, 74, 2).0);
                }
                old_strengths.push(branch.strength(&old_key));
                changed_strengths.push(branch.strength(&changed_key));
                contradiction_history &= !branch.available(&event_len(seed + 173_000, 74, 4).0);
                if index + 1 == 5 {
                    let before = branch.strength(&old_key);
                    branch.observe(event_len(seed + 174_000, 74, 3).0);
                    contradiction_history &= branch.strength(&old_key)
                        == if before > 0 { before + 2 } else { 1 };
                }
            }
            contradiction_history &= old_strengths.windows(2).all(|pair| pair[0] >= pair[1]);
            contradiction_history &= changed_strengths.windows(2).all(|pair| pair[0] <= pair[1]);

            let cumulative_m3 = (10u8..18).all(|relation| {
                event(seed ^ 0xa5a5_0000, relation).1
                    && event(seed ^ 0x5a5a_0000, relation).0 == event(seed, relation).0
            });
            let mut keep_all = ScalarLifecycle::new(false);
            for relation in 20u8..52 {
                keep_all.observe(event(seed + 180_000 + relation as u64, relation).0);
            }
            let mut shuffled = ScalarLifecycle::new(true);
            for relation in 20u8..52 {
                shuffled.observe(event(seed + 181_000 + relation as u64, relation).0);
            }
            let controls = keep_all.records.len() == 32
                && shuffled.records.is_empty()
                && keep_all.bytes() > shuffled.bytes();
            let passed = source_ok()
                && recurrence_ordering
                && pressure_ordering
                && dynamic_lifetime
                && crossed_tradeoff
                && interleaving_invariant
                && load_behavior
                && gap_reuse
                && contradiction_history
                && cumulative_m3
                && controls;
            super::GateCell {
                seed,
                recurrence_ordering,
                pressure_ordering,
                lifetimes,
                dynamic_lifetime,
                crossed_tradeoff,
                interleaving_invariant,
                load_behavior,
                gap_reuse,
                contradiction_history,
                cumulative_m3,
                controls,
                passed,
            }
        }
    };
}

#[allow(dead_code)]
mod frozen_m3 {
    include!(concat!(env!("OUT_DIR"), "/ds6_ds3_event_boundary.rs"));
    ds6_m3_access!();
}

fn no_path_arm(arm: &'static str, diagnostic: &'static str) -> ArmReport {
    let checks = vec![
        Check {
            name: "source/information audit",
            passed: frozen_m3::source_ok(),
        },
        Check {
            name: "single lifecycle",
            passed: false,
        },
    ];
    ArmReport {
        arm,
        physical_path: false,
        passed: false,
        first_collapse: "single lifecycle",
        checks,
        final_records: 0,
        raw_records: 0,
        diagnostic,
    }
}

fn run_once() -> Vec<ArmReport> {
    vec![
        frozen_m3::run_arm_a(),
        no_path_arm(
            "B surprise/contradiction timescale",
            "frozen d2.4 trace changes value/tau but has no physical deallocation path",
        ),
        no_path_arm(
            "C dependency/reuse economics",
            "frozen IR0/economics observe invalidation, reuse, bytes, and work but contain no organism-side erase update",
        ),
    ]
}

pub fn run_probe() -> ProbeReport {
    let arms = run_once();
    let duplicate_exact = arms == run_once();
    let passing = arms.iter().filter(|arm| arm.passed).collect::<Vec<_>>();
    let passing_arms = passing.len();
    let selected_arm = (passing_arms == 1).then(|| passing[0].arm);
    ProbeReport {
        protocol: PROTOCOL,
        seed: PROBE_SEED,
        arms,
        passing_arms,
        selected_arm,
        scientific_ambiguity: passing_arms > 1,
        diagnostic_complete: duplicate_exact && passing_arms <= 1,
        duplicate_exact,
    }
}

pub fn print_report(report: &ProbeReport) {
    println!("DS6 cumulative lifetime diagnostic PROBE");
    println!("protocol={} seed={}", report.protocol, report.seed);
    for arm in &report.arms {
        println!(
            "arm={} physical_path={} pass={} first_collapse={} records={}/{} diagnostic={}",
            arm.arm,
            arm.physical_path,
            arm.passed,
            arm.first_collapse,
            arm.final_records,
            arm.raw_records,
            arm.diagnostic
        );
        for check in &arm.checks {
            println!("  check={} pass={}", check.name, check.passed);
        }
    }
    println!(
        "passing_arms={} selected={:?} ambiguity={} duplicate_exact={} diagnostic_complete={}",
        report.passing_arms,
        report.selected_arm,
        report.scientific_ambiguity,
        report.duplicate_exact,
        report.diagnostic_complete
    );
}

fn micro_once() -> Vec<MicroCell> {
    vec![
        frozen_m3::run_micro_cell(107_000, false),
        frozen_m3::run_micro_cell(108_000, true),
    ]
}

pub fn run_micro() -> MicroReport {
    let cells = micro_once();
    let duplicate_exact = cells == micro_once();
    let passed = duplicate_exact && cells.iter().all(|cell| cell.passed);
    MicroReport {
        protocol: "ds6-cumulative-lifetime-micro-v1",
        cells,
        duplicate_exact,
        passed,
    }
}

pub fn render_micro(report: &MicroReport) -> String {
    let mut text = format!(
        "# DS6 cumulative lifetime MICRO result\n\nProtocol: `{}`.\n\nVerdict: **{}**.\n\n| seed | useful | one-offs removed | short gap | long gap | contradiction | stale blocked | reacquired | fresh layout | economy | controls | result |\n|---:|---:|---:|:---:|:---:|:---:|:---:|---:|:---:|:---:|:---:|:---:|\n",
        report.protocol,
        if report.passed { "PASS" } else { "FAIL" }
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "| {} | {}/{} | {}/{} | {} | {} | {} | {} | {}/2 | {} | {} | {} | {} |\n",
            cell.seed,
            cell.useful_survived,
            cell.useful_total,
            cell.oneoffs_removed,
            cell.oneoffs_total,
            cell.short_gap_survived_and_strengthened,
            cell.long_gap_disappeared,
            cell.contradiction_lost_advantage,
            cell.stale_path_blocked,
            cell.reacquired,
            cell.fresh_layout_exact,
            cell.economy,
            cell.controls,
            cell.passed
        ));
    }
    text.push_str(&format!(
        "\nDuplicate exact: `{}`.\n",
        report.duplicate_exact
    ));
    text
}

fn matched_once() -> Vec<MatchedCell> {
    vec![
        frozen_m3::run_matched_cell(109_000, false),
        frozen_m3::run_matched_cell(110_000, true),
    ]
}

pub fn run_matched() -> MatchedReport {
    let cells = matched_once();
    let duplicate_exact = cells == matched_once();
    let passed = duplicate_exact && cells.iter().all(|cell| cell.passed);
    MatchedReport {
        protocol: "ds6-cumulative-lifetime-matched-history-v1",
        cells,
        duplicate_exact,
        passed,
    }
}

pub fn render_matched(report: &MatchedReport) -> String {
    let mut text = format!(
        "# DS6 matched-history lifetime diagnostic result\n\nProtocol: `{}`.\n\nVerdict: **{}**.\n\n| seed | recurrence strengths 1/2/4/8 | disuse strengths 2/4/8/12 | high+long | low+short | reuse delta | fresh layout | controls | result |\n|---:|---|---|---:|---:|---:|:---:|:---:|:---:|\n",
        report.protocol,
        if report.passed { "PASS" } else { "FAIL" }
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "| {} | {:?} | {:?} | {} | {} | {} | {} | {} | {} |\n",
            cell.seed,
            cell.recurrence_strengths,
            cell.disuse_strengths,
            cell.high_long_strength,
            cell.low_short_strength,
            cell.reuse_delta,
            cell.fresh_layout_exact,
            cell.controls,
            cell.passed
        ));
    }
    text.push_str(&format!(
        "\nDuplicate exact: `{}`.\n",
        report.duplicate_exact
    ));
    text
}

fn gate_once() -> Vec<GateCell> {
    (111_000..=116_000)
        .map(frozen_m3::run_gate_cell)
        .collect()
}

pub fn run_gate() -> GateReport {
    let cells = gate_once();
    let duplicate_exact = cells == gate_once();
    let passed = duplicate_exact && cells.iter().all(|cell| cell.passed);
    GateReport {
        protocol: "ds6-cumulative-lifetime-gate-v1",
        cells,
        duplicate_exact,
        passed,
    }
}

pub fn render_gate(report: &GateReport) -> String {
    let mut text = format!(
        "# DS6 cumulative learned-lifetime GATE result\n\nProtocol: `{}`.\n\nVerdict: **{}**.\n\n| seed | recurrence | pressure | lifetimes 1/2/4/8/16 | crossed | interleaving | loads | reuse | contradiction | M3 | controls | result |\n|---:|:---:|:---:|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|\n",
        report.protocol,
        if report.passed { "PASS" } else { "FAIL" }
    );
    for cell in &report.cells {
        text.push_str(&format!(
            "| {} | {} | {} | {:?} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            cell.seed,
            cell.recurrence_ordering,
            cell.pressure_ordering,
            cell.lifetimes,
            cell.crossed_tradeoff,
            cell.interleaving_invariant,
            cell.load_behavior,
            cell.gap_reuse,
            cell.contradiction_history,
            cell.cumulative_m3,
            cell.controls,
            cell.passed
        ));
    }
    text.push_str(&format!(
        "\nDuplicate exact: `{}`.\n",
        report.duplicate_exact
    ));
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_is_deterministic_and_non_authoritative() {
        let report = run_probe();
        assert!(report.duplicate_exact);
        assert!(report.passing_arms <= 1);
        assert_ne!(PROTOCOL, "ds6-cumulative-lifetime-definitive");
    }
}
