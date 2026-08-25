# TC-DS0 old-window characterization negative v1

Status: immutable evaluator stop. No characterization matrix was published.

The frozen evaluator executed once in fresh E2B sandbox
`i48pvb86s9klz2ux6gbwv` from commit
`c1c11d59954d676351689aa4824ba4ff6a7b6f11`.

It stopped on the first case:

```text
phase               0
delay               0
initial resistance  1
scenario            prompt_modulation
```

Reference and production matched on:

- emitted eligibility `0:4`;
- complete candidate trajectory;
- one plastic update;
- zero proposals and deallocations;
- final candidate live, resistance `3`, coupling `2`, eligibility absent;
- final tick `15` and pressure phase `5`;
- durable-body hash;
- physical-transition hash;
- natural quiescence;
- independent checkpoint replay.

They differed only on the canonical live-checkpoint hash:

```text
reference
6406f30cf0e2cbf07283fdaee8c24df09297766a73815b64a059db67be14f66a

production
8a3646841daefb78cee97bbd03b1d7ce6e68cfe53ca43a379d9a3695a33f003a
```

The assertion ran before artifact publication. Therefore `matrix.csv`,
`report.md`, and `SHA256SUMS` do not exist and no row count is claimed.

## Classification boundary

V1 is negative under its frozen gate. The available observation suggests a
measurement-boundary issue because causal history and durable state matched,
but this record does not infer the private byte difference.

A separately preregistered diagnostic may reproduce only this one physical
case, decode the two checkpoints into causally relevant fields, locate their
first byte/field divergence, and test continuation under identical future
input. It may not run the 1,920-row matrix or change runtime physics.

`truelearner-core`, `LOCAL_WINDOW`, ARC, `arch.md`, and authority remain
unchanged.
