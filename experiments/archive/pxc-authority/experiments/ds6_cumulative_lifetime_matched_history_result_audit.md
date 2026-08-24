# DS6 matched-history lifetime result audit

Outcome: **DEVELOPMENT POSITIVE; DS6 GATE ELIGIBLE; M3 STILL AUTHORITATIVE**.

Protocol commit/tag:
`be6cec9e5d5adb9d667a5ef1bfcd208c6eda89b5` /
`ds6-cumulative-lifetime-matched-history-protocol-v2`.

Implementation commit/tag:
`aad0f2b52bfffdcf991301dc03b2568de96e005b` /
`ds6-cumulative-lifetime-matched-history-implementation`.

Frozen hashes:

- implementation:
  `1aab138746338f19a50ba72c1a2f9c61ca3c4b043ce488433966819ee86ed99c`;
- runner:
  `64ec38b9cd73b19810090979306a2750cdd19626d2a7630e4f17ab877881a10e`;
- result:
  `908413209b76d20e8aa38b64d7a8436d0e91ce4f991240ee9206554f2934d1b7`.

The complete matrix ran once in E2B sandbox `iyrkw7af5qpmwwfmq3bwm` and wrote
its create-new artifact before exit.

## Frozen outcome

Both fresh seeds `109000` and `110000` produced the exact same lawful ledger:

```text
recurrence count       1   2   4   8
strength after four
matched pressure       0   0   2   9

pressure updates       2   4   8  12
strength after six
matched recurrences    8   6   2   0

8 uses + 8 pressure    5
2 uses + 2 pressure    1
reuse after gap       +2
```

Fresh identity/layout, one-off removal and reacquisition, stale-path blocking,
no-pressure discrimination, persistent allocation accounting, and exact
duplicate controls all passed.

## Development claim

The byte-identical scalar mechanism produced different physical lifetimes from
different histories without a supplied lifetime variable or class:

> Past recurrence/use accumulates resistance to ordinary erasure pressure;
> continued non-use spends that resistance; reuse after a gap restores it
> through the same local update path.

The crossed cell shows both variables combine in the same state rather than
selecting independent modes. No `TEMPORARY`, `PERMANENT`, `TTL`, expiry class,
evaluator delete, retention oracle, future-use label, or task boundary entered
the lifecycle.

## Boundary

This is development evidence, not DS6 readiness or authority. GATE must test
the unchanged mechanism across more seeds, recurrence/pressure values,
interleavings, capacity loads, contradiction histories, and exact cumulative
M3 behavior. M3 remains authoritative and M4 remains absent.

