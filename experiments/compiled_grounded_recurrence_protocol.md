# RC0a preregistration: compiled grounded recurrence

Protocol identifier: `compiled-grounded-recurrence-rc0a-v1`

Status: frozen before RC0a implementation and before any definitive RC0a run.

## Scientific boundary

RC0a is a compatibility gate, not a new capability gate and not an economics
claim. It asks only:

> Can repeatedly successful use turn an already learned grounded recurrent
> transition into ordinary cheap local CELL/ARROW/SPIKE dispatch without losing
> binding generality or plastic fallback?

RC0a may stop re-interpreting one already learned transition. It may not
discover a faster algorithm, substitute a multi-step fragment, skip lower
effects, install a depth-specific macro, or introduce another executor.

The frozen expected scaling form is:

```text
generic grounded overhead  = fixed setup + k  * recurrent transitions
compiled grounded overhead = fixed setup + k' * recurrent transitions
```

The primary quantitative gate is a slope change. RC0a does not require total
compiled grounded work to beat the concrete reference.

## Frozen ancestry

- RG0a positive tag: `rg0a-reflected-grounding-functional-positive`
- RG0a positive commit:
  `c690fad5e9defccaae69f3920ee24e74a1f7ee37`
- RG0a protocol SHA-256:
  `34c6f77ae4501a8ff916a47f2099551f54bd24ecbc9a06cf035ebd4c3ef69ee4`
- RG0a definitive CSV SHA-256:
  `fe0109fb6ef138be740699ddb35323a9b2c72b99a10eb678981dd2374746c356`
- RG0a audit SHA-256:
  `6049fe7c8ba67cbd19998e351fb8e3d72215e51e9ad1951bced72cddc6812285`
- Frozen RG0a implementation SHA-256:
  `8b1361d3da1603a365e47ec5aa55074a4f2a41b946b3fa746533fc91f676c5a2`
- RP0a positive commit:
  `8faefa0c3e92965c61513ce802bc11cc2654ec63`
- S1.1 reference commit:
  `a114f66946122d943062512a4a1a6dd3c9e27fca`
- v20 reference commit:
  `e88194093fde02c42a963baeb492880396a5d55f`
- v21a reference commit:
  `b881b9ea0a03a1a1162a43cc0505e6bdda7173e9`
- v21b reference commit:
  `31d6f234071b927d93eff64663d0b9cf8da83851`

RC0a imports the unchanged frozen RG0a/RP0a competence state. It does not
relearn RP0a except when reconstructing definitive seeds to verify their frozen
endpoints.

## Earned compilation physics

Compilation observes only reflected arrow firings produced by generic grounded
execution and the ordinary terminal success/failure feedback already available
to the learner. For each transition, feedback is applied at most once per
episode, even when recurrence fires it repeatedly.

Reuse the frozen RP0a credit law unchanged:

- successful traced transition: `+2`;
- failed traced transition: `-1`;
- consolidation threshold: strength `6`.

Thus every compiled transition requires three separate successful generic
episodes. No evaluator role, correct target, answer identity, total depth, or
future concrete identity is available to compilation.

A persistent compiled arrow may contain only:

```text
parent learned-arrow identity
source reflected role
target reflected role
consolidation state
```

It must contain no lower CELL identity. At each invocation, fresh structural
grounding installs temporary ordinary arrows by joining that persistent
role-relative structure to the current episode's bindings:

```text
current bound source CELL
        -> temporary earned ARROW
        -> current bound target CELL
```

Compilation and temporary-arrow installation are charged separately. Mature
compiled firing must not scan the reflected program or dereference role
bindings per recurrent transition.

## Invalidation lifecycle

Before temporary installation, the compiled arrow's parent identity and
role-relative endpoints must remain locally compatible with the current frozen
learned topology. Any incompatible or replaced parent transition invalidates
the compiled dispatch set before it fires. Generic grounded interpretation then
resumes using the current learned topology.

Invalidation is triggered only by structural incompatibility, failed
dereference, or ordinary failure feedback. It may not consume evaluator
correctness or a supplied target. The invalidation condition replaces learned
arrow identities while preserving their role-relative endpoints, so resumed
generic execution remains behaviorally testable without an oracle.

Changed concrete bindings and fresh identities are not invalidations. They must
install a fresh temporary compiled route successfully.

## Arms

Every evaluation branch starts from the same frozen RP0a state and, where
applicable, the same earned compiled state.

1. **CONCRETE REFERENCE** — frozen lower direct routing, accounting reference
   only.
2. **GENERIC GROUNDED** — unchanged RG0a reflected lookup and binding
   dereference on every route firing.
3. **COMPILED GROUNDED** — earned role-relative transitions installed against
   fresh episode bindings and fired as temporary local arrows.
4. **CHANGED BINDINGS** — the same compiled state with a non-identity
   permutation of physical lower CELL positions and fresh identities.
5. **INVALIDATED TRANSITION** — learned parent arrows are replaced by new arrow
   identities with identical role endpoints. The old compiled set must
   invalidate before firing and generic grounded execution must resume.
6. **SUBTHRESHOLD EVIDENCE** — only two successful generic episodes. No
   transition may compile; execution must remain generic.
7. **SHUFFLED CONSOLIDATION EVIDENCE** — source/target associations in the
   observed compilation traces are deterministically shuffled. No incompatible
   compiled route may fire; local validation must invalidate it and resume the
   generic path.
8. **NO BINDINGS** — remove current temporary grounding before execution. It
   must not be fully competent through either path.

## Three-speed harness

### MICRO

- development fixture only; never claim eligible;
- one seed;
- depths `3` and `5` as needed by the focused tests;
- two acquisition episodes for subthreshold checks and three for compilation;
- two fresh evaluation queries;
- binding freshness, compiled delivery, slope mechanics, and invalidation.

### GATE

- development fixture only; never claim eligible;
- one development seed outside the definitive seed range;
- depths `3, 5, 8`;
- four fresh queries per depth;
- all eight arms and every qualitative gate.

### DEFINITIVE

- reconstruct integrated frozen RP0a seeds `0..8` and require exact frozen
  endpoint parity;
- three fresh generic compilation episodes per seed, at depths `1, 2, 3`;
- evaluate 16 fresh episodes at unseen depths `5, 8, 16, 32, 64, 128`;
- independent seed cells may run in parallel; each organism remains
  deterministic and single-threaded;
- serialize only after completion;
- execute the frozen definitive matrix exactly once.

MICRO and GATE may use an evaluator-constructed development fixture, write no
artifacts, and can never establish the RC0a claim.

## Work and slope accounting

Keep the frozen RG0a primitive counters unchanged and add separate counters for:

- compilation candidates and comparisons;
- success/failure credit updates;
- compiled transitions earned and invalidated;
- parent-topology validation comparisons;
- per-invocation binding reads and temporary-arrow installations;
- compiled arrow evaluations and firings;
- generic resumptions.

Acquisition and persistent compilation work are reported separately from mature
runtime. Temporary installation is part of mature runtime. No counter may be
redefined to make compilation cheaper.

For each mature arm and depth, aggregate runtime over an equal number of
episodes. Let `x = 3*depth + 2`, the frozen number of recurrent route firings,
and let `y = grounded runtime - concrete runtime`. Compute the ordinary least
squares slope over all definitive depths. Compare exact rational slopes without
rounding.

The slope gate passes only if:

```text
k > 0
k' >= 0
k' / k <= 0.20
```

This is at least an 80% reduction in per-transition interpretation overhead.
The compiled intercept is reported without a pass threshold, but it must be
independent of evaluation depth; no depth-keyed persistent state is permitted.

## Conjunctive gate

RC0a passes only if all conditions hold:

1. frozen ancestry and definitive RP0a reconstruction parity;
2. every persistent compiled transition was earned from three successful
   generic episodes under the frozen credit law;
3. compiled persistent state contains every required learned transition and no
   lower identity, episode identity, answer, target, or depth key;
4. concrete, generic, compiled, changed-binding, invalidation, subthreshold,
   and shuffled-evidence arms meet their specified behavioral/path criteria at
   every seed and depth;
5. compiled and changed-binding arms answer every episode with fresh lower
   identities, exact route-firing counts, quiescence, and no activity-limit hit;
6. mature compiled firing uses no reflected-arrow scan, per-step binding read or
   delivery, direct executor, pre-resolved persistent route, oracle, fallback,
   fragment substitution, skipped lower effect, or multi-step macro;
7. changed physical bindings do not alter persistent compiled state;
8. transition replacement invalidates before any compiled firing, leaves no
   stale compiled route, resumes generic grounded execution, and remains
   correct;
9. subthreshold evidence earns zero compiled transitions;
10. shuffled evidence produces no compatible compiled firing;
11. generic and compiled lower-effect counters are identical for identical
    episodes; RC0a removes interpretation only;
12. exact duplicate evaluation is deterministic, temporary bindings/routes are
    erased, permanent read-only fingerprints are stable, and all workspaces are
    destroyed;
13. the exact slope test satisfies `k'/k <= 0.20`;
14. source inspection confirms no RC0b fragment substitution, lower-work
    elimination, depth macro, evaluator target, concrete-identity cache, or new
    executor.

All gates are conjunctive. A qualitative or slope failure is a negative RC0a
result and blocks RC0b implementation.

## Result boundary

The definitive runner must refuse to overwrite an existing artifact. Only the
first execution of the implementation frozen against this protocol is claim
eligible. Expected artifact names are:

- `results/rc0a_compiled_grounded_recurrence.csv`
- `results/rc0a_compiled_grounded_recurrence.md`

RC0b fragment substitution, RE0 amortization claims, and F1 are explicitly out
of RC0a implementation scope.
