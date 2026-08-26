# SI0 simultaneous local incidence protocol v2

Status: frozen before any SI0 v2 observer change.

Parent: immutable SI0 v1 negative `293d202`.

## Scope

SI0 v2 is observer-only. It keeps byte-identical:

- runtime feature declaration: `d7d34bb477bc74657d8d1486d2c04fef759bb5f91ce5b08b805891f0bd75819c`;
- runtime physical law: `f19a89ac92c12cc4910047021c8bdedfa42b4c4dc2f5c3fcfa83e2a0b2a4c978`;
- runtime mechanics: `5f1172a0eaa0628d1775029c44e7a1b5bb2c4525c713b468f756a0705ef822a4`.

World construction, 10 families, 6 permutations, 120 rows, firing
predicates, Reference/Production configurations, replay, future-causal state,
durable body, PhysicalWork, clock, pending activity, and quiescence remain
unchanged from v1.

## Sole authorized repair

Replace the observer's sequential incidence-to-fire attachment with a
wave-level observation:

```text
Wave(tick, phase, causal_wave)
    incidences = set/multiset of local combined junction incidences
    fires      = set/multiset of junctions that fired from the wave
    effects    = remaining ordinary physical effects produced by the wave
```

All members are normalized by logical names only after execution. Recording
order, numeric CELL/ARROW/physical identities, insertion order, and serial may
not affect the normalized wave.

The observer must preserve genuine causal order between different wave keys.
In particular, a zero-delay consequence caused by wave `N` remains observable
only in wave `N+1`.

## Acceptance

- `120/120` rows pass.
- Every permutation equals the identity baseline after logical renaming.
- Reference equals Production on the frozen observation.
- Replay, expected firing, future-causal state, durable body, PhysicalWork,
  clock, pending activity, and natural quiescence remain exact.
- Runtime candidate hashes above remain exact before and after evidence.

One fresh E2B evidence execution is allowed after the observer and static gate
are frozen. Failure stops v2 without repair or rerun.

SI0 v2 does not perform architectural hardening, rerun RS2, or advance CE1,
FD2, ARC, authority, oracle status, or `arch.md`.
