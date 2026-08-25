#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowSpec, CellId, CellSpec, ContentHash, LiveCheckpoint, MechanicalConfig,
    PhysicalTransition, PlasticSubstrate, QuiescentCheckpoint, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [8_900_000, 9_000_101];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    LivePending,
    QuiescentFuture,
}

impl Family {
    const ALL: [Self; 2] = [Self::LivePending, Self::QuiescentFuture];

    fn name(self) -> &'static str {
        match self {
            Self::LivePending => "live_pending_exact_continuation",
            Self::QuiescentFuture => "quiescent_exact_future",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PhysicalWork {
    drive: u64,
    modulation: u64,
    updates: u64,
    proposals: u64,
    cell_proposals: u64,
    arrow_deallocations: u64,
    cell_deallocations: u64,
    qlp: u64,
    total: u64,
}

impl From<Work> for PhysicalWork {
    fn from(work: Work) -> Self {
        Self {
            drive: work.drive_deliveries,
            modulation: work.modulatory_deliveries,
            updates: work.local_return_updates,
            proposals: work.local_structural_proposals,
            cell_proposals: work.local_cell_proposals,
            arrow_deallocations: work.physical_deallocations,
            cell_deallocations: work.cell_deallocations,
            qlp: work.qualified_local_traversals,
            total: work.physical_total(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Snapshot {
    trace: Vec<PhysicalTransition>,
    trace_hash: String,
    physical_work: PhysicalWork,
    legacy_total: u64,
    tick: i64,
    body_hash: String,
    checkpoint_hash: String,
    quiescent: bool,
}

fn snapshot(body: &mut PlasticSubstrate) -> Snapshot {
    let run = body.propagate();
    let checkpoint = body.live_checkpoint(1).unwrap();
    Snapshot {
        trace_hash: ContentHash::of(format!("{:?}", run.physical_trace).as_bytes()).to_string(),
        trace: run.physical_trace,
        physical_work: run.work.into(),
        legacy_total: run.work.total(),
        tick: body.clock().tick,
        body_hash: ContentHash::of(&body.canonical_body_bytes(1).unwrap()).to_string(),
        checkpoint_hash: ContentHash::of(&checkpoint.canonical_bytes().unwrap()).to_string(),
        quiescent: run.naturally_quiescent,
    }
}

fn snapshot_after(body: &mut PlasticSubstrate, input: SpikeInput) -> Snapshot {
    body.enter(input);
    snapshot(body)
}

struct World {
    body: PlasticSubstrate,
    root: u64,
    anchor_b: CellId,
}

impl World {
    fn new(root: u64, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 32, 64, mechanics);
        body.set_physical_tracing(true);
        let anchor_a = body.add_cell(CellSpec {
            physical_id: root + 1,
            position: -10_000,
            region: 0,
            threshold: 100,
            resistance: 500,
        });
        let anchor_b = body.add_cell(CellSpec {
            physical_id: root + 2,
            position: 10_000,
            region: 0,
            threshold: 100,
            resistance: 500,
        });
        for (from, to) in [(anchor_a, anchor_b), (anchor_b, anchor_a)] {
            body.add_arrow(ArrowSpec {
                from,
                to,
                delay: 1,
                phase: 0,
                coupling: 1,
                resistance: 500,
                mode: TransmissionMode::Drive,
            });
        }
        Self {
            body,
            root,
            anchor_b,
        }
    }

    fn target(&mut self) -> CellId {
        let target = self.body.add_cell(CellSpec {
            physical_id: self.root + 100,
            position: 0,
            region: 0,
            threshold: 1,
            resistance: 1,
        });
        self.body.add_arrow(ArrowSpec {
            from: self.anchor_b,
            to: target,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 500,
            mode: TransmissionMode::Drive,
        });
        target
    }

    fn input(&self, target: CellId, tick: i64) -> SpikeInput {
        SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: self.root + 90_000 + u64::try_from(tick).unwrap_or_default(),
            target,
            impulse: 1,
        }
    }
}

fn restore_live(
    checkpoint: LiveCheckpoint,
    mechanics: MechanicalConfig,
    tracing: bool,
) -> PlasticSubstrate {
    let bytes = checkpoint.canonical_bytes().unwrap();
    let decoded = LiveCheckpoint::decode(&bytes).unwrap();
    let mut body =
        PlasticSubstrate::from_live_checkpoint_with_mechanics(decoded, mechanics).unwrap();
    if tracing {
        body.set_physical_tracing(true);
    }
    body
}

fn restore_quiescent(
    checkpoint: QuiescentCheckpoint,
    mechanics: MechanicalConfig,
    tracing: bool,
) -> PlasticSubstrate {
    let bytes = checkpoint.canonical_bytes().unwrap();
    let decoded = QuiescentCheckpoint::decode(&bytes).unwrap();
    let mut body =
        PlasticSubstrate::from_quiescent_checkpoint_with_mechanics(decoded, mechanics).unwrap();
    if tracing {
        body.set_physical_tracing(true);
    }
    body
}

fn diagnose(
    family: Family,
    root: u64,
    mechanics: MechanicalConfig,
) -> (Snapshot, Snapshot, Snapshot) {
    let mut world = World::new(root, mechanics);
    let target = world.target();
    match family {
        Family::LivePending => {
            let input = world.input(target, 5);
            world.body.enter(input);
            let checkpoint = world.body.live_checkpoint(1).unwrap();
            let mut uninterrupted = world.body.clone();
            let mut restored_default = restore_live(checkpoint.clone(), mechanics, false);
            let mut restored_traced = restore_live(checkpoint, mechanics, true);
            (
                snapshot(&mut uninterrupted),
                snapshot(&mut restored_default),
                snapshot(&mut restored_traced),
            )
        }
        Family::QuiescentFuture => {
            let checkpoint = world.body.quiescent_checkpoint(1).unwrap();
            let input = world.input(target, 1);
            let mut uninterrupted = world.body.clone();
            let mut restored_default = restore_quiescent(checkpoint.clone(), mechanics, false);
            let mut restored_traced = restore_quiescent(checkpoint, mechanics, true);
            (
                snapshot_after(&mut uninterrupted, input),
                snapshot_after(&mut restored_default, input),
                snapshot_after(&mut restored_traced, input),
            )
        }
    }
}

fn mechanics_name(mechanics: MechanicalConfig) -> &'static str {
    if mechanics == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn main() {
    let output_dir = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/ck0_continuation_negative_diagnostic_v1")
    });
    fs::create_dir_all(&output_dir).unwrap();
    let mut csv = String::from(
        "case,family,root,mechanics,default_trace_equal,traced_trace_equal,default_physical_work_equal,traced_physical_work_equal,default_legacy_total_equal,traced_legacy_total_equal,default_tick_equal,traced_tick_equal,default_body_equal,traced_body_equal,default_checkpoint_hash_equal,traced_checkpoint_hash_equal,default_quiescence_equal,traced_quiescence_equal,expected_trace_hash,default_trace_hash,traced_trace_hash,expected_physical_work,default_physical_work,traced_physical_work,expected_legacy_total,default_legacy_total,traced_legacy_total,expected_tick,default_tick,traced_tick,expected_body_hash,default_body_hash,traced_body_hash,expected_checkpoint_hash,default_checkpoint_hash,traced_checkpoint_hash\n",
    );
    let mut cases = 0usize;
    let mut all_physics_exact = true;
    let mut all_traced_observations_exact = true;
    let mut any_default_trace_missing = false;
    let mut any_legacy_total_difference = false;
    let mut any_raw_checkpoint_difference = false;
    for root in ROOTS {
        for family in Family::ALL {
            for mechanics in [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION] {
                cases += 1;
                let (expected, default, traced) = diagnose(family, root, mechanics);
                let default_trace_equal = default.trace == expected.trace;
                let traced_trace_equal = traced.trace == expected.trace;
                let default_physical_work_equal = default.physical_work == expected.physical_work;
                let traced_physical_work_equal = traced.physical_work == expected.physical_work;
                let default_legacy_total_equal = default.legacy_total == expected.legacy_total;
                let traced_legacy_total_equal = traced.legacy_total == expected.legacy_total;
                let default_tick_equal = default.tick == expected.tick;
                let traced_tick_equal = traced.tick == expected.tick;
                let default_body_equal = default.body_hash == expected.body_hash;
                let traced_body_equal = traced.body_hash == expected.body_hash;
                let default_checkpoint_hash_equal =
                    default.checkpoint_hash == expected.checkpoint_hash;
                let traced_checkpoint_hash_equal =
                    traced.checkpoint_hash == expected.checkpoint_hash;
                let default_quiescence_equal = default.quiescent == expected.quiescent;
                let traced_quiescence_equal = traced.quiescent == expected.quiescent;
                let physics_exact = traced_trace_equal
                    && traced_physical_work_equal
                    && traced_tick_equal
                    && traced_body_equal
                    && traced_quiescence_equal;
                all_physics_exact &= physics_exact;
                all_traced_observations_exact &= traced_trace_equal;
                any_default_trace_missing |= !default_trace_equal && traced_trace_equal;
                any_legacy_total_difference |=
                    !default_legacy_total_equal || !traced_legacy_total_equal;
                any_raw_checkpoint_difference |=
                    !default_checkpoint_hash_equal || !traced_checkpoint_hash_equal;
                writeln!(
                    csv,
                    "{cases},{},{root},{},{default_trace_equal},{traced_trace_equal},{default_physical_work_equal},{traced_physical_work_equal},{default_legacy_total_equal},{traced_legacy_total_equal},{default_tick_equal},{traced_tick_equal},{default_body_equal},{traced_body_equal},{default_checkpoint_hash_equal},{traced_checkpoint_hash_equal},{default_quiescence_equal},{traced_quiescence_equal},{},{},{},{:?},{:?},{:?},{},{},{},{},{},{},{},{},{},{},{},{}",
                    family.name(),
                    mechanics_name(mechanics),
                    expected.trace_hash,
                    default.trace_hash,
                    traced.trace_hash,
                    expected.physical_work,
                    default.physical_work,
                    traced.physical_work,
                    expected.legacy_total,
                    default.legacy_total,
                    traced.legacy_total,
                    expected.tick,
                    default.tick,
                    traced.tick,
                    expected.body_hash,
                    default.body_hash,
                    traced.body_hash,
                    expected.checkpoint_hash,
                    default.checkpoint_hash,
                    traced.checkpoint_hash,
                )
                .unwrap();
            }
        }
    }
    assert_eq!(cases, 8);
    let classification = if all_physics_exact && all_traced_observations_exact {
        "evaluator_observer_defect"
    } else {
        "runtime_checkpoint_negative"
    };
    let report = format!(
        "# CK0 continuation negative diagnostic v1\n\n- cases: {cases}/8\n- classification: {classification}\n- observer-enabled physical continuation exact: {all_physics_exact}\n- observer-enabled traces exact: {all_traced_observations_exact}\n- default restored observer omitted trace: {any_default_trace_missing}\n- legacy Work total differed: {any_legacy_total_difference}\n- raw checkpoint hash differed: {any_raw_checkpoint_difference}\n",
    );
    fs::write(output_dir.join("matrix.csv"), csv).unwrap();
    fs::write(output_dir.join("report.md"), report).unwrap();
    assert!(
        all_physics_exact,
        "CK0 diagnostic found physical divergence"
    );
    println!("CK0_CONTINUATION_NEGATIVE_DIAGNOSTIC_COMPLETE_V1");
}
