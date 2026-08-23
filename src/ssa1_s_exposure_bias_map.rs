//! SSA1-S development: selection-induced exposure bias characterization.

use std::collections::BTreeMap;

use crate::organism::{ArrowSpec, CellSpec, SpikeInput, Substrate};
use crate::ssa1_c2_lock_in_hysteresis_map::{Audit, RouteAudit};

pub const PROTOCOL: &str = "ssa1-s-exposure-bias-map-v1";
pub const FROZEN_ORGANISM: &str = "2125a197ad0e5796a12668cd76c2071236763af0";
pub const FROZEN_SSA1_R: &str = "afb8ea8";

const FIRING_THRESHOLD: i32 = 4;
const INHIBITION: i32 = -64;
const EPISODES: usize = 18_000;
const CHECKPOINTS: [usize; 6] = [0, 90, 900, 4_500, 9_000, 18_000];
const FULL_MATURITIES: [usize; 5] = [0, 2, 8, 32, 128];
const PROBE_MATURITIES: [usize; 3] = [0, 32, 128];

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
pub struct OpportunityRatio {
    pub alternative: usize,
    pub incumbent: usize,
}

impl OpportunityRatio {
    pub fn name(self) -> &'static str {
        match (self.alternative, self.incumbent) {
            (1, 8) => "1:8",
            (1, 4) => "1:4",
            (1, 2) => "1:2",
            (1, 1) => "1:1",
            (2, 1) => "2:1",
            (4, 1) => "4:1",
            (8, 1) => "8:1",
            _ => "unregistered",
        }
    }

    fn period(self) -> usize {
        self.alternative + self.incumbent
    }
}

const FULL_RATIOS: [OpportunityRatio; 7] = [
    OpportunityRatio {
        alternative: 1,
        incumbent: 8,
    },
    OpportunityRatio {
        alternative: 1,
        incumbent: 4,
    },
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
    OpportunityRatio {
        alternative: 8,
        incumbent: 1,
    },
];

const PROBE_RATIOS: [OpportunityRatio; 3] = [FULL_RATIOS[1], FULL_RATIOS[3], FULL_RATIOS[5]];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LandscapeClass {
    IncumbentLock,
    Mixed,
    Alternative,
    Subthreshold,
}

impl LandscapeClass {
    pub fn name(self) -> &'static str {
        match self {
            Self::IncumbentLock => "INCUMBENT_LOCK",
            Self::Mixed => "MIXED",
            Self::Alternative => "ALTERNATIVE",
            Self::Subthreshold => "SUBTHRESHOLD",
        }
    }

    fn adaptation_rank(self) -> usize {
        match self {
            Self::IncumbentLock => 0,
            Self::Subthreshold => 1,
            Self::Mixed => 2,
            Self::Alternative => 3,
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checkpoint {
    pub budget: usize,
    pub scheduled: [usize; 2],
    pub executions: [usize; 2],
    pub consequences: [usize; 2],
    pub unresolved: usize,
    pub audit: Audit,
    pub landscape: Landscape,
    pub independent_realizations: [usize; 2],
    pub class: LandscapeClass,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trajectory {
    pub maturity: usize,
    pub ratio: OpportunityRatio,
    pub phase_offset: usize,
    pub incumbent_side: usize,
    pub route_at_side: [usize; 2],
    pub initial_executions: usize,
    pub boundary_audit: Audit,
    pub boundary_landscape: Landscape,
    pub checkpoints: Vec<Checkpoint>,
    pub schedule_exact: bool,
    pub exposure_transferred: bool,
    pub duplicate_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EqualControl {
    pub maturity: usize,
    pub final_checkpoint: Checkpoint,
    pub schedule_exact: bool,
    pub exposure_transferred: bool,
    pub duplicate_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub seed: u64,
    pub incumbent_side: usize,
    pub route_at_side: [usize; 2],
    pub trajectories: Vec<Trajectory>,
    pub phase_controls: Vec<Trajectory>,
    pub equal_controls: Vec<EqualControl>,
    pub reversal_thresholds: Vec<(usize, Option<OpportunityRatio>)>,
    pub exposure_monotonic: bool,
    pub allocation_monotonic: bool,
    pub maturity_monotonic: bool,
    pub stale_blocked: bool,
    pub postclosure_inert: bool,
    pub anti_adaptation_audit: bool,
    pub count_capacity_safe: bool,
    pub controls_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub protocol: &'static str,
    pub stage: &'static str,
    pub cells: Vec<Cell>,
    pub classification: &'static str,
    pub exposure_varied: bool,
    pub phase_map_monotonic: bool,
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

    fn maturities(self) -> &'static [usize] {
        match self {
            Self::Probe => &PROBE_MATURITIES,
            Self::Micro | Self::Gate => &FULL_MATURITIES,
        }
    }

    fn ratios(self) -> &'static [OpportunityRatio] {
        match self {
            Self::Probe => &PROBE_RATIOS,
            Self::Micro | Self::Gate => &FULL_RATIOS,
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
    let base = identity.wrapping_mul(10_000).wrapping_add(4_000_000);
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
                    .expect("route has one physical side");
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

fn opportunity_is_alternative(clock: usize, ratio: OpportunityRatio, offset: usize) -> bool {
    let shifted = clock + offset;
    ((shifted + 1) * ratio.alternative) / ratio.period()
        > (shifted * ratio.alternative) / ratio.period()
}

fn variable_variant(clock: usize) -> usize {
    (clock.wrapping_mul(3) + clock / 7 + clock / 31) % 4
}

fn classify(landscape: &Landscape, route_at_role: [usize; 2]) -> LandscapeClass {
    let live = route_at_role.map(|route| landscape.live_supporters[route]);
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

fn independent_realizations(
    live: [usize; 2],
    route_at_side: [usize; 2],
    reverse: bool,
    identity: u64,
    duplicate_exact: &mut bool,
) -> [usize; 2] {
    let mut by_side = [0; 2];
    for side in 0..2 {
        for duplicate in 0..4u64 {
            let resolution = resolve_exact(
                live,
                &PhysicalWorld {
                    opportunity_side: None,
                    postclosure: false,
                    favor_side: Some(side),
                    route_at_side,
                    stale_route: [false; 2],
                    reverse_allocation: reverse ^ duplicate.is_multiple_of(2),
                },
                identity + side as u64 * 10 + duplicate,
                duplicate_exact,
            );
            if resolution.realized_side == Some(side) {
                by_side[side] += 1;
            }
        }
    }
    by_side
}

fn checkpoint(
    budget: usize,
    scheduled: [usize; 2],
    executions: [usize; 2],
    consequences: [usize; 2],
    unresolved: usize,
    session: &frozen_ssa1::Adapter,
    incumbent_side: usize,
    route_at_side: [usize; 2],
    reverse: bool,
    identity: u64,
    duplicate_exact: &mut bool,
) -> Checkpoint {
    let audit = session.audit();
    let landscape = session.inspect();
    let independent_realizations = independent_realizations(
        landscape.live_supporters,
        route_at_side,
        reverse,
        identity,
        duplicate_exact,
    );
    let route_at_role = [
        route_at_side[incumbent_side],
        route_at_side[1 - incumbent_side],
    ];
    Checkpoint {
        budget,
        scheduled,
        executions,
        consequences,
        unresolved,
        audit,
        class: classify(&landscape, route_at_role),
        landscape,
        independent_realizations,
    }
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

#[derive(Clone, Copy)]
struct TrajectorySpec {
    maturity: usize,
    ratio: OpportunityRatio,
    phase_offset: usize,
    incumbent_side: usize,
    route_at_side: [usize; 2],
    reverse: bool,
    equal_consequences: bool,
}

fn trajectory(
    mut session: frozen_ssa1::Adapter,
    initial_executions: usize,
    spec: TrajectorySpec,
    identity: u64,
    duplicate_exact: &mut bool,
) -> Trajectory {
    let boundary_audit = session.audit();
    let boundary_landscape = session.inspect();
    let mut scheduled = [0; 2];
    let mut executions = [0; 2];
    let mut consequences = [0; 2];
    let mut unresolved = 0;
    let mut checkpoints = Vec::with_capacity(CHECKPOINTS.len());
    let mut next_checkpoint = 0;
    for opportunity in 0..=EPISODES {
        while next_checkpoint < CHECKPOINTS.len() && CHECKPOINTS[next_checkpoint] == opportunity {
            checkpoints.push(checkpoint(
                opportunity,
                scheduled,
                executions,
                consequences,
                unresolved,
                &session,
                spec.incumbent_side,
                spec.route_at_side,
                spec.reverse,
                identity + 50_000_000 + opportunity as u64,
                duplicate_exact,
            ));
            next_checkpoint += 1;
        }
        if opportunity == EPISODES {
            break;
        }

        // Exogenous opportunity is fixed before the learner is queried.
        let alternative = opportunity_is_alternative(opportunity, spec.ratio, spec.phase_offset);
        let role = usize::from(alternative);
        let side = if alternative {
            1 - spec.incumbent_side
        } else {
            spec.incumbent_side
        };
        scheduled[role] += 1;
        let live = session.offer();
        let physical = resolve_exact(
            live,
            &PhysicalWorld {
                opportunity_side: Some(side),
                postclosure: false,
                favor_side: None,
                route_at_side: spec.route_at_side,
                stale_route: [false; 2],
                reverse_allocation: spec.reverse ^ opportunity.is_multiple_of(2),
            },
            identity + opportunity as u64,
            duplicate_exact,
        );
        if let (Some(route), Some(realized_side)) =
            (physical.realized_route, physical.realized_side)
        {
            let realized_role = usize::from(realized_side != spec.incumbent_side);
            executions[realized_role] += 1;
            let variant = if spec.equal_consequences || realized_role == 1 {
                0
            } else {
                variable_variant(opportunity + 17)
            };
            let before = session.audit().routes[route].evidence_observations;
            let _ = session.return_consequence(route, variant);
            let after = session.audit().routes[route].evidence_observations;
            consequences[realized_role] += usize::from(after > before);
        } else {
            unresolved += 1;
        }
    }
    let expected_alternative = EPISODES * spec.ratio.alternative / spec.ratio.period();
    let schedule_exact = scheduled == [EPISODES - expected_alternative, expected_alternative];
    let exposure_transferred =
        executions[0] > 0 && executions[1] > 0 && consequences[0] > 0 && consequences[1] > 0;
    Trajectory {
        maturity: spec.maturity,
        ratio: spec.ratio,
        phase_offset: spec.phase_offset,
        incumbent_side: spec.incumbent_side,
        route_at_side: spec.route_at_side,
        initial_executions,
        boundary_audit,
        boundary_landscape,
        checkpoints,
        schedule_exact,
        exposure_transferred,
        duplicate_exact: *duplicate_exact,
    }
}

fn threshold(
    trajectories: &[Trajectory],
    maturity: usize,
    ratios: &[OpportunityRatio],
) -> Option<OpportunityRatio> {
    ratios.iter().copied().find(|ratio| {
        trajectories.iter().any(|trajectory| {
            trajectory.maturity == maturity
                && trajectory.ratio == *ratio
                && trajectory.phase_offset == 0
                && trajectory
                    .checkpoints
                    .last()
                    .is_some_and(|point| point.class == LandscapeClass::Alternative)
        })
    })
}

fn monotonic_maps(
    trajectories: &[Trajectory],
    maturities: &[usize],
    ratios: &[OpportunityRatio],
) -> (bool, bool, bool) {
    let exposure_monotonic = maturities.iter().all(|maturity| {
        let points: Vec<_> = ratios
            .iter()
            .filter_map(|ratio| {
                trajectories.iter().find(|trajectory| {
                    trajectory.maturity == *maturity
                        && trajectory.ratio == *ratio
                        && trajectory.phase_offset == 0
                })
            })
            .collect();
        points.windows(2).all(|pair| {
            let left = pair[0].checkpoints.last().unwrap();
            let right = pair[1].checkpoints.last().unwrap();
            let left_alternative_route = pair[0].route_at_side[1 - pair[0].incumbent_side];
            let right_alternative_route = pair[1].route_at_side[1 - pair[1].incumbent_side];
            left.executions[1] <= right.executions[1]
                && left.audit.routes[left_alternative_route].evidence_observations
                    <= right.audit.routes[right_alternative_route].evidence_observations
        })
    });
    let allocation_monotonic = maturities.iter().all(|maturity| {
        let ranks: Vec<_> = ratios
            .iter()
            .filter_map(|ratio| {
                trajectories
                    .iter()
                    .find(|trajectory| {
                        trajectory.maturity == *maturity
                            && trajectory.ratio == *ratio
                            && trajectory.phase_offset == 0
                    })
                    .and_then(|trajectory| trajectory.checkpoints.last())
                    .map(|point| point.class.adaptation_rank())
            })
            .collect();
        ranks.windows(2).all(|pair| pair[0] <= pair[1])
    });
    let indices: Vec<_> = maturities
        .iter()
        .map(|maturity| {
            threshold(trajectories, *maturity, ratios)
                .and_then(|ratio| ratios.iter().position(|candidate| *candidate == ratio))
                .unwrap_or(ratios.len())
        })
        .collect();
    let maturity_monotonic = indices.windows(2).all(|pair| pair[0] <= pair[1]);
    (exposure_monotonic, allocation_monotonic, maturity_monotonic)
}

fn anti_adaptation_source_audit() -> bool {
    let source = include_str!("ssa1_s_exposure_bias_map.rs");
    let prefix = source
        .split("fn anti_adaptation_source_audit()")
        .next()
        .unwrap_or_default();
    let mechanism = prefix.split("fn trajectory(").nth(1).unwrap_or_default();
    mechanism.contains("let alternative = opportunity_is_alternative(")
        && mechanism.find("let alternative = opportunity_is_alternative(")
            < mechanism.find("let live = session.offer();")
        && !mechanism.contains("random(")
        && !mechanism.contains("softmax")
        && !mechanism.contains("propensity")
        && !mechanism.contains("replay_buffer")
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

fn physical_controls(
    seed: u64,
    incumbent_side: usize,
    route_at_side: [usize; 2],
    reverse: bool,
    duplicate_exact: &mut bool,
) -> (bool, bool) {
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
        ..early
    };
    let early_resolution = resolve_exact(live, &early, seed, duplicate_exact);
    let stale_resolution = resolve_exact(live, &stale, seed + 1, duplicate_exact);
    let baseline_resolution = resolve_exact(live, &baseline, seed + 2, duplicate_exact);
    let late_resolution = resolve_exact(live, &late, seed + 2, duplicate_exact);
    (
        early_resolution.realized_route == Some(alternative_route)
            && stale_resolution.realized_route != Some(alternative_route),
        late_resolution.realized_route == baseline_resolution.realized_route
            && late_resolution.opportunity_visible == 3,
    )
}

fn run_cell(stage: Stage, seed: u64, index: usize) -> Cell {
    let incumbent_side = index % 2;
    let route_at_side = if index.is_multiple_of(2) {
        [0, 1]
    } else {
        [1, 0]
    };
    let reverse = seed.is_multiple_of(2);
    let mut duplicate_exact = true;
    let mut trajectories = Vec::new();
    let mut phase_controls = Vec::new();
    let mut equal_controls = Vec::new();
    for maturity in stage.maturities().iter().copied() {
        let (boundary, initial_executions) = mature_session(
            seed + maturity as u64 * 1_000_000,
            maturity,
            incumbent_side,
            route_at_side,
            reverse,
            &mut duplicate_exact,
        );
        for (ratio_index, ratio) in stage.ratios().iter().copied().enumerate() {
            let spec = TrajectorySpec {
                maturity,
                ratio,
                phase_offset: 0,
                incumbent_side,
                route_at_side,
                reverse,
                equal_consequences: false,
            };
            trajectories.push(trajectory(
                boundary.clone(),
                initial_executions,
                spec,
                seed + 100_000_000 + maturity as u64 * 1_000_000 + ratio_index as u64 * 100_000,
                &mut duplicate_exact,
            ));
            if !matches!(stage, Stage::Probe) || ratio == FULL_RATIOS[3] {
                phase_controls.push(trajectory(
                    boundary.clone(),
                    initial_executions,
                    TrajectorySpec {
                        phase_offset: 1,
                        ..spec
                    },
                    seed + 200_000_000 + maturity as u64 * 1_000_000 + ratio_index as u64 * 100_000,
                    &mut duplicate_exact,
                ));
            }
        }
        let equal = trajectory(
            boundary,
            initial_executions,
            TrajectorySpec {
                maturity,
                ratio: FULL_RATIOS[3],
                phase_offset: 0,
                incumbent_side,
                route_at_side,
                reverse,
                equal_consequences: true,
            },
            seed + 300_000_000 + maturity as u64 * 1_000_000,
            &mut duplicate_exact,
        );
        equal_controls.push(EqualControl {
            maturity,
            final_checkpoint: equal.checkpoints.last().unwrap().clone(),
            schedule_exact: equal.schedule_exact,
            exposure_transferred: equal.exposure_transferred,
            duplicate_exact: equal.duplicate_exact,
        });
    }
    let reversal_thresholds = stage
        .maturities()
        .iter()
        .copied()
        .map(|maturity| (maturity, threshold(&trajectories, maturity, stage.ratios())))
        .collect();
    let (exposure_monotonic, allocation_monotonic, maturity_monotonic) =
        monotonic_maps(&trajectories, stage.maturities(), stage.ratios());
    let phase_controls_match = phase_controls.iter().all(|control| {
        trajectories.iter().any(|baseline| {
            baseline.maturity == control.maturity
                && baseline.ratio == control.ratio
                && baseline.checkpoints.last().map(|point| point.class)
                    == control.checkpoints.last().map(|point| point.class)
        })
    });
    let (stale_blocked, postclosure_inert) = physical_controls(
        seed + 400_000_000,
        incumbent_side,
        route_at_side,
        reverse,
        &mut duplicate_exact,
    );
    let anti_adaptation_audit = anti_adaptation_source_audit();
    let count_capacity_safe = trajectories
        .iter()
        .chain(&phase_controls)
        .all(|trajectory| {
            trajectory.checkpoints.iter().all(|point| {
                point
                    .audit
                    .routes
                    .iter()
                    .all(|route| route.evidence_observations <= u64::from(u16::MAX))
            })
        });
    let controls_passed = trajectories
        .iter()
        .chain(&phase_controls)
        .all(|trajectory| {
            trajectory.schedule_exact
                && trajectory.duplicate_exact
                && trajectory.initial_executions == trajectory.maturity
        })
        && equal_controls.iter().all(|control| {
            control.schedule_exact && control.exposure_transferred && control.duplicate_exact
        })
        && phase_controls_match
        && stale_blocked
        && postclosure_inert
        && anti_adaptation_audit
        && count_capacity_safe
        && duplicate_exact;
    Cell {
        seed,
        incumbent_side,
        route_at_side,
        trajectories,
        phase_controls,
        equal_controls,
        reversal_thresholds,
        exposure_monotonic,
        allocation_monotonic,
        maturity_monotonic,
        stale_blocked,
        postclosure_inert,
        anti_adaptation_audit,
        count_capacity_safe,
        controls_passed,
    }
}

fn report(stage: Stage, seeds: &[u64]) -> Report {
    let cells: Vec<_> = seeds
        .iter()
        .enumerate()
        .map(|(index, seed)| run_cell(stage, *seed, index))
        .collect();
    let exposure_varied = cells.iter().all(|cell| {
        stage.maturities().iter().all(|maturity| {
            let exposures: Vec<_> = stage
                .ratios()
                .iter()
                .filter_map(|ratio| {
                    cell.trajectories
                        .iter()
                        .find(|trajectory| {
                            trajectory.maturity == *maturity && trajectory.ratio == *ratio
                        })
                        .and_then(|trajectory| trajectory.checkpoints.last())
                        .map(|point| point.executions[1])
                })
                .collect();
            exposures.first() < exposures.last()
        })
    });
    let phase_map_monotonic = cells.iter().all(|cell| {
        cell.exposure_monotonic && cell.allocation_monotonic && cell.maturity_monotonic
    });
    let allocation_varied = cells.iter().all(|cell| {
        stage.maturities().iter().any(|maturity| {
            let classes: Vec<_> = stage
                .ratios()
                .iter()
                .filter_map(|ratio| {
                    cell.trajectories
                        .iter()
                        .find(|trajectory| {
                            trajectory.maturity == *maturity && trajectory.ratio == *ratio
                        })
                        .and_then(|trajectory| trajectory.checkpoints.last())
                        .map(|point| point.class)
                })
                .collect();
            classes.windows(2).any(|pair| pair[0] != pair[1])
        })
    });
    let controls_passed = cells.iter().all(|cell| cell.controls_passed);
    let classification = if controls_passed && exposure_varied && !allocation_varied {
        "C — exposure-insensitive allocation"
    } else if controls_passed && exposure_varied && phase_map_monotonic {
        "A — coherent selection-induced exposure phase map"
    } else if controls_passed && exposure_varied && allocation_varied {
        "B — non-monotonic but resolved exposure map"
    } else if controls_passed {
        "D — physical opportunity does not transfer"
    } else {
        "E — scientific ambiguity"
    };
    let frozen_parent_exact = frozen_source_invariant();
    Report {
        protocol: PROTOCOL,
        stage: stage.name(),
        cells,
        classification,
        exposure_varied,
        phase_map_monotonic,
        frozen_parent_exact,
        claim_eligible: false,
        passed: frozen_parent_exact && controls_passed,
    }
}

pub fn run_probe() -> Report {
    report(Stage::Probe, &[1_900_000_000, 1_900_000_001])
}

pub fn run_micro() -> Report {
    report(
        Stage::Micro,
        &[1_910_000_000, 1_920_000_001, 1_910_000_002, 1_920_000_003],
    )
}

pub fn run_gate() -> Report {
    report(
        Stage::Gate,
        &[
            1_930_000_000,
            1_940_000_001,
            1_950_000_002,
            1_960_000_003,
            1_970_000_004,
            1_980_000_005,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "explicit SSA1-S PROBE"]
    fn probe_measures_exposure_transfer() {
        assert!(run_probe().passed);
    }

    #[test]
    #[ignore = "explicit SSA1-S MICRO"]
    fn micro_freezes_exposure_phase_map() {
        assert!(run_micro().passed);
    }

    #[test]
    #[ignore = "explicit SSA1-S GATE"]
    fn gate_transfers_exposure_phase_map() {
        assert!(run_gate().passed);
    }
}
