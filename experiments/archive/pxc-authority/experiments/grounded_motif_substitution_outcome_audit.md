# RC0b definitive outcome audit

Protocol: `grounded-motif-substitution-rc0b-v1`

Outcome: RC0b-A positive; RC0b-B positive.

## Frozen execution boundary

- protocol tag: `rc0b-grounded-motif-substitution-protocol`
- protocol commit: `4ae4378eb5df95c9f1e829db58f3c7ba5c341b9b`
- implementation tag: `rc0b-grounded-motif-substitution-implementation`
- frozen implementation commit:
  `d50faa66a0c06258cbcde792103b3f9d7522e521`
- implementation source commit:
  `9d0813fa15a10736eb7a4e8738cfac0fe9b256d2`
- persistent E2B sandbox: `iv7qfq154p7ffq4xpxw0o`

Before execution, HEAD exactly matched the implementation tag, the worktree was
clean, and neither result path existed. The following claim-eligible command
was executed once:

```text
cargo run --release --bin grounded_motif_substitution -- --definitive
```

The runner created both artifacts with `create_new` semantics and exited zero.
No second definitive command was executed.

## Frozen artifacts

- `results/rc0b_grounded_motif_substitution.csv`
  SHA-256:
  `285ee87a7a77ea26b154cb728c63b1a53530891ae3ffd370e990dbd38e93f97e`
- `results/rc0b_grounded_motif_substitution.md`
  SHA-256:
  `fb3fa0bb7b351c4ddc77a8e83fe60f42ad29d57a64fc1edfbbbcc34f24018b72`

The CSV contains all 528 expected result rows:

```text
8 seeds × 6 depths × 11 arms = 528
```

Every result row has the frozen 19-column schema. The 15 gate rows contain one
extra trailing field: the gate status is serialized as a twentieth unnamed
field rather than under the final header. All 15 trailing values are `PASS`.
This is a presentation-only CSV defect; the raw write-once artifact is retained
unchanged. The named result fields, Markdown artifact, runner stdout, and gate
statuses agree. The experiment was not rerun and the artifact was not repaired.

## Frozen matrix

- integrated reconstructed seeds: `0..7`;
- motif acquisition depths: `3, 4, 6`;
- held-out depths: `5, 8, 16, 32, 64, 128`;
- fresh episodes per seed/depth: `16`;
- ordinary mature evaluations per arm: `768`;
- parallel cells: `8`;
- workspaces destroyed: `1,191,589 / 1,191,589`;
- maximum live workspaces per cell: `2`.

Frozen ancestry reconstructed successfully. FULL RC0a and concrete totals
exactly reproduce their earlier definitive totals:

```text
concrete       7,912,448
FULL RC0a      8,192,000
```

## RC0b-A: computational compaction

```text
FULL RC0a       8,192,000
motif           7,547,264
delta            -644,736
reduction          7.8703%
```

Every one of the 48 seed/depth cells had `W_sub < W_full`; no aggregate mean
hides a failed cell. All 768 motif episodes were correct, quiescent,
activity-limit free, and exactly equal to FULL RC0a over the preregistered
observable interface.

The physical deletion audit is exact:

```text
motif shortcut firings             97,920
relay activations eliminated       97,920
ordinary route firings eliminated  97,920
```

Thus the work reduction is missing substrate execution, not a renamed counter
or another interpretation optimization.

Per-depth RC0b-A work was:

| Depth | FULL RC0a | Motif | Delta | Reduction |
|---:|---:|---:|---:|---:|
| 5 | 106,496 | 98,944 | -7,552 | 7.0913% |
| 8 | 143,360 | 127,744 | -15,616 | 10.8929% |
| 16 | 264,192 | 227,072 | -37,120 | 14.0504% |
| 32 | 604,160 | 524,032 | -80,128 | 13.2627% |
| 64 | 1,677,312 | 1,511,168 | -166,144 | 9.9054% |
| 128 | 5,396,480 | 5,058,304 | -338,176 | 6.2666% |

RC0b-A is positive.

The supported claim is:

> Repeated grounded computation earned one executable substrate-native,
> role-relative motif which preserved the complete preregistered observable
> behavior while eliminating genuine lower-level relay execution.

## RC0b-B: economic per-use prerequisite

Across the preregistered balanced mature-use workload:

```text
concrete        7,912,448
motif           7,547,264
delta            -365,184
reduction          4.6153%
```

Every seed independently satisfies the frozen threshold:

```text
per-seed concrete  989,056
per-seed motif     943,408
per-seed delta     -45,648
```

The preregistered per-depth signs show the crossover rather than hiding it:

| Depth | Concrete | Motif | Motif − concrete | Relative |
|---:|---:|---:|---:|---:|
| 5 | 59,904 | 98,944 | +39,040 | +65.1709% |
| 8 | 96,768 | 127,744 | +30,976 | +32.0106% |
| 16 | 217,600 | 227,072 | +9,472 | +4.3529% |
| 32 | 557,568 | 524,032 | -33,536 | -6.0147% |
| 64 | 1,630,720 | 1,511,168 | -119,552 | -7.3312% |
| 128 | 5,349,888 | 5,058,304 | -291,584 | -5.4503% |

The motif remains more expensive at depths `5, 8, 16`, crosses below concrete
by depth `32`, and is cheaper in the frozen balanced aggregate. RC0b-B is
positive under its preregistered threshold.

This is not a full economic result. RC0b-B charges mature invocation work but
does not amortize acquisition, persistent carrying, installation across a use
horizon, or maintenance. It opens RE0; it does not answer RE0.

## Controls

All preregistered controls behaved conjunctively:

- fresh/changed surroundings: `768/768` correct and trace-equal, with 97,920
  motif firings;
- interruption/re-entry: `768/768` correct and trace-equal;
- context-effect invalidation: `768/768` correct and trace-equal, zero motif
  firings, RC0a resumed;
- forced stale same-endpoint control: `768/768` correct endpoints but `0/768`
  trace equality;
- RC0a parent invalidation: `768/768` correct and trace-equal, zero motif
  firings, RG0a resumed;
- subthreshold evidence: `768/768` correct, zero motif firings;
- shuffled recurrence evidence: `768/768` correct, zero motif firings;
- no bindings: `0/768` correct.

The forced-stale result establishes that answer equality cannot satisfy the
gate when an ordered observable intermediate effect is lost. Changed bindings,
re-entry, and both fallback levels rule out a concrete trajectory cache or an
irreversible bypass.

## Ladder status

```text
RG0a     functional abstract-to-concrete grounding       positive, frozen
RC0a     compiled recurrent dispatch                     positive, frozen
RC0b-A   genuine lower computational compaction          positive, frozen
RC0b-B   mature per-use work below concrete              positive, frozen
RE0      opened; acquisition/carrying/maintenance next
F1       remains blocked on RE0
```

No RC0b hierarchy, RE0 implementation, or F1 code was added.
