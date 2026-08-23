# CJ0-D local-subunit PROBE v4 absent-expectation protocol

Status: **PREREGISTERED MECHANICAL RETRY; PROBE v4 UNSPENT**.

PROBE v3 is frozen at commit `da8ba01`, tag
`cj0-d-local-subunit-probe-v3-frozen-negative`. Ten of eleven rows passed. The
absent-opportunity row physically passed with zero traversal, integration,
CELL firing, return, and outward crossing, but the evaluator's match expression
fell through to the blocked-return expectation of four integrations.

The sole authorized correction inserts this arm before the generic joint arm:

```text
Joint when weak opportunity is absent or spacing exceeds radius => 0
```

Fresh PROBE paths/report label are `v4`. No other source line, law byte,
constant, fixture, expectation, topology, schedule, or pass clause may change.
All pre-evidence validations and the v2 timing-floor rules remain exact. Any
failure is frozen and ends PROBE development.

This development protocol creates no surface beyond PROBE/MICRO/GATE and
cannot advance the lane past GATE.

