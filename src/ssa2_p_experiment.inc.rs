
pub const SSA2P_PROTOCOL: &str = "ssa2-p-preserved-affordance-generativity-v2";

const SSA2P_STATE_COUNT: usize = 4;
const SSA2P_STEP_TICKS: i32 = 8;
const SSA2P_INHIBITION: i32 = -64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ssa2pTrajectory {
    pub history: u64,
    pub depth: usize,
    pub states: Vec<usize>,
    pub physical_sides: Vec<usize>,
    pub handles: Vec<usize>,
    pub complete: bool,
    pub structurally_valid: bool,
    pub naturally_quiescent: bool,
    pub duplicate_exact: bool,
    pub one_propagation: bool,
    pub permanent_fingerprint: u64,
    pub start_fingerprint: u64,
    pub trace_fingerprint: u64,
    pub end_fingerprint: u64,
    pub work: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ssa2pCell {
    pub seed: u64,
    pub depth: usize,
    pub histories: usize,
    pub learned_live: [usize; 2],
    pub learned_from_blank: bool,
    pub trajectories: Vec<Ssa2pTrajectory>,
    pub distinct_trajectories: usize,
    pub distinct_trace_fingerprints: usize,
    pub both_sides_every_layer: bool,
    pub permanent_exact: bool,
    pub collapsed_live: [usize; 2],
    pub collapsed_distinct_trajectories: usize,
    pub collapsed_control: bool,
    pub blocked_control: bool,
    pub broken_transition_control: bool,
    pub duplicate_handle_control: bool,
    pub handle_permutation_control: bool,
    pub no_transient_control: bool,
    pub controls_passed: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ssa2pReport {
    pub protocol: &'static str,
    pub stage: &'static str,
    pub depth: usize,
    pub histories: usize,
    pub cells: Vec<Ssa2pCell>,
    pub valid_trajectories: usize,
    pub total_trajectories: usize,
    pub minimum_distinct_trajectories: usize,
    pub frozen_parent_exact: bool,
    pub classification: &'static str,
    pub claim_eligible: bool,
    pub passed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ssa2pStage {
    Probe,
    Micro,
    Gate,
}

impl Ssa2pStage {
    fn name(self) -> &'static str {
        match self {
            Self::Probe => "PROBE",
            Self::Micro => "MICRO",
            Self::Gate => "GATE",
        }
    }

    fn depth(self) -> usize {
        match self {
            Self::Probe => 8,
            Self::Micro => 16,
            Self::Gate => 32,
        }
    }

    fn histories(self) -> usize {
        match self {
            Self::Probe => 8,
            Self::Micro => 32,
            Self::Gate => 64,
        }
    }

    fn seeds(self) -> &'static [u64] {
        match self {
            Self::Probe => &[2_200_000_000, 2_200_000_001],
            Self::Micro => &[
                2_210_000_000,
                2_210_000_001,
                2_210_000_002,
                2_210_000_003,
            ],
            Self::Gate => &[
                2_220_000_000,
                2_220_000_001,
                2_220_000_002,
                2_220_000_003,
                2_220_000_004,
                2_220_000_005,
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Ssa2pRole {
    Source {
        layer: usize,
        state: usize,
    },
    Relay {
        layer: usize,
        state: usize,
        side: usize,
        ordinal: usize,
    },
    Gate {
        layer: usize,
        state: usize,
        side: usize,
    },
    Contender {
        layer: usize,
        state: usize,
        side: usize,
    },
    Effect {
        layer: usize,
        state: usize,
        side: usize,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Ssa2pBuildOptions {
    depth: usize,
    history_bits: usize,
    seed: u64,
    live: [usize; 2],
    reverse_allocation: bool,
    mirror: bool,
    handle_swap: bool,
    blocked_layer_side: Option<(usize, usize)>,
    broken_transition: Option<(usize, usize, usize)>,
}

#[derive(Clone, Debug)]
struct Ssa2pWorld {
    substrate: Substrate,
    ids: std::collections::BTreeMap<Ssa2pRole, crate::organism::CellId>,
    by_physical: std::collections::BTreeMap<u64, Ssa2pRole>,
    options: Ssa2pBuildOptions,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Ssa2pArrow {
    from: Ssa2pRole,
    to: Ssa2pRole,
    delay: i32,
    phase: i32,
    coupling: i32,
}

fn ssa2p_transition(state: usize, side: usize) -> usize {
    const NEXT: [[usize; 2]; SSA2P_STATE_COUNT] = [[1, 2], [3, 0], [0, 3], [2, 1]];
    NEXT[state][side]
}

fn ssa2p_role_code(role: Ssa2pRole) -> u64 {
    match role {
        Ssa2pRole::Source { layer, state } => {
            layer as u64 * 10_000 + state as u64 * 1_000 + 10
        }
        Ssa2pRole::Relay {
            layer,
            state,
            side,
            ordinal,
        } => {
            layer as u64 * 10_000
                + state as u64 * 1_000
                + side as u64 * 100
                + ordinal as u64
                + 100
        }
        Ssa2pRole::Gate {
            layer,
            state,
            side,
        } => layer as u64 * 10_000 + state as u64 * 1_000 + side as u64 * 100 + 180,
        Ssa2pRole::Contender {
            layer,
            state,
            side,
        } => layer as u64 * 10_000 + state as u64 * 1_000 + side as u64 * 100 + 220,
        Ssa2pRole::Effect {
            layer,
            state,
            side,
        } => layer as u64 * 10_000 + state as u64 * 1_000 + side as u64 * 100 + 240,
    }
}

fn ssa2p_cell_spec(seed: u64, role: Ssa2pRole) -> CellSpec {
    let threshold = match role {
        Ssa2pRole::Source { .. } | Ssa2pRole::Relay { .. } | Ssa2pRole::Effect { .. } => 1,
        Ssa2pRole::Gate { .. } => 2,
        Ssa2pRole::Contender { .. } => FIRING_THRESHOLD,
    };
    let physical_id = seed
        .wrapping_mul(1_000_003)
        .wrapping_add(ssa2p_role_code(role))
        .wrapping_add(1 << 40);
    CellSpec {
        physical_id,
        position: (ssa2p_role_code(role) % i32::MAX as u64) as i32,
        region: 0,
        threshold,
        state: 0,
        generation: 1,
        resistance: 16,
    }
}

fn ssa2p_roles(options: Ssa2pBuildOptions) -> Vec<Ssa2pRole> {
    let mut roles = Vec::new();
    for layer in 0..options.depth {
        for state in 0..SSA2P_STATE_COUNT {
            roles.push(Ssa2pRole::Source { layer, state });
            for side in 0..2 {
                for ordinal in 0..options.live[side].saturating_sub(1) {
                    roles.push(Ssa2pRole::Relay {
                        layer,
                        state,
                        side,
                        ordinal,
                    });
                }
                if options.live[side] > 0 {
                    roles.push(Ssa2pRole::Gate { layer, state, side });
                }
                roles.push(Ssa2pRole::Contender { layer, state, side });
                roles.push(Ssa2pRole::Effect { layer, state, side });
            }
        }
    }
    if options.reverse_allocation {
        roles.reverse();
    }
    roles
}

fn ssa2p_arrows(options: Ssa2pBuildOptions) -> Vec<Ssa2pArrow> {
    let mut arrows = Vec::new();
    for layer in 0..options.depth {
        for state in 0..SSA2P_STATE_COUNT {
            let source = Ssa2pRole::Source { layer, state };
            for side in 0..2 {
                let blocked = options.blocked_layer_side == Some((layer, side));
                let contender = Ssa2pRole::Contender { layer, state, side };
                let effect = Ssa2pRole::Effect { layer, state, side };
                if !blocked {
                    for ordinal in 0..options.live[side].saturating_sub(1) {
                        let relay = Ssa2pRole::Relay {
                            layer,
                            state,
                            side,
                            ordinal,
                        };
                        arrows.push(Ssa2pArrow {
                            from: source,
                            to: relay,
                            delay: [1, 3, 5][ordinal],
                            phase: 0,
                            coupling: 1,
                        });
                        arrows.push(Ssa2pArrow {
                            from: relay,
                            to: contender,
                            delay: 0,
                            phase: 0,
                            coupling: 1,
                        });
                    }
                    if options.live[side] > 0 {
                        let gate = Ssa2pRole::Gate { layer, state, side };
                        arrows.push(Ssa2pArrow {
                            from: source,
                            to: gate,
                            delay: 0,
                            phase: 0,
                            coupling: 1,
                        });
                        arrows.push(Ssa2pArrow {
                            from: gate,
                            to: contender,
                            delay: 0,
                            phase: 0,
                            coupling: 1,
                        });
                    }
                    arrows.push(Ssa2pArrow {
                        from: contender,
                        to: effect,
                        delay: 0,
                        phase: 0,
                        coupling: 1,
                    });
                }
                arrows.push(Ssa2pArrow {
                    from: contender,
                    to: Ssa2pRole::Contender {
                        layer,
                        state,
                        side: 1 - side,
                    },
                    delay: 0,
                    phase: -100,
                    coupling: SSA2P_INHIBITION,
                });
                if layer + 1 < options.depth
                    && options.broken_transition != Some((layer, state, side))
                {
                    let physical_side = if options.mirror { 1 - side } else { side };
                    let next = ssa2p_transition(state, physical_side);
                    arrows.push(Ssa2pArrow {
                        from: effect,
                        to: Ssa2pRole::Source {
                            layer: layer + 1,
                            state: next,
                        },
                        delay: 2,
                        phase: 0,
                        coupling: 1,
                    });
                }
            }
        }
    }
    if options.reverse_allocation {
        arrows.reverse();
    }
    arrows
}

// SSA2P_PHYSICAL_PATH_BEGIN
fn ssa2p_build(options: Ssa2pBuildOptions) -> Ssa2pWorld {
    let mut substrate = Substrate::new();
    let mut ids = std::collections::BTreeMap::new();
    let mut by_physical = std::collections::BTreeMap::new();
    for role in ssa2p_roles(options) {
        let spec = ssa2p_cell_spec(options.seed, role);
        by_physical.insert(spec.physical_id, role);
        ids.insert(role, substrate.add_cell(spec));
    }
    for arrow in ssa2p_arrows(options) {
        substrate.add_arrow(ArrowSpec {
            from: ids[&arrow.from],
            to: ids[&arrow.to],
            delay: arrow.delay,
            transient_delay: 0,
            phase: arrow.phase,
            coupling: arrow.coupling,
            generation: 1,
            resistance: 16,
        });
    }
    Ssa2pWorld {
        substrate,
        ids,
        by_physical,
        options,
    }
}

fn ssa2p_history_side(history: u64, layer: usize, history_bits: usize) -> usize {
    ((history >> (layer % history_bits)) & 1) as usize
}

fn ssa2p_inject(world: &mut Ssa2pWorld, history: u64) {
    let first_source = world.ids[&Ssa2pRole::Source { layer: 0, state: 0 }];
    world.substrate.enter(SpikeInput {
        arrival_tick: 0,
        phase: 0,
        origin_physical: world.options.seed.wrapping_add(1),
        target: first_source,
        impulse: 1,
    });
    for layer in 0..world.options.depth {
        let early_physical_side =
            ssa2p_history_side(history, layer, world.options.history_bits);
        for state in 0..SSA2P_STATE_COUNT {
            for side in 0..2 {
                if world.options.live[side] == 0 {
                    continue;
                }
                let physical_side = if world.options.mirror { 1 - side } else { side };
                let local_tick = if physical_side == early_physical_side {
                    6
                } else {
                    7
                };
                world.substrate.enter(SpikeInput {
                    arrival_tick: layer as i32 * SSA2P_STEP_TICKS + local_tick,
                    phase: side as i32 - 1,
                    origin_physical: world
                        .options
                        .seed
                        .wrapping_add(9_000_000)
                        .wrapping_add(layer as u64 * 100)
                        .wrapping_add(state as u64 * 10)
                        .wrapping_add(side as u64),
                    target: world.ids[&Ssa2pRole::Gate { layer, state, side }],
                    impulse: 1,
                });
            }
        }
    }
}

fn ssa2p_propagate(mut world: Ssa2pWorld, history: u64) -> (Execution, Ssa2pWorld) {
    ssa2p_inject(&mut world, history);
    let execution = world.substrate.propagate();
    (execution, world)
}
// SSA2P_PHYSICAL_PATH_END

fn ssa2p_decode(
    execution: &Execution,
    world: &Ssa2pWorld,
    history: u64,
) -> Ssa2pTrajectory {
    let mut states = Vec::new();
    let mut physical_sides = Vec::new();
    let mut unique_per_layer = true;
    for layer in 0..world.options.depth {
        let fired_roles: Vec<_> = execution
            .fired
            .iter()
            .filter_map(|physical| world.by_physical.get(physical).copied())
            .collect();
        let layer_states: Vec<_> = fired_roles
            .iter()
            .filter_map(|role| match *role {
                Ssa2pRole::Source {
                    layer: candidate,
                    state,
                } if candidate == layer => Some(state),
                _ => None,
            })
            .collect();
        let layer_effects: Vec<_> = fired_roles
            .iter()
            .filter_map(|role| match *role {
                Ssa2pRole::Effect {
                    layer: candidate,
                    state,
                    side,
                } if candidate == layer => Some((state, side)),
                _ => None,
            })
            .collect();
        unique_per_layer &= layer_states.len() == 1
            && layer_effects.len() == 1
            && layer_effects.first().is_some_and(|effect| effect.0 == layer_states[0]);
        if let (Some(state), Some((_, side))) = (layer_states.first(), layer_effects.first()) {
            states.push(*state);
            let physical_side = if world.options.mirror { 1 - *side } else { *side };
            physical_sides.push(physical_side);
        }
    }
    let transitions_valid = states.windows(2).enumerate().all(|(layer, pair)| {
        ssa2p_transition(pair[0], physical_sides[layer]) == pair[1]
    });
    let complete = states.len() == world.options.depth
        && physical_sides.len() == world.options.depth;
    let structurally_valid = complete && unique_per_layer && transitions_valid;
    let handles = physical_sides
        .iter()
        .map(|side| if world.options.handle_swap { 1 - side } else { *side })
        .collect();
    Ssa2pTrajectory {
        history,
        depth: world.options.depth,
        states,
        physical_sides,
        handles,
        complete,
        structurally_valid,
        naturally_quiescent: execution.naturally_quiescent,
        duplicate_exact: false,
        one_propagation: true,
        permanent_fingerprint: execution.permanent_fingerprint,
        start_fingerprint: execution.start_fingerprint,
        trace_fingerprint: execution.trace_fingerprint,
        end_fingerprint: execution.end_fingerprint,
        work: execution.work.total(),
    }
}

fn ssa2p_trajectory(options: Ssa2pBuildOptions, history: u64) -> Ssa2pTrajectory {
    let first_world = ssa2p_build(options);
    let second_world = first_world.clone();
    let (first_execution, first_world) = ssa2p_propagate(first_world, history);
    let (second_execution, _) = ssa2p_propagate(second_world, history);
    let duplicate_exact = first_execution == second_execution;
    let mut trajectory = ssa2p_decode(&first_execution, &first_world, history);
    trajectory.duplicate_exact = duplicate_exact;
    trajectory
}

fn ssa2p_learned_landscape(seed: u64, stable: [bool; 2]) -> Landscape {
    train_world(seed, stable, seed.is_multiple_of(2)).0
}

fn ssa2p_options(
    seed: u64,
    depth: usize,
    histories: usize,
    live: [usize; 2],
    index: usize,
) -> Ssa2pBuildOptions {
    assert!(histories.is_power_of_two() && histories >= 2);
    Ssa2pBuildOptions {
        depth,
        history_bits: histories.trailing_zeros() as usize,
        seed,
        live,
        reverse_allocation: index.is_multiple_of(2),
        mirror: index % 3 == 1,
        handle_swap: index % 3 == 2,
        blocked_layer_side: None,
        broken_transition: None,
    }
}

fn ssa2p_sequence_key(trajectory: &Ssa2pTrajectory) -> String {
    trajectory
        .physical_sides
        .iter()
        .map(|side| char::from(b'0' + *side as u8))
        .collect()
}

fn ssa2p_source_audit() -> bool {
    let source = include_str!("ssa2_p_experiment.inc.rs");
    let physical = source
        .split("// SSA2P_PHYSICAL_PATH_BEGIN")
        .nth(1)
        .and_then(|tail| tail.split("// SSA2P_PHYSICAL_PATH_END").next())
        .unwrap_or_default();
    physical.matches(".propagate()").count() == 1
        && !physical.contains("random")
        && !physical.contains("softmax")
        && !physical.contains("temperature")
        && !physical.contains("probability")
        && !physical.contains("decoder")
        && !physical.contains("choose")
}

fn ssa2p_run_cell(stage: Ssa2pStage, seed: u64, index: usize) -> Ssa2pCell {
    let depth = stage.depth();
    let histories = stage.histories();
    let learned = ssa2p_learned_landscape(seed, [true, true]);
    let learned_live = learned.live_supporters;
    let learned_from_blank = learned_live == [4, 4]
        && learned.admissions == [4, 4]
        && learned.value_score.iter().all(|score| *score <= 0);
    let options = ssa2p_options(
        seed + 100_000_000,
        depth,
        histories,
        learned_live,
        index,
    );
    let trajectories: Vec<_> = (0..histories as u64)
        .map(|history| ssa2p_trajectory(options, history))
        .collect();
    let distinct: std::collections::BTreeSet<_> =
        trajectories.iter().map(ssa2p_sequence_key).collect();
    let trace_fingerprints: std::collections::BTreeSet<_> = trajectories
        .iter()
        .map(|trajectory| trajectory.trace_fingerprint)
        .collect();
    let permanent: std::collections::BTreeSet<_> = trajectories
        .iter()
        .map(|trajectory| trajectory.permanent_fingerprint)
        .collect();
    let both_sides_every_layer = (0..depth).all(|layer| {
        let sides: std::collections::BTreeSet<_> = trajectories
            .iter()
            .filter_map(|trajectory| trajectory.physical_sides.get(layer).copied())
            .collect();
        sides.len() == 2
    });

    let collapsed = ssa2p_learned_landscape(seed + 1_000_000, [true, false]);
    let collapsed_live = collapsed.live_supporters;
    let collapsed_options = ssa2p_options(
        seed + 101_000_000,
        depth,
        histories,
        collapsed_live,
        index,
    );
    let collapsed_trajectories: Vec<_> = (0..histories as u64)
        .map(|history| ssa2p_trajectory(collapsed_options, history))
        .collect();
    let collapsed_distinct: std::collections::BTreeSet<_> = collapsed_trajectories
        .iter()
        .map(ssa2p_sequence_key)
        .collect();
    let collapsed_physical_side = usize::from(collapsed_options.mirror);
    let collapsed_control = collapsed_live[0] == 4
        && collapsed_live[1] < FIRING_THRESHOLD as usize
        && collapsed_distinct.len() == 1
        && collapsed_trajectories.iter().all(|trajectory| {
            trajectory.complete
                && trajectory.structurally_valid
                && trajectory
                    .physical_sides
                    .iter()
                    .all(|side| *side == collapsed_physical_side)
        });

    let control_history = histories.saturating_sub(1) as u64;
    let reference = &trajectories[control_history as usize];
    let control_layer = depth / 2;
    let blocked_side = reference.physical_sides[control_layer];
    let blocked_internal_side = if options.mirror {
        1 - blocked_side
    } else {
        blocked_side
    };
    let blocked = ssa2p_trajectory(
        Ssa2pBuildOptions {
            blocked_layer_side: Some((control_layer, blocked_internal_side)),
            ..options
        },
        control_history,
    );
    let blocked_control = blocked.complete
        && blocked.structurally_valid
        && blocked.physical_sides[control_layer] != blocked_side;

    let broken_layer = depth / 2;
    let broken_state = reference.states[broken_layer];
    let broken_side = reference.physical_sides[broken_layer];
    let broken = ssa2p_trajectory(
        Ssa2pBuildOptions {
            broken_transition: Some((
                broken_layer,
                broken_state,
                if options.mirror {
                    1 - broken_side
                } else {
                    broken_side
                },
            )),
            ..options
        },
        control_history,
    );
    let broken_transition_control = !broken.complete
        && broken.states.len() == broken_layer + 1
        && broken.physical_sides.len() == broken_layer + 1
        && broken.naturally_quiescent;

    let swapped = ssa2p_trajectory(
        Ssa2pBuildOptions {
            handle_swap: !options.handle_swap,
            ..options
        },
        control_history,
    );
    let handle_permutation_control = swapped.physical_sides == reference.physical_sides
        && swapped.states == reference.states
        && swapped
            .handles
            .iter()
            .zip(&reference.handles)
            .all(|(first, second)| *first == 1 - *second);
    let duplicate_handle_control = {
        let one_physical = std::collections::BTreeSet::from([ssa2p_sequence_key(reference)]);
        let duplicate_handles = [ssa2p_sequence_key(reference), ssa2p_sequence_key(reference)];
        one_physical.len() == 1
            && duplicate_handles
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                == 1
    };
    let no_transient_first = ssa2p_trajectory(options, 0);
    let no_transient_second = ssa2p_trajectory(options, 0);
    let no_transient_control = no_transient_first == no_transient_second
        && no_transient_first.duplicate_exact;

    let main_valid = trajectories.iter().all(|trajectory| {
        trajectory.complete
            && trajectory.structurally_valid
            && trajectory.naturally_quiescent
            && trajectory.duplicate_exact
            && trajectory.one_propagation
    });
    let permanent_exact = permanent.len() == 1;
    let controls_passed = collapsed_control
        && blocked_control
        && broken_transition_control
        && duplicate_handle_control
        && handle_permutation_control
        && no_transient_control
        && ssa2p_source_audit();
    let passed = learned_from_blank
        && main_valid
        && distinct.len() >= histories / 2
        && trace_fingerprints.len() >= histories / 2
        && both_sides_every_layer
        && permanent_exact
        && controls_passed;
    Ssa2pCell {
        seed,
        depth,
        histories,
        learned_live,
        learned_from_blank,
        trajectories,
        distinct_trajectories: distinct.len(),
        distinct_trace_fingerprints: trace_fingerprints.len(),
        both_sides_every_layer,
        permanent_exact,
        collapsed_live,
        collapsed_distinct_trajectories: collapsed_distinct.len(),
        collapsed_control,
        blocked_control,
        broken_transition_control,
        duplicate_handle_control,
        handle_permutation_control,
        no_transient_control,
        controls_passed,
        passed,
    }
}

fn ssa2p_frozen_parent_exact() -> bool {
    include_str!("ssa1_learned_variation_control.rs")
        .contains("pub const FROZEN_ORGANISM: &str = \"2125a197ad0e5796a12668cd76c2071236763af0\"")
        && include_str!("ssa1_s3_structural_commitment_causality.rs")
            .contains("ssa1_s3_composition.rs")
        && include_str!("../experiments/ssa1_s3_structural_commitment_causality_development_classification.md")
            .contains("Classification D — P8 readout, not established causal boundary")
}

fn ssa2p_report(stage: Ssa2pStage) -> Ssa2pReport {
    let cells: Vec<_> = stage
        .seeds()
        .iter()
        .enumerate()
        .map(|(index, seed)| ssa2p_run_cell(stage, *seed, index))
        .collect();
    let valid_trajectories = cells
        .iter()
        .flat_map(|cell| &cell.trajectories)
        .filter(|trajectory| {
            trajectory.complete
                && trajectory.structurally_valid
                && trajectory.naturally_quiescent
                && trajectory.duplicate_exact
        })
        .count();
    let total_trajectories = cells.iter().map(|cell| cell.trajectories.len()).sum();
    let minimum_distinct_trajectories = cells
        .iter()
        .map(|cell| cell.distinct_trajectories)
        .min()
        .unwrap_or(0);
    let frozen_parent_exact = ssa2p_frozen_parent_exact();
    let all_passed = cells.iter().all(|cell| cell.passed) && frozen_parent_exact;
    let local_differentiation = cells.iter().all(|cell| {
        cell.trajectories
            .iter()
            .flat_map(|trajectory| &trajectory.physical_sides)
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            == 2
    });
    let classification = if all_passed && matches!(stage, Ssa2pStage::Gate) {
        "A — conditional long-trajectory generativity"
    } else if all_passed {
        "A-DEVELOPING — stage target positive"
    } else if matches!(stage, Ssa2pStage::Gate)
        && cells.iter().all(|cell| cell.learned_from_blank)
        && local_differentiation
    {
        "C — transient differentiation without generative composition"
    } else if cells.iter().all(|cell| cell.learned_from_blank) && !local_differentiation {
        "D — no trajectory diversity"
    } else {
        "E — scientific ambiguity"
    };
    Ssa2pReport {
        protocol: SSA2P_PROTOCOL,
        stage: stage.name(),
        depth: stage.depth(),
        histories: stage.histories(),
        cells,
        valid_trajectories,
        total_trajectories,
        minimum_distinct_trajectories,
        frozen_parent_exact,
        classification,
        claim_eligible: false,
        passed: all_passed,
    }
}

pub fn run_ssa2_p_probe() -> Ssa2pReport {
    ssa2p_report(Ssa2pStage::Probe)
}

pub fn run_ssa2_p_micro() -> Ssa2pReport {
    ssa2p_report(Ssa2pStage::Micro)
}

pub fn run_ssa2_p_gate() -> Ssa2pReport {
    ssa2p_report(Ssa2pStage::Gate)
}

#[cfg(test)]
mod ssa2p_tests {
    use super::*;

    #[test]
    fn definitive_surface_is_absent() {
        assert!(!SSA2P_PROTOCOL.contains("definitive"));
    }

    #[test]
    #[ignore = "explicit SSA2-P PROBE"]
    fn probe_composes_preserved_affordances() {
        assert!(run_ssa2_p_probe().passed);
    }
}
