# CR0 v2 static-audit repair protocol v1

Status: frozen after the immutable positive v2 matrix and before audit repair.

Parent: `cr0-coupling-necessity-v2-evidence-positive-v1` (`9e72425`).

## Sole repair

In `experiments/tools/audit_cr0_coupling_necessity_v1.sh`, replace the FD1
anchor regex that requires age 5 and absolute tick 5:

```text
after_consequence@5/5:1/4/
```

with a regex that requires age 5, accepts the serialized phase-shifted absolute
tick, and still requires live resistance 4:

```text
after_consequence@5/[0-9]+:1/4/
```

The expected count remains exactly forty.

No other script, evaluator, evidence, source hash, physical-law, protocol,
result, or claim byte may change before the repaired audit runs.

## Execution

Run only the repaired static audit in E2B. Do not compile or execute Rust and
do not rerun either CR0 matrix.

If the audit passes, a result audit may accept the already-frozen v2 physical
evidence. Any other failure stops negative.
