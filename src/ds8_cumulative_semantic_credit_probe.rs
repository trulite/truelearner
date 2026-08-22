//! Development-only cumulative DS8 non-semantic-credit path probe.

pub const PROTOCOL: &str = "ds8-cumulative-semantic-credit-probe-v2";
pub const PROTOCOL_COMMIT: &str = "b044ffbbee46e40e256c756f5fb042b017996043";
pub const AUTHORITATIVE_M5: &str = "9c5ba68a6a4ae37b51575ebaae414ab51a248575";
pub const PROBE_SEED: u64 = 40_000_000;
pub const FROZEN_ACTIVATION_SHA256: &str =
    "6e3064a1609390933cda4afdc374579cd23316bb24b833c2024ac14d7138e458";
pub const FROZEN_AUDIT_SHA256: &str =
    "33b963dd50b711f49bc0e90d33adb0d9d80020e5c33ba024d3e861da56d9a326";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "a9a944d3ffab8fe53f303db773846c8f53f7dbb05c8558958567814e0b37f953";
pub const FROZEN_PROTOCOL_V2_SHA256: &str =
    "93cd9c69fa8ae1fc4589c4bed2d1a8add81a87b992bdd0fc9dc3bb33acfe218c";
pub const FROZEN_M5_ALLOCATOR_SHA256: &str =
    "e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7";
pub const FROZEN_M5_GATE_SHA256: &str =
    "abaedd16717543270c5ed0ef2c8a16e3a4c0fed0215764443948c36d4adfa297";
pub const FROZEN_M5_CSV_SHA256: &str =
    "86d9f6e3a8ab4ad5c242e0d7c619d8eda99e0da47faff623f26c8c6835b9a99a";
pub const FROZEN_M5_MD_SHA256: &str =
    "a336633c73565261d357a67ca02df3047ffcaf88488153bb2f43b621818ba5f0";
pub const FROZEN_D3_SHA256: &str =
    "a13f39c86b2c67d225530e7b17cdacd71f452a45be3b2c9942814c0748267f6d";
pub const FROZEN_D2_SHA256: &str =
    "ac257b53e28b0dbcfd4cbcb7ca855086d1de5812a07029f4b2405fda2a6da8f";
pub const FROZEN_C0_SHA256: &str =
    "5c8d00189593ca2f7efb47165efddf85111259f90433a016e5822b5b9578aed2";
pub const FROZEN_CP0_SHA256: &str =
    "c9fcc53d03296b169060499e2304de557f3f7a93744dbc1f935053f99d41c498";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    pub name: &'static str,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeReport {
    pub protocol: &'static str,
    pub seed: u64,
    pub checks: Vec<Check>,
    pub first_updates: usize,
    pub second_updates: usize,
    pub first_admissions: usize,
    pub second_admissions: usize,
    pub shuffled_first_admissions: usize,
    pub shuffled_second_admissions: usize,
    pub swapped_first_admissions: usize,
    pub swapped_second_admissions: usize,
    pub consequence_spikes: u64,
    pub consequence_routes: u64,
    pub first_collapse: &'static str,
    pub duplicate_exact: bool,
    pub passed: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SourceAudit {
    frozen_inputs: bool,
    no_semantic_channels: bool,
    physical_link_exact: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AuditReport {
    pub activation: bool,
    pub dependency: bool,
    pub protocol_v1: bool,
    pub protocol_v2: bool,
    pub allocator: bool,
    pub gate: bool,
    pub csv: bool,
    pub md: bool,
    pub d3: bool,
    pub d2: bool,
    pub c0: bool,
    pub cp0: bool,
    pub forbidden: usize,
    pub linkers: usize,
    pub normalizers: usize,
    pub occurrence_identity: usize,
    pub passed: bool,
}

impl SourceAudit {
    fn passed(self) -> bool {
        self.frozen_inputs && self.no_semantic_channels && self.physical_link_exact
    }
}

#[allow(dead_code)]
mod frozen_m5 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds7_cumulative_plasticity_targeting_probe_frozen.rs"
    ));

    const MINIMUM_DELAY: u8 = 2;
    const RECURRENT_SUPPORT: u16 = 4;
    const MINIMUM_MARGIN: u16 = 2;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RawConsequence {
        occurrences: [u64; 3],
        ticks: [u8; 3],
        arrows: [[u8; 2]; 2],
        root: u8,
    }

    fn raw_consequence(
        seed: u64,
        episode: usize,
        variant: usize,
        immediate: bool,
    ) -> RawConsequence {
        let base = seed
            .wrapping_mul(1_000_003)
            .wrapping_add(episode as u64 * 53)
            .wrapping_add(1 << 33);
        let (root, arrows) = match variant % 4 {
            0 => (0, [[0, 1], [1, 2]]),
            1 => (0, [[0, 2], [2, 1]]),
            2 => (1, [[1, 0], [0, 2]]),
            _ => (2, [[2, 1], [1, 0]]),
        };
        let first_tick = if immediate { 1 } else { MINIMUM_DELAY };
        RawConsequence {
            occurrences: [base, base + 1, base + 2],
            ticks: [first_tick, first_tick + 1, first_tick + 2],
            arrows,
            root,
        }
    }

    // DS8_ORGANISM_PATH_BEGIN

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct ConsequenceShape {
        temporal_rank: [u8; 3],
        propagation: [[u8; 2]; 2],
        activation: [u16; 3],
    }

    impl ConsequenceShape {
        fn magnitude(&self) -> u64 {
            self.activation.iter().map(|value| u64::from(*value)).sum()
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct ConsequenceWork {
        spikes: u64,
        routes: u64,
        temporal_checks: u64,
        comparisons: u64,
        observations: u64,
        abstentions: u64,
    }

    fn execute_and_normalize(
        raw: RawConsequence,
        work: &mut ConsequenceWork,
    ) -> Option<ConsequenceShape> {
        work.temporal_checks += 1;
        if raw.ticks.iter().copied().min()? < MINIMUM_DELAY {
            work.abstentions += 1;
            return None;
        }
        let mut activation = [0u16; 3];
        let mut visited = [false; 3];
        let mut queue = vec![raw.root];
        let mut propagation = Vec::new();
        while let Some(cell) = queue.pop() {
            let index = usize::from(cell);
            if visited[index] {
                continue;
            }
            visited[index] = true;
            activation[index] += 1;
            work.spikes += 1;
            for arrow in raw.arrows.iter().filter(|arrow| arrow[0] == cell) {
                propagation.push(*arrow);
                queue.push(arrow[1]);
                work.routes += 1;
            }
        }
        propagation.sort_unstable();
        let propagation: [[u8; 2]; 2] = propagation.try_into().ok()?;
        let minimum = *raw.ticks.iter().min()?;
        Some(ConsequenceShape {
            temporal_rank: raw.ticks.map(|tick| tick - minimum),
            propagation,
            activation,
        })
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct ConsequenceEvidence {
        shapes: BTreeMap<ConsequenceShape, u16>,
    }

    impl ConsequenceEvidence {
        fn margin(&self) -> (u16, u16) {
            let mut counts = self.shapes.values().copied().collect::<Vec<_>>();
            counts.sort_unstable_by(|left, right| right.cmp(left));
            let first = counts.first().copied().unwrap_or(0);
            let second = counts.get(1).copied().unwrap_or(0);
            (first, first.saturating_sub(second))
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct ConsequenceLearner {
        evidence: BTreeMap<EncounterSnapshot, ConsequenceEvidence>,
        work: ConsequenceWork,
    }

    impl ConsequenceLearner {
        fn observe(&mut self, encounter: EncounterSnapshot, raw: RawConsequence) -> bool {
            let Some(shape) = execute_and_normalize(raw, &mut self.work) else {
                return false;
            };
            let evidence = self.evidence.entry(encounter).or_default();
            self.work.comparisons += evidence.shapes.len() as u64;
            *evidence.shapes.entry(shape).or_default() += 1;
            self.work.observations += 1;
            true
        }

        fn direction(&mut self, encounters: [EncounterSnapshot; 2]) -> Option<EncounterSnapshot> {
            let eligible = encounters.map(|encounter| {
                self.work.comparisons += 1;
                let (support, margin) = self
                    .evidence
                    .get(&encounter)
                    .map(ConsequenceEvidence::margin)
                    .unwrap_or_default();
                support >= RECURRENT_SUPPORT && margin >= MINIMUM_MARGIN
            });
            match eligible {
                [true, false] => Some(encounters[0]),
                [false, true] => Some(encounters[1]),
                _ => {
                    self.work.abstentions += 1;
                    None
                }
            }
        }

        fn apply(
            &mut self,
            path: &mut PlasticityPath,
            active: EncounterSnapshot,
            other: EncounterSnapshot,
            raw: RawConsequence,
        ) -> bool {
            if path.eligibility.is_none() {
                self.work.abstentions += 1;
                return false;
            }
            if !self.observe(active, raw) {
                path.eligibility = None;
                return false;
            }
            let Some(direction) = self.direction([active, other]) else {
                path.eligibility = None;
                return false;
            };
            let differential = direction == active;
            path.delayed_experience(differential)
        }
    }

    // DS8_ORGANISM_PATH_END

    #[derive(Clone)]
    struct Trained {
        path: PlasticityPath,
        learner: ConsequenceLearner,
        first: EncounterSnapshot,
        second: EncounterSnapshot,
        first_updates: usize,
        second_updates: usize,
        all_magnitudes_equal: bool,
        first_blank_executed: bool,
        second_blank_executed: bool,
        executions: usize,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(super) struct InternalReport {
        pub(super) path_exists: bool,
        pub(super) physical_consequences: bool,
        pub(super) one_direction: bool,
        pub(super) equal_magnitude: bool,
        pub(super) active_only: bool,
        pub(super) abstention_controls: bool,
        pub(super) swapped_direction: bool,
        pub(super) learned_allocation: bool,
        pub(super) shuffled_control: bool,
        pub(super) fresh_transfer: bool,
        pub(super) first_updates: usize,
        pub(super) second_updates: usize,
        pub(super) first_admissions: usize,
        pub(super) second_admissions: usize,
        pub(super) shuffled_first_admissions: usize,
        pub(super) shuffled_second_admissions: usize,
        pub(super) swapped_first_admissions: usize,
        pub(super) swapped_second_admissions: usize,
        pub(super) consequence_spikes: u64,
        pub(super) consequence_routes: u64,
    }

    fn encounter_first(seed: u64, ordinal: u64, reverse_layout: bool) -> PhysicalEncounter {
        let base = seed + 10_000 + ordinal * 2;
        if reverse_layout {
            pattern_p(base, base + 1, 91, 90)
        } else {
            pattern_p(base, base + 1, 40, 41)
        }
    }

    fn encounter_second(seed: u64, ordinal: u64, reverse_layout: bool) -> PhysicalEncounter {
        let base = seed + 20_000 + ordinal * 2;
        if reverse_layout {
            pattern_n(base, base + 1, 101, 100)
        } else {
            pattern_n(base, base + 1, 50, 51)
        }
    }

    fn episode(
        path: &mut PlasticityPath,
        learner: &mut ConsequenceLearner,
        encounter: PhysicalEncounter,
        other: EncounterSnapshot,
        raw: RawConsequence,
    ) -> (bool, bool) {
        let active = encounter.snapshot();
        let Some(edge) = path.encounter(encounter) else {
            return (false, false);
        };
        if !path.execute(edge) {
            return (false, false);
        }
        let updated = learner.apply(path, active, other, raw);
        (true, updated)
    }

    fn train(seed: u64, swapped: bool) -> Trained {
        let mut path = PlasticityPath::default();
        let mut learner = ConsequenceLearner::default();
        let first = encounter_first(seed, 0, false).snapshot();
        let second = encounter_second(seed, 0, false).snapshot();
        let mut first_updates = 0;
        let mut second_updates = 0;
        let mut all_magnitudes_equal = true;
        let mut first_blank_executed = false;
        let mut second_blank_executed = false;
        let mut executions = 0;
        for round in 0..12usize {
            let first_variant = if swapped { round % 4 } else { 0 };
            let second_variant = if swapped { 0 } else { round % 4 };
            let first_raw = raw_consequence(seed + 100_000, round * 2, first_variant, false);
            let second_raw = raw_consequence(seed + 100_000, round * 2 + 1, second_variant, false);
            let mut work = ConsequenceWork::default();
            let first_shape = execute_and_normalize(first_raw, &mut work);
            let second_shape = execute_and_normalize(second_raw, &mut work);
            all_magnitudes_equal &= first_shape.as_ref().map(ConsequenceShape::magnitude)
                == Some(3)
                && second_shape.as_ref().map(ConsequenceShape::magnitude) == Some(3);
            let (first_executed, first_updated) = episode(
                &mut path,
                &mut learner,
                encounter_first(seed, round as u64, false),
                second,
                first_raw,
            );
            let (second_executed, second_updated) = episode(
                &mut path,
                &mut learner,
                encounter_second(seed, round as u64, false),
                first,
                second_raw,
            );
            if round == 0 {
                first_blank_executed = first_executed;
                second_blank_executed = second_executed;
            }
            executions += usize::from(first_executed) + usize::from(second_executed);
            first_updates += usize::from(first_updated);
            second_updates += usize::from(second_updated);
        }
        Trained {
            path,
            learner,
            first,
            second,
            first_updates,
            second_updates,
            all_magnitudes_equal,
            first_blank_executed,
            second_blank_executed,
            executions,
        }
    }

    fn admissions(path: &PlasticityPath, seed: u64, reverse_layout: bool) -> (usize, usize) {
        let mut path = path.clone();
        let mut first = 0;
        let mut second = 0;
        for ordinal in 100..108u64 {
            first += usize::from(
                path.encounter(encounter_first(seed, ordinal, reverse_layout))
                    .is_some(),
            );
            second += usize::from(
                path.encounter(encounter_second(seed, ordinal, reverse_layout))
                    .is_some(),
            );
        }
        (first, second)
    }

    fn equal_and_shuffled_abstain(
        seed: u64,
        first: EncounterSnapshot,
        second: EncounterSnapshot,
    ) -> bool {
        let mut equal = ConsequenceLearner::default();
        let mut shuffled = ConsequenceLearner::default();
        for round in 0..8usize {
            let _ = equal.observe(first, raw_consequence(seed + 300_000, round * 2, 0, false));
            let _ = equal.observe(
                second,
                raw_consequence(seed + 300_000, round * 2 + 1, 1, false),
            );
            let _ = shuffled.observe(
                first,
                raw_consequence(seed + 400_000, round * 2, round % 4, false),
            );
            let _ = shuffled.observe(
                second,
                raw_consequence(seed + 400_000, round * 2 + 1, (round + 2) % 4, false),
            );
        }
        equal.direction([first, second]).is_none() && shuffled.direction([first, second]).is_none()
    }

    fn active_only_control(trained: &Trained, seed: u64) -> bool {
        let mut path = trained.path.clone();
        let mut learner = trained.learner.clone();
        let first_before = path
            .encoder
            .recognized(trained.first)
            .and_then(|id| path.values.get(&id))
            .map_or(0, |record| record.score());
        let second_before = path
            .encoder
            .recognized(trained.second)
            .and_then(|id| path.values.get(&id))
            .map_or(0, |record| record.score());
        let (_, updated) = episode(
            &mut path,
            &mut learner,
            encounter_first(seed, 500, true),
            trained.second,
            raw_consequence(seed + 500_000, 0, 0, false),
        );
        let first_after = path
            .encoder
            .recognized(trained.first)
            .and_then(|id| path.values.get(&id))
            .map_or(0, |record| record.score());
        let second_after = path
            .encoder
            .recognized(trained.second)
            .and_then(|id| path.values.get(&id))
            .map_or(0, |record| record.score());
        updated && first_after == first_before + 1 && second_after == second_before
    }

    fn boundary_abstentions(trained: &Trained, seed: u64) -> bool {
        let mut absent_path = trained.path.clone();
        absent_path.eligibility = None;
        let mut absent_learner = trained.learner.clone();
        let absent_execution = !absent_learner.apply(
            &mut absent_path,
            trained.first,
            trained.second,
            raw_consequence(seed + 600_000, 0, 0, false),
        );

        let mut immediate_path = trained.path.clone();
        let mut immediate_learner = trained.learner.clone();
        let immediate_edge = immediate_path
            .encounter(encounter_first(seed, 600, false))
            .expect("learned first encounter remains admitted");
        let immediate_execution = immediate_path.execute(immediate_edge);
        let immediate_activity = !immediate_learner.apply(
            &mut immediate_path,
            trained.first,
            trained.second,
            raw_consequence(seed + 600_000, 1, 0, true),
        );

        let mut removed_path = trained.path.clone();
        let mut removed_learner = trained.learner.clone();
        let withheld = removed_path
            .encounter(encounter_first(seed, 700, false))
            .expect("learned first encounter remains admitted");
        let removed_executed = removed_path.execute(withheld);
        for ordinal in 701..725u64 {
            let _ = removed_path.encounter(encounter_first(seed, ordinal, false));
            if !removed_path.proposals.contains_key(&withheld) {
                break;
            }
        }
        let physically_removed = !removed_path.proposals.contains_key(&withheld);
        let removed_eligibility = !removed_learner.apply(
            &mut removed_path,
            trained.first,
            trained.second,
            raw_consequence(seed + 600_000, 2, 0, false),
        );
        absent_execution
            && immediate_execution
            && immediate_activity
            && removed_executed
            && physically_removed
            && removed_eligibility
            && equal_and_shuffled_abstain(seed, trained.first, trained.second)
    }

    pub(super) fn run_internal(seed: u64) -> InternalReport {
        let trained = train(seed, false);
        let direction = trained
            .learner
            .clone()
            .direction([trained.first, trained.second]);
        let (first_admissions, second_admissions) = admissions(&trained.path, seed, false);
        let mut shuffled_path = trained.path.clone();
        let swap_complete = shuffled_path.swap_values(trained.first, trained.second);
        let (shuffled_first_admissions, shuffled_second_admissions) =
            admissions(&shuffled_path, seed + 1_000_000, false);
        let fresh = admissions(&trained.path, seed + 2_000_000, true);

        let swapped = train(seed + 3_000_000, true);
        let swapped_direction = swapped
            .learner
            .clone()
            .direction([swapped.first, swapped.second]);
        let (swapped_first_admissions, swapped_second_admissions) =
            admissions(&swapped.path, seed + 3_000_000, true);

        InternalReport {
            path_exists: trained.first_blank_executed && trained.second_blank_executed,
            physical_consequences: trained.executions < 24
                && trained.learner.work.spikes == trained.executions as u64 * 3
                && trained.learner.work.routes == trained.executions as u64 * 2
                && trained.learner.work.observations == trained.executions as u64,
            one_direction: direction == Some(trained.first),
            equal_magnitude: trained.all_magnitudes_equal,
            active_only: active_only_control(&trained, seed),
            abstention_controls: boundary_abstentions(&trained, seed),
            swapped_direction: swapped_direction == Some(swapped.second)
                && swapped.first_updates > 0
                && swapped.second_updates > 0,
            learned_allocation: first_admissions == 8 && second_admissions <= 1,
            shuffled_control: swap_complete
                && shuffled_first_admissions <= 1
                && shuffled_second_admissions == 8,
            fresh_transfer: fresh.0 == 8 && fresh.1 <= 1,
            first_updates: trained.first_updates,
            second_updates: trained.second_updates,
            first_admissions,
            second_admissions,
            shuffled_first_admissions,
            shuffled_second_admissions,
            swapped_first_admissions,
            swapped_second_admissions,
            consequence_spikes: trained.learner.work.spikes,
            consequence_routes: trained.learner.work.routes,
        }
    }
}

pub fn audit() -> AuditReport {
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/ds8_cumulative_semantic_credit_probe.rs"
    ));
    let path = source
        .split("// DS8_ORGANISM_PATH_BEGIN")
        .nth(1)
        .and_then(|text| text.split("// DS8_ORGANISM_PATH_END").next())
        .unwrap_or_default();
    let forbidden = [
        ["cor", "rect"].concat(),
        ["wr", "ong"].concat(),
        ["rew", "ard"].concat(),
        ["lo", "ss"].concat(),
        ["expected", "_answer"].concat(),
        ["target", "_answer"].concat(),
        ["semantic", "_polarity"].concat(),
        ["selected", "_route"].concat(),
    ];
    let mut report = AuditReport {
        activation: env!("DS8_ACTIVATION_SHA256") == FROZEN_ACTIVATION_SHA256,
        dependency: env!("DS8_AUDIT_SHA256") == FROZEN_AUDIT_SHA256,
        protocol_v1: env!("DS8_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        protocol_v2: env!("DS8_PROTOCOL_V2_SHA256") == FROZEN_PROTOCOL_V2_SHA256,
        allocator: env!("DS8_M5_ALLOCATOR_SHA256") == FROZEN_M5_ALLOCATOR_SHA256,
        gate: env!("DS8_M5_GATE_SHA256") == FROZEN_M5_GATE_SHA256,
        csv: env!("DS8_M5_CSV_SHA256") == FROZEN_M5_CSV_SHA256,
        md: env!("DS8_M5_MD_SHA256") == FROZEN_M5_MD_SHA256,
        d3: env!("DS8_D3_SHA256") == FROZEN_D3_SHA256,
        d2: env!("DS8_D2_SHA256") == FROZEN_D2_SHA256,
        c0: env!("DS8_C0_SHA256") == FROZEN_C0_SHA256,
        cp0: env!("DS8_CP0_SHA256") == FROZEN_CP0_SHA256,
        forbidden: forbidden
            .iter()
            .map(|token| path.matches(token).count())
            .sum(),
        linkers: path
            .matches("path.delayed_experience(differential)")
            .count(),
        normalizers: path.matches("fn execute_and_normalize(").count(),
        occurrence_identity: path.matches("occurrences").count(),
        passed: false,
    };
    report.passed = report.activation
        && report.dependency
        && report.protocol_v1
        && report.protocol_v2
        && report.allocator
        && report.gate
        && report.csv
        && report.md
        && report.d3
        && report.d2
        && report.c0
        && report.cp0
        && report.forbidden == 0
        && report.linkers == 1
        && report.normalizers == 1
        && report.occurrence_identity == 0;
    report
}

fn source_audit() -> SourceAudit {
    let report = audit();
    SourceAudit {
        frozen_inputs: report.activation
            && report.dependency
            && report.protocol_v1
            && report.protocol_v2
            && report.allocator
            && report.gate
            && report.csv
            && report.md
            && report.d3
            && report.d2
            && report.c0
            && report.cp0,
        no_semantic_channels: report.forbidden == 0,
        physical_link_exact: report.linkers == 1
            && report.normalizers == 1
            && report.occurrence_identity == 0,
    }
}

fn check(name: &'static str, passed: bool) -> Check {
    Check { name, passed }
}

fn run_once() -> ProbeReport {
    let internal = frozen_m5::run_internal(PROBE_SEED);
    let source = source_audit();
    let m5 = crate::ds7_cumulative_plasticity_allocation_gate::run();
    let m5_exact = m5.passed
        && m5.cells.len() == 18
        && m5
            .cells
            .iter()
            .all(|cell| cell.cumulative_m4 && cell.passed);
    let checks = vec![
        check("variation before consequence", internal.path_exists),
        check(
            "physical delayed consequences",
            internal.physical_consequences,
        ),
        check("one differential contrast", internal.one_direction),
        check("equal consequence magnitude", internal.equal_magnitude),
        check("active eligibility only", internal.active_only),
        check("abstention controls", internal.abstention_controls),
        check("swapped histories reverse", internal.swapped_direction),
        check("learned allocation", internal.learned_allocation),
        check("shuffled M5 value control", internal.shuffled_control),
        check("fresh identity and layout", internal.fresh_transfer),
        check("source and information-flow audit", source.passed()),
        check("unchanged authoritative M5", m5_exact),
    ];
    let first_collapse = checks
        .iter()
        .find(|item| !item.passed)
        .map_or("none", |item| item.name);
    let passed = checks.iter().all(|item| item.passed);
    ProbeReport {
        protocol: PROTOCOL,
        seed: PROBE_SEED,
        checks,
        first_updates: internal.first_updates,
        second_updates: internal.second_updates,
        first_admissions: internal.first_admissions,
        second_admissions: internal.second_admissions,
        shuffled_first_admissions: internal.shuffled_first_admissions,
        shuffled_second_admissions: internal.shuffled_second_admissions,
        swapped_first_admissions: internal.swapped_first_admissions,
        swapped_second_admissions: internal.swapped_second_admissions,
        consequence_spikes: internal.consequence_spikes,
        consequence_routes: internal.consequence_routes,
        first_collapse,
        duplicate_exact: false,
        passed,
    }
}

pub fn run() -> ProbeReport {
    let mut first = run_once();
    let second = run_once();
    first.duplicate_exact = {
        let mut expected = first.clone();
        expected.duplicate_exact = false;
        expected == second
    };
    first
        .checks
        .push(check("duplicate exact", first.duplicate_exact));
    if !first.duplicate_exact {
        first.first_collapse = "duplicate exact";
        first.passed = false;
    }
    first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_is_duplicate_exact_and_conjunctive() {
        let report = run();
        assert!(report.duplicate_exact);
        assert!(report.checks.iter().all(|item| item.passed), "{report:#?}");
        assert!(report.passed, "first collapse: {}", report.first_collapse);
    }
}
