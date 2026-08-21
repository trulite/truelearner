# RE0 definitive outcome audit

Protocol: `reflected-compaction-economics-re0-v1`

Outcome: RE0 positive; economically useful one-level fractality under the
preregistered physical accounting and workloads.

## Frozen execution boundary

- parent outcome tag: `rc0b-grounded-motif-substitution-positive`;
- parent outcome commit:
  `f6dcec833966347ce360f3ec126202b4843319d5`;
- RE0 protocol tag: `re0-reflected-compaction-economics-protocol`;
- RE0 protocol commit:
  `4d6587b0f0bfe46ed3ceb919c4c3805c6c473d09`;
- RE0 implementation tag:
  `re0-reflected-compaction-economics-implementation`;
- frozen implementation commit:
  `c7455588e1e04b6e39c245b898cca1c8fc353dd0`;
- implementation source commit:
  `65aa542263ae7e1149e6578c46b749a38078419c`;
- persistent E2B sandbox: `iv7qfq154p7ffq4xpxw0o`.

Before execution, HEAD exactly matched the implementation tag, the worktree was
clean, and neither RE0 result path existed. The following claim-eligible
command was executed once:

```text
cargo run --release --bin reflected_economics_re0 -- --definitive
```

The runner reconstructed acquisition twice per frozen seed, consumed the
already-frozen RC0b runtime artifact without rerunning an RC0b runtime cell,
created both RE0 artifacts with write-once semantics, and exited zero. No
second definitive RE0 command was executed.

## Frozen artifacts

- `results/re0_reflected_economics.csv`
  SHA-256:
  `93c02acd71fc8dd642839fd31f84e18af385858efdf731a7fbce758c89c8d36b`;
- `results/re0_reflected_economics.md`
  SHA-256:
  `a93fc8304782d2af112fa0cf9147b961e98a696d395a3ae9f02cad073c60e0b5`.

The parent RC0b artifacts retained their frozen hashes:

- CSV:
  `285ee87a7a77ea26b154cb728c63b1a53530891ae3ffd370e990dbd38e93f97e`;
- Markdown:
  `fb3fa0bb7b351c4ddc77a8e83fe60f42ad29d57a64fc1edfbbbcc34f24018b72`.

The RE0 CSV has one consistent 25-column schema and exactly 12,611 data rows:

```text
economic rows       840
horizon rows      11,760
gate rows             11
```

An independent read-only audit recomputed every economic gain, exact ceiling
break-even, cycle-to-invocation conversion, and signed horizon delta from the
serialized inputs. All values reconcile. All eleven gate names are unique and
all eleven statuses are `PASS`.

## Acquisition and persistent asset

The primary view charges the complete observed blank-start stack once per
seed:

```text
RP0a acquisition + RC0a consolidation + RC0b motif acquisition
```

| Seed | RP0a | RC0a | RC0b | Full acquisition |
|---:|---:|---:|---:|---:|
| 0 | 10,740,059 | 1,148 | 2,638 | 10,743,845 |
| 1 | 4,452,016 | 1,220 | 2,638 | 4,455,874 |
| 2 | 21,480,745 | 1,244 | 2,638 | 21,484,627 |
| 3 | 16,321,757 | 1,268 | 2,638 | 16,325,663 |
| 4 | 15,321,640 | 1,196 | 2,638 | 15,325,474 |
| 5 | 23,786,489 | 1,148 | 2,638 | 23,790,275 |
| 6 | 18,869,627 | 1,100 | 2,638 | 18,873,365 |
| 7 | 2,676,006 | 1,148 | 2,638 | 2,679,792 |

Full acquisition spans `2,679,792..23,790,275` work, with mean
`14,209,864.375` and median `15,825,568.5`. This large frozen RP0a variation
is why the definitive full-stack horizons are orders of magnitude larger than
the prebuilt development fixture's horizons.

Every seed reconstructed RP0a parity, four RC0a compiled arrows, one RC0b motif
with three shortcuts, and the same role-relative motif fingerprint
`11454459317923793212`. The motif is charged once per seed and reused across
all six depths; no independent depth acquisition exists.

Persistent storage is:

```text
RP0a learned roles/program   6,608 bytes
RC0a compiled dispatch         160 bytes
RC0b motif                     148 bytes
full retained asset          6,916 bytes
```

Persistent installation is zero because installation-producing consolidation
is already charged. Mature maintenance is zero because invocation is read-only.
Temporary validation, binding, installation, and invocation remain inside the
frozen RC0b runtime work and are not double counted.

## Mature physical phase boundary

At zero carrying price, the frozen mature work per cycle is:

| Workload | Concrete | Motif | Physical saving |
|---|---:|---:|---:|
| depth 5 | 468 | 773 | -305 |
| depth 8 | 756 | 998 | -242 |
| depth 16 | 1,700 | 1,774 | -74 |
| depth 32 | 4,356 | 4,094 | 262 |
| depth 64 | 12,740 | 11,806 | 934 |
| depth 128 | 41,796 | 39,518 | 2,278 |
| balanced six-depth cycle | 61,816 | 58,963 | 2,853 |

The shallow workloads have nonpositive per-use gain, so no nonnegative reuse
horizon can amortize them. The deep and balanced workloads have positive gain,
so finite break-even is possible.

## Full-stack physical break-even

The primary zero-price result is:

| Workload | Minimum H* | Maximum H* | Seeds finite |
|---|---:|---:|---:|
| depth 5 | none | none | 0/8 |
| depth 8 | none | none | 0/8 |
| depth 16 | none | none | 0/8 |
| depth 32 | 10,229 uses | 90,803 uses | 8/8 |
| depth 64 | 2,870 uses | 25,472 uses | 8/8 |
| depth 128 | 1,177 uses | 10,444 uses | 8/8 |
| balanced cycle | 940 cycles | 8,339 cycles | 8/8 |

Each balanced cycle contains six invocations, so its range is
`5,640..50,034` mature invocations. For every positive cell, independent exact
recomputation is strictly positive at `H* - 1` and nonpositive at `H*`; the
reported ceiling is not an approximate crossing.

Thus every frozen seed preserves the preregistered crossover and reaches finite
full-stack break-even at depths `32`, `64`, and `128`, and under the shared
balanced workload. No aggregate average hides a failed seed.

## Carrying-price sensitivity

Zero carrying price is the preregistered primary physical result. The secondary
prices `1, 10, 100, 1000` millionths of work per byte-use do not change the
qualitative phase boundary. At the highest price:

| Workload | Minimum H* | Maximum H* |
|---|---:|---:|
| depth 5–16 | none | none |
| depth 32 | 10,506 | 93,265 |
| depth 64 | 2,891 | 25,662 |
| depth 128 | 1,180 | 10,476 |
| balanced cycle | 954 | 8,462 |

The positive claim does not depend on these priced scenarios, but they show
that the frozen 6,916-byte asset does not erase deep-workload gain throughout
the preregistered sensitivity range.

## Incremental diagnostics

The two secondary ownership views are not substitutes for the primary
blank-start claim. At zero price they report:

| View | Depth 32 | Depth 64 | Depth 128 | Balanced cycle |
|---|---:|---:|---:|---:|
| RC0a+RC0b incremental | 15 | 5 | 2 | 2 |
| RC0b-only incremental | 11 | 3 | 2 | 1 |

These horizons are identical across seeds because seed variability resides
primarily in the RP0a acquisition charge. They quantify reuse when the earlier
asset is already owned; they are not the basis of the RE0 pass.

## Determinism and lifecycle

Both acquisition observations were exact for all eight seeds. All
`2,267,722 / 2,267,722` temporary workspaces were destroyed, maximum live
workspace count was two, permanent fingerprints were stable, and the single
depth-general asset remained unchanged during accounting.

The definitive execution added no capability and changed no runtime behavior.
It measured acquisition and storage, consumed frozen mature runtime work, and
performed exact accounting only.

## Outcome and supported claim

RE0 is positive under its preregistered conjunctive gate.

The supported narrow claim is:

> The depth-general one-level learned reflective asset repaid its complete
> observed acquisition cost after finite reuse under physical zero-carrying
> accounting, while preserving the preregistered depth phase boundary.

Together with frozen RP0a, RG0a, RC0a, and RC0b, this establishes economically
useful one-level fractality under the tested workloads and accounting. It does
not establish benefit for shallow workloads, a second reflected level,
indefinite recursive improvement, or universal economic advantage.

## Ladder status

```text
RP0a   one-level functional fractality               positive, frozen
RG0a   abstract-to-concrete grounding                 positive, frozen
RC0a   reflected interpreter tax removed              positive, frozen
RC0b   lower computation eliminated; runtime wins     positive, frozen
RE0    full acquisition and persistence amortize      positive, frozen
F1     scientifically permitted; not implemented
```

No F1 code or second reflected level was added.
