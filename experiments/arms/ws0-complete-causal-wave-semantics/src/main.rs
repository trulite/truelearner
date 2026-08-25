#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, TransmissionTrigger, Work,
};

const ROOTS: [u64; 2] = [9_100_000, 9_300_001];
const CEILING: u64 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    MultipleDrive,
    MultipleModulatory,
    MixedSameJunction,
    MixedDifferentJunctions,
    DriveCausesModulatory,
    SourceFiresModulatory,
    PqlcContinuation,
    ZeroDelayDriveChain,
    ZeroDelayFanoutMerge,
    ZeroDelayPqlcChain,
    RecurrentDrive,
    MixedRecurrence,
    HandleRenaming,
    InsertionOrder,
}

impl Family {
    const ALL: [Self; 14] = [
        Self::MultipleDrive,
        Self::MultipleModulatory,
        Self::MixedSameJunction,
        Self::MixedDifferentJunctions,
        Self::DriveCausesModulatory,
        Self::SourceFiresModulatory,
        Self::PqlcContinuation,
        Self::ZeroDelayDriveChain,
        Self::ZeroDelayFanoutMerge,
        Self::ZeroDelayPqlcChain,
        Self::RecurrentDrive,
        Self::MixedRecurrence,
        Self::HandleRenaming,
        Self::InsertionOrder,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::MultipleDrive => "multiple_signed_drive_one_junction",
            Self::MultipleModulatory => "multiple_modulatory_one_junction",
            Self::MixedSameJunction => "mixed_drive_modulatory_same_junction",
            Self::MixedDifferentJunctions => "mixed_drive_modulatory_different_junctions",
            Self::DriveCausesModulatory => "drive_causes_later_modulatory",
            Self::SourceFiresModulatory => "source_fires_modulatory",
            Self::PqlcContinuation => "modulatory_causes_pqlc",
            Self::ZeroDelayDriveChain => "zero_delay_drive_chain",
            Self::ZeroDelayFanoutMerge => "zero_delay_fanout_merge",
            Self::ZeroDelayPqlcChain => "zero_delay_modulatory_pqlc_chain",
            Self::RecurrentDrive => "recurrent_drive",
            Self::MixedRecurrence => "mixed_recurrence_modulatory",
            Self::HandleRenaming => "handle_renaming",
            Self::InsertionOrder => "insertion_order",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Permutation {
    Identity,
    RenamePhysical,
    ReverseCells,
    ReverseArrows,
    ReverseInputs,
}

impl Permutation {
    const ALL: [Self; 5] = [
        Self::Identity,
        Self::RenamePhysical,
        Self::ReverseCells,
        Self::ReverseArrows,
        Self::ReverseInputs,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::RenamePhysical => "rename_physical",
            Self::ReverseCells => "reverse_cells",
            Self::ReverseArrows => "reverse_arrows",
            Self::ReverseInputs => "reverse_inputs",
        }
    }
}

#[derive(Clone, Copy)]
struct CellDef {
    name: &'static str,
    position: i32,
    threshold: i32,
}

#[derive(Clone, Copy)]
struct ArrowDef {
    name: &'static str,
    from: &'static str,
    to: &'static str,
    delay: i64,
    phase: i32,
    coupling: i32,
    mode: TransmissionMode,
    trigger: TransmissionTrigger,
}

#[derive(Clone, Copy)]
struct InputDef {
    target: &'static str,
    tick: i64,
    phase: i32,
    impulse: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct PhysicalCounts {
    drive_deliveries: u64,
    modulatory_deliveries: u64,
    fires: BTreeMap<String, u64>,
    resistance_updates: u64,
    coupling_updates: u64,
    qlp_traversals: u64,
    proposals: u64,
    deallocations: u64,
    maximum_wave: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<String>,
    counts: PhysicalCounts,
    work: Work,
    final_tick: i64,
    body_hash: String,
    continuation_hash: String,
    naturally_quiescent: bool,
    ceiling_reached: bool,
    checks: Vec<(String, bool)>,
}

impl Observation {
    fn passed(&self) -> bool {
        self.checks.iter().all(|(_, pass)| *pass)
    }

    fn failures(&self) -> String {
        self.checks
            .iter()
            .filter(|(_, pass)| !pass)
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join("|")
    }
}

struct World {
    body: PlasticSubstrate,
    mechanics: MechanicalConfig,
    cells: HashMap<&'static str, CellId>,
    cell_names: HashMap<CellId, &'static str>,
    arrows: HashMap<&'static str, ArrowId>,
    arrow_names: HashMap<ArrowId, &'static str>,
    physical_names: HashMap<u64, &'static str>,
}

fn cell_defs() -> Vec<CellDef> {
    vec![
        CellDef {
            name: "a",
            position: 0,
            threshold: 1,
        },
        CellDef {
            name: "b",
            position: 10,
            threshold: 1,
        },
        CellDef {
            name: "c",
            position: 20,
            threshold: 2,
        },
        CellDef {
            name: "d",
            position: 30,
            threshold: 2,
        },
        CellDef {
            name: "x",
            position: 40,
            threshold: 100,
        },
        CellDef {
            name: "y",
            position: 50,
            threshold: 100,
        },
        CellDef {
            name: "m1",
            position: 60,
            threshold: 1,
        },
        CellDef {
            name: "m2",
            position: 70,
            threshold: 1,
        },
        CellDef {
            name: "u",
            position: 80,
            threshold: 1,
        },
        CellDef {
            name: "v",
            position: 90,
            threshold: 1,
        },
        CellDef {
            name: "sink",
            position: 100,
            threshold: 100,
        },
    ]
}

fn physical_id(root: u64, logical_index: usize, permutation: Permutation) -> u64 {
    let offset = match permutation {
        Permutation::RenamePhysical => 100 - u64::try_from(logical_index).unwrap(),
        _ => u64::try_from(logical_index).unwrap() + 1,
    };
    root + offset
}

impl World {
    fn new(root: u64, permutation: Permutation, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 64, 128, mechanics);
        body.set_physical_tracing(true);
        let canonical = cell_defs();
        let mut insertion = canonical.clone();
        if permutation == Permutation::ReverseCells {
            insertion.reverse();
        }
        let logical_indices = canonical
            .iter()
            .enumerate()
            .map(|(index, cell)| (cell.name, index))
            .collect::<HashMap<_, _>>();
        let mut cells = HashMap::new();
        let mut cell_names = HashMap::new();
        let mut physical_names = HashMap::new();
        for def in insertion {
            let physical = physical_id(root, logical_indices[def.name], permutation);
            let id = body.add_cell(CellSpec {
                physical_id: physical,
                position: def.position,
                region: 0,
                threshold: def.threshold,
                resistance: 500,
            });
            cells.insert(def.name, id);
            cell_names.insert(id, def.name);
            physical_names.insert(physical, def.name);
        }
        Self {
            body,
            mechanics,
            cells,
            cell_names,
            arrows: HashMap::new(),
            arrow_names: HashMap::new(),
            physical_names,
        }
    }

    fn cell(&self, name: &'static str) -> CellId {
        self.cells[name]
    }

    fn add_arrows(&mut self, mut defs: Vec<ArrowDef>, permutation: Permutation) {
        if permutation == Permutation::ReverseArrows {
            defs.reverse();
        }
        for def in defs {
            let id = self.body.add_arrow_with_trigger(
                ArrowSpec {
                    from: self.cell(def.from),
                    to: self.cell(def.to),
                    delay: def.delay,
                    phase: def.phase,
                    coupling: def.coupling,
                    resistance: 500,
                    mode: def.mode,
                },
                def.trigger,
            );
            self.arrows.insert(def.name, id);
            self.arrow_names.insert(id, def.name);
        }
    }

    fn run(
        &mut self,
        mut inputs: Vec<InputDef>,
        permutation: Permutation,
    ) -> truelearner_core::ObservedRun {
        if permutation == Permutation::ReverseInputs {
            inputs.reverse();
        }
        for (serial, input) in inputs.into_iter().enumerate() {
            let origin = 99_000_000 + u64::try_from(serial).unwrap();
            self.physical_names.insert(origin, "external");
            self.body.enter(SpikeInput {
                arrival_tick: input.tick,
                phase: input.phase,
                origin_physical: origin,
                target: self.cell(input.target),
                impulse: input.impulse,
            });
        }
        self.body.propagate_with_observation_ceiling(CEILING)
    }
}

fn drive(
    name: &'static str,
    from: &'static str,
    to: &'static str,
    delay: i64,
    coupling: i32,
) -> ArrowDef {
    ArrowDef {
        name,
        from,
        to,
        delay,
        phase: 0,
        coupling,
        mode: TransmissionMode::Drive,
        trigger: TransmissionTrigger::SourceFires,
    }
}

fn modulation(name: &'static str, from: &'static str, to: &'static str, delay: i64) -> ArrowDef {
    ArrowDef {
        name,
        from,
        to,
        delay,
        phase: 0,
        coupling: 1,
        mode: TransmissionMode::Modulatory,
        trigger: TransmissionTrigger::SourceFires,
    }
}

fn qlp(name: &'static str, from: &'static str, to: &'static str, delay: i64) -> ArrowDef {
    ArrowDef {
        name,
        from,
        to,
        delay,
        phase: 0,
        coupling: 1,
        mode: TransmissionMode::Modulatory,
        trigger: TransmissionTrigger::QualifiedLocalParticipation,
    }
}

fn input(target: &'static str, tick: i64, impulse: i32) -> InputDef {
    InputDef {
        target,
        tick,
        phase: 0,
        impulse,
    }
}

fn definitions(family: Family) -> (Vec<ArrowDef>, Vec<InputDef>) {
    match family {
        Family::MultipleDrive => (Vec::new(), vec![input("c", 0, 2), input("c", 0, -1)]),
        Family::MultipleModulatory => (
            vec![
                drive("c_x", "c", "x", 1, 1),
                modulation("m1_c", "m1", "c", 1),
                modulation("m2_c", "m2", "c", 1),
            ],
            vec![input("c", 0, 2), input("m1", 0, 1), input("m2", 0, 1)],
        ),
        Family::MixedSameJunction => (
            vec![
                drive("c_x", "c", "x", 1, 1),
                drive("d_y", "d", "y", 1, 1),
                modulation("m1_c", "m1", "c", 1),
                modulation("m2_d", "m2", "d", 1),
            ],
            vec![
                input("c", 0, 2),
                input("m1", 0, 1),
                input("m2", 0, 1),
                input("c", 1, 2),
                input("d", 1, 2),
            ],
        ),
        Family::MixedDifferentJunctions => (
            vec![
                drive("c_x", "c", "x", 1, 1),
                modulation("m1_c", "m1", "c", 1),
            ],
            vec![input("c", 0, 2), input("m1", 0, 1), input("a", 1, 1)],
        ),
        Family::DriveCausesModulatory => (
            vec![
                drive("c_x", "c", "x", 1, 1),
                drive("a_m1", "a", "m1", 0, 1),
                modulation("m1_c", "m1", "c", 0),
            ],
            vec![input("c", 0, 2), input("a", 1, 1)],
        ),
        Family::SourceFiresModulatory => (
            vec![
                drive("c_x", "c", "x", 1, 1),
                modulation("m1_c", "m1", "c", 0),
            ],
            vec![input("c", 0, 2), input("m1", 1, 1)],
        ),
        Family::PqlcContinuation | Family::ZeroDelayPqlcChain => (
            vec![
                drive("c_x", "c", "x", 1, 1),
                drive("d_y", "d", "y", 1, 1),
                modulation("m1_d", "m1", "d", 0),
                qlp("d_c", "d", "c", 0),
            ],
            vec![input("c", 0, 2), input("d", 0, 2), input("m1", 1, 1)],
        ),
        Family::ZeroDelayDriveChain => (
            vec![drive("a_b", "a", "b", 0, 1), drive("b_c", "b", "c", 0, 2)],
            vec![input("a", 0, 1)],
        ),
        Family::ZeroDelayFanoutMerge | Family::InsertionOrder => (
            vec![
                drive("a_b", "a", "b", 0, 1),
                drive("a_u", "a", "u", 0, 1),
                drive("b_c", "b", "c", 0, 1),
                drive("u_c", "u", "c", 0, 1),
            ],
            vec![input("a", 0, 1)],
        ),
        Family::RecurrentDrive | Family::HandleRenaming => (
            vec![drive("a_b", "a", "b", 0, 1), drive("b_a", "b", "a", 0, 1)],
            vec![input("a", 0, 1)],
        ),
        Family::MixedRecurrence => (
            vec![
                drive("a_b", "a", "b", 0, 1),
                drive("b_a", "b", "a", 0, 1),
                drive("c_x", "c", "x", 1, 1),
                modulation("a_c", "a", "c", 0),
            ],
            vec![input("c", 0, 2), input("a", 1, 1)],
        ),
    }
}

fn logical_event(world: &World, event: &PhysicalEvent) -> String {
    let cell = |id: CellId| world.cell_names.get(&id).copied().unwrap_or("generated");
    let arrow = |id: ArrowId| {
        world
            .arrow_names
            .get(&id)
            .copied()
            .unwrap_or("generated_link")
    };
    match event {
        PhysicalEvent::DriveIncidence {
            target,
            arrivals,
            impulse,
            causal_wave,
        } => format!(
            "DRIVE:{}:{arrivals}:{impulse}:wave={causal_wave}",
            cell(*target)
        ),
        PhysicalEvent::ModulatoryIncidence {
            target,
            arrivals,
            impulse,
            causal_wave,
        } => format!(
            "MOD:{}:{arrivals}:{impulse}:wave={causal_wave}",
            cell(*target)
        ),
        PhysicalEvent::Deliver {
            mode,
            target,
            impulse,
        } => format!("DELIVER:{mode:?}:{}:{impulse}", cell(*target)),
        PhysicalEvent::Fire { cell: id } => format!("FIRE:{}", cell(*id)),
        PhysicalEvent::Resistance {
            arrow: id,
            before,
            after,
        } => format!("R:{}:{before}:{after}", arrow(*id)),
        PhysicalEvent::Coupling {
            arrow: id,
            before,
            after,
        } => format!("C:{}:{before}:{after}", arrow(*id)),
        PhysicalEvent::Deallocate { arrow: id } => format!("DEALLOCATE:{}", arrow(*id)),
        PhysicalEvent::CellDeallocate {
            cell: id,
            before_generation,
            after_generation,
        } => format!(
            "CELL_DEALLOCATE:{}:{}:{}",
            cell(*id),
            before_generation.0,
            after_generation.0
        ),
        PhysicalEvent::CellProposal {
            cell: id,
            source,
            target,
        } => format!(
            "CELL_PROPOSAL:{}:{}:{}",
            cell(*id),
            cell(*source),
            cell(*target)
        ),
        PhysicalEvent::Proposal {
            arrow: id,
            from,
            to,
        } => format!("PROPOSAL:{}:{}:{}", arrow(*id), cell(*from), cell(*to)),
        PhysicalEvent::Crossing(crossing) => format!(
            "CROSSING:{}:{}:{}",
            crossing.from_region, crossing.to_region, crossing.impulse
        ),
        PhysicalEvent::QualifiedLocalTraversal { arrow: id } => format!("QLP:{}", arrow(*id)),
    }
}

fn normalized_trace(world: &World, trace: &[PhysicalTransition]) -> Vec<String> {
    let mut moments: BTreeMap<(i64, i32), Vec<String>> = BTreeMap::new();
    for transition in trace {
        moments
            .entry((transition.tick, transition.phase))
            .or_default()
            .push(logical_event(world, &transition.event));
    }
    moments
        .into_iter()
        .map(|((tick, phase), mut events)| {
            events.sort();
            format!("{tick}:{phase}:[{}]", events.join("|"))
        })
        .collect()
}

fn body_hash(world: &World) -> String {
    let body = world.body.arena_body(1);
    let mut rows = body
        .cells
        .iter()
        .map(|cell| {
            format!(
                "CELL:{}:{}:{}:{}:{}:{}",
                world.cell_names[&cell.id],
                cell.position,
                cell.threshold,
                cell.resistance,
                cell.generation.0,
                cell.live
            )
        })
        .collect::<Vec<_>>();
    rows.extend(body.arrows.iter().map(|arrow| {
        format!(
            "ARROW:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            world
                .arrow_names
                .get(&arrow.id)
                .copied()
                .unwrap_or("generated_link"),
            world
                .cell_names
                .get(&arrow.from.id)
                .copied()
                .unwrap_or("generated"),
            world
                .cell_names
                .get(&arrow.to.id)
                .copied()
                .unwrap_or("generated"),
            arrow.delay,
            arrow.phase,
            arrow.coupling,
            arrow.resistance,
            arrow.transmission_mode,
            arrow.live
        )
    }));
    rows.sort();
    ContentHash::of(rows.join("\n").as_bytes()).to_string()
}

fn counts(world: &World, trace: &[PhysicalTransition]) -> PhysicalCounts {
    let mut counts = PhysicalCounts::default();
    for transition in trace {
        match &transition.event {
            PhysicalEvent::DriveIncidence {
                arrivals,
                causal_wave,
                ..
            } => {
                counts.drive_deliveries =
                    counts.drive_deliveries.saturating_add(u64::from(*arrivals));
                counts.maximum_wave = counts.maximum_wave.max(*causal_wave);
            }
            PhysicalEvent::ModulatoryIncidence {
                arrivals,
                causal_wave,
                ..
            } => {
                counts.modulatory_deliveries = counts
                    .modulatory_deliveries
                    .saturating_add(u64::from(*arrivals));
                counts.maximum_wave = counts.maximum_wave.max(*causal_wave);
            }
            PhysicalEvent::Fire { cell } => {
                *counts
                    .fires
                    .entry(world.cell_names[cell].into())
                    .or_default() += 1
            }
            PhysicalEvent::Resistance { .. } => counts.resistance_updates += 1,
            PhysicalEvent::Coupling { .. } => counts.coupling_updates += 1,
            PhysicalEvent::QualifiedLocalTraversal { .. } => counts.qlp_traversals += 1,
            PhysicalEvent::Proposal { .. } | PhysicalEvent::CellProposal { .. } => {
                counts.proposals += 1
            }
            PhysicalEvent::Deallocate { .. } | PhysicalEvent::CellDeallocate { .. } => {
                counts.deallocations += 1
            }
            PhysicalEvent::Deliver { .. } | PhysicalEvent::Crossing(_) => {}
        }
    }
    counts
}

fn fire_count(counts: &PhysicalCounts, name: &str) -> u64 {
    counts.fires.get(name).copied().unwrap_or(0)
}

fn checks(
    family: Family,
    counts: &PhysicalCounts,
    quiescent: bool,
    ceiling: bool,
) -> Vec<(String, bool)> {
    let mut result = vec![("naturally_quiescent".into(), quiescent && !ceiling)];
    match family {
        Family::MultipleDrive => result.extend([
            ("signed_drive_combined".into(), counts.drive_deliveries == 2),
            ("subthreshold_no_fire".into(), fire_count(counts, "c") == 0),
        ]),
        Family::MultipleModulatory => result.extend([
            (
                "two_modulatory_arrivals".into(),
                counts.modulatory_deliveries == 2,
            ),
            (
                "modulation_does_not_fire".into(),
                fire_count(counts, "c") == 1,
            ),
            (
                "multiplicity_retained".into(),
                counts.resistance_updates == 2,
            ),
        ]),
        Family::MixedSameJunction => result.extend([
            (
                "preparticipating_contact_updates".into(),
                counts.resistance_updates == 1,
            ),
            (
                "mixed_drive_fires_once".into(),
                fire_count(counts, "c") == 2,
            ),
            (
                "fresh_same_wave_participation_not_visible".into(),
                fire_count(counts, "d") == 1,
            ),
        ]),
        Family::MixedDifferentJunctions => result.extend([
            (
                "different_junction_drive_executes".into(),
                fire_count(counts, "a") == 1,
            ),
            (
                "different_junction_modulates".into(),
                counts.resistance_updates == 1,
            ),
        ]),
        Family::DriveCausesModulatory => result.extend([
            (
                "drive_chain_fires_modulator".into(),
                fire_count(counts, "m1") == 1,
            ),
            (
                "caused_modulation_updates".into(),
                counts.resistance_updates == 1,
            ),
            ("causal_waves_advance".into(), counts.maximum_wave >= 2),
        ]),
        Family::SourceFiresModulatory => result.extend([
            (
                "ordinary_modulator_fires".into(),
                fire_count(counts, "m1") == 1,
            ),
            (
                "ordinary_modulatory_arrow_updates".into(),
                counts.resistance_updates == 1,
            ),
            ("modulation_next_wave".into(), counts.maximum_wave >= 1),
        ]),
        Family::PqlcContinuation | Family::ZeroDelayPqlcChain => result.extend([
            ("pqlc_traverses".into(), counts.qlp_traversals == 1),
            (
                "both_contacts_update".into(),
                counts.resistance_updates == 2,
            ),
            ("pqlc_next_wave".into(), counts.maximum_wave >= 2),
        ]),
        Family::ZeroDelayDriveChain => result.extend([
            (
                "chain_fires".into(),
                fire_count(counts, "a") == 1
                    && fire_count(counts, "b") == 1
                    && fire_count(counts, "c") == 1,
            ),
            ("chain_waves".into(), counts.maximum_wave == 2),
        ]),
        Family::ZeroDelayFanoutMerge | Family::InsertionOrder => result.extend([
            (
                "fanout_fires_once".into(),
                fire_count(counts, "b") == 1 && fire_count(counts, "u") == 1,
            ),
            ("merge_combines_once".into(), fire_count(counts, "c") == 1),
            ("merge_wave".into(), counts.maximum_wave == 2),
        ]),
        Family::RecurrentDrive | Family::HandleRenaming => result.extend([
            (
                "cycle_each_fires_once".into(),
                fire_count(counts, "a") == 1 && fire_count(counts, "b") == 1,
            ),
            ("refractory_stops_cycle".into(), counts.maximum_wave == 2),
        ]),
        Family::MixedRecurrence => result.extend([
            (
                "mixed_cycle_settles".into(),
                fire_count(counts, "a") == 1 && fire_count(counts, "b") == 1,
            ),
            (
                "mixed_modulation_updates".into(),
                counts.resistance_updates == 1,
            ),
            ("mixed_wave_progresses".into(), counts.maximum_wave >= 2),
        ]),
    }
    result
}

fn observe(
    root: u64,
    family: Family,
    permutation: Permutation,
    mechanics: MechanicalConfig,
) -> Observation {
    let mut world = World::new(root, permutation, mechanics);
    let (arrows, inputs) = definitions(family);
    world.add_arrows(arrows, permutation);
    let observed = world.run(inputs, permutation);
    let trace = observed.run.physical_trace.clone();
    let physical_counts = counts(&world, &trace);
    let body_hash = body_hash(&world);
    let checkpoint = world.body.live_checkpoint(1).unwrap();
    let mut restored =
        PlasticSubstrate::from_live_checkpoint_with_mechanics(checkpoint, world.mechanics).unwrap();
    restored.set_physical_tracing(true);
    restored.enter(SpikeInput {
        arrival_tick: restored.clock().tick.saturating_add(1),
        phase: 0,
        origin_physical: 199_000_000,
        target: world.cell("sink"),
        impulse: 1,
    });
    let continuation = restored.propagate_with_observation_ceiling(CEILING);
    let continuation_hash = ContentHash::of(
        format!(
            "{:?}|{:?}|{}|{}|{}",
            normalized_trace(&world, &continuation.run.physical_trace),
            continuation.run.work,
            restored.clock().tick,
            body_hash(&World {
                body: restored,
                mechanics: world.mechanics,
                cells: world.cells.clone(),
                cell_names: world.cell_names.clone(),
                arrows: world.arrows.clone(),
                arrow_names: world.arrow_names.clone(),
                physical_names: world.physical_names.clone()
            }),
            continuation.run.naturally_quiescent
        )
        .as_bytes(),
    )
    .to_string();
    Observation {
        trace: normalized_trace(&world, &trace),
        counts: physical_counts.clone(),
        work: observed.run.work,
        final_tick: world.body.clock().tick,
        body_hash,
        continuation_hash,
        naturally_quiescent: observed.run.naturally_quiescent,
        ceiling_reached: observed.observation_ceiling_reached,
        checks: checks(
            family,
            &physical_counts,
            observed.run.naturally_quiescent,
            observed.observation_ceiling_reached,
        ),
    }
}

fn mechanics_name(mechanics: MechanicalConfig) -> &'static str {
    if mechanics == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn main() {
    let output_dir = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/ws0_complete_causal_wave_semantics_v1")
    });
    fs::create_dir_all(&output_dir).unwrap();
    let mut csv = String::from("case,family,root,permutation,mechanics,replay_equal,mechanics_equal,permutation_equal,predicates_pass,failed,quiescent,ceiling,drive_deliveries,modulatory_deliveries,resistance_updates,coupling_updates,qlp_traversals,maximum_wave,physical_work,final_tick,trace_hash,body_hash,continuation_hash\n");
    let mut cases = 0usize;
    let mut rows = 0usize;
    let mut clauses = 0usize;
    let mut passed = 0usize;
    let mut maximum_work = 0u64;
    let mut all_pass = true;
    for root in ROOTS {
        for family in Family::ALL {
            let baseline_reference = observe(
                root,
                family,
                Permutation::Identity,
                MechanicalConfig::REFERENCE,
            );
            let baseline_production = observe(
                root,
                family,
                Permutation::Identity,
                MechanicalConfig::PRODUCTION,
            );
            for permutation in Permutation::ALL {
                cases += 1;
                let reference = observe(root, family, permutation, MechanicalConfig::REFERENCE);
                let reference_replay =
                    observe(root, family, permutation, MechanicalConfig::REFERENCE);
                let production = observe(root, family, permutation, MechanicalConfig::PRODUCTION);
                let production_replay =
                    observe(root, family, permutation, MechanicalConfig::PRODUCTION);
                let mechanics_equal = reference == production;
                let permutation_equal =
                    reference == baseline_reference && production == baseline_production;
                for (mechanics, observation, replay) in [
                    (MechanicalConfig::REFERENCE, &reference, &reference_replay),
                    (
                        MechanicalConfig::PRODUCTION,
                        &production,
                        &production_replay,
                    ),
                ] {
                    rows += 1;
                    let replay_equal = observation == replay;
                    let row_pass = observation.passed()
                        && replay_equal
                        && mechanics_equal
                        && permutation_equal;
                    let row_clauses = observation.checks.len() + 3;
                    clauses += row_clauses;
                    passed += observation.checks.iter().filter(|(_, pass)| *pass).count()
                        + usize::from(replay_equal)
                        + usize::from(mechanics_equal)
                        + usize::from(permutation_equal);
                    maximum_work = maximum_work.max(observation.work.physical_total());
                    all_pass &= row_pass;
                    let trace_hash =
                        ContentHash::of(observation.trace.join("\n").as_bytes()).to_string();
                    writeln!(csv, "{cases},{},{root},{},{},{replay_equal},{mechanics_equal},{permutation_equal},{},{},{},{},{},{},{},{},{},{},{},{},{trace_hash},{},{}", family.name(), permutation.name(), mechanics_name(mechanics), observation.passed(), observation.failures(), observation.naturally_quiescent, observation.ceiling_reached, observation.counts.drive_deliveries, observation.counts.modulatory_deliveries, observation.counts.resistance_updates, observation.counts.coupling_updates, observation.counts.qlp_traversals, observation.counts.maximum_wave, observation.work.physical_total(), observation.final_tick, observation.body_hash, observation.continuation_hash).unwrap();
                }
            }
        }
    }
    let expected_cases = ROOTS.len() * Family::ALL.len() * Permutation::ALL.len();
    let expected_rows = expected_cases * 2;
    assert_eq!(cases, expected_cases);
    assert_eq!(rows, expected_rows);
    let report = format!("# WS0 complete causal-wave semantics v1\n\n- cases: {cases}/{expected_cases}\n- rows: {rows}/{expected_rows}\n- clauses: {passed}/{clauses}\n- maximum PhysicalWork: {maximum_work}\n");
    fs::write(output_dir.join("matrix.csv"), csv).unwrap();
    fs::write(output_dir.join("report.md"), report).unwrap();
    assert!(all_pass, "WS0 matrix failed");
    println!("WS0_COMPLETE_CAUSAL_WAVE_SEMANTICS_POSITIVE_V1");
}
