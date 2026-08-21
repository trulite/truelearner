# FFS-SAME0 implementation and development audit

Protocol: `ffs-same0-learned-correspondence-v1`

Status: implementation and development gates frozen; definitive scientific
matrix not run.

## Frozen references

- parent FFS0 outcome tag: `ffs0-full-fractal-scaling-positive`;
- parent FFS0 outcome commit:
  `2fd56376e32a403b5e1fac6dbdbc6f21e4f2645d`;
- FFS-SAME0 protocol tag: `ffs-same0-learned-correspondence-protocol`;
- FFS-SAME0 protocol commit:
  `95bd903ab22e24d299e0dbc3621b34e3e1da5c78`;
- FFS-SAME0 protocol SHA-256:
  `392f92a0074353c67c2c7bb0f18a22770e47729e7d3ddd9abfe6fef63cec249b`;
- implementation source commit before this audit:
  `ba1f121d252a233fd71bbd27681931ce7adae159`;
- FFS-SAME0 kernel/harness SHA-256:
  `296a1233042c631765cd97ab3ed2d6c25fd7bf93c6f7759fb7b5e382fc56885a`;
- FFS-SAME0 binary SHA-256:
  `08478d73c868f224aad08c8579bbc1788b30687fe85d998314e710fa82d69d35`;
- module registry SHA-256:
  `989956d68f8b48bfe3d62af27a797cc9c20b3b2637135fb9180e4dcf9730d19d`.

The frozen parent artifacts remain byte-identical:

- FFS0 definitive CSV:
  `74f0a92442aa71a60cedf047feeadbebe389586210959b758a7d2cf6fd43db56`;
- FFS0 definitive Markdown:
  `58d2c1efc124bdb481b43317ed2f373926d0da308b41778de0aacd1a79f3e0c2`;
- FFS0 outcome audit:
  `13b1d292a3f73889c682e72d8797109d775a79c6d4f978ab90a3ba1b0bd3c20d`.

Neither `results/ffs_same0_learned_correspondence.csv` nor
`results/ffs_same0_learned_correspondence.md` exists. MICRO and GATE cannot
write them; definitive mode uses create-new semantics and refuses overwrite.

## Additive implementation boundary

Relative to the positive FFS0 outcome, the implementation adds:

```text
experiments/ffs_same0_learned_correspondence_protocol.md
src/ffs_same0.rs
src/bin/ffs_same0_learned_correspondence.rs
one additive module declaration in src/lib.rs
```

No frozen FFS0 or earlier source body or result artifact changed. No old
definitive runner is called. Legacy regressions were not repeated because the
implementation is additive and the frozen sources and artifacts were verified
byte-for-byte instead.

## Structural identity separation

The module is split by explicit source markers into an organism-visible kernel
and evaluator/harness. The evaluator-only type `TruthFillerId` is declared
after the kernel boundary. It never appears in the kernel.

The organism-visible observation contains only:

```text
fresh OccurrenceId
two local event positions per relational atom
temporal lag
anonymous causal channel
local context marker
```

Every evaluator observation mints new occurrence values from an allocation
namespace, episode seed, and local ordinal. Evaluator truth does not enter that
function. Independently relabeling evaluator truth therefore leaves the entire
anonymous view exactly equal.

Within a candidate correspondence, prior and current observations always have
different occurrence identities. The occurrence-to-truth map exists only in
`EvaluatorEpisode` below the kernel boundary and is consulted only after an
attempt or execution has produced a current occurrence.

## Persistent-state audit

The only persistent correspondence state is:

```text
RelationMotif {
    context,
    two RelationAtom values
}
+ ordinary integer strength
```

The motif contains anonymous local positions, causal channels, and lags. It
contains no occurrence, evaluator truth, seed, episode, concrete payload,
future identifier, pointer, canonical representation, or result.

The recursive execution asset remains an ordinary `Arrow` containing only
CELL/ARROW identities, direct dependency-arrow identities and fingerprints,
role-relative residual effects, local compatibility, and strength. No
occurrence identity is stored in either persistent type.

Temporary binding stores one current occurrence capability for one invocation.
Every effect uses that capability within the invocation; the evaluator maps it
to truth afterward. Repeated held-out use leaves both persistent stores exactly
unchanged.

## Relational correspondence learning

Each candidate link contains a two-atom temporal/causal motif. A valid motif and
its decoy share one superficial causal atom. The second atom distinguishes
continuity. A separate valid motif has a different full causal shape but the
same continuity consequence.

Acquisition explores every locally available candidate. Each candidate is
physically executed through the same depth-8 CELL/ARROW/SPIKE task queue using
a provisional temporary binding. The evaluator observes the resulting terminal
consequence and supplies only success/failure:

```text
success  +2
failure  -1
retain    6
prune    -2
```

The earlier development draft briefly credited candidates directly from the
evaluator map. That was rejected before implementation freeze. The frozen
implementation charges the complete candidate route, binding, effects,
relational observations, comparison, credit, consolidation, and pruning.

Four GATE evidence episodes per candidate arm consolidate exactly two valid
motifs. Decoys are pruned. MICRO deliberately uses two episodes, remains below
threshold, reports zero consolidated motifs, and makes no A-E claim.

## Dynamic leak controls

All seventeen controls pass in GATE. The identity-specific controls establish:

```text
occurrence relabeling                 invariant
allocation-order perturbation         invariant
backing-memory order perturbation      invariant
same shape / different continuity      rejected
different shapes / same continuity     accepted
cross-invocation occurrence reuse      absent
deliberately injected reused token      detected
evaluator-truth relabeling              anonymous view unchanged
missing correspondence                  no concrete effect
ambiguous correspondence                no concrete effect
changed context                         stale rule does not bind
historical context return               old rule reused
persistent state during use             unchanged
```

The reused-token detector is not a source-name check. It injects a prior
occurrence from one observation into a later observation and requires the
freshness audit to fail.

The source audit separately scans only the organism kernel for truth/filler,
supplied equality, canonical or stable payload, correlation/workspace/future
object identifiers, evaluator levels/process/results, and economics. It passes.

## Recursive execution development result

After correspondence consolidation, fresh anonymous occurrences resolve into
temporary role bindings and support the same level-blind substitution cycle.
The development scale law is:

| Cell | Depth | Population | Learned motifs | Retained useful depth | Collapse point | Censored |
|---|---:|---:|---:|---:|---|---|
| S0 | 8 | 16 | 2 | 0 | compaction | no |
| S1 | 32 | 64 | 2 | 3 | none | no |
| S2 | 128 | 256 | 2 | 5 | none | no |
| S3 | 512 | 1,024 | 2 | 6 | none | yes |
| depth-only | 128 | 64 | 2 | 5 | none | no |
| population-only | 32 | 1,024 | 2 | 3 | none | no |

For every development edge:

```text
observable trace equal       true
child work < immediate parent
removed ordinary firings > 0
finite marginal break-even
structural = justified = realized depth
over-retained = under-retained = 0
```

The parent-relative work chains are:

```text
S1  162 -> 120 -> 83 -> 70
S2  594 -> 360 -> 227 -> 166 -> 137 -> 124
S3  2322 -> 1320 -> 803 -> 550 -> 425 -> 364 -> 335
```

The corresponding zero-price edge horizons are:

```text
S1  16, 12, 23 uses
S2  11, 11, 14, 20, 35 uses
S3  10, 10, 12, 16, 24, 41 uses
```

The exact same S1 asset transfers to depth 128 and population 1,024 with zero
new acquisition. Adaptive fallback distances are `0, 1, 2, 0` for stable,
child-own change, direct-parent change, and historical return; every trace is
exact and return charges zero reacquisition.

## Identity economics development result

Correspondence acquisition is 860 work and retains 26 bytes for two motifs.
Mature correspondence resolution costs 18 work per invocation.

Against generic anonymous two-candidate task search, learned correspondence
reaches finite internal break-even:

| Cell | Generic runtime | Learned primitive runtime | H* |
|---|---:|---:|---:|
| S0 | 96 | 54 | 21 |
| S1 | 312 | 162 | 6 |
| S2 | 1,176 | 594 | 2 |
| S3 | 4,632 | 2,322 | 1 |

Against the frozen supplied-SAME FFS0 mature hierarchy, however:

| Cell | Supplied SAME | SAME-less | Delta |
|---|---:|---:|---:|
| S0 | 36 | 54 | +18 |
| S1 | 52 | 70 | +18 |
| S2 | 106 | 124 | +18 |
| S3 | 317 | 335 | +18 |

Because the per-use delta is positive at every anchor, no reuse horizon can
repay the SAME-less stack against supplied SAME. Development D is therefore
`EXPENSIVE`. Nothing was tuned to remove this tax.

This is the intended necessity/efficiency distinction: development indicates
that supplied filler equality is not logically required for the tested
execution hierarchy, while remaining a valuable computational prior.

## Process availability

Development process status remains:

```text
execution   positive
learning    unavailable
retrieval   unavailable
decision    unavailable
```

No process adapter, retrieval matcher, semantic event class, or synthetic
learning/decision trace was added. E is therefore `PARTIAL`.

## E2B validation

Persistent sandbox: `iv7qfq154p7ffq4xpxw0o`

The clean implementation commit was validated with:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -q --lib ffs_same0
cargo test --release -q --bin ffs_same0_learned_correspondence
cargo run --release --bin ffs_same0_learned_correspondence -- --micro
cargo run --release --bin ffs_same0_learned_correspondence -- --gate
```

The remote chain exited zero:

- kernel/harness tests: 5 passed, 0 failed;
- binary/schema/accounting tests: 2 passed, 0 failed;
- formatting: PASS;
- all-target clippy: PASS;
- MICRO: PASS, development only, A-E not tested;
- GATE: PASS, development only;
- frozen ancestry: PASS;
- duplicate determinism: PASS;
- source audit: PASS;
- all dynamic identity-leak audits: PASS;
- scaling and orthogonal-depth signatures: PASS.

## Development status and stopping boundary

```text
A  correspondence reconstruction     PASS development
B  functional binding/execution      PASS development
C  recursive fractal recovery        PASS development
D  identity economics                EXPENSIVE development
E  process availability              PARTIAL development
```

These are implementation-development results only. They support freezing the
implementation for a single preregistered definitive matrix; they do not
consume or predict the scientific outcome.

No definitive FFS-SAME0 command has run. No result artifact exists. Any later
definitive run must start from the exact implementation tag produced with this
audit, execute once, and preserve whatever A-E outcomes it produces.
