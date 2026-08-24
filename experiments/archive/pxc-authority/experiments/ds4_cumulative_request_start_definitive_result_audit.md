# DS4 cumulative request/start definitive result audit

Status: **AUTHORITATIVE DEFINITIVE NEGATIVE; WRITE-ONCE OUTCOME FROZEN**.

The single DS4 definitive execution ran from invocation commit
`afbf496222d790abbf284aea14033f8ee062eae2` on dedicated E2B sandbox
`iowhjp1vfvtcbk2hkpqbz`. The sandbox was left running. The runner emitted the
cell-0 outcome-spend marker once, completed all sixteen cells, serialized both
create-new artifacts, and returned process status `1` for the conjunctive
negative. No rerun or rescue occurred.

## Frozen artifacts

- CSV:
  `results/ds4_cumulative_request_start_definitive.csv`;
  SHA-256
  `c6b626650fc199a8ebe2feae8115a8b27071088f963f919393f55e24fbe44a3a`;
- report:
  `results/ds4_cumulative_request_start_definitive.md`;
  SHA-256
  `97f1fc665e03be1ccd398dcdf34fbb262c525aabc23a812c8740c350dd890659`.

The digest of every pre-existing result with the two DS4 artifacts excluded
remained exactly
`97b85f9056a8404fb2caf81e0fa8e3a1cb06398533874a474a9fe2c9696797a4`.

## Conjunctive outcome

```text
matrix cells                         16
cells passing                        14
cells failing                         2
held-out correct                    512 / 512
explicit emissions                  512 / 512
natural queue quiescence            512 / 512
exact duplicate replays              16 / 16
passed numbered controls            190 / 192
source/authority audits              all true
```

The only collapses were:

```text
cell 2, seed 4_200_000
  observed request positions {1,2,3,4,5}
  missing position 0
  first collapse P3 held-out functional transfer
  control 6 false

cell 7, seed 4_700_000
  observed request positions {1,2,3,4,5}
  missing position 0
  first collapse P3 held-out functional transfer
  control 6 false
```

Both cells still learned exactly one request role at episode 8, executed all
`32/32` held-out chains correctly, emitted explicitly, quiesced naturally,
used learned M3 event completion, preserved persistent state, passed every
other control, and replayed exactly.

The preregistered protocol nevertheless required all six serialization
positions **inside every cell**. Incidental 32-episode coverage of only five
positions therefore makes each cell negative. Aggregate coverage, perfect
functional transfer on every sampled episode, or the fact that the omitted
position was exercised in fourteen other cells cannot satisfy or amend that
frozen per-cell clause after evidence was spent.

## Authority state

```text
M3 authoritative
DS0 + DS1 + DS2 + DS3 cumulative definitive positive
DS4 cumulative definitive negative
M4 absent
DS5 cumulative blocked
```

The development mechanism remains frozen and development-positive, but it is
not authoritative. This result cannot be reclassified as a pass and the
matrix cannot be rerun. Isolated DS5--DS8 parts-supplier development may remain
scientifically separate, but the serial authority line cannot advance beyond
M3 from this outcome.

Any future DS4 work must be a separately named, separately preregistered
experiment justified after this negative is frozen. It may not overwrite,
rescue, silently amend, or reuse the authority of this matrix.
