# RI0 opaque-identity renaming invariance immutable negative v1

Status: real scientific negative. Downstream development remains blocked.

Protocol: `5238d6f` (`ri0-renaming-invariance-protocol-v1`).

Frozen evaluator: `4460def` (`ri0-renaming-invariance-frozen-v1`).

One-shot E2B worker: `iys4xr05vtz7u3cdcz1o4`.

## Result

- rows: `28`;
- exact same-mechanics replay: `28/28`;
- natural quiescence: `28/28`;
- pending activity: zero everywhere;
- pending loads: zero everywhere;
- within-mechanics renaming equality: `18/28` rows;
- Reference/Production complete equality: `0/14` cases;
- full RI0 acceptance: `0/28` rows.

## Decisive renaming failures

### Numeric physical identity is causal

In the two-source same-tick collision, identity numbering produces one target
firing. These three bijections produce zero target firings under both mechanics:

- reverse all physical numeric identities;
- deterministic random physical-identity bijection;
- swap only the two competing source physical identities.

Swapping only unrelated physical identities leaves the firing intact. Reversing
internal CELL insertion while retaining physical identities also leaves this
world intact.

Thus the failure is localized to the numeric `origin_physical` ordering key:
renaming the two simultaneous causal sources changes which signed arrival is
processed first and changes whether the target fires.

### ARROW allocation name is causal

In the single-source parallel-arrow collision, identity insertion schedules
`+1` before `-1` and the target fires once. Reversing only ARROW insertion
causes `-1` to precede `+1` and the target never fires, under both mechanics.

The logical graph, source, target, delays, phases, couplings, initial input, and
serial-generation procedure are otherwise unchanged. Arrow allocation order
therefore leaks through serial tie-breaking into physical behavior.

Reversing internal CELL insertion in the parallel-arrow world also changes the
normalized complete observation while preserving the target firing count; this
is an additional renaming failure whose first differing physical transition
was not serialized by v1.

## Mechanics comparison

Reference and Production normalized transition hashes agree row-by-row, but
their frozen complete observations do not because PhysicalWork differs in all
14 cases (for example `37` vs `30` in the baseline two-source world). This is a
second representation-contract failure, separate from the decisive
within-mechanics renaming negatives.

## Classification

RI0 is a real negative:

> Arbitrary numeric physical identity and ARROW allocation order are currently
> observable organism physics for simultaneous same-phase arrivals.

No scheduler rule, serial rule, simultaneous-event law, RS2 predicate, or core
source was changed. RS2 v5, CE1, FD2, ARC, authority, oracle status, and
`arch.md` remain blocked and unchanged.

## Artifact hashes

- matrix:
  `3bb2199d305337be2c070e46295965b093b68815d0eed0ce20e42dda7d8248f5`;
- report:
  `184cd8d39ae08cbed9a1cd6b6cce19374193e6a18b57736c997691436e304f13`.

