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
            let source = source_audit();
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
