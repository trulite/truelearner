# TC-DS0 old-window characterization result v2

Status: characterization positive. No replacement mechanism or parameter was
selected. TC-DS1 requires a separately frozen protocol.

TC-DS0 v1 remains immutable negative on raw cross-mechanics checkpoint hashes.
Its diagnostic established that the difference was only zero-state CELL
`last_update_tick` bookkeeping. V2 changed only that comparison boundary.

## Matrix result

```text
physical cases                         960 / 960
Reference/Production mechanics rows  1920 / 1920
exact physical pairs                   960 / 960
quiescent and independently replayed  1920 / 1920
fresh-worker artifact replay              exact
```

The active core remained byte-identical:

```text
d49a6b98081cee65c3e7a5f64e9cf6356fa06ef456a9cf78a41d0cd1187bcd58
```

## Rectangular geometry

Across every pressure phase and both initial resistances:

```text
return delay 0,1,2,3,4  -> credit
return delay 5,8,12     -> zero credit
```

Prompt-return-shaped modulation credited `100/160` cases. Physically unrelated
modulation credited the same `100/160` cases, and its update count matched the
prompt-shaped row in `160/160` matched geometries.

This makes the retained cliff explicit:

```text
tick <= traversal + 4  -> eligible
tick >  traversal + 4  -> ineligible
```

Pressure phase changed later resistance trajectories but never softened or
shifted the credit boundary.

## Attribution and participation controls

- Nearby Drive produced zero candidate updates.
- A never-traversed observed path received zero updates in `160/160` cases.
- In the two-path fixture, one Modulatory arrival strengthened both recently
  traversed outgoing paths in `100/160` cases.
- Actual same-path repetition emitted a new rectangular deadline whenever the
  path remained live and the source could fire. It produced no durable update
  without Modulation. Weak resistance-1 paths ultimately disappeared after
  unsupported expiry; stable paths could renew again after longer delays.

Thus participation gating still works, but temporal attribution is both
rectangular and source-local: the current law cannot distinguish return-shaped
Modulation from an unrelated Modulatory route reaching the same source, and
one arrival can update multiple simultaneously eligible outgoing paths.

These observations are characterization, not candidate-selection predicates.

## Mechanics and replay

Reference and Production matched on eligibility emissions, per-tick path
trajectories, plastic updates, proposals, deallocations, final path state,
physical-transition hashes, durable-body hashes, clock, pressure phase, and
quiescence.

Both raw checkpoint hashes remain serialized. Each checkpoint replayed exactly
under its originating mechanics. They are not required to match across
mechanics because inactive zero-state decay bookkeeping is intentionally eager
in Reference and sparse in Production.

The primary and replay workers produced byte-identical artifacts:

```text
matrix.csv
8ecab66735025ca566f7d80652ea83d11c23447ac71d87f09e42b6b2bdb33b8c

report.md
3104319e8928173c15bea577744ca3973bd0421854fb37d47df5b6858b48e5da

SHA256SUMS
7963284e8cada1a1ca86a5b937ab4d133795a1e4fc443c22d8e288de78d1cf82
```

## E2B provenance

- targeted evaluator validation: `i66lao4w60le4hybt1bov`;
- immutable v1 negative: `i48pvb86s9klz2ux6gbwv`;
- checkpoint diagnostic: `i43hr6y5u0a6tiwk5xwdg`;
- v2 characterization: `ik3042iwgdvngh6u2dukh`;
- fresh exact artifact replay: `ispcumng3bvbk7zzolc0d`.

The archive worker could not resolve historical commit `0033ab2` for the
audit's redundant `git diff` check because `git archive` contains no history.
The preregistered exact core SHA-256 check passed and directly establishes the
required byte identity.

## Boundary

ARC A3-A5 did not run. ARC A2 did not rerun. No runtime, constant, candidate
law, authority, oracle, or `arch.md` change occurred.

