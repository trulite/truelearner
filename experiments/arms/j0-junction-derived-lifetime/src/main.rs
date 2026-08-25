#![forbid(unsafe_code)]

#[cfg(all(feature = "cc0-model", feature = "junction-model"))]
compile_error!("J0 discriminator models must be compiled separately");
#[cfg(not(any(feature = "cc0-model", feature = "junction-model")))]
compile_error!("select exactly one J0 discriminator model");

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowId, ArrowRef, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig,
    PhysicalEvent, PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [7_700_000, 7_800_001];
const PHASES: std::ops::Range<i64> = 0..10;
const EXPECTED_CASES: usize = 160;
const EXPECTED_ROWS: usize = 320;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Model {
    #[cfg(feature = "cc0-model")]
    Cc0,
    #[cfg(feature = "junction-model")]
    Junction,
}

const fn model() -> Model {
    #[cfg(feature = "cc0-model")]
    {
        Model::Cc0
    }
    #[cfg(feature = "junction-model")]
    {
        Model::Junction
    }
}

impl Model {
    fn name(self) -> &'static str {
        match self {
            #[cfg(feature = "cc0-model")]
            Self::Cc0 => "cc0_model",
            #[cfg(feature = "junction-model")]
            Self::Junction => "junction_model",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Useful,
    Unsupported,
    OneSurvives,
    AllGone,
    Reuse,
    TwoIncoming,
    OneIncoming,
    NearbyUnrelated,
}

impl Family {
    const ALL: [Self; 8] = [
        Self::Useful,
        Self::Unsupported,
        Self::OneSurvives,
        Self::AllGone,
        Self::Reuse,
        Self::TwoIncoming,
        Self::OneIncoming,
        Self::NearbyUnrelated,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Useful => "useful_two_link_relation",
            Self::Unsupported => "unsupported_relation",
            Self::OneSurvives => "one_incident_arrow_survives",
            Self::AllGone => "all_incident_arrows_gone",
            Self::Reuse => "generation_safe_reuse",
            Self::TwoIncoming => "two_participating_incoming",
            Self::OneIncoming => "one_incoming_not_participating",
            Self::NearbyUnrelated => "nearby_unrelated_topology",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    arrow_updates: u64,
    cell_updates: u64,
    arrow_deallocations: u64,
    cell_deallocations: u64,
}

impl WorkTotals {
    fn add(&mut self, work: Work) {
        self.physical = self.physical.saturating_add(work.physical_total());
        self.drive = self.drive.saturating_add(work.drive_deliveries);
        self.modulation = self.modulation.saturating_add(work.modulatory_deliveries);
        self.arrow_updates = self.arrow_updates.saturating_add(work.local_return_updates);
        #[cfg(feature = "cc0-model")]
        {
            self.cell_updates = self.cell_updates.saturating_add(work.cell_return_updates);
        }
        self.arrow_deallocations = self
            .arrow_deallocations
            .saturating_add(work.physical_deallocations);
        self.cell_deallocations = self
            .cell_deallocations
            .saturating_add(work.cell_deallocations);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    markers: Vec<String>,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    final_tick: i64,
    body_hash: String,
    naturally_quiescent: bool,
    checks: Vec<(String, bool)>,
}

impl Observation {
    fn passed(&self) -> bool {
        self.checks.iter().all(|(_, passed)| *passed)
    }

    fn failed_names(&self) -> String {
        self.checks
            .iter()
            .filter(|(_, passed)| !passed)
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join("|")
    }
}

struct World {
    body: PlasticSubstrate,
    origin: i64,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    naturally_quiescent: bool,
    next_physical: u64,
}

impl World {
    fn new(root: u64, phase: i64, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 32, 64, mechanics);
        body.set_physical_tracing(true);
        body.advance_time(phase);
        Self {
            body,
            origin: phase,
            trace: Vec::new(),
            work: WorkTotals::default(),
            naturally_quiescent: true,
            next_physical: root + 1,
        }
    }

    fn cell(&mut self, position: i32, threshold: i32, resistance: u32) -> CellId {
        let physical_id = self.next_physical;
        self.next_physical = self.next_physical.saturating_add(1);
        self.body.add_cell(CellSpec {
            physical_id,
            position,
            region: 0,
            threshold,
            resistance,
        })
    }

    fn arrow(
        &mut self,
        from: CellId,
        to: CellId,
        resistance: u32,
        mode: TransmissionMode,
    ) -> ArrowRef {
        let id = self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance,
            mode,
        });
        self.body.arrow_reference(id)
    }

    fn anchor(&mut self, cells: &[CellId]) -> CellId {
        let anchor = self.cell(1_000, 100, 500);
        for cell in cells {
            self.arrow(anchor, *cell, 500, TransmissionMode::Drive);
        }
        anchor
    }

    fn pulse(&mut self, target: CellId, age: i64) {
        self.pulse_many(&[target], age);
    }

    fn pulse_many(&mut self, targets: &[CellId], age: i64) {
        let inputs = targets
            .iter()
            .enumerate()
            .map(|(index, target)| SpikeInput {
                arrival_tick: self.origin.saturating_add(age),
                phase: 0,
                origin_physical: self
                    .next_physical
                    .saturating_add(10_000)
                    .saturating_add(u64::try_from(index).unwrap_or(u64::MAX)),
                target: *target,
                impulse: 1,
            })
            .collect::<Vec<_>>();
        let result = self.body.arrive(
            &inputs,
            i16::MAX,
        );
        self.trace.extend(result.physical_trace);
        self.work.add(result.work);
        self.naturally_quiescent &= result.naturally_quiescent;
    }

    fn advance_age(&mut self, age: i64) {
        let target = self.origin.saturating_add(age);
        while self.body.clock().tick < target {
            let result = self
                .body
                .advance_time_traced(self.body.clock().tick.saturating_add(1));
            self.trace.extend(result.physical_trace);
            self.work.add(result.work);
            self.naturally_quiescent &= result.naturally_quiescent;
        }
    }

    fn arrow_resistance(&self, reference: ArrowRef) -> Option<u32> {
        self.body
            .arena_body(1)
            .arrows
            .iter()
            .find(|arrow| arrow.id == reference.id)
            .map(|arrow| arrow.resistance)
    }

    fn arrow_update_count(&self, id: ArrowId) -> usize {
        self.trace
            .iter()
            .filter(|transition| {
                matches!(
                    transition.event,
                    PhysicalEvent::Resistance { arrow, .. } if arrow == id
                )
            })
            .count()
    }

    fn fire_count(&self, id: CellId) -> usize {
        self.trace
            .iter()
            .filter(
                |transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == id),
            )
            .count()
    }

    fn death_age(&self, id: CellId) -> Option<i64> {
        self.trace.iter().find_map(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::CellDeallocate { cell, .. } if cell == id
            )
            .then_some(transition.tick.saturating_sub(self.origin))
        })
    }

    fn finish(self, markers: Vec<String>, checks: Vec<(String, bool)>) -> Observation {
        Observation {
            markers,
            trace: self.trace,
            work: self.work,
            final_tick: self.body.clock().tick,
            body_hash: ContentHash::of(&self.body.canonical_body_bytes(1).unwrap()).to_string(),
            naturally_quiescent: self.naturally_quiescent,
            checks,
        }
    }
}

struct Chain {
    world: World,
    p: CellId,
    c: CellId,
    modulator: CellId,
    incoming: ArrowRef,
    outgoing: ArrowRef,
}

fn chain(
    root: u64,
    phase: i64,
    mechanics: MechanicalConfig,
    incoming_r: u32,
    outgoing_r: u32,
) -> Chain {
    let mut world = World::new(root, phase, mechanics);
    let p = world.cell(-100, 1, 500);
    let c = world.cell(0, 1, 1);
    let x = world.cell(100, 100, 500);
    let modulator = world.cell(200, 1, 500);
    world.anchor(&[p, x, modulator]);
    let incoming = world.arrow(p, c, incoming_r, TransmissionMode::Drive);
    let outgoing = world.arrow(c, x, outgoing_r, TransmissionMode::Drive);
    world.arrow(modulator, c, 500, TransmissionMode::Modulatory);
    Chain {
        world,
        p,
        c,
        modulator,
        incoming,
        outgoing,
    }
}

fn observe(family: Family, root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    match family {
        Family::Useful => observe_useful(root, phase, mechanics),
        Family::Unsupported => observe_unsupported(root, phase, mechanics),
        Family::OneSurvives => observe_one_survives(root, phase, mechanics),
        Family::AllGone => observe_all_gone(root, phase, mechanics),
        Family::Reuse => observe_reuse(root, phase, mechanics),
        Family::TwoIncoming => observe_two_incoming(root, phase, mechanics),
        Family::OneIncoming => observe_one_incoming(root, phase, mechanics),
        Family::NearbyUnrelated => observe_nearby(root, phase, mechanics),
    }
}

fn observe_useful(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let Chain {
        mut world,
        p,
        c,
        modulator,
        incoming,
        outgoing,
    } = chain(root, phase, mechanics, 1, 1);
    world.pulse(p, 0);
    world.pulse(modulator, 2);
    let cell_after = world.body.cell_resistance(c);
    let incoming_after = world.arrow_resistance(incoming);
    let outgoing_after = world.arrow_resistance(outgoing);
    let fires_before_probe = world.fire_count(c);
    world.advance_age(10);
    let live_at_10 = (
        world.body.cell_is_live(c),
        world.body.resolve_arrow(incoming).is_some(),
        world.body.resolve_arrow(outgoing).is_some(),
    );
    world.pulse(p, 10);
    let fires_after_probe = world.fire_count(c);
    let (model_checks, model_marker) = match model() {
        #[cfg(feature = "cc0-model")]
        Model::Cc0 => (
            vec![
                ("cc0_cell_consolidated".into(), cell_after == Some(4)),
                (
                    "cc0_incoming_not_consolidated".into(),
                    incoming_after == Some(1),
                ),
                (
                    "cc0_outgoing_consolidated".into(),
                    outgoing_after == Some(4),
                ),
                (
                    "cc0_relation_broken_at_10".into(),
                    live_at_10 == (Some(true), false, true),
                ),
                (
                    "cc0_probe_cannot_reexecute".into(),
                    fires_after_probe == fires_before_probe,
                ),
            ],
            "independent_cell_persistence",
        ),
        #[cfg(feature = "junction-model")]
        Model::Junction => (
            vec![
                (
                    "junction_cell_not_consolidated".into(),
                    cell_after == Some(1),
                ),
                (
                    "junction_incoming_consolidated".into(),
                    incoming_after == Some(4),
                ),
                (
                    "junction_outgoing_consolidated".into(),
                    outgoing_after == Some(4),
                ),
                (
                    "junction_relation_live_at_10".into(),
                    live_at_10 == (Some(true), true, true),
                ),
                (
                    "junction_probe_reexecutes".into(),
                    fires_after_probe == fires_before_probe + 1,
                ),
            ],
            "topology_derived_persistence",
        ),
    };
    let markers = vec![format!(
        "model={model_marker};cell={cell_after:?};incoming={incoming_after:?};outgoing={outgoing_after:?};live10={live_at_10:?};fires={fires_before_probe}->{fires_after_probe}"
    )];
    world.finish(markers, model_checks)
}

fn unsupported_world(
    root: u64,
    phase: i64,
    mechanics: MechanicalConfig,
    incoming_r: u32,
    outgoing_r: u32,
) -> (World, CellId, ArrowRef, ArrowRef) {
    let mut world = World::new(root, phase, mechanics);
    let p = world.cell(-100, 100, 500);
    let c = world.cell(0, 100, 1);
    let x = world.cell(100, 100, 500);
    world.anchor(&[p, x]);
    let incoming = world.arrow(p, c, incoming_r, TransmissionMode::Drive);
    let outgoing = world.arrow(c, x, outgoing_r, TransmissionMode::Drive);
    (world, c, incoming, outgoing)
}

fn observe_unsupported(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let (mut world, c, incoming, outgoing) = unsupported_world(root, phase, mechanics, 1, 1);
    world.advance_age(10);
    let state = (
        world.body.cell_is_live(c),
        world.body.resolve_arrow(incoming).is_some(),
        world.body.resolve_arrow(outgoing).is_some(),
        world.death_age(c),
    );
    let markers = vec![format!("state={state:?}")];
    let checks = vec![
        ("unsupported_incoming_died".into(), !state.1),
        ("unsupported_outgoing_died".into(), !state.2),
        (
            "orphan_junction_died_at_10".into(),
            state.0 == Some(false) && state.3 == Some(10),
        ),
    ];
    world.finish(markers, checks)
}

fn observe_one_survives(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let (mut world, c, incoming, outgoing) = unsupported_world(root, phase, mechanics, 4, 1);
    world.advance_age(10);
    let state = (
        world.body.cell_is_live(c),
        world.body.resolve_arrow(incoming).is_some(),
        world.body.resolve_arrow(outgoing).is_some(),
    );
    let (name, expected) = match model() {
        #[cfg(feature = "cc0-model")]
        Model::Cc0 => ("cc0_cell_dies_independently", (Some(false), true, false)),
        #[cfg(feature = "junction-model")]
        Model::Junction => ("junction_kept_by_one_incident", (Some(true), true, false)),
    };
    let markers = vec![format!("state={state:?}")];
    world.finish(markers, vec![(name.into(), state == expected)])
}

fn observe_all_gone(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let (mut world, c, incoming, outgoing) = unsupported_world(root, phase, mechanics, 1, 1);
    world.advance_age(10);
    let checks = vec![
        (
            "last_incident_gone".into(),
            world.body.resolve_arrow(incoming).is_none()
                && world.body.resolve_arrow(outgoing).is_none(),
        ),
        (
            "junction_deallocated".into(),
            world.body.cell_is_live(c) == Some(false),
        ),
        (
            "death_at_last_incident".into(),
            world.death_age(c) == Some(10),
        ),
    ];
    let death = world.death_age(c);
    world.finish(vec![format!("death={death:?}")], checks)
}

fn observe_reuse(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let (mut world, c, _, _) = unsupported_world(root, phase, mechanics, 1, 1);
    let old_ref = world.body.cell_reference(c);
    let old_slot = world.body.cell_resident_slot(c);
    world.advance_age(10);
    let replacement = world.cell(0, 1, 10);
    let replacement_ref = world.body.cell_reference(replacement);
    let replacement_slot = world.body.cell_resident_slot(replacement);
    let checks = vec![
        (
            "old_reference_stale".into(),
            world.body.resolve_cell(old_ref).is_none(),
        ),
        ("slot_reused".into(), old_slot == replacement_slot),
        ("fresh_cell_id".into(), replacement != c),
        (
            "generation_advanced".into(),
            replacement_ref.generation.0 == old_ref.generation.0 + 1,
        ),
        (
            "replacement_resolves".into(),
            world.body.resolve_cell(replacement_ref) == replacement_slot,
        ),
    ];
    world.finish(
        vec![format!(
            "old={old_ref:?}/{old_slot:?};new={replacement_ref:?}/{replacement_slot:?}"
        )],
        checks,
    )
}

struct IncomingFixture {
    world: World,
    p1: CellId,
    p2: CellId,
    c: CellId,
    modulator: CellId,
    incoming1: ArrowRef,
    incoming2: ArrowRef,
    outgoing: ArrowRef,
}

fn incoming_fixture(root: u64, phase: i64, mechanics: MechanicalConfig) -> IncomingFixture {
    let mut world = World::new(root, phase, mechanics);
    let p1 = world.cell(-100, 1, 500);
    let p2 = world.cell(-110, 1, 500);
    let c = world.cell(0, 1, 1);
    let x = world.cell(100, 100, 500);
    let modulator = world.cell(200, 1, 500);
    world.anchor(&[p1, p2, x, modulator]);
    let incoming1 = world.arrow(p1, c, 1, TransmissionMode::Drive);
    let incoming2 = world.arrow(p2, c, 1, TransmissionMode::Drive);
    let outgoing = world.arrow(c, x, 1, TransmissionMode::Drive);
    world.arrow(modulator, c, 500, TransmissionMode::Modulatory);
    IncomingFixture {
        world,
        p1,
        p2,
        c,
        modulator,
        incoming1,
        incoming2,
        outgoing,
    }
}

fn observe_two_incoming(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let IncomingFixture {
        mut world,
        p1,
        p2,
        c,
        modulator,
        incoming1,
        incoming2,
        outgoing,
    } = incoming_fixture(root, phase, mechanics);
    world.pulse_many(&[p1, p2], 0);
    world.pulse(modulator, 2);
    let state = (
        world.arrow_resistance(incoming1),
        world.arrow_resistance(incoming2),
        world.arrow_resistance(outgoing),
        world.body.cell_resistance(c),
    );
    let expected = match model() {
        #[cfg(feature = "cc0-model")]
        Model::Cc0 => (Some(1), Some(1), Some(4), Some(4)),
        #[cfg(feature = "junction-model")]
        Model::Junction => (Some(4), Some(4), Some(4), Some(1)),
    };
    #[cfg(feature = "cc0-model")]
    let expected_incoming_updates = 0;
    #[cfg(feature = "junction-model")]
    let expected_incoming_updates = 2;
    let markers = vec![format!("state={state:?}")];
    let checks = vec![
        ("model_specific_two_incoming".into(), state == expected),
        (
            "both_incoming_participated".into(),
            world.arrow_update_count(incoming1.id) + world.arrow_update_count(incoming2.id)
                == expected_incoming_updates,
        ),
    ];
    world.finish(markers, checks)
}

fn observe_one_incoming(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let IncomingFixture {
        mut world,
        p1,
        p2: _,
        c,
        modulator,
        incoming1,
        incoming2,
        outgoing,
    } = incoming_fixture(root, phase, mechanics);
    world.pulse(p1, 0);
    world.pulse(modulator, 2);
    let state = (
        world.arrow_resistance(incoming1),
        world.arrow_resistance(incoming2),
        world.arrow_resistance(outgoing),
        world.body.cell_resistance(c),
    );
    let expected = match model() {
        #[cfg(feature = "cc0-model")]
        Model::Cc0 => (Some(1), Some(1), Some(4), Some(4)),
        #[cfg(feature = "junction-model")]
        Model::Junction => (Some(4), Some(1), Some(4), Some(1)),
    };
    let checks = vec![
        ("model_specific_one_incoming".into(), state == expected),
        ("unused_incoming_unchanged".into(), state.1 == Some(1)),
    ];
    world.finish(vec![format!("state={state:?}")], checks)
}

fn observe_nearby(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let Chain {
        mut world,
        p,
        c,
        modulator,
        incoming,
        outgoing,
    } = chain(root, phase, mechanics, 1, 1);
    let d = world.cell(10, 1, 500);
    let e = world.cell(20, 100, 500);
    world.anchor(&[d, e]);
    let unrelated = world.arrow(d, e, 1, TransmissionMode::Drive);
    world.pulse_many(&[p, d], 0);
    world.pulse(modulator, 2);
    let state = (
        world.arrow_resistance(incoming),
        world.arrow_resistance(outgoing),
        world.arrow_resistance(unrelated),
        world.body.cell_resistance(c),
    );
    let expected = match model() {
        #[cfg(feature = "cc0-model")]
        Model::Cc0 => (Some(1), Some(4), Some(1), Some(4)),
        #[cfg(feature = "junction-model")]
        Model::Junction => (Some(4), Some(4), Some(1), Some(1)),
    };
    let checks = vec![
        ("model_specific_local_updates".into(), state == expected),
        ("nearby_unrelated_unchanged".into(), state.2 == Some(1)),
    ];
    world.finish(vec![format!("state={state:?}")], checks)
}

fn mechanics_name(config: MechanicalConfig) -> &'static str {
    if config == MechanicalConfig::REFERENCE {
        "reference"
    } else if config == MechanicalConfig::PRODUCTION {
        "production"
    } else {
        "unknown"
    }
}

fn main() {
    let output_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/j0_junction_derived_lifetime_v1"));
    fs::create_dir_all(&output_dir).unwrap();
    let model = model();
    let mut csv = String::from(
        "case,model,family,root,phase,mechanics,replay_equal,cross_mechanics_equal,checks_pass,failed,cell_updates,arrow_updates,cell_deallocations,arrow_deallocations,physical_work,trace_len,final_tick,naturally_quiescent,body_hash,markers\n",
    );
    let mut cases = 0usize;
    let mut rows = 0usize;
    let mut clauses = 0usize;
    let mut passed_clauses = 0usize;
    let mut all_pass = true;
    let mut maximum_work = 0u64;

    for root in ROOTS {
        for phase in PHASES {
            for family in Family::ALL {
                cases += 1;
                let reference = observe(family, root, phase, MechanicalConfig::REFERENCE);
                let reference_replay = observe(family, root, phase, MechanicalConfig::REFERENCE);
                let production = observe(family, root, phase, MechanicalConfig::PRODUCTION);
                let production_replay = observe(family, root, phase, MechanicalConfig::PRODUCTION);
                let cross_equal = reference == production;
                for (config, observation, replay) in [
                    (MechanicalConfig::REFERENCE, &reference, &reference_replay),
                    (
                        MechanicalConfig::PRODUCTION,
                        &production,
                        &production_replay,
                    ),
                ] {
                    rows += 1;
                    let replay_equal = observation == replay;
                    let check_count = observation.checks.len().saturating_add(3);
                    let row_pass = observation.passed()
                        && replay_equal
                        && cross_equal
                        && observation.naturally_quiescent;
                    clauses = clauses.saturating_add(check_count);
                    passed_clauses = passed_clauses.saturating_add(
                        observation.checks.iter().filter(|(_, pass)| *pass).count()
                            + usize::from(replay_equal)
                            + usize::from(cross_equal)
                            + usize::from(observation.naturally_quiescent),
                    );
                    all_pass &= row_pass;
                    maximum_work = maximum_work.max(observation.work.physical);
                    writeln!(
                        csv,
                        "{cases},{},{},{root},{phase},{},{replay_equal},{cross_equal},{},{},{},{},{},{},{},{},{},{},{},{}",
                        model.name(),
                        family.name(),
                        mechanics_name(config),
                        observation.passed(),
                        observation.failed_names(),
                        observation.work.cell_updates,
                        observation.work.arrow_updates,
                        observation.work.cell_deallocations,
                        observation.work.arrow_deallocations,
                        observation.work.physical,
                        observation.trace.len(),
                        observation.final_tick,
                        observation.naturally_quiescent,
                        observation.body_hash,
                        observation.markers.join("|").replace(',', ";"),
                    )
                    .unwrap();
                }
            }
        }
    }

    assert_eq!(cases, EXPECTED_CASES);
    assert_eq!(rows, EXPECTED_ROWS);
    let report = format!(
        "# J0 junction-derived lifetime: {}\n\n- cases: {cases}/{EXPECTED_CASES}\n- rows: {rows}/{EXPECTED_ROWS}\n- clauses: {passed_clauses}/{clauses}\n- Reference/Production exact: {all_pass}\n- replay exact: {all_pass}\n- natural quiescence: {all_pass}\n- maximum PhysicalWork: {maximum_work}\n",
        model.name(),
    );
    fs::write(output_dir.join("matrix.csv"), csv).unwrap();
    fs::write(output_dir.join("report.md"), report).unwrap();
    assert!(all_pass, "J0 {} discriminator arm failed", model.name());
    println!("J0_{}_POSITIVE_V1", model.name().to_ascii_uppercase());
}
