# PXR0 single-file physical-runtime development v2 result audit

Status: **DEVELOPMENT READY; SUCCESSOR AUTHORITY UNSPENT; PX-C UNSPENT**.

The complete frozen v2 evaluator ran exactly once from targeted-validation
commit `1d09e0b5a81a5c862c12f4801a6bd2e06d520e14` in fresh E2B sandbox
`i8varrm5e7vgnj7cuntcw`, state file
`/Users/satya/.cache/truelearner/pxr0-v2-matrix-20260824-a.json`. The sole
project command was:

```text
cargo run --release --manifest-path arms/pxr0-successor-readiness-v2/Cargo.toml
```

No rescue, rerun, row filter, schedule adjustment, predicate adjustment, or
runtime edit occurred.

## Development result

- phase-preserving invariance rows: `16/16`;
- unchanged functional row clauses: `384/384`;
- phase-changing safety controls: `12/12`;
- phase-control safety clauses: `72/72`;
- global clauses: `10/10`;
- complete result: `466/466`;
- maximum per-advance work: `15039 < 20000`;
- maximum resident memory: `6000 < 8192` bytes;
- natural quiescence on every advance: `true`;
- independent complete-trial exact replay: `true`;
- PXR0 successor authority: `false`; PX-C authority/evidence: `false`.

All 24 cumulative clauses are true in every phase-preserving row: blank
silence; generic bootstrap identity; qualified paired update; participation
specificity; reverse and reflection controls; learned ordered and nested
reuse; unsupported adjacency silence; retained resistance; nonparticipating
silence; qualified Modulation; Drive-without-return separation; ordinary Drive
propagation; later-arrival initiation; exactly-once complete and duplicate
outward crossings; incomplete/blocked/open/branch/cycle silence; stale silence;
one bounded reproposal under changed experience; quiescence; work/memory bounds;
exact replay; and the cumulative PX0-PX8+LR-C conjunction.

## Exact schedule geometry

Every world first advanced an empty substrate to its construction tick, then
created topology, then applied unchanged relative arrivals.

The 16 invariant rows use origins `0,130,260,390`, four times each. In every
row, origin modulus is zero; construction tick, pressure origin, and first
arrival tick equal the origin; both recorded deltas are zero. Every row has the
same registered functional observation: paired updates `2`, returned impulses
`2|2`, formation updates `12`, complete outward `1`, duplicate outward `1`,
all registered negative outward counts `0`, stale outward `0`, and exactly one
fresh proposal.

The 12 intentionally phase-changing controls use origins
`3,6,9,133,136,139,263,266,269,393,396,399`. Construction and first arrival
equal origin; pressure origin is respectively the preceding multiple of ten;
construction-minus-pressure repeats `3,6,9`; first-arrival-minus-construction
is zero. All 12 observations differ from their matching phase-zero row:

- modulus 3: paired updates `2`, impulses `2|2`, formation `10`, complete and
  duplicate outward `0`;
- modulus 6: paired updates `0`, impulses `0|0`, formation `5`, complete and
  duplicate outward `0`;
- modulus 9: paired updates `0`, impulses `0|0`, formation `12`, complete and
  duplicate outward `1`.

These are serialized lawful phase effects, not invariance failures. Across all
controls, incomplete, blocked, open, branch, cycle, and stale outward counts
remain zero; changed experience creates one proposal; replay, quiescence,
work, and memory safety all pass.

## Immutable surface and artifact hashes

The canonical runtime remains 474 lines, 13 types, 15 functions/methods, and
one active source file. Its bytes remain unchanged at SHA-256
`f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb`.
The active primary, semantic, evaluator, new-kind, and new-surface state is
zero; no identity or physical-law ambiguity appeared.

| artifact | SHA-256 |
|---|---|
| invariance CSV | `d1bf714bdf24bbee10c362727abec02f42066cedd05ee807c88ef2c645a96d5e` |
| phase-control CSV | `6900a8d6a5a504bed95ea729acec522c5cf28e30169779cad9d34f76588fbb7f` |
| result Markdown | `82b234bb9db445922885af29fd1b31097057372dc8417ddaa88e75cea4758848` |

This result establishes development readiness only. Work stops here for joint
human review of the exact canonical Rust file. It does not establish or spend
PXR0 successor authority and it does not authorize PX-C.
