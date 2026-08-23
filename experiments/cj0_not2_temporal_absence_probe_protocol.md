# CJ0-NOT-2 temporal-absence PROBE protocol

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

Using only existing physical time, transient CELL state, signed coupling,
ordinary firing/closure, pressure, and natural quiescence, can:

1. trigger activity leave temporary physical CELL state;
2. B activity arriving inside that physical window reduce or erase the state
   and prevent the alternative path;
3. when B remains absent through a later physical closure arrival, surviving
   state combine with that arrival, fire, and drive an output path?

There is no organism-visible `NOT-B`, timeout label, evaluator-selected branch,
or added persistent variable. The topology is identical across timing cases;
only ordinary B arrival time or absence differs. If the existing state cannot
satisfy the conjunction, NOT-2 freezes negative and no new absence law may be
added on this branch.

## Fixed PROBE matrix

Use five fresh cells in each fixture: trigger, B, physical-closure arrival,
transient integration, and output. Trigger, B, and closure cells receive only
positive external activity. Existing ARROW coupling delivers respectively
`+2`, `-2`, and `+2` into a transient cell with threshold `3`; its ordinary
output coupling is `+1`.

Trigger reaches transient state at tick `1`. Closure reaches it at tick `2`.
Run both normal and mirrored layouts with disjoint identities and these cases:

- B absent through closure;
- B reaches the transient cell in-window at tick `1`, after trigger by physical
  phase/order;
- B reaches at the closure tick before closure;
- B reaches after closure;
- B path blocked (`resistance = 0`);
- B path stale via weak long-delay coupling deallocated by ordinary pressure.

The timing sweep must include the ticks immediately before, at, and after
closure. The same topology and closure arrival are used in every case.
Every fixture is cloned before execution and repeated exactly. Serialize
trigger/B/closure/transient/output firing counts; signed arrival ticks and
impulses; output tick; quiescence; complete/permanent fingerprints; work;
persistent byte count; and duplicate equality.

## PROBE pass conjunction

- absent, blocked, stale, and after-closure B: transient/output each fire once;
- in-window B and B-before-closure-at-same-tick: trigger/B/closure fire,
  negative activity arrives no later than closure, and transient/output remain
  silent;
- after-closure B arrives strictly after the already-emitted output and has no
  retroactive effect;
- every run naturally quiesces and exact replay matches;
- both mirrors agree in outcome while identities, positions, insertion order,
  and timing order differ;
- frozen source/evidence hashes remain exact and no pre-existing tracked path
  differs from the PX2 tree.

The PROBE is classification-only. PASS permits a fresh definitive
preregistration; FAIL freezes NOT-2 at this boundary.

## Evidence discipline

The implementation must support a no-cell `--preflight`, refuse without
`--probe`, and atomically publish only:

```text
results/cj0_not2_temporal_absence_probe_v1.csv
results/cj0_not2_temporal_absence_probe_v1.md
```

The exact command may execute once after its source and audit are committed and
tagged:

```text
cargo run --release -p px0-physical-correspondence \
  --example cj0_not2_temporal_absence_probe -- --probe
```

