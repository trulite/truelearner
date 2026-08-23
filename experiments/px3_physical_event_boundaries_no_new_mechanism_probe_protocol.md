# PX3 physical event-boundary no-new-mechanism PROBE protocol

Status: **PREREGISTERED; PROBE EVIDENCE UNSPENT; PX3 ABSENT**.

## Frozen start and authority boundary

This independent development lane begins exactly at authoritative PX2 commit
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`, tag
`px2-physical-causal-direction-authoritative`.

| frozen input | SHA-256 |
|---|---|
| authoritative PX0–PX2 substrate law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| authoritative PX2 execution source | `c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5` |
| authoritative PX1 definitive CSV | `6613ff0a96bb3a60fbe7afeb92cd64edced3c6df5dcc04fe47518db158dd88f6` |
| authoritative PX2 definitive CSV | `921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18` |
| PX2 authority handoff | `98647ab1563593e18e345cd7e5a71c4991d18b397dfe2dec71a4756106d96509` |
| old typed-M3 frozen source (behavioral reference only) | `a8d8fe060b497c7a6b5f9a5a88b7ed2292dc8a729a8781f599547b6027efc0a0` |
| old typed-M3 definitive CSV (behavioral reference only) | `ac8c0a6c9b7badfa263ceb054ffe59c11162b1ca256c56cc6df5f0d378179401` |

The authoritative PX0–PX2 files are read-only. This workflow cannot advance
PX3 or any later generation, create an authoritative ancestor, or execute a
definitive matrix. The old typed-M3 source and result may define behavioral
questions only; they are never linked, included, parsed, serialized, or
executed.

## First question

> Do recurring co-participation, local timing, physical gaps, returned
> participation activity, recurrence, retained closure physics, and natural
> quiescence already produce a reusable, relation-specific organization in
> the actual PX0–PX2 CELL/ARROW/SPIKE state?

The null is not “no temporal signal.” A close pair can trivially be observed
in a transient trace. The required capability is reuse after the acquisition
stream is absent: the physical state must preserve which local activities
recurred together and distinguish that learned pairing from a crossed pairing.

## No-new-mechanism physical arm

The probe repeats the frozen PX2 anonymous route motif four times and retains
its single shared anonymous return hub. It changes no substrate law and adds
no new kind of CELL, ARROW, SPIKE, trace, plasticity update, cutoff, reset, or
closure rule. Every route undergoes ordinary PX0 correspondence acquisition,
actual PX1 participation return, and PX2 direction maturation.

The organism-visible block may contain only generic physical identities,
positions, regions, thresholds, resistance, coupling, delays, CELL state,
ARROW topology, SPIKE arrivals, local eligibility/return, pressure, and
quiescence. It may not contain `Event`, `Episode`, `History`, `Boundary`,
grouping labels, partitions, pair classes, old-M3 fields, or renamed
equivalents. Scenario meaning and pass clauses remain evaluator-only and have
no causal path into the substrate.

No evaluator calls `propagate` to declare a cut. Each complete acquisition or
held-out schedule is entered before one ordinary propagation, so temporal
gaps are physical arrival-time differences inside one continuous queue.

## Preregistered histories

Use four anonymous routes `0..3`, 12 recurrence rounds, within-cluster spacing
`0`, between-cluster gap `14`, and round spacing `32`. Alternate the early and
late cluster each round so every route has matched early/late exposure.

- reference A: recurring local co-participation `01 | 23`;
- reference B: recurring local co-participation `02 | 13`.

The labels above exist only in the evaluator. Per-route traversals, returns,
recurrence count, total arrivals, total elapsed time, and ordinary pressure
opportunities are matched. Each physical history is run twice from fresh
state for exact replay.

Held-out schedules are nonplastic clones after a fixed gap:

1. the pairing used during acquisition;
2. the crossed pairing;
3. the same two activities separated by a six-tick physical gap;
4. a single activity;
5. allocation-reversed and physical-identity-shifted replicas.

## Measurements

Serialize per route correspondence resistance, live direction paths,
direction resistance, actual traversal, consequence firing, participation
trace arrival/firing, local return, outward crossing, autonomous source
refiring, and held-out/post-gap reuse. Also serialize shared-hub firings,
complete and permanent fingerprints after common-time quiescent normalization,
arrow count, persistent bytes, exact duplicate equality, and the complete
work ledger.

The evaluator must explicitly record:

- whether references A and B leave distinguishable physical state;
- whether trained and crossed pairings yield distinguishable held-out state or
  outward behavior;
- whether close, gapped, and singleton schedules differ only transiently or
  remain reusable;
- whether recurrence below two presentations remains non-reusable;
- whether return removal prevents direction maturation;
- whether every finite schedule drains naturally with zero autonomous source
  refiring.

## Conjunctive interpretation

`EMERGES_WITHOUT_NEW_MECHANISM` requires all of the following:

1. frozen sources and exact parent pass audit;
2. all four routes acquire correspondence and mature only through actual
   participation plus ordinary physical return;
3. recurring references A and B leave different organism state despite
   matched per-route marginals;
4. each trained pairing is reusable after a gap and differs physically from
   its crossed pairing;
5. the six-tick gap prevents a false local cluster and singleton activity does
   not complete one;
6. one presentation remains subthreshold while recurrent presentation is
   reusable;
7. blocked return produces no reusable direction;
8. identity/layout replicas preserve the law without sharing identities;
9. every run is exact on duplicate replay, naturally quiescent, source-silent,
   and fully work-accounted.

If clauses 1, 2, and 5–9 pass but A/B physical states are equal and trained
versus crossed held-out behavior is symmetric, freeze
`NO_RELATION_SPECIFIC_STATE_IN_PX0_PX2` as a material negative. Transient
timing discrimination alone cannot rescue the result.

Any other failure is frozen under its first failing clause without tuning or
rerun. A negative cannot be repaired by adding a relation cell, pair token,
grouping ARROW selected by an evaluator, serializer, hidden reset, or old-M3
schema. If progress would require relation-specific persistent structure,
that is a genuinely new representation and this lane stops.

## Evidence discipline and atomic artifacts

The implementation must refuse without exactly `--preflight` or `--probe`.
Preflight performs source/hash/absence checks without entering a CELL and
without emitting an evidence marker. After implementation is committed and
tagged, exactly one command may spend evidence:

```text
cargo run --release -p px0-physical-correspondence \
  --example px3_physical_event_boundaries_probe -- --probe
```

It emits exactly one
`PX3_PHYSICAL_EVENT_BOUNDARIES_NO_NEW_MECHANISM_PROBE_EVIDENCE_SPENT`
marker and atomically publishes, whether positive or negative:

```text
results/px3_physical_event_boundaries_no_new_mechanism_probe_v1.csv
results/px3_physical_event_boundaries_no_new_mechanism_probe_v1.md
```

Staging files use the same basename prefixed by `.` and suffixed `.staging`.
Final or staging path pre-existence is a hard refusal. There is no rescue,
regeneration, parameter change, or rerun after the marker.
