# PX0-D2 dense-corner implementation audit

Status: **IMPLEMENTATION FROZEN; NO D2 CELL EXECUTED; PX0 LAW UNCHANGED**.

## Frozen identities

- protocol commit: `b820179`;
- protocol tag: `px0-d2-dense-corner-protocol-v1`;
- protocol SHA-256:
  `cf10c504bdde00469d8ec9c0839616db876a7b9acad66ffa61338510e51a6379`;
- diagnostic executable:
  `crates/px0-physical-correspondence/examples/stable_return_specificity_probe.rs`;
- diagnostic executable SHA-256:
  `a50dd8862b304abc107d729f236cb965c2e95cd823228ae9bb41119c65cf46a8`;
- active-law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

Only evaluator/example source changed. No organism or retained-physics source
changed.

## Matrix and observability

The implementation constructs the exact 256-cell balanced nearby matrix and
excludes spent stride `26` and distractor load `40`. Every cell owns a disjoint
`0x80000` physical identity range; all cell, device, distractor, and spike-origin
identities remain within that range.

Each of the 35 dense-world contexts serializes return opportunities and
completed returns separately, both resistance trajectories, live-arrow counts,
cloned A/B behavioral probes, deallocation work, queue comparisons, and total
work. Final rows independently serialize resistance, execution, no-use lifetime,
fingerprints, replay, and quiescence.

Behavioral probes run only on exact substrate clones and therefore have no
causal path back into the developmental trajectory. Their work is accounted
separately.

## Output and refusal surface

The sole D2 command is:

```text
cargo run --release -p px0-physical-correspondence \
  --example stable_return_specificity_probe -- \
  --d2 --output-prefix results/px0_d2_dense_corner_v1
```

It refuses all definitive flags with exit `2`. It checks all three final paths
for absence, publishes through create-new staging plus hard-link semantics, and
will not overwrite existing evidence.

## Validation

- formatting passed;
- strict focused Clippy passed;
- focused crate test `1/1` passed;
- release compilation passed;
- v3 definitive refusal passed with exit `2`;
- all three D2 final paths are absent;
- active-law hash is exact;
- no D2 cell or result exists.
