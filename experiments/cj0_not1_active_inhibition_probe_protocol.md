# CJ0-NOT-1 active-inhibition PROBE protocol

Status: **PREREGISTERED; PROBE EVIDENCE UNSPENT; PX2 REMAINS AUTHORITATIVE**.

## Exact starting point

- authoritative PX2 commit: `2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`;
- authoritative tree: `3e69c15c5a9f7259d8617aa23ffb9083064f53a1`;
- PX0 substrate source SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`;
- PX2 definitive CSV SHA-256:
  `921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18`.

No pre-existing byte may change. PX3 records and negatives are outside this
diagnostic and may not be reinterpreted.

## Narrow question

Can an ordinary, already-lawful negative-coupling ARROW carry activity from A
into the same transient CELL integration site used by a positive B path, so
that:

1. A absent -> B path fires and reaches its output;
2. A timely -> B path does not fire or reach its output;
3. A too late -> the earlier B output remains; there is no retroactive effect?

This is a test of existing signed coupling, not a proposal for a NOT primitive.
If signed coupling cannot satisfy the conjunction, NOT-1 freezes negative and
no inhibitory law may be added on this branch.

## Fixed PROBE matrix

Use four fresh cells in each fixture: ordinary A and B arrival cells, one
integration cell, and one output cell. A and B each fire only from positive
external activity. Existing ARROW coupling carries A as `-2` and B as `+2`
to the integration cell whose threshold is `2`; its ordinary output coupling
is `+1`.

Run both normal and mirrored physical layout with disjoint fresh identities.
For each layout execute from a common complete-state snapshot:

- `A-absent`: only B arrives;
- `A-timely`: A and B arrive so their signed impulses reach the integration
  cell at the same tick, with physical phase/order varied across layout;
- `A-late`: A's impulse reaches only after B has fired and emitted output;
- `A-blocked`: A is active but its ARROW is born non-live (`resistance = 0`);
- `A-stale`: A emits onto a weak long-delay ARROW that ordinary pressure
  deallocates before delivery.

Every fixture is cloned before execution and repeated exactly. Serialize A/B,
integration, and output firing counts; crossing counts; output tick; negative
arrival tick; quiescence; complete/permanent fingerprints; work ledger;
persistent byte count; and duplicate equality.

## PROBE pass conjunction

- A-absent, A-blocked, and A-stale: B integration and output each fire once;
- A-timely: A and B fire, negative activity physically arrives, and neither
  integration nor output fires;
- A-late: integration/output fire once strictly before negative arrival;
- every run naturally quiesces and exact replay matches;
- both mirrors agree in outcome while identities, positions, insertion order,
  and timing order differ;
- frozen source/evidence hashes remain exact and no pre-existing tracked path
  differs from the PX2 tree.

The PROBE is classification-only. PASS permits a fresh definitive
preregistration; FAIL freezes NOT-1 at this boundary.

## Evidence discipline

The implementation must support a no-cell `--preflight`, refuse without
`--probe`, and atomically publish only:

```text
results/cj0_not1_active_inhibition_probe_v1.csv
results/cj0_not1_active_inhibition_probe_v1.md
```

The exact command may execute once after its source and audit are committed and
tagged:

```text
cargo run --release -p px0-physical-correspondence \
  --example cj0_not1_active_inhibition_probe -- --probe
```

