# CJ0-OR ordinary convergence GATE v1 result audit

Status: **POSITIVE GATE FROZEN; DEFINITIVE EVIDENCE UNSPENT; AUTHORITY UNCHANGED**.

## Sole execution and artifacts

The preregistered GATE command executed exactly once after positive PROBE and
MICRO freezes, from unchanged implementation source SHA-256
`fc16f376d6ba34ef59add84bddace1d9a3360f237cf0019866a2632c58bcef43`:

```text
cargo run --release -p px0-physical-correspondence \
  --example cj0_or_ordinary_convergence -- --gate
```

It emitted exactly one
`CJ0_OR_ORDINARY_CONVERGENCE_GATE_V1_EVIDENCE_SPENT` marker and exited `0`.
There was no rerun, source change, rescue, or regeneration.

| artifact | SHA-256 | bytes |
|---|---|---:|
| `results/cj0_or_ordinary_convergence_gate_v1.csv` | `dd2e92b3b68b0e23bf35e082fc79b452313cc08482fdb4bc5243b576b94b09a9` | 18,558 |
| `results/cj0_or_ordinary_convergence_gate_v1.md` | `53922f9910118c9ba12362ac51e0f2f35feb8b4d695be7ee9497028af1dcbf54` | 1,101 |

Both staging paths are absent.

## Conjunctive result and coverage

- rows: `72/72` passed;
- independently serialized `P0-P13`: `1,008/1,008` passed;
- full exact replays: `72/72`;
- simultaneous refractory-suppressed rows: `12`, all exactly one downstream
  pulse;
- positive-skew rows: `60`, all exactly two bounded downstream pulses;
- route delay bins `0..5`: exactly `12` rows each;
- source skew bins `0..4`: `12|12|18|18|12` rows;
- CELL-handle permutations / layouts: all `24` / all `4`;
- ARROW allocation / SPIKE insertion orders: both / both;
- identity rotation, mirror placement, and phase-pattern values: all covered;
- blocked, stale, absent, and threshold-`2` controls: all exact;
- natural quiescence, inert follow-up, bounded source firing, and no runaway:
  every execution;
- incidental structural proposals: zero.

The GATE therefore discriminates the intended ordinary convergence result
from both artifacts: exact isolated-route sufficiency rules out the
threshold-`2` saturation explanation, while the serialized `12/60` split
shows that one-versus-two output cardinality follows refractory timing and is
not hidden or claimed invariant.

## Independent accounting audit

The CSV contains one `49`-field header and `72` `49`-field data rows. All final
bits are true; the claim-count column sums to `1,008`. Independent column sums
match the report:

| ledger | total |
|---|---:|
| external SPIKEs | 8,640 |
| source CELL firings | 2,160 |
| convergence / downstream / crossing | 1,056 / 1,056 / 1,056 |
| stale-route deallocations | 288 |
| incidental proposals | 0 |
| CELL / ARROW instances | 6,624 / 4,680 |
| aggregate persistent substrate bytes | 617,472 |
| work ledger | 96,097 operations |

Result storage is `19,659` bytes. Every clone, replay, physical control,
threshold control, and idle propagation is included. No accounting correction
is required.

## Classification and next action

GATE is a positive development boundary, not the final CJ0-OR classification
and not an authority advance. Freeze it separately at tag
`cj0-or-ordinary-convergence-gate-v1-positive`. The preregistered `120`-row
definitive command may then execute exactly once from the same frozen source.
Any definitive failed clause is an immutable scientific negative; it may not
be rescued or rerun.
