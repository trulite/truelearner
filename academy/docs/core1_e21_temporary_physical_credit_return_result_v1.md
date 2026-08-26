# CORE1-E21 — Temporary Physical Credit Return Result v1

## Status

**STOPPED NEGATIVE AT CORRECTED P2.** No valid P1 result exists, the frozen
evidence marker was not emitted, and the eight-seed matrix was not run. No
primitive is adopted and no CORE1 or ARC authority advances.

## Candidate

E21 attempted to replace E20's passive cross-completion marker with actual
temporary topology:

```text
returning consequence CELL
  -> temporary Modulatory arrow
  -> actually used contact CELL
  -> unchanged local return/PQLC
```

Same-admission traversal capture identified a contact only when both its stem
and outgoing positive Drive arrows had actually traversed. Capture was then to
be erased, leaving the physical arrow as the only persistent fact. A live
temporary connection anchored only its target contact and participating
incident Drive arrows, survived completion without timeout, and was removed
only after admitted consequence.

## Invalid v1 staging observation

The first exact useful-first P2 appeared positive:

```text
Modulatory deliveries  1 | 3
PQLC updates           2 | 9
final autonomous       action 1
replay                 exact
Reference/Production   exact
```

That result was ineligible. During same-admission contact identification, the
capture bits still invoked E20's passive deallocation protection. Hard seed 7
then exposed the contamination before any temporary return edge existed: its
initial junction emitted four motor crossings simultaneously.

Protocol v2 froze the sole conformance correction: under E21, capture could
record traversal but could not protect anything. Only materialized physical
topology could anchor a route. The complete exact P2 gate was then rerun from
scratch.

## Corrected decisive P2

The corrected candidate was exact across Reference replay and Production, but
failed both fixed useful-first opportunities:

```text
Modulatory deliveries  0 | 0
PQLC updates           0 | 0
final autonomous       none
replay                 exact
Reference/Production   exact
```

This zero-Modulatory result proves that no temporary return connection existed
when consequence fired. A tagged connection is itself protected from decay,
completion cannot remove it, and any live connection from the returning CELL
would necessarily schedule a Modulatory delivery before consequence cleanup.

The formation helper ran only after the physical action admission reached
quiescence. Without E20 passive protection, the tentative participating stem
and outgoing arrow had already deallocated by that boundary, so the used
contact could no longer be identified and no connection was materialized.

## Retained distinction

The proposed physical wire remains conceptually different from passive
metadata, but E21 v1 located its genesis too late:

```text
route traverses
-> tentative route disappears during the admission
-> quiescent observer tries to form return edge
-> no used live contact remains
-> consequence has no edge to traverse
```

Thus E21 does not refute temporary physical credit return. It establishes a
sharper necessary condition:

> The return topology must be created as part of the actual route-participation
> event, before that event's tentative contact can disappear. Reconstructing a
> wire from post-admission state is already too late.

That atomic topology-genesis question is not another lifetime extension and is
not repaired inside E21.

## Rejected repairs

After corrected P2 failed, E21 did not:

- re-enable passive USED-PENDING protection;
- create the connection from an evaluator-supplied route identity;
- retain a post-hoc path/action list;
- change PQLC gain, propagation, or eligibility;
- add a timeout, silence detector, or completion-created wire;
- retry P1 or spend the matrix.

## Evidence boundary

- Corrected P2 Reference/replay/Production: completed, exact, negative.
- Valid P1: not run after corrected P2 stopped the gate.
- Evidence marker: **not emitted**.
- Primary matrix: **not run**.

Strict release Clippy for the E21 evaluator passed, as did the focused core E18
in-flight-generation regression. The cumulative Academy sensorimotor slice
retained the same four expectation failures recorded at E20 in observation
paths that do not enable E21; two tests passed and four failed. They remain
visible and are not reclassified.

Raw bounded summary:
`experiments/results/core1_e21_temporary_physical_credit_return_v1/preflight.csv`.

Frozen corrected candidate commit: `f030e0a`.

SHA-256:

- protocol v1: `c2f50de80231290c112212ec2ac080477e3dc21f8d00f2936a3ea29670c2a170`;
- conformance correction v2: `563a10e7337847737da22c158d5a1e322006448eaa28047b0d9fe1cff8324f06`;
- evaluator: `91b805ac989f99e9c275f17615b02d604bdb41f08644f0f22b1a2e569a902915`;
- bounded preflight CSV: `e17badb2d9c7962ca7d08dd06dd3de5a88fddce6a9d2412ae8cd067e17a89d27`.
