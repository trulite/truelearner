# CJ0-NOT-2 temporal-absence PROBE implementation audit

Status: **IMPLEMENTATION FROZEN; PROBE EVIDENCE UNSPENT; PX2 REMAINS AUTHORITATIVE**.

## Frozen inputs and source

| item | SHA-256 |
|---|---|
| authoritative PX0 substrate source | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| authoritative PX2 definitive CSV | `921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18` |
| NOT-2 PROBE protocol | `b736efa9203740ea0932c7a2e997fc7fd2b583d2471f9856ff6061e8679ca498` |
| NOT-2 PROBE runner | `8b30f99954eb3f5ca9f9e7887ba6e11053f64d55c18349c392ab351ab9e12205` |

No path from the authoritative PX2 tree is changed.

## Mechanism and information-flow audit

The runner imports only the frozen public `PlasticSubstrate` API. All three
external inputs are positive firings. Existing signed ARROW coupling supplies
trigger `+2`, B `-2`, and closure `+2` activity to the same ordinary transient
CELL; threshold firing supplies the output path.

Trigger propagation drains first. The runner records the complete fingerprint
before trigger and after trigger reaches the transient CELL, before B or
closure is entered. It then supplies the same physical closure in every case.
The topology does not depend on measured state or output. Only the
preregistered physical B arrival time/absence changes.

There is no absence symbol, timeout label, outcome-selected branch, semantic
adapter, replacement CELL/ARROW/SPIKE type, or added substrate variable.
Scenario labels and pass/fail interpretation remain evaluator-only. The
substrate itself contains only its already-authoritative transient CELL state,
time, pressure, firing, arrows, and queue.

## Layout, lifecycle, work, and storage audit

Normal and mirrored fixtures reverse positions and allocation order and use
fresh disjoint identities. The sweep covers B at tick `1`, before closure at
tick `2`, and after closure at tick `3`, plus absent, zero-resistance blocked,
and ordinary-pressure-staled B paths. Every row serializes both quiescence
stages, physical deallocation, work, persistent bytes, complete/permanent
fingerprints, and exact replay.

## Validation

Before evidence, all passed:

```text
cargo fmt --all -- --check
cargo check -p px0-physical-correspondence --example cj0_not2_temporal_absence_probe
cargo clippy -p px0-physical-correspondence --example cj0_not2_temporal_absence_probe -- -D warnings
cargo run --release -p px0-physical-correspondence --example cj0_not2_temporal_absence_probe -- --preflight
git diff --check
```

Preflight entered no cell and final/staging artifacts were absent. Definitive
evidence, PX3 interpretation, and authority advancement remain absent.
