//! SSA1-R development: autonomous affordance adaptation in a fixed rich world.

use std::collections::BTreeMap;

use crate::organism::{ArrowSpec, CellSpec, SpikeInput, Substrate};
use crate::ssa1_c2_lock_in_hysteresis_map::{Audit, RouteAudit};

pub const PROTOCOL: &str = "ssa1-r-rich-changing-world-v1";
pub const FROZEN_ORGANISM: &str = "2125a197ad0e5796a12668cd76c2071236763af0";
pub const FROZEN_C2: &str = "064b169";

const FIRING_THRESHOLD: i32 = 4;
const INHIBITION: i32 = -64;
const FIELD_AMPLITUDES: [usize; 8] = [0, 1, 2, 3, 1, 3, 2, 0];
const DWELL_TIMES: [usize; 3] = [512, 4_096, 10_000];

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ClockConfig {
    field_offset: usize,
    amplitude_offset: usize,
    consequence_offset: usize,
}

impl Default for ClockConfig {
    fn default() -> Self {
        Self {
            field_offset: 0,
            amplitude_offset: 0,
            consequence_offset: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScheduledField {
    side: usize,
    amplitude: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhysicalWorld {
    field: Option<ScheduledField>,
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
    field_visible: usize,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhaseOutcome {
    pub phase: usize,
    pub stable_side: Option<usize>,
    pub episodes: usize,
    pub executions: [usize; 2],
    pub consequences: [usize; 2],
    pub field_visits: [[usize; 4]; 2],
    pub start_audit: Audit,
    pub end_audit: Audit,
    pub landscape: Landscape,
    pub independent_realizations: [usize; 2],
    pub field_balanced: bool,
    pub dominance_correct: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StationaryWorld {
    pub name: &'static str,
    pub outcome: PhaseOutcome,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangingWorld {
    pub name: &'static str,
    pub dwell: usize,
    pub phases: Vec<PhaseOutcome>,
    pub tracked_phases: usize,
    pub fully_tracked: bool,
    pub comparative_evidence: bool,
    pub passed_controls: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub seed: u64,
    pub route_at_side: [usize; 2],
    pub stationary_winner: StationaryWorld,
    pub multi_useful: StationaryWorld,
    pub changing: Vec<ChangingWorld>,
    pub no_field_control: ChangingWorld,
    pub postclosure_control: ChangingWorld,
    pub clock_phase_control: ChangingWorld,
    pub stale_control: bool,
    pub anti_pairing_audit: bool,
    pub duplicate_exact: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub protocol: &'static str,
    pub stage: &'static str,
    pub cells: Vec<Cell>,
    pub classification: &'static str,
    pub best_dwell: Option<usize>,
    pub frozen_parent_exact: bool,
    pub anti_pairing_audit: bool,
    pub count_capacity_safe: bool,
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

    fn dwell_times(self) -> &'static [usize] {
        match self {
            Self::Probe => &[4_096],
            Self::Micro | Self::Gate => &DWELL_TIMES,
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

fn field_at(clock: usize, config: ClockConfig) -> ScheduledField {
    let field_clock = clock + config.field_offset;
    ScheduledField {
        side: (field_clock.count_ones() as usize) % 2,
        amplitude: FIELD_AMPLITUDES[(clock + config.amplitude_offset) % 8],
    }
}

fn variable_variant(clock: usize, config: ClockConfig) -> usize {
    let shifted = clock + config.consequence_offset;
    (shifted.wrapping_mul(3) + shifted / 7 + shifted / 31) % 4
}

fn resolve(live: [usize; 2], world: &PhysicalWorld, identity: u64) -> Resolution {
    let base = identity.wrapping_mul(10_000).wrapping_add(3_000_000);
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
                    .unwrap();
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
    if let Some(field) = world.field {
        let route = world.route_at_side[field.side];
        for pulse in 0..field.amplitude {
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
        field_visible: world.field.map_or(0, |field| field.amplitude),
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
                    field: None,
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

#[derive(Clone, Copy)]
struct PhaseSpec {
    phase: usize,
    episodes: usize,
    stable_side: Option<usize>,
    field_enabled: bool,
    postclosure: bool,
    config: ClockConfig,
}

fn run_phase(
    session: &mut frozen_ssa1::Adapter,
    clock: &mut usize,
    spec: PhaseSpec,
    route_at_side: [usize; 2],
    reverse: bool,
    identity: u64,
    duplicate_exact: &mut bool,
) -> PhaseOutcome {
    let start_audit = session.audit();
    let mut executions = [0; 2];
    let mut consequences = [0; 2];
    let mut field_visits: [[usize; 4]; 2] = [[0; 4]; 2];
    for episode in 0..spec.episodes {
        // The complete physical episode is scheduled before the organism is
        // queried. Neither learned state nor the consequence regime enters
        // the field law.
        let scheduled = field_at(*clock, spec.config);
        if spec.field_enabled {
            field_visits[scheduled.side][scheduled.amplitude] += 1;
        }
        let live = session.offer();
        let world = PhysicalWorld {
            field: spec.field_enabled.then_some(scheduled),
            postclosure: spec.postclosure,
            favor_side: None,
            route_at_side,
            stale_route: [false; 2],
            reverse_allocation: reverse ^ episode.is_multiple_of(2),
        };
        let physical = resolve_exact(live, &world, identity + *clock as u64, duplicate_exact);
        if let (Some(route), Some(side)) = (physical.realized_route, physical.realized_side) {
            executions[side] += 1;
            let variant = match spec.stable_side {
                None => 0,
                Some(stable) if side == stable => 0,
                Some(_) => variable_variant(*clock, spec.config),
            };
            let before = session.audit().routes[route].evidence_observations;
            let _ = session.return_consequence(route, variant);
            let after = session.audit().routes[route].evidence_observations;
            consequences[side] += usize::from(after > before);
        }
        *clock += 1;
    }
    let end_audit = session.audit();
    let landscape = session.inspect();
    let independent_realizations = independent_realizations(
        landscape.live_supporters,
        route_at_side,
        reverse,
        identity + 50_000_000,
        duplicate_exact,
    );
    let field_balanced = if spec.field_enabled {
        (0..4).all(|amplitude| field_visits[0][amplitude].abs_diff(field_visits[1][amplitude]) <= 1)
    } else {
        field_visits == [[0; 4]; 2]
    };
    let live_by_side = route_at_side.map(|route| landscape.live_supporters[route]);
    let dominance_correct = match spec.stable_side {
        Some(stable) => {
            live_by_side[stable] == FIRING_THRESHOLD as usize
                && live_by_side[1 - stable] < FIRING_THRESHOLD as usize
                && independent_realizations[stable] > 0
                && independent_realizations[1 - stable] == 0
        }
        None => {
            live_by_side == [FIRING_THRESHOLD as usize; 2]
                && executions.iter().all(|count| *count > 0)
                && independent_realizations.iter().all(|count| *count > 0)
        }
    };
    PhaseOutcome {
        phase: spec.phase,
        stable_side: spec.stable_side,
        episodes: spec.episodes,
        executions,
        consequences,
        field_visits,
        start_audit,
        end_audit,
        landscape,
        independent_realizations,
        field_balanced,
        dominance_correct,
    }
}

fn stationary_world(
    seed: u64,
    route_at_side: [usize; 2],
    reverse: bool,
    stable_side: Option<usize>,
    name: &'static str,
    duplicate_exact: &mut bool,
) -> StationaryWorld {
    let mut session = frozen_ssa1::Adapter::blank(seed, route_at_side[0] == 1);
    let mut clock = 0;
    let outcome = run_phase(
        &mut session,
        &mut clock,
        PhaseSpec {
            phase: 0,
            episodes: 10_000,
            stable_side,
            field_enabled: true,
            postclosure: false,
            config: ClockConfig::default(),
        },
        route_at_side,
        reverse,
        seed + 10_000_000,
        duplicate_exact,
    );
    let comparative = outcome.executions.iter().all(|count| *count > 0)
        && outcome.consequences.iter().all(|count| *count > 0);
    let passed = outcome.dominance_correct && outcome.field_balanced && comparative;
    StationaryWorld {
        name,
        outcome,
        passed,
    }
}

#[derive(Clone, Copy)]
struct ChangingSpec {
    dwell: usize,
    field_enabled: bool,
    postclosure: bool,
    config: ClockConfig,
    stationary: bool,
}

fn changing_world(
    seed: u64,
    route_at_side: [usize; 2],
    reverse: bool,
    name: &'static str,
    spec: ChangingSpec,
    duplicate_exact: &mut bool,
) -> ChangingWorld {
    let mut session = frozen_ssa1::Adapter::blank(seed, route_at_side[0] == 1);
    let mut clock = 0;
    let stable_sides = if spec.stationary {
        [0, 0, 0]
    } else {
        [0, 1, 0]
    };
    let mut phases = Vec::with_capacity(3);
    for (phase, stable_side) in stable_sides.into_iter().enumerate() {
        phases.push(run_phase(
            &mut session,
            &mut clock,
            PhaseSpec {
                phase,
                episodes: spec.dwell,
                stable_side: Some(stable_side),
                field_enabled: spec.field_enabled,
                postclosure: spec.postclosure,
                config: spec.config,
            },
            route_at_side,
            reverse,
            seed + 20_000_000 + phase as u64 * 10_000_000,
            duplicate_exact,
        ));
    }
    let tracked_phases = phases
        .iter()
        .filter(|phase| phase.dominance_correct)
        .count();
    let comparative_evidence = phases.iter().all(|phase| {
        phase.executions.iter().all(|count| *count > 0)
            && phase.consequences.iter().all(|count| *count > 0)
            && phase.end_audit.observations > phase.start_audit.observations
    });
    let fully_tracked = tracked_phases == phases.len();
    let passed_controls = phases.iter().all(|phase| phase.field_balanced);
    ChangingWorld {
        name,
        dwell: spec.dwell,
        phases,
        tracked_phases,
        fully_tracked,
        comparative_evidence,
        passed_controls,
    }
}

fn anti_pairing_source_audit() -> bool {
    let source = include_str!("ssa1_r_rich_changing_world.rs");
    let mechanism = source
        .split("fn anti_pairing_source_audit()")
        .next()
        .unwrap_or_default();
    mechanism.contains("let scheduled = field_at(*clock, spec.config);")
        && mechanism.find("let scheduled = field_at(*clock, spec.config);")
            < mechanism.find("let live = session.offer();")
        && mechanism.contains("if let (Some(route), Some(side))")
        && !mechanism.contains("random(")
        && !mechanism.contains("softmax")
        && !mechanism.contains("evaluator_selected_winner")
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

fn stale_control(seed: u64, route_at_side: [usize; 2], reverse: bool) -> bool {
    let suppressed_side = 1;
    let stale_route = route_at_side[suppressed_side];
    let live = if stale_route == 0 { [1, 4] } else { [4, 1] };
    let world = PhysicalWorld {
        field: Some(ScheduledField {
            side: suppressed_side,
            amplitude: 3,
        }),
        postclosure: false,
        favor_side: None,
        route_at_side,
        stale_route: [stale_route == 0, stale_route == 1],
        reverse_allocation: reverse,
    };
    resolve(live, &world, seed).realized_route != Some(stale_route)
}

fn run_cell(stage: Stage, seed: u64, index: usize) -> Cell {
    let route_at_side = if index.is_multiple_of(2) {
        [0, 1]
    } else {
        [1, 0]
    };
    let reverse = seed.is_multiple_of(2);
    let mut duplicate_exact = true;
    let stationary_winner = stationary_world(
        seed + 1_000_000,
        route_at_side,
        reverse,
        Some(index % 2),
        "R0-stationary-winner",
        &mut duplicate_exact,
    );
    let multi_useful = stationary_world(
        seed + 2_000_000,
        route_at_side,
        !reverse,
        None,
        "R1-multi-useful",
        &mut duplicate_exact,
    );
    let changing: Vec<_> = stage
        .dwell_times()
        .iter()
        .copied()
        .map(|dwell| {
            changing_world(
                seed + 30_000_000 + dwell as u64,
                route_at_side,
                reverse,
                "R2-rich-changing",
                ChangingSpec {
                    dwell,
                    field_enabled: true,
                    postclosure: false,
                    config: ClockConfig::default(),
                    stationary: false,
                },
                &mut duplicate_exact,
            )
        })
        .collect();
    let control_dwell = 4_096;
    let no_field_control = changing_world(
        seed + 60_000_000,
        route_at_side,
        reverse,
        "C0-no-field",
        ChangingSpec {
            dwell: control_dwell,
            field_enabled: false,
            postclosure: false,
            config: ClockConfig::default(),
            stationary: false,
        },
        &mut duplicate_exact,
    );
    let postclosure_control = changing_world(
        seed + 70_000_000,
        route_at_side,
        reverse,
        "C1-postclosure-field",
        ChangingSpec {
            dwell: control_dwell,
            field_enabled: true,
            postclosure: true,
            config: ClockConfig::default(),
            stationary: false,
        },
        &mut duplicate_exact,
    );
    let clock_phase_control = changing_world(
        seed + 80_000_000,
        route_at_side,
        !reverse,
        "C2-clock-phase",
        ChangingSpec {
            dwell: control_dwell,
            field_enabled: true,
            postclosure: false,
            config: ClockConfig {
                field_offset: 8,
                amplitude_offset: 3,
                consequence_offset: 11,
            },
            stationary: false,
        },
        &mut duplicate_exact,
    );
    let stationary_control = changing_world(
        seed + 90_000_000,
        route_at_side,
        reverse,
        "C3-stationary-consequence",
        ChangingSpec {
            dwell: control_dwell,
            field_enabled: true,
            postclosure: false,
            config: ClockConfig::default(),
            stationary: true,
        },
        &mut duplicate_exact,
    );
    let no_field_not_better = no_field_control.tracked_phases
        <= changing
            .iter()
            .find(|world| world.dwell == control_dwell)
            .map_or(usize::MAX, |world| world.tracked_phases);
    let postclosure_inert = postclosure_control
        .phases
        .iter()
        .zip(&no_field_control.phases)
        .all(|(post, absent)| {
            post.landscape == absent.landscape
                && post.executions == absent.executions
                && post.field_visits.iter().flatten().sum::<usize>() > 0
        });
    let stationary_no_toggle = stationary_control
        .phases
        .iter()
        .all(|phase| phase.dominance_correct);
    let stale_control = stale_control(seed + 100_000_000, route_at_side, reverse);
    let anti_pairing_audit = anti_pairing_source_audit();
    let controls = no_field_not_better
        && postclosure_inert
        && stationary_no_toggle
        && clock_phase_control.passed_controls
        && stale_control;
    let passed = stationary_winner.passed
        && multi_useful.passed
        && changing.iter().all(|world| world.passed_controls)
        && controls
        && duplicate_exact
        && anti_pairing_audit;
    Cell {
        seed,
        route_at_side,
        stationary_winner,
        multi_useful,
        changing,
        no_field_control,
        postclosure_control,
        clock_phase_control,
        stale_control,
        anti_pairing_audit,
        duplicate_exact,
        passed,
    }
}

fn report(stage: Stage, seeds: &[u64]) -> Report {
    let cells: Vec<_> = seeds
        .iter()
        .enumerate()
        .map(|(index, seed)| run_cell(stage, *seed, index))
        .collect();
    let best_dwell = stage.dwell_times().iter().copied().find(|dwell| {
        cells.iter().all(|cell| {
            cell.changing
                .iter()
                .find(|world| world.dwell == *dwell)
                .is_some_and(|world| world.fully_tracked)
        })
    });
    let stationary_pass = cells
        .iter()
        .all(|cell| cell.stationary_winner.passed && cell.multi_useful.passed);
    let comparisons_available = cells
        .iter()
        .all(|cell| cell.changing.iter().any(|world| world.comparative_evidence));
    let any_tracking = cells
        .iter()
        .all(|cell| cell.changing.iter().any(|world| world.tracked_phases >= 2));
    let any_alternative = cells.iter().all(|cell| {
        cell.changing.iter().any(|world| {
            world
                .phases
                .iter()
                .all(|phase| phase.executions.iter().all(|count| *count > 0))
        })
    });
    let classification = if stationary_pass && best_dwell.is_some() {
        "A — autonomous rich-world landscape control"
    } else if stationary_pass && any_tracking {
        "B — partial rich-world adaptation"
    } else if comparisons_available {
        "C — contrast available but functionally insufficient"
    } else if !any_alternative {
        "D — natural counterexperience unavailable"
    } else {
        "E — scientific ambiguity"
    };
    let frozen_parent_exact = frozen_source_invariant();
    let anti_pairing_audit = cells.iter().all(|cell| cell.anti_pairing_audit);
    let controls_valid = cells.iter().all(|cell| {
        cell.duplicate_exact
            && cell.stale_control
            && cell.no_field_control.passed_controls
            && cell.postclosure_control.passed_controls
            && cell.clock_phase_control.passed_controls
    });
    Report {
        protocol: PROTOCOL,
        stage: stage.name(),
        cells,
        classification,
        best_dwell,
        frozen_parent_exact,
        anti_pairing_audit,
        count_capacity_safe: true,
        claim_eligible: false,
        passed: frozen_parent_exact && anti_pairing_audit && controls_valid,
    }
}

pub fn run_probe() -> Report {
    report(Stage::Probe, &[1_800_000_000, 1_800_000_001])
}

pub fn run_micro() -> Report {
    report(
        Stage::Micro,
        &[1_810_000_000, 1_820_000_001, 1_810_000_002, 1_820_000_003],
    )
}

pub fn run_gate() -> Report {
    report(
        Stage::Gate,
        &[
            1_830_000_000,
            1_840_000_001,
            1_850_000_002,
            1_860_000_003,
            1_870_000_004,
            1_880_000_005,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "explicit SSA1-R PROBE"]
    fn probe_runs_fixed_rich_world() {
        assert!(run_probe().passed);
    }

    #[test]
    #[ignore = "explicit SSA1-R MICRO"]
    fn micro_freezes_functional_classification() {
        assert!(run_micro().passed);
    }

    #[test]
    #[ignore = "explicit SSA1-R GATE"]
    fn gate_transfers_rich_world_classification() {
        assert!(run_gate().passed);
    }
}
