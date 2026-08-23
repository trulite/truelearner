# CJ0-OR ordinary convergence MICRO v1 result audit

Status: **POSITIVE MICRO FROZEN; GATE EVIDENCE UNSPENT; AUTHORITY UNCHANGED**.

## Sole execution and artifacts

The preregistered MICRO command executed exactly once from unchanged source
SHA-256
`fc16f376d6ba34ef59add84bddace1d9a3360f237cf0019866a2632c58bcef43`:

```text
cargo run --release -p px0-physical-correspondence \
  --example cj0_or_ordinary_convergence -- --micro
```

It emitted exactly one
`CJ0_OR_ORDINARY_CONVERGENCE_MICRO_V1_EVIDENCE_SPENT` marker and exited `0`.
PROBE had already been frozen at commit
`c64b2f99f69233d18598e0126157d27bd1f4e98c`, tag
`cj0-or-ordinary-convergence-probe-v1-positive`. There was no rerun, rescue,
regeneration, or source change.

| artifact | SHA-256 | bytes |
|---|---|---:|
| `results/cj0_or_ordinary_convergence_micro_v1.csv` | `07e2d751908698e17a11c674daf0dda3570650a516ff4981b1f75f51f04c77c9` | 6,534 |
| `results/cj0_or_ordinary_convergence_micro_v1.md` | `c395683306d5522458a1fd9943c31d5b04c46f923efb20a82825096a9cff80f3` | 1,094 |

Both staging paths are absent.

## Conjunctive and variation result

- rows: `24/24` passed;
- independently serialized `P0-P13`: `336/336` passed;
- full exact replays: `24/24`;
- lexicographic CELL-handle allocation permutations: all `24/24` covered;
- route delay bins `0|1|2|3|4|5`: `4|4|4|4|4|4` rows;
- source skew bins `0|1|2|3|4`: `0|6|6|6|6` rows;
- both-route firings: `[1,1,2,2]` in all `24` rows;
- isolated-route, blocked, stale, absent, threshold-saturation, symmetry,
  quiescence, source-bound, and no-runaway clauses: all pass;
- incidental structural proposals: zero.

The absence of simultaneous MICRO rows is the exact preregistered formula for
stage ordinal `1`; simultaneous refractory suppression was already observed
and frozen in six PROBE rows. MICRO specifically expands positive skew,
delays, all handle permutations, identity rotation, mirroring, ARROW
allocation, SPIKE insertion, and phase patterns. No after-the-fact cell was
added.

## Independent accounting audit

The CSV contains one `49`-field header and `24` `49`-field rows. Every final
bit is true and the claim-count column sums to `336`. Independent column sums
match the Markdown report:

| ledger | total |
|---|---:|
| external SPIKEs | 2,880 |
| source CELL firings | 720 |
| convergence / downstream / crossing | 360 / 360 / 360 |
| stale-route deallocations | 96 |
| incidental proposals | 0 |
| CELL / ARROW instances | 2,208 / 1,560 |
| aggregate persistent substrate bytes | 205,824 |
| work ledger | 32,237 operations |

Result storage is `7,628` bytes. All candidate, replay, control, and idle
executions are included; no accounting correction is required.

## Classification and next action

MICRO preserves the distinction between genuine isolated-route sufficiency,
positive-skew pulse multiplicity, simultaneous refractory suppression, and
threshold-`2` saturation. It does not yet classify CJ0-OR or advance
authority. Freeze this result separately at tag
`cj0-or-ordinary-convergence-micro-v1-positive`; GATE may then execute from
the same implementation. Any GATE failure must be frozen and definitive
evidence remains unspent.
