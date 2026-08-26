# CL0 ordinary CELL lifetime and reuse implementation audit v1

Status: candidate frozen before functional evidence.

## Scope

The candidate is feature-gated as `cl0 = ["pd1"]`. Default organism behavior
remains unchanged. CL0 adds no CELL kind or semantic role.

The only candidate CELL law is local phase-free resistance decay. Each live
CELL holds a local decay load. Complete accepted decay intervals reduce
ordinary CELL resistance. At zero resistance, the CELL becomes non-live,
advances generation, clears transient activation, and exposes its resident
slot for reuse.

`add_cell` always allocates a fresh monotonically increasing `CellId`. If a
dead resident slot exists, it replaces that slot's occupant and inherits the
generation advanced by death. The old identity no longer resolves.

## Stale-reference mechanics

CL0 records both source and target CELL generations on each ordinary ARROW.
It also preserves target physical identity in queued SPIKE ordering so a dead
target need not remain resident merely to order an inert stale delivery.

ARROW traversal requires current endpoint identity, live state, and generation
to match the stored endpoint reference. CELL death does not inspect or delete
incident ARROWs. They remain non-executable stale structure until ordinary
ARROW decay removes them.

AoS and SoA stores carry the same CL0 fields. Live-checkpoint version 3 records
CELL local decay load and queued target physical identity. The CL0 evaluator
uses only the accepted Reference and selected Production mechanical configs.

## Frozen hashes

```text
core lib.rs  6d77cedc36b9bd82fe05481e48872287496163187363567fb9c83f6585799655
mechanics.rs 40f3ae01cae1afc3cf8c4481a41db5bcb8d508258db3ee701238e0199bf6b3a9
core Cargo   8bc8529c190ec653b378efe38359c0865dabca343d0a33d1c3ba53e67d5d9278
evaluator    e46d83ff13c2d21d9a25170935e7d9c69579531e1cac2f93d03b31649fced5a0
evaluator Cargo acd832da21a51cae81de42ff7daca24b85c4327705e2f4c15dbf2cf1930d5611
protocol     350309d41c98f587c1a8ca62260c6035561d4ef943256d6b1d299d230b3d7d7d
```

## Pre-evidence validation

Reusable E2B Rust worker `ifk44bxtlfjlci644r63m` performed only:

- remote rustfmt canonicalization and exact format check;
- targeted default core release check;
- targeted CL0 evaluator/core release check; and
- targeted strict Clippy with `-D warnings`.

All passed after one mechanical unused-parameter/import correction. No
functional world or evaluator command has run. No Rust command ran locally.

## Boundary

Gate 9 remains deliberately unresolved until after the one-shot Gates 1–8
matrix. The candidate adds no CELL-resistance strengthening path. CV0, SV1,
RS2, FD2, ARC, authority, the oracle, and `arch.md` remain unchanged.
