# PX-C continuous seam baseline v1 audit

Status: development measurement frozen. No PX4--PX8 mechanism or serial
authority claim is advanced by this audit.

## Provenance

- PX3+LR-C authority ancestor:
  `f9057fe78a86db9111b0b69310d03accef3bc970`
- audit implementation exercised for the baseline:
  `2fddee7398c09b533386f84a92f1aa140f817add`
- committed baseline artifacts:
  `76c191f8ecbed30daf6eaa8df67bed5e6af58477`
- tag: `pxc-continuous-seam-baseline-v1`
- successful E2B sandbox: `inzoif3rnsw1tu4dby448`
- active-surface manifest SHA-256:
  `472440f5e989387044fa3d36c5364b2d65f30d01659742a829d007cb67f7ef9a`

The first archive preflight stopped before measurement because a Git archive
does not contain `.git`. The wrapper was corrected to require an explicit
audited commit for archive execution.

A later preflight returned zero only because `rg` was absent in the E2B image
and the scanner had conflated a missing search backend with no matches. Those
generated files were rejected and removed. The scanner was changed to use
`rg` when present, otherwise verified `grep`, and to fail closed on search
errors. The accepted baseline used `grep`.

## Accepted baseline

| category | occurrences |
|---|---:|
| typed episode | 66 |
| typed history | 7 |
| typed query | 14 |
| `begin_episode` | 1 |
| `erase_temporary` | 1 |
| seed-built development | 109 |
| explicit mechanism invocation | 72 |
| typed layer handoff | 98 |
| **total** | **368** |

The 368 occurrences occupy 295 unique manifested source lines.

| layer | occurrences |
|---|---:|
| PX0--PX3 + LR-C physical foundation | 0 |
| PX4 predecessor | 71 |
| PX5 predecessor | 14 |
| PX6 predecessor | 37 |
| PX7 predecessor | 136 |
| PX8 predecessor | 110 |

This is the intended shape: the already-authoritative physical foundation is
free of the audited seams, while every remaining occurrence belongs to a
predecessor surface that a PX4--PX8 lane must replace.

## Replay

The committed baseline at `76c191f...` was uploaded as a new immutable source
snapshot to the same persistent E2B worker. The audit ran with
`PXC_MAX_SEAMS=368` and reproduced:

```text
TOTAL_OCCURRENCES,368
UNIQUE_SOURCE_LINES,295
```

Both serialized predicates were checked in the sandbox. The ceiling passed.

## Artifact hashes

- report:
  `499cd0b43790bbbee906e0738eae982369b2435af933070ef8a6bab8256e9093`
- inventory:
  `f40ca354be9c59e77f376064baf1578154250f7c70cd57f0144ea2b9a45cdbbf`
- summary:
  `a76bcf979f46f004b2d8ff97c620aa56ca62739fba7a149e28df4cc9f77626ae`

## Interpretation boundary

The baseline is not evidence that PX4--PX8 fail. It measures the supply still
present before their cumulative physical replacements exist. Parallel lanes
may develop ahead, but only serial authority may replace a manifested
predecessor and spend a lower PX-C count.
