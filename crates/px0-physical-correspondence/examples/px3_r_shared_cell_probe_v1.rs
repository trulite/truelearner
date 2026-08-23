use px0_physical_correspondence::{
    ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const FROZEN_START: &str = "873094497ff6eb74363191dc5edc479c7d66de72";
const FROZEN_PARENT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const PX0_PX2_LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX2_SOURCE_SHA256: &str = "c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5";
const FROZEN_PX3_SOURCE_SHA256: &str =
    "39ec595fc1204a29083d271ebcadcdb7950c07d1c44e4ce07c0107fca54730ba";
const FROZEN_PX3_CSV_SHA256: &str =
    "685dc04db32a5785224c62ba5b589fa8e1e37382a8b613f5f2b5e396aa005f38";
const FROZEN_PX3_MD_SHA256: &str =
    "021be8698c010df1e09dc45f2bf9968f2255b6eb7851c38f36fd93be72260d3b";
const FROZEN_HANDOFF_SHA256: &str =
    "a029f250ed88f8f2fc164e0d2c9042675bf0a8c9ae51c89cf83ad1aa42e4fa9b";
const PROTOCOL_SHA256: &str = "7ac42a90fca11d42175acba597a4ac6fed6df47c94b49a5ac8e726ea95d12204";

const PROTOCOL: &str = "experiments/px3_r_shared_cell_probe_v1_protocol.md";
const RESULT_CSV: &str = "results/px3_r_shared_cell_probe_v1.csv";
const RESULT_MD: &str = "results/px3_r_shared_cell_probe_v1.md";
const STAGING_CSV: &str = "results/.px3_r_shared_cell_probe_v1.csv.staging";
const STAGING_MD: &str = "results/.px3_r_shared_cell_probe_v1.md.staging";

const NAMESPACE_MAIN: u64 = 0x9_B100_0000;
const NAMESPACE_REPLICA: u64 = 0x9_B200_0000;
const NAMESPACE_SPACING: u64 = 0x9_B300_0000;
const NAMESPACE_ALTERNATIVE: u64 = 0x9_B400_0000;
const NAMESPACE_CORRELATED: u64 = 0x9_B500_0000;
const NAMESPACE_BLOCKED: u64 = 0x9_B600_0000;
const NAMESPACE_ABSENT: u64 = 0x9_B700_0000;
const NAMESPACE_STALE: u64 = 0x9_B800_0000;
const NAMESPACE_AMBIGUOUS: u64 = 0x9_B900_0000;
const NAMESPACE_MULTIPLE: u64 = 0x9_BA00_0000;

const ROUTES: usize = 4;
const SITES: usize = 6;
const PORTS: usize = 12;
const TRAIN_ROUNDS: usize = 16;
const SWAP_ROUNDS: usize = 20;
const TRAIN_START: i64 = 2;
const ROUND_SPACING: i64 = 10;
const BETWEEN_GAP: i64 = 4;
const OBSERVATION_GAP: i64 = 10;
const SITE_ROUTES: [[usize; 2]; SITES] = [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]];

// PX3_R_SHARED_CELL_ORGANISM_VISIBLE_BEGIN
mod physics {
    use super::{
        ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger, PORTS,
        SITES,
    };

    #[derive(Clone)]
    pub(super) struct Matter {
        substrate: PlasticSubstrate,
        namespace: u64,
        sources: [CellId; PORTS],
        source_physical: [u64; PORTS],
        locals: [CellId; SITES],
        drivers: [CellId; PORTS],
        noise: CellId,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub(super) struct Counts {
        pub source_firings: [usize; PORTS],
        pub local_firings: [usize; SITES],
        pub outward: [usize; SITES],
        pub noise_firings: usize,
        pub quiescent: bool,
        pub work: WorkLedger,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct State {
        pub incoming_resistance: [u32; PORTS],
        pub incoming_live: [bool; PORTS],
        pub permanent_fingerprint: u64,
        pub complete_fingerprint: u64,
        pub arrow_count: usize,
        pub persistent_bytes: usize,
    }

    pub(super) fn fresh(
        namespace: u64,
        reverse_allocation: bool,
        mirror: bool,
        reverse_ids: bool,
        return_live: bool,
        weak_present: bool,
    ) -> Matter {
        let mut substrate = PlasticSubstrate::new();
        let mut sources = [None; PORTS];
        let mut source_physical = [0; PORTS];
        let mut locals = [None; SITES];
        let mut outsides = [None; SITES];
        let mut drivers = [None; PORTS];
        let site_order = if reverse_allocation {
            [5usize, 4, 3, 2, 1, 0]
        } else {
            [0usize, 1, 2, 3, 4, 5]
        };

        for site in site_order {
            let center = if mirror {
                -(site as i32) * 100
            } else {
                site as i32 * 100
            };
            let sides = if reverse_allocation { [1, 0] } else { [0, 1] };
            for side in sides {
                let port = site * 2 + side;
                let id_side = if reverse_ids { 1 - side } else { side };
                let offset = if mirror {
                    if side == 0 {
                        2
                    } else {
                        -2
                    }
                } else if side == 0 {
                    -2
                } else {
                    2
                };
                source_physical[port] = namespace + 100 + (site * 2 + id_side) as u64;
                sources[port] = Some(substrate.add_cell(cell(
                    source_physical[port],
                    center + offset,
                    0,
                    2,
                    1_000,
                )));
            }
            locals[site] =
                Some(substrate.add_cell(cell(namespace + 200 + site as u64, center, 0, 2, 1_000)));
            outsides[site] = Some(substrate.add_cell(cell(
                namespace + 300 + site as u64,
                center + 10_000,
                1,
                1,
                1_000,
            )));
        }
        for port in if reverse_allocation {
            (0..PORTS).rev().collect::<Vec<_>>()
        } else {
            (0..PORTS).collect::<Vec<_>>()
        } {
            drivers[port] = Some(substrate.add_cell(cell(
                namespace + 400 + port as u64,
                20_000 + port as i32 * 10,
                0,
                1,
                1_000,
            )));
        }
        let noise = substrate.add_cell(cell(namespace + 500, 30_000, 0, 1, 1_000));
        let sources = sources.map(|value| value.expect("physical source"));
        let locals = locals.map(|value| value.expect("local cell"));
        let outsides = outsides.map(|value| value.expect("outside cell"));
        let drivers = drivers.map(|value| value.expect("driver cell"));

        for site in 0..SITES {
            for side in 0..2 {
                let port = site * 2 + side;
                substrate.add_arrow(arrow(
                    locals[site],
                    sources[port],
                    1,
                    1,
                    if return_live { 1_000 } else { 0 },
                ));
                substrate.add_arrow(arrow(drivers[port], sources[port], 0, 2, 1_000));
                if weak_present {
                    substrate.add_arrow(arrow(sources[port], locals[site], 1, 1, 1));
                }
            }
            substrate.add_arrow(arrow(locals[site], outsides[site], 0, 1, 1_000));
        }

        Matter {
            substrate,
            namespace,
            sources,
            source_physical,
            locals,
            drivers,
            noise,
        }
    }

    pub(super) fn enter_development(
        matter: &mut Matter,
        tick: i64,
        port: usize,
        origin: u64,
        reverse: bool,
    ) {
        let phases = if reverse { [1, 0] } else { [0, 1] };
        for (serial, phase) in phases.into_iter().enumerate() {
            matter.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase,
                origin_physical: origin + serial as u64,
                target: matter.sources[port],
                impulse: 1,
            });
        }
    }

    pub(super) fn enter_use(matter: &mut Matter, tick: i64, port: usize, origin: u64) {
        matter.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: origin,
            target: matter.drivers[port],
            impulse: 1,
        });
    }

    pub(super) fn enter_noise(matter: &mut Matter, tick: i64, origin: u64) {
        matter.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: origin,
            target: matter.noise,
            impulse: 1,
        });
    }

    pub(super) fn propagate(matter: &mut Matter) -> Counts {
        let run = matter.substrate.propagate();
        counts(matter, &run)
    }

    pub(super) fn advance(matter: &mut Matter, tick: i64) -> WorkLedger {
        matter.substrate.advance_time(tick)
    }

    pub(super) fn state(matter: &Matter) -> State {
        let incoming_resistance = std::array::from_fn(|port| {
            let site = port / 2;
            matter
                .substrate
                .arrows_between(matter.sources[port], matter.locals[site])
                .into_iter()
                .filter(|arrow| matter.substrate.arrow_is_live(*arrow))
                .map(|arrow| matter.substrate.arrow_resistance(arrow))
                .max()
                .unwrap_or(0)
        });
        let incoming_live = std::array::from_fn(|port| incoming_resistance[port] > 0);
        State {
            incoming_resistance,
            incoming_live,
            permanent_fingerprint: matter.substrate.permanent_fingerprint(),
            complete_fingerprint: matter.substrate.complete_fingerprint(),
            arrow_count: matter.substrate.arrow_count(),
            persistent_bytes: matter.substrate.persistent_bytes(),
        }
    }

    fn counts(matter: &Matter, run: &Execution) -> Counts {
        Counts {
            source_firings: std::array::from_fn(|port| {
                firings_at(run, matter.source_physical[port])
            }),
            local_firings: std::array::from_fn(|site| {
                firings_at(run, matter.namespace + 200 + site as u64)
            }),
            outward: std::array::from_fn(|site| {
                run.crossings
                    .iter()
                    .filter(|crossing| {
                        crossing.from_physical == matter.namespace + 200 + site as u64
                            && crossing.to_physical == matter.namespace + 300 + site as u64
                    })
                    .count()
            }),
            noise_firings: firings_at(run, matter.namespace + 500),
            quiescent: run.naturally_quiescent,
            work: run.work.clone(),
        }
    }

    fn firings_at(run: &Execution, physical: u64) -> usize {
        run.trace
            .iter()
            .filter(|entry| entry.target_physical == physical && entry.fired)
            .count()
    }

    fn cell(
        physical_id: u64,
        position: i32,
        region: i16,
        threshold: i32,
        resistance: u32,
    ) -> CellSpec {
        CellSpec {
            physical_id,
            position,
            region,
            threshold,
            resistance,
        }
    }

    fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
        ArrowSpec {
            from,
            to,
            delay,
            phase: 0,
            coupling,
            resistance,
        }
    }
}
// PX3_R_SHARED_CELL_ORGANISM_VISIBLE_END

#[derive(Clone, Debug, PartialEq, Eq)]
struct Snapshot {
    state: physics::State,
    route_resistance: [[u32; 3]; ROUTES],
    route_live: [[bool; 3]; ROUTES],
    uses: [physics::Counts; SITES],
    work: WorkLedger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Scenario {
    name: &'static str,
    namespace: u64,
    training: physics::Counts,
    initial: Snapshot,
    swap_training: Option<physics::Counts>,
    swapped: Option<Snapshot>,
    duplicate_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Report {
    cases: Vec<Scenario>,
    frozen_exact: bool,
    matched_marginals: bool,
    trained_crossed: bool,
    swap_relearning: bool,
    hard_controls: bool,
    forbidden_clean: bool,
    positive: bool,
    classification: &'static str,
    total_work: u64,
}

#[derive(Clone, Copy)]
struct Fixture {
    namespace: u64,
    reverse_allocation: bool,
    mirror: bool,
    reverse_ids: bool,
    return_live: bool,
    weak_present: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let preflight = args == ["--preflight"];
    let probe = args == ["--probe"];
    if !preflight && !probe {
        eprintln!("PX3-R shared-CELL PROBE requires --preflight or --probe");
        std::process::exit(2);
    }
    assert!(
        source_audit(),
        "frozen source or forbidden-information audit failed"
    );
    for path in [RESULT_CSV, RESULT_MD, STAGING_CSV, STAGING_MD] {
        assert!(!Path::new(path).exists(), "PROBE artifact exists: {path}");
    }
    if preflight {
        println!("PX3_R_SHARED_CELL_PROBE_V1_PREFLIGHT_OK");
        return;
    }

    eprintln!("PX3_R_SHARED_CELL_PROBE_V1_EVIDENCE_SPENT");
    let report = run_probe();
    write_atomic(&report);
    println!("PX3_R_SHARED_CELL_PROBE_V1_{}", report.classification);
    if report.positive {
        std::process::exit(0);
    }
    std::process::exit(1);
}

fn run_probe() -> Report {
    let main_fixture = Fixture {
        namespace: NAMESPACE_MAIN,
        reverse_allocation: false,
        mirror: false,
        reverse_ids: false,
        return_live: true,
        weak_present: true,
    };
    let main_partition = [[0, 1], [2, 3]];
    let swapped_partition = [[0, 3], [2, 1]];
    let main = run_partition_case(
        "main-with-swap",
        main_fixture,
        main_partition,
        TRAIN_ROUNDS,
        ROUND_SPACING,
        BETWEEN_GAP,
        Some((swapped_partition, SWAP_ROUNDS)),
    );
    let main_duplicate = run_partition_case(
        "main-with-swap",
        main_fixture,
        main_partition,
        TRAIN_ROUNDS,
        ROUND_SPACING,
        BETWEEN_GAP,
        Some((swapped_partition, SWAP_ROUNDS)),
    );

    let replica_fixture = Fixture {
        namespace: NAMESPACE_REPLICA,
        reverse_allocation: true,
        mirror: true,
        reverse_ids: true,
        return_live: true,
        weak_present: true,
    };
    let replica = run_partition_case(
        "layout-order-identity-replica",
        replica_fixture,
        main_partition,
        TRAIN_ROUNDS,
        ROUND_SPACING,
        BETWEEN_GAP,
        None,
    );
    let replica_duplicate = run_partition_case(
        "layout-order-identity-replica",
        replica_fixture,
        main_partition,
        TRAIN_ROUNDS,
        ROUND_SPACING,
        BETWEEN_GAP,
        None,
    );

    let spacing_fixture = Fixture {
        namespace: NAMESPACE_SPACING,
        reverse_allocation: false,
        mirror: false,
        reverse_ids: true,
        return_live: true,
        weak_present: true,
    };
    let spacing = run_partition_case(
        "spacing-replica",
        spacing_fixture,
        main_partition,
        TRAIN_ROUNDS,
        14,
        6,
        None,
    );
    let spacing_duplicate = run_partition_case(
        "spacing-replica",
        spacing_fixture,
        main_partition,
        TRAIN_ROUNDS,
        14,
        6,
        None,
    );

    let alternative_fixture = Fixture {
        namespace: NAMESPACE_ALTERNATIVE,
        reverse_allocation: true,
        mirror: false,
        reverse_ids: false,
        return_live: true,
        weak_present: true,
    };
    let alternative = run_partition_case(
        "stable-alternative",
        alternative_fixture,
        swapped_partition,
        TRAIN_ROUNDS,
        ROUND_SPACING,
        BETWEEN_GAP,
        None,
    );
    let alternative_duplicate = run_partition_case(
        "stable-alternative",
        alternative_fixture,
        swapped_partition,
        TRAIN_ROUNDS,
        ROUND_SPACING,
        BETWEEN_GAP,
        None,
    );

    let correlated = run_correlated_case();
    let correlated_duplicate = run_correlated_case();
    let blocked = run_partition_case(
        "participation-blocked-return",
        Fixture {
            namespace: NAMESPACE_BLOCKED,
            reverse_allocation: false,
            mirror: false,
            reverse_ids: false,
            return_live: false,
            weak_present: true,
        },
        main_partition,
        TRAIN_ROUNDS,
        ROUND_SPACING,
        BETWEEN_GAP,
        None,
    );
    let blocked_duplicate = run_partition_case(
        "participation-blocked-return",
        Fixture {
            namespace: NAMESPACE_BLOCKED,
            reverse_allocation: false,
            mirror: false,
            reverse_ids: false,
            return_live: false,
            weak_present: true,
        },
        main_partition,
        TRAIN_ROUNDS,
        ROUND_SPACING,
        BETWEEN_GAP,
        None,
    );
    let absent = run_absent_case();
    let absent_duplicate = run_absent_case();
    let stale = run_stale_case();
    let stale_duplicate = run_stale_case();
    let ambiguous = run_simultaneous_case("ambiguous-three", NAMESPACE_AMBIGUOUS, 3);
    let ambiguous_duplicate = run_simultaneous_case("ambiguous-three", NAMESPACE_AMBIGUOUS, 3);
    let multiple = run_simultaneous_case("multiple-four", NAMESPACE_MULTIPLE, 4);
    let multiple_duplicate = run_simultaneous_case("multiple-four", NAMESPACE_MULTIPLE, 4);

    let mut cases = vec![
        mark_duplicate(main, &main_duplicate),
        mark_duplicate(replica, &replica_duplicate),
        mark_duplicate(spacing, &spacing_duplicate),
        mark_duplicate(alternative, &alternative_duplicate),
        mark_duplicate(correlated, &correlated_duplicate),
        mark_duplicate(blocked, &blocked_duplicate),
        mark_duplicate(absent, &absent_duplicate),
        mark_duplicate(stale, &stale_duplicate),
        mark_duplicate(ambiguous, &ambiguous_duplicate),
        mark_duplicate(multiple, &multiple_duplicate),
    ];

    let main = &cases[0];
    let replica = &cases[1];
    let spacing = &cases[2];
    let alternative = &cases[3];
    let correlated = &cases[4];
    let blocked = &cases[5];
    let absent = &cases[6];
    let stale = &cases[7];
    let ambiguous = &cases[8];
    let multiple = &cases[9];

    let matched_marginals = route_strength_equal(&main.initial)
        && route_strength_equal(main.swapped.as_ref().expect("swap snapshot"))
        && exact_source_counts(&main.training, TRAIN_ROUNDS)
        && exact_source_counts(
            main.swap_training.as_ref().expect("swap training"),
            SWAP_ROUNDS,
        )
        && main.training.noise_firings == TRAIN_ROUNDS * 2
        && main
            .swap_training
            .as_ref()
            .expect("swap training")
            .noise_firings
            == SWAP_ROUNDS * 2;
    let trained_crossed =
        expected_structure(&main.initial, &[0, 5]) && expected_use(&main.initial, &[0, 5]);
    let swap_relearning = main.swapped.as_ref().is_some_and(|swapped| {
        expected_structure(swapped, &[2, 3])
            && expected_use(swapped, &[2, 3])
            && sites_weaker(&main.initial, swapped, &[0, 5])
    });
    let positive_replicas = expected_structure(&replica.initial, &[0, 5])
        && expected_use(&replica.initial, &[0, 5])
        && route_strength_equal(&replica.initial)
        && expected_structure(&spacing.initial, &[0, 5])
        && expected_use(&spacing.initial, &[0, 5])
        && route_strength_equal(&spacing.initial)
        && expected_structure(&alternative.initial, &[2, 3])
        && expected_use(&alternative.initial, &[2, 3])
        && route_strength_equal(&alternative.initial);
    let negative_controls = no_structure(&correlated.initial)
        && no_use(&correlated.initial)
        && blocked.training.local_firings.iter().sum::<usize>() == TRAIN_ROUNDS * 2
        && no_structure(&blocked.initial)
        && no_use(&blocked.initial)
        && no_structure(&absent.initial)
        && no_use(&absent.initial)
        && no_structure(&stale.initial)
        && no_use(&stale.initial);
    let ambiguity_controls = expected_structure(&ambiguous.initial, &[0, 1, 3])
        && expected_use(&ambiguous.initial, &[0, 1, 3])
        && expected_structure(&multiple.initial, &[0, 1, 2, 3, 4, 5])
        && expected_use(&multiple.initial, &[0, 1, 2, 3, 4, 5]);
    let replay_quiet = cases.iter().all(|case| {
        case.duplicate_exact
            && case.training.quiescent
            && case.initial.uses.iter().all(|use_| use_.quiescent)
            && case
                .swap_training
                .as_ref()
                .is_none_or(|training| training.quiescent)
            && case
                .swapped
                .as_ref()
                .is_none_or(|snapshot| snapshot.uses.iter().all(|use_| use_.quiescent))
    });
    let source_exact = cases.iter().all(no_autonomous_source_refiring);
    let hard_controls = positive_replicas
        && negative_controls
        && ambiguity_controls
        && replay_quiet
        && source_exact;
    let frozen_exact = source_audit();
    let forbidden_clean = forbidden_audit();
    let positive = frozen_exact
        && matched_marginals
        && trained_crossed
        && swap_relearning
        && hard_controls
        && forbidden_clean;
    let mechanically_valid = frozen_exact
        && matched_marginals
        && negative_controls
        && replay_quiet
        && source_exact
        && forbidden_clean;
    let classification = if positive {
        "POSITIVE_CANDIDATE_SHARED_CELL_RECRUITMENT"
    } else if mechanically_valid {
        "FROZEN_NEGATIVE_SHARED_CELL_RECRUITMENT"
    } else {
        "FIRST_CLAUSE_FAILURE"
    };
    let total_work = cases.iter().map(scenario_work).sum();

    Report {
        cases: std::mem::take(&mut cases),
        frozen_exact,
        matched_marginals,
        trained_crossed,
        swap_relearning,
        hard_controls,
        forbidden_clean,
        positive,
        classification,
        total_work,
    }
}

fn run_partition_case(
    name: &'static str,
    fixture: Fixture,
    partition: [[usize; 2]; 2],
    rounds: usize,
    spacing: i64,
    gap: i64,
    swap: Option<([[usize; 2]; 2], usize)>,
) -> Scenario {
    let mut matter = make_matter(fixture);
    let (training, state_tick) = train_partition(
        &mut matter,
        fixture.namespace,
        TRAIN_START,
        partition,
        rounds,
        spacing,
        gap,
        true,
        true,
    );
    let initial = snapshot(&matter, state_tick);
    let (swap_training, swapped) = if let Some((next, next_rounds)) = swap {
        let start = state_tick + 2;
        let (training, next_tick) = train_partition(
            &mut matter,
            fixture.namespace + 0x10_0000,
            start,
            next,
            next_rounds,
            spacing,
            gap,
            true,
            true,
        );
        (Some(training), Some(snapshot(&matter, next_tick)))
    } else {
        (None, None)
    };
    Scenario {
        name,
        namespace: fixture.namespace,
        training,
        initial,
        swap_training,
        swapped,
        duplicate_exact: false,
    }
}

fn run_correlated_case() -> Scenario {
    let namespace = NAMESPACE_CORRELATED;
    let mut matter = make_matter(Fixture {
        namespace,
        reverse_allocation: false,
        mirror: false,
        reverse_ids: false,
        return_live: true,
        weak_present: true,
    });
    for round in 0..TRAIN_ROUNDS {
        let base = TRAIN_START + round as i64 * ROUND_SPACING;
        physics::enter_noise(&mut matter, base, namespace + round as u64 * 4);
        physics::enter_noise(
            &mut matter,
            base + BETWEEN_GAP,
            namespace + round as u64 * 4 + 1,
        );
    }
    let mut training = physics::propagate(&mut matter);
    let state_tick =
        TRAIN_START + (TRAIN_ROUNDS - 1) as i64 * ROUND_SPACING + BETWEEN_GAP + OBSERVATION_GAP;
    add_work(
        &mut training.work,
        &physics::advance(&mut matter, state_tick),
    );
    Scenario {
        name: "correlated-without-participation",
        namespace,
        training,
        initial: snapshot(&matter, state_tick),
        swap_training: None,
        swapped: None,
        duplicate_exact: false,
    }
}

fn run_absent_case() -> Scenario {
    let fixture = Fixture {
        namespace: NAMESPACE_ABSENT,
        reverse_allocation: false,
        mirror: false,
        reverse_ids: false,
        return_live: true,
        weak_present: false,
    };
    let mut matter = make_matter(fixture);
    let (training, state_tick) = train_partition(
        &mut matter,
        fixture.namespace,
        TRAIN_START,
        [[0, 1], [2, 3]],
        TRAIN_ROUNDS,
        ROUND_SPACING,
        BETWEEN_GAP,
        false,
        true,
    );
    Scenario {
        name: "absent-opportunity",
        namespace: fixture.namespace,
        training,
        initial: snapshot(&matter, state_tick),
        swap_training: None,
        swapped: None,
        duplicate_exact: false,
    }
}

fn run_stale_case() -> Scenario {
    let namespace = NAMESPACE_STALE;
    let mut matter = make_matter(Fixture {
        namespace,
        reverse_allocation: false,
        mirror: false,
        reverse_ids: false,
        return_live: true,
        weak_present: true,
    });
    let work = physics::advance(&mut matter, 30);
    let training = physics::Counts {
        quiescent: true,
        work,
        ..physics::Counts::default()
    };
    Scenario {
        name: "stale-field",
        namespace,
        training,
        initial: snapshot(&matter, 30),
        swap_training: None,
        swapped: None,
        duplicate_exact: false,
    }
}

fn run_simultaneous_case(name: &'static str, namespace: u64, width: usize) -> Scenario {
    let mut matter = make_matter(Fixture {
        namespace,
        reverse_allocation: false,
        mirror: false,
        reverse_ids: false,
        return_live: true,
        weak_present: true,
    });
    for round in 0..TRAIN_ROUNDS {
        let base = TRAIN_START + round as i64 * ROUND_SPACING;
        let reverse = round % 2 == 1;
        let active = if reverse {
            (0..width).rev().collect::<Vec<_>>()
        } else {
            (0..width).collect::<Vec<_>>()
        };
        for route in active {
            enter_route(&mut matter, base, route, namespace, round, true, reverse);
        }
        if width == 3 {
            enter_route(
                &mut matter,
                base + BETWEEN_GAP,
                3,
                namespace,
                round,
                true,
                reverse,
            );
        }
        physics::enter_noise(&mut matter, base, namespace + 0x80_000 + round as u64);
    }
    let mut training = physics::propagate(&mut matter);
    let state_tick = TRAIN_START
        + (TRAIN_ROUNDS - 1) as i64 * ROUND_SPACING
        + if width == 3 { BETWEEN_GAP } else { 0 }
        + OBSERVATION_GAP;
    add_work(
        &mut training.work,
        &physics::advance(&mut matter, state_tick),
    );
    Scenario {
        name,
        namespace,
        training,
        initial: snapshot(&matter, state_tick),
        swap_training: None,
        swapped: None,
        duplicate_exact: false,
    }
}

fn make_matter(fixture: Fixture) -> physics::Matter {
    physics::fresh(
        fixture.namespace,
        fixture.reverse_allocation,
        fixture.mirror,
        fixture.reverse_ids,
        fixture.return_live,
        fixture.weak_present,
    )
}

#[allow(clippy::too_many_arguments)]
fn train_partition(
    matter: &mut physics::Matter,
    origin: u64,
    start: i64,
    partition: [[usize; 2]; 2],
    rounds: usize,
    spacing: i64,
    gap: i64,
    development: bool,
    distractors: bool,
) -> (physics::Counts, i64) {
    for round in 0..rounds {
        let base = start + round as i64 * spacing;
        let order = if round % 2 == 0 { [0, 1] } else { [1, 0] };
        for (slot, index) in order.into_iter().enumerate() {
            let tick = base + slot as i64 * gap;
            let routes = if round % 2 == 0 {
                partition[index]
            } else {
                [partition[index][1], partition[index][0]]
            };
            for route in routes {
                enter_route(
                    matter,
                    tick,
                    route,
                    origin,
                    round,
                    development,
                    round % 2 == 1,
                );
            }
            if distractors {
                physics::enter_noise(
                    matter,
                    tick,
                    origin + 0x80_000 + round as u64 * 2 + slot as u64,
                );
            }
        }
    }
    let mut counts = physics::propagate(matter);
    let state_tick = start + rounds.saturating_sub(1) as i64 * spacing + gap + OBSERVATION_GAP;
    add_work(&mut counts.work, &physics::advance(matter, state_tick));
    (counts, state_tick)
}

fn enter_route(
    matter: &mut physics::Matter,
    tick: i64,
    route: usize,
    origin: u64,
    round: usize,
    development: bool,
    reverse: bool,
) {
    let mut ports = route_ports(route);
    if reverse {
        ports.reverse();
    }
    for (ordinal, port) in ports.into_iter().enumerate() {
        let physical_origin =
            origin + 0x10_000 + round as u64 * 0x100 + route as u64 * 0x20 + ordinal as u64 * 2;
        if development {
            physics::enter_development(matter, tick, port, physical_origin, reverse);
        } else {
            physics::enter_use(matter, tick, port, physical_origin);
        }
    }
}

fn snapshot(matter: &physics::Matter, tick: i64) -> Snapshot {
    let state = physics::state(matter);
    let route_resistance = route_resistance(&state);
    let route_live = route_live(&state);
    let uses = std::array::from_fn(|site| {
        let mut copy = matter.clone();
        let routes = SITE_ROUTES[site];
        enter_route(
            &mut copy,
            tick + 2,
            routes[0],
            0xE000_0000 + site as u64 * 0x100,
            0,
            false,
            false,
        );
        enter_route(
            &mut copy,
            tick + 2,
            routes[1],
            0xE100_0000 + site as u64 * 0x100,
            0,
            false,
            true,
        );
        physics::propagate(&mut copy)
    });
    let work = uses.iter().fold(WorkLedger::default(), |mut total, use_| {
        add_work(&mut total, &use_.work);
        total
    });
    Snapshot {
        state,
        route_resistance,
        route_live,
        uses,
        work,
    }
}

fn route_ports(route: usize) -> [usize; 3] {
    let mut ports = [usize::MAX; 3];
    let mut next = 0;
    for (site, routes) in SITE_ROUTES.iter().enumerate() {
        if routes[0] == route {
            ports[next] = site * 2;
            next += 1;
        } else if routes[1] == route {
            ports[next] = site * 2 + 1;
            next += 1;
        }
    }
    assert_eq!(next, 3, "each route has three physical ports");
    ports
}

fn route_resistance(state: &physics::State) -> [[u32; 3]; ROUTES] {
    std::array::from_fn(|route| {
        let mut values = route_ports(route).map(|port| state.incoming_resistance[port]);
        values.sort_unstable();
        values
    })
}

fn route_live(state: &physics::State) -> [[bool; 3]; ROUTES] {
    std::array::from_fn(|route| {
        let mut values = route_ports(route).map(|port| state.incoming_live[port]);
        values.sort_unstable();
        values
    })
}

fn route_strength_equal(snapshot: &Snapshot) -> bool {
    snapshot
        .route_resistance
        .windows(2)
        .all(|window| window[0] == window[1])
        && snapshot
            .route_live
            .windows(2)
            .all(|window| window[0] == window[1])
}

fn expected_structure(snapshot: &Snapshot, expected: &[usize]) -> bool {
    (0..SITES).all(|site| {
        let present = snapshot.state.incoming_live[site * 2]
            && snapshot.state.incoming_live[site * 2 + 1]
            && snapshot.state.incoming_resistance[site * 2] > 1
            && snapshot.state.incoming_resistance[site * 2 + 1] > 1;
        present == expected.contains(&site)
    })
}

fn expected_use(snapshot: &Snapshot, expected: &[usize]) -> bool {
    (0..SITES).all(|site| {
        let fires = snapshot.uses[site].local_firings.iter().sum::<usize>();
        let outward = snapshot.uses[site].outward.iter().sum::<usize>();
        if expected.contains(&site) {
            fires == 1 && outward == 1
        } else {
            fires == 0 && outward == 0
        }
    })
}

fn sites_weaker(before: &Snapshot, after: &Snapshot, sites: &[usize]) -> bool {
    sites.iter().all(|site| {
        let ports = [site * 2, site * 2 + 1];
        ports.into_iter().all(|port| {
            after.state.incoming_resistance[port] < before.state.incoming_resistance[port]
        })
    })
}

fn no_structure(snapshot: &Snapshot) -> bool {
    snapshot.state.incoming_live == [false; PORTS]
}

fn no_use(snapshot: &Snapshot) -> bool {
    snapshot.uses.iter().all(|use_| {
        use_.local_firings.iter().sum::<usize>() == 0 && use_.outward.iter().sum::<usize>() == 0
    })
}

fn exact_source_counts(counts: &physics::Counts, rounds: usize) -> bool {
    counts.source_firings == [rounds; PORTS]
}

fn no_autonomous_source_refiring(case: &Scenario) -> bool {
    let base_ok = match case.name {
        "main-with-swap"
        | "layout-order-identity-replica"
        | "spacing-replica"
        | "stable-alternative"
        | "participation-blocked-return"
        | "absent-opportunity" => exact_source_counts(&case.training, TRAIN_ROUNDS),
        "ambiguous-three" | "multiple-four" => exact_source_counts(&case.training, TRAIN_ROUNDS),
        "correlated-without-participation" | "stale-field" => {
            case.training.source_firings == [0; PORTS]
        }
        _ => false,
    };
    let swap_ok = case
        .swap_training
        .as_ref()
        .is_none_or(|counts| exact_source_counts(counts, SWAP_ROUNDS));
    let uses_ok = case
        .initial
        .uses
        .iter()
        .chain(
            case.swapped
                .iter()
                .flat_map(|snapshot| snapshot.uses.iter()),
        )
        .all(|counts| counts.source_firings.iter().sum::<usize>() == 6);
    base_ok && swap_ok && uses_ok
}

fn mark_duplicate(mut original: Scenario, duplicate: &Scenario) -> Scenario {
    original.duplicate_exact = original == *duplicate;
    original
}

fn scenario_work(case: &Scenario) -> u64 {
    let mut ledger = case.training.work.clone();
    add_work(&mut ledger, &case.initial.work);
    if let Some(training) = &case.swap_training {
        add_work(&mut ledger, &training.work);
    }
    if let Some(snapshot) = &case.swapped {
        add_work(&mut ledger, &snapshot.work);
    }
    ledger.total()
}

fn add_work(total: &mut WorkLedger, additional: &WorkLedger) {
    total.queue_comparisons += additional.queue_comparisons;
    total.spikes_delivered += additional.spikes_delivered;
    total.generation_checks += additional.generation_checks;
    total.state_updates += additional.state_updates;
    total.threshold_checks += additional.threshold_checks;
    total.firings += additional.firings;
    total.arrow_checks += additional.arrow_checks;
    total.spikes_emitted += additional.spikes_emitted;
    total.local_eligibility_writes += additional.local_eligibility_writes;
    total.local_return_updates += additional.local_return_updates;
    total.ordinary_pressure_updates += additional.ordinary_pressure_updates;
    total.local_structural_proposals += additional.local_structural_proposals;
    total.physical_deallocations += additional.physical_deallocations;
}

fn source_audit() -> bool {
    let hashes = [
        (
            "crates/px0-physical-correspondence/src/lib.rs",
            PX0_PX2_LAW_SHA256,
        ),
        (
            "crates/px0-physical-correspondence/examples/px2_physical_causal_direction.rs",
            PX2_SOURCE_SHA256,
        ),
        (
            "crates/px0-physical-correspondence/examples/px3_physical_event_boundaries_probe_v3.rs",
            FROZEN_PX3_SOURCE_SHA256,
        ),
        (
            "results/px3_physical_event_boundaries_no_new_mechanism_probe_v3.csv",
            FROZEN_PX3_CSV_SHA256,
        ),
        (
            "results/px3_physical_event_boundaries_no_new_mechanism_probe_v3.md",
            FROZEN_PX3_MD_SHA256,
        ),
        (
            "experiments/px3_physical_event_boundaries_frozen_negative_handoff.md",
            FROZEN_HANDOFF_SHA256,
        ),
        (PROTOCOL, PROTOCOL_SHA256),
    ];
    let hashes_exact = hashes
        .into_iter()
        .all(|(path, expected)| sha256(path).as_deref() == Some(expected));
    let ancestry_exact = command_output(&[
        "rev-parse",
        "px3-physical-event-boundaries-frozen-negative-handoff-v1^{commit}",
    ])
    .as_deref()
        == Some(FROZEN_START)
        && command_output(&[
            "rev-parse",
            "px2-physical-causal-direction-authoritative^{commit}",
        ])
        .as_deref()
            == Some(FROZEN_PARENT)
        && Command::new("git")
            .args(["merge-base", "--is-ancestor", FROZEN_PARENT, "HEAD"])
            .status()
            .is_ok_and(|status| status.success());
    let changed_paths =
        command_output(&["diff", "--name-only", FROZEN_START, "HEAD"]).unwrap_or_default();
    let isolated = changed_paths.lines().all(|path| {
        path == PROTOCOL
            || path == "experiments/px3_r_shared_cell_probe_v1_implementation_audit.md"
            || path == "crates/px0-physical-correspondence/examples/px3_r_shared_cell_probe_v1.rs"
    });
    hashes_exact && ancestry_exact && isolated && forbidden_audit()
}

fn forbidden_audit() -> bool {
    let source = include_str!("px3_r_shared_cell_probe_v1.rs");
    let physical = source
        .split("// PX3_R_SHARED_CELL_ORGANISM_VISIBLE_BEGIN")
        .nth(1)
        .and_then(|text| {
            text.split("// PX3_R_SHARED_CELL_ORGANISM_VISIBLE_END")
                .next()
        })
        .unwrap_or("")
        .to_ascii_lowercase();
    let forbidden = [
        "event",
        "episode",
        "history",
        "pair",
        "group",
        "member",
        "pair_key",
        "semantic",
        "boundary",
        "evaluator",
        "selected",
        "old_m3",
        "adapter",
        "serializer",
        "co-occurrence",
        "cooccurrence",
        "trace-to-trace",
        "continuation",
    ];
    !physical.is_empty() && forbidden.iter().all(|token| !physical.contains(token))
}

fn sha256(path: &str) -> Option<String> {
    let output = Command::new("sha256sum").arg(path).output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()?
        .split_whitespace()
        .next()
        .map(str::to_owned)
}

fn command_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn write_atomic(report: &Report) {
    let mut csv = String::from(
        "name,namespace,stage,route_resistance,route_live,incoming_resistance,incoming_live,training_source_firings,training_local_firings,training_outward,held_out_local_firings,held_out_outward,permanent_fingerprint,complete_fingerprint,arrow_count,persistent_bytes,duplicate_exact,queue_comparisons,spikes_delivered,generation_checks,state_updates,threshold_checks,firings,arrow_checks,spikes_emitted,local_eligibility_writes,local_return_updates,ordinary_pressure_updates,local_structural_proposals,physical_deallocations,work\n",
    );
    for case in &report.cases {
        write_csv_row(&mut csv, case, "initial", &case.training, &case.initial);
        if let (Some(training), Some(snapshot)) = (&case.swap_training, &case.swapped) {
            write_csv_row(&mut csv, case, "swap", training, snapshot);
        }
    }

    let markdown = format!(
        "# PX3-R Arm B anonymous shared-CELL recruitment PROBE v1\n\nVerdict: **{}**.\n\nThis is an independent development-arm result. It does not advance PX3, spend definitive evidence, modify PX0-PX2, or authorize PX4.\n\n| clause | result |\n|---|:---:|\n| frozen start, parent, source, and negative hashes exact | {} |\n| individual-route strength and activity marginals matched | {} |\n| trained structures execute and crossed structures do not | {} |\n| old structures weaken and swapped structures relearn | {} |\n| harder controls, replay, source, and quiescence | {} |\n| forbidden-information audit | {} |\n| positive development candidate | {} |\n\nThe organism consumed only physical CELL specifications, pre-existing weak and ordinary ARROWs, external and ARROW-carried SPIKEs, retained local return, ordinary pressure, decay, refractory state, generation checks, and natural queue drain. The explicit environment supplied the symmetric six-site opportunity field, matched occurrence schedules, driver-mediated held-out use, distractors, pressure gaps, blocked return, absent opportunity, and stale opportunity.\n\nEvery route's separately serialized resistance/live multiset is equal. The result rows serialize each anonymous CELL's two incoming ARROW values separately from route-local strength, all six held-out physical uses, fingerprints, topology/storage, and complete work ledgers. Total ledgered work: `{}` operations across `{}` rows.\n",
        report.classification,
        report.frozen_exact,
        report.matched_marginals,
        report.trained_crossed,
        report.swap_relearning,
        report.hard_controls,
        report.forbidden_clean,
        report.positive,
        report.total_work,
        report.cases.len() + usize::from(report.cases[0].swapped.is_some()),
    );
    create_new(STAGING_CSV, csv.as_bytes());
    create_new(STAGING_MD, markdown.as_bytes());
    rename(STAGING_CSV, RESULT_CSV).expect("publish PROBE CSV");
    rename(STAGING_MD, RESULT_MD).expect("publish PROBE report");
}

fn write_csv_row(
    csv: &mut String,
    case: &Scenario,
    stage: &str,
    training: &physics::Counts,
    snapshot: &Snapshot,
) {
    let mut ledger = training.work.clone();
    add_work(&mut ledger, &snapshot.work);
    csv.push_str(&format!(
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
        case.name,
        case.namespace,
        stage,
        join_nested_u32(&snapshot.route_resistance),
        join_nested_bool(&snapshot.route_live),
        join_u32(&snapshot.state.incoming_resistance),
        join_bool(&snapshot.state.incoming_live),
        join_usize(&training.source_firings),
        join_usize(&training.local_firings),
        join_usize(&training.outward),
        join_nested_usize(&snapshot.uses.clone().map(|counts| counts.local_firings),),
        join_nested_usize(&snapshot.uses.clone().map(|counts| counts.outward)),
        snapshot.state.permanent_fingerprint,
        snapshot.state.complete_fingerprint,
        snapshot.state.arrow_count,
        snapshot.state.persistent_bytes,
        case.duplicate_exact,
        ledger_csv(&ledger),
        ledger.total(),
    ));
}

fn create_new(path: &str, bytes: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create staging artifact");
    file.write_all(bytes).expect("write staging artifact");
    file.sync_all().expect("sync staging artifact");
}

fn join_usize(values: &[usize]) -> String {
    values
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_u32(values: &[u32]) -> String {
    values
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_bool(values: &[bool]) -> String {
    values
        .iter()
        .map(bool::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_nested_usize<const N: usize, const M: usize>(values: &[[usize; M]; N]) -> String {
    values
        .iter()
        .map(|row| join_usize(row))
        .collect::<Vec<_>>()
        .join(":")
}

fn join_nested_u32<const N: usize, const M: usize>(values: &[[u32; M]; N]) -> String {
    values
        .iter()
        .map(|row| join_u32(row))
        .collect::<Vec<_>>()
        .join(":")
}

fn join_nested_bool<const N: usize, const M: usize>(values: &[[bool; M]; N]) -> String {
    values
        .iter()
        .map(|row| join_bool(row))
        .collect::<Vec<_>>()
        .join(":")
}

fn ledger_csv(work: &WorkLedger) -> String {
    [
        work.queue_comparisons,
        work.spikes_delivered,
        work.generation_checks,
        work.state_updates,
        work.threshold_checks,
        work.firings,
        work.arrow_checks,
        work.spikes_emitted,
        work.local_eligibility_writes,
        work.local_return_updates,
        work.ordinary_pressure_updates,
        work.local_structural_proposals,
        work.physical_deallocations,
    ]
    .iter()
    .map(u64::to_string)
    .collect::<Vec<_>>()
    .join(",")
}
