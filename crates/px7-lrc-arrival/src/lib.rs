#![forbid(unsafe_code)]
//! One physical participation surface over retained LR-C dynamics.

use lr1_modulatory_physical_return::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
    TransmissionMode, WorkLedger,
};

pub const SITE_COUNT: usize = 10;
pub const BOUNDARY_A: usize = 0;
pub const BOUNDARY_B: usize = 1;
pub const BOUNDARY_C: usize = 2;
pub const BOUNDARY_D: usize = 3;
pub const INNER_ZERO: usize = 4;
pub const DOWNSTREAM_ZERO: usize = 5;
pub const INNER_ONE: usize = 6;
pub const DOWNSTREAM_ONE: usize = 7;
pub const RETURN_SITE: usize = 8;
pub const OUTWARD_SITE: usize = 9;

const SOURCE_BASE: u64 = 100;
const TRACE_BASE: u64 = 200;
const COINCIDENCE_BASE: u64 = 300;
const INNER_BASE: u64 = 400;
const DOWNSTREAM_BASE: u64 = 500;
const DOWNSTREAM_TRACE_BASE: u64 = 600;
const RETURN_SITE_SUFFIX: u64 = 700;
const RETURN_RELAY_BASE: u64 = 800;
const OUTWARD_SUFFIX: u64 = 900;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Form {
    pub namespace: u64,
    pub reverse_construction: bool,
    pub reflected_positions: bool,
    pub altered_pairing: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arrival {
    pub tick: i64,
    pub phase: i32,
    pub origin: u64,
    pub position: i32,
    pub impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Activity {
    pub execution: Execution,
}

impl Activity {
    pub fn work(&self) -> u64 {
        self.execution.work.total()
    }

    pub fn naturally_quiescent(&self) -> bool {
        self.execution.naturally_quiescent
    }
}

#[derive(Clone)]
pub struct Body {
    substrate: PlasticSubstrate,
    form: Form,
    sites: [CellId; SITE_COUNT],
    weak_links: [ArrowId; 2],
}

impl Body {
    pub fn new(form: Form) -> Self {
        let mut substrate = PlasticSubstrate::new();
        let mut sources = [None; 4];
        let mut traces = [None; 4];
        let boundary_order = if form.reverse_construction {
            [3, 2, 1, 0]
        } else {
            [0, 1, 2, 3]
        };
        for side in boundary_order {
            sources[side] = Some(substrate.add_cell(cell(
                physical(form.namespace, SOURCE_BASE + side as u64),
                coordinate(form, side),
                10 + side as i16,
                1,
            )));
            traces[side] = Some(substrate.add_cell(cell(
                physical(form.namespace, TRACE_BASE + side as u64),
                internal_position(form, 20 + side as i32),
                20 + side as i16,
                1,
            )));
        }
        let sources = sources.map(|value| value.expect("boundary cell"));
        let traces = traces.map(|value| value.expect("boundary trace"));

        let mut coincidence = [None; 2];
        let mut inner = [None; 2];
        let mut downstream = [None; 2];
        let mut downstream_trace = [None; 2];
        let mut return_relay = [None; 2];
        let stage_order = if form.reverse_construction {
            [1, 0]
        } else {
            [0, 1]
        };
        for stage in stage_order {
            coincidence[stage] = Some(substrate.add_cell(cell(
                physical(form.namespace, COINCIDENCE_BASE + stage as u64),
                internal_position(form, 40 + stage as i32),
                30 + stage as i16,
                2,
            )));
            inner[stage] = Some(substrate.add_cell(cell(
                physical(form.namespace, INNER_BASE + stage as u64),
                coordinate(form, INNER_ZERO + stage * 2),
                40 + stage as i16,
                1,
            )));
            downstream[stage] = Some(substrate.add_cell(cell(
                physical(form.namespace, DOWNSTREAM_BASE + stage as u64),
                coordinate(form, DOWNSTREAM_ZERO + stage * 2),
                50 + stage as i16,
                2,
            )));
            downstream_trace[stage] = Some(substrate.add_cell(cell(
                physical(form.namespace, DOWNSTREAM_TRACE_BASE + stage as u64),
                internal_position(form, 60 + stage as i32),
                60 + stage as i16,
                1,
            )));
            return_relay[stage] = Some(substrate.add_cell(cell(
                physical(form.namespace, RETURN_RELAY_BASE + stage as u64),
                internal_position(form, 80 + stage as i32),
                70 + stage as i16,
                2,
            )));
        }
        let coincidence = coincidence.map(|value| value.expect("coincidence cell"));
        let inner = inner.map(|value| value.expect("inner cell"));
        let downstream = downstream.map(|value| value.expect("downstream cell"));
        let downstream_trace = downstream_trace.map(|value| value.expect("downstream trace"));
        let return_relay = return_relay.map(|value| value.expect("return relay"));

        let return_site = substrate.add_cell(cell(
            physical(form.namespace, RETURN_SITE_SUFFIX),
            coordinate(form, RETURN_SITE),
            80,
            1,
        ));
        let outward = substrate.add_cell(cell(
            physical(form.namespace, OUTWARD_SUFFIX),
            coordinate(form, OUTWARD_SITE),
            90,
            1,
        ));

        for side in boundary_order {
            substrate.add_arrow(drive(sources[side], traces[side], 1, 1, 100));
        }

        let paired = if form.altered_pairing { 3 } else { 1 };
        substrate.add_arrow(drive(traces[0], coincidence[0], 0, 1, 100));
        substrate.add_arrow(drive(traces[paired], coincidence[0], 0, 1, 100));
        substrate.add_arrow(drive(coincidence[0], inner[0], 0, 1, 100));

        substrate.add_arrow(drive(downstream_trace[0], coincidence[1], 0, 1, 100));
        substrate.add_arrow(drive(traces[2], coincidence[1], 0, 1, 100));
        substrate.add_arrow(drive(coincidence[1], inner[1], 0, 1, 100));

        let weak_links = [
            substrate.add_arrow(drive(inner[0], downstream[0], 1, 1, 1)),
            substrate.add_arrow(drive(inner[1], downstream[1], 1, 1, 1)),
        ];

        for stage in stage_order {
            substrate.add_arrow(drive(downstream[stage], downstream_trace[stage], 1, 1, 100));
            substrate.add_arrow(drive(
                downstream_trace[stage],
                return_relay[stage],
                0,
                1,
                100,
            ));
            substrate.add_arrow(drive(return_site, return_relay[stage], 0, 1, 100));
            substrate.add_arrow(modulatory(return_relay[stage], inner[stage], 1));
        }
        substrate.add_arrow(drive(downstream[1], outward, 1, 1, 100));

        Self {
            substrate,
            form,
            sites: [
                sources[0],
                sources[1],
                sources[2],
                sources[3],
                inner[0],
                downstream[0],
                inner[1],
                downstream[1],
                return_site,
                outward,
            ],
            weak_links,
        }
    }

    pub fn coordinate(&self, site: usize) -> i32 {
        assert!(site < SITE_COUNT, "site index out of bounds");
        coordinate(self.form, site)
    }

    pub fn physical(&self, site: usize) -> u64 {
        assert!(site < SITE_COUNT, "site index out of bounds");
        let suffix = match site {
            BOUNDARY_A..=BOUNDARY_D => SOURCE_BASE + site as u64,
            INNER_ZERO => INNER_BASE,
            DOWNSTREAM_ZERO => DOWNSTREAM_BASE,
            INNER_ONE => INNER_BASE + 1,
            DOWNSTREAM_ONE => DOWNSTREAM_BASE + 1,
            RETURN_SITE => RETURN_SITE_SUFFIX,
            OUTWARD_SITE => OUTWARD_SUFFIX,
            _ => unreachable!(),
        };
        physical(self.form.namespace, suffix)
    }

    pub fn participate<I>(&mut self, arrivals: I) -> Activity
    where
        I: IntoIterator<Item = Arrival>,
    {
        for arrival in arrivals {
            let site = (0..SITE_COUNT)
                .find(|site| coordinate(self.form, *site) == arrival.position)
                .expect("arrival position has no physical site");
            self.substrate.enter(SpikeInput {
                arrival_tick: arrival.tick,
                phase: arrival.phase,
                origin_physical: arrival.origin,
                target: self.sites[site],
                impulse: arrival.impulse,
            });
        }
        Activity {
            execution: self.substrate.propagate(),
        }
    }

    pub fn elapse(&mut self, tick: i64) -> WorkLedger {
        self.substrate.advance_time(tick)
    }

    pub fn link_couplings(&self) -> [i32; 2] {
        self.weak_links
            .map(|link| self.substrate.arrow_coupling(link))
    }

    pub fn link_resistances(&self) -> [u32; 2] {
        self.weak_links
            .map(|link| self.substrate.arrow_resistance(link))
    }

    pub fn links_live(&self) -> [bool; 2] {
        self.weak_links
            .map(|link| self.substrate.arrow_is_live(link))
    }

    pub fn complete_fingerprint(&self) -> u64 {
        self.substrate.complete_fingerprint()
    }

    pub fn permanent_fingerprint(&self) -> u64 {
        self.substrate.permanent_fingerprint()
    }

    pub fn persistent_bytes(&self) -> usize {
        self.substrate.persistent_bytes()
    }
}

fn coordinate(form: Form, site: usize) -> i32 {
    let base = 100_000 + site as i32 * 10_000;
    if form.reflected_positions {
        -base
    } else {
        base
    }
}

fn internal_position(form: Form, offset: i32) -> i32 {
    let base = 500_000 + offset * 10_000;
    if form.reflected_positions {
        -base
    } else {
        base
    }
}

fn physical(namespace: u64, suffix: u64) -> u64 {
    namespace + suffix
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 100,
    }
}

fn drive(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode: TransmissionMode::Drive,
    }
}

fn modulatory(from: CellId, to: CellId, delay: i64) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling: 1,
        resistance: 100,
        mode: TransmissionMode::Modulatory,
    }
}
