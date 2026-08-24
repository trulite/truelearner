# PX3 integrated MICRO reversal implementation audit v1

Status: **FROZEN; E2B PREFLIGHT PASSED; EVIDENCE UNSPENT**.

- source commit: `e4306e36efc4f72e7e8df6fd6e3d09b2d30255a1`;
- manifest SHA-256:
  `71125806d683b7128ee0bccb55eef86c43c3d544f302bf7aed23f2026b2ecfb2`;
- source SHA-256:
  `aa8c4769ec6bb90d5724081fa007bf30b5700e5f49967668bb0195b51ccdc68d`;
- protocol SHA-256:
  `3d5394bcddd50a3ea8f785da50e3bf23eecbd9d88e88912c41cb1d1eded24e8e`;
- execution-protocol SHA-256:
  `9776b6cbcbc3de54042b9d022995410632a377c6fb224a16d97c983718def922`.

Persistent E2B sandbox `i6x9gykt9tvp6xfz5z8ra` passed formatting, release
check, 2/2 static tests, strict Clippy and the non-propagating `--preflight`.
Result/staging and recursive GATE surfaces were absent. The evidence marker was
not emitted and no MICRO world was constructed or propagated.

Static inspection confirms:

- zero P->effect candidates exist at construction;
- every P has exactly one distance-one effect opportunity;
- the same external unit background reaches all six P cells per experience;
- O->P coupling and background coupling are both one, while P threshold is two;
- only an externally completed P firing invokes unchanged generic proposal;
- all attributed M->P returns have coupling one;
- no effect/relay-to-P edge exists;
- initial and reversed worlds share byte-identical topology and schedules;
- candidate identity, generation, liveness, resistance, crossings, impulses,
  proposals, effects, held-out behavior and replay are independently recorded.

Only the frozen `--micro` command may spend development evidence once.
