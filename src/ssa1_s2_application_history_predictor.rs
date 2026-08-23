//! SSA1-S2 development: evaluator-side prediction of path-dependent basins.

use std::collections::{BTreeMap, BTreeSet};

use crate::organism::{ArrowSpec, CellSpec, SpikeInput, Substrate};
use crate::ssa1_c2_lock_in_hysteresis_map::{Audit, RouteAudit};
use crate::ssa1_s_exposure_bias_map::{LandscapeClass, OpportunityRatio};

pub const PROTOCOL: &str = "ssa1-s2-application-history-predictor-v1";
pub const FROZEN_SSA1_S: &str = "e937329";

const FIRING_THRESHOLD: i32 = 4;
const INHIBITION: i32 = -64;
const EPISODES: usize = 18_000;
const MACRO_PERIOD: usize = 90;
const MAIN_MATURITY: usize = 8;
const STRIDES: [usize; 12] = [1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];
const OFFSETS: [usize; 4] = [0, 1, 17, 43];
const PROBE_STRIDES: [usize; 3] = [1, 7, 11];
const PROBE_OFFSETS: [usize; 2] = [0, 1];
const RATIOS: [OpportunityRatio; 4] = [
    OpportunityRatio {
        alternative: 1,
        incumbent: 2,
    },
    OpportunityRatio {
        alternative: 1,
        incumbent: 1,
    },
    OpportunityRatio {
        alternative: 2,
        incumbent: 1,
    },
    OpportunityRatio {
        alternative: 4,
        incumbent: 1,
    },
];

#[allow(dead_code)]
mod frozen_ssa1 {
    include!(concat!(env!("OUT_DIR"), "/ssa1_c2_instrumented_frozen.rs"));

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct Adapter {
        session: frozen_learning::Session,
    }

    impl Adapter {
        pub(super) fn blank(seed: u64, swap_patterns: bool) -> Self {
            Self {
                session: frozen_learning::session(seed, swap_patterns),
            }
        }

        pub(super) fn offer(&mut self) -> [usize; 2] {
            frozen_learning::offer(&mut self.session)
        }

        pub(super) fn return_consequence(&mut self, route: usize, variant: usize) -> usize {
            frozen_learning::return_consequence(&mut self.session, route, variant)
        }

        pub(super) fn inspect(&self) -> Landscape {
            frozen_learning::inspect(&self.session)
        }

        pub(super) fn audit(&self) -> super::Audit {
            let audit = frozen_learning::c2_audit(&self.session);
            super::Audit {
                routes: audit.routes.map(|route| super::RouteAudit {
                    evidence_shapes: route.evidence_shapes,
                    evidence_observations: route.evidence_observations,
                    evidence_support: route.evidence_support,
                    evidence_margin: route.evidence_margin,
                    evidence_eligible: route.evidence_eligible,
                    m5_support: route.m5_support,
                    m5_rejection: route.m5_rejection,
                    m5_score: route.m5_score,
                    m5_value_resistance: route.m5_value_resistance,
                    prototype_resistance: route.prototype_resistance,
                    live_proposals: route.live_proposals,
                    proposal_resistance: route.proposal_resistance,
                }),
                observations: audit.observations,
                abstentions: audit.abstentions,
                applications: audit.applications,
                exploration_admissions: audit.exploration_admissions,
                completed_events: audit.completed_events,
            }
        }
    }
}

pub use frozen_ssa1::Landscape;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScheduleDescriptor {
    pub ratio: OpportunityRatio,
    pub stride: usize,
    pub offset: usize,
    pub discovery: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhysicalWorld {
    opportunity_side: Option<usize>,
    postclosure: bool,
    favor_side: Option<usize>,
    route_at_side: [usize; 2],
    stale_route: [bool; 2],
    reverse_allocation: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Resolution {
    realized_route: Option<usize>,
    realized_side: Option<usize>,
    start_fingerprint: u64,
    trace_fingerprint: u64,
    end_fingerprint: u64,
    opportunity_visible: usize,
    quiescent: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ApplicationSummary {
    pub effective_applications: usize,
    pub neutral_applications: usize,
    pub attribution_failures: usize,
    pub first_direction: Option<i8>,
    pub first_direction_episode: Option<usize>,
    pub first_4_balance: Option<i32>,
    pub first_4_episode: Option<usize>,
    pub first_8_balance: Option<i32>,
    pub first_8_episode: Option<usize>,
    pub first_16_balance: Option<i32>,
    pub first_16_episode: Option<usize>,
    pub first_opposing_gap: Option<i32>,
    pub first_opposing_episode: Option<usize>,
    pub longest_90_direction: Option<i8>,
    pub ninetieth_application_episode: Option<usize>,
    pub gap_after_episode_90: i32,
    pub alternative_threshold_episode: Option<usize>,
    pub incumbent_deallocation_episode: Option<usize>,
    pub first_8_code: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trajectory {
    pub seed: u64,
    pub descriptor: ScheduleDescriptor,
    pub incumbent_side: usize,
    pub route_at_side: [usize; 2],
    pub initial_executions: usize,
    pub scheduled: [usize; 2],
    pub executions: [usize; 2],
    pub consequences: [usize; 2],
    pub summary: ApplicationSummary,
    pub final_audit: Audit,
    pub final_landscape: Landscape,
    pub final_class: LandscapeClass,
    pub schedule_exact: bool,
    pub duplicate_exact: bool,
    pub trace_attributed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PredictorId {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
}

impl PredictorId {
    pub fn name(self) -> &'static str {
        match self {
            Self::P0 => "P0-ratio-majority",
            Self::P1 => "P1-first-application",
            Self::P2 => "P2-first-4-balance",
            Self::P3 => "P3-first-8-balance",
            Self::P4 => "P4-first-16-balance",
            Self::P5 => "P5-gap-before-opposition",
            Self::P6 => "P6-longest-run-first-90",
            Self::P7 => "P7-gap-after-episode-90",
            Self::P8 => "P8-structural-commitment",
            Self::P9 => "P9-composite-tuple",
        }
    }
}

const PREDICTORS: [PredictorId; 10] = [
    PredictorId::P0,
    PredictorId::P1,
    PredictorId::P2,
    PredictorId::P3,
    PredictorId::P4,
    PredictorId::P5,
    PredictorId::P6,
    PredictorId::P7,
    PredictorId::P8,
    PredictorId::P9,
];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SplitMetrics {
    pub total: usize,
    pub predicted: usize,
    pub correct: usize,
    pub accuracy_basis_points: usize,
    pub coverage_basis_points: usize,
    pub per_basin_total: [usize; 4],
    pub per_basin_correct: [usize; 4],
    pub latest_available_episode: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PredictorMetrics {
    pub id: PredictorId,
    pub discovery: SplitMetrics,
    pub held_out: SplitMetrics,
    pub minimum_cell_accuracy_basis_points: usize,
    pub qualifies: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub seed: u64,
    pub incumbent_side: usize,
    pub route_at_side: [usize; 2],
    pub trajectories: Vec<Trajectory>,
    pub sentinel_h2: Vec<Trajectory>,
    pub sentinel_h32: Vec<Trajectory>,
    pub stale_blocked: bool,
    pub postclosure_inert: bool,
    pub observation_inert: bool,
    pub controls_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub protocol: &'static str,
    pub stage: &'static str,
    pub cells: Vec<Cell>,
    pub predictors: Vec<PredictorMetrics>,
    pub selected_predictor: Option<PredictorId>,
    pub classification: &'static str,
    pub basin_diversity: usize,
    pub trace_attribution_exact: bool,
    pub frozen_parent_exact: bool,
    pub claim_eligible: bool,
    pub passed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Probe,
    Micro,
    Gate,
}

impl Stage {
    fn name(self) -> &'static str {
        match self {
            Self::Probe => "PROBE",
            Self::Micro => "MICRO",
            Self::Gate => "GATE",
        }
    }

    fn strides(self) -> &'static [usize] {
        match self {
            Self::Probe => &PROBE_STRIDES,
            Self::Micro | Self::Gate => &STRIDES,
        }
    }

    fn offsets(self) -> &'static [usize] {
        match self {
            Self::Probe => &PROBE_OFFSETS,
            Self::Micro | Self::Gate => &OFFSETS,
        }
    }
}

fn cell(physical_id: u64, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position: physical_id as i32,
        region: 0,
        threshold,
        state: 0,
        generation: 1,
        resistance: 16,
    }
}

fn resolve(live: [usize; 2], world: &PhysicalWorld, identity: u64) -> Resolution {
    let base = identity.wrapping_mul(10_000).wrapping_add(5_000_000);
    let mut physical = vec![
        (base + 10, 1),
        (base + 20, FIRING_THRESHOLD),
        (base + 30, FIRING_THRESHOLD),
        (base + 40, 1),
        (base + 50, 1),
    ];
    for (route, count) in live.iter().copied().enumerate() {
        for supporter in 0..count {
            physical.push((base + 100 + route as u64 * 100 + supporter as u64, 1));
        }
    }
    if world.reverse_allocation {
        physical.reverse();
    }
    let mut substrate = Substrate::new();
    let mut ids = BTreeMap::new();
    for (physical_id, threshold) in physical {
        ids.insert(
            physical_id,
            substrate.add_cell(cell(physical_id, threshold)),
        );
    }
    let source = ids[&(base + 10)];
    let contenders = [ids[&(base + 20)], ids[&(base + 30)]];
    let effects = [ids[&(base + 40)], ids[&(base + 50)]];
    for route in 0..2 {
        if !world.stale_route[route] {
            for supporter in 0..live[route] {
                let relay = ids[&(base + 100 + route as u64 * 100 + supporter as u64)];
                let side = world
                    .route_at_side
                    .iter()
                    .position(|candidate| *candidate == route)
                    .expect("route has physical side");
                let delay = if supporter == 3 && world.favor_side == Some(side) {
                    6
                } else {
                    [1, 3, 5, 7][supporter]
                };
                substrate.add_arrow(ArrowSpec {
                    from: source,
                    to: relay,
                    delay,
                    transient_delay: 0,
                    phase: 0,
                    coupling: 1,
                    generation: 1,
                    resistance: 16,
                });
                substrate.add_arrow(ArrowSpec {
                    from: relay,
                    to: contenders[route],
                    delay: 0,
                    transient_delay: 0,
                    phase: 0,
                    coupling: 1,
                    generation: 1,
                    resistance: 16,
                });
            }
            substrate.add_arrow(ArrowSpec {
                from: contenders[route],
                to: effects[route],
                delay: 0,
                transient_delay: 0,
                phase: 0,
                coupling: 1,
                generation: 1,
                resistance: 16,
            });
        }
        substrate.add_arrow(ArrowSpec {
            from: contenders[route],
            to: contenders[1 - route],
            delay: 0,
            transient_delay: 0,
            phase: -100,
            coupling: INHIBITION,
            generation: 1,
            resistance: 16,
        });
    }
    substrate.enter(SpikeInput {
        arrival_tick: 0,
        phase: 0,
        origin_physical: base + 1,
        target: source,
        impulse: 1,
    });
    if let Some(side) = world.opportunity_side {
        let route = world.route_at_side[side];
        for pulse in 0..3 {
            substrate.enter(SpikeInput {
                arrival_tick: if world.postclosure {
                    [20, 22, 24][pulse]
                } else {
                    [0, 2, 4][pulse]
                },
                phase: pulse as i32 - 1,
                origin_physical: base + 10_000 + pulse as u64,
                target: contenders[route],
                impulse: 1,
            });
        }
    }
    let execution = substrate.propagate();
    let realized_route = [base + 40, base + 50]
        .iter()
        .position(|effect| execution.fired.contains(effect));
    let realized_side = realized_route.and_then(|route| {
        world
            .route_at_side
            .iter()
            .position(|candidate| *candidate == route)
    });
    Resolution {
        realized_route,
        realized_side,
        start_fingerprint: execution.start_fingerprint,
        trace_fingerprint: execution.trace_fingerprint,
        end_fingerprint: execution.end_fingerprint,
        opportunity_visible: usize::from(world.opportunity_side.is_some()) * 3,
        quiescent: execution.naturally_quiescent,
    }
}

fn resolve_exact(
    live: [usize; 2],
    world: &PhysicalWorld,
    identity: u64,
    duplicate_exact: &mut bool,
) -> Resolution {
    let first = resolve(live, world, identity);
    let second = resolve(live, world, identity);
    *duplicate_exact &= first == second && first.quiescent;
    first
}

fn schedule_is_alternative(clock: usize, descriptor: ScheduleDescriptor) -> bool {
    let b_count = MACRO_PERIOD * descriptor.ratio.alternative
        / (descriptor.ratio.alternative + descriptor.ratio.incumbent);
    (descriptor.stride * (clock % MACRO_PERIOD) + descriptor.offset) % MACRO_PERIOD < b_count
}

fn variable_variant(clock: usize) -> usize {
    (clock.wrapping_mul(3) + clock / 7 + clock / 31) % 4
}

fn class(landscape: &Landscape, role_routes: [usize; 2]) -> LandscapeClass {
    let live = role_routes.map(|route| landscape.live_supporters[route]);
    match (
        live[0] >= FIRING_THRESHOLD as usize,
        live[1] >= FIRING_THRESHOLD as usize,
    ) {
        (true, false) => LandscapeClass::IncumbentLock,
        (true, true) => LandscapeClass::Mixed,
        (false, true) => LandscapeClass::Alternative,
        (false, false) => LandscapeClass::Subthreshold,
    }
}

fn gap(audit: &Audit, role_routes: [usize; 2]) -> i32 {
    audit.routes[role_routes[1]].m5_score - audit.routes[role_routes[0]].m5_score
}

fn mature_session(
    seed: u64,
    maturity: usize,
    incumbent_side: usize,
    route_at_side: [usize; 2],
    reverse: bool,
    duplicate_exact: &mut bool,
) -> (frozen_ssa1::Adapter, usize) {
    let mut session = frozen_ssa1::Adapter::blank(seed, route_at_side[0] == 1);
    let incumbent_route = route_at_side[incumbent_side];
    let mut executions = 0;
    for episode in 0..maturity {
        let live = session.offer();
        let physical = resolve_exact(
            live,
            &PhysicalWorld {
                opportunity_side: Some(incumbent_side),
                postclosure: false,
                favor_side: None,
                route_at_side,
                stale_route: [false; 2],
                reverse_allocation: reverse ^ episode.is_multiple_of(2),
            },
            seed + episode as u64,
            duplicate_exact,
        );
        if physical.realized_route == Some(incumbent_route) {
            executions += 1;
            let _ = session.return_consequence(incumbent_route, 0);
        }
    }
    (session, executions)
}

#[derive(Default)]
struct TraceBuilder {
    summary: ApplicationSummary,
    first_16: Vec<i8>,
    first_90: Vec<i8>,
    first_nonzero: Option<i8>,
}

impl TraceBuilder {
    fn observe_application(
        &mut self,
        episode: usize,
        before: &Audit,
        after: &Audit,
        role_routes: [usize; 2],
    ) {
        let application_delta = after.applications.saturating_sub(before.applications);
        if application_delta == 0 {
            return;
        }
        if application_delta != 1 {
            self.summary.attribution_failures += 1;
            return;
        }
        let delta = gap(after, role_routes) - gap(before, role_routes);
        let direction = delta.signum() as i8;
        if direction == 0 {
            self.summary.neutral_applications += 1;
            return;
        }
        self.summary.effective_applications += 1;
        if self.summary.first_direction.is_none() {
            self.summary.first_direction = Some(direction);
            self.summary.first_direction_episode = Some(episode);
            self.first_nonzero = Some(direction);
        } else if self.summary.first_opposing_episode.is_none()
            && self.first_nonzero.is_some_and(|first| first != direction)
        {
            self.summary.first_opposing_gap = Some(gap(before, role_routes));
            self.summary.first_opposing_episode = Some(episode);
        }
        if self.first_16.len() < 16 {
            self.first_16.push(direction);
            let balance: i32 = self.first_16.iter().map(|value| i32::from(*value)).sum();
            match self.first_16.len() {
                4 => {
                    self.summary.first_4_balance = Some(balance);
                    self.summary.first_4_episode = Some(episode);
                }
                8 => {
                    self.summary.first_8_balance = Some(balance);
                    self.summary.first_8_episode = Some(episode);
                    self.summary.first_8_code = Some(direction_code(&self.first_16));
                }
                16 => {
                    self.summary.first_16_balance = Some(balance);
                    self.summary.first_16_episode = Some(episode);
                }
                _ => {}
            }
        }
        if self.first_90.len() < 90 {
            self.first_90.push(direction);
            if self.first_90.len() == 90 {
                self.summary.ninetieth_application_episode = Some(episode);
                self.summary.longest_90_direction = longest_run_direction(&self.first_90);
            }
        }
    }
}

fn direction_code(directions: &[i8]) -> u32 {
    directions.iter().take(8).fold(0, |code, direction| {
        code * 3
            + match direction {
                -1 => 0,
                0 => 1,
                1 => 2,
                _ => unreachable!("signed direction"),
            }
    })
}

fn longest_run_direction(directions: &[i8]) -> Option<i8> {
    let mut longest = [0usize; 2];
    let mut current = 0;
    let mut previous = 0;
    for direction in directions.iter().copied() {
        if direction == previous {
            current += 1;
        } else {
            previous = direction;
            current = 1;
        }
        let index = usize::from(direction > 0);
        longest[index] = longest[index].max(current);
    }
    match longest[0].cmp(&longest[1]) {
        std::cmp::Ordering::Greater => Some(-1),
        std::cmp::Ordering::Less => Some(1),
        std::cmp::Ordering::Equal => Some(0),
    }
}

fn trajectory(
    mut session: frozen_ssa1::Adapter,
    initial_executions: usize,
    descriptor: ScheduleDescriptor,
    incumbent_side: usize,
    route_at_side: [usize; 2],
    reverse: bool,
    identity: u64,
    collect_trace: bool,
) -> Trajectory {
    let mut duplicate_exact = true;
    let role_routes = [
        route_at_side[incumbent_side],
        route_at_side[1 - incumbent_side],
    ];
    let mut scheduled = [0usize; 2];
    let mut executions = [0usize; 2];
    let mut consequences = [0usize; 2];
    let mut trace = TraceBuilder::default();
    let initial_landscape = session.inspect();
    let mut previous_live = role_routes.map(|route| initial_landscape.live_supporters[route]);
    for episode in 0..EPISODES {
        // The physical schedule is fixed before the organism is queried.
        let alternative = schedule_is_alternative(episode, descriptor);
        let role = usize::from(alternative);
        let side = if alternative {
            1 - incumbent_side
        } else {
            incumbent_side
        };
        scheduled[role] += 1;
        let before_audit = collect_trace.then(|| session.audit());
        let live = session.offer();
        let physical = resolve_exact(
            live,
            &PhysicalWorld {
                opportunity_side: Some(side),
                postclosure: false,
                favor_side: None,
                route_at_side,
                stale_route: [false; 2],
                reverse_allocation: reverse ^ episode.is_multiple_of(2),
            },
            identity + episode as u64,
            &mut duplicate_exact,
        );
        if let (Some(route), Some(realized_side)) =
            (physical.realized_route, physical.realized_side)
        {
            let realized_role = usize::from(realized_side != incumbent_side);
            executions[realized_role] += 1;
            let variant = if realized_role == 1 {
                0
            } else {
                variable_variant(episode + 17)
            };
            let before_observations = session.audit().routes[route].evidence_observations;
            let _ = session.return_consequence(route, variant);
            let after_observations = session.audit().routes[route].evidence_observations;
            consequences[realized_role] += usize::from(after_observations > before_observations);
        }
        if let Some(before) = before_audit {
            let after = session.audit();
            trace.observe_application(episode + 1, &before, &after, role_routes);
            if episode == 89 {
                trace.summary.gap_after_episode_90 = gap(&after, role_routes);
            }
        }
        let current_landscape = session.inspect();
        let current_live = role_routes.map(|route| current_landscape.live_supporters[route]);
        if trace.summary.alternative_threshold_episode.is_none()
            && previous_live[1] < FIRING_THRESHOLD as usize
            && current_live[1] >= FIRING_THRESHOLD as usize
        {
            trace.summary.alternative_threshold_episode = Some(episode + 1);
        }
        if trace.summary.incumbent_deallocation_episode.is_none()
            && previous_live[0] >= FIRING_THRESHOLD as usize
            && current_live[0] < FIRING_THRESHOLD as usize
        {
            trace.summary.incumbent_deallocation_episode = Some(episode + 1);
        }
        previous_live = current_live;
    }
    let expected_alternative = EPISODES * descriptor.ratio.alternative
        / (descriptor.ratio.alternative + descriptor.ratio.incumbent);
    let final_audit = session.audit();
    let final_landscape = session.inspect();
    let trace_attributed = collect_trace && trace.summary.attribution_failures == 0;
    Trajectory {
        seed: identity,
        descriptor,
        incumbent_side,
        route_at_side,
        initial_executions,
        scheduled,
        executions,
        consequences,
        final_class: class(&final_landscape, role_routes),
        summary: trace.summary,
        final_audit,
        final_landscape,
        schedule_exact: scheduled == [EPISODES - expected_alternative, expected_alternative],
        duplicate_exact,
        trace_attributed,
    }
}

fn descriptors(stage: Stage) -> Vec<ScheduleDescriptor> {
    let full_stride_positions: BTreeMap<_, _> = STRIDES
        .iter()
        .copied()
        .enumerate()
        .map(|(index, stride)| (stride, index))
        .collect();
    let full_offset_positions: BTreeMap<_, _> = OFFSETS
        .iter()
        .copied()
        .enumerate()
        .map(|(index, offset)| (offset, index))
        .collect();
    RATIOS
        .iter()
        .copied()
        .flat_map(|ratio| {
            let stride_positions = full_stride_positions.clone();
            let offset_positions = full_offset_positions.clone();
            stage.strides().iter().copied().flat_map(move |stride| {
                let stride_positions = stride_positions.clone();
                let offset_positions = offset_positions.clone();
                stage.offsets().iter().copied().map(move |offset| {
                    let stride_index = stride_positions[&stride];
                    let offset_index = offset_positions[&offset];
                    ScheduleDescriptor {
                        ratio,
                        stride,
                        offset,
                        discovery: (stride_index + offset_index).is_multiple_of(2),
                    }
                })
            })
        })
        .collect()
}

fn sentinel_descriptors() -> Vec<ScheduleDescriptor> {
    RATIOS
        .iter()
        .copied()
        .flat_map(|ratio| {
            [1usize, 7].into_iter().flat_map(move |stride| {
                [0usize, 1]
                    .into_iter()
                    .map(move |offset| ScheduleDescriptor {
                        ratio,
                        stride,
                        offset,
                        discovery: true,
                    })
            })
        })
        .collect()
}

fn basin_index(class: LandscapeClass) -> usize {
    match class {
        LandscapeClass::IncumbentLock => 0,
        LandscapeClass::Mixed => 1,
        LandscapeClass::Alternative => 2,
        LandscapeClass::Subthreshold => 3,
    }
}

fn signed_prediction(value: Option<i32>) -> Option<LandscapeClass> {
    value.map(|value| match value.signum() {
        -1 => LandscapeClass::IncumbentLock,
        0 => LandscapeClass::Mixed,
        1 => LandscapeClass::Alternative,
        _ => unreachable!("signum"),
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CompositeKey {
    first_8_sign: i8,
    opposing_gap_sign: i8,
    structural: u8,
}

fn structural_prediction(summary: &ApplicationSummary) -> LandscapeClass {
    match (
        summary.alternative_threshold_episode,
        summary.incumbent_deallocation_episode,
    ) {
        (Some(_), Some(_)) => LandscapeClass::Alternative,
        (Some(_), None) => LandscapeClass::Mixed,
        _ => LandscapeClass::IncumbentLock,
    }
}

fn composite_key(summary: &ApplicationSummary) -> Option<CompositeKey> {
    Some(CompositeKey {
        first_8_sign: summary.first_8_balance?.signum() as i8,
        opposing_gap_sign: summary.first_opposing_gap?.signum() as i8,
        structural: match structural_prediction(summary) {
            LandscapeClass::IncumbentLock => 0,
            LandscapeClass::Mixed => 1,
            LandscapeClass::Alternative => 2,
            LandscapeClass::Subthreshold => 3,
        },
    })
}

fn majority<T: Ord + Copy>(
    pairs: impl Iterator<Item = (T, LandscapeClass)>,
) -> BTreeMap<T, LandscapeClass> {
    let mut counts: BTreeMap<T, [usize; 4]> = BTreeMap::new();
    for (key, outcome) in pairs {
        counts.entry(key).or_default()[basin_index(outcome)] += 1;
    }
    counts
        .into_iter()
        .filter_map(|(key, counts)| {
            let maximum = *counts.iter().max().unwrap();
            let winners: Vec<_> = counts
                .iter()
                .enumerate()
                .filter(|(_, count)| **count == maximum)
                .map(|(index, _)| index)
                .collect();
            (winners.len() == 1).then(|| {
                let class = match winners[0] {
                    0 => LandscapeClass::IncumbentLock,
                    1 => LandscapeClass::Mixed,
                    2 => LandscapeClass::Alternative,
                    3 => LandscapeClass::Subthreshold,
                    _ => unreachable!(),
                };
                (key, class)
            })
        })
        .collect()
}

fn available_episode(id: PredictorId, summary: &ApplicationSummary) -> Option<usize> {
    match id {
        PredictorId::P0 => Some(0),
        PredictorId::P1 => summary.first_direction_episode,
        PredictorId::P2 => summary.first_4_episode,
        PredictorId::P3 => summary.first_8_episode,
        PredictorId::P4 => summary.first_16_episode,
        PredictorId::P5 => summary.first_opposing_episode,
        PredictorId::P6 => summary.ninetieth_application_episode,
        PredictorId::P7 => Some(90),
        PredictorId::P8 => Some(
            summary
                .incumbent_deallocation_episode
                .or(summary.alternative_threshold_episode)
                .unwrap_or(EPISODES),
        ),
        PredictorId::P9 => Some(
            summary
                .first_8_episode?
                .max(summary.first_opposing_episode?)
                .max(
                    summary
                        .incumbent_deallocation_episode
                        .or(summary.alternative_threshold_episode)
                        .unwrap_or(EPISODES),
                ),
        ),
    }
}

fn predict(
    id: PredictorId,
    trajectory: &Trajectory,
    ratio_map: &BTreeMap<OpportunityRatio, LandscapeClass>,
    composite_map: &BTreeMap<CompositeKey, LandscapeClass>,
) -> Option<LandscapeClass> {
    let summary = &trajectory.summary;
    match id {
        PredictorId::P0 => ratio_map.get(&trajectory.descriptor.ratio).copied(),
        PredictorId::P1 => signed_prediction(summary.first_direction.map(i32::from)),
        PredictorId::P2 => signed_prediction(summary.first_4_balance),
        PredictorId::P3 => signed_prediction(summary.first_8_balance),
        PredictorId::P4 => signed_prediction(summary.first_16_balance),
        PredictorId::P5 => signed_prediction(summary.first_opposing_gap),
        PredictorId::P6 => signed_prediction(summary.longest_90_direction.map(i32::from)),
        PredictorId::P7 => signed_prediction(Some(summary.gap_after_episode_90)),
        PredictorId::P8 => Some(structural_prediction(summary)),
        PredictorId::P9 => composite_key(summary).and_then(|key| composite_map.get(&key).copied()),
    }
}

fn split_metrics(
    id: PredictorId,
    trajectories: &[&Trajectory],
    ratio_map: &BTreeMap<OpportunityRatio, LandscapeClass>,
    composite_map: &BTreeMap<CompositeKey, LandscapeClass>,
) -> SplitMetrics {
    let mut metrics = SplitMetrics::default();
    for trajectory in trajectories {
        metrics.total += 1;
        let basin = basin_index(trajectory.final_class);
        metrics.per_basin_total[basin] += 1;
        if let Some(prediction) = predict(id, trajectory, ratio_map, composite_map) {
            metrics.predicted += 1;
            if prediction == trajectory.final_class {
                metrics.correct += 1;
                metrics.per_basin_correct[basin] += 1;
            }
            metrics.latest_available_episode = metrics
                .latest_available_episode
                .max(available_episode(id, &trajectory.summary).unwrap_or(EPISODES));
        }
    }
    if metrics.total > 0 {
        metrics.accuracy_basis_points = metrics.correct * 10_000 / metrics.total;
        metrics.coverage_basis_points = metrics.predicted * 10_000 / metrics.total;
    }
    metrics
}

fn predictor_metrics(cells: &[Cell]) -> Vec<PredictorMetrics> {
    let all: Vec<_> = cells
        .iter()
        .flat_map(|cell| cell.trajectories.iter())
        .collect();
    let discovery: Vec<_> = all
        .iter()
        .copied()
        .filter(|trajectory| trajectory.descriptor.discovery)
        .collect();
    let held_out: Vec<_> = all
        .iter()
        .copied()
        .filter(|trajectory| !trajectory.descriptor.discovery)
        .collect();
    let ratio_map = majority(
        discovery
            .iter()
            .map(|trajectory| (trajectory.descriptor.ratio, trajectory.final_class)),
    );
    let composite_map = majority(discovery.iter().filter_map(|trajectory| {
        composite_key(&trajectory.summary).map(|key| (key, trajectory.final_class))
    }));
    PREDICTORS
        .iter()
        .copied()
        .map(|id| {
            let discovery_metrics = split_metrics(id, &discovery, &ratio_map, &composite_map);
            let held_out_metrics = split_metrics(id, &held_out, &ratio_map, &composite_map);
            let minimum_cell_accuracy_basis_points = cells
                .iter()
                .map(|cell| {
                    let local: Vec<_> = cell
                        .trajectories
                        .iter()
                        .filter(|trajectory| !trajectory.descriptor.discovery)
                        .collect();
                    split_metrics(id, &local, &ratio_map, &composite_map).accuracy_basis_points
                })
                .min()
                .unwrap_or(0);
            let qualifies = id != PredictorId::P0
                && discovery_metrics.accuracy_basis_points >= 9_500
                && held_out_metrics.accuracy_basis_points >= 9_500
                && held_out_metrics.coverage_basis_points >= 9_000
                && minimum_cell_accuracy_basis_points >= 9_000;
            PredictorMetrics {
                id,
                discovery: discovery_metrics,
                held_out: held_out_metrics,
                minimum_cell_accuracy_basis_points,
                qualifies,
            }
        })
        .collect()
}

fn physical_controls(
    seed: u64,
    incumbent_side: usize,
    route_at_side: [usize; 2],
    reverse: bool,
) -> (bool, bool, bool) {
    let alternative_side = 1 - incumbent_side;
    let alternative_route = route_at_side[alternative_side];
    let live = if route_at_side[incumbent_side] == 0 {
        [4, 1]
    } else {
        [1, 4]
    };
    let early = PhysicalWorld {
        opportunity_side: Some(alternative_side),
        postclosure: false,
        favor_side: None,
        route_at_side,
        stale_route: [false; 2],
        reverse_allocation: reverse,
    };
    let stale = PhysicalWorld {
        stale_route: [alternative_route == 0, alternative_route == 1],
        ..early.clone()
    };
    let baseline = PhysicalWorld {
        opportunity_side: None,
        ..early.clone()
    };
    let late = PhysicalWorld {
        postclosure: true,
        ..early.clone()
    };
    let early_result = resolve(live, &early, seed);
    let stale_result = resolve(live, &stale, seed + 1);
    let baseline_result = resolve(live, &baseline, seed + 2);
    let late_result = resolve(live, &late, seed + 2);
    (
        early_result.realized_route == Some(alternative_route)
            && stale_result.realized_route != Some(alternative_route),
        late_result.realized_route == baseline_result.realized_route
            && late_result.opportunity_visible == 3,
        early_result == resolve(live, &early, seed),
    )
}

fn source_audit() -> bool {
    let source = include_str!("ssa1_s2_application_history_predictor.rs");
    let prefix = source.split("fn source_audit()").next().unwrap_or_default();
    let mechanism = prefix.split("fn trajectory(").nth(1).unwrap_or_default();
    mechanism.contains("let alternative = schedule_is_alternative(episode, descriptor);")
        && mechanism.find("let alternative = schedule_is_alternative(episode, descriptor);")
            < mechanism.find("let live = session.offer();")
        && !mechanism.contains("random(")
        && !mechanism.contains("softmax")
        && !mechanism.contains("reward")
}

fn frozen_source_invariant() -> bool {
    let source = include_str!("ds8_cumulative_semantic_credit_probe.rs");
    let path = source
        .split("// DS8_ORGANISM_PATH_BEGIN")
        .nth(1)
        .and_then(|tail| tail.split("// DS8_ORGANISM_PATH_END").next())
        .unwrap_or_default();
    path.contains("support >= RECURRENT_SUPPORT && margin >= MINIMUM_MARGIN")
        && path.contains("[true, false] => Some(encounters[0])")
        && path.contains("[false, true] => Some(encounters[1])")
        && path.contains("self.work.abstentions += 1")
        && !path.contains("pressure")
        && !path.contains("decay")
        && !path.contains("forget")
}

fn run_cell(stage: Stage, seed: u64, index: usize) -> Cell {
    let incumbent_side = index % 2;
    let route_at_side = if index.is_multiple_of(2) {
        [0, 1]
    } else {
        [1, 0]
    };
    let reverse = seed.is_multiple_of(2);
    let mut boundary_exact = true;
    let (boundary, initial_executions) = mature_session(
        seed,
        MAIN_MATURITY,
        incumbent_side,
        route_at_side,
        reverse,
        &mut boundary_exact,
    );
    let trajectories: Vec<_> = descriptors(stage)
        .into_iter()
        .enumerate()
        .map(|(descriptor_index, descriptor)| {
            trajectory(
                boundary.clone(),
                initial_executions,
                descriptor,
                incumbent_side,
                route_at_side,
                reverse,
                seed + 100_000_000 + descriptor_index as u64 * 100_000,
                true,
            )
        })
        .collect();
    let sentinels = sentinel_descriptors();
    let sentinel_h2 = if matches!(stage, Stage::Probe) {
        Vec::new()
    } else {
        let (boundary, initial) = mature_session(
            seed + 300_000_000,
            2,
            incumbent_side,
            route_at_side,
            reverse,
            &mut boundary_exact,
        );
        sentinels
            .iter()
            .copied()
            .enumerate()
            .map(|(descriptor_index, descriptor)| {
                trajectory(
                    boundary.clone(),
                    initial,
                    descriptor,
                    incumbent_side,
                    route_at_side,
                    reverse,
                    seed + 310_000_000 + descriptor_index as u64 * 100_000,
                    true,
                )
            })
            .collect()
    };
    let sentinel_h32 = if matches!(stage, Stage::Probe) {
        Vec::new()
    } else {
        let (boundary, initial) = mature_session(
            seed + 400_000_000,
            32,
            incumbent_side,
            route_at_side,
            reverse,
            &mut boundary_exact,
        );
        sentinels
            .iter()
            .copied()
            .enumerate()
            .map(|(descriptor_index, descriptor)| {
                trajectory(
                    boundary.clone(),
                    initial,
                    descriptor,
                    incumbent_side,
                    route_at_side,
                    reverse,
                    seed + 410_000_000 + descriptor_index as u64 * 100_000,
                    true,
                )
            })
            .collect()
    };
    let observation_descriptor = ScheduleDescriptor {
        ratio: RATIOS[1],
        stride: 7,
        offset: 1,
        discovery: true,
    };
    let observed = trajectory(
        boundary.clone(),
        initial_executions,
        observation_descriptor,
        incumbent_side,
        route_at_side,
        reverse,
        seed + 500_000_000,
        true,
    );
    let unobserved = trajectory(
        boundary,
        initial_executions,
        observation_descriptor,
        incumbent_side,
        route_at_side,
        reverse,
        seed + 500_000_000,
        false,
    );
    let observation_inert = observed.final_audit == unobserved.final_audit
        && observed.final_landscape == unobserved.final_landscape
        && observed.final_class == unobserved.final_class;
    let (stale_blocked, postclosure_inert, physical_duplicate) =
        physical_controls(seed + 600_000_000, incumbent_side, route_at_side, reverse);
    let h2_relation = sentinel_h2
        .iter()
        .all(|trajectory| trajectory.final_class != LandscapeClass::IncumbentLock);
    let h32_relation = sentinel_h32
        .iter()
        .all(|trajectory| trajectory.final_class == LandscapeClass::IncumbentLock);
    let sentinel_controls = sentinel_h2.iter().chain(&sentinel_h32).all(|trajectory| {
        trajectory.schedule_exact
            && trajectory.duplicate_exact
            && trajectory.trace_attributed
            && trajectory.scheduled == trajectory.executions
            && trajectory.executions == trajectory.consequences
    });
    let count_capacity_safe = trajectories
        .iter()
        .chain(&sentinel_h2)
        .chain(&sentinel_h32)
        .all(|trajectory| {
            trajectory
                .final_audit
                .routes
                .iter()
                .all(|route| route.evidence_observations <= u64::from(u16::MAX))
        });
    let controls_passed = boundary_exact
        && trajectories.iter().all(|trajectory| {
            trajectory.initial_executions == MAIN_MATURITY
                && trajectory.schedule_exact
                && trajectory.duplicate_exact
                && trajectory.trace_attributed
                && trajectory.scheduled == trajectory.executions
                && trajectory.executions == trajectory.consequences
        })
        && (matches!(stage, Stage::Probe) || (h2_relation && h32_relation && sentinel_controls))
        && observation_inert
        && stale_blocked
        && postclosure_inert
        && physical_duplicate
        && count_capacity_safe
        && source_audit();
    Cell {
        seed,
        incumbent_side,
        route_at_side,
        trajectories,
        sentinel_h2,
        sentinel_h32,
        stale_blocked,
        postclosure_inert,
        observation_inert,
        controls_passed,
    }
}

fn report(stage: Stage, seeds: &[u64]) -> Report {
    let cells: Vec<_> = seeds
        .iter()
        .enumerate()
        .map(|(index, seed)| run_cell(stage, *seed, index))
        .collect();
    let predictors = predictor_metrics(&cells);
    let selected_predictor = predictors
        .iter()
        .find(|metrics| metrics.qualifies)
        .map(|metrics| metrics.id);
    let classes: BTreeSet<_> = cells
        .iter()
        .flat_map(|cell| {
            cell.trajectories
                .iter()
                .map(|trajectory| trajectory.final_class)
        })
        .collect();
    let trace_attribution_exact = cells.iter().all(|cell| {
        cell.trajectories
            .iter()
            .all(|trajectory| trajectory.trace_attributed)
    });
    let controls_passed = cells.iter().all(|cell| cell.controls_passed);
    let classification = if !controls_passed || !trace_attribution_exact {
        "E — scientific ambiguity"
    } else {
        match selected_predictor {
            Some(PredictorId::P1)
            | Some(PredictorId::P2)
            | Some(PredictorId::P3)
            | Some(PredictorId::P4)
            | Some(PredictorId::P5)
            | Some(PredictorId::P6) => "A — early low-dimensional application law",
            Some(PredictorId::P7) | Some(PredictorId::P8) => "B — commitment-state law",
            Some(PredictorId::P9) => "C — composite history law",
            _ => "D — sequence-complex within tested library",
        }
    };
    let frozen_parent_exact = frozen_source_invariant();
    Report {
        protocol: PROTOCOL,
        stage: stage.name(),
        cells,
        predictors,
        selected_predictor,
        classification,
        basin_diversity: classes.len(),
        trace_attribution_exact,
        frozen_parent_exact,
        claim_eligible: false,
        passed: controls_passed && trace_attribution_exact && frozen_parent_exact,
    }
}

pub fn run_probe() -> Report {
    report(Stage::Probe, &[2_000_000_000, 2_000_000_001])
}

pub fn run_micro() -> Report {
    report(
        Stage::Micro,
        &[2_010_000_000, 2_020_000_001, 2_010_000_002, 2_020_000_003],
    )
}

pub fn run_gate() -> Report {
    report(
        Stage::Gate,
        &[
            2_030_000_000,
            2_040_000_001,
            2_050_000_002,
            2_060_000_003,
            2_070_000_004,
            2_080_000_005,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "explicit SSA1-S2 PROBE"]
    fn probe_attributes_application_history() {
        assert!(run_probe().passed);
    }

    #[test]
    #[ignore = "explicit SSA1-S2 MICRO"]
    fn micro_selects_or_rejects_predictor_library() {
        assert!(run_micro().passed);
    }

    #[test]
    #[ignore = "explicit SSA1-S2 GATE"]
    fn gate_transfers_application_history_law() {
        assert!(run_gate().passed);
    }
}
