# CORE0 — Blank Physical Organism protocol v1

## Status and lineage

This protocol is frozen before any CORE0 runtime or evaluator change. Its
parent is exact CE1-v2 negative handoff
`f74054c2fb24d1162d2f42f273a4ecccd7a58d56`. CORE0 is development evidence;
it cannot advance organism authority or modify `arch.md`.

## Question

If every identified learning-specific decision is removed in progressively
stripped bodies, how far does one small material substrate develop before a
capability first fails?

CORE0 is a destructive ablation, not a repair ladder. All four bodies and all
capability worlds are frozen before execution. A failure is recorded and is
never repaired inside CORE0.

## Common material substrate

All bodies retain only:

- ordinary CELL junctions with physical position, signed activation,
  threshold, refractory state, liveness and generation;
- ordinary directed ARROW links with signed coupling, resistance, local
  participation, delay, phase, liveness and generation;
- ordinary Drive and Modulatory arrivals;
- WS0/SI0 local causal waves: simultaneous incidence updates a junction once,
  and caused arrivals enter a later causal wave;
- Drive activation, firing and propagation;
- continuously relaxing path-local participation;
- local resistance decay, zero-resistance link death, J0 orphan-junction
  death and generation-safe reuse;
- local Modulation/participation interaction and the smallest already-working
  PQLC local continuation law;
- natural quiescence and exact replay.

Drive and Modulatory remain distinct physical effects in CORE0. PQLC remains
the smallest accepted implementation; CORE0 does not invent a replacement.

## Continuous material representation

CORE-B through CORE-D use signed Q32 fixed-point material values. `ONE` is
`2^32`.

- external whole impulses are admitted as `impulse * ONE`;
- CELL activation and threshold are Q32 values;
- ARROW coupling is signed Q32;
- ARROW resistance is unsigned Q32;
- traversal adds exactly `ONE` local participation;
- each elapsed tick relaxes participation by the already-frozen local ratio
  `15/16`;
- qualified local Modulation changes coupling magnitude by the remaining
  participation, preserving the existing coupling sign;
- the same coincidence changes resistance by
  `3 * remaining_participation`;
- local forgetting subtracts `ONE/10` resistance per elapsed tick;
- traversal alone changes neither coupling nor resistance.

These are direct material updates. There is no support accumulator, completed
quantum, minimum reward, eligibility predicate or threshold saying that enough
evidence has accumulated. Threshold comparison remains only the ordinary CELL
firing law.

For CORE-B through CORE-D, exact comparisons use Q32 material values. Integer
compatibility accessors are observer-only and cannot drive the organism.

## Frozen bodies

### CORE-A — cumulative candidate

Exact CE1-v2 cumulative candidate:

- integer activation/coupling/resistance;
- `plastic_support / 2^32` efficacy maturation;
- rounded integer resistance gain;
- exact generated `C+`/`C-` contact motif;
- external-arrival-only proposal;
- radius two;
- hard proposal suppression when capacity is insufficient.

CORE-A is the positive control, not the desired endpoint.

### CORE-B — continuous material

CORE-A topology, timing, variation and PQLC, but the continuous Q32 material
law above replaces support thresholds and integer learning rewards.

### CORE-C — no supplied contact motif

CORE-B material law, but variation creates only weak direct signed ARROW
alternatives between existing local junctions. It creates no contact CELL and
no `P -> C+/- -> X` macro. External-arrival triggering, radius two and bounded
capacity remain as controlled scaffolds.

### CORE-D — blank material body

CORE-B material law with generic bounded direct signed perturbation:

- every local CELL firing, regardless of boundary origin, may expose weak
  direct signed alternatives to live physical neighbors;
- opportunity strength is graded by distance as `ONE/(1+distance)`; there is
  no hard radius;
- at most one live candidate exists per `(source,target,sign)`;
- no contact CELL is created;
- all CORE0 worlds preallocate enough capacity, so capacity exhaustion cannot
  select a candidate.

CORE-D contains no architect-supplied contact morphology. It may therefore
fail contact construction or attribution; that is an intended result, not a
reason to repair it.

## Capability battery

Every body receives the same physical worlds and experience order:

| Gate | Capability |
|---|---|
| E0 | propagate ordinary activity |
| E1 | form a new local relation |
| E2 | retain a consequence-supported relation |
| E3 | remove unsupported structure |
| E4 | develop positive efficacy |
| E5 | develop negative efficacy |
| E6 | select between competing physical possibilities |
| E7 | build a contact compartment |
| E8 | contact-local spatial credit specificity |
| E9 | delayed consequence through remaining participation |
| E10 | depth 1/2/4/8/16 local consequence closure |
| E11 | stabilize executable recurrence with ordinary topology |
| E12 | generate, select and reuse recurrence-stabilizing topology |
| E13 | retain four distinct context/action relations `[1,4,2,3]` |
| E14 | unchanged frozen ARC A2 world |

The evaluator may inspect physical state for scoring but may not provide a
candidate identity, useful sign, route, target coupling, target resistance,
credit path, depth, stability score or ARC-specific organism input.

## Prefix rule

Within each body, gates execute in order. The first failed gate is frozen.
Later gates for that body are `NOT_REACHED`. Other bodies continue independently.
No physics, world, predicate, timing or comparator may change after the first
CORE0 matrix begins.

## Comparators and measurements

Each reached gate runs under `MechanicalConfig::REFERENCE` and
`MechanicalConfig::PRODUCTION`. They must agree on the wave-normalized physical
history, crossings, firings, Drive and Modulatory incidence, proposals,
deallocations, coupling/resistance material changes, clock, pending activity,
durable structure, quiescence and replay.

For every gate record:

- pass/fail/not-reached;
- experiences admitted;
- PhysicalWork;
- ExecutionCost separately;
- live durable CELL/ARROW counts;
- proposals and deallocations;
- coupling/resistance material deltas;
- first divergent physical transition, if any;
- whether any new physics was required (`false` for every CORE0 row by
  construction).

Raw checkpoint hashes, allocation capacity, elapsed CPU and representation
layout are not organism equivalence criteria.

## Static prohibitions

CORE-B through CORE-D must contain no causal equivalent of:

- `eligible_until`, eligibility window or global forgetting epoch;
- `plastic_support`, completed support quantum or `2^32` learning threshold;
- minimum integer resistance reward;
- task, action, answer, episode, history, query, credit, reward, correct,
  predecessor, path ID, route ID, depth or hop count;
- handle ordering or insertion ordering as causal physics;
- evaluator-selected sign/contact/candidate;
- ARC-specific runtime law.

CORE-C and CORE-D additionally forbid generated contact motifs. CORE-D forbids
external-arrival-only proposal and a hard spatial proposal radius.

## Execution discipline

1. Freeze this protocol.
2. Implement all four bodies and the complete evaluator.
3. Run formatting/static inspection and one targeted compilation/validation.
4. Freeze the candidate bytes.
5. Execute the complete CORE0 matrix exactly once.
6. Publish the prefix map without repair or rerun.

CORE0 cannot change authority, `arch.md`, ARC curriculum, Academy timing or any
accepted predecessor artifact.
