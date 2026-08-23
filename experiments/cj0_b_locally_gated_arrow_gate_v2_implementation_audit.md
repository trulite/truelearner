# CJ0 ARM CJ-B GATE v2 implementation audit

Status: **IMPLEMENTATION FROZEN; GATE V2 UNSPENT**.

## Frozen inputs

| source | SHA-256 |
|---|---|
| candidate physical module `src/lib.rs` | `ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188` |
| frozen GATE v1 evaluator `src/bin/gate.rs` | `84788baec691de2eb5bdce24f19c4456c43ce40717a93edbc11cf42a3e99d61d` |
| frozen GATE v1 CSV | `e76f7256033de9352fac76de72c5ff37a8dcec899c2ff5eec3447d9286604707` |
| frozen GATE v1 report | `08394d47aa7c7c494cc5d5b266868780ecba6009950db1f63eb50112bed1a1d8` |
| frozen GATE v1 negative audit | `874ac29ee7d7a50508152207c7b23bc9089c59d8834da2e8b3e66483d2e5a611` |
| frozen negative handoff | `9f462d4423fcb6838967d52c7471df67756d3f647f0b34fe6ef0b26de4ce011d` |

The authoritative PX0--PX2 sources and all frozen PX3/PX3-R negatives remain
unchanged. GATE v1 retains its original classification.

## Fresh implementation

| addition | SHA-256 |
|---|---|
| GATE v2 protocol | `2f34832522086b393577bfb430c8517a4c9e487a7273a2e1f056f2909aba6539` |
| GATE v2 evaluator `src/bin/gate_v2.rs` | `4733bc8d1cc06881102e0d36067649d22381be554b71056debf36a306b83bb86` |

The evaluator preserves all GATE v1 surfaces with fresh namespaces. Its sole
flat-schedule change is a uniform `changed_offset = 2` field used by both
alternating route orders, all four physical variants, and both timing strata.
No route-specific or outcome-specific branch is present.

The added timing-boundary field uses the same public CELL/ARROW/SPIKE physics.
It begins without a source-to-destination ARROW and records the generic first
proposal, ordinary pressure deallocation, stale generation check, generic
replacement, ordinary return maturation, and held-out execution. No physical
state or law was added.

## Pre-evidence validation

- formatting check: pass;
- focused candidate physical tests: `2/2` pass;
- strict all-target Clippy: pass;
- dependencies: zero;
- physical forbidden-vocabulary scan: clean;
- no-argument and neutral wrong-argument refusal: exit `2` and `2`;
- preflight: zero cells entered and zero artifacts written;
- result and staging paths: absent;
- candidate physical source and all enumerated GATE v1 bytes: exact;
- broad historical workspace suites: intentionally not run;
- later scientific surface: none.

No GATE v2 cell has executed. The implementation is frozen for its sole
preregistered run.
