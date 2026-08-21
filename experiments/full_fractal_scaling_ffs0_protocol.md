# FFS0 preregistration: full fractal scaling

Protocol identifier: `full-fractal-scaling-ffs0-v1`

Status: frozen before FFS0 implementation, development measurement, or any
definitive FFS0 run.

## Question

FFS0 asks:

> When one semantics-blind developmental kernel may consume its own executable
> products, what parent-relative hierarchy does it produce, which consecutive
> edges are functionally correct, physically smaller, and independently
> repayable, and how does that hierarchy scale with reusable computation?

FFS0 replaces the proposed sequential F1/F2 ladder with one scaling experiment.
It does not add a special learner or executor for any reported level. Reported
level is an evaluator measurement of dependency distance from the primitive
root and is never organism-visible.

## Frozen ancestry

- RE0 positive tag: `re0-reflected-compaction-economics-positive`;
- RE0 positive commit:
  `f24fadbdd618a825f6f960df84106acc9a0bf806`;
- RE0 implementation tag:
  `re0-reflected-compaction-economics-implementation`;
- RE0 implementation commit:
  `c7455588e1e04b6e39c245b898cca1c8fc353dd0`;
- RE0 protocol SHA-256:
  `c8b0633ad5a2459b3f4d61fccbdc7f2dcf6b30990d0157e57181b1803dcfdd71`;
- RE0 definitive CSV SHA-256:
  `93c02acd71fc8dd642839fd31f84e18af385858efdf731a7fbce758c89c8d36b`;
- RE0 definitive Markdown SHA-256:
  `a93fc8304782d2af112fa0cf9147b961e98a696d395a3ae9f02cad073c60e0b5`;
- RC0b positive tag: `rc0b-grounded-motif-substitution-positive`;
- RC0b positive commit:
  `f6dcec833966347ce360f3ec126202b4843319d5`.

FFS0 may compose the already-frozen CELL/ARROW/SPIKE, anonymous provenance,
temporary binding, proposal/probation/credit, consolidation/pruning,
grounding, local compilation, counterfactual substitution, dependency
validation, and invalidation principles. It may not alter any frozen RP0a,
RG0a, RC0a, RC0b, or RE0 source or artifact.

## The generic kernel

FFS0 introduces one additive generic kernel over the frozen primitive
vocabulary:

```text
ordinary anonymous arrow occurrences
        -> local recurrent-pair proposal
        -> probation and terminal credit
        -> consolidated ordinary arrow
        -> role-relative binding
        -> ordinary arrow execution
        -> output occurrences reenter the same observer
```

Primitive and learned executable structures use the same persistent arrow
representation. An arrow may contain:

```text
integer arrow identity
anonymous source and target CELL roles
zero or two direct dependency-arrow identities
the frozen fingerprints of those direct dependencies
ordered role-relative residual effects
one local compatibility site and marker
ordinary strength/consolidation state
```

An arrow may not contain:

```text
level or parent-level ordinal
reflection or motif depth
process or task type
program or total workload depth
identity-population size
episode, seed, answer, or target identity
evaluator boundary or correctness result
work counter, runtime delta, price, horizon, or break-even
callback, opcode, direct transformation, or special recursive executor
```

The two direct dependency identities are local structural dependencies, not a
level pointer. Primitive arrows have none. A learned arrow fires through the
same queue and effect-delivery path as a primitive arrow. Its firing is itself
an ordinary anonymous occurrence eligible for the same observer.

There is no `Macro`, `MetaRole`, `Level`, `MotifLevel`, `ProcessType`, or
hierarchy-manager organism type. The harness may use scale, process, and
dependency-distance labels only in input generation and reporting.

## Learner-visible recurrence

The observer receives the executed arrow occurrence stream, direct local
adjacency, role-relative effects, compatibility signatures, and ordinary
terminal success/failure. It receives no fragment boundary.

A candidate is emitted only for an adjacent local pair which occurs at least
three times in one successful episode. Candidate discovery scans only observed
adjacencies; it does not enumerate the arrow product space. Each distinct pair
receives at most one credit per episode:

- successful episode: `+2`;
- failed episode: `-1`;
- consolidation threshold: `6`;
- prune threshold: `-2`.

Thus a mature child requires three separately successful episodes. Fixed local
pair formation is the ordinary locality rule, not a supplied `2x` macro: the
observer knows only that two consecutive occurrences were locally coactive.
When the resulting arrow later occurs next to another arrow, the identical
rule applies again.

At consolidation, the child copies only the ordered residual-effect roles
required by the two observed dependencies and their local compatibility
signature. Concrete fillers are supplied by fresh temporary episode bindings.
The learner cannot read observable-trace equality or physical work reduction.

## Execution and local fallback

An invocation temporarily binds effect roles to fresh concrete filler
identities. Before a learned arrow fires, it validates:

1. both direct dependency identities and frozen fingerprints;
2. its local compatibility site and marker;
3. recursive validity of the direct dependencies, memoized within the current
   invocation.

Validation is charged physical work. If a learned arrow is invalid, it does
not fire; its two direct dependency arrows execute instead. Either dependency
may independently expand for the same reason. Consequently fallback follows
the ordinary dependency graph:

```text
invalid substitute X -> execute X's dependencies
invalid dependency Y  -> execute Y's dependencies
```

There is no `fallback_to_level` operation. The evaluator reports dependency
distance only afterward.

When a changed compatibility marker later returns, an unchanged historical
arrow may validate and fire again. Reuse must preserve the same evaluator-side
asset instance identity and content fingerprint; otherwise it is reacquisition.

## Observable interface

For every invocation:

```text
ObservableTrace =
    final concrete state
  + ordered role-relative externally meaningful effects with fresh fillers
  + quiescence and error/activity-limit signals
  + state at safe post-effect interruption boundaries
  + final context-effect ledger
```

Raw queue contents, transparent routes, dependency-validation order, temporary
bindings, and anonymous internal arrow occurrences are implementation and may
disappear.

Every parent/child comparison branches from an identical frozen start and
requires exact observable equality. A same-endpoint stale control deliberately
deletes one required ordered effect. It must retain the final endpoint but fail
observable equality. The broken control is evaluator-only and can never become
an organism execution path.

## Parent-relative accounting invariant

Every edge earns its existence against its immediate retained parent:

```text
L(k+1) vs L(k)
```

Never `L(k+1) vs L0` when `k > 0`.

For each candidate edge `k -> k+1`:

```text
functional:
    ObservableTrace(L(k+1)) == ObservableTrace(L(k))

computational:
    RuntimeWork(L(k+1)) < RuntimeWork(L(k))

economic:
    Delta_k(H) =
        Acquisition_k
      + Installation_k
      + Carrying_k(H)
      + H * (
            RuntimeWork(L(k+1))
          + Maintenance_k
          - RuntimeWork(L(k))
        )

    exists finite H such that Delta_k(H) <= 0
```

The zero-carrying-price physical result is primary. Secondary carrying prices
are `0, 1, 10, 100, 1000` millionths of work per incremental persistent
byte-use. Exact integer millionths and exact ceiling division are authoritative.
Reuse horizons are computed analytically and are not simulated.

`Acquisition_k` includes all work after the parent snapshot freezes through
child consolidation, including the three evidence executions, occurrence
observation, comparison, credit, and persistent insertion. Consolidation is
the installation-producing event, so persistent installation is zero unless a
separate observed installation operation exists. Runtime includes temporary
binding, validation, route firing, spike handling, effect delivery, and
fallback checks. Mature maintenance is measured separately and may not be
assumed zero.

Carrying charges the child's incremental persistent bytes plus any additional
structure required to retain its parent fallback. A parent already retained
for its own ordinary use is not charged twice. Cumulative blank-start economics
is reported separately and cannot replace marginal edge tests.

## Prefix rule

The claimed hierarchy is the longest consecutive dependency prefix for which
every edge is:

1. structurally retained;
2. functionally exact;
3. computationally cheaper than its immediate parent;
4. independently finitely repayable against that parent.

If an intermediate edge fails, every deeper candidate is diagnostic only:

```text
edge 0 -> 1  PASS
edge 1 -> 2  PASS
edge 2 -> 3  FAIL
edge 3 -> 4  PASS diagnostic

claimed prefix: root -> 1 -> 2
```

No deeper saving may subsidize or rescue a failed edge.

## Structural, justified, and realized depth

FFS0 reports three evaluator-only depths per root-to-leaf execution path:

- **structural depth:** longest dependency chain endogenously consolidated and
  retained by ordinary local physics;
- **justified depth:** longest consecutive candidate chain passing functional,
  computational, and marginal economic tests, ignoring structural rejection;
- **realized useful depth:** longest consecutive chain satisfying both.

The six-promotion execution ceiling is a harness safety limit. Reaching it is
reported as right-censored `>=6`, never as learned stopping. The organism does
not receive the counter.

## Asset identity and cost ownership

The evaluator records two identities which never enter organism state:

- `content_fingerprint`: hash of actual persistent arrow content;
- `asset_instance_id`: immutable acquisition-lineage identity minted when the
  persistent arrow instance is created.

The same asset reused across scale or context is charged once only when both
the asset instance identity and unchanged content fingerprint match. Two
independently learned lookalikes with identical content fingerprints receive
separate acquisition charges. Reconstructed or copied state is not shared
ownership unless it preserves the original persistent asset instance.

## Retention agreement

For every mature functionally evaluated candidate:

| Structural outcome | Economically justified | Classification |
|---|---|---|
| retained | yes | correct retention |
| retained | no | over-retention |
| rejected | yes | under-retention |
| rejected | no | correct rejection |

Candidates that never reach the frozen three-episode confidence threshold are
reported as subthreshold and are not classified as mature economic assets.
Functionally invalid candidates are classified separately rather than hidden
inside economic rejection.

Report:

```text
over-retained count
under-retained count
correct-retention count
correct-rejection count
retention precision
retention recall
agreement rate
```

The strong statement that the organism selects its useful abstraction depth is
permitted only if endogenous structural and realized useful depths agree and
there is neither over-retention nor under-retention in the primary matrix.

## Sparse scale matrix

Primary anchors:

| Scale | Primitive program depth | Fresh identity population |
|---|---:|---:|
| S0 | 8 | 16 |
| S1 | 32 | 64 |
| S2 | 128 | 256 |
| S3 | 512 | 1024 |

Orthogonal probes:

- depth-only: depth `128`, population `64`;
- population-only: depth `32`, population `1024`.

Each primitive program is a recurrence of four anonymous local arrow roles.
Only one of the four carries an externally meaningful role-relative effect;
the other routes are locally transparent. Program depth is always divisible by
four. Fresh concrete fillers and physical CELL identities are regenerated for
every episode.

Anchor cells learn independently and therefore receive separate acquisition
lineages even if their content fingerprints match. Two transfer probes also
reuse the exact S1 asset instance at the orthogonal depth and population
settings; those probes charge acquisition once and test genuine cross-scale
reuse.

Definitive seeds are `0..7`. Development uses only seed `40_000` with separate
identity and acquisition domains. Each acquisition generation receives three
successful episodes. Held-out evaluation uses four fresh episodes per
seed/scale/context in GATE and sixteen in DEFINITIVE.

The matrix is sparse by design. Reuse horizons remain analytical.

## Process closure and availability

The evaluator reports four process rows:

```text
execution   positive | negative
learning    positive | negative | unavailable
retrieval   positive | negative | unavailable
decision    positive | negative | unavailable
```

A process is **available** only if its real operation already executes through
the identical anonymous occurrence interface and a learned substitute can
replace that operation while preserving its preregistered state/effect
interface. Merely logging an opaque Rust operation and replaying its trace is
not availability.

There may be no `LearningEvent`, `RetrievalEvent`, `DecisionEvent`, process tag,
or process-specific trace adapter in organism-visible code. If the current
substrate cannot expose and replace a process without one, the row is
`unavailable`, not negative. `Negative` means the generic kernel actually
received the valid anonymous interface and failed a frozen closure criterion.

Unavailable rows do not make the execution-recursion claims fail. Cross-process
closure is an independent vector result and may be partial.

## Adaptive matrix

The S2 execution asset receives three evaluator-controlled environments, with
the environment visible only as ordinary local compatibility markers:

1. stable context;
2. changed marker at the active child's own compatibility site, expecting
   dependency fallback distance one;
3. changed marker at one direct parent's site, expecting recursive dependency
   fallback distance two;
4. return to the original context, expecting historical asset reuse without
   reacquisition.

All arms require exact observable traces and fresh fillers. The changed arms
must invalidate the minimal incompatible dependency suffix, preserve the
nearest valid substrate route, and avoid destroying unrelated arrows. Return
must restore the same asset instance identity and content fingerprint.

Report fallback distance, number of invalidated arrow firings avoided, recovery
work, and reacquisition work.

## Controls

FFS0 freezes these controls:

- fresh identities and changed binding permutations preserve behavior;
- no bindings cannot deliver the correct concrete effects;
- adjacency evidence shuffled independently in each episode cannot consolidate
  a firing child;
- two successful evidence episodes remain subthreshold;
- failed evidence applies ordinary negative credit and may prune;
- a same-endpoint substitute missing one required effect fails observable
  equality;
- a changed local marker invalidates before child firing and exposes direct
  dependencies;
- returned context reuses historical structure without reacquisition;
- duplicate acquisition and evaluation are bitwise deterministic;
- every temporary workspace and binding is erased;
- no result path is overwritten.

## Independent claims

FFS0 does not have one conjunctive verdict across conceptually different
questions.

### A — Functional recursion

Positive iff every definitive seed at S1, S2, and S3 produces at least two
additional consecutive generations with exact parent/child observable traces
under fresh held-out identities, using the same kernel implementation.

### B — Computational recursion

Positive iff every edge in every claimed realized useful prefix has strictly
lower physical runtime than its immediate parent, every removed route is
reconciled to ordinary CELL/ARROW/SPIKE work, and S1, S2, and S3 each contain at
least two such consecutive edges for every seed.

### C — Economic recursion

Positive iff every structurally retained mature child in the primary anchor
matrix is parent-relative economically justified at zero carrying price, no
mature economically justified child is structurally rejected, and every seed
at S1, S2, and S3 has at least two consecutive finitely repayable edges. Any
over-retention or under-retention makes C negative while leaving A and B
independent.

The strong phrase `the organism selects its own useful abstraction depth`
requires C positive and exact structural/realized-depth agreement.

### D — Cross-process closure

Reported independently for execution, learning, retrieval, and decision.
Unavailable and negative are distinct. No minimum number of available process
classes is required for A, B, C, or E.

### E — Adaptive recursion

Positive iff every S2 changed-context episode preserves exact observable
behavior, child-own change falls back exactly one dependency edge, parent change
falls back exactly two, no compatible lower structure is unnecessarily
invalidated, and context return reuses the historical asset with zero
reacquisition work.

## Scaling-law report

For every process, seed, scale, and context report:

```text
proposed candidates
functionally valid
computationally useful
economically justified
endogenously retained
over-retained
under-retained
structural depth
justified depth
realized useful depth
right-censored
fallback distance
recovery work
```

Also report retention precision, recall, and agreement. A scale-depth trend is
supported only if the median realized useful depth is nondecreasing across
S0..S3 and at least two distinct uncensored depths occur. Because the anchors
change both program depth and population, the depth-only and population-only
probes are required before attributing that trend to either axis.

No exact number of promotions is preregistered as the desired result.

## Harness speeds

### MICRO

- synthetic anonymous graphs only;
- one seed, depths `8` and `32`;
- at most three promotions;
- arithmetic, exact trace, prefix truncation, retention matrix, and one-step
  invalidation only;
- no claim and no result artifact.

### GATE

- development seed `40_000` only;
- complete sparse scale, transfer, adaptive, and control matrix;
- all six possible promotions may be attempted;
- process availability source audit;
- no claim and no result artifact.

### DEFINITIVE

- frozen seeds `0..7`;
- complete preregistered matrix;
- each independent cell runs single-threaded; cells may run in parallel;
- acquisition and evaluation duplicated for determinism;
- serialize once after all cells complete;
- execute the definitive FFS0 command exactly once.

Development may use a frozen development fixture. Definitive acquisition starts
from the frozen primitive FFS0 root for each independent asset lineage. No
development seed or state enters definitive evidence.

## Source audit

Before definitive execution, inspect organism-visible persistent and execution
code for names or behavior equivalent to:

```text
level
parent_level
meta
reflection_depth
process_type
task_depth
total_depth
motif_level
answer
expected_boundary
break_even
economic
price
horizon
```

Such terms may appear in the harness/report/evaluator only. There may be no
conditional execution on evaluator generation number and no depth-specific
learned structure or path. Direct dependency identities, compatibility
signatures, evaluator-only asset identities, and harness safety counters are
allowed within their stated boundaries.

## Outcome boundary

FFS0 may support separate A/B/C/D/E outcomes and a scaling-law result. A
positive A/B/C result would establish recursively useful execution structure;
a positive D result would extend that closure only to the process rows actually
positive; a positive E result would establish local graph-relative adaptive
fallback.

Even a fully positive FFS0 does not establish unbounded recursion, universal
self-improvement, semantic self-understanding, or benefit outside the frozen
workloads and accounting. A right-censored result supports only the observed
lower bound.

Expected write-once artifacts:

- `results/ffs0_full_fractal_scaling.csv`;
- `results/ffs0_full_fractal_scaling.md`.
