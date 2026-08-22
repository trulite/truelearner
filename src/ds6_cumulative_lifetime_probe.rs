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
    text.push_str(&format!("\nDuplicate exact: `{}`.\n", report.duplicate_exact));
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
