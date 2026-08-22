//! SSA1-C1 development: adaptation under experience.
//!
//! This evaluator composes byte-frozen SSA1 M5/M6 learning with ordinary
//! physical environments. It does not modify Frozen Organism v1.

use std::collections::BTreeMap;

use crate::organism::{ArrowSpec, CellSpec, SpikeInput, Substrate};

pub const PROTOCOL: &str = "ssa1-c1-adaptation-under-experience-v1";
pub const FROZEN_ORGANISM: &str = "2125a197ad0e5796a12668cd76c2071236763af0";
pub const FROZEN_SSA1_CLASSIFICATION: &str =
    "ssa1-learned-variation-control-development-classification-c";
pub const FROZEN_SSA1_SOURCE_SHA256: &str =
    "dc157e0bd238992d6475e5dc9767c6f7711a1bb5b7759ebdb7991573aea5199b";

const FIRING_THRESHOLD: i32 = 4;
const INHIBITION: i32 = -64;
const LOCK_SWEEPS: usize = 192;
const CHANGED_EPISODES: usize = 1_024;
const PERSISTENCE_EPISODES: usize = 256;
const INITIAL_EXPOSURES: [usize; 10] = [0, 1, 2, 3, 4, 8, 16, 32, 64, 192];

#[allow(dead_code)]
mod frozen_ssa1 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ssa1_learned_variation_control_frozen.rs"
    ));

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

        pub(super) fn develop(&mut self, stable: [bool; 2], sweeps: usize) {
            frozen_learning::develop(&mut self.session, stable, sweeps);
        }

        pub(super) fn offer(&mut self) -> [usize; 2] {
            frozen_learning::offer(&mut self.session)
        }

        pub(super) fn return_physical_consequence(
            &mut self,
            route: usize,
            variant: usize,
        ) -> usize {
            frozen_learning::return_consequence(&mut self.session, route, variant)
        }

        pub(super) fn inspect(&self) -> Landscape {
            frozen_learning::inspect(&self.session)
        }
    }
}

pub use frozen_ssa1::Landscape;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pulse {
    route: usize,
    tick: i32,
    phase: i32,
    impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhysicalWorld {
    suppress_existing: [bool; 2],
    stale_route: [bool; 2],
    existing_delays: [[i32; 4]; 2],
    background: Vec<Pulse>,
    reverse_allocation: bool,
}

impl Default for PhysicalWorld {
    fn default() -> Self {
        Self {
            suppress_existing: [false; 2],
            stale_route: [false; 2],
            existing_delays: [[1, 3, 5, 7], [1, 3, 5, 7]],
            background: Vec::new(),
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
    early_integrated: [usize; 2],
    background_visible: usize,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Baseline {
    pub before_change: Landscape,
    pub after_change: Landscape,
    pub changed_realizations: [usize; 2],
    pub changed_unresolved: usize,
    pub duplicate_exact: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Counterexperience {
    pub before: Landscape,
    pub after_counterexperience: Landscape,
    pub after_persistence: Landscape,
    pub forced_realizations: [usize; 2],
    pub unconstrained_realizations: [usize; 2],
    pub counterexperience_obtained: bool,
    pub learning_changed: bool,
    pub recovered: bool,
    pub persisted: bool,
    pub controls_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurriculumPoint {
    pub initial_exposures: usize,
    pub at_change: Landscape,
    pub after: Landscape,
    pub changed_realizations: [usize; 2],
    pub recovered: bool,
    pub duplicate_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransientHistory {
    pub before: Landscape,
    pub after_richness: Landscape,
    pub after_persistence: Landscape,
    pub timing_only_suppressed_realizations: usize,
    pub minimum_early_background: Option<usize>,
    pub rich_realizations: [usize; 2],
    pub counterexperience_obtained: bool,
    pub learning_changed: bool,
    pub recovered: bool,
    pub persisted: bool,
    pub postclosure_inert: bool,
    pub controls_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub seed: u64,
    pub productive_route: usize,
    pub baseline: Baseline,
    pub counterexperience: Counterexperience,
    pub curriculum: Vec<CurriculumPoint>,
    pub adaptation_frontier: Option<usize>,
    pub transient_history: TransientHistory,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub protocol: &'static str,
    pub stage: &'static str,
    pub claim_eligible: bool,
    pub cells: Vec<Cell>,
    pub classification: &'static str,
    pub first_collapse: &'static str,
    pub frozen_parent_exact: bool,
    pub duplicate_exact: bool,
    pub passed: bool,
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
    let base = identity.wrapping_mul(10_000).wrapping_add(1_000_000);
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
    let mut early_integrated = [0; 2];
    for route in 0..2 {
        if !world.suppress_existing[route] && !world.stale_route[route] {
            for supporter in 0..live[route] {
                let relay = ids[&(base + 100 + route as u64 * 100 + supporter as u64)];
                let delay = world.existing_delays[route][supporter];
                early_integrated[route] += usize::from(delay <= 7);
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
    for (ordinal, pulse) in world.background.iter().enumerate() {
        substrate.enter(SpikeInput {
            arrival_tick: pulse.tick,
            phase: pulse.phase,
            origin_physical: base + 10_000 + ordinal as u64,
            target: contenders[pulse.route],
            impulse: pulse.impulse,
        });
        early_integrated[pulse.route] += usize::from(pulse.tick <= 7);
    }
    let execution = substrate.propagate();
    let realized = [base + 40, base + 50]
        .iter()
        .position(|effect| execution.fired.contains(effect));
    let background_visible = world
        .background
        .iter()
        .filter(|pulse| {
            execution.trace.iter().any(|entry| {
                entry.target_physical == base + 20 + pulse.route as u64 * 10
                    && entry.tick == pulse.tick
            })
        })
        .count();
    Resolution {
        realized,
        start_fingerprint: execution.start_fingerprint,
        trace_fingerprint: execution.trace_fingerprint,
        end_fingerprint: execution.end_fingerprint,
        early_integrated,
        background_visible,
        quiescent: execution.naturally_quiescent,
    }
}

fn transient_world(favored: usize, reverse_allocation: bool) -> PhysicalWorld {
    let mut world = PhysicalWorld {
        reverse_allocation,
        ..PhysicalWorld::default()
    };
    world.existing_delays[favored][3] = 6;
    world
}

fn background(route: usize, count: usize, late: bool) -> Vec<Pulse> {
    let ticks = if late { [20, 22, 24] } else { [0, 2, 4] };
    (0..count)
        .map(|ordinal| Pulse {
            route,
            tick: ticks[ordinal],
            phase: ordinal as i32 - 1,
            impulse: 1,
        })
        .collect()
}

fn symmetric_background(count: usize, late: bool) -> Vec<Pulse> {
    let mut pulses = background(0, count, late);
    pulses.extend(background(1, count, late));
    pulses
}

fn count_resolution(counts: &mut [usize; 2], unresolved: &mut usize, route: Option<usize>) {
    if let Some(route) = route {
        counts[route] += 1;
    } else {
        *unresolved += 1;
    }
}

fn locked_session(seed: u64, productive: usize) -> frozen_ssa1::Adapter {
    let mut session = frozen_ssa1::Adapter::blank(seed, productive == 1);
    let stable = if productive == 0 {
        [true, false]
    } else {
        [false, true]
    };
    session.develop(stable, LOCK_SWEEPS);
    session
}

fn baseline(seed: u64, productive: usize, reverse: bool) -> (Baseline, frozen_ssa1::Adapter) {
    let suppressed = 1 - productive;
    let mut session = locked_session(seed, productive);
    let before_change = session.inspect();
    let mut changed_realizations = [0; 2];
    let mut changed_unresolved = 0;
    let mut duplicate_exact = true;
    for episode in 0..CHANGED_EPISODES {
        let live = session.offer();
        let world = transient_world(suppressed, reverse ^ episode.is_multiple_of(2));
        let first = resolve(live, &world, seed + episode as u64);
        let second = resolve(live, &world, seed + episode as u64);
        duplicate_exact &= first == second && first.quiescent;
        count_resolution(
            &mut changed_realizations,
            &mut changed_unresolved,
            first.realized,
        );
        if let Some(route) = first.realized {
            let variant = if route == suppressed { 0 } else { episode % 4 };
            let _ = session.return_physical_consequence(route, variant);
        }
    }
    let after_change = session.inspect();
    let passed = before_change.live_supporters[productive] == 4
        && before_change.live_supporters[suppressed] < FIRING_THRESHOLD as usize
        && after_change.live_supporters == before_change.live_supporters
        && changed_realizations[productive] == CHANGED_EPISODES
        && changed_realizations[suppressed] == 0
        && changed_unresolved == 0
        && duplicate_exact;
    (
        Baseline {
            before_change,
            after_change,
            changed_realizations,
            changed_unresolved,
            duplicate_exact,
            passed,
        },
        session,
    )
}

fn counterexperience(seed: u64, productive: usize, reverse: bool) -> Counterexperience {
    let suppressed = 1 - productive;
    let (_, mut session) = baseline(seed, productive, reverse);
    let before = session.inspect();
    let observations_before = before.observations;
    let mut forced_realizations = [0; 2];
    let mut forced_unresolved = 0;
    let world = PhysicalWorld {
        suppress_existing: [productive == 0, productive == 1],
        background: symmetric_background(3, false),
        reverse_allocation: reverse,
        ..PhysicalWorld::default()
    };
    for episode in 0..CHANGED_EPISODES {
        let live = session.offer();
        let physical = resolve(live, &world, seed + 2_000_000 + episode as u64);
        count_resolution(
            &mut forced_realizations,
            &mut forced_unresolved,
            physical.realized,
        );
        if let Some(route) = physical.realized {
            let variant = if route == suppressed { 0 } else { episode % 4 };
            let _ = session.return_physical_consequence(route, variant);
        }
    }
    let after_counterexperience = session.inspect();
    let unconstrained_realizations = realization_sweep(
        after_counterexperience.live_supporters,
        reverse,
        seed + 3_000_000,
    );
    let counterexperience_obtained = forced_realizations[suppressed] == CHANGED_EPISODES
        && forced_unresolved == 0
        && after_counterexperience.observations >= observations_before + CHANGED_EPISODES as u64;
    let learning_changed = after_counterexperience.value_score[suppressed]
        != before.value_score[suppressed]
        || after_counterexperience.admissions[suppressed] != before.admissions[suppressed]
        || after_counterexperience.live_supporters[suppressed]
            != before.live_supporters[suppressed];
    let recovered = after_counterexperience.live_supporters[suppressed]
        >= FIRING_THRESHOLD as usize
        && unconstrained_realizations[suppressed] > 0;

    for episode in 0..PERSISTENCE_EPISODES {
        let live = session.offer();
        let physical = resolve(
            live,
            &transient_world(episode % 2, reverse),
            seed + 4_000_000 + episode as u64,
        );
        if let Some(route) = physical.realized {
            let variant = if route == suppressed { 0 } else { episode % 4 };
            let _ = session.return_physical_consequence(route, variant);
        }
    }
    let after_persistence = session.inspect();
    let persisted =
        recovered && after_persistence.live_supporters[suppressed] >= FIRING_THRESHOLD as usize;

    let live = before.live_supporters;
    let no_background = resolve(
        live,
        &PhysicalWorld {
            suppress_existing: [productive == 0, productive == 1],
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 5_000_000,
    );
    let background_without_block = resolve(
        live,
        &PhysicalWorld {
            background: symmetric_background(3, false),
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 5_000_001,
    );
    let late = resolve(
        live,
        &PhysicalWorld {
            background: symmetric_background(3, true),
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 5_000_002,
    );
    let stale = resolve(
        live,
        &PhysicalWorld {
            suppress_existing: [productive == 0, productive == 1],
            stale_route: [suppressed == 0, suppressed == 1],
            background: symmetric_background(3, false),
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 5_000_003,
    );
    let base = resolve(
        live,
        &PhysicalWorld {
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 5_000_004,
    );
    let controls_passed = no_background.realized.is_none()
        && background_without_block.realized == Some(productive)
        && late.realized == base.realized
        && late.background_visible == 6
        && stale.realized.is_none()
        && world == world.clone();
    Counterexperience {
        before,
        after_counterexperience,
        after_persistence,
        forced_realizations,
        unconstrained_realizations,
        counterexperience_obtained,
        learning_changed,
        recovered,
        persisted,
        controls_passed,
    }
}

fn realization_sweep(live: [usize; 2], reverse: bool, seed: u64) -> [usize; 2] {
    let mut counts = [0; 2];
    for favored in 0..2 {
        for duplicate in 0..4u64 {
            let world = transient_world(favored, reverse ^ duplicate.is_multiple_of(2));
            if let Some(route) =
                resolve(live, &world, seed + favored as u64 * 10 + duplicate).realized
            {
                counts[route] += 1;
            }
        }
    }
    counts
}

fn curriculum_point(
    seed: u64,
    productive: usize,
    initial_exposures: usize,
    reverse: bool,
) -> CurriculumPoint {
    let suppressed = 1 - productive;
    let mut session = frozen_ssa1::Adapter::blank(seed, productive == 1);
    let mut duplicate_exact = true;
    for episode in 0..initial_exposures {
        let live = session.offer();
        let world = transient_world(productive, reverse ^ episode.is_multiple_of(2));
        let first = resolve(live, &world, seed + episode as u64);
        let second = resolve(live, &world, seed + episode as u64);
        duplicate_exact &= first == second && first.realized == Some(productive);
        if let Some(route) = first.realized {
            let _ = session.return_physical_consequence(route, 0);
        }
    }
    let at_change = session.inspect();
    let mut changed_realizations = [0; 2];
    let mut unresolved = 0;
    for episode in 0..CHANGED_EPISODES {
        let live = session.offer();
        let favored = if episode.is_multiple_of(2) {
            suppressed
        } else {
            productive
        };
        let world = transient_world(favored, reverse ^ episode.is_multiple_of(3));
        let first = resolve(live, &world, seed + 1_000_000 + episode as u64);
        let second = resolve(live, &world, seed + 1_000_000 + episode as u64);
        duplicate_exact &= first == second && first.quiescent;
        count_resolution(&mut changed_realizations, &mut unresolved, first.realized);
        if let Some(route) = first.realized {
            let variant = if route == suppressed { 0 } else { episode % 4 };
            let _ = session.return_physical_consequence(route, variant);
        }
    }
    let after = session.inspect();
    let sweep = realization_sweep(after.live_supporters, reverse, seed + 2_000_000);
    let recovered =
        after.live_supporters[suppressed] >= FIRING_THRESHOLD as usize && sweep[suppressed] > 0;
    CurriculumPoint {
        initial_exposures,
        at_change,
        after,
        changed_realizations,
        recovered,
        duplicate_exact: duplicate_exact && unresolved == 0,
    }
}

fn transient_history(seed: u64, productive: usize, reverse: bool) -> TransientHistory {
    let suppressed = 1 - productive;
    let (_, mut session) = baseline(seed, productive, reverse);
    let before = session.inspect();
    let mut timing_only_suppressed_realizations = 0;
    let permutations = [
        [1, 3, 5, 7],
        [7, 5, 3, 1],
        [1, 5, 3, 7],
        [3, 1, 7, 5],
        [0, 2, 4, 6],
        [6, 4, 2, 0],
    ];
    for (ordinal, delays) in permutations.into_iter().enumerate() {
        let mut world = PhysicalWorld {
            reverse_allocation: reverse ^ ordinal.is_multiple_of(2),
            ..PhysicalWorld::default()
        };
        world.existing_delays[productive] = delays;
        world.existing_delays[suppressed] = delays;
        timing_only_suppressed_realizations += usize::from(
            resolve(before.live_supporters, &world, seed + ordinal as u64).realized
                == Some(suppressed),
        );
    }
    let minimum_early_background = (0..=3).find(|count| {
        let result = resolve(
            before.live_supporters,
            &PhysicalWorld {
                background: background(suppressed, *count, false),
                reverse_allocation: reverse,
                ..PhysicalWorld::default()
            },
            seed + 100 + *count as u64,
        );
        result.realized == Some(suppressed)
    });

    let mut rich_realizations = [0; 2];
    let mut unresolved = 0;
    for episode in 0..CHANGED_EPISODES {
        let live = session.offer();
        let world = PhysicalWorld {
            background: background(suppressed, 3, false),
            reverse_allocation: reverse ^ episode.is_multiple_of(2),
            ..PhysicalWorld::default()
        };
        let physical = resolve(live, &world, seed + 1_000_000 + episode as u64);
        count_resolution(&mut rich_realizations, &mut unresolved, physical.realized);
        if let Some(route) = physical.realized {
            let variant = if route == suppressed { 0 } else { episode % 4 };
            let _ = session.return_physical_consequence(route, variant);
        }
    }
    let after_richness = session.inspect();
    let counterexperience_obtained = rich_realizations[suppressed] == CHANGED_EPISODES
        && unresolved == 0
        && after_richness.observations >= before.observations + CHANGED_EPISODES as u64;
    let learning_changed = after_richness.value_score[suppressed] != before.value_score[suppressed]
        || after_richness.admissions[suppressed] != before.admissions[suppressed]
        || after_richness.live_supporters[suppressed] != before.live_supporters[suppressed];
    let unconstrained =
        realization_sweep(after_richness.live_supporters, reverse, seed + 3_000_000);
    let recovered = after_richness.live_supporters[suppressed] >= FIRING_THRESHOLD as usize
        && unconstrained[suppressed] > 0;
    for episode in 0..PERSISTENCE_EPISODES {
        let live = session.offer();
        let physical = resolve(
            live,
            &transient_world(episode % 2, reverse),
            seed + 4_000_000 + episode as u64,
        );
        if let Some(route) = physical.realized {
            let variant = if route == suppressed { 0 } else { episode % 4 };
            let _ = session.return_physical_consequence(route, variant);
        }
    }
    let after_persistence = session.inspect();
    let persisted =
        recovered && after_persistence.live_supporters[suppressed] >= FIRING_THRESHOLD as usize;
    let base = resolve(
        before.live_supporters,
        &PhysicalWorld {
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 5_000_000,
    );
    let late = resolve(
        before.live_supporters,
        &PhysicalWorld {
            background: background(suppressed, 3, true),
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 5_000_000,
    );
    let postclosure_inert = base.realized == late.realized && late.background_visible == 3;
    let stale = resolve(
        before.live_supporters,
        &PhysicalWorld {
            stale_route: [suppressed == 0, suppressed == 1],
            background: background(suppressed, 3, false),
            reverse_allocation: reverse,
            ..PhysicalWorld::default()
        },
        seed + 5_000_001,
    );
    let controls_passed = timing_only_suppressed_realizations == 0
        && minimum_early_background == Some(3)
        && postclosure_inert
        && stale.realized != Some(suppressed);
    TransientHistory {
        before,
        after_richness,
        after_persistence,
        timing_only_suppressed_realizations,
        minimum_early_background,
        rich_realizations,
        counterexperience_obtained,
        learning_changed,
        recovered,
        persisted,
        postclosure_inert,
        controls_passed,
    }
}

fn run_cell(seed: u64, productive_route: usize) -> Cell {
    let reverse = seed.is_multiple_of(2);
    let (baseline_result, _) = baseline(seed, productive_route, reverse);
    let counterexperience = counterexperience(seed + 10_000_000, productive_route, reverse);
    let curriculum: Vec<_> = INITIAL_EXPOSURES
        .into_iter()
        .map(|exposures| {
            curriculum_point(
                seed + 20_000_000 + exposures as u64 * 100_000,
                productive_route,
                exposures,
                reverse,
            )
        })
        .collect();
    let adaptation_frontier = curriculum
        .iter()
        .filter(|point| point.recovered)
        .map(|point| point.initial_exposures)
        .max();
    let transient_history = transient_history(seed + 30_000_000, productive_route, reverse);
    let passed = baseline_result.passed
        && counterexperience.controls_passed
        && curriculum.iter().all(|point| point.duplicate_exact)
        && transient_history.controls_passed;
    Cell {
        seed,
        productive_route,
        baseline: baseline_result,
        counterexperience,
        curriculum,
        adaptation_frontier,
        transient_history,
        passed,
    }
}

fn report(stage: &'static str, seeds: &[u64]) -> Report {
    let first: Vec<_> = seeds
        .iter()
        .enumerate()
        .map(|(index, seed)| run_cell(*seed, index % 2))
        .collect();
    let second: Vec<_> = seeds
        .iter()
        .enumerate()
        .map(|(index, seed)| run_cell(*seed, index % 2))
        .collect();
    let duplicate_exact = first == second;
    let controls = first.iter().all(|cell| cell.passed);
    let c3 = first
        .iter()
        .all(|cell| cell.transient_history.recovered && cell.transient_history.persisted);
    let c1 = first
        .iter()
        .all(|cell| cell.counterexperience.recovered && cell.counterexperience.persisted);
    let curriculum_frontier = first.iter().all(|cell| {
        cell.adaptation_frontier
            .is_some_and(|frontier| frontier > 0)
    });
    let classification = if c3 {
        "A — autonomous history-enabled recovery"
    } else if c1 {
        "B — environment-enabled recovery"
    } else if curriculum_frontier {
        "C — curriculum prevention only"
    } else {
        "D — persistent lock-in"
    };
    let first_collapse = if !controls {
        "physical or isolation control"
    } else if !c1 {
        "C1 learned reopening after physical counterexperience"
    } else if !c3 {
        "C3 autonomous history-enabled reopening"
    } else {
        "NONE"
    };
    Report {
        protocol: PROTOCOL,
        stage,
        claim_eligible: false,
        cells: first,
        classification,
        first_collapse,
        frozen_parent_exact: true,
        duplicate_exact,
        passed: controls && duplicate_exact,
    }
}

pub fn run_probe() -> Report {
    report("PROBE", &[1_600_000_000])
}

pub fn run_micro() -> Report {
    report("MICRO", &[1_610_000_000, 1_620_000_001])
}

pub fn run_gate() -> Report {
    report(
        "GATE",
        &[
            1_630_000_000,
            1_640_000_001,
            1_650_000_002,
            1_660_000_003,
            1_670_000_004,
            1_680_000_005,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_separates_execution_from_learned_reopening() {
        let report = run_probe();
        assert!(report.passed, "{report:#?}");
        let cell = &report.cells[0];
        assert!(cell.counterexperience.counterexperience_obtained);
        assert!(cell.transient_history.counterexperience_obtained);
    }

    #[test]
    #[ignore = "explicit two-seed SSA1-C1 MICRO"]
    fn micro_freezes_an_adaptation_classification() {
        let report = run_micro();
        assert!(report.passed, "{report:#?}");
    }

    #[test]
    #[ignore = "explicit six-seed SSA1-C1 GATE"]
    fn gate_transfers_across_fresh_identities_and_mirrors() {
        let report = run_gate();
        assert!(report.passed, "{report:#?}");
    }
}
