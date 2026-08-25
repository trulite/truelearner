# CK0 junction checkpoint integration handoff v1

CK0 v1 is frozen negative at `184/216`, but it cleared the original structural
blocker: a dead J0 junction with nonzero dormant CELL resistance now restores
successfully and remains dead, generation-safe, and stale-reference inert.

The remaining failures are at the evaluator/continuation measurement boundary:

```text
24 clauses
    raw Reference/Production checkpoint hashes compared as physics

8 clauses
    composite continuation mismatch not serialized by component
    and full legacy Work compared instead of PhysicalWork
```

All `40/40` replays and quiescence checks passed. Every direct J0 liveness,
topology, reuse, and stale-reference assertion passed.

No J0/CV0 replay or RS2 rerun occurred because CK0 was not completely
positive. The exact record is
`academy/docs/ck0_junction_checkpoint_integration_negative_v1.md`.

Any successor should first preregister a read-only continuation diagnostic
that serializes expected and restored trace, PhysicalWork fields, private
legacy total only as diagnostic accounting, tick, body, pending activity, and
quiescence separately. It must not change the frozen CK0 runtime candidate or
checkpoint representation while diagnosing the eight composite failures.
