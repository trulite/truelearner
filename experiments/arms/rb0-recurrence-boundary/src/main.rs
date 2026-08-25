#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, Core0Profile, MechanicalConfig,
    ObservedRun, PhysicalEvent, PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode,
    Work,
};

const ONE: i64 = 1_i64 << 32;
const RESISTANCE: u32 = 1_000_000;
const FIRST_CEILING: u64 = 256;
const CONTINUATION_CEILING: u64 = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum MaterialProfile {
    Rs1Style,
    CoreB,
}

impl MaterialProfile {
    const ALL: [Self; 2] = [Self::Rs1Style, Self::CoreB];

    fn name(self) -> &'static str {
        match self {
            Self::Rs1Style => "rs1_style",
            Self::CoreB => "core_b",
        }
    }

    fn core0(self) -> Core0Profile {
        match self {
            Self::Rs1Style => Core0Profile::A,
            Self::CoreB => Core0Profile::B,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Topology {
    Cycle,
    Chain,
}

impl Topology {
    fn name(self) -> &'static str {
        match self {
            Self::Cycle => "cycle",
            Self::Chain => "chain",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PhasePattern {
    Core0,
    Rs1,
}

impl PhasePattern {
    const ALL: [Self; 2] = [Self::Core0, Self::Rs1];

    fn name(self) -> &'static str {
        match self {
            Self::Core0 => "core0_phase",
            Self::Rs1 => "rs1_phase",
        }
    }

    fn negative_phase(self) -> i32 {
        match self {
            Self::Core0 => 0,
            Self::Rs1 => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Case {
    topology: Topology,
    phase: PhasePattern,
    excitation: i64,
    inhibition: i64,
    threshold: i32,
    delay_ab: i64,
    delay_ba: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ActivityClass {
    Quiescent,
    Periodic,
    PersistentNonperiodic,
    Growing,
}

impl ActivityClass {
    fn name(self) -> &'static str {
        match self {
            Self::Quiescent => "quiescent",
            Self::Periodic => "periodic",
            Self::PersistentNonperiodic => "persistent_nonperiodic",
            Self::Growing => "growing",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    updates: u64,
    proposals: u64,
    deallocations: u64,
    qlp: u64,
}

impl WorkTotals {
    fn add(&mut self, work: Work) {
        self.physical = self.physical.saturating_add(work.physical_total());
        self.drive = self.drive.saturating_add(work.drive_deliveries);
        self.modulation = self.modulation.saturating_add(work.modulatory_deliveries);
        self.updates = self.updates.saturating_add(work.local_return_updates);
        self.proposals = self
            .proposals
            .saturating_add(work.local_structural_proposals);
        self.deallocations = self
            .deallocations
            .saturating_add(work.physical_deallocations);
        self.qlp = self.qlp.saturating_add(work.qualified_local_traversals);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    class: ActivityClass,
    trace: Vec<PhysicalTransition>,
    continuation_trace: Vec<PhysicalTransition>,
    excitatory_fires: [u64; 2],
    inhibitor_fires: [u64; 2],
    positive_incidences: u64,
    relay_incidences: u64,
    negative_incidences: u64,
    first_firings: u64,
    continuation_firings: u64,
    period_firings: u64,
    period_ticks: i64,
    work: WorkTotals,
    scheduled_deliveries: u64,
    continuation_deliveries: u64,
    final_tick: i64,
    pending: usize,
    naturally_quiescent: bool,
    first_ceiling: bool,
    continuation_ceiling: bool,
    initial_couplings: Vec<i64>,
    final_couplings: Vec<i64>,
    initial_resistances: Vec<u64>,
    final_resistances: Vec<u64>,
    body_hash: String,
}

struct World {
    body: PlasticSubstrate,
    excitatory: [CellId; 2],
    inhibitors: [CellId; 2],
    arrows: Vec<ArrowId>,
    initial_couplings: Vec<i64>,
    initial_resistances: Vec<u64>,
}

fn q32(whole: i64, half: bool) -> i64 {
    whole
        .saturating_mul(ONE)
        .saturating_add(if half { ONE / 2 } else { 0 })
}

fn material_text(value: i64) -> String {
    let negative = value.is_negative();
    let magnitude = value.unsigned_abs();
    let one = u64::try_from(ONE).unwrap();
    let whole = magnitude / one;
    let remainder = magnitude % one;
    let prefix = if negative { "-" } else { "" };
    if remainder == 0 {
        format!("{prefix}{whole}")
    } else if remainder == one / 2 {
        format!("{prefix}{whole}.5")
    } else {
        format!("{prefix}{whole}+{remainder}/{one}")
    }
}

fn cases() -> BTreeMap<Case, BTreeSet<&'static str>> {
    let mut cases = BTreeMap::<Case, BTreeSet<&'static str>>::new();
    let mut add = |case: Case, section: &'static str| {
        cases.entry(case).or_default().insert(section);
    };
    let e_values = [
        q32(1, false),
        q32(1, true),
        q32(2, false),
        q32(2, true),
        q32(3, false),
    ];
    let h_values = [
        q32(0, false),
        q32(1, false),
        q32(1, true),
        q32(2, false),
        q32(2, true),
        q32(3, false),
        q32(4, false),
        q32(8, false),
        q32(16, false),
    ];
    for phase in PhasePattern::ALL {
        for excitation in e_values {
            for inhibition in h_values {
                add(
                    Case {
                        topology: Topology::Cycle,
                        phase,
                        excitation,
                        inhibition,
                        threshold: 2,
                        delay_ab: 1,
                        delay_ba: 1,
                    },
                    "efficacy_plane",
                );
            }
        }
        for (excitation, threshold) in [(q32(1, false), 1), (q32(2, false), 2), (q32(3, false), 3)]
        {
            for inhibition in h_values {
                add(
                    Case {
                        topology: Topology::Cycle,
                        phase,
                        excitation,
                        inhibition,
                        threshold,
                        delay_ab: 1,
                        delay_ba: 1,
                    },
                    "threshold_section",
                );
            }
        }
        for (delay_ab, delay_ba) in [(0, 1), (1, 1), (2, 2), (3, 3)] {
            for inhibition in [
                q32(2, false),
                q32(2, true),
                q32(3, false),
                q32(4, false),
                q32(16, false),
            ] {
                add(
                    Case {
                        topology: Topology::Cycle,
                        phase,
                        excitation: q32(2, false),
                        inhibition,
                        threshold: 2,
                        delay_ab,
                        delay_ba,
                    },
                    "delay_section",
                );
            }
        }
        for inhibition in [q32(2, false), q32(16, false)] {
            add(
                Case {
                    topology: Topology::Cycle,
                    phase,
                    excitation: q32(2, false),
                    inhibition,
                    threshold: 2,
                    delay_ab: 1,
                    delay_ba: 1,
                },
                "historical_pair",
            );
        }
        add(
            Case {
                topology: Topology::Chain,
                phase,
                excitation: q32(2, false),
                inhibition: q32(16, false),
                threshold: 2,
                delay_ab: 1,
                delay_ba: 1,
            },
            "acyclic_control",
        );
    }
    cases
}

fn add_cell(body: &mut PlasticSubstrate, physical: u64, position: i32, threshold: i32) -> CellId {
    body.add_cell(CellSpec {
        physical_id: physical,
        position,
        region: 0,
        threshold,
        resistance: RESISTANCE,
    })
}

fn add_material_arrow(
    body: &mut PlasticSubstrate,
    profile: MaterialProfile,
    from: CellId,
    to: CellId,
    material: i64,
    delay: i64,
    phase: i32,
) -> ArrowId {
    let observer = material / ONE;
    let id = body.add_arrow(ArrowSpec {
        from,
        to,
        delay,
        phase,
        coupling: i32::try_from(observer).expect("frozen efficacy fits i32"),
        resistance: RESISTANCE,
        mode: TransmissionMode::Drive,
    });
    if profile == MaterialProfile::CoreB {
        assert!(body.set_core0_coupling_material(id, material));
    }
    id
}

fn build_world(
    case_index: usize,
    case: Case,
    profile: MaterialProfile,
    mechanics: MechanicalConfig,
) -> World {
    let root = 9_100_000_u64.saturating_add(u64::try_from(case_index).unwrap() * 100);
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 16, 32, mechanics);
    body.set_core0_profile(profile.core0());
    body.set_physical_tracing(true);
    let a = add_cell(&mut body, root + 1, 0, case.threshold);
    let b = add_cell(&mut body, root + 2, 100, case.threshold);
    let ia = add_cell(&mut body, root + 3, 10_000, 1);
    let ib = add_cell(&mut body, root + 4, 20_000, 1);
    let mut arrows = Vec::new();
    arrows.push(add_material_arrow(
        &mut body,
        profile,
        a,
        b,
        case.excitation,
        case.delay_ab,
        0,
    ));
    if case.topology == Topology::Cycle {
        arrows.push(add_material_arrow(
            &mut body,
            profile,
            b,
            a,
            case.excitation,
            case.delay_ba,
            0,
        ));
    }
    for (source, inhibitor, target) in [(a, ia, a), (b, ib, b)] {
        arrows.push(add_material_arrow(
            &mut body, profile, source, inhibitor, ONE, 0, 0,
        ));
        arrows.push(add_material_arrow(
            &mut body,
            profile,
            inhibitor,
            target,
            -case.inhibition,
            0,
            case.phase.negative_phase(),
        ));
    }
    let initial_couplings = arrows
        .iter()
        .map(|id| body.core0_coupling_material(*id))
        .collect::<Vec<_>>();
    let initial_resistances = arrows
        .iter()
        .map(|id| body.core0_resistance_material(*id))
        .collect::<Vec<_>>();
    body.enter(SpikeInput {
        arrival_tick: 0,
        phase: 0,
        origin_physical: root + 90,
        target: a,
        impulse: case.threshold,
    });
    World {
        body,
        excitatory: [a, b],
        inhibitors: [ia, ib],
        arrows,
        initial_couplings,
        initial_resistances,
    }
}

fn fire_counts(trace: &[PhysicalTransition], cells: [CellId; 2]) -> [u64; 2] {
    let mut counts = [0_u64; 2];
    for transition in trace {
        if let PhysicalEvent::Fire { cell } = transition.event {
            if cell == cells[0] {
                counts[0] = counts[0].saturating_add(1);
            } else if cell == cells[1] {
                counts[1] = counts[1].saturating_add(1);
            }
        }
    }
    counts
}

fn firing_sequence(trace: &[PhysicalTransition], cells: [CellId; 2]) -> Vec<(usize, i64)> {
    trace
        .iter()
        .filter_map(|transition| {
            let PhysicalEvent::Fire { cell } = transition.event else {
                return None;
            };
            let index = cells.iter().position(|candidate| *candidate == cell)?;
            Some((index, transition.tick))
        })
        .collect()
}

fn periodicity(trace: &[PhysicalTransition], cells: [CellId; 2]) -> Option<(u64, i64)> {
    let sequence = firing_sequence(trace, cells);
    for period in 1..=8_usize {
        if sequence.len() < period * 3 {
            continue;
        }
        let tail = &sequence[sequence.len() - period * 3..];
        let a = &tail[..period];
        let b = &tail[period..period * 2];
        let c = &tail[period * 2..];
        let same_cells = a
            .iter()
            .zip(b)
            .zip(c)
            .all(|((left, middle), right)| left.0 == middle.0 && middle.0 == right.0);
        let ab = b[0].1.saturating_sub(a[0].1);
        let bc = c[0].1.saturating_sub(b[0].1);
        if same_cells && ab > 0 && ab == bc {
            return Some((u64::try_from(period).unwrap(), ab));
        }
    }
    None
}

fn incidence_counts(
    trace: &[PhysicalTransition],
    excitatory: [CellId; 2],
    inhibitors: [CellId; 2],
) -> (u64, u64, u64) {
    let mut positive = 0_u64;
    let mut relay = 0_u64;
    let mut negative = 0_u64;
    for transition in trace {
        let PhysicalEvent::MaterialDriveIncidence {
            target, impulse, ..
        } = transition.event
        else {
            continue;
        };
        if impulse < 0 {
            negative = negative.saturating_add(1);
        } else if inhibitors.contains(&target) {
            relay = relay.saturating_add(1);
        } else if excitatory.contains(&target) {
            positive = positive.saturating_add(1);
        }
    }
    (positive.saturating_sub(1), relay, negative)
}

fn classify(
    first: &ObservedRun,
    continuation: Option<&ObservedRun>,
    trace: &[PhysicalTransition],
    cells: [CellId; 2],
) -> (ActivityClass, u64, i64) {
    if first.run.naturally_quiescent
        || continuation.is_some_and(|next| next.run.naturally_quiescent)
    {
        return (ActivityClass::Quiescent, 0, 0);
    }
    if let Some((period_firings, period_ticks)) = periodicity(trace, cells) {
        return (ActivityClass::Periodic, period_firings, period_ticks);
    }
    let first_firings = firing_sequence(&first.run.physical_trace, cells).len();
    let continuation_firings = continuation
        .map(|next| firing_sequence(&next.run.physical_trace, cells).len())
        .unwrap_or(0);
    if continuation.is_some_and(|next| next.observation_ceiling_reached)
        && continuation_firings.saturating_mul(usize::try_from(FIRST_CEILING).unwrap())
            > first_firings.saturating_mul(usize::try_from(CONTINUATION_CEILING).unwrap())
    {
        (ActivityClass::Growing, 0, 0)
    } else {
        (ActivityClass::PersistentNonperiodic, 0, 0)
    }
}

fn run_case(
    case_index: usize,
    case: Case,
    profile: MaterialProfile,
    mechanics: MechanicalConfig,
) -> Observation {
    let mut world = build_world(case_index, case, profile, mechanics);
    let first = world.body.propagate_with_observation_ceiling(FIRST_CEILING);
    let mut work = WorkTotals::default();
    work.add(first.run.work);
    let continuation = if first.observation_ceiling_reached {
        let next = world
            .body
            .propagate_with_observation_ceiling(CONTINUATION_CEILING);
        work.add(next.run.work);
        Some(next)
    } else {
        None
    };
    let continuation_trace = continuation
        .as_ref()
        .map_or_else(Vec::new, |next| next.run.physical_trace.clone());
    let mut trace = first.run.physical_trace.clone();
    trace.extend(continuation_trace.clone());
    let (class, period_firings, period_ticks) =
        classify(&first, continuation.as_ref(), &trace, world.excitatory);
    let excitatory_fires = fire_counts(&trace, world.excitatory);
    let inhibitor_fires = fire_counts(&trace, world.inhibitors);
    let (positive_incidences, relay_incidences, negative_incidences) =
        incidence_counts(&trace, world.excitatory, world.inhibitors);
    let first_firings =
        u64::try_from(firing_sequence(&first.run.physical_trace, world.excitatory).len()).unwrap();
    let continuation_firings = continuation.as_ref().map_or(0, |next| {
        u64::try_from(firing_sequence(&next.run.physical_trace, world.excitatory).len()).unwrap()
    });
    let final_couplings = world
        .arrows
        .iter()
        .map(|id| world.body.core0_coupling_material(*id))
        .collect::<Vec<_>>();
    let final_resistances = world
        .arrows
        .iter()
        .map(|id| world.body.core0_resistance_material(*id))
        .collect::<Vec<_>>();
    let naturally_quiescent = continuation
        .as_ref()
        .map_or(first.run.naturally_quiescent, |next| {
            next.run.naturally_quiescent
        });
    let continuation_deliveries = continuation
        .as_ref()
        .map_or(0, |next| next.scheduled_deliveries);
    let continuation_ceiling = continuation
        .as_ref()
        .is_some_and(|next| next.observation_ceiling_reached);
    Observation {
        class,
        trace: first.run.physical_trace,
        continuation_trace,
        excitatory_fires,
        inhibitor_fires,
        positive_incidences,
        relay_incidences,
        negative_incidences,
        first_firings,
        continuation_firings,
        period_firings,
        period_ticks,
        work,
        scheduled_deliveries: first.scheduled_deliveries,
        continuation_deliveries,
        final_tick: world.body.clock().tick,
        pending: world.body.pending_physical_activity(),
        naturally_quiescent,
        first_ceiling: first.observation_ceiling_reached,
        continuation_ceiling,
        initial_couplings: world.initial_couplings,
        final_couplings,
        initial_resistances: world.initial_resistances,
        final_resistances,
        body_hash: ContentHash::of(&world.body.canonical_body_bytes(1).unwrap()).to_string(),
    }
}

fn trace_hash(trace: &[PhysicalTransition]) -> String {
    ContentHash::of(format!("{trace:?}").as_bytes()).to_string()
}

fn first_divergence(left: &Observation, right: &Observation) -> String {
    let left_trace = left
        .trace
        .iter()
        .chain(&left.continuation_trace)
        .collect::<Vec<_>>();
    let right_trace = right
        .trace
        .iter()
        .chain(&right.continuation_trace)
        .collect::<Vec<_>>();
    let shared = left_trace.len().min(right_trace.len());
    for index in 0..shared {
        if left_trace[index] != right_trace[index] {
            return format!(
                "transition {index}: left={:?} right={:?}",
                left_trace[index], right_trace[index]
            );
        }
    }
    if left_trace.len() != right_trace.len() {
        return format!(
            "trace length: left={} right={}",
            left_trace.len(),
            right_trace.len()
        );
    }
    if left.class != right.class {
        return format!("class: left={:?} right={:?}", left.class, right.class);
    }
    "none".to_string()
}

fn vector_text<T: ToString>(values: &[T]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn write_results(
    destination: &Path,
    cases: &BTreeMap<Case, BTreeSet<&'static str>>,
    observations: &BTreeMap<(usize, MaterialProfile), Observation>,
) {
    fs::create_dir_all(destination).expect("create RB0 result directory");
    let mut csv = String::from("case_id,sections,profile,topology,phase,excitation_q32,excitation,inhibition_q32,inhibition,threshold,delay_ab,delay_ba,class,excite_a_fires,excite_b_fires,inhibitor_a_fires,inhibitor_b_fires,positive_incidences,relay_incidences,negative_incidences,first_firings,continuation_firings,period_firings,period_ticks,scheduled_deliveries,continuation_deliveries,physical_work,final_tick,pending,naturally_quiescent,first_ceiling,continuation_ceiling,initial_couplings,final_couplings,initial_resistances,final_resistances,trace_hash,continuation_trace_hash,body_hash,replay_exact,mechanics_exact,integrity_pass\n");
    let mut comparison = String::from("case_id,topology,phase,excitation,inhibition,threshold,delay_ab,delay_ba,rs1_class,core_b_class,first_divergence\n");
    let mut report = String::from("# RB0 recurrence boundary result\n\n");
    let mut class_counts = BTreeMap::<(MaterialProfile, ActivityClass), usize>::new();
    let mut core_b_finite = false;
    let mut core_b_useful = false;
    let mut all_integrity = true;
    for (index, (case, sections)) in cases.iter().enumerate() {
        for profile in MaterialProfile::ALL {
            if profile == MaterialProfile::Rs1Style
                && (case.excitation % ONE != 0 || case.inhibition % ONE != 0)
            {
                continue;
            }
            let observation = &observations[&(index, profile)];
            *class_counts
                .entry((profile, observation.class))
                .or_default() += 1;
            if profile == MaterialProfile::CoreB
                && case.topology == Topology::Cycle
                && observation.class == ActivityClass::Quiescent
            {
                core_b_finite = true;
                core_b_useful |= observation.excitatory_fires == [1, 1];
            }
            let coupling_static = observation.initial_couplings == observation.final_couplings;
            let resistance_only_decays = observation
                .final_resistances
                .iter()
                .zip(&observation.initial_resistances)
                .all(|(after, before)| after <= before);
            let integrity = coupling_static
                && resistance_only_decays
                && observation.work.modulation == 0
                && observation.work.updates == 0
                && observation.work.proposals == 0
                && observation.work.deallocations == 0
                && observation.work.qlp == 0;
            all_integrity &= integrity;
            let row = vec![
                (index + 1).to_string(),
                sections.iter().copied().collect::<Vec<_>>().join("|"),
                profile.name().to_string(),
                case.topology.name().to_string(),
                case.phase.name().to_string(),
                case.excitation.to_string(),
                material_text(case.excitation),
                case.inhibition.to_string(),
                material_text(case.inhibition),
                case.threshold.to_string(),
                case.delay_ab.to_string(),
                case.delay_ba.to_string(),
                observation.class.name().to_string(),
                observation.excitatory_fires[0].to_string(),
                observation.excitatory_fires[1].to_string(),
                observation.inhibitor_fires[0].to_string(),
                observation.inhibitor_fires[1].to_string(),
                observation.positive_incidences.to_string(),
                observation.relay_incidences.to_string(),
                observation.negative_incidences.to_string(),
                observation.first_firings.to_string(),
                observation.continuation_firings.to_string(),
                observation.period_firings.to_string(),
                observation.period_ticks.to_string(),
                observation.scheduled_deliveries.to_string(),
                observation.continuation_deliveries.to_string(),
                observation.work.physical.to_string(),
                observation.final_tick.to_string(),
                observation.pending.to_string(),
                observation.naturally_quiescent.to_string(),
                observation.first_ceiling.to_string(),
                observation.continuation_ceiling.to_string(),
                vector_text(&observation.initial_couplings),
                vector_text(&observation.final_couplings),
                vector_text(&observation.initial_resistances),
                vector_text(&observation.final_resistances),
                trace_hash(&observation.trace),
                trace_hash(&observation.continuation_trace),
                observation.body_hash.clone(),
                "true".to_string(),
                "true".to_string(),
                integrity.to_string(),
            ];
            writeln!(csv, "{}", row.join(",")).unwrap();
        }
        if case.excitation % ONE == 0 && case.inhibition % ONE == 0 {
            let left = &observations[&(index, MaterialProfile::Rs1Style)];
            let right = &observations[&(index, MaterialProfile::CoreB)];
            writeln!(
                comparison,
                "{},{},{},{},{},{},{},{},{},{},{}",
                index + 1,
                case.topology.name(),
                case.phase.name(),
                material_text(case.excitation),
                material_text(case.inhibition),
                case.threshold,
                case.delay_ab,
                case.delay_ba,
                left.class.name(),
                right.class.name(),
                first_divergence(left, right).replace(',', ";"),
            )
            .unwrap();
        }
    }
    report.push_str("## Classification counts\n\n| Profile | Quiescent | Periodic | Persistent nonperiodic | Growing |\n|---|---:|---:|---:|---:|\n");
    for profile in MaterialProfile::ALL {
        writeln!(
            report,
            "| {} | {} | {} | {} | {} |",
            profile.name(),
            class_counts
                .get(&(profile, ActivityClass::Quiescent))
                .copied()
                .unwrap_or(0),
            class_counts
                .get(&(profile, ActivityClass::Periodic))
                .copied()
                .unwrap_or(0),
            class_counts
                .get(&(profile, ActivityClass::PersistentNonperiodic))
                .copied()
                .unwrap_or(0),
            class_counts
                .get(&(profile, ActivityClass::Growing))
                .copied()
                .unwrap_or(0),
        )
        .unwrap();
    }
    writeln!(
        report,
        "\nCORE-B finite quiescent region: `{core_b_finite}`  \nCORE-B useful first traversal inside that region: `{core_b_useful}`  \nAll static/inert controls: `{all_integrity}`"
    )
    .unwrap();
    report.push_str("\n## E=2, T=2, delay 1+1 boundary\n\n| Phase | H | RS1-style | CORE-B | CORE-B A/B fires |\n|---|---:|---|---|---|\n");
    for phase in PhasePattern::ALL {
        for inhibition in [
            q32(0, false),
            q32(1, false),
            q32(1, true),
            q32(2, false),
            q32(2, true),
            q32(3, false),
            q32(4, false),
            q32(8, false),
            q32(16, false),
        ] {
            let case = Case {
                topology: Topology::Cycle,
                phase,
                excitation: q32(2, false),
                inhibition,
                threshold: 2,
                delay_ab: 1,
                delay_ba: 1,
            };
            let index = cases
                .keys()
                .position(|candidate| *candidate == case)
                .unwrap();
            let core_b = &observations[&(index, MaterialProfile::CoreB)];
            let legacy = if inhibition % ONE == 0 {
                observations[&(index, MaterialProfile::Rs1Style)]
                    .class
                    .name()
            } else {
                "n/a"
            };
            writeln!(
                report,
                "| {} | {} | {} | {} | {}|{} |",
                phase.name(),
                material_text(inhibition),
                legacy,
                core_b.class.name(),
                core_b.excitatory_fires[0],
                core_b.excitatory_fires[1],
            )
            .unwrap();
        }
    }
    fs::write(destination.join("matrix.csv"), csv).expect("write RB0 matrix");
    fs::write(destination.join("profile_comparison.csv"), comparison)
        .expect("write RB0 comparison");
    fs::write(destination.join("report.md"), report).expect("write RB0 report");
}

fn main() {
    let destination = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/rb0_recurrence_boundary_v1"));
    let cases = cases();
    let mut observations = BTreeMap::<(usize, MaterialProfile), Observation>::new();
    let mut all_replay = true;
    let mut all_mechanics = true;
    eprintln!("RB0_RECURRENCE_BOUNDARY_V1_EVIDENCE_SPENT");
    for (index, case) in cases.keys().copied().enumerate() {
        for profile in MaterialProfile::ALL {
            if profile == MaterialProfile::Rs1Style
                && (case.excitation % ONE != 0 || case.inhibition % ONE != 0)
            {
                continue;
            }
            let reference = run_case(index, case, profile, MechanicalConfig::REFERENCE);
            let replay = run_case(index, case, profile, MechanicalConfig::REFERENCE);
            let production = run_case(index, case, profile, MechanicalConfig::PRODUCTION);
            all_replay &= reference == replay;
            all_mechanics &= reference == production;
            observations.insert((index, profile), reference);
        }
    }
    assert!(all_replay, "RB0 exact replay mismatch");
    assert!(all_mechanics, "RB0 Reference/Production mismatch");
    let core0_h0 = Case {
        topology: Topology::Cycle,
        phase: PhasePattern::Core0,
        excitation: q32(2, false),
        inhibition: q32(0, false),
        threshold: 2,
        delay_ab: 1,
        delay_ba: 1,
    };
    let rs1_h16 = Case {
        inhibition: q32(16, false),
        phase: PhasePattern::Rs1,
        ..core0_h0
    };
    let h0_index = cases
        .keys()
        .position(|candidate| *candidate == core0_h0)
        .unwrap();
    let h16_index = cases
        .keys()
        .position(|candidate| *candidate == rs1_h16)
        .unwrap();
    assert_ne!(
        observations[&(h0_index, MaterialProfile::CoreB)].class,
        ActivityClass::Quiescent,
        "uninhibited executable recurrence must remain active"
    );
    assert_eq!(
        observations[&(h16_index, MaterialProfile::Rs1Style)].class,
        ActivityClass::Quiescent,
        "RS1-style H16 control must settle"
    );
    for (index, case) in cases.keys().copied().enumerate() {
        if case.topology != Topology::Chain {
            continue;
        }
        for profile in MaterialProfile::ALL {
            let observation = &observations[&(index, profile)];
            assert_eq!(observation.class, ActivityClass::Quiescent);
            assert_eq!(observation.excitatory_fires, [1, 1]);
        }
    }
    write_results(&destination, &cases, &observations);
    println!(
        "RB0_COMPLETE cases={} profile_rows={} replay_exact={} mechanics_exact={}",
        cases.len(),
        observations.len(),
        all_replay,
        all_mechanics
    );
}
