use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const AUTHORITATIVE_COMMIT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const PX0_PX2_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PROTOCOL_SHA256: &str = "8780e9b4ceb521c53052e3f3e63e8f096c6e5d116352d922f78faf1d880df009";
const IMPLEMENTATION_TAG: &str = "cj0-or-ordinary-convergence-implementation-v1";
const PX3_NEGATIVE_COMMIT: &str = "873094497ff6eb74363191dc5edc479c7d66de72";
const PX3_R_NEGATIVE_COMMIT: &str = "5feb9b4c4755ed40d58ffc9cb8769d5523ea46f0";
const SOURCE_PATH: &str =
    "crates/px0-physical-correspondence/examples/cj0_or_ordinary_convergence.rs";
const PROTOCOL_PATH: &str = "experiments/cj0_or_ordinary_convergence_protocol.md";
const LAW_PATH: &str = "crates/px0-physical-correspondence/src/lib.rs";
#[cfg(test)]
const DEVELOPMENT_NAMESPACE: u64 = 0xcf00_0000_0000;
const STRONG_RESISTANCE: u32 = 512;

// BEGIN NUMERIC PHYSICAL BLOCK

const CELL_ORDERS: [[usize; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [1, 2, 0, 3],
    [1, 2, 3, 0],
    [1, 3, 0, 2],
    [1, 3, 2, 0],
    [2, 0, 1, 3],
    [2, 0, 3, 1],
    [2, 1, 0, 3],
    [2, 1, 3, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [3, 1, 0, 2],
    [3, 1, 2, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0],
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct NumericSpec {
    namespace: u64,
    positions: [i32; 4],
    cell_order: [usize; 4],
    identity_rotation: usize,
    reverse_links: bool,
    reverse_entries: bool,
    delay: i64,
    skew: i64,
    phase_pattern: usize,
    link_kind: [u8; 2],
    level_threshold: i32,
}

#[derive(Clone)]
struct NumericFixture {
    substrate: PlasticSubstrate,
    cells: [CellId; 4],
    physical: [u64; 4],
    links: [Option<ArrowId>; 2],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NumericRun {
    active: [bool; 2],
    firings: [usize; 4],
    crossings: usize,
    external_spikes: usize,
    cells: usize,
    arrows: usize,
    live_links: [bool; 2],
    persistent_bytes: usize,
    execution: Execution,
    idle: Execution,
}

fn numeric_build(spec: &NumericSpec) -> NumericFixture {
    let mut substrate = PlasticSubstrate::new();
    let mut handles = vec![None; 4];
    let mut physical = [0_u64; 4];
    for (index, value) in physical.iter_mut().enumerate() {
        *value = spec.namespace + 0x100 + ((index + spec.identity_rotation) % 4) as u64;
    }
    for index in spec.cell_order {
        let threshold = if index < 2 {
            4
        } else if index == 2 {
            spec.level_threshold
        } else {
            1
        };
        handles[index] = Some(substrate.add_cell(CellSpec {
            physical_id: physical[index],
            position: spec.positions[index],
            region: i16::from(index == 3),
            threshold,
            resistance: 1_024,
        }));
    }
    let cells: [CellId; 4] = handles
        .into_iter()
        .map(|handle| handle.expect("numeric CELL handle"))
        .collect::<Vec<_>>()
        .try_into()
        .expect("exactly four numeric CELL handles");
    let mut links = [None, None];
    let order = if spec.reverse_links { [1, 0] } else { [0, 1] };
    for index in order {
        let kind = spec.link_kind[index];
        if kind == 0 {
            continue;
        }
        let (delay, coupling, resistance) = match kind {
            1 => (spec.delay, 1, STRONG_RESISTANCE),
            2 => (spec.delay, 0, STRONG_RESISTANCE),
            3 => (6, 1, 1),
            _ => panic!("numeric ARROW kind must be in 0..=3"),
        };
        links[index] = Some(substrate.add_arrow(ArrowSpec {
            from: cells[index],
            to: cells[2],
            delay,
            phase: 0,
            coupling,
            resistance,
        }));
    }
    substrate.add_arrow(ArrowSpec {
        from: cells[2],
        to: cells[3],
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance: STRONG_RESISTANCE,
    });
    NumericFixture {
        substrate,
        cells,
        physical,
        links,
    }
}

fn numeric_phase(pattern: usize, ordinal: usize) -> i32 {
    match pattern % 3 {
        0 => ordinal as i32,
        1 => 0,
        _ => (3 - ordinal) as i32,
    }
}

fn numeric_run(spec: &NumericSpec, active: [bool; 2]) -> NumericRun {
    let mut fixture = numeric_build(spec);
    let mut entries = Vec::new();
    for (index, enabled) in active.into_iter().enumerate() {
        if !enabled {
            continue;
        }
        for ordinal in 0..4 {
            entries.push(SpikeInput {
                arrival_tick: if index == 0 { 0 } else { spec.skew },
                phase: numeric_phase(spec.phase_pattern, ordinal),
                origin_physical: spec.namespace + 0x800 + index as u64 * 0x10 + ordinal as u64,
                target: fixture.cells[index],
                impulse: 1,
            });
        }
    }
    if spec.reverse_entries {
        entries.reverse();
    }
    let external_spikes = entries.len();
    for entry in entries {
        fixture.substrate.enter(entry);
    }
    let execution = fixture.substrate.propagate();
    let idle = fixture.substrate.propagate();
    let mut firings = [0_usize; 4];
    for entry in &execution.trace {
        for (index, physical) in fixture.physical.into_iter().enumerate() {
            if entry.target_physical == physical && entry.fired {
                firings[index] += 1;
            }
        }
    }
    let live_links = fixture
        .links
        .map(|link| link.is_some_and(|arrow| fixture.substrate.arrow_is_live(arrow)));
    NumericRun {
        active,
        firings,
        crossings: execution.crossings.len(),
        external_spikes,
        cells: 4,
        arrows: fixture.substrate.arrow_count(),
        live_links,
        persistent_bytes: fixture.substrate.persistent_bytes(),
        execution,
        idle,
    }
}

// END NUMERIC PHYSICAL BLOCK

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Stage {
    flag: &'static str,
    name: &'static str,
    ordinal: usize,
    namespace: u64,
    rows: usize,
    csv: &'static str,
    markdown: &'static str,
    staging_csv: &'static str,
    staging_markdown: &'static str,
    marker: &'static str,
}

const STAGES: [Stage; 4] = [
    Stage {
        flag: "--probe",
        name: "PROBE v1",
        ordinal: 0,
        namespace: 0xc100_0000_0000,
        rows: 8,
        csv: "results/cj0_or_ordinary_convergence_probe_v1.csv",
        markdown: "results/cj0_or_ordinary_convergence_probe_v1.md",
        staging_csv: "results/.cj0_or_ordinary_convergence_probe_v1.csv.staging",
        staging_markdown: "results/.cj0_or_ordinary_convergence_probe_v1.md.staging",
        marker: "CJ0_OR_ORDINARY_CONVERGENCE_PROBE_V1_EVIDENCE_SPENT",
    },
    Stage {
        flag: "--micro",
        name: "MICRO v1",
        ordinal: 1,
        namespace: 0xc200_0000_0000,
        rows: 24,
        csv: "results/cj0_or_ordinary_convergence_micro_v1.csv",
        markdown: "results/cj0_or_ordinary_convergence_micro_v1.md",
        staging_csv: "results/.cj0_or_ordinary_convergence_micro_v1.csv.staging",
        staging_markdown: "results/.cj0_or_ordinary_convergence_micro_v1.md.staging",
        marker: "CJ0_OR_ORDINARY_CONVERGENCE_MICRO_V1_EVIDENCE_SPENT",
    },
    Stage {
        flag: "--gate",
        name: "GATE v1",
        ordinal: 2,
        namespace: 0xc300_0000_0000,
        rows: 72,
        csv: "results/cj0_or_ordinary_convergence_gate_v1.csv",
        markdown: "results/cj0_or_ordinary_convergence_gate_v1.md",
        staging_csv: "results/.cj0_or_ordinary_convergence_gate_v1.csv.staging",
        staging_markdown: "results/.cj0_or_ordinary_convergence_gate_v1.md.staging",
        marker: "CJ0_OR_ORDINARY_CONVERGENCE_GATE_V1_EVIDENCE_SPENT",
    },
    Stage {
        flag: "--definitive",
        name: "definitive",
        ordinal: 3,
        namespace: 0xc400_0000_0000,
        rows: 120,
        csv: "results/cj0_or_ordinary_convergence_definitive.csv",
        markdown: "results/cj0_or_ordinary_convergence_definitive.md",
        staging_csv: "results/.cj0_or_ordinary_convergence_definitive.csv.staging",
        staging_markdown: "results/.cj0_or_ordinary_convergence_definitive.md.staging",
        marker: "CJ0_OR_ORDINARY_CONVERGENCE_DEFINITIVE_EVIDENCE_SPENT",
    },
];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Accounting {
    work: u64,
    persistent_bytes: usize,
    external_spikes: usize,
    source_firings: usize,
    convergence_firings: usize,
    downstream_firings: usize,
    crossings: usize,
    deallocations: u64,
    proposals: u64,
    cells: usize,
    arrows: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Claims([bool; 14]);

impl Claims {
    fn count(&self) -> usize {
        self.0.into_iter().filter(|value| *value).count()
    }

    fn all(&self) -> bool {
        self.count() == self.0.len()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: usize,
    spec: NumericSpec,
    outcomes: Vec<NumericRun>,
    accounting: Accounting,
    joint_expected: usize,
    refractory_suppressed: bool,
    replay_exact: bool,
    claims: Claims,
    passed: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args == ["--preflight"] {
        require_source_audit();
        for stage in STAGES {
            require_absent(stage);
        }
        println!("CJ0_OR_ORDINARY_CONVERGENCE_PREFLIGHT_OK");
        return;
    }
    let Some(stage) = STAGES.iter().copied().find(|stage| args == [stage.flag]) else {
        eprintln!("CJ0-OR requires --preflight, --probe, --micro, --gate, or --definitive");
        std::process::exit(2);
    };
    require_source_audit();
    require_clean_worktree();
    require_absent(stage);
    require_prior_stage(stage);
    eprintln!("{}", stage.marker);

    let rows = (0..stage.rows)
        .map(|seed| evaluate_row(stage, seed))
        .collect::<Vec<_>>();
    let csv = csv(stage, &rows);
    let markdown = markdown(stage, &rows);
    publish(stage, &csv, &markdown);
    if rows.iter().any(|row| !row.passed) {
        std::process::exit(1);
    }
}

fn config(stage: Stage, seed: usize) -> NumericSpec {
    let ordinal = stage.ordinal;
    let layout = (seed + 3 * ordinal) % 4;
    let distance = [8_i32, 16, 32, 64][layout];
    let mirror = !(seed + ordinal).is_multiple_of(2);
    let sign = if mirror { -1 } else { 1 };
    NumericSpec {
        namespace: stage.namespace + seed as u64 * 0x10_0000,
        positions: [-distance * sign, distance * sign, 0, distance * sign * 3],
        cell_order: CELL_ORDERS[(5 * seed + 7 * ordinal) % CELL_ORDERS.len()],
        identity_rotation: (seed + ordinal) % 4,
        reverse_links: !(seed / 2 + ordinal).is_multiple_of(2),
        reverse_entries: !(seed / 3 + ordinal).is_multiple_of(2),
        delay: ((seed + 2 * ordinal) % 6) as i64,
        skew: ((seed / 6 + ordinal) % 5) as i64,
        phase_pattern: (seed + ordinal) % 3,
        link_kind: [1, 1],
        level_threshold: 1,
    }
}

fn case_spec(base: &NumericSpec, index: usize) -> NumericSpec {
    let mut spec = base.clone();
    spec.namespace = base.namespace + index as u64 * 0x1000;
    spec
}

fn push_case(
    outcomes: &mut Vec<NumericRun>,
    base: &NumericSpec,
    index: usize,
    kinds: [u8; 2],
    threshold: i32,
    active: [bool; 2],
    skew: Option<i64>,
) {
    let mut spec = case_spec(base, index);
    spec.link_kind = kinds;
    spec.level_threshold = threshold;
    if let Some(value) = skew {
        spec.skew = value;
    }
    outcomes.push(numeric_run(&spec, active));
}

fn evaluate_row(stage: Stage, seed: usize) -> Row {
    let spec = config(stage, seed);
    let mut outcomes = Vec::with_capacity(23);
    let candidate_cases = [
        ([false, false], 0),
        ([true, false], 1),
        ([false, true], 2),
        ([true, true], 3),
    ];
    for (active, index) in candidate_cases {
        push_case(&mut outcomes, &spec, index, [1, 1], 1, active, None);
    }
    for (active, index) in candidate_cases {
        push_case(&mut outcomes, &spec, index, [1, 1], 1, active, None);
    }
    push_case(&mut outcomes, &spec, 4, [2, 1], 1, [true, false], None);
    push_case(&mut outcomes, &spec, 5, [2, 1], 1, [true, true], None);
    push_case(&mut outcomes, &spec, 6, [1, 2], 1, [false, true], None);
    push_case(&mut outcomes, &spec, 7, [1, 2], 1, [true, true], None);
    push_case(&mut outcomes, &spec, 8, [0, 1], 1, [true, false], None);
    push_case(&mut outcomes, &spec, 9, [0, 1], 1, [true, true], None);
    push_case(&mut outcomes, &spec, 10, [1, 0], 1, [false, true], None);
    push_case(&mut outcomes, &spec, 11, [1, 0], 1, [true, true], None);
    push_case(&mut outcomes, &spec, 12, [3, 1], 1, [true, false], None);
    push_case(&mut outcomes, &spec, 13, [3, 1], 1, [true, true], None);
    push_case(&mut outcomes, &spec, 14, [1, 3], 1, [false, true], None);
    push_case(&mut outcomes, &spec, 15, [1, 3], 1, [true, true], None);
    push_case(&mut outcomes, &spec, 16, [1, 1], 2, [true, false], Some(0));
    push_case(&mut outcomes, &spec, 17, [1, 1], 2, [false, true], Some(0));
    push_case(&mut outcomes, &spec, 18, [1, 1], 2, [true, true], Some(0));
    assert_eq!(outcomes.len(), 23);

    let joint_expected = if spec.skew == 0 { 1 } else { 2 };
    let refractory_suppressed = outcomes[3].firings[2] == 1;
    let replay_exact = outcomes[0..4] == outcomes[4..8];
    let accounting = account(&outcomes);
    let primary_none = &outcomes[0];
    let primary_first = &outcomes[1];
    let primary_second = &outcomes[2];
    let primary_joint = &outcomes[3];

    let p0 = symmetric(&spec)
        && spec.namespace >= stage.namespace
        && spec.namespace < stage.namespace + stage.rows as u64 * 0x10_0000;
    let p1 = exact_single(primary_first, 0);
    let p2 = exact_single(primary_second, 1);
    let p3 = primary_joint.firings[0..2] == [1, 1]
        && primary_joint.firings[2] == joint_expected
        && primary_joint.firings[3] == joint_expected
        && primary_joint.crossings == joint_expected
        && (1..=2).contains(&primary_joint.firings[2]);
    let p4 = primary_none.firings == [0, 0, 0, 0]
        && primary_none.crossings == 0
        && primary_none.external_spikes == 0;
    let p5 = suppressed_alone(&outcomes[8], 0)
        && intact_with_both(&outcomes[9], 1)
        && suppressed_alone(&outcomes[10], 1)
        && intact_with_both(&outcomes[11], 0);
    let p6 = suppressed_alone(&outcomes[16], 0)
        && intact_with_both(&outcomes[17], 1)
        && suppressed_alone(&outcomes[18], 1)
        && intact_with_both(&outcomes[19], 0)
        && outcomes[16..20]
            .iter()
            .all(|outcome| outcome.execution.work.physical_deallocations == 1);
    let p7 = suppressed_alone(&outcomes[12], 0)
        && intact_with_both(&outcomes[13], 1)
        && suppressed_alone(&outcomes[14], 1)
        && intact_with_both(&outcomes[15], 0)
        && outcomes[12..16].iter().all(|outcome| outcome.arrows == 2);
    let p8 = outcomes[20].firings[2..4] == [0, 0]
        && outcomes[21].firings[2..4] == [0, 0]
        && outcomes[22].firings[2..4] == [1, 1];
    let p9 =
        primary_joint.firings[2] == joint_expected && refractory_suppressed == (spec.skew == 0);
    let p10 = outcomes.iter().all(naturally_stopped);
    let p11 = outcomes.iter().all(bounded_sources);
    let p12 = replay_exact;
    let audited_accounting = account_independently(&outcomes);
    let p13 = accounting == audited_accounting
        && accounting.external_spikes == 120
        && accounting.source_firings == 30
        && accounting.cells == 92
        && accounting.arrows == 65
        && accounting.deallocations == 4
        && accounting.proposals == 0
        && accounting.persistent_bytes > 0
        && accounting.work > 0;
    let claims = Claims([p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13]);
    let passed = claims.all();
    Row {
        seed,
        spec,
        outcomes,
        accounting,
        joint_expected,
        refractory_suppressed,
        replay_exact,
        claims,
        passed,
    }
}

fn symmetric(spec: &NumericSpec) -> bool {
    spec.positions[0].abs() == spec.positions[1].abs()
        && spec.positions[0] == -spec.positions[1]
        && spec.link_kind == [1, 1]
        && spec.level_threshold == 1
        && spec.delay < 6
}

fn exact_single(outcome: &NumericRun, index: usize) -> bool {
    let mut sources = [0_usize; 2];
    sources[index] = 1;
    outcome.firings[0..2] == sources
        && outcome.firings[2..4] == [1, 1]
        && outcome.crossings == 1
        && outcome.external_spikes == 4
}

fn suppressed_alone(outcome: &NumericRun, active: usize) -> bool {
    outcome.firings[active] == 1
        && outcome.firings[1 - active] == 0
        && outcome.firings[2..4] == [0, 0]
        && outcome.crossings == 0
}

fn intact_with_both(outcome: &NumericRun, intact: usize) -> bool {
    outcome.firings[0..2] == [1, 1]
        && outcome.firings[2..4] == [1, 1]
        && outcome.crossings == 1
        && outcome.live_links[intact]
}

fn naturally_stopped(outcome: &NumericRun) -> bool {
    outcome.execution.naturally_quiescent
        && outcome.idle.naturally_quiescent
        && outcome.idle.trace.is_empty()
        && outcome.idle.crossings.is_empty()
        && outcome.idle.work.total() == 0
        && outcome.idle.start_fingerprint == outcome.idle.end_fingerprint
        && outcome.execution.end_fingerprint == outcome.idle.start_fingerprint
}

fn bounded_sources(outcome: &NumericRun) -> bool {
    let expected = outcome.active.map(usize::from);
    let source_total = outcome.firings[0] + outcome.firings[1];
    outcome.firings[0..2] == expected
        && outcome.external_spikes == source_total * 4
        && outcome.firings[2] <= source_total
        && outcome.firings[3] <= outcome.firings[2]
        && outcome.crossings == outcome.firings[2]
}

fn add_work(target: &mut WorkLedger, source: &WorkLedger) {
    target.queue_comparisons += source.queue_comparisons;
    target.spikes_delivered += source.spikes_delivered;
    target.generation_checks += source.generation_checks;
    target.state_updates += source.state_updates;
    target.threshold_checks += source.threshold_checks;
    target.firings += source.firings;
    target.arrow_checks += source.arrow_checks;
    target.spikes_emitted += source.spikes_emitted;
    target.local_eligibility_writes += source.local_eligibility_writes;
    target.local_return_updates += source.local_return_updates;
    target.ordinary_pressure_updates += source.ordinary_pressure_updates;
    target.local_structural_proposals += source.local_structural_proposals;
    target.physical_deallocations += source.physical_deallocations;
}

fn account(outcomes: &[NumericRun]) -> Accounting {
    let mut accounting = Accounting::default();
    let mut work = WorkLedger::default();
    for outcome in outcomes {
        add_work(&mut work, &outcome.execution.work);
        add_work(&mut work, &outcome.idle.work);
        accounting.persistent_bytes += outcome.persistent_bytes;
        accounting.external_spikes += outcome.external_spikes;
        accounting.source_firings += outcome.firings[0] + outcome.firings[1];
        accounting.convergence_firings += outcome.firings[2];
        accounting.downstream_firings += outcome.firings[3];
        accounting.crossings += outcome.crossings;
        accounting.cells += outcome.cells;
        accounting.arrows += outcome.arrows;
    }
    accounting.work = work.total();
    accounting.deallocations = work.physical_deallocations;
    accounting.proposals = work.local_structural_proposals;
    accounting
}

fn account_independently(outcomes: &[NumericRun]) -> Accounting {
    Accounting {
        work: outcomes
            .iter()
            .map(|outcome| outcome.execution.work.total() + outcome.idle.work.total())
            .sum(),
        persistent_bytes: outcomes
            .iter()
            .map(|outcome| outcome.persistent_bytes)
            .sum(),
        external_spikes: outcomes.iter().map(|outcome| outcome.external_spikes).sum(),
        source_firings: outcomes
            .iter()
            .map(|outcome| outcome.firings[0..2].iter().sum::<usize>())
            .sum(),
        convergence_firings: outcomes.iter().map(|outcome| outcome.firings[2]).sum(),
        downstream_firings: outcomes.iter().map(|outcome| outcome.firings[3]).sum(),
        crossings: outcomes.iter().map(|outcome| outcome.crossings).sum(),
        deallocations: outcomes
            .iter()
            .map(|outcome| {
                outcome.execution.work.physical_deallocations
                    + outcome.idle.work.physical_deallocations
            })
            .sum(),
        proposals: outcomes
            .iter()
            .map(|outcome| {
                outcome.execution.work.local_structural_proposals
                    + outcome.idle.work.local_structural_proposals
            })
            .sum(),
        cells: outcomes.iter().map(|outcome| outcome.cells).sum(),
        arrows: outcomes.iter().map(|outcome| outcome.arrows).sum(),
    }
}

fn require_source_audit() {
    assert_eq!(sha256(LAW_PATH), PX0_PX2_SHA256, "authoritative law hash");
    assert_eq!(sha256(PROTOCOL_PATH), PROTOCOL_SHA256, "protocol hash");
    assert!(
        command_ok(&["merge-base", "--is-ancestor", AUTHORITATIVE_COMMIT, "HEAD"]),
        "authoritative ancestry"
    );
    assert!(
        command_output(&["diff", "--quiet", AUTHORITATIVE_COMMIT, "--", LAW_PATH]).is_empty(),
        "authoritative law bytes changed"
    );
    assert_eq!(
        command_output(&[
            "rev-parse",
            "px3-physical-event-boundaries-frozen-negative-handoff-v1^{commit}"
        ]),
        PX3_NEGATIVE_COMMIT,
        "PX3 negative tag moved"
    );
    assert_eq!(
        command_output(&[
            "rev-parse",
            "px3-r-c-downstream-convergence-frozen-negative-handoff-v1^{commit}"
        ]),
        PX3_R_NEGATIVE_COMMIT,
        "PX3-R negative tag moved"
    );
    let tagged = Command::new("git")
        .args(["show", &format!("{IMPLEMENTATION_TAG}:{SOURCE_PATH}")])
        .output()
        .expect("read tagged implementation source");
    assert!(tagged.status.success(), "implementation tag unavailable");
    assert_eq!(
        tagged.stdout,
        std::fs::read(SOURCE_PATH).expect("read implementation source")
    );
}

fn require_clean_worktree() {
    assert!(
        command_output(&["status", "--porcelain"]).is_empty(),
        "evidence requires a clean worktree"
    );
}

fn require_absent(stage: Stage) {
    for path in [
        stage.csv,
        stage.markdown,
        stage.staging_csv,
        stage.staging_markdown,
    ] {
        assert!(!Path::new(path).exists(), "stage artifact exists: {path}");
    }
}

fn require_prior_stage(stage: Stage) {
    for prior in STAGES.iter().take(stage.ordinal) {
        assert!(Path::new(prior.csv).is_file(), "prior CSV absent");
        assert!(Path::new(prior.markdown).is_file(), "prior report absent");
    }
    for later in STAGES.iter().skip(stage.ordinal + 1) {
        require_absent(*later);
    }
}

fn command_ok(args: &[&str]) -> bool {
    Command::new("git")
        .args(args)
        .status()
        .expect("run git audit")
        .success()
}

fn command_output(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("run git audit");
    assert!(output.status.success(), "git audit failed: {args:?}");
    String::from_utf8(output.stdout)
        .expect("git output is UTF-8")
        .trim()
        .to_string()
}

fn sha256(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("run sha256sum");
    assert!(output.status.success(), "sha256sum failed: {path}");
    String::from_utf8(output.stdout)
        .expect("sha256 output is UTF-8")
        .split_whitespace()
        .next()
        .expect("sha256 digest")
        .to_string()
}

fn csv(stage: Stage, rows: &[Row]) -> String {
    let mut output = String::from(
        "stage,row,namespace,positions,cell_order,identity_rotation,reverse_arrows,reverse_spikes,delay,skew,phase_pattern,first_firings,second_firings,joint_firings,joint_expected,refractory_suppressed,blocked_outputs,stale_outputs,absent_outputs,threshold2_outputs,naturally_quiescent,replay_exact,external_spikes,source_firings,convergence_firings,downstream_firings,crossings,deallocations,proposals,cells,arrows,persistent_bytes,work,p0,p1,p2,p3,p4,p5,p6,p7,p8,p9,p10,p11,p12,p13,claims,passed\n",
    );
    for row in rows {
        let fields = vec![
            stage.name.to_string(),
            row.seed.to_string(),
            format!("0x{:x}", row.spec.namespace),
            join_i32(row.spec.positions),
            join_usize(row.spec.cell_order),
            row.spec.identity_rotation.to_string(),
            row.spec.reverse_links.to_string(),
            row.spec.reverse_entries.to_string(),
            row.spec.delay.to_string(),
            row.spec.skew.to_string(),
            row.spec.phase_pattern.to_string(),
            join_usize(row.outcomes[1].firings),
            join_usize(row.outcomes[2].firings),
            join_usize(row.outcomes[3].firings),
            row.joint_expected.to_string(),
            row.refractory_suppressed.to_string(),
            format!(
                "{}|{}|{}|{}",
                row.outcomes[8].firings[3],
                row.outcomes[9].firings[3],
                row.outcomes[10].firings[3],
                row.outcomes[11].firings[3]
            ),
            format!(
                "{}|{}|{}|{}",
                row.outcomes[16].firings[3],
                row.outcomes[17].firings[3],
                row.outcomes[18].firings[3],
                row.outcomes[19].firings[3]
            ),
            format!(
                "{}|{}|{}|{}",
                row.outcomes[12].firings[3],
                row.outcomes[13].firings[3],
                row.outcomes[14].firings[3],
                row.outcomes[15].firings[3]
            ),
            format!(
                "{}|{}|{}",
                row.outcomes[20].firings[3],
                row.outcomes[21].firings[3],
                row.outcomes[22].firings[3]
            ),
            row.outcomes.iter().all(naturally_stopped).to_string(),
            row.replay_exact.to_string(),
            row.accounting.external_spikes.to_string(),
            row.accounting.source_firings.to_string(),
            row.accounting.convergence_firings.to_string(),
            row.accounting.downstream_firings.to_string(),
            row.accounting.crossings.to_string(),
            row.accounting.deallocations.to_string(),
            row.accounting.proposals.to_string(),
            row.accounting.cells.to_string(),
            row.accounting.arrows.to_string(),
            row.accounting.persistent_bytes.to_string(),
            row.accounting.work.to_string(),
        ];
        output.push_str(&fields.join(","));
        for claim in row.claims.0 {
            output.push(',');
            output.push_str(&claim.to_string());
        }
        output.push_str(&format!(",{},{}\n", row.claims.count(), row.passed));
    }
    output
}

fn markdown(stage: Stage, rows: &[Row]) -> String {
    let passed = rows.iter().all(|row| row.passed);
    let claims = rows.iter().map(|row| row.claims.count()).sum::<usize>();
    let accounting = rows.iter().fold(Accounting::default(), |mut sum, row| {
        sum.work += row.accounting.work;
        sum.persistent_bytes += row.accounting.persistent_bytes;
        sum.external_spikes += row.accounting.external_spikes;
        sum.source_firings += row.accounting.source_firings;
        sum.convergence_firings += row.accounting.convergence_firings;
        sum.downstream_firings += row.accounting.downstream_firings;
        sum.crossings += row.accounting.crossings;
        sum.deallocations += row.accounting.deallocations;
        sum.proposals += row.accounting.proposals;
        sum.cells += row.accounting.cells;
        sum.arrows += row.accounting.arrows;
        sum
    });
    let simultaneous = rows.iter().filter(|row| row.refractory_suppressed).count();
    let skewed = rows.len() - simultaneous;
    format!(
        "# CJ0-OR ordinary convergence {}\n\nOutcome: **{}**.\n\n- rows: `{}/{}`;\n- independent clauses: `{}/{}`;\n- exact replays: `{}/{}`;\n- simultaneous refractory-suppressed rows: `{}`;\n- positive-skew two-output rows: `{}`;\n- external SPIKEs / source firings: `{}/{}`;\n- convergence / downstream / crossing totals: `{}/{}/{}`;\n- stale-route deallocations / incidental proposals: `{}/{}`;\n- constructed CELL / ARROW instances: `{}/{}`;\n- aggregate persistent substrate bytes: `{}`;\n- ledgered work: `{}` operations.\n\nEach isolated route reached the ordinary convergence CELL and downstream CELL. Both routes together reached them at least once and at most twice. Simultaneous cardinality suppression is attributed to the frozen refractory rule; the threshold-2 `0,0,1` controls are excluded as saturation/conjunction rather than disjunction. Every queue drained naturally, every idle follow-up was inert, and there was no autonomous source refiring or runaway propagation.\n\nThis stage does not change PX0-PX2, reinterpret a PX3 negative, add an OR law, or advance authority.\n",
        stage.name,
        if passed { "POSITIVE" } else { "NEGATIVE" },
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
        claims,
        rows.len() * 14,
        rows.iter().filter(|row| row.replay_exact).count(),
        rows.len(),
        simultaneous,
        skewed,
        accounting.external_spikes,
        accounting.source_firings,
        accounting.convergence_firings,
        accounting.downstream_firings,
        accounting.crossings,
        accounting.deallocations,
        accounting.proposals,
        accounting.cells,
        accounting.arrows,
        accounting.persistent_bytes,
        accounting.work,
    )
}

fn join_i32(values: [i32; 4]) -> String {
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn join_usize(values: [usize; 4]) -> String {
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn write_staging(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create fresh CJ0-OR staging artifact");
    file.write_all(contents.as_bytes())
        .expect("write CJ0-OR staging artifact");
    file.sync_all().expect("sync CJ0-OR staging artifact");
}

fn publish(stage: Stage, csv_contents: &str, markdown_contents: &str) {
    write_staging(stage.staging_csv, csv_contents);
    write_staging(stage.staging_markdown, markdown_contents);
    rename(stage.staging_csv, stage.csv).expect("publish CJ0-OR CSV");
    rename(stage.staging_markdown, stage.markdown).expect("publish CJ0-OR report");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn development_stage(rows: usize) -> Stage {
        Stage {
            flag: "--development",
            name: "development",
            ordinal: 0,
            namespace: DEVELOPMENT_NAMESPACE,
            rows,
            csv: "unused.csv",
            markdown: "unused.md",
            staging_csv: "unused.csv.staging",
            staging_markdown: "unused.md.staging",
            marker: "unused",
        }
    }

    #[test]
    fn isolated_and_simultaneous_reachability_is_bounded() {
        let stage = development_stage(1);
        let row = evaluate_row(stage, 0);
        assert_eq!(row.outcomes[1].firings, [1, 0, 1, 1]);
        assert_eq!(row.outcomes[2].firings, [0, 1, 1, 1]);
        assert_eq!(row.outcomes[3].firings, [1, 1, 1, 1]);
        assert!(row.refractory_suppressed);
        assert!(row.passed);
    }

    #[test]
    fn skew_exposes_two_pulses_without_runaway() {
        let stage = development_stage(7);
        let row = evaluate_row(stage, 6);
        assert_eq!(row.spec.skew, 1);
        assert_eq!(row.outcomes[3].firings, [1, 1, 2, 2]);
        assert!(!row.refractory_suppressed);
        assert!(row.passed);
    }

    #[test]
    fn physical_controls_and_replay_are_exact() {
        let stage = development_stage(8);
        for seed in 0..stage.rows {
            let row = evaluate_row(stage, seed);
            assert!(row.claims.0[5]);
            assert!(row.claims.0[6]);
            assert!(row.claims.0[7]);
            assert!(row.claims.0[8]);
            assert!(row.claims.0[10]);
            assert!(row.claims.0[11]);
            assert!(row.claims.0[12]);
            assert!(row.claims.0[13]);
            assert!(row.passed);
        }
    }
}
