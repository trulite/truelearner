# CJ0-NOT-2 temporal-absence definitive protocol

Status: **PREREGISTERED; DEFINITIVE EVIDENCE UNSPENT; PX2 REMAINS AUTHORITATIVE**.

## Frozen basis

| item | SHA-256 |
|---|---|
| PX0 substrate source | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| PX2 definitive CSV | `921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18` |
| NOT-2 PROBE CSV | `07cb0d4ccbd817c6de56166f89d4e5719a4d645bfab9d78718169538d36cad7d` |
| NOT-2 PROBE result audit | frozen tag `cj0-not2-temporal-absence-probe-v1-positive` |

The exact ancestor is PX2 commit
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`. No authoritative or frozen
development byte may change. PX3 negatives remain uninterpreted.

## Definitive claim and boundary

> Existing time and transient CELL state support a physical temporal-absence
> behavior: trigger activity leaves temporary state; B reaching that state
> before physical closure prevents the alternative; if B remains absent until
> closure, the surviving state and closure activity fire another path.

The claim is a measured physical timing relation, not a `NOT-B` symbol or a
general timeout/evaluator branch. Same-tick B after closure belongs to the
non-retroactive boundary.

## Fresh fixed matrix

Execute exactly `112` fresh cells: sixteen seed/layout strata crossed with
seven worlds. Namespace is
`0x6_4100_0000 + seed * 0x0100_0000 + world * 0x0010_0000`.
No definitive identity occurs in PROBE evidence.

Seeds rotate normal/mirrored positions, forward/reverse cell allocation,
forward/reverse ARROW insertion, spacings `10/14`, and harmless external
phase/origin order. Every topology has trigger, B, closure, transient, and
output roles. Existing couplings into transient state are `+2`, `-2`, `+2`,
with threshold `3`. Trigger reaches state at tick `1`; physical closure reaches
it at tick `2`.

Seven worlds use the same topology and closure:

1. B absent through closure;
2. B at tick `1`, after trigger;
3. B at tick `2`, ordered before closure;
4. B at tick `2`, ordered after closure;
5. B at tick `3`, after closure/output;
6. B active with a zero-resistance blocked path;
7. B active with a weak delayed path made stale by ordinary pressure.

Trigger propagation drains first. Before B or closure is entered, serialize
initial and post-trigger complete fingerprints and trigger quiescence. Then
serialize every role's firing, signed transient arrivals/ticks, output tick,
pressure deallocation, final quiescence, work, storage, complete/permanent
fingerprints, and exact duplicate equality.

## Conjunctive classification

- post-trigger complete state differs from initial state in every row without
  any new persistent allocation;
- B absent, same-tick-after, tick-3, blocked, and stale worlds emit the
  transient/output path exactly once at closure;
- B at tick `1` and tick `2` before closure physically delivers negative
  activity no later than closure and prevents transient/output firing;
- same-tick-after and tick-3 B cannot retroactively erase output;
- blocked/stale B paths deliver no negative impulse; stale rows record physical
  deallocation;
- all `112/112` rows naturally quiesce at both stages, perform nonzero bounded
  work, use fixed storage, preserve frozen hashes, and replay exactly.

Any failed row freezes NOT-2 definitive negative. PASS freezes NOT-2 positive
only; it does not classify NOT-1 or advance authority.

## Forbidden surfaces

No absence/timeout symbol, evaluator-selected path, new persistent variable,
new threshold/decay/pressure law, semantic adapter, or logical NOT primitive
may enter the substrate. Scenario and expected-output labels remain
evaluator-only.

## Evidence command and atomic publication

The implementation must refuse without `--definitive`, provide a no-cell
`--preflight-not2`, and atomically publish only:

```text
results/cj0_not2_temporal_absence_definitive_v1.csv
results/cj0_not2_temporal_absence_definitive_v1.md
```

After implementation commit/tag and focused validation, execute exactly once:

```text
cargo run --release -p px0-physical-correspondence \
  --example cj0_not_physical_definitive -- --definitive-not2
```

