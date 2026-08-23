# PX8 physical closure-emission PROBE v2 mechanical amendment

Status: **PREREGISTERED; RETRY EVIDENCE UNSPENT; PHYSICS UNCHANGED**.

## Frozen reason

PROBE v1 is an immutable invalid execution at commit
`6546327d85b369d8fdcf4d76205f2a3c85e78565`, tag
`px8-physical-closure-emission-probe-v1-invalid`.

| frozen v1 artifact | SHA-256 |
|---|---|
| raw CSV | `d33963361b6c2a3f40a9eaa172d296dcdbd8f668afbdd1e26f136e00028e629f` |
| raw report | `870691020517c5d8232de24067fa3bd50581fb2c869c3620bb1474bed92ce44b` |
| executed source | `3529ad6ff9a81740a2a1084b973e98a3c9d92a6c58ef952540b29c2c374ae332` |
| invalid-execution audit | `504812e1777602ccda30ae8d51b11dae308b3772cdb8483a37b65eb403a66b87` |

The only defect was a mismatch between executed and serialized unrelated
activity count. It did not expose a missing physical edge or a scientific
alternative.

## Sole correction

Replace the unrelated-arrival load table `0,4,12,24` with the exact strictly
positive table `4,8,12,24`, and execute exactly the serialized count without a
minimum or fallback. This corrected table applies to the fresh PROBE v2 and
the still-unspent MICRO and GATE matrices.

PROBE v2 uses fresh namespace base `0x8_8100_0000` and fresh atomic paths:

```text
results/px8_physical_closure_emission_probe_v2.csv
results/px8_physical_closure_emission_probe_v2.md
```

The v1 paths and namespace remain spent and may not be read as a prerequisite.
MICRO must require a frozen positive v2 report.

No CELL, ARROW, SPIKE, threshold, coupling, delay, phase, resistance, pressure,
condition, expectation, clause, matrix size, evaluator boundary, or organism
law may change. The retained substrate source hash remains
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

## Execution and stopping rule

After a separately frozen corrected implementation passes the original
preflight and focused validation, run PROBE v2 exactly once. It must produce
`16/16` rows, `160/160` claims, and exact executed/serialized unrelated loads.
Any negative or further accounting mismatch is frozen and stops the lane.
A positive v2, not v1, makes MICRO eligible under the original protocol.
