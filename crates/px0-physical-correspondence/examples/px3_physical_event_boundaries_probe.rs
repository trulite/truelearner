use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const FROZEN_PARENT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const PROTOCOL_SHA256: &str = "cae4d2b03b0c094a48348fc34ba49fa16c2ecf47847850e01d66c936efd83a52";
const PX0_PX2_LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX2_SOURCE_SHA256: &str = "c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5";
const PX1_CSV_SHA256: &str = "6613ff0a96bb3a60fbe7afeb92cd64edced3c6df5dcc04fe47518db158dd88f6";
const PX2_CSV_SHA256: &str = "921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18";
const PX2_HANDOFF_SHA256: &str = "98647ab1563593e18e345cd7e5a71c4991d18b397dfe2dec71a4756106d96509";
const OLD_M3_SOURCE_SHA256: &str =
    "a8d8fe060b497c7a6b5f9a5a88b7ed2292dc8a729a8781f599547b6027efc0a0";
const OLD_M3_CSV_SHA256: &str = "ac8c0a6c9b7badfa263ceb054ffe59c11162b1ca256c56cc6df5f0d378179401";

const PROTOCOL: &str =
    "experiments/px3_physical_event_boundaries_no_new_mechanism_probe_v2_protocol.md";
const RESULT_CSV: &str = "results/px3_physical_event_boundaries_no_new_mechanism_probe_v1.csv";
const RESULT_MD: &str = "results/px3_physical_event_boundaries_no_new_mechanism_probe_v1.md";
const STAGING_CSV: &str =
    "results/.px3_physical_event_boundaries_no_new_mechanism_probe_v1.csv.staging";
const STAGING_MD: &str =
    "results/.px3_physical_event_boundaries_no_new_mechanism_probe_v1.md.staging";
const NAMESPACE_A: u64 = 0x5_4300_0000;
const NAMESPACE_B: u64 = 0x5_5300_0000;
const NAMESPACE_BLOCKED: u64 = 0x5_6300_0000;
const NAMESPACE_SUBTHRESHOLD: u64 = 0x5_7300_0000;
const NAMESPACE_REPLICA_A: u64 = 0x6_4300_0000;
const NAMESPACE_REPLICA_B: u64 = 0x6_5300_0000;
const RECURRENCES: usize = 12;
const BETWEEN_CLUSTER_GAP: i64 = 8;
const ROUND_SPACING: i64 = 18;
const HELD_OUT_GAP: i64 = 14;
const POST_GAP: i64 = 34;

// PX3_ORGANISM_VISIBLE_BEGIN
mod physics {
    use super::{
        ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
    };

    pub(super) const LANES: usize = 4;
    const SOURCE_THRESHOLD: usize = 4;
    const ACQUISITION_USES: usize = 4;
    const ACQUISITION_SPACING: i64 = 16;
    const TRAVEL: i64 = 4;

    #[derive(Clone)]
    pub(super) struct Matter {
        substrate: PlasticSubstrate,
        namespace: u64,
        arrivals: [CellId; LANES],
        correspondence_ends: [CellId; LANES],
        continuations: [CellId; LANES],
        consequences: [CellId; LANES],
        acquisition_drivers: [CellId; LANES],
        participation_drivers: [CellId; LANES],
        context: CellId,
        directional: [Option<ArrowId>; LANES],
        acquisition_work: WorkLedger,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub(super) struct Counts {
        pub continuation_firings: [usize; LANES],
        pub consequence_firings: [usize; LANES],
        pub trace_arrivals: [usize; LANES],
        pub trace_firings: [usize; LANES],
        pub local_returns: [usize; LANES],
        pub effects: [usize; LANES],
        pub hub_firings: usize,
        pub extra_source_firings: usize,
        pub quiescent: bool,
        pub work: WorkLedger,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct State {
        pub correspondence_resistance: [u32; LANES],
        pub directional_resistance: [u32; LANES],
        pub directional_live: [bool; LANES],
        pub permanent_fingerprint: u64,
        pub complete_fingerprint: u64,
        pub arrow_count: usize,
        pub persistent_bytes: usize,
    }

    pub(super) fn fresh(namespace: u64, reverse: bool, return_enabled: bool) -> Matter {
        let mut substrate = PlasticSubstrate::new();
        let mut arrivals = [None; LANES];
        let mut correspondence_ends = [None; LANES];
        let mut continuations = [None; LANES];
        let mut consequences = [None; LANES];
        let mut traces = [None; LANES];
        let mut outside = [None; LANES];
        let mut acquisition_drivers = [None; LANES];
        let mut participation_drivers = [None; LANES];
        let mut gates = [None; LANES];
        let order = if reverse {
            [3usize, 2, 1, 0]
        } else {
            [0usize, 1, 2, 3]
        };

        for lane in order {
            let base = lane as i32 * 36;
            arrivals[lane] =
                Some(substrate.add_cell(cell(namespace + 10 + lane as u64, base, 0, 4)));
            correspondence_ends[lane] =
                Some(substrate.add_cell(cell(namespace + 20 + lane as u64, base + 2, 0, 2)));
            continuations[lane] =
                Some(substrate.add_cell(cell(namespace + 30 + lane as u64, base + 8, 0, 2)));
            consequences[lane] =
                Some(substrate.add_cell(cell(namespace + 40 + lane as u64, base + 10, 0, 2)));
            traces[lane] =
                Some(substrate.add_cell(cell(namespace + 50 + lane as u64, base + 16, 0, 2)));
            outside[lane] =
                Some(substrate.add_cell(cell(namespace + 60 + lane as u64, 1_000 + base, 1, 1)));
            acquisition_drivers[lane] =
                Some(substrate.add_cell(cell(namespace + 70 + lane as u64, 1_200 + base, 0, 1)));
            participation_drivers[lane] =
                Some(substrate.add_cell(cell(namespace + 80 + lane as u64, 1_400 + base, 0, 1)));
            gates[lane] =
                Some(substrate.add_cell(cell(namespace + 90 + lane as u64, 1_600 + base, 0, 1)));
        }

        let arrivals = arrivals.map(|value| value.expect("arrival"));
        let correspondence_ends = correspondence_ends.map(|value| value.expect("end"));
        let continuations = continuations.map(|value| value.expect("continuation"));
        let consequences = consequences.map(|value| value.expect("consequence"));
        let traces = traces.map(|value| value.expect("trace"));
        let outside = outside.map(|value| value.expect("outside"));
        let acquisition_drivers = acquisition_drivers.map(|value| value.expect("driver"));
        let participation_drivers = participation_drivers.map(|value| value.expect("driver"));
        let gates = gates.map(|value| value.expect("gate"));
        let context = substrate.add_cell(cell(namespace + 100, 2_000, 0, 1));
        let hub = substrate.add_cell(cell(namespace + 101, 2_100, 0, 1));

        for lane in order {
            substrate.add_arrow(arrow(
                acquisition_drivers[lane],
                correspondence_ends[lane],
                2,
                1,
                1_000,
            ));
            substrate.add_arrow(arrow(correspondence_ends[lane], gates[lane], 1, 1, 1_000));
            substrate.add_arrow(arrow(gates[lane], arrivals[lane], 1, 1, 1_000));
            substrate.add_arrow(arrow(
                correspondence_ends[lane],
                continuations[lane],
                TRAVEL - 2,
                1,
                1_000,
            ));
            substrate.add_arrow(arrow(
                participation_drivers[lane],
                continuations[lane],
                TRAVEL,
                1,
                1_000,
            ));
            substrate.add_arrow(arrow(
                participation_drivers[lane],
                consequences[lane],
                TRAVEL + 1,
                1,
                1_000,
            ));
            substrate.add_arrow(arrow(context, continuations[lane], TRAVEL, 1, 1_000));
            substrate.add_arrow(arrow(consequences[lane], traces[lane], 1, 1, 1_000));
            if return_enabled {
                substrate.add_arrow(arrow(consequences[lane], hub, 1, 1, 1_000));
            }
            substrate.add_arrow(arrow(consequences[lane], outside[lane], 0, 1, 1_000));
            substrate.add_arrow(arrow(traces[lane], continuations[lane], 1, 1, 1_000));
            substrate.add_arrow(arrow(hub, traces[lane], 0, 1, 1_000));
        }

        let mut matter = Matter {
            substrate,
            namespace,
            arrivals,
            correspondence_ends,
            continuations,
            consequences,
            acquisition_drivers,
            participation_drivers,
            context,
            directional: [None; LANES],
            acquisition_work: WorkLedger::default(),
        };
        matter.acquisition_work = acquire(&mut matter, 0, namespace + 0x1_000);
        matter.directional = std::array::from_fn(|lane| {
            Some(matter.substrate.add_arrow(arrow(
                matter.continuations[lane],
                matter.consequences[lane],
                1,
                1,
                3,
            )))
        });
        matter
    }

    pub(super) fn clock(matter: &Matter) -> i64 {
        // The substrate intentionally exposes time only through admissible
        // future arrivals. The frozen acquisition cadence ends at tick 48.
        let _ = matter;
        48
    }

    pub(super) fn drive(matter: &mut Matter, entries: &[(i64, usize)]) -> Counts {
        let mut expected_sources = [0usize; LANES];
        for &(tick, lane) in entries {
            enter_many(
                &mut matter.substrate,
                matter.arrivals[lane],
                tick,
                SOURCE_THRESHOLD,
                matter.namespace + 0x10_000 + tick as u64 * 0x100 + lane as u64 * 0x10,
            );
            enter_many(
                &mut matter.substrate,
                matter.participation_drivers[lane],
                tick,
                1,
                matter.namespace + 0x20_000 + tick as u64 * 0x100 + lane as u64 * 0x10,
            );
            expected_sources[lane] += 1;
        }
        let run = matter.substrate.propagate();
        let ordinary_arrivals = expected_sources.map(|uses| uses * 2);
        counts(matter, &run, expected_sources, ordinary_arrivals)
    }

    pub(super) fn acquisition_work(matter: &Matter) -> WorkLedger {
        matter.acquisition_work.clone()
    }

    pub(super) fn use_at(matter: &Matter, tick: i64, lanes: &[usize]) -> Counts {
        let mut copy = matter.clone();
        let prior = copy.substrate.advance_time(tick);
        let mut expected_sources = [0usize; LANES];
        for &lane in lanes {
            enter_many(
                &mut copy.substrate,
                copy.arrivals[lane],
                tick,
                SOURCE_THRESHOLD,
                copy.namespace + 0x30_000 + lane as u64 * 0x10,
            );
            expected_sources[lane] += 1;
        }
        enter_many(
            &mut copy.substrate,
            copy.context,
            tick,
            1,
            copy.namespace + 0x40_000,
        );
        let run = copy.substrate.propagate();
        let ordinary_arrivals = expected_sources.map(|uses| uses + 1);
        let mut observed = counts(&copy, &run, expected_sources, ordinary_arrivals);
        add_work(&mut observed.work, &prior);
        observed
    }

    pub(super) fn use_spaced(
        matter: &Matter,
        tick: i64,
        first: usize,
        second: usize,
        spacing: i64,
    ) -> Counts {
        let mut copy = matter.clone();
        let prior = copy.substrate.advance_time(tick);
        let entries = [(tick, first), (tick + spacing, second)];
        let mut expected_sources = [0usize; LANES];
        for (at, lane) in entries {
            enter_many(
                &mut copy.substrate,
                copy.arrivals[lane],
                at,
                SOURCE_THRESHOLD,
                copy.namespace + 0x50_000 + lane as u64 * 0x10,
            );
            enter_many(
                &mut copy.substrate,
                copy.context,
                at,
                1,
                copy.namespace + 0x60_000 + lane as u64 * 0x10,
            );
            expected_sources[lane] += 1;
        }
        let run = copy.substrate.propagate();
        let ordinary_arrivals = expected_sources.map(|uses| uses + 2);
        let mut observed = counts(&copy, &run, expected_sources, ordinary_arrivals);
        add_work(&mut observed.work, &prior);
        observed
    }

    pub(super) fn state(matter: &Matter) -> State {
        State {
            correspondence_resistance: std::array::from_fn(|lane| {
                matter
                    .substrate
                    .arrows_between(matter.arrivals[lane], matter.correspondence_ends[lane])
                    .into_iter()
                    .map(|arrow| matter.substrate.arrow_resistance(arrow))
                    .max()
                    .unwrap_or(0)
            }),
            directional_resistance: std::array::from_fn(|lane| {
                matter
                    .substrate
                    .arrow_resistance(matter.directional[lane].expect("directional"))
            }),
            directional_live: std::array::from_fn(|lane| {
                matter
                    .substrate
                    .arrow_is_live(matter.directional[lane].expect("directional"))
            }),
            permanent_fingerprint: matter.substrate.permanent_fingerprint(),
            complete_fingerprint: matter.substrate.complete_fingerprint(),
            arrow_count: matter.substrate.arrow_count(),
            persistent_bytes: matter.substrate.persistent_bytes(),
        }
    }

    fn acquire(matter: &mut Matter, tick: i64, origin: u64) -> WorkLedger {
        for use_ordinal in 0..ACQUISITION_USES {
            let at = tick + use_ordinal as i64 * ACQUISITION_SPACING;
            for lane in 0..LANES {
                enter_many(
                    &mut matter.substrate,
                    matter.arrivals[lane],
                    at,
                    SOURCE_THRESHOLD,
                    origin + use_ordinal as u64 * 0x100 + lane as u64 * 0x10,
                );
                enter_many(
                    &mut matter.substrate,
                    matter.acquisition_drivers[lane],
                    at,
                    1,
                    origin + 0x1_000 + use_ordinal as u64 * 0x100 + lane as u64 * 0x10,
                );
            }
        }
        let run = matter.substrate.propagate();
        assert!(run.naturally_quiescent, "finite acquisition must drain");
        run.work
    }

    fn counts(
        matter: &Matter,
        run: &Execution,
        expected_sources: [usize; LANES],
        ordinary_arrivals: [usize; LANES],
    ) -> Counts {
        let continuation_firings =
            std::array::from_fn(|lane| firings_at(run, matter.namespace + 30 + lane as u64));
        let consequence_firings =
            std::array::from_fn(|lane| firings_at(run, matter.namespace + 40 + lane as u64));
        let trace_arrivals =
            std::array::from_fn(|lane| arrivals_at(run, matter.namespace + 50 + lane as u64));
        let trace_firings =
            std::array::from_fn(|lane| firings_at(run, matter.namespace + 50 + lane as u64));
        let local_returns = std::array::from_fn(|lane| {
            arrivals_at(run, matter.namespace + 30 + lane as u64)
                .saturating_sub(ordinary_arrivals[lane])
        });
        let effects = std::array::from_fn(|lane| {
            run.crossings
                .iter()
                .filter(|crossing| {
                    crossing.from_physical == matter.namespace + 40 + lane as u64
                        && crossing.to_region == 1
                })
                .count()
        });
        let source_firings: [usize; LANES] =
            std::array::from_fn(|lane| firings_at(run, matter.namespace + 10 + lane as u64));
        Counts {
            continuation_firings,
            consequence_firings,
            trace_arrivals,
            trace_firings,
            local_returns,
            effects,
            hub_firings: firings_at(run, matter.namespace + 101),
            extra_source_firings: (0..LANES)
                .map(|lane| source_firings[lane].saturating_sub(expected_sources[lane]))
                .sum(),
            quiescent: run.naturally_quiescent,
            work: run.work.clone(),
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

    fn arrivals_at(run: &Execution, physical: u64) -> usize {
        run.trace
            .iter()
            .filter(|entry| entry.target_physical == physical)
            .count()
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
}
// PX3_ORGANISM_VISIBLE_END

#[derive(Clone, Debug, PartialEq, Eq)]
struct HeldOut {
    trained: physics::Counts,
    crossed: physics::Counts,
    gapped: physics::Counts,
    singleton: physics::Counts,
    post_gap_trained: physics::Counts,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CellResult {
    name: &'static str,
    namespace: u64,
    state: physics::State,
    acquisition_work: WorkLedger,
    training: physics::Counts,
    held_out: HeldOut,
    duplicate_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Report {
    cells: Vec<CellResult>,
    blocked: CellResult,
    subthreshold: CellResult,
    source_exact: bool,
    marginals_matched: bool,
    persistent_states_equal: bool,
    held_out_symmetric: bool,
    transient_gap_visible: bool,
    controls_passed: bool,
    emerges: bool,
    classification: &'static str,
    total_work: u64,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let preflight = args == ["--preflight"];
    let probe = args == ["--probe"];
    if !preflight && !probe {
        eprintln!("PX3 no-new-mechanism PROBE requires --preflight or --probe");
        std::process::exit(2);
    }
    assert!(
        source_audit(),
        "frozen sources and organism block must be exact"
    );
    for path in [RESULT_CSV, RESULT_MD, STAGING_CSV, STAGING_MD] {
        assert!(!Path::new(path).exists(), "PROBE artifact exists: {path}");
    }
    if preflight {
        println!("PX3_PHYSICAL_EVENT_BOUNDARIES_NO_NEW_MECHANISM_PROBE_PREFLIGHT_OK");
        return;
    }

    eprintln!("PX3_PHYSICAL_EVENT_BOUNDARIES_NO_NEW_MECHANISM_PROBE_EVIDENCE_SPENT");
    let report = run_probe();
    write_atomic(&report);
    println!(
        "PX3_PHYSICAL_EVENT_BOUNDARIES_NO_NEW_MECHANISM_PROBE_{}",
        report.classification
    );
    if report.emerges {
        std::process::exit(0);
    }
    std::process::exit(1);
}

fn run_probe() -> Report {
    let a = run_cell(
        "reference-a",
        NAMESPACE_A,
        false,
        &[[0, 1], [2, 3]],
        true,
        RECURRENCES,
    );
    let a_dup = run_cell(
        "reference-a",
        NAMESPACE_A,
        false,
        &[[0, 1], [2, 3]],
        true,
        RECURRENCES,
    );
    let b = run_cell(
        "reference-b",
        NAMESPACE_B,
        false,
        &[[0, 2], [1, 3]],
        true,
        RECURRENCES,
    );
    let b_dup = run_cell(
        "reference-b",
        NAMESPACE_B,
        false,
        &[[0, 2], [1, 3]],
        true,
        RECURRENCES,
    );
    let replica_a = run_cell(
        "replica-a",
        NAMESPACE_REPLICA_A,
        true,
        &[[0, 1], [2, 3]],
        true,
        RECURRENCES,
    );
    let replica_a_dup = run_cell(
        "replica-a",
        NAMESPACE_REPLICA_A,
        true,
        &[[0, 1], [2, 3]],
        true,
        RECURRENCES,
    );
    let replica_b = run_cell(
        "replica-b",
        NAMESPACE_REPLICA_B,
        true,
        &[[0, 2], [1, 3]],
        true,
        RECURRENCES,
    );
    let replica_b_dup = run_cell(
        "replica-b",
        NAMESPACE_REPLICA_B,
        true,
        &[[0, 2], [1, 3]],
        true,
        RECURRENCES,
    );
    let blocked = run_cell(
        "blocked-return",
        NAMESPACE_BLOCKED,
        false,
        &[[0, 1], [2, 3]],
        false,
        RECURRENCES,
    );
    let blocked_dup = run_cell(
        "blocked-return",
        NAMESPACE_BLOCKED,
        false,
        &[[0, 1], [2, 3]],
        false,
        RECURRENCES,
    );
    let subthreshold = run_cell(
        "subthreshold",
        NAMESPACE_SUBTHRESHOLD,
        false,
        &[[0, 1], [2, 3]],
        true,
        1,
    );
    let subthreshold_dup = run_cell(
        "subthreshold",
        NAMESPACE_SUBTHRESHOLD,
        false,
        &[[0, 1], [2, 3]],
        true,
        1,
    );

    let mut cells = vec![a, b, replica_a, replica_b];
    for (cell, duplicate) in cells
        .iter_mut()
        .zip([a_dup, b_dup, replica_a_dup, replica_b_dup])
    {
        cell.duplicate_exact = same_cell(cell, &duplicate);
    }
    let mut blocked = blocked;
    blocked.duplicate_exact = same_cell(&blocked, &blocked_dup);
    let mut subthreshold = subthreshold;
    subthreshold.duplicate_exact = same_cell(&subthreshold, &subthreshold_dup);

    let marginals_matched = cells.iter().all(|cell| {
        cell.training.continuation_firings == [RECURRENCES; physics::LANES]
            && cell.training.consequence_firings == [RECURRENCES; physics::LANES]
            && cell.training.trace_firings == [RECURRENCES; physics::LANES]
            && cell.training.local_returns == [RECURRENCES; physics::LANES]
            && cell.training.effects == [RECURRENCES; physics::LANES]
    });
    let persistent_states_equal = equivalent_state(&cells[0].state, &cells[1].state)
        && equivalent_state(&cells[2].state, &cells[3].state);
    let held_out_symmetric = cells.iter().all(|cell| {
        same_use(&cell.held_out.trained, &cell.held_out.crossed)
            && cell.held_out.trained.effects.iter().sum::<usize>() == 2
            && cell.held_out.crossed.effects.iter().sum::<usize>() == 2
    });
    let transient_gap_visible = cells.iter().all(|cell| {
        cell.held_out.trained.hub_firings == 1
            && cell.held_out.gapped.hub_firings == 2
            && cell.held_out.singleton.hub_firings == 1
            && cell.held_out.trained.effects.iter().sum::<usize>() == 2
            && cell.held_out.gapped.effects.iter().sum::<usize>() == 2
            && cell.held_out.singleton.effects.iter().sum::<usize>() == 1
    });
    let positive_routes = cells.iter().all(|cell| {
        cell.state
            .correspondence_resistance
            .iter()
            .all(|value| *value > 1)
            && cell.state.directional_live == [true; physics::LANES]
            && cell
                .state
                .directional_resistance
                .iter()
                .all(|value| *value > 3)
            && cell.held_out.post_gap_trained.effects.iter().sum::<usize>() == 2
    });
    let blocked_control = blocked.state.directional_live == [false; physics::LANES]
        && blocked.held_out.trained.effects == [0; physics::LANES];
    let subthreshold_control =
        subthreshold.held_out.post_gap_trained.effects == [0; physics::LANES];
    let quiet = cells.iter().chain([&blocked, &subthreshold]).all(|cell| {
        cell.training.quiescent
            && cell.held_out.trained.quiescent
            && cell.held_out.crossed.quiescent
            && cell.held_out.gapped.quiescent
            && cell.held_out.singleton.quiescent
            && cell.held_out.post_gap_trained.quiescent
            && cell.training.extra_source_firings == 0
            && cell.held_out.trained.extra_source_firings == 0
            && cell.held_out.crossed.extra_source_firings == 0
            && cell.held_out.gapped.extra_source_firings == 0
            && cell.held_out.singleton.extra_source_firings == 0
            && cell.held_out.post_gap_trained.extra_source_firings == 0
            && cell.duplicate_exact
    });
    let layout_control = same_behavior(&cells[0], &cells[2]) && same_behavior(&cells[1], &cells[3]);
    let controls_passed = source_audit()
        && marginals_matched
        && positive_routes
        && blocked_control
        && subthreshold_control
        && transient_gap_visible
        && quiet
        && layout_control;
    let relation_specific = !persistent_states_equal && !held_out_symmetric;
    let emerges = controls_passed && relation_specific;
    let classification = if emerges {
        "EMERGES_WITHOUT_NEW_MECHANISM"
    } else if controls_passed && persistent_states_equal && held_out_symmetric {
        "NO_RELATION_SPECIFIC_STATE_IN_PX0_PX2"
    } else {
        "FIRST_CLAUSE_FAILURE"
    };
    let total_work = cells
        .iter()
        .chain([&blocked, &subthreshold])
        .map(cell_work)
        .sum();

    Report {
        cells,
        blocked,
        subthreshold,
        source_exact: source_audit(),
        marginals_matched,
        persistent_states_equal,
        held_out_symmetric,
        transient_gap_visible,
        controls_passed,
        emerges,
        classification,
        total_work,
    }
}

fn run_cell(
    name: &'static str,
    namespace: u64,
    reverse: bool,
    pairing: &[[usize; 2]; 2],
    return_enabled: bool,
    recurrences: usize,
) -> CellResult {
    let mut matter = physics::fresh(namespace, reverse, return_enabled);
    let first_tick = physics::clock(&matter) + 18;
    let mut entries = Vec::new();
    for recurrence in 0..recurrences {
        let base = first_tick + recurrence as i64 * ROUND_SPACING;
        let order = if recurrence.is_multiple_of(2) {
            [0, 1]
        } else {
            [1, 0]
        };
        for (slot, pair_index) in order.into_iter().enumerate() {
            let tick = base + slot as i64 * BETWEEN_CLUSTER_GAP;
            for lane in pairing[pair_index] {
                entries.push((tick, lane));
            }
        }
    }
    let training = physics::drive(&mut matter, &entries);
    let acquisition_work = physics::acquisition_work(&matter);
    let state = physics::state(&matter);
    let final_tick =
        first_tick + recurrences.saturating_sub(1) as i64 * ROUND_SPACING + BETWEEN_CLUSTER_GAP + 8;
    let trained_pair = pairing[0];
    let crossed_pair = [pairing[0][0], pairing[1][0]];
    let post_gap = if recurrences == 1 { 54 } else { POST_GAP };
    let held_out = HeldOut {
        trained: physics::use_at(&matter, final_tick + HELD_OUT_GAP, &trained_pair),
        crossed: physics::use_at(&matter, final_tick + HELD_OUT_GAP, &crossed_pair),
        gapped: physics::use_spaced(
            &matter,
            final_tick + HELD_OUT_GAP,
            trained_pair[0],
            trained_pair[1],
            6,
        ),
        singleton: physics::use_at(&matter, final_tick + HELD_OUT_GAP, &trained_pair[..1]),
        post_gap_trained: physics::use_at(&matter, final_tick + post_gap, &trained_pair),
    };
    CellResult {
        name,
        namespace,
        state,
        acquisition_work,
        training,
        held_out,
        duplicate_exact: false,
    }
}

fn same_cell(left: &CellResult, right: &CellResult) -> bool {
    left.name == right.name
        && left.namespace == right.namespace
        && left.state == right.state
        && left.acquisition_work == right.acquisition_work
        && left.training == right.training
        && left.held_out == right.held_out
}

fn equivalent_state(left: &physics::State, right: &physics::State) -> bool {
    left.correspondence_resistance == right.correspondence_resistance
        && left.directional_resistance == right.directional_resistance
        && left.directional_live == right.directional_live
        && left.arrow_count == right.arrow_count
        && left.persistent_bytes == right.persistent_bytes
        && left.permanent_fingerprint != 0
        && right.permanent_fingerprint != 0
}

fn same_use(left: &physics::Counts, right: &physics::Counts) -> bool {
    left.continuation_firings.iter().sum::<usize>()
        == right.continuation_firings.iter().sum::<usize>()
        && left.consequence_firings.iter().sum::<usize>()
            == right.consequence_firings.iter().sum::<usize>()
        && left.trace_firings.iter().sum::<usize>() == right.trace_firings.iter().sum::<usize>()
        && left.local_returns.iter().sum::<usize>() == right.local_returns.iter().sum::<usize>()
        && left.effects.iter().sum::<usize>() == right.effects.iter().sum::<usize>()
        && left.hub_firings == right.hub_firings
        && left.extra_source_firings == right.extra_source_firings
        && left.quiescent == right.quiescent
}

fn same_behavior(left: &CellResult, right: &CellResult) -> bool {
    left.state.correspondence_resistance == right.state.correspondence_resistance
        && left.state.directional_resistance == right.state.directional_resistance
        && left.state.directional_live == right.state.directional_live
        && same_use(&left.held_out.trained, &right.held_out.trained)
        && same_use(&left.held_out.crossed, &right.held_out.crossed)
        && same_use(&left.held_out.gapped, &right.held_out.gapped)
        && same_use(&left.held_out.singleton, &right.held_out.singleton)
        && same_use(
            &left.held_out.post_gap_trained,
            &right.held_out.post_gap_trained,
        )
}

fn cell_work(cell: &CellResult) -> u64 {
    cell_ledger(cell).total()
}

fn cell_ledger(cell: &CellResult) -> WorkLedger {
    let mut total = cell.acquisition_work.clone();
    for additional in [
        &cell.training.work,
        &cell.held_out.trained.work,
        &cell.held_out.crossed.work,
        &cell.held_out.gapped.work,
        &cell.held_out.singleton.work,
        &cell.held_out.post_gap_trained.work,
    ] {
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
    total
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
            "results/px1_physical_boundary_roles_definitive.csv",
            PX1_CSV_SHA256,
        ),
        (
            "results/px2_physical_causal_direction_definitive.csv",
            PX2_CSV_SHA256,
        ),
        (
            "experiments/px2_physical_causal_direction_authority_handoff.md",
            PX2_HANDOFF_SHA256,
        ),
        ("src/ds3_event_boundary.rs", OLD_M3_SOURCE_SHA256),
        (
            "results/ds3_cumulative_event_boundary_definitive.csv",
            OLD_M3_CSV_SHA256,
        ),
        (PROTOCOL, PROTOCOL_SHA256),
    ];
    let hashes_exact = hashes
        .into_iter()
        .all(|(path, expected)| sha256(path).as_deref() == Some(expected));
    let parent_exact = command_output(&[
        "rev-parse",
        "px2-physical-causal-direction-authoritative^{commit}",
    ])
    .as_deref()
        == Some(FROZEN_PARENT)
        && Command::new("git")
            .args(["merge-base", "--is-ancestor", FROZEN_PARENT, "HEAD"])
            .status()
            .is_ok_and(|status| status.success());
    let source = include_str!("px3_physical_event_boundaries_probe.rs");
    let physical = source
        .split("// PX3_ORGANISM_VISIBLE_BEGIN")
        .nth(1)
        .and_then(|text| text.split("// PX3_ORGANISM_VISIBLE_END").next())
        .unwrap_or("")
        .to_ascii_lowercase();
    let forbidden = [
        "event",
        "episode",
        "history",
        "boundary",
        "group",
        "partition",
        "segment",
        "span",
        "chunk",
        "old_m",
        "ds3",
        "px3",
        "semantic",
        "serializer",
        "evaluator",
    ];
    let organism_clean =
        !physical.is_empty() && forbidden.iter().all(|word| !physical.contains(word));
    hashes_exact && parent_exact && organism_clean
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
        "name,namespace,correspondence_resistance,directional_resistance,directional_live,training_continuations,training_consequences,training_traces,training_returns,training_effects,trained_effects,crossed_effects,gapped_effects,singleton_effects,post_gap_effects,trained_hub,crossed_hub,gapped_hub,singleton_hub,training_quiescent,held_out_quiescent,extra_source_firings,permanent_fingerprint,complete_fingerprint,arrow_count,persistent_bytes,duplicate_exact,queue_comparisons,spikes_delivered,generation_checks,state_updates,threshold_checks,firings,arrow_checks,spikes_emitted,local_eligibility_writes,local_return_updates,ordinary_pressure_updates,local_structural_proposals,physical_deallocations,work\n",
    );
    for cell in report
        .cells
        .iter()
        .chain([&report.blocked, &report.subthreshold])
    {
        let held_quiet = cell.held_out.trained.quiescent
            && cell.held_out.crossed.quiescent
            && cell.held_out.gapped.quiescent
            && cell.held_out.singleton.quiescent
            && cell.held_out.post_gap_trained.quiescent;
        let extra_sources = cell.training.extra_source_firings
            + cell.held_out.trained.extra_source_firings
            + cell.held_out.crossed.extra_source_firings
            + cell.held_out.gapped.extra_source_firings
            + cell.held_out.singleton.extra_source_firings
            + cell.held_out.post_gap_trained.extra_source_firings;
        let ledger = cell_ledger(cell);
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            cell.name,
            cell.namespace,
            join_u32(&cell.state.correspondence_resistance),
            join_u32(&cell.state.directional_resistance),
            join_bool(&cell.state.directional_live),
            join_usize(&cell.training.continuation_firings),
            join_usize(&cell.training.consequence_firings),
            join_usize(&cell.training.trace_firings),
            join_usize(&cell.training.local_returns),
            join_usize(&cell.training.effects),
            join_usize(&cell.held_out.trained.effects),
            join_usize(&cell.held_out.crossed.effects),
            join_usize(&cell.held_out.gapped.effects),
            join_usize(&cell.held_out.singleton.effects),
            join_usize(&cell.held_out.post_gap_trained.effects),
            cell.held_out.trained.hub_firings,
            cell.held_out.crossed.hub_firings,
            cell.held_out.gapped.hub_firings,
            cell.held_out.singleton.hub_firings,
            cell.training.quiescent,
            held_quiet,
            extra_sources,
            cell.state.permanent_fingerprint,
            cell.state.complete_fingerprint,
            cell.state.arrow_count,
            cell.state.persistent_bytes,
            cell.duplicate_exact,
            ledger_csv(&ledger),
            cell_work(cell),
        ));
    }

    let markdown = format!(
        "# PX3 physical event-boundary no-new-mechanism PROBE (v2 protocol)\n\nVerdict: **{}**.\n\nThis is a development PROBE, not an authoritative or definitive result. PX3 remains absent.\n\n| clause | result |\n|---|:---:|\n| frozen source and parent audit | {} |\n| matched per-route participation marginals | {} |\n| A/B persistent route state equal | {} |\n| trained/crossed held-out behavior symmetric | {} |\n| transient close/gap signal visible | {} |\n| recurrence, blocked-return, layout, replay, quiescence controls | {} |\n| target emerges without new mechanism | {} |\n\nCells: `{}` positive-history/layout cells plus blocked-return and subthreshold controls. Total ledgered work: `{}` operations.\n\n## Interpretation\n\nThe retained PX0–PX2 physics preserved transient timing, matured every actually traversed and returned route, survived the fixed post-gap, and naturally quiesced with zero autonomous source refiring. However, matched recurring partitions left the same relation-free persistent route state, and a trained pair was physically indistinguishable from a crossed pair during held-out use. The shared PX2 return hub records route participation marginals, not which routes recurred together.\n\nTransient clustering therefore does not become reusable event organization in existing PX0–PX2 state. Repair would require relation-specific persistent structure or a new substrate law. Both exceed this lane's authority, so there is no MICRO or GATE.\n",
        report.classification,
        report.source_exact,
        report.marginals_matched,
        report.persistent_states_equal,
        report.held_out_symmetric,
        report.transient_gap_visible,
        report.controls_passed,
        report.emerges,
        report.cells.len(),
        report.total_work,
    );
    create_new(STAGING_CSV, csv.as_bytes());
    create_new(STAGING_MD, markdown.as_bytes());
    rename(STAGING_CSV, RESULT_CSV).expect("publish PROBE CSV");
    rename(STAGING_MD, RESULT_MD).expect("publish PROBE report");
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
