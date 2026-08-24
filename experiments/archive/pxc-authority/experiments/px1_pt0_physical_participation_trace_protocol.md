# PX1-PT0 physical participation-trace protocol

Status: **PREREGISTERED; DEVELOPMENT EVIDENCE UNSPENT; PX1 NON-AUTHORITATIVE**.

## Frozen parent

- authoritative PX0 commit: `e884ae133a562d475565a36700d929b51dd2b2d2`;
- active PX0 law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- PX1 recurrent-stability negative commit:
  `e28ce95621176cf024c7ae4a3ede145542912ff9`;
- negative CSV SHA-256:
  `7ddf75567e4b61fd735a042ddafb949fd85be57021285465a08ca17285c61e80`;
- negative audit SHA-256:
  `5935c9aea0ff330aefe28a80903a10d515e440c0fd07fa658b4db68644651a28`.

PT0 does not amend or rescue that negative. It isolates its first missing
physical relation.

## Question

> Can returned activity change only a recently participating continuation by
> overlapping a short-lived trace of that continuation's actual physical
> activity, without provenance metadata or a new substrate variable?

## First mechanism tested

The PX0 law remains byte-identical. Each anonymous continuation occupies a
physically distinct local branch cell and outgoing arrow. When a branch cell
fires, the existing arrow-local eligibility window is ordinary transient
physical state. A later return reaches branch cells without carrying a branch
label. Only a branch whose outgoing structure was recently active may use it.

No field, enum, object, or call may encode `chosen`, `cause`, `return_for`, role
identity, correctness, or ownership. The evaluator may know which anonymous
branch was physically activated but cannot route a different kind of return to
it.

## PROBE worlds

Two anonymous continuations have identical weak physical opportunities.

1. **in-window participation** — A physically fires and its continuation
   executes; identical return activity subsequently reaches A and B inside the
   frozen local window. Only A may strengthen.
2. **expired participation** — A executes, but identical return reaches both
   branches only after its eligibility has expired. Neither may strengthen.
3. **no participation** — neither continuation executes; identical nearby
   return reaches both. Neither may strengthen.
4. **swap** — B, not A, executes before identical return. Attribution must swap.
5. **joint participation** — A and B both genuinely execute before identical
   return. Both may strengthen.

All worlds record candidate resistance/coupling consequences, actual branch
firings, actual continuation effects, return arrivals, quiescence, work, and
complete fingerprints.

## MICRO controls

- fresh physical identities;
- mirrored layout;
- reversed allocation order;
- duplicate complete-state replay;
- return with no live eligibility;
- participation with no return;
- return just inside versus just outside the measured physical window;
- a second nearby nonparticipating opportunity;
- no semantic/type/chooser source surface;
- frozen PX0 hash exact.

## Pass rule

PT0 passes only if every primary and transfer world establishes:

```text
executed branch + in-window return     → that branch strengthens
nonexecuted branch + same return       → no strengthening
executed branch + late return          → no strengthening
two executed branches + return         → both strengthen
swap physical execution                → attribution swaps
duplicate complete state               → duplicate exact
all finite worlds                       → natural quiescence
```

A result that suppresses return, permits only one branch to learn by rule, or
depends on handle/allocation/layout identity fails.

## Stopping rule

If PT0 passes, a separately preregistered PX1 stability retry may combine the
physical participation topology with the previously quiet margin arm. No
damping arm is selected by PT0 itself.

If PT0 fails, freeze the first physical attribution collapse. Do not add a
trace field or provenance representation automatically.

PT0 is development-only. Definitive execution, PX1 authority, PX2, PX-C, the
continuous organism, and Harness H1 remain forbidden.

