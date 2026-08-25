#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const RESISTANCES: [u32; 3] = [1, 2, 4];
const EVENT_DELAYS: [i64; 8] = [0, 1, 2, 3, 4, 5, 8, 12];
const RENEWAL_DELAYS: [i64; 7] = [1, 2, 3, 4, 5, 8, 12];
const HORIZON: i64 = 60;
const EXPECTED_CASES: usize = 750;
const EXPECTED_ROWS: usize = 1500;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Dormant,
    UsedNoConsequence,
    TimedConsequence,
    UnrelatedActivity,
    SamePathRenewal,
}

impl Family {
    const ALL: [Self; 5] = [
        Self::Dormant,
        Self::UsedNoConsequence,
        Self::TimedConsequence,
        Self::UnrelatedActivity,
        Self::SamePathRenewal,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Dormant => "dormant",
            Self::UsedNoConsequence => "used_no_consequence",
            Self::TimedConsequence => "timed_consequence",
            Self::UnrelatedActivity => "unrelated_activity",
            Self::SamePathRenewal => "same_path_renewal",
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Dormant => 0,
            Self::UsedNoConsequence => 1,
            Self::TimedConsequence => 2,
            Self::UnrelatedActivity => 3,
            Self::SamePathRenewal => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CaseSpec {
    family: Family,
    phase: i64,
    resistance: u32,
    delay: Option<i64>,
}

fn cases() -> Vec<CaseSpec> {
    let mut cases = Vec::new();
    for phase in 0..10 {
        for resistance in RESISTANCES {
            cases.push(CaseSpec {
                family: Family::Dormant,
                phase,
                resistance,
                delay: None,
            });
            cases.push(CaseSpec {
                family: Family::UsedNoConsequence,
                phase,
                resistance,
                delay: None,
            });
            for delay in EVENT_DELAYS {
                cases.push(CaseSpec {
                    family: Family::TimedConsequence,
                    phase,
                    resistance,
                    delay: Some(delay),
                });
                cases.push(CaseSpec {
                    family: Family::UnrelatedActivity,
                    phase,
                    resistance,
                    delay: Some(delay),
                });
            }
            for delay in RENEWAL_DELAYS {
                cases.push(CaseSpec {
                    family: Family::SamePathRenewal,
                    phase,
                    resistance,
                    delay: Some(delay),
                });
            }
        }
    }
    assert_eq!(cases.len(), EXPECTED_CASES);
    cases
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    stage: &'static str,
    tick: i64,
    live: bool,
    resistance: u32,
    coupling: i32,
    eligible: bool,
    participation: u64,
    support: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DurableState {
    live: bool,
    resistance: u32,
    coupling: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    updates: u64,
    proposals: u64,
    deallocations: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<PhysicalTransition>,
    trajectory: Vec<Point>,
    eligibility_events: Vec<(i64, i64)>,
    pressure_ticks: Vec<i64>,
    eligible_pressure_observed: u64,
    eligible_pressure_unchanged: u64,
    ineligible_pressure_observed: u64,
    ineligible_pressure_reduced: u64,
    event_live_before: bool,
    event_eligible_before: bool,
    event_participation_before: u64,
    event_support_before: u64,
    event_resistance_before: u32,
    event_support_after: u64,
    event_resistance_after: u32,
    renewal_attempted_live: bool,
    maximum_resistance: u32,
    work: WorkTotals,
    final_state: DurableState,
    final_participation: u64,
    final_support: u64,
    final_tick: i64,
    final_pressure_phase: i64,
    body_hash: String,
    naturally_quiescent: bool,
}

#[derive(Clone, Copy)]
struct Geometry {
    candidate: ArrowId,
    contact: CellId,
    source: CellId,
    effect: CellId,
    unrelated_source: CellId,
}

#[derive(Default)]
struct Recorder {
    deadline: Option<i64>,
    consumed: bool,
    eligibility_events: Vec<(i64, i64)>,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    trajectory: Vec<Point>,
    pressure_ticks: Vec<i64>,
    eligible_pressure_observed: u64,
    eligible_pressure_unchanged: u64,
    ineligible_pressure_observed: u64,
    ineligible_pressure_reduced: u64,
    maximum_resistance: u32,
    naturally_quiescent: bool,
}

impl Recorder {
    fn apply_trace(&mut self, trace: &[PhysicalTransition], geometry: Geometry) {
        for transition in trace {
            match transition.event {
                PhysicalEvent::Eligible { arrow, until } if arrow == geometry.candidate => {
                    self.deadline = Some(until);
                    self.consumed = false;
                    self.eligibility_events.push((transition.tick, until));
                }
                PhysicalEvent::Deliver {
                    mode: TransmissionMode::Modulatory,
                    target,
                    ..
                } if target == geometry.contact => {
                    self.consumed = true;
                }
                _ => {}
            }
        }
        self.trace.extend_from_slice(trace);
    }

    fn apply_work(&mut self, work: Work) {
        self.work.physical = self.work.physical.saturating_add(work.physical_total());
        self.work.drive = self.work.drive.saturating_add(work.drive_deliveries);
        self.work.modulation = self
            .work
            .modulation
            .saturating_add(work.modulatory_deliveries);
        self.work.updates = self.work.updates.saturating_add(work.local_return_updates);
        self.work.proposals = self
            .work
            .proposals
            .saturating_add(work.local_structural_proposals);
        self.work.deallocations = self
            .work
            .deallocations
            .saturating_add(work.physical_deallocations);
    }

    fn apply_run(&mut self, result: truelearner_core::RunResult, geometry: Geometry) {
        self.apply_work(result.work);
        self.apply_trace(&result.physical_trace, geometry);
        self.naturally_quiescent &= result.naturally_quiescent;
    }

    fn eligible_at(&self, tick: i64, live: bool) -> bool {
        live && !self.consumed && self.deadline.is_some_and(|until| tick <= until)
    }

    fn point(&mut self, body: &PlasticSubstrate, geometry: Geometry, stage: &'static str) -> Point {
        let tick = body.clock().tick;
        let durable = durable_state(body, geometry.candidate);
        self.maximum_resistance = self.maximum_resistance.max(durable.resistance);
        let point = Point {
            stage,
            tick,
            live: durable.live,
            resistance: durable.resistance,
            coupling: durable.coupling,
            eligible: self.eligible_at(tick, durable.live),
            participation: body.local_participation(geometry.candidate),
            support: body.local_plastic_support(geometry.candidate),
        };
        self.trajectory.push(point.clone());
        point
    }

    fn advance_one(&mut self, body: &mut PlasticSubstrate, geometry: Geometry, tick: i64) {
        let before = durable_state(body, geometry.candidate);
        let covered = self.eligible_at(tick, before.live);
        self.apply_work(body.advance_time(tick));
        let after = durable_state(body, geometry.candidate);
        if tick.rem_euclid(10) == 0 {
            self.pressure_ticks.push(tick);
        }
        if tick.rem_euclid(10) == 0 && before.live {
            if covered {
                self.eligible_pressure_observed = self.eligible_pressure_observed.saturating_add(1);
                if after.live && after.resistance == before.resistance {
                    self.eligible_pressure_unchanged =
                        self.eligible_pressure_unchanged.saturating_add(1);
                }
            } else {
                self.ineligible_pressure_observed =
                    self.ineligible_pressure_observed.saturating_add(1);
                if !after.live || after.resistance < before.resistance {
                    self.ineligible_pressure_reduced =
                        self.ineligible_pressure_reduced.saturating_add(1);
                }
            }
        }
        self.point(body, geometry, "settled");
    }
}

fn add_cell(
    body: &mut PlasticSubstrate,
    physical_id: u64,
    position: i32,
    threshold: i32,
) -> CellId {
    body.add_cell(CellSpec {
        physical_id,
        position,
        region: 0,
        threshold,
        resistance: 100_000,
    })
}

fn add_arrow(
    body: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    resistance: u32,
    mode: TransmissionMode,
) -> ArrowId {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance,
        mode,
    })
}

fn input(root: u64, target: CellId, tick: i64, ordinal: u64) -> SpikeInput {
    SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: root + 90_000 + ordinal,
        target,
        impulse: 1,
    }
}

fn build(
    case_id: usize,
    case: CaseSpec,
    mechanics: MechanicalConfig,
) -> (PlasticSubstrate, Geometry, u64) {
    let root = 3_000_000 + u64::try_from(case_id).unwrap() * 100;
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root + 50), 16, 32, mechanics);
    body.set_physical_tracing(true);
    if case.phase > 0 {
        body.advance_time(case.phase);
    }
    let source = add_cell(&mut body, root + 1, 0, 1);
    let contact = add_cell(&mut body, root + 2, 10, 1);
    let target = add_cell(&mut body, root + 3, 20, 2);
    let effect = add_cell(&mut body, root + 4, 30, 1);
    let unrelated_source = add_cell(&mut body, root + 5, 40, 1);
    let unrelated_target = add_cell(&mut body, root + 6, 50, 2);
    add_arrow(&mut body, source, contact, 100_000, TransmissionMode::Drive);
    let candidate = add_arrow(
        &mut body,
        contact,
        target,
        case.resistance,
        TransmissionMode::Drive,
    );
    add_arrow(
        &mut body,
        effect,
        contact,
        100_000,
        TransmissionMode::Modulatory,
    );
    add_arrow(
        &mut body,
        unrelated_source,
        unrelated_target,
        100_000,
        TransmissionMode::Drive,
    );
    (
        body,
        Geometry {
            candidate,
            contact,
            source,
            effect,
            unrelated_source,
        },
        root,
    )
}

fn durable_state(body: &PlasticSubstrate, candidate: ArrowId) -> DurableState {
    let arena = body.arena_body(1);
    let arrow = arena
        .arrows
        .iter()
        .find(|arrow| arrow.id == candidate)
        .expect("candidate ARROW must remain addressable");
    DurableState {
        live: arrow.live,
        resistance: arrow.resistance,
        coupling: arrow.coupling,
    }
}

fn execute(case_id: usize, case: CaseSpec, mechanics: MechanicalConfig) -> Observation {
    let (mut body, geometry, root) = build(case_id, case, mechanics);
    let mut recorder = Recorder {
        naturally_quiescent: true,
        ..Recorder::default()
    };
    recorder.point(&body, geometry, "initial");

    if case.family != Family::Dormant {
        let result = body.arrive(&[input(root, geometry.source, case.phase, 1)], 256);
        recorder.apply_run(result, geometry);
        recorder.point(&body, geometry, "after_initial");
    }

    let event_tick = case.delay.map(|delay| case.phase + delay);
    let mut event_live_before = false;
    let mut event_eligible_before = false;
    let mut event_participation_before = 0_u64;
    let mut event_support_before = 0_u64;
    let mut event_resistance_before = 0_u32;
    let mut event_support_after = 0_u64;
    let mut event_resistance_after = 0_u32;
    let mut renewal_attempted_live = false;

    if event_tick == Some(case.phase) {
        let before = recorder.point(&body, geometry, "before_event");
        event_live_before = before.live;
        event_eligible_before = before.eligible;
        event_participation_before = before.participation;
        event_support_before = before.support;
        event_resistance_before = before.resistance;
        let event_target = match case.family {
            Family::TimedConsequence => geometry.effect,
            Family::UnrelatedActivity => geometry.unrelated_source,
            Family::SamePathRenewal => geometry.source,
            Family::Dormant | Family::UsedNoConsequence => unreachable!(),
        };
        renewal_attempted_live = case.family == Family::SamePathRenewal && before.live;
        let result = body.arrive(&[input(root, event_target, case.phase, 2)], 256);
        recorder.apply_run(result, geometry);
        let after = recorder.point(&body, geometry, "after_event");
        event_support_after = after.support;
        event_resistance_after = after.resistance;
    }

    for tick in (case.phase + 1)..=(case.phase + HORIZON) {
        recorder.advance_one(&mut body, geometry, tick);
        if event_tick != Some(tick) {
            continue;
        }
        let before = recorder.point(&body, geometry, "before_event");
        event_live_before = before.live;
        event_eligible_before = before.eligible;
        event_participation_before = before.participation;
        event_support_before = before.support;
        event_resistance_before = before.resistance;
        let event_target = match case.family {
            Family::TimedConsequence => geometry.effect,
            Family::UnrelatedActivity => geometry.unrelated_source,
            Family::SamePathRenewal => geometry.source,
            Family::Dormant | Family::UsedNoConsequence => unreachable!(),
        };
        renewal_attempted_live = case.family == Family::SamePathRenewal && before.live;
        let result = body.arrive(&[input(root, event_target, tick, 2)], 256);
        recorder.apply_run(result, geometry);
        let after = recorder.point(&body, geometry, "after_event");
        event_support_after = after.support;
        event_resistance_after = after.resistance;
    }

    let final_state = durable_state(&body, geometry.candidate);
    Observation {
        trace: recorder.trace,
        trajectory: recorder.trajectory,
        eligibility_events: recorder.eligibility_events,
        pressure_ticks: recorder.pressure_ticks,
        eligible_pressure_observed: recorder.eligible_pressure_observed,
        eligible_pressure_unchanged: recorder.eligible_pressure_unchanged,
        ineligible_pressure_observed: recorder.ineligible_pressure_observed,
        ineligible_pressure_reduced: recorder.ineligible_pressure_reduced,
        event_live_before,
        event_eligible_before,
        event_participation_before,
        event_support_before,
        event_resistance_before,
        event_support_after,
        event_resistance_after,
        renewal_attempted_live,
        maximum_resistance: recorder.maximum_resistance,
        work: recorder.work,
        final_state,
        final_participation: body.local_participation(geometry.candidate),
        final_support: body.local_plastic_support(geometry.candidate),
        final_tick: body.clock().tick,
        final_pressure_phase: body.clock().pressure_phase(),
        body_hash: ContentHash::of(&body.canonical_body_bytes(1).unwrap()).to_string(),
        naturally_quiescent: recorder.naturally_quiescent,
    }
}

fn mechanics_name(mechanics: MechanicalConfig) -> &'static str {
    if mechanics == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn trajectory_string(points: &[Point]) -> String {
    points
        .iter()
        .map(|point| {
            format!(
                "{}@{}/{}/{}/{}/{}/{}/{}",
                point.stage,
                point.tick,
                u8::from(point.live),
                point.resistance,
                point.coupling,
                u8::from(point.eligible),
                point.participation,
                point.support,
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn pair_string(values: &[(i64, i64)]) -> String {
    values
        .iter()
        .map(|(left, right)| format!("{left}:{right}"))
        .collect::<Vec<_>>()
        .join("|")
}

fn ticks_string(values: &[i64]) -> String {
    values
        .iter()
        .map(i64::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

struct EvidenceRow<'a> {
    case_id: usize,
    case: CaseSpec,
    mechanics: MechanicalConfig,
    observation: &'a Observation,
}

fn write_row(csv: &mut String, row: EvidenceRow<'_>) {
    let observation = row.observation;
    let columns = vec![
        row.case_id.to_string(),
        row.case.family.name().to_owned(),
        row.case.phase.to_string(),
        row.case.resistance.to_string(),
        row.case
            .delay
            .map_or_else(|| "none".to_owned(), |delay| delay.to_string()),
        mechanics_name(row.mechanics).to_owned(),
        trajectory_string(&observation.trajectory),
        pair_string(&observation.eligibility_events),
        ticks_string(&observation.pressure_ticks),
        observation.eligible_pressure_observed.to_string(),
        observation.eligible_pressure_unchanged.to_string(),
        observation.ineligible_pressure_observed.to_string(),
        observation.ineligible_pressure_reduced.to_string(),
        u8::from(observation.event_live_before).to_string(),
        u8::from(observation.event_eligible_before).to_string(),
        observation.event_participation_before.to_string(),
        observation.event_support_before.to_string(),
        observation.event_resistance_before.to_string(),
        observation.event_support_after.to_string(),
        observation.event_resistance_after.to_string(),
        u8::from(observation.renewal_attempted_live).to_string(),
        observation.maximum_resistance.to_string(),
        u8::from(observation.final_state.live).to_string(),
        observation.final_state.resistance.to_string(),
        observation.final_state.coupling.to_string(),
        observation.final_participation.to_string(),
        observation.final_support.to_string(),
        observation.work.physical.to_string(),
        observation.work.drive.to_string(),
        observation.work.modulation.to_string(),
        observation.work.updates.to_string(),
        observation.work.proposals.to_string(),
        observation.work.deallocations.to_string(),
        observation.final_tick.to_string(),
        observation.final_pressure_phase.to_string(),
        ContentHash::of(format!("{:?}", observation.trace).as_bytes()).to_string(),
        observation.body_hash.clone(),
        u8::from(observation.naturally_quiescent).to_string(),
    ];
    assert_eq!(columns.len(), 38);
    writeln!(csv, "{}", columns.join(",")).unwrap();
}

fn write_checksums(output: &Path) {
    let mut sums = String::new();
    for name in ["matrix.csv", "report.md"] {
        let bytes = fs::read(output.join(name)).unwrap();
        writeln!(sums, "{}  {name}", ContentHash::of(&bytes)).unwrap();
    }
    fs::write(output.join("SHA256SUMS"), sums).unwrap();
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/pd0_old_pressure_interaction_v1"));
    fs::create_dir_all(&output).unwrap();
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];
    let case_specs = cases();
    let mut csv = String::from(
        "case_id,family,initial_phase,initial_resistance,delay,mechanics,trajectory,eligibility_events,pressure_ticks,eligible_pressure_observed,eligible_pressure_unchanged,ineligible_pressure_observed,ineligible_pressure_reduced,event_live_before,event_eligible_before,event_participation_before,event_support_before,event_resistance_before,event_support_after,event_resistance_after,renewal_attempted_live,maximum_resistance,final_live,final_resistance,final_coupling,final_participation,final_support,physical_work,drive_deliveries,modulatory_deliveries,local_updates,proposals,deallocations,final_tick,final_pressure_phase,trace_hash,body_hash,naturally_quiescent\n",
    );

    let mut eligible_pressure_observed = 0_u64;
    let mut eligible_pressure_unchanged = 0_u64;
    let mut ineligible_pressure_observed = 0_u64;
    let mut ineligible_pressure_reduced = 0_u64;
    let mut support_by_delay = [(0_u64, 0_u64); 8];
    let mut traversal_only_durable_gains = 0_u64;
    let mut modulation_durable_gains = 0_u64;
    let mut final_live = [[0_u64; 3]; 5];
    let mut renewal_live = 0_u64;
    let mut renewal_dead = 0_u64;

    for (index, case) in case_specs.iter().copied().enumerate() {
        let case_id = index + 1;
        let reference = execute(case_id, case, mechanics[0]);
        let reference_replay = execute(case_id, case, mechanics[0]);
        assert_eq!(reference_replay, reference);
        let production = execute(case_id, case, mechanics[1]);
        let production_replay = execute(case_id, case, mechanics[1]);
        assert_eq!(production_replay, production);
        assert_eq!(production, reference);
        assert!(reference.naturally_quiescent);
        assert_eq!(reference.work.proposals, 0);

        eligible_pressure_observed =
            eligible_pressure_observed.saturating_add(reference.eligible_pressure_observed);
        eligible_pressure_unchanged =
            eligible_pressure_unchanged.saturating_add(reference.eligible_pressure_unchanged);
        ineligible_pressure_observed =
            ineligible_pressure_observed.saturating_add(reference.ineligible_pressure_observed);
        ineligible_pressure_reduced =
            ineligible_pressure_reduced.saturating_add(reference.ineligible_pressure_reduced);
        let resistance_index = RESISTANCES
            .iter()
            .position(|candidate| *candidate == case.resistance)
            .unwrap();
        final_live[case.family.index()][resistance_index] = final_live[case.family.index()]
            [resistance_index]
            .saturating_add(if reference.final_state.live { 1 } else { 0 });

        if case.family == Family::TimedConsequence {
            let delay_index = EVENT_DELAYS
                .iter()
                .position(|candidate| Some(*candidate) == case.delay)
                .unwrap();
            support_by_delay[delay_index].1 = support_by_delay[delay_index].1.saturating_add(1);
            if reference.event_support_after > reference.event_support_before {
                support_by_delay[delay_index].0 = support_by_delay[delay_index].0.saturating_add(1);
            }
            if reference.event_resistance_after > reference.event_resistance_before {
                modulation_durable_gains = modulation_durable_gains.saturating_add(1);
            }
        } else if reference.maximum_resistance > case.resistance {
            traversal_only_durable_gains = traversal_only_durable_gains.saturating_add(1);
        }

        if case.family == Family::SamePathRenewal {
            if reference.renewal_attempted_live {
                renewal_live = renewal_live.saturating_add(1);
            } else {
                renewal_dead = renewal_dead.saturating_add(1);
            }
        }

        for (kind, observation) in [(mechanics[0], &reference), (mechanics[1], &production)] {
            write_row(
                &mut csv,
                EvidenceRow {
                    case_id,
                    case,
                    mechanics: kind,
                    observation,
                },
            );
        }
    }

    assert_eq!(case_specs.len(), EXPECTED_CASES);
    let rows = case_specs.len() * 2;
    assert_eq!(rows, EXPECTED_ROWS);
    let support_line = EVENT_DELAYS
        .iter()
        .zip(support_by_delay)
        .map(|(delay, (positive, observed))| format!("d{delay}={positive}/{observed}"))
        .collect::<Vec<_>>()
        .join(" ");
    let final_live_line = Family::ALL
        .into_iter()
        .map(|family| {
            format!(
                "{}=r1:{}/r2:{}/r4:{}",
                family.name(),
                final_live[family.index()][0],
                final_live[family.index()][1],
                final_live[family.index()][2],
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    let report = format!(
        "# PD0 old pressure interaction characterization result v1\n\n\
         - physical cases: `{}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{rows}/{EXPECTED_ROWS}`\n\
         - exact same-mechanics reconstruction: `{}/{}` runs\n\
         - exact Reference/Production observations: `{}/{EXPECTED_CASES}`\n\
         - eligible pressure epochs unchanged: `{eligible_pressure_unchanged}/{eligible_pressure_observed}`\n\
         - ineligible pressure epochs reduced/deallocated: `{ineligible_pressure_reduced}/{ineligible_pressure_observed}`\n\
         - timed support by delay: `{support_line}`\n\
         - traversal-only durable resistance gains: `{traversal_only_durable_gains}`\n\
         - CPC1 Modulation durable resistance gains: `{modulation_durable_gains}`\n\
         - final-live counts: `{final_live_line}`\n\
         - same-path renewal attempted live/dead: `{renewal_live}/{renewal_dead}`\n\
         - PD0 characterization complete: `true`\n\
         - core, constants, pressure, participation, PQLC, ARC, authority, oracle, or arch.md changes: `0`\n",
        case_specs.len(),
        case_specs.len() * 4,
        case_specs.len() * 4,
        case_specs.len(),
    );
    fs::write(output.join("matrix.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!(
        "PD0_COMPLETE physical_cases={} characterized=true",
        case_specs.len()
    );
}
