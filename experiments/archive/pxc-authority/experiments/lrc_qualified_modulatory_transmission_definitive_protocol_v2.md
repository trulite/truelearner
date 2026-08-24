# LR-C qualified modulatory transmission definitive protocol v2

Status: **PREREGISTERED; V2 AUTHORITY CELLS UNTOUCHED**.

V1 is non-authoritative because implementation tests executed its later
definitive namespaces. V2 retains the frozen candidate law and exact 31-world,
12-claim matrix while enforcing disjoint development and authority identities.

## Frozen inputs

- candidate law: `967ab7d` / `lrc-qualified-modulatory-candidate-law-v1`;
- active law SHA-256:
  `7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10`;
- v1 collapse: `d828713` / `lrc-definitive-v1-namespace-contamination`;
- v1 protocol supplies the unchanged law claim, twelve primary worlds,
  nineteen PX0--PX2 conformance worlds, observables, anti-cheat rules and
  `496/496`, `5,952/5,952` conjunctive threshold.

## Disjoint identities

Development-only seeds, executable by tests:

```text
6101, 6113, 6121, 6131, 6143, 6151, 6163, 6173,
6197, 6203, 6211, 6221, 6229, 6247, 6257, 6263
```

Definitive v2 authority seeds:

```text
7103, 7109, 7121, 7127, 7129, 7151, 7159, 7177,
7187, 7193, 7207, 7211, 7213, 7219, 7229, 7237
```

V2 namespace base is `0x7_4400_0000_0000`; suite/world strides are unchanged.
The two 496-namespace sets must be disjoint from one another and from v1.

## Execution firewall

- Unit tests and development serialization call the physical runner only with
  development seeds.
- Preflight validates frozen hashes, source audit, cardinality, uniqueness,
  disjointness and absent artifacts. It may construct numeric authority
  namespaces but may not call `run`, `replay`, `build_pair` or `propagate` with
  an authority seed.
- The harness carries an in-process execution guard. Physical execution of an
  authority seed is refused unless `--definitive-v2` has installed the sole
  authority token.
- The frozen `--definitive-v2` command runs all 496 authority cells and their
  exact duplicates once, then publishes v2 artifacts atomically.

No rescue or rerun is permitted. A positive requires every unchanged v1
physical predicate plus exact replay, natural quiescence, 45-column
serialization and `5,952/5,952` claims. PX3 remains outside the workflow.
