# Physical Body V1 successor authority protocol

Status: **PREREGISTERED — NO AUTHORITY EVIDENCE SPENT**.

## Frozen parent and claim

This protocol is a direct successor of production branch commit
`97d74d5` and the accepted PX-C authority at
`ec87c438aa8c52389fd2734667363ef43acaef93`.

The claim under test is:

> The arena-based TrueLearner physical body preserves the complete retained
> PX0–PX8 continuous-organism behavior while making stable identity,
> persistence, restart, compaction, and resident layout explicit, without
> changing the accepted CELL/ARROW/SPIKE transition laws.

The production candidates are frozen by source digest:

- `truelearner/crates/core/src/lib.rs`:
  `e6767845f27ddb9bb57bfb1fcab6dd1663178449faddc4a630b628e3d1148a8d`;
- `truelearner/crates/arena-format/src/lib.rs`:
  `8c35c3c07fe95b2cc76cbe9ceb47d83f250c5e0c7c40481e7371583afa48a812`;
- `truelearner/Cargo.lock`:
  `592e90c54d28b6cb6cfdb970db3120ffe4c97c50adb3720627ff7f6c34f4900d`.

No production source, physical constant, schedule law, capacity, pressure
law, or transmission law may change after this protocol.

## Disjoint authority worlds

The cumulative matrix contains sixteen fresh roots:

```text
4_100_001 .. 4_100_016
```

They cross both allocation orders, both reflected layouts, and four fresh
phase-preserving physical origins:

```text
1_040, 1_170, 1_300, 1_430
```

Every origin is congruent to zero modulo the retained ten-tick pressure
period. Construction, pressure origin, and first arrival are equal in every
row. These roots and origins are disjoint from PX-C development and authority
evidence.

Each row evaluates the unchanged 32-clause cumulative PX-C contract. The
matrix therefore contributes `16 × 32 = 512` row clauses.

## Frozen body invariants

The authority evaluator additionally records sixteen physical-body clauses:

1. canonical arena bytes decode and re-encode identically;
2. equivalent arena bodies have identical content hashes;
3. body manifests are canonical, sorted, and hash-stable;
4. CELL references remain stable across resident compaction;
5. ARROW references remain stable across resident compaction;
6. compaction preserves exact subsequent physical behavior and crossings;
7. a quiescent checkpoint round-trips to identical canonical bytes;
8. quiescent restart preserves physical clock and pressure phase;
9. quiescent restart preserves exact subsequent behavior;
10. a live checkpoint round-trips to identical canonical bytes;
11. live restart preserves pending spikes and admitted load availability;
12. configured CELL and ARROW capacities reject overflow;
13. dead ARROW slots are reused deterministically with a new generation;
14. stale references cannot resolve or execute after reuse;
15. corruption, truncation, overlap, and trailing bytes fail closed;
16. stale durable internal references fail closed.

The retained PX-C global contract contributes twelve more clauses. The sole
definitive result is positive only at:

```text
16/16 rows
512/512 row clauses
12/12 cumulative global clauses
16/16 physical-body clauses
540/540 total clauses
```

## Static evidence-eligibility gates

Before the definitive run, one targeted E2B validation may establish only:

- the authority evaluator compiles;
- its `--preflight` mode parses the frozen case registry without constructing
  or executing any definitive world;
- formatting and strict Clippy pass for the two production packages and the
  authority verifier;
- production metadata contains exactly `truelearner-core` and
  `truelearner-arena-format` and no dependency path under `experiments/`;
- production contains no `unsafe`, mmap/memmap, directly mutable mapped state,
  or unclassified production Rust file;
- the three frozen digests above match.

No workspace-wide historical suite is required. The ten already-frozen
development tests are not rerun merely for preflight.

## Definitive execution rule

After the evaluator and implementation audit are committed and tagged, one
fresh E2B sandbox executes exactly:

```text
cargo run --release --manifest-path \
  experiments/verification/physical-body-v1-authority/Cargo.toml -- \
  --authority
```

The evaluator must print the unique marker
`PHYSICAL_BODY_V1_AUTHORITY_EVIDENCE_SPENT`, write its CSV and Markdown using
create-new staging files followed by atomic rename, and write all rows and
all clause vectors before returning nonzero on a negative. The command may
not be rerun, rescued, or relabeled.

The fresh sandbox is terminated after artifacts are downloaded. No Rust or
organism command runs locally.

## Acceptance and aftermath

A positive result additionally requires a non-executing result audit that
checks artifact completeness, clause accounting, frozen hashes, disjoint
identities, phase geometry, natural quiescence, exact replay, outward-only
crossings, work and memory bounds, and zero new semantic/runtime surfaces.

Only then may `arch.md` be deliberately updated to name Physical Body V1 as
the successor oracle. Promotion to `main`, cold storage, asynchronous loading,
network transport, and organism-visible storage affordances remain separate
actions.

Any failed definitive clause freezes this authority attempt as an immutable
negative. Any production-law ambiguity stops before evidence.
