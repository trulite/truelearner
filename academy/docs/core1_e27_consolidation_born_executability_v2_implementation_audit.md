# CORE1-E27 — Compact v2 Implementation Audit

## Freeze

- v1 abort and v2 protocol: `252c86f`;
- compact fixture/evaluator: `76c8343`;
- E27 candidate physics: byte-identical to `8ec7ae7`;
- frozen v2 evidence: not yet spent at the time of this audit.

## Harness correction

Academy now exposes a CORE1-only spatial fixture constructor with an explicit
validated context count. It calls the same body builder used by the 1,024-
context constructor and changes no cell, arrow, propagation, learning, or
observation law.

The v2 evaluator requests five compartments and selects raster contexts
`0..4`. It runs two arms:

```text
B  E26 frozen stack, E27 disabled
X  same stack, E27 enabled
```

The full regimen, recovery, probes, actions, consequence timing, E22, PQLC,
E24, G+W, and E26 checks are unchanged. Reference, exact Reference replay, and
Production remain required across eight roots.

## Equivalence and rejection audit

- all five context compartments retain the complete four-motor spatial body;
- selected context indices carry no action or usefulness semantics;
- B must reproduce E26's full `0/8` autonomous result at `8/8` exactness before
  X is interpreted;
- X receives no route, action, consequence, or probe information;
- E27 core physics is byte-identical to its v1 audited implementation;
- E14, E16, E22, E24, E25, E26, and E27-v1 evaluators are unchanged;
- v1 partial results were deleted and cannot be mistaken for v2 evidence;
- v2 uses new roots `95000000..95000007` and a new result directory.

## Runtime control

The preregistered off-matrix root `94999999` ran both full arms once under
Reference:

```text
internal evaluator time   62 ms
wall time                 0.69 s
baseline                  pass
candidate                 pass
```

This replaces the invalid v1 shape, which global-scanned a replicated 1,024-
context body for every execution and was manually stopped after eighteen
minutes.

## Other controls

- E27 ordinary-source two-arrow execution control: pass;
- unchanged E26 creation control: pass;
- Academy blocked-return control: pass;
- strict release Clippy for the v2 binary: pass;
- release build: pass;
- non-evidence observer check: pass.

## SHA-256

- protocol:
  `08e34b14908b96b11184eea3d8c6a2636bf0b8c0e59b9bf7cfdfbc2b74b556e0`;
- unchanged core candidate:
  `778782ea97b3dd0c65e5a55a72e0fec8e97329c814e742f88991fdf3c8cb949b`;
- Academy compact-fixture surface:
  `01b842c23cf12ae4d2189507b55e5e3b4ddc8cda753f0cf7177dd9cef667cc94`;
- v2 evaluator:
  `bc4bf35263673adf6de8fbd51ef31d90799fe43daabd2297967ea3e479b26f2a`.
