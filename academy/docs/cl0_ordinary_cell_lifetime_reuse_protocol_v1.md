# CL0 ordinary CELL lifetime and reuse protocol v1

Status: frozen before any CL0 organism, evaluator, or executable change.

Parent: CV0 Gate-D negative `f5c7bdd`.

## Question

Can an ordinary CELL have finite, generation-safe physical lifetime and
resident-slot reuse under the same local persistence discipline as an ARROW,
without introducing a special temporary/contact CELL class?

Signed variation, inhibition, CV0 contact genesis, SV1, RS2, FD2, ARC,
authority, and the oracle remain frozen.

## Sole candidate law

Feature-gate the candidate as `cl0`. Every ordinary CELL already has:

```text
resistance
generation
live state
```

CL0 adds only local durable decay state analogous to the accepted phase-free
ARROW decay state:

```text
elapsed physical time
    → advances CELL-local decay load

one complete ordinary decay interval
    → CELL resistance decreases by one

resistance reaches zero
    → CELL becomes non-live
    → transient activation/refractory state clears
    → generation advances
    → resident CellSlot becomes reusable
```

The decay interval is the already-accepted local physical decay period. It is
measured from each CELL's own durable decay state, not absolute clock phase,
birth phase, a global sweep, or an expiry timestamp.

When a dead resident slot is reused, the new ordinary CELL receives:

- a fresh monotonically allocated `CellId`;
- the reclaimed `CellSlot`;
- the generation produced by the prior occupant's death; and
- its own `CellSpec` and zero local decay load.

Thus physical reference identity remains `ArenaId + CellId + Generation`,
while `CellSlot` remains disposable resident placement.

CELL death does not cascade-delete incident ARROWs. Their stored endpoint
references become stale immediately and cannot execute. Incident ARROWs remain
ordinary structures and disappear only through their own accepted local decay.

## Frozen gates

### 1 — weak CELL lifetime

An otherwise isolated ordinary CELL with resistance 1 dies after exactly one
ordinary local decay interval.

### 2 — proportional persistence

Otherwise identical resistance 1, 2, and 4 CELLs live for exactly 1, 2, and 4
local decay intervals respectively. No persistence class or CELL kind differs.

### 3 — phase invariance

Construct otherwise identical CELLs at absolute clock phases 0 through 9 and
give them identical subsequent elapsed time. Equal physical age must produce
equal remaining resistance and death age.

### 4 — resident-slot reuse

After an ordinary CELL dies, one newly added ordinary CELL reuses its resident
slot, receives a fresh `CellId`, and carries the next generation. The old
`CellId + Generation` never resolves to the new occupant.

### 5 — stale reference safety

After reuse, the old `CellRef` cannot resolve or receive/emit physical
activity. The replacement remains independently addressable through its new
reference.

### 6 — incoming stale ARROW

An incident ARROW whose stored target reference names the dead CELL generation
cannot deliver into the replacement occupant, even while that ARROW remains
live under its own resistance.

### 7 — outgoing stale ARROW

An incident ARROW whose stored source reference names the dead CELL generation
cannot execute from the replacement occupant. No source-ID, slot, or topology
alias may reactivate it.

### 8 — orphan topology

An ordinary weak CELL with ordinary weak incident ARROWs receives no
consequence. CELL and ARROWs become non-executable at their respective local
death events; all resident structure becomes reusable through independent
ordinary decay. CELL death must not immediately delete incident ARROWs.

### 9 — useful CELL consolidation support

Audit the accepted consequence law after Gates 1–8. If an already-established
ordinary local interaction legitimately increases CELL resistance, show that
the same CELL then persists proportionally longer.

If accepted physics contains no CELL-resistance consolidation, CL0 stops at
Gate 9. It must not invent CELL learning, transfer ARROW support into a CELL,
infer usefulness from incident degree/activity, or add evaluator
consolidation. Gates 1–8 remain diagnostic evidence but do not establish CL0
development readiness.

### 10 — representation and replay

For every executed Gate 1–8 world and phase/identity permutation, Reference
and Production mechanics must agree exactly on:

- ordered physical transitions;
- Drive and Modulatory deliveries;
- CELL and ARROW death events;
- resistance/generation/live state;
- reference resolution and resident-slot reuse;
- pending activity and final durable body;
- clock, PhysicalWork, natural quiescence; and
- exact same-mechanics replay.

ExecutionCost and resident capacity bookkeeping remain non-physical.

## Static prohibitions

Reject any new substrate logic equivalent to:

```text
TemporaryCell / ContactCell / EphemeralCell
TTL, expires_at, remaining_ticks, global forgetting phase
GC, orphan flag, degree/reference counting cleanup
incident-ARROW cascade deletion
use or participation protecting CELL lifetime
sign, inhibition, reward, usefulness, or evaluator identity
```

The new law may inspect only ordinary CELL state and elapsed local physical
time. Traversal or firing alone must not increase CELL resistance.

## Decision

- **CL0 development positive:** Gates 1–10 pass using only already-established
  CELL consolidation.
- **CL0 stopped negative:** stop at the first failed frozen gate. No rescue or
  second candidate inside CL0.

If Gate 9 is negative, the exact classification is: ordinary CELL lifetime
and generation-safe reuse work, but useful CELL persistence has no accepted
consequence-supported consolidation path. Do not resume CV0 until that missing
role is independently resolved.
