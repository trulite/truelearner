# PX1-PT1 attributed-margin stability MICRO implementation audit

Status: **IMPLEMENTATION FROZEN; MICRO EVIDENCE UNSPENT; PX1 NON-AUTHORITATIVE**.

## Frozen implementation

- source:
  `crates/px0-physical-correspondence/examples/px1_pt1_attributed_margin_stability.rs`;
- SHA-256:
  `8b094e5ac9dca5c41baf20bb2791da1bcdd0406fbfeb1946dc03d240c4ad0c38`;
- positive reserve-3 PROBE source SHA-256:
  `1bcfe295fb8989d1c6489e7c255b128912b98afef8b550eea15b5eaf0e06b443`;
- preregistered MICRO protocol SHA-256:
  `3c3e0d968e247988cb34f5ecac602c2c1af634758060afdf577b6aad927df829`.

The reserve `3`, margin threshold `4`, thresholds, delays, couplings, PX0
acquisition law, effect-triggered physical coincidence cells, and ordinary
return law are unchanged from the positive PROBE. The implementation adds only
the six preregistered physical worlds, fresh mirrored/reversed layouts, and
independent evaluator-side serialization.

## Independently serialized stages

For both branches, during training and held-out execution, the frozen runner
records branch firing, outlet firing, trace arrival, trace firing, local return,
continuation resistance, and outward effect. It separately records source
refiring, quiescence, post-gap effects, work, complete fingerprint, and exact
duplicate replay.

The controls alter only ordinary physical activity or topology:

- no support supplies neither branch-completing activity;
- blocked return removes outlet-to-hub propagation while retaining actual
  branch/outlet participation and direct trace arrival;
- return without effect injects one anonymous physical arrival into the hub
  while neither branch/outlet participates;
- joint participation physically supports both continuations;
- transfer cells use fresh namespaces, mirrored placement, and reversed
  allocation order.

No organism-visible branch label, selected-route field, provenance metadata,
semantic role, return ownership, or evaluator-selected local credit is added.

## Validation before evidence

- `cargo fmt --all -- --check`: pass after formatting;
- focused example compilation: pass;
- strict focused Clippy (`-D warnings`): pass;
- focused substrate unit test: `1/1` pass;
- `--definitive` refusal: exit `2` before the harness;
- MICRO result paths absent;
- all source-audit hashes exact.

The only authorized development command is the write-once `--micro` surface.
GATE and definitive execution remain forbidden.
