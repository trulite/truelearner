# TC-DS0 checkpoint-negative diagnostic result v1

Status: frozen classification; mechanics-bookkeeping difference.

The diagnostic executed once in fresh E2B sandbox
`i43hr6y5u0a6tiwk5xwdg` from frozen commit
`bf44332956a3243745b39a7d3a605ffbe8a0ccd1`.

## Exact difference

The first overall byte difference was checkpoint checksum byte 66. The first
payload difference was byte 762.

All seven CELLs differed only in `last_update_tick`:

```text
Reference   state=0, last_update_tick=15
Production  state=0, last_update_tick=0
```

Reference FullScan eagerly calls decay on inactive zero-state CELLs and moves
their bookkeeping timestamp. Production Frontier does not touch inactive
zero-state CELLs. State, refractory values, every ARROW runtime tuple, pending
counts, next serial, clock, durable body, and physical trace were identical.

Both checkpoints independently replayed exactly. After restoration under their
originating mechanics, a shared future batch exercised all seven CELLs. The
causal continuation and final durable body matched exactly:

```text
continuation trace hash
e3dff6fb1c09a647d53c4753da24d31ed6212b78b32a9a7230ebafe22a2c61d3

continuation durable-body hash
248d32a9780d881a98d62ae891c389f05abad4e55b23d71b4f83c54981b1e5b9
```

## Classification

The differing hash is mechanics bookkeeping, not a changed physical organism
history. TC-DS0 v1 remains immutable negative because its evaluator compared
the raw hashes. A v2 repair may change only cross-mechanics comparison:

- continue serializing both checkpoint hashes;
- require exact independent replay for each;
- do not require Reference and Production raw live-checkpoint bytes to match;
- retain exact equality for causal history, durable body, clock, pressure,
  pending physical activity, quiescence, and every other frozen field.

No runtime, ARC, parameter, candidate law, authority, or `arch.md` change is
supported by this diagnostic.

Artifacts:

- `results/tc_ds0_checkpoint_diagnostic_v1/cells.csv`;
- `results/tc_ds0_checkpoint_diagnostic_v1/report.md`.

Hashes:

```text
cells.csv  c8b500eaeea2ea949a21ddbb8e618106e9fdcab75a30b9a95484850c7280de3b
report.md  337d0dfb1795cafa0523d359a2009b631e7ab3efd85e1f692c8c4c19d4e9fc7b
```

