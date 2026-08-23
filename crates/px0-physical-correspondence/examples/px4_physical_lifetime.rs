use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{self, rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const SIDES: usize = 2;
const SOURCE_IMPULSES: usize = 4;
const ACQUISITION_EXPERIENCES: usize = 4;
const EXPERIENCE_SPACING: i64 = 14;
const WEAK_RESISTANCE: u32 = 3;
const PX2_COMMIT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const PX2_TAG: &str = "px2-physical-causal-direction-authoritative^{}";
const ACTIVE_LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX2_CSV_SHA256: &str = "921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18";
const PX2_MD_SHA256: &str = "eef9c336baea6aa1e5c2debde2e1286b2839759c55fd5fc008c7775fd4103cda";
const PX2_AUDIT_SHA256: &str = "7076aca03014d19040020b6bfb126e92f7d25dcac3df9cdab92de7dd7849c6fe";
const PX2_HANDOFF_SHA256: &str = "98647ab1563593e18e345cd7e5a71c4991d18b397dfe2dec71a4756106d96509";
const PROTOCOL_SHA256: &str = "fd152ef5c73c071e68fe41bb0e1b38707b00a43b8c2447ee647e847624876bb5";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Probe,
    Micro,
    Gate,
}

impl Stage {
    fn parse(value: &str) -> Self {
        match value {
            "probe" => Self::Probe,
            "micro" => Self::Micro,
            "gate" => Self::Gate,
            _ => panic!("stage must be probe, micro, or gate"),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Probe => "PROBE",
            Self::Micro => "MICRO",
            Self::Gate => "GATE",
        }
    }

    fn base(self) -> u64 {
        match self {
            Self::Probe => 0x6_4400_0000,
            Self::Micro => 0x6_5400_0000,
            Self::Gate => 0x6_7400_0000,
        }
    }

    fn cases(self) -> &'static [Case] {
        match self {
            Self::Probe | Self::Micro => &Case::CORE,
            Self::Gate => &Case::GATE,
        }
    }

    fn layouts(self) -> &'static [Layout] {
        match self {
            Self::Probe => &PROBE_LAYOUTS,
            Self::Micro => &MICRO_LAYOUTS,
            Self::Gate => &GATE_LAYOUTS,
        }
    }

    fn paths(self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            Self::Probe => (
                "results/px4_physical_lifetime_probe_v1.csv",
                "results/px4_physical_lifetime_probe_v1.md",
                "results/.px4_physical_lifetime_probe_v1.csv.staging",
                "results/.px4_physical_lifetime_probe_v1.md.staging",
            ),
            Self::Micro => (
                "results/px4_physical_lifetime_micro_v1.csv",
                "results/px4_physical_lifetime_micro_v1.md",
                "results/.px4_physical_lifetime_micro_v1.csv.staging",
                "results/.px4_physical_lifetime_micro_v1.md.staging",
            ),
            Self::Gate => (
                "results/px4_physical_lifetime_gate_v1.csv",
                "results/px4_physical_lifetime_gate_v1.md",
                "results/.px4_physical_lifetime_gate_v1.csv.staging",
                "results/.px4_physical_lifetime_gate_v1.md.staging",
            ),
        }
    }
}

// Case names and branching are evaluator-only. The substrate receives only
// physical CELL/ARROW/SPIKE construction and arrivals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    HighUse,
    LowUse,
    Disuse,
    ForwardToReverse,
    CorrelationOnly,
    ReturnAbsent,
    ReverseToForward,
    Reacquisition,
}

impl Case {
    const CORE: [Self; 6] = [
        Self::HighUse,
        Self::LowUse,
        Self::Disuse,
        Self::ForwardToReverse,
        Self::CorrelationOnly,
        Self::ReturnAbsent,
    ];
    const GATE: [Self; 8] = [
        Self::HighUse,
        Self::LowUse,
        Self::Disuse,
        Self::ForwardToReverse,
        Self::CorrelationOnly,
        Self::ReturnAbsent,
        Self::ReverseToForward,
        Self::Reacquisition,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::HighUse => "matched-high-use-survival",
            Self::LowUse => "matched-low-use-forgetting",
            Self::Disuse => "disuse-to-zero",
            Self::ForwardToReverse => "forward-to-reverse-competition",
            Self::CorrelationOnly => "correlation-without-traversal",
            Self::ReturnAbsent => "traversal-without-return",
            Self::ReverseToForward => "reverse-to-forward-competition",
            Self::Reacquisition => "full-deallocation-opposite-reacquisition",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Layout {
    name: &'static str,
    side_spacing: i32,
    traversal_delay: i64,
    first_tick: i64,
    mirror: bool,
    reverse_allocation: bool,
    reverse_arrival: bool,
    distractors: usize,
}

const PROBE_LAYOUTS: [Layout; 1] = [Layout {
    name: "P0",
    side_spacing: 34,
    traversal_delay: 3,
    first_tick: 71,
    mirror: false,
    reverse_allocation: false,
    reverse_arrival: false,
    distractors: 0,
}];

const MICRO_LAYOUTS: [Layout; 4] = [
    Layout {
        name: "M0",
        side_spacing: 34,
        traversal_delay: 3,
        first_tick: 71,
        mirror: false,
        reverse_allocation: false,
        reverse_arrival: false,
        distractors: 0,
    },
    Layout {
        name: "M1",
        side_spacing: 46,
        traversal_delay: 4,
        first_tick: 73,
        mirror: true,
        reverse_allocation: true,
        reverse_arrival: true,
        distractors: 4,
    },
    Layout {
        name: "M2",
        side_spacing: 58,
        traversal_delay: 5,
        first_tick: 79,
        mirror: false,
        reverse_allocation: true,
        reverse_arrival: false,
        distractors: 12,
    },
    Layout {
        name: "M3",
        side_spacing: 74,
        traversal_delay: 6,
        first_tick: 83,
        mirror: true,
        reverse_allocation: false,
        reverse_arrival: true,
        distractors: 24,
    },
];

const GATE_LAYOUTS: [Layout; 4] = [
    Layout {
        name: "G0",
        side_spacing: 38,
        traversal_delay: 3,
        first_tick: 89,
        mirror: false,
        reverse_allocation: false,
        reverse_arrival: false,
        distractors: 0,
    },
    Layout {
        name: "G1",
        side_spacing: 52,
        traversal_delay: 4,
        first_tick: 97,
        mirror: true,
        reverse_allocation: true,
        reverse_arrival: true,
        distractors: 8,
    },
    Layout {
        name: "G2",
        side_spacing: 68,
        traversal_delay: 5,
        first_tick: 101,
        mirror: false,
        reverse_allocation: true,
        reverse_arrival: false,
        distractors: 24,
    },
    Layout {
        name: "G3",
        side_spacing: 86,
        traversal_delay: 6,
        first_tick: 107,
        mirror: true,
        reverse_allocation: false,
        reverse_arrival: true,
        distractors: 48,
    },
];

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    arrivals: [CellId; SIDES],
    correspondence_ends: [CellId; SIDES],
    continuations: [CellId; SIDES],
    consequences: [CellId; SIDES],
    acquisition_drivers: [CellId; SIDES],
    participation_drivers: [CellId; SIDES],
    independent_drivers: [CellId; SIDES],
    distractor_drivers: Vec<CellId>,
    context: CellId,
    known_arrows: Vec<ArrowId>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ExecutionMetrics {
    effects: [usize; SIDES],
    continuation_firings: [usize; SIDES],
    consequence_firings: [usize; SIDES],
    quiescent: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct PhysicalOutcome {
    source_exact: bool,
    passed: bool,
    trained_resistance: [u32; SIDES],
    final_resistance: [u32; SIDES],
    trained_live: [bool; SIDES],
    final_live: [bool; SIDES],
    trained_generation: [u32; SIDES],
    final_generation: [u32; SIDES],
    before: ExecutionMetrics,
    after: ExecutionMetrics,
    stale: ExecutionMetrics,
    disuse_resistance: [u32; 4],
    disuse_monotone: bool,
    old_correspondence_live: [usize; SIDES],
    fresh_correspondence: [usize; SIDES],
    fresh_direction_ids: bool,
    stale_refused: bool,
    work: WorkLedger,
    arrow_slots: usize,
    known_live_arrows: usize,
    persistent_bytes: usize,
    fingerprint: u64,
    quiescent: bool,
}

#[derive(Clone, Debug)]
struct Row {
    stage: Stage,
    layout: Layout,
    case: Case,
    namespace: u64,
    outcome: PhysicalOutcome,
    duplicate_exact: bool,
}

fn main() {
    let mut args = env::args().skip(1);
    let mut stage = None;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--stage" => stage = Some(Stage::parse(&args.next().expect("--stage needs a value"))),
            "--definitive" => {
                eprintln!("PX4 definitive execution is not authorized");
                std::process::exit(2);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    let stage = stage.expect("development --stage is required");
    let source_exact = source_audit();
    let mut rows = Vec::new();
    for (layout_index, layout) in stage.layouts().iter().copied().enumerate() {
        for (case_index, case) in stage.cases().iter().copied().enumerate() {
            let namespace =
                stage.base() + layout_index as u64 * 0x0100_0000 + case_index as u64 * 0x0010_0000;
            let mut outcome = run_case(namespace, layout, case);
            let duplicate = run_case(namespace + 0x0008_0000, layout, case);
            outcome.source_exact = source_exact;
            let duplicate_exact = equivalent(&outcome, &duplicate);
            outcome.passed &= source_exact && duplicate_exact;
            rows.push(Row {
                stage,
                layout,
                case,
                namespace,
                outcome,
                duplicate_exact,
            });
        }
    }
    let passed = rows.iter().all(|row| row.outcome.passed);
    publish(stage, &csv(&rows), &markdown(stage, &rows, passed));
    std::process::exit(if passed { 0 } else { 1 });
}

fn run_case(namespace: u64, layout: Layout, case: Case) -> PhysicalOutcome {
    match case {
        Case::HighUse => run_use_case(namespace, layout, 0, 12, 120, true),
        Case::LowUse => run_use_case(namespace, layout, 0, 3, 120, false),
        Case::Disuse => run_disuse(namespace, layout),
        Case::ForwardToReverse => run_competition(namespace, layout, 0, 1),
        Case::CorrelationOnly => run_correlation(namespace, layout),
        Case::ReturnAbsent => run_return_absent(namespace, layout),
        Case::ReverseToForward => run_competition(namespace, layout, 1, 0),
        Case::Reacquisition => run_reacquisition(namespace, layout),
    }
}

fn run_use_case(
    namespace: u64,
    layout: Layout,
    active: usize,
    uses: usize,
    gap: i64,
    expected_survival: bool,
) -> PhysicalOutcome {
    let mut world = build_world(namespace, layout, true);
    let order = arrival_order(layout);
    let (correspondence, mut work, mut quiescent) =
        acquire_correspondence(&mut world, layout.first_tick, namespace + 0x10_000, order);
    remember(&mut world, &correspondence);
    let candidates = add_candidates(&mut world);
    let start = layout.first_tick + ACQUISITION_EXPERIENCES as i64 * 16 + 18;
    let training = train(
        &mut world,
        [active == 0, active == 1],
        [active != 0, active != 1],
        uses,
        start,
        namespace + 0x20_000,
        order,
    );
    add_work(&mut work, &training.0);
    quiescent &= training.1;
    let last = start + (uses as i64 - 1) * EXPERIENCE_SPACING;
    let trained_resistance = candidate_resistance(&world, &candidates);
    let trained_live = candidate_live(&world, &candidates);
    let trained_generation = candidate_generation(&world, &candidates);
    let before = measure(&world, last + 10, order);
    let gap_work = world.substrate.advance_time(last + gap);
    add_work(&mut work, &gap_work);
    let final_resistance = candidate_resistance(&world, &candidates);
    let final_live = candidate_live(&world, &candidates);
    let final_generation = candidate_generation(&world, &candidates);
    let after = measure(&world, last + gap, order);
    let survival_clause = final_live[active] == expected_survival
        && (expected_survival == (after.effects[active] > 0));
    let removal_clause = if expected_survival {
        final_resistance[active] > 0 && final_generation[active] == trained_generation[active]
    } else {
        final_resistance[active] == 0 && final_generation[active] > trained_generation[active]
    };
    let passed = trained_live[active]
        && trained_resistance[active] > 0
        && before.effects[active] > 0
        && survival_clause
        && removal_clause
        && before.quiescent
        && after.quiescent
        && quiescent;
    finish(
        world,
        PhysicalOutcome {
            passed,
            trained_resistance,
            final_resistance,
            trained_live,
            final_live,
            trained_generation,
            final_generation,
            before,
            after: after.clone(),
            stale: after,
            stale_refused: !expected_survival && final_resistance[active] == 0,
            work,
            quiescent,
            ..PhysicalOutcome::default()
        },
    )
}

fn run_disuse(namespace: u64, layout: Layout) -> PhysicalOutcome {
    let mut world = build_world(namespace, layout, true);
    let order = arrival_order(layout);
    let (correspondence, mut work, mut quiescent) =
        acquire_correspondence(&mut world, layout.first_tick, namespace + 0x10_000, order);
    remember(&mut world, &correspondence);
    let candidates = add_candidates(&mut world);
    let start = layout.first_tick + ACQUISITION_EXPERIENCES as i64 * 16 + 18;
    let training = train(
        &mut world,
        [true, false],
        [false, true],
        8,
        start,
        namespace + 0x20_000,
        order,
    );
    add_work(&mut work, &training.0);
    quiescent &= training.1;
    let last = start + 7 * EXPERIENCE_SPACING;
    let trained_resistance = candidate_resistance(&world, &candidates);
    let trained_live = candidate_live(&world, &candidates);
    let trained_generation = candidate_generation(&world, &candidates);
    let before = measure(&world, last + 10, order);
    let mut disuse_resistance = [trained_resistance[0], 0, 0, 0];
    for (index, gap) in [40, 120, 360].into_iter().enumerate() {
        let gap_work = world.substrate.advance_time(last + gap);
        add_work(&mut work, &gap_work);
        disuse_resistance[index + 1] = world.substrate.arrow_resistance(candidates[0]);
    }
    let final_resistance = candidate_resistance(&world, &candidates);
    let final_live = candidate_live(&world, &candidates);
    let final_generation = candidate_generation(&world, &candidates);
    let after = measure(&world, last + 360, order);
    let disuse_monotone = disuse_resistance.windows(2).all(|pair| pair[1] <= pair[0]);
    let passed = trained_live[0]
        && before.effects[0] > 0
        && disuse_monotone
        && disuse_resistance[3] == 0
        && !final_live[0]
        && final_generation[0] > trained_generation[0]
        && after.effects[0] == 0
        && before.quiescent
        && after.quiescent
        && quiescent;
    finish(
        world,
        PhysicalOutcome {
            passed,
            trained_resistance,
            final_resistance,
            trained_live,
            final_live,
            trained_generation,
            final_generation,
            before,
            after: after.clone(),
            stale: after,
            disuse_resistance,
            disuse_monotone,
            stale_refused: final_resistance[0] == 0,
            work,
            quiescent,
            ..PhysicalOutcome::default()
        },
    )
}

fn run_competition(
    namespace: u64,
    layout: Layout,
    old: usize,
    contemporary: usize,
) -> PhysicalOutcome {
    let mut world = build_world(namespace, layout, true);
    let order = arrival_order(layout);
    let (correspondence, mut work, mut quiescent) =
        acquire_correspondence(&mut world, layout.first_tick, namespace + 0x10_000, order);
    remember(&mut world, &correspondence);
    let old_candidates = add_candidates(&mut world);
    let first_start = layout.first_tick + ACQUISITION_EXPERIENCES as i64 * 16 + 18;
    let first_training = train(
        &mut world,
        [old == 0, old == 1],
        [old != 0, old != 1],
        8,
        first_start,
        namespace + 0x20_000,
        order,
    );
    add_work(&mut work, &first_training.0);
    quiescent &= first_training.1;
    let first_last = first_start + 7 * EXPERIENCE_SPACING;
    let before = measure(&world, first_last + 10, order);
    let trained_resistance = candidate_resistance(&world, &old_candidates);
    let trained_live = candidate_live(&world, &old_candidates);
    let trained_generation = candidate_generation(&world, &old_candidates);
    let contemporary_arrow = world.substrate.add_arrow(ArrowSpec {
        from: world.continuations[contemporary],
        to: world.consequences[contemporary],
        delay: 1,
        phase: 1,
        coupling: 1,
        resistance: WEAK_RESISTANCE,
    });
    world.known_arrows.push(contemporary_arrow);
    let second_start = first_last + 18;
    let second_training = train(
        &mut world,
        [contemporary == 0, contemporary == 1],
        [contemporary != 0, contemporary != 1],
        20,
        second_start,
        namespace + 0x30_000,
        order,
    );
    add_work(&mut work, &second_training.0);
    quiescent &= second_training.1;
    let second_last = second_start + 19 * EXPERIENCE_SPACING;
    let mut final_resistance = candidate_resistance(&world, &old_candidates);
    let mut final_live = candidate_live(&world, &old_candidates);
    let mut final_generation = candidate_generation(&world, &old_candidates);
    final_resistance[contemporary] = world.substrate.arrow_resistance(contemporary_arrow);
    final_live[contemporary] = world.substrate.arrow_is_live(contemporary_arrow);
    final_generation[contemporary] = world.substrate.arrow_generation(contemporary_arrow);
    let after = measure(&world, second_last + 10, order);
    let passed = trained_live[old]
        && before.effects[old] > 0
        && final_resistance[old] == 0
        && !final_live[old]
        && final_generation[old] > trained_generation[old]
        && final_live[contemporary]
        && final_resistance[contemporary] > 0
        && after.effects[old] == 0
        && after.effects[contemporary] > 0
        && before.quiescent
        && after.quiescent
        && quiescent;
    finish(
        world,
        PhysicalOutcome {
            passed,
            trained_resistance,
            final_resistance,
            trained_live,
            final_live,
            trained_generation,
            final_generation,
            before,
            after: after.clone(),
            stale: after,
            stale_refused: final_resistance[old] == 0,
            work,
            quiescent,
            ..PhysicalOutcome::default()
        },
    )
}

fn run_correlation(namespace: u64, layout: Layout) -> PhysicalOutcome {
    let mut world = build_world(namespace, layout, true);
    let order = arrival_order(layout);
    let (correspondence, mut work, mut quiescent) =
        acquire_correspondence(&mut world, layout.first_tick, namespace + 0x10_000, order);
    remember(&mut world, &correspondence);
    let candidates = add_candidates(&mut world);
    let start = layout.first_tick + ACQUISITION_EXPERIENCES as i64 * 16 + 18;
    let training = train(
        &mut world,
        [false, false],
        [true, true],
        12,
        start,
        namespace + 0x20_000,
        order,
    );
    add_work(&mut work, &training.0);
    quiescent &= training.1;
    let last = start + 11 * EXPERIENCE_SPACING;
    let final_resistance = candidate_resistance(&world, &candidates);
    let final_live = candidate_live(&world, &candidates);
    let final_generation = candidate_generation(&world, &candidates);
    let after = measure(&world, last + 10, order);
    let passed = final_resistance == [0, 0]
        && final_live == [false, false]
        && after.effects == [0, 0]
        && after.quiescent
        && quiescent;
    finish(
        world,
        PhysicalOutcome {
            passed,
            final_resistance,
            final_live,
            final_generation,
            after: after.clone(),
            stale: after,
            stale_refused: true,
            work,
            quiescent,
            ..PhysicalOutcome::default()
        },
    )
}

fn run_return_absent(namespace: u64, layout: Layout) -> PhysicalOutcome {
    let mut world = build_world(namespace, layout, false);
    let order = arrival_order(layout);
    let (correspondence, mut work, mut quiescent) =
        acquire_correspondence(&mut world, layout.first_tick, namespace + 0x10_000, order);
    remember(&mut world, &correspondence);
    let candidates = add_candidates(&mut world);
    let start = layout.first_tick + ACQUISITION_EXPERIENCES as i64 * 16 + 18;
    let training = train(
        &mut world,
        [true, false],
        [false, true],
        12,
        start,
        namespace + 0x20_000,
        order,
    );
    add_work(&mut work, &training.0);
    quiescent &= training.1;
    let last = start + 11 * EXPERIENCE_SPACING;
    let final_resistance = candidate_resistance(&world, &candidates);
    let final_live = candidate_live(&world, &candidates);
    let final_generation = candidate_generation(&world, &candidates);
    let after = measure(&world, last + 10, order);
    let passed = final_resistance == [0, 0]
        && final_live == [false, false]
        && after.effects == [0, 0]
        && after.quiescent
        && quiescent;
    finish(
        world,
        PhysicalOutcome {
            passed,
            final_resistance,
            final_live,
            final_generation,
            after: after.clone(),
            stale: after,
            stale_refused: true,
            work,
            quiescent,
            ..PhysicalOutcome::default()
        },
    )
}

fn run_reacquisition(namespace: u64, layout: Layout) -> PhysicalOutcome {
    let mut world = build_world(namespace, layout, true);
    let order = arrival_order(layout);
    let (old_correspondence, mut work, mut quiescent) =
        acquire_correspondence(&mut world, layout.first_tick, namespace + 0x10_000, order);
    remember(&mut world, &old_correspondence);
    let old_candidates = add_candidates(&mut world);
    let start = layout.first_tick + ACQUISITION_EXPERIENCES as i64 * 16 + 18;
    let training = train(
        &mut world,
        [true, false],
        [false, true],
        8,
        start,
        namespace + 0x20_000,
        order,
    );
    add_work(&mut work, &training.0);
    quiescent &= training.1;
    let last = start + 7 * EXPERIENCE_SPACING;
    let before = measure(&world, last + 10, order);
    let trained_resistance = candidate_resistance(&world, &old_candidates);
    let trained_live = candidate_live(&world, &old_candidates);
    let trained_generation = candidate_generation(&world, &old_candidates);
    let old_correspondence_generations: [Vec<u32>; SIDES] = std::array::from_fn(|side| {
        old_correspondence[side]
            .iter()
            .map(|arrow| world.substrate.arrow_generation(*arrow))
            .collect()
    });
    let gap_tick = last + 800;
    let gap_work = world.substrate.advance_time(gap_tick);
    add_work(&mut work, &gap_work);
    let old_correspondence_live = std::array::from_fn(|side| {
        old_correspondence[side]
            .iter()
            .filter(|arrow| world.substrate.arrow_is_live(**arrow))
            .count()
    });
    let stale = measure(&world, gap_tick, order);
    let old_correspondence_stale = (0..SIDES).all(|side| {
        old_correspondence[side]
            .iter()
            .zip(old_correspondence_generations[side].iter())
            .all(|(arrow, generation)| {
                !world.substrate.arrow_is_live(*arrow)
                    && world.substrate.arrow_generation(*arrow) > *generation
            })
    });
    let reacquisition_start = gap_tick + 16;
    let (fresh_correspondence_ids, reacquisition_work, reacquisition_quiescent) =
        acquire_correspondence(&mut world, reacquisition_start, namespace + 0x30_000, order);
    add_work(&mut work, &reacquisition_work);
    quiescent &= reacquisition_quiescent;
    let fresh_correspondence = std::array::from_fn(|side| {
        fresh_correspondence_ids[side]
            .iter()
            .filter(|arrow| !old_correspondence[side].contains(arrow))
            .count()
    });
    remember(&mut world, &fresh_correspondence_ids);
    let fresh_candidates = add_candidates(&mut world);
    let fresh_direction_ids = (0..SIDES).all(|side| fresh_candidates[side] != old_candidates[side]);
    let second_start = reacquisition_start + ACQUISITION_EXPERIENCES as i64 * 16 + 18;
    let second_training = train(
        &mut world,
        [false, true],
        [true, false],
        20,
        second_start,
        namespace + 0x40_000,
        order,
    );
    add_work(&mut work, &second_training.0);
    quiescent &= second_training.1;
    let second_last = second_start + 19 * EXPERIENCE_SPACING;
    let final_resistance = candidate_resistance(&world, &fresh_candidates);
    let final_live = candidate_live(&world, &fresh_candidates);
    let final_generation = candidate_generation(&world, &fresh_candidates);
    let after = measure(&world, second_last + 10, order);
    let stale_refused = stale.effects == [0, 0];
    let passed = trained_live[0]
        && before.effects[0] > 0
        && candidate_resistance(&world, &old_candidates) == [0, 0]
        && old_correspondence_live == [0, 0]
        && old_correspondence_stale
        && stale_refused
        && fresh_correspondence.iter().all(|count| *count > 0)
        && fresh_direction_ids
        && final_live[1]
        && final_resistance[1] > 0
        && after.effects == [0, 1]
        && before.quiescent
        && stale.quiescent
        && after.quiescent
        && quiescent;
    finish(
        world,
        PhysicalOutcome {
            passed,
            trained_resistance,
            final_resistance,
            trained_live,
            final_live,
            trained_generation,
            final_generation,
            before,
            after,
            stale,
            old_correspondence_live,
            fresh_correspondence,
            fresh_direction_ids,
            stale_refused,
            work,
            quiescent,
            ..PhysicalOutcome::default()
        },
    )
}

fn build_world(namespace: u64, layout: Layout, return_enabled: bool) -> World {
    let mut substrate = PlasticSubstrate::new();
    let mut arrivals = [None; SIDES];
    let mut correspondence_ends = [None; SIDES];
    let mut continuations = [None; SIDES];
    let mut consequences = [None; SIDES];
    let mut traces = [None; SIDES];
    let mut outside = [None; SIDES];
    let mut acquisition_drivers = [None; SIDES];
    let mut participation_drivers = [None; SIDES];
    let mut independent_drivers = [None; SIDES];
    let mut gates = [None; SIDES];
    let allocation_order = if layout.reverse_allocation {
        [1, 0]
    } else {
        [0, 1]
    };
    for side in allocation_order {
        let slot = if layout.mirror {
            SIDES - 1 - side
        } else {
            side
        };
        let base = slot as i32 * layout.side_spacing;
        arrivals[side] = Some(substrate.add_cell(cell(namespace + 10 + side as u64, base, 0, 4)));
        correspondence_ends[side] =
            Some(substrate.add_cell(cell(namespace + 20 + side as u64, base + 2, 0, 2)));
        continuations[side] =
            Some(substrate.add_cell(cell(namespace + 30 + side as u64, base + 8, 0, 2)));
        consequences[side] =
            Some(substrate.add_cell(cell(namespace + 40 + side as u64, base + 10, 0, 2)));
        traces[side] =
            Some(substrate.add_cell(cell(namespace + 50 + side as u64, base + 16, 0, 2)));
        outside[side] =
            Some(substrate.add_cell(cell(namespace + 60 + side as u64, 1_000 + base, 1, 1)));
        acquisition_drivers[side] =
            Some(substrate.add_cell(cell(namespace + 70 + side as u64, 1_100 + base, 0, 1)));
        participation_drivers[side] =
            Some(substrate.add_cell(cell(namespace + 80 + side as u64, 1_200 + base, 0, 1)));
        independent_drivers[side] =
            Some(substrate.add_cell(cell(namespace + 90 + side as u64, 1_300 + base, 0, 1)));
        gates[side] =
            Some(substrate.add_cell(cell(namespace + 100 + side as u64, 1_400 + base, 0, 1)));
    }
    let arrivals = arrivals.map(|value| value.expect("arrival"));
    let correspondence_ends = correspondence_ends.map(|value| value.expect("correspondence"));
    let continuations = continuations.map(|value| value.expect("continuation"));
    let consequences = consequences.map(|value| value.expect("consequence"));
    let traces = traces.map(|value| value.expect("trace"));
    let outside = outside.map(|value| value.expect("outside"));
    let acquisition_drivers = acquisition_drivers.map(|value| value.expect("acquisition driver"));
    let participation_drivers =
        participation_drivers.map(|value| value.expect("participation driver"));
    let independent_drivers = independent_drivers.map(|value| value.expect("independent driver"));
    let gates = gates.map(|value| value.expect("gate"));
    let context = substrate.add_cell(cell(namespace + 110, 1_500, 0, 1));
    let hub = substrate.add_cell(cell(namespace + 111, 1_600, 0, 1));
    let mut known_arrows = Vec::new();
    let mut distractor_drivers = Vec::new();
    for ordinal in 0..layout.distractors {
        let base = 10_000 + ordinal as i32 * 4;
        let driver = substrate.add_cell(cell(namespace + 1_000 + ordinal as u64 * 2, base, 0, 1));
        let sink = substrate.add_cell(cell(namespace + 1_001 + ordinal as u64 * 2, base + 1, 0, 1));
        known_arrows.push(substrate.add_arrow(arrow(driver, sink, 1, 1)));
        distractor_drivers.push(driver);
    }
    for side in allocation_order {
        known_arrows.push(substrate.add_arrow(arrow(
            acquisition_drivers[side],
            correspondence_ends[side],
            2,
            1,
        )));
        known_arrows.push(substrate.add_arrow(arrow(correspondence_ends[side], gates[side], 1, 1)));
        known_arrows.push(substrate.add_arrow(arrow(gates[side], arrivals[side], 1, 1)));
        known_arrows.push(substrate.add_arrow(arrow(
            correspondence_ends[side],
            continuations[side],
            layout.traversal_delay - 2,
            1,
        )));
        known_arrows.push(substrate.add_arrow(arrow(
            participation_drivers[side],
            continuations[side],
            layout.traversal_delay,
            1,
        )));
        known_arrows.push(substrate.add_arrow(arrow(
            participation_drivers[side],
            consequences[side],
            layout.traversal_delay + 1,
            1,
        )));
        known_arrows.push(substrate.add_arrow(arrow(
            independent_drivers[side],
            consequences[side],
            layout.traversal_delay + 1,
            2,
        )));
        known_arrows.push(substrate.add_arrow(arrow(
            context,
            continuations[side],
            layout.traversal_delay,
            1,
        )));
        known_arrows.push(substrate.add_arrow(arrow(consequences[side], traces[side], 1, 1)));
        if return_enabled {
            known_arrows.push(substrate.add_arrow(arrow(consequences[side], hub, 1, 1)));
        }
        known_arrows.push(substrate.add_arrow(arrow(consequences[side], outside[side], 0, 1)));
        known_arrows.push(substrate.add_arrow(arrow(traces[side], continuations[side], 1, 1)));
        known_arrows.push(substrate.add_arrow(arrow(hub, traces[side], 0, 1)));
    }
    World {
        substrate,
        namespace,
        arrivals,
        correspondence_ends,
        continuations,
        consequences,
        acquisition_drivers,
        participation_drivers,
        independent_drivers,
        distractor_drivers,
        context,
        known_arrows,
    }
}

fn acquire_correspondence(
    world: &mut World,
    start: i64,
    origin: u64,
    order: [usize; SIDES],
) -> ([Vec<ArrowId>; SIDES], WorkLedger, bool) {
    let mut work = WorkLedger::default();
    let mut quiescent = true;
    for experience in 0..ACQUISITION_EXPERIENCES {
        let tick = start + experience as i64 * 16;
        for side in order {
            enter_many(
                &mut world.substrate,
                world.arrivals[side],
                tick,
                SOURCE_IMPULSES,
                origin + experience as u64 * 0x100 + side as u64 * 0x10,
            );
            enter_many(
                &mut world.substrate,
                world.acquisition_drivers[side],
                tick,
                1,
                origin + 0x1_000 + experience as u64 * 0x100 + side as u64 * 0x10,
            );
        }
        let run = world.substrate.propagate();
        add_work(&mut work, &run.work);
        quiescent &= run.naturally_quiescent;
    }
    let correspondence = std::array::from_fn(|side| {
        world
            .substrate
            .arrows_between(world.arrivals[side], world.correspondence_ends[side])
            .into_iter()
            .filter(|arrow| world.substrate.arrow_is_live(*arrow))
            .collect()
    });
    (correspondence, work, quiescent)
}

fn add_candidates(world: &mut World) -> [ArrowId; SIDES] {
    std::array::from_fn(|side| {
        let id = world.substrate.add_arrow(ArrowSpec {
            from: world.continuations[side],
            to: world.consequences[side],
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: WEAK_RESISTANCE,
        });
        world.known_arrows.push(id);
        id
    })
}

fn train(
    world: &mut World,
    participants: [bool; SIDES],
    independent: [bool; SIDES],
    experiences: usize,
    start: i64,
    origin: u64,
    order: [usize; SIDES],
) -> (WorkLedger, bool) {
    let mut work = WorkLedger::default();
    let mut quiescent = true;
    for experience in 0..experiences {
        let tick = start + experience as i64 * EXPERIENCE_SPACING;
        for side in order {
            enter_many(
                &mut world.substrate,
                world.arrivals[side],
                tick,
                SOURCE_IMPULSES,
                origin + experience as u64 * 0x1_000 + side as u64 * 0x100,
            );
            if participants[side] {
                enter_many(
                    &mut world.substrate,
                    world.participation_drivers[side],
                    tick,
                    1,
                    origin + 0x100_000 + experience as u64 * 0x1_000 + side as u64 * 0x100,
                );
            }
            if independent[side] {
                enter_many(
                    &mut world.substrate,
                    world.independent_drivers[side],
                    tick,
                    1,
                    origin + 0x200_000 + experience as u64 * 0x1_000 + side as u64 * 0x100,
                );
            }
        }
        for (ordinal, driver) in world.distractor_drivers.iter().enumerate() {
            enter_many(
                &mut world.substrate,
                *driver,
                tick + 1,
                1,
                origin + 0x300_000 + experience as u64 * 0x1_000 + ordinal as u64,
            );
        }
        let run = world.substrate.propagate();
        add_work(&mut work, &run.work);
        quiescent &= run.naturally_quiescent;
    }
    (work, quiescent)
}

fn measure(world: &World, tick: i64, order: [usize; SIDES]) -> ExecutionMetrics {
    let mut clone = world.clone();
    clone.substrate.advance_time(tick);
    for side in order {
        enter_many(
            &mut clone.substrate,
            clone.arrivals[side],
            tick,
            SOURCE_IMPULSES,
            clone.namespace + 0x600_000 + side as u64 * 0x10,
        );
    }
    enter_many(
        &mut clone.substrate,
        clone.context,
        tick,
        1,
        clone.namespace + 0x700_000,
    );
    let run = clone.substrate.propagate();
    ExecutionMetrics {
        effects: std::array::from_fn(|side| outward_effects(&run, clone.namespace, side)),
        continuation_firings: std::array::from_fn(|side| {
            firings_at(&run, clone.namespace + 30 + side as u64)
        }),
        consequence_firings: std::array::from_fn(|side| {
            firings_at(&run, clone.namespace + 40 + side as u64)
        }),
        quiescent: run.naturally_quiescent,
    }
}

fn finish(world: World, mut outcome: PhysicalOutcome) -> PhysicalOutcome {
    outcome.arrow_slots = world.substrate.arrow_count();
    outcome.known_live_arrows = world
        .known_arrows
        .iter()
        .filter(|arrow| world.substrate.arrow_is_live(**arrow))
        .count();
    outcome.persistent_bytes = world.substrate.persistent_bytes();
    outcome.fingerprint = world.substrate.complete_fingerprint();
    outcome
}

fn equivalent(left: &PhysicalOutcome, right: &PhysicalOutcome) -> bool {
    left.passed == right.passed
        && left.trained_resistance == right.trained_resistance
        && left.final_resistance == right.final_resistance
        && left.trained_live == right.trained_live
        && left.final_live == right.final_live
        && left.trained_generation == right.trained_generation
        && left.final_generation == right.final_generation
        && left.before == right.before
        && left.after == right.after
        && left.stale == right.stale
        && left.disuse_resistance == right.disuse_resistance
        && left.disuse_monotone == right.disuse_monotone
        && left.old_correspondence_live == right.old_correspondence_live
        && left.fresh_correspondence == right.fresh_correspondence
        && left.fresh_direction_ids == right.fresh_direction_ids
        && left.stale_refused == right.stale_refused
        && left.work == right.work
        && left.arrow_slots == right.arrow_slots
        && left.known_live_arrows == right.known_live_arrows
        && left.persistent_bytes == right.persistent_bytes
        && left.quiescent == right.quiescent
}

fn source_audit() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == ACTIVE_LAW_SHA256
        && sha256("results/px2_physical_causal_direction_definitive.csv") == PX2_CSV_SHA256
        && sha256("results/px2_physical_causal_direction_definitive.md") == PX2_MD_SHA256
        && sha256("experiments/px2_physical_causal_direction_definitive_result_audit.md")
            == PX2_AUDIT_SHA256
        && sha256("experiments/px2_physical_causal_direction_authority_handoff.md")
            == PX2_HANDOFF_SHA256
        && sha256("experiments/px4_physical_lifetime_development_protocol.md") == PROTOCOL_SHA256
        && git_output(&["rev-parse", PX2_TAG]).is_some_and(|value| value == PX2_COMMIT)
        && Command::new("git")
            .args(["merge-base", "--is-ancestor", PX2_COMMIT, "HEAD"])
            .status()
            .is_ok_and(|status| status.success())
        && active_information_flow_audit()
}

fn active_information_flow_audit() -> bool {
    let substrate =
        fs::read_to_string("crates/px0-physical-correspondence/src/lib.rs").unwrap_or_default();
    let source =
        fs::read_to_string("crates/px0-physical-correspondence/examples/px4_physical_lifetime.rs")
            .unwrap_or_default();
    let substrate_forbidden = [
        "LifetimeClass",
        "RetentionPolicy",
        "DeletionPolicy",
        "EvaluatorPath",
        "TypedMemory",
        "M4Record",
        "DS6Record",
    ];
    let source_forbidden = [
        ["Lifetime", "Class"].concat(),
        ["Retention", "Policy"].concat(),
        ["Deletion", "Policy"].concat(),
        ["Evaluator", "Path"].concat(),
        ["Typed", "Memory"].concat(),
        ["M4", "Record"].concat(),
        ["DS6", "Record"].concat(),
        ["reinstatement", "_mode"].concat(),
        ["future", "_use"].concat(),
        ["t", "tl"].concat(),
        ["ex", "piry"].concat(),
    ];
    substrate_forbidden
        .iter()
        .all(|token| !substrate.contains(token))
        && source_forbidden.iter().all(|token| !source.contains(token))
}

fn remember(world: &mut World, arrows: &[Vec<ArrowId>; SIDES]) {
    for side in arrows {
        for arrow in side {
            if !world.known_arrows.contains(arrow) {
                world.known_arrows.push(*arrow);
            }
        }
    }
}

fn candidate_resistance(world: &World, candidates: &[ArrowId; SIDES]) -> [u32; SIDES] {
    std::array::from_fn(|side| world.substrate.arrow_resistance(candidates[side]))
}

fn candidate_live(world: &World, candidates: &[ArrowId; SIDES]) -> [bool; SIDES] {
    std::array::from_fn(|side| world.substrate.arrow_is_live(candidates[side]))
}

fn candidate_generation(world: &World, candidates: &[ArrowId; SIDES]) -> [u32; SIDES] {
    std::array::from_fn(|side| world.substrate.arrow_generation(candidates[side]))
}

fn arrival_order(layout: Layout) -> [usize; SIDES] {
    if layout.reverse_arrival {
        [1, 0]
    } else {
        [0, 1]
    }
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 1_000,
    }
}

fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 1_000,
    }
}

fn enter_many(
    substrate: &mut PlasticSubstrate,
    target: CellId,
    tick: i64,
    count: usize,
    origin: u64,
) {
    for ordinal in 0..count {
        substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: ordinal as i32,
            origin_physical: origin + ordinal as u64,
            target,
            impulse: 1,
        });
    }
}

fn firings_at(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn outward_effects(run: &Execution, namespace: u64, side: usize) -> usize {
    run.crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == namespace + 40 + side as u64
                && crossing.from_region == 0
                && crossing.to_region == 1
        })
        .count()
}

fn add_work(total: &mut WorkLedger, next: &WorkLedger) {
    total.queue_comparisons += next.queue_comparisons;
    total.spikes_delivered += next.spikes_delivered;
    total.generation_checks += next.generation_checks;
    total.state_updates += next.state_updates;
    total.threshold_checks += next.threshold_checks;
    total.firings += next.firings;
    total.arrow_checks += next.arrow_checks;
    total.spikes_emitted += next.spikes_emitted;
    total.local_eligibility_writes += next.local_eligibility_writes;
    total.local_return_updates += next.local_return_updates;
    total.ordinary_pressure_updates += next.ordinary_pressure_updates;
    total.local_structural_proposals += next.local_structural_proposals;
    total.physical_deallocations += next.physical_deallocations;
}

fn pair_u32(values: [u32; SIDES]) -> String {
    format!("{}|{}", values[0], values[1])
}

fn pair_usize(values: [usize; SIDES]) -> String {
    format!("{}|{}", values[0], values[1])
}

fn pair_bool(values: [bool; SIDES]) -> String {
    format!("{}|{}", values[0], values[1])
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "stage,layout,case,namespace,passed,source_exact,duplicate_exact,trained_resistance,final_resistance,trained_live,final_live,trained_generation,final_generation,before_effects,after_effects,stale_effects,disuse_resistance,disuse_monotone,old_correspondence_live,fresh_correspondence,fresh_direction_ids,stale_refused,quiescent,work_total,pressure_updates,deallocations,arrow_slots,known_live_arrows,persistent_bytes,fingerprint\n",
    );
    for row in rows {
        let outcome = &row.outcome;
        output.push_str(&format!(
            "{},{},{},0x{:x},{},{},{},{},{},{},{},{},{},{},{},{},{}|{}|{}|{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.stage.name(),
            row.layout.name,
            row.case.name(),
            row.namespace,
            outcome.passed,
            outcome.source_exact,
            row.duplicate_exact,
            pair_u32(outcome.trained_resistance),
            pair_u32(outcome.final_resistance),
            pair_bool(outcome.trained_live),
            pair_bool(outcome.final_live),
            pair_u32(outcome.trained_generation),
            pair_u32(outcome.final_generation),
            pair_usize(outcome.before.effects),
            pair_usize(outcome.after.effects),
            pair_usize(outcome.stale.effects),
            outcome.disuse_resistance[0],
            outcome.disuse_resistance[1],
            outcome.disuse_resistance[2],
            outcome.disuse_resistance[3],
            outcome.disuse_monotone,
            pair_usize(outcome.old_correspondence_live),
            pair_usize(outcome.fresh_correspondence),
            outcome.fresh_direction_ids,
            outcome.stale_refused,
            outcome.quiescent,
            outcome.work.total(),
            outcome.work.ordinary_pressure_updates,
            outcome.work.physical_deallocations,
            outcome.arrow_slots,
            outcome.known_live_arrows,
            outcome.persistent_bytes,
            outcome.fingerprint,
        ));
    }
    output
}

fn markdown(stage: Stage, rows: &[Row], passed: bool) -> String {
    let mut output = format!(
        "# PX4 physical learned-lifetime {} v1\n\nStatus: **{}** (`{}/{}` cells).\n\nFrozen parent: `{}` / `px2-physical-causal-direction-authoritative`.\n\nActive-law SHA-256: `{}`. No substrate or PX0--PX2 source changed.\n\nThis is development evidence only. It is not a definitive matrix and creates no authority.\n\n| layout | case | result | trained resistance | final resistance | before effects | after effects | stale refused | work | slots/live/bytes |\n|---|---|---:|---:|---:|---:|---:|---:|---:|---:|\n",
        stage.name(),
        if passed { "PASS" } else { "FAIL" },
        rows.iter().filter(|row| row.outcome.passed).count(),
        rows.len(),
        PX2_COMMIT,
        ACTIVE_LAW_SHA256,
    );
    for row in rows {
        let outcome = &row.outcome;
        output.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {}/{}/{} |\n",
            row.layout.name,
            row.case.name(),
            if outcome.passed { "PASS" } else { "FAIL" },
            pair_u32(outcome.trained_resistance),
            pair_u32(outcome.final_resistance),
            pair_usize(outcome.before.effects),
            pair_usize(outcome.after.effects),
            outcome.stale_refused,
            outcome.work.total(),
            outcome.arrow_slots,
            outcome.known_live_arrows,
            outcome.persistent_bytes,
        ));
    }
    output.push_str("\n## Interpretation\n\n");
    if passed {
        output.push_str(
            "All preregistered physical clauses passed using only byte-frozen PX0--PX2 state and laws. Resistance changed through actual traversal/ordinary return and ordinary pressure; zero-resistance paths refused stale execution. No lifetime-specific mechanism executed.\n",
        );
    } else {
        output.push_str(
            "At least one preregistered physical clause failed. This result is frozen without rescue or rerun.\n",
        );
    }
    output
}

fn publish(stage: Stage, csv_contents: &str, md_contents: &str) {
    let (csv_path, md_path, csv_staging, md_staging) = stage.paths();
    for path in [csv_path, md_path, csv_staging, md_staging] {
        assert!(!Path::new(path).exists(), "refusing to overwrite {path}");
    }
    write_new(csv_staging, csv_contents);
    write_new(md_staging, md_contents);
    rename(csv_staging, csv_path).expect("publish CSV");
    rename(md_staging, md_path).expect("publish Markdown");
}

fn write_new(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap_or_else(|error| panic!("create {path}: {error}"));
    file.write_all(contents.as_bytes())
        .unwrap_or_else(|error| panic!("write {path}: {error}"));
    file.sync_all()
        .unwrap_or_else(|error| panic!("sync {path}: {error}"));
}

fn sha256(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("run sha256sum");
    assert!(output.status.success(), "hash {path}");
    String::from_utf8(output.stdout)
        .expect("hash utf8")
        .split_whitespace()
        .next()
        .expect("hash field")
        .to_string()
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output.status.success().then(|| {
        String::from_utf8(output.stdout)
            .expect("git utf8")
            .trim()
            .to_string()
    })
}
