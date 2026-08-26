# CK0 continuation negative diagnostic protocol v1

Status: frozen before any diagnostic evaluator or execution.

Parent: CK0 v1 frozen negative
`6dfec8aa932df0fe985cba66ab510b653a1316b3`
(`ck0-junction-checkpoint-integration-negative-v1`).

## Question

Which exact component caused the eight composite continuation failures in CK0
v1's live-pending and quiescent-future families?

## Frozen surface

This is a read-only diagnostic. The canonical runtime remains byte-identical
to CK0 v1:

`078cf11b3082cade5640b42abfcf52496faf3b36e0c0af10abefa7a9d75992de`.

The frozen CK0 evaluator and its evidence are not edited or relabeled.

The diagnostic reconstructs only the two failing CK0 families under the same
two roots and both Reference and Production mechanics. From each exact
checkpoint it runs three continuations:

1. uninterrupted body with physical tracing enabled;
2. restored body with its native/default observer setting;
3. an independently restored body after explicitly enabling the causally
   inert physical-trace observer.

All three receive identical future physical inputs.

## Serialized components

For every continuation the diagnostic records separately:

- ordered physical trace and hash;
- Drive deliveries;
- Modulatory deliveries;
- local-return updates;
- proposals and CELL proposals;
- ARROW and CELL deallocations;
- qualified-local traversals;
- `PhysicalWork` total;
- private legacy `Work::total()` as diagnostic mechanics accounting only;
- final physical tick;
- durable-body hash;
- live-checkpoint raw hash;
- natural quiescence.

It also records expected/actual equality independently for every field. No
composite pass predicate may hide the first differing component.

## Classification

Evaluator/observer defect:

- physical state, PhysicalWork, tick, body, and quiescence are exact;
- a missing trace is repaired solely by enabling the observer after restore;
- and/or only raw checkpoint bytes or legacy total differ.

Runtime/checkpoint negative:

- any durable body, physical work field, tick, pending/future physical event,
  quiescence, or observer-enabled trace genuinely differs after restore.

## Stop rule

After targeted formatting/check/strict Clippy, the diagnostic executes once in
a fresh E2B worker. It makes no CK0-positive claim.

Only an evaluator/observer-only classification permits a separately
preregistered CK0 v2 measurement correction. A physical difference blocks all
downstream work. J0, CV0, RS2, CE1, FD2, ARC, authority, oracle status, and
`arch.md` remain unchanged during this diagnostic.
