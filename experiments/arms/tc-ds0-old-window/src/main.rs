#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, ContentHash, MechanicalConfig, PhysicalEvent, PhysicalTransition,
    PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const DELAYS: [i64; 8] = [0, 1, 2, 3, 4, 5, 8, 12];
const RESISTANCES: [u32; 2] = [1, 20];
const EXPECTED_CASES: usize = 960;
const EXPECTED_ROWS: usize = 1_920;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    PromptModulation,
    UnrelatedModulation,
    NearbyDrive,
    SamePathRepeat,
    TwoPathsOneModulation,
    WrongPathOnly,
}

impl Scenario {
    const ALL: [Self; 6] = [
        Self::PromptModulation,
        Self::UnrelatedModulation,
        Self::NearbyDrive,
        Self::SamePathRepeat,
        Self::TwoPathsOneModulation,
        Self::WrongPathOnly,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::PromptModulation => "prompt_modulation",
            Self::UnrelatedModulation => "unrelated_modulation",
            Self::NearbyDrive => "nearby_drive",
            Self::SamePathRepeat => "same_path_repeat",
            Self::TwoPathsOneModulation => "two_paths_one_modulation",
            Self::WrongPathOnly => "wrong_path_only",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Case {
    phase: i64,
    delay: i64,
    resistance: u32,
    scenario: Scenario,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PathState {
    live: bool,
    resistance: u32,
    coupling: i32,
    eligible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    candidate_eligible_events: String,
    competitor_eligible_events: String,
    candidate_trajectory: String,
    competitor_trajectory: String,
    candidate_updates: u64,
    competitor_updates: u64,
    proposals: u64,
    deallocations: u64,
    final_candidate: PathState,
    final_competitor: Option<PathState>,
    final_tick: i64,
    final_pressure_phase: i64,
    final_body_hash: String,
    final_checkpoint_hash: String,
    physical_trace_hash: String,
    naturally_quiescent: bool,
    replay_exact: bool,
}

#[derive(Clone, Copy, Debug)]
struct Geometry {
    candidate: ArrowId,
    competitor: Option<ArrowId>,
    initial_source: truelearner_core::CellId,
    event_target: truelearner_core::CellId,
}

#[derive(Default)]
struct Recorder {
    candidate_until: Option<i64>,
    competitor_until: Option<i64>,
    candidate_consumed: bool,
    competitor_consumed: bool,
    candidate_eligible_events: Vec<(i64, i64)>,
    competitor_eligible_events: Vec<(i64, i64)>,
    candidate_updates: u64,
    competitor_updates: u64,
    candidate_trajectory: Vec<String>,
    competitor_trajectory: Vec<String>,
    trace: Vec<PhysicalTransition>,
    proposals: u64,
    deallocations: u64,
}

impl Recorder {
    fn apply_trace(&mut self, trace: &[PhysicalTransition], geometry: Geometry) {
        for transition in trace {
            match &transition.event {
                PhysicalEvent::Eligible { arrow, until } if *arrow == geometry.candidate => {
                    self.candidate_until = Some(*until);
                    self.candidate_consumed = false;
                    self.candidate_eligible_events
                        .push((transition.tick, *until));
                }
                PhysicalEvent::Eligible { arrow, until }
                    if geometry.competitor.is_some_and(|id| id == *arrow) =>
                {
                    self.competitor_until = Some(*until);
                    self.competitor_consumed = false;
                    self.competitor_eligible_events
                        .push((transition.tick, *until));
                }
                PhysicalEvent::Resistance { arrow, .. } if *arrow == geometry.candidate => {
                    self.candidate_updates = self.candidate_updates.saturating_add(1);
                    self.candidate_consumed = true;
                }
                PhysicalEvent::Resistance { arrow, .. }
                    if geometry.competitor.is_some_and(|id| id == *arrow) =>
                {
                    self.competitor_updates = self.competitor_updates.saturating_add(1);
                    self.competitor_consumed = true;
                }
                _ => {}
            }
        }
        self.trace.extend_from_slice(trace);
    }

    fn apply_work(&mut self, work: Work) {
        self.proposals = self
            .proposals
            .saturating_add(work.local_structural_proposals);
        self.deallocations = self
            .deallocations
            .saturating_add(work.physical_deallocations);
    }

    fn record(&mut self, body: &PlasticSubstrate, geometry: Geometry, label: &str) {
        let tick = body.clock().tick;
        let candidate = durable_state(body, geometry.candidate);
        let candidate_eligible = candidate.live
            && !self.candidate_consumed
            && self.candidate_until.is_some_and(|until| tick <= until);
        self.candidate_trajectory.push(format!(
            "{label}@{tick}:{}:{}:{}:{}",
            u8::from(candidate.live),
            candidate.resistance,
            candidate.coupling,
            u8::from(candidate_eligible)
        ));
        if let Some(competitor) = geometry.competitor {
            let state = durable_state(body, competitor);
            let eligible = state.live
                && !self.competitor_consumed
                && self.competitor_until.is_some_and(|until| tick <= until);
            self.competitor_trajectory.push(format!(
                "{label}@{tick}:{}:{}:{}:{}",
                u8::from(state.live),
                state.resistance,
                state.coupling,
                u8::from(eligible)
            ));
        }
    }
}

fn input(target: truelearner_core::CellId, tick: i64, origin: u64, impulse: i32) -> SpikeInput {
    SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target,
        impulse,
    }
}

fn add_cell(
    body: &mut PlasticSubstrate,
    physical_id: u64,
    position: i32,
    threshold: i32,
) -> truelearner_core::CellId {
    body.add_cell(truelearner_core::CellSpec {
        physical_id,
        position,
        region: 0,
        threshold,
        resistance: 100,
    })
}

fn add_drive(
    body: &mut PlasticSubstrate,
    from: truelearner_core::CellId,
    to: truelearner_core::CellId,
    resistance: u32,
) -> ArrowId {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance,
        mode: TransmissionMode::Drive,
    })
}

fn add_modulatory(
    body: &mut PlasticSubstrate,
    from: truelearner_core::CellId,
    to: truelearner_core::CellId,
) {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: 100,
        mode: TransmissionMode::Modulatory,
    });
}

fn build(case: Case, mechanics: MechanicalConfig) -> (PlasticSubstrate, Geometry) {
    let arena = ArenaId(
        900_000
            + u64::try_from(case.phase).unwrap()
            + u64::try_from(case.delay).unwrap() * 10
            + u64::from(case.resistance) * 1_000,
    );
    let mut body = PlasticSubstrate::with_mechanics(arena, 12, 16, mechanics);
    body.set_physical_tracing(true);
    let source_a = add_cell(&mut body, 10_001, 0, 1);
    let target_a = add_cell(&mut body, 10_002, 10, 2);
    let event = add_cell(&mut body, 10_003, 20, 2);
    let nearby = add_cell(&mut body, 10_004, 30, 1);
    let nearby_target = add_cell(&mut body, 10_005, 40, 2);
    let source_b = add_cell(&mut body, 10_006, 50, 1);
    let target_b = add_cell(&mut body, 10_007, 60, 2);

    let candidate = add_drive(&mut body, source_a, target_a, case.resistance);
    let mut competitor = None;
    let (initial_source, event_target) = match case.scenario {
        Scenario::PromptModulation => {
            add_modulatory(&mut body, target_a, source_a);
            (source_a, target_a)
        }
        Scenario::UnrelatedModulation => {
            add_modulatory(&mut body, event, source_a);
            (source_a, event)
        }
        Scenario::NearbyDrive => {
            add_drive(&mut body, nearby, nearby_target, 100);
            (source_a, nearby)
        }
        Scenario::SamePathRepeat => (source_a, source_a),
        Scenario::TwoPathsOneModulation => {
            competitor = Some(add_drive(&mut body, source_a, target_b, case.resistance));
            add_modulatory(&mut body, target_a, source_a);
            (source_a, target_a)
        }
        Scenario::WrongPathOnly => {
            competitor = Some(add_drive(&mut body, source_b, target_b, case.resistance));
            add_modulatory(&mut body, target_b, source_a);
            (source_b, target_b)
        }
    };
    (
        body,
        Geometry {
            candidate,
            competitor,
            initial_source,
            event_target,
        },
    )
}

fn durable_state(body: &PlasticSubstrate, arrow: ArrowId) -> PathState {
    let durable = body
        .arena_body(1)
        .arrows
        .into_iter()
        .find(|candidate| candidate.id == arrow)
        .expect("observed ARROW must remain addressable");
    PathState {
        live: durable.live,
        resistance: durable.resistance,
        coupling: durable.coupling,
        eligible: false,
    }
}

fn format_eligible(events: &[(i64, i64)]) -> String {
    events
        .iter()
        .map(|(tick, until)| format!("{tick}:{until}"))
        .collect::<Vec<_>>()
        .join("|")
}

fn run(case: Case, mechanics: MechanicalConfig) -> Observation {
    let (mut body, geometry) = build(case, mechanics);
    let mut recorder = Recorder::default();
    if case.phase > 0 {
        recorder.apply_work(body.advance_time(case.phase));
    }
    recorder.record(&body, geometry, "constructed");

    let initial = body.arrive(&[input(geometry.initial_source, case.phase, 80_001, 1)], 99);
    assert!(initial.naturally_quiescent);
    recorder.apply_trace(&initial.physical_trace, geometry);
    recorder.apply_work(initial.work);
    recorder.record(&body, geometry, "initial");

    let event_tick = case.phase.saturating_add(case.delay);
    for tick in (case.phase + 1)..event_tick {
        recorder.apply_work(body.advance_time(tick));
        recorder.record(&body, geometry, "advance");
    }

    let event_impulse = match case.scenario {
        Scenario::PromptModulation
        | Scenario::UnrelatedModulation
        | Scenario::TwoPathsOneModulation
        | Scenario::WrongPathOnly => 2,
        Scenario::NearbyDrive | Scenario::SamePathRepeat => 1,
    };
    let event = body.arrive(
        &[input(
            geometry.event_target,
            event_tick,
            80_002,
            event_impulse,
        )],
        99,
    );
    assert!(event.naturally_quiescent);
    recorder.apply_trace(&event.physical_trace, geometry);
    recorder.apply_work(event.work);
    recorder.record(&body, geometry, "event");

    let final_tick = case
        .phase
        .saturating_add(15)
        .max(event_tick.saturating_add(6));
    for tick in (body.clock().tick + 1)..=final_tick {
        recorder.apply_work(body.advance_time(tick));
        recorder.record(&body, geometry, "advance");
    }

    let checkpoint = body.live_checkpoint(1).unwrap();
    let checkpoint_bytes = checkpoint.canonical_bytes().unwrap();
    let restored = PlasticSubstrate::from_live_checkpoint_with_mechanics(
        truelearner_core::LiveCheckpoint::decode(&checkpoint_bytes).unwrap(),
        mechanics,
    )
    .unwrap();
    let replay_exact = restored
        .live_checkpoint(1)
        .unwrap()
        .canonical_bytes()
        .unwrap()
        == checkpoint_bytes;
    let body_bytes = body.canonical_body_bytes(1).unwrap();
    let trace_text = format!("{:?}", recorder.trace);
    let mut final_candidate = durable_state(&body, geometry.candidate);
    final_candidate.eligible = final_candidate.live
        && !recorder.candidate_consumed
        && recorder
            .candidate_until
            .is_some_and(|until| final_tick <= until);
    let final_competitor = geometry.competitor.map(|arrow| {
        let mut state = durable_state(&body, arrow);
        state.eligible = state.live
            && !recorder.competitor_consumed
            && recorder
                .competitor_until
                .is_some_and(|until| final_tick <= until);
        state
    });

    Observation {
        candidate_eligible_events: format_eligible(&recorder.candidate_eligible_events),
        competitor_eligible_events: format_eligible(&recorder.competitor_eligible_events),
        candidate_trajectory: recorder.candidate_trajectory.join("|"),
        competitor_trajectory: recorder.competitor_trajectory.join("|"),
        candidate_updates: recorder.candidate_updates,
        competitor_updates: recorder.competitor_updates,
        proposals: recorder.proposals,
        deallocations: recorder.deallocations,
        final_candidate,
        final_competitor,
        final_tick,
        final_pressure_phase: body.clock().pressure_phase(),
        final_body_hash: ContentHash::of(&body_bytes).to_string(),
        final_checkpoint_hash: ContentHash::of(&checkpoint_bytes).to_string(),
        physical_trace_hash: ContentHash::of(trace_text.as_bytes()).to_string(),
        naturally_quiescent: true,
        replay_exact,
    }
}

fn csv_row(
    csv: &mut String,
    case_id: usize,
    mechanics: &str,
    case: Case,
    observation: &Observation,
) {
    let competitor = observation.final_competitor.clone().unwrap_or(PathState {
        live: false,
        resistance: 0,
        coupling: 0,
        eligible: false,
    });
    let fields = vec![
        case_id.to_string(),
        mechanics.to_owned(),
        case.phase.to_string(),
        case.delay.to_string(),
        case.resistance.to_string(),
        case.scenario.name().to_owned(),
        observation.candidate_eligible_events.clone(),
        observation.competitor_eligible_events.clone(),
        observation.candidate_updates.to_string(),
        observation.competitor_updates.to_string(),
        observation.proposals.to_string(),
        observation.deallocations.to_string(),
        u8::from(observation.final_candidate.live).to_string(),
        observation.final_candidate.resistance.to_string(),
        observation.final_candidate.coupling.to_string(),
        u8::from(observation.final_candidate.eligible).to_string(),
        u8::from(competitor.live).to_string(),
        competitor.resistance.to_string(),
        competitor.coupling.to_string(),
        u8::from(competitor.eligible).to_string(),
        observation.final_tick.to_string(),
        observation.final_pressure_phase.to_string(),
        observation.final_body_hash.clone(),
        observation.final_checkpoint_hash.clone(),
        observation.physical_trace_hash.clone(),
        u8::from(observation.naturally_quiescent).to_string(),
        u8::from(observation.replay_exact).to_string(),
        observation.candidate_trajectory.clone(),
        observation.competitor_trajectory.clone(),
    ];
    writeln!(csv, "{}", fields.join(",")).unwrap();
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/tc_ds0_old_window_v1"));
    fs::create_dir_all(&output).unwrap();

    let mut csv = String::from(
        "case_id,mechanics,phase,delay,initial_resistance,scenario,candidate_eligible_events,competitor_eligible_events,candidate_updates,competitor_updates,proposals,deallocations,final_candidate_live,final_candidate_resistance,final_candidate_coupling,final_candidate_eligible,final_competitor_live,final_competitor_resistance,final_competitor_coupling,final_competitor_eligible,final_tick,final_pressure_phase,final_body_hash,final_checkpoint_hash,physical_trace_hash,naturally_quiescent,replay_exact,candidate_trajectory,competitor_trajectory\n",
    );
    let mut cases = 0usize;
    let mut rows = 0usize;
    let mut prompt_credit = 0usize;
    let mut unrelated_credit = 0usize;
    let mut prompt_unrelated_equal = 0usize;
    let mut two_path_cross_credit = 0usize;
    let mut wrong_path_credit = 0usize;
    let mut prompt_by_key: Vec<(i64, i64, u32, u64)> = Vec::new();

    for phase in 0..10 {
        for delay in DELAYS {
            for resistance in RESISTANCES {
                for scenario in Scenario::ALL {
                    let case = Case {
                        phase,
                        delay,
                        resistance,
                        scenario,
                    };
                    cases += 1;
                    let reference = run(case, MechanicalConfig::REFERENCE);
                    let production = run(case, MechanicalConfig::PRODUCTION);
                    assert_eq!(production, reference, "mechanics diverged for {case:?}");
                    assert!(reference.naturally_quiescent && reference.replay_exact);
                    csv_row(&mut csv, cases, "reference", case, &reference);
                    csv_row(&mut csv, cases, "production", case, &production);
                    rows += 2;

                    match scenario {
                        Scenario::PromptModulation => {
                            prompt_credit += usize::from(reference.candidate_updates > 0);
                            prompt_by_key.push((
                                phase,
                                delay,
                                resistance,
                                reference.candidate_updates,
                            ));
                        }
                        Scenario::UnrelatedModulation => {
                            unrelated_credit += usize::from(reference.candidate_updates > 0);
                            let prompt = prompt_by_key
                                .iter()
                                .find(|(p, d, r, _)| *p == phase && *d == delay && *r == resistance)
                                .map(|(_, _, _, updates)| *updates)
                                .expect("prompt row precedes unrelated row");
                            prompt_unrelated_equal +=
                                usize::from(prompt == reference.candidate_updates);
                        }
                        Scenario::TwoPathsOneModulation => {
                            two_path_cross_credit += usize::from(
                                reference.candidate_updates > 0 && reference.competitor_updates > 0,
                            );
                        }
                        Scenario::WrongPathOnly => {
                            wrong_path_credit += usize::from(reference.candidate_updates > 0);
                        }
                        Scenario::NearbyDrive | Scenario::SamePathRepeat => {}
                    }
                }
            }
        }
    }

    assert_eq!(cases, EXPECTED_CASES);
    assert_eq!(rows, EXPECTED_ROWS);
    let report = format!(
        "# TC-DS0 old-window characterization v1\n\n\
         Status: characterization complete; no candidate selected.\n\n\
         - physical cases: `{cases}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{rows}/{EXPECTED_ROWS}`\n\
         - reference/production exact pairs: `{cases}/{EXPECTED_CASES}`\n\
         - naturally quiescent and replay-exact rows: `{rows}/{EXPECTED_ROWS}`\n\
         - prompt-modulation cases receiving credit: `{prompt_credit}/160`\n\
         - unrelated-modulation cases receiving credit: `{unrelated_credit}/160`\n\
         - prompt/unrelated pairs with equal update count: `{prompt_unrelated_equal}/160`\n\
         - two-path cases in which both paths received one return: `{two_path_cross_credit}/160`\n\
         - never-traversed observed paths receiving credit: `{wrong_path_credit}/160`\n\n\
         This report describes the frozen rectangular law. Counts are not a\n\
         parameter-selection score and do not advance TC-DS1, ARC, or authority.\n"
    );
    fs::write(output.join("matrix.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!("TC_DS0_CHARACTERIZATION_COMPLETE cases={cases} rows={rows}");
}

fn write_checksums(output: &Path) {
    let mut sums = String::new();
    for name in ["matrix.csv", "report.md"] {
        let bytes = fs::read(output.join(name)).unwrap();
        writeln!(sums, "{}  {name}", ContentHash::of(&bytes)).unwrap();
    }
    fs::write(output.join("SHA256SUMS"), sums).unwrap();
}
