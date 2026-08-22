//! Development-only post-M7 DS5 physical closure-emission successor.

use std::collections::{BTreeSet, HashSet, VecDeque};

use crate::binding::{IdentitySource, OpaqueId};
use crate::iteration::frozen_iterable_operation;
use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "post-m7-ds5-closure-emission-v1";
pub const AUTHORITATIVE_M7: &str = "b607ed52f640a3e202da3cc6b73ac58b180caf83";
pub const PROBE_V1_SEED: u64 = 880_000_000;
pub const PROBE_RETRY_SEED: u64 = 881_000_000;
pub const MICRO_SEEDS: [u64; 2] = [882_000_000, 882_500_000];
pub const GATE_SEEDS: [u64; 6] = [
    884_000_000,
    884_500_000,
    885_000_000,
    885_500_000,
    886_000_000,
    886_500_000,
];
pub const FROZEN_PROTOCOL_SHA256: &str =
    "140d2392263359c666f364e1923f956a4b2b09e4107a0fd2f7f8f469d97be154";
pub const FROZEN_M7_HANDOFF_SHA256: &str =
    "b4a9012f8fbbb1fa8fdfd36921a82e162c73f4c2175c809bd48c0dae78e45520";
pub const FROZEN_M7_CSV_SHA256: &str =
    "13619c786471b34f5dc9da914c4a0f454bab8d95a87142ce6c9e35808f3dd91a";
pub const FROZEN_M7_MD_SHA256: &str =
    "d1f4d3dc6c944b8ab146a121b0fb0df7d6270b3d4363ca6d4e18b8b53925b1cd";
pub const FROZEN_M7_SOURCE_SHA256: &str =
    "67e170f12d7b7649a0a291ddfc16cd80e4b5c15564b65cd09c884f3e52b9ac5b";
pub const FROZEN_V20_SOURCE_SHA256: &str =
    "8a17e7a5fda9519ad0d4a9d29d04d2434dd5b5ee857e74c1296c5f8b3b06f897";
pub const FROZEN_V20_RESULT_SHA256: &str =
    "468d50db53ed7451f2680621b85067a046fccba3c8ef19097dabaec22a3806b4";
pub const FROZEN_V21_SOURCE_SHA256: &str =
    "85230e7b6b0d669a3b2e163f3e281975c9fbd5d98709b923efff418d36ff9f1a";
pub const FROZEN_V21A_RESULT_SHA256: &str =
    "0f4a30c378aba506492351588412aab71ba6c174ef358d2d0758c82ec87cfc20";
pub const FROZEN_V21B_RESULT_SHA256: &str =
    "ca4f2ffb8b77ac237bfce19d66d21820d26d34b727bbf95262003dffd93ad300";
pub const FROZEN_BOUNDARY_SOURCE_SHA256: &str =
    "3eb802f394a225a4ad7f0938b4a672723da2c1303ff95e805423de8161057527";
pub const FROZEN_CLOSURE_SOURCE_SHA256: &str =
    "860e89304e86f254dd02a5aa35cf63cc240af160039b4166fa0cb5856dacb84a";
pub const FROZEN_RETURN_SOURCE_SHA256: &str =
    "f17afa482bf345eb680463f7418b6b6c2553cd78eab9b4dbfce74f7ca1483d51";
pub const FROZEN_M5_SOURCE_SHA256: &str =
    "e755a70deada891e5c4db3b55809ca84ea8ad31a8bd3affe564bf08a95f8dff7";
pub const FROZEN_M6_SOURCE_SHA256: &str =
    "11b4229122b3e0788ca30c55579b91ffe07461de9a138860690134565fcf2ed6";
pub const FROZEN_PROBE_V1_RESULT_SHA256: &str =
    "1e2faea63db4b165a4e35b3f9fdccccb373eecbe4b5c173102612593cf4716c4";
pub const FROZEN_PROBE_RETRY_RESULT_SHA256: &str =
    "c947e50e77fb56d696a53eb1b4f125f5710405fc708e1071d0056579cb52a085";
pub const FROZEN_MICRO_RESULT_SHA256: &str =
    "21c716c87b364e4611d11773d0ff4a914e0d19325ce3b90084be146d8c891e2c";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeV1Report {
    pub protocol: &'static str,
    pub seed: u64,
    pub claim_eligible: bool,
    pub expected_negative: bool,
    pub exact_m7: bool,
    pub protocol_frozen: bool,
    pub frozen_parts: bool,
    pub physical_closure_path: bool,
    pub terminal_supervision_sites: usize,
    pub semantic_population_sites: usize,
    pub lawful_m6_links: usize,
    pub lawful_updates: usize,
    pub first_collapse: &'static str,
}

pub fn run_probe_v1() -> ProbeV1Report {
    let v21 = include_str!("continuation.rs");
    let old_finish = v21
        .split("pub fn run_finish_experiment")
        .nth(1)
        .unwrap_or_default();
    let terminal_supervision_sites = old_finish.matches("learn_finish_from_terminal").count();
    let semantic_population_sites = [
        "NO_RESULT_CELL_ID",
        "EXPLICIT_ANSWER_CELL_ID",
        "SemanticEvent::NoResult",
        "FinishRoute::AnswerCurrent",
    ]
    .iter()
    .map(|site| v21.matches(site).count())
    .sum();
    let exact_m7 = exact_m7();
    let protocol_frozen = env!("POST_M7_DS5_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256;
    let frozen_parts = frozen_parts_exact();
    let physical_closure_path = v21.contains("self.lookup_outputs.len()")
        && v21.contains("self.current")
        && v21.contains("remaining_queued_spikes");
    let frozen_negative =
        env!("POST_M7_DS5_PROBE_V1_RESULT_SHA256") == FROZEN_PROBE_V1_RESULT_SHA256;
    let expected_negative = exact_m7
        && protocol_frozen
        && frozen_parts
        && frozen_negative
        && physical_closure_path
        && terminal_supervision_sites > 0
        && semantic_population_sites > 0;
    ProbeV1Report {
        protocol: PROTOCOL,
        seed: PROBE_V1_SEED,
        claim_eligible: false,
        expected_negative,
        exact_m7,
        protocol_frozen,
        frozen_parts,
        physical_closure_path,
        terminal_supervision_sites,
        semantic_population_sites,
        lawful_m6_links: 0,
        lawful_updates: 0,
        first_collapse:
            "V21b terminal answer supervision; lawful M6 closure-to-active-trace edge absent",
    }
}

fn exact_m7() -> bool {
    AUTHORITATIVE_M7 == "b607ed52f640a3e202da3cc6b73ac58b180caf83"
        && env!("POST_M7_DS5_M7_HANDOFF_SHA256") == FROZEN_M7_HANDOFF_SHA256
        && env!("POST_M7_DS5_M7_CSV_SHA256") == FROZEN_M7_CSV_SHA256
        && env!("POST_M7_DS5_M7_MD_SHA256") == FROZEN_M7_MD_SHA256
        && env!("POST_M7_DS5_M7_SOURCE_SHA256") == FROZEN_M7_SOURCE_SHA256
}

fn frozen_parts_exact() -> bool {
    env!("POST_M7_DS5_V20_SOURCE_SHA256") == FROZEN_V20_SOURCE_SHA256
        && env!("POST_M7_DS5_V20_RESULT_SHA256") == FROZEN_V20_RESULT_SHA256
        && env!("POST_M7_DS5_V21_SOURCE_SHA256") == FROZEN_V21_SOURCE_SHA256
        && env!("POST_M7_DS5_V21A_RESULT_SHA256") == FROZEN_V21A_RESULT_SHA256
        && env!("POST_M7_DS5_V21B_RESULT_SHA256") == FROZEN_V21B_RESULT_SHA256
        && env!("POST_M7_DS5_BOUNDARY_SOURCE_SHA256") == FROZEN_BOUNDARY_SOURCE_SHA256
        && env!("POST_M7_DS5_CLOSURE_SOURCE_SHA256") == FROZEN_CLOSURE_SOURCE_SHA256
        && env!("POST_M7_DS5_RETURN_SOURCE_SHA256") == FROZEN_RETURN_SOURCE_SHA256
        && env!("POST_M7_DS5_M5_SOURCE_SHA256") == FROZEN_M5_SOURCE_SHA256
        && env!("POST_M7_DS5_M6_SOURCE_SHA256") == FROZEN_M6_SOURCE_SHA256
}

#[allow(dead_code)]
mod frozen_cumulative {
    include!(concat!(
        env!("OUT_DIR"),
        "/post_m6_ds4_arrival_initiation_frozen.rs"
    ));

    pub(super) struct Credit(frozen_m6::CreditGate);

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(super) struct State {
        pub(super) observations: u64,
        pub(super) updates: usize,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(super) struct Controls {
        pub(super) passed: bool,
    }

    pub(super) fn credit(seed: u64) -> Credit {
        Credit(frozen_m6::credit_gate(seed))
    }

    pub(super) fn differential(credit: &mut Credit, activity: bool) -> bool {
        frozen_m6::apply_recurrence(&mut credit.0, activity)
    }

    pub(super) fn state(credit: &Credit) -> State {
        let state = frozen_m6::credit_state(&credit.0);
        State {
            observations: state.observations,
            updates: state.updates,
        }
    }

    pub(super) fn controls(seed: u64) -> Controls {
        Controls {
            passed: frozen_m6::controls(seed).passed(),
        }
    }
}

#[allow(dead_code)]
mod frozen_event {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds4_cumulative_request_start_port.rs"
    ));

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(super) struct Activity {
        pub(super) completion: usize,
        pub(super) learned: usize,
        pub(super) generic: usize,
        pub(super) work: u64,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(super) enum Fixture {
        Standard,
        Relabelled,
        Missing,
        Invalid,
    }

    pub(super) struct Gate(frozen_m3::Ds4EventGate);

    pub(super) fn gate(seed: u64) -> Option<Gate> {
        frozen_m3::ds4_event_gate(seed, 2).map(Gate)
    }

    pub(super) fn activity(gate: &mut Gate, seed: u64, fixture: Fixture) -> Activity {
        let fixture = match fixture {
            Fixture::Standard => EventFixture::Standard,
            Fixture::Relabelled => EventFixture::Relabelled,
            Fixture::Missing => EventFixture::MissingClose,
            Fixture::Invalid => EventFixture::InvalidTransition,
        };
        let activity = frozen_m3::event_completion_activity(&mut gate.0, seed, fixture);
        Activity {
            completion: activity.completion_spikes,
            learned: activity.learned_uses,
            generic: activity.generic_spans,
            work: activity.physical_work,
        }
    }

    pub(super) fn state(gate: &Gate) -> (usize, usize) {
        let state = frozen_m3::ds4_event_state(&gate.0);
        (state.chunks, state.persistent_bytes)
    }
}

#[allow(dead_code)]
mod anonymous_role {
    include!(concat!(env!("OUT_DIR"), "/request_roles.rs"));

    use super::frozen_cumulative;

    pub(super) struct Session {
        learner: RequestRoleLearner,
        identities: IdentitySource,
        rng: DeterministicRng,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(super) struct Step {
        pub(super) selected: bool,
        pub(super) connected: bool,
        pub(super) crossing: Option<OpaqueId>,
        pub(super) consequence: bool,
        pub(super) update: bool,
        pub(super) position: usize,
    }

    pub(super) fn session(seed: u64) -> Session {
        Session {
            learner: RequestRoleLearner::new(40, seed + 10),
            identities: IdentitySource::new(seed + 20),
            rng: DeterministicRng::new(seed + 30),
        }
    }

    pub(super) fn step(
        session: &mut Session,
        credit: &mut frozen_cumulative::Credit,
        activity: usize,
        carried: OpaqueId,
        acquire: bool,
        connected: bool,
        symmetric: bool,
    ) -> Step {
        let occurrence = session.identities.issue();
        let family = if symmetric {
            RequestEncodingFamily::Symmetric
        } else if acquire {
            RequestEncodingFamily::Training
        } else {
            RequestEncodingFamily::Transferred
        };
        let encoded = request_encoding(
            &mut session.identities,
            &mut session.rng,
            occurrence,
            family,
        );
        if acquire {
            session.learner.observe(&encoded.occurrences);
        }
        if activity == 0 {
            return Step {
                position: encoded.target_position,
                ..Step::default()
            };
        }
        let choice = if acquire {
            session.learner.choose(&encoded.occurrences)
        } else {
            session.learner.evaluated(&encoded.occurrences)
        };
        let selected_occurrence = match choice.outcome {
            BindingOutcome::Answer(identity) => Some(identity),
            BindingOutcome::NotFound | BindingOutcome::Ambiguous => None,
        };
        let selected = choice.pattern_cell.is_some();
        let connected = connected && selected_occurrence == Some(occurrence);

        // POST_M7_DS5_M6_GATE_BEGIN
        let consequence = acquire && connected;
        let update = consequence && frozen_cumulative::differential(credit, connected);
        if update {
            session.learner.feedback(choice.pattern_cell, update);
        }
        // POST_M7_DS5_M6_GATE_END

        Step {
            selected,
            connected,
            crossing: connected.then_some(carried),
            consequence,
            update,
            position: encoded.target_position,
        }
    }

    pub(super) fn ready(session: &Session) -> bool {
        session.learner.target_role(request_signature(0)).is_some()
    }

    pub(super) fn roles(session: &Session) -> usize {
        session.learner.consolidated_cells().len()
    }

    pub(super) fn fingerprint(session: &Session) -> u64 {
        session.learner.fingerprint()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ClosureFixture {
    Standard,
    Relabelled,
    Missing,
    Invalid,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RunOptions {
    connected: bool,
    initiate: bool,
    acquire: bool,
    closure_fixture: Option<ClosureFixture>,
}

#[derive(Clone, Debug)]
struct Episode {
    relations: Vec<(OpaqueId, OpaqueId)>,
    query: OpaqueId,
    terminal: OpaqueId,
}

#[derive(Clone, Debug)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.state
    }

    fn shuffle<T>(&mut self, values: &mut [T]) {
        for index in (1..values.len()).rev() {
            let selected = self.next_u64() as usize % (index + 1);
            values.swap(index, selected);
        }
    }
}

fn chain_episode(
    identities: &mut IdentitySource,
    rng: &mut DeterministicRng,
    depth: usize,
    distractors: usize,
) -> Episode {
    let chain = (0..=depth).map(|_| identities.issue()).collect::<Vec<_>>();
    let mut relations = chain
        .windows(2)
        .map(|pair| (pair[0], pair[1]))
        .collect::<Vec<_>>();
    for _ in 0..distractors {
        relations.push((identities.issue(), identities.issue()));
    }
    rng.shuffle(&mut relations);
    Episode {
        relations,
        query: chain[0],
        terminal: *chain.last().unwrap(),
    }
}

fn duplicate_episode(identities: &mut IdentitySource) -> Episode {
    let query = identities.issue();
    let terminal = identities.issue();
    Episode {
        relations: vec![(query, terminal), (query, terminal)],
        query,
        terminal,
    }
}

fn branch_episode(identities: &mut IdentitySource) -> Episode {
    let query = identities.issue();
    Episode {
        relations: vec![(query, identities.issue()), (query, identities.issue())],
        query,
        terminal: query,
    }
}

fn cycle_episode(identities: &mut IdentitySource) -> Episode {
    let first = identities.issue();
    let second = identities.issue();
    Episode {
        relations: vec![(first, second), (second, first)],
        query: first,
        terminal: first,
    }
}

struct LearnedInitiation {
    event: frozen_event::Gate,
    role: anonymous_role::Session,
    credit: frozen_cumulative::Credit,
    identities: IdentitySource,
}

impl LearnedInitiation {
    fn acquire(seed: u64) -> Option<(Self, usize)> {
        let event = frozen_event::gate(seed)?;
        let mut gate = Self {
            event,
            role: anonymous_role::session(seed + 10_000),
            credit: frozen_cumulative::credit(seed + 20_000),
            identities: IdentitySource::new(seed + 30_000),
        };
        let mut competence = 0;
        for episode in 1..=4_000usize {
            let activity = frozen_event::activity(
                &mut gate.event,
                seed + episode as u64,
                frozen_event::Fixture::Standard,
            );
            let carried = gate.identities.issue();
            let _ = anonymous_role::step(
                &mut gate.role,
                &mut gate.credit,
                activity.completion,
                carried,
                true,
                true,
                false,
            );
            if anonymous_role::ready(&gate.role) {
                competence = episode;
                break;
            }
        }
        (competence > 0).then_some((gate, competence))
    }

    fn activate(&mut self, seed: u64, relabelled: bool) -> bool {
        let fixture = if relabelled {
            frozen_event::Fixture::Relabelled
        } else {
            frozen_event::Fixture::Standard
        };
        let activity = frozen_event::activity(&mut self.event, seed, fixture);
        let carried = self.identities.issue();
        anonymous_role::step(
            &mut self.role,
            &mut self.credit,
            activity.completion,
            carried,
            false,
            true,
            false,
        )
        .connected
    }

    fn fingerprint(&self) -> (u64, (usize, usize), frozen_cumulative::State) {
        (
            anonymous_role::fingerprint(&self.role),
            frozen_event::state(&self.event),
            frozen_cumulative::state(&self.credit),
        )
    }
}

struct ClosurePath {
    event: frozen_event::Gate,
    role: anonymous_role::Session,
    credit: frozen_cumulative::Credit,
}

impl ClosurePath {
    fn new(seed: u64) -> Option<Self> {
        Some(Self {
            event: frozen_event::gate(seed)?,
            role: anonymous_role::session(seed + 10_000),
            credit: frozen_cumulative::credit(seed + 20_000),
        })
    }

    fn fingerprint(&self) -> (u64, (usize, usize), frozen_cumulative::State) {
        (
            anonymous_role::fingerprint(&self.role),
            frozen_event::state(&self.event),
            frozen_cumulative::state(&self.credit),
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct PhysicalRun {
    crossings: Vec<OpaqueId>,
    traversals: usize,
    attempts: usize,
    closures: usize,
    premature: usize,
    selections: usize,
    consequences: usize,
    updates: usize,
    position: Option<usize>,
    ambiguous: bool,
    cycle_blocked: bool,
    naturally_quiescent: bool,
    temporary_erased: bool,
    physical_work: u64,
}

fn execute(
    episode: &Episode,
    path: &mut ClosurePath,
    seed: u64,
    options: RunOptions,
) -> PhysicalRun {
    let mut run = PhysicalRun::default();
    if !options.initiate {
        run.naturally_quiescent = true;
        return run;
    }
    let mut operation = frozen_iterable_operation();
    operation.lookup.begin_episode();
    for &(left, right) in &episode.relations {
        operation.lookup.observe_relation(left, right);
    }
    let mut queue = VecDeque::from([episode.query]);
    let mut history = HashSet::new();
    while let Some(current) = queue.pop_front() {
        if !history.insert(current) {
            run.cycle_blocked = true;
            break;
        }
        run.attempts += 1;
        let mut successors = HashSet::new();
        for cell in operation.lookup.temporary_identity_cells() {
            run.physical_work += 1;
            if let Some(successor) = operation.lookup.compare_temporary_identity(current, cell) {
                successors.insert(successor);
            }
        }
        match successors.len() {
            0 => {
                run.closures += 1;
                let fixture = match options.closure_fixture.unwrap_or(ClosureFixture::Standard) {
                    ClosureFixture::Standard => frozen_event::Fixture::Standard,
                    ClosureFixture::Relabelled => frozen_event::Fixture::Relabelled,
                    ClosureFixture::Missing => frozen_event::Fixture::Missing,
                    ClosureFixture::Invalid => frozen_event::Fixture::Invalid,
                };
                let activity = frozen_event::activity(&mut path.event, seed, fixture);
                run.physical_work += activity.work;
                let step = anonymous_role::step(
                    &mut path.role,
                    &mut path.credit,
                    activity.completion,
                    current,
                    options.acquire,
                    options.connected,
                    false,
                );
                run.selections += usize::from(step.selected);
                run.consequences += usize::from(step.consequence);
                run.updates += usize::from(step.update);
                run.position = Some(step.position);
                if let Some(crossing) = step.crossing {
                    run.crossings.push(crossing);
                }
            }
            1 => {
                run.traversals += 1;
                queue.push_back(*successors.iter().next().unwrap());
            }
            _ => {
                run.ambiguous = true;
                break;
            }
        }
    }
    operation.lookup.erase_temporary();
    run.temporary_erased = operation.lookup.temporary_counts() == (0, 0)
        && operation.lookup.temporary_capacities() == (0, 0, 0);
    run.naturally_quiescent = queue.is_empty() && !run.ambiguous && !run.cycle_blocked;
    run
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub exact_m7: bool,
    pub protocol_frozen: bool,
    pub probe_negative_frozen: bool,
    pub probe_retry_frozen: bool,
    pub micro_frozen: bool,
    pub frozen_parts: bool,
    pub linker_exact: bool,
    pub information_boundary: bool,
    pub lane_isolated: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.exact_m7
            && self.protocol_frozen
            && self.probe_negative_frozen
            && self.probe_retry_frozen
            && self.micro_frozen
            && self.frozen_parts
            && self.linker_exact
            && self.information_boundary
            && self.lane_isolated
    }
}

fn source_audit() -> SourceAudit {
    let source = include_str!("post_m7_ds5_closure_emission.rs");
    let linker = source
        .split("// POST_M7_DS5_M6_GATE_BEGIN")
        .nth(1)
        .and_then(|tail| tail.split("// POST_M7_DS5_M6_GATE_END").next())
        .unwrap_or_default();
    let forbidden = [
        ["expected", "_answer"].concat(),
        ["episode", ".terminal"].concat(),
        ["semantic", "_credit"].concat(),
        ["finish", "_route"].concat(),
        ["no_", "result"].concat(),
        ["answer", "_identity"].concat(),
        ["remaining", "_steps"].concat(),
    ];
    let lane_fragments = [
        ["lane", "_b"].concat(),
        ["ss", "a0"].concat(),
        ["stochastic", "_superposition"].concat(),
    ];
    SourceAudit {
        exact_m7: exact_m7(),
        protocol_frozen: env!("POST_M7_DS5_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        probe_negative_frozen: env!("POST_M7_DS5_PROBE_V1_RESULT_SHA256")
            == FROZEN_PROBE_V1_RESULT_SHA256,
        probe_retry_frozen: env!("POST_M7_DS5_PROBE_RETRY_RESULT_SHA256")
            == FROZEN_PROBE_RETRY_RESULT_SHA256,
        micro_frozen: env!("POST_M7_DS5_MICRO_RESULT_SHA256") == FROZEN_MICRO_RESULT_SHA256,
        frozen_parts: frozen_parts_exact(),
        linker_exact: !linker.is_empty()
            && linker.matches("frozen_cumulative::differential").count() == 1
            && linker.matches("session.learner.feedback").count() == 1,
        information_boundary: forbidden.iter().all(|item| !linker.contains(item)),
        lane_isolated: lane_fragments
            .iter()
            .all(|fragment| !source.contains(fragment)),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Control {
    pub number: usize,
    pub name: &'static str,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Snapshot {
    source: SourceAudit,
    learners: usize,
    ready: usize,
    single_roles: usize,
    competence: Vec<usize>,
    m7_competence: Vec<usize>,
    m7_activations: usize,
    selections: usize,
    consequences: usize,
    updates: usize,
    observations: u64,
    crossings: usize,
    correct: usize,
    total: usize,
    quiescent: usize,
    positions: BTreeSet<usize>,
    depths: BTreeSet<usize>,
    physical_work: u64,
    m7_nonplastic: bool,
    closure_nonplastic: bool,
    temporary_erased: bool,
    namespaces: BTreeSet<u64>,
    held_out: BTreeSet<u64>,
    controls: Vec<Control>,
}

fn snapshot(seeds: &[u64], held_out_per_learner: usize) -> Snapshot {
    let source = source_audit();
    let mut ready = 0;
    let mut single_roles = 0;
    let mut competence = Vec::new();
    let mut m7_competence = Vec::new();
    let mut m7_activations = 0;
    let mut selections = 0;
    let mut consequences = 0;
    let mut updates = 0;
    let mut observations = 0;
    let mut crossings = 0;
    let mut correct = 0;
    let total = seeds.len() * held_out_per_learner;
    let mut quiescent = 0;
    let mut positions = BTreeSet::new();
    let mut depths = BTreeSet::new();
    let mut physical_work = 0;
    let mut m7_nonplastic = true;
    let mut closure_nonplastic = true;
    let mut temporary_erased = true;
    let mut namespaces = BTreeSet::new();
    let mut held_out = BTreeSet::new();
    let mut control_results = [true; 12];

    for &seed in seeds {
        namespaces.insert(seed);
        let Some((mut initiation, m7_at)) = LearnedInitiation::acquire(seed) else {
            continue;
        };
        m7_competence.push(m7_at);
        let Some(mut path) = ClosurePath::new(seed + 100_000) else {
            continue;
        };
        let mut identities = IdentitySource::new(seed + 200_000);
        let mut rng = DeterministicRng::new(seed + 300_000);
        let mut learned_at = None;
        for episode_index in 1..=4_000usize {
            let event_seed = seed + 1_000 + episode_index as u64;
            namespaces.insert(event_seed);
            let initiated = initiation.activate(event_seed, episode_index.is_multiple_of(2));
            m7_activations += usize::from(initiated);
            let depth = (episode_index - 1) % 5;
            let episode = chain_episode(&mut identities, &mut rng, depth, 3);
            let run = execute(
                &episode,
                &mut path,
                event_seed + 10_000,
                RunOptions {
                    connected: true,
                    initiate: initiated,
                    acquire: true,
                    closure_fixture: Some(ClosureFixture::Standard),
                },
            );
            selections += run.selections;
            consequences += run.consequences;
            updates += run.updates;
            crossings += run.crossings.len();
            physical_work += run.physical_work;
            temporary_erased &= run.temporary_erased;
            if anonymous_role::ready(&path.role) {
                learned_at = Some(episode_index);
                break;
            }
        }
        if let Some(episode) = learned_at {
            ready += 1;
            competence.push(episode);
        }
        single_roles += usize::from(anonymous_role::roles(&path.role) == 1);
        observations += frozen_cumulative::state(&path.credit).observations;

        let before_m7 = initiation.fingerprint();
        let before_closure = path.fingerprint();
        for held_index in 0..held_out_per_learner {
            let event_seed = seed + 250_000 + held_index as u64;
            held_out.insert(event_seed);
            let depth = [0, 1, 2, 5, 8, 16, 32][held_index % 7];
            depths.insert(depth);
            let episode = chain_episode(&mut identities, &mut rng, depth, held_index % 5);
            let initiated = initiation.activate(event_seed, held_index.is_multiple_of(2));
            m7_activations += usize::from(initiated);
            let run = execute(
                &episode,
                &mut path,
                event_seed + 10_000,
                RunOptions {
                    connected: true,
                    initiate: initiated,
                    acquire: false,
                    closure_fixture: Some(if held_index.is_multiple_of(2) {
                        ClosureFixture::Standard
                    } else {
                        ClosureFixture::Relabelled
                    }),
                },
            );
            crossings += run.crossings.len();
            correct += usize::from(run.crossings == [episode.terminal]);
            quiescent += usize::from(run.naturally_quiescent);
            if let Some(position) = run.position {
                positions.insert(position);
            }
            physical_work += run.physical_work;
            temporary_erased &= run.temporary_erased;
            control_results[1] &= run.premature == 0 && run.closures == 1;
        }
        m7_nonplastic &= before_m7 == initiation.fingerprint();
        closure_nonplastic &= before_closure == path.fingerprint();

        let control_seed = seed + 700_000;
        let finite = chain_episode(&mut identities, &mut rng, 3, 1);
        let no_initiation = execute(
            &finite,
            &mut path,
            control_seed,
            RunOptions {
                connected: true,
                initiate: false,
                acquire: false,
                closure_fixture: Some(ClosureFixture::Standard),
            },
        );
        control_results[0] &= no_initiation.crossings.is_empty()
            && no_initiation.traversals == 0
            && no_initiation.updates == 0;

        let missing = execute(
            &finite,
            &mut path,
            control_seed + 1,
            RunOptions {
                connected: true,
                initiate: true,
                acquire: false,
                closure_fixture: Some(ClosureFixture::Missing),
            },
        );
        let invalid = execute(
            &finite,
            &mut path,
            control_seed + 2,
            RunOptions {
                connected: true,
                initiate: true,
                acquire: false,
                closure_fixture: Some(ClosureFixture::Invalid),
            },
        );
        let blocked = execute(
            &finite,
            &mut path,
            control_seed + 3,
            RunOptions {
                connected: false,
                initiate: true,
                acquire: false,
                closure_fixture: Some(ClosureFixture::Standard),
            },
        );
        control_results[2] &= missing.crossings.is_empty()
            && invalid.crossings.is_empty()
            && blocked.crossings.is_empty();

        let branch = execute(
            &branch_episode(&mut identities),
            &mut path,
            control_seed + 4,
            RunOptions {
                connected: true,
                initiate: true,
                acquire: false,
                closure_fixture: Some(ClosureFixture::Standard),
            },
        );
        let cycle = execute(
            &cycle_episode(&mut identities),
            &mut path,
            control_seed + 5,
            RunOptions {
                connected: true,
                initiate: true,
                acquire: false,
                closure_fixture: Some(ClosureFixture::Standard),
            },
        );
        control_results[3] &= branch.ambiguous
            && branch.crossings.is_empty()
            && cycle.cycle_blocked
            && cycle.crossings.is_empty();
        control_results[4] &= invalid.updates == 0 && no_initiation.updates == 0;
        control_results[5] &= frozen_cumulative::controls(seed + 800_000).passed;

        let zero = chain_episode(&mut identities, &mut rng, 0, 0);
        let zero_run = execute(
            &zero,
            &mut path,
            control_seed + 6,
            RunOptions {
                connected: true,
                initiate: true,
                acquire: false,
                closure_fixture: Some(ClosureFixture::Standard),
            },
        );
        let duplicate = duplicate_episode(&mut identities);
        let duplicate_run = execute(
            &duplicate,
            &mut path,
            control_seed + 7,
            RunOptions {
                connected: true,
                initiate: true,
                acquire: false,
                closure_fixture: Some(ClosureFixture::Standard),
            },
        );
        control_results[6] &=
            zero_run.crossings == [zero.query] && duplicate_run.crossings == [duplicate.terminal];
        control_results[7] &= finite.terminal
            == execute(
                &finite,
                &mut path,
                control_seed + 8,
                RunOptions {
                    connected: true,
                    initiate: true,
                    acquire: false,
                    closure_fixture: Some(ClosureFixture::Relabelled),
                },
            )
            .crossings[0];
        control_results[8] &= anonymous_role::ready(&path.role);
        control_results[9] &= source.information_boundary;
        control_results[10] &= m7_nonplastic && closure_nonplastic;
        control_results[11] &= source.lane_isolated;
    }

    let names = [
        "learned-M7-initiation-required",
        "successor-prevents-premature-crossing",
        "missing-stale-blocked-closure-silent",
        "branch-cycle-cannot-cross",
        "invalid-nonrecurrent-no-learning",
        "M6-history-controls",
        "duplicate-zero-hop-physical",
        "finite-depth-by-structure",
        "fresh-identity-layout-position-transfer",
        "post-execution-comparison-only",
        "M7-M6-role-held-out-nonplastic",
        "lane-B-SSA-absent",
    ];
    let controls = names
        .into_iter()
        .enumerate()
        .map(|(index, name)| Control {
            number: index + 1,
            name,
            passed: control_results[index],
        })
        .collect();
    Snapshot {
        source,
        learners: seeds.len(),
        ready,
        single_roles,
        competence,
        m7_competence,
        m7_activations,
        selections,
        consequences,
        updates,
        observations,
        crossings,
        correct,
        total,
        quiescent,
        positions,
        depths,
        physical_work,
        m7_nonplastic,
        closure_nonplastic,
        temporary_erased,
        namespaces,
        held_out,
        controls,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DevelopmentReport {
    pub protocol: &'static str,
    pub mode: &'static str,
    pub claim_eligible: bool,
    pub development_ready: bool,
    pub m7_authoritative: bool,
    pub m8_exists: bool,
    pub first_collapse: &'static str,
    pub source: SourceAudit,
    pub learners: usize,
    pub ready_learners: usize,
    pub single_role_learners: usize,
    pub average_competence_millis: u64,
    pub m7_activations: usize,
    pub selections: usize,
    pub consequences: usize,
    pub updates: usize,
    pub m6_observations: u64,
    pub crossings: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub natural_quiescence: usize,
    pub positions: usize,
    pub depths: usize,
    pub physical_work: u64,
    pub m7_nonplastic: bool,
    pub closure_nonplastic: bool,
    pub temporary_erased: bool,
    pub duplicate_exact: bool,
    pub controls: Vec<Control>,
}

fn report(mode: &'static str, seeds: &[u64], held_out: usize, full: bool) -> DevelopmentReport {
    let first = snapshot(seeds, held_out);
    let second = snapshot(seeds, held_out);
    let duplicate_exact = first == second;
    let path = first.m7_activations > 0
        && first.selections > 0
        && first.consequences > 0
        && first.updates > 0
        && first.observations > 0;
    let acquisition = first.ready == first.learners
        && first.single_roles == first.learners
        && first.competence.len() == first.learners
        && first.m7_competence.len() == first.learners;
    let transfer = !full
        || (first.correct == first.total
            && first.quiescent == first.total
            && first.positions.len() == 6
            && first.depths.len() >= 6);
    let controls = !full
        || (first.controls.len() == 12 && first.controls.iter().all(|control| control.passed));
    let lifecycle = duplicate_exact
        && first.physical_work > 0
        && first.m7_nonplastic
        && first.closure_nonplastic
        && first.temporary_erased
        && first.namespaces.is_disjoint(&first.held_out);
    let stages = [
        first.source.passed(),
        path,
        acquisition,
        transfer,
        controls,
        lifecycle,
    ];
    let first_collapse = stages
        .iter()
        .position(|stage| !stage)
        .map_or("NONE", |index| {
            [
                "P0 exact M7/source/lane authority",
                "P1-P5 initiation/closure/M6 active-trace path",
                "P6 one anonymous closure-boundary role",
                "P7 held-out transfer and natural quiescence",
                "P8 controls",
                "P8 determinism/lifecycle/nonplasticity",
            ][index]
        });
    let competence_sum = first.competence.iter().sum::<usize>() as u64;
    let competence_count = first.competence.len() as u64;
    DevelopmentReport {
        protocol: PROTOCOL,
        mode,
        claim_eligible: false,
        development_ready: stages.iter().all(|stage| *stage),
        m7_authoritative: true,
        m8_exists: false,
        first_collapse,
        source: first.source,
        learners: first.learners,
        ready_learners: first.ready,
        single_role_learners: first.single_roles,
        average_competence_millis: (competence_sum * 1_000)
            .checked_div(competence_count)
            .unwrap_or(0),
        m7_activations: first.m7_activations,
        selections: first.selections,
        consequences: first.consequences,
        updates: first.updates,
        m6_observations: first.observations,
        crossings: first.crossings,
        held_out_correct: first.correct,
        held_out_total: first.total,
        natural_quiescence: first.quiescent,
        positions: first.positions.len(),
        depths: first.depths.len(),
        physical_work: first.physical_work,
        m7_nonplastic: first.m7_nonplastic,
        closure_nonplastic: first.closure_nonplastic,
        temporary_erased: first.temporary_erased,
        duplicate_exact,
        controls: first.controls,
    }
}

pub fn run_probe_retry() -> DevelopmentReport {
    report("PROBE-RETRY", &[PROBE_RETRY_SEED], 0, false)
}

pub fn run_development(mode: HarnessMode) -> DevelopmentReport {
    match mode {
        HarnessMode::Micro => report("MICRO", &MICRO_SEEDS, 8, true),
        HarnessMode::Gate => report("GATE", &GATE_SEEDS, 32, true),
        HarnessMode::Definitive => DevelopmentReport {
            protocol: PROTOCOL,
            mode: "DEFINITIVE-FORBIDDEN",
            claim_eligible: false,
            development_ready: false,
            m7_authoritative: true,
            m8_exists: false,
            first_collapse: "definitive rejected before learner or seed construction",
            source: source_audit(),
            learners: 0,
            ready_learners: 0,
            single_role_learners: 0,
            average_competence_millis: 0,
            m7_activations: 0,
            selections: 0,
            consequences: 0,
            updates: 0,
            m6_observations: 0,
            crossings: 0,
            held_out_correct: 0,
            held_out_total: 0,
            natural_quiescence: 0,
            positions: 0,
            depths: 0,
            physical_work: 0,
            m7_nonplastic: false,
            closure_nonplastic: false,
            temporary_erased: false,
            duplicate_exact: false,
            controls: Vec::new(),
        },
    }
}

pub fn definitive_rejected() -> bool {
    let report = run_development(HarnessMode::Definitive);
    report.mode == "DEFINITIVE-FORBIDDEN"
        && report.learners == 0
        && !report.claim_eligible
        && !report.development_ready
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_v1_freezes_the_expected_first_collapse() {
        let report = run_probe_v1();
        assert!(report.expected_negative, "{report:#?}");
        assert!(report.physical_closure_path);
        assert_eq!(report.lawful_m6_links, 0);
        assert_eq!(report.lawful_updates, 0);
    }

    #[test]
    fn probe_retry_closes_only_the_preregistered_edge() {
        let report = run_probe_retry();
        assert!(report.development_ready, "{report:#?}");
        assert!(report.updates > 0 && report.m6_observations > 0);
        assert!(!report.claim_eligible && report.m7_authoritative && !report.m8_exists);
    }

    #[test]
    fn micro_is_conjunctive_and_development_only() {
        let report = run_development(HarnessMode::Micro);
        assert!(report.development_ready, "{report:#?}");
        assert_eq!(report.held_out_correct, report.held_out_total);
        assert!(report.controls.iter().all(|control| control.passed));
    }

    #[test]
    fn gate_reaches_development_readiness_without_m8() {
        let report = run_development(HarnessMode::Gate);
        assert!(report.development_ready, "{report:#?}");
        assert_eq!(report.ready_learners, 6);
        assert_eq!(report.held_out_correct, 192);
        assert_eq!(report.natural_quiescence, 192);
        assert_eq!(report.positions, 6);
        assert!(report.duplicate_exact && !report.m8_exists);
    }

    #[test]
    fn definitive_is_inert() {
        assert!(definitive_rejected());
    }
}
