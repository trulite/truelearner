# PX2-H1 matched-schedule hysteresis diagnostic implementation audit

Status: **IMPLEMENTATION FROZEN; DIAGNOSTIC EVIDENCE UNSPENT**.

- source SHA-256:
  `59db8ed88e70e02570d45902c4f480bf9843e352004d47f032f1c805742c1adc`;
- frozen GATE source SHA-256:
  `8cdd72cff084c6a85d65629fd6504f5ca96f14d281a7a5ac518fd9c4754579ec`;
- protocol SHA-256:
  `e16afd6adae4477900bb457427f4335431628f6e4c42c1ab054c69081c1b217b`.

The diagnostic keeps one weak path per direction, removes distractor load, and
changes only the evaluator-controlled order of 12 forward and 12 reverse
physical participation histories. The four frozen layout/timing strata and all
PX0/PX1 substrate laws remain unchanged.

Per-experience output records continuation firing, trace firing, local return,
live state, resistance before/after, evaluator-inferred return gain and pressure
spend, first maturation, and deallocation. Inferred accounting is written only
after substrate execution and has no causal path into it.

Formatting, focused compilation, strict focused Clippy, focused substrate tests,
spent-GATE refusal, frozen hashes, fresh result-path absence, and diff checks
passed before evidence. No GATE or definitive rerun is possible through this
surface.
