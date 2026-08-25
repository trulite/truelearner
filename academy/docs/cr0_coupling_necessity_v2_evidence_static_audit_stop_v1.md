# CR0 coupling-necessity v2 evidence and static-audit stop v1

Status: immutable positive physical matrix; final CR0 handoff blocked only on a
post-evidence static-audit false negative.

Frozen evaluator: `cr0-coupling-necessity-v2-frozen-v1` (`6fb360f`).

E2B evidence sandbox: `igqql0x2kvj39alq8w063`.

## Physical matrix

The sole v2 execution passed:

- `400/400` physical cases;
- `800/800` mechanics rows;
- `240/240` retained-behavior cases;
- `160/160` efficacy-control cases;
- exact same-mechanics replay everywhere;
- exact Reference/Production physical equality within every arm;
- all functional predicates;
- natural quiescence everywhere;
- maximum PhysicalWork `66`.

Both CSV header and rows contain 26 fields. There are zero replay, mechanics,
predicate, or case failures.

Evidence hashes:

```text
matrix a44eb399095609e5f2fa9cd3b4a0250f15f341d2139056a0aed75168053af07f
report 0987ef90b02dd277ae378a5411ecb938e4dfe82bad5a66098b9024947eed0ee1
```

At threshold 2, every persistence-only row serialized:

```text
post state      resistance 4 / coupling 1
baseline state  resistance 1 / coupling 1
target fires    0 / 0
outward         0
```

Every efficacy-plus-persistence row serialized:

```text
post state      resistance 4 / coupling 2
baseline state  resistance 1 / coupling 1
target fires    1 / 0
outward         1
```

The threshold-1, threshold-3, and two-input topology controls all passed.

## Static-audit stop

The subsequent static audit stopped before its PASS marker. All six protected
source/artifact hashes passed. Its FD1 anchor predicate then expected forty
rows containing the literal point:

```text
after_consequence@5/5:1/4/
```

It found four: the two roots times two mechanics at construction phase zero.
The remaining 36 valid rows preserve age 5 but have phase-shifted absolute
ticks 6 through 14. The correct source-evidence predicate is:

```text
after_consequence@5/<absolute tick>:1/4/
```

That predicate finds exactly forty rows in the same frozen FD1 artifact. This
is a source-audit regex defect; it does not touch or reinterpret any CR0 v2
physical row.

The v2 matrix must not rerun. A separately preregistered audit-only repair may
change that one regex and rerun only the static audit.
