# CJ0-OR ordinary convergence PROBE v1 result audit

Status: **POSITIVE PROBE FROZEN; MICRO EVIDENCE UNSPENT; AUTHORITY UNCHANGED**.

## Sole execution

The preregistered PROBE command executed exactly once from frozen
implementation commit `37672dc1f268de0078ea582682049b7575109652`, tag
`cj0-or-ordinary-convergence-implementation-v1`:

```text
cargo run --release -p px0-physical-correspondence \
  --example cj0_or_ordinary_convergence -- --probe
```

It emitted exactly one
`CJ0_OR_ORDINARY_CONVERGENCE_PROBE_V1_EVIDENCE_SPENT` marker and exited `0`.
There was no rerun, regeneration, rescue, or implementation change.

## Frozen artifacts

| artifact | SHA-256 | bytes |
|---|---|---:|
| `results/cj0_or_ordinary_convergence_probe_v1.csv` | `5b47bbe6f0fdc5cc7cf7eb26a77b83690f28a9d74078042b8c15f84f9527fccd` | 2,495 |
| `results/cj0_or_ordinary_convergence_probe_v1.md` | `7db0efecbe35990fd247a80c9778d09616cb0eba35fc262777144ddc5f66a64a` | 1,085 |

Both staging paths are absent. The executed source remains exact at SHA-256
`fc16f376d6ba34ef59add84bddace1d9a3360f237cf0019866a2632c58bcef43`.
The authoritative substrate law remains exact at SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

## Conjunctive result

- rows: `8/8` passed;
- independently serialized `P0-P13`: `112/112` passed;
- full exact replays: `8/8`;
- isolated first-route firings: `[1,0,1,1]` in every row;
- isolated second-route firings: `[0,1,1,1]` in every row;
- both-route simultaneous rows: `6`, each `[1,1,1,1]`;
- both-route positive-skew rows: `2`, each `[1,1,2,2]`;
- blocked/stale/absent downstream controls: `0,1,0,1` for each symmetric
  orientation in every row;
- threshold-`2` downstream controls: `0,0,1` in every row;
- natural quiescence and inert idle follow-up: every execution;
- autonomous or runaway refiring: zero;
- incidental local structural proposals: zero.

The simultaneous one-pulse rows are explicitly attributed to the frozen
one-tick refractory rule. They do not supply isolated-route sufficiency: that
is independently established by the two isolated cases. The threshold-`2`
`0,0,1` rows are separately serialized saturation/conjunction controls and
are excluded from the OR claim.

## Independent CSV and accounting audit

The CSV has exactly one `49`-field header and eight `49`-field data rows. No
row has a false final bit; summing the serialized claim-count field yields
`112`. Independent column sums exactly match the Markdown report:

| ledger | total |
|---|---:|
| external SPIKEs | 960 |
| source CELL firings | 240 |
| convergence CELL firings | 108 |
| downstream CELL firings | 108 |
| region crossings | 108 |
| stale-route physical deallocations | 32 |
| incidental structural proposals | 0 |
| constructed CELL instances | 736 |
| constructed ARROW instances | 520 |
| aggregate persistent substrate bytes | 68,608 |
| work ledger | 10,434 operations |

These totals include every candidate execution, exact replay, physical
control, threshold control, and inert follow-up. Result storage is `3,580`
bytes. No accounting correction is required.

## Classification and next action

PROBE establishes the focused opportunity but does not classify CJ0-OR or
advance authority. Freeze this positive separately at tag
`cj0-or-ordinary-convergence-probe-v1-positive`. MICRO may then execute from
the unchanged implementation. A MICRO failure must be frozen and stops GATE
and definitive evidence.
