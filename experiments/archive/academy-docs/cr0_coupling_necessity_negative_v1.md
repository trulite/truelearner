# CR0 coupling-necessity immutable negative v1

Status: stopped measurement negative. CR0 scientific classification is not
established by v1.

Frozen candidate: `cr0-coupling-necessity-frozen-v1` (`5d23d34`).

E2B evidence sandbox: `i87zchu9rtnuelzd1194x`.

## Completed evidence

The evaluator serialized its complete matrix before stopping:

- `400/400` physical cases;
- `800/800` mechanics rows;
- all `400/400` functional predicates passed under both mechanics;
- exact same-mechanics reconstruction passed on all `800/800` rows;
- all four efficacy controls produced their preregistered behavior;
- maximum PhysicalWork was `66`;
- all cases naturally quiesced.

Evidence hashes:

```text
matrix 34d47505accf166edb1c3a05ce6ee11b4807268351947b91e94bc661fefd6950
report 1292f8a2f3b54f67f0a0972dc156dc7df12bc4888d1ade08c237d2ad0d0af9b1
```

## Exact stop

Reference/Production comparison failed in `240/400` physical cases, producing
`480` serialized rows marked `mechanics_equal=false`. The affected families
were exactly the six retained-behavior families:

```text
cpc0_contact_locality          40 cases
cpc1_temporal_participation    40 cases
pqlc0_one_hop                  40 cases
pqlc1_depth_16                 40 cases
fd0_equal_persistence          40 cases
fd1_consolidation              40 cases
```

Pairwise column comparison localized every mismatch to `live_hash`, the hash
of raw `LiveCheckpoint::canonical_bytes()`. In all 240 cases, Reference and
Production still matched exactly on:

- complete ordered physical transitions and transition hash;
- candidate durable and transient observations used by the gate;
- Drive, Modulatory, Fire, Resistance, QLP, Crossing, Proposal, and
  Deallocation counts;
- PhysicalWork;
- physical clock;
- canonical durable body hash;
- natural quiescence and same-mechanics replay.

There were zero other pairwise field differences.

## Classification

This is a measurement/evaluator defect, not evidence for or against coupling
plasticity.

The frozen CR0 protocol requires canonical pending activity and future-relevant
physical state, but explicitly does not require implementation-specific
mechanics state to be equal. The evaluator accidentally included the complete
raw live-checkpoint hash inside its derived `Observation` equality. Reference
and Production may serialize different causally inert runtime timestamps while
producing the same physical history; this measurement boundary was already
recognized in prior representation-independence work.

Two packaging defects are also preserved:

- the CSV header omitted the final serialized `case_pass` column, so rows have
  26 fields while the header has 25;
- the report rendered the aggregate `all_pass` value for both replay and
  mechanics, incorrectly printing replay as false even though every
  same-mechanics replay passed.

The v1 report's prose saying “CR0 establishes” coupling's role is therefore not
an accepted claim. The panic and this audit control classification.

## Boundary

CR0 v1 is permanently negative and may not be rerun or relabeled. A fresh v2
is scientifically eligible with only these measurement repairs:

1. retain and serialize raw live-checkpoint hashes diagnostically, but exclude
   them from Reference/Production physical equality;
2. add the missing `case_pass` header;
3. report replay, mechanics, predicates, and total acceptance separately;
4. use fresh roots.

No world, physical law, initial durable state, threshold, schedule, predicate,
or decision rule may change.
