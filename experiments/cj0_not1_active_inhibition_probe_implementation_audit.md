# CJ0-NOT-1 active-inhibition PROBE implementation audit

Status: **IMPLEMENTATION FROZEN; PROBE EVIDENCE UNSPENT; PX2 REMAINS AUTHORITATIVE**.

## Frozen inputs and source

| item | SHA-256 |
|---|---|
| authoritative PX0 substrate source | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| authoritative PX2 definitive CSV | `921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18` |
| NOT-1 PROBE protocol | `486c761e83afe9f72e3d18516e623812697398c5b2bc1ac2a3cf789dadd12e44` |
| NOT-1 PROBE runner | `37d1968f77f5a25ed84f557c4eaf174fa09e9356f897264d505c987fa121113c` |

The checkout began clean at exact commit
`2fbee861a0aeed335d3ffa8f9095ca28f2ac6129`. The implementation adds only
this evaluator/fixture runner and CJ0-NOT protocol/audit records. No path that
existed in the authoritative tree is changed.

## Mechanism and leak audit

The runner imports the authoritative public `PlasticSubstrate` API and uses
only ordinary cells, arrows, positive external spikes, signed existing ARROW
coupling, transient cell state, thresholds, pressure, firing, and queue drain.
It defines no CELL/ARROW/SPIKE replacement, no inhibitory or logical-NOT state,
and no new persistent substrate variable. Scenario names and pass clauses are
evaluator-only and have no causal path into `PlasticSubstrate`.

Normal and mirrored fixtures reverse allocation/position and use disjoint
physical identities. The stale condition is produced mechanically: a weak
long-delay negative ARROW emits, ordinary pressure deallocates it, and the
queued generation becomes physically invalid before delivery. The blocked
condition uses the existing zero-resistance non-live rule.

## Work and storage audit

Every row serializes the authoritative work-ledger total, persistent substrate
bytes, complete and permanent fingerprints, crossing count, natural
quiescence, and exact duplicate equality. The runner adds no dependency and
does not use wall-clock time, randomness, hidden cutoff, or a semantic adapter.

## Validation

Before evidence, all passed:

```text
cargo fmt --all -- --check
cargo check -p px0-physical-correspondence --example cj0_not1_active_inhibition_probe
cargo clippy -p px0-physical-correspondence --example cj0_not1_active_inhibition_probe -- -D warnings
cargo run --release -p px0-physical-correspondence --example cj0_not1_active_inhibition_probe -- --preflight
git diff --check
```

Preflight entered no cell, printed only the preregistered preflight marker, and
confirmed that final and staging artifacts were absent. Definitive evidence,
NOT-2 evidence, PX3 interpretation, and authority advancement remain absent.
