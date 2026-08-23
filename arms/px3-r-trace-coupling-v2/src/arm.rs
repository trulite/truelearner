use crate::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, LocalActivityOpportunity, PlasticSubstrate,
    SpikeInput, WorkLedger,
};

pub const LANES: usize = 4;
const ACQUISITION_USES: usize = 4;
const ACQUISITION_SPACING: i64 = 16;
const SOURCE_THRESHOLD: usize = 4;

#[derive(Clone)]
pub struct Matter {
    substrate: PlasticSubstrate,
    namespace: u64,
    arrivals: [CellId; LANES],
    correspondence_ends: [CellId; LANES],
    continuations: [CellId; LANES],
    consequences: [CellId; LANES],
    traces: [CellId; LANES],
    acquisition_drivers: [CellId; LANES],
    participation_drivers: [CellId; LANES],
    nearby: Option<[CellId; LANES]>,
    directional: [Option<ArrowId>; LANES],
    acquisition_work: WorkLedger,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub continuation_firings: [usize; LANES],
    pub consequence_firings: [usize; LANES],
    pub trace_arrivals: [usize; LANES],
    pub trace_firings: [usize; LANES],
    pub route_returns: [usize; LANES],
    pub local_arrivals: [usize; LANES],
    pub local_impulse: [i32; LANES],
    pub effects: [usize; LANES],
    pub hub_firings: usize,
    pub source_firings: [usize; LANES],
    pub extra_source_firings: usize,
    pub quiescent: bool,
    pub work: WorkLedger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub correspondence_resistance: [u32; LANES],
    pub directional_resistance: [u32; LANES],
    pub directional_live: [bool; LANES],
    pub local_resistance: [[u32; LANES]; LANES],
    pub local_live: [[bool; LANES]; LANES],
    pub permanent_fingerprint: u64,
    pub complete_fingerprint: u64,
    pub arrow_count: usize,
    pub persistent_bytes: usize,
}

pub fn fresh(
    namespace: u64,
    reverse: bool,
    opportunity: Option<LocalActivityOpportunity>,
    trace_positions: [i32; LANES],
    nearby_matter: bool,
) -> Matter {
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
    let mut nearby = [None; LANES];
    let order = if reverse {
        [3usize, 2, 1, 0]
    } else {
        [0usize, 1, 2, 3]
    };

    for lane in order {
        let base = 1_000 + lane as i32 * 100;
        arrivals[lane] = Some(substrate.add_cell(cell(namespace + 10 + lane as u64, base, 0, 4)));
        correspondence_ends[lane] =
            Some(substrate.add_cell(cell(namespace + 20 + lane as u64, base + 2, 0, 2)));
        continuations[lane] =
            Some(substrate.add_cell(cell(namespace + 30 + lane as u64, base + 20, 0, 2)));
        consequences[lane] =
            Some(substrate.add_cell(cell(namespace + 40 + lane as u64, base + 40, 0, 2)));
        traces[lane] = Some(substrate.add_cell(cell(
            namespace + 50 + lane as u64,
            trace_positions[lane],
            0,
            4,
        )));
        outside[lane] =
            Some(substrate.add_cell(cell(namespace + 60 + lane as u64, 20_000 + base, 1, 1)));
        acquisition_drivers[lane] =
            Some(substrate.add_cell(cell(namespace + 70 + lane as u64, 30_000 + base, 0, 1)));
        participation_drivers[lane] =
            Some(substrate.add_cell(cell(namespace + 80 + lane as u64, 40_000 + base, 0, 1)));
        gates[lane] =
            Some(substrate.add_cell(cell(namespace + 90 + lane as u64, 50_000 + base, 0, 1)));
        if nearby_matter {
            nearby[lane] = Some(substrate.add_cell(cell(
                namespace + 110 + lane as u64,
                trace_positions[lane] + 4,
                0,
                100,
            )));
        }
    }

    let arrivals = unwrap_cells(arrivals);
    let correspondence_ends = unwrap_cells(correspondence_ends);
    let continuations = unwrap_cells(continuations);
    let consequences = unwrap_cells(consequences);
    let traces = unwrap_cells(traces);
    let outside = unwrap_cells(outside);
    let acquisition_drivers = unwrap_cells(acquisition_drivers);
    let participation_drivers = unwrap_cells(participation_drivers);
    let gates = unwrap_cells(gates);
    let nearby = nearby_matter.then(|| unwrap_cells(nearby));
    let hub = substrate.add_cell(cell(namespace + 100, 60_000, 0, 1));

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
            2,
            1,
            1_000,
        ));
        substrate.add_arrow(arrow(
            participation_drivers[lane],
            continuations[lane],
            4,
            1,
            1_000,
        ));
        substrate.add_arrow(arrow(
            participation_drivers[lane],
            consequences[lane],
            5,
            1,
            1_000,
        ));
        substrate.add_arrow(arrow(consequences[lane], traces[lane], 1, 2, 1_000));
        substrate.add_arrow(arrow(consequences[lane], hub, 1, 1, 1_000));
        substrate.add_arrow(arrow(consequences[lane], outside[lane], 0, 1, 1_000));
        substrate.add_arrow(arrow(traces[lane], continuations[lane], 1, 1, 1_000));
        substrate.add_arrow(arrow(hub, traces[lane], 0, 2, 1_000));
    }

    let mut matter = Matter {
        substrate,
        namespace,
        arrivals,
        correspondence_ends,
        continuations,
        consequences,
        traces,
        acquisition_drivers,
        participation_drivers,
        nearby,
        directional: [None; LANES],
        acquisition_work: WorkLedger::default(),
    };
    matter.acquisition_work = acquire(&mut matter);
    matter.directional = std::array::from_fn(|lane| {
        Some(matter.substrate.add_arrow(arrow(
            matter.continuations[lane],
            matter.consequences[lane],
            1,
            1,
            3,
        )))
    });
    let maturation = drive(&mut matter, &[(64, 0), (64, 1), (72, 2), (72, 3)]);
    add_work(&mut matter.acquisition_work, &maturation.work);
    if let Some(value) = opportunity {
        matter.substrate.install_local_activity_opportunity(value);
    }
    matter
}

pub fn drive(matter: &mut Matter, entries: &[(i64, usize)]) -> Counts {
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
    counts(matter, &run, expected_sources)
}

pub fn use_at(matter: &Matter, tick: i64, entries: &[(i64, usize)]) -> (Counts, State) {
    let mut copy = matter.clone();
    let prior = copy.substrate.advance_time(tick);
    let shifted = entries
        .iter()
        .map(|(offset, lane)| (tick + offset, *lane))
        .collect::<Vec<_>>();
    let mut observed = drive(&mut copy, &shifted);
    add_work(&mut observed.work, &prior);
    let state = state(&copy);
    (observed, state)
}

pub fn drive_nearby(matter: &mut Matter, entries: &[(i64, usize)]) -> Counts {
    let cells = matter.nearby.expect("nearby CELL matter required");
    for &(tick, lane) in entries {
        enter_many(
            &mut matter.substrate,
            cells[lane],
            tick,
            100,
            matter.namespace + 0x30_000 + tick as u64 * 0x100 + lane as u64,
        );
    }
    let run = matter.substrate.propagate();
    counts(matter, &run, [0; LANES])
}

pub fn advance(matter: &mut Matter, tick: i64) -> WorkLedger {
    matter.substrate.advance_time(tick)
}

pub fn state(matter: &Matter) -> State {
    let correspondence_resistance = std::array::from_fn(|lane| {
        matter
            .substrate
            .arrows_between(matter.arrivals[lane], matter.correspondence_ends[lane])
            .into_iter()
            .filter(|arrow| matter.substrate.arrow_is_live(*arrow))
            .map(|arrow| matter.substrate.arrow_resistance(arrow))
            .max()
            .unwrap_or(0)
    });
    let directional_resistance = std::array::from_fn(|lane| {
        matter
            .substrate
            .arrow_resistance(matter.directional[lane].expect("directional ARROW"))
    });
    let directional_live = std::array::from_fn(|lane| {
        matter
            .substrate
            .arrow_is_live(matter.directional[lane].expect("directional ARROW"))
    });
    let local_resistance = std::array::from_fn(|from| {
        std::array::from_fn(|to| {
            if from == to {
                return 0;
            }
            matter
                .substrate
                .arrows_between(matter.traces[from], matter.traces[to])
                .into_iter()
                .filter(|arrow| matter.substrate.arrow_is_live(*arrow))
                .map(|arrow| matter.substrate.arrow_resistance(arrow))
                .max()
                .unwrap_or(0)
        })
    });
    let local_live = std::array::from_fn(|from| {
        std::array::from_fn(|to| from != to && local_resistance[from][to] > 0)
    });
    State {
        correspondence_resistance,
        directional_resistance,
        directional_live,
        local_resistance,
        local_live,
        permanent_fingerprint: matter.substrate.permanent_fingerprint(),
        complete_fingerprint: matter.substrate.complete_fingerprint(),
        arrow_count: matter.substrate.arrow_count(),
        persistent_bytes: matter.substrate.persistent_bytes(),
    }
}

pub fn acquisition_work(matter: &Matter) -> WorkLedger {
    matter.acquisition_work.clone()
}

fn acquire(matter: &mut Matter) -> WorkLedger {
    for use_ordinal in 0..ACQUISITION_USES {
        let tick = use_ordinal as i64 * ACQUISITION_SPACING;
        for lane in 0..LANES {
            enter_many(
                &mut matter.substrate,
                matter.arrivals[lane],
                tick,
                SOURCE_THRESHOLD,
                matter.namespace + 0x1_000 + use_ordinal as u64 * 0x100 + lane as u64 * 0x10,
            );
            enter_many(
                &mut matter.substrate,
                matter.acquisition_drivers[lane],
                tick,
                1,
                matter.namespace + 0x2_000 + use_ordinal as u64 * 0x100 + lane as u64 * 0x10,
            );
        }
    }
    let run = matter.substrate.propagate();
    assert!(run.naturally_quiescent, "finite acquisition must drain");
    run.work
}

fn counts(matter: &Matter, run: &Execution, expected_sources: [usize; LANES]) -> Counts {
    let continuation_firings =
        std::array::from_fn(|lane| firings_at(run, matter.namespace + 30 + lane as u64));
    let consequence_firings =
        std::array::from_fn(|lane| firings_at(run, matter.namespace + 40 + lane as u64));
    let trace_arrivals =
        std::array::from_fn(|lane| arrivals_at(run, matter.namespace + 50 + lane as u64));
    let trace_firings =
        std::array::from_fn(|lane| firings_at(run, matter.namespace + 50 + lane as u64));
    let hub_firings = firings_at(run, matter.namespace + 100);
    let route_returns = std::array::from_fn(|lane| {
        arrivals_at(run, matter.namespace + 30 + lane as u64)
            .saturating_sub(expected_sources[lane] * 2)
    });
    let local_arrivals = std::array::from_fn(|lane| {
        trace_arrivals[lane]
            .saturating_sub(consequence_firings[lane])
            .saturating_sub(hub_firings)
    });
    let local_impulse = std::array::from_fn(|lane| {
        impulse_at(run, matter.namespace + 50 + lane as u64)
            .saturating_sub(consequence_firings[lane] as i32 * 2)
            .saturating_sub(hub_firings as i32 * 2)
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
    let source_firings =
        std::array::from_fn(|lane| firings_at(run, matter.namespace + 10 + lane as u64));
    Counts {
        continuation_firings,
        consequence_firings,
        trace_arrivals,
        trace_firings,
        route_returns,
        local_arrivals,
        local_impulse,
        effects,
        hub_firings,
        source_firings,
        extra_source_firings: source_firings
            .iter()
            .zip(expected_sources)
            .map(|(actual, expected)| actual.saturating_sub(expected))
            .sum(),
        quiescent: run.naturally_quiescent,
        work: run.work.clone(),
    }
}

fn unwrap_cells(values: [Option<CellId>; LANES]) -> [CellId; LANES] {
    values.map(|value| value.expect("physical CELL"))
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

fn impulse_at(run: &Execution, physical: u64) -> i32 {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical)
        .map(|entry| entry.impulse)
        .sum()
}

pub fn add_work(total: &mut WorkLedger, next: &WorkLedger) {
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
