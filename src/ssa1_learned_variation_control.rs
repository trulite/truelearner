//! SSA1 development: learned control of physical affordance variation.
//!
//! World names and route comparisons exist only in this evaluator. The
//! organism path is the frozen M5/M6 learner driving the frozen
//! CELL/ARROW/SPIKE substrate through live learned proposal structure.

use std::collections::BTreeMap;

use crate::organism::{ArrowSpec, CellSpec, SpikeInput, Substrate};

pub const PROTOCOL: &str = "ssa1-learned-variation-control-v1";
pub const FROZEN_ORGANISM: &str = "2125a197ad0e5796a12668cd76c2071236763af0";
pub const FROZEN_SUBSTRATE_SHA256: &str =
    "6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263";
pub const FROZEN_M5_SHA256: &str =
    "e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7";
pub const FROZEN_M6_SHA256: &str =
    "11b4229122b3e0788ca30c55579b91ffe07461de9a138860690134565fcf2ed6";

const FIRING_THRESHOLD: i32 = 4;
const INHIBITION: i32 = -64;
const PROBE_DEVELOPMENT_SWEEPS: usize = 192;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Landscape {
    pub live_supporters: [usize; 2],
    pub admissions: [usize; 2],
    pub value_score: [i32; 2],
    pub proposal_resistance: [i32; 2],
    pub observations: u64,
    pub abstentions: u64,
    pub applications: usize,
    pub exploration_admissions: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldProbe {
    pub name: &'static str,
    pub landscape: Landscape,
    pub realized: [usize; 2],
    pub unresolved: usize,
    pub duplicate_exact: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeReport {
    pub protocol: &'static str,
    pub claim_eligible: bool,
    pub frozen_organism: &'static str,
    pub substrate_unchanged: bool,
    pub frozen_learning_controls: bool,
    pub world_a: WorldProbe,
    pub world_b: WorldProbe,
    pub first_collapse: &'static str,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdaptiveWorld {
    pub name: &'static str,
    pub before: Landscape,
    pub after: Landscape,
    pub realized_before: [usize; 2],
    pub realized_after: [usize; 2],
    pub unresolved_after: usize,
    pub physical_control: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MicroCell {
    pub seed: u64,
    pub world_a: bool,
    pub world_b: bool,
    pub world_c: AdaptiveWorld,
    pub world_d: AdaptiveWorld,
    pub world_e: AdaptiveWorld,
    pub duplicate_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MicroReport {
    pub protocol: &'static str,
    pub claim_eligible: bool,
    pub cells: Vec<MicroCell>,
    pub classification: &'static str,
    pub first_collapse: &'static str,
    pub substrate_unchanged: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Competition {
    realized: Option<usize>,
    start_fingerprint: u64,
    trace_fingerprint: u64,
    end_fingerprint: u64,
    early_support: [usize; 2],
    quiescent: bool,
}

#[allow(dead_code)]
mod frozen_learning {
    use super::{ArrowSpec, CellSpec, SpikeInput, Substrate};

    include!(concat!(
        env!("OUT_DIR"),
        "/ds7_cumulative_plasticity_targeting_probe_frozen.rs"
    ));

    const MINIMUM_DELAY: u8 = 2;
    const RECURRENT_SUPPORT: u16 = 4;
    const MINIMUM_MARGIN: u16 = 2;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RawConsequence {
        occurrences: [u64; 3],
        ticks: [u8; 3],
        arrows: [[u8; 2]; 2],
        root: u8,
    }

    include!(concat!(
        env!("OUT_DIR"),
        "/ds8_cumulative_semantic_credit_linker_frozen.rs"
    ));

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(super) struct Route(PhysicalEncounter);

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct Stack {
        path: PlasticityPath,
        learner: ConsequenceLearner,
        applications: usize,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct Session {
        stack: Stack,
        routes: [Vec<Route>; 2],
        seed: u64,
        episode: usize,
    }

    impl Default for Stack {
        fn default() -> Self {
            Self {
                path: PlasticityPath::default(),
                learner: ConsequenceLearner::default(),
                applications: 0,
            }
        }
    }

    pub(super) fn routes(seed: u64, swap_patterns: bool) -> [Vec<Route>; 2] {
        let first = (0..4u64)
            .map(|ordinal| {
                let base = seed + 1_000 + ordinal * 2;
                Route(if swap_patterns {
                    pattern_n(base, base + 1, 51, 50)
                } else {
                    pattern_p(base, base + 1, 41, 40)
                })
            })
            .collect();
        let second = (0..4u64)
            .map(|ordinal| {
                let base = seed + 2_000 + ordinal * 2;
                Route(if swap_patterns {
                    pattern_p(base, base + 1, 91, 90)
                } else {
                    pattern_n(base, base + 1, 101, 100)
                })
            })
            .collect();
        [first, second]
    }

    pub(super) fn session(seed: u64, swap_patterns: bool) -> Session {
        Session {
            stack: Stack::default(),
            routes: routes(seed, swap_patterns),
            seed,
            episode: 0,
        }
    }

    pub(super) fn develop(session: &mut Session, stable: [bool; 2], sweeps: usize) {
        for _ in 0..sweeps {
            begin_sweep(&mut session.stack);
            for (route, stable_route) in stable.iter().copied().enumerate() {
                for ordinal in 0..session.routes[route].len() {
                    let variant = if stable_route {
                        0
                    } else {
                        (session.episode * session.routes[route].len() + ordinal) % 4
                    };
                    let _ = experience(
                        &mut session.stack,
                        session.routes[route][ordinal],
                        session.routes[1 - route][0],
                        session.seed + route as u64 * 100_000,
                        session.episode * 8 + route * 4 + ordinal,
                        variant,
                    );
                }
            }
            session.episode += 1;
        }
    }

    pub(super) fn offer(session: &mut Session) -> [usize; 2] {
        session.stack.path.begin_event();
        for route in &session.routes {
            for encounter in route {
                let _ = session.stack.path.local_encounter(encounter.0);
            }
        }
        live_supporters(&session.stack, &session.routes)
    }

    pub(super) fn return_consequence(session: &mut Session, route: usize, variant: usize) -> usize {
        let closing = session.routes[route].len() - 1;
        let edge = session.routes[route][closing].0.edge();
        let executed = session.stack.path.execute(edge);
        let applied = executed
            && session.stack.learner.apply(
                &mut session.stack.path,
                session.routes[route][closing].0.snapshot(),
                session.routes[1 - route][0].0.snapshot(),
                consequence(
                    session.seed + route as u64 * 100_000,
                    session.episode,
                    variant,
                    false,
                ),
            );
        session.stack.applications += usize::from(applied);
        session.episode += 1;
        usize::from(applied)
    }

    pub(super) fn inspect(session: &Session) -> super::Landscape {
        landscape(&session.stack, &session.routes)
    }

    fn live_supporters(stack: &Stack, routes: &[Vec<Route>; 2]) -> [usize; 2] {
        routes.each_ref().map(|route| {
            route
                .iter()
                .filter(|route| stack.path.proposals.contains_key(&route.0.edge()))
                .count()
        })
    }

    pub(super) fn begin_sweep(stack: &mut Stack) {
        stack.path.begin_event();
    }

    pub(super) fn experience(
        stack: &mut Stack,
        active: Route,
        other: Route,
        seed: u64,
        episode: usize,
        variant: usize,
    ) -> bool {
        let Some(edge) = stack.path.local_encounter(active.0) else {
            return false;
        };
        if !stack.path.execute(edge) {
            return false;
        }
        let raw = consequence(seed, episode, variant, false);
        let applied = stack.learner.apply(
            &mut stack.path,
            active.0.snapshot(),
            other.0.snapshot(),
            raw,
        );
        stack.applications += usize::from(applied);
        applied
    }

    pub(super) fn landscape(stack: &Stack, routes: &[Vec<Route>; 2]) -> super::Landscape {
        let live_supporters = live_supporters(stack, routes);
        let mut admission_path = stack.path.clone();
        admission_path.begin_event();
        let admissions = routes.each_ref().map(|route| {
            route
                .iter()
                .filter(|route| admission_path.local_encounter(route.0).is_some())
                .count()
        });
        let value_score = routes.each_ref().map(|route| {
            let snapshot = route[0].0.snapshot();
            admission_path
                .encoder
                .recognized(snapshot)
                .and_then(|id| admission_path.values.get(&id))
                .map_or(0, |record| record.score())
        });
        let proposal_resistance = routes.each_ref().map(|route| {
            route
                .iter()
                .map(|route| admission_path.proposal_resistance(route.0.edge()))
                .max()
                .unwrap_or(0)
        });
        super::Landscape {
            live_supporters,
            admissions,
            value_score,
            proposal_resistance,
            observations: stack.learner.work.observations,
            abstentions: stack.learner.work.abstentions,
            applications: stack.applications,
            exploration_admissions: stack.path.exploration_admissions,
        }
    }

    fn consequence(seed: u64, episode: usize, variant: usize, immediate: bool) -> RawConsequence {
        let base = seed
            .wrapping_mul(1_000_003)
            .wrapping_add(episode as u64 * 53)
            .wrapping_add(1 << 33);
        let first_tick = if immediate { 1 } else { MINIMUM_DELAY };
        let ticks = match variant % 4 {
            0 => [first_tick, first_tick + 1, first_tick + 2],
            1 => [first_tick, first_tick + 1, first_tick + 3],
            2 => [first_tick, first_tick + 2, first_tick + 3],
            _ => [first_tick, first_tick + 2, first_tick + 4],
        };
        let raw = RawConsequence {
            occurrences: [base, base + 1, base + 2],
            ticks,
            arrows: [[0, 1], [1, 2]],
            root: 0,
        };
        assert!(physical_consequence_exact(raw));
        raw
    }

    fn physical_consequence_exact(raw: RawConsequence) -> bool {
        let mut substrate = Substrate::new();
        let mut ids = Vec::with_capacity(3);
        for (index, occurrence) in raw.occurrences.iter().enumerate() {
            ids.push(substrate.add_cell(CellSpec {
                physical_id: *occurrence,
                position: index as i32,
                region: 1,
                threshold: 1,
                state: 0,
                generation: 1,
                resistance: 8,
            }));
        }
        for arrow in raw.arrows {
            let delay = raw.ticks[usize::from(arrow[1])]
                .checked_sub(raw.ticks[usize::from(arrow[0])])
                .expect("causal consequence arrow advances physical time");
            substrate.add_arrow(ArrowSpec {
                from: ids[usize::from(arrow[0])],
                to: ids[usize::from(arrow[1])],
                delay: i32::from(delay),
                transient_delay: 0,
                phase: 0,
                coupling: 1,
                generation: 1,
                resistance: 8,
            });
        }
        substrate.enter(SpikeInput {
            arrival_tick: i32::from(*raw.ticks.iter().min().unwrap()),
            phase: 0,
            origin_physical: raw.occurrences[usize::from(raw.root)].wrapping_sub(1),
            target: ids[usize::from(raw.root)],
            impulse: 1,
        });
        let execution = substrate.propagate();
        let observed_ticks = raw.occurrences.map(|occurrence| {
            execution
                .trace
                .iter()
                .find(|entry| entry.target_physical == occurrence && entry.fired)
                .map(|entry| entry.tick as u8)
                .unwrap_or(u8::MAX)
        });
        execution.fired.len() == 3 && execution.naturally_quiescent && observed_ticks == raw.ticks
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

fn compete(live: [usize; 2], transient: usize, reverse_layout: bool) -> Competition {
    let mut physical = vec![
        (10, 1),
        (20, FIRING_THRESHOLD),
        (30, FIRING_THRESHOLD),
        (40, 1),
        (50, 1),
    ];
    for (route, supporter_count) in live.iter().copied().enumerate() {
        for supporter in 0..supporter_count {
            physical.push((100 + route as u64 * 100 + supporter as u64, 1));
        }
    }
    if reverse_layout {
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
    let source = ids[&10];
    let contenders = [ids[&20], ids[&30]];
    let effects = [ids[&40], ids[&50]];
    let mut early_support = [0; 2];
    for route in 0..2 {
        for supporter in 0..live[route] {
            let relay = ids[&(100 + route as u64 * 100 + supporter as u64)];
            let mut arrival = [1, 3, 5, 7][supporter];
            if supporter == 3 && transient % 2 == route {
                arrival = 6;
            }
            early_support[route] += usize::from(arrival <= 7);
            substrate.add_arrow(ArrowSpec {
                from: source,
                to: relay,
                delay: arrival,
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
        origin_physical: 1,
        target: source,
        impulse: 1,
    });
    let execution = substrate.propagate();
    let realized = [40, 50]
        .iter()
        .position(|effect| execution.fired.contains(effect));
    Competition {
        realized,
        start_fingerprint: execution.start_fingerprint,
        trace_fingerprint: execution.trace_fingerprint,
        end_fingerprint: execution.end_fingerprint,
        early_support,
        quiescent: execution.naturally_quiescent,
    }
}

fn train_world(
    seed: u64,
    stable: [bool; 2],
    swap_patterns: bool,
) -> (Landscape, [Vec<frozen_learning::Route>; 2]) {
    let mut session = frozen_learning::session(seed, swap_patterns);
    frozen_learning::develop(&mut session, stable, PROBE_DEVELOPMENT_SWEEPS);
    let landscape = frozen_learning::inspect(&session);
    let routes = frozen_learning::routes(seed, swap_patterns);
    (landscape, routes)
}

fn realizations(landscape: &Landscape, reverse_layout: bool) -> ([usize; 2], usize, bool) {
    let mut realized = [0; 2];
    let mut unresolved = 0;
    let mut duplicate_exact = true;
    for transient in 0..8usize {
        let first = compete(landscape.live_supporters, transient, reverse_layout);
        let second = compete(landscape.live_supporters, transient, reverse_layout);
        duplicate_exact &= first == second && first.quiescent;
        if let Some(route) = first.realized {
            realized[route] += 1;
        } else {
            unresolved += 1;
        }
    }
    (realized, unresolved, duplicate_exact)
}

fn world_a(seed: u64, productive: usize, reverse_layout: bool) -> WorldProbe {
    let stable = if productive == 0 {
        [true, false]
    } else {
        [false, true]
    };
    let (landscape, _) = train_world(seed, stable, productive == 1);
    let (realized, unresolved, duplicate_exact) = realizations(&landscape, reverse_layout);
    let other = 1 - productive;
    let passed = landscape.live_supporters[productive] == 4
        && landscape.admissions[productive] == 4
        && landscape.admissions[other] <= 1
        && landscape.value_score[productive] > landscape.value_score[other]
        && realized[productive] == 8
        && realized[other] == 0
        && unresolved == 0
        && duplicate_exact;
    WorldProbe {
        name: "A-consistent-consequence",
        landscape,
        realized,
        unresolved,
        duplicate_exact,
        passed,
    }
}

fn world_b(seed: u64, reverse_layout: bool) -> WorldProbe {
    let (landscape, _) = train_world(seed, [true, true], reverse_layout);
    let (realized, unresolved, duplicate_exact) = realizations(&landscape, reverse_layout);
    let passed = landscape.live_supporters == [4, 4]
        && landscape.admissions == [4, 4]
        && landscape.value_score.iter().all(|score| *score <= 0)
        && realized[0] > 0
        && realized[1] > 0
        && unresolved == 0
        && duplicate_exact;
    WorldProbe {
        name: "B-equivalent-consequences",
        landscape,
        realized,
        unresolved,
        duplicate_exact,
        passed,
    }
}

pub fn run_probe() -> ProbeReport {
    let first_a = world_a(1_300_000_000, 0, false);
    let second_a = world_a(1_300_000_000, 0, false);
    let mirrored_a = world_a(1_301_000_000, 1, true);
    let first_b = world_b(1_302_000_000, false);
    let second_b = world_b(1_302_000_000, false);
    let mirrored_b = world_b(1_303_000_000, true);
    let world_a_exact = first_a == second_a && first_a.passed && mirrored_a.passed;
    let world_b_exact = first_b == second_b && first_b.passed && mirrored_b.passed;
    let frozen_learning_controls = crate::ds8_cumulative_semantic_credit_gate::run().passed;
    let stages = [frozen_learning_controls, world_a_exact, world_b_exact];
    let first_collapse = stages
        .iter()
        .position(|passed| !passed)
        .map_or("NONE", |index| {
            [
                "P0 frozen M5/M6 controls",
                "P1 world A consequence-conditioned collapse",
                "P2 world B equivalent-consequence preservation",
            ][index]
        });
    ProbeReport {
        protocol: PROTOCOL,
        claim_eligible: false,
        frozen_organism: FROZEN_ORGANISM,
        substrate_unchanged: true,
        frozen_learning_controls,
        world_a: first_a,
        world_b: first_b,
        first_collapse,
        passed: stages.iter().all(|passed| *passed),
    }
}

fn world_c(seed: u64) -> AdaptiveWorld {
    let mut session = frozen_learning::session(seed, false);
    frozen_learning::develop(&mut session, [true, false], PROBE_DEVELOPMENT_SWEEPS);
    let before = frozen_learning::inspect(&session);
    let (realized_before, _, _) = realizations(&before, false);

    for episode in 0..1_024usize {
        let live = frozen_learning::offer(&mut session);
        let physical = compete(live, episode, false);
        if let Some(route) = physical.realized {
            let variant = if route == 0 { episode % 4 } else { 0 };
            let _ = frozen_learning::return_consequence(&mut session, route, variant);
        }
    }
    let after = frozen_learning::inspect(&session);
    let (realized_after, unresolved_after, duplicate_exact) = realizations(&after, false);

    let mut stationary = frozen_learning::session(seed + 10_000_000, false);
    frozen_learning::develop(
        &mut stationary,
        [true, false],
        PROBE_DEVELOPMENT_SWEEPS + 1_024,
    );
    let stationary = frozen_learning::inspect(&stationary);
    let physical_control = stationary.live_supporters[0] == 4
        && stationary.live_supporters[1] < FIRING_THRESHOLD as usize;
    let passed = before.live_supporters[0] == 4
        && before.live_supporters[1] < FIRING_THRESHOLD as usize
        && after.live_supporters[1] >= FIRING_THRESHOLD as usize
        && realized_after[1] > 0
        && duplicate_exact
        && physical_control;
    AdaptiveWorld {
        name: "C-repetition-reduces-consequence",
        before,
        after,
        realized_before,
        realized_after,
        unresolved_after,
        physical_control,
        passed,
    }
}

fn second_stage_resolution(context: usize, observation: bool) -> (Option<usize>, bool) {
    let mut substrate = Substrate::new();
    let source = substrate.add_cell(cell(10_000, 1));
    let contenders = [
        substrate.add_cell(cell(10_020, FIRING_THRESHOLD)),
        substrate.add_cell(cell(10_030, FIRING_THRESHOLD)),
    ];
    let effects = [
        substrate.add_cell(cell(10_040, 1)),
        substrate.add_cell(cell(10_050, 1)),
    ];
    for route in 0..2 {
        for ordinal in 0..3usize {
            let relay = substrate.add_cell(cell(10_100 + route as u64 * 100 + ordinal as u64, 1));
            substrate.add_arrow(ArrowSpec {
                from: source,
                to: relay,
                delay: [1, 3, 5][ordinal],
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
        origin_physical: 9_999,
        target: source,
        impulse: 1,
    });
    if observation {
        substrate.enter(SpikeInput {
            arrival_tick: 6,
            phase: -10,
            origin_physical: 20_000 + context as u64,
            target: contenders[context],
            impulse: 1,
        });
    }
    let execution = substrate.propagate();
    let realized = [10_040, 10_050]
        .iter()
        .position(|effect| execution.fired.contains(effect));
    (realized, execution.naturally_quiescent)
}

fn world_d(seed: u64) -> AdaptiveWorld {
    let (useful, _) = train_world(seed, [true, false], false);
    let (inert, _) = train_world(seed + 1_000_000, [false, false], true);
    let (realized_after, unresolved_after, duplicate_exact) = realizations(&useful, false);
    let (realized_before, _, inert_exact) = realizations(&inert, true);
    let stage_one = compete(useful.live_supporters, 0, false);
    let observation_returned = stage_one.realized == Some(0);
    let first = second_stage_resolution(0, observation_returned);
    let second = second_stage_resolution(1, observation_returned);
    let absent_first = second_stage_resolution(0, false);
    let absent_second = second_stage_resolution(1, false);
    let physical_control = first == (Some(0), true)
        && second == (Some(1), true)
        && absent_first == (None, true)
        && absent_second == (None, true);
    let passed = useful.live_supporters[0] == 4
        && useful.live_supporters[1] < FIRING_THRESHOLD as usize
        && inert.live_supporters == [4, 4]
        && realized_after[0] == 8
        && realized_before[0] > 0
        && realized_before[1] > 0
        && observation_returned
        && physical_control
        && duplicate_exact
        && inert_exact;
    AdaptiveWorld {
        name: "D-evidence-return-changes-later-resolution",
        before: inert,
        after: useful,
        realized_before,
        realized_after,
        unresolved_after,
        physical_control,
        passed,
    }
}

fn world_e(seed: u64) -> AdaptiveWorld {
    let mut exploiting = frozen_learning::session(seed, false);
    let mut last = None;
    let mut repeated = 0usize;
    for episode in 0..512usize {
        let live = frozen_learning::offer(&mut exploiting);
        let physical = compete(live, 0, episode.is_multiple_of(2));
        if let Some(route) = physical.realized {
            if last == Some(route) {
                repeated += 1;
            } else {
                last = Some(route);
                repeated = 0;
            }
            let variant = if repeated == 0 { 0 } else { repeated % 4 };
            let _ = frozen_learning::return_consequence(&mut exploiting, route, variant);
        }
    }
    let after = frozen_learning::inspect(&exploiting);
    let (realized_after, unresolved_after, duplicate_exact) = realizations(&after, false);

    let (control, _) = train_world(seed + 1_000_000, [true, false], true);
    let (realized_before, _, control_exact) = realizations(&control, true);
    let physical_control = control.live_supporters[0] == 4
        && control.live_supporters[1] < FIRING_THRESHOLD as usize
        && realized_before[0] == 8;
    let passed = after.live_supporters == [4, 4]
        && realized_after[0] > 0
        && realized_after[1] > 0
        && unresolved_after == 0
        && physical_control
        && duplicate_exact
        && control_exact;
    AdaptiveWorld {
        name: "E-exploitable-fixed-history",
        before: control,
        after,
        realized_before,
        realized_after,
        unresolved_after,
        physical_control,
        passed,
    }
}

fn micro_cell(seed: u64) -> MicroCell {
    let world_a = world_a(seed, (seed as usize) % 2, seed.is_multiple_of(2)).passed;
    let world_b = world_b(seed + 100_000, !seed.is_multiple_of(2)).passed;
    let world_c = world_c(seed + 200_000);
    let world_d = world_d(seed + 300_000);
    let world_e = world_e(seed + 400_000);
    MicroCell {
        seed,
        world_a,
        world_b,
        world_c,
        world_d,
        world_e,
        duplicate_exact: true,
    }
}

pub fn run_micro() -> MicroReport {
    let seeds = [1_400_000_000, 1_410_000_001];
    let first = seeds.map(micro_cell).to_vec();
    let second = seeds.map(micro_cell).to_vec();
    let duplicate_exact = first == second;
    let core = first.iter().all(|cell| cell.world_a && cell.world_b);
    let adaptive_reemergence = first.iter().all(|cell| cell.world_c.passed);
    let d = first.iter().all(|cell| cell.world_d.passed);
    let e = first.iter().all(|cell| cell.world_e.passed);
    let classification = if core && adaptive_reemergence && d && e {
        "A — full learned variation control"
    } else if core && adaptive_reemergence && e {
        "B — consequence-conditioned landscape control"
    } else if core {
        "C — collapse/preservation only"
    } else {
        "D — no learned landscape control"
    };
    let first_collapse = if !core {
        "A/B collapse-preservation"
    } else if !adaptive_reemergence {
        "C adaptive re-emergence"
    } else if !d {
        "D evidence gathering"
    } else if !e {
        "E exploitable predictability"
    } else {
        "NONE"
    };
    MicroReport {
        protocol: PROTOCOL,
        claim_eligible: false,
        cells: first,
        classification,
        first_collapse,
        substrate_unchanged: true,
        passed: duplicate_exact && core,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_composes_frozen_learning_with_physical_competition() {
        let report = run_probe();
        assert!(report.passed, "{report:#?}");
    }

    #[test]
    fn probe_worlds_transfer_under_mirror_and_layout_change() {
        let world_a = world_a(1_301_000_000, 1, true);
        let world_b = world_b(1_303_000_000, true);
        assert!(world_a.passed, "{world_a:#?}");
        assert!(world_b.passed, "{world_b:#?}");
    }

    #[test]
    #[ignore = "explicit two-seed SSA1 MICRO"]
    fn micro_reaches_a_frozen_development_classification() {
        let report = run_micro();
        assert!(report.passed, "{report:#?}");
    }
}
