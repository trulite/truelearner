# PX3 physical event-boundary no-new-mechanism PROBE v2 implementation audit

Status: **IMPLEMENTATION READY TO FREEZE; PROBE EVIDENCE UNSPENT; PX3 ABSENT**.

## Frozen implementation candidate

- source:
  `crates/px0-physical-correspondence/examples/px3_physical_event_boundaries_probe.rs`;
- source SHA-256:
  `a15f2b1b5070d3fc707b68d0a4f7135834efbd9fc919e6a3c27d60f7751afad9`;
- organism-visible block SHA-256:
  `ac11bd435098469cdf2a16b3d75dddf4285396c3a75aa31a87bff1f775142fee`;
- v2 protocol SHA-256:
  `cae4d2b03b0c094a48348fc34ba49fa16c2ecf47847850e01d66c936efd83a52`;
- exact frozen parent:
  `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`.

The v1 protocol remains frozen and unexecuted. The v2 cadence amendment is
frozen at commit `fe2c4b2c8d98bf3afcafd8e4e13a5be719ac3413`, tag
`px3-physical-event-boundaries-no-new-mechanism-probe-v2-protocol`.

## No-addition physical construction

The organism block repeats the exact kinds of physical structure already in
the authoritative PX2 world:

- four anonymous copies of arrival CELL, nearby correspondence-end CELL,
  continuation CELL, consequence CELL, participation-trace CELL, outward CELL,
  acquisition driver, participation driver, and return gate;
- the retained shared anonymous return hub and context CELL;
- the same acquisition, traversal, consequence, trace, return, outward, and
  weak direction-candidate ARROW pattern;
- the same thresholds, positive coupling, delays, resistance, local
  eligibility window, returned-activity strengthening, ordinary pressure,
  decay, refractory behavior, generation invalidation, SPIKE ordering, and
  natural queue drain.

Four copies instead of two vary only population size. No relation-specific
CELL, pair ARROW, closure token, new trace, negative coupling, evaluator-chosen
local update, reset, cutoff, or new substrate rule exists. All acquisition and
held-out inputs are queued as physical arrivals before one ordinary
propagation; `propagate` is never called to declare an internal cut.

## Organism/evaluator isolation

The organism-visible block is delimited by explicit markers. Its frozen bytes
are audited against the block hash above. A preflight source scan rejects the
forbidden target vocabulary and old-M3/PX3/evaluator dependencies inside that
block. Scenario partitions, trained/crossed comparisons, expected results,
classification, and artifact serialization exist only after the block and
cannot select an internal path or local update.

The old typed-M3 file is not compiled, included, parsed, serialized, or called.
Preflight verifies its frozen hash only, as a provenance check for the
behavioral reference. No old schema enters organism execution.

## Fixed physical opportunities

- PX0 correspondence: four uses per route at ticks `0,16,32,48`;
- weak PX2 direction candidates: resistance `3`, created only after
  correspondence acquisition;
- first recurrence arrivals: tick `66`;
- twelve rounds, round spacing `18`, paired simultaneous arrivals, second
  cluster at `+8`, alternating cluster order;
- one external source-threshold arrival set and one participation-driver
  arrival for every listed active route;
- held-out context: one ordinary shared physical pulse per simultaneous use;
- gapped control: two context pulses separated by `6` ticks;
- held-out/post-gap observations on clones, with no plastic effect on the
  acquired cell;
- blocked-return arm: only the already-preregistered omission of the
  consequence-to-hub ARROW;
- subthreshold arm: one recurrence round followed by the fixed pressure gap;
- reverse-allocation replicas and fresh namespaces exactly as preregistered.

There is no unstated environmental arrival or hidden schedule.

## Work and atomicity

Acquisition, training, initial held-out advance, propagation, crossed, gapped,
singleton, and post-gap work are all accumulated. The CSV serializes every
`WorkLedger` field plus its checked total, arrow count, persistent bytes,
complete/permanent fingerprints, and duplicate equality.

Final and staging artifacts are absent. Publication uses `create_new`, file
sync, and same-directory rename. Both PASS and scientific FAIL publish; an
existing final or staging path refuses execution.

## Pre-evidence validation

- formatting: pass;
- focused compile: pass;
- strict focused Clippy: pass;
- no-argument refusal: exit `2` before source audit or any CELL;
- wrong-argument refusal: exit `2` before source audit or any CELL;
- no-CELL `--preflight`: pass with exactly one preflight marker;
- evidence marker during validation: absent;
- final and staging artifacts: absent;
- authoritative PX0–PX2 source/result hashes: exact;
- authoritative tag resolves to the exact frozen parent and is an ancestor;
- organism-visible forbidden-token scan: pass.

No probe cell, duplicate, control, result, or evidence marker has executed.
The sole `--probe` command remains unspent.

