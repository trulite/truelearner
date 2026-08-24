# RC0b preregistration: grounded motif substitution

Protocol identifier: `grounded-motif-substitution-rc0b-v1`

Status: frozen before RC0b implementation and before any definitive RC0b run.

## Scientific boundary

RC0b asks whether repeated grounded computation can earn one substrate-native
motif which makes a future execution observationally equivalent but physically
smaller.

> Equivalence is defined only over the preregistered observable interface;
> unobserved internal routing is explicitly allowed to disappear.

RC0b begins only after RC0a has removed recurrent reflected interpretation and
binding-dereference cost. It may compact actual lower execution. It may not add
a new executor, supplied macro, depth-specific route, answer cache, evaluator
target, or second motif.

Two outcomes are frozen separately:

- **RC0b-A — computational compaction:** the earned motif plus every required
  residual effect has the same observable trace and less work than the same
  mature RC0a computation;
- **RC0b-B — economic prerequisite:** across the frozen balanced mature-use
  workload, the whole substituted invocation costs less than the unchanged
  concrete invocation.

RC0b-A does not imply RC0b-B. Only RC0b-B may open RE0. RC0b contains no
acquisition/carrying amortization claim.

## Frozen ancestry

- RC0a positive tag: `rc0a-compiled-grounded-recurrence-positive`
- RC0a positive commit:
  `3ae01384327bcd2197fd3e2c057b1f9a20c296c5`
- RC0a implementation tag: `rc0a-compiled-grounded-recurrence-implementation`
- RC0a implementation commit:
  `411986336983ff96c5e5665d72dcd450acdd2f0b`
- RC0a protocol SHA-256:
  `e641aa2367e5bf504b28b22fb84268e69092577b9b6efdf5903d924b990dd2a4`
- RC0a definitive CSV SHA-256:
  `398d0e56f9a528871b97fd00b9a24fe3a9d2c2b96253cb4268ada264fbc9faef`
- frozen RC0a implementation SHA-256:
  `3274fbf2380c0db457b585d03e7ac0476daf393f3910e2ec37f0b65d3513ab55`
- F0.2a positive tag: `f0.2a-learned-computational-substitution-positive`
- F0.2a positive commit:
  `929fbe9c0ac432b945d49fd19bef4b27fd7b5461`
- F0.2a implementation commit:
  `fef656678f0f1bbb003e297671779201b2270d75`
- F0.2a protocol commit:
  `90a0cc55f8cf9deccb878c2bf2e06b4197933208`
- F0.2a protocol SHA-256:
  `72343746421ed5d83f78cb300cb91894610bec5ea0248c40ac4a02c87c44f888`

The frozen RC0a/RG0a/RP0a state is imported. RC0b does not relearn those
capabilities except when definitive seed reconstruction verifies their frozen
endpoints.

## Preregistered observable interface

For one invocation:

```text
ObservableTrace =
    final concrete state
  + ordered externally meaningful effects
  + quiescence/error signals
  + state exposed at declared interruption boundaries
  + context-dependent effects
```

The interface is concretely frozen as:

1. final answer and final current concrete identity;
2. ordered `CurrentUpdate(identity)`, `ContextEffect(site, marker)`, and
   `Answer(identity)` events;
3. quiescence, missing-binding/error, and activity-limit signals;
4. at each declared safe boundary, the current identity, ordered-effect prefix
   length, and active context marker;
5. the final context-effect ledger.

Raw queue contents, role-routing spikes, transparent relay occurrences,
temporary arrows, comparison order, and internal reflected activity are not
observable. They are implementation and may disappear.

A safe interruption boundary occurs immediately before recurrence re-enters
the learned `Apply` role, after the preceding current update and its ordered
effects have completed. Queue-internal states between role firings are not
declared boundaries. Re-entry reconstructs the suffix from the boundary's
exposed state and the same external context.

The evaluator may record this interface and schedule interruptions. The learner
cannot read `ObservableTrace`, boundary markers, a counterfactual equality bit,
or any evaluator classification.

## The one allowed motif

The only RC0b motif is a learned role-relative recurrence cycle already
executed by RC0a. It may fuse locally transparent relay occurrences inside
that cycle. A relay occurrence is locally transparent only when:

- its incoming spike identity is its outgoing spike identity;
- it has no local concrete-state mutation;
- it emits no ordinary local context effect;
- its incoming and outgoing transitions belong to the earned RC0a dispatch;
- its parent learned-arrow identities and role endpoints are stable.

The expected eligible relays in the frozen program are the roles reached
between `Apply -> Slot1 -> Slot2`, `Lookup -> Result -> Current`, and
`StoreCurrent -> Success -> Apply`. Their names are used only here and by the
evaluator for reporting. Candidate discovery receives integer role identities,
local spike identity continuity, local mutation/effect flags, recurrence, and
ordinary terminal success/failure.

The one persistent motif may contain only:

```text
canonical recurrent role cycle
parent compiled-arrow identities
role-relative emitter/relay/target triples
local transparency compatibility signatures
consolidation state
```

It must contain no concrete CELL or filler identity, query, answer, episode,
depth, prefix/suffix length, observation boundary, effect marker, or evaluator
result.

At invocation, ordinary bindings join the persistent motif to temporary local
arrows. The substitute is realized by the same CELL/ARROW/SPIKE executor used
by RG0a and RC0a: it rewires only the eligible temporary relay routes around
the current episode's bound cells. Lookup, current-state mutation, answer
delivery, nontransparent context effects, and all other residual computation
still execute and are fully charged. There is no RC0b opcode or direct
transformation executor.

## Learner-visible discovery and credit

The organism observes its ordinary role-relative RC0a event stream. It detects
a recurrent cycle by repeated local role sequences, canonicalized without an
externally supplied start or end. At most one evidence credit is applied to a
candidate motif per episode even if the cycle repeats many times.

Candidate relay triples are emitted only from adjacent locally possible
compiled routes in the observed cycle. There is no exhaustive role-product
search. Reuse the frozen credit law:

- successful traced episode: `+2`;
- failed traced episode: `-1`;
- consolidation threshold: strength `6`.

Thus the motif requires three separate successful ordinary RC0a episodes. A
candidate is earned from recurrence, spike continuity, transparency, and
ordinary terminal feedback only. Work reduction and trace equivalence are
scientific evaluation criteria; neither is optimization feedback supplied to
the learner.

## Local validation and fallback

Before temporary motif installation, validate the parent compiled arrows,
role endpoints, and local transparency signatures. If a previously transparent
relay acquires a local effect or mutation, the motif invalidates before firing.
The normal hierarchy is:

```text
earned motif invalid
        -> RC0a compiled recurrence
RC0a parent invalid
        -> RG0a generic grounded interpretation
```

Fresh concrete identities, changed physical bindings, changed chain prefix or
suffix, unseen depth, and a safe interruption/re-entry are not invalidations.
No fallback may use a target or answer oracle.

## Identical-start counterfactual

Every mature full/substitute comparison branches from the same frozen
permanent state and identical episode state:

```text
FULL RC0a
    -> observable trace T
    -> work W_full

MOTIF SUBSTITUTE + residuals
    -> observable trace T'
    -> work W_sub
```

RC0b-A eligibility requires `T' == T` and `W_sub < W_full`. Residual effect
work and invocation/validation/installation work are included in `W_sub`.
Random seeds and episode specifications are identical across the branches.

## Arms

Every evaluation branch starts from the same frozen learned program, earned
RC0a dispatch, and, where applicable, the same earned motif.

1. **CONCRETE REFERENCE** — unchanged frozen lower direct execution.
2. **FULL RC0a** — mature compiled grounded recurrence with no motif.
3. **MOTIF SUBSTITUTE** — the earned one-motif fast path plus residual work.
4. **CHANGED SURROUNDINGS** — motif execution with fresh concrete identities,
   a non-identity permutation of physical lower cells, and held-out chain
   prefixes/suffixes and total depths.
5. **INTERRUPTION/RE-ENTRY** — interrupt at each declared safe boundary and
   reconstruct the remaining suffix from exposed state; traces and total work
   are reported against uninterrupted execution.
6. **CONTEXT EFFECT INVALIDATION** — a formerly transparent relay gains a
   fresh ordered `ContextEffect`; the motif must invalidate before firing and
   FULL RC0a must preserve the effect.
7. **FORCED STALE SAME-ENDPOINT CONTROL** — evaluator-only broken control that
   deliberately bypasses motif compatibility validation after the same context
   change. It must reach the same final endpoint while omitting/misordering the
   required effect, hence fail `ObservableTrace` equality. This arm is never an
   organism capability path.
8. **RC0a PARENT INVALIDATION** — replace learned parent-arrow identities while
   preserving role endpoints. Both the motif and compiled dispatch invalidate;
   RG0a generic grounding resumes and remains correct.
9. **SUBTHRESHOLD EVIDENCE** — two successful episodes only; no motif is earned
   and FULL RC0a executes.
10. **SHUFFLED RECURRENCE EVIDENCE** — deterministically shuffle observed
    role-relative adjacency while retaining marginal events. No compatible
    motif may fire; FULL RC0a executes.
11. **NO BINDINGS** — remove current grounding before execution. Neither the
    motif nor fallback may be fully competent.

The same-endpoint arm is the primary semantic control: endpoint equality alone
must be insufficient when an ordered intermediate effect is required.

## Three-speed harness

### MICRO

- development only; never claim eligible;
- one seed;
- acquisition depths `3, 4, 6`;
- held-out depths `5` and `8`, two fresh episodes each;
- focused checks for motif acquisition, fresh binding, actual relay removal,
  observable equality, context invalidation, and forced-stale trace failure.

### GATE

- development only; never claim eligible;
- one development seed outside the definitive seed range;
- acquisition depths `3, 4, 6`;
- held-out depths `5, 8, 13`, four fresh episodes each;
- all eleven arms and every qualitative and accounting gate.

### DEFINITIVE

- reconstruct eight integrated frozen RP0a/RC0a seeds `0..7` and require exact
  frozen endpoint parity;
- acquire one motif from three fresh successful FULL RC0a episodes at depths
  `3, 4, 6`;
- evaluate 16 fresh episodes per seed at held-out depths
  `5, 8, 16, 32, 64, 128`;
- use a balanced uniform workload over all seed/depth/episode cells;
- independent cells may run in parallel; each organism remains deterministic
  and single-threaded;
- retain compact traces in memory and serialize once after completion;
- execute the frozen definitive matrix exactly once.

Development modes write no result artifacts and cannot establish either RC0b
outcome.

## Work accounting

All frozen RG0a and RC0a counters retain their meanings. RC0b adds distinct
primitive counters for:

- recurrent observations and adjacency comparisons;
- motif candidates and success/failure credit updates;
- motifs earned and invalidated;
- parent/transparency compatibility comparisons;
- temporary shortcut binding reads and installations;
- shortcut arrow evaluations and firings;
- residual effects;
- RC0a and RG0a resumptions.

Acquisition work and persistent storage are reported separately from mature
runtime. Mature `W_sub` includes validation, temporary installation, shortcut
firing, and all residual work. Removed relay activations and their ordinary
route firings are absent physical events; they are not charged synthetically.
The runner must also report primitive lower event deltas so work reduction is
auditable as genuine event elimination rather than counter relabeling.

## Frozen outcome thresholds

### RC0b-A

RC0b-A passes only if, in every definitive seed/depth cell:

- the motif and FULL RC0a observable traces are exactly equal for all ordinary,
  changed-surrounding, and interruption/re-entry episodes;
- both paths are correct, quiescent, and activity-limit free;
- `W_sub < W_full` after all motif and residual runtime charges;
- at least one transparent lower relay activation and its associated ordinary
  route firing are eliminated per recurrent cycle;
- no stateful or externally meaningful lower event is eliminated.

The aggregate work delta and per-depth deltas are reported. No mean improvement
may hide a failing cell.

### RC0b-B

RC0b-B passes only if, over the preregistered balanced definitive mature-use
workload:

```text
sum(W_sub) < sum(W_concrete)
```

and every seed's aggregate substituted work is below that seed's aggregate
concrete work. Per-depth signs are reported without suppression but are not a
separate conjunctive threshold. This result is only an economic prerequisite;
RE0 must later charge acquisition, installation, carrying, and maintenance.

## Conjunctive scientific gate

RC0b-A additionally requires:

1. frozen ancestry reconstruction parity;
2. exactly one persistent motif, earned from three separate successful FULL
   RC0a episodes under the frozen credit law;
3. persistent motif state contains only the allowed role-relative structural
   fields and no forbidden identity or evaluator information;
4. candidate discovery consumes no supplied motif/boundary/depth/answer labels;
5. ordinary and changed-surrounding arms preserve fresh binding generality;
6. interruption/re-entry preserves the declared boundary state and complete
   observable trace;
7. context-effect change invalidates before shortcut firing, preserves the
   ordered effect through RC0a, and does not destroy the valid learned program;
8. forced stale execution reaches the same final endpoint but fails exact trace
   equality, establishing that the evaluator checks more than answers;
9. parent replacement invalidates both fast paths and resumes correct RG0a;
10. subthreshold and shuffled evidence produce zero compatible motif firings;
11. no-bindings remains incompetent;
12. duplicate evaluation is deterministic, branch starts/fingerprints match,
    temporary state is erased, and permanent read-only state is stable;
13. the same CELL/ARROW/SPIKE executor runs FULL and SUBSTITUTE branches;
14. source inspection finds no second motif, hierarchy, direct executor,
    supplied macro boundary, depth key, answer cache, concrete-identity cache,
    evaluator feedback, uncharged residual, or suppressed observable effect.

Failure of any condition is an RC0b-A negative. RC0b-B is evaluated only after
the RC0b-A gate but is recorded independently.

## Stopping rule and result boundary

If RC0b-A is negative, stop before RE0, hierarchy, or F1. If RC0b-A is positive
and RC0b-B is negative, record computational compaction positive / economic
prerequisite negative and keep RE0 blocked. Only a positive RC0b-B opens RE0.

The definitive runner must refuse to overwrite an existing artifact. Only the
first execution of the implementation frozen against this protocol is claim
eligible. Expected artifact names are:

- `results/rc0b_grounded_motif_substitution.csv`
- `results/rc0b_grounded_motif_substitution.md`

Motif hierarchy, arbitrary chunks, acquisition amortization, RE0, and F1 are
explicitly out of RC0b implementation scope.
