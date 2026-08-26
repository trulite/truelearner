# CL0 ordinary CELL lifetime and reuse handoff v1

Status: Gates 1–8 positive; Gate 9 negative; no development readiness.

## Outcome

```text
ordinary CELL resistance
    + local elapsed time
    -> proportional phase-free lifetime                         PASS

resistance reaches zero
    -> non-live
    -> generation advances
    -> fresh CellId reuses resident CellSlot                    PASS

old incoming/outgoing references
    -> immediately stale
    -> incident ARROWs decay independently                      PASS

qualified consequence
    -> accepted ARROW resistance can increase
    -> accepted CELL resistance cannot increase                 STOP NEGATIVE
```

CL0 solved the CV0 orphan-reclamation prerequisite mechanically, but it did
not solve useful contact retention. Running CV0 now would make supported and
unsupported contact CELLs share the same finite lifetime.

## Frozen lineage

- CV0 parent/result: `f5c7bdd`, tag
  `cv0-bounded-local-contact-genesis-gate-d-negative-v1`;
- protocol: `3e5367b`, tag
  `cl0-ordinary-cell-lifetime-reuse-protocol-v1`;
- implementation/evaluator draft: `3626a9c`;
- remote formatting: `a7e83c9`;
- targeted compile-hygiene correction: `9a12559`;
- frozen candidate: `4cc8fe4`, tag
  `cl0-ordinary-cell-lifetime-reuse-frozen-v1`;
- Gate 9 audit-only correction: `071a666`.

## E2B provenance

- preserved reusable Rust compilation worker: `ifk44bxtlfjlci644r63m`;
- sole Gates 1–8 matrix worker: `iw23q00zu5dwzs6wicbrf`;
- successful Gate 9 worker: `ine5gylab32gb3rkk70xv`;
- discarded pre-evidence Gate 9 plumbing failure: `i5swm36phkm9kca2aut3m`.

## Boundary

Do not resume CV0 yet and do not make CELL survival depend on sign, contact
role, degree, firing, participation, or evaluator usefulness.

A successor may independently ask whether the already-established local
coincidence:

```text
qualified Modulation
    x
ordinary local participation
```

can legitimately consolidate ordinary CELL resistance as well as ARROW
resistance, and under what physical locality. CL0 does not choose that law.

CV0, SV1, RS2, FD2, ARC, authority, the oracle, and `arch.md` remain unchanged.
