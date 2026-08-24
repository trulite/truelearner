# PX-C continuous-organism protocol v1

Status: **PREREGISTERED; IMPLEMENTATION ABSENT; DEVELOPMENT AND AUTHORITY EVIDENCE UNSPENT**.

Exact parent: PXR0 v2 accepted commit
`b76f4f1c276555a6f80ff697dbc9b9ef850df76e` / tag
`pxr0-v2-accepted-pxc-parent-v1`. Parent kernel SHA-256 is
`f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb`.
PX-C may add only the smallest continuous boundary integration and
evaluator/audit evidence. It must not edit `arch.md`, retained PX0-PX8
authority sources, or implement any future distributed, framebuffer,
foveation, arena, timing-ring, SIMD, serving-mode, or optimization surface.

## Sole production integration

The active production closure remains exactly one Rust file:
`crates/pxr0-physical-runtime/src/lib.rs`. The only permitted source change is
one public method, making the exhaustive inventory 13 types and 16
functions/methods:

```rust
pub fn arrive(&mut self, inputs: &[SpikeInput], outward_region: i16) -> RunResult
```

`arrive` must append each anonymous physical input through the existing
`enter`, execute the existing `propagate` to natural quiescence, then retain in
the returned result only already-produced ordinary crossings whose
`to_region == outward_region`. Filtering occurs after all physical transitions
and cannot affect retained state, work, queue order, pressure, plasticity, or
later arrivals. No other constant, field, function, transition, visibility,
or dependency may change.

Every tested activity batch uses `arrive`; the harness may use
`advance_time` only between empty-queue batches for ordinary physical pause,
pressure, and decay. It may not call `enter` or `propagate` directly. Generic
`add_cell`/`add_arrow` bootstrap constructs one physical body before the first
arrival. The same `PlasticSubstrate` instance then persists through every
batch. There is no reset, reconstruction, clone, cleanup, or typed layer call
inside a trial.

## One continuous physical trial

Each trial creates exactly one empty substrate, advances its empty clock to a
phase-preserving origin, constructs one retained body containing causally
isolated physical subcircuits, and calls `arrive` repeatedly over this fixed
batch sequence:

```text
blank; 0; 10/11; 20/22; 31; 40; 51; pause-to-60;
61; 70; age-to-110; 111; 120
```

All ticks are relative to the registered origin. Causally isolated subcircuits
share the same substrate, clock, pressure epoch, queue, scheduler, Work
observer, and resident allocation. They differ only in physical identity,
position, topology, resistance, and arrival pattern. Internal CELL regions are
zero and the sole observable exterior region is one; `arrive(..., 1)` therefore
returns outward crossings only.

The retained body jointly contains: paired correspondence/direction paths; a
selective-participation path; six three-stage recursive paths for complete,
incomplete, adjacent-batch, duplicate, blocked, and aged/reproposal controls;
direct, duplicate-direct, open, fork, and ring physical controls; and an
ordinary Drive-only path. Every topology is generic physical bootstrap, not a
named world constructor or organism API.

## Frozen row clauses

Each complete trial is independently replayed byte-exactly and serializes 32
conjunctive clauses:

1. blank arrival batch is outward-silent and naturally quiescent;
2. every nonblank `arrive` result is naturally quiescent;
3. every exposed crossing terminates in exterior region one;
4. construction tick, pressure origin, and first arrival equal the registered
   modulo-ten origin;
5. the retained substrate reaches the final tick without reconstruction;
6. early physical return produces local updates;
7. the paired held batch initiates exactly one outward crossing;
8. the selective-participation held batch initiates exactly one outward crossing;
9. six ordered formation batches produce retained local updates;
10. later complete recursive arrivals cross outward exactly once;
11. incomplete recursive arrivals remain outward-silent;
12. first unsupported adjacent batch remains outward-silent;
13. second unsupported adjacent batch remains outward-silent;
14. duplicate recursive arrivals cross outward exactly once;
15. blocked recursive output remains outward-silent;
16. direct physical traversal crosses outward exactly once;
17. duplicate same-tick direct arrivals cross outward exactly once;
18. open physical topology remains outward-silent;
19. fork topology remains outward-silent;
20. ring/cycle topology remains outward-silent;
21. aged stale structure neither executes nor crosses outward;
22. changed post-aging activity creates exactly one fresh local proposal;
23. ordinary aging produces at least one physical deallocation;
24. the isolated Drive-only batch causes zero local return updates and zero
    Modulatory deliveries;
25. that Drive-only batch still delivers ordinary Drive;
26. prior batches contain both Modulatory delivery and qualified local update;
27. pause-to-60 occurs with an empty queue and later complete reuse succeeds;
28. maximum per-batch work is at most `200000`;
29. maximum retained resident allocation is at most `65536` bytes;
30. complete-trial replay is exact;
31. the frozen PXR0 466/466 cumulative conformance inputs are exact;
32. clauses 1--31 are conjunctively true.

The claim maps these physical clauses to cumulative PX0-PX8 capabilities:
correspondence, participation, direction, ordered organization, lifetime,
allocation, qualified return, initiation, recursive composition, closure,
exactly-once output, forgetting, stale-generation blocking, fresh proposal,
changed experience, unsupported adjacency, incomplete/open/fork/cycle
negatives, pause/resume, ordinary Drive/Modulation separation, quiescence,
bounded work/memory, and replay. The runtime contains none of those evaluator
labels or any cognitive representation.

## Development and authority matrices

The evaluator is tooling-only at
`arms/pxc-continuous-organism/src/main.rs`, depending directly only on the
one-file runtime. Both modes and all predicates must be frozen before either
matrix runs.

Development uses roots `3_100_001..3_100_016`, balanced reverse/reflection
quadrants, and origins `0,130,260,390` once per quadrant. It writes create-new:

```text
results/pxc_continuous_development_v1.csv
results/pxc_continuous_development_v1.md
```

Authority uses disjoint roots `3_200_001..3_200_016`, the same balanced
quadrants, and phase-equivalent origins `520,650,780,910` once per quadrant. It
writes create-new:

```text
results/pxc_continuous_authority_v1.csv
results/pxc_continuous_authority_v1.md
```

Each mode has 512 row clauses plus 12 globals: exact roots; balanced layouts;
balanced origins; modulo-ten timing; disjoint namespaces; one substrate and no
direct `enter`/`propagate` in the harness; exact final runtime/spec/manifest;
one-file/29-entry/page/dependency gate; zero primary/semantic/evaluator/new
kind/new surface state; exact accepted-PXR0 and frozen-development inputs;
16 rows/32 clauses; publication and replay. Success is `524/524`.

All rows, timing, batch observations, bounds, 32 booleans, globals, and verdict
are serialized before aggregate assertions. Development and authority each
execute at most once. A failed development row freezes negative and forbids
authority. No post-observation source, mechanism, topology, schedule, bound,
or predicate repair is allowed.

## Validation and authority discipline

Before development, freeze source, evaluator, final 29-entry one-page spec,
inventory/dependency/leakage audits, and hashes. One fresh E2B targeted worker
may run formatting, strict package Clippy, static audits, taxonomy zero gates,
and page rendering; it must not run either matrix. A second fresh worker runs
development exactly once.

Only a frozen positive development result permits a separately committed
authority-execution audit binding its hashes. One fresh disjoint authority
worker then runs the static firewall and authority matrix exactly once. A
portable result audit may inspect generated files without rerunning Rust.
Authority requires 524/524, zero seams/guards/kinds/surfaces, one page/29
entries, exact replay, natural quiescence, bounds, unchanged physical laws,
and clean create-new publication. Success establishes final PX-C continuous
organism authority only; it authorizes neither `arch.md` edits nor future
runtime work.
