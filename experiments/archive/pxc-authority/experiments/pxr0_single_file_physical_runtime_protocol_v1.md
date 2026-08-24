# PXR0 single-file physical-runtime extraction protocol v1

Status: **PREREGISTERED SUCCESSOR DEVELOPMENT; IMPLEMENTATION ABSENT; NO AUTHORITY CLAIM**.

Parent authority is exact commit
`9ba11aeb4f88d6707cfba1afdb5f6dce3e380b9f` / tag
`px8-lrc-closure-authority-v3-authority-v1`. PXR0 is a pre-PX-C extraction
phase. It may establish successor development readiness only and may not
advance PX0--PX8, continuous-organism, or PXR0 authority.

## Frozen extraction boundary

The entire production organism closure is exactly one source file:
`crates/pxr0-physical-runtime/src/lib.rs`. It has no production dependency and
contains no submodule, include, generated source, feature-selected production
path, named world constructor, evaluator, or test module. Its active manifest
contains exactly that file. Tooling may depend on the runtime; the runtime may
not depend on tooling.

The candidate inventory is exactly 28 visible entries:

- types/state/results (13): `CellId`, `ArrowId`, `CellSpec`,
  `TransmissionMode`, `ArrowSpec`, `SpikeInput`, private `Cell`, private
  `Arrow`, private `Spike`, `Crossing`, `Work`, `RunResult`, and
  `PlasticSubstrate`;
- functions/methods (15): `Work::total`, `PlasticSubstrate::{new, add_cell,
  add_arrow, enter, advance_time, propagate, apply_modulatory_return,
  elapse_to, propose_local_arrows, decay_cell, require_cell, spike_order,
  resident_bytes}`, and free `pressure_arrow`.

Standard structural derives do not contain transition logic and must be listed
on the type line. No transition may be hidden in a macro or generated body.
The one-page inventory audit must reconcile these 28 entries to source with
zero omissions and zero extras.

Generic `CellSpec`/`ArrowSpec`/`SpikeInput` construction is retained as physical
bootstrap. `RunResult` may expose only outward/inter-region `Crossing` values,
the smallest registered `Work` counters, natural quiescence, and resident
bytes. `resident_bytes` is the one counted read-only audit hook required for
the memory bound; it cannot modify execution.

The following move out of production closure into non-organism research
tooling: PX4 `Field`/`Fork` and builders; PX7 `Form`/`Body`/`Activity`, layouts,
and inspection; PX8 `Layout`/`Reading`/`RecursiveBody`/`CompactBody`/
`CompactForm` and all choreography; repeated cell/Drive/Modulation/physical
builders; fingerprints, state-byte encoders, push helpers, detailed ledgers,
trace entries, arrow getter suites, memory/fingerprint inspection surfaces,
and `Reading`/merge/add-work helpers.

## No-law-change rule

The local constants and transitions are copied exactly from authoritative
`crates/lr1-modulatory-physical-return/src/lib.rs`: delivery ordering,
generation/live checks, activation/threshold/refractory behavior, Drive,
eligibility, qualified Modulation, resistance increase, ordinary pressure,
deallocation, local proposal radius/order/specification, time decay, crossing
serialization, and natural queue exhaustion. Removing diagnostics must not
change state mutation, queue order, crossings, work totals, or outcomes. Any
required new state, transition, selector, law, or changed outcome is a
scientific fork and freezes negative.

## Development gates

Before execution, freeze the candidate source, one-file manifest, exhaustive
inventory auditor, dependency-direction audit, one-page specification source
and renderer, taxonomy/guard invocation, cumulative evaluator, exact roots,
clauses, bounds, and output paths.

The rendered specification is A4 portrait, 36-point margins, at least 10-point
text and 12-point leading. It must visibly list all 28 entries and cover only:
physical state, local transition rules, continuous execution, and external
crossings. Banned cognitive vocabulary is the v2 one-page list. Rendering must
produce exactly one legible PDF page and reconcile zero omitted/extra entries.

The candidate gate must prove:

1. active manifest has one row and one existing Rust source;
2. source inventory is exactly 13 types + 15 functions/methods;
3. no production modules, includes, macros, cfg paths, tests, generated code,
   dependencies, or reverse tooling dependency;
4. banned cognitive vocabulary has zero matches;
5. primary seams, semantic guards, evaluator guards, new kinds, and new
   guarded surfaces are all zero;
6. the rendered specification is one page with exact field/rule coverage;
7. retained PX0--PX8 files and parent authority inputs remain byte-identical.

## Frozen readiness matrix

The evaluator is non-organism tooling at
`arms/pxr0-successor-readiness/src/main.rs`, depending only on PXR0. It
reconstructs generic physical worlds without named constructors in production.
There are 16 fresh roots `1_075_001..1_075_016`; reverse/reflected quadrants
and schedule shifts `0,137,274,411` each occur four times with disjoint
namespaces. Every complete trial is independently repeated and must be
byte-exact.

Each row records these 24 clauses:

1. blank substrate is silent and quiescent;
2. generic cell/arrow bootstrap preserves physical identities;
3. repeated paired traversal plus return changes qualified links;
4. only participating structure changes in the alternative control;
5. reversed construction preserves outcomes;
6. reflected positions preserve outcomes;
7. ordered multi-arrival organization becomes reusable;
8. unsupported adjacent batches remain silent;
9. three nested physical stages become reusable;
10. resistance survives supported reuse;
11. nonparticipating alternatives remain silent;
12. qualified Modulation changes eligible structure;
13. Drive without return does not update eligible structure;
14. Drive still propagates physical activation;
15. later ordinary arrivals initiate retained propagation;
16. complete nested organization crosses outward exactly once;
17. duplicate same-tick arrivals cross outward exactly once;
18. incomplete, blocked, open, branch, and cycle counterexamples stay silent;
19. aged unsupported structure neither executes nor crosses outward;
20. changed physical experience creates exactly one fresh local proposal;
21. every advance is naturally quiescent;
22. per-advance work is at most `20000` and resident memory at most `8192`;
23. independent complete-trial replay is byte-exact;
24. cumulative PX0--PX8+LR-C conjunction is true.

Eight global clauses require exact roots, balanced quadrants, balanced shifts,
disjoint namespaces, one-file/dependency direction, zero inventory/taxonomy
surface, exactly 16 rows/384 row clauses, and exact replay/publication. Success
is `392/392` clauses.

## Execution and stop discipline

After batched inspection and edits, freeze one candidate commit/tag. Run one
targeted E2B validation for that exact candidate: package formatting/Clippy,
static gates, page rendering, taxonomy, and the complete internally replayed
readiness matrix. It serializes CSV/Markdown before aggregate assertions.
Do not run workspace-wide suites or repair predicates after observing results.

If positive, freeze the successor conformance/readiness result, exact artifact
hashes, page artifact, delta/handoff, and a readiness tag. Then stop. A
definitive PXR0 authority run, PX-C protocol, PX-C implementation, or
continuous-organism evidence requires separate authorization.
