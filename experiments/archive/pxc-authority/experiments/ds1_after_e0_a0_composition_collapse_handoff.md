# Frozen DS1 after DS-E0 + DS-A0 composition collapse handoff

Protocol: `ds1-after-e0-a0-composition-retry-v2`.

Outcome: **CUMULATIVE DS1 DEVELOPMENT COLLAPSE AT 4. opaque alternatives >1
are one-to-one real roots at the DS1 choice surface**.

This is a development-only first-collapse freeze. It is not definitive, not
claim eligible, not a cumulative scientific result, and does not create M1.

## Ordered outcome

| Stage | Status |
|---|---|
| 0 exact lineage and immutable hashes | READY |
| 1 actual DS-E0 learned temporary relational structure | READY |
| 2 format-only E0 serialization and frozen DS1 read-only consumption | READY |
| 3 actual DS-A0 plastic route formation before bridge | READY |
| 4 opaque alternatives >1, one-to-one real roots at choice surface | **COLLAPSE: exact E0 episode yields one root/handle/effect** |
| 5 frozen DS1 opaque choice | BLOCKED |
| 6 selected ordinary CELL/ARROW/SPIKE execution | BLOCKED |
| 7 natural parent-substrate consequence visibility | BLOCKED |
| 8 unchanged `apply_consequence` update | BLOCKED |
| 9 held-out reconstruction/functional controls | BLOCKED |
| 10 invalidation/reopening/reconsolidation | BLOCKED |
| 11 naturally available recursion/economics | BLOCKED |

The corrected cumulative harness uses the same actual E0 raw episode and
formed `EventRelations` for E0-B and A0. It does not use the independent A0
fixture on the primary path. Across every seed, three exact E0 support exports
teach one generic A0 template; the exact target export then installs one live
three-CELL/two-ARROW route and exposes one opaque handle. That advances the
prior stage-4 dependency from zero cumulative actions to one actual cumulative
action, but still cannot satisfy the preregistered greater-than-one alternative
requirement. No second route is synthesized.

## Mechanically derived call/path inventory

```text
frozen choose definitions                         1
composition source edges into frozen choose       1
runtime frozen choose calls                       0
choice-to-execute_handle source edges              1
runtime selected execution calls                  0
post-execution consequence-observation edges       0
parent execute_handle consequence paths            0
runtime consequence visibility events              0
frozen apply_consequence definitions               1
composition apply_consequence call edges           0
runtime DS1 apply/update calls                      0
```

The source edges are present for legal later-stage wiring but are unreachable
after stage 4. Consequence visibility and apply/update are separate inventories
and separate runtime counters. Mutation tests independently detect an added
observer edge, parent consequence site, or update call.

## Validation

The clean amended implementation snapshot
`15d5e271889c3596f4311a597f03877cf4519a01` passed locally and in E2B:

```text
cargo fmt --all -- --check
cargo clippy --release --bin ds1_after_e0_a0_composition_retry -- -D warnings
cargo test --release --bin ds1_after_e0_a0_composition_retry   # 23 passed
cargo run --release --quiet --bin ds1_after_e0_a0_composition_retry -- --micro
cargo run --release --quiet --bin ds1_after_e0_a0_composition_retry -- --gate
```

MICRO seed 100 and GATE seeds 100..104 deterministically froze at stage 4 with
identical inventories. `--definitive` rejected in the runner with status 2
before any harness call. The results-tree digest was unchanged locally and
remotely at
`491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4`.
No file was written under `results/`. Broad legacy regression was not run
because frozen/shared mechanism bytes did not change.

E2B used only
`/Users/satya/.cache/truelearner/ds1-after-e0-a0-cumulative-e2b.json`.
Persistent sandbox `i0r9yemltx9u0cut1mggl` was reused, reset to an 86,400
second timeout, never killed, and left running. Remote frozen hashes, format,
strict release Clippy, 23 focused tests, MICRO, GATE, definitive rejection, and
results digest preservation all passed.

## Freeze and authority

- preregistration: `33f39adf28e3b585475ec608e3aaf008889a3db1` /
  `ds1-after-e0-a0-composition-retry-protocol`;
- implementation: `a990690067413b4ff45d1d31ec7e307265eee612` /
  `ds1-after-e0-a0-composition-retry-implementation`;
- validated accounting amendment: `15d5e271889c3596f4311a597f03877cf4519a01` /
  `ds1-after-e0-a0-composition-retry-implementation-amendment`;
- collapse freeze tag: `ds1-after-e0-a0-composition-retry-collapse-handoff`
  (the commit containing this handoff).

**M0 `1d74c0e` remains authoritative. M1 is absent. Cumulative definitive is
not authorized or run.**
