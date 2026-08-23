# CJ0 Arm A coincidence-threshold CELL PROBE v3 staging-retry protocol

Status: **PREREGISTERED FINAL MECHANICAL RETRY; EVIDENCE UNSPENT**.

V1 and v2 are immutable invalid executions at tags
`cj0-a-coincidence-threshold-probe-v1-invalid` and
`cj0-a-coincidence-threshold-probe-v2-invalid`. Neither produced an artifact or
scientific classification.

V3 retains every v2 physical identity, parameter, threshold, topology,
timestamp, marginal, world, control, ordered claim, and stop rule. Its only
change is burst staging: both external occurrence sets registered for ticks
`t,t+1` are entered into the pending physical queue before one call to ordinary
propagation. The substrate, not the evaluator, orders these external arrivals
with all proposed-ARROW traversal, CELL firing, return, effect, decay,
refractory, pressure, and generation activity. The complete finite queue is
serialized and must naturally drain before the next burst.

Later supported single uses remain one entry set followed by one propagation.
Held-out one-shot use is unchanged. The repeated-singleton self-evidence
control uses the identical two-occurrence burst helper as genuine weak
bootstrap. Initial A+B/C+D and reversal A+D/C+B therefore receive exactly the
same physical burst staging; no world or expected answer selects staging.

Fresh v3 result paths replace v2 paths. The sole execution after frozen
implementation is:

```text
cargo run --release -p px0-physical-correspondence \
  --example cj0_a_coincidence_threshold -- --probe-v3
```

It emits one `CJ0_A_COINCIDENCE_THRESHOLD_PROBE_V3_EVIDENCE_SPENT` marker and
atomically publishes `results/cj0_a_coincidence_threshold_probe_v3.csv` and
`.md`. A material failure is terminal and freezes CJ-A negative; no MICRO,
GATE, recursion, OR/timing, definitive, authority, rescue, or further retry is
permitted.
