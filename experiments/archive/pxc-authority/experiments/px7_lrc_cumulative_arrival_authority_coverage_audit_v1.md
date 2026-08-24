# PX7 LR-C cumulative arrival authority coverage audit v1

Status: **FROZEN SOURCE/DEPENDENCY CLASSIFICATION; AUTHORITY EVIDENCE UNSPENT**.

## Exact serial lineage

The protocol commit `ea53d4a869b2f556a17d5e9f00f9169a51c13c4c` is
directly parented by exact PX6 authority
`1ca6df74eacbb743f23ca5b5810919985036cd64`. The isolated PX7 development
branch and tags remain unchanged. Only the byte-frozen active PX7 crate and the
scientifically frozen evaluator geometry/predicates were ported.

The current active surface remains PX6 manifest v4, SHA-256
`653289cf42577dabb242475fd88abe24405b3e9a7e3cd4f2961489cc5fe6953a`,
until functional evidence is frozen. The registered v5 transformation replaces
only:

```text
PX7,src/post_m6_ds4_arrival_initiation.rs,predecessor-target
```

with:

```text
PX7,crates/px7-lrc-arrival/src/lib.rs,authoritative-physical-arrival-initiation
```

No PX0--PX6 or PX8 row may change.

## Complete active dependency closure

The unique cumulative active Rust closure is exactly:

| source | classification |
|---|---|
| `crates/lr1-modulatory-physical-return/src/lib.rs` | authoritative PX0--PX3+LR-C law and shared PX5/PX6 reduction |
| `arms/px4-lrc-lifetime/src/lib.rs` | authoritative PX4 physical API |
| `crates/px7-lrc-arrival/src/lib.rs` | new PX7 level-blind physical topology/participation surface |

The PX7 crate declares only LR-C. It contains no local module, generated
include, optional feature, build script, macro source, evaluator dependency,
or frozen mechanism namespace. PX5 and PX6 add no active source.

## Evaluator-only classification

`arms/px7-lrc-arrival-authority/src/main.rs` is evaluator-only: it owns fresh
matrix identities, physical arrival schedules, observations, predicates,
resource accounting, embedded duplicate-state comparison, input hashes,
execution firewall, and write-once publication. It exports no organism API and
dependency flow is one-way from evaluator to the three active sources.

The authority Cargo manifest, protocol/audit prose, result serialization,
static audit script, taxonomy/comparator scripts, frozen earlier evaluators,
and all result artifacts are evaluator-only. No root crate, DS predecessor,
isolated PX7 report, development handoff, or PX8 source is reachable from the
active PX7 crate.

## Resource and leakage boundary

Complete observations cover every physical batch plus fresh cumulative
conformance. Each row requires maximum work `<=4096`, maximum persistent bytes
`<=4096`, stable retained structural allocation, natural quiescence, and exact
duplicate equality. The evaluator contains no unsafe code, interior mutability,
hidden global/thread-local state, generated code, semantic mechanism object,
artificial leak, or measured-result feedback into later physical input.

Active sources: **3 unique files**. New active PX7 sources: **1**. Evaluator
sources: **1**. Unclassified candidate sources: **0**.
