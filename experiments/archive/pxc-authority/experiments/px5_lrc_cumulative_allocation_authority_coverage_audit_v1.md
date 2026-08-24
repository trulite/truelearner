# PX5 LR-C cumulative allocation authority coverage audit v1

Status: **FROZEN SOURCE/DEPENDENCY CLASSIFICATION; AUTHORITY EVIDENCE UNSPENT**.

## Serial parent and exact replacement

This audit is on the serial branch whose protocol commit is directly parented
by PX4 authority `2348f4318e4c4ca85d6be06017e8ccd7be8b9c87`.

Manifest v3 is byte-identical to PX4 authority manifest v2 except that these
two PX5 predecessor rows:

```text
PX5,src/ds7_cumulative_plasticity_targeting_probe.rs,predecessor-target
PX5,src/ds7_cumulative_plasticity_allocation_gate.rs,predecessor-target
```

are replaced by:

```text
PX5,crates/lr1-modulatory-physical-return/src/lib.rs,shared-authoritative-physical-allocation
```

Every PX0--PX4, PX6, PX7 and PX8 row, including the PX4 surface label, remains
byte-identical. Manifest v3 SHA-256 is
`32fda2d86f6c836438fdd01fc5433c8731f4ec92e3e052e5d1a64fe751d15388`.

## Complete active dependency closure

The authority evaluator package declares exactly two direct dependencies:

```text
px4-lrc-lifetime
lr1-modulatory-physical-return
```

`px4-lrc-lifetime` declares exactly one dependency,
`lr1-modulatory-physical-return`. The LR-C crate declares none. The unique
active Rust dependency closure is therefore exactly:

| path | classification |
|---|---|
| `crates/lr1-modulatory-physical-return/src/lib.rs` | authoritative PX0--PX3+LR-C foundation and shared PX5 physical law |
| `arms/px4-lrc-lifetime/src/lib.rs` | authoritative PX4 physical API |

PX5 adds no active Rust source. Both active hashes are frozen by the protocol,
authority binary and static audit. There is no root-crate, DS7, isolated PX5
evaluator, unrelated PX4 development, optional feature, build script, dynamic
module or PX6 dependency.

## Evaluator-only classification

| path | reason excluded from active manifest |
|---|---|
| `arms/px5-lrc-allocation-authority/src/main.rs` | fresh geometry, anonymous PX4 arrivals, public observations, fixed predicates, exact replay and write-once authority publication only; exports no organism API |
| `arms/px5-lrc-allocation-authority/Cargo.toml` | package/dependency metadata only |
| `arms/px4-lrc-lifetime/src/main.rs` | frozen PX4 development evaluator, unreachable from the authority dependency closure |
| `arms/px4-lrc-lifetime/src/bin/px4_lrc_lifetime_authority_v1.rs` | frozen PX4 authority evaluator wrapper |
| `arms/px4-lrc-lifetime/tests/physics.rs` | frozen PX4 evaluator tests |
| `experiments/px5_lrc_cumulative_allocation_authority_*` | protocol/audit prose only |
| `results/px5_lrc_allocation_authority_v1.*` | future serialized evaluator evidence only |

The new evaluator source SHA-256 is
`d44c806ac7ecc61ed3b561d210f4d542d9189537f93b33f5f16114ee060b11e3`.
It uses ordinary structs, arrays, vectors and direct loops. It contains
`#![forbid(unsafe_code)]`; no unsafe block/function/implementation, interior
mutability, global/thread-local state, proc macro, generated include, semantic
mechanism object, artificial leak or evaluator feedback from measured outcomes
to later organism inputs exists.

The returned topology, loads, identities, timings, pressure observations and
reuse count are fixed by the preregistered matrix. The evaluator never writes
resistance, eligibility, generation, coupling, live state or a candidate. All
such transitions execute inside the manifested LR-C law. All arrivals use the
authoritative PX4 `arrive` API.

## Memory and bounded-work boundary

The evaluator observes `persistent_bytes()` before and after four fixed reuse
exposures. Equality is a row clause; it cannot influence a later arrival.
Generation-safe dead records retained by the parent law are counted rather
than hidden. The fixed byte ceiling is `24000`; the fixed work-ledger ceiling
is `100000` per row. Natural quiescence is conjunctive.

Active sources: **2 unique files**. New active PX5 sources: **0**.
Evaluator sources: **1 new file**. Unclassified candidate sources: **0**.

The static audit script independently rechecks exact hashes, dependency names,
manifest rows, forbidden Rust techniques, forbidden semantic mechanism types,
artifact absence and these classifications before any definitive execution.
