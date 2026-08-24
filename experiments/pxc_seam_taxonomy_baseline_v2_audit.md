# PX-C seam taxonomy baseline v2 audit

Status: development audit frozen. The v1 368/295 reference is unchanged, and
no PX4--PX8 authority is advanced.

## Frozen lineage

- PX3+LR-C authority: `f9057fe78a86db9111b0b69310d03accef3bc970`
- taxonomy protocol implementation: `7fa611afb60db97c5f8a3f75ff80abb4efee5973`
- taxonomy baseline commit: `8c5e8b7e5615528807510d18ead7580b9248f2cc`
- tag: `pxc-seam-taxonomy-baseline-v2`
- fresh E2B sandbox: `iqxox8l9afkmbvoren5kq`
- manifest SHA-256:
  `472440f5e989387044fa3d36c5364b2d65f30d01659742a829d007cb67f7ef9a`

The v2 script verifies all three frozen v1 artifact hashes before scanning.
Their hashes remained:

- report:
  `499cd0b43790bbbee906e0738eae982369b2435af933070ef8a6bab8256e9093`
- inventory:
  `f40ca354be9c59e77f376064baf1578154250f7c70cd57f0144ea2b9a45cdbbf`
- summary:
  `a76bcf979f46f004b2d8ff97c620aa56ca62739fba7a149e28df4cc9f77626ae`

## Primary taxonomy

| kind | count |
|---|---:|
| typed representation | 87 |
| explicit mechanism invocation | 72 |
| episode/reset boundary | 1 |
| seed/history synthesis | 61 |
| semantic condition | 38 |
| manual temporary cleanup | 1 |
| typed handoff | 98 |
| evaluator-derived input | 10 |
| **sum** | **368** |

The primary kinds are mutually exclusive and exhaustive over the frozen
headline inventory. The layer totals remain PX0--PX3+LR-C 0, PX4 71, PX5 14,
PX6 37, PX7 136, and PX8 110.

## Relocation guards

The independent active-surface scans established:

```text
semantic-condition guard    218
evaluator-input guard       752
```

These are occurrence counters, may overlap, and do not alter the immutable
headline total. They are deliberately broader than the primary classification
so a renamed semantic adapter remains visible.

## Ceiling replay

The committed baseline was uploaded as a new immutable E2B snapshot and rerun
with all three ceilings active:

```text
PXC_MAX_TOTAL=368
PXC_MAX_SEMANTIC_GUARD=218
PXC_MAX_EVALUATOR_GUARD=752
```

The replay reproduced all counts. E2B also verified exact SHA-256 equality for
the taxonomy inventory, guard inventory, and summary:

- taxonomy inventory:
  `b19bf54d7d3133cca0caf98ecca89d483499cae8a6fe53ac0faac464df186441`
- guard inventory:
  `471905f91806a0fa9b4bb9419653e8a98b0e0cb1784638b1ff5e7f6414b5f1d8`
- summary:
  `ccfb10e50e491067fbd7e52157161f6a096e69f5e5e6b832245ce876c730c607`

## Readiness rule

After each lane reaches development readiness, its new versioned manifest must
prove complete candidate-surface coverage and the E2B audit must record:

1. total and unique-line counts;
2. all primary-kind counts;
3. both relocation guards;
4. all layer counts; and
5. the exact manifest hash.

A functional positive is not a physicalization reduction if any of the three
ceilings increases. This prevents disappearance by renaming, relocation, or a
new semantic adapter.
