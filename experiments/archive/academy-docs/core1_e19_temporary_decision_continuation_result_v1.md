# CORE1-E19 — Temporary Decision Continuation Result v1

## Status

**STOPPED NEGATIVE AT PREFLIGHT.** The frozen eight-seed evidence command was
not executed. No authority, ARC, or downstream CORE1 result advances.

## Candidate tested

The implementation adds exactly one payload-free retained marker:

```text
DecisionOpenness = CLOSED | OPEN
```

One permanent neutral receptor/sink pair receives a positive physical
completion SPIKE. If and only if the marker is OPEN, that completion admits one
ordinary junction wave into the already-existing candidate-source and
context-trace CELLs. Route-local refractory remains separate E17-R state.
Ordinary consequence is a later, distinct admission and is the only operation
that closes OPEN.

The marker stores no action, attempt, iteration, timeout, route, alternative,
success value, or consequence value. E18 in-flight protection is disabled.

## Decisive hard-position preflight

Seed 7 uses opaque order `3|2|1|4`, with useful route `4`.

```text
OPEN
route 3 participates
completion returns with 35 physical work
OPEN remains OPEN
one ordinary junction wave runs and quiesces
no executable non-refractory route exists
no second attempt
no consequence
no learned policy
```

Reference and Production observations are exact. Both naturally quiesce.

This fails the central `8/8` target on one preregistered hard permutation before
evidence expenditure. The OPEN marker works as a marker; existing variation
does not reconstruct a complete executable alternative from one returned
junction wave.

## Positive-first control

Seed 0 uses order `1|2|3|4`, with useful route `1` first.

The route physically participates, completion returns, OPEN remains live across
completion, and later ordinary consequence is admitted and changes OPEN to
CLOSED. However:

```text
PQLC updates after completion return = 0
final autonomous useful action       = none
```

The completion-triggered junction wave occurs before the later consequence and
does not preserve the eligibility needed to consolidate the successful route.
The second opportunity again finds the useful route only after another failed
route and regenerated activity, but consequence again produces zero updates.

Thus E19 has two independent failures:

1. one completion return does not regenerate an executable later alternative;
2. completion-before-consequence does not retain successful causal eligibility
   long enough for PQLC consolidation.

## Controls that passed

- CLOSED plus ordinary silence produced no action;
- withholding completion after one physical attempt produced no reactivation;
- a completion SPIKE was positively delivered while CLOSED but was ineligible
  to emit a junction wave;
- failed completion produced zero consequence/PQLC updates;
- completion did not close OPEN;
- admitted consequence changed OPEN to CLOSED;
- another completion after CLOSED was ineligible;
- every observed physical admission naturally quiesced;
- the hard negative was Reference/Production exact.

The complete success control did not pass because consequence closed OPEN but
produced zero PQLC updates. Closing alone is not learned success.

## Why the candidate was not widened

Any immediate repair crosses the frozen rejection boundary:

- a second junction activation without a second returned completion is an
  unreturned self-loop;
- holding all weak routes is an alternative reservoir;
- waiting for silence is a timeout;
- retaining the selected path until consequence adds causal/path identity;
- closing or consolidating on completion conflates completion with consequence;
- special delayed-consequence eligibility changes existing PQLC physics.

None was introduced.

## Retained conclusion

> Payload-free local openness and positive completion are coherent physical
> primitives, but OPEN + one completion return + E17-R + existing variation +
> existing PQLC is insufficient for bounded serial decision continuation in
> the frozen ARC3 body.

The next boundary is not “more exploration.” It is whether the substrate has a
lawful physical interaction that can remain causally eligible across completion
until consequence while also regenerating a later opportunity without an
unreturned loop or stored alternative set.

## Evidence boundary

- Focused compile and strict Clippy: passed.
- Seed 7 Reference/Production preflight: exact.
- Seed 0 positive-first preflight: stopped negative as above.
- Frozen eight-seed evidence marker: **not emitted**.
- Primary matrix: **not run**.

Raw bounded preflight summary:
`experiments/results/core1_e19_temporary_decision_continuation_v1/preflight.csv`.

Frozen implementation commit: `d62ac8d`.

SHA-256:

- protocol v1: `abcfadda2011a04ee6ac7c9eff49baf4610456ed7ff2b3483e8c9a2570a976a8`;
- corrected protocol v2: `1ea105982082a1516f4a6ed9fa97d9e16a161f30f90e44c8b8412b96b0c36cc7`;
- bounded preflight CSV: `c5ef91ff163ae8b22f99d48d772b731e5e9b74f33ab9a6258fbcc2d19e24fca5`;
- frozen evaluator: `fa5233d1cc537e3d0e1b36a52db69dc4e503c93d98d011a2f5b1656916424e8b`.
