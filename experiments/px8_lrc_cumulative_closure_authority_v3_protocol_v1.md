# PX8 LR-C cumulative physical-closure authority v3 protocol v1

Status: **PREREGISTERED; AUTHORITY-V3 EVIDENCE UNSPENT; FINAL PX-C FORBIDDEN**.

## Frozen parent and sole repair

This protocol is the first child of frozen v2 diagnostic result
`15c40708220587a12ea872291e2aca3f934ff794`, tagged
`px8-lrc-closure-authority-v2-negative-diagnostic-result-v1`.

Active PX8 remains SHA-256
`8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f`.
Retained LR-C, PX4, and PX7 remain respectively
`7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10`,
`a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71`,
and `d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e`.

The sole repair is clause 12 measurement semantics. Mature retained fixtures
(primary, uninterrupted, incomplete, duplicate, blocked, and cumulative PX7)
require exact same-body `before == after` persistent bytes. The deliberately
aged stale/reproposal fixture does not require allocation equality. It instead
conjunctively requires:

```text
outward_crossings == 0
stale_route_executions == outward + inward_modulation == 0
memory_after <= 8192
queue_empty && naturally_quiescent
exact deterministic replay
fresh lawful proposals permitted
```

`fresh_proposals` is the deterministic structural append count derived from
the observed stale byte delta using the frozen retained-arrow allocation size
`64` bytes. A nonnegative proposal count is serialized but is not itself a
failure. Memory before, after, signed delta, capacity, outward crossings,
stale route executions, fresh proposals, queue/quiescence, and replay must be
serialized separately and unconditionally.

Every active mechanism, law, physical world, schedule, threshold, behavioral
clause, work ceiling `20000`, memory ceiling `8192`, and other predicate is
unchanged.

## Fresh authority identities

Roots are exactly `865001..865016`. Primary namespaces are `root << 32`,
compact controls use `+10000..+60000`, and cumulative PX7 uses
`(root + 1_000_000) << 32`. Construction/reflection and twists
`0,137,274,411` remain balanced. Schedules are identical to v2.

The fresh evaluator package is `arms/px8-lrc-closure-authority-v3`, directly
depending only on active PX8 and retained PX7. It accepts only
`--authority-v3`, emits exactly one
`PX8_LRC_CLOSURE_AUTHORITY_V3_EVIDENCE_SPENT` marker, and writes only:

```text
results/px8_lrc_closure_authority_v3.csv
results/px8_lrc_closure_authority_v3.md
```

## Clauses and unconditional publication

Clauses 1--11 and 13--14 are byte-equivalent to v2. Clause 12 is the sole
repair above. Threshold remains `16/16`, `224/224` row clauses, `6/6` globals,
and `230/230` total.

All sixteen rows, all fourteen booleans, all mature byte pairs, and all stale
metrics must be create-new serialized before the aggregate row/global verdict
is asserted. Thus a negative remains fully diagnostic. Overwrite, rescue,
partial roots, or rerun is forbidden.

## Validation and definitive execution

Batch evaluator, coverage, static audit, hashes, and firewall. One fresh E2B
sandbox runs package rustfmt check, strict package Clippy, one no-world test,
static audit, and preflight. After an implementation/audit freeze, one fresh
E2B sandbox runs exactly once:

```text
cargo run --release --manifest-path arms/px8-lrc-closure-authority-v3/Cargo.toml -- --authority-v3
```

No Rust, project program, or project audit runs locally.

## Conditional PX-C gate

Only a positive frozen v3 result may replace exactly the PX8 predecessor in
manifest v5 and run fresh taxonomy/comparator against immutable PX7:

```text
primary seams       110 -> 0 required
semantic guards     <= 36
evaluator guards    <= 136
foundation          0
new kinds           0
new surfaces        0
```

On v3 failure, freeze the immutable negative and stop. On success, freeze the
result audit, taxonomy/comparator, and clean PX8-only authority handoff. Do not
start final PX-C continuous-organism authority.
