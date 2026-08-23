use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const SIDES: usize = 2;
const SOURCE_THRESHOLD: usize = 4;
const ACQUISITION: usize = 4;
const EXPERIENCES: usize = 24;
const EXPERIENCE_SPACING: i64 = 14;
const ADVERSARIAL_EXTRA: usize = 2;
const FORGET_GAP: i64 = 800;
const PX0_SOURCE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX1_CSV_SHA256: &str = "6613ff0a96bb3a60fbe7afeb92cd64edced3c6df5dcc04fe47518db158dd88f6";
const PX1_AUDIT_SHA256: &str = "fa4a516fcb6977a45e547ca1bb3b7db3b427c05b381fb60d2700e92fa2ae7c70";
const PX1_HANDOFF_SHA256: &str = "ab4142a24f6ca1095c1c1364f391253752808382ac6ee70ef9d49eac722df28c";
const PROTOCOL_SHA256: &str = "5c43ffda226a125bbbaf7f24dbb1ec8e70861b78a2d730c630535254def85c23";
const PROBE_V1_SHA256: &str = "bfc2963d2afcda7de28fc6c7e2636f34800a2b7e3fa51056319bc71fe8bb0de9";
const PROBE_V1_AUDIT_SHA256: &str =
    "45c6f331fe7d29403314495f9db532ef85ec03b4d6f32469d3d8f3d995982cb8";
const PROBE_V2_PROTOCOL_SHA256: &str =
    "4a8278c6f5cc42f996dfa662176d3c56204b822ae83f8608e84b0186a9ecac0f";
const PROBE_V2_SHA256: &str = "50ee980884107a6b84ec6c9724b6eeb14faadbd1bdbb2caad435ea79eda83fcd";
const PROBE_V2_AUDIT_SHA256: &str =
    "825aa0a190fb7da3934adb403b9187e8aa4634c0ed5bc70670c67e4422ae1194";
const PROBE_HANDOFF_SHA256: &str =
    "f11f6b98e63b7da817cbcc7b7f0accf66fe313394c4ddbd2274dd960b6355473";
const MICRO_PROTOCOL_SHA256: &str =
    "bfc51f7c4cb1cd675b04c25c3538f1e29e2022c97b01cac5dbc3e32e549249df";
const MICRO_SOURCE_SHA256: &str =
    "cc31a4992488c5fefe35703d34174ac6c50c0d4fbdd1c79623b3723a3aa4e27a";
const MICRO_CSV_SHA256: &str = "62ff9fa71834edcc0cb0b71232f6bc02221d48e5862d3b35ccc1187f8973d937";
const MICRO_AUDIT_SHA256: &str = "68ad6876bb6c88f7d6c7bee5ddf6b7ef8f65e3aee8c387bca2eae4574f0726f9";
const MICRO_HANDOFF_SHA256: &str =
    "aa433961c26a86783bdb372a23b2d0da982e14180f2fbeb51d2de5a407cce6c7";
const GATE_PROTOCOL_SHA256: &str =
    "fd0235ef42b54e4cc5a7e0f2673d57249f8b60b4bb2ca7ddc35396c162e75917";
const GATE_SOURCE_SHA256: &str = "8cdd72cff084c6a85d65629fd6504f5ca96f14d281a7a5ac518fd9c4754579ec";
const GATE_CSV_SHA256: &str = "ef63c70d3ce980d71cbe1e085174b654bd4dcc4505d3e308e2ed59a34abeaec5";
const GATE_NEGATIVE_AUDIT_SHA256: &str =
    "2e04bb306d6426181461357d4caeb8a10b9a8c7499ac2bac65f2f04efe6ac943";
const GATE_NEGATIVE_HANDOFF_SHA256: &str =
    "98865d55579e5ecdbe9981415227601990834c3187720a72053f3fc2f070a814";
const H1_PROTOCOL_SHA256: &str = "e16afd6adae4477900bb457427f4335431628f6e4c42c1ab054c69081c1b217b";
const H1_SUMMARY_CSV: &str = "results/px2_h1_matched_schedule_hysteresis_summary_v1.csv";
const H1_TRAJECTORY_CSV: &str = "results/px2_h1_matched_schedule_hysteresis_trajectory_v1.csv";
const H1_REPORT_MD: &str = "results/px2_h1_matched_schedule_hysteresis_v1.md";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum Scenario {
    Forward,
    Reverse,
    CorrelationOnly,
    Joint,
    BlockedReturn,
    AdversarialForward,
    AdversarialReverse,
    InterleavedForwardFirst,
    InterleavedReverseFirst,
    LifecycleForwardToReverse,
    LifecycleReverseToForward,
}

#[allow(dead_code)]
impl Scenario {
    const ALL: [Self; 11] = [
        Self::Forward,
        Self::Reverse,
        Self::CorrelationOnly,
        Self::Joint,
        Self::BlockedReturn,
        Self::AdversarialForward,
        Self::AdversarialReverse,
        Self::InterleavedForwardFirst,
        Self::InterleavedReverseFirst,
        Self::LifecycleForwardToReverse,
        Self::LifecycleReverseToForward,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Forward => "forward-participation",
            Self::Reverse => "reverse-participation",
            Self::CorrelationOnly => "correlation-only",
            Self::Joint => "joint-participation",
            Self::BlockedReturn => "participation-without-return",
            Self::AdversarialForward => "adversarial-correlation-forward-traversal",
            Self::AdversarialReverse => "adversarial-correlation-reverse-traversal",
            Self::InterleavedForwardFirst => "interleaved-forward-first",
            Self::InterleavedReverseFirst => "interleaved-reverse-first",
            Self::LifecycleForwardToReverse => "lifecycle-forward-to-reverse",
            Self::LifecycleReverseToForward => "lifecycle-reverse-to-forward",
        }
    }

    fn participants(self, experience: usize) -> [bool; SIDES] {
        match self {
            Self::Forward
            | Self::BlockedReturn
            | Self::AdversarialForward
            | Self::LifecycleForwardToReverse => [true, false],
            Self::Reverse | Self::AdversarialReverse | Self::LifecycleReverseToForward => {
                [false, true]
            }
            Self::Joint => [true, true],
            Self::CorrelationOnly => [false, false],
            Self::InterleavedForwardFirst => {
                if experience.is_multiple_of(2) {
                    [true, false]
                } else {
                    [false, true]
                }
            }
            Self::InterleavedReverseFirst => {
                if experience.is_multiple_of(2) {
                    [false, true]
                } else {
                    [true, false]
                }
            }
        }
    }

    fn independent_changes(self, experience: usize) -> [bool; SIDES] {
        match self {
            Self::Forward
            | Self::BlockedReturn
            | Self::AdversarialForward
            | Self::LifecycleForwardToReverse => [false, true],
            Self::Reverse | Self::AdversarialReverse | Self::LifecycleReverseToForward => {
                [true, false]
            }
            Self::CorrelationOnly => [true, true],
            Self::Joint => [false, false],
            Self::InterleavedForwardFirst | Self::InterleavedReverseFirst => {
                let participants = self.participants(experience);
                [!participants[0], !participants[1]]
            }
        }
    }

    fn adversarial_wrong_side(self) -> Option<usize> {
        match self {
            Self::AdversarialForward => Some(1),
            Self::AdversarialReverse => Some(0),
            _ => None,
        }
    }

    fn lifecycle_reacquisition(self) -> Option<Self> {
        match self {
            Self::LifecycleForwardToReverse => Some(Self::Reverse),
            Self::LifecycleReverseToForward => Some(Self::Forward),
            _ => None,
        }
    }

    fn is_lifecycle(self) -> bool {
        self.lifecycle_reacquisition().is_some()
    }

    fn expected_mature(self) -> [bool; SIDES] {
        match self {
            Self::Forward | Self::AdversarialForward | Self::LifecycleReverseToForward => {
                [true, false]
            }
            Self::Reverse | Self::AdversarialReverse | Self::LifecycleForwardToReverse => {
                [false, true]
            }
            Self::Joint | Self::InterleavedForwardFirst | Self::InterleavedReverseFirst => {
                [true, true]
            }
            Self::CorrelationOnly | Self::BlockedReturn => [false, false],
        }
    }

    fn return_enabled(self) -> bool {
        self != Self::BlockedReturn
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Stratum {
    name: &'static str,
    side_spacing: i32,
    traversal_delay: i64,
    first_experience: i64,
    heldout_gap: i64,
    postgap_gap: i64,
    distractor_load: usize,
    parallel_paths: usize,
    mirror: bool,
    reverse_allocation: bool,
    reverse_arrival: bool,
}

const STRATA: [Stratum; 4] = [
    Stratum {
        name: "G0",
        side_spacing: 36,
        traversal_delay: 3,
        first_experience: 66,
        heldout_gap: 14,
        postgap_gap: 34,
        distractor_load: 0,
        parallel_paths: 1,
        mirror: false,
        reverse_allocation: false,
        reverse_arrival: false,
    },
    Stratum {
        name: "G1",
        side_spacing: 48,
        traversal_delay: 4,
        first_experience: 67,
        heldout_gap: 14,
        postgap_gap: 34,
        distractor_load: 8,
        parallel_paths: 2,
        mirror: true,
        reverse_allocation: true,
        reverse_arrival: true,
    },
    Stratum {
        name: "G2",
        side_spacing: 64,
        traversal_delay: 5,
        first_experience: 68,
        heldout_gap: 14,
        postgap_gap: 34,
        distractor_load: 24,
        parallel_paths: 3,
        mirror: false,
        reverse_allocation: true,
        reverse_arrival: false,
    },
    Stratum {
        name: "G3",
        side_spacing: 80,
        traversal_delay: 6,
        first_experience: 69,
        heldout_gap: 14,
        postgap_gap: 34,
        distractor_load: 48,
        parallel_paths: 4,
        mirror: true,
        reverse_allocation: false,
        reverse_arrival: true,
    },
];

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    traversal_delay: i64,
    arrivals: [CellId; SIDES],
    correspondence_ends: [CellId; SIDES],
    continuations: [CellId; SIDES],
    consequences: [CellId; SIDES],
    acquisition_drivers: [CellId; SIDES],
    participation_drivers: [CellId; SIDES],
    independent_drivers: [CellId; SIDES],
    distractor_drivers: Vec<CellId>,
    context: CellId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExecutionMetrics {
    continuation_firings: [usize; SIDES],
    consequence_firings: [usize; SIDES],
    trace_arrivals: [usize; SIDES],
    trace_firings: [usize; SIDES],
    local_returns: [usize; SIDES],
    effects: [usize; SIDES],
    extra_arrival_firings: usize,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Metrics {
    correspondence_resistance: [u32; SIDES],
    directional_live_paths: [usize; SIDES],
    directional_min_resistance: [u32; SIDES],
    directional_max_resistance: [u32; SIDES],
    training_continuation_firings: [usize; SIDES],
    training_consequence_firings: [usize; SIDES],
    training_trace_arrivals: [usize; SIDES],
    training_trace_firings: [usize; SIDES],
    training_local_returns: [usize; SIDES],
    training_effects: [usize; SIDES],
    training_distractor_firings: usize,
    training_extra_arrival_firings: usize,
    heldout: ExecutionMetrics,
    postgap: ExecutionMetrics,
    training_quiescent: bool,
    lifecycle_first_effects: [usize; SIDES],
    lifecycle_first_extra_arrival_firings: usize,
    lifecycle_first_quiescent: bool,
    lifecycle_old_direction_live: [usize; SIDES],
    lifecycle_old_direction_stale: bool,
    lifecycle_old_correspondence_live: [usize; SIDES],
    lifecycle_old_correspondence_stale: bool,
    lifecycle_stale_effects: [usize; SIDES],
    lifecycle_stale_extra_arrival_firings: usize,
    lifecycle_stale_quiescent: bool,
    lifecycle_fresh_correspondence: [usize; SIDES],
    lifecycle_fresh_direction_ids: bool,
    work: WorkLedger,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ResultRow {
    stratum: &'static str,
    scenario: Scenario,
    metrics: Metrics,
    duplicate_exact: bool,
    passed: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct TrainingMetrics {
    continuation_firings: [usize; SIDES],
    consequence_firings: [usize; SIDES],
    trace_arrivals: [usize; SIDES],
    trace_firings: [usize; SIDES],
    local_returns: [usize; SIDES],
    effects: [usize; SIDES],
    distractor_firings: usize,
    extra_arrival_firings: usize,
    quiescent: bool,
    work: WorkLedger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScheduleKind {
    ForwardBlock,
    ReverseBlock,
    ForwardAlternating,
    ReverseAlternating,
    Rotation(usize),
}

impl ScheduleKind {
    const ALL: [Self; 10] = [
        Self::ForwardBlock,
        Self::ReverseBlock,
        Self::ForwardAlternating,
        Self::ReverseAlternating,
        Self::Rotation(0),
        Self::Rotation(1),
        Self::Rotation(2),
        Self::Rotation(3),
        Self::Rotation(4),
        Self::Rotation(5),
    ];

    fn name(self) -> String {
        match self {
            Self::ForwardBlock => "forward-block".to_string(),
            Self::ReverseBlock => "reverse-block".to_string(),
            Self::ForwardAlternating => "forward-alternating".to_string(),
            Self::ReverseAlternating => "reverse-alternating".to_string(),
            Self::Rotation(value) => format!("rotation-{value}"),
        }
    }

    fn schedule(self) -> Vec<usize> {
        match self {
            Self::ForwardBlock => [vec![0; 12], vec![1; 12]].concat(),
            Self::ReverseBlock => [vec![1; 12], vec![0; 12]].concat(),
            Self::ForwardAlternating => (0..EXPERIENCES).map(|value| value % 2).collect(),
            Self::ReverseAlternating => (0..EXPERIENCES).map(|value| 1 - value % 2).collect(),
            Self::Rotation(rotation) => {
                let base = [0usize, 0, 1, 0, 1, 1];
                (0..EXPERIENCES)
                    .map(|value| base[(value + rotation) % base.len()])
                    .collect()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct H1Step {
    experience: usize,
    participant: usize,
    continuation_firings: [usize; SIDES],
    trace_firings: [usize; SIDES],
    local_returns: [usize; SIDES],
    live_before: [bool; SIDES],
    live_after: [bool; SIDES],
    resistance_before: [u32; SIDES],
    resistance_after: [u32; SIDES],
    return_gain: [u32; SIDES],
    pressure_spent: [u32; SIDES],
    naturally_quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct H1Metrics {
    schedule: Vec<usize>,
    steps: Vec<H1Step>,
    first_mature: [i32; SIDES],
    first_deallocation: [i32; SIDES],
    final_live: [usize; SIDES],
    final_resistance: [u32; SIDES],
    heldout_effects: [usize; SIDES],
    postgap_effects: [usize; SIDES],
    source_refiring: usize,
    naturally_quiescent: bool,
    work: WorkLedger,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct H1Row {
    stratum: &'static str,
    schedule_name: String,
    metrics: H1Metrics,
    duplicate_exact: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args != ["--h1-schedule-diagnostic"] {
        eprintln!("PX2-H1 development requires --h1-schedule-diagnostic");
        std::process::exit(2);
    }
    assert!(
        source_audit(),
        "authoritative PX0/PX1 inputs must remain exact"
    );
    for path in [H1_SUMMARY_CSV, H1_TRAJECTORY_CSV, H1_REPORT_MD] {
        assert!(!Path::new(path).exists(), "PX2-H1 result exists");
    }
    eprintln!("PX2_H1_MATCHED_SCHEDULE_HYSTERESIS_DIAGNOSTIC_V1_EVIDENCE");

    let mut rows = Vec::new();
    for (stratum_ordinal, stratum) in STRATA.into_iter().enumerate() {
        let diagnostic_stratum = Stratum {
            distractor_load: 0,
            parallel_paths: 1,
            ..stratum
        };
        for (schedule_ordinal, schedule) in ScheduleKind::ALL.into_iter().enumerate() {
            let namespace = 0x2_4200_0000
                + stratum_ordinal as u64 * 0x1000_0000
                + schedule_ordinal as u64 * 0x0100_0000;
            let first = run_h1(namespace, diagnostic_stratum, schedule);
            let second = run_h1(namespace, diagnostic_stratum, schedule);
            let duplicate_exact = first == second;
            rows.push(H1Row {
                stratum: diagnostic_stratum.name,
                schedule_name: schedule.name(),
                metrics: first,
                duplicate_exact,
            });
        }
    }

    write_new(H1_SUMMARY_CSV, &h1_summary_csv(&rows));
    write_new(H1_TRAJECTORY_CSV, &h1_trajectory_csv(&rows));
    write_new(H1_REPORT_MD, &h1_markdown(&rows));
}

fn source_audit() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == PX0_SOURCE_SHA256
        && sha256("results/px1_physical_boundary_roles_definitive.csv") == PX1_CSV_SHA256
        && sha256("experiments/px1_physical_boundary_roles_definitive_result_audit.md")
            == PX1_AUDIT_SHA256
        && sha256("experiments/px1_physical_boundary_roles_authority_handoff.md")
            == PX1_HANDOFF_SHA256
        && sha256("experiments/px2_physical_causal_direction_trace_sufficiency_protocol.md")
            == PROTOCOL_SHA256
        && sha256("results/px2_physical_causal_direction_trace_sufficiency_probe_v1.csv")
            == PROBE_V1_SHA256
        && sha256(
            "experiments/px2_physical_causal_direction_trace_sufficiency_probe_v1_negative_audit.md",
        ) == PROBE_V1_AUDIT_SHA256
        && sha256(
            "experiments/px2_physical_causal_direction_trace_sufficiency_probe_v2_protocol.md",
        ) == PROBE_V2_PROTOCOL_SHA256
        && sha256("results/px2_physical_causal_direction_trace_sufficiency_probe_v2.csv")
            == PROBE_V2_SHA256
        && sha256(
            "experiments/px2_physical_causal_direction_trace_sufficiency_probe_v2_result_audit.md",
        ) == PROBE_V2_AUDIT_SHA256
        && sha256("experiments/px2_physical_causal_direction_trace_sufficiency_handoff.md")
            == PROBE_HANDOFF_SHA256
        && sha256("experiments/px2_physical_causal_direction_trace_sufficiency_micro_protocol.md")
            == MICRO_PROTOCOL_SHA256
        && git_sha256(
            "px2-physical-causal-direction-trace-sufficiency-micro-implementation-v1",
            "crates/px0-physical-correspondence/examples/px2_physical_causal_direction.rs",
        ) == MICRO_SOURCE_SHA256
        && sha256("results/px2_physical_causal_direction_trace_sufficiency_micro_v1.csv")
            == MICRO_CSV_SHA256
        && sha256(
            "experiments/px2_physical_causal_direction_trace_sufficiency_micro_v1_result_audit.md",
        ) == MICRO_AUDIT_SHA256
        && sha256(
            "experiments/px2_physical_causal_direction_trace_sufficiency_micro_v1_handoff.md",
        ) == MICRO_HANDOFF_SHA256
        && sha256("experiments/px2_physical_causal_direction_trace_sufficiency_gate_protocol.md")
            == GATE_PROTOCOL_SHA256
        && git_sha256(
            "px2-physical-causal-direction-trace-sufficiency-gate-implementation-v1",
            "crates/px0-physical-correspondence/examples/px2_physical_causal_direction.rs",
        ) == GATE_SOURCE_SHA256
        && sha256("results/px2_physical_causal_direction_trace_sufficiency_gate_v1.csv")
            == GATE_CSV_SHA256
        && sha256(
            "experiments/px2_physical_causal_direction_trace_sufficiency_gate_v1_negative_audit.md",
        ) == GATE_NEGATIVE_AUDIT_SHA256
        && sha256(
            "experiments/px2_physical_causal_direction_trace_sufficiency_gate_v1_negative_handoff.md",
        ) == GATE_NEGATIVE_HANDOFF_SHA256
        && sha256("experiments/px2_h1_matched_schedule_hysteresis_diagnostic_protocol.md")
            == H1_PROTOCOL_SHA256
}

fn run_h1(namespace: u64, stratum: Stratum, schedule_kind: ScheduleKind) -> H1Metrics {
    let mut world = build_world(namespace, true, stratum);
    let order = arrival_order(stratum);
    let (_, mut work, acquisition_quiescent) =
        acquire_correspondence(&mut world, 0, namespace + 0x10_000, order);
    let directional = add_directional_candidates(&mut world, 1);
    let schedule = schedule_kind.schedule();
    assert_eq!(schedule.iter().filter(|side| **side == 0).count(), 12);
    assert_eq!(schedule.iter().filter(|side| **side == 1).count(), 12);
    let mut steps = Vec::new();
    let mut first_mature = [-1i32; SIDES];
    let mut first_deallocation = [-1i32; SIDES];
    let mut source_refiring = 0usize;
    let mut naturally_quiescent = acquisition_quiescent;

    for (experience, participant) in schedule.iter().copied().enumerate() {
        let tick = stratum.first_experience + experience as i64 * EXPERIENCE_SPACING;
        let live_before =
            std::array::from_fn(|side| world.substrate.arrow_is_live(directional[side][0]));
        let resistance_before = std::array::from_fn(|side| {
            if live_before[side] {
                world.substrate.arrow_resistance(directional[side][0])
            } else {
                0
            }
        });
        for side in order {
            enter_many(
                &mut world.substrate,
                world.arrivals[side],
                tick,
                SOURCE_THRESHOLD,
                namespace + 0x20_000 + experience as u64 * 0x1_000 + side as u64 * 0x100,
            );
            if side == participant {
                enter_many(
                    &mut world.substrate,
                    world.participation_drivers[side],
                    tick,
                    1,
                    namespace + 0x120_000 + experience as u64 * 0x1_000 + side as u64 * 0x100,
                );
            } else {
                enter_many(
                    &mut world.substrate,
                    world.independent_drivers[side],
                    tick,
                    1,
                    namespace + 0x220_000 + experience as u64 * 0x1_000 + side as u64 * 0x100,
                );
            }
        }
        let run = world.substrate.propagate();
        let continuation_firings =
            std::array::from_fn(|side| firings_at(&run, continuation_physical(namespace, side)));
        let trace_firings =
            std::array::from_fn(|side| firings_at(&run, trace_physical(namespace, side)));
        let local_returns = std::array::from_fn(|side| {
            arrivals_at(&run, continuation_physical(namespace, side))
                .saturating_sub(1 + usize::from(side == participant))
        });
        let live_after =
            std::array::from_fn(|side| world.substrate.arrow_is_live(directional[side][0]));
        let resistance_after = std::array::from_fn(|side| {
            if live_after[side] {
                world.substrate.arrow_resistance(directional[side][0])
            } else {
                0
            }
        });
        let return_gain = std::array::from_fn(|side| {
            if live_before[side] && continuation_firings[side] > 0 && local_returns[side] > 0 {
                3
            } else {
                0
            }
        });
        let pressure_spent = std::array::from_fn(|side| {
            resistance_before[side]
                .saturating_add(return_gain[side])
                .saturating_sub(resistance_after[side])
        });
        for side in 0..SIDES {
            if first_mature[side] < 0 && resistance_after[side] > 3 {
                first_mature[side] = experience as i32;
            }
            if first_deallocation[side] < 0 && live_before[side] && !live_after[side] {
                first_deallocation[side] = experience as i32;
            }
            source_refiring += firings_at(&run, arrival_physical(namespace, side));
        }
        source_refiring = source_refiring.saturating_sub(SIDES);
        add_work(&mut work, &run.work);
        naturally_quiescent &= run.naturally_quiescent;
        steps.push(H1Step {
            experience,
            participant,
            continuation_firings,
            trace_firings,
            local_returns,
            live_before,
            live_after,
            resistance_before,
            resistance_after,
            return_gain,
            pressure_spent,
            naturally_quiescent: run.naturally_quiescent,
        });
    }

    let last_tick = stratum.first_experience + (EXPERIENCES as i64 - 1) * EXPERIENCE_SPACING;
    let heldout = measure_execution(&world, last_tick + stratum.heldout_gap, order);
    let postgap = measure_execution(&world, last_tick + stratum.postgap_gap, order);
    source_refiring += heldout.extra_arrival_firings + postgap.extra_arrival_firings;
    naturally_quiescent &= heldout.quiescent && postgap.quiescent;

    H1Metrics {
        schedule,
        steps,
        first_mature,
        first_deallocation,
        final_live: std::array::from_fn(|side| live_count(&world.substrate, &directional[side])),
        final_resistance: std::array::from_fn(|side| {
            max_resistance(&world.substrate, &directional[side])
        }),
        heldout_effects: heldout.effects,
        postgap_effects: postgap.effects,
        source_refiring,
        naturally_quiescent,
        work,
        fingerprint: world.substrate.complete_fingerprint(),
    }
}

#[allow(dead_code)]
fn run_world(namespace: u64, scenario: Scenario, stratum: Stratum) -> Metrics {
    if scenario.is_lifecycle() {
        return run_lifecycle(namespace, scenario, stratum);
    }
    let mut world = build_world(namespace, scenario.return_enabled(), stratum);
    let arrival_order = arrival_order(stratum);
    let (correspondence, mut work, acquisition_quiescent) =
        acquire_correspondence(&mut world, 0, namespace + 0x10_000, arrival_order);
    let directional = add_directional_candidates(&mut world, stratum.parallel_paths);
    let training = train(
        &mut world,
        scenario,
        stratum,
        stratum.first_experience,
        namespace + 0x20_000,
        arrival_order,
    );
    add_work(&mut work, &training.work);
    let last_experience_tick =
        stratum.first_experience + (EXPERIENCES as i64 - 1) * EXPERIENCE_SPACING;
    let heldout = measure_execution(
        &world,
        last_experience_tick + stratum.heldout_gap,
        arrival_order,
    );
    let postgap = measure_execution(
        &world,
        last_experience_tick + stratum.postgap_gap,
        arrival_order,
    );
    let (directional_live_paths, directional_min_resistance, directional_max_resistance) =
        directional_stats(&world.substrate, &directional);
    let fingerprint = world.substrate.complete_fingerprint();

    Metrics {
        correspondence_resistance: std::array::from_fn(|side| {
            max_resistance(&world.substrate, &correspondence[side])
        }),
        directional_live_paths,
        directional_min_resistance,
        directional_max_resistance,
        training_continuation_firings: training.continuation_firings,
        training_consequence_firings: training.consequence_firings,
        training_trace_arrivals: training.trace_arrivals,
        training_trace_firings: training.trace_firings,
        training_local_returns: training.local_returns,
        training_effects: training.effects,
        training_distractor_firings: training.distractor_firings,
        training_extra_arrival_firings: training.extra_arrival_firings,
        heldout,
        postgap,
        training_quiescent: acquisition_quiescent && training.quiescent,
        lifecycle_first_effects: [0, 0],
        lifecycle_first_extra_arrival_firings: 0,
        lifecycle_first_quiescent: false,
        lifecycle_old_direction_live: [0, 0],
        lifecycle_old_direction_stale: false,
        lifecycle_old_correspondence_live: [0, 0],
        lifecycle_old_correspondence_stale: false,
        lifecycle_stale_effects: [0, 0],
        lifecycle_stale_extra_arrival_firings: 0,
        lifecycle_stale_quiescent: false,
        lifecycle_fresh_correspondence: [0, 0],
        lifecycle_fresh_direction_ids: false,
        work,
        fingerprint,
    }
}

fn run_lifecycle(namespace: u64, scenario: Scenario, stratum: Stratum) -> Metrics {
    let mut world = build_world(namespace, true, stratum);
    let arrival_order = arrival_order(stratum);
    let (old_correspondence, mut work, first_acquisition_quiescent) =
        acquire_correspondence(&mut world, 0, namespace + 0x10_000, arrival_order);
    let old_correspondence_generations: [Vec<u32>; SIDES] = std::array::from_fn(|side| {
        old_correspondence[side]
            .iter()
            .map(|arrow| world.substrate.arrow_generation(*arrow))
            .collect()
    });
    let old_directional = add_directional_candidates(&mut world, stratum.parallel_paths);
    let old_directional_generations: [Vec<u32>; SIDES] = std::array::from_fn(|side| {
        old_directional[side]
            .iter()
            .map(|arrow| world.substrate.arrow_generation(*arrow))
            .collect()
    });
    let first_training = train(
        &mut world,
        scenario,
        stratum,
        stratum.first_experience,
        namespace + 0x20_000,
        arrival_order,
    );
    add_work(&mut work, &first_training.work);
    let first_last_tick = stratum.first_experience + (EXPERIENCES as i64 - 1) * EXPERIENCE_SPACING;
    let first_heldout =
        measure_execution(&world, first_last_tick + stratum.heldout_gap, arrival_order);

    let gap_tick = first_last_tick + FORGET_GAP;
    let gap_work = world.substrate.advance_time(gap_tick);
    add_work(&mut work, &gap_work);
    let lifecycle_old_direction_live =
        std::array::from_fn(|side| live_count(&world.substrate, &old_directional[side]));
    let lifecycle_old_direction_stale = stale_with_new_generation(
        &world.substrate,
        &old_directional,
        &old_directional_generations,
    );
    let lifecycle_old_correspondence_live =
        std::array::from_fn(|side| live_count(&world.substrate, &old_correspondence[side]));
    let lifecycle_old_correspondence_stale = stale_with_new_generation(
        &world.substrate,
        &old_correspondence,
        &old_correspondence_generations,
    );
    let stale_execution = measure_execution(&world, gap_tick, arrival_order);

    let reacquisition_start = gap_tick + 16;
    let (fresh_correspondence, reacquisition_work, reacquisition_quiescent) =
        acquire_correspondence(
            &mut world,
            reacquisition_start,
            namespace + 0x30_000,
            arrival_order,
        );
    add_work(&mut work, &reacquisition_work);
    let lifecycle_fresh_correspondence = std::array::from_fn(|side| {
        fresh_correspondence[side]
            .iter()
            .filter(|arrow| !old_correspondence[side].contains(arrow))
            .count()
    });
    let fresh_directional = add_directional_candidates(&mut world, stratum.parallel_paths);
    let lifecycle_fresh_direction_ids = (0..SIDES).all(|side| {
        fresh_directional[side]
            .iter()
            .all(|arrow| !old_directional[side].contains(arrow))
    });
    let contemporary = scenario.lifecycle_reacquisition().expect("lifecycle world");
    let second_start = reacquisition_start + ACQUISITION as i64 * 16 + 18;
    let second_training = train(
        &mut world,
        contemporary,
        stratum,
        second_start,
        namespace + 0x40_000,
        arrival_order,
    );
    add_work(&mut work, &second_training.work);
    let second_last_tick = second_start + (EXPERIENCES as i64 - 1) * EXPERIENCE_SPACING;
    let heldout = measure_execution(
        &world,
        second_last_tick + stratum.heldout_gap,
        arrival_order,
    );
    let postgap = measure_execution(
        &world,
        second_last_tick + stratum.postgap_gap,
        arrival_order,
    );
    let (directional_live_paths, directional_min_resistance, directional_max_resistance) =
        directional_stats(&world.substrate, &fresh_directional);

    Metrics {
        correspondence_resistance: std::array::from_fn(|side| {
            max_resistance(&world.substrate, &fresh_correspondence[side])
        }),
        directional_live_paths,
        directional_min_resistance,
        directional_max_resistance,
        training_continuation_firings: second_training.continuation_firings,
        training_consequence_firings: second_training.consequence_firings,
        training_trace_arrivals: second_training.trace_arrivals,
        training_trace_firings: second_training.trace_firings,
        training_local_returns: second_training.local_returns,
        training_effects: second_training.effects,
        training_distractor_firings: second_training.distractor_firings,
        training_extra_arrival_firings: second_training.extra_arrival_firings,
        heldout,
        postgap,
        training_quiescent: first_acquisition_quiescent
            && first_training.quiescent
            && reacquisition_quiescent
            && second_training.quiescent,
        lifecycle_first_effects: first_heldout.effects,
        lifecycle_first_extra_arrival_firings: first_heldout.extra_arrival_firings,
        lifecycle_first_quiescent: first_heldout.quiescent,
        lifecycle_old_direction_live,
        lifecycle_old_direction_stale,
        lifecycle_old_correspondence_live,
        lifecycle_old_correspondence_stale,
        lifecycle_stale_effects: stale_execution.effects,
        lifecycle_stale_extra_arrival_firings: stale_execution.extra_arrival_firings,
        lifecycle_stale_quiescent: stale_execution.quiescent,
        lifecycle_fresh_correspondence,
        lifecycle_fresh_direction_ids,
        work,
        fingerprint: world.substrate.complete_fingerprint(),
    }
}

fn arrival_order(stratum: Stratum) -> [usize; SIDES] {
    if stratum.reverse_arrival {
        [1, 0]
    } else {
        [0, 1]
    }
}

fn acquire_correspondence(
    world: &mut World,
    start_tick: i64,
    origin_base: u64,
    arrival_order: [usize; SIDES],
) -> ([Vec<ArrowId>; SIDES], WorkLedger, bool) {
    let mut work = WorkLedger::default();
    let mut quiescent = true;
    for experience in 0..ACQUISITION {
        let tick = start_tick + experience as i64 * 16;
        for side in arrival_order {
            enter_many(
                &mut world.substrate,
                world.arrivals[side],
                tick,
                SOURCE_THRESHOLD,
                origin_base + experience as u64 * 0x100 + side as u64 * 0x10,
            );
            enter_many(
                &mut world.substrate,
                world.acquisition_drivers[side],
                tick,
                1,
                origin_base + 0x1_000 + experience as u64 * 0x100 + side as u64 * 0x10,
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
            .collect::<Vec<_>>()
    });
    (correspondence, work, quiescent)
}

fn add_directional_candidates(world: &mut World, parallel_paths: usize) -> [Vec<ArrowId>; SIDES] {
    std::array::from_fn(|side| {
        (0..parallel_paths)
            .map(|ordinal| {
                world.substrate.add_arrow(ArrowSpec {
                    from: world.continuations[side],
                    to: world.consequences[side],
                    delay: 1,
                    phase: ordinal as i32,
                    coupling: 1,
                    resistance: 3,
                })
            })
            .collect()
    })
}

fn train(
    world: &mut World,
    scenario: Scenario,
    stratum: Stratum,
    start_tick: i64,
    origin_base: u64,
    arrival_order: [usize; SIDES],
) -> TrainingMetrics {
    let mut metrics = TrainingMetrics {
        quiescent: true,
        ..TrainingMetrics::default()
    };
    let mut arrival_firings = 0usize;
    for experience in 0..EXPERIENCES {
        let tick = start_tick + experience as i64 * EXPERIENCE_SPACING;
        let participants = scenario.participants(experience);
        let independent = scenario.independent_changes(experience);
        for side in arrival_order {
            enter_many(
                &mut world.substrate,
                world.arrivals[side],
                tick,
                SOURCE_THRESHOLD,
                origin_base + experience as u64 * 0x1_000 + side as u64 * 0x100,
            );
            if participants[side] {
                enter_many(
                    &mut world.substrate,
                    world.participation_drivers[side],
                    tick,
                    1,
                    origin_base + 0x100_000 + experience as u64 * 0x1_000 + side as u64 * 0x100,
                );
            }
            if independent[side] {
                enter_many(
                    &mut world.substrate,
                    world.independent_drivers[side],
                    tick,
                    1,
                    origin_base + 0x200_000 + experience as u64 * 0x1_000 + side as u64 * 0x100,
                );
            }
        }
        if let Some(side) = scenario.adversarial_wrong_side() {
            for extra in 1..=ADVERSARIAL_EXTRA {
                enter_many(
                    &mut world.substrate,
                    world.independent_drivers[side],
                    tick + extra as i64 * 2,
                    1,
                    origin_base + 0x300_000 + experience as u64 * 0x1_000 + extra as u64 * 0x100,
                );
            }
        }
        for (ordinal, driver) in world.distractor_drivers.iter().enumerate() {
            enter_many(
                &mut world.substrate,
                *driver,
                tick + 1,
                1,
                origin_base + 0x400_000 + experience as u64 * 0x1_000 + ordinal as u64,
            );
        }
        let run = world.substrate.propagate();
        for (side, participated) in participants.into_iter().enumerate() {
            metrics.continuation_firings[side] +=
                firings_at(&run, continuation_physical(world.namespace, side));
            metrics.consequence_firings[side] +=
                firings_at(&run, consequence_physical(world.namespace, side));
            metrics.trace_arrivals[side] +=
                arrivals_at(&run, trace_physical(world.namespace, side));
            metrics.trace_firings[side] += firings_at(&run, trace_physical(world.namespace, side));
            let ordinary_inputs = 1 + usize::from(participated);
            metrics.local_returns[side] +=
                arrivals_at(&run, continuation_physical(world.namespace, side))
                    .saturating_sub(ordinary_inputs);
            metrics.effects[side] += outward_effects(&run, world.namespace, side);
            arrival_firings += firings_at(&run, arrival_physical(world.namespace, side));
        }
        metrics.distractor_firings += (0..stratum.distractor_load)
            .map(|ordinal| firings_at(&run, distractor_sink_physical(world.namespace, ordinal)))
            .sum::<usize>();
        add_work(&mut metrics.work, &run.work);
        metrics.quiescent &= run.naturally_quiescent;
    }
    metrics.extra_arrival_firings = arrival_firings.saturating_sub(EXPERIENCES * SIDES);
    metrics
}

fn build_world(namespace: u64, return_enabled: bool, stratum: Stratum) -> World {
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
    let allocation_order = if stratum.reverse_allocation {
        [1, 0]
    } else {
        [0, 1]
    };
    for side in allocation_order {
        let slot = if stratum.mirror {
            SIDES - 1 - side
        } else {
            side
        };
        let base = slot as i32 * stratum.side_spacing;
        arrivals[side] =
            Some(substrate.add_cell(cell(arrival_physical(namespace, side), base, 0, 4)));
        correspondence_ends[side] = Some(substrate.add_cell(cell(
            correspondence_physical(namespace, side),
            base + 2,
            0,
            2,
        )));
        continuations[side] =
            Some(substrate.add_cell(cell(continuation_physical(namespace, side), base + 8, 0, 2)));
        consequences[side] =
            Some(substrate.add_cell(cell(consequence_physical(namespace, side), base + 10, 0, 2)));
        traces[side] =
            Some(substrate.add_cell(cell(trace_physical(namespace, side), base + 16, 0, 2)));
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
    let mut distractor_drivers = Vec::new();
    for ordinal in 0..stratum.distractor_load {
        let base = 10_000 + ordinal as i32 * 4;
        let driver = substrate.add_cell(cell(
            distractor_driver_physical(namespace, ordinal),
            base,
            0,
            1,
        ));
        let sink = substrate.add_cell(cell(
            distractor_sink_physical(namespace, ordinal),
            base + 1,
            0,
            1,
        ));
        substrate.add_arrow(arrow(driver, sink, 1, 1));
        distractor_drivers.push(driver);
    }

    for side in allocation_order {
        substrate.add_arrow(arrow(
            acquisition_drivers[side],
            correspondence_ends[side],
            2,
            1,
        ));
        substrate.add_arrow(arrow(correspondence_ends[side], gates[side], 1, 1));
        substrate.add_arrow(arrow(gates[side], arrivals[side], 1, 1));
        substrate.add_arrow(arrow(
            correspondence_ends[side],
            continuations[side],
            stratum.traversal_delay - 2,
            1,
        ));
        substrate.add_arrow(arrow(
            participation_drivers[side],
            continuations[side],
            stratum.traversal_delay,
            1,
        ));
        substrate.add_arrow(arrow(
            participation_drivers[side],
            consequences[side],
            stratum.traversal_delay + 1,
            1,
        ));
        substrate.add_arrow(arrow(
            independent_drivers[side],
            consequences[side],
            stratum.traversal_delay + 1,
            2,
        ));
        substrate.add_arrow(arrow(
            context,
            continuations[side],
            stratum.traversal_delay,
            1,
        ));
        substrate.add_arrow(arrow(consequences[side], traces[side], 1, 1));
        if return_enabled {
            substrate.add_arrow(arrow(consequences[side], hub, 1, 1));
        }
        substrate.add_arrow(arrow(consequences[side], outside[side], 0, 1));
        substrate.add_arrow(arrow(traces[side], continuations[side], 1, 1));
        substrate.add_arrow(arrow(hub, traces[side], 0, 1));
    }

    World {
        substrate,
        namespace,
        traversal_delay: stratum.traversal_delay,
        arrivals,
        correspondence_ends,
        continuations,
        consequences,
        acquisition_drivers,
        participation_drivers,
        independent_drivers,
        distractor_drivers,
        context,
    }
}

fn measure_execution(world: &World, tick: i64, arrival_order: [usize; SIDES]) -> ExecutionMetrics {
    let mut clone = world.clone();
    clone.substrate.advance_time(tick);
    for side in arrival_order {
        enter_many(
            &mut clone.substrate,
            clone.arrivals[side],
            tick,
            SOURCE_THRESHOLD,
            clone.namespace + 0x6_000 + side as u64 * 0x10,
        );
    }
    enter_many(
        &mut clone.substrate,
        clone.context,
        tick,
        1,
        clone.namespace + 0x7_000,
    );
    let run = clone.substrate.propagate();
    let continuation_firings =
        std::array::from_fn(|side| firings_at(&run, continuation_physical(clone.namespace, side)));
    let consequence_firings =
        std::array::from_fn(|side| firings_at(&run, consequence_physical(clone.namespace, side)));
    let trace_arrivals = std::array::from_fn(|side| {
        arrivals_at_tick(
            &run,
            trace_physical(clone.namespace, side),
            tick + clone.traversal_delay + 2,
        )
    });
    let trace_firings =
        std::array::from_fn(|side| firings_at(&run, trace_physical(clone.namespace, side)));
    let local_returns = std::array::from_fn(|side| {
        arrivals_at_tick(
            &run,
            continuation_physical(clone.namespace, side),
            tick + clone.traversal_delay + 3,
        )
    });
    let effects = std::array::from_fn(|side| outward_effects(&run, clone.namespace, side));
    let arrival_firings = (0..SIDES)
        .map(|side| firings_at(&run, arrival_physical(clone.namespace, side)))
        .sum::<usize>();
    ExecutionMetrics {
        continuation_firings,
        consequence_firings,
        trace_arrivals,
        trace_firings,
        local_returns,
        effects,
        extra_arrival_firings: arrival_firings.saturating_sub(SIDES),
        quiescent: run.naturally_quiescent,
    }
}

#[allow(dead_code)]
fn gate_passed(scenario: Scenario, stratum: Stratum, metrics: &Metrics) -> bool {
    let effective = scenario.lifecycle_reacquisition().unwrap_or(scenario);
    let participants = std::array::from_fn(|side| {
        (0..EXPERIENCES)
            .filter(|experience| effective.participants(*experience)[side])
            .count()
    });
    let expected_mature = scenario.expected_mature();
    let expected_effects = expected_mature.map(usize::from);
    let expected_live_paths =
        expected_mature.map(|mature| if mature { stratum.parallel_paths } else { 0 });
    let consequence_firings_exact = match effective {
        Scenario::BlockedReturn => {
            metrics.training_consequence_firings[0] > 0
                && metrics.training_consequence_firings[0] <= EXPERIENCES
                && metrics.training_consequence_firings[1] == EXPERIENCES
        }
        Scenario::AdversarialForward => {
            metrics.training_consequence_firings[0] == EXPERIENCES
                && metrics.training_consequence_firings[1]
                    >= metrics.training_consequence_firings[0] * (ADVERSARIAL_EXTRA + 1)
        }
        Scenario::AdversarialReverse => {
            metrics.training_consequence_firings[1] == EXPERIENCES
                && metrics.training_consequence_firings[0]
                    >= metrics.training_consequence_firings[1] * (ADVERSARIAL_EXTRA + 1)
        }
        _ => metrics.training_consequence_firings == [EXPERIENCES, EXPERIENCES],
    };
    let return_physics = if effective.return_enabled() {
        metrics
            .training_trace_firings
            .iter()
            .all(|value| *value > 0)
            && metrics
                .training_local_returns
                .iter()
                .all(|value| *value > 0)
    } else {
        metrics.training_trace_firings == [0, 0] && metrics.training_local_returns == [0, 0]
    };
    let lifecycle = if scenario.is_lifecycle() {
        let first_effects = match scenario {
            Scenario::LifecycleForwardToReverse => [1, 0],
            Scenario::LifecycleReverseToForward => [0, 1],
            _ => unreachable!(),
        };
        metrics.lifecycle_first_effects == first_effects
            && metrics.lifecycle_first_extra_arrival_firings == 0
            && metrics.lifecycle_first_quiescent
            && metrics.lifecycle_old_direction_live == [0, 0]
            && metrics.lifecycle_old_direction_stale
            && metrics.lifecycle_old_correspondence_live == [0, 0]
            && metrics.lifecycle_old_correspondence_stale
            && metrics.lifecycle_stale_effects == [0, 0]
            && metrics.lifecycle_stale_extra_arrival_firings == 0
            && metrics.lifecycle_stale_quiescent
            && metrics
                .lifecycle_fresh_correspondence
                .iter()
                .all(|value| *value > 0)
            && metrics.lifecycle_fresh_direction_ids
    } else {
        true
    };
    metrics
        .correspondence_resistance
        .iter()
        .all(|value| *value > 1)
        && metrics.training_continuation_firings == participants
        && consequence_firings_exact
        && return_physics
        && metrics.training_effects == metrics.training_consequence_firings
        && metrics.training_distractor_firings == stratum.distractor_load * EXPERIENCES
        && metrics.training_extra_arrival_firings == 0
        && metrics.directional_live_paths == expected_live_paths
        && (0..SIDES).all(|side| {
            if expected_mature[side] {
                metrics.directional_min_resistance[side] > 3
                    && metrics.directional_max_resistance[side]
                        >= metrics.directional_min_resistance[side]
            } else {
                metrics.directional_min_resistance[side] == 0
                    && metrics.directional_max_resistance[side] == 0
            }
        })
        && metrics.heldout.continuation_firings == [1, 1]
        && metrics.heldout.consequence_firings == expected_effects
        && metrics.heldout.trace_firings == expected_effects
        && metrics.heldout.local_returns == expected_effects
        && metrics.heldout.effects == expected_effects
        && metrics.postgap.effects == expected_effects
        && metrics.heldout.extra_arrival_firings == 0
        && metrics.postgap.extra_arrival_firings == 0
        && metrics.training_quiescent
        && metrics.heldout.quiescent
        && metrics.postgap.quiescent
        && lifecycle
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

fn arrival_physical(namespace: u64, side: usize) -> u64 {
    namespace + 10 + side as u64
}

fn correspondence_physical(namespace: u64, side: usize) -> u64 {
    namespace + 20 + side as u64
}

fn continuation_physical(namespace: u64, side: usize) -> u64 {
    namespace + 30 + side as u64
}

fn consequence_physical(namespace: u64, side: usize) -> u64 {
    namespace + 40 + side as u64
}

fn trace_physical(namespace: u64, side: usize) -> u64 {
    namespace + 50 + side as u64
}

fn distractor_driver_physical(namespace: u64, ordinal: usize) -> u64 {
    namespace + 0x500 + ordinal as u64 * 2
}

fn distractor_sink_physical(namespace: u64, ordinal: usize) -> u64 {
    distractor_driver_physical(namespace, ordinal) + 1
}

fn firings_at(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn arrivals_at_tick(run: &Execution, physical: u64, tick: i64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.tick == tick)
        .count()
}

fn arrivals_at(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical)
        .count()
}

fn outward_effects(run: &Execution, namespace: u64, side: usize) -> usize {
    run.crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == consequence_physical(namespace, side)
                && crossing.to_physical == namespace + 60 + side as u64
        })
        .count()
}

fn max_resistance(substrate: &PlasticSubstrate, arrows: &[ArrowId]) -> u32 {
    arrows
        .iter()
        .filter(|arrow| substrate.arrow_is_live(**arrow))
        .map(|arrow| substrate.arrow_resistance(*arrow))
        .max()
        .unwrap_or(0)
}

fn live_count(substrate: &PlasticSubstrate, arrows: &[ArrowId]) -> usize {
    arrows
        .iter()
        .filter(|arrow| substrate.arrow_is_live(**arrow))
        .count()
}

fn directional_stats(
    substrate: &PlasticSubstrate,
    directional: &[Vec<ArrowId>; SIDES],
) -> ([usize; SIDES], [u32; SIDES], [u32; SIDES]) {
    let live = std::array::from_fn(|side| live_count(substrate, &directional[side]));
    let minimum = std::array::from_fn(|side| {
        directional[side]
            .iter()
            .filter(|arrow| substrate.arrow_is_live(**arrow))
            .map(|arrow| substrate.arrow_resistance(*arrow))
            .min()
            .unwrap_or(0)
    });
    let maximum = std::array::from_fn(|side| max_resistance(substrate, &directional[side]));
    (live, minimum, maximum)
}

fn stale_with_new_generation(
    substrate: &PlasticSubstrate,
    arrows: &[Vec<ArrowId>; SIDES],
    generations: &[Vec<u32>; SIDES],
) -> bool {
    (0..SIDES).all(|side| {
        arrows[side]
            .iter()
            .zip(&generations[side])
            .all(|(arrow, generation)| {
                !substrate.arrow_is_live(*arrow)
                    && substrate.arrow_generation(*arrow) != *generation
            })
    })
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

fn sha256(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("run sha256sum");
    assert!(output.status.success(), "hash {path}");
    String::from_utf8(output.stdout)
        .expect("utf8 hash")
        .split_whitespace()
        .next()
        .expect("hash")
        .to_string()
}

fn git_sha256(reference: &str, path: &str) -> String {
    let output = Command::new("git")
        .args(["show", &format!("{reference}:{path}")])
        .output()
        .expect("run git show");
    assert!(output.status.success(), "read frozen git blob");
    let mut child = Command::new("sha256sum")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("run sha256sum for git blob");
    child
        .stdin
        .as_mut()
        .expect("sha256 stdin")
        .write_all(&output.stdout)
        .expect("hash git blob");
    let hashed = child.wait_with_output().expect("finish git blob hash");
    assert!(hashed.status.success(), "hash frozen git blob");
    String::from_utf8(hashed.stdout)
        .expect("utf8 hash")
        .split_whitespace()
        .next()
        .expect("hash")
        .to_string()
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

fn h1_winner(effects: [usize; SIDES]) -> i32 {
    match effects {
        [1, 0] => 0,
        [0, 1] => 1,
        [1, 1] => 2,
        _ => -1,
    }
}

fn h1_classification(rows: &[H1Row]) -> &'static str {
    let controlled = rows.iter().all(|row| {
        row.duplicate_exact
            && row.metrics.source_refiring == 0
            && row.metrics.naturally_quiescent
            && row
                .metrics
                .schedule
                .iter()
                .filter(|side| **side == 0)
                .count()
                == 12
            && row
                .metrics
                .schedule
                .iter()
                .filter(|side| **side == 1)
                .count()
                == 12
    });
    let protection_first = rows.iter().all(|row| {
        let first = row.metrics.schedule[0];
        let other = 1 - first;
        h1_winner(row.metrics.heldout_effects) == first as i32
            && row.metrics.first_mature[first] >= 0
            && row.metrics.first_deallocation[other] >= 0
            && row.metrics.first_mature[first] < row.metrics.first_deallocation[other]
    });
    let symmetric = rows.iter().all(|row| row.metrics.heldout_effects == [1, 1]);
    if controlled && protection_first {
        "A — protection-first hysteresis"
    } else if controlled && symmetric {
        "B — matched histories remain symmetric"
    } else if controlled {
        "C — ordering matters but protection-first is not sufficient"
    } else {
        "D — no stable classification"
    }
}

fn h1_summary_csv(rows: &[H1Row]) -> String {
    let mut output = String::from(
        "stratum,schedule,schedule_first,forward_count,reverse_count,first_mature,first_deallocation,final_live,final_resistance,heldout_effects,postgap_effects,source_refiring,quiescent,work,fingerprint,duplicate_exact\n",
    );
    for row in rows {
        let value = &row.metrics;
        let fields = [
            row.stratum.to_string(),
            row.schedule_name.clone(),
            value.schedule[0].to_string(),
            value
                .schedule
                .iter()
                .filter(|side| **side == 0)
                .count()
                .to_string(),
            value
                .schedule
                .iter()
                .filter(|side| **side == 1)
                .count()
                .to_string(),
            format!("{}|{}", value.first_mature[0], value.first_mature[1]),
            format!(
                "{}|{}",
                value.first_deallocation[0], value.first_deallocation[1]
            ),
            pair_usize(value.final_live),
            pair_u32(value.final_resistance),
            pair_usize(value.heldout_effects),
            pair_usize(value.postgap_effects),
            value.source_refiring.to_string(),
            value.naturally_quiescent.to_string(),
            value.work.total().to_string(),
            value.fingerprint.to_string(),
            row.duplicate_exact.to_string(),
        ];
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

fn h1_trajectory_csv(rows: &[H1Row]) -> String {
    let mut output = String::from(
        "stratum,schedule,experience,participant,continuation_firings,trace_firings,local_returns,live_before,live_after,resistance_before,resistance_after,return_gain,pressure_spent,quiescent\n",
    );
    for row in rows {
        for step in &row.metrics.steps {
            let fields = [
                row.stratum.to_string(),
                row.schedule_name.clone(),
                step.experience.to_string(),
                step.participant.to_string(),
                pair_usize(step.continuation_firings),
                pair_usize(step.trace_firings),
                pair_usize(step.local_returns),
                pair_bool(step.live_before),
                pair_bool(step.live_after),
                pair_u32(step.resistance_before),
                pair_u32(step.resistance_after),
                pair_u32(step.return_gain),
                pair_u32(step.pressure_spent),
                step.naturally_quiescent.to_string(),
            ];
            output.push_str(&fields.join(","));
            output.push('\n');
        }
    }
    output
}

fn h1_markdown(rows: &[H1Row]) -> String {
    let mut output = format!(
        "# PX2-H1 matched-schedule hysteresis diagnostic v1\n\nClassification: **{}**.\n\nCells: `{}`; duplicate-exact: `{}`.\n\n| stratum | schedule | first | first mature | first deallocation | final resistance | held-out | post-gap | replay |\n|---|---|---:|---:|---:|---:|---:|---:|---:|\n",
        h1_classification(rows),
        rows.len(),
        rows.iter().filter(|row| row.duplicate_exact).count(),
    );
    for row in rows {
        let value = &row.metrics;
        output.push_str(&format!(
            "| {} | {} | {} | `{}|{}` | `{}|{}` | `{}` | `{}` | `{}` | {} |\n",
            row.stratum,
            row.schedule_name,
            value.schedule[0],
            value.first_mature[0],
            value.first_mature[1],
            value.first_deallocation[0],
            value.first_deallocation[1],
            pair_u32(value.final_resistance),
            pair_usize(value.heldout_effects),
            pair_usize(value.postgap_effects),
            row.duplicate_exact,
        ));
    }
    output.push_str(
        "\nThe substrate law is unchanged. This diagnostic does not repair GATE v1, advance PX2, or unblock PX3.\n",
    );
    output
}

#[allow(dead_code)]
fn csv(rows: &[ResultRow]) -> String {
    let mut output = String::from(
        "stratum,scenario,correspondence_resistance,directional_live_paths,directional_min_resistance,directional_max_resistance,training_continuation_firings,training_consequence_firings,training_trace_arrivals,training_trace_firings,training_local_returns,training_effects,training_distractor_firings,training_extra_arrival_firings,heldout_continuation_firings,heldout_consequence_firings,heldout_trace_arrivals,heldout_trace_firings,heldout_local_returns,heldout_effects,postgap_effects,heldout_extra_arrival_firings,postgap_extra_arrival_firings,training_quiescent,heldout_quiescent,postgap_quiescent,lifecycle_first_effects,lifecycle_first_extra_arrival_firings,lifecycle_first_quiescent,lifecycle_old_direction_live,lifecycle_old_direction_stale,lifecycle_old_correspondence_live,lifecycle_old_correspondence_stale,lifecycle_stale_effects,lifecycle_stale_extra_arrival_firings,lifecycle_stale_quiescent,lifecycle_fresh_correspondence,lifecycle_fresh_direction_ids,work,fingerprint,duplicate_exact,passed\n",
    );
    for row in rows {
        let value = &row.metrics;
        let fields = vec![
            row.stratum.to_string(),
            row.scenario.name().to_string(),
            pair_u32(value.correspondence_resistance),
            pair_usize(value.directional_live_paths),
            pair_u32(value.directional_min_resistance),
            pair_u32(value.directional_max_resistance),
            pair_usize(value.training_continuation_firings),
            pair_usize(value.training_consequence_firings),
            pair_usize(value.training_trace_arrivals),
            pair_usize(value.training_trace_firings),
            pair_usize(value.training_local_returns),
            pair_usize(value.training_effects),
            value.training_distractor_firings.to_string(),
            value.training_extra_arrival_firings.to_string(),
            pair_usize(value.heldout.continuation_firings),
            pair_usize(value.heldout.consequence_firings),
            pair_usize(value.heldout.trace_arrivals),
            pair_usize(value.heldout.trace_firings),
            pair_usize(value.heldout.local_returns),
            pair_usize(value.heldout.effects),
            pair_usize(value.postgap.effects),
            value.heldout.extra_arrival_firings.to_string(),
            value.postgap.extra_arrival_firings.to_string(),
            value.training_quiescent.to_string(),
            value.heldout.quiescent.to_string(),
            value.postgap.quiescent.to_string(),
            pair_usize(value.lifecycle_first_effects),
            value.lifecycle_first_extra_arrival_firings.to_string(),
            value.lifecycle_first_quiescent.to_string(),
            pair_usize(value.lifecycle_old_direction_live),
            value.lifecycle_old_direction_stale.to_string(),
            pair_usize(value.lifecycle_old_correspondence_live),
            value.lifecycle_old_correspondence_stale.to_string(),
            pair_usize(value.lifecycle_stale_effects),
            value.lifecycle_stale_extra_arrival_firings.to_string(),
            value.lifecycle_stale_quiescent.to_string(),
            pair_usize(value.lifecycle_fresh_correspondence),
            value.lifecycle_fresh_direction_ids.to_string(),
            value.work.total().to_string(),
            value.fingerprint.to_string(),
            row.duplicate_exact.to_string(),
            row.passed.to_string(),
        ];
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

#[allow(dead_code)]
fn markdown(rows: &[ResultRow]) -> String {
    let passed = rows.iter().all(|row| row.passed);
    let mut output = format!(
        "# PX2 physical causal direction trace-sufficiency GATE v1\n\nOutcome: **{}** (`{}/{}` cells).\n\n| stratum | world | traversal | consequence | trace fire/local return | live paths | resistance min/max | held-out effect | post-gap effect | distractor fire | lifecycle first/old-dir/old-corr/stale/fresh-corr/fresh-dir | source refire train/held/post | quiescent train/held/post | replay | pass |\n|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
        if passed { "POSITIVE" } else { "NEGATIVE" },
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
    );
    for row in rows {
        let value = &row.metrics;
        output.push_str(&format!(
            "| {} | {} | `{}` | `{}` | `{}/{}` | `{}` | `{}/{}` | `{}` | `{}` | {} | `{}/{}/{}/{}/{}/{}` | `{}/{}/{}` | `{}/{}/{}` | {} | {} |\n",
            row.stratum,
            row.scenario.name(),
            pair_usize(value.training_continuation_firings),
            pair_usize(value.training_consequence_firings),
            pair_usize(value.training_trace_firings),
            pair_usize(value.training_local_returns),
            pair_usize(value.directional_live_paths),
            pair_u32(value.directional_min_resistance),
            pair_u32(value.directional_max_resistance),
            pair_usize(value.heldout.effects),
            pair_usize(value.postgap.effects),
            value.training_distractor_firings,
            pair_usize(value.lifecycle_first_effects),
            pair_usize(value.lifecycle_old_direction_live),
            pair_usize(value.lifecycle_old_correspondence_live),
            pair_usize(value.lifecycle_stale_effects),
            pair_usize(value.lifecycle_fresh_correspondence),
            value.lifecycle_fresh_direction_ids,
            value.training_extra_arrival_firings,
            value.heldout.extra_arrival_firings,
            value.postgap.extra_arrival_firings,
            value.training_quiescent,
            value.heldout.quiescent,
            value.postgap.quiescent,
            row.duplicate_exact,
            row.passed,
        ));
    }
    output.push_str(
        "\nNo PX0/PX1 physics changed. PX2 remains non-authoritative; no definitive evidence was executed.\n",
    );
    output
}

fn write_new(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create PX2 GATE artifact");
    file.write_all(contents.as_bytes())
        .expect("write PX2 GATE artifact");
    file.sync_all().expect("sync PX2 GATE artifact");
}
