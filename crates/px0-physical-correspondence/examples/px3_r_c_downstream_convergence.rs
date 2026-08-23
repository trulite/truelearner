use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
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
const PX3_NEGATIVE_SOURCE_SHA256: &str =
    "39ec595fc1204a29083d271ebcadcdb7950c07d1c44e4ce07c0107fca54730ba";
const PX3_NEGATIVE_HANDOFF_SHA256: &str =
    "a029f250ed88f8f2fc164e0d2c9042675bf0a8c9ae51c89cf83ad1aa42e4fa9b";
const PX3_NEGATIVE_CSV_SHA256: &str =
    "685dc04db32a5785224c62ba5b589fa8e1e37382a8b613f5f2b5e396aa005f38";
const PX3_NEGATIVE_REPORT_SHA256: &str =
    "021be8698c010df1e09dc45f2bf9968f2255b6eb7851c38f36fd93be72260d3b";
const FIRST_COLLAPSE_SHA256: &str =
    "5bc10db8be8625be3ebbe5ae67bad4986b2190d4b4d656db015204ae2517e56f";
const PROTOCOL_SHA256: &str = "f51d557d66757c04d7905aa54fae4ca71de4f5243f22d99cdb9273fd584bd80b";

const FIRST_COLLAPSE: &str = "experiments/px3_r_c_downstream_convergence_first_collapse.md";
const PROTOCOL: &str = "experiments/px3_r_c_downstream_convergence_development_protocol.md";
const PROBE_CSV: &str = "results/px3_r_c_downstream_convergence_probe_v1.csv";
const PROBE_MD: &str = "results/px3_r_c_downstream_convergence_probe_v1.md";
const PROBE_STAGING_CSV: &str = "results/.px3_r_c_downstream_convergence_probe_v1.csv.staging";
const PROBE_STAGING_MD: &str = "results/.px3_r_c_downstream_convergence_probe_v1.md.staging";
const MICRO_CSV: &str = "results/px3_r_c_downstream_convergence_micro_v1.csv";
const MICRO_MD: &str = "results/px3_r_c_downstream_convergence_micro_v1.md";
const MICRO_STAGING_CSV: &str = "results/.px3_r_c_downstream_convergence_micro_v1.csv.staging";
const MICRO_STAGING_MD: &str = "results/.px3_r_c_downstream_convergence_micro_v1.md.staging";
const GATE_CSV: &str = "results/px3_r_c_downstream_convergence_gate_v1.csv";
const GATE_MD: &str = "results/px3_r_c_downstream_convergence_gate_v1.md";
const GATE_STAGING_CSV: &str = "results/.px3_r_c_downstream_convergence_gate_v1.csv.staging";
const GATE_STAGING_MD: &str = "results/.px3_r_c_downstream_convergence_gate_v1.md.staging";

const RECURRENCES: usize = 10;
const FIRST_USE: i64 = 64;
const HELD_OUT_GAP: i64 = 14;

// PX3RC_ORGANISM_VISIBLE_BEGIN
mod physics {
    use super::{
        ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
    };

    pub(super) const LANES: usize = 4;
    pub(super) const SITES: usize = 4;
    const SOURCE_THRESHOLD: usize = 4;
    const ACQUISITION_USES: usize = 4;
    const ACQUISITION_SPACING: i64 = 16;
    const TRAVEL: i64 = 4;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(super) struct Shape {
        pub namespace: u64,
        pub mirror: bool,
        pub reverse_allocation: bool,
        pub reverse_arrival: bool,
        pub id_rotation: usize,
        pub distractors: usize,
        pub spacing: i64,
    }

    #[derive(Clone)]
    pub(super) struct Matter {
        substrate: PlasticSubstrate,
        shape: Shape,
        arrivals: [CellId; LANES],
        correspondence_ends: [CellId; LANES],
        continuations: [CellId; LANES],
        consequences: [CellId; LANES],
        acquisition_drivers: [CellId; LANES],
        participation_drivers: [CellId; LANES],
        approaches: [[CellId; SITES]; LANES],
        sites: [CellId; SITES],
        site_drivers: [CellId; SITES],
        distractor_cells: Vec<CellId>,
        directional: [Option<ArrowId>; LANES],
        tick: i64,
        opportunity_enabled: bool,
        opportunities_added: u64,
        acquisition_work: WorkLedger,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub(super) struct Counts {
        pub continuations: [usize; LANES],
        pub consequences: [usize; LANES],
        pub traces: [usize; LANES],
        pub route_effects: [usize; LANES],
        pub site_effects: [usize; SITES],
        pub site_input_max: [i32; SITES],
        pub extra_sources: usize,
        pub quiescent: bool,
        pub work: WorkLedger,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct State {
        pub correspondence: [u32; LANES],
        pub direction: [u32; LANES],
        pub direction_live: [bool; LANES],
        pub opportunity_resistance: [[u32; SITES]; LANES],
        pub opportunity_live: [[bool; SITES]; LANES],
        pub opportunity_impulse: [[i32; SITES]; LANES],
        pub arrow_count: usize,
        pub persistent_bytes: usize,
        pub permanent_fingerprint: u64,
        pub complete_fingerprint: u64,
        pub opportunities_added: u64,
        pub measurement_work: WorkLedger,
    }

    pub(super) fn fresh(shape: Shape, returns: bool, opportunities: bool) -> Matter {
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
        let mut sites = [None; SITES];
        let mut site_drivers = [None; SITES];
        let mut site_outside = [None; SITES];
        let mut approaches = [[None; SITES]; LANES];
        let order = if shape.reverse_allocation {
            [3usize, 2, 1, 0]
        } else {
            [0usize, 1, 2, 3]
        };
        let sign = if shape.mirror { -1 } else { 1 };

        for lane in order {
            let base = sign * lane as i32 * 36;
            let physical = physical_lane(&shape, lane);
            arrivals[lane] =
                Some(substrate.add_cell(cell(shape.namespace + 0x100 + physical, base, 0, 4)));
            correspondence_ends[lane] = Some(substrate.add_cell(cell(
                shape.namespace + 0x200 + physical,
                base + sign * 2,
                0,
                2,
            )));
            continuations[lane] = Some(substrate.add_cell(cell(
                shape.namespace + 0x300 + physical,
                base + sign * 8,
                0,
                2,
            )));
            consequences[lane] = Some(substrate.add_cell(cell(
                shape.namespace + 0x400 + physical,
                base + sign * 10,
                0,
                2,
            )));
            traces[lane] = Some(substrate.add_cell(cell(
                shape.namespace + 0x500 + physical,
                base + sign * 16,
                0,
                2,
            )));
            outside[lane] = Some(substrate.add_cell(cell(
                shape.namespace + 0x600 + physical,
                30_000 + base,
                1,
                1,
            )));
            acquisition_drivers[lane] = Some(substrate.add_cell(cell(
                shape.namespace + 0x700 + physical,
                31_000 + base,
                0,
                1,
            )));
            participation_drivers[lane] = Some(substrate.add_cell(cell(
                shape.namespace + 0x800 + physical,
                32_000 + base,
                0,
                1,
            )));
            gates[lane] = Some(substrate.add_cell(cell(
                shape.namespace + 0x900 + physical,
                33_000 + base,
                0,
                1,
            )));
        }

        for site in order {
            let physical = physical_site(&shape, site);
            sites[site] = Some(substrate.add_cell(cell(
                shape.namespace + 0xa00 + physical,
                sign * (10_000 + site as i32 * 20),
                0,
                3,
            )));
            site_drivers[site] = Some(substrate.add_cell(cell(
                shape.namespace + 0xb00 + physical,
                sign * (20_000 + site as i32 * 20),
                0,
                1,
            )));
            site_outside[site] = Some(substrate.add_cell(cell(
                shape.namespace + 0xc00 + physical,
                sign * (40_000 + site as i32 * 20),
                1,
                1,
            )));
        }

        for lane in order {
            for site in order {
                let physical_lane = physical_lane(&shape, lane);
                let physical_site = physical_site(&shape, site);
                approaches[lane][site] = Some(substrate.add_cell(cell(
                    shape.namespace + 0x1000 + physical_lane * 0x10 + physical_site,
                    sign * (50_000 + lane as i32 * 100 + site as i32 * 10),
                    0,
                    2,
                )));
            }
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
        let sites = sites.map(|value| value.expect("site"));
        let site_drivers = site_drivers.map(|value| value.expect("driver"));
        let site_outside = site_outside.map(|value| value.expect("outside"));
        let approaches = approaches.map(|row| row.map(|value| value.expect("approach")));
        let hub = substrate.add_cell(cell(shape.namespace + 0xd00, sign * 60_000, 0, 1));

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
            substrate.add_arrow(arrow(consequences[lane], traces[lane], 1, 1, 1_000));
            substrate.add_arrow(arrow(consequences[lane], hub, 1, 1, 1_000));
            substrate.add_arrow(arrow(consequences[lane], outside[lane], 0, 1, 1_000));
            substrate.add_arrow(arrow(traces[lane], continuations[lane], 1, 1, 1_000));
            substrate.add_arrow(arrow(hub, traces[lane], 0, 1, 1_000));
            for approach in approaches[lane].iter().copied() {
                substrate.add_arrow(arrow(continuations[lane], approach, 1, 2, 1_000));
            }
        }
        for site in order {
            substrate.add_arrow(arrow(site_drivers[site], sites[site], 6, 1, 1_000));
            substrate.add_arrow(arrow(sites[site], site_outside[site], 0, 1, 1_000));
            if returns {
                for row in &approaches {
                    substrate.add_arrow(arrow(sites[site], row[site], 1, 1, 1_000));
                }
            }
        }

        let mut distractor_cells = Vec::new();
        for ordinal in 0..shape.distractors {
            distractor_cells.push(substrate.add_cell(cell(
                shape.namespace + 0x3000 + ordinal as u64,
                sign * (70_000 + ordinal as i32 * 10),
                0,
                1,
            )));
        }

        let mut matter = Matter {
            substrate,
            shape,
            arrivals,
            correspondence_ends,
            continuations,
            consequences,
            acquisition_drivers,
            participation_drivers,
            approaches,
            sites,
            site_drivers,
            distractor_cells,
            directional: [None; LANES],
            tick: 0,
            opportunity_enabled: opportunities,
            opportunities_added: 0,
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
        expose(&mut matter);
        matter
    }

    pub(super) fn drive(
        matter: &mut Matter,
        tick: i64,
        lanes: &[usize],
        sites: &[usize],
        expose_now: bool,
    ) -> Counts {
        assert!(tick >= matter.tick, "arrival cannot precede matter time");
        if expose_now {
            expose(matter);
        }
        let lane_order = ordered(lanes, matter.shape.reverse_arrival);
        let site_order = ordered(sites, matter.shape.reverse_arrival);
        for lane in lane_order.iter().copied() {
            enter_many(
                &mut matter.substrate,
                matter.arrivals[lane],
                tick,
                SOURCE_THRESHOLD,
                matter.shape.namespace + 0x10_000 + lane as u64 * 0x10,
            );
            enter_many(
                &mut matter.substrate,
                matter.participation_drivers[lane],
                tick,
                1,
                matter.shape.namespace + 0x20_000 + lane as u64 * 0x10,
            );
        }
        for site in site_order {
            enter_many(
                &mut matter.substrate,
                matter.site_drivers[site],
                tick,
                1,
                matter.shape.namespace + 0x30_000 + site as u64 * 0x10,
            );
        }
        for (ordinal, target) in matter.distractor_cells.iter().copied().enumerate() {
            enter_many(
                &mut matter.substrate,
                target,
                tick,
                1,
                matter.shape.namespace + 0x40_000 + ordinal as u64 * 0x10,
            );
        }
        let run = matter.substrate.propagate();
        matter.tick = tick + 7;
        counts(matter, &run, lanes)
    }

    pub(super) fn observe(matter: &Matter, lanes: &[usize]) -> Counts {
        let mut copy = matter.clone();
        drive(&mut copy, matter.tick, lanes, &[], false)
    }

    pub(super) fn settle(matter: &mut Matter, ticks: i64) -> WorkLedger {
        let target = matter.tick + ticks;
        let work = matter.substrate.advance_time(target);
        matter.tick = target;
        work
    }

    pub(super) fn state(matter: &Matter) -> State {
        let opportunity_resistance = std::array::from_fn(|lane| {
            std::array::from_fn(|site| {
                live_arrows(matter, lane, site)
                    .into_iter()
                    .map(|arrow| matter.substrate.arrow_resistance(arrow))
                    .max()
                    .unwrap_or(0)
            })
        });
        let opportunity_live = std::array::from_fn(|lane| {
            std::array::from_fn(|site| !live_arrows(matter, lane, site).is_empty())
        });
        let mut opportunity_impulse = [[0; SITES]; LANES];
        let mut measurement_work = WorkLedger::default();
        for (lane, impulses) in opportunity_impulse.iter_mut().enumerate() {
            let mut copy = matter.clone();
            let observed = drive(&mut copy, matter.tick, &[lane], &[], false);
            add_work(&mut measurement_work, &observed.work);
            *impulses = observed.site_input_max;
        }
        State {
            correspondence: std::array::from_fn(|lane| {
                matter
                    .substrate
                    .arrows_between(matter.arrivals[lane], matter.correspondence_ends[lane])
                    .into_iter()
                    .filter(|arrow| matter.substrate.arrow_is_live(*arrow))
                    .map(|arrow| matter.substrate.arrow_resistance(arrow))
                    .max()
                    .unwrap_or(0)
            }),
            direction: std::array::from_fn(|lane| {
                matter
                    .substrate
                    .arrow_resistance(matter.directional[lane].expect("direction"))
            }),
            direction_live: std::array::from_fn(|lane| {
                matter
                    .substrate
                    .arrow_is_live(matter.directional[lane].expect("direction"))
            }),
            opportunity_resistance,
            opportunity_live,
            opportunity_impulse,
            arrow_count: matter.substrate.arrow_count(),
            persistent_bytes: matter.substrate.persistent_bytes(),
            permanent_fingerprint: matter.substrate.permanent_fingerprint(),
            complete_fingerprint: matter.substrate.complete_fingerprint(),
            opportunities_added: matter.opportunities_added,
            measurement_work,
        }
    }

    pub(super) fn acquisition_work(matter: &Matter) -> WorkLedger {
        matter.acquisition_work.clone()
    }

    pub(super) fn tick(matter: &Matter) -> i64 {
        matter.tick
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
                    matter.shape.namespace
                        + 0x50_000
                        + use_ordinal as u64 * 0x100
                        + lane as u64 * 0x10,
                );
                enter_many(
                    &mut matter.substrate,
                    matter.acquisition_drivers[lane],
                    tick,
                    1,
                    matter.shape.namespace
                        + 0x60_000
                        + use_ordinal as u64 * 0x100
                        + lane as u64 * 0x10,
                );
            }
        }
        let run = matter.substrate.propagate();
        assert!(run.naturally_quiescent, "finite acquisition must drain");
        matter.tick = 53;
        run.work
    }

    fn expose(matter: &mut Matter) {
        if !matter.opportunity_enabled {
            return;
        }
        for lane in 0..LANES {
            for site in 0..SITES {
                if live_arrows(matter, lane, site).is_empty() {
                    matter.substrate.add_arrow(arrow(
                        matter.approaches[lane][site],
                        matter.sites[site],
                        1,
                        1,
                        1,
                    ));
                    matter.opportunities_added += 1;
                }
            }
        }
    }

    fn live_arrows(matter: &Matter, lane: usize, site: usize) -> Vec<ArrowId> {
        matter
            .substrate
            .arrows_between(matter.approaches[lane][site], matter.sites[site])
            .into_iter()
            .filter(|arrow| matter.substrate.arrow_is_live(*arrow))
            .collect()
    }

    fn counts(matter: &Matter, run: &Execution, active: &[usize]) -> Counts {
        let continuations = std::array::from_fn(|lane| {
            firings_at(
                run,
                matter.shape.namespace + 0x300 + physical_lane(&matter.shape, lane),
            )
        });
        let consequences = std::array::from_fn(|lane| {
            firings_at(
                run,
                matter.shape.namespace + 0x400 + physical_lane(&matter.shape, lane),
            )
        });
        let traces = std::array::from_fn(|lane| {
            firings_at(
                run,
                matter.shape.namespace + 0x500 + physical_lane(&matter.shape, lane),
            )
        });
        let route_effects = std::array::from_fn(|lane| {
            crossings_from(
                run,
                matter.shape.namespace + 0x400 + physical_lane(&matter.shape, lane),
            )
        });
        let site_effects = std::array::from_fn(|site| {
            crossings_from(
                run,
                matter.shape.namespace + 0xa00 + physical_site(&matter.shape, site),
            )
        });
        let site_input_max = std::array::from_fn(|site| {
            run.trace
                .iter()
                .filter(|entry| {
                    entry.target_physical
                        == matter.shape.namespace + 0xa00 + physical_site(&matter.shape, site)
                })
                .map(|entry| entry.impulse)
                .max()
                .unwrap_or(0)
        });
        let source_firings = (0..LANES)
            .map(|lane| {
                firings_at(
                    run,
                    matter.shape.namespace + 0x100 + physical_lane(&matter.shape, lane),
                )
            })
            .sum::<usize>();
        Counts {
            continuations,
            consequences,
            traces,
            route_effects,
            site_effects,
            site_input_max,
            extra_sources: source_firings.saturating_sub(active.len()),
            quiescent: run.naturally_quiescent,
            work: run.work.clone(),
        }
    }

    fn physical_lane(shape: &Shape, lane: usize) -> u64 {
        ((lane + shape.id_rotation) % LANES) as u64
    }

    fn physical_site(shape: &Shape, site: usize) -> u64 {
        ((site + shape.id_rotation * 3) % SITES) as u64
    }

    fn ordered(values: &[usize], reverse: bool) -> Vec<usize> {
        let mut result = values.to_vec();
        if reverse {
            result.reverse();
        }
        result
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

    fn crossings_from(run: &Execution, physical: u64) -> usize {
        run.crossings
            .iter()
            .filter(|crossing| crossing.from_physical == physical && crossing.to_region == 1)
            .count()
    }

    pub(super) fn add_work(total: &mut WorkLedger, additional: &WorkLedger) {
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
// PX3RC_ORGANISM_VISIBLE_END

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Probe,
    Micro,
    Gate,
}

impl Stage {
    fn paths(self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            Self::Probe => (PROBE_CSV, PROBE_MD, PROBE_STAGING_CSV, PROBE_STAGING_MD),
            Self::Micro => (MICRO_CSV, MICRO_MD, MICRO_STAGING_CSV, MICRO_STAGING_MD),
            Self::Gate => (GATE_CSV, GATE_MD, GATE_STAGING_CSV, GATE_STAGING_MD),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Probe => "PROBE",
            Self::Micro => "MICRO",
            Self::Gate => "GATE",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Totals {
    continuations: [usize; physics::LANES],
    consequences: [usize; physics::LANES],
    traces: [usize; physics::LANES],
    route_effects: [usize; physics::LANES],
    site_effects: [usize; physics::SITES],
    extra_sources: usize,
    quiescent: bool,
    work: WorkLedger,
}

#[derive(Clone)]
struct Acquisition {
    shape: physics::Shape,
    totals: Totals,
    state: physics::State,
    trained_use: [usize; 2],
    crossed_use: [usize; 2],
    individual_use: [usize; physics::LANES],
    trained_common: [usize; 2],
    crossed_common: [usize; 2],
    matter: physics::Matter,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    stage: &'static str,
    cell: String,
    namespace: u64,
    correspondence: [u32; physics::LANES],
    direction: [u32; physics::LANES],
    opportunity: [[u32; physics::SITES]; physics::LANES],
    impulse: [[i32; physics::SITES]; physics::LANES],
    trained_use: [usize; 2],
    crossed_use: [usize; 2],
    individual_use: [usize; physics::LANES],
    trained_common: [usize; 2],
    crossed_common: [usize; 2],
    old_score_before: u32,
    old_score_after: u32,
    new_score_after: u32,
    swap_new_use: [usize; 2],
    swap_old_use: [usize; 2],
    controls: String,
    opportunities_added: u64,
    arrow_count: usize,
    persistent_bytes: usize,
    work: u64,
    fingerprint: u64,
    duplicate_exact: bool,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Report {
    stage: Stage,
    rows: Vec<Row>,
    first_collapse: &'static str,
    passed: bool,
    total_work: u64,
    total_storage: usize,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let stage = match args.as_slice() {
        [arg] if arg == "--preflight" => None,
        [arg] if arg == "--probe" => Some(Stage::Probe),
        [arg] if arg == "--micro" => Some(Stage::Micro),
        [arg] if arg == "--gate" => Some(Stage::Gate),
        _ => {
            eprintln!("PX3-R Arm C requires --preflight, --probe, --micro, or --gate");
            std::process::exit(2);
        }
    };
    assert!(
        source_audit(),
        "frozen inputs and information boundary must be exact"
    );
    if let Some(stage) = stage {
        require_paths_absent(stage);
        eprintln!("PX3_R_C_{}_DEVELOPMENT_EVIDENCE_SPENT", stage.name());
        let report = run_stage(stage);
        write_atomic(&report);
        println!(
            "PX3_R_C_{}_{}",
            stage.name(),
            if report.passed {
                "PASS"
            } else {
                "FIRST_COLLAPSE"
            }
        );
        if !report.passed {
            std::process::exit(1);
        }
    } else {
        for stage in [Stage::Probe, Stage::Micro, Stage::Gate] {
            require_paths_absent(stage);
        }
        println!("PX3_R_C_DOWNSTREAM_CONVERGENCE_PREFLIGHT_OK");
    }
}

fn run_stage(stage: Stage) -> Report {
    match stage {
        Stage::Probe => run_probe(),
        Stage::Micro => run_micro(),
        Stage::Gate => run_gate(),
    }
}

fn base_shape(namespace: u64) -> physics::Shape {
    physics::Shape {
        namespace,
        mirror: false,
        reverse_allocation: false,
        reverse_arrival: false,
        id_rotation: 0,
        distractors: 0,
        spacing: 8,
    }
}

fn acquire_initial(shape: physics::Shape, returns: bool, opportunities: bool) -> Acquisition {
    let mut matter = physics::fresh(shape, returns, opportunities);
    let mut totals = Totals::default();
    physics::add_work(&mut totals.work, &physics::acquisition_work(&matter));
    totals.quiescent = true;
    for recurrence in 0..RECURRENCES {
        let base = FIRST_USE + recurrence as i64 * shape.spacing * 2;
        let entries = if recurrence.is_multiple_of(2) {
            [
                ([0usize, 1], 0usize, base),
                ([2usize, 3], 1usize, base + shape.spacing),
            ]
        } else {
            [
                ([2usize, 3], 1usize, base),
                ([0usize, 1], 0usize, base + shape.spacing),
            ]
        };
        for (lanes, site, tick) in entries {
            add_counts(
                &mut totals,
                &physics::drive(&mut matter, tick, &lanes, &[site], true),
            );
        }
    }
    physics::add_work(
        &mut totals.work,
        &physics::settle(&mut matter, HELD_OUT_GAP),
    );
    let state = physics::state(&matter);
    physics::add_work(&mut totals.work, &state.measurement_work);
    let trained_use = [
        site_crossings(&physics::observe(&matter, &[0, 1])),
        site_crossings(&physics::observe(&matter, &[2, 3])),
    ];
    let crossed_use = [
        site_crossings(&physics::observe(&matter, &[0, 3])),
        site_crossings(&physics::observe(&matter, &[2, 1])),
    ];
    let individual_use =
        std::array::from_fn(|lane| site_crossings(&physics::observe(&matter, &[lane])));
    let trained_common = [common_count(&state, 0, 1), common_count(&state, 2, 3)];
    let crossed_common = [common_count(&state, 0, 3), common_count(&state, 2, 1)];
    Acquisition {
        shape,
        totals,
        state,
        trained_use,
        crossed_use,
        individual_use,
        trained_common,
        crossed_common,
        matter,
    }
}

fn run_probe() -> Report {
    let shape = base_shape(0x9_4300_0000);
    let main = acquire_initial(shape, true, true);
    let duplicate = acquire_initial(shape, true, true);
    let no_return = acquire_initial(base_shape(0x9_4310_0000), false, true);
    let absent = acquire_initial(base_shape(0x9_4320_0000), true, false);
    let correlation = correlation_control(base_shape(0x9_4330_0000));
    let controls_pass =
        no_return.trained_use == [0, 0] && absent.trained_use == [0, 0] && correlation == 0;
    let duplicate_exact = acquisition_equal(&main, &duplicate);
    let passed = acquisition_pass(&main) && controls_pass && duplicate_exact;
    let row = row_from_acquisition(
        Stage::Probe,
        "matched-marginal",
        &main,
        format!(
            "correlation={correlation}|without_return={:?}|absent={:?}",
            no_return.trained_use, absent.trained_use
        ),
        duplicate_exact,
        passed,
    );
    report(Stage::Probe, vec![row])
}

fn run_micro() -> Report {
    let shape = base_shape(0x9_5300_0000);
    let initial = acquire_initial(shape, true, true);
    let duplicate_initial = acquire_initial(shape, true, true);
    let old_score_before = old_score(&initial.state);
    let mut matter = initial.matter.clone();
    let mut totals = initial.totals.clone();
    train_swap(&mut matter, &mut totals, shape);
    physics::add_work(
        &mut totals.work,
        &physics::settle(&mut matter, HELD_OUT_GAP),
    );
    let state = physics::state(&matter);
    physics::add_work(&mut totals.work, &state.measurement_work);
    let swap_new_use = [
        site_crossings(&physics::observe(&matter, &[0, 3])),
        site_crossings(&physics::observe(&matter, &[2, 1])),
    ];
    let swap_old_use = [
        site_crossings(&physics::observe(&matter, &[0, 1])),
        site_crossings(&physics::observe(&matter, &[2, 3])),
    ];
    let old_score_after = old_score(&state);
    let new_score_after = new_score(&state);
    let stale = stale_control(&initial.matter);
    let ambiguous = ambiguous_control(base_shape(0x9_5310_0000));
    let multiple = multiple_control(base_shape(0x9_5320_0000));
    let no_return = acquire_initial(base_shape(0x9_5330_0000), false, true);
    let absent = acquire_initial(base_shape(0x9_5340_0000), true, false);
    let correlation = correlation_control(base_shape(0x9_5350_0000));
    let duplicate_exact = acquisition_equal(&initial, &duplicate_initial);
    let controls_pass = stale == 0
        && ambiguous.0
        && multiple
        && no_return.trained_use == [0, 0]
        && absent.trained_use == [0, 0]
        && correlation == 0;
    let passed = acquisition_pass(&initial)
        && old_score_before > 0
        && old_score_after < old_score_before
        && new_score_after > 0
        && swap_new_use == [1, 1]
        && swap_old_use == [0, 0]
        && route_equal(&state)
        && totals.quiescent
        && totals.extra_sources == 0
        && controls_pass
        && duplicate_exact;
    let mut row = row_from_acquisition(
        Stage::Micro,
        "swap-and-controls",
        &initial,
        format!(
            "stale={stale}|ambiguous_symmetric={}|ambiguous_sites={:?}|multiple={multiple}|correlation={correlation}|without_return={:?}|absent={:?}",
            ambiguous.0, ambiguous.1, no_return.trained_use, absent.trained_use
        ),
        duplicate_exact,
        passed,
    );
    row.correspondence = state.correspondence;
    row.direction = state.direction;
    row.opportunity = state.opportunity_resistance;
    row.impulse = state.opportunity_impulse;
    row.old_score_before = old_score_before;
    row.old_score_after = old_score_after;
    row.new_score_after = new_score_after;
    row.swap_new_use = swap_new_use;
    row.swap_old_use = swap_old_use;
    row.opportunities_added = state.opportunities_added;
    row.arrow_count = state.arrow_count;
    row.persistent_bytes = state.persistent_bytes;
    row.work = totals.work.total();
    row.fingerprint = state.permanent_fingerprint;
    report(Stage::Micro, vec![row])
}

fn run_gate() -> Report {
    let shapes = [
        physics::Shape {
            namespace: 0x9_6300_0000,
            mirror: false,
            reverse_allocation: false,
            reverse_arrival: false,
            id_rotation: 0,
            distractors: 0,
            spacing: 8,
        },
        physics::Shape {
            namespace: 0x9_6310_0000,
            mirror: true,
            reverse_allocation: true,
            reverse_arrival: false,
            id_rotation: 1,
            distractors: 8,
            spacing: 11,
        },
        physics::Shape {
            namespace: 0x9_6320_0000,
            mirror: false,
            reverse_allocation: true,
            reverse_arrival: true,
            id_rotation: 2,
            distractors: 16,
            spacing: 8,
        },
        physics::Shape {
            namespace: 0x9_6330_0000,
            mirror: true,
            reverse_allocation: false,
            reverse_arrival: true,
            id_rotation: 3,
            distractors: 24,
            spacing: 11,
        },
    ];
    let mut rows = Vec::new();
    for (ordinal, shape) in shapes.into_iter().enumerate() {
        let initial = acquire_initial(shape, true, true);
        let duplicate = acquire_initial(shape, true, true);
        let old_score_before = old_score(&initial.state);
        let duplicate_exact = acquisition_equal(&initial, &duplicate);
        let mut matter = initial.matter.clone();
        let mut totals = initial.totals.clone();
        train_swap(&mut matter, &mut totals, shape);
        physics::add_work(
            &mut totals.work,
            &physics::settle(&mut matter, HELD_OUT_GAP),
        );
        let state = physics::state(&matter);
        physics::add_work(&mut totals.work, &state.measurement_work);
        let swap_new_use = [
            site_crossings(&physics::observe(&matter, &[0, 3])),
            site_crossings(&physics::observe(&matter, &[2, 1])),
        ];
        let swap_old_use = [
            site_crossings(&physics::observe(&matter, &[0, 1])),
            site_crossings(&physics::observe(&matter, &[2, 3])),
        ];
        let old_score_after = old_score(&state);
        let new_score_after = new_score(&state);
        let passed = acquisition_pass(&initial)
            && old_score_after < old_score_before
            && new_score_after > 0
            && swap_new_use == [1, 1]
            && swap_old_use == [0, 0]
            && route_equal(&state)
            && totals.quiescent
            && totals.extra_sources == 0
            && duplicate_exact;
        let mut row = row_from_acquisition(
            Stage::Gate,
            &format!("gate-{ordinal}"),
            &initial,
            format!(
                "mirror={}|reverse_allocation={}|reverse_arrival={}|rotation={}|distractors={}|spacing={}",
                shape.mirror,
                shape.reverse_allocation,
                shape.reverse_arrival,
                shape.id_rotation,
                shape.distractors,
                shape.spacing
            ),
            duplicate_exact,
            passed,
        );
        row.correspondence = state.correspondence;
        row.direction = state.direction;
        row.opportunity = state.opportunity_resistance;
        row.impulse = state.opportunity_impulse;
        row.old_score_before = old_score_before;
        row.old_score_after = old_score_after;
        row.new_score_after = new_score_after;
        row.swap_new_use = swap_new_use;
        row.swap_old_use = swap_old_use;
        row.opportunities_added = state.opportunities_added;
        row.arrow_count = state.arrow_count;
        row.persistent_bytes = state.persistent_bytes;
        row.work = totals.work.total();
        row.fingerprint = state.permanent_fingerprint;
        rows.push(row);
    }
    report(Stage::Gate, rows)
}

fn train_swap(matter: &mut physics::Matter, totals: &mut Totals, shape: physics::Shape) {
    let start = physics::tick(matter) + shape.spacing;
    for recurrence in 0..RECURRENCES {
        let base = start + recurrence as i64 * shape.spacing * 2;
        let entries = if recurrence.is_multiple_of(2) {
            [
                ([0usize, 3], 0usize, base),
                ([2usize, 1], 1usize, base + shape.spacing),
            ]
        } else {
            [
                ([2usize, 1], 1usize, base),
                ([0usize, 3], 0usize, base + shape.spacing),
            ]
        };
        for (lanes, site, tick) in entries {
            add_counts(totals, &physics::drive(matter, tick, &lanes, &[site], true));
        }
    }
}

fn correlation_control(shape: physics::Shape) -> usize {
    let mut matter = physics::fresh(shape, true, true);
    let mut last = FIRST_USE;
    for recurrence in 0..RECURRENCES {
        let base = FIRST_USE + recurrence as i64 * shape.spacing * 2;
        physics::drive(&mut matter, base, &[], &[0], true);
        physics::drive(&mut matter, base + shape.spacing, &[], &[1], true);
        last = base + shape.spacing;
    }
    let _ = last;
    physics::settle(&mut matter, HELD_OUT_GAP);
    site_crossings(&physics::observe(&matter, &[0, 1]))
}

fn stale_control(matter: &physics::Matter) -> usize {
    let mut copy = matter.clone();
    physics::settle(&mut copy, 220);
    site_crossings(&physics::observe(&copy, &[0, 1]))
}

fn ambiguous_control(shape: physics::Shape) -> (bool, [u32; physics::SITES]) {
    let mut matter = physics::fresh(shape, true, true);
    for recurrence in 0..RECURRENCES {
        let tick = FIRST_USE + recurrence as i64 * shape.spacing * 2;
        physics::drive(&mut matter, tick, &[0, 1, 2], &[], true);
    }
    physics::settle(&mut matter, HELD_OUT_GAP);
    let state = physics::state(&matter);
    let sums = std::array::from_fn(|site| {
        (0..physics::LANES)
            .map(|lane| state.opportunity_resistance[lane][site])
            .sum()
    });
    (sums.iter().all(|value| *value == sums[0]), sums)
}

fn multiple_control(shape: physics::Shape) -> bool {
    let mut matter = physics::fresh(shape, true, true);
    for recurrence in 0..RECURRENCES {
        let tick = FIRST_USE + recurrence as i64 * shape.spacing * 2;
        physics::drive(&mut matter, tick, &[0, 1, 2, 3], &[0, 1, 2, 3], true);
    }
    physics::settle(&mut matter, HELD_OUT_GAP);
    let state = physics::state(&matter);
    [(0, 1), (2, 3), (0, 3), (2, 1)]
        .into_iter()
        .all(|(left, right)| common_count(&state, left, right) > 0)
}

fn acquisition_pass(value: &Acquisition) -> bool {
    value.totals.continuations == [RECURRENCES; physics::LANES]
        && value.totals.consequences == [RECURRENCES; physics::LANES]
        && value.totals.traces == [RECURRENCES; physics::LANES]
        && value.totals.route_effects == [RECURRENCES; physics::LANES]
        && route_equal(&value.state)
        && value.state.direction_live == [true; physics::LANES]
        && value.trained_use == [1, 1]
        && value.crossed_use == [0, 0]
        && value.individual_use == [0; physics::LANES]
        && value.trained_common == [1, 1]
        && value.crossed_common == [0, 0]
        && value.totals.extra_sources == 0
        && value.totals.quiescent
}

fn route_equal(state: &physics::State) -> bool {
    state
        .correspondence
        .iter()
        .all(|value| *value == state.correspondence[0])
        && state
            .direction
            .iter()
            .all(|value| *value == state.direction[0])
}

fn acquisition_equal(left: &Acquisition, right: &Acquisition) -> bool {
    left.shape == right.shape
        && left.totals == right.totals
        && left.state == right.state
        && left.trained_use == right.trained_use
        && left.crossed_use == right.crossed_use
        && left.individual_use == right.individual_use
        && left.trained_common == right.trained_common
        && left.crossed_common == right.crossed_common
}

fn common_count(state: &physics::State, left: usize, right: usize) -> usize {
    (0..physics::SITES)
        .filter(|site| {
            state.opportunity_live[left][*site]
                && state.opportunity_live[right][*site]
                && state.opportunity_impulse[left][*site] >= 2
                && state.opportunity_impulse[right][*site] >= 2
        })
        .count()
}

fn old_score(state: &physics::State) -> u32 {
    state.opportunity_resistance[0][0].min(state.opportunity_resistance[1][0])
        + state.opportunity_resistance[2][1].min(state.opportunity_resistance[3][1])
}

fn new_score(state: &physics::State) -> u32 {
    state.opportunity_resistance[0][0].min(state.opportunity_resistance[3][0])
        + state.opportunity_resistance[2][1].min(state.opportunity_resistance[1][1])
}

fn site_crossings(counts: &physics::Counts) -> usize {
    counts.site_effects.iter().sum()
}

fn add_counts(total: &mut Totals, additional: &physics::Counts) {
    for lane in 0..physics::LANES {
        total.continuations[lane] += additional.continuations[lane];
        total.consequences[lane] += additional.consequences[lane];
        total.traces[lane] += additional.traces[lane];
        total.route_effects[lane] += additional.route_effects[lane];
    }
    for site in 0..physics::SITES {
        total.site_effects[site] += additional.site_effects[site];
    }
    total.extra_sources += additional.extra_sources;
    total.quiescent &= additional.quiescent;
    physics::add_work(&mut total.work, &additional.work);
}

fn row_from_acquisition(
    stage: Stage,
    cell: &str,
    value: &Acquisition,
    controls: String,
    duplicate_exact: bool,
    passed: bool,
) -> Row {
    Row {
        stage: stage.name(),
        cell: cell.to_string(),
        namespace: value.shape.namespace,
        correspondence: value.state.correspondence,
        direction: value.state.direction,
        opportunity: value.state.opportunity_resistance,
        impulse: value.state.opportunity_impulse,
        trained_use: value.trained_use,
        crossed_use: value.crossed_use,
        individual_use: value.individual_use,
        trained_common: value.trained_common,
        crossed_common: value.crossed_common,
        old_score_before: 0,
        old_score_after: 0,
        new_score_after: 0,
        swap_new_use: [0, 0],
        swap_old_use: [0, 0],
        controls,
        opportunities_added: value.state.opportunities_added,
        arrow_count: value.state.arrow_count,
        persistent_bytes: value.state.persistent_bytes,
        work: value.totals.work.total(),
        fingerprint: value.state.permanent_fingerprint,
        duplicate_exact,
        passed,
    }
}

fn report(stage: Stage, rows: Vec<Row>) -> Report {
    let passed = !rows.is_empty() && rows.iter().all(|row| row.passed);
    let total_work = rows.iter().map(|row| row.work).sum();
    let total_storage = rows.iter().map(|row| row.persistent_bytes).sum();
    Report {
        stage,
        rows,
        first_collapse: if passed { "NONE" } else { "STAGE_CONJUNCTION" },
        passed,
        total_work,
        total_storage,
    }
}

fn require_paths_absent(stage: Stage) {
    let (csv, md, staging_csv, staging_md) = stage.paths();
    for path in [csv, md, staging_csv, staging_md] {
        assert!(!Path::new(path).exists(), "stage artifact exists: {path}");
    }
}

fn write_atomic(report: &Report) {
    let (csv, md, staging_csv, staging_md) = report.stage.paths();
    let mut csv_file = exclusive(staging_csv);
    writeln!(csv_file, "stage,cell,namespace,correspondence,direction,opportunity,impulse,trained_use,crossed_use,individual_use,trained_common,crossed_common,old_score_before,old_score_after,new_score_after,swap_new_use,swap_old_use,controls,opportunities_added,arrow_count,persistent_bytes,work,fingerprint,duplicate_exact,passed").expect("write CSV header");
    for row in &report.rows {
        writeln!(
            csv_file,
            "{},{},{:#x},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            row.stage,
            row.cell,
            row.namespace,
            quoted(&format!("{:?}", row.correspondence)),
            quoted(&format!("{:?}", row.direction)),
            quoted(&format!("{:?}", row.opportunity)),
            quoted(&format!("{:?}", row.impulse)),
            quoted(&format!("{:?}", row.trained_use)),
            quoted(&format!("{:?}", row.crossed_use)),
            quoted(&format!("{:?}", row.individual_use)),
            quoted(&format!("{:?}", row.trained_common)),
            quoted(&format!("{:?}", row.crossed_common)),
            row.old_score_before,
            row.old_score_after,
            row.new_score_after,
            quoted(&format!("{:?}", row.swap_new_use)),
            quoted(&format!("{:?}", row.swap_old_use)),
            quoted(&row.controls),
            row.opportunities_added,
            row.arrow_count,
            row.persistent_bytes,
            row.work,
            row.fingerprint,
            row.duplicate_exact,
            row.passed
        )
        .expect("write CSV row");
    }
    csv_file.flush().expect("flush CSV staging");

    let mut md_file = exclusive(staging_md);
    writeln!(
        md_file,
        "# PX3-R Arm C {} development result\n\nVerdict: **{}**. PX3 remains absent; this is not definitive evidence.\n\n- first collapse: `{}`\n- rows: `{}`\n- total ledgered work: `{}`\n- summed persistent storage: `{}` bytes\n\n| cell | trained use | crossed use | trained common | crossed common | old before/after | new after | swap new/old | duplicate | pass |\n|---|---:|---:|---:|---:|---:|---:|---:|:---:|:---:|",
        report.stage.name(),
        if report.passed { "PASS" } else { "FROZEN NEGATIVE" },
        report.first_collapse,
        report.rows.len(),
        report.total_work,
        report.total_storage
    )
    .expect("write report header");
    for row in &report.rows {
        writeln!(
            md_file,
            "| {} | {:?} | {:?} | {:?} | {:?} | {}/{} | {} | {:?}/{:?} | {} | {} |",
            row.cell,
            row.trained_use,
            row.crossed_use,
            row.trained_common,
            row.crossed_common,
            row.old_score_before,
            row.old_score_after,
            row.new_score_after,
            row.swap_new_use,
            row.swap_old_use,
            row.duplicate_exact,
            row.passed
        )
        .expect("write report row");
    }
    writeln!(md_file, "\n## Serialized physical state").expect("write state heading");
    for row in &report.rows {
        writeln!(
            md_file,
            "\n### {}\n\n- individual correspondence resistance: `{:?}`\n- individual direction resistance: `{:?}`\n- opportunity resistance: `{:?}`\n- measured opportunity impulse: `{:?}`\n- controls: `{}`\n- opportunity additions / ARROW count / persistent bytes: `{}/{}/{}`\n- permanent fingerprint: `{}`",
            row.cell,
            row.correspondence,
            row.direction,
            row.opportunity,
            row.impulse,
            row.controls,
            row.opportunities_added,
            row.arrow_count,
            row.persistent_bytes,
            row.fingerprint
        )
        .expect("write state");
    }
    md_file.flush().expect("flush report staging");
    rename(staging_csv, csv).expect("publish CSV atomically");
    rename(staging_md, md).expect("publish report atomically");
}

fn exclusive(path: &str) -> std::fs::File {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap_or_else(|error| panic!("create staging {path}: {error}"))
}

fn quoted(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
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
            PX3_NEGATIVE_SOURCE_SHA256,
        ),
        (
            "experiments/px3_physical_event_boundaries_frozen_negative_handoff.md",
            PX3_NEGATIVE_HANDOFF_SHA256,
        ),
        (
            "results/px3_physical_event_boundaries_no_new_mechanism_probe_v3.csv",
            PX3_NEGATIVE_CSV_SHA256,
        ),
        (
            "results/px3_physical_event_boundaries_no_new_mechanism_probe_v3.md",
            PX3_NEGATIVE_REPORT_SHA256,
        ),
        (FIRST_COLLAPSE, FIRST_COLLAPSE_SHA256),
        (PROTOCOL, PROTOCOL_SHA256),
    ];
    let hashes_exact = hashes
        .into_iter()
        .all(|(path, hash)| sha256(path).as_deref() == Some(hash));
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
    let source = include_str!("px3_r_c_downstream_convergence.rs");
    let physical = source
        .split("// PX3RC_ORGANISM_VISIBLE_BEGIN")
        .nth(1)
        .and_then(|text| text.split("// PX3RC_ORGANISM_VISIBLE_END").next())
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
        "old_m3",
        "adapter",
        "co_occurrence",
        "scenario",
        "expected",
        "evaluator",
        "trained",
        "crossed",
    ];
    hashes_exact && ancestry_exact && forbidden.iter().all(|token| !physical.contains(token))
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
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8(output.stdout).ok()?.trim().to_owned())
}
