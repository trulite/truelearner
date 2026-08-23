use px0_physical_correspondence::{
    ArrowSpec, CellId, CellSpec, PlasticSubstrate, SpikeInput, TraceEntry,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX2_COMMIT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const PX0_SHA: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX2_SHA: &str = "921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18";
const NOT1_PROBE_SHA: &str = "4f3ad19bea689a60641852ef038e7ba5d8938e8dcdba802f0019dea8df68dedb";
const NOT2_PROBE_SHA: &str = "07cb0d4ccbd817c6de56166f89d4e5719a4d645bfab9d78718169538d36cad7d";
const NOT1_PROTOCOL_SHA: &str = "35b2140b6422a0769581c06c43deb921ec2a8d6d22e127bdabe6638468f329ba";
const NOT2_PROTOCOL_SHA: &str = "376afddf1d307fe1de645bc07782d469cea1b3b4145869a31dd368c7eb879f23";

const NOT1_CSV: &str = "results/cj0_not1_active_inhibition_definitive_v1.csv";
const NOT1_MD: &str = "results/cj0_not1_active_inhibition_definitive_v1.md";
const NOT1_STAGE_CSV: &str = "results/.cj0_not1_active_inhibition_definitive_v1.csv.staging";
const NOT1_STAGE_MD: &str = "results/.cj0_not1_active_inhibition_definitive_v1.md.staging";
const NOT2_CSV: &str = "results/cj0_not2_temporal_absence_definitive_v1.csv";
const NOT2_MD: &str = "results/cj0_not2_temporal_absence_definitive_v1.md";
const NOT2_STAGE_CSV: &str = "results/.cj0_not2_temporal_absence_definitive_v1.csv.staging";
const NOT2_STAGE_MD: &str = "results/.cj0_not2_temporal_absence_definitive_v1.md.staging";

const SEEDS: usize = 16;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [arg] if arg == "--preflight-not1" => preflight(Track::Not1),
        [arg] if arg == "--preflight-not2" => preflight(Track::Not2),
        [arg] if arg == "--definitive-not1" => run_not1(),
        [arg] if arg == "--definitive-not2" => run_not2(),
        _ => {
            eprintln!("CJ0 NOT definitive requires an explicit track preflight or definitive flag");
            std::process::exit(2);
        }
    }
}

#[derive(Clone, Copy)]
enum Track {
    Not1,
    Not2,
}

fn preflight(track: Track) {
    assert!(frozen_inputs_exact(), "frozen inputs must remain exact");
    assert_outputs_absent(track);
    match track {
        Track::Not1 => println!("CJ0_NOT1_ACTIVE_INHIBITION_DEFINITIVE_V1_PREFLIGHT_OK"),
        Track::Not2 => println!("CJ0_NOT2_TEMPORAL_ABSENCE_DEFINITIVE_V1_PREFLIGHT_OK"),
    }
}

fn frozen_inputs_exact() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == PX0_SHA
        && sha256("results/px2_physical_causal_direction_definitive.csv") == PX2_SHA
        && sha256("results/cj0_not1_active_inhibition_probe_v1.csv") == NOT1_PROBE_SHA
        && sha256("results/cj0_not2_temporal_absence_probe_v1.csv") == NOT2_PROBE_SHA
        && sha256("experiments/cj0_not1_active_inhibition_definitive_protocol.md")
            == NOT1_PROTOCOL_SHA
        && sha256("experiments/cj0_not2_temporal_absence_definitive_protocol.md")
            == NOT2_PROTOCOL_SHA
        && git_output(&["rev-parse", "2fbee861^{commit}"]) == PX2_COMMIT
        && git_output(&[
            "diff",
            "--name-only",
            PX2_COMMIT,
            "--",
            "crates/px0-physical-correspondence/src/lib.rs",
            "results/px0_physical_correspondence_definitive.csv",
            "results/px1_physical_boundary_roles_definitive.csv",
            "results/px2_physical_causal_direction_definitive.csv",
        ])
        .is_empty()
}

fn assert_outputs_absent(track: Track) {
    let paths = match track {
        Track::Not1 => [NOT1_CSV, NOT1_MD, NOT1_STAGE_CSV, NOT1_STAGE_MD],
        Track::Not2 => [NOT2_CSV, NOT2_MD, NOT2_STAGE_CSV, NOT2_STAGE_MD],
    };
    for path in paths {
        assert!(
            !Path::new(path).exists(),
            "evidence path already exists: {path}"
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Layout {
    mirror: bool,
    reverse_cells: bool,
    reverse_arrows: bool,
    spacing: i32,
    external_phase: i32,
}

fn layout(seed: usize) -> Layout {
    Layout {
        mirror: seed & 1 != 0,
        reverse_cells: seed & 2 != 0,
        reverse_arrows: seed & 4 != 0,
        spacing: if seed & 8 == 0 { 10 } else { 14 },
        external_phase: i32::try_from(seed % 3).expect("bounded phase") - 1,
    }
}

fn cell_spec(
    namespace: u64,
    layout: Layout,
    offset: u64,
    ordinal: i32,
    region: i16,
    threshold: i32,
) -> CellSpec {
    let sign = if layout.mirror { -1 } else { 1 };
    CellSpec {
        physical_id: namespace + offset,
        position: ordinal * layout.spacing * sign,
        region,
        threshold,
        resistance: 100,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Not1World {
    Absent,
    Early,
    CoincidentBefore,
    CoincidentAfter,
    Late,
    Blocked,
    Stale,
}

impl Not1World {
    const ALL: [Self; 7] = [
        Self::Absent,
        Self::Early,
        Self::CoincidentBefore,
        Self::CoincidentAfter,
        Self::Late,
        Self::Blocked,
        Self::Stale,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Absent => "a-absent",
            Self::Early => "a-one-tick-early",
            Self::CoincidentBefore => "a-coincident-before-b",
            Self::CoincidentAfter => "a-coincident-after-b",
            Self::Late => "a-one-tick-late",
            Self::Blocked => "a-blocked",
            Self::Stale => "a-stale",
        }
    }
}

#[derive(Clone)]
struct Not1Fixture {
    substrate: PlasticSubstrate,
    a: CellId,
    b: CellId,
    a_physical: u64,
    b_physical: u64,
    integration_physical: u64,
    output_physical: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Not1Observation {
    a_firings: usize,
    b_firings: usize,
    integration_firings: usize,
    output_firings: usize,
    positive_tick: i64,
    negative_arrivals: usize,
    negative_tick: i64,
    output_tick: i64,
    crossings: usize,
    deallocations: u64,
    quiescent: bool,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    work: u64,
    storage: usize,
}

#[derive(Clone, Debug)]
struct Not1Row {
    seed: usize,
    layout: Layout,
    world: Not1World,
    namespace: u64,
    observation: Not1Observation,
    duplicate_exact: bool,
    passed: bool,
}

fn build_not1(namespace: u64, layout: Layout, world: Not1World) -> Not1Fixture {
    let mut substrate = PlasticSubstrate::new();
    let add = |substrate: &mut PlasticSubstrate, offset, ordinal, region, threshold| {
        substrate.add_cell(cell_spec(
            namespace,
            namespace_layout(layout),
            offset,
            ordinal,
            region,
            threshold,
        ))
    };
    let (a, b, integration, output) = if layout.reverse_cells {
        let output = add(&mut substrate, 40, 3, 2, 1);
        let integration = add(&mut substrate, 30, 2, 1, 2);
        let b = add(&mut substrate, 20, 1, 0, 1);
        let a = add(&mut substrate, 10, 0, 0, 1);
        (a, b, integration, output)
    } else {
        let a = add(&mut substrate, 10, 0, 0, 1);
        let b = add(&mut substrate, 20, 1, 0, 1);
        let integration = add(&mut substrate, 30, 2, 1, 2);
        let output = add(&mut substrate, 40, 3, 2, 1);
        (a, b, integration, output)
    };
    let (a_delay, a_phase, a_resistance) = match world {
        Not1World::Early => (0, 0, 100),
        Not1World::CoincidentBefore => (1, -1, 100),
        Not1World::CoincidentAfter => (1, 1, 100),
        Not1World::Late => (2, 0, 100),
        Not1World::Blocked => (1, -1, 0),
        Not1World::Stale => (11, 0, 1),
        Not1World::Absent => (1, -1, 100),
    };
    let a_arrow = ArrowSpec {
        from: a,
        to: integration,
        delay: a_delay,
        phase: a_phase,
        coupling: -2,
        resistance: a_resistance,
    };
    let b_arrow = ArrowSpec {
        from: b,
        to: integration,
        delay: 1,
        phase: if world == Not1World::CoincidentAfter {
            -1
        } else {
            1
        },
        coupling: 2,
        resistance: 100,
    };
    let output_arrow = ArrowSpec {
        from: integration,
        to: output,
        delay: 0,
        phase: 2,
        coupling: 1,
        resistance: 100,
    };
    if layout.reverse_arrows {
        substrate.add_arrow(output_arrow);
        substrate.add_arrow(b_arrow);
        substrate.add_arrow(a_arrow);
    } else {
        substrate.add_arrow(a_arrow);
        substrate.add_arrow(b_arrow);
        substrate.add_arrow(output_arrow);
    }
    Not1Fixture {
        substrate,
        a,
        b,
        a_physical: namespace + 10,
        b_physical: namespace + 20,
        integration_physical: namespace + 30,
        output_physical: namespace + 40,
    }
}

fn namespace_layout(layout: Layout) -> Layout {
    layout
}

fn execute_not1(mut fixture: Not1Fixture, layout: Layout, world: Not1World) -> Not1Observation {
    if world != Not1World::Absent {
        fixture.substrate.enter(SpikeInput {
            arrival_tick: 0,
            phase: layout.external_phase,
            origin_physical: fixture.a_physical + 0x10_000,
            target: fixture.a,
            impulse: 1,
        });
    }
    fixture.substrate.enter(SpikeInput {
        arrival_tick: 0,
        phase: -layout.external_phase,
        origin_physical: fixture.b_physical + 0x20_000,
        target: fixture.b,
        impulse: 1,
    });
    let run = fixture.substrate.propagate();
    let count = |physical| firing_count(&run.trace, physical);
    let positive_tick = first_tick(&run.trace, fixture.integration_physical, |impulse| {
        impulse > 0
    });
    let negative_tick = first_tick(&run.trace, fixture.integration_physical, |impulse| {
        impulse < 0
    });
    Not1Observation {
        a_firings: count(fixture.a_physical),
        b_firings: count(fixture.b_physical),
        integration_firings: count(fixture.integration_physical),
        output_firings: count(fixture.output_physical),
        positive_tick,
        negative_arrivals: run
            .trace
            .iter()
            .filter(|entry| {
                entry.target_physical == fixture.integration_physical && entry.impulse < 0
            })
            .count(),
        negative_tick,
        output_tick: first_firing_tick(&run.trace, fixture.output_physical),
        crossings: run.crossings.len(),
        deallocations: run.work.physical_deallocations,
        quiescent: run.naturally_quiescent,
        complete_fingerprint: fixture.substrate.complete_fingerprint(),
        permanent_fingerprint: fixture.substrate.permanent_fingerprint(),
        work: run.work.total(),
        storage: fixture.substrate.persistent_bytes(),
    }
}

fn classify_not1(world: Not1World, observation: &Not1Observation) -> bool {
    let common = observation.b_firings == 1
        && observation.positive_tick == 1
        && observation.quiescent
        && (1..1_000).contains(&observation.work)
        && observation.storage == 384;
    common
        && match world {
            Not1World::Absent => {
                observation.a_firings == 0
                    && observation.negative_arrivals == 0
                    && observation.integration_firings == 1
                    && observation.output_firings == 1
            }
            Not1World::Early | Not1World::CoincidentBefore => {
                observation.a_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.negative_tick <= observation.positive_tick
                    && observation.integration_firings == 0
                    && observation.output_firings == 0
            }
            Not1World::CoincidentAfter => {
                observation.a_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.negative_tick == observation.positive_tick
                    && observation.integration_firings == 1
                    && observation.output_firings == 1
                    && observation.output_tick == observation.positive_tick
            }
            Not1World::Late => {
                observation.a_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.output_firings == 1
                    && observation.output_tick < observation.negative_tick
            }
            Not1World::Blocked => {
                observation.a_firings == 1
                    && observation.negative_arrivals == 0
                    && observation.output_firings == 1
            }
            Not1World::Stale => {
                observation.a_firings == 1
                    && observation.negative_arrivals == 0
                    && observation.output_firings == 1
                    && observation.deallocations >= 1
            }
        }
}

fn run_not1() {
    assert!(frozen_inputs_exact(), "frozen inputs must remain exact");
    assert_outputs_absent(Track::Not1);
    eprintln!("CJ0_NOT1_ACTIVE_INHIBITION_DEFINITIVE_V1_EVIDENCE_SPENT");
    let mut rows = Vec::new();
    for seed in 0..SEEDS {
        let layout = layout(seed);
        for (world_index, world) in Not1World::ALL.into_iter().enumerate() {
            let namespace = 0x6_3100_0000
                + u64::try_from(seed).expect("seed") * 0x0100_0000
                + u64::try_from(world_index).expect("world") * 0x0010_0000;
            let fixture = build_not1(namespace, layout, world);
            let first = execute_not1(fixture.clone(), layout, world);
            let duplicate = execute_not1(fixture, layout, world);
            let duplicate_exact = first == duplicate;
            let passed = classify_not1(world, &first) && duplicate_exact;
            rows.push(Not1Row {
                seed,
                layout,
                world,
                namespace,
                observation: first,
                duplicate_exact,
                passed,
            });
        }
    }
    let passed = rows.len() == 112 && rows.iter().all(|row| row.passed);
    publish_not1(&rows, passed);
    if !passed {
        std::process::exit(1);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Not2World {
    Absent,
    InWindow,
    AtClosureBefore,
    AtClosureAfter,
    AfterClosure,
    Blocked,
    Stale,
}

impl Not2World {
    const ALL: [Self; 7] = [
        Self::Absent,
        Self::InWindow,
        Self::AtClosureBefore,
        Self::AtClosureAfter,
        Self::AfterClosure,
        Self::Blocked,
        Self::Stale,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Absent => "b-absent-through-closure",
            Self::InWindow => "b-tick-1-after-trigger",
            Self::AtClosureBefore => "b-tick-2-before-closure",
            Self::AtClosureAfter => "b-tick-2-after-closure",
            Self::AfterClosure => "b-tick-3-after-output",
            Self::Blocked => "b-blocked",
            Self::Stale => "b-stale",
        }
    }
}

#[derive(Clone)]
struct Not2Fixture {
    substrate: PlasticSubstrate,
    trigger: CellId,
    b: CellId,
    closure: CellId,
    trigger_physical: u64,
    b_physical: u64,
    closure_physical: u64,
    transient_physical: u64,
    output_physical: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Not2Observation {
    initial_fingerprint: u64,
    after_trigger_fingerprint: u64,
    trigger_firings: usize,
    b_firings: usize,
    closure_firings: usize,
    transient_firings: usize,
    output_firings: usize,
    positive_arrivals: usize,
    negative_arrivals: usize,
    negative_tick: i64,
    closure_tick: i64,
    output_tick: i64,
    trigger_quiescent: bool,
    final_quiescent: bool,
    deallocations: u64,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    work: u64,
    storage: usize,
}

#[derive(Clone, Debug)]
struct Not2Row {
    seed: usize,
    layout: Layout,
    world: Not2World,
    namespace: u64,
    observation: Not2Observation,
    duplicate_exact: bool,
    passed: bool,
}

fn build_not2(namespace: u64, layout: Layout, world: Not2World) -> Not2Fixture {
    let mut substrate = PlasticSubstrate::new();
    let add = |substrate: &mut PlasticSubstrate, offset, ordinal, region, threshold| {
        substrate.add_cell(cell_spec(
            namespace,
            namespace_layout(layout),
            offset,
            ordinal,
            region,
            threshold,
        ))
    };
    let (trigger, b, closure, transient, output) = if layout.reverse_cells {
        let output = add(&mut substrate, 50, 4, 2, 1);
        let transient = add(&mut substrate, 40, 3, 1, 3);
        let closure = add(&mut substrate, 30, 2, 0, 1);
        let b = add(&mut substrate, 20, 1, 0, 1);
        let trigger = add(&mut substrate, 10, 0, 0, 1);
        (trigger, b, closure, transient, output)
    } else {
        let trigger = add(&mut substrate, 10, 0, 0, 1);
        let b = add(&mut substrate, 20, 1, 0, 1);
        let closure = add(&mut substrate, 30, 2, 0, 1);
        let transient = add(&mut substrate, 40, 3, 1, 3);
        let output = add(&mut substrate, 50, 4, 2, 1);
        (trigger, b, closure, transient, output)
    };
    let trigger_arrow = ArrowSpec {
        from: trigger,
        to: transient,
        delay: 1,
        phase: 0,
        coupling: 2,
        resistance: 100,
    };
    let (b_delay, b_phase, b_resistance) = match world {
        Not2World::InWindow => (0, 0, 100),
        Not2World::AtClosureBefore => (1, -1, 100),
        Not2World::AtClosureAfter => (1, 2, 100),
        Not2World::AfterClosure => (2, 0, 100),
        Not2World::Blocked => (0, 0, 0),
        Not2World::Stale => (10, 0, 1),
        Not2World::Absent => (0, 0, 100),
    };
    let b_arrow = ArrowSpec {
        from: b,
        to: transient,
        delay: b_delay,
        phase: b_phase,
        coupling: -2,
        resistance: b_resistance,
    };
    let closure_arrow = ArrowSpec {
        from: closure,
        to: transient,
        delay: 1,
        phase: if world == Not2World::AtClosureAfter {
            -1
        } else {
            1
        },
        coupling: 2,
        resistance: 100,
    };
    let output_arrow = ArrowSpec {
        from: transient,
        to: output,
        delay: 0,
        phase: 3,
        coupling: 1,
        resistance: 100,
    };
    if layout.reverse_arrows {
        substrate.add_arrow(output_arrow);
        substrate.add_arrow(closure_arrow);
        substrate.add_arrow(b_arrow);
        substrate.add_arrow(trigger_arrow);
    } else {
        substrate.add_arrow(trigger_arrow);
        substrate.add_arrow(b_arrow);
        substrate.add_arrow(closure_arrow);
        substrate.add_arrow(output_arrow);
    }
    Not2Fixture {
        substrate,
        trigger,
        b,
        closure,
        trigger_physical: namespace + 10,
        b_physical: namespace + 20,
        closure_physical: namespace + 30,
        transient_physical: namespace + 40,
        output_physical: namespace + 50,
    }
}

fn execute_not2(mut fixture: Not2Fixture, layout: Layout, world: Not2World) -> Not2Observation {
    let initial_fingerprint = fixture.substrate.complete_fingerprint();
    fixture.substrate.enter(SpikeInput {
        arrival_tick: 0,
        phase: layout.external_phase,
        origin_physical: fixture.trigger_physical + 0x10_000,
        target: fixture.trigger,
        impulse: 1,
    });
    let trigger_run = fixture.substrate.propagate();
    let after_trigger_fingerprint = fixture.substrate.complete_fingerprint();
    fixture.substrate.enter(SpikeInput {
        arrival_tick: 1,
        phase: layout.external_phase,
        origin_physical: fixture.closure_physical + 0x30_000,
        target: fixture.closure,
        impulse: 1,
    });
    if world != Not2World::Absent {
        fixture.substrate.enter(SpikeInput {
            arrival_tick: 1,
            phase: -layout.external_phase,
            origin_physical: fixture.b_physical + 0x20_000,
            target: fixture.b,
            impulse: 1,
        });
    }
    let final_run = fixture.substrate.propagate();
    let traces = trigger_run
        .trace
        .iter()
        .chain(&final_run.trace)
        .cloned()
        .collect::<Vec<_>>();
    let transient = traces
        .iter()
        .filter(|entry| entry.target_physical == fixture.transient_physical)
        .collect::<Vec<_>>();
    Not2Observation {
        initial_fingerprint,
        after_trigger_fingerprint,
        trigger_firings: firing_count(&traces, fixture.trigger_physical),
        b_firings: firing_count(&traces, fixture.b_physical),
        closure_firings: firing_count(&traces, fixture.closure_physical),
        transient_firings: firing_count(&traces, fixture.transient_physical),
        output_firings: firing_count(&traces, fixture.output_physical),
        positive_arrivals: transient.iter().filter(|entry| entry.impulse > 0).count(),
        negative_arrivals: transient.iter().filter(|entry| entry.impulse < 0).count(),
        negative_tick: transient
            .iter()
            .find(|entry| entry.impulse < 0)
            .map_or(-1, |entry| entry.tick),
        closure_tick: final_run
            .trace
            .iter()
            .find(|entry| entry.target_physical == fixture.transient_physical && entry.impulse > 0)
            .map_or(-1, |entry| entry.tick),
        output_tick: first_firing_tick(&traces, fixture.output_physical),
        trigger_quiescent: trigger_run.naturally_quiescent,
        final_quiescent: final_run.naturally_quiescent,
        deallocations: trigger_run.work.physical_deallocations
            + final_run.work.physical_deallocations,
        complete_fingerprint: fixture.substrate.complete_fingerprint(),
        permanent_fingerprint: fixture.substrate.permanent_fingerprint(),
        work: trigger_run.work.total() + final_run.work.total(),
        storage: fixture.substrate.persistent_bytes(),
    }
}

fn classify_not2(world: Not2World, observation: &Not2Observation) -> bool {
    let common = observation.initial_fingerprint != observation.after_trigger_fingerprint
        && observation.trigger_firings == 1
        && observation.closure_firings == 1
        && observation.positive_arrivals == 2
        && observation.closure_tick == 2
        && observation.trigger_quiescent
        && observation.final_quiescent
        && (1..1_000).contains(&observation.work)
        && observation.storage == 496;
    common
        && match world {
            Not2World::Absent => {
                observation.b_firings == 0
                    && observation.negative_arrivals == 0
                    && observation.transient_firings == 1
                    && observation.output_firings == 1
            }
            Not2World::InWindow | Not2World::AtClosureBefore => {
                observation.b_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.negative_tick <= observation.closure_tick
                    && observation.transient_firings == 0
                    && observation.output_firings == 0
            }
            Not2World::AtClosureAfter => {
                observation.b_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.negative_tick == observation.closure_tick
                    && observation.transient_firings == 1
                    && observation.output_firings == 1
                    && observation.output_tick == observation.closure_tick
            }
            Not2World::AfterClosure => {
                observation.b_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.output_firings == 1
                    && observation.output_tick < observation.negative_tick
            }
            Not2World::Blocked => {
                observation.b_firings == 1
                    && observation.negative_arrivals == 0
                    && observation.output_firings == 1
            }
            Not2World::Stale => {
                observation.b_firings == 1
                    && observation.negative_arrivals == 0
                    && observation.output_firings == 1
                    && observation.deallocations >= 1
            }
        }
}

fn run_not2() {
    assert!(frozen_inputs_exact(), "frozen inputs must remain exact");
    assert_outputs_absent(Track::Not2);
    eprintln!("CJ0_NOT2_TEMPORAL_ABSENCE_DEFINITIVE_V1_EVIDENCE_SPENT");
    let mut rows = Vec::new();
    for seed in 0..SEEDS {
        let layout = layout(seed);
        for (world_index, world) in Not2World::ALL.into_iter().enumerate() {
            let namespace = 0x6_4100_0000
                + u64::try_from(seed).expect("seed") * 0x0100_0000
                + u64::try_from(world_index).expect("world") * 0x0010_0000;
            let fixture = build_not2(namespace, layout, world);
            let first = execute_not2(fixture.clone(), layout, world);
            let duplicate = execute_not2(fixture, layout, world);
            let duplicate_exact = first == duplicate;
            let passed = classify_not2(world, &first) && duplicate_exact;
            rows.push(Not2Row {
                seed,
                layout,
                world,
                namespace,
                observation: first,
                duplicate_exact,
                passed,
            });
        }
    }
    let passed = rows.len() == 112 && rows.iter().all(|row| row.passed);
    publish_not2(&rows, passed);
    if !passed {
        std::process::exit(1);
    }
}

fn firing_count(trace: &[TraceEntry], physical: u64) -> usize {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn first_firing_tick(trace: &[TraceEntry], physical: u64) -> i64 {
    trace
        .iter()
        .find(|entry| entry.target_physical == physical && entry.fired)
        .map_or(-1, |entry| entry.tick)
}

fn first_tick(trace: &[TraceEntry], physical: u64, predicate: impl Fn(i32) -> bool) -> i64 {
    trace
        .iter()
        .find(|entry| entry.target_physical == physical && predicate(entry.impulse))
        .map_or(-1, |entry| entry.tick)
}

fn publish_not1(rows: &[Not1Row], passed: bool) {
    let mut csv = String::from(
        "seed,mirror,reverse_cells,reverse_arrows,spacing,world,namespace,a_firings,b_firings,integration_firings,output_firings,positive_tick,negative_arrivals,negative_tick,output_tick,crossings,deallocations,quiescent,complete_fingerprint,permanent_fingerprint,work,storage,duplicate_exact,passed\n",
    );
    for row in rows {
        let o = &row.observation;
        csv.push_str(&format!(
            "{},{},{},{},{},{},{:#x},{},{},{},{},{},{},{},{},{},{},{},{:#x},{:#x},{},{},{},{}\n",
            row.seed,
            row.layout.mirror,
            row.layout.reverse_cells,
            row.layout.reverse_arrows,
            row.layout.spacing,
            row.world.name(),
            row.namespace,
            o.a_firings,
            o.b_firings,
            o.integration_firings,
            o.output_firings,
            o.positive_tick,
            o.negative_arrivals,
            o.negative_tick,
            o.output_tick,
            o.crossings,
            o.deallocations,
            o.quiescent,
            o.complete_fingerprint,
            o.permanent_fingerprint,
            o.work,
            o.storage,
            row.duplicate_exact,
            row.passed,
        ));
    }
    let report = format!(
        "# CJ0-NOT-1 active-inhibition definitive v1\n\nClassification: **{}**. Rows: `{}/{}`.\n\nAll seven timing/lifecycle worlds, sixteen fresh layout strata, signed arrivals, output timing, pressure, quiescence, replay, work, storage, and fingerprints are serialized independently in the CSV.\n\nThe classification concerns existing ordinary signed coupling only. No logical NOT primitive was added and no authority was advanced.\n",
        if passed { "POSITIVE" } else { "NEGATIVE" },
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
    );
    atomic_publish(NOT1_STAGE_CSV, NOT1_CSV, csv.as_bytes());
    atomic_publish(NOT1_STAGE_MD, NOT1_MD, report.as_bytes());
}

fn publish_not2(rows: &[Not2Row], passed: bool) {
    let mut csv = String::from(
        "seed,mirror,reverse_cells,reverse_arrows,spacing,world,namespace,initial_fingerprint,after_trigger_fingerprint,trigger_firings,b_firings,closure_firings,transient_firings,output_firings,positive_arrivals,negative_arrivals,negative_tick,closure_tick,output_tick,trigger_quiescent,final_quiescent,deallocations,complete_fingerprint,permanent_fingerprint,work,storage,duplicate_exact,passed\n",
    );
    for row in rows {
        let o = &row.observation;
        csv.push_str(&format!(
            "{},{},{},{},{},{},{:#x},{:#x},{:#x},{},{},{},{},{},{},{},{},{},{},{},{},{},{:#x},{:#x},{},{},{},{}\n",
            row.seed,
            row.layout.mirror,
            row.layout.reverse_cells,
            row.layout.reverse_arrows,
            row.layout.spacing,
            row.world.name(),
            row.namespace,
            o.initial_fingerprint,
            o.after_trigger_fingerprint,
            o.trigger_firings,
            o.b_firings,
            o.closure_firings,
            o.transient_firings,
            o.output_firings,
            o.positive_arrivals,
            o.negative_arrivals,
            o.negative_tick,
            o.closure_tick,
            o.output_tick,
            o.trigger_quiescent,
            o.final_quiescent,
            o.deallocations,
            o.complete_fingerprint,
            o.permanent_fingerprint,
            o.work,
            o.storage,
            row.duplicate_exact,
            row.passed,
        ));
    }
    let report = format!(
        "# CJ0-NOT-2 temporal-absence definitive v1\n\nClassification: **{}**. Rows: `{}/{}`.\n\nAll seven closure/timing/lifecycle worlds, sixteen fresh layout strata, trigger-state fingerprints, signed arrivals, output timing, pressure, two-stage quiescence, replay, work, storage, and fingerprints are serialized independently in the CSV.\n\nNo absence symbol, timeout label, evaluator-selected branch, or new persistent variable was added, and no authority was advanced.\n",
        if passed { "POSITIVE" } else { "NEGATIVE" },
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
    );
    atomic_publish(NOT2_STAGE_CSV, NOT2_CSV, csv.as_bytes());
    atomic_publish(NOT2_STAGE_MD, NOT2_MD, report.as_bytes());
}

fn atomic_publish(staging: &str, final_path: &str, bytes: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(staging)
        .unwrap_or_else(|error| panic!("create {staging}: {error}"));
    file.write_all(bytes)
        .unwrap_or_else(|error| panic!("write {staging}: {error}"));
    file.sync_all()
        .unwrap_or_else(|error| panic!("sync {staging}: {error}"));
    rename(staging, final_path).unwrap_or_else(|error| panic!("publish {final_path}: {error}"));
}

fn sha256(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("run sha256sum");
    assert!(output.status.success(), "sha256sum failed for {path}");
    String::from_utf8(output.stdout)
        .expect("sha256 output")
        .split_whitespace()
        .next()
        .expect("sha256 digest")
        .to_string()
}

fn git_output(args: &[&str]) -> String {
    let output = Command::new("git").args(args).output().expect("run git");
    assert!(output.status.success(), "git command failed: {args:?}");
    String::from_utf8(output.stdout)
        .expect("git output")
        .trim()
        .to_string()
}
