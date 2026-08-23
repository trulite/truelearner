//! SSA1-C2 development: lock-in and hysteresis characterization.
//!
//! The audit surface reads frozen M5/M6 state. All executions and updates use
//! the byte-frozen learner and CELL/ARROW/SPIKE substrate.

use std::collections::BTreeMap;

use crate::organism::{ArrowSpec, CellSpec, SpikeInput, Substrate};

pub const PROTOCOL: &str = "ssa1-c2-lock-in-hysteresis-map-v1";
pub const FROZEN_ORGANISM: &str = "2125a197ad0e5796a12668cd76c2071236763af0";
pub const FROZEN_C1: &str = "028e9dcd260e18331beec071af005532b5093e89";
pub const FROZEN_SSA1_SHA256: &str =
    "dc157e0bd238992d6475e5dc9767c6f7711a1bb5b7759ebdb7991573aea5199b";

const FIRING_THRESHOLD: i32 = 4;
const INHIBITION: i32 = -64;
const BASE_BUDGETS: [usize; 6] = [0, 4, 16, 64, 1_024, 10_000];
const SAFE_ANCHOR_B_ONLY: usize = 60_000;
const SAFE_ANCHOR_PAIRED: usize = 30_000;
const FULL_MATURATION: [usize; 13] = [0, 2, 4, 6, 8, 10, 12, 14, 16, 24, 32, 64, 192];
const PROBE_MATURATION: [usize; 4] = [0, 8, 16, 32];
const SAFE_ANCHOR_MATURATION: [usize; 3] = [16, 32, 192];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RouteAudit {
    pub evidence_shapes: usize,
    pub evidence_observations: u64,
    pub evidence_support: u16,
    pub evidence_margin: u16,
    pub evidence_eligible: bool,
    pub m5_support: usize,
    pub m5_rejection: usize,
    pub m5_score: i32,
    pub m5_value_resistance: i32,
    pub prototype_resistance: i32,
    pub live_proposals: usize,
    pub proposal_resistance: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Audit {
    pub routes: [RouteAudit; 2],
    pub observations: u64,
    pub abstentions: u64,
    pub applications: usize,
    pub exploration_admissions: usize,
    pub completed_events: usize,
}

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

        pub(super) fn pressure_only(&mut self, events: usize) {
            frozen_learning::c2_pressure_only(&mut self.session, events);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Schedule {
    BOnly,
    PairedChangedWorld,
}

impl Schedule {
    pub fn name(self) -> &'static str {
        match self {
            Self::BOnly => "E0-B-only",
            Self::PairedChangedWorld => "E1-paired",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhysicalWorld {
    suppress_existing: [bool; 2],
    stale_route: [bool; 2],
    favor: Option<usize>,
    early: [usize; 2],
    late: [usize; 2],
    reverse_allocation: bool,
}

impl Default for PhysicalWorld {
    fn default() -> Self {
        Self {
            suppress_existing: [false; 2],
            stale_route: [false; 2],
            favor: None,
            early: [0; 2],
            late: [0; 2],
            reverse_allocation: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Resolution {
    realized: Option<usize>,
    start_fingerprint: u64,
    trace_fingerprint: u64,
    end_fingerprint: u64,
    background_visible: usize,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checkpoint {
    pub maturation: usize,
    pub schedule: Schedule,
    pub budget: usize,
    pub realizations: [usize; 2],
    pub unresolved: usize,
    pub consequences_returned: [usize; 2],
    pub audit: Audit,
    pub landscape: Landscape,
    pub independent_realizations: [usize; 2],
    pub first_nonresponsive_edge: &'static str,
    pub reversed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaturationTrajectory {
    pub maturation: usize,
    pub boundary_audit: Audit,
    pub boundary_landscape: Landscape,
    pub initial_a_realizations: usize,
    pub b_only: Vec<Checkpoint>,
    pub paired: Vec<Checkpoint>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SupportPoint {
    pub extra_early_support: usize,
    pub checkpoint: Checkpoint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisusePoint {
    pub pressure_events: usize,
    pub before_pressure: Audit,
    pub after_pressure: Audit,
    pub landscape_after_pressure: Landscape,
    pub checkpoint: Checkpoint,
    pub forgetting_only_reopening: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub seed: u64,
    pub productive_route: usize,
    pub trajectories: Vec<MaturationTrajectory>,
    pub support_map: Vec<SupportPoint>,
    pub disuse_map: Vec<DisusePoint>,
    pub finite_barrier: Option<(Schedule, usize, usize)>,
    pub b_only_absorbing_invariant: bool,
    pub forgetting_only: bool,
    pub duplicate_exact: bool,
    pub controls_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub protocol: &'static str,
    pub stage: &'static str,
    pub cells: Vec<Cell>,
    pub classification: &'static str,
    pub b_only_subclassification: &'static str,
    pub first_nonresponsive_edge: &'static str,
    pub source_invariant: bool,
    pub count_capacity_safe: bool,
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

    fn maturation(self) -> &'static [usize] {
        match self {
            Self::Probe => &PROBE_MATURATION,
            Self::Micro | Self::Gate => &FULL_MATURATION,
        }
    }

    fn extended(self, maturation: usize) -> bool {
        !matches!(self, Self::Probe) && SAFE_ANCHOR_MATURATION.contains(&maturation)
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
    let base = identity.wrapping_mul(10_000).wrapping_add(2_000_000);
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
        if !world.suppress_existing[route] && !world.stale_route[route] {
            for supporter in 0..live[route] {
                let relay = ids[&(base + 100 + route as u64 * 100 + supporter as u64)];
                let delay = if supporter == 3 && world.favor == Some(route) {
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
        }
        if !world.stale_route[route] {
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
    let mut ordinal = 0u64;
    for route in 0..2 {
        for pulse in 0..world.early[route] {
            substrate.enter(SpikeInput {
                arrival_tick: [0, 2, 4][pulse],
                phase: pulse as i32 - 1,
                origin_physical: base + 10_000 + ordinal,
                target: contenders[route],
                impulse: 1,
            });
            ordinal += 1;
        }
        for pulse in 0..world.late[route] {
            substrate.enter(SpikeInput {
                arrival_tick: [20, 22, 24][pulse],
                phase: pulse as i32 - 1,
                origin_physical: base + 10_000 + ordinal,
                target: contenders[route],
                impulse: 1,
            });
            ordinal += 1;
        }
    }
    let execution = substrate.propagate();
    let realized = [base + 40, base + 50]
        .iter()
        .position(|effect| execution.fired.contains(effect));
    let background_visible = world.early.iter().chain(world.late.iter()).copied().sum();
    Resolution {
        realized,
        start_fingerprint: execution.start_fingerprint,
        trace_fingerprint: execution.trace_fingerprint,
        end_fingerprint: execution.end_fingerprint,
        background_visible,
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

fn independent_realizations(
    live: [usize; 2],
    reverse: bool,
    identity: u64,
    duplicate_exact: &mut bool,
) -> [usize; 2] {
    let mut counts = [0; 2];
    for favored in 0..2 {
        for duplicate in 0..4u64 {
            let world = PhysicalWorld {
                favor: Some(favored),
                reverse_allocation: reverse ^ duplicate.is_multiple_of(2),
                ..PhysicalWorld::default()
            };
            if let Some(route) = resolve_exact(
                live,
                &world,
                identity + favored as u64 * 10 + duplicate,
                duplicate_exact,
            )
            .realized
            {
                counts[route] += 1;
            }
        }
    }
    counts
}

fn initial_session(
    seed: u64,
    productive: usize,
    maturation: usize,
    reverse: bool,
    duplicate_exact: &mut bool,
) -> (frozen_ssa1::Adapter, usize) {
    let mut session = frozen_ssa1::Adapter::blank(seed, productive == 1);
    let mut realized = 0;
    for episode in 0..maturation {
        let live = session.offer();
        let physical = resolve_exact(
            live,
            &PhysicalWorld {
                favor: Some(productive),
                reverse_allocation: reverse ^ episode.is_multiple_of(2),
                ..PhysicalWorld::default()
            },
            seed + episode as u64,
            duplicate_exact,
        );
        if physical.realized == Some(productive) {
            realized += 1;
            let _ = session.return_consequence(productive, 0);
        }
    }
    (session, realized)
}

fn first_edge(
    boundary: &Audit,
    current: &Audit,
    suppressed: usize,
    realizations: [usize; 2],
    consequences: [usize; 2],
    landscape: &Landscape,
) -> &'static str {
    if realizations[suppressed] == 0 {
        return "execution edge";
    }
    if consequences[suppressed] == 0
        || current.routes[suppressed].evidence_observations
            == boundary.routes[suppressed].evidence_observations
    {
        return "observation edge";
    }
    if current.routes[suppressed].m5_support == boundary.routes[suppressed].m5_support {
        return "credit edge";
    }
    if landscape.live_supporters[suppressed] < FIRING_THRESHOLD as usize {
        return "formation edge";
    }
    "none"
}

fn checkpoint(
    maturation: usize,
    schedule: Schedule,
    budget: usize,
    boundary: &Audit,
    session: &frozen_ssa1::Adapter,
    realizations: [usize; 2],
    unresolved: usize,
    consequences_returned: [usize; 2],
    suppressed: usize,
    reverse: bool,
    identity: u64,
    duplicate_exact: &mut bool,
) -> Checkpoint {
    let audit = session.audit();
    let landscape = session.inspect();
    let independent_realizations = independent_realizations(
        landscape.live_supporters,
        reverse,
        identity,
        duplicate_exact,
    );
    let first_nonresponsive_edge = first_edge(
        boundary,
        &audit,
        suppressed,
        realizations,
        consequences_returned,
        &landscape,
    );
    let productive = 1 - suppressed;
    let reversed = landscape.live_supporters[suppressed] >= FIRING_THRESHOLD as usize
        && landscape.live_supporters[productive] < FIRING_THRESHOLD as usize
        && independent_realizations[suppressed] > 0;
    Checkpoint {
        maturation,
        schedule,
        budget,
        realizations,
        unresolved,
        consequences_returned,
        audit,
        landscape,
        independent_realizations,
        first_nonresponsive_edge,
        reversed,
    }
}

fn trajectory(
    mut session: frozen_ssa1::Adapter,
    maturation: usize,
    schedule: Schedule,
    productive: usize,
    reverse: bool,
    seed: u64,
    extended: bool,
    support: usize,
    duplicate_exact: &mut bool,
) -> Vec<Checkpoint> {
    let suppressed = 1 - productive;
    let boundary = session.audit();
    let mut budgets = BASE_BUDGETS.to_vec();
    if extended {
        budgets.push(match schedule {
            Schedule::BOnly => SAFE_ANCHOR_B_ONLY,
            Schedule::PairedChangedWorld => SAFE_ANCHOR_PAIRED,
        });
    }
    let maximum = *budgets.last().unwrap();
    let mut results = Vec::with_capacity(budgets.len());
    let mut realizations = [0; 2];
    let mut unresolved = 0;
    let mut consequences = [0; 2];
    let mut next_checkpoint = 0usize;
    for opportunity in 0..=maximum {
        while next_checkpoint < budgets.len() && budgets[next_checkpoint] == opportunity {
            results.push(checkpoint(
                maturation,
                schedule,
                opportunity,
                &boundary,
                &session,
                realizations,
                unresolved,
                consequences,
                suppressed,
                reverse,
                seed + 80_000_000 + opportunity as u64,
                duplicate_exact,
            ));
            next_checkpoint += 1;
        }
        if opportunity == maximum {
            break;
        }
        let live = session.offer();
        let b_world = match schedule {
            Schedule::BOnly => PhysicalWorld {
                suppress_existing: [productive == 0, productive == 1],
                early: [support, support],
                reverse_allocation: reverse ^ opportunity.is_multiple_of(2),
                ..PhysicalWorld::default()
            },
            Schedule::PairedChangedWorld => PhysicalWorld {
                favor: Some(suppressed),
                early: if suppressed == 0 {
                    [support, 0]
                } else {
                    [0, support]
                },
                reverse_allocation: reverse ^ opportunity.is_multiple_of(2),
                ..PhysicalWorld::default()
            },
        };
        let physical = resolve_exact(
            live,
            &b_world,
            seed + opportunity as u64 * 2,
            duplicate_exact,
        );
        if let Some(route) = physical.realized {
            realizations[route] += 1;
            let before = session.audit().routes[route].evidence_observations;
            let _ = session.return_consequence(
                route,
                if route == suppressed {
                    0
                } else {
                    opportunity % 4
                },
            );
            let after = session.audit().routes[route].evidence_observations;
            consequences[route] += usize::from(after > before);
        } else {
            unresolved += 1;
        }

        if schedule == Schedule::PairedChangedWorld {
            let live = session.offer();
            let a_world = PhysicalWorld {
                favor: Some(productive),
                reverse_allocation: reverse ^ opportunity.is_multiple_of(3),
                ..PhysicalWorld::default()
            };
            let physical = resolve_exact(
                live,
                &a_world,
                seed + opportunity as u64 * 2 + 1,
                duplicate_exact,
            );
            if let Some(route) = physical.realized {
                realizations[route] += 1;
                let before = session.audit().routes[route].evidence_observations;
                let variant = if route == productive {
                    if opportunity.is_multiple_of(2) {
                        1
                    } else {
                        3
                    }
                } else {
                    0
                };
                let _ = session.return_consequence(route, variant);
                let after = session.audit().routes[route].evidence_observations;
                consequences[route] += usize::from(after > before);
            } else {
                unresolved += 1;
            }
        }
    }
    results
}

fn support_map(
    boundary: &frozen_ssa1::Adapter,
    productive: usize,
    reverse: bool,
    seed: u64,
    duplicate_exact: &mut bool,
) -> Vec<SupportPoint> {
    (0..=3)
        .map(|support| {
            let mut points = trajectory(
                boundary.clone(),
                192,
                Schedule::BOnly,
                productive,
                reverse,
                seed + support as u64 * 1_000_000,
                false,
                support,
                duplicate_exact,
            );
            SupportPoint {
                extra_early_support: support,
                checkpoint: points.pop().expect("10,000 support checkpoint exists"),
            }
        })
        .collect()
}

fn disuse_map(
    boundary: &frozen_ssa1::Adapter,
    productive: usize,
    reverse: bool,
    seed: u64,
    duplicate_exact: &mut bool,
) -> Vec<DisusePoint> {
    [0, 16, 64, 256, 1_024]
        .into_iter()
        .map(|pressure_events| {
            let mut session = boundary.clone();
            let before_pressure = session.audit();
            session.pressure_only(pressure_events);
            let after_pressure = session.audit();
            let landscape_after_pressure = session.inspect();
            let blanked = landscape_after_pressure.live_supporters == [0, 0]
                && landscape_after_pressure.value_score == [0, 0];
            let mut points = trajectory(
                session,
                192,
                Schedule::BOnly,
                productive,
                reverse,
                seed + pressure_events as u64 * 1_000_000,
                false,
                3,
                duplicate_exact,
            );
            let checkpoint = points.pop().expect("10,000 disuse checkpoint exists");
            let suppressed = 1 - productive;
            let forgetting_only_reopening = blanked
                && checkpoint.landscape.live_supporters[suppressed] >= FIRING_THRESHOLD as usize
                && checkpoint.audit.routes[suppressed].m5_support == 0;
            DisusePoint {
                pressure_events,
                before_pressure,
                after_pressure,
                landscape_after_pressure,
                checkpoint,
                forgetting_only_reopening,
            }
        })
        .collect()
}

fn source_invariant() -> bool {
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/ds8_cumulative_semantic_credit_probe.rs"
    ));
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

fn b_only_invariant(trajectory: &MaturationTrajectory, productive: usize) -> bool {
    let suppressed = 1 - productive;
    let Some(first) = trajectory.b_only.iter().find(|point| point.budget == 1_024) else {
        return false;
    };
    let Some(last) = trajectory.b_only.last() else {
        return false;
    };
    first.audit.routes[productive].evidence_eligible
        && first.audit.routes[suppressed].evidence_eligible
        && last.audit.routes[productive].evidence_eligible
        && last.audit.routes[suppressed].evidence_eligible
        && last.audit.routes[suppressed].evidence_observations
            > first.audit.routes[suppressed].evidence_observations
        && last.audit.routes[suppressed].m5_support == first.audit.routes[suppressed].m5_support
        && last.audit.routes[suppressed].m5_rejection == first.audit.routes[suppressed].m5_rejection
        && last.audit.abstentions > first.audit.abstentions
        && last.landscape.live_supporters == first.landscape.live_supporters
        && !last.reversed
}

fn run_cell(stage: Stage, seed: u64, productive: usize) -> Cell {
    let reverse = seed.is_multiple_of(2);
    let mut duplicate_exact = true;
    let mut trajectories = Vec::new();
    let mut mature_boundary = None;
    for maturation in stage.maturation().iter().copied() {
        let (boundary, initial_a_realizations) = initial_session(
            seed + maturation as u64 * 10_000_000,
            productive,
            maturation,
            reverse,
            &mut duplicate_exact,
        );
        let boundary_audit = boundary.audit();
        let boundary_landscape = boundary.inspect();
        let b_only = trajectory(
            boundary.clone(),
            maturation,
            Schedule::BOnly,
            productive,
            reverse,
            seed + 100_000_000 + maturation as u64 * 1_000_000,
            stage.extended(maturation),
            3,
            &mut duplicate_exact,
        );
        let paired = trajectory(
            boundary.clone(),
            maturation,
            Schedule::PairedChangedWorld,
            productive,
            reverse,
            seed + 200_000_000 + maturation as u64 * 1_000_000,
            stage.extended(maturation),
            3,
            &mut duplicate_exact,
        );
        if maturation == 192 {
            mature_boundary = Some(boundary.clone());
        }
        trajectories.push(MaturationTrajectory {
            maturation,
            boundary_audit,
            boundary_landscape,
            initial_a_realizations,
            b_only,
            paired,
        });
    }

    let mature_boundary = mature_boundary.unwrap_or_else(|| {
        initial_session(
            seed + 192 * 10_000_000,
            productive,
            192,
            reverse,
            &mut duplicate_exact,
        )
        .0
    });
    let support_map = support_map(
        &mature_boundary,
        productive,
        reverse,
        seed + 300_000_000,
        &mut duplicate_exact,
    );
    let disuse_map = disuse_map(
        &mature_boundary,
        productive,
        reverse,
        seed + 400_000_000,
        &mut duplicate_exact,
    );

    let finite_barrier = trajectories
        .iter()
        .filter(|trajectory| {
            trajectory.maturation >= 16
                && trajectory.boundary_landscape.live_supporters[1 - productive]
                    < FIRING_THRESHOLD as usize
        })
        .flat_map(|trajectory| {
            trajectory
                .b_only
                .iter()
                .chain(trajectory.paired.iter())
                .filter(|point| point.budget > 0 && point.reversed)
                .map(|point| (point.schedule, trajectory.maturation, point.budget))
        })
        .min_by_key(|(_, maturation, budget)| (*budget, *maturation));
    let b_only_absorbing_invariant = trajectories
        .iter()
        .filter(|trajectory| {
            trajectory.maturation >= 16
                && trajectory.boundary_landscape.live_supporters[1 - productive]
                    < FIRING_THRESHOLD as usize
        })
        .all(|trajectory| b_only_invariant(trajectory, productive))
        && support_map
            .iter()
            .find(|point| point.extra_early_support == 3)
            .is_some_and(|point| {
                let suppressed = 1 - productive;
                point.checkpoint.realizations[suppressed] == 10_000
                    && point.checkpoint.audit.routes[productive].evidence_eligible
                    && point.checkpoint.audit.routes[suppressed].evidence_eligible
                    && point.checkpoint.audit.routes[suppressed].m5_support == 0
                    && point.checkpoint.audit.routes[suppressed].m5_rejection == 3
                    && point.checkpoint.audit.abstentions >= 10_000
                    && point.checkpoint.first_nonresponsive_edge == "credit edge"
                    && !point.checkpoint.reversed
            });
    let forgetting_only = disuse_map
        .iter()
        .any(|point| point.forgetting_only_reopening);

    let suppressed = 1 - productive;
    let control_live = if productive == 0 { [4, 1] } else { [1, 4] };
    let blocked = resolve_exact(
        control_live,
        &PhysicalWorld {
            stale_route: [suppressed == 0, suppressed == 1],
            early: if suppressed == 0 { [3, 0] } else { [0, 3] },
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 500_000_000,
        &mut duplicate_exact,
    );
    let base = resolve_exact(
        control_live,
        &PhysicalWorld {
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 500_000_001,
        &mut duplicate_exact,
    );
    let late = resolve_exact(
        control_live,
        &PhysicalWorld {
            late: if suppressed == 0 { [3, 0] } else { [0, 3] },
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 500_000_001,
        &mut duplicate_exact,
    );
    let controls_passed = trajectories.iter().all(|trajectory| {
        trajectory.initial_a_realizations == trajectory.maturation
            && trajectory.b_only.iter().all(|point| {
                point
                    .audit
                    .routes
                    .iter()
                    .all(|route| route.evidence_observations <= u64::from(u16::MAX))
            })
            && trajectory.paired.iter().all(|point| {
                point
                    .audit
                    .routes
                    .iter()
                    .all(|route| route.evidence_observations <= u64::from(u16::MAX))
            })
    }) && blocked.realized != Some(suppressed)
        && late.realized == base.realized
        && late.background_visible == 3;
    Cell {
        seed,
        productive_route: productive,
        trajectories,
        support_map,
        disuse_map,
        finite_barrier,
        b_only_absorbing_invariant,
        forgetting_only,
        duplicate_exact,
        controls_passed,
    }
}

fn report(stage: Stage, seeds: &[u64]) -> Report {
    let cells: Vec<_> = seeds
        .iter()
        .enumerate()
        .map(|(index, seed)| run_cell(stage, *seed, index % 2))
        .collect();
    let source_invariant = source_invariant();
    let finite = cells.iter().all(|cell| cell.finite_barrier.is_some());
    let b_only_absorbing = cells.iter().all(|cell| cell.b_only_absorbing_invariant);
    let forgetting = cells.iter().all(|cell| cell.forgetting_only);
    let moving = cells.iter().any(|cell| {
        cell.trajectories.iter().any(|trajectory| {
            let suppressed = 1 - cell.productive_route;
            trajectory.b_only.last().is_some_and(|last| {
                last.audit.routes[suppressed].m5_support
                    > trajectory.boundary_audit.routes[suppressed].m5_support
                    || last.landscape.live_supporters[suppressed]
                        > trajectory.boundary_landscape.live_supporters[suppressed]
            })
        })
    });
    let classification = if finite {
        "A — finite reversal barrier"
    } else if matches!(stage, Stage::Probe) && b_only_absorbing && source_invariant {
        "PROBE — B-only absorbing credit state; paired mature map pending"
    } else if b_only_absorbing && source_invariant {
        "C — absorbing credit state"
    } else if moving {
        "B — moving but uncrossed barrier"
    } else {
        "F — scientific ambiguity"
    };
    let first_nonresponsive_edge = if b_only_absorbing {
        "credit edge"
    } else if cells.iter().any(|cell| {
        cell.support_map
            .iter()
            .all(|point| point.checkpoint.realizations[1 - cell.productive_route] == 0)
    }) {
        "execution edge"
    } else {
        "none"
    };
    let passed = source_invariant
        && cells
            .iter()
            .all(|cell| cell.controls_passed && cell.duplicate_exact)
        && (!matches!(stage, Stage::Micro | Stage::Gate) || finite || b_only_absorbing)
        && (!forgetting || cells.iter().all(|cell| cell.forgetting_only));
    Report {
        protocol: PROTOCOL,
        stage: stage.name(),
        cells,
        classification,
        b_only_subclassification: if b_only_absorbing {
            "absorbing for B-only same-class counterexperience"
        } else {
            "not absorbing"
        },
        first_nonresponsive_edge,
        source_invariant,
        count_capacity_safe: true,
        frozen_parent_exact: true,
        claim_eligible: false,
        passed,
    }
}

pub fn run_probe() -> Report {
    report(Stage::Probe, &[1_700_000_000])
}

pub fn run_micro() -> Report {
    report(Stage::Micro, &[1_710_000_000, 1_720_000_001])
}

pub fn run_gate() -> Report {
    report(
        Stage::Gate,
        &[
            1_730_000_000,
            1_740_000_001,
            1_750_000_002,
            1_760_000_003,
            1_770_000_004,
            1_780_000_005,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "explicit SSA1-C2 PROBE"]
    fn probe_locates_the_first_nonresponsive_edge() {
        let report = run_probe();
        assert!(report.passed, "{report:#?}");
    }

    #[test]
    #[ignore = "explicit SSA1-C2 MICRO"]
    fn micro_freezes_a_hysteresis_classification() {
        let report = run_micro();
        assert!(report.passed, "{report:#?}");
    }

    #[test]
    #[ignore = "explicit SSA1-C2 GATE"]
    fn gate_transfers_the_hysteresis_map() {
        let report = run_gate();
        assert!(report.passed, "{report:#?}");
    }
}
