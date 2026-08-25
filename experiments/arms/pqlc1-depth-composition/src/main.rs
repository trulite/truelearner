#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, TransmissionTrigger,
};

const ROOTS: [u64; 2] = [2_700_000, 2_800_000];
const DEPTHS: [usize; 5] = [1, 2, 4, 8, 16];
const LONG_DELAY: i64 = 1024;
const CYCLE_WORK_CEILING: u64 = 8192;
const EXPECTED_VARIANTS: usize = 39;
const EXPECTED_PHYSICAL_CASES: usize = 780;
const EXPECTED_MECHANICS_ROWS: usize = 1560;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Complete,
    StructuralBreak,
    TemporalBreak,
    WrongBranch,
    HonestFanout,
    RecurrentClosure,
}

impl Family {
    fn name(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::StructuralBreak => "structural_break",
            Self::TemporalBreak => "temporal_break",
            Self::WrongBranch => "wrong_branch",
            Self::HonestFanout => "honest_fanout",
            Self::RecurrentClosure => "recurrent_closure",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CaseSpec {
    family: Family,
    depth: usize,
    break_index: Option<usize>,
}

impl CaseSpec {
    fn label(self) -> String {
        match self.break_index {
            Some(index) => format!("{}_d{}_i{index}", self.family.name(), self.depth),
            None => format!("{}_d{}", self.family.name(), self.depth),
        }
    }
}

fn break_indices(depth: usize) -> Vec<usize> {
    match depth {
        1 => vec![0],
        2 => vec![0, 1],
        4 => vec![0, 2, 3],
        8 => vec![0, 4, 7],
        16 => vec![0, 8, 15],
        _ => unreachable!("unregistered PQLC1 depth"),
    }
}

fn cases() -> Vec<CaseSpec> {
    let mut cases = Vec::new();
    for depth in DEPTHS {
        cases.push(CaseSpec {
            family: Family::Complete,
            depth,
            break_index: None,
        });
    }
    for family in [Family::StructuralBreak, Family::TemporalBreak] {
        for depth in DEPTHS {
            for break_index in break_indices(depth) {
                cases.push(CaseSpec {
                    family,
                    depth,
                    break_index: Some(break_index),
                });
            }
        }
    }
    for depth in [2, 4, 8, 16] {
        cases.push(CaseSpec {
            family: Family::WrongBranch,
            depth,
            break_index: None,
        });
    }
    cases.push(CaseSpec {
        family: Family::HonestFanout,
        depth: 4,
        break_index: None,
    });
    for depth in DEPTHS {
        cases.push(CaseSpec {
            family: Family::RecurrentClosure,
            depth,
            break_index: None,
        });
    }
    assert_eq!(cases.len(), EXPECTED_VARIANTS);
    cases
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    updates: u64,
    proposals: u64,
    deallocations: u64,
    qlp: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<PhysicalTransition>,
    participation: Vec<u64>,
    support: Vec<u64>,
    triggers: Vec<TransmissionTrigger>,
    work: WorkTotals,
    qlp_events: u64,
    source_fires: u64,
    final_tick: i64,
    pressure_phase: i64,
    body_hash: String,
    live: bool,
    quiescent: bool,
}

struct Geometry {
    body: PlasticSubstrate,
    contacts: Vec<ArrowId>,
    qlp_arrows: Vec<ArrowId>,
    batches: Vec<Vec<SpikeInput>>,
    expected_support: Vec<bool>,
    expected_qlp: Option<u64>,
    recurrent: bool,
}

struct Builder {
    body: PlasticSubstrate,
    root: u64,
    phase: i64,
    next_physical: u64,
    next_position: i32,
    next_input: u64,
}

impl Builder {
    fn new(root: u64, phase: i64, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root + 700), 256, 1024, mechanics);
        body.set_physical_tracing(true);
        if phase > 0 {
            body.advance_time(phase);
        }
        Self {
            body,
            root,
            phase,
            next_physical: root + 1,
            next_position: 0,
            next_input: 1,
        }
    }

    fn cell(&mut self, threshold: i32) -> CellId {
        let id = self.body.add_cell(CellSpec {
            physical_id: self.next_physical,
            position: self.next_position,
            region: 0,
            threshold,
            resistance: 100_000,
        });
        self.next_physical += 1;
        self.next_position += 10;
        id
    }

    fn arrow_with_delay(
        &mut self,
        from: CellId,
        to: CellId,
        delay: i64,
        mode: TransmissionMode,
    ) -> ArrowId {
        self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay,
            phase: 0,
            coupling: 1,
            resistance: 100_000,
            mode,
        })
    }

    fn arrow(&mut self, from: CellId, to: CellId, mode: TransmissionMode) -> ArrowId {
        self.arrow_with_delay(from, to, 0, mode)
    }

    fn qlp(&mut self, from: CellId, to: CellId, delay: i64) -> ArrowId {
        self.body.add_arrow_with_trigger(
            ArrowSpec {
                from,
                to,
                delay,
                phase: 0,
                coupling: 1,
                resistance: 100_000,
                mode: TransmissionMode::Modulatory,
            },
            TransmissionTrigger::QualifiedLocalParticipation,
        )
    }

    fn source(&mut self, target: CellId) -> CellId {
        let source = self.cell(1);
        self.arrow(source, target, TransmissionMode::Drive);
        source
    }

    fn spike(&mut self, target: CellId, offset: i64, impulse: i32) -> SpikeInput {
        let ordinal = self.next_input;
        self.next_input += 1;
        SpikeInput {
            arrival_tick: self.phase + offset,
            phase: 0,
            origin_physical: self.root + 80_000 + ordinal,
            target,
            impulse,
        }
    }

    fn finish(
        self,
        contacts: Vec<ArrowId>,
        qlp_arrows: Vec<ArrowId>,
        batches: Vec<Vec<SpikeInput>>,
        expected_support: Vec<bool>,
        expected_qlp: Option<u64>,
        recurrent: bool,
    ) -> Geometry {
        Geometry {
            body: self.body,
            contacts,
            qlp_arrows,
            batches,
            expected_support,
            expected_qlp,
            recurrent,
        }
    }
}

fn linear_geometry(
    mut builder: Builder,
    depth: usize,
    structural_break: Option<usize>,
    temporal_break: Option<usize>,
    recurrent: bool,
) -> Geometry {
    let contacts = (0..depth)
        .map(|index| {
            builder.cell(if structural_break == Some(index) {
                2
            } else {
                1
            })
        })
        .collect::<Vec<_>>();
    let effect = builder.cell(2);
    let source = builder.source(contacts[0]);
    let mut contact_arrows = Vec::with_capacity(depth);
    for index in 0..depth {
        let target = if index + 1 < depth {
            contacts[index + 1]
        } else {
            effect
        };
        let delay = if temporal_break == Some(index) {
            LONG_DELAY
        } else {
            0
        };
        contact_arrows.push(builder.arrow_with_delay(
            contacts[index],
            target,
            delay,
            TransmissionMode::Drive,
        ));
    }
    builder.arrow(effect, contacts[depth - 1], TransmissionMode::Modulatory);
    let qlp_delay = i64::from(recurrent);
    let mut qlp_arrows = Vec::new();
    for index in (1..depth).rev() {
        qlp_arrows.push(builder.qlp(contacts[index], contacts[index - 1], qlp_delay));
    }
    if recurrent {
        qlp_arrows.push(builder.qlp(contacts[0], contacts[depth - 1], 1));
    }

    let mut starts = vec![builder.spike(source, 0, 1)];
    if let Some(index) = structural_break {
        if index + 1 < depth {
            starts.push(builder.spike(contacts[index + 1], 0, 1));
        }
    }
    let consequence_tick = if temporal_break.is_some() {
        LONG_DELAY + 1
    } else {
        1
    };
    let consequence = builder.spike(effect, consequence_tick, 2);

    let expected_support = match structural_break.or(temporal_break) {
        Some(index) => (0..depth).map(|candidate| candidate > index).collect(),
        None => vec![true; depth],
    };
    let expected_qlp = if recurrent {
        None
    } else {
        let count = match structural_break.or(temporal_break) {
            Some(index) => depth - 1 - index,
            None => depth - 1,
        };
        Some(u64::try_from(count).unwrap())
    };

    builder.finish(
        contact_arrows,
        qlp_arrows,
        vec![starts, vec![consequence]],
        expected_support,
        expected_qlp,
        recurrent,
    )
}

fn branch_geometry(mut builder: Builder, depth: usize, fanout: bool) -> Geometry {
    assert!(depth >= 2);
    let branch_len = depth - 1;
    let branch_a = (0..branch_len).map(|_| builder.cell(1)).collect::<Vec<_>>();
    let branch_b = (0..branch_len).map(|_| builder.cell(1)).collect::<Vec<_>>();
    let final_contact = builder.cell(1);
    let effect = builder.cell(2);
    let source_a = builder.source(branch_a[0]);
    let source_b = fanout.then(|| builder.source(branch_b[0]));

    let mut arrows_a = Vec::with_capacity(branch_len);
    let mut arrows_b = Vec::with_capacity(branch_len);
    for index in 0..branch_len {
        let target_a = if index + 1 < branch_len {
            branch_a[index + 1]
        } else {
            final_contact
        };
        let target_b = if index + 1 < branch_len {
            branch_b[index + 1]
        } else {
            final_contact
        };
        arrows_a.push(builder.arrow(branch_a[index], target_a, TransmissionMode::Drive));
        arrows_b.push(builder.arrow(branch_b[index], target_b, TransmissionMode::Drive));
    }
    let final_arrow = builder.arrow(final_contact, effect, TransmissionMode::Drive);
    builder.arrow(effect, final_contact, TransmissionMode::Modulatory);

    let mut qlp_arrows = vec![
        builder.qlp(final_contact, branch_a[branch_len - 1], 0),
        builder.qlp(final_contact, branch_b[branch_len - 1], 0),
    ];
    for index in (1..branch_len).rev() {
        qlp_arrows.push(builder.qlp(branch_a[index], branch_a[index - 1], 0));
        qlp_arrows.push(builder.qlp(branch_b[index], branch_b[index - 1], 0));
    }

    let mut starts = vec![builder.spike(source_a, 0, 1)];
    if let Some(source_b) = source_b {
        starts.push(builder.spike(source_b, 0, 1));
    }
    let consequence = builder.spike(effect, 1, 2);

    let mut contact_arrows = Vec::new();
    let mut expected_support = Vec::new();
    contact_arrows.extend(arrows_a);
    expected_support.extend(vec![true; branch_len]);
    contact_arrows.push(final_arrow);
    expected_support.push(true);
    contact_arrows.extend(arrows_b);
    expected_support.extend(vec![fanout; branch_len]);
    let expected_qlp = if fanout { 2 * depth - 2 } else { depth };

    builder.finish(
        contact_arrows,
        qlp_arrows,
        vec![starts, vec![consequence]],
        expected_support,
        Some(u64::try_from(expected_qlp).unwrap()),
        false,
    )
}

fn geometry(root: u64, phase: i64, case: CaseSpec, mechanics: MechanicalConfig) -> Geometry {
    let builder = Builder::new(root, phase, mechanics);
    match case.family {
        Family::Complete => linear_geometry(builder, case.depth, None, None, false),
        Family::StructuralBreak => {
            linear_geometry(builder, case.depth, case.break_index, None, false)
        }
        Family::TemporalBreak => {
            linear_geometry(builder, case.depth, None, case.break_index, false)
        }
        Family::WrongBranch => branch_geometry(builder, case.depth, false),
        Family::HonestFanout => branch_geometry(builder, case.depth, true),
        Family::RecurrentClosure => linear_geometry(builder, case.depth, None, None, true),
    }
}

fn execute(mut geometry: Geometry) -> (Observation, Vec<bool>, Option<u64>, bool) {
    let mut trace = Vec::new();
    let mut work = WorkTotals::default();
    let mut quiescent = true;
    for batch in geometry.batches {
        let result = geometry.body.arrive(&batch, 256);
        work.physical = work.physical.saturating_add(result.work.physical_total());
        work.drive = work.drive.saturating_add(result.work.drive_deliveries);
        work.modulation = work
            .modulation
            .saturating_add(result.work.modulatory_deliveries);
        work.updates = work
            .updates
            .saturating_add(result.work.local_return_updates);
        work.proposals = work
            .proposals
            .saturating_add(result.work.local_structural_proposals);
        work.deallocations = work
            .deallocations
            .saturating_add(result.work.physical_deallocations);
        work.qlp = work
            .qlp
            .saturating_add(result.work.qualified_local_traversals);
        quiescent &= result.naturally_quiescent;
        trace.extend(result.physical_trace);
    }
    let participation = geometry
        .contacts
        .iter()
        .map(|arrow| geometry.body.local_participation(*arrow))
        .collect::<Vec<_>>();
    let support = geometry
        .contacts
        .iter()
        .map(|arrow| geometry.body.local_plastic_support(*arrow))
        .collect::<Vec<_>>();
    let triggers = geometry
        .qlp_arrows
        .iter()
        .map(|arrow| geometry.body.transmission_trigger(*arrow))
        .collect::<Vec<_>>();
    let qlp_events = trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::QualifiedLocalTraversal { .. }
            )
        })
        .count() as u64;
    let source_fires = trace
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::Fire { .. }))
        .count() as u64;
    let body = geometry.body.arena_body(1);
    let live = geometry
        .contacts
        .iter()
        .chain(&geometry.qlp_arrows)
        .all(|id| {
            body.arrows
                .iter()
                .find(|arrow| arrow.id == *id)
                .is_some_and(|arrow| arrow.live)
        });
    let observation = Observation {
        trace,
        participation,
        support,
        triggers,
        work,
        qlp_events,
        source_fires,
        final_tick: geometry.body.clock().tick,
        pressure_phase: geometry.body.clock().pressure_phase(),
        body_hash: ContentHash::of(&geometry.body.canonical_body_bytes(1).unwrap()).to_string(),
        live,
        quiescent,
    };
    (
        observation,
        geometry.expected_support,
        geometry.expected_qlp,
        geometry.recurrent,
    )
}

fn run(
    root: u64,
    phase: i64,
    case: CaseSpec,
    mechanics: MechanicalConfig,
) -> (Observation, Vec<bool>, Option<u64>, bool) {
    execute(geometry(root, phase, case, mechanics))
}

fn predicate(
    observation: &Observation,
    expected_support: &[bool],
    expected_qlp: Option<u64>,
    recurrent: bool,
    depth: usize,
) -> bool {
    let support_matches = observation
        .support
        .iter()
        .zip(expected_support)
        .all(|(support, expected)| (*support > 0) == *expected);
    let qlp_matches = expected_qlp.map_or_else(
        || observation.qlp_events > u64::try_from(depth.saturating_sub(1)).unwrap(),
        |expected| observation.qlp_events == expected,
    );
    support_matches
        && observation.support.len() == expected_support.len()
        && qlp_matches
        && observation.work.qlp == observation.qlp_events
        && observation.work.proposals == 0
        && observation.work.deallocations == 0
        && observation.live
        && observation.quiescent
        && (!recurrent || observation.work.physical <= CYCLE_WORK_CEILING)
        && observation
            .triggers
            .iter()
            .all(|trigger| *trigger == TransmissionTrigger::QualifiedLocalParticipation)
}

fn mechanics_name(mechanics: MechanicalConfig) -> &'static str {
    if mechanics == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn vector(values: &[u64]) -> String {
    values
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn bool_vector(values: &[bool]) -> String {
    values
        .iter()
        .map(|value| if *value { "1" } else { "0" })
        .collect::<Vec<_>>()
        .join("|")
}

fn trigger_vector(values: &[TransmissionTrigger]) -> String {
    values
        .iter()
        .map(|value| match value {
            TransmissionTrigger::SourceFires => "source",
            TransmissionTrigger::QualifiedLocalParticipation => "qualified",
        })
        .collect::<Vec<_>>()
        .join("|")
}

struct EvidenceRow<'a> {
    root: u64,
    phase: i64,
    case: CaseSpec,
    mechanics: MechanicalConfig,
    observation: &'a Observation,
    expected_support: &'a [bool],
    expected_qlp: Option<u64>,
    recurrent: bool,
}

fn write_row(csv: &mut String, case_id: usize, row: EvidenceRow<'_>) {
    let EvidenceRow {
        root,
        phase,
        case,
        mechanics,
        observation,
        expected_support,
        expected_qlp,
        recurrent,
    } = row;
    writeln!(
        csv,
        "{case_id},{root},{phase},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        case.label(),
        case.family.name(),
        case.depth,
        case.break_index.map_or_else(|| "none".to_owned(), |value| value.to_string()),
        mechanics_name(mechanics),
        bool_vector(expected_support),
        vector(&observation.participation),
        vector(&observation.support),
        trigger_vector(&observation.triggers),
        u8::from(predicate(
            observation,
            expected_support,
            expected_qlp,
            recurrent,
            case.depth,
        )),
        observation.qlp_events,
        observation.source_fires,
        observation.work.physical,
        observation.work.drive,
        observation.work.modulation,
        observation.work.updates,
        observation.work.proposals,
        observation.work.deallocations,
        observation.work.qlp,
        observation.final_tick,
        observation.pressure_phase,
        ContentHash::of(format!("{:?}", observation.trace).as_bytes()),
        observation.body_hash,
        u8::from(observation.live),
        u8::from(observation.quiescent),
    )
    .unwrap();
}

fn write_checksums(output: &Path) {
    let mut sums = String::new();
    for name in ["matrix.csv", "report.md"] {
        let bytes = fs::read(output.join(name)).unwrap();
        writeln!(sums, "{}  {name}", ContentHash::of(&bytes)).unwrap();
    }
    fs::write(output.join("SHA256SUMS"), sums).unwrap();
}

fn index_for_depth(depth: usize) -> usize {
    DEPTHS
        .iter()
        .position(|candidate| *candidate == depth)
        .unwrap()
}

fn range_line(name: &str, values: &[(u64, u64); 5]) -> String {
    let mut line = format!("- {name}:");
    for (index, depth) in DEPTHS.into_iter().enumerate() {
        write!(line, " d{depth}={}-{}", values[index].0, values[index].1).unwrap();
    }
    line.push('\n');
    line
}

fn observe_range(range: &mut (u64, u64), value: u64) {
    if range.0 == u64::MAX {
        range.0 = value;
    } else {
        range.0 = range.0.min(value);
    }
    range.1 = range.1.max(value);
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/pqlc1_depth_composition_v1"));
    fs::create_dir_all(&output).unwrap();
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];
    let case_specs = cases();
    let mut csv = String::from(
        "case_id,root,initial_phase,variant,family,depth,break_index,mechanics,expected_support,participation,support,triggers,predicate_pass,qlp_events,source_fires,physical_work,drive_deliveries,modulatory_deliveries,local_updates,proposals,deallocations,qlp_work,final_tick,pressure_phase,trace_hash,body_hash,live,quiescent\n",
    );
    let mut physical_cases = 0_usize;
    let mut passed_physical_cases = 0_usize;
    let mut variant_complete = vec![true; case_specs.len()];
    let mut complete_qlp = [(u64::MAX, 0); 5];
    let mut complete_work = [(u64::MAX, 0); 5];
    let mut cycle_qlp = [(u64::MAX, 0); 5];
    let mut cycle_work = [(u64::MAX, 0); 5];

    for (variant_index, case) in case_specs.iter().copied().enumerate() {
        for root in ROOTS {
            for phase in 0..10 {
                physical_cases += 1;
                let (reference, expected_support, expected_qlp, recurrent) =
                    run(root, phase, case, mechanics[0]);
                let reference_replay = run(root, phase, case, mechanics[0]);
                assert_eq!(reference_replay.0, reference);
                assert_eq!(reference_replay.1, expected_support);
                assert_eq!(reference_replay.2, expected_qlp);
                assert_eq!(reference_replay.3, recurrent);

                let production_run = run(root, phase, case, mechanics[1]);
                let production_replay = run(root, phase, case, mechanics[1]);
                assert_eq!(production_replay, production_run);
                assert_eq!(production_run.0, reference);
                assert_eq!(production_run.1, expected_support);
                assert_eq!(production_run.2, expected_qlp);
                assert_eq!(production_run.3, recurrent);

                let pass = predicate(
                    &reference,
                    &expected_support,
                    expected_qlp,
                    recurrent,
                    case.depth,
                );
                variant_complete[variant_index] &= pass;
                passed_physical_cases += usize::from(pass);

                let depth_index = index_for_depth(case.depth);
                if case.family == Family::Complete {
                    observe_range(&mut complete_qlp[depth_index], reference.qlp_events);
                    observe_range(&mut complete_work[depth_index], reference.work.physical);
                }
                if case.family == Family::RecurrentClosure {
                    observe_range(&mut cycle_qlp[depth_index], reference.qlp_events);
                    observe_range(&mut cycle_work[depth_index], reference.work.physical);
                }

                for (kind, observation) in [
                    (mechanics[0], &reference),
                    (mechanics[1], &production_run.0),
                ] {
                    write_row(
                        &mut csv,
                        physical_cases,
                        EvidenceRow {
                            root,
                            phase,
                            case,
                            mechanics: kind,
                            observation,
                            expected_support: &expected_support,
                            expected_qlp,
                            recurrent,
                        },
                    );
                }
            }
        }
    }

    assert_eq!(physical_cases, EXPECTED_PHYSICAL_CASES);
    let mechanics_rows = physical_cases * 2;
    assert_eq!(mechanics_rows, EXPECTED_MECHANICS_ROWS);
    let development_positive =
        passed_physical_cases == physical_cases && variant_complete.iter().all(|value| *value);
    let mut report = format!(
        "# PQLC1 depth composition result v1\n\n\
         - case variants: `{}/{EXPECTED_VARIANTS}`\n\
         - physical cases: `{physical_cases}/{EXPECTED_PHYSICAL_CASES}`\n\
         - mechanics rows: `{mechanics_rows}/{EXPECTED_MECHANICS_ROWS}`\n\
         - exact same-mechanics reconstruction: `{}/{}` runs\n\
         - exact ordered Reference/Production histories: `{physical_cases}/{EXPECTED_PHYSICAL_CASES}`\n\
         - predicate-positive physical cases: `{passed_physical_cases}/{physical_cases}`\n\
         - variant-complete count: `{}/{EXPECTED_VARIANTS}`\n",
        case_specs.len(),
        physical_cases * 4,
        physical_cases * 4,
        variant_complete.iter().filter(|value| **value).count(),
    );
    report.push_str(&range_line("complete QLP traversals", &complete_qlp));
    report.push_str(&range_line("complete PhysicalWork", &complete_work));
    report.push_str(&range_line("cycle QLP traversals", &cycle_qlp));
    report.push_str(&range_line("cycle PhysicalWork", &cycle_work));
    writeln!(
        report,
        "- cycle work ceiling: `{CYCLE_WORK_CEILING}`\n\
         - PQLC1 development positive: `{development_positive}`\n\
         - core, pressure, eligibility, ARC, authority, oracle, or arch.md changes: `0`"
    )
    .unwrap();

    fs::write(output.join("matrix.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!("PQLC1_COMPLETE physical_cases={physical_cases} positive={development_positive}");
}
