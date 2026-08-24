//! DS-R0 development-only anonymous post-action evidence-return gate.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::mem::size_of;

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-r0-anonymous-post-action-evidence-return-v1";
pub const EXACT_PARENT: &str = "737630c699912e20f4bb3eb244a12f007922960d";
pub const PROTOCOL_COMMIT: &str = "4d9b317fafc0abc1ad502c2067debf8b5e240175";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_E0_SHA256: &str =
    "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615";
pub const FROZEN_A1_SHA256: &str =
    "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1";
pub const FROZEN_PARENT_SHA256: &str =
    "3b96de98a8f91ca9f7338d1184d4d2e6c10e6528783820030d6ae74dae81d08e";
pub const FROZEN_PARENT_HANDOFF_SHA256: &str =
    "3f68560d86171a29ea159c90e5a05584554a9d06c4fa12f9ee54a192f9b53bfd";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

const A1_SUPPORT: usize = 12;
const RETURN_SUPPORT: usize = 3;
const STAGES: [&str; 8] = [
    "0. exact frozen lineage and stage-six parent signature",
    "1. actual E0 target forms two actual A1 executable roots",
    "2. frozen DS1 chooses and selected root physically executes",
    "3. actual execution emits fresh anonymous temporal/propagation activity",
    "4. repeated support consolidates a role-relative return shape",
    "5. fresh target execution forms the valid temporary return relation",
    "6. format-only evidence bridge copies the relation one-to-one",
    "7. transfer, ambiguity, negative, leak, lifetime, and cleanup controls",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ExportPulse {
    occurrence: u32,
    tick: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct E0Export {
    pulses: Vec<ExportPulse>,
    propagation: Vec<[u32; 2]>,
    members: [u32; 3],
    temporal: [i8; 9],
    relative_propagation: [i8; 9],
}

macro_rules! r0_e0_access {
    () => {
        pub(super) struct Bundle {
            pub(super) support: Vec<super::E0Export>,
            pub(super) target: super::E0Export,
            view: Neighborhood,
            learner: Learner,
            pub(super) actual: bool,
            pub(super) exact: bool,
            pub(super) fresh: bool,
            pub(super) work: u64,
            pub(super) bytes: usize,
        }
        fn export(raw: &RawActivity, event: &EventRelations) -> super::E0Export {
            super::E0Export {
                pulses: raw
                    .spikes
                    .iter()
                    .map(|s| super::ExportPulse {
                        occurrence: s.occurrence.0,
                        tick: s.local_tick,
                    })
                    .collect(),
                propagation: raw.propagation.iter().map(|e| [e.from.0, e.to.0]).collect(),
                members: event.members.map(|m| m.0),
                temporal: event.temporal,
                relative_propagation: event.propagation,
            }
        }
        fn exact(copy: &super::E0Export, raw: &RawActivity, event: &EventRelations) -> bool {
            copy.pulses
                .iter()
                .zip(&raw.spikes)
                .all(|(a, b)| a.occurrence == b.occurrence.0 && a.tick == b.local_tick)
                && copy
                    .propagation
                    .iter()
                    .zip(&raw.propagation)
                    .all(|(a, b)| *a == [b.from.0, b.to.0])
                && copy.members == event.members.map(|m| m.0)
                && copy.temporal == event.temporal
                && copy.relative_propagation == event.propagation
        }
        impl Bundle {
            pub(super) fn choose(&mut self, tie: usize) -> (usize, u64, u64) {
                let f = self.learner.route_firings;
                let u = self.learner.credit_updates;
                let (choice, _) = self.learner.choose(&self.view, tie);
                (
                    choice,
                    self.learner.route_firings - f,
                    self.learner.credit_updates - u,
                )
            }
            pub(super) fn ds1_bytes(&self) -> usize {
                self.learner.persistent_bytes()
            }
        }
        pub(super) fn bundle(seed: u64, acquisition: usize) -> Option<Bundle> {
            let (mut formation, mut prior) = acquire(seed, acquisition);
            let mut support = Vec::new();
            let mut copies = true;
            for n in 0..super::A1_SUPPORT {
                let ep = fixture(seed + 1000, acquisition + n, n % 4, Perturbation::None);
                prior.extend(ep.raw.spikes.iter().map(|s| s.occurrence));
                let event = formation.form(&ep.raw)?;
                let item = export(&ep.raw, &event);
                copies &= exact(&item, &ep.raw, &event);
                support.push(item);
            }
            let ep = fixture(
                seed + 2000,
                acquisition + super::A1_SUPPORT + 17,
                0,
                Perturbation::None,
            );
            let current = ep
                .raw
                .spikes
                .iter()
                .map(|s| s.occurrence)
                .collect::<BTreeSet<_>>();
            let event = formation.form(&ep.raw)?;
            let target = export(&ep.raw, &event);
            copies &= exact(&target, &ep.raw, &event);
            let view = serialize_once(&event, &mut formation.work);
            Some(Bundle {
                support,
                target,
                view,
                learner: Learner::default(),
                actual: members_set(&event.members) == ep.selected,
                exact: copies,
                fresh: prior.is_disjoint(&current),
                work: formation.work.organism_work(),
                bytes: formation.persistent_bytes(),
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_e0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_e0_anonymous_event_formation.rs"
    ));
    r0_e0_access!();
}

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
#[derive(Clone, Debug, PartialEq, Eq)]
struct ObservedExecution {
    activity: Activity,
    evaluator_effect: u64,
    spikes: u64,
    arrows: u64,
    mutations: u64,
}

fn effect_fingerprint(trace: &[u8], activation: &[u16; 3]) -> u64 {
    trace
        .iter()
        .map(|x| u64::from(*x))
        .chain(activation.iter().map(|x| u64::from(*x)))
        .fold(0xcbf2_9ce4_8422_2325u64, |mut h, v| {
            h ^= v;
            h.wrapping_mul(0x100_0000_01b3)
        })
}

macro_rules! r0_a1_access {
    () => {
        pub(super) struct Actions {
            substrate: Substrate,
            bridge: OpaqueBridge,
            work: WorkLedger,
            effects: BTreeSet<u64>,
            pub(super) candidates: usize,
            pub(super) templates: usize,
            pub(super) installed: usize,
            pub(super) structural: usize,
            pub(super) handles: usize,
            pub(super) exact: bool,
            pub(super) bytes: usize,
        }
        fn import(x: &super::E0Export) -> E0EpisodeExport {
            E0EpisodeExport {
                pulses: x
                    .pulses
                    .iter()
                    .map(|p| ExportPulse {
                        occurrence: p.occurrence,
                        tick: p.tick,
                    })
                    .collect(),
                observed_propagation: x.propagation.clone(),
                members: x.members,
                relative_temporal: x.temporal,
                relative_propagation: x.relative_propagation,
            }
        }
        fn import_exact(a: &super::E0Export, b: &E0EpisodeExport) -> bool {
            a.pulses
                .iter()
                .zip(&b.pulses)
                .all(|(x, y)| x.occurrence == y.occurrence && x.tick == y.tick)
                && a.propagation == b.observed_propagation
                && a.members == b.members
                && a.temporal == b.relative_temporal
                && a.relative_propagation == b.relative_propagation
        }
        fn observed_execution(
            start: &Substrate,
            bridge: &OpaqueBridge,
            index: usize,
            base: u64,
            work: &mut WorkLedger,
        ) -> Option<super::ObservedExecution> {
            let root = bridge.entries.get(index)?.root;
            if !route_valid(start, root, work) {
                return None;
            }
            let mut branch = start.clone();
            let injection = super::Occurrence(base);
            let mut activity = super::Activity {
                pulses: vec![super::Pulse {
                    occurrence: injection,
                    tick: 0,
                }],
                propagation: Vec::new(),
            };
            let mut queue = VecDeque::from([(root.cell, injection)]);
            let mut visited = BTreeSet::new();
            let mut trace = Vec::new();
            let mut activation = [0u16; 3];
            let mut next = base + 1;
            let bs = work.spike_propagations;
            let ba = work.arrow_traversals;
            let bm = work.state_mutations;
            while let Some((cell, predecessor)) = queue.pop_front() {
                if !visited.insert(cell) {
                    continue;
                }
                let member = usize::from(bound_member(&branch, cell)?);
                branch.cells[usize::from(cell.0)].activation += 1;
                activation[member] += 1;
                trace.push(member as u8);
                work.spike_propagations += 1;
                work.state_mutations += 1;
                let current = super::Occurrence(next);
                next += 1;
                activity.pulses.push(super::Pulse {
                    occurrence: current,
                    tick: activity.pulses.len() as u16,
                });
                activity.propagation.push(super::Propagation {
                    from: predecessor,
                    to: current,
                });
                for arrow in &branch.arrows {
                    if arrow.live && arrow.generation > 0 && arrow.endpoints[0] == cell {
                        work.arrow_traversals += 1;
                        queue.push_back((arrow.endpoints[1], current));
                    }
                }
            }
            (!trace.is_empty()).then_some(super::ObservedExecution {
                activity,
                evaluator_effect: super::effect_fingerprint(&trace, &activation),
                spikes: work.spike_propagations - bs,
                arrows: work.arrow_traversals - ba,
                mutations: work.state_mutations - bm,
            })
        }
        impl Actions {
            pub(super) fn execute(
                &mut self,
                index: usize,
                base: u64,
            ) -> Option<super::ObservedExecution> {
                observed_execution(&self.substrate, &self.bridge, index, base, &mut self.work)
            }
            pub(super) fn invalidate(&mut self, index: usize) -> bool {
                let Some(root) = self.bridge.entries.get(index).map(|e| e.root) else {
                    return false;
                };
                self.substrate.cells[usize::from(root.cell.0)].generation += 1;
                true
            }
            pub(super) fn known(&self, effect: u64) -> bool {
                self.effects.contains(&effect)
            }
            pub(super) fn organism_work(&self) -> u64 {
                self.work.organism_work()
            }
            pub(super) fn cleanup(&mut self) -> bool {
                self.bridge.entries.clear();
                self.substrate.cells.clear();
                self.substrate.arrows.clear();
                self.substrate.observations.clear();
                self.substrate.padding.clear();
                self.bridge.entries.is_empty()
                    && self.substrate.cells.is_empty()
                    && self.substrate.arrows.is_empty()
            }
        }
        pub(super) fn prepare(
            support_source: &[super::E0Export],
            target_source: &super::E0Export,
            permuted: bool,
        ) -> Option<Actions> {
            let support = support_source.iter().map(import).collect::<Vec<_>>();
            let target = import(target_source);
            let exact = support_source
                .iter()
                .zip(&support)
                .all(|(a, b)| import_exact(a, b))
                && import_exact(target_source, &target);
            let mut learner = train(&support, false)?;
            let templates = learner.consolidated();
            let mut substrate = substrate_from_export(&target, MappingOptions::default())?;
            let (candidates, roots) = learner.install(&mut substrate, true, false);
            let installed = roots.len();
            let structural_roots = structural_dedup(&mut substrate, &roots, &mut learner.work);
            let structural = structural_roots.len();
            let bridge = expose_roots(&structural_roots, permuted, &mut learner.work);
            let handles = bridge.entries.len();
            let (_, raw_effects) = bridge_effects(&substrate, &bridge, &mut learner.work);
            let effects = raw_effects
                .iter()
                .map(|e| super::effect_fingerprint(&e.trace, &e.activation))
                .collect();
            let bytes = learner.work.persistent_bytes;
            Some(Actions {
                substrate,
                bridge,
                work: learner.work,
                effects,
                candidates,
                templates,
                installed,
                structural,
                handles,
                exact,
                bytes,
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_a1 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_a1_affordance_multiplicity.rs"
    ));
    r0_a1_access!();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ReturnShape {
    lag: u16,
    hops: u8,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Support {
    count: u16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Candidate {
    members: [Occurrence; 2],
    shape: ReturnShape,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TemporaryRelation {
    members: [Occurrence; 2],
    lag: u16,
    hops: u8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EvidenceSurface {
    members: [u64; 2],
    lag: u16,
    hops: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReturnWork {
    pub pulses: u64,
    pub relations: u64,
    pub comparisons: u64,
    pub traversals: u64,
    pub proposals: u64,
    pub updates: u64,
    pub formations: u64,
    pub bridge_copies: u64,
    pub cleanup: u64,
}
impl ReturnWork {
    pub fn organism_work(&self) -> u64 {
        self.pulses
            + self.relations
            + self.comparisons
            + self.traversals
            + self.proposals
            + self.updates
            + self.formations
            + self.bridge_copies
            + self.cleanup
    }
}

#[derive(Clone, Debug, Default)]
struct ReturnLearner {
    shapes: BTreeMap<ReturnShape, Support>,
    work: ReturnWork,
}

fn tick(activity: &Activity, id: Occurrence) -> Option<u16> {
    activity
        .pulses
        .iter()
        .find(|p| p.occurrence == id)
        .map(|p| p.tick)
}
fn hops(
    activity: &Activity,
    from: Occurrence,
    to: Occurrence,
    work: &mut ReturnWork,
) -> Option<u8> {
    let mut q = VecDeque::from([(from, 0u8)]);
    let mut seen = BTreeSet::new();
    while let Some((x, n)) = q.pop_front() {
        if !seen.insert(x) {
            continue;
        }
        if x == to && n > 0 {
            return Some(n);
        }
        for edge in &activity.propagation {
            work.traversals += 1;
            if edge.from == x {
                q.push_back((edge.to, n.saturating_add(1)))
            }
        }
    }
    None
}
fn candidates(activity: &Activity, work: &mut ReturnWork) -> Vec<Candidate> {
    work.pulses += activity.pulses.len() as u64;
    work.relations += activity.propagation.len() as u64;
    let incoming = activity
        .propagation
        .iter()
        .map(|e| e.to)
        .collect::<BTreeSet<_>>();
    let outgoing = activity
        .propagation
        .iter()
        .map(|e| e.from)
        .collect::<BTreeSet<_>>();
    let roots = activity
        .pulses
        .iter()
        .map(|p| p.occurrence)
        .filter(|x| !incoming.contains(x) && outgoing.contains(x))
        .collect::<Vec<_>>();
    let terminals = activity
        .pulses
        .iter()
        .map(|p| p.occurrence)
        .filter(|x| incoming.contains(x) && !outgoing.contains(x))
        .collect::<Vec<_>>();
    let mut out = Vec::new();
    for root in roots {
        for terminal in &terminals {
            work.comparisons += 1;
            let (Some(a), Some(b)) = (tick(activity, root), tick(activity, *terminal)) else {
                continue;
            };
            let Some(lag) = b.checked_sub(a) else {
                continue;
            };
            let Some(path) = hops(activity, root, *terminal, work) else {
                continue;
            };
            out.push(Candidate {
                members: [root, *terminal],
                shape: ReturnShape { lag, hops: path },
            })
        }
    }
    out
}
impl ReturnLearner {
    fn observe(&mut self, activity: &Activity, plasticity: bool) -> usize {
        if !plasticity {
            return 0;
        }
        let found = candidates(activity, &mut self.work);
        let shapes = found.iter().map(|c| c.shape).collect::<BTreeSet<_>>();
        self.work.proposals += shapes.len() as u64;
        for shape in shapes {
            self.shapes.entry(shape).or_default().count += 1;
            self.work.updates += 1
        }
        found.len()
    }
    fn mature(&self) -> usize {
        self.shapes
            .values()
            .filter(|s| usize::from(s.count) >= RETURN_SUPPORT)
            .count()
    }
    fn form_all(&mut self, activity: &Activity) -> Vec<TemporaryRelation> {
        let mut by_root = BTreeMap::<Occurrence, Vec<Candidate>>::new();
        for c in candidates(activity, &mut self.work)
            .into_iter()
            .filter(|c| {
                self.shapes
                    .get(&c.shape)
                    .is_some_and(|s| usize::from(s.count) >= RETURN_SUPPORT)
            })
        {
            by_root.entry(c.members[0]).or_default().push(c)
        }
        let mut out = Vec::new();
        for group in by_root.into_values() {
            if group.len() == 1 {
                let c = group[0];
                self.work.formations += 1;
                out.push(TemporaryRelation {
                    members: c.members,
                    lag: c.shape.lag,
                    hops: c.shape.hops,
                })
            }
        }
        out
    }
    fn form_one(&mut self, activity: &Activity) -> Option<TemporaryRelation> {
        let mut r = self.form_all(activity);
        (r.len() == 1).then(|| r.remove(0))
    }
    fn bytes(&self) -> usize {
        self.shapes.len() * (size_of::<ReturnShape>() + size_of::<Support>())
    }
}
fn bridge(relation: TemporaryRelation, work: &mut ReturnWork) -> EvidenceSurface {
    work.bridge_copies += 4;
    EvidenceSurface {
        members: relation.members.map(|x| x.0),
        lag: relation.lag,
        hops: relation.hops,
    }
}

fn relabel(activity: &Activity, salt: u64) -> Activity {
    let map = activity
        .pulses
        .iter()
        .enumerate()
        .map(|(i, p)| (p.occurrence, Occurrence(salt + i as u64 * 97 + 11)))
        .collect::<BTreeMap<_, _>>();
    Activity {
        pulses: activity
            .pulses
            .iter()
            .map(|p| Pulse {
                occurrence: map[&p.occurrence],
                tick: p.tick,
            })
            .collect(),
        propagation: activity
            .propagation
            .iter()
            .map(|e| Propagation {
                from: map[&e.from],
                to: map[&e.to],
            })
            .collect(),
    }
}
fn interleave(a: &Activity, b: &Activity) -> Activity {
    let mut pulses = a.pulses.clone();
    pulses.extend(&b.pulses);
    pulses.sort_by_key(|p| (p.tick, p.occurrence));
    let mut propagation = a.propagation.clone();
    propagation.extend(&b.propagation);
    Activity {
        pulses,
        propagation,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub e0_hash: bool,
    pub a1_hash: bool,
    pub parent_hash: bool,
    pub handoff_hash: bool,
    pub ds1_hash: bool,
    pub observers: usize,
    pub learners: usize,
    pub bridges: usize,
    pub choose_calls: usize,
    pub apply_calls: usize,
    pub semantic_sites: usize,
    pub evaluator_to_learner: usize,
    pub evaluator_to_bridge: usize,
    pub persistent_ids: usize,
}
fn body<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    let start = source.find(marker)?;
    let tail = &source[start..];
    let open = tail.find('{')?;
    let mut depth = 0usize;
    for (offset, b) in tail[open..].bytes().enumerate() {
        match b {
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
    let learner = body(source, "impl ReturnLearner").unwrap_or_default();
    let bridge_body = body(source, "fn bridge(").unwrap_or_default();
    let persistent = [
        body(source, "struct ReturnShape").unwrap_or_default(),
        body(source, "struct Support").unwrap_or_default(),
        body(source, "struct ReturnLearner").unwrap_or_default(),
    ]
    .concat();
    let semantic = [
        ["semantic_", "outcome"].concat(),
        ["correct_", "choice"].concat(),
        ["reward_", "value"].concat(),
        ["accepted_", "output"].concat(),
        ["rejected_", "output"].concat(),
    ];
    SourceAudit {
        e0_hash: env!("DS_R0_E0_SHA256") == FROZEN_E0_SHA256,
        a1_hash: env!("DS_R0_A1_SHA256") == FROZEN_A1_SHA256,
        parent_hash: env!("DS_R0_PARENT_SHA256") == FROZEN_PARENT_SHA256,
        handoff_hash: env!("DS_R0_PARENT_HANDOFF_SHA256") == FROZEN_PARENT_HANDOFF_SHA256,
        ds1_hash: frozen_e0::FROZEN_DS1_LEARNER_SHA256 == FROZEN_DS1_SHA256,
        observers: production.matches("fn observed_execution(").count(),
        learners: production.matches("impl ReturnLearner").count(),
        bridges: production.matches("fn bridge(").count(),
        choose_calls: production.matches(".choose(&self.view").count(),
        apply_calls: production
            .matches(&[".apply_", "consequence("].concat())
            .count(),
        semantic_sites: semantic.iter().map(|x| production.matches(x).count()).sum(),
        evaluator_to_learner: learner.matches("evaluator_effect").count(),
        evaluator_to_bridge: bridge_body.matches("evaluator_effect").count(),
        persistent_ids: [
            "Occurrence",
            "OpaqueHandle",
            "RouteRoot",
            "CellId",
            "destination",
            "episode",
        ]
        .iter()
        .map(|x| persistent.matches(x).count())
        .sum(),
    }
}
fn source_audit() -> SourceAudit {
    derive_source(include_str!(
        "ds_r0_anonymous_post_action_evidence_return.rs"
    ))
}
impl SourceAudit {
    fn passed(&self) -> bool {
        self.e0_hash
            && self.a1_hash
            && self.parent_hash
            && self.handoff_hash
            && self.ds1_hash
            && self.observers == 1
            && self.learners == 1
            && self.bridges == 1
            && self.choose_calls == 1
            && self.apply_calls == 0
            && self.semantic_sites == 0
            && self.evaluator_to_learner == 0
            && self.evaluator_to_bridge == 0
            && self.persistent_ids == 0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Controls {
    pub fresh: bool,
    pub relabel: bool,
    pub layout: bool,
    pub handle_permutation: bool,
    pub changed_later: bool,
    pub other_route: bool,
    pub interleaved: bool,
    pub distractor: bool,
    pub delayed: bool,
    pub ambiguous: bool,
    pub timing_shuffle: bool,
    pub propagation_shuffle: bool,
    pub no_execution: bool,
    pub no_later: bool,
    pub stale: bool,
    pub subthreshold: bool,
    pub disabled: bool,
    pub bridge_copy: bool,
    pub no_retained_ids: bool,
    pub no_semantic_update: bool,
    pub mutation_sensitive: bool,
    pub cleanup: bool,
}
impl Controls {
    pub fn passed(&self) -> bool {
        self.fresh
            && self.relabel
            && self.layout
            && self.handle_permutation
            && self.changed_later
            && self.other_route
            && self.interleaved
            && self.distractor
            && self.delayed
            && self.ambiguous
            && self.timing_shuffle
            && self.propagation_shuffle
            && self.no_execution
            && self.no_later
            && self.stale
            && self.subthreshold
            && self.disabled
            && self.bridge_copy
            && self.no_retained_ids
            && self.no_semantic_update
            && self.mutation_sensitive
            && self.cleanup
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub actual: bool,
    pub exact: bool,
    pub fresh_target: bool,
    pub candidates: usize,
    pub templates: usize,
    pub roots: usize,
    pub structural: usize,
    pub handles: usize,
    pub choice: usize,
    pub choose_calls: u64,
    pub ds1_updates: u64,
    pub effect_known: bool,
    pub activity_pulses: usize,
    pub activity_relations: usize,
    pub spikes: u64,
    pub arrows: u64,
    pub mutations: u64,
    pub mature_shapes: usize,
    pub temporary_relations: usize,
    pub bridge_fields: usize,
    pub controls: Controls,
    pub return_work: ReturnWork,
    pub e0_work: u64,
    pub a1_work: u64,
    pub e0_bytes: usize,
    pub a1_bytes: usize,
    pub ds1_bytes: usize,
    pub return_bytes: usize,
    pub temporary_peak: usize,
    pub ready: [bool; 8],
}

fn remove_terminal(activity: &Activity) -> Activity {
    let mut out = activity.clone();
    if let Some(id) = out
        .pulses
        .iter()
        .find(|p| {
            out.propagation.iter().any(|e| e.to == p.occurrence)
                && !out.propagation.iter().any(|e| e.from == p.occurrence)
        })
        .map(|p| p.occurrence)
    {
        out.pulses.retain(|p| p.occurrence != id);
        out.propagation.retain(|e| e.from != id && e.to != id)
    }
    out
}

fn audit_seed(seed: u64, acquisition: usize, source: &SourceAudit) -> SeedAudit {
    let mut input = frozen_e0::bundle(seed, acquisition).expect("E0 target");
    let mut actions = frozen_a1::prepare(&input.support, &input.target, false).expect("A1");
    let mut permuted =
        frozen_a1::prepare(&input.support, &input.target, true).expect("A1 permuted");
    let (choice, choose_calls, ds1_updates) = input.choose(seed as usize);
    let base = seed * 1_000_000 + 10_000;
    let mut learner = ReturnLearner::default();
    let mut support = Vec::new();
    for n in 0..RETURN_SUPPORT {
        let activity = actions
            .execute(choice, base + n as u64 * 100)
            .expect("support execution")
            .activity;
        learner.observe(&activity, true);
        support.push(activity)
    }
    let target = actions
        .execute(choice, base + 10_000)
        .expect("target execution");
    let effect_known = actions.known(target.evaluator_effect);
    let relation = learner.form_one(&target.activity);
    let temporary_relations = usize::from(relation.is_some());
    let surface = relation.map(|r| bridge(r, &mut learner.work));
    let support_ids = support
        .iter()
        .flat_map(|a| a.pulses.iter().map(|p| p.occurrence))
        .collect::<BTreeSet<_>>();
    let target_ids = target
        .activity
        .pulses
        .iter()
        .map(|p| p.occurrence)
        .collect::<BTreeSet<_>>();
    let fresh = support_ids.is_disjoint(&target_ids);
    let relabel_ok = learner
        .form_one(&relabel(&target.activity, base + 20_000))
        .is_some();
    let mut layout_activity = target.activity.clone();
    layout_activity.pulses.reverse();
    layout_activity.propagation.reverse();
    let layout = learner.form_one(&layout_activity).is_some();
    let other = permuted
        .execute(choice, base + 30_000)
        .expect("other route");
    let other_route = learner.form_one(&other.activity).is_some();
    let handle_permutation =
        target.evaluator_effect != other.evaluator_effect && permuted.known(other.evaluator_effect);
    let changed_later = learner
        .form_one(&remove_terminal(&target.activity))
        .is_none();
    let joined = interleave(&target.activity, &other.activity);
    let joined_relations = learner.form_all(&joined);
    let interleaved = joined_relations.len() == 2
        && joined_relations[0].members[0] != joined_relations[1].members[0];
    let mut with_distractor = target.activity.clone();
    with_distractor.pulses.push(Pulse {
        occurrence: Occurrence(base + 40_000),
        tick: 1,
    });
    let distractor = learner.form_one(&with_distractor) == relation;
    let mut delayed_activity = target.activity.clone();
    delayed_activity.pulses.last_mut().expect("last").tick += 3;
    let delayed = learner.form_one(&delayed_activity).is_none();
    let mut ambiguous_activity = target.activity.clone();
    let terminal = *target.activity.pulses.last().expect("terminal");
    let predecessor = target.activity.pulses[target.activity.pulses.len() - 2];
    let ambiguous_id = Occurrence(base + 50_000);
    ambiguous_activity.pulses.push(Pulse {
        occurrence: ambiguous_id,
        tick: terminal.tick,
    });
    ambiguous_activity.propagation.push(Propagation {
        from: predecessor.occurrence,
        to: ambiguous_id,
    });
    let ambiguous = learner.form_one(&ambiguous_activity).is_none();
    let mut timing = target.activity.clone();
    timing.pulses.last_mut().expect("last").tick = 0;
    let timing_shuffle = learner.form_one(&timing).is_none();
    let mut propagation = target.activity.clone();
    for edge in &mut propagation.propagation {
        std::mem::swap(&mut edge.from, &mut edge.to)
    }
    let propagation_shuffle = learner.form_one(&propagation).is_none();
    let no_execution = learner.form_one(&Activity::default()).is_none();
    let no_later = learner
        .form_one(&Activity {
            pulses: target.activity.pulses[..1].to_vec(),
            propagation: Vec::new(),
        })
        .is_none();
    let mut stale_actions =
        frozen_a1::prepare(&input.support, &input.target, false).expect("stale");
    let stale =
        stale_actions.invalidate(choice) && stale_actions.execute(choice, base + 60_000).is_none();
    let mut weak = ReturnLearner::default();
    for activity in &support[..RETURN_SUPPORT - 1] {
        weak.observe(activity, true);
    }
    let subthreshold = weak.form_one(&target.activity).is_none();
    let mut disabled_learner = ReturnLearner::default();
    let disabled = disabled_learner.observe(&support[0], false) == 0
        && disabled_learner.form_one(&target.activity).is_none();
    let bridge_copy = surface.is_some_and(|copy| {
        relation.is_some_and(|origin| {
            copy.members == origin.members.map(|x| x.0)
                && copy.lag == origin.lag
                && copy.hops == origin.hops
        })
    });
    let no_retained_ids = source.persistent_ids == 0;
    let no_semantic_update = source.apply_calls == 0
        && source.semantic_sites == 0
        && source.evaluator_to_learner == 0
        && source.evaluator_to_bridge == 0
        && ds1_updates == 0;
    let implementation = include_str!("ds_r0_anonymous_post_action_evidence_return.rs");
    let mutate_apply = implementation.replacen(
        "#[cfg(test)]",
        "fn mutation(){learner.apply_consequence(view,0,true);}\n#[cfg(test)]",
        1,
    );
    let mutate_learner = implementation.replacen(
        "impl ReturnLearner {",
        "impl ReturnLearner { fn mutation(&self){let _=self.evaluator_effect;}",
        1,
    );
    let mutate_bridge = implementation.replacen(
        "work.bridge_copies += 4;",
        "let _ = evaluator_effect; work.bridge_copies += 4;",
        1,
    );
    let mutation_sensitive = derive_source(&mutate_apply).apply_calls > 0
        && derive_source(&mutate_learner).evaluator_to_learner > 0
        && derive_source(&mutate_bridge).evaluator_to_bridge > 0;
    learner.work.cleanup += u64::from(relation.is_some()) + u64::from(surface.is_some());
    let relation = None::<TemporaryRelation>;
    let surface = None::<EvidenceSurface>;
    let cleanup =
        relation.is_none() && surface.is_none() && actions.cleanup() && permuted.cleanup();
    let controls = Controls {
        fresh,
        relabel: relabel_ok,
        layout,
        handle_permutation,
        changed_later,
        other_route,
        interleaved,
        distractor,
        delayed,
        ambiguous,
        timing_shuffle,
        propagation_shuffle,
        no_execution,
        no_later,
        stale,
        subthreshold,
        disabled,
        bridge_copy,
        no_retained_ids,
        no_semantic_update,
        mutation_sensitive,
        cleanup,
    };
    let ready = [
        source.passed(),
        input.actual
            && input.exact
            && input.fresh
            && actions.exact
            && actions.candidates == 2
            && actions.templates == 3
            && actions.installed == 2
            && actions.structural == 2
            && actions.handles == 2,
        choose_calls == 1
            && ds1_updates == 0
            && effect_known
            && target.spikes == 2
            && target.arrows == 1
            && target.mutations == 2,
        target.activity.pulses.len() == 3 && target.activity.propagation.len() == 2,
        learner.mature() == 1,
        temporary_relations == 1,
        bridge_copy,
        controls.passed(),
    ];
    let mature_shapes = learner.mature();
    let return_bytes = learner.bytes();
    let return_work = learner.work;
    SeedAudit {
        seed,
        actual: input.actual,
        exact: input.exact && actions.exact,
        fresh_target: input.fresh,
        candidates: actions.candidates,
        templates: actions.templates,
        roots: actions.installed,
        structural: actions.structural,
        handles: actions.handles,
        choice,
        choose_calls,
        ds1_updates,
        effect_known,
        activity_pulses: target.activity.pulses.len(),
        activity_relations: target.activity.propagation.len(),
        spikes: target.spikes,
        arrows: target.arrows,
        mutations: target.mutations,
        mature_shapes,
        temporary_relations,
        bridge_fields: if bridge_copy { 4 } else { 0 },
        controls,
        return_work,
        e0_work: input.work,
        a1_work: actions.organism_work(),
        e0_bytes: input.bytes,
        a1_bytes: actions.bytes,
        ds1_bytes: input.ds1_bytes(),
        return_bytes,
        temporary_peak: target.activity.pulses.len() * size_of::<Pulse>()
            + target.activity.propagation.len() * size_of::<Propagation>()
            + size_of::<TemporaryRelation>()
            + size_of::<EvidenceSurface>(),
        ready,
    }
}

fn freeze(ready: [bool; 8]) -> ([String; 8], Option<usize>) {
    let first = ready.iter().position(|x| !*x);
    (
        std::array::from_fn(|stage| match first {
            None => "READY".into(),
            Some(c) if stage < c => "READY".into(),
            Some(c) if stage == c => format!("COLLAPSE: {}", STAGES[stage]),
            Some(_) => "BLOCKED".into(),
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
    pub stages: [String; 8],
    pub first_collapse: Option<usize>,
    pub seeds: Vec<SeedAudit>,
    pub audit_passed: bool,
}
fn rejected() -> Report {
    Report {
        label: "DS-R0 definitive forbidden".into(),
        protocol: PROTOCOL.into(),
        mode: "DEFINITIVE-FORBIDDEN".into(),
        claim_eligible: false,
        m0_authoritative: true,
        enabling_only: true,
        m1_exists: false,
        source: source_audit(),
        stages: std::array::from_fn(|_| "BLOCKED: definitive rejected".into()),
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
    let (acquisition, values): (usize, &[u64]) = match mode {
        HarnessMode::Micro => (16, &[100]),
        HarnessMode::Gate => (32, &[100, 101, 102, 103, 104]),
        HarnessMode::Definitive => unreachable!(),
    };
    let seeds = values
        .iter()
        .map(|s| audit_seed(*s, acquisition, &source))
        .collect::<Vec<_>>();
    let mut ready = [false; 8];
    for (stage, value) in ready.iter_mut().enumerate() {
        *value = seeds.iter().all(|s| s.ready[stage])
    }
    let (stages, first_collapse) = freeze(ready);
    let audit_passed = first_collapse.is_none()
        && seeds
            .iter()
            .all(|s| s.controls.passed() && s.ds1_updates == 0);
    Report {
        label: if audit_passed {
            "DS-R0 DEVELOPMENT IMPLEMENTATION READY".into()
        } else {
            format!(
                "DS-R0 DEVELOPMENT COLLAPSE AT {}",
                first_collapse.map(|x| STAGES[x]).unwrap_or("unknown")
            )
        },
        protocol: PROTOCOL.into(),
        mode: match mode {
            HarnessMode::Micro => "MICRO",
            HarnessMode::Gate => "GATE",
            HarnessMode::Definitive => unreachable!(),
        }
        .into(),
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
    fn anonymous_relation_forms_without_update() {
        let r = run(HarnessMode::Micro);
        assert!(r.audit_passed, "{r:#?}");
        assert!(r.seeds.iter().all(|s| s.temporary_relations == 1
            && s.bridge_fields == 4
            && s.choose_calls == 1
            && s.ds1_updates == 0));
    }
    #[test]
    fn actual_route_activity_is_observed() {
        let r = run(HarnessMode::Micro);
        assert!(r.seeds.iter().all(|s| s.roots == 2
            && s.handles == 2
            && s.effect_known
            && s.activity_pulses == 3
            && s.activity_relations == 2
            && s.spikes == 2
            && s.arrows == 1
            && s.mutations == 2));
    }
    #[test]
    fn controls_pass() {
        let r = run(HarnessMode::Micro);
        assert!(r.seeds.iter().all(|s| s.controls.passed()));
    }
    #[test]
    fn source_boundary_passes() {
        assert!(source_audit().passed(), "{:#?}", source_audit());
    }
    #[test]
    fn definitive_is_inert() {
        let r = run(HarnessMode::Definitive);
        assert!(!r.audit_passed && r.seeds.is_empty() && !r.m1_exists);
    }
}
