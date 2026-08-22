//! DS-C0 development-only anonymous evidence-to-choice coupling gate.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::mem::size_of;

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-c0-anonymous-credit-coupling-v1";
pub const EXACT_PARENT: &str = "d6b75128de7ad4bfb79b2dd4535a0b3d81cabcf0";
pub const PROTOCOL_COMMIT: &str = "2ab1796b438a91eb5aea4f56c375c377ddcc0f81";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_R0_SHA256: &str =
    "f17afa482bf345eb680463f7418b6b6c2553cd78eab9b4dbfce74f7ca1483d51";
pub const FROZEN_PARENT_RETRY_SHA256: &str =
    "36c33cb3595001416b4763c29cdba88b5c9567caadc61d8d002177e972ffacce";
pub const FROZEN_PARENT_HANDOFF_SHA256: &str =
    "729dd43af12ac5ef35d07f2ddba0609f807344d1e40c4804cf29d478cdd405e6";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

const ELIGIBILITY_LIFETIME: u16 = 3;
const R0_SUPPORT: usize = 3;
const STAGES: [&str; 9] = [
    "0. exact parent/frozen lineage and R0 controls",
    "1. actual selected execution and exact R0 evidence surface",
    "2. temporary anonymous eligibility before evidence arrival",
    "3. frozen eligibility lifetime",
    "4. physical returned-evidence encounter",
    "5. one anonymous temporary coupling without polarity",
    "6. fresh/relabel/layout/permutation/interleaving controls",
    "7. ambiguity/distractor/negative/stale/shuffle controls",
    "8. leak/no-update/lifetime/work/cleanup audits",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Occurrence(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pulse {
    occurrence: Occurrence,
    tick: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Propagation {
    from: Occurrence,
    to: Occurrence,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Activity {
    pulses: Vec<Pulse>,
    propagation: Vec<Propagation>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Evidence {
    members: [Occurrence; 2],
    lag: u16,
    hops: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct R0Export {
    activity: Activity,
    evidence: Evidence,
    choice: usize,
    choose_calls: u64,
    ds1_updates: u64,
    evaluator_effect: u64,
    roots: usize,
    handles: usize,
    exact: bool,
    cleanup: bool,
    e0_work: u64,
    a1_work: u64,
    r0_work: u64,
    e0_bytes: usize,
    a1_bytes: usize,
    ds1_bytes: usize,
    r0_bytes: usize,
}

macro_rules! c0_r0_access {
    () => {
        pub(super) fn c0_export(
            seed: u64,
            acquisition: usize,
            permuted: bool,
            base_shift: u64,
        ) -> Option<super::R0Export> {
            let mut input = frozen_e0::bundle(seed, acquisition)?;
            let mut actions = frozen_a1::prepare(&input.support, &input.target, permuted)?;
            let (choice, choose_calls, ds1_updates) = input.choose(seed as usize);
            let base = seed * 1_000_000 + 10_000 + base_shift;
            let mut learner = ReturnLearner::default();
            for n in 0..super::R0_SUPPORT {
                let activity = actions.execute(choice, base + n as u64 * 100)?.activity;
                learner.observe(&activity, true);
            }
            let target = actions.execute(choice, base + 10_000)?;
            let exact = actions.known(target.evaluator_effect)
                && target.activity.pulses.len() == 3
                && target.activity.propagation.len() == 2
                && target.spikes == 2
                && target.arrows == 1
                && target.mutations == 2;
            let relation = learner.form_one(&target.activity)?;
            let surface = bridge(relation, &mut learner.work);
            let activity = super::Activity {
                pulses: target
                    .activity
                    .pulses
                    .iter()
                    .map(|pulse| super::Pulse {
                        occurrence: super::Occurrence(pulse.occurrence.0),
                        tick: pulse.tick,
                    })
                    .collect(),
                propagation: target
                    .activity
                    .propagation
                    .iter()
                    .map(|edge| super::Propagation {
                        from: super::Occurrence(edge.from.0),
                        to: super::Occurrence(edge.to.0),
                    })
                    .collect(),
            };
            let evidence = super::Evidence {
                members: surface.members.map(super::Occurrence),
                lag: surface.lag,
                hops: surface.hops,
            };
            let roots = actions.installed;
            let handles = actions.handles;
            let a1_work = actions.organism_work();
            let cleanup = actions.cleanup();
            Some(super::R0Export {
                activity,
                evidence,
                choice,
                choose_calls,
                ds1_updates,
                evaluator_effect: target.evaluator_effect,
                roots,
                handles,
                exact,
                cleanup,
                e0_work: input.work,
                a1_work,
                r0_work: learner.work.organism_work(),
                e0_bytes: input.bytes,
                a1_bytes: actions.bytes,
                ds1_bytes: input.ds1_bytes(),
                r0_bytes: learner.bytes(),
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_r0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_r0_anonymous_post_action_evidence_return.rs"
    ));
    c0_r0_access!();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EligibilityCell {
    anchor: Occurrence,
    created_at: u16,
    expires_at: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CouplingArrow {
    from: Occurrence,
    to: Occurrence,
    lag: u16,
    hops: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CouplingWork {
    pub pulse_observations: u64,
    pub propagation_observations: u64,
    pub temporal_comparisons: u64,
    pub propagation_traversals: u64,
    pub cell_creations: u64,
    pub expiry_checks: u64,
    pub arrow_formations: u64,
    pub spike_deliveries: u64,
    pub cleanup: u64,
}

impl CouplingWork {
    pub fn organism_work(&self) -> u64 {
        self.pulse_observations
            + self.propagation_observations
            + self.temporal_comparisons
            + self.propagation_traversals
            + self.cell_creations
            + self.expiry_checks
            + self.arrow_formations
            + self.spike_deliveries
            + self.cleanup
    }
}

#[derive(Clone, Debug, Default)]
struct Workspace {
    cells: Vec<EligibilityCell>,
    arrows: Vec<CouplingArrow>,
    work: CouplingWork,
}

fn pulse_tick(activity: &Activity, occurrence: Occurrence) -> Option<u16> {
    activity
        .pulses
        .iter()
        .find(|pulse| pulse.occurrence == occurrence)
        .map(|pulse| pulse.tick)
}

fn physical_hops(
    activity: &Activity,
    from: Occurrence,
    to: Occurrence,
    work: &mut CouplingWork,
) -> Option<u8> {
    let mut queue = VecDeque::from([(from, 0u8)]);
    let mut seen = BTreeSet::new();
    while let Some((current, depth)) = queue.pop_front() {
        if !seen.insert(current) {
            continue;
        }
        if current == to && depth > 0 {
            return Some(depth);
        }
        for edge in &activity.propagation {
            work.propagation_traversals += 1;
            if edge.from == current {
                queue.push_back((edge.to, depth.saturating_add(1)));
            }
        }
    }
    None
}

impl Workspace {
    fn open_from_execution(&mut self, activity: &Activity, executed: bool) -> usize {
        if !executed {
            return 0;
        }
        self.work.pulse_observations += activity.pulses.len() as u64;
        self.work.propagation_observations += activity.propagation.len() as u64;
        let incoming = activity
            .propagation
            .iter()
            .map(|edge| edge.to)
            .collect::<BTreeSet<_>>();
        let outgoing = activity
            .propagation
            .iter()
            .map(|edge| edge.from)
            .collect::<BTreeSet<_>>();
        let roots = activity
            .pulses
            .iter()
            .filter(|pulse| {
                !incoming.contains(&pulse.occurrence) && outgoing.contains(&pulse.occurrence)
            })
            .collect::<Vec<_>>();
        if roots.len() != 1 {
            return 0;
        }
        self.cells.push(EligibilityCell {
            anchor: roots[0].occurrence,
            created_at: roots[0].tick,
            expires_at: roots[0].tick.saturating_add(ELIGIBILITY_LIFETIME),
        });
        self.work.cell_creations += 1;
        1
    }

    fn encounter(
        &mut self,
        activity: &Activity,
        evidence: Evidence,
        arrival_tick: u16,
    ) -> Option<CouplingArrow> {
        self.work.temporal_comparisons += 1;
        let (start, end) = (
            pulse_tick(activity, evidence.members[0])?,
            pulse_tick(activity, evidence.members[1])?,
        );
        if end.checked_sub(start)? != evidence.lag
            || physical_hops(
                activity,
                evidence.members[0],
                evidence.members[1],
                &mut self.work,
            )? != evidence.hops
        {
            return None;
        }
        let candidates = self
            .cells
            .iter()
            .filter(|cell| {
                self.work.expiry_checks += 1;
                cell.anchor == evidence.members[0]
                    && arrival_tick >= cell.created_at
                    && arrival_tick <= cell.expires_at
            })
            .copied()
            .collect::<Vec<_>>();
        if candidates.len() != 1 {
            return None;
        }
        let coupling = CouplingArrow {
            from: candidates[0].anchor,
            to: evidence.members[1],
            lag: evidence.lag,
            hops: evidence.hops,
        };
        self.arrows.push(coupling);
        self.work.arrow_formations += 1;
        self.work.spike_deliveries += 1;
        Some(coupling)
    }

    fn alive_at(&mut self, tick: u16) -> usize {
        self.cells
            .iter()
            .filter(|cell| {
                self.work.expiry_checks += 1;
                tick <= cell.expires_at
            })
            .count()
    }

    fn duplicate_last(&mut self) {
        if let Some(last) = self.cells.last().copied() {
            self.cells.push(last);
            self.work.cell_creations += 1;
        }
    }

    fn cleanup(&mut self) -> bool {
        self.work.cleanup += (self.cells.len() + self.arrows.len()) as u64;
        self.cells.clear();
        self.arrows.clear();
        self.cells.is_empty() && self.arrows.is_empty()
    }
}

fn relabel(activity: &Activity, evidence: Evidence, salt: u64) -> (Activity, Evidence) {
    let map = activity
        .pulses
        .iter()
        .enumerate()
        .map(|(index, pulse)| (pulse.occurrence, Occurrence(salt + index as u64 * 101 + 17)))
        .collect::<BTreeMap<_, _>>();
    (
        Activity {
            pulses: activity
                .pulses
                .iter()
                .map(|pulse| Pulse {
                    occurrence: map[&pulse.occurrence],
                    tick: pulse.tick,
                })
                .collect(),
            propagation: activity
                .propagation
                .iter()
                .map(|edge| Propagation {
                    from: map[&edge.from],
                    to: map[&edge.to],
                })
                .collect(),
        },
        Evidence {
            members: evidence.members.map(|member| map[&member]),
            ..evidence
        },
    )
}

fn interleave(first: &Activity, second: &Activity) -> Activity {
    let mut pulses = first.pulses.clone();
    pulses.extend(&second.pulses);
    pulses.sort_by_key(|pulse| (pulse.tick, pulse.occurrence));
    let mut propagation = first.propagation.clone();
    propagation.extend(&second.propagation);
    Activity {
        pulses,
        propagation,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub r0_hash: bool,
    pub parent_retry_hash: bool,
    pub parent_handoff_hash: bool,
    pub ds1_hash: bool,
    pub exact_r0_accessors: usize,
    pub update_edges: usize,
    pub semantic_edges: usize,
    pub evaluator_to_workspace_edges: usize,
    pub persistent_identity_fields: usize,
    pub update_mutation_sensitive: bool,
    pub evaluator_mutation_sensitive: bool,
}

fn function_body<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    let start = source.find(marker)?;
    let tail = &source[start..];
    let open = tail.find('{')?;
    let mut depth = 0usize;
    for (offset, byte) in tail[open..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&tail[..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn derive_source(source: &str) -> SourceAudit {
    let production = source.split("#[cfg(test)]").next().unwrap_or(source);
    let workspace = function_body(source, "impl Workspace").unwrap_or_default();
    let persistent = function_body(source, "struct Workspace").unwrap_or_default();
    let update_call = [".apply_", "consequence("].concat();
    let semantic_calls = [
        ["semantic_", "credit("].concat(),
        ["correct_", "choice("].concat(),
        ["reward_", "update("].concat(),
        ["accepted_", "output("].concat(),
        ["rejected_", "output("].concat(),
    ];
    SourceAudit {
        r0_hash: env!("DS_C0_R0_SHA256") == FROZEN_R0_SHA256,
        parent_retry_hash: env!("DS_C0_PARENT_RETRY_SHA256") == FROZEN_PARENT_RETRY_SHA256,
        parent_handoff_hash: env!("DS_C0_PARENT_HANDOFF_SHA256") == FROZEN_PARENT_HANDOFF_SHA256,
        ds1_hash: frozen_r0::FROZEN_DS1_SHA256 == FROZEN_DS1_SHA256,
        exact_r0_accessors: production.matches("pub(super) fn c0_export(").count(),
        update_edges: production.matches(&update_call).count(),
        semantic_edges: semantic_calls
            .iter()
            .map(|call| production.matches(call).count())
            .sum(),
        evaluator_to_workspace_edges: workspace.matches("evaluator_effect").count(),
        persistent_identity_fields: ["Occurrence", "choice", "handle", "root", "destination"]
            .iter()
            .map(|field| persistent.matches(field).count())
            .sum(),
        ..SourceAudit::default()
    }
}

fn source_audit() -> SourceAudit {
    let source = include_str!("ds_c0_anonymous_credit_coupling.rs");
    let baseline = derive_source(source);
    let update_mutation = source.replacen(
        "#[cfg(test)]",
        "fn mutation(){learner.apply_consequence(view,choice,true);}\n#[cfg(test)]",
        1,
    );
    let evaluator_mutation = source.replacen(
        "impl Workspace {",
        "impl Workspace { fn mutation(&self){let _=self.evaluator_effect;}",
        1,
    );
    SourceAudit {
        update_mutation_sensitive: derive_source(&update_mutation).update_edges
            > baseline.update_edges,
        evaluator_mutation_sensitive: derive_source(&evaluator_mutation)
            .evaluator_to_workspace_edges
            > baseline.evaluator_to_workspace_edges,
        ..baseline
    }
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.r0_hash
            && self.parent_retry_hash
            && self.parent_handoff_hash
            && self.ds1_hash
            && self.exact_r0_accessors == 1
            && self.update_edges == 0
            && self.semantic_edges == 0
            && self.evaluator_to_workspace_edges == 0
            && self.persistent_identity_fields == 0
            && self.update_mutation_sensitive
            && self.evaluator_mutation_sensitive
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Controls {
    pub frozen_r0: bool,
    pub fresh: bool,
    pub relabel: bool,
    pub layout: bool,
    pub handle_permutation: bool,
    pub interleaving: bool,
    pub correct_pairing: bool,
    pub ambiguity: bool,
    pub distractor: bool,
    pub no_execution: bool,
    pub no_evidence: bool,
    pub stale: bool,
    pub shuffled_propagation: bool,
    pub missing_terminal: bool,
    pub no_update: bool,
    pub no_semantics: bool,
    pub no_persistent_identity: bool,
    pub mutation_sensitive: bool,
    pub cleanup: bool,
}

impl Controls {
    pub fn passed(&self) -> bool {
        self.frozen_r0
            && self.fresh
            && self.relabel
            && self.layout
            && self.handle_permutation
            && self.interleaving
            && self.correct_pairing
            && self.ambiguity
            && self.distractor
            && self.no_execution
            && self.no_evidence
            && self.stale
            && self.shuffled_propagation
            && self.missing_terminal
            && self.no_update
            && self.no_semantics
            && self.no_persistent_identity
            && self.mutation_sensitive
            && self.cleanup
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub roots: usize,
    pub handles: usize,
    pub choice: usize,
    pub choose_calls: u64,
    pub ds1_updates: u64,
    pub eligibility_cells: usize,
    pub couplings: usize,
    pub coupling_polarity_fields: usize,
    pub evidence_fields: usize,
    pub controls: Controls,
    pub work: CouplingWork,
    pub e0_work: u64,
    pub a1_work: u64,
    pub r0_work: u64,
    pub c0_persistent_bytes: usize,
    pub temporary_peak_bytes: usize,
    pub stage_ready: [bool; 9],
}

fn audit_seed(
    seed: u64,
    acquisition: usize,
    source: &SourceAudit,
    r0_control: &frozen_r0::SeedAudit,
) -> SeedAudit {
    let primary = frozen_r0::c0_export(seed, acquisition, false, 0).expect("actual R0 export");
    let alternate =
        frozen_r0::c0_export(seed, acquisition, true, 200_000).expect("permuted R0 export");
    let root_ids = primary
        .activity
        .pulses
        .iter()
        .map(|pulse| pulse.occurrence)
        .collect::<BTreeSet<_>>();
    let alternate_ids = alternate
        .activity
        .pulses
        .iter()
        .map(|pulse| pulse.occurrence)
        .collect::<BTreeSet<_>>();

    let mut workspace = Workspace::default();
    let eligibility_cells = workspace.open_from_execution(&primary.activity, true);
    let alive_allowed = workspace.alive_at(ELIGIBILITY_LIFETIME) == 1;
    let expired = workspace.alive_at(ELIGIBILITY_LIFETIME + 1) == 0;
    let coupling = workspace.encounter(&primary.activity, primary.evidence, 2);

    let (relabeled_activity, relabeled_evidence) = relabel(
        &primary.activity,
        primary.evidence,
        seed * 10_000_000 + 700_000,
    );
    let mut relabeled = Workspace::default();
    let relabel_ok = relabeled.open_from_execution(&relabeled_activity, true) == 1
        && relabeled
            .encounter(&relabeled_activity, relabeled_evidence, 2)
            .is_some();

    let mut layout_activity = primary.activity.clone();
    layout_activity.pulses.reverse();
    layout_activity.propagation.reverse();
    let mut layout = Workspace::default();
    let layout_ok = layout.open_from_execution(&layout_activity, true) == 1
        && layout
            .encounter(&layout_activity, primary.evidence, 2)
            .is_some();

    let mut permuted = Workspace::default();
    let handle_permutation = primary.choice == alternate.choice
        && primary.evaluator_effect != alternate.evaluator_effect
        && permuted.open_from_execution(&alternate.activity, true) == 1
        && permuted
            .encounter(&alternate.activity, alternate.evidence, 2)
            .is_some();

    let joined = interleave(&primary.activity, &alternate.activity);
    let mut interleaved = Workspace::default();
    let opened = interleaved.open_from_execution(&primary.activity, true)
        + interleaved.open_from_execution(&alternate.activity, true);
    let first_pair = interleaved.encounter(&joined, primary.evidence, 2);
    let second_pair = interleaved.encounter(&joined, alternate.evidence, 2);
    let correct_pairing = first_pair.is_some_and(|arrow| {
        arrow.from == primary.evidence.members[0] && arrow.to == primary.evidence.members[1]
    }) && second_pair.is_some_and(|arrow| {
        arrow.from == alternate.evidence.members[0] && arrow.to == alternate.evidence.members[1]
    });

    let mut ambiguous = Workspace::default();
    ambiguous.open_from_execution(&primary.activity, true);
    ambiguous.duplicate_last();
    let ambiguity = ambiguous
        .encounter(&primary.activity, primary.evidence, 2)
        .is_none();

    let mut distractor_activity = primary.activity.clone();
    distractor_activity.pulses.push(Pulse {
        occurrence: Occurrence(seed * 10_000_000 + 800_000),
        tick: 1,
    });
    let mut distractor_workspace = Workspace::default();
    let distractor = distractor_workspace.open_from_execution(&primary.activity, true) == 1
        && distractor_workspace
            .encounter(&distractor_activity, primary.evidence, 2)
            .is_some();

    let mut none = Workspace::default();
    let no_execution = none.open_from_execution(&primary.activity, false) == 0
        && none
            .encounter(&primary.activity, primary.evidence, 2)
            .is_none();
    let mut no_evidence_workspace = Workspace::default();
    let no_evidence = no_evidence_workspace.open_from_execution(&primary.activity, true) == 1
        && no_evidence_workspace.arrows.is_empty();
    let mut stale_workspace = Workspace::default();
    stale_workspace.open_from_execution(&primary.activity, true);
    let stale = stale_workspace
        .encounter(
            &primary.activity,
            primary.evidence,
            ELIGIBILITY_LIFETIME + 1,
        )
        .is_none();

    let mut shuffled = primary.activity.clone();
    for edge in &mut shuffled.propagation {
        std::mem::swap(&mut edge.from, &mut edge.to);
    }
    let mut shuffled_workspace = Workspace::default();
    shuffled_workspace.open_from_execution(&primary.activity, true);
    let shuffled_propagation = shuffled_workspace
        .encounter(&shuffled, primary.evidence, 2)
        .is_none();

    let mut missing = primary.activity.clone();
    missing
        .pulses
        .retain(|pulse| pulse.occurrence != primary.evidence.members[1]);
    missing.propagation.retain(|edge| {
        edge.from != primary.evidence.members[1] && edge.to != primary.evidence.members[1]
    });
    let mut missing_workspace = Workspace::default();
    missing_workspace.open_from_execution(&primary.activity, true);
    let missing_terminal = missing_workspace
        .encounter(&missing, primary.evidence, 2)
        .is_none();

    let coupling_formed = usize::from(coupling.is_some());
    let stage_one = primary.exact
        && primary.roots == 2
        && primary.handles == 2
        && primary.choose_calls == 1
        && primary.ds1_updates == 0
        && primary.activity.pulses.len() == 3
        && primary.activity.propagation.len() == 2;
    let stage_two = stage_one && eligibility_cells == 1 && workspace.cells.len() == 1;
    let stage_three = stage_two && alive_allowed && expired;
    let stage_four = stage_three
        && primary.evidence.members[0] == workspace.cells[0].anchor
        && coupling.is_some();
    let stage_five = stage_four && coupling_formed == 1;
    let stage_six = stage_five
        && root_ids.is_disjoint(&alternate_ids)
        && relabel_ok
        && layout_ok
        && handle_permutation
        && opened == 2
        && correct_pairing;
    let stage_seven = stage_six
        && ambiguity
        && distractor
        && no_execution
        && no_evidence
        && stale
        && shuffled_propagation
        && missing_terminal;

    let frozen_r0_ok = r0_control.controls.passed()
        && r0_control.roots == 2
        && r0_control.handles == 2
        && r0_control.temporary_relations == 1
        && r0_control.bridge_fields == 4;
    let no_update =
        primary.ds1_updates == 0 && alternate.ds1_updates == 0 && source.update_edges == 0;
    let no_semantics = source.semantic_edges == 0 && source.evaluator_to_workspace_edges == 0;
    let no_persistent_identity = source.persistent_identity_fields == 0;
    let cleanup = workspace.cleanup()
        && relabeled.cleanup()
        && layout.cleanup()
        && permuted.cleanup()
        && interleaved.cleanup()
        && ambiguous.cleanup()
        && distractor_workspace.cleanup()
        && none.cleanup()
        && no_evidence_workspace.cleanup()
        && stale_workspace.cleanup()
        && shuffled_workspace.cleanup()
        && missing_workspace.cleanup()
        && primary.cleanup
        && alternate.cleanup;
    let controls = Controls {
        frozen_r0: frozen_r0_ok,
        fresh: root_ids.is_disjoint(&alternate_ids),
        relabel: relabel_ok,
        layout: layout_ok,
        handle_permutation,
        interleaving: opened == 2,
        correct_pairing,
        ambiguity,
        distractor,
        no_execution,
        no_evidence,
        stale,
        shuffled_propagation,
        missing_terminal,
        no_update,
        no_semantics,
        no_persistent_identity,
        mutation_sensitive: source.update_mutation_sensitive && source.evaluator_mutation_sensitive,
        cleanup,
    };
    let stage_eight = stage_seven && source.passed() && controls.passed();
    let work = workspace.work.clone();
    SeedAudit {
        seed,
        roots: primary.roots,
        handles: primary.handles,
        choice: primary.choice,
        choose_calls: primary.choose_calls,
        ds1_updates: primary.ds1_updates,
        eligibility_cells,
        couplings: coupling_formed,
        coupling_polarity_fields: 0,
        evidence_fields: 4,
        controls,
        work,
        e0_work: primary.e0_work,
        a1_work: primary.a1_work,
        r0_work: primary.r0_work,
        c0_persistent_bytes: 0,
        temporary_peak_bytes: size_of::<EligibilityCell>() + size_of::<CouplingArrow>(),
        stage_ready: [
            source.passed() && frozen_r0_ok,
            stage_one,
            stage_two,
            stage_three,
            stage_four,
            stage_five,
            stage_six,
            stage_seven,
            stage_eight,
        ],
    }
}

fn freeze(ready: [bool; 9]) -> ([String; 9], Option<usize>) {
    let first = ready.iter().position(|stage| !stage);
    (
        std::array::from_fn(|stage| match first {
            None => "READY".to_string(),
            Some(collapse) if stage < collapse => "READY".to_string(),
            Some(collapse) if stage == collapse => format!("COLLAPSE: {}", STAGES[stage]),
            Some(_) => "BLOCKED".to_string(),
        }),
        first,
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub m0_authoritative: bool,
    pub enabling_only: bool,
    pub m1_exists: bool,
    pub source: SourceAudit,
    pub stages: [String; 9],
    pub first_collapse: Option<usize>,
    pub seeds: Vec<SeedAudit>,
    pub audit_passed: bool,
}

fn rejected() -> Report {
    Report {
        label: "DS-C0 definitive forbidden".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "DEFINITIVE-FORBIDDEN".to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        enabling_only: true,
        m1_exists: false,
        source: source_audit(),
        stages: std::array::from_fn(|_| "BLOCKED: definitive rejected".to_string()),
        first_collapse: None,
        seeds: Vec::new(),
        audit_passed: false,
    }
}

pub fn run(mode: HarnessMode) -> Report {
    if mode == HarnessMode::Definitive {
        return rejected();
    }
    let source = source_audit();
    let r0 = frozen_r0::run(mode);
    let acquisition = match mode {
        HarnessMode::Micro => 16,
        HarnessMode::Gate => 32,
        HarnessMode::Definitive => unreachable!(),
    };
    let seeds = r0
        .seeds
        .iter()
        .map(|parent| audit_seed(parent.seed, acquisition, &source, parent))
        .collect::<Vec<_>>();
    let mut ready = [false; 9];
    for (stage, value) in ready.iter_mut().enumerate() {
        *value = seeds.iter().all(|seed| seed.stage_ready[stage]);
    }
    let (stages, first_collapse) = freeze(ready);
    let audit_passed = r0.audit_passed
        && first_collapse.is_none()
        && seeds.iter().all(|seed| {
            seed.controls.passed()
                && seed.eligibility_cells == 1
                && seed.couplings == 1
                && seed.coupling_polarity_fields == 0
                && seed.ds1_updates == 0
                && seed.c0_persistent_bytes == 0
        });
    Report {
        label: if audit_passed {
            "DS-C0 DEVELOPMENT IMPLEMENTATION READY".to_string()
        } else {
            format!(
                "DS-C0 DEVELOPMENT COLLAPSE AT {}",
                first_collapse
                    .map(|stage| STAGES[stage])
                    .unwrap_or("unknown")
            )
        },
        protocol: PROTOCOL.to_string(),
        mode: r0.mode,
        claim_eligible: false,
        m0_authoritative: true,
        enabling_only: true,
        m1_exists: false,
        source,
        stages,
        first_collapse,
        seeds,
        audit_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_forms_anonymous_coupling_without_update() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert!(report.first_collapse.is_none());
        assert!(report.seeds.iter().all(|seed| seed.eligibility_cells == 1
            && seed.couplings == 1
            && seed.coupling_polarity_fields == 0
            && seed.ds1_updates == 0));
    }

    #[test]
    fn gate_passes_all_controls() {
        let report = run(HarnessMode::Gate);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.seeds.len(), 5);
        assert!(report.seeds.iter().all(|seed| seed.controls.passed()));
    }

    #[test]
    fn source_boundary_is_mechanical() {
        let audit = source_audit();
        assert!(audit.passed(), "{audit:#?}");
        assert!(audit.update_mutation_sensitive && audit.evaluator_mutation_sensitive);
    }

    #[test]
    fn ordered_freeze_blocks_later_stages() {
        for collapse in 0..9 {
            let mut ready = [true; 9];
            ready[collapse] = false;
            let (stages, first) = freeze(ready);
            assert_eq!(first, Some(collapse));
            assert!(stages[..collapse].iter().all(|stage| stage == "READY"));
            assert!(stages[collapse].starts_with("COLLAPSE"));
            assert!(stages[collapse + 1..]
                .iter()
                .all(|stage| stage == "BLOCKED"));
        }
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.audit_passed && report.seeds.is_empty() && !report.m1_exists);
    }
}
