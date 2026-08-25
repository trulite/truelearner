# RI0 opaque-identity renaming invariance protocol v1

Status: frozen before evaluator implementation or execution.

Parent: RS2 v4 immutable negative `f28b076`.

## Question

Can arbitrary numeric CELL/ARROW names change physical behavior when graph,
couplings, thresholds, delays, phases, initial activity, and serial-generation
procedure are otherwise identical?

RI0 changes no organism law and does not rerun RS2.

## Names under test

The diagnostic distinguishes:

1. internal `CellId`/`ArrowId` allocation names, changed by insertion order
   while stable physical identities remain fixed;
2. numeric `physical_id` labels used by the global scheduling key, changed by
   a bijection and mapped back to logical CELL names for comparison.

Resident slots and arena placement remain causally irrelevant.

## Minimal worlds

### Same-tick two-source collision

Two ordinary sources fire at the same tick. Their ordinary Drive arrows reach
one thresholded target at the same tick and phase with impulses `+1` and `-1`.
An unrelated CELL is present as a renaming control.

### Same-source parallel-arrow collision

One source fires and schedules parallel `+1` and `-1` Drive arrows to the same
target at the same tick and phase. Arrow insertion order is permuted while the
logical arrows are mapped back before comparison.

These worlds intentionally expose any causal dependence on numeric tie-break
names. No learning, Modulation, variation, decay decision, RS2 topology, or
semantic outcome is involved.

## Frozen permutations

- identity numbering;
- reverse numbering;
- deterministic random bijection;
- swap only the two causally competing sources/contacts;
- swap only unrelated CELLs;
- reverse internal CELL insertion order with physical IDs fixed;
- reverse parallel ARROW insertion order.

Each case runs under Reference and Production and is reconstructed for exact
same-mechanics replay.

## Normalized comparison

Before comparison, all CELL and ARROW IDs and physical IDs are mapped back to
the fixed logical graph names. Compare:

- complete ordered physical transition history;
- Drive deliveries and firing sequence;
- Work and physical clock;
- normalized durable and transient final state;
- pending activity;
- natural quiescence and exact replay.

Execution-cost counters are excluded.

## Decision

- If every permutation differs only in trace labels, RI0 is positive and the
  RS2 identity-permutation fixture/predicate requires later repair.
- If any bijective renaming changes firing, activity, pending state, or durable
  outcome, RI0 is a real scientific negative: arbitrary numeric identity leaks
  into organism physics.

Any negative stops. RI0 adds no scheduler repair, simultaneous-event rule,
serial change, comparator repair, or RS2 v5. CE1, FD2, ARC, authority, oracle
status, and `arch.md` remain blocked and unchanged.

