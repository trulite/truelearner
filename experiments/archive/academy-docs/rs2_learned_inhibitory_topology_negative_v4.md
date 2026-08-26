# RS2 learned inhibitory topology immutable negative v4

Status: complete frozen negative. CE1, FD2 v2, and ARC A2 did not run.

Protocol: `0ec2df4` (`rs2-learned-inhibitory-topology-protocol-v4`).

Frozen evaluator: `05f9af0`
(`rs2-learned-inhibitory-topology-frozen-v4`).

One-shot E2B worker: `iheo1t3ieh9r4xx3ilke0`.

## Identity correction result

The disjoint evaluator namespaces fixed the v3 collision. The complete frozen
matrix executed and published both result artifacts. No duplicate physical
identity was attempted or admitted.

## Exact matrix result

- cases: `180/180`;
- rows: `360/360`;
- clauses: `2760/3160`;
- replay exact: `360/360` rows;
- Reference/Production exact: `0/180` cases;
- rows passing their family-specific checks: `320/360`;
- maximum PhysicalWork: `128`.

The result contains exactly two failure classes.

### 1. Representation comparison

All 360 rows record `cross_equal=false`. Pairing every Reference row with its
Production row shows that all serialized observation columns agree except the
live-checkpoint hash. Durable body hashes, final tick, work counters, trace
length, markers, quiescence/ceiling classification, and family predicates are
equal between mechanics.

The frozen comparator nevertheless required the complete `Observation`,
including live-checkpoint bytes, to be equal. RS2 v4 therefore cannot claim
representation equivalence. The CSV does not serialize a physical-trace hash,
so this result does not relabel the checkpoint-hash difference as the only
possible internal `Observation` difference.

### 2. Identity-permutation predicate

All 20 identity-permutation cases fail in both mechanics, producing 40 failed
rows. Training still selects and consolidates the negative relation, removes
the unsupported relation, leaves anchors unchanged, and settles recurrence
with A/B each firing once. The selected generated contact fires twice in the
probe, while the frozen predicate requires exactly once:

```text
failed=learned_negative_traverses
fires=a1/b1/contact2
```

Every other family-specific predicate passes in every root, phase, and
mechanics row.

## Classification boundary

The evaluator-identity defect is resolved. The remaining failures were first
observed only after the complete matrix became executable. V4 is therefore
frozen negative without comparator repair, predicate repair, diagnostic rerun,
or scientific promotion.

No organism source changed in v4. CE1, FD2 v2, the frozen ARC A2 replay,
authority, oracle status, and `arch.md` remain unchanged.

## Artifacts

- matrix SHA-256:
  `36f425811fb55f22305d2f1aa7584a9f6c7d949a647356c4e58c6776436761b5`;
- report SHA-256:
  `354f9a43a4199b5ea3b70f7483f09933b3e1dd98b5ed651da10f30269aa76fbe`.

