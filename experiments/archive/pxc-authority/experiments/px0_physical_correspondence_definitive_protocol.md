# PX0 substrate-native physical correspondence definitive protocol

Status: **PREREGISTERED; DEFINITIVE EVIDENCE UNSPENT; PX0 AUTHORITY ABSENT**.

This protocol is committed and tagged before the authority implementation and
before any definitive cell, seed, command, marker, or result artifact exists.

## Frozen authority ancestor

The authority workflow begins exactly at combined PX0-R development-readiness
commit `745a5c3dc6d929faa2908359c5eb0462e8eac663`, tag
`px0-r-generic-physical-reproposal-development-readiness`.

The following bytes are frozen:

| artifact | SHA-256 |
|---|---|
| retained substrate physics | `6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263` |
| PX0/PX0-R active physical law | `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d` |
| PX0-v1 readiness | `60f21e096079a5532f23d8a2974052f373a2fc6481c71099292147c6deb8cf5b` |
| PX0-R protocol | `65cc95374badd259ef023a17d0f745f39d26b5377b34317da3db92599758c107` |
| PX0-R readiness | `508f958da3a7c583e611e5a113a5a66c4e98792b7dda9378523b8354447544e3` |

The definitive implementation may add only an evaluator/authority wrapper,
fixed matrix data, source/hash audits, and atomic write-once serialization. It
may not modify the active physical law or retained physics.

## Authority question

> Can one unchanged substrate-native CELL/ARROW/SPIKE law acquire an
> executable physical correspondence, reuse it while its structure remains
> live, physically forget it completely, and learn a fresh correspondence
> from renewed anonymous activity and contemporary returned physical
> consequence, without semantic intermediate representation or historical
> resurrection?

## Authoritative claim boundary

Freshly reacquired physical structure need not reproduce the old arrow
identity or exact topology. Functional correspondence and physical provenance
are authoritative; historical structural identity is not.

Every old variable arrow must remain non-live after forgetting. Reacquisition
must allocate fresh `ArrowId` and generation-bearing structure from current
local physical opportunity. If the return world changes from route A to route
B, the fresh survivor and outward behavior must follow B.

## Exact matrix

The matrix contains exactly 16 blank deterministic cells. Cell `i` uses base
namespace `0x8000000 + i * 0x40000`; none appeared in PX0/PX0-R development.

For cell `i`:

- initial selected route: `i mod 3`;
- contemporary reacquisition route: `(initial + 1) mod 3`;
- spare route: `(initial + 2) mod 3`;
- allocation order: forward for even `i`, reverse for odd `i`;
- physical layout mirror: false for `i mod 4` in `{0,1}`, true otherwise;
- experience spacing: `8 + (i mod 8)` ticks;
- active local opportunity count: two for `i < 8`, three for `i >= 8`;
- arrival phase order alternates within experience and is mirrored by cell.

All cells use three physically available local route neighborhoods, fresh cell
identities, fresh spike origins, varied physical spacing/layout, the same
generic proposal law, and ordinary physical return through scaffold activity.
No route index enters the learner.

## Exact within-cell sequence

Each cell runs these stages in order without reset:

1. **Initial acquisition.** Four anonymous active presentations occur at
   ticks `0`, `s`, `2s`, and `3s`; ordinary return supports only initial route
   A. A held-out presentation at `4s` must cross outward exactly once through
   retained A structure.
2. **Survival-window reuse.** No source activity occurs for 20 ticks. The
   original A arrows must remain live. Fresh anonymous A activity must execute
   one outward crossing using those same live identities.
3. **True forgetting.** With no sustaining source activity, ordinary time and
   pressure advance to tick 300. Every original A variable arrow must reach
   resistance zero, remain stored only as dead provenance, and be
   non-executable. Time alone must create zero proposal and resurrect nothing.
4. **Reacquisition.** The physical return world now supports B. Renewed
   anonymous activity occurs at tick 300 and four more times separated by `s`.
   Generic adjacency-limited opportunity must create fresh arrows. Held-out B
   activity at `300 + 5s` must cross outward exactly once. A without returned
   support at `300 + 6s` must not cross. Every original A arrow remains dead.
5. **Independent controls.** Fresh blank fixtures exercise absent return,
   ambiguous equal return, and complete-state duplicate replay with namespaces
   derived from the cell base and disjoint from the primary fixture.

## Twelve conjunctive cell controls

Every cell must pass all twelve:

1. `P0` frozen law/namespace/layout parameters are exact;
2. `P1` anonymous activity plus ordinary return acquires A;
3. `P2` held-out A executes exactly once and becomes naturally quiescent;
4. `P3` bounded absence preserves the original live A identities and reuse;
5. `P4` sufficient pressure makes all original A arrows resistance-zero and
   non-live;
6. `P5` no renewed activity creates no proposal, no outward effect, and no
   resurrection;
7. `P6` renewed anonymous activity creates fresh identities, none equal to an
   original A arrow;
8. `P7` contemporary opposite-return topology retains B and restores exactly
   one outward crossing while historical A stays silent;
9. `P8` absent return retains no executable correspondence;
10. `P9` ambiguous equal return creates no privileged outward winner;
11. `P10` exact complete-state duplicates replay exactly;
12. `P11` all executions drain naturally, old paths stay dead, and proposal,
    deallocation, work, storage, identity, and fingerprint accounting is
    complete.

The definitive outcome is conjunctive: `192/192` controls, `16/16` cells, all
matrix dimensions, exact frozen hashes, semantic/source isolation, and atomic
artifacts must pass. Any single failure is a definitive PX0 negative.

## Anti-cheat and source audit

Organism-visible state and execution may contain only the frozen active law's
CELL/ARROW/SPIKE physics. The authority wrapper must verify that active source
contains no motif, relation object, episode, history, query, answer,
correct/wrong, route template, evaluator-selected mutation, semantic boundary,
or renamed equivalent. The active crate must retain zero normal dependencies.

The wrapper may observe identities, liveness, resistance, crossings,
fingerprints, work, and storage. It may construct physical cells, scaffold
topology, and anonymous spike schedules. It may not mutate, strengthen,
weaken, delete, restore, or select an active arrow after construction.

The same broad opportunity law must support A before forgetting and B after
forgetting. This opposite-return control is the primary protection against
historical resurrection or answer-bearing opportunity structure.

## Pre-evidence validation

Before execution, the frozen implementation snapshot must pass:

- formatting, focused compilation, focused tests, and strict Clippy;
- retained-physics and active-law hash checks;
- zero-dependency and semantic-source audits;
- exact development-lineage hashes;
- a no-cell source preflight;
- refusal without `--definitive`;
- refusal if either final or staging artifact already exists;
- the existing cleanup and deterministic affordance-law conformance checks.

Preflight may not invoke a matrix cell or emit the evidence-spend marker.

## Sole execution and no-rescue rule

The definitive command is executed exactly once from a clean, committed,
tagged implementation snapshot in a fresh authority sandbox:

```text
cargo run --release -p px0-physical-correspondence --example definitive -- --definitive
```

The runner repeats no-cell preflight, emits exactly one
`PX0_DEFINITIVE_EVIDENCE_SPENT` marker immediately before cell zero, executes
all 16 cells in literal order, publishes both artifacts atomically, and exits
`0` for PASS or `1` for a complete scientific FAIL.

An infrastructure failure before the marker spends no evidence. Any panic,
interruption, serialization failure, partial result, or infrastructure failure
after the marker is an immutable incomplete negative. No rerun, rescue,
tuning, matrix amendment, or result regeneration is permitted.

## Atomic write-once artifacts

The preregistration digest over every sorted pre-existing result-file hash is
`9dad0d2c8ddd4439aebd3aafc71665eb8d8e47dd2510a6a71e15ff9d0fd46920`.
At preregistration, all four paths below are absent:

```text
results/.px0_physical_correspondence_definitive.csv.staging
results/.px0_physical_correspondence_definitive.md.staging
results/px0_physical_correspondence_definitive.csv
results/px0_physical_correspondence_definitive.md
```

The runner creates and syncs staging files with create-new semantics, then
publishes without replacement. Both complete PASS and complete scientific
FAIL publish write-once artifacts. Result commits may not alter executed source
bytes.

## Program consequence

PASS creates the first authoritative physicalization ancestor:

```text
retained substrate physics
        ↓
PX0 authoritative substrate-native physical correspondence
```

PASS makes cumulative PX1 development eligible. PX1 must consume PX0-produced
physical structure directly in the same substrate; serialization into old M0
or M1 schemas, `RelationMotif`, or a behavioral adapter is forbidden.

FAIL leaves PX0 non-authoritative and PX1, PX-C, the continuous organism, and
Harness H1 blocked. No result authorizes changing the frozen active law.
