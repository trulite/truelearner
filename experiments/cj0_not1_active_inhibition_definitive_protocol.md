# CJ0-NOT-1 active-inhibition definitive protocol

Status: **PREREGISTERED; DEFINITIVE EVIDENCE UNSPENT; PX2 REMAINS AUTHORITATIVE**.

## Frozen basis

| item | SHA-256 |
|---|---|
| PX0 substrate source | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| PX2 definitive CSV | `921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18` |
| NOT-1 PROBE CSV | `4f3ad19bea689a60641852ef038e7ba5d8938e8dcdba802f0019dea8df68dedb` |
| NOT-1 PROBE result audit | frozen tag `cj0-not1-active-inhibition-probe-v1-positive` |

The exact ancestor is PX2 commit
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`. No authoritative or frozen
development byte may change. PX3 negatives remain uninterpreted.

## Definitive claim and boundary

> Existing ordinary signed coupling supports active physical inhibition: A
> activity reaching an integration site before the B path's threshold crossing
> suppresses that crossing; A absent leaves B operational; A arriving only
> after the crossing cannot erase the already-emitted result.

This claim is explicitly order-sensitive. Same-tick A after B belongs to the
non-retroactive boundary, not the timely condition. No logical NOT primitive or
general Boolean complement is claimed.

## Fresh fixed matrix

Execute exactly `112` fresh cells: sixteen seed/layout strata crossed with
seven worlds. Namespace is
`0x6_3100_0000 + seed * 0x0100_0000 + world * 0x0010_0000`.
No definitive identity occurs in PROBE evidence.

Seeds rotate:

- normal/mirrored positions;
- forward/reverse cell allocation;
- forward/reverse ARROW insertion;
- two physical spacings (`10`, `14`);
- alternative harmless external phases and origin identities.

The seven worlds use identical A/B/integration/output roles and existing
couplings `A -> integration = -2`, `B -> integration = +2`, threshold `2`:

1. A absent;
2. A one tick early;
3. A coincident and ordered before B;
4. A coincident but ordered after B;
5. A one tick late;
6. A active with a zero-resistance blocked path;
7. A active with a weak delayed path made stale by ordinary pressure.

Every complete starting state is cloned and executed twice. The evaluator
serializes every role's firing, signed integration arrivals and ticks, output
tick, crossing count, pressure deallocation, natural quiescence, work ledger,
persistent bytes, complete/permanent fingerprints, and duplicate equality.

## Conjunctive classification

- absent, coincident-after, one-tick-late, blocked, and stale worlds emit B's
  output exactly once;
- early and coincident-before worlds physically receive negative A activity
  before threshold crossing and emit no integration/output firing;
- coincident-after and late negative arrivals occur strictly after the output
  or after its same-tick threshold crossing and have no retroactive effect;
- blocked/stale paths deliver no negative impulse; stale rows record physical
  deallocation;
- all `112/112` rows naturally quiesce, perform nonzero bounded work, use fixed
  storage per layout, preserve frozen hashes, and replay exactly.

Any failed row freezes NOT-1 definitive negative. PASS freezes NOT-1 positive
only; it does not classify NOT-2 or advance authority.

## Evidence command and atomic publication

The implementation must refuse without `--definitive`, provide a no-cell
`--preflight-not1`, and atomically publish only:

```text
results/cj0_not1_active_inhibition_definitive_v1.csv
results/cj0_not1_active_inhibition_definitive_v1.md
```

After implementation commit/tag and focused validation, execute exactly once:

```text
cargo run --release -p px0-physical-correspondence \
  --example cj0_not_physical_definitive -- --definitive-not1
```

