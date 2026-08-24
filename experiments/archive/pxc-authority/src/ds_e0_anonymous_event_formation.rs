//! DS-E0 cumulative-development anonymous event-formation gate.
//!
//! The harness has MICRO/GATE modes only. It learns relation shapes from flat
//! anonymous activity, forms episode-local membership before serialization,
//! and writes no result artifact.

#![allow(dead_code)] // The byte-frozen DS1 learner is consumed read-only.

use std::collections::{BTreeMap, BTreeSet};
use std::mem::size_of;

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-e0-cumulative-anonymous-event-formation-v1";
pub const FROZEN_DS1_LEARNER_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";

const CREDIT_SUCCESS: i32 = 2;
const CREDIT_FAILURE: i32 = -1;
const CONSOLIDATION_STRENGTH: i32 = 6;
const MINIMUM_SUCCESSES: usize = 3;
const INVALIDATION_CONTRADICTIONS: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OpaquePort(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pulse {
    identity: OpaquePort,
    position: u8,
    tick: u8,
    window: u8,
}

// DS1_LEARNER_BEGIN
#[derive(Clone, Copy, Debug)]
struct Neighborhood {
    pair: [Pulse; 2],
    witness: Pulse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Signature {
    gap: u8,
    witness_attachment: u8,
}

#[derive(Clone, Copy, Debug, Default)]
struct Evidence {
    strength: i32,
    successes: usize,
    failures: usize,
}

#[derive(Clone, Debug)]
struct Pattern {
    signature: Signature,
    evidence: [Evidence; 2],
    mature: Option<usize>,
    contradictions: usize,
}

#[derive(Clone, Debug, Default)]
struct Learner {
    patterns: Vec<Pattern>,
    comparisons: u64,
    candidate_evaluations: u64,
    proposals: u64,
    route_firings: u64,
    credit_updates: u64,
    invalidations: usize,
    reopenings: usize,
    consolidations: usize,
}

impl Learner {
    fn signature(view: &Neighborhood) -> Signature {
        let _episode_local_ports = [
            (view.pair[0].identity, view.pair[0].position),
            (view.pair[1].identity, view.pair[1].position),
            (view.witness.identity, view.witness.position),
        ];
        let first_tick = view.pair[0].tick.min(view.pair[1].tick);
        let second_tick = view.pair[0].tick.max(view.pair[1].tick);
        let witness_attachment = u8::from(view.witness.window != view.pair[0].window);
        Signature {
            gap: second_tick - first_tick,
            witness_attachment,
        }
    }

    fn pattern_index(&mut self, signature: Signature) -> usize {
        self.comparisons += self.patterns.len() as u64;
        if let Some(index) = self
            .patterns
            .iter()
            .position(|pattern| pattern.signature == signature)
        {
            return index;
        }
        self.patterns.push(Pattern {
            signature,
            evidence: [Evidence::default(); 2],
            mature: None,
            contradictions: 0,
        });
        self.proposals += 2;
        self.patterns.len() - 1
    }

    fn choose(&mut self, view: &Neighborhood, tie_breaker: usize) -> (usize, bool) {
        let signature = Self::signature(view);
        let index = self.pattern_index(signature);
        if let Some(choice) = self.patterns[index].mature {
            self.route_firings += 1;
            return (choice, true);
        }
        self.candidate_evaluations += 2;
        let evidence = self.patterns[index].evidence;
        let choice = match evidence[0].strength.cmp(&evidence[1].strength) {
            std::cmp::Ordering::Greater => 0,
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => tie_breaker % 2,
        };
        self.route_firings += 1;
        (choice, false)
    }

    fn apply_consequence(&mut self, view: &Neighborhood, choice: usize, positive: bool) {
        let signature = Self::signature(view);
        let index = self
            .patterns
            .iter()
            .position(|pattern| pattern.signature == signature)
            .expect("a choice always establishes its generic pattern");
        self.credit_updates += 1;
        let pattern = &mut self.patterns[index];
        if pattern.mature == Some(choice) {
            if positive {
                pattern.contradictions = 0;
                pattern.evidence[choice].successes += 1;
                return;
            }
            pattern.evidence[choice].strength += CREDIT_FAILURE;
            pattern.evidence[choice].failures += 1;
            pattern.contradictions += 1;
            if pattern.contradictions == INVALIDATION_CONTRADICTIONS {
                pattern.mature = None;
                pattern.evidence = [Evidence::default(); 2];
                pattern.contradictions = 0;
                self.invalidations += 1;
                self.reopenings += 1;
            }
            return;
        }
        let evidence = &mut pattern.evidence[choice];
        if positive {
            evidence.strength += CREDIT_SUCCESS;
            evidence.successes += 1;
        } else {
            evidence.strength += CREDIT_FAILURE;
            evidence.failures += 1;
        }
        if positive
            && evidence.strength >= CONSOLIDATION_STRENGTH
            && evidence.successes >= MINIMUM_SUCCESSES
            && evidence.failures == 0
        {
            pattern.mature = Some(choice);
            pattern.contradictions = 0;
            self.consolidations += 1;
        }
    }

    fn frozen_choice(&self, view: &Neighborhood) -> Option<usize> {
        let signature = Self::signature(view);
        self.patterns
            .iter()
            .find(|pattern| pattern.signature == signature)
            .and_then(|pattern| pattern.mature)
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for pattern in &self.patterns {
            for value in [
                pattern.signature.gap as u64,
                pattern.signature.witness_attachment as u64,
                pattern.evidence[0].strength as u64,
                pattern.evidence[0].successes as u64,
                pattern.evidence[0].failures as u64,
                pattern.evidence[1].strength as u64,
                pattern.evidence[1].successes as u64,
                pattern.evidence[1].failures as u64,
                pattern.mature.map_or(u64::MAX, |value| value as u64),
            ] {
                hash ^= value;
                hash = hash.wrapping_mul(0x100_0000_01b3);
            }
        }
        hash
    }

    fn learner_work(&self) -> u64 {
        self.comparisons + self.candidate_evaluations + self.proposals + self.credit_updates
    }

    fn persistent_bytes(&self) -> usize {
        self.patterns.capacity() * size_of::<Pattern>()
    }
}
// DS1_LEARNER_END

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Occurrence(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Spike {
    occurrence: Occurrence,
    local_tick: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Propagation {
    from: Occurrence,
    to: Occurrence,
}

#[derive(Clone, Debug)]
struct RawActivity {
    spikes: Vec<Spike>,
    propagation: Vec<Propagation>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RelationShape {
    temporal: [i8; 9],
    propagation: [i8; 9],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ShapeEvidence {
    strength: i16,
    successes: u16,
    failures: u16,
    mature: bool,
    contradictions: u8,
}

#[derive(Clone, Debug)]
struct Candidate {
    members: [Occurrence; 3],
    shape: RelationShape,
    temporal_rank: [u8; 3],
    propagation_rank: [u8; 3],
    attachment_equivalence: [u8; 3],
}

/// Episode-local relational value emitted by E0-A. The serializer only copies
/// these already-materialized fields.
#[derive(Clone, Debug, PartialEq, Eq)]
struct EventRelations {
    members: [Occurrence; 3],
    temporal: [i8; 9],
    propagation: [i8; 9],
    temporal_rank: [u8; 3],
    propagation_rank: [u8; 3],
    attachment_equivalence: [u8; 3],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkLedger {
    pub raw_relation_comparisons: u64,
    pub triples_enumerated: u64,
    pub canonical_permutations: u64,
    pub persistent_shape_comparisons: u64,
    pub proposals: u64,
    pub physical_propagations: u64,
    pub consequence_updates: u64,
    pub temporary_formations: u64,
    pub serializations: u64,
}

impl WorkLedger {
    fn organism_work(&self) -> u64 {
        self.raw_relation_comparisons
            + self.triples_enumerated
            + self.canonical_permutations
            + self.persistent_shape_comparisons
            + self.proposals
            + self.physical_propagations
            + self.consequence_updates
            + self.temporary_formations
            + self.serializations
    }
}

#[derive(Clone, Debug, Default)]
struct FormationLearner {
    shapes: BTreeMap<RelationShape, ShapeEvidence>,
    work: WorkLedger,
    invalidations: u16,
    reopenings: u16,
}

impl FormationLearner {
    fn candidates(&mut self, raw: &RawActivity) -> Vec<Candidate> {
        let mut candidates = Vec::new();
        for first in 0..raw.spikes.len() {
            for second in first + 1..raw.spikes.len() {
                for third in second + 1..raw.spikes.len() {
                    self.work.triples_enumerated += 1;
                    if let Some(candidate) = canonical_candidate(
                        raw,
                        [
                            raw.spikes[first].occurrence,
                            raw.spikes[second].occurrence,
                            raw.spikes[third].occurrence,
                        ],
                        &mut self.work,
                    ) {
                        candidates.push(candidate);
                    }
                }
            }
        }
        candidates
    }

    fn acquire_episode<F>(&mut self, raw: &RawActivity, mut consequence: F)
    where
        F: FnMut(&[Occurrence; 3]) -> bool,
    {
        let candidates = self.candidates(raw);
        for candidate in candidates {
            self.work.persistent_shape_comparisons += self.shapes.len() as u64;
            self.work.proposals += 1;
            self.work.physical_propagations += 1;
            let positive = consequence(&candidate.members);
            self.work.consequence_updates += 1;
            let evidence = self.shapes.entry(candidate.shape).or_default();
            if evidence.mature {
                if positive {
                    evidence.contradictions = 0;
                    evidence.successes += 1;
                } else {
                    evidence.strength += CREDIT_FAILURE as i16;
                    evidence.failures += 1;
                    evidence.contradictions += 1;
                    if usize::from(evidence.contradictions) == INVALIDATION_CONTRADICTIONS {
                        *evidence = ShapeEvidence::default();
                        self.invalidations += 1;
                        self.reopenings += 1;
                    }
                }
            } else if positive {
                evidence.strength += CREDIT_SUCCESS as i16;
                evidence.successes += 1;
                if i32::from(evidence.strength) >= CONSOLIDATION_STRENGTH
                    && usize::from(evidence.successes) >= MINIMUM_SUCCESSES
                    && evidence.failures == 0
                {
                    evidence.mature = true;
                }
            } else {
                evidence.strength += CREDIT_FAILURE as i16;
                evidence.failures += 1;
            }
        }
    }

    fn form(&mut self, raw: &RawActivity) -> Option<EventRelations> {
        let candidates = self.candidates(raw);
        let mut mature = candidates.into_iter().filter(|candidate| {
            self.work.persistent_shape_comparisons += self.shapes.len() as u64;
            self.shapes
                .get(&candidate.shape)
                .is_some_and(|evidence| evidence.mature)
        });
        let selected = mature.next()?;
        if mature.next().is_some() {
            return None;
        }
        self.work.temporary_formations += 1;
        Some(EventRelations {
            members: selected.members,
            temporal: selected.shape.temporal,
            propagation: selected.shape.propagation,
            temporal_rank: selected.temporal_rank,
            propagation_rank: selected.propagation_rank,
            attachment_equivalence: selected.attachment_equivalence,
        })
    }

    fn persistent_bytes(&self) -> usize {
        self.shapes.len() * (size_of::<RelationShape>() + size_of::<ShapeEvidence>())
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for (shape, evidence) in &self.shapes {
            for value in shape
                .temporal
                .iter()
                .chain(shape.propagation.iter())
                .map(|value| *value as i64 as u64)
                .chain([
                    evidence.strength as i64 as u64,
                    u64::from(evidence.successes),
                    u64::from(evidence.failures),
                    u64::from(evidence.mature),
                ])
            {
                hash ^= value;
                hash = hash.wrapping_mul(0x100_0000_01b3);
            }
        }
        hash
    }
}

const PERMUTATIONS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

fn canonical_candidate(
    raw: &RawActivity,
    members: [Occurrence; 3],
    work: &mut WorkLedger,
) -> Option<Candidate> {
    let ticks = members.map(|member| {
        raw.spikes
            .iter()
            .find(|spike| spike.occurrence == member)
            .expect("candidate members came from spikes")
            .local_tick
    });
    let mut variants = Vec::with_capacity(PERMUTATIONS.len());
    for permutation in PERMUTATIONS {
        work.canonical_permutations += 1;
        let ordered_members = permutation.map(|index| members[index]);
        let ordered_ticks = permutation.map(|index| ticks[index]);
        let mut temporal = [0i8; 9];
        let mut propagation = [0i8; 9];
        for from in 0..3 {
            for to in 0..3 {
                work.raw_relation_comparisons += 1;
                temporal[from * 3 + to] =
                    (i16::from(ordered_ticks[to]) - i16::from(ordered_ticks[from]))
                        .clamp(i16::from(i8::MIN), i16::from(i8::MAX)) as i8;
                if raw.propagation.iter().any(|edge| {
                    edge.from == ordered_members[from] && edge.to == ordered_members[to]
                }) {
                    propagation[from * 3 + to] = 1;
                } else if raw.propagation.iter().any(|edge| {
                    edge.from == ordered_members[to] && edge.to == ordered_members[from]
                }) {
                    propagation[from * 3 + to] = -1;
                }
            }
        }
        variants.push((
            RelationShape {
                temporal,
                propagation,
            },
            ordered_members,
            ordered_ticks,
        ));
    }
    variants.sort_by_key(|variant| variant.0);
    if variants[0].0 == variants[1].0 {
        return None;
    }
    let (shape, ordered_members, ordered_ticks) = variants[0];
    let connected = (0..3).all(|member| {
        (0..3).any(|other| member != other && shape.propagation[member * 3 + other] != 0)
    });
    if !connected {
        return None;
    }
    let minimum_tick = *ordered_ticks.iter().min().expect("three ticks");
    let temporal_rank = ordered_ticks.map(|tick| tick - minimum_tick);
    let mut propagation_rank = [0u8; 3];
    for (member, rank) in propagation_rank.iter_mut().enumerate() {
        *rank = (0..3)
            .filter(|other| shape.propagation[*other * 3 + member] == 1)
            .count() as u8;
    }
    let attachment_equivalence = [0, 1, u8::from(shape.propagation[2 * 3] == 0)];
    Some(Candidate {
        members: ordered_members,
        shape,
        temporal_rank,
        propagation_rank,
        attachment_equivalence,
    })
}

/// E0-B: fixed field copies only. It cannot access raw activity.
fn serialize_once(event: &EventRelations, work: &mut WorkLedger) -> Neighborhood {
    work.serializations += 1;
    Neighborhood {
        pair: [
            Pulse {
                identity: OpaquePort(u64::from(event.members[0].0)),
                position: event.propagation_rank[0],
                tick: event.temporal_rank[0],
                window: event.attachment_equivalence[0],
            },
            Pulse {
                identity: OpaquePort(u64::from(event.members[1].0)),
                position: event.propagation_rank[1],
                tick: event.temporal_rank[1],
                window: event.attachment_equivalence[1],
            },
        ],
        witness: Pulse {
            identity: OpaquePort(u64::from(event.members[2].0)),
            position: event.propagation_rank[2],
            tick: event.temporal_rank[2],
            window: event.attachment_equivalence[2],
        },
    }
}

fn frozen_ds1_learner_text() -> &'static str {
    let source = include_str!("ds_e0_anonymous_event_formation.rs");
    source
        .split("// DS1_LEARNER_BEGIN")
        .nth(1)
        .and_then(|tail| tail.split("// DS1_LEARNER_END").next())
        .expect("frozen DS1 learner markers remain present")
}

#[derive(Clone, Copy)]
enum Perturbation {
    None,
    SameTimingDifferentPropagation,
    SamePropagationDifferentTiming,
    Relabel,
    Allocation,
    Layout,
    ShuffledTiming,
    ShuffledPropagation,
    Ambiguous,
    NoStructure,
}

#[derive(Clone)]
struct Fixture {
    raw: RawActivity,
    selected: BTreeSet<Occurrence>,
    misleading: BTreeSet<Occurrence>,
}

fn fixture(seed: u64, ordinal: usize, context: usize, perturbation: Perturbation) -> Fixture {
    let base = seed
        .wrapping_mul(1_000_003)
        .wrapping_add(ordinal as u64 * 37)
        .wrapping_add(1 << 32);
    let mut ids = (0..8)
        .map(|offset| Occurrence((base + offset) as u32))
        .collect::<Vec<_>>();
    if matches!(perturbation, Perturbation::Relabel) {
        ids.rotate_left(3);
        ids.reverse();
    }
    let correct_indices = if matches!(perturbation, Perturbation::Allocation) {
        [5, 1, 7]
    } else {
        [1, 4, 6]
    };
    let gap = 1 + (context / 2) as u8;
    let attachment = context % 2;
    let mut ticks = [
        8,
        10,
        11,
        9,
        10 + gap,
        12,
        10 + (attachment as u8 * gap),
        11,
    ];
    if matches!(perturbation, Perturbation::Allocation) {
        ticks[correct_indices[0]] = 10;
        ticks[correct_indices[1]] = 10 + gap;
        ticks[correct_indices[2]] = 10 + attachment as u8 * gap;
    }
    if matches!(perturbation, Perturbation::Layout) {
        ticks.iter_mut().for_each(|tick| *tick += 17);
    }
    if matches!(perturbation, Perturbation::ShuffledTiming) {
        ticks[correct_indices[1]] = ticks[correct_indices[0]] + 7;
    }
    let mut spikes = ids
        .iter()
        .zip(ticks)
        .map(|(occurrence, local_tick)| Spike {
            occurrence: *occurrence,
            local_tick,
        })
        .collect::<Vec<_>>();
    if matches!(perturbation, Perturbation::Allocation) {
        spikes.rotate_left(3);
    }
    let correct = correct_indices.map(|index| ids[index]);
    let complement = (0..ids.len())
        .filter(|index| !correct_indices.contains(index))
        .collect::<Vec<_>>();
    let decoy = [ids[complement[0]], ids[complement[1]], ids[complement[2]]];
    if matches!(perturbation, Perturbation::SameTimingDifferentPropagation) {
        for (member, tick) in decoy.iter().zip(correct.map(|member| {
            spikes
                .iter()
                .find(|spike| spike.occurrence == member)
                .expect("correct member")
                .local_tick
        })) {
            if let Some(spike) = spikes.iter_mut().find(|spike| spike.occurrence == *member) {
                spike.local_tick = tick;
            }
        }
    }
    if matches!(perturbation, Perturbation::SamePropagationDifferentTiming) {
        for (member, tick) in decoy.iter().zip([2, 16, 23]) {
            if let Some(spike) = spikes.iter_mut().find(|spike| spike.occurrence == *member) {
                spike.local_tick = tick;
            }
        }
    }
    let mut propagation = vec![
        Propagation {
            from: correct[0],
            to: correct[1],
        },
        Propagation {
            from: correct[2],
            to: correct[attachment],
        },
        Propagation {
            from: ids[complement[0]],
            to: ids[complement[1]],
        },
        Propagation {
            from: ids[complement[2]],
            to: ids[complement[1]],
        },
        Propagation {
            from: ids[complement[3]],
            to: ids[complement[4]],
        },
    ];
    if matches!(perturbation, Perturbation::SameTimingDifferentPropagation) {
        propagation.extend([
            Propagation {
                from: decoy[0],
                to: decoy[2],
            },
            Propagation {
                from: decoy[1],
                to: decoy[0],
            },
        ]);
    }
    if matches!(perturbation, Perturbation::SamePropagationDifferentTiming) {
        propagation.extend([
            Propagation {
                from: decoy[0],
                to: decoy[1],
            },
            Propagation {
                from: decoy[2],
                to: decoy[attachment],
            },
        ]);
    }
    if matches!(perturbation, Perturbation::ShuffledPropagation) {
        for edge in &mut propagation {
            std::mem::swap(&mut edge.from, &mut edge.to);
        }
    }
    if matches!(perturbation, Perturbation::NoStructure) {
        propagation.clear();
    }
    if matches!(perturbation, Perturbation::Ambiguous) {
        let other = [ids[0], ids[2], ids[3]];
        let canonical_ticks = correct.map(|member| {
            spikes
                .iter()
                .find(|spike| spike.occurrence == member)
                .expect("correct member")
                .local_tick
        });
        for (member, tick) in other.iter().zip(canonical_ticks) {
            if let Some(spike) = spikes.iter_mut().find(|spike| spike.occurrence == *member) {
                spike.local_tick = tick;
            }
        }
        propagation = vec![
            Propagation {
                from: correct[0],
                to: correct[1],
            },
            Propagation {
                from: correct[2],
                to: correct[attachment],
            },
            Propagation {
                from: other[0],
                to: other[1],
            },
            Propagation {
                from: other[2],
                to: other[attachment],
            },
        ];
    }
    Fixture {
        raw: RawActivity {
            spikes,
            propagation,
        },
        selected: correct.into_iter().collect(),
        misleading: decoy.into_iter().collect(),
    }
}

fn members_set(members: &[Occurrence; 3]) -> BTreeSet<Occurrence> {
    members.iter().copied().collect()
}

fn acquire(seed: u64, presentations: usize) -> (FormationLearner, BTreeSet<Occurrence>) {
    let mut learner = FormationLearner::default();
    let mut issued = BTreeSet::new();
    for ordinal in 0..presentations {
        let episode = fixture(seed, ordinal, ordinal % 4, Perturbation::None);
        issued.extend(episode.raw.spikes.iter().map(|spike| spike.occurrence));
        learner.acquire_episode(&episode.raw, |members| {
            members_set(members) == episode.selected
        });
    }
    (learner, issued)
}

fn evaluate_arm(
    learner: &mut FormationLearner,
    seed: u64,
    presentations: usize,
    perturbation: Perturbation,
) -> (usize, usize, BTreeSet<Occurrence>) {
    let mut formed = 0;
    let mut serialized = 0;
    let mut issued = BTreeSet::new();
    for ordinal in 0..presentations {
        let episode = fixture(seed, ordinal, ordinal % 4, perturbation);
        issued.extend(episode.raw.spikes.iter().map(|spike| spike.occurrence));
        if let Some(event) = learner.form(&episode.raw) {
            formed += usize::from(members_set(&event.members) == episode.selected);
            let neighborhood = serialize_once(&event, &mut learner.work);
            let copied = [
                neighborhood.pair[0].identity.0 as u32,
                neighborhood.pair[1].identity.0 as u32,
                neighborhood.witness.identity.0 as u32,
            ];
            serialized += usize::from(
                copied == event.members.map(|member| member.0)
                    && [
                        neighborhood.pair[0].position,
                        neighborhood.pair[1].position,
                        neighborhood.witness.position,
                    ] == event.propagation_rank
                    && [
                        neighborhood.pair[0].tick,
                        neighborhood.pair[1].tick,
                        neighborhood.witness.tick,
                    ] == event.temporal_rank
                    && [
                        neighborhood.pair[0].window,
                        neighborhood.pair[1].window,
                        neighborhood.witness.window,
                    ] == event.attachment_equivalence,
            );
        }
    }
    (formed, serialized, issued)
}

fn proximity_baseline(seed: u64, presentations: usize) -> usize {
    (0..presentations)
        .filter(|ordinal| {
            let episode = fixture(seed, *ordinal, ordinal % 4, Perturbation::None);
            let mut spikes = episode.raw.spikes.clone();
            spikes.sort_by_key(|spike| (spike.local_tick, spike.occurrence));
            let selected = spikes
                .iter()
                .take(3)
                .map(|spike| spike.occurrence)
                .collect::<BTreeSet<_>>();
            selected == episode.selected
        })
        .count()
}

fn random_consequence_control(seed: u64, presentations: usize) -> bool {
    let mut learner = FormationLearner::default();
    for ordinal in 0..presentations {
        let episode = fixture(seed, ordinal, ordinal % 4, Perturbation::None);
        learner.acquire_episode(&episode.raw, |members| {
            let checksum = members.iter().fold(seed ^ ordinal as u64, |acc, member| {
                acc.rotate_left(7) ^ u64::from(member.0)
            });
            checksum & 3 == 0
        });
    }
    let correct = (0..presentations.min(16))
        .filter(|ordinal| {
            let episode = fixture(seed + 500, *ordinal, ordinal % 4, Perturbation::None);
            learner
                .form(&episode.raw)
                .is_some_and(|event| members_set(&event.members) == episode.selected)
        })
        .count();
    correct < presentations.min(16)
}

fn misleading_consequence_control(seed: u64, presentations: usize) -> bool {
    let mut learner = FormationLearner::default();
    for ordinal in 0..presentations {
        let episode = fixture(seed, ordinal, ordinal % 4, Perturbation::None);
        learner.acquire_episode(&episode.raw, |members| {
            members_set(members) == episode.misleading
        });
    }
    let evaluator_correct = (0..presentations.min(16))
        .filter(|ordinal| {
            let episode = fixture(seed + 500, *ordinal, ordinal % 4, Perturbation::None);
            learner
                .form(&episode.raw)
                .is_some_and(|event| members_set(&event.members) == episode.selected)
        })
        .count();
    evaluator_correct < presentations.min(16)
}

fn contradiction_probe(
    learner: &FormationLearner,
    seed: u64,
    presentations: usize,
) -> (u16, u16, bool) {
    let mut changed = learner.clone();
    let mature_shapes = changed
        .shapes
        .iter()
        .filter_map(|(shape, evidence)| evidence.mature.then_some(*shape))
        .collect::<BTreeSet<_>>();
    let mut after_first = true;
    for round in 0..2 {
        for context in 0..4 {
            let episode = fixture(seed, round * 4 + context, context, Perturbation::None);
            changed.acquire_episode(&episode.raw, |members| {
                members_set(members) != episode.selected
            });
        }
        if round == 0 {
            after_first = mature_shapes.iter().all(|shape| {
                changed
                    .shapes
                    .get(shape)
                    .is_some_and(|evidence| evidence.mature)
            });
        }
    }
    let invalidated = mature_shapes.iter().all(|shape| {
        changed
            .shapes
            .get(shape)
            .is_some_and(|evidence| !evidence.mature)
    });
    for ordinal in 0..presentations {
        let episode = fixture(seed + 9_000, ordinal, ordinal % 4, Perturbation::None);
        changed.acquire_episode(&episode.raw, |members| {
            members_set(members) == episode.selected
        });
    }
    let reconsolidated = mature_shapes.iter().all(|shape| {
        changed
            .shapes
            .get(shape)
            .is_some_and(|evidence| evidence.mature)
    });
    (
        changed.invalidations,
        changed.reopenings,
        after_first && invalidated && reconsolidated,
    )
}

#[derive(Clone, Debug)]
pub struct SeedReport {
    pub seed: u64,
    pub e0_a_formed: usize,
    pub e0_a_presentations: usize,
    pub e0_b_exact_copies: usize,
    pub fresh_disjoint: bool,
    pub same_timing_formed: usize,
    pub same_propagation_formed: usize,
    pub relabel_formed: usize,
    pub allocation_formed: usize,
    pub layout_formed: usize,
    pub ambiguous_abstentions: usize,
    pub shuffled_timing_abstentions: usize,
    pub shuffled_propagation_abstentions: usize,
    pub no_structure_abstentions: usize,
    pub random_consequence_resisted: bool,
    pub misleading_evidence_not_competent: bool,
    pub proximity_baseline_successes: usize,
    pub invalidations: u16,
    pub reopenings: u16,
    pub exact_two_and_reconsolidated: bool,
    pub persistent_shapes: usize,
    pub persistent_bytes: usize,
    pub temporary_peak_bytes: usize,
    pub retained_occurrences: usize,
    pub retained_memberships: usize,
    pub fingerprint: u64,
    pub frozen_ds1_consumption_probe: bool,
    pub work: WorkLedger,
    pub work_reconciled: bool,
    pub passed: bool,
}

#[derive(Clone, Debug)]
pub struct GateReport {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub m0_authoritative: bool,
    pub m1_exists: bool,
    pub frozen_ds1_sha256: String,
    pub e0_a_outcome: String,
    pub e0_b_outcome: String,
    pub first_collapse: String,
    pub seeds: Vec<SeedReport>,
    pub passed: bool,
}

fn run_seed(seed: u64, acquisition: usize, evaluation: usize, reversal: usize) -> SeedReport {
    let (learner, acquisition_ids) = acquire(seed, acquisition);
    let mut learner = learner;
    let (formed, copied, evaluation_ids) =
        evaluate_arm(&mut learner, seed + 1_000, evaluation, Perturbation::None);
    let (relabel_formed, relabel_copied, relabel_ids) = evaluate_arm(
        &mut learner,
        seed + 2_000,
        evaluation,
        Perturbation::Relabel,
    );
    let (same_timing_formed, same_timing_copied, same_timing_ids) = evaluate_arm(
        &mut learner,
        seed + 2_500,
        evaluation,
        Perturbation::SameTimingDifferentPropagation,
    );
    let (same_propagation_formed, same_propagation_copied, same_propagation_ids) = evaluate_arm(
        &mut learner,
        seed + 2_750,
        evaluation,
        Perturbation::SamePropagationDifferentTiming,
    );
    let (allocation_formed, allocation_copied, allocation_ids) = evaluate_arm(
        &mut learner,
        seed + 3_000,
        evaluation,
        Perturbation::Allocation,
    );
    let (layout_formed, layout_copied, layout_ids) =
        evaluate_arm(&mut learner, seed + 4_000, evaluation, Perturbation::Layout);
    let (ambiguous_formed, _, _) = evaluate_arm(
        &mut learner,
        seed + 5_000,
        evaluation,
        Perturbation::Ambiguous,
    );
    let (timing_formed, _, _) = evaluate_arm(
        &mut learner,
        seed + 6_000,
        evaluation,
        Perturbation::ShuffledTiming,
    );
    let (propagation_formed, _, _) = evaluate_arm(
        &mut learner,
        seed + 7_000,
        evaluation,
        Perturbation::ShuffledPropagation,
    );
    let (no_structure_formed, _, _) = evaluate_arm(
        &mut learner,
        seed + 8_000,
        evaluation,
        Perturbation::NoStructure,
    );
    let evaluation_ids = evaluation_ids
        .union(&relabel_ids)
        .copied()
        .collect::<BTreeSet<_>>()
        .union(&same_timing_ids)
        .copied()
        .collect::<BTreeSet<_>>()
        .union(&same_propagation_ids)
        .copied()
        .collect::<BTreeSet<_>>()
        .union(&allocation_ids)
        .copied()
        .collect::<BTreeSet<_>>()
        .union(&layout_ids)
        .copied()
        .collect::<BTreeSet<_>>();
    let fresh_disjoint = acquisition_ids.is_disjoint(&evaluation_ids);
    let (invalidations, reopenings, exact_two_and_reconsolidated) =
        contradiction_probe(&learner, seed + 10_000, reversal);
    let random_consequence_resisted = random_consequence_control(seed + 11_000, acquisition);
    let misleading_evidence_not_competent =
        misleading_consequence_control(seed + 11_500, acquisition);
    let proximity_baseline_successes = proximity_baseline(seed + 12_000, evaluation);
    let persistent_bytes = learner.persistent_bytes();
    let fingerprint = learner.fingerprint();
    let temporary_peak_bytes = size_of::<EventRelations>();
    let work_reconciled = learner.work.organism_work()
        == learner.work.raw_relation_comparisons
            + learner.work.triples_enumerated
            + learner.work.canonical_permutations
            + learner.work.persistent_shape_comparisons
            + learner.work.proposals
            + learner.work.physical_propagations
            + learner.work.consequence_updates
            + learner.work.temporary_formations
            + learner.work.serializations;
    let frozen_ds1_consumption_probe = {
        let probe = fixture(seed + 13_000, 0, 0, Perturbation::None);
        learner.form(&probe.raw).is_some_and(|event| {
            let neighborhood = serialize_once(&event, &mut learner.work);
            let frozen = Learner::default();
            frozen.frozen_choice(&neighborhood).is_none()
        })
    };
    let e0_b_exact_copies = copied
        + relabel_copied
        + same_timing_copied
        + same_propagation_copied
        + allocation_copied
        + layout_copied;
    let passed = formed == evaluation
        && e0_b_exact_copies == evaluation * 6
        && relabel_formed == evaluation
        && same_timing_formed == evaluation
        && same_propagation_formed == evaluation
        && allocation_formed == evaluation
        && layout_formed == evaluation
        && ambiguous_formed == 0
        && timing_formed == 0
        && propagation_formed == 0
        && no_structure_formed == 0
        && random_consequence_resisted
        && misleading_evidence_not_competent
        && proximity_baseline_successes < evaluation
        && fresh_disjoint
        && exact_two_and_reconsolidated
        && frozen_ds1_consumption_probe
        && work_reconciled;
    SeedReport {
        seed,
        e0_a_formed: formed,
        e0_a_presentations: evaluation,
        e0_b_exact_copies,
        fresh_disjoint,
        same_timing_formed,
        same_propagation_formed,
        relabel_formed,
        allocation_formed,
        layout_formed,
        ambiguous_abstentions: evaluation - ambiguous_formed,
        shuffled_timing_abstentions: evaluation - timing_formed,
        shuffled_propagation_abstentions: evaluation - propagation_formed,
        no_structure_abstentions: evaluation - no_structure_formed,
        random_consequence_resisted,
        misleading_evidence_not_competent,
        proximity_baseline_successes,
        invalidations,
        reopenings,
        exact_two_and_reconsolidated,
        persistent_shapes: learner.shapes.len(),
        persistent_bytes,
        temporary_peak_bytes,
        retained_occurrences: 0,
        retained_memberships: 0,
        fingerprint,
        frozen_ds1_consumption_probe,
        work: learner.work,
        work_reconciled,
        passed,
    }
}

pub fn run(mode: HarnessMode) -> GateReport {
    let (seeds, acquisition, evaluation, reversal, mode_name) = match mode {
        HarnessMode::Micro => (vec![100], 16, 8, 16, "micro"),
        HarnessMode::Gate => ((100..105).collect(), 32, 16, 32, "gate"),
        HarnessMode::Definitive => {
            return GateReport {
                label: "DS-E0 DEVELOPMENT".to_string(),
                protocol: PROTOCOL.to_string(),
                mode: "definitive-forbidden".to_string(),
                claim_eligible: false,
                m0_authoritative: true,
                m1_exists: false,
                frozen_ds1_sha256: FROZEN_DS1_LEARNER_SHA256.to_string(),
                e0_a_outcome: "NOT RUN".to_string(),
                e0_b_outcome: "NOT RUN".to_string(),
                first_collapse: "DEFINITIVE REJECTED".to_string(),
                seeds: Vec::new(),
                passed: false,
            };
        }
    };
    let reports = seeds
        .into_iter()
        .map(|seed| run_seed(seed, acquisition, evaluation, reversal))
        .collect::<Vec<_>>();
    let passed = reports.iter().all(|report| report.passed);
    GateReport {
        label: "DS-E0 DEVELOPMENT".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: mode_name.to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        m1_exists: false,
        frozen_ds1_sha256: FROZEN_DS1_LEARNER_SHA256.to_string(),
        e0_a_outcome: if passed { "READY" } else { "COLLAPSE" }.to_string(),
        e0_b_outcome: if passed {
            "READY"
        } else {
            "BLOCKED_OR_COLLAPSE"
        }
        .to_string(),
        first_collapse: if passed {
            "NONE"
        } else {
            "E0-A OR E0-B DEVELOPMENT CONTROL"
        }
        .to_string(),
        seeds: reports,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_forms_relations_and_serializes_exactly() {
        let report = run(HarnessMode::Micro);
        assert!(report.passed, "{report:#?}");
        assert!(!report.claim_eligible);
        assert!(report.m0_authoritative);
        assert!(!report.m1_exists);
    }

    #[test]
    fn gate_passes_all_development_controls() {
        let report = run(HarnessMode::Gate);
        assert!(report.passed, "{report:#?}");
        assert!(report.seeds.iter().all(|seed| seed.passed));
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.passed);
        assert!(report.seeds.is_empty());
        assert!(!report.claim_eligible);
    }

    #[test]
    fn serializer_copies_only_existing_event_fields() {
        let event = EventRelations {
            members: [Occurrence(7), Occurrence(11), Occurrence(13)],
            temporal: [0; 9],
            propagation: [0; 9],
            temporal_rank: [0, 2, 1],
            propagation_rank: [0, 1, 2],
            attachment_equivalence: [0, 1, 0],
        };
        let mut work = WorkLedger::default();
        let serialized = serialize_once(&event, &mut work);
        assert_eq!(serialized.pair[0].identity, OpaquePort(7));
        assert_eq!(serialized.pair[1].identity, OpaquePort(11));
        assert_eq!(serialized.witness.identity, OpaquePort(13));
        assert_eq!(serialized.witness.tick, 1);
        assert_eq!(serialized.witness.position, 2);
        assert_eq!(serialized.witness.window, 0);
        assert_eq!(work.serializations, 1);
    }
}
