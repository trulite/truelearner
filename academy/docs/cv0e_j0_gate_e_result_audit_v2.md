# CV0-E/J0 Gate E result audit v2

Status: development positive; frozen before the full CV0 matrix.

Frozen evaluator: `c688551` (`cv0e-j0-junction-lifetime-frozen-v2`).

E2B evidence worker: `i2wubg6md3f760lzilwr0`.

## Result

- cases: `60/60`;
- Reference/Production rows: `120/120`;
- clauses: `1720/1720`;
- Gate E and boundary controls: PASS;
- Reference/Production ordered observation: exact;
- replay: exact;
- natural quiescence: true;
- maximum PhysicalWork: `21`.

Every positive-selection world recorded:

```text
P -> C+    resistance 1 -> 4
C+ -> X    resistance 1 -> 4
C+ CELL    resistance unchanged at 1

C- links   resistance remains 1, then both deallocate
C- CELL    orphan-deallocates

probe       C+ fires exactly once again
```

The two boundary anchor links remained at resistance `500` immediately after
the selected consequence, proving no credit leakage. Their ordinary local
decay to `499` in the anchor-only age-10 control was identical across mechanics
and produced no delivery, firing, proposal, update, or deallocation.

Allocating the anchor first changed it from `CellId(2)/CellSlot(2)` to
`CellId(0)/CellSlot(0)` without changing selection or any required outcome.

## Artifact hashes

- matrix CSV: `d6a644631bfc63acf68bd4ee1b70d4e23a879ef052f5a8300c45c73d408fe4d9`;
- report: `3ab5b96f387ad4adc993b3e48af5ce4a42f1b22d1b1b9d3aa512d3b1b761899a`.

The one-shot worker was terminated automatically. The full CV0 stage had not
run when this result was frozen.

