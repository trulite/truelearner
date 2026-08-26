# CORE1-E20 — Two-Lifetime Composition Protocol v1

## Status

Preregistered before any E20 implementation or runtime observation. E20 is a
composition test of the two independent boundaries isolated by frozen E19. It
is not a new symmetry-breaking tournament and does not advance accepted
physics or ARC authority.

## Question

Is the following minimal composition sufficient for bounded serial resolution
and ordinary consequence learning in the frozen CORE1-B body?

```text
junction OPEN
+ completion-triggered fresh local variation
+ route-local USED-PENDING eligibility
+ existing refractory competition
+ existing PQLC
```

The two retained facts have different locations and lifetimes:

```text
JUNCTION  CLOSED | OPEN
ROUTE     CLEAR  | USED-PENDING
```

`OPEN` means only that the local decision is unresolved. `USED-PENDING` means
only that this physical route actually participated and remains eligible for a
later consequence. Neither mark has a payload.

## Frozen E19 base

Reuse without reinterpretation:

- the E19 spatial frame, four opaque actions, eight balanced permutations,
  useful-route assignments, two supported opportunities, and autonomous probe;
- one positive physical completion receptor and fixed local junction;
- completion distinct from consequence;
- E17 route-refractory competition, reset at interaction end;
- the existing CORE1-B generic variation, consequence topology, and PQLC;
- natural quiescence, exact replay, and Reference/Production equality;
- the E19 CLOSED, missing-completion, failed-attempt, and post-success controls.

E18 in-flight protection and all-live-arrow protection remain disabled.

## Candidate A — completion-triggered fresh variation

One delivered completion while `OPEN` earns exactly one bounded local
variation/competition cycle at the installed junction. The cycle is a fixed
physical two-stage source incidence followed by the existing context incidence:

```text
completion receptor fires
  -> local candidate sources receive generation incidence
  -> the same local sources receive subdivision/refresh incidence
  -> existing context traces receive competition incidence
  -> ordinary generic variation, decay, and refractory physics quiesce
```

The two source stages are the already-required construction depth of generic
`source -> contact -> motor` variation. They are both downstream of the one
delivered completion. They are not an OPEN self-loop, timer, silence detector,
or second draw. No further stage may occur without another delivered
completion.

The cycle stores no action list or alternative set. Any executable route after
the cycle must be ordinary local topology regenerated or refreshed by that
cycle. Completion does not directly invoke a motor and does not identify the
next route.

## Candidate B — route-local USED-PENDING

During an actual route attempt only, every ordinary Drive arrow that physically
traverses receives one local binary `USED-PENDING` mark. An untraversed arrow
does not. Placement in route morphology is the only route specificity; the bit
contains no route, action, context, episode, attempt, path, outcome, or value.

While `USED-PENDING`, that traversed arrow may lose transmission and activation
state normally, and its participation trace relaxes normally, but its remaining
tentative material may not be deallocated before consequence can inspect it.
Completion neither credits nor clears the mark.

Ordinary admitted consequence uses the unchanged PQLC rule over physically
participating local structure. After that consequence transition, every
`USED-PENDING` mark in the local interaction clears, regardless of whether an
individual arrow received a material update. Closing `OPEN` and clearing route
eligibility are separate effects of the same admitted consequence.

If another actual route participates before consequence, it receives its own
local mark. E20 introduces no episode identity with which to associate a later
return to only one of several pending routes. The matrix must expose rather
than repair any resulting credit ambiguity.

## Mandatory preflights

The frozen eight-seed command is ineligible unless both preflights pass in
Reference, exact Reference replay, and Production.

### P1 — hard-position regeneration

Seed 7, opaque order `3|2|1|4`, useful route `4`:

- route 3 physically participates;
- one positive completion is delivered while `OPEN`;
- exactly one fresh local variation cycle occurs;
- at least one executable non-refractory route exists afterward;
- a second route physically participates without timeout or unreturned
  self-trigger;
- withholding completion after the first attempt produces no second attempt;
- all admissions naturally quiesce.

### P2 — completion-before-consequence credit

Seed 0, opaque order `1|2|3|4`, useful route `1`:

- route 1 physically participates and writes nonzero `USED-PENDING` marks;
- completion is positively delivered before consequence;
- the marks remain present after completion;
- ordinary consequence is admitted later;
- existing PQLC records nonzero local material updates;
- consequence clears every mark and changes `OPEN` to `CLOSED`;
- a later completion is delivered but cannot start a variation cycle;
- the useful route becomes autonomous after the frozen two consequences.

Failure of either preflight stops E20. No eight-seed evidence marker is emitted,
no matrix runs, and no rescue is permitted inside E20.

## Eight-seed gate

Only after P1 and P2 pass, execute the frozen eight seeds once. For every seed:

- a first route participates;
- the useful route participates within at most four positive completions in
  both opportunities;
- every retry is preceded by an actual completion;
- both useful completions precede their consequences;
- each useful consequence produces nonzero PQLC updates;
- the useful action is autonomous after two ordinary consequences;
- `OPEN` closes only on admitted consequence;
- every `USED-PENDING` mark clears on consequence;
- CLOSED, missing-completion, and failed-attempt controls pass;
- work is bounded; all activity naturally quiesces;
- exact replay and Reference/Production equality hold.

Primary success is `8/8`. Partial seed coverage does not earn a primitive.

## Representation audit

The implementation must demonstrate:

- `OPEN` remains exactly the payload-free E19 enum;
- `USED-PENDING` is one binary bit per physical arrow and defaults to clear;
- only actual Drive traversal can write it;
- completion cannot write, clear, or credit it;
- ordinary consequence is the only E20 operation that globally clears it;
- no stored action ID, motor ID, route ID, ordered alternative list, attempt
  index, iteration count, expected outcome, episode ID, timer, deadline, or TTL;
- no evaluator lookup of usefulness inside initiation, regeneration,
  competition, eligibility, completion, or PQLC;
- neither pending eligibility nor OPEN enters durable learned material.

Diagnostic counts and opaque seed order may be observed by the evaluator but
may not be supplied to the organism as state.

## Rejection screen

Reject E20 if any of the following occurs:

- completion merely replays E19's single junction incidence rather than a
  complete fresh local variation cycle;
- variation preserves a stored alternative reservoir;
- a retry occurs without returned completion;
- OPEN self-loops or expires by timeout/silence;
- a marker stores action, route, episode, iteration, or outcome identity;
- an untraversed route receives `USED-PENDING`;
- completion clears, credits, or closes route eligibility;
- consequence needs a special material-update rule instead of existing PQLC;
- successful consequence leaves OPEN or any pending mark live;
- exact replay, mechanics equality, bounded work, or quiescence fails.

## Frozen measurements

Record per attempt: attempted/participating action, completion delivery and
work, number of completion-rooted variation cycles, executable regenerated
routes, pending marks before/after completion and consequence, consequence
admission/Modulatory deliveries/PQLC updates, OPEN transitions, PhysicalWork,
and quiescence.

Record per seed: both attempt sequences, final autonomous action, mandatory
controls, exact replay, and mechanics equality. Hash the protocol, evaluator,
preflight evidence, and any definitive matrix.

## Allowed implementation surface

E20 may add only experimental `core1` surfaces for:

1. a bounded two-stage completion-rooted local variation incidence;
2. one payload-free per-arrow pending bit, traversal capture, deallocation
   protection while set, diagnostics, and consequence clearing.

It may not change accepted defaults, existing PQLC gain/propagation,
consequence topology, durable checkpoint schema, ARC policy, or any prior frozen
experiment.
