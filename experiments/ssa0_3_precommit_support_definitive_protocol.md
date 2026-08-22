# SSA0.3 definitive pre-closure support protocol

Status: **PREREGISTERED DEFINITIVE SUBSTRATE AUTHORITY; EVIDENCE UNSPENT**.

Protocol namespace: `ssa0-3-precommit-support-definitive-v1`.

This is a separately named, authority-owned substrate specification. It tests
only whether deterministic local support can change a realized executable
CELL/ARROW/SPIKE trajectory before the first contender threshold firing and
its immediate inhibitory closure, while equivalent support after that closure
is inert. It does not reinterpret prior Classification C, does not advance M6
or M7, cannot be consumed by Lane A, and does not begin SSA1 or SSA2.

No definitive cell existed or executed before this document was frozen.

## Independent lineage and byte audit

The authority audited the clean checkout at
`eeb14186a000a7eefba17e6f9e288e7335c44043` before preregistration. The
following annotated tags and peeled commits were exact:

| frozen stage | tag object | peeled commit |
|---|---|---|
| immutable SSA0 Classification C | lightweight | `34277893201c1a72765b143de4b3da1912b6e3b6` |
| SSA0.3 development protocol | `afa220ed9404d1f5699fbfb3145d5ff2fb9ea634` | `c6f28ce979f05c358f313ee4fc202a6304fa1b70` |
| SSA0.3 implementation | `c569f240ed091209989188f6fdf6bf13c86913fb` | `ace5f39e77e83ca0478c283a157d9f7dd2f87429` |
| positive PROBE | `d39b97dc0554d728664c121fd2414dbdebf6cd7b` | `95507864249ec3906a0ae4c0ae2863f6d459dab8` |
| positive MICRO | `842668479058dc25453af517c68e93c777600d32` | `577cd56555fcbd3b5f2dbfee44f252e930039fed` |
| positive GATE | `937954f01957bc726c011e1c0767fba9bafc474f` | `eeb14186a000a7eefba17e6f9e288e7335c44043` |
| development Classification A | `d8c7e2636f32e3043c3888d40fb8b7d1b4274ede` | `eeb14186a000a7eefba17e6f9e288e7335c44043` |
| authoritative M6 | lightweight | `aa4e22efd8a65b7694956a53cfaa970582695215` |

The audited ancestry is M6 -> immutable C -> SSA0.3 protocol -> implementation
-> PROBE -> MICRO -> GATE/Classification A. No tag was moved or amended.

| frozen artifact | SHA-256 |
|---|---|
| SSA0 protocol | `92ff3f758977a575e2f8ca651f7a45756e15241a6d4bf829012a266bae9489fc` |
| SSA0 source | `180b24f6b682ec5d274e44b0c680062d10b1f68b6fddeb4d857ec599b32f6299` |
| SSA0 runner | `fb693aa098e45617deefd5ae9b9de1003528d4b7fbfa87078545fbda5e90fa7f` |
| SSA0 Classification C Markdown | `905a29fa34c4af08815e039ab195f8b834a5631d4110190f87ace3f45985a0e4` |
| SSA0 Classification C raw | `ae61b54a63c8dab35d51fcdb642f91bbc02c696bcd20efc8e877d02a3728fe75` |
| SSA0.3 development protocol | `f9121b0b08867b4189892ab9f46658ebd4a95874c73c5a9182bce17f4f49cef1` |
| SSA0.3 source | `4a4e727f4f8ca6ee03faaae76de1a1091472de20ed9d91388e7a36056326edd7` |
| SSA0.3 runner | `3711f123be3a4efc0494cc01f85fd8bc176ffc765eb1a2183b1e14c76baea435` |
| SSA0.3 GATE Markdown | `ded620267a4a88e63588c1807149ece25d50876c9b53717827bd9070ead99ce2` |
| SSA0.3 GATE raw | `bae174af3e326d0c3446a023e415914188711f09a61426b95217891d063da4e7` |
| DS-A1 source | `b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1` |
| M6 authority source | `c2a95199139828e360713320ad57c77a100fc0135ba06b9219624d4f16e1d18d` |
| research runtime | `e570b3cd0fcff759a02a38a685f22f33bc28de65e25b7beb34f77d138f3fd711` |

The SSA0 and SSA0.3 `SSA0_PHYSICS_BEGIN`--`SSA0_PHYSICS_END` regions were
byte-identical, with region SHA-256
`a7447803c137a4577bab0ea24333bd6db9c31d665c06754d0179652e8fc049bd`.
The original SSA0.3 source/runner equal their implementation-tag blobs. The
immutable C result/source and the DS-A1, M6-authority, research-runtime, and
`src/lib.rs` files equal their frozen ancestors. No Lane A source or protocol
mentions SSA0 or SSA0.3.

## Frozen physics and information boundary

The definitive source must copy the complete development representation and
propagation loop, with the marked physics region byte-identical to both frozen
SSA0 sources. The following remain unchanged: CELL and ARROW fields, SPIKE
ordering, threshold `4`, saturating integer integration, generation/liveness,
one-shot firing, queue order, effect propagation, and immediate mutual
inhibition impulse `-64`.

Only an external authority fixture, observations, predicates, source/hash
audit, and atomic serializer may be added. The organism receives no commitment
metadata. Closure is observed only as the first contender's actual threshold
firing, followed immediately in the physical trace by its inhibitory SPIKE to
the other contender. Route names and expectations are evaluator-only and are
never read by propagation.

Forbidden everywhere in the runtime mechanism are RNG/noise, an action
chooser, harness sampling, stored probability, supporter-count or precommit
score, softmax/temperature/noisy argmax, semantic action/effect labels,
evaluator selection of a winner, a commitment boundary value, or a commitment
CELL. Descriptive counts may be serialized after execution only.

## Fresh namespace and physical worlds

The exhaustive pre-SSA scan found development physical identity offsets only
through `0x0050_0000`, occurrence origins only in `0x7000_0001` through
`0x7300_0001`, contender arrival ticks only in `2,4,5,6,7,8,9,10,11`, and
experimental phases only in `-200,-10,0,200`. Parent transfer used offset
`0x4000_0000`. The definitive matrix uses none of those experimental values.

Each matrix row receives a unique world ordinal `0..743`. Physical identities
are allocated only from the disjoint interval beginning at `0x9000_0000`, with
a stride of `0x0001_0000` per ordinal and fixed per-world endpoint offsets.
Occurrence identities begin at `0xe300_0000` plus the ordinal. Exact duplicate
replay reuses that complete physical start state only for the required replay
of the same row. No other row reuses a physical or occurrence identity.

Four fixed transfer allocations are crossed in this order:

| allocation | CELL order | ARROW order | inert padding | layout origin |
|---|---|---|---:|---:|
| `spiral_29` | rotate left 3 | rotate left 5 | 29 | -6000 |
| `reverse_31` | full reverse | full reverse | 31 | 6000 |
| `woven_37` | rotate left 7 | rotate right 11 | 37 | -12000 |
| `mirror_43` | full reverse | rotate left 13 | 43 | 12000 |

The allocation also shifts every live CELL coordinate by the unique world
ordinal. Padding CELLs have no incident ARROW and cannot fire. These layouts,
padding counts, physical identities, allocations, and occurrence identities
are absent from every earlier SSA result.

## Fresh commitment schedules

Three schedule families are crossed in this order. `target` is the externally
designated slower route for a mirror; `competitor` is the other route. Every
base delivery is an ordinary live source->relay->contender path of impulse one.

| schedule | target base ticks | competitor base ticks | contender phase | baseline closure `C` | target slow tick `S` |
|---|---|---|---:|---:|---:|
| `quartz_29` | `[17,21,25,37]` | `[17,21,25,29]` | 101 | 29 | 37 |
| `rill_53` | `[41,45,49,61]` | `[41,45,49,53]` | 307 | 53 | 61 |
| `spire_89` | `[73,79,85,101]` | `[73,79,85,89]` | 709 | 89 | 101 |

For a schedule, `F`, `B`, and `T` denote its first, second, and third base
ticks. Its ordinary off-tick intervention phase is contender phase plus `17`.
Its same-tick before/after phases are contender phase minus/plus `1000`.
Coincident-spacing phases are contender phase minus `503` and `401` for early
delivery and plus `1401` and `1503` for late delivery. All ticks, commitment
schedules, and intervention phases are fresh relative to prior SSA evidence.

## Exact 744-row matrix and order

The nested order is schedule as listed above, allocation as listed above,
physical mirror `target=0` then `target=1`, family as listed below, and
condition in table order. This is `3 * 4 * 2 * 31 = 744` rows. Each row executes
twice consecutively from the exact complete state, for 1,488 deterministic
propagations. No ordering, condition, expected route, expected tick, or
predicate may change after this protocol freeze.

`target` means the mirrored slower route and `competitor` means `1-target`.
Every late condition must retain every added positive physical delivery in its
trace even though inhibition has already closed the realized route.

| family | condition | added target deliveries | expected first contender/tick |
|---|---|---|---|
| timing | `baseline` | none | competitor / `C` |
| timing | `well_before` | `(C-7, off)` | target / `T` |
| timing | `just_before` | `(C-1, off)` | target / `C-1` |
| timing | `closure_before` | `(C, before)` | target / `C` |
| timing | `closure_after` | `(C, after)` | competitor / `C` |
| timing | `just_after` | `(C+1, off)` | competitor / `C` |
| timing | `well_after` | `(C+7, off)` | competitor / `C` |
| number | `one_early` | `(C-2, off)` | target / `C-2` |
| number | `one_late` | `(C+2, off)` | competitor / `C` |
| number | `two_early` | `(T-2, off),(T-1, off)` | target / `T-1` |
| number | `two_late` | `(C+2, off),(C+5, off)` | competitor / `C` |
| impulse | `unit_early` | `(C-3, off, impulse 1)` | target / `C-3` |
| impulse | `unit_late` | `(C+3, off, impulse 1)` | competitor / `C` |
| impulse | `double_early` | `(C-3, off, impulse 2)` | target / `C-3` |
| impulse | `double_late` | `(C+3, off, impulse 2)` | competitor / `C` |
| spacing | `wide_early` | `(F+1, off),(T-1, off)` | target / `T-1` |
| spacing | `wide_late` | `(C+2, off),(C+11, off)` | competitor / `C` |
| spacing | `near_early` | `(T-2, off),(T-1, off)` | target / `T-1` |
| spacing | `near_late` | `(C+2, off),(C+3, off)` | competitor / `C` |
| spacing | `coincident_early` | `(T-1, early phase 1),(T-1, early phase 2)` | target / `T-1` |
| spacing | `coincident_late` | `(C+3, late phase 1),(C+3, late phase 2)` | competitor / `C` |
| count | `total_4_pre_3` | none | competitor / `C` |
| count | `total_5_pre_3` | `(C+3, off)` | competitor / `C` |
| count | `total_6_pre_3` | `(C+3, off),(C+6, off)` | competitor / `C` |
| count | `total_7_pre_3` | `(C+3, off),(C+6, off),(C+9, off)` | competitor / `C` |
| route | `target_blocked` | target supporter ARROWs non-live | competitor / `C` |
| route | `target_stale` | target supporter ARROW generation stale | competitor / `C` |
| route | `target_absent` | no target supporter path | competitor / `C` |
| route | `competitor_blocked` | competitor supporter ARROWs non-live | target / `S` |
| route | `competitor_stale` | competitor supporter ARROW generation stale | target / `S` |
| route | `competitor_absent` | no competitor supporter path | target / `S` |

The same-count temporal comparison is `one_early` versus `one_late`, and also
`closure_before` versus `closure_after`: total target count is five in each
member, while the realized executable route must reverse only on the
pre-closure side. The different-count/equal-preintegration comparison is the
four-row `count` family: target total count is four through seven, exactly
three target deliveries precede baseline closure, and behavior must remain
competitor realization at `C`.

## Per-row and global predicates

A row passes only when all of the following hold in both exact replays:

1. complete start bytes, start/permanent/trace/end fingerprints, full trace,
   fired sequence, realized effect set, and physical work ledger are exact;
2. the first contender firing is the preregistered physical route and tick;
3. the immediately next applicable trace entry is the same-tick inhibitory
   impulse `-64` to the other contender, which never fires afterward;
4. exactly one live executable effect is reached and belongs to the first
   contender, except that no special exception is available for a failed
   route control;
5. every added delivery classified pre-closure occurs no later than the
   firing SPIKE that crosses threshold, and every added delivery classified
   late is visibly delivered after the actual closure ordering;
6. blocked, stale, or absent paths never fire their contender or effect; and
7. every live-alone route control reaches its own nonempty, distinct effect.

The definitive positive is conjunctive. All 744 rows must pass; all duplicate
replays must be exact; all same-count, equal-preintegration, number, impulse,
spacing, mirror, schedule, identity, occurrence, allocation, layout, closure,
late-visibility, blocked/stale/absent, and independent-execution aggregates
must pass; the marked physics must be byte-exact; all frozen hashes must match;
the forbidden-runtime audit must pass; M6 and Lane A must remain isolated; and
the result paths must have been atomically created once.

Any scientific predicate failure freezes a definitive negative exactly. A
panic, interruption, transport failure, nonzero process after the first cell,
missing or partial artifact, or publication failure after the evidence
boundary freezes an incomplete/negative authority outcome. There is no rescue,
rerun, resume, replacement, selective reuse, tuning, or reinterpretation.

## No-cell executable freeze

After this protocol commit/tag, the authority may add only the copied physics,
fresh fixtures, evaluator, source/hash audit, and atomic wrapper described
here. Before freezing the exact executable snapshot it may run only:

```text
cargo fmt --all -- --check
cargo check --release --bin ssa0_3_precommit_support_definitive
cargo clippy --release --bin ssa0_3_precommit_support_definitive -- -D warnings
cargo test --release --bin ssa0_3_precommit_support_definitive
cargo run --release --quiet --bin ssa0_3_precommit_support_definitive -- --audit
cargo run --release --quiet --bin ssa0_3_precommit_support_definitive
```

Tests are restricted to source/hash/matrix-shape audits, zero-cell path
refusal, and create-new atomic publication using bounded non-definitive
temporary paths. `--audit` must enumerate only static specifications and must
refuse if any fixed final/staging path exists. No test or audit may call a
definitive row. No broad historical suite may run because shared frozen code
must not change. The no-argument command must refuse before harness entry with
exit `2`.

The implementation snapshot is committed/tagged only after the above focused
checks pass in the fresh dedicated E2B authority sandbox. The protocol and all
development artifacts must remain byte-identical in that snapshot.

## One-shot E2B command and atomic boundary

Dedicated state, absent at preregistration:

```text
/Users/satya/.cache/truelearner/ssa0-3-precommit-support-definitive-authority-e2b.json
```

Transport and credentials:

```text
/Users/satya/work/br/truelearner/scripts/e2b_persistent.py
/Users/satya/work/br/truelearner/.env.e2b
```

The state must create a fresh `truelearner-rust-1-97-worker`, distinct from all
development and Lane A sandboxes, from a clean archive of the exact tagged
implementation commit. It must be left running.

The final and fixed staging paths must be absent locally and remotely:

```text
results/ssa0_3_precommit_support_definitive_v1.csv
results/ssa0_3_precommit_support_definitive_v1.md
results/.ssa0_3_precommit_support_definitive_v1.csv.staging
results/.ssa0_3_precommit_support_definitive_v1.md.staging
```

The evidence boundary is the entry into row ordinal `0` after the final
zero-cell preflight. The release command authorized exactly once is:

```text
cargo run --release --quiet --bin ssa0_3_precommit_support_definitive -- --definitive
```

The runner has only `--audit` and `--definitive`; there is no default, override,
filter, partial, development, resume, append, overwrite, seed, schedule, or
output option. It must compute the whole frozen matrix in memory, fully write
and sync both fixed create-new staging files, and atomically hard-link both
without replacing an existing path. The CSV and Markdown are downloaded only
after the sole command returns. Post-run work is read-only hashing, parsing,
and source/result audit; it may never invoke Rust or a cell again.

## Frozen disposition

If and only if the full conjunction passes, freeze the result as
`ssa0-3-precommit-support-definitive-positive`, later freeze the independently
audited authority as `deterministic-affordance-causal-window-authoritative`,
and include the narrow pre-closure support law in the frozen candidate minimal
substrate specification.

On any failure, freeze the exact definitive negative or incomplete outcome.
In every disposition, immutable Classification C and developmental
Classification A remain unchanged; M6/M7 authority is unchanged; Lane A is
isolated; SSA1 and SSA2 remain blocked; and program-priority decisions remain
outside evidentiary scope.
